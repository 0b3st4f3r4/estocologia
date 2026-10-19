//! The H2 gate under the e-process, retroactively (empirical/p22-parada/DECLARACAO_2.md, 2026-10-19).
//!
//! Reads the 38 distinct tests of the gate as the multiplicity record kept them, plus the two clustered rows the
//! declaration admits, and runs one ledger per area with the rule declared before this code ran: a non-empty
//! published set contributes the mixture calibrator of its per-domain p-value; an empty set, a rejection of every
//! set, or a set accepted only by saturation contributes the calibrator at p = 1, which is one half; areas merge by
//! the mean, because their rounds reuse data; the gate passes with three areas at or above 1/alpha. Deterministic;
//! no seed.

use std::collections::BTreeMap;

use stochology_methods::sequential::{Calibrator, Ledger, Merge};

pub const TABLE: &str = "empirical/p22-parada/testes.tsv";
pub const ALPHA: f64 = 0.05;
pub const AREAS_REQUIRED: usize = 3;

#[derive(Debug, Clone)]
pub struct Test {
    pub round: String,
    pub domain: String,
    pub area: String,
    pub p_dom: f64,
    pub s_hat: String,
    pub note: String,
}

impl Test {
    /// Whether the test makes the claim the null denies: a non-empty set of parents, not by saturation.
    pub fn claims(&self) -> bool {
        self.s_hat.starts_with('{') && self.note != "saturated"
    }

    /// The p-value the ledger records: the per-domain p when the test claims, 1 otherwise.
    pub fn p(&self) -> f64 {
        if self.claims() { self.p_dom } else { 1.0 }
    }
}

pub fn load(path: &str) -> Vec<Test> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("could not read {path}"));
    text.lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            Test {
                round: f[0].to_string(),
                domain: f[1].to_string(),
                area: f[2].to_string(),
                p_dom: f[6].parse().unwrap_or(1.0),
                s_hat: f[7].to_string(),
                note: f.get(8).copied().unwrap_or("").to_string(),
            }
        })
        .collect()
}

/// The verdict of one area: its tests, the mean e-value and whether it crossed 1/alpha.
#[derive(Debug, Clone)]
pub struct AreaVerdict {
    pub area: String,
    pub tests: usize,
    pub claims: usize,
    pub mean_e: f64,
    pub passes: bool,
}

/// Runs the ledgers under one reading. `clustered` replaces the recorded row of a domain by its clustered row where
/// the table has one; otherwise the clustered rows are left out.
pub fn run_reading(tests: &[Test], clustered: bool) -> Vec<AreaVerdict> {
    let mut by_area: BTreeMap<&str, Vec<&Test>> = BTreeMap::new();
    for t in tests {
        if t.note == "clustered" {
            continue;
        }
        let replaced = clustered
            && tests
                .iter()
                .any(|c| c.note == "clustered" && c.domain == t.domain && c.round == t.round);
        if replaced {
            continue;
        }
        by_area.entry(t.area.as_str()).or_default().push(t);
    }
    if clustered {
        for c in tests.iter().filter(|c| c.note == "clustered") {
            by_area.entry(c.area.as_str()).or_default().push(c);
        }
    }
    by_area
        .into_iter()
        .map(|(area, rows)| {
            let mut ledger = Ledger::new(Merge::Mean, Calibrator::Mixture);
            for t in &rows {
                ledger.record_p(&format!("{}/{}", t.round, t.domain), t.p());
            }
            // The declaration reads the verdict at the last round: the mean is order-free, the running maximum is not.
            let verdict = ledger.verdict(ALPHA);
            AreaVerdict {
                area: area.to_string(),
                tests: rows.len(),
                claims: rows.iter().filter(|t| t.claims()).count(),
                mean_e: verdict.evidence,
                passes: verdict.evidence >= 1.0 / ALPHA,
            }
        })
        .collect()
}

pub fn run() {
    let tests = load(TABLE);
    let distinct = tests.iter().filter(|t| t.note != "clustered").count();
    println!("== The H2 gate under the e-process, retroactively ==");
    println!(
        "{distinct} distinct tests as recorded, {} clustered rows admitted by the declaration | alpha = {ALPHA} | threshold 1/alpha = {:.0} | merge: mean within each area | gate: {AREAS_REQUIRED} areas",
        tests.len() - distinct,
        1.0 / ALPHA
    );
    for (label, clustered) in [
        ("as recorded", false),
        ("with dependent rows corrected", true),
    ] {
        println!("\n-- reading: {label} --");
        println!(
            "  {:<16} {:>5} {:>6} {:>14}  verdict",
            "area", "tests", "claims", "mean e"
        );
        let verdicts = run_reading(&tests, clustered);
        for v in &verdicts {
            println!(
                "  {:<16} {:>5} {:>6} {:>14.4}  {}",
                v.area,
                v.tests,
                v.claims,
                v.mean_e,
                if v.passes {
                    "crosses 1/alpha"
                } else {
                    "does not"
                }
            );
        }
        let passing: Vec<&str> = verdicts
            .iter()
            .filter(|v| v.passes)
            .map(|v| v.area.as_str())
            .collect();
        println!(
            "  areas that cross: {} ({}) | gate under the e-process: {}",
            passing.len(),
            passing.join(", "),
            if passing.len() >= AREAS_REQUIRED {
                "PASSES"
            } else {
                "FAILS"
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(area: &str, s_hat: &str, p: f64, note: &str) -> Test {
        Test {
            round: "h2".into(),
            domain: area.to_lowercase(),
            area: area.into(),
            p_dom: p,
            s_hat: s_hat.into(),
            note: note.into(),
        }
    }

    #[test]
    fn a_test_without_a_set_makes_no_claim_and_contributes_one_half() {
        let rows = vec![t("A", "∅", 1e-9, ""), t("A", "rejeita tudo", 1e-9, "")];
        let v = run_reading(&rows, false);
        assert_eq!(v[0].claims, 0);
        assert!((v[0].mean_e - 0.5).abs() < 1e-9, "{}", v[0].mean_e);
        assert!(!v[0].passes);
    }

    #[test]
    fn a_saturated_set_makes_no_claim() {
        let rows = vec![t("A", "{x}", 1e-30, "saturated")];
        let v = run_reading(&rows, false);
        assert_eq!(v[0].claims, 0);
        assert!(!v[0].passes);
    }

    #[test]
    fn one_strong_claim_among_empty_rounds_can_carry_an_area_and_a_weak_one_cannot() {
        let strong = vec![
            t("A", "{x}", 1e-7, ""),
            t("A", "∅", 1.0, ""),
            t("A", "∅", 1.0, ""),
        ];
        let weak = vec![
            t("B", "{x}", 0.01, ""),
            t("B", "∅", 1.0, ""),
            t("B", "∅", 1.0, ""),
        ];
        assert!(run_reading(&strong, false)[0].passes);
        assert!(!run_reading(&weak, false)[0].passes);
    }

    #[test]
    fn the_clustered_row_replaces_the_recorded_one_only_in_the_second_reading() {
        let mut rec = t("A", "{x}", 1e-7, "");
        rec.round = "h2g".into();
        let mut clu = t("A", "{x}", 0.76, "clustered");
        clu.round = "h2g".into();
        let rows = vec![rec, clu];
        let first = run_reading(&rows, false);
        let second = run_reading(&rows, true);
        assert_eq!(first[0].tests, 1);
        assert!(first[0].passes);
        assert_eq!(second[0].tests, 1);
        assert!(!second[0].passes);
    }

    #[test]
    fn the_sealed_table_has_the_declared_shape() {
        // Tests run from the crate directory; the instrument runs from the workspace root.
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../",
            "empirical/p22-parada/testes.tsv"
        );
        let tests = load(path);
        assert_eq!(tests.iter().filter(|t| t.note != "clustered").count(), 38);
        assert_eq!(tests.iter().filter(|t| t.note == "clustered").count(), 2);
        assert_eq!(tests.iter().filter(|t| t.note == "saturated").count(), 1);
    }
}
