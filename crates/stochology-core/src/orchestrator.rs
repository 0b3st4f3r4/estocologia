//! Master orchestrator M: drives the cycle and submits the card to the panel A1-A8.

use crate::agents::{AgentA, Approval, Check, approve, check};
use crate::layer::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrchestratorM {
    pub session_id: String,
    pub trail_hash: String,
}

impl OrchestratorM {
    pub fn new(session_id: &str, trail_hash: &str) -> Self {
        OrchestratorM {
            session_id: session_id.to_string(),
            trail_hash: trail_hash.to_string(),
        }
    }

    /// Reads the two bits of the indicated agent A_i on the card (no attack is executed here).
    pub fn conduct_attack(&self, agent: AgentA, card: &Card) -> Result<Check, String> {
        if card.id.trim().is_empty() {
            return Err("card without identifier".to_string());
        }
        if !self.trail_hash.is_empty()
            && (self.trail_hash.len() != 64
                || !self.trail_hash.chars().all(|c| c.is_ascii_hexdigit()))
        {
            return Err("trail_hash outside the SHA-256 format (64 hex)".to_string());
        }
        Ok(check(agent, card))
    }

    /// Runs the whole panel and returns the verdict with the eight bit pairs.
    pub fn panel(&self, card: &Card) -> Approval {
        approve(card)
    }
}

#[cfg(test)]
mod tests {
    use super::OrchestratorM;
    use crate::agents::AgentA;
    use crate::layer::{Card, Layer, UncertaintyType};

    #[test]
    fn panel_rejects_an_incomplete_card() {
        let m = OrchestratorM::new("session-h0", "0".repeat(64).as_str());
        let f = Card::new("h0.no-safeguards", Layer::World, UncertaintyType::Aleatory);
        assert!(!m.panel(&f).approved);
    }

    #[test]
    fn a_single_attack_runs_and_refuses_a_corrupted_trail_hash() {
        let m = OrchestratorM::new("session-h1", "1".repeat(64).as_str());
        let f = Card::new("h1.a", Layer::Model, UncertaintyType::Epistemic);
        assert!(m.conduct_attack(AgentA::A3Lifecycle, &f).is_ok());

        let corrupted = OrchestratorM::new("session-h1", "not-sha256");
        assert!(corrupted.conduct_attack(AgentA::A5Governance, &f).is_err());
    }
}
