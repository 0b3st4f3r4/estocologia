//! The synthetic rehearsals of the three human protocols: the null model of each one.
//!
//! Items 3, 5 and 9 of docs/DECLARED_PROPOSALS.md depend on people, and an agent session does not take anyone's
//! place. What it can do is answer another question: **how much does a fluent and uncalibrated respondent
//! score on the same instrument?** Without that floor, a real specialist has nothing to be compared against, and the
//! protocol may be measuring fluency instead of knowledge.
//!
//! Three rehearsals, all with synthetic input and with the label in the output itself:
//!
//! - shelf: four declared respondents, scored on the real pre-test items, with Brier, skill and
//!   ECE by the same evaluator the panel uses;
//! - listening: the arithmetic of coverage, and the limit of the gate, which checks fields and **not** provenance;
//! - bridge: three synthetic registries, and the gate only opens with designated roles, cases at the minimum and a status
//!   compatible with the fair test still to be done.
//!
//! Hard rule: no output from here counts coverage. The rehearsals do not write to empirical/ and do not seal anything.

use std::path::{Path, PathBuf};

use stochology_horizons::h3::pretest::{Pretest, evaluate};

use crate::escuta;
use crate::ponte;

/// The five item pools of the pre-test, with the resolutions the world has already given.
pub const ITEMS: [(&str, &str); 5] = [
    (
        "desenvolvimento_preston",
        "empirical/p12-shelf/preteste/desenvolvimento_preston.yaml",
    ),
    (
        "direito_prisoes",
        "empirical/p12-shelf/preteste/direito_prisoes.yaml",
    ),
    (
        "direito_teto_beneficio",
        "empirical/p12-shelf/preteste/direito_teto_beneficio.yaml",
    ),
    (
        "educacao_star",
        "empirical/p12-shelf/preteste/educacao_star.yaml",
    ),
    ("h1_real_c", "empirical/p12-shelf/preteste/h1_real_c.yaml"),
];

/// Repository root, the same convention as the instruments that read data/.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

/// A synthetic respondent, with the declared answering rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Respondent {
    /// p = 0.50 for every item: the declared null of the protocol.
    Coin,
    /// constant p = 0.75: speaks with confidence and has no information.
    Fluent,
    /// constant p = 0.90: the yes bias, which looks good when the pool is unbalanced.
    YesBias,
    /// Knows 80% of the items (p = 0.85, correct) and guesses the other 20% (p = 0.50): the plausible specialist.
    Expert,
    /// Always correct with p = 0.99: the ceiling, which the evaluator reads as skill 0 by 0/0.
    Oracle,
}

impl Respondent {
    pub const ALL: [Respondent; 5] = [
        Respondent::Coin,
        Respondent::Fluent,
        Respondent::YesBias,
        Respondent::Expert,
        Respondent::Oracle,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Respondent::Coin => "coin p=0.50",
            Respondent::Fluent => "fluent p=0.75",
            Respondent::YesBias => "yes-bias p=0.90",
            Respondent::Expert => "expert p=0.85/0.50",
            Respondent::Oracle => "oracle p=0.99",
        }
    }

    /// The answer the respondent gives to the item, with the confidence it declares in it.
    ///
    /// Coin and Fluent guess yes or no without looking at the resolution, and therefore hit about half the time;
    /// what separates them is the declared confidence. YesBias always answers yes. Oracle answers the resolution.
    pub fn answers_item(self, resolution: bool, rng: &mut stochology_core::Rng) -> (f64, bool) {
        match self {
            Respondent::Coin => {
                let answer = rng.uniform(2) == 1;
                (0.50, answer == resolution)
            }
            Respondent::Fluent => {
                let answer = rng.uniform(2) == 1;
                (0.75, answer == resolution)
            }
            Respondent::YesBias => (0.90, resolution),
            Respondent::Expert => {
                if rng.uniform(100) < 80 {
                    (0.85, true)
                } else {
                    let answer = rng.uniform(2) == 1;
                    (0.50, answer == resolution)
                }
            }
            Respondent::Oracle => (0.99, true),
        }
    }

    /// Answers the whole pool: the declared confidence and the hit of each item.
    pub fn answers(self, resolutions: &[bool], seed: u64) -> (Vec<f64>, Vec<bool>) {
        let mut rng = stochology_core::Rng::new(seed);
        let mut confidence = Vec::with_capacity(resolutions.len());
        let mut hit = Vec::with_capacity(resolutions.len());
        for &yes in resolutions {
            let (p, correct) = self.answers_item(yes, &mut rng);
            confidence.push(p);
            hit.push(correct);
        }
        (confidence, hit)
    }
}

/// Reads the resolutions of an item file: one line "resolucao: sim|nao" per item.
pub fn parse_resolutions(text: &str) -> Vec<bool> {
    let mut resolutions = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if let Some(value) = l.strip_prefix("resolucao:") {
            match value.trim() {
                "sim" => resolutions.push(true),
                "nao" => resolutions.push(false),
                _ => {}
            }
        }
    }
    resolutions
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// The SHELF panel rehearsal: the four respondents on the five real pools.
pub fn shelf() -> Result<String, String> {
    let root = root();
    let mut out = String::new();
    out.push_str("== synthetic rehearsal :: the SHELF panel, with declared respondents ==\n");
    out.push_str(
        "label: SYNTHETIC. No line below counts panel coverage; what is measured is the floor of a\n\
         fluent and uncalibrated respondent, so that the real panel has something to be compared against.\n\n",
    );
    out.push_str(&format!(
        "{:<22} {:<26} {:>3} {:>8} {:>8} {:>8} {:>9}\n",
        "respondent", "item pool", "n", "brier", "skill", "ece", "approved"
    ));

    let mut total_confidence: Vec<Vec<f64>> = vec![Vec::new(); Respondent::ALL.len()];
    let mut total_hit: Vec<Vec<bool>> = vec![Vec::new(); Respondent::ALL.len()];

    for (index, (name, path)) in ITEMS.iter().enumerate() {
        let text = read(&root.join(path))?;
        let resolutions = parse_resolutions(&text);
        if resolutions.is_empty() {
            return Err(format!("pool without items: {path}"));
        }
        for (r, respondent) in Respondent::ALL.iter().enumerate() {
            let seed = 0x5EED_0000 + index as u64 * 16 + r as u64;
            let (confidence, hit) = respondent.answers(&resolutions, seed);
            let evaluated = evaluate(&confidence, &hit);
            total_confidence[r].extend_from_slice(&confidence);
            total_hit[r].extend_from_slice(&hit);
            out.push_str(&format!(
                "{:<22} {:<26} {:>3} {:>8.4} {:>8.4} {:>8.4} {:>9}\n",
                respondent.name(),
                name,
                evaluated.n,
                evaluated.brier,
                evaluated.skill,
                evaluated.ece,
                if evaluated.approved { "yes" } else { "no" }
            ));
        }
    }

    out.push_str(&format!(
        "\n{:<22} {:<26} {:>3} {:>8} {:>8} {:>8} {:>9}\n",
        "respondent", "all pools", "n", "brier", "skill", "ece", "approved"
    ));
    for (r, respondent) in Respondent::ALL.iter().enumerate() {
        let evaluated: Pretest = evaluate(&total_confidence[r], &total_hit[r]);
        out.push_str(&format!(
            "{:<22} {:<26} {:>3} {:>8.4} {:>8.4} {:>8.4} {:>9}\n",
            respondent.name(),
            "all pools",
            evaluated.n,
            evaluated.brier,
            evaluated.skill,
            evaluated.ece,
            if evaluated.approved { "yes" } else { "no" }
        ));
    }
    out.push_str(
        "\nReading: the fluent respondent declares 0.75 knowing nothing. If it passes the same threshold the\n\
         panel uses, the threshold measures declared confidence and not knowledge. The expert shows the floor a real\n\
         panel needs to beat: positive skill with 80% of the items known and the rest guessed.\n\
         Declared divergence: the perfect oracle appears with skill 0 and rejected, because with accuracy 1 the\n\
         climatological variance is zero and the 0/0 ratio is read as 0 by the evaluator guard.\n",
    );
    Ok(out)
}

/// A synthetic listening registry with `heard` covered rows.
fn registry_with(heard: usize, fabricated: bool) -> String {
    let mut text = String::from(
        "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\n",
    );
    let decisions = [
        "h2g/saida/direito_teto_beneficio",
        "h2g/saida/educacao_star_turmas",
        "h2g/saida/saude_rand",
        "h1-real/saida/b",
        "h3/prisoes",
        "h3/prisoes20",
        "h3/prisoes_ri",
    ];
    for (k, decision) in decisions.iter().enumerate() {
        if k < heard {
            let who = if fabricated {
                "synthetic structural equivalent"
            } else {
                "structural equivalent heard"
            };
            let file = format!("transcricao-{:02}.md", k + 1);
            text.push_str(&format!("{decision}\ttitulares\t1\t{who}\t{file}\tnao\n"));
        } else {
            text.push_str(&format!("{decision}\ttitulares\t1\tnenhum\t-\tnao\n"));
        }
    }
    text
}

/// The listening rehearsal: the arithmetic of coverage and the limit of the gate.
pub fn listening() -> Result<String, String> {
    let real = escuta::parse(&read(&root().join(escuta::REGISTRY))?)?;
    let verdict = real.verdict();
    let mut out = String::new();
    out.push_str("== synthetic rehearsal :: listening to those who pay for the error ==\n");
    out.push_str(
        "label: SYNTHETIC. The registries below are built in code; none of them counts coverage.\n\n",
    );
    out.push_str(&format!(
        "real registry: {} decisions | coverage {} of {} | minimum {} | {}\n",
        real.rows.len(),
        verdict.coverage,
        real.rows.len(),
        real.min_groups,
        if verdict.sealed {
            "BOX SEALED"
        } else {
            "BOX OPEN"
        }
    ));

    out.push_str("\ncoverage arithmetic, with a synthetic registry:\n");
    for k in 0..=3usize {
        let registry = escuta::parse(&registry_with(k, false))?;
        let v = registry.verdict();
        out.push_str(&format!(
            "  {} group(s) heard | coverage {} of {} | minimum {} | {}\n",
            k,
            v.coverage,
            registry.rows.len(),
            registry.min_groups,
            if v.sealed { "BOX SEALED" } else { "BOX OPEN" }
        ));
    }

    out.push_str("\n-- the limit of the gate: it checks the field, not the provenance --\n");
    let fabricated = escuta::parse(&registry_with(7, true))?;
    let vf = fabricated.verdict();
    out.push_str(&format!(
        "  seven fabricated records, with a plausible name and a transcript file: coverage {} of {} | {}\n",
        vf.coverage,
        fabricated.rows.len(),
        if vf.sealed { "BOX SEALED" } else { "BOX OPEN" }
    ));
    out.push_str(
        "  reading: the gate does not tell a signed transcript from an invented transcript, because it only looks at\n\
         the fields. Real coverage needs provenance -- the hash of the signed transcript, chained into the\n\
         governance registry -- and that does not exist today. It stays declared as the next field of the registry:\n\
         transcript_hash, checked like the seals, and not a new stamp of confidence.\n",
    );

    out.push_str("\n-- the frozen script against the forbidden vocabulary --\n");
    let script = [
        "What is your worst scenario that is still plausible?",
        "What would make you refuse this intervention?",
        "When this policy fails, who in fact bears the cost?",
    ];
    let forbidden = [
        "quantil", "beta", "brier", "evpi", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8",
    ];
    let mut leaks = 0;
    for question in script {
        let lower = question.to_lowercase();
        let found: Vec<&str> = forbidden
            .iter()
            .copied()
            .filter(|p| lower.contains(p))
            .collect();
        out.push_str(&format!(
            "  {:<70} {}\n",
            question,
            if found.is_empty() {
                "no leak".to_string()
            } else {
                format!("LEAKS: {found:?}")
            }
        ));
        if !found.is_empty() {
            leaks += 1;
        }
    }
    out.push_str(&format!(
        "  questions with vocabulary leaks: {} of {}\n",
        leaks,
        script.len()
    ));
    Ok(out)
}

const BRIDGE_HEADER: &str = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\n";

/// A synthetic bridge registry: designated roles, cases done and declared status.
fn bridge_registry(designated_roles: usize, cases: usize, status: &str) -> String {
    let mut text = String::from(BRIDGE_HEADER);
    let roles = [
        ("escolhedor_dos_casos", "from outside the program"),
        ("analistas_braco_A", "two humans"),
        ("analistas_braco_B", "two humans"),
        ("juiz", "outside human, blind to the arm"),
    ];
    for (k, (role, requirement)) in roles.iter().enumerate() {
        let who = if k < designated_roles {
            "synthetic-person"
        } else {
            "-"
        };
        let state = if k < designated_roles {
            "designado"
        } else {
            "nao designado"
        };
        text.push_str(&format!("{role}\t{requirement}\t{who}\t{state}\n"));
    }
    text.push_str(&format!("casos\tcasos mais dificeis\t-\t{cases} de 12\n"));
    text.push_str(&format!(
        "status_da_tese\tnao sustentada ate o teste justo\t-\t{status}\n"
    ));
    text
}

/// How many of the four roles are designated in the verdict.
fn designated_roles_count(designated: &[String]) -> usize {
    designated
        .iter()
        .filter(|p| !p.starts_with("casos"))
        .count()
}

/// The bridge rehearsal: four synthetic registries and the gate that only opens with the fair test.
pub fn bridge() -> Result<String, String> {
    let real = ponte::parse_bridge(&read(&root().join(ponte::REGISTRY))?)?;
    let vr = real.verdict();
    let mut out = String::new();
    out.push_str("== synthetic rehearsal :: the bridge thesis at scale ==\n");
    out.push_str(
        "label: SYNTHETIC. No registry below counts a chosen case or a designated role.\n\n",
    );
    out.push_str(&format!(
        "real registry: {} roles | pending {:?} | cases missing {} | status \"{}\" | {}\n",
        real.roles.len(),
        vr.pending_roles,
        vr.cases_missing,
        vr.declared_status,
        if vr.box_closed {
            "THESIS NOT SUPPORTED"
        } else {
            "BOX OPEN"
        }
    ));
    let cases = [
        (0usize, 0usize, "nao sustentada", "no role, no case"),
        (4, 0, "em teste", "roles complete, cases missing"),
        (4, 12, "em teste", "fair test assembled"),
        (0, 0, "sustentada", "status forged without the fair test"),
    ];
    out.push_str("\nsynthetic registries:\n");
    for (roles, cases_done, status, label) in cases {
        let registry = ponte::parse_bridge(&bridge_registry(roles, cases_done, status))?;
        let v = registry.verdict();
        let divergence = if v.divergences.is_empty() {
            String::new()
        } else {
            format!(" | DIVERGENCE: {}", v.divergences.join("; "))
        };
        out.push_str(&format!(
            "  {:<38} roles {} of 4 | cases {:>2} of 12 | status \"{}\": {}{}\n",
            label,
            designated_roles_count(&v.designated_roles),
            cases_done,
            v.declared_status,
            if v.box_closed {
                "box closed"
            } else {
                "box open"
            },
            divergence
        ));
    }
    out.push_str(
        "\nReading: the gate separates three things an opinion conflates -- a filled role, a chosen case\n\
         and a declared status. A \"supported\" status with zero cases is rejected, as the real registry already rejects.\n",
    );
    Ok(out)
}

/// Deterministic bootstrap index generator for resampling `n` items `replicas` times with a fixed seed.
pub fn bootstrap_samples(n: usize, replicas: usize, seed: u64) -> Vec<Vec<usize>> {
    let mut g = stochology_core::Rng::new(seed);
    let mut out = Vec::with_capacity(replicas);
    for _ in 0..replicas {
        let mut sample = Vec::with_capacity(n);
        for _ in 0..n {
            sample.push(g.uniform(n as u64) as usize);
        }
        out.push(sample);
    }
    out
}

/// The Point 21 rehearsal (`DECLARACAO.md` §7): three model coders of different families over the 60 external errors.
pub fn layers() -> Result<String, String> {
    use crate::camadas;
    let root = root();
    let cases = camadas::parse_catalogue(&read(&root.join(camadas::CATALOGUE))?)?;
    let reg = camadas::parse_registry(&read(&root.join(camadas::REGISTRY))?)?;
    let codings_rows = camadas::parse_codings(&read(&root.join(camadas::REHEARSAL_CODINGS))?)?;
    if codings_rows.len() != cases.len() {
        return Err(format!(
            "rehearsal codings have {} rows, catalogue has {}",
            codings_rows.len(),
            cases.len()
        ));
    }
    let items: Vec<Vec<usize>> = codings_rows.into_iter().map(|(_, c)| c).collect();
    let eval = camadas::evaluate_codings(&items, reg.bootstrap_replicas, reg.bootstrap_seed);

    let mut out = String::new();
    out.push_str("== synthetic rehearsal :: can the layers fail? (three model-family coders) ==\n");
    out.push_str(
        "label: SYNTHETIC. As declared in empirical/p21-camadas/DECLARACAO.md §7, this run measures\n\
         only the floor of Fleiss' kappa across three model coders on the 60 external errors; it counts\n\
         zero human coders and does not open the governance box.\n\n",
    );
    out.push_str(&format!(
        "catalogue: {} errors | coders: {} (synthetic model families) | bootstrap: {} replicas (seed {})\n",
        eval.n_errors, eval.n_coders, reg.bootstrap_replicas, reg.bootstrap_seed
    ));
    out.push_str(&format!(
        "overall Fleiss' kappa: {:.4} | 90% bootstrap CI [{:.4}; {:.4}] -> synthetic floor: {}\n\n",
        eval.overall_kappa,
        eval.overall_ci90.0,
        eval.overall_ci90.1,
        eval.frame_verdict.as_str()
    ));
    out.push_str(&format!(
        "{:<8} {:>10} {:>10} {:>22}\n",
        "code", "mentions", "kappa", "90% bootstrap CI"
    ));
    for (name, kc, ci, mentions) in &eval.per_code_kappa {
        out.push_str(&format!(
            "{:<8} {:>10} {:>10.4} [{:>7.4}; {:>7.4}]\n",
            name, mentions, kc, ci.0, ci.1
        ));
    }
    out.push_str(&format!(
        "\nfalsifier shares (majority >= 2 of 3 coders):\n  ARIT majority share: {:.4} [{:.4}; {:.4}]\n  FORA majority share: {:.4} [{:.4}; {:.4}]\n  ARIT + FORA share:   {:.4} [{:.4}; {:.4}] (ceiling 0.5000)\n",
        eval.arit_majority_share,
        eval.arit_ci90.0,
        eval.arit_ci90.1,
        eval.fora_majority_share,
        eval.fora_ci90.0,
        eval.fora_ci90.1,
        eval.falsifier_sum_share,
        eval.falsifier_sum_ci90.0,
        eval.falsifier_sum_ci90.1
    ));
    out.push_str(&format!(
        "synthetic thesis (a) floor: {} (real human verdict stays LOCKED at BOX CLOSED in labs --layers)\n",
        eval.thesis_a_verdict.as_str()
    ));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_coin_has_no_skill() {
        let resolutions: Vec<bool> = (0..40).map(|i| i % 2 == 0).collect();
        let (confidence, hit) = Respondent::Coin.answers(&resolutions, 7);
        let evaluated = evaluate(&confidence, &hit);
        // Guessing with half of the hits: the skill stays around zero, never much above it.
        assert!(evaluated.skill.abs() < 0.10, "skill {}", evaluated.skill);
    }

    #[test]
    fn fluent_and_yes_bias_stay_below_the_expert() {
        let resolutions: Vec<bool> = (0..40).map(|i| i % 2 == 0).collect();
        let mut skills = Vec::new();
        for r in [Respondent::Fluent, Respondent::YesBias, Respondent::Expert] {
            let (confidence, hit) = r.answers(&resolutions, 11);
            skills.push(evaluate(&confidence, &hit).skill);
        }
        assert!(
            skills[0] < skills[2],
            "fluent {} vs expert {}",
            skills[0],
            skills[2]
        );
        assert!(
            skills[1] < skills[2],
            "bias {} vs expert {}",
            skills[1],
            skills[2]
        );
    }

    #[test]
    fn the_five_real_pools_have_items() {
        let root = root();
        for (name, path) in ITEMS {
            let text = std::fs::read_to_string(root.join(path)).expect("pre-test pool");
            let resolutions = parse_resolutions(&text);
            assert!(
                resolutions.len() >= 10,
                "{name} has {} items",
                resolutions.len()
            );
        }
    }

    #[test]
    fn the_real_registry_is_sealed_and_the_synthetic_one_with_seven_opens() {
        let text = std::fs::read_to_string(root().join(escuta::REGISTRY)).expect("registry");
        let real = escuta::parse(&text).expect("parse");
        assert!(real.verdict().sealed);
        let fabricated = escuta::parse(&registry_with(7, true)).expect("synthetic");
        assert_eq!(fabricated.verdict().coverage, 7);
        assert!(
            !fabricated.verdict().sealed,
            "the gate checks the field, not the provenance: that is what this test records"
        );
    }

    #[test]
    fn the_synthetic_bridge_only_opens_with_everything_and_rejects_a_forged_status() {
        let empty = ponte::parse_bridge(&bridge_registry(0, 0, "nao sustentada")).expect("empty");
        assert!(empty.verdict().box_closed);
        let complete = ponte::parse_bridge(&bridge_registry(4, 12, "em teste")).expect("complete");
        assert!(!complete.verdict().box_closed);
        assert!(complete.verdict().divergences.is_empty());
        let forged = ponte::parse_bridge(&bridge_registry(0, 0, "sustentada")).expect("forged");
        assert!(!forged.verdict().divergences.is_empty());
    }

    #[test]
    fn the_three_rehearsals_run_and_label_themselves_synthetic() {
        for out in [
            shelf().expect("shelf"),
            listening().expect("listening"),
            bridge().expect("bridge"),
            layers().expect("layers"),
        ] {
            assert!(
                out.contains("SYNTHETIC"),
                "each rehearsal carries the label"
            );
        }
    }
}
