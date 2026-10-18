//! Point 16, declaration 3: thesis (c′), the advantage of the architecture concentrated at the seams.
//!
//! Thesis (c) predicts only "more" and tied at the ceiling. Thesis (c′) predicts an **interaction**: the
//! architecture beats ordinary analysis on seam errors (stratum S) and ties on classical errors (stratum C).
//! `empirical/p16-ponte/DECLARACAO_3.md` froze the rule on 2026-10-18; this module enforces it over
//! `empirical/p16-ponte/registro_c_linha.tsv`, without touching `registro.tsv` or the verdict on thesis (c).
//!
//! The gate, in the order it is applied:
//! 1. the seam kinds in the record are exactly the closed list of six frozen in the declaration;
//! 2. while either stratum is below its minimum (12 cases, 8 defective and 4 clean), or the roles of the fair test
//!    of thesis (c) are not designated, the box is closed and no line may say `sustentada`;
//! 3. with both strata at the minimum and the judge's scores in, the status is **computed** from the scores —
//!    `sustentada` iff `Δ_S > Δ_C`, `|A_C − B_C| ≤ 1` case, and A raises no more false alarms than B in either
//!    stratum — and a declared status that differs from the computed one is rejected.
//!
//! Declared and not enforced here: the reliability axis (Fleiss' κ over five runs per machine arm), which the
//! declaration reads as a description and not as part of the rule.
//!
//! Run: `labs --bridge` (the section after the fair test of thesis (c)).

use crate::ponte::BridgeVerdict;

/// The record of thesis (c′).
pub const REGISTRY: &str = "empirical/p16-ponte/registro_c_linha.tsv";

/// The closed list of seam kinds, in the order of `DECLARACAO_3.md` §3.
pub const SEAM_KINDS: [&str; 6] = [
    "resolucao_da_pergunta",
    "dinamica_nao_ergodica",
    "laco_performativo",
    "selecao_dentro_da_decisao",
    "cauda_sem_media",
    "quem_paga_o_erro",
];

/// The scores of one stratum, absent until the judge has scored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scores {
    pub flagged_a: usize,
    pub flagged_b: usize,
    pub alarms_a: usize,
    pub alarms_b: usize,
}

/// One stratum of the record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stratum {
    pub name: String,
    pub cases_done: usize,
    pub scores: Option<Scores>,
}

/// The record of thesis (c′).
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub cases_per_stratum: usize,
    pub defective_per_stratum: usize,
    pub clean_per_stratum: usize,
    pub machine_repetitions: usize,
    pub seam_kinds: Vec<String>,
    pub seam: Stratum,
    pub classical: Stratum,
    pub declared_status: String,
}

/// What the gate decides about thesis (c′).
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub box_closed: bool,
    /// Why the box is closed, one reason per line.
    pub pending: Vec<String>,
    /// The status that the scores support, when the box is open.
    pub computed_status: Option<&'static str>,
    pub delta_seam: Option<f64>,
    pub delta_classical: Option<f64>,
    pub divergences: Vec<String>,
}

fn parse_count(field: &str) -> Option<usize> {
    let f = field.trim();
    if f == "-" {
        return None;
    }
    f.parse().ok()
}

/// Reads the record: comments `# key<TAB>value` and the stratum table.
pub fn parse(text: &str) -> Result<Record, String> {
    let mut cases = None;
    let mut defective = None;
    let mut clean = None;
    let mut repetitions = None;
    let mut kinds: Vec<String> = Vec::new();
    let mut seam = None;
    let mut classical = None;
    let mut status = None;
    for line in text.lines() {
        let l = line.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        if let Some(rest) = l.strip_prefix('#') {
            let fields: Vec<&str> = rest.trim().split('\t').collect();
            if fields.len() < 2 {
                continue;
            }
            let number = |v: &str, key: &str| -> Result<usize, String> {
                v.trim()
                    .parse()
                    .map_err(|_| format!("{key} is not a number"))
            };
            match fields[0] {
                "casos_por_estrato" => cases = Some(number(fields[1], fields[0])?),
                "defeituosos_por_estrato" => defective = Some(number(fields[1], fields[0])?),
                "limpos_por_estrato" => clean = Some(number(fields[1], fields[0])?),
                "repeticoes_por_braco_de_maquina" => {
                    repetitions = Some(number(fields[1], fields[0])?)
                }
                "tipos_de_costura" => {
                    kinds = fields[1].split(',').map(|k| k.trim().to_string()).collect()
                }
                _ => {}
            }
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields[0] == "estrato" {
            continue;
        }
        if fields[0] == "status_c_linha" {
            status = fields.last().map(|s| s.trim().to_string());
            continue;
        }
        if fields.len() < 6 {
            return Err(format!("stratum row with {} fields: {l}", fields.len()));
        }
        let done = fields[1]
            .split_whitespace()
            .next()
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| format!("case count unreadable: {}", fields[1]))?;
        let marks: Vec<Option<usize>> = fields[2..6].iter().map(|f| parse_count(f)).collect();
        let scores = match (marks[0], marks[1], marks[2], marks[3]) {
            (Some(flagged_a), Some(flagged_b), Some(alarms_a), Some(alarms_b)) => Some(Scores {
                flagged_a,
                flagged_b,
                alarms_a,
                alarms_b,
            }),
            (None, None, None, None) => None,
            _ => {
                return Err(format!(
                    "stratum {} has part of the scores: all four or none",
                    fields[0]
                ));
            }
        };
        let stratum = Stratum {
            name: fields[0].to_string(),
            cases_done: done,
            scores,
        };
        match fields[0] {
            "S" => seam = Some(stratum),
            "C" => classical = Some(stratum),
            other => return Err(format!("unknown stratum {other}: only S and C exist")),
        }
    }
    Ok(Record {
        cases_per_stratum: cases.ok_or("record without casos_por_estrato")?,
        defective_per_stratum: defective.ok_or("record without defeituosos_por_estrato")?,
        clean_per_stratum: clean.ok_or("record without limpos_por_estrato")?,
        machine_repetitions: repetitions.ok_or("record without repeticoes_por_braco_de_maquina")?,
        seam_kinds: kinds,
        seam: seam.ok_or("record without stratum S")?,
        classical: classical.ok_or("record without stratum C")?,
        declared_status: status.ok_or("record without status_c_linha")?,
    })
}

/// The rule of `DECLARACAO_3.md` §4, over the scores of two complete strata.
pub fn rule(seam: &Scores, classical: &Scores, defective: usize) -> (&'static str, f64, f64) {
    let d = defective.max(1) as f64;
    let delta_s = (seam.flagged_a as f64 - seam.flagged_b as f64) / d;
    let delta_c = (classical.flagged_a as f64 - classical.flagged_b as f64) / d;
    let parity_in_c = classical.flagged_a.abs_diff(classical.flagged_b) <= 1;
    let no_alarm_price = seam.alarms_a <= seam.alarms_b && classical.alarms_a <= classical.alarms_b;
    let status = if delta_s > delta_c && parity_in_c && no_alarm_price {
        "sustentada"
    } else {
        "nao sustentada"
    };
    (status, delta_s, delta_c)
}

impl Record {
    /// The gate. `fair_test` is the verdict on the roles of thesis (c): the same selector, arms, and judge.
    pub fn verdict(&self, fair_test: Option<&BridgeVerdict>) -> Verdict {
        let mut pending = Vec::new();
        let mut divergences = Vec::new();
        let frozen: Vec<String> = SEAM_KINDS.iter().map(|k| k.to_string()).collect();
        if self.seam_kinds != frozen {
            divergences.push(format!(
                "seam kinds {:?} differ from the closed list frozen in DECLARACAO_3.md",
                self.seam_kinds
            ));
        }
        if self.defective_per_stratum + self.clean_per_stratum != self.cases_per_stratum {
            divergences.push(format!(
                "{} defective + {} clean is not {} cases per stratum",
                self.defective_per_stratum, self.clean_per_stratum, self.cases_per_stratum
            ));
        }
        for stratum in [&self.seam, &self.classical] {
            if stratum.cases_done < self.cases_per_stratum {
                pending.push(format!(
                    "stratum {} at {} of {} cases",
                    stratum.name, stratum.cases_done, self.cases_per_stratum
                ));
            }
            if let Some(s) = stratum.scores
                && (s.flagged_a.max(s.flagged_b) > self.defective_per_stratum
                    || s.alarms_a.max(s.alarms_b) > self.clean_per_stratum)
            {
                divergences.push(format!(
                    "stratum {} scores exceed its {} defective and {} clean cases",
                    stratum.name, self.defective_per_stratum, self.clean_per_stratum
                ));
            }
        }
        match fair_test {
            Some(v) if v.box_closed => pending.push(format!(
                "the fair test of thesis (c) has {} role(s) pending",
                v.pending_roles.len()
            )),
            Some(_) => {}
            None => pending.push("the fair test record of thesis (c) was not read".to_string()),
        }
        let both_scored = self.seam.scores.zip(self.classical.scores);
        if pending.is_empty() && both_scored.is_none() {
            pending.push("the judge has not scored both strata".to_string());
        }
        let box_closed = !pending.is_empty();
        let mut computed_status = None;
        let mut delta_seam = None;
        let mut delta_classical = None;
        if box_closed {
            if self.declared_status != "nao sustentada" {
                divergences.push(format!(
                    "declared status \"{}\" with the box closed: (c′) only changes state from the judge's scores",
                    self.declared_status
                ));
            }
        } else if let Some((s, c)) = both_scored {
            let (status, ds, dc) = rule(&s, &c, self.defective_per_stratum);
            computed_status = Some(status);
            delta_seam = Some(ds);
            delta_classical = Some(dc);
            if self.declared_status != status {
                divergences.push(format!(
                    "declared status \"{}\" but the scores give \"{status}\"",
                    self.declared_status
                ));
            }
        }
        Verdict {
            box_closed,
            pending,
            computed_status,
            delta_seam,
            delta_classical,
            divergences,
        }
    }
}

/// Report of thesis (c′).
pub fn report(record: Option<&Record>, fair_test: Option<&BridgeVerdict>) -> String {
    let mut s = String::new();
    s.push_str("-- thesis (c′): the advantage at the seams (DECLARACAO_3.md) --\n");
    let Some(r) = record else {
        s.push_str("the record of thesis (c′) was not read\n");
        return s;
    };
    s.push_str(&format!(
        "per stratum: {} cases ({} defective, {} clean) | machine runs per arm: {} (reliability, descriptive)\n",
        r.cases_per_stratum, r.defective_per_stratum, r.clean_per_stratum, r.machine_repetitions
    ));
    s.push_str(&format!("seam kinds: {}\n", r.seam_kinds.join(", ")));
    for stratum in [&r.seam, &r.classical] {
        let scores = match stratum.scores {
            Some(x) => format!(
                "flagged A {} B {} | alarms A {} B {}",
                x.flagged_a, x.flagged_b, x.alarms_a, x.alarms_b
            ),
            None => "not scored".to_string(),
        };
        s.push_str(&format!(
            "  stratum {}  {} of {} cases  {}\n",
            stratum.name, stratum.cases_done, r.cases_per_stratum, scores
        ));
    }
    let v = r.verdict(fair_test);
    if v.divergences.is_empty() {
        s.push_str("divergences: none\n");
    } else {
        s.push_str(&format!("divergences: REJECTED on {:?}\n", v.divergences));
    }
    if v.box_closed {
        s.push_str(&format!(
            "verdict: (c′) NOT SUPPORTED and box closed — {}\n",
            v.pending.join("; ")
        ));
    } else if let (Some(status), Some(ds), Some(dc)) =
        (v.computed_status, v.delta_seam, v.delta_classical)
    {
        s.push_str(&format!(
            "verdict: the scores give \"{status}\" (Δ_S = {ds:+.3}, Δ_C = {dc:+.3})\n"
        ));
    }
    s
}

/// The section printed by `labs --bridge`, with its two proofs.
pub fn run(registry_path: &str, fair_test: Option<&BridgeVerdict>) {
    let record = std::fs::read_to_string(registry_path)
        .ok()
        .and_then(|t| parse(&t).ok());
    print!("\n{}", report(record.as_ref(), fair_test));
    let open = BridgeVerdict {
        designated_roles: Vec::new(),
        pending_roles: Vec::new(),
        cases_missing: 0,
        declared_status: "em teste".to_string(),
        divergences: Vec::new(),
        box_closed: false,
    };
    println!("-- negative proof: an advantage of A equal in both strata is (c), not (c′) --");
    let general = synthetic(8, 6, 8, 6, "sustentada");
    let v = general.verdict(Some(&open));
    println!(
        "  computed {:?} | divergences {:?}",
        v.computed_status, v.divergences
    );
    println!("-- positive proof: advantage at the seams, parity in the classical stratum --");
    let located = synthetic(8, 5, 8, 8, "sustentada");
    let v = located.verdict(Some(&open));
    println!(
        "  computed {:?} | divergences {:?}",
        v.computed_status, v.divergences
    );
}

/// A synthetic record with both strata complete, for the proofs. Never written to disk.
pub fn synthetic(
    seam_a: usize,
    seam_b: usize,
    classical_a: usize,
    classical_b: usize,
    status: &str,
) -> Record {
    Record {
        cases_per_stratum: 12,
        defective_per_stratum: 8,
        clean_per_stratum: 4,
        machine_repetitions: 5,
        seam_kinds: SEAM_KINDS.iter().map(|k| k.to_string()).collect(),
        seam: Stratum {
            name: "S".to_string(),
            cases_done: 12,
            scores: Some(Scores {
                flagged_a: seam_a,
                flagged_b: seam_b,
                alarms_a: 0,
                alarms_b: 0,
            }),
        },
        classical: Stratum {
            name: "C".to_string(),
            cases_done: 12,
            scores: Some(Scores {
                flagged_a: classical_a,
                flagged_b: classical_b,
                alarms_a: 0,
                alarms_b: 0,
            }),
        },
        declared_status: status.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ponte;
    use std::path::Path;

    fn read_file(relative: &str) -> String {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("root")
            .to_path_buf();
        std::fs::read_to_string(root.join(relative)).expect("record file")
    }

    fn open_fair_test() -> BridgeVerdict {
        BridgeVerdict {
            designated_roles: vec!["juiz".to_string()],
            pending_roles: Vec::new(),
            cases_missing: 0,
            declared_status: "em teste".to_string(),
            divergences: Vec::new(),
            box_closed: false,
        }
    }

    #[test]
    fn the_record_in_the_tree_keeps_the_box_closed_and_does_not_lie() {
        let record = parse(&read_file(REGISTRY)).expect("record of (c′)");
        let fair = ponte::parse_bridge(&read_file(ponte::REGISTRY))
            .expect("record of (c)")
            .verdict();
        let v = record.verdict(Some(&fair));
        assert!(v.box_closed);
        assert!(v.divergences.is_empty(), "{:?}", v.divergences);
        assert_eq!(record.declared_status, "nao sustentada");
        assert_eq!(
            v.pending.len(),
            3,
            "two strata and the fair test: {:?}",
            v.pending
        );
        assert_eq!(record.seam_kinds.len(), 6);
    }

    #[test]
    fn a_supported_c_prime_with_the_box_closed_is_rejected() {
        let text = read_file(REGISTRY).replace("\tnao sustentada\n", "\tsustentada\n");
        let record = parse(&text).expect("tampered record");
        let v = record.verdict(Some(&open_fair_test()));
        assert!(v.box_closed);
        assert_eq!(v.divergences.len(), 1, "{:?}", v.divergences);
    }

    #[test]
    fn a_general_advantage_is_not_c_prime() {
        // A beats B by two cases in both strata: that is thesis (c), and (c′) fails on parity in C.
        let v = synthetic(8, 6, 8, 6, "sustentada").verdict(Some(&open_fair_test()));
        assert_eq!(v.computed_status, Some("nao sustentada"));
        assert_eq!(v.divergences.len(), 1, "{:?}", v.divergences);
    }

    #[test]
    fn the_located_advantage_is_c_prime() {
        let v = synthetic(8, 5, 8, 8, "sustentada").verdict(Some(&open_fair_test()));
        assert_eq!(v.computed_status, Some("sustentada"));
        assert!(v.divergences.is_empty(), "{:?}", v.divergences);
        assert!((v.delta_seam.unwrap() - 0.375).abs() < 1e-12);
        assert_eq!(v.delta_classical, Some(0.0));
    }

    #[test]
    fn the_tie_at_the_ceiling_in_s_is_not_c_prime() {
        let v = synthetic(8, 8, 8, 8, "nao sustentada").verdict(Some(&open_fair_test()));
        assert_eq!(v.computed_status, Some("nao sustentada"));
        assert!(v.divergences.is_empty());
    }

    #[test]
    fn an_advantage_paid_in_false_alarms_is_not_c_prime() {
        let mut record = synthetic(8, 5, 8, 8, "nao sustentada");
        if let Some(s) = record.seam.scores.as_mut() {
            s.alarms_a = 2;
            s.alarms_b = 1;
        }
        let v = record.verdict(Some(&open_fair_test()));
        assert_eq!(v.computed_status, Some("nao sustentada"));
        assert!(v.divergences.is_empty());
    }

    #[test]
    fn complete_strata_without_the_roles_keep_the_box_closed() {
        let closed = ponte::parse_bridge(&read_file(ponte::REGISTRY))
            .expect("record of (c)")
            .verdict();
        let v = synthetic(8, 5, 8, 8, "nao sustentada").verdict(Some(&closed));
        assert!(v.box_closed, "the arms and the judge are the roles of (c)");
        assert_eq!(v.computed_status, None);
        let v = synthetic(8, 5, 8, 8, "nao sustentada").verdict(None);
        assert!(v.box_closed);
    }

    #[test]
    fn a_changed_list_of_seam_kinds_is_rejected() {
        let text = read_file(REGISTRY).replace("cauda_sem_media,", "");
        let v = parse(&text)
            .expect("record")
            .verdict(Some(&open_fair_test()));
        assert!(
            v.divergences.iter().any(|d| d.contains("closed list")),
            "{:?}",
            v.divergences
        );
    }

    #[test]
    fn partial_scores_and_unknown_strata_are_not_read() {
        let partial = read_file(REGISTRY).replace("S\t0 de 12\t-\t-", "S\t0 de 12\t3\t-");
        assert!(parse(&partial).is_err());
        let unknown = read_file(REGISTRY).replace("\nC\t0 de 12", "\nX\t0 de 12");
        assert!(parse(&unknown).is_err());
        let overflow = synthetic(9, 5, 8, 8, "nao sustentada").verdict(Some(&open_fair_test()));
        assert!(overflow.divergences.iter().any(|d| d.contains("exceed")));
    }
}
