//! stochology-h1 — a bolsa, o emprego e o fogo: o kernel do horizonte um.
//!
//! Tres mecanismos do H1 vivem aqui: o arrasto da alavancagem (media aritmetica contra media
//! geometrica), o vies de selecao de um experimento contra o grupo observado, e o estimador de Hill
//! da cauda. O resumo obrigatorio vem de `stochology-h0`: onde a dinamica e multiplicativa, a media
//! sozinha e proibida.
//!
//! Declarado e nao implementado: a leitura das cinco bases de `dados-reais/` (vive nos laboratorios
//! do capitulo 17 e 18) e a correlacao intracoorte, que segue sem instrumento em Rust.

use stochology_core::Rng;

/// Arrasto da alavancagem: a diferenca entre a media aritmetica alavancada e a taxa composta.
pub fn arrasto(alavancagem: f64, retornos: &[f64]) -> f64 {
    if retornos.is_empty() {
        return 0.0;
    }
    let n = retornos.len() as f64;
    let media = retornos.iter().sum::<f64>() / n;
    let composta = retornos
        .iter()
        .map(|r| (1.0 + alavancagem * r).max(1e-12).ln())
        .sum::<f64>()
        / n;
    alavancagem * media - (composta.exp() - 1.0)
}

/// Taxa composta da serie alavancada, por periodo.
pub fn taxa_composta(alavancagem: f64, retornos: &[f64]) -> f64 {
    if retornos.is_empty() {
        return 0.0;
    }
    let n = retornos.len() as f64;
    let soma = retornos
        .iter()
        .map(|r| (1.0 + alavancagem * r).max(1e-12).ln())
        .sum::<f64>();
    (soma / n).exp() - 1.0
}

/// Fracao da riqueza que um unico dia adverso destroi numa carteira alavancada.
pub fn fator_de_ruina(alavancagem: f64, pior_dia: f64) -> f64 {
    (1.0 - (1.0 + alavancagem * pior_dia).max(0.0)).clamp(0.0, 1.0)
}

/// Estimador de Hill do indice de cauda, com os `k` maiores: `alfa = k / soma(ln(x_i/x_k+1))`.
pub fn hill(serie: &[f64], k: usize) -> f64 {
    stochology_core::hill_no_corte(serie, k, k)
}

/// Comparacao crua, ganho do experimento e vies de selecao entre o grupo observado e o sorteado.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViesDeSelecao {
    pub cru: f64,
    pub experimental: f64,
    pub vies: f64,
}

pub fn media(serie: &[f64]) -> f64 {
    if serie.is_empty() {
        return 0.0;
    }
    serie.iter().sum::<f64>() / serie.len() as f64
}

pub fn vies_de_selecao(
    tratados: &[f64],
    controle_observado: &[f64],
    controle_experimental: &[f64],
) -> ViesDeSelecao {
    let cru = media(tratados) - media(controle_observado);
    let experimental = media(tratados) - media(controle_experimental);
    // o vies e o deslocamento da estimativa crua: quanto o grupo observado mente contra o sorteado
    ViesDeSelecao { cru, experimental, vies: cru - experimental }
}

/// Serie multiplicativa com cauda a direita: o mundo em que a mediana e a media contam historias
/// diferentes. Serve de contraste declarado para o resumo do H0.
pub fn serie_multiplicativa(semente: u64, n: usize, passos: usize, media_do_log: f64) -> Vec<f64> {
    let mut rng = Rng::novo(semente);
    (0..n)
        .map(|_| {
            let mut valor = 1.0_f64;
            for _ in 0..passos {
                valor *= (media_do_log + 0.18 * h0_normal(&mut rng)).exp();
            }
            valor
        })
        .collect()
}

pub(crate) fn h0_normal(rng: &mut Rng) -> f64 {
    let u1 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let u2 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

#[cfg(test)]
mod testes {
    use super::*;
    use stochology_h0::resumo;

    #[test]
    fn arrasto_de_serie_simetrica_tem_valor_exato() {
        let serie = [-0.1, 0.1];
        let esperado = 1.0 - (0.99_f64).sqrt();
        let obtido = arrasto(1.0, &serie);
        assert!((obtido - esperado).abs() < 1e-12, "arrasto {obtido}, esperado {esperado}");
        assert!((arrasto(2.0, &serie) - (1.0 - 0.96_f64.sqrt())).abs() < 1e-12);
    }

    #[test]
    fn arrasto_cresce_com_a_alavancagem() {
        let mut rng = Rng::novo(2026);
        let serie: Vec<f64> = (0..2_000).map(|_| 0.0004 + 0.011 * h0_normal(&mut rng)).collect();
        let (um, dois, tres) = (arrasto(1.0, &serie), arrasto(2.0, &serie), arrasto(3.0, &serie));
        assert!(um < dois && dois < tres, "arrasto monotonico: {um} {dois} {tres}");
        assert!(um > 0.0, "arrasto positivo com volatilidade: {um}");
        assert!(
            (taxa_composta(1.0, &serie) - (media(&serie) - um)).abs() < 1e-12,
            "a taxa composta e a media menos o arrasto"
        );
    }

    #[test]
    fn o_dia_de_1987_derruba_quem_esta_a_tres_vezes() {
        let destruido = fator_de_ruina(3.0, -0.226);
        assert!((destruido - 0.678).abs() < 1e-12, "destruido {destruido}");
        assert!(fator_de_ruina(1.0, -0.226) < 0.23);
        assert_eq!(fator_de_ruina(5.0, -0.30), 1.0, "carteira zerada");
        assert_eq!(fator_de_ruina(1.0, 0.10), 0.0, "dia bom nao destrói");
    }

    #[test]
    fn hill_recupera_o_alfa_de_uma_pareto() {
        let mut rng = Rng::novo(7);
        let alfa = 1.81_f64;
        let serie: Vec<f64> = (0..4_000)
            .map(|_| {
                let u = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
                (1.0 - u).powf(-1.0 / alfa)
            })
            .collect();
        let obtido = hill(&serie, 400);
        assert!((obtido - alfa).abs() < 0.35, "Hill {obtido} contra {alfa}");
        assert!(hill(&serie, 1).is_finite());
        assert!(hill(&serie, 100_000).is_nan(), "sem k suficiente nao ha estimativa");
    }

    #[test]
    fn o_vies_de_selecao_inverte_o_sinal_do_ganho() {
        // Forma do LaLonde: tratados com ganho real, controle observado mais rico que o sorteado.
        let tratados = vec![6_349.0, 6_349.0];
        let observado = vec![14_847.0, 14_847.0];
        let experimental = vec![4_555.0, 4_555.0];
        let v = vies_de_selecao(&tratados, &observado, &experimental);
        assert!((v.cru - (-8_498.0)).abs() < 1e-9);
        assert!((v.experimental - 1_794.0).abs() < 1e-9);
        assert!((v.vies - (-10_292.0)).abs() < 1e-9);
    }

    #[test]
    fn a_mediana_e_a_media_contam_historias_diferentes() {
        let serie = serie_multiplicativa(2026, 20_000, 2_660, 0.00016);
        let r = resumo(&serie);
        assert!(r.mediana < r.media / 3.0, "mediana {} media {}", r.mediana, r.media);
        assert!(r.p95 > 4.0 * r.mediana);
        assert!(r.p05 < r.mediana);
    }
}
