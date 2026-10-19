//! The multiverse of the health post: how much of the answer is the modeller's choice.
//!
//! Chapter 5 answers "how often does a day's mean wait pass 20 minutes?" under one specification: an M/M/1 queue, rates
//! drawn from normal laws, and a declared correlation. Three choices sit inside that sentence, and each has reasonable
//! alternatives the text did not take:
//!
//! - **the service law**, through its coefficient of variation c: the M/G/1 wait of Pollaczek and Khinchine is
//!   W = 60 lambda (1 + c^2) / (2 mu (mu - lambda)) minutes, which is the M/M/1 wait at c = 1; c in {0.5, 1.0, 1.5};
//! - **the law of the rates**: normal, or lognormal with the same mean and standard deviation;
//! - **the correlation** between arrivals and pace, applied to the underlying normals: -0.5, 0 or +0.5.
//!
//! The eighteen combinations are run with the same draws (common random numbers), so that the differences between them
//! are differences of specification and not of chance. The between-specification variance of the answer is split into
//! the main effect of each choice and their interaction, which in a balanced full design add up to one. Declared before
//! its first run in `empirical/p26-multiverso/DECLARACAO.md`. Deterministic: SplitMix64, seed 2026.

use stochology_core::Rng;

use crate::propagate::{DRAWS, LAMBDA, LIMIT_MIN, MU, PROMISE, SD_LAMBDA, SD_MU, SEED};

pub const SERVICE_CV: [f64; 3] = [0.5, 1.0, 1.5];
pub const CORRELATIONS: [f64; 3] = [-0.5, 0.0, 0.5];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RateLaw {
    Normal,
    Lognormal,
}

impl RateLaw {
    pub fn name(self) -> &'static str {
        match self {
            RateLaw::Normal => "normal",
            RateLaw::Lognormal => "lognormal",
        }
    }
}

pub const LAWS: [RateLaw; 2] = [RateLaw::Normal, RateLaw::Lognormal];

/// The mean wait in the queue of an M/G/1 post, in minutes; infinite when the arrivals reach the pace.
pub fn wait(lambda: f64, mu: f64, cv: f64) -> f64 {
    if lambda >= mu || mu <= 0.0 || lambda < 0.0 {
        f64::INFINITY
    } else {
        60.0 * lambda * (1.0 + cv * cv) / (2.0 * mu * (mu - lambda))
    }
}

/// A rate with the given mean and standard deviation, from a standard normal z.
pub fn rate(law: RateLaw, mean: f64, sd: f64, z: f64) -> f64 {
    match law {
        RateLaw::Normal => mean + sd * z,
        RateLaw::Lognormal => {
            let s2 = (1.0 + sd * sd / (mean * mean)).ln();
            (mean.ln() - 0.5 * s2 + s2.sqrt() * z).exp()
        }
    }
}

fn gaussian(rng: &mut Rng) -> f64 {
    let u = |r: &mut Rng| ((r.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let (a, b) = (u(rng), u(rng));
    (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
}

/// The pairs of standard normals every specification shares, drawn in the order the propagation lab draws them.
pub fn draws(n: usize, seed: u64) -> Vec<(f64, f64)> {
    let mut rng = Rng::new(seed);
    (0..n)
        .map(|_| {
            let z1 = gaussian(&mut rng);
            let z2 = gaussian(&mut rng);
            (z1, z2)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spec {
    pub cv: f64,
    pub law: RateLaw,
    pub rho: f64,
}

/// The share of days above the limit under one specification, and its Monte Carlo standard deviation.
pub fn share_above(spec: Spec, z: &[(f64, f64)]) -> (f64, f64) {
    let root = (1.0 - spec.rho * spec.rho).max(0.0).sqrt();
    let above = z
        .iter()
        .filter(|(z1, z2)| {
            let lambda = rate(spec.law, LAMBDA, SD_LAMBDA, *z1);
            let mu = rate(spec.law, MU, SD_MU, spec.rho * z1 + root * z2);
            wait(lambda, mu, spec.cv) > LIMIT_MIN
        })
        .count() as f64
        / z.len() as f64;
    (above, (above * (1.0 - above) / z.len() as f64).sqrt())
}

pub fn specs() -> Vec<Spec> {
    let mut v = Vec::new();
    for cv in SERVICE_CV {
        for law in LAWS {
            for rho in CORRELATIONS {
                v.push(Spec { cv, law, rho });
            }
        }
    }
    v
}

/// The main-effect shares of the between-specification variance of a balanced full design: for each factor, the
/// variance of the means at its levels over the total variance; the interaction is the rest.
pub fn main_effects(results: &[(Spec, f64)]) -> (f64, f64, f64, f64, f64) {
    let n = results.len() as f64;
    let mean = results.iter().map(|r| r.1).sum::<f64>() / n;
    let total = results.iter().map(|r| (r.1 - mean).powi(2)).sum::<f64>() / n;
    let effect = |key: &dyn Fn(&Spec) -> i64| -> f64 {
        let mut groups: Vec<(i64, Vec<f64>)> = Vec::new();
        for (s, p) in results {
            let k = key(s);
            match groups.iter_mut().find(|g| g.0 == k) {
                Some(g) => g.1.push(*p),
                None => groups.push((k, vec![*p])),
            }
        }
        groups
            .iter()
            .map(|(_, v)| {
                let m = v.iter().sum::<f64>() / v.len() as f64;
                (m - mean).powi(2) * v.len() as f64
            })
            .sum::<f64>()
            / n
    };
    let cv = effect(&|s| (s.cv * 10.0).round() as i64) / total;
    let law = effect(&|s| s.law as i64) / total;
    let rho = effect(&|s| (s.rho * 10.0).round() as i64) / total;
    (total, cv, law, rho, 1.0 - cv - law - rho)
}

pub fn run() {
    println!("== The multiverse of the health post: how much of the answer is the modeller's choice ==");
    println!(
        "question: P(W > {LIMIT_MIN} min), promise <= {PROMISE} | lambda mean {LAMBDA} sd {SD_LAMBDA}, mu mean {MU} sd {SD_MU} per hour | 18 specifications, {DRAWS} common draws, seed {SEED}"
    );
    let z = draws(DRAWS, SEED);
    let mut results = Vec::new();
    let mut largest_mc_sd: f64 = 0.0;
    println!("  service cv | rate law  | correlation | P(W > {LIMIT_MIN}) | MC sd  | decision");
    for s in specs() {
        let (p, sd) = share_above(s, &z);
        largest_mc_sd = largest_mc_sd.max(sd);
        println!(
            "        {:.1} | {:<9} |        {:+.1} |       {:.4} | {:.4} | {}",
            s.cv,
            s.law.name(),
            s.rho,
            p,
            sd,
            if p > PROMISE { "second nurse" } else { "promise kept" }
        );
        results.push((s, p));
    }
    let nurse = results.iter().filter(|r| r.1 > PROMISE).count();
    let lo = results.iter().map(|r| r.1).fold(f64::INFINITY, f64::min);
    let hi = results.iter().map(|r| r.1).fold(f64::NEG_INFINITY, f64::max);
    let (total, cv, law, rho, inter) = main_effects(&results);
    let between_sd = total.sqrt();
    println!(
        "\nthe 18 specifications: {nurse} say second nurse, {} say promise kept | P from {lo:.4} to {hi:.4} | between-specification sd {between_sd:.4}, largest Monte Carlo sd {largest_mc_sd:.4} (ratio {:.0})",
        18 - nurse,
        between_sd / largest_mc_sd
    );
    println!(
        "between-specification variance of P, by choice: service cv {:.2} | rate law {:.2} | correlation {:.2} | interaction {:.2}",
        cv, law, rho, inter
    );
    println!(
        "  as a budget of the specification: service-cv={cv:.2}; rate-law={law:.2}; correlation={rho:.2}; interaction={inter:.2}"
    );
    let book = results
        .iter()
        .find(|r| r.0 == Spec { cv: 1.0, law: RateLaw::Normal, rho: 0.0 })
        .expect("the book's specification is in the design");
    println!(
        "the book's specification (M/M/1, normal rates, no correlation): P = {:.4}, {}",
        book.1,
        if book.1 > PROMISE { "second nurse" } else { "promise kept" }
    );
    println!("\n-- the rules declared before the run --");
    let r1 = nurse > 0 && nurse < 18;
    let r2 = cv > law + rho;
    let r3 = between_sd > 10.0 * largest_mc_sd;
    let say = |b: bool| if b { "holds" } else { "fails" };
    println!("  R1, the 18 specifications do not agree on the decision: {}", say(r1));
    println!(
        "  R2, the service cv carries more of the between-specification variance than the rate law and the correlation together: {}",
        say(r2)
    );
    println!(
        "  R3, the between-specification sd is more than ten times the largest Monte Carlo sd: {}",
        say(r3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mg1_wait_is_the_mm1_wait_at_unit_cv() {
        assert!((wait(8.0, 12.0, 1.0) - crate::propagate::wait(8.0, 12.0)).abs() < 1e-12);
        assert!((wait(8.0, 12.0, 0.0) - 5.0).abs() < 1e-12);
        assert!(wait(12.0, 12.0, 0.5).is_infinite());
    }

    #[test]
    fn the_lognormal_keeps_the_mean_and_the_sd() {
        let n = 200_000;
        let z = draws(n, 11);
        let v: Vec<f64> = z.iter().map(|(a, _)| rate(RateLaw::Lognormal, 12.0, 0.8, *a)).collect();
        let m = v.iter().sum::<f64>() / n as f64;
        let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n as f64).sqrt();
        assert!((m - 12.0).abs() < 0.01 && (sd - 0.8).abs() < 0.01, "{m} {sd}");
    }

    #[test]
    fn the_main_effects_of_a_balanced_design_add_up_to_one_and_find_a_pure_effect() {
        // a response that depends only on the service cv: all of its variance is that choice's
        let results: Vec<(Spec, f64)> = specs().into_iter().map(|s| (s, s.cv * s.cv)).collect();
        let (_, cv, law, rho, inter) = main_effects(&results);
        assert!((cv - 1.0).abs() < 1e-12 && law.abs() < 1e-12 && rho.abs() < 1e-12 && inter.abs() < 1e-12);
        assert_eq!(specs().len(), 18);
    }

    #[test]
    fn the_books_specification_reproduces_the_propagation_lab_draw_for_draw() {
        // the one specification already published (Chapter 5, labs --propagate); the other seventeen are not run here
        let z = draws(DRAWS, SEED);
        let (p, _) = share_above(Spec { cv: 1.0, law: RateLaw::Normal, rho: 0.0 }, &z);
        let published = crate::propagate::answers(0.0, DRAWS, SEED).mc_p;
        assert_eq!(p, published);
    }
}
