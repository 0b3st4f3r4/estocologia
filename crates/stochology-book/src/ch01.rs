//! Chapter 1 — the invisible target: why the fixed seed is the basis of the check.

use stochology_core::Rng;

/// A machine that draws: same seed, same sequence.
pub fn machine(seed: u64, n: usize) -> Vec<u64> {
    let mut rng = Rng::new(seed);
    (0..n).map(|_| rng.uniform(1_000_000)).collect()
}

/// Two independent machines with the same seed agree draw by draw.
pub fn two_machines_agree(seed: u64, n: usize) -> bool {
    machine(seed, n) == machine(seed, n)
}

/// How many of the first n draws coincide when the seed changes.
pub fn matches_with_another_seed(seed: u64, n: usize) -> usize {
    let a = machine(seed, n);
    let b = machine(seed + 1, n);
    a.iter().zip(b.iter()).filter(|(x, y)| x == y).count()
}

/// Frequency of heads in the first n tosses of an honest coin.
pub fn frequency_of_heads(seed: u64, n: usize) -> f64 {
    let mut rng = Rng::new(seed);
    let heads = (0..n).filter(|_| rng.uniform(2) == 0).count();
    heads as f64 / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_machines_agree_draw_by_draw() {
        assert!(two_machines_agree(1713, 10_000));
    }

    #[test]
    fn different_seeds_almost_never_match() {
        let c = matches_with_another_seed(1713, 100_000);
        assert!(c < 200, "matches: {}", c);
    }

    #[test]
    fn honest_coin_concentrates_near_half() {
        for seed in [1713, 1909, 2026] {
            let f = frequency_of_heads(seed, 100_000);
            assert!((f - 0.5).abs() < 0.01, "frequency: {}", f);
        }
    }
}
