//! Three fields of the card for what happens between the specialists (contract 1.5, 2026-10-19).
//!
//! The audit of the safeguard panel against the failure modes of teams of agents (`docs/MAST_AUDIT.md`) found three
//! that nothing in the program saw, because they happen between the specialists and not in a card: a specialist that
//! writes outside its role, a finding withheld, an action that departs from the reasoning written beside it. Each gets
//! a field, optional like the fields of 1.3 and 1.4, and checked when present:
//!
//! - **authorship** (`autoria`), `field:role, field:role`: who wrote each field. A field may be written only by the
//!   specialist whose role it belongs to ([`role_of`]); a field outside the map, an unknown role or a field named
//!   twice is refused.
//! - **findings** (`achados`), `logged=N; used=U; unused=finding: reason | finding: reason`: every finding a specialist
//!   logged is either used or listed as unused with its reason; the count must close, and no unused finding may go
//!   without a reason.
//! - **decision rule** (`regra_de_decisao`), `p=P; C=C; L=L; act=protect` or `act=do not protect`: the inequality the
//!   action rests on (Chapter 11: protect when p > C/L). The action is recomputed from the card's own numbers and
//!   refused when it departs from them; at p = C/L either act is accepted.

/// The specialist roles of Chapter 16.
pub const ROLES: [&str; 8] = [
    "code",
    "data",
    "lifecycle",
    "inference",
    "governance",
    "fairness",
    "domain",
    "decision",
];

/// The role that writes a field of the card, by its name on disk.
pub fn role_of(field: &str) -> Option<&'static str> {
    Some(match field {
        "artefato_simulavel" | "semente" | "teste_de_propriedade" => "code",
        "unidade_independente" | "trata_ausentes" | "mecanismo_de_ausentes" => "data",
        "deriva_declarada" | "modo_de_deriva" => "lifecycle",
        "modelo_nulo"
        | "margem"
        | "patologias_cobertas"
        | "orcamento_de_incerteza"
        | "multiverso"
        | "confiabilidade"
        | "correlacao_de_selecao"
        | "corte_de_ece"
        | "grafo_causal" => "inference",
        "trilha_integra" => "governance",
        "subgrupos_auditados" | "quem_paga_pelo_erro" | "auditoria_de_subgrupos" => "fairness",
        "definicao" | "priori_elicitada" | "pergunta_do_decisor" | "escopo_ecologico" => "domain",
        "acao_do_pacote" | "clausula_de_saida" | "evento_dominante" | "regra_de_decisao"
        | "resolucao" => "decision",
        _ => return None,
    })
}

/// Reads an authorship: the pairs (field, role), each field in its own role.
pub fn parse_authorship(text: &str) -> Result<Vec<(String, String)>, String> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    for part in text.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let (field, role) = part
            .split_once(':')
            .ok_or_else(|| format!("'{part}' is not field:role"))?;
        let (field, role) = (field.trim(), role.trim());
        // contract 1.6: the role may carry who played it, `role@model/persona`
        let role = match role.split_once('@') {
            Some((r, agent)) => {
                match agent.split_once('/') {
                    Some((model, persona))
                        if !model.trim().is_empty() && !persona.trim().is_empty() => {}
                    _ => return Err(format!("'{agent}' is not model/persona")),
                }
                r.trim()
            }
            None => role,
        };
        if !ROLES.contains(&role) {
            return Err(format!("'{role}' is not a role of the team"));
        }
        let owner =
            role_of(field).ok_or_else(|| format!("'{field}' is not a field with an owner"))?;
        if owner != role {
            return Err(format!(
                "'{field}' belongs to {owner}, and {role} wrote it: a specialist outside its role"
            ));
        }
        if pairs.iter().any(|(f, _)| f == field) {
            return Err(format!("'{field}' is named twice"));
        }
        pairs.push((field.to_string(), role.to_string()));
    }
    if pairs.is_empty() {
        return Err("the authorship names no field".into());
    }
    Ok(pairs)
}

/// The findings of a card: (logged, used, the unused ones with their reasons).
pub type Findings = (usize, usize, Vec<(String, String)>);

/// Reads the findings: `logged=N; used=U; unused=finding: reason | finding: reason`.
pub fn parse_findings(text: &str) -> Result<Findings, String> {
    let (mut logged, mut used, mut unused) = (None, None, Vec::new());
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not key=value"))?;
        let value = value.trim();
        match key.trim() {
            "logged" => {
                logged = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("'{part}' is not a count"))?,
                )
            }
            "used" => {
                used = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("'{part}' is not a count"))?,
                )
            }
            "unused" => {
                for item in value.split('|').map(str::trim).filter(|i| !i.is_empty()) {
                    let (finding, reason) = item.split_once(':').unwrap_or((item, ""));
                    if reason.trim().is_empty() {
                        return Err(format!(
                            "the unused finding '{}' has no reason",
                            finding.trim()
                        ));
                    }
                    unused.push((finding.trim().to_string(), reason.trim().to_string()));
                }
            }
            other => return Err(format!("unknown key '{other}'")),
        }
    }
    let logged = logged.ok_or("the findings do not say how many were logged")?;
    let used = used.ok_or("the findings do not say how many were used")?;
    if used > logged {
        return Err(format!("{used} used of {logged} logged"));
    }
    if used + unused.len() != logged {
        return Err(format!(
            "{logged} logged, {used} used and {} listed unused: {} finding(s) withheld",
            unused.len(),
            logged as i64 - used as i64 - unused.len() as i64
        ));
    }
    Ok((logged, used, unused))
}

/// The two acts of a protection decision.
pub const ACTS: [&str; 2] = ["protect", "do not protect"];

/// Reads a decision rule and recomputes its act: (p, C, L, act).
pub fn parse_decision_rule(text: &str) -> Result<(f64, f64, f64, String), String> {
    let (mut p, mut c, mut l, mut act) = (None, None, None, None);
    let mut range: Option<String> = None;
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not key=value"))?;
        let value = value.trim();
        let number = || {
            value
                .parse::<f64>()
                .map_err(|_| format!("'{part}' is not a number"))
        };
        match key.trim() {
            "p" => p = Some(number()?),
            "C" => c = Some(number()?),
            "L" => l = Some(number()?),
            "act" => act = Some(value.to_string()),
            // contract 1.6: the range of C/L over which the net benefit was read (decision curve)
            "faixa" => range = Some(value.to_string()),
            other => return Err(format!("unknown key '{other}'")),
        }
    }
    let p = p.ok_or("the rule has no probability p")?;
    let c = c.ok_or("the rule has no cost of protecting C")?;
    let l = l.ok_or("the rule has no loss L")?;
    let act = act.ok_or("the rule has no act")?;
    if !(p > 0.0 && p < 1.0) {
        return Err(format!("p = {p} is not strictly between 0 and 1"));
    }
    if !(c > 0.0 && l > 0.0) {
        return Err("C and L must be positive".into());
    }
    if !ACTS.contains(&act.as_str()) {
        return Err(format!(
            "'{act}' is not one of the acts: protect, do not protect"
        ));
    }
    let threshold = c / l;
    if let Some(r) = &range {
        let (lo, hi) = r
            .split_once(':')
            .and_then(|(a, b)| Some((a.trim().parse::<f64>().ok()?, b.trim().parse::<f64>().ok()?)))
            .ok_or_else(|| format!("'faixa={r}' is not lo:hi"))?;
        if !(0.0 < lo && lo < hi && hi < 1.0) {
            return Err(format!(
                "the range {lo}:{hi} is not inside (0, 1) and ordered"
            ));
        }
        if !(lo <= threshold && threshold <= hi) {
            return Err(format!(
                "the range {lo}:{hi} does not contain C/L = {threshold:.4}: the curve does not cover the point of the rule"
            ));
        }
    }
    let implied = if p > threshold {
        "protect"
    } else {
        "do not protect"
    };
    if (p - threshold).abs() > 1e-12 && act != implied {
        return Err(format!(
            "p = {p} against C/L = {threshold:.4} implies '{implied}', and the card acts '{act}': the action departs from its reasoning"
        ));
    }
    Ok((p, c, l, act))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorship_may_name_who_played_the_role_and_the_rule_may_name_its_range() {
        assert!(
            parse_authorship("modelo_nulo:inference@claude-sonnet-5-5/analista cetico").is_ok()
        );
        assert!(parse_authorship("modelo_nulo:inference@claude-sonnet-5-5").is_err());
        assert!(parse_authorship("clausula_de_saida:inference@m/p").is_err());
        assert!(
            parse_decision_rule("p=0.05; C=1800; L=34000; act=do not protect; faixa=0.02:0.10")
                .is_ok()
        );
        assert!(
            parse_decision_rule("p=0.05; C=1800; L=34000; act=do not protect; faixa=0.06:0.10")
                .is_err()
        );
        assert!(
            parse_decision_rule("p=0.05; C=1800; L=34000; act=do not protect; faixa=0.10:0.02")
                .is_err()
        );
    }

    #[test]
    fn a_field_written_outside_its_role_is_refused() {
        assert_eq!(
            parse_authorship("modelo_nulo:inference, clausula_de_saida:decision")
                .expect("reads")
                .len(),
            2
        );
        assert!(parse_authorship("clausula_de_saida:inference").is_err());
        assert!(parse_authorship("modelo_nulo:orchestrator").is_err());
        assert!(parse_authorship("campo_inventado:data").is_err());
        assert!(parse_authorship("modelo_nulo:inference, modelo_nulo:inference").is_err());
        assert!(parse_authorship("").is_err());
    }

    #[test]
    fn a_withheld_finding_is_refused_and_an_unused_one_needs_a_reason() {
        let f = parse_findings("logged=3; used=1; unused=a break in 2019: outside the window | a second peak: below the threshold")
            .expect("reads");
        assert_eq!((f.0, f.1, f.2.len()), (3, 1, 2));
        assert!(
            parse_findings("logged=3; used=1; unused=a break in 2019: outside the window").is_err()
        );
        assert!(parse_findings("logged=2; used=1; unused=a break in 2019").is_err());
        assert!(parse_findings("logged=1; used=2").is_err());
        assert!(parse_findings("logged=2; used=2").is_ok());
    }

    #[test]
    fn the_act_is_recomputed_from_the_cards_own_numbers() {
        // the shipowner of Chapter 11: C/L = 1,800 / 34,000 = 0.0529 against p = 0.05, so the rule says not to insure
        assert!(parse_decision_rule("p=0.05; C=1800; L=34000; act=do not protect").is_ok());
        assert!(parse_decision_rule("p=0.05; C=1800; L=34000; act=protect").is_err());
        assert!(parse_decision_rule("p=0.10; C=1800; L=34000; act=protect").is_ok());
        assert!(parse_decision_rule("p=0; C=1800; L=34000; act=protect").is_err());
        assert!(parse_decision_rule("p=0.05; C=1800; L=34000; act=insure").is_err());
        // at the threshold either act is accepted
        assert!(parse_decision_rule("p=0.5; C=1; L=2; act=protect").is_ok());
        assert!(parse_decision_rule("p=0.5; C=1; L=2; act=do not protect").is_ok());
    }
}
