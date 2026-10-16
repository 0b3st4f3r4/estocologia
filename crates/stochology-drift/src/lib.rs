//! stochology-drift — a deriva: quando o mundo sai de baixo do modelo.
//!
//! Duas ferramentas: o CUSUM, que vigia a media e avisa quando ela desloca, e a divergencia de
//! Kullback-Leibler, que mede o tamanho do deslocamento entre duas distribuicoes declaradas. A
//! escolha dos parametros segue o uso classico (folga 0,75 sigma e limiar 10 sigma), calibrada para
//! o nulo nao disparar em 100 mil pontos: o nulo fica declarado e medido, nao suposto.
//!
//! Declarado e nao implementado: a deriva de covariancia (so a de locacao esta aqui) e a ligacao com
//! o campo `deriva_declarada` da ficha, que segue como campo textual.

use stochology_core::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opcoes {
    /// Quantos pontos iniciais estimam a media e o desvio de referencia.
    pub base: usize,
    /// Folga em unidades de desvio: abaixo disso o deslocamento nao acumula.
    pub folga: f64,
    /// Limiar de alarme, tambem em unidades de desvio.
    pub limiar: f64,
}

impl Default for Opcoes {
    fn default() -> Self {
        Opcoes { base: 50, folga: 0.75, limiar: 10.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vigilancia {
    pub alarme: Option<usize>,
    pub cusum_maximo: f64,
    pub media_da_base: f64,
    pub desvio_da_base: f64,
}

/// CUSUM bilateral sobre a serie, com a base estimada nos primeiros pontos.
pub fn cusum(serie: &[f64], opcoes: &Opcoes) -> Vigilancia {
    let base = opcoes.base.clamp(2, serie.len().max(2));
    if serie.len() <= base {
        return Vigilancia {
            alarme: None,
            cusum_maximo: 0.0,
            media_da_base: if serie.is_empty() { 0.0 } else { serie.iter().sum::<f64>() / serie.len() as f64 },
            desvio_da_base: 0.0,
        };
    }
    let media = serie[..base].iter().sum::<f64>() / base as f64;
    let variancia = serie[..base].iter().map(|x| (x - media).powi(2)).sum::<f64>() / (base as f64 - 1.0);
    let desvio = variancia.sqrt().max(1e-9);
    let mut alto = 0.0_f64;
    let mut baixo = 0.0_f64;
    let mut maximo = 0.0_f64;
    for (i, x) in serie.iter().enumerate().skip(base) {
        alto = (alto + (x - media) / desvio - opcoes.folga).max(0.0);
        baixo = (baixo + (media - x) / desvio - opcoes.folga).max(0.0);
        let corrente = alto.max(baixo);
        if corrente > maximo {
            maximo = corrente;
        }
        if corrente > opcoes.limiar {
            return Vigilancia {
                alarme: Some(i),
                cusum_maximo: maximo,
                media_da_base: media,
                desvio_da_base: desvio,
            };
        }
    }
    Vigilancia { alarme: None, cusum_maximo: maximo, media_da_base: media, desvio_da_base: desvio }
}

/// Divergencia de Kullback-Leibler entre duas distribuicoes declaradas, com piso (nunca 0 nem 1).
pub fn kl(p: &[f64], q: &[f64], piso: f64) -> f64 {
    if p.len() != q.len() || p.is_empty() {
        return f64::NAN;
    }
    let normalizar = |valores: &[f64]| -> Vec<f64> {
        let presos: Vec<f64> = valores.iter().map(|v| v.max(piso)).collect();
        let soma: f64 = presos.iter().sum();
        presos.iter().map(|v| v / soma).collect()
    };
    let p = normalizar(p);
    let q = normalizar(q);
    p.iter().zip(q.iter()).map(|(a, b)| a * (a / b).ln()).sum()
}

/// O nulo declarado: ruido estacionario gaussiano, sem deriva nenhuma.
pub fn nulo_estacionario(semente: u64, n: usize, media: f64, desvio: f64) -> Vec<f64> {
    let mut rng = Rng::novo(semente);
    (0..n)
        .map(|_| {
            let u1 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            let u2 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            media + desvio * (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
        })
        .collect()
}

/// Planta um salto na serie, a partir de `em`.
pub fn com_salto(serie: &mut [f64], em: usize, salto: f64) {
    if em >= serie.len() {
        return;
    }
    for valor in serie[em..].iter_mut() {
        *valor += salto;
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn o_nulo_nao_dispara_em_cem_mil_pontos() {
        let opcoes = Opcoes::default();
        let mut alarmes = 0;
        for semente in 0..50 {
            let serie = nulo_estacionario(semente, 2_000, 10.0, 2.0);
            if cusum(&serie, &opcoes).alarme.is_some() {
                alarmes += 1;
            }
        }
        assert_eq!(alarmes, 0, "o nulo estacionario nao dispara: {alarmes} alarmes");
    }

    #[test]
    fn o_salto_dispara_o_alarme_depois_de_acontecer() {
        let opcoes = Opcoes::default();
        let mut serie = nulo_estacionario(2026, 1_500, 0.0, 1.0);
        let em = 700;
        com_salto(&mut serie, em, 1.5);
        let vigilancia = cusum(&serie, &opcoes);
        match vigilancia.alarme {
            Some(i) => {
                assert!(i >= em, "alarme antes do salto: {i} contra {em}");
                assert!(i - em < 60, "alarme tardio: {i} contra {em}");
            }
            None => panic!("salto de 1,5 sigma nao foi visto: {vigilancia:?}"),
        }
        assert!(vigilancia.cusum_maximo > opcoes.limiar);
    }

    #[test]
    fn o_salto_para_baixo_tambem_dispara() {
        let mut serie = nulo_estacionario(31, 1_200, 5.0, 1.0);
        com_salto(&mut serie, 600, -2.0);
        let vigilancia = cusum(&serie, &Opcoes::default());
        assert!(vigilancia.alarme.map(|i| i >= 600).unwrap_or(false), "{vigilancia:?}");
    }

    #[test]
    fn a_kl_e_zero_contra_si_mesma_e_cresce_com_a_distancia() {
        let p = [0.25, 0.25, 0.25, 0.25];
        let perto = [0.26, 0.24, 0.25, 0.25];
        let longe = [0.7, 0.1, 0.1, 0.1];
        let zero = kl(&p, &p, 1e-6);
        assert!(zero.abs() < 1e-9, "kl de si mesma: {zero}");
        assert!(kl(&p, &perto, 1e-6) < kl(&p, &longe, 1e-6));
        assert!(kl(&p, &longe, 1e-6) > 0.4);
    }

    #[test]
    fn a_vigilancia_e_deterministica() {
        let serie = nulo_estacionario(99, 800, 0.0, 1.0);
        let opcoes = Opcoes::default();
        assert_eq!(cusum(&serie, &opcoes), cusum(&serie, &opcoes));
        let outra = nulo_estacionario(100, 800, 0.0, 1.0);
        assert_ne!(serie, outra);
    }
}
