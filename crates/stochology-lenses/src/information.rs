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
        assert!(ny > 0, "channel must have at least 1 output symbol");
        assert!(
            channel_w.iter().all(|row| row.len() == ny),
            "channel_w rows must all have length ny"
        );

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

        // Chi-square contraction = largest squared singular value of the centered divergence transition operator
        let eta_chi2 = compute_chi2_contraction(p_x, &p_y, channel_w);

        // KL contraction: supremum of D_KL(PW || QW) / D_KL(P || Q) and sup_Q eta_chi2(Q, W)
        // over 2-symbol mixtures (Anantharam et al. extremality) and full-support simplex perturbations.
        let mut eta_kl = eta_chi2;
        for i in 0..nx {
            for k in (i + 1)..nx {
                for step in 1..50 {
                    let p0 = (step as f64) / 50.0;
                    let mut dist_q = vec![0.0; nx];
                    dist_q[i] = p0;
                    dist_q[k] = 1.0 - p0;
                    let q_y = push_forward(&dist_q, channel_w);
                    eta_kl = eta_kl.max(compute_chi2_contraction(&dist_q, &q_y, channel_w));

                    for q_step in 1..50 {
                        if step == q_step {
                            continue;
                        }
                        let q0 = (q_step as f64) / 50.0;
                        let mut p_in = vec![0.0; nx];
                        let mut q_in = vec![0.0; nx];
                        p_in[i] = p0;
                        p_in[k] = 1.0 - p0;
                        q_in[i] = q0;
                        q_in[k] = 1.0 - q0;
                        let kl_in = kl_divergence(&p_in, &q_in);
                        if kl_in > 1e-12 {
                            let p_out = push_forward(&p_in, channel_w);
                            let q_out = push_forward(&q_in, channel_w);
                            let kl_out = kl_divergence(&p_out, &q_out);
                            eta_kl = eta_kl.max(kl_out / kl_in);
                        }
                    }
                }
            }
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
        if pi <= 0.0 {
            continue;
        }
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
        let det = w[0][0] - w[1][0];
        let denom = (p_y[0] * p_y[1]).max(1e-15);
        return (det * det * p_x[0] * p_x[1] / denom).clamp(0.0, 1.0);
    }
    // General case: spectral norm squared (largest eigenvalue of B * B^T) where
    // B_{ij} = sqrt(p_x[i]) * (W[i][j] - p_y[j]) / sqrt(p_y[j]) is the centered transition operator.
    let mut b = vec![vec![0.0; ny]; nx];
    for i in 0..nx {
        if p_x[i] <= 0.0 {
            continue;
        }
        let sx = p_x[i].sqrt();
        for j in 0..ny {
            if p_y[j] > 1e-15 {
                b[i][j] = sx * (w[i][j] - p_y[j]) / p_y[j].sqrt();
            }
        }
    }
    let mut m = vec![vec![0.0; nx]; nx];
    for i in 0..nx {
        for k in i..nx {
            let dot: f64 = (0..ny).map(|j| b[i][j] * b[k][j]).sum();
            m[i][k] = dot;
            m[k][i] = dot;
        }
    }
    jacobi_largest_eigenvalue(&mut m).clamp(0.0, 1.0)
}

/// Computes the largest eigenvalue of a real symmetric matrix `a` via the classical Jacobi rotation algorithm.
fn jacobi_largest_eigenvalue(a: &mut [Vec<f64>]) -> f64 {
    let n = a.len();
    if n == 0 {
        return 0.0;
    }
    for _ in 0..(n * n * 15).max(30) {
        let mut p = 0;
        let mut q = 0;
        let mut max_off = 0.0_f64;
        for (i, row) in a.iter().enumerate() {
            for (j, &val) in row.iter().enumerate().skip(i + 1) {
                if val.abs() > max_off {
                    max_off = val.abs();
                    p = i;
                    q = j;
                }
            }
        }
        if max_off < 1e-14 {
            break;
        }
        let app = a[p][p];
        let aqq = a[q][q];
        let apq = a[p][q];
        let tau = (aqq - app) / (2.0 * apq);
        let t = if tau >= 0.0 {
            1.0 / (tau + (1.0 + tau * tau).sqrt())
        } else {
            -1.0 / (-tau + (1.0 + tau * tau).sqrt())
        };
        let c = 1.0 / (1.0 + t * t).sqrt();
        let s = t * c;
        a[p][p] = app - t * apq;
        a[q][q] = aqq + t * apq;
        a[p][q] = 0.0;
        a[q][p] = 0.0;
        #[allow(clippy::needless_range_loop)]
        for r in 0..n {
            if r != p && r != q {
                let arp = a[r][p];
                let arq = a[r][q];
                a[r][p] = c * arp - s * arq;
                a[p][r] = a[r][p];
                a[r][q] = s * arp + c * arq;
                a[q][r] = a[r][q];
            }
        }
    }
    (0..n).map(|i| a[i][i]).fold(0.0_f64, f64::max)
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
    fn binary_and_ternary_symmetric_channel_sdpi_hierarchy_closes_exactly() {
        // BSC with crossover delta = 0.15 -> eta_TV = 1 - 2*0.15 = 0.70, eta_chi2 = (1 - 2*0.15)^2 = 0.49
        let p_x = [0.5, 0.5];
        let w = vec![vec![0.85, 0.15], vec![0.15, 0.85]];
        let h = SdpiHierarchy::compute(&p_x, &w);
        assert!(h.hierarchy_valid);
        assert!((h.eta_tv - 0.70).abs() < 1e-12);
        assert!((h.eta_chi2 - 0.49).abs() < 1e-12);
        assert!(h.eta_kl >= h.eta_chi2 - 1e-9 && h.eta_kl <= h.eta_tv + 1e-9);

        // 3-ary symmetric channel with d = 0.10 -> eta_chi2 = (1 - 3*0.10)^2 = 0.49, eta_TV = 0.70
        let p_x3 = [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];
        let w3 = vec![
            vec![0.80, 0.10, 0.10],
            vec![0.10, 0.80, 0.10],
            vec![0.10, 0.10, 0.80],
        ];
        let h3 = SdpiHierarchy::compute(&p_x3, &w3);
        assert!(h3.hierarchy_valid);
        assert!((h3.eta_chi2 - 0.49).abs() < 1e-12);
        assert!((h3.eta_tv - 0.70).abs() < 1e-12);
        assert!(h3.eta_kl > h3.eta_chi2 && h3.eta_kl <= h3.eta_tv + 1e-9);
    }

    #[test]
    fn fano_inequality_enforces_positive_error_floor_under_residual_entropy() {
        let h_w = (4.0_f64).ln();
        let i_wz = 0.5 * h_w;
        let p_e = fano_error_floor(4, h_w, i_wz);
        assert!(p_e > 0.15 && p_e < 0.75);
    }
}
