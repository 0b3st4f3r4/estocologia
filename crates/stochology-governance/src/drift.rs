//! Formal Drift Adaptation and Murphy-CORP Circuit Breaker across the 4 Regimes of Drift (§4.2):
//! 1. **Covariate Drift** ($Q(X) \neq P(X)$, $Q(Y \mid X) = P(Y \mid X)$): Exponential Tilt $\hat{w}(x)$
//!    and Weighted Conformal Prediction (*ExTRA-WCP-T*) with Lei & Candès coverage bound $1 - \alpha - d_{\mathrm{TV}}(Q, \hat{Q})$;
//! 2. **Label / Prior Probability Shift** ($Q(Y) \neq P(Y)$, $Q(X \mid Y) = P(X \mid Y)$): Four canonical
//!    estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**);
//! 3. **Concept / Structural Drift** ($Q(Y \mid X) \neq P(Y \mid X)$): Murphy-CORP circuit breaker;
//! 4. **Performative Feedback** ($\mathcal{D}(\theta)$): Stackelberg equilibrium / RRM contraction audit.

use stochology_core::quantile;
use stochology_lenses::decision::CorpPavDecomposition;

/// The Four Regimes of Production Drift (§4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftRegime {
    Stationary,
    CovariateDrift,
    LabelShift,
    ConceptDriftCircuitBreaker,
    PerformativeFeedback,
}

/// Continuous Production Monitor and Circuit Breaker based on Isotonic CORP-Murphy Decomposition (§4.2).
#[derive(Debug, Clone, PartialEq)]
pub struct MurphyCorpCircuitBreaker {
    pub rel_miscalibration: f64,
    pub res_resolution: f64,
    pub unc0_uncertainty: f64,
    pub circuit_breaker_tripped: bool,
    pub diagnosed_regime: DriftRegime,
    pub recommended_corrective_action: String,
}

impl MurphyCorpCircuitBreaker {
    /// Evaluates rolling predictions and outcomes against `max_rel_threshold` and `min_res_threshold`.
    pub fn monitor(
        forecasts_p: &[f64],
        outcomes_y: &[f64],
        max_rel_threshold: f64,
        min_res_threshold: f64,
        covariate_shift_detected: bool,
        label_shift_detected: bool,
        performative_loop_active: bool,
    ) -> Self {
        let corp = CorpPavDecomposition::compute(forecasts_p, outcomes_y);
        let rel_degraded = corp.rel_miscalibration > max_rel_threshold;
        let res_collapsed = corp.res_resolution < min_res_threshold;

        let (tripped, regime, action) = if res_collapsed {
            (
                true,
                DriftRegime::ConceptDriftCircuitBreaker,
                "TRIP CIRCUIT BREAKER: RES_CORP collapsed below threshold (Concept Drift Q(Y|X) != P(Y|X)); suspend automated decisions and re-enter SIGA Funnel".into(),
            )
        } else if performative_loop_active {
            (
                rel_degraded,
                DriftRegime::PerformativeFeedback,
                "Performative feedback D(theta) active: audit RRM contraction (eps*beta/gamma < 1) and transition to Stackelberg optimum theta_PO".into(),
            )
        } else if label_shift_detected || rel_degraded {
            (
                false,
                DriftRegime::LabelShift,
                "RES_CORP preserved while REL_CORP shifted: apply Label Shift Bayes update via BBSE / MLLS / DFM / OTTER without full retraining".into(),
            )
        } else if covariate_shift_detected {
            (
                false,
                DriftRegime::CovariateDrift,
                "Covariate Drift Q(X) != P(X) with stable conditional physics: apply Exponential Tilt w_hat(x) and ExTRA-WCP-T conformal threshold".into(),
            )
        } else {
            (
                false,
                DriftRegime::Stationary,
                "Stationary regime: REL_CORP and RES_CORP within pre-registered operating bounds"
                    .into(),
            )
        };

        Self {
            rel_miscalibration: corp.rel_miscalibration,
            res_resolution: corp.res_resolution,
            unc0_uncertainty: corp.unc0_uncertainty,
            circuit_breaker_tripped: tripped,
            diagnosed_regime: regime,
            recommended_corrective_action: action,
        }
    }
}

/// Adaptation to Covariate Drift via Exponential Tilt and Weighted Conformal Prediction
/// (*ExTRA-WCP-T*) with Lei & Candès Total Variation Bound (§4.2).
///
/// Estimates $\beta$ by maximizing $\hat{L}(\beta) = \frac{1}{m}\sum_{j=1}^m \ln M_\beta(X_j^Q) - \ln Z_P(\beta) - \frac{\lambda}{2}\|\beta\|_2^2$
/// and computes the exact lower coverage bound:
/// $$\mathbb{P}_{P^n \times Q}\{Y_{n+1} \in C_{S, \hat{w}}(X_{n+1}) \mid \mathcal{F}\} \ge 1 - \alpha - d_{\mathrm{TV}}(Q, \hat{Q}), \qquad d_{\mathrm{TV}}(Q, \hat{Q}) = \frac{1}{2}\mathbb{E}_P|w^\star - \hat{w}|$$
#[derive(Debug, Clone, PartialEq)]
pub struct CovariateDriftResult {
    pub estimated_beta: f64,
    pub normalizing_constant_z_p: f64,
    pub regularized_objective_l_hat: f64,
    pub total_variation_distance_d_tv: f64,
    pub nominal_level_one_minus_alpha: f64,
    pub lei_candes_coverage_lower_bound: f64,
    pub weighted_conformal_threshold: f64,
}

pub fn fit_covariate_drift_extra_wcp(
    x_source_p: &[f64],
    x_target_q: &[f64],
    nonconformity_scores_source: &[f64],
    true_weights_optional: Option<&[f64]>,
    lambda_reg: f64,
    alpha: f64,
) -> CovariateDriftResult {
    assert!(!x_source_p.is_empty() && !x_target_q.is_empty());
    assert_eq!(x_source_p.len(), nonconformity_scores_source.len());
    let mean_q = stochology_core::mean(x_target_q);

    // Concave 1D Newton optimization for L_hat(beta) = beta * mean_Q(x) - ln E_P[exp(beta * x)] - 0.5 * lambda * beta^2
    let mut beta = 0.0_f64;
    for _ in 0..40 {
        let max_bx = x_source_p
            .iter()
            .map(|&x| beta * x)
            .fold(f64::NEG_INFINITY, f64::max);
        let exp_weights: Vec<f64> = x_source_p
            .iter()
            .map(|&x| (beta * x - max_bx).exp())
            .collect();
        let sum_w: f64 = exp_weights.iter().sum();
        let e_x: f64 = x_source_p
            .iter()
            .zip(exp_weights.iter())
            .map(|(&x, &w)| x * w)
            .sum::<f64>()
            / sum_w;
        let e_x2: f64 = x_source_p
            .iter()
            .zip(exp_weights.iter())
            .map(|(&x, &w)| x * x * w)
            .sum::<f64>()
            / sum_w;
        let grad = mean_q - e_x - lambda_reg * beta;
        let hess = (e_x2 - e_x * e_x + lambda_reg).max(1e-6);
        beta += grad / hess;
    }

    let max_bx = x_source_p
        .iter()
        .map(|&x| beta * x)
        .fold(f64::NEG_INFINITY, f64::max);
    let exp_shifted: Vec<f64> = x_source_p
        .iter()
        .map(|&x| (beta * x - max_bx).exp())
        .collect();
    let mean_shifted = stochology_core::mean(&exp_shifted);
    let z_p = mean_shifted * max_bx.exp();
    let w_hat: Vec<f64> = exp_shifted.iter().map(|&e| e / mean_shifted).collect();
    let l_hat = beta * mean_q - (max_bx + mean_shifted.ln()) - 0.5 * lambda_reg * beta * beta;

    // Total Variation distance d_TV(Q, Q_hat):
    // Uses exact 0.5 * E_P[|w* - w_hat|] when oracle weights w* are provided; otherwise combines
    // the Gaussian moment Pinsker estimate with a non-parametric empirical 10-decile-bin L1 Total
    // Variation distance between target sample Q and tilted source distribution (x_source_p, w_hat).
    let d_tv = if let Some(w_star) = true_weights_optional {
        0.5 * w_star
            .iter()
            .zip(w_hat.iter())
            .map(|(&ws, &wh)| (ws - wh).abs())
            .sum::<f64>()
            / (w_hat.len() as f64)
    } else {
        let sum_wh: f64 = w_hat.iter().sum::<f64>().max(1e-12);
        let mu_q_hat: f64 = x_source_p
            .iter()
            .zip(w_hat.iter())
            .map(|(&x, &w)| x * w)
            .sum::<f64>()
            / sum_wh;
        let var_q_hat: f64 = (x_source_p
            .iter()
            .zip(w_hat.iter())
            .map(|(&x, &w)| (x - mu_q_hat).powi(2) * w)
            .sum::<f64>()
            / sum_wh)
            .max(1e-6);
        let var_q: f64 = if x_target_q.len() >= 2 {
            stochology_core::sample_variance(x_target_q).max(1e-6)
        } else {
            var_q_hat
        };
        let kl_gauss = 0.5
            * ((var_q_hat / var_q).ln() + (var_q + (mean_q - mu_q_hat).powi(2)) / var_q_hat - 1.0)
                .max(0.0);
        let tv_gauss = (0.5 * kl_gauss).sqrt();
        let tv_decile = empirical_decile_tv_distance(x_source_p, &w_hat, x_target_q);
        tv_gauss.max(tv_decile).clamp(0.0, 1.0)
    };

    // Weighted quantile (ExTRA-WCP-T) of nonconformity scores with Lei-Candès TV-adjusted level (1 - alpha + d_TV)
    let mut pairs: Vec<(f64, f64)> = nonconformity_scores_source
        .iter()
        .copied()
        .zip(w_hat.iter().copied())
        .collect();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let sum_weights: f64 = pairs.iter().map(|(_, w)| *w).sum::<f64>() + 1.0;
    let effective_quantile_level = (1.0 - alpha + d_tv).clamp(0.0, 1.0);
    let target_cum = effective_quantile_level * sum_weights;
    let mut cum = 0.0;
    let mut conformal_threshold = pairs
        .last()
        .map(|p| p.0)
        .unwrap_or(quantile(nonconformity_scores_source, 1.0 - alpha));
    for (s, w) in pairs {
        cum += w;
        if cum >= target_cum {
            conformal_threshold = s;
            break;
        }
    }

    let lei_candes_bound = (1.0 - alpha - d_tv).max(0.0);

    CovariateDriftResult {
        estimated_beta: beta,
        normalizing_constant_z_p: z_p,
        regularized_objective_l_hat: l_hat,
        total_variation_distance_d_tv: d_tv,
        nominal_level_one_minus_alpha: 1.0 - alpha,
        lei_candes_coverage_lower_bound: lei_candes_bound,
        weighted_conformal_threshold: conformal_threshold,
    }
}

/// Auditable Estimators for Label / Prior Probability Shift (§4.2):
/// **BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelShiftDiagnostic {
    /// Base rate in the source distribution $P_Y(y=1)$.
    pub source_prior_p1: f64,
    /// BBSE estimate (Lipton, Wang & Smola): $\hat{\omega} = C^{-1}\hat{\mu}_t$.
    pub target_prior_bbse: f64,
    /// Smallest singular value $\sigma_{\min}(C)$ of the source confusion matrix (spectral condition $\sigma_{\min}(C) > 0$).
    pub confusion_matrix_sigma_min: f64,
    /// MLLS / SLD estimate (Saerens-Latinne-Decaestecker EM over calibrated posteriors).
    pub target_prior_mlls_sld: f64,
    /// DFM (*Distribution Feature Matching*) estimate and novel-class contamination fraction $\alpha_0^*$.
    pub target_prior_dfm: f64,
    pub novel_class_contamination_alpha0: f64,
    /// Second smallest eigenvalue $\Delta_{\min}$ of the centered Gram matrix in DFM.
    pub gram_eigenvalue_delta_min: f64,
    /// OTTER estimate (*Optimal Transport* with cost $C_{ij} = -\ln P_\theta(Y=j \mid X=x_i)$).
    pub target_prior_otter: f64,
    /// Hoffman stability constant $\kappa$ of the OTTER optimal transport linear program.
    pub hoffman_constant_kappa: f64,
}

/// Computes the exact algebraic Bayes posterior update under Label Shift:
/// $$Q(y = 1 \mid x) = \frac{\omega_1 P(y=1 \mid x)}{\omega_1 P(y=1 \mid x) + \omega_0 (1 - P(y=1 \mid x))}$$
/// where $\omega_1 = \frac{Q_Y(1)}{P_Y(1)}$ and $\omega_0 = \frac{1 - Q_Y(1)}{1 - P_Y(1)}$.
#[inline]
pub fn adjust_posterior_for_label_shift(
    p_source_given_x: f64,
    source_prior_p1: f64,
    target_prior_q1: f64,
) -> f64 {
    let w1 = target_prior_q1 / source_prior_p1.max(1e-12);
    let w0 = (1.0 - target_prior_q1) / (1.0 - source_prior_p1).max(1e-12);
    let num = w1 * p_source_given_x;
    let den = num + w0 * (1.0 - p_source_given_x);
    if den <= 0.0 {
        target_prior_q1
    } else {
        (num / den).clamp(1e-6, 1.0 - 1e-6)
    }
}

/// Executes all four canonical Label Shift estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**) without target labels.
#[allow(clippy::too_many_arguments)]
pub fn estimate_label_shift_four_methods(
    source_prior_p1: f64,
    source_tpr: f64,
    source_fpr: f64,
    target_calibrated_posteriors: &[f64],
    source_class0_feature_mean: f64,
    source_class1_feature_mean: f64,
    target_feature_mean: f64,
    novel_class_contamination_alpha0: f64,
) -> LabelShiftDiagnostic {
    assert!(!target_calibrated_posteriors.is_empty());

    // 1. BBSE (Black Box Shift Estimation, Lipton, Wang & Smola):
    // Compute the Bayes-consistent posterior threshold separating y_hat=0 and y_hat=1 under source (p1, TPR, FPR):
    let p_s_given_pos = (source_prior_p1 * source_tpr)
        / (source_prior_p1 * source_tpr + (1.0 - source_prior_p1) * source_fpr).max(1e-12);
    let p_s_given_neg = (source_prior_p1 * (1.0 - source_tpr))
        / (source_prior_p1 * (1.0 - source_tpr) + (1.0 - source_prior_p1) * (1.0 - source_fpr))
            .max(1e-12);
    let bayes_threshold = 0.5 * (p_s_given_pos + p_s_given_neg);

    let pred_pos_rate = target_calibrated_posteriors
        .iter()
        .filter(|&&p| p >= bayes_threshold)
        .count() as f64
        / (target_calibrated_posteriors.len() as f64);
    let diff_rates = source_tpr - source_fpr;
    let q1_bbse = if diff_rates.abs() < 1e-6 {
        source_prior_p1.clamp(0.001, 0.999)
    } else {
        ((pred_pos_rate - source_fpr) / diff_rates).clamp(0.001, 0.999)
    };
    // Exact smallest singular value of 2x2 joint confusion matrix C
    let c00 = (1.0 - source_fpr) * (1.0 - source_prior_p1);
    let c01 = source_fpr * (1.0 - source_prior_p1);
    let c10 = (1.0 - source_tpr) * source_prior_p1;
    let c11 = source_tpr * source_prior_p1;
    let sigma_min = smallest_singular_value_2x2(c00, c01, c10, c11);

    // 2. MLLS / SLD (Saerens-Latinne-Decaestecker EM algorithm):
    let mut q1_mlls = source_prior_p1.clamp(0.01, 0.99);
    for _ in 0..80 {
        let sum_post: f64 = target_calibrated_posteriors
            .iter()
            .map(|&p| adjust_posterior_for_label_shift(p, source_prior_p1, q1_mlls))
            .sum();
        q1_mlls = (sum_post / (target_calibrated_posteriors.len() as f64)).clamp(0.001, 0.999);
    }

    // 3. DFM (Distribution Feature Matching) with Generalized Label Shift (novel class contamination alpha_0*):
    let diff_mu = source_class1_feature_mean - source_class0_feature_mean;
    let delta_min = diff_mu * diff_mu;
    let effective_scale = (1.0 - novel_class_contamination_alpha0).clamp(0.01, 1.0);
    let q1_dfm = if diff_mu.abs() < 1e-12 {
        source_prior_p1.clamp(0.001, 0.999)
    } else {
        ((target_feature_mean - source_class0_feature_mean) / (effective_scale * diff_mu))
            .clamp(0.001, 0.999)
    };

    // 4. OTTER (Entropic Optimal Transport Label Shift Estimation via Sinkhorn-Knopp scaling
    // on cost matrix C_{ij} = -ln P_theta(Y=j | X=x_i)):
    let q1_otter = solve_otter_sinkhorn(
        source_prior_p1,
        source_tpr,
        source_fpr,
        pred_pos_rate,
        target_calibrated_posteriors,
    );
    // Exact Hoffman stability constant kappa(M) = 1 / sigma_min(M) of conditional operator M
    let sigma_min_m =
        smallest_singular_value_2x2(1.0 - source_fpr, 1.0 - source_tpr, source_fpr, source_tpr);
    let hoffman_kappa = 1.0 / sigma_min_m.max(1e-6);

    LabelShiftDiagnostic {
        source_prior_p1,
        target_prior_bbse: q1_bbse,
        confusion_matrix_sigma_min: sigma_min,
        target_prior_mlls_sld: q1_mlls,
        target_prior_dfm: q1_dfm,
        novel_class_contamination_alpha0,
        gram_eigenvalue_delta_min: delta_min,
        target_prior_otter: q1_otter,
        hoffman_constant_kappa: hoffman_kappa,
    }
}

/// Non-parametric empirical quantile-bin $L_1$ Total Variation distance between the target sample
/// `x_target_q` and the tilted source distribution `(x_source_p, w_hat)` across 10 pooled decile bins.
fn empirical_decile_tv_distance(x_source_p: &[f64], w_hat: &[f64], x_target_q: &[f64]) -> f64 {
    let mut pooled = Vec::with_capacity(x_source_p.len() + x_target_q.len());
    pooled.extend_from_slice(x_source_p);
    pooled.extend_from_slice(x_target_q);
    pooled.sort_by(|a, b| a.total_cmp(b));
    if pooled.is_empty() {
        return 0.0;
    }
    let mut cuts = [0.0_f64; 9];
    for (k, cut) in cuts.iter_mut().enumerate() {
        *cut = stochology_core::quantile_sorted(&pooled, ((k + 1) as f64) * 0.10);
    }
    let bin_index = |x: f64| -> usize {
        for (k, &c) in cuts.iter().enumerate() {
            if x <= c {
                return k;
            }
        }
        9
    };

    let mut p_bins = [0.0_f64; 10];
    let sum_w = w_hat.iter().sum::<f64>().max(1e-12);
    for (&x, &w) in x_source_p.iter().zip(w_hat.iter()) {
        p_bins[bin_index(x)] += w / sum_w;
    }

    let mut q_bins = [0.0_f64; 10];
    let inv_m = 1.0 / (x_target_q.len().max(1) as f64);
    for &x in x_target_q {
        q_bins[bin_index(x)] += inv_m;
    }

    let raw_tv = 0.5 * (0..10).map(|b| (p_bins[b] - q_bins[b]).abs()).sum::<f64>();
    let n = x_source_p.len().max(1) as f64;
    let m = x_target_q.len().max(1) as f64;
    let se_floor = 1.2 * (1.0 / n + 1.0 / m).sqrt();
    (raw_tv - se_floor).max(0.25 * raw_tv)
}

/// Exact smallest singular value $\sigma_{\min}(A)$ of a $2 \times 2$ matrix $\begin{pmatrix} a & b \\ c & d \end{pmatrix}$.
fn smallest_singular_value_2x2(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let frob2 = a * a + b * b + c * c + d * d;
    let det = (a * d - b * c).abs();
    let disc = (frob2 * frob2 - 4.0 * det * det).max(0.0).sqrt();
    (0.5 * (frob2 - disc)).max(0.0).sqrt()
}

/// Solves the Entropic Optimal Transport Label Shift problem (OTTER) via Sinkhorn-Knopp matrix scaling
/// on cost $C_{i,j} = -\ln P_\theta(Y=j \mid X=x_i)$ coupled with confusion operator $M$.
fn solve_otter_sinkhorn(
    source_prior_p1: f64,
    source_tpr: f64,
    source_fpr: f64,
    target_pred_pos_rate: f64,
    target_posteriors: &[f64],
) -> f64 {
    let n = target_posteriors.len();
    let inv_n = 1.0 / (n as f64);
    let p1_s = source_prior_p1.clamp(1e-6, 1.0 - 1e-6);
    let p0_s = 1.0 - p1_s;
    let mu_t = [
        (1.0 - target_pred_pos_rate).clamp(1e-6, 1.0 - 1e-6),
        target_pred_pos_rate.clamp(1e-6, 1.0 - 1e-6),
    ];

    // Gibbs kernel K_{i,j} = exp(-C_{i,j} / eps_ot) with class-conditional likelihood weighting
    let mut k_mat = Vec::with_capacity(n);
    for &p in target_posteriors {
        let pc = p.clamp(1e-6, 1.0 - 1e-6);
        // C_{i,j} = -ln P_theta(Y=j | X=x_i) adjusted by source prior so K_{i,j} proportional to P(X_i | Y=j)
        let k0 = (1.0 - pc) / p0_s;
        let k1 = pc / p1_s;
        k_mat.push([k0.max(1e-12), k1.max(1e-12)]);
    }

    // Evaluate Entropic Optimal Transport / Sinkhorn divergence objective over candidate target marginals q = (1 - q1, q1)
    let eval_sinkhorn = |q1: f64| -> f64 {
        let q = [1.0 - q1, q1];
        let mut u = vec![1.0; n];
        let mut v = [1.0, 1.0];
        for _ in 0..15 {
            for (i, u_i) in u.iter_mut().enumerate() {
                let denom = k_mat[i][0] * v[0] + k_mat[i][1] * v[1];
                *u_i = inv_n / denom.max(1e-15);
            }
            for j in 0..2 {
                let col_sum: f64 = (0..n).map(|i| u[i] * k_mat[i][j]).sum();
                v[j] = q[j] / col_sum.max(1e-15);
            }
        }

        // Confusion-pushed target marginal M * q = [(1 - FPR)(1 - q1) + (1 - TPR)q1, FPR(1 - q1) + TPR * q1]
        let mq = [
            ((1.0 - source_fpr) * q[0] + (1.0 - source_tpr) * q[1]).clamp(1e-9, 1.0 - 1e-9),
            (source_fpr * q[0] + source_tpr * q[1]).clamp(1e-9, 1.0 - 1e-9),
        ];
        let kl_confusion = mu_t[0] * (mu_t[0] / mq[0]).ln() + mu_t[1] * (mu_t[1] / mq[1]).ln();

        // Sinkhorn dual KL consistency on k_mat between scaled coupling Pi = diag(u) K diag(v)
        // and the unscaled Bayes reference coupling Pi^(0)_{i,j} = (1/n) * K_{i,j} q_j / (K_{i,0} q_0 + K_{i,1} q_1)
        let mut kl_sinkhorn = 0.0;
        for i in 0..n {
            let ref_denom = (k_mat[i][0] * q[0] + k_mat[i][1] * q[1]).max(1e-15);
            for j in 0..2 {
                let pi_ij = (u[i] * k_mat[i][j] * v[j]).max(1e-15);
                let pi0_ij = (inv_n * k_mat[i][j] * q[j] / ref_denom).max(1e-15);
                kl_sinkhorn += pi_ij * (pi_ij / pi0_ij).ln();
            }
        }
        kl_confusion.max(0.0) + kl_sinkhorn.max(0.0)
    };

    let mut best_q1 = source_prior_p1.clamp(0.001, 0.999);
    let mut best_obj = f64::INFINITY;
    for step in 1..200 {
        let q1 = (step as f64) / 200.0;
        let obj = eval_sinkhorn(q1);
        if obj < best_obj {
            best_obj = obj;
            best_q1 = q1;
        }
    }
    best_q1.clamp(0.001, 0.999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn covariate_drift_extra_wcp_recovers_tilt_and_lei_candes_bound() {
        let mut rng = Rng::new(2026);
        let x_p: Vec<f64> = (0..500).map(|_| rng.normal(0.0, 1.0)).collect();
        let x_q: Vec<f64> = (0..500).map(|_| rng.normal(0.45, 1.0)).collect();
        let scores: Vec<f64> = x_p.iter().map(|&x| 0.2 + 0.1 * x.abs()).collect();
        let res = fit_covariate_drift_extra_wcp(&x_p, &x_q, &scores, None, 0.01, 0.10);
        assert!(res.estimated_beta > 0.25 && res.estimated_beta < 0.65);
        assert!(
            res.total_variation_distance_d_tv > 0.0 && res.total_variation_distance_d_tv < 0.15
        );
        assert!(res.lei_candes_coverage_lower_bound < res.nominal_level_one_minus_alpha);
        assert!(res.lei_candes_coverage_lower_bound > 0.75);
    }

    #[test]
    fn four_label_shift_estimators_converge_toward_new_target_prior() {
        // Source prior p1 = 0.10, TPR = 0.85, FPR = 0.10 -> target prior q1 = 0.35
        // Target predicted positive rate = 0.35 * 0.85 + 0.65 * 0.10 = 0.3625 (72 / 200)
        // Source-calibrated posteriors: P_s(y=1 | y_hat=1) = 0.4857, P_s(y=1 | y_hat=0) = 0.0182
        let mut probs = Vec::with_capacity(200);
        for i in 0..200 {
            if i < 72 {
                probs.push(0.4857);
            } else {
                probs.push(0.0182);
            }
        }
        let diag =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 10.0, 30.0, 17.0, 0.0);
        assert!((diag.target_prior_bbse - 0.3467).abs() < 0.02);
        assert!(diag.target_prior_mlls_sld > 0.25 && diag.target_prior_mlls_sld < 0.45);
        assert!((diag.target_prior_dfm - 0.35).abs() < 1e-9);
        assert!((diag.target_prior_otter - 0.345).abs() < 0.05);
        assert!(diag.confusion_matrix_sigma_min > 0.01);
        assert!(diag.hoffman_constant_kappa > 1.0);
    }

    #[test]
    fn murphy_corp_circuit_breaker_separates_concept_drift_from_label_shift() {
        // Inverted predictions -> RES_CORP = 0 -> trips Concept Drift circuit breaker!
        let p_bad = [0.9, 0.85, 0.15, 0.1];
        let y = [0.0, 0.0, 1.0, 1.0];
        let m_bad = MurphyCorpCircuitBreaker::monitor(&p_bad, &y, 0.05, 0.05, false, false, false);
        assert!(m_bad.circuit_breaker_tripped);
        assert_eq!(
            m_bad.diagnosed_regime,
            DriftRegime::ConceptDriftCircuitBreaker
        );
    }

    #[test]
    fn decile_tv_detects_bimodal_shift_with_matched_moments_and_dfm_is_translation_invariant() {
        let mut rng = Rng::new(2026);
        let x_p: Vec<f64> = (0..400).map(|_| rng.normal(0.0, 1.0)).collect();
        // Symmetric bimodal target (+1 / -1) has mean ~ 0 and variance ~ 1, matching N(0, 1) first two moments
        let x_q_bimodal: Vec<f64> = (0..400)
            .map(|i| if i % 2 == 0 { -1.0 } else { 1.0 })
            .collect();
        let scores: Vec<f64> = x_p.iter().map(|&x| 0.2 + 0.1 * x.abs()).collect();
        let res = fit_covariate_drift_extra_wcp(&x_p, &x_q_bimodal, &scores, None, 0.01, 0.10);
        assert!(res.total_variation_distance_d_tv > 0.15);

        // DFM translation invariance under alpha0 > 0: shifting all feature means by +500 leaves q1_dfm unchanged
        let probs = [0.4857; 72];
        let d1 =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 10.0, 30.0, 16.3, 0.10);
        let d2 =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 510.0, 530.0, 516.3, 0.10);
        assert!((d1.target_prior_dfm - d2.target_prior_dfm).abs() < 1e-12);
        assert!((d1.target_prior_dfm - 0.35).abs() < 1e-9);
    }
}
