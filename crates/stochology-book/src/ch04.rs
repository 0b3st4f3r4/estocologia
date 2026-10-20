//! Chapter 4 — luck, ignorance and the turkey: a coin whose bias changes slowly.

use stochology_core::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coverage {
    pub steps: usize,
    pub repetitions: usize,
    pub discount: f64,
    /// Fraction of steps in which the 95% interval of the fixed model covers the truth.
    pub static_coverage: f64,
    /// Fraction of steps in which the interval of the model with a forgetting factor covers the truth.
    pub discounted_coverage: f64,
    pub static_width_100: f64,
    pub static_width_final: f64,
    pub discounted_width_final: f64,
}

/// Bias that climbs from 0.2 to 0.6 along the steps, with a reproducible draw.
pub fn bias_at(step: usize, steps: usize) -> f64 {
    0.2 + 0.4 * (step as f64 / steps as f64)
}

fn width(p: f64, effective_n: f64) -> f64 {
    if effective_n <= 1.0 {
        return 1.0;
    }
    2.0 * 1.96 * (p * (1.0 - p) / effective_n).sqrt()
}

/// Runs the world that changes and measures the coverage of both models.
///
/// Book configuration: 6,400 steps, 200 repetitions, discount 0.99.
/// Instrument: ~cargo run --release --bin labs -- --chapter 4~
pub fn moving_coin(seed: u64, steps: usize, repetitions: usize, discount: f64) -> Coverage {
    let mut rng = Rng::new(seed);
    let mut static_covers = 0usize;
    let mut discounted_covers = 0usize;
    let mut counted = 0usize;
    let mut static_width_100 = 0.0;
    let mut static_width_final = 0.0;
    let mut discounted_width_final = 0.0;
    let effective_n = (1.0 + discount) / (1.0 - discount);

    for _ in 0..repetitions {
        let mut heads = 0.0_f64;
        let mut sum = 0.0_f64;
        let mut weight = 0.0_f64;
        for step in 1..=steps {
            let truth = bias_at(step, steps);
            let x = if (rng.next_u64() as f64) / (u64::MAX as f64) < truth {
                1.0
            } else {
                0.0
            };
            heads += x;

            let p_static = heads / step as f64;
            let l_static = width(p_static, step as f64);

            sum = discount * sum + x;
            weight = discount * weight + 1.0;
            let p_discounted = sum / weight;
            let l_discounted = width(p_discounted, effective_n);

            if step >= 100 {
                counted += 1;
                if (p_static - truth).abs() <= l_static / 2.0 {
                    static_covers += 1;
                }
                if (p_discounted - truth).abs() <= l_discounted / 2.0 {
                    discounted_covers += 1;
                }
            }
            if step == 100 {
                static_width_100 += l_static;
            }
            if step == steps {
                static_width_final += l_static;
                discounted_width_final += l_discounted;
            }
        }
    }

    let r = repetitions as f64;
    let c = counted as f64;
    Coverage {
        steps,
        repetitions,
        discount,
        static_coverage: static_covers as f64 / c,
        discounted_coverage: discounted_covers as f64 / c,
        static_width_100: static_width_100 / r,
        static_width_final: static_width_final / r,
        discounted_width_final: discounted_width_final / r,
    }
}

/// A survey population: groups with a share of the population and a probability of answering yes.
/// Within a group, people differ: each one answers yes with the probability of the group.
pub const SURVEY_SHARES: [f64; 5] = [0.25, 0.25, 0.20, 0.15, 0.15];
pub const SURVEY_YES: [f64; 5] = [0.10, 0.30, 0.45, 0.60, 0.85];

/// The law of total variance for the answers of a survey: Var(Y) = E[Var(Y | group)] + Var(E[Y | group]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VarianceSplit {
    /// The aleatory parcel: the disagreement inside the groups.
    pub within: f64,
    /// The parcel of the groups: how far their means lie from each other.
    pub between: f64,
    pub total: f64,
    pub yes_rate: f64,
}

/// Closed form of the split for a yes/no answer.
pub fn variance_split(shares: &[f64], yes: &[f64]) -> VarianceSplit {
    let yes_rate: f64 = shares.iter().zip(yes).map(|(w, p)| w * p).sum();
    let within: f64 = shares.iter().zip(yes).map(|(w, p)| w * p * (1.0 - p)).sum();
    let between: f64 = shares
        .iter()
        .zip(yes)
        .map(|(w, p)| w * (p - yes_rate) * (p - yes_rate))
        .sum();
    VarianceSplit {
        within,
        between,
        total: within + between,
        yes_rate,
    }
}

/// How a simulated respondent answers on behalf of a group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Respondent {
    /// A person drawn from the group: yes with the probability of the group.
    Person,
    /// A simulated respondent that returns the most common answer of the group, always.
    MostCommon,
}

/// The split measured on a simulated sample of `n` answers.
pub fn simulated_split(seed: u64, n: usize, who: Respondent) -> VarianceSplit {
    let mut rng = Rng::new(seed);
    let k = SURVEY_SHARES.len();
    let (mut count, mut yes) = (vec![0usize; k], vec![0usize; k]);
    for _ in 0..n {
        let u = (rng.next_u64() as f64) / (u64::MAX as f64);
        let mut acc = 0.0;
        let mut g = k - 1;
        for (i, w) in SURVEY_SHARES.iter().enumerate() {
            acc += w;
            if u < acc {
                g = i;
                break;
            }
        }
        let answer = match who {
            Respondent::Person => (rng.next_u64() as f64) / (u64::MAX as f64) < SURVEY_YES[g],
            Respondent::MostCommon => SURVEY_YES[g] > 0.5,
        };
        count[g] += 1;
        yes[g] += usize::from(answer);
    }
    let nf = n as f64;
    let rate: f64 = yes.iter().sum::<usize>() as f64 / nf;
    let (mut within, mut between) = (0.0, 0.0);
    for g in 0..k {
        let w = count[g] as f64 / nf;
        let m = yes[g] as f64 / count[g] as f64;
        within += w * m * (1.0 - m);
        between += w * (m - rate) * (m - rate);
    }
    VarianceSplit {
        within,
        between,
        total: within + between,
        yes_rate: rate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fixed_model_is_almost_always_wrong_and_the_forgetting_model_covers() {
        let c = moving_coin(2026, 6_400, 200, 0.99);
        assert!(
            c.static_coverage < 0.2,
            "static coverage: {}",
            c.static_coverage
        );
        assert!(
            c.discounted_coverage > 0.90,
            "discounted coverage: {}",
            c.discounted_coverage
        );
    }

    #[test]
    fn the_fixed_interval_shrinks_and_the_discounted_one_stays_wide() {
        let c = moving_coin(2026, 6_400, 200, 0.99);
        assert!(c.static_width_final < c.static_width_100);
        assert!(c.discounted_width_final > c.static_width_final);
    }

    #[test]
    fn the_bias_moves_from_two_tenths_to_six_tenths() {
        assert!((bias_at(0, 3_200) - 0.2).abs() < 1e-12);
        assert!((bias_at(3_200, 3_200) - 0.6).abs() < 1e-12);
    }

    /// A mutant of the split: it keeps only the variance between the groups and calls it the total.
    fn total_without_the_aleatory_parcel(shares: &[f64], yes: &[f64]) -> f64 {
        variance_split(shares, yes).between
    }

    #[test]
    fn the_split_obeys_the_law_of_total_variance() {
        let s = variance_split(&SURVEY_SHARES, &SURVEY_YES);
        // Oracle: for a yes/no answer the total variance is rate (1 - rate), whatever the groups.
        assert!((s.total - s.yes_rate * (1.0 - s.yes_rate)).abs() < 1e-12);
        // Limit cases: identical groups have no variance between them; groups of certain answers
        // have none inside them.
        let same = variance_split(&[0.5, 0.5], &[0.3, 0.3]);
        assert!(same.between.abs() < 1e-15 && (same.within - 0.21).abs() < 1e-15);
        let certain = variance_split(&[0.5, 0.5], &[0.0, 1.0]);
        assert!(certain.within.abs() < 1e-15 && (certain.between - 0.25).abs() < 1e-15);
    }

    #[test]
    fn a_most_common_answer_respondent_erases_the_aleatory_parcel_and_moves_the_rate() {
        let real = simulated_split(2026, 200_000, Respondent::Person);
        let sim = simulated_split(2026, 200_000, Respondent::MostCommon);
        let exact = variance_split(&SURVEY_SHARES, &SURVEY_YES);
        assert!((real.within - exact.within).abs() < 0.005);
        assert!((real.yes_rate - exact.yes_rate).abs() < 0.005);
        assert!(sim.within < 1e-12, "within {}", sim.within);
        // The simulated sample answers yes only for the groups above one half: 0.15 + 0.15.
        assert!((sim.yes_rate - 0.30).abs() < 0.005, "rate {}", sim.yes_rate);
        assert!(sim.total < exact.total);
    }

    #[test]
    fn the_mutant_that_drops_the_aleatory_parcel_is_killed() {
        let exact = variance_split(&SURVEY_SHARES, &SURVEY_YES);
        let mutant = total_without_the_aleatory_parcel(&SURVEY_SHARES, &SURVEY_YES);
        assert!((mutant - exact.yes_rate * (1.0 - exact.yes_rate)).abs() > 0.05);
    }
}
