//! Binary `gates` — Enforces the 8 Epistemological Gates (G1 to G8) over the YAML Cards (`cards/*.yaml`)
//! and their anchored CSV datasets (`data/*.csv`), and executes negative defect proofs (`--defect`)
//! that must exit with code `1`.

use std::env;
use std::path::Path;
use stochology_cli::{
    evaluate_empirical_card_severe_test, load_atlantic_storms_decade_environments,
    load_atlantic_storms_forecasts, load_bank_call_clusters, load_cards_from_disk,
    load_oot_ood_flight_reserves, load_toronto_arrests_prevalences,
};
use stochology_core::{
    apply_cromwell_floor, mean, verify_resolution_phi5, ChouldechovaAudit, ExceedanceDesignEffect,
    InfoGapOption, PerformativeSystem, DEFAULT_SEED,
};
use stochology_governance::{
    query_with_bitemporal_clock, BitemporalRecord, MurphyCorpCircuitBreaker, TemporalVault,
};
use stochology_lenses::decision::{CorpPavDecomposition, ExactCoinCalibrationBenchmark};
use stochology_lenses::information::{DpiMarkovChain, SdpiHierarchy};
use stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm};
use stochology_lenses::relational::RelationalComplexSystemsSynthesis;

fn run_positive_gates(root: &Path) -> Result<(), String> {
    println!("=== EPISTEMOLOGICAL GATES G1-G8 (STOCHOLOGY PROTOCOL) ===");

    // G1-G5: Load the 3 YAML card files directly from disk (`cards/*.yaml`), verify SHA-256 pre-registration
    // and dataset hashes, recompute all empirical quantities from `data/*.csv`, and run the 8-Step SIGA Roadmap.
    let cards = load_cards_from_disk(root)?;
    for (rel_path, card) in &cards {
        let (_audit, siga_rep) = evaluate_empirical_card_severe_test(root, card)?;
        if !siga_rep.approved_end_to_end {
            return Err(format!(
                "Card '{rel_path}' failed 8-Step SIGA Roadmap: {:?}",
                siga_rep.steps_1_to_8_completed
            ));
        }
    }
    println!(
        "  [G1-G5] YAML Cards ({}/{} from disk, verified against CSVs): Phi1 Simulability, Phi2 Moulton-Campbell/Exceedance, Phi3 Null/Missingness, Phi4 Chouldechova/Exit, Phi5 Resolution -> APPROVED",
        cards.len(),
        cards.len()
    );

    // G6: Prohibitions 1-4, Performative Prediction, Info-Gap + Sniedovich, SDPI + 3-Step DPI Chain, and CORP-PAV on real data
    let perf_model = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.5);
    let sys = perf_model.diagnose();
    let perfgd = perf_model.perfgd_trajectory(0.0, 0.5, 60);
    let theta_perfgd = *perfgd.last().unwrap_or(&0.0);
    if !sys.rrm_converges
        || sys.risk_at_po >= sys.risk_at_ps
        || (theta_perfgd - sys.theta_po).abs() > 1e-5
    {
        return Err("G6 failed performative / PerfGD Stackelberg verification".into());
    }
    let ig = InfoGapOption {
        id: "robust",
        nominal_performance: 88.0,
        adverse_sensitivity: 8.0,
        favorable_propensity: 15.0,
        wald_global_maximin_bound: 40.0,
        reversible: true,
    }
    .evaluate(60.0, 120.0, 20.0);
    if !ig.approved_sniedovich || ig.zero_ing_at_nominal != 0.0 {
        return Err("G6 failed Info-Gap / Sniedovich check".into());
    }
    let sdpi = SdpiHierarchy::compute(&[0.5, 0.5], &[vec![0.85, 0.15], vec![0.15, 0.85]]);
    let dpi_chain = DpiMarkovChain::compute(
        &[0.5, 0.5],
        &[vec![0.90, 0.10], vec![0.10, 0.90]],
        &[vec![0.85, 0.15], vec![0.15, 0.85]],
        &[vec![0.80, 0.20], vec![0.20, 0.80]],
    );
    if !sdpi.hierarchy_valid || !dpi_chain.dpi_satisfied {
        return Err("G6 failed SDPI hierarchy / 3-step DPI Markov chain check".into());
    }
    let (storm_probs, storm_outcomes) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&storm_probs, &storm_outcomes);
    if corp.identity_error > 1e-12 {
        return Err("G6 failed CORP-PAV identity check on atlantic_storms.csv".into());
    }
    println!("  [G6] Prohibitions 1-4, Performativity (PS vs PO), Info-Gap + Sniedovich, SDPI/DPI Chain, and CORP-PAV -> APPROVED");

    // G7: Bitemporal Clock, Canary Column, Single-Opening Vault on real H1 reserve, and Drift Circuit Breaker
    let rec = [BitemporalRecord {
        row_id: 1,
        t_event: 10,
        t_availability: 15,
        feature_x: 1,
        outcome_y: 1,
        canary_column: 1,
        t_availability_canary: 999,
    }];
    let audit_on = query_with_bitemporal_clock(&rec, 50, true);
    let audit_off = query_with_bitemporal_clock(&rec, 50, false);
    if audit_on.temporal_intrusion_alarm || !audit_off.temporal_intrusion_alarm {
        return Err("G7 failed Bitemporal Clock / Canary check".into());
    }
    let (_x13_h1, y13_h1, x23_h1, y23_h1, _, _) = load_oot_ood_flight_reserves(root);
    let p_base_13_h1 = mean(&y13_h1).clamp(0.01, 0.99);
    let logit_base_13_h1 = (p_base_13_h1 / (1.0 - p_base_13_h1)).ln();
    let probs_h1: Vec<f64> = x23_h1
        .iter()
        .map(|&x| {
            let p_model = 1.0 / (1.0 + (-(logit_base_13_h1 + 0.15 * x)).exp());
            apply_cromwell_floor(p_model, 0.001)
        })
        .collect();
    let mut vault = TemporalVault::seal("g7_nyc_flights_2023_h1_vault", y23_h1.clone());
    let receipt = vault
        .open_once(&probs_h1, 0.20)
        .map_err(|e| format!("G7 failed initial Temporal Vault opening: {e:?}"))?;
    if vault.open_once(&probs_h1, 0.20).is_ok() {
        return Err("G7 permitted illegal reopening of Temporal Vault".into());
    }
    let cb = MurphyCorpCircuitBreaker::monitor(
        &storm_probs,
        &storm_outcomes,
        0.02,
        0.05,
        false,
        false,
        false,
    );
    if cb.circuit_breaker_tripped || receipt.evaluated_brier_score <= 0.0 {
        return Err("G7 failed Murphy-CORP circuit breaker check on calibrated storms".into());
    }
    println!("  [G7] Bitemporal Clock (t_disp), Canary Column, Single-Opening Vault, and Murphy-CORP Circuit Breaker -> APPROVED");

    // G8: Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S* + Named Discarded Atoms),
    // Nonlinear NILE/IRM over Phi, and Exact Coin Calibration vs. Decision Benchmark
    let machine = AlmostPerfectMachine::evaluate_canonical(DEFAULT_SEED);
    if !machine.all_p1_to_p5_approved {
        return Err(format!(
            "G8 failed Almost-Perfect Machine P1-P5 check: {:?}",
            (
                machine.p1_universal_density,
                machine.p2_informational_sufficiency,
                machine.p3_fano_minimax_optimality,
                machine.p4_compositional_efficiency,
                machine.p5_atomic_completeness
            )
        ));
    }
    let storm_envs = load_atlantic_storms_decade_environments(root);
    let authentic = AuthenticModelForm::audit_environments(
        DEFAULT_SEED,
        &storm_envs,
        &[
            "atom_0_barometric_pressure_drop".to_string(),
            "atom_1_regime_shifted_lat_proxy".to_string(),
        ],
        0.15,
        true,
    );
    if !authentic.approved_authentic_form
        || authentic.invariant_core_s_star != vec![0]
        || authentic.discarded_atoms_named.is_empty()
    {
        return Err(format!(
            "G8 failed Authentic Model Form check on atlantic_storms.csv: S*={:?}, crit={:?}, id={:?}",
            authentic.invariant_core_s_star,
            authentic.criterion_a,
            authentic.identities_audit
        ));
    }
    let coin_bench = ExactCoinCalibrationBenchmark::evaluate(DEFAULT_SEED, 1_000);
    if !coin_bench.separates_decision_from_probability_calibration {
        return Err("G8 failed Exact Coin Calibration vs. Decision benchmark".into());
    }
    let rel_syn = RelationalComplexSystemsSynthesis::evaluate_canonical(DEFAULT_SEED);
    if !rel_syn.all_six_axes_verified {
        return Err("G8 failed Relational Ontology & Complex Systems 6-axis verification".into());
    }
    println!("  [G8] Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S*), Nonlinear NILE/IRM, Coin Benchmark, and Relational/Complex Systems (Schelling & Daisyworld) -> APPROVED");
    Ok(())
}

fn run_negative_proof(root: &Path, defect: &str) -> ! {
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    match defect {
        "cromwell" => {
            let mut c = cards[0].1.clone();
            c.decision_rule_and_scope.event_probability_p = 0.0;
            if let Err(v) = c.audit() {
                if evaluate_empirical_card_severe_test(root, &c).is_err() {
                    eprintln!("NEGATIVE PROOF [cromwell] BLOCKED SUCCESSFULLY: {v:?}");
                    std::process::exit(1);
                }
            }
        }
        "phi5" => {
            let mut c = cards[1].1.clone();
            c.decision_maker_question.altered_action =
                "Set a single annual aggregate average for the entire year".into();
            if !verify_resolution_phi5(
                &c.epistemological_header.unit_and_margin.independent_unit,
                &c.decision_maker_question.altered_action,
            ) && c.audit().is_err()
                && evaluate_empirical_card_severe_test(root, &c).is_err()
            {
                eprintln!(
                    "NEGATIVE PROOF [phi5] BLOCKED SUCCESSFULLY: coarse annual aggregate action against daily unit"
                );
                std::process::exit(1);
            }
        }
        "moulton" => {
            let mut c = cards[1].1.clone();
            c.epistemological_header
                .unit_and_margin
                .effective_sample_size_n_eff = c
                .epistemological_header
                .unit_and_margin
                .nominal_sample_size_n;
            let bank_clusters = load_bank_call_clusters(root);
            let b = ExceedanceDesignEffect::from_clustered_observations(&bank_clusters, 0.90);
            if b.n_effective_p < (b.n_nominal as f64) * 0.10
                && c.audit().is_err()
                && evaluate_empirical_card_severe_test(root, &c).is_err()
            {
                eprintln!(
                    "NEGATIVE PROOF [moulton] BLOCKED SUCCESSFULLY: nominal pseudoreplication n={} reduced to n_eff={:.0}",
                    b.n_nominal, b.n_effective_p
                );
                std::process::exit(1);
            }
        }
        "performativity" => {
            let mut c = cards[2].1.clone();
            c.epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .rrm_convergence_condition = "eps * beta / gamma = 0.20 * 6.0 / 1.0 = 1.20".into();
            let unstable = PerformativeSystem::new(1.0, -1.2, 0.5, 1.0, 1.0).diagnose();
            if !unstable.rrm_converges && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [performativity] BLOCKED SUCCESSFULLY: RRM radius {:.2} >= 1 diverges/oscillates",
                    unstable.rrm_contraction_radius
                );
                std::process::exit(1);
            }
        }
        "sniedovich" => {
            let mut c = cards[0].1.clone();
            c.uncertainty_budget_and_authorship
                .info_gap_robustness
                .global_maximin_safeguard =
                "Wald global Maximin bound = -999 in No-Man's Land with irreversible collapse"
                    .into();
            let turkey = InfoGapOption {
                id: "christmas_turkey",
                nominal_performance: 100.0,
                adverse_sensitivity: 25.0,
                favorable_propensity: 20.0,
                wald_global_maximin_bound: -500.0,
                reversible: false,
            }
            .evaluate(60.0, 120.0, 20.0);
            if !turkey.approved_sniedovich && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [sniedovich] BLOCKED SUCCESSFULLY: collapse in No-Man's Land (Wald={})",
                    turkey.wald_global_maximin_bound
                );
                std::process::exit(1);
            }
        }
        "clock" => {
            let rec = [BitemporalRecord {
                row_id: 1,
                t_event: 10,
                t_availability: 15,
                feature_x: 1,
                outcome_y: 1,
                canary_column: 1,
                t_availability_canary: 999,
            }];
            let mutated = query_with_bitemporal_clock(&rec, 50, false);
            if mutated.temporal_intrusion_alarm {
                eprintln!(
                    "NEGATIVE PROOF [clock] BLOCKED SUCCESSFULLY: canary triggered temporal intrusion alarm"
                );
                std::process::exit(1);
            }
        }
        "vault" => {
            let (_, _, _, y23_h1, _, _) = load_oot_ood_flight_reserves(root);
            let preds = vec![0.10; y23_h1.len()];
            let mut vault = TemporalVault::seal("violated_h1_vault", y23_h1);
            let _ = vault.open_once(&preds, 0.20);
            if let Err(e) = vault.open_once(&preds, 0.20) {
                eprintln!("NEGATIVE PROOF [vault] BLOCKED SUCCESSFULLY: reopening refused ({e:?})");
                std::process::exit(1);
            }
        }
        "concept_drift" => {
            let (storm_probs, storm_outcomes) = load_atlantic_storms_forecasts(root);
            let inverted_probs: Vec<f64> = storm_probs.iter().map(|&p| 1.0 - p).collect();
            let m = MurphyCorpCircuitBreaker::monitor(
                &inverted_probs,
                &storm_outcomes,
                0.05,
                0.05,
                false,
                false,
                false,
            );
            if m.circuit_breaker_tripped {
                eprintln!(
                    "NEGATIVE PROOF [concept_drift] BLOCKED SUCCESSFULLY: {}",
                    m.recommended_corrective_action
                );
                std::process::exit(1);
            }
        }
        "chouldechova" => {
            let mut c = cards[0].1.clone();
            c.decision_rule_and_scope.who_pays_for_error =
                "Chouldechova trilemma audit: equalizing PPV and FPR with FPR_A/FPR_B = 1.000 equally fair".into();
            let (p_a, p_b) = load_toronto_arrests_prevalences(root);
            let (_, _, ratio) =
                ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
            if (ratio - 1.0).abs() > 1e-6 && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [chouldechova] BLOCKED SUCCESSFULLY: algebraic impossibility FPR_A/FPR_B = {ratio:.4} != 1"
                );
                std::process::exit(1);
            }
        }
        "machine" => {
            let lossy_machine =
                AlmostPerfectMachine::evaluate_with_config(DEFAULT_SEED, 0.35, 2, 6, 0.12, 0.05);
            let storm_envs = load_atlantic_storms_decade_environments(root);
            let silent_drop_form = AuthenticModelForm::audit_environments(
                DEFAULT_SEED,
                &storm_envs,
                &[
                    "atom_0_barometric_pressure_drop".to_string(),
                    "atom_1_regime_shifted_lat_proxy".to_string(),
                ],
                0.15,
                false,
            );
            if !lossy_machine.all_p1_to_p5_approved && !silent_drop_form.approved_authentic_form {
                eprintln!(
                    "NEGATIVE PROOF [machine] BLOCKED SUCCESSFULLY: DPI gap {:.4} > delta=0.05 (P2={}) and silently dropped spurious atoms rejected (findings_closed={})",
                    lossy_machine.dpi_gap_delta_nats,
                    lossy_machine.p2_informational_sufficiency,
                    silent_drop_form.identities_audit.findings_accounting_closed
                );
                std::process::exit(1);
            }
        }
        other => {
            eprintln!("Unknown defect: '{other}'");
            std::process::exit(2);
        }
    }
    eprintln!("CRITICAL ERROR: negative proof '{defect}' did not block!");
    std::process::exit(0);
}

fn print_usage() {
    println!(
        "Usage: gates [--help | --defect <defect_name>]\n\
         Defects: cromwell, phi5, moulton, performativity, sniedovich, clock, vault, concept_drift, chouldechova, machine"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");

    match args.as_slice() {
        [] => {}
        [flag] if flag == "--help" || flag == "-h" => {
            print_usage();
            return;
        }
        [flag] if flag == "--defect" => {
            eprintln!("Missing defect name after '--defect'.");
            print_usage();
            std::process::exit(2);
        }
        [flag, defect] if flag == "--defect" => {
            run_negative_proof(root, defect);
        }
        other => {
            eprintln!("Unknown arguments: {other:?}");
            print_usage();
            std::process::exit(2);
        }
    }

    match run_positive_gates(root) {
        Ok(()) => {
            println!("=== ALL 8 EPISTEMOLOGICAL GATES APPROVED (EXIT CODE 0) ===");
        }
        Err(msg) => {
            eprintln!("EPISTEMOLOGICAL GATES FAILED: {msg}");
            std::process::exit(1);
        }
    }
}
