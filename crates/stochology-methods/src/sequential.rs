//! stochology-methods — sequential: evidence that survives optional stopping.
//!
//! The registry counts specifications tried (K) and corrects afterwards (Bonferroni over 38 at
//! the H2 gate). Counting is honest, but a correction chosen after the path is known does not
//! control the path itself: "run another round until the gate passes" is optional stopping, and a
//! hash on every round seals each step without sealing the stopping rule. An e-process closes
//! that gap by construction. Each round contributes an e-value, a non-negative statistic whose
//! expectation under the null is at most 1; the running product of conditionally valid e-values is
//! a test supermartingale, and Ville's inequality gives
//!
//! ```text
//! P_H0( sup_n E_n >= 1/alpha ) <= alpha
//! ```
//!
//! for every alpha, at every stopping time, including "stop when it passes". It is Royall's bound
//! (`P(sup_n LR_n >= k) <= 1/k`) with the likelihood ratio replaced by any e-value, so a round
//! need not have a likelihood to enter the ledger: a p-value enters through a calibrator.
//!
//! Two merging rules, and when each is valid:
//! - **product**: rounds are sequential and each e-value is valid *conditionally on the past*
//!   (the next round is designed after seeing the previous one, but its own data are fresh);
//! - **mean**: rounds may reuse the same data or depend on each other arbitrarily; the mean of
//!   e-values is always an e-value, at the price of growing only linearly.
//!
//! Declared and not implemented: the retroactive application to the 38 tests of the H2 gate. The
//! rounds of H2 changed data, test and environment, and the per-round p-values were recorded in
//! heterogeneous forms; which of them are conditionally valid is a declaration to be written
//! before any recomputation, not a choice to make while computing. See
//! `empirical/p22-parada/DECLARACAO.md`.

use stochology_core::Rng;

/// Floor that keeps a p-value away from zero: no probability is 0% (AGENTS.md §I, prohibition 4).
pub const P_FLOOR: f64 = 1e-12;

/// The p-to-e calibrators declared for the ledger.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Calibrator {
    /// `e = kappa * p^(kappa - 1)`, with `kappa` in (0, 1). Its integral over [0, 1] is exactly 1.
    Power { kappa: f64 },
    /// The mixture of all power calibrators over `kappa` uniform in (0, 1):
    /// `e = (1 - p + p ln p) / (p (ln p)^2)`. Admissible and free of a tuning parameter.
    #[default]
    Mixture,
}

/// Turns a valid p-value into an e-value. A p-value outside (0, 1] is clamped, never trusted as 0.
pub fn calibrate(p: f64, calibrator: Calibrator) -> f64 {
    let p = if p.is_nan() {
        1.0
    } else {
        p.clamp(P_FLOOR, 1.0)
    };
    match calibrator {
        Calibrator::Power { kappa } => {
            let kappa = kappa.clamp(1e-6, 1.0 - 1e-6);
            kappa * p.powf(kappa - 1.0)
        }
        Calibrator::Mixture => {
            let q = 1.0 - p;
            if q < 1e-4 {
                // Expansion around p = 1: numerator q^2/2 + q^3/6, denominator q^2 + O(q^4).
                return 0.5 + q / 6.0;
            }
            let ln = p.ln();
            (1.0 - p + p * ln) / (p * ln * ln)
        }
    }
}

/// How the rounds of a ledger are merged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Merge {
    /// Running product: sequential rounds, each valid conditionally on the past.
    Product,
    /// Running mean: valid under arbitrary dependence between rounds.
    Mean,
}

/// One round of the ledger: its label, its declared p-value, and the e-value it contributed.
#[derive(Debug, Clone, PartialEq)]
pub struct Round {
    pub label: String,
    pub p: f64,
    pub e: f64,
}

/// What the ledger says after the last round, at level `alpha`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Verdict {
    pub rounds: usize,
    /// The accumulated evidence, `E_n`.
    pub evidence: f64,
    /// The largest `E_k` seen along the path: the one that Ville's inequality bounds.
    pub running_max: f64,
    /// The first round (1-based) at which `E_k >= 1/alpha`, if any.
    pub first_crossing: Option<usize>,
    /// `log2(E_n)`: the evidence in bits, in the same ruler as the Occam cost of the registry.
    pub bits: f64,
    pub rejects: bool,
}

/// A ledger of rounds whose stopping rule need not be declared: the bound holds at any stop.
#[derive(Debug, Clone, PartialEq)]
pub struct Ledger {
    pub merge: Merge,
    pub calibrator: Calibrator,
    rounds: Vec<Round>,
}

impl Ledger {
    pub fn new(merge: Merge, calibrator: Calibrator) -> Self {
        Ledger {
            merge,
            calibrator,
            rounds: Vec::new(),
        }
    }

    /// Records a round from its p-value and returns the e-value it contributed.
    pub fn record_p(&mut self, label: &str, p: f64) -> f64 {
        let e = calibrate(p, self.calibrator);
        self.rounds.push(Round {
            label: label.to_string(),
            p,
            e,
        });
        e
    }

    /// Records a round that already produced an e-value (a likelihood ratio, for instance).
    /// A negative or non-finite e-value is not an e-value and is recorded as 0.
    pub fn record_e(&mut self, label: &str, e: f64) {
        let e = if e.is_finite() && e > 0.0 { e } else { 0.0 };
        self.rounds.push(Round {
            label: label.to_string(),
            p: f64::NAN,
            e,
        });
    }

    pub fn rounds(&self) -> &[Round] {
        &self.rounds
    }

    /// The path `E_1, ..., E_n` under the declared merge.
    pub fn path(&self) -> Vec<f64> {
        let mut path = Vec::with_capacity(self.rounds.len());
        let mut product = 1.0_f64;
        let mut sum = 0.0_f64;
        for (i, round) in self.rounds.iter().enumerate() {
            match self.merge {
                Merge::Product => {
                    product *= round.e;
                    path.push(product);
                }
                Merge::Mean => {
                    sum += round.e;
                    path.push(sum / (i + 1) as f64);
                }
            }
        }
        path
    }

    pub fn verdict(&self, alpha: f64) -> Verdict {
        let alpha = alpha.clamp(P_FLOOR, 1.0);
        let threshold = 1.0 / alpha;
        let path = self.path();
        let evidence = path.last().copied().unwrap_or(1.0);
        let running_max = path.iter().copied().fold(1.0_f64, f64::max);
        let first_crossing = path.iter().position(|e| *e >= threshold).map(|i| i + 1);
        Verdict {
            rounds: path.len(),
            evidence,
            running_max,
            first_crossing,
            bits: evidence.max(f64::MIN_POSITIVE).log2(),
            rejects: first_crossing.is_some(),
        }
    }
}

/// A valid p-value under the null: uniform on (0, 1), from the fixed-seed generator.
pub fn null_p(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

/// The naive rule that the e-process replaces: stop at the first round with `p < alpha`.
/// Returns the round (1-based) at which it would have declared the gate passed.
pub fn naive_first_pass(ps: &[f64], alpha: f64) -> Option<usize> {
    ps.iter().position(|p| *p < alpha).map(|i| i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integral(calibrator: Calibrator) -> f64 {
        // Midpoint rule on a log-spaced grid, which resolves the singularity at p = 0.
        let n = 200_000;
        let (lo, hi) = (P_FLOOR.ln(), 0.0_f64);
        let mut total = 0.0;
        for i in 0..n {
            let a = (lo + (hi - lo) * i as f64 / n as f64).exp();
            let b = (lo + (hi - lo) * (i + 1) as f64 / n as f64).exp();
            total += calibrate(0.5 * (a + b), calibrator) * (b - a);
        }
        total
    }

    #[test]
    fn calibrators_have_expectation_at_most_one_under_the_null() {
        for calibrator in [
            Calibrator::Mixture,
            Calibrator::Power { kappa: 0.5 },
            Calibrator::Power { kappa: 0.1 },
        ] {
            let mass = integral(calibrator);
            assert!(mass <= 1.0 + 1e-3, "{calibrator:?}: E[e] = {mass}");
            // The floor at 1e-12 truncates the mass below it: 1e-12^kappa for the power family.
            assert!(mass > 0.9, "{calibrator:?} wastes evidence: E[e] = {mass}");
        }
    }

    #[test]
    fn calibrators_are_decreasing_and_never_negative() {
        let grid = [1e-9, 1e-6, 1e-3, 0.01, 0.05, 0.2, 0.5, 0.9, 0.99999, 1.0];
        for calibrator in [Calibrator::Mixture, Calibrator::Power { kappa: 0.5 }] {
            let values: Vec<f64> = grid.iter().map(|p| calibrate(*p, calibrator)).collect();
            assert!(values.iter().all(|e| e.is_finite() && *e >= 0.0));
            assert!(
                values.windows(2).all(|w| w[0] >= w[1]),
                "{calibrator:?} is not monotone: {values:?}"
            );
        }
        let at_one = calibrate(1.0, Calibrator::Mixture);
        assert!((at_one - 0.5).abs() < 1e-9, "mixture at p = 1: {at_one}");
        let near_one = calibrate(1.0 - 2e-4, Calibrator::Mixture);
        assert!((near_one - 0.5).abs() < 1e-3, "{near_one}");
    }

    #[test]
    fn a_zero_p_value_is_clamped_not_trusted() {
        let e0 = calibrate(0.0, Calibrator::Mixture);
        let ef = calibrate(P_FLOOR, Calibrator::Mixture);
        assert!(e0.is_finite() && (e0 - ef).abs() < 1e-6 * ef);
        assert_eq!(calibrate(f64::NAN, Calibrator::Mixture), 0.5);
    }

    #[test]
    fn ville_bounds_the_product_under_optional_stopping() {
        // 4 000 null paths of 40 rounds each, stopped at the first crossing: the fraction that
        // ever crosses 1/alpha stays at or below alpha, with a binomial margin of 3 deviations.
        let alpha = 0.05;
        let paths = 4_000;
        let mut rng = Rng::new(2026);
        let mut crossed = 0;
        for _ in 0..paths {
            let mut ledger = Ledger::new(Merge::Product, Calibrator::Mixture);
            for r in 0..40 {
                ledger.record_p(&format!("round {r}"), null_p(&mut rng));
            }
            if ledger.verdict(alpha).rejects {
                crossed += 1;
            }
        }
        let rate = crossed as f64 / paths as f64;
        let margin = 3.0 * (alpha * (1.0 - alpha) / paths as f64).sqrt();
        assert!(rate <= alpha + margin, "e-process crossed in {rate}");
    }

    #[test]
    fn the_naive_rule_does_not_survive_the_same_path() {
        // The same null p-values, read by "stop when p < alpha": with seven rounds (the H2 gate)
        // the false pass rate is 1 - 0.95^7, about 0.30, six times the declared alpha.
        let alpha = 0.05;
        let paths = 4_000;
        let mut rng = Rng::new(2026);
        let mut naive = 0;
        let mut eprocess = 0;
        for _ in 0..paths {
            let ps: Vec<f64> = (0..7).map(|_| null_p(&mut rng)).collect();
            if naive_first_pass(&ps, alpha).is_some() {
                naive += 1;
            }
            let mut ledger = Ledger::new(Merge::Product, Calibrator::Mixture);
            for (r, p) in ps.iter().enumerate() {
                ledger.record_p(&format!("round {r}"), *p);
            }
            if ledger.verdict(alpha).rejects {
                eprocess += 1;
            }
        }
        let naive = naive as f64 / paths as f64;
        let eprocess = eprocess as f64 / paths as f64;
        let expected = 1.0 - (1.0 - alpha).powi(7);
        assert!((naive - expected).abs() < 0.03, "naive {naive}");
        assert!(eprocess <= alpha, "e-process {eprocess}");
        assert!(naive > 4.0 * alpha);
    }

    #[test]
    fn real_signal_accumulates_and_crosses() {
        // Rounds with p around 0.01, none of which crosses 1/alpha = 20 alone under the mixture,
        // cross together in a few rounds: evidence accumulates instead of being re-drawn.
        let alone = calibrate(0.01, Calibrator::Mixture);
        assert!(alone < 20.0, "one round alone: {alone}");
        let mut ledger = Ledger::new(Merge::Product, Calibrator::Mixture);
        for r in 0..4 {
            ledger.record_p(&format!("round {r}"), 0.01);
        }
        let verdict = ledger.verdict(0.05);
        assert!(verdict.rejects, "{verdict:?}");
        assert!(verdict.first_crossing.unwrap() <= 3, "{verdict:?}");
        assert!(verdict.bits > 4.32, "{verdict:?}");
    }

    #[test]
    fn a_failed_round_costs_evidence_in_the_product_and_not_in_the_bound() {
        let mut ledger = Ledger::new(Merge::Product, Calibrator::Mixture);
        ledger.record_p("strong", 0.001);
        let before = ledger.verdict(0.05).evidence;
        ledger.record_p("null-looking", 0.8);
        let after = ledger.verdict(0.05);
        assert!(after.evidence < before, "a weak round must pay: {after:?}");
        assert!((after.running_max - before).abs() < 1e-12);
    }

    #[test]
    fn the_mean_is_valid_under_dependence_and_slower() {
        // Ten identical copies of the same null p-value: the product would be fooled by the
        // duplication (it multiplies the same evidence ten times); the mean is not.
        let mut rng = Rng::new(7);
        let mut crossed_mean = 0;
        let paths = 4_000;
        for _ in 0..paths {
            let p = null_p(&mut rng);
            let mut ledger = Ledger::new(Merge::Mean, Calibrator::Mixture);
            for r in 0..10 {
                ledger.record_p(&format!("copy {r}"), p);
            }
            if ledger.verdict(0.05).rejects {
                crossed_mean += 1;
            }
        }
        let rate = crossed_mean as f64 / paths as f64;
        assert!(rate <= 0.05 + 0.011, "mean under duplication: {rate}");
        let mut product = Ledger::new(Merge::Product, Calibrator::Mixture);
        let mut mean = Ledger::new(Merge::Mean, Calibrator::Mixture);
        for r in 0..10 {
            product.record_p(&format!("copy {r}"), 0.02);
            mean.record_p(&format!("copy {r}"), 0.02);
        }
        assert!(product.verdict(0.05).rejects);
        assert!(!mean.verdict(0.05).rejects);
    }

    #[test]
    fn invalid_e_values_are_recorded_as_zero_and_the_ledger_is_deterministic() {
        let mut ledger = Ledger::new(Merge::Product, Calibrator::default());
        ledger.record_e("negative", -3.0);
        assert_eq!(ledger.verdict(0.05).evidence, 0.0);
        let build = || {
            let mut rng = Rng::new(11);
            let mut l = Ledger::new(Merge::Product, Calibrator::Mixture);
            for r in 0..12 {
                l.record_p(&format!("round {r}"), null_p(&mut rng));
            }
            l
        };
        assert_eq!(build(), build());
        assert_eq!(build().rounds().len(), 12);
        assert_eq!(
            Ledger::new(Merge::Mean, Calibrator::Mixture)
                .verdict(0.05)
                .evidence,
            1.0
        );
    }
}
