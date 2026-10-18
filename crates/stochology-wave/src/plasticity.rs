//! Kuramoto-Oja plasticity: System 2 scars the hypergraph after resolving a conflict.
//!
//! The rule has two coupled motions. The first is a phase torque: the magnetic angle
//! alpha_{vh} rotates in the direction that reduces the error between the arrival phase and the
//! phase that the System 2 verdict validated. The second is Hebbian with Oja regularization: the
//! conductance eta_{vh} grows when the arrivals are in phase and decays under the quadratic term,
//! which prevents weight explosion without an arbitrary cut.
//!
//! Gate (divergence declared with respect to the original draft): consolidation requires **target
//! resonance gain**, not a monotone drop of frustration. The frustration <Psi, L Psi> is not
//! monotone under plasticity that creates coupling -- in the symmetric antiphase case it equals
//! zero and rises when the bond comes into existence. It is reported in the commit as a
//! diagnostic; what the gate charges is that the same input blow arrives stronger at the target,
//! with the norm preserved.

use crate::complex::Complex;
use crate::magnetic::{MagneticHypergraph, Propagation, arrival_phase};
use crate::seal::{Seal, wrap_alpha};

/// Hyperparameters of the plasticity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlasticityConfig {
    /// mu_alpha: step of the phase torque.
    pub lr_alpha: f64,
    /// mu_eta: step of the conductance.
    pub lr_eta: f64,
    /// beta_oja: quadratic regularization.
    pub oja: f64,
    /// Floor of the conductance.
    pub min_weight: f64,
    /// Internal relaxation epochs.
    pub epochs: usize,
}

impl Default for PlasticityConfig {
    fn default() -> Self {
        Self {
            lr_alpha: 0.35,
            lr_eta: 0.25,
            oja: 0.10,
            min_weight: 0.05,
            epochs: 12,
        }
    }
}

/// System 2 verdict about a vertex in conflict.
#[derive(Debug, Clone, PartialEq)]
pub struct VerdictS2 {
    pub target: usize,
    /// Phasor state validated by the deliberation.
    pub phasor: Complex,
    /// (hyperedge index, polarity): +1 reinforces, -1 isolates the refuted path.
    pub polarities: Vec<(usize, f64)>,
}

/// Auditable record of the accepted topological mutation.
#[derive(Debug, Clone, PartialEq)]
pub struct PlasticCommit {
    pub seal_before: u64,
    pub seal_after: u64,
    pub amplitude_before: f64,
    pub amplitude_after: f64,
    pub gain_before: f64,
    pub gain_after: f64,
    pub frustration_before: f64,
    pub frustration_after: f64,
}

/// Mutation refused: the gate restored the previous topology.
#[derive(Debug, Clone, PartialEq)]
pub struct PlasticRefusal {
    pub reason: String,
    pub gain_before: f64,
    pub gain_after: f64,
    pub amplitude_before: f64,
    pub amplitude_after: f64,
}

/// Applies the Kuramoto-Oja rule over the hyperedges cited in the verdict.
pub fn apply(
    graph: &mut MagneticHypergraph,
    environment: &[Complex],
    verdict: &VerdictS2,
    cfg: &PlasticityConfig,
) -> Result<(), String> {
    if verdict.target >= graph.vertices.len() {
        return Err(format!("target {} outside the hypergraph", verdict.target));
    }
    if environment.len() != graph.vertices.len() {
        return Err("environment with incompatible size".to_string());
    }
    if cfg.lr_alpha < 0.0 || cfg.lr_eta < 0.0 || cfg.oja < 0.0 || cfg.epochs == 0 {
        return Err("invalid hyperparameter".to_string());
    }
    let q = graph.q;
    let two_pi_q = 2.0 * std::f64::consts::PI * q;
    if two_pi_q.abs() < 1e-12 {
        return Err("null q: the magnetic angle has no effect".to_string());
    }

    let amp_s2 = verdict.phasor.modulus();
    let phase_s2 = verdict.phasor.phase();

    for _ in 0..cfg.epochs {
        for &(h, sigma) in &verdict.polarities {
            if h >= graph.hyperedges.len() {
                continue;
            }

            let (ctx_amp, arrival) = {
                let edge = &graph.hyperedges[h];
                match arrival_phase(environment, edge, verdict.target, q) {
                    Some((_context, arrival, incoming)) => (incoming, arrival),
                    None => continue,
                }
            };
            if ctx_amp < 1e-12 {
                continue;
            }

            let dphi = phase_s2 - arrival;
            let edge = &mut graph.hyperedges[h];
            let Some(inc) = edge
                .incidences
                .iter_mut()
                .find(|i| i.vertex == verdict.target)
            else {
                continue;
            };

            let d_alpha = (cfg.lr_alpha / two_pi_q) * sigma * amp_s2 * ctx_amp * dphi.sin();
            inc.alpha = wrap_alpha(inc.alpha + d_alpha);

            let hebb = sigma * dphi.cos() * amp_s2 * ctx_amp;
            let oja = cfg.oja * inc.weight * (amp_s2 * amp_s2);
            inc.weight = (inc.weight + cfg.lr_eta * (hebb - oja)).max(cfg.min_weight);
            if !inc.weight.is_finite() || !inc.alpha.is_finite() {
                return Err("plasticity produced a non-finite value".to_string());
            }
        }
    }
    Ok(())
}

fn gain(propagation: &Propagation, target: usize) -> f64 {
    let amp = propagation.state[target].modulus();
    let incoming = propagation.incoming[target];
    if incoming > 1e-12 {
        amp / incoming
    } else {
        0.0
    }
}

/// Runs the cycle: propagate, apply plasticity, propagate again, and seal or roll back.
pub fn consolidate(
    graph: &mut MagneticHypergraph,
    pulse: &[Complex],
    steps: usize,
    dt: f64,
    verdict: &VerdictS2,
    cfg: &PlasticityConfig,
) -> Result<PlasticCommit, PlasticRefusal> {
    let seal_before = graph.topology_seal();
    let before = match graph.propagate(pulse, steps, dt) {
        Ok(p) => p,
        Err(e) => {
            return Err(PlasticRefusal {
                reason: format!("initial propagation failed: {e}"),
                gain_before: 0.0,
                gain_after: 0.0,
                amplitude_before: 0.0,
                amplitude_after: 0.0,
            });
        }
    };
    let amplitude_before = before.state[verdict.target].modulus();
    let gain_before = gain(&before, verdict.target);
    let initial_norm = before.initial_norm2;

    let backup = graph.clone();
    if let Err(e) = apply(graph, &before.state, verdict, cfg) {
        *graph = backup;
        return Err(PlasticRefusal {
            reason: format!("plasticity refused: {e}"),
            gain_before,
            gain_after: gain_before,
            amplitude_before,
            amplitude_after: amplitude_before,
        });
    }

    let after = match graph.propagate(pulse, steps, dt) {
        Ok(p) => p,
        Err(e) => {
            *graph = backup;
            return Err(PlasticRefusal {
                reason: format!("post-plasticity propagation failed: {e}"),
                gain_before,
                gain_after: gain_before,
                amplitude_before,
                amplitude_after: amplitude_before,
            });
        }
    };
    let amplitude_after = after.state[verdict.target].modulus();
    let gain_after = gain(&after, verdict.target);

    let state_finite = after.state.iter().all(|c| c.is_finite());
    let accepted = state_finite
        && gain_after.is_finite()
        && gain_after > gain_before + 1e-6
        && (after.norm2 - initial_norm).abs() < 1e-9;

    if !accepted {
        *graph = backup;
        return Err(PlasticRefusal {
            reason: if state_finite {
                format!(
                    "resonance gain did not rise enough (before {gain_before:.4}, after {gain_after:.4})"
                )
            } else {
                "non-finite state after plasticity".to_string()
            },
            gain_before,
            gain_after,
            amplitude_before,
            amplitude_after,
        });
    }

    Ok(PlasticCommit {
        seal_before,
        seal_after: graph.topology_seal(),
        amplitude_before,
        amplitude_after,
        gain_before,
        gain_after,
        frustration_before: before.frustration,
        frustration_after: after.frustration,
    })
}

/// Seal of a commit: useful for chaining learning epochs in one record.
pub fn commit_seal(commit: &PlasticCommit) -> u64 {
    let mut seal = Seal::new();
    seal.write_u64(commit.seal_before);
    seal.write_u64(commit.seal_after);
    seal.write_f64(commit.amplitude_before);
    seal.write_f64(commit.amplitude_after);
    seal.write_f64(commit.gain_before);
    seal.write_f64(commit.gain_after);
    seal.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::magnetic::tests::{constructive_graph, pulse};

    #[test]
    fn good_verdict_consolidates_and_seals() {
        let mut g = constructive_graph(true);
        let before = g.propagate(&pulse(), 3, 0.65).expect("before");
        assert!(before.state[3].modulus() < 1e-9);
        let verdict = VerdictS2 {
            target: 3,
            phasor: Complex::polar(0.8, before.state[1].phase()),
            polarities: vec![(1, 1.0), (2, 1.0)],
        };
        let commit = consolidate(
            &mut g,
            &pulse(),
            3,
            0.65,
            &verdict,
            &PlasticityConfig::default(),
        )
        .expect("commit");
        assert!(
            commit.amplitude_after > 0.04,
            "amplitude {}",
            commit.amplitude_after
        );
        assert!(commit.gain_after > commit.gain_before);
        assert_ne!(commit.seal_before, commit.seal_after);
        let after = g.propagate(&pulse(), 3, 0.65).expect("after");
        assert!((after.norm2 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn bad_verdict_is_rolled_back() {
        let mut g = constructive_graph(false);
        let before = g.propagate(&pulse(), 2, 0.65).expect("before");
        let seal = g.topology_seal();
        let verdict = VerdictS2 {
            target: 3,
            phasor: Complex::polar(0.9, before.state[3].phase() + 0.9),
            polarities: vec![(2, -1.0)],
        };
        let refusal = consolidate(
            &mut g,
            &pulse(),
            2,
            0.65,
            &verdict,
            &PlasticityConfig::default(),
        )
        .expect_err("should have refused");
        assert_eq!(
            g.topology_seal(),
            seal,
            "topology should have returned to the original"
        );
        assert!(
            refusal.reason.contains("gain"),
            "reason: {}",
            refusal.reason
        );
    }

    #[test]
    fn target_outside_the_graph_is_an_error() {
        let mut g = constructive_graph(false);
        let verdict = VerdictS2 {
            target: 9,
            phasor: Complex::polar(1.0, 0.0),
            polarities: vec![(0, 1.0)],
        };
        assert!(apply(&mut g, &pulse(), &verdict, &PlasticityConfig::default()).is_err());
    }
}
