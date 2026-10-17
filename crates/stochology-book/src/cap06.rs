//! Capítulo 6 — O filtro invisível: o Literary Digest, a identidade de Meng
//! e a unidade independente.
//!
//! Laboratório autocontido: SplitMix64 embutido, sem dependências externas.
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
    fn bernoulli(&mut self, p: f64) -> bool {
        self.uniforme() < p
    }
    /// Normal padrão por Box-Muller.
    fn normal(&mut self) -> f64 {
        let u1 = self.uniforme().max(1e-18);
        let u2 = self.uniforme();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

/// A pesquisa viciada de 1936: votantes com propensão a responder que depende do voto.
fn literary_digest(
    semente: u64,
    n_eleitores: u64,
    p_roosevelt: f64,
    q_roosevelt: f64,
    q_landon: f64,
) -> (u64, f64) {
    let mut rng = SplitMix64::novo(semente);
    let mut respondentes = 0u64;
    let mut roosevelt_entre_respondentes = 0u64;
    for _ in 0..n_eleitores {
        let roosevelt = rng.bernoulli(p_roosevelt);
        let q = if roosevelt { q_roosevelt } else { q_landon };
        if rng.bernoulli(q) {
            respondentes += 1;
            if roosevelt {
                roosevelt_entre_respondentes += 1;
            }
        }
    }
    (
        respondentes,
        roosevelt_entre_respondentes as f64 / respondentes as f64,
    )
}

/// Erro esperado pela identidade de Meng: rho * sqrt((1-f)/f) * sigma_Y.
fn meng(rho: f64, f: f64, sigma_y: f64) -> f64 {
    rho * ((1.0 - f) / f).sqrt() * sigma_y
}

/// Coeficiente de correlação entre duas colunas.
fn correlacao(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    let mut cab = 0.0;
    let mut va = 0.0;
    let mut vb = 0.0;
    for i in 0..a.len() {
        let da = a[i] - ma;
        let db = b[i] - mb;
        cab += da * db;
        va += da * da;
        vb += db * db;
    }
    cab / (va.sqrt() * vb.sqrt())
}

/// O filtro fabrica relações: dois traços independentes na população ficam
/// negativamente associados entre os selecionados (Berkson).
fn berkson(semente: u64, n: usize, corte: f64) -> (f64, f64, f64) {
    let mut rng = SplitMix64::novo(semente);
    let mut t_pop = Vec::with_capacity(n);
    let mut s_pop = Vec::with_capacity(n);
    let mut t_sel = Vec::new();
    let mut s_sel = Vec::new();
    for _ in 0..n {
        let t = rng.normal();
        let s = rng.normal();
        t_pop.push(t);
        s_pop.push(s);
        if t + s > corte {
            t_sel.push(t);
            s_sel.push(s);
        }
    }
    (
        correlacao(&t_pop, &s_pop),
        correlacao(&t_sel, &s_sel),
        100.0 * t_sel.len() as f64 / n as f64,
    )
}

/// Tamanho efetivo de uma amostra de n letras numa cadeia de dois estados com
/// persistência p: mede a variância da média amostral em muitas réplicas.
fn tamanho_efetivo(semente: u64, p_persistencia: f64, n: usize, replicas: usize) -> (f64, f64) {
    let mut rng = SplitMix64::novo(semente);
    let mut medias = Vec::with_capacity(replicas);
    for _ in 0..replicas {
        let mut estado = rng.bernoulli(0.5);
        let mut soma = 0usize;
        for _ in 0..n {
            if rng.uniforme() > p_persistencia {
                estado = !estado;
            }
            if estado {
                soma += 1;
            }
        }
        medias.push(soma as f64 / n as f64);
    }
    let m = medias.iter().sum::<f64>() / replicas as f64;
    let var = medias.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (replicas as f64 - 1.0);
    let p_medio = 0.5;
    (var, p_medio * (1.0 - p_medio) / var)
}

pub fn rodar() {
    println!("== Capítulo 6 :: o filtro invisível ==");

    // (1) A pesquisa viciada de 1936.
    let p_roosevelt = 0.608_f64;
    let sigma_y = (p_roosevelt * (1.0 - p_roosevelt)).sqrt();
    let n_eleitores = 10_000_000u64;
    let (respondentes, r_entre) = literary_digest(1936, n_eleitores, p_roosevelt, 0.168, 0.3517);
    let f = respondentes as f64 / n_eleitores as f64;
    let erro = r_entre - p_roosevelt;
    println!(
        "Literary Digest: {} de {} eleitores responderam (f = {:.3}); Roosevelt entre eles = {:.4}, Landon = {:.4}",
        respondentes,
        n_eleitores,
        f,
        r_entre,
        1.0 - r_entre
    );
    println!(
        "erro medido = {:+.4} ({:.1} pontos); identidade de Meng com rho = -0.21: {:+.4}",
        erro,
        100.0 * erro,
        meng(-0.21, f, sigma_y)
    );
    let n_ef = (sigma_y / erro).powi(2);
    println!(
        "o erro vale o erro-padrao de uma amostra aleatoria de {:.1} pessoas; a margem de erro da pesquisa era {:.4} pontos, e o erro foi de {:.1} pontos = {:.0} margens",
        n_ef,
        100.0 * (0.24 * 0.76 / respondentes as f64).sqrt(),
        100.0 * erro.abs(),
        erro.abs() / (0.24 * 0.76 / respondentes as f64).sqrt()
    );
    // Gallup e as amostras aleatórias.
    let mut rng = SplitMix64::novo(2026);
    let mut sete = Vec::new();
    for _ in 0..2_000 {
        let mut caras = 0;
        for _ in 0..7 {
            if rng.bernoulli(p_roosevelt) {
                caras += 1;
            }
        }
        sete.push(caras as f64 / 7.0 - p_roosevelt);
    }
    let desvio_sete = (sete.iter().map(|x| x * x).sum::<f64>() / 1999.0).sqrt();
    println!(
        "desvio de 2000 amostras aleatorias de 7 pessoas = {:.4} ({:.1} pontos); de 50 000 entrevistas = {:.4} ({:.2} pontos)",
        desvio_sete,
        100.0 * desvio_sete,
        sigma_y / 50_000f64.sqrt(),
        100.0 * sigma_y / 50_000f64.sqrt()
    );

    // (2) O filtro fabrica relações (Berkson).
    for corte in [1.0_f64, 1.5, 2.0] {
        let (pop, sel, frac) = berkson(2026, 400_000, corte);
        println!(
            "Berkson com corte {:.1}: correlacao na populacao = {:+.4}, entre selecionados ({:.1}% da populacao) = {:+.4}",
            corte, pop, frac, sel
        );
    }

    // (3) Simpson: as duas cirurgias de cálculo renal (Charig e colegas, 1986).
    let abertas_pequenas = 81.0 / 87.0;
    let outras_pequenas = 234.0 / 270.0;
    let abertas_grandes = 192.0 / 263.0;
    let outras_grandes = 55.0 / 80.0;
    let bruto_abertas = (81.0 + 192.0) / (87.0 + 263.0);
    let bruto_outras = (234.0 + 55.0) / (270.0 + 80.0);
    println!(
        "Simpson: pequenos {:.3} x {:.3}; grandes {:.3} x {:.3}; bruto {:.3} x {:.3} (inverte)",
        abertas_pequenas,
        outras_pequenas,
        abertas_grandes,
        outras_grandes,
        bruto_abertas,
        bruto_outras
    );
    let ponderado_abertas = 0.5 * abertas_pequenas + 0.5 * abertas_grandes;
    let ponderado_outras = 0.5 * outras_pequenas + 0.5 * outras_grandes;
    println!(
        "ponderado por uma populacao meio a meio: {:.3} x {:.3} (a aberta volta a vencer)",
        ponderado_abertas, ponderado_outras
    );

    // (4) A unidade independente.
    for p in [0.5_f64, 0.8, 0.235] {
        let (var, n_ef) = tamanho_efetivo(2026, p, 100, 20_000);
        println!(
            "cadeia de dois estados com persistencia {:.3}: variancia da media de 100 letras = {:.6} (independente: {:.6}), tamanho efetivo = {:.0}",
            p,
            var,
            0.25 / 100.0,
            n_ef
        );
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn berkson_induz_correlacao_negativa_e_persistencia_reduz_n_efetivo() {
        let (pop, sel, _) = berkson(2026, 20_000, 1.5);
        assert!(pop.abs() < 0.03);
        assert!(sel < -0.30);
        let (_, n_indep) = tamanho_efetivo(2026, 0.5, 100, 2_000);
        let (_, n_pers) = tamanho_efetivo(2026, 0.8, 100, 2_000);
        assert!(n_pers < n_indep * 0.4);
    }
}
