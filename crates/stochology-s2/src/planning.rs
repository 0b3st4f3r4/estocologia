//! The R axis by System 2: the resolution of the question chosen by the instrument, never by guesswork.
//!
//! With the same total resource, asking about the aggregate loses what asking about the day delivers. System 2
//! evaluates both arms with `stochology_core::medir` and compares the measured loss with the **declared cost** of
//! the margin that the fine arm requires. The finer resolutions (R2 and R3) remain declared beyond the measured
//! reach: the instrument measures the aggregate/fine pair, and extrapolating would be inventing a number.
//!
//! Declared and not implemented: coupling the plan to the real budget of attendants (the cost of the margin enters
//! as a number declared by the decision-maker) and the link with the audit vault.

use stochology_core::{Resolution, ResolutionLoss};

/// What the evaluation returns for each step of the resolution ladder.
#[derive(Debug, Clone, PartialEq)]
pub struct Evaluation {
    pub resolution: Resolution,
    pub loss_min: f64,
    pub cost_min: f64,
    pub total_min: f64,
    pub out_of_reach: Option<&'static str>,
}

/// The decision of System 2 about the resolution of the question.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolutionDecision {
    pub chosen: Resolution,
    pub aggregate_loss_min: f64,
    pub margin_cost_min: f64,
    pub reason: &'static str,
}

/// The declared reason for R2 and R3 staying out: the instrument does not measure those steps today.
pub const OUT_OF_REACH: &str =
    "the instrument measures the aggregate/fine pair; the step is not extrapolated";

/// Evaluates the four steps: R0 without loss and without cost, R1 with the measured loss and the declared cost,
/// R2 and R3 declared beyond the measured reach.
pub fn evaluate(loss: &ResolutionLoss, margin_cost_min: f64) -> Vec<Evaluation> {
    let cost = margin_cost_min.max(0.0);
    vec![
        Evaluation {
            resolution: Resolution::R0Aggregated,
            loss_min: 0.0,
            cost_min: 0.0,
            total_min: 0.0,
            out_of_reach: None,
        },
        Evaluation {
            resolution: Resolution::R1Day,
            loss_min: loss.loss,
            cost_min: cost,
            total_min: loss.loss - cost,
            out_of_reach: None,
        },
        Evaluation {
            resolution: Resolution::R2Hour,
            loss_min: f64::NAN,
            cost_min: f64::NAN,
            total_min: f64::NAN,
            out_of_reach: Some(OUT_OF_REACH),
        },
        Evaluation {
            resolution: Resolution::R3Subquestion,
            loss_min: f64::NAN,
            cost_min: f64::NAN,
            total_min: f64::NAN,
            out_of_reach: Some(OUT_OF_REACH),
        },
    ]
}

/// The decision rule of System 2: it keeps the fine arm when the loss of the aggregate pays for the margin.
pub fn decide(loss: &ResolutionLoss, margin_cost_min: f64) -> ResolutionDecision {
    let cost = margin_cost_min.max(0.0);
    let fine = loss.loss > cost;
    ResolutionDecision {
        chosen: if fine {
            Resolution::R1Day
        } else {
            Resolution::R0Aggregated
        },
        aggregate_loss_min: loss.loss,
        margin_cost_min: cost,
        reason: if fine {
            "the loss of the aggregate pays for the margin of the day"
        } else {
            "the margin of the day does not pay for itself: the aggregate suffices"
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::{R_HORIZON_DAYS, measure_resolution_loss};

    /// The calls are those of the instrument itself (`labs --resolucao`): 1909, 200 days, 10 hours.
    fn chapter_16_post() -> ResolutionLoss {
        measure_resolution_loss(1909, R_HORIZON_DAYS, 10, 5.0, 11.0)
    }

    fn without_volatility() -> ResolutionLoss {
        measure_resolution_loss(1909, R_HORIZON_DAYS, 10, 8.0, 8.0)
    }

    #[test]
    fn without_volatility_the_loss_is_zero_and_the_aggregate_suffices() {
        let loss = without_volatility();
        assert_eq!(loss.loss, 0.0);
        let decision = decide(&loss, 12.0);
        assert_eq!(decision.chosen, Resolution::R0Aggregated);
        assert_eq!(
            decision.reason,
            "the margin of the day does not pay for itself: the aggregate suffices"
        );
    }

    #[test]
    fn with_volatility_the_loss_pays_for_the_margin() {
        let loss = chapter_16_post();
        assert!(
            loss.loss > 20.0,
            "the loss measured at the chapter 16 post: {}",
            loss.loss
        );
        let decision = decide(&loss, 12.0);
        assert_eq!(decision.chosen, Resolution::R1Day);
        assert_eq!(decision.aggregate_loss_min, loss.loss);
    }

    #[test]
    fn an_expensive_margin_returns_to_the_aggregate() {
        let loss = chapter_16_post();
        let decision = decide(&loss, 100.0);
        assert_eq!(decision.chosen, Resolution::R0Aggregated);
        assert!(decision.aggregate_loss_min < decision.margin_cost_min);
    }

    #[test]
    fn the_evaluation_declares_the_steps_out_of_reach() {
        let loss = chapter_16_post();
        let evaluations = evaluate(&loss, 12.0);
        assert_eq!(evaluations.len(), 4);
        assert_eq!(evaluations[0].resolution, Resolution::R0Aggregated);
        assert_eq!(evaluations[1].total_min, loss.loss - 12.0);
        assert!(evaluations[2].out_of_reach.is_some());
        assert!(evaluations[3].out_of_reach.is_some());
        assert!(evaluations[2].loss_min.is_nan());
        assert!(evaluations[0].out_of_reach.is_none());
    }

    #[test]
    fn the_resolution_ladder_comes_from_the_core() {
        assert_eq!(
            Resolution::ALL.map(Resolution::code),
            ["R0-agregada", "R1-dia", "R2-hora", "R3-subpergunta"]
        );
    }
}
