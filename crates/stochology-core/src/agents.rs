//! Adversarial panel A1-A8: each agent carries the severity of its test.

use crate::layer::Card;
use crate::rng::Rng;

/// Severity scale from 0 to 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Severity(pub u8);

/// Approval floor: every A_i has to reach this value.
pub const MINIMUM_SEVERITY: Severity = Severity(6);

/// The eight agents of the adversarial panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentA {
    A1Code,
    A2Data,
    A3Lifecycle,
    A4Inference,
    A5Governance,
    A6Fairness,
    A7Domain,
    A8Decision,
}

impl AgentA {
    pub const ALL: [AgentA; 8] = [
        AgentA::A1Code,
        AgentA::A2Data,
        AgentA::A3Lifecycle,
        AgentA::A4Inference,
        AgentA::A5Governance,
        AgentA::A6Fairness,
        AgentA::A7Domain,
        AgentA::A8Decision,
    ];

    pub fn name(self) -> &'static str {
        match self {
            AgentA::A1Code => "A1-code",
            AgentA::A2Data => "A2-data",
            AgentA::A3Lifecycle => "A3-lifecycle",
            AgentA::A4Inference => "A4-inference",
            AgentA::A5Governance => "A5-governance",
            AgentA::A6Fairness => "A6-fairness",
            AgentA::A7Domain => "A7-domain",
            AgentA::A8Decision => "A8-decision",
        }
    }

    pub fn target(self) -> &'static str {
        match self {
            AgentA::A1Code => "property tests in code",
            AgentA::A2Data => "noise injection and sentinel values",
            AgentA::A3Lifecycle => "live temporal drift",
            AgentA::A4Inference => "inferential pathologies",
            AgentA::A5Governance => "tampering of the hash trail",
            AgentA::A6Fairness => "inversion of subgroup metrics",
            AgentA::A7Domain => "prior and domain shocks",
            AgentA::A8Decision => "crisis violating the exit clause",
        }
    }

    /// The card safeguard that this agent demands.
    fn safeguard_met(self, f: &Card) -> bool {
        match self {
            AgentA::A1Code => f.property_test,
            AgentA::A2Data => f.handles_missing,
            AgentA::A3Lifecycle => f.declared_drift,
            AgentA::A4Inference => f.pathologies_covered,
            AgentA::A5Governance => f.trail_intact,
            AgentA::A6Fairness => f.subgroups_audited,
            AgentA::A7Domain => f.elicited_prior,
            AgentA::A8Decision => f.exit_clause.is_some(),
        }
    }

    /// The structural anchor of the card in this agent's band: a declared safeguard without a structural field falls in the 5..=6 edge.
    pub fn structural_anchor(self, f: &Card) -> bool {
        match self {
            AgentA::A1Code => {
                f.simulable_artifact.as_ref().is_some_and(|s| !s.is_empty()) && f.seed.is_some()
            }
            AgentA::A2Data => {
                f.independent_unit.as_ref().is_some_and(|s| !s.is_empty())
                    && f.margin.is_some_and(|m| m > 0.0 && m < 1.0)
            }
            AgentA::A3Lifecycle => f.ecological_scope && f.seed.is_some(),
            AgentA::A4Inference => {
                f.null_model.as_ref().is_some_and(|s| !s.is_empty())
                    && f.margin.is_some_and(|m| m > 0.0 && m < 1.0)
            }
            AgentA::A5Governance => {
                !f.id.is_empty() && f.simulable_artifact.as_ref().is_some_and(|s| !s.is_empty())
            }
            AgentA::A6Fairness => f
                .who_pays_for_the_error
                .as_ref()
                .is_some_and(|s| !s.is_empty()),
            AgentA::A7Domain => !f.definition.is_empty() || f.ecological_scope,
            AgentA::A8Decision => {
                f.who_pays_for_the_error
                    .as_ref()
                    .is_some_and(|s| !s.is_empty())
                    && f.exit_clause.as_ref().is_some_and(|s| !s.is_empty())
            }
        }
    }

    fn seed(self, base_seed: u64) -> u64 {
        let ordinal = self as u64 + 1;
        base_seed ^ ordinal.wrapping_mul(0x9E37_79B9_7F4A_7C15)
    }
}

/// Severity of attack A_i on the card: deterministic for the given seed.
///
/// Four regimes cover the scale 1..=10 without leaving the floor 6 unreachable:
/// - safeguard + structural anchor: 8..=10 (approved with room to spare);
/// - declared safeguard without structural anchor: 5..=6 (the exact edge of floor 6);
/// - structural anchor without safeguard: 3..=5 (fails at the lower edge of the floor);
/// - no safeguard and no anchor: 1..=3 (fails).
pub fn stress(agent: AgentA, card: &Card, seed: u64) -> Severity {
    let mut rng = Rng::new(agent.seed(seed));
    match (agent.safeguard_met(card), agent.structural_anchor(card)) {
        (true, true) => Severity((8 + rng.uniform(3)) as u8),
        (true, false) => Severity((5 + rng.uniform(2)) as u8),
        (false, true) => Severity((3 + rng.uniform(3)) as u8),
        (false, false) => Severity((1 + rng.uniform(3)) as u8),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    pub items: Vec<(AgentA, Severity)>,
    pub approved: bool,
}

impl Approval {
    pub fn severity(&self, agent: AgentA) -> Severity {
        self.items
            .iter()
            .find(|(a, _)| *a == agent)
            .map(|(_, s)| *s)
            .unwrap_or(Severity(0))
    }
}

/// Condition of docs/PLAN.md: Approved(X) = AND_{i=1..8} A_i(X) >= Severity_min.
pub fn approve(card: &Card, seed: u64) -> Approval {
    let items: Vec<(AgentA, Severity)> = AgentA::ALL
        .iter()
        .map(|a| (*a, stress(*a, card, seed)))
        .collect();
    let approved = items.iter().all(|(_, s)| *s >= MINIMUM_SEVERITY);
    Approval { items, approved }
}

#[cfg(test)]
mod tests {
    use super::{AgentA, MINIMUM_SEVERITY, approve, stress};
    use crate::layer::{Card, Layer, UncertaintyType};

    fn good_card() -> Card {
        // A8 demands the exit clause, which comes with the complete header.
        Card::new("h3.voos", Layer::Agent, UncertaintyType::Epistemic)
            .with_safeguards()
            .complete()
    }

    #[test]
    fn eight_agents_approve_a_card_with_safeguards() {
        let r = approve(&good_card(), 2026);
        assert_eq!(r.items.len(), 8);
        assert!(r.approved, "a card with the eight safeguards should pass");
        for (a, s) in &r.items {
            assert!(*s >= MINIMUM_SEVERITY, "{} failed", a.name());
        }
    }

    #[test]
    fn a_missing_safeguard_fails_the_agent_of_that_band() {
        let mut f = good_card();
        f.trail_intact = false;
        let r = approve(&f, 2026);
        assert!(!r.approved);
        assert!(r.severity(AgentA::A5Governance) < MINIMUM_SEVERITY);
        assert!(r.severity(AgentA::A1Code) >= MINIMUM_SEVERITY);
    }

    #[test]
    fn severity_reproduces_with_the_same_seed() {
        let f = good_card();
        for a in AgentA::ALL {
            assert_eq!(stress(a, &f, 7), stress(a, &f, 7));
        }
    }

    #[test]
    fn safeguard_without_structural_anchor_tests_the_5_and_6_edge_and_fails_an_empty_stamp() {
        let mut without_anchor =
            Card::new("empty.stamp", Layer::Model, UncertaintyType::Epistemic).with_safeguards();
        without_anchor.exit_clause = Some("stop".into());
        // Without .complete(), the artifact, the seed, the unit, the null model and who pays for
        // the error are missing: the agents fall in the (true, false) regime -> 5 or 6, touching
        // the exact edge of floor 6.
        let mut saw_five = false;
        let mut saw_six = false;
        for seed in 1..=32 {
            for a in [
                AgentA::A1Code,
                AgentA::A2Data,
                AgentA::A4Inference,
                AgentA::A6Fairness,
            ] {
                let s = stress(a, &without_anchor, seed);
                assert!(s.0 == 5 || s.0 == 6, "expected the edge 5..=6, got {}", s.0);
                if s.0 == 5 {
                    saw_five = true;
                }
                if s.0 == 6 {
                    saw_six = true;
                }
            }
        }
        assert!(saw_five && saw_six, "the edge 5 and 6 has to be reachable");
        let r = approve(&without_anchor, 2026);
        assert!(
            !r.approved,
            "a card with booleans set but no structural anchor cannot pass"
        );
    }
}
