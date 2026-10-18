//! stochology-horizons — the queue and the call centre: the kernel of horizon zero.
//!
//! Here live the Erlang formula, the minimum scale that delivers a service target, the **Poisson
//! null** that serves as contrast and the mandatory summary of the second prohibition (mean, median,
//! quantiles) — H0 is the horizon where the toll of the solitary mean was measured, and the helper is
//! shared by the other horizons.
//!
//! Declared and not implemented: the queue with abandonment (lives in `stochology-book/src/fila.rs`) and
//! the reading of the real base `data/bank_calls.csv` (lives in the `labs --banco` binary).

use stochology_core::Rng;

/// The Erlang formula and the minimum scale live in the core (they also serve the R axis) and are
/// re-exported here, because H0 is the horizon where the queue was born.
pub use stochology_core::erlang::{erlang_b, erlang_c, minimal_scale};

/// The declared null model of H0: Poisson arrivals with the same mean.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PoissonNull {
    pub mean: f64,
}

impl PoissonNull {
    pub fn new(mean: f64) -> Self {
        PoissonNull { mean }
    }

    /// One count per step, by Knuth's method.
    pub fn count(&self, rng: &mut Rng) -> u32 {
        if self.mean <= 0.0 {
            return 0;
        }
        if self.mean > 700.0 {
            // outside the method's range: Gaussian with equal mean and variance
            return (self.mean + self.mean.sqrt() * standard_normal(rng))
                .max(0.0)
                .round() as u32;
        }
        let limit = (-self.mean).exp();
        let mut product = 1.0_f64;
        let mut k = 0u32;
        loop {
            k += 1;
            product *= open_uniform(rng);
            if product <= limit || k > 10_000 {
                return k - 1;
            }
        }
    }

    pub fn series(&self, seed: u64, steps: usize) -> Vec<u32> {
        let mut rng = Rng::new(seed);
        (0..steps).map(|_| self.count(&mut rng)).collect()
    }
}

fn open_uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn standard_normal(rng: &mut Rng) -> f64 {
    let u1 = open_uniform(rng);
    let u2 = open_uniform(rng);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Dispersion index: variance over mean. Poisson sits at 1; the heavy tail sits above.
pub fn dispersion_index(counts: &[u32]) -> f64 {
    if counts.len() < 2 {
        return 0.0;
    }
    let n = counts.len() as f64;
    let mean = counts.iter().map(|c| *c as f64).sum::<f64>() / n;
    if mean == 0.0 {
        return 0.0;
    }
    let variance = counts
        .iter()
        .map(|c| (*c as f64 - mean).powi(2))
        .sum::<f64>()
        / (n - 1.0);
    variance / mean
}

/// Mandatory summary where the function is nonlinear: mean, median, quantiles and dispersion, together.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub n: usize,
    pub mean: f64,
    pub median: f64,
    pub p05: f64,
    pub p95: f64,
    pub maximum: f64,
    pub deviation: f64,
}

/// Quantile by linear interpolation over the sorted sample (type 7 rule).
pub fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let q = q.clamp(0.0, 1.0);
    let position = q * (sorted.len() as f64 - 1.0);
    let low = position.floor() as usize;
    let high = position.ceil() as usize;
    if low == high {
        return sorted[low];
    }
    let weight = position - low as f64;
    sorted[low] * (1.0 - weight) + sorted[high] * weight
}

pub fn summary(series: &[f64]) -> Summary {
    let mut sorted = series.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    let mean = if n == 0 {
        0.0
    } else {
        sorted.iter().sum::<f64>() / n as f64
    };
    let deviation = if n < 2 {
        0.0
    } else {
        (sorted.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n as f64 - 1.0)).sqrt()
    };
    Summary {
        n,
        mean,
        median: quantile(&sorted, 0.5),
        p05: quantile(&sorted, 0.05),
        p95: quantile(&sorted, 0.95),
        maximum: sorted.last().copied().unwrap_or(0.0),
        deviation,
    }
}

/// The toll of reporting only the mean: how far the mean passes the median, as a fraction of the median.
pub fn mean_toll(series: &[f64]) -> f64 {
    let r = summary(series);
    if r.median == 0.0 {
        return 0.0;
    }
    r.mean / r.median - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Independent route: direct sum of `a^k/k!` in logarithmic scale (log-sum-exp), with no recurrence.
    fn erlang_c_by_sum(agents: u32, load: f64) -> f64 {
        let ln_factorial: Vec<f64> = (0..=agents as usize)
            .scan(0.0_f64, |acc, k| {
                if k > 0 {
                    *acc += (k as f64).ln();
                }
                Some(*acc)
            })
            .collect();
        let terms: Vec<f64> = (0..=agents as usize)
            .map(|k| (k as f64) * load.ln() - ln_factorial[k])
            .collect();
        let largest = terms.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let last = terms[..agents as usize]
            .iter()
            .map(|t| (t - largest).exp())
            .sum::<f64>();
        let queue =
            (terms[agents as usize] - largest).exp() * (agents as f64) / (agents as f64 - load);
        queue / (last + queue)
    }

    #[test]
    fn erlang_matches_the_exact_reference() {
        // Exact values computed with a rational fraction (Fraction) on 2026-10-16.
        let cases = [
            (3u32, 2.0_f64, 0.444_444_444_444_444_4_f64),
            (5, 3.0, 0.236_151_603_498_542_28),
            (10, 5.0, 0.036_105_359_158_320_19),
            (20, 17.0, 0.385_056_229_893_324_83),
        ];
        for (agents, load, expected) in cases {
            let obtained = erlang_c(agents, load);
            assert!(
                (obtained - expected).abs() < 1e-12,
                "erlang_c({agents}, {load}) = {obtained}, reference {expected}"
            );
        }
    }

    #[test]
    fn recurrence_and_log_sum_agree() {
        for (agents, load) in [(1u32, 0.5_f64), (5, 3.0), (10, 5.0), (30, 20.0), (60, 45.0)] {
            let recursion = erlang_c(agents, load);
            let sum = erlang_c_by_sum(agents, load);
            assert!(
                (recursion - sum).abs() < 1e-9,
                "two routes diverge at ({agents}, {load}): {recursion} against {sum}"
            );
        }
    }

    #[test]
    fn erlang_c_falls_with_more_servers_and_stays_between_b_and_one() {
        let load = 8.0;
        let mut previous = 1.0;
        for agents in 9..40 {
            let c = erlang_c(agents, load);
            assert!(
                c <= previous + 1e-12,
                "did not fall at {agents}: {c} > {previous}"
            );
            assert!(c >= erlang_b(agents, load) - 1e-12);
            assert!((0.0..=1.0).contains(&c));
            previous = c;
        }
        assert_eq!(
            erlang_c(8, 8.0),
            1.0,
            "load equal to capacity does not hold the queue"
        );
    }

    #[test]
    fn minimal_scale_meets_target_and_one_less_does_not() {
        for (lambda, mu, target) in [
            (8.0_f64, 1.0_f64, 0.8_f64),
            (234.0, 1.0, 0.8),
            (5.0, 2.0, 0.9),
        ] {
            let agents = minimal_scale(lambda, mu, target);
            let load = lambda / mu;
            assert!(erlang_c(agents, load) <= 1.0 - target + 1e-12);
            if (agents as f64) > load.ceil() {
                assert!(erlang_c(agents - 1, load) > 1.0 - target - 1e-12);
            }
        }
    }

    #[test]
    fn poisson_null_matches_mean_and_variance() {
        let null = PoissonNull::new(8.0);
        let series = null.series(2026, 200_000);
        let mean = series.iter().map(|c| *c as f64).sum::<f64>() / series.len() as f64;
        assert!((mean - 8.0).abs() < 0.08, "null mean: {mean}");
        let dispersion = dispersion_index(&series);
        assert!(
            (dispersion - 1.0).abs() < 0.05,
            "Poisson sits at 1: {dispersion}"
        );
    }

    #[test]
    fn null_series_is_deterministic() {
        let null = PoissonNull::new(3.0);
        assert_eq!(null.series(99, 500), null.series(99, 500));
        assert_ne!(null.series(99, 500), null.series(100, 500));
    }

    #[test]
    fn heavy_tail_bursts_the_dispersion_index() {
        // Declared null: Poisson, which sits at 1. The two-regime world sits far above.
        let null = PoissonNull::new(4.0);
        let quiet = null.series(1, 20_000);
        let mut rng = Rng::new(2);
        let mut stormy = Vec::with_capacity(20_000);
        for i in 0..20_000 {
            let mean = if i % 20 == 0 { 40.0 } else { 2.0 };
            stormy.push(PoissonNull::new(mean).count(&mut rng));
        }
        let mild = dispersion_index(&quiet);
        let wild = dispersion_index(&stormy);
        assert!(mild < 1.1, "null stays near 1: {mild}");
        assert!(wild > 3.0, "two regimes burst the null: {wild}");
    }

    #[test]
    fn summary_brings_median_and_quantiles_alongside_mean() {
        let mut rng = Rng::new(42);
        let series: Vec<f64> = (0..20_000)
            .map(|_| {
                let u = open_uniform(&mut rng);
                (1.0 / (1.0 - u)).powf(1.0 / 1.81)
            })
            .collect();
        let r = summary(&series);
        assert_eq!(r.n, 20_000);
        assert!(r.p05 < r.median && r.median < r.p95 && r.p95 <= r.maximum);
        assert!(
            r.mean > r.median,
            "right tail: mean {} median {}",
            r.mean,
            r.median
        );
        assert!(
            mean_toll(&series) > 0.5,
            "toll of the solitary mean: {}",
            mean_toll(&series)
        );
    }

    #[test]
    fn quantile_interpolates_and_stays_inside_the_sample() {
        let sorted = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(quantile(&sorted, 0.0), 1.0);
        assert_eq!(quantile(&sorted, 1.0), 4.0);
        assert!((quantile(&sorted, 0.5) - 2.5).abs() < 1e-12);
        assert!((quantile(&sorted, 0.25) - 1.75).abs() < 1e-12);
    }
}
