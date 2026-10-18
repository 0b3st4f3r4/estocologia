//! Deterministic SplitMix64 generator (fixed seed, no external dependencies).

/// One SplitMix64 step over the mutable state: the only place where the mixing constants live.
pub fn splitmix64_step(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        splitmix64_step(&mut self.state)
    }

    /// Integer in [0, limit).
    pub fn uniform(&mut self, limit: u64) -> u64 {
        if limit == 0 {
            return 0;
        }
        self.next_u64() % limit
    }

    /// Heavy-tail magnitude in the range [start, end) in hundredths.
    pub fn cents(&mut self, start: u64, end: u64) -> u64 {
        start + self.uniform(end.saturating_sub(start).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::new(2026);
        let mut b = Rng::new(2026);
        for _ in 0..64 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_diverge() {
        let mut a = Rng::new(1);
        let mut b = Rng::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }
}
