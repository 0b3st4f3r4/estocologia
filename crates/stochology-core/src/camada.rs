//! Camadas fundamentais da realidade e a ficha YAML em memória.

use crate::resolucao::Resolucao;

/// Mundo (L1), Modelo (L2) e Agente (L3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Camada {
    Mundo,
    Modelo,
    Agente,
}

impl Camada {
    pub fn codigo(self) -> &'static str {
        match self {
            Camada::Mundo => "L1",
            Camada::Modelo => "L2",
            Camada::Agente => "L3",
        }
    }
}

/// Tipo de incerteza da ficha: aleatória, epistêmica ou knightiana.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoIncerteza {
    Aleatoria,
    Epistemica,
    Knightiana,
}

impl TipoIncerteza {
    pub fn codigo(self) -> &'static str {
        match self {
            TipoIncerteza::Aleatoria => "aleatoria",
            TipoIncerteza::Epistemica => "epistemica",
            TipoIncerteza::Knightiana => "knightiana",
        }
    }
}

/// Ficha estruturada: única linguagem aceita entre os agentes.
#[derive(Debug, Clone, PartialEq)]
pub struct Ficha {
    pub id: String,
    pub camada: Camada,
    pub incerteza: TipoIncerteza,
    pub definicao: String,
    pub artefato_simulavel: Option<String>,
    pub semente: Option<u64>,
    pub unidade_independente: Option<String>,
    /// Grão em que a pergunta do decisor é feita: R0 agregada a R3 subpergunta.
    /// O invariante Phi5 cobra que a ação não esteja num grão mais grosso que a unidade.
    pub resolucao: Resolucao,
    pub margem: Option<f64>,
    pub modelo_nulo: Option<String>,
    pub escopo_ecologico: bool,
    pub quem_paga_pelo_erro: Option<String>,
    pub clausula_de_saida: Option<String>,
    pub pergunta_do_decisor: String,
    pub acao_do_pacote: String,
    pub teste_de_propriedade: bool,
    pub trata_ausentes: bool,
    pub deriva_declarada: bool,
    pub patologias_cobertas: bool,
    pub trilha_integra: bool,
    pub subgrupos_auditados: bool,
    pub priori_elicitada: bool,
}

impl Ficha {
    pub fn nova(id: &str, camada: Camada, incerteza: TipoIncerteza) -> Self {
        Ficha {
            id: id.to_string(),
            camada,
            incerteza,
            definicao: String::new(),
            artefato_simulavel: None,
            semente: None,
            unidade_independente: None,
            resolucao: Resolucao::R1Dia,
            margem: None,
            modelo_nulo: None,
            escopo_ecologico: false,
            quem_paga_pelo_erro: None,
            clausula_de_saida: None,
            pergunta_do_decisor: String::new(),
            acao_do_pacote: String::new(),
            teste_de_propriedade: false,
            trata_ausentes: false,
            deriva_declarada: false,
            patologias_cobertas: false,
            trilha_integra: false,
            subgrupos_auditados: false,
            priori_elicitada: false,
        }
    }

    /// Troca a resolução da pergunta declarada pela ficha (Phi5).
    pub fn com_resolucao(mut self, resolucao: Resolucao) -> Self {
        self.resolucao = resolucao;
        self
    }

    /// Marca os oito cuidados que a banca adversarial A1-A8 cobra.
    pub fn com_cuidados(mut self) -> Self {
        self.teste_de_propriedade = true;
        self.trata_ausentes = true;
        self.deriva_declarada = true;
        self.patologias_cobertas = true;
        self.trilha_integra = true;
        self.subgrupos_auditados = true;
        self.priori_elicitada = true;
        self
    }

    /// Preenche o cabeçalho mínimo que Phi1-Phi4 exigem.
    pub fn completa(mut self) -> Self {
        self.artefato_simulavel = Some(format!("crates/stochology-core#{}", self.id));
        self.semente = Some(2026);
        self.unidade_independente = Some("dia".to_string());
        self.margem = Some(0.51);
        self.modelo_nulo = Some("media-historica".to_string());
        self.escopo_ecologico = true;
        self.quem_paga_pelo_erro = Some("decisor-declarado".to_string());
        self.clausula_de_saida = Some("parar-se-ECE>0.10".to_string());
        self
    }
}

#[cfg(test)]
mod testes {
    use super::{Camada, Ficha, TipoIncerteza};
    use crate::resolucao::Resolucao;

    #[test]
    fn nova_traz_r1_dia_como_resolucao_padrao() {
        let f = Ficha::nova("ph5.a", Camada::Modelo, TipoIncerteza::Epistemica);
        assert_eq!(f.resolucao, Resolucao::R1Dia);
    }

    #[test]
    fn com_resolucao_troca_a_resolucao_e_completa_preserva() {
        let f = Ficha::nova("ph5.b", Camada::Mundo, TipoIncerteza::Aleatoria)
            .com_resolucao(Resolucao::R0Agregada)
            .completa();
        assert_eq!(f.resolucao, Resolucao::R0Agregada);
        assert_eq!(f.unidade_independente.as_deref(), Some("dia"));
        let fina = f.clone().com_resolucao(Resolucao::R3Subpergunta);
        assert_eq!(fina.resolucao, Resolucao::R3Subpergunta);
        assert_ne!(fina.resolucao, f.resolucao);
    }
}
