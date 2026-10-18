//! Wave engine of Stochology: magnetic hypergraph, phase plasticity, and spikes.
//!
//! Three layers, in the order in which System 1 and System 2 take turns:
//!
//! 1. **Local superposition** (`phasor`): the Phi of a hyperedge evaluated as a phasor. The
//!    energy has an incoherent term and a cross interference term; the coherence C(h) decides
//!    between direct propagation (System 1), beating (Chrononomy), and conflict (System 2).
//! 2. **Multi-step propagation** (`magnetic`): Hermitian magnetic Laplacian L^(q) with the
//!    directional charge in alpha and the unitary Cayley propagator. Norm preserved, auditable
//!    seal chain, detection of arrivals that cancel each other.
//! 3. **Learning** (`plasticity` and `spike`): Kuramoto-Oja consolidates the System 2 verdict
//!    with a gain gate and rollback; the 40-byte spike bus learns the causal arrow between the
//!    five lenses through phasor STDP and recognizes polyphonic words.
//!
//! Divergences declared with respect to the draft that originated this crate:
//!
//! - the frustration <Psi, L Psi> does not serve as a single trigger: in symmetric antiphase it
//!   equals zero precisely when the conflict exists. The trigger is the coherence of arrivals
//!   per vertex, and frustration stays as a diagnostic;
//! - the plasticity gate charges target resonance gain, not a monotone drop of frustration.
//!
//! Nothing here uses an external dependency; the state is deterministic and `wave --check`
//! takes the report down when any one of the six gates fails.

pub mod complex;
pub mod gate;
pub mod magnetic;
pub mod phasor;
pub mod plasticity;
pub mod seal;
pub mod spike;
