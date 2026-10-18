//! Chapter 3 — probability without mystery: trees in natural frequencies and decibans.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tree {
    pub people: u64,
    pub sick: u64,
    pub true_positives: u64,
    pub healthy: u64,
    pub false_positives: u64,
    pub positives: u64,
}

impl Tree {
    /// Fraction of sick people among the positives: the answer that matters to the patient.
    pub fn fraction_sick_among_positives(&self) -> f64 {
        self.true_positives as f64 / self.positives as f64
    }

    /// "one in every how many" positives is in fact sick.
    pub fn one_in_every(&self) -> f64 {
        self.positives as f64 / self.true_positives as f64
    }
}

/// Builds the tree of 10 thousand (or n) people in natural frequencies.
pub fn tree(people: u64, prevalence: f64, sensitivity: f64, false_positive: f64) -> Tree {
    let sick = (people as f64 * prevalence).round() as u64;
    let healthy = people - sick;
    let true_positives = (sick as f64 * sensitivity).round() as u64;
    let false_positives = (healthy as f64 * false_positive).round() as u64;
    Tree {
        people,
        sick,
        true_positives,
        healthy,
        false_positives,
        positives: true_positives + false_positives,
    }
}

/// Strength of the evidence in decibans: ten times the decimal logarithm of the ratio.
pub fn decibans(ratio: f64) -> f64 {
    10.0 * ratio.log10()
}

/// Updates a prior belief by applying likelihood ratios.
pub fn posterior_by_odds(prior: f64, ratios: &[f64]) -> f64 {
    let mut odds = prior / (1.0 - prior);
    for r in ratios {
        odds *= r;
    }
    odds / (1.0 + odds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mammography_gives_one_in_ten() {
        let a = tree(10_000, 0.01, 0.90, 0.09);
        assert_eq!(a.sick, 100);
        assert_eq!(a.true_positives, 90);
        assert_eq!(a.false_positives, 891);
        assert_eq!(a.positives, 981);
        assert!(
            (a.one_in_every() - 10.9).abs() < 0.05,
            "{}",
            a.one_in_every()
        );
    }

    #[test]
    fn a_ratio_of_ten_is_ten_decibans() {
        assert!((decibans(10.0) - 10.0).abs() < 1e-12);
        assert!((decibans(4.2) - 6.2324).abs() < 1e-3);
    }

    #[test]
    fn two_independent_tests_against_correlated_tests() {
        let independent = posterior_by_odds(0.01, &[10.0, 10.0]);
        let correlated = posterior_by_odds(0.01, &[10.0, 4.2]);
        assert!((independent - 0.5025).abs() < 0.002, "{}", independent);
        assert!((correlated - 0.2977).abs() < 0.002, "{}", correlated);
    }
}
