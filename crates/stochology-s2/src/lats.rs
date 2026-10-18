//! Tree search with estimate (LATS) over a declared world.

use stochology_core::Rng;

/// The declared world over which deliberation happens.
pub trait World {
    fn actions(&self, state: &str) -> Vec<String>;
    fn step(&self, state: &str, action: &str) -> String;
    fn evaluate(&self, state: &str) -> f64;
    fn terminal(&self, state: &str) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
    pub action: String,
    pub value: f64,
    pub nodes_visited: usize,
    pub trail: Vec<String>,
}

/// Tree search with estimate: UCB selection, deterministic rollout, mean of the leaf.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lats {
    pub depth: usize,
    pub budget: usize,
    pub exploration: f64,
    pub seed: u64,
}

impl Default for Lats {
    fn default() -> Self {
        Lats {
            depth: 3,
            budget: 40,
            exploration: 1.0,
            seed: 2026,
        }
    }
}

impl Lats {
    pub fn deliberate<W: World>(&self, world: &W, root: &str) -> Option<Decision> {
        let actions = world.actions(root);
        if actions.is_empty() {
            return None;
        }
        let mut rng = Rng::new(self.seed);
        let mut statistics = vec![(0u32, 0.0_f64); actions.len()];
        for _ in 0..self.budget.max(1) {
            let total: u32 = statistics.iter().map(|(n, _)| *n).sum();
            let mut chosen = 0usize;
            let mut best = f64::NEG_INFINITY;
            for (i, (n, sum)) in statistics.iter().enumerate() {
                let ucb = if *n == 0 {
                    f64::INFINITY
                } else {
                    sum / *n as f64
                        + self.exploration * (((total as f64 + 1.0).ln()) / *n as f64).sqrt()
                };
                if ucb > best {
                    best = ucb;
                    chosen = i;
                }
            }
            let mut state = world.step(root, &actions[chosen]);
            for _ in 1..self.depth.max(1) {
                if world.terminal(&state) {
                    break;
                }
                let next = world.actions(&state);
                if next.is_empty() {
                    break;
                }
                let i = rng.uniform(next.len() as u64) as usize;
                state = world.step(&state, &next[i]);
            }
            let value = world.evaluate(&state);
            statistics[chosen].0 += 1;
            statistics[chosen].1 += value;
        }
        let (index, (visits, sum)) = statistics
            .iter()
            .enumerate()
            .max_by_key(|(i, (n, _))| (*n, std::cmp::Reverse(*i)))
            .map(|(i, e)| (i, *e))?;
        Some(Decision {
            action: actions[index].clone(),
            value: if visits == 0 {
                0.0
            } else {
                sum / visits as f64
            },
            nodes_visited: visits as usize,
            trail: vec![actions[index].clone()],
        })
    }
}

/// What a greedy agent does: the action of greatest immediate value, without looking ahead.
pub fn greedy<W: World>(world: &W, root: &str) -> Option<Decision> {
    let actions = world.actions(root);
    let mut best: Option<Decision> = None;
    for action in actions {
        let state = world.step(root, &action);
        let value = world.evaluate(&state);
        let candidate = Decision {
            action,
            value,
            nodes_visited: 1,
            trail: Vec::new(),
        };
        if best.as_ref().map(|m| value > m.value).unwrap_or(true) {
            best = Some(candidate);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Toy world: the immediate prize is smaller than the prize of two steps.
    struct TwoPaths;

    impl World for TwoPaths {
        fn actions(&self, state: &str) -> Vec<String> {
            match state {
                "root" => vec!["immediate".into(), "deferred".into()],
                "deferred" => vec!["advance".into()],
                _ => Vec::new(),
            }
        }

        fn step(&self, state: &str, action: &str) -> String {
            match (state, action) {
                ("root", "immediate") => "end_immediate".into(),
                ("root", "deferred") => "deferred".into(),
                ("deferred", "advance") => "end_deferred".into(),
                _ => state.to_string(),
            }
        }

        fn evaluate(&self, state: &str) -> f64 {
            match state {
                "end_immediate" => 0.35,
                "deferred" => 0.20,
                "end_deferred" => 0.90,
                _ => 0.0,
            }
        }

        fn terminal(&self, state: &str) -> bool {
            state.starts_with("end_")
        }
    }

    #[test]
    fn deliberation_beats_the_greedy_agent() {
        let world = TwoPaths;
        let lats = Lats {
            depth: 2,
            budget: 40,
            exploration: 1.0,
            seed: 2026,
        };
        let decision = lats.deliberate(&world, "root").expect("decides");
        let greedy_agent = greedy(&world, "root").expect("decides");
        assert_eq!(greedy_agent.action, "immediate");
        assert_eq!(decision.action, "deferred");
        assert!(
            decision.value > greedy_agent.value,
            "{} against {}",
            decision.value,
            greedy_agent.value
        );
    }

    #[test]
    fn the_decision_is_deterministic_in_the_seed() {
        let world = TwoPaths;
        let lats = Lats::default();
        let first = lats.deliberate(&world, "root").expect("decides");
        let second = lats.deliberate(&world, "root").expect("decides");
        assert_eq!(first, second);
        let other = Lats { seed: 7, ..lats }
            .deliberate(&world, "root")
            .expect("decides");
        assert_eq!(other.action, first.action);
    }

    #[test]
    fn a_world_without_actions_does_not_decide() {
        struct Empty;
        impl World for Empty {
            fn actions(&self, _: &str) -> Vec<String> {
                Vec::new()
            }
            fn step(&self, e: &str, _: &str) -> String {
                e.to_string()
            }
            fn evaluate(&self, _: &str) -> f64 {
                0.0
            }
            fn terminal(&self, _: &str) -> bool {
                true
            }
        }
        assert!(Lats::default().deliberate(&Empty, "root").is_none());
        assert!(greedy(&Empty, "root").is_none());
    }
}
