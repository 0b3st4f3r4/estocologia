//! Magnetic hypergraph: the Hermitian Laplacian L^(q), the unitary Cayley propagator, and the
//! audit by seal chain.
//!
//! The complex incidence matrix B_q(v, h) = eta_{vh} * e^{i 2 pi q alpha_{vh}} carries the
//! topological weight in eta and the directional charge in alpha. With W_e the hyperedge weights,
//!
//!   A^(q) = D_v^{-1/2} B_q W_e D_e^{-1} B_q^dagger D_v^{-1/2},   L^(q) = I - A^(q).
//!
//! Since B_q W_e D_e^{-1} B_q^dagger is Hermitian, L^(q) is too, and the Cayley step
//! (I + i dt L / 2)^{-1} (I - i dt L / 2) preserves the l2 norm up to floating-point error.
//! Explicit Euler does not: the norm grows at every step.

use std::f64::consts::PI;

use crate::complex::Complex;
use crate::seal::{CHAIN_SEED, Seal};

const HERMITIAN_TOLERANCE: f64 = 1e-11;

#[derive(Debug, Clone, PartialEq)]
pub struct Incidence {
    pub vertex: usize,
    pub weight: f64,
    pub alpha: f64,
}

impl Incidence {
    pub fn new(vertex: usize, weight: f64, alpha: f64) -> Self {
        Self {
            vertex,
            weight,
            alpha,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MagneticHyperedge {
    pub id: String,
    pub weight: f64,
    pub incidences: Vec<Incidence>,
}

impl MagneticHyperedge {
    pub fn new(id: &str, weight: f64, incidences: Vec<Incidence>) -> Self {
        Self {
            id: id.to_string(),
            weight,
            incidences,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MagneticHypergraph {
    pub vertices: Vec<String>,
    pub hyperedges: Vec<MagneticHyperedge>,
    pub q: f64,
}

/// Diagnostic of a state over the hypergraph.
#[derive(Debug, Clone, PartialEq)]
pub struct Propagation {
    pub state: Vec<Complex>,
    pub initial_norm2: f64,
    pub norm2: f64,
    /// <Psi, L Psi>, the gradient energy of the state.
    pub frustration: f64,
    /// |(L Psi)_v| per vertex.
    pub tension: Vec<f64>,
    /// Sum of the arrival contributions per vertex, |A_uv| |psi_u| with u != v.
    pub incoming: Vec<f64>,
    /// Coherence of the arrivals per vertex; 0 means arrivals that cancel each other.
    pub local_coherence: Vec<f64>,
    /// Vertices with a relevant arrival and coherence below the critical threshold.
    pub dissonant: Vec<usize>,
}

/// One audited step of the propagation.
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub index: usize,
    pub state: Vec<Complex>,
    pub norm2: f64,
    pub frustration: f64,
    pub dissonant: Vec<usize>,
    pub state_seal: u64,
    pub chain_seal: u64,
}

impl MagneticHypergraph {
    pub fn new(vertices: Vec<&str>, hyperedges: Vec<MagneticHyperedge>, q: f64) -> Self {
        Self {
            vertices: vertices.into_iter().map(str::to_string).collect(),
            hyperedges,
            q,
        }
    }

    /// Structural problems: index out of range, repeated incidence, non-finite value.
    pub fn validate(&self) -> Vec<String> {
        let mut problems = Vec::new();
        if !self.q.is_finite() {
            problems.push("non-finite q".to_string());
        }
        for (h, edge) in self.hyperedges.iter().enumerate() {
            if !edge.weight.is_finite() || edge.weight < 0.0 {
                problems.push(format!("invalid weight in hyperedge {h}"));
            }
            if edge.incidences.len() < 2 {
                problems.push(format!("hyperedge {h} with fewer than two incidences"));
            }
            let mut seen = Vec::new();
            for inc in &edge.incidences {
                if inc.vertex >= self.vertices.len() {
                    problems.push(format!(
                        "hyperedge {h} cites nonexistent vertex {}",
                        inc.vertex
                    ));
                }
                if seen.contains(&inc.vertex) {
                    problems.push(format!("hyperedge {h} repeats vertex {}", inc.vertex));
                }
                seen.push(inc.vertex);
                if !inc.weight.is_finite() || !inc.alpha.is_finite() {
                    problems.push(format!("non-finite incidence in hyperedge {h}"));
                }
            }
        }
        problems
    }

    /// Builds the normalized L^(q).
    #[allow(clippy::needless_range_loop)]
    pub fn laplacian(&self) -> Vec<Vec<Complex>> {
        let n = self.vertices.len();
        let m = self.hyperedges.len();
        let mut b_q = vec![vec![Complex::ZERO; m]; n];
        let mut degree_v = vec![0.0; n];
        let mut degree_e = vec![0.0; m];

        for (h, edge) in self.hyperedges.iter().enumerate() {
            for inc in &edge.incidences {
                let theta = 2.0 * PI * self.q * inc.alpha;
                let entry = Complex::polar(inc.weight, theta);
                b_q[inc.vertex][h] = entry;
                let m2 = entry.modulus_sqr();
                degree_e[h] += m2;
                degree_v[inc.vertex] += edge.weight * m2;
            }
        }

        let inv_sqrt_v: Vec<f64> = degree_v
            .iter()
            .map(|&d| if d > 1e-15 { 1.0 / d.sqrt() } else { 0.0 })
            .collect();
        let inv_e: Vec<f64> = degree_e
            .iter()
            .map(|&d| if d > 1e-15 { 1.0 / d } else { 0.0 })
            .collect();

        let mut lap = vec![vec![Complex::ZERO; n]; n];
        for u in 0..n {
            for v in 0..n {
                let mut sum = Complex::ZERO;
                for h in 0..m {
                    let scale = self.hyperedges[h].weight * inv_e[h];
                    sum = sum + (b_q[u][h] * b_q[v][h].conj()) * scale;
                }
                let a = sum * (inv_sqrt_v[u] * inv_sqrt_v[v]);
                lap[u][v] = if u == v {
                    Complex::ONE - a
                } else {
                    Complex::ZERO - a
                };
            }
        }
        lap
    }

    /// Largest Hermiticity deviation of L^(q); zero for the exact operator.
    #[allow(clippy::needless_range_loop)]
    pub fn hermitian_asymmetry(&self, lap: &[Vec<Complex>]) -> f64 {
        let n = lap.len();
        let mut largest: f64 = 0.0;
        for u in 0..n {
            for v in u..n {
                let diff = (lap[u][v] - lap[v][u].conj()).modulus();
                if diff > largest {
                    largest = diff;
                }
            }
        }
        largest
    }

    pub fn check_hermitian(&self, lap: &[Vec<Complex>]) -> Result<f64, String> {
        let largest = self.hermitian_asymmetry(lap);
        if largest > HERMITIAN_TOLERANCE {
            return Err(format!(
                "L^(q) is not Hermitian: largest deviation {largest:.3e} above {HERMITIAN_TOLERANCE:.1e}"
            ));
        }
        Ok(largest)
    }

    /// Canonical seal of the topology: q, hyperedge weights, weights, and magnetic angles.
    pub fn topology_seal(&self) -> u64 {
        let mut seal = Seal::new();
        seal.write_f64(self.q);
        for edge in &self.hyperedges {
            seal.write_text(&edge.id);
            seal.write_f64(edge.weight);
            for inc in &edge.incidences {
                seal.write_u64(inc.vertex as u64);
                seal.write_f64(inc.weight);
                seal.write_f64(inc.alpha);
            }
        }
        seal.finish()
    }

    /// Diagnostic of an arbitrary state over the current topology.
    pub fn diagnose(&self, state: &[Complex], initial_norm2: f64) -> Propagation {
        let lap = self.laplacian();
        self.diagnose_with(&lap, state, initial_norm2)
    }

    #[allow(clippy::needless_range_loop)]
    fn diagnose_with(
        &self,
        lap: &[Vec<Complex>],
        state: &[Complex],
        initial_norm2: f64,
    ) -> Propagation {
        let n = state.len();
        let mut lpsi = vec![Complex::ZERO; n];
        for u in 0..n {
            let mut sum = Complex::ZERO;
            for v in 0..n {
                sum = sum + lap[u][v] * state[v];
            }
            lpsi[u] = sum;
        }

        let mut frustration = 0.0;
        let mut tension = vec![0.0; n];
        let mut incoming = vec![0.0; n];
        let mut local_coherence = vec![0.0; n];
        let mut dissonant = Vec::new();

        for u in 0..n {
            frustration += (state[u].conj() * lpsi[u]).re;
            tension[u] = lpsi[u].modulus();

            let mut arrival = Complex::ZERO;
            let mut arrival_sum = 0.0;
            for v in 0..n {
                if v == u {
                    continue;
                }
                arrival = arrival + lap[u][v] * state[v];
                arrival_sum += lap[u][v].modulus() * state[v].modulus();
            }
            incoming[u] = arrival_sum;
            if arrival_sum > 1e-12 {
                local_coherence[u] = (arrival.modulus() / arrival_sum).clamp(0.0, 1.0);
            }
            // Relevant arrival that cancels: energy coming in and none coming out.
            if arrival_sum > 0.10 && local_coherence[u] < 0.35 {
                dissonant.push(u);
            }
        }

        Propagation {
            state: state.to_vec(),
            initial_norm2,
            norm2: state.iter().map(|c| c.modulus_sqr()).sum(),
            frustration: frustration.max(0.0),
            tension,
            incoming,
            local_coherence,
            dissonant,
        }
    }

    /// Propagates `steps` Cayley steps and returns the final diagnostic.
    pub fn propagate(
        &self,
        initial: &[Complex],
        steps: usize,
        dt: f64,
    ) -> Result<Propagation, String> {
        let n = self.vertices.len();
        if initial.len() != n {
            return Err(format!(
                "state with {} positions for {n} vertices",
                initial.len()
            ));
        }
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!("invalid dt: {dt}"));
        }
        let lap = self.laplacian();
        self.check_hermitian(&lap)?;
        let initial_norm2: f64 = initial.iter().map(|c| c.modulus_sqr()).sum();
        let mut psi = initial.to_vec();
        for _ in 0..steps {
            psi = cayley_step(&lap, &psi, dt)?;
        }
        Ok(self.diagnose_with(&lap, &psi, initial_norm2))
    }

    /// Propagates recording one audited step per iteration, with state seal and chained chain.
    pub fn propagate_audited(
        &self,
        initial: &[Complex],
        steps: usize,
        dt: f64,
    ) -> Result<Vec<Step>, String> {
        let n = self.vertices.len();
        if initial.len() != n {
            return Err(format!(
                "state with {} positions for {n} vertices",
                initial.len()
            ));
        }
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!("invalid dt: {dt}"));
        }
        let lap = self.laplacian();
        self.check_hermitian(&lap)?;

        let mut psi = initial.to_vec();
        let mut history = Vec::with_capacity(steps + 1);
        let mut previous = CHAIN_SEED;

        for step_index in 0..=steps {
            if step_index > 0 {
                psi = cayley_step(&lap, &psi, dt)?;
            }
            let frustration = frustration_of(&lap, &psi);
            let state_seal = seal_of_state(&psi);

            let mut seal = Seal::new();
            seal.write_u64(previous);
            seal.write_u64(step_index as u64);
            seal.write_u64(state_seal);
            seal.write_f64(frustration);
            let chain_seal = seal.finish();
            previous = chain_seal;

            let diag = self.diagnose_with(&lap, &psi, 0.0);
            history.push(Step {
                index: step_index,
                state: psi.clone(),
                norm2: diag.norm2,
                frustration,
                dissonant: diag.dissonant,
                state_seal,
                chain_seal,
            });
        }
        Ok(history)
    }
}

/// Canonical seal of a phasor state.
pub fn seal_of_state(psi: &[Complex]) -> u64 {
    let mut seal = Seal::new();
    for c in psi {
        seal.write_f64(c.re);
        seal.write_f64(c.im);
    }
    seal.finish()
}

/// Gradient energy <Psi, L Psi>.
pub fn frustration_of(lap: &[Vec<Complex>], psi: &[Complex]) -> f64 {
    let n = psi.len();
    let mut total = 0.0;
    for u in 0..n {
        let mut sum = Complex::ZERO;
        for v in 0..n {
            sum = sum + lap[u][v] * psi[v];
        }
        total += (psi[u].conj() * sum).re;
    }
    total.max(0.0)
}

/// One Cayley step: (I + i dt L / 2) psi' = (I - i dt L / 2) psi.
pub fn cayley_step(lap: &[Vec<Complex>], psi: &[Complex], dt: f64) -> Result<Vec<Complex>, String> {
    let n = psi.len();
    let half = Complex::new(0.0, 0.5 * dt);
    let mut a = vec![vec![Complex::ZERO; n]; n];
    let mut b = vec![Complex::ZERO; n];

    for u in 0..n {
        let mut sum = Complex::ZERO;
        for v in 0..n {
            let identity = if u == v { Complex::ONE } else { Complex::ZERO };
            a[u][v] = identity + half * lap[u][v];
            sum = sum + (identity - half * lap[u][v]) * psi[v];
        }
        b[u] = sum;
    }
    solve_linear(a, b)
}

/// Complex Gaussian elimination with partial pivoting.
#[allow(clippy::needless_range_loop)]
pub fn solve_linear(mut a: Vec<Vec<Complex>>, mut b: Vec<Complex>) -> Result<Vec<Complex>, String> {
    let n = b.len();
    for k in 0..n {
        let mut pivot = k;
        let mut largest = a[k][k].modulus();
        for i in (k + 1)..n {
            let m = a[i][k].modulus();
            if m > largest {
                largest = m;
                pivot = i;
            }
        }
        if largest < 1e-14 {
            return Err(format!("singular system at step {k}"));
        }
        a.swap(k, pivot);
        b.swap(k, pivot);

        let p = a[k][k];
        for i in (k + 1)..n {
            let factor = match a[i][k].div(p) {
                Some(f) => f,
                None => return Err(format!("degenerate pivot at step {k}")),
            };
            b[i] = b[i] - factor * b[k];
            for j in k..n {
                a[i][j] = a[i][j] - factor * a[k][j];
            }
        }
    }

    let mut x = vec![Complex::ZERO; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum = sum - a[i][j] * x[j];
        }
        match sum.div(a[i][i]) {
            Some(v) => x[i] = v,
            None => return Err(format!("null diagonal at step {i}")),
        }
    }
    Ok(x)
}

/// Recomputes the seal chain and reports tampering of state, step, or frustration.
pub fn verify_chain(steps: &[Step]) -> Result<(), String> {
    let mut expected = CHAIN_SEED;
    for step in steps {
        let measured = seal_of_state(&step.state);
        if measured != step.state_seal {
            return Err(format!(
                "step {}: state seal diverges (declared 0x{:016x}, measured 0x{:016x})",
                step.index, step.state_seal, measured
            ));
        }
        let mut seal = Seal::new();
        seal.write_u64(expected);
        seal.write_u64(step.index as u64);
        seal.write_u64(measured);
        seal.write_f64(step.frustration);
        let chain = seal.finish();
        if chain != step.chain_seal {
            return Err(format!(
                "step {}: broken chain (declared 0x{:016x}, measured 0x{:016x})",
                step.index, step.chain_seal, chain
            ));
        }
        expected = chain;
    }
    Ok(())
}

/// Arrival phase of a vertex at the target, through an incidence: used by plasticity.
pub(crate) fn arrival_phase(
    environment: &[Complex],
    hyperedge: &MagneticHyperedge,
    target: usize,
    q: f64,
) -> Option<(Complex, f64, f64)> {
    let two_pi_q = 2.0 * PI * q;
    let mut context = Complex::ZERO;
    let mut target_alpha = None;
    let mut target_weight = None;
    for inc in &hyperedge.incidences {
        if inc.vertex == target {
            target_alpha = Some(inc.alpha);
            target_weight = Some(inc.weight);
        } else {
            // B_q(u, h)^* psi_u = eta e^{-i 2 pi q alpha} psi_u
            let transport = Complex::polar(inc.weight, -two_pi_q * inc.alpha);
            context = context + transport * environment[inc.vertex];
        }
    }
    let alpha = target_alpha?;
    let _ = target_weight?;
    let arrival = context.phase() + two_pi_q * alpha;
    Some((context, arrival, context.modulus()))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn constructive_graph(destructive: bool) -> MagneticHypergraph {
        MagneticHypergraph::new(
            vec!["V0_WORLD", "V1_THESIS", "V2_ANTITHESIS", "V3_AGENT"],
            vec![
                MagneticHyperedge::new(
                    "H0_DIFFUSION",
                    1.0,
                    vec![
                        Incidence::new(0, 1.0, 0.0),
                        Incidence::new(1, 1.0, 0.0),
                        Incidence::new(2, 1.0, 0.0),
                    ],
                ),
                MagneticHyperedge::new(
                    "H1_THESIS_PATH",
                    1.0,
                    vec![Incidence::new(1, 1.0, 0.0), Incidence::new(3, 1.0, 0.0)],
                ),
                MagneticHyperedge::new(
                    "H2_ANTITHESIS_PATH",
                    1.0,
                    vec![
                        Incidence::new(2, 1.0, 0.0),
                        Incidence::new(3, 1.0, if destructive { 1.0 } else { 0.0 }),
                    ],
                ),
            ],
            0.5,
        )
    }

    pub(crate) fn pulse() -> Vec<Complex> {
        vec![
            Complex::polar(1.0, 0.0),
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
        ]
    }

    #[test]
    fn laplacian_is_hermitian() {
        let g = constructive_graph(false);
        let lap = g.laplacian();
        assert!(g.hermitian_asymmetry(&lap) < 1e-14);
        assert!(g.check_hermitian(&lap).is_ok());
    }

    #[test]
    fn cayley_preserves_the_norm() {
        let g = constructive_graph(false);
        let p = g.propagate(&pulse(), 6, 0.65).expect("propagate");
        assert!((p.norm2 - 1.0).abs() < 1e-10, "norm {}", p.norm2);
        assert!((p.initial_norm2 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn antiphase_at_the_target_cancels_the_arrival() {
        let constructive = constructive_graph(false)
            .propagate(&pulse(), 3, 0.65)
            .expect("propagate");
        let destructive = constructive_graph(true)
            .propagate(&pulse(), 3, 0.65)
            .expect("propagate");
        let a_constructive = constructive.state[3].modulus();
        let a_destructive = destructive.state[3].modulus();
        assert!(
            a_constructive > 0.15,
            "constructive amplitude {}",
            a_constructive
        );
        assert!(
            a_destructive < 1e-9,
            "destructive amplitude {}",
            a_destructive
        );
        assert!(destructive.local_coherence[3] < 0.05);
        assert!(destructive.dissonant.contains(&3));
        assert!(!constructive.dissonant.contains(&3));
    }

    #[test]
    fn chain_detects_tampering() {
        let g = constructive_graph(false);
        let steps = g.propagate_audited(&pulse(), 3, 0.65).expect("audited");
        verify_chain(&steps).expect("clean chain");
        let mut corrupted = steps.clone();
        corrupted[2].state[3].im += 0.001;
        assert!(verify_chain(&corrupted).is_err());
    }

    #[test]
    fn same_input_same_seal() {
        let g = constructive_graph(false);
        let a = g.propagate_audited(&pulse(), 4, 0.65).expect("a");
        let b = g.propagate_audited(&pulse(), 4, 0.65).expect("b");
        assert_eq!(
            a.last().map(|s| s.chain_seal),
            b.last().map(|s| s.chain_seal)
        );
    }

    #[test]
    fn state_with_wrong_size_is_refused() {
        let g = constructive_graph(false);
        assert!(g.propagate(&[Complex::ONE], 1, 0.5).is_err());
        assert!(g.propagate(&pulse(), 1, 0.0).is_err());
    }

    #[test]
    fn validation_catches_nonexistent_vertex() {
        let g = MagneticHypergraph::new(
            vec!["A"],
            vec![MagneticHyperedge::new(
                "H",
                1.0,
                vec![Incidence::new(0, 1.0, 0.0), Incidence::new(7, 1.0, 0.0)],
            )],
            0.5,
        );
        assert!(!g.validate().is_empty());
    }
}
