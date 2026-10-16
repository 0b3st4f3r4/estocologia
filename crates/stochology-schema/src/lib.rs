//! stochology-schema — o contrato da ficha em arquivo, com versao e migracao.
//!
//! Ate 2026-10-16 a ficha era contrato em codigo so (`stochology-core/src/camada.rs`): os campos que
//! Phi1-Phi5 cobram viviam na struct, sem arquivo versionado e sem caminho de migracao. Este crate da
//! o formato canonico (uma linha por campo, `campo = valor`, ordem fixa, sem dependencia externa) e a
//! cadeia de migracoes por versao.
//!
//! Declarado e nao implementado: assinatura das migracoes no cofre de auditoria (`stochology-audit`) e
//! leitura de YAML de terceiros — o formato e proprio, e a troca entre versoes e explicita.

use std::collections::BTreeMap;
use stochology_core::{Camada, Ficha, Resolucao, TipoIncerteza};

/// Versao do contrato, `maior.menor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Versao {
    pub maior: u16,
    pub menor: u16,
}

impl Versao {
    pub const fn nova(maior: u16, menor: u16) -> Self {
        Versao { maior, menor }
    }

    pub fn texto(self) -> String {
        format!("{}.{}", self.maior, self.menor)
    }

    pub fn de_texto(texto: &str) -> Result<Versao, Erro> {
        let (a, b) = texto
            .trim()
            .split_once('.')
            .ok_or_else(|| Erro::VersaoInvalida(texto.to_string()))?;
        let maior = a
            .trim()
            .parse()
            .map_err(|_| Erro::VersaoInvalida(texto.to_string()))?;
        let menor = b
            .trim()
            .parse()
            .map_err(|_| Erro::VersaoInvalida(texto.to_string()))?;
        Ok(Versao::nova(maior, menor))
    }
}

/// Versao que este crate emite.
pub const VERSAO_DA_FICHA: Versao = Versao::nova(1, 0);

#[derive(Debug, Clone, PartialEq)]
pub enum Erro {
    VersaoInvalida(String),
    ValorInvalido { campo: String, valor: String },
    LinhaSemSeparador(String),
    VersaoAusente,
    MigracaoAusente { de: Versao, para: Versao },
    CadeiaDescontinua { de: Versao, para: Versao },
}

impl std::fmt::Display for Erro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Erro::VersaoInvalida(t) => write!(f, "versao invalida: {t}"),
            Erro::ValorInvalido { campo, valor } => write!(f, "valor invalido em {campo}: {valor}"),
            Erro::LinhaSemSeparador(l) => write!(f, "linha sem ' = ': {l}"),
            Erro::VersaoAusente => write!(f, "documento sem a linha 'versao = M.m'"),
            Erro::MigracaoAusente { de, para } => {
                write!(f, "sem migracao de {} para {}", de.texto(), para.texto())
            }
            Erro::CadeiaDescontinua { de, para } => {
                write!(f, "cadeia descontinua entre {} e {}", de.texto(), para.texto())
            }
        }
    }
}

impl std::error::Error for Erro {}

fn escapar(valor: &str) -> String {
    valor
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn desescapar(valor: &str) -> String {
    let mut saida = String::with_capacity(valor.len());
    let mut chars = valor.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            saida.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => saida.push('\n'),
            Some('r') => saida.push('\r'),
            Some(outro) => saida.push(outro),
            None => saida.push('\\'),
        }
    }
    saida
}

fn opcional(valor: &Option<String>) -> Option<String> {
    valor.clone()
}

/// Os campos da ficha em forma de documento: nome canonico para valor em texto.
pub fn documento(ficha: &Ficha) -> BTreeMap<String, String> {
    let mut doc = BTreeMap::new();
    doc.insert("id".into(), ficha.id.clone());
    doc.insert("camada".into(), ficha.camada.codigo().into());
    doc.insert("incerteza".into(), ficha.incerteza.codigo().into());
    doc.insert("definicao".into(), ficha.definicao.clone());
    doc.insert("resolucao".into(), ficha.resolucao.codigo().into());
    doc.insert("pergunta_do_decisor".into(), ficha.pergunta_do_decisor.clone());
    doc.insert("acao_do_pacote".into(), ficha.acao_do_pacote.clone());
    for (nome, valor) in [
        ("artefato_simulavel", opcional(&ficha.artefato_simulavel)),
        ("unidade_independente", opcional(&ficha.unidade_independente)),
        ("modelo_nulo", opcional(&ficha.modelo_nulo)),
        ("quem_paga_pelo_erro", opcional(&ficha.quem_paga_pelo_erro)),
        ("clausula_de_saida", opcional(&ficha.clausula_de_saida)),
    ] {
        if let Some(v) = valor {
            doc.insert(nome.into(), v);
        }
    }
    if let Some(s) = ficha.semente {
        doc.insert("semente".into(), s.to_string());
    }
    if let Some(m) = ficha.margem {
        doc.insert("margem".into(), m.to_string());
    }
    for (nome, valor) in [
        ("escopo_ecologico", ficha.escopo_ecologico),
        ("teste_de_propriedade", ficha.teste_de_propriedade),
        ("trata_ausentes", ficha.trata_ausentes),
        ("deriva_declarada", ficha.deriva_declarada),
        ("patologias_cobertas", ficha.patologias_cobertas),
        ("trilha_integra", ficha.trilha_integra),
        ("subgrupos_auditados", ficha.subgrupos_auditados),
        ("priori_elicitada", ficha.priori_elicitada),
    ] {
        doc.insert(nome.into(), valor.to_string());
    }
    doc
}

fn exigir(doc: &BTreeMap<String, String>, campo: &str) -> Result<String, Erro> {
    doc.get(campo)
        .cloned()
        .ok_or_else(|| Erro::ValorInvalido { campo: campo.into(), valor: "(ausente)".into() })
}

pub fn ficha_de_documento(doc: &BTreeMap<String, String>) -> Result<Ficha, Erro> {
    let booleano = |campo: &str| -> Result<bool, Erro> {
        let bruto = doc.get(campo).cloned().unwrap_or_else(|| "false".into());
        bruto
            .parse()
            .map_err(|_| Erro::ValorInvalido { campo: campo.into(), valor: bruto })
    };
    let camada = match exigir(doc, "camada")?.as_str() {
        "L1" => Camada::Mundo,
        "L2" => Camada::Modelo,
        "L3" => Camada::Agente,
        outro => {
            return Err(Erro::ValorInvalido { campo: "camada".into(), valor: outro.into() });
        }
    };
    let incerteza = match exigir(doc, "incerteza")?.as_str() {
        "aleatoria" => TipoIncerteza::Aleatoria,
        "epistemica" => TipoIncerteza::Epistemica,
        "knightiana" => TipoIncerteza::Knightiana,
        outro => {
            return Err(Erro::ValorInvalido { campo: "incerteza".into(), valor: outro.into() });
        }
    };
    let resolucao = match exigir(doc, "resolucao")?.as_str() {
        "R0-agregada" => Resolucao::R0Agregada,
        "R1-dia" => Resolucao::R1Dia,
        "R2-hora" => Resolucao::R2Hora,
        "R3-subpergunta" => Resolucao::R3Subpergunta,
        outro => {
            return Err(Erro::ValorInvalido { campo: "resolucao".into(), valor: outro.into() });
        }
    };
    let semente = match doc.get("semente") {
        Some(t) => Some(
            t.parse()
                .map_err(|_| Erro::ValorInvalido { campo: "semente".into(), valor: t.clone() })?,
        ),
        None => None,
    };
    let margem = match doc.get("margem") {
        Some(t) => Some(
            t.parse()
                .map_err(|_| Erro::ValorInvalido { campo: "margem".into(), valor: t.clone() })?,
        ),
        None => None,
    };
    let mut ficha = Ficha::nova(&exigir(doc, "id")?, camada, incerteza);
    ficha.definicao = doc.get("definicao").cloned().unwrap_or_default();
    ficha.pergunta_do_decisor = doc.get("pergunta_do_decisor").cloned().unwrap_or_default();
    ficha.acao_do_pacote = doc.get("acao_do_pacote").cloned().unwrap_or_default();
    ficha.artefato_simulavel = doc.get("artefato_simulavel").cloned();
    ficha.unidade_independente = doc.get("unidade_independente").cloned();
    ficha.modelo_nulo = doc.get("modelo_nulo").cloned();
    ficha.quem_paga_pelo_erro = doc.get("quem_paga_pelo_erro").cloned();
    ficha.clausula_de_saida = doc.get("clausula_de_saida").cloned();
    ficha.resolucao = resolucao;
    ficha.semente = semente;
    ficha.margem = margem;
    ficha.escopo_ecologico = booleano("escopo_ecologico")?;
    ficha.teste_de_propriedade = booleano("teste_de_propriedade")?;
    ficha.trata_ausentes = booleano("trata_ausentes")?;
    ficha.deriva_declarada = booleano("deriva_declarada")?;
    ficha.patologias_cobertas = booleano("patologias_cobertas")?;
    ficha.trilha_integra = booleano("trilha_integra")?;
    ficha.subgrupos_auditados = booleano("subgrupos_auditados")?;
    ficha.priori_elicitada = booleano("priori_elicitada")?;
    Ok(ficha)
}

/// Texto canonico: a linha de versao e, abaixo, uma linha por campo em ordem alfabetica.
pub fn texto(ficha: &Ficha) -> String {
    let mut saida = format!("versao = {}\n", VERSAO_DA_FICHA.texto());
    for (campo, valor) in documento(ficha) {
        saida.push_str(&format!("{campo} = {}\n", escapar(&valor)));
    }
    saida
}

pub fn de_texto(entrada: &str) -> Result<Ficha, Erro> {
    let mut doc = BTreeMap::new();
    let mut versao = None;
    for linha in entrada.lines() {
        let linha = linha.trim();
        if linha.is_empty() || linha.starts_with('#') {
            continue;
        }
        // separador e o primeiro '=': o valor pode ser vazio e pode conter o proprio sinal
        let (campo, valor) = linha
            .split_once('=')
            .ok_or_else(|| Erro::LinhaSemSeparador(linha.to_string()))?;
        let campo = campo.trim();
        if campo == "versao" {
            versao = Some(Versao::de_texto(valor)?);
            continue;
        }
        doc.insert(campo.to_string(), desescapar(valor.trim()));
    }
    let versao = versao.ok_or(Erro::VersaoAusente)?;
    if versao != VERSAO_DA_FICHA {
        let mut doc_migrado = doc.clone();
        PADRAO.de_para(&mut doc_migrado, versao, VERSAO_DA_FICHA)?;
        return ficha_de_documento(&doc_migrado);
    }
    ficha_de_documento(&doc)
}

/// Uma etapa de migracao do contrato.
#[derive(Debug, Clone)]
pub struct Migracao {
    pub de: Versao,
    pub para: Versao,
    pub renomeia: Vec<(&'static str, &'static str)>,
    pub descarta: Vec<&'static str>,
    pub acrescenta: Vec<(&'static str, &'static str)>,
    pub nota: &'static str,
}

impl Migracao {
    pub fn simples(de: Versao, para: Versao, nota: &'static str) -> Self {
        Migracao { de, para, renomeia: Vec::new(), descarta: Vec::new(), acrescenta: Vec::new(), nota }
    }

    pub fn com_renomeacao(mut self, de: &'static str, para: &'static str) -> Self {
        self.renomeia.push((de, para));
        self
    }

    pub fn com_descarte(mut self, campo: &'static str) -> Self {
        self.descarta.push(campo);
        self
    }

    pub fn com_acrescimo(mut self, campo: &'static str, valor: &'static str) -> Self {
        self.acrescenta.push((campo, valor));
        self
    }

    /// Aplica a etapa no documento, no lugar.
    pub fn aplicar(&self, doc: &mut BTreeMap<String, String>) -> Result<(), Erro> {
        for (de, para) in &self.renomeia {
            if let Some(v) = doc.remove(*de) {
                doc.insert((*para).to_string(), v);
            }
        }
        for campo in &self.descarta {
            doc.remove(*campo);
        }
        for (campo, valor) in &self.acrescenta {
            doc.insert((*campo).to_string(), (*valor).to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Cadeia {
    migracoes: Vec<Migracao>,
}

impl Cadeia {
    /// Cadeia exige elo continuo: cada etapa comeca onde a anterior terminou.
    pub fn nova(mut migracoes: Vec<Migracao>) -> Result<Self, Erro> {
        for (i, m) in migracoes.iter().enumerate() {
            if m.de >= m.para {
                return Err(Erro::CadeiaDescontinua { de: m.de, para: m.para });
            }
            if migracoes[..i].iter().any(|outra| outra.de == m.de) {
                return Err(Erro::CadeiaDescontinua { de: m.de, para: m.para });
            }
        }
        migracoes.sort_by_key(|m| (m.de, m.para));
        for janela in migracoes.windows(2) {
            if janela[0].para != janela[1].de {
                return Err(Erro::CadeiaDescontinua { de: janela[0].para, para: janela[1].de });
            }
        }
        Ok(Cadeia { migracoes })
    }

    pub fn versoes(&self) -> Vec<Versao> {
        self.migracoes
            .iter()
            .flat_map(|m| [m.de, m.para])
            .fold(Vec::new(), |mut acc, v| {
                if !acc.contains(&v) {
                    acc.push(v);
                }
                acc
            })
    }

    /// Leva o documento da versao `de` ate `para`, etapa por etapa.
    pub fn de_para(
        &self,
        doc: &mut BTreeMap<String, String>,
        de: Versao,
        para: Versao,
    ) -> Result<(), Erro> {
        if de == para {
            return Ok(());
        }
        let mut atual = de;
        let mut passos = 0usize;
        while atual != para {
            let etapa = self
                .migracoes
                .iter()
                .find(|m| m.de == atual)
                .ok_or(Erro::MigracaoAusente { de: atual, para })?;
            etapa.aplicar(doc)?;
            atual = etapa.para;
            passos += 1;
            if passos > self.migracoes.len() {
                return Err(Erro::MigracaoAusente { de: atual, para });
            }
        }
        Ok(())
    }
}

/// Cadeia viva do contrato. Vazia por ora: a ficha nasceu na 1.0 e nenhuma migracao foi declarada.
pub static PADRAO: std::sync::LazyLock<Cadeia> =
    std::sync::LazyLock::new(|| Cadeia { migracoes: Vec::new() });

#[cfg(test)]
mod testes {
    use super::*;
    use stochology_core::Ficha;

    fn ficha_cheia() -> Ficha {
        let mut f = Ficha::nova("ph.schema.1", Camada::Modelo, TipoIncerteza::Epistemica)
            .com_resolucao(Resolucao::R3Subpergunta)
            .completa()
            .com_cuidados();
        f.definicao = "linha de definicao".into();
        f.pergunta_do_decisor = "em media?".into();
        f.acao_do_pacote = "escalar".into();
        f
    }

    #[test]
    fn texto_e_de_texto_preservam_a_ficha() {
        let f = ficha_cheia();
        let volta = de_texto(&texto(&f)).expect("round trip");
        assert_eq!(f, volta);
    }

    #[test]
    fn campo_ausente_vira_none() {
        let mut f = Ficha::nova("ph.schema.2", Camada::Mundo, TipoIncerteza::Aleatoria);
        f.artefato_simulavel = None;
        f.semente = None;
        let doc = documento(&f);
        assert!(!doc.contains_key("artefato_simulavel"));
        let volta = de_texto(&texto(&f)).expect("round trip");
        assert_eq!(volta.artefato_simulavel, None);
        assert_eq!(volta.semente, None);
    }

    #[test]
    fn quebra_de_linha_na_definicao_sobrevive() {
        let mut f = ficha_cheia();
        f.definicao = "primeira linha\nsegunda linha".into();
        let texto = texto(&f);
        assert_eq!(texto.lines().count(), documento(&f).len() + 1);
        assert_eq!(de_texto(&texto).expect("round trip").definicao, f.definicao);
    }

    #[test]
    fn valor_invalido_reprova() {
        let texto = "versao = 1.0\nid = x\ncamada = L9\nincerteza = aleatoria\nresolucao = R1-dia\n";
        match de_texto(texto) {
            Err(Erro::ValorInvalido { campo, .. }) => assert_eq!(campo, "camada"),
            outro => panic!("esperava valor invalido, veio {outro:?}"),
        }
    }

    #[test]
    fn documento_sem_versao_reprova() {
        let texto = "id = x\ncamada = L1\nincerteza = aleatoria\nresolucao = R0-agregada\n";
        assert_eq!(de_texto(texto), Err(Erro::VersaoAusente));
    }

    #[test]
    fn cadeia_aplica_renomeacao_descarte_e_acrescimo() {
        let cadeia = Cadeia::nova(vec![
            Migracao::simples(Versao::nova(1, 0), Versao::nova(1, 1), "margem vira margem_da_unidade")
                .com_renomeacao("margem", "margem_da_unidade"),
            Migracao::simples(Versao::nova(1, 1), Versao::nova(2, 0), "escopo vira texto")
                .com_descarte("escopo_ecologico")
                .com_acrescimo("escopo", "declarado"),
        ])
        .expect("cadeia continua");
        assert_eq!(cadeia.versoes().len(), 3);
        let mut doc = documento(&ficha_cheia());
        cadeia
            .de_para(&mut doc, Versao::nova(1, 0), Versao::nova(2, 0))
            .expect("migra");
        assert!(doc.contains_key("margem_da_unidade"));
        assert!(!doc.contains_key("margem"));
        assert!(!doc.contains_key("escopo_ecologico"));
        assert_eq!(doc.get("escopo").map(String::as_str), Some("declarado"));
    }

    #[test]
    fn cadeia_com_buraco_reprova() {
        let cadeia = Cadeia::nova(vec![
            Migracao::simples(Versao::nova(1, 0), Versao::nova(1, 1), "primeiro elo"),
            Migracao::simples(Versao::nova(1, 2), Versao::nova(2, 0), "elo solto"),
        ]);
        match cadeia {
            Err(Erro::CadeiaDescontinua { de, para }) => {
                assert_eq!(de, Versao::nova(1, 1));
                assert_eq!(para, Versao::nova(1, 2));
            }
            outro => panic!("esperava cadeia descontinua, veio {outro:?}"),
        }
    }

    #[test]
    fn etapa_ausente_reprova_com_a_versao_do_buraco() {
        let cadeia = Cadeia::nova(vec![Migracao::simples(
            Versao::nova(1, 0),
            Versao::nova(1, 1),
            "primeiro elo",
        )])
        .expect("cadeia continua");
        let mut doc = documento(&ficha_cheia());
        match cadeia.de_para(&mut doc, Versao::nova(1, 0), Versao::nova(2, 0)) {
            Err(Erro::MigracaoAusente { de, para }) => {
                assert_eq!(de, Versao::nova(1, 1));
                assert_eq!(para, Versao::nova(2, 0));
            }
            outro => panic!("esperava migracao ausente, veio {outro:?}"),
        }
    }

    #[test]
    fn versao_de_texto_reprova_texto_livre() {
        assert!(Versao::de_texto("um.zero").is_err());
        assert!(Versao::de_texto("1").is_err());
        assert_eq!(Versao::de_texto(" 1.0 ").expect("le"), Versao::nova(1, 0));
    }
}
