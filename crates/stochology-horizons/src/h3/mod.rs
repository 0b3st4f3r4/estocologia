//! stochology-horizons — the continuous mesh: strict clock, calibration and the performative loop.
//!
//! H3 is the horizon where everything runs live: the six flows under a strict clock, the ECE of the vault,
//! the Brier skill and the performative loop (what the agent does to the indicator once it starts
//! being paid on it). The third prohibition of the contract lives here: run with and without the action.
//!
//! Declared and not implemented: the reading of the six real flows and the Rhode Island vault, which
//! remain as record; what exists here is the mechanism, not the base.

pub mod beta;
pub mod chain;
pub mod pretest;
pub mod shelf;

use stochology_core::Rng;

fn uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn normal(rng: &mut Rng) -> f64 {
    let u1 = uniform(rng);
    let u2 = uniform(rng);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Expected calibration error, by confidence buckets. It falls with the sample size.
pub fn ece(confidence: &[f64], hits: &[bool], buckets: usize) -> f64 {
    if confidence.len() != hits.len() || confidence.is_empty() || buckets == 0 {
        return f64::NAN;
    }
    let mut sum = vec![0.0_f64; buckets];
    let mut hit_sums = vec![0.0_f64; buckets];
    let mut count = vec![0usize; buckets];
    for (p, y) in confidence.iter().zip(hits.iter()) {
        let index = ((p.clamp(0.0, 1.0) * buckets as f64).floor() as usize).min(buckets - 1);
        sum[index] += p;
        hit_sums[index] += if *y { 1.0 } else { 0.0 };
        count[index] += 1;
    }
    let n = confidence.len() as f64;
    let mut ece = 0.0;
    for index in 0..buckets {
        if count[index] == 0 {
            continue;
        }
        let mean_confidence = sum[index] / count[index] as f64;
        let mean_hit = hit_sums[index] / count[index] as f64;
        ece += (count[index] as f64 / n) * (mean_hit - mean_confidence).abs();
    }
    ece
}

/// Brier skill against the declared climatology (the base frequency).
pub fn brier_skill(forecasts: &[f64], observed: &[bool]) -> f64 {
    if forecasts.len() != observed.len() || forecasts.is_empty() {
        return f64::NAN;
    }
    let n = forecasts.len() as f64;
    let climate = observed.iter().filter(|y| **y).count() as f64 / n;
    let brier = forecasts
        .iter()
        .zip(observed.iter())
        .map(|(p, y)| (p - if *y { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        / n;
    let brier_climate = observed
        .iter()
        .map(|y| (climate - if *y { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        / n;
    if brier_climate == 0.0 {
        return 0.0;
    }
    1.0 - brier / brier_climate
}

/// How many events read the future: the read cannot pass the clock.
pub fn leaks(clocks: &[u64], reads: &[u64]) -> usize {
    clocks
        .iter()
        .zip(reads.iter())
        .filter(|(clock, read)| read > clock)
        .count()
}

/// The canary planted on purpose: if it fires, there was a leak.
pub fn planted_canary(clocks: &[u64], reads: &[u64]) -> bool {
    leaks(clocks, reads) > 0
}

/// Flow under a strict clock: the read of each event is the instant itself (one unit of delay).
pub fn strict_flow(seed: u64, n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut rng = Rng::new(seed);
    let mut clocks = Vec::with_capacity(n);
    let mut instant = 0u64;
    for _ in 0..n {
        instant += 1 + rng.uniform(3);
        clocks.push(instant);
    }
    let reads = clocks.clone();
    (clocks, reads)
}

/// The performative loop: the gain on the indicator against the gain on the target, with the action.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActionLoop {
    pub gain_on_indicator: f64,
    pub gain_on_target: f64,
    pub ratio: f64,
    pub passive: f64,
}

/// An agent that acts in the top decile of the indicator. The indicator only follows the target if its own noise
/// is small: it is the ratio that measures the loop.
pub fn action_loop(seed: u64, n: usize, indicator_noise: f64) -> ActionLoop {
    let mut rng = Rng::new(seed);
    let mut target = Vec::with_capacity(n);
    let mut indicator = Vec::with_capacity(n);
    for _ in 0..n {
        let x = normal(&mut rng);
        target.push(x + normal(&mut rng));
        indicator.push(x + indicator_noise * normal(&mut rng));
    }
    let mean_target = target.iter().sum::<f64>() / n as f64;
    let mean_indicator = indicator.iter().sum::<f64>() / n as f64;
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| {
        indicator[*b]
            .partial_cmp(&indicator[*a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let cut = (n / 10).max(1);
    let chosen = &order[..cut];
    let gain_on_indicator =
        chosen.iter().map(|i| indicator[*i]).sum::<f64>() / cut as f64 - mean_indicator;
    let gain_on_target = chosen.iter().map(|i| target[*i]).sum::<f64>() / cut as f64 - mean_target;
    let ratio = if gain_on_indicator.abs() < 1e-12 {
        0.0
    } else {
        gain_on_target / gain_on_indicator
    };
    let target_sd = (target
        .iter()
        .map(|v| (v - mean_target).powi(2))
        .sum::<f64>()
        / n as f64)
        .sqrt();
    let indicator_sd = (indicator
        .iter()
        .map(|v| (v - mean_indicator).powi(2))
        .sum::<f64>()
        / n as f64)
        .sqrt();
    let covariance = target
        .iter()
        .zip(indicator.iter())
        .map(|(a, g)| (a - mean_target) * (g - mean_indicator))
        .sum::<f64>()
        / n as f64;
    let passive = if target_sd > 0.0 && indicator_sd > 0.0 {
        covariance / (target_sd * indicator_sd)
    } else {
        0.0
    };
    ActionLoop {
        gain_on_indicator,
        gain_on_target,
        ratio,
        passive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ece_is_zero_for_the_calibrated_and_high_for_the_overshooting() {
        let mut rng = Rng::new(2026);
        let mut confidence = Vec::new();
        let mut hits = Vec::new();
        for _ in 0..20_000 {
            let p = uniform(&mut rng);
            confidence.push(p);
            hits.push(uniform(&mut rng) < p);
        }
        let calibrated = ece(&confidence, &hits, 20);
        assert!(calibrated < 0.02, "calibrated measures {calibrated}");
        let exaggerated: Vec<f64> = confidence.iter().map(|p| (p * 1.6).min(0.99)).collect();
        let overshot = ece(&exaggerated, &hits, 20);
        assert!(overshot > 0.10, "overshot measures {overshot}");
    }

    #[test]
    fn ece_tolerance_depends_on_n() {
        let mut small = 0.0;
        let mut large = 0.0;
        let rounds = 60;
        for seed in 0..rounds {
            let mut rng = Rng::new(1000 + seed);
            let mut confidence = Vec::new();
            let mut hits = Vec::new();
            for _ in 0..8_000 {
                let p = uniform(&mut rng);
                confidence.push(p);
                hits.push(uniform(&mut rng) < p);
            }
            small += ece(&confidence[..200], &hits[..200], 20);
            large += ece(&confidence, &hits, 20);
        }
        let small = small / rounds as f64;
        let large = large / rounds as f64;
        assert!(
            small > large,
            "200 points measure {small}, 8000 measure {large}"
        );
    }

    #[test]
    fn brier_skill_rewards_the_good_and_punishes_the_bad() {
        let mut rng = Rng::new(17);
        let mut good = Vec::new();
        let mut bad = Vec::new();
        let mut observed = Vec::new();
        for _ in 0..5_000 {
            let p = 0.25 + 0.5 * uniform(&mut rng);
            good.push(p);
            bad.push(1.0 - p);
            observed.push(uniform(&mut rng) < p);
        }
        assert!(brier_skill(&good, &observed) > 0.0);
        assert!(brier_skill(&bad, &observed) < 0.0);
    }

    #[test]
    fn the_canary_catches_the_reading_of_the_future() {
        let (clocks, reads) = strict_flow(2026, 500);
        assert_eq!(leaks(&clocks, &reads), 0);
        assert!(!planted_canary(&clocks, &reads));
        let mut punctured = reads.clone();
        punctured[7] = clocks[7] + 1;
        assert_eq!(leaks(&clocks, &punctured), 1);
        assert!(planted_canary(&clocks, &punctured));
        let (clocks2, reads2) = strict_flow(2026, 500);
        assert_eq!(clocks, clocks2);
        assert_eq!(reads, reads2);
    }

    #[test]
    fn the_loop_measures_what_the_action_does_to_the_indicator() {
        let perfect = action_loop(2026, 40_000, 0.0);
        assert!(
            (perfect.ratio - 1.0).abs() < 0.12,
            "an indicator that is the target does not shrink: {perfect:?}"
        );
        let biased = action_loop(2026, 40_000, 1.0);
        assert!(
            biased.ratio < 0.8,
            "with its own noise, the gain evaporates: {biased:?}"
        );
        let worse = action_loop(2026, 40_000, 2.0);
        assert!(
            worse.ratio < biased.ratio,
            "the loop grows with noise: {worse:?}"
        );
        assert!(biased.passive < perfect.passive);
    }

    #[test]
    fn the_passive_correlation_follows_the_exact_account() {
        // target = x + e and indicator = x + noise * z, with everything standard normal:
        // corr = 1 / sqrt(2 * (1 + noise^2)). The null here has an exact value, not a range.
        for noise in [0.0_f64, 0.5, 1.0, 2.0] {
            let measured = action_loop(2026, 40_000, noise).passive;
            let exact = 1.0 / (2.0 * (1.0 + noise * noise)).sqrt();
            assert!(
                (measured - exact).abs() < 0.03,
                "noise {noise}: {measured} against {exact}"
            );
        }
    }
}
