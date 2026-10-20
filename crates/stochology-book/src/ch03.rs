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

// ------------------------------------------- the Dutch book and the world

/// Certain profit of the bookmaker who sells the agent both tickets (one pays 1 if it rains,
/// the other if it does not), found by enumerating the two states: in each, exactly one ticket pays.
pub fn dutch_book_profit(price_rain: f64, price_dry: f64) -> f64 {
    let mut worst = f64::INFINITY;
    for rain in [true, false] {
        let paid = if rain { 1.0 } else { 0.0 } + if rain { 0.0 } else { 1.0 };
        worst = worst.min(price_rain + price_dry - paid);
    }
    worst
}

/// The nearest coherent prices: both divided by their sum.
pub fn make_coherent(price_rain: f64, price_dry: f64) -> (f64, f64) {
    let total = price_rain + price_dry;
    (price_rain / total, price_dry / total)
}

/// Expected profit per round of a bettor who knows that it rains with frequency `q`, against an agent
/// that quotes `price_rain` for the rain ticket: sells it when it is dear, buys it when it is cheap.
pub fn edge_over_the_agent(price_rain: f64, q: f64) -> f64 {
    let sold = price_rain - q; // receives the price, pays 1 with chance q
    let bought = q - price_rain;
    sold.max(bought)
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

    /// Oracle: 0.60 + 0.50 - 1 = 0.10 certain; after coherence the book is gone (to rounding) and,
    /// against a world that rains 25% of the time, the edge is 0.6/1.1 - 0.25 = 0.2955, three times the book.
    #[test]
    fn coherence_removes_the_book_and_leaves_the_distance_to_the_world() {
        assert!((dutch_book_profit(0.60, 0.50) - 0.10).abs() < 1e-12);
        let (r, d) = make_coherent(0.60, 0.50);
        assert!(dutch_book_profit(r, d).abs() < 1e-12);
        let q = 0.25;
        assert!((edge_over_the_agent(r, q) - (0.6 / 1.1 - 0.25)).abs() < 1e-12);
        assert!(edge_over_the_agent(r, q) > 2.5 * dutch_book_profit(0.60, 0.50));
        // a calibrated agent leaves nothing to the bettor who knows the world
        assert!(edge_over_the_agent(q, q).abs() < 1e-12);
    }

    /// Mutant: a "coherence" that divides only the dry price by the sum leaves a book of 0.0545.
    #[test]
    fn the_mutant_that_fixes_one_price_still_has_a_book() {
        let (total, rain, dry) = (1.10, 0.60, 0.50);
        let left = dutch_book_profit(rain, dry / total);
        assert!(left > 0.05);
        let (r, d) = make_coherent(rain, dry);
        assert!(dutch_book_profit(r, d) < 1e-12);
    }
}
