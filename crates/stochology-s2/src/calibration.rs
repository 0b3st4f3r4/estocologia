//! The labeled set that feeds the Brier: strict reading, minimum sample and abstention counted.
//!
//! The runner freezes answers; the calibration needs an outcome. This module reads the operator's outcome file and
//! returns the score with the rules declared before looking at the number:
//!
//! - **strict format** — `empirical/p20-jev/calibracao/desfechos.tsv`, TSV in UTF-8, header
//!   `id case_id declared_probability observed_outcome outcome_source`, `id` sequential from 1 to N;
//! - **probability in the open interval** — a strict 0.0 or 1.0 triggers `DogmaticExtreme`, because the golden rule
//!   forbids the dogmatic extremes;
//! - **minimum sample** — below `MIN_SAMPLE` the instrument returns `InsufficientSample` and does not publish a
//!   score, instead of publishing a fragile number;
//! - **abstention does not become 0.5** — the case in which the model abstained leaves the denominator of the
//!   conditional Brier and enters the abstention rate, which is reported next to the score.
//!
//! The hash of `desfechos.tsv` is declared in `calibracao/SHA256`, in the same format and with the same timing rule
//! as the weights artifact: **before** the first run.

use crate::bridge::Refusal;
use std::path::Path;

/// The exact header of the outcome file.
pub const HEADER: &str = "id	case_id	declared_probability	observed_outcome	outcome_source";
/// The name of the outcome file.
pub const FILE_NAME: &str = "desfechos.tsv";
/// The canonical directory of the labeled set.
pub const DEFAULT_DIR: &str = "empirical/p20-jev/calibracao";
/// Below this count the score is not published. A choice of specification, declared once.
pub const MIN_SAMPLE: usize = 30;
/// Buckets of the ECE. The rule is that of stochology_horizons::h3::ece, reused and not rewritten, so that there are
/// not two definitions of calibration in the same repository.
pub const ECE_BUCKETS: usize = 10;

/// One line of the labeled set.
#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    pub id: usize,
    pub case_id: String,
    pub probability: f64,
    pub outcome: bool,
    pub source: String,
}

/// The result of the calibration, or the reason there is no score.
#[derive(Debug, Clone, PartialEq)]
pub enum Calibration {
    /// Node line of data: there is no calibration without an empirical test.
    Empty,
    /// There is data, and there is no sample to publish a score.
    InsufficientSample { n: usize, minimum: usize },
    /// The conditional score, with the abstention next to it.
    Score {
        n: usize,
        brier: f64,
        ece: f64,
        abstentions: usize,
        abstention_rate: f64,
    },
}

impl Calibration {
    /// The score, when it exists.
    pub fn score(&self) -> Option<f64> {
        match self {
            Calibration::Score { brier, .. } => Some(*brier),
            _ => None,
        }
    }

    /// The reading in one line, for the instrument to print.
    pub fn reading(&self) -> String {
        match self {
            Calibration::Empty => String::from(
                "without a labeled set: no (probability, outcome) pair, Brier not published",
            ),
            Calibration::InsufficientSample { n, minimum } => format!(
                "InsufficientSample: {n} pairs, declared minimum of {minimum}; the score is not published"
            ),
            Calibration::Score {
                n,
                brier,
                ece,
                abstentions,
                abstention_rate,
            } => format!(
                "Brier {brier:.6} and ECE {ece:.6} in {ECE_BUCKETS} buckets over {n} pairs | abstentions {abstentions} ({:.1}%) outside the denominator",
                abstention_rate * 100.0
            ),
        }
    }
}

/// Reads the TSV with the field validations, in the order in which the file is read.
pub fn read_outcomes(path: &str) -> Result<Vec<Outcome>, Refusal> {
    let text =
        std::fs::read_to_string(path).map_err(|_| Refusal::WeightsMissing(path.to_string()))?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("").trim_end();
    if header != HEADER {
        return Err(Refusal::ArtifactUnreadable(format!(
            "header {header:?}, expected {HEADER:?}"
        )));
    }
    let mut output = Vec::new();
    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 5 {
            return Err(Refusal::ArtifactUnreadable(format!(
                "line {}: {} fields, expected 5",
                i + 2,
                fields.len()
            )));
        }
        let id: usize = fields[0]
            .trim()
            .parse()
            .map_err(|_| Refusal::ArtifactUnreadable(format!("invalid id on line {}", i + 2)))?;
        let expected = output.len() + 1;
        if id != expected {
            return Err(Refusal::ArtifactUnreadable(format!(
                "id out of order: {id} on line {}, expected {expected}",
                i + 2
            )));
        }
        let case_id = fields[1].trim().to_string();
        if case_id.is_empty() {
            return Err(Refusal::ArtifactUnreadable(format!(
                "empty case_id on line {}",
                i + 2
            )));
        }
        let probability: f64 = fields[2].trim().parse().map_err(|_| {
            Refusal::ArtifactUnreadable(format!("invalid probability on line {}", i + 2))
        })?;
        if !(0.0 < probability && probability < 1.0) {
            return Err(Refusal::DogmaticExtreme(format!(
                "probability {probability} on line {}",
                i + 2
            )));
        }
        let outcome = match fields[3].trim() {
            "0" => false,
            "1" => true,
            other => {
                return Err(Refusal::ArtifactUnreadable(format!(
                    "outcome {other:?} on line {}",
                    i + 2
                )));
            }
        };
        let source = fields[4].trim().to_string();
        if source.is_empty() {
            return Err(Refusal::ArtifactUnreadable(format!(
                "empty source on line {}",
                i + 2
            )));
        }
        output.push(Outcome {
            id,
            case_id,
            probability,
            outcome,
            source,
        });
    }
    Ok(output)
}

/// Writes the labeled set and its `SHA256`, in the same pattern as the weights artifact.
pub fn write_outcomes(
    dir: &Path,
    lines: &[(String, f64, bool, String)],
) -> std::io::Result<String> {
    std::fs::create_dir_all(dir)?;
    let mut text = String::from(HEADER);
    text.push('\n');
    for (i, (case_id, p, o, source)) in lines.iter().enumerate() {
        text.push_str(&format!(
            "{}	{}	{:.4}	{}	{}
",
            i + 1,
            case_id,
            p,
            if *o { 1 } else { 0 },
            source
        ));
    }
    let path = dir.join(FILE_NAME);
    std::fs::write(&path, text.as_bytes())?;
    let hash = stochology_core::audit::sha256_hex(text.as_bytes());
    std::fs::write(dir.join("SHA256"), format!("{hash}  {FILE_NAME}\n"))?;
    Ok(hash)
}

/// The conditional score: abstention leaves the denominator, and the rate goes next to it.
pub fn calibrate(outcomes: &[Outcome], abstentions: usize, minimum: usize) -> Calibration {
    if outcomes.is_empty() {
        return Calibration::Empty;
    }
    let n = outcomes.len();
    if n < minimum {
        return Calibration::InsufficientSample { n, minimum };
    }
    let pairs: Vec<(f64, bool)> = outcomes
        .iter()
        .map(|d| (d.probability, d.outcome))
        .collect();
    let brier = crate::runner::brier(&pairs);
    let confidence: Vec<f64> = pairs.iter().map(|(p, _)| *p).collect();
    let hit: Vec<bool> = pairs.iter().map(|(_, y)| *y).collect();
    let ece = stochology_horizons::h3::ece(&confidence, &hit, ECE_BUCKETS);
    let total = n + abstentions;
    let abstention_rate = abstentions as f64 / total.max(1) as f64;
    Calibration::Score {
        n,
        brier,
        ece,
        abstentions,
        abstention_rate,
    }
}

/// Loads the labeled set with the declared hash checked beforehand, as in the weights artifact.
pub fn load(dir: &str, abstentions: usize, minimum: usize) -> Result<Calibration, Refusal> {
    let expected = crate::runner::declared_hash(dir)?;
    let path = format!("{dir}/{FILE_NAME}");
    crate::bridge::verify_weights(&path, &expected)?;
    let outcomes = read_outcomes(&path)?;
    Ok(calibrate(&outcomes, abstentions, minimum))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("p20-jev-cal-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&d).ok();
        d
    }

    fn three() -> Vec<(String, f64, bool, String)> {
        vec![
            ("C-001".into(), 0.75, true, "hospital-record".into()),
            ("C-002".into(), 0.20, false, "hospital-record".into()),
            ("C-003".into(), 0.60, true, "audited-report".into()),
        ]
    }

    #[test]
    fn the_labeled_set_reads_and_the_brier_matches() {
        let d = temp("read");
        let hash = write_outcomes(&d, &three()).expect("writes");
        let outcomes = read_outcomes(&d.join(FILE_NAME).to_string_lossy()).expect("reads");
        assert_eq!(outcomes.len(), 3, "three lines");
        assert_eq!(outcomes[0].id, 1, "sequential id");
        let expected =
            ((0.75f64 - 1.0).powi(2) + (0.20f64 - 0.0).powi(2) + (0.60f64 - 1.0).powi(2)) / 3.0;
        match calibrate(&outcomes, 0, 1) {
            Calibration::Score { brier, n, .. } => {
                assert_eq!(n, 3);
                assert!(
                    (brier - expected).abs() < 1e-12,
                    "{brier} against {expected}"
                );
            }
            other => panic!("expected a score: {other:?}"),
        }
        assert_eq!(hash.len(), 64, "the manifest has 64 hex");
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn a_probability_at_the_extreme_refuses() {
        let d = temp("extreme");
        write_outcomes(&d, &[("C-1".into(), 1.0, true, "source".into())]).expect("writes");
        match read_outcomes(&d.join(FILE_NAME).to_string_lossy()) {
            Err(Refusal::DogmaticExtreme(_)) => {}
            other => panic!("expected a dogmatic extreme: {other:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn an_outcome_that_is_neither_0_nor_1_is_an_unreadable_artifact() {
        let d = temp("outcome");
        std::fs::create_dir_all(&d).ok();
        let text = format!("{HEADER}\n1\tC-1\t0.4000\t2\tsource\n");
        std::fs::write(d.join(FILE_NAME), text.as_bytes()).expect("writes");
        match read_outcomes(&d.join(FILE_NAME).to_string_lossy()) {
            Err(Refusal::ArtifactUnreadable(m)) => assert!(m.contains("outcome"), "{m}"),
            other => panic!("expected an unreadable artifact: {other:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn an_id_out_of_order_refuses() {
        let d = temp("order");
        std::fs::create_dir_all(&d).ok();
        let text = format!("{HEADER}\n2\tC-1\t0.4000\t1\tsource\n");
        std::fs::write(d.join(FILE_NAME), text.as_bytes()).expect("writes");
        match read_outcomes(&d.join(FILE_NAME).to_string_lossy()) {
            Err(Refusal::ArtifactUnreadable(m)) => assert!(m.contains("out of order"), "{m}"),
            other => panic!("expected an id out of order: {other:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn without_data_there_is_no_brier() {
        let cal = calibrate(&[], 0, MIN_SAMPLE);
        assert_eq!(cal, Calibration::Empty);
        assert!(cal.score().is_none(), "without a pair, no score");
        assert!(
            cal.reading().contains("Brier not published"),
            "{}",
            cal.reading()
        );
    }

    #[test]
    fn a_sample_below_the_minimum_does_not_publish_a_score() {
        let d = temp("minimum");
        write_outcomes(&d, &three()).expect("writes");
        let outcomes = read_outcomes(&d.join(FILE_NAME).to_string_lossy()).expect("reads");
        let cal = calibrate(&outcomes, 0, MIN_SAMPLE);
        match &cal {
            Calibration::InsufficientSample { n, minimum } => {
                assert_eq!((*n, *minimum), (3, MIN_SAMPLE))
            }
            other => panic!("expected an insufficient sample: {other:?}"),
        }
        assert!(
            cal.reading().contains("InsufficientSample"),
            "{}",
            cal.reading()
        );
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn the_abstention_leaves_the_denominator_and_enters_the_rate() {
        let d = temp("abstention");
        write_outcomes(&d, &three()).expect("writes");
        let outcomes = read_outcomes(&d.join(FILE_NAME).to_string_lossy()).expect("reads");
        let without = calibrate(&outcomes, 0, 1);
        let with = calibrate(&outcomes, 1, 1);
        assert_eq!(
            without.score(),
            with.score(),
            "the abstention does not change the conditional score"
        );
        match with {
            Calibration::Score {
                abstentions,
                abstention_rate,
                n,
                ..
            } => {
                assert_eq!((n, abstentions), (3, 1));
                assert!((abstention_rate - 0.25).abs() < 1e-12, "{abstention_rate}");
            }
            other => panic!("expected a score: {other:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn the_ece_uses_the_h3_rule_and_the_declared_buckets() {
        let one = Outcome {
            id: 1,
            case_id: "C-1".into(),
            probability: 0.3,
            outcome: false,
            source: "f".into(),
        };
        match calibrate(&[one], 0, 1) {
            Calibration::Score { ece, .. } => assert!((ece - 0.3).abs() < 1e-12, "ece {ece}"),
            other => panic!("expected a score: {other:?}"),
        }
        let a = Outcome {
            id: 1,
            case_id: "C-A".into(),
            probability: 0.55,
            outcome: true,
            source: "f".into(),
        };
        let b = Outcome {
            id: 2,
            case_id: "C-B".into(),
            probability: 0.55,
            outcome: false,
            source: "f".into(),
        };
        match calibrate(&[a, b], 0, 1) {
            Calibration::Score { ece, .. } => assert!((ece - 0.05).abs() < 1e-12, "ece {ece}"),
            other => panic!("expected a score: {other:?}"),
        }
        assert_eq!(ECE_BUCKETS, 10, "declared buckets");
        assert!(
            calibrate(&[], 0, 1)
                .reading()
                .contains("Brier not published")
        );
    }

    #[test]
    fn the_hash_gate_holds_for_the_labeled_set() {
        let d = temp("hash");
        write_outcomes(&d, &three()).expect("writes");
        assert!(
            load(&d.to_string_lossy(), 0, 1).is_ok(),
            "with a declared hash, it loads"
        );
        let path = d.join(FILE_NAME);
        let mut text = std::fs::read_to_string(&path).expect("reads");
        text.push_str("4\tC-004\t0.3000\t0\tsource\n");
        std::fs::write(&path, text.as_bytes()).expect("writes");
        match load(&d.to_string_lossy(), 0, 1) {
            Err(Refusal::WeightsHashMismatch { .. }) => {}
            other => panic!("expected a refusal by hash: {other:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }
}
