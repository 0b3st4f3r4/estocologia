//! Capítulo 7 — Caçando fantasmas no ruído: modelo nulo exato, jardim dos caminhos,
//! severidade e os cinco erros.
//!
//! Laboratório autocontido: SplitMix64 embutido (o mesmo desenho de
//! crates/stochology-core/src/rng.rs), sem dependências externas.
//! Uso: cargo run --release

#[derive(Clone, Copy)]
struct SplitMix64 {
    estado: u64,
}

impl SplitMix64 {
    fn novo(semente: u64) -> Self {
        Self { estado: semente }
    }
    fn proximo(&mut self) -> u64 {
        stochology_core::rng::passo_splitmix64(&mut self.estado)
    }
    fn uniforme(&mut self) -> f64 {
        (self.proximo() >> 11) as f64 / ((1u64 << 53) as f64)
    }
    fn normal(&mut self) -> f64 {
        let u1 = self.uniforme().max(1e-18);
        let u2 = self.uniforme();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    fn bernoulli(&mut self, p: f64) -> bool {
        self.uniforme() < p
    }
}

/// Função erro (Abramowitz e Stegun 7.1.26).
fn erf(x: f64) -> f64 {
    let s = x.signum();
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let y = 1.0
        - (((((1.061_405_429 * t - 1.453_152_027) * t + 1.421_413_741) * t - 0.284_496_736) * t
            + 0.254_829_592)
            * t)
            * (-x * x).exp();
    s * y
}

fn cdf_normal(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / 2f64.sqrt()))
}

fn media(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn variancia_amostral(xs: &[f64]) -> f64 {
    let m = media(xs);
    xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0)
}

/// p bilateral do teste t de duas amostras, com a aproximação normal.
fn t_test(a: &[f64], b: &[f64]) -> f64 {
    let se =
        (variancia_amostral(a) / a.len() as f64 + variancia_amostral(b) / b.len() as f64).sqrt();
    let t = (media(a) - media(b)) / se;
    2.0 * (1.0 - cdf_normal(t.abs()))
}

/// Mínimos quadrados com equações normais: devolve beta, (X'X)^-1 e o resíduo médio quadrático.
fn regressao(colunas: &[Vec<f64>], y: &[f64]) -> Option<(Vec<f64>, Vec<Vec<f64>>, f64)> {
    let k = colunas.len() + 1;
    let n = y.len();
    let mut a = vec![vec![0.0f64; k]; k];
    let mut b = vec![0.0f64; k];
    for i in 0..k {
        for j in 0..k {
            let mut s = 0.0;
            for r in 0..n {
                let x = if i == 0 { 1.0 } else { colunas[i - 1][r] };
                let z = if j == 0 { 1.0 } else { colunas[j - 1][r] };
                s += x * z;
            }
            a[i][j] = s;
        }
        let mut s = 0.0;
        for r in 0..n {
            let x = if i == 0 { 1.0 } else { colunas[i - 1][r] };
            s += x * y[r];
        }
        b[i] = s;
    }
    // Gauss-Jordan com a identidade ao lado, para obter a inversa.
    let mut aug = vec![vec![0.0f64; 2 * k]; k];
    for i in 0..k {
        for j in 0..k {
            aug[i][j] = a[i][j];
        }
        aug[i][k + i] = 1.0;
    }
    for col in 0..k {
        let mut piv = col;
        for r in col + 1..k {
            if aug[r][col].abs() > aug[piv][col].abs() {
                piv = r;
            }
        }
        if aug[piv][col].abs() < 1e-9 {
            return None;
        }
        aug.swap(col, piv);
        let d = aug[col][col];
        for j in 0..2 * k {
            aug[col][j] /= d;
        }
        for r in 0..k {
            if r != col {
                let f = aug[r][col];
                for j in 0..2 * k {
                    aug[r][j] -= f * aug[col][j];
                }
            }
        }
    }
    let inv: Vec<Vec<f64>> = (0..k)
        .map(|i| (0..k).map(|j| aug[i][k + j]).collect())
        .collect();
    let beta: Vec<f64> = (0..k)
        .map(|i| (0..k).map(|j| inv[i][j] * b[j]).sum::<f64>())
        .collect();
    let mut rss = 0.0;
    for r in 0..n {
        let mut p = beta[0];
        for (i, c) in colunas.iter().enumerate() {
            p += beta[i + 1] * c[r];
        }
        rss += (y[r] - p) * (y[r] - p);
    }
    Some((beta.clone(), inv, rss / (n - k).max(1) as f64))
}

/// p bilateral do efeito do grupo (primeira coluna), controlando pelas demais.
fn p_efeito(y: &[f64], x: &[f64], covs: &[&Vec<f64>]) -> Option<f64> {
    let mut cols: Vec<Vec<f64>> = vec![x.to_vec()];
    for c in covs {
        cols.push((*c).clone());
    }
    let (beta, inv, s2) = regressao(&cols, y)?;
    let se = (s2 * inv[1][1]).sqrt();
    if se <= 0.0 {
        return None;
    }
    Some(2.0 * (1.0 - cdf_normal((beta[1] / se).abs())))
}

/// O jardim dos modelos: 500 candidatos ajustados em ruído puro.
struct Jardim {
    replicas: usize,
    candidatos: usize,
    valida_melhor_que_zero: f64,
    testa_melhor_que_zero: f64,
    erro_medio_validacao: f64,
    erro_medio_teste: f64,
}

fn jardim_de_modelos(
    semente: u64,
    replicas: usize,
    candidatos: usize,
    preditores: usize,
) -> Jardim {
    let mut rng = SplitMix64::novo(semente);
    let (n_treino, n_val, n_teste) = (25usize, 25usize, 25usize);
    let mut val_ganha = 0usize;
    let mut test_ganha = 0usize;
    let mut soma_val = 0.0;
    let mut soma_test = 0.0;
    for _ in 0..replicas {
        let total = n_treino + n_val + n_teste;
        let mut x = vec![vec![0.0f64; total]; preditores];
        for linha in x.iter_mut() {
            for v in linha.iter_mut() {
                *v = rng.normal();
            }
        }
        let y: Vec<f64> = (0..total).map(|_| rng.normal()).collect();
        let mut melhor_val = f64::INFINITY;
        let mut melhor_teste = f64::INFINITY;
        for _ in 0..candidatos {
            let k = 1 + (rng.proximo() % 5) as usize;
            let mut escolhidos: Vec<usize> = Vec::new();
            while escolhidos.len() < k {
                let c = (rng.proximo() % preditores as u64) as usize;
                if !escolhidos.contains(&c) {
                    escolhidos.push(c);
                }
            }
            let cols: Vec<Vec<f64>> = escolhidos
                .iter()
                .map(|c| x[*c][0..n_treino].to_vec())
                .collect();
            let (beta, _, _) = match regressao(&cols, &y[0..n_treino]) {
                Some(b) => b,
                None => continue,
            };
            let mut sq_val = 0.0;
            for r in 0..n_val {
                let linha = n_treino + r;
                let mut p = beta[0];
                for (i, c) in escolhidos.iter().enumerate() {
                    p += beta[i + 1] * x[*c][linha];
                }
                sq_val += (y[linha] - p) * (y[linha] - p);
            }
            let mse_val = sq_val / n_val as f64;
            if mse_val < melhor_val {
                melhor_val = mse_val;
                let mut sq = 0.0;
                for r in 0..n_teste {
                    let linha = n_treino + n_val + r;
                    let mut p = beta[0];
                    for (i, c) in escolhidos.iter().enumerate() {
                        p += beta[i + 1] * x[*c][linha];
                    }
                    sq += (y[linha] - p) * (y[linha] - p);
                }
                melhor_teste = sq / n_teste as f64;
            }
        }
        if melhor_val < 1.0 {
            val_ganha += 1;
        }
        if melhor_teste < 1.0 {
            test_ganha += 1;
        }
        soma_val += melhor_val;
        soma_test += melhor_teste;
    }
    Jardim {
        replicas,
        candidatos,
        valida_melhor_que_zero: 100.0 * val_ganha as f64 / replicas as f64,
        testa_melhor_que_zero: 100.0 * test_ganha as f64 / replicas as f64,
        erro_medio_validacao: soma_val / replicas as f64,
        erro_medio_teste: soma_test / replicas as f64,
    }
}

/// Mão quente: caras depois de cara em três lances de uma moeda honesta.
fn mao_quente() -> (f64, f64) {
    let mut soma_por_sequencia = 0.0;
    let mut definidas = 0usize;
    let mut hh = 0usize;
    let mut ht = 0usize;
    for mascara in 0..8u32 {
        let s: Vec<bool> = (0..3).map(|i| mascara & (1 << i) != 0).collect();
        let (mut a, mut b) = (0usize, 0usize);
        for i in 0..2 {
            if s[i] {
                if s[i + 1] {
                    a += 1;
                } else {
                    b += 1;
                }
            }
        }
        hh += a;
        ht += b;
        if a + b > 0 {
            soma_por_sequencia += a as f64 / (a + b) as f64;
            definidas += 1;
        }
    }
    (
        soma_por_sequencia / definidas as f64,
        hh as f64 / (hh + ht) as f64,
    )
}

/// 100 pessoas jogando 20 moedas cada uma: o fantasma que o acaso produz.
fn fantasma(semente: u64, replicas: usize) -> (f64, f64) {
    let mut rng = SplitMix64::novo(semente);
    let mut com_seis = 0usize;
    let mut soma_maior = 0.0;
    let mut recorde = 0usize;
    for _ in 0..replicas {
        let mut maior = 0usize;
        for _ in 0..100 {
            let (mut atual, mut melhor) = (0usize, 0usize);
            for _ in 0..20 {
                if rng.bernoulli(0.5) {
                    atual += 1;
                    melhor = melhor.max(atual);
                } else {
                    atual = 0;
                }
            }
            maior = maior.max(melhor);
        }
        soma_maior += maior as f64;
        recorde = recorde.max(maior);
        if maior >= 6 {
            com_seis += 1;
        }
    }
    (
        100.0 * com_seis as f64 / replicas as f64,
        soma_maior / replicas as f64,
    )
}

/// O jardim dos caminhos que se bifurcam: liberdades de pesquisador num
/// experimento sem efeito nenhum. Mede dois jardins: o pequeno (quatro
/// liberdades, cada uma usada uma vez) e o grande (com parada opcional).
struct Caminhos {
    replicas: usize,
    pequeno: f64,
    grande: f64,
    pre_registrado: f64,
}

fn jardim_dos_caminhos(semente: u64, replicas: usize) -> Caminhos {
    let mut rng = SplitMix64::novo(semente);
    let n = 40usize;
    let mut pequenos = 0usize;
    let mut grandes = 0usize;
    let mut pre = 0usize;
    for _ in 0..replicas {
        let mut e1 = Vec::with_capacity(2 * n);
        let mut e2 = Vec::with_capacity(2 * n);
        let mut c1 = Vec::with_capacity(2 * n);
        let mut c2 = Vec::with_capacity(2 * n);
        let mut c3 = Vec::with_capacity(2 * n);
        let mut sexo = Vec::with_capacity(2 * n);
        let mut grupo = Vec::with_capacity(2 * n);
        for i in 0..2 * n {
            grupo.push(if i < n { 0.0 } else { 1.0 });
            let a = rng.normal();
            let b = rng.normal();
            let d = rng.normal();
            e1.push(a);
            e2.push(0.7 * a + 0.7 * b);
            c1.push(d);
            c2.push(rng.normal());
            c3.push(rng.normal());
            sexo.push(if rng.bernoulli(0.5) { 1.0 } else { 0.0 });
        }
        let dv1: Vec<f64> = e1.iter().zip(c1.iter()).map(|(a, d)| a + 0.4 * d).collect();
        let dv2: Vec<f64> = e2.iter().zip(c2.iter()).map(|(a, d)| a + 0.4 * d).collect();
        let analisa = |y: &Vec<f64>, m: usize, covs: &[&Vec<f64>]| -> f64 {
            let idx: Vec<usize> = (0..m).chain(n..n + m).collect();
            let yy: Vec<f64> = idx.iter().map(|i| y[*i]).collect();
            let gg: Vec<f64> = idx.iter().map(|i| grupo[*i]).collect();
            let cc: Vec<Vec<f64>> = covs
                .iter()
                .map(|c| idx.iter().map(|i| c[*i]).collect())
                .collect();
            let refs: Vec<&Vec<f64>> = cc.iter().collect();
            p_efeito(&yy, &gg, &refs).unwrap_or(1.0)
        };
        let exclui = |y: &Vec<f64>, m: usize| -> f64 {
            let mut max_i = 0usize;
            let mut max_v = f64::NEG_INFINITY;
            for i in 0..m {
                if y[n + i] > max_v {
                    max_v = y[n + i];
                    max_i = n + i;
                }
            }
            let idx: Vec<usize> = (0..2 * m).filter(|i| *i != max_i).collect();
            let yy: Vec<f64> = idx.iter().map(|i| y[*i]).collect();
            let meio = m - 1;
            t_test(&yy[0..meio], &yy[meio..])
        };
        if t_test(&dv1[0..30], &dv1[n..n + 30]) < 0.05 {
            pre += 1;
        }

        // jardim pequeno: quatro liberdades, cada uma usada uma vez
        let mut p_pequeno = 1.0f64;
        for y in [&dv1, &dv2] {
            for m in [20usize, 30] {
                p_pequeno = p_pequeno.min(t_test(&y[0..m], &y[n..n + m]));
                p_pequeno = p_pequeno.min(analisa(y, m, &[&c1]));
            }
            p_pequeno = p_pequeno.min(exclui(y, 30));
        }
        if p_pequeno < 0.05 {
            pequenos += 1;
        }

        // jardim grande: parada opcional, covariáveis em série, exclusão e subgrupo
        let mut p_grande = 1.0f64;
        for y in [&dv1, &dv2] {
            for m in [10usize, 20, 30, 40] {
                let p = t_test(&y[0..m], &y[n..n + m]);
                p_grande = p_grande.min(p);
                if p < 0.05 {
                    break;
                }
            }
            for m in [20usize, 30, 40] {
                for covs in [vec![&c1], vec![&c1, &c2], vec![&c1, &c2, &c3]] {
                    p_grande = p_grande.min(analisa(y, m, &covs));
                }
                p_grande = p_grande.min(exclui(y, m));
            }
            for alvo in [0.0f64, 1.0] {
                let idx: Vec<usize> = (0..2 * n).filter(|i| sexo[*i] == alvo).collect();
                let a: Vec<f64> = idx.iter().filter(|i| **i < n).map(|i| y[*i]).collect();
                let b: Vec<f64> = idx.iter().filter(|i| **i >= n).map(|i| y[*i]).collect();
                if a.len() > 3 && b.len() > 3 {
                    p_grande = p_grande.min(t_test(&a, &b));
                }
            }
        }
        if p_grande < 0.05 {
            grandes += 1;
        }
    }
    Caminhos {
        replicas,
        pequeno: 100.0 * pequenos as f64 / replicas as f64,
        grande: 100.0 * grandes as f64 / replicas as f64,
        pre_registrado: 100.0 * pre as f64 / replicas as f64,
    }
}

/// Valor preditivo de um achado positivo (Ioannidis, 2005).
fn vpp(poder: f64, chances: f64, alpha: f64) -> f64 {
    (poder * chances) / (chances - (1.0 - poder) * chances + alpha)
}

/// A severidade da banca adversarial A1-A8, no mesmo desenho de
/// crates/stochology-core/src/agentes.rs.
fn severidade_do_agente(ordinal: u64, cuidado_atendido: bool, semente: u64) -> u8 {
    let s = semente ^ ordinal.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut rng = SplitMix64::novo(s);
    let sorteio = rng.proximo() % 3;
    if cuidado_atendido {
        (8 + sorteio) as u8
    } else {
        (2 + sorteio) as u8
    }
}

pub fn rodar() {
    println!("== Capítulo 7 :: caçando fantasmas no ruído ==");

    let j = jardim_de_modelos(2026, 200, 500, 50);
    println!(
        "jardim de modelos: {} replicas, {} candidatos em 50 preditores de puro ruido, 25 casos de treino",
        j.replicas, j.candidatos
    );
    println!(
        "o vencedor da validacao bate prever zero em {:.1}% das replicas; diante de dados novos, em {:.1}%",
        j.valida_melhor_que_zero, j.testa_melhor_que_zero
    );
    println!(
        "erro quadratico medio do vencedor: {:.3} na validacao (prever zero: 1,000) e {:.3} em dados novos",
        j.erro_medio_validacao, j.erro_medio_teste
    );

    let (media_sequencia, pool) = mao_quente();
    let (pct_seis, maior_medio) = fantasma(2026, 2_000);
    println!(
        "mao quente: media das proporcoes por sequencia = {:.4} (5/12 = {:.4}); proporcao agrupada = {:.4}",
        media_sequencia,
        5.0 / 12.0,
        pool
    );
    println!(
        "100 pessoas jogando 20 moedas: em {:.1}% dos experimentos alguem emenda 6 caras seguidas; a maior sequencia tem media {:.1}",
        pct_seis, maior_medio
    );

    let c = jardim_dos_caminhos(2026, 4_000);
    println!(
        "jardim dos caminhos ({} experimentos sem efeito nenhum): com as quatro liberdades de Simmons, alguem acha p < 0,05 em {:.1}% deles; com a familia grande, em {:.1}%; o teste pre-registrado acusa {:.1}%",
        c.replicas, c.pequeno, c.grande, c.pre_registrado
    );
    println!(
        "severidade de Mayo: 1 - p = {:.2} no teste pre-registrado com p = 0,04; 1 - taxa de falso positivo = {:.2} no jardim",
        1.0 - 0.04,
        1.0 - c.grande / 100.0
    );

    for chances in [1.0 / 1000.0, 1.0 / 10.0, 1.0 / 3.0] {
        println!(
            "Ioannidis com R = 1/{:.0}, poder 0,80: VPP = {:.3} com alfa 0,05 e {:.3} com o alfa do jardim",
            1.0 / chances,
            vpp(0.80, chances, 0.05),
            vpp(0.80, chances, c.grande / 100.0)
        );
    }

    let boa: Vec<u8> = (1..=8)
        .map(|i| severidade_do_agente(i, true, 2026))
        .collect();
    let pior: u8 = (1..=8)
        .map(|i| severidade_do_agente(i, false, 2026))
        .min()
        .unwrap();
    println!(
        "banca A1-A8 com a semente 2026, ficha com os oito cuidados: severidades {:?}, piso 6, aprovada = {}",
        boa,
        boa.iter().all(|s| *s >= 6)
    );
    println!(
        "a mesma ficha sem o cuidado de uma faixa: a menor severidade cai para {} (reprovada)",
        pior
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn jardim_inflaciona_falso_positivo_e_mao_quente_tem_vies_negativo() {
        let (exato, ing) = mao_quente();
        assert!((ing - 0.5).abs() < 1e-9);
        assert!(exato < 0.42);
        let c = jardim_dos_caminhos(2026, 200);
        assert!(c.pre_registrado < 10.0);
        assert!(c.grande > c.pequeno && c.pequeno > c.pre_registrado);
    }
}
