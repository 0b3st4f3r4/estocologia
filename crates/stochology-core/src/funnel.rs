//! Funnel of four filters, EVSI pruning and the K attempt counter.
//!
//! The book describes the expensive funnel in four steps (value of information, topological and
//! unit pruning, severe test against the null, heavy inference and panel). This module implements
//! the *cheap* step, which cuts the card before any heavy simulation:
//!
//! 1. the **four header filters** (F1 to F4), each one with the explicit reason for the refusal;
//! 2. the **EVSI pruning**: the exam only pays for itself if the value of sample information can
//!    cover its cost. Since no exam is worth more than knowing the state, the upper bound of the
//!    EVSI is the EVPI, which depends only on the cost/loss ratio C/L and on the probability of the
//!    state;
//! 3. the **K attempt counter**: how many specifications were tried for the same question, by gate
//!    and in the order they were tried.
//!
//! Nothing here uses a clock, system entropy or global state: the only randomness is the `Rng`
//! with an explicit seed, and only in the test that checks the bound by sampling.
//!
//! **Declared limits.** The model of the pruning has two acts and two states, with 0 < C < L; the
//! bound is exact for that model, and not for problems with more states or more acts. The funnel
//! cuts the card by the header, not by the data: passing the four filters is a necessary condition,
//! not a sufficient one — the invariants Phi1-Phi5 and the panel are charged afterwards. The funnel
//! cuts early so as not to spend heavily on what does not even have a header.

use crate::layer::Card;

// --------------------------------------------------------------------- filters

/// The four filters of the funnel, in the order in which they cut the card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Filter {
    /// (F1) there is no decision maker question.
    DecisionMakerQuestion,
    /// (F2) there is no declared generative process.
    GenerativeProcess,
    /// (F3) there is no independent unit or null model.
    UnitOrNull,
    /// (F4) there is no declared action with an exit clause.
    ActionWithExit,
}

impl Filter {
    /// The four gates, in the canonical order of the funnel.
    pub const ALL: [Filter; 4] = [
        Filter::DecisionMakerQuestion,
        Filter::GenerativeProcess,
        Filter::UnitOrNull,
        Filter::ActionWithExit,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Filter::DecisionMakerQuestion => "F1",
            Filter::GenerativeProcess => "F2",
            Filter::UnitOrNull => "F3",
            Filter::ActionWithExit => "F4",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Filter::DecisionMakerQuestion => "decision maker question",
            Filter::GenerativeProcess => "declared generative process",
            Filter::UnitOrNull => "independent unit and null model",
            Filter::ActionWithExit => "declared action with exit clause",
        }
    }

    /// Position in the funnel, from 0 to 3: it is the key of the count by gate.
    pub fn index(self) -> usize {
        match self {
            Filter::DecisionMakerQuestion => 0,
            Filter::GenerativeProcess => 1,
            Filter::UnitOrNull => 2,
            Filter::ActionWithExit => 3,
        }
    }
}

/// The refusal of a card: which filter blocked it and the explicit reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub filter: Filter,
    pub reason: String,
}

impl Refusal {
    pub fn new(filter: Filter, reason: impl Into<String>) -> Self {
        Refusal {
            filter,
            reason: reason.into(),
        }
    }

    /// A readable line, in the format of the funnel report.
    pub fn line(&self) -> String {
        format!(
            "{} {}: refused by {}",
            self.filter.code(),
            self.filter.name(),
            self.reason
        )
    }
}

fn is_empty(text: Option<&str>) -> bool {
    text.map(str::trim).unwrap_or("").is_empty()
}

fn missing_decision_maker_question(f: &Card) -> Option<Refusal> {
    if f.decision_maker_question.trim().is_empty() {
        Some(Refusal::new(
            Filter::DecisionMakerQuestion,
            "no decision maker question: the card does not say what is decided",
        ))
    } else {
        None
    }
}

fn missing_generative_process(f: &Card) -> Option<Refusal> {
    let mut missing: Vec<&str> = Vec::new();
    if is_empty(f.simulable_artifact.as_deref()) {
        missing.push("simulable artifact");
    }
    if f.seed.is_none() {
        missing.push("fixed seed");
    }
    if missing.is_empty() {
        return None;
    }
    Some(Refusal::new(
        Filter::GenerativeProcess,
        format!(
            "no declared generative process: missing {}",
            missing.join(" and ")
        ),
    ))
}

fn missing_unit_or_null(f: &Card) -> Option<Refusal> {
    let mut missing: Vec<&str> = Vec::new();
    if is_empty(f.independent_unit.as_deref()) {
        missing.push("independent unit");
    }
    if is_empty(f.null_model.as_deref()) {
        missing.push("null model");
    }
    if missing.is_empty() {
        return None;
    }
    Some(Refusal::new(
        Filter::UnitOrNull,
        format!(
            "no independent unit or null model: missing {}",
            missing.join(" and ")
        ),
    ))
}

fn missing_action_with_exit(f: &Card) -> Option<Refusal> {
    let mut missing: Vec<&str> = Vec::new();
    if f.package_action.trim().is_empty() {
        missing.push("declared action");
    }
    if is_empty(f.exit_clause.as_deref()) {
        missing.push("exit clause");
    }
    if missing.is_empty() {
        return None;
    }
    Some(Refusal::new(
        Filter::ActionWithExit,
        format!(
            "no declared action with exit clause: missing {}",
            missing.join(" and ")
        ),
    ))
}

fn evaluate_filter(filter: Filter, f: &Card) -> Option<Refusal> {
    match filter {
        Filter::DecisionMakerQuestion => missing_decision_maker_question(f),
        Filter::GenerativeProcess => missing_generative_process(f),
        Filter::UnitOrNull => missing_unit_or_null(f),
        Filter::ActionWithExit => missing_action_with_exit(f),
    }
}

/// Runs the card through the four filters, in order, and stops at the first one that refuses it.
pub fn filter_card(f: &Card) -> Result<(), Refusal> {
    for filter in Filter::ALL {
        if let Some(refusal) = evaluate_filter(filter, f) {
            return Err(refusal);
        }
    }
    Ok(())
}

/// All the refusals of the card, one per gate — what each filter has to say about it.
pub fn refusals(f: &Card) -> Vec<Refusal> {
    Filter::ALL
        .iter()
        .filter_map(|filter| evaluate_filter(*filter, f))
        .collect()
}

// --------------------------------------------------- pruning by the value of information

/// The declared act in the two-act problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Act {
    Act,
    NotAct,
}

impl Act {
    pub fn code(self) -> &'static str {
        match self {
            Act::Act => "act",
            Act::NotAct => "not-act",
        }
    }
}

/// Cost/loss ratio of the act: `cost` is what is spent acting (including on those who did not
/// need it) and `loss` is what is lost by letting the case pass.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostLoss {
    pub cost: f64,
    pub loss: f64,
}

impl CostLoss {
    /// Requires 0 < cost < loss: if the cost reaches the loss, acting never pays off and there is
    /// no decision to inform — the degenerate case stays outside the contract.
    pub fn new(cost: f64, loss: f64) -> Self {
        assert!(
            cost.is_finite() && cost > 0.0,
            "cost has to be positive and finite, got {cost}"
        );
        assert!(
            loss.is_finite() && loss > cost,
            "loss has to exceed the cost ({cost}), got {loss}"
        );
        CostLoss { cost, loss }
    }

    pub fn ratio(&self) -> f64 {
        self.cost / self.loss
    }

    /// Threshold of the act: p* = C/L. Below it the best act is not to act; from it on, to act.
    pub fn threshold(&self) -> f64 {
        self.ratio()
    }

    /// The best act before looking at any data. At the threshold the two tie and the function
    /// chooses to act.
    pub fn best_act(&self, p: f64) -> Act {
        if p.clamp(0.0, 1.0) >= self.threshold() {
            Act::Act
        } else {
            Act::NotAct
        }
    }

    /// The expected value of perfect information: what is paid to know the state before acting.
    /// Tent function: it rises with p up to the threshold C/L and falls after it.
    pub fn evpi(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        if p <= self.threshold() {
            p * (self.loss - self.cost)
        } else {
            self.cost * (1.0 - p)
        }
    }

    /// Upper bound of the expected value of sample information (EVSI) of any exam: no imperfect
    /// exam is worth more than knowing the state, hence EVSI <= EVPI.
    pub fn evsi_upper_bound(&self, p: f64) -> f64 {
        self.evpi(p)
    }

    /// EVSI of an exam with declared sensitivity and specificity, computed exactly in the two-act
    /// two-state model.
    pub fn evsi(&self, p: f64, sensitivity: f64, specificity: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        let se = sensitivity.clamp(0.0, 1.0);
        let sp = specificity.clamp(0.0, 1.0);
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        let prob_positive = se * p + (1.0 - sp) * (1.0 - p);
        let prob_negative = sp * (1.0 - p) + (1.0 - se) * p;
        let ppv = if prob_positive > 0.0 {
            se * p / prob_positive
        } else {
            0.0
        };
        let npv = if prob_negative > 0.0 {
            sp * (1.0 - p) / prob_negative
        } else {
            0.0
        };
        let posterior_negative = 1.0 - npv;
        // the optimal act of each result: acting costs C, not acting loses L with the posterior of the state
        let gain_positive = (-self.cost).max(-ppv * self.loss);
        let gain_negative = (-self.cost).max(-posterior_negative * self.loss);
        let with_exam = prob_positive * gain_positive + prob_negative * gain_negative;
        let without_information = (-self.cost).max(-p * self.loss);
        (with_exam - without_information).max(0.0)
    }

    /// Prunes the exam when the EVSI upper bound does not cross the threshold (the threshold is the
    /// cost of the exam, in the same unit as C and L).
    pub fn prune(&self, p: f64, threshold: f64) -> Pruning {
        assert!(
            threshold.is_finite() && threshold >= 0.0,
            "the threshold has to be non-negative and finite, got {threshold}"
        );
        let p = p.clamp(0.0, 1.0);
        let bound = self.evsi_upper_bound(p);
        let (pruned, reason) = if bound <= 0.0 {
            (true, Some(PruneReason::NoUncertainty))
        } else if bound <= threshold {
            (true, Some(PruneReason::BoundBelowThreshold))
        } else {
            (false, None)
        };
        Pruning {
            cost_loss_ratio: self.ratio(),
            probability: p,
            upper_bound: bound,
            threshold,
            decision_without_information: self.best_act(p),
            pruned,
            reason,
        }
    }
}

/// Why the exam was pruned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PruneReason {
    /// There is no uncertainty to resolve: the probability is at the extreme and the EVPI is zero.
    NoUncertainty,
    /// The upper bound of the sample information does not cover the cost of the exam.
    BoundBelowThreshold,
}

impl PruneReason {
    pub fn code(self) -> &'static str {
        match self {
            PruneReason::NoUncertainty => "no-uncertainty",
            PruneReason::BoundBelowThreshold => "bound-below-threshold",
        }
    }

    pub fn text(self) -> &'static str {
        match self {
            PruneReason::NoUncertainty => {
                "there is no uncertainty to resolve: the probability is at the extreme and the EVPI is zero"
            }
            PruneReason::BoundBelowThreshold => {
                "the EVSI upper bound does not cover the threshold of the exam; no exam can cross it"
            }
        }
    }
}

/// The verdict of the pruning, with the reason recorded.
#[derive(Debug, Clone, PartialEq)]
pub struct Pruning {
    pub cost_loss_ratio: f64,
    pub probability: f64,
    pub upper_bound: f64,
    pub threshold: f64,
    pub decision_without_information: Act,
    pub pruned: bool,
    pub reason: Option<PruneReason>,
}

impl Pruning {
    /// A readable line with every number of the decision.
    pub fn line(&self) -> String {
        let verdict = match self.reason {
            Some(m) => format!("pruned by {} ({})", m.code(), m.text()),
            None => "goes on to the exam".to_string(),
        };
        format!(
            "C/L ratio = {:.4} | p = {:.4} | EVSI upper bound = {:.4} | threshold = {:.4} | act without information = {} | {}",
            self.cost_loss_ratio,
            self.probability,
            self.upper_bound,
            self.threshold,
            self.decision_without_information.code(),
            verdict
        )
    }
}

// ------------------------- optimal stop (tau* = 0) and dominance invariant in p

/// Local action policy before an unidentifiable latent parameter p in [0, 1].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalPolicy {
    /// Realistic engagement (a_R): collects local value V_L and preserves coupling to V_I.
    Realistic,
    /// Rumination / search without information (a_U): pays cost c_R and reduces coupling by alpha in [0, 1).
    Rumination,
    /// Paralysis / avoidance (a_D): pays the fixed avoidance cost c_E.
    Paralysis,
}

impl LocalPolicy {
    pub fn code(self) -> &'static str {
        match self {
            LocalPolicy::Realistic => "realistic",
            LocalPolicy::Rumination => "rumination",
            LocalPolicy::Paralysis => "paralysis",
        }
    }
}

/// Payoff matrix with separable utility U(a, p) over the hidden variable p in [0, 1].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DominanceMatrix {
    pub intrinsic_value: f64,
    pub local_value: f64,
    pub rumination_cost: f64,
    pub avoidance_cost: f64,
    pub rumination_coupling: f64,
}

impl DominanceMatrix {
    /// Requires V_I > 0, V_L > 0, c_R > 0, c_E > 0 and 0 <= alpha < 1.
    pub fn new(
        intrinsic_value: f64,
        local_value: f64,
        rumination_cost: f64,
        avoidance_cost: f64,
        rumination_coupling: f64,
    ) -> Self {
        assert!(
            intrinsic_value.is_finite() && intrinsic_value > 0.0,
            "intrinsic_value has to be positive and finite, got {intrinsic_value}"
        );
        assert!(
            local_value.is_finite() && local_value > 0.0,
            "local_value has to be positive and finite, got {local_value}"
        );
        assert!(
            rumination_cost.is_finite() && rumination_cost > 0.0,
            "rumination_cost has to be positive and finite, got {rumination_cost}"
        );
        assert!(
            avoidance_cost.is_finite() && avoidance_cost > 0.0,
            "avoidance_cost has to be positive and finite, got {avoidance_cost}"
        );
        assert!(
            rumination_coupling.is_finite() && (0.0..1.0).contains(&rumination_coupling),
            "rumination_coupling has to be in [0, 1), got {rumination_coupling}"
        );
        DominanceMatrix {
            intrinsic_value,
            local_value,
            rumination_cost,
            avoidance_cost,
            rumination_coupling,
        }
    }

    /// Expected utility E[U(a)] for p in [0, 1].
    pub fn expected_utility(&self, policy: LocalPolicy, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        match policy {
            LocalPolicy::Realistic => p * self.intrinsic_value + self.local_value,
            LocalPolicy::Rumination => {
                p * self.rumination_coupling * self.intrinsic_value - self.rumination_cost
            }
            LocalPolicy::Paralysis => -self.avoidance_cost,
        }
    }

    /// Net margin E[U(a_R)] - E[U(a_U)] = p(1 - alpha)V_I + V_L + c_R.
    pub fn margin_over_rumination(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        p * (1.0 - self.rumination_coupling) * self.intrinsic_value
            + self.local_value
            + self.rumination_cost
    }

    /// Net margin E[U(a_R)] - E[U(a_D)] = p V_I + V_L + c_E.
    pub fn margin_over_paralysis(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        p * self.intrinsic_value + self.local_value + self.avoidance_cost
    }

    /// Policy that maximises the expected utility at p.
    pub fn best_policy(&self, p: f64) -> LocalPolicy {
        let u_r = self.expected_utility(LocalPolicy::Realistic, p);
        let u_u = self.expected_utility(LocalPolicy::Rumination, p);
        let u_d = self.expected_utility(LocalPolicy::Paralysis, p);
        if u_r >= u_u && u_r >= u_d {
            LocalPolicy::Realistic
        } else if u_u >= u_d {
            LocalPolicy::Rumination
        } else {
            LocalPolicy::Paralysis
        }
    }

    /// Checks whether the optimal policy is invariant over the whole interval p in [0, 1] (da*/dp = 0).
    pub fn invariant_in_p(&self) -> bool {
        self.margin_over_rumination(0.0) > 0.0
            && self.margin_over_rumination(1.0) > 0.0
            && self.margin_over_paralysis(0.0) > 0.0
            && self.margin_over_paralysis(1.0) > 0.0
            && self.best_policy(0.0) == LocalPolicy::Realistic
            && self.best_policy(1.0) == LocalPolicy::Realistic
    }

    /// Bellman stopping rule (VOC = EVSI - c_R): when the channel brings no information
    /// (se + sp == 1 => Delta I_t = 0 => EVSI = 0) or does not pay the step, tau* = 0.
    pub fn optimal_stop(
        &self,
        cp: &CostLoss,
        p: f64,
        sensitivity: f64,
        specificity: f64,
    ) -> OptimalStop {
        let p = p.clamp(0.0, 1.0);
        let se = sensitivity.clamp(0.0, 1.0);
        let sp = specificity.clamp(0.0, 1.0);
        let channel_without_information = (se + sp - 1.0).abs() <= 1e-12;
        let evsi = if channel_without_information {
            0.0
        } else {
            cp.evsi(p, se, sp)
        };
        let voc = evsi - self.rumination_cost;
        let tau_star = if voc <= 0.0 { 0 } else { 1 };
        OptimalStop {
            probability: p,
            evsi,
            step_cost: self.rumination_cost,
            voc,
            channel_without_information,
            tau_star,
            dominant_policy: self.best_policy(p),
        }
    }
}

/// Result of the Bellman cut over one step of internal deliberation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptimalStop {
    pub probability: f64,
    pub evsi: f64,
    pub step_cost: f64,
    pub voc: f64,
    pub channel_without_information: bool,
    pub tau_star: usize,
    pub dominant_policy: LocalPolicy,
}

impl OptimalStop {
    pub fn line(&self) -> String {
        format!(
            "p = {:.4} | EVSI = {:.4} | c_R = {:.4} | VOC = {:.4} | without information = {} | tau* = {} | policy = {}",
            self.probability,
            self.evsi,
            self.step_cost,
            self.voc,
            self.channel_without_information,
            self.tau_star,
            self.dominant_policy.code()
        )
    }
}

// ---------------------------------------------------- K attempt counter

/// One attempted specification, with the gate where it stopped (or none, if it passed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attempt {
    pub number: usize,
    pub refusal: Option<Filter>,
    pub label: String,
}

impl Attempt {
    pub fn passed(&self) -> bool {
        self.refusal.is_none()
    }
}

/// The honest scoreboard of the search: how many specifications were tried, by gate, in order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CounterK {
    attempts: Vec<Attempt>,
    by_filter: [usize; 4],
}

impl CounterK {
    pub fn new() -> Self {
        CounterK {
            attempts: Vec::new(),
            by_filter: [0; 4],
        }
    }

    /// Records an attempt and returns its number in the sequence.
    pub fn record(&mut self, label: &str, refusal: Option<Filter>) -> usize {
        let number = self.attempts.len() + 1;
        if let Some(filter) = refusal {
            self.by_filter[filter.index()] += 1;
        }
        self.attempts.push(Attempt {
            number,
            refusal,
            label: label.to_string(),
        });
        number
    }

    /// Shortcut for the attempt blocked by a gate.
    pub fn refused(&mut self, filter: Filter, label: &str) -> usize {
        self.record(label, Some(filter))
    }

    /// Shortcut for the attempt that crossed the funnel.
    pub fn approved(&mut self, label: &str) -> usize {
        self.record(label, None)
    }

    pub fn total(&self) -> usize {
        self.attempts.len()
    }

    pub fn approved_count(&self) -> usize {
        self.attempts.iter().filter(|t| t.passed()).count()
    }

    pub fn by_filter(&self, filter: Filter) -> usize {
        self.by_filter[filter.index()]
    }

    pub fn sequence(&self) -> &[Attempt] {
        &self.attempts
    }

    /// Closes the account: the report declares the K and the accepted final version.
    pub fn report(&self, final_version: Option<&str>) -> Report {
        Report {
            total: self.total(),
            by_filter: self.by_filter,
            approved: self.approved_count(),
            sequence: self.attempts.clone(),
            final_version: final_version.map(str::to_string),
        }
    }
}

/// What the counter returns: the total, the count by gate, the sequence and the final version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub total: usize,
    /// Blocked by filter, in the order F1 to F4.
    pub by_filter: [usize; 4],
    pub approved: usize,
    pub sequence: Vec<Attempt>,
    pub final_version: Option<String>,
}

impl Report {
    /// A readable line, in the format K = ... .
    pub fn line(&self) -> String {
        let by_gate: Vec<String> = Filter::ALL
            .iter()
            .map(|f| format!("{} {}", f.code(), self.by_filter[f.index()]))
            .collect();
        let version = self.final_version.as_deref().unwrap_or("not declared");
        format!(
            "K = {} attempts ({}); approved {}; final version: {}",
            self.total,
            by_gate.join(", "),
            self.approved,
            version
        )
    }
}

// ------------------------------------------------------------------- the funnel

/// The whole funnel: the four filters and the K counter in the same place.
#[derive(Debug, Clone, Default)]
pub struct Funnel {
    counter: CounterK,
}

impl Funnel {
    pub fn new() -> Self {
        Funnel {
            counter: CounterK::new(),
        }
    }

    /// Tries the card: returns the refusal and counts the attempt at the gate that blocked it — or
    /// counts it as approved when the four filters let it through.
    pub fn attempt(&mut self, label: &str, f: &Card) -> Result<(), Refusal> {
        match filter_card(f) {
            Ok(()) => {
                self.counter.approved(label);
                Ok(())
            }
            Err(refusal) => {
                self.counter.refused(refusal.filter, label);
                Err(refusal)
            }
        }
    }

    pub fn counter(&self) -> &CounterK {
        &self.counter
    }

    /// Closes the search by declaring the final version.
    pub fn close(&self, final_version: &str) -> Report {
        self.counter.report(Some(final_version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layer::{Card, Layer, UncertaintyType};
    use crate::rng::Rng;

    /// The complete card: it passes the four filters and also the invariants Phi1-Phi4.
    fn complete_card() -> Card {
        let mut f = Card::new("funnel.1", Layer::Model, UncertaintyType::Epistemic).complete();
        f.definition = "peak-day service with the contingency plan".to_string();
        f.decision_maker_question = "should we trigger the contingency plan today?".to_string();
        f.package_action = "trigger the contingency plan".to_string();
        f
    }

    #[test]
    fn a_complete_card_passes_the_four_filters() {
        let f = complete_card();
        assert!(filter_card(&f).is_ok(), "a complete card should pass");
        assert!(refusals(&f).is_empty());
        // the header that the four filters read is whole in the card
        assert!(!f.decision_maker_question.trim().is_empty());
        assert!(!f.package_action.trim().is_empty());
        assert!(f.simulable_artifact.is_some() && f.seed.is_some());
        assert!(f.independent_unit.is_some() && f.null_model.is_some());
        assert!(f.exit_clause.is_some());
    }

    #[test]
    fn f1_refuses_a_card_without_a_decision_maker_question() {
        let mut f = complete_card();
        f.decision_maker_question = "   ".to_string();
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::DecisionMakerQuestion);
        assert_eq!(refusal.filter.code(), "F1");
        assert!(
            refusal.reason.contains("decision maker question"),
            "{}",
            refusal.reason
        );
        assert_eq!(refusals(&f).len(), 1);
    }

    #[test]
    fn f2_refuses_a_card_without_a_generative_process() {
        let mut f = complete_card();
        f.simulable_artifact = None;
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::GenerativeProcess);
        assert!(
            refusal.reason.contains("simulable artifact"),
            "{}",
            refusal.reason
        );
    }

    #[test]
    fn f2_also_refuses_a_card_without_a_seed() {
        let mut f = complete_card();
        f.seed = None;
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::GenerativeProcess);
        assert!(refusal.reason.contains("fixed seed"), "{}", refusal.reason);
    }

    #[test]
    fn f3_refuses_a_card_without_an_independent_unit() {
        let mut f = complete_card();
        f.independent_unit = None;
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::UnitOrNull);
        assert!(
            refusal.reason.contains("independent unit"),
            "{}",
            refusal.reason
        );
    }

    #[test]
    fn f3_refuses_a_card_without_a_null_model() {
        let mut f = complete_card();
        f.null_model = None;
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::UnitOrNull);
        assert!(refusal.reason.contains("null model"), "{}", refusal.reason);
    }

    #[test]
    fn f4_refuses_a_card_without_a_declared_action() {
        let mut f = complete_card();
        f.package_action = String::new();
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::ActionWithExit);
        assert!(
            refusal.reason.contains("declared action"),
            "{}",
            refusal.reason
        );
    }

    #[test]
    fn f4_refuses_a_card_without_an_exit_clause() {
        let mut f = complete_card();
        f.exit_clause = None;
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(refusal.filter, Filter::ActionWithExit);
        assert!(refusal.reason.contains("exit clause"), "{}", refusal.reason);
    }

    #[test]
    fn the_funnel_stops_at_the_first_gate() {
        let mut f = complete_card();
        f.decision_maker_question = String::new();
        f.simulable_artifact = None;
        f.independent_unit = None;
        f.package_action = String::new();
        let all = refusals(&f);
        assert_eq!(all.len(), 4, "the four filters had something to refuse");
        assert_eq!(all[0].filter, Filter::DecisionMakerQuestion);
        let refusal = filter_card(&f).expect_err("should refuse");
        assert_eq!(
            refusal.filter,
            Filter::DecisionMakerQuestion,
            "stops at the first one"
        );
    }

    #[test]
    fn the_pruning_fires_when_the_evsi_does_not_cover_the_threshold() {
        let cp = CostLoss::new(1.0, 10.0);
        assert!((cp.threshold() - 0.1).abs() < 1e-12);
        let pruning = cp.prune(0.001, 0.05);
        assert!(pruning.pruned, "{}", pruning.line());
        assert_eq!(pruning.reason, Some(PruneReason::BoundBelowThreshold));
        assert!(
            (pruning.upper_bound - 0.009).abs() < 1e-12,
            "bound {:.6}",
            pruning.upper_bound
        );
        assert_eq!(pruning.decision_without_information, Act::NotAct);
    }

    #[test]
    fn the_pruning_does_not_fire_when_the_evsi_covers_it() {
        let cp = CostLoss::new(1.0, 10.0);
        // at the threshold itself the bound is maximal: C(1 - C/L) = 0.9
        let pruning = cp.prune(0.1, 0.05);
        assert!(!pruning.pruned, "{}", pruning.line());
        assert_eq!(pruning.reason, None);
        assert!(
            (pruning.upper_bound - 0.9).abs() < 1e-12,
            "bound {:.6}",
            pruning.upper_bound
        );
        assert_eq!(pruning.decision_without_information, Act::Act);
        // far from the threshold, but still covering: p = 0.3 gives the bound 0.7
        let other = cp.prune(0.3, 0.5);
        assert!(!other.pruned);
        assert!((other.upper_bound - 0.7).abs() < 1e-12);
    }

    #[test]
    fn the_pruning_declares_no_uncertainty_at_the_extreme() {
        let cp = CostLoss::new(1.0, 10.0);
        let pruning = cp.prune(1.0, 0.0);
        assert!(pruning.pruned);
        assert_eq!(pruning.reason, Some(PruneReason::NoUncertainty));
        assert!(pruning.upper_bound.abs() < 1e-15);
    }

    #[test]
    fn the_upper_bound_limits_the_evsi_of_any_exam() {
        let cp = CostLoss::new(1.0, 10.0);
        let accuracies = [0.50, 0.70, 0.90, 0.99];
        let mut worst_slack = f64::MIN;
        for p in [0.02, 0.05, 0.10, 0.30, 0.60, 0.90] {
            let bound = cp.evsi_upper_bound(p);
            for se in accuracies {
                for sp in accuracies {
                    let v = cp.evsi(p, se, sp);
                    assert!(v >= -1e-12, "negative EVSI: {v:.6}");
                    assert!(
                        v <= bound + 1e-12,
                        "EVSI {v:.6} passed the bound {bound:.6} at p={p}, se={se}, sp={sp}"
                    );
                    worst_slack = worst_slack.max(v - bound);
                }
            }
            // the perfect exam spends the whole bound
            let perfect = cp.evsi(p, 1.0, 1.0);
            assert!(
                (perfect - bound).abs() < 1e-12,
                "perfect exam {perfect:.6} against bound {bound:.6}"
            );
        }
        assert!(
            worst_slack <= 0.0,
            "no exam passed the bound, worst slack {worst_slack:.6}"
        );
    }

    #[test]
    fn a_more_accurate_exam_is_not_worth_less() {
        let cp = CostLoss::new(2.0, 10.0);
        let p = 0.4;
        let weak = cp.evsi(p, 0.60, 0.60);
        let medium = cp.evsi(p, 0.80, 0.80);
        let strong = cp.evsi(p, 0.95, 0.95);
        assert!(weak <= medium + 1e-12, "{weak:.6} > {medium:.6}");
        assert!(medium <= strong + 1e-12, "{medium:.6} > {strong:.6}");
        assert!(
            strong < cp.evsi_upper_bound(p),
            "an imperfect exam does not spend the bound"
        );
    }

    #[test]
    fn the_measured_evsi_matches_the_exact_one() {
        let cp = CostLoss::new(1.0, 10.0);
        let (p, se, sp) = (0.30_f64, 0.90_f64, 0.70_f64);
        let threshold = cp.threshold();
        let n = 200_000_usize;
        let mut rng = Rng::new(2026);
        let mut informed_sum = 0.0_f64;
        let mut fixed_sum = 0.0_f64;
        let fixed_act = cp.best_act(p);
        for _ in 0..n {
            let u = (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
            let sick = u < p;
            let v = (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
            let positive = if sick { v < se } else { v >= sp };
            let posterior = if positive {
                se * p / (se * p + (1.0 - sp) * (1.0 - p))
            } else {
                let npv = sp * (1.0 - p) / (sp * (1.0 - p) + (1.0 - se) * p);
                1.0 - npv
            };
            let act = posterior >= threshold;
            let informed_payoff = if act {
                -cp.cost
            } else if sick {
                -cp.loss
            } else {
                0.0
            };
            let fixed_payoff = match fixed_act {
                Act::Act => -cp.cost,
                Act::NotAct => {
                    if sick {
                        -cp.loss
                    } else {
                        0.0
                    }
                }
            };
            informed_sum += informed_payoff;
            fixed_sum += fixed_payoff;
        }
        let measured = (informed_sum - fixed_sum) / n as f64;
        let exact = cp.evsi(p, se, sp);
        let bound = cp.evsi_upper_bound(p);
        assert!(
            (measured - exact).abs() < 0.02,
            "measured {measured:.4} against exact {exact:.4} (bound {bound:.4})"
        );
        assert!(
            measured <= bound + 0.02,
            "measured {measured:.4} passed the bound {bound:.4}"
        );
    }

    #[test]
    fn the_counter_adds_up_by_gate_and_keeps_the_sequence() {
        let mut k = CounterK::new();
        assert_eq!(k.total(), 0);
        assert_eq!(
            k.report(None).line(),
            "K = 0 attempts (F1 0, F2 0, F3 0, F4 0); approved 0; final version: not declared"
        );
        assert_eq!(k.refused(Filter::DecisionMakerQuestion, "v1"), 1);
        assert_eq!(k.refused(Filter::ActionWithExit, "v2"), 2);
        assert_eq!(k.refused(Filter::ActionWithExit, "v3"), 3);
        assert_eq!(k.approved("v4"), 4);
        assert_eq!(k.total(), 4);
        assert_eq!(k.by_filter(Filter::DecisionMakerQuestion), 1);
        assert_eq!(k.by_filter(Filter::GenerativeProcess), 0);
        assert_eq!(k.by_filter(Filter::ActionWithExit), 2);
        assert_eq!(k.approved_count(), 1);
        let labels: Vec<&str> = k.sequence().iter().map(|t| t.label.as_str()).collect();
        assert_eq!(labels, vec!["v1", "v2", "v3", "v4"]);
        assert!(k.sequence()[3].passed());
        assert_eq!(k.sequence()[0].refusal, Some(Filter::DecisionMakerQuestion));
        assert_eq!(k.sequence()[1].number, 2);
        let r = k.report(Some("v4"));
        assert_eq!(r.total, 4);
        assert_eq!(r.by_filter, [1, 0, 0, 2]);
        assert_eq!(r.approved, 1);
        assert_eq!(r.sequence.len(), 4);
        assert_eq!(
            r.line(),
            "K = 4 attempts (F1 1, F2 0, F3 0, F4 2); approved 1; final version: v4"
        );
    }

    #[test]
    fn the_funnel_counts_the_attempts_up_to_the_final_version() {
        let mut funnel = Funnel::new();
        let mut without_question = complete_card();
        without_question.decision_maker_question = String::new();
        let mut without_exit = complete_card();
        without_exit.exit_clause = None;
        let final_ = complete_card();

        assert!(funnel.attempt("v1", &without_question).is_err());
        assert!(funnel.attempt("v2", &without_exit).is_err());
        assert!(funnel.attempt("v3", &final_).is_ok());
        assert_eq!(funnel.counter().total(), 3);
        let r = funnel.close("v3");
        assert_eq!(r.total, 3);
        assert_eq!(r.approved, 1);
        assert_eq!(
            r.line(),
            "K = 3 attempts (F1 1, F2 0, F3 0, F4 1); approved 1; final version: v3"
        );
    }

    #[test]
    fn a_channel_without_information_zeroes_the_evsi_and_forces_an_immediate_stop() {
        let cp = CostLoss::new(1.0, 10.0);
        let m = DominanceMatrix::new(5.0, 2.0, 0.25, 1.50, 0.30);
        for p in [0.0, 0.01, 0.10, 0.30, 0.50, 0.90, 0.99, 1.0] {
            for (se, sp) in [(0.50, 0.50), (0.80, 0.20), (0.10, 0.90)] {
                let evsi = cp.evsi(p, se, sp);
                assert!(
                    evsi.abs() < 1e-12,
                    "channel with se+sp=1 should have EVSI=0, got {evsi:.6} at p={p}"
                );
                let stop = m.optimal_stop(&cp, p, se, sp);
                assert!(stop.channel_without_information, "{}", stop.line());
                assert!(stop.evsi.abs() < 1e-12);
                assert!((stop.voc - (-0.25)).abs() < 1e-12);
                assert_eq!(stop.tau_star, 0);
                assert_eq!(stop.dominant_policy, LocalPolicy::Realistic);
            }
        }
    }

    #[test]
    fn strict_dominance_invariant_in_the_hidden_probability() {
        let m = DominanceMatrix::new(4.0, 1.5, 0.40, 1.20, 0.25);
        assert!(m.invariant_in_p());
        // in the worst case (p = 0), acting still beats ruminating by V_L + c_R and paralysis by V_L + c_E
        assert!((m.margin_over_rumination(0.0) - 1.90).abs() < 1e-12);
        assert!((m.margin_over_paralysis(0.0) - 2.70).abs() < 1e-12);
        for k in 0..=100 {
            let p = k as f64 / 100.0;
            assert_eq!(m.best_policy(p), LocalPolicy::Realistic);
            let u_r = m.expected_utility(LocalPolicy::Realistic, p);
            let u_u = m.expected_utility(LocalPolicy::Rumination, p);
            let u_d = m.expected_utility(LocalPolicy::Paralysis, p);
            assert!((u_r - u_u - m.margin_over_rumination(p)).abs() < 1e-12);
            assert!((u_r - u_d - m.margin_over_paralysis(p)).abs() < 1e-12);
            assert!(u_r > u_u && u_r > u_d);
        }
    }
}
