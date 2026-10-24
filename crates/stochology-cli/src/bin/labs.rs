//! Binary `labs` — Executes the 7 methodological and empirical laboratories of the Stochology Protocol v1.0
//! over all 13 real datasets (`data/*.csv`) and the 3 canonical Epistemological Cards (`cards/*.yaml`).

use std::env;
use std::path::Path;
use stochology_cli::{
    load_atlantic_storms_decade_environments, load_atlantic_storms_forecasts,
    load_bank_call_clusters, load_cards_from_disk, load_danish_fire, load_djia_returns,
    load_lalonde_nsw_cps, load_nyc_flights_2013_2023, load_nyc_flights_cascade_2013_2023,
    load_oot_ood_flight_reserves, load_toronto_arrests_prevalences, load_vic_elec_demand,
    verify_card_against_empirical_data,
};
use stochology_core::{
    apply_cromwell_floor, mean, sample_std, ChouldechovaAudit, ExceedanceDesignEffect,
    InfoGapOption, KennedyOHaganDiscrepancy, ManskiBounds, MengAudit, NonlinearSummary,
    OperationalWeightingGap, PerformativeSystem, Rng, DEFAULT_SEED,
};
use stochology_governance::{
    estimate_label_shift_four_methods, fit_calibrated_logistic_1d, fit_covariate_drift_extra_wcp,
    query_with_bitemporal_clock, BitemporalRecord, MurphyCorpCircuitBreaker, TemporalVault,
};
use stochology_lenses::{
    amplitude_constrained_mi_bound, fano_error_floor, hidden_confounding_threshold_lambda_star,
    run_icp, run_nonlinear_icp_nile, AlmostPerfectMachine, AuthenticModelForm, CausalEnvironment,
    CorpPavDecomposition, DpiMarkovChain, ExactCoinCalibrationBenchmark, HillTailDiagnostic,
    MixingTimeAudit, SdpiHierarchy, TimeKellyDiagnostic,
};

/// Lab 1: Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$), Meng, Manski, Jensen, and Performativity.
fn lab_layers(root: &Path) {
    println!("=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===");
    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
    let mut pop_y = Vec::with_capacity(nsw_c.len() + cps_c.len());
    let mut rec = Vec::with_capacity(nsw_c.len() + cps_c.len());
    for &y in &cps_c {
        pop_y.push(y);
        rec.push(true);
    }
    for &y in &nsw_c {
        pop_y.push(y);
        rec.push(false);
    }
    let meng = MengAudit::from_population(&rec, &pop_y);
    let manski = ManskiBounds::compute(&nsw_t, nsw_t.len() + 30, 0.0, 60_000.0);
    println!(
        "  [Bridge L1->L2] Meng (CPS n={} vs NSW n={}): rho_R_Y={:+.4}, selection_err={:+.2}, n_eff_Meng={:.2} | Manski MNAR=[{:.1}, {:.1}]",
        cps_c.len(),
        nsw_c.len(),
        meng.rho_r_y,
        meng.estimation_error,
        meng.n_eff_meng,
        manski.lower_bound,
        manski.upper_bound
    );

    let clusters = load_bank_call_clusters(root);
    let daily_totals: Vec<f64> = clusters.iter().map(|c| c.iter().sum::<f64>()).collect();
    let split_idx = daily_totals.len() * 4 / 5;
    // Convex congestion cost f(x) = x^2 evaluated with out-of-sample CRPS on held-out days
    let jensen = NonlinearSummary::evaluate_out_of_sample(
        &daily_totals[..split_idx],
        &daily_totals[split_idx..],
        |x| x * x,
        2.0,
    );
    println!(
        "  [Bridge L2->L3] Jensen Gap (bank_calls, {} days): f(E[X])={:.2}, E[f(X)]={:.2}, Delta_J={:+.2}, median={:.2}, q05={:.2}, q95={:.2}, OOS_CRPS={:.2}",
        daily_totals.len(),
        jensen.f_at_mean,
        jensen.expected_f,
        jensen.jensen_gap,
        jensen.median_f,
        jensen.q05_f,
        jensen.q95_f,
        jensen.empirical_crps
    );

    let perf = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.5).diagnose();
    let osc = PerformativeSystem::simulate_rrm_oscillation(5);
    println!(
        "  [Bridge L3->L1] Performative Prediction: theta_SL={:.4}, theta_PS={:.4}, theta_PO={:.4}, PR(PS)={:.4} vs PR(PO)={:.4}, r_RRM={:.3} (converges={}), oscillation_violated={:?}",
        perf.theta_sl,
        perf.theta_ps,
        perf.theta_po,
        perf.risk_at_ps,
        perf.risk_at_po,
        perf.rrm_contraction_radius,
        perf.rrm_converges,
        osc
    );
}

/// Lab 2: Kennedy-O'Hagan Gaussian Process Structural Discrepancy and Info-Gap + Sniedovich Safeguard.
fn lab_uncertainty() {
    println!("=== LAB 2: KENNEDY-O'HAGAN GP DISCREPANCY AND INFO-GAP + SNIEDOVICH ===");
    let koh = KennedyOHaganDiscrepancy::canonical_benchmark();
    println!(
        "  [Kennedy-O'Hagan GP] theta_true={:.3} | without_GP={:.4} (bias={:.4}, extrap_err={:.4}) | with_GP={:.4} (bias={:.4}, extrap_err={:.4})",
        koh.theta_true,
        koh.theta_naive_without_gp,
        koh.pseudo_convergence_bias_without_gp,
        koh.extrapolation_error_without_gp,
        koh.theta_anchored_with_gp,
        koh.parameter_error_with_gp,
        koh.extrapolation_error_with_gp
    );

    let fragile = InfoGapOption {
        id: "fragile_nominal_turkey",
        nominal_performance: 100.0,
        adverse_sensitivity: 25.0,
        favorable_propensity: 20.0,
        wald_global_maximin_bound: -450.0,
        reversible: false,
    }
    .evaluate(60.0, 120.0, 20.0);

    let robust = InfoGapOption {
        id: "robust_satisficing_wald",
        nominal_performance: 88.0,
        adverse_sensitivity: 8.0,
        favorable_propensity: 15.0,
        wald_global_maximin_bound: 45.0,
        reversible: true,
    }
    .evaluate(60.0, 120.0, 20.0);

    println!(
        "  [Info-Gap + Sniedovich] {}: alpha_hat={:.2}, zero_ing={:.1}, Wald={:.1}, approved={} | {}: alpha_hat={:.2}, beta_hat={:.2}, Wald={:.1}, approved={}",
        fragile.id,
        fragile.robustness_alpha_hat,
        fragile.zero_ing_at_nominal,
        fragile.wald_global_maximin_bound,
        fragile.approved_sniedovich,
        robust.id,
        robust.robustness_alpha_hat,
        robust.opportuneness_beta_hat,
        robust.wald_global_maximin_bound,
        robust.approved_sniedovich
    );
}

/// Lab 3: The Five Transversal Lenses on Real Datasets.
fn lab_lenses(root: &Path) {
    println!(
        "=== LAB 3: THE FIVE TRANSVERSAL LENSES (SDPI/DPI, TIME/KELLY, HILL, ICP, CORP-PAV) ==="
    );
    // 1. Information Lens
    let sdpi = SdpiHierarchy::compute(&[0.5, 0.5], &[vec![0.85, 0.15], vec![0.15, 0.85]]);
    let dpi = DpiMarkovChain::compute(
        &[0.5, 0.5],
        &[vec![0.90, 0.10], vec![0.10, 0.90]],
        &[vec![0.85, 0.15], vec![0.15, 0.85]],
        &[vec![0.80, 0.20], vec![0.20, 0.80]],
    );
    let i_amp = amplitude_constrained_mi_bound(1.5, 1.0, 0.05);
    let fano = fano_error_floor(4, (4.0_f64).ln(), 0.65 * (4.0_f64).ln());
    println!(
        "  [Lens 1: Information] SDPI: eta_chi2={:.4} <= eta_KL={:.4} <= eta_TV={:.4} (valid={}) | DPI W->X->Y->Z: {:.4} >= {:.4} >= {:.4} (monotonic={}) | I_amp={:.4} | Fano P_e >= {:.4}",
        sdpi.eta_chi2,
        sdpi.eta_kl,
        sdpi.eta_tv,
        sdpi.hierarchy_valid,
        dpi.i_w_x_nats,
        dpi.i_w_y_nats,
        dpi.i_w_z_nats,
        dpi.dpi_satisfied,
        i_amp,
        fano
    );

    // 2. Time Lens (DJIA 1885-2023 + Kelly + Mixing Time)
    let djia_returns = load_djia_returns(root);
    let mu_annual = mean(&djia_returns) * 252.0;
    let sigma_annual = sample_std(&djia_returns) * (252.0_f64).sqrt();
    let kelly = TimeKellyDiagnostic::compute(mu_annual, sigma_annual, 0.54, 0.25, 0.20);
    let mix = MixingTimeAudit::compute(sdpi.eta_kl, 0.002, 4.0, 0.02);
    println!(
        "  [Lens 2: Time] DJIA ({} trading days): g_ensemble={:.4} vs g_time_Ito={:.4} (drain={:.4}) | Kelly f*={:.4} | Mixing steps={}",
        djia_returns.len(),
        kelly.ensemble_rate_mu,
        kelly.time_rate_ito,
        kelly.variance_drain,
        kelly.asymmetric_kelly_fraction,
        mix.mixing_steps
    );

    // 3. Scale & Tail Lens (Danish Fire + Victoria Electricity Demand)
    let fire_losses = load_danish_fire(root);
    let hill = HillTailDiagnostic::audit(&fire_losses, Some(100));
    let vic_demand = load_vic_elec_demand(root);
    let hill_vic = HillTailDiagnostic::audit(&vic_demand, Some(100));
    println!(
        "  [Lens 3: Tail] Danish Fire (n={}): Hill alpha_k={:.4} (+/- {:.4}), regime={:?}, median={:.3}, MAD={:.3}, q99={:.2}, tail_n_eff={:.1} | VicElec (n={}): q99={:.1} MW",
        hill.sample_size_n,
        hill.alpha_hill,
        hill.asymptotic_se_alpha,
        hill.regime,
        hill.median,
        hill.mad,
        hill.quantile_99,
        hill.penalized_tail_n_eff,
        hill_vic.sample_size_n,
        hill_vic.quantile_99
    );

    // 4. Invariance Lens (ICP + Linear OLS Penalty + Nonlinear NILE/IRM over Phi + Authentic Form)
    let mut rng = Rng::new(DEFAULT_SEED);
    let mut envs = Vec::new();
    for (idx, (sc, sp_w)) in [(1.0, 1.2), (2.2, -1.4), (0.7, 2.8)]
        .into_iter()
        .enumerate()
    {
        let mut x = Vec::with_capacity(300);
        let mut y = Vec::with_capacity(300);
        for _ in 0..300 {
            let x0 = rng.normal(0.0, sc);
            let yi = 1.75 * x0 + rng.normal(0.0, 0.12);
            let x1 = sp_w * yi + rng.normal(0.0, 0.35);
            x.push(vec![x0, x1]);
            y.push(yi);
        }
        envs.push(CausalEnvironment {
            name: format!("regime_{idx}"),
            x,
            y,
        });
    }
    let icp = run_icp(&envs, 0.12);
    let storm_envs = load_atlantic_storms_decade_environments(root);
    let nl_icp = run_nonlinear_icp_nile(&storm_envs, 0.15);
    let auth_form = AuthenticModelForm::audit_environments(
        DEFAULT_SEED,
        &storm_envs,
        &[
            "atom_0_barometric_pressure_drop".to_string(),
            "atom_1_regime_shifted_lat_proxy".to_string(),
        ],
        0.15,
        true,
    );
    let lambda_star = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.60);
    let pen_fmt: Vec<String> = icp
        .env_ols_invariance_penalties
        .iter()
        .map(|v| format!("{v:.4}"))
        .collect();
    let nl_fmt: Vec<String> = nl_icp
        .nonlinear_nile_irm_penalties
        .iter()
        .map(|v| format!("{v:.4}"))
        .collect();
    println!(
        "  [Lens 4: Invariance] ICP S*={:?}, env_OLS_penalty={:?}, Storms_nonlinear_NILE_IRM={:?}, Lambda*={:.3} | AuthenticForm(S*={:?}, discarded='{}', A={})",
        icp.accepted_causal_predictors,
        pen_fmt,
        nl_fmt,
        lambda_star,
        auth_form.invariant_core_s_star,
        auth_form.discarded_atoms_named[0].atom_name,
        auth_form.approved_authentic_form
    );

    // 5. Decision Lens (CORP-PAV on Atlantic Storms + Exact Fair-Coin Benchmark)
    let (probs, outcomes) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&probs, &outcomes);
    let coin = ExactCoinCalibrationBenchmark::evaluate(DEFAULT_SEED, 1_000);
    println!(
        "  [Lens 5: Decision] CORP-PAV (Atlantic Storms, n={}): BS={:.5} = REL({:.5}) - RES({:.5}) + UNC0({:.5}) [id_err={:.2e}] | FairCoin(p0=0.50): agree@0.30={:.0}%, REL_over={:.4} vs REL_cal={:.4}, regret@0.70=+{:.2}",
        probs.len(),
        corp.brier_total,
        corp.rel_miscalibration,
        corp.res_resolution,
        corp.unc0_uncertainty,
        corp.identity_error,
        coin.decision_agreement_rate_at_p_star_a * 100.0,
        coin.overconfident_corp.rel_miscalibration,
        coin.calibrated_corp.rel_miscalibration,
        coin.shifted_threshold_excess_regret_per_unit
    );

    // 6. Almost-Perfect Machine M = (T, Phi, theta) (P1-P5)
    let machine = AlmostPerfectMachine::evaluate_canonical(DEFAULT_SEED);
    println!(
        "  [Almost-Perfect Machine M=(T,Phi,theta)] sup_err={:.4} (Barron C_f={:.2}), DPI_gap={:.4} nats, regions={} (ratio={:.1}x), d_eff={:.2}/{}, P1..P5={}",
        machine.sup_error_epsilon,
        machine.barron_norm_c_f,
        machine.dpi_gap_delta_nats,
        machine.composed_linear_regions,
        machine.region_to_parameter_ratio,
        machine.atomic_effective_dimension,
        machine.width_w,
        machine.all_p1_to_p5_approved
    );
}

/// Lab 4: Invariants $\Phi_1$–$\Phi_5$, Moulton-Campbell, Exceedance ICC, and Chouldechova.
fn lab_invariants(root: &Path) {
    println!("=== LAB 4: INVARIANTS PHI1-PHI5, MOULTON-CAMPBELL, EXCEEDANCE, AND CHOULDECHOVA ===");
    let b1 = ExceedanceDesignEffect::benchmark_calibration_25028();
    let b2 = ExceedanceDesignEffect::benchmark_pseudoreplication_500_to_86();
    let amp = OperationalWeightingGap::variance_amplification_benchmark_69x();
    println!(
        "  [Benchmarks Phi2] n={} -> m_bar={:.2}, m_tilde={:.2}, rho_I(0.90)={:.3}, n_eff={:.1} (naive Kish={:.1}) | n={} -> n_eff={:.1} | var_amp={:.1}x",
        b1.n_nominal,
        b1.m_bar,
        b1.m_tilde_moulton,
        b1.rho_exceedance_p,
        b1.n_effective_p,
        b1.n_effective_naive_kish,
        b2.n_nominal,
        b2.n_effective_p,
        amp
    );

    let clusters = load_bank_call_clusters(root);
    let real_deff = ExceedanceDesignEffect::from_clustered_observations(&clusters, 0.90);
    println!(
        "  [Real Data: bank_calls.csv] n={}, clusters={}, m_bar={:.2}, m_tilde={:.2}, rho_I(0.90)={:.4}, DEFF={:.2}, n_eff(0.90)={:.1}",
        real_deff.n_nominal,
        real_deff.num_clusters_c,
        real_deff.m_bar,
        real_deff.m_tilde_moulton,
        real_deff.rho_exceedance_p,
        real_deff.deff_exceedance,
        real_deff.n_effective_p
    );

    let (p_a, p_b) = load_toronto_arrests_prevalences(root);
    let (fpr_a, fpr_b, ratio) =
        ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
    println!(
        "  [Phi4: Chouldechova on toronto_arrests.csv] p_A={:.4} vs p_B={:.4} -> equalizing PPV=0.65 & FNR=0.25 forces FPR_A={:.4} vs FPR_B={:.4} (ratio={:.2}x)",
        p_a, p_b, fpr_a, fpr_b, ratio
    );
}

/// Lab 5: 4-Filter Eliminatory Funnel, 8-Step SIGA Roadmap, and Natural-Frequency Executive Memoranda on the 3 Canonical YAML Cards.
fn lab_siga(root: &Path) {
    println!("=== LAB 5: 4-FILTER ELIMINATORY FUNNEL AND 8-STEP SIGA ROADMAP ===");
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    for (rel_path, card) in cards {
        let (_audit, rep) = verify_card_against_empirical_data(root, &card)
            .expect("verify card against empirical dataset");
        println!(
            "  [SIGA {}] dataset={}, EVPI={:.1}, p_null={:.4} (unclamped), chi2_SBC={:.2}, E_30={:.1}, steps_1_to_8={:?}, approved={}",
            rel_path,
            card.data_and_audit_contract.dataset_path,
            rep.funnel_result.calculated_evpi,
            rep.null_p_value,
            rep.funnel_result.sbc_chi2_statistic,
            rep.e_process.final_e_value,
            rep.steps_1_to_8_completed,
            rep.approved_end_to_end
        );
        for line in rep.natural_frequency_memo.formatted_summary.lines() {
            println!("    {line}");
        }
    }
}

/// Lab 6: Production Governance: Bitemporal Clock, Single-Opening Vault, and 4 Regimes of Drift.
fn lab_governance(root: &Path) {
    println!("=== LAB 6: PRODUCTION GOVERNANCE: BITEMPORAL CLOCK, VAULT, AND 4 DRIFT REGIMES ===");
    let records = [
        BitemporalRecord {
            row_id: 1,
            t_event: 10,
            t_availability: 12,
            feature_x: 0,
            outcome_y: 0,
            canary_column: 0,
            t_availability_canary: 999,
        },
        BitemporalRecord {
            row_id: 2,
            t_event: 11,
            t_availability: 14,
            feature_x: 1,
            outcome_y: 1,
            canary_column: 1,
            t_availability_canary: 999,
        },
    ];
    let on = query_with_bitemporal_clock(&records, 50, true);
    let off = query_with_bitemporal_clock(&records, 50, false);
    println!(
        "  [Bitemporal Clock + Canary] enabled: MI_canary={:.4}, alarm={} | disabled (mutation): MI_canary={:.4}, alarm={}",
        on.canary_mutual_information_nats,
        on.temporal_intrusion_alarm,
        off.canary_mutual_information_nats,
        off.temporal_intrusion_alarm
    );

    let mut vault = TemporalVault::seal("vault_2026_oot", vec![0.0, 1.0, 1.0, 0.0]);
    let v_rep = vault
        .open_once(&[0.05, 0.95, 0.90, 0.05], 0.02)
        .expect("single opening");
    println!(
        "  [Single-Opening Temporal Vault] sha256_seal={}..{}, Brier={:.4}, state={:?}",
        &v_rep.sha256_seal[..8],
        &v_rep.sha256_seal[56..],
        v_rep.evaluated_brier_score,
        v_rep.final_state
    );

    let (x13, y13, x23, _y23) = load_nyc_flights_2013_2023(root);
    let p_base_13 = mean(&y13).clamp(0.01, 0.99);
    let logit_base_13 = (p_base_13 / (1.0 - p_base_13)).ln();
    let scores_13: Vec<f64> = x13
        .iter()
        .zip(y13.iter())
        .map(|(&x, &y)| {
            let p_hat = 1.0 / (1.0 + (-(logit_base_13 + 0.15 * x)).exp());
            (y - p_hat).abs()
        })
        .collect();
    let cov = fit_covariate_drift_extra_wcp(&x13, &x23, &scores_13, None, 0.02, 0.10);
    println!(
        "  [Covariate Drift: NYC Flights 2013->2023] beta_tilt={:+.4}, Z_P={:.4}, ExTRA_WCP_T_threshold={:.4}, Lei_Candes_bound >= {:.3}",
        cov.estimated_beta,
        cov.normalizing_constant_z_p,
        cov.weighted_conformal_threshold,
        cov.lei_candes_coverage_lower_bound
    );

    // Fit a real 1D logistic classifier on 2013 source cascade data and evaluate on 2023 target data
    let (cx13, cy13, cx23, _cy23) = load_nyc_flights_cascade_2013_2023(root);
    let (p_source_13, tpr_s, fpr_s, mu_0_s, mu_1_s, post_target) =
        fit_calibrated_logistic_1d(&cx13, &cy13, &cx23);
    let ls = estimate_label_shift_four_methods(
        p_source_13,
        tpr_s,
        fpr_s,
        &post_target,
        mu_0_s,
        mu_1_s,
        mean(&cx23),
        0.02,
    );
    let cb = MurphyCorpCircuitBreaker::monitor(
        &[0.9, 0.85, 0.15, 0.1],
        &[0.0, 0.0, 1.0, 1.0],
        0.05,
        0.05,
        false,
        false,
        false,
    );
    println!(
        "  [Label Shift 4 Estimators (fit on 2013, TPR={:.3}, FPR={:.3})] BBSE={:.4} (sigma_min={:.3}), MLLS/SLD={:.4}, DFM={:.4} (Delta_min={:.1}), OTTER={:.4} (kappa={:.2}) | Concept Drift Breaker={:?} (tripped={})",
        tpr_s,
        fpr_s,
        ls.target_prior_bbse,
        ls.confusion_matrix_sigma_min,
        ls.target_prior_mlls_sld,
        ls.target_prior_dfm,
        ls.gram_eigenvalue_delta_min,
        ls.target_prior_otter,
        ls.hoffman_constant_kappa,
        cb.diagnosed_regime,
        cb.circuit_breaker_tripped
    );
}

/// Lab 7: Transversal Real-Data Synthesis Across All 13 Empirical Datasets (`data/*.csv`) and 3 Canonical Cards (`cards/*.yaml`).
fn lab_real_data(root: &Path) {
    println!("=== LAB 7: TRANSVERSAL REAL-DATA SYNTHESIS (13 EMPIRICAL DATASETS & 3 CANONICAL CARDS) ===");

    // 1. Public Policy & Observational Selection (nsw_experimental.csv + cps_observational.csv)
    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
    let mut pop_y = Vec::with_capacity(nsw_c.len() + cps_c.len());
    let mut rec = Vec::with_capacity(nsw_c.len() + cps_c.len());
    for &y in &cps_c {
        pop_y.push(y);
        rec.push(true);
    }
    for &y in &nsw_c {
        pop_y.push(y);
        rec.push(false);
    }
    let meng = MengAudit::from_population(&rec, &pop_y);
    let manski = ManskiBounds::compute(&nsw_t, nsw_t.len() + 30, 0.0, 60_000.0);
    println!(
        "  [1/6 Policy: nsw_experimental.csv + cps_observational.csv] Bridge L1->L2 & Phi3: rho_R_Y={:+.4}, n={} -> n_eff={:.2}, Manski=[{:.1}, {:.1}]",
        meng.rho_r_y,
        cps_c.len(),
        meng.n_eff_meng,
        manski.lower_bound,
        manski.upper_bound
    );

    // 2. Banking Operations & Queue Physics (bank_calls.csv + card_02_bank_queue_staffing.yaml)
    let clusters = load_bank_call_clusters(root);
    let daily_totals: Vec<f64> = clusters.iter().map(|c| c.iter().sum::<f64>()).collect();
    let split_idx = daily_totals.len() * 4 / 5;
    let jensen = NonlinearSummary::evaluate_out_of_sample(
        &daily_totals[..split_idx],
        &daily_totals[split_idx..],
        |x| x * x,
        2.0,
    );
    let deff = ExceedanceDesignEffect::from_clustered_observations(&clusters, 0.90);
    println!(
        "  [2/6 Banking: bank_calls.csv -> card_02] Bridge L2->L3 & Phi2/Phi5: n={} (days={}), Delta_J={:+.2e}, OOS_CRPS={:.2e}, DEFF(0.90)={:.2} -> n_eff={:.1}",
        deff.n_nominal,
        deff.num_clusters_c,
        jensen.jensen_gap,
        jensen.empirical_crps,
        deff.deff_exceedance,
        deff.n_effective_p
    );

    // 3. Long-Horizon Financial Non-Ergodicity & Performativity (djia_1885_2023.csv)
    let djia = load_djia_returns(root);
    let mu_ann = mean(&djia) * 252.0;
    let sig_ann = sample_std(&djia) * (252.0_f64).sqrt();
    let kelly = TimeKellyDiagnostic::compute(mu_ann, sig_ann, 0.54, 0.25, 0.20);
    let perf = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.2).diagnose();
    println!(
        "  [3/6 Finance: djia_1885_2023.csv] Lens 2 & Bridge L3->L1: days={}, g_ens={:.4} -> g_time={:.4} (drain={:.4}), Kelly f*={:.4}, theta_PS={:.4} vs theta_PO={:.4} (r={:.2})",
        djia.len(),
        kelly.ensemble_rate_mu,
        kelly.time_rate_ito,
        kelly.variance_drain,
        kelly.asymmetric_kelly_fraction,
        perf.theta_ps,
        perf.theta_po,
        perf.rrm_contraction_radius
    );

    // 4. Heavy Tails & Infinite Variance in Reinsurance and Energy (danish_fire.csv + vic_elec.csv)
    let fire = load_danish_fire(root);
    let hill_fire = HillTailDiagnostic::audit(&fire, Some(100));
    let vic = load_vic_elec_demand(root);
    let hill_vic = HillTailDiagnostic::audit(&vic, Some(100));
    println!(
        "  [4/6 Tails: danish_fire.csv + vic_elec.csv] Lens 3: DanishFire(n={}, alpha={:.4}, {:?}, q99={:.2}M, tail_n_eff={:.1}) | VicElec(n={}, alpha={:.4}, q99={:.1} MW)",
        hill_fire.sample_size_n,
        hill_fire.alpha_hill,
        hill_fire.regime,
        hill_fire.quantile_99,
        hill_fire.penalized_tail_n_eff,
        hill_vic.sample_size_n,
        hill_vic.alpha_hill,
        hill_vic.quantile_99
    );

    // 5. Isotonic Calibration & Algebraic Fairness (atlantic_storms.csv + toronto_arrests.csv + card_01_atlantic_storms_evacuation.yaml)
    let (storm_p, storm_y) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&storm_p, &storm_y);
    let (p_a, p_b) = load_toronto_arrests_prevalences(root);
    let (_, _, fpr_ratio) =
        ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
    println!(
        "  [5/6 Calibration & Fairness: atlantic_storms.csv + toronto_arrests.csv -> card_01] Lens 5 & Phi4: Storms BS={:.5} (REL={:.5}, RES={:.5}, err={:.2e}) | Toronto FPR_A/FPR_B={:.2}x",
        corp.brier_total,
        corp.rel_miscalibration,
        corp.res_resolution,
        corp.identity_error,
        fpr_ratio
    );

    // 6. Multi-Horizon & Out-of-Domain Aviation Drift (nyc_flights_2013/2023 + H1 reserves + houston_flights_2011.csv -> card_03)
    let (x13, y13, x23, _y23) = load_nyc_flights_2013_2023(root);
    let (x13_h1, y13_h1, x23_h1, y23_h1, x_hou, _y_hou) = load_oot_ood_flight_reserves(root);
    let p_base_13 = mean(&y13).clamp(0.01, 0.99);
    let logit_base_13 = (p_base_13 / (1.0 - p_base_13)).ln();
    let scores_13: Vec<f64> = x13
        .iter()
        .zip(y13.iter())
        .map(|(&x, &y)| {
            let p_hat = 1.0 / (1.0 + (-(logit_base_13 + 0.15 * x)).exp());
            (y - p_hat).abs()
        })
        .collect();
    let p_base_13_h1 = mean(&y13_h1).clamp(0.01, 0.99);
    let logit_base_13_h1 = (p_base_13_h1 / (1.0 - p_base_13_h1)).ln();
    let scores_13_h1: Vec<f64> = x13_h1
        .iter()
        .zip(y13_h1.iter())
        .map(|(&x, &y)| {
            let p_hat = 1.0 / (1.0 + (-(logit_base_13_h1 + 0.15 * x)).exp());
            (y - p_hat).abs()
        })
        .collect();
    let cov_nyc = fit_covariate_drift_extra_wcp(&x13, &x23, &scores_13, None, 0.02, 0.10);
    let cov_h1 = fit_covariate_drift_extra_wcp(&x13_h1, &x23_h1, &scores_13_h1, None, 0.02, 0.10);
    let cov_hou = fit_covariate_drift_extra_wcp(&x13, &x_hou, &scores_13, None, 0.02, 0.10);

    // Strictly out-of-time forecast calibrated on 2013 H1 source parameters without touching y23_h1
    let probs_h1: Vec<f64> = x23_h1
        .iter()
        .map(|&x| {
            let p_model = 1.0 / (1.0 + (-(logit_base_13_h1 + 0.15 * x)).exp());
            apply_cromwell_floor(p_model, 0.001)
        })
        .collect();
    let mut oot_vault = TemporalVault::seal("nyc_flights_2023_h1_oot", y23_h1);
    let vault_rep = oot_vault
        .open_once(&probs_h1, 0.20)
        .expect("single opening of H1 reserve vault");
    println!(
        "  [6/6 Aviation Drift (5 CSVs): nyc_flights_2013/2023 + H1 reserves + houston_flights_2011.csv -> card_03] Lei-Candes(NYC 13->23 >= {:.3}, H1 13->23 >= {:.3}, NYC->Houston OOD >= {:.3}) | H1 Vault={:?} (Brier={:.4})",
        cov_nyc.lei_candes_coverage_lower_bound,
        cov_h1.lei_candes_coverage_lower_bound,
        cov_hou.lei_candes_coverage_lower_bound,
        vault_rep.final_state,
        vault_rep.evaluated_brier_score
    );

    // 7. End-to-End Epistemological Card Ledger (cards/card_01..03.yaml)
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    let mut cards_approved = 0usize;
    for (_, card) in &cards {
        if let Ok((_, rep)) = verify_card_against_empirical_data(root, card) {
            if rep.approved_end_to_end {
                cards_approved += 1;
            }
        }
    }
    println!(
        "  [Synthesis Ledger] 13/13 real CSV datasets audited | {}/{} canonical YAML cards approved (G1-G8 & SIGA)",
        cards_approved,
        cards.len()
    );
}

fn print_usage() {
    println!(
        "Usage: labs [--help | --all | --layers | --uncertainty | --lenses | --invariants | --siga | --governance | --real-data]"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        eprintln!("Too many arguments: {args:?}");
        print_usage();
        std::process::exit(2);
    }
    let mode = args.first().map(|s| s.as_str()).unwrap_or("--all");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");

    match mode {
        "--help" | "-h" => print_usage(),
        "--layers" => lab_layers(root),
        "--uncertainty" => lab_uncertainty(),
        "--lenses" => lab_lenses(root),
        "--invariants" => lab_invariants(root),
        "--siga" => lab_siga(root),
        "--governance" => lab_governance(root),
        "--real-data" => lab_real_data(root),
        "--all" => {
            lab_layers(root);
            lab_uncertainty();
            lab_lenses(root);
            lab_invariants(root);
            lab_siga(root);
            lab_governance(root);
            lab_real_data(root);
        }
        other => {
            eprintln!("Unknown option: '{other}'.");
            print_usage();
            std::process::exit(2);
        }
    }
}
