//! Epistemic spike bus: asynchronous communication between the five lenses.
//!
//! There is no free text between agents. Each event is an immutable tuple of 40 logical bytes
//! (lens, vertex, instant, amplitude, phase, evidence seal). Learning the causal order between
//! lenses is the work of the phasor STDP rule; recognizing a pattern is the work of the spike
//! vocabulary, which compares intervals and phase lags, not absolute instants.

use stochology_hypergraph::Lens;

use crate::seal::{Seal, wrap_alpha, wrap_pi};

/// Number of operational lenses.
pub const LENSES: usize = 5;

/// Canonical position of the lens in the synaptic matrix.
pub fn index(lens: Lens) -> usize {
    match lens {
        Lens::Fluxonomy => 0,
        Lens::Chrononomy => 1,
        Lens::Scalonomy => 2,
        Lens::Morphonomy => 3,
        Lens::Crisonomy => 4,
    }
}

/// Lens from the canonical position.
pub fn lens_at(position: usize) -> Lens {
    match position % LENSES {
        0 => Lens::Fluxonomy,
        1 => Lens::Chrononomy,
        2 => Lens::Scalonomy,
        3 => Lens::Morphonomy,
        _ => Lens::Crisonomy,
    }
}

/// One spike on the bus.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spike {
    pub lens: Lens,
    pub vertex: u32,
    pub t_us: u64,
    pub amplitude: f64,
    pub phase: f64,
    pub evidence_hash: u64,
}

impl Spike {
    pub fn new(
        lens: Lens,
        vertex: u32,
        t_us: u64,
        amplitude: f64,
        phase: f64,
        evidence_hash: u64,
    ) -> Self {
        Self {
            lens,
            vertex,
            t_us,
            amplitude,
            phase,
            evidence_hash,
        }
    }
}

/// One expected note inside a word: lens, delay, and phase lag relative to the anchor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub lens: Lens,
    pub delay_us: u64,
    pub phase_lag: f64,
}

impl Note {
    pub fn new(lens: Lens, delay_us: u64, phase_lag: f64) -> Self {
        Self {
            lens,
            delay_us,
            phase_lag,
        }
    }
}

/// Minimum contribution threshold of a note.
const MIN_CONTRIBUTION: f64 = 0.10;

/// One word of the spike vocabulary.
#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub id: &'static str,
    pub meaning: &'static str,
    /// Temporal tolerance of the coincidence, in microseconds.
    pub tau_us: f64,
    pub threshold: f64,
    pub notes: Vec<Note>,
}

/// Word recognized in a spike stream.
#[derive(Debug, Clone, PartialEq)]
pub struct FulfilledWord {
    pub id: &'static str,
    pub meaning: &'static str,
    pub anchor_us: u64,
    pub resonance: f64,
    pub resulting_phase: f64,
    pub seal: u64,
}

impl Word {
    /// Looks for the word in the stream, at any absolute instant.
    pub fn matches(&self, stream: &[Spike]) -> Option<FulfilledWord> {
        if self.notes.len() < 2 || stream.len() < self.notes.len() {
            return None;
        }
        let first = self.notes[0];
        let mut best: Option<FulfilledWord> = None;

        for (i, anchor) in stream.iter().enumerate() {
            if anchor.lens != first.lens {
                continue;
            }
            let mut sum_re = anchor.amplitude;
            let mut sum_im = 0.0;
            let mut matched = 1usize;
            let mut seal = Seal::new();
            seal.write_u64(anchor.evidence_hash);

            for note in &self.notes[1..] {
                let mut best_note = 0.0;
                let mut contribution = (0.0, 0.0);
                let mut evidence = 0u64;

                for candidate in &stream[(i + 1)..] {
                    if candidate.lens != note.lens || candidate.t_us < anchor.t_us {
                        continue;
                    }
                    let delay = (candidate.t_us - anchor.t_us) as f64;
                    let error = (delay - note.delay_us as f64).abs();
                    let decay = (-error / self.tau_us).exp();

                    let phase_lag = wrap_pi(candidate.phase - anchor.phase);
                    let phase_error = wrap_pi(phase_lag - note.phase_lag);

                    let effective = candidate.amplitude * decay;
                    let note_score = effective * phase_error.cos();
                    if note_score > best_note {
                        best_note = note_score;
                        contribution =
                            (effective * phase_error.cos(), effective * phase_error.sin());
                        evidence = candidate.evidence_hash;
                    }
                }

                if best_note > MIN_CONTRIBUTION {
                    matched += 1;
                    sum_re += contribution.0;
                    sum_im += contribution.1;
                    seal.write_u64(evidence);
                }
            }

            if matched != self.notes.len() {
                continue;
            }

            let m = self.notes.len() as f64;
            let resonance = (sum_re * sum_re + sum_im * sum_im).sqrt() / m;
            if resonance < self.threshold {
                continue;
            }

            let fulfilled = FulfilledWord {
                id: self.id,
                meaning: self.meaning,
                anchor_us: anchor.t_us,
                resonance,
                resulting_phase: sum_im.atan2(sum_re),
                seal: seal.finish(),
            };
            if best
                .as_ref()
                .is_none_or(|b| fulfilled.resonance > b.resonance)
            {
                best = Some(fulfilled);
            }
        }
        best
    }
}

/// Hyperparameters of the phasor STDP rule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StdpConfig {
    pub q: f64,
    pub a_plus: f64,
    pub a_minus: f64,
    pub tau_plus_us: f64,
    pub tau_minus_us: f64,
    pub lr_alpha: f64,
    pub oja: f64,
    pub horizon_us: u64,
}

impl Default for StdpConfig {
    fn default() -> Self {
        Self {
            q: 0.5,
            a_plus: 0.18,
            a_minus: 0.12,
            tau_plus_us: 20_000.0,
            tau_minus_us: 20_000.0,
            lr_alpha: 0.30,
            oja: 0.04,
            horizon_us: 60_000,
        }
    }
}

/// 5x5 synaptic matrix between lenses, with the spike stream that trained it.
#[derive(Debug, Clone, PartialEq)]
pub struct StdpBus {
    pub conductance: [[f64; LENSES]; LENSES],
    pub alpha: [[f64; LENSES]; LENSES],
    pub stream: Vec<Spike>,
}

impl StdpBus {
    #[allow(clippy::needless_range_loop)]
    pub fn new(initial: f64) -> Self {
        let mut conductance = [[0.0; LENSES]; LENSES];
        for i in 0..LENSES {
            for j in 0..LENSES {
                if i != j {
                    conductance[i][j] = initial;
                }
            }
        }
        Self {
            conductance,
            alpha: [[0.0; LENSES]; LENSES],
            stream: Vec::new(),
        }
    }

    /// Publishes one spike and applies STDP against the recent stream.
    pub fn publish(&mut self, spike: Spike, cfg: &StdpConfig) {
        let two_pi_q = 2.0 * std::f64::consts::PI * cfg.q;
        let pos = index(spike.lens);

        for past in self.stream.iter().rev() {
            if spike.t_us < past.t_us {
                continue;
            }
            let dt = spike.t_us - past.t_us;
            if dt > cfg.horizon_us {
                break;
            }
            if dt == 0 || past.lens == spike.lens {
                continue;
            }

            let pre = index(past.lens);
            let dt_f = dt as f64;

            // Causal LTP: pre before post.
            let k_plus = cfg.a_plus * (-dt_f / cfg.tau_plus_us).exp();
            let arrival = past.phase + two_pi_q * self.alpha[pre][pos];
            let dphi = wrap_pi(spike.phase - arrival);
            let hebb = k_plus * past.amplitude * spike.amplitude * dphi.cos();
            let oja = cfg.oja * self.conductance[pre][pos] * spike.amplitude * spike.amplitude;

            self.conductance[pre][pos] = (self.conductance[pre][pos] + hebb - oja).clamp(0.01, 2.0);
            let d_alpha =
                (cfg.lr_alpha / two_pi_q) * k_plus * past.amplitude * spike.amplitude * dphi.sin();
            self.alpha[pre][pos] = wrap_alpha(self.alpha[pre][pos] + d_alpha);

            // Anticausal LTD: the reverse path pays the cost.
            let k_minus = cfg.a_minus * (-dt_f / cfg.tau_minus_us).exp();
            let reverse_arrival = spike.phase + two_pi_q * self.alpha[pos][pre];
            let dphi_rev = wrap_pi(past.phase - reverse_arrival);
            let ltd = k_minus * past.amplitude * spike.amplitude * dphi_rev.cos().abs();
            self.conductance[pos][pre] = (self.conductance[pos][pre] - ltd).clamp(0.01, 2.0);
        }

        self.stream.push(spike);
    }

    /// Causal asymmetry: eta(from -> to) - eta(to -> from).
    pub fn asymmetry(&self, from: Lens, to: Lens) -> f64 {
        self.conductance[index(from)][index(to)] - self.conductance[index(to)][index(from)]
    }

    /// Canonical seal of the synaptic matrix.
    #[allow(clippy::needless_range_loop)]
    pub fn seal(&self) -> u64 {
        let mut seal = Seal::new();
        for i in 0..LENSES {
            for j in 0..LENSES {
                seal.write_f64(self.conductance[i][j]);
                seal.write_f64(self.alpha[i][j]);
            }
        }
        seal.finish()
    }

    /// Requires a minimum positive asymmetry in every link of the declared causal chain.
    pub fn check_causality(&self, chain: &[(Lens, Lens)], minimum: f64) -> Result<(), String> {
        for &(from, to) in chain {
            let a = self.asymmetry(from, to);
            if a < minimum || a.is_nan() {
                return Err(format!(
                    "insufficient causality in {} -> {}: asymmetry {a:.4}",
                    from.name(),
                    to.name()
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn vocabulary() -> Vec<Word> {
        vec![
            Word {
                id: "W_CASCADE",
                meaning: "deformation, bottleneck, cycle drift, amplification, and rupture",
                tau_us: 8_000.0,
                threshold: 0.70,
                notes: vec![
                    Note::new(Lens::Morphonomy, 0, 0.0),
                    Note::new(Lens::Fluxonomy, 10_000, 0.25),
                    Note::new(Lens::Chrononomy, 22_000, 0.50),
                    Note::new(Lens::Scalonomy, 35_000, 0.80),
                    Note::new(Lens::Crisonomy, 50_000, 1.10),
                ],
            },
            Word {
                id: "W_BRAKE",
                meaning: "abrupt phase inversion between flux and form",
                tau_us: 5_000.0,
                threshold: 0.70,
                notes: vec![
                    Note::new(Lens::Fluxonomy, 0, 0.0),
                    Note::new(Lens::Morphonomy, 6_000, std::f64::consts::PI),
                ],
            },
        ]
    }

    pub(crate) fn cascade(base: u64, embedded: bool) -> Vec<Spike> {
        let phase = |k: usize| {
            if embedded {
                let ps = [0.10, 0.35, 0.60, 0.90, 1.20];
                ps[k]
            } else {
                let ps = [0.10, 0.90, 0.20, 1.60, 0.50];
                ps[k]
            }
        };
        let lenses = [
            Lens::Morphonomy,
            Lens::Fluxonomy,
            Lens::Chrononomy,
            Lens::Scalonomy,
            Lens::Crisonomy,
        ];
        let times = [0u64, 10_200, 21_800, 35_100, 49_900];
        let amps = [0.90, 0.88, 0.85, 0.92, 0.96];
        (0..5)
            .map(|k| {
                Spike::new(
                    lenses[k],
                    10 + k as u32,
                    base + times[k],
                    amps[k],
                    phase(k),
                    0xA000 + base + k as u64,
                )
            })
            .collect()
    }

    #[test]
    fn recognizes_cascade_at_any_instant() {
        let voc = vocabulary();
        let stream = cascade(1_000, true);
        let found = voc[0].matches(&stream).expect("cascade");
        assert!(found.resonance > 0.80, "resonance {}", found.resonance);
        let shifted: Vec<Spike> = stream
            .iter()
            .map(|e| Spike {
                t_us: e.t_us + 5_000_000,
                ..*e
            })
            .collect();
        let other = voc[0].matches(&shifted).expect("shifted cascade");
        assert!((other.resonance - found.resonance).abs() < 1e-9);
        assert_eq!(other.seal, found.seal);
    }

    #[test]
    fn wrong_intervals_do_not_match() {
        let voc = vocabulary();
        let stream: Vec<Spike> = cascade(1_000, true)
            .into_iter()
            .zip([0u64, 2_000, 4_000, 6_000, 8_000])
            .map(|(e, t)| Spike {
                t_us: 1_000 + t,
                ..e
            })
            .collect();
        assert!(voc[0].matches(&stream).is_none());
    }

    #[test]
    fn stdp_creates_causal_asymmetry() {
        let mut bus = StdpBus::new(0.25);
        let cfg = StdpConfig::default();
        for ep in 0..5u64 {
            for e in cascade(ep * 200_000, true) {
                bus.publish(e, &cfg);
            }
        }
        let chain = [
            (Lens::Morphonomy, Lens::Fluxonomy),
            (Lens::Fluxonomy, Lens::Chrononomy),
            (Lens::Chrononomy, Lens::Scalonomy),
            (Lens::Scalonomy, Lens::Crisonomy),
        ];
        for &(from, to) in &chain {
            assert!(
                bus.asymmetry(from, to) > 0.05,
                "asymmetry {} -> {} = {}",
                from.name(),
                to.name(),
                bus.asymmetry(from, to)
            );
        }
        bus.check_causality(&chain, 0.05).expect("causal chain");
    }

    #[test]
    fn bus_seal_is_stable() {
        let cfg = StdpConfig::default();
        let mut a = StdpBus::new(0.25);
        let mut b = StdpBus::new(0.25);
        for e in cascade(0, true) {
            a.publish(e, &cfg);
            b.publish(e, &cfg);
        }
        assert_eq!(a.seal(), b.seal());
    }

    #[test]
    fn index_and_lens_round_trip() {
        for l in Lens::ALL {
            assert_eq!(lens_at(index(l)), l);
        }
    }
}
