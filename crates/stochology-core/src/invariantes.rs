//! Invariantes estruturais Phi1-Phi5 e a trava contra o Erro Tipo III.

use crate::camada::Ficha;
use crate::resolucao::Resolucao;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Portao {
    Phi1,
    Phi2,
    Phi3,
    Phi4,
    Phi5,
    TipoIII,
}

impl Portao {
    pub fn codigo(self) -> &'static str {
        match self {
            Portao::Phi1 => "Phi1-simulabilidade",
            Portao::Phi2 => "Phi2-unidade-independente",
            Portao::Phi3 => "Phi3-camadas-e-modelo-nulo",
            Portao::Phi4 => "Phi4-escopo-e-saida",
            Portao::Phi5 => "Phi5-resolucao-da-pergunta",
            Portao::TipoIII => "tipo-III-pergunta-do-decisor",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Falha {
    pub portao: Portao,
    pub detalhe: String,
}

impl Falha {
    /// Linha do relatório: o código do portão seguido do detalhe.
    pub fn linha(&self) -> String {
        format!("{}: {}", self.portao.codigo(), self.detalhe)
    }
}

/// Relatório de falhas, uma linha por portão reprovado.
pub fn relatorio(falhas: &[Falha]) -> String {
    if falhas.is_empty() {
        return "sem falhas: Phi1-Phi5 e a trava de tipo III aprovados".to_string();
    }
    falhas
        .iter()
        .map(Falha::linha)
        .collect::<Vec<String>>()
        .join("\n")
}

/// Marcas de grão do dia ou mais grosso na unidade independente declarada.
///
/// Limite declarado: unidade mais fina que o dia (hora, cliente, subgrupo) fica fora
/// desta checagem — a ficha só é cobrada quando o dado já está no dia ou acima dele.
fn no_grao_do_dia_ou_maior(unidade: &str) -> bool {
    const ACIMA: [&str; 12] = [
        "semana",
        "mes",
        "mês",
        "mensal",
        "trimestr",
        "semestr",
        "ano",
        "anual",
        "agregad",
        "média",
        "media",
        "global",
    ];
    const DIA: [&str; 4] = ["dia", "diari", "jornada", "24h"];
    let u = unidade.trim().to_lowercase();
    ACIMA.iter().any(|m| u.contains(m)) || DIA.iter().any(|m| u.contains(m))
}

/// Avalia a ficha contra os cinco invariantes e a trava de Tipo III.
pub fn avaliar(f: &Ficha) -> Vec<Falha> {
    let mut falhas = Vec::new();

    if f.semente.is_none() || f.artefato_simulavel.is_none() {
        falhas.push(Falha {
            portao: Portao::Phi1,
            detalhe: "falta semente fixa ou artefato simulável".to_string(),
        });
    }

    if f.margem.is_none() || f.unidade_independente.is_none() {
        falhas.push(Falha {
            portao: Portao::Phi2,
            detalhe: "margem sem unidade independente declarada".to_string(),
        });
    }

    if f.modelo_nulo.is_none() || f.definicao.trim().is_empty() {
        falhas.push(Falha {
            portao: Portao::Phi3,
            detalhe: "sem modelo nulo ou sem definição em uma linha".to_string(),
        });
    }

    if !f.escopo_ecologico || f.quem_paga_pelo_erro.is_none() || f.clausula_de_saida.is_none() {
        falhas.push(Falha {
            portao: Portao::Phi4,
            detalhe: "escopo ecológico, quem paga pelo erro ou cláusula de saída ausente".to_string(),
        });
    }

    // Phi5: a ação declarada tem de estar no mesmo grão da unidade independente. Se a
    // unidade é o dia (ou maior) e a resolução é R0Agregada, a ação está num grão mais
    // grosso que o dado. R1Dia ou mais fina passa.
    if let Some(unidade) = f.unidade_independente.as_deref() {
        if no_grao_do_dia_ou_maior(unidade) && f.resolucao == Resolucao::R0Agregada {
            falhas.push(Falha {
                portao: Portao::Phi5,
                detalhe: format!(
                    "ação em {} contra unidade independente \"{}\": a ação está num grão mais grosso que o dado",
                    f.resolucao.codigo(),
                    unidade.trim()
                ),
            });
        }
    }

    let pergunta = f.pergunta_do_decisor.trim();
    let acao = f.acao_do_pacote.trim().to_lowercase();
    let casa_com_acao = !acao.is_empty() && pergunta.to_lowercase().contains(&acao);
    if !pergunta.ends_with('?') || !casa_com_acao {
        falhas.push(Falha {
            portao: Portao::TipoIII,
            detalhe: "pergunta_do_decisor precisa terminar em ? e citar a ação do pacote"
                .to_string(),
        });
    }

    falhas
}

#[cfg(test)]
mod testes {
    use super::{Falha, Portao, avaliar, relatorio};
    use crate::camada::{Camada, Ficha, TipoIncerteza};
    use crate::resolucao::Resolucao;

    fn ficha_boa() -> Ficha {
        let mut f = Ficha::nova("h1.a", Camada::Modelo, TipoIncerteza::Epistemica).completa();
        f.definicao = "crescimento logaritmico do fundo alavancado".to_string();
        f.pergunta_do_decisor = "devemos manter o fundo 3x ou reduzir para 1x?".to_string();
        f.acao_do_pacote = "reduzir para 1x".to_string();
        f
    }

    #[test]
    fn ficha_completa_passa() {
        assert!(avaliar(&ficha_boa()).is_empty());
    }

    #[test]
    fn ficha_completa_em_r1_dia_passa_phi5() {
        let f = ficha_boa();
        assert_eq!(f.resolucao, Resolucao::R1Dia);
        assert_eq!(f.unidade_independente.as_deref(), Some("dia"));
        let falhas = avaliar(&f);
        assert!(
            !falhas.iter().any(|x| x.portao == Portao::Phi5),
            "Phi5 não podia reprovar: {}",
            relatorio(&falhas)
        );
        assert!(falhas.is_empty(), "{}", relatorio(&falhas));
    }

    #[test]
    fn acao_agregada_contra_unidade_dia_reprova_phi5() {
        let f = ficha_boa().com_resolucao(Resolucao::R0Agregada);
        let falhas = avaliar(&f);
        let phi5: Vec<&Falha> = falhas.iter().filter(|x| x.portao == Portao::Phi5).collect();
        assert_eq!(phi5.len(), 1, "{}", relatorio(&falhas));
        assert!(
            phi5[0].detalhe.contains(Resolucao::R0Agregada.codigo()),
            "detalhe sem a resolução declarada: {}",
            phi5[0].detalhe
        );
        // Só Phi5 muda: o resto da ficha continua aprovado.
        assert_eq!(falhas.len(), 1, "{}", relatorio(&falhas));
    }

    #[test]
    fn codigo_do_portao_phi5_aparece_no_relatorio_de_falhas() {
        let f = ficha_boa().com_resolucao(Resolucao::R0Agregada);
        let texto = relatorio(&avaliar(&f));
        assert!(
            texto.contains(Portao::Phi5.codigo()),
            "relatório sem o código do portão: {}",
            texto
        );
        assert!(texto.contains("Phi5-resolucao-da-pergunta"), "{}", texto);
        assert!(relatorio(&[]).contains("aprovados"));
    }

    #[test]
    fn resolucao_mais_fina_que_o_dia_tambem_passa() {
        for r in [Resolucao::R2Hora, Resolucao::R3Subpergunta] {
            let f = ficha_boa().com_resolucao(r);
            assert!(
                !avaliar(&f).iter().any(|x| x.portao == Portao::Phi5),
                "{} reprovou com unidade dia",
                r.codigo()
            );
        }
    }

    #[test]
    fn unidade_de_semana_tambem_reprova_com_r0_agregada() {
        let mut f = ficha_boa().com_resolucao(Resolucao::R0Agregada);
        f.unidade_independente = Some("semana".to_string());
        assert!(avaliar(&f).iter().any(|x| x.portao == Portao::Phi5));
    }

    #[test]
    fn unidade_mais_fina_que_o_dia_fica_fora_do_phi5_declarado() {
        // Limite declarado: a checagem só morde quando o dado está no dia ou acima dele.
        let mut f = ficha_boa().com_resolucao(Resolucao::R0Agregada);
        f.unidade_independente = Some("hora".to_string());
        assert!(!avaliar(&f).iter().any(|x| x.portao == Portao::Phi5));
    }

    #[test]
    fn sem_unidade_o_phi5_nao_acusa_e_o_phi2_sim() {
        let mut f = ficha_boa().com_resolucao(Resolucao::R0Agregada);
        f.unidade_independente = None;
        let falhas = avaliar(&f);
        assert!(!falhas.iter().any(|x| x.portao == Portao::Phi5));
        assert!(falhas.iter().any(|x| x.portao == Portao::Phi2));
    }

    #[test]
    fn sem_pergunta_do_decisor_reprova_tipo_iii() {
        let mut f = ficha_boa();
        f.pergunta_do_decisor = "devemos manter o fundo 3x".to_string();
        let falhas = avaliar(&f);
        assert!(falhas.iter().any(|x| x.portao == Portao::TipoIII));
    }

    #[test]
    fn sem_semente_reprova_phi1() {
        let mut f = ficha_boa();
        f.semente = None;
        let falhas = avaliar(&f);
        assert!(falhas.iter().any(|x| x.portao == Portao::Phi1));
    }
}
