//! The H3 chain, re-executed as far as the tree allows.
//!
//! The empirical record brings four lines of H3, three with the mark **P** (record only) and one with **D**
//! (declared divergence): the six flows under strict clock with canary, the Rhode Island vault, the
//! performative loop measured (Goodhart) and the H3 hypergraph. This module walks the four and, for each one, says
//! **what can be redone** and **what is left only to declare** — because the raw data of three of them left the tree
//! with the Python scaffolding (`docs/declarations/REMOVED_NON_SIGNIFICANT.md`).
//!
//! Three classes, declared in `empirical/p14-h3/DECLARACAO.md`:
//!
//! - **measured**: the raw data is in the tree and the instrument measures again;
//! - **derived**: the number is in a versioned record and the instrument re-derives the arithmetic and the verdict;
//! - **record**: the raw data is not in the tree; what remains is the declaration of what was measured and why it is not redone.
//!
//! The only derivable line today is the loop: `empirical/p06-laco/resultados.json` is in the tree, with the 2SLS
//! estimate, the interval by day bootstrap and the first-stage F. Against it the instrument redoes the account of the
//! criterion declared in the front's pre-registration — sign, order of magnitude, strict compatibility and strength of the
//! instrument — and **it knows how to fail**: a record in which the ratio falls inside the declared band changes the verdict,
//! and a record in which the ratio field does not match the account fails.
//!
//! Run: ~labs --h3~

/// The machine record of the four lines of the chain.
pub const REGISTRY: &str = "empirical/p14-h3/registro.tsv";
/// The versioned record of the measured performative loop (background point 6).
pub const LOOP_RESULTS: &str = "empirical/p06-laco/resultados.json";
/// The crude correction made after seeing the result.
pub const LOOP_POSTHOC: &str = "empirical/p06-laco/posthoc.json";

/// What can be done with a line of the chain.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Class {
    /// The raw data is in the tree: the instrument measures again.
    Measured,
    /// The number is in a versioned record: the instrument re-derives the account and the verdict.
    Derived,
    /// The raw data is not in the tree: the declaration remains.
    Record,
}

impl Class {
    pub fn from_line(text: &str) -> Option<Class> {
        match text {
            "medida" => Some(Class::Measured),
            "derivada" => Some(Class::Derived),
            "registro" => Some(Class::Record),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Class::Measured => "measured",
            Class::Derived => "derived",
            Class::Record => "record",
        }
    }
}

/// A line of the chain, with what it claims and what can be redone.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub line: String,
    pub claim: String,
    pub class: Class,
    /// The mark of the empirical record: R, D, P or V (the re-derived verdict).
    pub mark: String,
    /// The versioned file that supports the re-derivation ("-" when there is none).
    pub record: String,
    pub numbers: String,
    pub command: String,
}

/// The record of the chain, with the stamp.
#[derive(Debug, Clone, PartialEq)]
pub struct Chain {
    pub stamp: String,
    pub lines: Vec<Line>,
}

/// The criterion declared before running, read from the machine record (the source is in the front's pre-registration).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoopCriterion {
    pub declared_r: f64,
    pub min_ratio: f64,
    pub max_ratio: f64,
    pub min_f: f64,
}

/// What the re-derivation of the loop returns.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopVerdict {
    pub r_iv: f64,
    pub ci90: (f64, f64),
    pub first_stage_f: f64,
    /// The ratio recomputed here: declared r / measured r.
    pub ratio: f64,
    /// The ratio that the record itself brings, for checking.
    pub ratio_in_record: f64,
    pub sign_matches: bool,
    pub order_matches: bool,
    pub strict: bool,
    /// The containment field that the record brings, for checking.
    pub containment_in_record: bool,
    pub naive_bias: f64,
    pub verdict: String,
    pub divergences: Vec<String>,
}

/// Reads the chain record: `# key<TAB>value` comments and the tab-separated table.
pub fn read_chain(text: &str) -> Result<Chain, String> {
    let mut stamp = String::new();
    let mut lines = Vec::new();
    for line in text.lines() {
        let l = line.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        if let Some(rest) = l.strip_prefix('#') {
            let fields: Vec<&str> = rest.trim().split('\t').collect();
            if fields.len() >= 2 && fields[0] == "registros" {
                stamp = fields[1].trim().to_string();
            }
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() < 7 || fields[0] == "linha" {
            continue;
        }
        let class =
            Class::from_line(fields[2]).ok_or_else(|| format!("unknown class: {}", fields[2]))?;
        lines.push(Line {
            line: fields[0].to_string(),
            claim: fields[1].to_string(),
            class,
            mark: fields[3].to_string(),
            record: fields[4].to_string(),
            numbers: fields[5].to_string(),
            command: fields[6].to_string(),
        });
    }
    if stamp.is_empty() || lines.is_empty() {
        return Err("chain record without stamp or without lines".to_string());
    }
    Ok(Chain { stamp, lines })
}

/// Reads the criterion of the loop, which lives in the machine record with the declared source.
pub fn read_criterion(text: &str) -> Result<LoopCriterion, String> {
    let mut declared_r = None;
    let mut min_ratio = None;
    let mut max_ratio = None;
    let mut min_f = None;
    for line in text.lines() {
        let l = line.trim_end();
        let Some(rest) = l.strip_prefix('#') else {
            continue;
        };
        let fields: Vec<&str> = rest.trim().split('\t').collect();
        if fields.len() < 2 {
            continue;
        }
        let number = |v: &str| {
            v.trim()
                .parse::<f64>()
                .map_err(|_| format!("{v} is not a number"))
        };
        match fields[0] {
            "r_declarado" => declared_r = Some(number(fields[1])?),
            "razao_minima" => min_ratio = Some(number(fields[1])?),
            "razao_maxima" => max_ratio = Some(number(fields[1])?),
            "f_minimo" => min_f = Some(number(fields[1])?),
            _ => {}
        }
    }
    match (declared_r, min_ratio, max_ratio, min_f) {
        (Some(r), Some(lo), Some(hi), Some(f)) => Ok(LoopCriterion {
            declared_r: r,
            min_ratio: lo,
            max_ratio: hi,
            min_f: f,
        }),
        _ => Err("loop criterion incomplete in the record".to_string()),
    }
}

/// A minimal, declared JSON field reader: it finds the key and reads what comes after. It is not a general parser;
/// it is enough for the loop's record, and the test against the real file pins the result.
fn after_key<'a>(js: &'a str, key: &str) -> Option<&'a str> {
    let target = format!("\"{key}\"");
    let pos = js.find(&target)? + target.len();
    let rest = js[pos..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    Some(rest)
}

/// The first number after the key.
pub fn number(js: &str, key: &str) -> Option<f64> {
    let rest = after_key(js, key)?;
    let end = rest
        .find(|c: char| {
            !(c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E')
        })
        .unwrap_or(rest.len());
    rest[..end].parse::<f64>().ok()
}

/// The first list of numbers after the key.
pub fn list(js: &str, key: &str) -> Option<Vec<f64>> {
    let rest = after_key(js, key)?;
    let start = rest.find('[')? + 1;
    let end = rest.find(']')?;
    rest[start..end]
        .split(',')
        .map(|p| p.trim().parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>()
}

/// The first boolean after the key.
pub fn boolean(js: &str, key: &str) -> Option<bool> {
    let rest = after_key(js, key)?;
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

/// Redoes the account of the criterion declared in the pre-registration of point 6, against the versioned record.
pub fn rederive_loop(
    resultados: &str,
    posthoc: &str,
    criterion: LoopCriterion,
) -> Result<LoopVerdict, String> {
    let r_iv = number(resultados, "r_IV").ok_or("record without r_IV")?;
    let ic = list(resultados, "ic90").ok_or("record without ic90")?;
    if ic.len() != 2 {
        return Err("ic90 without two extremes".to_string());
    }
    let f = number(resultados, "F_primeiro_estagio").ok_or("record without the first-stage F")?;
    let ratio_in_record =
        number(resultados, "razao_0,6_sobre_r_IV").ok_or("record without the ratio")?;
    let containment_in_record =
        boolean(resultados, "0,6_no_ic90").ok_or("record without the containment field")?;
    let bias =
        number(resultados, "vies_ingenuo_vs_exogena").ok_or("record without the naive bias")?;

    let ratio = criterion.declared_r / r_iv;
    let sign_matches = ic[0] > 0.0;
    let order_matches = ratio >= criterion.min_ratio && ratio <= criterion.max_ratio;
    let strict = criterion.declared_r >= ic[0] && criterion.declared_r <= ic[1];

    let mut divergences = Vec::new();
    if (ratio - ratio_in_record).abs() > 1e-9 {
        divergences.push(format!(
            "the record's ratio ({ratio_in_record:.6}) does not match the account ({ratio:.6})"
        ));
    }
    if containment_in_record != strict {
        divergences.push(format!(
            "the record's containment ({containment_in_record}) does not match the account ({strict})"
        ));
    }
    let verdict = if f < criterion.min_f {
        "undecided: the instrument is weak (F below the declared minimum)".to_string()
    } else if !sign_matches {
        "does not match: the 90% CI includes 0 or stays below 0".to_string()
    } else if order_matches {
        format!(
            "the H3 loop has the sign and the order of the measured one (ratio {ratio:.4} inside the band)"
        )
    } else {
        format!(
            "the H3 loop overestimates the measured one: the sign matches and the order does not (ratio {ratio:.4}, band from {:.4} to {:.4})",
            criterion.min_ratio, criterion.max_ratio
        )
    };
    // the post-hoc only enters the report if it exists, and it decides nothing
    let _ = number(
        posthoc,
        "r_IV / P(fila depois da desistência) (correção grosseira, supõe novas chamadas como pedidos comuns)",
    );
    Ok(LoopVerdict {
        r_iv,
        ci90: (ic[0], ic[1]),
        first_stage_f: f,
        ratio,
        ratio_in_record,
        sign_matches,
        order_matches,
        strict,
        containment_in_record,
        naive_bias: bias,
        verdict,
        divergences,
    })
}

/// Report of the chain: one line per H3 line, with what was redone and what is left only to declare.
pub fn report(chain: &Chain, verdict: Option<&LoopVerdict>) -> String {
    let mut s = String::new();
    s.push_str(&format!("record stamp: {}\n", chain.stamp));
    let width = chain
        .lines
        .iter()
        .map(|l| l.record.chars().count())
        .chain(std::iter::once("versioned record".chars().count()))
        .max()
        .unwrap_or(0)
        + 2;
    s.push_str(&format!(
        "{:<16} {:<9} {:<4} {:<width$} {}\n",
        "line", "class", "mark", "versioned record", "what the instrument does"
    ));
    for l in &chain.lines {
        let what = match l.class {
            Class::Derived if verdict.is_some() => "re-derives the account and the verdict",
            Class::Derived => "does NOT re-derive: the versioned record was not read",
            Class::Measured => "measures again from the data in the tree",
            Class::Record => "declares: the raw data is not in the tree",
        };
        s.push_str(&format!(
            "{:<16} {:<9} {:<4} {:<width$} {}\n",
            l.line,
            l.class.name(),
            l.mark,
            l.record,
            what
        ));
    }
    match verdict {
        Some(v) => {
            s.push_str("\n-- the performative loop, re-derived from the record in the tree --\n");
            s.push_str(&format!(
                "2SLS estimate: r = {:.10} | 90% CI from {:.10} to {:.10} | first-stage F = {:.2}\n",
                v.r_iv, v.ci90.0, v.ci90.1, v.first_stage_f
            ));
            s.push_str(&format!(
                "ratio declared r / measured r: {:.10} (the record brings {:.10}) | declared band: {:.4} to {:.4}\n",
                v.ratio, v.ratio_in_record, 1.0 / 3.0, 3.0
            ));
            s.push_str(&format!(
                "sign matches: {} | order of magnitude matches: {} | compatible in the strict sense: {} (the record says {})\n",
                v.sign_matches, v.order_matches, v.strict, v.containment_in_record
            ));
            s.push_str(&format!(
                "bias of the naive forecaster against exogenous demand: {:.4}\n",
                v.naive_bias
            ));
            if v.divergences.is_empty() {
                s.push_str("divergences between the account and the record: none\n");
            } else {
                s.push_str(&format!(
                    "divergences between the account and the record: FAILED at {:?}\n",
                    v.divergences
                ));
            }
            s.push_str(&format!("verdict: {}\n", v.verdict));
        }
        None => s.push_str("\nthe loop was not re-derived: the versioned record was not read\n"),
    }
    s
}

/// The laboratory: the H3 chain, the negative proof and the positive proof.
pub fn run(path: &str, results_path: &str, posthoc_path: &str) {
    println!("== Point 14 :: the H3 chain, re-executed as far as the tree allows ==");
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            println!("chain record not read ({path}): {e}");
            return;
        }
    };
    let chain = match read_chain(&text) {
        Ok(c) => c,
        Err(e) => {
            println!("invalid chain record: {e}");
            return;
        }
    };
    let criterion = read_criterion(&text);
    let verdict = match (
        criterion,
        std::fs::read_to_string(results_path),
        std::fs::read_to_string(posthoc_path),
    ) {
        (Ok(c), Ok(r), Ok(p)) => rederive_loop(&r, &p, c).ok(),
        _ => None,
    };
    print!("{}", report(&chain, verdict.as_ref()));

    println!("\n-- negative proof: a record in which the ratio falls inside the declared band --");
    let inside = format!(
        "{{\"principal_15min\": {{\"r_IV\": {}, \"ic90\": [{}, {}], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": {}, \"0,6_no_ic90\": false, \"vies_ingenuo_vs_exogena\": 0.29}}}}",
        0.4,
        0.35,
        0.45,
        0.6 / 0.4
    );
    if let Ok(c) = read_criterion(&text) {
        match rederive_loop(&inside, "{}", c) {
            Ok(v) => println!(
                "  with measured r 0.4: order matches = {} | verdict: {}",
                v.order_matches, v.verdict
            ),
            Err(e) => println!("  synthetic case not read: {e}"),
        }
    }

    println!(
        "-- negative proof 2: a record in which the ratio field does not match the account --"
    );
    let liar = "{\"principal_15min\": {\"r_IV\": 0.1446378859, \"ic90\": [0.1319811884, 0.1568385251], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": 2.0, \"0,6_no_ic90\": true, \"vies_ingenuo_vs_exogena\": 0.29}}";
    if let Ok(c) = read_criterion(&text) {
        match rederive_loop(liar, "{}", c) {
            Ok(v) => println!("  divergences: {:?}", v.divergences),
            Err(e) => println!("  synthetic case not read: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("root")
            .to_path_buf()
    }
    fn read_file(rel: &str) -> String {
        std::fs::read_to_string(root().join(rel)).expect("record file")
    }

    #[test]
    fn the_chain_has_four_lines_with_the_marks_of_the_record() {
        let chain = read_chain(&read_file(REGISTRY)).expect("chain record");
        assert_eq!(chain.lines.len(), 4, "the four lines of H3");
        let marks: Vec<&str> = chain.lines.iter().map(|l| l.mark.as_str()).collect();
        assert_eq!(marks, vec!["P", "D", "V", "P"]);
        let classes: Vec<&str> = chain.lines.iter().map(|l| l.class.name()).collect();
        assert_eq!(classes, vec!["record", "record", "derived", "record"]);
    }

    #[test]
    fn the_loop_is_rederived_from_the_versioned_record() {
        let criterion = read_criterion(&read_file(REGISTRY)).expect("criterion");
        assert_eq!(criterion.declared_r, 0.6);
        let v = rederive_loop(
            &read_file(LOOP_RESULTS),
            &read_file(LOOP_POSTHOC),
            criterion,
        )
        .expect("re-derivation");
        assert!(
            (v.r_iv - 0.1446378859145094).abs() < 1e-12,
            "measured r: {}",
            v.r_iv
        );
        assert!((v.ci90.0 - 0.131981188365084).abs() < 1e-12);
        assert!(
            (v.ratio - 4.148290720694298).abs() < 1e-9,
            "ratio: {}",
            v.ratio
        );
        assert!((v.ratio - v.ratio_in_record).abs() < 1e-9);
        assert!(v.sign_matches);
        assert!(!v.order_matches, "the ratio 4.148 stays outside the band");
        assert!(!v.strict, "0.6 is not in the 90% CI");
        assert!(
            v.divergences.is_empty(),
            "the account matches the record: {:?}",
            v.divergences
        );
        assert!(
            v.verdict.starts_with("the H3 loop overestimates"),
            "verdict: {}",
            v.verdict
        );
    }

    #[test]
    fn the_verdict_flips_when_the_ratio_enters_the_band() {
        let criterion = read_criterion(&read_file(REGISTRY)).expect("criterion");
        let inside = "{\"principal_15min\": {\"r_IV\": 0.4, \"ic90\": [0.35, 0.45], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": 1.5, \"0,6_no_ic90\": false, \"vies_ingenuo_vs_exogena\": 0.29}}";
        let v = rederive_loop(inside, "{}", criterion).expect("re-derivation");
        assert!(v.order_matches);
        assert!(
            v.verdict
                .starts_with("the H3 loop has the sign and the order"),
            "verdict: {}",
            v.verdict
        );
    }

    #[test]
    fn the_record_that_lies_in_the_ratio_fails() {
        let criterion = read_criterion(&read_file(REGISTRY)).expect("criterion");
        let liar = "{\"principal_15min\": {\"r_IV\": 0.1446378859, \"ic90\": [0.1319811884, 0.1568385251], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": 2.0, \"0,6_no_ic90\": true, \"vies_ingenuo_vs_exogena\": 0.29}}";
        let v = rederive_loop(liar, "{}", criterion).expect("re-derivation");
        assert_eq!(
            v.divergences.len(),
            2,
            "the ratio and the containment diverge: {:?}",
            v.divergences
        );
    }

    #[test]
    fn the_weak_instrument_is_undecided() {
        let criterion = read_criterion(&read_file(REGISTRY)).expect("criterion");
        let weak = "{\"principal_15min\": {\"r_IV\": 0.4, \"ic90\": [0.35, 0.45], \"F_primeiro_estagio\": 4.0, \"razao_0,6_sobre_r_IV\": 1.5, \"0,6_no_ic90\": false, \"vies_ingenuo_vs_exogena\": 0.29}}";
        let v = rederive_loop(weak, "{}", criterion).expect("re-derivation");
        assert!(v.verdict.starts_with("undecided"), "verdict: {}", v.verdict);
    }

    #[test]
    fn a_derived_line_without_a_versioned_record_fails() {
        let criterion = read_criterion(&read_file(REGISTRY)).expect("criterion");
        let missing = "{\"principal_15min\": {\"ic90\": [0.1, 0.2]}}";
        assert!(
            rederive_loop(missing, "{}", criterion).is_err(),
            "without r_IV there is no re-derivation"
        );
    }

    #[test]
    fn the_json_reader_reads_the_real_record() {
        let js = read_file(LOOP_RESULTS);
        assert!((number(&js, "chamadas_indice").expect("field") - 198980.0).abs() < 1e-9);
        let ic = list(&js, "ic90").expect("list");
        assert_eq!(ic.len(), 2);
        assert_eq!(boolean(&js, "0,6_no_ic90"), Some(false));
    }
}
