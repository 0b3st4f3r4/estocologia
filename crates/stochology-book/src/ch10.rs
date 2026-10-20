//! Chapter 10 — time and ruin: ensemble mean against time mean.
//!
//! Self-contained laboratory, no external dependencies.
//! Usage: cargo run --release

// ---------------------------------------------------------------- measured output
#[derive(Clone)]
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64(stochology_core::rng::offset_seed(seed))
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
    TwiceKelly,
    Kelly,
    HalfKelly,
}

impl Stake {
    pub fn fraction(&self) -> f64 {
        match self {
            Stake::All => 1.0,
            Stake::TwiceKelly => 2.0 * kelly_peters_coin(0.5),
            Stake::Kelly => kelly_peters_coin(0.5),
            Stake::HalfKelly => 0.5 * kelly_peters_coin(0.5),
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Stake::All => "all-in (f = 1.00)",
            Stake::TwiceKelly => "twice Kelly (f = 0.50)",
            Stake::Kelly => "full Kelly (f = 0.25)",
            Stake::HalfKelly => "half Kelly (f = 0.125)",
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
/// Tolerance on the log of the wealth below which a trajectory counts as back at the start.
const LOG_TIE_TOLERANCE: f64 = 1e-9;

/// Median of the maximum of `n` draws from a Pareto with P(X > x) = x^(-alpha):
/// P(max <= m) = (1 - m^(-alpha))^n = 1/2.
pub fn median_maximum(n: u64, alpha: f64) -> f64 {
    let tail = -(-std::f64::consts::LN_2 / n as f64).exp_m1();
    tail.powf(-1.0 / alpha)
}

/// Share of the true mean carried by values above m: m^(1 - alpha), evaluated at the median maximum.
/// Until 2026-10-19 the laboratory printed n^(1 - alpha), which places the maximum at n itself.
pub fn share_of_mean_above_median_maximum(n: u64, alpha: f64) -> f64 {
    median_maximum(n, alpha).powf(1.0 - alpha)
}

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
            // Strictly above the start, in log space with a tolerance: at f = 0.50 the factors 1.25 and
            // 0.8 multiply to exactly 1, so k heads and k tails return to the start in exact arithmetic,
            // and a bare `> 1.0` let the rounding of the product decide each tie (until 2026-10-19).
            if *m <= steps && curve[*m].ln() > LOG_TIE_TOLERANCE {
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

/// Optimal Kelly fraction for a binary bet that gains `b` with chance p and loses `a`
/// with chance 1 - p: maximising p ln(1 + f b) + (1 - p) ln(1 - f a) gives
/// f* = p/a - (1 - p)/b, the edge (p b - (1 - p) a) divided by the product a b.
pub fn kelly_binary(p: f64, b: f64, a: f64) -> f64 {
    p / a - (1.0 - p) / b
}

/// The Peters coin (heads +50%, tails -40%, chance 1/2 each): f* = 0.5/0.4 - 0.5/0.5 = 0.25.
/// Twice that fraction, 0.50, has growth exactly zero; above it, the trajectory decays.
pub fn kelly_peters_coin(p: f64) -> f64 {
    kelly_binary(p, 0.5, 0.4)
}

/// Ensemble mean and time mean of the Peters coin with the fraction f.
pub fn peters_coin_with_f(p: f64, f: f64) -> (f64, f64) {
    let expectation = 1.0 + f * (p * 1.5 + (1.0 - p) * 0.6 - 1.0);
    let log_time = (1.0 - p) * (1.0 - 0.4 * f).ln() + p * (1.0 + 0.5 * f).ln();
    (expectation, log_time)
}

// ------------------------------------------------- the insurance seller

/// The insurance seller earns 1% per week and, with chance q, loses 95% of what is staked.
pub const SELLER_GAIN: f64 = 0.01;
pub const SELLER_LOSS: f64 = 0.95;

/// Time growth of whoever stakes everything on the insurance seller's strategy.
pub fn weekly_rate(q: f64) -> f64 {
    (1.0 - q) * (1.0 + SELLER_GAIN).ln() + q * (1.0 - SELLER_LOSS).ln()
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
        // Kelly for a gain of 1% against a loss of 95%, capped at f = 1 (no borrowing).
        let mut f = kelly_binary(1.0 - q_hat, SELLER_GAIN, SELLER_LOSS).clamp(0.0, 1.0);
        if rule == Rule::HalfLaplace {
            f *= 0.5;
        }
        if f >= 0.999 {
            all_in += 1;
        }
        let mut wealth = 1.0;
        for _ in 0..steps {
            if rng.uniform() < q {
                wealth *= 1.0 - f * SELLER_LOSS;
            } else {
                wealth *= 1.0 + f * SELLER_GAIN;
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
    pub share_of_sum_from_the_largest: f64,
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
        fraction_dominated_by_max += max / (mean * n as f64);
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
        share_of_sum_from_the_largest: fraction_dominated_by_max / repetitions as f64,
    }
}

/// Hill estimator with the `k` largest values over the threshold x_(k+1):
/// alpha_hat = k / sum ln(x_(i) / x_(k+1)), the definition of Chapter 14 and of the tail lens.
/// Until 2026-10-20 the threshold was x_(k), whose own term is ln(1) = 0, so k divided a sum of
/// k - 1 terms (`labs --oracles`, Appendix E).
pub fn hill(xs: &[f64], k: usize) -> f64 {
    stochology_core::hill_at_cutoff(xs, k, k)
}

/// The Pareto sample on which the laboratory reads Hill.
pub fn hill_sample(seed: u64, n: usize, alpha: f64) -> Vec<f64> {
    let mut rng = SplitMix64::new(seed);
    (0..n).map(|_| rng.pareto(alpha)).collect()
}

/// Generates a Pareto sample and returns Hill for several k, with the asymptotic
/// interval alpha_hat (1 +- 1.96/sqrt(k)).
pub fn hill_by_k(seed: u64, n: usize, alpha: f64, ks: &[usize]) -> Vec<(usize, f64, f64, f64)> {
    let xs = hill_sample(seed, n, alpha);
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
    let sample_mean = returns.iter().sum::<f64>() / sessions as f64;
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
            // The drag is measured against the arithmetic mean of the returns actually drawn:
            // measured against the population mu it would carry L times the sampling error
            // of the mean, which over 100 000 sessions is of the size of the drag itself.
            Portfolio {
                leverage: *l,
                arithmetic_mean: *l * sample_mean,
                time_growth: sum_log / sessions as f64,
                drag: *l * sample_mean - sum_log / sessions as f64,
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
    let f_optimal = kelly_peters_coin(0.5);
    let (_, g_optimal) = peters_coin_with_f(0.5, f_optimal);
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
        "full Kelly (heads +50%, tails -40%): f* = p/0.4 - (1-p)/0.5 = {:.4} of the wealth; even-odds bet with p = 0.6: 2p - 1 = {:.2}",
        f_optimal,
        kelly_even_odds(0.6)
    );
    println!(
        "time growth per round: f* = {:+.6} | 2f* = {:+.6} | f = 0.70 {:+.6} | f = 1 {:+.6}",
        g_optimal,
        peters_coin_with_f(0.5, 2.0 * f_optimal).1,
        peters_coin_with_f(0.5, 0.70).1,
        peters_coin_with_f(0.5, 1.0).1
    );
    println!();
    let stakes = [
        Stake::All,
        Stake::TwiceKelly,
        Stake::Kelly,
        Stake::HalfKelly,
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
        "== Chapter 10 :: the insurance seller (earns 1% per week, loses 95% with q = 0.004) =="
    );
    let q = 0.004;
    let g = weekly_rate(q);
    println!(
        "time growth of the all-in: {:+.5} per week ({})",
        g,
        if g < 0.0 {
            "negative: the typical all-in trajectory decays"
        } else {
            "positive: the all-in grows in time"
        }
    );
    println!(
        "Kelly fraction with the true q: {:.3} | with Laplace after 250 clean weeks: {:.3} | with the raw frequency 0/250: capped at 1",
        kelly_binary(1.0 - q, SELLER_GAIN, SELLER_LOSS),
        kelly_binary(1.0 - 1.0 / 252.0, SELLER_GAIN, SELLER_LOSS)
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
        "share of each sample's sum that comes from its single largest value (n = 1 000 000): {:.3}",
        pareto_samples(99, 1_000_000, 1.1, 200).share_of_sum_from_the_largest
    );
    println!(
        "in the median, {:.2}% of the true mean comes from values above the maximum of a sample of one million (median maximum {:.0})",
        100.0 * share_of_mean_above_median_maximum(1_000_000, 1.1),
        median_maximum(1_000_000, 1.1)
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
        "reading: a Gaussian model fitted to this tail underestimates the loss in every repetition"
    );
    println!();
}

/// The per-session drift and volatility calibrated by the two numbers of the Dow
/// (h1-real/README.md): a drag of 0.14 per year at 3x and a growth of 0.051 per year at 1x,
/// with d = 252 sessions per year.
///   3x: 1/2 * 9 * sigma^2 * d = 0.14 -> sigma^2 = 0.14 / 1134 -> sigma = 0.01111 per session;
///   1x: (m - 1/2 sigma^2) d = 0.051 -> m d = 0.0666 -> m = 0.000264 per session.
/// Rectified 2026-10-18: the earlier calibration used sigma = 0.01518 and m = 0.000317, which
/// inflated the drag and put 3x above 2x. Its comment wrote 4.5 sigma^2 = 0.14 -> 0.01518, but
/// that equation gives 0.1764 and, with d, 0.01111: the earlier sigma follows from neither, and
/// its origin is declared not reconstructable (Appendix E, 2026-10-20).
pub fn dow_calibration() -> (f64, f64) {
    (0.000264, 0.01111)
}

fn dow_leverage() {
    println!("== Chapter 10 :: volatility drag at 1x, 2x and 3x ==");
    let (mu, sigma) = dow_calibration();
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

// ------------------------------------------- extremes against the Gaussian

/// Standard normal quantile by the rational approximation of Acklam (relative error below 1.2e-9),
/// accurate in the far tail where the erf of Abramowitz and Stegun, used elsewhere, is not.
#[allow(clippy::excessive_precision)]
pub fn normal_quantile_tail(p: f64) -> f64 {
    const A: [f64; 6] = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383577518672690e+02,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];
    const B: [f64; 5] = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];
    const C: [f64; 6] = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];
    const D: [f64; 4] = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];
    let q_poly = |q: f64| {
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    if p < 0.02425 {
        q_poly((-2.0 * p.ln()).sqrt())
    } else if p > 1.0 - 0.02425 {
        -q_poly((-2.0 * (1.0 - p).ln()).sqrt())
    } else {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    }
}

pub const DAYS_PER_YEAR: f64 = 252.0;

/// The 1-in-100-year daily loss of a loss with P(X > x) = x^(-alpha), x >= 1, exact.
pub fn exact_hundred_year_loss(alpha: f64) -> f64 {
    (100.0 * DAYS_PER_YEAR).powf(1.0 / alpha)
}

/// The same level for a Gaussian with the exact moments of that Pareto (alpha > 2).
pub fn gaussian_hundred_year_loss_exact(alpha: f64) -> f64 {
    let mean = alpha / (alpha - 1.0);
    let sd = (alpha / ((alpha - 1.0).powi(2) * (alpha - 2.0))).sqrt();
    mean + sd * normal_quantile_tail(1.0 - 1.0 / (100.0 * DAYS_PER_YEAR))
}

pub struct HundredYear {
    pub gaussian: f64,
    pub extremes: f64,
    /// The threshold itself, which is what the extrapolation gives when its ratio is left out.
    pub threshold: f64,
}

/// One sample of `n` daily losses: the Gaussian fitted by mean and standard deviation, and the
/// extremes level of Weissman, x_(k+1) * (k / (n p))^(1/alpha_hat), with Hill's alpha_hat at k.
pub fn hundred_year_from_sample(xs: &[f64], k: usize) -> HundredYear {
    let n = xs.len() as f64;
    let mean = xs.iter().sum::<f64>() / n;
    let sd = (xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
    let mut v = xs.to_vec();
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let alpha_hat = hill(xs, k);
    let p = 1.0 / (100.0 * DAYS_PER_YEAR);
    let threshold = v[k];
    HundredYear {
        gaussian: mean + sd * normal_quantile_tail(1.0 - p),
        extremes: threshold * (k as f64 / (n * p)).powf(1.0 / alpha_hat),
        threshold,
    }
}

pub struct ExtremesRun {
    pub truth: f64,
    pub gaussian_exact: f64,
    pub gaussian_median: f64,
    pub gaussian_below: f64,
    pub extremes_median: f64,
    pub extremes_q25: f64,
    pub extremes_q75: f64,
    pub extremes_below: f64,
}

fn quantile_of(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// `repetitions` samples of ten years (2 520 days) of Pareto losses with tail alpha.
pub fn extremes_run(seed: u64, alpha: f64, repetitions: usize, k: usize) -> ExtremesRun {
    let mut rng = SplitMix64::new(seed);
    let truth = exact_hundred_year_loss(alpha);
    let (mut g, mut e) = (Vec::new(), Vec::new());
    for _ in 0..repetitions {
        let xs: Vec<f64> = (0..2_520).map(|_| rng.pareto(alpha)).collect();
        let h = hundred_year_from_sample(&xs, k);
        g.push(h.gaussian);
        e.push(h.extremes);
    }
    let below = |v: &[f64]| v.iter().filter(|x| **x < truth).count() as f64 / v.len() as f64;
    ExtremesRun {
        truth,
        gaussian_exact: gaussian_hundred_year_loss_exact(alpha),
        gaussian_below: below(&g),
        extremes_below: below(&e),
        gaussian_median: quantile_of(&mut g, 0.5),
        extremes_median: quantile_of(&mut e, 0.5),
        extremes_q25: quantile_of(&mut e, 0.25),
        extremes_q75: quantile_of(&mut e, 0.75),
    }
}

fn extremes_against_gaussian() {
    println!(
        "== Chapter 10 :: the 1-in-100-year loss, extremes against the Gaussian (alpha = 3, 2 520 days, 200 repetitions) =="
    );
    let r = extremes_run(1010, 3.0, 200, 100);
    println!(
        "  exact level: {:.2}; Gaussian with the exact moments: {:.2}",
        r.truth, r.gaussian_exact
    );
    println!(
        "  Gaussian fitted to each sample: median {:.2}, below the exact level in {:.0}% of the samples",
        r.gaussian_median,
        100.0 * r.gaussian_below
    );
    println!(
        "  extremes (Hill k = 100, Weissman): median {:.2}, quartiles [{:.2}; {:.2}], below the exact level in {:.0}% of the samples",
        r.extremes_median,
        r.extremes_q25,
        r.extremes_q75,
        100.0 * r.extremes_below
    );
    println!();
}

pub fn run() {
    peters_coin();
    insurance_seller();
    wild_tail();
    extremes_against_gaussian();
    dow_leverage();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Oracle: z(1 - 1/25200) = 3.946309 (reference value of the inverse normal).
    #[test]
    fn the_tail_quantile_is_accurate_where_the_erf_is_not() {
        assert!((normal_quantile_tail(1.0 - 1.0 / 25_200.0) - 3.946_309_3).abs() < 1e-6);
        assert!(normal_quantile_tail(0.5).abs() < 1e-9);
        assert!((normal_quantile_tail(0.975) - 1.959_963_985).abs() < 1e-6);
    }

    /// Oracle: P(X > x) = x^-alpha at the exact level equals 1/25200; the Gaussian with the same
    /// moments sits more than five times lower; the extremes level is on target in the median.
    /// Mutant: dropping the ratio k/(n p) returns the threshold, far below the level, and is killed.
    #[test]
    fn extremes_find_the_hundred_year_loss_and_the_gaussian_does_not() {
        let truth = exact_hundred_year_loss(3.0);
        assert!((truth.powf(-3.0) - 1.0 / 25_200.0).abs() < 1e-12);
        assert!(gaussian_hundred_year_loss_exact(3.0) < truth / 5.0);
        let r = extremes_run(5, 3.0, 200, 100);
        assert!(
            (r.extremes_median / truth - 1.0).abs() < 0.25,
            "{}",
            r.extremes_median
        );
        assert!(r.gaussian_below > 0.99);
        assert!(r.extremes_below > 0.2 && r.extremes_below < 0.8);
        let mut rng = SplitMix64::new(5);
        let xs: Vec<f64> = (0..2_520).map(|_| rng.pareto(3.0)).collect();
        assert!(hundred_year_from_sample(&xs, 100).threshold < truth / 5.0);
    }

    #[test]
    fn peters_coin_grows_in_the_ensemble_and_decays_in_time() {
        let (ensemble, time) = peters_coin_with_f(0.5, 1.0);
        assert!(ensemble > 0.0);
        assert!(time < 0.0);
        let f_optimal = kelly_peters_coin(0.5);
        let (_, time_kelly) = peters_coin_with_f(0.5, f_optimal);
        assert!(time_kelly > 0.0);
    }

    /// Property: the closed-form Kelly fraction maximises the growth over a dense grid,
    /// and twice it has growth zero for the Peters coin.
    #[test]
    fn kelly_fraction_is_the_argmax_of_growth() {
        let f_star = kelly_peters_coin(0.5);
        assert!((f_star - 0.25).abs() < 1e-12);
        let g = |f: f64| peters_coin_with_f(0.5, f).1;
        let best = (1..2_500)
            .map(|i| i as f64 / 1_000.0)
            .fold(0.0_f64, |acc, f| if g(f) > g(acc) { f } else { acc });
        assert!(
            (best - f_star).abs() < 1e-3,
            "grid argmax {best} against {f_star}"
        );
        assert!(g(2.0 * f_star).abs() < 1e-12);
        for (p, b, a) in [(0.6, 1.0, 1.0), (0.55, 0.3, 0.2), (0.996, 0.01, 0.95)] {
            let f = kelly_binary(p, b, a);
            let h = |x: f64| p * (1.0 + x * b).ln() + (1.0 - p) * (1.0 - x * a).ln();
            assert!(h(f) >= h(f - 1e-3) && h(f) >= h(f + 1e-3));
        }
    }

    /// The drag measured against the sample mean follows L^2 sigma^2 / 2 within 5%.
    #[test]
    fn drag_grows_with_the_square_of_leverage() {
        let sigma = 0.01111;
        for p in long_leverage(2026, 100_000, 0.000264, sigma) {
            let theory = 0.5 * p.leverage * p.leverage * sigma * sigma;
            assert!(
                (p.drag / theory - 1.0).abs() < 0.05,
                "L = {}: {} vs {}",
                p.leverage,
                p.drag,
                theory
            );
        }
    }

    #[test]
    fn the_share_above_the_maximum_uses_the_median_maximum_not_n() {
        let m = super::median_maximum(1_000_000, 1.1);
        // (1 - m^-alpha)^n = 1/2 at the median.
        let cdf = (1.0 - m.powf(-1.1)).powf(1_000_000.0);
        assert!(
            (cdf - 0.5).abs() < 1e-9,
            "cdf at the median maximum = {cdf}"
        );
        let share = super::share_of_mean_above_median_maximum(1_000_000, 1.1);
        assert!((share - 0.27547).abs() < 1e-4, "share = {share}");
        assert!(
            (share - 1e-6f64.powf(0.1)).abs() > 0.02,
            "the old n^(1-alpha) is retired"
        );
    }

    #[test]
    fn twice_kelly_ties_do_not_count_as_above_the_start() {
        // At f = 0.50 five heads and five tails return exactly to the start; in exact arithmetic the
        // share strictly above the start after 10 rounds is P(heads >= 6) = 386/1024 = 0.377.
        let c = super::cohort(7, 20_000, 10, super::Stake::TwiceKelly);
        assert!(
            (c.above_start[0] - 386.0 / 1024.0).abs() < 0.015,
            "above the start = {}",
            c.above_start[0]
        );
    }
}
