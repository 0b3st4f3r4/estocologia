//! Generates a synthetic round of 30 pairs, to attest the whole path of item 13.
//!
//! This example is **not** the model: it is a fixture generator. It writes the two sources that the ingestor reads
//! (request/answer and case/probability/outcome/source) with deterministic rules, without drawing and without a
//! number hidden in the code:
//!
//! - request of each case: the same text that the bridge sends, through bridge::request;
//! - answer: the canonical document of the card of the rehearsal model, which passes the schema;
//! - probability: 0.05 + 0.90 * (((n * 7) % 19) / 18), always in the open interval (0, 1);
//! - outcome: 1 when (n * 13) % 5 < 3, and 0 in the other two fifths;
//! - source: the fixed label synthetic-round, so that the record is not confused with an observed outcome.
//!
//! Usage: cargo run --release --example synthetic_round -- <output_directory>

use std::path::PathBuf;

use stochology_s2::bridge::{FrozenModel, RehearsalModel, request};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <output_directory>", args[0]);
        std::process::exit(1);
    }
    let dir = PathBuf::from(&args[1]);
    std::fs::create_dir_all(&dir).expect("creates the output directory");

    let mut rehearsal = RehearsalModel::new();
    let mut inferences = String::new();
    let mut outcomes = String::from("case_id\tprobability\toutcome\tsource\n");
    let mut positives = 0usize;

    for n in 1..=30usize {
        let question = format!("case {n}");
        let request_text = request(&question);
        let answer = rehearsal.decode(&request_text);
        inferences.push_str(&format!(
            "{request_text}\t{}\n",
            answer.replace('\n', "\\n")
        ));

        let p = 0.05 + 0.90 * (((n * 7) % 19) as f64 / 18.0);
        let outcome = usize::from((n * 13) % 5 < 3);
        positives += outcome;
        outcomes.push_str(&format!("C-{n:03}\t{p:.4}\t{outcome}\tsynthetic-round\n"));
    }

    let inf_file = dir.join("frozen_inferences.tsv");
    let out_file = dir.join("audited_outcomes.tsv");
    std::fs::write(&inf_file, inferences.as_bytes()).expect("writes the inferences");
    std::fs::write(&out_file, outcomes.as_bytes()).expect("writes the outcomes");

    println!("synthetic round written to {}", dir.display());
    println!("  30 inferences: {inf_file:?}");
    println!(
        "  30 outcomes: {out_file:?} ({positives} with outcome 1, {} with 0)",
        30 - positives
    );
    println!("  rules: p = 0.05 + 0.90 * (((n * 7) % 19) / 18); outcome = 1 when (n * 13) % 5 < 3");
}
