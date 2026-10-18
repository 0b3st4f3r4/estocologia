//! Deliberative adversarial stress: the panel A1-A8 attacked in the order that belief learned.
//!
//! The ordinary stress runs the eight bands always in the same order. Here System 2 keeps a **belief state
//! about the bands** — fed by the cards that the panel has already rejected — and attacks first whoever belief
//! points to as the weak band, stopping at the first rejection.
//!
//! The declared null is the fixed order `A1 ... A8` (`DECLARED_ORDER`): without learning, the deliberative order
//! coincides with it. What is measured is the number of attacks until the first rejection, measured in
//! `stochology-core` itself, and not the correctness of an intuition.
//!
//! Declared and not implemented: the asymmetric cost of each attack (here an attack costs the same as any
//! other) and the choice of which care to add to the card after the rejection.

use stochology_core::{AgentA, Approval, Card, MINIMUM_SEVERITY, Severity, approve, stress};

use crate::belief::{BeliefState, panel_belief};

/// The declared order of the panel: A1 to A8, without any deliberation.
pub const DECLARED_ORDER: [AgentA; 8] = AgentA::ALL;

/// What the attack spent: how many attacks until the first rejection, and with whom it stopped.
#[derive(Debug, Clone, PartialEq)]
pub struct Attack {
    pub agent: AgentA,
    pub severity: Severity,
    pub attacks: usize,
    pub failed: bool,
    pub trail: Vec<(AgentA, Severity)>,
}

/// The belief of System 2 about the bands: which one will reject the next card.
#[derive(Debug, Clone, PartialEq)]
pub struct PanelBelief {
    belief: BeliefState,
}

impl Default for PanelBelief {
    fn default() -> Self {
        PanelBelief::new()
    }
}

impl PanelBelief {
    pub fn new() -> Self {
        PanelBelief {
            belief: panel_belief(),
        }
    }

    /// Learns from a card that has already gone through the whole panel.
    pub fn update(&mut self, approval: &Approval, weight: f64) {
        self.belief.observe_approval(approval, weight);
    }

    pub fn probability(&self, agent: AgentA) -> f64 {
        self.belief.probability(agent.name()).unwrap_or(0.0)
    }

    pub fn entropy(&self) -> f64 {
        self.belief.entropy()
    }

    /// The bands in order of attack: the most likely to reject first.
    pub fn order(&self) -> Vec<AgentA> {
        let ranking = self.belief.ranking();
        let mut order: Vec<AgentA> = Vec::with_capacity(ranking.len());
        for (name, _) in ranking {
            if let Some(agent) = AgentA::ALL.iter().find(|a| a.name() == name) {
                order.push(*agent);
            }
        }
        for agent in AgentA::ALL {
            if !order.contains(&agent) {
                order.push(agent);
            }
        }
        order
    }
}

/// Runs the panel in the given order and stops at the first rejection.
pub fn stress_in_order(card: &Card, seed: u64, order: &[AgentA]) -> Attack {
    let mut trail = Vec::new();
    for agent in order {
        let severity = stress(*agent, card, seed);
        trail.push((*agent, severity));
        if severity < MINIMUM_SEVERITY {
            return Attack {
                agent: *agent,
                severity,
                attacks: trail.len(),
                failed: true,
                trail,
            };
        }
    }
    let last = order.last().copied().unwrap_or(AgentA::A8Decision);
    let severity = trail
        .iter()
        .find(|(a, _)| *a == last)
        .map(|(_, s)| *s)
        .unwrap_or(Severity(0));
    Attack {
        agent: last,
        severity,
        attacks: trail.len(),
        failed: false,
        trail,
    }
}

/// The deliberative attack: the order comes from belief, and the result is the same as `stress_in_order`.
pub fn stress_deliberative(card: &Card, seed: u64, panel: &PanelBelief) -> Attack {
    stress_in_order(card, seed, &panel.order())
}

/// Learns from a batch of cards, as System 2 would before attacking the next one.
pub fn learn(panel: &mut PanelBelief, cards: &[Card], seed: u64, weight: f64) {
    for card in cards {
        panel.update(&approve(card, seed), weight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::belief::{CEILING, FLOOR};
    use stochology_core::{Card, Layer, UncertaintyType};

    /// The card is born with the eight cares: `com_cuidados` closes the seven boolean fields and `completa`
    /// brings the header, including the exit clause that A8 charges.
    fn complete_card(id: &str) -> Card {
        Card::new(id, Layer::Agent, UncertaintyType::Epistemic)
            .with_safeguards()
            .complete()
    }

    fn cards_without_trail() -> Vec<Card> {
        (0..4)
            .map(|i| {
                let mut f = complete_card(&format!("training.{i}"));
                f.trail_intact = false;
                f
            })
            .collect()
    }

    #[test]
    fn belief_learns_the_weak_band_and_attacks_it_first() {
        let mut panel = PanelBelief::new();
        learn(&mut panel, &cards_without_trail(), 2026, 3.0);
        assert_eq!(panel.order()[0], AgentA::A5Governance);
        let candidate = {
            let mut f = complete_card("target.1");
            f.trail_intact = false;
            f
        };
        let deliberated = stress_deliberative(&candidate, 2026, &panel);
        let declared = stress_in_order(&candidate, 2026, &DECLARED_ORDER);
        assert!(deliberated.failed && declared.failed);
        assert_eq!(deliberated.agent, AgentA::A5Governance);
        assert_eq!(
            deliberated.attacks, 1,
            "belief got it right on the first try: {deliberated:?}"
        );
        assert_eq!(
            declared.attacks, 5,
            "in the declared order, A5 is the fifth: {declared:?}"
        );
    }

    #[test]
    fn without_learning_the_deliberative_order_is_the_declared_one() {
        let panel = PanelBelief::new();
        assert_eq!(panel.order(), DECLARED_ORDER.to_vec());
    }

    #[test]
    fn a_card_with_all_eight_cares_fails_in_no_order() {
        let mut panel = PanelBelief::new();
        learn(&mut panel, &cards_without_trail(), 2026, 3.0);
        let good = complete_card("good.1");
        let deliberated = stress_deliberative(&good, 2026, &panel);
        assert!(
            !deliberated.failed,
            "the stress is not a witch hunt: {deliberated:?}"
        );
        assert_eq!(
            deliberated.attacks, 8,
            "with no rejection, the whole panel runs"
        );
        assert!(
            deliberated
                .trail
                .iter()
                .all(|(_, s)| *s >= MINIMUM_SEVERITY)
        );
    }

    #[test]
    fn the_panel_belief_respects_the_floor_and_the_ceiling() {
        let mut panel = PanelBelief::new();
        learn(&mut panel, &cards_without_trail(), 2026, 3.0);
        let mut sum = 0.0;
        for agent in AgentA::ALL {
            let p = panel.probability(agent);
            assert!(
                (FLOOR - 1e-12..=CEILING + 1e-12).contains(&p),
                "{} at {p}",
                agent.name()
            );
            sum += p;
        }
        assert!((sum - 1.0).abs() < 0.02, "sum of the probabilities: {sum}");
        assert!(panel.entropy() > 0.0);
    }

    #[test]
    fn the_attack_is_deterministic() {
        let panel = PanelBelief::new();
        let card = complete_card("determinism.1");
        assert_eq!(
            stress_deliberative(&card, 2026, &panel),
            stress_deliberative(&card, 2026, &panel)
        );
        assert_eq!(
            stress_in_order(&card, 2026, &DECLARED_ORDER),
            stress_in_order(&card, 2026, &DECLARED_ORDER)
        );
    }
}
