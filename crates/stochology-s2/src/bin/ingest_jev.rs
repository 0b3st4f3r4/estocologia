//! The ingestor of item 13: it prepares the weights artifact and the labeled set of the Brier.
//!
//! Outside the production library on purpose: it is an operator tool, not a piece of the apparatus. It takes the
//! paths explicitly, it embeds neither a number nor a data file name, and it validates before persisting: every
//! source goes through a field check, the result is written in a rehearsal directory and read back by the same gates
//! that the bridge uses (declared hash, format, sequential id, open interval of the probability) and only then goes
//! to the destination. If anything fails, the destination is not touched.
//!
//! Usage:
//!   ingest-jev --check <source_inferences.tsv> <source_outcomes.tsv>
//!   ingest-jev <weights_dir> <source_inferences.tsv> <calibration_dir> <source_outcomes.tsv>
//!   ingest-jev --record <measure.txt> <rounds.tsv> --date YYYY-MM-DD
//!
//! The --check mode is phase 1 of the closure: it validates both sources, runs the rehearsal and writes to no
//! destination. It serves the operator to know, before any persistence, whether the set has volume, interval and
//! provenance for the declared criterion.

use std::env;
use std::path::{Path, PathBuf};
use std::process;

use stochology_s2::bridge::validate_output;
use stochology_s2::{calibration, runner};

fn usage_and_abort(program: &str) -> ! {
    eprintln!("Usage:");
    eprintln!("  {program} --check <source_inferences.tsv> <source_outcomes.tsv>");
    eprintln!("  {program} --record <measure.txt> <rounds.tsv> --date YYYY-MM-DD");
    eprintln!(
        "  {program} <weights_dir> <source_inferences.tsv> <calibration_dir> <source_outcomes.tsv>"
    );
    eprintln!();
    eprintln!("Arguments:");
    eprintln!(
        "  weights_dir              destination of the artifact (e.g.: empirical/p20-jev/pesos)"
    );
    eprintln!("  source_inferences.tsv    request-TAB-answer lines, one per frozen case");
    eprintln!(
        "  calibration_dir          destination of the labeled set (e.g.: empirical/p20-jev/calibracao)"
    );
    eprintln!("  source_outcomes.tsv      case_id-TAB-prob-TAB-outcome-TAB-source lines");
    eprintln!();
    eprintln!(
        "Nothing is written to the destination before the rehearsal passes the gates of the bridge."
    );
    process::exit(1);
}

fn die(message: String) -> ! {
    eprintln!("{message}");
    process::exit(1);
}

/// Reads the frozen inferences, with two fields and nothing empty.
/// The provenance line of the inference source: the hash of the model file and its revision.
const MODEL_LINE: &str = "# model-sha256\t";

type Inferences = (Vec<(String, String)>, Option<runner::ModelFile>);

fn read_inferences(path: &Path) -> Inferences {
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        die(format!(
            "Error opening the inference source {}: {e}",
            path.display()
        ))
    });
    let mut pairs = Vec::new();
    let mut model: Option<runner::ModelFile> = None;
    for (i, line) in content.lines().enumerate() {
        let l = line.trim_end();
        if let Some(rest) = l.strip_prefix(MODEL_LINE) {
            if model.is_some() {
                die(format!(
                    "Error on line {} of {}: more than one model line",
                    i + 1,
                    path.display()
                ));
            }
            let (sha, revision) = rest.split_once('\t').unwrap_or((rest, ""));
            model = Some(runner::ModelFile::new(sha, revision).unwrap_or_else(|e| {
                die(format!(
                    "Error on line {} of {}: {e:?}",
                    i + 1,
                    path.display()
                ))
            }));
            continue;
        }
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() != 2 {
            die(format!(
                "Error on line {} of {}: expected two fields separated by TAB, found {}",
                i + 1,
                path.display(),
                fields.len()
            ));
        }
        if fields[0].trim().is_empty() || fields[1].trim().is_empty() {
            die(format!(
                "Error on line {} of {}: empty field",
                i + 1,
                path.display()
            ));
        }
        // The answer can have more than one line (the canonical document of the card does). In the source, which is
        // line by line, the break comes escaped as the sequence of two characters slash-n, and the ingestor undoes
        // the escape here. Without the escape there would not be two ambiguous lines.
        // Both columns come escaped from the source. The request too: if only the answer were unescaped, the key of
        // the artifact (the sha of the request text) would stop matching the stored text, and the bridge would
        // abstain in 100% of the cases without saying why. That is how the first round with the real model came out
        // 32 of 32 valid in the check and 100% abstention in the instrument.
        pairs.push((
            fields[0].replace("\\n", "\n"),
            fields[1].replace("\\n", "\n"),
        ));
    }
    if pairs.is_empty() {
        die(format!("{} has no inference at all", path.display()));
    }
    (pairs, model)
}

fn print_model(model: &Option<runner::ModelFile>) {
    match model {
        Some(m) => println!("  model file: {} | revision {}", m.sha256, m.revision),
        None => println!(
            "  model file: not declared | the artifact will run, and the round will not enter the record"
        ),
    }
}

/// Reads the outcomes, with the rules of the front applied before any write.
fn read_outcomes(path: &Path) -> Vec<(String, f64, bool, String)> {
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        die(format!(
            "Error opening the outcome source {}: {e}",
            path.display()
        ))
    });
    let mut pairs = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let l = line.trim_end();
        // The upstream generator still emits its header with the legacy keys; the header is skipped so that the
        // source keeps entering the ramp unchanged.
        if l.is_empty()
            || l.starts_with('#')
            || l.starts_with("case_id\t")
            || l.starts_with("caso_id\t")
        {
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() != 4 {
            die(format!(
                "Error on line {} of {}: expected four fields separated by TAB, found {}",
                i + 1,
                path.display(),
                fields.len()
            ));
        }
        if fields[0].trim().is_empty() || fields[3].trim().is_empty() {
            die(format!(
                "Error on line {} of {}: empty case_id or source",
                i + 1,
                path.display()
            ));
        }
        let prob: f64 = fields[1].trim().parse().unwrap_or_else(|_| {
            die(format!(
                "Line {}: invalid probability '{}'",
                i + 1,
                fields[1]
            ))
        });
        if !(0.0 < prob && prob < 1.0) {
            die(format!(
                "Line {}: probability {prob} outside the open interval (0, 1); dogmatic extreme refused",
                i + 1
            ));
        }
        let outcome = match fields[2].trim() {
            "0" => false,
            "1" => true,
            other => die(format!(
                "Line {}: outcome '{}' is neither 0 nor 1",
                i + 1,
                other
            )),
        };
        pairs.push((
            fields[0].trim().to_string(),
            prob,
            outcome,
            fields[3].trim().to_string(),
        ));
    }
    if pairs.is_empty() {
        die(format!("{} has no outcome at all", path.display()));
    }
    pairs
}

fn rehearsal_dir(label: &str) -> PathBuf {
    let d = env::temp_dir().join(format!("ingest-jev-{label}-{}", process::id()));
    std::fs::create_dir_all(&d).ok();
    d
}

/// Phase 1 of the closure: checks the sources and rehearses, without touching any destination.
fn check(source_inferences: &Path, source_outcomes: &Path) {
    println!("== Check of the sources (phase 1), without writing to a destination ==");
    let (inferences, model) = read_inferences(source_inferences);
    println!(
        "  inferences: {} request/answer pairs, with both fields",
        inferences.len()
    );
    print_model(&model);
    let outcomes = read_outcomes(source_outcomes);
    println!(
        "  outcomes: {} pairs, with p in (0, 1), outcome 0 or 1 and a declared source",
        outcomes.len()
    );
    let rehearsal_weights = rehearsal_dir("check-weights");
    let rehearsal_cal = rehearsal_dir("check-cal");
    runner::write_artifact_with_model(&rehearsal_weights, &inferences, model.as_ref())
        .unwrap_or_else(|e| die(format!("Failure writing the rehearsal artifact: {e}")));
    let artifact =
        runner::LocalRunner::load(&rehearsal_weights.to_string_lossy()).unwrap_or_else(|e| {
            die(format!(
                "The rehearsal artifact does not pass the gate: {e:?}"
            ))
        });
    calibration::write_outcomes(&rehearsal_cal, &outcomes)
        .unwrap_or_else(|e| die(format!("Failure writing the rehearsal set: {e}")));
    let cal = calibration::load(&rehearsal_cal.to_string_lossy(), 0, calibration::MIN_SAMPLE)
        .unwrap_or_else(|e| die(format!("The rehearsal set does not pass the gate: {e:?}")));
    std::fs::remove_dir_all(&rehearsal_weights).ok();
    std::fs::remove_dir_all(&rehearsal_cal).ok();
    println!(
        "  rehearsal artifact: {} answers, hash {}",
        artifact.answers(),
        artifact.sha256()
    );
    // The schema is charged here, by the real parser: what does not pass becomes an abstention in the bridge, and it
    // is better to find out in phase 1 than in the reading of the instrument.
    let mut outside = 0usize;
    let mut first_error = String::new();
    for (i, (_, answer)) in inferences.iter().enumerate() {
        if let Err(refusal) = validate_output(answer) {
            outside += 1;
            if first_error.is_empty() {
                first_error = format!("case {}: {refusal:?}", i + 1);
            }
        }
    }
    println!(
        "  card schema: {} of {} valid answers",
        inferences.len() - outside,
        inferences.len()
    );
    if outside > 0 {
        eprintln!("  {outside} answer(s) outside the schema; the first: {first_error}");
    }
    println!("  rehearsal set: {}", cal.reading());
    let volume = outcomes.len() >= calibration::MIN_SAMPLE;
    println!(
        "  volume criterion (N >= {}): {}",
        calibration::MIN_SAMPLE,
        if volume {
            "met"
        } else {
            "NOT met, the score will not be published"
        }
    );
    println!("  no destination was touched.");
    if !volume || outside > 0 {
        process::exit(1);
    }
}

/// Phase 4, mechanical: reads the card of the measure and writes the line of the round, or refuses with the reason.
fn record(card: &Path, rounds: &Path, date: &str) {
    let text = std::fs::read_to_string(card)
        .unwrap_or_else(|e| die(format!("Error opening the measure {}: {e}", card.display())));
    let measure = stochology_s2::bridge::from_card(&text)
        .unwrap_or_else(|e| die(format!("Unreadable measure: {e}")));
    let line = match stochology_s2::bridge::registry_line(&measure, date) {
        Ok(l) => l,
        Err(reason) => die(format!("Record refused: {reason}")),
    };
    let header =
        "date	weights_hash	model_hash	requests	abstentions	n_pairs	tokens_per_call	ms_per_call	brier	ece
";
    let existing = std::fs::read_to_string(rounds).unwrap_or_default();
    let mut destination = existing.clone();
    if destination.is_empty() {
        destination.push_str(header);
    }
    if destination.contains(&line) {
        die(String::from(
            "this round is already in the record: the line exists",
        ));
    }
    destination.push_str(&line);
    destination.push('\n');
    std::fs::write(rounds, destination.as_bytes())
        .unwrap_or_else(|e| die(format!("Failure writing {}: {e}", rounds.display())));
    println!("record: {line}");
    println!("  written to {}", rounds.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 6 && args[1] == "--record" {
        record(Path::new(&args[2]), Path::new(&args[3]), &args[5]);
        return;
    }
    if args.len() == 4 && args[1] == "--check" {
        check(Path::new(&args[2]), Path::new(&args[3]));
        return;
    }
    if args.len() != 5 {
        usage_and_abort(&args[0]);
    }
    let weights_dir = Path::new(&args[1]);
    let source_inferences = Path::new(&args[2]);
    let calibration_dir = Path::new(&args[3]);
    let source_outcomes = Path::new(&args[4]);

    println!("== Ingestion of Jev-LLM artifacts (item 13) ==");

    println!(
        "\n[1/3] Reading inferences from {}...",
        source_inferences.display()
    );
    let (inferences, model) = read_inferences(source_inferences);
    println!(
        "  {} request/answer pairs, all of them with both fields",
        inferences.len()
    );
    print_model(&model);

    println!(
        "\n[2/3] Reading outcomes from {}...",
        source_outcomes.display()
    );
    let outcomes = read_outcomes(source_outcomes);
    println!(
        "  {} labeled pairs, with probability in (0, 1), outcome 0 or 1 and a declared source",
        outcomes.len()
    );

    println!(
        "\n[3/3] Rehearsal before persisting: both files go to a temporary directory and come back through the gates"
    );
    let rehearsal_weights = rehearsal_dir("weights");
    let rehearsal_cal = rehearsal_dir("calibration");
    runner::write_artifact_with_model(&rehearsal_weights, &inferences, model.as_ref())
        .unwrap_or_else(|e| die(format!("Failure writing the rehearsal artifact: {e}")));
    let artifact =
        runner::LocalRunner::load(&rehearsal_weights.to_string_lossy()).unwrap_or_else(|e| {
            die(format!(
                "The rehearsal artifact does not pass the gate: {e:?}"
            ))
        });
    println!(
        "  artifact: {} frozen answers, verified hash {}",
        artifact.answers(),
        artifact.sha256()
    );
    calibration::write_outcomes(&rehearsal_cal, &outcomes)
        .unwrap_or_else(|e| die(format!("Failure writing the rehearsal set: {e}")));
    let checked = calibration::load(&rehearsal_cal.to_string_lossy(), 0, calibration::MIN_SAMPLE)
        .unwrap_or_else(|e| die(format!("The rehearsal set does not pass the gate: {e:?}")));
    println!("  labeled set: {}", checked.reading());
    std::fs::remove_dir_all(&rehearsal_weights).ok();
    std::fs::remove_dir_all(&rehearsal_cal).ok();

    println!("\nPersisting to the declared destinations...");
    let weights_hash = runner::write_artifact_with_model(weights_dir, &inferences, model.as_ref())
        .unwrap_or_else(|e| die(format!("Failure in runner::write_artifact: {e}")));
    println!(
        "  {} -> modelo.congelado + SHA256 ({weights_hash})",
        weights_dir.display()
    );
    let cal_hash = calibration::write_outcomes(calibration_dir, &outcomes)
        .unwrap_or_else(|e| die(format!("Failure in calibration::write_outcomes: {e}")));
    println!(
        "  {} -> desfechos.tsv + SHA256 ({cal_hash})",
        calibration_dir.display()
    );

    println!("\nDone. Check the seals and run the instrument:");
    println!("  (cd {} && sha256sum -c SHA256)", weights_dir.display());
    println!(
        "  (cd {} && sha256sum -c SHA256)",
        calibration_dir.display()
    );
    println!("  cargo run --release --bin labs -- --jev");
}
