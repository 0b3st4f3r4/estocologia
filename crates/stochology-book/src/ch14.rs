//! Chapter 14 — H1, part 2: the tail of the Danish fires and the art of failing one's own model.
//!
//! Self-contained, no external dependencies: reads the public dataset, estimates Hill on the real tail and on a
//! simulated tail, applies the Laplace rule to the 36-month clean window and measures the evidence in bits.
//! Run from the repository root:
//!     cargo run --release --bin labs -- --chapter 17

const RETENTION: f64 = 50.0; // layer "100 xs 50", in M DKK of 1985
const U_POT: f64 = 10.0; // threshold of the POT fit of the repository
const N_EXC: f64 = 83.0; // excesses above U_POT in the training set (h1-real/saida/c/resultados.json)
const SIGMA_POT: f64 = 7.387414319903844; // sigma of the published GPD fit
const XI_POT: f64 = 0.4064637172276745; // xi of the published GPD fit
const P_POT: f64 = 0.042961531409410614; // monthly p implied by the training tail (published)
const TRAIN_YEARS: i32 = 9; // 1980-1988
const WINDOW: usize = 36; // clean months that the claim reads
const P_NEGLIGIBLE: f64 = 1.0 / 1200.0; // "negligible risk": one month hit per century
const N_BOOT: usize = 300; // bootstrap replicas by monthly blocks

#[derive(Clone)]
struct Sm64 {
    state: u64,
}

impl Sm64 {
    fn new(s: u64) -> Self {
        Sm64 { state: s }
    }
    fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

/// Hill estimator with the k largest positive observations (delegates to `stochology-horizons`).
fn hill(x: &[f64], k: usize) -> f64 {
    let n_pos = x.iter().filter(|v| **v > 0.0).count();
    stochology_horizons::h1::hill(x, k.min(n_pos.saturating_sub(1)))
}

fn quantile(mut v: Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((p * v.len() as f64).ceil() as usize).max(1).min(v.len()) - 1;
    v[i]
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn sd(x: &[f64]) -> f64 {
    let m = mean(x);
    (x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt()
}

/// Laplace rule of succession: (k+1)/(n+2).
fn laplace(k: usize, n: usize) -> f64 {
    (k as f64 + 1.0) / (n as f64 + 2.0)
}

/// Evidence of a clean window of n months: likelihood ratio "negligible" against "dangerous".
fn evidence_bits(n: usize, p_dangerous: f64) -> (f64, f64, f64) {
    let p0_negligible = (1.0 - P_NEGLIGIBLE).powi(n as i32);
    let p0_danger = (1.0 - p_dangerous).powi(n as i32);
    let lr = p0_negligible / p0_danger;
    (lr, lr.log2(), p0_danger)
}

/// Mutual information, in bits, between "did the window come out clean?" and the 50/50 hypothesis dangerous against negligible.
fn mutual_information(n: usize, p_dangerous: f64) -> f64 {
    let pc = [
        (1.0 - p_dangerous).powi(n as i32),
        (1.0 - P_NEGLIGIBLE).powi(n as i32),
    ];
    let mut mi = 0.0;
    for (i, hypothesis) in [0.5f64, 0.5].iter().enumerate() {
        for j in 0..2 {
            let p = hypothesis * if j == 0 { pc[i] } else { 1.0 - pc[i] };
            let px = 0.5 * if j == 0 { pc[0] } else { 1.0 - pc[0] }
                + 0.5 * if j == 0 { pc[1] } else { 1.0 - pc[1] };
            if p > 0.0 {
                mi += p * (p / (0.5 * px)).log2();
            }
        }
    }
    mi
}

/// Clean months needed for "accusing the danger" to have probability 95%.
fn months_for_95(p: f64) -> usize {
    (0.05f64.ln() / (1.0 - p).ln()).ceil() as usize
}

struct Data {
    all: Vec<f64>,
    months_top: Vec<(String, f64)>,
    by_month: Vec<Vec<f64>>,
}

/// Reads the dataset: losses in chronological order, the largest loss of each month and the list of losses per month.
fn read_data(path: &str) -> Data {
    let text = std::fs::read_to_string(path).expect("did not read the fires CSV");
    let mut all = Vec::new();
    let mut months_top: Vec<(String, f64)> = Vec::new();
    let mut by_month: Vec<Vec<f64>> = (0..(TRAIN_YEARS * 12) as usize)
        .map(|_| Vec::new())
        .collect();
    for (i, line) in text.lines().enumerate() {
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split(',').collect();
        let loss: f64 = c[1].trim().parse().unwrap();
        let month = c[0].trim()[..7].to_string();
        all.push(loss);
        match months_top.last_mut() {
            Some((m, mx)) if *m == month => {
                if loss > *mx {
                    *mx = loss;
                }
            }
            _ => months_top.push((month.clone(), loss)),
        }
        if &month[..4] < "1989" {
            let idx = (month[..4].parse::<i32>().unwrap() - 1980) as usize * 12
                + month[5..7].parse::<usize>().unwrap()
                - 1;
            by_month[idx].push(loss);
        }
    }
    Data {
        all,
        months_top,
        by_month,
    }
}

/// Months of the period, months in which the layer was hit and the largest loss of the period.
fn period(d: &Data, y0: i32, y1: i32) -> (usize, usize, f64) {
    let of_period: Vec<&(String, f64)> = d
        .months_top
        .iter()
        .filter(|(m, _)| {
            let year: i32 = m[..4].parse().unwrap();
            year >= y0 && year <= y1
        })
        .collect();
    let hit = of_period.iter().filter(|(_, mx)| *mx > RETENTION).count();
    let largest = of_period.iter().map(|(_, mx)| *mx).fold(0.0f64, f64::max);
    (of_period.len(), hit, largest)
}

/// Hill k=50 with the months of the training set resampled with replacement (bootstrap of monthly blocks).
fn bootstrap_hill(d: &Data, k: usize, reps: usize, seed: u64) -> Vec<f64> {
    let mut rng = Sm64::new(seed);
    let mut out = Vec::with_capacity(reps);
    for _ in 0..reps {
        let mut sample = Vec::new();
        for _ in 0..d.by_month.len() {
            let j = (rng.uniform() * d.by_month.len() as f64) as usize % d.by_month.len();
            sample.extend(d.by_month[j].iter().cloned());
        }
        out.push(hill(&sample, k));
    }
    out
}

/// The 95% bootstrap interval of Hill k=50 on the training months, as `run` prints it, under any seed.
pub(crate) fn hill_interval(seed: u64) -> (f64, f64) {
    let d = read_data(&data_path("data/danish_fire.csv"));
    let rep = bootstrap_hill(&d, 50, N_BOOT, seed);
    (quantile(rep.clone(), 0.025), quantile(rep, 0.975))
}

/// Simulated tail: Pareto with tail alpha, same sample size, Hill k in each repetition.
fn simulated_tail(alpha: f64, n: usize, k: usize, reps: usize, seed: u64) -> Vec<f64> {
    let mut rng = Sm64::new(seed);
    let mut out = Vec::with_capacity(reps);
    for _ in 0..reps {
        let sample: Vec<f64> = (0..n)
            .map(|_| (1.0 - rng.uniform()).powf(-1.0 / alpha))
            .collect();
        out.push(hill(&sample, k));
    }
    out
}

/// Resolves the data path: optional base in ESTOCOLOGIA_DADOS, otherwise the repository.
fn data_path(default: &str) -> String {
    match std::env::var("ESTOCOLOGIA_DADOS") {
        Ok(base) => format!("{}/{}", base.trim_end_matches('/'), default),
        Err(_) => default.to_string(),
    }
}

pub fn run() {
    let path = data_path("data/danish_fire.csv");
    let d = read_data(&path);

    println!("== Chapter 14 :: the tail, the clean window and the information ==");
    println!(
        "Copenhagen Re fires above 1 M DKK, 1980-1990: {} claims in {} months",
        d.all.len(),
        d.months_top.len()
    );

    // (1) months in which the layer 100 xs 50 was hit, by period
    println!();
    println!(
        "{:<12} {:>7} {:>17} {:>13}",
        "period", "months", "months hit", "largest loss"
    );
    for (name, y0, y1) in [
        ("1980-1982", 1980, 1982),
        ("1983-1985", 1983, 1985),
        ("1986-1988", 1986, 1988),
        ("1989-1990", 1989, 1990),
    ] {
        let (m, hit, largest) = period(&d, y0, y1);
        println!("{:<12} {:>7} {:>17} {:>13.1}", name, m, hit, largest);
    }
    print!("months hit and value:");
    for (m, mx) in d.months_top.iter().filter(|(_, mx)| *mx > RETENTION) {
        print!(" {} {:.1};", m, mx);
    }
    println!();

    // (2) Hill on the tail of the training set (1980-1988) and the interval by bootstrap of monthly blocks
    let train: Vec<f64> = d.by_month.concat();
    println!();
    println!(
        "training 1980-1988: {} claims in {} months",
        train.len(),
        TRAIN_YEARS * 12
    );
    print!("Hill on the training set:");
    for k in [25usize, 50, 100] {
        print!(" k={} alpha={:.4}", k, hill(&train, k));
    }
    println!();
    let rep = bootstrap_hill(&d, 50, N_BOOT, 18_001);
    println!(
        "Hill k=50, 95% CI by bootstrap of monthly blocks ({} replicas): [{:.2}; {:.2}]",
        N_BOOT,
        quantile(rep.clone(), 0.025),
        quantile(rep, 0.975)
    );

    // (3) the same estimator on a simulated tail with the estimated alpha
    let alpha = 1.81f64;
    let sim = simulated_tail(alpha, 1500, 50, 200, 18_002);
    println!(
        "simulated Pareto with alpha = {:.2}, n = 1500, 200 repetitions: mean Hill k=50 = {:.3} (sd {:.3}), 90% of the repetitions between {:.2} and {:.2}",
        alpha,
        mean(&sim),
        sd(&sim),
        quantile(sim.clone(), 0.05),
        quantile(sim, 0.95)
    );

    // (4) the clean window: Laplace, evidence in bits and months for a severe test
    println!();
    println!("window of {} months without a claim on the layer:", WINDOW);
    println!(
        "Laplace on the window: p = (0+1)/(36+2) = {:.5}, or 1 chance in {:.0} per month",
        laplace(0, WINDOW),
        1.0 / laplace(0, WINDOW)
    );
    println!(
        "monthly frequency measured in the training set: p = 5/108 = {:.5}",
        5.0 / 108.0
    );
    println!(
        "monthly p of the training tail (published POT/GPD fit: xi = {:.3}, sigma = {:.3}, {} excesses above {:.0}): p = {:.5}",
        XI_POT, SIGMA_POT, N_EXC as usize, U_POT, P_POT
    );
    for (name, p) in [("measured frequency", 5.0 / 108.0), ("POT/GPD tail", P_POT)] {
        let (lr, bits, p0) = evidence_bits(WINDOW, p);
        println!(
            "  {:<18} p = {:.5}: P(clean window | dangerous) = {:.4}; likelihood ratio = {:.2} ({:.2} bits); clean months for 95% = {}",
            name,
            p,
            p0,
            lr,
            bits,
            months_for_95(p)
        );
        println!(
            "  {:<18} mutual information window x (dangerous vs negligible) = {:.4} bits",
            "",
            mutual_information(WINDOW, p)
        );
    }
    let (lr_vault, bits_vault, _) = evidence_bits(WINDOW, 2.0 / 24.0);
    println!(
        "reference: under the rate that the vault showed (2 months hit in 24, p = 1/12), the same clean window would be worth {:.2} bits (ratio {:.1})",
        bits_vault, lr_vault
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laplace_avoids_zero_and_hill_recovers_tail() {
        assert!((laplace(0, 36) - 1.0 / 38.0).abs() < 1e-12);
        let (_, bits, _) = evidence_bits(36, 5.0 / 108.0);
        assert!((bits - 2.42).abs() < 0.02);
        let xs = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0];
        assert!(hill(&xs, 3).is_finite());
    }
}
