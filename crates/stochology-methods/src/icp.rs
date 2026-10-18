//! stochology-methods — invariant causal prediction over declared environments.
//!
//! The method: inside each environment, regress the target on the candidate set; if the residual
//! distribution does not change across environments, the set is accepted as invariant. Chance
//! enters through the Kolmogorov-Smirnov statistic, with a threshold that depends on n and on
//! alpha.
//!
//! Declared and not implemented: a permutation test of the null model (here the null is the
//! asymptotic KS threshold itself) and environments that come from an external base, which live in
//! `data/`.

use stochology_core::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NoRows,
    UnequalRows,
    MissingColumn(usize),
    TargetAmongPredictors(usize),
    TooManyPredictors(usize),
    SingularSystem,
    SingleEnvironment,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoRows => write!(f, "environment with no rows"),
            Error::UnequalRows => write!(f, "columns with different row counts"),
            Error::MissingColumn(j) => write!(f, "column {j} missing"),
            Error::TargetAmongPredictors(j) => {
                write!(f, "the target appears as a predictor: column {j}")
            }
            Error::TooManyPredictors(n) => write!(f, "{n} candidates: the enumeration stops at 16"),
            Error::SingularSystem => write!(f, "singular system: collinear predictors"),
            Error::SingleEnvironment => write!(f, "invariance needs two environments or more"),
        }
    }
}

impl std::error::Error for Error {}

/// An environment: the same set of columns measured under a different condition.
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub name: String,
    pub columns: Vec<Vec<f64>>,
}

impl Environment {
    pub fn new(name: &str, columns: Vec<Vec<f64>>) -> Result<Self, Error> {
        let rows = columns.first().map(Vec::len).ok_or(Error::NoRows)?;
        if rows == 0 {
            return Err(Error::NoRows);
        }
        if columns.iter().any(|c| c.len() != rows) {
            return Err(Error::UnequalRows);
        }
        Ok(Environment {
            name: name.to_string(),
            columns,
        })
    }

    pub fn rows(&self) -> usize {
        self.columns.first().map(Vec::len).unwrap_or(0)
    }

    pub fn column(&self, j: usize) -> Result<&Vec<f64>, Error> {
        self.columns.get(j).ok_or(Error::MissingColumn(j))
    }

    /// Averages the rows `per_group` at a time: the unit stops being the row and becomes the group.
    pub fn mean_by_group(&self, per_group: usize) -> Result<Environment, Error> {
        if per_group == 0 {
            return Err(Error::UnequalRows);
        }
        let mut columns = Vec::with_capacity(self.columns.len());
        for column in &self.columns {
            let mut means = Vec::with_capacity(column.len() / per_group);
            for block in column.chunks(per_group) {
                if block.len() < per_group {
                    break;
                }
                means.push(block.iter().sum::<f64>() / per_group as f64);
            }
            columns.push(means);
        }
        if columns.first().map(Vec::len).unwrap_or(0) == 0 {
            return Err(Error::NoRows);
        }
        Environment::new(&format!("{}#mean{}", self.name, per_group), columns)
    }
}

/// Least squares with an intercept. Returns the coefficients (intercept first) and the residual.
pub fn regression(target: &[f64], predictors: &[Vec<f64>]) -> Result<(Vec<f64>, Vec<f64>), Error> {
    let n = target.len();
    if n == 0 {
        return Err(Error::NoRows);
    }
    if predictors.iter().any(|p| p.len() != n) {
        return Err(Error::UnequalRows);
    }
    let p = predictors.len() + 1;
    let mut a = vec![vec![0.0_f64; p]; p];
    let mut b = vec![0.0_f64; p];
    // the first column is the intercept
    for i in 0..n {
        let mut row = Vec::with_capacity(p);
        row.push(1.0);
        for predictor in predictors {
            row.push(predictor[i]);
        }
        for u in 0..p {
            b[u] += row[u] * target[i];
            for v in 0..p {
                a[u][v] += row[u] * row[v];
            }
        }
    }
    let coef = solve(a, b)?;
    let mut residual = Vec::with_capacity(n);
    for i in 0..n {
        let mut fit = coef[0];
        for (u, predictor) in predictors.iter().enumerate() {
            fit += coef[u + 1] * predictor[i];
        }
        residual.push(target[i] - fit);
    }
    Ok((coef, residual))
}

fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, Error> {
    let n = b.len();
    for column in 0..n {
        let mut pivot = column;
        for row in column + 1..n {
            if a[row][column].abs() > a[pivot][column].abs() {
                pivot = row;
            }
        }
        if a[pivot][column].abs() < 1e-12 {
            return Err(Error::SingularSystem);
        }
        a.swap(column, pivot);
        b.swap(column, pivot);
        let divisor = a[column][column];
        for v in column..n {
            a[column][v] /= divisor;
        }
        b[column] /= divisor;
        for row in 0..n {
            if row == column {
                continue;
            }
            let factor = a[row][column];
            if factor == 0.0 {
                continue;
            }
            for v in column..n {
                a[row][v] -= factor * a[column][v];
            }
            b[row] -= factor * b[column];
        }
    }
    Ok(b)
}

/// Kolmogorov-Smirnov statistic of two samples (largest distance between the cumulative curves).
pub fn ks(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 1.0;
    }
    let mut sorted_a: Vec<f64> = a.to_vec();
    let mut sorted_b: Vec<f64> = b.to_vec();
    sorted_a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    sorted_b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let (n, m) = (a.len() as f64, b.len() as f64);
    let (mut i, mut j, mut largest) = (0usize, 0usize, 0.0_f64);
    while i < a.len() || j < b.len() {
        let next = match (sorted_a.get(i), sorted_b.get(j)) {
            (Some(x), Some(y)) => x.min(*y),
            (Some(x), None) => *x,
            (None, Some(y)) => *y,
            (None, None) => break,
        };
        while i < a.len() && sorted_a[i] <= next {
            i += 1;
        }
        while j < b.len() && sorted_b[j] <= next {
            j += 1;
        }
        let distance = (i as f64 / n - j as f64 / m).abs();
        if distance > largest {
            largest = distance;
        }
    }
    largest
}

/// Asymptotic KS threshold of two samples: `c(alpha) * sqrt((n+m)/(n*m))`.
pub fn ks_threshold(n: usize, m: usize, alpha: f64) -> f64 {
    if n == 0 || m == 0 {
        return 0.0;
    }
    let c = (-0.5 * (alpha / 2.0).ln()).sqrt();
    c * (((n + m) as f64) / ((n * m) as f64)).sqrt()
}

fn environment_residuals(
    environment: &Environment,
    target: usize,
    predictors: &[usize],
) -> Result<Vec<f64>, Error> {
    if predictors.contains(&target) {
        return Err(Error::TargetAmongPredictors(target));
    }
    let columns: Vec<Vec<f64>> = predictors
        .iter()
        .map(|j| environment.column(*j).cloned())
        .collect::<Result<_, _>>()?;
    let (_, residual) = regression(environment.column(target)?, &columns)?;
    Ok(residual)
}

/// The set is accepted when the residual does not change from environment to environment, at level `alpha`.
pub fn accepted_set(
    environments: &[Environment],
    target: usize,
    predictors: &[usize],
    alpha: f64,
) -> Result<bool, Error> {
    if environments.len() < 2 {
        return Err(Error::SingleEnvironment);
    }
    if predictors.len() > 16 {
        return Err(Error::TooManyPredictors(predictors.len()));
    }
    let residuals: Vec<Vec<f64>> = environments
        .iter()
        .map(|environment| environment_residuals(environment, target, predictors))
        .collect::<Result<_, _>>()?;
    for i in 0..residuals.len() {
        for j in i + 1..residuals.len() {
            let threshold = ks_threshold(residuals[i].len(), residuals[j].len(), alpha);
            if ks(&residuals[i], &residuals[j]) > threshold {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// Every accepted subset of `candidates`, from the smallest to the largest.
pub fn accepted_family(
    environments: &[Environment],
    target: usize,
    candidates: &[usize],
    alpha: f64,
) -> Result<Vec<Vec<usize>>, Error> {
    if candidates.len() > 16 {
        return Err(Error::TooManyPredictors(candidates.len()));
    }
    let mut accepted = Vec::new();
    for mask in 0u32..(1u32 << candidates.len()) {
        let set: Vec<usize> = (0..candidates.len())
            .filter(|k| mask & (1 << k) != 0)
            .map(|k| candidates[k])
            .collect();
        if accepted_set(environments, target, &set, alpha)? {
            accepted.push(set);
        }
    }
    accepted.sort_by_key(|c| (c.len(), c.clone()));
    Ok(accepted)
}

/// The area that survives: the intersection of the accepted sets. An empty family gives an empty
/// area — and, if the empty set is accepted, there is no area to defend either: no predictor is
/// necessary.
pub fn area(accepted: &[Vec<usize>]) -> Vec<usize> {
    let first = match accepted.first() {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut area: Vec<usize> = first.clone();
    for set in &accepted[1..] {
        area.retain(|j| set.contains(j));
    }
    area
}

/// How many subsets a set of `n` candidates has: `2^n`.
pub fn count_sets(n: usize) -> u64 {
    1u64 << n
}

/// Standard normal by Box-Muller, for the synthetic worlds of the tests.
pub fn normal(rng: &mut Rng) -> f64 {
    let u1 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let u2 = ((rng.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Synthetic world with the marginal changing from environment to environment: X1 has scale 1 in
    /// environment A and 3 in B, the target noise has its own scale per environment, and X2 carries
    /// the label.
    fn world(seed: u64, per_environment: usize, noise_a: f64, noise_b: f64) -> Vec<Environment> {
        let mut rng = Rng::new(seed);
        let mut environments = Vec::new();
        for (index, environment) in ["A", "B"].iter().enumerate() {
            let level = index as f64;
            let scale = if index == 0 { 1.0 } else { 3.0 };
            let noise = if index == 0 { noise_a } else { noise_b };
            let mut x1 = Vec::new();
            let mut x2 = Vec::new();
            let mut y = Vec::new();
            for _ in 0..per_environment {
                let a = scale * normal(&mut rng);
                x1.push(a);
                x2.push(0.9 * level + normal(&mut rng));
                y.push(2.0 * a + noise * normal(&mut rng));
            }
            environments.push(
                Environment::new(environment, vec![y, x1, x2]).expect("synthetic environment"),
            );
        }
        environments
    }

    #[test]
    fn the_invariant_predictor_is_accepted_and_the_confounder_is_rejected() {
        let environments = world(2026, 120, 1.0, 1.0);
        assert!(accepted_set(&environments, 0, &[1], 0.05).expect("tests"));
        assert!(!accepted_set(&environments, 0, &[2], 0.05).expect("tests"));
        let family = accepted_family(&environments, 0, &[1, 2], 0.05).expect("family");
        assert!(
            family.contains(&vec![1]),
            "the invariant enters: {family:?}"
        );
        assert!(
            !family.contains(&vec![2]),
            "the confounder stays out: {family:?}"
        );
        assert_eq!(area(&family), vec![1]);
    }

    #[test]
    fn empty_area_when_nothing_is_invariant() {
        // the target noise changes scale across environments: not the empty set, nor {X1}, nor {X2}.
        let environments = world(7, 80, 1.0, 4.0);
        let family = accepted_family(&environments, 0, &[1, 2], 0.05).expect("family");
        assert!(family.is_empty(), "nothing is invariant: {family:?}");
        assert!(area(&family).is_empty());
    }

    #[test]
    fn the_group_unit_shrinks_the_family() {
        let fine = world(11, 240, 1.0, 1.0);
        let coarse: Vec<Environment> = fine
            .iter()
            .map(|a| a.mean_by_group(20).expect("mean by group"))
            .collect();
        let fine_family = accepted_family(&fine, 0, &[1, 2], 0.05).expect("fine family");
        let coarse_family = accepted_family(&coarse, 0, &[1, 2], 0.05).expect("coarse family");
        // The power of the test walks with n, and n must be counted on the independent unit: with 12
        // points per environment the KS threshold is loose and the accepted family grows.
        assert_eq!(coarse[0].rows(), 12);
        assert!(ks_threshold(12, 12, 0.05) > ks_threshold(240, 240, 0.05));
        assert!(
            coarse_family.len() >= fine_family.len(),
            "coarse grain accepts more: {} against {}",
            coarse_family.len(),
            fine_family.len()
        );
        assert_eq!(area(&fine_family), vec![1]);
    }

    #[test]
    fn ks_knows_its_own_threshold() {
        assert_eq!(ks(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), 0.0);
        let far = ks(&[0.0, 0.1, 0.2], &[5.0, 5.1, 5.2]);
        assert!(far > 0.9, "separated samples give a high ks: {far}");
        let mut rng = Rng::new(3);
        let a: Vec<f64> = (0..400).map(|_| normal(&mut rng)).collect();
        let b: Vec<f64> = (0..400).map(|_| normal(&mut rng) + 1.2).collect();
        let statistic = ks(&a, &b);
        assert!(statistic > ks_threshold(400, 400, 0.05));
    }

    #[test]
    fn the_threshold_falls_with_the_sample_size() {
        assert!(ks_threshold(50, 50, 0.05) > ks_threshold(500, 500, 0.05));
        assert!(ks_threshold(500, 500, 0.05) > ks_threshold(5000, 5000, 0.05));
    }

    #[test]
    fn collinear_predictors_fail() {
        let environments = [Environment::new(
            "A",
            vec![
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0, 3.0],
            ],
        )
        .expect("environment")];
        match regression(
            &[1.0, 2.0, 3.0],
            &[vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0]],
        ) {
            Err(Error::SingularSystem) => {}
            other => panic!("expected a singular system, got {other:?}"),
        }
        assert_eq!(environments.len(), 1);
    }

    #[test]
    fn the_target_as_predictor_fails() {
        let environments = world(1, 20, 1.0, 1.0);
        match accepted_set(&environments, 1, &[1], 0.05) {
            Err(Error::TargetAmongPredictors(j)) => assert_eq!(j, 1),
            other => panic!("expected the target among the predictors, got {other:?}"),
        }
    }

    #[test]
    fn the_set_count_matches_the_enumeration() {
        let environments = world(5, 30, 1.0, 1.0);
        let family = accepted_family(&environments, 0, &[1, 2], 0.05).expect("family");
        assert_eq!(count_sets(2), 4);
        assert!(family.len() <= count_sets(2) as usize);
        assert_eq!(count_sets(10), 1024);
    }
}
