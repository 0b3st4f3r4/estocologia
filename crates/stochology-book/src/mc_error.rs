//! The Monte Carlo error of the numbers the book draws at random.
//!
//! A simulated quantile or a simulated share is itself an estimate: run again with another seed, it moves. A fixed
//! seed makes the number reproducible, not exact. This instrument measures how far each simulated number that a
//! reading of the book depends on moves, in two ways. For a quantile, it reruns the same simulation under twenty
//! other seeds and reports the spread, together with the distribution-free interval that the order statistics of
//! the published run already give. For a share counted over N independent draws, it reports the exact
//! Clopper-Pearson interval. Then it says whether the reading the book draws from the number survives the spread.

use crate::streams::{self, Binary, NULL_SEED, NULL_SIMULATIONS};

pub const SEEDS: u64 = 20;

/// Twenty seeds other than the published one, fixed and listed: base + 1 .. base + 20.
pub fn seeds(base: u64) -> Vec<u64> {
    (1..=SEEDS).map(|k| base.wrapping_add(k)).collect()
}

/// Mean, sample standard deviation, minimum and maximum.
pub fn spread(v: &[f64]) -> (f64, f64, f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
    let lo = v.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (m, sd, lo, hi)
}

/// The distribution-free 95% interval of the q-quantile from N sorted draws: the order statistics at
/// N q -/+ 1.96 sqrt(N q (1 - q)), clamped to the sample.
pub fn order_interval(sorted: &[f64], q: f64) -> (f64, f64) {
    let n = sorted.len() as f64;
    let half = 1.96 * (n * q * (1.0 - q)).sqrt();
    let lo = ((n * q - half).floor().max(1.0) as usize - 1).min(sorted.len() - 1);
    let hi = ((n * q + half).ceil() as usize).clamp(1, sorted.len()) - 1;
    (sorted[lo], sorted[hi])
}

fn ln_factorials(n: usize) -> Vec<f64> {
    let mut f = vec![0.0; n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] + (i as f64).ln();
    }
    f
}

/// P(X <= k) for X ~ Binomial(n, p).
fn binomial_cdf(k: usize, n: usize, p: f64, lf: &[f64]) -> f64 {
    if p <= 0.0 {
        return 1.0;
    }
    if p >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    (0..=k.min(n))
        .map(|i| {
            (lf[n] - lf[i] - lf[n - i] + i as f64 * p.ln() + (n - i) as f64 * (1.0 - p).ln()).exp()
        })
        .sum::<f64>()
        .min(1.0)
}

/// The exact 95% Clopper-Pearson interval of a share of x successes in n draws.
pub fn clopper_pearson(x: usize, n: usize) -> (f64, f64) {
    let lf = ln_factorials(n);
    let solve = |target: &dyn Fn(f64) -> f64| -> f64 {
        let (mut a, mut b) = (0.0f64, 1.0f64);
        for _ in 0..80 {
            let m = 0.5 * (a + b);
            if target(m) > 0.0 { a = m } else { b = m }
        }
        0.5 * (a + b)
    };
    let lo = if x == 0 {
        0.0
    } else {
        // P(X >= x | lo) = 0.025
        solve(&|p| 0.025 - (1.0 - binomial_cdf(x - 1, n, p, &lf)))
    };
    let hi = if x == n {
        1.0
    } else {
        // P(X <= x | hi) = 0.025
        solve(&|p| binomial_cdf(x, n, p, &lf) - 0.025)
    };
    (lo, hi)
}

/// One binary vault of Chapters 8 and 20: its name, its verdict, and the spread of its null q95 over the seeds.
#[derive(Debug, Clone)]
pub struct VaultAudit {
    pub name: &'static str,
    pub ece: f64,
    pub published_q95: f64,
    pub order_interval: (f64, f64),
    pub seeds: (f64, f64, f64, f64),
    pub above_in: usize,
}

pub fn audit_vault(name: &'static str, b: &Binary) -> VaultAudit {
    let draws = streams::ece_null_draws(&b.forecasts, NULL_SIMULATIONS, NULL_SEED);
    let q95s: Vec<f64> = seeds(NULL_SEED)
        .into_iter()
        .map(|s| streams::ece_null_q95(&b.forecasts, NULL_SIMULATIONS, s))
        .collect();
    VaultAudit {
        name,
        ece: b.ece,
        published_q95: b.null_q95,
        order_interval: order_interval(&draws, 0.95),
        seeds: spread(&q95s),
        above_in: q95s.iter().filter(|q| b.ece > **q).count(),
    }
}

/// The shares the book publishes from a count of independent simulated draws: chapter, what, the share as printed,
/// and the number of draws.
pub fn shares() -> Vec<(&'static str, &'static str, f64, usize)> {
    vec![
        (
            "7",
            "garden of paths, Simmons' four freedoms: false positives",
            0.203,
            4000,
        ),
        (
            "7",
            "garden of paths, the large family: false positives",
            0.423,
            4000,
        ),
        ("7", "pre-registered test: false positives", 0.052, 4000),
        ("8", "null ECE above 0.03 at Toronto's size", 0.451, 2000),
        (
            "10",
            "investors with no loss in 250 weeks (all-in under the raw frequency)",
            0.368,
            10_000,
        ),
        (
            "10",
            "investors ruined in 1,000 weeks under the raw frequency",
            0.0194,
            10_000,
        ),
        (
            "13",
            "replicas of the claim world as bad as the real index (p)",
            0.0140,
            2000,
        ),
    ]
}

pub fn run() {
    println!("== The Monte Carlo error of the simulated numbers ==");
    println!(
        "each simulated quantile rerun under {SEEDS} seeds other than the published one; each simulated share with its exact 95% interval"
    );

    println!(
        "\n-- the null q95 of the ECE in the rebuilt binary vaults (Chapters 8 and 20; {NULL_SIMULATIONS} simulations each) --"
    );
    let mut vaults = Vec::new();
    let f13 = streams::flights_vault(streams::FLIGHTS_2013, 2013);
    vaults.push(audit_vault("flights 2013, raw", &f13.raw));
    let f23 = streams::flights_vault(streams::FLIGHTS_2023, 2023);
    vaults.push(audit_vault("flights 2023, corrected", &f23.corrected));
    let (cy, _) = streams::cyclones();
    vaults.push(audit_vault("cyclones 2015-2022", &cy));
    let ar = streams::arrests();
    vaults.push(audit_vault("Toronto 2001-2002", &ar.verdict));
    for v in &vaults {
        println!(
            "  {:<24} ECE {:.4} | published q95 {:.4}, order-statistic 95% interval [{:.4}; {:.4}] | {SEEDS} seeds: mean {:.4}, sd {:.5}, range [{:.4}; {:.4}] | ECE above q95 in {} of {SEEDS}",
            v.name,
            v.ece,
            v.published_q95,
            v.order_interval.0,
            v.order_interval.1,
            v.seeds.0,
            v.seeds.1,
            v.seeds.2,
            v.seeds.3,
            v.above_in
        );
    }

    println!(
        "\n-- the simulated null at Toronto's size in the laboratory of Chapter 8 (2,000 simulations each) --"
    );
    let f = crate::ch08::toronto_like_forecasts();
    let published = crate::ch08::ece_tolerance(20261015, &f, 2_000);
    let q95s: Vec<f64> = seeds(20261015)
        .into_iter()
        .map(|s| crate::ch08::ece_tolerance(s, &f, 2_000).1)
        .collect();
    let (m, sd, lo, hi) = spread(&q95s);
    println!(
        "  published q95 {:.4} | {SEEDS} seeds: mean {:.4}, sd {:.5}, range [{:.4}; {:.4}] | Toronto's real ECE 0.0354 below the simulated q95 in {} of {SEEDS}",
        published.1,
        m,
        sd,
        lo,
        hi,
        q95s.iter().filter(|q| 0.0354 < **q).count()
    );

    println!("\n-- the bootstrap interval of the tail index in Chapter 14 (300 replicas each) --");
    let (plo, phi) = crate::ch14::hill_interval(18_001);
    let ivs: Vec<(f64, f64)> = seeds(18_001)
        .into_iter()
        .map(crate::ch14::hill_interval)
        .collect();
    let los: Vec<f64> = ivs.iter().map(|i| i.0).collect();
    let his: Vec<f64> = ivs.iter().map(|i| i.1).collect();
    let (ml, sl, ll, hl) = spread(&los);
    let (mh, sh, lh, hh) = spread(&his);
    println!(
        "  published [{plo:.2}; {phi:.2}] | {SEEDS} seeds: lower mean {ml:.3} sd {sl:.3} range [{ll:.2}; {hl:.2}], upper mean {mh:.3} sd {sh:.3} range [{lh:.2}; {hh:.2}] | the interval contains 2 in {} of {SEEDS}",
        ivs.iter().filter(|i| i.0 < 2.0 && i.1 > 2.0).count()
    );

    println!("\n-- shares counted over independent draws, with the exact 95% interval --");
    for (ch, what, share, n) in shares() {
        let x = (share * n as f64).round() as usize;
        let (lo, hi) = clopper_pearson(x, n);
        let sd = (share * (1.0 - share) / n as f64).sqrt();
        println!(
            "  Chapter {ch:>2}: {what}: {share:.4} of {n} (about {x}) | Monte Carlo sd {sd:.4} | exact 95% interval [{lo:.4}; {hi:.4}]"
        );
    }

    let count =
        |test: &dyn Fn(&str) -> bool| SEED_REGISTER.iter().filter(|(_, how)| test(how)).count();
    let own = count(&|h| h.starts_with("its own"));
    let audited = count(&|h| h.contains("--mc-error"));
    let spread = count(&|h| h.contains("--seed-spread"));
    let not_audited = count(&|h| h.starts_with("not audited"));
    let other: Vec<&str> = SEED_REGISTER
        .iter()
        .filter(|(_, h)| {
            !h.starts_with("its own")
                && !h.contains("--mc-error")
                && !h.contains("--seed-spread")
                && !h.starts_with("not audited")
        })
        .map(|(file, _)| *file)
        .collect();
    println!(
        "\n-- the register of seeds: {} modules draw random numbers | {own} print their own spread | {audited} are audited here | {spread} are rerun whole by --seed-spread | {not_audited} are not audited | {} of another kind ({}) --",
        SEED_REGISTER.len(),
        other.len(),
        other.join(", ")
    );
}

/// How the Monte Carlo error of each module that draws random numbers is reported (item 18 (g), 2026-10-19). A test
/// reads every source file of the crate and refuses one that draws random numbers and is not listed here, so that a new
/// stochastic laboratory cannot enter without saying where its spread is measured, or that it is not.
pub const SEED_REGISTER: [(&str, &str); 27] = [
    (
        "banco.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch01.rs",
        "the chapter is about seeds: it prints three, and a test checks that two seeds almost never match",
    ),
    (
        "ch04.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch05.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch06.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch07.rs",
        "shares of the gardens of paths: exact interval in --mc-error",
    ),
    (
        "ch08.rs",
        "the null q95 at Toronto's size: twenty seeds in --mc-error",
    ),
    (
        "ch09.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch10.rs",
        "shares of the investors: exact interval in --mc-error",
    ),
    (
        "ch11.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch12.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch13.rs",
        "the p of the claim world: exact interval in --mc-error",
    ),
    (
        "ch14.rs",
        "the bootstrap interval of the tail index: twenty seeds in --mc-error",
    ),
    (
        "ch15.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch16.rs",
        "a hash of the card's identifier, not a draw: the triage is a deterministic stand-in",
    ),
    (
        "ch17.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch19.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "ch20.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "discrepancy.rs",
        "averages over 1,000 datasets; the rms error at x = 8 carries the spread of one dataset",
    ),
    (
        "ensaio.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "fila.rs",
        "whole run rerun under twenty seeds in --seed-spread (docs/SEED_SPREAD.md)",
    ),
    (
        "multiverse.rs",
        "its own: the Monte Carlo sd of each specification is printed",
    ),
    (
        "passk.rs",
        "its own: the band of 2,000 repetitions is printed",
    ),
    (
        "ponte.rs",
        "a selection, not an estimate: the seed draws the cases, and a test checks it",
    ),
    (
        "propagate.rs",
        "its own: the Monte Carlo sd of each probability is printed",
    ),
    (
        "stacking.rs",
        "its own: the interval of the paired difference over 1,000 datasets is printed",
    ),
    (
        "streams.rs",
        "the null q95 of the rebuilt vaults: twenty seeds in --mc-error",
    ),
];

/// Whether a source draws random numbers: a generator is built or a step of SplitMix64 is taken.
pub fn draws_random_numbers(source: &str) -> bool {
    ["Rng::new", "splitmix", "SplitMix", "fn rng", "struct Rng"]
        .iter()
        .any(|needle| source.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clopper_pearson_matches_known_values() {
        // 0 of 10: the upper bound is 1 - 0.025^(1/10) = 0.3085
        let (lo, hi) = clopper_pearson(0, 10);
        assert_eq!(lo, 0.0);
        assert!((hi - 0.308_45).abs() < 1e-4, "{hi}");
        // 5 of 10: [0.1871; 0.8129]
        let (lo, hi) = clopper_pearson(5, 10);
        assert!(
            (lo - 0.187_1).abs() < 1e-3 && (hi - 0.812_9).abs() < 1e-3,
            "{lo} {hi}"
        );
    }

    #[test]
    fn the_order_interval_brackets_the_quantile() {
        let v: Vec<f64> = (0..2000).map(|i| i as f64).collect();
        let (lo, hi) = order_interval(&v, 0.95);
        assert!(lo < 1899.0 && hi > 1899.0 && hi - lo < 50.0, "{lo} {hi}");
    }

    #[test]
    fn every_module_that_draws_random_numbers_says_where_its_spread_is_measured() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut unlisted = Vec::new();
        let mut drawing = 0;
        for entry in std::fs::read_dir(&dir).expect("the source directory") {
            let path = entry.expect("an entry").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            if !name.ends_with(".rs")
                || name == "main.rs"
                || name == "lib.rs"
                || name == "mc_error.rs"
            {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("readable");
            if draws_random_numbers(&source) {
                drawing += 1;
                if !SEED_REGISTER.iter().any(|(file, _)| *file == name) {
                    unlisted.push(name);
                }
            }
        }
        assert!(
            unlisted.is_empty(),
            "draws random numbers and is not in SEED_REGISTER: {unlisted:?}"
        );
        assert_eq!(
            drawing,
            SEED_REGISTER.len(),
            "a listed module no longer draws, or was removed"
        );
    }

    #[test]
    fn the_seeds_are_twenty_and_none_is_the_published_one() {
        let s = seeds(2026);
        assert_eq!(s.len(), 20);
        assert!(!s.contains(&2026));
    }
}
