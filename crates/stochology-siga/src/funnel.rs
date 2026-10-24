//! The 4 Eliminatory Filters (§3.1): Sequential Pruning Before Inference.
//!
//! - **Filter 1 (Decision & Resolution):** $p^* = \frac{C}{\eta L} \in (0, 1)$, $\mathrm{EVPI} > C_{\text{total}}$, and Invariant $\Phi_5$.
//! - **Filter 2 (Effective Sample Size & Causal Specification):** Moulton-Campbell $n_{\mathrm{eff}}(p) \ge n_{\min}$,
//!   declarative Markov Blanket specification check, and ban on conditioning on colliders (Berkson's paradox).
//! - **Filter 3 (Generative Physics & Prior Predictive Check):** Simulation-Based Calibration (SBC $\chi^2$
//!   rank uniformity) and declarative SHELF quartile domain coherence check.
//! - **Filter 4 (Cryptographic Pre-Registration & Cross-Authorship):** SHA-256 seal, independent
//!   cross-authorship (`null_model != decision_rule != adversarial_reviewer`), and numeric exit clause.

use stochology_core::{is_valid_sha256_hex, verify_resolution_phi5};
use stochology_lenses::decision::CostLossRule;

/// Candidate hypothesis/model submitted to the 4 Eliminatory Filters (§3.1).
#[derive(Debug, Clone)]
pub struct FunnelCandidate {
    // Filter 1
    pub who_decides: String,
    pub altered_action: String,
    pub independent_unit: String,
    pub prior_probability_p: f64,
    pub protection_cost_c: f64,
    pub event_loss_l: f64,
    pub efficacy_eta: f64,
    pub decision_volume_units: f64,
    pub total_modeling_cost: f64,

    // Filter 2
    pub effective_tail_n_eff_p: f64,
    pub minimum_required_n_eff: f64,
    pub markov_blanket_mapped: bool,
    pub conditions_on_collider_berkson: bool,

    // Filter 3
    pub sbc_rank_histogram: Vec<usize>,
    pub sbc_chi2_critical: f64,
    pub shelf_quartiles_q25_q50_q75: (f64, f64, f64),
    pub physical_domain_bounds: (f64, f64),

    // Filter 4
    pub preregistered_plan_text: String,
    pub declared_sha256_seal: String,
    pub author_null_model: String,
    pub author_decision_rule: String,
    pub author_adversarial_reviewer: String,
    pub numeric_exit_clause: String,
}

/// Verdict of the 4 Eliminatory Filters.
#[derive(Debug, Clone, PartialEq)]
pub struct FunnelResult {
    pub calculated_evpi: f64,
    pub filter_1_approved: bool,
    pub filter_2_approved: bool,
    pub sbc_chi2_statistic: f64,
    pub filter_3_approved: bool,
    pub filter_4_approved: bool,
    /// First filter (1..=4) where the candidate was pruned, or `None` if all 4 passed.
    pub pruned_at_filter: Option<u8>,
    pub pruning_reason: Option<String>,
    pub approved_all_filters: bool,
}

/// Evaluates the uniformity of Simulation-Based Calibration (SBC) rank counts via Pearson's $\chi^2$.
pub fn sbc_rank_chi2(rank_bins: &[usize]) -> f64 {
    assert!(!rank_bins.is_empty());
    let total: f64 = rank_bins.iter().map(|&obs| obs as f64).sum();
    if total <= 0.0 {
        return f64::INFINITY;
    }
    let expected = total / (rank_bins.len() as f64);
    rank_bins
        .iter()
        .map(|&obs| {
            let diff = (obs as f64) - expected;
            (diff * diff) / expected
        })
        .sum()
}

/// Lanczos approximation for $\ln \Gamma(z)$ ($z > 0$) with 9 coefficients (precision $\sim 10^{-15}$).
pub fn ln_gamma_lanczos(z: f64) -> f64 {
    assert!(z > 0.0, "ln_gamma_lanczos requires z > 0");
    const COEFFS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let z_minus_1 = z - 1.0;
    let mut x = COEFFS[0];
    for (i, &c) in COEFFS.iter().enumerate().skip(1) {
        x += c / (z_minus_1 + (i as f64));
    }
    let t = z_minus_1 + 7.5;
    0.5 * std::f64::consts::TAU.ln() + (z_minus_1 + 0.5) * t.ln() - t + x.ln()
}

/// Regularized incomplete Beta function $I_x(a, b) = \frac{B(x; a, b)}{B(a, b)}$ evaluated via
/// Lentz's continued-fraction algorithm (DLMF §8.17.22).
pub fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    assert!(a > 0.0 && b > 0.0, "requires a > 0 and b > 0");
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let sym_threshold = (a + 1.0) / (a + b + 2.0);
    if x > sym_threshold {
        return 1.0 - regularized_incomplete_beta(1.0 - x, b, a);
    }
    let ln_beta = ln_gamma_lanczos(a) + ln_gamma_lanczos(b) - ln_gamma_lanczos(a + b);
    let front = (a * x.ln() + b * (1.0 - x).ln() - ln_beta).exp() / a;

    // Modified Lentz continued fraction for I_x(a, b)
    let tiny = 1e-30_f64;
    let mut c = 1.0_f64;
    let mut d = 1.0 - ((a + b) * x / (a + 1.0));
    if d.abs() < tiny {
        d = tiny;
    }
    d = 1.0 / d;
    let mut f = d;

    for m in 1..=200 {
        let m_f = m as f64;
        let two_m = 2.0 * m_f;
        // Even step d_{2m}
        let num_even = (m_f * (b - m_f) * x) / ((a + two_m - 1.0) * (a + two_m));
        d = 1.0 + num_even * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + num_even / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        f *= c * d;

        // Odd step d_{2m+1}
        let num_odd = -((a + m_f) * (a + b + m_f) * x) / ((a + two_m) * (a + two_m + 1.0));
        d = 1.0 + num_odd * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + num_odd / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let delta = c * d;
        f *= delta;
        if (delta - 1.0).abs() < 1e-14 {
            break;
        }
    }
    (front * f).clamp(0.0, 1.0)
}

/// Fits a conjugate Beta prior $\mathrm{Beta}(\hat{\alpha}, \hat{\beta})$ to SHELF elicited quartiles
/// $(q_{25}, q_{50}, q_{75})$ (O'Hagan / Oakley — Filter 3, §3.1) by minimizing the exact CDF residual
/// $\bigl(I_{q_{25}}(\alpha, \beta) - 0.25\bigr)^2 + \bigl(I_{q_{50}}(\alpha, \beta) - 0.50\bigr)^2 + \bigl(I_{q_{75}}(\alpha, \beta) - 0.75\bigr)^2$.
/// Returns `(alpha_hat, beta_hat, cdf_rmse)`.
pub fn fit_shelf_beta_prior(q25: f64, q50: f64, q75: f64) -> (f64, f64, f64) {
    assert!(
        0.0 < q25 && q25 < q50 && q50 < q75 && q75 < 1.0,
        "requires 0 < q25 < q50 < q75 < 1"
    );
    // Method-of-moments warm start from median and IQR
    let iqr = (q75 - q25).max(1e-4);
    let sd_approx = (iqr / 1.349).max(1e-4);
    let mu0 = q50.clamp(0.01, 0.99);
    let conc0 = ((mu0 * (1.0 - mu0)) / (sd_approx * sd_approx) - 1.0).clamp(1.0, 400.0);

    let obj = |a: f64, b: f64| -> f64 {
        let r25 = regularized_incomplete_beta(q25, a, b) - 0.25;
        let r50 = regularized_incomplete_beta(q50, a, b) - 0.50;
        let r75 = regularized_incomplete_beta(q75, a, b) - 0.75;
        ((r25 * r25 + r50 * r50 + r75 * r75) / 3.0).sqrt()
    };

    let mut best_a = (mu0 * conc0).max(0.2);
    let mut best_b = ((1.0 - mu0) * conc0).max(0.2);
    let mut best_err = obj(best_a, best_b);

    for scale in [0.5, 0.2, 0.08, 0.02, 0.005] {
        for da in -4..=4 {
            for db in -4..=4 {
                let cand_a = (best_a * (1.0 + (da as f64) * scale)).clamp(0.1, 1000.0);
                let cand_b = (best_b * (1.0 + (db as f64) * scale)).clamp(0.1, 1000.0);
                let err = obj(cand_a, cand_b);
                if err < best_err {
                    best_err = err;
                    best_a = cand_a;
                    best_b = cand_b;
                }
            }
        }
    }
    (best_a, best_b, best_err)
}

/// Evaluates the 4 Eliminatory Filters in strict sequential order (Filter 1 -> 2 -> 3 -> 4).
pub fn evaluate_four_filter_funnel(c: &FunnelCandidate) -> FunnelResult {
    let params_valid = c.protection_cost_c.is_finite()
        && c.protection_cost_c > 0.0
        && c.event_loss_l.is_finite()
        && c.event_loss_l > 0.0
        && c.efficacy_eta.is_finite()
        && c.efficacy_eta > 0.0
        && c.efficacy_eta <= 1.0
        && c.prior_probability_p.is_finite()
        && c.prior_probability_p > 0.0
        && c.prior_probability_p < 1.0
        && c.decision_volume_units.is_finite()
        && c.decision_volume_units > 0.0
        && c.total_modeling_cost.is_finite()
        && c.total_modeling_cost >= 0.0;

    let denom = c.efficacy_eta * c.event_loss_l;
    let raw_p_star = if params_valid && denom > 0.0 && denom.is_finite() {
        c.protection_cost_c / denom
    } else {
        f64::NAN
    };
    let p_star_in_unit_interval = raw_p_star.is_finite() && raw_p_star > 0.0 && raw_p_star < 1.0;

    let evpi = if params_valid && p_star_in_unit_interval {
        let rule = CostLossRule::new(c.protection_cost_c, c.event_loss_l, c.efficacy_eta);
        rule.calculate_evpi(c.prior_probability_p, c.decision_volume_units)
    } else {
        0.0
    };

    // Filter 1: Decision & Resolution (p* in (0, 1) + Phi5 + EVPI > C_total)
    let f1_ok = !c.who_decides.trim().is_empty()
        && !c.altered_action.trim().is_empty()
        && params_valid
        && p_star_in_unit_interval
        && verify_resolution_phi5(&c.independent_unit, &c.altered_action)
        && evpi > c.total_modeling_cost;

    if !f1_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: false,
            filter_2_approved: false,
            sbc_chi2_statistic: 0.0,
            filter_3_approved: false,
            filter_4_approved: false,
            pruned_at_filter: Some(1),
            pruning_reason: Some(
                "Pruned at Filter 1: p* outside (0, 1), EVPI <= total_modeling_cost, or Phi5 resolution violation"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    // Filter 2: Effective Tail Sample Size n_eff(p) & Markov Blanket without Berkson Collider
    let f2_ok = c.effective_tail_n_eff_p >= c.minimum_required_n_eff
        && c.markov_blanket_mapped
        && !c.conditions_on_collider_berkson;

    if !f2_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: true,
            filter_2_approved: false,
            sbc_chi2_statistic: 0.0,
            filter_3_approved: false,
            filter_4_approved: false,
            pruned_at_filter: Some(2),
            pruning_reason: Some(
                "Pruned at Filter 2: insufficient tail n_eff(p) or conditioning on a Berkson collider"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    // Filter 3: Generative Physics (SBC rank uniformity + SHELF quartile coherence)
    let chi2 = sbc_rank_chi2(&c.sbc_rank_histogram);
    let (q25, q50, q75) = c.shelf_quartiles_q25_q50_q75;
    let (lo, hi) = c.physical_domain_bounds;
    let shelf_ok = lo <= q25 && q25 < q50 && q50 < q75 && q75 <= hi;
    let f3_ok = chi2 <= c.sbc_chi2_critical && shelf_ok;

    if !f3_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: true,
            filter_2_approved: true,
            sbc_chi2_statistic: chi2,
            filter_3_approved: false,
            filter_4_approved: false,
            pruned_at_filter: Some(3),
            pruning_reason: Some(
                "Pruned at Filter 3: SBC rank histogram non-uniform or SHELF prior violates physical domain"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    // Filter 4: Cryptographic SHA-256 Pre-Registration, Cross-Authorship, and Numeric Exit Clause
    let expected_hash = stochology_core::sha256_hex(c.preregistered_plan_text.as_bytes());
    let seal_ok =
        is_valid_sha256_hex(&c.declared_sha256_seal) && c.declared_sha256_seal == expected_hash;
    let authors_ok = !c.author_null_model.trim().is_empty()
        && !c.author_decision_rule.trim().is_empty()
        && !c.author_adversarial_reviewer.trim().is_empty()
        && c.author_null_model != c.author_decision_rule
        && c.author_adversarial_reviewer != c.author_null_model
        && c.author_adversarial_reviewer != c.author_decision_rule;
    let exit_ok = !c.numeric_exit_clause.trim().is_empty()
        && c.numeric_exit_clause.chars().any(|ch| ch.is_ascii_digit());
    let f4_ok = seal_ok && authors_ok && exit_ok;

    if !f4_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: true,
            filter_2_approved: true,
            sbc_chi2_statistic: chi2,
            filter_3_approved: true,
            filter_4_approved: false,
            pruned_at_filter: Some(4),
            pruning_reason: Some(
                "Pruned at Filter 4: invalid SHA-256 pre-registration, non-independent cross-authorship, or missing exit clause"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    FunnelResult {
        calculated_evpi: evpi,
        filter_1_approved: true,
        filter_2_approved: true,
        sbc_chi2_statistic: chi2,
        filter_3_approved: true,
        filter_4_approved: true,
        pruned_at_filter: None,
        pruning_reason: None,
        approved_all_filters: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::sha256_hex;

    fn valid_candidate() -> FunnelCandidate {
        let plan = "siga_plan_coastal_evacuation_shift_seed_2026";
        FunnelCandidate {
            who_decides: "Coastal Emergency Operations Director".into(),
            altered_action: "Trigger mandatory coastal evacuation order within 6h synoptic shift"
                .into(),
            independent_unit: "6h synoptic storm shift".into(),
            prior_probability_p: 0.05,
            protection_cost_c: 1_800.0,
            event_loss_l: 34_000.0,
            efficacy_eta: 1.0,
            decision_volume_units: 100.0,
            total_modeling_cost: 15_000.0,
            effective_tail_n_eff_p: 1_300.0,
            minimum_required_n_eff: 100.0,
            markov_blanket_mapped: true,
            conditions_on_collider_berkson: false,
            sbc_rank_histogram: vec![20, 20, 20, 20, 20],
            sbc_chi2_critical: 9.49,
            shelf_quartiles_q25_q50_q75: (0.02, 0.05, 0.09),
            physical_domain_bounds: (0.001, 0.999),
            preregistered_plan_text: plan.into(),
            declared_sha256_seal: sha256_hex(plan.as_bytes()),
            author_null_model: "inference".into(),
            author_decision_rule: "decision".into(),
            author_adversarial_reviewer: "independent_auditor".into(),
            numeric_exit_clause: "Suspend if REL_CORP > 0.015 over 14 days".into(),
        }
    }

    #[test]
    fn sound_candidate_passes_all_four_filters() {
        let c = valid_candidate();
        let res = evaluate_four_filter_funnel(&c);
        assert!(res.approved_all_filters);
        assert_eq!(res.pruned_at_filter, None);

        let (a_hat, b_hat, rmse) = fit_shelf_beta_prior(0.02, 0.05, 0.09);
        assert!(a_hat > 0.5 && b_hat > 5.0 && rmse < 0.03);
    }

    #[test]
    fn funnel_prunes_berkson_collider_at_filter2_and_same_author_at_filter4() {
        let mut c2 = valid_candidate();
        c2.conditions_on_collider_berkson = true;
        let r2 = evaluate_four_filter_funnel(&c2);
        assert_eq!(r2.pruned_at_filter, Some(2));

        let mut c4 = valid_candidate();
        c4.author_adversarial_reviewer = "inference".into();
        let r4 = evaluate_four_filter_funnel(&c4);
        assert_eq!(r4.pruned_at_filter, Some(4));

        let mut c4_no_digit = valid_candidate();
        c4_no_digit.numeric_exit_clause = "Suspend model when calibration degrades".into();
        let r4_no_digit = evaluate_four_filter_funnel(&c4_no_digit);
        assert_eq!(r4_no_digit.pruned_at_filter, Some(4));

        let mut c1_bad_pstar = valid_candidate();
        c1_bad_pstar.protection_cost_c = 40_000.0; // C >= eta * L -> p* >= 1.0
        let r1_bad_pstar = evaluate_four_filter_funnel(&c1_bad_pstar);
        assert_eq!(r1_bad_pstar.pruned_at_filter, Some(1));

        // Out-of-domain cost/loss/eta/NaN values short-circuit cleanly at Filter 1 without panicking
        let mut c1_bad_eta = valid_candidate();
        c1_bad_eta.efficacy_eta = 1.5;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_bad_eta).pruned_at_filter,
            Some(1)
        );

        let mut c1_nonpos_cost = valid_candidate();
        c1_nonpos_cost.protection_cost_c = 0.0;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_nonpos_cost).pruned_at_filter,
            Some(1)
        );

        let mut c1_nan_loss = valid_candidate();
        c1_nan_loss.event_loss_l = f64::NAN;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_nan_loss).pruned_at_filter,
            Some(1)
        );
    }
}
