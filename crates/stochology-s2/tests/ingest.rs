//! The tests of the ingestor: what it writes, what it refuses and what it does not touch when it refuses.

use std::path::{Path, PathBuf};
use std::process::Command;

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("ingest-jev-test-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&d).expect("creates the test directory");
    d
}

/// The complete canonical card, with id and margin parameterized: it is what the real parser accepts.
fn card(id: &str, margin: f64) -> String {
    format!(
        "versao = 1.2\n\
         acao_do_pacote = abstain when the output leaves the schema\n\
         artefato_simulavel = crates/stochology-core#jev.test.{id}\n\
         camada = L2\n\
         clausula_de_saida = stop-if-ECE>0.10\n\
         definicao = test case\n\
         deriva_declarada = true\n\
         escopo_ecologico = true\n\
         id = {id}\n\
         incerteza = epistemica\n\
         margem = {margin:.4}\n\
         modelo_nulo = historical-mean\n\
         patologias_cobertas = true\n\
         pergunta_do_decisor = does the case appear?\n\
         priori_elicitada = true\n\
         quem_paga_pelo_erro = declared-decider\n\
         resolucao = R1-dia\n\
         semente = 2026\n\
         subgrupos_auditados = true\n\
         teste_de_propriedade = true\n\
         trata_ausentes = true\n\
         trilha_integra = true\n\
         unidade_independente = day\n"
    )
}

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_ingest-jev")
}

fn check_seal(dir: &Path, file: &str) {
    let manifest = std::fs::read_to_string(dir.join("SHA256")).expect("reads the manifest");
    let declared = manifest
        .split_whitespace()
        .next()
        .expect("hash in the manifest")
        .to_lowercase();
    let data = std::fs::read(dir.join(file)).expect("reads the file");
    assert_eq!(
        declared,
        stochology_core::audit::sha256_hex(&data),
        "the manifest matches the file"
    );
}

#[test]
fn the_ingestor_writes_the_artifact_and_the_set_with_the_seals() {
    let d = temp("ok");
    let inf = d.join("inferences.tsv");
    std::fs::write(&inf, "question A\tanswer A\nquestion B\tanswer B\n").expect("writes");
    let out = d.join("outcomes.tsv");
    std::fs::write(&out, "C-001\t0.7000\t1\trecord\nC-002\t0.2000\t0\trecord\n").expect("writes");
    let weights = d.join("weights");
    let cal = d.join("calibration");
    let output = Command::new(bin())
        .arg(&weights)
        .arg(&inf)
        .arg(&cal)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    assert!(
        output.status.success(),
        "the ingestor should pass: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    check_seal(&weights, "modelo.congelado");
    check_seal(&cal, "desfechos.tsv");
    let text = std::fs::read_to_string(cal.join("desfechos.tsv")).expect("reads the tsv");
    assert!(
        text.starts_with("id\tcase_id\tdeclared_probability\t"),
        "canonical header"
    );
    assert_eq!(text.lines().count(), 3, "header and two cases");
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn a_probability_at_the_extreme_aborts_and_writes_nothing() {
    let d = temp("extreme");
    let inf = d.join("inferences.tsv");
    std::fs::write(&inf, "question A\tanswer A\n").expect("writes");
    let out = d.join("outcomes.tsv");
    std::fs::write(&out, "C-001\t1.0000\t1\trecord\n").expect("writes");
    let weights = d.join("weights");
    let cal = d.join("calibration");
    let output = Command::new(bin())
        .arg(&weights)
        .arg(&inf)
        .arg(&cal)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    assert!(
        !output.status.success(),
        "a probability at 1.0 had to abort"
    );
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("dogmatic extreme"), "{error}");
    assert!(
        !weights.exists(),
        "nothing is written to the weights destination"
    );
    assert!(
        !cal.exists(),
        "nothing is written to the calibration destination"
    );
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn without_arguments_the_ingestor_prints_the_usage_and_exits_with_1() {
    let output = Command::new(bin()).output().expect("runs the ingestor");
    assert_eq!(output.status.code(), Some(1), "wrong usage exits with 1");
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Usage:"), "{error}");
    assert!(error.contains("weights_dir"), "{error}");
}

#[test]
fn the_check_mode_validates_without_touching_a_destination() {
    let d = temp("check");
    let inf = d.join("inferences.tsv");
    std::fs::write(&inf, "question A\tanswer A\nquestion B\tanswer B\n").expect("writes");
    let out = d.join("outcomes.tsv");
    std::fs::write(&out, "C-001\t0.4000\t1\trecord\n").expect("writes");
    let output = Command::new(bin())
        .arg("--check")
        .arg(&inf)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("no destination was touched"), "{text}");
    assert!(
        text.contains("InsufficientSample"),
        "with one pair, the score is not published: {text}"
    );
    assert_eq!(
        output.status.code(),
        Some(1),
        "volume below the minimum exits with 1"
    );
    assert!(
        !d.join("weights").exists(),
        "the check mode does not create a destination"
    );
    assert!(
        !d.join("calibration").exists(),
        "the check mode does not create a destination"
    );
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn the_check_mode_passes_with_thirty_pairs() {
    let d = temp("check30");
    let inf = d.join("inferences.tsv");
    let mut lines = String::new();
    for n in 1..=30 {
        let p = 0.05 + 0.9 * (n as f64 / 31.0);
        let text = card(&format!("case-{n}"), p).replace('\n', "\\n");
        lines.push_str(&format!("question {n}\t{text}\n"));
    }
    std::fs::write(&inf, lines.as_bytes()).expect("writes");
    let out = d.join("outcomes.tsv");
    let mut out_lines = String::new();
    for n in 1..=30 {
        let p = 0.05 + 0.9 * (n as f64 / 31.0);
        out_lines.push_str(&format!("C-{n:03}\t{p:.4}\t{}\trecord\n", n % 2));
    }
    std::fs::write(&out, out_lines.as_bytes()).expect("writes");
    let output = Command::new(bin())
        .arg("--check")
        .arg(&inf)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{text}{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("volume criterion (N >= 30): met"), "{text}");
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn the_check_mode_refuses_an_answer_outside_the_schema() {
    let d = temp("check-schema");
    let inf = d.join("inferences.tsv");
    let mut lines = String::new();
    for n in 1..=30 {
        // free text everywhere: no answer is a canonical card
        lines.push_str(&format!("question {n}\tthis is free text {n}\n"));
    }
    std::fs::write(&inf, lines.as_bytes()).expect("writes");
    let out = d.join("outcomes.tsv");
    let mut out_lines = String::new();
    for n in 1..=30 {
        let p = 0.05 + 0.9 * (n as f64 / 31.0);
        out_lines.push_str(&format!("C-{n:03}\t{p:.4}\t{}\trecord\n", n % 2));
    }
    std::fs::write(&out, out_lines.as_bytes()).expect("writes");
    let output = Command::new(bin())
        .arg("--check")
        .arg(&inf)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    let text = String::from_utf8_lossy(&output.stdout);
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(text.contains("0 of 30 valid answers"), "{text}");
    assert_eq!(
        output.status.code(),
        Some(1),
        "an answer outside the schema exits with 1"
    );
    assert!(error.contains("outside the schema"), "{error}");
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn the_escape_of_the_request_does_not_break_the_key_of_the_artifact() {
    // The source escapes both columns. If the ingestor unescaped only the answer, the key of the artifact would
    // become the sha of the escaped text and the stored request the raw text: the bridge looks, does not find, and
    // abstains in 100% of the cases. This test exists because that is exactly what happened in the first real round.
    let d = temp("request-escape");
    let inf = d.join("inferences.tsv");
    let request = "line one\nline two";
    let answer = card("case-escape", 0.4).replace('\n', "\\n");
    std::fs::write(
        &inf,
        format!("{}\t{}\n", request.replace('\n', "\\n"), answer),
    )
    .expect("writes");
    let out = d.join("outcomes.tsv");
    std::fs::write(&out, "C-001\t0.4000\t1\trecord\n").expect("writes");
    let weights = d.join("weights");
    let cal = d.join("calibration");
    let output = Command::new(bin())
        .arg(&weights)
        .arg(&inf)
        .arg(&cal)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let runner =
        stochology_s2::runner::LocalRunner::load(&weights.to_string_lossy()).expect("loads");
    let stored = runner.requests();
    assert_eq!(stored.len(), 1, "one stored request");
    assert_eq!(
        stored[0], request,
        "the stored request is the raw text, not the escaped one"
    );
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn the_model_line_of_the_source_enters_the_sealed_artifact() {
    let d = temp("model");
    let sha = "ab".repeat(32);
    let inf = d.join("inferences.tsv");
    std::fs::write(
        &inf,
        format!("# model-sha256\t{sha}\torg/model@rev 2026-10-18\nquestion A\tanswer A\n"),
    )
    .expect("writes");
    let out = d.join("outcomes.tsv");
    std::fs::write(&out, "C-001\t0.7000\t1\trecord\n").expect("writes");
    let weights = d.join("weights");
    let cal = d.join("calibration");
    let output = Command::new(bin())
        .arg(&weights)
        .arg(&inf)
        .arg(&cal)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    assert!(
        output.status.success(),
        "the ingestor should pass: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    check_seal(&weights, "modelo.congelado");
    let artifact = std::fs::read_to_string(weights.join("modelo.congelado")).expect("reads");
    assert!(
        artifact.contains(&format!("@model {sha} org/model@rev 2026-10-18\n")),
        "the model line is inside the sealed text: {artifact}"
    );
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn a_malformed_model_line_aborts_and_writes_nothing() {
    let d = temp("bad-model");
    let inf = d.join("inferences.tsv");
    std::fs::write(
        &inf,
        "# model-sha256\tnot-a-hash\trev\nquestion A\tanswer A\n",
    )
    .expect("writes");
    let out = d.join("outcomes.tsv");
    std::fs::write(&out, "C-001\t0.7000\t1\trecord\n").expect("writes");
    let weights = d.join("weights");
    let cal = d.join("calibration");
    let output = Command::new(bin())
        .arg(&weights)
        .arg(&inf)
        .arg(&cal)
        .arg(&out)
        .output()
        .expect("runs the ingestor");
    assert!(!output.status.success(), "a bad model hash aborts");
    assert!(!weights.exists(), "the destination is not touched");
    std::fs::remove_dir_all(&d).ok();
}
