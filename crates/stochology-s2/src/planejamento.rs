//! O eixo R pelo Sistema 2: a resolucao da pergunta escolhida pelo instrumento, nunca por palpite.
//!
//! Com o mesmo recurso total, perguntar pelo agregado perde o que perguntar pelo dia entrega. O Sistema 2
//! avalia os dois bracos com `stochology_core::medir` e compara a perda medida com o **custo declarado** da
//! margem que o braco fino exige. As resolucoes mais finas (R2 e R3) ficam declaradas fora do alcance
//! medido: o instrumento mede o par agregado/fino, e extrapolar seria inventar numero.
//!
//! Declarado e nao implementado: o acoplamento do plano com o orcamento real de atendentes (o custo da
//! margem entra como numero declarado pelo decisor) e a ligacao com o cofre de auditoria.

use stochology_core::{PerdaDeResolucao, Resolucao};

/// O que a avaliacao devolve para cada degrau da escada de resolucao.
#[derive(Debug, Clone, PartialEq)]
pub struct Avaliacao {
    pub resolucao: Resolucao,
    pub perda_min: f64,
    pub custo_min: f64,
    pub total_min: f64,
    pub fora_do_alcance: Option<&'static str>,
}

/// A decisao do Sistema 2 sobre a resolucao da pergunta.
#[derive(Debug, Clone, PartialEq)]
pub struct DecisaoDeResolucao {
    pub escolhida: Resolucao,
    pub perda_do_agregado_min: f64,
    pub custo_da_margem_min: f64,
    pub motivo: &'static str,
}

/// O motivo declarado de R2 e R3 ficarem fora: o instrumento nao mede esses degraus hoje.
pub const FORA_DO_ALCANCE: &str =
    "o instrumento mede o par agregado/fino; o degrau nao e extrapolado";

/// Avalia os quatro degraus: R0 sem perda e sem custo, R1 com a perda medida e o custo declarado,
/// R2 e R3 declarados fora do alcance medido.
pub fn avaliar(perda: &PerdaDeResolucao, custo_da_margem_min: f64) -> Vec<Avaliacao> {
    let custo = custo_da_margem_min.max(0.0);
    vec![
        Avaliacao {
            resolucao: Resolucao::R0Agregada,
            perda_min: 0.0,
            custo_min: 0.0,
            total_min: 0.0,
            fora_do_alcance: None,
        },
        Avaliacao {
            resolucao: Resolucao::R1Dia,
            perda_min: perda.perda,
            custo_min: custo,
            total_min: perda.perda - custo,
            fora_do_alcance: None,
        },
        Avaliacao {
            resolucao: Resolucao::R2Hora,
            perda_min: f64::NAN,
            custo_min: f64::NAN,
            total_min: f64::NAN,
            fora_do_alcance: Some(FORA_DO_ALCANCE),
        },
        Avaliacao {
            resolucao: Resolucao::R3Subpergunta,
            perda_min: f64::NAN,
            custo_min: f64::NAN,
            total_min: f64::NAN,
            fora_do_alcance: Some(FORA_DO_ALCANCE),
        },
    ]
}

/// A regra de decisao do Sistema 2: fica com o braco fino quando a perda do agregado paga a margem.
pub fn decidir(perda: &PerdaDeResolucao, custo_da_margem_min: f64) -> DecisaoDeResolucao {
    let custo = custo_da_margem_min.max(0.0);
    let fina = perda.perda > custo;
    DecisaoDeResolucao {
        escolhida: if fina {
            Resolucao::R1Dia
        } else {
            Resolucao::R0Agregada
        },
        perda_do_agregado_min: perda.perda,
        custo_da_margem_min: custo,
        motivo: if fina {
            "a perda do agregado paga a margem do dia"
        } else {
            "a margem do dia nao se paga: o agregado basta"
        },
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use stochology_core::medir_perda_de_resolucao;

    /// As chamadas sao as do proprio instrumento (`labs --resolucao`): 1909, 200 dias, 10 horas.
    fn posto_do_capitulo_16() -> PerdaDeResolucao {
        medir_perda_de_resolucao(1909, 200, 10, 5.0, 11.0)
    }

    fn sem_oscilacao() -> PerdaDeResolucao {
        medir_perda_de_resolucao(1909, 200, 10, 8.0, 8.0)
    }

    #[test]
    fn sem_oscilacao_a_perda_e_zero_e_o_agregado_basta() {
        let perda = sem_oscilacao();
        assert_eq!(perda.perda, 0.0);
        let decisao = decidir(&perda, 12.0);
        assert_eq!(decisao.escolhida, Resolucao::R0Agregada);
        assert_eq!(
            decisao.motivo,
            "a margem do dia nao se paga: o agregado basta"
        );
    }

    #[test]
    fn com_oscilacao_a_perda_paga_a_margem() {
        let perda = posto_do_capitulo_16();
        assert!(
            perda.perda > 40.0,
            "a perda medida no posto do capitulo 16: {}",
            perda.perda
        );
        let decisao = decidir(&perda, 12.0);
        assert_eq!(decisao.escolhida, Resolucao::R1Dia);
        assert_eq!(decisao.perda_do_agregado_min, perda.perda);
    }

    #[test]
    fn margem_cara_volta_para_o_agregado() {
        let perda = posto_do_capitulo_16();
        let decisao = decidir(&perda, 100.0);
        assert_eq!(decisao.escolhida, Resolucao::R0Agregada);
        assert!(decisao.perda_do_agregado_min < decisao.custo_da_margem_min);
    }

    #[test]
    fn a_avaliacao_declara_os_degraus_fora_do_alcance() {
        let perda = posto_do_capitulo_16();
        let avaliacoes = avaliar(&perda, 12.0);
        assert_eq!(avaliacoes.len(), 4);
        assert_eq!(avaliacoes[0].resolucao, Resolucao::R0Agregada);
        assert_eq!(avaliacoes[1].total_min, perda.perda - 12.0);
        assert!(avaliacoes[2].fora_do_alcance.is_some());
        assert!(avaliacoes[3].fora_do_alcance.is_some());
        assert!(avaliacoes[2].perda_min.is_nan());
        assert!(avaliacoes[0].fora_do_alcance.is_none());
    }

    #[test]
    fn a_escada_de_resolucao_vem_do_core() {
        assert_eq!(
            Resolucao::TODAS.map(Resolucao::codigo),
            ["R0-agregada", "R1-dia", "R2-hora", "R3-subpergunta"]
        );
    }
}
