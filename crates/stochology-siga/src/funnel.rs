//! The 4 Eliminatory Filters (§3.1): Sequential Pruning Before Inference.
//!
//! - **Filter 1 (Decision & Resolution):** $\mathrm{EVPI} > C_{\text{total}}$ and Invariant $\Phi_5$.
//! - **Filter 2 (Effective Sample Size & Causal Structure):** Moulton-Campbell $n_{\mathrm{eff}}(p) \ge n_{\min}$,
//!   Markov Blanket identification, and strict ban on conditioning on colliders (Berkson's paradox).
//! - **Filter 3 (Generative Physics & Prior Predictive Check):** Simulation-Based Calibration (SBC $\chi^2$
//!   rank uniformity) and SHELF quartile prior elicitation.
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

/// Evaluates the 4 Eliminatory Filters in strict sequential order (Filter 1 -> 2 -> 3 -> 4).
pub fn evaluate_four_filter_funnel(c: &FunnelCandidate) -> FunnelResult {
    let rule = CostLossRule::new(c.protection_cost_c, c.event_loss_l, c.efficacy_eta);
    let evpi = rule.calculate_evpi(c.prior_probability_p, c.decision_volume_units);

    // Filter 1: Decision & Resolution (Phi5 + EVPI > C_total)
    let f1_ok = !c.who_decides.trim().is_empty()
        && !c.altered_action.trim().is_empty()
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
                "Pruned at Filter 1: EVPI <= total_modeling_cost or Phi5 resolution violation"
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
        let plan = "siga_plan_sepsis_shift_seed_2026";
        FunnelCandidate {
            who_decides: "ICU Attending Physician".into(),
            altered_action: "Trigger sepsis bundle within 1h on the shift".into(),
            independent_unit: "12h ICU shift".into(),
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
    }
}
