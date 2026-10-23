//! Lens 4 — Structure and Invariance: Pearl's Causal Ladder, Invariant Causal Prediction (ICP),
//! NILE Estimator, and Hidden Confounding Threshold $\Lambda^*$ (§1.3.4).
//!
//! Implements:
//! - The three rungs of Judea Pearl's Causal Ladder (`1-See`, `2-Act`, `3-Imagine`);
//! - Invariant Causal Prediction (ICP) of Peters, Bühlmann & Meinshausen:
//!   $$Y \perp E \mid X_S \implies \hat{S}^* = \bigcap_{S : \text{invariant}} S$$
//! - NILE (*Nonlinear Invariant Risk Minimization* / instrumental invariance score, Christiansen et al.);
//! - Second-order expansion of worst-case interventional loss $L_{\mathrm{wc}}(\theta) \approx L_0 + \nabla_\theta L^\top \theta + \frac{1}{2}\theta^\top H_\theta \theta$
//!   and critical hidden confounding sensitivity threshold $\Lambda^*$.

use stochology_core::{mean, sample_variance};

/// Rung on Judea Pearl's Causal Ladder (§1.3.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PearlRung {
    /// 1st Rung — Passive association $P(y \mid x)$ (*See*).
    SeeAssociation,
    /// 2nd Rung — Active intervention $P(y \mid \mathrm{do}(x))$ (*Act*).
    ActIntervention,
    /// 3rd Rung — Counterfactual imagination $P(y_x \mid x', y')$ (*Imagine*).
    ImagineCounterfactual,
}

impl PearlRung {
    pub const fn code(self) -> &'static str {
        match self {
            Self::SeeAssociation => "1-See",
            Self::ActIntervention => "2-Act",
            Self::ImagineCounterfactual => "3-Imagine",
        }
    }
}

/// Data observed within a historical regime or interventional environment $e \in \mathcal{E}$.
#[derive(Debug, Clone)]
pub struct CausalEnvironment {
    pub name: String,
    /// Candidate predictor matrix: each row is an observation with $p$ variables.
    pub x: Vec<Vec<f64>>,
    /// Outcome vector $Y$.
    pub y: Vec<f64>,
}

/// Output of Invariant Causal Prediction (ICP) and NILE across environments $e \in \mathcal{E}$.
#[derive(Debug, Clone, PartialEq)]
pub struct IcpResult {
    /// Subsets $S \subseteq \{0, \dots, p-1\}$ whose slopes and residual distributions remain invariant across all environments.
    pub invariant_subsets: Vec<Vec<usize>>,
    /// Structural intersection accepted by ICP: $\hat{S}^* = \bigcap_{S : \text{invariant}} S$.
    pub accepted_causal_predictors: Vec<usize>,
    /// NILE invariance penalty score per candidate variable.
    pub nile_scores_by_variable: Vec<f64>,
}

/// Executes Invariant Causal Prediction (ICP, Peters, Bühlmann & Meinshausen)
/// coupled with the NILE instrumental invariance penalty (Christiansen et al.) across the provided environments.
pub fn run_icp(environments: &[CausalEnvironment], invariance_tolerance: f64) -> IcpResult {
    assert!(
        environments.len() >= 2,
        "ICP requires at least 2 distinct environments"
    );
    assert!(
        !environments[0].x.is_empty(),
        "environments must contain observations"
    );
    let p = environments[0].x[0].len();
    assert!(
        (1..=12).contains(&p),
        "dimension p for subset search must be in 1..=12"
    );
    for env in environments {
        assert!(
            env.x.len() >= 2 && env.x.len() == env.y.len(),
            "each environment must have at least 2 observations and matching x and y lengths"
        );
        assert!(
            env.x.iter().all(|row| row.len() == p),
            "all rows across all environments must have uniform feature dimension p"
        );
    }

    // NILE instrumental invariance penalty per variable j (Christiansen et al.):
    // Fits the pooled structural candidate on variable j, then evaluates the environment-projected
    // Wald / two-stage residual shift and slope divergence across environments e in E.
    let mut nile_scores = vec![0.0; p];
    for (j, score_slot) in nile_scores.iter_mut().enumerate() {
        let mut pooled_xj = Vec::new();
        let mut pooled_y = Vec::new();
        for env in environments {
            for (row, &yi) in env.x.iter().zip(env.y.iter()) {
                pooled_xj.push(row[j]);
                pooled_y.push(yi);
            }
        }
        let (beta_pool, alpha_pool, _) = univariate_ols_fit(&pooled_xj, &pooled_y);
        let mut betas = Vec::with_capacity(environments.len());
        let mut res_sds = Vec::with_capacity(environments.len());
        let mut env_proj_penalty = 0.0;
        for env in environments {
            let col_j: Vec<f64> = env.x.iter().map(|row| row[j]).collect();
            let (b, _, sd) = univariate_ols_fit(&col_j, &env.y);
            let mean_res_e: f64 = col_j
                .iter()
                .zip(env.y.iter())
                .map(|(&xi, &yi)| yi - (alpha_pool + beta_pool * xi))
                .sum::<f64>()
                / (col_j.len() as f64);
            let var_x_e = sample_variance(&col_j);
            env_proj_penalty +=
                mean_res_e * mean_res_e + (b - beta_pool) * (b - beta_pool) * var_x_e;
            betas.push(b);
            res_sds.push(sd);
        }
        *score_slot = (env_proj_penalty / (environments.len() as f64))
            + sample_variance(&betas)
            + sample_variance(&res_sds);
    }

    let mut invariant_subsets = Vec::new();
    let total_masks = 1usize << p;

    for mask in 1..total_masks {
        let subset: Vec<usize> = (0..p).filter(|&bit| (mask & (1 << bit)) != 0).collect();
        if is_subset_invariant(environments, &subset, invariance_tolerance) {
            invariant_subsets.push(subset);
        }
    }

    let accepted = if invariant_subsets.is_empty() {
        Vec::new()
    } else {
        let mut inter = invariant_subsets[0].clone();
        for s in &invariant_subsets[1..] {
            inter.retain(|item| s.contains(item));
        }
        inter
    };

    IcpResult {
        invariant_subsets,
        accepted_causal_predictors: accepted,
        nile_scores_by_variable: nile_scores,
    }
}

fn univariate_ols_fit(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    let mx = mean(x);
    let my = mean(y);
    let mut num = 0.0;
    let mut den = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        num += (xi - mx) * (yi - my);
        den += (xi - mx) * (xi - mx);
    }
    let beta = if den > 1e-12 { num / den } else { 0.0 };
    let alpha = my - beta * mx;
    let res: Vec<f64> = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| yi - (alpha + beta * xi))
        .collect();
    let sd = sample_variance(&res).sqrt();
    (beta, alpha, sd)
}

fn is_subset_invariant(environments: &[CausalEnvironment], subset: &[usize], tol: f64) -> bool {
    // Pool all environments to fit the reference structural linear model on `subset`
    let mut pooled_x = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            let sub_row: Vec<f64> = subset.iter().map(|&idx| row[idx]).collect();
            pooled_x.push(sub_row);
            pooled_y.push(yi);
        }
    }
    let coeffs = fit_ols_multi(&pooled_x, &pooled_y);

    // Check invariance across environments of: (1) environment-specific slopes, (2) residual mean, (3) residual SD
    let mut env_means = Vec::with_capacity(environments.len());
    let mut env_sds = Vec::with_capacity(environments.len());
    let mut max_coeff_dev = 0.0_f64;

    for env in environments {
        let env_x: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| subset.iter().map(|&idx| r[idx]).collect())
            .collect();
        let env_coeffs = fit_ols_multi(&env_x, &env.y);
        for (&c_pool, &c_env) in coeffs.iter().zip(env_coeffs.iter()) {
            max_coeff_dev = max_coeff_dev.max((c_pool - c_env).abs());
        }

        let residuals: Vec<f64> = env_x
            .iter()
            .zip(env.y.iter())
            .map(|(r, &yi)| {
                let pred: f64 = coeffs[0]
                    + r.iter()
                        .zip(coeffs[1..].iter())
                        .map(|(&xj, &bj)| xj * bj)
                        .sum::<f64>();
                yi - pred
            })
            .collect();
        env_means.push(mean(&residuals).abs());
        env_sds.push(sample_variance(&residuals).sqrt());
    }

    let max_mean = env_means.into_iter().fold(0.0_f64, f64::max);
    let min_sd = env_sds.iter().copied().fold(f64::INFINITY, f64::min);
    let max_sd = env_sds.into_iter().fold(0.0_f64, f64::max);

    max_coeff_dev <= tol && max_mean <= tol && (max_sd - min_sd) <= tol
}

fn fit_ols_multi(x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let n = x.len();
    let d = x[0].len() + 1;
    let mut xtx = vec![vec![0.0; d]; d];
    let mut xty = vec![0.0; d];
    for (row, &yi) in x.iter().zip(y.iter()) {
        for i in 0..d {
            let xi = if i == 0 { 1.0 } else { row[i - 1] };
            xty[i] += xi * yi;
            for j in 0..d {
                let xj = if j == 0 { 1.0 } else { row[j - 1] };
                xtx[i][j] += xi * xj;
            }
        }
    }
    for (i, row) in xtx.iter_mut().enumerate() {
        row[i] += 1e-8 * (n as f64);
    }
    // Gaussian elimination
    let mut aug: Vec<Vec<f64>> = xtx
        .into_iter()
        .enumerate()
        .map(|(i, mut r)| {
            r.push(xty[i]);
            r
        })
        .collect();
    for col in 0..d {
        let mut piv = col;
        for row in (col + 1)..d {
            if aug[row][col].abs() > aug[piv][col].abs() {
                piv = row;
            }
        }
        aug.swap(col, piv);
        let diag = aug[col][col];
        if diag.abs() < 1e-15 {
            continue;
        }
        #[allow(clippy::needless_range_loop)]
        for j in col..=d {
            aug[col][j] /= diag;
        }
        for row in 0..d {
            if row != col {
                let f = aug[row][col];
                #[allow(clippy::needless_range_loop)]
                for j in col..=d {
                    aug[row][j] -= f * aug[col][j];
                }
            }
        }
    }
    aug.into_iter().map(|r| r[d]).collect()
}

/// Second-order Taylor expansion of worst-case interventional loss with diagonal Hessian (§1.3.4):
/// $$L_{\mathrm{wc}}(\theta) \approx L_0 + \nabla_\theta L^\top \theta + \frac{1}{2}\sum_{i=1}^d H_{ii} \theta_i^2$$
pub fn second_order_worst_case_loss(
    base_loss_l0: f64,
    gradient: &[f64],
    hessian_diag: &[f64],
    theta: &[f64],
) -> f64 {
    assert_eq!(gradient.len(), theta.len());
    assert_eq!(hessian_diag.len(), theta.len());
    let lin: f64 = gradient
        .iter()
        .zip(theta.iter())
        .map(|(&g, &t)| g * t)
        .sum();
    let quad: f64 = hessian_diag
        .iter()
        .zip(theta.iter())
        .map(|(&h, &t)| 0.5 * h * t * t)
        .sum();
    base_loss_l0 + lin + quad
}

/// Second-order Taylor expansion of worst-case interventional loss with a full $d \times d$ Hessian matrix (§1.3.4):
/// $$L_{\mathrm{wc}}(\theta) \approx L_0 + \nabla_\theta L^\top \theta + \frac{1}{2}\theta^\top H_\theta \theta$$
pub fn second_order_worst_case_loss_full(
    base_loss_l0: f64,
    gradient: &[f64],
    hessian_matrix: &[Vec<f64>],
    theta: &[f64],
) -> f64 {
    let d = theta.len();
    assert_eq!(gradient.len(), d);
    assert_eq!(hessian_matrix.len(), d);
    assert!(hessian_matrix.iter().all(|row| row.len() == d));
    let lin: f64 = gradient
        .iter()
        .zip(theta.iter())
        .map(|(&g, &t)| g * t)
        .sum();
    let mut quad = 0.0;
    for i in 0..d {
        for j in 0..d {
            quad += 0.5 * theta[i] * hessian_matrix[i][j] * theta[j];
        }
    }
    base_loss_l0 + lin + quad
}

/// Critical Hidden Confounding Sensitivity Threshold $\Lambda^* \ge 1$ (§1.3.4):
/// Computes the minimum odds-ratio / confounding strength $\Lambda^*$ required to nullify
/// an observed causal effect `estimated_effect` relative to critical decision bound `null_threshold`.
pub fn hidden_confounding_threshold_lambda_star(
    estimated_effect: f64,
    standard_error: f64,
    null_threshold: f64,
) -> f64 {
    let conservative_lower = (estimated_effect.abs() - 1.96 * standard_error).max(0.0);
    if conservative_lower <= null_threshold.abs() {
        1.0
    } else {
        let ratio = conservative_lower / null_threshold.abs().max(1e-6);
        ratio + (ratio * (ratio - 1.0)).max(0.0).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn icp_isolates_true_structural_cause_and_rejects_spurious_child() {
        // Structural DAG: X0 -> Y -> X1
        // Across environments, the variance of X0 and the spurious mechanism Y -> X1 change,
        // while the causal mechanism Y = 1.75 * X0 + eps remains strictly invariant!
        let mut rng = Rng::new(2026);
        let mut envs = Vec::new();
        for (idx, (scale_x0, spurious_weight)) in [(1.0, 1.2), (2.2, -1.4), (0.7, 2.8)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(300);
            let mut y = Vec::with_capacity(300);
            for _ in 0..300 {
                let x0 = rng.normal(0.0, scale_x0);
                let yi = 1.75 * x0 + rng.normal(0.0, 0.12);
                let x1 = spurious_weight * yi + rng.normal(0.0, 0.35);
                x.push(vec![x0, x1]);
                y.push(yi);
            }
            envs.push(CausalEnvironment {
                name: format!("env_{idx}"),
                x,
                y,
            });
        }

        let res = run_icp(&envs, 0.12);
        assert_eq!(res.accepted_causal_predictors, vec![0]);
        assert!(res.nile_scores_by_variable[0] < res.nile_scores_by_variable[1] * 0.05);
    }
}
