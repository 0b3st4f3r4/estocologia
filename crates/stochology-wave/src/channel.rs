//! A spike bus over any number of channels, on any integer clock.
//!
//! The bus of `spike` is the five lenses on a microsecond clock, which is what the engine's gates need and what no
//! dataset offers. This module carries the same phasor STDP rule over N channels whose clock unit is whatever the
//! data use (half-hours, minutes, days): the time constants of `StdpConfig` are read in that unit. With five
//! channels in lens order it reproduces `StdpBus` number for number, which a test pins.

use crate::seal::{Seal, wrap_alpha, wrap_pi};
use crate::spike::StdpConfig;

/// One event on a channel: when, how strong, and at which phase.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event {
    pub channel: usize,
    pub t: u64,
    pub amplitude: f64,
    pub phase: f64,
}

impl Event {
    pub fn new(channel: usize, t: u64, amplitude: f64, phase: f64) -> Self {
        Self {
            channel,
            t,
            amplitude,
            phase,
        }
    }
}

/// Synaptic matrices between N channels and the stream that trained them.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelBus {
    pub conductance: Vec<Vec<f64>>,
    pub alpha: Vec<Vec<f64>>,
    pub stream: Vec<Event>,
}

impl ChannelBus {
    pub fn new(channels: usize, initial: f64) -> Self {
        let mut conductance = vec![vec![0.0; channels]; channels];
        for (i, row) in conductance.iter_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                if i != j {
                    *c = initial;
                }
            }
        }
        Self {
            conductance,
            alpha: vec![vec![0.0; channels]; channels],
            stream: Vec::new(),
        }
    }

    pub fn channels(&self) -> usize {
        self.conductance.len()
    }

    /// Publishes one event and applies the phasor STDP rule against the recent stream: causal potentiation of
    /// the path from each earlier event to this one, anticausal depression of the reverse path.
    pub fn publish(&mut self, event: Event, cfg: &StdpConfig) {
        let n = self.channels();
        if event.channel >= n {
            return;
        }
        let two_pi_q = 2.0 * std::f64::consts::PI * cfg.q;
        let pos = event.channel;

        for k in (0..self.stream.len()).rev() {
            let past = self.stream[k];
            if event.t < past.t {
                continue;
            }
            let dt = event.t - past.t;
            if dt > cfg.horizon_us {
                break;
            }
            if dt == 0 || past.channel == event.channel {
                continue;
            }
            let pre = past.channel;
            let dt_f = dt as f64;

            let k_plus = cfg.a_plus * (-dt_f / cfg.tau_plus_us).exp();
            let arrival = past.phase + two_pi_q * self.alpha[pre][pos];
            let dphi = wrap_pi(event.phase - arrival);
            let hebb = k_plus * past.amplitude * event.amplitude * dphi.cos();
            let oja = cfg.oja * self.conductance[pre][pos] * event.amplitude * event.amplitude;
            self.conductance[pre][pos] = (self.conductance[pre][pos] + hebb - oja).clamp(0.01, 2.0);
            let d_alpha =
                (cfg.lr_alpha / two_pi_q) * k_plus * past.amplitude * event.amplitude * dphi.sin();
            self.alpha[pre][pos] = wrap_alpha(self.alpha[pre][pos] + d_alpha);

            let k_minus = cfg.a_minus * (-dt_f / cfg.tau_minus_us).exp();
            let reverse_arrival = event.phase + two_pi_q * self.alpha[pos][pre];
            let dphi_rev = wrap_pi(past.phase - reverse_arrival);
            let ltd = k_minus * past.amplitude * event.amplitude * dphi_rev.cos().abs();
            self.conductance[pos][pre] = (self.conductance[pos][pre] - ltd).clamp(0.01, 2.0);
        }
        self.stream.push(event);
    }

    /// Publishes a batch in time order (stable: events at the same instant keep their given order).
    pub fn publish_all(&mut self, mut events: Vec<Event>, cfg: &StdpConfig) {
        events.sort_by_key(|e| e.t);
        for e in events {
            self.publish(e, cfg);
        }
    }

    /// Causal asymmetry: eta(from -> to) - eta(to -> from).
    pub fn asymmetry(&self, from: usize, to: usize) -> f64 {
        self.conductance[from][to] - self.conductance[to][from]
    }

    #[allow(clippy::needless_range_loop)]
    pub fn seal(&self) -> u64 {
        let mut seal = Seal::new();
        let n = self.channels();
        for i in 0..n {
            for j in 0..n {
                seal.write_f64(self.conductance[i][j]);
                seal.write_f64(self.alpha[i][j]);
            }
        }
        seal.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spike::{Spike, StdpBus, index, lens_at};
    use stochology_hypergraph::Lens;

    fn cascade(base: u64) -> Vec<Spike> {
        let lenses = [
            Lens::Morphonomy,
            Lens::Fluxonomy,
            Lens::Chrononomy,
            Lens::Scalonomy,
            Lens::Crisonomy,
        ];
        let times = [0u64, 10_200, 21_800, 35_100, 49_900];
        let amps = [0.90, 0.88, 0.85, 0.92, 0.96];
        let phases = [0.10, 0.35, 0.60, 0.90, 1.20];
        (0..5)
            .map(|k| {
                Spike::new(
                    lenses[k],
                    k as u32,
                    base + times[k],
                    amps[k],
                    phases[k],
                    k as u64,
                )
            })
            .collect()
    }

    #[test]
    fn five_channels_in_lens_order_reproduce_the_lens_bus() {
        let cfg = StdpConfig::default();
        let mut lens_bus = StdpBus::new(0.25);
        let mut channel_bus = ChannelBus::new(5, 0.25);
        for ep in 0..5u64 {
            for s in cascade(ep * 200_000) {
                lens_bus.publish(s, &cfg);
                channel_bus.publish(
                    Event::new(index(s.lens), s.t_us, s.amplitude, s.phase),
                    &cfg,
                );
            }
        }
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(lens_bus.conductance[i][j], channel_bus.conductance[i][j]);
                assert_eq!(lens_bus.alpha[i][j], channel_bus.alpha[i][j]);
            }
        }
        assert_eq!(lens_bus.seal(), channel_bus.seal());
        assert_eq!(lens_at(index(Lens::Scalonomy)), Lens::Scalonomy);
    }

    #[test]
    fn a_planted_lead_gives_a_positive_arrow_and_its_mirror_a_negative_one() {
        let cfg = StdpConfig {
            tau_plus_us: 6.0,
            tau_minus_us: 6.0,
            horizon_us: 12,
            ..StdpConfig::default()
        };
        let mut lead = ChannelBus::new(2, 0.25);
        let mut mirror = ChannelBus::new(2, 0.25);
        for k in 0..40u64 {
            let t = k * 48 + 10;
            lead.publish_all(
                vec![Event::new(0, t, 0.8, 0.5), Event::new(1, t + 4, 0.8, 1.0)],
                &cfg,
            );
            mirror.publish_all(
                vec![Event::new(1, t, 0.8, 0.5), Event::new(0, t + 4, 0.8, 1.0)],
                &cfg,
            );
        }
        assert!(lead.asymmetry(0, 1) > 0.05, "{}", lead.asymmetry(0, 1));
        assert!(mirror.asymmetry(0, 1) < -0.05, "{}", mirror.asymmetry(0, 1));
    }

    #[test]
    fn events_beyond_the_horizon_leave_the_bus_untouched() {
        let cfg = StdpConfig {
            tau_plus_us: 6.0,
            tau_minus_us: 6.0,
            horizon_us: 12,
            ..StdpConfig::default()
        };
        let mut bus = ChannelBus::new(2, 0.25);
        bus.publish_all(
            vec![Event::new(0, 0, 0.9, 0.0), Event::new(1, 30, 0.9, 0.0)],
            &cfg,
        );
        assert_eq!(bus.asymmetry(0, 1), 0.0);
    }
}
