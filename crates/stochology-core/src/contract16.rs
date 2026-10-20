//! The fields of contract 1.6 (2026-10-20): six optional fields, an extension of the decision rule and of the authorship.
//!
//! Each field is optional like those of 1.3 to 1.5, absent in a card that migrates from 1.5, and checked when present:
//!
//! - **selection correlation** (`correlacao_de_selecao`), one item per estimand separated by `|`:
//!   `estimando=NAME; rho=R; f=F; sigma=S; erro=E; n_efetivo=N`. Meng's identity gives the error of the mean as
//!   `rho * sqrt((1-f)/f) * sigma` and the effective size as `f / ((1-f) rho^2)`; both are recomputed from rho, f and
//!   sigma and the card is refused when the declared value departs by more than 1%.
//! - **ECE cut** (`corte_de_ece`): `corte=C; q95_nulo=Q; n=N; previsoes=reais|simuladas`. A cut below the 95th
//!   percentile of the null ECE for that n is a cut the null itself crosses, and is refused. The card says where the null's
//!   predictions come from, because the percentile changes with them.
//! - **drift mode** (`modo_de_deriva`): `modo=covariavel|rotulo|conceito|publico`; for label drift the detection
//!   (`deteccao=BBSE` or another named test) comes before the correction of the intercept and is required.
//! - **missing mechanism** (`mecanismo_de_ausentes`): `mecanismo=MCAR|MAR|MNAR`; MNAR requires a
//!   `sensibilidade=` analysis.
//! - **subgroup audit** (`auditoria_de_subgrupos`): `taxa=FNR|FPR|PPV|TPR; familia=...; paga=...`, the three required.
//! - **causal graph** (`grafo_causal`): edges `A->B, B->C`, acyclic; the colliders are readable from it.
//!
//! Declared and not done: the recomputation of the margin of Phi2 against the null of the halves needs data that a
//! card does not carry, and stays outside this validator; the test of the independences the graph implies (Phi3) is
//! left to the instruments that hold the data.

use std::collections::BTreeMap;

fn pairs(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut map = BTreeMap::new();
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let (k, v) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not key=value"))?;
        if map
            .insert(k.trim().to_string(), v.trim().to_string())
            .is_some()
        {
            return Err(format!("'{}' is named twice", k.trim()));
        }
    }
    Ok(map)
}

fn only(map: &BTreeMap<String, String>, allowed: &[&str]) -> Result<(), String> {
    match map.keys().find(|k| !allowed.contains(&k.as_str())) {
        Some(k) => Err(format!("unknown key '{k}'")),
        None => Ok(()),
    }
}

fn number(map: &BTreeMap<String, String>, key: &str) -> Result<f64, String> {
    let v = map.get(key).ok_or_else(|| format!("no '{key}'"))?;
    v.parse::<f64>()
        .ok()
        .filter(|x| x.is_finite())
        .ok_or_else(|| format!("'{key}={v}' is not a number"))
}

fn text<'a>(map: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, String> {
    map.get(key)
        .map(String::as_str)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("no '{key}'"))
}

/// Meng's error of the mean and effective sample size: (error, n_eff).
pub fn meng(rho: f64, f: f64, sigma: f64) -> (f64, f64) {
    (
        rho * ((1.0 - f) / f).sqrt() * sigma,
        f / ((1.0 - f) * rho * rho),
    )
}

fn close(declared: f64, recomputed: f64) -> bool {
    (declared - recomputed).abs() <= 0.01 * recomputed.abs().max(1e-12)
}

/// Reads the selection correlation and recomputes the error and the effective size of each estimand.
pub fn parse_selection(text_in: &str) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    for item in text_in.split('|').map(str::trim).filter(|i| !i.is_empty()) {
        let m = pairs(item)?;
        only(&m, &["estimando", "rho", "f", "sigma", "erro", "n_efetivo"])?;
        let name = text(&m, "estimando")?.to_string();
        let (rho, f, sigma) = (number(&m, "rho")?, number(&m, "f")?, number(&m, "sigma")?);
        if !(rho != 0.0 && (-1.0..=1.0).contains(&rho)) {
            return Err(format!(
                "rho = {rho} is not in [-1, 1] and different from 0"
            ));
        }
        if !(f > 0.0 && f < 1.0) {
            return Err(format!("f = {f} is not strictly between 0 and 1"));
        }
        if sigma <= 0.0 {
            return Err("sigma must be positive".into());
        }
        let (error, n_eff) = meng(rho, f, sigma);
        let (declared_error, declared_n) = (number(&m, "erro")?, number(&m, "n_efetivo")?);
        if !close(declared_error, error) {
            return Err(format!(
                "{name}: the error declared {declared_error} departs from Meng's {error:.4}"
            ));
        }
        if !close(declared_n, n_eff) {
            return Err(format!(
                "{name}: the effective size declared {declared_n} departs from {n_eff:.2}"
            ));
        }
        if names.contains(&name) {
            return Err(format!("estimand '{name}' is named twice"));
        }
        names.push(name);
    }
    if names.is_empty() {
        return Err("the selection correlation names no estimand".into());
    }
    Ok(names)
}

pub const ECE_SOURCES: [&str; 2] = ["reais", "simuladas"];

/// Reads the ECE cut; refuses a cut below the 95th percentile of the null.
pub fn parse_ece_cut(text_in: &str) -> Result<(f64, f64), String> {
    let m = pairs(text_in)?;
    only(&m, &["corte", "q95_nulo", "n", "previsoes"])?;
    let (cut, q95) = (number(&m, "corte")?, number(&m, "q95_nulo")?);
    let n = number(&m, "n")?;
    if n < 1.0 || n.fract() != 0.0 {
        return Err(format!("n = {n} is not a count"));
    }
    if q95 <= 0.0 {
        return Err("the null percentile must be positive".into());
    }
    let source = text(&m, "previsoes")?;
    if !ECE_SOURCES.contains(&source) {
        return Err(format!(
            "'{source}' is not a source of predictions: reais, simuladas"
        ));
    }
    if cut < q95 {
        return Err(format!(
            "the cut {cut} is below the null's 95th percentile {q95}: the null itself crosses it"
        ));
    }
    Ok((cut, q95))
}

pub const DRIFT_MODES: [&str; 4] = ["covariavel", "rotulo", "conceito", "publico"];

pub fn parse_drift_mode(text_in: &str) -> Result<String, String> {
    let m = pairs(text_in)?;
    only(&m, &["modo", "deteccao"])?;
    let mode = text(&m, "modo")?;
    if !DRIFT_MODES.contains(&mode) {
        return Err(format!(
            "'{mode}' is not a drift mode: covariavel, rotulo, conceito, publico"
        ));
    }
    if mode == "rotulo" && text(&m, "deteccao").is_err() {
        return Err(
            "label drift requires the detection (deteccao=) before the correction of the intercept"
                .into(),
        );
    }
    Ok(mode.to_string())
}

pub const MECHANISMS: [&str; 3] = ["MCAR", "MAR", "MNAR"];

pub fn parse_missing_mechanism(text_in: &str) -> Result<String, String> {
    let m = pairs(text_in)?;
    only(&m, &["mecanismo", "sensibilidade"])?;
    let mech = text(&m, "mecanismo")?;
    if !MECHANISMS.contains(&mech) {
        return Err(format!("'{mech}' is not a mechanism: MCAR, MAR, MNAR"));
    }
    if mech == "MNAR" && text(&m, "sensibilidade").is_err() {
        return Err("MNAR requires a sensitivity analysis (sensibilidade=)".into());
    }
    Ok(mech.to_string())
}

pub const RATES: [&str; 4] = ["FNR", "FPR", "PPV", "TPR"];

pub fn parse_subgroup_audit(text_in: &str) -> Result<String, String> {
    let m = pairs(text_in)?;
    only(&m, &["taxa", "familia", "paga"])?;
    let rate = text(&m, "taxa")?;
    if !RATES.contains(&rate) {
        return Err(format!("'{rate}' is not a rate: FNR, FPR, PPV, TPR"));
    }
    text(&m, "familia")?;
    text(&m, "paga")?;
    Ok(rate.to_string())
}

/// Reads the causal graph `A->B, B->C`: the edges, refused if a node points to itself or the graph has a cycle.
pub fn parse_causal_graph(text_in: &str) -> Result<Vec<(String, String)>, String> {
    let mut edges: Vec<(String, String)> = Vec::new();
    for part in text_in.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let (a, b) = part
            .split_once("->")
            .ok_or_else(|| format!("'{part}' is not A->B"))?;
        let (a, b) = (a.trim(), b.trim());
        if a.is_empty() || b.is_empty() {
            return Err(format!("'{part}' has an empty node"));
        }
        if a == b {
            return Err(format!("'{a}' points to itself"));
        }
        if edges.iter().any(|(x, y)| x == a && y == b) {
            return Err(format!("the edge {a}->{b} is named twice"));
        }
        edges.push((a.to_string(), b.to_string()));
    }
    if edges.is_empty() {
        return Err("the causal graph has no edge".into());
    }
    // Kahn: a graph with a cycle leaves nodes that never reach in-degree zero.
    let mut nodes: Vec<&str> = edges
        .iter()
        .flat_map(|(a, b)| [a.as_str(), b.as_str()])
        .collect();
    nodes.sort();
    nodes.dedup();
    let mut indegree: BTreeMap<&str, usize> = nodes.iter().map(|n| (*n, 0)).collect();
    for (_, b) in &edges {
        *indegree.get_mut(b.as_str()).unwrap() += 1;
    }
    let mut ready: Vec<&str> = indegree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(n, _)| *n)
        .collect();
    let mut seen = 0;
    while let Some(n) = ready.pop() {
        seen += 1;
        for (a, b) in &edges {
            if a == n {
                let d = indegree.get_mut(b.as_str()).unwrap();
                *d -= 1;
                if *d == 0 {
                    ready.push(b.as_str());
                }
            }
        }
    }
    if seen != nodes.len() {
        return Err("the causal graph has a cycle".into());
    }
    Ok(edges)
}

/// The colliders of a graph: nodes with two parents that are not themselves linked.
pub fn colliders(edges: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    let mut nodes: Vec<&str> = edges.iter().map(|(_, b)| b.as_str()).collect();
    nodes.sort();
    nodes.dedup();
    for n in nodes {
        let parents: Vec<&str> = edges
            .iter()
            .filter(|(_, b)| b == n)
            .map(|(a, _)| a.as_str())
            .collect();
        let linked = |x: &str, y: &str| {
            edges
                .iter()
                .any(|(a, b)| (a == x && b == y) || (a == y && b == x))
        };
        'pairs: for i in 0..parents.len() {
            for j in i + 1..parents.len() {
                if !linked(parents[i], parents[j]) {
                    out.push(n.to_string());
                    break 'pairs;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Oracle of the checklist: rho = -0.21, f = 0.24, sigma = 0.488 give an error of -0.1824; the effective size is 7.16.
    #[test]
    fn meng_gives_the_error_of_the_checklist_and_the_effective_size() {
        let (e, n) = meng(-0.21, 0.24, 0.488);
        assert!((e - -0.1824).abs() < 1e-4, "{e}");
        assert!((n - 7.16).abs() < 0.01, "{n}");
        let ok = format!(
            "estimando=media; rho=-0.21; f=0.24; sigma=0.488; erro={e:.4}; n_efetivo={n:.2}"
        );
        assert_eq!(parse_selection(&ok).unwrap(), vec!["media".to_string()]);
        // mutant: the error with the sign dropped, or with f and 1-f swapped, is refused
        assert!(
            parse_selection(
                "estimando=media; rho=-0.21; f=0.24; sigma=0.488; erro=0.1824; n_efetivo=7.16"
            )
            .is_err()
        );
        assert!(
            parse_selection(
                "estimando=media; rho=-0.21; f=0.24; sigma=0.488; erro=-0.0576; n_efetivo=7.16"
            )
            .is_err()
        );
        assert!(
            parse_selection(
                "estimando=media; rho=-0.21; f=0.24; sigma=0.488; erro=-0.1824; n_efetivo=100"
            )
            .is_err()
        );
        assert!(
            parse_selection("estimando=a; rho=0; f=0.2; sigma=1; erro=0; n_efetivo=1").is_err()
        );
    }

    #[test]
    fn the_ece_cut_must_clear_the_null_and_name_its_predictions() {
        assert!(
            parse_ece_cut("corte=0.0425; q95_nulo=0.0425; n=1488; previsoes=simuladas").is_ok()
        );
        assert!(parse_ece_cut("corte=0.03; q95_nulo=0.0425; n=1488; previsoes=simuladas").is_err());
        assert!(
            parse_ece_cut("corte=0.05; q95_nulo=0.0425; n=1488; previsoes=imaginadas").is_err()
        );
        assert!(parse_ece_cut("corte=0.05; q95_nulo=0.0425; n=1488.5; previsoes=reais").is_err());
    }

    #[test]
    fn drift_missing_and_subgroup_fields_ask_for_what_their_mode_requires() {
        assert!(parse_drift_mode("modo=covariavel").is_ok());
        assert!(parse_drift_mode("modo=rotulo").is_err());
        assert!(parse_drift_mode("modo=rotulo; deteccao=BBSE").is_ok());
        assert!(parse_drift_mode("modo=sazonal").is_err());
        assert!(parse_missing_mechanism("mecanismo=MAR").is_ok());
        assert!(parse_missing_mechanism("mecanismo=MNAR").is_err());
        assert!(
            parse_missing_mechanism("mecanismo=MNAR; sensibilidade=delta de -0.2 a 0.2").is_ok()
        );
        assert!(
            parse_subgroup_audit(
                "taxa=FNR; familia=sexo x faixa etaria; paga=o paciente nao triado"
            )
            .is_ok()
        );
        assert!(parse_subgroup_audit("taxa=FNR; familia=sexo").is_err());
        assert!(parse_subgroup_audit("taxa=acuracia; familia=a; paga=b").is_err());
    }

    #[test]
    fn the_causal_graph_is_acyclic_and_shows_its_colliders() {
        let g = parse_causal_graph("U->X, U->Y, X->Y").unwrap();
        assert_eq!(g.len(), 3);
        let c = parse_causal_graph("A->S, B->S").unwrap();
        assert_eq!(colliders(&c), vec!["S".to_string()]);
        // the triangle U->X, U->Y, X->Y: Y has parents U and X, which are linked, so no collider
        assert!(colliders(&g).is_empty());
        assert!(parse_causal_graph("A->B, B->A").is_err());
        assert!(parse_causal_graph("A->A").is_err());
        assert!(parse_causal_graph("A-B").is_err());
        assert!(parse_causal_graph("").is_err());
    }
}
