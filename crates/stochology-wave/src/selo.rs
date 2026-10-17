//! Selo deterministico: quantizacao canonica de ponto flutuante e hash FNV-1a de 64 bits.
//!
//! O ponto flutuante nao e comparavel bit a bit entre arquiteturas quando o ultimo bit da
//! mantissa se move. A quantizacao em grade fixa de 10^-9 remove esse ruido e o -0.0, de modo
//! que o mesmo estado logico produz o mesmo selo em qualquer maquina.

/// Grade canonica de quantizacao: 10^-9.
pub const ESCALA: f64 = 1_000_000_000.0;

/// Abaixo deste modulo o valor e tratado como zero exato (mata -0.0 e poeira de arredondamento).
pub const ZERO_CANONICO: f64 = 1e-12;

/// Quantiza um `f64` para inteiro na grade canonica.
pub fn quantiza(v: f64) -> i64 {
    if !v.is_finite() {
        return i64::MIN;
    }
    let limpo = if v.abs() < ZERO_CANONICO { 0.0 } else { v };
    (limpo * ESCALA).round() as i64
}

/// Envolve um angulo em (-pi, pi].
pub fn envolve_pi(theta: f64) -> f64 {
    let mut a = (theta + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
    if a <= 0.0 {
        a += 2.0 * std::f64::consts::PI;
    }
    a - std::f64::consts::PI
}

/// Envolve o potencial magnetico em (-1, 1].
pub fn envolve_alfa(alfa: f64) -> f64 {
    let mut a = (alfa + 1.0) % 2.0;
    if a <= 0.0 {
        a += 2.0;
    }
    a - 1.0
}

/// Semente da cadeia de selos do motor.
pub const SEMENTE_CADEIA: u64 = 0x5354_4F43_484F_4C4F; // "STOCHOLO"

#[derive(Debug, Clone)]
pub struct Selo {
    estado: u64,
}

impl Default for Selo {
    fn default() -> Self {
        Self::novo()
    }
}

impl Selo {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIMO: u64 = 0x0000_0100_0000_01b3;

    pub fn novo() -> Self {
        Self {
            estado: Self::OFFSET,
        }
    }

    pub fn escreve_u64(&mut self, v: u64) {
        for b in v.to_le_bytes() {
            self.estado ^= b as u64;
            self.estado = self.estado.wrapping_mul(Self::PRIMO);
        }
    }

    pub fn escreve_texto(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.estado ^= *b as u64;
            self.estado = self.estado.wrapping_mul(Self::PRIMO);
        }
    }

    /// Escreve o valor quantizado; NaN e infinito viram o sentinela `i64::MIN`.
    pub fn escreve_f64(&mut self, v: f64) {
        self.escreve_u64(quantiza(v) as u64);
    }

    pub fn estado(&self) -> u64 {
        self.estado
    }

    pub fn fecha(&self) -> u64 {
        self.estado
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn zero_e_negativo_zero_selam_igual() {
        let mut a = Selo::novo();
        let mut b = Selo::novo();
        a.escreve_f64(0.0);
        b.escreve_f64(-0.0);
        assert_eq!(a.fecha(), b.fecha());
    }

    #[test]
    fn poeira_abaixo_da_grade_nao_muda_o_selo() {
        let mut a = Selo::novo();
        let mut b = Selo::novo();
        a.escreve_f64(0.123456789);
        b.escreve_f64(0.123456789 + 1e-13);
        assert_eq!(a.fecha(), b.fecha());
    }

    #[test]
    fn mudanca_de_milesimo_muda_o_selo() {
        let mut a = Selo::novo();
        let mut b = Selo::novo();
        a.escreve_f64(0.5);
        b.escreve_f64(0.501);
        assert_ne!(a.fecha(), b.fecha());
    }

    #[test]
    fn envolve_pi_em_antiffase_fica_em_pi_ou_menos_pi() {
        let a = envolve_pi(std::f64::consts::PI);
        assert!(a.abs() <= std::f64::consts::PI + 1e-12);
        let b = envolve_pi(3.0 * std::f64::consts::PI);
        assert!(b.abs() <= std::f64::consts::PI + 1e-12);
    }

    #[test]
    fn envolve_alfa_fica_no_intervalo() {
        for k in -7..7 {
            let a = envolve_alfa(k as f64 * 0.73);
            assert!(a > -1.0 - 1e-12 && a <= 1.0 + 1e-12, "alfa fora: {a}");
        }
    }
}
