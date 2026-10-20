//! Analytic oracles and the mutants of Appendix E.
//!
//! A fixed seed reproduces a wrong formula as faithfully as a right one, and the gates check the
//! form of a card, not the algebra of its artifact. An oracle is an identity that the formula must
//! satisfy at a point where the answer is known in closed form (a limit case, a symmetry, a
//! fixed point). A mutant is a recorded error of Appendix E, rewritten as a function with the same
//! signature as the instrument it once lived in. The test of the oracles is not that the list is
//! long but that every mutant dies: an oracle that no mutant fails measures nothing.
//!
//! Two guards keep the suite honest. Every oracle must pass on the instruments as they are, or the
//! oracle (or the instrument) is wrong and the run says which; and an oracle is itself a claim, so
//! the run prints one with a wrong reference and shows that it rejects the right formula.

use crate::ch11::TwoActs;
use crate::{ch05, ch08, ch10, ch11, multiverse, propagate};
use stochology_core::erlang::{erlang_b, erlang_c};

/// The formulas under test, as function pointers so that a mutant can replace one of them.
#[derive(Clone, Copy)]
pub struct Formulas {
    /// Mean wait in the queue of an M/G/1 post, minutes (Pollaczek–Khinchine).
    pub wait_mg1: fn(f64, f64, f64) -> f64,
    /// Mean wait in the queue of an M/M/1 post, minutes.
    pub wait_mm1: fn(f64, f64) -> f64,
    /// Gradient of the M/M/1 wait in (lambda, mu).
    pub gradient_mm1: fn(f64, f64) -> (f64, f64),
    /// Mean M/M/1 wait over a demand that is low or high with equal chance (Chapter 5).
    pub wait_two_rates: fn(f64, f64, f64) -> f64,
    /// Mean M/M/1 wait over a uniform demand on [a, b] (Chapter 5).
    pub wait_uniform: fn(f64, f64, f64) -> f64,
    /// Second derivative of the M/M/1 wait in lambda (Chapter 5).
    pub wait_curvature: fn(f64, f64) -> f64,
    /// Per-session drift and volatility of the synthetic market (Chapter 10).
    pub dow_calibration: fn() -> (f64, f64),
    /// Erlang C: probability of waiting with c servers and load a.
    pub erlang_c: fn(u32, f64) -> f64,
    /// Kelly fraction of a bet that gains b with chance p and loses a otherwise.
    pub kelly: fn(f64, f64, f64) -> f64,
    /// Level of return period T under a GEV(xi, mu, sigma).
    pub gev_level: fn(f64, f64, f64, f64) -> f64,
    /// Annual cost of a dike of height h: works plus 3000 times the exceedance.
    pub dike_cost: fn(f64, f64, f64, f64) -> f64,
    /// Positive predictive value of a confusion matrix.
    pub ppv: fn(f64, f64, f64) -> f64,
    /// The FPR returned by the Chouldechova identity.
    pub fpr_identity: fn(f64, f64, f64) -> f64,
    /// The FPR after moving one threshold until the FNR hits a target (binormal ROC).
    pub fpr_for_fnr: fn(f64, f64, f64) -> f64,
    /// Expected value of perfect information of two acts in two states.
    pub evpi: fn(&TwoActs) -> f64,
    /// Expected value of the information of a test with sensitivity and specificity.
    pub evsi: fn(&TwoActs, f64, f64) -> f64,
    /// Hill estimator over the k largest values.
    pub hill: fn(&[f64], usize) -> f64,
    /// Hill estimator as the laboratory of Chapter 10 computes it.
    pub hill_ch10: fn(&[f64], usize) -> f64,
    /// Median of the maximum of n draws of a Pareto(alpha).
    pub median_maximum: fn(u64, f64) -> f64,
    /// Resolution term of Murphy's decomposition.
    pub resolution: fn(&[f64], &[f64]) -> f64,
}

fn evpi_of(t: &TwoActs) -> f64 {
    t.evpi()
}

impl Formulas {
    /// The formulas as the instruments compute them today.
    pub fn instruments() -> Self {
        Formulas {
            wait_mg1: multiverse::wait,
            wait_mm1: propagate::wait,
            gradient_mm1: propagate::gradient,
            wait_two_rates: ch05::mean_wait_two_rates,
            wait_uniform: ch05::mean_wait_uniform,
            wait_curvature: ch05::wait_curvature,
            dow_calibration: ch10::dow_calibration,
            erlang_c,
            kelly: ch10::kelly_binary,
            gev_level: ch11::gev_level,
            dike_cost: ch11::dike_cost,
            ppv: ch11::ppv_from_matrix,
            fpr_identity: ch11::fpr_from_identity,
            fpr_for_fnr: ch11::fpr_for_fnr,
            evpi: evpi_of,
            evsi: ch11::evsi,
            hill: stochology_horizons::h1::hill,
            hill_ch10: ch10::hill,
            median_maximum: ch10::median_maximum,
            resolution: ch08::resolution,
        }
    }
}

/// An identity with a declared tolerance. `check` returns the largest discrepancy found
/// (an error, or a count of violated inequalities); the oracle passes when it is finite and
/// no larger than the tolerance.
pub struct Oracle {
    pub id: &'static str,
    pub chapter: &'static str,
    pub statement: &'static str,
    pub tolerance: f64,
    pub check: fn(&Formulas) -> f64,
}

impl Oracle {
    pub fn discrepancy(&self, f: &Formulas) -> f64 {
        (self.check)(f)
    }
    pub fn passes(&self, f: &Formulas) -> bool {
        let d = self.discrepancy(f);
        d.is_finite() && d <= self.tolerance
    }
}

fn relative(a: f64, b: f64) -> f64 {
    if !a.is_finite() || !b.is_finite() {
        return f64::INFINITY;
    }
    (a - b).abs() / b.abs().max(1e-300)
}

/// Strictly greater, and false when either side is NaN.
fn greater(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

fn worst(errors: impl Iterator<Item = f64>) -> f64 {
    // A NaN must fail the oracle, so it becomes infinite instead of vanishing in a max.
    errors.fold(
        0.0,
        |m, e| if e.is_nan() { f64::INFINITY } else { m.max(e) },
    )
}

/// Stable stations: (lambda, mu) with lambda < mu.
fn stations() -> Vec<(f64, f64)> {
    let mut v = Vec::new();
    for mu in [6.0, 12.0, 20.0] {
        for lambda in [0.5, 4.0, 8.0, 11.5, 19.0] {
            if lambda < mu {
                v.push((lambda, mu));
            }
        }
    }
    v
}

// ------------------------------------------------- queues (Chapters 5 and 12)

fn pk_reduces_to_mm1(f: &Formulas) -> f64 {
    // With exponential service (cv = 1), P-K is the M/M/1 wait in the queue:
    // W_q = lambda / (mu (mu - lambda)) hours, written out here, not borrowed.
    worst(stations().into_iter().flat_map(|(l, m)| {
        let reference = 60.0 * l / (m * (m - l));
        [
            relative((f.wait_mg1)(l, m, 1.0), reference),
            relative((f.wait_mm1)(l, m), reference),
        ]
    }))
}

fn pk_deterministic_half(f: &Formulas) -> f64 {
    // Constant service (cv = 0) waits exactly half of exponential service.
    worst(
        stations()
            .into_iter()
            .map(|(l, m)| relative((f.wait_mg1)(l, m, 0.0), 0.5 * (f.wait_mg1)(l, m, 1.0))),
    )
}

fn pk_monotone(f: &Formulas) -> f64 {
    // More arrivals or more variable service never shorten the wait.
    let mut violations = 0.0;
    for mu in [6.0, 12.0] {
        for cv in [0.0, 0.5, 1.0, 1.5] {
            let mut previous = (f.wait_mg1)(0.1, mu, cv);
            let mut lambda = 0.2;
            while lambda < mu {
                let w = (f.wait_mg1)(lambda, mu, cv);
                if !greater(w, previous) {
                    violations += 1.0;
                }
                previous = w;
                lambda += 0.1;
            }
        }
        let mut previous = (f.wait_mg1)(4.0, mu, 0.0);
        for cv in [0.25, 0.5, 1.0, 2.0] {
            let w = (f.wait_mg1)(4.0, mu, cv);
            if !greater(w, previous) {
                violations += 1.0;
            }
            previous = w;
        }
    }
    violations
}

fn mm1_gradient(f: &Formulas) -> f64 {
    // The analytic gradient agrees with a central difference of the wait it differentiates.
    let h = 1e-6;
    worst(
        stations()
            .into_iter()
            .filter(|(l, m)| m - l > 0.6)
            .flat_map(|(l, m)| {
                let (dl, dm) = (f.gradient_mm1)(l, m);
                let nl = ((f.wait_mm1)(l + h, m) - (f.wait_mm1)(l - h, m)) / (2.0 * h);
                let nm = ((f.wait_mm1)(l, m + h) - (f.wait_mm1)(l, m - h)) / (2.0 * h);
                [relative(dl, nl), relative(dm, nm)]
            }),
    )
}

fn mm1(l: f64, m: f64) -> f64 {
    60.0 * l / (m * (m - l))
}

fn jensen_two_state(f: &Formulas) -> f64 {
    // With demand lambda - delta or lambda + delta, the toll of Jensen is exact:
    // E[W] - W(lambda) = W(lambda) mu delta^2 / (lambda ((mu - lambda)^2 - delta^2)); zero without spread.
    let mut errors = Vec::new();
    for (l, m) in [(8.0, 12.0), (4.0, 6.0), (10.0, 20.0)] {
        errors.push(relative((f.wait_two_rates)(l, l, m), mm1(l, m)));
        for share in [0.25, 0.5, 0.75, 0.95] {
            let d: f64 = share * (m - l).min(l);
            let toll = mm1(l, m) * m * d * d / (l * ((m - l) * (m - l) - d * d));
            errors.push(relative(
                (f.wait_two_rates)(l - d, l + d, m) - mm1(l, m),
                toll,
            ));
        }
    }
    worst(errors.into_iter())
}

fn uniform_mean_is_an_integral(f: &Formulas) -> f64 {
    // (b - a) E[W] is the integral of W from a to b, so its derivative in b is W(b) and in a is -W(a).
    let h = 1e-5;
    let integral = |a: f64, b: f64, m: f64| (b - a) * (f.wait_uniform)(a, b, m);
    worst(
        [(5.0, 11.0, 12.0), (1.0, 4.0, 6.0), (2.0, 15.0, 20.0)]
            .into_iter()
            .flat_map(|(a, b, m)| {
                let db = (integral(a, b + h, m) - integral(a, b - h, m)) / (2.0 * h);
                let da = (integral(a + h, b, m) - integral(a - h, b, m)) / (2.0 * h);
                [relative(db, mm1(b, m)), relative(-da, mm1(a, m))]
            }),
    )
}

fn curvature_matches_wait(f: &Formulas) -> f64 {
    // The curvature that the second-order account uses is the second difference of W.
    let h = 1e-3;
    worst(
        [(8.0, 12.0), (4.0, 6.0), (10.0, 20.0)]
            .into_iter()
            .map(|(l, m)| {
                let second = (mm1(l + h, m) - 2.0 * mm1(l, m) + mm1(l - h, m)) / (h * h);
                relative((f.wait_curvature)(l, m), second)
            }),
    )
}

fn dow_calibration_round_trip(f: &Formulas) -> f64 {
    // The per-session constants give back the two annual numbers they were fitted to, with
    // d = 252 sessions: a drag of 0.14 at 3x and a growth of 0.051 at 1x. The tolerance is the
    // rounding of the printed constants.
    let (m, s) = (f.dow_calibration)();
    let d = 252.0;
    let drag3 = 0.5 * 9.0 * s * s * d;
    let growth1 = (m - 0.5 * s * s) * d;
    relative(drag3, 0.14).max(relative(growth1, 0.051))
}

fn erlang_c_single_server(f: &Formulas) -> f64 {
    // With one server the chance of waiting is the utilisation itself.
    worst(
        [0.1, 0.3, 0.5, 0.8, 0.95]
            .into_iter()
            .map(|rho| ((f.erlang_c)(1, rho) - rho).abs()),
    )
}

fn erlang_c_above_b(f: &Formulas) -> f64 {
    // Whoever is blocked in the loss system would have waited in the delay system: C >= B.
    let mut violations = 0.0;
    for c in 1..=10u32 {
        for share in [0.2, 0.5, 0.8, 0.95] {
            let a = share * c as f64;
            let cv = (f.erlang_c)(c, a);
            if !(cv >= erlang_b(c, a) - 1e-15 && cv <= 1.0) {
                violations += 1.0;
            }
        }
    }
    violations
}

// ------------------------------------------------- growth in time (Chapter 10)

fn bets() -> Vec<(f64, f64, f64)> {
    // (p, b, a) with a positive edge p b > (1 - p) a; the Peters coin is the second.
    vec![
        (0.6, 1.0, 1.0),
        (0.5, 0.5, 0.4),
        (0.55, 2.0, 1.0),
        (0.3, 4.0, 0.5),
        (0.99, 0.01, 0.95),
    ]
}

fn kelly_stationary(f: &Formulas) -> f64 {
    // The derivative of g(f) = p ln(1 + f b) + (1 - p) ln(1 - f a) vanishes at f*,
    // and f* lies inside the admissible range (0, 1/a).
    worst(bets().into_iter().map(|(p, b, a)| {
        let k = (f.kelly)(p, b, a);
        if !(k > 0.0 && k < 1.0 / a) {
            return f64::INFINITY;
        }
        (p * b / (1.0 + k * b) - (1.0 - p) * a / (1.0 - k * a)).abs()
    }))
}

fn kelly_null_edge(f: &Formulas) -> f64 {
    // A bet without an edge (p b = (1 - p) a) deserves no stake.
    worst(
        [(0.5, 1.0, 1.0), (0.4, 0.75, 0.5), (0.2, 2.0, 0.5)]
            .into_iter()
            .map(|(p, b, a)| {
                let edge: f64 = p * b - (1.0 - p) * a;
                debug_assert!(edge.abs() < 1e-12);
                (f.kelly)(p, b, a).abs()
            }),
    )
}

// ------------------------------------------------- extremes (Chapter 11)

fn gev_cdf(xi: f64, mu: f64, sigma: f64, h: f64) -> f64 {
    let z = (h - mu) / sigma;
    if xi.abs() < 1e-12 {
        (-(-z).exp()).exp()
    } else {
        let t = 1.0 + xi * z;
        if t <= 0.0 {
            if xi > 0.0 { 0.0 } else { 1.0 }
        } else {
            (-t.powf(-1.0 / xi)).exp()
        }
    }
}

fn gev_inverts_cdf(f: &Formulas) -> f64 {
    // The level of period T is the quantile 1 - 1/T of the annual maximum, Gumbel included.
    let mut errors = Vec::new();
    for xi in [-0.2, -0.05, 0.0, 0.1, 0.3] {
        for period in [10.0, 100.0, 10_000.0] {
            let level = (f.gev_level)(xi, 2.0, 0.5, period);
            errors.push((gev_cdf(xi, 2.0, 0.5, level) - (1.0 - 1.0 / period)).abs());
        }
    }
    worst(errors.into_iter())
}

fn exceedance(f: &Formulas, xi: f64, h: f64) -> f64 {
    ((f.dike_cost)(xi, 2.0, 0.5, h) - (h - 2.0).max(0.0)) / 3_000.0
}

fn exceedance_continuous_and_decreasing(f: &Formulas) -> f64 {
    // The Gumbel branch is the limit of the GEV as xi -> 0, and a taller dike is exceeded less.
    let mut d: f64 = 0.0;
    for h in [2.0, 3.0, 4.0, 6.0] {
        let g = exceedance(f, 0.0, h);
        d = d.max((exceedance(f, 1e-6, h) - g).abs());
        d = d.max((exceedance(f, -1e-6, h) - g).abs());
    }
    let mut previous = exceedance(f, 0.0, 1.0);
    for h in [1.5, 2.0, 3.0, 4.0, 6.0] {
        let p = exceedance(f, 0.0, h);
        if !greater(previous, p) {
            d = f64::INFINITY;
        }
        previous = p;
    }
    d
}

// ------------------------------------------------- fairness (Chapter 11)

fn matrices() -> Vec<(f64, f64, f64)> {
    // (base rate, FNR, FPR)
    let mut v = Vec::new();
    for p in [0.1, 0.35, 0.6] {
        for fnr in [0.05, 0.3] {
            for fpr in [0.02, 0.2, 0.45] {
                v.push((p, fnr, fpr));
            }
        }
    }
    v
}

fn chouldechova_round_trip(f: &Formulas) -> f64 {
    // An identity of the confusion matrix returns that matrix's own FPR exactly.
    worst(matrices().into_iter().map(|(p, fnr, fpr)| {
        let ppv = (f.ppv)(p, fnr, fpr);
        ((f.fpr_identity)(p, ppv, fnr) - fpr).abs()
    }))
}

fn roc_trade_off(f: &Formulas) -> f64 {
    // Moving to the observed FNR is a fixed point; lowering the FNR raises the FPR.
    let mut d: f64 = 0.0;
    for (_, fnr, fpr) in matrices() {
        d = d.max(((f.fpr_for_fnr)(fnr, fpr, fnr) - fpr).abs());
        if !greater((f.fpr_for_fnr)(fnr, fpr, 0.5 * fnr), fpr) {
            d = f64::INFINITY;
        }
    }
    d
}

// ------------------------------------------------- value of information (Chapters 11 and 15)

fn decisions() -> Vec<TwoActs> {
    let mut v = Vec::new();
    for p_sick in [0.05, 0.3, 0.7] {
        for (s, h) in [(-50.0, -10.0), (-200.0, -3.0), (-5.0, -1.0)] {
            v.push(TwoActs {
                u_act_sick: s / 5.0,
                u_not_act_sick: s,
                u_act_healthy: h,
                u_not_act_healthy: 0.0,
                p_sick,
            });
        }
    }
    v
}

fn evpi_null_and_homogeneous(f: &Formulas) -> f64 {
    // A payoff that does not depend on the state has no value of information; EVPI is never
    // negative; and scaling every payoff by gamma scales EVPI by gamma.
    let mut d: f64 = 0.0;
    for (a, b) in [(3.0, 1.0), (1.0, 3.0), (-2.0, 0.0)] {
        let flat = TwoActs {
            u_act_sick: a,
            u_not_act_sick: b,
            u_act_healthy: a,
            u_not_act_healthy: b,
            p_sick: 0.4,
        };
        d = d.max((f.evpi)(&flat).abs());
    }
    for t in decisions() {
        let e = (f.evpi)(&t);
        if e < -1e-12 {
            d = f64::INFINITY;
        }
        let gamma = 2.5;
        let scaled = TwoActs {
            u_act_sick: gamma * t.u_act_sick,
            u_not_act_sick: gamma * t.u_not_act_sick,
            u_act_healthy: gamma * t.u_act_healthy,
            u_not_act_healthy: gamma * t.u_not_act_healthy,
            p_sick: t.p_sick,
        };
        d = d.max(((f.evpi)(&scaled) - gamma * e).abs());
    }
    d
}

fn evsi_bounds(f: &Formulas) -> f64 {
    // A test whose result does not depend on the state is worth nothing; a perfect test is worth
    // EVPI; and every test lies between the two.
    let mut d: f64 = 0.0;
    for t in decisions() {
        let evpi = (f.evpi)(&t);
        d = d.max((f.evsi)(&t, 0.7, 0.3).abs());
        d = d.max(((f.evsi)(&t, 1.0, 1.0) - evpi).abs());
        for (se, sp) in [(0.9, 0.8), (0.6, 0.95)] {
            let e = (f.evsi)(&t, se, sp);
            if !(e >= -1e-12 && e <= evpi + 1e-12) {
                d = f64::INFINITY;
            }
        }
    }
    d
}

// ------------------------------------------------- tails (Chapters 10, 14 and 15)

/// Expected Pareto quantiles: a deterministic heavy tail.
fn pareto_quantiles(n: usize, alpha: f64) -> Vec<f64> {
    (1..=n)
        .map(|i| (n as f64 / (i as f64 - 0.5)).powf(1.0 / alpha))
        .collect()
}

fn hill_scale_invariance(f: &Formulas) -> f64 {
    // Hill reads only the ratios x_(i)/x_(k+1), so a change of units cannot move it.
    let xs = pareto_quantiles(5_000, 1.8);
    let base = (f.hill)(&xs, 200);
    worst([1e-3, 7.0, 1e6].into_iter().map(|kappa| {
        let scaled: Vec<f64> = xs.iter().map(|x| kappa * x).collect();
        relative((f.hill)(&scaled, 200), base)
    }))
}

/// A sample whose k largest values exceed the (k+1)-th, set at 1, by known logarithms c_i.
fn known_log_spacings(k: usize) -> (Vec<f64>, f64) {
    let logs: Vec<f64> = (1..=k)
        .map(|i| 0.05 * i as f64 + 0.01 * (i % 3) as f64)
        .collect();
    let mut xs: Vec<f64> = logs.iter().map(|c| c.exp()).collect();
    xs.push(1.0);
    xs.extend([0.9, 0.8, 0.5]);
    let expected = k as f64 / logs.iter().sum::<f64>();
    (xs, expected)
}

fn hill_definition(f: &Formulas) -> f64 {
    // The definition the book writes in Chapter 14: alpha = k / sum ln(x_(i)/x_(k+1)).
    worst([5usize, 25, 100].into_iter().map(|k| {
        let (xs, expected) = known_log_spacings(k);
        relative((f.hill)(&xs, k), expected)
    }))
}

fn hill_one_threshold(f: &Formulas) -> f64 {
    // Chapter 10 computes Hill with the same definition as Chapter 14, so on the same sample the two
    // implementations agree, and on known spacings they return the definition.
    let xs = pareto_quantiles(200_000, 1.1);
    let mut d = worst(
        [100usize, 1_000, 20_000]
            .into_iter()
            .map(|k| relative((f.hill_ch10)(&xs, k), (f.hill)(&xs, k))),
    );
    for k in [5usize, 25, 100] {
        let (ys, expected) = known_log_spacings(k);
        d = d.max(relative((f.hill_ch10)(&ys, k), expected));
    }
    d
}

fn median_maximum_solves(f: &Formulas) -> f64 {
    // The median maximum m solves (1 - m^-alpha)^n = 1/2.
    let mut errors = Vec::new();
    for n in [10u64, 1_000, 1_000_000] {
        for alpha in [1.1, 2.0, 3.0] {
            let m = (f.median_maximum)(n, alpha);
            let log_p = n as f64 * (-m.powf(-alpha)).ln_1p();
            errors.push((log_p + std::f64::consts::LN_2).abs());
        }
    }
    worst(errors.into_iter())
}

// ------------------------------------------------- calibration (Chapter 8)

fn constant_forecaster(f: &Formulas) -> f64 {
    // A forecaster who always says the same number cannot sort days, so its resolution is zero,
    // whatever the order in which the days arrive.
    let n = 1_000;
    let forecasts = vec![0.3; n];
    let front_loaded: Vec<f64> = (0..n).map(|i| if i < 300 { 1.0 } else { 0.0 }).collect();
    let alternating: Vec<f64> = (0..n).map(|i| if i % 3 == 0 { 1.0 } else { 0.0 }).collect();
    worst(
        [&front_loaded, &alternating]
            .into_iter()
            .map(|y| (f.resolution)(&forecasts, y).abs()),
    )
}

/// The catalogue of oracles.
pub fn catalogue() -> Vec<Oracle> {
    vec![
        Oracle {
            id: "pk_reduces_to_mm1",
            chapter: "5, 12",
            statement: "W(lambda, mu, cv = 1) = 60 lambda / (mu (mu - lambda))",
            tolerance: 1e-12,
            check: pk_reduces_to_mm1,
        },
        Oracle {
            id: "pk_deterministic_half",
            chapter: "5, 12",
            statement: "W(lambda, mu, 0) = W(lambda, mu, 1) / 2",
            tolerance: 1e-12,
            check: pk_deterministic_half,
        },
        Oracle {
            id: "pk_monotone",
            chapter: "5, 12",
            statement: "W rises with lambda and with cv",
            tolerance: 0.0,
            check: pk_monotone,
        },
        Oracle {
            id: "mm1_gradient",
            chapter: "5",
            statement: "analytic gradient = central difference of W",
            tolerance: 1e-6,
            check: mm1_gradient,
        },
        Oracle {
            id: "jensen_two_state",
            chapter: "5",
            statement: "E[W] - W(lambda) = W mu delta^2 / (lambda ((mu - lambda)^2 - delta^2))",
            tolerance: 1e-12,
            check: jensen_two_state,
        },
        Oracle {
            id: "uniform_mean_is_an_integral",
            chapter: "5",
            statement: "d/db [(b - a) E[W]] = W(b), d/da = -W(a)",
            tolerance: 1e-7,
            check: uniform_mean_is_an_integral,
        },
        Oracle {
            id: "curvature_matches_wait",
            chapter: "5",
            statement: "W''(lambda) = second difference of W",
            tolerance: 1e-5,
            check: curvature_matches_wait,
        },
        Oracle {
            id: "dow_calibration_round_trip",
            chapter: "10",
            statement: "4.5 sigma^2 d = 0.14 and (m - sigma^2/2) d = 0.051, d = 252",
            tolerance: 1e-3,
            check: dow_calibration_round_trip,
        },
        Oracle {
            id: "erlang_c_single_server",
            chapter: "12, 15",
            statement: "C(1, rho) = rho",
            tolerance: 1e-12,
            check: erlang_c_single_server,
        },
        Oracle {
            id: "erlang_c_above_b",
            chapter: "12, 15",
            statement: "B(c, a) <= C(c, a) <= 1",
            tolerance: 0.0,
            check: erlang_c_above_b,
        },
        Oracle {
            id: "kelly_stationary",
            chapter: "10",
            statement: "g'(f*) = 0 with f* in (0, 1/a)",
            tolerance: 1e-12,
            check: kelly_stationary,
        },
        Oracle {
            id: "kelly_null_edge",
            chapter: "10",
            statement: "p b = (1 - p) a  =>  f* = 0",
            tolerance: 1e-12,
            check: kelly_null_edge,
        },
        Oracle {
            id: "gev_inverts_cdf",
            chapter: "11",
            statement: "F(level(T)) = 1 - 1/T, Gumbel branch included",
            tolerance: 1e-9,
            check: gev_inverts_cdf,
        },
        Oracle {
            id: "exceedance_continuous_and_decreasing",
            chapter: "11",
            statement: "P(max > h) continuous at xi = 0 and falling in h",
            tolerance: 1e-5,
            check: exceedance_continuous_and_decreasing,
        },
        Oracle {
            id: "chouldechova_round_trip",
            chapter: "11",
            statement: "FPR(p, PPV(p, FNR, FPR), FNR) = FPR",
            tolerance: 1e-12,
            check: chouldechova_round_trip,
        },
        Oracle {
            id: "roc_trade_off",
            chapter: "11",
            statement: "fixed point at the observed FNR; lower FNR => higher FPR",
            tolerance: 1e-12,
            check: roc_trade_off,
        },
        Oracle {
            id: "evpi_null_and_homogeneous",
            chapter: "11, 15",
            statement: "state-free payoff => 0; EVPI >= 0; EVPI(gamma U) = gamma EVPI(U)",
            tolerance: 1e-12,
            check: evpi_null_and_homogeneous,
        },
        Oracle {
            id: "evsi_bounds",
            chapter: "11",
            statement: "uninformative => 0; perfect => EVPI; 0 <= EVSI <= EVPI",
            tolerance: 1e-12,
            check: evsi_bounds,
        },
        Oracle {
            id: "hill_scale_invariance",
            chapter: "10, 14, 15",
            statement: "alpha(kappa X) = alpha(X)",
            tolerance: 1e-12,
            check: hill_scale_invariance,
        },
        Oracle {
            id: "hill_definition",
            chapter: "14",
            statement: "alpha = k / sum ln(x_(i) / x_(k+1)) on known spacings",
            tolerance: 1e-12,
            check: hill_definition,
        },
        Oracle {
            id: "hill_one_threshold",
            chapter: "10, 14",
            statement: "the Hill of Chapter 10 is the Hill of Chapter 14",
            tolerance: 1e-12,
            check: hill_one_threshold,
        },
        Oracle {
            id: "median_maximum_solves",
            chapter: "10",
            statement: "(1 - m^-alpha)^n = 1/2",
            tolerance: 1e-9,
            check: median_maximum_solves,
        },
        Oracle {
            id: "constant_forecaster",
            chapter: "8",
            statement: "a constant forecast has RES = 0 in any order",
            tolerance: 1e-15,
            check: constant_forecaster,
        },
    ]
}

// ------------------------------------------------- the mutants

/// A recorded error, rewritten with the signature of the instrument it lived in.
pub struct Mutant {
    pub id: &'static str,
    /// Where the error is recorded: the Appendix E row, or the review that proposed it.
    pub record: &'static str,
    pub apply: fn(&mut Formulas),
}

fn m_wait_forgets_mu(f: &mut Formulas) {
    fn w(l: f64, m: f64, cv: f64) -> f64 {
        60.0 * l * (1.0 + cv * cv) / (2.0 * (m - l))
    }
    f.wait_mg1 = w;
}

fn m_wait_includes_service(f: &mut Formulas) {
    fn w(l: f64, m: f64, cv: f64) -> f64 {
        multiverse::wait(l, m, cv) + 60.0 / m
    }
    f.wait_mg1 = w;
}

fn m_mm1_forgets_mu(f: &mut Formulas) {
    fn w(l: f64, m: f64) -> f64 {
        60.0 * l / (m - l)
    }
    f.wait_mm1 = w;
}

fn m_gradient_drops_two(f: &mut Formulas) {
    fn g(l: f64, m: f64) -> (f64, f64) {
        let gap = m - l;
        (60.0 / (gap * gap), -60.0 * l * (m - l) / (m * gap).powi(2))
    }
    f.gradient_mm1 = g;
}

/// Chapter 5's uniform mean until 2026-10-19: a six-point midpoint rule.
pub fn uniform_six_point_midpoint(a: f64, b: f64, mu: f64) -> f64 {
    (0..6)
        .map(|i| mm1(a + (b - a) * (i as f64 + 0.5) / 6.0, mu))
        .sum::<f64>()
        / 6.0
}

fn m_uniform_six_points(f: &mut Formulas) {
    f.wait_uniform = uniform_six_point_midpoint;
}

fn m_two_rates_at_the_mean(f: &mut Formulas) {
    fn w(low: f64, high: f64, m: f64) -> f64 {
        mm1(0.5 * (low + high), m)
    }
    f.wait_two_rates = w;
}

fn m_curvature_wrong_power(f: &mut Formulas) {
    fn c(l: f64, m: f64) -> f64 {
        120.0 / (m - l).powi(2)
    }
    f.wait_curvature = c;
}

/// Chapter 10's calibration until 2026-10-18.
pub fn earlier_dow_calibration() -> (f64, f64) {
    (0.000317, 0.01518)
}

fn m_earlier_dow_calibration(f: &mut Formulas) {
    f.dow_calibration = earlier_dow_calibration;
}

fn m_erlang_c_is_b(f: &mut Formulas) {
    f.erlang_c = erlang_b;
}

fn m_kelly_without_ab(f: &mut Formulas) {
    fn k(p: f64, b: f64, a: f64) -> f64 {
        p * b - (1.0 - p) * a
    }
    f.kelly = k;
}

fn m_kelly_edge_over_gain(f: &mut Formulas) {
    fn k(p: f64, b: f64, a: f64) -> f64 {
        (p * b - (1.0 - p) * a) / b
    }
    f.kelly = k;
}

fn m_gumbel_returns_mu(f: &mut Formulas) {
    fn level(xi: f64, mu: f64, sigma: f64, period: f64) -> f64 {
        if xi.abs() < 1e-9 {
            mu
        } else {
            ch11::gev_level(xi, mu, sigma, period)
        }
    }
    f.gev_level = level;
}

fn m_gumbel_exceedance_sign(f: &mut Formulas) {
    fn cost(xi: f64, mu: f64, sigma: f64, h: f64) -> f64 {
        if xi.abs() < 1e-9 {
            (h - 2.0).max(0.0) + 3_000.0 * (1.0 - (-((h - mu) / sigma).exp()).exp())
        } else {
            ch11::dike_cost(xi, mu, sigma, h)
        }
    }
    f.dike_cost = cost;
}

fn m_identity_one_minus_fpr(f: &mut Formulas) {
    fn fpr(p: f64, ppv: f64, fnr: f64) -> f64 {
        1.0 - ch11::fpr_from_identity(p, ppv, fnr)
    }
    f.fpr_identity = fpr;
}

fn m_threshold_lowers_both(f: &mut Formulas) {
    fn fpr(observed_fnr: f64, observed_fpr: f64, target_fnr: f64) -> f64 {
        observed_fpr * target_fnr / observed_fnr
    }
    f.fpr_for_fnr = fpr;
}

fn m_evsi_best_act_per_state(f: &mut Formulas) {
    fn evsi(t: &TwoActs, se: f64, sp: f64) -> f64 {
        let p = t.p_sick;
        let positive = se * p + (1.0 - sp) * (1.0 - p);
        let q_pos = se * p / positive;
        let q_neg = (1.0 - se) * p / (1.0 - positive);
        let best_per_state = |q: f64| {
            q * t.u_act_sick.max(t.u_not_act_sick)
                + (1.0 - q) * t.u_act_healthy.max(t.u_not_act_healthy)
        };
        positive * best_per_state(q_pos) + (1.0 - positive) * best_per_state(q_neg)
            - t.best_without_information().1
    }
    f.evsi = evsi;
}

fn m_evpi_one_act(f: &mut Formulas) {
    fn evpi(t: &TwoActs) -> f64 {
        let with = t.p_sick * t.u_act_sick + (1.0 - t.p_sick) * t.u_act_healthy;
        with - t.best_without_information().1
    }
    f.evpi = evpi;
}

fn m_hill_log_levels(f: &mut Formulas) {
    fn hill(xs: &[f64], k: usize) -> f64 {
        let mut v: Vec<f64> = xs.iter().copied().filter(|x| *x > 0.0).collect();
        v.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        k as f64 / v[..k].iter().map(|x| x.ln()).sum::<f64>()
    }
    f.hill = hill;
}

/// Chapter 10's Hill until 2026-10-20: threshold x_(k), whose own term is ln(1) = 0.
pub fn hill_threshold_xk(xs: &[f64], k: usize) -> f64 {
    stochology_core::hill_at_cutoff(xs, k, k.saturating_sub(1))
}

fn m_hill_threshold_xk(f: &mut Formulas) {
    f.hill_ch10 = hill_threshold_xk;
}

fn m_median_maximum_at_n(f: &mut Formulas) {
    fn m(n: u64, _alpha: f64) -> f64 {
        n as f64
    }
    f.median_maximum = m;
}

fn m_boxes_cut_ties(f: &mut Formulas) {
    fn res(forecasts: &[f64], outcomes: &[f64]) -> f64 {
        // Ten boxes of equal mass in the arbitrary order of ties.
        let n = forecasts.len();
        let base = outcomes.iter().sum::<f64>() / n as f64;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|a, b| forecasts[*a].partial_cmp(&forecasts[*b]).unwrap());
        (0..10)
            .map(|k| {
                let idx = &order[k * n / 10..(k + 1) * n / 10];
                let m = idx.len() as f64;
                let ym = idx.iter().map(|i| outcomes[*i]).sum::<f64>() / m;
                m / n as f64 * (ym - base) * (ym - base)
            })
            .sum()
    }
    f.resolution = res;
}

/// The mutants. All but the first come from Appendix E; the first is the error of a review
/// that proposed these oracles and wrote the M/M/1 reference without the division by mu.
pub fn mutants() -> Vec<Mutant> {
    vec![
        Mutant {
            id: "wait_forgets_mu",
            record: "the review of 2026-10-20 that proposed the oracles (its own reference)",
            apply: m_wait_forgets_mu,
        },
        Mutant {
            id: "mm1_forgets_mu",
            record: "the same error, in the M/M/1 formula of the propagation laboratory",
            apply: m_mm1_forgets_mu,
        },
        Mutant {
            id: "wait_includes_service",
            record: "E, 2026-10-19, ch. 5/8/12: each wait included the client's own service",
            apply: m_wait_includes_service,
        },
        Mutant {
            id: "gradient_drops_two",
            record: "not recorded: an algebra slip in dW/dmu, kept as a control of mm1_gradient",
            apply: m_gradient_drops_two,
        },
        Mutant {
            id: "uniform_six_points",
            record: "E, 2026-10-19, ch. 5: a six-point midpoint rule for the uniform mean",
            apply: m_uniform_six_points,
        },
        Mutant {
            id: "two_rates_at_the_mean",
            record: "not recorded: the lesson of Chapter 5, W at the mean demand",
            apply: m_two_rates_at_the_mean,
        },
        Mutant {
            id: "curvature_wrong_power",
            record: "not recorded: (mu - lambda)^2 for the cube in W''",
            apply: m_curvature_wrong_power,
        },
        Mutant {
            id: "earlier_dow_calibration",
            record: "E, 2026-10-18, ch. 10: sigma = 0.01518 and m = 0.000317",
            apply: m_earlier_dow_calibration,
        },
        Mutant {
            id: "erlang_c_is_b",
            record: "not recorded: the loss formula in place of the delay formula",
            apply: m_erlang_c_is_b,
        },
        Mutant {
            id: "kelly_without_ab",
            record: "E, 2026-10-18, ch. 10: the Kelly formula omitted the division by ab",
            apply: m_kelly_without_ab,
        },
        Mutant {
            id: "kelly_edge_over_gain",
            record: "E, 2026-10-18, ch. 10: the insurance seller divided the edge by the gain only",
            apply: m_kelly_edge_over_gain,
        },
        Mutant {
            id: "gumbel_returns_mu",
            record: "E, 2026-10-18, ch. 11: the xi -> 0 branch returned mu",
            apply: m_gumbel_returns_mu,
        },
        Mutant {
            id: "gumbel_exceedance_sign",
            record: "E, 2026-10-18, ch. 11: exp(-exp(+z)) for the exceedance",
            apply: m_gumbel_exceedance_sign,
        },
        Mutant {
            id: "identity_one_minus_fpr",
            record: "E, 2026-10-18, ch. 11: the Chouldechova identity used 1 - FPR for FPR",
            apply: m_identity_one_minus_fpr,
        },
        Mutant {
            id: "threshold_lowers_both",
            record: "E, 2026-10-18, ch. 11: equalising the FNR lowered both errors",
            apply: m_threshold_lowers_both,
        },
        Mutant {
            id: "evsi_best_act_per_state",
            record: "E, 2026-10-18, ch. 11: the best act per state inside each branch",
            apply: m_evsi_best_act_per_state,
        },
        Mutant {
            id: "evpi_one_act",
            record: "E, 2026-10-19, ch. 15: the payoffs of one act passed as the best per state",
            apply: m_evpi_one_act,
        },
        Mutant {
            id: "hill_log_levels",
            record: "not recorded: logarithms of the levels instead of the ratios",
            apply: m_hill_log_levels,
        },
        Mutant {
            id: "hill_threshold_xk",
            record: "E, 2026-10-20, ch. 10: the threshold x_(k), found by these oracles",
            apply: m_hill_threshold_xk,
        },
        Mutant {
            id: "median_maximum_at_n",
            record: "E, 2026-10-19, ch. 10: n^(1 - alpha) placed the maximum at n",
            apply: m_median_maximum_at_n,
        },
        Mutant {
            id: "boxes_cut_ties",
            record: "E, 2026-10-18, ch. 8: boxes of equal mass split runs of identical forecasts",
            apply: m_boxes_cut_ties,
        },
    ]
}

/// The oracles that a mutant fails.
pub fn killed_by(m: &Mutant) -> Vec<&'static str> {
    let mut f = Formulas::instruments();
    (m.apply)(&mut f);
    catalogue()
        .into_iter()
        .filter(|o| !o.passes(&f))
        .map(|o| o.id)
        .collect()
}

/// The audit of an oracle: the review's reference for the M/M/1 wait, without the division by mu,
/// measured against the instrument. Returns the largest gap in minutes.
pub fn wrong_reference_gap() -> f64 {
    worst(stations().into_iter().map(|(l, m)| {
        let wrong_reference = 60.0 * l / (m - l);
        (multiverse::wait(l, m, 1.0) - wrong_reference).abs()
    }))
}

pub fn run() {
    println!("== Analytic oracles: identities the instruments must satisfy ==");
    let real = Formulas::instruments();
    let oracles = catalogue();
    let mut real_failures = 0;
    for o in &oracles {
        let d = o.discrepancy(&real);
        let verdict = if o.passes(&real) { "pass" } else { "FAIL" };
        if !o.passes(&real) {
            real_failures += 1;
        }
        println!(
            "{verdict}  {:<38} ch. {:<10} discrepancy {:>9.2e} <= {:.0e}  {}",
            o.id, o.chapter, d, o.tolerance, o.statement
        );
    }
    println!(
        "instruments: {} of {} oracles pass",
        oracles.len() - real_failures,
        oracles.len()
    );

    println!();
    println!("== Mutants: recorded errors, rewritten with the instrument's signature ==");
    let ms = mutants();
    let mut killed = 0;
    for m in &ms {
        let by = killed_by(m);
        if !by.is_empty() {
            killed += 1;
        }
        let verdict = if by.is_empty() { "SURVIVES" } else { "killed" };
        println!(
            "{verdict:<8}  {:<26} by {:<2} oracle(s): {}",
            m.id,
            by.len(),
            by.join(", ")
        );
        println!("          {}", m.record);
    }
    println!(
        "mutation score: {killed} of {} mutants killed ({:.0}%)",
        ms.len(),
        100.0 * killed as f64 / ms.len() as f64
    );

    println!();
    println!("== An oracle is a claim too ==");
    println!(
        "the reference 60 lambda / (mu - lambda), without the division by mu, rejects the instrument by up to {:.2} min",
        wrong_reference_gap()
    );
    println!(
        "reading: an oracle that fails on the instrument convicts one of the two; the closed form decides which"
    );

    println!();
    println!("== A mutant must reproduce the value it stands for ==");
    let (m0, s0) = earlier_dow_calibration();
    println!(
        "earlier calibration (m = {m0}, sigma = {s0}): drag at 3x = {:.4}/year, growth at 1x = {:.4}/year (fitted to 0.14 and 0.051)",
        0.5 * 9.0 * s0 * s0 * 252.0,
        (m0 - 0.5 * s0 * s0) * 252.0
    );
    println!(
        "its stated equation 4.5 sigma^2 = 0.14 gives sigma = {:.4}; with d = 252, {:.5}: the earlier {s0} follows from neither",
        (0.14_f64 / 4.5).sqrt(),
        (0.14_f64 / (4.5 * 252.0)).sqrt()
    );
    println!(
        "the earlier sigma solves 4.5 sigma^2 d = 0.14 with d = {:.1} sessions, not 252",
        0.14 / (4.5 * s0 * s0)
    );
    println!(
        "uniform mean on [5, 11] at mu = 12: six-point rule {:.2} min, closed form {:.2} min",
        uniform_six_point_midpoint(5.0, 11.0, 12.0),
        ch05::mean_wait_uniform(5.0, 11.0, 12.0)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_oracle_passes_on_the_instruments() {
        let real = Formulas::instruments();
        for o in catalogue() {
            assert!(
                o.passes(&real),
                "oracle {} fails: {:e}",
                o.id,
                o.discrepancy(&real)
            );
        }
    }

    #[test]
    fn every_mutant_is_killed() {
        for m in mutants() {
            assert!(
                !killed_by(&m).is_empty(),
                "mutant {} survives every oracle",
                m.id
            );
        }
    }

    #[test]
    fn every_oracle_kills_something_or_is_a_declared_guard() {
        // An oracle that no mutant fails is not yet shown to measure anything.
        let ms = mutants();
        let mut silent = Vec::new();
        for o in catalogue() {
            let kills = ms.iter().any(|m| {
                let mut f = Formulas::instruments();
                (m.apply)(&mut f);
                !o.passes(&f)
            });
            if !kills {
                silent.push(o.id);
            }
        }
        assert!(
            silent.iter().all(|id| [
                "pk_monotone",
                "erlang_c_above_b",
                "kelly_null_edge",
                "hill_definition"
            ]
            .contains(id)),
            "oracles without a mutant: {silent:?}"
        );
    }

    #[test]
    fn the_reviews_reference_rejects_the_right_formula() {
        // With lambda = 8 and mu = 12 the right wait is 10 min; the reference without /mu says 120.
        assert!((multiverse::wait(8.0, 12.0, 1.0) - 10.0).abs() < 1e-12);
        assert!(wrong_reference_gap() > 100.0);
    }

    #[test]
    fn the_mutants_of_chapter_5_reproduce_the_values_printed_before() {
        assert_eq!(
            format!("{:.2}", uniform_six_point_midpoint(5.0, 11.0, 12.0)),
            "14.10"
        );
        assert_eq!(
            format!("{:.2}", ch05::mean_wait_uniform(5.0, 11.0, 12.0)),
            "14.46"
        );
    }

    #[test]
    fn the_old_threshold_reproduces_the_value_chapter_10_printed() {
        // The sample of the Chapter 10 laboratory: seed 7, n = 200,000, alpha = 1.1. With the
        // threshold x_(k) it printed 1.083 at k = 100; with x_(k+1) it prints 1.080.
        let xs = ch10::hill_sample(7, 200_000, 1.1);
        assert_eq!(format!("{:.3}", hill_threshold_xk(&xs, 100)), "1.083");
        assert_eq!(format!("{:.3}", ch10::hill(&xs, 100)), "1.080");
    }
}
