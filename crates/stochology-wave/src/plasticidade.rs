//! Plasticidade Kuramoto-Oja: o Sistema 2 cicatriza o hipergrafo depois de resolver um conflito.
//!
//! A regra tem dois movimentos acoplados. O primeiro e um torque de fase: o angulo magnetico
//! alfa_{vh} gira no sentido de reduzir o erro entre a fase de chegada e a fase que o veredito
//! do Sistema 2 validou. O segundo e hebbiano com regularizacao de Oja: a condutancia eta_{vh}
//! cresce quando as chegadas estao em fase e decai sob o termo quadratico, o que impede a
//! explosao de pesos sem corte arbitrario.
//!
//! Portao (divergencia declarada em relacao ao rascunho original): a consolidacao exige **ganho
//! de ressonancia no alvo**, nao queda monotona da frustracao. A frustracao <Psi, L Psi> nao e
//! monotona sob plasticidade que cria acoplamento -- no caso de antifase simetrica ela vale zero
//! e sobe quando o vinculo passa a existir. Ela e reportada no commit como diagnostico; o que o
//! portao cobra e que a mesma pancada de entrada chegue mais forte ao alvo, com norma preservada.

use crate::complexo::Complexo;
use crate::magnetico::{HipergrafoMag, Propagacao, fase_de_chegada};
use crate::selo::{Selo, envolve_alfa};

/// Hiperparametros da plasticidade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfigPlasticidade {
    /// mu_alfa: passo do torque de fase.
    pub lr_alfa: f64,
    /// mu_eta: passo da condutancia.
    pub lr_eta: f64,
    /// beta_oja: regularizacao quadratica.
    pub oja: f64,
    /// Piso da condutancia.
    pub peso_min: f64,
    /// Ciclos internos de relaxacao.
    pub epocas: usize,
}

impl Default for ConfigPlasticidade {
    fn default() -> Self {
        Self {
            lr_alfa: 0.35,
            lr_eta: 0.25,
            oja: 0.10,
            peso_min: 0.05,
            epocas: 12,
        }
    }
}

/// Veredito do Sistema 2 sobre um vertice em conflito.
#[derive(Debug, Clone, PartialEq)]
pub struct VereditoS2 {
    pub alvo: usize,
    /// Estado fasorial validado pela deliberacao.
    pub fasor: Complexo,
    /// (indice da hiperaresta, polaridade): +1 reforca, -1 isola a via refutada.
    pub polaridades: Vec<(usize, f64)>,
}

/// Registro auditavel da mutacao topologica aceita.
#[derive(Debug, Clone, PartialEq)]
pub struct CommitPlastico {
    pub selo_antes: u64,
    pub selo_depois: u64,
    pub amplitude_antes: f64,
    pub amplitude_depois: f64,
    pub ganho_antes: f64,
    pub ganho_depois: f64,
    pub frustracao_antes: f64,
    pub frustracao_depois: f64,
}

/// Mutacao recusada: o portao restaurou a topologia anterior.
#[derive(Debug, Clone, PartialEq)]
pub struct RecusaPlastica {
    pub motivo: String,
    pub ganho_antes: f64,
    pub ganho_depois: f64,
    pub amplitude_antes: f64,
    pub amplitude_depois: f64,
}

/// Aplica a regra Kuramoto-Oja sobre as hiperarestas citadas no veredito.
pub fn aplica(
    grafo: &mut HipergrafoMag,
    ambiente: &[Complexo],
    veredito: &VereditoS2,
    cfg: &ConfigPlasticidade,
) -> Result<(), String> {
    if veredito.alvo >= grafo.vertices.len() {
        return Err(format!("alvo {} fora do hipergrafo", veredito.alvo));
    }
    if ambiente.len() != grafo.vertices.len() {
        return Err("ambiente com tamanho incompativel".to_string());
    }
    if cfg.lr_alfa < 0.0 || cfg.lr_eta < 0.0 || cfg.oja < 0.0 || cfg.epocas == 0 {
        return Err("hiperparametro invalido".to_string());
    }
    let q = grafo.q;
    let dois_pi_q = 2.0 * std::f64::consts::PI * q;
    if dois_pi_q.abs() < 1e-12 {
        return Err("q nulo: o angulo magnetico nao tem efeito".to_string());
    }

    let amp_s2 = veredito.fasor.modulo();
    let fase_s2 = veredito.fasor.fase();

    for _ in 0..cfg.epocas {
        for &(h, sigma) in &veredito.polaridades {
            if h >= grafo.hiperarestas.len() {
                continue;
            }

            let (ctx_amp, chegada) = {
                let aresta = &grafo.hiperarestas[h];
                match fase_de_chegada(ambiente, aresta, veredito.alvo, q) {
                    Some((_contexto, chegada, entrada)) => (entrada, chegada),
                    None => continue,
                }
            };
            if ctx_amp < 1e-12 {
                continue;
            }

            let dphi = fase_s2 - chegada;
            let aresta = &mut grafo.hiperarestas[h];
            let Some(inc) = aresta
                .incidencias
                .iter_mut()
                .find(|i| i.vertice == veredito.alvo)
            else {
                continue;
            };

            let d_alfa = (cfg.lr_alfa / dois_pi_q) * sigma * amp_s2 * ctx_amp * dphi.sin();
            inc.alfa = envolve_alfa(inc.alfa + d_alfa);

            let hebb = sigma * dphi.cos() * amp_s2 * ctx_amp;
            let oja = cfg.oja * inc.peso * (amp_s2 * amp_s2);
            inc.peso = (inc.peso + cfg.lr_eta * (hebb - oja)).max(cfg.peso_min);
            if !inc.peso.is_finite() || !inc.alfa.is_finite() {
                return Err("plasticidade produziu valor nao finito".to_string());
            }
        }
    }
    Ok(())
}

fn ganho(propagacao: &Propagacao, alvo: usize) -> f64 {
    let amp = propagacao.estado[alvo].modulo();
    let entrada = propagacao.entradas[alvo];
    if entrada > 1e-12 { amp / entrada } else { 0.0 }
}

/// Executa o ciclo: propaga, aplica plasticidade, propaga de novo e sela ou reverte.
pub fn consolida(
    grafo: &mut HipergrafoMag,
    pulso: &[Complexo],
    saltos: usize,
    dt: f64,
    veredito: &VereditoS2,
    cfg: &ConfigPlasticidade,
) -> Result<CommitPlastico, RecusaPlastica> {
    let selo_antes = grafo.selo_topologia();
    let antes = match grafo.propaga(pulso, saltos, dt) {
        Ok(p) => p,
        Err(e) => {
            return Err(RecusaPlastica {
                motivo: format!("propagacao inicial falhou: {e}"),
                ganho_antes: 0.0,
                ganho_depois: 0.0,
                amplitude_antes: 0.0,
                amplitude_depois: 0.0,
            });
        }
    };
    let amplitude_antes = antes.estado[veredito.alvo].modulo();
    let ganho_antes = ganho(&antes, veredito.alvo);
    let norma_inicial = antes.norma2_inicial;

    let copia = grafo.clone();
    if let Err(e) = aplica(grafo, &antes.estado, veredito, cfg) {
        *grafo = copia;
        return Err(RecusaPlastica {
            motivo: format!("plasticidade recusada: {e}"),
            ganho_antes,
            ganho_depois: ganho_antes,
            amplitude_antes,
            amplitude_depois: amplitude_antes,
        });
    }

    let depois = match grafo.propaga(pulso, saltos, dt) {
        Ok(p) => p,
        Err(e) => {
            *grafo = copia;
            return Err(RecusaPlastica {
                motivo: format!("propagacao pos-plasticidade falhou: {e}"),
                ganho_antes,
                ganho_depois: ganho_antes,
                amplitude_antes,
                amplitude_depois: amplitude_antes,
            });
        }
    };
    let amplitude_depois = depois.estado[veredito.alvo].modulo();
    let ganho_depois = ganho(&depois, veredito.alvo);

    let estado_finito = depois.estado.iter().all(|c| c.finito());
    let aceito = estado_finito
        && ganho_depois.is_finite()
        && ganho_depois > ganho_antes + 1e-6
        && (depois.norma2 - norma_inicial).abs() < 1e-9;

    if !aceito {
        *grafo = copia;
        return Err(RecusaPlastica {
            motivo: if estado_finito {
                format!(
                    "ganho de ressonancia nao subiu o bastante (antes {ganho_antes:.4}, depois {ganho_depois:.4})"
                )
            } else {
                "estado nao finito apos plasticidade".to_string()
            },
            ganho_antes,
            ganho_depois,
            amplitude_antes,
            amplitude_depois,
        });
    }

    Ok(CommitPlastico {
        selo_antes,
        selo_depois: grafo.selo_topologia(),
        amplitude_antes,
        amplitude_depois,
        ganho_antes,
        ganho_depois,
        frustracao_antes: antes.frustracao,
        frustracao_depois: depois.frustracao,
    })
}

/// Selo de um commit: util para encadear epocas de aprendizado num registro.
pub fn selo_do_commit(commit: &CommitPlastico) -> u64 {
    let mut selo = Selo::novo();
    selo.escreve_u64(commit.selo_antes);
    selo.escreve_u64(commit.selo_depois);
    selo.escreve_f64(commit.amplitude_antes);
    selo.escreve_f64(commit.amplitude_depois);
    selo.escreve_f64(commit.ganho_antes);
    selo.escreve_f64(commit.ganho_depois);
    selo.fecha()
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::magnetico::testes::{grafo_construtivo, pulso};

    #[test]
    fn veredito_bom_consolida_e_sela() {
        let mut g = grafo_construtivo(true);
        let antes = g.propaga(&pulso(), 3, 0.65).expect("antes");
        assert!(antes.estado[3].modulo() < 1e-9);
        let veredito = VereditoS2 {
            alvo: 3,
            fasor: Complexo::polar(0.8, antes.estado[1].fase()),
            polaridades: vec![(1, 1.0), (2, 1.0)],
        };
        let commit = consolida(
            &mut g,
            &pulso(),
            3,
            0.65,
            &veredito,
            &ConfigPlasticidade::default(),
        )
        .expect("commit");
        assert!(
            commit.amplitude_depois > 0.04,
            "amplitude {}",
            commit.amplitude_depois
        );
        assert!(commit.ganho_depois > commit.ganho_antes);
        assert_ne!(commit.selo_antes, commit.selo_depois);
        let depois = g.propaga(&pulso(), 3, 0.65).expect("depois");
        assert!((depois.norma2 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn veredito_ruim_e_revertido() {
        let mut g = grafo_construtivo(false);
        let antes = g.propaga(&pulso(), 2, 0.65).expect("antes");
        let selo = g.selo_topologia();
        let veredito = VereditoS2 {
            alvo: 3,
            fasor: Complexo::polar(0.9, antes.estado[3].fase() + 0.9),
            polaridades: vec![(2, -1.0)],
        };
        let recusa = consolida(
            &mut g,
            &pulso(),
            2,
            0.65,
            &veredito,
            &ConfigPlasticidade::default(),
        )
        .expect_err("devia recusar");
        assert_eq!(
            g.selo_topologia(),
            selo,
            "topologia devia voltar ao original"
        );
        assert!(recusa.motivo.contains("ganho"), "motivo: {}", recusa.motivo);
    }

    #[test]
    fn alvo_fora_do_grafo_e_erro() {
        let mut g = grafo_construtivo(false);
        let veredito = VereditoS2 {
            alvo: 9,
            fasor: Complexo::polar(1.0, 0.0),
            polaridades: vec![(0, 1.0)],
        };
        assert!(aplica(&mut g, &pulso(), &veredito, &ConfigPlasticidade::default()).is_err());
    }
}
