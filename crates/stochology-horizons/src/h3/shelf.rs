//! SHELF elicitation: the fit of the prior by the quartiles and the controls of the protocol.
//!
//! The `empirical/p12-shelf` protocol elicits median, quartiles and extremes, and then fits a Normal by
//! least squares on q25/q50/q75 (the family of the four gaussian domains). Here live the fit, the
//! 90% width elicited against the implied one (the overconfidence control, ratio >= 0.75), the maximum
//! error on the quartiles and the reading of the anchor.
//!
//! Declared and not implemented: the Beta fit of the `h1-real/c` domain (frequency) and the odds-ratio
//! scale of the `h2/direito_prisoes` domain, which enter when those domains are elicited.

/// z of the standard normal at 0.25 and at 0.75: the fit uses the three central quartiles.
pub const Z_QUARTILE: f64 = 0.674_489_750_196_081_7;

/// z of the standard normal at 0.95: it enters the implied 90% width.
pub const Z_CENTRAL_90: f64 = 1.644_853_626_951_472_2;

/// Minimum ratio declared by the protocol (overconfidence control).
pub const MIN_WIDTH_RATIO: f64 = 0.75;

/// Expected median in the anchor reading with S = the expert's own median.
pub const EXPECTED_ANCHOR: f64 = 0.50;

/// Declared tolerance of the anchor.
pub const ANCHOR_TOLERANCE: f64 = 0.10;

/// Maximum tolerated shift of the median, in units of the sd of the prior itself.
pub const MAX_SHIFT_SD: f64 = 0.20;

/// The Normal prior that the three central quartiles imply.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalPrior {
    pub mean: f64,
    pub sd: f64,
    /// The largest deviation between an elicited quartile and the quartile that the fitted Normal itself predicts.
    pub max_quartile_error: f64,
}

/// Fits the Normal by least squares on the three central quartiles.
pub fn fit_normal(q25: f64, q50: f64, q75: f64) -> NormalPrior {
    // points (z, q): (-z, q25), (0, q50), (+z, q75); the least squares gives slope and intercept
    let zs = [-Z_QUARTILE, 0.0, Z_QUARTILE];
    let qs = [q25, q50, q75];
    let n = 3.0_f64;
    let sum_z: f64 = zs.iter().sum();
    let sum_q: f64 = qs.iter().sum();
    let sum_zz: f64 = zs.iter().map(|z| z * z).sum();
    let sum_zq: f64 = zs.iter().zip(qs.iter()).map(|(z, q)| z * q).sum();
    let sd = (sum_zq - sum_z * sum_q / n) / (sum_zz - sum_z * sum_z / n);
    let mean = sum_q / n - sd * sum_z / n;
    let max_quartile_error = zs
        .iter()
        .zip(qs.iter())
        .map(|(z, q)| (q - (mean + sd * z)).abs())
        .fold(0.0_f64, f64::max);
    NormalPrior {
        mean,
        sd,
        max_quartile_error,
    }
}

/// Elicited 90% width: the distance between the two extremes.
pub fn elicited_width_90(q05: f64, q95: f64) -> f64 {
    q95 - q05
}

/// The 90% width that the fitted Normal itself implies.
pub fn implied_width_90(sd: f64) -> f64 {
    2.0 * Z_CENTRAL_90 * sd
}

/// The ratio of the overconfidence control.
pub fn width_ratio_90(elicited_width: f64, sd: f64) -> f64 {
    let implied = implied_width_90(sd);
    if implied <= 0.0 {
        return f64::NAN;
    }
    elicited_width / implied
}

pub fn overconfidence_ok(ratio: f64) -> bool {
    ratio >= MIN_WIDTH_RATIO
}

/// The anchor: `P(theta > S)` with `S` equal to the expert's own median must give 0.50 +- 0.10.
pub fn anchor_at_median_ok(p: f64) -> bool {
    (p - EXPECTED_ANCHOR).abs() <= ANCHOR_TOLERANCE
}

/// The probability that the quantity stays above `S` in the expert's own Normal.
///
/// It is the coherence reading of the anchor: if the expert answers `P(theta > S)` far from what
/// his own prior predicts, the prior and the anchor tell different stories, and both are recorded.
pub fn p_above(s: f64, mean: f64, sd: f64) -> f64 {
    if sd <= 0.0 {
        return f64::NAN;
    }
    1.0 - phi((s - mean) / sd)
}

/// Cumulative distribution of the standard normal: `Phi(z) = 0.5 (1 + erf(z / sqrt(2)))`, with the erf by
/// the Abramowitz-Stegun 7.1.26 approximation (error < 1.5e-7).
///
/// The division by the square root of 2 belongs to `Phi` itself: without it the zeroth would sit at 0.76 instead of 0.69, and the
/// anchor forecast would come out inflated. The test `phi_matches_the_table` keeps that.
fn phi(z: f64) -> f64 {
    let x = z / std::f64::consts::SQRT_2;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let polynomial = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    let erf = sign * (1.0 - polynomial * (-x * x).exp());
    0.5 * (1.0 + erf)
}

/// The shift of the median, in units of the sd, when the expert's median is moved.
pub fn shift_in_sd(new_median: f64, original_median: f64, sd: f64) -> f64 {
    if sd <= 0.0 {
        return f64::NAN;
    }
    (new_median - original_median).abs() / sd
}

pub fn shift_within_limit(shift_sd: f64) -> bool {
    shift_sd <= MAX_SHIFT_SD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovers_the_normal_that_generated_the_quartiles() {
        let (mean, sd) = (4.5_f64, 1.853_253_2_f64);
        let p = fit_normal(mean - Z_QUARTILE * sd, mean, mean + Z_QUARTILE * sd);
        assert!((p.mean - mean).abs() < 1e-12, "mean {}", p.mean);
        assert!((p.sd - sd).abs() < 1e-12, "sd {}", p.sd);
        assert!(
            p.max_quartile_error < 1e-12,
            "error {}",
            p.max_quartile_error
        );
    }

    #[test]
    fn asymmetric_quartiles_leave_a_visible_error() {
        let p = fit_normal(3.0, 4.5, 5.75);
        assert!(p.max_quartile_error > 0.0);
        assert!(p.max_quartile_error < 0.3, "error {}", p.max_quartile_error);
        assert!(p.sd > 0.0);
    }

    #[test]
    fn the_implied_width_matches_the_elicited_one_when_the_extremes_are_from_the_normal() {
        let sd = 1.5_f64;
        let q05 = 4.5 - Z_CENTRAL_90 * sd;
        let q95 = 4.5 + Z_CENTRAL_90 * sd;
        let ratio = width_ratio_90(elicited_width_90(q05, q95), sd);
        assert!((ratio - 1.0).abs() < 1e-12, "ratio {ratio}");
        assert!(overconfidence_ok(ratio));
    }

    #[test]
    fn narrow_extremes_fail_in_the_control() {
        let sd = 2.0_f64;
        let implied = implied_width_90(sd);
        let narrow = 0.5 * implied;
        let ratio = width_ratio_90(narrow, sd);
        assert!(!overconfidence_ok(ratio), "ratio {ratio}");
        assert!((ratio - 0.5).abs() < 1e-12);
    }

    #[test]
    fn the_anchor_and_the_shift_have_declared_limits() {
        assert!(anchor_at_median_ok(0.50));
        assert!(anchor_at_median_ok(0.59));
        assert!(!anchor_at_median_ok(0.65));
        assert!(shift_within_limit(0.19));
        assert!(!shift_within_limit(0.21));
        assert!((shift_in_sd(4.9, 4.5, 2.0) - 0.2).abs() < 1e-12);
    }

    #[test]
    fn the_anchor_predicts_what_the_prior_says() {
        let p = fit_normal(1.75, 3.25, 4.75);
        // at the median itself the prior predicts exact 0.50
        let at_median = p_above(p.mean, p.mean, p.sd);
        assert!(
            (at_median - 0.5).abs() < 1e-7,
            "P(theta > median): {at_median}"
        );
        // at 4.5 the fitted prior predicts 0.2870; the expert declared 0.29, within 0.003
        let predicted = p_above(4.5, p.mean, p.sd);
        assert!(
            (predicted - 0.2870).abs() < 0.001,
            "P(theta > 4.5) by the prior: {predicted}"
        );
        // the 0.50 +- 0.10 rule holds at the median itself, and there it closes exactly
        assert!(
            anchor_at_median_ok(at_median),
            "at the median itself: {at_median}"
        );
        // at 4.5 there is no 0.50 rule: what is read is the coherence between the declared and the prior itself
        assert!(
            (0.29 - predicted).abs() < 0.01,
            "the declared and the predicted agree: {predicted}"
        );
    }

    #[test]
    fn phi_matches_the_table() {
        // the values of the standard normal table, item by item
        for (z, expected) in [
            (0.0_f64, 0.5_f64),
            (0.5, 0.691_462),
            (1.0, 0.841_345),
            (1.96, 0.975_002),
            (-1.0, 0.158_655),
        ] {
            let obtained = phi(z);
            assert!(
                (obtained - expected).abs() < 1e-6,
                "Phi({z}) = {obtained}, table {expected}"
            );
        }
    }

    #[test]
    fn the_symmetric_fit_of_the_mean_at_q50() {
        let p = fit_normal(3.25, 4.5, 5.75);
        assert!((p.mean - 4.5).abs() < 1e-12, "mean {}", p.mean);
        let expected = (Z_QUARTILE * (5.75 - 3.25)) / (2.0 * Z_QUARTILE * Z_QUARTILE);
        assert!((p.sd - expected).abs() < 1e-12);
    }
}
