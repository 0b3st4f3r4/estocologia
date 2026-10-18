//! Point 13 of the research: listening to those who pay for the error and the seal on the box.
//!
//! The program has seven decisions that touch real people and the registry says **none** heard across the seven
//! (`governance/GRUPOS.md`). The protocol of this front is in `empirical/p13-escuta/protocolo.md`, the machine
//! registry in `empirical/p13-escuta/registro.tsv`, and this module is what reads the registry and **knows how to
//! reject**: a decision that claims to be issued with the "heard" column at none is a divergence, and while the listening
//! coverage does not reach the declared minimum the box stays sealed.
//!
//! Run: ~labs --listening~

use std::path::Path;

/// The registry path, relative to the repository root — the same convention as the instruments that read `data/`.
pub const REGISTRY: &str = "empirical/p13-escuta/registro.tsv";

/// A decision of the program, with whoever pays for its error.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub decision: String,
    pub groups: String,
    /// How many holders the decision reaches, when the number exists ("-" when it does not).
    pub holders: String,
    /// Who was heard; "nenhum" while nobody has spoken.
    pub heard: String,
    /// Where the signed record of the listening session is; "-" while there is none.
    pub record: String,
    /// Whether the decision may be issued as a guideline: "sim" or "nao".
    pub may_issue: String,
}

impl Row {
    /// Listening exists when someone was heard **and** the signed record is pointed to.
    pub fn has_listening(&self) -> bool {
        self.heard != "nenhum" && self.heard != "-" && self.record != "-" && !self.record.is_empty()
    }

    /// The divergence the gate catches: the decision claims to be issued with nobody heard.
    pub fn divergent(&self) -> bool {
        self.may_issue == "sim" && !self.has_listening()
    }
}

/// The whole registry, with the stamp, the window deadline and the declared minimum coverage.
#[derive(Debug, Clone, PartialEq)]
pub struct Registry {
    pub stamp: String,
    pub deadline: String,
    /// How the listening is conducted: "protocolo cego" or "facilitador neutro", declared before the first contact.
    pub conduct: String,
    pub min_groups: usize,
    pub rows: Vec<Row>,
}

/// What the registry decides: how many groups have been heard, whether the box remains sealed and who is divergent.
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub coverage: usize,
    pub min_groups: usize,
    pub sealed: bool,
    pub divergences: Vec<String>,
}

/// Reads the registry: comment lines with `# key<TAB>value` and the tab-separated table.
pub fn parse(text: &str) -> Result<Registry, String> {
    let mut stamp = String::new();
    let mut deadline = String::new();
    let mut conduct = String::new();
    let mut min_groups: Option<usize> = None;
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.trim().is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('#') {
            let fields: Vec<&str> = rest.trim().split('\t').collect();
            if fields.len() >= 2 {
                match fields[0] {
                    "registros" => stamp = fields[1].trim().to_string(),
                    "prazo" => deadline = fields[1].trim().to_string(),
                    "conducao" => conduct = fields[1].trim().to_string(),
                    "grupos_minimos" => {
                        min_groups = Some(
                            fields[1]
                                .trim()
                                .parse()
                                .map_err(|_| "grupos_minimos is not a number")?,
                        )
                    }
                    _ => {}
                }
            }
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 6 || fields[0] == "decisao" {
            continue;
        }
        rows.push(Row {
            decision: fields[0].to_string(),
            groups: fields[1].to_string(),
            holders: fields[2].to_string(),
            heard: fields[3].to_string(),
            record: fields[4].to_string(),
            may_issue: fields[5].to_string(),
        });
    }
    if deadline.is_empty() || min_groups.is_none() || conduct.is_empty() {
        return Err("registry without deadline, without conduct or without min_groups".to_string());
    }
    Ok(Registry {
        stamp,
        deadline,
        conduct,
        min_groups: min_groups.unwrap(),
        rows,
    })
}

/// Reads the registry from a file. Returns the error instead of panicking, so the gate can reject.
pub fn parse_file(path: &str) -> Result<Registry, String> {
    if !Path::new(path).exists() {
        return Err(format!("registry missing: {path}"));
    }
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    parse(&text)
}

impl Registry {
    /// Coverage is the number of decisions with a registered listening session; the seal falls when it reaches the minimum.
    pub fn verdict(&self) -> Verdict {
        let coverage = self.rows.iter().filter(|l| l.has_listening()).count();
        Verdict {
            coverage,
            min_groups: self.min_groups,
            sealed: coverage < self.min_groups,
            divergences: self
                .rows
                .iter()
                .filter(|l| l.divergent())
                .map(|l| l.decision.clone())
                .collect(),
        }
    }

    pub fn without_listening(&self) -> Vec<&Row> {
        self.rows.iter().filter(|l| !l.has_listening()).collect()
    }
}

/// Report of the gate: the registry table, the coverage, the deadline and the verdict.
pub fn report(registry: &Registry) -> String {
    let verdict = registry.verdict();
    let mut out = String::new();
    out.push_str(&format!(
        "decisions: {} | registry stamp: {} | conduct: {} | window deadline: {} | minimum coverage: {} of {}\n",
        registry.rows.len(),
        registry.stamp,
        registry.conduct,
        registry.deadline,
        registry.min_groups,
        registry.rows.len()
    ));
    out.push_str(&format!(
        "{:<36} {:>9} {:<12} {:<10} {}\n",
        "decision", "holders", "heard", "record", "may issue"
    ));
    for row in &registry.rows {
        out.push_str(&format!(
            "{:<36} {:>9} {:<12} {:<10} {}\n",
            row.decision, row.holders, row.heard, row.record, row.may_issue
        ));
    }
    out.push_str(&format!(
        "\nlistening coverage: {} of {} groups heard\n",
        verdict.coverage,
        registry.rows.len()
    ));
    if verdict.divergences.is_empty() {
        out.push_str("divergences: none — no decision claims to be issued with nobody heard\n");
    } else {
        out.push_str(&format!(
            "divergences: REJECTED on {:?}\n",
            verdict.divergences
        ));
    }
    if verdict.sealed {
        out.push_str(&format!(
            "verdict: BOX SEALED — the coverage of {} stays below the minimum of {}; the decisions remain as sensitivity analysis (n = 1) and do not become public-policy guidance nor an F8 gate decision until {} or until the minimum coverage\n",
            verdict.coverage, verdict.min_groups, registry.deadline
        ));
    } else {
        out.push_str(&format!(
            "verdict: BOX OPEN for the covered decisions — the coverage of {} reached the minimum of {}\n",
            verdict.coverage, verdict.min_groups
        ));
    }
    out
}

/// The laboratory: the program registry, the negative proof and the positive proof.
pub fn run(path: &str) {
    println!("== Point 13 :: listening to those who pay for the error ==");
    match parse_file(path) {
        Ok(registry) => {
            print!("{}", report(&registry));
            let unheard = registry.without_listening();
            println!("\nwithout listening, one by one:");
            for row in unheard {
                println!("  {:<34} | {}", row.decision, row.groups);
            }
        }
        Err(error) => println!("registry not read: {error}"),
    }

    println!("\n-- negative proof: the decision that claims to be issued with nobody heard --");
    let liar = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tnenhum\t-\tsim\nh3/prisoes20\tprisoes\t65276\tnenhum\t-\tnao\nh3/prisoes_ri\tparadas\t86113\tnenhum\t-\tnao\n";
    match parse(liar) {
        Ok(r) => println!("{}", report(&r)),
        Err(e) => println!("could not read the synthetic case: {e}"),
    }

    println!("-- positive proof: two groups heard, with a signed record --");
    let honest = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tassociacao de defesa publica de TORONTO\tregistro-01.md\tsim\nh3/prisoes20\tprisoes\t65276\tcoletivo de ex-detentos\tregistro-02.md\tsim\n";
    match parse(honest) {
        Ok(r) => println!("{}", report(&r)),
        Err(e) => println!("could not read the synthetic case: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry_path() -> String {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("root")
            .to_path_buf();
        root.join(REGISTRY).to_string_lossy().to_string()
    }

    #[test]
    fn the_program_registry_has_seven_decisions_and_none_heard() {
        let registry = parse_file(&registry_path()).expect("registry of point 13");
        assert_eq!(
            registry.rows.len(),
            7,
            "the seven decisions of governance/GRUPOS.md"
        );
        assert_eq!(registry.without_listening().len(), 7);
        assert_eq!(registry.verdict().coverage, 0);
        assert_eq!(registry.verdict().min_groups, 2);
        assert_eq!(
            registry.verdict().divergences.len(),
            0,
            "the registry does not lie"
        );
        assert!(registry.verdict().sealed);
    }

    #[test]
    fn a_decision_issued_with_nobody_heard_is_rejected() {
        let text = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tnenhum\t-\tsim\n";
        let registry = parse(text).expect("synthetic case");
        let verdict = registry.verdict();
        assert_eq!(verdict.divergences, vec!["h3/prisoes".to_string()]);
        assert!(verdict.sealed, "with one group heard the box stays sealed");
    }

    #[test]
    fn the_minimum_coverage_opens_the_box() {
        let text = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tdefensoria\tregistro-01.md\tsim\nh3/prisoes20\tprisoes\t65276\tcoletivo\tregistro-02.md\tsim\n";
        let verdict = parse(text).expect("synthetic case").verdict();
        assert_eq!(verdict.coverage, 2);
        assert!(!verdict.sealed);
        assert!(verdict.divergences.is_empty());
    }

    #[test]
    fn listening_without_a_signed_record_does_not_count() {
        let text = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tconversa informal\t-\tnao\n";
        let registry = parse(text).expect("synthetic case");
        assert_eq!(
            registry.verdict().coverage,
            0,
            "a conversation without a record is not listening"
        );
    }

    #[test]
    fn the_deadline_and_the_minimum_coverage_come_from_the_registry() {
        let registry = parse_file(&registry_path()).expect("registry of point 13");
        assert_eq!(registry.deadline, "2026-12-15");
        assert_eq!(registry.stamp, "2026-10-16");
    }

    #[test]
    fn the_declared_conduct_is_protocolo_cego() {
        let registry = parse_file(&registry_path()).expect("registry of point 13");
        assert_eq!(
            registry.conduct, "protocolo cego",
            "the form chosen in declaration 2"
        );
    }
}
