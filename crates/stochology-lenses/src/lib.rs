//! `stochology-lenses` — The Five Transversal Lenses of the Stochology Protocol (§1.3),
//! plus the $(\varepsilon, \delta)$-Almost-Perfect Machine `M = (T, Φ, θ)` and First-Class Authentic Model Form (`δ(x) + Ŝ*`).

pub mod decision;
pub mod information;
pub mod invariance;
pub mod machine;
pub mod relational;
pub mod tail;
pub mod time;

pub use decision::{
    brier_score, critical_cost_loss_threshold, logarithmic_score, quantile_score,
    CorpPavDecomposition, CostLossRule, ExactCoinCalibrationBenchmark,
};
pub use information::{
    amplitude_constrained_mi_bound, binary_entropy_nats, blahut_arimoto_capacity,
    estimate_empirical_channel, fano_error_floor, kl_divergence, DpiMarkovChain, SdpiHierarchy,
};
pub use invariance::{
    chow_invariance_f_stat, cinelli_hazlett_robustness_value,
    hidden_confounding_threshold_lambda_star, run_icp, run_nonlinear_icp_nile,
    second_order_worst_case_loss, second_order_worst_case_loss_full, CausalEnvironment, IcpResult,
    NonlinearIcpNileResult, PearlRung,
};
pub use machine::{
    AlmostPerfectMachine, AuthenticModelForm, AuthenticityCriterionA, DiscardedAtomRecord,
    IdentityClosureAudit,
};
pub use relational::{
    DaisyworldEquilibriumPoint, DaisyworldHomeostasisAudit, DaisyworldSystem,
    DissipativeAutopoieticAudit, RelationalComplexSystemsSynthesis, RelationalFigurationAudit,
    SchellingCell, SchellingPhaseTransitionAudit, SchellingSegregationModel,
};
pub use tail::{
    dedh_moment_tail_index, expected_shortfall_99, gpd_pot_fit, hill_estimator, HillTailDiagnostic,
    TailMomentRegime,
};
pub use time::{
    asymmetric_bet_log_growth, asymmetric_kelly_fraction, empirical_kelly_optimal_fraction,
    ito_time_growth_rate, simulate_absorbing_trajectories, MixingTimeAudit, TimeKellyDiagnostic,
};
