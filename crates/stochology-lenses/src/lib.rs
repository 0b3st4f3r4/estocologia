//! `stochology-lenses` — The Five Transversal Lenses of the Stochology Protocol v1.0 (§1.3).

pub mod decision;
pub mod information;
pub mod invariance;
pub mod tail;
pub mod time;

pub use decision::{
    brier_score, critical_cost_loss_threshold, quantile_score, CorpPavDecomposition, CostLossRule,
};
pub use information::{
    amplitude_constrained_mi_bound, binary_entropy_nats, fano_error_floor, kl_divergence,
    SdpiHierarchy,
};
pub use invariance::{
    hidden_confounding_threshold_lambda_star, run_icp, second_order_worst_case_loss,
    CausalEnvironment, IcpResult, PearlRung,
};
pub use tail::{hill_estimator, HillTailDiagnostic, TailMomentRegime};
pub use time::{
    asymmetric_bet_log_growth, asymmetric_kelly_fraction, ito_time_growth_rate,
    simulate_absorbing_trajectories, MixingTimeAudit, TimeKellyDiagnostic,
};
