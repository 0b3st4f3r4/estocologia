//! Heavy inference: random-walk Metropolis with the target declared by the caller.
//!
//! docs/PLAN.md §4 places "MCMC inference" among the tasks of System 2. What exists here is the
//! Metropolis-Hastings sampler with a symmetric gaussian proposal and a fixed seed: the target enters as a
//! declared log-density, and the chain returns samples, acceptance rate and a summary of its final half.
//!
//! Declared and not implemented: step adaptation (the step is declared, not learned), convergence diagnostics
//! by R-hat and SBC, and multidimensional targets.

use stochology_core::Rng;

fn uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn normal(rng: &mut Rng) -> f64 {
    let u1 = uniform(rng);
    let u2 = uniform(rng);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Metropolis sampler: the step of the proposal and the seed of the chain.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metropolis {
    pub step: f64,
    pub seed: u64,
}

/// The chain: the samples in the order they came out, and how many proposals were accepted.
#[derive(Debug, Clone, PartialEq)]
pub struct Chain {
    pub samples: Vec<f64>,
    pub accepted: usize,
}

impl Chain {
    pub fn acceptance_rate(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.accepted as f64 / self.samples.len() as f64
    }

    pub fn mean(&self, burn_in: usize) -> f64 {
        let body = self.body(burn_in);
        if body.is_empty() {
            return 0.0;
        }
        body.iter().sum::<f64>() / body.len() as f64
    }

    pub fn std_dev(&self, burn_in: usize) -> f64 {
        let body = self.body(burn_in);
        if body.len() < 2 {
            return 0.0;
        }
        let mean = body.iter().sum::<f64>() / body.len() as f64;
        (body.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (body.len() as f64 - 1.0)).sqrt()
    }

    /// Quantile of the final part of the chain, by linear interpolation (the same rule as H0).
    pub fn quantile(&self, burn_in: usize, q: f64) -> f64 {
        let mut body = self.body(burn_in).to_vec();
        body.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        if body.is_empty() {
            return 0.0;
        }
        let position = q.clamp(0.0, 1.0) * (body.len() as f64 - 1.0);
        let low = position.floor() as usize;
        let high = position.ceil() as usize;
        if low == high {
            return body[low];
        }
        let weight = position - low as f64;
        body[low] * (1.0 - weight) + body[high] * weight
    }

    fn body(&self, burn_in: usize) -> &[f64] {
        let cutoff = burn_in.min(self.samples.len());
        &self.samples[cutoff..]
    }
}

impl Metropolis {
    /// Runs the chain from `start`, with `log_target` declared as an unnormalized density.
    pub fn sample<F: Fn(f64) -> f64>(&self, log_target: F, start: f64, iterations: usize) -> Chain {
        let mut rng = Rng::new(self.seed);
        let mut current = start;
        let mut log_current = log_target(current);
        let mut samples = Vec::with_capacity(iterations);
        let mut accepted = 0usize;
        for _ in 0..iterations {
            let proposal = current + self.step * normal(&mut rng);
            let log_proposal = log_target(proposal);
            // a target outside the support returns -inf: reject without drawing
            let accept = if !log_proposal.is_finite() {
                false
            } else if !log_current.is_finite() {
                true
            } else {
                let ratio = (log_proposal - log_current).min(0.0);
                ratio.exp() > uniform(&mut rng)
            };
            if accept {
                current = proposal;
                log_current = log_proposal;
                accepted += 1;
            }
            samples.push(current);
        }
        Chain { samples, accepted }
    }
}

/// Log-density of a normal, up to a constant.
pub fn log_normal(mean: f64, sd: f64) -> impl Fn(f64) -> f64 {
    move |x: f64| -0.5 * ((x - mean) / sd).powi(2)
}

/// Log-density of a beta with the two count parameters.
pub fn log_beta(alpha: f64, beta: f64) -> impl Fn(f64) -> f64 {
    move |x: f64| {
        if x <= 0.0 || x >= 1.0 {
            f64::NEG_INFINITY
        } else {
            (alpha - 1.0) * x.ln() + (beta - 1.0) * (1.0 - x).ln()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_chain_recovers_the_mean_of_a_normal() {
        let chain = Metropolis {
            step: 1.5,
            seed: 2026,
        }
        .sample(log_normal(2.0, 1.0), 0.0, 200_000);
        let mean = chain.mean(20_000);
        assert!((mean - 2.0).abs() < 0.05, "mean of the chain: {mean}");
        let sd = chain.std_dev(20_000);
        assert!((sd - 1.0).abs() < 0.05, "std dev of the chain: {sd}");
        let acceptance = chain.acceptance_rate();
        assert!(
            (0.15..0.60).contains(&acceptance),
            "acceptance: {acceptance}"
        );
    }

    #[test]
    fn the_chain_recovers_the_conjugate_posterior_of_the_beta() {
        // Beta(4, 8): exact mean 4/12 = 1/3, on the support [0, 1].
        let chain = Metropolis {
            step: 0.15,
            seed: 7,
        }
        .sample(log_beta(4.0, 8.0), 0.5, 200_000);
        let mean = chain.mean(20_000);
        assert!(
            (mean - 1.0 / 3.0).abs() < 0.02,
            "mean of the posterior: {mean}"
        );
        assert!(
            chain.samples.iter().all(|x| (0.0..=1.0).contains(x)),
            "the chain respects the support"
        );
        let median = chain.quantile(20_000, 0.5);
        assert!(
            median < mean,
            "Beta(4,8) has its median below the mean: {median} against {mean}"
        );
    }

    #[test]
    fn the_chain_is_deterministic_in_the_seed() {
        let sampler = Metropolis {
            step: 1.0,
            seed: 99,
        };
        let target = log_normal(0.0, 1.0);
        assert_eq!(
            sampler.sample(&target, 0.0, 5_000),
            sampler.sample(&target, 0.0, 5_000)
        );
        let other = Metropolis {
            step: 1.0,
            seed: 100,
        };
        assert_ne!(
            sampler.sample(&target, 0.0, 5_000),
            other.sample(&target, 0.0, 5_000)
        );
    }

    #[test]
    fn a_large_step_drops_the_acceptance() {
        let target = log_normal(0.0, 1.0);
        let small = Metropolis { step: 0.1, seed: 3 }.sample(&target, 0.0, 50_000);
        let large = Metropolis {
            step: 50.0,
            seed: 3,
        }
        .sample(&target, 0.0, 50_000);
        assert!(small.acceptance_rate() > large.acceptance_rate());
        assert!(
            large.acceptance_rate() < 0.05,
            "an absurd proposal is almost never accepted: {}",
            large.acceptance_rate()
        );
    }

    #[test]
    fn a_target_outside_the_support_accepts_nothing() {
        let chain = Metropolis { step: 1.0, seed: 5 }.sample(
            |x: f64| if x < 0.0 { f64::NEG_INFINITY } else { -x },
            1.0,
            1_000,
        );
        assert!(
            chain.samples.iter().all(|x| *x >= 0.0),
            "the chain stays on the support"
        );
    }
}
