//! Structured Epistemological Card (`EpistemologicalCard`, `contract_version: "1.0"`, §2.3),
//! zero-dependency YAML parser/serializer, and algorithmic validator (`audit()`) enforcing
//! the 4 Prohibitions and the 5 Boundary Invariants ($\Phi_1$ to $\Phi_5$).

use crate::invariants::verify_resolution_phi5;
use crate::layers::{respects_cromwell, Layer, UncertaintyClass};
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
                    weighted_cluster_mean_m_tilde: "m_tilde = m_bar * (1 + CV^2) = 59.59 * 1.992 = 118.71 (Moulton-Campbell)".into(),
                    threshold_exceedance_icc: "rho_I(0.90) = 0.155 -> DEFF_exceedance = 19.25".into(),
                    effective_sample_size_n_eff: 1_300,
                    unit_margin_of_error: "+/- 1.4 percentage points over 1,300 effective shifts".into(),
                },
                noise_physics: NoisePhysics {
                    stochastic_regime: "Non-ergodic multiplicative organ-failure cascade with sub-exponential tail (alpha_Hill = 1.60)".into(),
                    absorbing_states: "Irreversible refractory septic shock / clinical mortality barrier".into(),
                    bridge_l3_l1_performativity: BridgePerformativity {
                        agent_type: "performative".into(),
                        wasserstein_sensitivity_epsilon: 0.18,
                        rrm_convergence_condition: "eps * beta / gamma = 0.18 * 1.50 / 1.00 = 0.27 < 1 (strict contraction)".into(),
                        stability_optimality_divergence: "theta_PS = 1.2195 vs Stackelberg theta_PO = 1.5625 (audited)".into(),
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
        if dm_lower.is_empty()
            || dm_lower == "system"
            || dm_lower == "team"
            || dm_lower == "generic"
            || dm_lower == "analyst"
        {
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

        // Prohibition 2: Ban on the lone mean
        let nl = self
            .epistemological_header
            .nonlinearity
            .to_ascii_lowercase();
        for term in [
            "f(e[x])", "e[f(x)]", "jensen", "median", "q05", "q95", "crps",
        ] {
            if !nl.contains(term) {
                v.push(ContractViolation {
                    gate_code: "PROHIBITION_2_LONE_MEAN",
                    description: format!(
                        "nonlinearity field omits '{}' required by Prohibition 2",
                        term
                    ),
                });
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
        if perf.agent_type == "performative" {
            let div = perf.stability_optimality_divergence.to_ascii_lowercase();
            if !div.contains("theta_ps") || !div.contains("theta_po") {
                v.push(ContractViolation {
                    gate_code: "PROHIBITION_3_PS_VS_PO",
                    description: "performative agent must explicitly report theta_PS and theta_PO"
                        .into(),
                });
            }
        }

        // Invariant Φ1: Simulability (seed=2026 and executable null model)
        let seed_env = &self.data_and_audit_contract.seed_and_environment;
        if !seed_env.contains("seed=") {
            v.push(ContractViolation {
                gate_code: "PHI1_SEED",
                description: "seed_and_environment must declare 'seed=...'".into(),
            });
        }
        if self.data_and_audit_contract.null_model.trim().is_empty() {
            v.push(ContractViolation {
                gate_code: "PHI1_PHI3_NULL_MODEL",
                description: "executable null_model is mandatory".into(),
            });
        }

        // Invariant Φ2: Independent unit, Moulton-Campbell m_tilde, and exceedance n_eff(p)
        let um = &self.epistemological_header.unit_and_margin;
        let u_lower = um.independent_unit.to_ascii_lowercase();
        if u_lower.is_empty() || u_lower == "row" || u_lower == "transaction" {
            v.push(ContractViolation {
                gate_code: "PHI2_PSEUDOREPLICATION",
                description: "independent_unit cannot be the correlated transactional row".into(),
            });
        }
        if um.effective_sample_size_n_eff == 0
            || um.effective_sample_size_n_eff > um.nominal_sample_size_n
        {
            v.push(ContractViolation {
                gate_code: "PHI2_N_EFF",
                description: format!(
                    "effective_sample_size_n_eff ({}) invalid relative to nominal n ({})",
                    um.effective_sample_size_n_eff, um.nominal_sample_size_n
                ),
            });
        }
        if !um.weighted_cluster_mean_m_tilde.contains("CV")
            && !um
                .weighted_cluster_mean_m_tilde
                .to_ascii_lowercase()
                .contains("moulton")
        {
            v.push(ContractViolation {
                gate_code: "PHI2_MOULTON_CAMPBELL",
                description: "weighted_cluster_mean_m_tilde must declare Moulton-Campbell correction (1 + CV^2)".into(),
            });
        }
        if !um.threshold_exceedance_icc.contains("rho_I") {
            v.push(ContractViolation {
                gate_code: "PHI2_EXCEEDANCE_ICC",
                description: "threshold_exceedance_icc must declare rho_I(p)".into(),
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
        let rung = self
            .epistemological_header
            .causal_ladder
            .pearl_rung
            .as_str();
        if !matches!(rung, "1-See" | "2-Act" | "3-Imagine") {
            v.push(ContractViolation {
                gate_code: "PHI3_PEARL_LADDER",
                description: "pearl_rung must be '1-See', '2-Act', or '3-Imagine'".into(),
            });
        }

        // Variance budget and Info-Gap + Sniedovich safeguard
        let vb = &self.uncertainty_budget_and_authorship.variance_budget;
        let sum_var = vb.inputs + vb.parameters + vb.covariance;
        if (sum_var - 1.0).abs() > 1e-6 {
            v.push(ContractViolation {
                gate_code: "VARIANCE_BUDGET",
                description: format!("variance_budget sums to {sum_var:.4}, must equal 1.0"),
            });
        }
        let ig = &self.uncertainty_budget_and_authorship.info_gap_robustness;
        if ig.critical_requirement_rc.trim().is_empty()
            || ig.robustness_horizon_alpha_hat.trim().is_empty()
            || ig.global_maximin_safeguard.trim().is_empty()
        {
            v.push(ContractViolation {
                gate_code: "INFO_GAP_SNIEDOVICH",
                description: "info_gap_robustness and global_maximin_safeguard are mandatory"
                    .into(),
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

        // Prohibition 4: Cromwell, attempt counter K >= 1, Murphy-CORP decomposition, and findings
        let dr = &self.decision_rule_and_scope;
        if !respects_cromwell(dr.event_probability_p, dr.base_rate_floor_epsilon) {
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
        let mc = &self.data_and_audit_contract.murphy_corp_decomposition;
        let bs_reconstructed =
            mc.miscalibration_rel_mcb - mc.resolution_res_dsc + mc.base_uncertainty_unc0;
        if mc.miscalibration_rel_mcb < 0.0
            || mc.resolution_res_dsc < 0.0
            || mc.base_uncertainty_unc0 < 0.0
            || (mc.total_score_brier_or_crps - bs_reconstructed).abs() > 1e-4
        {
            v.push(ContractViolation {
                gate_code: "LENS5_MURPHY_CORP",
                description: format!(
                    "Murphy-CORP identity BS = REL - RES + UNC0 fails: {} vs {}",
                    mc.total_score_brier_or_crps, bs_reconstructed
                ),
            });
        }
        let fnd = &self.data_and_audit_contract.findings;
        if fnd.used + fnd.unused.len() != fnd.registered
            || fnd.unused.iter().any(|s| s.trim().is_empty())
        {
            v.push(ContractViolation {
                gate_code: "CONTRACT_FINDINGS",
                description: "findings accounting (used + unused == registered) does not close"
                    .into(),
            });
        }

        // Lens 5 and Invariant Φ4: Economic boundary p* = C / (eta * L), Chouldechova, and exit clause
        if dr.protection_cost_c <= 0.0
            || dr.event_loss_l <= 0.0
            || dr.mitigation_efficacy_eta <= 0.0
            || dr.mitigation_efficacy_eta > 1.0
        {
            v.push(ContractViolation {
                gate_code: "LENS5_COST_LOSS_PARAMETERS",
                description: "C > 0, L > 0, and eta in (0, 1] are mandatory".into(),
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
        if !wp.contains("chouldechova") && (!wp.contains("fpr") || !wp.contains("ppv")) {
            v.push(ContractViolation {
                gate_code: "PHI4_CHOULDECHOVA",
                description:
                    "who_pays_for_error must audit Chouldechova's trilemma (FPR, FNR, PPV)".into(),
            });
        }
        if dr.scope_statement.trim().is_empty() || dr.exit_clause.trim().is_empty() {
            v.push(ContractViolation {
                gate_code: "PHI4_SCOPE_AND_EXIT",
                description: "scope_statement and numeric exit_clause are mandatory".into(),
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
                .map(|item| format!("      - \"{}\"", item.replace('"', "\\\"")))
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
            id = self.id,
            version = self.contract_version,
            layer = self.layer.code(),
            uncertainty = self.uncertainty.code(),
            who_decides = self.decision_maker_question.who_decides,
            altered_action = self.decision_maker_question.altered_action,
            question = self.decision_maker_question.question,
            generating_process = self.epistemological_header.generating_process,
            nonlinearity = self.epistemological_header.nonlinearity,
            independent_unit = self.epistemological_header.unit_and_margin.independent_unit,
            nominal_sample_size_n = self
                .epistemological_header
                .unit_and_margin
                .nominal_sample_size_n,
            weighted_cluster_mean_m_tilde = self
                .epistemological_header
                .unit_and_margin
                .weighted_cluster_mean_m_tilde,
            threshold_exceedance_icc = self
                .epistemological_header
                .unit_and_margin
                .threshold_exceedance_icc,
            effective_sample_size_n_eff = self
                .epistemological_header
                .unit_and_margin
                .effective_sample_size_n_eff,
            unit_margin_of_error = self
                .epistemological_header
                .unit_and_margin
                .unit_margin_of_error,
            stochastic_regime = self.epistemological_header.noise_physics.stochastic_regime,
            absorbing_states = self.epistemological_header.noise_physics.absorbing_states,
            agent_type = self
                .epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .agent_type,
            wasserstein_sensitivity_epsilon = self
                .epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .wasserstein_sensitivity_epsilon,
            rrm_convergence_condition = self
                .epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .rrm_convergence_condition,
            stability_optimality_divergence = self
                .epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .stability_optimality_divergence,
            pearl_rung = self.epistemological_header.causal_ladder.pearl_rung,
            identification_strategy = self
                .epistemological_header
                .causal_ladder
                .identification_strategy,
            untestable_assumption = self
                .epistemological_header
                .causal_ladder
                .untestable_assumption,
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
            structural_discrepancy_gp = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .structural_discrepancy_gp,
            ev_description = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .description,
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
            critical_requirement_rc = self
                .uncertainty_budget_and_authorship
                .info_gap_robustness
                .critical_requirement_rc,
            robustness_horizon_alpha_hat = self
                .uncertainty_budget_and_authorship
                .info_gap_robustness
                .robustness_horizon_alpha_hat,
            opportuneness_horizon_beta_hat = self
                .uncertainty_budget_and_authorship
                .info_gap_robustness
                .opportuneness_horizon_beta_hat,
            global_maximin_safeguard = self
                .uncertainty_budget_and_authorship
                .info_gap_robustness
                .global_maximin_safeguard,
            auth_null = self
                .uncertainty_budget_and_authorship
                .field_authorship
                .null_model,
            auth_decision = self
                .uncertainty_budget_and_authorship
                .field_authorship
                .decision_rule,
            auth_reviewer = self
                .uncertainty_budget_and_authorship
                .field_authorship
                .adversarial_reviewer,
            missingness_mechanism = self.data_and_audit_contract.missingness_mechanism,
            seed_and_environment = self.data_and_audit_contract.seed_and_environment,
            null_model = self.data_and_audit_contract.null_model,
            preregistration_hash = self.data_and_audit_contract.preregistration_hash,
            attempt_counter_k = self.data_and_audit_contract.attempt_counter_k,
            e_process_multiplicity = self.data_and_audit_contract.e_process_multiplicity,
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
            declared_act = self.decision_rule_and_scope.declared_act,
            scope_statement = self.decision_rule_and_scope.scope_statement,
            who_pays_for_error = self.decision_rule_and_scope.who_pays_for_error,
            exit_clause = self.decision_rule_and_scope.exit_clause,
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
                unused.push(unquote(rest));
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
                if val.is_empty() || val == "[]" {
                    stack.push((indent, key));
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
            req(k)?
                .parse::<f64>()
                .map_err(|e| format!("invalid numeric value in '{k}': {e}"))
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

fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        t[1..t.len() - 1].replace("\\\"", "\"")
    } else {
        t.to_string()
    }
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
}
