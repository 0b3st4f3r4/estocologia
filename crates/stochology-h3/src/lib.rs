//! stochology-h3 — a malha contínua: relógio estrito, calibração e o laço performativo.
//!
//! O H3 e o horizonte em que tudo corre ao vivo: os seis fluxos sob relógio estrito, a ECE do cofre,
//! a habilidade de Brier e o laco performativo (o que o agente faz com o indicador depois que passa
//! a ser cobrado por ele). A terceira proibicao do contrato vive aqui: rodar com e sem a acao.
//!
//! Declarado e nao implementado: a leitura dos seis fluxos reais e o cofre de Rhode Island, que
//! seguem como registro; o que existe aqui e o mecanismo, nao a base.

pub mod preteste;

use stochology_core::Rng;

fn uniforme(rng: &mut Rng) -> f64 {
    ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn normal(rng: &mut Rng) -> f64 {
    let u1 = uniforme(rng);
    let u2 = uniforme(rng);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Erro de calibracao esperado, por baldes de confianca. Cai com o tamanho da amostra.
pub fn ece(confianca: &[f64], acerto: &[bool], baldes: usize) -> f64 {
    if confianca.len() != acerto.len() || confianca.is_empty() || baldes == 0 {
        return f64::NAN;
    }
    let mut soma = vec![0.0_f64; baldes];
    let mut acertos = vec![0.0_f64; baldes];
    let mut contagem = vec![0usize; baldes];
    for (p, y) in confianca.iter().zip(acerto.iter()) {
        let indice = ((p.clamp(0.0, 1.0) * baldes as f64).floor() as usize).min(baldes - 1);
        soma[indice] += p;
        acertos[indice] += if *y { 1.0 } else { 0.0 };
        contagem[indice] += 1;
    }
    let n = confianca.len() as f64;
    let mut ece = 0.0;
    for indice in 0..baldes {
        if contagem[indice] == 0 {
            continue;
        }
        let media_confianca = soma[indice] / contagem[indice] as f64;
        let media_acerto = acertos[indice] / contagem[indice] as f64;
        ece += (contagem[indice] as f64 / n) * (media_acerto - media_confianca).abs();
    }
    ece
}

/// Habilidade de Brier contra a climatologia declarada (a frequencia de base).
pub fn habilidade_de_brier(previsoes: &[f64], observado: &[bool]) -> f64 {
    if previsoes.len() != observado.len() || previsoes.is_empty() {
        return f64::NAN;
    }
    let n = previsoes.len() as f64;
    let base = observado.iter().filter(|y| **y).count() as f64 / n;
    let brier = previsoes
        .iter()
        .zip(observado.iter())
        .map(|(p, y)| (p - if *y { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        / n;
    let brier_base = observado
        .iter()
        .map(|y| (base - if *y { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        / n;
    if brier_base == 0.0 {
        return 0.0;
    }
    1.0 - brier / brier_base
}

/// Quantos eventos leem o futuro: a leitura nao pode passar do relogio.
pub fn vazamentos(relogios: &[u64], leituras: &[u64]) -> usize {
    relogios
        .iter()
        .zip(leituras.iter())
        .filter(|(relogio, leitura)| leitura > relogio)
        .count()
}

/// O canario plantado de proposito: se ele disparar, havia vazamento.
pub fn canario_plantado(relogios: &[u64], leituras: &[u64]) -> bool {
    vazamentos(relogios, leituras) > 0
}

/// Fluxo sob relogio estrito: a leitura de cada evento e o proprio instante (uma unidade de atraso).
pub fn fluxo_estrito(semente: u64, n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut rng = Rng::novo(semente);
    let mut relogios = Vec::with_capacity(n);
    let mut instante = 0u64;
    for _ in 0..n {
        instante += 1 + rng.uniforme(3);
        relogios.push(instante);
    }
    let leituras = relogios.clone();
    (relogios, leituras)
}

/// O laco performativo: o ganho no indicador contra o ganho no alvo, com a acao.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LacoDaAcao {
    pub ganho_no_indicador: f64,
    pub ganho_no_alvo: f64,
    pub razao: f64,
    pub passivo: f64,
}

/// Um agente que age no decil de cima do indicador. O indicador so acompanha o alvo se o ruido proprio
/// dele for pequeno: e a razao que mede o laco.
pub fn laco_da_acao(semente: u64, n: usize, ruido_do_indicador: f64) -> LacoDaAcao {
    let mut rng = Rng::novo(semente);
    let mut alvo = Vec::with_capacity(n);
    let mut indicador = Vec::with_capacity(n);
    for _ in 0..n {
        let x = normal(&mut rng);
        alvo.push(x + normal(&mut rng));
        indicador.push(x + ruido_do_indicador * normal(&mut rng));
    }
    let media_alvo = alvo.iter().sum::<f64>() / n as f64;
    let media_indicador = indicador.iter().sum::<f64>() / n as f64;
    let mut ordem: Vec<usize> = (0..n).collect();
    ordem.sort_by(|a, b| indicador[*b].partial_cmp(&indicador[*a]).unwrap_or(std::cmp::Ordering::Equal));
    let corte = (n / 10).max(1);
    let escolhidos = &ordem[..corte];
    let ganho_no_indicador =
        escolhidos.iter().map(|i| indicador[*i]).sum::<f64>() / corte as f64 - media_indicador;
    let ganho_no_alvo = escolhidos.iter().map(|i| alvo[*i]).sum::<f64>() / corte as f64 - media_alvo;
    let razao = if ganho_no_indicador.abs() < 1e-12 {
        0.0
    } else {
        ganho_no_alvo / ganho_no_indicador
    };
    let desvio_alvo = (alvo.iter().map(|v| (v - media_alvo).powi(2)).sum::<f64>() / n as f64).sqrt();
    let desvio_indicador =
        (indicador.iter().map(|v| (v - media_indicador).powi(2)).sum::<f64>() / n as f64).sqrt();
    let covariancia = alvo
        .iter()
        .zip(indicador.iter())
        .map(|(a, g)| (a - media_alvo) * (g - media_indicador))
        .sum::<f64>()
        / n as f64;
    let passivo = if desvio_alvo > 0.0 && desvio_indicador > 0.0 {
        covariancia / (desvio_alvo * desvio_indicador)
    } else {
        0.0
    };
    LacoDaAcao { ganho_no_indicador, ganho_no_alvo, razao, passivo }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn a_ece_e_zero_no_calibrado_e_alta_no_exagerado() {
        let mut rng = Rng::novo(2026);
        let mut confianca = Vec::new();
        let mut acerto = Vec::new();
        for _ in 0..20_000 {
            let p = uniforme(&mut rng);
            confianca.push(p);
            acerto.push(uniforme(&mut rng) < p);
        }
        let calibrado = ece(&confianca, &acerto, 20);
        assert!(calibrado < 0.02, "calibrado mede {calibrado}");
        let exagerada: Vec<f64> = confianca.iter().map(|p| (p * 1.6).min(0.99)).collect();
        let exagerado = ece(&exagerada, &acerto, 20);
        assert!(exagerado > 0.10, "exagerado mede {exagerado}");
    }

    #[test]
    fn a_tolerancia_da_ece_depende_de_n() {
        let mut pequenas = 0.0;
        let mut grandes = 0.0;
        let rodadas = 60;
        for semente in 0..rodadas {
            let mut rng = Rng::novo(1000 + semente);
            let mut confianca = Vec::new();
            let mut acerto = Vec::new();
            for _ in 0..8_000 {
                let p = uniforme(&mut rng);
                confianca.push(p);
                acerto.push(uniforme(&mut rng) < p);
            }
            pequenas += ece(&confianca[..200], &acerto[..200], 20);
            grandes += ece(&confianca, &acerto, 20);
        }
        let pequenas = pequenas / rodadas as f64;
        let grandes = grandes / rodadas as f64;
        assert!(pequenas > grandes, "200 pontos medem {pequenas}, 8000 medem {grandes}");
    }

    #[test]
    fn a_habilidade_de_brier_premeia_o_bom_e_pune_o_mau() {
        let mut rng = Rng::novo(17);
        let mut bom = Vec::new();
        let mut mau = Vec::new();
        let mut observado = Vec::new();
        for _ in 0..5_000 {
            let p = 0.25 + 0.5 * uniforme(&mut rng);
            bom.push(p);
            mau.push(1.0 - p);
            observado.push(uniforme(&mut rng) < p);
        }
        assert!(habilidade_de_brier(&bom, &observado) > 0.0);
        assert!(habilidade_de_brier(&mau, &observado) < 0.0);
    }

    #[test]
    fn o_canario_pega_a_leitura_do_futuro() {
        let (relogios, leituras) = fluxo_estrito(2026, 500);
        assert_eq!(vazamentos(&relogios, &leituras), 0);
        assert!(!canario_plantado(&relogios, &leituras));
        let mut furadas = leituras.clone();
        furadas[7] = relogios[7] + 1;
        assert_eq!(vazamentos(&relogios, &furadas), 1);
        assert!(canario_plantado(&relogios, &furadas));
        let (relogios2, leituras2) = fluxo_estrito(2026, 500);
        assert_eq!(relogios, relogios2);
        assert_eq!(leituras, leituras2);
    }

    #[test]
    fn o_laco_mede_o_que_a_acao_faz_com_o_indicador() {
        let perfeito = laco_da_acao(2026, 40_000, 0.0);
        assert!(
            (perfeito.razao - 1.0).abs() < 0.12,
            "indicador que e o alvo nao encolhe: {perfeito:?}"
        );
        let viciado = laco_da_acao(2026, 40_000, 1.0);
        assert!(viciado.razao < 0.8, "com ruido proprio, o ganho evapora: {viciado:?}");
        let pior = laco_da_acao(2026, 40_000, 2.0);
        assert!(pior.razao < viciado.razao, "o laco cresce com o ruido: {pior:?}");
        assert!(viciado.passivo < perfeito.passivo);
    }

    #[test]
    fn a_correlacao_passiva_segue_a_conta_exata() {
        // alvo = x + e e indicador = x + ruido * z, com tudo normal padrao:
        // corr = 1 / sqrt(2 * (1 + ruido^2)). O nulo aqui tem valor exato, nao faixa.
        for ruido in [0.0_f64, 0.5, 1.0, 2.0] {
            let medido = laco_da_acao(2026, 40_000, ruido).passivo;
            let exato = 1.0 / (2.0 * (1.0 + ruido * ruido)).sqrt();
            assert!((medido - exato).abs() < 0.03, "ruido {ruido}: {medido} contra {exato}");
        }
    }
}
