//! Capítulo 18 — H1, parte 2: a cauda dos incêndios dinamarqueses e a arte de reprovar o próprio modelo.
//!
//! Autocontido, sem dependências externas: lê a base pública, estima Hill na cauda real e numa cauda
//! simulada, aplica a regra de Laplace à janela de 36 meses limpos e mede a evidência em bits.
//! Rodar da raiz do repositório:
//!     cargo run --release --bin labs -- --cap 18

const RETENCAO: f64 = 50.0; // camada "100 xs 50", em M DKK de 1985
const U_POT: f64 = 10.0; // limiar do ajuste POT do repositório
const N_EXC: f64 = 83.0; // excessos acima de U_POT no treino (h1-real/saida/c/resultados.json)
const SIGMA_POT: f64 = 7.387414319903844; // sigma do ajuste GPD publicado
const XI_POT: f64 = 0.4064637172276745; // xi do ajuste GPD publicado
const P_POT: f64 = 0.042961531409410614; // p mensal que a cauda do treino implica (publicado)
const ANOS_TREINO: i32 = 9; // 1980-1988
const JANELA: usize = 36; // meses limpos que a alegação lê
const P_DESPREZIVEL: f64 = 1.0 / 1200.0; // "risco desprezível": um mês atingido por século
const N_BOOT: usize = 300; // réplicas do bootstrap por blocos mensais

#[derive(Clone)]
struct Sm64 {
    estado: u64,
}

impl Sm64 {
    fn novo(s: u64) -> Self {
        Sm64 { estado: s }
    }
    fn proximo_u64(&mut self) -> u64 {
        stochology_core::rng::passo_splitmix64(&mut self.estado)
    }
    fn uniforme(&mut self) -> f64 {
        (self.proximo_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

/// Estimador de Hill com as k maiores observações positivas (delega a `stochology-h1`).
fn hill(x: &[f64], k: usize) -> f64 {
    let n_pos = x.iter().filter(|v| **v > 0.0).count();
    stochology_h1::hill(x, k.min(n_pos.saturating_sub(1)))
}

fn quantil(mut v: Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((p * v.len() as f64).ceil() as usize).max(1).min(v.len()) - 1;
    v[i]
}

fn media(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn desvio(x: &[f64]) -> f64 {
    let m = media(x);
    (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt()
}

/// Regra de sucessão de Laplace: (k+1)/(n+2).
fn laplace(k: usize, n: usize) -> f64 {
    (k as f64 + 1.0) / (n as f64 + 2.0)
}

/// Evidência de uma janela limpa de n meses: razão de verossimilhança "desprezível" contra "perigoso".
fn bits_evidencia(n: usize, p_perigoso: f64) -> (f64, f64, f64) {
    let p0_desprez = (1.0 - P_DESPREZIVEL).powi(n as i32);
    let p0_perigo = (1.0 - p_perigoso).powi(n as i32);
    let rv = p0_desprez / p0_perigo;
    (rv, rv.log2(), p0_perigo)
}

/// Informação mútua, em bits, entre "a janela saiu limpa?" e a hipótese 50/50 perigoso contra desprezível.
fn informacao_mutua(n: usize, p_perigoso: f64) -> f64 {
    let pc = [
        (1.0 - p_perigoso).powi(n as i32),
        (1.0 - P_DESPREZIVEL).powi(n as i32),
    ];
    let mut mi = 0.0;
    for (i, hip) in [0.5f64, 0.5].iter().enumerate() {
        for j in 0..2 {
            let p = hip * if j == 0 { pc[i] } else { 1.0 - pc[i] };
            let px = 0.5 * if j == 0 { pc[0] } else { 1.0 - pc[0] }
                + 0.5 * if j == 0 { pc[1] } else { 1.0 - pc[1] };
            if p > 0.0 {
                mi += p * (p / (0.5 * px)).log2();
            }
        }
    }
    mi
}

/// Meses limpos necessários para que "acusar o perigo" tenha probabilidade de 95%.
fn meses_para_95(p: f64) -> usize {
    (0.05f64.ln() / (1.0 - p).ln()).ceil() as usize
}

struct Dados {
    todas: Vec<f64>,
    meses: Vec<(String, f64)>,
    por_mes: Vec<Vec<f64>>,
}

/// Lê a base: perdas em ordem cronológica, maior perda de cada mês e a lista de perdas por mês.
fn ler_dados(caminho: &str) -> Dados {
    let texto = std::fs::read_to_string(caminho).expect("nao li o CSV dos incendios");
    let mut todas = Vec::new();
    let mut meses: Vec<(String, f64)> = Vec::new();
    let mut por_mes: Vec<Vec<f64>> = (0..(ANOS_TREINO * 12) as usize)
        .map(|_| Vec::new())
        .collect();
    for (i, linha) in texto.lines().enumerate() {
        if i == 0 || linha.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = linha.split(',').collect();
        let perda: f64 = c[1].trim().parse().unwrap();
        let mes = c[0].trim()[..7].to_string();
        todas.push(perda);
        match meses.last_mut() {
            Some((m, mx)) if *m == mes => {
                if perda > *mx {
                    *mx = perda;
                }
            }
            _ => meses.push((mes.clone(), perda)),
        }
        if &mes[..4] < "1989" {
            let idx = (mes[..4].parse::<i32>().unwrap() - 1980) as usize * 12
                + mes[5..7].parse::<usize>().unwrap()
                - 1;
            por_mes[idx].push(perda);
        }
    }
    Dados {
        todas,
        meses,
        por_mes,
    }
}

/// Meses do período, meses em que a camada foi atingida e a maior perda do período.
fn periodo(d: &Dados, a0: i32, a1: i32) -> (usize, usize, f64) {
    let do_periodo: Vec<&(String, f64)> = d
        .meses
        .iter()
        .filter(|(m, _)| {
            let ano: i32 = m[..4].parse().unwrap();
            ano >= a0 && ano <= a1
        })
        .collect();
    let atingidos = do_periodo.iter().filter(|(_, mx)| *mx > RETENCAO).count();
    let maior = do_periodo.iter().map(|(_, mx)| *mx).fold(0.0f64, f64::max);
    (do_periodo.len(), atingidos, maior)
}

/// Hill k=50 com os meses do treino reamostrados com reposição (bootstrap de blocos mensais).
fn bootstrap_hill(d: &Dados, k: usize, reps: usize, semente: u64) -> Vec<f64> {
    let mut rng = Sm64::novo(semente);
    let mut out = Vec::with_capacity(reps);
    for _ in 0..reps {
        let mut amostra = Vec::new();
        for _ in 0..d.por_mes.len() {
            let j = (rng.uniforme() * d.por_mes.len() as f64) as usize % d.por_mes.len();
            amostra.extend(d.por_mes[j].iter().cloned());
        }
        out.push(hill(&amostra, k));
    }
    out
}

/// Cauda simulada: Pareto de cauda alpha, mesmo tamanho amostral, Hill k em cada repetição.
fn cauda_simulada(alfa: f64, n: usize, k: usize, reps: usize, semente: u64) -> Vec<f64> {
    let mut rng = Sm64::novo(semente);
    let mut out = Vec::with_capacity(reps);
    for _ in 0..reps {
        let amostra: Vec<f64> = (0..n)
            .map(|_| (1.0 - rng.uniforme()).powf(-1.0 / alfa))
            .collect();
        out.push(hill(&amostra, k));
    }
    out
}

/// Resolve o caminho do dado: base opcional em ESTOCOLOGIA_DADOS, senao o repositorio.
fn caminho_de_dados(padrao: &str) -> String {
    match std::env::var("ESTOCOLOGIA_DADOS") {
        Ok(base) => format!("{}/{}", base.trim_end_matches('/'), padrao),
        Err(_) => padrao.to_string(),
    }
}

pub fn rodar() {
    let caminho = caminho_de_dados("dados-reais/danish_fire.csv");
    let d = ler_dados(&caminho);

    println!("== Capítulo 18 :: a cauda, a janela limpa e a informação ==");
    println!(
        "incendios da Copenhagen Re acima de 1 M DKK, 1980-1990: {} sinistros em {} meses",
        d.todas.len(),
        d.meses.len()
    );

    // (1) meses em que a camada 100 xs 50 foi atingida, por período
    println!();
    println!(
        "{:<12} {:>7} {:>17} {:>13}",
        "periodo", "meses", "meses atingidos", "maior perda"
    );
    for (nome, a0, a1) in [
        ("1980-1982", 1980, 1982),
        ("1983-1985", 1983, 1985),
        ("1986-1988", 1986, 1988),
        ("1989-1990", 1989, 1990),
    ] {
        let (m, at, maior) = periodo(&d, a0, a1);
        println!("{:<12} {:>7} {:>17} {:>13.1}", nome, m, at, maior);
    }
    print!("meses atingidos e valor:");
    for (m, mx) in d.meses.iter().filter(|(_, mx)| *mx > RETENCAO) {
        print!(" {} {:.1};", m, mx);
    }
    println!();

    // (2) Hill na cauda do treino (1980-1988) e o intervalo por bootstrap de blocos mensais
    let treino: Vec<f64> = d.por_mes.concat();
    println!();
    println!(
        "treino 1980-1988: {} sinistros em {} meses",
        treino.len(),
        ANOS_TREINO * 12
    );
    print!("Hill no treino:");
    for k in [25usize, 50, 100] {
        print!(" k={} alpha={:.4}", k, hill(&treino, k));
    }
    println!();
    let rep = bootstrap_hill(&d, 50, N_BOOT, 18_001);
    println!(
        "Hill k=50, IC 95% por bootstrap de blocos mensais ({} replicas): [{:.2}; {:.2}]",
        N_BOOT,
        quantil(rep.clone(), 0.025),
        quantil(rep, 0.975)
    );

    // (3) o mesmo estimador numa cauda simulada com o alpha estimado
    let alfa = 1.81f64;
    let sim = cauda_simulada(alfa, 1500, 50, 200, 18_002);
    println!(
        "Pareto simulado com alpha = {:.2}, n = 1500, 200 repeticoes: Hill k=50 medio = {:.3} (dp {:.3}), 90% das repeticoes entre {:.2} e {:.2}",
        alfa,
        media(&sim),
        desvio(&sim),
        quantil(sim.clone(), 0.05),
        quantil(sim, 0.95)
    );

    // (4) a janela limpa: Laplace, evidência em bits e meses para um teste severo
    println!();
    println!("janela de {} meses sem sinistro na camada:", JANELA);
    println!(
        "Laplace na janela: p = (0+1)/(36+2) = {:.5}, ou 1 chance em {:.0} por mes",
        laplace(0, JANELA),
        1.0 / laplace(0, JANELA)
    );
    println!(
        "frequencia mensal medida no treino: p = 5/108 = {:.5}",
        5.0 / 108.0
    );
    println!(
        "p mensal da cauda do treino (ajuste POT/GPD publicado: xi = {:.3}, sigma = {:.3}, {} excessos acima de {:.0}): p = {:.5}",
        XI_POT, SIGMA_POT, N_EXC as usize, U_POT, P_POT
    );
    for (nome, p) in [("frequencia medida", 5.0 / 108.0), ("cauda POT/GPD", P_POT)] {
        let (rv, bits, p0) = bits_evidencia(JANELA, p);
        println!(
            "  {:<18} p = {:.5}: P(janela limpa | perigoso) = {:.4}; razao de verossimilhanca = {:.2} ({:.2} bits); meses limpos para 95% = {}",
            nome,
            p,
            p0,
            rv,
            bits,
            meses_para_95(p)
        );
        println!(
            "  {:<18} informacao mutua janela x (perigoso vs desprezivel) = {:.4} bits",
            "",
            informacao_mutua(JANELA, p)
        );
    }
    let (rv_cofre, bits_cofre, _) = bits_evidencia(JANELA, 2.0 / 24.0);
    println!(
        "referencia: sob a taxa que o cofre mostrou (2 meses atingidos em 24, p = 1/12), a mesma janela limpa valeria {:.2} bits (razao {:.1})",
        bits_cofre, rv_cofre
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn laplace_evita_zero_e_hill_recupera_cauda() {
        assert!((laplace(0, 36) - 1.0 / 38.0).abs() < 1e-12);
        let (_, bits, _) = bits_evidencia(36, 5.0 / 108.0);
        assert!((bits - 2.42).abs() < 0.02);
        let xs = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0];
        assert!(hill(&xs, 3).is_finite());
    }
}
