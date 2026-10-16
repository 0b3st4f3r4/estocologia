//! Capítulo 16 — H0: o posto de saúde, a espera e a unidade independente.
//!
//! Laboratório autocontido, sem dependências externas: SplitMix64 embutido, fila M/M/1 com um
//! atendente, recursão de Lindley, modelo nulo de metades e teste severo.
//! Crate de rascunho: ~/tmp/labs-cap16 (com [workspace] vazio no Cargo.toml).
//! Rodar: cargo run --release

const SEMENTE: u64 = 1909; // a mesma semente do Lab 0 do curso (Erlang, 1909)
const HORAS: f64 = 10.0; // o posto abre 10 h por dia
const MU: f64 = 12.0; // atendimentos por hora, um atendente
const DIAS: usize = 2000; // dias por mundo
const REPS: usize = 2000; // repartições do modelo nulo

#[derive(Clone)]
struct Sm64 {
    estado: u64,
}

impl Sm64 {
    fn novo(semente: u64) -> Self {
        Sm64 { estado: semente }
    }
    fn proximo_u64(&mut self) -> u64 {
        self.estado = self.estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.estado;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn uniforme(&mut self) -> f64 {
        (self.proximo_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

/// Poisson por inversão (Knuth): exato e suficiente para taxas de até ~110.
fn poisson(rng: &mut Sm64, lambda: f64) -> usize {
    let limite = (-lambda).exp();
    let (mut k, mut p) = (0usize, 1.0f64);
    loop {
        k += 1;
        p *= rng.uniforme();
        if p <= limite {
            return k - 1;
        }
    }
}

fn exponencial(rng: &mut Sm64, taxa: f64) -> f64 {
    -(1.0 - rng.uniforme()).ln() / taxa
}

/// Um dia do posto, aberto e vazio. Chegadas Poisson(lambda * HORAS) espalhadas no dia,
/// atendimento exponencial de média 1/mu, espera pela recursão de Lindley. Sai em minutos.
fn fila_do_dia(rng: &mut Sm64, lambda: f64, mu: f64) -> Vec<f64> {
    let n = poisson(rng, lambda * HORAS);
    let mut chegadas: Vec<f64> = (0..n).map(|_| rng.uniforme() * HORAS).collect();
    chegadas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let servico: Vec<f64> = (0..n).map(|_| exponencial(rng, mu)).collect();
    let mut espera = vec![0.0f64; n];
    for i in 1..n {
        espera[i] = (espera[i - 1] + servico[i - 1] - (chegadas[i] - chegadas[i - 1])).max(0.0);
    }
    espera.iter().map(|e| 60.0 * e).collect()
}

struct Mundo {
    esperas: Vec<f64>,
    soma_dia: Vec<f64>,
    n_dia: Vec<f64>,
}

fn mundo<F: FnMut(&mut Sm64) -> f64, G: Fn(f64) -> f64>(
    semente: u64,
    dias: usize,
    mut taxa_do_dia: F,
    mu_de: G,
) -> Mundo {
    let mut rng = Sm64::novo(semente);
    let mut esperas = Vec::new();
    let mut soma_dia = Vec::with_capacity(dias);
    let mut n_dia = Vec::with_capacity(dias);
    for _ in 0..dias {
        let lambda = taxa_do_dia(&mut rng);
        let e = fila_do_dia(&mut rng, lambda, mu_de(lambda));
        soma_dia.push(e.iter().sum::<f64>());
        n_dia.push(e.len() as f64);
        esperas.extend(e);
    }
    Mundo { esperas, soma_dia, n_dia }
}

fn media(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn desvio(x: &[f64]) -> f64 {
    let m = media(x);
    (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt()
}

fn quantil(x: &[f64], p: f64) -> f64 {
    let mut v = x.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((p * v.len() as f64).ceil() as usize).max(1).min(v.len()) - 1;
    v[i]
}

/// Margem do dia: erro-padrão da razão de somas, com o dia como unidade independente.
/// É a margem honesta quando os pacientes de um mesmo dia não são independentes.
fn margem_por_dia(x: &[f64], n: &[f64]) -> f64 {
    let soma: f64 = x.iter().sum();
    let total: f64 = n.iter().sum();
    let m = soma / total;
    let num: f64 = x
        .iter()
        .zip(n)
        .map(|(s, c)| (s - m * c) * (s - m * c))
        .sum();
    num.sqrt() / total
}

/// Margem por paciente: a pseudorreplicação que o modelo nulo põe à prova.
fn margem_por_paciente(e: &[f64]) -> f64 {
    desvio(e) / (e.len() as f64).sqrt()
}

fn linha_mundo(nome: &str, m: &Mundo) {
    let media_e = media(&m.esperas);
    let acima: f64 = m.esperas.iter().filter(|e| **e > 30.0).count() as f64 / m.esperas.len() as f64;
    println!(
        "{:<28} {:>8} {:>7.1} {:>7.1} {:>7.1} {:>7} {:>12.2} {:>15.2}",
        nome,
        m.esperas.len(),
        media_e,
        quantil(&m.esperas, 0.5),
        quantil(&m.esperas, 0.9),
        format!("{:.0}%", 100.0 * acima),
        margem_por_dia(&m.soma_dia, &m.n_dia),
        margem_por_paciente(&m.esperas),
    );
}

pub fn rodar() {
    println!("== Capítulo 16 :: a mesma média, duas esperas ==");
    println!(
        "{} dias de {:.0} h por mundo, um atendente, mu = {:.0} por hora, semente {} (+ índice do mundo)",
        DIAS, HORAS, MU, SEMENTE
    );

    // (1) os cinco mundos do Lab 0, todos com demanda média 8
    let alegacao = mundo(SEMENTE + 10, DIAS, |_| 8.0, |_| MU);
    let variavel = mundo(
        SEMENTE + 20,
        DIAS,
        |r| if r.uniforme() < 0.5 { 5.0 } else { 11.0 },
        |_| MU,
    );
    let quase = mundo(
        SEMENTE + 30,
        DIAS,
        |r| if r.uniforme() < 0.5 { 7.0 } else { 9.0 },
        |_| MU,
    );
    let uniforme = mundo(SEMENTE + 40, DIAS, |r| 5.0 + 6.0 * r.uniforme(), |_| MU);
    let reforco = mundo(
        SEMENTE + 50,
        DIAS,
        |r| if r.uniforme() < 0.5 { 5.0 } else { 11.0 },
        |l| if l > 8.0 { 24.0 } else { 12.0 },
    );

    println!(
        "{:<28} {:>8} {:>7} {:>7} {:>7} {:>7} {:>12} {:>15}",
        "mundo (pacientes por hora)", "pacientes", "espera", "mediana", "p90", ">30min", "margem/dia", "margem/pac."
    );
    linha_mundo("8 todo dia (a alegacao)", &alegacao);
    linha_mundo("7 ou 9", &quase);
    linha_mundo("uniforme entre 5 e 11", &uniforme);
    linha_mundo("5 ou 11", &variavel);
    linha_mundo("5 ou 11, reforco no surto", &reforco);

    // (2) o modelo nulo de metades: 4000 dias de lambda = 8, repartidos ao acaso em duas metades
    let nulo = mundo(SEMENTE + 60, 2 * DIAS, |_| 8.0, |_| MU);
    let mut rng = Sm64::novo(SEMENTE + 61);
    let mut difs = Vec::with_capacity(REPS);
    for _ in 0..REPS {
        let (mut sa, mut na, mut sb, mut nb) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for d in 0..nulo.soma_dia.len() {
            if rng.uniforme() < 0.5 {
                sa += nulo.soma_dia[d];
                na += nulo.n_dia[d];
            } else {
                sb += nulo.soma_dia[d];
                nb += nulo.n_dia[d];
            }
        }
        difs.push(sa / na - sb / nb);
    }
    let sd_difs = desvio(&difs);
    let max_abs = difs.iter().fold(0.0f64, |m, d| m.max(d.abs()));
    let n_pac = nulo.esperas.len() as f64;
    let previsto_dia = desvio(
        &nulo
            .soma_dia
            .iter()
            .zip(&nulo.n_dia)
            .map(|(s, c)| s / c)
            .collect::<Vec<f64>>(),
    ) * (2.0 / DIAS as f64).sqrt();
    let previsto_pac = desvio(&nulo.esperas) * (2.0 / (n_pac / 2.0)).sqrt();

    println!();
    println!(
        "nulo de metades: {} dias de demanda 8, {} reparticoes de {} dias contra {}",
        2 * DIAS,
        REPS,
        DIAS,
        DIAS
    );
    println!("desvio medido das diferencas = {:.4} min", sd_difs);
    println!("previsto pela unidade dia     = {:.4} min", previsto_dia);
    println!("previsto pela unidade paciente= {:.4} min  ({} pacientes)", previsto_pac, n_pac as usize);
    println!(
        "nulo: max |diferenca| em {} reparticoes = {:.3} min",
        REPS, max_abs
    );

    // (3) teste severo: a diferença entre os dois mundos contra o nulo
    let obs = media(&variavel.esperas) - media(&alegacao.esperas);
    let acima_obs = difs.iter().filter(|d| d.abs() >= obs.abs()).count() as f64 / REPS as f64;
    println!();
    println!(
        "teste severo: diferenca observada = {:.1} min, {:.1} desvios do nulo, fracao do nulo >= observado = {:.4}",
        obs,
        obs / sd_difs,
        acima_obs
    );
    println!(
        "cauda: 8 em 100 pacientes esperam mais de 30 min no mundo 8; {:.0} em 100 no mundo 5 ou 11",
        100.0 * variavel.esperas.iter().filter(|e| **e > 30.0).count() as f64 / variavel.esperas.len() as f64
    );
}
