//! Core of Stochology: layers L1-L3, invariants Phi1-Phi4,
//! safeguard panel A1-A8 and orchestrator M.
//!
//! Every artifact is deterministic: no function uses a clock, system entropy or
//! global state. Seeds are explicit in every card.

pub mod agents;
pub mod audit;
pub mod budget;
pub mod conduct;
pub mod erlang;
pub mod funnel;
pub mod invariants;
pub mod layer;
pub mod orchestrator;
pub mod resolution;
pub mod rng;
pub mod s1;
pub mod schema;

pub use agents::{AgentA, Approval, Check, approve, check};
pub use funnel::{DominanceMatrix, LocalPolicy, OptimalStop};
pub use invariants::{Failure, Gate, evaluate, report};
pub use layer::{Card, Layer, UncertaintyType};
pub use orchestrator::OrchestratorM;
pub use resolution::{
    R_HORIZON_DAYS, Resolution, ResolutionLoss, measure as measure_resolution_loss,
};
pub use rng::Rng;

/// Hill estimator over the `k` largest positive values, using `cutoff_index` (`k` or `k - 1`) as threshold.
pub fn hill_at_cutoff(series: &[f64], k: usize, cutoff_index: usize) -> f64 {
    let mut positives: Vec<f64> = series.iter().copied().filter(|x| *x > 0.0).collect();
    if positives.len() <= cutoff_index || positives_too_short(positives.len(), k) {
        return f64::NAN;
    }
    positives.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let cutoff = positives[cutoff_index];
    if cutoff <= 0.0 {
        return f64::NAN;
    }
    let sum: f64 = positives[..k].iter().map(|x| (x / cutoff).ln()).sum();
    if sum <= 0.0 {
        return f64::NAN;
    }
    k as f64 / sum
}

fn positives_too_short(len: usize, k: usize) -> bool {
    len < k || k == 0
}

/// Murphy decomposition (REL, RES, UNC) grouping by exact forecast value.
pub fn murphy_decomposition(prev: &[f64], res: &[f64]) -> (f64, f64, f64) {
    if prev.is_empty() || prev.len() != res.len() {
        return (0.0, 0.0, 0.0);
    }
    let n = prev.len() as f64;
    let base = res.iter().sum::<f64>() / n;
    let mut distinct: Vec<f64> = prev.to_vec();
    distinct.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    distinct.dedup();
    let mut rel = 0.0;
    let mut resol = 0.0;
    for value in distinct {
        let idx: Vec<usize> = (0..prev.len()).filter(|i| prev[*i] == value).collect();
        if idx.is_empty() {
            continue;
        }
        let nb = idx.len() as f64;
        let ob = idx.iter().map(|i| res[*i]).sum::<f64>() / nb;
        rel += (nb / n) * (value - ob) * (value - ob);
        resol += (nb / n) * (ob - base) * (ob - base);
    }
    (rel, resol, base * (1.0 - base))
}
