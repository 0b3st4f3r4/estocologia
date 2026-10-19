//! stochology-s2 — the deliberative System 2: tree search, belief state, inference and stress.
//!
//! System 1 (deterministic triage, O(1)) lives in `stochology-core/src/s1.rs` and closes the error family
//! of a card. System 2 is the slow mode that docs/PLAN.md §4 and Chapter 16 declare: **LATS** (tree search
//! with estimate) plus the **belief-state engine** over the hypergraph, and the fronts that demand
//! deliberation — heavy inference (MCMC) and adversarial stress.
//!
//! # What this crate does, file by file
//!
//! - `lats`: the declared world (`World`), the search with UCB and the greedy agent that serves as null.
//! - `belief`: the belief-state engine, with the floor and the ceiling of the contract on every probability.
//! - `hypergraph`: the System 2 world over the real hypergraph — the goal is the node that **no lens
//!   covers**, measured edge by edge.
//! - `inference`: random-walk Metropolis, with the target declared by the caller.
//! - `stress`: the panel A1-A8 attacked in the order that belief learned, against the declared order A1 to A8.
//! - `planning`: the resolution of the question (R axis) chosen by the instrument, never by guesswork.
//!
//! # Declared and not implemented
//!
//! - **Deep code synthesis (F1).** `docs/PLAN.md` §4 asks System 2 to synthesize code. This crate does
//!   not generate a program: what exists is the search over the hypergraph, the inference and the stress. The
//!   front remains declared, and there is no honest halfway house between tree search and a code generator.
//! - **Attack order with asymmetric cost.** The deliberative stress counts attacks, not the price of each
//!   attack; the cost weighting per band remains declared.
//! - **R2 and R3 in planning.** The instrument measures the aggregate/fine pair (R0 and R1); the finer
//!   resolutions remain declared as beyond the measured reach, and are not estimated by extrapolation.

pub mod belief;
pub mod bridge;
pub mod calibration;
pub mod hypergraph;
pub mod inference;
pub mod lats;
pub mod panel;
pub mod planning;
pub mod runner;
pub mod stress;

pub use belief::{BeliefState, CEILING, FLOOR};
pub use calibration::{Calibration, MIN_SAMPLE, Outcome, calibrate, read_outcomes};
pub use hypergraph::{HypergraphWorld, Trail, action_destination, sample_graph};
pub use inference::{Chain, Metropolis, log_beta, log_normal};
pub use lats::{Decision, Lats, World, greedy};
pub use panel::{
    Assignment, Authority, Closure, Lens, MIN_READINGS, Piece, Reading, Receipt, Refusal, Reviewer,
    Unit, VETOES_PER_WINDOW, Veto, Workbench, WorkbenchState, apply, assign, close,
};
pub use planning::{Evaluation, ResolutionDecision, decide, evaluate as evaluate_resolutions};
pub use runner::{LocalRunner, brier, declared_hash};
pub use stress::{
    Attack, DECLARED_ORDER, PanelBelief, learn, stress_deliberative, stress_in_order,
};
