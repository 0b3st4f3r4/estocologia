//! Capítulo 10 — o tempo e a ruína: média de conjunto contra média no tempo.
//!
//! Laboratório autocontido, sem dependências externas.
//! Uso: cargo run --release

// ---------------------------------------------------------------- saida medida
#[derive(Clone)]
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn new(semente: u64) -> Self {
        SplitMix64(semente)
    }
    pub fn proximo(&mut self) -> u64 {
        stochology_core::rng::passo_splitmix64(&mut self.0)
    }
    pub fn uniforme(&mut self) -> f64 {
        (self.proximo() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn normal(&mut self) -> f64 {
        loop {
            let u = 2.0 * self.uniforme() - 1.0;
            let v = 2.0 * self.uniforme() - 1.0;
            let s = u * u + v * v;
            if s > 0.0 && s < 1.0 {
                return u * (-2.0 * s.ln() / s).sqrt();
            }
        }
    }
    /// Pareto de cauda P(X > x) = x^(-alfa), x >= 1.
    pub fn pareto(&mut self, alfa: f64) -> f64 {
        let u = self.uniforme().max(1e-300);
        u.powf(-1.0 / alfa)
    }
}

// ------------------------------------------------- a moeda de Peters

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Aposta {
    Tudo,
    Kelly,
    MeioKelly,
    Decimo,
}

impl Aposta {
    pub fn fracao(&self) -> f64 {
        match self {
            Aposta::Tudo => 1.0,
            Aposta::Kelly => 0.70,
            Aposta::MeioKelly => 0.35,
            Aposta::Decimo => 0.14,
        }
    }
    pub fn nome(&self) -> &'static str {
        match self {
            Aposta::Tudo => "aposta total (f = 1,00)",
            Aposta::Kelly => "Kelly cheio (f = 0,70)",
            Aposta::MeioKelly => "meio Kelly (f = 0,35)",
            Aposta::Decimo => "quarto de Kelly (f = 0,14)",
        }
    }
}

pub struct Coorte {
    pub nome: &'static str,
    pub acima_do_inicio: Vec<f64>,
    pub fracao_ruinada: f64,
    pub mediana_final: f64,
    pub media_final: f64,
}

/// Uma trajetória da moeda de Peters: cara multiplica por 1,5, coroa por 0,6.
/// A cada rodada arrisca a fração f da riqueza.
pub fn uma_trajetoria(rng: &mut SplitMix64, passos: usize, f: f64) -> Vec<f64> {
    let mut riqueza = 1.0;
    let mut curva = Vec::with_capacity(passos + 1);
    curva.push(riqueza);
    for _ in 0..passos {
        let m = if rng.uniforme() < 0.5 { 1.5 } else { 0.6 };
        riqueza *= 1.0 + f * (m - 1.0);
        curva.push(riqueza);
    }
    curva
}

pub fn coorte(semente: u64, trajetorias: usize, passos: usize, aposta: Aposta) -> Coorte {
    let mut rng = SplitMix64::new(semente);
    let marcos = [10usize, 100, 1000];
    let mut acima = vec![0usize; marcos.len()];
    let mut ruinados = 0usize;
    let mut finais = Vec::with_capacity(trajetorias);
    for _ in 0..trajetorias {
        let curva = uma_trajetoria(&mut rng, passos, aposta.fracao());
        for (i, m) in marcos.iter().enumerate() {
            if *m <= passos && curva[*m] > 1.0 {
                acima[i] += 1;
            }
        }
        let fim = *curva.last().unwrap();
        if fim < 1e-6 {
            ruinados += 1;
        }
        finais.push(fim);
    }
    finais.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = trajetorias as f64;
    Coorte {
        nome: aposta.nome(),
        acima_do_inicio: acima.iter().map(|c| *c as f64 / n).collect(),
        fracao_ruinada: ruinados as f64 / n,
        mediana_final: finais[trajetorias / 2],
        media_final: finais.iter().sum::<f64>() / n,
    }
}

/// Média da coorte (esperança) e média no tempo (log) por rodada, teóricas:
/// o pagamento `a` (cara) e `b` (coroa) multiplicam a riqueza.
pub fn taxas_da_moeda(p: f64, a: f64, b: f64) -> (f64, f64) {
    (p * a + (1.0 - p) * b, p * a.ln() + (1.0 - p) * b.ln())
}

/// Critério de Kelly para uma aposta de igual para igual com chance p: f* = 2p - 1.
pub fn kelly_igual_para_igual(p: f64) -> f64 {
    2.0 * p - 1.0
}

/// Fração ótima de Kelly para o pagamento 5:4 (cara +50%, coroa -40%):
/// f* = p|b| - (1-p)a = p(0,5 + 0,4) - 0,4 = 0,9p - 0,4: com p = 0,6, f* = 0,14.
pub fn kelly_moeda_de_peters(p: f64) -> f64 {
    p * 0.9 - 0.4
}

/// Média de conjunto e média no tempo da moeda de Peters com a fração f.
pub fn moeda_de_peters_com_f(p: f64, f: f64) -> (f64, f64) {
    let esperanca = 1.0 + f * (p * 1.5 + (1.0 - p) * 0.6 - 1.0);
    let log_tempo = (1.0 - p) * (1.0 - 0.4 * f).ln() + p * (1.0 + 0.5 * f).ln();
    (esperanca, log_tempo)
}

// ------------------------------------------------- o vendedor de seguro

/// O crescimento no tempo de quem aposta tudo na estratégia do vendedor de seguro:
/// ganha 1% por semana com probabilidade 1 - q e perde 50% com probabilidade q.
pub fn taxa_semanal(q: f64) -> f64 {
    (1.0 - q) * 1.01_f64.ln() + q * 0.5_f64.ln()
}

/// Uma coorte de investidores que estimam q pelo próprio histórico e alavancam.
/// Abaixo disto a carteira é considerada arruinada: 0,01% do capital inicial.
pub const RUINA: f64 = 1e-6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Regra {
    Frequencia,
    Laplace,
    MeioLaplace,
}

pub struct Seguradora {
    pub nome: &'static str,
    pub fracao_sem_perda: f64,
    pub fracao_aposta_total: f64,
    pub fracao_ruinada: f64,
    pub fracao_ruinada_em_1000: f64,
    pub mediana_final: f64,
    pub media_final: f64,
}

/// Duas regras de alavancagem sobre o mesmo histórico de 250 semanas:
/// `laplace = false` — a frequência observada entra direto no Kelly, zero perdas
/// estima q = 0 e manda apostar tudo (f = 1);
/// `laplace = true` — a regra de sucessão (s+1)/(n+2) impede o zero.
pub fn seguradoras(
    semente: u64,
    investidores: usize,
    semanas_hist: usize,
    passos: usize,
    q: f64,
    regra: Regra,
) -> Seguradora {
    let mut rng = SplitMix64::new(semente);
    let (mut sem_perda, mut aposta_total, mut ruinados, mut ruinados_1000) = (0usize, 0usize, 0usize, 0usize);
    let mut finais = Vec::with_capacity(investidores);
    for _ in 0..investidores {
        let mut perdas = 0usize;
        for _ in 0..semanas_hist {
            if rng.uniforme() < q {
                perdas += 1;
            }
        }
        if perdas == 0 {
            sem_perda += 1;
        }
        let q_est = if regra == Regra::Laplace || regra == Regra::MeioLaplace {
            (perdas as f64 + 1.0) / (semanas_hist as f64 + 2.0)
        } else {
            perdas as f64 / semanas_hist as f64
        };
        let mu = 0.01;
        let mut f = (((1.0 - q_est) * mu - q_est * 0.5) / mu).clamp(0.0, 1.0);
        if regra == Regra::MeioLaplace {
            f *= 0.5;
        }
        if f >= 0.999 {
            aposta_total += 1;
        }
        let mut riqueza = 1.0;
        for _ in 0..passos {
            if rng.uniforme() < q {
                riqueza *= 1.0 - f * 0.5;
            } else {
                riqueza *= 1.0 + f * mu;
            }
        }
        if riqueza < RUINA {
            ruinados += 1;
            ruinados_1000 += 1;
        }
        finais.push(riqueza);
    }
    finais.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = investidores as f64;
    Seguradora {
        nome: match regra {
            Regra::Frequencia => "frequência observada + Kelly pleno",
            Regra::Laplace => "Laplace + Kelly pleno",
            Regra::MeioLaplace => "Laplace + meio Kelly",
        },
        fracao_sem_perda: sem_perda as f64 / n,
        fracao_aposta_total: aposta_total as f64 / n,
        fracao_ruinada: ruinados as f64 / n,
        fracao_ruinada_em_1000: ruinados_1000 as f64 / n,
        mediana_final: finais[investidores / 2],
        media_final: finais.iter().sum::<f64>() / n,
    }
}

// ------------------------------------------------- a cauda selvagem

pub struct AmostraPareto {
    pub n: usize,
    pub alfa: f64,
    pub media: f64,
    pub mediana: f64,
    pub fracao_media_abaixo_do_verdadeiro: f64,
    pub cobertura_do_intervalo_de_95: f64,
    pub fracao_da_media_acima_do_maior_visto: f64,
}

/// Sorteia `repeticoes` amostras de tamanho `n` de um Pareto com P(X > x) = x^(-alfa).
pub fn pareto_amostras(semente: u64, n: usize, alfa: f64, repeticoes: usize) -> AmostraPareto {
    let mut rng = SplitMix64::new(semente);
    let media_verdadeira = if alfa > 1.0 { alfa / (alfa - 1.0) } else { f64::INFINITY };
    let mut medias = Vec::with_capacity(repeticoes);
    let (mut abaixo, mut cobre) = (0usize, 0usize);
    let mut fracao_dominada_pelo_maior = 0.0;
    for _ in 0..repeticoes {
        let mut xs = Vec::with_capacity(n);
        for _ in 0..n {
            xs.push(rng.pareto(alfa));
        }
        let media = xs.iter().sum::<f64>() / n as f64;
        let var = xs.iter().map(|x| (x - media).powi(2)).sum::<f64>() / (n as f64 - 1.0);
        let ep = (var / n as f64).sqrt();
        if media < media_verdadeira {
            abaixo += 1;
        }
        if media - 1.96 * ep <= media_verdadeira && media_verdadeira <= media + 1.96 * ep {
            cobre += 1;
        }
        let maior = xs.iter().cloned().fold(f64::MIN, f64::max);
        let acima_do_maior: f64 = xs.iter().filter(|x| **x > maior).sum();
        fracao_dominada_pelo_maior += acima_do_maior / (media * n as f64);
        medias.push(media);
    }
    medias.sort_by(|a, b| a.partial_cmp(b).unwrap());
    AmostraPareto {
        n,
        alfa,
        media: medias.iter().sum::<f64>() / repeticoes as f64,
        mediana: medias[repeticoes / 2],
        fracao_media_abaixo_do_verdadeiro: abaixo as f64 / repeticoes as f64,
        cobertura_do_intervalo_de_95: cobre as f64 / repeticoes as f64,
        fracao_da_media_acima_do_maior_visto: fracao_dominada_pelo_maior / repeticoes as f64,
    }
}

/// Estimador de Hill com os `k` maiores valores: alpha_hat = k / soma dos logaritmos.
pub fn hill(xs: &[f64], k: usize) -> f64 {
    let mut ordenado = xs.to_vec();
    ordenado.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let limiar = ordenado[k - 1];
    let soma: f64 = ordenado[..k].iter().map(|x| (x / limiar).ln()).sum();
    k as f64 / soma
}

/// Gera uma amostra de Pareto e devolve o Hill para vários k, com o intervalo
/// assintótico alpha_hat (1 +- 1,96/sqrt(k)).
pub fn hill_por_k(semente: u64, n: usize, alfa: f64, ks: &[usize]) -> Vec<(usize, f64, f64, f64)> {
    let mut rng = SplitMix64::new(semente);
    let xs: Vec<f64> = (0..n).map(|_| rng.pareto(alfa)).collect();
    ks.iter()
        .map(|k| {
            let h = hill(&xs, *k);
            let erro = 1.96 / (*k as f64).sqrt();
            (*k, h, h * (1.0 - erro), h * (1.0 + erro))
        })
        .collect()
}

// ------------------------------------------------- crescimento no tempo

/// Taxa de crescimento por período de uma aposta binária: com probabilidade p a
/// riqueza multiplica por (1 + ganho) e com (1 - p) por (1 - perda).
pub fn crescimento_no_tempo(p: f64, ganho: f64, perda: f64) -> f64 {
    p * (1.0 + ganho).ln() + (1.0 - p) * (1.0 - perda).ln()
}

// ------------------------------------------------- arrasto da volatilidade

pub struct Carteira {
    pub alavancagem: f64,
    pub media_aritmetica: f64,
    pub crescimento_temporal: f64,
    pub arrasto: f64,
    pub riqueza_final: f64,
}

/// Gera uma série diária de retornos normais e mede carteiras alavancadas em 1x, 2x
/// e 3x, rebalanceadas todo dia: a riqueza multiplica por (1 + L r) a cada sessão.
pub fn alavancagem_longa(semente: u64, sessoes: usize, mu: f64, sigma: f64) -> Vec<Carteira> {
    let mut rng = SplitMix64::new(semente);
    let retornos: Vec<f64> = (0..sessoes).map(|_| mu + sigma * rng.normal()).collect();
    [1.0, 2.0, 3.0]
        .iter()
        .map(|l| {
            let mut riqueza = 1.0;
            let mut soma_log = 0.0;
            for r in &retornos {
                let fator = 1.0 + l * r;
                riqueza *= fator;
                soma_log += fator.ln();
            }
            Carteira {
                alavancagem: *l,
                media_aritmetica: *l * mu,
                crescimento_temporal: soma_log / sessoes as f64,
                arrasto: *l * mu - soma_log / sessoes as f64,
                riqueza_final: riqueza,
            }
        })
        .collect()
}

/// A regra do arrasto da volatilidade: mu - sigma^2/2 por período.
pub fn arrasto_teorico(mu: f64, sigma: f64) -> (f64, f64) {
    (mu - 0.5 * sigma * sigma, 0.5 * sigma * sigma)
}




fn moeda_de_peters() {
    println!("== Capítulo 10 :: a moeda de Peters (cara +50%, coroa -40%) ==");
    let (esperanca, log_tempo) = taxas_da_moeda(0.5, 1.5, 0.6);
    let f_otimo = kelly_moeda_de_peters(0.6);
    let (_, g_otimo) = moeda_de_peters_com_f(0.6, f_otimo);
    println!(
        "média de conjunto por rodada = {:.4}  ({:+.2}% por rodada)",
        esperanca,
        100.0 * (esperanca - 1.0)
    );
    println!(
        "média no tempo por rodada  = {:.6}  ({:+.3}% por rodada)",
        log_tempo,
        100.0 * log_tempo
    );
    println!(
        "produto de uma cara e uma coroa: 1,5 x 0,6 = {:.2}",
        1.5 * 0.6
    );
    println!(
        "Kelly cheio (5:4): f* = {:.4} da riqueza; com pagamento de igual para igual, 2p - 1 = {:.2}",
        f_otimo,
        kelly_igual_para_igual(0.6)
    );
    println!(
        "crescimento no tempo com Kelly cheio: {:+.4} por rodada",
        g_otimo
    );
    println!();
    let apostas = [Aposta::Tudo, Aposta::Kelly, Aposta::MeioKelly, Aposta::Decimo];
    for a in apostas {
        let c = coorte(2026, 10_000, 1_000, a);
        println!(
            "{:<26} acima do início: 10 rodadas {:.3}  100 rodadas {:.3}  1000 rodadas {:.5}",
            c.nome, c.acima_do_inicio[0], c.acima_do_inicio[1], c.acima_do_inicio[2]
        );
        println!(
            "{:<26} ruína (< 1e-6 do capital) = {:.4}  mediana final = {:.3e}  média final = {:.3e}",
            "", c.fracao_ruinada, c.mediana_final, c.media_final
        );
    }
    println!();
    let c = coorte(2026, 10_000, 1_000, Aposta::Tudo);
    println!(
        "aposta total: log10 da mediana final = {:.1}, log10 da média final = {:.1}",
        c.mediana_final.log10(),
        c.media_final.log10()
    );
    println!(
        "parte da média que vem da trajetória mais rica (1% mais rico): {:.3}",
        fracao_da_media_no_topo(2026, 10_000, 1_000)
    );
    println!();
}

/// Fração da média da coorte que vem do 1% de trajetórias mais ricas.
fn fracao_da_media_no_topo(semente: u64, trajetorias: usize, passos: usize) -> f64 {
    let mut rng = SplitMix64::new(semente);
    let mut finais = Vec::with_capacity(trajetorias);
    for _ in 0..trajetorias {
        let c = uma_trajetoria(&mut rng, passos, 1.0);
        finais.push(*c.last().unwrap());
    }
    finais.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let total: f64 = finais.iter().sum();
    let topo: f64 = finais.iter().take(trajetorias / 100).sum();
    topo / total
}

fn vendedor_de_seguro() {
    println!("== Capítulo 10 :: o vendedor de seguro (ganha 1% por semana, perde 50% com q = 0,004) ==");
    let q = 0.004;
    println!(
        "crescimento no tempo da aposta total: {:+.5} por semana (negativo: a ruína é certa)",
        taxa_semanal(q)
    );
    for regra in [Regra::Frequencia, Regra::Laplace, Regra::MeioLaplace] {
        let s = seguradoras(2026, 10_000, 250, 1_000, q, regra);
        println!(
            "{:<32} sem perda = {:.3}  aposta total = {:.3}  ruina em 1000 semanas = {:.4}  mediana final = {:.3e}  média final = {:.3e}",
            s.nome, s.fracao_sem_perda, s.fracao_aposta_total, s.fracao_ruinada_em_1000, s.mediana_final, s.media_final
        );
    }
    let sem_perda_teorico = (1.0 - q).powi(250);
    println!("probabilidade teórica de 250 semanas sem perda = {:.4}", sem_perda_teorico);
    println!(
        "Laplace com zero perdas em 250 semanas: q_est = 1/252 = {:.6}",
        1.0 / 252.0
    );
    println!();
}

fn cauda_selvagem() {
    println!("== Capítulo 10 :: a cauda selvagem: Pareto com alfa = 1,1 ==");
    let media_verdadeira = 1.1 / 0.1;
    println!("média verdadeira (alfa/(alfa-1)) = {:.1}", media_verdadeira);
    for n in [1_000usize, 1_000_000] {
        let a = pareto_amostras(99, n, 1.1, 200);
        println!(
            "n = {:>9}: média amostral média = {:>8.2}  mediana = {:>7.2}  abaixo de 11 em {:.2}  IC95 cobre em {:.2}",
            a.n, a.media, a.mediana, a.fracao_media_abaixo_do_verdadeiro, a.cobertura_do_intervalo_de_95
        );
    }
    println!(
        "fração da média que vem de valores acima do maior visto: {:.3}",
        pareto_amostras(99, 1_000_000, 1.1, 200).fracao_da_media_acima_do_maior_visto
    );
    println!(
        "em mediana, {:.2}% da média verdadeira vêm de valores acima do máximo de uma amostra de um milhão",
        100.0 * (1.0 / 1_000_000f64).powf(0.1)
    );
    println!();
    println!("estimador de Hill com alfa = 1,1 e n = 200 000:");
    for (k, h, lo, hi) in hill_por_k(7, 200_000, 1.1, &[100, 500, 1_000, 5_000, 20_000]) {
        println!("  k = {:>6}  alfa_hat = {:.3}  IC95 aprox [{:.3}; {:.3}]", k, h, lo, hi);
    }
    println!(
        "gregarina: um modelo gaussiano ajustado a essa cauda subestima a perda em todas as repetições"
    );
    println!();
}

fn alavancagem_do_dow() {
    println!("== Capítulo 10 :: o arrasto da volatilidade em 1x, 2x e 3x ==");
    // Calibracao pelo arrasto do H1 (h1-real/README.md):
    //   1/2 * sigma^2 * d = 0,14 ao ano em 3x, e g1 = 0,051 ao ano em 1x.
    //   Com d = 252 sessoes: 4,5 sigma^2 = 0,14 -> sigma = 0,01518;
    //   1x: m d - 1/2 sigma^2 d = 0,051 -> m d = 0,080 -> m = 0,000317 por sessao.
    let mu = 0.000317;
    let sigma = 0.01518;
    let sessoes = 100_000;    for c in alavancagem_longa(2026, sessoes, mu, sigma) {
        println!(
            "L = {:.0}x: media aritmetica = {:+.6}/sessao, crescimento medido = {:+.6}/sessao, arrasto = {:+.6}/sessao ({:+.4}/ano), riqueza em {} sessoes = {:.3e}",
            c.alavancagem,
            c.media_aritmetica,
            c.crescimento_temporal,
            c.arrasto,
            c.arrasto * 252.0,
            sessoes,
            c.riqueza_final
        );
    }
    // a mesma coorte com horizonte curto, para ver a mediana contra a media
    for sessoes in [2_000usize, 10_000] {
        println!("horizonte de {} sessões:", sessoes);
        for c in alavancagem_longa(1909 + sessoes as u64, sessoes, mu, sigma) {
            println!(
                "  L = {:.0}x: crescimento medido = {:+.6}/sessao, riqueza final = {:.3e}",
                c.alavancagem, c.crescimento_temporal, c.riqueza_final
            );
        }
    }
    println!(
        "o crash de 1987 (-22,6% num dia) tira {:.1}% do capital em 3x, {:.1}% em 1x; a ruína comeca em L = {:.1}",
        100.0 * (1.0 - (1.0 - 3.0 * 0.226)),
        22.6,
        1.0 / 0.226
    );
    // O mesmo cálculo com dois choques absorventes plantados (o 19/10/1987 e
    // a corrida de 1929-32), que a série sintética não tem.
    let mut rng = SplitMix64::new(1987);
    let mut retornos: Vec<f64> = (0..20_000).map(|_| mu + sigma * rng.normal()).collect();
    retornos[5_000] = -0.226;
    for i in 5_100..5_140 {
        retornos[i] = -0.03;
    }
    println!("com um crash de -22,6% num dia e 40 dias de -3% (20 mil sessões):");
    for l in [1.0, 2.0, 3.0] {
        let mut riqueza = 1.0;
        let mut soma_log = 0.0;
        let mut ruina = false;
        for r in &retornos {
            let fator = 1.0 + l * r;
            if fator <= 0.0 {
                ruina = true;
                break;
            }
            riqueza *= fator;
            soma_log += fator.ln();
        }
        println!(
            "  L = {:.0}x: crescimento medido = {:+.6}/sessão, riqueza final = {:.3e}{}",
            l,
            soma_log / retornos.len() as f64,
            riqueza,
            if ruina { "  RUÍNA" } else { "" }
        );
    }
    println!();
}

pub fn rodar() {
    moeda_de_peters();
    vendedor_de_seguro();
    cauda_selvagem();
    alavancagem_do_dow();
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn moeda_de_peters_cresce_no_conjunto_e_decai_no_tempo() {
        let (conjunto, tempo) = moeda_de_peters_com_f(0.5, 1.0);
        assert!(conjunto > 0.0);
        assert!(tempo < 0.0);
        let f_otimo = kelly_moeda_de_peters(0.5);
        let (_, tempo_kelly) = moeda_de_peters_com_f(0.5, f_otimo);
        assert!(tempo_kelly > 0.0);
    }
}

