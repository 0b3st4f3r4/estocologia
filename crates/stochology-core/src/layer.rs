//! Fundamental layers of reality and the in-memory card.

use crate::resolution::Resolution;

/// World (L1), Model (L2) and Agent (L3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Layer {
    World,
    Model,
    Agent,
}

impl Layer {
    pub fn code(self) -> &'static str {
        match self {
            Layer::World => "L1",
            Layer::Model => "L2",
            Layer::Agent => "L3",
        }
    }
}

/// Card uncertainty type: aleatory, epistemic or Knightian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UncertaintyType {
    Aleatory,
    Epistemic,
    Knightian,
}

impl UncertaintyType {
    pub fn code(self) -> &'static str {
        match self {
            UncertaintyType::Aleatory => "aleatoria",
            UncertaintyType::Epistemic => "epistemica",
            UncertaintyType::Knightian => "knightiana",
        }
    }
}

/// Structured card: the only language accepted between agents.
#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub id: String,
    pub layer: Layer,
    pub uncertainty: UncertaintyType,
    pub definition: String,
    pub simulable_artifact: Option<String>,
    pub seed: Option<u64>,
    pub independent_unit: Option<String>,
    /// Grain at which the decision maker's question is asked: R0 aggregated to R3 sub-question.
    /// Invariant Phi5 demands that the action is not at a coarser grain than the unit.
    pub resolution: Resolution,
    pub margin: Option<f64>,
    pub null_model: Option<String>,
    pub ecological_scope: bool,
    pub who_pays_for_the_error: Option<String>,
    pub exit_clause: Option<String>,
    pub decision_maker_question: String,
    pub package_action: String,
    pub property_test: bool,
    pub handles_missing: bool,
    pub declared_drift: bool,
    pub pathologies_covered: bool,
    pub trail_intact: bool,
    pub subgroups_audited: bool,
    pub elicited_prior: bool,
    /// Contract 1.3: where the variance of the output comes from, `name=share; ...`, shares closing to one.
    pub uncertainty_budget: Option<String>,
    /// Contract 1.3: the event that carries the expected cost, `name; p=probability; share=fraction`.
    pub dominant_event: Option<String>,
    /// Contract 1.4: the specifications the answer was computed under, `specifications=N; agree=A; shares=...`.
    pub multiverse: Option<String>,
    /// Contract 1.4: what average success hides, for a card of an agent, `k=K; pass1=P1; passk=PK`.
    pub reliability: Option<String>,
    /// Contract 1.5: who wrote each field, `field:role, ...`, each field in its own role.
    pub authorship: Option<String>,
    /// Contract 1.5: the findings logged, used and not used, `logged=N; used=U; unused=finding: reason | ...`.
    pub findings: Option<String>,
    /// Contract 1.5: the inequality the action rests on, `p=P; C=C; L=L; act=protect|do not protect`.
    pub decision_rule: Option<String>,
    /// Contract 1.6: Meng's selection correlation by estimand, `estimando=NAME; rho=R; f=F; sigma=S; erro=E; n_efetivo=N | ...`.
    pub selection_correlation: Option<String>,
    /// Contract 1.6: the ECE cut above the null's 95th percentile, `corte=C; q95_nulo=Q; n=N; previsoes=reais|simuladas`.
    pub ece_cut: Option<String>,
    /// Contract 1.6: the mode of the declared drift, `modo=covariavel|rotulo|conceito|publico[; deteccao=...]`.
    pub drift_mode: Option<String>,
    /// Contract 1.6: the missing-data mechanism, `mecanismo=MCAR|MAR|MNAR[; sensibilidade=...]`.
    pub missing_mechanism: Option<String>,
    /// Contract 1.6: the subgroup audit, `taxa=FNR|FPR|PPV|TPR; familia=...; paga=...`.
    pub subgroup_audit: Option<String>,
    /// Contract 1.6: the causal graph, `A->B, B->C`.
    pub causal_graph: Option<String>,
}

impl Card {
    pub fn new(id: &str, layer: Layer, uncertainty: UncertaintyType) -> Self {
        Card {
            id: id.to_string(),
            layer,
            uncertainty,
            definition: String::new(),
            simulable_artifact: None,
            seed: None,
            independent_unit: None,
            resolution: Resolution::R1Day,
            margin: None,
            null_model: None,
            ecological_scope: false,
            who_pays_for_the_error: None,
            exit_clause: None,
            decision_maker_question: String::new(),
            package_action: String::new(),
            property_test: false,
            handles_missing: false,
            declared_drift: false,
            pathologies_covered: false,
            trail_intact: false,
            subgroups_audited: false,
            elicited_prior: false,
            uncertainty_budget: None,
            dominant_event: None,
            multiverse: None,
            reliability: None,
            authorship: None,
            findings: None,
            decision_rule: None,
            selection_correlation: None,
            ece_cut: None,
            drift_mode: None,
            missing_mechanism: None,
            subgroup_audit: None,
            causal_graph: None,
        }
    }

    /// Declares the uncertainty budget and the dominant event (contract 1.3).
    pub fn with_budget(mut self, budget: &str, dominant_event: &str) -> Self {
        self.uncertainty_budget = Some(budget.to_string());
        self.dominant_event = Some(dominant_event.to_string());
        self
    }

    /// Declares the multiverse of the answer (contract 1.4).
    pub fn with_multiverse(mut self, multiverse: &str) -> Self {
        self.multiverse = Some(multiverse.to_string());
        self
    }

    /// Declares the reliability of an agent over k runs (contract 1.4).
    pub fn with_reliability(mut self, reliability: &str) -> Self {
        self.reliability = Some(reliability.to_string());
        self
    }

    /// Declares who wrote each field, the findings and the decision rule (contract 1.5).
    pub fn with_conduct(mut self, authorship: &str, findings: &str, decision_rule: &str) -> Self {
        self.authorship = Some(authorship.to_string());
        self.findings = Some(findings.to_string());
        self.decision_rule = Some(decision_rule.to_string());
        self
    }

    /// Replaces the resolution of the question declared by the card (Phi5).
    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Marks the eight safeguards that the safeguard panel A1-A8 demands.
    pub fn with_safeguards(mut self) -> Self {
        self.property_test = true;
        self.handles_missing = true;
        self.declared_drift = true;
        self.pathologies_covered = true;
        self.trail_intact = true;
        self.subgroups_audited = true;
        self.elicited_prior = true;
        self
    }

    /// Fills the minimal header that Phi1-Phi4 require.
    pub fn complete(mut self) -> Self {
        self.simulable_artifact = Some(format!("crates/stochology-core#{}", self.id));
        self.seed = Some(2026);
        self.independent_unit = Some("dia".to_string());
        self.margin = Some(0.51);
        self.null_model = Some("media-historica".to_string());
        self.ecological_scope = true;
        self.who_pays_for_the_error = Some("decisor-declarado".to_string());
        self.exit_clause = Some("parar-se-ECE>0.10".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Layer, UncertaintyType};
    use crate::resolution::Resolution;

    #[test]
    fn new_brings_r1_day_as_the_default_resolution() {
        let f = Card::new("ph5.a", Layer::Model, UncertaintyType::Epistemic);
        assert_eq!(f.resolution, Resolution::R1Day);
    }

    #[test]
    fn with_resolution_swaps_the_resolution_and_complete_preserves_it() {
        let f = Card::new("ph5.b", Layer::World, UncertaintyType::Aleatory)
            .with_resolution(Resolution::R0Aggregated)
            .complete();
        assert_eq!(f.resolution, Resolution::R0Aggregated);
        assert_eq!(f.independent_unit.as_deref(), Some("dia"));
        let fine = f.clone().with_resolution(Resolution::R3Subquestion);
        assert_eq!(fine.resolution, Resolution::R3Subquestion);
        assert_ne!(fine.resolution, f.resolution);
    }
}
