//! stochology-horizons — the stock market, employment and fire: the kernel of horizon one.
//!
//! Three mechanisms of H1 live here: the leverage drag (arithmetic mean against geometric
//! mean), the selection bias of an experiment against the observed group, and the Hill
//! tail estimator. The mandatory summary comes from `stochology-horizons`: where the dynamics is multiplicative, the mean
//! alone is forbidden.
//!
//! Declared and not implemented: the reading of the five bases in `data/` (lives in the laboratories
//! of chapters 17 and 18) and the intra-cohort correlation, which remains without an instrument in Rust.

use stochology_core::Rng;

/// Leverage drag: the difference between the levered arithmetic mean and the compound rate.
pub fn drag(leverage: f64, returns: &[f64]) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }
    let n = returns.len() as f64;
    let mean = returns.iter().sum::<f64>() / n;
    let compound = returns
        .iter()
        .map(|r| (1.0 + leverage * r).max(1e-12).ln())
        .sum::<f64>()
        / n;
    leverage * mean - (compound.exp() - 1.0)
}

/// Compound rate of the levered series, per period.
pub fn compound_rate(leverage: f64, returns: &[f64]) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }
    let n = returns.len() as f64;
    let sum = returns
        .iter()
        .map(|r| (1.0 + leverage * r).max(1e-12).ln())
        .sum::<f64>();
    (sum / n).exp() - 1.0
}

/// Fraction of wealth that a single adverse day destroys in a levered portfolio.
pub fn ruin_factor(leverage: f64, worst_day: f64) -> f64 {
    (1.0 - (1.0 + leverage * worst_day).max(0.0)).clamp(0.0, 1.0)
}

/// Hill estimator of the tail index, with the `k` largest: `alpha = k / sum(ln(x_i/x_k+1))`.
pub fn hill(series: &[f64], k: usize) -> f64 {
    stochology_core::hill_at_cutoff(series, k, k)
}

/// Raw comparison, experiment gain and selection bias between the observed and the drawn group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionBias {
    pub raw: f64,
    pub experimental: f64,
    pub bias: f64,
}

pub fn mean(series: &[f64]) -> f64 {
    if series.is_empty() {
        return 0.0;
    }
    series.iter().sum::<f64>() / series.len() as f64
}

pub fn selection_bias(
    treated: &[f64],
    observed_control: &[f64],
    experimental_control: &[f64],
) -> SelectionBias {
    let raw = mean(treated) - mean(observed_control);
    let experimental = mean(treated) - mean(experimental_control);
    // the bias is the shift of the raw estimate: how much the observed group lies against the drawn one
    SelectionBias {
        raw,
        experimental,
        bias: raw - experimental,
    }
}

/// Multiplicative series with a right tail: the world where the median and the mean tell
/// different stories. It serves as declared contrast for the summary of H0.
pub fn multiplicative_series(seed: u64, n: usize, steps: usize, mean_of_log: f64) -> Vec<f64> {
    let mut rng = Rng::new(seed);
    (0..n)
        .map(|_| {
            let mut value = 1.0_f64;
            for _ in 0..steps {
                value *= (mean_of_log + 0.18 * standard_normal(&mut rng)).exp();
            }
            value
        })
        .collect()
}

pub(crate) fn standard_normal(rng: &mut Rng) -> f64 {
    let u1 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let u2 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h0::summary;

    #[test]
    fn drag_of_symmetric_series_has_exact_value() {
        let series = [-0.1, 0.1];
        let expected = 1.0 - (0.99_f64).sqrt();
        let obtained = drag(1.0, &series);
        assert!(
            (obtained - expected).abs() < 1e-12,
            "drag {obtained}, expected {expected}"
        );
        assert!((drag(2.0, &series) - (1.0 - 0.96_f64.sqrt())).abs() < 1e-12);
    }

    #[test]
    fn drag_grows_with_leverage() {
        let mut rng = Rng::new(2026);
        let series: Vec<f64> = (0..2_000)
            .map(|_| 0.0004 + 0.011 * standard_normal(&mut rng))
            .collect();
        let (one, two, three) = (drag(1.0, &series), drag(2.0, &series), drag(3.0, &series));
        assert!(
            one < two && two < three,
            "monotone drag: {one} {two} {three}"
        );
        assert!(one > 0.0, "positive drag with volatility: {one}");
        assert!(
            (compound_rate(1.0, &series) - (mean(&series) - one)).abs() < 1e-12,
            "the compound rate is the mean minus the drag"
        );
    }

    #[test]
    fn the_1987_day_brings_down_whoever_is_at_three_times() {
        let destroyed = ruin_factor(3.0, -0.226);
        assert!((destroyed - 0.678).abs() < 1e-12, "destroyed {destroyed}");
        assert!(ruin_factor(1.0, -0.226) < 0.23);
        assert_eq!(ruin_factor(5.0, -0.30), 1.0, "portfolio wiped out");
        assert_eq!(ruin_factor(1.0, 0.10), 0.0, "a good day destroys nothing");
    }

    #[test]
    fn hill_recovers_the_alpha_of_a_pareto() {
        let mut rng = Rng::new(7);
        let alpha = 1.81_f64;
        let series: Vec<f64> = (0..4_000)
            .map(|_| {
                let u = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
                (1.0 - u).powf(-1.0 / alpha)
            })
            .collect();
        let obtained = hill(&series, 400);
        assert!(
            (obtained - alpha).abs() < 0.35,
            "Hill {obtained} against {alpha}"
        );
        assert!(hill(&series, 1).is_finite());
        assert!(
            hill(&series, 100_000).is_nan(),
            "without enough k there is no estimate"
        );
    }

    #[test]
    fn selection_bias_flips_the_sign_of_the_gain() {
        // LaLonde shape: treated with a real gain, observed control richer than the drawn one.
        let treated = vec![6_349.0, 6_349.0];
        let observed = vec![14_847.0, 14_847.0];
        let experimental = vec![4_555.0, 4_555.0];
        let v = selection_bias(&treated, &observed, &experimental);
        assert!((v.raw - (-8_498.0)).abs() < 1e-9);
        assert!((v.experimental - 1_794.0).abs() < 1e-9);
        assert!((v.bias - (-10_292.0)).abs() < 1e-9);
    }

    #[test]
    fn median_and_mean_tell_different_stories() {
        let series = multiplicative_series(2026, 20_000, 2_660, 0.00016);
        let r = summary(&series);
        assert!(
            r.median < r.mean / 3.0,
            "median {} mean {}",
            r.median,
            r.mean
        );
        assert!(r.p95 > 4.0 * r.median);
        assert!(r.p05 < r.median);
    }
}
