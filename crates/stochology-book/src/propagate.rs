//! Three answers to the same question: the propagation of uncertainty through the health post's queue.
//!
//! The mean wait of a day at the post of Chapters 5 and 12 is W(lambda, mu) = 60 lambda / (mu (mu - lambda))
//! minutes, with lambda the arrivals and mu the services per hour. Neither rate is known exactly: from day to day the
//! arrivals vary around 8 per hour and the pace of service around 12. The question the post asks is not the mean
//! wait; it is how often a day's mean wait passes 20 minutes, because the post has promised that this happens on no
//! more than 1 day in 20. Three methods answer:
//!
//! - **plug-in**: the formula at the mean rates, one number and no probability;
//! - **delta**: the formula linearised at the mean, which gives a mean and a standard deviation and, read as a
//!   normal, a probability; its second-order correction moves the mean by half the curvature times the variance;
//! - **Monte Carlo**: the rates drawn from their joint distribution, correlation included, the formula applied to
//!   each draw, and the share of days above 20 minutes counted (a day whose arrivals reach its pace never clears its
//!   queue, and counts as above; the mean and sd are of the days that clear).
//!
//! The uncertainty budget splits the delta variance into the part of each rate and the part of their covariance. The
//! declared rule: when delta and Monte Carlo disagree about the probability that decides by more than half the
//! threshold, the linear answer is not allowed to decide. Deterministic: SplitMix64, seed 2026.

use stochology_core::Rng;

pub const LAMBDA: f64 = 8.0;
pub const MU: f64 = 12.0;
pub const SD_LAMBDA: f64 = 0.6;
pub const SD_MU: f64 = 0.8;
pub const LIMIT_MIN: f64 = 20.0;
pub const PROMISE: f64 = 0.05;
pub const DRAWS: usize = 400_000;
pub const SEED: u64 = 2026;

/// Mean wait in the queue of an M/M/1 post, in minutes; infinite when the arrivals reach the pace.
pub fn wait(lambda: f64, mu: f64) -> f64 {
    if lambda >= mu || mu <= 0.0 || lambda < 0.0 {
        f64::INFINITY
    } else {
        60.0 * lambda / (mu * (mu - lambda))
    }
}

/// The gradient of the wait: (dW/dlambda, dW/dmu).
pub fn gradient(lambda: f64, mu: f64) -> (f64, f64) {
    let gap = mu - lambda;
    (
        60.0 / (gap * gap),
        -60.0 * lambda * (2.0 * mu - lambda) / (mu * gap).powi(2),
    )
}

/// The Hessian of the wait, by central differences: (d2/dlambda2, d2/dmu2, d2/dlambda dmu).
pub fn hessian(lambda: f64, mu: f64) -> (f64, f64, f64) {
    let h = 1e-4;
    let w = |l: f64, m: f64| wait(l, m);
    let ll = (w(lambda + h, mu) - 2.0 * w(lambda, mu) + w(lambda - h, mu)) / (h * h);
    let mm = (w(lambda, mu + h) - 2.0 * w(lambda, mu) + w(lambda, mu - h)) / (h * h);
    let lm = (w(lambda + h, mu + h) - w(lambda + h, mu - h) - w(lambda - h, mu + h)
        + w(lambda - h, mu - h))
        / (4.0 * h * h);
    (ll, mm, lm)
}

fn normal_sf(z: f64) -> f64 {
    // complementary error function, Abramowitz and Stegun 7.1.26, accurate to 1.5e-7
    let x = z / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.327_591_1 * x.abs());
    let y = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    let erfc = y * (-x * x).exp();
    let erfc = if x >= 0.0 { erfc } else { 2.0 - erfc };
    0.5 * erfc
}

fn gaussian(rng: &mut Rng) -> f64 {
    let u = |r: &mut Rng| ((r.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let (a, b) = (u(rng), u(rng));
    (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
}

/// The three answers and the budget for one correlation between the two rates.
#[derive(Debug, Clone, PartialEq)]
pub struct Answers {
    pub rho: f64,
    pub plug_in: f64,
    pub delta_mean: f64,
    pub second_order_mean: f64,
    pub delta_sd: f64,
    pub delta_p: f64,
    pub mc_mean: f64,
    pub mc_median: f64,
    pub mc_sd: f64,
    pub mc_q95: f64,
    pub mc_p: f64,
    pub mc_p_sd: f64,
    pub budget: (f64, f64, f64),
    pub unstable_days: usize,
    pub linear_may_decide: bool,
}

pub fn answers(rho: f64, draws: usize, seed: u64) -> Answers {
    let plug_in = wait(LAMBDA, MU);
    let (gl, gm) = gradient(LAMBDA, MU);
    let cov = rho * SD_LAMBDA * SD_MU;
    let budget = (
        gl * gl * SD_LAMBDA * SD_LAMBDA,
        gm * gm * SD_MU * SD_MU,
        2.0 * gl * gm * cov,
    );
    let var = budget.0 + budget.1 + budget.2;
    let delta_sd = var.max(0.0).sqrt();
    let (hll, hmm, hlm) = hessian(LAMBDA, MU);
    let second_order_mean =
        plug_in + 0.5 * (hll * SD_LAMBDA * SD_LAMBDA + hmm * SD_MU * SD_MU + 2.0 * hlm * cov);
    let delta_p = normal_sf((LIMIT_MIN - plug_in) / delta_sd);

    let mut rng = Rng::new(seed);
    let mut ws = Vec::with_capacity(draws);
    let root = (1.0 - rho * rho).max(0.0).sqrt();
    for _ in 0..draws {
        let (z1, z2) = (gaussian(&mut rng), gaussian(&mut rng));
        let lambda = LAMBDA + SD_LAMBDA * z1;
        let mu = MU + SD_MU * (rho * z1 + root * z2);
        ws.push(wait(lambda, mu));
    }
    ws.sort_by(f64::total_cmp);
    let finite: Vec<f64> = ws.iter().copied().filter(|w| w.is_finite()).collect();
    let mc_mean = finite.iter().sum::<f64>() / finite.len() as f64;
    let mc_sd = (finite.iter().map(|w| (w - mc_mean).powi(2)).sum::<f64>()
        / (finite.len() as f64 - 1.0))
        .sqrt();
    let q = |p: f64| ws[((p * (ws.len() - 1) as f64).round()) as usize];
    let above = ws.iter().filter(|w| **w > LIMIT_MIN).count() as f64 / ws.len() as f64;
    Answers {
        rho,
        plug_in,
        delta_mean: plug_in,
        second_order_mean,
        delta_sd,
        delta_p,
        mc_mean,
        mc_median: q(0.5),
        mc_sd,
        mc_q95: q(0.95),
        mc_p: above,
        mc_p_sd: (above * (1.0 - above) / ws.len() as f64).sqrt(),
        budget,
        unstable_days: ws.iter().filter(|w| w.is_infinite()).count(),
        linear_may_decide: (delta_p - above).abs() <= 0.5 * PROMISE,
    }
}

fn verdict(p: f64) -> &'static str {
    if p > PROMISE {
        "second nurse"
    } else {
        "promise kept"
    }
}

pub fn run() {
    println!("== Three answers to the same question: the wait of a day at the health post ==");
    println!(
        "W = 60 lambda / (mu (mu - lambda)) min | lambda ~ N({LAMBDA}, {SD_LAMBDA}^2) per hour | mu ~ N({MU}, {SD_MU}^2) per hour | question: P(W > {LIMIT_MIN} min), promise <= {PROMISE} | {DRAWS} draws, seed {SEED}"
    );
    let (gl, gm) = gradient(LAMBDA, MU);
    println!(
        "sensitivities at the mean: dW/dlambda = {gl:+.3} min per arrival/h, dW/dmu = {gm:+.3} min per service/h"
    );
    for rho in [-0.5, 0.0, 0.5] {
        let a = answers(rho, DRAWS, SEED);
        let total = a.budget.0 + a.budget.1 + a.budget.2;
        println!("\n-- correlation between arrivals and pace: {rho:+.1} --");
        println!(
            "  plug-in:     W = {:.2} min | no probability | decision: {}",
            a.plug_in,
            verdict(0.0)
        );
        println!(
            "  delta:       mean {:.2} (second order {:.2}) | sd {:.2} | P(W > {LIMIT_MIN}) = {:.4} | decision: {}",
            a.delta_mean,
            a.second_order_mean,
            a.delta_sd,
            a.delta_p,
            verdict(a.delta_p)
        );
        println!(
            "  Monte Carlo: mean {:.2} | median {:.2} | sd {:.2} | q95 {:.2} | P(W > {LIMIT_MIN}) = {:.4} (MC sd {:.4}) | days that never clear {} | decision: {}",
            a.mc_mean,
            a.mc_median,
            a.mc_sd,
            a.mc_q95,
            a.mc_p,
            a.mc_p_sd,
            a.unstable_days,
            verdict(a.mc_p)
        );
        println!(
            "  budget of the delta variance {:.2} min^2: arrivals {:.2} ({:.0}%), pace {:.2} ({:.0}%), covariance {:+.2} ({:+.0}%)",
            total,
            a.budget.0,
            100.0 * a.budget.0 / total,
            a.budget.1,
            100.0 * a.budget.1 / total,
            a.budget.2,
            100.0 * a.budget.2 / total
        );
        println!(
            "  the linear answer may decide: {} (delta and Monte Carlo differ by {:.4}, {} half the promise, {:.3})",
            a.linear_may_decide,
            (a.delta_p - a.mc_p).abs(),
            if a.linear_may_decide {
                "within"
            } else {
                "more than"
            },
            0.5 * PROMISE
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wait_at_the_mean_is_ten_minutes_and_the_gradient_matches_differences() {
        assert!((wait(8.0, 12.0) - 10.0).abs() < 1e-12);
        let (gl, gm) = gradient(8.0, 12.0);
        let h = 1e-6;
        assert!((gl - (wait(8.0 + h, 12.0) - wait(8.0 - h, 12.0)) / (2.0 * h)).abs() < 1e-5);
        assert!((gm - (wait(8.0, 12.0 + h) - wait(8.0, 12.0 - h)) / (2.0 * h)).abs() < 1e-5);
        assert!(wait(12.0, 12.0).is_infinite());
    }

    #[test]
    fn the_covariance_moves_the_spread_and_not_the_plug_in() {
        let neg = answers(-0.5, 20_000, 1);
        let pos = answers(0.5, 20_000, 1);
        assert_eq!(neg.plug_in, pos.plug_in);
        assert!(
            neg.delta_sd > pos.delta_sd,
            "{} {}",
            neg.delta_sd,
            pos.delta_sd
        );
    }

    #[test]
    fn the_normal_tail_is_right_at_known_points() {
        assert!((normal_sf(0.0) - 0.5).abs() < 1e-7);
        assert!((normal_sf(1.959_964) - 0.025).abs() < 1e-6);
    }
}
