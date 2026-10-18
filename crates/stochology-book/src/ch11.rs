//! Chapter 11 — the price of error: cost/loss, EVPI, dikes, Goodhart and the trilemma.
//!
//! Self-contained laboratory, no external dependencies.
//! Usage: cargo run --release

// ---------------------------------------------------------------- measured output
#[derive(Clone)]
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.0)
    }
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Standard normal distribution function, via the erf of Abramowitz and Stegun 7.1.26.
pub fn phi(x: f64) -> f64 {
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.231_641_9 * x);
    let y = 1.0
        - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736) * t
            + 0.254_829_592)
            * t
            * (-x * x).exp();
    0.5 * (1.0 + s * y)
}

/// Inverts the standard normal: the quantile z with P(N(0,1) <= z) = p.
pub fn normal_quantile(p: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    let (mut lo, mut hi) = (-10.0_f64, 10.0_f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if phi(mid) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

// ------------------------------------------------- the cost/loss threshold

pub struct Shipowner {
    pub wealth: f64,
    pub premium: f64,
    pub loss: f64,
    pub p: f64,
    pub ships: usize,
}

impl Shipowner {
    /// The threshold p > C/L: from which chance on it pays to pay the premium.
    pub fn threshold(&self) -> f64 {
        self.premium / self.loss
    }
    /// What the expectation says to do.
    pub fn protects_by_expectation(&self) -> bool {
        self.p > self.threshold()
    }
    /// Expected loss without insurance, and expected result with insurance.
    pub fn expectation_accounts(&self) -> (f64, f64) {
        (self.p * self.loss, self.premium - self.p * self.loss)
    }
    /// Growth in time per month of the shipowner who owns ~ships~ ships, with or
    /// without insurance over the lost cargo.
    pub fn owner_growth(&self, ships: usize, insurance: bool) -> f64 {
        let w = self.wealth;
        let mut g = 0.0;
        for i in 0..=ships {
            let prob = binomial(ships, i, self.p);
            if prob <= 0.0 {
                continue;
            }
            let premium_paid = if insurance { self.premium } else { 0.0 };
            let net_loss = if insurance { 0.0 } else { i as f64 * self.loss };
            let w_end = w - premium_paid - net_loss;
            if w_end <= 0.0 {
                return f64::NEG_INFINITY;
            }
            g += prob * (w_end / w).ln();
        }
        g
    }
}

/// Binomial probability P(X = i) for k trials with chance p.
pub fn binomial(k: usize, i: usize, p: f64) -> f64 {
    if i > k {
        return 0.0;
    }
    let mut c = 1.0;
    for j in 0..i {
        c *= (k - j) as f64 / (j + 1) as f64;
    }
    c * p.powi(i as i32) * (1.0 - p).powi((k - i) as i32)
}

/// Growth in time per month of an insurer with ~contracts~ independent
/// contracts, each with premium ~premium~ and loss ~loss~ with chance ~p~ and
/// wealth ~w~: mean minus variance/2 approximation for the sum.
pub fn insurer_growth(w: f64, contracts: usize, premium: f64, loss: f64, p: f64) -> f64 {
    let n = contracts as f64;
    let mean = n * (premium - p * loss);
    let var = n * loss * loss * p * (1.0 - p);
    (1.0 + mean / w).ln() - 0.5 * var / (w + mean).powi(2)
}
// ------------------------------------------------- the value of information

/// Values of two acts in two states: (payoff_act, payoff_state) for each cell.
#[derive(Clone, Copy)]
pub struct TwoActs {
    pub u_act_sick: f64,
    pub u_not_act_sick: f64,
    pub u_act_healthy: f64,
    pub u_not_act_healthy: f64,
    pub p_sick: f64,
}

impl TwoActs {
    pub fn expected_gain(&self, act: bool) -> f64 {
        if act {
            self.p_sick * self.u_act_sick + (1.0 - self.p_sick) * self.u_act_healthy
        } else {
            self.p_sick * self.u_not_act_sick + (1.0 - self.p_sick) * self.u_not_act_healthy
        }
    }
    /// The best act without information and the gain it delivers.
    pub fn best_without_information(&self) -> (bool, f64) {
        let a = self.expected_gain(true);
        let b = self.expected_gain(false);
        if a >= b { (true, a) } else { (false, b) }
    }
    /// The average gain of whoever knows the state before acting.
    pub fn with_perfect_information(&self) -> f64 {
        let d = self.u_act_sick.max(self.u_not_act_sick);
        let s = self.u_act_healthy.max(self.u_not_act_healthy);
        self.p_sick * d + (1.0 - self.p_sick) * s
    }
    pub fn evpi(&self) -> f64 {
        self.with_perfect_information() - self.best_without_information().1
    }
    /// The chance at which the best act changes:
    /// p* = (u_act_healthy - u_not_act_healthy)
    ///      / ((u_act_healthy - u_not_act_healthy) + (u_not_act_sick - u_act_sick)).
    pub fn action_threshold(&self) -> f64 {
        let a = self.u_act_healthy - self.u_not_act_healthy;
        let b = self.u_not_act_sick - self.u_act_sick;
        a / (a + b)
    }
}

/// Expected value of the information of an imperfect test: (sensitivity, specificity).
pub fn evsi(test: &TwoActs, sensitivity: f64, specificity: f64) -> f64 {
    let p = test.p_sick;
    let ppv = sensitivity * p / (sensitivity * p + (1.0 - specificity) * (1.0 - p));
    let npv = specificity * (1.0 - p) / (specificity * (1.0 - p) + (1.0 - sensitivity) * p);
    let positive_gain = ppv * test.u_act_sick.max(test.u_not_act_sick)
        + (1.0 - ppv) * test.u_act_healthy.max(test.u_not_act_healthy);
    let negative_gain = npv * test.u_not_act_sick.max(test.u_act_sick)
        + (1.0 - npv) * test.u_not_act_healthy.max(test.u_act_healthy);
    let positive_prob = sensitivity * p + (1.0 - specificity) * (1.0 - p);
    let with_test = positive_prob * positive_gain + (1.0 - positive_prob) * negative_gain;
    with_test - test.best_without_information().1
}

// ------------------------------------------------- the dikes of Holland

/// GEV quantile of a sample of maxima: the level associated with the return period ~T~.
/// F(h) = exp(-(1 + xi (h - mu) / sigma)^(-1/xi)); the level is
/// mu - sigma (1 - (-ln(1-1/T))^(-xi)) / xi.
pub fn gev_level(xi: f64, mu: f64, sigma: f64, period: f64) -> f64 {
    let y = (-(1.0_f64 - 1.0 / period).ln()).powf(-xi);
    if xi.abs() < 1e-9 {
        mu - sigma * (y - 1.0)
    } else {
        mu - sigma * (1.0 - y) / xi
    }
}

/// Annual cost of a dike of height h: the works, h - 2, plus the expected damage,
/// 3000 x P(maximum > h).
pub fn dike_cost(xi: f64, mu: f64, sigma: f64, h: f64) -> f64 {
    let works = (h - 2.0).max(0.0);
    if h <= mu + sigma / xi.max(1e-9) && xi > 0.0 {
        // above the support the probability is zero
    }
    let z = 1.0 + xi * (h - mu) / sigma;
    let prob = if z <= 0.0 {
        0.0
    } else if xi.abs() < 1e-9 {
        (-((h - mu) / sigma).exp()).exp()
    } else {
        let t = z.powf(-1.0 / xi);
        1.0 - (-t).exp()
    };
    works + 3_000.0 * prob
}

/// The heights of a fine grid that minimise the cost of each model.
pub fn optimal_height(xi: f64, mu: f64, sigma: f64) -> f64 {
    let mut best = 0.0;
    let mut best_cost = f64::INFINITY;
    let mut h = 0.0;
    while h <= 30.0 {
        let c = dike_cost(xi, mu, sigma, h);
        if c < best_cost {
            best_cost = c;
            best = h;
        }
        h += 0.001;
    }
    best
}

/// A group: how many rows, the base rate of the outcome, the observed FNR and FPR.
pub struct Group {
    pub name: &'static str,
    pub n: usize,
    pub base_rate: f64,
    pub observed_fnr: f64,
    pub observed_fpr: f64,
}

/// The FPR of a group when the FNR is driven to a target, described by the observed
/// pair (FNR, FPR). The account is Chouldechova's: with the threshold and the score
/// statistic z fixed, the FPR and the FNR move together, and one cannot choose both.
/// With the score tail treated as a gaussian of the same shape in both groups, the
/// ratio (1-FPR)/(1-FNR) is conserved, so that
/// FPR_target = 1 - (1 - FPR_obs) * (1 - FNR_target) / (1 - FNR_obs).
pub fn fpr_for_fnr(observed_fnr: f64, observed_fpr: f64, target_fnr: f64) -> f64 {
    let denom = 1.0 - observed_fnr;
    if denom <= 1e-9 {
        return observed_fpr;
    }
    (1.0 - (1.0 - observed_fpr) * (1.0 - target_fnr) / denom).clamp(0.0, 1.0)
}

/// The positive predictive value the confusion matrix delivers: TP/(TP+FP).
pub fn ppv_from_matrix(base_rate: f64, fnr: f64, fpr: f64) -> f64 {
    let tp = (1.0 - fnr) * base_rate;
    let fp = fpr * (1.0 - base_rate);
    tp / (tp + fp)
}

/// The PPV that the Chouldechova identity requires, given the base rate, the FNR and the FPR,
/// by the inverted form PPV = 1 / (1 + [(1-FPR)(1-p)] / [p(1-FNR)]).
pub fn ppv_from_identity(base_rate: f64, fnr: f64, fpr: f64) -> f64 {
    let odds = (1.0 - fpr) * (1.0 - base_rate) / (base_rate * (1.0 - fnr));
    1.0 / (1.0 + odds)
}

/// The two directions of the trilemma: matching the FNR downwards (to the value
/// of the least penalised group) and upwards (to that of the most penalised).
pub struct Trilemma {
    pub groups: [Group; 2],
    pub fpr_with_matched_fnr: [f64; 2],
    pub matched_target_fnr: f64,
}

pub fn trilemma(a: Group, b: Group) -> Trilemma {
    let target = a.observed_fnr.max(b.observed_fnr);
    let fpr_with_matched_fnr = [
        fpr_for_fnr(a.observed_fnr, a.observed_fpr, target),
        fpr_for_fnr(b.observed_fnr, b.observed_fpr, target),
    ];
    Trilemma {
        groups: [a, b],
        fpr_with_matched_fnr,
        matched_target_fnr: target,
    }
}
fn bernoulli_ship() {
    println!("== Chapter 11 :: the Bernoulli ship (C = 1.800, L = 34.000, p = 0,05) ==");
    let n = Shipowner {
        wealth: 100_000.0,
        premium: 1_800.0,
        loss: 34_000.0,
        p: 0.05,
        ships: 1,
    };
    let (expected_loss, insurance_result) = n.expectation_accounts();
    println!(
        "threshold p > C/L: p* = {:.5} ({:.3}%)  ->  p = 0,050 {} the threshold",
        n.threshold(),
        100.0 * n.threshold(),
        if n.protects_by_expectation() {
            "passes"
        } else {
            "does not pass"
        }
    );
    println!(
        "in expectation: expected loss without insurance = {:.0}, insurance result = {:+.0} per month",
        expected_loss, insurance_result
    );
    println!(
        "shipowner growth in time: 1 ship without insurance = {:+.4}, 1 ship with insurance = {:+.4}, 3 ships without insurance = {:+.4}",
        n.owner_growth(1, false),
        n.owner_growth(1, true),
        n.owner_growth(3, false)
    );
    for contracts in [1usize, 10, 100, 1_000] {
        println!(
            "insurer with {:>5} contracts: growth in time = {:+.6} per month",
            contracts,
            insurer_growth(200_000.0, contracts, 1_800.0, 34_000.0, 0.05)
        );
    }
    println!();
}

fn value_of_information() {
    println!("== Chapter 11 :: the EVPI (treating costs 1, leaving a sick patient costs 10) ==");
    let d = TwoActs {
        u_act_sick: -1.0,
        u_not_act_sick: -10.0,
        u_act_healthy: -1.0,
        u_not_act_healthy: 0.0,
        p_sick: 0.3,
    };
    println!(
        "action threshold: p > {:.2}  ->  with p = 0,3 the best act without information {} ({:+.2})",
        d.action_threshold(),
        if d.best_without_information().0 {
            "treat"
        } else {
            "do not treat"
        },
        d.best_without_information().1
    );
    println!(
        "gain of whoever knows the state before acting = {:+.2}",
        d.with_perfect_information()
    );
    println!("EVPI = {:+.2}", d.evpi());
    for (label, sens, spec) in [
        ("sensitive  (0,90; 0,70)", 0.9, 0.7),
        ("specific (0,30; 0,95)", 0.3, 0.95),
    ] {
        println!("  test {}: EVSI = {:+.4}", label, evsi(&d, sens, spec));
    }
    for p in [0.05_f64, 0.3, 0.6] {
        let mut e = d;
        e.p_sick = p;
        println!(
            "  with prevalence {:.2}: best act without information = {}, EVPI = {:+.3}",
            p,
            if e.best_without_information().0 {
                "treat"
            } else {
                "do not treat"
            },
            e.evpi()
        );
    }
    println!();
}

fn dikes() {
    println!("== Chapter 11 :: the van Dantzig dikes, four tail models ==");
    // Models calibrated so that the highest 100-year level is 4,26 m and the
    // Gumbel 10 000-year level is 4,76 m: mu = 1,2567 and sigma = 0,762.
    let mu = 1.2567;
    let sigma = 0.762;
    let xis = [-0.1, 0.0, 0.1, 0.2];
    print!("  {:<8}", "model");
    for xi in xis {
        print!("  xi = {:+.1}", xi);
    }
    println!("   optimal height");
    for period in [100.0_f64, 10_000.0] {
        print!("  T = {:<6.0}", period);
        for xi in xis {
            print!("  {:>9.2}", gev_level(xi, mu, sigma, period));
        }
        println!();
    }
    let mut heights = Vec::new();
    for xi in xis {
        let h = optimal_height(xi, mu, sigma);
        heights.push(h);
        println!(
            "  optimal height with xi = {:+.1}: {:.3} m  (cost {:.3})",
            xi,
            h,
            dike_cost(xi, mu, sigma, h)
        );
    }
    let mean = heights.iter().sum::<f64>() / heights.len() as f64;
    let wald = heights.iter().cloned().fold(f64::MIN, f64::max);
    let mut costs = vec![vec![0.0; 4]; 4];
    for (i, xi) in xis.iter().enumerate() {
        for (j, h) in heights.iter().enumerate() {
            costs[i][j] = dike_cost(*xi, mu, sigma, *h);
        }
    }
    // Savage: for each model, the regret is the cost minus the best cost of the model
    let smallest: Vec<f64> = (0..4)
        .map(|i| costs[i].iter().cloned().fold(f64::INFINITY, f64::min))
        .collect();
    let mut savage = heights[0];
    let mut best_regret = f64::INFINITY;
    for j in 0..4 {
        let worst = (0..4)
            .map(|i| costs[i][j] - smallest[i])
            .fold(f64::MIN, f64::max);
        if worst < best_regret {
            best_regret = worst;
            savage = heights[j];
        }
    }
    let precaution = xis
        .iter()
        .map(|xi| gev_level(*xi, mu, sigma, 10_000.0))
        .fold(f64::MIN, f64::max);
    println!("  expected value with the Gumbel: {:.2} m", heights[1]);
    println!("  mean of the models: {:.2} m", mean);
    println!("  Wald maximin: {:.2} m", wald);
    println!(
        "  Savage minimax regret: {:.2} m (largest regret {:.2})",
        savage, best_regret
    );
    println!(
        "  precaution: 1 in 10 000 in the worst model = {:.2} m",
        precaution
    );
    println!(
        "  EVPI of the model set = {:+.3} m",
        precaution - heights[1]
    );
    println!();
}
fn trilemma_block() {
    println!("== Chapter 11 :: the Chouldechova trilemma ==");
    // Base rates and errors of H1 (h1/README.md): 0,069 against 0,111 and FNR 0,24 against 0,36.
    // The FPR by group comes from the same fairness block: 0,105 against 0,196.
    let white = Group {
        name: "White",
        n: 4_776,
        base_rate: 0.069,
        observed_fnr: 0.24,
        observed_fpr: 0.105,
    };
    let black = Group {
        name: "Black",
        n: 4_776,
        base_rate: 0.111,
        observed_fnr: 0.36,
        observed_fpr: 0.196,
    };
    let t = trilemma(white, black);
    println!(
        "{:<8} {:>10} {:>10} {:>10} {:>16} {:>16} {:>10}",
        "group", "base rate", "FNR", "FPR", "false positives", "false negatives", "PPV"
    );
    let vpp = |g: &Group| {
        (1.0 - g.observed_fpr) * (1.0 - g.base_rate)
            / ((1.0 - g.observed_fpr) * (1.0 - g.base_rate) + (1.0 - g.observed_fnr) * g.base_rate)
    };
    let mut fp_before = 0.0;
    let mut fn_before = 0.0;
    for g in &t.groups {
        let fp = g.n as f64 * (1.0 - g.base_rate) * g.observed_fpr;
        let fneg = g.n as f64 * g.base_rate * g.observed_fnr;
        fp_before += fp;
        fn_before += fneg;
        println!(
            "{:<8} {:>10.3} {:>10.3} {:>10.3} {:>16.0} {:>16.0} {:>10.4}",
            g.name,
            g.base_rate,
            g.observed_fnr,
            g.observed_fpr,
            fp,
            fneg,
            vpp(g)
        );
    }
    println!(
        "observed cases: White {} marked by the facts, Black {}",
        (t.groups[0].n as f64 * t.groups[0].base_rate) as u32,
        (t.groups[1].n as f64 * t.groups[1].base_rate) as u32
    );
    println!();
    for (label, target) in [
        ("downwards, to the FNR of White", t.groups[0].observed_fnr),
        ("upwards, to the FNR of Black", t.groups[1].observed_fnr),
    ] {
        let fprs: Vec<f64> = t
            .groups
            .iter()
            .map(|g| fpr_for_fnr(g.observed_fnr, g.observed_fpr, target))
            .collect();
        let mut fp = 0.0;
        let mut fneg = 0.0;
        println!("-- FNR matched {} at {:.3} --", label, target);
        for (i, g) in t.groups.iter().enumerate() {
            let f = g.n as f64 * (1.0 - g.base_rate) * fprs[i];
            let n_ = g.n as f64 * g.base_rate * target;
            fp += f;
            fneg += n_;
            println!(
                "{:<8} {:>10.3} {:>10.3} {:>10.3} {:>16.0} {:>16.0}",
                g.name, g.base_rate, target, fprs[i], f, n_
            );
        }
        println!(
            "total false positives: {:.0} -> {:.0} ({:+.0})",
            fp_before,
            fp,
            fp - fp_before
        );
        println!(
            "total false negatives: {:.0} -> {:.0} ({:+.0})",
            fn_before,
            fneg,
            fneg - fn_before
        );
        println!();
    }
    println!("Chouldechova identity: FPR = p/(1-p) * (1-PPV)/PPV * (1-FNR)");
    println!(
        "{:<8} {:>16} {:>18} {:>14}",
        "group", "PPV of the matrix", "PPV of the identity", "ratio"
    );
    for g in &t.groups {
        let from_matrix = ppv_from_matrix(g.base_rate, g.observed_fnr, g.observed_fpr);
        let from_identity = ppv_from_identity(g.base_rate, g.observed_fnr, g.observed_fpr);
        println!(
            "{:<8} {:>16.4} {:>18.4} {:>14.1}x",
            g.name,
            from_matrix,
            from_identity,
            from_identity / from_matrix
        );
    }
    println!("reading: with the observed FNR and FPR, the score would only be calibrated by group");
    println!(
        "if the positive predictive value were the one in the middle column. It is much larger: the score"
    );
    println!("separates reasonably and is not read as a probability of risk.");
    println!();
}

pub fn run() {
    bernoulli_ship();
    value_of_information();
    dikes();
    trilemma_block();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dike_gev_underestimates_with_gumbel_and_chouldechova_closes() {
        let h_gumbel = optimal_height(0.0, 1.2567, 0.762);
        let h_frechet = optimal_height(0.2, 1.2567, 0.762);
        assert!(h_frechet > h_gumbel + 2.0);
        let v1 = ppv_from_matrix(0.111, 0.36, 0.196);
        assert!(v1 > 0.2 && v1 < 0.5);
    }
}
