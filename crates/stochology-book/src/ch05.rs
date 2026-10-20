//! Chapter 5 — When the mean lies: convexity, Jensen and the variance toll.
//!
//! Self-contained laboratory: SplitMix64 built in, no external dependencies.
//! Fixed seed of the health centre: 1909 (the same as Lab 0 of the course).
//! Usage: cargo run --release

#[derive(Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self {
            state: stochology_core::rng::offset_seed(seed),
        }
    }
    fn next(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    /// Uniform in [0,1).
    fn uniform(&mut self) -> f64 {
        (self.next() >> 11) as f64 / ((1u64 << 53) as f64)
    }
    /// Exponential with the given mean.
    fn exponential(&mut self, mean: f64) -> f64 {
        let u = self.uniform().max(1e-18);
        -mean * u.ln()
    }
}

/// Statistics of a simulated world.
#[derive(Clone)]
struct Measure {
    patients: usize,
    mean_wait_min: f64,
    median_min: f64,
    p90_min: f64,
    over_30_per_100: f64,
    waits: Vec<f64>,
    days: Vec<Vec<f64>>,
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let pos = q * (sorted.len() as f64 - 1.0);
    let i = pos.floor() as usize;
    let j = (i + 1).min(sorted.len() - 1);
    let frac = pos - i as f64;
    sorted[i] * (1.0 - frac) + sorted[j] * frac
}

/// Simulates the centre: one server with rate mu patients/hour, a 10 hour day
/// starting empty, and an arrival rate that can change from day to day.
fn simulate<F, G>(seed: u64, n_days: usize, hours_per_day: f64, mu_of: G, lambda_of: F) -> Measure
where
    F: Fn(usize) -> f64,
    G: Fn(usize) -> f64,
{
    let mut rng = SplitMix64::new(seed);
    let mut waits: Vec<f64> = Vec::new();
    let mut days: Vec<Vec<f64>> = Vec::with_capacity(n_days);
    for d in 0..n_days {
        let lambda = lambda_of(d);
        let mu = mu_of(d);
        let mut arrivals = Vec::new();
        let mut t = 0.0;
        loop {
            t += rng.exponential(60.0 / lambda);
            if t >= 60.0 * hours_per_day {
                break;
            }
            arrivals.push(t);
        }
        let mut free = 0.0f64;
        let mut of_the_day: Vec<f64> = Vec::with_capacity(arrivals.len());
        for arrival in arrivals {
            let start = arrival.max(free);
            let wait = start - arrival;
            free = start + rng.exponential(60.0 / mu);
            of_the_day.push(wait);
        }
        waits.extend_from_slice(&of_the_day);
        days.push(of_the_day);
    }
    let mut sorted = waits.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = waits.iter().sum::<f64>() / waits.len() as f64;
    let over = waits.iter().filter(|x| **x > 30.0).count() as f64;
    Measure {
        patients: waits.len(),
        mean_wait_min: mean,
        median_min: quantile(&sorted, 0.5),
        p90_min: quantile(&sorted, 0.9),
        over_30_per_100: 100.0 * over / waits.len() as f64,
        waits,
        days,
    }
}

/// Long-run mean wait with a single attendant: W(lambda) = lambda/(mu(mu-lambda)), in hours.
fn w_formula(lambda: f64, mu: f64) -> f64 {
    60.0 * lambda / (mu * (mu - lambda))
}

/// Mean wait over a demand that is `low` or `high` with equal chance: the mean of the two waits.
pub fn mean_wait_two_rates(low: f64, high: f64, mu: f64) -> f64 {
    0.5 * w_formula(low, mu) + 0.5 * w_formula(high, mu)
}

/// Exact mean of W over a uniform rate on [a, b]:
/// 60 / (mu (b - a)) * [(a - b) + mu ln((mu - a)/(mu - b))].
/// Until 2026-10-19 a six-point midpoint rule gave 14.10 min instead of 14.46; a reimplementation found it.
pub fn mean_wait_uniform(a: f64, b: f64, mu: f64) -> f64 {
    60.0 / (mu * (b - a)) * ((a - b) + mu * ((mu - a) / (mu - b)).ln())
}

/// Second derivative of W in lambda: 120 / (mu - lambda)^3.
pub fn wait_curvature(lambda: f64, mu: f64) -> f64 {
    120.0 / (mu - lambda).powi(3)
}

fn variance(xs: &[f64]) -> f64 {
    let m = xs.iter().sum::<f64>() / xs.len() as f64;
    xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0)
}

fn deviation(xs: &[f64]) -> f64 {
    variance(xs).sqrt()
}

/// The halves null: splits the days at random into two halves and measures the deviation
/// of the difference between the means (Phi2 of the book).
fn halves_null(m: &Measure, seed: u64) -> f64 {
    let mut rng = SplitMix64::new(seed);
    let mean_of = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let n_days = m.days.len();
    let mut diffs = Vec::with_capacity(2_000);
    for _ in 0..2_000 {
        let mut a: Vec<f64> = Vec::new();
        let mut b: Vec<f64> = Vec::new();
        for day in m.days.iter() {
            if rng.uniform() < 0.5 {
                a.extend_from_slice(day);
            } else {
                b.extend_from_slice(day);
            }
        }
        diffs.push(mean_of(&a) - mean_of(&b));
    }
    let _ = n_days;
    deviation(&diffs)
}

pub fn run() {
    println!("== Chapter 5 :: when the mean lies ==");

    let mu = 12.0;
    println!(
        "W(5) = {:.2} min, W(8) = {:.2} min, W(11) = {:.2} min",
        w_formula(5.0, mu),
        w_formula(8.0, mu),
        w_formula(11.0, mu)
    );
    let f_mean = w_formula(8.0, mu);
    let e_f_bimodal = mean_wait_two_rates(5.0, 11.0, mu);
    let var_bimodal = 9.0;
    let var_uniform = 3.0;
    let e_f_uniform = mean_wait_uniform(5.0, 11.0, mu);
    let f2 = wait_curvature(8.0, mu);
    println!(
        "Jensen: f(E[X]) = {:.2} min | E[f(X)] = {:.2} (5 or 11) and {:.2} (uniform 5..11)",
        f_mean, e_f_bimodal, e_f_uniform
    );
    println!(
        "second order: half f''(8) Var(X) = {:.2} min (5 or 11) and {:.2} min (uniform)",
        0.5 * f2 * var_bimodal,
        0.5 * f2 * var_uniform
    );

    let seed = 1909u64;
    let days = 4_000usize;
    let hours = 10.0;
    let constant = simulate(seed, days, hours, |_| mu, |_| 8.0);
    let bimodal = simulate(
        seed,
        days,
        hours,
        |_| mu,
        |d| if d % 2 == 0 { 5.0 } else { 11.0 },
    );
    let uniform = simulate(seed, days, hours, |_| mu, |d| 5.0 + ((d * 7919) % 7) as f64);
    let reinforced = simulate(
        seed,
        days,
        hours,
        |d| if d % 2 == 0 { mu } else { 2.0 * mu },
        |d| if d % 2 == 0 { 5.0 } else { 11.0 },
    );

    for (name, m) in [
        ("8 every day", &constant),
        ("5 or 11", &bimodal),
        ("uniform 5..11", &uniform),
        ("5 or 11, surge reinforcement", &reinforced),
    ] {
        println!(
            "{:>28}: {:>7} patients | mean {:>5.1} min | median {:>5.1} | p90 {:>5.1} | >30 min: {:>5.1} per 100",
            name, m.patients, m.mean_wait_min, m.median_min, m.p90_min, m.over_30_per_100
        );
    }

    let surge: Vec<f64> = bimodal
        .days
        .iter()
        .enumerate()
        .filter(|(d, _)| d % 2 == 1)
        .flat_map(|(_, v)| v.iter().copied())
        .collect();
    let mild: Vec<f64> = bimodal
        .days
        .iter()
        .enumerate()
        .filter(|(d, _)| d % 2 == 0)
        .flat_map(|(_, v)| v.iter().copied())
        .collect();
    let mean_of = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    println!(
        "on surge days: {:.1} min in {} patients ({:.1}% of the total); on mild days: {:.1} min",
        mean_of(&surge),
        surge.len(),
        100.0 * surge.len() as f64 / bimodal.patients as f64,
        mean_of(&mild)
    );

    let constant_null = halves_null(&constant, 2026);
    let bimodal_null = halves_null(&bimodal, 2026);
    let n_days = constant.days.len();
    for (name, m) in [("8 every day", &constant), ("5 or 11", &bimodal)] {
        let day_mean: Vec<f64> = m.days.iter().map(|d| mean_of(d)).collect();
        let patient_margin = (variance(&m.waits) * 2.0 / (m.patients as f64 / 2.0)).sqrt();
        let day_margin = (variance(&day_mean) * 2.0 / (n_days as f64 / 2.0)).sqrt();
        let null = if name == "8 every day" {
            constant_null
        } else {
            bimodal_null
        };
        println!(
            "halves null in the {:>11} world: measured deviation {:.3} min | margin per day {:.3} | per patient {:.3} | wait variance {:.0} min^2",
            name,
            null,
            day_margin,
            patient_margin,
            variance(&m.waits)
        );
    }
    println!(
        "difference between the worlds = {:.1} min = {:.1} deviations of the null",
        bimodal.mean_wait_min - constant.mean_wait_min,
        (bimodal.mean_wait_min - constant.mean_wait_min) / constant_null
    );
    println!(
        "median of the 5 or 11 world = {:.1} min, against the mean wait of {:.1} min of the constant world",
        bimodal.median_min, constant.mean_wait_min
    );

    let mut rng = SplitMix64::new(1909);
    let mut sum = 0.0;
    let mut below = 0usize;
    let n = 100_000usize;
    for _ in 0..n {
        let u = rng.uniform();
        let center = if u < 0.45 { 4.0 } else { 8.0 };
        let u1 = rng.uniform().max(1e-18);
        let u2 = rng.uniform();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let score = center + z;
        sum += score;
        if score < 6.0 {
            below += 1;
        }
    }
    println!(
        "class of {} students: mean = {:.2}, failing (score < 6.0) = {:.1} per 100",
        n,
        sum / n as f64,
        100.0 * below as f64 / n as f64
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jensen_in_the_queue_varies_above_the_constant() {
        let constant = simulate(1909, 200, 10.0, |_| 12.0, |_| 8.0);
        let variable = simulate(
            1909,
            200,
            10.0,
            |_| 12.0,
            |d| if d % 2 == 0 { 5.0 } else { 11.0 },
        );
        assert!(variable.mean_wait_min > constant.mean_wait_min);
        assert!((w_formula(8.0, 12.0) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn the_uniform_jensen_mean_is_exact() {
        let mu = 12.0_f64;
        let exact = 60.0 / (mu * 6.0) * (-6.0 + mu * 7.0_f64.ln());
        let fine: f64 = (0..600_000)
            .map(|i| w_formula(5.0 + 6.0 * (i as f64 + 0.5) / 600_000.0, mu))
            .sum::<f64>()
            / 600_000.0;
        assert!((exact - fine).abs() < 1e-6, "{exact} {fine}");
        assert!((exact - 14.459).abs() < 1e-3);
    }
}
