//! Aritmetica complexa minima do motor ondulatorio, sem dependencia externa.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complexo {
    pub re: f64,
    pub im: f64,
}

impl Complexo {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    pub const UM: Self = Self { re: 1.0, im: 0.0 };
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    pub fn novo(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Fasor de amplitude `r` e fase `theta`: r * e^{i theta}.
    pub fn polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn conj(self) -> Self {
        Self::novo(self.re, -self.im)
    }

    pub fn modulo2(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn modulo(self) -> f64 {
        self.modulo2().sqrt()
    }

    pub fn fase(self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn finito(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    pub fn escala(self, k: f64) -> Self {
        Self::novo(self.re * k, self.im * k)
    }

    /// Divisao complexa; `None` quando o divisor e degenerado.
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, rhs: Self) -> Option<Self> {
        let d = rhs.modulo2();
        if d < 1e-300 {
            return None;
        }
        Some(Self::novo(
            (self.re * rhs.re + self.im * rhs.im) / d,
            (self.im * rhs.re - self.re * rhs.im) / d,
        ))
    }
}

impl std::ops::Add for Complexo {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::novo(self.re + r.re, self.im + r.im)
    }
}

impl std::ops::Sub for Complexo {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::novo(self.re - r.re, self.im - r.im)
    }
}

impl std::ops::Neg for Complexo {
    type Output = Self;
    fn neg(self) -> Self {
        Self::novo(-self.re, -self.im)
    }
}

impl std::ops::Mul for Complexo {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::novo(
            self.re * r.re - self.im * r.im,
            self.re * r.im + self.im * r.re,
        )
    }
}

impl std::ops::Mul<f64> for Complexo {
    type Output = Self;
    fn mul(self, r: f64) -> Self {
        self.escala(r)
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn polar_e_fase_concordam() {
        let z = Complexo::polar(2.0, 0.5);
        assert!((z.modulo() - 2.0).abs() < 1e-12);
        assert!((z.fase() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn produto_por_conjugado_da_quadrado_do_modulo() {
        let z = Complexo::novo(0.6, -0.8);
        let p = z * z.conj();
        assert!((p.re - 1.0).abs() < 1e-12);
        assert!(p.im.abs() < 1e-12);
    }

    #[test]
    fn antiffase_soma_zero() {
        let a = Complexo::polar(0.9, 0.0);
        let b = Complexo::polar(0.9, std::f64::consts::PI);
        assert!((a + b).modulo() < 1e-12);
    }

    #[test]
    fn divisao_desfaz() {
        let a = Complexo::novo(1.0, 2.0);
        let b = Complexo::novo(3.0, -1.0);
        let q = a.div(b).expect("divisor nao degenerado");
        let volta = q * b;
        assert!((volta.re - a.re).abs() < 1e-12);
        assert!((volta.im - a.im).abs() < 1e-12);
    }
}
