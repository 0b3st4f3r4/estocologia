//! Reliability is not average success: pass^k against the heterogeneity of the tasks.
//!
//! An agent is run n times on each of N tasks. Its average success, pass^1, is the share of runs that succeed. Whoever
//! delegates a task asks another question: will it succeed every time it is asked? That is pass^k, the chance that k
//! runs of the same task all succeed. If task i succeeds with chance p_i, then pass^1 = E[p] and pass^k = E[p^k], and
//! by Jensen's inequality E[p^k] >= (E p)^k, with equality only when every task has the same chance. The distance
//! between pass^k and (pass^1)^k is the variance of the chances across tasks at work, the same term that the variance
//! of a sum carries in Chapter 6: two agents with the same pass^1 can differ by an order of magnitude in pass^8.
//!
//! Per task, the unbiased estimator of p_i^k from c successes in n runs is C(c, k) / C(n, k); pass^k is its mean over
//! the tasks. The instrument draws the chances of four worlds from Beta distributions, runs the agent, and repeats the
//! whole experiment to give the Monte Carlo spread of each reading. Declared before its first run in
//! `empirical/p25-passk/DECLARACAO.md`. Deterministic: SplitMix64, seed 2026.

use stochology_core::Rng;

pub const TASKS: usize = 200;
pub const RUNS: usize = 8;
pub const REPLICATES: usize = 2000;
pub const SEED: u64 = 2026;

/// A world: its name and the law of the chances of its tasks, Beta(a, b), or one chance `fixed` for every task.
#[derive(Debug, Clone, Copy)]
pub struct World {
    pub name: &'static str,
    pub a: f64,
    pub b: f64,
    pub fixed: Option<f64>,
}

pub fn worlds() -> [World; 4] {
    [
        World {
            name: "A, every task at 0.5",
            a: 0.0,
            b: 0.0,
            fixed: Some(0.5),
        },
        World {
            name: "B, Beta(2, 2)",
            a: 2.0,
            b: 2.0,
            fixed: None,
        },
        World {
            name: "C, Beta(0.3, 0.3)",
            a: 0.3,
            b: 0.3,
            fixed: None,
        },
        World {
            name: "D, Beta(1.4, 0.6)",
            a: 1.4,
            b: 0.6,
            fixed: None,
        },
    ]
}

/// E[p^k] exactly: 1 for k = 0, the product of (a + j) / (a + b + j) for j < k, or fixed^k.
pub fn exact_moment(w: &World, k: usize) -> f64 {
    match w.fixed {
        Some(p) => p.powi(k as i32),
        None => (0..k)
            .map(|j| (w.a + j as f64) / (w.a + w.b + j as f64))
            .product(),
    }
}

/// C(n, k) as a float; zero when k > n.
pub fn choose(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    (0..k).map(|j| (n - j) as f64 / (k - j) as f64).product()
}

/// The unbiased estimator of pass^k from the success counts of each task over `runs` runs.
pub fn pass_k(counts: &[usize], runs: usize, k: usize) -> f64 {
    let denom = choose(runs, k);
    counts.iter().map(|&c| choose(c, k) / denom).sum::<f64>() / counts.len() as f64
}

fn uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn gaussian(rng: &mut Rng) -> f64 {
    let (a, b) = (uniform(rng), uniform(rng));
    (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
}

/// Gamma(shape, 1) by Marsaglia and Tsang (2000); a shape below one is boosted by U^(1/shape).
pub fn gamma(rng: &mut Rng, shape: f64) -> f64 {
    if shape < 1.0 {
        let g = gamma(rng, shape + 1.0);
        return g * uniform(rng).powf(1.0 / shape);
    }
    let d = shape - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let x = gaussian(rng);
        let v = (1.0 + c * x).powi(3);
        if v <= 0.0 {
            continue;
        }
        let u = uniform(rng);
        if u.ln() < 0.5 * x * x + d - d * v + d * v.ln() {
            return d * v;
        }
    }
}

fn chance(rng: &mut Rng, w: &World) -> f64 {
    match w.fixed {
        Some(p) => p,
        None => {
            let x = gamma(rng, w.a);
            let y = gamma(rng, w.b);
            x / (x + y)
        }
    }
}

/// One experiment: N tasks, their chances drawn from the world, `runs` runs of each; the success counts.
pub fn experiment(rng: &mut Rng, w: &World, tasks: usize, runs: usize) -> Vec<usize> {
    (0..tasks)
        .map(|_| {
            let p = chance(rng, w);
            (0..runs).filter(|_| uniform(rng) < p).count()
        })
        .collect()
}

/// The 2.5% and 97.5% order statistics of a sample.
pub fn interval(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((p * (v.len() - 1) as f64).round()) as usize];
    (q(0.025), q(0.975))
}

/// The readings of one world: per k, the estimate of the first replicate, its 95% band over the replicates, the band
/// of the gap pass^k - (pass^1)^k, and the exact E[p^k].
#[derive(Debug, Clone)]
pub struct Reading {
    pub k: usize,
    pub first: f64,
    pub first_independent: f64,
    pub band: (f64, f64),
    pub gap_band: (f64, f64),
    pub exact: f64,
}

pub fn readings(
    w: &World,
    tasks: usize,
    runs: usize,
    replicates: usize,
    seed: u64,
) -> Vec<Reading> {
    let mut rng = Rng::new(seed);
    let all: Vec<Vec<usize>> = (0..replicates)
        .map(|_| experiment(&mut rng, w, tasks, runs))
        .collect();
    (1..=runs)
        .map(|k| {
            let est: Vec<f64> = all.iter().map(|c| pass_k(c, runs, k)).collect();
            let gaps: Vec<f64> = all
                .iter()
                .map(|c| pass_k(c, runs, k) - pass_k(c, runs, 1).powi(k as i32))
                .collect();
            Reading {
                k,
                first: est[0],
                first_independent: pass_k(&all[0], runs, 1).powi(k as i32),
                band: interval(est),
                gap_band: interval(gaps),
                exact: exact_moment(w, k),
            }
        })
        .collect()
}

pub fn run() {
    println!(
        "== Reliability is not average success: pass^k against the heterogeneity of the tasks =="
    );
    println!(
        "{TASKS} tasks, {RUNS} runs of each; pass^k = mean over tasks of C(c, k) / C({RUNS}, k); {REPLICATES} replicates of the whole experiment, seed {SEED}"
    );
    let mut verdicts = Vec::new();
    for w in worlds() {
        let r = readings(&w, TASKS, RUNS, REPLICATES, SEED);
        println!("\n-- world {} --", w.name);
        println!(
            "   k | first run: pass^k  (pass^1)^k | 95% band of pass^k  | exact E[p^k] | 95% band of the gap"
        );
        for x in &r {
            println!(
                "  {:>2} |          {:.4}      {:.4}  | [{:.4}; {:.4}]  |     {:.4}   | [{:+.4}; {:+.4}]",
                x.k,
                x.first,
                x.first_independent,
                x.band.0,
                x.band.1,
                x.exact,
                x.gap_band.0,
                x.gap_band.1
            );
        }
        let exact_inside = r.iter().all(|x| x.band.0 <= x.exact && x.exact <= x.band.1);
        let gap_zero_inside = r
            .iter()
            .skip(1)
            .all(|x| x.gap_band.0 <= 0.0 && 0.0 <= x.gap_band.1);
        let gap_above = r.iter().skip(1).all(|x| x.gap_band.0 > 0.0);
        let last = r.last().expect("k = RUNS");
        println!(
            "  agent card: reliability k = {RUNS}; pass^1 {:.4}; pass^{RUNS} {:.4}; read as independent {:.6}; ratio {:.1}",
            r[0].first,
            last.first,
            last.first_independent,
            last.first / last.first_independent
        );
        println!(
            "  exact moment inside the band for every k: {exact_inside} | gap band contains 0 for every k >= 2: {gap_zero_inside} | gap band above 0 for every k >= 2: {gap_above}"
        );
        verdicts.push((w.name, exact_inside, gap_zero_inside, gap_above));
    }
    println!("\n-- the rules declared before the run --");
    let r1 = verdicts[0].2;
    let r2 = verdicts[1..].iter().all(|v| v.3);
    let r3 = verdicts.iter().all(|v| v.1);
    println!(
        "  R1, world A: the gap band contains 0 for every k >= 2: {}",
        if r1 { "holds" } else { "fails" }
    );
    println!(
        "  R2, worlds B, C and D: the gap band lies above 0 for every k >= 2: {}",
        if r2 { "holds" } else { "fails" }
    );
    println!(
        "  R3, every world: the exact E[p^k] lies inside the band of the estimator for every k: {}",
        if r3 { "holds" } else { "fails" }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_estimator_is_exact_on_known_counts() {
        // 8 of 8 gives 1 for every k; 4 of 8 gives C(4, 2) / C(8, 2) = 6 / 28 at k = 2; 0 of 8 gives 0
        assert_eq!(pass_k(&[8], 8, 5), 1.0);
        assert!((pass_k(&[4], 8, 2) - 6.0 / 28.0).abs() < 1e-12);
        assert_eq!(pass_k(&[0], 8, 1), 0.0);
        assert!((pass_k(&[8, 0], 8, 1) - 0.5).abs() < 1e-12);
        assert_eq!(choose(3, 5), 0.0);
    }

    #[test]
    fn jensen_holds_for_the_beta_moments_and_binds_only_without_spread() {
        for w in worlds() {
            for k in 1..=8 {
                let m = exact_moment(&w, k);
                let independent = exact_moment(&w, 1).powi(k as i32);
                assert!(m >= independent - 1e-15, "{} {k}", w.name);
                if w.fixed.is_some() {
                    assert!((m - independent).abs() < 1e-15);
                }
            }
        }
        // Beta(2, 2): E[p^2] = 2 * 3 / (4 * 5) = 0.3
        assert!((exact_moment(&worlds()[1], 2) - 0.3).abs() < 1e-12);
    }

    #[test]
    fn the_gamma_sampler_has_the_right_mean_below_and_above_one() {
        let mut rng = Rng::new(7);
        for shape in [0.3, 2.0] {
            let n = 40_000;
            let m = (0..n).map(|_| gamma(&mut rng, shape)).sum::<f64>() / n as f64;
            assert!((m - shape).abs() < 0.05 * shape.max(1.0), "{shape} {m}");
        }
    }
}
