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
    brier_score, critical_cost_loss_threshold, quantile_score, CorpPavDecomposition, CostLossRule,
    ExactCoinCalibrationBenchmark,
};
pub use information::{
    amplitude_constrained_mi_bound, binary_entropy_nats, fano_error_floor, kl_divergence,
    DpiMarkovChain, SdpiHierarchy,
};
pub use invariance::{
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
pub use tail::{hill_estimator, HillTailDiagnostic, TailMomentRegime};
pub use time::{
    asymmetric_bet_log_growth, asymmetric_kelly_fraction, ito_time_growth_rate,
    simulate_absorbing_trajectories, MixingTimeAudit, TimeKellyDiagnostic,
};
