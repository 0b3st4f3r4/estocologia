//! Inferencia pesada: Metropolis de passo aleatorio com o alvo declarado pelo chamador.
//!
//! O PLAN.md §4 poe a "inferencia MCMC" entre as tarefas do Sistema 2. O que existe aqui e o amostrador
//! de Metropolis-Hastings com proposta gaussiana simetrica e semente fixa: o alvo entra como log-densidade
//! declarada, e a cadeia devolve amostras, taxa de aceitacao e resumo da metade final.
//!
//! Declarado e nao implementado: adaptacao do passo (o passo e declarado, nao aprendido), diagnostico de
//! convergencia por R-hat e SBC, e alvos multidimensionais.

use stochology_core::Rng;

fn uniforme(rng: &mut Rng) -> f64 {
    ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn normal(rng: &mut Rng) -> f64 {
    let u1 = uniforme(rng);
    let u2 = uniforme(rng);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Amostrador de Metropolis: passo da proposta e semente da cadeia.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metropolis {
    pub passo: f64,
    pub semente: u64,
}

/// A cadeia: as amostras na ordem em que sairam, e quantas propostas foram aceitas.
#[derive(Debug, Clone, PartialEq)]
pub struct Cadeia {
    pub amostras: Vec<f64>,
    pub aceitos: usize,
}

impl Cadeia {
    pub fn aceitacao(&self) -> f64 {
        if self.amostras.is_empty() {
            return 0.0;
        }
        self.aceitos as f64 / self.amostras.len() as f64
    }

    pub fn media(&self, queimados: usize) -> f64 {
        let corpo = self.corpo(queimados);
        if corpo.is_empty() {
            return 0.0;
        }
        corpo.iter().sum::<f64>() / corpo.len() as f64
    }

    pub fn desvio(&self, queimados: usize) -> f64 {
        let corpo = self.corpo(queimados);
        if corpo.len() < 2 {
            return 0.0;
        }
        let media = corpo.iter().sum::<f64>() / corpo.len() as f64;
        (corpo.iter().map(|x| (x - media).powi(2)).sum::<f64>() / (corpo.len() as f64 - 1.0)).sqrt()
    }

    /// Quantil da parte final da cadeia, por interpolacao linear (a mesma regra do H0).
    pub fn quantil(&self, queimados: usize, q: f64) -> f64 {
        let mut corpo = self.corpo(queimados).to_vec();
        corpo.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        if corpo.is_empty() {
            return 0.0;
        }
        let posicao = q.clamp(0.0, 1.0) * (corpo.len() as f64 - 1.0);
        let baixo = posicao.floor() as usize;
        let alto = posicao.ceil() as usize;
        if baixo == alto {
            return corpo[baixo];
        }
        let peso = posicao - baixo as f64;
        corpo[baixo] * (1.0 - peso) + corpo[alto] * peso
    }

    fn corpo(&self, queimados: usize) -> &[f64] {
        let corte = queimados.min(self.amostras.len());
        &self.amostras[corte..]
    }
}

impl Metropolis {
    /// Roda a cadeia a partir de `inicio`, com `log_alvo` declarada como densidade nao normalizada.
    pub fn amostrar<F: Fn(f64) -> f64>(&self, log_alvo: F, inicio: f64, iteracoes: usize) -> Cadeia {
        let mut rng = Rng::novo(self.semente);
        let mut atual = inicio;
        let mut log_atual = log_alvo(atual);
        let mut amostras = Vec::with_capacity(iteracoes);
        let mut aceitos = 0usize;
        for _ in 0..iteracoes {
            let proposta = atual + self.passo * normal(&mut rng);
            let log_proposta = log_alvo(proposta);
            // alvo fora do suporte devolve -inf: rejeita sem sortear
            let aceita = if !log_proposta.is_finite() {
                false
            } else if !log_atual.is_finite() {
                true
            } else {
                let razao = (log_proposta - log_atual).min(0.0);
                razao.exp() > uniforme(&mut rng)
            };
            if aceita {
                atual = proposta;
                log_atual = log_proposta;
                aceitos += 1;
            }
            amostras.push(atual);
        }
        Cadeia { amostras, aceitos }
    }
}

/// Log-densidade de uma normal, a menos de constante.
pub fn log_normal(media: f64, desvio: f64) -> impl Fn(f64) -> f64 {
    move |x: f64| -0.5 * ((x - media) / desvio).powi(2)
}

/// Log-densidade de uma beta com os dois parametros de contagem.
pub fn log_beta(alfa: f64, beta: f64) -> impl Fn(f64) -> f64 {
    move |x: f64| {
        if x <= 0.0 || x >= 1.0 {
            f64::NEG_INFINITY
        } else {
            (alfa - 1.0) * x.ln() + (beta - 1.0) * (1.0 - x).ln()
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn a_cadeia_recupera_a_media_de_uma_normal() {
        let cadeia = Metropolis { passo: 1.5, semente: 2026 }.amostrar(log_normal(2.0, 1.0), 0.0, 200_000);
        let media = cadeia.media(20_000);
        assert!((media - 2.0).abs() < 0.05, "media da cadeia: {media}");
        let desvio = cadeia.desvio(20_000);
        assert!((desvio - 1.0).abs() < 0.05, "desvio da cadeia: {desvio}");
        let aceitacao = cadeia.aceitacao();
        assert!((0.15..0.60).contains(&aceitacao), "aceitacao: {aceitacao}");
    }

    #[test]
    fn a_cadeia_recupera_a_posterior_conjugada_da_beta() {
        // Beta(4, 8): media exata 4/12 = 1/3, no suporte [0, 1].
        let cadeia = Metropolis { passo: 0.15, semente: 7 }.amostrar(log_beta(4.0, 8.0), 0.5, 200_000);
        let media = cadeia.media(20_000);
        assert!((media - 1.0 / 3.0).abs() < 0.02, "media da posterior: {media}");
        assert!(cadeia.amostras.iter().all(|x| (0.0..=1.0).contains(x)), "a cadeia respeita o suporte");
        let mediana = cadeia.quantil(20_000, 0.5);
        assert!(mediana < media, "a Beta(4,8) tem mediana abaixo da media: {mediana} contra {media}");
    }

    #[test]
    fn a_cadeia_e_deterministica_na_semente() {
        let passo = Metropolis { passo: 1.0, semente: 99 };
        let alvo = log_normal(0.0, 1.0);
        assert_eq!(passo.amostrar(&alvo, 0.0, 5_000), passo.amostrar(&alvo, 0.0, 5_000));
        let outra = Metropolis { passo: 1.0, semente: 100 };
        assert_ne!(passo.amostrar(&alvo, 0.0, 5_000), outra.amostrar(&alvo, 0.0, 5_000));
    }

    #[test]
    fn passo_grande_derruba_a_aceitacao() {
        let alvo = log_normal(0.0, 1.0);
        let pequeno = Metropolis { passo: 0.1, semente: 3 }.amostrar(&alvo, 0.0, 50_000);
        let grande = Metropolis { passo: 50.0, semente: 3 }.amostrar(&alvo, 0.0, 50_000);
        assert!(pequeno.aceitacao() > grande.aceitacao());
        assert!(grande.aceitacao() < 0.05, "proposta absurda quase nao aceita: {}", grande.aceitacao());
    }

    #[test]
    fn alvo_fora_do_suporte_nao_aceita_nada() {
        let cadeia = Metropolis { passo: 1.0, semente: 5 }
            .amostrar(|x: f64| if x < 0.0 { f64::NEG_INFINITY } else { -x }, 1.0, 1_000);
        assert!(cadeia.amostras.iter().all(|x| *x >= 0.0), "a cadeia fica no suporte");
    }
}
