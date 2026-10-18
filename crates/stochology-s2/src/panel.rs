//! Workbench without an orchestrator: propose openly, veto expensively, close mechanically.
//!
//! The orchestrator `M` decides in three acts: it chooses the piece, distributes the lenses and signs the closure.
//! Here the three acts separate and the middle one disappears. Proposing is free, as long as it goes through the
//! card schema. Vetoing costs: it burns the finite quota of the window, it demands a falsifying artifact that
//! exists in the piece, and it records a `p_failure` bet outside the dogmatic extremes. Closing has no signature:
//! it is a pure function over the state of the workbench, and the seal of the closure is the `sha256` of the
//! canonical transcript.
//!
//! # The four rules the compiler charges
//!
//! - **G8.1** the lens -> reviewer assignment comes out identical from the same seed and the same piece;
//! - **G8.2** a veto without an artifact of the piece, or a reading without an artifact, is a protocol error, not a
//!   decision;
//! - **G8.3** a veto above the quota of the window, or a repeated veto on the same lens, is a protocol error;
//! - **G8.4** silence does not decide: before the deadline the closure is `Pending`, after the deadline without a
//!   reading quorum the closure is `Archived`, and nothing advances by inertia.
//!
//! A protocol error does not become a `Closure`: it comes out as `Err(Refusal)`, and whoever consumes the
//! workbench treats it as `Exit 1`. That is what the initial design got wrong when it returned
//! `Closure::Archived` for an exceeded quota: archiving is the destination of a piece without a complete
//! deliberation, not the punishment of a cheating reviewer.
//!
//! # Purity and audit
//!
//! `close` does not mutate budget. It returns a `Receipt` with the consumption that the decision implies, and the
//! caller applies it with `apply`, which is the only function with `&mut`. Thus the same input reproduces the same
//! seal as many times as it is run, and the quota never depends on the order in which someone decided to call the
//! function.
//!
//! Declared and not implemented: the demotion of quota by empirical reversal (the tension metric between reversal
//! and artifact opening) and the rotating window between workbenches. The workbench measures the closure; the price
//! of the reversal is a research record, and not a field of this module.

use stochology_core::audit::sha256_hex;
use stochology_core::{AgentA, Card, Rng};

/// Veto quota per reviewer and per window.
pub const VETOES_PER_WINDOW: u8 = 2;

/// Distinct minimum readings for a sustained piece to exist. Below this there is no decision: there is a file.
pub const MIN_READINGS: usize = 6;

/// Identifier of the lens: 1 to 8, in the same order as `AgentA`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lens(pub u8);

impl Lens {
    pub const ALL: [Lens; 8] = [
        Lens(1),
        Lens(2),
        Lens(3),
        Lens(4),
        Lens(5),
        Lens(6),
        Lens(7),
        Lens(8),
    ];

    /// The lens of an agent of the panel A1-A8. The lens does not change meaning by drawing.
    pub fn of_agent(agent: AgentA) -> Lens {
        match agent {
            AgentA::A1Code => Lens(1),
            AgentA::A2Data => Lens(2),
            AgentA::A3Lifecycle => Lens(3),
            AgentA::A4Inference => Lens(4),
            AgentA::A5Governance => Lens(5),
            AgentA::A6Fairness => Lens(6),
            AgentA::A7Domain => Lens(7),
            AgentA::A8Decision => Lens(8),
        }
    }

    pub fn agent(self) -> Option<AgentA> {
        AgentA::ALL
            .iter()
            .copied()
            .find(|a| Lens::of_agent(*a) == self)
    }
}

/// Who reviews. A stable number, without a title.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reviewer(pub u32);

/// Unit that absorbs the damage of the error (Phi4). It is not the person who signs: it is the room, the group, the
/// department.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit(String);

impl Unit {
    pub fn new(label: &str) -> Result<Unit, Refusal> {
        let clean = label.trim();
        if clean.is_empty() {
            return Err(Refusal::UnitMissing);
        }
        Ok(Unit(clean.to_string()))
    }

    pub fn label(&self) -> &str {
        &self.0
    }
}

/// The piece in deliberation: closed content, declared artifacts and the unit that pays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub id: String,
    pub content_hash: String,
    pub unit: Unit,
    /// Hashes that a veto or a reading may cite as material proof.
    pub artifacts: Vec<String>,
}

impl Piece {
    pub fn new(id: &str, content: &[u8], unit: Unit) -> Piece {
        Piece {
            id: id.to_string(),
            content_hash: sha256_hex(content),
            unit,
            artifacts: Vec::new(),
        }
    }

    /// Declares a citable artifact and returns the hash itself.
    pub fn with_artifact(mut self, content: &[u8]) -> (Piece, String) {
        let hash = sha256_hex(content);
        if !self.artifacts.contains(&hash) {
            self.artifacts.push(hash.clone());
        }
        (self, hash)
    }

    /// Converts the card of the contract into a piece. Without `quem_paga_pelo_erro`, the piece does not enter the
    /// workbench.
    pub fn from_card(card: &Card) -> Result<Piece, Refusal> {
        let unit = Unit::new(card.who_pays_for_the_error.as_deref().unwrap_or(""))?;
        let canonical = format!(
            "{}|{}|{}|{}|{}",
            card.id,
            card.layer.code(),
            card.uncertainty.code(),
            card.definition,
            card.decision_maker_question
        );
        let mut piece = Piece::new(&card.id, canonical.as_bytes(), unit);
        if let Some(artifact) = &card.simulable_artifact {
            piece = piece.with_artifact(artifact.as_bytes()).0;
        }
        Ok(piece)
    }

    pub fn knows(&self, artifact: &str) -> bool {
        artifact == self.content_hash || self.artifacts.iter().any(|a| a == artifact)
    }
}

/// Assignment published before the reading: the lens and the reviewer who occupies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub lens: Lens,
    pub reviewer: Reviewer,
}

/// Registered reading: the reviewer of the seat read the artifact, and the hash proves which one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub reviewer: Reviewer,
    pub lens: Lens,
    pub artifact: String,
}

impl Reading {
    pub fn new(reviewer: Reviewer, lens: Lens, artifact: &str) -> Reading {
        Reading {
            reviewer,
            lens,
            artifact: artifact.to_string(),
        }
    }
}

/// Veto: expensive, material and bet on. Without all three, it is a protocol error.
#[derive(Clone, Debug, PartialEq)]
pub struct Veto {
    pub reviewer: Reviewer,
    pub lens: Lens,
    /// Hash of the artifact that falsifies the piece.
    pub falsifying_artifact: String,
    /// Declared probability that the piece fails the test. Outside (0, 1) it is dogma, and it does not enter.
    pub p_failure: f64,
}

impl Veto {
    pub fn new(reviewer: Reviewer, lens: Lens, falsifying_artifact: &str, p_failure: f64) -> Veto {
        Veto {
            reviewer,
            lens,
            falsifying_artifact: falsifying_artifact.to_string(),
            p_failure,
        }
    }
}

/// Veto quota of a reviewer in the current window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Authority {
    pub reviewer: Reviewer,
    pub window: u32,
    pub vetoes_left: u8,
}

impl Authority {
    pub fn new(reviewer: Reviewer, window: u32) -> Authority {
        Authority {
            reviewer,
            window,
            vetoes_left: VETOES_PER_WINDOW,
        }
    }
}

/// Protocol error. It is not a closure: it is `Exit 1` for whoever consumes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// G8.1: workbench smaller than the eight lenses.
    WorkbenchInsufficient { present: usize },
    /// G8.2: the artifact cited does not exist in the piece.
    ArtifactUnknown { lens: Lens },
    /// G8.3: veto above the quota of the window.
    QuotaExhausted { reviewer: Reviewer },
    /// G8.3: two vetoes of the same reviewer on the same lens.
    DuplicateVeto { reviewer: Reviewer },
    /// G8.5: a bet at 0 or 1, or outside the open interval.
    DogmaticBet { reviewer: Reviewer },
    /// G8.7: the lens does not belong to this reviewer in the published assignment.
    LensOutOfAssignment { lens: Lens, reviewer: Reviewer },
    /// G8.4/Phi4: a veto or a decision without a unit that pays.
    UnitMissing,
    /// Vote of someone who is neither in the assignment nor in the quota.
    NoAuthority { reviewer: Reviewer },
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::WorkbenchInsufficient { .. } => "G8.1",
            Refusal::ArtifactUnknown { .. } => "G8.2",
            Refusal::QuotaExhausted { .. } => "G8.3",
            Refusal::DuplicateVeto { .. } => "G8.3",
            Refusal::DogmaticBet { .. } => "G8.5",
            Refusal::LensOutOfAssignment { .. } => "G8.7",
            Refusal::UnitMissing => "Phi4",
            Refusal::NoAuthority { .. } => "G8.3",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::WorkbenchInsufficient { present } => {
                format!("workbench with {present} reviewers; eight lenses demand eight bodies")
            }
            Refusal::ArtifactUnknown { lens } => {
                format!(
                    "lens {} cited an artifact that the piece does not declare",
                    lens.0
                )
            }
            Refusal::QuotaExhausted { reviewer } => {
                format!(
                    "reviewer {} exceeded the veto quota of the window",
                    reviewer.0
                )
            }
            Refusal::DuplicateVeto { reviewer } => {
                format!("reviewer {} vetoed the same lens twice", reviewer.0)
            }
            Refusal::DogmaticBet { reviewer } => {
                format!("reviewer {} bet 0 or 1: dogmatic extreme", reviewer.0)
            }
            Refusal::LensOutOfAssignment { lens, reviewer } => format!(
                "lens {} does not belong to reviewer {} in the published assignment",
                lens.0, reviewer.0
            ),
            Refusal::UnitMissing => {
                "piece without a unit that absorbs the error (Phi4)".to_string()
            }
            Refusal::NoAuthority { reviewer } => {
                format!(
                    "reviewer {} has neither a seat nor a quota in this workbench",
                    reviewer.0
                )
            }
        }
    }
}

/// Closure: what the workbench decided. It has no human signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Closure {
    /// Deadline open and no valid veto: nothing decided yet.
    Pending,
    /// Node valid veto, deadline expired and reading quorum met.
    Upheld { seal: String, readings: usize },
    /// One valid veto is enough: the conjunction of the panel A1-A8 was already like this.
    NotUpheld { reason: String, vetoes: usize },
    /// Deadline expired without a reading quorum, or a piece with no condition to deliberate.
    Archived { reason: String },
}

impl Closure {
    pub fn code(&self) -> &'static str {
        match self {
            Closure::Pending => "pending",
            Closure::Upheld { .. } => "upheld",
            Closure::NotUpheld { .. } => "not-upheld",
            Closure::Archived { .. } => "archived",
        }
    }

    pub fn decides(&self) -> bool {
        !matches!(self, Closure::Pending)
    }
}

/// What the closure cost: quota consumption to be applied outside the pure function, and the sealed transcript.
#[derive(Clone, Debug, PartialEq)]
pub struct Receipt {
    pub consumption: Vec<(Reviewer, u8)>,
    pub valid_readings: usize,
    pub valid_vetoes: Vec<Reviewer>,
    pub transcript_seal: String,
}

impl Receipt {
    pub fn markdown_line(&self, closure: &Closure) -> String {
        format!(
            "| {} | {} | {} | {} | {} |",
            closure.code(),
            self.valid_readings,
            self.valid_vetoes.len(),
            self.consumption
                .iter()
                .map(|(a, c)| format!("{}:-{}", a.0, c))
                .collect::<Vec<_>>()
                .join(" "),
            &self.transcript_seal[..12]
        )
    }
}

/// State of the workbench in one round: published assignment, readings, vetoes, quotas and the deadline clock.
#[derive(Clone, Debug, PartialEq)]
pub struct WorkbenchState {
    pub assignments: [Assignment; 8],
    pub readings: Vec<Reading>,
    pub vetoes: Vec<Veto>,
    pub budgets: Vec<Authority>,
    pub deadline_expired: bool,
}

impl WorkbenchState {
    pub fn new(assignments: [Assignment; 8], window: u32) -> WorkbenchState {
        let mut budgets: Vec<Authority> = Vec::new();
        for d in assignments {
            if !budgets.iter().any(|a| a.reviewer == d.reviewer) {
                budgets.push(Authority::new(d.reviewer, window));
            }
        }
        WorkbenchState {
            assignments,
            readings: Vec::new(),
            vetoes: Vec::new(),
            budgets,
            deadline_expired: false,
        }
    }

    pub fn seat(&self, lens: Lens) -> Option<Reviewer> {
        self.assignments
            .iter()
            .find(|d| d.lens == lens)
            .map(|d| d.reviewer)
    }

    pub fn quota(&self, reviewer: Reviewer) -> u8 {
        self.budgets
            .iter()
            .find(|a| a.reviewer == reviewer)
            .map(|a| a.vetoes_left)
            .unwrap_or(0)
    }

    pub fn read(mut self, reading: Reading) -> Self {
        self.readings.push(reading);
        self
    }

    pub fn veto(mut self, veto: Veto) -> Self {
        self.vetoes.push(veto);
        self
    }

    pub fn with_expired_deadline(mut self) -> Self {
        self.deadline_expired = true;
        self
    }
}

/// Audited draw of the assignment: Fisher-Yates over the reviewers, seed mixed with the piece.
///
/// The lens is not drawn: lens 1 remains the care about code. What is drawn is the body that occupies each seat.
/// Same piece, same seed, same matrix down to the last bit.
pub fn assign(
    piece: &Piece,
    seed: u64,
    reviewers: &[Reviewer],
) -> Result<[Assignment; 8], Refusal> {
    let mut pool: Vec<Reviewer> = reviewers.to_vec();
    pool.sort();
    pool.dedup();
    if pool.len() < Lens::ALL.len() {
        return Err(Refusal::WorkbenchInsufficient {
            present: pool.len(),
        });
    }

    let mut rng = Rng::new(mix(seed, &piece.content_hash));
    for i in (1..pool.len()).rev() {
        let j = rng.uniform((i + 1) as u64) as usize;
        pool.swap(i, j);
    }

    let mut assignments = [Assignment {
        lens: Lens(1),
        reviewer: Reviewer(0),
    }; 8];
    for (index, seat) in assignments.iter_mut().enumerate() {
        *seat = Assignment {
            lens: Lens(index as u8 + 1),
            reviewer: pool[index],
        };
    }
    Ok(assignments)
}

fn mix(seed: u64, hash_hex: &str) -> u64 {
    let mut h = seed ^ 0x9E37_79B9_7F4A_7C15;
    for byte in hash_hex.as_bytes() {
        h = (h.rotate_left(5) ^ (*byte as u64)).wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

/// Pure function of closure: it validates the round, returns the closure and the receipt. Node state, no arbiter.
///
/// The order of the checks is the order of intransigence: first the round is valid (seat, artifact, bet, quota);
/// then one valid veto brings it down; then the deadline; then the quorum. Node path returns `Upheld` by inertia.
pub fn close(piece: &Piece, state: &WorkbenchState) -> Result<(Closure, Receipt), Refusal> {
    let mut valid_readings = 0usize;
    let mut lenses_read: Vec<Lens> = Vec::new();

    for reading in &state.readings {
        require_seat(state, reading.lens, reading.reviewer)?;
        if !piece.knows(&reading.artifact) {
            return Err(Refusal::ArtifactUnknown { lens: reading.lens });
        }
        if !lenses_read.contains(&reading.lens) {
            lenses_read.push(reading.lens);
            valid_readings += 1;
        }
    }

    let mut quotas: Vec<(Reviewer, u8)> = state
        .budgets
        .iter()
        .map(|a| (a.reviewer, a.vetoes_left))
        .collect();
    let mut valid_vetoes: Vec<Reviewer> = Vec::new();
    let mut vetoed_lenses: Vec<Lens> = Vec::new();

    for veto in &state.vetoes {
        require_seat(state, veto.lens, veto.reviewer)?;
        if !piece.knows(&veto.falsifying_artifact) {
            return Err(Refusal::ArtifactUnknown { lens: veto.lens });
        }
        if !(veto.p_failure > 0.0 && veto.p_failure < 1.0) {
            return Err(Refusal::DogmaticBet {
                reviewer: veto.reviewer,
            });
        }
        if vetoed_lenses.contains(&veto.lens) {
            return Err(Refusal::DuplicateVeto {
                reviewer: veto.reviewer,
            });
        }
        let Some(quota) = quotas.iter_mut().find(|(a, _)| *a == veto.reviewer) else {
            return Err(Refusal::NoAuthority {
                reviewer: veto.reviewer,
            });
        };
        if quota.1 == 0 {
            return Err(Refusal::QuotaExhausted {
                reviewer: veto.reviewer,
            });
        }
        quota.1 -= 1;
        vetoed_lenses.push(veto.lens);
        valid_vetoes.push(veto.reviewer);
    }

    let mut consumption: Vec<(Reviewer, u8)> = Vec::new();
    for (reviewer, remaining) in &quotas {
        let initial = state.quota(*reviewer);
        if *remaining < initial {
            consumption.push((*reviewer, initial - *remaining));
        }
    }

    let seal = seal(piece, state, valid_readings, &valid_vetoes);
    let receipt = Receipt {
        consumption,
        valid_readings,
        valid_vetoes: valid_vetoes.clone(),
        transcript_seal: seal.clone(),
    };

    if !valid_vetoes.is_empty() {
        return Ok((
            Closure::NotUpheld {
                reason: "valid veto with material proof and a burned quota".to_string(),
                vetoes: valid_vetoes.len(),
            },
            receipt,
        ));
    }

    if !state.deadline_expired {
        return Ok((Closure::Pending, receipt));
    }

    if valid_readings < MIN_READINGS {
        return Ok((
            Closure::Archived {
                reason: format!(
                    "deadline expired with {valid_readings} of {MIN_READINGS} readings required"
                ),
            },
            receipt,
        ));
    }

    Ok((
        Closure::Upheld {
            seal,
            readings: valid_readings,
        },
        receipt,
    ))
}

fn require_seat(state: &WorkbenchState, lens: Lens, reviewer: Reviewer) -> Result<(), Refusal> {
    match state.seat(lens) {
        Some(seat) if seat == reviewer => Ok(()),
        _ => Err(Refusal::LensOutOfAssignment { lens, reviewer }),
    }
}

fn seal(piece: &Piece, state: &WorkbenchState, readings: usize, vetoes: &[Reviewer]) -> String {
    let mut seats: Vec<String> = state
        .assignments
        .iter()
        .map(|d| format!("{}:{}", d.lens.0, d.reviewer.0))
        .collect();
    seats.sort();

    let mut bets: Vec<String> = state
        .vetoes
        .iter()
        .map(|v| format!("{}:{}:{:?}", v.reviewer.0, v.lens.0, v.p_failure))
        .collect();
    bets.sort();

    let mut voters: Vec<u32> = vetoes.iter().map(|a| a.0).collect();
    voters.sort();

    let body = format!(
        "piece={};content={};unit={};artifacts={};seats={};readings={};voters={};bets={};deadline={}",
        piece.id,
        piece.content_hash,
        piece.unit.label(),
        piece.artifacts.join(","),
        seats.join(","),
        readings,
        voters
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(","),
        bets.join(","),
        state.deadline_expired
    );
    sha256_hex(body.as_bytes())
}

/// Applies the consumption of the receipt to the quotas. The only function with `&mut` in the workbench.
pub fn apply(receipt: &Receipt, budgets: &mut [Authority]) -> Result<(), Refusal> {
    for (reviewer, consumption) in &receipt.consumption {
        match budgets.iter_mut().find(|a| a.reviewer == *reviewer) {
            Some(a) if a.vetoes_left >= *consumption => a.vetoes_left -= *consumption,
            Some(_) => {
                return Err(Refusal::QuotaExhausted {
                    reviewer: *reviewer,
                });
            }
            None => {
                return Err(Refusal::NoAuthority {
                    reviewer: *reviewer,
                });
            }
        }
    }
    Ok(())
}

/// The whole workbench: piece, published assignment and current state. What the pilot runs for 14 days.
#[derive(Clone, Debug, PartialEq)]
pub struct Workbench {
    pub piece: Piece,
    pub seed: u64,
    pub state: WorkbenchState,
}

impl Workbench {
    pub fn new(
        piece: Piece,
        seed: u64,
        reviewers: &[Reviewer],
        window: u32,
    ) -> Result<Workbench, Refusal> {
        let assignments = assign(&piece, seed, reviewers)?;
        let state = WorkbenchState::new(assignments, window);
        Ok(Workbench { piece, seed, state })
    }

    pub fn close(&self) -> Result<(Closure, Receipt), Refusal> {
        close(&self.piece, &self.state)
    }

    /// Closes and consumes: it is the only path that spends quota, and it spends it in one go.
    pub fn close_and_consume(&mut self) -> Result<(Closure, Receipt), Refusal> {
        let (closure, receipt) = self.close()?;
        apply(&receipt, &mut self.state.budgets)?;
        Ok((closure, receipt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reviewers() -> Vec<Reviewer> {
        (1..=10).map(Reviewer).collect()
    }

    fn piece() -> (Piece, String) {
        let unit = Unit::new("room-3").expect("unit of the room");
        let p = Piece::new("p-2026-01", b"test-content", unit);
        p.with_artifact(b"test-counterproof")
    }

    fn workbench(p: Piece, seed: u64) -> Workbench {
        Workbench::new(p, seed, &reviewers(), 1).expect("workbench with ten bodies")
    }

    fn read_all(b: &Workbench) -> WorkbenchState {
        let mut state = b.state.clone();
        for d in state.assignments {
            let reading = Reading::new(d.reviewer, d.lens, &b.piece.content_hash);
            state = state.read(reading);
        }
        state
    }

    fn veto_of(b: &Workbench, index: usize, artifact: &str, p_failure: f64) -> Veto {
        let d = b.state.assignments[index];
        Veto::new(d.reviewer, d.lens, artifact, p_failure)
    }

    #[test]
    fn g8_1_the_assignment_is_reproducible() {
        let (p, _) = piece();
        let a = assign(&p, 2026, &reviewers()).expect("assigns");
        let b = assign(&p, 2026, &reviewers()).expect("assigns");
        assert_eq!(a, b);
    }

    #[test]
    fn g8_1_lens_preserved_and_distinct_bodies() {
        let (p, _) = piece();
        let d = assign(&p, 7, &reviewers()).expect("assigns");
        for (index, seat) in d.iter().enumerate() {
            assert_eq!(seat.lens, Lens(index as u8 + 1));
        }
        let mut bodies: Vec<Reviewer> = d.iter().map(|x| x.reviewer).collect();
        bodies.sort();
        bodies.dedup();
        assert_eq!(bodies.len(), 8);
    }

    #[test]
    fn g8_1_a_different_seed_changes_the_matrix() {
        let (p, _) = piece();
        let base = assign(&p, 1, &reviewers()).expect("assigns");
        let diverges = (2..40u64)
            .map(|s| assign(&p, s, &reviewers()).expect("assigns"))
            .any(|m| m != base);
        assert!(
            diverges,
            "different seeds should always draw another matrix"
        );
    }

    #[test]
    fn g8_1_a_short_workbench_refuses() {
        let (p, _) = piece();
        let short: Vec<Reviewer> = (1..=7).map(Reviewer).collect();
        let error = assign(&p, 1, &short).expect_err("seven bodies do not cover eight lenses");
        assert_eq!(error.code(), "G8.1");
    }

    #[test]
    fn g8_2_a_veto_with_an_unknown_artifact_refuses() {
        let (p, _) = piece();
        let b = workbench(p, 3);
        let state = b
            .state
            .clone()
            .veto(veto_of(&b, 0, "hash-that-does-not-exist", 0.7));
        let error = close(&b.piece, &state).expect_err("veto without material proof");
        assert_eq!(error.code(), "G8.2");
    }

    #[test]
    fn g8_2_a_reading_without_an_artifact_refuses() {
        let (p, _) = piece();
        let b = workbench(p, 3);
        let d = b.state.assignments[2];
        let state = b
            .state
            .clone()
            .read(Reading::new(d.reviewer, d.lens, "made-up"));
        let error = close(&b.piece, &state).expect_err("reading of a nonexistent artifact");
        assert_eq!(error.code(), "G8.2");
    }

    #[test]
    fn g8_3_an_exhausted_quota_refuses() {
        let (p, artifact) = piece();
        let b = workbench(p, 5);
        let target = b.state.assignments[0].reviewer;

        // The quota belongs to the window, not to the round: two earlier pieces already burned the two vetoes.
        let mut state = b.state.clone().veto(veto_of(&b, 0, &artifact, 0.6));
        let (_, first) = close(&b.piece, &state).expect("the first veto holds");
        apply(&first, &mut state.budgets).expect("burns the first quota");
        state.vetoes.clear();
        state = state.veto(veto_of(&b, 0, &artifact, 0.65));
        let (_, second) = close(&b.piece, &state).expect("the second veto holds");
        apply(&second, &mut state.budgets).expect("burns the second quota");
        state.readings.clear();
        state.vetoes = vec![veto_of(&b, 0, &artifact, 0.7)];

        let error = close(&b.piece, &state).expect_err("the quota of the window is already zero");
        assert_eq!(error.code(), "G8.3");
        assert_eq!(state.quota(target), 0);
    }

    #[test]
    fn g8_3_a_full_window_quota_zeroes_the_power_of_veto() {
        let (p, artifact) = piece();
        let b = workbench(p, 5);
        let mut state = b.state.clone();
        let target = state.assignments[3].reviewer;
        let quota = state
            .budgets
            .iter_mut()
            .find(|a| a.reviewer == target)
            .expect("seat");
        quota.vetoes_left = 0;
        state = state.veto(veto_of(&b, 3, &artifact, 0.8));

        let error = close(&b.piece, &state).expect_err("veto from whoever already spent the quota");
        assert_eq!(error.code(), "G8.3");
    }

    #[test]
    fn g8_3_a_duplicate_veto_refuses() {
        let (p, artifact) = piece();
        let b = workbench(p, 5);
        let state = b
            .state
            .clone()
            .veto(veto_of(&b, 0, &artifact, 0.6))
            .veto(veto_of(&b, 0, &artifact, 0.55));
        let error = close(&b.piece, &state).expect_err("two vetoes on the same lens");
        assert_eq!(error.code(), "G8.3");
    }

    #[test]
    fn g8_5_a_dogmatic_bet_refuses() {
        let (p, artifact) = piece();
        let b = workbench(p, 11);
        for bet in [0.0, 1.0, -0.2, 1.4, f64::NAN] {
            let state = b.state.clone().veto(veto_of(&b, 0, &artifact, bet));
            let error = close(&b.piece, &state).expect_err("dogmatic extreme in the veto");
            assert_eq!(error.code(), "G8.5");
        }
    }

    #[test]
    fn g8_7_a_lens_outside_the_assignment_refuses() {
        let (p, artifact) = piece();
        let b = workbench(p, 13);
        let d0 = b.state.assignments[0];
        let d1 = b.state.assignments[1];
        let intrusion = Veto::new(d1.reviewer, d0.lens, &artifact, 0.5);
        let state = b.state.clone().veto(intrusion);
        let error = close(&b.piece, &state).expect_err("vote on someone else's lens");
        assert_eq!(error.code(), "G8.7");
    }

    #[test]
    fn g8_4_silence_before_the_deadline_is_pending() {
        let (p, _) = piece();
        let b = workbench(p, 17);
        let (closure, receipt) = b.close().expect("valid round");
        assert_eq!(closure, Closure::Pending);
        assert!(receipt.consumption.is_empty());
        assert!(!closure.decides());
    }

    #[test]
    fn g8_4_silence_after_the_deadline_archives() {
        let (p, _) = piece();
        let b = workbench(p, 17);
        let state = b.state.clone().with_expired_deadline();
        let (closure, _) = close(&b.piece, &state).expect("valid round");
        match closure {
            Closure::Archived { reason } => assert!(reason.contains("deadline expired")),
            other => panic!("silence with an expired deadline does not uphold: {other:?}"),
        }
    }

    #[test]
    fn g8_4_a_full_quorum_upholds() {
        let (p, _) = piece();
        let b = workbench(p, 19);
        let state = read_all(&b).with_expired_deadline();
        let (closure, receipt) = close(&b.piece, &state).expect("valid round");
        assert_eq!(receipt.valid_readings, 8);
        assert!(matches!(closure, Closure::Upheld { readings: 8, .. }));
    }

    #[test]
    fn g8_4_an_insufficient_quorum_archives() {
        let (p, _) = piece();
        let b = workbench(p, 19);
        let mut state = b.state.clone();
        for d in b.state.assignments.iter().take(MIN_READINGS - 1) {
            state = state.read(Reading::new(d.reviewer, d.lens, &b.piece.content_hash));
        }
        let state = state.with_expired_deadline();
        let (closure, _) = close(&b.piece, &state).expect("valid round");
        assert!(matches!(closure, Closure::Archived { .. }));
    }

    #[test]
    fn g8_6_a_valid_veto_brings_it_down_and_consumes() {
        let (p, artifact) = piece();
        let mut b = workbench(p, 23);
        b.state = b.state.clone().veto(veto_of(&b, 4, &artifact, 0.72));
        let target = b.state.assignments[4].reviewer;
        let before = b.state.quota(target);

        let (closure, receipt) = b.close_and_consume().expect("valid round");
        assert!(matches!(closure, Closure::NotUpheld { vetoes: 1, .. }));
        assert_eq!(receipt.consumption, vec![(target, 1)]);
        assert_eq!(b.state.quota(target), before - 1);
    }

    #[test]
    fn g8_6_the_veto_wins_even_without_quorum() {
        let (p, artifact) = piece();
        let b = workbench(p, 29);
        let state = b.state.clone().veto(veto_of(&b, 0, &artifact, 0.9));
        let (closure, _) = close(&b.piece, &state).expect("valid round");
        assert_eq!(closure.code(), "not-upheld");
    }

    #[test]
    fn g8_8_the_seal_is_deterministic_and_sensitive() {
        let (p, _) = piece();
        let b = workbench(p, 31);
        let state = read_all(&b).with_expired_deadline();
        let (f1, r1) = close(&b.piece, &state).expect("valid round");
        let (f2, r2) = close(&b.piece, &state).expect("valid round");
        assert_eq!(f1, f2);
        assert_eq!(r1.transcript_seal, r2.transcript_seal);

        let mut tampered = b.piece.clone();
        tampered.artifacts.push(sha256_hex(b"another-artifact"));
        let (_, r3) = close(&tampered, &state).expect("valid round");
        assert_ne!(r1.transcript_seal, r3.transcript_seal);
    }

    #[test]
    fn g8_8_apply_outside_the_pure_function() {
        let (p, artifact) = piece();
        let b = workbench(p, 37);
        let state = b.state.clone().veto(veto_of(&b, 1, &artifact, 0.4));
        let (_, receipt) = close(&b.piece, &state).expect("valid round");

        let mut budgets = b.state.budgets.clone();
        apply(&receipt, &mut budgets).expect("consumes once");
        apply(&receipt, &mut budgets).expect("consumes the second quota of the window");
        let error = apply(&receipt, &mut budgets).expect_err("quota already consumed");
        assert_eq!(error.code(), "G8.3");
    }

    #[test]
    fn phi4_a_piece_without_a_unit_refuses() {
        let error = Unit::new("   ").expect_err("empty unit");
        assert_eq!(error.code(), "Phi4");
    }

    #[test]
    fn phi4_a_card_without_whoever_pays_refuses() {
        use stochology_core::{Layer, UncertaintyType};
        let card = Card::new("f-1", Layer::Model, UncertaintyType::Epistemic);
        let error = Piece::from_card(&card).expect_err("card without whoever pays");
        assert_eq!(error.code(), "Phi4");
    }

    #[test]
    fn the_receipt_produces_a_line_without_an_opinion() {
        let (p, _) = piece();
        let b = workbench(p, 41);
        let state = read_all(&b).with_expired_deadline();
        let (closure, receipt) = close(&b.piece, &state).expect("valid round");
        let line = receipt.markdown_line(&closure);
        assert!(line.starts_with("| upheld | 8 | 0 |"));
    }
}
