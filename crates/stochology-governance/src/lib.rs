//! `stochology-governance` — Bitemporal Clock, Single-Opening Temporal Vault,
//! and Formal Adaptation to the 4 Regimes of Production Drift (§4).

pub mod clock;
pub mod drift;
pub mod vault;

pub use clock::{query_with_bitemporal_clock, BitemporalClockAudit, BitemporalRecord};
pub use drift::{
    adjust_posterior_for_label_shift, estimate_label_shift_four_methods,
    fit_calibrated_logistic_1d, fit_covariate_drift_extra_wcp, CovariateDriftResult, DriftRegime,
    LabelShiftDiagnostic, MurphyCorpCircuitBreaker,
};
pub use vault::{TemporalVault, VaultError, VaultOpeningReport, VaultState};
