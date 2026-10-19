//! Chapter 19 — the gate of the invariant set: a minimal ICP over three school environments.
//!
//! Self-contained laboratory, no external dependencies: embedded SplitMix64, least squares by
//! normal equations, invariance test by permutation (Kruskal-Wallis of location + Brown-Forsythe
//! of scale, Bonferroni over 2) and the saturation rule of K = 3 (1-NN leaving the row out).
//! Run from the repository root:
//!     cargo run --release --bin labs -- --chapter 19

const SEED: u64 = 2019;
const N_PER_ENV: usize = 600;
const N_ENVS: usize = 3;
const B_PERM: usize = 999;
const ALPHA: f64 = 0.05;
const SATURATION_THRESHOLD: f64 = 0.99;

// ---------------------------------------------------------------- seeded chance

struct Sm64 {
    state: u64,
}

impl Sm64 {
    fn new(seed: u64) -> Self {
        Sm64 {
            state: stochology_core::rng::offset_seed(seed) ^ 0x9E37_79B9_7F4A_7C15,
        }
    }
    fn next(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.state)
    }
    fn uniform(&mut self) -> f64 {
        ((self.next() >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn normal(&mut self) -> f64 {
        let u1 = self.uniform().max(1e-12);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

// ---------------------------------------------------------------- the three worlds

struct Data {
    y: Vec<f64>,
    env: Vec<usize>,
    cands: Vec<Vec<f64>>,
    names: Vec<&'static str>,
}

/// 0: the environment intervenes in X. 1: the environment acts directly on Y. 2: world 0 plus the column x5,
/// which is the environment itself (one class per school).
fn generate(world: u8, seed: u64) -> Data {
    let mut rng = Sm64::new(seed);
    let mean_x1 = [0.0, 1.5, 3.0];
    let mean_x2 = [4.0, 6.0, 8.0];
    // x4 is a variable that the environment shifts and that does not act on Y: "the trip to school".
    let mean_x4 = [0.0, 2.0, 4.0];
    let shift_y = match world {
        1 => [0.0, 6.0, -4.0],
        _ => [0.0, 0.0, 0.0],
    };
    let mut y = Vec::new();
    let mut env = Vec::new();
    let (mut c1, mut c2, mut c3, mut c4, mut c5) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for e in 0..N_ENVS {
        for _ in 0..N_PER_ENV {
            let x1 = mean_x1[e] + 1.5 * rng.normal();
            let x2 = mean_x2[e] + 1.0 * rng.normal();
            let x3 = rng.normal();
            let noise = rng.normal();
            let yy = 1.0 * x1 + 0.8 * x2 + shift_y[e] + noise;
            // x4 is a child of Y: a sensor that copies the score with noise (like the light sensors of the
            // light tunnel, which are children of the LEDs).
            let x4 = mean_x4[e] + 1.0 * rng.normal();
            y.push(yy);
            env.push(e);
            c1.push(x1);
            c2.push(x2);
            c3.push(x3);
            c4.push(x4);
            c5.push(e as f64);
        }
    }
    let mut cands = vec![c1, c2, c3];
    let mut names = vec!["x1 study", "x2 sleep", "x3 noise"];
    if world != 1 {
        // the sensor enters worlds A and C; in world B only the first three candidates stay
        cands.push(c4);
        names.push("x4 transport");
    }
    if world == 2 {
        cands.push(c5);
        names.push("x5 class");
    }
    Data {
        y,
        env,
        cands,
        names,
    }
}

// ---------------------------------------------------------------- least squares

/// Least squares by normal equations. `x` comes in columns: x[column][row].
#[allow(clippy::needless_range_loop)] // normal equations written with explicit indices
fn ols(x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let p = x.len();
    let n = y.len();
    let mut a = vec![vec![0.0; p + 1]; p];
    for i in 0..n {
        for j in 0..p {
            for k in 0..p {
                a[j][k] += x[j][i] * x[k][i];
            }
            a[j][p] += x[j][i] * y[i];
        }
    }
    for j in 0..p {
        a[j][j] += 1e-10;
    }
    for col in 0..p {
        let mut piv = col;
        for r in (col + 1)..p {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        a.swap(col, piv);
        let d = a[col][col];
        for k in col..=p {
            a[col][k] /= d;
        }
        for r in 0..p {
            if r != col {
                let f = a[r][col];
                if f != 0.0 {
                    for k in col..=p {
                        a[r][k] -= f * a[col][k];
                    }
                }
            }
        }
    }
    (0..p).map(|j| a[j][p]).collect()
}

/// Expansion of K = 2: a categorical variable becomes a block of indicators (reference = most frequent level),
/// a continuous one enters as a linear term.
fn columns(data: &Data, s: &[usize]) -> Vec<Vec<f64>> {
    let n = data.y.len();
    let mut cols = vec![vec![1.0; n]];
    for &j in s {
        let v = &data.cands[j];
        let mut distinct: Vec<f64> = v.clone();
        distinct.sort_by(|a, b| a.partial_cmp(b).unwrap());
        distinct.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
        if distinct.len() <= 10 {
            let mut most_frequent = distinct[0];
            let mut most_frequent_n = 0usize;
            for &d in &distinct {
                let c = v.iter().filter(|x| (**x - d).abs() < 1e-12).count();
                if c > most_frequent_n {
                    most_frequent_n = c;
                    most_frequent = d;
                }
            }
            for &d in &distinct {
                if (d - most_frequent).abs() < 1e-12 {
                    continue;
                }
                cols.push(
                    v.iter()
                        .map(|x| if (*x - d).abs() < 1e-12 { 1.0 } else { 0.0 })
                        .collect(),
                );
            }
        } else {
            cols.push(v.clone());
        }
    }
    cols
}

// ---------------------------------------------------------------- the two tests

fn kruskal(u: &[f64], g: &[usize], k: usize) -> f64 {
    let n = u.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| u[a].partial_cmp(&u[b]).unwrap());
    let mut rank = vec![0.0; n];
    let mut i = 0;
    let mut correction = 0.0;
    while i < n {
        let mut j = i;
        while j + 1 < n && (u[idx[j + 1]] - u[idx[i]]).abs() < 1e-12 {
            j += 1;
        }
        let r = ((i + j) as f64) / 2.0 + 1.0;
        for t in i..=j {
            rank[idx[t]] = r;
        }
        let size = (j - i + 1) as f64;
        correction += size * size * size - size;
        i = j + 1;
    }
    let (mut sum, mut count) = (vec![0.0; k], vec![0.0; k]);
    for row in 0..n {
        sum[g[row]] += rank[row];
        count[g[row]] += 1.0;
    }
    let mut h = 0.0;
    for gg in 0..k {
        h += count[gg] * (sum[gg] / count[gg] - (n as f64 + 1.0) / 2.0).powi(2);
    }
    h *= 12.0 / ((n as f64) * (n as f64 + 1.0));
    h / (1.0 - correction / ((n as f64).powi(3) - n as f64)).max(1e-12)
}

#[allow(clippy::needless_range_loop)] // group index used on three parallel slices
fn brown_forsythe(u: &[f64], g: &[usize], k: usize) -> f64 {
    let n = u.len();
    let mut median = vec![0.0; k];
    for gg in 0..k {
        let mut v: Vec<f64> = (0..n).filter(|&i| g[i] == gg).map(|i| u[i]).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        median[gg] = if v.len() % 2 == 1 {
            v[v.len() / 2]
        } else {
            0.5 * (v[v.len() / 2 - 1] + v[v.len() / 2])
        };
    }
    let z: Vec<f64> = (0..n).map(|i| (u[i] - median[g[i]]).abs()).collect();
    let (mut sum, mut count) = (vec![0.0; k], vec![0.0; k]);
    let mut total = 0.0;
    for i in 0..n {
        sum[g[i]] += z[i];
        count[g[i]] += 1.0;
        total += z[i];
    }
    let grand = total / n as f64;
    let (mut between, mut within) = (0.0, 0.0);
    for gg in 0..k {
        let mg = sum[gg] / count[gg];
        between += count[gg] * (mg - grand).powi(2);
        for i in 0..n {
            if g[i] == gg {
                within += (z[i] - mg).powi(2);
            }
        }
    }
    ((n - k) as f64 / (k - 1) as f64) * between / within.max(1e-300)
}

/// p of location, p of scale and p of the set (Bonferroni over 2), by permutation of the labels.
fn invariance_test(u: &[f64], env: &[usize], seed: u64) -> (f64, f64, f64) {
    let h_obs = kruskal(u, env, N_ENVS);
    let f_obs = brown_forsythe(u, env, N_ENVS);
    let mut rng = Sm64::new(seed);
    let mut g = env.to_vec();
    let (mut more_h, mut more_f) = (0usize, 0usize);
    for _ in 0..B_PERM {
        for i in (1..g.len()).rev() {
            let j = ((rng.uniform() * ((i + 1) as f64)) as usize).min(i);
            g.swap(i, j);
        }
        if kruskal(u, &g, N_ENVS) >= h_obs - 1e-12 {
            more_h += 1;
        }
        if brown_forsythe(u, &g, N_ENVS) >= f_obs - 1e-12 {
            more_f += 1;
        }
    }
    let p_loc = (1 + more_h) as f64 / (B_PERM + 1) as f64;
    let p_scale = (1 + more_f) as f64 / (B_PERM + 1) as f64;
    (p_loc, p_scale, (2.0 * p_loc.min(p_scale)).min(1.0))
}

/// Mean hit rate of the 1-NN leaving the row out, when predicting the environment from the columns of S.
fn accuracy_1nn(data: &Data, s: &[usize]) -> f64 {
    let n = data.y.len();
    if s.is_empty() {
        return 1.0 / N_ENVS as f64;
    }
    let mut cols: Vec<Vec<f64>> = Vec::new();
    for &j in s {
        let v = &data.cands[j];
        let m = v.iter().sum::<f64>() / n as f64;
        let dp = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n as f64)
            .sqrt()
            .max(1e-12);
        cols.push(v.iter().map(|x| (x - m) / dp).collect());
    }
    let mut hits = 0.0;
    for i in 0..n {
        let mut cell: Vec<usize> = Vec::new();
        for j in 0..n {
            if j != i && cols.iter().all(|c| (c[i] - c[j]).abs() < 1e-9) {
                cell.push(j);
            }
        }
        if !cell.is_empty() {
            let equal = cell.iter().filter(|&&j| data.env[j] == data.env[i]).count();
            hits += equal as f64 / cell.len() as f64;
        } else {
            let (mut best, mut neighbour) = (f64::INFINITY, 0usize);
            for j in 0..n {
                if j == i {
                    continue;
                }
                let d: f64 = cols.iter().map(|c| (c[i] - c[j]).powi(2)).sum();
                if d < best {
                    best = d;
                    neighbour = j;
                }
            }
            if data.env[neighbour] == data.env[i] {
                hits += 1.0;
            }
        }
    }
    hits / n as f64
}

// ---------------------------------------------------------------- the minimal ICP

struct Row {
    s: Vec<usize>,
    p_loc: f64,
    p_scale: f64,
    p_set: f64,
    accepted: bool,
    accuracy: f64,
    saturated: bool,
}

struct Report {
    rows: Vec<Row>,
    accepted_family: usize,
    accepted_all: usize,
    saturated_tested: usize,
    saturated_accepted: usize,
    s_with_rule: Vec<usize>,
    s_without_rule: Vec<usize>,
}

fn icp(data: &Data, saturation_rule: bool) -> Report {
    let p = data.cands.len();
    let n = data.y.len();
    let mut rows = Vec::new();
    for mask in 0..(1usize << p) {
        let s: Vec<usize> = (0..p).filter(|j| mask & (1 << j) != 0).collect();
        let x = columns(data, &s);
        let b = ols(&x, &data.y);
        let fitted: Vec<f64> = (0..n)
            .map(|i| (0..x.len()).map(|c| x[c][i] * b[c]).sum())
            .collect();
        let r: Vec<f64> = (0..n).map(|i| data.y[i] - fitted[i]).collect();
        let abs: Vec<f64> = r.iter().map(|v| v.abs()).collect();
        let b_abs = ols(&x, &abs);
        let mean_abs = abs.iter().sum::<f64>() / n as f64;
        let u: Vec<f64> = (0..n)
            .map(|i| {
                let fit: f64 = (0..x.len()).map(|c| x[c][i] * b_abs[c]).sum();
                let s_hat = fit.max(0.1 * mean_abs).max(1e-12);
                r[i] / s_hat
            })
            .collect();
        let (p_loc, p_scale, p_set) =
            invariance_test(&u, &data.env, SEED ^ ((mask as u64 + 1) * 0x9E37));
        let accuracy = accuracy_1nn(data, &s);
        let saturated = saturation_rule && accuracy >= SATURATION_THRESHOLD;
        rows.push(Row {
            s,
            p_loc,
            p_scale,
            p_set,
            accepted: p_set >= ALPHA,
            accuracy,
            saturated,
        });
    }
    let in_family: Vec<&Row> = rows.iter().filter(|l| !l.saturated).collect();
    let accepted_family = in_family.iter().filter(|l| l.accepted).count();
    let accepted_all = rows.iter().filter(|l| l.accepted).count();
    let saturated_accepted = rows
        .iter()
        .filter(|l| l.accepted && l.accuracy >= SATURATION_THRESHOLD)
        .count();
    let intersection = |source: &[&Row]| -> Vec<usize> {
        let accepted: Vec<&&Row> = source.iter().filter(|l| l.accepted).collect();
        if accepted.is_empty() {
            return Vec::new();
        }
        (0..p)
            .filter(|j| accepted.iter().all(|l| l.s.contains(j)))
            .collect()
    };
    let with_rule = intersection(&in_family);
    let all: Vec<&Row> = rows.iter().collect();
    let without_rule = intersection(&all);
    let saturated_tested = rows
        .iter()
        .filter(|l| l.accuracy >= SATURATION_THRESHOLD)
        .count();
    Report {
        rows,
        accepted_family,
        accepted_all,
        saturated_tested,
        saturated_accepted,
        s_with_rule: with_rule,
        s_without_rule: without_rule,
    }
}

fn name(data: &Data, s: &[usize]) -> String {
    if s.is_empty() {
        return "{}".to_string();
    }
    format!(
        "{{{}}}",
        s.iter()
            .map(|&j| data.names[j])
            .collect::<Vec<_>>()
            .join(", ")
    )
}

pub fn run() {
    println!("== Chapter 19 :: the gate of the invariant set ==");
    println!(
        "seed {} | {} environments x {} rows | {} permutations per test | alpha = {:.2}",
        SEED, N_ENVS, N_PER_ENV, B_PERM, ALPHA
    );
    let titles = [
        "world A: the environment intervenes in X (3 schools, 4 candidates)",
        "world B: the environment acts directly on Y (the year, 3 candidates)",
        "world C: world A plus x5, the column that is the environment itself (5 candidates)",
    ];
    let of_interest: [Vec<usize>; 9] = [
        vec![],
        vec![0],
        vec![1],
        vec![2],
        vec![3],
        vec![0, 1],
        vec![0, 1, 3],
        vec![4],
        vec![0, 1, 4],
    ];
    for (world, title) in titles.iter().enumerate() {
        let data = generate(world as u8, SEED);
        let r = icp(&data, true);
        println!("\n{}", title);
        println!(
            "  {:<34} {:>7} {:>8} {:>8}  {:<10} {:>7}",
            "set", "p_location", "p_scale", "p_set", "verdict", "1-NN"
        );
        for target in of_interest.iter() {
            if target.iter().any(|&j| j >= data.cands.len()) {
                continue;
            }
            if let Some(l) = r.rows.iter().find(|l| l.s == *target) {
                println!(
                    "  {:<34} {:>7.4} {:>8.4} {:>8.4}  {:<10} {:>7.3}",
                    name(&data, &l.s),
                    l.p_loc,
                    l.p_scale,
                    l.p_set,
                    if l.accepted { "accepted" } else { "rejected" },
                    l.accuracy
                );
            }
        }
        if std::env::var("CAP19_ALL").is_ok() {
            for l in &r.rows {
                println!(
                    "    [debug] {:<34} p_loc={:.4} p_scale={:.4} p_set={:.4} {} 1nn={:.3} sat={}",
                    name(&data, &l.s),
                    l.p_loc,
                    l.p_scale,
                    l.p_set,
                    if l.accepted { "accepted" } else { "rejected" },
                    l.accuracy,
                    l.saturated
                );
            }
        }
        println!(
            "  accepted in the family: {} of {} (with the saturated ones it would be {}) | saturated tested: {} | saturated accepted: {}",
            r.accepted_family,
            1usize << data.cands.len(),
            r.accepted_all,
            r.saturated_tested,
            r.saturated_accepted
        );
        println!(
            "  S_hat with the saturation rule = {}  | without the rule = {}  | gate: {}",
            name(&data, &r.s_with_rule),
            name(&data, &r.s_without_rule),
            if r.s_with_rule.is_empty() {
                "FAILS (S_hat empty)"
            } else {
                "PASSES (S_hat not empty)"
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ols_recovers_a_line_and_kruskal_is_zero_on_identical_groups() {
        let x = vec![vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 2.0, 3.0, 4.0]];
        let y = vec![3.0, 5.0, 7.0, 9.0];
        let b = ols(&x, &y);
        assert!((b[0] - 1.0).abs() < 1e-6);
        assert!((b[1] - 2.0).abs() < 1e-6);
        let u = [1.0, 2.0, 3.0, 1.0, 2.0, 3.0];
        let g = [0, 0, 0, 1, 1, 1];
        assert!(kruskal(&u, &g, 2).abs() < 1e-9);
    }
}
