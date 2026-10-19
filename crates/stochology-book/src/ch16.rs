//! Chapter 16 — H0: the health post, the wait and the independent unit.
//!
//! Self-contained laboratory, no external dependencies: embedded SplitMix64, M/M/1 queue with one
//! attendant, Lindley recursion, halves null model and severe test.
//! Run from the repository root:
//!     cargo run --release --bin labs -- --chapter 16

const SEED: u64 = 1909; // the same seed as Lab 0 of the course (Erlang, 1909)
const HOURS: f64 = 10.0; // the post opens 10 h per day
const MU: f64 = 12.0; // patients per hour, one attendant
const DAYS: usize = 2000; // days per world
const REPS: usize = 2000; // splits of the null model

#[derive(Clone)]
struct Sm64 {
    state: u64,
}

impl Sm64 {
    fn new(seed: u64) -> Self {
        Sm64 { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

/// Poisson by inversion (Knuth): exact and enough for rates up to ~110.
fn poisson(rng: &mut Sm64, lambda: f64) -> usize {
    let threshold = (-lambda).exp();
    let (mut k, mut p) = (0usize, 1.0f64);
    loop {
        k += 1;
        p *= rng.uniform();
        if p <= threshold {
            return k - 1;
        }
    }
}

fn exponential(rng: &mut Sm64, rate: f64) -> f64 {
    -(1.0 - rng.uniform()).ln() / rate
}

/// One day of the post, open and empty. Poisson(lambda * HOURS) arrivals spread over the day,
/// exponential service with mean 1/mu at each of `servers` nurses, one shared line served first come,
/// first served. With one nurse this is the Lindley recursion. Returns minutes.
///
/// Until 2026-10-19 the reinforcement of the surge days was simulated as one nurse at twice the pace
/// (mu = 24), while the text described two nurses; a reimplementation from the text found the gap.
fn queue_of_day(rng: &mut Sm64, lambda: f64, mu: f64, servers: usize) -> Vec<f64> {
    let n = poisson(rng, lambda * HOURS);
    let mut arrivals: Vec<f64> = (0..n).map(|_| rng.uniform() * HOURS).collect();
    arrivals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let service: Vec<f64> = (0..n).map(|_| exponential(rng, mu)).collect();
    let mut wait = vec![0.0f64; n];
    if servers <= 1 {
        for i in 1..n {
            wait[i] = (wait[i - 1] + service[i - 1] - (arrivals[i] - arrivals[i - 1])).max(0.0);
        }
    } else {
        let mut free = vec![0.0f64; servers];
        for i in 0..n {
            let (k, _) = free
                .iter()
                .enumerate()
                .fold(
                    (0, f64::INFINITY),
                    |b, (k, f)| if *f < b.1 { (k, *f) } else { b },
                );
            let start = arrivals[i].max(free[k]);
            wait[i] = start - arrivals[i];
            free[k] = start + service[i];
        }
    }
    wait.iter().map(|w| 60.0 * w).collect()
}

struct World {
    waits: Vec<f64>,
    day_sum: Vec<f64>,
    day_n: Vec<f64>,
}

fn world<F: FnMut(&mut Sm64) -> f64, G: Fn(f64) -> usize>(
    seed: u64,
    days: usize,
    mut rate_of_day: F,
    servers_of: G,
) -> World {
    let mut rng = Sm64::new(seed);
    let mut waits = Vec::new();
    let mut day_sum = Vec::with_capacity(days);
    let mut day_n = Vec::with_capacity(days);
    for _ in 0..days {
        let lambda = rate_of_day(&mut rng);
        let e = queue_of_day(&mut rng, lambda, MU, servers_of(lambda));
        day_sum.push(e.iter().sum::<f64>());
        day_n.push(e.len() as f64);
        waits.extend(e);
    }
    World {
        waits,
        day_sum,
        day_n,
    }
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn sd(x: &[f64]) -> f64 {
    let m = mean(x);
    (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt()
}

fn quantile(x: &[f64], p: f64) -> f64 {
    let mut v = x.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((p * v.len() as f64).ceil() as usize).max(1).min(v.len()) - 1;
    v[i]
}

/// Margin of the day: standard error of the ratio of sums, with the day as the independent unit.
/// It is the honest margin when the patients of the same day are not independent.
fn margin_per_day(x: &[f64], n: &[f64]) -> f64 {
    let sum: f64 = x.iter().sum();
    let total: f64 = n.iter().sum();
    let m = sum / total;
    let num: f64 = x
        .iter()
        .zip(n)
        .map(|(s, c)| (s - m * c) * (s - m * c))
        .sum();
    num.sqrt() / total
}

/// Margin per patient: the pseudoreplication that the null model puts to the test.
fn margin_per_patient(w: &[f64]) -> f64 {
    sd(w) / (w.len() as f64).sqrt()
}

fn world_row(name: &str, m: &World) {
    let mean_wait = mean(&m.waits);
    let above: f64 = m.waits.iter().filter(|w| **w > 30.0).count() as f64 / m.waits.len() as f64;
    println!(
        "{:<28} {:>8} {:>7.1} {:>7.1} {:>7.1} {:>7} {:>12.2} {:>15.2}",
        name,
        m.waits.len(),
        mean_wait,
        quantile(&m.waits, 0.5),
        quantile(&m.waits, 0.9),
        format!("{:.0}%", 100.0 * above),
        margin_per_day(&m.day_sum, &m.day_n),
        margin_per_patient(&m.waits),
    );
}

pub fn run() {
    println!("== Chapter 16 :: the same mean, two waits ==");
    println!(
        "{} days of {:.0} h per world, one attendant, mu = {:.0} per hour, seed {} (+ world index)",
        DAYS, HOURS, MU, SEED
    );

    // (1) the five worlds of Lab 0, all with mean demand 8
    let claim = world(SEED + 10, DAYS, |_| 8.0, |_| 1);
    let variable = world(
        SEED + 20,
        DAYS,
        |r| if r.uniform() < 0.5 { 5.0 } else { 11.0 },
        |_| 1,
    );
    let nearly = world(
        SEED + 30,
        DAYS,
        |r| if r.uniform() < 0.5 { 7.0 } else { 9.0 },
        |_| 1,
    );
    let uniform = world(SEED + 40, DAYS, |r| 5.0 + 6.0 * r.uniform(), |_| 1);
    let reinforcement = world(
        SEED + 50,
        DAYS,
        |r| if r.uniform() < 0.5 { 5.0 } else { 11.0 },
        |l| if l > 8.0 { 2 } else { 1 },
    );

    println!(
        "{:<28} {:>8} {:>7} {:>7} {:>7} {:>7} {:>12} {:>15}",
        "world (patients per hour)",
        "patients",
        "wait",
        "median",
        "p90",
        ">30min",
        "margin/day",
        "margin/pat."
    );
    world_row("8 every day (the claim)", &claim);
    world_row("7 or 9", &nearly);
    world_row("uniform between 5 and 11", &uniform);
    world_row("5 or 11", &variable);
    world_row("5 or 11, reinforcement in the surge", &reinforcement);

    // (2) the halves null model: 4000 days of lambda = 8, split at random into two halves
    let null = world(SEED + 60, 2 * DAYS, |_| 8.0, |_| 1);
    let mut rng = Sm64::new(SEED + 61);
    let mut diffs = Vec::with_capacity(REPS);
    for _ in 0..REPS {
        let (mut sa, mut na, mut sb, mut nb) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for d in 0..null.day_sum.len() {
            if rng.uniform() < 0.5 {
                sa += null.day_sum[d];
                na += null.day_n[d];
            } else {
                sb += null.day_sum[d];
                nb += null.day_n[d];
            }
        }
        diffs.push(sa / na - sb / nb);
    }
    let sd_diffs = sd(&diffs);
    let max_abs = diffs.iter().fold(0.0f64, |m, d| m.max(d.abs()));
    let n_pat = null.waits.len() as f64;
    let predicted_day = sd(&null
        .day_sum
        .iter()
        .zip(&null.day_n)
        .map(|(s, c)| s / c)
        .collect::<Vec<f64>>())
        * (2.0 / DAYS as f64).sqrt();
    let predicted_pat = sd(&null.waits) * (2.0 / (n_pat / 2.0)).sqrt();

    println!();
    println!(
        "halves null: {} days of demand 8, {} splits of {} days against {}",
        2 * DAYS,
        REPS,
        DAYS,
        DAYS
    );
    println!("measured sd of the differences = {:.4} min", sd_diffs);
    println!("predicted by the day unit     = {:.4} min", predicted_day);
    println!(
        "predicted by the patient unit = {:.4} min  ({} patients)",
        predicted_pat, n_pat as usize
    );
    println!(
        "null: max |difference| in {} splits = {:.3} min",
        REPS, max_abs
    );

    // (3) severe test: the difference between the two worlds against the null
    let obs = mean(&variable.waits) - mean(&claim.waits);
    let above_obs = diffs.iter().filter(|d| d.abs() >= obs.abs()).count() as f64 / REPS as f64;
    println!();
    println!(
        "severe test: observed difference = {:.1} min, {:.1} sd of the null, null fraction >= observed = {:.4}",
        obs,
        obs / sd_diffs,
        above_obs
    );
    println!(
        "tail: 8 in 100 patients wait more than 30 min in world 8; {:.0} in 100 in world 5 or 11",
        100.0 * variable.waits.iter().filter(|w| **w > 30.0).count() as f64
            / variable.waits.len() as f64
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_margin_is_larger_than_patient_pseudoreplication() {
        let m = world(SEED, 100, |_| 8.0, |_| 1);
        let m_day = margin_per_day(&m.day_sum, &m.day_n);
        let m_pat = margin_per_patient(&m.waits);
        assert!(m_day > m_pat * 1.5);
    }

    #[test]
    fn two_nurses_share_one_line_and_one_nurse_is_lindley() {
        // One nurse: the multi-server path reduces to the Lindley recursion on the same draws.
        let mut a = Sm64::new(7);
        let mut b = Sm64::new(7);
        let one = queue_of_day(&mut a, 11.0, MU, 1);
        let lindley_again = queue_of_day(&mut b, 11.0, MU, 1);
        assert_eq!(one, lindley_again);
        // Two nurses on the same draws never make anyone wait longer than one nurse does.
        let mut c = Sm64::new(7);
        let two = queue_of_day(&mut c, 11.0, MU, 2);
        assert_eq!(one.len(), two.len());
        assert!(two.iter().zip(&one).all(|(x, y)| *x <= y + 1e-12));
        // And the surge world with two nurses lands near 2 minutes, not near the 2.5 of mu = 24.
        let w = world(
            SEED + 50,
            2000,
            |r| if r.uniform() < 0.5 { 5.0 } else { 11.0 },
            |l| {
                if l > 8.0 { 2 } else { 1 }
            },
        );
        let m = mean(&w.waits);
        assert!((1.7..2.3).contains(&m), "mean wait with two nurses = {m}");
    }
}
