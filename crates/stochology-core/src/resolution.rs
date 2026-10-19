//! Horizon R: resolution of the question.
//!
//! The empirical axis of the project (H0 to H3) says *where* the question is asked: seed, full
//! cycle, transfer between domains, live mesh. This module adds a second, orthogonal axis:
//! *how fine* the decision maker's question is, and what it costs to ask coarsely.
//!
//! The working hypothesis: with the same total resource, refining the question is not rhetoric —
//! it changes the number and it changes the action. The instrument below measures the loss of
//! resolving the same question at the aggregate.

use crate::rng::Rng;

/// Degree of resolution of the decision maker's question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resolution {
    /// "On average, does the service hold?" — the question asked to the average day.
    R0Aggregated,
    /// "Each day, does the service hold?" — the question asked to the distribution of days.
    R1Day,
    /// "Each hour, does the service hold?" — the question asked to the peak.
    R2Hour,
    /// "Each hour, and for whom?" — the question that brings the sub-question and the subgroup.
    R3Subquestion,
}

impl Resolution {
    pub const ALL: [Resolution; 4] = [
        Resolution::R0Aggregated,
        Resolution::R1Day,
        Resolution::R2Hour,
        Resolution::R3Subquestion,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Resolution::R0Aggregated => "R0-agregada",
            Resolution::R1Day => "R1-dia",
            Resolution::R2Hour => "R2-hora",
            Resolution::R3Subquestion => "R3-subpergunta",
        }
    }

    /// The finer the question, the larger the number. The order matters: comparing resolutions
    /// only makes sense when one refines the other.
    pub fn fineness(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolutionLoss {
    pub days: usize,
    pub arrival_low: f64,
    pub arrival_high: f64,
    pub aggregated_capacity: f64,
    pub aggregated_wait: f64,
    pub fine_wait: f64,
    pub loss: f64,
    pub above30_aggregated: f64,
    pub above30_fine: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct Measure {
    wait_sum: f64,
    above: usize,
    clients: usize,
}

impl Measure {
    fn add(&mut self, other: Measure) {
        self.wait_sum += other.wait_sum;
        self.above += other.above;
        self.clients += other.clients;
    }

    fn mean_wait(&self) -> f64 {
        if self.clients == 0 {
            0.0
        } else {
            self.wait_sum / self.clients as f64
        }
    }

    fn fraction_above30(&self) -> f64 {
        if self.clients == 0 {
            0.0
        } else {
            self.above as f64 / self.clients as f64
        }
    }
}

/// One day of queue, with the two arms paired: the same arrivals and the same service bases,
/// changing only the service rate. Without that the comparison would also measure the draw.
///
/// The day lasts `hours`, with Poisson arrivals at rate `lam` and the queue starting empty; each
/// client waits for what is left of the queue: W_1 = 0, W_{n+1} = max(0, W_n + S_n - gap_{n+1}).
/// Until 2026-10-19 the day had a fixed count of round(lam * hours) arrivals, and each client's wait
/// included its own service (the first client waited for a service that did not exist); a
/// reimplementation from the text found both. See `uniform_open`.
fn paired_day(
    rng: &mut Rng,
    hours: usize,
    lam: f64,
    mu_aggregated: f64,
    mu_fine: f64,
) -> (Measure, Measure) {
    let mut aggregated = Measure::default();
    let mut fine = Measure::default();
    let (mut aggregated_wait, mut fine_wait) = (0.0_f64, 0.0_f64);
    let mut previous_base: Option<f64> = None;
    let mut clock = 0.0_f64;
    loop {
        let gap = -uniform_open(rng).ln() / lam;
        clock += gap;
        if clock > hours as f64 {
            break;
        }
        if let Some(base) = previous_base {
            aggregated_wait = (aggregated_wait + base / mu_aggregated - gap).max(0.0);
            fine_wait = (fine_wait + base / mu_fine - gap).max(0.0);
        }
        previous_base = Some(-uniform_open(rng).ln());
        aggregated.wait_sum += aggregated_wait * 60.0;
        fine.wait_sum += fine_wait * 60.0;
        aggregated.clients += 1;
        fine.clients += 1;
        if aggregated_wait * 60.0 > 30.0 {
            aggregated.above += 1;
        }
        if fine_wait * 60.0 > 30.0 {
            fine.above += 1;
        }
    }
    (aggregated, fine)
}

/// A uniform draw on the open interval (0, 1), so that its logarithm is finite.
fn uniform_open(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 1.0) / ((1u64 << 53) as f64 + 1.0)
}

/// Days of the R-horizon laboratories. Until 2026-10-19 they used 200, and the printed waits carried a
/// seed spread of about 2 minutes against two printed decimals; 4,000 days bring it under half a minute.
pub const R_HORIZON_DAYS: usize = 4000;

/// The utilisation the staffing aims at: capacity = demand / 0.8. Declared on 2026-10-18; until
/// then the capacity equalled the demand (utilisation 1), a critical queue with no steady state,
/// where the measured waits were set by the length of the day rather than by the question.
pub const TARGET_UTILISATION: f64 = 0.8;

/// Measures the loss of resolving the same capacity question at the aggregate.
///
/// The two arms spend the **same total resource**: the aggregate uses the mean capacity every day;
/// the fine one uses the capacity of that day. Only the resolution of the question changes.
/// Both arms staff for the declared utilisation of 0.8 of the demand they plan for.
pub fn measure(
    seed: u64,
    days: usize,
    hours_per_day: usize,
    arrival_low: f64,
    arrival_high: f64,
) -> ResolutionLoss {
    let mean = (arrival_low + arrival_high) / 2.0 / TARGET_UTILISATION;
    let mut rng = Rng::new(seed);
    let mut aggregated = Measure::default();
    let mut fine = Measure::default();

    for day in 0..days {
        let busy = day % 2 == 1;
        let lam = if busy { arrival_high } else { arrival_low };
        let mu_fine = if busy { arrival_high } else { arrival_low } / TARGET_UTILISATION;
        let (a, f) = paired_day(&mut rng, hours_per_day, lam, mean, mu_fine);
        aggregated.add(a);
        fine.add(f);
    }

    ResolutionLoss {
        days,
        arrival_low,
        arrival_high,
        aggregated_capacity: mean,
        aggregated_wait: aggregated.mean_wait(),
        fine_wait: fine.mean_wait(),
        loss: aggregated.mean_wait() - fine.mean_wait(),
        above30_aggregated: aggregated.fraction_above30(),
        above30_fine: fine.fraction_above30(),
    }
}

// ---------------------------------------------------------------------------------------------
// Predictions 3 and 5 of the R axis
// ---------------------------------------------------------------------------------------------

/// The service target of the staffing: the Erlang C plan leaves at most 20% of the clients queued.
pub const SCALE_TARGET: f64 = 0.80;
/// The failed-day criterion, declared before running: the day fails when **more than 10%** of the
/// clients wait more than 30 min.
pub const FAILED_DAY_ABOVE30: f64 = 0.10;
/// The threshold of prediction 3, written in the document of the axis: more than **one quarter** of
/// failed days.
pub const DECLARED_FAILED_DAY_FRACTION: f64 = 0.25;

/// One simulated day with an integer number of agents, through the memoryless queue recursion.
///
/// Declared and left as it was measured: this day keeps the two defects that `paired_day` lost on
/// 2026-10-19 (a fixed count of round(lam * hours) arrivals, and each client's wait including its own
/// service), and models c agents as one line served at c clients per hour. Predictions 3 and 5 of the
/// R axis were declared and measured with it (`empirical/`); correcting it is a new specification,
/// to be declared before it is run and counted in K, not an edit of the old one.
fn day_with_agents(rng: &mut Rng, hours: usize, lam: f64, agents: u32) -> Measure {
    let mut measure = Measure::default();
    let mut wait = 0.0_f64;
    let mu = agents.max(1) as f64;
    for _ in 0..(lam * hours as f64).round() as usize {
        let gap = -uniform_open(rng).ln() / lam;
        let base = -uniform_open(rng).ln();
        wait = (wait + base / mu - gap).max(0.0);
        measure.wait_sum += wait * 60.0;
        measure.clients += 1;
        if wait * 60.0 > 30.0 {
            measure.above += 1;
        }
    }
    measure
}

/// One arrangement of oscillation: the amplitude, the R0 staffing and what happened across the days.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OscillationPoint {
    /// The weak day reaches `mean - amplitude` and the strong one `mean + amplitude`.
    pub amplitude: f64,
    pub days: usize,
    /// How many days the R0 plan (the staffing of the average day) failed.
    pub failed_days: usize,
    pub failed_day_fraction: f64,
    /// The R0 staffing, in agents, sized by the average day.
    pub agents_r0: u32,
    /// The resolution loss: mean wait of R0 minus that of R1, in minutes.
    pub loss: f64,
    pub above30_r0: f64,
    pub above30_r1: f64,
}

/// What prediction 3 returns.
#[derive(Debug, Clone, PartialEq)]
pub struct OscillationThreshold {
    pub points: Vec<OscillationPoint>,
    /// The smallest amplitude at which the R0 decision fails on more than one quarter of the days.
    pub threshold: Option<f64>,
    /// The resolution loss at that amplitude: the price, in minutes, of asking coarsely.
    pub loss_at_threshold: Option<f64>,
    /// Whether the fraction of failed days grows with the amplitude.
    pub monotone: bool,
}

/// Measures prediction 3: for each amplitude, how many days the decision taken at R0 fails, and
/// what refining costs. The two arms spend the same total resource: the R0 staffing is that of the
/// average day, the R1 staffing is that of the day, and the even day is strong and the odd one weak.
pub fn oscillation_threshold(
    seed: u64,
    days: usize,
    hours_per_day: usize,
    mean: f64,
    amplitudes: &[f64],
) -> OscillationThreshold {
    let mut rng = Rng::new(seed);
    let agents_r0 = crate::erlang::minimal_scale(mean, 1.0, SCALE_TARGET);
    let mut points = Vec::new();
    for &amplitude in amplitudes {
        let mut failed = 0usize;
        let mut r0 = Measure::default();
        let mut r1 = Measure::default();
        for day in 0..days {
            let strong = day % 2 == 1;
            let lam = if strong {
                mean + amplitude
            } else {
                (mean - amplitude).max(0.0)
            };
            let a = day_with_agents(&mut rng, hours_per_day, lam, agents_r0);
            let agents_r1 = crate::erlang::minimal_scale(lam.max(1e-9), 1.0, SCALE_TARGET);
            let f = day_with_agents(&mut rng, hours_per_day, lam, agents_r1);
            if a.fraction_above30() > FAILED_DAY_ABOVE30 {
                failed += 1;
            }
            r0.add(a);
            r1.add(f);
        }
        points.push(OscillationPoint {
            amplitude,
            days,
            failed_days: failed,
            failed_day_fraction: failed as f64 / days.max(1) as f64,
            agents_r0,
            loss: r0.mean_wait() - r1.mean_wait(),
            above30_r0: r0.fraction_above30(),
            above30_r1: r1.fraction_above30(),
        });
    }
    let crossing = points
        .iter()
        .find(|p| p.failed_day_fraction > DECLARED_FAILED_DAY_FRACTION);
    OscillationThreshold {
        threshold: crossing.map(|p| p.amplitude),
        loss_at_threshold: crossing.map(|p| p.loss),
        monotone: points
            .windows(2)
            .all(|pair| pair[1].failed_day_fraction >= pair[0].failed_day_fraction - 1e-12),
        points,
    }
}

/// The **calibrated** R0 staffing: the smallest number of agents at which the flat demand of the
/// average day makes at most `target_failed_days` of the days fail. It is the second specification
/// of the design — the first one, the Erlang C plan of the 80% target, already left 1 day in 5
/// failing with flat demand. The calibration entered as a new specification, declared and counted
/// in K, and not as a fix.
pub fn calibrated_scale(
    seed: u64,
    days: usize,
    hours_per_day: usize,
    mean: f64,
    target_failed_days: f64,
) -> u32 {
    let initial = crate::erlang::minimal_scale(mean, 1.0, SCALE_TARGET).max(1);
    for agents in initial..(initial + 40) {
        let mut rng = Rng::new(seed);
        let mut failed = 0usize;
        for _ in 0..days {
            let m = day_with_agents(&mut rng, hours_per_day, mean, agents);
            if m.fraction_above30() > FAILED_DAY_ABOVE30 {
                failed += 1;
            }
        }
        if (failed as f64 / days.max(1) as f64) <= target_failed_days {
            return agents;
        }
    }
    initial + 40
}

/// Prediction 3 measured with a given R0 plan (in agents) — it is what allows comparing the two
/// specifications of the design side by side.
pub fn oscillation_threshold_with_plan(
    seed: u64,
    days: usize,
    hours_per_day: usize,
    mean: f64,
    amplitudes: &[f64],
    agents_r0: u32,
) -> OscillationThreshold {
    let mut rng = Rng::new(seed);
    let mut points = Vec::new();
    for &amplitude in amplitudes {
        let mut failed = 0usize;
        let mut r0 = Measure::default();
        let mut r1 = Measure::default();
        for day in 0..days {
            let strong = day % 2 == 1;
            let lam = if strong {
                mean + amplitude
            } else {
                (mean - amplitude).max(0.0)
            };
            let a = day_with_agents(&mut rng, hours_per_day, lam, agents_r0);
            let agents_r1 = crate::erlang::minimal_scale(lam.max(1e-9), 1.0, SCALE_TARGET);
            let f = day_with_agents(&mut rng, hours_per_day, lam, agents_r1);
            if a.fraction_above30() > FAILED_DAY_ABOVE30 {
                failed += 1;
            }
            r0.add(a);
            r1.add(f);
        }
        points.push(OscillationPoint {
            amplitude,
            days,
            failed_days: failed,
            failed_day_fraction: failed as f64 / days.max(1) as f64,
            agents_r0,
            loss: r0.mean_wait() - r1.mean_wait(),
            above30_r0: r0.fraction_above30(),
            above30_r1: r1.fraction_above30(),
        });
    }
    let crossing = points
        .iter()
        .find(|p| p.failed_day_fraction > DECLARED_FAILED_DAY_FRACTION);
    OscillationThreshold {
        threshold: crossing.map(|p| p.amplitude),
        loss_at_threshold: crossing.map(|p| p.loss),
        monotone: points
            .windows(2)
            .all(|pair| pair[1].failed_day_fraction >= pair[0].failed_day_fraction - 1e-12),
        points,
    }
}

/// The check of prediction 5 over the daily fractions.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplicationCheck {
    /// Every day passes and the aggregate does not: prediction 5 would be falsified here.
    pub violations: usize,
    /// The aggregate passes and some day does not: proof that the converse of the prediction fails.
    pub counterexamples: usize,
    pub days: usize,
    pub aggregated: f64,
    /// The aggregate rebuilt as the weighted mean of the days: it is the conservation arithmetic.
    pub aggregated_by_conservation: f64,
    pub identity_checked: bool,
}

/// Checks the conservation of the aggregate: the sum of the days has to rebuild the measured aggregate.
pub fn aggregated_conservation(per_day: &[(usize, usize)], aggregated: (usize, usize)) -> bool {
    let above: usize = per_day.iter().map(|(a, _)| *a).sum();
    let clients: usize = per_day.iter().map(|(_, n)| *n).sum();
    above == aggregated.0 && clients == aggregated.1
}

/// Checks the implication of prediction 5 over a set of days: each day with (above, clients).
pub fn check_implication(days: &[(usize, usize)], threshold: f64) -> ImplicationCheck {
    let clients: usize = days.iter().map(|(_, n)| *n).sum();
    let above: usize = days.iter().map(|(a, _)| *a).sum();
    let aggregated = if clients == 0 {
        0.0
    } else {
        above as f64 / clients as f64
    };
    let weighted = if clients == 0 {
        0.0
    } else {
        days.iter()
            .map(|(a, n)| {
                if *n == 0 {
                    0.0
                } else {
                    *a as f64 / *n as f64 * *n as f64
                }
            })
            .sum::<f64>()
            / clients as f64
    };
    let all_ok = days
        .iter()
        .all(|(a, n)| *n == 0 || (*a as f64 / *n as f64) <= threshold);
    let any_failed = days
        .iter()
        .any(|(a, n)| *n > 0 && (*a as f64 / *n as f64) > threshold);
    let mut violations = 0usize;
    let mut counterexamples = 0usize;
    if all_ok && aggregated > threshold {
        violations += 1;
    }
    if any_failed && aggregated <= threshold {
        counterexamples += 1;
    }
    ImplicationCheck {
        violations,
        counterexamples,
        days: days.len(),
        aggregated,
        aggregated_by_conservation: weighted,
        identity_checked: (aggregated - weighted).abs() < 1e-12,
    }
}

/// What prediction 5 returns over a set of demand profiles.
#[derive(Debug, Clone, PartialEq)]
pub struct Transfer {
    pub profiles: usize,
    /// Profiles in which R1 survived (no failed day) and R0 did not: the prediction would be falsified here.
    pub violations: usize,
    /// Profiles in which R0 survived and R1 did not: the converse of the prediction.
    pub counterexamples: usize,
    /// Whether the sum of the days rebuilt the aggregate in every profile.
    pub conservation_checked: bool,
    pub threshold: f64,
}

/// Measures prediction 5: the same decision taken at R1 (each day) and at R0 (the aggregate).
pub fn transfer(seed: u64, days: usize, hours_per_day: usize, profiles: &[(f64, f64)]) -> Transfer {
    let mut rng = Rng::new(seed);
    let mut violations = 0usize;
    let mut counterexamples = 0usize;
    let mut conservation = true;
    for &(mean, amplitude) in profiles {
        let agents_r0 = crate::erlang::minimal_scale(mean, 1.0, SCALE_TARGET);
        let mut per_day: Vec<(usize, usize)> = Vec::new();
        let mut r0 = Measure::default();
        for day in 0..days {
            let strong = day % 2 == 1;
            let lam = if strong {
                mean + amplitude
            } else {
                (mean - amplitude).max(0.0)
            };
            let a = day_with_agents(&mut rng, hours_per_day, lam, agents_r0);
            per_day.push((a.above, a.clients));
            r0.add(a);
        }
        let check = check_implication(&per_day, FAILED_DAY_ABOVE30);
        violations += check.violations;
        counterexamples += check.counterexamples;
        if !aggregated_conservation(&per_day, (r0.above, r0.clients)) || !check.identity_checked {
            conservation = false;
        }
    }
    Transfer {
        profiles: profiles.len(),
        violations,
        counterexamples,
        conservation_checked: conservation,
        threshold: FAILED_DAY_ABOVE30,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_order_of_resolutions_is_the_order_of_fineness() {
        let fineness: Vec<u8> = Resolution::ALL.iter().map(|r| r.fineness()).collect();
        assert_eq!(fineness, vec![0, 1, 2, 3]);
        assert!(Resolution::R2Hour > Resolution::R1Day);
    }

    #[test]
    fn with_the_same_resource_the_fine_question_waits_less() {
        let p = measure(1909, 200, 10, 5.0, 11.0);
        assert!(
            p.fine_wait < p.aggregated_wait,
            "fine {:.2} against aggregated {:.2}",
            p.fine_wait,
            p.aggregated_wait
        );
        assert!(p.loss > 0.0);
        assert!(
            p.above30_fine < p.above30_aggregated,
            "above 30 min: fine {:.3} against aggregated {:.3}",
            p.above30_fine,
            p.above30_aggregated
        );
    }

    #[test]
    fn without_variation_there_is_no_resolution_loss() {
        let p = measure(1909, 100, 10, 8.0, 8.0);
        assert!(p.loss.abs() < 1e-9, "loss {:.6}", p.loss);
    }

    // ------------------------------------------------------- predictions 3 and 5 of the R axis

    #[test]
    fn with_the_calibrated_plan_the_threshold_exists_and_the_fraction_grows() {
        let amplitudes = [0.0, 1.0, 2.0, 3.0];
        let agents = calibrated_scale(1909, 200, 10, 8.0, 0.10);
        let l = oscillation_threshold_with_plan(1909, 200, 10, 8.0, &amplitudes, agents);
        assert!(
            l.monotone,
            "the fraction has to grow with the amplitude: {:?}",
            l.points
        );
        assert!(
            l.points[0].failed_day_fraction <= 0.10,
            "the plan is calibrated on flat demand: {:.3}",
            l.points[0].failed_day_fraction
        );
        let last = l.points.last().expect("last point");
        assert!(
            last.failed_day_fraction > DECLARED_FAILED_DAY_FRACTION,
            "with amplitude {:.1} the failure passes one quarter: {:.3}",
            last.amplitude,
            last.failed_day_fraction
        );
        assert!(
            l.threshold.is_some(),
            "the point exists and the instrument finds it"
        );
        assert!(
            l.threshold.expect("threshold") > 0.0,
            "the point is not zero"
        );
    }

    #[test]
    fn with_the_erlang_plan_prediction_3_does_not_hold() {
        // The first specification of the design: the Erlang C staffing of the average day. It fails
        // on one fifth of the days already with flat demand, so the point marks nothing — and the
        // fraction is not monotone on the grid. It is the proof that the instrument knows how to
        // reject the prediction.
        let amplitudes = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let l = oscillation_threshold(1909, 200, 10, 8.0, &amplitudes);
        assert!(
            l.points[0].failed_day_fraction > 0.15,
            "with flat demand the Erlang staffing already fails one day in five: {:.3}",
            l.points[0].failed_day_fraction
        );
        assert!(
            !l.monotone,
            "and the fraction does not grow monotonically in this design"
        );
        let loss_at_threshold = l
            .loss_at_threshold
            .expect("the crossing exists on the grid");
        assert!(
            loss_at_threshold < 0.0,
            "and at the crossing refining costs, instead of paying: {loss_at_threshold:.2} min"
        );
    }

    #[test]
    fn below_the_threshold_refining_buys_less_than_above() {
        let amplitudes = [0.5, 1.5, 2.5, 3.5];
        let l = oscillation_threshold(1909, 40, 10, 8.0, &amplitudes);
        let loss_below = l.points[0].loss;
        let loss_above = l.points[3].loss;
        assert!(
            loss_above > loss_below,
            "the loss grows with the oscillation: {loss_above:.2} against {loss_below:.2}"
        );
    }

    #[test]
    fn the_transfer_has_no_violation_and_has_a_counterexample() {
        let profiles = [(8.0, 0.5), (8.0, 1.0), (8.0, 2.0), (8.0, 3.0)];
        let t = transfer(1909, 40, 10, &profiles);
        assert_eq!(t.violations, 0, "a result that survives R1 survives R0");
        assert!(
            t.counterexamples > 0,
            "and the converse does not hold: {:?}",
            t
        );
        assert!(
            t.conservation_checked,
            "the sum of the days rebuilds the aggregate"
        );
    }

    #[test]
    fn the_conservation_fails_an_aggregate_that_does_not_add_up() {
        let per_day = vec![(5usize, 100usize), (4, 100), (3, 100)];
        assert!(aggregated_conservation(&per_day, (12, 300)));
        assert!(!aggregated_conservation(&per_day, (12, 299)), "one n short");
        assert!(
            !aggregated_conservation(&per_day, (11, 300)),
            "one above short"
        );
    }

    #[test]
    fn the_implication_check_finds_the_counterexample_and_not_the_violation() {
        let days = vec![(1usize, 100usize), (30, 100), (5, 100), (2, 100)];
        let c = check_implication(&days, FAILED_DAY_ABOVE30);
        assert_eq!(c.violations, 0);
        assert_eq!(c.counterexamples, 1);
        assert!(
            (c.aggregated - 0.095).abs() < 1e-12,
            "aggregated {:.4}",
            c.aggregated
        );
        assert!(c.identity_checked);
        let clean = vec![(5usize, 100usize), (6, 100)];
        let c2 = check_implication(&clean, FAILED_DAY_ABOVE30);
        assert_eq!(c2.violations, 0);
        assert_eq!(c2.counterexamples, 0);
    }
}
