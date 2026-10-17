//! Banca adversarial A1-A8: cada agente carrega a severidade do seu teste.

use crate::camada::Ficha;
use crate::rng::Rng;

/// Escala de severidade de 0 a 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Severidade(pub u8);

/// Piso de aprovação: todo A_i precisa atingir este valor.
pub const SEVERIDADE_MINIMA: Severidade = Severidade(6);

/// Os oito agentes da banca adversarial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgenteA {
    A1Codigo,
    A2Dados,
    A3CicloVida,
    A4Inferencia,
    A5Governanca,
    A6Justica,
    A7Dominio,
    A8Decisao,
}

impl AgenteA {
    pub const TODOS: [AgenteA; 8] = [
        AgenteA::A1Codigo,
        AgenteA::A2Dados,
        AgenteA::A3CicloVida,
        AgenteA::A4Inferencia,
        AgenteA::A5Governanca,
        AgenteA::A6Justica,
        AgenteA::A7Dominio,
        AgenteA::A8Decisao,
    ];

    pub fn nome(self) -> &'static str {
        match self {
            AgenteA::A1Codigo => "A1-codigo",
            AgenteA::A2Dados => "A2-dados",
            AgenteA::A3CicloVida => "A3-ciclo-de-vida",
            AgenteA::A4Inferencia => "A4-inferencia",
            AgenteA::A5Governanca => "A5-governanca",
            AgenteA::A6Justica => "A6-justica",
            AgenteA::A7Dominio => "A7-dominio",
            AgenteA::A8Decisao => "A8-decisao",
        }
    }

    pub fn alvo(self) -> &'static str {
        match self {
            AgenteA::A1Codigo => "testes de propriedades em codigo",
            AgenteA::A2Dados => "injecao de ruido e valores sentinela",
            AgenteA::A3CicloVida => "deriva temporal ao vivo",
            AgenteA::A4Inferencia => "patologias inferenciais",
            AgenteA::A5Governanca => "adulteracao da trilha de hash",
            AgenteA::A6Justica => "inversao de metricas de subgrupo",
            AgenteA::A7Dominio => "choques de priori e dominio",
            AgenteA::A8Decisao => "crise violando a clausula de saida",
        }
    }

    /// O cuidado da ficha que este agente cobra.
    fn cuidado_atendido(self, f: &Ficha) -> bool {
        match self {
            AgenteA::A1Codigo => f.teste_de_propriedade,
            AgenteA::A2Dados => f.trata_ausentes,
            AgenteA::A3CicloVida => f.deriva_declarada,
            AgenteA::A4Inferencia => f.patologias_cobertas,
            AgenteA::A5Governanca => f.trilha_integra,
            AgenteA::A6Justica => f.subgrupos_auditados,
            AgenteA::A7Dominio => f.priori_elicitada,
            AgenteA::A8Decisao => f.clausula_de_saida.is_some(),
        }
    }

    /// A âncora estrutural da ficha na faixa deste agente: cuidado declarado sem campo estrutural cai na borda 5..=6.
    pub fn ancora_estrutural(self, f: &Ficha) -> bool {
        match self {
            AgenteA::A1Codigo => {
                f.artefato_simulavel.as_ref().is_some_and(|s| !s.is_empty()) && f.semente.is_some()
            }
            AgenteA::A2Dados => {
                f.unidade_independente
                    .as_ref()
                    .is_some_and(|s| !s.is_empty())
                    && f.margem.is_some_and(|m| m > 0.0 && m < 1.0)
            }
            AgenteA::A3CicloVida => f.escopo_ecologico && f.semente.is_some(),
            AgenteA::A4Inferencia => {
                f.modelo_nulo.as_ref().is_some_and(|s| !s.is_empty())
                    && f.margem.is_some_and(|m| m > 0.0 && m < 1.0)
            }
            AgenteA::A5Governanca => {
                !f.id.is_empty() && f.artefato_simulavel.as_ref().is_some_and(|s| !s.is_empty())
            }
            AgenteA::A6Justica => f
                .quem_paga_pelo_erro
                .as_ref()
                .is_some_and(|s| !s.is_empty()),
            AgenteA::A7Dominio => !f.definicao.is_empty() || f.escopo_ecologico,
            AgenteA::A8Decisao => {
                f.quem_paga_pelo_erro
                    .as_ref()
                    .is_some_and(|s| !s.is_empty())
                    && f.clausula_de_saida.as_ref().is_some_and(|s| !s.is_empty())
            }
        }
    }

    fn semente(self, semente_base: u64) -> u64 {
        let ordinal = self as u64 + 1;
        semente_base ^ ordinal.wrapping_mul(0x9E37_79B9_7F4A_7C15)
    }
}

/// Severidade do ataque A_i sobre a ficha: determinística para a semente dada.
///
/// Quatro regimes cobrem a escala 1..=10 sem deixar o piso 6 inacessível:
/// - cuidado + âncora estrutural: 8..=10 (aprovado com folga);
/// - cuidado declarado sem âncora estrutural: 5..=6 (fronteira exata do piso 6);
/// - âncora estrutural sem cuidado: 3..=5 (reprovado na borda inferior do piso);
/// - sem cuidado e sem âncora: 1..=3 (reprovado).
pub fn estressar(agente: AgenteA, ficha: &Ficha, semente: u64) -> Severidade {
    let mut rng = Rng::novo(agente.semente(semente));
    match (
        agente.cuidado_atendido(ficha),
        agente.ancora_estrutural(ficha),
    ) {
        (true, true) => Severidade((8 + rng.uniforme(3)) as u8),
        (true, false) => Severidade((5 + rng.uniforme(2)) as u8),
        (false, true) => Severidade((3 + rng.uniforme(3)) as u8),
        (false, false) => Severidade((1 + rng.uniforme(3)) as u8),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aprovacao {
    pub itens: Vec<(AgenteA, Severidade)>,
    pub aprovado: bool,
}

impl Aprovacao {
    pub fn severidade(&self, agente: AgenteA) -> Severidade {
        self.itens
            .iter()
            .find(|(a, _)| *a == agente)
            .map(|(_, s)| *s)
            .unwrap_or(Severidade(0))
    }
}

/// Condição do PLAN.md: Aprovado(X) = AND_{i=1..8} A_i(X) >= Severidade_min.
pub fn aprovado(ficha: &Ficha, semente: u64) -> Aprovacao {
    let itens: Vec<(AgenteA, Severidade)> = AgenteA::TODOS
        .iter()
        .map(|a| (*a, estressar(*a, ficha, semente)))
        .collect();
    let aprovado = itens.iter().all(|(_, s)| *s >= SEVERIDADE_MINIMA);
    Aprovacao { itens, aprovado }
}

#[cfg(test)]
mod testes {
    use super::{AgenteA, SEVERIDADE_MINIMA, aprovado, estressar};
    use crate::camada::{Camada, Ficha, TipoIncerteza};

    fn ficha_boa() -> Ficha {
        // A8 cobra a cláusula de saída, que vem no cabeçalho completo.
        Ficha::nova("h3.voos", Camada::Agente, TipoIncerteza::Epistemica)
            .com_cuidados()
            .completa()
    }

    #[test]
    fn oito_agentes_aprovam_ficha_com_cuidados() {
        let r = aprovado(&ficha_boa(), 2026);
        assert_eq!(r.itens.len(), 8);
        assert!(r.aprovado, "ficha com os oito cuidados deveria passar");
        for (a, s) in &r.itens {
            assert!(*s >= SEVERIDADE_MINIMA, "{} reprovou", a.nome());
        }
    }

    #[test]
    fn falta_de_cuidado_reprova_o_agente_da_faixa() {
        let mut f = ficha_boa();
        f.trilha_integra = false;
        let r = aprovado(&f, 2026);
        assert!(!r.aprovado);
        assert!(r.severidade(AgenteA::A5Governanca) < SEVERIDADE_MINIMA);
        assert!(r.severidade(AgenteA::A1Codigo) >= SEVERIDADE_MINIMA);
    }

    #[test]
    fn severidade_reproduz_com_a_mesma_semente() {
        let f = ficha_boa();
        for a in AgenteA::TODOS {
            assert_eq!(estressar(a, &f, 7), estressar(a, &f, 7));
        }
    }

    #[test]
    fn cuidado_sem_ancora_estrutural_testa_a_fronteira_5_e_6_e_reprova_carimbo_vazio() {
        let mut sem_ancora =
            Ficha::nova("carimbo.vazio", Camada::Modelo, TipoIncerteza::Epistemica).com_cuidados();
        sem_ancora.clausula_de_saida = Some("parar".into());
        // Sem .completa(), faltam artefato, semente, unidade, modelo nulo e quem paga pelo erro:
        // os agentes caem no regime (true, false) -> 5 ou 6, tocando a fronteira exata do piso 6.
        let mut viu_cinco = false;
        let mut viu_seis = false;
        for semente in 1..=32 {
            for a in [
                AgenteA::A1Codigo,
                AgenteA::A2Dados,
                AgenteA::A4Inferencia,
                AgenteA::A6Justica,
            ] {
                let s = estressar(a, &sem_ancora, semente);
                assert!(
                    s.0 == 5 || s.0 == 6,
                    "esperava fronteira 5..=6, veio {}",
                    s.0
                );
                if s.0 == 5 {
                    viu_cinco = true;
                }
                if s.0 == 6 {
                    viu_seis = true;
                }
            }
        }
        assert!(
            viu_cinco && viu_seis,
            "a fronteira 5 e 6 precisa ser alcancavel"
        );
        let r = aprovado(&sem_ancora, 2026);
        assert!(
            !r.aprovado,
            "ficha com booleano marcado mas sem ancora estrutural nao pode passar"
        );
    }
}
