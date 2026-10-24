//! Structured Epistemological Card (`EpistemologicalCard`, `contract_version: "1.0"`, §2.3),
//! zero-dependency YAML parser/serializer, and algorithmic validator (`audit()`) enforcing
//! the 4 Prohibitions and the 5 Boundary Invariants ($\Phi_1$ to $\Phi_5$).

use crate::invariants::verify_resolution_phi5;
use crate::layers::{respects_cromwell, Layer, UncertaintyClass};
use crate::rng::DEFAULT_SEED;
use crate::sha256::is_valid_sha256_hex;
use std::collections::BTreeMap;

/// Canonical version of the Stochology Epistemological Contract.
pub const CONTRACT_VERSION: &str = "1.0";

/// Missingness mechanism classification according to Donald Rubin (Invariant $\Phi_3$).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingnessMechanism {
    Mcar,
    Mar,
    Mnar,
}

impl MissingnessMechanism {
    pub fn from_prefix(s: &str) -> Option<Self> {
        let t = s.trim();
        if t.starts_with("MCAR") {
            Some(Self::Mcar)
        } else if t.starts_with("MAR") {
            Some(Self::Mar)
        } else if t.starts_with("MNAR") {
            Some(Self::Mnar)
        } else {
            None
        }
    }
}

/// Decision-maker's 3-part operational question (Step 2 of SIGA & Prohibition 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionMakerQuestion {
    pub who_decides: String,
    pub altered_action: String,
    pub question: String,
}

/// Independent unit, Moulton-Campbell cluster correction, and exceedance ICC (Invariant $\Phi_2$).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitAndMargin {
    pub independent_unit: String,
    pub nominal_sample_size_n: usize,
    pub weighted_cluster_mean_m_tilde: String,
    pub threshold_exceedance_icc: String,
    pub effective_sample_size_n_eff: usize,
    pub unit_margin_of_error: String,
}

/// Performative prediction audit on Bridge $L_3 \to L_1$ (Prohibition 3).
#[derive(Debug, Clone, PartialEq)]
pub struct BridgePerformativity {
    pub agent_type: String,
    pub wasserstein_sensitivity_epsilon: f64,
    pub rrm_convergence_condition: String,
    pub stability_optimality_divergence: String,
}

/// Noise physics, absorbing barriers, and performative dynamics.
#[derive(Debug, Clone, PartialEq)]
pub struct NoisePhysics {
    pub stochastic_regime: String,
    pub absorbing_states: String,
    pub bridge_l3_l1_performativity: BridgePerformativity,
}

/// Pearl's Causal Ladder declaration (Lens 4 & Invariant $\Phi_3$).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalLadder {
    pub pearl_rung: String,
    pub identification_strategy: String,
    pub untestable_assumption: String,
}

/// Epistemological header of the card (§2.3).
#[derive(Debug, Clone, PartialEq)]
pub struct EpistemologicalHeader {
    pub generating_process: String,
    pub nonlinearity: String,
    pub unit_and_margin: UnitAndMargin,
    pub noise_physics: NoisePhysics,
    pub causal_ladder: CausalLadder,
}

/// Variance budget and Kennedy-O'Hagan Gaussian Process structural discrepancy (§1.2).
#[derive(Debug, Clone, PartialEq)]
pub struct VarianceBudget {
    pub inputs: f64,
    pub parameters: f64,
    pub covariance: f64,
    pub structural_discrepancy_gp: String,
}

/// Dominant cost-driving event under Cromwell's Rule.
#[derive(Debug, Clone, PartialEq)]
pub struct DominantEvent {
    pub description: String,
    pub probability_p: f64,
    pub base_rate_floor_epsilon: f64,
    pub cost_share: f64,
}

/// Info-Gap robustness/opportuneness horizons and Sniedovich's Global Maximin Safeguard (§1.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoGapRobustness {
    pub critical_requirement_rc: String,
    pub robustness_horizon_alpha_hat: String,
    pub opportuneness_horizon_beta_hat: String,
    pub global_maximin_safeguard: String,
}

/// Independent cross-authorship and adversarial review (Filter 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAuthorship {
    pub null_model: String,
    pub decision_rule: String,
    pub adversarial_reviewer: String,
}

/// Uncertainty budget, Info-Gap robustness, and cross-authorship section.
#[derive(Debug, Clone, PartialEq)]
pub struct UncertaintyBudgetAndAuthorship {
    pub variance_budget: VarianceBudget,
    pub dominant_event: DominantEvent,
    pub info_gap_robustness: InfoGapRobustness,
    pub field_authorship: FieldAuthorship,
}

/// Isotonic CORP-Murphy decomposition via PAV ($\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$, Lens 5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MurphyCorpDecomposition {
    pub total_score_brier_or_crps: f64,
    pub miscalibration_rel_mcb: f64,
    pub resolution_res_dsc: f64,
    pub base_uncertainty_unc0: f64,
}

/// Accounting of registered, used, and discarded findings (Prohibition 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingsRegistry {
    pub registered: usize,
    pub used: usize,
    pub unused: Vec<String>,
}

/// Data contract, pre-registration seal, e-process multiplicity, and CORP-Murphy audit.
#[derive(Debug, Clone, PartialEq)]
pub struct DataAndAuditContract {
    pub missingness_mechanism: String,
    pub seed_and_environment: String,
    pub null_model: String,
    pub preregistration_hash: String,
    pub attempt_counter_k: usize,
    pub e_process_multiplicity: String,
    pub murphy_corp_decomposition: MurphyCorpDecomposition,
    pub findings: FindingsRegistry,
}

/// Cost-loss decision boundary $p^* = \frac{C}{\eta L}$, Chouldechova fairness audit, and numeric exit clause ($\Phi_4$).
#[derive(Debug, Clone, PartialEq)]
pub struct DecisionRuleAndScope {
    pub event_probability_p: f64,
    pub base_rate_floor_epsilon: f64,
    pub protection_cost_c: f64,
    pub event_loss_l: f64,
    pub mitigation_efficacy_eta: f64,
    pub critical_threshold_p_star: f64,
    pub declared_act: String,
    pub scope_statement: String,
    pub who_pays_for_error: String,
    pub exit_clause: String,
}

/// Complete Epistemological Card (`contract_version: "1.0"`).
#[derive(Debug, Clone, PartialEq)]
pub struct EpistemologicalCard {
    pub id: String,
    pub contract_version: String,
    pub layer: Layer,
    pub uncertainty: UncertaintyClass,
    pub decision_maker_question: DecisionMakerQuestion,
    pub epistemological_header: EpistemologicalHeader,
    pub uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship,
    pub data_and_audit_contract: DataAndAuditContract,
    pub decision_rule_and_scope: DecisionRuleAndScope,
}

/// Formal contract violation returned by [`EpistemologicalCard::audit`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractViolation {
    pub gate_code: &'static str,
    pub description: String,
}

impl EpistemologicalCard {
    /// Returns a canonical, fully valid v1.0 Epistemological Card (Clinical Sepsis Triage).
    pub fn canonical_example() -> Self {
        Self {
            id: "clinical_sepsis_triage_icu_shift".into(),
            contract_version: CONTRACT_VERSION.into(),
            layer: Layer::ModelL2,
            uncertainty: UncertaintyClass::EpistemicSpecification,
            decision_maker_question: DecisionMakerQuestion {
                who_decides: "On-Duty ICU Attending Physician".into(),
                altered_action: "Trigger early sepsis bundle within 1h for patients on the shift above threshold p*".into(),
                question: "Does the calibrated probability of septic shock within the 12h shift exceed the cost-loss threshold p* = 0.05294?".into(),
            },
            epistemological_header: EpistemologicalHeader {
                generating_process: "Hierarchical log-logistic hazard with hospital-shift random effects and Kennedy-O'Hagan GP discrepancy".into(),
                nonlinearity: "f(E[X])=1420.0, E[f(X)]=1890.0, Jensen gap Delta_J=+470.0, median=1510.0, q05=820.0, q95=4120.0, CRPS=215.4".into(),
                unit_and_margin: UnitAndMargin {
                    independent_unit: "12h clinical ICU shift (cluster of bed-hours)".into(),
                    nominal_sample_size_n: 25_028,
                    weighted_cluster_mean_m_tilde: "m_tilde = m_bar * (1 + CV^2) = 59.59 * 1.998 = 119.05 (Moulton-Campbell, CV = 0.999)".into(),
                    threshold_exceedance_icc: "rho_I(0.90) = 0.155 -> DEFF_exceedance = 19.30".into(),
                    effective_sample_size_n_eff: 1_297,
                    unit_margin_of_error: "+/- 1.4 percentage points over 1,297 effective shifts".into(),
                },
                noise_physics: NoisePhysics {
                    stochastic_regime: "Non-ergodic multiplicative organ-failure cascade with sub-exponential tail (alpha_Hill = 1.60)".into(),
                    absorbing_states: "Irreversible refractory septic shock / clinical mortality barrier".into(),
                    bridge_l3_l1_performativity: BridgePerformativity {
                        agent_type: "performative".into(),
                        wasserstein_sensitivity_epsilon: 0.18,
                        rrm_convergence_condition: "eps * beta / gamma = 0.18 * 1.50 / 1.00 = 0.27 < 1 (strict contraction)".into(),
                        stability_optimality_divergence: "theta_PS = 1.8293 vs Stackelberg theta_PO = 1.9631 (audited)".into(),
                    },
                },
                causal_ladder: CausalLadder {
                    pearl_rung: "2-Act".into(),
                    identification_strategy: "Invariant Causal Prediction (ICP) + NILE across 3 hospital shift regimes; Markov blanket verified without collider".into(),
                    untestable_assumption: "Hidden confounding bounded by sensitivity threshold Lambda* <= 1.50".into(),
                },
            },
            uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship {
                variance_budget: VarianceBudget {
                    inputs: 0.45,
                    parameters: 0.35,
                    covariance: 0.20,
                    structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) per Kennedy-O'Hagan".into(),
                },
                dominant_event: DominantEvent {
                    description: "Refractory septic shock within the first 12h shift".into(),
                    probability_p: 0.05,
                    base_rate_floor_epsilon: 0.001,
                    cost_share: 1.00,
                },
                info_gap_robustness: InfoGapRobustness {
                    critical_requirement_rc: "Operational shift survival rate above 92% (robust satisficing)".into(),
                    robustness_horizon_alpha_hat: "alpha_hat = 3.50 structural standard deviations tolerated without violating r_c".into(),
                    opportuneness_horizon_beta_hat: "beta_hat = 1.20 deviations for windfall mortality reduction r_w".into(),
                    global_maximin_safeguard: "Wald global Maximin bound >= 85% outside U(alpha, u_hat) with reversible escalation protocol".into(),
                },
                field_authorship: FieldAuthorship {
                    null_model: "inference".into(),
                    decision_rule: "decision".into(),
                    adversarial_reviewer: "independent_adversarial_auditor".into(),
                },
            },
            data_and_audit_contract: DataAndAuditContract {
                missingness_mechanism: "MAR (arterial lactate 4.2% missing; Meng correlation rho_R_Y = 0.003; Manski bounds verified)".into(),
                seed_and_environment: "seed=2026; rustc=1.97; linux-x86_64; container_sha256=ba7816bf".into(),
                null_model: "crates/stochology-cli#exact_cluster_null_m_tilde_seed_2026".into(),
                preregistration_hash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
                attempt_counter_k: 1,
                e_process_multiplicity: "Cumulative e-process E_1 = 24.8 (threshold 1/alpha = 20.0 for alpha = 0.05)".into(),
                murphy_corp_decomposition: MurphyCorpDecomposition {
                    total_score_brier_or_crps: 0.046,
                    miscalibration_rel_mcb: 0.002,
                    resolution_res_dsc: 0.004,
                    base_uncertainty_unc0: 0.048,
                },
                findings: FindingsRegistry {
                    registered: 2,
                    used: 1,
                    unused: vec![
                        "Three-way interaction discarded due to insufficient tail n_eff(0.90) in Filter 2".into(),
                    ],
                },
            },
            decision_rule_and_scope: DecisionRuleAndScope {
                event_probability_p: 0.05,
                base_rate_floor_epsilon: 0.001,
                protection_cost_c: 1_800.0,
                event_loss_l: 34_000.0,
                mitigation_efficacy_eta: 1.0,
                critical_threshold_p_star: 0.05294,
                declared_act: "do_not_protect".into(),
                scope_statement: "Valid for adult inpatient wards under standardized triage; does not apply to pediatric or neonatal ICU".into(),
                who_pays_for_error: "Chouldechova trilemma audit: base rates p_A=0.06 vs p_B=0.04; equalizing PPV at 0.72 implies monitored ratio FPR_A/FPR_B = 1.53".into(),
                exit_clause: "Immediately suspend the model if REL_CORP > 0.015 or RES_CORP < 0.002 over a 14-day rolling window".into(),
            },
        }
    }

    /// Audits the Epistemological Card against the 4 Prohibitions and the 5 Boundary Invariants ($\Phi_1$–$\Phi_5$).
    pub fn audit(&self) -> Result<(), Vec<ContractViolation>> {
        let mut v = Vec::new();

        // Contract Version
        if self.contract_version.trim() != CONTRACT_VERSION {
            v.push(ContractViolation {
                gate_code: "CONTRACT_VERSION",
                description: format!(
                    "contract_version '{}' differs from canonical version '{}'",
                    self.contract_version, CONTRACT_VERSION
                ),
            });
        }

        // Step 2 & Prohibition 1: 3-part decision-maker question
        let q = &self.decision_maker_question;
        let dm_lower = q.who_decides.trim().to_ascii_lowercase();
        let dm_tokens: Vec<&str> = dm_lower
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|t| !t.is_empty())
            .collect();
        let forbidden_dm_tokens = [
            "system",
            "team",
            "generic",
            "analyst",
            "model",
            "ai",
            "algorithm",
        ];
        if dm_tokens.len() < 2 || dm_tokens.iter().any(|t| forbidden_dm_tokens.contains(t)) {
            v.push(ContractViolation {
                gate_code: "STEP2_DECISION_MAKER",
                description: "who_decides cannot be empty or generic".into(),
            });
        }
        if q.altered_action.trim().is_empty() {
            v.push(ContractViolation {
                gate_code: "STEP2_ACTION",
                description: "altered_action is mandatory in Step 2".into(),
            });
        }
        if !q.question.trim().ends_with('?') {
            v.push(ContractViolation {
                gate_code: "STEP2_QUESTION_MARK",
                description: "decision-maker question must end with '?'".into(),
            });
        }

        // Prohibition 2: Ban on the lone mean (keywords + numeric validation)
        let nl = self
            .epistemological_header
            .nonlinearity
            .to_ascii_lowercase();
        let mut nl_missing_term = false;
        for term in [
            "f(e[x])", "e[f(x)]", "jensen", "median", "q05", "q95", "crps",
        ] {
            if !nl.contains(term) {
                nl_missing_term = true;
                v.push(ContractViolation {
                    gate_code: "PROHIBITION_2_LONE_MEAN",
                    description: format!(
                        "nonlinearity field omits '{}' required by Prohibition 2",
                        term
                    ),
                });
            }
        }
        if !nl_missing_term {
            let f_ex = extract_number_after(&nl, "f(e[x])");
            let e_fx = extract_number_after(&nl, "e[f(x)]");
            let delta_j = extract_number_after(&nl, "delta_j")
                .or_else(|| extract_number_after(&nl, "jensen"));
            let med = extract_number_after(&nl, "median");
            let q05 = extract_number_after(&nl, "q05");
            let q95 = extract_number_after(&nl, "q95");
            let crps = extract_number_after(&nl, "crps");
            match (f_ex, e_fx, delta_j, med, q05, q95, crps) {
                (Some(f_ex), Some(e_fx), Some(dj), Some(med), Some(q05), Some(q95), Some(crps))
                    if dj.abs() > 1e-6
                        && ((e_fx - f_ex) - dj).abs() <= 0.5_f64.max(e_fx.abs() * 1e-3)
                        && q05 < med
                        && med < q95
                        && q05 <= f_ex
                        && f_ex <= q95
                        && q05 <= e_fx
                        && e_fx <= q95
                        && 0.0 < crps
                        && crps < (q95 - q05) => {}
                _ => {
                    v.push(ContractViolation {
                        gate_code: "PROHIBITION_2_LONE_MEAN",
                        description: "nonlinearity numeric values fail non-zero Jensen gap identity (Delta_J = E[f(X)] - f(E[X])), strict quantile ordering (q05 < median < q95), or 0 < CRPS < (q95 - q05)".into(),
                    });
                }
            }
        }

        // Prohibition 3: Performativity on Bridge L3 -> L1
        let perf = &self
            .epistemological_header
            .noise_physics
            .bridge_l3_l1_performativity;
        if perf.agent_type != "passive" && perf.agent_type != "performative" {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_AGENT_TYPE",
                description: "agent_type must be 'passive' or 'performative'".into(),
            });
        }
        if !perf.wasserstein_sensitivity_epsilon.is_finite()
            || perf.wasserstein_sensitivity_epsilon < 0.0
            || (perf.agent_type == "performative"
                && (perf.wasserstein_sensitivity_epsilon <= 0.0
                    || perf.wasserstein_sensitivity_epsilon >= 1.0))
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_EPSILON",
                description: "wasserstein_sensitivity_epsilon must be finite and in (0, 1) for performative agents (or >= 0 for passive)".into(),
            });
        }
        if !validate_rrm_contraction_string(
            &perf.rrm_convergence_condition,
            perf.wasserstein_sensitivity_epsilon,
        ) {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_RRM_CONTRACTION",
                description:
                    "rrm_convergence_condition must numerically satisfy eps * beta / gamma < 1"
                        .into(),
            });
        }
        if perf.agent_type == "performative" {
            let div = perf.stability_optimality_divergence.to_ascii_lowercase();
            let theta_ps = extract_number_after(&div, "theta_ps");
            let theta_po = extract_number_after(&div, "theta_po");
            match (theta_ps, theta_po) {
                (Some(ps), Some(po)) if (ps - po).abs() > 1e-6 => {}
                _ => {
                    v.push(ContractViolation {
                        gate_code: "PROHIBITION_3_PS_VS_PO",
                        description: "performative agent must explicitly report distinct numeric theta_PS and theta_PO"
                            .into(),
                    });
                }
            }
        }

        // Invariant Φ1: Simulability (seed=2026 and executable null model)
        let seed_env = &self.data_and_audit_contract.seed_and_environment;
        if parse_seed_from_env(seed_env) != Some(DEFAULT_SEED) {
            v.push(ContractViolation {
                gate_code: "PHI1_SEED",
                description: format!(
                    "seed_and_environment must declare canonical 'seed={DEFAULT_SEED}'"
                ),
            });
        }
        let nm_trimmed = self.data_and_audit_contract.null_model.trim();
        let nm_lower = nm_trimmed.to_ascii_lowercase();
        if nm_trimmed.is_empty()
            || nm_lower == "none"
            || nm_lower.starts_with("none ")
            || (!nm_trimmed.contains('#') && !nm_trimmed.contains("::"))
        {
            v.push(ContractViolation {
                gate_code: "PHI1_PHI3_NULL_MODEL",
                description: "executable null_model (containing '#' or '::') is mandatory".into(),
            });
        }

        // Invariant Φ2: Independent unit, Moulton-Campbell m_tilde, and exceedance n_eff(p)
        let um = &self.epistemological_header.unit_and_margin;
        let u_lower = um.independent_unit.trim().to_ascii_lowercase();
        if u_lower.is_empty()
            || ((u_lower.contains("row")
                || u_lower.contains("transaction")
                || u_lower.contains("transactional"))
                && !u_lower.contains("cluster of"))
        {
            v.push(ContractViolation {
                gate_code: "PHI2_PSEUDOREPLICATION",
                description: "independent_unit cannot be the correlated transactional row".into(),
            });
        }
        let m_tilde_lower = um.weighted_cluster_mean_m_tilde.to_ascii_lowercase();
        let icc_lower = um.threshold_exceedance_icc.to_ascii_lowercase();
        let parsed_m_tilde = parse_m_tilde(&m_tilde_lower);
        let parsed_rho_i = extract_number_after(&icc_lower, "rho_i(0.90) =")
            .or_else(|| extract_number_after(&icc_lower, "rho_i"));

        if (!um.weighted_cluster_mean_m_tilde.contains("CV") && !m_tilde_lower.contains("moulton"))
            || m_tilde_lower.contains("ignored")
            || !parsed_m_tilde.is_some_and(|m| m > 1.0)
        {
            v.push(ContractViolation {
                gate_code: "PHI2_MOULTON_CAMPBELL",
                description: "weighted_cluster_mean_m_tilde must declare valid Moulton-Campbell correction (m_tilde > 1)".into(),
            });
        }
        if !um.threshold_exceedance_icc.contains("rho_I")
            || icc_lower.contains("unadjusted")
            || !parsed_rho_i.is_some_and(|r| r > 0.0 && r <= 1.0)
        {
            v.push(ContractViolation {
                gate_code: "PHI2_EXCEEDANCE_ICC",
                description: "threshold_exceedance_icc must declare valid rho_I(p) in (0, 1]"
                    .into(),
            });
        }
        let n_eff_coherent = match (parsed_m_tilde, parsed_rho_i) {
            (Some(m_tilde), Some(rho_i)) if m_tilde > 1.0 && rho_i > 0.0 && rho_i <= 1.0 => {
                let expected_deff = 1.0 + (m_tilde - 1.0).max(0.0) * rho_i;
                let expected_n_eff = (um.nominal_sample_size_n as f64) / expected_deff;
                ((um.effective_sample_size_n_eff as f64) - expected_n_eff).abs() <= 1.5
            }
            _ => false,
        };
        if um.effective_sample_size_n_eff == 0
            || um.effective_sample_size_n_eff >= um.nominal_sample_size_n
            || !n_eff_coherent
            || !um.unit_margin_of_error.contains("+/-")
        {
            v.push(ContractViolation {
                gate_code: "PHI2_N_EFF",
                description: format!(
                    "effective_sample_size_n_eff ({}) invalid relative to nominal n ({}) and DEFF_exceedance, or unit_margin_of_error omits '+/-'",
                    um.effective_sample_size_n_eff, um.nominal_sample_size_n
                ),
            });
        }

        // Invariant Φ3: Rubin missingness mechanism + SHA-256 pre-registration + Pearl's Ladder
        if MissingnessMechanism::from_prefix(&self.data_and_audit_contract.missingness_mechanism)
            .is_none()
        {
            v.push(ContractViolation {
                gate_code: "PHI3_MISSINGNESS_MECHANISM",
                description: "missingness_mechanism must start with MCAR, MAR, or MNAR".into(),
            });
        }
        if !is_valid_sha256_hex(&self.data_and_audit_contract.preregistration_hash) {
            v.push(ContractViolation {
                gate_code: "PHI3_PREREGISTRATION_HASH",
                description: "preregistration_hash must be a 64-character hexadecimal SHA-256"
                    .into(),
            });
        }
        let cl = &self.epistemological_header.causal_ladder;
        let rung = cl.pearl_rung.as_str();
        let id_strat = cl.identification_strategy.to_ascii_lowercase();
        let untest = cl.untestable_assumption.to_ascii_lowercase();
        let rung_ok = if !q.altered_action.trim().is_empty() {
            matches!(rung, "2-Act" | "3-Imagine")
        } else {
            matches!(rung, "1-See" | "2-Act" | "3-Imagine")
        };
        let mb_ok = id_strat.contains("markov blanket")
            && !id_strat.contains("not mapped")
            && !id_strat.contains("unmapped");
        let collider_ok = (id_strat.contains("without collider")
            || id_strat.contains("no collider")
            || id_strat.contains("free of collider"))
            && !id_strat.contains("conditioned on collider")
            && !id_strat.contains("without collider audit");
        let lambda_ok = untest.contains("lambda*");
        if !rung_ok || !mb_ok || !collider_ok || !lambda_ok {
            v.push(ContractViolation {
                gate_code: "PHI3_PEARL_LADDER",
                description: "causal_ladder requires '2-Act' or '3-Imagine' for interventional decisions, mapped Markov blanket without collider, and Lambda* sensitivity bound".into(),
            });
        }

        // Variance budget and Info-Gap + Sniedovich safeguard
        let vb = &self.uncertainty_budget_and_authorship.variance_budget;
        let sum_var = vb.inputs + vb.parameters + vb.covariance;
        let gp_lower = vb.structural_discrepancy_gp.to_ascii_lowercase();
        if !vb.inputs.is_finite()
            || !vb.parameters.is_finite()
            || !vb.covariance.is_finite()
            || vb.inputs < 0.0
            || vb.parameters < 0.0
            || vb.covariance < 0.0
            || (sum_var - 1.0).abs() > 1e-6
            || (!gp_lower.contains("gp") && !gp_lower.contains("gaussian process"))
        {
            v.push(ContractViolation {
                gate_code: "VARIANCE_BUDGET",
                description: format!("variance_budget sums to {sum_var:.4} (must equal 1.0) and structural_discrepancy_gp must declare a Gaussian Process (GP)"),
            });
        }
        let dom = &self.uncertainty_budget_and_authorship.dominant_event;
        if !dom.probability_p.is_finite()
            || !dom.base_rate_floor_epsilon.is_finite()
            || !respects_cromwell(dom.probability_p, dom.base_rate_floor_epsilon)
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_CROMWELL",
                description: format!(
                    "dominant_event.probability_p ({}) violates Cromwell interval [{}, {}]",
                    dom.probability_p,
                    dom.base_rate_floor_epsilon,
                    1.0 - dom.base_rate_floor_epsilon
                ),
            });
        }
        if !dom.cost_share.is_finite() || dom.cost_share < 0.50 || dom.cost_share > 1.0 {
            v.push(ContractViolation {
                gate_code: "DOMINANT_EVENT_COST_SHARE",
                description: "dominant_event.cost_share must be finite and in [0.50, 1.0]".into(),
            });
        }
        let ig = &self.uncertainty_budget_and_authorship.info_gap_robustness;
        let alpha_hat_lower = ig.robustness_horizon_alpha_hat.to_ascii_lowercase();
        let safeguard_lower = ig.global_maximin_safeguard.to_ascii_lowercase();
        let alpha_hat_val = extract_number_after(&alpha_hat_lower, "alpha_hat");
        if ig.critical_requirement_rc.trim().is_empty()
            || !alpha_hat_val.is_some_and(|a| a > 0.0)
            || !safeguard_lower.contains("reversible")
            || safeguard_lower.contains("irreversible")
            || safeguard_lower.contains("no-man's land")
            || safeguard_lower.contains("unbounded")
            || safeguard_lower.contains("-999")
        {
            v.push(ContractViolation {
                gate_code: "INFO_GAP_SNIEDOVICH",
                description: "info_gap_robustness requires alpha_hat > 0 and global_maximin_safeguard with reversible protocol outside No-Man's Land".into(),
            });
        }

        // Independent cross-authorship (Filter 4)
        let auth = &self.uncertainty_budget_and_authorship.field_authorship;
        if auth.null_model.trim().is_empty()
            || auth.decision_rule.trim().is_empty()
            || auth.adversarial_reviewer.trim().is_empty()
            || auth.null_model.trim() == auth.decision_rule.trim()
            || auth.adversarial_reviewer.trim() == auth.null_model.trim()
            || auth.adversarial_reviewer.trim() == auth.decision_rule.trim()
        {
            v.push(ContractViolation {
                gate_code: "FILTER4_CROSS_AUTHORSHIP",
                description:
                    "null_model, decision_rule, and adversarial_reviewer must have distinct authors"
                        .into(),
            });
        }

        // Prohibition 4: Cromwell, attempt counter K >= 1, e-process E_t, Murphy-CORP decomposition, and findings
        let dr = &self.decision_rule_and_scope;
        if !dr.event_probability_p.is_finite()
            || !dr.base_rate_floor_epsilon.is_finite()
            || !respects_cromwell(dr.event_probability_p, dr.base_rate_floor_epsilon)
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_CROMWELL",
                description: format!(
                    "event_probability_p ({}) violates Cromwell interval [{}, {}]",
                    dr.event_probability_p,
                    dr.base_rate_floor_epsilon,
                    1.0 - dr.base_rate_floor_epsilon
                ),
            });
        }
        if self.data_and_audit_contract.attempt_counter_k < 1 {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_COUNTER_K",
                description: "attempt_counter_K must be >= 1".into(),
            });
        }
        if !validate_e_process_string(
            &self.data_and_audit_contract.e_process_multiplicity,
            self.data_and_audit_contract.attempt_counter_k,
        ) {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_E_PROCESS",
                description:
                    "e_process_multiplicity must declare finite E_t with E_t / K >= 1/alpha >= 20.0"
                        .into(),
            });
        }
        let mc = &self.data_and_audit_contract.murphy_corp_decomposition;
        let bs_reconstructed =
            mc.miscalibration_rel_mcb - mc.resolution_res_dsc + mc.base_uncertainty_unc0;
        if !mc.total_score_brier_or_crps.is_finite()
            || !mc.miscalibration_rel_mcb.is_finite()
            || !mc.resolution_res_dsc.is_finite()
            || !mc.base_uncertainty_unc0.is_finite()
            || mc.miscalibration_rel_mcb < 0.0
            || mc.resolution_res_dsc <= 0.0
            || mc.base_uncertainty_unc0 <= 0.0
            || mc.miscalibration_rel_mcb >= mc.resolution_res_dsc
            || (mc.total_score_brier_or_crps - bs_reconstructed).abs() > 1e-4
        {
            v.push(ContractViolation {
                gate_code: "LENS5_MURPHY_CORP",
                description: format!(
                    "Murphy-CORP identity BS = REL - RES + UNC0 (with RES > REL >= 0) fails: {} vs {}",
                    mc.total_score_brier_or_crps, bs_reconstructed
                ),
            });
        }
        let fnd = &self.data_and_audit_contract.findings;
        if fnd.used.saturating_add(fnd.unused.len()) != fnd.registered
            || fnd.unused.iter().any(|s| s.trim().is_empty())
        {
            v.push(ContractViolation {
                gate_code: "CONTRACT_FINDINGS",
                description: "findings accounting (used + unused == registered) does not close"
                    .into(),
            });
        }

        // Lens 5 and Invariant Φ4: Economic boundary p* = C / (eta * L), Chouldechova, and exit clause
        if !dr.protection_cost_c.is_finite()
            || !dr.event_loss_l.is_finite()
            || !dr.mitigation_efficacy_eta.is_finite()
            || !dr.critical_threshold_p_star.is_finite()
            || dr.protection_cost_c <= 0.0
            || dr.event_loss_l <= 0.0
            || dr.mitigation_efficacy_eta <= 0.0
            || dr.mitigation_efficacy_eta > 1.0
        {
            v.push(ContractViolation {
                gate_code: "LENS5_COST_LOSS_PARAMETERS",
                description: "finite C > 0, L > 0, and eta in (0, 1] are mandatory".into(),
            });
        } else {
            let expected_p_star =
                dr.protection_cost_c / (dr.mitigation_efficacy_eta * dr.event_loss_l);
            if (dr.critical_threshold_p_star - expected_p_star).abs() > 1e-3 {
                v.push(ContractViolation {
                    gate_code: "LENS5_THRESHOLD_P_STAR",
                    description: format!(
                        "critical_threshold_p_star ({:.5}) diverges from C/(eta*L) = {:.5}",
                        dr.critical_threshold_p_star, expected_p_star
                    ),
                });
            }
            let expected_act = if dr.event_probability_p > expected_p_star {
                "protect"
            } else {
                "do_not_protect"
            };
            if dr.declared_act.trim() != expected_act {
                v.push(ContractViolation {
                    gate_code: "LENS5_INCOHERENT_ACT",
                    description: format!(
                        "declared_act '{}' incoherent with p={} vs p*={:.5} (expected '{}')",
                        dr.declared_act, dr.event_probability_p, expected_p_star, expected_act
                    ),
                });
            }
        }
        let wp = dr.who_pays_for_error.to_ascii_lowercase();
        let chouldechova_ratio = extract_number_after(&wp, "fpr_a/fpr_b");
        if !wp.contains("chouldechova")
            || !wp.contains("fpr")
            || !wp.contains("ppv")
            || wp.contains("not audited")
            || wp.contains("equally fair")
            || !chouldechova_ratio.is_some_and(|r| (r - 1.0).abs() > 1e-3)
        {
            v.push(ContractViolation {
                gate_code: "PHI4_CHOULDECHOVA",
                description:
                    "who_pays_for_error must algebraically audit Chouldechova's trilemma (FPR, PPV, and FPR_A/FPR_B != 1)".into(),
            });
        }
        let scope_lower = dr.scope_statement.to_ascii_lowercase();
        let has_scope_exclusion =
            scope_lower.contains("does not apply") || scope_lower.contains("excluded");
        let has_numeric_exit = dr.exit_clause.chars().any(|c| c.is_ascii_digit());
        if dr.scope_statement.trim().is_empty()
            || !has_scope_exclusion
            || dr.exit_clause.trim().is_empty()
            || !has_numeric_exit
        {
            v.push(ContractViolation {
                gate_code: "PHI4_SCOPE_AND_EXIT",
                description: "scope_statement (with explicit non-applicability boundary) and numeric exit_clause are mandatory".into(),
            });
        }

        // Invariant Φ5: Question resolution vs. independent unit
        if !verify_resolution_phi5(&um.independent_unit, &q.altered_action) {
            v.push(ContractViolation {
                gate_code: "PHI5_RESOLUTION",
                description:
                    "altered_action uses coarse aggregate granularity incompatible with independent_unit"
                        .into(),
            });
        }

        if v.is_empty() {
            Ok(())
        } else {
            Err(v)
        }
    }

    /// Serializes the Epistemological Card into the structured YAML format of Contract v1.0 (§2.3).
    pub fn to_yaml(&self) -> String {
        let unused_yaml = if self.data_and_audit_contract.findings.unused.is_empty() {
            "      []".to_string()
        } else {
            self.data_and_audit_contract
                .findings
                .unused
                .iter()
                .map(|item| format!("      - \"{}\"", escape_yaml_str(item)))
                .collect::<Vec<_>>()
                .join("\n")
        };

        format!(
            r#"id: "{id}"
contract_version: "{version}"
layer: "{layer}"
uncertainty: "{uncertainty}"

decision_maker_question:
  who_decides: "{who_decides}"
  altered_action: "{altered_action}"
  question: "{question}"

epistemological_header:
  generating_process: "{generating_process}"
  nonlinearity: "{nonlinearity}"
  unit_and_margin:
    independent_unit: "{independent_unit}"
    nominal_sample_size_n: {nominal_sample_size_n}
    weighted_cluster_mean_m_tilde: "{weighted_cluster_mean_m_tilde}"
    threshold_exceedance_icc: "{threshold_exceedance_icc}"
    effective_sample_size_n_eff: {effective_sample_size_n_eff}
    unit_margin_of_error: "{unit_margin_of_error}"
  noise_physics:
    stochastic_regime: "{stochastic_regime}"
    absorbing_states: "{absorbing_states}"
    bridge_L3_L1_performativity:
      agent_type: "{agent_type}"
      wasserstein_sensitivity_epsilon: {wasserstein_sensitivity_epsilon}
      rrm_convergence_condition: "{rrm_convergence_condition}"
      stability_optimality_divergence: "{stability_optimality_divergence}"
  causal_ladder:
    pearl_rung: "{pearl_rung}"
    identification_strategy: "{identification_strategy}"
    untestable_assumption: "{untestable_assumption}"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: {inputs:.2}
    parameters: {parameters:.2}
    covariance: {covariance:.2}
    structural_discrepancy_gp: "{structural_discrepancy_gp}"
  dominant_event:
    description: "{ev_description}"
    probability_p: {ev_prob}
    base_rate_floor_epsilon: {ev_eps}
    cost_share: {ev_share:.2}
  info_gap_robustness:
    critical_requirement_rc: "{critical_requirement_rc}"
    robustness_horizon_alpha_hat: "{robustness_horizon_alpha_hat}"
    opportuneness_horizon_beta_hat: "{opportuneness_horizon_beta_hat}"
    global_maximin_safeguard: "{global_maximin_safeguard}"
  field_authorship:
    null_model: "{auth_null}"
    decision_rule: "{auth_decision}"
    adversarial_reviewer: "{auth_reviewer}"

data_and_audit_contract:
  missingness_mechanism: "{missingness_mechanism}"
  seed_and_environment: "{seed_and_environment}"
  null_model: "{null_model}"
  preregistration_hash: "{preregistration_hash}"
  attempt_counter_K: {attempt_counter_k}
  e_process_multiplicity: "{e_process_multiplicity}"
  murphy_corp_decomposition:
    total_score_brier_or_crps: {total_score:.3}
    miscalibration_REL_MCB: {miscalibration:.3}
    resolution_RES_DSC: {resolution:.3}
    base_uncertainty_UNC0: {base_uncertainty:.3}
  findings:
    registered: {fnd_reg}
    used: {fnd_used}
    unused:
{unused_yaml}

decision_rule_and_scope:
  event_probability_p: {dr_p}
  base_rate_floor_epsilon: {dr_eps}
  protection_cost_C: {dr_c:.1}
  event_loss_L: {dr_l:.1}
  mitigation_efficacy_eta: {dr_eta:.1}
  critical_threshold_p_star: {dr_p_star:.5}
  declared_act: "{declared_act}"
  scope_statement: "{scope_statement}"
  who_pays_for_error: "{who_pays_for_error}"
  exit_clause: "{exit_clause}"
"#,
            id = escape_yaml_str(&self.id),
            version = escape_yaml_str(&self.contract_version),
            layer = self.layer.code(),
            uncertainty = self.uncertainty.code(),
            who_decides = escape_yaml_str(&self.decision_maker_question.who_decides),
            altered_action = escape_yaml_str(&self.decision_maker_question.altered_action),
            question = escape_yaml_str(&self.decision_maker_question.question),
            generating_process = escape_yaml_str(&self.epistemological_header.generating_process),
            nonlinearity = escape_yaml_str(&self.epistemological_header.nonlinearity),
            independent_unit =
                escape_yaml_str(&self.epistemological_header.unit_and_margin.independent_unit),
            nominal_sample_size_n = self
                .epistemological_header
                .unit_and_margin
                .nominal_sample_size_n,
            weighted_cluster_mean_m_tilde = escape_yaml_str(
                &self
                    .epistemological_header
                    .unit_and_margin
                    .weighted_cluster_mean_m_tilde
            ),
            threshold_exceedance_icc = escape_yaml_str(
                &self
                    .epistemological_header
                    .unit_and_margin
                    .threshold_exceedance_icc
            ),
            effective_sample_size_n_eff = self
                .epistemological_header
                .unit_and_margin
                .effective_sample_size_n_eff,
            unit_margin_of_error = escape_yaml_str(
                &self
                    .epistemological_header
                    .unit_and_margin
                    .unit_margin_of_error
            ),
            stochastic_regime =
                escape_yaml_str(&self.epistemological_header.noise_physics.stochastic_regime),
            absorbing_states =
                escape_yaml_str(&self.epistemological_header.noise_physics.absorbing_states),
            agent_type = escape_yaml_str(
                &self
                    .epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .agent_type
            ),
            wasserstein_sensitivity_epsilon = self
                .epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .wasserstein_sensitivity_epsilon,
            rrm_convergence_condition = escape_yaml_str(
                &self
                    .epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .rrm_convergence_condition
            ),
            stability_optimality_divergence = escape_yaml_str(
                &self
                    .epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .stability_optimality_divergence
            ),
            pearl_rung = escape_yaml_str(&self.epistemological_header.causal_ladder.pearl_rung),
            identification_strategy = escape_yaml_str(
                &self
                    .epistemological_header
                    .causal_ladder
                    .identification_strategy
            ),
            untestable_assumption = escape_yaml_str(
                &self
                    .epistemological_header
                    .causal_ladder
                    .untestable_assumption
            ),
            inputs = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .inputs,
            parameters = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .parameters,
            covariance = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .covariance,
            structural_discrepancy_gp = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .variance_budget
                    .structural_discrepancy_gp
            ),
            ev_description = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .dominant_event
                    .description
            ),
            ev_prob = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .probability_p,
            ev_eps = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .base_rate_floor_epsilon,
            ev_share = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .cost_share,
            critical_requirement_rc = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .critical_requirement_rc
            ),
            robustness_horizon_alpha_hat = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .robustness_horizon_alpha_hat
            ),
            opportuneness_horizon_beta_hat = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .opportuneness_horizon_beta_hat
            ),
            global_maximin_safeguard = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .global_maximin_safeguard
            ),
            auth_null = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .field_authorship
                    .null_model
            ),
            auth_decision = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .field_authorship
                    .decision_rule
            ),
            auth_reviewer = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .field_authorship
                    .adversarial_reviewer
            ),
            missingness_mechanism =
                escape_yaml_str(&self.data_and_audit_contract.missingness_mechanism),
            seed_and_environment =
                escape_yaml_str(&self.data_and_audit_contract.seed_and_environment),
            null_model = escape_yaml_str(&self.data_and_audit_contract.null_model),
            preregistration_hash =
                escape_yaml_str(&self.data_and_audit_contract.preregistration_hash),
            attempt_counter_k = self.data_and_audit_contract.attempt_counter_k,
            e_process_multiplicity =
                escape_yaml_str(&self.data_and_audit_contract.e_process_multiplicity),
            total_score = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .total_score_brier_or_crps,
            miscalibration = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .miscalibration_rel_mcb,
            resolution = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .resolution_res_dsc,
            base_uncertainty = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .base_uncertainty_unc0,
            fnd_reg = self.data_and_audit_contract.findings.registered,
            fnd_used = self.data_and_audit_contract.findings.used,
            unused_yaml = unused_yaml,
            dr_p = self.decision_rule_and_scope.event_probability_p,
            dr_eps = self.decision_rule_and_scope.base_rate_floor_epsilon,
            dr_c = self.decision_rule_and_scope.protection_cost_c,
            dr_l = self.decision_rule_and_scope.event_loss_l,
            dr_eta = self.decision_rule_and_scope.mitigation_efficacy_eta,
            dr_p_star = self.decision_rule_and_scope.critical_threshold_p_star,
            declared_act = escape_yaml_str(&self.decision_rule_and_scope.declared_act),
            scope_statement = escape_yaml_str(&self.decision_rule_and_scope.scope_statement),
            who_pays_for_error = escape_yaml_str(&self.decision_rule_and_scope.who_pays_for_error),
            exit_clause = escape_yaml_str(&self.decision_rule_and_scope.exit_clause),
        )
    }

    /// Parses an Epistemological Card from the YAML template of Contract v1.0.
    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        let mut stack: Vec<(usize, String)> = Vec::new();
        let mut unused: Vec<String> = Vec::new();

        for raw_line in yaml.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let indent = raw_line.len() - raw_line.trim_start().len();
            if let Some(rest) = trimmed.strip_prefix("- ") {
                while let Some(&(last_indent, _)) = stack.last() {
                    if indent < last_indent {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                let current_path = stack
                    .iter()
                    .map(|(_, s)| s.as_str())
                    .collect::<Vec<_>>()
                    .join(".");
                if current_path == "data_and_audit_contract.findings.unused" {
                    unused.push(unquote(rest));
                }
                continue;
            }
            while let Some(&(last_indent, _)) = stack.last() {
                if indent <= last_indent {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some((k, v)) = trimmed.split_once(':') {
                let key = k.trim().to_string();
                let val = v.trim();
                if val.is_empty() {
                    stack.push((indent, key));
                } else if val == "[]" {
                    // Explicit empty inline sequence
                } else {
                    let mut full_path = stack
                        .iter()
                        .map(|(_, s)| s.as_str())
                        .collect::<Vec<_>>()
                        .join(".");
                    if !full_path.is_empty() {
                        full_path.push('.');
                    }
                    full_path.push_str(&key);
                    map.insert(full_path, unquote(val));
                }
            }
        }

        let req = |k: &str| -> Result<String, String> {
            map.get(k)
                .cloned()
                .ok_or_else(|| format!("missing mandatory YAML field: '{k}'"))
        };
        let req_f64 = |k: &str| -> Result<f64, String> {
            let v = req(k)?
                .parse::<f64>()
                .map_err(|e| format!("invalid numeric value in '{k}': {e}"))?;
            if !v.is_finite() {
                return Err(format!("non-finite numeric value in '{k}': {v}"));
            }
            Ok(v)
        };
        let req_usize = |k: &str| -> Result<usize, String> {
            req(k)?
                .parse::<usize>()
                .map_err(|e| format!("invalid integer in '{k}': {e}"))
        };

        let layer = Layer::from_code(&req("layer")?)
            .ok_or_else(|| "invalid layer (use L1, L2, or L3)".to_string())?;
        let uncertainty = UncertaintyClass::from_code(&req("uncertainty")?)
            .ok_or_else(|| "invalid uncertainty class".to_string())?;

        Ok(Self {
            id: req("id")?,
            contract_version: req("contract_version")?,
            layer,
            uncertainty,
            decision_maker_question: DecisionMakerQuestion {
                who_decides: req("decision_maker_question.who_decides")?,
                altered_action: req("decision_maker_question.altered_action")?,
                question: req("decision_maker_question.question")?,
            },
            epistemological_header: EpistemologicalHeader {
                generating_process: req("epistemological_header.generating_process")?,
                nonlinearity: req("epistemological_header.nonlinearity")?,
                unit_and_margin: UnitAndMargin {
                    independent_unit: req(
                        "epistemological_header.unit_and_margin.independent_unit",
                    )?,
                    nominal_sample_size_n: req_usize(
                        "epistemological_header.unit_and_margin.nominal_sample_size_n",
                    )?,
                    weighted_cluster_mean_m_tilde: req(
                        "epistemological_header.unit_and_margin.weighted_cluster_mean_m_tilde",
                    )?,
                    threshold_exceedance_icc: req(
                        "epistemological_header.unit_and_margin.threshold_exceedance_icc",
                    )?,
                    effective_sample_size_n_eff: req_usize(
                        "epistemological_header.unit_and_margin.effective_sample_size_n_eff",
                    )?,
                    unit_margin_of_error: req(
                        "epistemological_header.unit_and_margin.unit_margin_of_error",
                    )?,
                },
                noise_physics: NoisePhysics {
                    stochastic_regime: req(
                        "epistemological_header.noise_physics.stochastic_regime",
                    )?,
                    absorbing_states: req("epistemological_header.noise_physics.absorbing_states")?,
                    bridge_l3_l1_performativity: BridgePerformativity {
                        agent_type: req(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.agent_type",
                        )?,
                        wasserstein_sensitivity_epsilon: req_f64(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.wasserstein_sensitivity_epsilon",
                        )?,
                        rrm_convergence_condition: req(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.rrm_convergence_condition",
                        )?,
                        stability_optimality_divergence: req(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.stability_optimality_divergence",
                        )?,
                    },
                },
                causal_ladder: CausalLadder {
                    pearl_rung: req("epistemological_header.causal_ladder.pearl_rung")?,
                    identification_strategy: req(
                        "epistemological_header.causal_ladder.identification_strategy",
                    )?,
                    untestable_assumption: req(
                        "epistemological_header.causal_ladder.untestable_assumption",
                    )?,
                },
            },
            uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship {
                variance_budget: VarianceBudget {
                    inputs: req_f64("uncertainty_budget_and_authorship.variance_budget.inputs")?,
                    parameters: req_f64(
                        "uncertainty_budget_and_authorship.variance_budget.parameters",
                    )?,
                    covariance: req_f64(
                        "uncertainty_budget_and_authorship.variance_budget.covariance",
                    )?,
                    structural_discrepancy_gp: req(
                        "uncertainty_budget_and_authorship.variance_budget.structural_discrepancy_gp",
                    )?,
                },
                dominant_event: DominantEvent {
                    description: req(
                        "uncertainty_budget_and_authorship.dominant_event.description",
                    )?,
                    probability_p: req_f64(
                        "uncertainty_budget_and_authorship.dominant_event.probability_p",
                    )?,
                    base_rate_floor_epsilon: req_f64(
                        "uncertainty_budget_and_authorship.dominant_event.base_rate_floor_epsilon",
                    )?,
                    cost_share: req_f64(
                        "uncertainty_budget_and_authorship.dominant_event.cost_share",
                    )?,
                },
                info_gap_robustness: InfoGapRobustness {
                    critical_requirement_rc: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.critical_requirement_rc",
                    )?,
                    robustness_horizon_alpha_hat: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.robustness_horizon_alpha_hat",
                    )?,
                    opportuneness_horizon_beta_hat: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.opportuneness_horizon_beta_hat",
                    )?,
                    global_maximin_safeguard: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.global_maximin_safeguard",
                    )?,
                },
                field_authorship: FieldAuthorship {
                    null_model: req("uncertainty_budget_and_authorship.field_authorship.null_model")?,
                    decision_rule: req(
                        "uncertainty_budget_and_authorship.field_authorship.decision_rule",
                    )?,
                    adversarial_reviewer: req(
                        "uncertainty_budget_and_authorship.field_authorship.adversarial_reviewer",
                    )?,
                },
            },
            data_and_audit_contract: DataAndAuditContract {
                missingness_mechanism: req("data_and_audit_contract.missingness_mechanism")?,
                seed_and_environment: req("data_and_audit_contract.seed_and_environment")?,
                null_model: req("data_and_audit_contract.null_model")?,
                preregistration_hash: req("data_and_audit_contract.preregistration_hash")?,
                attempt_counter_k: req_usize("data_and_audit_contract.attempt_counter_K")?,
                e_process_multiplicity: req("data_and_audit_contract.e_process_multiplicity")?,
                murphy_corp_decomposition: MurphyCorpDecomposition {
                    total_score_brier_or_crps: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.total_score_brier_or_crps",
                    )?,
                    miscalibration_rel_mcb: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.miscalibration_REL_MCB",
                    )?,
                    resolution_res_dsc: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.resolution_RES_DSC",
                    )?,
                    base_uncertainty_unc0: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.base_uncertainty_UNC0",
                    )?,
                },
                findings: FindingsRegistry {
                    registered: req_usize("data_and_audit_contract.findings.registered")?,
                    used: req_usize("data_and_audit_contract.findings.used")?,
                    unused,
                },
            },
            decision_rule_and_scope: DecisionRuleAndScope {
                event_probability_p: req_f64("decision_rule_and_scope.event_probability_p")?,
                base_rate_floor_epsilon: req_f64(
                    "decision_rule_and_scope.base_rate_floor_epsilon",
                )?,
                protection_cost_c: req_f64("decision_rule_and_scope.protection_cost_C")?,
                event_loss_l: req_f64("decision_rule_and_scope.event_loss_L")?,
                mitigation_efficacy_eta: req_f64(
                    "decision_rule_and_scope.mitigation_efficacy_eta",
                )?,
                critical_threshold_p_star: req_f64(
                    "decision_rule_and_scope.critical_threshold_p_star",
                )?,
                declared_act: req("decision_rule_and_scope.declared_act")?,
                scope_statement: req("decision_rule_and_scope.scope_statement")?,
                who_pays_for_error: req("decision_rule_and_scope.who_pays_for_error")?,
                exit_clause: req("decision_rule_and_scope.exit_clause")?,
            },
        })
    }
}

fn escape_yaml_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        let inner = &t[1..t.len() - 1];
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(ch);
            }
        }
        out
    } else {
        t.to_string()
    }
}

fn extract_number_after(text_lower: &str, marker_lower: &str) -> Option<f64> {
    let idx = text_lower.find(marker_lower)?;
    let rest = &text_lower[idx + marker_lower.len()..];
    let mut clause_end = rest.find([',', ';']).unwrap_or(rest.len());
    if let Some(arrow_idx) = rest.find("->") {
        clause_end = clause_end.min(arrow_idx);
    }
    let clause = &rest[..clause_end];
    let after_marker = if marker_lower.trim_end().ends_with('=') {
        clause
    } else if let Some(eq_idx) = clause.find('=') {
        &clause[eq_idx + 1..]
    } else {
        clause
    };
    let trimmed = after_marker.trim_start_matches([' ', '\t', ':']);
    let bytes = trimmed.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut has_digits = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        has_digits = true;
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            has_digits = true;
            i += 1;
        }
    }
    if !has_digits {
        return None;
    }
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let mut exp_i = i + 1;
        if exp_i < bytes.len() && (bytes[exp_i] == b'+' || bytes[exp_i] == b'-') {
            exp_i += 1;
        }
        let exp_digits_start = exp_i;
        while exp_i < bytes.len() && bytes[exp_i].is_ascii_digit() {
            exp_i += 1;
        }
        if exp_i > exp_digits_start {
            i = exp_i;
        }
    }
    let val = trimmed[..i].parse::<f64>().ok()?;
    val.is_finite().then_some(val)
}

fn parse_m_tilde(m_tilde_lower: &str) -> Option<f64> {
    let prefix = if let Some(idx) = m_tilde_lower.find("(moulton") {
        &m_tilde_lower[..idx]
    } else {
        m_tilde_lower
    };
    if let Some(last_eq) = prefix.rfind('=') {
        if let Some(v) = extract_number_after(&prefix[last_eq..], "=") {
            return Some(v);
        }
    }
    extract_number_after(m_tilde_lower, "m_tilde")
}

fn parse_seed_from_env(seed_env: &str) -> Option<u64> {
    for part in seed_env.split(';') {
        let t = part.trim();
        if let Some(rest) = t.strip_prefix("seed=") {
            return rest.trim().parse::<u64>().ok();
        }
    }
    None
}

fn validate_rrm_contraction_string(cond: &str, declared_eps: f64) -> bool {
    if !declared_eps.is_finite() || declared_eps < 0.0 || !cond.contains("< 1") {
        return false;
    }
    let before_lt = cond.split("< 1").next().unwrap_or("");
    let nums: Vec<f64> = before_lt
        .split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '+' || c == '-'))
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite())
        .collect();
    if nums.len() >= 4 {
        let eps = nums[0];
        let beta = nums[1];
        let gamma = nums[2];
        let ratio = nums[3];
        (eps - declared_eps).abs() <= 1e-4
            && gamma > 0.0
            && beta >= gamma
            && ((eps * beta / gamma) - ratio).abs() <= 1e-2
            && (0.0..1.0).contains(&ratio)
    } else {
        false
    }
}

fn validate_e_process_string(e_str: &str, k: usize) -> bool {
    if k < 1 {
        return false;
    }
    let lower = e_str.to_ascii_lowercase();
    let Some(e_val) = extract_number_after(&lower, "e_") else {
        return false;
    };
    let threshold = extract_number_after(&lower, "1/alpha")
        .or_else(|| extract_number_after(&lower, "threshold"))
        .unwrap_or(20.0);
    e_val > 0.0 && threshold >= 20.0 && (e_val / (k as f64)) >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_card_passes_audit_and_yaml_roundtrip() {
        let c = EpistemologicalCard::canonical_example();
        assert_eq!(c.audit(), Ok(()));
        let yaml = c.to_yaml();
        let parsed = EpistemologicalCard::from_yaml(&yaml).expect("YAML parse must succeed");
        assert_eq!(parsed.audit(), Ok(()));
        assert_eq!(parsed, c);
        assert_eq!(parsed.contract_version, "1.0");
    }

    #[test]
    fn card_rejects_cromwell_murphy_and_authorship_violations() {
        let mut c = EpistemologicalCard::canonical_example();
        c.decision_rule_and_scope.event_probability_p = 0.0; // Violates Cromwell
        c.uncertainty_budget_and_authorship
            .field_authorship
            .adversarial_reviewer = "inference".into(); // Violates Filter 4
        c.data_and_audit_contract
            .murphy_corp_decomposition
            .total_score_brier_or_crps = 0.999; // Violates Murphy-CORP
        let errs = c.audit().expect_err("must reject defective card");
        assert!(errs.iter().any(|e| e.gate_code == "PROHIBITION_4_CROMWELL"));
        assert!(errs
            .iter()
            .any(|e| e.gate_code == "FILTER4_CROSS_AUTHORSHIP"));
        assert!(errs.iter().any(|e| e.gate_code == "LENS5_MURPHY_CORP"));
    }

    #[test]
    fn card_audit_enforces_numeric_values_for_seed_jensen_rrm_eprocess_and_nan() {
        let mut c = EpistemologicalCard::canonical_example();
        c.data_and_audit_contract.seed_and_environment = "seed=42; rustc=1.97".into();
        c.epistemological_header.nonlinearity =
            "f(E[X])=1420.0, E[f(X)]=1890.0, Jensen gap Delta_J=+999.0, median=1510.0, q05=820.0, q95=4120.0, CRPS=215.4".into();
        c.epistemological_header
            .noise_physics
            .bridge_l3_l1_performativity
            .rrm_convergence_condition =
            "eps * beta / gamma = 0.80 * 1.50 / 1.00 = 1.20 < 1".into();
        c.data_and_audit_contract.e_process_multiplicity =
            "Cumulative e-process E_1 = 5.0 (threshold 1/alpha = 20.0 for alpha = 0.05)".into();
        c.uncertainty_budget_and_authorship.variance_budget.inputs = f64::NAN;

        let errs = c.audit().expect_err("must reject numeric violations");
        assert!(errs.iter().any(|e| e.gate_code == "PHI1_SEED"));
        assert!(errs
            .iter()
            .any(|e| e.gate_code == "PROHIBITION_2_LONE_MEAN"));
        assert!(errs
            .iter()
            .any(|e| e.gate_code == "PROHIBITION_3_RRM_CONTRACTION"));
        assert!(errs
            .iter()
            .any(|e| e.gate_code == "PROHIBITION_4_E_PROCESS"));
        assert!(errs.iter().any(|e| e.gate_code == "VARIANCE_BUDGET"));
    }

    #[test]
    fn yaml_escapes_quotes_colons_backslashes_and_newlines_and_rejects_nan() {
        let mut c = EpistemologicalCard::canonical_example();
        c.decision_maker_question.altered_action =
            "Action with \"quotes\", colon: value, backslash \\ and\nnewline".into();
        let yaml = c.to_yaml();
        let parsed = EpistemologicalCard::from_yaml(&yaml).expect("escaped YAML must parse");
        assert_eq!(parsed, c);

        let nan_yaml = yaml.replace("inputs: 0.45", "inputs: NaN");
        assert!(EpistemologicalCard::from_yaml(&nan_yaml).is_err());
    }

    #[test]
    fn trojan_horse_card_is_rejected_across_all_defective_gates() {
        let trojan_yaml = r#"id: "trojan_horse_adversarial_bypass"
contract_version: "1.0"
layer: "L2"
uncertainty: "epistemic_specification"

decision_maker_question:
  who_decides: "The System / Data Science Team"
  altered_action: "Set a single annual staffing average across all shifts"
  question: "Should we deploy the model?"

epistemological_header:
  generating_process: "None"
  nonlinearity: "f(E[X])=100.0, E[f(X)]=100.0, Jensen gap Delta_J=0.0, median=10.0, q05=10.0, q95=10.001, CRPS=999999.0"
  unit_and_margin:
    independent_unit: "transactional row in a 5-minute window"
    nominal_sample_size_n: 25028
    weighted_cluster_mean_m_tilde: "m_tilde = 1.0 (Moulton-Campbell ignored, CV = 0)"
    threshold_exceedance_icc: "rho_I(0.90) = 0.85 -> DEFF = 1.0 (unadjusted)"
    effective_sample_size_n_eff: 25028
    unit_margin_of_error: "0.0%"
  noise_physics:
    stochastic_regime: "Gaussian i.i.d."
    absorbing_states: "None"
    bridge_L3_L1_performativity:
      agent_type: "performative"
      wasserstein_sensitivity_epsilon: 0.99
      rrm_convergence_condition: "eps * beta / gamma = 50.0 / 1.0 = 0.99 < 1 (oscillatory divergence masked as 3-number string)"
      stability_optimality_divergence: "theta_PS = 0.0 vs theta_PO = 0.00001"
  causal_ladder:
    pearl_rung: "1-See"
    identification_strategy: "Observational correlation conditioned on collider (hospitalized patients only); Markov blanket not mapped"
    untestable_assumption: "None"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: 1.00
    parameters: 0.00
    covariance: 0.00
    structural_discrepancy_gp: "None (delta(x) = 0 assumed)"
  dominant_event:
    description: "Negligible noise event"
    probability_p: 0.50
    base_rate_floor_epsilon: 0.001
    cost_share: 0.01
  info_gap_robustness:
    critical_requirement_rc: "None"
    robustness_horizon_alpha_hat: "alpha_hat = 0.0 (fragile around nominal estimate)"
    opportuneness_horizon_beta_hat: "beta_hat = 999.0"
    global_maximin_safeguard: "No-Man's Land: Wald global Maximin bound = -999999.0 (unbounded ruin, irreversible action)"
  field_authorship:
    null_model: "Author A"
    decision_rule: "Author A ( Decision )"
    adversarial_reviewer: "Author A ( Reviewer )"

data_and_audit_contract:
  missingness_mechanism: "MNAR (70% missing; complete-case analysis used, Manski bounds ignored)"
  seed_and_environment: "seed=2026"
  null_model: "none (skipped)"
  preregistration_hash: "0000000000000000000000000000000000000000000000000000000000000000"
  attempt_counter_K: 50
  e_process_multiplicity: "E_50 = 50.1 (threshold = 1.001)"
  murphy_corp_decomposition:
    total_score_brier_or_crps: 0.500
    miscalibration_REL_MCB: 0.250
    resolution_RES_DSC: 0.000
    base_uncertainty_UNC0: 0.250
  findings:
    registered: 1
    used: 1
    unused:
      []

decision_rule_and_scope:
  event_probability_p: 0.50
  base_rate_floor_epsilon: 0.001
  protection_cost_C: 10.0
  event_loss_L: 100.0
  mitigation_efficacy_eta: 1.0
  critical_threshold_p_star: 0.10000
  declared_act: "protect"
  scope_statement: "Applies universally everywhere forever"
  who_pays_for_error: "Chouldechova not audited; model assumed equally fair on PPV, FPR, and FNR simultaneously"
  exit_clause: "We will review the model at management discretion when convenient"
"#;

        let card = EpistemologicalCard::from_yaml(trojan_yaml).expect("Trojan YAML must parse");
        let violations = card
            .audit()
            .expect_err("Trojan Horse card must be rejected");
        assert!(
            violations.len() >= 10,
            "Expected at least 10 gate violations, got {}: {violations:?}",
            violations.len()
        );
        for expected_gate in [
            "STEP2_DECISION_MAKER",
            "PROHIBITION_2_LONE_MEAN",
            "PROHIBITION_3_RRM_CONTRACTION",
            "PHI1_PHI3_NULL_MODEL",
            "PHI2_PSEUDOREPLICATION",
            "PHI2_MOULTON_CAMPBELL",
            "PHI2_EXCEEDANCE_ICC",
            "PHI2_N_EFF",
            "PHI3_PEARL_LADDER",
            "VARIANCE_BUDGET",
            "DOMINANT_EVENT_COST_SHARE",
            "INFO_GAP_SNIEDOVICH",
            "PROHIBITION_4_E_PROCESS",
            "LENS5_MURPHY_CORP",
            "PHI4_CHOULDECHOVA",
            "PHI4_SCOPE_AND_EXIT",
            "PHI5_RESOLUTION",
        ] {
            assert!(
                violations.iter().any(|v| v.gate_code == expected_gate),
                "Missing expected violation for gate '{expected_gate}' in {violations:?}"
            );
        }
    }
}
