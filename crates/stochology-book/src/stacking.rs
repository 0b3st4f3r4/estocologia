//! Combining models when none of them is true: Bayesian model averaging against stacking.
//!
//! Two candidate models of one quantity, each a normal with known spread 1 and a centre its author is nearly sure of:
//! the left model puts its prior on a centre at -2 with standard deviation 0.05, the right model at +2. They are two
//! confident experts, each right about one part of the world. Bayesian model averaging (BMA) weights each model by its
//! posterior probability, the prior times the marginal likelihood; when the truth is one of the candidates, that weight
//! goes to it, as it should. When the truth is neither (here, half the data come from each side), BMA still picks one
//! of them, because it answers "which model is true?", a question with no right answer. Stacking (Yao, Vehtari,
//! Simpson and Gelman, 2018) answers another: which mixture of the predictive distributions predicts best? It chooses
//! the weights that maximise the logarithmic score of the leave-one-out predictive densities, a proper scoring rule
//! (Chapter 8) used to combine, not only to judge. And a model listed twice changes BMA, which doubles its prior mass,
//! but not stacking, which only sees predictions.
//!
//! Every density is in closed form (conjugate normal), so the leave-one-out predictive is exact. Declared before its
//! first run in `empirical/p27-stacking/DECLARACAO.md`. Deterministic: SplitMix64, seed 2026.

use stochology_core::Rng;

pub const N_TRAIN: usize = 40;
pub const N_TEST: usize = 2000;
pub const DATASETS: usize = 1000;
pub const SEED: u64 = 2026;
pub const PRIOR_SD: f64 = 0.05;
pub const CENTRES: [f64; 2] = [-2.0, 2.0];
const ITERATIONS: usize = 500;

/// A candidate: a normal of spread 1 whose centre has prior N(centre, PRIOR_SD^2).
#[derive(Debug, Clone, Copy)]
pub struct Model {
    pub centre: f64,
}

fn log_normal_density(y: f64, mean: f64, var: f64) -> f64 {
    -0.5 * ((2.0 * std::f64::consts::PI * var).ln() + (y - mean).powi(2) / var)
}

impl Model {
    /// The posterior of the centre after observations with sum `sum` and count `n`: (mean, variance).
    pub fn posterior(&self, sum: f64, n: usize) -> (f64, f64) {
        let precision = 1.0 / (PRIOR_SD * PRIOR_SD) + n as f64;
        let var = 1.0 / precision;
        (var * (self.centre / (PRIOR_SD * PRIOR_SD) + sum), var)
    }

    /// The log predictive density of y after observations with sum `sum` and count `n`.
    pub fn log_predictive(&self, y: f64, sum: f64, n: usize) -> f64 {
        let (m, v) = self.posterior(sum, n);
        log_normal_density(y, m, 1.0 + v)
    }

    /// The log marginal likelihood of the data, by the chain rule of the sequential predictives.
    pub fn log_marginal(&self, ys: &[f64]) -> f64 {
        let mut sum = 0.0;
        let mut total = 0.0;
        for (i, &y) in ys.iter().enumerate() {
            total += self.log_predictive(y, sum, i);
            sum += y;
        }
        total
    }
}

/// BMA weights with equal prior probability per listed model.
pub fn bma_weights(models: &[Model], ys: &[f64]) -> Vec<f64> {
    let logs: Vec<f64> = models.iter().map(|m| m.log_marginal(ys)).collect();
    let top = logs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let raw: Vec<f64> = logs.iter().map(|l| (l - top).exp()).collect();
    let total: f64 = raw.iter().sum();
    raw.iter().map(|r| r / total).collect()
}

/// Stacking weights: the mixture weights that maximise the sum of the log leave-one-out predictive densities, by the
/// fixed point w_k <- mean_i w_k p_ki / sum_j w_j p_ji, which never lowers the objective.
pub fn stacking_weights(models: &[Model], ys: &[f64]) -> Vec<f64> {
    let sum: f64 = ys.iter().sum();
    let n = ys.len();
    let dens: Vec<Vec<f64>> = ys
        .iter()
        .map(|&y| {
            models
                .iter()
                .map(|m| m.log_predictive(y, sum - y, n - 1).exp())
                .collect()
        })
        .collect();
    let k = models.len();
    let mut w = vec![1.0 / k as f64; k];
    for _ in 0..ITERATIONS {
        let mut next = vec![0.0; k];
        for row in &dens {
            let mix: f64 = row
                .iter()
                .zip(&w)
                .map(|(p, wk)| p * wk)
                .sum::<f64>()
                .max(1e-300);
            for j in 0..k {
                next[j] += w[j] * row[j] / mix;
            }
        }
        w = next.iter().map(|x| x / n as f64).collect();
    }
    w
}

/// The mean log score per point of a mixture of the full-data predictives on test data.
pub fn test_score(models: &[Model], w: &[f64], train: &[f64], test: &[f64]) -> f64 {
    let sum: f64 = train.iter().sum();
    let n = train.len();
    test.iter()
        .map(|&y| {
            models
                .iter()
                .zip(w)
                .map(|(m, wk)| wk * m.log_predictive(y, sum, n).exp())
                .sum::<f64>()
                .max(1e-300)
                .ln()
        })
        .sum::<f64>()
        / test.len() as f64
}

fn uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn gaussian(rng: &mut Rng) -> f64 {
    let (a, b) = (uniform(rng), uniform(rng));
    (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
}

/// A world: the truth draws each point from N(-2, 1) with chance `left`, else from N(+2, 1).
pub fn draw(rng: &mut Rng, left: f64, n: usize) -> Vec<f64> {
    (0..n)
        .map(|_| {
            let centre = if uniform(rng) < left { -2.0 } else { 2.0 };
            centre + gaussian(rng)
        })
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct WorldResult {
    pub bma: Vec<f64>,
    pub stacking: Vec<f64>,
    pub best_single: Vec<f64>,
    pub bma_left_weight: Vec<f64>,
    pub stacking_left_weight: Vec<f64>,
    pub dup_stacking: Vec<f64>,
    pub dup_bma: Vec<f64>,
    pub dup_stacking_left_family: Vec<f64>,
    pub dup_bma_left_family: Vec<f64>,
}

pub fn world(left: f64, datasets: usize, seed: u64) -> WorldResult {
    let mut rng = Rng::new(seed);
    let two = [Model { centre: CENTRES[0] }, Model { centre: CENTRES[1] }];
    let three = [two[0], two[0], two[1]];
    let mut r = WorldResult::default();
    for _ in 0..datasets {
        let train = draw(&mut rng, left, N_TRAIN);
        let test = draw(&mut rng, left, N_TEST);
        let wb = bma_weights(&two, &train);
        let ws = stacking_weights(&two, &train);
        r.bma.push(test_score(&two, &wb, &train, &test));
        r.stacking.push(test_score(&two, &ws, &train, &test));
        let singles = [
            test_score(&two, &[1.0, 0.0], &train, &test),
            test_score(&two, &[0.0, 1.0], &train, &test),
        ];
        r.best_single.push(singles[0].max(singles[1]));
        r.bma_left_weight.push(wb[0]);
        r.stacking_left_weight.push(ws[0]);
        let wb3 = bma_weights(&three, &train);
        let ws3 = stacking_weights(&three, &train);
        r.dup_bma.push(test_score(&three, &wb3, &train, &test));
        r.dup_stacking.push(test_score(&three, &ws3, &train, &test));
        r.dup_bma_left_family.push(wb3[0] + wb3[1]);
        r.dup_stacking_left_family.push(ws3[0] + ws3[1]);
    }
    r
}

pub fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// The mean of the differences a - b and its 95% interval by the normal approximation over datasets.
pub fn paired(a: &[f64], b: &[f64]) -> (f64, f64, f64) {
    let d: Vec<f64> = a.iter().zip(b).map(|(x, y)| x - y).collect();
    let m = mean(&d);
    let sd = (d.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (d.len() as f64 - 1.0)).sqrt();
    let half = 1.96 * sd / (d.len() as f64).sqrt();
    (m, m - half, m + half)
}

fn mean_abs_diff(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum::<f64>() / a.len() as f64
}

pub fn run() {
    println!("== Combining models when none is true: Bayesian model averaging against stacking ==");
    println!(
        "candidates: N(c, 1) with c ~ N(-2, {PRIOR_SD}^2) (left) and c ~ N(+2, {PRIOR_SD}^2) (right) | {N_TRAIN} training points, {N_TEST} test points, {DATASETS} datasets, seed {SEED} | score: mean log predictive density per test point"
    );
    let open = world(0.5, DATASETS, SEED);
    let closed = world(1.0, DATASETS, SEED + 1);
    let mut verdicts = Vec::new();
    for (name, r) in [
        (
            "M-open: half the points from each side, neither candidate is true",
            &open,
        ),
        (
            "M-closed: every point from the left, the left candidate is true",
            &closed,
        ),
    ] {
        let (d, lo, hi) = paired(&r.stacking, &r.bma);
        println!("\n-- {name} --");
        println!(
            "  log score: BMA {:.4} | stacking {:.4} | best single model {:.4} | stacking - BMA {:+.4} [{:+.4}; {:+.4}]",
            mean(&r.bma),
            mean(&r.stacking),
            mean(&r.best_single),
            d,
            lo,
            hi
        );
        let certain = r
            .bma_left_weight
            .iter()
            .filter(|w| **w < 0.01 || **w > 0.99)
            .count();
        println!(
            "  weight on the left model: BMA mean {:.3} (above 0.99 or below 0.01 in {} of {DATASETS}) | stacking mean {:.3}",
            mean(&r.bma_left_weight),
            certain,
            mean(&r.stacking_left_weight)
        );
        println!(
            "  left model listed twice: stacking score moves by {:.5} on average, its left family weighs {:.3} (was {:.3}) | BMA score moves by {:.5}, its left family weighs {:.3} (was {:.3})",
            mean_abs_diff(&r.dup_stacking, &r.stacking),
            mean(&r.dup_stacking_left_family),
            mean(&r.stacking_left_weight),
            mean_abs_diff(&r.dup_bma, &r.bma),
            mean(&r.dup_bma_left_family),
            mean(&r.bma_left_weight)
        );
        verdicts.push((d, lo, hi, r));
    }
    println!("\n-- the rules declared before the run --");
    let say = |b: bool| if b { "holds" } else { "fails" };
    let o = &verdicts[0];
    let c = &verdicts[1];
    println!(
        "  R1, M-open: stacking scores above BMA, with the 95% interval of the difference above zero: {}",
        say(o.1 > 0.0)
    );
    let r2 = [o.3, c.3].iter().all(|r| {
        mean_abs_diff(&r.dup_stacking, &r.stacking) < 0.001
            && (mean(&r.dup_stacking_left_family) - mean(&r.stacking_left_weight)).abs() < 0.01
    });
    println!(
        "  R2, both worlds: listing the left model twice moves stacking's score by less than 0.001 and its left-family weight by less than 0.01: {}",
        say(r2)
    );
    println!(
        "  R3, M-closed: the two differ by less than 0.01 in mean log score: {}",
        say(c.0.abs() < 0.01)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sequential_predictives_give_the_closed_form_marginal_likelihood() {
        // the marginal of y under N(c, 1), c ~ N(m0, s0^2) is a joint normal; for one point it is N(m0, 1 + s0^2)
        let m = Model { centre: -2.0 };
        let one = m.log_marginal(&[0.3]);
        assert!((one - log_normal_density(0.3, -2.0, 1.0 + PRIOR_SD * PRIOR_SD)).abs() < 1e-12);
        // the chain rule does not depend on the order of the data
        let a = m.log_marginal(&[0.3, -1.0, 2.5]);
        let b = m.log_marginal(&[2.5, 0.3, -1.0]);
        assert!((a - b).abs() < 1e-10);
    }

    #[test]
    fn stacking_finds_the_best_mixture_on_a_grid() {
        // the fixed point must reach the maximum of the leave-one-out log score over the weight on the left model
        let models = [Model { centre: -2.0 }, Model { centre: 2.0 }];
        let ys: Vec<f64> = (0..30)
            .map(|i| {
                if i < 10 {
                    -2.0 + 0.1 * i as f64
                } else {
                    2.0 - 0.05 * i as f64
                }
            })
            .collect();
        let sum: f64 = ys.iter().sum();
        let score = |w: f64| -> f64 {
            ys.iter()
                .map(|&y| {
                    let a = models[0].log_predictive(y, sum - y, ys.len() - 1).exp();
                    let b = models[1].log_predictive(y, sum - y, ys.len() - 1).exp();
                    (w * a + (1.0 - w) * b).ln()
                })
                .sum()
        };
        let w = stacking_weights(&models, &ys);
        let best_grid = (0..=1000)
            .map(|i| score(i as f64 / 1000.0))
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(
            score(w[0]) >= best_grid - 1e-6,
            "{} {}",
            score(w[0]),
            best_grid
        );
        assert!((w.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_duplicated_model_doubles_its_prior_mass_in_bma() {
        let models = [Model { centre: -2.0 }, Model { centre: 2.0 }];
        let three = [models[0], models[0], models[1]];
        let ys = [-0.1, 0.2, 0.05];
        let w2 = bma_weights(&models, &ys);
        let w3 = bma_weights(&three, &ys);
        let a = w2[0] / w2[1];
        assert!(((w3[0] + w3[1]) / w3[2] - 2.0 * a).abs() < 1e-9);
    }
}
