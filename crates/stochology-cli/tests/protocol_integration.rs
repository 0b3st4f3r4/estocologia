use std::path::PathBuf;
use std::process::Command;

use stochology_cli::{
    compute_unclamped_cluster_permutation_null, load_cards_from_disk,
    verify_card_against_empirical_data,
};
use stochology_core::card::EpistemologicalCard;
use stochology_core::layers::{ManskiBounds, PerformativeSystem};
use stochology_core::rng::{OnlineMoments, SplitMix64};
use stochology_core::sha256::hmac_sha256_hex;
use stochology_core::uncertainty::KennedyOHaganDiscrepancy;
use stochology_governance::clock::{query_with_bitemporal_clock, BitemporalRecord};
use stochology_governance::drift::{AdaptiveConformalInference, MurphyCorpCircuitBreaker};
use stochology_governance::vault::{TemporalVault, VaultError, VaultState};
use stochology_lenses::decision::CorpPavDecomposition;
use stochology_lenses::information::{blahut_arimoto_capacity, DpiMarkovChain, SdpiHierarchy};
use stochology_lenses::invariance::{
    chow_invariance_f_stat, cinelli_hazlett_robustness_value, CausalEnvironment,
};
use stochology_lenses::tail::{dedh_moment_tail_index, gpd_pot_fit};
use stochology_lenses::time::empirical_kelly_optimal_fraction;
use stochology_siga::funnel::fit_shelf_beta_prior;
use stochology_siga::roadmap::EvidenceProcess;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn yaml_cards_on_disk_roundtrip_and_pass_empirical_siga_roadmap() {
    let root = workspace_root();
    let cards = load_cards_from_disk(&root).expect("load canonical YAML cards from disk");
    assert_eq!(cards.len(), 3);

    for (rel_path, card) in &cards {
        let roundtrip = EpistemologicalCard::from_yaml(&card.to_yaml())
            .unwrap_or_else(|e| panic!("failed YAML roundtrip for {rel_path}: {e}"));
        assert_eq!(&roundtrip, card, "YAML roundtrip mismatch for {rel_path}");
        assert!(card.audit().is_ok(), "card.audit() failed for {rel_path}");

        let (_emp, report) = verify_card_against_empirical_data(&root, card)
            .unwrap_or_else(|e| panic!("empirical verification failed for {rel_path}: {e}"));
        assert!(
            report.approved_end_to_end,
            "SIGA roadmap failed for {rel_path}: {:?}",
            report.steps_1_to_8_completed
        );
        assert!(report.funnel_result.approved_all_filters);
        assert!(report.null_p_value <= 0.05);
        assert!(
            report
                .natural_frequency_memo
                .formatted_summary
                .contains("threshold p*=")
                && report
                    .natural_frequency_memo
                    .formatted_summary
                    .contains("Exit clause:")
        );
    }

    // Tampering with declared empirical metrics must be rejected by verify_card_against_empirical_data
    let mut tampered = cards[0].1.clone();
    tampered.decision_rule_and_scope.event_probability_p = 0.25;
    assert!(
        verify_card_against_empirical_data(&root, &tampered).is_err(),
        "tampered event_probability_p must be rejected against CSV"
    );
}

#[test]
fn unclamped_cluster_permutation_null_does_not_fabricate_significance_on_noise() {
    let mut rng = SplitMix64::new(2026);
    let mut cluster_forecasts = Vec::new();
    let mut cluster_outcomes = Vec::new();
    for _ in 0..50 {
        let mut cf = Vec::with_capacity(10);
        let mut co = Vec::with_capacity(10);
        for _ in 0..10 {
            cf.push(rng.next_f64());
            co.push(if rng.next_f64() < 0.10 { 1.0 } else { 0.0 });
        }
        cluster_forecasts.push(cf);
        cluster_outcomes.push(co);
    }
    let (obs_stat, null_samples) = compute_unclamped_cluster_permutation_null(
        &cluster_forecasts,
        &cluster_outcomes,
        2026,
        200,
    );
    assert_eq!(null_samples.len(), 200);
    let extreme_count = null_samples.iter().filter(|&&v| v >= obs_stat).count();
    let p_val = (extreme_count as f64 + 1.0) / (null_samples.len() as f64 + 1.0);
    assert!(
        p_val > 0.05,
        "pure noise must not fabricate significance without clamp (obs={obs_stat:.4}, p={p_val:.4})"
    );
}

#[test]
fn cross_crate_governance_pipeline_end_to_end() {
    let records = vec![
        BitemporalRecord {
            row_id: 1,
            t_event: 10,
            t_availability: 12,
            feature_x: 1,
            outcome_y: 0,
            canary_column: 0,
            t_availability_canary: 99,
        },
        BitemporalRecord {
            row_id: 2,
            t_event: 11,
            t_availability: 14,
            feature_x: 1,
            outcome_y: 1,
            canary_column: 1,
            t_availability_canary: 99,
        },
    ];
    let clock = query_with_bitemporal_clock(&records, 15, true);
    assert!(!clock.temporal_intrusion_alarm);

    let mut vault = TemporalVault::seal("LOCK-INT-2026", vec![0.0, 0.0, 1.0, 1.0]);
    let receipt = vault
        .open_once(&[0.1, 0.2, 0.8, 0.9], 0.20)
        .expect("first opening succeeds");
    assert_eq!(receipt.final_state, VaultState::OpenedApproved);
    assert!(vault.open_once(&[0.1, 0.2, 0.8, 0.9], 0.20).is_err());

    let breaker = MurphyCorpCircuitBreaker::monitor(
        &[0.1, 0.2, 0.3, 0.7, 0.8, 0.9],
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        0.05,
        0.05,
        false,
        false,
        false,
    );
    assert!(!breaker.circuit_breaker_tripped);
}

#[test]
fn property_corp_murphy_identity_holds_across_seeds() {
    for seed in 2026..2046 {
        let mut rng = SplitMix64::new(seed);
        let n = 60;
        let mut forecasts = Vec::with_capacity(n);
        let mut outcomes = Vec::with_capacity(n);
        for i in 0..n {
            // Include occasional exact ties to exercise tie-pooling
            let p = if i % 7 == 0 {
                0.5
            } else {
                (rng.next_f64() * 20.0).round() / 20.0
            };
            let y = if rng.next_f64() < p { 1.0 } else { 0.0 };
            forecasts.push(p);
            outcomes.push(y);
        }
        let corp = CorpPavDecomposition::compute(&forecasts, &outcomes);
        assert!(
            corp.identity_error < 1e-10,
            "CORP identity gap exceeded tolerance for seed {seed}"
        );
        assert!(corp.rel_miscalibration >= -1e-12);
        assert!(corp.res_resolution >= -1e-12);
    }
}

#[test]
fn property_sdpi_and_dpi_contraction_hierarchy_holds_across_seeds() {
    for seed in 2026..2041 {
        let mut rng = SplitMix64::new(seed);
        let rows = 3;
        let cols = 3;
        let mut channel = vec![vec![0.0; cols]; rows];
        for r in &mut channel {
            let mut sum = 0.0;
            for c in r.iter_mut() {
                *c = 0.05 + rng.next_f64();
                sum += *c;
            }
            for c in r.iter_mut() {
                *c /= sum;
            }
        }
        let prior = vec![1.0 / 3.0; rows];
        let sdpi = SdpiHierarchy::compute(&prior, &channel);
        assert!(
            sdpi.hierarchy_valid,
            "SDPI hierarchy violated for seed {seed}: chi2={}, kl={}, tv={}",
            sdpi.eta_chi2, sdpi.eta_kl, sdpi.eta_tv
        );
        let dpi = DpiMarkovChain::compute(&prior, &channel, &channel, &channel);
        assert!(dpi.dpi_satisfied);
    }
}

#[test]
fn cli_binaries_handle_help_and_defects_correctly() {
    let gates_bin = env!("CARGO_BIN_EXE_gates");
    let labs_bin = env!("CARGO_BIN_EXE_labs");
    let root = workspace_root();

    let gates_help = Command::new(gates_bin)
        .arg("--help")
        .current_dir(&root)
        .output()
        .expect("run gates --help");
    assert_eq!(gates_help.status.code(), Some(0));

    let gates_missing_defect = Command::new(gates_bin)
        .arg("--defect")
        .current_dir(&root)
        .output()
        .expect("run gates --defect");
    assert_eq!(gates_missing_defect.status.code(), Some(2));

    let gates_unknown_flag = Command::new(gates_bin)
        .arg("--unknown-flag")
        .current_dir(&root)
        .output()
        .expect("run gates --unknown-flag");
    assert_eq!(gates_unknown_flag.status.code(), Some(2));

    let gates_positive = Command::new(gates_bin)
        .current_dir(&root)
        .output()
        .expect("run gates positive");
    assert_eq!(gates_positive.status.code(), Some(0));

    for defect in [
        "cromwell",
        "phi5",
        "moulton",
        "performativity",
        "sniedovich",
        "clock",
        "vault",
        "concept_drift",
        "chouldechova",
        "machine",
    ] {
        let out = Command::new(gates_bin)
            .args(["--defect", defect])
            .current_dir(&root)
            .output()
            .unwrap_or_else(|e| panic!("failed to run gates --defect {defect}: {e}"));
        assert_eq!(
            out.status.code(),
            Some(1),
            "expected exit code 1 for --defect {defect}, got {:?}",
            out.status.code()
        );
    }

    let labs_help = Command::new(labs_bin)
        .arg("--help")
        .current_dir(&root)
        .output()
        .expect("run labs --help");
    assert_eq!(labs_help.status.code(), Some(0));

    let labs_unknown = Command::new(labs_bin)
        .arg("--invalid-lab")
        .current_dir(&root)
        .output()
        .expect("run labs --invalid-lab");
    assert_eq!(labs_unknown.status.code(), Some(2));
}

#[test]
fn cross_crate_enriched_epistemological_primitives_end_to_end() {
    let mut rng = SplitMix64::new(2026);
    let mut moments = OnlineMoments::new();
    let mut student_samples = Vec::with_capacity(600);
    for _ in 0..600 {
        let x = rng.student_t(3.5);
        moments.push(x);
        student_samples.push(x.abs() + 1.0);
    }
    assert!(moments.excess_kurtosis() > 0.5);
    assert!(moments.jarque_bera_stat() > 15.0);

    let tag = hmac_sha256_hex(b"Jefe", b"what do ya want for nothing?");
    assert_eq!(
        tag,
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    );

    let (lo, hi, q) =
        ManskiBounds::lee_trimming_bounds(&[2.0, 3.0, 4.0, 5.0, 6.0], 5, &[1.0, 2.0, 3.0, 4.0], 5);
    assert!((q - 0.20).abs() < 1e-12 && lo <= hi);

    let perf = PerformativeSystem::new(1.0, 0.20, 0.50, 1.0, 1.50);
    let perfgd = perf.perfgd_trajectory(0.0, 0.5, 40);
    let theta_po = perf.diagnose().theta_po;
    assert!((perfgd.last().copied().unwrap() - theta_po).abs() < 1e-4);

    let (koh, lml, _, _) = KennedyOHaganDiscrepancy::calibrate_empirical_bayes(
        &[0.0, 1.0, 2.0, 3.0],
        &[0.10, 0.25, 0.20, 0.05],
        0.02,
        5.0,
        &[0.5, 1.5, 3.0],
        &[0.1, 0.3, 1.0],
    );
    assert!(lml.is_finite() && koh.theta_anchored_with_gp.is_finite());

    let bsc = vec![vec![0.85, 0.15], vec![0.15, 0.85]];
    let (cap, _) = blahut_arimoto_capacity(&bsc, 50, 1e-10);
    assert!((cap - 0.2704).abs() < 1e-3);

    let returns = [0.08, 0.07, 0.06, 0.09, -0.32, 0.07, 0.08, 0.06];
    let (f_emp, g_emp, f_merton) = empirical_kelly_optimal_fraction(&returns);
    assert!(f_emp > 0.0 && f_emp < f_merton && g_emp > 0.0);

    let xi_dedh = dedh_moment_tail_index(&student_samples, 40);
    let (xi_gpd, sigma_gpd, es99_gpd) = gpd_pot_fit(&student_samples, 0.85);
    assert!(xi_dedh.is_finite() && xi_gpd.is_finite() && sigma_gpd > 0.0 && es99_gpd > 1.0);

    let env1 = CausalEnvironment {
        name: "E1".to_string(),
        x: vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]],
        y: vec![2.0, 4.1, 5.9, 8.0],
    };
    let env2 = CausalEnvironment {
        name: "E2".to_string(),
        x: vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]],
        y: vec![2.1, 3.9, 6.0, 8.1],
    };
    assert!(chow_invariance_f_stat(&[env1, env2], &[0]) < 1.0);
    let rv = cinelli_hazlett_robustness_value(4.5, 100, 1.0);
    assert!(rv > 0.30 && rv < 1.0);

    let (alpha_b, beta_b, rmse_b) = fit_shelf_beta_prior(0.02, 0.05, 0.10);
    assert!(alpha_b > 0.2 && beta_b > alpha_b && rmse_b < 0.05);

    let grapa = EvidenceProcess::from_grapa_betting(&[0.4; 25], 0.05, 1);
    assert!(grapa.rejects_null_hypothesis);

    let mut vault = TemporalVault::seal("LOCK-TAMPER-TEST", vec![0.0, 1.0]);
    vault.sha256_seal =
        "0000000000000000000000000000000000000000000000000000000000000000".to_string();
    assert_eq!(
        vault.open_once(&[0.1, 0.9], 0.25),
        Err(VaultError::SealTampered)
    );

    let cal: Vec<f64> = (0..100).map(|i| i as f64 / 50.0).collect();
    let stream: Vec<f64> = (0..100).map(|i| 1.40 + (i % 20) as f64 * 0.025).collect();
    let aci = AdaptiveConformalInference::evaluate_stream(&cal, &stream, 0.10, 0.02);
    assert!(aci.empirical_coverage_aci > aci.empirical_coverage_static);
}
