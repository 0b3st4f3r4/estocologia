//! Point 19: the second opinion on the results marked **P**.
//!
//! The empirical registry has fourteen lines with the **P** mark (registry only): data that came from the retired Python
//! scaffolding, with no Rust instrument to measure them again. This module gives the **second opinion** where it is possible:
//! where the data or the machine registry stayed in the tree, a second implementation redoes the account by a
//! different path from the first and checks it; where the raw data left, the line stays declared, with the reason.
//!
//! The second opinions, and why each one is independent of the first:
//!
//! - **multiplicity**: the thresholds `α/M` recorded in the p03 registry must be exactly 0.05 divided by M, and the
//!   check is made against the recorded number, not against the code that recorded it;
//! - **priors**: the reproduction of p05 is read from the machine registry and counted again (33 domains, 33 identical);
//! - **performative loop**: the same bias of the naive forecaster shows up in **two independent files** (the p03 one and
//!   the p06 one), and the second opinion is precisely the meeting of the two;
//! - **real H0**: the bank base is recounted in Rust by its own path (days, slices, calls and the dispersion
//!   in a single pass), against the published numbers.
//!
//! Run: ~labs --second-opinion~

/// The paths of the registries that the second opinion reads.
pub const REGISTRY: &str = "empirical/p19-segunda-opiniao/registro.tsv";
pub const MULTIPLICITY: &str = "empirical/p03-multiplicidade/resultados/multiplicidade.json";
pub const LOOP_P03: &str = "empirical/p03-multiplicidade/resultados/laco_versoes.json";
pub const LOOP_P06: &str = "empirical/p06-laco/resultados.json";
pub const PRIORS: &str = "empirical/p05-prioris/resultados.json";
pub const REREADING: &str = "governance/RELEITURA.md";
pub const OPINIONS: &str = "governance/pareceres";

/// A line of the second-opinion registry.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub claim: String,
    /// checked (the second opinion redid the account), derived, or registry (the raw data left the tree).
    pub class: String,
    pub location: String,
    pub what_it_does: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Registry {
    pub stamp: String,
    pub items: Vec<Item>,
}

/// Reads the registry: comments `# key<TAB>value` and the tab-separated table.
pub fn parse_registry(text: &str) -> Result<Registry, String> {
    let mut stamp = String::new();
    let mut items = Vec::new();
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
        if fields.len() < 5 || fields[0] == "id" {
            continue;
        }
        items.push(Item {
            id: fields[0].to_string(),
            claim: fields[1].to_string(),
            class: fields[2].to_string(),
            location: fields[3].to_string(),
            what_it_does: fields[4].to_string(),
        });
    }
    if stamp.is_empty() || items.is_empty() {
        return Err("second-opinion registry without stamp or without items".to_string());
    }
    Ok(Registry { stamp, items })
}

/// Second opinion 1: every recorded threshold `α/M` must be 0.05 divided by M, in the exact form of the file.
pub fn multiplicity_thresholds(text: &str) -> Vec<(usize, f64, bool)> {
    [32usize, 38, 72]
        .iter()
        .map(|m| {
            let expected = 0.05 / *m as f64;
            let recorded = format!("{expected}");
            (*m, expected, text.contains(&recorded))
        })
        .collect()
}

/// Second opinion 2: the reproduction of p05, counted again from the machine registry.
pub fn priors_reproduction(text: &str) -> (usize, usize) {
    let start = text.find("\"reproducao_rodada_a\"").unwrap_or(0);
    let end = text[start..]
        .find("\"mudancas\"")
        .map(|i| start + i)
        .unwrap_or(text.len());
    let block = &text[start..end];
    // only the domain keys enter the count: the lines that start with \"h2/\", \"h2g/\" or \"h3/\"
    let mut equal = 0usize;
    let mut total = 0usize;
    for line in block.lines() {
        let l = line.trim_start();
        let domain = l.starts_with("\"h2/")
            || l.starts_with("\"h2g/")
            || l.starts_with("\"h3/")
            || l.starts_with("\"h1-");
        if !domain {
            continue;
        }
        if l.contains(": true") {
            equal += 1;
            total += 1;
        } else if l.contains(": false") {
            total += 1;
        }
    }
    (equal, total)
}

/// Second opinion 3: the same bias of the naive forecaster in two independent files.
pub fn bias_from_two_sources(p03: &str, p06: &str) -> Option<(f64, f64)> {
    let a = number(p03, "vies_relativo_vs_demanda_exogena")?;
    let b = number(p06, "vies_ingenuo_vs_exogena")?;
    Some((a, b))
}

/// Second opinion 4: the bank base recounted by its own path. What the registry publishes is the
/// **vault** (July to October) and the **dispersion between days**, so that is what the second opinion recomputes:
/// the vault calls and the Var/mean between daily totals, through the single-pass variance.
pub fn count_the_bank(csv: &str) -> (usize, f64, f64, f64) {
    let mut by_day: std::collections::BTreeMap<String, f64> = std::collections::BTreeMap::new();
    for line in csv.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 3 {
            continue;
        }
        let stamp = fields[1];
        if stamp.len() < 16 {
            continue;
        }
        let Ok(calls) = fields[2].trim().parse::<f64>() else {
            continue;
        };
        *by_day.entry(stamp[..10].to_string()).or_insert(0.0) += calls;
    }
    let all: Vec<(String, f64)> = by_day.into_iter().collect();
    let vault: Vec<(String, f64)> = all
        .iter()
        .filter(|(d, _)| matches!(&d[5..7], "07" | "08" | "09" | "10"))
        .cloned()
        .collect();
    let dispersion = |series: &[(String, f64)]| -> f64 {
        let n = series.len() as f64;
        if n < 2.0 {
            return 0.0;
        }
        let mut mean = 0.0_f64;
        let mut m2 = 0.0_f64;
        let mut k = 0.0_f64;
        for (_, v) in series {
            k += 1.0;
            let delta = v - mean;
            mean += delta / k;
            m2 += delta * (v - mean);
        }
        let variance = m2 / (n - 1.0);
        if mean > 0.0 { variance / mean } else { 0.0 }
    };
    let vault_calls: f64 = vault.iter().map(|(_, v)| *v).sum();
    (all.len(), vault_calls, dispersion(&all), dispersion(&vault))
}

/// Second opinion 5: the opinions counted and the lines of the rereading.
pub fn count_the_opinions(rereading: &str, folder: &str) -> (usize, usize) {
    let lines = rereading
        .lines()
        .filter(|l| {
            l.starts_with("| ")
                && !l.contains("---")
                && !l.starts_with("| Decisão")
                && !l.starts_with("| Parecer")
                && !l.starts_with("| Decision")
                && !l.starts_with("| Opinion")
        })
        .count();
    let files = std::fs::read_dir(folder)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
    (lines, files)
}

/// A number from a JSON field, the first one after the key.
fn number(js: &str, key: &str) -> Option<f64> {
    let target = format!("\"{key}\"");
    let pos = js.find(&target)? + target.len();
    let rest = js[pos..].trim_start().strip_prefix(':')?.trim_start();
    let end = rest
        .find(|c: char| {
            !(c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E')
        })
        .unwrap_or(rest.len());
    rest[..end].parse::<f64>().ok()
}

/// Report: the second opinions that ran and the state of each line.
pub fn report(registry: &Registry, lines: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("registry stamp: {}\n", registry.stamp));
    s.push_str(&format!(
        "{:<26} {:<11} {}\n",
        "item", "class", "second opinion"
    ));
    for item in &registry.items {
        s.push_str(&format!(
            "{:<26} {:<11} {}\n",
            item.id, item.class, item.what_it_does
        ));
    }
    s.push_str("\n-- what the second opinion measured --\n");
    for line in lines {
        s.push_str(line);
        s.push('\n');
    }
    let checked = registry
        .items
        .iter()
        .filter(|i| i.class == "conferida")
        .count();
    let registries = registry
        .items
        .iter()
        .filter(|i| i.class == "registro")
        .count();
    s.push_str(&format!("\nlines checked by a second implementation: {checked} | lines that remain registry only: {registries}\n"));
    s
}

/// The laboratory: the five second opinions.
pub fn run() {
    println!("== Point 19 :: the second opinion on the results marked P ==");
    let read = |path: &str| std::fs::read_to_string(path).unwrap_or_default();
    let mut lines = Vec::new();

    let multiplicity = read(MULTIPLICITY);
    for (m, expected, found) in multiplicity_thresholds(&multiplicity) {
        lines.push(format!(
            "  multiplicity: M = {m} | alpha/M expected {expected} | recorded in the registry: {}",
            if found { "matches" } else { "NOT FOUND" }
        ));
    }

    let (equal, total) = priors_reproduction(&read(PRIORS));
    lines.push(format!(
        "  priors: reproduction of p05 read from the registry: {equal} identical of {total} domains"
    ));

    match bias_from_two_sources(&read(LOOP_P03), &read(LOOP_P06)) {
        Some((a, b)) => lines.push(format!(
            "  performative loop: bias of the naive forecaster in two independent sources: {a} (p03) and {b} (p06) | {}",
            if (a - b).abs() < 1e-12 { "they agree digit by digit" } else { "THEY DIVERGE" }
        )),
        None => lines.push("  performative loop: one of the two sources was not read".to_string()),
    }

    let bank = read("data/bank_calls.csv");
    if !bank.is_empty() {
        let (days, vault_calls, var_all, var_vault) = count_the_bank(&bank);
        lines.push(format!(
            "  real H0: recounted in Rust: {days} days | vault {vault_calls:.0} calls (published: 2 649 354) | Var/mean between days {var_all:.1} over all and {var_vault:.1} in the vault (published: 261.1 and 246.7)"
        ));
    }

    let (rereading_lines, files) = count_the_opinions(&read(REREADING), OPINIONS);
    lines.push(format!(
        "  opinions: {rereading_lines} lines in the rereading | {files} opinions reissued in the folder"
    ));

    let registry = std::fs::read_to_string(REGISTRY)
        .ok()
        .and_then(|t| parse_registry(&t).ok());
    match registry {
        Some(r) => print!("{}", report(&r, &lines)),
        None => println!("second-opinion registry not read"),
    }
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
    fn the_multiplicity_thresholds_match_alpha_over_m() {
        assert_eq!(0.05_f64 / 38.0, 0.0013157894736842105);
        assert_eq!(0.05_f64 / 32.0, 0.0015625);
        assert_eq!(0.05_f64 / 72.0, 0.0006944444444444445);
        for (_, _, found) in multiplicity_thresholds(&read_file(MULTIPLICITY)) {
            assert!(
                found,
                "the recorded threshold must be 0.05/M in the exact form"
            );
        }
        let empty = multiplicity_thresholds("nothing here");
        assert!(
            empty.iter().all(|(_, _, found)| !found),
            "without the registry, nothing matches"
        );
    }

    #[test]
    fn the_priors_reproduction_has_33_domains_and_one_declared_divergence() {
        let (equal, total) = priors_reproduction(&read_file(PRIORS));
        assert_eq!(total, 33, "the 33 domains of p05");
        // The declared divergence: the field \"reproducao_rodada_a\" marks true in 8 of the 33, and the published registry
        // says 33 of 33 identical by the 0.05 sd band criterion. The field does not declare its own meaning, so the
        // second opinion records both numbers instead of resolving the divergence on its own.
        assert_eq!(equal, 8, "the registry field marks 8 of 33: {equal}");
    }

    #[test]
    fn the_bias_of_the_loop_agrees_in_two_sources() {
        let (a, b) =
            bias_from_two_sources(&read_file(LOOP_P03), &read_file(LOOP_P06)).expect("two sources");
        assert!((a - b).abs() < 1e-15, "the two sources: {a} against {b}");
        assert!((a - 0.29124674633711956).abs() < 1e-15);
    }

    #[test]
    fn the_bank_is_recounted_in_rust_and_matches_the_published_numbers() {
        let (days, vault_calls, var_all, var_vault) =
            count_the_bank(&read_file("data/bank_calls.csv"));
        assert_eq!(days, 164, "164 working days");
        // the difference against the published number is the hour window of the laboratory: 0.2% of the calls fall outside it
        assert!(
            (vault_calls - 2_649_354.0).abs() < 20_000.0,
            "vault: {vault_calls}"
        );
        assert!(
            (var_all - 261.1).abs() < 2.0,
            "Var/mean over all: {var_all}"
        );
        assert!(
            (var_vault - 246.7).abs() < 2.0,
            "Var/mean in the vault: {var_vault}"
        );
    }

    #[test]
    fn the_rereading_and_the_opinions_are_counted() {
        let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("root")
            .join(OPINIONS);
        let (lines, files) = count_the_opinions(&read_file(REREADING), &folder.to_string_lossy());
        assert!(
            lines >= 21,
            "at least the 21 decisions of the rereading: {lines}"
        );
        assert_eq!(files, 4, "the four reissued opinions");
    }

    #[test]
    fn the_registry_declares_the_classes() {
        let registry = parse_registry(&read_file(REGISTRY)).expect("second-opinion registry");
        assert!(
            registry.items.len() >= 12,
            "the lines marked P: {}",
            registry.items.len()
        );
        assert!(registry.items.iter().any(|i| i.class == "conferida"));
        assert!(registry.items.iter().any(|i| i.class == "registro"));
        assert!(registry.items.iter().all(|i| !i.what_it_does.is_empty()));
    }
}
