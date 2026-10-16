//! Capítulo 2 — Mundo, Modelo e Agente: a falácia mora na ponte.
//!
//! A falácia do promotor troca P(evidência | inocente) por P(inocente | evidência).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ponte {
    /// O que o Modelo entrega: P(evidência | inocente).
    pub verossimilhanca: f64,
    /// O que o Agente responde: P(inocente | evidência).
    pub posterior_de_inocencia: f64,
}

/// Calcula as duas quantidades que o promotor confunde.
pub fn falacia_do_promotor(
    prevalencia: f64,
    sensibilidade: f64,
    falso_positivo: f64,
) -> Ponte {
    let p_evidencia = prevalencia * sensibilidade + (1.0 - prevalencia) * falso_positivo;
    let posterior_de_culpa = prevalencia * sensibilidade / p_evidencia;
    Ponte {
        verossimilhanca: falso_positivo,
        posterior_de_inocencia: 1.0 - posterior_de_culpa,
    }
}

/// A razão entre o que foi dito e o que se podia concluir.
pub fn fator_da_troca(prevalencia: f64, sensibilidade: f64, falso_positivo: f64) -> f64 {
    let p = falacia_do_promotor(prevalencia, sensibilidade, falso_positivo);
    p.posterior_de_inocencia / p.verossimilhanca
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn coincidencia_rara_nao_vira_culpa_em_populacao_pequena() {
        // Coincidência de 1 em 1 milhão, mas a chance prévia de o réu ser o culpado
        // também é de 1 em 1 milhão: a evidência separa pouco.
        let p = falacia_do_promotor(1.0 / 1_000_000.0, 1.0, 1.0 / 1_000_000.0);
        assert!((p.verossimilhanca - 1e-6).abs() < 1e-18);
        assert!(p.posterior_de_inocencia > 0.49 && p.posterior_de_inocencia < 0.51);
    }

    #[test]
    fn quando_a_prevalencia_e_alta_a_ponte_quase_nao_mente() {
        let p = falacia_do_promotor(0.9, 1.0, 1e-6);
        assert!(p.posterior_de_inocencia < 1e-5);
    }

    #[test]
    fn o_fator_da_troca_cresce_quando_a_prevalencia_cai() {
        let alto = fator_da_troca(0.01, 1.0, 1e-6);
        let baixo = fator_da_troca(1e-6, 1.0, 1e-6);
        assert!(baixo > alto * 1000.0, "{} vs {}", alto, baixo);
    }
}
