//! stochology-core — the card contract in a file, with version and migration.
//!
//! Until 2026-10-16 the card was a contract in code only (`stochology-core/src/layer.rs`): the
//! fields that Phi1-Phi5 charge lived in the struct, with no versioned file and no migration path.
//! This crate provides the canonical format (one line per field, `field = value`, fixed order, no
//! external dependency) and the chain of migrations by version.
//!
//! Declared and not implemented: signature of the migrations in the audit vault
//! (`stochology-core`) and reading of third-party YAML — the format is our own, and the exchange
//! between versions is explicit.

use crate::{Card, Layer, Resolution, UncertaintyType};
use std::collections::BTreeMap;

/// Version of the contract, `major.minor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl Version {
    pub const fn new(major: u16, minor: u16) -> Self {
        Version { major, minor }
    }

    pub fn text(self) -> String {
        format!("{}.{}", self.major, self.minor)
    }

    pub fn from_text(text: &str) -> Result<Version, SchemaError> {
        let (a, b) = text
            .trim()
            .split_once('.')
            .ok_or_else(|| SchemaError::InvalidVersion(text.to_string()))?;
        let major = a
            .trim()
            .parse()
            .map_err(|_| SchemaError::InvalidVersion(text.to_string()))?;
        let minor = b
            .trim()
            .parse()
            .map_err(|_| SchemaError::InvalidVersion(text.to_string()))?;
        Ok(Version::new(major, minor))
    }
}

/// Version that this crate emits.
pub const SCHEMA_VERSION: Version = Version::new(1, 6);

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaError {
    InvalidVersion(String),
    InvalidValue { field: String, value: String },
    LineWithoutSeparator(String),
    MissingVersion,
    MissingMigration { from: Version, to: Version },
    DiscontinuousChain { from: Version, to: Version },
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaError::InvalidVersion(t) => write!(f, "invalid version: {t}"),
            SchemaError::InvalidValue { field, value } => {
                write!(f, "invalid value in {field}: {value}")
            }
            SchemaError::LineWithoutSeparator(l) => write!(f, "line without ' = ': {l}"),
            SchemaError::MissingVersion => write!(f, "document without the line 'versao = M.m'"),
            SchemaError::MissingMigration { from, to } => {
                write!(f, "no migration from {} to {}", from.text(), to.text())
            }
            SchemaError::DiscontinuousChain { from, to } => {
                write!(
                    f,
                    "discontinuous chain between {} and {}",
                    from.text(),
                    to.text()
                )
            }
        }
    }
}

impl std::error::Error for SchemaError {}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unescape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            output.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some(other) => output.push(other),
            None => output.push('\\'),
        }
    }
    output
}

fn optional(value: &Option<String>) -> Option<String> {
    value.clone()
}

/// The fields of the card as a document: canonical name mapped to value in text.
pub fn document(card: &Card) -> BTreeMap<String, String> {
    let mut doc = BTreeMap::new();
    doc.insert("id".into(), card.id.clone());
    doc.insert("camada".into(), card.layer.code().into());
    doc.insert("incerteza".into(), card.uncertainty.code().into());
    doc.insert("definicao".into(), card.definition.clone());
    doc.insert("resolucao".into(), card.resolution.code().into());
    doc.insert(
        "pergunta_do_decisor".into(),
        card.decision_maker_question.clone(),
    );
    doc.insert("acao_do_pacote".into(), card.package_action.clone());
    for (name, value) in [
        ("artefato_simulavel", optional(&card.simulable_artifact)),
        ("unidade_independente", optional(&card.independent_unit)),
        ("modelo_nulo", optional(&card.null_model)),
        (
            "quem_paga_pelo_erro",
            optional(&card.who_pays_for_the_error),
        ),
        ("clausula_de_saida", optional(&card.exit_clause)),
        ("orcamento_de_incerteza", optional(&card.uncertainty_budget)),
        ("evento_dominante", optional(&card.dominant_event)),
        ("multiverso", optional(&card.multiverse)),
        ("confiabilidade", optional(&card.reliability)),
        ("autoria", optional(&card.authorship)),
        ("achados", optional(&card.findings)),
        ("regra_de_decisao", optional(&card.decision_rule)),
        (
            "correlacao_de_selecao",
            optional(&card.selection_correlation),
        ),
        ("corte_de_ece", optional(&card.ece_cut)),
        ("modo_de_deriva", optional(&card.drift_mode)),
        ("mecanismo_de_ausentes", optional(&card.missing_mechanism)),
        ("auditoria_de_subgrupos", optional(&card.subgroup_audit)),
        ("grafo_causal", optional(&card.causal_graph)),
    ] {
        if let Some(v) = value {
            doc.insert(name.into(), v);
        }
    }
    if let Some(s) = card.seed {
        doc.insert("semente".into(), s.to_string());
    }
    if let Some(m) = card.margin {
        doc.insert("margem".into(), m.to_string());
    }
    for (name, value) in [
        ("escopo_ecologico", card.ecological_scope),
        ("teste_de_propriedade", card.property_test),
        ("trata_ausentes", card.handles_missing),
        ("deriva_declarada", card.declared_drift),
        ("patologias_cobertas", card.pathologies_covered),
        ("trilha_integra", card.trail_intact),
        ("subgrupos_auditados", card.subgroups_audited),
        ("priori_elicitada", card.elicited_prior),
    ] {
        doc.insert(name.into(), value.to_string());
    }
    doc
}

fn require(doc: &BTreeMap<String, String>, field: &str) -> Result<String, SchemaError> {
    doc.get(field)
        .cloned()
        .ok_or_else(|| SchemaError::InvalidValue {
            field: field.into(),
            value: "(missing)".into(),
        })
}

pub fn card_from_document(doc: &BTreeMap<String, String>) -> Result<Card, SchemaError> {
    let boolean = |field: &str| -> Result<bool, SchemaError> {
        let raw = doc.get(field).cloned().unwrap_or_else(|| "false".into());
        raw.parse().map_err(|_| SchemaError::InvalidValue {
            field: field.into(),
            value: raw,
        })
    };
    let layer = match require(doc, "camada")?.as_str() {
        "L1" => Layer::World,
        "L2" => Layer::Model,
        "L3" => Layer::Agent,
        other => {
            return Err(SchemaError::InvalidValue {
                field: "camada".into(),
                value: other.into(),
            });
        }
    };
    let uncertainty = match require(doc, "incerteza")?.as_str() {
        "aleatoria" => UncertaintyType::Aleatory,
        "epistemica" => UncertaintyType::Epistemic,
        "knightiana" => UncertaintyType::Knightian,
        other => {
            return Err(SchemaError::InvalidValue {
                field: "incerteza".into(),
                value: other.into(),
            });
        }
    };
    let resolution = match require(doc, "resolucao")?.as_str() {
        "R0-agregada" => Resolution::R0Aggregated,
        "R1-dia" => Resolution::R1Day,
        "R2-hora" => Resolution::R2Hour,
        "R3-subpergunta" => Resolution::R3Subquestion,
        other => {
            return Err(SchemaError::InvalidValue {
                field: "resolucao".into(),
                value: other.into(),
            });
        }
    };
    let seed = match doc.get("semente") {
        Some(t) => Some(t.parse().map_err(|_| SchemaError::InvalidValue {
            field: "semente".into(),
            value: t.clone(),
        })?),
        None => None,
    };
    let margin = match doc.get("margem") {
        Some(t) => Some(t.parse().map_err(|_| SchemaError::InvalidValue {
            field: "margem".into(),
            value: t.clone(),
        })?),
        None => None,
    };
    let uncertainty_budget = doc.get("orcamento_de_incerteza").cloned();
    if let Some(b) = &uncertainty_budget {
        crate::budget::parse_budget(b).map_err(|e| SchemaError::InvalidValue {
            field: "orcamento_de_incerteza".into(),
            value: format!("{b} ({e})"),
        })?;
    }
    let dominant_event = doc.get("evento_dominante").cloned();
    if let Some(d) = &dominant_event {
        crate::budget::parse_dominant_event(d).map_err(|e| SchemaError::InvalidValue {
            field: "evento_dominante".into(),
            value: format!("{d} ({e})"),
        })?;
    }
    let multiverse = doc.get("multiverso").cloned();
    if let Some(m) = &multiverse {
        crate::budget::parse_multiverse(m).map_err(|e| SchemaError::InvalidValue {
            field: "multiverso".into(),
            value: format!("{m} ({e})"),
        })?;
    }
    if let Some(b) = &uncertainty_budget
        && crate::budget::names_specification(b)
        && multiverse.is_none()
    {
        return Err(SchemaError::InvalidValue {
            field: "orcamento_de_incerteza".into(),
            value: format!(
                "{b} (a share for the specification is measured by a multiverse, and none is declared)"
            ),
        });
    }
    let reliability = doc.get("confiabilidade").cloned();
    if let Some(r) = &reliability {
        crate::budget::parse_reliability(r).map_err(|e| SchemaError::InvalidValue {
            field: "confiabilidade".into(),
            value: format!("{r} ({e})"),
        })?;
    }
    let checked = |field: &str,
                   parse: fn(&str) -> Result<(), String>|
     -> Result<Option<String>, SchemaError> {
        match doc.get(field) {
            Some(v) => {
                parse(v).map_err(|e| SchemaError::InvalidValue {
                    field: field.into(),
                    value: format!("{v} ({e})"),
                })?;
                Ok(Some(v.clone()))
            }
            None => Ok(None),
        }
    };
    let authorship = checked("autoria", |v| {
        crate::conduct::parse_authorship(v).map(|_| ())
    })?;
    let findings = checked("achados", |v| crate::conduct::parse_findings(v).map(|_| ()))?;
    let decision_rule = checked("regra_de_decisao", |v| {
        crate::conduct::parse_decision_rule(v).map(|_| ())
    })?;
    let selection_correlation = checked("correlacao_de_selecao", |v| {
        crate::contract16::parse_selection(v).map(|_| ())
    })?;
    let ece_cut = checked("corte_de_ece", |v| {
        crate::contract16::parse_ece_cut(v).map(|_| ())
    })?;
    let drift_mode = checked("modo_de_deriva", |v| {
        crate::contract16::parse_drift_mode(v).map(|_| ())
    })?;
    let missing_mechanism = checked("mecanismo_de_ausentes", |v| {
        crate::contract16::parse_missing_mechanism(v).map(|_| ())
    })?;
    let subgroup_audit = checked("auditoria_de_subgrupos", |v| {
        crate::contract16::parse_subgroup_audit(v).map(|_| ())
    })?;
    let causal_graph = checked("grafo_causal", |v| {
        crate::contract16::parse_causal_graph(v).map(|_| ())
    })?;
    let mut card = Card::new(&require(doc, "id")?, layer, uncertainty);
    card.selection_correlation = selection_correlation;
    card.ece_cut = ece_cut;
    card.drift_mode = drift_mode;
    card.missing_mechanism = missing_mechanism;
    card.subgroup_audit = subgroup_audit;
    card.causal_graph = causal_graph;
    card.authorship = authorship;
    card.findings = findings;
    card.decision_rule = decision_rule;
    card.uncertainty_budget = uncertainty_budget;
    card.dominant_event = dominant_event;
    card.multiverse = multiverse;
    card.reliability = reliability;
    card.definition = doc.get("definicao").cloned().unwrap_or_default();
    card.decision_maker_question = doc.get("pergunta_do_decisor").cloned().unwrap_or_default();
    card.package_action = doc.get("acao_do_pacote").cloned().unwrap_or_default();
    card.simulable_artifact = doc.get("artefato_simulavel").cloned();
    card.independent_unit = doc.get("unidade_independente").cloned();
    card.null_model = doc.get("modelo_nulo").cloned();
    card.who_pays_for_the_error = doc.get("quem_paga_pelo_erro").cloned();
    card.exit_clause = doc.get("clausula_de_saida").cloned();
    card.resolution = resolution;
    card.seed = seed;
    card.margin = margin;
    card.ecological_scope = boolean("escopo_ecologico")?;
    card.property_test = boolean("teste_de_propriedade")?;
    card.handles_missing = boolean("trata_ausentes")?;
    card.declared_drift = boolean("deriva_declarada")?;
    card.pathologies_covered = boolean("patologias_cobertas")?;
    card.trail_intact = boolean("trilha_integra")?;
    card.subgroups_audited = boolean("subgrupos_auditados")?;
    card.elicited_prior = boolean("priori_elicitada")?;
    Ok(card)
}

/// Canonical text: the version line and, below it, one line per field in alphabetical order.
pub fn text(card: &Card) -> String {
    let mut output = format!("versao = {}\n", SCHEMA_VERSION.text());
    for (field, value) in document(card) {
        output.push_str(&format!("{field} = {}\n", escape(&value)));
    }
    output
}

pub fn from_text(input: &str) -> Result<Card, SchemaError> {
    let mut doc = BTreeMap::new();
    let mut version = None;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // the separator is the first '=': the value may be empty and may contain the sign itself
        let (field, value) = line
            .split_once('=')
            .ok_or_else(|| SchemaError::LineWithoutSeparator(line.to_string()))?;
        let field = field.trim();
        if field == "versao" {
            version = Some(Version::from_text(value)?);
            continue;
        }
        doc.insert(field.to_string(), unescape(value.trim()));
    }
    let version = version.ok_or(SchemaError::MissingVersion)?;
    if version != SCHEMA_VERSION {
        let mut migrated = doc.clone();
        DECLARED_CHAIN.migrate(&mut migrated, version, SCHEMA_VERSION)?;
        return card_from_document(&migrated);
    }
    card_from_document(&doc)
}

/// One migration step of the contract.
#[derive(Debug, Clone)]
pub struct Migration {
    pub from: Version,
    pub to: Version,
    pub renames: Vec<(&'static str, &'static str)>,
    pub drops: Vec<&'static str>,
    pub adds: Vec<(&'static str, &'static str)>,
    /// Optional fields that the step brings into the contract without a value: absent until a card declares them.
    pub introduces: Vec<&'static str>,
    pub note: &'static str,
}

impl Migration {
    pub fn simple(from: Version, to: Version, note: &'static str) -> Self {
        Migration {
            from,
            to,
            renames: Vec::new(),
            drops: Vec::new(),
            adds: Vec::new(),
            introduces: Vec::new(),
            note,
        }
    }

    pub fn with_rename(mut self, from: &'static str, to: &'static str) -> Self {
        self.renames.push((from, to));
        self
    }

    pub fn with_drop(mut self, field: &'static str) -> Self {
        self.drops.push(field);
        self
    }

    pub fn with_addition(mut self, field: &'static str, value: &'static str) -> Self {
        self.adds.push((field, value));
        self
    }

    /// Applies the step to the document, in place.
    pub fn apply(&self, doc: &mut BTreeMap<String, String>) -> Result<(), SchemaError> {
        for (from, to) in &self.renames {
            if let Some(v) = doc.remove(*from) {
                doc.insert((*to).to_string(), v);
            }
        }
        for field in &self.drops {
            doc.remove(*field);
        }
        for (field, value) in &self.adds {
            doc.insert((*field).to_string(), (*value).to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Chain {
    migrations: Vec<Migration>,
}

impl Chain {
    /// The chain demands a continuous link: each step starts where the previous one ended.
    pub fn new(mut migrations: Vec<Migration>) -> Result<Self, SchemaError> {
        for (i, m) in migrations.iter().enumerate() {
            if m.from >= m.to {
                return Err(SchemaError::DiscontinuousChain {
                    from: m.from,
                    to: m.to,
                });
            }
            if migrations[..i].iter().any(|other| other.from == m.from) {
                return Err(SchemaError::DiscontinuousChain {
                    from: m.from,
                    to: m.to,
                });
            }
        }
        migrations.sort_by_key(|m| (m.from, m.to));
        for window in migrations.windows(2) {
            if window[0].to != window[1].from {
                return Err(SchemaError::DiscontinuousChain {
                    from: window[0].to,
                    to: window[1].from,
                });
            }
        }
        Ok(Chain { migrations })
    }

    pub fn versions(&self) -> Vec<Version> {
        self.migrations
            .iter()
            .flat_map(|m| [m.from, m.to])
            .fold(Vec::new(), |mut acc, v| {
                if !acc.contains(&v) {
                    acc.push(v);
                }
                acc
            })
    }

    /// One line per step, for the report: what the step does and why.
    pub fn description(&self) -> Vec<String> {
        self.migrations
            .iter()
            .map(|m| {
                let mut ops = Vec::new();
                for (from, to) in &m.renames {
                    ops.push(format!("renames {from} -> {to}"));
                }
                for field in &m.drops {
                    ops.push(format!("drops {field}"));
                }
                for (field, value) in &m.adds {
                    ops.push(format!("adds {field} = {value}"));
                }
                for field in &m.introduces {
                    ops.push(format!(
                        "introduces {field} (optional, absent until declared)"
                    ));
                }
                format!(
                    "from {} to {}: {} | {}",
                    m.from.text(),
                    m.to.text(),
                    ops.join("; "),
                    m.note
                )
            })
            .collect()
    }

    /// Takes the document from version `from` up to `to`, step by step.
    pub fn migrate(
        &self,
        doc: &mut BTreeMap<String, String>,
        from: Version,
        to: Version,
    ) -> Result<(), SchemaError> {
        if from == to {
            return Ok(());
        }
        let mut current = from;
        let mut steps = 0usize;
        while current != to {
            let step = self
                .migrations
                .iter()
                .find(|m| m.from == current)
                .ok_or(SchemaError::MissingMigration { from: current, to })?;
            step.apply(doc)?;
            current = step.to;
            steps += 1;
            if steps > self.migrations.len() {
                return Err(SchemaError::MissingMigration { from: current, to });
            }
        }
        Ok(())
    }
}

/// Reads the chain declared in a file: lines `de<TAB>para<TAB>operacao<TAB>campo<TAB>valor<TAB>nota`,
/// with `#` for comments. Several lines with the same pair (de, para) form a single step.
pub fn chain_from_text(text: &str) -> Result<Chain, SchemaError> {
    let mut steps: Vec<Migration> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let l = line.trim_end();
        if l.trim().is_empty() || l.trim_start().starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() < 6 {
            return Err(SchemaError::InvalidValue {
                field: "migracoes.tsv".into(),
                value: format!("line {} with {} fields", i + 1, fields.len()),
            });
        }
        let from = Version::from_text(fields[0].trim())?;
        let to = Version::from_text(fields[1].trim())?;
        let operation = fields[2].trim();
        let field = fields[3].trim();
        let value = fields[4].trim();
        let note: &'static str = Box::leak(fields[5].trim().to_string().into_boxed_str());
        let position = match steps.iter().position(|m| m.from == from && m.to == to) {
            Some(p) => {
                if !steps[p].note.contains(note) {
                    let joined = format!("{} / {}", steps[p].note, note);
                    steps[p].note = Box::leak(joined.into_boxed_str());
                }
                p
            }
            None => {
                steps.push(Migration::simple(from, to, note));
                steps.len() - 1
            }
        };
        match operation {
            "acrescenta" => {
                let field: &'static str = Box::leak(field.to_string().into_boxed_str());
                let value: &'static str = Box::leak(value.to_string().into_boxed_str());
                steps[position].adds.push((field, value));
            }
            "renomeia" => {
                let from_field: &'static str = Box::leak(field.to_string().into_boxed_str());
                let to_field: &'static str = Box::leak(value.to_string().into_boxed_str());
                steps[position].renames.push((from_field, to_field));
            }
            "introduz" => {
                let field: &'static str = Box::leak(field.to_string().into_boxed_str());
                steps[position].introduces.push(field);
            }
            "descarta" => {
                let field: &'static str = Box::leak(field.to_string().into_boxed_str());
                steps[position].drops.push(field);
            }
            other => {
                return Err(SchemaError::InvalidValue {
                    field: "operacao".into(),
                    value: other.to_string(),
                });
            }
        }
    }
    Chain::new(steps)
}

/// The chain declared in `migracoes.tsv`, embedded in the binary at compilation: the contract is
/// the file.
pub static DECLARED_CHAIN: std::sync::LazyLock<Chain> = std::sync::LazyLock::new(|| {
    chain_from_text(include_str!("../migracoes.tsv")).expect("the declared chain loads")
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Card;

    fn full_card() -> Card {
        let mut f = Card::new("ph.schema.1", Layer::Model, UncertaintyType::Epistemic)
            .with_resolution(Resolution::R3Subquestion)
            .complete()
            .with_safeguards();
        f.definition = "definition line".into();
        f.decision_maker_question = "on average?".into();
        f.package_action = "scale".into();
        f
    }

    #[test]
    fn text_and_from_text_preserve_the_card() {
        let f = full_card();
        let round_trip = from_text(&text(&f)).expect("round trip");
        assert_eq!(f, round_trip);
    }

    #[test]
    fn the_budget_and_the_dominant_event_survive_the_round_trip() {
        let f = full_card().with_budget(
            "arrivals=0.42; pace=0.58; covariance=0.00",
            "a day that never clears; p=0.00004; share=0.30",
        );
        assert_eq!(from_text(&text(&f)).expect("round trip"), f);
    }

    #[test]
    fn a_budget_that_does_not_close_is_refused_when_read() {
        let f = full_card().with_budget("arrivals=0.42", "rain; p=0.1; share=0.5");
        match from_text(&text(&f)) {
            Err(SchemaError::InvalidValue { field, .. }) => {
                assert_eq!(field, "orcamento_de_incerteza")
            }
            other => panic!("expected the budget to be refused, got {other:?}"),
        }
    }

    #[test]
    fn a_version_1_2_document_migrates_with_the_new_fields_absent() {
        let current = text(&full_card());
        let old = current.replacen("versao = 1.6", "versao = 1.2", 1);
        let migrated = from_text(&old).expect("migrates");
        assert_eq!(migrated.uncertainty_budget, None);
        assert_eq!(migrated.dominant_event, None);
        assert_eq!(migrated, full_card());
        assert!(
            DECLARED_CHAIN
                .description()
                .iter()
                .any(|d| d.contains("introduces orcamento_de_incerteza"))
        );
    }

    #[test]
    fn the_multiverse_and_the_reliability_survive_the_round_trip_and_are_checked() {
        let f = full_card()
            .with_budget("inputs=0.40; specification=0.60", "rain; p=0.1; share=0.5")
            .with_multiverse("specifications=18; agree=10; shares=service-cv:0.93, rate-law:0.00, correlation:0.06, interaction:0.02")
            .with_reliability("k=8; pass1=0.5312; passk=0.0700");
        assert_eq!(from_text(&text(&f)).expect("round trip"), f);
        let mut no_multiverse = f.clone();
        no_multiverse.multiverse = None;
        assert!(matches!(
            from_text(&text(&no_multiverse)),
            Err(SchemaError::InvalidValue { field, .. }) if field == "orcamento_de_incerteza"
        ));
        let promising = full_card().with_reliability("k=8; pass1=0.5; passk=0.6");
        assert!(matches!(
            from_text(&text(&promising)),
            Err(SchemaError::InvalidValue { field, .. }) if field == "confiabilidade"
        ));
    }

    #[test]
    fn a_version_1_3_document_migrates_with_the_fields_of_1_4_absent() {
        let current = text(&full_card());
        let old = current.replacen("versao = 1.6", "versao = 1.3", 1);
        let migrated = from_text(&old).expect("migrates");
        assert_eq!(
            (migrated.multiverse.clone(), migrated.reliability.clone()),
            (None, None)
        );
        assert_eq!(migrated, full_card());
    }

    #[test]
    fn the_conduct_fields_survive_the_round_trip_and_each_refusal_names_its_field() {
        let f = full_card().with_conduct(
            "modelo_nulo:inference, clausula_de_saida:decision",
            "logged=2; used=1; unused=a break in 2019: outside the window",
            "p=0.05; C=1800; L=34000; act=do not protect",
        );
        assert_eq!(from_text(&text(&f)).expect("round trip"), f);
        let refused = |card: Card, expected: &str| match from_text(&text(&card)) {
            Err(SchemaError::InvalidValue { field, .. }) => assert_eq!(field, expected),
            other => panic!("expected {expected} to be refused, got {other:?}"),
        };
        let mut outside = f.clone();
        outside.authorship = Some("clausula_de_saida:inference".into());
        refused(outside, "autoria");
        let mut withheld = f.clone();
        withheld.findings =
            Some("logged=3; used=1; unused=a break in 2019: outside the window".into());
        refused(withheld, "achados");
        let mut departs = f.clone();
        departs.decision_rule = Some("p=0.05; C=1800; L=34000; act=protect".into());
        refused(departs, "regra_de_decisao");
    }

    #[test]
    fn a_version_1_4_document_migrates_with_the_fields_of_1_5_absent() {
        let old = text(&full_card()).replacen("versao = 1.6", "versao = 1.4", 1);
        let migrated = from_text(&old).expect("migrates");
        assert_eq!(migrated, full_card());
        assert_eq!(migrated.decision_rule, None);
    }

    #[test]
    fn a_version_1_5_document_migrates_with_the_fields_of_1_6_absent() {
        let old = text(&full_card()).replacen("versao = 1.6", "versao = 1.5", 1);
        let migrated = from_text(&old).expect("migrates");
        assert_eq!(migrated, full_card());
        assert_eq!(migrated.selection_correlation, None);
        assert_eq!(migrated.causal_graph, None);
        assert!(
            DECLARED_CHAIN
                .description()
                .iter()
                .any(|d| d.contains("introduces grafo_causal"))
        );
    }

    #[test]
    fn the_fields_of_1_6_round_trip_and_are_refused_when_they_depart() {
        let mut f = full_card();
        f.selection_correlation = Some(
            "estimando=media; rho=-0.21; f=0.24; sigma=0.488; erro=-0.1824; n_efetivo=7.16".into(),
        );
        f.ece_cut = Some("corte=0.0425; q95_nulo=0.0425; n=1488; previsoes=simuladas".into());
        f.drift_mode = Some("modo=rotulo; deteccao=BBSE".into());
        f.missing_mechanism = Some("mecanismo=MNAR; sensibilidade=delta de -0.2 a 0.2".into());
        f.subgroup_audit =
            Some("taxa=FNR; familia=sexo x faixa etaria; paga=o paciente nao triado".into());
        f.causal_graph = Some("U->X, U->Y, X->Y".into());
        assert_eq!(from_text(&text(&f)).expect("round trip"), f);
        for (field, bad) in [
            (
                "correlacao_de_selecao",
                "estimando=media; rho=-0.21; f=0.24; sigma=0.488; erro=0.1; n_efetivo=7.16",
            ),
            (
                "corte_de_ece",
                "corte=0.03; q95_nulo=0.0425; n=1488; previsoes=reais",
            ),
            ("modo_de_deriva", "modo=rotulo"),
            ("mecanismo_de_ausentes", "mecanismo=MNAR"),
            ("auditoria_de_subgrupos", "taxa=FNR; familia=sexo"),
            ("grafo_causal", "A->B, B->A"),
        ] {
            let mut g = f.clone();
            match field {
                "correlacao_de_selecao" => g.selection_correlation = Some(bad.into()),
                "corte_de_ece" => g.ece_cut = Some(bad.into()),
                "modo_de_deriva" => g.drift_mode = Some(bad.into()),
                "mecanismo_de_ausentes" => g.missing_mechanism = Some(bad.into()),
                "auditoria_de_subgrupos" => g.subgroup_audit = Some(bad.into()),
                _ => g.causal_graph = Some(bad.into()),
            }
            match from_text(&text(&g)) {
                Err(SchemaError::InvalidValue { field: got, .. }) => assert_eq!(got, field),
                other => panic!("{field} not refused: {other:?}"),
            }
        }
    }

    #[test]
    fn a_missing_field_becomes_none() {
        let mut f = Card::new("ph.schema.2", Layer::World, UncertaintyType::Aleatory);
        f.simulable_artifact = None;
        f.seed = None;
        let doc = document(&f);
        assert!(!doc.contains_key("artefato_simulavel"));
        let round_trip = from_text(&text(&f)).expect("round trip");
        assert_eq!(round_trip.simulable_artifact, None);
        assert_eq!(round_trip.seed, None);
    }

    #[test]
    fn a_line_break_in_the_definition_survives() {
        let mut f = full_card();
        f.definition = "first line\nsecond line".into();
        let text = text(&f);
        assert_eq!(text.lines().count(), document(&f).len() + 1);
        assert_eq!(
            from_text(&text).expect("round trip").definition,
            f.definition
        );
    }

    #[test]
    fn an_invalid_value_fails() {
        let text = "versao = 1.0\nid = x\ncamada = L9\nincerteza = aleatoria\nresolucao = R1-dia\n";
        match from_text(text) {
            Err(SchemaError::InvalidValue { field, .. }) => assert_eq!(field, "camada"),
            other => panic!("expected an invalid value, got {other:?}"),
        }
    }

    #[test]
    fn a_document_without_a_version_fails() {
        let text = "id = x\ncamada = L1\nincerteza = aleatoria\nresolucao = R0-agregada\n";
        assert_eq!(from_text(text), Err(SchemaError::MissingVersion));
    }

    #[test]
    fn the_chain_applies_rename_drop_and_addition() {
        let chain = Chain::new(vec![
            Migration::simple(
                Version::new(1, 0),
                Version::new(1, 1),
                "margem becomes margem_da_unidade",
            )
            .with_rename("margem", "margem_da_unidade"),
            Migration::simple(Version::new(1, 1), Version::new(2, 0), "scope becomes text")
                .with_drop("escopo_ecologico")
                .with_addition("escopo", "declared"),
        ])
        .expect("continuous chain");
        assert_eq!(chain.versions().len(), 3);
        let mut doc = document(&full_card());
        chain
            .migrate(&mut doc, Version::new(1, 0), Version::new(2, 0))
            .expect("migrates");
        assert!(doc.contains_key("margem_da_unidade"));
        assert!(!doc.contains_key("margem"));
        assert!(!doc.contains_key("escopo_ecologico"));
        assert_eq!(doc.get("escopo").map(String::as_str), Some("declared"));
    }

    #[test]
    fn a_chain_with_a_hole_fails() {
        let chain = Chain::new(vec![
            Migration::simple(Version::new(1, 0), Version::new(1, 1), "first link"),
            Migration::simple(Version::new(1, 2), Version::new(2, 0), "loose link"),
        ]);
        match chain {
            Err(SchemaError::DiscontinuousChain { from, to }) => {
                assert_eq!(from, Version::new(1, 1));
                assert_eq!(to, Version::new(1, 2));
            }
            other => panic!("expected a discontinuous chain, got {other:?}"),
        }
    }

    #[test]
    fn a_missing_step_fails_with_the_version_of_the_hole() {
        let chain = Chain::new(vec![Migration::simple(
            Version::new(1, 0),
            Version::new(1, 1),
            "first link",
        )])
        .expect("continuous chain");
        let mut doc = document(&full_card());
        match chain.migrate(&mut doc, Version::new(1, 0), Version::new(2, 0)) {
            Err(SchemaError::MissingMigration { from, to }) => {
                assert_eq!(from, Version::new(1, 1));
                assert_eq!(to, Version::new(2, 0));
            }
            other => panic!("expected a missing migration, got {other:?}"),
        }
    }

    #[test]
    fn from_text_fails_on_free_text() {
        assert!(Version::from_text("um.zero").is_err());
        assert!(Version::from_text("1").is_err());
        assert_eq!(
            Version::from_text(" 1.0 ").expect("reads"),
            Version::new(1, 0)
        );
    }

    #[test]
    fn the_chain_of_the_file_is_continuous_and_ends_at_the_card_version() {
        let versions: Vec<String> = DECLARED_CHAIN.versions().iter().map(|v| v.text()).collect();
        assert_eq!(
            versions,
            vec!["1.0", "1.1", "1.2", "1.3", "1.4", "1.5", "1.6"],
            "the chain declared in the file"
        );
        assert_eq!(
            DECLARED_CHAIN.versions().last().copied(),
            Some(SCHEMA_VERSION),
            "the chain ends at the card version"
        );
    }

    #[test]
    fn a_chain_with_a_hole_or_an_unknown_operation_fails() {
        let hole = "1.0\t1.1\tacrescenta\tresolucao\tR0-agregada\tnota\n1.2\t1.3\tacrescenta\tx\ty\tnota\n";
        assert!(chain_from_text(hole).is_err(), "hole in the chain");
        let unknown = "1.0\t1.1\tinventa\tresolucao\tR0-agregada\tnota\n";
        assert!(chain_from_text(unknown).is_err(), "unknown operation");
        let backwards = "1.1\t1.0\tacrescenta\tx\ty\tnota\n";
        assert!(
            chain_from_text(backwards).is_err(),
            "step that walks backwards"
        );
        let without_value = "1.0\t1.1\tacrescenta\tresolucao\n";
        assert!(chain_from_text(without_value).is_err(), "short line");
    }

    #[test]
    fn a_document_of_version_1_0_migrates_with_the_declared_defaults() {
        let card = full_card();
        assert_eq!(
            card.resolution,
            Resolution::R3Subquestion,
            "the test card declares R3"
        );
        let current = text(&card);
        let mut old = String::from("versao = 1.0\n");
        for line in current.lines().skip(1) {
            if line.starts_with("resolucao ") || line.starts_with("priori_elicitada ") {
                continue;
            }
            old.push_str(line);
            old.push('\n');
        }
        let migrated = from_text(&old).expect("migrates from 1.0 to the card version");
        assert_eq!(
            migrated.resolution,
            Resolution::R0Aggregated,
            "the declared addition value"
        );
        assert!(!migrated.elicited_prior, "the declared addition value");
        let mut expected = card.clone();
        expected.resolution = Resolution::R0Aggregated;
        expected.elicited_prior = false;
        assert_eq!(migrated, expected, "the rest of the document stays intact");
    }

    #[test]
    fn the_chain_does_not_touch_a_document_already_at_the_card_version() {
        let card = full_card();
        let round_trip = from_text(&text(&card)).expect("no migration");
        assert_eq!(
            card, round_trip,
            "the current version does not cross the chain"
        );
    }
}
