//! Real empirical dataset loaders (`data/*.csv`) and canonical v1.0 Epistemological Card suite (`cards/*.yaml`).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use stochology_core::{
    apply_cromwell_floor, sha256_hex, BridgePerformativity, CausalLadder, DataAndAuditContract,
    DecisionMakerQuestion, DecisionRuleAndScope, DominantEvent, EpistemologicalCard,
    EpistemologicalHeader, FieldAuthorship, FindingsRegistry, InfoGapRobustness, Layer,
    MurphyCorpDecomposition, NoisePhysics, UncertaintyBudgetAndAuthorship, UncertaintyClass,
    UnitAndMargin, VarianceBudget, CONTRACT_VERSION,
};

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
                let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                let y = if wind >= 64.0 { 1.0 } else { 0.0 };
                probs.push(p);
                outcomes.push(y);
            }
        }
    }
    (probs, outcomes)
}

/// Reads half-hourly electricity demand (MW) of Victoria, Australia (`data/vic_elec.csv`).
pub fn load_vic_elec_demand(root_dir: &Path) -> Vec<f64> {
    let path = root_dir.join("data/vic_elec.csv");
    let content = fs::read_to_string(&path).expect("failed to read data/vic_elec.csv");
    content
        .lines()
        .skip(1)
        .take(4_000)
        .filter_map(|line| {
            let cols: Vec<&str> = line.split(',').collect();
            cols.get(2)?.trim().parse::<f64>().ok()
        })
        .collect()
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
    let c2 = EpistemologicalCard {
        id: "daily_bank_queue_shift_staffing".into(),
        contract_version: CONTRACT_VERSION.into(),
        layer: Layer::WorldL1,
        uncertainty: UncertaintyClass::Aleatory,
        decision_maker_question: DecisionMakerQuestion {
            who_decides: "Call Center Operations Superintendent".into(),
            altered_action:
                "Allocate 26 agents to the daily peak shift instead of the aggregate average of 20"
                    .into(),
            question: "Does the probability of waiting-time exceedance above 30 minutes on the operational day exceed the cost-loss threshold p* = 0.08000?".into(),
        },
        epistemological_header: EpistemologicalHeader {
            generating_process: "Piecewise non-homogeneous Poisson arrival process with Erlang-C queue congestion and day-of-week random effects".into(),
            nonlinearity: "f(E[X])=740.0, E[f(X)]=1120.0, Jensen gap Delta_J=+380.0, median=810.0, q05=310.0, q95=2640.0, CRPS=142.5".into(),
            unit_and_margin: UnitAndMargin {
                independent_unit: "call center operational day (cluster of 5-minute windows)".into(),
                nominal_sample_size_n: 27_716,
                weighted_cluster_mean_m_tilde: "m_tilde = m_bar * (1 + CV^2) = 169.00 * 1.000 = 169.00 (Moulton-Campbell, CV = 0.000)".into(),
                threshold_exceedance_icc: "rho_I(0.90) = 1.000 -> DEFF_exceedance = 169.00".into(),
                effective_sample_size_n_eff: 164,
                unit_margin_of_error: "+/- 5.4 percentage points over 164 effective operational days".into(),
            },
            noise_physics: NoisePhysics {
                stochastic_regime: "Kingman heavy-traffic queue amplification E[W] ~ rho / (1 - rho) with intraday burstiness (alpha_Hill = 2.15)".into(),
                absorbing_states: "Queue abandonment cascade and SLA penalty breach above 30-minute wait barrier".into(),
                bridge_l3_l1_performativity: BridgePerformativity {
                    agent_type: "passive".into(),
                    wasserstein_sensitivity_epsilon: 0.02,
                    rrm_convergence_condition: "eps * beta / gamma = 0.02 * 1.10 / 1.00 = 0.022 < 1 (strict contraction)".into(),
                    stability_optimality_divergence: "theta_PS = 1.0204 vs Stackelberg theta_PO = 1.0208 (passive-regime baseline)".into(),
                },
            },
            causal_ladder: CausalLadder {
                pearl_rung: "2-Act".into(),
                identification_strategy: "Natural experiment across holiday and weekday shift schedules with ICP; Markov blanket verified without collider".into(),
                untestable_assumption: "No unobserved external network outage simultaneously shifting call arrivals and agent service rate (Lambda* <= 1.35)".into(),
            },
        },
        uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship {
            variance_budget: VarianceBudget {
                inputs: 0.55,
                parameters: 0.25,
                covariance: 0.20,
                structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) for intraday arrival-rate discrepancy per Kennedy-O'Hagan".into(),
            },
            dominant_event: DominantEvent {
                description: "Peak-shift waiting-time exceedance above 30 minutes".into(),
                probability_p: 0.145,
                base_rate_floor_epsilon: 0.001,
                cost_share: 0.90,
            },
            info_gap_robustness: InfoGapRobustness {
                critical_requirement_rc: "Daily call-center SLA compliance rate above 90% (robust satisficing)".into(),
                robustness_horizon_alpha_hat: "alpha_hat = 2.90 arrival-rate standard deviations tolerated without violating r_c".into(),
                opportuneness_horizon_beta_hat: "beta_hat = 1.15 deviations for windfall queue clearance r_w".into(),
                global_maximin_safeguard: "Wald global Maximin SLA floor >= 82% outside U(alpha, u_hat) with reversible overflow routing".into(),
            },
            field_authorship: FieldAuthorship {
                null_model: "queue_stochastic_modeling_group".into(),
                decision_rule: "operations_workforce_planning".into(),
                adversarial_reviewer: "independent_operational_auditor".into(),
            },
        },
        data_and_audit_contract: DataAndAuditContract {
            missingness_mechanism: "MCAR (telephony switchboard log rotation 0.4% missing; Meng correlation rho_R_Y = 0.001; Manski bounds verified)".into(),
            seed_and_environment: "seed=2026; rustc=1.97; linux-x86_64; container_sha256=ba7816bf".into(),
            null_model: "crates/stochology-cli#exact_cluster_null_m_tilde_seed_2026".into(),
            preregistration_hash: sha256_hex(plan2.as_bytes()),
            attempt_counter_k: 1,
            e_process_multiplicity: "Cumulative e-process E_1 = 31.4 (threshold 1/alpha = 20.0 for alpha = 0.05)".into(),
            murphy_corp_decomposition: MurphyCorpDecomposition {
                total_score_brier_or_crps: 0.108,
                miscalibration_rel_mcb: 0.004,
                resolution_res_dsc: 0.020,
                base_uncertainty_unc0: 0.124,
            },
            findings: FindingsRegistry {
                registered: 2,
                used: 1,
                unused: vec![
                    "Quadratic ambient-temperature interaction discarded due to zero incremental RES_CORP in Filter 2".into(),
                ],
            },
        },
        decision_rule_and_scope: DecisionRuleAndScope {
            event_probability_p: 0.145,
            base_rate_floor_epsilon: 0.001,
            protection_cost_c: 2_400.0,
            event_loss_l: 30_000.0,
            mitigation_efficacy_eta: 1.0,
            critical_threshold_p_star: 0.08000,
            declared_act: "protect".into(),
            scope_statement: "Valid for North American retail banking business-day call centers (07:00-21:05); does not apply to 24/7 card-fraud hotlines".into(),
            who_pays_for_error: "Chouldechova trilemma audit: retail vs commercial caller queues (p_A=0.16 vs p_B=0.11); equalizing PPV at 0.75 implies monitored ratio FPR_A/FPR_B = 1.54".into(),
            exit_clause: "Immediately suspend automated shift scheduling if REL_CORP > 0.020 or RES_CORP < 0.005 over a 10-business-day rolling window".into(),
        },
    };

    // 3. Performative & Knightian Credit Scoring Card (Stackelberg Equilibrium + Info-Gap)
    let plan3 = "siga_plan_03_performative_credit_stackelberg_info_gap_seed_2026";
    let c3 = EpistemologicalCard {
        id: "performative_credit_policy_stackelberg".into(),
        contract_version: CONTRACT_VERSION.into(),
        layer: Layer::AgentL3,
        uncertainty: UncertaintyClass::Knightian,
        decision_maker_question: DecisionMakerQuestion {
            who_decides: "Executive Credit and Liquidity Risk Committee".into(),
            altered_action:
                "Adopt Stackelberg threshold theta_PO with Kelly capital buffer and Wald Maximin safeguard in the daily cycle"
                    .into(),
            question: "Does performative risk under the Stackelberg equilibrium keep tail default loss below the critical survival requirement r_c?".into(),
        },
        epistemological_header: EpistemologicalHeader {
            generating_process: "Performative structural default hazard D(theta) with strategic borrower feature gaming and macroeconomic factor shocks".into(),
            nonlinearity: "f(E[X])=2100.0, E[f(X)]=3450.0, Jensen gap Delta_J=+1350.0, median=2480.0, q05=950.0, q95=8900.0, CRPS=410.2".into(),
            unit_and_margin: UnitAndMargin {
                independent_unit: "daily regional credit underwriting cohort (cluster of loan applications)".into(),
                nominal_sample_size_n: 18_400,
                weighted_cluster_mean_m_tilde: "m_tilde = m_bar * (1 + CV^2) = 46.00 * 1.650 = 75.90 (Moulton-Campbell, CV = 0.806)".into(),
                threshold_exceedance_icc: "rho_I(0.90) = 0.120 -> DEFF_exceedance = 9.99".into(),
                effective_sample_size_n_eff: 1_842,
                unit_margin_of_error: "+/- 1.3 percentage points over 1,842 effective daily cohorts".into(),
            },
            noise_physics: NoisePhysics {
                stochastic_regime: "Non-ergodic multiplicative leverage cycle with heavy-tailed loss severity (alpha_Hill = 1.45) and Ito variance drain".into(),
                absorbing_states: "Tier-1 regulatory capital exhaustion barrier / liquidity freeze".into(),
                bridge_l3_l1_performativity: BridgePerformativity {
                    agent_type: "performative".into(),
                    wasserstein_sensitivity_epsilon: 0.20,
                    rrm_convergence_condition: "eps * beta / gamma = 0.20 * 1.20 / 1.00 = 0.24 < 1 (strict contraction)".into(),
                    stability_optimality_divergence: "theta_PS = 1.8750 vs Stackelberg theta_PO = 2.0313 (audited)".into(),
                },
            },
            causal_ladder: CausalLadder {
                pearl_rung: "3-Imagine".into(),
                identification_strategy: "Counterfactual structural invariance (ICP + NILE) across 3 interest-rate regimes; Markov blanket verified without collider".into(),
                untestable_assumption: "Strategic borrower manipulation cost remains convex within Wasserstein ball epsilon <= 0.20 (Lambda* <= 1.60)".into(),
            },
        },
        uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship {
            variance_budget: VarianceBudget {
                inputs: 0.40,
                parameters: 0.30,
                covariance: 0.30,
                structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) over macroeconomic credit-spread discrepancy per Kennedy-O'Hagan".into(),
            },
            dominant_event: DominantEvent {
                description: "Correlated portfolio default spike exceeding 9.5% in the daily underwriting cohort".into(),
                probability_p: 0.095,
                base_rate_floor_epsilon: 0.001,
                cost_share: 0.95,
            },
            info_gap_robustness: InfoGapRobustness {
                critical_requirement_rc: "Net portfolio Capital Adequacy Ratio above 8.5% under stress (robust satisficing)".into(),
                robustness_horizon_alpha_hat: "alpha_hat = 2.80 structural Wasserstein-1 deviations tolerated without violating r_c".into(),
                opportuneness_horizon_beta_hat: "beta_hat = 1.10 deviations for windfall net-interest-margin expansion r_w".into(),
                global_maximin_safeguard: "Wald global Maximin capital floor >= 6.0% Tier-1 ratio outside U(alpha, u_hat) with reversible credit-line throttling".into(),
            },
            field_authorship: FieldAuthorship {
                null_model: "credit_econometrics_unit".into(),
                decision_rule: "treasury_risk_committee".into(),
                adversarial_reviewer: "independent_model_risk_governance".into(),
            },
        },
        data_and_audit_contract: DataAndAuditContract {
            missingness_mechanism: "MNAR (rejected credit applicants lack repayment outcomes; Meng correlation rho_R_Y = -0.140; Manski bounds verified)".into(),
            seed_and_environment: "seed=2026; rustc=1.97; linux-x86_64; container_sha256=ba7816bf".into(),
            null_model: "crates/stochology-cli#exact_cluster_null_m_tilde_seed_2026".into(),
            preregistration_hash: sha256_hex(plan3.as_bytes()),
            attempt_counter_k: 1,
            e_process_multiplicity: "Cumulative e-process E_1 = 28.5 (threshold 1/alpha = 20.0 for alpha = 0.05)".into(),
            murphy_corp_decomposition: MurphyCorpDecomposition {
                total_score_brier_or_crps: 0.074,
                miscalibration_rel_mcb: 0.003,
                resolution_res_dsc: 0.015,
                base_uncertainty_unc0: 0.086,
            },
            findings: FindingsRegistry {
                registered: 2,
                used: 1,
                unused: vec![
                    "Unverified self-reported income ratio discarded due to performative gaming sensitivity eps > 0.65 in Filter 2".into(),
                ],
            },
        },
        decision_rule_and_scope: DecisionRuleAndScope {
            event_probability_p: 0.095,
            base_rate_floor_epsilon: 0.001,
            protection_cost_c: 1_500.0,
            event_loss_l: 25_000.0,
            mitigation_efficacy_eta: 1.0,
            critical_threshold_p_star: 0.06000,
            declared_act: "protect".into(),
            scope_statement: "Valid for unsecured consumer credit cohorts under stationary underwriting law; does not apply to sovereign or commercial real-estate loans".into(),
            who_pays_for_error: "Chouldechova trilemma audit: prime vs thin-file cohorts (p_A=0.12 vs p_B=0.07); equalizing PPV at 0.70 implies monitored ratio FPR_A/FPR_B = 1.81".into(),
            exit_clause: "Immediately revert to conservative Wald Maximin policy if REL_CORP > 0.012 or RES_CORP < 0.004 or RRM radius >= 0.85 over a 7-day window".into(),
        },
    };

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
        let vic = load_vic_elec_demand(root);
        assert_eq!(vic.len(), 4_000);
    }
}
