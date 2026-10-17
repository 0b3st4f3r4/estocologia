//! Portao global de verificacao do motor ondulatorio.
//!
//! Seis cobrancas, cada uma com prova propria: estrutura do hipergrafo, Hermiticidade de L^(q),
//! conservacao da norma pelo propagador de Cayley, integridade da cadeia de selos (com prova
//! negativa interna), determinismo entre duas execucoes e assimetria causal aprendida pelo STDP.
//! Qualquer uma que falhe derruba o relatorio.

use stochology_hypergraph::Lente;

use crate::complexo::Complexo;
use crate::espicula::BarramentoStdp;
use crate::magnetico::{HipergrafoMag, verifica_cadeia};
use crate::selo::Selo;

/// Resultado de um portao.
#[derive(Debug, Clone, PartialEq)]
pub struct Resultado {
    pub portao: &'static str,
    pub passou: bool,
    pub detalhe: String,
}

/// Relatorio completo.
#[derive(Debug, Clone, PartialEq)]
pub struct Relatorio {
    pub resultados: Vec<Resultado>,
    pub selo: u64,
}

impl Relatorio {
    pub fn passou(&self) -> bool {
        !self.resultados.is_empty() && self.resultados.iter().all(|r| r.passou)
    }

    pub fn falhas(&self) -> Vec<&Resultado> {
        self.resultados.iter().filter(|r| !r.passou).collect()
    }
}

/// Cenario submetido ao portao.
#[derive(Debug, Clone)]
pub struct Cenario<'a> {
    pub grafo: &'a HipergrafoMag,
    pub pulso: Vec<Complexo>,
    pub barramento: &'a BarramentoStdp,
    pub cadeia: Vec<(Lente, Lente)>,
    pub saltos: usize,
    pub dt: f64,
    /// Assimetria causal minima exigida em cada elo da cadeia.
    pub assimetria_minima: f64,
}

fn resultado(portao: &'static str, veredito: Result<(), String>, detalhe: String) -> Resultado {
    match veredito {
        Ok(()) => Resultado {
            portao,
            passou: true,
            detalhe,
        },
        Err(e) => Resultado {
            portao,
            passou: false,
            detalhe: e,
        },
    }
}

/// Roda os seis portoes.
pub fn verifica(cenario: &Cenario<'_>) -> Relatorio {
    let mut resultados = Vec::new();

    // G1: estrutura.
    let estrutura = {
        let problemas = cenario.grafo.valida();
        let tamanho_ok = cenario.pulso.len() == cenario.grafo.vertices.len();
        let dt_ok = cenario.dt > 0.0 && cenario.dt.is_finite();
        if !problemas.is_empty() {
            Err(problemas.join("; "))
        } else if !tamanho_ok {
            Err(format!(
                "pulso com {} posicoes para {} vertices",
                cenario.pulso.len(),
                cenario.grafo.vertices.len()
            ))
        } else if !dt_ok {
            Err(format!("dt invalido: {}", cenario.dt))
        } else {
            Ok(())
        }
    };
    resultados.push(resultado(
        "G1 estrutura",
        estrutura,
        format!(
            "{} vertices, {} hiperarestas, q = {}",
            cenario.grafo.vertices.len(),
            cenario.grafo.hiperarestas.len(),
            cenario.grafo.q
        ),
    ));

    // G2: Hermiticidade.
    let lap = cenario.grafo.laplaciano();
    let hermitiano = cenario.grafo.confere_hermitiano(&lap).map(|_| ());
    let assimetria = cenario.grafo.assimetria_hermitiana(&lap);
    resultados.push(resultado(
        "G2 hermitiano",
        hermitiano,
        format!("maior desvio de L^(q) em relacao a L^(q)^dagger: {assimetria:.3e}"),
    ));

    // G3: conservacao da norma.
    let norma_inicial: f64 = cenario.pulso.iter().map(|c| c.modulo2()).sum();
    let unitario = match cenario
        .grafo
        .propaga(&cenario.pulso, cenario.saltos, cenario.dt)
    {
        Ok(p) => {
            let desvio = (p.norma2 - norma_inicial).abs();
            if desvio < 1e-9 {
                Ok(())
            } else {
                Err(format!(
                    "norma mudou de {norma_inicial:.12} para {:.12}",
                    p.norma2
                ))
            }
        }
        Err(e) => Err(e),
    };
    resultados.push(resultado(
        "G3 unitario",
        unitario,
        format!(
            "norma l2 preservada em {} saltos de dt = {}",
            cenario.saltos, cenario.dt
        ),
    ));

    // G4: cadeia de selos, com prova negativa embutida.
    let cadeia = match cenario
        .grafo
        .propaga_auditado(&cenario.pulso, cenario.saltos, cenario.dt)
    {
        Ok(saltos) => match verifica_cadeia(&saltos) {
            Ok(()) => {
                let mut corrompido = saltos.clone();
                if let Some(s) = corrompido.last_mut()
                    && let Some(c) = s.estado.first_mut()
                {
                    c.re += 0.001;
                }
                if verifica_cadeia(&corrompido).is_err() {
                    Ok(())
                } else {
                    Err("prova negativa nao detectou adulteracao de estado".to_string())
                }
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    };
    resultados.push(resultado(
        "G4 cadeia",
        cadeia,
        format!(
            "selo de estado encadeado em {} saltos; adulteracao de 0.001 rejeitada",
            cenario.saltos
        ),
    ));

    // G5: determinismo.
    let determinismo = match (
        cenario
            .grafo
            .propaga_auditado(&cenario.pulso, cenario.saltos, cenario.dt),
        cenario
            .grafo
            .propaga_auditado(&cenario.pulso, cenario.saltos, cenario.dt),
    ) {
        (Ok(a), Ok(b)) => {
            let sa = a.last().map(|s| s.selo_cadeia);
            let sb = b.last().map(|s| s.selo_cadeia);
            if sa.is_some() && sa == sb {
                Ok(())
            } else {
                Err("duas execucoes do mesmo pulso divergiram".to_string())
            }
        }
        (Err(e), _) | (_, Err(e)) => Err(e),
    };
    resultados.push(resultado(
        "G5 determinismo",
        determinismo,
        "duas execucoes independentes selam o mesmo estado".to_string(),
    ));

    // G6: causalidade aprendida.
    let causalidade = cenario
        .barramento
        .confere_causalidade(&cenario.cadeia, cenario.assimetria_minima);
    resultados.push(resultado(
        "G6 causalidade",
        causalidade,
        format!(
            "assimetria minima exigida de {:.2} em {} elos; selo sinaptico 0x{:016x}",
            cenario.assimetria_minima,
            cenario.cadeia.len(),
            cenario.barramento.selo()
        ),
    ));

    let mut selo = Selo::novo();
    for r in &resultados {
        selo.escreve_texto(r.portao);
        selo.escreve_u64(r.passou as u64);
    }
    selo.escreve_u64(cenario.grafo.selo_topologia());
    selo.escreve_u64(cenario.barramento.selo());

    Relatorio {
        resultados,
        selo: selo.fecha(),
    }
}
