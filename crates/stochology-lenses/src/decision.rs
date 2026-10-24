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

    /// Computes Richardson's (2000) Relative Economic Value $V(p^*) \in (-\infty, 1]$ of a probabilistic
    /// forecast series `forecasts_p` against binary outcomes `outcomes_y`:
    /// $$V(p^*) = \frac{E_{\text{climatology}} - E_{\text{forecast}}}{E_{\text{climatology}} - E_{\text{perfect}}}$$
    /// where $V = 1$ is a perfect oracle and $V = 0$ is naive climatological base-rate action.
    pub fn relative_economic_value(&self, forecasts_p: &[f64], outcomes_y: &[f64]) -> f64 {
        assert_eq!(forecasts_p.len(), outcomes_y.len());
        assert!(!forecasts_p.is_empty());
        let n = forecasts_p.len() as f64;
        let pi_bar = outcomes_y.iter().sum::<f64>() / n;
        let residual_loss = (1.0 - self.efficacy_eta) * self.event_loss_l;

        // Expected expense per unit under perfect oracle: protect iff y_i == 1 (when C < eta * L)
        let e_perfect = pi_bar * (self.protection_cost_c + residual_loss).min(self.event_loss_l);
        // Expected expense per unit under climatology: protect always if pi_bar > p* else never
        let e_clim = if pi_bar > self.critical_threshold_p_star {
            self.protection_cost_c + pi_bar * residual_loss
        } else {
            pi_bar * self.event_loss_l
        };

        let mut e_forecast = 0.0;
        for (&p, &y) in forecasts_p.iter().zip(outcomes_y.iter()) {
            if p > self.critical_threshold_p_star {
                e_forecast += self.protection_cost_c + y * residual_loss;
            } else {
                e_forecast += y * self.event_loss_l;
            }
        }
        e_forecast /= n;

        let denom = e_clim - e_perfect;
        if denom.abs() < 1e-12 {
            0.0
        } else {
            (e_clim - e_forecast) / denom
        }
    }
}

/// Strictly proper Logarithmic (Ignorance) Score with Cromwell floor $\varepsilon_{\text{floor}} \in (0, 0.5)$:
/// $$\mathrm{LogS}(p, y) = -\frac{1}{n}\sum_{i=1}^n \Bigl[y_i \ln \tilde{p}_i + (1 - y_i)\ln(1 - \tilde{p}_i)\Bigr], \qquad \tilde{p}_i = \mathrm{clamp}(p_i, \varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}})$$
pub fn logarithmic_score(forecasts_p: &[f64], outcomes_y: &[f64], epsilon_floor: f64) -> f64 {
    assert_eq!(forecasts_p.len(), outcomes_y.len());
    assert!(!forecasts_p.is_empty());
    let n = forecasts_p.len() as f64;
    forecasts_p
        .iter()
        .zip(outcomes_y.iter())
        .map(|(&p, &y)| {
            let pc = stochology_core::apply_cromwell_floor(p, epsilon_floor);
            -(y * pc.ln() + (1.0 - y) * (1.0 - pc).ln())
        })
        .sum::<f64>()
        / n
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

/// Exact zero-cost benchmark proving that *Calibrated Decisions Are Not Calibrated Probabilities*
/// (anchored to a known ground-truth fair coin $p_0 = 0.50$ with `seed = 2026`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExactCoinCalibrationBenchmark {
    pub seed: u64,
    pub sample_size_n: usize,
    pub true_coin_probability: f64,
    pub preregistration_sha256: String,
    pub empirical_outcome_mean: f64,
    pub calibrated_corp: CorpPavDecomposition,
    pub overconfident_corp: CorpPavDecomposition,
    /// Fraction of trials where both forecasters take the exact same binary action under $p^*_A = 0.30$.
    pub decision_agreement_rate_at_p_star_a: f64,
    /// Excess economic regret per unit suffered by the overconfident forecaster when the cost-loss threshold shifts to $p^*_B = 0.70$.
    pub shifted_threshold_excess_regret_per_unit: f64,
    /// True if: (1) decision agreement at $p^*_A$ is 100%, (2) overconfident $\mathrm{REL}_{\mathrm{CORP}} > 0.08 \gg \text{calibrated }\mathrm{REL}_{\mathrm{CORP}}$,
    /// (3) $\mathrm{RES}_{\mathrm{CORP}}$ is invariant under strictly monotone distortion, and (4) shifted threshold excess regret $> 0$.
    pub separates_decision_from_probability_calibration: bool,
}

impl ExactCoinCalibrationBenchmark {
    /// Executes the exact fair-coin ($p_0 = 0.50$) calibration vs. decision audit (`seed = 2026`).
    pub fn evaluate(seed: u64, sample_size_n: usize) -> Self {
        let n = sample_size_n.max(100);
        let payload = format!(
            "exact_coin_calibration_v1|seed={seed}|p_true=0.50000|n={n}|p_star_a=0.30000|p_star_b=0.70000"
        );
        let prereg_sha = stochology_core::sha256_hex(payload.as_bytes());

        let mut rng = stochology_core::Rng::new(seed);
        let mut p_cal = Vec::with_capacity(n);
        let mut p_over = Vec::with_capacity(n);
        let mut y_out = Vec::with_capacity(n);

        for i in 0..n {
            // Centered symmetric signal around fair-coin base rate 0.50
            let delta = if i.is_multiple_of(2) { -0.05 } else { 0.05 };
            let p_true_i = 0.50 + delta;
            let y = if rng.bernoulli(p_true_i) { 1.0 } else { 0.0 };
            p_cal.push(p_true_i);
            // Strictly monotone distortion perched in [0.80, 0.90]: identical ranking and identical action at p*_A = 0.30
            p_over.push(0.85 + delta);
            y_out.push(y);
        }

        let rule_a = CostLossRule::new(30.0, 100.0, 1.0); // p*_A = 0.30
        let rule_b = CostLossRule::new(70.0, 100.0, 1.0); // p*_B = 0.70

        let mut agree_a = 0usize;
        let mut loss_cal_b = 0.0;
        let mut loss_over_b = 0.0;

        for i in 0..n {
            let act_cal_a = rule_a.recommend_action(p_cal[i]);
            let act_over_a = rule_a.recommend_action(p_over[i]);
            if act_cal_a == act_over_a {
                agree_a += 1;
            }

            let act_cal_b = rule_b.recommend_action(p_cal[i]);
            let act_over_b = rule_b.recommend_action(p_over[i]);
            loss_cal_b += if act_cal_b == "protect" {
                rule_b.protection_cost_c
            } else {
                y_out[i] * rule_b.event_loss_l
            };
            loss_over_b += if act_over_b == "protect" {
                rule_b.protection_cost_c
            } else {
                y_out[i] * rule_b.event_loss_l
            };
        }

        let n_f = n as f64;
        let agree_rate_a = (agree_a as f64) / n_f;
        let excess_regret_b = (loss_over_b - loss_cal_b) / n_f;
        let y_bar = y_out.iter().sum::<f64>() / n_f;

        let cal_corp = CorpPavDecomposition::compute(&p_cal, &y_out);
        let over_corp = CorpPavDecomposition::compute(&p_over, &y_out);

        let separates = (agree_rate_a - 1.0).abs() < 1e-12
            && over_corp.rel_miscalibration > 0.08
            && cal_corp.rel_miscalibration < 0.01
            && (cal_corp.res_resolution - over_corp.res_resolution).abs() < 1e-12
            && excess_regret_b > 10.0;

        Self {
            seed,
            sample_size_n: n,
            true_coin_probability: 0.50,
            preregistration_sha256: prereg_sha,
            empirical_outcome_mean: y_bar,
            calibrated_corp: cal_corp,
            overconfident_corp: over_corp,
            decision_agreement_rate_at_p_star_a: agree_rate_a,
            shifted_threshold_excess_regret_per_unit: excess_regret_b,
            separates_decision_from_probability_calibration: separates,
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

    #[test]
    fn exact_coin_benchmark_proves_calibrated_decisions_are_not_calibrated_probabilities() {
        let bench = ExactCoinCalibrationBenchmark::evaluate(2026, 1_000);
        assert!(bench.separates_decision_from_probability_calibration);
        assert_eq!(bench.decision_agreement_rate_at_p_star_a, 1.0);
        assert!(bench.overconfident_corp.rel_miscalibration > 0.10);
        assert!(bench.calibrated_corp.rel_miscalibration < 0.005);
        assert!(
            (bench.calibrated_corp.res_resolution - bench.overconfident_corp.res_resolution).abs()
                < 1e-12
        );
        assert!(bench.shifted_threshold_excess_regret_per_unit > 15.0);

        let rule_b = CostLossRule::new(70.0, 100.0, 1.0);
        let rev_cal = rule_b.relative_economic_value(&[0.2, 0.8, 0.1, 0.9], &[0.0, 1.0, 0.0, 1.0]);
        assert!((rev_cal - 1.0).abs() < 1e-12);
        let logs = logarithmic_score(&[0.1, 0.9], &[0.0, 1.0], 0.001);
        assert!(logs > 0.0 && logs < 0.2);
    }
}
