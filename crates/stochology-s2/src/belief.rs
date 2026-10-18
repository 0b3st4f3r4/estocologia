//! The belief-state engine: what System 2 believes, with the floor and the ceiling of the contract.

use stochology_core::{AgentA, Approval, Card, MINIMUM_SEVERITY};

/// Epistemological floor: no probability descends to 0%.
pub const FLOOR: f64 = 0.01;

/// Epistemological ceiling: no probability rises to 100%.
pub const CEILING: f64 = 0.99;

/// Belief state: weight per hypothesis, held inside [FLOOR, CEILING] on every reading.
#[derive(Debug, Clone, PartialEq)]
pub struct BeliefState {
    weights: Vec<(String, f64)>,
}

impl BeliefState {
    pub fn new(hypotheses: &[&str]) -> Self {
        BeliefState::from_names(hypotheses.iter().map(|h| (*h).to_string()).collect())
    }

    pub fn from_names(hypotheses: Vec<String>) -> Self {
        let weight = 1.0 / hypotheses.len().max(1) as f64;
        BeliefState {
            weights: hypotheses.into_iter().map(|h| (h, weight)).collect(),
        }
    }

    pub fn hypotheses(&self) -> Vec<&str> {
        self.weights.iter().map(|(h, _)| h.as_str()).collect()
    }

    fn total(&self) -> f64 {
        self.weights.iter().map(|(_, p)| *p).sum()
    }

    pub fn probability(&self, hypothesis: &str) -> Option<f64> {
        let total = self.total();
        self.weights
            .iter()
            .find(|(h, _)| h == hypothesis)
            .map(|(_, p)| {
                if total > 0.0 {
                    (p / total).clamp(FLOOR, CEILING)
                } else {
                    0.0
                }
            })
    }

    /// Bayesian update: multiplies by the weight of the evidence and re-clamps to the contract interval.
    pub fn observe(&mut self, evidence: &[(&str, f64)]) {
        for (hypothesis, likelihood) in evidence {
            if let Some((_, weight)) = self.weights.iter_mut().find(|(h, _)| h == hypothesis) {
                *weight *= likelihood.max(0.0);
            }
        }
        self.reclamp();
    }

    /// Card evidence: reinforces the hypotheses that match the layer and the type of uncertainty.
    pub fn observe_card(&mut self, card: &Card, weight: f64) {
        let targets: Vec<(String, f64)> = vec![
            (card.layer.code().to_string(), weight),
            (card.uncertainty.code().to_string(), weight),
        ];
        for (hypothesis, w) in targets {
            if let Some((_, p)) = self.weights.iter_mut().find(|(h, _)| *h == hypothesis) {
                *p *= w.max(0.0);
            }
        }
        self.reclamp();
    }

    /// Audit evidence: raises the weight of whoever rejected the card.
    pub fn observe_approval(&mut self, approval: &Approval, weight: f64) {
        let rejected: Vec<(String, f64)> = approval
            .items
            .iter()
            .filter(|(_, s)| *s < MINIMUM_SEVERITY)
            .map(|(a, _)| (a.name().to_string(), weight))
            .collect();
        for (hypothesis, w) in rejected {
            if let Some((_, p)) = self.weights.iter_mut().find(|(h, _)| *h == hypothesis) {
                *p *= w.max(0.0);
            }
        }
        self.reclamp();
    }

    fn reclamp(&mut self) {
        let total = self.total();
        if total <= 0.0 {
            let uniform = 1.0 / self.weights.len().max(1) as f64;
            for (_, weight) in self.weights.iter_mut() {
                *weight = uniform;
            }
            return;
        }
        for (_, weight) in self.weights.iter_mut() {
            *weight = (*weight / total).clamp(FLOOR, CEILING);
        }
    }

    pub fn entropy(&self) -> f64 {
        let total = self.total();
        if total <= 0.0 {
            return 0.0;
        }
        -self
            .weights
            .iter()
            .map(|(_, p)| {
                let q = (p / total).clamp(FLOOR, CEILING);
                q * q.ln()
            })
            .sum::<f64>()
    }

    pub fn most_likely(&self) -> Option<&str> {
        self.weights
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(h, _)| h.as_str())
    }

    /// The hypotheses in decreasing order of weight; a tie is broken by the declared order.
    pub fn ranking(&self) -> Vec<(String, f64)> {
        let mut items: Vec<usize> = (0..self.weights.len()).collect();
        items.sort_by(|i, j| {
            self.weights[*j]
                .1
                .partial_cmp(&self.weights[*i].1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(i.cmp(j))
        });
        items
            .into_iter()
            .map(|i| {
                (
                    self.weights[i].0.clone(),
                    self.probability(&self.weights[i].0).unwrap_or(0.0),
                )
            })
            .collect()
    }
}

/// The belief of the adversarial stress: the hypothesis space is the eight bands A1 to A8.
pub fn panel_belief() -> BeliefState {
    BeliefState::from_names(AgentA::ALL.iter().map(|a| a.name().to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::{Card, Layer, UncertaintyType};

    #[test]
    fn the_belief_state_never_hits_zero_nor_one() {
        let mut belief = BeliefState::new(&["a", "b", "c"]);
        for _ in 0..200 {
            belief.observe(&[("a", 3.0)]);
        }
        let pa = belief.probability("a").expect("exists");
        let pb = belief.probability("b").expect("exists");
        assert!((FLOOR - 1e-12..=CEILING + 1e-12).contains(&pa), "a at {pa}");
        assert!((FLOOR - 1e-12..=CEILING + 1e-12).contains(&pb), "b at {pb}");
        assert_eq!(belief.most_likely(), Some("a"));
        assert!(belief.entropy() > 0.0);
    }

    #[test]
    fn invalid_evidence_does_not_zero_the_state() {
        let mut belief = BeliefState::new(&["a", "b"]);
        belief.observe(&[("a", 0.0), ("b", 0.0)]);
        let pa = belief.probability("a").expect("exists");
        assert!(
            (pa - 0.5).abs() < 1e-12,
            "with no useful evidence it returns to the uniform: {pa}"
        );
    }

    #[test]
    fn the_card_evidence_moves_belief_to_the_layer_of_the_card() {
        let mut belief = BeliefState::new(&["L1", "L2", "L3", "aleatoria", "epistemica"]);
        let card = Card::new("s2.1", Layer::Agent, UncertaintyType::Epistemic);
        for _ in 0..5 {
            belief.observe_card(&card, 3.0);
        }
        // the two hypotheses that match the card (L3 and epistemica) rise together, and tie
        let top = belief.most_likely().expect("has a top");
        assert!(top == "L3" || top == "epistemica", "top came out {top}");
        assert!(
            belief.probability("L3").expect("exists") > belief.probability("L1").expect("exists")
        );
        assert!(
            belief.probability("epistemica").expect("exists")
                > belief.probability("aleatoria").expect("exists")
        );
        let ranking = belief.ranking();
        assert!(ranking[0].1 > ranking.last().expect("has a last").1);
    }

    #[test]
    fn the_ranking_breaks_ties_by_declared_order() {
        let belief = BeliefState::new(&["first", "second", "third"]);
        let ranking = belief.ranking();
        assert_eq!(ranking[0].0, "first");
        assert_eq!(ranking[2].0, "third");
    }
}
