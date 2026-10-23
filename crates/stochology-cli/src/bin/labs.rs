//! Binary `labs` — Executes the 6 methodological and empirical laboratories of the Stochology Protocol v1.0
//! over real datasets (`data/*.csv`) and the canonical Epistemological Cards (`cards/*.yaml`).

use std::env;
use std::path::Path;
use stochology_cli::{
    build_v1_card_suite, load_atlantic_storms_forecasts, load_bank_call_clusters, load_danish_fire,
    load_djia_returns, load_lalonde_nsw_cps, load_nyc_flights_2013_2023,
    load_toronto_arrests_prevalences, load_vic_elec_demand,
};
use stochology_core::{
    mean, sample_std, ChouldechovaAudit, ExceedanceDesignEffect, InfoGapOption,
    KennedyOHaganDiscrepancy, ManskiBounds, MengAudit, NonlinearSummary, OperationalWeightingGap,
    PerformativeSystem, Rng, DEFAULT_SEED,
};
use stochology_governance::{
    estimate_label_shift_four_methods, fit_covariate_drift_extra_wcp, query_with_bitemporal_clock,
    BitemporalRecord, MurphyCorpCircuitBreaker, TemporalVault,
};
use stochology_lenses::{
    amplitude_constrained_mi_bound, fano_error_floor, hidden_confounding_threshold_lambda_star,
    run_icp, CausalEnvironment, CorpPavDecomposition, HillTailDiagnostic, MixingTimeAudit,
    SdpiHierarchy, TimeKellyDiagnostic,
};
use stochology_siga::run_full_siga_roadmap;

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
        "=== LAB 3: THE FIVE TRANSVERSAL LENSES (SDPI, TIME/KELLY, HILL, ICP/NILE, CORP-PAV) ==="
    );
    // 1. Information Lens
    let sdpi = SdpiHierarchy::compute(&[0.5, 0.5], &[vec![0.85, 0.15], vec![0.15, 0.85]]);
    let i_amp = amplitude_constrained_mi_bound(1.5, 1.0, 0.05);
    let fano = fano_error_floor(4, (4.0_f64).ln(), 0.65 * (4.0_f64).ln());
    println!(
        "  [Lens 1: Information] SDPI: eta_chi2={:.4} <= eta_KL={:.4} <= eta_TV={:.4} (valid={}) | I_amp_bound={:.4} | Fano P_e >= {:.4}",
        sdpi.eta_chi2, sdpi.eta_kl, sdpi.eta_tv, sdpi.hierarchy_valid, i_amp, fano
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

    // 4. Invariance Lens (ICP + NILE + Lambda*)
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
    let lambda_star = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.60);
    let nile_fmt: Vec<String> = icp
        .nile_scores_by_variable
        .iter()
        .map(|v| format!("{v:.4}"))
        .collect();
    println!(
        "  [Lens 4: Invariance] ICP S*={:?}, NILE={:?}, hidden confounding threshold Lambda*={:.3}",
        icp.accepted_causal_predictors, nile_fmt, lambda_star
    );

    // 5. Decision Lens (CORP-PAV on Atlantic Storms)
    let (probs, outcomes) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&probs, &outcomes);
    println!(
        "  [Lens 5: Decision] CORP-PAV (Atlantic Storms, n={}): BS={:.5} = REL({:.5}) - RES({:.5}) + UNC0({:.5}) [id_err={:.2e}]",
        probs.len(),
        corp.brier_total,
        corp.rel_miscalibration,
        corp.res_resolution,
        corp.unc0_uncertainty,
        corp.identity_error
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

/// Lab 5: 4-Filter Eliminatory Funnel and 8-Step SIGA Roadmap on the 3 Canonical YAML Cards.
fn lab_siga() {
    println!("=== LAB 5: 4-FILTER ELIMINATORY FUNNEL AND 8-STEP SIGA ROADMAP ===");
    let suite = build_v1_card_suite();
    let null_samples: Vec<f64> = (0..199).map(|i| (i as f64) * 0.01).collect();
    for (rel_path, plan_text, card) in suite {
        let rep = run_full_siga_roadmap(&card, &plan_text, 4.2, &null_samples, &[24.8]);
        println!(
            "  [SIGA {}] EVPI={:.1}, p_null={:.4}, chi2_SBC={:.2}, E_1={:.1}, steps_1_to_8={:?}, approved={}",
            rel_path,
            rep.funnel_result.calculated_evpi,
            rep.null_p_value,
            rep.funnel_result.sbc_chi2_statistic,
            rep.e_process.final_e_value,
            rep.steps_1_to_8_completed,
            rep.approved_end_to_end
        );
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

    let (x13, _y13, x23, y23) = load_nyc_flights_2013_2023(root);
    let scores_13: Vec<f64> = x13.iter().map(|&x| 0.15 + 0.10 * x.abs()).collect();
    let cov = fit_covariate_drift_extra_wcp(&x13, &x23, &scores_13, None, 0.02, 0.10);
    println!(
        "  [Covariate Drift: NYC Flights 2013->2023] beta_tilt={:+.4}, Z_P={:.4}, ExTRA_WCP_T_threshold={:.4}, Lei_Candes_bound >= {:.3}",
        cov.estimated_beta,
        cov.normalizing_constant_z_p,
        cov.weighted_conformal_threshold,
        cov.lei_candes_coverage_lower_bound
    );

    let post_23: Vec<f64> = x23
        .iter()
        .map(|&x| (1.0 / (1.0 + (-1.2 * x).exp())).clamp(0.01, 0.99))
        .collect();
    let ls = estimate_label_shift_four_methods(
        0.22,
        0.82,
        0.12,
        &post_23,
        -0.30,
        1.10,
        mean(&x23),
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
    let _ = y23;
    println!(
        "  [Label Shift 4 Estimators] BBSE={:.4} (sigma_min={:.3}), MLLS/SLD={:.4}, DFM={:.4} (Delta_min={:.1}), OTTER={:.4} (kappa={:.2}) | Concept Drift Breaker={:?} (tripped={})",
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

fn print_usage() {
    println!(
        "Usage: labs [--help | --all | --layers | --uncertainty | --lenses | --invariants | --siga | --governance]"
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
        "--layers" | "--bridges" => lab_layers(root),
        "--uncertainty" => lab_uncertainty(),
        "--lenses" => lab_lenses(root),
        "--invariants" | "--exceedance" => lab_invariants(root),
        "--siga" => lab_siga(),
        "--governance" | "--production" => lab_governance(root),
        "--all" => {
            lab_layers(root);
            lab_uncertainty();
            lab_lenses(root);
            lab_invariants(root);
            lab_siga();
            lab_governance(root);
        }
        other => {
            eprintln!("Unknown option: '{other}'.");
            print_usage();
            std::process::exit(2);
        }
    }
}
