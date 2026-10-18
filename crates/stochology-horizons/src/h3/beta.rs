//! The Beta fit of the three quartiles — the family of the \`h1-real/c\` domain (a frequency).
//!
//! The \`empirical/p12-shelf\` protocol says that real H1 fits **Beta** to the three central quartiles, because the
//! quantity is a frequency in (0, 1) and the Normal does not respect the support. Here live the regularised
//! incomplete Beta function (Lentz continued fraction, Numerical Recipes standard), the quantile by bisection and the fit by
//! deterministic search over (log a, log b).
//!
//! Declared and not implemented: confidence interval for the parameters and the maximum-likelihood fit —
//! the criterion here is the squared error on the three quartiles, as the protocol declares.

/// Logarithm of the gamma, by Lanczos (z > 0) with reflection for z < 0.5.
pub fn ln_gamma(z: f64) -> f64 {
    const G: [f64; 9] = [
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
    if z < 0.5 {
        // Gamma(z) Gamma(1-z) = pi / sin(pi z), everything in logarithm
        (std::f64::consts::PI / (std::f64::consts::PI * z).sin()).ln() - ln_gamma(1.0 - z)
    } else {
        let z = z - 1.0;
        let mut x = G[0];
        for (i, g) in G.iter().enumerate().skip(1) {
            x += g / (z + i as f64);
        }
        let t = z + 7.5;
        0.5 * (2.0 * std::f64::consts::PI).ln() + (z + 0.5) * t.ln() - t + x.ln()
    }
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0_f64;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=300 {
        let mf = m as f64;
        let m2 = 2.0 * mf;
        // even step
        let aa = mf * (b - mf) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        h *= d * c;
        // odd step
        let aa = -(a + mf) * (qab + mf) * x / ((a + m2) * (a + m2 + 1.0));
        d = 1.0 + aa * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-12 {
            break;
        }
    }
    h
}

/// The regularised incomplete Beta function \`I_x(a, b)\`.
pub fn incomplete_beta(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let ln_bt = ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln();
    let bt = ln_bt.exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

/// The quantile of the Beta: \`x\` such that \`I_x(a, b) = p\`, by deterministic bisection.
pub fn beta_quantile(p: f64, a: f64, b: f64) -> f64 {
    let (mut low, mut high) = (0.0_f64, 1.0_f64);
    for _ in 0..200 {
        let mid = 0.5 * (low + high);
        if incomplete_beta(a, b, mid) < p {
            low = mid;
        } else {
            high = mid;
        }
    }
    0.5 * (low + high)
}

/// The two fitted parameters and the maximum error on the three quartiles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BetaPrior {
    pub a: f64,
    pub b: f64,
    pub max_quartile_error: f64,
}

/// Fits the Beta to the three central quartiles, by coarse grid and local refinement — deterministic.
pub fn fit_beta(q25: f64, q50: f64, q75: f64) -> BetaPrior {
    let error = |a: f64, b: f64| -> f64 {
        [(q25, 0.25), (q50, 0.5), (q75, 0.75)]
            .iter()
            .map(|(q, p)| (beta_quantile(*p, a, b) - q).abs())
            .fold(0.0_f64, f64::max)
    };
    let (mut best_a, mut best_b, mut best_error) = (1.0_f64, 1.0_f64, error(1.0, 1.0));
    for ia in 0..34 {
        let a = 0.2 * 1.3_f64.powi(ia);
        for ib in 0..34 {
            let b = 0.2 * 1.3_f64.powi(ib);
            let e = error(a, b);
            if e < best_error {
                best_a = a;
                best_b = b;
                best_error = e;
            }
        }
    }
    for _ in 0..14 {
        let (a, b) = (best_a, best_b);
        for da in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            for db in [-2.0, -1.0, 0.0, 1.0, 2.0] {
                let ca = (a * (1.0 + 0.04 * da)).clamp(0.01, 5_000.0);
                let cb = (b * (1.0 + 0.04 * db)).clamp(0.01, 5_000.0);
                let e = error(ca, cb);
                if e < best_error {
                    best_a = ca;
                    best_b = cb;
                    best_error = e;
                }
            }
        }
    }
    BetaPrior {
        a: best_a,
        b: best_b,
        max_quartile_error: best_error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_gamma_matches_factorials() {
        for n in 1..=10u32 {
            let obtained = ln_gamma(n as f64 + 1.0);
            let expected = (1..=n).map(|k| (k as f64).ln()).sum::<f64>();
            assert!(
                (obtained - expected).abs() < 1e-9,
                "ln Gamma({n}+1): {obtained} against {expected}"
            );
        }
    }

    #[test]
    fn incomplete_beta_matches_the_uniform() {
        for x in [0.1_f64, 0.35, 0.5, 0.9] {
            let v = incomplete_beta(1.0, 1.0, x);
            assert!((v - x).abs() < 1e-9, "I_{x}(1,1) = {v}");
        }
    }

    #[test]
    fn incomplete_beta_matches_the_closed_form_of_beta_2_2() {
        // I_x(2,2) = 3x^2 - 2x^3
        for x in [0.2_f64, 0.5, 0.8] {
            let v = incomplete_beta(2.0, 2.0, x);
            let exact = 3.0 * x * x - 2.0 * x * x * x;
            assert!((v - exact).abs() < 1e-9, "I_{x}(2,2): {v} against {exact}");
        }
    }

    #[test]
    fn the_quantile_recovers_the_known_quartiles() {
        let q25 = beta_quantile(0.25, 2.0, 5.0);
        let q50 = beta_quantile(0.50, 2.0, 5.0);
        let q75 = beta_quantile(0.75, 2.0, 5.0);
        assert!(q25 < q50 && q50 < q75, "{q25} {q50} {q75}");
        assert!((q50 - 0.2634).abs() < 0.005, "median of Beta(2,5): {q50}");
        assert!((beta_quantile(0.5, 1.0, 1.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn the_fit_recovers_the_beta_that_generated_the_quartiles() {
        let p = fit_beta(
            beta_quantile(0.25, 2.0, 5.0),
            beta_quantile(0.50, 2.0, 5.0),
            beta_quantile(0.75, 2.0, 5.0),
        );
        assert!((p.a - 2.0).abs() < 0.2, "a = {}", p.a);
        assert!((p.b - 5.0).abs() < 0.5, "b = {}", p.b);
        assert!(
            p.max_quartile_error < 0.01,
            "error {}",
            p.max_quartile_error
        );
    }

    #[test]
    fn the_fit_of_the_uniform_stays_symmetric() {
        let p = fit_beta(0.25, 0.50, 0.75);
        assert!((p.a - p.b).abs() < 0.1, "a {} b {}", p.a, p.b);
        assert!(
            p.max_quartile_error < 0.01,
            "error {}",
            p.max_quartile_error
        );
    }

    #[test]
    fn the_fit_is_deterministic() {
        assert_eq!(fit_beta(0.1, 0.3, 0.6), fit_beta(0.1, 0.3, 0.6));
    }
}
