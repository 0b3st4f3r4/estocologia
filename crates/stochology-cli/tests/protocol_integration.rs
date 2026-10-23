use std::fs;
use std::path::PathBuf;
use std::process::Command;

use stochology_cli::build_v1_card_suite;
use stochology_core::card::EpistemologicalCard;
use stochology_core::rng::SplitMix64;
use stochology_governance::clock::{query_with_bitemporal_clock, BitemporalRecord};
use stochology_governance::drift::MurphyCorpCircuitBreaker;
use stochology_governance::vault::{TemporalVault, VaultState};
use stochology_lenses::decision::CorpPavDecomposition;
use stochology_lenses::information::SdpiHierarchy;
use stochology_siga::roadmap::run_full_siga_roadmap;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn yaml_cards_on_disk_roundtrip_and_pass_siga_roadmap() {
    let root = workspace_root();
    let suite = build_v1_card_suite();
    assert_eq!(suite.len(), 3);

    let mut rng = SplitMix64::new(2026);
    let null_samples: Vec<f64> = (0..256).map(|_| rng.standard_normal()).collect();

    for (rel_path, prereg_text, expected_card) in &suite {
        let full_path = root.join(rel_path);
        let content = fs::read_to_string(&full_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", full_path.display(), e));
        let parsed = EpistemologicalCard::from_yaml(&content)
            .unwrap_or_else(|e| panic!("failed to parse {}: {}", full_path.display(), e));
        assert_eq!(&parsed, expected_card);
        assert!(parsed.audit().is_ok());

        let report = run_full_siga_roadmap(
            &parsed,
            prereg_text,
            3.5,
            &null_samples,
            &[1.8, 2.1, 2.5, 2.2, 1.9, 2.4],
        );
        assert!(report.approved_end_to_end);
        assert!(report.funnel_result.approved_all_filters);
    }
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
fn property_sdpi_contraction_hierarchy_holds_across_seeds() {
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

    let gates_cromwell = Command::new(gates_bin)
        .args(["--defect", "cromwell"])
        .current_dir(&root)
        .output()
        .expect("run gates --defect cromwell");
    assert_eq!(gates_cromwell.status.code(), Some(1));

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
