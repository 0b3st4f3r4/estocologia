//! Lens 5 — Decision: Cost-Loss Ratio ($C / (\eta L)$), EVPI, and Isotonic
//! CORP-Murphy PAV Decomposition (§1.3.5).
//!
//! Implements:
//! - Economic decision boundary with mitigation efficacy $\eta \in (0, 1]$:
//!   $$p > p^* \triangleq \frac{C}{\eta L}$$
//! - Expected Value of Perfect Information ($\mathrm{EVPI}$);
//! - Brier Score ($\mathrm{BS}$), Quantile Score ($\mathrm{QS}_\alpha$), and $\mathrm{CRPS}$;
//! - Exact isotonic **CORP** (*Consistent, Optimal, Reproducible, PAV-calibrated*,
//!   Arnold, Walz, Ziegel & Gneiting) decomposition:
//!   $$\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$$
//!   via the *Pool-Adjacent-Violators* (PAV) algorithm.

/// Computes the critical probability threshold $p^* = \frac{C}{\eta L}$.
#[inline]
pub fn critical_cost_loss_threshold(cost_c: f64, loss_l: f64, efficacy_eta: f64) -> f64 {
    assert!(
        cost_c > 0.0 && loss_l > 0.0 && efficacy_eta > 0.0 && efficacy_eta <= 1.0,
        "requires C > 0, L > 0, and eta in (0, 1]"
    );
    cost_c / (efficacy_eta * loss_l)
}

/// Economic Cost-Loss Rule and Expected Value of Perfect Information ($\mathrm{EVPI}$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostLossRule {
    pub protection_cost_c: f64,
    pub event_loss_l: f64,
    pub efficacy_eta: f64,
    pub critical_threshold_p_star: f64,
}

impl CostLossRule {
    pub fn new(protection_cost_c: f64, event_loss_l: f64, efficacy_eta: f64) -> Self {
        let p_star = critical_cost_loss_threshold(protection_cost_c, event_loss_l, efficacy_eta);
        Self {
            protection_cost_c,
            event_loss_l,
            efficacy_eta,
            critical_threshold_p_star: p_star,
        }
    }

    /// Returns `"protect"` if $p > p^*$ else `"do_not_protect"`.
    #[inline]
    pub fn recommend_action(&self, probability_p: f64) -> &'static str {
        if probability_p > self.critical_threshold_p_star {
            "protect"
        } else {
            "do_not_protect"
        }
    }

    /// Computes the Expected Value of Perfect Information ($\mathrm{EVPI}$) per unit and across `volume_units`:
    /// - If $p \le p^*$: $\mathrm{EVPI}_{\text{unit}} = p (\eta L - C)$;
    /// - If $p > p^*$: $\mathrm{EVPI}_{\text{unit}} = (1 - p) C$.
    pub fn calculate_evpi(&self, prior_probability_p: f64, volume_units: f64) -> f64 {
        let effective_gain =
            (self.efficacy_eta * self.event_loss_l - self.protection_cost_c).max(0.0);
        let unit_evpi = if prior_probability_p <= self.critical_threshold_p_star {
            prior_probability_p * effective_gain
        } else {
            (1.0 - prior_probability_p) * self.protection_cost_c
        };
        unit_evpi * volume_units.max(0.0)
    }
}

/// Strictly proper Quantile Score (pinball loss) at level $\alpha \in (0, 1)$ (§1.3.5):
/// $$\mathrm{QS}_\alpha(q_\alpha, y) = (\mathbb{I}\{y \le q_\alpha\} - \alpha)(q_\alpha - y)$$
#[inline]
pub fn quantile_score(alpha: f64, forecast_q: f64, observation_y: f64) -> f64 {
    assert!(alpha > 0.0 && alpha < 1.0);
    let ind = if observation_y <= forecast_q {
        1.0
    } else {
        0.0
    };
    (ind - alpha) * (forecast_q - observation_y)
}

/// Mean Brier Score $\mathrm{BS} = \frac{1}{n}\sum_{i=1}^n (p_i - y_i)^2$.
pub fn brier_score(forecasts_p: &[f64], outcomes_y: &[f64]) -> f64 {
    assert_eq!(forecasts_p.len(), outcomes_y.len());
    assert!(!forecasts_p.is_empty());
    forecasts_p
        .iter()
        .zip(outcomes_y.iter())
        .map(|(&p, &y)| (p - y).powi(2))
        .sum::<f64>()
        / (forecasts_p.len() as f64)
}

/// Exact Isotonic **CORP** (*Consistent, Optimal, Reproducible, PAV-calibrated*) Decomposition
/// of the Brier Score (Dimitriadis, Gneiting & Jordan / Arnold et al. — §1.3.5):
/// $$\mathrm{BS}(p, y) = \underbrace{\bigl(\mathrm{BS}(p, y) - \mathrm{BS}(q, y)\bigr)}_{\mathrm{REL}_{\mathrm{CORP}}\text{ (Miscalibration)}} - \underbrace{\frac{1}{n}\sum_{i=1}^n (q_i - \bar{y})^2}_{\mathrm{RES}_{\mathrm{CORP}}\text{ (Resolution)}} + \underbrace{\bar{y}(1 - \bar{y})}_{\mathrm{UNC}_0\text{ (Uncertainty)}}$$
/// where $q_1 \le q_2 \le \dots \le q_n$ are the isotonically recalibrated probabilities via PAV.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpPavDecomposition {
    pub brier_total: f64,
    pub rel_miscalibration: f64,
    pub res_resolution: f64,
    pub unc0_uncertainty: f64,
    /// Exact numerical residual $|\mathrm{BS} - (\mathrm{REL} - \mathrm{RES} + \mathrm{UNC}_0)|$.
    pub identity_error: f64,
    /// Isotonically recalibrated PAV probabilities $q_i$ sorted by original forecast order.
    pub recalibrated_probabilities_q: Vec<f64>,
}

impl CorpPavDecomposition {
    /// Computes the exact CORP decomposition using the Pool-Adjacent-Violators (PAV) algorithm.
    pub fn compute(forecasts_p: &[f64], outcomes_y: &[f64]) -> Self {
        assert_eq!(forecasts_p.len(), outcomes_y.len());
        assert!(!forecasts_p.is_empty());
        let n = forecasts_p.len();

        // Sort pairs by forecast p_i ascending (with deterministic tie-breaking on y_i)
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| {
            forecasts_p[i]
                .total_cmp(&forecasts_p[j])
                .then_with(|| outcomes_y[i].total_cmp(&outcomes_y[j]))
        });

        // Step 1: Group identical forecast values into initial blocks before running PAV.
        // Each block stores: (start_pos_in_sorted, end_pos_in_sorted, sum_y, count)
        let mut initial_blocks: Vec<(usize, usize, f64, usize)> = Vec::new();
        for pos in 0..n {
            let orig = idx[pos];
            let p_val = forecasts_p[orig];
            let y_val = outcomes_y[orig];
            if let Some(last) = initial_blocks.last_mut() {
                let prev_orig = idx[last.1];
                if (forecasts_p[prev_orig] - p_val).abs() < 1e-15 {
                    last.1 = pos;
                    last.2 += y_val;
                    last.3 += 1;
                    continue;
                }
            }
            initial_blocks.push((pos, pos, y_val, 1));
        }

        // Step 2: Pool adjacent violators over the tied initial blocks whenever block average violates monotonicity
        let mut blocks: Vec<(usize, usize, f64, usize)> = Vec::with_capacity(initial_blocks.len());
        for blk in initial_blocks {
            blocks.push(blk);
            while blocks.len() >= 2 {
                let len = blocks.len();
                let mean_prev = blocks[len - 2].2 / (blocks[len - 2].3 as f64);
                let mean_curr = blocks[len - 1].2 / (blocks[len - 1].3 as f64);
                if mean_prev > mean_curr {
                    let removed = blocks.pop().unwrap();
                    let prev = blocks.last_mut().unwrap();
                    prev.1 = removed.1;
                    prev.2 += removed.2;
                    prev.3 += removed.3;
                } else {
                    break;
                }
            }
        }

        let mut q_orig = vec![0.0; n];
        for &(start, end, sum_y, count) in &blocks {
            let q_val = sum_y / (count as f64);
            for &orig_i in &idx[start..=end] {
                q_orig[orig_i] = q_val;
            }
        }

        let n_f = n as f64;
        let y_bar = outcomes_y.iter().sum::<f64>() / n_f;
        let bs = brier_score(forecasts_p, outcomes_y);
        let bs_q = brier_score(&q_orig, outcomes_y);
        let rel = (bs - bs_q).max(0.0);
        let res = q_orig.iter().map(|&q| (q - y_bar).powi(2)).sum::<f64>() / n_f;
        let unc0 = y_bar * (1.0 - y_bar);
        let reconstructed = rel - res + unc0;

        Self {
            brier_total: bs,
            rel_miscalibration: rel,
            res_resolution: res,
            unc0_uncertainty: unc0,
            identity_error: (bs - reconstructed).abs(),
            recalibrated_probabilities_q: q_orig,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corp_pav_decomposition_closes_identity_and_proves_resolution_invariance() {
        let p = [0.05, 0.15, 0.25, 0.35, 0.65, 0.75, 0.85, 0.95];
        let y = [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0];
        let c1 = CorpPavDecomposition::compute(&p, &y);
        assert!(c1.identity_error < 1e-14);
        assert!(c1.rel_miscalibration >= 0.0 && c1.res_resolution > 0.0);

        // Strictly monotone distortion of p changes REL_CORP but leaves RES_CORP invariant!
        let p_distorted: Vec<f64> = p.iter().map(|&x| x * x).collect();
        let c2 = CorpPavDecomposition::compute(&p_distorted, &y);
        assert!(c2.identity_error < 1e-14);
        assert!((c1.res_resolution - c2.res_resolution).abs() < 1e-14);
        assert!(c2.rel_miscalibration > c1.rel_miscalibration);
    }

    #[test]
    fn corp_pav_preserves_isotonicity_for_tied_forecasts() {
        let forecasts = [0.75, 0.0, 0.25, 0.75, 0.0];
        let outcomes = [0.0, 1.0, 1.0, 0.0, 0.0];
        let corp = CorpPavDecomposition::compute(&forecasts, &outcomes);
        assert!(corp.identity_error < 1e-14);
        for &q in &corp.recalibrated_probabilities_q {
            assert!((q - 0.4).abs() < 1e-12, "expected constant 0.4, got {q}");
        }
        assert!(corp.res_resolution < 1e-14);
    }
}
