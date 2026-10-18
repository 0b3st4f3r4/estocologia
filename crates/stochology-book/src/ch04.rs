//! Chapter 4 — luck, ignorance and the turkey: a coin whose bias changes slowly.

use stochology_core::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coverage {
    pub steps: usize,
    pub repetitions: usize,
    pub discount: f64,
    /// Fraction of steps in which the 95% interval of the fixed model covers the truth.
    pub static_coverage: f64,
    /// Fraction of steps in which the interval of the model with a forgetting factor covers the truth.
    pub discounted_coverage: f64,
    pub static_width_100: f64,
    pub static_width_final: f64,
    pub discounted_width_final: f64,
}

/// Bias that climbs from 0.2 to 0.6 along the steps, with a reproducible draw.
pub fn bias_at(step: usize, steps: usize) -> f64 {
    0.2 + 0.4 * (step as f64 / steps as f64)
}

fn width(p: f64, effective_n: f64) -> f64 {
    if effective_n <= 1.0 {
        return 1.0;
    }
    2.0 * 1.96 * (p * (1.0 - p) / effective_n).sqrt()
}

/// Runs the world that changes and measures the coverage of both models.
///
/// Book configuration: 6,400 steps, 200 repetitions, discount 0.99.
/// Instrument: ~cargo run --release --bin labs -- --chapter 4~
pub fn moving_coin(seed: u64, steps: usize, repetitions: usize, discount: f64) -> Coverage {
    let mut rng = Rng::new(seed);
    let mut static_covers = 0usize;
    let mut discounted_covers = 0usize;
    let mut counted = 0usize;
    let mut static_width_100 = 0.0;
    let mut static_width_final = 0.0;
    let mut discounted_width_final = 0.0;
    let effective_n = (1.0 + discount) / (1.0 - discount);

    for _ in 0..repetitions {
        let mut heads = 0.0_f64;
        let mut sum = 0.0_f64;
        let mut weight = 0.0_f64;
        for step in 1..=steps {
            let truth = bias_at(step, steps);
            let x = if (rng.next_u64() as f64) / (u64::MAX as f64) < truth {
                1.0
            } else {
                0.0
            };
            heads += x;

            let p_static = heads / step as f64;
            let l_static = width(p_static, step as f64);

            sum = discount * sum + x;
            weight = discount * weight + 1.0;
            let p_discounted = sum / weight;
            let l_discounted = width(p_discounted, effective_n);

            if step >= 100 {
                counted += 1;
                if (p_static - truth).abs() <= l_static / 2.0 {
                    static_covers += 1;
                }
                if (p_discounted - truth).abs() <= l_discounted / 2.0 {
                    discounted_covers += 1;
                }
            }
            if step == 100 {
                static_width_100 += l_static;
            }
            if step == steps {
                static_width_final += l_static;
                discounted_width_final += l_discounted;
            }
        }
    }

    let r = repetitions as f64;
    let c = counted as f64;
    Coverage {
        steps,
        repetitions,
        discount,
        static_coverage: static_covers as f64 / c,
        discounted_coverage: discounted_covers as f64 / c,
        static_width_100: static_width_100 / r,
        static_width_final: static_width_final / r,
        discounted_width_final: discounted_width_final / r,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fixed_model_is_almost_always_wrong_and_the_forgetting_model_covers() {
        let c = moving_coin(2026, 6_400, 200, 0.99);
        assert!(
            c.static_coverage < 0.2,
            "static coverage: {}",
            c.static_coverage
        );
        assert!(
            c.discounted_coverage > 0.90,
            "discounted coverage: {}",
            c.discounted_coverage
        );
    }

    #[test]
    fn the_fixed_interval_shrinks_and_the_discounted_one_stays_wide() {
        let c = moving_coin(2026, 6_400, 200, 0.99);
        assert!(c.static_width_final < c.static_width_100);
        assert!(c.discounted_width_final > c.static_width_final);
    }

    #[test]
    fn the_bias_moves_from_two_tenths_to_six_tenths() {
        assert!((bias_at(0, 3_200) - 0.2).abs() < 1e-12);
        assert!((bias_at(3_200, 3_200) - 0.6).abs() < 1e-12);
    }
}
