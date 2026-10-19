//! Two fields of the card for the uncertainty that reaches the decision (contract 1.3, 2026-10-19).
//!
//! **The uncertainty budget** says where the variance of the output comes from: each input's share of it, and the
//! share of the covariances between inputs, which can be negative when they cancel. Written `name=share; name=share`,
//! the shares must add up to one within 0.05, so that nothing is left unaccounted for.
//!
//! **The dominant event** says which event carries the expected cost: its name, its probability and the fraction of
//! the expected cost it accounts for. Written `name; p=probability; share=fraction`, with the probability strictly
//! between 0 and 1 (Cromwell) and the fraction in (0, 1]. Frequency and severity are different axes: a rare event can
//! carry most of the cost.
//!
//! Both are optional in the contract: a card that does not declare them is valid, and one that declares them in a form
//! that cannot be read, or whose budget does not close, is refused when it is read.
//!
//! Contract 1.4 (2026-10-19) adds two more, from the research on models of agents and of modelling:
//!
//! **The multiverse** says under how many reasonable specifications the answer was computed, how many of them reach the
//! card's decision, and how the variance between them splits by choice. Written
//! `specifications=N; agree=A; shares=name:share, name:share`, with N at least 2, A at most N, and the shares closing to
//! one within 0.05. A budget that names a share `specification` must come with a multiverse, because that share is
//! measured there and nowhere else.
//!
//! **The reliability** of an agent says what average success hides: `k=K; pass1=P1; passk=PK`, with K at least 2 and
//! 0 <= PK <= P1 <= 1 (the chance that K runs all succeed cannot exceed the chance that one does).

/// Tolerance on the sum of the shares of a budget.
pub const BUDGET_TOLERANCE: f64 = 0.05;

/// Reads a budget: the named shares, which must close to one.
pub fn parse_budget(text: &str) -> Result<Vec<(String, f64)>, String> {
    let mut items = Vec::new();
    for part in text.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (name, share) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not name=share"))?;
        let name = name.trim();
        if name.is_empty() {
            return Err(format!("'{part}' has no name"));
        }
        let share: f64 = share
            .trim()
            .parse()
            .map_err(|_| format!("'{part}' has no numeric share"))?;
        if !share.is_finite() {
            return Err(format!("'{part}' has a share that is not finite"));
        }
        items.push((name.to_string(), share));
    }
    if items.is_empty() {
        return Err("the budget names no input".into());
    }
    let total: f64 = items.iter().map(|(_, s)| s).sum();
    if (total - 1.0).abs() > BUDGET_TOLERANCE {
        return Err(format!(
            "the shares add up to {total:.3}, not to one within {BUDGET_TOLERANCE}"
        ));
    }
    Ok(items)
}

/// Reads a dominant event: its name, its probability and its share of the expected cost.
pub fn parse_dominant_event(text: &str) -> Result<(String, f64, f64), String> {
    let mut parts = text.split(';').map(str::trim);
    let name = parts.next().unwrap_or("").to_string();
    if name.is_empty() || name.contains('=') {
        return Err("the dominant event has no name".into());
    }
    let (mut p, mut share) = (None, None);
    for part in parts {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not key=value"))?;
        let value: f64 = value
            .trim()
            .parse()
            .map_err(|_| format!("'{part}' is not a number"))?;
        match key.trim() {
            "p" => p = Some(value),
            "share" => share = Some(value),
            other => return Err(format!("unknown key '{other}'")),
        }
    }
    let p = p.ok_or("the dominant event has no probability p")?;
    let share = share.ok_or("the dominant event has no share of the expected cost")?;
    if !(p > 0.0 && p < 1.0) {
        return Err(format!("p = {p} is not strictly between 0 and 1"));
    }
    if !(share > 0.0 && share <= 1.0) {
        return Err(format!("share = {share} is not in (0, 1]"));
    }
    Ok((name, p, share))
}

/// The multiverse of a card: (specifications, agreeing, shares by choice).
pub type Multiverse = (usize, usize, Vec<(String, f64)>);

/// Reads a multiverse: `specifications=N; agree=A; shares=name:share, name:share`.
pub fn parse_multiverse(text: &str) -> Result<Multiverse, String> {
    let (mut n, mut agree, mut shares) = (None, None, None);
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not key=value"))?;
        let value = value.trim();
        match key.trim() {
            "specifications" => {
                n = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("'{part}' is not a count"))?,
                )
            }
            "agree" => {
                agree = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("'{part}' is not a count"))?,
                )
            }
            "shares" => shares = Some(parse_budget(&value.replace(',', ";").replace(':', "="))?),
            other => return Err(format!("unknown key '{other}'")),
        }
    }
    let n = n.ok_or("the multiverse names no number of specifications")?;
    let agree = agree.ok_or("the multiverse does not say how many specifications agree")?;
    let shares = shares.ok_or("the multiverse does not split its variance by choice")?;
    if n < 2 {
        return Err(format!("{n} specification is not a multiverse"));
    }
    if agree > n {
        return Err(format!("{agree} of {n} specifications cannot agree"));
    }
    Ok((n, agree, shares))
}

/// Reads a reliability: `k=K; pass1=P1; passk=PK`.
pub fn parse_reliability(text: &str) -> Result<(usize, f64, f64), String> {
    let (mut k, mut p1, mut pk) = (None, None, None);
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("'{part}' is not key=value"))?;
        let value = value.trim();
        match key.trim() {
            "k" => {
                k = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("'{part}' is not a count"))?,
                )
            }
            "pass1" => {
                p1 = Some(
                    value
                        .parse::<f64>()
                        .map_err(|_| format!("'{part}' is not a number"))?,
                )
            }
            "passk" => {
                pk = Some(
                    value
                        .parse::<f64>()
                        .map_err(|_| format!("'{part}' is not a number"))?,
                )
            }
            other => return Err(format!("unknown key '{other}'")),
        }
    }
    let k = k.ok_or("the reliability does not declare k")?;
    let p1 = p1.ok_or("the reliability has no pass1")?;
    let pk = pk.ok_or("the reliability has no passk")?;
    if k < 2 {
        return Err(format!("k = {k}: reliability needs at least two runs"));
    }
    if !(0.0..=1.0).contains(&p1) || !(0.0..=1.0).contains(&pk) {
        return Err("pass1 and passk are shares, between 0 and 1".into());
    }
    if pk > p1 {
        return Err(format!(
            "passk = {pk} above pass1 = {p1}: k runs cannot all succeed more often than one"
        ));
    }
    Ok((k, p1, pk))
}

/// Whether a budget names a share for the specification.
pub fn names_specification(budget: &str) -> bool {
    parse_budget(budget)
        .map(|items| items.iter().any(|(name, _)| name == "specification"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_budget_that_closes_is_read_with_a_negative_covariance() {
        let b = parse_budget("arrivals=0.82; pace=1.15; covariance=-0.97").expect("reads");
        assert_eq!(b.len(), 3);
        assert_eq!(b[2], ("covariance".to_string(), -0.97));
    }

    #[test]
    fn a_budget_that_does_not_close_is_refused() {
        assert!(parse_budget("arrivals=0.42").is_err());
        assert!(parse_budget("arrivals=0.42; pace").is_err());
        assert!(parse_budget("").is_err());
        assert!(parse_budget("=1.0").is_err());
    }

    #[test]
    fn a_dominant_event_needs_a_probability_inside_the_open_interval() {
        let e = parse_dominant_event("critical failure; p=0.01; share=0.95").expect("reads");
        assert_eq!(e, ("critical failure".to_string(), 0.01, 0.95));
        assert!(parse_dominant_event("critical failure; p=0; share=0.95").is_err());
        assert!(parse_dominant_event("critical failure; p=0.01").is_err());
        assert!(parse_dominant_event("critical failure; p=0.01; share=1.2").is_err());
        assert!(parse_dominant_event("p=0.01; share=0.5").is_err());
    }

    #[test]
    fn a_multiverse_needs_two_specifications_an_agreement_inside_them_and_closing_shares() {
        let m = parse_multiverse(
            "specifications=18; agree=10; shares=service-cv:0.93, rate-law:0.00, correlation:0.06, interaction:0.02",
        )
        .expect("reads");
        assert_eq!((m.0, m.1, m.2.len()), (18, 10, 4));
        assert!(parse_multiverse("specifications=1; agree=1; shares=a:1.0").is_err());
        assert!(parse_multiverse("specifications=18; agree=19; shares=a:1.0").is_err());
        assert!(parse_multiverse("specifications=18; agree=10; shares=a:0.5").is_err());
        assert!(parse_multiverse("specifications=18; agree=10").is_err());
        assert!(names_specification("inputs=0.4; specification=0.6"));
        assert!(!names_specification("arrivals=0.42; pace=0.58"));
    }

    #[test]
    fn a_reliability_cannot_promise_more_runs_than_one() {
        assert_eq!(
            parse_reliability("k=8; pass1=0.5312; passk=0.0700").expect("reads"),
            (8, 0.5312, 0.07)
        );
        assert!(parse_reliability("k=8; pass1=0.5; passk=0.6").is_err());
        assert!(parse_reliability("k=1; pass1=0.5; passk=0.5").is_err());
        assert!(parse_reliability("k=8; pass1=1.2; passk=0.1").is_err());
        assert!(parse_reliability("k=8; pass1=0.5").is_err());
    }
}
