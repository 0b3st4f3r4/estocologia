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
        }
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
