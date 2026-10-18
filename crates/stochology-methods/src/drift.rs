//! stochology-methods — drift: when the world slips out from under the model.
//!
//! Two tools: CUSUM, which watches the mean and warns when it shifts, and the Kullback-Leibler
//! divergence, which measures the size of the shift between two declared distributions. The
//! choice of parameters follows the classical usage (slack 0.75 sigma and threshold 10 sigma),
//! calibrated so that the null does not fire in 100 thousand points: the null stays declared and
//! measured, not assumed.
//!
//! Declared and not implemented: covariance drift (only location drift is here) and the link to
//! the `deriva_declarada` field of the card (stochology-core), which remains a textual field.

use stochology_core::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Options {
    /// How many initial points estimate the reference mean and deviation.
    pub base: usize,
    /// Slack in deviation units: below it the shift does not accumulate.
    pub slack: f64,
    /// Alarm threshold, also in deviation units.
    pub threshold: f64,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            base: 50,
            slack: 0.75,
            threshold: 10.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Surveillance {
    pub alarm: Option<usize>,
    pub max_cusum: f64,
    pub baseline_mean: f64,
    pub baseline_deviation: f64,
}

/// Two-sided CUSUM over the series, with the baseline estimated on the first points.
pub fn cusum(series: &[f64], options: &Options) -> Surveillance {
    let base = options.base.clamp(2, series.len().max(2));
    if series.len() <= base {
        return Surveillance {
            alarm: None,
            max_cusum: 0.0,
            baseline_mean: if series.is_empty() {
                0.0
            } else {
                series.iter().sum::<f64>() / series.len() as f64
            },
            baseline_deviation: 0.0,
        };
    }
    let mean = series[..base].iter().sum::<f64>() / base as f64;
    let variance = series[..base]
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / (base as f64 - 1.0);
    let deviation = variance.sqrt().max(1e-9);
    let mut up = 0.0_f64;
    let mut down = 0.0_f64;
    let mut largest = 0.0_f64;
    for (i, x) in series.iter().enumerate().skip(base) {
        up = (up + (x - mean) / deviation - options.slack).max(0.0);
        down = (down + (mean - x) / deviation - options.slack).max(0.0);
        let current = up.max(down);
        if current > largest {
            largest = current;
        }
        if current > options.threshold {
            return Surveillance {
                alarm: Some(i),
                max_cusum: largest,
                baseline_mean: mean,
                baseline_deviation: deviation,
            };
        }
    }
    Surveillance {
        alarm: None,
        max_cusum: largest,
        baseline_mean: mean,
        baseline_deviation: deviation,
    }
}

/// Kullback-Leibler divergence between two declared distributions, with a floor (never 0 nor 1).
pub fn kl(p: &[f64], q: &[f64], floor: f64) -> f64 {
    if p.len() != q.len() || p.is_empty() {
        return f64::NAN;
    }
    let normalize = |values: &[f64]| -> Vec<f64> {
        let clamped: Vec<f64> = values.iter().map(|v| v.max(floor)).collect();
        let total: f64 = clamped.iter().sum();
        clamped.iter().map(|v| v / total).collect()
    };
    let p = normalize(p);
    let q = normalize(q);
    p.iter().zip(q.iter()).map(|(a, b)| a * (a / b).ln()).sum()
}

/// The declared null: stationary Gaussian noise, with no drift at all.
pub fn stationary_null(seed: u64, n: usize, mean: f64, deviation: f64) -> Vec<f64> {
    let mut rng = Rng::new(seed);
    (0..n)
        .map(|_| {
            let u1 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            let u2 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            mean + deviation * (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
        })
        .collect()
}

/// Plants a jump in the series, starting at `at`.
pub fn add_jump(series: &mut [f64], at: usize, jump: f64) {
    if at >= series.len() {
        return;
    }
    for value in series[at..].iter_mut() {
        *value += jump;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_null_does_not_fire_in_one_hundred_thousand_points() {
        let options = Options::default();
        let mut alarms = 0;
        for seed in 0..50 {
            let series = stationary_null(seed, 2_000, 10.0, 2.0);
            if cusum(&series, &options).alarm.is_some() {
                alarms += 1;
            }
        }
        assert_eq!(
            alarms, 0,
            "the stationary null does not fire: {alarms} alarms"
        );
    }

    #[test]
    fn the_jump_fires_the_alarm_after_it_happens() {
        let options = Options::default();
        let mut series = stationary_null(2026, 1_500, 0.0, 1.0);
        let at = 700;
        add_jump(&mut series, at, 1.5);
        let surveillance = cusum(&series, &options);
        match surveillance.alarm {
            Some(i) => {
                assert!(i >= at, "alarm before the jump: {i} against {at}");
                assert!(i - at < 60, "late alarm: {i} against {at}");
            }
            None => panic!("a 1.5 sigma jump was not seen: {surveillance:?}"),
        }
        assert!(surveillance.max_cusum > options.threshold);
    }

    #[test]
    fn the_downward_jump_also_fires() {
        let mut series = stationary_null(31, 1_200, 5.0, 1.0);
        add_jump(&mut series, 600, -2.0);
        let surveillance = cusum(&series, &Options::default());
        assert!(
            surveillance.alarm.map(|i| i >= 600).unwrap_or(false),
            "{surveillance:?}"
        );
    }

    #[test]
    fn kl_is_zero_against_itself_and_grows_with_distance() {
        let p = [0.25, 0.25, 0.25, 0.25];
        let near = [0.26, 0.24, 0.25, 0.25];
        let far = [0.7, 0.1, 0.1, 0.1];
        let zero = kl(&p, &p, 1e-6);
        assert!(zero.abs() < 1e-9, "kl of itself: {zero}");
        assert!(kl(&p, &near, 1e-6) < kl(&p, &far, 1e-6));
        assert!(kl(&p, &far, 1e-6) > 0.4);
    }

    #[test]
    fn surveillance_is_deterministic() {
        let series = stationary_null(99, 800, 0.0, 1.0);
        let options = Options::default();
        assert_eq!(cusum(&series, &options), cusum(&series, &options));
        let other = stationary_null(100, 800, 0.0, 1.0);
        assert_ne!(series, other);
    }
}
