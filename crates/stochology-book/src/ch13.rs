//! Chapter 13 laboratory — the A1-A8 adversarial panel.
//!
//! The approval condition is the logical AND of the eight severities against the floor 6:
//!   Approved(X) = AND_{i=1..8} ( A_i(X) >= Minimum_severity ).
//! Fixed seed. No external dependencies. SplitMix64 built in.

fn splitmix64(state: &mut u64) -> u64 {
    stochology_core::rng::splitmix64_step(state)
}

fn uniform(state: &mut u64, limit: u64) -> u64 {
    splitmix64(state) % limit
}

/// The eight concerns the panel charges, one per track.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Concern {
    PropertyTest,
    HandlesMissing,
    DeclaredDrift,
    PathologiesCovered,
    IntactTrail,
    AuditedSubgroups,
    ElicitedPrior,
    ExitClause,
}

const ALL: [Concern; 8] = [
    Concern::PropertyTest,
    Concern::HandlesMissing,
    Concern::DeclaredDrift,
    Concern::PathologiesCovered,
    Concern::IntactTrail,
    Concern::AuditedSubgroups,
    Concern::ElicitedPrior,
    Concern::ExitClause,
];

impl Concern {
    fn name(self) -> &'static str {
        match self {
            Concern::PropertyTest => "A1-code",
            Concern::HandlesMissing => "A2-data",
            Concern::DeclaredDrift => "A3-lifecycle",
            Concern::PathologiesCovered => "A4-inference",
            Concern::IntactTrail => "A5-governance",
            Concern::AuditedSubgroups => "A6-fairness",
            Concern::ElicitedPrior => "A7-domain",
            Concern::ExitClause => "A8-decision",
        }
    }

    /// Ordinal that enters the mixing of the seed, as in the repository module.
    fn ordinal(self) -> u64 {
        match self {
            Concern::PropertyTest => 1,
            Concern::HandlesMissing => 2,
            Concern::DeclaredDrift => 3,
            Concern::PathologiesCovered => 4,
            Concern::IntactTrail => 5,
            Concern::AuditedSubgroups => 6,
            Concern::ElicitedPrior => 7,
            Concern::ExitClause => 8,
        }
    }

    fn seed(self, base: u64) -> u64 {
        base ^ self.ordinal().wrapping_mul(0x9E37_79B9_7F4A_7C15)
    }
}

/// Scale 0 to 10. Approval floor: 6.
const FLOOR: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Concerns {
    property_test: bool,
    handles_missing: bool,
    declared_drift: bool,
    pathologies_covered: bool,
    intact_trail: bool,
    audited_subgroups: bool,
    elicited_prior: bool,
    exit_clause: bool,
}

impl Concerns {
    fn all() -> Self {
        Concerns {
            property_test: true,
            handles_missing: true,
            declared_drift: true,
            pathologies_covered: true,
            intact_trail: true,
            audited_subgroups: true,
            elicited_prior: true,
            exit_clause: true,
        }
    }

    fn meets(&self, c: Concern) -> bool {
        match c {
            Concern::PropertyTest => self.property_test,
            Concern::HandlesMissing => self.handles_missing,
            Concern::DeclaredDrift => self.declared_drift,
            Concern::PathologiesCovered => self.pathologies_covered,
            Concern::IntactTrail => self.intact_trail,
            Concern::AuditedSubgroups => self.audited_subgroups,
            Concern::ElicitedPrior => self.elicited_prior,
            Concern::ExitClause => self.exit_clause,
        }
    }

    fn remove(&mut self, c: Concern) {
        match c {
            Concern::PropertyTest => self.property_test = false,
            Concern::HandlesMissing => self.handles_missing = false,
            Concern::DeclaredDrift => self.declared_drift = false,
            Concern::PathologiesCovered => self.pathologies_covered = false,
            Concern::IntactTrail => self.intact_trail = false,
            Concern::AuditedSubgroups => self.audited_subgroups = false,
            Concern::ElicitedPrior => self.elicited_prior = false,
            Concern::ExitClause => self.exit_clause = false,
        }
    }

    fn missing(&self) -> Vec<Concern> {
        ALL.iter().copied().filter(|c| !self.meets(*c)).collect()
    }
}

#[derive(Debug, Clone)]
struct Card {
    id: String,
    concerns: Concerns,
}

/// Deterministic severity of the attack on the card, for the given seed.
fn stress(agent: Concern, card: &Card, seed: u64) -> u8 {
    let mut rng = agent.seed(seed);
    if card.concerns.meets(agent) {
        (8 + uniform(&mut rng, 3)) as u8
    } else {
        (2 + uniform(&mut rng, 3)) as u8
    }
}

#[derive(Debug, Clone)]
struct Approval {
    items: Vec<(Concern, u8)>,
    approved: bool,
}

fn panel(card: &Card, seed: u64) -> Approval {
    let items: Vec<(Concern, u8)> = ALL.iter().map(|c| (*c, stress(*c, card, seed))).collect();
    let approved = items.iter().all(|(_, s)| *s >= FLOOR);
    Approval { items, approved }
}

impl Approval {
    fn lowest(&self) -> u8 {
        self.items.iter().map(|(_, s)| *s).min().unwrap_or(0)
    }

    fn line(&self) -> String {
        self.items
            .iter()
            .map(|(c, s)| format!("{}={}", c.name(), s))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// System 1: closed categorical triage with probability restricted to [0,01; 0,99].
#[derive(Debug, Clone)]
struct AssessmentS1 {
    label: &'static str,
    probabilities: Vec<f64>,
}

fn s1(file: &str) -> AssessmentS1 {
    let mut rng = 0u64;
    for b in file.as_bytes() {
        rng = rng.wrapping_mul(1_099_511_628_211).wrapping_add(*b as u64);
    }
    let raw = (uniform(&mut rng, 99) as f64 + 1.0) / 100.0;
    let restricted = raw.clamp(0.01, 0.99);
    AssessmentS1 {
        label: "card-in-order",
        probabilities: vec![restricted, 1.0 - restricted],
    }
}
pub fn run() {
    println!("== Chapter 13 :: the A1-A8 adversarial panel ==");

    let complete = Card {
        id: "h3.voos".to_string(),
        concerns: Concerns::all(),
    };
    let with_defect = Card {
        id: "h3.voos-com-defeito".to_string(),
        concerns: Concerns {
            intact_trail: false,
            audited_subgroups: false,
            ..Concerns::all()
        },
    };

    for (label, card) in [
        ("complete card", &complete),
        ("card with A5 and A6 removed", &with_defect),
    ] {
        let r = panel(card, 2026);
        println!(
            "{} (card {}): {} | lowest={} floor={} approved={}",
            label,
            card.id,
            r.line(),
            r.lowest(),
            FLOOR,
            r.approved
        );
    }

    // One concern removed at a time: how many agents reject and who rejects.
    let mut rejections_per_agent = [0usize; 8];
    let mut lowest_global = 10u8;
    let mut rejected_cards = 0usize;
    for c in ALL {
        let mut concerns = Concerns::all();
        concerns.remove(c);
        let f = Card {
            id: format!("card-without-{}", c.name()),
            concerns,
        };
        let r = panel(&f, 2026);
        let rejects: Vec<&'static str> = r
            .items
            .iter()
            .filter(|(_, s)| *s < FLOOR)
            .map(|(a, _)| a.name())
            .collect();
        if !r.approved {
            rejected_cards += 1;
        }
        if r.lowest() < lowest_global {
            lowest_global = r.lowest();
        }
        for (i, (_, s)) in r.items.iter().enumerate() {
            if *s < FLOOR {
                rejections_per_agent[i] += 1;
            }
        }
        let missing = f.concerns.missing();
        println!(
            "without {}: rejecting={:?} count_rejecting={} lowest={}",
            missing[0].name(),
            rejects,
            rejects.len(),
            r.lowest()
        );
    }
    println!(
        "sweep: cards rejected={} of 8 | lowest global severity={} | severities < floor per agent={:?}",
        rejected_cards, lowest_global, rejections_per_agent
    );

    // Reseeding the panel does not change the verdict.
    for seed in [7u64, 2026, 999_983] {
        let r = panel(&complete, seed);
        println!(
            "complete card with seed {}: lowest={} approved={}",
            seed,
            r.lowest(),
            r.approved
        );
    }

    // System 1: probabilities restricted to the epistemological interval.
    let s = s1("h3.voos");
    let outside = s
        .probabilities
        .iter()
        .filter(|p| **p < 0.01 || **p > 0.99)
        .count();
    println!(
        "system 1: label={} probabilities={:?} sum={:.4} outside_[0.01;0.99]={}",
        s.label,
        s.probabilities
            .iter()
            .map(|p| (p * 10_000.0).round() / 10_000.0)
            .collect::<Vec<f64>>(),
        s.probabilities.iter().sum::<f64>(),
        outside
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_approves_complete_card_and_rejects_when_a_concern_is_missing() {
        let mut f = Card {
            id: "h3.test".into(),
            concerns: Concerns::all(),
        };
        assert!(panel(&f, 2026).approved);
        f.concerns.remove(Concern::IntactTrail);
        assert!(!panel(&f, 2026).approved);
    }
}
