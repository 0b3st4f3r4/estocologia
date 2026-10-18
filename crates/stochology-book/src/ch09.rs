//! Chapter 9 — the ladder of cause and effect: confounder against collider and a minimal ICP.
//!
//! Self-contained laboratory, no external dependencies: built-in SplitMix64, the
//! invariance test through the likelihood ratio of an F, and the draw with a fixed seed.
//! Usage: cargo run --release

// ---------------------------------------------------------------- measured output
#[derive(Clone)]
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        stochology_core::rng::splitmix64_step(&mut self.0)
    }
    /// Uniform on [0,1).
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Standard normal (Marsaglia, polar).
    pub fn normal(&mut self) -> f64 {
        loop {
            let u = 2.0 * self.uniform() - 1.0;
            let v = 2.0 * self.uniform() - 1.0;
            let s = u * u + v * v;
            if s > 0.0 && s < 1.0 {
                return u * (-2.0 * s.ln() / s).sqrt();
            }
        }
    }
}

// ---------------------------------------------------------------- statistics

fn ln_gamma(x: f64) -> f64 {
    // Lanczos, g = 7, n = 9.
    const C: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        std::f64::consts::PI.ln() - (std::f64::consts::PI * x).sin().ln() - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = C[0];
        let t = x + 7.5;
        for (i, c) in C.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

/// Regularised incomplete beta I_x(a,b), through Lentz's continued fraction.
pub fn incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let front =
        (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        front * fraction(x, a, b) / a
    } else {
        1.0 - front * fraction(1.0 - x, b, a) / b
    }
}

fn fraction(x: f64, a: f64, b: f64) -> f64 {
    let mut c = 1.0;
    let mut d = 1.0 - (a + b) * x / (a + 1.0);
    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..300 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let mut num = m * (b - m) * x / ((a + m2 - 1.0) * (a + m2));
        d = 1.0 + num * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + num / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        h *= d * c;
        num = -(a + m) * (a + b + m) * x / ((a + m2) * (a + m2 + 1.0));
        d = 1.0 + num * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + num / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-14 {
            break;
        }
    }
    h
}

/// P(F > f) for an F with (d1, d2) degrees of freedom.
pub fn p_f_greater(f: f64, d1: f64, d2: f64) -> f64 {
    if f <= 0.0 {
        return 1.0;
    }
    let x = d2 / (d2 + d1 * f);
    incomplete_beta(x, d2 / 2.0, d1 / 2.0)
}
// ---------------------------------------------------------------- the world

/// One environment (one site, one year, one school): how many rows and how it shifts the cause X.
pub struct Environment {
    pub name: &'static str,
    pub n: usize,
    pub shift: f64,
    /// Standard deviation of the noise of the symptom Z in this environment. It is
    /// what changes the meaning of Z from one site to the other.
    pub noise_z: f64,
}

pub const THREE_SITES: [Environment; 3] = [
    Environment {
        name: "site_A",
        n: 4000,
        shift: -1.0,
        noise_z: 0.5,
    },
    Environment {
        name: "site_B",
        n: 4000,
        shift: 0.0,
        noise_z: 2.0,
    },
    Environment {
        name: "site_C",
        n: 4000,
        shift: 1.0,
        noise_z: 3.0,
    },
];

/// One row: the cause X, the symptom Z (child of X), the noise W, the outcome Y and the environment.
#[derive(Clone, Copy)]
pub struct Row {
    pub environment: usize,
    pub x: f64,
    pub z: f64,
    pub w: f64,
    pub y: f64,
}

/// Generates the world. `instability = 0` is the honest case:
/// Y = 2X + noise (same mechanism in every environment), X -> Z, W pure noise.
/// With `instability = 1`, the environment acts directly on Y and the mechanism changes.
pub fn generate(seed: u64, environments: &[Environment], instability: f64) -> Vec<Row> {
    let mut rng = SplitMix64::new(seed);
    let mut rows = Vec::new();
    for (e, env) in environments.iter().enumerate() {
        for _ in 0..env.n {
            let x = env.shift + rng.normal();
            let z = 2.0 * x + env.noise_z * rng.normal();
            let w = rng.normal();
            // instability = 0: the mechanism X -> Y is the same at every site.
            // instability > 0: the site also enters through X and the coefficient changes.
            let y = 2.0 * x + rng.normal() + instability * 2.0 * x;
            rows.push(Row {
                environment: e,
                x,
                z,
                w,
                y,
            });
        }
    }
    rows
}

// ---------------------------------------------------------------- the test

/// Result of the invariance test for one candidate set.
pub struct Test {
    pub name: String,
    pub f: f64,
    pub p: f64,
    /// Fitted common slope (for single-variable sets).
    pub common_slope: f64,
    /// Slopes per environment (for single-variable sets).
    pub slopes_by_environment: Vec<f64>,
}

pub fn column(rows: &[Row], e: usize, name: &str) -> Vec<f64> {
    rows.iter()
        .filter(|l| l.environment == e)
        .map(|l| value(l, name))
        .collect()
}

fn moments(xs: &[f64], ys: &[f64]) -> (f64, f64, f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..xs.len() {
        sxx += (xs[i] - mx) * (xs[i] - mx);
        sxy += (xs[i] - mx) * (ys[i] - my);
    }
    (mx, my, sxx, sxy)
}

/// Fit with a common slope and a free intercept per environment: returns (rss, common slope).
pub fn restricted_fit(rows: &[Row], environments: &[Environment], name: &str) -> (f64, f64) {
    let (mut sxx, mut sxy) = (0.0, 0.0);
    for (e, _) in environments.iter().enumerate() {
        let xs = column(rows, e, name);
        let ys = column(rows, e, "y");
        let (_, _, a, b) = moments(&xs, &ys);
        sxx += a;
        sxy += b;
    }
    let beta = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    let mut rss = 0.0;
    for (e, env) in environments.iter().enumerate() {
        let xs = column(rows, e, name);
        let ys = column(rows, e, "y");
        let (mx, my, _, _) = moments(&xs, &ys);
        let a0 = my - beta * mx;
        for i in 0..env.n {
            let r = ys[i] - a0 - beta * xs[i];
            rss += r * r;
        }
    }
    (rss, beta)
}

/// Fit with slope and intercept free per environment: returns (rss, slopes).
pub fn free_fit(rows: &[Row], environments: &[Environment], name: &str) -> (f64, Vec<f64>) {
    let mut rss = 0.0;
    let mut slopes = Vec::new();
    for (e, env) in environments.iter().enumerate() {
        let xs = column(rows, e, name);
        let ys = column(rows, e, "y");
        let (mx, my, sxx, sxy) = moments(&xs, &ys);
        let beta = if sxx > 0.0 { sxy / sxx } else { 0.0 };
        let a0 = my - beta * mx;
        for i in 0..env.n {
            let r = ys[i] - a0 - beta * xs[i];
            rss += r * r;
        }
        slopes.push(beta);
    }
    (rss, slopes)
}

fn solve(a: &mut [Vec<f64>], b: &mut [f64]) -> Vec<f64> {
    let n = b.len();
    for i in 0..n {
        let mut pivot = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[pivot][i].abs() {
                pivot = k;
            }
        }
        a.swap(i, pivot);
        b.swap(i, pivot);
        for k in i + 1..n {
            let factor = a[k][i] / a[i][i];
            for l in i..n {
                a[k][l] -= factor * a[i][l];
            }
            b[k] -= factor * b[i];
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for l in i + 1..n {
            s -= a[i][l] * x[l];
        }
        x[i] = s / a[i][i];
    }
    x
}

/// The invariance test (linear ICP) for one candidate set of variables.
/// Null hypothesis: P(Y | X_S, E = e) is the same in every environment.
pub fn icp(rows: &[Row], environments: &[Environment], names: &[&str]) -> Test {
    let n_total: usize = environments.iter().map(|a| a.n).sum();
    let e = environments.len();
    let p = names.len();
    let k_free = e * (1 + p);
    let k_restricted = e + p;

    if p == 0 {
        // the empty set has intercepts only: it never rejects on slope.
        let slopes = vec![0.0; environments.len()];
        return Test {
            name: "(empty)".into(),
            f: 0.0,
            p: 1.0,
            common_slope: 0.0,
            slopes_by_environment: slopes,
        };
    }

    // Restricted: free intercept per environment + slope common to all.
    // Equivalent to subtracting each environment's mean before fitting.
    let mut a = vec![vec![0.0; p]; p];
    let mut b = vec![0.0; p];
    let mut columns_by_environment: Vec<Vec<Vec<f64>>> = Vec::new();
    for (i, env) in environments.iter().enumerate() {
        let columns: Vec<Vec<f64>> = names.iter().map(|nm| column(rows, i, nm)).collect();
        let ys = column(rows, i, "y");
        let (mx, my, _, _) = moments(&columns[0], &ys);
        let mxs: Vec<f64> = columns
            .iter()
            .map(|c| c.iter().sum::<f64>() / env.n as f64)
            .collect();
        let _ = mx;
        for k in 0..p {
            for l in 0..p {
                let mut s = 0.0;
                for r in 0..env.n {
                    s += (columns[k][r] - mxs[k]) * (columns[l][r] - mxs[l]);
                }
                a[k][l] += s;
            }
            let mut s = 0.0;
            for r in 0..env.n {
                s += (columns[k][r] - mxs[k]) * (ys[r] - my);
            }
            b[k] += s;
        }
        columns_by_environment.push(columns);
    }
    let beta = solve(&mut a, &mut b);
    let mut rss_restricted = 0.0;
    for (i, env) in environments.iter().enumerate() {
        let columns = &columns_by_environment[i];
        let ys = column(rows, i, "y");
        let mxs: Vec<f64> = columns
            .iter()
            .map(|c| c.iter().sum::<f64>() / env.n as f64)
            .collect();
        let my = ys.iter().sum::<f64>() / env.n as f64;
        let mut a0 = my;
        for k in 0..p {
            a0 -= beta[k] * mxs[k];
        }
        for r in 0..env.n {
            let mut predicted = a0;
            for k in 0..p {
                predicted += beta[k] * columns[k][r];
            }
            rss_restricted += (ys[r] - predicted) * (ys[r] - predicted);
        }
    }

    // Free: intercept and slope per environment (system of (p+1) unknowns).
    let mut rss_free = 0.0;
    let mut slopes_free = Vec::new();
    for (i, env) in environments.iter().enumerate() {
        let columns = &columns_by_environment[i];
        let ys = column(rows, i, "y");
        let mut m = vec![vec![0.0; p + 1]; p + 1];
        let mut v = vec![0.0; p + 1];
        m[0][0] = env.n as f64;
        v[0] = ys.iter().sum::<f64>();
        for k in 0..p {
            let sk: f64 = columns[k].iter().sum();
            m[0][k + 1] = sk;
            m[k + 1][0] = sk;
            v[k + 1] = columns[k]
                .iter()
                .zip(ys.iter())
                .map(|(x, y)| x * y)
                .sum::<f64>();
            for l in 0..p {
                m[k + 1][l + 1] = columns[k]
                    .iter()
                    .zip(columns[l].iter())
                    .map(|(x, z)| x * z)
                    .sum::<f64>();
            }
        }
        let solution = solve(&mut m, &mut v);
        for r in 0..env.n {
            let mut predicted = solution[0];
            for k in 0..p {
                predicted += solution[k + 1] * columns[k][r];
            }
            rss_free += (ys[r] - predicted) * (ys[r] - predicted);
        }
        slopes_free.push(solution[1]);
    }

    let df_num = (k_free - k_restricted) as f64;
    let df_den = (n_total - k_free) as f64;
    let f = ((rss_restricted - rss_free) / df_num) / (rss_free / df_den);
    let p_value = p_f_greater(f, df_num, df_den);
    Test {
        name: names.join("+"),
        f,
        p: p_value,
        common_slope: beta[0],
        slopes_by_environment: slopes_free,
    }
}

/// The intersection of the sets accepted by the test.
pub fn intersection(accepted: &[Vec<&str>]) -> Vec<String> {
    if accepted.is_empty() {
        return Vec::new();
    }
    let mut current: Vec<String> = accepted[0].iter().map(|s| s.to_string()).collect();
    for set in accepted.iter().skip(1) {
        current.retain(|v| set.contains(&v.as_str()));
    }
    current
}

pub fn value(l: &Row, name: &str) -> f64 {
    match name {
        "x" => l.x,
        "z" => l.z,
        "w" => l.w,
        "y" => l.y,
        _ => f64::NAN,
    }
}

/// Pearson correlation between two columns, over all rows.
pub fn correlation(rows: &[Row], a: &str, b: &str) -> f64 {
    let xs: Vec<f64> = rows.iter().map(|l| value(l, a)).collect();
    let ys: Vec<f64> = rows.iter().map(|l| value(l, b)).collect();
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..xs.len() {
        sxy += (xs[i] - mx) * (ys[i] - my);
        sxx += (xs[i] - mx).powi(2);
        syy += (ys[i] - my).powi(2);
    }
    sxy / (sxx * syy).sqrt()
}

/// Slope of the simple regression of Y on `name`, over all rows together (the naive count).
pub fn naive_slope(rows: &[Row], name: &str) -> f64 {
    let xs: Vec<f64> = rows.iter().map(|l| value(l, name)).collect();
    let ys: Vec<f64> = rows.iter().map(|l| l.y).collect();
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let (mut sxy, mut sxx) = (0.0, 0.0);
    for i in 0..xs.len() {
        sxy += (xs[i] - mx) * (ys[i] - my);
        sxx += (xs[i] - mx).powi(2);
    }
    sxy / sxx
}

/// The three steps of Pearl, classified by the shape of the question.
pub fn pearl_step(question: &str) -> &'static str {
    if question.contains("do(") {
        return "2. intervention";
    }
    if question.starts_with("if ") && question.contains('?') {
        return "2. intervention";
    }
    if question.starts_with("would ") || question.contains("have happened") {
        return "3. counterfactual";
    }
    "1. association"
}

const QUESTIONS: [&str; 3] = [
    "neighbourhoods with more police have more crime",
    "if we double policing, does crime fall?",
    "would this robbery have happened with a patrol car on the corner?",
];

fn kidney_stones() {
    println!("== Chapter 9 :: Simpson's paradox in kidney-stone surgery ==");
    // Charig and colleagues (1986): successes in 350 open procedures and 350 in the new technique.
    let (small_open, small_new) = (81.0, 234.0);
    let (large_open, large_new) = (192.0, 55.0);
    let (n_small_open, n_small_new) = (87.0, 270.0);
    let (n_large_open, n_large_new) = (263.0, 80.0);
    let open = small_open + large_open;
    let new = small_new + large_new;
    let n_open = n_small_open + n_large_open;
    let n_new = n_small_new + n_large_new;
    let small = n_small_open + n_small_new;
    let large = n_large_open + n_large_new;
    println!(
        "small: open {}/{} = {:.4}  new {}/{} = {:.4}",
        small_open as u32,
        n_small_open as u32,
        small_open / n_small_open,
        small_new as u32,
        n_small_new as u32,
        small_new / n_small_new
    );
    println!(
        "large: open {}/{} = {:.4}  new {}/{} = {:.4}",
        large_open as u32,
        n_large_open as u32,
        large_open / n_large_open,
        large_new as u32,
        n_large_new as u32,
        large_new / n_large_new
    );
    println!(
        "raw:   open {}/{} = {:.4}  new {}/{} = {:.4}",
        open as u32,
        n_open as u32,
        open / n_open,
        new as u32,
        n_new as u32,
        new / n_new
    );
    let w_small = small / (small + large);
    let w_large = large / (small + large);
    let weighted_open =
        w_small * (small_open / n_small_open) + w_large * (large_open / n_large_open);
    let weighted_new = w_small * (small_new / n_small_new) + w_large * (large_new / n_large_new);
    println!(
        "weight of the strata in the whole population: small {:.4}, large {:.4}",
        w_small, w_large
    );
    println!(
        "weighted by the whole population: open {:.4}  new {:.4}  -> the advantage flips back",
        weighted_open, weighted_new
    );
    println!(
        "fraction of hard cases in the open surgery = {:.4}, in the new one = {:.4}",
        n_large_open / n_open,
        n_large_new / n_new
    );
    println!();
}

fn pearl_ladder() {
    println!("== Chapter 9 :: the three steps of Pearl ==");
    for q in QUESTIONS {
        println!("  [{:>16}] {}", pearl_step(q), q);
    }
    println!();
}

fn one_round() {
    println!("== Chapter 9 :: a minimal ICP, three sites, three candidates ==");
    let rows = generate(2026, &THREE_SITES, 0.0);
    println!(
        "rows = {} (4000 per site), X -> Y with coefficient 2, X -> Z (child) with noise different per site, W pure noise",
        rows.len()
    );
    println!(
        "naive correlation with Y: corr(X,Y) = {:.4}  corr(Z,Y) = {:.4}  corr(W,Y) = {:.4}",
        correlation(&rows, "x", "y"),
        correlation(&rows, "z", "y"),
        correlation(&rows, "w", "y")
    );
    println!(
        "naive slope (all sites together): X = {:.4}  Z = {:.4}  W = {:.4}",
        naive_slope(&rows, "x"),
        naive_slope(&rows, "z"),
        naive_slope(&rows, "w")
    );
    let sets: [&[&str]; 7] = [
        &["x"],
        &["z"],
        &["w"],
        &["x", "z"],
        &["x", "w"],
        &["z", "w"],
        &["x", "z", "w"],
    ];
    let mut accepted: Vec<Vec<&str>> = Vec::new();
    for set in sets {
        let t = icp(&rows, &THREE_SITES, set);
        let slopes: Vec<String> = t
            .slopes_by_environment
            .iter()
            .map(|b| format!("{:.3}", b))
            .collect();
        println!(
            "  candidate {{{:>5}}}  F = {:>7.3}  p = {:.3e}  common = {:.3}  per site = [{}]  {}",
            t.name,
            t.f,
            t.p,
            t.common_slope,
            slopes.join(", "),
            if t.p > 0.05 { "ACCEPTED" } else { "rejected" }
        );
        if t.p > 0.05 {
            accepted.push(set.to_vec());
        }
    }
    let t = icp(&rows, &THREE_SITES, &[]);
    println!(
        "  candidate {{empty}}  F = {:.3}  p = {:.1}  (intercepts only)",
        t.f, t.p
    );
    println!(
        "accepted sets: {} -> intersection S_hat = {{{}}}",
        accepted.len(),
        intersection(&accepted).join(", ")
    );
    println!();
}

fn many_rounds(repetitions: usize, instability: f64, label: &str) {
    let sets: [&[&str]; 7] = [
        &["x"],
        &["z"],
        &["w"],
        &["x", "z"],
        &["x", "w"],
        &["z", "w"],
        &["x", "z", "w"],
    ];
    let mut rejections = [0usize; 7];
    let mut sum_f = [0.0f64; 7];
    let mut accepted_x = 0usize;
    let mut inter_x = 0usize;
    let mut inter_z = 0usize;
    let mut inter_empty = 0usize;
    let mut all_accepted = 0usize;
    let mut empty_by_power = 0usize;
    let mut inter_w = 0usize;
    let mut inter_with_x_without_w = 0usize;
    let mut naive_x = 0.0;
    let mut naive_z = 0.0;
    for r in 0..repetitions {
        let rows = generate(2026 + r as u64, &THREE_SITES, instability);
        let mut accepted: Vec<Vec<&str>> = Vec::new();
        for (i, set) in sets.iter().enumerate() {
            let t = icp(&rows, &THREE_SITES, set);
            sum_f[i] += t.f;
            if t.p <= 0.05 {
                rejections[i] += 1;
            } else {
                accepted.push(set.to_vec());
            }
        }
        if icp(&rows, &THREE_SITES, &["x"]).p > 0.05 {
            accepted_x += 1;
        }
        let inter = intersection(&accepted);
        if inter == vec!["x".to_string()] {
            inter_x += 1;
        }
        if inter == vec!["z".to_string()] {
            inter_z += 1;
        }
        if inter == vec!["w".to_string()] {
            inter_w += 1;
        }
        if inter.contains(&"x".to_string()) && !inter.contains(&"w".to_string()) {
            inter_with_x_without_w += 1;
        }
        if inter.is_empty() {
            inter_empty += 1;
            if accepted.len() == sets.len() {
                all_accepted += 1;
            } else if !accepted.iter().any(|c| c.contains(&"x")) {
                empty_by_power += 1;
            }
        }
        naive_x += naive_slope(&rows, "x");
        naive_z += naive_slope(&rows, "z");
    }
    let n = repetitions as f64;
    println!("== Chapter 9 :: {} repetitions, {}", repetitions, label);
    println!(
        "mean naive slope (all sites together): X = {:.4}  Z = {:.4}",
        naive_x / n,
        naive_z / n
    );
    for (i, set) in sets.iter().enumerate() {
        println!(
            "  {{{:>5}}}  mean F = {:>8.2}  rejection rate = {:.3}",
            set.join("+"),
            sum_f[i] / n,
            rejections[i] as f64 / n
        );
    }
    println!(
        "  {{x}} accepted in {:.3} of the rounds",
        accepted_x as f64 / n
    );
    println!(
        "  intersection = {{x}} in {:.3} of the rounds",
        inter_x as f64 / n
    );
    println!(
        "  intersection = {{z}} in {:.3} of the rounds",
        inter_z as f64 / n
    );
    println!(
        "  intersection = {{w}} in {:.3} of the rounds",
        inter_w as f64 / n
    );
    println!(
        "  intersection contains x and does not contain w in {:.3} of the rounds",
        inter_with_x_without_w as f64 / n
    );
    let empty_by_w =
        (inter_empty as f64 / n) - (all_accepted as f64 / n) - (empty_by_power as f64 / n);
    println!(
        "  empty intersection in {:.3} (of which {:.3} because only the noise W was left, {:.3} because the cause itself was not left, and {:.3} because W entered some sets and not others)",
        inter_empty as f64 / n,
        inter_w as f64 / n,
        empty_by_power as f64 / n,
        empty_by_w
    );
    println!();
}

pub fn run() {
    kidney_stones();
    pearl_ladder();
    one_round();
    many_rounds(200, 0.0, "stable mechanism: Y = 2X + noise at every site");
    many_rounds(
        200,
        1.0,
        "unstable mechanism: the coefficient of X in Y changes from site to site",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pearl_steps_and_icp_intersection_work() {
        assert!(pearl_step("P(Y | X)").contains("association"));
        assert!(pearl_step("P(Y | do(X))").contains("intervention"));
        assert!(pearl_step("would this robbery have happened?").contains("counterfactual"));
        assert_eq!(
            intersection(&[vec!["x", "z"], vec!["x"]]),
            vec!["x".to_string()]
        );
    }
}
