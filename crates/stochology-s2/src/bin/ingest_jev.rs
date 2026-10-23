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
        "  {program} --from-data <data_dir> <weights_dir> <calibration_dir> <rounds.tsv> --date YYYY-MM-DD"
    );
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

/// Builds the 32 audited real cases from `data/danish_fire.csv` (16 windows) and `data/atlantic_storms.csv` (16 seasons),
/// runs the frozen tensor weights at temperature 0, rehearses through the gates, persists the sealed artifacts, and records the round.
fn build_from_data(
    data_dir: &Path,
    weights_dir: &Path,
    calibration_dir: &Path,
    rounds: &Path,
    date: &str,
) {
    let fire_path = data_dir.join("danish_fire.csv");
    let fire_txt = std::fs::read_to_string(&fire_path)
        .unwrap_or_else(|e| die(format!("Error reading {}: {e}", fire_path.display())));
    let mut fire_losses: Vec<(String, f64)> = Vec::new();
    for line in fire_txt.lines().skip(1) {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if let Some((dt, val)) = l.split_once(',')
            && let Ok(loss) = val.trim().parse::<f64>()
        {
            fire_losses.push((dt.trim().to_string(), loss));
        }
    }
    if fire_losses.len() < 16 * 135 {
        die(format!(
            "danish_fire.csv has {} rows, expected at least {}",
            fire_losses.len(),
            16 * 135
        ));
    }

    let storm_path = data_dir.join("atlantic_storms.csv");
    let storm_txt = std::fs::read_to_string(&storm_path)
        .unwrap_or_else(|e| die(format!("Error reading {}: {e}", storm_path.display())));
    // Collect (wind, pressure) per year for 1990..=2005
    let mut storm_years: std::collections::BTreeMap<u32, Vec<(f64, f64)>> =
        std::collections::BTreeMap::new();
    for line in storm_txt.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 12 {
            continue;
        }
        let Ok(year) = fields[2].trim().parse::<u32>() else {
            continue;
        };
        if !(1990..=2005).contains(&year) {
            continue;
        }
        let wind = fields[10].trim().parse::<f64>().unwrap_or(0.0);
        let pressure = fields[11].trim().parse::<f64>().unwrap_or(1013.0);
        storm_years.entry(year).or_default().push((wind, pressure));
    }
    if storm_years.len() != 16 {
        die(format!(
            "atlantic_storms.csv yielded {} seasons in 1990..=2005, expected 16",
            storm_years.len()
        ));
    }

    let tensor = runner::TensorWeights::canonical();
    let tensor_bytes = tensor.to_bytes();
    let tensor_sha = tensor.sha256();
    let model = runner::ModelFile::new(&tensor_sha, runner::TENSOR_REVISION)
        .unwrap_or_else(|e| die(format!("Invalid tensor model metadata: {e:?}")));

    let mut inferences: Vec<(String, String)> = Vec::with_capacity(32);
    let mut outcomes: Vec<(String, f64, bool, String)> = Vec::with_capacity(32);

    // 16 Danish fire windows (135 events each: 100 baseline, 35 evaluation for > 9.0M DKK tail claim)
    for w in 0..16usize {
        let start = w * 135;
        let split = start + 100;
        let end = start + 135;
        let base = &fire_losses[start..split];
        let eval = &fire_losses[split..end];
        let mean_loss = base.iter().map(|(_, x)| *x).sum::<f64>() / 100.0;
        let tail_5m = base.iter().filter(|(_, x)| *x > 5.0).count() as f64 / 100.0;
        let max_loss = base.iter().map(|(_, x)| *x).fold(0.0f64, f64::max);
        let cluster = base
            .windows(2)
            .filter(|pair| pair[0].1 > 3.0 && pair[1].1 > 3.0)
            .count() as f64
            / 99.0;
        let features = [
            (mean_loss - 3.0) / 2.0,
            (tail_5m - 0.12) * 5.0,
            (max_loss - 15.0) / 20.0,
            (cluster - 0.08) * 6.0,
        ];
        let observed = eval.iter().any(|(_, x)| *x > 9.0);
        let case_id = format!("DF-{:02}", w + 1);
        let question = format!(
            "Danish fire window {case_id} ({} to {}): does the next 35-claim subwindow exceed 9.0M DKK?",
            base.first().map(|x| x.0.as_str()).unwrap_or(""),
            eval.last().map(|x| x.0.as_str()).unwrap_or("")
        );
        let (req, card_text, prob) = tensor.decode_case(
            &case_id,
            &question,
            features,
            "claim-window",
            "reinsurance-pool",
            &format!("crates/stochology-s2#jev.{}", case_id.to_lowercase()),
        );
        let source = format!("data/danish_fire.csv#rows-{}-{}", start + 2, end + 1);
        inferences.push((req, card_text));
        outcomes.push((case_id, prob, observed, source));
    }

    // 16 Atlantic storm seasons (1990..=2005: first half baseline, second half evaluation for >= 105 kt intense hurricane)
    for (&year, records) in &storm_years {
        let mid = (records.len() / 2).max(1);
        let (base, eval) = records.split_at(mid);
        let mean_wind = base.iter().map(|(w, _)| *w).sum::<f64>() / base.len() as f64;
        let max_wind = base.iter().map(|(w, _)| *w).fold(0.0f64, f64::max);
        let min_press = base.iter().map(|(_, p)| *p).fold(1013.0f64, f64::min);
        let ts_share = base.iter().filter(|(w, _)| *w >= 50.0).count() as f64 / base.len() as f64;
        let features = [
            (mean_wind - 45.0) / 15.0,
            (max_wind - 75.0) / 30.0,
            (990.0 - min_press) / 25.0,
            (ts_share - 0.35) * 3.0,
        ];
        let observed = eval.iter().any(|(w, _)| *w >= 105.0);
        let case_id = format!("AS-{year}");
        let question = format!(
            "Atlantic storm season {year} ({} observations): does the second half reach 105 kt?",
            records.len()
        );
        let (req, card_text, prob) = tensor.decode_case(
            &case_id,
            &question,
            features,
            "storm-season",
            "coastal-municipalities",
            &format!("crates/stochology-s2#jev.as.{year}"),
        );
        let source = format!("data/atlantic_storms.csv#season-{year}");
        inferences.push((req, card_text));
        outcomes.push((case_id, prob, observed, source));
    }

    let rehearsal_weights = rehearsal_dir("from-data-weights");
    let rehearsal_cal = rehearsal_dir("from-data-cal");
    runner::write_artifact_with_model(&rehearsal_weights, &inferences, Some(&model))
        .unwrap_or_else(|e| die(format!("Failure writing rehearsal artifact: {e}")));
    let _ = runner::LocalRunner::load(&rehearsal_weights.to_string_lossy())
        .unwrap_or_else(|e| die(format!("Rehearsal artifact failed gate: {e:?}")));
    for (i, (_, ans)) in inferences.iter().enumerate() {
        if let Err(r) = validate_output(ans) {
            die(format!("Case {} failed card schema: {r:?}", i + 1));
        }
    }
    calibration::write_outcomes(&rehearsal_cal, &outcomes)
        .unwrap_or_else(|e| die(format!("Failure writing rehearsal outcomes: {e}")));
    let _ = calibration::load(&rehearsal_cal.to_string_lossy(), 0, calibration::MIN_SAMPLE)
        .unwrap_or_else(|e| die(format!("Rehearsal outcomes failed gate: {e:?}")));
    std::fs::remove_dir_all(&rehearsal_weights).ok();
    std::fs::remove_dir_all(&rehearsal_cal).ok();

    std::fs::create_dir_all(weights_dir)
        .unwrap_or_else(|e| die(format!("Failure creating {}: {e}", weights_dir.display())));
    let tensor_path = weights_dir.join(runner::TENSOR_FILE_NAME);
    std::fs::write(&tensor_path, &tensor_bytes)
        .unwrap_or_else(|e| die(format!("Failure writing {}: {e}", tensor_path.display())));
    let weights_hash = runner::write_artifact_with_model(weights_dir, &inferences, Some(&model))
        .unwrap_or_else(|e| die(format!("Failure writing artifact: {e}")));
    let cal_hash = calibration::write_outcomes(calibration_dir, &outcomes)
        .unwrap_or_else(|e| die(format!("Failure writing outcomes: {e}")));

    println!("== Frozen Tensor Execution & Ingestion (item 13) ==");
    println!("  tensor file: {} ({tensor_sha})", tensor_path.display());
    println!(
        "  artifact:    {}/modelo.congelado ({weights_hash})",
        weights_dir.display()
    );
    println!(
        "  outcomes:    {}/desfechos.tsv ({cal_hash})",
        calibration_dir.display()
    );

    let m = stochology_s2::bridge::measure();
    let card_txt = stochology_s2::bridge::measure_card(&m);
    let tmp_card = rehearsal_dir("card").join("measure.txt");
    std::fs::write(&tmp_card, card_txt.as_bytes()).expect("writes measure card");
    let existing = std::fs::read_to_string(rounds).unwrap_or_default();
    let line = stochology_s2::bridge::registry_line(&m, date)
        .unwrap_or_else(|e| die(format!("Record refused: {e}")));
    if !existing.contains(&line) {
        record(&tmp_card, rounds, date);
    } else {
        println!("record already present in {}: {line}", rounds.display());
    }
    if let Some(parent) = tmp_card.parent() {
        std::fs::remove_dir_all(parent).ok();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 8 && args[1] == "--from-data" && args[6] == "--date" {
        build_from_data(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
            &args[7],
        );
        return;
    }
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
