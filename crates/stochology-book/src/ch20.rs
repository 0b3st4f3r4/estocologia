//! Chapter 20 — the live clock, the leakage canary and the vault.
//!
//! Self-contained laboratory, no external dependencies: embedded SplitMix64, logistic regression by reweighted
//! least squares (IRLS), Platt scaling, 10-bin ECE with equal mass and the canary planted on purpose (a column
//! equal to the label, published at the instant of the label).
//! Run from the repository root:
//!     cargo run --release --bin labs -- --chapter 20

const SEED: u64 = 2021;
const N_TRAIN: usize = 120_000;
const N_DEVELOPMENT: usize = 58_000;
const N_VAULT: usize = 24_466;
const BINS: usize = 10;

// The three covariates have the same distribution in the three periods: what changed between the training
// period (2005-2012) and the development and vault periods (2013-2015) was the level of the world.
const P_NIGHT: f64 = 0.18;
const P_SPEED: f64 = 0.28;
const P_PLATE: f64 = 0.12;

// The intercept of the old period (rate ~ 3.6%) and that of the new period (rate ~ 2.3%).
const B0_TRAIN: f64 = -4.005;
const B0_NEW: f64 = -4.42;
const B_NIGHT: f64 = 0.8;
const B_SPEED: f64 = 1.0;
const B_PLATE: f64 = 0.5;

struct Sm64 {
    state: u64,
}

impl Sm64 {
    fn new(seed: u64) -> Self {
        Sm64 {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }
    fn next(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        ((self.next() >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn bernoulli(&mut self, p: f64) -> f64 {
        if self.uniform() < p { 1.0 } else { 0.0 }
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn logit(p: f64) -> f64 {
    (p / (1.0 - p)).ln()
}

struct Period {
    y: Vec<f64>,
    x: Vec<Vec<f64>>,
}

#[allow(clippy::needless_range_loop)] // row-wise construction of the design matrix
fn generate(n: usize, b0: f64, p_night: f64, p_speed: f64, p_plate: f64, rng: &mut Sm64) -> Period {
    let mut y = Vec::with_capacity(n);
    let night = vec![0.0; n];
    let speed = vec![0.0; n];
    let plate = vec![0.0; n];
    let mut x = vec![vec![1.0; n], night, speed, plate];
    for i in 0..n {
        let x1 = rng.bernoulli(p_night);
        let x2 = rng.bernoulli(p_speed);
        let x3 = rng.bernoulli(p_plate);
        x[1][i] = x1;
        x[2][i] = x2;
        x[3][i] = x3;
        let eta = b0 + B_NIGHT * x1 + B_SPEED * x2 + B_PLATE * x3;
        y.push(rng.bernoulli(sigmoid(eta)));
    }
    Period { y, x }
}

// ---------------------------------------------------------------- logistic regression by IRLS

/// Logistic regression by reweighted least squares, with a small ridge outside the intercept.
#[allow(clippy::needless_range_loop)] // reweighted least squares written with explicit indices
fn fit(x: &[Vec<f64>], y: &[f64], lambda: f64) -> Vec<f64> {
    let p = x.len();
    let n = y.len();
    let mut b = vec![0.0; p];
    for _ in 0..60 {
        let mut eta = vec![0.0; n];
        let mut w = vec![0.0; n];
        let mut z = vec![0.0; n];
        for i in 0..n {
            eta[i] = (0..p).map(|c| x[c][i] * b[c]).sum();
            let mu = sigmoid(eta[i]).clamp(1e-9, 1.0 - 1e-9);
            w[i] = (mu * (1.0 - mu)).max(1e-9);
            z[i] = eta[i] + (y[i] - mu) / w[i];
        }
        let mut a = vec![vec![0.0; p + 1]; p];
        for i in 0..n {
            for j in 0..p {
                for k in 0..p {
                    a[j][k] += w[i] * x[j][i] * x[k][i];
                }
                a[j][p] += w[i] * x[j][i] * z[i];
            }
        }
        for j in 1..p {
            a[j][j] += lambda;
        }
        for col in 0..p {
            let mut piv = col;
            for r in (col + 1)..p {
                if a[r][col].abs() > a[piv][col].abs() {
                    piv = r;
                }
            }
            a.swap(col, piv);
            let d = a[col][col];
            for k in col..=p {
                a[col][k] /= d;
            }
            for r in 0..p {
                if r != col {
                    let f = a[r][col];
                    if f != 0.0 {
                        for k in col..=p {
                            a[r][k] -= f * a[col][k];
                        }
                    }
                }
            }
        }
        let new: Vec<f64> = (0..p).map(|j| a[j][p]).collect();
        let step: f64 = (0..p).map(|j| (new[j] - b[j]).abs()).fold(0.0, f64::max);
        b = new;
        if step < 1e-10 {
            break;
        }
    }
    b
}

fn predict(b: &[f64], x: &[Vec<f64>], i: usize) -> f64 {
    sigmoid((0..b.len()).map(|c| x[c][i] * b[c]).sum())
}

// ---------------------------------------------------------------- metrics

fn ece(p: &[f64], y: &[f64]) -> f64 {
    let n = p.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| p[a].partial_cmp(&p[b]).unwrap());
    let size = n / BINS;
    let mut sum = 0.0;
    for c in 0..BINS {
        let start = c * size;
        let stop = if c == BINS - 1 { n } else { (c + 1) * size };
        let m = (stop - start) as f64;
        let mp: f64 = (start..stop).map(|k| p[order[k]]).sum::<f64>() / m;
        let my: f64 = (start..stop).map(|k| y[order[k]]).sum::<f64>() / m;
        sum += m / n as f64 * (mp - my).abs();
    }
    sum
}

fn brier(p: &[f64], y: &[f64]) -> f64 {
    let n = p.len() as f64;
    p.iter().zip(y).map(|(a, b)| (a - b).powi(2)).sum::<f64>() / n
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn citl(p: &[f64], y: &[f64]) -> f64 {
    mean(p) - mean(y)
}

fn skill(p: &[f64], y: &[f64], reference: f64) -> f64 {
    let b_ref: f64 = y.iter().map(|v| (reference - v).powi(2)).sum::<f64>() / y.len() as f64;
    1.0 - brier(p, y) / b_ref
}

pub fn run() {
    println!("== Chapter 20 :: the clock, the canary and the vault ==");
    let mut rng = Sm64::new(SEED);
    let train = generate(N_TRAIN, B0_TRAIN, P_NIGHT, P_SPEED, P_PLATE, &mut rng);
    let development = generate(N_DEVELOPMENT, B0_NEW, P_NIGHT, P_SPEED, P_PLATE, &mut rng);
    let vault = generate(N_VAULT, B0_NEW, P_NIGHT, P_SPEED, P_PLATE, &mut rng);

    println!(
        "seed {} | training {} stops (rate {:.4}) | development {} (rate {:.4}) | vault {} (rate {:.4})",
        SEED,
        N_TRAIN,
        mean(&train.y),
        N_DEVELOPMENT,
        mean(&development.y),
        N_VAULT,
        mean(&vault.y)
    );

    // The raw model: trained in the old regime, applied to the new regime.
    let b_raw = fit(&train.x, &train.y, 1e-4);
    let p_raw: Vec<f64> = (0..N_VAULT).map(|i| predict(&b_raw, &vault.x, i)).collect();
    // The Platt correction: intercept and slope, fitted on the development period.
    let dev_stage: Vec<f64> = (0..N_DEVELOPMENT)
        .map(|i| logit(predict(&b_raw, &development.x, i).clamp(1e-9, 1.0 - 1e-9)))
        .collect();
    let x_platt = vec![vec![1.0; N_DEVELOPMENT], dev_stage];
    let b_platt = fit(&x_platt, &development.y, 1e-6);
    let p_platt: Vec<f64> = (0..N_VAULT)
        .map(|i| {
            let s = logit(p_raw[i].clamp(1e-9, 1.0 - 1e-9));
            sigmoid(b_platt[0] + b_platt[1] * s)
        })
        .collect();

    let rate_dev = mean(&development.y);
    let rate_vault = mean(&vault.y);
    println!(
        "raw model (trained in the old period of higher rate): predicted mean {:.4} ({:.0}% above the vault rate)",
        mean(&p_raw),
        100.0 * (mean(&p_raw) / rate_vault - 1.0)
    );
    println!(
        "  CITL = {:+.4} | ECE = {:.4} | Brier = {:.5} | skill = {:+.4}",
        citl(&p_raw, &vault.y),
        ece(&p_raw, &vault.y),
        brier(&p_raw, &vault.y),
        skill(&p_raw, &vault.y, rate_dev)
    );
    println!(
        "official model with Platt (a = {:.2}, b = {:.2}): predicted mean {:.4}",
        b_platt[1],
        b_platt[0],
        mean(&p_platt)
    );
    println!(
        "  CITL = {:+.4} | ECE = {:.4} | Brier = {:.5} | skill = {:+.4}",
        citl(&p_platt, &vault.y),
        ece(&p_platt, &vault.y),
        brier(&p_platt, &vault.y),
        skill(&p_platt, &vault.y, rate_dev)
    );
    println!(
        "climatology fixed at the past rate: Brier = {:.5} (p(1-p) = {:.5})",
        vault.y.iter().map(|v| (rate_dev - v).powi(2)).sum::<f64>() / N_VAULT as f64,
        rate_dev * (1.0 - rate_dev)
    );

    // ------------------------------------------------------------ the canary
    // The canary column equals the label and only becomes knowable at the instant of the label.
    // With the honest clock it is not published at emission time; with the clock switched off, it is.
    println!("\ncanary: a column equal to the label, published at the instant of the label");
    for honest_clock in [true, false] {
        let mut x_train = train.x.clone();
        let mut x_vault = vault.x.clone();
        let mut canary_rng = Sm64::new(SEED ^ 0xCA11_A710);
        let copy = |y: &[f64], rng: &mut Sm64| -> Vec<f64> {
            // the canary is a copy of the label with 1% of flips: a leak, not the perfect answer
            y.iter()
                .map(|v| if rng.uniform() < 0.01 { 1.0 - v } else { *v })
                .collect()
        };
        if honest_clock {
            x_train.push(vec![0.0; N_TRAIN]);
            x_vault.push(vec![0.0; N_VAULT]);
        } else {
            x_train.push(copy(&train.y, &mut canary_rng));
            x_vault.push(copy(&vault.y, &mut canary_rng));
        }
        let b = fit(&x_train, &train.y, 1e-4);
        let p: Vec<f64> = (0..N_VAULT).map(|i| predict(&b, &x_vault, i)).collect();
        let available = if honest_clock { 0 } else { N_VAULT };
        println!(
            "  clock {}: canary available in {} of {} emissions | beta_canary = {:+.2} | skill gain = {:+.4}",
            if honest_clock {
                "honest"
            } else {
                "switched off"
            },
            available,
            N_VAULT,
            b[4],
            skill(&p, &vault.y, rate_dev) - skill(&p_raw, &vault.y, rate_dev)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sigmoid_inverts_logit_and_brier_measures_squared_error() {
        for p in [0.1, 0.25, 0.5, 0.8] {
            assert!((sigmoid(logit(p)) - p).abs() < 1e-12);
        }
        assert!((brier(&[0.5, 0.5], &[0.0, 1.0]) - 0.25).abs() < 1e-12);
    }
}
