//! Point 24: resonance, not conflict (empirical/p24-ressonancia/DECLARACAO.md, sealed before this module first read
//! a dataset).
//!
//! Declaration 2 of point 23 refuted the engine's claim that deliberation pays where the phasors of a flight's
//! carrier, origin and destination conflict: in both New York vaults System 2 gained more in resonance. That reading
//! suggested a new claim, and a claim suggested by data cannot be tested on the same data. This module tests it on
//! three vaults no reading of the programme had used: New York from April to June of 2013 and of 2023, and Houston
//! from October to December of 2011. The encoding, the two systems, the regimes, the rival and the bootstrap are those
//! of declaration 2, called unchanged from `wave2.rs`; only the splits and the direction of the claim are new.

use stochology_core::Rng;

use crate::streams::{days_from_civil, load_flights};
use crate::wave::{Case, ConflictReading, MIN_STRATUM, SEED};
use crate::wave2::conflict_test_deviation;

pub const NYC_2013_H1: &str = "data/nyc_flights_2013_h1.csv";
pub const NYC_2023_H1: &str = "data/nyc_flights_2023_h1.csv";
pub const HOUSTON_2011: &str = "data/houston_flights_2011.csv";
pub const TRAINING_DAYS: i64 = 60;
pub const INTERVALS_BELOW_REQUIRED: usize = 2;

/// Cases split by the instant of emission: the 60 days before `vault_start` to learn, then the vault up to (not
/// including) `vault_end`. Dates are (year, month, day).
pub fn split_cases(
    path: &str,
    year: i64,
    vault_start: (i64, i64, i64),
    vault_end: (i64, i64, i64),
) -> (Vec<Case>, Vec<Case>) {
    let flights = load_flights(path, year);
    let start = days_from_civil(vault_start.0, vault_start.1, vault_start.2) * 1440;
    let end = days_from_civil(vault_end.0, vault_end.1, vault_end.2) * 1440;
    let train_start = start - TRAINING_DAYS * 1440;
    let case = |f: &crate::streams::Flight| Case {
        day: f.day,
        hour: f.dep_hour,
        levels: [f.carrier.clone(), f.origin.clone(), f.dest.clone()],
        y: f.y,
    };
    let train = flights
        .iter()
        .filter(|f| f.emission >= train_start && f.emission < start)
        .map(case)
        .collect();
    let vault = flights
        .iter()
        .filter(|f| f.emission >= start && f.emission < end)
        .map(case)
        .collect();
    (train, vault)
}

/// A vault: its label, its file, its year, its first day and the day after its last, as (year, month, day).
pub type Vault = (
    &'static str,
    &'static str,
    i64,
    (i64, i64, i64),
    (i64, i64, i64),
);

/// The three fresh vaults, in the order the declaration names them.
pub fn vaults() -> Vec<Vault> {
    vec![
        (
            "New York 2013, April to June",
            NYC_2013_H1,
            2013,
            (2013, 4, 1),
            (2013, 7, 1),
        ),
        (
            "New York 2023, April to June",
            NYC_2023_H1,
            2023,
            (2023, 4, 1),
            (2023, 7, 1),
        ),
        (
            "Houston 2011, October to December",
            HOUSTON_2011,
            2011,
            (2011, 10, 1),
            (2012, 1, 1),
        ),
    ]
}

/// The verdict over the three readings, by the sealed rules.
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub testable: bool,
    pub all_negative: bool,
    pub intervals_below: usize,
    pub supported: bool,
    pub beats_rival: usize,
    pub adds: bool,
}

pub fn verdict(readings: &[ConflictReading]) -> Verdict {
    let testable = !readings.is_empty()
        && readings
            .iter()
            .all(|r| r.regimes[3] >= MIN_STRATUM && r.regimes[1] >= MIN_STRATUM);
    let all_negative = readings.iter().all(|r| r.delta < 0.0);
    let intervals_below = readings.iter().filter(|r| r.ci.1 < 0.0).count();
    let beats_rival = readings.iter().filter(|r| -r.delta > r.delta_rival).count();
    Verdict {
        testable,
        all_negative,
        intervals_below,
        supported: testable && all_negative && intervals_below >= INTERVALS_BELOW_REQUIRED,
        beats_rival,
        adds: testable && beats_rival >= 2,
    }
}

/// The synthetic flights of declaration 2 (a common evening peak, twelve carriers peaking every two hours, three
/// origins at 8, 14 and 20), with `bonus` placed this time in the three carrier-origin pairs that peak at the same
/// hour: the world in which the resonance claim is true.
pub fn synthetic_resonant(seed: u64, bonus: bool, n: usize) -> Vec<Case> {
    let mut rng = Rng::new(seed);
    let origins = [8.0, 14.0, 20.0];
    let tau = 2.0 * std::f64::consts::PI;
    (0..n)
        .map(|i| {
            let c = rng.uniform(12) as usize;
            let o = rng.uniform(3) as usize;
            let d = rng.uniform(6) as usize;
            let (pc, po) = (2.0 * c as f64, origins[o]);
            let hour = 5 + rng.uniform(19) as i64;
            let h = hour as f64;
            let mut z = -1.5
                + 2.5 * (tau * (h - 19.0) / 24.0).cos()
                + (tau * (h - pc) / 24.0).cos()
                + (tau * (h - po) / 24.0).cos();
            if bonus && pc == po && (17..=21).contains(&hour) {
                z += 1.5;
            }
            let p = 1.0 / (1.0 + (-z).exp());
            let u = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            Case {
                day: (i / 700) as i64,
                hour,
                levels: [format!("C{c:02}"), format!("O{o}"), format!("D{d}")],
                y: if u < p { 1.0 } else { 0.0 },
            }
        })
        .collect()
}

fn print_reading(label: &str, r: &ConflictReading) {
    println!("\n-- {label} --");
    println!(
        "  training {} | vault flights with three eligible sources {} | quiescent {} resonance {} beating {} conflict {}",
        r.train, r.included, r.regimes[0], r.regimes[1], r.regimes[2], r.regimes[3]
    );
    println!(
        "  Brier System 1 (additive) {:.5} | System 2 (cells) {:.5}",
        r.brier_s1, r.brier_s2
    );
    println!(
        "  gain of System 2: conflict {:+.5} | resonance {:+.5} | delta {:+.5}, 95% CI by day [{:+.5}; {:+.5}]",
        r.gain_conflict, r.gain_resonance, r.delta, r.ci.0, r.ci.1
    );
    println!(
        "  rival delta (level agreement, no phase) {:+.5} | resonance beats the rival here: {}",
        r.delta_rival,
        -r.delta > r.delta_rival
    );
}

pub fn run() {
    println!("== Point 24: resonance, not conflict (empirical/p24-ressonancia/DECLARACAO.md) ==");
    println!("controls on synthetic data, seed {SEED}:");
    for (label, bonus) in [
        ("interaction in resonance", true),
        ("no interaction", false),
    ] {
        let base = synthetic_resonant(SEED, bonus, 60_000);
        let vault = synthetic_resonant(SEED + 1, bonus, 60_000);
        print_reading(
            &format!("control, {label}"),
            &conflict_test_deviation(&base, &vault, SEED),
        );
    }

    println!("\nthe three fresh vaults:");
    let mut readings = Vec::new();
    for (label, path, year, start, end) in vaults() {
        let (train, vault) = split_cases(path, year, start, end);
        let r = conflict_test_deviation(&train, &vault, SEED);
        print_reading(label, &r);
        readings.push(r);
    }
    let v = verdict(&readings);
    println!("\n== verdict, by the rules sealed before the run ==");
    println!(
        "testable in all three: {} | delta negative in all three: {} | intervals entirely below zero: {} of 3 (need {})",
        v.testable, v.all_negative, v.intervals_below, INTERVALS_BELOW_REQUIRED
    );
    println!(
        "the resonance claim: {} | adds over level agreement (in 2 of 3): {} ({} of 3)",
        if !v.testable {
            "NOT TESTABLE"
        } else if v.supported {
            "SUPPORTED"
        } else {
            "NOT SUPPORTED"
        },
        if v.adds { "yes" } else { "no" },
        v.beats_rival
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reading(delta: f64, hi: f64, rival: f64, n: usize) -> ConflictReading {
        ConflictReading {
            train: 0,
            included: 0,
            regimes: [0, n, 0, n],
            brier_s1: 0.0,
            brier_s2: 0.0,
            gain_conflict: 0.0,
            gain_resonance: 0.0,
            delta,
            ci: (delta - 0.001, hi),
            delta_rival: rival,
            testable: true,
        }
    }

    #[test]
    fn the_rule_needs_three_negatives_and_two_intervals_below_zero() {
        let ok = [
            reading(-0.002, -0.001, 0.0, 600),
            reading(-0.002, -0.0005, 0.0, 600),
            reading(-0.001, 0.0005, 0.0, 600),
        ];
        assert!(verdict(&ok).supported);
        let one_positive = [
            reading(-0.002, -0.001, 0.0, 600),
            reading(-0.002, -0.0005, 0.0, 600),
            reading(0.001, 0.002, 0.0, 600),
        ];
        assert!(!verdict(&one_positive).supported);
        let one_interval = [
            reading(-0.002, -0.001, 0.0, 600),
            reading(-0.002, 0.0005, 0.0, 600),
            reading(-0.001, 0.0005, 0.0, 600),
        ];
        assert!(!verdict(&one_interval).supported);
        let thin = [
            reading(-0.002, -0.001, 0.0, 600),
            reading(-0.002, -0.0005, 0.0, 400),
            reading(-0.001, -0.0005, 0.0, 600),
        ];
        assert!(!verdict(&thin).testable);
    }

    #[test]
    fn an_interaction_planted_in_resonance_is_found_and_its_absence_is_not() {
        let with = conflict_test_deviation(
            &synthetic_resonant(SEED, true, 60_000),
            &synthetic_resonant(SEED + 1, true, 60_000),
            SEED,
        );
        let without = conflict_test_deviation(
            &synthetic_resonant(SEED, false, 60_000),
            &synthetic_resonant(SEED + 1, false, 60_000),
            SEED,
        );
        assert!(with.ci.1 < 0.0, "{:?}", with.ci);
        assert!(
            without.ci.0 <= 0.0 && without.ci.1 >= 0.0,
            "{:?}",
            without.ci
        );
    }
}
