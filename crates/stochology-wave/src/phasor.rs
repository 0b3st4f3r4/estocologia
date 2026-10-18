//! Local superposition operator: the Phi of a hyperedge, evaluated as a phasor.
//!
//! Each vertex enters with amplitude, phase, topological conductance, and propagation delay. The
//! hyperedge does not add scalars: it adds phasors. The resulting energy has two terms, the
//! incoherent sum of the energies and the cross interference term, and the coherence index
//! decides which of the two deliberation regimes answers.

use crate::complex::Complex;

/// Thresholds of the cognitive gate of a hyperedge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thresholds {
    /// Below this there is not enough signal: quiescent regime.
    pub activation: f64,
    /// Coherence from which the hyperedge propagates directly (System 1).
    pub resonance: f64,
    /// Coherence below which, given energy, the hyperedge opens System 2.
    pub critical: f64,
    /// Minimum sum of amplitudes for the antiphase to count as conflict and not as noise.
    pub critical_sum: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            activation: 0.15,
            resonance: 0.85,
            critical: 0.35,
            critical_sum: 0.80,
        }
    }
}

/// Deliberation regime triggered by the hyperedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regime {
    /// Insufficient signal.
    Quiescent,
    /// Aligned phases: direct propagation through System 1.
    Resonance,
    /// Nearby phases without alignment: beating, monitor Chrononomy.
    Beating,
    /// Energy present but cancelled by antiphase: open the hyperedge in System 2.
    Conflict,
}

impl Regime {
    pub fn name(self) -> &'static str {
        match self {
            Regime::Quiescent => "quiescent",
            Regime::Resonance => "resonance",
            Regime::Beating => "beating",
            Regime::Conflict => "conflict",
        }
    }
}

/// One leg of the hyperedge: the state of the vertex already transported to the hyperedge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coupling {
    pub amplitude: f64,
    pub phase: f64,
    pub conductance: f64,
    pub delay: f64,
}

impl Coupling {
    pub fn new(amplitude: f64, phase: f64, conductance: f64, delay: f64) -> Self {
        Self {
            amplitude,
            phase,
            conductance,
            delay,
        }
    }

    /// Effective amplitude after conduction.
    pub fn effective(&self) -> f64 {
        self.conductance * self.amplitude
    }

    /// Effective phase after the propagation delay.
    pub fn effective_phase(&self) -> f64 {
        self.phase + self.delay
    }

    pub fn phasor(&self) -> Complex {
        Complex::polar(self.effective(), self.effective_phase())
    }
}

/// Complete diagnostic of the hyperedge at one instant.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub phasor: Complex,
    /// E(h) = |Psi_h|^2, the observable energy.
    pub energy: f64,
    /// Incoherent sum of the energies, without the cross term.
    pub classical_energy: f64,
    /// Interference term: energy minus classical energy.
    pub interference: f64,
    /// C(h) = |Psi_h| / sum of the effective amplitudes, in [0, 1].
    pub coherence: f64,
    pub amplitude_sum: f64,
    pub regime: Regime,
}

/// Evaluates the superposition operator of a hyperedge.
pub fn superpose(couplings: &[Coupling], thresholds: &Thresholds) -> Field {
    let mut phasor = Complex::ZERO;
    let mut sum = 0.0;
    let mut classical = 0.0;

    for a in couplings {
        phasor = phasor + a.phasor();
        let amp = a.effective();
        sum += amp;
        classical += amp * amp;
    }

    let energy = phasor.modulus_sqr();
    let coherence = if sum > 1e-12 {
        (phasor.modulus() / sum).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let regime = if sum < thresholds.activation {
        Regime::Quiescent
    } else if coherence >= thresholds.resonance {
        Regime::Resonance
    } else if coherence <= thresholds.critical && sum >= thresholds.critical_sum {
        Regime::Conflict
    } else {
        Regime::Beating
    };

    Field {
        phasor,
        energy,
        classical_energy: classical,
        interference: energy - classical,
        coherence,
        amplitude_sum: sum,
        regime,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn pair(phase_a: f64, phase_b: f64, amp: f64) -> Vec<Coupling> {
        vec![
            Coupling::new(amp, phase_a, 1.0, 0.0),
            Coupling::new(amp, phase_b, 1.0, 0.0),
        ]
    }

    #[test]
    fn in_phase_there_is_superadditive_gain() {
        let c = superpose(&pair(0.10, 0.15, 0.8), &Thresholds::default());
        assert!(
            c.energy > 1.8 * c.classical_energy,
            "energy {} classical {}",
            c.energy,
            c.classical_energy
        );
        assert!(c.interference > 0.0);
        assert_eq!(c.regime, Regime::Resonance);
    }

    #[test]
    fn in_antiphase_the_energy_collapses_and_opens_system_2() {
        let c = superpose(&pair(0.0, PI * 0.96, 0.85), &Thresholds::default());
        assert!(c.energy < 0.05, "energy {}", c.energy);
        assert!(c.interference < 0.0);
        assert_eq!(c.regime, Regime::Conflict);
    }

    #[test]
    fn weak_signal_stays_quiescent() {
        let c = superpose(&pair(0.0, 0.0, 0.05), &Thresholds::default());
        assert_eq!(c.regime, Regime::Quiescent);
        assert!((c.coherence - 1.0).abs() < 1e-12);
    }

    #[test]
    fn propagation_delay_enters_the_phase() {
        let mut cs = pair(0.0, 0.0, 0.5);
        cs[1].delay = PI;
        let c = superpose(&cs, &Thresholds::default());
        assert!(c.energy < 1e-12);
    }
}
