//! Deterministic pseudo-random number generator (`SplitMix64`, Invariant $\Phi_1$)
//! and order-statistic / sample moment primitives.

/// Canonical deterministic seed mandated by Invariant $\Phi_1$ (`seed = 2026`).
pub const DEFAULT_SEED: u64 = 2026;

/// Deterministic 64-bit `SplitMix64` generator with zero external dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitMix64 {
    state: u64,
}

/// Alias used across the workspace.
pub type Rng = SplitMix64;

impl SplitMix64 {
    /// Creates a new generator initialized with `seed`.
    #[inline]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Creates a new generator initialized with [`DEFAULT_SEED`] (`2026`).
    #[inline]
    pub const fn with_default_seed() -> Self {
        Self::new(DEFAULT_SEED)
    }

    /// Returns the next deterministic `u64`.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Returns a uniform sample in $[0, 1)$ with 53 bits of mantissa precision.
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        let bits = self.next_u64() >> 11;
        (bits as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    /// Returns a uniform sample in $[low, high)$.
    #[inline]
    pub fn uniform(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.next_f64()
    }

    /// Returns a standard normal sample $\mathcal{N}(0, 1)$ via the Box-Muller transform.
    #[inline]
    pub fn standard_normal(&mut self) -> f64 {
        let u1 = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }

    /// Returns a normal sample $\mathcal{N}(\mu, \sigma^2)$.
    #[inline]
    pub fn normal(&mut self, mean: f64, std_dev: f64) -> f64 {
        mean + std_dev * self.standard_normal()
    }

    /// Returns a Bernoulli trial with success probability $p \in [0, 1]$.
    #[inline]
    pub fn bernoulli(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// Returns a Pareto sample $X = x_m (1 - U)^{-1/\alpha}$ with scale $x_m > 0$ and tail index $\alpha > 0$.
    #[inline]
    pub fn pareto(&mut self, x_m: f64, alpha: f64) -> f64 {
        let u = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        x_m * (1.0 - u).powf(-1.0 / alpha)
    }

    /// In-place Fisher-Yates shuffle.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let n = slice.len();
        for i in (1..n).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

/// Arithmetic mean of a non-empty slice.
pub fn mean(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "mean requires a non-empty slice");
    xs.iter().sum::<f64>() / (xs.len() as f64)
}

/// Unbiased sample variance ($n - 1$ denominator) of a slice with at least 2 elements.
pub fn sample_variance(xs: &[f64]) -> f64 {
    assert!(
        xs.len() >= 2,
        "sample_variance requires at least 2 observations"
    );
    let m = mean(xs);
    let ss: f64 = xs
        .iter()
        .map(|&x| {
            let d = x - m;
            d * d
        })
        .sum();
    ss / ((xs.len() - 1) as f64)
}

/// Sample standard deviation.
#[inline]
pub fn sample_std(xs: &[f64]) -> f64 {
    sample_variance(xs).sqrt()
}

/// Empirical quantile $q \in [0, 1]$ with linear interpolation.
pub fn quantile(xs: &[f64], q: f64) -> f64 {
    assert!(!xs.is_empty(), "quantile requires a non-empty slice");
    assert!((0.0..=1.0).contains(&q), "quantile q must be in [0, 1]");
    let mut sorted = xs.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    quantile_sorted(&sorted, q)
}

/// Empirical quantile $q \in [0, 1]$ on an already-sorted slice.
pub fn quantile_sorted(sorted: &[f64], q: f64) -> f64 {
    assert!(
        !sorted.is_empty(),
        "quantile_sorted requires a non-empty slice"
    );
    assert!((0.0..=1.0).contains(&q), "quantile q must be in [0, 1]");
    if sorted.len() == 1 {
        return sorted[0];
    }
    let max_idx = sorted.len() - 1;
    let pos = (q * (max_idx as f64)).clamp(0.0, max_idx as f64);
    let lo = (pos.floor() as usize).min(max_idx);
    let hi = (pos.ceil() as usize).min(max_idx);
    let frac = pos - (lo as f64);
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

/// Sample median ($q_{50}$).
#[inline]
pub fn median(xs: &[f64]) -> f64 {
    quantile(xs, 0.5)
}

/// Median Absolute Deviation: $\mathrm{MAD} = \mathrm{median}(|X - \mathrm{median}(X)|)$.
pub fn median_absolute_deviation(xs: &[f64]) -> f64 {
    let med = median(xs);
    let devs: Vec<f64> = xs.iter().map(|&x| (x - med).abs()).collect();
    median(&devs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_is_strictly_deterministic() {
        let mut r1 = Rng::with_default_seed();
        let mut r2 = Rng::new(2026);
        for _ in 0..32 {
            assert_eq!(r1.next_u64(), r2.next_u64());
        }
    }

    #[test]
    fn basic_order_statistics_hold() {
        let xs = [1.0, 2.0, 3.0, 4.0, 100.0];
        assert_eq!(median(&xs), 3.0);
        assert_eq!(median_absolute_deviation(&xs), 1.0);
        assert!((mean(&xs) - 22.0).abs() < 1e-12);
    }
}
