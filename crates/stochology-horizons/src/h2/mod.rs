//! stochology-horizons — the seven rounds of causal invariance and the multiplicity.
//!
//! A round is a target, a set of candidates and the environments: it returns how many subsets
//! exist (2^n), how many are accepted and which area survives. On top of that run the attempt
//! counter K and the Bonferroni correction that H2g demands.
//!
//! Declared and not implemented: the seven real rounds (light tunnel, pharmacology, yeast
//! genetics, STAR) remain as record — the method is what is here; the external base is not.

pub mod genetics;

use stochology_methods::icp::{Environment, Error, accepted_family, area, count_sets, normal};

/// One round: a target, the candidates and what is left of the area.
#[derive(Debug, Clone, PartialEq)]
pub struct Round {
    pub name: String,
    pub target: usize,
    pub candidates: Vec<usize>,
    pub subsets: u64,
    pub accepted: usize,
    pub area: Vec<usize>,
}

pub fn round(
    name: &str,
    environments: &[Environment],
    target: usize,
    candidates: &[usize],
    alpha: f64,
) -> Result<Round, Error> {
    let familia = accepted_family(environments, target, candidates, alpha)?;
    Ok(Round {
        name: name.to_string(),
        target,
        candidates: candidates.to_vec(),
        subsets: count_sets(candidates.len()),
        accepted: familia.len(),
        area: area(&familia),
    })
}

/// Declared multiplicity: how many areas survive before and after Bonferroni.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Multiplicity {
    pub k: usize,
    pub alpha: f64,
    pub corrected_alpha: f64,
    pub areas_before: usize,
    pub areas_after: usize,
}

pub fn multiplicity(p_values: &[f64], alpha: f64) -> Multiplicity {
    let k = p_values.len();
    let corrected_alpha = if k == 0 { alpha } else { alpha / k as f64 };
    Multiplicity {
        k,
        alpha,
        corrected_alpha,
        areas_before: p_values.iter().filter(|p| **p <= alpha).count(),
        areas_after: p_values.iter().filter(|p| **p <= corrected_alpha).count(),
    }
}

/// Attempt counter: how many distinct specifications were run, not counting repeats.
pub fn k_counter(attempts: &[String]) -> usize {
    let mut seen: Vec<String> = Vec::new();
    for attempt in attempts {
        let clean = attempt
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if !seen.contains(&clean) {
            seen.push(clean);
        }
    }
    seen.len()
}

/// Simulated null of the round: how many times the subset is accepted when the environment labels are
/// shuffled. It separates real invariance from an artefact of the test.
pub fn shuffle_null(
    environments: &[Environment],
    target: usize,
    predictors: &[usize],
    alpha: f64,
    seed: u64,
    repetitions: usize,
) -> Result<f64, Error> {
    let mut rng = stochology_core::Rng::new(seed);
    let rows = environments
        .first()
        .map(Environment::rows)
        .ok_or(Error::NoRows)?;
    let columns: Vec<Vec<f64>> = (0..environments[0].columns.len())
        .map(|j| {
            environments
                .iter()
                .flat_map(|a| a.column(j).cloned().unwrap_or_default())
                .collect()
        })
        .collect();
    let mut accepted = 0usize;
    for _ in 0..repetitions.max(1) {
        // shuffles the order of the rows in every column, with the same permutation
        let order = permutation(&mut rng, columns[0].len());
        let mut shuffled: Vec<Environment> = Vec::new();
        for (index, environment) in environments.iter().enumerate() {
            let mut blocks: Vec<Vec<f64>> = vec![Vec::new(); columns.len()];
            for i in 0..rows {
                let origin = order[index * rows + i];
                for (j, column) in columns.iter().enumerate() {
                    blocks[j].push(column[origin]);
                }
            }
            shuffled.push(Environment::new(&environment.name, blocks)?);
        }
        if stochology_methods::icp::accepted_set(&shuffled, target, predictors, alpha)? {
            accepted += 1;
        }
    }
    Ok(accepted as f64 / repetitions.max(1) as f64)
}

fn permutation(rng: &mut stochology_core::Rng, n: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = rng.uniform(i as u64 + 1) as usize;
        order.swap(i, j);
    }
    order
}

/// Synthetic world of the rounds: the marginal of X1 changes from environment to environment (scale 1 and 3) and
/// X2 carries the label. The target depends only on X1, with the same noise in both environments — and that is why
/// {X1} is invariant and the empty set is not.
pub fn synthetic_world(seed: u64, per_environment: usize) -> Vec<Environment> {
    let mut rng = stochology_core::Rng::new(seed);
    let mut environments = Vec::new();
    for (index, name) in ["A", "B"].iter().enumerate() {
        let e = index as f64;
        let scale = if index == 0 { 1.0 } else { 3.0 };
        let mut x1 = Vec::new();
        let mut x2 = Vec::new();
        let mut y = Vec::new();
        for _ in 0..per_environment {
            let a = scale * normal(&mut rng);
            x1.push(a);
            x2.push(0.9 * e + normal(&mut rng));
            y.push(2.0 * a + 0.6 * normal(&mut rng));
        }
        environments.push(Environment::new(name, vec![y, x1, x2]).expect("synthetic environment"));
    }
    environments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_keeps_the_area_and_the_subset_count() {
        let environments = synthetic_world(2026, 140);
        let r = round("light-tunnel", &environments, 0, &[1, 2], 0.05).expect("round");
        assert_eq!(r.subsets, 4);
        assert_eq!(
            r.area,
            vec![1],
            "the surviving area is the invariant: {r:?}"
        );
        assert!(
            r.accepted < r.subsets as usize,
            "not every subset passes: {r:?}"
        );
    }

    #[test]
    fn ten_candidates_give_one_thousand_and_twenty_four_subsets() {
        assert_eq!(count_sets(10), 1024);
        let environments = synthetic_world(3, 40);
        let r = round("count", &environments, 0, &[1, 2], 0.05).expect("round");
        assert_eq!(r.subsets, 4);
    }

    #[test]
    fn unit_grain_changes_the_accepted_family() {
        let fine = synthetic_world(11, 240);
        let coarse: Vec<Environment> = fine
            .iter()
            .map(|a| a.mean_by_group(20).expect("mean per group"))
            .collect();
        let fine_round = round("fine", &fine, 0, &[1, 2], 0.05).expect("round");
        let coarse_round = round("coarse", &coarse, 0, &[1, 2], 0.05).expect("round");
        // The power of the test depends on the grain: with 12 points per environment the KS threshold is loose, the
        // empty set and the confounded one pass, and the area dissolves. Whoever counts n on a pseudoreplicated
        // row is left with a test that promises more than it can deliver (Phi2).
        assert_eq!(coarse[0].rows(), 12);
        assert!(
            stochology_methods::icp::ks_threshold(12, 12, 0.05)
                > stochology_methods::icp::ks_threshold(240, 240, 0.05)
        );
        assert!(
            coarse_round.accepted >= fine_round.accepted,
            "coarse grain accepts more: {} against {}",
            coarse_round.accepted,
            fine_round.accepted
        );
        assert_eq!(
            fine_round.area,
            vec![1],
            "at fine grain the invariant survives: {fine_round:?}"
        );
        assert_eq!(
            coarse_round.accepted, 4,
            "at coarse grain everything passes, including the empty set: {coarse_round:?}"
        );
        assert!(
            coarse_round.area.is_empty(),
            "and the area dissolves: {coarse_round:?}"
        );
    }

    #[test]
    fn bonferroni_drops_one_area_and_counts_k() {
        let mut p = vec![0.0001, 0.0005, 0.001, 0.03];
        p.extend((0..34).map(|i| 0.2 + 0.02 * i as f64));
        let m = multiplicity(&p, 0.05);
        assert_eq!(m.k, 38);
        assert_eq!(m.areas_before, 4);
        assert_eq!(m.areas_after, 3);
        assert!((m.corrected_alpha - 0.05 / 38.0).abs() < 1e-15);
    }

    #[test]
    fn k_counter_joins_repeated_specifications() {
        let mut attempts: Vec<String> = (0..38).map(|i| format!("Round {}", i)).collect();
        attempts.push("round 0".into());
        attempts.push(" ROUND   1 ".into());
        assert_eq!(attempts.len(), 40);
        assert_eq!(k_counter(&attempts), 38);
    }

    #[test]
    fn shuffled_null_shows_the_power_of_the_test() {
        let environments = synthetic_world(5, 150);
        let confounded = shuffle_null(&environments, 0, &[2], 0.05, 2026, 20).expect("null");
        let actual =
            stochology_methods::icp::accepted_set(&environments, 0, &[2], 0.05).expect("tests");
        assert!(!actual, "the confounded subset fails with the real label");
        assert!(
            confounded > 0.5,
            "without the label, the same subset passes: {confounded}"
        );
    }

    #[test]
    fn without_area_the_round_returns_an_empty_area() {
        // A dispersao do alvo muda de ambiente para ambiente: nem o vazio, nem {X1} sobrevivem.
        let mut rng = stochology_core::Rng::new(9);
        let mut environments = Vec::new();
        for (index, name) in ["A", "B"].iter().enumerate() {
            let noise = if index == 0 { 1.0 } else { 4.0 };
            let mut x1 = Vec::new();
            let mut y = Vec::new();
            for _ in 0..80 {
                let a = normal(&mut rng);
                x1.push(a);
                y.push(2.0 * a + noise * normal(&mut rng));
            }
            environments.push(Environment::new(name, vec![y, x1]).expect("environment"));
        }
        let r = round("dispersed", &environments, 0, &[1], 0.05).expect("round");
        assert_eq!(r.accepted, 0, "nothing is invariant: {r:?}");
        assert!(r.area.is_empty());
    }
}
