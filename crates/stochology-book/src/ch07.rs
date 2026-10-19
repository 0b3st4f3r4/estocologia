//! Chapter 7 — Hunting ghosts in the noise: exact null model, garden of
//! forking paths, severity and the five errors.
//!
//! Self-contained laboratory: built-in SplitMix64 (the same design as
//! crates/stochology-core/src/rng.rs), no external dependencies.
//! Usage: cargo run --release

#[derive(Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self {
            state: stochology_core::rng::offset_seed(seed),
        }
    }
    fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }
    fn normal(&mut self) -> f64 {
        let u1 = self.uniform().max(1e-18);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    fn bernoulli(&mut self, p: f64) -> bool {
        self.uniform() < p
    }
}

/// Error function (Abramowitz and Stegun 7.1.26).
fn erf(x: f64) -> f64 {
    let s = x.signum();
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let y = 1.0
        - (((((1.061_405_429 * t - 1.453_152_027) * t + 1.421_413_741) * t - 0.284_496_736) * t
            + 0.254_829_592)
            * t)
            * (-x * x).exp();
    s * y
}

fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / 2f64.sqrt()))
}

fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn sample_variance(xs: &[f64]) -> f64 {
    let m = mean(xs);
    xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0)
}

/// Two-sample t test, two-sided p, with the normal approximation.
fn t_test(a: &[f64], b: &[f64]) -> f64 {
    let se = (sample_variance(a) / a.len() as f64 + sample_variance(b) / b.len() as f64).sqrt();
    let t = (mean(a) - mean(b)) / se;
    2.0 * (1.0 - normal_cdf(t.abs()))
}

/// Least squares through the normal equations: returns beta, (X'X)^-1 and the mean squared residual.
fn regression(columns: &[Vec<f64>], y: &[f64]) -> Option<(Vec<f64>, Vec<Vec<f64>>, f64)> {
    let k = columns.len() + 1;
    let n = y.len();
    let mut a = vec![vec![0.0f64; k]; k];
    let mut b = vec![0.0f64; k];
    for i in 0..k {
        for j in 0..k {
            let mut s = 0.0;
            for r in 0..n {
                let x = if i == 0 { 1.0 } else { columns[i - 1][r] };
                let z = if j == 0 { 1.0 } else { columns[j - 1][r] };
                s += x * z;
            }
            a[i][j] = s;
        }
        let mut s = 0.0;
        for r in 0..n {
            let x = if i == 0 { 1.0 } else { columns[i - 1][r] };
            s += x * y[r];
        }
        b[i] = s;
    }
    // Gauss-Jordan with the identity alongside, to obtain the inverse.
    let mut aug = vec![vec![0.0f64; 2 * k]; k];
    for i in 0..k {
        for j in 0..k {
            aug[i][j] = a[i][j];
        }
        aug[i][k + i] = 1.0;
    }
    for col in 0..k {
        let mut pivot = col;
        for r in col + 1..k {
            if aug[r][col].abs() > aug[pivot][col].abs() {
                pivot = r;
            }
        }
        if aug[pivot][col].abs() < 1e-9 {
            return None;
        }
        aug.swap(col, pivot);
        let d = aug[col][col];
        for j in 0..2 * k {
            aug[col][j] /= d;
        }
        for r in 0..k {
            if r != col {
                let f = aug[r][col];
                for j in 0..2 * k {
                    aug[r][j] -= f * aug[col][j];
                }
            }
        }
    }
    let inv: Vec<Vec<f64>> = (0..k)
        .map(|i| (0..k).map(|j| aug[i][k + j]).collect())
        .collect();
    let beta: Vec<f64> = (0..k)
        .map(|i| (0..k).map(|j| inv[i][j] * b[j]).sum::<f64>())
        .collect();
    let mut rss = 0.0;
    for r in 0..n {
        let mut p = beta[0];
        for (i, c) in columns.iter().enumerate() {
            p += beta[i + 1] * c[r];
        }
        rss += (y[r] - p) * (y[r] - p);
    }
    Some((beta.clone(), inv, rss / (n - k).max(1) as f64))
}

/// Two-sided p for the effect of the group (first column), controlling for the rest.
fn effect_p(y: &[f64], x: &[f64], covs: &[&Vec<f64>]) -> Option<f64> {
    let mut columns: Vec<Vec<f64>> = vec![x.to_vec()];
    for c in covs {
        columns.push((*c).clone());
    }
    let (beta, inv, s2) = regression(&columns, y)?;
    let se = (s2 * inv[1][1]).sqrt();
    if se <= 0.0 {
        return None;
    }
    Some(2.0 * (1.0 - normal_cdf((beta[1] / se).abs())))
}

/// The garden of models: 500 candidates fitted on pure noise.
struct Garden {
    replicas: usize,
    candidates: usize,
    validation_beats_zero: f64,
    test_beats_zero: f64,
    mean_validation_error: f64,
    mean_test_error: f64,
}

fn model_garden(seed: u64, replicas: usize, candidates: usize, predictors: usize) -> Garden {
    let mut rng = SplitMix64::new(seed);
    let (n_train, n_val, n_test) = (25usize, 25usize, 25usize);
    let mut val_wins = 0usize;
    let mut test_wins = 0usize;
    let mut sum_val = 0.0;
    let mut sum_test = 0.0;
    for _ in 0..replicas {
        let total = n_train + n_val + n_test;
        let mut x = vec![vec![0.0f64; total]; predictors];
        for row in x.iter_mut() {
            for v in row.iter_mut() {
                *v = rng.normal();
            }
        }
        let y: Vec<f64> = (0..total).map(|_| rng.normal()).collect();
        let mut best_val = f64::INFINITY;
        let mut best_test = f64::INFINITY;
        for _ in 0..candidates {
            let k = 1 + (rng.next_u64() % 5) as usize;
            let mut chosen: Vec<usize> = Vec::new();
            while chosen.len() < k {
                let c = (rng.next_u64() % predictors as u64) as usize;
                if !chosen.contains(&c) {
                    chosen.push(c);
                }
            }
            let columns: Vec<Vec<f64>> =
                chosen.iter().map(|c| x[*c][0..n_train].to_vec()).collect();
            let (beta, _, _) = match regression(&columns, &y[0..n_train]) {
                Some(b) => b,
                None => continue,
            };
            let mut sq_val = 0.0;
            for r in 0..n_val {
                let row = n_train + r;
                let mut p = beta[0];
                for (i, c) in chosen.iter().enumerate() {
                    p += beta[i + 1] * x[*c][row];
                }
                sq_val += (y[row] - p) * (y[row] - p);
            }
            let mse_val = sq_val / n_val as f64;
            if mse_val < best_val {
                best_val = mse_val;
                let mut sq = 0.0;
                for r in 0..n_test {
                    let row = n_train + n_val + r;
                    let mut p = beta[0];
                    for (i, c) in chosen.iter().enumerate() {
                        p += beta[i + 1] * x[*c][row];
                    }
                    sq += (y[row] - p) * (y[row] - p);
                }
                best_test = sq / n_test as f64;
            }
        }
        if best_val < 1.0 {
            val_wins += 1;
        }
        if best_test < 1.0 {
            test_wins += 1;
        }
        sum_val += best_val;
        sum_test += best_test;
    }
    Garden {
        replicas,
        candidates,
        validation_beats_zero: 100.0 * val_wins as f64 / replicas as f64,
        test_beats_zero: 100.0 * test_wins as f64 / replicas as f64,
        mean_validation_error: sum_val / replicas as f64,
        mean_test_error: sum_test / replicas as f64,
    }
}

/// Hot hand: heads after heads in three tosses of an honest coin.
fn hot_hand() -> (f64, f64) {
    let mut sum_per_sequence = 0.0;
    let mut defined = 0usize;
    let mut hh = 0usize;
    let mut ht = 0usize;
    for mask in 0..8u32 {
        let s: Vec<bool> = (0..3).map(|i| mask & (1 << i) != 0).collect();
        let (mut a, mut b) = (0usize, 0usize);
        for i in 0..2 {
            if s[i] {
                if s[i + 1] {
                    a += 1;
                } else {
                    b += 1;
                }
            }
        }
        hh += a;
        ht += b;
        if a + b > 0 {
            sum_per_sequence += a as f64 / (a + b) as f64;
            defined += 1;
        }
    }
    (
        sum_per_sequence / defined as f64,
        hh as f64 / (hh + ht) as f64,
    )
}

/// 100 people tossing 20 coins each: the ghost that chance produces.
fn ghost(seed: u64, replicas: usize) -> (f64, f64) {
    let mut rng = SplitMix64::new(seed);
    let mut with_six = 0usize;
    let mut sum_longest = 0.0;
    let mut record = 0usize;
    for _ in 0..replicas {
        let mut longest = 0usize;
        for _ in 0..100 {
            let (mut current, mut best) = (0usize, 0usize);
            for _ in 0..20 {
                if rng.bernoulli(0.5) {
                    current += 1;
                    best = best.max(current);
                } else {
                    current = 0;
                }
            }
            longest = longest.max(best);
        }
        sum_longest += longest as f64;
        record = record.max(longest);
        if longest >= 6 {
            with_six += 1;
        }
    }
    (
        100.0 * with_six as f64 / replicas as f64,
        sum_longest / replicas as f64,
    )
}

/// The garden of forking paths: researcher freedoms in an experiment with no
/// effect at all. Measures two gardens: the small one (four freedoms, each used
/// once) and the large one (with optional stopping).
struct Paths {
    replicas: usize,
    small: f64,
    large: f64,
    pre_registered: f64,
}

fn garden_of_paths(seed: u64, replicas: usize) -> Paths {
    let mut rng = SplitMix64::new(seed);
    let n = 40usize;
    let mut smalls = 0usize;
    let mut larges = 0usize;
    let mut pre = 0usize;
    for _ in 0..replicas {
        let mut e1 = Vec::with_capacity(2 * n);
        let mut e2 = Vec::with_capacity(2 * n);
        let mut c1 = Vec::with_capacity(2 * n);
        let mut c2 = Vec::with_capacity(2 * n);
        let mut c3 = Vec::with_capacity(2 * n);
        let mut sex = Vec::with_capacity(2 * n);
        let mut group = Vec::with_capacity(2 * n);
        for i in 0..2 * n {
            group.push(if i < n { 0.0 } else { 1.0 });
            let a = rng.normal();
            let b = rng.normal();
            let d = rng.normal();
            e1.push(a);
            e2.push(0.7 * a + 0.7 * b);
            c1.push(d);
            c2.push(rng.normal());
            c3.push(rng.normal());
            sex.push(if rng.bernoulli(0.5) { 1.0 } else { 0.0 });
        }
        let dv1: Vec<f64> = e1.iter().zip(c1.iter()).map(|(a, d)| a + 0.4 * d).collect();
        let dv2: Vec<f64> = e2.iter().zip(c2.iter()).map(|(a, d)| a + 0.4 * d).collect();
        let analyse = |y: &Vec<f64>, m: usize, covs: &[&Vec<f64>]| -> f64 {
            let idx: Vec<usize> = (0..m).chain(n..n + m).collect();
            let yy: Vec<f64> = idx.iter().map(|i| y[*i]).collect();
            let gg: Vec<f64> = idx.iter().map(|i| group[*i]).collect();
            let cc: Vec<Vec<f64>> = covs
                .iter()
                .map(|c| idx.iter().map(|i| c[*i]).collect())
                .collect();
            let refs: Vec<&Vec<f64>> = cc.iter().collect();
            effect_p(&yy, &gg, &refs).unwrap_or(1.0)
        };
        let exclude = |y: &Vec<f64>, m: usize| -> f64 {
            let mut max_i = 0usize;
            let mut max_v = f64::NEG_INFINITY;
            for i in 0..m {
                if y[n + i] > max_v {
                    max_v = y[n + i];
                    max_i = n + i;
                }
            }
            // The first m of each group, without the most extreme case of the treated group. Until
            // 2026-10-19 this took the first 2m rows of the file (all of the first group and part of
            // the second) and split them at m - 1, so the "exclusion" compared mixed groups and often
            // excluded nobody; a reimplementation from the text found it.
            let control: Vec<f64> = (0..m).map(|i| y[i]).collect();
            let treated: Vec<f64> = (n..n + m).filter(|i| *i != max_i).map(|i| y[i]).collect();
            t_test(&control, &treated)
        };
        if t_test(&dv1[0..30], &dv1[n..n + 30]) < 0.05 {
            pre += 1;
        }

        // small garden: four freedoms, each used once
        let mut p_small = 1.0f64;
        for y in [&dv1, &dv2] {
            for m in [20usize, 30] {
                p_small = p_small.min(t_test(&y[0..m], &y[n..n + m]));
                p_small = p_small.min(analyse(y, m, &[&c1]));
            }
            p_small = p_small.min(exclude(y, 30));
        }
        if p_small < 0.05 {
            smalls += 1;
        }

        // large garden: optional stopping, covariates in series, exclusion and subgroup
        let mut p_large = 1.0f64;
        for y in [&dv1, &dv2] {
            for m in [10usize, 20, 30, 40] {
                let p = t_test(&y[0..m], &y[n..n + m]);
                p_large = p_large.min(p);
                if p < 0.05 {
                    break;
                }
            }
            for m in [20usize, 30, 40] {
                for covs in [vec![&c1], vec![&c1, &c2], vec![&c1, &c2, &c3]] {
                    p_large = p_large.min(analyse(y, m, &covs));
                }
                p_large = p_large.min(exclude(y, m));
            }
            for target in [0.0f64, 1.0] {
                let idx: Vec<usize> = (0..2 * n).filter(|i| sex[*i] == target).collect();
                let a: Vec<f64> = idx.iter().filter(|i| **i < n).map(|i| y[*i]).collect();
                let b: Vec<f64> = idx.iter().filter(|i| **i >= n).map(|i| y[*i]).collect();
                if a.len() > 3 && b.len() > 3 {
                    p_large = p_large.min(t_test(&a, &b));
                }
            }
        }
        if p_large < 0.05 {
            larges += 1;
        }
    }
    Paths {
        replicas,
        small: 100.0 * smalls as f64 / replicas as f64,
        large: 100.0 * larges as f64 / replicas as f64,
        pre_registered: 100.0 * pre as f64 / replicas as f64,
    }
}

/// Predictive value of a positive finding (Ioannidis, 2005).
fn ppv(power: f64, odds: f64, alpha: f64) -> f64 {
    (power * odds) / (odds - (1.0 - power) * odds + alpha)
}

/// The bit of one band of the safeguard panel A1-A8, in the same design as
/// crates/stochology-core/src/agents.rs: the safeguard is declared or it is not. Until 2026-10-19
/// the bit was printed as a severity drawn by a seed; that number is not Mayo's severity and was
/// removed.
fn band_passes(care_met: bool) -> bool {
    care_met
}

pub fn run() {
    println!("== Chapter 7 :: hunting ghosts in the noise ==");

    let g = model_garden(2026, 200, 500, 50);
    println!(
        "model garden: {} replicas, {} candidates over 50 pure-noise predictors, 25 training cases",
        g.replicas, g.candidates
    );
    println!(
        "the validation winner beats predicting zero in {:.1}% of the replicas; on fresh data, in {:.1}%",
        g.validation_beats_zero, g.test_beats_zero
    );
    println!(
        "mean squared error of the winner: {:.3} on validation (predicting zero: 1.000) and {:.3} on fresh data",
        g.mean_validation_error, g.mean_test_error
    );

    let (mean_sequence, pool) = hot_hand();
    let (pct_six, mean_longest) = ghost(2026, 2_000);
    println!(
        "hot hand: mean of the proportions per sequence = {:.4} (5/12 = {:.4}); pooled proportion = {:.4}",
        mean_sequence,
        5.0 / 12.0,
        pool
    );
    println!(
        "100 people tossing 20 coins: in {:.1}% of the experiments someone strings 6 heads in a row; the longest run averages {:.1}",
        pct_six, mean_longest
    );

    let c = garden_of_paths(2026, 4_000);
    println!(
        "garden of paths ({} experiments with no effect at all): with Simmons' four freedoms, someone finds p < 0.05 in {:.1}% of them; with the large family, in {:.1}%; the pre-registered test flags {:.1}%",
        c.replicas, c.small, c.large, c.pre_registered
    );
    println!(
        "Mayo severity: 1 - p = {:.2} in the pre-registered test with p = 0.04; 1 - false-positive rate = {:.2} in the garden",
        1.0 - 0.04,
        1.0 - c.large / 100.0
    );

    for odds in [1.0 / 1000.0, 1.0 / 10.0, 1.0 / 3.0] {
        println!(
            "Ioannidis with R = 1/{:.0}, power 0.80: PPV = {:.3} with alpha 0.05 and {:.3} with the garden alpha",
            1.0 / odds,
            ppv(0.80, odds, 0.05),
            ppv(0.80, odds, c.large / 100.0)
        );
    }

    let good: Vec<u8> = (1..=8).map(|_| u8::from(band_passes(true))).collect();
    let mut one_missing = good.clone();
    one_missing[4] = u8::from(band_passes(false));
    println!(
        "safeguard panel A1-A8, card with the eight safeguards: bits {:?}, approved = {}",
        good,
        good.iter().all(|b| *b == 1)
    );
    println!(
        "the same card without one band's safeguard: {} of 8 bits set, approved = {}",
        one_missing.iter().filter(|b| **b == 1).count(),
        one_missing.iter().all(|b| *b == 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garden_inflates_false_positive_and_hot_hand_has_negative_bias() {
        let (exact, pooled) = hot_hand();
        assert!((pooled - 0.5).abs() < 1e-9);
        assert!(exact < 0.42);
        let c = garden_of_paths(2026, 200);
        assert!(c.pre_registered < 10.0);
        assert!(c.large > c.small && c.small > c.pre_registered);
    }

    #[test]
    fn the_garden_of_paths_without_any_effect_stays_inside_its_bounds() {
        // Without effect, every freedom is a fair test on its own (about 5%); the garden can only add.
        let c = garden_of_paths(7, 2_000);
        assert!(
            (3.5..7.5).contains(&c.pre_registered),
            "pre-registered {}",
            c.pre_registered
        );
        assert!(c.small > c.pre_registered && c.large > c.small);
    }
}
