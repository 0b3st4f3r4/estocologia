//! Calibrating is not understanding: a physical parameter learnt with and without a model of the discrepancy.
//!
//! A simulator says y = theta x. The world says y = theta x / (1 + x / 20): the same slope at the origin, theta = 0.65,
//! and a bend the simulator does not have. Thirty measurements on x in (0, 4], with noise of standard deviation 0.05,
//! are used to learn theta three ways, each a Bayesian regression with known noise and N(0, 1) priors:
//!
//! - **without discrepancy**: y = theta x + noise. The fit looks fine inside the range, and theta absorbs the bend;
//! - **with a discrepancy that knows what it may not do**: y = theta x + d(x) + noise with
//!   d(x) = b2 x^2 + b3 x^3 + b4 x^4, so that d(0) = 0 and d'(0) = 0: the simulator is right at the origin, and the
//!   discrepancy is what grows away from it (Kennedy and O'Hagan, 2001; Brynjarsdottir and O'Hagan, 2014, who show that
//!   such prior knowledge about the discrepancy is what lets the parameter be learnt);
//! - **with a discrepancy that may do anything**: d(x) = b1 x + b2 x^2 + b3 x^3 + b4 x^4. Now theta and b1 enter the
//!   data only through their sum, and the data cannot tell them apart.
//!
//! Declared before its first run in `empirical/p28-discrepancia/DECLARACAO.md`. Deterministic: SplitMix64, seed 2026.

use stochology_core::Rng;

pub const THETA: f64 = 0.65;
pub const POINTS: usize = 30;
pub const NOISE_SD: f64 = 0.05;
pub const DATASETS: usize = 1000;
pub const SEED: u64 = 2026;
pub const EXTRAPOLATE_AT: f64 = 8.0;

pub fn world(x: f64) -> f64 {
    THETA * x / (1.0 + x / 20.0)
}

pub fn design_points() -> Vec<f64> {
    (1..=POINTS)
        .map(|i| 4.0 * i as f64 / POINTS as f64)
        .collect()
}

/// The columns of each model at x: theta first, then the discrepancy terms.
pub fn columns(model: usize, x: f64) -> Vec<f64> {
    match model {
        0 => vec![x],
        1 => vec![x, x * x, x.powi(3), x.powi(4)],
        _ => vec![x, x, x * x, x.powi(3), x.powi(4)],
    }
}

pub const MODELS: [&str; 3] = [
    "without discrepancy",
    "discrepancy with d(0) = d'(0) = 0",
    "discrepancy free at the origin",
];

/// Inverse of a small symmetric positive definite matrix by Gauss-Jordan elimination.
pub fn invert(mut a: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut inv: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    for col in 0..n {
        let pivot = (col..n)
            .max_by(|&i, &j| a[i][col].abs().total_cmp(&a[j][col].abs()))
            .expect("non-empty");
        a.swap(col, pivot);
        inv.swap(col, pivot);
        let p = a[col][col];
        for j in 0..n {
            a[col][j] /= p;
            inv[col][j] /= p;
        }
        for i in 0..n {
            if i != col {
                let f = a[i][col];
                for j in 0..n {
                    a[i][j] -= f * a[col][j];
                    inv[i][j] -= f * inv[col][j];
                }
            }
        }
    }
    inv
}

/// The posterior mean and covariance of the coefficients, with prior N(0, I) and known noise.
pub fn posterior(model: usize, xs: &[f64], ys: &[f64]) -> (Vec<f64>, Vec<Vec<f64>>) {
    let k = columns(model, 1.0).len();
    let mut precision = vec![vec![0.0; k]; k];
    let mut rhs = vec![0.0; k];
    let s2 = NOISE_SD * NOISE_SD;
    for (&x, &y) in xs.iter().zip(ys) {
        let c = columns(model, x);
        for i in 0..k {
            rhs[i] += c[i] * y / s2;
            for j in 0..k {
                precision[i][j] += c[i] * c[j] / s2;
            }
        }
    }
    for (i, row) in precision.iter_mut().enumerate() {
        row[i] += 1.0;
    }
    let cov = invert(precision);
    let mean = (0..k)
        .map(|i| (0..k).map(|j| cov[i][j] * rhs[j]).sum())
        .collect();
    (mean, cov)
}

fn uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn gaussian(rng: &mut Rng) -> f64 {
    let (a, b) = (uniform(rng), uniform(rng));
    (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
}

#[derive(Debug, Clone, Default)]
pub struct Summary {
    pub theta_mean: f64,
    pub theta_sd: f64,
    pub coverage: f64,
    pub rmse_in_range: f64,
    pub extrapolation_error: f64,
}

pub fn summaries(datasets: usize, seed: u64) -> Vec<Summary> {
    let xs = design_points();
    let mut rng = Rng::new(seed);
    let mut acc = vec![Summary::default(); 3];
    for _ in 0..datasets {
        let ys: Vec<f64> = xs
            .iter()
            .map(|&x| world(x) + NOISE_SD * gaussian(&mut rng))
            .collect();
        for (m, a) in acc.iter_mut().enumerate() {
            let (mean, cov) = posterior(m, &xs, &ys);
            let sd = cov[0][0].sqrt();
            a.theta_mean += mean[0];
            a.theta_sd += sd;
            if (mean[0] - THETA).abs() <= 1.96 * sd {
                a.coverage += 1.0;
            }
            let fit = |x: f64| -> f64 { columns(m, x).iter().zip(&mean).map(|(c, b)| c * b).sum() };
            a.rmse_in_range += (xs
                .iter()
                .zip(&ys)
                .map(|(&x, &y)| (fit(x) - y).powi(2))
                .sum::<f64>()
                / xs.len() as f64)
                .sqrt();
            a.extrapolation_error += fit(EXTRAPOLATE_AT) - world(EXTRAPOLATE_AT);
        }
    }
    for a in &mut acc {
        let n = datasets as f64;
        a.theta_mean /= n;
        a.theta_sd /= n;
        a.coverage /= n;
        a.rmse_in_range /= n;
        a.extrapolation_error /= n;
    }
    acc
}

pub fn run() {
    println!(
        "== Calibrating is not understanding: a parameter learnt with and without the discrepancy =="
    );
    println!(
        "world y = {THETA} x / (1 + x / 20) | simulator y = theta x | {POINTS} points on (0, 4], noise sd {NOISE_SD} | priors N(0, 1) | {DATASETS} datasets, seed {SEED}"
    );
    let s = summaries(DATASETS, SEED);
    println!(
        "  model                               | theta mean | theta sd | 95% interval covers {THETA} | fit rmse in range | error at x = {EXTRAPOLATE_AT}"
    );
    for (name, m) in MODELS.iter().zip(&s) {
        println!(
            "  {:<35} |     {:.4} |   {:.4} |                     {:.3} |            {:.4} |       {:+.3}",
            name, m.theta_mean, m.theta_sd, m.coverage, m.rmse_in_range, m.extrapolation_error
        );
    }
    println!(
        "  the world at x = {EXTRAPOLATE_AT}: {:.3}; the noise sd: {NOISE_SD}",
        world(EXTRAPOLATE_AT)
    );
    println!("\n-- the rules declared before the run --");
    let say = |b: bool| if b { "holds" } else { "fails" };
    println!(
        "  R1, without discrepancy: the interval of theta covers it in fewer than half the datasets while the fit in range is within 1.5 noise sds: {}",
        say(s[0].coverage < 0.5 && s[0].rmse_in_range < 1.5 * NOISE_SD)
    );
    println!(
        "  R2, with the discrepancy that knows d(0) = d'(0) = 0: coverage of at least 0.90: {}",
        say(s[1].coverage >= 0.90)
    );
    println!(
        "  R3, with the discrepancy free at the origin: the sd of theta is at least five times the constrained one: {}",
        say(s[2].theta_sd >= 5.0 * s[1].theta_sd)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_inverse_returns_the_identity() {
        let a = vec![
            vec![4.0, 1.0, 0.5],
            vec![1.0, 3.0, 0.2],
            vec![0.5, 0.2, 2.0],
        ];
        let inv = invert(a.clone());
        for i in 0..3 {
            for j in 0..3 {
                let e: f64 = (0..3).map(|k| a[i][k] * inv[k][j]).sum();
                assert!((e - if i == j { 1.0 } else { 0.0 }).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn a_noiseless_line_is_recovered_without_discrepancy() {
        let xs = design_points();
        let ys: Vec<f64> = xs.iter().map(|x| 0.65 * x).collect();
        let (mean, _) = posterior(0, &xs, &ys);
        assert!((mean[0] - 0.65).abs() < 1e-4, "{}", mean[0]);
    }

    #[test]
    fn the_free_discrepancy_cannot_separate_theta_from_its_linear_term() {
        // theta and b1 enter only through their sum: with equal priors their posterior means are equal
        let xs = design_points();
        let ys: Vec<f64> = xs.iter().map(|x| 0.65 * x).collect();
        let (mean, _) = posterior(2, &xs, &ys);
        assert!((mean[0] - mean[1]).abs() < 1e-9);
        assert!((mean[0] + mean[1] - 0.65).abs() < 0.01);
    }
}
