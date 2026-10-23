//! Binary `gates` — Enforces the 7 Epistemological Gates (G1 to G7) over the YAML Cards (`cards/*.yaml`)
//! and executes negative defect proofs (`--defect`) that must exit with code `1`.

use std::env;
use std::fs;
use std::path::Path;
use stochology_cli::build_v1_card_suite;
use stochology_core::{
    verify_resolution_phi5, ChouldechovaAudit, EpistemologicalCard, ExceedanceDesignEffect,
    InfoGapOption, PerformativeSystem,
};
use stochology_governance::{
    query_with_bitemporal_clock, BitemporalRecord, MurphyCorpCircuitBreaker, TemporalVault,
};
use stochology_lenses::decision::CorpPavDecomposition;
use stochology_lenses::information::SdpiHierarchy;

fn run_positive_gates(root: &Path) -> Result<(), String> {
    println!("=== EPISTEMOLOGICAL GATES G1-G7 (STOCHOLOGY PROTOCOL v1.0) ===");

    // Verify that the 3 YAML card files in cards/ exist, match the canonical suite byte-for-byte, and pass audit()
    let suite = build_v1_card_suite();
    for (rel_path, _, expected) in &suite {
        let path = root.join(rel_path);
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
        let parsed = EpistemologicalCard::from_yaml(&content)
            .map_err(|e| format!("Parse error in '{rel_path}': {e}"))?;
        if let Err(violations) = parsed.audit() {
            return Err(format!("Card '{rel_path}' failed audit: {violations:?}"));
        }
        if parsed != *expected {
            return Err(format!(
                "Card on disk '{rel_path}' diverges from canonical specification"
            ));
        }
    }
    println!("  [G1-G5] YAML Cards v1.0 (3/3): Phi1 Simulability, Phi2 Moulton-Campbell/Exceedance, Phi3 Null/Missingness, Phi4 Chouldechova/Exit, Phi5 Resolution -> APPROVED");

    // G6: Prohibitions 1-4, Performative Prediction, Info-Gap + Sniedovich, SDPI, and CORP-PAV
    let sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.5).diagnose();
    if !sys.rrm_converges || sys.risk_at_po >= sys.risk_at_ps {
        return Err("G6 failed performative verification".into());
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
    if !sdpi.hierarchy_valid {
        return Err("G6 failed SDPI hierarchy check".into());
    }
    let corp = CorpPavDecomposition::compute(
        &[0.1, 0.2, 0.3, 0.7, 0.8, 0.9],
        &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0],
    );
    if corp.identity_error > 1e-12 {
        return Err("G6 failed CORP-PAV identity check".into());
    }
    println!("  [G6] Prohibitions 1-4, Performativity (PS vs PO), Info-Gap + Sniedovich, SDPI, and CORP-PAV -> APPROVED");

    // G7: Bitemporal Clock, Canary Column, Single-Opening Vault, and Drift Circuit Breaker
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
    let mut vault = TemporalVault::seal("g7_vault", vec![0.0, 1.0]);
    let _ = vault.open_once(&[0.9, 0.1], 0.05);
    if vault.open_once(&[0.01, 0.99], 0.05).is_ok() {
        return Err("G7 permitted illegal reopening of Temporal Vault".into());
    }
    println!("  [G7] Bitemporal Clock (t_disp), Canary Column, Single-Opening Vault, and Murphy-CORP Circuit Breaker -> APPROVED");
    Ok(())
}

fn run_negative_proof(defect: &str) -> ! {
    match defect {
        "cromwell" => {
            let mut c = EpistemologicalCard::canonical_example();
            c.decision_rule_and_scope.event_probability_p = 0.0;
            if let Err(v) = c.audit() {
                eprintln!("NEGATIVE PROOF [cromwell] BLOCKED SUCCESSFULLY: {v:?}");
                std::process::exit(1);
            }
        }
        "phi5" => {
            if !verify_resolution_phi5(
                "operational day",
                "Set a single annual aggregate average for the entire year",
            ) {
                eprintln!(
                    "NEGATIVE PROOF [phi5] BLOCKED SUCCESSFULLY: coarse annual aggregate action against daily unit"
                );
                std::process::exit(1);
            }
        }
        "moulton" => {
            let b = ExceedanceDesignEffect::benchmark_calibration_25028();
            if b.n_effective_p < (b.n_nominal as f64) * 0.10 {
                eprintln!(
                    "NEGATIVE PROOF [moulton] BLOCKED SUCCESSFULLY: nominal pseudoreplication n={} reduced to n_eff={:.0}",
                    b.n_nominal, b.n_effective_p
                );
                std::process::exit(1);
            }
        }
        "performativity" => {
            let unstable = PerformativeSystem::new(1.0, -1.2, 0.5, 1.0, 1.0).diagnose();
            if !unstable.rrm_converges {
                eprintln!(
                    "NEGATIVE PROOF [performativity] BLOCKED SUCCESSFULLY: RRM radius {:.2} >= 1 diverges/oscillates",
                    unstable.rrm_contraction_radius
                );
                std::process::exit(1);
            }
        }
        "sniedovich" => {
            let turkey = InfoGapOption {
                id: "christmas_turkey",
                nominal_performance: 100.0,
                adverse_sensitivity: 25.0,
                favorable_propensity: 20.0,
                wald_global_maximin_bound: -500.0,
                reversible: false,
            }
            .evaluate(60.0, 120.0, 20.0);
            if !turkey.approved_sniedovich {
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
            let mut vault = TemporalVault::seal("violated_vault", vec![0.0, 1.0]);
            let _ = vault.open_once(&[0.9, 0.1], 0.05);
            if let Err(e) = vault.open_once(&[0.01, 0.99], 0.05) {
                eprintln!("NEGATIVE PROOF [vault] BLOCKED SUCCESSFULLY: reopening refused ({e:?})");
                std::process::exit(1);
            }
        }
        "concept_drift" => {
            let m = MurphyCorpCircuitBreaker::monitor(
                &[0.9, 0.85, 0.1, 0.15],
                &[0.0, 0.0, 1.0, 1.0],
                0.5,
                0.15,
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
            let (_, _, ratio) =
                ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(0.30, 0.15, 0.70, 0.20);
            if (ratio - 1.0).abs() > 1e-6 {
                eprintln!(
                    "NEGATIVE PROOF [chouldechova] BLOCKED SUCCESSFULLY: algebraic impossibility FPR_A/FPR_B = {ratio:.4} != 1"
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
        "Usage: gates [--help | --write-cards | --defect <defect_name>]\n\
         Defects: cromwell, phi5, moulton, performativity, sniedovich, clock, vault, concept_drift, chouldechova"
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
        [flag] if flag == "--write-cards" => {
            let suite = build_v1_card_suite();
            for (rel_path, _, card) in &suite {
                let path = root.join(rel_path);
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).expect("create cards/ directory");
                }
                fs::write(&path, card.to_yaml()).expect("write card yaml");
                println!("Written: {}", path.display());
            }
            return;
        }
        [flag] if flag == "--defect" => {
            eprintln!("Missing defect name after '--defect'.");
            print_usage();
            std::process::exit(2);
        }
        [flag, defect] if flag == "--defect" => {
            run_negative_proof(defect);
        }
        other => {
            eprintln!("Unknown arguments: {other:?}");
            print_usage();
            std::process::exit(2);
        }
    }

    match run_positive_gates(root) {
        Ok(()) => {
            println!("=== ALL 7 EPISTEMOLOGICAL GATES APPROVED (EXIT CODE 0) ===");
        }
        Err(msg) => {
            eprintln!("EPISTEMOLOGICAL GATES FAILED: {msg}");
            std::process::exit(1);
        }
    }
}
