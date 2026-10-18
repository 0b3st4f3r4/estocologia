//! Point 16: the bridge thesis at scale, with the fair test still to be done.
//!
//! Pilot P01 (`empirical/p01-tese/`) gave a **tie at the ceiling**: both arms flagged all 8 planted defects and
//! raised no false alarm on the 4 clean cases, and the pilot declaration fixed that a tie at the ceiling counts as
//! **thesis not supported**. The pilot README says what a fair test is missing: harder cases chosen
//! by someone **from outside the program**, **human** analysts and a **human** judge.
//!
//! This module does two things. First, it **re-derives** the pilot's verdict from the blind judge's notes
//! (`empirical/p01-tese/juiz/notas.yaml`) instead of repeating the README number. Second, it **locks the thesis**: while the
//! fair test has no selector, analysts and judge designated, no line of the registry may say the thesis
//! is supported — the gate rejects.
//!
//! Run: ~labs --bridge~

/// The registry of the fair test front.
pub const REGISTRY: &str = "empirical/p16-ponte/registro.tsv";
/// The notes of the pilot's blind judge.
pub const JUDGE_NOTES: &str = "empirical/p01-tese/juiz/notas.yaml";

/// A judge note: one case, one arm, the meaning of the note.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub case: String,
    /// X or Y — the blind label; the map is in `juiz/mapa_XY.json`.
    pub arm: char,
    pub kind: String,
    /// The judge's mark: in a defective case, 1 = flagged the defect; in a clean case, 1 = false alarm.
    pub mark: i64,
}

/// The pilot score, re-derived from the notes.
#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    pub cases: usize,
    pub defective: usize,
    pub clean: usize,
    pub flagged_x: usize,
    pub flagged_y: usize,
    pub false_alarms_x: usize,
    pub false_alarms_y: usize,
    /// Cases where the two arms disagreed on the meaning of the mark.
    pub discordant_pairs: usize,
}

/// Reads the judge notes from the pilot file: `caso_XX:` and the two lines `X:` and `Y:` with flow fields.
pub fn parse_notes(text: &str) -> Vec<Note> {
    let mut notes = Vec::new();
    let mut case = String::new();
    for line in text.lines() {
        let l = line.trim_end();
        if l.trim().is_empty() || l.trim_start().starts_with('#') {
            continue;
        }
        if !l.starts_with(' ') && l.ends_with(':') {
            case = l.trim_end_matches(':').trim().to_string();
            continue;
        }
        let t = l.trim_start();
        let arm = if t.starts_with("X:") {
            Some('X')
        } else if t.starts_with("Y:") {
            Some('Y')
        } else {
            None
        };
        let Some(arm) = arm else { continue };
        let mark_field = |key: &str| -> i64 {
            let target = format!("{key}:");
            match t.find(&target) {
                Some(i) => {
                    let rest = &t[i + target.len()..];
                    rest.trim_start()
                        .split([',', '}'])
                        .next()
                        .unwrap_or("")
                        .trim()
                        .parse::<i64>()
                        .unwrap_or(0)
                }
                None => 0,
            }
        };
        let field = |key: &str| -> String {
            let target = format!("{key}:");
            match t.find(&target) {
                Some(i) => {
                    let rest = &t[i + target.len()..];
                    rest.trim_start()
                        .split([',', '}'])
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string()
                }
                None => String::new(),
            }
        };
        let kind = field("tipo");
        let _ = kind.clone();
        notes.push(Note {
            case: case.clone(),
            arm,
            kind,
            mark: mark_field("nota"),
        });
    }
    notes
}

/// Scores the pilot: how many defects each arm flagged and how many false alarms it raised, plus the discordant pairs.
pub fn score(notes: &[Note]) -> Score {
    let mut by_case: Vec<(String, i64, i64)> = Vec::new();
    for n in notes {
        match by_case.iter_mut().find(|(c, _, _)| *c == n.case) {
            Some(entry) => {
                if n.arm == 'X' {
                    entry.1 = n.mark;
                } else {
                    entry.2 = n.mark;
                }
            }
            None => {
                let (x, y) = if n.arm == 'X' {
                    (n.mark, 0)
                } else {
                    (0, n.mark)
                };
                by_case.push((n.case.clone(), x, y));
            }
        }
    }
    let kind_of_case = |case: &str| -> String {
        notes
            .iter()
            .find(|n| n.case == case)
            .map(|n| n.kind.clone())
            .unwrap_or_default()
    };
    let mut flagged_x = 0;
    let mut flagged_y = 0;
    let mut false_alarms_x = 0;
    let mut false_alarms_y = 0;
    let mut defective = 0;
    let mut clean = 0;
    let mut discordant = 0;
    for (case, x, y) in &by_case {
        if kind_of_case(case) == "defeito" {
            defective += 1;
            if *x == 1 {
                flagged_x += 1;
            }
            if *y == 1 {
                flagged_y += 1;
            }
        } else {
            clean += 1;
            if *x == 1 {
                false_alarms_x += 1;
            }
            if *y == 1 {
                false_alarms_y += 1;
            }
        }
        if x != y {
            discordant += 1;
        }
    }
    Score {
        cases: by_case.len(),
        defective,
        clean,
        flagged_x,
        flagged_y,
        false_alarms_x,
        false_alarms_y,
        discordant_pairs: discordant,
    }
}

/// The pilot verdict by the rule declared beforehand: a tie at the ceiling counts as thesis not supported.
pub fn pilot_verdict(p: &Score) -> String {
    let ceiling = p.flagged_x == p.defective
        && p.flagged_y == p.defective
        && p.false_alarms_x == 0
        && p.false_alarms_y == 0;
    if ceiling && p.discordant_pairs == 0 {
        format!(
            "tie at the ceiling: both arms flagged {} of {} defects and raised {} false alarms on {} clean cases; by the declared rule, the thesis is NOT supported",
            p.flagged_x, p.defective, p.false_alarms_x, p.clean
        )
    } else if p.discordant_pairs > 0 {
        format!(
            "there are {} discordant pair(s): sensitivity {} and {} of {}, alarms {} and {} — the pilot discriminates and the verdict comes from the judge notes",
            p.discordant_pairs,
            p.flagged_x,
            p.flagged_y,
            p.defective,
            p.false_alarms_x,
            p.false_alarms_y
        )
    } else {
        "the two lineages tie below the ceiling: power is missing, and the verdict does not hold"
            .to_string()
    }
}

/// A requirement of the fair test: the role, what is required and who has already been designated.
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement {
    pub role: String,
    pub requirement: String,
    pub who: String,
    pub state: String,
}

/// The registry of the front: deadline, minimum cases and the roles.
#[derive(Debug, Clone, PartialEq)]
pub struct Bridge {
    pub stamp: String,
    pub deadline: String,
    pub min_cases: usize,
    /// The verdict declared by the owner if the window lapses without the roles: not supported, definitively.
    pub verdict_if_window_lapses: String,
    pub roles: Vec<Requirement>,
}

/// What the gate decides about the thesis.
#[derive(Debug, Clone, PartialEq)]
pub struct BridgeVerdict {
    pub designated_roles: Vec<String>,
    pub pending_roles: Vec<String>,
    pub cases_missing: usize,
    pub declared_status: String,
    /// No line may say the thesis is supported before the fair test.
    pub divergences: Vec<String>,
    pub box_closed: bool,
}

/// Reads the registry of the front: comments `# key<TAB>value` and the tab-separated table.
pub fn parse_bridge(text: &str) -> Result<Bridge, String> {
    let mut stamp = String::new();
    let mut deadline = String::new();
    let mut min_cases = 0usize;
    let mut verdict_if_window_lapses = String::new();
    let mut roles = Vec::new();
    for line in text.lines() {
        let l = line.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        if let Some(rest) = l.strip_prefix('#') {
            let fields: Vec<&str> = rest.trim().split('\t').collect();
            if fields.len() >= 2 {
                match fields[0] {
                    "registros" => stamp = fields[1].trim().to_string(),
                    "prazo" => deadline = fields[1].trim().to_string(),
                    "casos_minimos" => {
                        min_cases = fields[1]
                            .trim()
                            .parse()
                            .map_err(|_| "casos_minimos is not a number")?
                    }
                    "veredito_se_o_prazo_vencer" => {
                        verdict_if_window_lapses = fields[1].trim().to_string()
                    }
                    _ => {}
                }
            }
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() < 4 || fields[0] == "papel" {
            continue;
        }
        roles.push(Requirement {
            role: fields[0].to_string(),
            requirement: fields[1].to_string(),
            who: fields[2].to_string(),
            state: fields[3].to_string(),
        });
    }
    if deadline.is_empty() || roles.is_empty() || verdict_if_window_lapses.is_empty() {
        return Err(
            "bridge registry without deadline, without roles or without the window verdict"
                .to_string(),
        );
    }
    Ok(Bridge {
        stamp,
        deadline,
        min_cases,
        verdict_if_window_lapses,
        roles,
    })
}

impl Bridge {
    /// The gate: the designated roles, the pending ones, and the divergences (whoever claims what it cannot).
    pub fn verdict(&self) -> BridgeVerdict {
        let mut designated = Vec::new();
        let mut pending = Vec::new();
        let mut cases_missing = 0usize;
        let mut divergences = Vec::new();
        let mut status = "nao sustentada".to_string();
        let mut everything_designated = true;
        for p in &self.roles {
            if p.role == "status_da_tese" {
                status = p.state.clone();
                continue;
            }
            if p.role == "casos" {
                let done: usize = p
                    .state
                    .split_whitespace()
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);
                cases_missing = self.min_cases.saturating_sub(done);
                if cases_missing > 0 {
                    everything_designated = false;
                    pending.push(format!("casos ({} de {})", done, self.min_cases));
                } else {
                    designated.push(format!("casos ({} de {})", done, self.min_cases));
                }
                continue;
            }
            if p.state == "designado" {
                designated.push(p.role.clone());
            } else {
                everything_designated = false;
                pending.push(p.role.clone());
            }
        }
        if status != "nao sustentada" && status != "em teste" {
            divergences.push(format!("declared status \"{status}\" with the fair test still to be done: the thesis only changes state after the judge's result"));
        }
        if !everything_designated && status == "em teste" {
            divergences.push(
                "status em teste without the designated roles and the minimum cases".to_string(),
            );
        }
        BridgeVerdict {
            designated_roles: designated,
            pending_roles: pending,
            cases_missing,
            declared_status: status,
            divergences,
            box_closed: !everything_designated,
        }
    }
}

/// An eligible case from the external list: the identifier and the classification of the outside catalogue.
#[derive(Debug, Clone, PartialEq)]
pub struct EligibleCase {
    pub id: String,
    /// "defeito" or "limpo", by the classification of the external catalogue.
    pub group: String,
}

/// The mechanical choice of cases: the permutation of the list by the declared seed, in the declared proportion.
/// If the external list does not support the requested proportion, the choice is **rejected** instead of completed
/// with whatever there is — that is the gate of the choice rule.
pub fn choose_cases(
    list: &[EligibleCase],
    defective: usize,
    clean: usize,
    seed: u64,
) -> Result<Vec<String>, String> {
    let mut rng = stochology_core::Rng::new(seed);
    let mut chosen = Vec::new();
    for (group, how_many) in [("defeito", defective), ("limpo", clean)] {
        let mut candidates: Vec<&EligibleCase> = list.iter().filter(|c| c.group == group).collect();
        if candidates.len() < how_many {
            return Err(format!(
                "the external list does not support the choice: {how_many} cases of group {group} requested, {} eligible",
                candidates.len()
            ));
        }
        for i in (1..candidates.len()).rev() {
            let j = rng.uniform(i as u64 + 1) as usize;
            candidates.swap(i, j);
        }
        for c in candidates.into_iter().take(how_many) {
            chosen.push(c.id.clone());
        }
    }
    Ok(chosen)
}

/// Report: the pilot re-derived and the state of the fair test.
pub fn report(score: Option<&Score>, bridge: Option<&Bridge>) -> String {
    let mut s = String::new();
    s.push_str("-- pilot P01, re-derived from the blind judge notes --\n");
    match score {
        Some(p) => {
            s.push_str(&format!(
                "cases {} | defective {} | clean {} | discordant pairs {}\n",
                p.cases, p.defective, p.clean, p.discordant_pairs
            ));
            s.push_str(&format!(
                "flagged the defect: arm X {} of {} | arm Y {} of {}\n",
                p.flagged_x, p.defective, p.flagged_y, p.defective
            ));
            s.push_str(&format!(
                "false alarm on the clean ones: arm X {} of {} | arm Y {} of {}\n",
                p.false_alarms_x, p.clean, p.false_alarms_y, p.clean
            ));
            s.push_str(&format!("verdict: {}\n", pilot_verdict(p)));
        }
        None => s.push_str("the judge notes were not read\n"),
    }
    s.push_str("\n-- the fair test --\n");
    match bridge {
        Some(p) => {
            let v = p.verdict();
            s.push_str(&format!(
                "window deadline: {} | minimum cases: {}\n",
                p.deadline, p.min_cases
            ));
            s.push_str(&format!(
                "if the window lapses without the roles: {}\n",
                p.verdict_if_window_lapses
            ));
            for role in &p.roles {
                s.push_str(&format!(
                    "  {:<22} {:<10} {}\n",
                    role.role, role.state, role.requirement
                ));
            }
            s.push_str(&format!(
                "designated: {:?} | pending: {:?} | cases missing: {}\n",
                v.designated_roles, v.pending_roles, v.cases_missing
            ));
            if v.divergences.is_empty() {
                s.push_str(
                    "divergences: none — no line claims what the fair test does not yet support\n",
                );
            } else {
                s.push_str(&format!("divergences: REJECTED on {:?}\n", v.divergences));
            }
            if v.box_closed {
                s.push_str(&format!(
                    "verdict: THESIS NOT SUPPORTED and box closed — with {} pending, the thesis does not become \"supported\" by any path other than the fair test, and only by {}\n",
                    v.pending_roles.len(),
                    p.deadline
                ));
            } else {
                s.push_str("verdict: THE FAIR TEST MAY RUN — the roles are designated and the cases at the minimum; the status only changes with the judge's result\n");
            }
        }
        None => s.push_str("the registry of the front was not read\n"),
    }
    s
}

/// The laboratory: the pilot re-derived, the state of the fair test and the two proofs.
pub fn run(registry_path: &str, notes_path: &str) {
    println!("== Point 16 :: the bridge thesis at scale ==");
    let score = std::fs::read_to_string(notes_path)
        .ok()
        .map(|t| score(&parse_notes(&t)));
    let bridge = std::fs::read_to_string(registry_path)
        .ok()
        .and_then(|t| parse_bridge(&t).ok());
    print!("{}", report(score.as_ref(), bridge.as_ref()));

    println!(
        "\n-- negative proof: a registry that says the thesis is supported without the fair test --"
    );
    let liar = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\nescolhedor_dos_casos\tde fora\t-\tnao designado\nanalistas_braco_A\tdois humanos\t-\tnao designado\nanalistas_braco_B\tdois humanos\t-\tnao designado\njuiz\tum humano de fora\t-\tnao designado\ncasos\tmais dificeis\t-\t0 de 12\nstatus_da_tese\tnao sustentada ate o teste justo\t-\tsustentada\n";
    match parse_bridge(liar) {
        Ok(p) => {
            let v = p.verdict();
            println!("  divergences: {:?}", v.divergences);
            println!("  box closed: {}", v.box_closed);
        }
        Err(e) => println!("  synthetic case not read: {e}"),
    }

    println!("-- positive proof: designated roles and cases at the minimum --");
    let honest = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\nescolhedor_dos_casos\tde fora\tfulano (de fora)\tdesignado\nanalistas_braco_A\tdois humanos\tfulano e beltrano\tdesignado\nanalistas_braco_B\tdois humanos, cegos\tsicrano e beltrano\tdesignado\njuiz\tum humano de fora\tjuiz de fora\tdesignado\ncasos\tmais dificeis\tfulano\t12 de 12\nstatus_da_tese\tnao sustentada ate o resultado\t-\tem teste\n";
    match parse_bridge(honest) {
        Ok(p) => {
            let v = p.verdict();
            println!(
                "  divergences: {:?} | box closed: {}",
                v.divergences, v.box_closed
            );
        }
        Err(e) => println!("  synthetic case not read: {e}"),
    }

    let fair_test = bridge.as_ref().map(Bridge::verdict);
    crate::costura::run(crate::costura::REGISTRY, fair_test.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn read_file(relative: &str) -> String {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("root")
            .to_path_buf();
        std::fs::read_to_string(root.join(relative)).expect("registry file")
    }

    #[test]
    fn the_pilot_re_derives_the_tie_at_the_ceiling() {
        let p = score(&parse_notes(&read_file(JUDGE_NOTES)));
        assert_eq!(p.cases, 12, "the twelve cases of the pilot");
        assert_eq!(p.defective, 8);
        assert_eq!(p.clean, 4);
        assert_eq!(
            (p.flagged_x, p.flagged_y),
            (8, 8),
            "both arms flagged every defect"
        );
        assert_eq!(
            (p.false_alarms_x, p.false_alarms_y),
            (0, 0),
            "no false alarm"
        );
        assert_eq!(
            p.discordant_pairs, 0,
            "with no discordant pair, McNemar has nothing to test"
        );
        let v = pilot_verdict(&p);
        assert!(v.starts_with("tie at the ceiling"), "verdict: {v}");
        assert!(v.contains("NOT supported"));
    }

    #[test]
    fn the_fair_test_registry_declares_what_is_missing() {
        let bridge = parse_bridge(&read_file(REGISTRY)).expect("bridge registry");
        let v = bridge.verdict();
        assert!(
            v.box_closed,
            "without selector, analysts and judge, the box stays closed"
        );
        assert_eq!(v.cases_missing, bridge.min_cases);
        assert_eq!(v.declared_status, "nao sustentada");
        assert!(
            v.divergences.is_empty(),
            "the registry does not lie: {:?}",
            v.divergences
        );
        assert_eq!(v.pending_roles.len(), 5);
    }

    #[test]
    fn a_supported_thesis_without_a_fair_test_is_rejected() {
        let liar = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\njuiz\tum humano de fora\t-\tnao designado\ncasos\tmais dificeis\t-\t0 de 12\nstatus_da_tese\tnao sustentada ate o teste justo\t-\tsustentada\n";
        let v = parse_bridge(liar).expect("synthetic case").verdict();
        assert_eq!(v.divergences.len(), 1, "{:?}", v.divergences);
        assert!(v.box_closed);
    }

    #[test]
    fn with_the_roles_designated_the_test_may_run() {
        let honest = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\nescolhedor_dos_casos\tde fora\tfulano\tdesignado\nanalistas_braco_A\tdois\tfulano e beltrano\tdesignado\nanalistas_braco_B\tdois\tsicrano\tdesignado\njuiz\tum de fora\tjuiz\tdesignado\ncasos\tmais dificeis\tfulano\t12 de 12\nstatus_da_tese\tnao sustentada ate o resultado\t-\tem teste\n";
        let v = parse_bridge(honest).expect("synthetic case").verdict();
        assert!(!v.box_closed);
        assert!(v.divergences.is_empty(), "{:?}", v.divergences);
        assert_eq!(v.declared_status, "em teste");
    }

    #[test]
    fn the_status_em_teste_without_the_roles_is_rejected() {
        let hasty = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\njuiz\tum de fora\t-\tnao designado\ncasos\tmais dificeis\t-\t0 de 12\nstatus_da_tese\tnao sustentada ate o resultado\t-\tem teste\n";
        let v = parse_bridge(hasty).expect("synthetic case").verdict();
        assert_eq!(v.divergences.len(), 1, "{:?}", v.divergences);
    }

    // ---------------------------------------------------------------- the mechanical choice rule

    fn test_list(defective: usize, clean: usize) -> Vec<EligibleCase> {
        let mut list = Vec::new();
        for i in 0..defective {
            list.push(EligibleCase {
                id: format!("defeito_{i:02}"),
                group: "defeito".to_string(),
            });
        }
        for i in 0..clean {
            list.push(EligibleCase {
                id: format!("limpo_{i:02}"),
                group: "limpo".to_string(),
            });
        }
        list
    }

    #[test]
    fn the_choice_is_mechanical_deterministic_and_in_proportion() {
        let list = test_list(40, 20);
        let a = choose_cases(&list, 8, 4, 20270114).expect("choice");
        let b = choose_cases(&list, 8, 4, 20270114).expect("choice");
        assert_eq!(a, b, "the same list and the same seed give the same choice");
        assert_eq!(a.len(), 12);
        let defective = a.iter().filter(|id| id.starts_with("defeito")).count();
        let clean = a.iter().filter(|id| id.starts_with("limpo")).count();
        assert_eq!((defective, clean), (8, 4), "the declared proportion");
        let mut distinct = a.clone();
        distinct.sort();
        distinct.dedup();
        assert_eq!(distinct.len(), 12, "without repetition");
        let c = choose_cases(&list, 8, 4, 20270115).expect("choice");
        assert_ne!(a, c, "another seed gives another choice");
        let mut ordered = list.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
        ordered.sort();
        let mut chosen = a.clone();
        chosen.sort();
        assert_eq!(
            chosen.len(),
            12,
            "the choice comes from the list, and not from outside it"
        );
        assert!(chosen.iter().all(|id| ordered.contains(id)));
    }

    #[test]
    fn the_list_that_does_not_support_the_proportion_is_rejected() {
        let short = test_list(5, 20);
        let error = choose_cases(&short, 8, 4, 20270114).expect_err("should reject");
        assert!(
            error.contains("does not support the choice"),
            "error: {error}"
        );
        let without_clean = test_list(40, 2);
        assert!(choose_cases(&without_clean, 8, 4, 20270114).is_err());
        let empty: Vec<EligibleCase> = Vec::new();
        assert!(choose_cases(&empty, 8, 4, 20270114).is_err());
    }

    #[test]
    fn the_registry_declares_the_verdict_if_the_window_lapses() {
        let bridge = parse_bridge(&read_file(REGISTRY)).expect("bridge registry");
        assert!(
            bridge
                .verdict_if_window_lapses
                .contains("nao sustentada em definitivo"),
            "the registry carries the owner's decision: {}",
            bridge.verdict_if_window_lapses
        );
    }
}
