//! Chapter 10 — time and ruin: ensemble mean against time mean.
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
    pub fn normal(&mut self) -> f64 {
        loop {
            let u = 2.0 * self.uniform() - 1.0;
            let v = 2.0 * self.uniform() - 1.0;
            let s = u * u + v * v;
            if s > 0.0 && s < 1.0 {
                return u * (-2.0 * s.ln() / s).sqrt();
            }
        }
    }
    /// Pareto tail P(X > x) = x^(-alpha), x >= 1.
    pub fn pareto(&mut self, alpha: f64) -> f64 {
        let u = self.uniform().max(1e-300);
        u.powf(-1.0 / alpha)
    }
}

// ------------------------------------------------- the Peters coin

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stake {
    All,
    Kelly,
    HalfKelly,
    QuarterKelly,
}

impl Stake {
    pub fn fraction(&self) -> f64 {
        match self {
            Stake::All => 1.0,
            Stake::Kelly => 0.70,
            Stake::HalfKelly => 0.35,
            Stake::QuarterKelly => 0.14,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Stake::All => "all-in (f = 1.00)",
            Stake::Kelly => "full Kelly (f = 0.70)",
            Stake::HalfKelly => "half Kelly (f = 0.35)",
            Stake::QuarterKelly => "quarter Kelly (f = 0.14)",
        }
    }
}

pub struct Cohort {
    pub name: &'static str,
    pub above_start: Vec<f64>,
    pub fraction_ruined: f64,
    pub final_median: f64,
    pub final_mean: f64,
}

/// One trajectory of the Peters coin: heads multiplies by 1.5, tails by 0.6.
/// Every round risks the fraction f of the wealth.
pub fn one_trajectory(rng: &mut SplitMix64, steps: usize, f: f64) -> Vec<f64> {
    let mut wealth = 1.0;
    let mut curve = Vec::with_capacity(steps + 1);
    curve.push(wealth);
    for _ in 0..steps {
        let m = if rng.uniform() < 0.5 { 1.5 } else { 0.6 };
        wealth *= 1.0 + f * (m - 1.0);
        curve.push(wealth);
    }
    curve
}

pub fn cohort(seed: u64, trajectories: usize, steps: usize, stake: Stake) -> Cohort {
    let mut rng = SplitMix64::new(seed);
    let marks = [10usize, 100, 1000];
    let mut above = vec![0usize; marks.len()];
    let mut ruined = 0usize;
    let mut finals = Vec::with_capacity(trajectories);
    for _ in 0..trajectories {
        let curve = one_trajectory(&mut rng, steps, stake.fraction());
        for (i, m) in marks.iter().enumerate() {
            if *m <= steps && curve[*m] > 1.0 {
                above[i] += 1;
            }
        }
        let end = *curve.last().unwrap();
        if end < 1e-6 {
            ruined += 1;
        }
        finals.push(end);
    }
    finals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = trajectories as f64;
    Cohort {
        name: stake.name(),
        above_start: above.iter().map(|c| *c as f64 / n).collect(),
        fraction_ruined: ruined as f64 / n,
        final_median: finals[trajectories / 2],
        final_mean: finals.iter().sum::<f64>() / n,
    }
}

/// Cohort mean (expectation) and time mean (log) per round, theoretical:
/// the payoffs `a` (heads) and `b` (tails) multiply the wealth.
pub fn coin_rates(p: f64, a: f64, b: f64) -> (f64, f64) {
    (p * a + (1.0 - p) * b, p * a.ln() + (1.0 - p) * b.ln())
}

/// Kelly criterion for an even-odds bet with chance p: f* = 2p - 1.
pub fn kelly_even_odds(p: f64) -> f64 {
    2.0 * p - 1.0
}

/// Optimal Kelly fraction for the 5:4 payoff (heads +50%, tails -40%):
/// f* = p|b| - (1-p)a = p(0.5 + 0.4) - 0.4 = 0.9p - 0.4: with p = 0.6, f* = 0.14.
pub fn kelly_peters_coin(p: f64) -> f64 {
    p * 0.9 - 0.4
}

/// Ensemble mean and time mean of the Peters coin with the fraction f.
pub fn peters_coin_with_f(p: f64, f: f64) -> (f64, f64) {
    let expectation = 1.0 + f * (p * 1.5 + (1.0 - p) * 0.6 - 1.0);
    let log_time = (1.0 - p) * (1.0 - 0.4 * f).ln() + p * (1.0 + 0.5 * f).ln();
    (expectation, log_time)
}

// ------------------------------------------------- the insurance seller

/// Time growth of whoever stakes everything on the insurance seller's strategy:
/// earns 1% per week with probability 1 - q and loses 50% with probability q.
pub fn weekly_rate(q: f64) -> f64 {
    (1.0 - q) * 1.01_f64.ln() + q * 0.5_f64.ln()
}

/// A cohort of investors who estimate q from their own history and lever up.
/// Below this the portfolio is considered ruined: 0.01% of the initial capital.
pub const RUIN: f64 = 1e-6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Frequency,
    Laplace,
    HalfLaplace,
}

pub struct Insurer {
    pub name: &'static str,
    pub fraction_no_loss: f64,
    pub fraction_all_in: f64,
    pub fraction_ruined: f64,
    pub fraction_ruined_at_1000: f64,
    pub final_median: f64,
    pub final_mean: f64,
}

/// Two leverage rules over the same 250-week history:
/// `laplace = false` — the observed frequency goes straight into Kelly, zero losses
/// estimates q = 0 and orders an all-in (f = 1);
/// `laplace = true` — the succession rule (s+1)/(n+2) forbids the zero.
pub fn insurers(
    seed: u64,
    investors: usize,
    weeks_history: usize,
    steps: usize,
    q: f64,
    rule: Rule,
) -> Insurer {
    let mut rng = SplitMix64::new(seed);
    let (mut no_loss, mut all_in, mut ruined, mut ruined_1000) = (0usize, 0usize, 0usize, 0usize);
    let mut finals = Vec::with_capacity(investors);
    for _ in 0..investors {
        let mut losses = 0usize;
        for _ in 0..weeks_history {
            if rng.uniform() < q {
                losses += 1;
            }
        }
        if losses == 0 {
            no_loss += 1;
        }
        let q_hat = if rule == Rule::Laplace || rule == Rule::HalfLaplace {
            (losses as f64 + 1.0) / (weeks_history as f64 + 2.0)
        } else {
            losses as f64 / weeks_history as f64
        };
        let mu = 0.01;
        let mut f = (((1.0 - q_hat) * mu - q_hat * 0.5) / mu).clamp(0.0, 1.0);
        if rule == Rule::HalfLaplace {
            f *= 0.5;
        }
        if f >= 0.999 {
            all_in += 1;
        }
        let mut wealth = 1.0;
        for _ in 0..steps {
            if rng.uniform() < q {
                wealth *= 1.0 - f * 0.5;
            } else {
                wealth *= 1.0 + f * mu;
            }
        }
        if wealth < RUIN {
            ruined += 1;
            ruined_1000 += 1;
        }
        finals.push(wealth);
    }
    finals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = investors as f64;
    Insurer {
        name: match rule {
            Rule::Frequency => "observed frequency + full Kelly",
            Rule::Laplace => "Laplace + full Kelly",
            Rule::HalfLaplace => "Laplace + half Kelly",
        },
        fraction_no_loss: no_loss as f64 / n,
        fraction_all_in: all_in as f64 / n,
        fraction_ruined: ruined as f64 / n,
        fraction_ruined_at_1000: ruined_1000 as f64 / n,
        final_median: finals[investors / 2],
        final_mean: finals.iter().sum::<f64>() / n,
    }
}

// ------------------------------------------------- the wild tail

pub struct ParetoSample {
    pub n: usize,
    pub alpha: f64,
    pub mean: f64,
    pub median: f64,
    pub fraction_mean_below_true: f64,
    pub coverage_of_95_interval: f64,
    pub fraction_of_mean_above_max_seen: f64,
}

/// Draws `repetitions` samples of size `n` from a Pareto with P(X > x) = x^(-alpha).
pub fn pareto_samples(seed: u64, n: usize, alpha: f64, repetitions: usize) -> ParetoSample {
    let mut rng = SplitMix64::new(seed);
    let true_mean = if alpha > 1.0 {
        alpha / (alpha - 1.0)
    } else {
        f64::INFINITY
    };
    let mut means = Vec::with_capacity(repetitions);
    let (mut below, mut covers) = (0usize, 0usize);
    let mut fraction_dominated_by_max = 0.0;
    for _ in 0..repetitions {
        let mut xs = Vec::with_capacity(n);
        for _ in 0..n {
            xs.push(rng.pareto(alpha));
        }
        let mean = xs.iter().sum::<f64>() / n as f64;
        let var = xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n as f64 - 1.0);
        let se = (var / n as f64).sqrt();
        if mean < true_mean {
            below += 1;
        }
        if mean - 1.96 * se <= true_mean && true_mean <= mean + 1.96 * se {
            covers += 1;
        }
        let max = xs.iter().cloned().fold(f64::MIN, f64::max);
        let above_max: f64 = xs.iter().filter(|x| **x > max).sum();
        fraction_dominated_by_max += above_max / (mean * n as f64);
        means.push(mean);
    }
    means.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ParetoSample {
        n,
        alpha,
        mean: means.iter().sum::<f64>() / repetitions as f64,
        median: means[repetitions / 2],
        fraction_mean_below_true: below as f64 / repetitions as f64,
        coverage_of_95_interval: covers as f64 / repetitions as f64,
        fraction_of_mean_above_max_seen: fraction_dominated_by_max / repetitions as f64,
    }
}

/// Hill estimator with the `k` largest values: alpha_hat = k / sum of the logarithms.
pub fn hill(xs: &[f64], k: usize) -> f64 {
    stochology_core::hill_at_cutoff(xs, k, k.saturating_sub(1))
}

/// Generates a Pareto sample and returns Hill for several k, with the asymptotic
/// interval alpha_hat (1 +- 1.96/sqrt(k)).
pub fn hill_by_k(seed: u64, n: usize, alpha: f64, ks: &[usize]) -> Vec<(usize, f64, f64, f64)> {
    let mut rng = SplitMix64::new(seed);
    let xs: Vec<f64> = (0..n).map(|_| rng.pareto(alpha)).collect();
    ks.iter()
        .map(|k| {
            let h = hill(&xs, *k);
            let error = 1.96 / (*k as f64).sqrt();
            (*k, h, h * (1.0 - error), h * (1.0 + error))
        })
        .collect()
}

// ------------------------------------------------- growth in time

/// Growth rate per period of a binary bet: with probability p the wealth
/// multiplies by (1 + gain) and with (1 - p) by (1 - loss).
pub fn growth_in_time(p: f64, gain: f64, loss: f64) -> f64 {
    p * (1.0 + gain).ln() + (1.0 - p) * (1.0 - loss).ln()
}

// ------------------------------------------------- volatility drag

pub struct Portfolio {
    pub leverage: f64,
    pub arithmetic_mean: f64,
    pub time_growth: f64,
    pub drag: f64,
    pub final_wealth: f64,
}

/// Generates a daily series of normal returns and measures portfolios levered at 1x, 2x
/// and 3x, rebalanced every day: the wealth multiplies by (1 + L r) each session.
pub fn long_leverage(seed: u64, sessions: usize, mu: f64, sigma: f64) -> Vec<Portfolio> {
    let mut rng = SplitMix64::new(seed);
    let returns: Vec<f64> = (0..sessions).map(|_| mu + sigma * rng.normal()).collect();
    [1.0, 2.0, 3.0]
        .iter()
        .map(|l| {
            let mut wealth = 1.0;
            let mut sum_log = 0.0;
            for r in &returns {
                let factor = 1.0 + l * r;
                wealth *= factor;
                sum_log += factor.ln();
            }
            Portfolio {
                leverage: *l,
                arithmetic_mean: *l * mu,
                time_growth: sum_log / sessions as f64,
                drag: *l * mu - sum_log / sessions as f64,
                final_wealth: wealth,
            }
        })
        .collect()
}

/// The volatility-drag rule: mu - sigma^2/2 per period.
pub fn theoretical_drag(mu: f64, sigma: f64) -> (f64, f64) {
    (mu - 0.5 * sigma * sigma, 0.5 * sigma * sigma)
}

fn peters_coin() {
    println!("== Chapter 10 :: the Peters coin (heads +50%, tails -40%) ==");
    let (expectation, log_time) = coin_rates(0.5, 1.5, 0.6);
    let f_optimal = kelly_peters_coin(0.6);
    let (_, g_optimal) = peters_coin_with_f(0.6, f_optimal);
    println!(
        "ensemble mean per round = {:.4}  ({:+.2}% per round)",
        expectation,
        100.0 * (expectation - 1.0)
    );
    println!(
        "time mean per round      = {:.6}  ({:+.3}% per round)",
        log_time,
        100.0 * log_time
    );
    println!(
        "product of one head and one tail: 1.5 x 0.6 = {:.2}",
        1.5 * 0.6
    );
    println!(
        "full Kelly (5:4): f* = {:.4} of the wealth; with an even-odds payoff, 2p - 1 = {:.2}",
        f_optimal,
        kelly_even_odds(0.6)
    );
    println!("time growth with full Kelly: {:+.4} per round", g_optimal);
    println!();
    let stakes = [
        Stake::All,
        Stake::Kelly,
        Stake::HalfKelly,
        Stake::QuarterKelly,
    ];
    for a in stakes {
        let c = cohort(2026, 10_000, 1_000, a);
        println!(
            "{:<26} above the start: 10 rounds {:.3}  100 rounds {:.3}  1000 rounds {:.5}",
            c.name, c.above_start[0], c.above_start[1], c.above_start[2]
        );
        println!(
            "{:<26} ruin (< 1e-6 of the capital) = {:.4}  final median = {:.3e}  final mean = {:.3e}",
            "", c.fraction_ruined, c.final_median, c.final_mean
        );
    }
    println!();
    let c = cohort(2026, 10_000, 1_000, Stake::All);
    println!(
        "all-in: log10 of the final median = {:.1}, log10 of the final mean = {:.1}",
        c.final_median.log10(),
        c.final_mean.log10()
    );
    println!(
        "part of the mean that comes from the richest trajectory (richest 1%): {:.3}",
        top_share_of_mean(2026, 10_000, 1_000)
    );
    println!();
}

/// Fraction of the cohort mean that comes from the richest 1% of trajectories.
fn top_share_of_mean(seed: u64, trajectories: usize, steps: usize) -> f64 {
    let mut rng = SplitMix64::new(seed);
    let mut finals = Vec::with_capacity(trajectories);
    for _ in 0..trajectories {
        let c = one_trajectory(&mut rng, steps, 1.0);
        finals.push(*c.last().unwrap());
    }
    finals.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let total: f64 = finals.iter().sum();
    let top: f64 = finals.iter().take(trajectories / 100).sum();
    top / total
}

fn insurance_seller() {
    println!(
        "== Chapter 10 :: the insurance seller (earns 1% per week, loses 50% with q = 0.004) =="
    );
    let q = 0.004;
    println!(
        "time growth of the all-in: {:+.5} per week (negative: ruin is certain)",
        weekly_rate(q)
    );
    for rule in [Rule::Frequency, Rule::Laplace, Rule::HalfLaplace] {
        let s = insurers(2026, 10_000, 250, 1_000, q, rule);
        println!(
            "{:<32} no loss = {:.3}  all-in = {:.3}  ruin in 1000 weeks = {:.4}  final median = {:.3e}  final mean = {:.3e}",
            s.name,
            s.fraction_no_loss,
            s.fraction_all_in,
            s.fraction_ruined_at_1000,
            s.final_median,
            s.final_mean
        );
    }
    let no_loss_theoretical = (1.0 - q).powi(250);
    println!(
        "theoretical probability of 250 weeks with no loss = {:.4}",
        no_loss_theoretical
    );
    println!(
        "Laplace with zero losses in 250 weeks: q_hat = 1/252 = {:.6}",
        1.0 / 252.0
    );
    println!();
}

fn wild_tail() {
    println!("== Chapter 10 :: the wild tail: Pareto with alpha = 1.1 ==");
    let true_mean = 1.1 / 0.1;
    println!("true mean (alpha/(alpha-1)) = {:.1}", true_mean);
    for n in [1_000usize, 1_000_000] {
        let a = pareto_samples(99, n, 1.1, 200);
        println!(
            "n = {:>9}: mean of the sample means = {:>8.2}  median = {:>7.2}  below 11 in {:.2}  95% CI covers in {:.2}",
            a.n, a.mean, a.median, a.fraction_mean_below_true, a.coverage_of_95_interval
        );
    }
    println!(
        "fraction of the mean that comes from values above the largest seen: {:.3}",
        pareto_samples(99, 1_000_000, 1.1, 200).fraction_of_mean_above_max_seen
    );
    println!(
        "in the median, {:.2}% of the true mean comes from values above the maximum of a sample of one million",
        100.0 * (1.0 / 1_000_000f64).powf(0.1)
    );
    println!();
    println!("Hill estimator with alpha = 1.1 and n = 200 000:");
    for (k, h, lo, hi) in hill_by_k(7, 200_000, 1.1, &[100, 500, 1_000, 5_000, 20_000]) {
        println!(
            "  k = {:>6}  alpha_hat = {:.3}  95% CI approx [{:.3}; {:.3}]",
            k, h, lo, hi
        );
    }
    println!(
        "gregarine: a Gaussian model fitted to this tail underestimates the loss in every repetition"
    );
    println!();
}

fn dow_leverage() {
    println!("== Chapter 10 :: volatility drag at 1x, 2x and 3x ==");
    // Calibration by the H1 drag (h1-real/README.md):
    //   1/2 * sigma^2 * d = 0.14 per year at 3x, and g1 = 0.051 per year at 1x.
    //   With d = 252 sessions: 4.5 sigma^2 = 0.14 -> sigma = 0.01518;
    //   1x: m d - 1/2 sigma^2 d = 0.051 -> m d = 0.080 -> m = 0.000317 per session.
    let mu = 0.000317;
    let sigma = 0.01518;
    let sessions = 100_000;
    for c in long_leverage(2026, sessions, mu, sigma) {
        println!(
            "L = {:.0}x: arithmetic mean = {:+.6}/session, measured growth = {:+.6}/session, drag = {:+.6}/session ({:+.4}/year), wealth in {} sessions = {:.3e}",
            c.leverage,
            c.arithmetic_mean,
            c.time_growth,
            c.drag,
            c.drag * 252.0,
            sessions,
            c.final_wealth
        );
    }
    // the same cohort over a short horizon, to see the median against the mean
    for sessions in [2_000usize, 10_000] {
        println!("horizon of {} sessions:", sessions);
        for c in long_leverage(1909 + sessions as u64, sessions, mu, sigma) {
            println!(
                "  L = {:.0}x: measured growth = {:+.6}/session, final wealth = {:.3e}",
                c.leverage, c.time_growth, c.final_wealth
            );
        }
    }
    println!(
        "the 1987 crash (-22.6% in one day) takes {:.1}% of the capital at 3x, {:.1}% at 1x; ruin starts at L = {:.1}",
        100.0 * (1.0 - (1.0 - 3.0 * 0.226)),
        22.6,
        1.0 / 0.226
    );
    // The same calculation with two absorbing shocks planted (19/10/1987 and
    // the 1929-32 run), which the synthetic series does not have.
    let mut rng = SplitMix64::new(1987);
    let mut returns: Vec<f64> = (0..20_000).map(|_| mu + sigma * rng.normal()).collect();
    returns[5_000] = -0.226;
    for i in 5_100..5_140 {
        returns[i] = -0.03;
    }
    println!("with a -22.6% crash in one day and 40 days of -3% (20 thousand sessions):");
    for l in [1.0, 2.0, 3.0] {
        let mut wealth = 1.0;
        let mut sum_log = 0.0;
        let mut ruin = false;
        for r in &returns {
            let factor = 1.0 + l * r;
            if factor <= 0.0 {
                ruin = true;
                break;
            }
            wealth *= factor;
            sum_log += factor.ln();
        }
        println!(
            "  L = {:.0}x: measured growth = {:+.6}/session, final wealth = {:.3e}{}",
            l,
            sum_log / returns.len() as f64,
            wealth,
            if ruin { "  RUIN" } else { "" }
        );
    }
    println!();
}

pub fn run() {
    peters_coin();
    insurance_seller();
    wild_tail();
    dow_leverage();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peters_coin_grows_in_the_ensemble_and_decays_in_time() {
        let (ensemble, time) = peters_coin_with_f(0.5, 1.0);
        assert!(ensemble > 0.0);
        assert!(time < 0.0);
        let f_optimal = kelly_peters_coin(0.5);
        let (_, time_kelly) = peters_coin_with_f(0.5, f_optimal);
        assert!(time_kelly > 0.0);
    }
}
