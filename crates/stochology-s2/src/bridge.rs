//! The Jev-LLM bridge: the framework of the bridge, with the gates that close without the weights.
//!
//! Item 13 asks for the local runner of frozen weights with the `sha256` of the weights, the decoding restricted to the
//! card schema, the determinism proof in the form of G1, the calibration against the Brier of the deterministic Jev,
//! the cost per call and the parallel question of the adequacy of the hypothesis space. **The local runner exists,
//! in `runner.rs`**; what does not exist in this tree are the weights of the model, and that is why the gate of
//! the weights keeps refusing. This module delivers the framework with the gates that depend on the model, plus
//! the ones that already close:
//!
//! - **the gate of the weights** — without the file and without the `sha256` matching, the bridge **does not run**,
//!   and no number comes out;
//! - **the restricted decoding** — the output has to be the canonical document of the card, and the margin cannot
//!   touch 0% or 100%: outside that, the bridge **abstains** (`Category::Others`, the abstention that already
//!   exists in `s1.rs`);
//! - **the determinism proof** — two runs and the same `sha256`, in the form of G1; and the proof **fails** a model
//!   that changes its answer, which is exactly what `docs/JEV_INTEGRATION.md` declares about the free LLM;
//! - **the cost per call** and the **adequacy of the hypothesis space** — measured on the rehearsal model, and
//!   declared as a rule for when the weights exist.
//!
//! Run: ~labs --jev~

use stochology_core::s1::Category;

/// What the bridge needs from a frozen model.
pub trait FrozenModel {
    fn name(&self) -> &str;
    /// Decodes a request into the canonical text of the card.
    fn decode(&mut self, request: &str) -> String;
    /// The tokens spent in the last call.
    fn tokens(&self) -> usize;
    /// The declared time cost per call, in milliseconds (0 in the in-memory table, without a clock).
    fn ms(&self) -> u128 {
        0
    }
}

/// The request that the bridge sends to the model: the question of the decision-maker, in the schema.
pub fn request(question: &str) -> String {
    format!("answer in the canonical card schema, without free text: {question}")
}

/// Why the bridge refuses an output or a run.
#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    /// Without the weights file, the bridge does not run.
    WeightsMissing(String),
    /// With a hash different from the declared one, the bridge does not run.
    WeightsHashMismatch { expected: String, measured: String },
    /// The operator did not declare the hash before running.
    HashNotDeclared(String),
    /// The artifact exists and the hash matches, and the format is not the declared one.
    ArtifactUnreadable(String),
    /// The output is not the canonical document of the card.
    OutOfSchema(String),
    /// The output breaks the prohibition of the dogmatic extremes.
    DogmaticExtreme(String),
}

impl Refusal {
    /// The declared abstention: the auditable "I do not know" that already exists in System 1.
    pub fn abstention(&self) -> Category {
        Category::Others
    }
}

/// The gate of the weights: the file has to exist and the `sha256` has to match the declared one.
pub fn verify_weights(path: &str, expected: &str) -> Result<String, Refusal> {
    let data = std::fs::read(path).map_err(|_| Refusal::WeightsMissing(path.to_string()))?;
    let measured = stochology_core::audit::sha256_hex(&data);
    let expected = expected.trim().to_lowercase();
    if measured != expected {
        return Err(Refusal::WeightsHashMismatch { expected, measured });
    }
    Ok(measured)
}

/// The restricted decoding: the output has to be the canonical document of the card, and the margin cannot touch
/// the extremes. Outside that, the bridge abstains instead of accepting free text.
pub fn validate_output(output: &str) -> Result<stochology_core::Card, Refusal> {
    let card = stochology_core::schema::from_text(output)
        .map_err(|e| Refusal::OutOfSchema(format!("{e:?}")))?;
    if let Some(m) = card.margin
        && !(0.0 < m && m < 1.0)
    {
        return Err(Refusal::DogmaticExtreme(format!("margin {m}")));
    }
    Ok(card)
}

/// The accounting of the bridge: calls, tokens and time, with the cost per call.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Accounting {
    pub calls: usize,
    pub tokens: usize,
    pub ms: u128,
}

impl Accounting {
    /// Tokens and milliseconds per call.
    pub fn per_call(&self) -> (f64, f64) {
        let n = self.calls.max(1) as f64;
        (self.tokens as f64 / n, self.ms as f64 / n)
    }
}

/// The determinism proof in the form of G1: two runs, the same `sha256`.
pub fn prove_determinism(
    model: &mut dyn FrozenModel,
    requests: &[String],
    accounting: &mut Accounting,
) -> (String, String, bool) {
    let run_once = |model: &mut dyn FrozenModel, accounting: &mut Accounting| -> String {
        let mut outputs = Vec::new();
        for r in requests {
            let s = model.decode(r);
            accounting.calls += 1;
            accounting.tokens += model.tokens();
            accounting.ms += model.ms();
            outputs.push(s);
        }
        stochology_core::audit::sha256_hex(outputs.join("\n").as_bytes())
    };
    let a = run_once(model, accounting);
    let b = run_once(model, accounting);
    let identical = a == b;
    (a, b, identical)
}

/// The fraction of abstentions above which the hypothesis space does not serve the model.
pub const MAX_ABSTENTIONS: f64 = 0.30;

/// The parallel question of the adequacy of the hypothesis space.
#[derive(Debug, Clone, PartialEq)]
pub struct Adequacy {
    pub requests: usize,
    pub abstentions: usize,
    pub fraction: f64,
    pub verdict: String,
}

/// Measures the adequacy: how many requests the bridge had to abstain on, and the verdict declared before running.
pub fn space_adequacy(
    model: &mut dyn FrozenModel,
    requests: &[String],
    accounting: &mut Accounting,
) -> Adequacy {
    let mut abstentions = 0usize;
    for r in requests {
        let output = model.decode(r);
        accounting.calls += 1;
        accounting.tokens += model.tokens();
        accounting.ms += model.ms();
        if validate_output(&output).is_err() {
            abstentions += 1;
        }
    }
    let fraction = abstentions as f64 / requests.len().max(1) as f64;
    let verdict = if fraction > MAX_ABSTENTIONS {
        format!(
            "the card schema does NOT serve this model: {:.1}% of abstentions, above the declared limit of {:.0}%",
            fraction * 100.0,
            MAX_ABSTENTIONS * 100.0
        )
    } else {
        format!(
            "the card schema serves this model: {:.1}% of abstentions, inside the declared limit of {:.0}%",
            fraction * 100.0,
            MAX_ABSTENTIONS * 100.0
        )
    };
    Adequacy {
        requests: requests.len(),
        abstentions,
        fraction,
        verdict,
    }
}

/// A rehearsal model: deterministic, answers in the schema of the card.
pub struct RehearsalModel {
    pub tokens_per_call: usize,
}

impl RehearsalModel {
    pub fn new() -> Self {
        RehearsalModel {
            tokens_per_call: 42,
        }
    }
}

impl Default for RehearsalModel {
    fn default() -> Self {
        RehearsalModel::new()
    }
}

impl FrozenModel for RehearsalModel {
    fn name(&self) -> &str {
        "rehearsal model (deterministic)"
    }
    fn decode(&mut self, request: &str) -> String {
        let card = stochology_core::Card::new(
            "jev.bridge.1",
            stochology_core::Layer::Model,
            stochology_core::UncertaintyType::Epistemic,
        )
        .complete()
        .with_safeguards();
        let mut card = card;
        card.definition = request.to_string();
        card.decision_maker_question = "does the bridge answer?".into();
        card.package_action = "abstain when the output leaves the schema".into();
        stochology_core::schema::text(&card)
    }
    fn tokens(&self) -> usize {
        self.tokens_per_call
    }
}

/// An unstable rehearsal model: changes its answer on every call — it is the free LLM of the text, which does not
/// close G1.
pub struct UnstableRehearsalModel {
    counter: usize,
}

impl UnstableRehearsalModel {
    pub fn new() -> Self {
        UnstableRehearsalModel { counter: 0 }
    }
}

impl Default for UnstableRehearsalModel {
    fn default() -> Self {
        UnstableRehearsalModel::new()
    }
}

impl FrozenModel for UnstableRehearsalModel {
    fn name(&self) -> &str {
        "unstable rehearsal model (the free LLM)"
    }
    fn decode(&mut self, request: &str) -> String {
        self.counter += 1;
        format!("{request} — attempt {} (free text)", self.counter)
    }
    fn tokens(&self) -> usize {
        7
    }
}

/// The laboratory: the gates of the bridge, with what closes without the weights and what only closes with them.
pub fn run() {
    println!("== Point 20 :: the Jev-LLM bridge ==");

    println!("\n-- the gate of the weights --");
    let weights_dir = crate::runner::DEFAULT_DIR;
    let path = format!("{weights_dir}/{}", crate::runner::FILE_NAME);
    match crate::runner::declared_hash(weights_dir) {
        Err(refusal) => println!(
            "  REFUSED: {refusal:?} | the operator declares the sha256 in {weights_dir}/SHA256 before running, and no number comes out"
        ),
        Ok(declared) => match verify_weights(&path, &declared) {
            Ok(sha) => println!("  weights verified: {sha}"),
            Err(refusal) => match &refusal {
                Refusal::WeightsHashMismatch { expected, measured } => {
                    println!("  REFUSED: declared hash {expected}, measured {measured}")
                }
                other => println!("  REFUSED: {other:?}"),
            },
        },
    }

    println!("\n-- the restricted decoding --");
    let mut rehearsal = RehearsalModel::new();
    let p = request("does the health post stand the demand?");
    let output = rehearsal.decode(&p);
    match validate_output(&output) {
        Ok(card) => println!(
            "  the rehearsal model answers in the schema: card {} | layer {} | words of the document: {}",
            card.id,
            card.layer.code(),
            output.split_whitespace().count()
        ),
        Err(r) => println!("  the rehearsal output did not pass: {r:?}"),
    }
    let outside = "this is free text, not a card";
    match validate_output(outside) {
        Ok(_) => println!("  PROBLEM: free text accepted"),
        Err(r) => println!(
            "  free text refused: {r:?} | abstention {:?}",
            r.abstention()
        ),
    }
    let extreme: Vec<String> = output
        .lines()
        .filter(|l| !l.starts_with("margem"))
        .map(|l| l.to_string())
        .chain(std::iter::once("margem = 1.0".to_string()))
        .collect();
    match validate_output(&extreme.join("\n")) {
        Ok(_) => println!("  PROBLEM: margin at 100% accepted"),
        Err(r) => println!(
            "  margin at 100% refused: {r:?} | abstention {:?}",
            r.abstention()
        ),
    }

    println!("\n-- the determinism proof, in the form of G1 --");
    let requests: Vec<String> = (1..=4).map(|i| request(&format!("case {i}"))).collect();
    let mut accounting = Accounting::default();
    let (a, b, identical) =
        prove_determinism(&mut RehearsalModel::new(), &requests, &mut accounting);
    println!("  rehearsal model: run 1 {a:.16} | run 2 {b:.16} | identical: {identical}");
    let mut accounting2 = Accounting::default();
    let (c, d, identical2) = prove_determinism(
        &mut UnstableRehearsalModel::new(),
        &requests,
        &mut accounting2,
    );
    println!("  unstable model: run 1 {c:.16} | run 2 {d:.16} | identical: {identical2}");
    println!(
        "  reading: gate G1 closes with a frozen model and does NOT close with a model that changes its answer — which is what docs/JEV_INTEGRATION.md declares about the free LLM"
    );

    println!("\n-- the cost per call and the adequacy of the space --");
    let (tokens, ms) = accounting.per_call();
    println!(
        "  rehearsal model: {} calls | {tokens:.1} tokens per call | {ms:.1} ms per call",
        accounting.calls
    );
    let mut accounting3 = Accounting::default();
    let ad = space_adequacy(
        &mut UnstableRehearsalModel::new(),
        &requests,
        &mut accounting3,
    );
    println!(
        "  unstable model: {} requests | {} abstentions ({:.1}%) | {}",
        ad.requests,
        ad.abstentions,
        ad.fraction * 100.0,
        ad.verdict
    );
    let mut accounting4 = Accounting::default();
    let ad2 = space_adequacy(&mut RehearsalModel::new(), &requests, &mut accounting4);
    println!(
        "  rehearsal model: {} requests | {} abstentions ({:.1}%) | {}",
        ad2.requests,
        ad2.abstentions,
        ad2.fraction * 100.0,
        ad2.verdict
    );

    println!("\n-- the local runner of the frozen weights --");
    let mut runner_abstentions = 0usize;
    match crate::runner::LocalRunner::load(crate::runner::DEFAULT_DIR) {
        Ok(mut runner) => {
            println!(
                "  weights verified: {} | {} frozen answers in the artifact",
                runner.sha256(),
                runner.answers()
            );
            match runner.model() {
                Some(f) => {
                    println!(
                        "  model file declared: {} | revision {}",
                        f.sha256, f.revision
                    );
                    if f.revision.starts_with("stochology-s2/tensor-") {
                        println!(
                            "  boundary: tensor-v1 is a 96-parameter numerical MLP baseline with template card formatting (0.0% abstention by construction); open-weights LLM (GGUF) inference with and without grammar remains open"
                        );
                    }
                }
                None => println!(
                    "  model file: not declared | the hash above proves the table did not change, not which model wrote it, and the round does not enter the record"
                ),
            }
            // The adequacy and G1 are measured on the cases of the artifact itself when it stores them (v2). With a
            // v1 artifact, which has only the hash, it falls back to its own probe — and the instrument says which of
            // the two it used.
            let from_artifact = runner.requests();
            let (ps, probe) = if from_artifact.is_empty() {
                (
                    (1..=4)
                        .map(|i| request(&format!("case {i}")))
                        .collect::<Vec<String>>(),
                    "own probe of the instrument (v1 artifact, without the request text)",
                )
            } else {
                (from_artifact, "the cases of the artifact itself")
            };
            println!("  adequacy measured on: {probe} ({} cases)", ps.len());
            let mut c = Accounting::default();
            let (a, b, identical) = prove_determinism(&mut runner, &ps, &mut c);
            println!(
                "  determinism in the form of G1: {a:.16} against {b:.16} | identical: {identical}"
            );
            let mut c2 = Accounting::default();
            let ad = space_adequacy(&mut runner, &ps, &mut c2);
            runner_abstentions = ad.abstentions;
            let (tk, msq) = c2.per_call();
            println!(
                "  adequacy: {} requests | {} abstentions ({:.1}%) | {}",
                ad.requests,
                ad.abstentions,
                ad.fraction * 100.0,
                ad.verdict
            );
            println!("  measured cost: {tk:.1} tokens per call | {msq:.1} ms per call");
            println!(
                "  Brier: the rule is in runner::brier and the labeled set in calibration::load, with a minimum sample and abstention counted"
            );
        }
        Err(refusal) => match &refusal {
            Refusal::HashNotDeclared(c) => println!(
                "  REFUSED: hash not declared in {c} | the operator declares the sha256 before running"
            ),
            Refusal::WeightsMissing(c) => {
                println!(
                    "  REFUSED: weights missing in {c} | without the weights, the bridge does not run"
                )
            }
            Refusal::WeightsHashMismatch { expected, measured } => {
                println!("  REFUSED: declared hash {expected}, measured {measured}")
            }
            Refusal::ArtifactUnreadable(m) => println!("  REFUSED: unreadable artifact: {m}"),
            other => println!("  REFUSED: {other:?}"),
        },
    }
    println!("\n-- the calibration: the Brier of the frozen model --");
    match crate::calibration::load(
        crate::calibration::DEFAULT_DIR,
        runner_abstentions,
        crate::calibration::MIN_SAMPLE,
    ) {
        Ok(cal) => println!("  {}", cal.reading()),
        Err(refusal) => match &refusal {
            Refusal::HashNotDeclared(c) => println!(
                "  without a labeled set: hash not declared in {c} | the Brier demands pairs with provenance declared before running"
            ),
            Refusal::WeightsMissing(c) => println!(
                "  without a labeled set: {c} absent | without a (probability, outcome) pair there is no calibration, and the instrument does not invent one"
            ),
            other => println!("  labeled set refused: {other:?}"),
        },
    }
    println!(
        "  declared rule: 0.0 and 1.0 refuse, an id out of order refuses, an outcome outside 0 and 1 refuses, below {} pairs the score is not published, abstention leaves the denominator",
        crate::calibration::MIN_SAMPLE
    );

    let dir = std::env::temp_dir().join(format!("p20-jev-rehearsal-{}", std::process::id()));
    match crate::runner::write_rehearsal_artifact(&dir, &["case 1", "case 2", "case 3", "case 4"]) {
        Ok(hash) => match crate::runner::LocalRunner::load(&dir.to_string_lossy()) {
            Ok(mut r) => {
                let ps: Vec<String> = (1..=4).map(|i| request(&format!("case {i}"))).collect();
                let mut c = Accounting::default();
                let (_, _, identical) = prove_determinism(&mut r, &ps, &mut c);
                let mut c2 = Accounting::default();
                let ad = space_adequacy(&mut r, &ps, &mut c2);
                let (tk, msq) = c2.per_call();
                println!(
                    "  rehearsal of the runner (these are not the real weights): {hash:.16} | {} answers | G1 identical: {identical} | abstentions {:.1}% | {tk:.1} tokens per call | {msq:.1} ms per call",
                    r.answers(),
                    ad.fraction * 100.0
                );
                println!(
                    "  reading: the path artifact -> declared hash -> frozen table -> gate runs end to end; what is missing is the model file"
                );
            }
            Err(e) => {
                println!("  rehearsal of the runner: the rehearsal artifact did not load: {e:?}")
            }
        },
        Err(e) => {
            println!("  rehearsal of the runner: the rehearsal artifact could not be written: {e}")
        }
    }
    std::fs::remove_dir_all(&dir).ok();

    println!("\n-- the declared state --");
    println!(
        "  local runner: exists in crates/stochology-s2/src/runner.rs, with artifact, hash declared before the run and abstention per request with no answer"
    );
    let has_weights = crate::runner::LocalRunner::load(crate::runner::DEFAULT_DIR).is_ok();
    let cal = crate::calibration::load(
        crate::calibration::DEFAULT_DIR,
        0,
        crate::calibration::MIN_SAMPLE,
    );
    let has_score = matches!(cal, Ok(crate::calibration::Calibration::Score { .. }));
    if has_weights {
        println!(
            "  frozen weights: artifact verified in {weights_dir} | the bridge runs with it, and the hash stays declared before every run"
        );
    } else {
        println!(
            "  frozen weights: absent from the tree | the operator declares the sha256 in {weights_dir}/SHA256 and leaves the model in {weights_dir}/{}; until then, the gate refuses and no real calibration number is published",
            crate::runner::FILE_NAME
        );
    }
    if has_score {
        println!(
            "  labeled set: with a published score | the Brier above is that of the verified set, not of the rehearsal"
        );
    } else {
        println!(
            "  labeled set: without a published score | the reader exists and the set is what is missing"
        );
    }
    println!(
        "  what already closes: the gate of the weights, the restricted decoding with abstention, the determinism proof, the cost per call, the adequacy rule of the space, the runner and the reader of the labeled set"
    );
}

/// The measure of the instrument, in a machine-readable form.
///
/// `labs --jev` prints for a human; this structure exists so that the record is not typed by hand. It comes out as a
/// card (field = value), which is the format the repository already uses, and re-enters through `from_card`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Measure {
    pub weights: Option<String>,
    /// The `sha256` of the model file that the `@model` line declares; the table hash does not prove it.
    pub model: Option<String>,
    pub answers: usize,
    pub requests: usize,
    pub probe: String,
    pub abstentions: usize,
    pub abstention_fraction: f64,
    pub g1_identical: bool,
    pub g1_hash: String,
    pub tokens_per_call: f64,
    pub ms_per_call: f64,
    pub n_pairs: usize,
    pub brier: Option<f64>,
    pub ece: Option<f64>,
    pub min_sample: usize,
}

/// Measures what the instrument knows how to measure, without printing anything.
pub fn measure() -> Measure {
    let mut m = Measure {
        min_sample: crate::calibration::MIN_SAMPLE,
        probe: String::from("none: without an artifact"),
        ..Measure::default()
    };
    if let Ok(mut runner) = crate::runner::LocalRunner::load(crate::runner::DEFAULT_DIR) {
        m.weights = Some(runner.sha256().to_string());
        m.model = runner.model().map(|f| f.sha256.clone());
        m.answers = runner.answers();
        let from_artifact = runner.requests();
        let (ps, probe) = if from_artifact.is_empty() {
            (
                (1..=4)
                    .map(|i| request(&format!("case {i}")))
                    .collect::<Vec<String>>(),
                String::from("own probe (v1 artifact)"),
            )
        } else {
            (from_artifact, String::from("cases of the artifact itself"))
        };
        m.requests = ps.len();
        m.probe = probe;
        let mut c = Accounting::default();
        let (a, _, identical) = prove_determinism(&mut runner, &ps, &mut c);
        m.g1_identical = identical;
        m.g1_hash = a;
        let mut c2 = Accounting::default();
        let ad = space_adequacy(&mut runner, &ps, &mut c2);
        m.abstentions = ad.abstentions;
        m.abstention_fraction = ad.fraction;
        let (tk, msq) = c2.per_call();
        m.tokens_per_call = tk;
        m.ms_per_call = msq;
    }
    if let Ok(crate::calibration::Calibration::Score { n, brier, ece, .. }) =
        crate::calibration::load(
            crate::calibration::DEFAULT_DIR,
            m.abstentions,
            crate::calibration::MIN_SAMPLE,
        )
    {
        m.n_pairs = n;
        m.brier = Some(brier);
        m.ece = Some(ece);
    }
    m
}

/// Decimals declared per field. Writing and reading come out of here; there is no loose epsilon.
pub const FRACTION_DECIMALS: usize = 6;
pub const SCORE_DECIMALS: usize = 6;
pub const COST_DECIMALS: usize = 1;

/// Rounds to the declared decimals, so that the card is exact on the way back.
///
/// Without this, writing `1.0 / 33.0` with six decimals and reading it back gives `0.030303`, which is not the same
/// `f64` (`0.030303030303030304`): the test becomes a lottery of representation and the record gains an epsilon that
/// nobody declared. Quantizing at the edge, the comparison becomes one of equality again.
pub fn quantize(x: f64, decimals: usize) -> f64 {
    if !x.is_finite() {
        return x;
    }
    let scale = 10f64.powi(decimals as i32);
    (x * scale).round() / scale
}

impl Measure {
    /// The measure with every numeric field reduced to the decimals that the card carries.
    pub fn quantized(mut self) -> Measure {
        self.abstention_fraction = quantize(self.abstention_fraction, FRACTION_DECIMALS);
        self.tokens_per_call = quantize(self.tokens_per_call, COST_DECIMALS);
        self.ms_per_call = quantize(self.ms_per_call, COST_DECIMALS);
        self.brier = self.brier.map(|v| quantize(v, SCORE_DECIMALS));
        self.ece = self.ece.map(|v| quantize(v, SCORE_DECIMALS));
        self
    }
}

fn score_field(v: Option<f64>, decimals: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", decimals, x),
        _ => String::from("(absent)"),
    }
}

/// The measure as a card, one line per field.
pub fn measure_card(m: &Measure) -> String {
    let m = m.clone().quantized();
    let absent = String::from("(absent)");
    let mut s = String::new();
    s.push_str(&format!(
        "weights = {}\n",
        m.weights.clone().unwrap_or(absent.clone())
    ));
    s.push_str(&format!(
        "model = {}\n",
        m.model.clone().unwrap_or(absent.clone())
    ));
    s.push_str(&format!("answers = {}\n", m.answers));
    s.push_str(&format!("requests = {}\n", m.requests));
    s.push_str(&format!("probe = {}\n", m.probe));
    s.push_str(&format!("abstentions = {}\n", m.abstentions));
    s.push_str(&format!(
        "abstention_fraction = {:.*}\n",
        FRACTION_DECIMALS, m.abstention_fraction
    ));
    s.push_str(&format!("g1_identical = {}\n", m.g1_identical));
    s.push_str(&format!("g1_hash = {}\n", m.g1_hash));
    s.push_str(&format!(
        "tokens_per_call = {:.*}\n",
        COST_DECIMALS, m.tokens_per_call
    ));
    s.push_str(&format!(
        "ms_per_call = {:.*}\n",
        COST_DECIMALS, m.ms_per_call
    ));
    s.push_str(&format!("n_pairs = {}\n", m.n_pairs));
    s.push_str(&format!(
        "brier = {}\n",
        score_field(m.brier, SCORE_DECIMALS)
    ));
    s.push_str(&format!("ece = {}\n", score_field(m.ece, SCORE_DECIMALS)));
    s.push_str(&format!("min_sample = {}\n", m.min_sample));
    s
}

/// Reads the card back into the measure. An absent field becomes the default, an invalid numeric field is an error.
pub fn from_card(text: &str) -> Result<Measure, String> {
    let mut m = Measure::default();
    for line in text.lines() {
        let (key, value) = match line.split_once('=') {
            Some(pair) => (pair.0.trim(), pair.1.trim()),
            None => continue,
        };
        let number = |v: &str| -> Result<f64, String> {
            let x = v
                .parse::<f64>()
                .map_err(|_| format!("{key}: {v:?} is not a number"))?;
            if !x.is_finite() {
                return Err(format!(
                    "{key}: {v:?} is not a finite number; absence is written (absent)"
                ));
            }
            Ok(x)
        };
        let optional = |v: &str| -> Result<Option<f64>, String> {
            if v == "(absent)" || v.is_empty() {
                return Ok(None);
            }
            number(v).map(Some)
        };
        let integer = |v: &str| {
            v.parse::<usize>()
                .map_err(|_| format!("{key}: {v:?} is not an integer"))
        };
        match key {
            "weights" => {
                m.weights = if value == "(absent)" || value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                }
            }
            "model" => {
                m.model = if value == "(absent)" || value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                }
            }
            "answers" => m.answers = integer(value)?,
            "requests" => m.requests = integer(value)?,
            "probe" => m.probe = value.to_string(),
            "abstentions" => m.abstentions = integer(value)?,
            "abstention_fraction" => m.abstention_fraction = number(value)?,
            "g1_identical" => m.g1_identical = value == "true",
            "g1_hash" => m.g1_hash = value.to_string(),
            "tokens_per_call" => m.tokens_per_call = number(value)?,
            "ms_per_call" => m.ms_per_call = number(value)?,
            "n_pairs" => m.n_pairs = integer(value)?,
            "brier" => m.brier = optional(value)?,
            "ece" => m.ece = optional(value)?,
            "min_sample" => m.min_sample = integer(value)?,
            _ => {}
        }
    }
    Ok(m)
}

/// The registry line of a round, or the reason there is no line.
///
/// The rule is declared beforehand: without a verified weight, without a declared model file, without G1 closed, without a minimum sample and
/// without Brier and ECE, there is no line. Registering half a measure would be worse than registering nothing.
pub fn registry_line(m: &Measure, date: &str) -> Result<String, String> {
    let date = date.trim();
    if date.len() != 10 || date.chars().filter(|c| *c == '-').count() != 2 {
        return Err(String::from(
            "the date has to come in the format YYYY-MM-DD",
        ));
    }
    let weights = m
        .weights
        .clone()
        .ok_or("without a verified weight: the record demands the sha256 of the artifact")?;
    let model = m.model.clone().ok_or(
        "without a declared model file: the artifact hash proves the table did not change, not which model wrote it",
    )?;
    if !m.g1_identical {
        return Err(String::from(
            "G1 did not close: two runs with different answers do not become a record",
        ));
    }
    if m.n_pairs < m.min_sample {
        return Err(format!(
            "insufficient sample: {} pairs against the minimum of {}",
            m.n_pairs, m.min_sample
        ));
    }
    let brier = m.brier.ok_or("without a measured Brier")?;
    let ece = m.ece.ok_or("without a measured ECE")?;
    if !brier.is_finite() || !ece.is_finite() {
        return Err(String::from(
            "a non-finite Brier or ECE does not become a record: a score has to be a number",
        ));
    }
    Ok(format!(
        "{date}\t{weights}\t{model}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{brier:.6}\t{ece:.6}",
        m.requests, m.abstentions, m.n_pairs, m.tokens_per_call, m.ms_per_call
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_verified_weights_the_bridge_does_not_run() {
        let r = verify_weights("empirical/p20-jev/pesos/does-not-exist.bin", "abc");
        match r {
            Err(Refusal::WeightsMissing(c)) => assert!(c.contains("does-not-exist")),
            other => panic!("expected missing weights: {other:?}"),
        }
        assert_eq!(
            verify_weights("empirical/p20-jev/pesos/does-not-exist.bin", "abc")
                .unwrap_err()
                .abstention(),
            Category::Others
        );
    }

    #[test]
    fn weights_with_a_different_hash_refuse() {
        let path = std::env::temp_dir().join("rehearsal_weights.bin");
        std::fs::write(&path, b"rehearsal weights").expect("writes");
        let expected = stochology_core::audit::sha256_hex(b"any other thing");
        let r = verify_weights(&path.to_string_lossy(), &expected);
        assert!(
            matches!(r, Err(Refusal::WeightsHashMismatch { .. })),
            "{r:?}"
        );
        let right = stochology_core::audit::sha256_hex(b"rehearsal weights");
        assert!(
            verify_weights(&path.to_string_lossy(), &right).is_ok(),
            "with the right hash, it passes"
        );
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn an_output_outside_the_schema_becomes_an_abstention() {
        let r = validate_output("this is free text, not a card");
        assert!(matches!(r, Err(Refusal::OutOfSchema(_))), "{r:?}");
        assert_eq!(
            r.unwrap_err().abstention(),
            Category::Others,
            "the abstention is the auditable \"I do not know\""
        );
    }

    #[test]
    fn the_margin_at_the_extreme_is_refused() {
        let mut rehearsal = RehearsalModel::new();
        let clean = rehearsal.decode(&request("case"));
        assert!(
            clean.contains("margem = "),
            "the rehearsal card declares a margin"
        );
        let with_extreme: Vec<String> = clean
            .lines()
            .filter(|l| !l.starts_with("margem"))
            .map(|l| l.to_string())
            .chain(std::iter::once("margem = 1.0".to_string()))
            .collect();
        let refusal = validate_output(&with_extreme.join("\n"))
            .expect_err("the margin at the extreme had to be refused");
        assert!(
            matches!(refusal, Refusal::DogmaticExtreme(_)),
            "{refusal:?}"
        );
        assert_eq!(refusal.abstention(), Category::Others);
    }

    #[test]
    fn the_determinism_proof_passes_on_the_rehearsal_model() {
        let mut accounting = Accounting::default();
        let requests: Vec<String> = (1..=3).map(|i| request(&format!("case {i}"))).collect();
        let (a, b, identical) =
            prove_determinism(&mut RehearsalModel::new(), &requests, &mut accounting);
        assert!(identical, "the frozen model closes G1: {a} against {b}");
        assert_eq!(accounting.calls, 6, "two runs of three requests");
    }

    #[test]
    fn the_determinism_proof_fails_the_unstable_model() {
        let mut accounting = Accounting::default();
        let requests: Vec<String> = (1..=3).map(|i| request(&format!("case {i}"))).collect();
        let (a, b, identical) = prove_determinism(
            &mut UnstableRehearsalModel::new(),
            &requests,
            &mut accounting,
        );
        assert!(
            !identical,
            "the model that changes its answer does NOT close G1: {a} against {b}"
        );
    }

    #[test]
    fn the_measure_goes_and_returns_through_the_card() {
        let m = Measure {
            weights: Some(String::from("abc")),
            model: Some(String::from("def")),
            answers: 32,
            requests: 32,
            probe: String::from("cases of the artifact itself"),
            abstentions: 1,
            abstention_fraction: 1.0 / 33.0,
            g1_identical: true,
            g1_hash: String::from("deadbeef"),
            tokens_per_call: 95.25,
            ms_per_call: 1.5,
            n_pairs: 32,
            brier: Some(0.25),
            ece: Some(0.125),
            min_sample: 30,
        };
        let card = measure_card(&m);
        let back = from_card(&card).expect("reads back");
        assert_eq!(back.weights, m.weights);
        assert_eq!(back.model, m.model);
        assert_eq!(back.requests, 32);
        assert_eq!(back.abstentions, 1);
        assert!(back.g1_identical);
        assert!((back.brier.unwrap() - 0.25).abs() < 1e-12);
        assert!((back.ece.unwrap() - 0.125).abs() < 1e-12);
        assert_eq!(
            back,
            m.quantized(),
            "the card comes back identical after the declared quantization"
        );
    }

    #[test]
    fn the_declared_quantization_closes_one_third() {
        let m = Measure {
            abstention_fraction: 1.0 / 33.0,
            tokens_per_call: 95.25,
            ms_per_call: 1.04,
            ..Measure::default()
        };
        let back = from_card(&measure_card(&m)).expect("reads back");
        assert_eq!(
            back.abstention_fraction, 0.030303,
            "six decimals, no epsilon"
        );
        assert_eq!(back.tokens_per_call, 95.3, "one decimal: 95.25 goes up");
        assert_eq!(back.ms_per_call, 1.0, "one decimal: 1.04 goes down");
        assert_eq!(
            back,
            m.quantized(),
            "quantizing at the edge makes the round trip an equality"
        );
        assert_ne!(
            back.abstention_fraction,
            1.0 / 33.0,
            "the card is lossy by declaration, not by accident"
        );
    }

    #[test]
    fn absence_returns_absent_and_nan_is_refused() {
        let m = Measure {
            weights: Some(String::from("abc")),
            ..Measure::default()
        };
        let card = measure_card(&m);
        assert!(card.contains("brier = (absent)"), "{card}");
        let back = from_card(&card).expect("reads back");
        assert_eq!(back.brier, None, "absence does not come back as NaN");
        assert_eq!(back.ece, None, "absence does not come back as NaN");
        assert!(
            from_card("brier = NaN\n").is_err(),
            "NaN does not enter through the reading"
        );
        assert!(
            from_card("ece = inf\n").is_err(),
            "infinity does not enter through the reading"
        );
        assert!(
            from_card("abstention_fraction = (absent)\n").is_err(),
            "a mandatory field does not stay absent"
        );
    }

    #[test]
    fn a_non_finite_score_does_not_become_a_record() {
        let m = Measure {
            weights: Some(String::from("abc")),
            model: Some(String::from("def")),
            g1_identical: true,
            n_pairs: 32,
            min_sample: 30,
            brier: Some(f64::NAN),
            ece: Some(0.1),
            ..Measure::default()
        };
        let reason = registry_line(&m, "2026-10-17").expect_err("NaN does not become a line");
        assert!(reason.contains("finite"), "{reason}");
    }

    #[test]
    fn the_record_refuses_half_a_measure() {
        let complete = Measure {
            weights: Some(String::from("abc")),
            model: Some(String::from("def")),
            g1_identical: true,
            n_pairs: 32,
            min_sample: 30,
            brier: Some(0.2),
            ece: Some(0.1),
            ..Measure::default()
        };
        assert!(
            registry_line(&complete, "2026-10-17").is_ok(),
            "a complete measure registers"
        );

        let without_weight = Measure {
            weights: None,
            ..complete.clone()
        };
        assert!(
            registry_line(&without_weight, "2026-10-17")
                .unwrap_err()
                .contains("sha256")
        );

        let without_model = Measure {
            model: None,
            ..complete.clone()
        };
        assert!(
            registry_line(&without_model, "2026-10-17")
                .unwrap_err()
                .contains("model file"),
            "the table hash alone does not register a round"
        );

        let without_g1 = Measure {
            g1_identical: false,
            ..complete.clone()
        };
        assert!(
            registry_line(&without_g1, "2026-10-17")
                .unwrap_err()
                .contains("G1")
        );

        let short_sample = Measure {
            n_pairs: 3,
            ..complete.clone()
        };
        assert!(
            registry_line(&short_sample, "2026-10-17")
                .unwrap_err()
                .contains("sample")
        );

        let without_score = Measure {
            brier: None,
            ..complete.clone()
        };
        assert!(
            registry_line(&without_score, "2026-10-17")
                .unwrap_err()
                .contains("Brier")
        );

        assert!(
            registry_line(&complete, "17/10/2026")
                .unwrap_err()
                .contains("YYYY-MM-DD")
        );
    }

    #[test]
    fn the_cost_per_call_and_the_adequacy_come_out_of_the_rehearsal() {
        let mut accounting = Accounting::default();
        let requests: Vec<String> = (1..=4).map(|i| request(&format!("case {i}"))).collect();
        let ad = space_adequacy(&mut RehearsalModel::new(), &requests, &mut accounting);
        assert_eq!(
            ad.abstentions, 0,
            "the rehearsal model answers in the schema"
        );
        assert!(ad.verdict.contains("serves this model"), "{}", ad.verdict);
        let (tokens, _) = accounting.per_call();
        assert!((tokens - 42.0).abs() < 1e-9, "tokens per call: {tokens}");
        let mut accounting2 = Accounting::default();
        let ad2 = space_adequacy(
            &mut UnstableRehearsalModel::new(),
            &requests,
            &mut accounting2,
        );
        assert_eq!(
            ad2.abstentions, 4,
            "the unstable model abstains on all of them"
        );
        assert!(ad2.verdict.contains("does NOT serve"), "{}", ad2.verdict);
    }
}
