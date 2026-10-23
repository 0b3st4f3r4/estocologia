//! Lens 2 — Time: Non-Ergodicity (Itô), Asymmetric Kelly Survival,
//! and Heavy-Tailed Mixing Time (§1.3.2).
//!
//! Implements:
//! - Itô quadratic variance drain:
//!   $$g_{\text{time}} = \mathbb{E}[\ln M] = \mu - \frac{1}{2}\sigma^2 < \ln \mathbb{E}[M] = \mu = g_{\text{ensemble}}$$
//! - Optimal Kelly Fraction for asymmetric returns (gain $b$, loss $a$, win probability $p$):
//!   $$f^* = \frac{p}{a} - \frac{1 - p}{b}$$
//! - Mixing-time audit for stochastic / Langevin sampling via the log-Sobolev constant
//!   associated with $\eta_{\mathrm{KL}}$ and warm-start bounds under tail profile $H_R = \pi(w > R)$.

use stochology_core::{median, Rng};

/// Non-Ergodicity and Kelly Survival Diagnostic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeKellyDiagnostic {
    /// Ensemble average growth rate $g_{\text{ensemble}} = \mu$.
    pub ensemble_rate_mu: f64,
    /// Volatility $\sigma$.
    pub volatility_sigma: f64,
    /// Compound time-average growth rate experienced by a single trajectory $g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$.
    pub time_rate_ito: f64,
    /// Quadratic variance drain $\frac{1}{2}\sigma^2$.
    pub variance_drain: f64,
    /// Indicates whether the system grows in the ensemble average ($g_{\text{ensemble}} > 0$) while collapsing in time ($g_{\text{time}} < 0$).
    pub non_ergodic_trap: bool,
    /// Optimal asymmetric Kelly fraction $f^* = \frac{p}{a} - \frac{1 - p}{b}$.
    pub asymmetric_kelly_fraction: f64,
    /// Exact logarithmic growth rate evaluated at the Kelly fraction $f^*$.
    pub log_growth_at_kelly: f64,
}

/// Computes Itô's time-average growth rate $g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$.
#[inline]
pub fn ito_time_growth_rate(mu: f64, sigma: f64) -> f64 {
    mu - 0.5 * sigma * sigma
}

/// Computes the Optimal Kelly Fraction for asymmetric returns (§1.3.2):
/// $$f^* = \frac{p}{a} - \frac{1 - p}{b}$$
/// where $p \in (0, 1)$ is the calibrated win probability, $b > 0$ is the fractional gain on success,
/// and $a \in (0, 1]$ is the fractional loss on failure.
#[inline]
pub fn asymmetric_kelly_fraction(p: f64, gain_b: f64, loss_a: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "probability p must be in (0, 1)");
    assert!(
        gain_b > 0.0 && loss_a > 0.0,
        "gain b and loss a must be positive"
    );
    ((p / loss_a) - ((1.0 - p) / gain_b)).max(0.0)
}

/// Expected logarithmic growth rate $\mathbb{E}[\ln(1 + R(f))] = p \ln(1 + f b) + (1 - p)\ln(1 - f a)$.
#[inline]
pub fn asymmetric_bet_log_growth(p: f64, gain_b: f64, loss_a: f64, fraction_f: f64) -> f64 {
    if 1.0 - fraction_f * loss_a <= 0.0 || 1.0 + fraction_f * gain_b <= 0.0 {
        f64::NEG_INFINITY
    } else {
        p * (1.0 + fraction_f * gain_b).ln() + (1.0 - p) * (1.0 - fraction_f * loss_a).ln()
    }
}

impl TimeKellyDiagnostic {
    pub fn compute(mu: f64, sigma: f64, p: f64, gain_b: f64, loss_a: f64) -> Self {
        let g_time = ito_time_growth_rate(mu, sigma);
        let f_star = asymmetric_kelly_fraction(p, gain_b, loss_a);
        let g_kelly = asymmetric_bet_log_growth(p, gain_b, loss_a, f_star);
        Self {
            ensemble_rate_mu: mu,
            volatility_sigma: sigma,
            time_rate_ito: g_time,
            variance_drain: 0.5 * sigma * sigma,
            non_ergodic_trap: mu > 0.0 && g_time < 0.0,
            asymmetric_kelly_fraction: f_star,
            log_growth_at_kelly: g_kelly,
        }
    }
}

/// Simulates multiplicative trajectories with an absorbing ruin barrier (`ruin_barrier_fraction`),
/// returning `(ensemble_mean, trajectory_median, ruin_probability)`.
#[allow(clippy::too_many_arguments)]
pub fn simulate_absorbing_trajectories(
    seed: u64,
    steps: usize,
    trajectories: usize,
    p: f64,
    gain_b: f64,
    loss_a: f64,
    exposure_fraction: f64,
    ruin_barrier_fraction: f64,
) -> (f64, f64, f64) {
    if trajectories == 0 {
        return (1.0, 1.0, 0.0);
    }
    let mut rng = Rng::new(seed);
    let mut final_wealth = Vec::with_capacity(trajectories);
    let mut ruined = 0usize;
    for _ in 0..trajectories {
        let mut w = 1.0;
        for _ in 0..steps {
            if w <= ruin_barrier_fraction {
                w = ruin_barrier_fraction;
                break;
            }
            if rng.bernoulli(p) {
                w *= 1.0 + exposure_fraction * gain_b;
            } else {
                w *= (1.0 - exposure_fraction * loss_a).max(0.0);
            }
        }
        if w <= ruin_barrier_fraction {
            ruined += 1;
        }
        final_wealth.push(w);
    }
    let ens_mean = final_wealth.iter().sum::<f64>() / (trajectories as f64);
    let traj_med = median(&final_wealth);
    let p_ruin = (ruined as f64) / (trajectories as f64);
    (ens_mean, traj_med, p_ruin)
}

/// Mixing-Time Audit for Langevin / MCMC sampling under Heavy-Tailed Posterior Profiles (§1.3.2):
/// $$d_{\mathrm{TV}}(\mu_t, \pi) \le \sqrt{\frac{1}{2} D_{\mathrm{KL}}(\mu_0 \| \pi)\, \eta_{\mathrm{KL}}^t} + H_R$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MixingTimeAudit {
    pub local_eta_kl: f64,
    /// Log-Sobolev constant $\alpha_{\mathrm{LSI}} = -\frac{1}{2}\ln \eta_{\mathrm{KL}}$.
    pub log_sobolev_constant: f64,
    /// Tail profile mass $H_R = \pi(w > R)$ outside the warm-start truncation ball.
    pub tail_profile_h_r: f64,
    pub initial_kl: f64,
    /// Minimum number of steps $t$ required to reach total variation tolerance $\varepsilon$
    /// (or `usize::MAX` if $\varepsilon \le H_R$ renders the tolerance unreachable).
    pub mixing_steps: usize,
}

impl MixingTimeAudit {
    pub fn compute(
        local_eta_kl: f64,
        tail_profile_h_r: f64,
        initial_kl: f64,
        tolerance_eps: f64,
    ) -> Self {
        assert!(local_eta_kl > 0.0 && local_eta_kl < 1.0);
        assert!(tolerance_eps > 0.0 && tail_profile_h_r >= 0.0);
        let alpha_lsi = -0.5 * local_eta_kl.ln();
        let steps = if tolerance_eps <= tail_profile_h_r {
            usize::MAX
        } else {
            let eff_eps = tolerance_eps - tail_profile_h_r;
            if 0.5 * initial_kl <= eff_eps * eff_eps {
                0
            } else {
                let num = (2.0 * eff_eps * eff_eps / initial_kl).ln();
                let den = local_eta_kl.ln();
                (num / den).ceil() as usize
            }
        };
        Self {
            local_eta_kl,
            log_sobolev_constant: alpha_lsi,
            tail_profile_h_r,
            initial_kl,
            mixing_steps: steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peters_coin_and_asymmetric_kelly() {
        // Peters coin: p = 0.5, gain b = 0.50 (+50%), loss a = 0.40 (-40%)
        // Full exposure f = 1.0 has positive ensemble mean (+5%), but E[ln M] < 0!
        // Optimal Kelly fraction f* = 0.5/0.4 - 0.5/0.5 = 1.25 - 1.0 = 0.25 achieves positive time growth!
        let diag = TimeKellyDiagnostic::compute(0.05, 0.45, 0.50, 0.50, 0.40);
        assert!(diag.non_ergodic_trap);
        assert!((diag.asymmetric_kelly_fraction - 0.25).abs() < 1e-12);
        assert!(diag.log_growth_at_kelly > 0.0);

        let (_, med_overbet, ruin_overbet) =
            simulate_absorbing_trajectories(2026, 300, 1000, 0.5, 0.50, 0.40, 1.0, 0.05);
        let (_, med_kelly, ruin_kelly) =
            simulate_absorbing_trajectories(2026, 300, 1000, 0.5, 0.50, 0.40, 0.25, 0.05);
        assert!(med_kelly > 1.5 && med_overbet < 0.1);
        assert!(
            ruin_overbet > 0.80 && ruin_kelly < 0.05,
            "ruin_overbet={ruin_overbet}, ruin_kelly={ruin_kelly}"
        );
        assert_eq!(
            simulate_absorbing_trajectories(2026, 10, 0, 0.5, 0.5, 0.4, 0.25, 0.05),
            (1.0, 1.0, 0.0)
        );
    }

    #[test]
    fn mixing_time_grows_as_contraction_weakens_and_respects_tail_mass_h_r() {
        let fast = MixingTimeAudit::compute(0.80, 0.001, 5.0, 0.05);
        let heavier_tail = MixingTimeAudit::compute(0.80, 0.03, 5.0, 0.05);
        let slow = MixingTimeAudit::compute(0.98, 0.01, 5.0, 0.05);
        let unreachable = MixingTimeAudit::compute(0.80, 0.06, 5.0, 0.05);
        assert!(heavier_tail.mixing_steps > fast.mixing_steps);
        assert!(slow.mixing_steps > fast.mixing_steps * 5);
        assert_eq!(unreachable.mixing_steps, usize::MAX);
    }
}
