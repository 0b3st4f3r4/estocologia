//! Lens 4 — Structure and Invariance: Pearl's Causal Ladder, Invariant Causal Prediction (ICP),
//! Environment-Projected OLS Invariance Penalty, and Hidden Confounding Threshold $\Lambda^*$ (§1.3.4).
//!
//! Implements:
//! - The three rungs of Judea Pearl's Causal Ladder (`1-See`, `2-Act`, `3-Imagine`);
//! - Invariant Causal Prediction (ICP) of Peters, Bühlmann & Meinshausen:
//!   $$Y \perp E \mid X_S \implies \hat{S}^* = \bigcap_{S : \text{invariant}} S$$
//! - Environment-projected linear OLS slope and residual-variance invariance penalty across environments $e \in \mathcal{E}$;
//! - Second-order expansion of worst-case interventional loss $L_{\mathrm{wc}}(\theta) \approx L_0 + \nabla_\theta L^\top \theta + \frac{1}{2}\theta^\top H_\theta \theta$
//!   and critical hidden confounding sensitivity threshold $\Lambda^*$.

use stochology_core::{mean, sample_std, sample_variance};

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

/// Output of Invariant Causal Prediction (ICP) and environment-projected OLS invariance penalty across environments $e \in \mathcal{E}$.
#[derive(Debug, Clone, PartialEq)]
pub struct IcpResult {
    /// Subsets $S \subseteq \{0, \dots, p-1\}$ whose slopes and residual distributions remain invariant across all environments.
    pub invariant_subsets: Vec<Vec<usize>>,
    /// Structural intersection accepted by ICP: $\hat{S}^* = \bigcap_{S : \text{invariant}} S$.
    pub accepted_causal_predictors: Vec<usize>,
    /// Environment-projected linear OLS slope and residual-SD invariance penalty per candidate variable.
    pub env_ols_invariance_penalties: Vec<f64>,
}

/// Executes Invariant Causal Prediction (ICP, Peters, Bühlmann & Meinshausen)
/// coupled with the environment-projected linear OLS invariance penalty across the provided environments.
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

    // Environment-projected linear OLS invariance penalty per variable j:
    // Fits the pooled linear OLS candidate on variable j, then evaluates the environment-projected
    // residual shift and slope/residual-SD variance across environments e in E.
    let mut env_penalties = vec![0.0; p];
    for (j, score_slot) in env_penalties.iter_mut().enumerate() {
        let mut raw_pooled_xj = Vec::new();
        let mut pooled_y = Vec::new();
        for env in environments {
            for (row, &yi) in env.x.iter().zip(env.y.iter()) {
                raw_pooled_xj.push(row[j]);
                pooled_y.push(yi);
            }
        }
        let sd_xj = sample_std(&raw_pooled_xj).max(1e-12);
        let pooled_xj: Vec<f64> = raw_pooled_xj.iter().map(|&x| x / sd_xj).collect();
        let (beta_pool, alpha_pool, _) = univariate_ols_fit(&pooled_xj, &pooled_y);
        let mut betas = Vec::with_capacity(environments.len());
        let mut res_sds = Vec::with_capacity(environments.len());
        let mut env_proj_penalty = 0.0;
        for env in environments {
            let col_j: Vec<f64> = env.x.iter().map(|row| row[j] / sd_xj).collect();
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

    for mask in 0..total_masks {
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
        env_ols_invariance_penalties: env_penalties,
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
    let mut pooled_x_raw = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            let sub_row: Vec<f64> = subset.iter().map(|&idx| row[idx]).collect();
            pooled_x_raw.push(sub_row);
            pooled_y.push(yi);
        }
    }

    let sigma_y = sample_std(&pooled_y).max(1e-12);
    let mut sigma_x = Vec::with_capacity(subset.len());
    for col in 0..subset.len() {
        let col_vals: Vec<f64> = pooled_x_raw.iter().map(|r| r[col]).collect();
        sigma_x.push(sample_std(&col_vals).max(1e-12));
    }

    let pooled_x: Vec<Vec<f64>> = pooled_x_raw
        .iter()
        .map(|r| {
            r.iter()
                .zip(sigma_x.iter())
                .map(|(&v, &sx)| v / sx)
                .collect()
        })
        .collect();
    let coeffs = fit_ols_multi(&pooled_x, &pooled_y);

    let pooled_residuals: Vec<f64> = pooled_x
        .iter()
        .zip(pooled_y.iter())
        .map(|(r, &yi)| {
            let pred: f64 = coeffs[0]
                + r.iter()
                    .zip(coeffs[1..].iter())
                    .map(|(&xj, &bj)| xj * bj)
                    .sum::<f64>();
            yi - pred
        })
        .collect();
    let sigma_res_pool = sample_std(&pooled_residuals).max(1e-12);

    // Check scale-invariant stability across environments of:
    // (1) environment-specific slopes normalized by sigma_Y / sigma_Xj,
    // (2) residual mean normalized by pooled residual SD,
    // (3) residual SD spread normalized by pooled residual SD.
    let mut env_means = Vec::with_capacity(environments.len());
    let mut env_sds = Vec::with_capacity(environments.len());
    let mut max_coeff_dev = 0.0_f64;

    for env in environments {
        let env_x: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| {
                subset
                    .iter()
                    .zip(sigma_x.iter())
                    .map(|(&idx, &sx)| r[idx] / sx)
                    .collect()
            })
            .collect();
        let env_coeffs = fit_ols_multi(&env_x, &env.y);
        for (&c_pool, &c_env) in coeffs[1..].iter().zip(env_coeffs[1..].iter()) {
            max_coeff_dev = max_coeff_dev.max((c_pool - c_env).abs() / sigma_y);
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
        env_means.push(0.5 * mean(&residuals).abs() / sigma_res_pool);
        env_sds.push(0.5 * sample_std(&residuals) / sigma_res_pool);
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
        let ratio = if null_threshold.abs() > 1e-4 {
            (conservative_lower / null_threshold.abs()).max(1.0)
        } else {
            let d_std = conservative_lower / (4.0 * standard_error).max(1e-6);
            (0.91 * d_std).exp().max(1.0)
        };
        ratio + (ratio * (ratio - 1.0)).max(0.0).sqrt()
    }
}

/// Formal Chow (1960) structural stability $F$-statistic across environments $e \in \mathcal{E}$ for predictor subset `subset`:
/// $$F_{\text{Chow}}(S) = \frac{(\mathrm{RSS}_{\text{pooled}} - \sum_{e=1}^{|\mathcal{E}|} \mathrm{RSS}_e) / \bigl((|\mathcal{E}| - 1) d\bigr)}{\bigl(\sum_{e=1}^{|\mathcal{E}|} \mathrm{RSS}_e\bigr) / (N - |\mathcal{E}| d)}$$
/// where $d = |S| + 1$. Near-$1.0$ (or small) values indicate structural invariance across environments; large values reject invariance.
pub fn chow_invariance_f_stat(environments: &[CausalEnvironment], subset: &[usize]) -> f64 {
    assert!(environments.len() >= 2);
    let mut pooled_x = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            pooled_x.push(subset.iter().map(|&idx| row[idx]).collect::<Vec<f64>>());
            pooled_y.push(yi);
        }
    }
    let coeffs_pool = fit_ols_multi(&pooled_x, &pooled_y);
    let rss_pool: f64 = pooled_x
        .iter()
        .zip(pooled_y.iter())
        .map(|(r, &y)| (y - eval_basis_model(&coeffs_pool, r)).powi(2))
        .sum();

    let mut rss_env_sum = 0.0;
    for env in environments {
        let env_x: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| subset.iter().map(|&idx| r[idx]).collect())
            .collect();
        let coeffs_e = fit_ols_multi(&env_x, &env.y);
        rss_env_sum += env_x
            .iter()
            .zip(env.y.iter())
            .map(|(r, &y)| (y - eval_basis_model(&coeffs_e, r)).powi(2))
            .sum::<f64>();
    }
    let k_env = environments.len() as f64;
    let d = (subset.len() + 1) as f64;
    let n_tot = pooled_y.len() as f64;
    let df1 = ((k_env - 1.0) * d).max(1.0);
    let df2 = (n_tot - k_env * d).max(1.0);
    (((rss_pool - rss_env_sum).max(0.0) / df1) / (rss_env_sum.max(1e-12) / df2)).max(0.0)
}

/// Cinelli & Hazlett (2020, *JRSS-B*) Omitted Variable Bias Robustness Value $RV_q \in [0, 1)$:
/// Computes the minimum partial $R^2$ that an unobserved confounder $U$ would need to share simultaneously
/// with both the treatment/cause $X$ and the outcome $Y$ to reduce the estimated effect by a fraction $q \in (0, 1]$:
/// $$RV_q = \frac{1}{2}\left(\sqrt{f_q^4 + 4 f_q^2} - f_q^2\right), \qquad f_q = q \cdot \frac{|t|}{\sqrt{\mathrm{df}}}$$
pub fn cinelli_hazlett_robustness_value(
    t_statistic: f64,
    degrees_of_freedom: usize,
    q_fraction: f64,
) -> f64 {
    assert!(degrees_of_freedom >= 1 && q_fraction > 0.0 && q_fraction <= 1.0);
    let f_q = q_fraction * t_statistic.abs() / (degrees_of_freedom as f64).sqrt();
    let f2 = f_q * f_q;
    0.5 * ((f2 * f2 + 4.0 * f2).sqrt() - f2)
}

/// Output of Nonlinear Invariant Causal Prediction and Nonlinear NILE/IRM over representation $\Phi(X)$.
#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearIcpNileResult {
    /// Subsets $S \subseteq \{0, \dots, p-1\}$ whose nonlinear conditional expectations $\mathbb{E}_e[Y \mid \Phi(X_S)]$ and residual distributions are invariant across $e \in \mathcal{E}$.
    pub invariant_subsets: Vec<Vec<usize>>,
    /// Structural intersection accepted over $\Phi$: $\hat{S}^* = \bigcap_{S : \text{invariant}} S$.
    pub accepted_causal_predictors: Vec<usize>,
    /// Linear baseline (`env_ols_invariance_penalties`) preserved for direct comparison.
    pub linear_ols_baseline_penalties: Vec<f64>,
    /// True nonlinear NILE + IRM-v1 risk-gradient penalty over representation $\Phi(X_j)$ per candidate variable $j$.
    pub nonlinear_nile_irm_penalties: Vec<f64>,
    /// Maximum environment deviation of the accepted subset $\hat{S}^*$ on representation $\Phi$.
    pub max_accepted_env_deviation: f64,
}

pub(crate) fn expand_nonlinear_basis_1d(z: f64) -> [f64; 3] {
    let zc = z.clamp(-6.0, 6.0);
    [
        zc,
        (zc * zc - 1.0) * std::f64::consts::FRAC_1_SQRT_2,
        (zc * zc * zc - 3.0 * zc) / 6.0_f64.sqrt(),
    ]
}

fn expand_nonlinear_subset_row(row: &[f64], subset: &[usize], sigma_x: &[f64]) -> Vec<f64> {
    let mut phi = Vec::with_capacity(subset.len() * 3);
    for (&idx, &sx) in subset.iter().zip(sigma_x.iter()) {
        let z = row[idx] / sx;
        phi.extend_from_slice(&expand_nonlinear_basis_1d(z));
    }
    phi
}

fn eval_basis_model(coeffs: &[f64], phi_row: &[f64]) -> f64 {
    coeffs[0]
        + phi_row
            .iter()
            .zip(coeffs[1..].iter())
            .map(|(&p, &c)| p * c)
            .sum::<f64>()
}

/// Executes Nonlinear Invariant Causal Prediction and computes the Nonlinear NILE + IRM-v1
/// risk-gradient penalty over the 3-atom orthonormal Hermite basis
/// $\Phi(z) = \bigl[z,\ \frac{z^2-1}{\sqrt{2}},\ \frac{z^3-3z}{\sqrt{6}}\bigr]$
/// alongside the linear OLS baseline.
pub fn run_nonlinear_icp_nile(
    environments: &[CausalEnvironment],
    invariance_tolerance: f64,
) -> NonlinearIcpNileResult {
    let linear_base = run_icp(environments, invariance_tolerance);
    let p = environments[0].x[0].len();

    let mut nonlinear_penalties = vec![0.0; p];
    for (j, slot) in nonlinear_penalties.iter_mut().enumerate() {
        let mut raw_pooled_xj = Vec::new();
        let mut pooled_y = Vec::new();
        for env in environments {
            for (row, &yi) in env.x.iter().zip(env.y.iter()) {
                raw_pooled_xj.push(row[j]);
                pooled_y.push(yi);
            }
        }
        let mean_xj = mean(&raw_pooled_xj);
        let sd_xj = sample_std(&raw_pooled_xj).max(1e-12);
        let mean_y = mean(&pooled_y);
        let sigma_y = sample_std(&pooled_y).max(1e-12);
        let pooled_phi: Vec<Vec<f64>> = raw_pooled_xj
            .iter()
            .map(|&x| expand_nonlinear_basis_1d((x - mean_xj) / sd_xj).to_vec())
            .collect();
        let pooled_y_norm: Vec<f64> = pooled_y.iter().map(|&y| (y - mean_y) / sigma_y).collect();
        let theta_pool = fit_ols_multi(&pooled_phi, &pooled_y_norm);

        let mut nile_func_shift = 0.0;
        let mut irm_grad_penalty = 0.0;
        let mut env_pooled_risks = Vec::with_capacity(environments.len());

        for env in environments {
            let env_phi: Vec<Vec<f64>> = env
                .x
                .iter()
                .map(|r| expand_nonlinear_basis_1d((r[j] - mean_xj) / sd_xj).to_vec())
                .collect();
            let env_y_norm: Vec<f64> = env.y.iter().map(|&y| (y - mean_y) / sigma_y).collect();
            let theta_env = fit_ols_multi(&env_phi, &env_y_norm);

            let n_e = env_phi.len() as f64;
            let mut func_diff_sq = 0.0;
            let mut irm_grad_w1 = 0.0;
            let mut risk_pool_e = 0.0;

            for (phi_i, &yi) in env_phi.iter().zip(env_y_norm.iter()) {
                let f_pool = eval_basis_model(&theta_pool, phi_i);
                let f_env = eval_basis_model(&theta_env, phi_i);
                let diff = f_env - f_pool;
                func_diff_sq += diff * diff;
                let err_pool = yi - f_pool;
                irm_grad_w1 += -2.0 * err_pool * f_pool;
                risk_pool_e += err_pool * err_pool;
            }
            nile_func_shift += func_diff_sq / n_e;
            let g1 = irm_grad_w1 / n_e;
            irm_grad_penalty += g1 * g1;
            env_pooled_risks.push(risk_pool_e / n_e);
        }

        let k_env = environments.len() as f64;
        *slot = (nile_func_shift / k_env)
            + (irm_grad_penalty / k_env)
            + sample_variance(&env_pooled_risks);
    }

    let mut invariant_subsets = Vec::new();
    let total_masks = 1usize << p;
    for mask in 0..total_masks {
        let subset: Vec<usize> = (0..p).filter(|&bit| (mask & (1 << bit)) != 0).collect();
        let (inv, _) = is_nonlinear_subset_invariant(environments, &subset, invariance_tolerance);
        if inv {
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

    let max_accepted_env_deviation = if accepted.is_empty() {
        1.0
    } else {
        let (_, dev) = is_nonlinear_subset_invariant(environments, &accepted, invariance_tolerance);
        dev
    };

    NonlinearIcpNileResult {
        invariant_subsets,
        accepted_causal_predictors: accepted,
        linear_ols_baseline_penalties: linear_base.env_ols_invariance_penalties,
        nonlinear_nile_irm_penalties: nonlinear_penalties,
        max_accepted_env_deviation,
    }
}

fn is_nonlinear_subset_invariant(
    environments: &[CausalEnvironment],
    subset: &[usize],
    tol: f64,
) -> (bool, f64) {
    let mut pooled_x_raw = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            pooled_x_raw.push(row.clone());
            pooled_y.push(yi);
        }
    }

    let sigma_y = sample_std(&pooled_y).max(1e-12);
    let mut sigma_x = Vec::with_capacity(subset.len());
    for &idx in subset {
        let col_vals: Vec<f64> = pooled_x_raw.iter().map(|r| r[idx]).collect();
        sigma_x.push(sample_std(&col_vals).max(1e-12));
    }

    let pooled_phi: Vec<Vec<f64>> = pooled_x_raw
        .iter()
        .map(|r| expand_nonlinear_subset_row(r, subset, &sigma_x))
        .collect();
    let coeffs = fit_ols_multi(&pooled_phi, &pooled_y);

    let pooled_residuals: Vec<f64> = pooled_phi
        .iter()
        .zip(pooled_y.iter())
        .map(|(phi_r, &yi)| yi - eval_basis_model(&coeffs, phi_r))
        .collect();
    let sigma_res_pool = sample_std(&pooled_residuals).max(1e-12);

    let mut env_means = Vec::with_capacity(environments.len());
    let mut env_sds = Vec::with_capacity(environments.len());
    let mut max_func_dev = 0.0_f64;

    for env in environments {
        let env_phi: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| expand_nonlinear_subset_row(r, subset, &sigma_x))
            .collect();
        let env_coeffs = fit_ols_multi(&env_phi, &env.y);

        let f_rms = (env_phi
            .iter()
            .map(|phi_r| {
                let d = eval_basis_model(&coeffs, phi_r) - eval_basis_model(&env_coeffs, phi_r);
                d * d
            })
            .sum::<f64>()
            / (env_phi.len() as f64))
            .sqrt()
            / sigma_y;
        max_func_dev = max_func_dev.max(f_rms);

        let residuals: Vec<f64> = env_phi
            .iter()
            .zip(env.y.iter())
            .map(|(phi_r, &yi)| yi - eval_basis_model(&coeffs, phi_r))
            .collect();
        env_means.push(0.5 * mean(&residuals).abs() / sigma_res_pool);
        env_sds.push(0.5 * sample_std(&residuals) / sigma_res_pool);
    }

    let max_mean = env_means.into_iter().fold(0.0_f64, f64::max);
    let min_sd = env_sds.iter().copied().fold(f64::INFINITY, f64::min);
    let max_sd = env_sds.into_iter().fold(0.0_f64, f64::max);
    let sd_spread = max_sd - min_sd;
    let worst_dev = max_func_dev.max(max_mean).max(sd_spread);

    (
        max_func_dev <= tol && max_mean <= tol && sd_spread <= tol,
        worst_dev,
    )
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
        assert!(res.env_ols_invariance_penalties[0] < res.env_ols_invariance_penalties[1] * 0.05);

        let f_true = chow_invariance_f_stat(&envs, &[0]);
        let f_spurious = chow_invariance_f_stat(&envs, &[1]);
        assert!(f_true < 3.0 && f_spurious > 50.0);
        let rv = cinelli_hazlett_robustness_value(12.0, 890, 1.0);
        assert!(rv > 0.25 && rv < 1.0);
    }

    #[test]
    fn icp_is_scale_invariant_and_returns_empty_set_under_null_environment_shift() {
        let mut rng = Rng::new(2026);
        let mut envs = Vec::new();
        let mut envs_scaled = Vec::new();
        let mut envs_y_scaled = Vec::new();
        for (idx, (scale_x0, spurious_weight)) in [(1.0, 1.2), (2.2, -1.4), (0.7, 2.8)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(300);
            let mut x_scaled = Vec::with_capacity(300);
            let mut y = Vec::with_capacity(300);
            let mut y_scaled = Vec::with_capacity(300);
            for _ in 0..300 {
                let x0 = rng.normal(0.0, scale_x0);
                let yi = 1.75 * x0 + rng.normal(0.0, 0.12);
                let x1 = spurious_weight * yi + rng.normal(0.0, 0.35);
                x.push(vec![x0, x1]);
                x_scaled.push(vec![x0 * 100.0, x1]);
                y.push(yi);
                y_scaled.push(yi * 1000.0);
            }
            envs.push(CausalEnvironment {
                name: format!("env_{idx}"),
                x: x.clone(),
                y: y.clone(),
            });
            envs_scaled.push(CausalEnvironment {
                name: format!("env_scaled_{idx}"),
                x: x_scaled,
                y,
            });
            envs_y_scaled.push(CausalEnvironment {
                name: format!("env_y_scaled_{idx}"),
                x,
                y: y_scaled,
            });
        }

        let res_orig = run_icp(&envs, 0.12);
        let res_scaled = run_icp(&envs_scaled, 0.12);
        let res_y_scaled = run_icp(&envs_y_scaled, 0.12);
        assert_eq!(res_orig.accepted_causal_predictors, vec![0]);
        assert_eq!(
            res_orig.accepted_causal_predictors,
            res_scaled.accepted_causal_predictors
        );
        assert_eq!(
            res_orig.accepted_causal_predictors,
            res_y_scaled.accepted_causal_predictors
        );
        assert_eq!(res_orig.invariant_subsets, res_scaled.invariant_subsets);
        assert_eq!(res_orig.invariant_subsets, res_y_scaled.invariant_subsets);
        assert!(
            (res_orig.env_ols_invariance_penalties[0] - res_scaled.env_ols_invariance_penalties[0])
                .abs()
                < 1e-9
        );

        // Null environment shift: Y ~ N(0, 1) independent of environment and predictors
        let mut null_envs = Vec::new();
        for idx in 0..3 {
            let mut x = Vec::with_capacity(400);
            let mut y = Vec::with_capacity(400);
            for _ in 0..400 {
                x.push(vec![
                    rng.normal(0.0, 1.0 + idx as f64),
                    rng.normal(0.0, 1.0),
                ]);
                y.push(rng.normal(0.0, 1.0));
            }
            null_envs.push(CausalEnvironment {
                name: format!("null_{idx}"),
                x,
                y,
            });
        }
        let res_null = run_icp(&null_envs, 0.12);
        assert!(
            res_null.invariant_subsets.contains(&Vec::new()),
            "intercept-only empty set must be invariant when Y does not shift"
        );
        assert!(
            res_null.accepted_causal_predictors.is_empty(),
            "intersection must be empty when empty set is invariant"
        );
    }

    #[test]
    fn lambda_star_remains_finite_and_sensible_at_zero_null_threshold() {
        let lam_zero = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.0);
        assert!(
            lam_zero.is_finite() && lam_zero > 2.0 && lam_zero < 1_000.0,
            "expected finite sensible E-value at null_threshold=0.0, got {lam_zero}"
        );
        let lam_nonzero = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.60);
        assert!(lam_nonzero > 4.0 && lam_nonzero < 6.0);
    }

    #[test]
    fn nonlinear_nile_irm_isolates_nonlinear_structural_cause_over_phi() {
        // Strongly nonlinear structural causal model:
        // X0 ~ N(0, scale_e), Y = 1.10 * X0 + 0.75 * (X0^2 - 1) - 0.15 * X0^3 + eps_Y, X1 = sp_e * Y + eps_1
        // Notice E[X0 * (X0^2 - 1)] = 0 so a purely linear model suffers huge residual variance on the quadratic curvature!
        let mut rng = Rng::new(2026);
        let mut envs = Vec::new();
        for (idx, (scale_x0, spurious_w)) in [(0.8, 1.3), (1.5, -1.5), (1.1, 2.4)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(350);
            let mut y = Vec::with_capacity(350);
            for _ in 0..350 {
                let x0 = rng.normal(0.0, scale_x0);
                let yi =
                    1.10 * x0 + 0.75 * (x0 * x0 - 1.0) - 0.15 * x0.powi(3) + rng.normal(0.0, 0.10);
                let x1 = spurious_w * yi + rng.normal(0.0, 0.30);
                x.push(vec![x0, x1]);
                y.push(yi);
            }
            envs.push(CausalEnvironment {
                name: format!("nonlin_env_{idx}"),
                x,
                y,
            });
        }

        let nl = run_nonlinear_icp_nile(&envs, 0.15);
        assert_eq!(nl.accepted_causal_predictors, vec![0]);
        assert!(
            nl.nonlinear_nile_irm_penalties[0] < nl.nonlinear_nile_irm_penalties[1] * 0.05,
            "nonlinear NILE/IRM penalty on true cause X0 ({:.5}) must be < 5% of spurious child X1 ({:.5})",
            nl.nonlinear_nile_irm_penalties[0],
            nl.nonlinear_nile_irm_penalties[1]
        );
        // Nonlinear NILE penalty on X0 is also strictly smaller than the misspecified linear OLS penalty on X0
        assert!(nl.nonlinear_nile_irm_penalties[0] < nl.linear_ols_baseline_penalties[0]);
    }
}
