//! Capítulo 8 — O termômetro do adivinho: escores próprios, decomposição de Murphy
//! e a tolerância do ECE que depende de n.
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
        self.estado = self.estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.estado;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
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

fn sigmoide(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn logit(p: f64) -> f64 {
    let p = p.clamp(1e-6, 1.0 - 1e-6);
    (p / (1.0 - p)).ln()
}

/// Os pedaços da decomposição de Murphy e o ECE de dez caixas de massa igual.
struct Decomposicao {
    brier: f64,
    rel: f64,
    res: f64,
    unc: f64,
    ece: f64,
    /// Brier da previsão substituída pela média da sua caixa; é nesta que a
    /// identidade de Murphy fecha exatamente.
    brier_caixa: f64,
}

fn decompor(previsoes: &[f64], desfechos: &[f64]) -> Decomposicao {
    let n = previsoes.len();
    let base = desfechos.iter().sum::<f64>() / n as f64;
    let brier = previsoes
        .iter()
        .zip(desfechos.iter())
        .map(|(p, y)| (p - y) * (p - y))
        .sum::<f64>()
        / n as f64;
    let mut ordem: Vec<usize> = (0..n).collect();
    ordem.sort_by(|a, b| previsoes[*a].partial_cmp(&previsoes[*b]).unwrap());
    let caixas = 10usize;
    let mut rel = 0.0;
    let mut res = 0.0;
    let mut ece = 0.0;
    let mut dentro = 0.0;
    for k in 0..caixas {
        let inicio = k * n / caixas;
        let fim = (k + 1) * n / caixas;
        if fim <= inicio {
            continue;
        }
        let idx = &ordem[inicio..fim];
        let m = idx.len() as f64;
        let pm = idx.iter().map(|i| previsoes[*i]).sum::<f64>() / m;
        let ym = idx.iter().map(|i| desfechos[*i]).sum::<f64>() / m;
        rel += m / n as f64 * (pm - ym) * (pm - ym);
        res += m / n as f64 * (ym - base) * (ym - base);
        ece += m / n as f64 * (pm - ym).abs();
        for i in idx {
            let d = pm - desfechos[*i];
            dentro += d * d / n as f64;
        }
    }
    Decomposicao {
        brier,
        rel,
        res,
        unc: base * (1.0 - base),
        ece,
        brier_caixa: dentro,
    }
}


/// Ajuste de Platt: regressão logística de y sobre o logit da previsão.
fn platt(previsoes: &[f64], desfechos: &[f64]) -> (f64, f64) {
    // O logit entra padronizado: sem isso o Newton estoura quando as previsões
    // encostam em 0 e 1.
    let xs: Vec<f64> = previsoes.iter().map(|p| logit(*p)).collect();
    let media_x = xs.iter().sum::<f64>() / xs.len() as f64;
    let var_x = xs.iter().map(|x| (x - media_x) * (x - media_x)).sum::<f64>() / xs.len() as f64;
    let desvio_x = var_x.sqrt();
    // Previsor constante (o climatologista): só resta o intercepto, e a
    // recalibração devolve exatamente a taxa-base.
    if desvio_x < 1e-9 {
        let base = desfechos.iter().sum::<f64>() / desfechos.len() as f64;
        return (logit(base), 0.0);
    }
    let mut a = 0.0f64;
    let mut b = 1.0f64;
    for _ in 0..60 {
        let (mut g0, mut g1) = (0.0f64, 0.0f64);
        let (mut h00, mut h01, mut h11) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..previsoes.len() {
            let x = (xs[i] - media_x) / desvio_x;
            let q = sigmoide(a + b * x);
            let w = q * (1.0 - q);
            g0 += desfechos[i] - q;
            g1 += (desfechos[i] - q) * x;
            h00 += w;
            h01 += w * x;
            h11 += w * x * x;
        }
        let det = h00 * h11 - h01 * h01;
        if det.abs() < 1e-12 {
            break;
        }
        let da = (h11 * g0 - h01 * g1) / det;
        let db = (-h01 * g0 + h00 * g1) / det;
        a += da;
        b += db;
        if da.abs() + db.abs() < 1e-10 {
            break;
        }
    }
    // volta para a escala original: a' + b' * (x - m)/s = a + b * x
    (a - b * media_x / desvio_x, b / desvio_x)
}

fn recalibrar(previsoes: &[f64], a: f64, b: f64) -> Vec<f64> {
    previsoes
        .iter()
        .map(|p| sigmoide(a + b * logit(*p)))
        .collect()
}

/// Tolerância do ECE: 2 000 simulações de rótulos perfeitamente calibrados
/// com as mesmas previsões, e o quantil do ECE nulo.
fn tolerancia_ece(semente: u64, previsoes: &[f64], simulacoes: usize) -> (f64, f64, f64, f64, f64) {
    let mut rng = SplitMix64::novo(semente);
    let mut eces = Vec::with_capacity(simulacoes);
    for _ in 0..simulacoes {
        let desfechos: Vec<f64> = previsoes
            .iter()
            .map(|p| if rng.bernoulli(*p) { 1.0 } else { 0.0 })
            .collect();
        eces.push(decompor(previsoes, &desfechos).ece);
    }
    eces.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |f: f64| eces[((f * (eces.len() - 1) as f64).round()) as usize];
    let acima = 100.0 * eces.iter().filter(|e| **e > 0.03).count() as f64 / eces.len() as f64;
    (
        q(0.5),
        q(0.95),
        q(0.99),
        eces.iter().sum::<f64>() / eces.len() as f64,
        acima,
    )
}


pub fn rodar() {
    println!("== Capítulo 8 :: o termômetro do adivinho ==");

    // Os dois meteorologistas: o preguicoso diz 20% todo dia, o especialista
    // acompanha a chance real. Quatro mil dias, semente 2026.
    let mut rng = SplitMix64::novo(2026);
    let n = 4_000usize;
    let mut p_verdade = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    let mut preguicoso = Vec::with_capacity(n);
    let mut especialista = Vec::with_capacity(n);
    let mut confiante = Vec::with_capacity(n);
    let mut ruidoso = Vec::with_capacity(n);
    for _ in 0..n {
        // chance real do dia: logit-normal com media 20%
        let z = -1.386 + 1.1 * rng.normal();
        let p = sigmoide(z);
        p_verdade.push(p);
        y.push(if rng.bernoulli(p) { 1.0 } else { 0.0 });
        preguicoso.push(0.20);
        especialista.push(sigmoide(z + 0.6 * rng.normal()));
        // o confiante ve a mesma coisa que o especialista, mas arredonda o sinal para os extremos
        confiante.push(sigmoide(2.2 * (z + 0.6 * rng.normal())));
        ruidoso.push(sigmoide(z + 1.6 * rng.normal()));
    }
    let base = y.iter().sum::<f64>() / n as f64;
    println!(
        "{} dias, taxa-base observada = {:.3}, chance media declarada pelo especialista = {:.3}",
        n,
        base,
        especialista.iter().sum::<f64>() / n as f64
    );
    for (nome, p) in [
        ("preguicoso (0,20 sempre)", &preguicoso),
        ("especialista", &especialista),
        ("confiante demais", &confiante),
        ("ruidoso", &ruidoso),
    ] {
        let d = decompor(p, &y);
        let (a, b) = platt(p, &y);
        let depois = recalibrar(p, a, b);
        let dd = decompor(&depois, &y);
        println!(
            "{:>24}: Brier {:.4} | REL {:.4} RES {:.4} UNC {:.4} | ECE {:.4} | apos Platt: Brier {:.4}, ECE {:.4} (a = {:.2}, b = {:.2})",
            nome, d.brier, d.rel, d.res, d.unc, d.ece, dd.brier, dd.ece, a, b
        );
    }
    let d = decompor(&especialista, &y);
    println!(
        "identidade de Murphy no especialista: REL - RES + UNC = {:.7}, Brier das previsoes agrupadas nas caixas = {:.7}, diferenca = {:.2e}",
        d.rel - d.res + d.unc,
        d.brier_caixa,
        (d.rel - d.res + d.unc - d.brier_caixa).abs()
    );
    println!(
        "a previsao crua tem Brier {:.7}; agrupar em 10 caixas custa {:.2e}, e a identidade fecha",
        d.brier,
        d.brier_caixa - d.brier
    );


    // A tolerância do ECE depende de n: três cofres do H3.
    let mut rng = SplitMix64::novo(20261015);
    let casos: [(usize, f64, f64, &str); 3] = [
        (1_488, 0.50, 1.0, "prisoes Toronto 1997-2002"),
        (24_466, 0.0226, 1.6, "Rhode Island 2013-2015"),
        (104_489, 0.130, 1.3, "voos NY 2023"),
    ];
    for (tamanho, taxa, espalhamento, nome) in casos {
        let previsoes: Vec<f64> = (0..tamanho)
            .map(|_| sigmoide(logit(taxa) + espalhamento * rng.normal()))
            .collect();
        let (q50, q95, q99, media, acima) = tolerancia_ece(20261015, &previsoes, 2_000);
        println!(
            "{:>26}: n = {:>7} | q50 nulo {:.4} | q95 nulo {:.4} | q99 nulo {:.4} | media {:.4} | ECE nulo acima de 0,03 em {:.1}% das simulacoes",
            nome, tamanho, q50, q95, q99, media, acima
        );
    }
    // A mesma n, dois espalhamentos de previsao: a nula depende das previsoes.
    for espalhamento in [0.4_f64, 1.0, 2.0] {
        let previsoes: Vec<f64> = (0..1_488)
            .map(|_| sigmoide(logit(0.5) + espalhamento * rng.normal()))
            .collect();
        let (q50, q95, _, _, _) = tolerancia_ece(20261015, &previsoes, 2_000);
        println!(
            "n = 1 488, espalhamento {:.1}: q50 nulo {:.4}, q95 nulo {:.4}",
            espalhamento, q50, q95
        );
    }
}
