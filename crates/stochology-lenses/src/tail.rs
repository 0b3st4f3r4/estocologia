//! Lens 3 — Scale and Tail: Hill Estimator and Moment Existence Regimes (§1.3.3).
//!
//! Implements:
//! - Hill's semi-parametric tail index estimator over the $k$ largest order statistics:
//!   $$\hat{\alpha}_{k,n} = \left(\frac{1}{k}\sum_{i=1}^k \ln \frac{X_{(n-i+1)}}{X_{(n-k)}}\right)^{-1}$$
//! - Moment existence regime classification ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$);
//! - Penalized effective tail sample size $n_{\mathrm{eff}}(p)$ and robust scale summaries (median, MAD, $q_{90}, q_{99}$).

use stochology_core::{median_absolute_deviation, quantile_sorted};

/// Moment existence regime dictated by tail exponent $\alpha$ (§1.3.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailMomentRegime {
    /// $\alpha \le 1$: Neither theoretical mean nor variance exists (Law of Large Numbers fails).
    InfiniteMeanAndVariance,
    /// $1 < \alpha \le 2$: Theoretical mean exists, but variance is infinite (Classical CLT fails; $z$-scores are invalid).
    FiniteMeanInfiniteVariance,
    /// $\alpha > 2$: Finite mean and variance, though higher-order moments may still diverge.
    FiniteMeanAndVariance,
}

/// Complete Hill tail diagnostic on an empirical sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillTailDiagnostic {
    pub sample_size_n: usize,
    pub order_statistics_k: usize,
    pub threshold_x_nk: f64,
    pub alpha_hill: f64,
    /// Asymptotic standard error $\hat{\alpha}_{k,n} / \sqrt{k}$.
    pub asymptotic_se_alpha: f64,
    pub regime: TailMomentRegime,
    pub median: f64,
    pub mad: f64,
    pub quantile_90: f64,
    pub quantile_99: f64,
    /// Effective sample size penalized for tail estimation at $q_{99}$.
    pub penalized_tail_n_eff: f64,
}

/// Computes Hill's estimator $\hat{\alpha}_{k,n}$ on a slice of positive observations.
pub fn hill_estimator(xs: &[f64], k: usize) -> f64 {
    let mut pos: Vec<f64> = xs.iter().copied().filter(|&x| x > 0.0).collect();
    pos.sort_by(|a, b| a.total_cmp(b));
    let n = pos.len();
    assert!(
        k >= 2 && k < n,
        "Hill estimator requires 2 <= k < number of positive observations"
    );
    let threshold = pos[n - k - 1];
    assert!(threshold > 0.0);
    let mean_log_excess: f64 = pos[(n - k)..]
        .iter()
        .map(|&x| (x / threshold).ln())
        .sum::<f64>()
        / (k as f64);
    if mean_log_excess <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / mean_log_excess
    }
}

/// Dekkers, Einmahl & de Haan (1989) moment estimator $\hat{\xi}_{\mathrm{DEdH}} \in \mathbb{R}$ for the extreme-value
/// shape parameter $\xi = 1/\alpha$, valid across all three generalized extreme value domains
/// ($\xi > 0$ Fréchet heavy-tailed, $\xi = 0$ Gumbel exponential-tailed, $\xi < 0$ Weibull bounded-tailed):
/// $$\hat{\xi}_{\mathrm{DEdH}} = M_{k,n}^{(1)} + 1 - \frac{1}{2}\left(1 - \frac{\bigl(M_{k,n}^{(1)}\bigr)^2}{M_{k,n}^{(2)}}\right)^{-1}, \qquad M_{k,n}^{(r)} = \frac{1}{k}\sum_{i=1}^k \left(\ln \frac{X_{(n-i+1)}}{X_{(n-k)}}\right)^r$$
pub fn dedh_moment_tail_index(xs: &[f64], k: usize) -> f64 {
    let mut pos: Vec<f64> = xs
        .iter()
        .copied()
        .filter(|&x| x.is_finite() && x > 0.0)
        .collect();
    pos.sort_by(|a, b| a.total_cmp(b));
    let n = pos.len();
    assert!(k >= 2 && k < n, "DEdH moment estimator requires 2 <= k < n");
    let threshold = pos[n - k - 1].max(1e-15);
    let mut m1 = 0.0;
    let mut m2 = 0.0;
    for &x in &pos[(n - k)..] {
        let l = (x / threshold).ln();
        m1 += l;
        m2 += l * l;
    }
    m1 /= k as f64;
    m2 /= k as f64;
    let ratio = (m1 * m1) / m2.max(1e-15);
    let denom = (1.0 - ratio).max(1e-12);
    m1 + 1.0 - 0.5 / denom
}

/// Generalized Pareto Distribution (GPD) Peaks-Over-Threshold (POT) Pickands-Balkema-de Haan fit
/// above threshold `threshold_u` via Hosking-Wallis (1987) Probability-Weighted Moments (PWM).
/// Returns `(xi_gpd, beta_gpd, pot_expected_shortfall_99)`.
pub fn gpd_pot_fit(xs: &[f64], threshold_u: f64) -> (f64, f64, f64) {
    let mut pos: Vec<f64> = xs.iter().copied().filter(|&x| x.is_finite()).collect();
    pos.sort_by(|a, b| a.total_cmp(b));
    let n = pos.len();
    let mut excesses: Vec<f64> = pos
        .iter()
        .copied()
        .filter(|&x| x > threshold_u)
        .map(|x| x - threshold_u)
        .collect();
    assert!(
        excesses.len() >= 4,
        "GPD POT fit requires at least 4 exceedances above threshold_u"
    );
    excesses.sort_by(|a, b| a.total_cmp(b));
    let k = excesses.len() as f64;
    let alpha_0: f64 = excesses.iter().sum::<f64>() / k;
    let mut alpha_1 = 0.0;
    for (idx, &y) in excesses.iter().enumerate() {
        let w = 1.0 - ((idx as f64) + 0.35) / k;
        alpha_1 += w * y;
    }
    alpha_1 /= k;
    let denom = (alpha_0 - 2.0 * alpha_1).max(1e-12);
    let xi = (2.0 - alpha_0 / denom).clamp(-0.49, 0.95);
    let beta = ((2.0 * alpha_0 * alpha_1) / denom).max(1e-12);

    // POT 99% VaR and Expected Shortfall ES_0.99 = (VaR_0.99 + beta - xi * u) / (1 - xi)
    let p_tail = 0.01_f64;
    let zeta_u = (k / (n as f64)).max(p_tail);
    let var_99 = if xi.abs() > 1e-6 {
        threshold_u + (beta / xi) * ((zeta_u / p_tail).powf(xi) - 1.0)
    } else {
        threshold_u + beta * (zeta_u / p_tail).ln()
    };
    let es_99 = (var_99 + beta - xi * threshold_u) / (1.0 - xi).max(0.05);
    (xi, beta, es_99)
}

/// Empirical Expected Shortfall (Conditional Value-at-Risk, $\mathrm{ES}_{0.99} = \mathbb{E}[X \mid X \ge q_{0.99}]$).
pub fn expected_shortfall_99(xs: &[f64]) -> f64 {
    let mut clean: Vec<f64> = xs.iter().copied().filter(|&x| x.is_finite()).collect();
    assert!(!clean.is_empty());
    clean.sort_by(|a, b| a.total_cmp(b));
    let q99 = quantile_sorted(&clean, 0.99);
    let tail: Vec<f64> = clean.into_iter().filter(|&x| x >= q99).collect();
    stochology_core::mean(&tail)
}

impl HillTailDiagnostic {
    /// Audits the tail behavior of `xs` using `k_opt` (or automatic Clauset-Shalizi-Newman
    /// Kolmogorov-Smirnov distance minimization over $k \in [\max(10, n/100), \min(n/5, 250)]$,
    /// falling back to $\lfloor \sqrt{n} \rfloor$ if needed).
    pub fn audit(xs: &[f64], k_opt: Option<usize>) -> Self {
        let mut pos: Vec<f64> = xs
            .iter()
            .copied()
            .filter(|&x| x.is_finite() && x > 0.0)
            .collect();
        pos.sort_by(|a, b| a.total_cmp(b));
        let n = pos.len();
        assert!(n >= 10, "Hill audit requires at least 10 positive points");
        let k = k_opt
            .unwrap_or_else(|| {
                let fallback = ((n as f64).sqrt().floor() as usize).clamp(5, n / 2);
                let k_min = 10.max(n / 100);
                let k_max = (n / 5).min(250).min(n - 1);
                if k_min > k_max {
                    return fallback;
                }
                let mut best_k = fallback;
                let mut best_ks = f64::INFINITY;
                for cand_k in k_min..=k_max {
                    let alpha_k = hill_estimator(&pos, cand_k);
                    if !alpha_k.is_finite() || alpha_k <= 0.0 {
                        continue;
                    }
                    let thresh = pos[n - cand_k - 1];
                    let mut d_ks = 0.0_f64;
                    for (idx, &x) in pos[(n - cand_k)..].iter().enumerate() {
                        let emp_cdf = ((idx + 1) as f64) / (cand_k as f64);
                        let theo_cdf = 1.0 - (x / thresh).powf(-alpha_k);
                        d_ks = d_ks.max((emp_cdf - theo_cdf).abs());
                    }
                    if d_ks < best_ks {
                        best_ks = d_ks;
                        best_k = cand_k;
                    }
                }
                best_k
            })
            .clamp(2, n - 1);

        let threshold = pos[n - k - 1];
        let alpha = hill_estimator(&pos, k);
        let se = if alpha.is_finite() {
            alpha / (k as f64).sqrt()
        } else {
            0.0
        };
        let regime = if alpha <= 1.0 {
            TailMomentRegime::InfiniteMeanAndVariance
        } else if alpha <= 2.0 {
            TailMomentRegime::FiniteMeanInfiniteVariance
        } else {
            TailMomentRegime::FiniteMeanAndVariance
        };

        let med = quantile_sorted(&pos, 0.50);
        let mad = median_absolute_deviation(&pos);
        let q90 = quantile_sorted(&pos, 0.90);
        let q99 = quantile_sorted(&pos, 0.99);

        // Tail effective sample size at q99 grounded in Weissman (1978) / de Haan & Ferreira (2006)
        // extreme quantile variance scaling relative to the finite-variance threshold alpha_0 = 2.0.
        let raw_exceedances = ((n as f64) * 0.01).max(1.0);
        let penalized_n_eff = raw_exceedances * (alpha / 2.0).powi(2).clamp(0.1, 1.0);

        Self {
            sample_size_n: n,
            order_statistics_k: k,
            threshold_x_nk: threshold,
            alpha_hill: alpha,
            asymptotic_se_alpha: se,
            regime,
            median: med,
            mad,
            quantile_90: q90,
            quantile_99: q99,
            penalized_tail_n_eff: penalized_n_eff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn hill_estimator_recovers_pareto_exponent_and_classifies_moments() {
        let mut rng = Rng::new(2026);
        let true_alpha = 1.50;
        let samples: Vec<f64> = (0..4_000).map(|_| rng.pareto(1.0, true_alpha)).collect();
        let diag = HillTailDiagnostic::audit(&samples, Some(200));
        assert!((diag.alpha_hill - true_alpha).abs() < 0.20);
        assert_eq!(diag.regime, TailMomentRegime::FiniteMeanInfiniteVariance);
        assert!(diag.quantile_99 > diag.quantile_90 * 2.0);
        let expected_n_eff = 40.0 * (diag.alpha_hill / 2.0).powi(2).clamp(0.1, 1.0);
        assert!((diag.penalized_tail_n_eff - expected_n_eff).abs() < 1e-12);

        let diag_auto = HillTailDiagnostic::audit(&samples, None);
        assert!((diag_auto.alpha_hill - true_alpha).abs() < 0.20);
        assert!((40..=250).contains(&diag_auto.order_statistics_k));
        assert_eq!(hill_estimator(&[5.0; 20], 5), f64::INFINITY);

        let xi_dedh = dedh_moment_tail_index(&samples, 200);
        assert!((xi_dedh - (1.0 / true_alpha)).abs() < 0.20);
        let (xi_gpd, beta_gpd, es_pot) = gpd_pot_fit(&samples, diag.threshold_x_nk);
        assert!(xi_gpd > 0.2 && beta_gpd > 0.0 && es_pot > diag.quantile_99);
        assert!(expected_shortfall_99(&samples) > diag.quantile_99);
    }
}
