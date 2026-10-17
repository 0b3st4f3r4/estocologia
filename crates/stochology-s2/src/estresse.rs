//! Estresse adversarial deliberativo: a banca A1-A8 atacada na ordem que a crenca aprendeu.
//!
//! O estresse comum roda as oito faixas sempre na mesma ordem. Aqui o Sistema 2 mantem um **estado de
//! crenca sobre as faixas** — alimentado pelas fichas que a banca ja reprovou — e ataca primeiro quem a
//! crenca aponta como faixa fraca, parando no primeiro reprovado.
//!
//! O nulo declarado e a ordem fixa `A1 ... A8` (`ORDEM_DECLARADA`): sem aprendizado, a ordem deliberativa
//! coincide com ela. O que se mede e o numero de ataques ate o primeiro reprovado, medido no proprio
//! `stochology-core`, e nao o acerto de uma intuicao.
//!
//! Declarado e nao implementado: o custo assimetrico de cada ataque (aqui um ataque custa o mesmo que
//! qualquer outro) e a escolha de qual cuidado acrescentar a ficha depois do reprovado.

use stochology_core::{
    AgenteA, Aprovacao, Ficha, SEVERIDADE_MINIMA, Severidade, aprovado, estressar,
};

use crate::crenca::{EstadoCrenca, crenca_da_banca};

/// A ordem declarada da banca: A1 a A8, sem nenhuma deliberacao.
pub const ORDEM_DECLARADA: [AgenteA; 8] = AgenteA::TODOS;

/// O que o ataque gastou: quantos ataques ate o primeiro reprovado, e com quem parou.
#[derive(Debug, Clone, PartialEq)]
pub struct Atuacao {
    pub agente: AgenteA,
    pub severidade: Severidade,
    pub ataques: usize,
    pub reprovou: bool,
    pub trilha: Vec<(AgenteA, Severidade)>,
}

/// A crenca do Sistema 2 sobre as faixas: qual delas vai reprovar a proxima ficha.
#[derive(Debug, Clone, PartialEq)]
pub struct CrencaDaBanca {
    crenca: EstadoCrenca,
}

impl Default for CrencaDaBanca {
    fn default() -> Self {
        CrencaDaBanca::nova()
    }
}

impl CrencaDaBanca {
    pub fn nova() -> Self {
        CrencaDaBanca {
            crenca: crenca_da_banca(),
        }
    }

    /// Aprende com uma ficha que ja passou pela banca inteira.
    pub fn atualizar(&mut self, aprovacao: &Aprovacao, peso: f64) {
        self.crenca.observar_aprovacao(aprovacao, peso);
    }

    pub fn probabilidade(&self, agente: AgenteA) -> f64 {
        self.crenca.probabilidade(agente.nome()).unwrap_or(0.0)
    }

    pub fn entropia(&self) -> f64 {
        self.crenca.entropia()
    }

    /// As faixas em ordem de ataque: a mais provavel de reprovar primeiro.
    pub fn ordem(&self) -> Vec<AgenteA> {
        let ranking = self.crenca.ranking();
        let mut ordem: Vec<AgenteA> = Vec::with_capacity(ranking.len());
        for (nome, _) in ranking {
            if let Some(agente) = AgenteA::TODOS.iter().find(|a| a.nome() == nome) {
                ordem.push(*agente);
            }
        }
        for agente in AgenteA::TODOS {
            if !ordem.contains(&agente) {
                ordem.push(agente);
            }
        }
        ordem
    }
}

/// Roda a banca na ordem dada e para no primeiro reprovado.
pub fn estressar_em_ordem(ficha: &Ficha, semente: u64, ordem: &[AgenteA]) -> Atuacao {
    let mut trilha = Vec::new();
    for agente in ordem {
        let severidade = estressar(*agente, ficha, semente);
        trilha.push((*agente, severidade));
        if severidade < SEVERIDADE_MINIMA {
            return Atuacao {
                agente: *agente,
                severidade,
                ataques: trilha.len(),
                reprovou: true,
                trilha,
            };
        }
    }
    let ultimo = ordem.last().copied().unwrap_or(AgenteA::A8Decisao);
    let severidade = trilha
        .iter()
        .find(|(a, _)| *a == ultimo)
        .map(|(_, s)| *s)
        .unwrap_or(Severidade(0));
    Atuacao {
        agente: ultimo,
        severidade,
        ataques: trilha.len(),
        reprovou: false,
        trilha,
    }
}

/// O ataque deliberativo: a ordem vem da crenca, e o resultado e o mesmo de `estressar_em_ordem`.
pub fn estressar_deliberativo(ficha: &Ficha, semente: u64, banca: &CrencaDaBanca) -> Atuacao {
    estressar_em_ordem(ficha, semente, &banca.ordem())
}

/// Aprende com um lote de fichas, como o Sistema 2 faria antes de atacar a proxima.
pub fn aprender(banca: &mut CrencaDaBanca, fichas: &[Ficha], semente: u64, peso: f64) {
    for ficha in fichas {
        banca.atualizar(&aprovado(ficha, semente), peso);
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::crenca::{PISO, TETO};
    use stochology_core::{Camada, Ficha, TipoIncerteza};

    /// A ficha nasce com os oito cuidados: `com_cuidados` fecha os sete campos booleanos e `completa`
    /// traz o cabecalho, inclusive a clausula de saida que A8 cobra.
    fn ficha_completa(id: &str) -> Ficha {
        Ficha::nova(id, Camada::Agente, TipoIncerteza::Epistemica)
            .com_cuidados()
            .completa()
    }

    fn fichas_sem_trilha() -> Vec<Ficha> {
        (0..4)
            .map(|i| {
                let mut f = ficha_completa(&format!("treino.{i}"));
                f.trilha_integra = false;
                f
            })
            .collect()
    }

    #[test]
    fn a_crenca_aprende_a_faixa_fraca_e_ataca_ela_primeiro() {
        let mut banca = CrencaDaBanca::nova();
        aprender(&mut banca, &fichas_sem_trilha(), 2026, 3.0);
        assert_eq!(banca.ordem()[0], AgenteA::A5Governanca);
        let candidata = {
            let mut f = ficha_completa("alvo.1");
            f.trilha_integra = false;
            f
        };
        let deliberado = estressar_deliberativo(&candidata, 2026, &banca);
        let declarado = estressar_em_ordem(&candidata, 2026, &ORDEM_DECLARADA);
        assert!(deliberado.reprovou && declarado.reprovou);
        assert_eq!(deliberado.agente, AgenteA::A5Governanca);
        assert_eq!(
            deliberado.ataques, 1,
            "a crenca acertou de primeira: {deliberado:?}"
        );
        assert_eq!(
            declarado.ataques, 5,
            "na ordem declarada, A5 e o quinto: {declarado:?}"
        );
    }

    #[test]
    fn sem_aprendizado_a_ordem_deliberativa_e_a_declarada() {
        let banca = CrencaDaBanca::nova();
        assert_eq!(banca.ordem(), ORDEM_DECLARADA.to_vec());
    }

    #[test]
    fn ficha_com_os_oito_cuidados_nao_reprova_em_nenhuma_ordem() {
        let mut banca = CrencaDaBanca::nova();
        aprender(&mut banca, &fichas_sem_trilha(), 2026, 3.0);
        let boa = ficha_completa("boa.1");
        let deliberado = estressar_deliberativo(&boa, 2026, &banca);
        assert!(
            !deliberado.reprovou,
            "o estresse nao e cacada as bruxas: {deliberado:?}"
        );
        assert_eq!(deliberado.ataques, 8, "sem reprovado, a banca inteira roda");
        assert!(
            deliberado
                .trilha
                .iter()
                .all(|(_, s)| *s >= SEVERIDADE_MINIMA)
        );
    }

    #[test]
    fn a_crenca_da_banca_respeita_o_piso_e_o_teto() {
        let mut banca = CrencaDaBanca::nova();
        aprender(&mut banca, &fichas_sem_trilha(), 2026, 3.0);
        let mut soma = 0.0;
        for agente in AgenteA::TODOS {
            let p = banca.probabilidade(agente);
            assert!(
                (PISO - 1e-12..=TETO + 1e-12).contains(&p),
                "{} em {p}",
                agente.nome()
            );
            soma += p;
        }
        assert!((soma - 1.0).abs() < 0.02, "soma das probabilidades: {soma}");
        assert!(banca.entropia() > 0.0);
    }

    #[test]
    fn o_ataque_e_deterministico() {
        let banca = CrencaDaBanca::nova();
        let ficha = ficha_completa("determinismo.1");
        assert_eq!(
            estressar_deliberativo(&ficha, 2026, &banca),
            estressar_deliberativo(&ficha, 2026, &banca)
        );
        assert_eq!(
            estressar_em_ordem(&ficha, 2026, &ORDEM_DECLARADA),
            estressar_em_ordem(&ficha, 2026, &ORDEM_DECLARADA)
        );
    }
}
