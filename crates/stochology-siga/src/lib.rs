//! `stochology-siga` — The 4 Eliminatory Filters and the 8-Step SIGA Operational Roadmap (§3).

pub mod funnel;
pub mod roadmap;

pub use funnel::{
    evaluate_four_filter_funnel, fit_shelf_beta_prior, ln_gamma_lanczos,
    regularized_incomplete_beta, sbc_rank_chi2, FunnelCandidate, FunnelResult,
};
pub use roadmap::{
    run_full_siga_roadmap, run_full_siga_roadmap_empirical, simulate_sbc_beta_binomial_ranks,
    EvidenceProcess, NaturalFrequencyMemorandum, SigaExecutionReport,
};
