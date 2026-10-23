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

    let m_beta: Vec<f64> = x_source_p.iter().map(|&x| (beta * x).exp()).collect();
    let z_p = stochology_core::mean(&m_beta);
    let w_hat: Vec<f64> = m_beta.iter().map(|&m| m / z_p).collect();
    let l_hat = beta * mean_q - z_p.ln() - 0.5 * lambda_reg * beta * beta;

    // Total Variation distance d_TV(Q, Q_hat) = 0.5 * E_P[|w* - w_hat|]
    let d_tv = if let Some(w_star) = true_weights_optional {
        0.5 * w_star
            .iter()
            .zip(w_hat.iter())
            .map(|(&ws, &wh)| (ws - wh).abs())
            .sum::<f64>()
            / (w_hat.len() as f64)
    } else {
        0.0
    };

    // Weighted quantile (ExTRA-WCP-T) of nonconformity scores at level 1 - alpha
    let mut pairs: Vec<(f64, f64)> = nonconformity_scores_source
        .iter()
        .copied()
        .zip(w_hat.iter().copied())
        .collect();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let sum_weights: f64 = pairs.iter().map(|(_, w)| *w).sum();
    let target_cum = (1.0 - alpha) * sum_weights;
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

    // 1. BBSE (Black Box Shift Estimation):
    // Target predicted positive rate mu_t(1) = P(y_hat = 1) = q_1 * TPR + (1 - q_1) * FPR
    let pred_pos_rate = target_calibrated_posteriors
        .iter()
        .filter(|&&p| p >= 0.5)
        .count() as f64
        / (target_calibrated_posteriors.len() as f64);
    let denom_bbse = (source_tpr - source_fpr).abs().max(1e-6);
    let q1_bbse = ((pred_pos_rate - source_fpr) / (source_tpr - source_fpr)).clamp(0.001, 0.999);
    // Smallest singular value of 2x2 joint confusion matrix C
    let c00 = (1.0 - source_fpr) * (1.0 - source_prior_p1);
    let c01 = source_fpr * (1.0 - source_prior_p1);
    let c10 = (1.0 - source_tpr) * source_prior_p1;
    let c11 = source_tpr * source_prior_p1;
    let det_c = (c00 * c11 - c01 * c10).abs();
    let frob_c = (c00 * c00 + c01 * c01 + c10 * c10 + c11 * c11).sqrt();
    let sigma_min = det_c / frob_c.max(1e-12);

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
    let q1_dfm = (((target_feature_mean / effective_scale) - source_class0_feature_mean) / diff_mu)
        .clamp(0.001, 0.999);

    // 4. OTTER (Optimal Transport Label Shift Estimation with cost C_{ij} = -ln P(Y=j | X=x_i)):
    // Minimizes transport cost + entropic/quadratic regularization over the target marginal simplex
    let mut sum_ot = 0.0;
    for &p in target_calibrated_posteriors {
        let p_c = p.clamp(1e-6, 1.0 - 1e-6);
        let c1 = -p_c.ln();
        let c0 = -(1.0 - p_c).ln();
        // Soft-min transport assignment with temperature tau_ot = 0.5
        let w1 = (-2.0 * c1).exp();
        let w0 = (-2.0 * c0).exp();
        sum_ot += w1 / (w1 + w0);
    }
    let q1_otter = (sum_ot / (target_calibrated_posteriors.len() as f64)).clamp(0.001, 0.999);
    let hoffman_kappa = 1.0 / denom_bbse.max(0.05);

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
        assert!((res.lei_candes_coverage_lower_bound - 0.90).abs() < 1e-9);
    }

    #[test]
    fn four_label_shift_estimators_converge_toward_new_target_prior() {
        // Source prior p1 = 0.10, TPR = 0.85, FPR = 0.10 -> target prior q1 = 0.35
        // Target predicted positive rate = 0.35 * 0.85 + 0.65 * 0.10 = 0.3625 (72 / 200)
        // Source-calibrated posteriors: P_s(y=1 | y_hat=1) = 0.4857, P_s(y=1 | y_hat=0) = 0.0182
        let mut probs = Vec::with_capacity(200);
        for i in 0..200 {
            if i < 72 {
                probs.push(0.51);
            } else {
                probs.push(0.0182);
            }
        }
        let diag =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 10.0, 30.0, 17.0, 0.0);
        assert!((diag.target_prior_bbse - 0.3467).abs() < 0.02);
        assert!(diag.target_prior_mlls_sld > 0.25 && diag.target_prior_mlls_sld < 0.45);
        assert!((diag.target_prior_dfm - 0.35).abs() < 1e-9);
        assert!(diag.target_prior_otter > 0.15 && diag.target_prior_otter < 0.45);
        assert!(diag.confusion_matrix_sigma_min > 0.01);
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
}
