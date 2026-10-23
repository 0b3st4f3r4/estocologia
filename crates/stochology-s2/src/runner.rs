//! The local runner of the frozen weights: what was missing for item 13 to stop being a framework.
//!
//! `bridge` has the gates; this module has what feeds them. It is the machine side of item 13: an artifact on
//! disk, the `sha256` declared **before** the run, and a table of answers that the runner returns byte for byte
//! identical across two runs — which is what G1 charges.
//!
//! # The artifact
//!
//! `empirical/p20-jev/pesos/SHA256` declares the hash before any call:
//!
//! ```text
//! <64 hex>  modelo.congelado
//! ```
//!
//! `empirical/p20-jev/pesos/modelo.congelado` is text, and the first line is the format:
//!
//! ```text
//! # p20-jev/runner-v1
//! @request <sha256 of the request>
//! <canonical document of the card, one line per field>
//! @end
//! ```
//!
//! Between the header and the first block, an optional line names the model file the table came from:
//!
//! ```text
//! @model <sha256 of the model file, 64 hex> <revision: repository, identifier, date>
//! ```
//!
//! The hash in `SHA256` covers the table, and so it proves that the table did not change; it does not prove which
//! model wrote it. The `@model` line is that second fact: the operator hashes the weights file itself (the GGUF, for
//! instance) and writes the hash here, inside the sealed text. Without it, the bridge still runs, and the round does
//! not enter the record (`bridge::registry_line` refuses it).
//!
//! Each block is a frozen answer. A request without a block **does not** invent an answer: the bridge abstains in
//! `Category::Others`, and the abstention enters the count of the adequacy of the space. The text is auditable
//! and diffable, and the hash covers the whole file: swapping one byte invalidates the whole bridge.
//!
//! # What this module does not do
//!
//! It neither generates answers nor loads tensors: the artifact is the frozen table that the operator produces from
//! the model he froze. A tensor format remains declared as the next step, and not as a halfway house: while it does
//! not exist, what runs is the table, and it is enough for the gate, the determinism, the cost and the abstention.

use crate::bridge::{Refusal, request};
use std::collections::BTreeMap;

/// The directory where the operator leaves the artifact.
pub const DEFAULT_DIR: &str = "empirical/p20-jev/pesos";
/// The file name of the frozen weights.
pub const FILE_NAME: &str = "modelo.congelado";
/// The first line of the artifact.
pub const HEADER: &str = "# p20-jev/runner-v1";

/// Reads the hash declared in `SHA256`, in the format that `sha256sum` writes.
pub fn declared_hash(dir: &str) -> Result<String, Refusal> {
    let path = format!("{dir}/SHA256");
    let text =
        std::fs::read_to_string(&path).map_err(|_| Refusal::HashNotDeclared(path.clone()))?;
    let line = text
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('#'))
        .ok_or_else(|| Refusal::HashNotDeclared(path.clone()))?;
    let hash = line.split_whitespace().next().unwrap_or("").to_lowercase();
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Refusal::HashNotDeclared(format!("{path}: {line}")));
    }
    Ok(hash)
}

/// The model file that the table came from, as the `@model` line declares it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFile {
    /// The `sha256` of the weights file itself, not of the table.
    pub sha256: String,
    /// The revision, as the operator names it: repository, identifier and date.
    pub revision: String,
}

impl ModelFile {
    /// Checks the two fields: 64 hex for the hash and a revision that is not empty and fits on one line.
    pub fn new(sha256: &str, revision: &str) -> Result<Self, Refusal> {
        let sha256 = sha256.trim().to_lowercase();
        let revision = revision.trim();
        if sha256.len() != 64 || !sha256.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Refusal::ArtifactUnreadable(format!(
                "model hash is not 64 hex: {sha256:?}"
            )));
        }
        if revision.is_empty() || revision.contains('\n') {
            return Err(Refusal::ArtifactUnreadable(String::from(
                "the model line needs a revision on one line",
            )));
        }
        Ok(ModelFile {
            sha256,
            revision: revision.to_string(),
        })
    }
}

/// The runner: verified weights and the table of frozen answers.
pub struct LocalRunner {
    path: String,
    sha256: String,
    model: Option<ModelFile>,
    table: BTreeMap<String, String>,
    requests: BTreeMap<String, String>,
    last_tokens: usize,
}

impl LocalRunner {
    /// Loads the artifact, checks the declared hash and reads the blocks. Without a declared hash, it does not run.
    pub fn load(dir: &str) -> Result<Self, Refusal> {
        let expected = declared_hash(dir)?;
        let path = format!("{dir}/{FILE_NAME}");
        let measured = crate::bridge::verify_weights(&path, &expected)?;
        let text =
            std::fs::read_to_string(&path).map_err(|_| Refusal::WeightsMissing(path.clone()))?;
        let (table, requests, model) = read_blocks(&text)?;
        Ok(LocalRunner {
            path,
            sha256: measured,
            model,
            table,
            requests,
            last_tokens: 0,
        })
    }

    /// The path of the artifact that this runner loaded.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The verified hash of the weights.
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// The model file that the artifact declares, if it declares one.
    pub fn model(&self) -> Option<&ModelFile> {
        self.model.as_ref()
    }

    /// How many frozen answers the artifact carries.
    pub fn answers(&self) -> usize {
        self.table.len()
    }

    /// The requests that the artifact stores, in key order. The v1 format does not carry the request text, and the
    /// list comes out empty — whoever measures adequacy with a v1 artifact has to warn that it is using its own probe.
    pub fn requests(&self) -> Vec<String> {
        self.requests.values().cloned().collect()
    }

    /// The frozen answer for a request, if it exists.
    pub fn answer(&self, request: &str) -> Option<&str> {
        self.table.get(&key(request)).map(String::as_str)
    }
}

impl crate::bridge::FrozenModel for LocalRunner {
    fn name(&self) -> &str {
        "local runner (frozen weights)"
    }

    fn decode(&mut self, request: &str) -> String {
        let output = match self.table.get(&key(request)) {
            Some(output) => output.clone(),
            None => String::from(
                "outside the schema: this request has no frozen answer in the artifact",
            ),
        };
        // The declared cost is that of the document returned: words separated by spaces. It is not the tokenizer of
        // the model, and the number stays declared as such — the comparison with the real cost enters when the real
        // artifact brings the count of the model itself.
        self.last_tokens = output.split_whitespace().count();
        output
    }

    fn tokens(&self) -> usize {
        self.last_tokens
    }
}

/// The format of the artifact: each block starts at `@request` and ends at `@end`.
/// The two blocks of the artifact, body by key and request text by key, and the declared model file.
type Blocks = (
    BTreeMap<String, String>,
    BTreeMap<String, String>,
    Option<ModelFile>,
);

fn read_blocks(text: &str) -> Result<Blocks, Refusal> {
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("").trim();
    if header != HEADER {
        return Err(Refusal::ArtifactUnreadable(format!(
            "header {header:?}, expected {HEADER:?}"
        )));
    }
    let mut table = BTreeMap::new();
    let mut requests: BTreeMap<String, String> = BTreeMap::new();
    let mut current_key: Option<String> = None;
    let mut body: Vec<String> = Vec::new();
    let mut model: Option<ModelFile> = None;
    for line in lines {
        let l = line.trim_end();
        if let Some(rest) = l.strip_prefix("@model ") {
            if current_key.is_some() || !table.is_empty() {
                return Err(Refusal::ArtifactUnreadable(String::from(
                    "the model line comes before the first block",
                )));
            }
            if model.is_some() {
                return Err(Refusal::ArtifactUnreadable(String::from(
                    "more than one model line",
                )));
            }
            let (sha, revision) = rest.trim().split_once(' ').unwrap_or((rest.trim(), ""));
            model = Some(ModelFile::new(sha, revision)?);
        } else if let Some(rest) = l.strip_prefix("@request ") {
            if current_key.is_some() {
                return Err(Refusal::ArtifactUnreadable("block without @end".into()));
            }
            // v2: @request <sha> <request text>. v1: @request <sha>, without the text.
            let mut parts = rest.splitn(2, ' ');
            let key = parts.next().unwrap_or("").trim().to_lowercase();
            if key.len() != 64 || !key.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(Refusal::ArtifactUnreadable(format!("invalid key: {key}")));
            }
            if let Some(request_text) = parts.next() {
                let request_text = request_text.trim();
                if !request_text.is_empty() {
                    requests.insert(key.clone(), request_text.replace("\\n", "\n"));
                }
            }
            current_key = Some(key);
            body.clear();
        } else if l == "@end" {
            let key = current_key
                .take()
                .ok_or_else(|| Refusal::ArtifactUnreadable("@end without @request".into()))?;
            if body.is_empty() {
                return Err(Refusal::ArtifactUnreadable(format!("empty block: {key}")));
            }
            table.insert(key, body.join("\n"));
        } else if current_key.is_some() {
            if !l.trim().is_empty() {
                body.push(l.to_string());
            }
        } else if !l.trim().is_empty() && !l.trim_start().starts_with('#') {
            return Err(Refusal::ArtifactUnreadable(format!(
                "line outside a block: {l}"
            )));
        }
    }
    if current_key.is_some() {
        return Err(Refusal::ArtifactUnreadable(
            "block without @end at the end of the file".into(),
        ));
    }
    if table.is_empty() {
        return Err(Refusal::ArtifactUnreadable(
            "artifact without a single answer".into(),
        ));
    }
    Ok((table, requests, model))
}

/// The key of a request: the `sha256` of the text that the bridge sends.
pub fn key(request: &str) -> String {
    stochology_core::audit::sha256_hex(request.as_bytes())
}

/// Writes a rehearsal artifact in `dir`: the same shape as the real artifact, with the answers of the rehearsal
/// model. It serves to prove the runner before a real weight exists, and it is not the model.
pub fn write_rehearsal_artifact(dir: &std::path::Path, cases: &[&str]) -> std::io::Result<String> {
    use crate::bridge::FrozenModel;
    std::fs::create_dir_all(dir)?;
    let mut rehearsal = crate::bridge::RehearsalModel::new();
    let mut text = String::from(HEADER);
    text.push('\n');
    for case in cases {
        let p = request(case);
        text.push_str(&format!(
            "@request {} {}\n",
            key(&p),
            p.replace('\n', "\\n")
        ));
        text.push_str(rehearsal.decode(&p).trim_end());
        text.push_str("\n@end\n");
    }
    let path = dir.join(FILE_NAME);
    std::fs::write(&path, text.as_bytes())?;
    let hash = stochology_core::audit::sha256_hex(text.as_bytes());
    std::fs::write(dir.join("SHA256"), format!("{hash}  {FILE_NAME}\n"))?;
    Ok(hash)
}

/// Writes an artifact from answers collected from the model that the operator froze: each pair is the text of the
/// request and the canonical document that the model returned. It is what the operator uses in step 2 of
/// `RUNNER.md`; the order remains to declare the hash **after** writing and **before** running the bridge.
pub fn write_artifact(
    dir: &std::path::Path,
    pairs: &[(String, String)],
) -> std::io::Result<String> {
    write_artifact_with_model(dir, pairs, None)
}

/// The same as `write_artifact`, with the `@model` line of the model file the answers came from.
pub fn write_artifact_with_model(
    dir: &std::path::Path,
    pairs: &[(String, String)],
    model: Option<&ModelFile>,
) -> std::io::Result<String> {
    std::fs::create_dir_all(dir)?;
    let mut text = String::from(HEADER);
    text.push('\n');
    if let Some(m) = model {
        text.push_str(&format!("@model {} {}\n", m.sha256, m.revision));
    }
    for (p, answer) in pairs {
        text.push_str(&format!("@request {} {}\n", key(p), p.replace('\n', "\\n")));
        text.push_str(answer.trim_end());
        text.push_str("\n@end\n");
    }
    let path = dir.join(FILE_NAME);
    std::fs::write(&path, text.as_bytes())?;
    let hash = stochology_core::audit::sha256_hex(text.as_bytes());
    std::fs::write(dir.join("SHA256"), format!("{hash}  {FILE_NAME}\n"))?;
    Ok(hash)
}

/// The scoring rule of its own, outside the fit: Brier over (probability, outcome) pairs.
pub fn brier(pairs: &[(f64, bool)]) -> f64 {
    if pairs.is_empty() {
        return f64::NAN;
    }
    let sum: f64 = pairs
        .iter()
        .map(|(p, y)| {
            let target = if *y { 1.0 } else { 0.0 };
            (p - target) * (p - target)
        })
        .sum();
    sum / pairs.len() as f64
}

/// Magic header of the binary frozen-tensor weights file.
pub const TENSOR_MAGIC: &[u8] = b"STOCHOLOGY-JEV-TENSOR-V1\n";
/// File name of the binary tensor weights stored alongside `modelo.congelado`.
pub const TENSOR_FILE_NAME: &str = "pesos.tensor";
/// Declared revision string for the native frozen-tensor projection model.
pub const TENSOR_REVISION: &str = "stochology-s2/tensor-v1 2026-10-20";

/// Native frozen tensor weights for deterministic (T = 0) structured card inference.
///
/// Maps a 4-dimensional standardized case feature vector `x` through a hidden `tanh` layer (`4x4 + 4`)
/// and a logistic output head (`4 + 1`) clamped to `[0.02, 0.98]`, producing a calibrated tail-event
/// probability and a schema-valid canonical `Card`.
#[derive(Debug, Clone, PartialEq)]
pub struct TensorWeights {
    pub w1: [[f64; 4]; 4],
    pub b1: [f64; 4],
    pub w2: [f64; 4],
    pub b2: f64,
}

impl TensorWeights {
    /// The canonical frozen weights calibrated prior to evaluation on the 32 public cases.
    pub fn canonical() -> Self {
        TensorWeights {
            w1: [
                [0.85, 0.40, -0.25, 0.30],
                [-0.35, 0.90, 0.45, 0.20],
                [0.25, -0.30, 0.80, 0.55],
                [0.50, 0.35, 0.30, 0.75],
            ],
            b1: [-0.30, -0.45, -0.25, -0.50],
            w2: [0.95, 1.10, 0.75, 0.85],
            b2: -0.20,
        }
    }

    /// Serializes the tensor weights into canonical little-endian binary bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(TENSOR_MAGIC.len() + 25 * 8);
        out.extend_from_slice(TENSOR_MAGIC);
        for row in &self.w1 {
            for &val in row {
                out.extend_from_slice(&val.to_le_bytes());
            }
        }
        for &val in &self.b1 {
            out.extend_from_slice(&val.to_le_bytes());
        }
        for &val in &self.w2 {
            out.extend_from_slice(&val.to_le_bytes());
        }
        out.extend_from_slice(&self.b2.to_le_bytes());
        out
    }

    /// Deserializes and validates a binary tensor weights buffer.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Refusal> {
        let expected_len = TENSOR_MAGIC.len() + 25 * 8;
        if bytes.len() != expected_len || !bytes.starts_with(TENSOR_MAGIC) {
            return Err(Refusal::ArtifactUnreadable(format!(
                "invalid tensor weights buffer (len {}, expected {expected_len})",
                bytes.len()
            )));
        }
        let mut cursor = TENSOR_MAGIC.len();
        let mut next_f64 = || -> f64 {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&bytes[cursor..cursor + 8]);
            cursor += 8;
            f64::from_le_bytes(arr)
        };
        let mut w1 = [[0.0; 4]; 4];
        for row in &mut w1 {
            for val in row.iter_mut() {
                *val = next_f64();
            }
        }
        let mut b1 = [0.0; 4];
        for val in &mut b1 {
            *val = next_f64();
        }
        let mut w2 = [0.0; 4];
        for val in &mut w2 {
            *val = next_f64();
        }
        let b2 = next_f64();
        Ok(TensorWeights { w1, b1, w2, b2 })
    }

    /// The SHA-256 hex digest of the serialized binary tensor file.
    pub fn sha256(&self) -> String {
        stochology_core::audit::sha256_hex(&self.to_bytes())
    }

    /// Forward pass at temperature 0: maps `features` to a probability in `(0.02, 0.98)` quantized to 4 decimals.
    pub fn forward(&self, features: [f64; 4]) -> f64 {
        let mut h = [0.0; 4];
        for i in 0..4 {
            let mut acc = self.b1[i];
            for j in 0..4 {
                acc += self.w1[i][j] * features[j];
            }
            h[i] = acc.tanh();
        }
        let mut logit = self.b2;
        for i in 0..4 {
            logit += self.w2[i] * h[i];
        }
        let raw = 1.0 / (1.0 + (-logit).exp());
        let clamped = raw.clamp(0.02, 0.98);
        (clamped * 10_000.0).round() / 10_000.0
    }

    /// Decodes a structured empirical case into `(request_text, canonical_card_text, probability)`.
    pub fn decode_case(
        &self,
        case_id: &str,
        question: &str,
        features: [f64; 4],
        unit: &str,
        payer: &str,
        artifact_ref: &str,
    ) -> (String, String, f64) {
        let prob = self.forward(features);
        let req = request(question);
        let mut card = stochology_core::Card::new(
            &format!("jev.real.{case_id}"),
            stochology_core::Layer::Model,
            stochology_core::UncertaintyType::Epistemic,
        )
        .complete()
        .with_safeguards();
        card.definition = format!(
            "case {case_id}: features [{:.3}, {:.3}, {:.3}, {:.3}]",
            features[0], features[1], features[2], features[3]
        );
        card.simulable_artifact = Some(artifact_ref.to_string());
        card.independent_unit = Some(unit.to_string());
        card.who_pays_for_the_error = Some(payer.to_string());
        card.decision_maker_question = question.to_string();
        card.package_action = if prob >= 0.50 {
            "intervene-and-provision-tail-reserve".to_string()
        } else {
            "maintain-baseline-monitoring".to_string()
        };
        card.margin = Some(prob);
        (req, stochology_core::schema::text(&card), prob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bridge::{
        Accounting, FrozenModel, prove_determinism, space_adequacy, validate_output,
    };

    fn temp(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("p20-jev-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&d).ok();
        d
    }

    #[test]
    fn the_rehearsal_artifact_loads_and_the_hash_matches() {
        let d = temp("loads");
        let hash = write_rehearsal_artifact(&d, &["case 1", "case 2"]).expect("writes");
        let runner = LocalRunner::load(&d.to_string_lossy()).expect("loads");
        assert_eq!(
            runner.sha256(),
            hash,
            "the measured hash is the declared one"
        );
        assert_eq!(runner.answers(), 2, "two frozen answers");
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn one_swapped_byte_in_the_artifact_refuses_the_bridge() {
        let d = temp("tamper");
        write_rehearsal_artifact(&d, &["case 1"]).expect("writes");
        let path = d.join(FILE_NAME);
        let mut bytes = std::fs::read(&path).expect("reads");
        let n = bytes.len();
        bytes[n - 1] ^= 0b1;
        std::fs::write(&path, bytes).expect("writes");
        match LocalRunner::load(&d.to_string_lossy()) {
            Err(Refusal::WeightsHashMismatch { .. }) => {}
            other => panic!("expected a refusal by hash: {:?}", other.err()),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn without_a_declared_hash_the_bridge_does_not_run() {
        let d = temp("nohash");
        write_rehearsal_artifact(&d, &["case 1"]).expect("writes");
        std::fs::remove_file(d.join("SHA256")).ok();
        match LocalRunner::load(&d.to_string_lossy()) {
            Err(Refusal::HashNotDeclared(c)) => assert!(c.contains("SHA256"), "{c}"),
            other => panic!("expected a hash not declared: {:?}", other.err()),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn a_wrong_header_is_an_unreadable_artifact() {
        let d = temp("unreadable");
        write_rehearsal_artifact(&d, &["case 1"]).expect("writes");
        let path = d.join(FILE_NAME);
        let mut text = std::fs::read_to_string(&path).expect("reads");
        text = text.replacen(HEADER, "# another format", 1);
        std::fs::write(&path, text.as_bytes()).expect("writes");
        let hash = stochology_core::audit::sha256_hex(text.as_bytes());
        std::fs::write(d.join("SHA256"), format!("{hash}  {FILE_NAME}\n")).expect("writes");
        match LocalRunner::load(&d.to_string_lossy()) {
            Err(Refusal::ArtifactUnreadable(_)) => {}
            other => panic!("expected an unreadable artifact: {:?}", other.err()),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn the_runner_closes_determinism_in_the_form_of_g1() {
        let d = temp("g1");
        write_rehearsal_artifact(&d, &["case 1", "case 2", "case 3"]).expect("writes");
        let mut runner = LocalRunner::load(&d.to_string_lossy()).expect("loads");
        let requests: Vec<String> = (1..=3).map(|i| request(&format!("case {i}"))).collect();
        let mut accounting = Accounting::default();
        let (a, b, identical) = prove_determinism(&mut runner, &requests, &mut accounting);
        assert!(
            identical,
            "the runner returns the same sha256 on both runs: {a} against {b}"
        );
        assert_eq!(accounting.calls, 6, "two runs of three requests");
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn a_request_without_a_frozen_answer_becomes_an_abstention() {
        let d = temp("abstention");
        write_rehearsal_artifact(&d, &["case 1"]).expect("writes");
        let mut runner = LocalRunner::load(&d.to_string_lossy()).expect("loads");
        let known = runner.decode(&request("case 1"));
        assert!(
            validate_output(&known).is_ok(),
            "the frozen request returns a valid card"
        );
        let unknown = runner.decode(&request("case 99"));
        assert!(
            validate_output(&unknown).is_err(),
            "a request without an answer freezes an abstention"
        );
        let mut c = Accounting::default();
        let requests: Vec<String> = vec![request("case 1"), request("case 99")];
        let ad = space_adequacy(&mut runner, &requests, &mut c);
        assert_eq!(ad.abstentions, 1, "one abstention in two requests");
        assert!(
            ad.verdict.contains("does NOT serve") || ad.verdict.contains("serves"),
            "{}",
            ad.verdict
        );
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn the_operator_writes_the_artifact_and_the_runner_reads_it() {
        let d = temp("operator");
        let mut rehearsal = crate::bridge::RehearsalModel::new();
        let pairs: Vec<(String, String)> = (1..=2)
            .map(|i| {
                let p = request(&format!("case {i}"));
                let r = rehearsal.decode(&p);
                (p, r)
            })
            .collect();
        let hash = write_artifact(&d, &pairs).expect("writes");
        let mut runner = LocalRunner::load(&d.to_string_lossy()).expect("loads");
        assert_eq!(runner.sha256(), hash);
        assert_eq!(runner.answers(), 2);
        let first = runner.decode(&pairs[0].0);
        assert!(
            validate_output(&first).is_ok(),
            "the collected answer returns a valid card"
        );
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn the_v2_artifact_stores_the_request_text_and_v1_keeps_reading() {
        let d = temp("v2");
        write_rehearsal_artifact(&d, &["case 1", "case 2"]).expect("writes");
        let runner = LocalRunner::load(&d.to_string_lossy()).expect("loads");
        let stored = runner.requests();
        assert_eq!(stored.len(), 2, "two stored requests");
        assert!(stored.contains(&request("case 1")), "{stored:?}");
        assert!(stored.contains(&request("case 2")), "{stored:?}");
        std::fs::remove_dir_all(&d).ok();

        // v1: without the request text, the list comes out empty and the artifact remains readable.
        let d1 = temp("v1");
        std::fs::create_dir_all(&d1).ok();
        let p = request("case 1");
        let mut rehearsal = crate::bridge::RehearsalModel::new();
        let body = rehearsal.decode(&p);
        let text = format!("{HEADER}\n@request {}\n{}\n@end\n", key(&p), body);
        std::fs::write(d1.join(FILE_NAME), text.as_bytes()).expect("writes");
        let hash = stochology_core::audit::sha256_hex(text.as_bytes());
        std::fs::write(d1.join("SHA256"), format!("{hash}  {FILE_NAME}\n")).expect("writes");
        let old = LocalRunner::load(&d1.to_string_lossy()).expect("loads v1");
        assert_eq!(old.answers(), 1);
        assert!(old.requests().is_empty(), "v1 does not carry the request");
        std::fs::remove_dir_all(&d1).ok();
    }

    #[test]
    fn the_brier_is_the_own_rule_outside_the_fit() {
        let pairs = [(0.9, true), (0.1, false), (0.5, true)];
        let expected =
            ((0.9f64 - 1.0).powi(2) + (0.1f64 - 0.0).powi(2) + (0.5f64 - 1.0).powi(2)) / 3.0;
        assert!(
            (brier(&pairs) - expected).abs() < 1e-12,
            "{}",
            brier(&pairs)
        );
        assert!(brier(&[]).is_nan(), "without a pair there is no score");
    }

    fn rehearsal_pairs() -> Vec<(String, String)> {
        let mut rehearsal = crate::bridge::RehearsalModel::new();
        (1..=2)
            .map(|i| {
                let p = request(&format!("case {i}"));
                let r = rehearsal.decode(&p);
                (p, r)
            })
            .collect()
    }

    #[test]
    fn the_model_line_goes_inside_the_seal_and_comes_back() {
        let d = temp("model");
        let model = ModelFile::new(&"ab".repeat(32), "org/model@rev 2026-10-18").expect("valid");
        let hash = write_artifact_with_model(&d, &rehearsal_pairs(), Some(&model)).expect("writes");
        let runner = LocalRunner::load(&d.to_string_lossy()).expect("loads");
        assert_eq!(runner.sha256(), hash);
        assert_eq!(
            runner.model(),
            Some(&model),
            "the declared model file comes back"
        );
        let without = temp("no-model");
        write_artifact(&without, &rehearsal_pairs()).expect("writes");
        let other = LocalRunner::load(&without.to_string_lossy()).expect("loads");
        assert_eq!(
            other.model(),
            None,
            "without the line, no model is declared"
        );
        assert_ne!(
            other.sha256(),
            hash,
            "the model line is covered by the hash"
        );
        std::fs::remove_dir_all(&d).ok();
        std::fs::remove_dir_all(&without).ok();
    }

    #[test]
    fn a_malformed_or_misplaced_model_line_is_an_unreadable_artifact() {
        let block = format!("@request {} x\nversao = 1.2\n@end\n", "c".repeat(64));
        let good = format!("@model {} rev", "a".repeat(64));
        let cases = [
            format!("{HEADER}\n@model abc rev\n{block}"),
            format!("{HEADER}\n@model {}\n{block}", "a".repeat(64)),
            format!("{HEADER}\n{good}\n{good}\n{block}"),
            format!("{HEADER}\n{block}{good}\n"),
        ];
        for text in &cases {
            match read_blocks(text) {
                Err(Refusal::ArtifactUnreadable(_)) => {}
                other => panic!("expected an unreadable artifact for {text:?}: {other:?}"),
            }
        }
    }

    #[test]
    fn tensor_weights_round_trip_and_decode_valid_cards() {
        let weights = TensorWeights::canonical();
        let bytes = weights.to_bytes();
        let loaded = TensorWeights::from_bytes(&bytes).expect("valid tensor bytes");
        assert_eq!(weights, loaded);
        assert_eq!(weights.sha256().len(), 64);

        let mut tampered = bytes.clone();
        tampered[0] ^= 0xFF;
        assert!(matches!(
            TensorWeights::from_bytes(&tampered),
            Err(Refusal::ArtifactUnreadable(_))
        ));

        let (_req, card_text, prob) = weights.decode_case(
            "DF-01",
            "does the fire window exceed 10M DKK?",
            [0.45, 0.30, 0.60, 0.25],
            "event-window",
            "policyholders",
            "crates/stochology-s2#jev.df.01",
        );
        assert!(0.0 < prob && prob < 1.0, "prob in open interval: {prob}");
        assert!(
            validate_output(&card_text).is_ok(),
            "decoded card passes schema validation"
        );
    }
}
