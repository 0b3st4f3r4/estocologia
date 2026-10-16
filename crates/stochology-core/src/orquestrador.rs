//! Orquestrador mestre M: conduz o ciclo e submete a ficha à banca A1-A8.

use crate::agentes::{Aprovacao, AgenteA, Severidade, aprovado, estressar};
use crate::camada::Ficha;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrquestradorM {
    pub id_sessao: String,
    pub hash_trilha: String,
}

impl OrquestradorM {
    pub fn novo(id_sessao: &str, hash_trilha: &str) -> Self {
        OrquestradorM {
            id_sessao: id_sessao.to_string(),
            hash_trilha: hash_trilha.to_string(),
        }
    }

    /// Conduz o estresse adversarial usando o agente A_i indicado.
    pub fn conduzir_ataque(&self, agente: AgenteA, ficha: &Ficha) -> Result<Severidade, String> {
        if ficha.id.trim().is_empty() {
            return Err("ficha sem identificador".to_string());
        }
        let semente = ficha.semente.unwrap_or(2026);
        Ok(estressar(agente, ficha, semente))
    }

    /// Roda a banca inteira e devolve o veredito com as oito severidades.
    pub fn banca(&self, ficha: &Ficha) -> Aprovacao {
        aprovado(ficha, ficha.semente.unwrap_or(2026))
    }
}

#[cfg(test)]
mod testes {
    use super::OrquestradorM;
    use crate::agentes::AgenteA;
    use crate::camada::{Camada, Ficha, TipoIncerteza};

    #[test]
    fn banca_reprova_ficha_incompleta() {
        let m = OrquestradorM::novo("sessao-h0", "0".repeat(64).as_str());
        let f = Ficha::nova("h0.sem-cuidados", Camada::Mundo, TipoIncerteza::Aleatoria);
        assert!(!m.banca(&f).aprovado);
    }

    #[test]
    fn ataque_unico_roda() {
        let m = OrquestradorM::novo("sessao-h1", "1".repeat(64).as_str());
        let f = Ficha::nova("h1.a", Camada::Modelo, TipoIncerteza::Epistemica);
        assert!(m.conduzir_ataque(AgenteA::A3CicloVida, &f).is_ok());
    }
}
