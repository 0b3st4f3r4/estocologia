//! Chapter 6 — The invisible filter: the Literary Digest, Meng's identity
//! and the independent unit.
//!
//! Self-contained laboratory: built-in SplitMix64, no external dependencies.
//! Usage: cargo run --release

#[derive(Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }
    fn bernoulli(&mut self, p: f64) -> bool {
        self.uniform() < p
    }
    /// Standard normal by Box-Muller.
    fn normal(&mut self) -> f64 {
        let u1 = self.uniform().max(1e-18);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

/// The rigged 1936 poll: voters whose propensity to answer depends on the vote.
fn literary_digest(
    seed: u64,
    n_voters: u64,
    p_roosevelt: f64,
    q_roosevelt: f64,
    q_landon: f64,
) -> (u64, f64) {
    let mut rng = SplitMix64::new(seed);
    let mut respondents = 0u64;
    let mut roosevelt_among_respondents = 0u64;
    for _ in 0..n_voters {
        let roosevelt = rng.bernoulli(p_roosevelt);
        let q = if roosevelt { q_roosevelt } else { q_landon };
        if rng.bernoulli(q) {
            respondents += 1;
            if roosevelt {
                roosevelt_among_respondents += 1;
            }
        }
    }
    (
        respondents,
        roosevelt_among_respondents as f64 / respondents as f64,
    )
}

/// Expected error by Meng's identity: rho * sqrt((1-f)/f) * sigma_Y.
fn meng(rho: f64, f: f64, sigma_y: f64) -> f64 {
    rho * ((1.0 - f) / f).sqrt() * sigma_y
}

/// Correlation coefficient between two columns.
fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;
    let mut cov_ab = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for i in 0..a.len() {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov_ab += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    cov_ab / (var_a.sqrt() * var_b.sqrt())
}

/// The filter manufactures relations: two traits that are independent in the
/// population become negatively associated among the selected (Berkson).
fn berkson(seed: u64, n: usize, cutoff: f64) -> (f64, f64, f64) {
    let mut rng = SplitMix64::new(seed);
    let mut t_population = Vec::with_capacity(n);
    let mut s_population = Vec::with_capacity(n);
    let mut t_selected = Vec::new();
    let mut s_selected = Vec::new();
    for _ in 0..n {
        let t = rng.normal();
        let s = rng.normal();
        t_population.push(t);
        s_population.push(s);
        if t + s > cutoff {
            t_selected.push(t);
            s_selected.push(s);
        }
    }
    (
        correlation(&t_population, &s_population),
        correlation(&t_selected, &s_selected),
        100.0 * t_selected.len() as f64 / n as f64,
    )
}

/// Effective size of a sample of n letters in a two-state chain with
/// persistence p: measures the variance of the sample mean over many replicas.
fn effective_size(seed: u64, persistence: f64, n: usize, replicas: usize) -> (f64, f64) {
    let mut rng = SplitMix64::new(seed);
    let mut means = Vec::with_capacity(replicas);
    for _ in 0..replicas {
        let mut state = rng.bernoulli(0.5);
        let mut sum = 0usize;
        for _ in 0..n {
            if rng.uniform() > persistence {
                state = !state;
            }
            if state {
                sum += 1;
            }
        }
        means.push(sum as f64 / n as f64);
    }
    let m = means.iter().sum::<f64>() / replicas as f64;
    let var = means.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (replicas as f64 - 1.0);
    let mean_p = 0.5;
    (var, mean_p * (1.0 - mean_p) / var)
}

pub fn run() {
    println!("== Chapter 6 :: the invisible filter ==");

    // (1) The rigged 1936 poll.
    let p_roosevelt = 0.608_f64;
    let sigma_y = (p_roosevelt * (1.0 - p_roosevelt)).sqrt();
    let n_voters = 10_000_000u64;
    let (respondents, r_among) = literary_digest(1936, n_voters, p_roosevelt, 0.168, 0.3517);
    let f = respondents as f64 / n_voters as f64;
    let error = r_among - p_roosevelt;
    println!(
        "Literary Digest: {} of {} voters answered (f = {:.3}); Roosevelt among them = {:.4}, Landon = {:.4}",
        respondents,
        n_voters,
        f,
        r_among,
        1.0 - r_among
    );
    println!(
        "measured error = {:+.4} ({:.1} points); Meng identity with rho = -0.21: {:+.4}",
        error,
        100.0 * error,
        meng(-0.21, f, sigma_y)
    );
    let n_ef = (sigma_y / error).powi(2);
    println!(
        "the error is worth the standard error of a random sample of {:.1} people; the poll's margin was {:.4} points, and the error was {:.1} points = {:.0} margins",
        n_ef,
        100.0 * (0.24 * 0.76 / respondents as f64).sqrt(),
        100.0 * error.abs(),
        error.abs() / (0.24 * 0.76 / respondents as f64).sqrt()
    );
    // Gallup and the random samples.
    let mut rng = SplitMix64::new(2026);
    let mut seven = Vec::new();
    for _ in 0..2_000 {
        let mut heads = 0;
        for _ in 0..7 {
            if rng.bernoulli(p_roosevelt) {
                heads += 1;
            }
        }
        seven.push(heads as f64 / 7.0 - p_roosevelt);
    }
    let deviation_seven = (seven.iter().map(|x| x * x).sum::<f64>() / 1999.0).sqrt();
    println!(
        "deviation of 2000 random samples of 7 people = {:.4} ({:.1} points); of 50 000 interviews = {:.4} ({:.2} points)",
        deviation_seven,
        100.0 * deviation_seven,
        sigma_y / 50_000f64.sqrt(),
        100.0 * sigma_y / 50_000f64.sqrt()
    );

    // (2) The filter manufactures relations (Berkson).
    for cutoff in [1.0_f64, 1.5, 2.0] {
        let (pop, sel, frac) = berkson(2026, 400_000, cutoff);
        println!(
            "Berkson with cutoff {:.1}: correlation in the population = {:+.4}, among the selected ({:.1}% of the population) = {:+.4}",
            cutoff, pop, frac, sel
        );
    }

    // (3) Simpson: the two kidney-stone surgeries (Charig and colleagues, 1986).
    let open_small = 81.0 / 87.0;
    let other_small = 234.0 / 270.0;
    let open_large = 192.0 / 263.0;
    let other_large = 55.0 / 80.0;
    let raw_open = (81.0 + 192.0) / (87.0 + 263.0);
    let raw_other = (234.0 + 55.0) / (270.0 + 80.0);
    println!(
        "Simpson: small {:.3} x {:.3}; large {:.3} x {:.3}; raw {:.3} x {:.3} (reverses)",
        open_small, other_small, open_large, other_large, raw_open, raw_other
    );
    let weighted_open = 0.5 * open_small + 0.5 * open_large;
    let weighted_other = 0.5 * other_small + 0.5 * other_large;
    println!(
        "weighted by a half-and-half population: {:.3} x {:.3} (the open surgery wins again)",
        weighted_open, weighted_other
    );

    // (4) The independent unit.
    for persistence in [0.5_f64, 0.8, 0.235] {
        let (var, n_ef) = effective_size(2026, persistence, 100, 20_000);
        println!(
            "two-state chain with persistence {:.3}: variance of the mean of 100 letters = {:.6} (independent: {:.6}), effective size = {:.0}",
            persistence,
            var,
            0.25 / 100.0,
            n_ef
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn berkson_induces_negative_correlation_and_persistence_reduces_effective_n() {
        let (pop, sel, _) = berkson(2026, 20_000, 1.5);
        assert!(pop.abs() < 0.03);
        assert!(sel < -0.30);
        let (_, n_independent) = effective_size(2026, 0.5, 100, 2_000);
        let (_, n_persistent) = effective_size(2026, 0.8, 100, 2_000);
        assert!(n_persistent < n_independent * 0.4);
    }
}
