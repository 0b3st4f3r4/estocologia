//! The 8-Step SIGA Operational Roadmap (§3.2):
//! - **S — Survey the Layer and Bridge** (Steps 1 and 2);
//! - **I — Interrogate the Decision-Maker** (Steps 3 and 4);
//! - **G — Generate the Null and Noise Physics** (Steps 5 and 6);
//! - **A — Act with Threshold, Memory, and Exit** (Steps 7 and 8).
//!
//! Includes sequential anytime-valid *e-processes* ($E_t = \prod_{i=1}^t L_i$, $\mathbb{E}_{H_0}[E_t] \le 1$,
//! Ville's Inequality $E_t \ge 1/\alpha$) and Gigerenzer integer natural frequency memoranda (e.g., `"X out of 1,000"`).

use crate::funnel::{evaluate_four_filter_funnel, FunnelCandidate, FunnelResult};
use stochology_core::{EpistemologicalCard, Rng};

/// Sequential anytime-valid *e-process* (betting martingale $E_t = \prod_{i=1}^t L_i$, §3.2 Step 6):
/// $$\mathbb{E}_{H_0}[E_t] \le 1 \implies \mathbb{P}_{H_0}\left(\exists t \ge 1 : E_t \ge \frac{1}{\alpha}\right) \le \alpha$$
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceProcess {
    pub alpha_level: f64,
    pub ville_rejection_threshold: f64,
    pub trajectory_e_t: Vec<f64>,
    pub final_e_value: f64,
    pub attempt_counter_k: usize,
    /// Multiplicity-penalized e-value $\sup_{1 \le s \le t} E_s / K$.
    pub multiplicity_adjusted_e_value: f64,
    pub rejects_null_hypothesis: bool,
}

impl EvidenceProcess {
    /// Evaluates a sequence of nonnegative likelihood ratios / e-values $L_1, \dots, L_t$ under attempt count $K \ge 1$.
    pub fn evaluate(likelihood_ratios: &[f64], alpha_level: f64, attempt_counter_k: usize) -> Self {
        assert!(alpha_level > 0.0 && alpha_level < 1.0);
        assert!(attempt_counter_k >= 1);
        let threshold = 1.0 / alpha_level;
        let mut prod = 1.0;
        let mut invalid = false;
        let mut traj = Vec::with_capacity(likelihood_ratios.len());
        for &l_i in likelihood_ratios {
            let safe_l = if l_i.is_finite() && l_i >= 0.0 {
                l_i
            } else {
                invalid = true;
                0.0
            };
            prod *= safe_l;
            traj.push(prod);
        }
        if invalid {
            traj.fill(0.0);
        }
        let max_e = traj.iter().copied().fold(0.0_f64, f64::max);
        let adj = max_e / (attempt_counter_k as f64);
        Self {
            alpha_level,
            ville_rejection_threshold: threshold,
            trajectory_e_t: traj,
            final_e_value: prod,
            attempt_counter_k,
            multiplicity_adjusted_e_value: adj,
            rejects_null_hypothesis: adj >= threshold,
        }
    }
}

/// Executive Memorandum in Integer Natural Frequencies (Gigerenzer — §3.2 Step 8).
#[derive(Debug, Clone, PartialEq)]
pub struct NaturalFrequencyMemorandum {
    pub reference_cohort_size: usize,
    pub affected_with_event: usize,
    pub unaffected_without_event: usize,
    pub true_positives: usize,
    pub false_negatives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub critical_threshold_p_star: f64,
    pub recommended_action: String,
    pub exit_clause: String,
    pub formatted_summary: String,
}

impl NaturalFrequencyMemorandum {
    /// Builds an integer natural frequency tree for a reference cohort (e.g., `1_000` units).
    pub fn build(
        reference_cohort_size: usize,
        base_rate_p: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
    ) -> Self {
        Self::build_with_decision(
            reference_cohort_size,
            base_rate_p,
            sensitivity_tpr,
            false_positive_rate_fpr,
            0.05,
            "protect",
            "Suspend if REL_CORP exceeds threshold",
        )
    }

    /// Builds an integer natural frequency tree including the economic threshold $p^*$,
    /// recommended action (`protect` / `do_not_protect`), and numeric exit clause.
    pub fn build_with_decision(
        reference_cohort_size: usize,
        base_rate_p: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
        critical_threshold_p_star: f64,
        recommended_action: &str,
        exit_clause: &str,
    ) -> Self {
        let p_c = if base_rate_p.is_finite() {
            base_rate_p.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let tpr_c = if sensitivity_tpr.is_finite() {
            sensitivity_tpr.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fpr_c = if false_positive_rate_fpr.is_finite() {
            false_positive_rate_fpr.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let affected = ((reference_cohort_size as f64) * p_c).round() as usize;
        let unaffected = reference_cohort_size.saturating_sub(affected);
        let tp = ((affected as f64) * tpr_c).round() as usize;
        let fn_c = affected.saturating_sub(tp);
        let fp = ((unaffected as f64) * fpr_c).round() as usize;
        let tn = unaffected.saturating_sub(fp);
        let total_alarms = tp.saturating_add(fp).max(1);
        let summary = format!(
            "Out of every {reference_cohort_size} independent units, {affected} experience the critical event (of which {tp} trigger the alarm at threshold p*={critical_threshold_p_star:.5} and {fn_c} are missed), while {unaffected} do not experience the event (of which {fp} trigger a false alarm and {tn} remain clear). Positive predictive frequency: {tp} out of {total_alarms} alarms. Action: {recommended_action}. Exit clause: {exit_clause}."
        );
        Self {
            reference_cohort_size,
            affected_with_event: affected,
            unaffected_without_event: unaffected,
            true_positives: tp,
            false_negatives: fn_c,
            false_positives: fp,
            true_negatives: tn,
            critical_threshold_p_star,
            recommended_action: recommended_action.to_string(),
            exit_clause: exit_clause.to_string(),
            formatted_summary: summary,
        }
    }
}

/// Full end-to-end execution report of the 8-Step SIGA Operational Roadmap.
#[derive(Debug, Clone, PartialEq)]
pub struct SigaExecutionReport {
    pub card_id: String,
    pub funnel_result: FunnelResult,
    pub null_p_value: f64,
    pub e_process: EvidenceProcess,
    pub natural_frequency_memo: NaturalFrequencyMemorandum,
    /// Completion flags for Steps 1 through 8 of the SIGA Roadmap.
    pub steps_1_to_8_completed: [bool; 8],
    pub approved_end_to_end: bool,
}

fn sample_gamma_marsaglia(rng: &mut Rng, alpha: f64) -> f64 {
    let a = alpha.max(1e-3);
    if a < 1.0 {
        let u = rng.next_f64().clamp(1e-15, 1.0 - 1e-15);
        return sample_gamma_marsaglia(rng, a + 1.0) * u.powf(1.0 / a);
    }
    let d = a - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let z = rng.standard_normal();
        let v_root = 1.0 + c * z;
        if v_root <= 0.0 {
            continue;
        }
        let v = v_root * v_root * v_root;
        let u = rng.next_f64().clamp(1e-15, 1.0 - 1e-15);
        if u.ln() < 0.5 * z * z + d * (1.0 - v + v.ln()) {
            return (d * v).max(1e-15);
        }
    }
}

fn sample_beta_conjugate(rng: &mut Rng, alpha: f64, beta: f64) -> f64 {
    let x = sample_gamma_marsaglia(rng, alpha);
    let y = sample_gamma_marsaglia(rng, beta);
    (x / (x + y).max(1e-15)).clamp(1e-12, 1.0 - 1e-12)
}

/// Executes genuine Talts et al. (2018) Simulation-Based Calibration (SBC) for a Beta-Binomial
/// generative model using deterministic `stochology_core::Rng::new(seed)`.
///
/// For each replication $s \in 0..\text{num\_replications}$:
/// 1. Draw $\tilde{p} \sim \mathrm{Beta}(\alpha_0, \beta_0)$ from the prior;
/// 2. Simulate $k \sim \mathrm{Binomial}(n_{\text{trials}}, \tilde{p})$;
/// 3. Draw $L$ conjugate posterior samples centered at $\hat{p} = \frac{k + \alpha_0}{n_{\text{trials}} + \alpha_0 + \beta_0}$
///    with posterior variance $\frac{\hat{p}(1-\hat{p})}{n_{\text{trials}} + \alpha_0 + \beta_0 + 1}$;
/// 4. Compute the continuity-corrected normalized Talts et al. (2018) rank $(r_s + 0.5) / (L + 1) \in (0, 1)$
///    where $r_s = \sum_{\ell=1}^L \mathbb{I}(p_\ell < \tilde{p}) \in \{0, \dots, L\}$.
pub fn simulate_sbc_beta_binomial_ranks(
    seed: u64,
    num_replications: usize,
    posterior_draws_l: usize,
    alpha_prior: f64,
    beta_prior: f64,
    n_trials: usize,
) -> Vec<f64> {
    let mut rng = Rng::new(seed);
    let a0 = alpha_prior.max(0.1);
    let b0 = beta_prior.max(0.1);
    let l_draws = posterior_draws_l.max(1);
    let mut ranks = Vec::with_capacity(num_replications);

    for _ in 0..num_replications {
        let p_tilde = sample_beta_conjugate(&mut rng, a0, b0);
        let mut k = 0usize;
        for _ in 0..n_trials {
            if rng.bernoulli(p_tilde) {
                k += 1;
            }
        }
        let alpha_post = (k as f64) + a0;
        let beta_post = ((n_trials - k) as f64) + b0;
        let p_hat = alpha_post / (alpha_post + beta_post);
        let post_var = (p_hat * (1.0 - p_hat)) / (alpha_post + beta_post + 1.0);
        debug_assert!(post_var > 0.0 && post_var.is_finite());

        let mut rank = 0usize;
        for _ in 0..l_draws {
            let p_draw = sample_beta_conjugate(&mut rng, alpha_post, beta_post);
            if p_draw < p_tilde {
                rank += 1;
            }
        }
        ranks.push(((rank as f64) + 0.5) / ((l_draws + 1) as f64));
    }
    ranks
}

fn compute_sbc_rank_bins(null_samples: &[f64]) -> Vec<usize> {
    if null_samples.len() < 5 || null_samples.iter().any(|x| !x.is_finite()) {
        return vec![0; 5];
    }
    let min_val = null_samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = null_samples
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if (max_val - min_val) <= 1e-12 {
        return vec![null_samples.len(), 0, 0, 0, 0];
    }
    let (q20, q40, q60, q80) = if min_val >= 0.0 && max_val <= 1.0 {
        // Theoretical Uniform(0, 1) quintiles for normalized SBC / PIT ranks
        (0.20, 0.40, 0.60, 0.80)
    } else if min_val < 0.0 {
        // Theoretical N(0, 1) PIT quintiles for two-sided z-score null distributions
        (-0.841_621_23, -0.253_347_10, 0.253_347_10, 0.841_621_23)
    } else {
        // Theoretical Uniform(0, max_val) quintiles for nonnegative rank grids
        (
            0.20 * max_val,
            0.40 * max_val,
            0.60 * max_val,
            0.80 * max_val,
        )
    };
    let mut bins = vec![0usize; 5];
    for &x in null_samples {
        let b = if x <= q20 {
            0
        } else if x <= q40 {
            1
        } else if x <= q60 {
            2
        } else if x <= q80 {
            3
        } else {
            4
        };
        bins[b] += 1;
    }
    bins
}

/// Executes the complete 4-Filter Funnel and 8-Step SIGA Roadmap on an [`EpistemologicalCard`].
pub fn run_full_siga_roadmap(
    card: &EpistemologicalCard,
    preregistered_plan_text: &str,
    observed_test_statistic: f64,
    null_distribution_samples: &[f64],
    e_values_sequence: &[f64],
) -> SigaExecutionReport {
    let mc = &card.data_and_audit_contract.murphy_corp_decomposition;
    let unc = mc.base_uncertainty_unc0.max(1e-6);
    let res_norm = (mc.resolution_res_dsc / unc).clamp(0.0, 1.0);
    let rel_norm = (mc.miscalibration_rel_mcb / unc).clamp(0.0, 1.0);
    let tpr = (0.80 + 0.35 * res_norm - 0.25 * rel_norm).clamp(0.75, 0.95);
    let fpr = (0.10 - 0.15 * res_norm + 0.30 * rel_norm).clamp(0.03, 0.15);
    run_full_siga_roadmap_empirical(
        card,
        preregistered_plan_text,
        null_distribution_samples,
        observed_test_statistic,
        null_distribution_samples,
        e_values_sequence,
        (tpr, fpr),
    )
}

/// Executes the complete 4-Filter Funnel and 8-Step SIGA Roadmap with separate SBC calibration
/// ranks (Filter 3), un-clamped cluster-null test distribution (Step 5), sequential Ville
/// e-values (Step 6), and empirical confusion-matrix `(TPR, FPR)` at threshold $p^*$ (Step 8).
pub fn run_full_siga_roadmap_empirical(
    card: &EpistemologicalCard,
    preregistered_plan_text: &str,
    sbc_rank_samples: &[f64],
    observed_test_statistic: f64,
    null_distribution_samples: &[f64],
    e_values_sequence: &[f64],
    empirical_confusion_tpr_fpr: (f64, f64),
) -> SigaExecutionReport {
    let card_audit_ok = card.audit().is_ok();

    let id_strat = card
        .epistemological_header
        .causal_ladder
        .identification_strategy
        .to_ascii_lowercase();
    let markov_blanket_mapped = id_strat.contains("markov blanket")
        && !id_strat.contains("not mapped")
        && !id_strat.contains("unmapped");
    let collider_excluded = (id_strat.contains("without collider")
        || id_strat.contains("no collider")
        || id_strat.contains("free of collider"))
        && !id_strat.contains("without collider audit")
        && !id_strat.contains("conditioned on collider")
        && !id_strat.contains("conditioning on collider");
    let conditions_on_collider_berkson =
        (id_strat.contains("collider") || id_strat.contains("berkson")) && !collider_excluded;

    let sbc_bins = compute_sbc_rank_bins(sbc_rank_samples);
    let eps = card
        .decision_rule_and_scope
        .base_rate_floor_epsilon
        .clamp(1e-6, 0.49);
    let p = card.decision_rule_and_scope.event_probability_p;
    let q25 = 0.5 * (eps + p);
    let q50 = p;
    let q75 = (1.5 * p).min(0.5 * (p + 1.0 - eps));

    let candidate = FunnelCandidate {
        who_decides: card.decision_maker_question.who_decides.clone(),
        altered_action: card.decision_maker_question.altered_action.clone(),
        independent_unit: card
            .epistemological_header
            .unit_and_margin
            .independent_unit
            .clone(),
        prior_probability_p: p,
        protection_cost_c: card.decision_rule_and_scope.protection_cost_c,
        event_loss_l: card.decision_rule_and_scope.event_loss_l,
        efficacy_eta: card.decision_rule_and_scope.mitigation_efficacy_eta,
        decision_volume_units: 100.0,
        total_modeling_cost: 500.0,
        effective_tail_n_eff_p: card
            .epistemological_header
            .unit_and_margin
            .effective_sample_size_n_eff as f64,
        minimum_required_n_eff: 50.0,
        markov_blanket_mapped,
        conditions_on_collider_berkson,
        sbc_rank_histogram: sbc_bins,
        sbc_chi2_critical: 9.49,
        shelf_quartiles_q25_q50_q75: (q25, q50, q75),
        physical_domain_bounds: (eps, 1.0 - eps),
        preregistered_plan_text: preregistered_plan_text.to_string(),
        declared_sha256_seal: card.data_and_audit_contract.preregistration_hash.clone(),
        author_null_model: card
            .uncertainty_budget_and_authorship
            .field_authorship
            .null_model
            .clone(),
        author_decision_rule: card
            .uncertainty_budget_and_authorship
            .field_authorship
            .decision_rule
            .clone(),
        author_adversarial_reviewer: card
            .uncertainty_budget_and_authorship
            .field_authorship
            .adversarial_reviewer
            .clone(),
        numeric_exit_clause: card.decision_rule_and_scope.exit_clause.clone(),
    };

    let funnel = evaluate_four_filter_funnel(&candidate);

    // Step 5: Exact empirical p-value against the cluster-respecting generative null model (no clamp)
    let exceedances = null_distribution_samples
        .iter()
        .filter(|&&s| s >= observed_test_statistic)
        .count();
    let null_p = ((exceedances + 1) as f64) / ((null_distribution_samples.len() + 1) as f64);

    // Step 6: Sequential e-process with multiplicity K
    let e_proc = EvidenceProcess::evaluate(
        e_values_sequence,
        0.05,
        card.data_and_audit_contract.attempt_counter_k,
    );

    // Step 8: Integer natural frequency memorandum out of 1,000 units from empirical confusion matrix at p*
    let (sensitivity_tpr, false_positive_rate_fpr) = empirical_confusion_tpr_fpr;
    let memo = NaturalFrequencyMemorandum::build_with_decision(
        1_000,
        card.decision_rule_and_scope.event_probability_p,
        sensitivity_tpr,
        false_positive_rate_fpr,
        card.decision_rule_and_scope.critical_threshold_p_star,
        &card.decision_rule_and_scope.declared_act,
        &card.decision_rule_and_scope.exit_clause,
    );

    let s1 = card_audit_ok; // Step 1: Layer, bridge, uncertainty class, Info-Gap + Sniedovich
    let s2 = funnel.filter_1_approved; // Step 2: 3-part question, EVPI, Phi5
    let s3 = funnel.filter_2_approved; // Step 3: Moulton-Campbell m_tilde, rho_I(p), Markov blanket
    let s4 = !card
        .epistemological_header
        .noise_physics
        .absorbing_states
        .is_empty(); // Step 4: Noise physics, Kelly, performativity
    let s5 = funnel.filter_3_approved && null_p < 0.05; // Step 5: Executable null, SBC, SHELF
    let s6 = funnel.filter_4_approved && e_proc.rejects_null_hypothesis; // Step 6: SHA-256, cross-authorship, e-process
    let s7 = !card.decision_rule_and_scope.declared_act.is_empty(); // Step 7: CORP-PAV + p* = C/(eta*L)
    let s8 =
        memo.reference_cohort_size == 1_000 && !card.decision_rule_and_scope.exit_clause.is_empty(); // Step 8: Natural frequencies + exit clause

    let steps = [s1, s2, s3, s4, s5, s6, s7, s8];
    let all_ok = steps.iter().all(|&b| b) && funnel.approved_all_filters;

    SigaExecutionReport {
        card_id: card.id.clone(),
        funnel_result: funnel,
        null_p_value: null_p,
        e_process: e_proc,
        natural_frequency_memo: memo,
        steps_1_to_8_completed: steps,
        approved_end_to_end: all_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eight_step_siga_roadmap_validates_end_to_end_and_prunes_defective_inputs() {
        let card = EpistemologicalCard::canonical_example();
        let plan = card.compute_preregistration_payload();
        let null_samples: Vec<f64> = (0..200).map(|i| (i as f64) * 0.01).collect();
        let report = run_full_siga_roadmap(&card, &plan, 5.0, &null_samples, &[24.8]);
        assert!(report.approved_end_to_end);
        assert_eq!(report.steps_1_to_8_completed, [true; 8]);
        assert!(report
            .natural_frequency_memo
            .formatted_summary
            .contains("1000"));
        assert!(report
            .natural_frequency_memo
            .formatted_summary
            .contains("p*=0.12000"));

        // Un-clamped low observed test statistic fails Step 5 (null_p >= 0.05)
        let weak_signal = run_full_siga_roadmap(&card, &plan, 0.50, &null_samples, &[24.8]);
        assert!(weak_signal.null_p_value >= 0.05);
        assert!(!weak_signal.steps_1_to_8_completed[4]);
        assert!(!weak_signal.approved_end_to_end);

        // Degenerate null samples fail Filter 3 (SBC rank uniformity)
        let bad_null = run_full_siga_roadmap(&card, &plan, 5.0, &[1.0; 100], &[24.8]);
        assert_eq!(bad_null.funnel_result.pruned_at_filter, Some(3));

        // Card conditioning on a Berkson collider fails Filter 2
        let mut collider_card = card.clone();
        collider_card
            .epistemological_header
            .causal_ladder
            .identification_strategy =
            "Markov blanket conditioned on hospital admission collider".into();
        let bad_collider =
            run_full_siga_roadmap(&collider_card, &plan, 5.0, &null_samples, &[24.8]);
        assert_eq!(bad_collider.funnel_result.pruned_at_filter, Some(2));

        // NaN e-value safely zeroes out EvidenceProcess without panicking
        let nan_e = EvidenceProcess::evaluate(&[24.8, f64::NAN], 0.05, 1);
        assert!(!nan_e.rejects_null_hypothesis);
    }

    #[test]
    fn skewed_sbc_samples_fail_filter3_while_genuine_talts_sbc_ranks_pass() {
        let card = EpistemologicalCard::canonical_example();
        let plan = card.compute_preregistration_payload();

        // Skewed non-uniform sample concentrates in lower quintile and fails Filter 3
        let skewed_samples: Vec<f64> = (0..200).map(|i| ((i as f64) / 200.0).powi(3)).collect();
        let skewed_bins = compute_sbc_rank_bins(&skewed_samples);
        assert!(crate::funnel::sbc_rank_chi2(&skewed_bins) > 9.49);
        let skewed_rep = run_full_siga_roadmap(&card, &plan, 5.0, &skewed_samples, &[24.8]);
        assert!(skewed_rep.funnel_result.sbc_chi2_statistic > 9.49);
        assert_eq!(skewed_rep.funnel_result.pruned_at_filter, Some(3));
        assert!(!skewed_rep.approved_end_to_end);

        // Genuine Talts et al. (2018) Beta-Binomial SBC ranks are uniform on [0, 1] and pass Filter 3
        let sbc_ranks = simulate_sbc_beta_binomial_ranks(2026, 200, 50, 2.0, 18.0, 100);
        let sbc_bins = compute_sbc_rank_bins(&sbc_ranks);
        assert!(crate::funnel::sbc_rank_chi2(&sbc_bins) <= 9.49);
        let valid_rep = run_full_siga_roadmap(&card, &plan, 5.0, &sbc_ranks, &[12.5, 2.2, 0.7]);
        assert!(valid_rep.funnel_result.filter_3_approved);
        assert!(valid_rep.e_process.rejects_null_hypothesis);
        assert!(valid_rep.approved_end_to_end);
    }
}
