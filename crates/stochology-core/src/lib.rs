//! `stochology-core` — Core mathematical and epistemological primitives of the Stochology Protocol.

pub mod card;
pub mod invariants;
pub mod layers;
pub mod rng;
pub mod sha256;
pub mod uncertainty;

pub use card::{
    BridgePerformativity, CausalLadder, ContractViolation, DataAndAuditContract,
    DecisionMakerQuestion, DecisionRuleAndScope, DominantEvent, EpistemologicalCard,
    EpistemologicalHeader, FieldAuthorship, FindingsRegistry, InfoGapRobustness,
    MissingnessMechanism, MurphyCorpDecomposition, NoisePhysics, UncertaintyBudgetAndAuthorship,
    UnitAndMargin, VarianceBudget,
};
pub use invariants::{
    verify_resolution_phi5, ChouldechovaAudit, ExceedanceDesignEffect, OperationalWeightingGap,
};
pub use layers::{
    apply_cromwell_floor, empirical_crps, empirical_wasserstein_1, laplace_succession,
    respects_cromwell, Layer, ManskiBounds, MengAudit, NonlinearSummary, PerformativeDiagnostic,
    PerformativeSystem, UncertaintyClass,
};
pub use rng::{
    block_bootstrap_ci, mean, median, median_absolute_deviation, quantile, quantile_sorted,
    sample_std, sample_variance, OnlineMoments, Rng, SplitMix64, DEFAULT_SEED,
};
pub use sha256::{
    hmac_sha256_bytes, hmac_sha256_hex, is_valid_sha256_hex, sha256_bytes, sha256_hex,
};
pub use uncertainty::{
    cholesky_decompose, cholesky_log_det, cholesky_solve, InfoGapEvaluation, InfoGapOption,
    KennedyOHaganDiscrepancy,
};
