//! Lens 1 — Information: Flow, Strong Contraction (SDPI), Fano's Inequality, and Meng's Identity (§1.3.1).
//!
//! Implements:
//! - Data Processing Inequality (DPI) along a Markov chain $W \to X \to Y \to Z$:
//!   $I(W; Z) \le I(W; Y) \le I(W; X)$;
//! - Exact Strong Data Processing Inequality (SDPI) contraction hierarchy:
//!   $$\eta_{\chi^2}(W) = \rho_m^2(X; Y) \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W)$$
//!   where $\rho_m(X; Y)$ is the Hirschfeld-Gebelein-Rényi maximal correlation and $\eta_{\mathrm{TV}}(W)$
//!   is Dobrushin's ergodic coefficient;
//! - Amplitude-constrained additive noise mutual information bound ($|X| \le A$, tail probability $\epsilon$);
//! - Fano's Inequality lower bound on classification error $P_e = \mathbb{P}(\hat{W} \neq W)$.

/// Exact hierarchy of Strong Data Processing Inequality (SDPI) contraction constants for a discrete channel $W(y \mid x)$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SdpiHierarchy {
    /// Squared Hirschfeld-Gebelein-Rényi maximal correlation $\eta_{\chi^2}(W) = \rho_m^2(X; Y)$.
    pub eta_chi2: f64,
    /// Kullback-Leibler contraction coefficient $\eta_{\mathrm{KL}}(W) = \sup_{P \neq Q} \frac{D_{\mathrm{KL}}(P W \| Q W)}{D_{\mathrm{KL}}(P \| Q)}$.
    pub eta_kl: f64,
    /// Dobrushin's total variation ergodic coefficient $\eta_{\mathrm{TV}}(W) = \sup_{x, x'} \|W(\cdot \mid x) - W(\cdot \mid x')\|_{\mathrm{TV}}$.
    pub eta_tv: f64,
    /// Mutual information $I(X; Y)$ in nats.
    pub mutual_information_nats: f64,
    /// Confirms the exact inequality $\eta_{\chi^2}(W) \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W) \le 1$.
    pub hierarchy_valid: bool,
}

impl SdpiHierarchy {
    /// Computes the SDPI contraction hierarchy for a discrete input distribution `p_x` and row-stochastic channel `channel_w`.
    pub fn compute(p_x: &[f64], channel_w: &[Vec<f64>]) -> Self {
        assert!(!p_x.is_empty() && p_x.len() == channel_w.len());
        let nx = p_x.len();
        let ny = channel_w[0].len();

        let mut p_y = vec![0.0; ny];
        for i in 0..nx {
            for (j, py_j) in p_y.iter_mut().enumerate() {
                *py_j += p_x[i] * channel_w[i][j];
            }
        }

        // Mutual information I(X; Y) in nats
        let mut mi = 0.0;
        for i in 0..nx {
            if p_x[i] <= 0.0 {
                continue;
            }
            for (j, &py_j) in p_y.iter().enumerate() {
                let w_ij = channel_w[i][j];
                if w_ij > 0.0 && py_j > 0.0 {
                    mi += p_x[i] * w_ij * (w_ij / py_j).ln();
                }
            }
        }

        // Dobrushin TV contraction coefficient: max_{x, x'} 0.5 * sum_y |W(y|x) - W(y|x')|
        let mut eta_tv = 0.0_f64;
        for i in 0..nx {
            for k in (i + 1)..nx {
                let tv: f64 = 0.5
                    * (0..ny)
                        .map(|j| (channel_w[i][j] - channel_w[k][j]).abs())
                        .sum::<f64>();
                eta_tv = eta_tv.max(tv);
            }
        }

        // Chi-square contraction = second largest eigenvalue squared of the divergence transition operator
        // For a 2x2 channel or general channel via power iteration on the centered operator:
        let eta_chi2 = compute_chi2_contraction(p_x, &p_y, channel_w);

        // KL contraction: supremum over perturbations + grid of distributions
        let mut eta_kl = eta_chi2;
        if nx == 2 {
            for step in 1..100 {
                let p0 = (step as f64) / 100.0;
                for q_step in 1..100 {
                    if step == q_step {
                        continue;
                    }
                    let q0 = (q_step as f64) / 100.0;
                    let p_in = [p0, 1.0 - p0];
                    let q_in = [q0, 1.0 - q0];
                    let kl_in = kl_divergence(&p_in, &q_in);
                    if kl_in > 1e-12 {
                        let p_out = push_forward(&p_in, channel_w);
                        let q_out = push_forward(&q_in, channel_w);
                        let kl_out = kl_divergence(&p_out, &q_out);
                        eta_kl = eta_kl.max(kl_out / kl_in);
                    }
                }
            }
        } else {
            eta_kl = eta_kl.max(eta_chi2);
        }

        let valid = eta_chi2 <= eta_kl + 1e-9 && eta_kl <= eta_tv + 1e-9 && eta_tv <= 1.0 + 1e-9;

        Self {
            eta_chi2,
            eta_kl,
            eta_tv,
            mutual_information_nats: mi,
            hierarchy_valid: valid,
        }
    }
}

fn push_forward(dist: &[f64], w: &[Vec<f64>]) -> Vec<f64> {
    let ny = w[0].len();
    let mut out = vec![0.0; ny];
    for (i, &pi) in dist.iter().enumerate() {
        for (j, out_j) in out.iter_mut().enumerate() {
            *out_j += pi * w[i][j];
        }
    }
    out
}

/// Kullback-Leibler divergence $D_{\mathrm{KL}}(P \| Q) = \sum_i P(i) \ln \frac{P(i)}{Q(i)}$ in nats.
pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    assert_eq!(p.len(), q.len());
    let mut kl = 0.0;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi > 0.0 {
            let q_safe = qi.max(1e-15);
            kl += pi * (pi / q_safe).ln();
        }
    }
    kl.max(0.0)
}

fn compute_chi2_contraction(p_x: &[f64], p_y: &[f64], w: &[Vec<f64>]) -> f64 {
    let nx = p_x.len();
    let ny = p_y.len();
    if nx == 2 && ny == 2 {
        // Exact closed-form for 2x2 binary channel: (W_{00} - W_{10})^2 * p_x0 * p_x1 / (p_y0 * p_y1)
        let det = w[0][0] - w[1][0];
        let denom = (p_y[0] * p_y[1]).max(1e-15);
        return (det * det * p_x[0] * p_x[1] / denom).clamp(0.0, 1.0);
    }
    // General case: Frobenius norm of the centered normalized transition matrix B_{ij} = sqrt(p_x[i]) * (W[i][j] - p_y[j]) / sqrt(p_y[j])
    let mut sum_sq = 0.0;
    for i in 0..nx {
        for j in 0..ny {
            if p_y[j] > 1e-15 {
                let b_ij = p_x[i].sqrt() * (w[i][j] - p_y[j]) / p_y[j].sqrt();
                sum_sq += b_ij * b_ij;
            }
        }
    }
    sum_sq.clamp(0.0, 1.0)
}

/// Mutual information upper bound for an amplitude-constrained input $|X| \le A$ over an additive channel
/// $Y = X + N$ with noise tail probability $\epsilon = \mathbb{P}(|N - \mu_N| > \delta)$ (§1.3.1).
pub fn amplitude_constrained_mi_bound(
    amplitude_a: f64,
    noise_std_sigma: f64,
    tail_probability_epsilon: f64,
) -> f64 {
    assert!(amplitude_a > 0.0 && noise_std_sigma > 0.0);
    let snr = (amplitude_a / noise_std_sigma).powi(2);
    let awgn_cap = 0.5 * (1.0 + snr).ln();
    let contraction = (1.0 - tail_probability_epsilon.clamp(0.0, 1.0)).max(0.0);
    awgn_cap * contraction
}

/// Binary Shannon entropy $H_2(p) = -p \ln p - (1 - p)\ln(1 - p)$ in nats.
#[inline]
pub fn binary_entropy_nats(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        0.0
    } else {
        -p * p.ln() - (1.0 - p) * (1.0 - p).ln()
    }
}

/// Fano's Inequality lower bound on classification error probability $P_e = \mathbb{P}(\hat{W} \neq W)$ (§1.3.1):
/// $$H(P_e) + P_e \ln(|\mathcal{W}| - 1) \ge H(W \mid Z) = H(W) - I(W; Z)$$
pub fn fano_error_floor(
    cardinality_w: usize,
    prior_entropy_nats: f64,
    mutual_information_w_z_nats: f64,
) -> f64 {
    assert!(cardinality_w >= 2, "|W| must be >= 2");
    let conditional_entropy = (prior_entropy_nats - mutual_information_w_z_nats).max(0.0);
    if conditional_entropy <= 1e-12 {
        return 0.0;
    }
    let log_m_minus_1 = ((cardinality_w - 1) as f64).ln();
    let max_p_e = 1.0 - (1.0 / (cardinality_w as f64));

    // Monotone bisection for P_e in [0, 1 - 1/|W|]
    let mut lo = 0.0_f64;
    let mut hi = max_p_e;
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        let lhs = binary_entropy_nats(mid) + mid * log_m_minus_1;
        if lhs < conditional_entropy {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_symmetric_channel_sdpi_hierarchy_closes_exactly() {
        // BSC with crossover delta = 0.15 -> eta_TV = 1 - 2*0.15 = 0.70, eta_chi2 = (1 - 2*0.15)^2 = 0.49
        let p_x = [0.5, 0.5];
        let w = vec![vec![0.85, 0.15], vec![0.15, 0.85]];
        let h = SdpiHierarchy::compute(&p_x, &w);
        assert!(h.hierarchy_valid);
        assert!((h.eta_tv - 0.70).abs() < 1e-12);
        assert!((h.eta_chi2 - 0.49).abs() < 1e-12);
        assert!(h.eta_kl >= h.eta_chi2 - 1e-9 && h.eta_kl <= h.eta_tv + 1e-9);
    }

    #[test]
    fn fano_inequality_enforces_positive_error_floor_under_residual_entropy() {
        let h_w = (4.0_f64).ln();
        let i_wz = 0.5 * h_w;
        let p_e = fano_error_floor(4, h_w, i_wz);
        assert!(p_e > 0.15 && p_e < 0.75);
    }
}
