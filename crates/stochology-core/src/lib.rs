//! `stochology-core` — Core mathematical and epistemological primitives of the Stochology Protocol v1.0.

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
    UnitAndMargin, VarianceBudget, CONTRACT_VERSION,
};
pub use invariants::{
    verify_resolution_phi5, ChouldechovaAudit, ExceedanceDesignEffect, OperationalWeightingGap,
};
pub use layers::{
    apply_cromwell_floor, empirical_crps, laplace_succession, respects_cromwell, Layer,
    ManskiBounds, MengAudit, NonlinearSummary, PerformativeDiagnostic, PerformativeSystem,
    UncertaintyClass,
};
pub use rng::{
    mean, median, median_absolute_deviation, quantile, quantile_sorted, sample_std,
    sample_variance, Rng, SplitMix64, DEFAULT_SEED,
};
pub use sha256::{is_valid_sha256_hex, sha256_bytes, sha256_hex};
pub use uncertainty::{InfoGapEvaluation, InfoGapOption, KennedyOHaganDiscrepancy};
