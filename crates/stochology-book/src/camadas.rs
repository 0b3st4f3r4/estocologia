//! Point 21: can the layers fail? (`empirical/p21-camadas/`).
//!
//! Thesis (a) states that most errors of reasoning under uncertainty are misalignments between World, Model, and Agent
//! rather than errors of arithmetic. For that claim to be a refutable typing rather than an all-absorbing vocabulary:
//! 1. at least 60 documented errors from sources outside the programme are described neutrally in `erros.tsv`, with a
//!    lexical gate refusing any description that leaks the vocabulary of the layers;
//! 2. at least three independent coders (at least two from outside the programme) code each error into the closed list
//!    of seven codes (`WM`, `MA`, `WA`, `WMA`, `ARIT`, `FORA`, `NC`);
//! 3. Fleiss' κ is computed overall and per code (one against the rest), with a 90% bootstrap interval over the errors;
//! 4. the decision rule of `DECLARACAO.md` §6 separates **typing** (`κ >= 0.60` and `q05 >= 0.40`), **vocabulary**
//!    (`κ < 0.40`), and **inconclusive**, and tests thesis (a) against the falsifiers `ARIT` and `FORA` (`κ(ARIT) >= 0.40`
//!    and `q95(ARIT + FORA) < 0.50`, or **not testable by this design** if `ARIT` is almost never used).
//!
//! While the external human coders are not designated and have not coded the 60 errors, the gate keeps the box closed
//! (`BOX CLOSED`) and rejects any registry row claiming `tipagem` or `sustentada`.
//!
//! Run: `labs --layers` (and `labs --rehearsal-layers` for the §7 synthetic model-coder floor).

/// Path to the governance registry of point 21.
pub const REGISTRY: &str = "empirical/p21-camadas/registro.tsv";
/// Path to the sealed catalogue of 60 documented external errors.
pub const CATALOGUE: &str = "empirical/p21-camadas/erros.tsv";
/// Path to the synthetic model-coder matrix of §7 (`DECLARACAO.md`).
pub const REHEARSAL_CODINGS: &str = "empirical/p21-camadas/codificacoes_ensaio.tsv";

/// The closed list of seven codes frozen in `DECLARACAO.md` §3.
pub const CODES: [&str; 7] = ["WM", "MA", "WA", "WMA", "ARIT", "FORA", "NC"];

/// Forbidden layer-vocabulary tokens that must never appear in `erros.tsv`.
pub const FORBIDDEN_VOCABULARY: [&str; 27] = [
    "camada",
    "camadas",
    "layer",
    "layers",
    "costura",
    "costuras",
    "seam",
    "seams",
    "l1",
    "l2",
    "l3",
    "d(w",
    "d(m",
    "d(a",
    "wma",
    "wm",
    "ma",
    "wa",
    "arit",
    "fora",
    "nc",
    "estocologia",
    "stochology",
    "desalinhamento",
    "desalinhamentos",
    "misalignment",
    "misalignments",
];

/// One documented external error from `erros.tsv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCase {
    pub id: String,
    pub source: String,
    pub domain: String,
    pub description: String,
}

/// One row of the governance registry (`registro.tsv`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleRow {
    pub role: String,
    pub requirement: String,
    pub who: String,
    pub state: String,
}

/// The parsed governance registry of `empirical/p21-camadas/registro.tsv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registry {
    pub stamp: String,
    pub min_errors: usize,
    pub min_coders: usize,
    pub min_external: usize,
    pub bootstrap_seed: u64,
    pub bootstrap_replicas: usize,
    pub declared_codes: Vec<String>,
    pub roles: Vec<RoleRow>,
}

/// Verdict of the governance gate on `registro.tsv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateVerdict {
    pub designated_coders: usize,
    pub external_coders: usize,
    pub coded_errors: usize,
    pub pending: Vec<String>,
    pub declared_layers_status: String,
    pub declared_thesis_status: String,
    pub divergences: Vec<String>,
    pub box_closed: bool,
}

/// Classification of the three-layer frame by `DECLARACAO.md` §6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameVerdict {
    /// Overall κ >= 0.60 and 90% CI lower bound >= 0.40.
    Typing,
    /// Overall κ < 0.40.
    Vocabulary,
    /// 0.40 <= κ < 0.60 or 90% CI lower bound < 0.40.
    Inconclusive,
}

impl FrameVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            FrameVerdict::Typing => "typing",
            FrameVerdict::Vocabulary => "vocabulary",
            FrameVerdict::Inconclusive => "inconclusive",
        }
    }
}

/// Verdict on thesis (a) ("most errors are misalignment, not arithmetic") by `DECLARACAO.md` §6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThesisAVerdict {
    /// Frame is a typing, κ(ARIT) >= 0.40, and 90% CI upper bound of (ARIT + FORA) majority share < 0.50.
    Sustained,
    /// Falsifier ARIT was almost never used (< 3 assignments across all coders): not testable by this design.
    NotTestable,
    /// Any of the conditions failed while ARIT is alive.
    NotSustained,
}

impl ThesisAVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            ThesisAVerdict::Sustained => "sustained",
            ThesisAVerdict::NotTestable => "not testable by this design",
            ThesisAVerdict::NotSustained => "not sustained",
        }
    }
}

/// Full statistical evaluation of an `N x M` coding matrix under `DECLARACAO.md` §§5-6.
#[derive(Debug, Clone, PartialEq)]
pub struct CodingEvaluation {
    pub n_errors: usize,
    pub n_coders: usize,
    pub overall_kappa: f64,
    pub overall_ci90: (f64, f64),
    pub per_code_kappa: Vec<(String, f64, (f64, f64), usize)>,
    pub arit_majority_share: f64,
    pub arit_ci90: (f64, f64),
    pub fora_majority_share: f64,
    pub fora_ci90: (f64, f64),
    pub falsifier_sum_share: f64,
    pub falsifier_sum_ci90: (f64, f64),
    pub frame_verdict: FrameVerdict,
    pub thesis_a_verdict: ThesisAVerdict,
}

/// Scans text for forbidden layer-vocabulary tokens (matched as whole lowercase alphanumeric words or whitespace-normalized prefixes `d(w`, `d(m`, `d(a`).
pub fn vocabulary_leaks(description: &str) -> Vec<&'static str> {
    let lower = description.to_lowercase();
    let compact: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    let words: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    let mut leaks = Vec::new();
    for &token in &FORBIDDEN_VOCABULARY {
        if token.contains('(') {
            if compact.contains(token) {
                leaks.push(token);
            }
        } else if words.contains(&token) {
            leaks.push(token);
        }
    }
    leaks
}

/// Parses and validates `empirical/p21-camadas/erros.tsv`.
pub fn parse_catalogue(text: &str) -> Result<Vec<ErrorCase>, String> {
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("").trim_end();
    if header != "id\tfonte\tdominio\tdescricao" {
        return Err(format!("invalid catalogue header: {header:?}"));
    }
    let mut out = Vec::new();
    for (idx, raw) in lines.enumerate() {
        let l = raw.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() != 4 {
            return Err(format!(
                "line {}: expected 4 tab-separated fields, found {}",
                idx + 2,
                fields.len()
            ));
        }
        let expected_id = format!("E-{:02}", out.len() + 1);
        let id = fields[0].trim();
        if id != expected_id {
            return Err(format!(
                "line {}: id {id:?} out of sequence, expected {expected_id:?}",
                idx + 2
            ));
        }
        let source = fields[1].trim().to_string();
        let domain = fields[2].trim().to_string();
        let description = fields[3].trim().to_string();
        if source.is_empty() || domain.is_empty() || description.is_empty() {
            return Err(format!("line {}: empty field in {id}", idx + 2));
        }
        let combined = format!("{source} {domain} {description}");
        let leaks = vocabulary_leaks(&combined);
        if !leaks.is_empty() {
            return Err(format!(
                "line {} ({id}): description leaks layer vocabulary {:?}",
                idx + 2,
                leaks
            ));
        }
        out.push(ErrorCase {
            id: id.to_string(),
            source,
            domain,
            description,
        });
    }
    if out.len() < 60 {
        return Err(format!(
            "catalogue has {} errors, minimum declared is 60",
            out.len()
        ));
    }
    Ok(out)
}

/// Parses `empirical/p21-camadas/registro.tsv`.
pub fn parse_registry(text: &str) -> Result<Registry, String> {
    let mut stamp = String::new();
    let mut min_errors = None;
    let mut min_coders = None;
    let mut min_external = None;
    let mut bootstrap_seed = None;
    let mut bootstrap_replicas = None;
    let mut declared_codes = Vec::new();
    let mut roles = Vec::new();

    for line in text.lines() {
        let l = line.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        if let Some(rest) = l.strip_prefix('#') {
            let fields: Vec<&str> = rest.trim().split('\t').collect();
            if fields.len() >= 2 {
                let val = fields[1].trim();
                match fields[0].trim() {
                    "registros" => stamp = val.to_string(),
                    "erros_minimos" => {
                        min_errors = Some(val.parse().map_err(|_| "invalid erros_minimos")?)
                    }
                    "codificadores_minimos" => {
                        min_coders = Some(val.parse().map_err(|_| "invalid codificadores_minimos")?)
                    }
                    "externos_minimos" => {
                        min_external = Some(val.parse().map_err(|_| "invalid externos_minimos")?)
                    }
                    "semente_bootstrap" => {
                        bootstrap_seed = Some(val.parse().map_err(|_| "invalid semente_bootstrap")?)
                    }
                    "replicas_bootstrap" => {
                        bootstrap_replicas =
                            Some(val.parse().map_err(|_| "invalid replicas_bootstrap")?)
                    }
                    "codigos_fechados" => {
                        declared_codes = val.split(',').map(|s| s.trim().to_string()).collect();
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
        roles.push(RoleRow {
            role: fields[0].trim().to_string(),
            requirement: fields[1].trim().to_string(),
            who: fields[2].trim().to_string(),
            state: fields[3].trim().to_string(),
        });
    }

    Ok(Registry {
        stamp,
        min_errors: min_errors.ok_or("missing erros_minimos")?,
        min_coders: min_coders.ok_or("missing codificadores_minimos")?,
        min_external: min_external.ok_or("missing externos_minimos")?,
        bootstrap_seed: bootstrap_seed.ok_or("missing semente_bootstrap")?,
        bootstrap_replicas: bootstrap_replicas.ok_or("missing replicas_bootstrap")?,
        declared_codes,
        roles,
    })
}

impl Registry {
    /// Evaluates the governance gate on the registry.
    pub fn verdict(&self) -> GateVerdict {
        let mut designated_coders = 0usize;
        let mut external_coders = 0usize;
        let mut coded_errors = 0usize;
        let mut pending = Vec::new();
        let mut divergences = Vec::new();
        let mut declared_layers_status = String::from("inconclusivo");
        let mut declared_thesis_status = String::from("nao sustentada");

        let expected_codes: Vec<String> = CODES.iter().map(|s| s.to_string()).collect();
        if self.declared_codes != expected_codes {
            divergences.push(format!(
                "declared codes {:?} differ from closed list {:?}",
                self.declared_codes, expected_codes
            ));
        }

        for r in &self.roles {
            match r.role.as_str() {
                "catalogo_de_erros" => {
                    if r.state != "congelado" {
                        pending.push("catalogo_de_erros".to_string());
                    }
                }
                role if role.starts_with("codificador_interno") => {
                    if r.state == "designado" && r.who != "-" && !r.who.is_empty() {
                        designated_coders += 1;
                    } else {
                        pending.push(role.to_string());
                    }
                }
                role if role.starts_with("codificador_externo") => {
                    if r.state == "designado" && r.who != "-" && !r.who.is_empty() {
                        designated_coders += 1;
                        external_coders += 1;
                    } else {
                        pending.push(role.to_string());
                    }
                }
                "codificacoes" => {
                    coded_errors = r
                        .state
                        .split_whitespace()
                        .next()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);
                    if coded_errors < self.min_errors {
                        pending.push(format!(
                            "codificacoes ({} de {})",
                            coded_errors, self.min_errors
                        ));
                    }
                }
                "status_camadas" => {
                    declared_layers_status = r.state.clone();
                }
                "status_tese_a" => {
                    declared_thesis_status = r.state.clone();
                }
                _ => {}
            }
        }

        let box_closed = designated_coders < self.min_coders
            || external_coders < self.min_external
            || coded_errors < self.min_errors
            || !pending.is_empty();

        if box_closed {
            if declared_layers_status != "inconclusivo" {
                divergences.push(format!(
                    "declared status_camadas {:?} while box is closed (human coders pending)",
                    declared_layers_status
                ));
            }
            if declared_thesis_status != "nao sustentada" {
                divergences.push(format!(
                    "declared status_tese_a {:?} while box is closed (human coders pending)",
                    declared_thesis_status
                ));
            }
        }

        GateVerdict {
            designated_coders,
            external_coders,
            coded_errors,
            pending,
            declared_layers_status,
            declared_thesis_status,
            divergences,
            box_closed,
        }
    }
}

/// Maps a code string (`WM`..`NC`) to its index `0..7`.
pub fn code_index(code: &str) -> Result<usize, String> {
    CODES
        .iter()
        .position(|&c| c == code.trim())
        .ok_or_else(|| format!("unknown code {code:?}; allowed: {CODES:?}"))
}

/// Parses an `N x M` coding TSV (`id<TAB>coder_1<TAB>coder_2<TAB>coder_3...`).
pub fn parse_codings(text: &str) -> Result<Vec<(String, Vec<usize>)>, String> {
    let mut rows = Vec::new();
    let mut header_seen = false;
    let mut n_coders = 0usize;
    for (idx, line) in text.lines().enumerate() {
        let l = line.trim_end();
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if !header_seen {
            if fields.len() < 4 || fields[0].trim() != "id" {
                return Err(format!(
                    "line {}: header must start with id and at least 3 coder columns",
                    idx + 1
                ));
            }
            n_coders = fields.len() - 1;
            header_seen = true;
            continue;
        }
        if fields.len() != n_coders + 1 {
            return Err(format!(
                "line {}: expected {} columns, found {}",
                idx + 1,
                n_coders + 1,
                fields.len()
            ));
        }
        let id = fields[0].trim().to_string();
        let mut codes = Vec::with_capacity(n_coders);
        for &c in &fields[1..] {
            codes.push(code_index(c)?);
        }
        rows.push((id, codes));
    }
    if rows.is_empty() {
        return Err("coding table is empty".to_string());
    }
    Ok(rows)
}

/// Computes Fleiss' κ over `k` categories for a slice of items, where each item holds the category indices assigned by `n` coders.
pub fn fleiss_kappa(items: &[Vec<usize>], k: usize) -> f64 {
    if items.is_empty() {
        return 0.0;
    }
    let n_items = items.len();
    let n_coders = items[0].len();
    if n_coders < 2 {
        return 0.0;
    }
    let mut p_bar = 0.0;
    let mut cat_totals = vec![0usize; k];
    let denom_item = (n_coders * (n_coders - 1)) as f64;

    for row in items {
        let mut counts = vec![0usize; k];
        for &c in row {
            if c < k {
                counts[c] += 1;
                cat_totals[c] += 1;
            }
        }
        let sum_pairs: usize = counts.iter().map(|&nij| nij * nij.saturating_sub(1)).sum();
        p_bar += sum_pairs as f64 / denom_item;
    }
    p_bar /= n_items as f64;

    let total_assignments = (n_items * n_coders) as f64;
    let p_e: f64 = cat_totals
        .iter()
        .map(|&tot| {
            let pj = tot as f64 / total_assignments;
            pj * pj
        })
        .sum();

    if (1.0 - p_e).abs() < 1e-12 {
        0.0
    } else {
        (p_bar - p_e) / (1.0 - p_e)
    }
}

/// Computes the one-versus-rest Fleiss' κ for category `target_cat`.
pub fn kappa_for_code(items: &[Vec<usize>], target_cat: usize) -> f64 {
    let binary: Vec<Vec<usize>> = items
        .iter()
        .map(|row| row.iter().map(|&c| usize::from(c == target_cat)).collect())
        .collect();
    fleiss_kappa(&binary, 2)
}

/// Returns `(arit_majority_share, fora_majority_share, sum_share)` where majority means `> n_coders / 2`.
pub fn majority_shares(items: &[Vec<usize>]) -> (f64, f64, f64) {
    if items.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let arit_idx = 4usize; // "ARIT"
    let fora_idx = 5usize; // "FORA"
    let n_coders = items[0].len();
    let maj = n_coders / 2 + 1;
    let mut arit_maj = 0usize;
    let mut fora_maj = 0usize;
    for row in items {
        let a = row.iter().filter(|&&c| c == arit_idx).count();
        let f = row.iter().filter(|&&c| c == fora_idx).count();
        if a >= maj {
            arit_maj += 1;
        }
        if f >= maj {
            fora_maj += 1;
        }
    }
    let n = items.len() as f64;
    let sa = arit_maj as f64 / n;
    let sf = fora_maj as f64 / n;
    (sa, sf, sa + sf)
}

fn quantiles_90(mut vals: Vec<f64>) -> (f64, f64) {
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if vals.is_empty() {
        return (0.0, 0.0);
    }
    let n = vals.len();
    let lo_idx = ((n as f64 * 0.05).floor() as usize).min(n - 1);
    let hi_idx = ((n as f64 * 0.95).ceil() as usize)
        .saturating_sub(1)
        .min(n - 1);
    (vals[lo_idx], vals[hi_idx])
}

/// Evaluates an `N x M` coding matrix with `replicas` bootstrap resamples and applies the decision rule of `DECLARACAO.md` §6.
pub fn evaluate_codings(items: &[Vec<usize>], replicas: usize, seed: u64) -> CodingEvaluation {
    let n_errors = items.len();
    let n_coders = items.first().map(Vec::len).unwrap_or(0);
    let k = CODES.len();

    let overall_kappa = fleiss_kappa(items, k);
    let (arit_share, fora_share, sum_share) = majority_shares(items);

    let boot_indices = crate::ensaio::bootstrap_samples(n_errors, replicas, seed);
    let mut boot_overall = Vec::with_capacity(replicas);
    let mut boot_per_code: Vec<Vec<f64>> = vec![Vec::with_capacity(replicas); k];
    let mut boot_arit = Vec::with_capacity(replicas);
    let mut boot_fora = Vec::with_capacity(replicas);
    let mut boot_sum = Vec::with_capacity(replicas);

    for sample_idx in &boot_indices {
        let sample: Vec<Vec<usize>> = sample_idx.iter().map(|&i| items[i].clone()).collect();
        boot_overall.push(fleiss_kappa(&sample, k));
        for c in 0..k {
            boot_per_code[c].push(kappa_for_code(&sample, c));
        }
        let (sa, sf, ss) = majority_shares(&sample);
        boot_arit.push(sa);
        boot_fora.push(sf);
        boot_sum.push(ss);
    }

    let overall_ci90 = quantiles_90(boot_overall);
    let arit_ci90 = quantiles_90(boot_arit);
    let fora_ci90 = quantiles_90(boot_fora);
    let falsifier_sum_ci90 = quantiles_90(boot_sum);

    let mut per_code_kappa = Vec::with_capacity(k);
    for (c, &name) in CODES.iter().enumerate() {
        let kc = kappa_for_code(items, c);
        let ci = quantiles_90(boot_per_code[c].clone());
        let mentions: usize = items
            .iter()
            .map(|r| r.iter().filter(|&&x| x == c).count())
            .sum();
        per_code_kappa.push((name.to_string(), kc, ci, mentions));
    }

    let frame_verdict = if overall_kappa >= 0.60 && overall_ci90.0 >= 0.40 {
        FrameVerdict::Typing
    } else if overall_kappa < 0.40 {
        FrameVerdict::Vocabulary
    } else {
        FrameVerdict::Inconclusive
    };

    let arit_mentions = per_code_kappa[4].3;
    let arit_kappa = per_code_kappa[4].1;
    let thesis_a_verdict = if arit_mentions < 3 {
        ThesisAVerdict::NotTestable
    } else if frame_verdict == FrameVerdict::Typing
        && arit_kappa >= 0.40
        && falsifier_sum_ci90.1 < 0.50
    {
        ThesisAVerdict::Sustained
    } else {
        ThesisAVerdict::NotSustained
    };

    CodingEvaluation {
        n_errors,
        n_coders,
        overall_kappa,
        overall_ci90,
        per_code_kappa,
        arit_majority_share: arit_share,
        arit_ci90,
        fora_majority_share: fora_share,
        fora_ci90,
        falsifier_sum_share: sum_share,
        falsifier_sum_ci90,
        frame_verdict,
        thesis_a_verdict,
    }
}

/// Runs the Point 21 laboratory (`labs --layers`).
pub fn run(registry_path: &str, catalogue_path: &str) {
    println!("== Point 21 :: can the layers fail? (thesis a and the seven codes) ==");

    match std::fs::read_to_string(catalogue_path).and_then(|t| {
        parse_catalogue(&t).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }) {
        Ok(cases) => {
            let mut domains: Vec<String> = cases.iter().map(|c| c.domain.clone()).collect();
            domains.sort();
            domains.dedup();
            println!(
                "catalogue: {} documented external errors in {} | {} domains | vocabulary leaks: 0 of {}",
                cases.len(),
                catalogue_path,
                domains.len(),
                cases.len()
            );
        }
        Err(e) => println!("catalogue error: {e}"),
    }

    match std::fs::read_to_string(registry_path).and_then(|t| {
        parse_registry(&t).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }) {
        Ok(reg) => {
            let v = reg.verdict();
            println!(
                "governance registry: stamp {} | minimum {} errors, {} coders ({} external) | bootstrap seed {} ({} replicas)",
                reg.stamp,
                reg.min_errors,
                reg.min_coders,
                reg.min_external,
                reg.bootstrap_seed,
                reg.bootstrap_replicas
            );
            for r in &reg.roles {
                println!("  {:<24} {:<16} {}", r.role, r.state, r.requirement);
            }
            println!(
                "designated human coders: {} of {} (external: {} of {}) | coded errors: {} of {}",
                v.designated_coders,
                reg.min_coders,
                v.external_coders,
                reg.min_external,
                v.coded_errors,
                reg.min_errors
            );
            if v.divergences.is_empty() {
                println!(
                    "divergences: none — no line claims typing or support while human coders are pending"
                );
            } else {
                println!("divergences: REJECTED on {:?}", v.divergences);
            }
            if v.box_closed {
                println!(
                    "verdict: BOX CLOSED — frame status \"{}\" and thesis (a) \"{}\" until {} external human coders and 1 internal coder complete the {} errors",
                    v.declared_layers_status,
                    v.declared_thesis_status,
                    reg.min_external,
                    reg.min_errors
                );
            } else {
                println!("verdict: BOX OPEN — human coders complete");
            }
        }
        Err(e) => println!("registry error: {e}"),
    }

    println!("\n-- negative proof 1: a registry that claims typing with human coders pending --");
    if let Ok(raw) = std::fs::read_to_string(registry_path) {
        let forged = raw
            .replace("status_camadas\tinconclusivo ate os 3 codificadores humanos\t-\tinconclusivo", "status_camadas\tinconclusivo ate os 3 codificadores humanos\t-\ttipagem")
            .replace("status_tese_a\tnao sustentada ate o teste com codificadores humanos\t-\tnao sustentada", "status_tese_a\tnao sustentada ate o teste com codificadores humanos\t-\tsustentada");
        if let Ok(r) = parse_registry(&forged) {
            let v = r.verdict();
            println!(
                "  box closed: {} | divergences: {:?}",
                v.box_closed, v.divergences
            );
        }
    }

    println!("-- negative proof 2: high agreement where no coder ever uses the ARIT falsifier --");
    let no_arit: Vec<Vec<usize>> = (0..60)
        .map(|i| {
            let c = i % 4; // only WM, MA, WA, WMA
            vec![c, c, c]
        })
        .collect();
    let eval_no_arit = evaluate_codings(&no_arit, 200, 20_261_021);
    println!(
        "  overall kappa {:.4} [{:.4}; {:.4}] -> frame: {} | ARIT mentions: {} -> thesis (a): {}",
        eval_no_arit.overall_kappa,
        eval_no_arit.overall_ci90.0,
        eval_no_arit.overall_ci90.1,
        eval_no_arit.frame_verdict.as_str(),
        eval_no_arit.per_code_kappa[4].3,
        eval_no_arit.thesis_a_verdict.as_str()
    );

    println!("-- positive proof: high agreement with active ARIT falsifier below 50% share --");
    let active_arit: Vec<Vec<usize>> = (0..60)
        .map(|i| {
            let c = if i < 10 {
                4 // ARIT (10 of 60)
            } else if i < 14 {
                5 // FORA (4 of 60)
            } else {
                i % 4 // WM, MA, WA, WMA
            };
            vec![c, c, c]
        })
        .collect();
    let eval_pos = evaluate_codings(&active_arit, 200, 20_261_021);
    println!(
        "  overall kappa {:.4} [{:.4}; {:.4}] | kappa(ARIT) {:.4} | (ARIT+FORA) q95 {:.4} -> frame: {} | thesis (a): {}",
        eval_pos.overall_kappa,
        eval_pos.overall_ci90.0,
        eval_pos.overall_ci90.1,
        eval_pos.per_code_kappa[4].1,
        eval_pos.falsifier_sum_ci90.1,
        eval_pos.frame_verdict.as_str(),
        eval_pos.thesis_a_verdict.as_str()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn root_file(rel: &str) -> String {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("repository root")
            .to_path_buf();
        std::fs::read_to_string(root.join(rel)).expect("reads file")
    }

    #[test]
    fn the_catalogue_has_sixty_external_errors_and_zero_vocabulary_leaks() {
        let cases = parse_catalogue(&root_file(CATALOGUE)).expect("valid 60-error catalogue");
        assert_eq!(cases.len(), 60);
        for c in &cases {
            assert!(vocabulary_leaks(&c.description).is_empty());
        }
    }

    #[test]
    fn a_description_that_mentions_layers_or_seams_is_rejected() {
        let bad = "id\tfonte\tdominio\tdescricao\nE-01\tsrc\tdom\tO erro ocorreu na costura entre a camada L1 e L2.\n";
        let err = parse_catalogue(bad).expect_err("must reject vocabulary leak");
        assert!(err.contains("leaks layer vocabulary"), "{err}");

        let bad_plural_and_codes = "id\tfonte\tdominio\tdescricao\nE-01\tsrc\tdom\tA misalignment across seams and d ( W , M ) or FORA in stochology.\n";
        let err2 = parse_catalogue(bad_plural_and_codes)
            .expect_err("must reject plurals, code names, and spaced d(W, M) tokens");
        assert!(err2.contains("leaks layer vocabulary"), "{err2}");
    }

    #[test]
    fn the_governance_registry_keeps_the_box_closed_and_rejects_forged_status() {
        let reg = parse_registry(&root_file(REGISTRY)).expect("valid registry");
        let v = reg.verdict();
        assert!(v.box_closed);
        assert!(v.divergences.is_empty(), "{:?}", v.divergences);
        assert_eq!(v.designated_coders, 0);
        assert_eq!(v.external_coders, 0);

        let forged = root_file(REGISTRY).replace("\tinconclusivo\n", "\ttipagem\n");
        let vf = parse_registry(&forged).expect("parses").verdict();
        assert!(!vf.divergences.is_empty());
    }

    #[test]
    fn decision_rule_distinguishes_typing_vocabulary_and_untestable_falsifier() {
        // 1. Vocabulary: diffuse disagreement
        let diffuse: Vec<Vec<usize>> = (0..60)
            .map(|i| vec![i % 5, (i + 1) % 5, (i + 2) % 5])
            .collect();
        let ev_vocab = evaluate_codings(&diffuse, 200, 20_261_021);
        assert_eq!(ev_vocab.frame_verdict, FrameVerdict::Vocabulary);
        assert_eq!(ev_vocab.thesis_a_verdict, ThesisAVerdict::NotSustained);

        // 2. Untestable when ARIT is never used
        let no_arit: Vec<Vec<usize>> = (0..60).map(|i| vec![i % 4, i % 4, i % 4]).collect();
        let ev_untestable = evaluate_codings(&no_arit, 200, 20_261_021);
        assert_eq!(ev_untestable.frame_verdict, FrameVerdict::Typing);
        assert_eq!(ev_untestable.thesis_a_verdict, ThesisAVerdict::NotTestable);

        // 3. Typing + Sustained when ARIT is alive and (ARIT + FORA) q95 < 0.50
        let valid: Vec<Vec<usize>> = (0..60)
            .map(|i| {
                let c = if i < 10 { 4 } else { i % 4 };
                vec![c, c, c]
            })
            .collect();
        let ev_sustained = evaluate_codings(&valid, 200, 20_261_021);
        assert_eq!(ev_sustained.frame_verdict, FrameVerdict::Typing);
        assert_eq!(ev_sustained.thesis_a_verdict, ThesisAVerdict::Sustained);
    }
}
