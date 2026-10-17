//! Capítulo 19 — o portão do conjunto invariante: um ICP mínimo sobre três ambientes escolares.
//!
//! Laboratório autocontido, sem dependências externas: SplitMix64 embutido, mínimos quadrados por
//! equações normais, teste de invariância por permutação (Kruskal-Wallis da locação + Brown-Forsythe
//! da escala, Bonferroni sobre 2) e a regra de saturação do K = 3 (1-NN deixando a linha de fora).
//! Rodar da raiz do repositório:
//!     cargo run --release --bin labs -- --cap 19

const SEMENTE: u64 = 2019;
const N_POR_AMBIENTE: usize = 600;
const N_AMBIENTES: usize = 3;
const B_PERM: usize = 999;
const ALFA: f64 = 0.05;
const LIMIAR_SATURACAO: f64 = 0.99;

// ---------------------------------------------------------------- acaso com semente

struct Sm64 {
    estado: u64,
}

impl Sm64 {
    fn novo(semente: u64) -> Self {
        Sm64 {
            estado: semente ^ 0x9E37_79B9_7F4A_7C15,
        }
    }
    fn proximo(&mut self) -> u64 {
        self.estado = self.estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.estado;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn uniforme(&mut self) -> f64 {
        ((self.proximo() >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn normal(&mut self) -> f64 {
        let u1 = self.uniforme().max(1e-12);
        let u2 = self.uniforme();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

// ---------------------------------------------------------------- os três mundos

struct Dados {
    y: Vec<f64>,
    amb: Vec<usize>,
    cand: Vec<Vec<f64>>,
    nomes: Vec<&'static str>,
}

/// 0: o ambiente intervém em X. 1: o ambiente age direto em Y. 2: o mundo 0 mais a coluna x5,
/// que é o próprio ambiente (uma turma por escola).
fn gerar(mundo: u8, semente: u64) -> Dados {
    let mut rng = Sm64::novo(semente);
    let media_x1 = [0.0, 1.5, 3.0];
    let media_x2 = [4.0, 6.0, 8.0];
    // x4 é uma variável que o ambiente desloca e que não age em Y: o "transporte até a escola".
    let media_x4 = [0.0, 2.0, 4.0];
    let desloc_y = match mundo {
        1 => [0.0, 6.0, -4.0],
        _ => [0.0, 0.0, 0.0],
    };
    let mut y = Vec::new();
    let mut amb = Vec::new();
    let (mut c1, mut c2, mut c3, mut c4, mut c5) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for e in 0..N_AMBIENTES {
        for _ in 0..N_POR_AMBIENTE {
            let x1 = media_x1[e] + 1.5 * rng.normal();
            let x2 = media_x2[e] + 1.0 * rng.normal();
            let x3 = rng.normal();
            let erro = rng.normal();
            let yy = 1.0 * x1 + 0.8 * x2 + desloc_y[e] + erro;
            // x4 é filho de Y: um sensor que copia a nota com ruído (como os sensores de luz do
            // túnel de luz, que são filhos dos LEDs).
            let x4 = media_x4[e] + 1.0 * rng.normal();
            y.push(yy);
            amb.push(e);
            c1.push(x1);
            c2.push(x2);
            c3.push(x3);
            c4.push(x4);
            c5.push(e as f64);
        }
    }
    let mut cand = vec![c1, c2, c3];
    let mut nomes = vec!["x1 estudo", "x2 sono", "x3 ruido"];
    if mundo != 1 {
        // o sensor entra nos mundos A e C; no mundo B ficam só os três primeiros candidatos
        cand.push(c4);
        nomes.push("x4 transporte");
    }
    if mundo == 2 {
        cand.push(c5);
        nomes.push("x5 turma");
    }
    Dados { y, amb, cand, nomes }
}

// ---------------------------------------------------------------- mínimos quadrados

/// Mínimos quadrados por equações normais. `x` vem em colunas: x[coluna][linha].
fn mqo(x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let p = x.len();
    let n = y.len();
    let mut a = vec![vec![0.0; p + 1]; p];
    for i in 0..n {
        for j in 0..p {
            for k in 0..p {
                a[j][k] += x[j][i] * x[k][i];
            }
            a[j][p] += x[j][i] * y[i];
        }
    }
    for j in 0..p {
        a[j][j] += 1e-10;
    }
    for col in 0..p {
        let mut piv = col;
        for r in (col + 1)..p {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        a.swap(col, piv);
        let d = a[col][col];
        for k in col..=p {
            a[col][k] /= d;
        }
        for r in 0..p {
            if r != col {
                let f = a[r][col];
                if f != 0.0 {
                    for k in col..=p {
                        a[r][k] -= f * a[col][k];
                    }
                }
            }
        }
    }
    (0..p).map(|j| a[j][p]).collect()
}

/// Expansão do K = 2: categórico vira bloco de indicadoras (referência = nível mais frequente),
/// contínuo entra como termo linear.
fn colunas(dados: &Dados, s: &[usize]) -> Vec<Vec<f64>> {
    let n = dados.y.len();
    let mut cols = vec![vec![1.0; n]];
    for &j in s {
        let v = &dados.cand[j];
        let mut distintos: Vec<f64> = v.clone();
        distintos.sort_by(|a, b| a.partial_cmp(b).unwrap());
        distintos.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
        if distintos.len() <= 10 {
            let mut melhor = distintos[0];
            let mut melhor_n = 0usize;
            for &d in &distintos {
                let c = v.iter().filter(|x| (**x - d).abs() < 1e-12).count();
                if c > melhor_n {
                    melhor_n = c;
                    melhor = d;
                }
            }
            for &d in &distintos {
                if (d - melhor).abs() < 1e-12 {
                    continue;
                }
                cols.push(
                    v.iter()
                        .map(|x| if (*x - d).abs() < 1e-12 { 1.0 } else { 0.0 })
                        .collect(),
                );
            }
        } else {
            cols.push(v.clone());
        }
    }
    cols
}

// ---------------------------------------------------------------- os dois testes

fn kruskal(u: &[f64], g: &[usize], k: usize) -> f64 {
    let n = u.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| u[a].partial_cmp(&u[b]).unwrap());
    let mut posto = vec![0.0; n];
    let mut i = 0;
    let mut correcao = 0.0;
    while i < n {
        let mut j = i;
        while j + 1 < n && (u[idx[j + 1]] - u[idx[i]]).abs() < 1e-12 {
            j += 1;
        }
        let r = ((i + j) as f64) / 2.0 + 1.0;
        for t in i..=j {
            posto[idx[t]] = r;
        }
        let tam = (j - i + 1) as f64;
        correcao += tam * tam * tam - tam;
        i = j + 1;
    }
    let (mut soma, mut cont) = (vec![0.0; k], vec![0.0; k]);
    for linha in 0..n {
        soma[g[linha]] += posto[linha];
        cont[g[linha]] += 1.0;
    }
    let mut h = 0.0;
    for gg in 0..k {
        h += cont[gg] * (soma[gg] / cont[gg] - (n as f64 + 1.0) / 2.0).powi(2);
    }
    h *= 12.0 / ((n as f64) * (n as f64 + 1.0));
    h / (1.0 - correcao / ((n as f64).powi(3) - n as f64)).max(1e-12)
}

fn brown_forsythe(u: &[f64], g: &[usize], k: usize) -> f64 {
    let n = u.len();
    let mut med = vec![0.0; k];
    for gg in 0..k {
        let mut v: Vec<f64> = (0..n).filter(|&i| g[i] == gg).map(|i| u[i]).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        med[gg] = if v.len() % 2 == 1 {
            v[v.len() / 2]
        } else {
            0.5 * (v[v.len() / 2 - 1] + v[v.len() / 2])
        };
    }
    let z: Vec<f64> = (0..n).map(|i| (u[i] - med[g[i]]).abs()).collect();
    let (mut soma, mut cont) = (vec![0.0; k], vec![0.0; k]);
    let mut total = 0.0;
    for i in 0..n {
        soma[g[i]] += z[i];
        cont[g[i]] += 1.0;
        total += z[i];
    }
    let media = total / n as f64;
    let (mut entre, mut dentro) = (0.0, 0.0);
    for gg in 0..k {
        let mg = soma[gg] / cont[gg];
        entre += cont[gg] * (mg - media).powi(2);
        for i in 0..n {
            if g[i] == gg {
                dentro += (z[i] - mg).powi(2);
            }
        }
    }
    ((n - k) as f64 / (k - 1) as f64) * entre / dentro.max(1e-300)
}

/// p de locação, p de escala e p do conjunto (Bonferroni sobre 2), por permutação dos rótulos.
fn teste(u: &[f64], amb: &[usize], semente: u64) -> (f64, f64, f64) {
    let h_obs = kruskal(u, amb, N_AMBIENTES);
    let f_obs = brown_forsythe(u, amb, N_AMBIENTES);
    let mut rng = Sm64::novo(semente);
    let mut g = amb.to_vec();
    let (mut mais_h, mut mais_f) = (0usize, 0usize);
    for _ in 0..B_PERM {
        for i in (1..g.len()).rev() {
            let j = ((rng.uniforme() * ((i + 1) as f64)) as usize).min(i);
            g.swap(i, j);
        }
        if kruskal(u, &g, N_AMBIENTES) >= h_obs - 1e-12 {
            mais_h += 1;
        }
        if brown_forsythe(u, &g, N_AMBIENTES) >= f_obs - 1e-12 {
            mais_f += 1;
        }
    }
    let p_loc = (1 + mais_h) as f64 / (B_PERM + 1) as f64;
    let p_esc = (1 + mais_f) as f64 / (B_PERM + 1) as f64;
    (p_loc, p_esc, (2.0 * p_loc.min(p_esc)).min(1.0))
}

/// Acerto médio do 1-NN deixando a linha de fora, ao prever o ambiente pelas colunas de S.
fn acerto_1nn(dados: &Dados, s: &[usize]) -> f64 {
    let n = dados.y.len();
    if s.is_empty() {
        return 1.0 / N_AMBIENTES as f64;
    }
    let mut cols: Vec<Vec<f64>> = Vec::new();
    for &j in s {
        let v = &dados.cand[j];
        let m = v.iter().sum::<f64>() / n as f64;
        let dp = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n as f64)
            .sqrt()
            .max(1e-12);
        cols.push(v.iter().map(|x| (x - m) / dp).collect());
    }
    let mut acertos = 0.0;
    for i in 0..n {
        let mut celula: Vec<usize> = Vec::new();
        for j in 0..n {
            if j != i && cols.iter().all(|c| (c[i] - c[j]).abs() < 1e-9) {
                celula.push(j);
            }
        }
        if !celula.is_empty() {
            let iguais = celula.iter().filter(|&&j| dados.amb[j] == dados.amb[i]).count();
            acertos += iguais as f64 / celula.len() as f64;
        } else {
            let (mut melhor, mut viz) = (f64::INFINITY, 0usize);
            for j in 0..n {
                if j == i {
                    continue;
                }
                let d: f64 = cols.iter().map(|c| (c[i] - c[j]).powi(2)).sum();
                if d < melhor {
                    melhor = d;
                    viz = j;
                }
            }
            if dados.amb[viz] == dados.amb[i] {
                acertos += 1.0;
            }
        }
    }
    acertos / n as f64
}

// ---------------------------------------------------------------- o ICP mínimo

struct Linha {
    s: Vec<usize>,
    p_loc: f64,
    p_esc: f64,
    p_s: f64,
    aceito: bool,
    acerto: f64,
    saturado: bool,
}

struct Relatorio {
    linhas: Vec<Linha>,
    aceitos_familia: usize,
    aceitos_todos: usize,
    saturados_testados: usize,
    saturados_aceitos: usize,
    s_com_regra: Vec<usize>,
    s_sem_regra: Vec<usize>,
}

fn icp(dados: &Dados, regra_saturacao: bool) -> Relatorio {
    let p = dados.cand.len();
    let n = dados.y.len();
    let mut linhas = Vec::new();
    for mascara in 0..(1usize << p) {
        let s: Vec<usize> = (0..p).filter(|j| mascara & (1 << j) != 0).collect();
        let x = colunas(dados, &s);
        let b = mqo(&x, &dados.y);
        let ajustado: Vec<f64> = (0..n)
            .map(|i| (0..x.len()).map(|c| x[c][i] * b[c]).sum())
            .collect();
        let r: Vec<f64> = (0..n).map(|i| dados.y[i] - ajustado[i]).collect();
        let abs: Vec<f64> = r.iter().map(|v| v.abs()).collect();
        let b_abs = mqo(&x, &abs);
        let media_abs = abs.iter().sum::<f64>() / n as f64;
        let u: Vec<f64> = (0..n)
            .map(|i| {
                let ajuste: f64 = (0..x.len()).map(|c| x[c][i] * b_abs[c]).sum();
                let s_hat = ajuste.max(0.1 * media_abs).max(1e-12);
                r[i] / s_hat
            })
            .collect();
        let (p_loc, p_esc, p_s) = teste(&u, &dados.amb, SEMENTE ^ ((mascara as u64 + 1) * 0x9E37));
        let acerto = acerto_1nn(dados, &s);
        let saturado = regra_saturacao && acerto >= LIMIAR_SATURACAO;
        linhas.push(Linha {
            s,
            p_loc,
            p_esc,
            p_s,
            aceito: p_s >= ALFA,
            acerto,
            saturado,
        });
    }
    let na_familia: Vec<&Linha> = linhas.iter().filter(|l| !l.saturado).collect();
    let aceitos_familia = na_familia.iter().filter(|l| l.aceito).count();
    let aceitos_todos = linhas.iter().filter(|l| l.aceito).count();
    let saturados_aceitos = linhas.iter().filter(|l| l.aceito && l.acerto >= LIMIAR_SATURACAO).count();
    let intersecao = |fonte: &[&Linha]| -> Vec<usize> {
        let aceitos: Vec<&&Linha> = fonte.iter().filter(|l| l.aceito).collect();
        if aceitos.is_empty() {
            return Vec::new();
        }
        (0..p)
            .filter(|j| aceitos.iter().all(|l| l.s.contains(j)))
            .collect()
    };
    let com_regra = intersecao(&na_familia);
    let todos: Vec<&Linha> = linhas.iter().collect();
    let sem_regra = intersecao(&todos);
    let saturados_testados = linhas.iter().filter(|l| l.acerto >= LIMIAR_SATURACAO).count();
    Relatorio {
        linhas,
        aceitos_familia,
        aceitos_todos,
        saturados_testados,
        saturados_aceitos,
        s_com_regra: com_regra,
        s_sem_regra: sem_regra,
    }
}

fn nome(dados: &Dados, s: &[usize]) -> String {
    if s.is_empty() {
        return "{}".to_string();
    }
    format!(
        "{{{}}}",
        s.iter()
            .map(|&j| dados.nomes[j])
            .collect::<Vec<_>>()
            .join(", ")
    )
}

pub fn rodar() {
    println!("== Capítulo 19 :: o portão do conjunto invariante ==");
    println!(
        "semente {} | {} ambientes x {} linhas | {} permutacoes por teste | alfa = {:.2}",
        SEMENTE, N_AMBIENTES, N_POR_AMBIENTE, B_PERM, ALFA
    );
    let titulos = [
        "mundo A: o ambiente intervem em X (3 escolas, 4 candidatos)",
        "mundo B: o ambiente age direto em Y (o ano, 3 candidatos)",
        "mundo C: mundo A mais x5, a coluna que e o proprio ambiente (5 candidatos)",
    ];
    let interessa: [Vec<usize>; 9] = [
        vec![],
        vec![0],
        vec![1],
        vec![2],
        vec![3],
        vec![0, 1],
        vec![0, 1, 3],
        vec![4],
        vec![0, 1, 4],
    ];
    for (mundo, titulo) in titulos.iter().enumerate() {
        let dados = gerar(mundo as u8, SEMENTE);
        let r = icp(&dados, true);
        println!("\n{}", titulo);
        println!(
            "  {:<34} {:>7} {:>8} {:>8}  {:<10} {:>7}",
            "conjunto", "p_local", "p_escala", "p_conj", "veredito", "1-NN"
        );
        for alvo in interessa.iter() {
            if alvo.iter().any(|&j| j >= dados.cand.len()) {
                continue;
            }
            if let Some(l) = r.linhas.iter().find(|l| l.s == *alvo) {
                println!(
                    "  {:<34} {:>7.4} {:>8.4} {:>8.4}  {:<10} {:>7.3}",
                    nome(&dados, &l.s),
                    l.p_loc,
                    l.p_esc,
                    l.p_s,
                    if l.aceito { "aceito" } else { "rejeitado" },
                    l.acerto
                );
            }
        }
        if std::env::var("CAP19_TUDO").is_ok() {
            for l in &r.linhas {
                println!(
                    "    [debug] {:<34} p_loc={:.4} p_esc={:.4} p_S={:.4} {} 1nn={:.3} sat={}",
                    nome(&dados, &l.s),
                    l.p_loc,
                    l.p_esc,
                    l.p_s,
                    if l.aceito { "aceito" } else { "rejeitado" },
                    l.acerto,
                    l.saturado
                );
            }
        }
        println!(
            "  aceitos na familia: {} de {} (com os saturados seriam {}) | saturados testados: {} | aceitos saturados: {}",
            r.aceitos_familia,
            1usize << dados.cand.len(),
            r.aceitos_todos,
            r.saturados_testados,
            r.saturados_aceitos
        );
        println!(
            "  S_hat com a regra de saturacao = {}  | sem a regra = {}  | portao: {}",
            nome(&dados, &r.s_com_regra),
            nome(&dados, &r.s_sem_regra),
            if r.s_com_regra.is_empty() {
                "FALHA (S_hat vazio)"
            } else {
                "PASSA (S_hat nao vazio)"
            }
        );
    }
}
