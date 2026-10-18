//! Chapter 2 — World, Model and Agent: the fallacy lives on the bridge.
//!
//! The prosecutor's fallacy swaps P(evidence | innocent) for P(innocent | evidence).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bridge {
    /// What the Model delivers: P(evidence | innocent).
    pub likelihood: f64,
    /// What the Agent answers: P(innocent | evidence).
    pub posterior_of_innocence: f64,
}

/// Computes the two quantities the prosecutor confuses.
pub fn prosecutor_fallacy(prevalence: f64, sensitivity: f64, false_positive: f64) -> Bridge {
    let p_evidence = prevalence * sensitivity + (1.0 - prevalence) * false_positive;
    let posterior_of_guilt = prevalence * sensitivity / p_evidence;
    Bridge {
        likelihood: false_positive,
        posterior_of_innocence: 1.0 - posterior_of_guilt,
    }
}

/// The ratio between what was said and what could be concluded.
pub fn swap_factor(prevalence: f64, sensitivity: f64, false_positive: f64) -> f64 {
    let p = prosecutor_fallacy(prevalence, sensitivity, false_positive);
    p.posterior_of_innocence / p.likelihood
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rare_coincidence_does_not_become_guilt_in_a_small_population() {
        // A one-in-a-million coincidence, but the prior chance that the defendant is guilty
        // is also one in a million: the evidence separates very little.
        let p = prosecutor_fallacy(1.0 / 1_000_000.0, 1.0, 1.0 / 1_000_000.0);
        assert!((p.likelihood - 1e-6).abs() < 1e-18);
        assert!(p.posterior_of_innocence > 0.49 && p.posterior_of_innocence < 0.51);
    }

    #[test]
    fn when_prevalence_is_high_the_bridge_barely_lies() {
        let p = prosecutor_fallacy(0.9, 1.0, 1e-6);
        assert!(p.posterior_of_innocence < 1e-5);
    }

    #[test]
    fn the_swap_factor_grows_when_prevalence_falls() {
        let high = swap_factor(0.01, 1.0, 1e-6);
        let low = swap_factor(1e-6, 1.0, 1e-6);
        assert!(low > high * 1000.0, "{} vs {}", high, low);
    }
}
