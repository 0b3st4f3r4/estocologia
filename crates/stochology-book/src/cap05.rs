//! Capítulo 5 — Quando a média mente: convexidade, Jensen e o pedágio da variância.
//!
//! Laboratório autocontido: SplitMix64 embutido, sem dependências externas.
//! Semente fixa do posto de saúde: 1909 (a mesma do Lab 0 do curso).
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
    /// Uniforme em [0,1).
    fn uniforme(&mut self) -> f64 {
        (self.proximo() >> 11) as f64 / ((1u64 << 53) as f64)
    }
    /// Exponencial de média dada.
    fn exponencial(&mut self, media: f64) -> f64 {
        let u = self.uniforme().max(1e-18);
        -media * u.ln()
    }
}

/// Estatísticas de um mundo simulado.
#[derive(Clone)]
struct Medida {
    pacientes: usize,
    espera_media_min: f64,
    mediana_min: f64,
    p90_min: f64,
    acima_30_por_100: f64,
    esperas: Vec<f64>,
    dias: Vec<Vec<f64>>,
}

fn quantil(ordenadas: &[f64], q: f64) -> f64 {
    if ordenadas.is_empty() {
        return f64::NAN;
    }
    let pos = q * (ordenadas.len() as f64 - 1.0);
    let i = pos.floor() as usize;
    let j = (i + 1).min(ordenadas.len() - 1);
    let frac = pos - i as f64;
    ordenadas[i] * (1.0 - frac) + ordenadas[j] * frac
}


/// Simula o posto: um servidor com taxa mu pacientes/hora, dia de 10 horas
/// começando vazio, e uma taxa de chegada que pode mudar de dia para dia.
fn simular<F, G>(semente: u64, n_dias: usize, horas_por_dia: f64, mu_de: G, lambda_de: F) -> Medida
where
    F: Fn(usize) -> f64,
    G: Fn(usize) -> f64,
{
    let mut rng = SplitMix64::novo(semente);
    let mut esperas: Vec<f64> = Vec::new();
    let mut dias: Vec<Vec<f64>> = Vec::with_capacity(n_dias);
    for d in 0..n_dias {
        let lambda = lambda_de(d);
        let mu = mu_de(d);
        let mut chegadas = Vec::new();
        let mut t = 0.0;
        loop {
            t += rng.exponencial(60.0 / lambda);
            if t >= 60.0 * horas_por_dia {
                break;
            }
            chegadas.push(t);
        }
        let mut livre = 0.0f64;
        let mut do_dia: Vec<f64> = Vec::with_capacity(chegadas.len());
        for c in chegadas {
            let inicio = c.max(livre);
            let espera = inicio - c;
            livre = inicio + rng.exponencial(60.0 / mu);
            do_dia.push(espera);
        }
        esperas.extend_from_slice(&do_dia);
        dias.push(do_dia);
    }
    let mut ordenadas = esperas.clone();
    ordenadas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let media = esperas.iter().sum::<f64>() / esperas.len() as f64;
    let acima = esperas.iter().filter(|x| **x > 30.0).count() as f64;
    Medida {
        pacientes: esperas.len(),
        espera_media_min: media,
        mediana_min: quantil(&ordenadas, 0.5),
        p90_min: quantil(&ordenadas, 0.9),
        acima_30_por_100: 100.0 * acima / esperas.len() as f64,
        esperas,
        dias,
    }
}

/// Espera média de longo prazo com um atendente: W(lambda) = lambda/(mu(mu-lambda)), em horas.
fn w_formula(lambda: f64, mu: f64) -> f64 {
    60.0 * lambda / (mu * (mu - lambda))
}

fn variancia(xs: &[f64]) -> f64 {
    let m = xs.iter().sum::<f64>() / xs.len() as f64;
    xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0)
}

fn desvio(xs: &[f64]) -> f64 {
    variancia(xs).sqrt()
}


/// O nulo de metades: reparte os dias ao acaso em duas metades e mede o desvio
/// da diferença entre as médias (Φ2 do livro).
fn nulo_de_metades(m: &Medida, semente: u64) -> f64 {
    let mut rng = SplitMix64::novo(semente);
    let media_de = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let n_d = m.dias.len();
    let mut difs = Vec::with_capacity(2_000);
    for _ in 0..2_000 {
        let mut a: Vec<f64> = Vec::new();
        let mut b: Vec<f64> = Vec::new();
        for dia in m.dias.iter() {
            if rng.uniforme() < 0.5 {
                a.extend_from_slice(dia);
            } else {
                b.extend_from_slice(dia);
            }
        }
        difs.push(media_de(&a) - media_de(&b));
    }
    let _ = n_d;
    desvio(&difs)
}

pub fn rodar() {
    println!("== Capítulo 5 :: quando a média mente ==");

    let mu = 12.0;
    println!(
        "W(5) = {:.2} min, W(8) = {:.2} min, W(11) = {:.2} min",
        w_formula(5.0, mu),
        w_formula(8.0, mu),
        w_formula(11.0, mu)
    );
    let f_mean = w_formula(8.0, mu);
    let e_f_bimodal = 0.5 * w_formula(5.0, mu) + 0.5 * w_formula(11.0, mu);
    let var_bimodal = 9.0;
    let var_uniforme = 3.0;
    let e_f_uniforme = (5..=10)
        .map(|i| w_formula(i as f64 + 0.5, mu))
        .sum::<f64>()
        / 6.0;
    let f2 = 120.0 / (mu - 8.0).powi(3);
    println!(
        "Jensen: f(E[X]) = {:.2} min | E[f(X)] = {:.2} (5 ou 11) e {:.2} (uniforme 5..11)",
        f_mean, e_f_bimodal, e_f_uniforme
    );
    println!(
        "segunda ordem: meia f''(8) Var(X) = {:.2} min (5 ou 11) e {:.2} min (uniforme)",
        0.5 * f2 * var_bimodal,
        0.5 * f2 * var_uniforme
    );

    let semente = 1909u64;
    let dias = 4_000usize;
    let horas = 10.0;
    let constante = simular(semente, dias, horas, |_| mu, |_| 8.0);
    let bimodal = simular(
        semente,
        dias,
        horas,
        |_| mu,
        |d| if d % 2 == 0 { 5.0 } else { 11.0 },
    );
    let uniforme = simular(semente, dias, horas, |_| mu, |d| 5.0 + ((d * 7919) % 7) as f64);
    let reforco = simular(
        semente,
        dias,
        horas,
        |d| if d % 2 == 0 { mu } else { 2.0 * mu },
        |d| if d % 2 == 0 { 5.0 } else { 11.0 },
    );

    for (nome, m) in [
        ("8 todo dia", &constante),
        ("5 ou 11", &bimodal),
        ("uniforme 5..11", &uniforme),
        ("5 ou 11, reforco no surto", &reforco),
    ] {
        println!(
            "{:>28}: {:>7} pacientes | media {:>5.1} min | mediana {:>5.1} | p90 {:>5.1} | >30 min: {:>5.1} em 100",
            nome, m.pacientes, m.espera_media_min, m.mediana_min, m.p90_min, m.acima_30_por_100
        );
    }


    let surto: Vec<f64> = bimodal
        .dias
        .iter()
        .enumerate()
        .filter(|(d, _)| d % 2 == 1)
        .flat_map(|(_, v)| v.iter().copied())
        .collect();
    let leve: Vec<f64> = bimodal
        .dias
        .iter()
        .enumerate()
        .filter(|(d, _)| d % 2 == 0)
        .flat_map(|(_, v)| v.iter().copied())
        .collect();
    let media_de = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    println!(
        "nos dias de surto: {:.1} min em {} pacientes ({:.1}% do total); nos dias leves: {:.1} min",
        media_de(&surto),
        surto.len(),
        100.0 * surto.len() as f64 / bimodal.pacientes as f64,
        media_de(&leve)
    );

    let nulo_constante = nulo_de_metades(&constante, 2026);
    let nulo_bimodal = nulo_de_metades(&bimodal, 2026);
    let n_d = constante.dias.len();
    for (nome, m) in [("8 todo dia", &constante), ("5 ou 11", &bimodal)] {
        let media_dia: Vec<f64> = m.dias.iter().map(|d| media_de(d)).collect();
        let margem_paciente = (variancia(&m.esperas) * 2.0 / (m.pacientes as f64 / 2.0)).sqrt();
        let margem_dia = (variancia(&media_dia) * 2.0 / (n_d as f64 / 2.0)).sqrt();
        let nulo = if nome == "8 todo dia" {
            nulo_constante
        } else {
            nulo_bimodal
        };
        println!(
            "nulo de metades no mundo {:>11}: desvio medido {:.3} min | margem por dia {:.3} | por paciente {:.3} | variancia da espera {:.0} min^2",
            nome, nulo, margem_dia, margem_paciente, variancia(&m.esperas)
        );
    }
    println!(
        "diferenca entre os mundos = {:.1} min = {:.1} desvios do nulo",
        bimodal.espera_media_min - constante.espera_media_min,
        (bimodal.espera_media_min - constante.espera_media_min) / nulo_constante
    );
    println!(
        "mediana do mundo 5 ou 11 = {:.1} min, contra a espera media de {:.1} min do mundo constante",
        bimodal.mediana_min, constante.espera_media_min
    );


    let mut rng = SplitMix64::novo(1909);
    let mut soma = 0.0;
    let mut abaixo = 0usize;
    let n = 100_000usize;
    for _ in 0..n {
        let u = rng.uniforme();
        let centro = if u < 0.45 { 4.0 } else { 8.0 };
        let u1 = rng.uniforme().max(1e-18);
        let u2 = rng.uniforme();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let nota = centro + z;
        soma += nota;
        if nota < 6.0 {
            abaixo += 1;
        }
    }
    println!(
        "turma de {} alunos: media = {:.2}, reprovados (nota < 6,0) = {:.1} em 100",
        n,
        soma / n as f64,
        100.0 * abaixo as f64 / n as f64
    );
}
