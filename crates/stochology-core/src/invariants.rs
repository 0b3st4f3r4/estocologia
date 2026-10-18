//! Structural invariants Phi1-Phi5 and the lock against Type III Error.

use crate::layer::Card;
use crate::resolution::Resolution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gate {
    Phi1,
    Phi2,
    Phi3,
    Phi4,
    Phi5,
    TypeIII,
}

impl Gate {
    pub fn code(self) -> &'static str {
        match self {
            Gate::Phi1 => "Phi1-simulability",
            Gate::Phi2 => "Phi2-independent-unit",
            Gate::Phi3 => "Phi3-layers-and-null-model",
            Gate::Phi4 => "Phi4-scope-and-exit",
            Gate::Phi5 => "Phi5-question-resolution",
            Gate::TypeIII => "type-III-decision-maker-question",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub gate: Gate,
    pub detail: String,
}

impl Failure {
    /// Report line: the gate code followed by the detail.
    pub fn line(&self) -> String {
        format!("{}: {}", self.gate.code(), self.detail)
    }
}

/// Failure report, one line per rejected gate.
pub fn report(failures: &[Failure]) -> String {
    if failures.is_empty() {
        return "no failures: Phi1-Phi5 and the type III lock approved".to_string();
    }
    failures
        .iter()
        .map(Failure::line)
        .collect::<Vec<String>>()
        .join("\n")
}

/// Grain marks of the day or coarser in the declared independent unit.
///
/// Declared limit: a unit finer than the day (hour, client, subgroup) stays outside this
/// check — the card is only charged when the data is already at the day or above it.
fn at_day_grain_or_coarser(unit: &str) -> bool {
    const COARSER: [&str; 12] = [
        "semana", "mes", "mês", "mensal", "trimestr", "semestr", "ano", "anual", "agregad",
        "média", "media", "global",
    ];
    const DAY: [&str; 4] = ["dia", "diari", "jornada", "24h"];
    let u = unit.trim().to_lowercase();
    COARSER.iter().any(|m| u.contains(m)) || DAY.iter().any(|m| u.contains(m))
}

/// Evaluates the card against the five invariants and the Type III lock.
pub fn evaluate(f: &Card) -> Vec<Failure> {
    let mut failures = Vec::new();

    if f.seed.is_none() || f.simulable_artifact.is_none() {
        failures.push(Failure {
            gate: Gate::Phi1,
            detail: "fixed seed or simulable artifact missing".to_string(),
        });
    }

    if f.margin.is_none() || f.independent_unit.is_none() {
        failures.push(Failure {
            gate: Gate::Phi2,
            detail: "margin without a declared independent unit".to_string(),
        });
    }

    if f.null_model.is_none() || f.definition.trim().is_empty() {
        failures.push(Failure {
            gate: Gate::Phi3,
            detail: "no null model or no one-line definition".to_string(),
        });
    }

    if !f.ecological_scope || f.who_pays_for_the_error.is_none() || f.exit_clause.is_none() {
        failures.push(Failure {
            gate: Gate::Phi4,
            detail: "ecological scope, who pays for the error or exit clause missing".to_string(),
        });
    }

    // Phi5: the declared action has to be at the same grain as the independent unit. If the
    // unit is the day (or coarser) and the resolution is R0Aggregated, the action is at a coarser
    // grain than the data. R1Day or finer passes.
    if let Some(unit) = f.independent_unit.as_deref()
        && at_day_grain_or_coarser(unit)
        && f.resolution == Resolution::R0Aggregated
    {
        failures.push(Failure {
            gate: Gate::Phi5,
            detail: format!(
                "action at {} against independent unit \"{}\": the action is at a coarser grain than the data",
                f.resolution.code(),
                unit.trim()
            ),
        });
    }

    let question = f.decision_maker_question.trim();
    let action = f.package_action.trim().to_lowercase();
    let matches_action = !action.is_empty() && question.to_lowercase().contains(&action);
    if !question.ends_with('?') || !matches_action {
        failures.push(Failure {
            gate: Gate::TypeIII,
            detail: "decision_maker_question has to end in ? and quote the package action"
                .to_string(),
        });
    }

    failures
}

#[cfg(test)]
mod tests {
    use super::{Failure, Gate, evaluate, report};
    use crate::layer::{Card, Layer, UncertaintyType};
    use crate::resolution::Resolution;

    fn good_card() -> Card {
        let mut f = Card::new("h1.a", Layer::Model, UncertaintyType::Epistemic).complete();
        f.definition = "logarithmic growth of the leveraged fund".to_string();
        f.decision_maker_question = "should we keep the 3x fund or reduce it to 1x?".to_string();
        f.package_action = "reduce it to 1x".to_string();
        f
    }

    #[test]
    fn a_complete_card_passes() {
        assert!(evaluate(&good_card()).is_empty());
    }

    #[test]
    fn a_complete_card_at_r1_day_passes_phi5() {
        let f = good_card();
        assert_eq!(f.resolution, Resolution::R1Day);
        assert_eq!(f.independent_unit.as_deref(), Some("dia"));
        let failures = evaluate(&f);
        assert!(
            !failures.iter().any(|x| x.gate == Gate::Phi5),
            "Phi5 could not fail: {}",
            report(&failures)
        );
        assert!(failures.is_empty(), "{}", report(&failures));
    }

    #[test]
    fn an_aggregated_action_against_a_day_unit_fails_phi5() {
        let f = good_card().with_resolution(Resolution::R0Aggregated);
        let failures = evaluate(&f);
        let phi5: Vec<&Failure> = failures.iter().filter(|x| x.gate == Gate::Phi5).collect();
        assert_eq!(phi5.len(), 1, "{}", report(&failures));
        assert!(
            phi5[0].detail.contains(Resolution::R0Aggregated.code()),
            "detail without the declared resolution: {}",
            phi5[0].detail
        );
        // Only Phi5 changes: the rest of the card stays approved.
        assert_eq!(failures.len(), 1, "{}", report(&failures));
    }

    #[test]
    fn the_phi5_gate_code_appears_in_the_failure_report() {
        let f = good_card().with_resolution(Resolution::R0Aggregated);
        let text = report(&evaluate(&f));
        assert!(
            text.contains(Gate::Phi5.code()),
            "report without the gate code: {}",
            text
        );
        assert!(text.contains("Phi5-question-resolution"), "{}", text);
        assert!(report(&[]).contains("approved"));
    }

    #[test]
    fn a_resolution_finer_than_the_day_also_passes() {
        for r in [Resolution::R2Hour, Resolution::R3Subquestion] {
            let f = good_card().with_resolution(r);
            assert!(
                !evaluate(&f).iter().any(|x| x.gate == Gate::Phi5),
                "{} failed with a day unit",
                r.code()
            );
        }
    }

    #[test]
    fn a_week_unit_also_fails_with_r0_aggregated() {
        let mut f = good_card().with_resolution(Resolution::R0Aggregated);
        f.independent_unit = Some("semana".to_string());
        assert!(evaluate(&f).iter().any(|x| x.gate == Gate::Phi5));
    }

    #[test]
    fn a_unit_finer_than_the_day_stays_outside_the_declared_phi5() {
        // Declared limit: the check only bites when the data is at the day or above it.
        let mut f = good_card().with_resolution(Resolution::R0Aggregated);
        f.independent_unit = Some("hora".to_string());
        assert!(!evaluate(&f).iter().any(|x| x.gate == Gate::Phi5));
    }

    #[test]
    fn without_a_unit_phi5_does_not_fire_and_phi2_does() {
        let mut f = good_card().with_resolution(Resolution::R0Aggregated);
        f.independent_unit = None;
        let failures = evaluate(&f);
        assert!(!failures.iter().any(|x| x.gate == Gate::Phi5));
        assert!(failures.iter().any(|x| x.gate == Gate::Phi2));
    }

    #[test]
    fn without_a_decision_maker_question_type_iii_fails() {
        let mut f = good_card();
        f.decision_maker_question = "should we keep the 3x fund".to_string();
        let failures = evaluate(&f);
        assert!(failures.iter().any(|x| x.gate == Gate::TypeIII));
    }

    #[test]
    fn without_a_seed_phi1_fails() {
        let mut f = good_card();
        f.seed = None;
        let failures = evaluate(&f);
        assert!(failures.iter().any(|x| x.gate == Gate::Phi1));
    }
}
