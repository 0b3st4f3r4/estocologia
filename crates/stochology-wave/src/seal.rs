//! Deterministic seal: canonical floating-point quantization and 64-bit FNV-1a hash.
//!
//! Floating point is not comparable bit by bit across architectures when the last mantissa bit
//! moves. Quantization on a fixed grid of 10^-9 removes that noise and the -0.0, so that the
//! same logical state produces the same seal on any machine.

/// Canonical quantization grid: 10^-9.
pub const SCALE: f64 = 1_000_000_000.0;

/// Below this modulus the value is treated as exact zero (it kills -0.0 and rounding dust).
pub const CANONICAL_ZERO: f64 = 1e-12;

/// Quantizes an `f64` to an integer on the canonical grid.
pub fn quantize(v: f64) -> i64 {
    if !v.is_finite() {
        return i64::MIN;
    }
    let clean = if v.abs() < CANONICAL_ZERO { 0.0 } else { v };
    (clean * SCALE).round() as i64
}

/// Wraps an angle into (-pi, pi].
pub fn wrap_pi(theta: f64) -> f64 {
    let mut a = (theta + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
    if a <= 0.0 {
        a += 2.0 * std::f64::consts::PI;
    }
    a - std::f64::consts::PI
}

/// Wraps the magnetic potential into (-1, 1].
pub fn wrap_alpha(alpha: f64) -> f64 {
    let mut a = (alpha + 1.0) % 2.0;
    if a <= 0.0 {
        a += 2.0;
    }
    a - 1.0
}

/// Seed of the seal chain of the engine.
pub const CHAIN_SEED: u64 = 0x5354_4F43_484F_4C4F; // "STOCHOLO"

#[derive(Debug, Clone)]
pub struct Seal {
    state: u64,
}

impl Default for Seal {
    fn default() -> Self {
        Self::new()
    }
}

impl Seal {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    pub fn new() -> Self {
        Self {
            state: Self::OFFSET,
        }
    }

    pub fn write_u64(&mut self, v: u64) {
        for b in v.to_le_bytes() {
            self.state ^= b as u64;
            self.state = self.state.wrapping_mul(Self::PRIME);
        }
    }

    pub fn write_text(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.state ^= *b as u64;
            self.state = self.state.wrapping_mul(Self::PRIME);
        }
    }

    /// Writes the quantized value; NaN and infinity become the sentinel `i64::MIN`.
    pub fn write_f64(&mut self, v: f64) {
        self.write_u64(quantize(v) as u64);
    }

    pub fn state(&self) -> u64 {
        self.state
    }

    pub fn finish(&self) -> u64 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_and_negative_zero_seal_alike() {
        let mut a = Seal::new();
        let mut b = Seal::new();
        a.write_f64(0.0);
        b.write_f64(-0.0);
        assert_eq!(a.finish(), b.finish());
    }

    #[test]
    fn dust_below_the_grid_does_not_change_the_seal() {
        let mut a = Seal::new();
        let mut b = Seal::new();
        a.write_f64(0.123456789);
        b.write_f64(0.123456789 + 1e-13);
        assert_eq!(a.finish(), b.finish());
    }

    #[test]
    fn a_thousandth_of_change_changes_the_seal() {
        let mut a = Seal::new();
        let mut b = Seal::new();
        a.write_f64(0.5);
        b.write_f64(0.501);
        assert_ne!(a.finish(), b.finish());
    }

    #[test]
    fn wrap_pi_in_antiphase_stays_at_pi_or_minus_pi() {
        let a = wrap_pi(std::f64::consts::PI);
        assert!(a.abs() <= std::f64::consts::PI + 1e-12);
        let b = wrap_pi(3.0 * std::f64::consts::PI);
        assert!(b.abs() <= std::f64::consts::PI + 1e-12);
    }

    #[test]
    fn wrap_alpha_stays_in_the_interval() {
        for k in -7..7 {
            let a = wrap_alpha(k as f64 * 0.73);
            assert!(a > -1.0 - 1e-12 && a <= 1.0 + 1e-12, "alpha out: {a}");
        }
    }
}
