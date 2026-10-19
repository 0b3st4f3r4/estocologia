//! Deterministic gates CLI.
//!
//! Usage:
//!   cargo run --release --bin gates -- --horizon all --reproduction
//!   cargo run --release --bin gates -- --horizon h3 --defect h3.voos23
//!
//! Frozen data, kept byte-identical on purpose: the card identifiers, the one-line
//! definitions of the registered cards, the "sessao-" session prefix and the data tokens
//! ("dia", "media-historica", "decisor-declarado", "parar-se-ECE>0.10"). They are hashed
//! into the published Merkle root 9716e28a87bec9cf... or matched by keyword lists inside
//! stochology-core, so translating them would move the root and break the Phi5 grain check.

use stochology_core::audit::{Ledger, sha256_hex};
use stochology_core::{AgentA, Card, Layer, OrchestratorM, Rng, UncertaintyType, evaluate};
use stochology_horizons::h1::hill;
use stochology_hypergraph::lenses;
use stochology_methods::drift::{Options as DriftOptions, cusum, stationary_null};
use stochology_methods::fairness::{Group as JusticeGroup, demographic_parity};

struct Config {
    horizon: String,
    reproduction: bool,
    defect: Option<String>,
}

fn config() -> Config {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cfg = Config {
        horizon: "all".to_string(),
        reproduction: false,
        defect: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--horizon" => {
                if i + 1 < args.len() {
                    cfg.horizon = args[i + 1].clone();
                    i += 1;
                }
            }
            "--reproduction" => cfg.reproduction = true,
            "--defect" => {
                if i + 1 < args.len() {
                    cfg.defect = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            other => {
                eprintln!("unknown flag: {other}");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    cfg
}

/// Session key hashed into every ledger event.
///
/// The published Merkle root was sealed with the Portuguese originals, so the "all"
/// horizon keeps the frozen key "todos" while the flag itself reads "all".
fn session_key(horizon: &str) -> &str {
    if horizon == "all" { "todos" } else { horizon }
}

fn card(
    id: &str,
    layer: Layer,
    uncertainty: UncertaintyType,
    definition: &str,
    question: &str,
    action: &str,
) -> Card {
    let mut c = Card::new(id, layer, uncertainty);
    c.definition = definition.to_string();
    c.simulable_artifact = Some(format!("crates/stochology-core#{id}"));
    c.seed = Some(2026);
    // "dia" is matched by the Phi5 grain keyword list in stochology-core.
    c.independent_unit = Some("dia".to_string());
    c.margin = Some(0.51);
    c.null_model = Some("media-historica".to_string());
    c.ecological_scope = true;
    c.who_pays_for_the_error = Some("decisor-declarado".to_string());
    c.exit_clause = Some("parar-se-ECE>0.10".to_string());
    c.decision_maker_question = question.to_string();
    c.package_action = action.to_string();
    c.property_test = AgentA::A1Code.structural_anchor(&c);
    c.handles_missing = AgentA::A2Data.structural_anchor(&c);
    c.declared_drift = AgentA::A3Lifecycle.structural_anchor(&c);
    c.pathologies_covered = AgentA::A4Inference.structural_anchor(&c);
    c.trail_intact = AgentA::A5Governance.structural_anchor(&c);
    c.subgroups_audited = AgentA::A6Fairness.structural_anchor(&c);
    c.elicited_prior = AgentA::A7Domain.structural_anchor(&c);
    c
}

/// The three intermediate cards the panel rejected along horizons H1, H2 and H3, each one
/// with the safeguards and the structural anchors actually missing.
fn cards_rejected_in_history() -> Vec<Card> {
    let mut h1_fire_triple_bet = card(
        "h1.fire.triple-bet",
        Layer::Agent,
        UncertaintyType::Knightian,
        "reinsurer triples the bet after 36 clean months with Brier skill -0,015",
        "should we triple the insurer's bet?",
        "triple the insurer's bet",
    );
    h1_fire_triple_bet.pathologies_covered = false;
    h1_fire_triple_bet.exit_clause = None;

    let mut h2_icp_yeast_one_line = card(
        "h2.icp-yeast.one-line",
        Layer::Model,
        UncertaintyType::Epistemic,
        "intervention validation with a single line per mutant (4 of 40 approved)",
        "should we accept genetics with one line?",
        "accept with one line",
    );
    h2_icp_yeast_one_line.elicited_prior = false;
    h2_icp_yeast_one_line.null_model = None;

    let mut h3_prisons_toronto = card(
        "h3.prisons.toronto",
        Layer::Agent,
        UncertaintyType::Knightian,
        "intercept-only recalibration in Toronto 2020-2021 with ECE 0,067",
        "should we operate the model in Toronto?",
        "operate in Toronto",
    );
    h3_prisons_toronto.declared_drift = false;
    h3_prisons_toronto.subgroups_audited = false;

    vec![
        h1_fire_triple_bet,
        h2_icp_yeast_one_line,
        h3_prisons_toronto,
    ]
}

fn cards(horizon: &str) -> Vec<Card> {
    let h0 = vec![
        card(
            "h0.posto-saude",
            Layer::World,
            UncertaintyType::Aleatory,
            "fila com chegada nao linear e espera convexa",
            "should we open a second room on busy days?",
            "open a second room",
        ),
        card(
            "h0.banco-2003",
            Layer::Model,
            UncertaintyType::Epistemic,
            "2,65 milhoes de chamadas que valem 81 dias de unidade independente",
            "should we reinforce service at the peaks?",
            "reinforce service",
        ),
    ];
    let h1 = vec![
        card(
            "h1.a-multiplicativo",
            Layer::Model,
            UncertaintyType::Aleatory,
            "fundo alavancado 3x sofre arrasto da volatilidade",
            "should we reduce leverage from 3x to 1x?",
            "reduce leverage",
        ),
        card(
            "h1.b-lalonde",
            Layer::Model,
            UncertaintyType::Epistemic,
            "comparacao ingenua carrega vies de selecao de -10292 dolares",
            "should we keep the training program?",
            "keep the training program",
        ),
        card(
            "h1.c-incendios",
            Layer::World,
            UncertaintyType::Knightian,
            "incendios acima de 50 milhoes tem cauda com alfa 1,81",
            "should we triple the bet of the insurer?",
            "triple the bet",
        ),
    ];
    let h2 = vec![
        card(
            "h2.icp-tunel-de-luz",
            Layer::Model,
            UncertaintyType::Epistemic,
            "1024 sensores, causas invariantes entre regimes",
            "should we declare the three LEDs as the cause of the current?",
            "declare the three LEDs",
        ),
        card(
            "h2.icp-fda-herg",
            Layer::Model,
            UncertaintyType::Epistemic,
            "16 medidas por voluntario e tres canais ionicos",
            "should we predict verapamil by the invariance test?",
            "predict verapamil",
        ),
        card(
            "h2.icp-estrela-levedura",
            Layer::Model,
            UncertaintyType::Epistemic,
            "40 genes de levedura com p igual a 0,001",
            "should we keep genetics as the third pillar?",
            "keep genetics",
        ),
    ];
    let h3 = vec![
        card(
            "h3.voos23",
            Layer::Agent,
            UncertaintyType::Epistemic,
            "malha sob relogio com ECE 0,0048 e habilidade 0,019",
            "should we keep the model in the flight pipeline?",
            "keep the model",
        ),
        card(
            "h3.bolsa",
            Layer::Agent,
            UncertaintyType::Knightian,
            "Goodhart medido por instrumento quatro vezes menor",
            "should we correct the target of the stock pipeline?",
            "correct the target",
        ),
        card(
            "h3.energia",
            Layer::Agent,
            UncertaintyType::Epistemic,
            "carga com canario de vazamento armado",
            "should we release the canary in the energy pipeline?",
            "release the canary",
        ),
        card(
            "h3.prisoes",
            Layer::Agent,
            UncertaintyType::Knightian,
            "tres cofres ate acertar a calibracao policial",
            "should we swap the vault from Toronto to Rhode Island?",
            "swap the vault",
        ),
    ];
    match horizon {
        "h0" => h0,
        "h1" => h1,
        "h2" => h2,
        "h3" => h3,
        _ => [h0, h1, h2, h3].concat(),
    }
}

struct Execution {
    report: Vec<String>,
    gates: Vec<(String, bool, String)>,
    root: String,
    failures: usize,
}

/// Card list of the horizon, with the defect injected when asked for.
fn list_with_defect(cfg: &Config) -> Vec<Card> {
    let mut l = cards(&cfg.horizon);
    if let Some(target) = &cfg.defect {
        for c in l.iter_mut() {
            if &c.id == target {
                c.trail_intact = false;
                c.subgroups_audited = false;
            }
        }
    }
    l
}

fn run(cfg: &Config) -> Execution {
    let mut report = Vec::new();
    let mut gates: Vec<(String, bool, String)> = Vec::new();
    let list = list_with_defect(cfg);

    report.push(format!(
        "horizon={} cards={} defect={}",
        cfg.horizon,
        list.len(),
        cfg.defect.clone().unwrap_or_else(|| "none".to_string())
    ));

    let session_id = format!("sessao-{}", session_key(&cfg.horizon));
    let orchestrator = OrchestratorM::new(&session_id, "");
    let mut ledger = Ledger::new(&session_id);
    let mut panel_ok = true;
    let mut checks_passed = 0usize;
    let mut checks_total = 0usize;

    for c in &list {
        let verdict = orchestrator.panel(c);
        checks_passed += verdict.passed();
        checks_total += verdict.items.len();
        if !verdict.approved {
            panel_ok = false;
        }
        let failing: Vec<&str> = verdict.failing().iter().map(|a| a.name()).collect();
        report.push(format!(
            "  card {} checks_passed={}/{} failing={:?} approved={}",
            c.id,
            verdict.passed(),
            verdict.items.len(),
            failing,
            verdict.approved
        ));
        ledger.record(&c.id, &format!("{}|{}", c.definition, verdict.approved));
    }

    // Lens sample, all of it with a fixed seed.
    let mut rng = Rng::new(1909);
    let series: Vec<f64> = (0..20_000)
        .map(|_| {
            let u = (rng.next_u64() as f64) / (u64::MAX as f64);
            (1.0 - u).powf(-1.0 / 1.81)
        })
        .collect();
    let alpha = lenses::hill_index(&series, 500);
    let (median, ruined) = lenses::simulate_multiplicative(2026, 200, 2_000, 0.50, 0.40);
    let (rel, resol, unc) = lenses::murphy_decomposition(
        &[0.1, 0.1, 0.2, 0.2, 0.8, 0.8, 0.9, 0.9],
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    );
    report.push(format!(
        "  lenses: hill_alpha={:.4} wild_tail={} median_log_wealth={:.4} ruined={:.3} murphy_rel={:.4} res={:.4} unc={:.4}",
        alpha,
        lenses::wild_tail(alpha),
        median,
        ruined,
        rel,
        resol,
        unc
    ));

    // G1 determinism: same input, same root.
    let mirror = run_without_gates(cfg);
    let deterministic = mirror.root == ledger.merkle_root();

    // G2 fixed seed.
    let seed_ok = list.iter().all(|c| c.seed.is_some());

    // G3 scope: simulable artifact declared for every card.
    let scope_ok = list.iter().all(|c| c.simulable_artifact.is_some());

    // G4 invariants Phi1-Phi4 and the type III lock.
    let mut invariant_failures = 0;
    for c in &list {
        invariant_failures += evaluate(c).len();
    }
    let invariants_ok = invariant_failures == 0;

    // G5 trail: intact chain and tampering detected.
    let mut tampered = ledger.clone();
    if !tampered.events.is_empty() {
        tampered.events[0].content.push('!');
    }
    let trail_ok = ledger.verify() && !tampered.verify();

    // G6 safeguard panel A1-A8, with drift surveillance (A3), subgroup parity (A6), the
    // H1 Hill (A4) and the mandatory rejection of the three intermediate cards whose
    // safeguards and anchors are missing.
    let alpha_h1 = hill(&series, 500);
    let null_surveillance = cusum(
        &stationary_null(2026, 500, 0.0, 1.0),
        &DriftOptions::default(),
    );
    let null_parity = demographic_parity(&[
        JusticeGroup::new("cohort-a", 100, 30, 24, 6),
        JusticeGroup::new("cohort-b", 100, 30, 24, 6),
    ]);
    let history_rejected_ok = cards_rejected_in_history()
        .iter()
        .all(|c| !orchestrator.panel(c).approved);
    let panel_modules_ok = (alpha - alpha_h1).abs() < 1e-12
        && null_surveillance.alarm.is_none()
        && null_parity.abs() < 1e-12
        && history_rejected_ok;
    let panel_gate_ok = panel_ok && checks_passed == checks_total && panel_modules_ok;

    // G7 declared exit clause and A8 declared and anchored.
    let exit_ok = list.iter().all(|c| c.exit_clause.is_some())
        && list.iter().all(|c| {
            orchestrator
                .conduct_attack(AgentA::A8Decision, c)
                .map(|c| c.passed())
                .unwrap_or(false)
        });

    let root = ledger.merkle_root();
    gates.push((
        "G1".to_string(),
        deterministic,
        format!("root determinism {}", &root[..16]),
    ));
    gates.push((
        "G2".to_string(),
        seed_ok,
        format!("fixed seed in {} cards", list.len()),
    ));
    gates.push((
        "G3".to_string(),
        scope_ok,
        "declared scope: simulated artifact only".to_string(),
    ));
    gates.push((
        "G4".to_string(),
        invariants_ok,
        format!("Phi1-Phi5 and type III (failures={})", invariant_failures),
    ));
    gates.push((
        "G5".to_string(),
        trail_ok,
        "SHA-256 chain intact and tampering detected".to_string(),
    ));
    gates.push((
        "G6".to_string(),
        panel_gate_ok,
        format!(
            "panel A1-A8: {} of {} checks declared and anchored",
            checks_passed, checks_total
        ),
    ));
    gates.push((
        "G7".to_string(),
        exit_ok,
        "declared exit clause and A8 declared and anchored".to_string(),
    ));

    let failures = gates.iter().filter(|(_, ok, _)| !ok).count();
    // The root enters the report so that two runs can be compared byte by byte.
    report.push(format!("merkle_root={}", root));

    Execution {
        report,
        gates,
        root,
        failures,
    }
}

/// Same pass, no gates: it serves as the mirror for G1.
fn run_without_gates(cfg: &Config) -> Execution {
    let list = list_with_defect(cfg);
    let session_id = format!("sessao-{}", session_key(&cfg.horizon));
    let mut ledger = Ledger::new(&session_id);
    let orchestrator = OrchestratorM::new(&session_id, "");
    for c in &list {
        let verdict = orchestrator.panel(c);
        ledger.record(&c.id, &format!("{}|{}", c.definition, verdict.approved));
    }
    let root = ledger.merkle_root();
    Execution {
        report: Vec::new(),
        gates: Vec::new(),
        root,
        failures: 0,
    }
}

fn main() {
    let cfg = config();
    let exec = run(&cfg);
    println!("== Stochology :: deterministic gates ==");
    for line in &exec.report {
        println!("{}", line);
    }

    if cfg.reproduction {
        let second = run(&cfg);
        let identical = second.report == exec.report && second.root == exec.root;
        println!(
            "reproduction: second run identical = {} (root {})",
            identical,
            &second.root[..16]
        );
    }

    for (name, ok, detail) in &exec.gates {
        println!(
            "{} {} {}",
            name,
            if *ok { "ok      " } else { "failed  " },
            detail
        );
    }

    let report_hash = sha256_hex(exec.report.join("|").as_bytes());
    println!("report_hash={}", report_hash);

    if exec.failures == 0 {
        println!("verdict: 7 gates ok");
        std::process::exit(0);
    }
    println!("verdict: {} gates failed", exec.failures);
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_gates_pass_the_all_horizon_with_an_identical_reproduction() {
        let cfg = Config {
            horizon: "all".to_string(),
            reproduction: true,
            defect: None,
        };
        let a = run(&cfg);
        let b = run(&cfg);
        assert_eq!(a.failures, 0);
        assert_eq!(a.gates.len(), 7);
        assert_eq!(a.report, b.report);
        assert_eq!(a.root, b.root);
        assert!(a.root.starts_with("9716e28a87bec9cf"));
    }

    #[test]
    fn defect_in_h3_voos23_fails_gate_g6() {
        let cfg = Config {
            horizon: "h3".to_string(),
            reproduction: false,
            defect: Some("h3.voos23".to_string()),
        };
        let exec = run(&cfg);
        assert_eq!(exec.failures, 1);
        let g6 = exec.gates.iter().find(|(n, _, _)| n == "G6").expect("G6");
        assert!(
            !g6.1,
            "G6 must fail when h3.voos23 loses trail and subgroups"
        );
    }

    #[test]
    fn each_individual_horizon_closes_the_seven_gates() {
        for h in ["h0", "h1", "h2", "h3"] {
            let cfg = Config {
                horizon: h.to_string(),
                reproduction: false,
                defect: None,
            };
            let exec = run(&cfg);
            assert_eq!(exec.failures, 0, "horizon {h} failed");
        }
        let orchestrator = OrchestratorM::new("sessao-todos", "");
        let rejected = cards_rejected_in_history();
        assert_eq!(rejected.len(), 3);
        for c in &rejected {
            let v = orchestrator.panel(c);
            assert!(
                !v.approved,
                "historical card {} should be rejected by the panel",
                c.id
            );
        }
    }
}
