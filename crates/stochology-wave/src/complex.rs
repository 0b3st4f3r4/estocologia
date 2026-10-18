//! Minimal complex arithmetic of the wave engine, with no external dependency.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Phasor of amplitude `r` and phase `theta`: r * e^{i theta}.
    pub fn polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }

    pub fn modulus_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn modulus(self) -> f64 {
        self.modulus_sqr().sqrt()
    }

    pub fn phase(self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    pub fn scale(self, k: f64) -> Self {
        Self::new(self.re * k, self.im * k)
    }

    /// Complex division; `None` when the divisor is degenerate.
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, rhs: Self) -> Option<Self> {
        let d = rhs.modulus_sqr();
        if d < 1e-300 {
            return None;
        }
        Some(Self::new(
            (self.re * rhs.re + self.im * rhs.im) / d,
            (self.im * rhs.re - self.re * rhs.im) / d,
        ))
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.re + r.re, self.im + r.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.re - r.re, self.im - r.im)
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::new(
            self.re * r.re - self.im * r.im,
            self.re * r.im + self.im * r.re,
        )
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, r: f64) -> Self {
        self.scale(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polar_and_phase_agree() {
        let z = Complex::polar(2.0, 0.5);
        assert!((z.modulus() - 2.0).abs() < 1e-12);
        assert!((z.phase() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn product_with_conjugate_is_modulus_squared() {
        let z = Complex::new(0.6, -0.8);
        let p = z * z.conj();
        assert!((p.re - 1.0).abs() < 1e-12);
        assert!(p.im.abs() < 1e-12);
    }

    #[test]
    fn antiphase_sums_to_zero() {
        let a = Complex::polar(0.9, 0.0);
        let b = Complex::polar(0.9, std::f64::consts::PI);
        assert!((a + b).modulus() < 1e-12);
    }

    #[test]
    fn division_undoes() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, -1.0);
        let q = a.div(b).expect("non-degenerate divisor");
        let back = q * b;
        assert!((back.re - a.re).abs() < 1e-12);
        assert!((back.im - a.im).abs() < 1e-12);
    }
}
