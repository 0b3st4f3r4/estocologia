//! Global verification gate of the wave engine.
//!
//! Six charges, each with its own proof: structure of the hypergraph, Hermiticity of L^(q),
//! conservation of the norm by the Cayley propagator, integrity of the seal chain (with an
//! internal negative proof), determinism between two runs, and the causal asymmetry learned by
//! STDP. Any one that fails takes the report down.

use stochology_hypergraph::Lens;

use crate::complex::Complex;
use crate::magnetic::{MagneticHypergraph, verify_chain};
use crate::seal::Seal;
use crate::spike::StdpBus;

/// Outcome of one gate.
#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    pub gate: &'static str,
    pub passed: bool,
    pub detail: String,
}

/// Complete report.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub results: Vec<Outcome>,
    pub seal: u64,
}

impl Report {
    pub fn passed(&self) -> bool {
        !self.results.is_empty() && self.results.iter().all(|r| r.passed)
    }

    pub fn failures(&self) -> Vec<&Outcome> {
        self.results.iter().filter(|r| !r.passed).collect()
    }
}

/// Scenario submitted to the gate.
#[derive(Debug, Clone)]
pub struct Scenario<'a> {
    pub graph: &'a MagneticHypergraph,
    pub pulse: Vec<Complex>,
    pub bus: &'a StdpBus,
    pub chain: Vec<(Lens, Lens)>,
    pub steps: usize,
    pub dt: f64,
    /// Minimum causal asymmetry required in every link of the chain.
    pub min_asymmetry: f64,
}

fn outcome(gate: &'static str, verdict: Result<(), String>, detail: String) -> Outcome {
    match verdict {
        Ok(()) => Outcome {
            gate,
            passed: true,
            detail,
        },
        Err(e) => Outcome {
            gate,
            passed: false,
            detail: e,
        },
    }
}

/// Runs the six gates.
pub fn verify(scenario: &Scenario<'_>) -> Report {
    let mut results = Vec::new();

    // G1: structure.
    let structure = {
        let problems = scenario.graph.validate();
        let size_ok = scenario.pulse.len() == scenario.graph.vertices.len();
        let dt_ok = scenario.dt > 0.0 && scenario.dt.is_finite();
        if !problems.is_empty() {
            Err(problems.join("; "))
        } else if !size_ok {
            Err(format!(
                "pulse with {} positions for {} vertices",
                scenario.pulse.len(),
                scenario.graph.vertices.len()
            ))
        } else if !dt_ok {
            Err(format!("invalid dt: {}", scenario.dt))
        } else {
            Ok(())
        }
    };
    results.push(outcome(
        "G1 structure",
        structure,
        format!(
            "{} vertices, {} hyperedges, q = {}",
            scenario.graph.vertices.len(),
            scenario.graph.hyperedges.len(),
            scenario.graph.q
        ),
    ));

    // G2: Hermiticity.
    let lap = scenario.graph.laplacian();
    let hermitian = scenario.graph.check_hermitian(&lap).map(|_| ());
    let asymmetry = scenario.graph.hermitian_asymmetry(&lap);
    results.push(outcome(
        "G2 hermitian",
        hermitian,
        format!("largest deviation of L^(q) with respect to L^(q)^dagger: {asymmetry:.3e}"),
    ));

    // G3: conservation of the norm.
    let initial_norm: f64 = scenario.pulse.iter().map(|c| c.modulus_sqr()).sum();
    let unitary = match scenario
        .graph
        .propagate(&scenario.pulse, scenario.steps, scenario.dt)
    {
        Ok(p) => {
            let deviation = (p.norm2 - initial_norm).abs();
            if deviation < 1e-9 {
                Ok(())
            } else {
                Err(format!(
                    "norm changed from {initial_norm:.12} to {:.12}",
                    p.norm2
                ))
            }
        }
        Err(e) => Err(e),
    };
    results.push(outcome(
        "G3 unitary",
        unitary,
        format!(
            "l2 norm preserved over {} steps of dt = {}",
            scenario.steps, scenario.dt
        ),
    ));

    // G4: seal chain, with an embedded negative proof.
    let chain = match scenario
        .graph
        .propagate_audited(&scenario.pulse, scenario.steps, scenario.dt)
    {
        Ok(steps) => match verify_chain(&steps) {
            Ok(()) => {
                let mut corrupted = steps.clone();
                if let Some(s) = corrupted.last_mut()
                    && let Some(c) = s.state.first_mut()
                {
                    c.re += 0.001;
                }
                if verify_chain(&corrupted).is_err() {
                    Ok(())
                } else {
                    Err("negative proof did not detect state tampering".to_string())
                }
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    };
    results.push(outcome(
        "G4 chain",
        chain,
        format!(
            "state seal chained over {} steps; tampering of 0.001 rejected",
            scenario.steps
        ),
    ));

    // G5: determinism.
    let determinism = match (
        scenario
            .graph
            .propagate_audited(&scenario.pulse, scenario.steps, scenario.dt),
        scenario
            .graph
            .propagate_audited(&scenario.pulse, scenario.steps, scenario.dt),
    ) {
        (Ok(a), Ok(b)) => {
            let sa = a.last().map(|s| s.chain_seal);
            let sb = b.last().map(|s| s.chain_seal);
            if sa.is_some() && sa == sb {
                Ok(())
            } else {
                Err("two runs of the same pulse diverged".to_string())
            }
        }
        (Err(e), _) | (_, Err(e)) => Err(e),
    };
    results.push(outcome(
        "G5 determinism",
        determinism,
        "two independent runs seal the same state".to_string(),
    ));

    // G6: learned causality.
    let causality = scenario
        .bus
        .check_causality(&scenario.chain, scenario.min_asymmetry);
    results.push(outcome(
        "G6 causality",
        causality,
        format!(
            "minimum asymmetry required of {:.2} in {} links; synaptic seal 0x{:016x}",
            scenario.min_asymmetry,
            scenario.chain.len(),
            scenario.bus.seal()
        ),
    ));

    let mut seal = Seal::new();
    for r in &results {
        seal.write_text(r.gate);
        seal.write_u64(r.passed as u64);
    }
    seal.write_u64(scenario.graph.topology_seal());
    seal.write_u64(scenario.bus.seal());

    Report {
        results,
        seal: seal.finish(),
    }
}
