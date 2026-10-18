//! Calibration pretest of the SHELF panel: 20 resolved forecasts, with the four entry criteria.
//!
//! The `empirical/p12-shelf` protocol fixes the design: 20 binary items already resolved, each with
//! `p` in (0, 1), Brier against the climatology of its own pool, curve in equal-mass buckets, ECE and
//! Murphy decomposition, and four approval criteria. This module is the instrument that measures that.
//!
//! Two declared choices:
//!
//! - **The decomposition uses one bucket per distinct value of `p`.** That is what makes the identity
//!   `REL - RES + UNC = Brier` close exactly; with equal-mass buckets it closes only by approximation,
//!   because inside a bucket the `p` differ.
//! - **The ECE threshold does not come from taste: it comes from the null.** `pretest_null` simulates the
//!   perfectly calibrated expert in the same design and returns the distribution of the ECE he would get. A threshold
//!   below the p90 of that null fails whoever is calibrated, and the number is measured in `labs --preteste`.
//!
//! Declared and not implemented: the filling of the 20 items per domain (the pool is desk work, with
//! a checked source) and the reading of the anchor and order controls, which depend on the panel table.

use stochology_core::Rng;

/// Buckets of the curve declared by the protocol.
pub const PROTOCOL_BUCKETS: usize = 10;

/// Threshold declared by the protocol for the ECE.
pub const PROTOCOL_MAX_ECE: f64 = 0.10;

/// Buckets of the **adjusted gate** (declaration 1 of point 12, 2026-10-16): 20 items in 5 equal-mass
/// buckets give 4 items per bucket, and criterion 3 bites again.
pub const ADJUSTED_BUCKETS: usize = 5;

/// Threshold of the adjusted gate: the measured p90 of the ECE of a perfectly calibrated expert in this
/// design (0.2316), rounded up. See `labs --preteste`.
pub const ADJUSTED_MAX_ECE: f64 = 0.25;

/// Minimum bucket size for the systematic-section rule to bite.
pub const MIN_BUCKET_N: usize = 4;

/// Maximum deviation tolerated in a large bucket.
pub const MAX_BUCKET_DEVIATION: f64 = 0.25;

fn uniform(rng: &mut Rng) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

/// One bucket of the curve: how many items, the forecast mean and the observed rate.
#[derive(Debug, Clone, PartialEq)]
pub struct Bucket {
    pub index: usize,
    pub n: usize,
    pub forecast_mean: f64,
    pub observed_rate: f64,
}

impl Bucket {
    pub fn deviation(&self) -> f64 {
        self.observed_rate - self.forecast_mean
    }
}

/// The Murphy decomposition: reliability, resolution and uncertainty of the climatology.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Murphy {
    pub rel: f64,
    pub res: f64,
    pub unc: f64,
}

/// The declared reason for a rejection in the pretest.
#[derive(Debug, Clone, PartialEq)]
pub enum Rejection {
    /// Systematic bucket by the protocol rule as written (fixed deviation threshold).
    SystematicBucketByFixedThreshold {
        index: usize,
        n: usize,
        deviation: f64,
    },
    /// Systematic bucket by the corrected rule (declaration 3): binomial test with Bonferroni.
    SystematicBucketByTest {
        index: usize,
        n: usize,
        p_with_bonferroni: f64,
    },
    NonPositiveSkill {
        skill: f64,
    },
    EceAboveLimit {
        ece: f64,
        limit: f64,
    },
    SystematicBucket {
        index: usize,
        n: usize,
        deviation: f64,
    },
    NonMonotoneCurve {
        mean_of_low_bucket: f64,
        mean_of_high_bucket: f64,
    },
}

/// What the pretest returns: the measurements and the verdict.
#[derive(Debug, Clone, PartialEq)]
pub struct Pretest {
    pub n: usize,
    pub brier: f64,
    pub brier_climate: f64,
    pub skill: f64,
    pub ece: f64,
    pub buckets: Vec<Bucket>,
    pub murphy: Murphy,
    pub approved: bool,
    pub rejections: Vec<Rejection>,
}

/// Indices with `p` outside (0, 1): the protocol refuses exact 0 and 1.
pub fn invalid_probabilities(confidence: &[f64]) -> Vec<usize> {
    confidence
        .iter()
        .enumerate()
        .filter(|(_, p)| !(0.0 < **p && **p < 1.0))
        .map(|(i, _)| i)
        .collect()
}

/// Equal-mass buckets: sorts by `p` and cuts into `buckets` groups of the same size.
pub fn equal_mass_buckets(confidence: &[f64], outcome: &[bool], buckets: usize) -> Vec<Bucket> {
    if confidence.len() != outcome.len() || confidence.is_empty() || buckets == 0 {
        return Vec::new();
    }
    let mut order: Vec<usize> = (0..confidence.len()).collect();
    order.sort_by(|a, b| {
        confidence[*a]
            .partial_cmp(&confidence[*b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut bucket_list = Vec::with_capacity(buckets);
    let n = order.len();
    for index in 0..buckets {
        let start = index * n / buckets;
        let end = (index + 1) * n / buckets;
        if start >= end {
            continue;
        }
        let group = &order[start..end];
        let forecast_mean = group.iter().map(|i| confidence[*i]).sum::<f64>() / group.len() as f64;
        let hits = group.iter().filter(|i| outcome[**i]).count() as f64;
        bucket_list.push(Bucket {
            index,
            n: group.len(),
            forecast_mean,
            observed_rate: hits / group.len() as f64,
        });
    }
    bucket_list
}

/// A bucket judged by the **binomial test**, not by a fixed threshold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestedBucket {
    pub index: usize,
    pub n: usize,
    pub hits: usize,
    pub forecast_mean: f64,
    /// Two-sided p of the binomial test for the observed number of hits, given the forecast mean.
    pub two_sided_p: f64,
    /// The same p with the Bonferroni correction for the number of buckets.
    pub p_with_bonferroni: f64,
}

impl TestedBucket {
    pub fn is_systematic(self, alfa: f64) -> bool {
        self.p_with_bonferroni < alfa
    }
}

fn binomial_pmf(n: usize, p: f64) -> Vec<f64> {
    // pmf by recurrence; p held in (0,1) to avoid dividing by zero
    let p = p.clamp(1e-9, 1.0 - 1e-9);
    let mut pmf = vec![0.0_f64; n + 1];
    pmf[0] = (1.0 - p).powi(n as i32);
    for i in 1..=n {
        let i_f = i as f64;
        let n_f = n as f64;
        pmf[i] = pmf[i - 1] * (n_f - i_f + 1.0) / i_f * p / (1.0 - p);
    }
    pmf
}

/// Criterion 3 by the right test: a bucket is systematic when the number of hits is improbable given the
/// **mean that the expert himself forecast** for it, with Bonferroni over the buckets.
///
/// The protocol fixes `|forecast mean − observed rate| > 0.25` with `n >= 4`, and that rule is noise: measured in
/// the calibrated null of the 20-item design in 5 buckets, it fails 70% of the calibrated ones. The binomial test has no
/// such vice, because the threshold of each bucket comes from its own forecast and its size.
pub fn buckets_by_binomial_test(buckets: &[Bucket]) -> Vec<TestedBucket> {
    let how_many = buckets.len() as f64;
    buckets
        .iter()
        .map(|bucket| {
            let pmf = binomial_pmf(bucket.n, bucket.forecast_mean);
            let hits = (bucket.observed_rate * bucket.n as f64).round() as usize;
            let hits = hits.min(bucket.n);
            let lower_tail: f64 = pmf[..=hits].iter().sum();
            let upper_tail: f64 = pmf[hits..].iter().sum();
            let two_sided_p = (2.0 * lower_tail.min(upper_tail)).min(1.0);
            TestedBucket {
                index: bucket.index,
                n: bucket.n,
                hits,
                forecast_mean: bucket.forecast_mean,
                two_sided_p,
                p_with_bonferroni: (two_sided_p * how_many).min(1.0),
            }
        })
        .collect()
}

/// Expected calibration error in the equal-mass buckets.
pub fn equal_mass_ece(confidence: &[f64], outcome: &[bool], buckets: usize) -> f64 {
    let buckets = equal_mass_buckets(confidence, outcome, buckets);
    if buckets.is_empty() {
        return f64::NAN;
    }
    let n = confidence.len() as f64;
    buckets
        .iter()
        .map(|c| (c.n as f64 / n) * c.deviation().abs())
        .sum()
}

/// Criterion 4 of the protocol, literally: the forecast mean of the top bucket above the bottom one.
///
/// **Measured limit:** with equal-mass buckets the buckets are cut by sorted `p`, so the forecast
/// mean of the top bucket is larger than the bottom one **by construction**. This criterion cannot fail — the
/// test `protocol_criterion_4_cannot_fail` proves it — and that is why the diagnostic that bites is
/// `observed_curve_is_weakly_monotone`, reported beside the criterion and not in its place.
pub fn curve_is_weakly_monotone(buckets: &[Bucket]) -> bool {
    match (buckets.first(), buckets.last()) {
        (Some(low), Some(high)) => high.forecast_mean > low.forecast_mean,
        _ => false,
    }
}

/// Declared diagnostic: the **observed** rate of the top bucket above the bottom one. This one can fail,
/// and it is what reads the shape of the curve when the expert swaps the ends.
pub fn observed_curve_is_weakly_monotone(buckets: &[Bucket]) -> bool {
    match (buckets.first(), buckets.last()) {
        (Some(low), Some(high)) => high.observed_rate > low.observed_rate,
        _ => false,
    }
}

/// Murphy decomposition with one bucket per distinct value of `p` (makes the identity close exactly).
pub fn murphy_decomposition(confidence: &[f64], outcome: &[bool]) -> Murphy {
    let res: Vec<f64> = outcome.iter().map(|y| if *y { 1.0 } else { 0.0 }).collect();
    let (rel, res_m, unc) = stochology_core::murphy_decomposition(confidence, &res);
    Murphy {
        rel,
        res: res_m,
        unc,
    }
}

fn brier(confidence: &[f64], outcome: &[bool]) -> f64 {
    confidence
        .iter()
        .zip(outcome.iter())
        .map(|(p, y)| (p - if *y { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        / confidence.len() as f64
}

/// Runs the four criteria of the protocol **as written** (10 buckets, ECE <= 0.10).
///
/// Declared: this gate was measured and replaced by the adjusted one (`ADJUSTED_BUCKETS`,
/// `ADJUSTED_MAX_ECE`), because the calibrated null fails it in 99.92% of the cases.
pub fn evaluate(confidence: &[f64], outcome: &[bool]) -> Pretest {
    evaluate_with(confidence, outcome, PROTOCOL_BUCKETS, PROTOCOL_MAX_ECE)
}

/// Runs the four criteria with the bucket design and the threshold declared by the caller, with criterion 3
/// by the **corrected rule** (declaration 3 of point 12: two-sided binomial test with Bonferroni).
pub fn evaluate_corrected(
    confidence: &[f64],
    outcome: &[bool],
    buckets: usize,
    max_ece: f64,
) -> Pretest {
    evaluate_inner(confidence, outcome, buckets, max_ece, true)
}

/// The gate in force for the SHELF panel pretest: 5 equal-mass buckets, ECE <= 0.25 and criterion 3
/// by the binomial test (declaration 3).
pub fn evaluate_adjusted(confidence: &[f64], outcome: &[bool]) -> Pretest {
    evaluate_corrected(confidence, outcome, ADJUSTED_BUCKETS, ADJUSTED_MAX_ECE)
}

/// Runs the four criteria with the bucket design and the threshold declared by the caller, with criterion 3
/// by the protocol rule **as written** (fixed threshold).
pub fn evaluate_with(
    confidence: &[f64],
    outcome: &[bool],
    buckets: usize,
    max_ece: f64,
) -> Pretest {
    evaluate_inner(confidence, outcome, buckets, max_ece, false)
}

fn evaluate_inner(
    confidence: &[f64],
    outcome: &[bool],
    buckets: usize,
    max_ece: f64,
    bucket_by_test: bool,
) -> Pretest {
    let n = confidence.len();
    let b = brier(confidence, outcome);
    let mean: f64 = outcome.iter().filter(|y| **y).count() as f64 / n.max(1) as f64;
    let brier_climate = mean * (1.0 - mean);
    let skill = if brier_climate == 0.0 {
        0.0
    } else {
        1.0 - b / brier_climate
    };
    let bucket_list = equal_mass_buckets(confidence, outcome, buckets);
    let ece = equal_mass_ece(confidence, outcome, buckets);
    let mut rejections = Vec::new();
    // The negation is on purpose: NaN must fall into the rejection, not pass the comparison.
    let positive_skill = skill > 0.0;
    if !positive_skill {
        rejections.push(Rejection::NonPositiveSkill { skill });
    }
    let ece_within_limit = ece <= max_ece;
    if !ece_within_limit {
        rejections.push(Rejection::EceAboveLimit {
            ece,
            limit: max_ece,
        });
    }
    for bucket in &bucket_list {
        if bucket.n >= MIN_BUCKET_N && bucket.deviation().abs() > MAX_BUCKET_DEVIATION {
            rejections.push(Rejection::SystematicBucketByFixedThreshold {
                index: bucket.index,
                n: bucket.n,
                deviation: bucket.deviation(),
            });
        }
    }
    if bucket_by_test {
        // declaration 3: criterion 3 counts by the binomial test, and the fixed-threshold rule leaves the verdict
        rejections.retain(|r| !matches!(r, Rejection::SystematicBucketByFixedThreshold { .. }));
        for bucket in buckets_by_binomial_test(&bucket_list) {
            if bucket.is_systematic(0.05) {
                rejections.push(Rejection::SystematicBucketByTest {
                    index: bucket.index,
                    n: bucket.n,
                    p_with_bonferroni: bucket.p_with_bonferroni,
                });
            }
        }
    }
    if !curve_is_weakly_monotone(&bucket_list) {
        let (low, high) = (bucket_list.first(), bucket_list.last());
        rejections.push(Rejection::NonMonotoneCurve {
            mean_of_low_bucket: low.map(|c| c.forecast_mean).unwrap_or(0.0),
            mean_of_high_bucket: high.map(|c| c.forecast_mean).unwrap_or(0.0),
        });
    }
    Pretest {
        n,
        brier: b,
        brier_climate,
        skill,
        ece,
        buckets: bucket_list,
        murphy: murphy_decomposition(confidence, outcome),
        approved: rejections.is_empty(),
        rejections,
    }
}

/// The distribution of the ECE, of the skill and of the Brier of a perfectly calibrated expert.
#[derive(Debug, Clone, PartialEq)]
pub struct Null {
    pub repetitions: usize,
    pub items: usize,
    pub buckets: usize,
    pub ece_p50: f64,
    pub ece_p90: f64,
    pub ece_p95: f64,
    pub max_ece: f64,
    pub skill_p50: f64,
    pub skill_p10: f64,
    pub brier_p50: f64,
    pub fraction_passing: f64,
    /// Fraction of calibrated experts that criterion 3 rejects on its own (the systematic bucket).
    pub rejected_criterion_3: f64,
    /// Fraction of calibrated experts that criterion 2 rejects on its own (the ECE above the threshold).
    pub rejected_criterion_2: f64,
    /// Fraction of calibrated experts rejected by some criterion: it is the vice of the gate.
    pub rejected_by_any: f64,
    /// Fraction of calibrated experts that the **corrected criterion 3** (binomial test with Bonferroni) rejects.
    pub rejected_criterion_3_binomial: f64,
}

fn quantile_of(values: &mut [f64], q: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let position = q.clamp(0.0, 1.0) * (values.len() as f64 - 1.0);
    let low = position.floor() as usize;
    let high = position.ceil() as usize;
    if low == high {
        return values[low];
    }
    let weight = position - low as f64;
    values[low] * (1.0 - weight) + values[high] * weight
}

/// The declared null of the pretest: `p` comes from the declared uniform prior and the outcome is the Bernoulli draw
/// from it. It runs the same design (n items, equal-mass buckets) and returns the distribution of the measurements.
pub fn pretest_null(seed: u64, repetitions: usize, items: usize, buckets: usize) -> Null {
    null_with_gate(seed, repetitions, items, buckets, PROTOCOL_MAX_ECE)
}

/// The same null, with the ECE threshold declared by the caller: it is the adjusted gate that matters.
pub fn null_with_gate(
    seed: u64,
    repetitions: usize,
    items: usize,
    buckets: usize,
    max_ece: f64,
) -> Null {
    let mut rng = Rng::new(seed);
    let mut eces = Vec::with_capacity(repetitions);
    let mut skills = Vec::with_capacity(repetitions);
    let mut briers = Vec::with_capacity(repetitions);
    let mut passing = 0usize;
    let mut rejected_3 = 0usize;
    let mut rejected_2 = 0usize;
    let mut rejected_any = 0usize;
    let mut rejected_3_binomial = 0usize;
    for _ in 0..repetitions {
        let confidence: Vec<f64> = (0..items).map(|_| uniform(&mut rng)).collect();
        let outcome: Vec<bool> = confidence.iter().map(|p| uniform(&mut rng) < *p).collect();
        let b = brier(&confidence, &outcome);
        let mean: f64 = outcome.iter().filter(|y| **y).count() as f64 / items as f64;
        let climate = mean * (1.0 - mean);
        let skill = if climate == 0.0 {
            0.0
        } else {
            1.0 - b / climate
        };
        let ece = equal_mass_ece(&confidence, &outcome, buckets);
        eces.push(ece);
        skills.push(skill);
        briers.push(b);
        // the vice of the gate: runs the four criteria, with the threshold of the adjusted gate, in the calibrated world
        let verdict = evaluate_with(&confidence, &outcome, buckets, max_ece);
        let c3 = verdict
            .rejections
            .iter()
            .any(|r| matches!(r, Rejection::SystematicBucketByFixedThreshold { .. }));
        let c2 = verdict
            .rejections
            .iter()
            .any(|r| matches!(r, Rejection::EceAboveLimit { .. }));
        if c3 {
            rejected_3 += 1;
        }
        if c2 {
            rejected_2 += 1;
        }
        if !verdict.approved {
            rejected_any += 1;
        }
        let buckets = equal_mass_buckets(&confidence, &outcome, buckets);
        if buckets_by_binomial_test(&buckets)
            .iter()
            .any(|c| c.is_systematic(0.05))
        {
            rejected_3_binomial += 1;
        }
        if skill > 0.0 && ece <= max_ece && !c3 && !c2 {
            passing += 1;
        }
    }
    Null {
        repetitions,
        items,
        buckets,
        ece_p50: quantile_of(&mut eces.clone(), 0.50),
        ece_p90: quantile_of(&mut eces.clone(), 0.90),
        ece_p95: quantile_of(&mut eces.clone(), 0.95),
        max_ece: eces.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        skill_p50: quantile_of(&mut skills.clone(), 0.50),
        skill_p10: quantile_of(&mut skills.clone(), 0.10),
        brier_p50: quantile_of(&mut briers.clone(), 0.50),
        fraction_passing: passing as f64 / repetitions as f64,
        rejected_criterion_3: rejected_3 as f64 / repetitions as f64,
        rejected_criterion_2: rejected_2 as f64 / repetitions as f64,
        rejected_by_any: rejected_any as f64 / repetitions as f64,
        rejected_criterion_3_binomial: rejected_3_binomial as f64 / repetitions as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_murphy_identity_closes_exactly() {
        let mut rng = Rng::new(2026);
        for _ in 0..20 {
            let confidence: Vec<f64> = (0..40).map(|_| uniform(&mut rng)).collect();
            let outcome: Vec<bool> = confidence.iter().map(|p| uniform(&mut rng) < *p).collect();
            let m = murphy_decomposition(&confidence, &outcome);
            let b = brier(&confidence, &outcome);
            assert!(
                (m.rel - m.res + m.unc - b).abs() < 1e-12,
                "identity: {m:?} against {b}"
            );
        }
    }

    #[test]
    fn a_calibrated_pretest_passes_the_four_criteria() {
        // verification design: 10 bands of 4 items, with the observed rate equal to the forecast mean.
        let mut confidence = Vec::new();
        let mut outcome = Vec::new();
        for band in 0..10 {
            let p = 0.05 + band as f64 * 0.10;
            let hits = (p * 4.0).round() as usize;
            for i in 0..4 {
                confidence.push(p);
                outcome.push(i < hits);
            }
        }
        let r = evaluate(&confidence, &outcome);
        // the granularity of 4 items per bucket already costs about 0.06 of ECE to whoever is calibrated:
        // it is a floor of the design, not an error of the expert.
        assert!(r.ece < 0.08, "ece of the perfect design: {}", r.ece);
        assert!(r.skill > 0.2, "skill of the perfect design: {}", r.skill);
        assert!(curve_is_weakly_monotone(&r.buckets));
        assert!(observed_curve_is_weakly_monotone(&r.buckets));
        assert!(r.approved, "rejections: {:?}", r.rejections);
    }

    #[test]
    fn protocol_criterion_4_cannot_fail() {
        // Any set of items, calibrated or inverted, passes criterion 4 as it is written:
        // the buckets are cut by sorted p, so the top mean always exceeds the bottom one.
        let confidence: Vec<f64> = (0..20).map(|i| 0.95 - i as f64 * 0.045).collect();
        let inverted: Vec<bool> = (0..20).map(|i| i % 3 != 0).collect();
        let buckets = equal_mass_buckets(&confidence, &inverted, PROTOCOL_BUCKETS);
        assert!(
            curve_is_weakly_monotone(&buckets),
            "criterion 4 passes even with the forecast inverted"
        );
    }

    #[test]
    fn the_overshooting_pretest_fails_by_ece_and_by_bucket() {
        let confidence: Vec<f64> = (0..40).map(|i| 0.9 + (i % 5) as f64 * 0.02).collect();
        let outcome: Vec<bool> = (0..40).map(|i| i % 4 == 0).collect();
        let r = evaluate(&confidence, &outcome);
        assert!(!r.approved);
        assert!(
            r.rejections
                .iter()
                .any(|x| matches!(x, Rejection::EceAboveLimit { .. }))
        );
        assert!(
            r.rejections
                .iter()
                .any(|x| matches!(x, Rejection::SystematicBucketByFixedThreshold { .. }))
        );
        assert!(
            r.skill < 0.0,
            "forecasting high and missing brings the skill down: {}",
            r.skill
        );
    }

    #[test]
    fn inverted_probability_fails_by_monotonicity() {
        // inverted forecast: the expert gives high p where the outcome is no, and low p where it is yes
        let confidence: Vec<f64> = (0..20).map(|i| 0.025 + i as f64 * 0.05).collect();
        let outcome: Vec<bool> = confidence.iter().map(|p| *p < 0.5).collect();
        let r = evaluate(&confidence, &outcome);
        // criterion 4 does not bite (it is inert); what denounces the swapped curve is the observed diagnostic.
        assert!(
            r.rejections
                .iter()
                .all(|x| !matches!(x, Rejection::NonMonotoneCurve { .. }))
        );
        assert!(!observed_curve_is_weakly_monotone(&r.buckets));
        assert!(
            r.buckets.first().expect("has bucket").observed_rate
                > r.buckets.last().expect("has bucket").observed_rate
        );
    }

    #[test]
    fn the_protocol_design_puts_two_items_per_bucket() {
        let buckets = equal_mass_buckets(
            &(0..20)
                .map(|i| (i as f64 + 0.5) / 20.0)
                .collect::<Vec<f64>>(),
            &[true; 20],
            PROTOCOL_BUCKETS,
        );
        assert_eq!(buckets.len(), 10);
        assert!(
            buckets.iter().all(|c| c.n == 2),
            "with 20 items and 10 buckets, n = 2: {buckets:?}"
        );
        assert!(
            buckets.iter().all(|c| c.n < MIN_BUCKET_N),
            "criterion 3 stays vacuous in this design"
        );
    }

    #[test]
    fn the_binomial_test_does_not_fail_the_calibrated_bucket() {
        // the bucket that failed the third pretest: n = 4, forecast mean 0.725, observed rate 1
        let buckets = vec![Bucket {
            index: 3,
            n: 4,
            forecast_mean: 0.725,
            observed_rate: 1.0,
        }];
        let t = bucket(&buckets);
        assert!(
            !t[0].is_systematic(0.05),
            "p with Bonferroni {}",
            t[0].p_with_bonferroni
        );
        assert!(t[0].two_sided_p > 0.5, "two-sided p {}", t[0].two_sided_p);
    }

    #[test]
    fn the_binomial_test_fails_the_impossible_bucket() {
        // the case it must catch: forecasting 0.9 in a bucket and no item coming out yes
        let buckets = vec![Bucket {
            index: 0,
            n: 4,
            forecast_mean: 0.9,
            observed_rate: 0.0,
        }];
        let t = bucket(&buckets);
        assert!(
            t[0].is_systematic(0.05),
            "p with Bonferroni {}",
            t[0].p_with_bonferroni
        );
    }

    fn bucket(buckets: &[Bucket]) -> Vec<TestedBucket> {
        buckets_by_binomial_test(buckets)
    }

    #[test]
    fn the_bucket_at_the_mean_has_p_near_one() {
        for (n, p) in [(4usize, 0.5_f64), (4, 0.25), (4, 0.75), (10, 0.7)] {
            let hits = (n as f64 * p).round();
            let buckets = vec![Bucket {
                index: 0,
                n,
                forecast_mean: p,
                observed_rate: hits / n as f64,
            }];
            let t = bucket(&buckets);
            assert!(t[0].two_sided_p > 0.5, "n {n} p {p}: {}", t[0].two_sided_p);
        }
    }

    #[test]
    fn p_is_the_unit_of_the_protocol() {
        assert_eq!(invalid_probabilities(&[0.5, 0.0, 1.0, 0.01]), vec![1, 2]);
        assert!(invalid_probabilities(&[0.5, 0.25]).is_empty());
    }

    #[test]
    fn the_pretest_null_grows_when_the_sample_shrinks() {
        let twenty = pretest_null(2026, 2_000, 20, PROTOCOL_BUCKETS);
        let two_hundred = pretest_null(2026, 2_000, 200, PROTOCOL_BUCKETS);
        assert!(
            twenty.ece_p50 > two_hundred.ece_p50,
            "{} against {}",
            twenty.ece_p50,
            two_hundred.ece_p50
        );
        assert!(twenty.ece_p90 >= twenty.ece_p50);
        assert!(twenty.ece_p95 >= twenty.ece_p90);
        assert!(
            twenty.skill_p50 > 0.0,
            "the calibrated expert has positive skill: {}",
            twenty.skill_p50
        );
        assert!(
            twenty.fraction_passing < 1.0,
            "the 0.10 threshold does not let everyone pass: {}",
            twenty.fraction_passing
        );
    }

    #[test]
    fn the_null_is_deterministic() {
        let a = pretest_null(7, 500, 20, PROTOCOL_BUCKETS);
        let b = pretest_null(7, 500, 20, PROTOCOL_BUCKETS);
        assert_eq!(a, b);
    }
}
