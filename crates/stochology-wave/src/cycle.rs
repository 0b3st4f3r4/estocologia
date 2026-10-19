//! Phase as time: a cyclic profile reduced to one phasor.
//!
//! The superposition of `phasor` adds phasors, and a phasor whose phase means only "for" or "against" (0 or pi)
//! adds nothing a signed sum does not. Here the phase is a position in a cycle. A profile measured at positions of
//! a cycle of length `period` (hours of a day, months of a year) is reduced to its first harmonic: the amplitude is
//! how much the profile swings, and the phase is when it peaks. Two sources whose profiles peak at the same hour
//! superpose in resonance; two that peak half a cycle apart cancel.

use crate::complex::Complex;

/// One point of a cyclic profile: its position in the cycle, its value, and its weight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub position: f64,
    pub value: f64,
    pub weight: f64,
}

/// The first harmonic of a weighted profile, as `(amplitude, phase)`.
///
/// The weighted mean is removed first, so a constant profile has amplitude zero. With the profile
/// `m + A cos(2 pi (x - x0) / period)` sampled evenly over whole cycles, the result is `(A, 2 pi x0 / period)`.
/// Returns `(0, 0)` when there is no weight.
pub fn first_harmonic(points: &[Point], period: f64) -> (f64, f64) {
    let total: f64 = points.iter().map(|p| p.weight.max(0.0)).sum();
    if total <= 0.0 || period.is_nan() || period <= 0.0 {
        return (0.0, 0.0);
    }
    let mean = points
        .iter()
        .map(|p| p.weight.max(0.0) * p.value)
        .sum::<f64>()
        / total;
    let omega = 2.0 * std::f64::consts::PI / period;
    let mut c = Complex::ZERO;
    for p in points {
        let w = p.weight.max(0.0);
        c = c + Complex::polar(w * (p.value - mean), omega * p.position);
    }
    let c = c.scale(1.0 / total);
    (
        2.0 * c.modulus(),
        if c.modulus() > 0.0 { c.phase() } else { 0.0 },
    )
}

/// The phase of a position in a cycle, in (-pi, pi].
pub fn phase_of(position: f64, period: f64) -> f64 {
    crate::seal::wrap_pi(2.0 * std::f64::consts::PI * position / period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn profile(peak_hour: f64, amplitude: f64) -> Vec<Point> {
        (0..24)
            .map(|h| Point {
                position: h as f64,
                value: 3.0 + amplitude * (2.0 * PI * (h as f64 - peak_hour) / 24.0).cos(),
                weight: 1.0,
            })
            .collect()
    }

    #[test]
    fn a_cosine_peaking_at_eighteen_hours_is_recovered() {
        let (a, phi) = first_harmonic(&profile(18.0, 0.6), 24.0);
        assert!((a - 0.6).abs() < 1e-12, "{a}");
        assert!((phi - phase_of(18.0, 24.0)).abs() < 1e-12, "{phi}");
    }

    #[test]
    fn a_flat_profile_has_no_amplitude() {
        let (a, _) = first_harmonic(&profile(9.0, 0.0), 24.0);
        assert!(a < 1e-12);
        assert_eq!(first_harmonic(&[], 24.0), (0.0, 0.0));
    }

    #[test]
    fn profiles_half_a_day_apart_cancel_in_the_superposition() {
        use crate::phasor::{Coupling, Thresholds, superpose};
        let (a1, p1) = first_harmonic(&profile(6.0, 1.0), 24.0);
        let (a2, p2) = first_harmonic(&profile(18.0, 1.0), 24.0);
        let field = superpose(
            &[
                Coupling::new(a1, p1, 1.0, 0.0),
                Coupling::new(a2, p2, 1.0, 0.0),
            ],
            &Thresholds::default(),
        );
        assert!(field.coherence < 1e-9, "{}", field.coherence);
        let (b1, q1) = first_harmonic(&profile(17.0, 1.0), 24.0);
        let (b2, q2) = first_harmonic(&profile(18.0, 1.0), 24.0);
        let aligned = superpose(
            &[
                Coupling::new(b1, q1, 1.0, 0.0),
                Coupling::new(b2, q2, 1.0, 0.0),
            ],
            &Thresholds::default(),
        );
        assert!(aligned.coherence > 0.95, "{}", aligned.coherence);
    }
}
