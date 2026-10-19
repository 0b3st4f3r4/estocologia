//! Safeguard panel A1-A8: each agent returns two declared bits, not a score.
//!
//! What this module is, said plainly: the panel does NOT execute an attack. For each agent it reads
//! two bits from the card, whether the safeguard that agent demands was declared and whether the
//! structural field that anchors it is present, and the agent passes only when both are set.
//! Until 2026-10-19 the two bits were dressed as a "severity" from 0 to 10, drawn by a seed inside a
//! band that the bits selected; the decimal carried no information beyond the bits, so it was
//! removed (Appendix E, rectifications of 2026-10-19). The panel verifies that a safeguard was
//! declared and anchored, not that it works. The attacks that do run (the hash trail tampered on
//! purpose, the planted defects) live in the gates, not here. No seed enters the verdict.

use crate::layer::Card;

/// The eight agents of the safeguard panel.
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

    /// The structural anchor of the card for this agent: a declared safeguard without the field that anchors it does not pass.
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
}

/// What the panel returns for one agent: the two declared bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Check {
    /// The safeguard this agent demands was declared on the card.
    pub safeguard: bool,
    /// The structural field that anchors the safeguard is present.
    pub anchor: bool,
}

impl Check {
    /// The agent passes only when the safeguard is declared and anchored.
    pub fn passed(self) -> bool {
        self.safeguard && self.anchor
    }

    /// The two bits as printed in the reports: safeguard first, anchor second.
    pub fn bits(self) -> &'static str {
        match (self.safeguard, self.anchor) {
            (true, true) => "11",
            (true, false) => "10",
            (false, true) => "01",
            (false, false) => "00",
        }
    }
}

/// The check of agent A_i on the card. Deterministic and free of any seed.
pub fn check(agent: AgentA, card: &Card) -> Check {
    Check {
        safeguard: agent.safeguard_met(card),
        anchor: agent.structural_anchor(card),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    pub items: Vec<(AgentA, Check)>,
    pub approved: bool,
}

impl Approval {
    /// The check of one agent; an agent absent from the list counts as two unset bits.
    pub fn check(&self, agent: AgentA) -> Check {
        self.items
            .iter()
            .find(|(a, _)| *a == agent)
            .map(|(_, c)| *c)
            .unwrap_or(Check {
                safeguard: false,
                anchor: false,
            })
    }

    /// How many of the agents passed.
    pub fn passed(&self) -> usize {
        self.items.iter().filter(|(_, c)| c.passed()).count()
    }

    /// The agents that did not pass, in panel order.
    pub fn failing(&self) -> Vec<AgentA> {
        self.items
            .iter()
            .filter(|(_, c)| !c.passed())
            .map(|(a, _)| *a)
            .collect()
    }

    /// The panel as eight bit pairs, one per agent, for the reports.
    pub fn bits(&self) -> String {
        self.items
            .iter()
            .map(|(a, c)| format!("{}={}", a.name(), c.bits()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Approved(X) = AND_{i=1..8} (safeguard_i(X) AND anchor_i(X)).
pub fn approve(card: &Card) -> Approval {
    let items: Vec<(AgentA, Check)> = AgentA::ALL.iter().map(|a| (*a, check(*a, card))).collect();
    let approved = items.iter().all(|(_, c)| c.passed());
    Approval { items, approved }
}

#[cfg(test)]
mod tests {
    use super::{AgentA, approve, check};
    use crate::layer::{Card, Layer, UncertaintyType};

    fn good_card() -> Card {
        // A8 demands the exit clause, which comes with the complete header.
        Card::new("h3.voos", Layer::Agent, UncertaintyType::Epistemic)
            .with_safeguards()
            .complete()
    }

    #[test]
    fn eight_agents_approve_a_card_with_safeguards() {
        let r = approve(&good_card());
        assert_eq!(r.items.len(), 8);
        assert!(r.approved, "a card with the eight safeguards should pass");
        assert_eq!(r.passed(), 8);
        for (a, c) in &r.items {
            assert_eq!(c.bits(), "11", "{} failed", a.name());
        }
    }

    #[test]
    fn a_missing_safeguard_fails_the_agent_of_that_band() {
        let mut f = good_card();
        f.trail_intact = false;
        let r = approve(&f);
        assert!(!r.approved);
        assert_eq!(r.failing(), vec![AgentA::A5Governance]);
        assert_eq!(r.check(AgentA::A5Governance).bits(), "01");
        assert!(r.check(AgentA::A1Code).passed());
    }

    #[test]
    fn safeguard_without_structural_anchor_does_not_pass() {
        let mut without_anchor =
            Card::new("empty.stamp", Layer::Model, UncertaintyType::Epistemic).with_safeguards();
        without_anchor.exit_clause = Some("stop".into());
        // Without .complete(), the artifact, the seed, the unit, the null model and who pays for
        // the error are missing: the declaration stands without its anchor.
        for a in [
            AgentA::A1Code,
            AgentA::A2Data,
            AgentA::A4Inference,
            AgentA::A6Fairness,
        ] {
            assert_eq!(check(a, &without_anchor).bits(), "10");
        }
        assert!(
            !approve(&without_anchor).approved,
            "a card with booleans set but no structural anchor cannot pass"
        );
    }

    /// The report carries exactly the bits: two cards with the same bits give the same report,
    /// whatever else differs between them.
    #[test]
    fn the_report_is_a_function_of_the_bits_only() {
        let a = good_card();
        let mut b = good_card();
        b.id = "another.id".into();
        b.seed = Some(999_983);
        assert_eq!(approve(&a), approve(&b));
        assert_eq!(approve(&a).bits(), approve(&b).bits());
    }
}
