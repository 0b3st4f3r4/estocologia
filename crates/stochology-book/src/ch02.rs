//! Chapter 2 — World, Model and Agent: the fallacy lives on the bridge.
//!
//! The prosecutor's fallacy swaps P(evidence | innocent) for P(innocent | evidence).
//! The gambler's fallacy gives memory to a process that has none.

use stochology_core::Rng;

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

fn unit(rng: &mut Rng) -> f64 {
    (rng.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
}

/// What follows a streak: how many sequences began with it and how often the next draw repeats it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AfterStreak {
    pub cases: usize,
    pub repeats: f64,
}

/// A fair coin, a process without memory: among `sequences` runs of `streak + 1` tosses, those that begin
/// with `streak` heads in a row, and how often the next toss is heads.
pub fn coin_after_streak(seed: u64, sequences: usize, streak: usize) -> AfterStreak {
    let mut rng = Rng::new(seed);
    let (mut cases, mut heads_next) = (0usize, 0usize);
    for _ in 0..sequences {
        let all_heads = (0..streak).all(|_| unit(&mut rng) < 0.5);
        let next_head = unit(&mut rng) < 0.5;
        if all_heads {
            cases += 1;
            heads_next += usize::from(next_head);
        }
    }
    AfterStreak {
        cases,
        repeats: heads_next as f64 / cases as f64,
    }
}

/// An urn drawn without replacement, a process with memory: among `sequences` runs, those that begin with
/// `streak` red balls in a row, and how often the next ball is red.
pub fn urn_after_streak(
    seed: u64,
    sequences: usize,
    streak: usize,
    reds: usize,
    blues: usize,
) -> AfterStreak {
    let mut rng = Rng::new(seed);
    let (mut cases, mut red_next) = (0usize, 0usize);
    for _ in 0..sequences {
        let (mut r, b) = (reds, blues);
        let mut all_red = true;
        for _ in 0..streak {
            if unit(&mut rng) < r as f64 / (r + b) as f64 {
                r -= 1;
            } else {
                all_red = false;
                break;
            }
        }
        if all_red {
            cases += 1;
            red_next += usize::from(unit(&mut rng) < r as f64 / (r + b) as f64);
        }
    }
    AfterStreak {
        cases,
        repeats: red_next as f64 / cases as f64,
    }
}

/// Exact chance that the next ball is red after `streak` reds, without replacement.
pub fn urn_exact(reds: usize, blues: usize, streak: usize) -> f64 {
    (reds - streak) as f64 / (reds + blues - streak) as f64
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

    #[test]
    fn a_coin_forgets_the_streak_and_an_urn_does_not() {
        let coin = coin_after_streak(2026, 1_000_000, 5);
        assert!(coin.cases > 25_000);
        assert!((coin.repeats - 0.5).abs() < 0.01, "coin {}", coin.repeats);
        let urn = urn_after_streak(2026, 1_000_000, 5, 10, 10);
        let exact = urn_exact(10, 10, 5);
        assert!((exact - 1.0 / 3.0).abs() < 1e-15);
        assert!((urn.repeats - exact).abs() < 0.01, "urn {}", urn.repeats);
    }

    #[test]
    fn the_mutant_that_gives_the_coin_the_memory_of_the_urn_is_killed() {
        // The mutant answers for a coin with the formula of an urn that is being emptied.
        let mutant = urn_exact(10, 10, 5);
        let coin = coin_after_streak(7, 400_000, 5);
        assert!((coin.repeats - mutant).abs() > 0.1);
    }
}
