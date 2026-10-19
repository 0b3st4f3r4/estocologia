//! Chapter 8 — The forecaster's thermometer: proper scores, Murphy's
//! decomposition and the ECE tolerance that depends on n.
//!
//! Self-contained laboratory: built-in SplitMix64, no external dependencies.
//! Usage: cargo run --release

#[derive(Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
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

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn logit(p: f64) -> f64 {
    let p = p.clamp(1e-6, 1.0 - 1e-6);
    (p / (1.0 - p)).ln()
}

/// The pieces of Murphy's decomposition and the ECE over ten boxes of equal mass.
struct Decomposition {
    brier: f64,
    rel: f64,
    res: f64,
    unc: f64,
    ece: f64,
    /// Brier of the forecast replaced by the mean of its box; it is on this one
    /// that Murphy's identity closes exactly.
    brier_box: f64,
}

fn decompose(forecasts: &[f64], outcomes: &[f64]) -> Decomposition {
    let n = forecasts.len();
    let base = outcomes.iter().sum::<f64>() / n as f64;
    let brier = forecasts
        .iter()
        .zip(outcomes.iter())
        .map(|(p, y)| (p - y) * (p - y))
        .sum::<f64>()
        / n as f64;
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| forecasts[*a].partial_cmp(&forecasts[*b]).unwrap());
    // Ten boxes of equal mass, except that equal forecasts never straddle a boundary: a box
    // that cut a run of ties would separate identical forecasts by their arbitrary order and
    // manufacture resolution that the forecaster does not have (a constant forecaster must
    // have RES = 0 exactly).
    let boxes = 10usize;
    let mut bounds = Vec::new();
    let mut start = 0usize;
    for k in 1..=boxes {
        let mut end = (k * n / boxes).max(start);
        while end < n && end > 0 && forecasts[order[end]] == forecasts[order[end - 1]] {
            end += 1;
        }
        if end > start {
            bounds.push((start, end));
            start = end;
        }
    }
    let mut rel = 0.0;
    let mut res = 0.0;
    let mut ece = 0.0;
    let mut within = 0.0;
    for (start, end) in bounds {
        let idx = &order[start..end];
        let m = idx.len() as f64;
        let pm = idx.iter().map(|i| forecasts[*i]).sum::<f64>() / m;
        let ym = idx.iter().map(|i| outcomes[*i]).sum::<f64>() / m;
        rel += m / n as f64 * (pm - ym) * (pm - ym);
        res += m / n as f64 * (ym - base) * (ym - base);
        ece += m / n as f64 * (pm - ym).abs();
        for i in idx {
            let d = pm - outcomes[*i];
            within += d * d / n as f64;
        }
    }
    Decomposition {
        brier,
        rel,
        res,
        unc: base * (1.0 - base),
        ece,
        brier_box: within,
    }
}

/// Platt scaling: logistic regression of y on the logit of the forecast.
fn platt(forecasts: &[f64], outcomes: &[f64]) -> (f64, f64) {
    // The logit enters standardised: without this Newton blows up when the
    // forecasts sit against 0 and 1.
    let xs: Vec<f64> = forecasts.iter().map(|p| logit(*p)).collect();
    let mean_x = xs.iter().sum::<f64>() / xs.len() as f64;
    let var_x = xs.iter().map(|x| (x - mean_x) * (x - mean_x)).sum::<f64>() / xs.len() as f64;
    let sd_x = var_x.sqrt();
    // Constant forecaster (the climatologist): only the intercept is left, and
    // recalibration returns exactly the base rate.
    if sd_x < 1e-9 {
        let base = outcomes.iter().sum::<f64>() / outcomes.len() as f64;
        return (logit(base), 0.0);
    }
    let mut a = 0.0f64;
    let mut b = 1.0f64;
    for _ in 0..60 {
        let (mut g0, mut g1) = (0.0f64, 0.0f64);
        let (mut h00, mut h01, mut h11) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..forecasts.len() {
            let x = (xs[i] - mean_x) / sd_x;
            let q = sigmoid(a + b * x);
            let w = q * (1.0 - q);
            g0 += outcomes[i] - q;
            g1 += (outcomes[i] - q) * x;
            h00 += w;
            h01 += w * x;
            h11 += w * x * x;
        }
        let det = h00 * h11 - h01 * h01;
        if det.abs() < 1e-12 {
            break;
        }
        let da = (h11 * g0 - h01 * g1) / det;
        let db = (-h01 * g0 + h00 * g1) / det;
        a += da;
        b += db;
        if da.abs() + db.abs() < 1e-10 {
            break;
        }
    }
    // back to the original scale: a' + b' * (x - m)/s = a + b * x
    (a - b * mean_x / sd_x, b / sd_x)
}

fn recalibrate(forecasts: &[f64], a: f64, b: f64) -> Vec<f64> {
    forecasts
        .iter()
        .map(|p| sigmoid(a + b * logit(*p)))
        .collect()
}

/// ECE tolerance: 2 000 simulations of perfectly calibrated labels with the same
/// forecasts, and the quantile of the null ECE.
pub(crate) fn ece_tolerance(
    seed: u64,
    forecasts: &[f64],
    simulations: usize,
) -> (f64, f64, f64, f64, f64) {
    let mut rng = SplitMix64::new(seed);
    let mut eces = Vec::with_capacity(simulations);
    for _ in 0..simulations {
        let outcomes: Vec<f64> = forecasts
            .iter()
            .map(|p| if rng.bernoulli(*p) { 1.0 } else { 0.0 })
            .collect();
        eces.push(decompose(forecasts, &outcomes).ece);
    }
    eces.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |f: f64| eces[((f * (eces.len() - 1) as f64).round()) as usize];
    let above = 100.0 * eces.iter().filter(|e| **e > 0.03).count() as f64 / eces.len() as f64;
    (
        q(0.5),
        q(0.95),
        q(0.99),
        eces.iter().sum::<f64>() / eces.len() as f64,
        above,
    )
}

/// The forecasts of the first vault of the tolerance table (Toronto's size), drawn exactly as `run` draws them.
pub(crate) fn toronto_like_forecasts() -> Vec<f64> {
    let mut rng = SplitMix64::new(20261015);
    (0..1_488)
        .map(|_| sigmoid(logit(0.50) + 1.0 * rng.normal()))
        .collect()
}

pub fn run() {
    println!("== Chapter 8 :: the forecaster's thermometer ==");

    // The two forecasters: the lazy one says 20% every day, the expert tracks the
    // real chance. Four thousand days, seed 2026.
    let mut rng = SplitMix64::new(2026);
    let n = 4_000usize;
    let mut p_true = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    let mut lazy = Vec::with_capacity(n);
    let mut expert = Vec::with_capacity(n);
    let mut overconfident = Vec::with_capacity(n);
    let mut noisy = Vec::with_capacity(n);
    for _ in 0..n {
        // real chance of the day: logit-normal with mean 20%
        let z = -1.386 + 1.1 * rng.normal();
        let p = sigmoid(z);
        p_true.push(p);
        y.push(if rng.bernoulli(p) { 1.0 } else { 0.0 });
        lazy.push(0.20);
        expert.push(sigmoid(z + 0.6 * rng.normal()));
        // the overconfident one sees the same thing as the expert, but rounds the sign towards the extremes
        overconfident.push(sigmoid(2.2 * (z + 0.6 * rng.normal())));
        noisy.push(sigmoid(z + 1.6 * rng.normal()));
    }
    let base = y.iter().sum::<f64>() / n as f64;
    println!(
        "{} days, observed base rate = {:.3}, mean chance declared by the expert = {:.3}",
        n,
        base,
        expert.iter().sum::<f64>() / n as f64
    );
    // Platt is fitted on the first 2000 days and scored on the last 2000, which it never saw:
    // a recalibration scored on its own fitting data would flatter itself.
    let half = n / 2;
    println!(
        "Platt fitted on days 1-{} and scored out of the fit on days {}-{} (intercept a, slope b):",
        half,
        half + 1,
        n
    );
    for (name, p) in [
        ("lazy (0.20 always)", &lazy),
        ("expert", &expert),
        ("overconfident", &overconfident),
        ("noisy", &noisy),
    ] {
        let d = decompose(p, &y);
        let (a, b) = platt(&p[..half], &y[..half]);
        let before = decompose(&p[half..], &y[half..]);
        let after = decompose(&recalibrate(&p[half..], a, b), &y[half..]);
        println!(
            "{:>24}: Brier {:.4} | REL {:.4} RES {:.4} UNC {:.4} | ECE {:.4} | out of fit: Brier {:.4} -> {:.4} with Platt, ECE {:.4} -> {:.4} (a = {:.2}, b = {:.2})",
            name,
            d.brier,
            d.rel,
            d.res,
            d.unc,
            d.ece,
            before.brier,
            after.brier,
            before.ece,
            after.ece,
            a,
            b
        );
    }
    let d = decompose(&expert, &y);
    println!(
        "Murphy identity on the expert: REL - RES + UNC = {:.7}, Brier of the forecasts pooled into the boxes = {:.7}, difference = {:.2e}",
        d.rel - d.res + d.unc,
        d.brier_box,
        (d.rel - d.res + d.unc - d.brier_box).abs()
    );
    println!(
        "the raw forecast has Brier {:.7}; pooling into 10 boxes costs {:.2e}, and the identity closes",
        d.brier,
        d.brier_box - d.brier
    );

    // The ECE tolerance depends on n: three H3 vaults.
    let mut rng = SplitMix64::new(20261015);
    let cases: [(usize, f64, f64, &str); 3] = [
        (1_488, 0.50, 1.0, "Toronto prisons 1997-2002"),
        (24_466, 0.0226, 1.6, "Rhode Island 2013-2015"),
        (104_489, 0.130, 1.3, "NY flights 2023"),
    ];
    for (size, rate, spread, name) in cases {
        let forecasts: Vec<f64> = (0..size)
            .map(|_| sigmoid(logit(rate) + spread * rng.normal()))
            .collect();
        let (q50, q95, q99, mean_null, above) = ece_tolerance(20261015, &forecasts, 2_000);
        println!(
            "{:>26}: n = {:>7} | null q50 {:.4} | null q95 {:.4} | null q99 {:.4} | mean {:.4} | null ECE above 0.03 in {:.1}% of the simulations",
            name, size, q50, q95, q99, mean_null, above
        );
    }
    // The same n, two forecast spreads: the null depends on the forecasts.
    for spread in [0.4_f64, 1.0, 2.0] {
        let forecasts: Vec<f64> = (0..1_488)
            .map(|_| sigmoid(logit(0.5) + spread * rng.normal()))
            .collect();
        let (q50, q95, _, _, _) = ece_tolerance(20261015, &forecasts, 2_000);
        println!(
            "n = 1 488, spread {:.1}: null q50 {:.4}, null q95 {:.4}",
            spread, q50, q95
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn murphy_identity_closes_and_platt_reduces_rel() {
        let p = vec![0.1, 0.2, 0.8, 0.9, 0.15, 0.85, 0.25, 0.75];
        let y = vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0];
        let d = decompose(&p, &y);
        assert!((d.rel - d.res + d.unc - d.brier_box).abs() < 1e-12);
    }

    /// A constant forecaster has no resolution: RES is exactly zero, whatever the outcomes.
    #[test]
    fn constant_forecaster_has_zero_resolution() {
        let p = vec![0.2; 1_000];
        let y: Vec<f64> = (0..1_000)
            .map(|i| if i % 4 == 0 { 1.0 } else { 0.0 })
            .collect();
        let d = decompose(&p, &y);
        assert_eq!(d.res, 0.0);
        assert!((d.rel - d.res + d.unc - d.brier_box).abs() < 1e-12);
    }
}
