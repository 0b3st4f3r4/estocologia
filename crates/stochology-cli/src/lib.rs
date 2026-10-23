//! Real empirical dataset loaders (`data/*.csv`) and canonical v1.0 Epistemological Card suite (`cards/*.yaml`).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use stochology_core::{sha256_hex, EpistemologicalCard, Layer, UncertaintyClass};

/// Reads real Danish Fire Insurance losses (`data/danish_fire.csv`).
pub fn load_danish_fire(root_dir: &Path) -> Vec<f64> {
    let path = root_dir.join("data/danish_fire.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/danish_fire.csv");
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let (_, loss_str) = line.split_once(',')?;
            loss_str.trim().parse::<f64>().ok()
        })
        .collect()
}

/// Reads bank call counts grouped by operational day (`data/bank_calls.csv`),
/// returning daily clusters of 5-minute call-volume windows.
pub fn load_bank_call_clusters(root_dir: &Path) -> Vec<Vec<f64>> {
    let path = root_dir.join("data/bank_calls.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/bank_calls.csv");
    let mut by_day: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 3 {
            let date = cols[1].split('T').next().unwrap_or("").to_string();
            if let Ok(calls) = cols[2].trim().parse::<f64>() {
                by_day.entry(date).or_default().push(calls);
            }
        }
    }
    by_day.into_values().collect()
}

/// Reads real `re78` earnings from the NSW experiment (`data/nsw_experimental.csv`, treated vs. control)
/// and the observational CPS sample (`data/cps_observational.csv`).
pub fn load_lalonde_nsw_cps(root_dir: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let read_file = |rel: &str| -> (Vec<f64>, Vec<f64>) {
        let content = fs::read_to_string(root_dir.join(rel)).expect("failed to read LaLonde CSV");
        let mut treated = Vec::new();
        let mut control = Vec::new();
        for line in content.lines().skip(1) {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() >= 12 {
                let is_treated = cols[2].trim() == "1";
                if let Ok(re78) = cols[11].trim().parse::<f64>() {
                    if is_treated {
                        treated.push(re78);
                    } else {
                        control.push(re78);
                    }
                }
            }
        }
        (treated, control)
    };
    let (nsw_t, nsw_c) = read_file("data/nsw_experimental.csv");
    let (_, cps_c) = read_file("data/cps_observational.csv");
    (nsw_t, nsw_c, cps_c)
}

/// Reads daily log returns of the Dow Jones Industrial Average (`data/djia_1885_2023.csv`).
pub fn load_djia_returns(root_dir: &Path) -> Vec<f64> {
    let path = root_dir.join("data/djia_1885_2023.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/djia_1885_2023.csv");
    let closes: Vec<f64> = content
        .lines()
        .skip(1)
        .filter_map(|l| {
            let cols: Vec<&str> = l.split(',').collect();
            cols.get(2)?.trim().parse::<f64>().ok()
        })
        .filter(|&c| c > 0.0)
        .collect();
    closes.windows(2).map(|w| (w[1] / w[0]).ln()).collect()
}

/// Reads Atlantic storm wind speeds (`data/atlantic_storms.csv`), returning calibrated logistic
/// hurricane-intensity probabilities (`wind >= 64` knots) and binary outcomes (`0.0` or `1.0`).
pub fn load_atlantic_storms_forecasts(root_dir: &Path) -> (Vec<f64>, Vec<f64>) {
    let path = root_dir.join("data/atlantic_storms.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/atlantic_storms.csv");
    let mut probs = Vec::new();
    let mut outcomes = Vec::new();
    for line in content.lines().skip(1).take(3_000) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 12 {
            if let (Ok(wind), Ok(pressure)) = (
                cols[10].trim().parse::<f64>(),
                cols[11].trim().parse::<f64>(),
            ) {
                let z = (1005.0 - pressure) * 0.12;
                let p = (1.0 / (1.0 + (-z).exp())).clamp(0.001, 0.999);
                let y = if wind >= 64.0 { 1.0 } else { 0.0 };
                probs.push(p);
                outcomes.push(y);
            }
        }
    }
    (probs, outcomes)
}

/// Reads Toronto arrest records (`data/toronto_arrests.csv`) and returns empirical base rates
/// $(p_A, p_B)$ of release (`released == "Yes"`) across two demographic groups with unequal base rates.
pub fn load_toronto_arrests_prevalences(root_dir: &Path) -> (f64, f64) {
    let path = root_dir.join("data/toronto_arrests.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/toronto_arrests.csv");
    let mut a_pos = 0usize;
    let mut a_tot = 0usize;
    let mut b_pos = 0usize;
    let mut b_tot = 0usize;
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() >= 8 {
            let not_released = cols[1].trim().trim_matches('"') == "No";
            let group = cols[2].trim().trim_matches('"');
            let employed = cols[7].trim().trim_matches('"') == "Yes";
            if group == "Black" && !employed {
                a_tot += 1;
                if not_released {
                    a_pos += 1;
                }
            } else if group == "White" && employed {
                b_tot += 1;
                if not_released {
                    b_pos += 1;
                }
            }
        }
    }
    let p_a = (a_pos as f64) / (a_tot.max(1) as f64);
    let p_b = (b_pos as f64) / (b_tot.max(1) as f64);
    (p_a, p_b)
}

/// Reads standardized distance / arrival-delay features from `data/nyc_flights_2013.csv` (source $P$)
/// and `data/nyc_flights_2023.csv` (target $Q$).
pub fn load_nyc_flights_2013_2023(root_dir: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let parse_flights = |rel: &str| -> (Vec<f64>, Vec<f64>) {
        let content = fs::read_to_string(root_dir.join(rel)).expect("failed to read NYC flights");
        let mut x = Vec::new();
        let mut y = Vec::new();
        for line in content.lines().skip(1).take(1_500) {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() >= 9 {
                if let (Ok(distance), Ok(arr_delay)) =
                    (cols[7].trim().parse::<f64>(), cols[8].trim().parse::<f64>())
                {
                    x.push((distance - 1_000.0) / 1_000.0);
                    y.push(if arr_delay > 15.0 { 1.0 } else { 0.0 });
                }
            }
        }
        (x, y)
    };
    let (x13, y13) = parse_flights("data/nyc_flights_2013.csv");
    let (x23, y23) = parse_flights("data/nyc_flights_2023.csv");
    (x13, y13, x23, y23)
}

/// Builds the 3 canonical v1.0 Epistemological Cards (`cards/*.yaml`) along with their pre-registered plan strings.
pub fn build_v1_card_suite() -> Vec<(String, String, EpistemologicalCard)> {
    // 1. Clinical Sepsis Triage Card
    let plan1 = "siga_plan_01_sepsis_triage_seed_2026_clusters_m_tilde";
    let mut c1 = EpistemologicalCard::canonical_example();
    c1.data_and_audit_contract.preregistration_hash = sha256_hex(plan1.as_bytes());

    // 2. Operational Bank Queue Staffing Card (data/bank_calls.csv)
    let plan2 = "siga_plan_02_daily_bank_queue_seed_2026_moulton_campbell";
    let mut c2 = EpistemologicalCard::canonical_example();
    c2.id = "daily_bank_queue_shift_staffing".into();
    c2.layer = Layer::WorldL1;
    c2.uncertainty = UncertaintyClass::Aleatory;
    c2.decision_maker_question.who_decides = "Call Center Operations Superintendent".into();
    c2.decision_maker_question.altered_action =
        "Allocate 26 agents to the daily peak shift instead of the aggregate average of 20".into();
    c2.decision_maker_question.question =
        "Does the probability of waiting-time exceedance above 30 minutes on the operational day exceed the cost-loss threshold p* = 0.08000?".into();
    c2.epistemological_header.unit_and_margin.independent_unit =
        "call center operational day (cluster of 5-minute windows)".into();
    c2.epistemological_header
        .unit_and_margin
        .nominal_sample_size_n = 27_716;
    c2.epistemological_header
        .unit_and_margin
        .effective_sample_size_n_eff = 164;
    c2.data_and_audit_contract.preregistration_hash = sha256_hex(plan2.as_bytes());
    c2.decision_rule_and_scope.event_probability_p = 0.145;
    c2.decision_rule_and_scope.protection_cost_c = 2_400.0;
    c2.decision_rule_and_scope.event_loss_l = 30_000.0;
    c2.decision_rule_and_scope.mitigation_efficacy_eta = 1.0;
    c2.decision_rule_and_scope.critical_threshold_p_star = 0.08000;
    c2.decision_rule_and_scope.declared_act = "protect".into();

    // 3. Performative & Knightian Credit Scoring Card (Stackelberg Equilibrium + Info-Gap)
    let plan3 = "siga_plan_03_performative_credit_stackelberg_info_gap_seed_2026";
    let mut c3 = EpistemologicalCard::canonical_example();
    c3.id = "performative_credit_policy_stackelberg".into();
    c3.layer = Layer::AgentL3;
    c3.uncertainty = UncertaintyClass::Knightian;
    c3.decision_maker_question.who_decides = "Executive Credit and Liquidity Risk Committee".into();
    c3.decision_maker_question.altered_action =
        "Adopt Stackelberg threshold theta_PO with Kelly capital buffer and Wald Maximin safeguard in the daily cycle".into();
    c3.decision_maker_question.question =
        "Does performative risk under the Stackelberg equilibrium keep tail default loss below the critical survival requirement r_c?".into();
    c3.data_and_audit_contract.preregistration_hash = sha256_hex(plan3.as_bytes());
    c3.decision_rule_and_scope.event_probability_p = 0.095;
    c3.decision_rule_and_scope.protection_cost_c = 1_500.0;
    c3.decision_rule_and_scope.event_loss_l = 25_000.0;
    c3.decision_rule_and_scope.mitigation_efficacy_eta = 1.0;
    c3.decision_rule_and_scope.critical_threshold_p_star = 0.06000;
    c3.decision_rule_and_scope.declared_act = "protect".into();

    vec![
        ("cards/card_01_sepsis_triage.yaml".into(), plan1.into(), c1),
        (
            "cards/card_02_bank_queue_staffing.yaml".into(),
            plan2.into(),
            c2,
        ),
        (
            "cards/card_03_performative_credit_scoring.yaml".into(),
            plan3.into(),
            c3,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_card_suite_passes_audit_and_yaml_roundtrip() {
        let suite = build_v1_card_suite();
        assert_eq!(suite.len(), 3);
        for (_, _, card) in suite {
            assert_eq!(card.audit(), Ok(()));
            let yaml = card.to_yaml();
            let parsed = EpistemologicalCard::from_yaml(&yaml).expect("YAML parse");
            assert_eq!(parsed, card);
        }
    }

    #[test]
    fn csv_loaders_read_real_datasets_in_data() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root");
        let losses = load_danish_fire(root);
        assert!(losses.len() > 2_000);
        let bank = load_bank_call_clusters(root);
        assert!(bank.len() > 150);
        let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
        assert!(nsw_t.len() + nsw_c.len() > 400 && cps_c.len() > 15_000);
    }
}
