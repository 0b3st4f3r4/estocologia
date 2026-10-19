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
}
