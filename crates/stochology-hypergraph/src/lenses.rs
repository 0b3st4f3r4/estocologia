//! Engine of the five operational lenses.

use stochology_core::Rng;

/// Fluxonomy: empirical mutual information I(X;Y), in bits.
pub fn mutual_information(x: &[usize], y: &[usize]) -> f64 {
    assert_eq!(x.len(), y.len(), "samples of different lengths");
    let n = x.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mut total = 0.0;
    let mut xs: Vec<usize> = x.to_vec();
    let mut ys: Vec<usize> = y.to_vec();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    for &a in &xs {
        for &b in &ys {
            let n_xy = x
                .iter()
                .zip(y.iter())
                .filter(|(p, q)| **p == a && **q == b)
                .count() as f64;
            let n_x = x.iter().filter(|p| **p == a).count() as f64;
            let n_y = y.iter().filter(|q| **q == b).count() as f64;
            if n_xy > 0.0 {
                total += (n_xy / n) * ((n_xy * n) / (n_x * n_y)).log2();
            }
        }
    }
    total
}

/// Data-processing inequality: I(X;Z) <= I(X;Y) when Z comes out of Y.
pub fn fluxonomy_does_not_increase(x: &[usize], y: &[usize], z: &[usize]) -> bool {
    mutual_information(x, z) <= mutual_information(x, y) + 1e-9
}

/// Chrononomy: volatility drag, mu - sigma^2/2.
pub fn volatility_drag(mu: f64, sigma: f64) -> f64 {
    mu - 0.5 * sigma * sigma
}

/// Chrononomy: Peters' coin grows on the ensemble average and dies in time.
pub fn non_ergodic(gain: f64, loss: f64) -> bool {
    (1.0 + gain) * (1.0 - loss) < 1.0
}

/// Chrononomy: multiplicative simulation with a fixed seed.
/// Returns (median of the wealth log, fraction of ruined paths).
pub fn simulate_multiplicative(
    seed: u64,
    steps: usize,
    paths: usize,
    gain: f64,
    loss: f64,
) -> (f64, f64) {
    let mut rng = Rng::new(seed);
    let mut logs: Vec<f64> = Vec::with_capacity(paths);
    let mut ruined = 0usize;
    let threshold = 0.1_f64.ln();
    for _ in 0..paths {
        let mut log_wealth = 0.0_f64;
        for _ in 0..steps {
            if rng.uniform(2) == 0 {
                log_wealth += (1.0 + gain).ln();
            } else {
                log_wealth += (1.0 - loss).ln();
            }
        }
        if log_wealth < threshold {
            ruined += 1;
        }
        logs.push(log_wealth);
    }
    logs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = logs[logs.len() / 2];
    (median, ruined as f64 / paths as f64)
}

/// Scalonomy: Hill estimator over the upper tail.
pub fn hill_index(values: &[f64], k: usize) -> f64 {
    stochology_core::hill_at_cutoff(values, k, k)
}

/// Scalonomy: wild tail when the exponent drops below 2.
pub fn wild_tail(alpha: f64) -> bool {
    alpha < 2.0
}

/// Morphonomy: least-squares slope.
pub fn slope(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let num: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x - mx) * (y - my))
        .sum();
    let den: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    num / den
}

/// Morphonomy: the same rule survives when the regime changes.
pub fn invariant_across_regimes(slopes: &[f64], tolerance: f64) -> bool {
    let largest = slopes.iter().cloned().fold(f64::MIN, f64::max);
    let smallest = slopes.iter().cloned().fold(f64::MAX, f64::min);
    (largest - smallest).abs() <= tolerance
}

/// Crisonomy: Brier score.
pub fn brier_score(forecasts: &[f64], outcomes: &[f64]) -> f64 {
    let n = forecasts.len() as f64;
    forecasts
        .iter()
        .zip(outcomes.iter())
        .map(|(p, o)| (p - o) * (p - o))
        .sum::<f64>()
        / n
}

/// Crisonomy: Murphy decomposition (REL, RES, UNC).
///
/// Groups by exact forecast: for binary outcomes the identity
/// Brier = REL - RES + UNC closes byte for byte.
pub fn murphy_decomposition(forecasts: &[f64], outcomes: &[f64]) -> (f64, f64, f64) {
    stochology_core::murphy_decomposition(forecasts, outcomes)
}

/// Crisonomy: expected value of perfect information.
///
/// payoffs is the payoff of the best act in each state; best_without_info is the
/// best expected payoff before looking at the data.
pub fn evpi(payoffs: &[f64], probabilities: &[f64], best_without_info: f64) -> f64 {
    let with_info: f64 = payoffs
        .iter()
        .zip(probabilities.iter())
        .map(|(p, w)| p * w)
        .sum();
    with_info - best_without_info
}

/// Crisonomy: expected value of perfect information from the payoff table itself, one row per act
/// and one column per state: E_state[max_act payoff] - max_act E_state[payoff]. Returns
/// (best expected payoff without information, expected payoff with perfect information, EVPI).
///
/// Prefer this to `evpi`, whose caller must pass the best payoff of each state; on 2026-10-19 a
/// caller passed the payoffs of one act instead, and the EVPI of Chapter 12 came out as zero.
pub fn evpi_from_actions(actions: &[Vec<f64>], probabilities: &[f64]) -> (f64, f64, f64) {
    let expected =
        |row: &Vec<f64>| -> f64 { row.iter().zip(probabilities).map(|(x, p)| x * p).sum() };
    let without = actions
        .iter()
        .map(expected)
        .fold(f64::NEG_INFINITY, f64::max);
    let with: f64 = probabilities
        .iter()
        .enumerate()
        .map(|(s, p)| {
            p * actions
                .iter()
                .map(|row| row[s])
                .fold(f64::NEG_INFINITY, f64::max)
        })
        .sum();
    (without, with, with - without)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_chain_respects_processing() {
        let x: Vec<usize> = (0..400).map(|i| i % 4).collect();
        let y: Vec<usize> = x.iter().map(|v| v / 2).collect();
        let z: Vec<usize> = y.iter().map(|v| (*v + 1) % 2).collect();
        assert!(fluxonomy_does_not_increase(&x, &y, &z));
    }

    #[test]
    fn peters_coin_is_not_ergodic() {
        assert!(non_ergodic(0.50, 0.40));
        let (median, ruined) = simulate_multiplicative(2026, 400, 2_000, 0.50, 0.40);
        assert!(median < 0.0, "median wealth should shrink: {}", median);
        assert!(ruined > 0.9, "ruined fraction: {}", ruined);
    }

    #[test]
    fn volatility_drag_discounts_variance() {
        assert!((volatility_drag(0.051, 0.0) - 0.051).abs() < 1e-12);
        assert!(volatility_drag(0.051, 0.2) < 0.051);
    }

    #[test]
    fn hill_recovers_pareto_alpha() {
        let mut rng = Rng::new(1909);
        let alpha = 1.81;
        let sample: Vec<f64> = (0..20_000)
            .map(|_| {
                let u = (rng.next_u64() as f64) / (u64::MAX as f64);
                (1.0_f64 - u).powf(-1.0 / alpha)
            })
            .collect();
        let h = hill_index(&sample, 500);
        assert!((h - alpha).abs() < 0.25, "Hill returned {}", h);
        assert!(wild_tail(h));
    }

    #[test]
    fn invariant_regime_approves_broken_regime() {
        assert!(invariant_across_regimes(&[2.00, 2.02, 1.99], 0.05));
        assert!(!invariant_across_regimes(&[2.00, 2.60], 0.05));
    }

    #[test]
    fn brier_and_murphy_close_the_account() {
        let prev = [0.1, 0.1, 0.2, 0.2, 0.8, 0.8, 0.9, 0.9];
        let res = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let brier = brier_score(&prev, &res);
        let (rel, resol, unc) = murphy_decomposition(&prev, &res);
        assert!((brier - 0.1).abs() < 1e-12, "brier {}", brier);
        assert!((brier - (rel - resol + unc)).abs() < 1e-12);
    }

    #[test]
    fn evpi_of_the_example_is_three() {
        // Two acts: A pays 10 in the first state and 0 in the second, B the reverse. The best
        // payoff of each state is 10, and the best act without information yields 0.7 * 10 = 7.
        let payoffs = [10.0, 10.0];
        let probabilities = [0.7, 0.3];
        assert!((evpi(&payoffs, &probabilities, 7.0) - 3.0).abs() < 1e-9);
        let (without, with, value) =
            evpi_from_actions(&[vec![10.0, 0.0], vec![0.0, 10.0]], &probabilities);
        assert!((without - 7.0).abs() < 1e-12 && (with - 10.0).abs() < 1e-12);
        assert!((value - 3.0).abs() < 1e-12);
    }

    #[test]
    fn the_sensor_of_chapter_12_is_worth_what_it_saves_in_bad_years() {
        // Act: +10 in a good year (0.7), -4 in a bad one; not acting: 0. Perfect information avoids
        // the bad years: 7.0 against 5.8, an EVPI of 1.2, not zero.
        let p = [0.7, 0.3];
        let (without, with, value) = evpi_from_actions(&[vec![10.0, -4.0], vec![0.0, 0.0]], &p);
        assert!((without - 5.8).abs() < 1e-12 && (with - 7.0).abs() < 1e-12);
        assert!((value - 1.2).abs() < 1e-12);
        // An act whose payoff does not depend on the state leaves nothing for the sensor to change.
        let (_, _, useless) = evpi_from_actions(&[vec![10.0, 10.0], vec![0.0, 0.0]], &p);
        assert!(useless.abs() < 1e-12);
    }

    #[test]
    fn simple_slope() {
        let xs = [1.0, 2.0, 3.0, 4.0];
        let ys = [3.0, 5.0, 7.0, 9.0];
        assert!((slope(&xs, &ys) - 2.0).abs() < 1e-9);
    }
}
