//! Capítulo 17 — H1, parte 1: o arrasto da volatilidade no Dow Jones e o experimento de LaLonde.
//!
//! Autocontido, sem dependências externas: lê as bases públicas do repositório e reproduz os números
//! publicados do H1 real — crescimento temporal, arrasto, coortes de 20 anos de início mensal, o choque
//! de 1987, o nulo de anos reamostrados e a decomposição crua contra o experimento sorteado do NSW.
//! Rodar da raiz do repositório:
//!     cargo run --release --bin labs -- --cap 17
//! (aceita três caminhos opcionais na execução isolada: o CSV do Dow e os CSVs do NSW e do CPS)

const ANO_FIM_TREINO: i32 = 1999; // o treino é 1885-1999; o cofre é 2000-2023
const HORIZONTE: i32 = 20; // anos por coorte ("quem segura por décadas")
const ALAVANCAGEM_ALEGADA: f64 = 3.0;
const N_NULO: usize = 2000; // reamostragens de anos para o mundo da alegação

#[derive(Clone)]
struct Sm64 {
    estado: u64,
}

impl Sm64 {
    fn novo(s: u64) -> Self {
        Sm64 { estado: s }
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

/// Dias desde 1970-01-01 (Howard Hinnant, days_from_civil).
fn dias_desde_1970(a: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { a - 1 } else { a };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn bissexto(a: i64) -> bool {
    (a % 4 == 0 && a % 100 != 0) || a % 400 == 0
}

fn dias_no_mes(a: i64, m: i64) -> i64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if bissexto(a) {
                29
            } else {
                28
            }
        }
    }
}

/// Lê o Dow Jones: rótulos de data e fechamentos. Colunas: rownames,date,value.
fn ler_dow(caminho: &str) -> (Vec<String>, Vec<f64>) {
    let texto = std::fs::read_to_string(caminho).expect("nao li o CSV do Dow");
    let mut rotulos = Vec::new();
    let mut valores = Vec::new();
    for (i, linha) in texto.lines().enumerate() {
        if i == 0 || linha.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = linha.split(',').collect();
        if c.len() < 3 {
            continue;
        }
        rotulos.push(c[1].trim().to_string());
        valores.push(c[2].trim().parse().unwrap());
    }
    (rotulos, valores)
}

/// Log-crescimento acumulado: c[k] é a soma de ln(1 + L r[i]) para i < k.
fn acumulado(r: &[f64], l: f64) -> Vec<f64> {
    let mut c = vec![0.0f64; r.len() + 1];
    for i in 0..r.len() {
        let y = 1.0 + l * r[i];
        c[i + 1] = c[i] + if y > 0.0 { y.ln() } else { f64::NEG_INFINITY };
    }
    c
}

/// Coortes que começam no primeiro pregão de cada mês e terminam no último pregão antes de (início + 20 anos).
/// Devolve o ln da riqueza final de cada coorte. Só usa sessões até `fim` (o treino).
fn coortes_mensais(datas: &[i64], rotulos: &[String], r: &[f64], fim: usize, l: f64) -> Vec<f64> {
    let c = acumulado(r, l);
    let mut out = Vec::new();
    for s in 1..fim {
        if rotulos[s - 1][..7] == rotulos[s][..7] {
            continue; // só o primeiro pregão do mês
        }
        let ano: i64 = rotulos[s][..4].parse().unwrap();
        let mes: i64 = rotulos[s][5..7].parse().unwrap();
        let dia: i64 = rotulos[s][8..10].parse().unwrap();
        let ano_alvo = ano + HORIZONTE as i64;
        let alvo = dias_desde_1970(ano_alvo, mes, dia.min(dias_no_mes(ano_alvo, mes)));
        if datas[fim - 1] < alvo {
            continue; // a coorte não caberia no treino
        }
        let mut f = s;
        for j in s..fim {
            if datas[j] <= alvo {
                f = j;
            } else {
                break;
            }
        }
        out.push(c[f] - c[s]);
    }
    out
}

/// Somas móveis de 20 anos de uma série anual: as coortes do teste severo.
fn coortes_anuais(serie: &[f64]) -> Vec<f64> {
    let h = HORIZONTE as usize;
    (0..serie.len().saturating_sub(h - 1))
        .map(|s| serie[s..s + h].iter().sum())
        .collect()
}

fn fracao_acima(coortes: &[f64]) -> f64 {
    coortes.iter().filter(|x| **x > 0.0).count() as f64 / coortes.len() as f64
}

/// Mundo da alegação: os anos do treino são reamostrados e a riqueza cresce 3 vezes a soma aritmética
/// anual, sem arrasto. Devolve (fração média, quantil 1%, p-valor contra o observado).
fn mundo_da_alegacao(anual_aritmetica: &[f64], observado: f64, reps: usize, semente: u64) -> (f64, f64, f64) {
    let mut rng = Sm64::novo(semente);
    let n = anual_aritmetica.len();
    let mut fracs = Vec::with_capacity(reps);
    for _ in 0..reps {
        let serie: Vec<f64> = (0..n)
            .map(|_| ALAVANCAGEM_ALEGADA * anual_aritmetica[(rng.uniforme() * n as f64) as usize % n])
            .collect();
        fracs.push(fracao_acima(&coortes_anuais(&serie)));
    }
    let p = fracs.iter().filter(|x| **x <= observado).count() as f64 / reps as f64;
    (media(&fracs), quantil(fracs, 0.01), p)
}

/// Uma pessoa do NSW ou do CPS: as colunas usadas são treat (2), re75 (10) e re78 (11).
struct Pessoa {
    tratado: bool,
    re75: f64,
    re78: f64,
}

fn ler_lalonde(caminho: &str) -> Vec<Pessoa> {
    let texto = std::fs::read_to_string(caminho).expect("nao li o CSV do NSW/CPS");
    let mut out = Vec::new();
    for (i, linha) in texto.lines().enumerate() {
        if i == 0 || linha.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = linha.split(',').collect();
        if c.len() < 12 {
            continue;
        }
        out.push(Pessoa {
            tratado: c[2].trim() == "1",
            re75: c[10].trim().parse().unwrap(),
            re78: c[11].trim().parse().unwrap(),
        });
    }
    out
}

fn renda(v: &[&Pessoa]) -> f64 {
    v.iter().map(|p| p.re78).sum::<f64>() / v.len() as f64
}

fn emprego(v: &[&Pessoa], ano: u8) -> f64 {
    let n = v
        .iter()
        .filter(|p| if ano == 75 { p.re75 > 0.0 } else { p.re78 > 0.0 })
        .count();
    n as f64 / v.len() as f64
}

/// Resolve o caminho do dado: base opcional em ESTOCOLOGIA_DADOS, senao o repositorio.
fn caminho_de_dados(padrao: &str) -> String {
    match std::env::var("ESTOCOLOGIA_DADOS") {
        Ok(base) => format!("{}/{}", base.trim_end_matches('/'), padrao),
        Err(_) => padrao.to_string(),
    }
}

pub fn rodar() {
    let csv_dow = caminho_de_dados("dados-reais/djia_1885_2023.csv");
    let csv_nsw = caminho_de_dados("dados-reais/nsw_experimental.csv");
    let csv_cps = caminho_de_dados("dados-reais/cps_observational.csv");

    println!("== Capítulo 17 :: arrasto da volatilidade e o viés de seleção ==");
    let (rotulos, precos) = ler_dow(&csv_dow);
    let datas: Vec<i64> = rotulos
        .iter()
        .map(|s| {
            dias_desde_1970(
                s[..4].parse().unwrap(),
                s[5..7].parse().unwrap(),
                s[8..10].parse().unwrap(),
            )
        })
        .collect();
    let r: Vec<f64> = (1..precos.len()).map(|i| precos[i] / precos[i - 1] - 1.0).collect();
    let anos: Vec<i32> = rotulos[1..].iter().map(|s| s[..4].parse().unwrap()).collect();
    let fim = anos.iter().position(|a| *a > ANO_FIM_TREINO).unwrap_or(anos.len());
    let mut anos_u: Vec<i32> = anos[..fim].to_vec();
    anos_u.dedup();
    let n_anos = anos_u.len() as f64;
    let rt = &r[..fim];

    println!(
        "treino 1885-1999: {} pregoes em {:.0} anos; serie inteira 1885-2023: {} pregoes",
        rt.len(),
        n_anos,
        r.len()
    );
    let mu_dia = media(rt);
    println!(
        "media aritmetica diaria = {:.9} ({:.4} ao ano), desvio diario = {:.6} ({:.4} ao ano)",
        mu_dia,
        mu_dia * rt.len() as f64 / n_anos,
        desvio(rt),
        desvio(rt) * (rt.len() as f64 / n_anos).sqrt()
    );

    println!();
    println!("crescimento temporal por ano e arrasto (coortes e trajetorias do treino)");
    let mut g = Vec::new();
    for l in [1.0f64, 2.0, ALAVANCAGEM_ALEGADA] {
        let gl = rt.iter().map(|x| (1.0 + l * x).ln()).sum::<f64>() / n_anos;
        g.push(gl);
        println!(
            "  L={:.0}: a soma aritmetica daria {:.4} ao ano; o crescimento realizado e {:.4}; arrasto {:.4}",
            l,
            l * mu_dia * rt.len() as f64 / n_anos,
            gl,
            l * mu_dia * rt.len() as f64 / n_anos - gl
        );
    }
    let (mut pior, mut ipior) = (0.0f64, 0usize);
    for (i, x) in rt.iter().enumerate() {
        if *x < pior {
            pior = *x;
            ipior = i;
        }
    }
    println!(
        "pior pregao: {:.4} ({:.2}%) em {}; perda com L=1 {:.3}, L=2 {:.3}, L=3 {:.3}; ruina diaria em L >= {:.2}",
        pior,
        100.0 * pior,
        rotulos[ipior + 1],
        -pior,
        -2.0 * pior,
        -ALAVANCAGEM_ALEGADA * pior,
        -1.0 / pior
    );

    for (nome, de, ate) in [
        ("ano civil 1929-1932", "1929-01-01", "1932-12-31"),
        ("pico a fundo, 1929-09-03 a 1932-07-08", "1929-09-03", "1932-07-08"),
    ] {
        let i0 = rotulos.iter().position(|s| s.as_str() >= de).unwrap();
        let i1 = rotulos.iter().rposition(|s| s.as_str() <= ate).unwrap();
        for l in [1.0f64, ALAVANCAGEM_ALEGADA] {
            let w: f64 = (i0..i1).map(|i| 1.0 + l * r[i]).product();
            println!(
                "  {} com L={:.0}: o capital termina em {:.5} do inicial ({:.1}% destruido)",
                nome,
                l,
                w,
                100.0 * (1.0 - w)
            );
        }
    }

    println!();
    println!("coortes de 20 anos com inicio no primeiro pregao de cada mes (treino):");
    println!("{:<6} {:>12} {:>14} {:>14} {:>16}", "L", "coortes", "mediana (x)", "media (x)", "acima do inicio");
    for l in [1.0f64, 2.0, ALAVANCAGEM_ALEGADA] {
        let co = coortes_mensais(&datas, &rotulos, &r, fim, l);
        let riquezas: Vec<f64> = co.iter().map(|x| x.exp()).collect();
        let n = co.len();
        println!(
            "{:<6.0} {:>12} {:>14.2} {:>14.1} {:>16.4}",
            l,
            n,
            quantil(co.clone(), 0.5).exp(),
            media(&riquezas),
            fracao_acima(&co)
        );
    }

    // ln-crescimento por ano-unidade (ano civil): a unidade do teste severo.
    let mut anos_serie: Vec<i32> = Vec::new();
    let mut g1: Vec<f64> = Vec::new();
    let mut g3: Vec<f64> = Vec::new();
    let mut a_arit: Vec<f64> = Vec::new();
    for i in 0..fim {
        if anos_serie.last() != Some(&anos[i]) {
            anos_serie.push(anos[i]);
            g1.push(0.0);
            g3.push(0.0);
            a_arit.push(0.0);
        }
        let j = g1.len() - 1;
        g1[j] += (1.0 + r[i]).ln();
        g3[j] += (1.0 + ALAVANCAGEM_ALEGADA * r[i]).ln();
        a_arit[j] += r[i];
    }
    println!();
    println!(
        "coortes por ano-unidade ({} anos, soma movel de {}): acima do inicio em 1x = {:.4}, em 3x = {:.4} ({} coortes)",
        anos_serie.len(),
        HORIZONTE,
        fracao_acima(&coortes_anuais(&g1)),
        fracao_acima(&coortes_anuais(&g3)),
        coortes_anuais(&g3).len()
    );
    let obs = fracao_acima(&coortes_anuais(&g3));
    let (media_nulo, q01, p) = mundo_da_alegacao(&a_arit, obs, N_NULO, 17_001);
    println!(
        "mundo da alegacao (anos do treino reamostrados, crescimento 3*soma aritmetica anual, sem arrasto):"
    );
    println!(
        "  {} replicas: fracao media das coortes acima do inicio = {:.4}, quantil 1% = {:.4}, p(fracao <= {:.4}) = {:.4}",
        N_NULO, media_nulo, q01, obs, p
    );

    let nsw = ler_lalonde(&csv_nsw);
    let cps = ler_lalonde(&csv_cps);
    let tratados: Vec<&Pessoa> = nsw.iter().filter(|p| p.tratado).collect();
    let controle: Vec<&Pessoa> = nsw.iter().filter(|p| !p.tratado).collect();
    let todos_cps: Vec<&Pessoa> = cps.iter().collect();
    println!();
    println!(
        "LaLonde: NSW com {} tratados e {} controles sorteados; CPS com {} observacoes",
        tratados.len(),
        controle.len(),
        cps.len()
    );
    println!("{:<40} {:>10} {:>14}", "comparacao (desfecho de 1978)", "emprego", "renda (US$)");
    println!(
        "{:<40} {:>10.3} {:>14.0}",
        "experimento sorteado (NSW trat. - NSW ctrl)",
        emprego(&tratados, 78) - emprego(&controle, 78),
        renda(&tratados) - renda(&controle)
    );
    println!(
        "{:<40} {:>10.3} {:>14.0}",
        "comparacao crua (NSW trat. - CPS)",
        emprego(&tratados, 78) - emprego(&todos_cps, 78),
        renda(&tratados) - renda(&todos_cps)
    );
    println!(
        "{:<40} {:>10.3} {:>14.0}",
        "vies de selecao implicado (cru - experimento)",
        (emprego(&tratados, 78) - emprego(&todos_cps, 78)) - (emprego(&tratados, 78) - emprego(&controle, 78)),
        (renda(&tratados) - renda(&todos_cps)) - (renda(&tratados) - renda(&controle))
    );
    println!(
        "niveis de emprego em 1978: NSW tratados {:.3}, NSW controles {:.3}, CPS {:.3}; em 100 pessoas: {:.0}, {:.0} e {:.0}",
        emprego(&tratados, 78),
        emprego(&controle, 78),
        emprego(&todos_cps, 78),
        100.0 * emprego(&tratados, 78),
        100.0 * emprego(&controle, 78),
        100.0 * emprego(&todos_cps, 78)
    );
    println!(
        "renda media de 1978 (US$): NSW tratados {:.0}, NSW controles {:.0}, CPS {:.0}",
        renda(&tratados),
        renda(&controle),
        renda(&todos_cps)
    );
    println!(
        "placebo de 1975 (emprego): NSW tratados {:.3}, NSW controles {:.3}, CPS {:.3}; diferenca crua tratados - CPS = {:.3}",
        emprego(&tratados, 75),
        emprego(&controle, 75),
        emprego(&todos_cps, 75),
        emprego(&tratados, 75) - emprego(&todos_cps, 75)
    );
}