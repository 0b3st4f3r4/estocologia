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

/// Three-step Markov chain Data Processing Inequality (DPI) diagnostic along $W \to X \to Y \to Z$ (§1.3.1):
/// $$I(W; Z) \le I(W; Y) \le I(W; X)$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DpiMarkovChain {
    /// Mutual information $I(W; X)$ in nats.
    pub i_w_x_nats: f64,
    /// Mutual information $I(W; Y)$ in nats.
    pub i_w_y_nats: f64,
    /// Mutual information $I(W; Z)$ in nats.
    pub i_w_z_nats: f64,
    /// Confirms the Data Processing Inequality $I(W; Z) \le I(W; Y) \le I(W; X)$.
    pub dpi_satisfied: bool,
}

impl DpiMarkovChain {
    /// Evaluates the Data Processing Inequality along a discrete 3-step Markov chain $W \to X \to Y \to Z$
    /// defined by prior `p_w` and row-stochastic transition matrices `w_to_x`, `x_to_y`, and `y_to_z`.
    pub fn compute(
        p_w: &[f64],
        w_to_x: &[Vec<f64>],
        x_to_y: &[Vec<f64>],
        y_to_z: &[Vec<f64>],
    ) -> Self {
        let w_to_y = compose_channels(w_to_x, x_to_y);
        let w_to_z = compose_channels(&w_to_y, y_to_z);
        let i_w_x = channel_mutual_information_nats(p_w, w_to_x);
        let i_w_y = channel_mutual_information_nats(p_w, &w_to_y);
        let i_w_z = channel_mutual_information_nats(p_w, &w_to_z);
        let dpi_ok = i_w_z <= i_w_y + 1e-12 && i_w_y <= i_w_x + 1e-12;
        Self {
            i_w_x_nats: i_w_x,
            i_w_y_nats: i_w_y,
            i_w_z_nats: i_w_z,
            dpi_satisfied: dpi_ok,
        }
    }
}

fn compose_channels(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    assert!(!a.is_empty() && !b.is_empty());
    let rows_a = a.len();
    let mid = a[0].len();
    assert_eq!(
        mid,
        b.len(),
        "channel dimensions must match for composition"
    );
    let cols_b = b[0].len();
    let mut out = vec![vec![0.0; cols_b]; rows_a];
    for (i, row_a) in a.iter().enumerate() {
        for (k, &aik) in row_a.iter().enumerate() {
            for (j, &bkj) in b[k].iter().enumerate() {
                out[i][j] += aik * bkj;
            }
        }
    }
    out
}

fn channel_mutual_information_nats(p_in: &[f64], channel: &[Vec<f64>]) -> f64 {
    assert!(!p_in.is_empty() && p_in.len() == channel.len());
    let nx = p_in.len();
    let ny = channel[0].len();
    let mut p_out = vec![0.0; ny];
    for i in 0..nx {
        for (j, py_j) in p_out.iter_mut().enumerate() {
            *py_j += p_in[i] * channel[i][j];
        }
    }
    let mut mi = 0.0;
    for i in 0..nx {
        if p_in[i] <= 0.0 {
            continue;
        }
        for (j, &py_j) in p_out.iter().enumerate() {
            let w_ij = channel[i][j];
            if w_ij > 0.0 && py_j > 0.0 {
                mi += p_in[i] * w_ij * (w_ij / py_j).ln();
            }
        }
    }
    mi.max(0.0)
}

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

        // KL contraction: by Anantharam et al. (2013), eta_KL(W) = sup_Q eta_chi2(Q, W) is achieved
        // on 2-symbol input distributions Q = p0 * delta_i + (1 - p0) * delta_k.
        // For each pair i < k, we maximize the exact 2-symbol chi-square contraction closed form
        // g_{ik}(p0) via golden-section search on [1e-9, 1 - 1e-9], and also verify binary mixture KL ratios.
        let inv_phi = 0.5 * (5.0_f64.sqrt() - 1.0);
        let mut eta_kl = eta_chi2;
        for i in 0..nx {
            for k in (i + 1)..nx {
                let eval_chi2_pair = |p0: f64| -> f64 {
                    let p1 = 1.0 - p0;
                    let mut sum = 0.0;
                    for (&wij, &wkj) in channel_w[i].iter().zip(channel_w[k].iter()) {
                        let denom = p0 * wij + p1 * wkj;
                        if denom > 1e-15 {
                            let diff = wij - wkj;
                            sum += (diff * diff) / denom;
                        }
                    }
                    (p0 * p1 * sum).clamp(0.0, 1.0)
                };

                let mut a = 1e-9_f64;
                let mut b = 1.0 - 1e-9_f64;
                let mut c = b - inv_phi * (b - a);
                let mut d = a + inv_phi * (b - a);
                let mut fc = eval_chi2_pair(c);
                let mut fd = eval_chi2_pair(d);
                for _ in 0..64 {
                    if fc >= fd {
                        b = d;
                        d = c;
                        fd = fc;
                        c = b - inv_phi * (b - a);
                        fc = eval_chi2_pair(c);
                    } else {
                        a = c;
                        c = d;
                        fc = fd;
                        d = a + inv_phi * (b - a);
                        fd = eval_chi2_pair(d);
                    }
                }
                eta_kl = eta_kl.max(eval_chi2_pair(0.5 * (a + b)));

                for step in 1..50 {
                    let p0 = (step as f64) / 50.0;
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
/// Composes the Smith (1971) / McKellips (2004) peak-amplitude AWGN capacity upper bound with SDPI erasure/outage contraction.
pub fn amplitude_constrained_mi_bound(
    amplitude_a: f64,
    noise_std_sigma: f64,
    tail_probability_epsilon: f64,
) -> f64 {
    assert!(amplitude_a > 0.0 && noise_std_sigma > 0.0);
    let ratio = amplitude_a / noise_std_sigma;
    let snr = ratio * ratio;
    let variance_cap = 0.5 * (1.0 + snr).ln();
    let mckellips_cap =
        (1.0 + ratio * (2.0 / (std::f64::consts::PI * std::f64::consts::E)).sqrt()).ln();
    let awgn_peak_cap = variance_cap.min(mckellips_cap);
    let contraction = (1.0 - tail_probability_epsilon.clamp(0.0, 1.0)).max(0.0);
    awgn_peak_cap * contraction
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

/// Exact Blahut-Arimoto alternating minimization algorithm (Blahut 1972; Arimoto 1972)
/// computing the Shannon channel capacity $C(W) = \max_{P_X} I(X; Y)$ (in nats) and the capacity-achieving
/// input distribution $P_X^\star$ for a discrete memoryless channel `channel_w`.
pub fn blahut_arimoto_capacity(
    channel_w: &[Vec<f64>],
    max_iters: usize,
    tol: f64,
) -> (f64, Vec<f64>) {
    assert!(!channel_w.is_empty() && !channel_w[0].is_empty());
    let nx = channel_w.len();
    let ny = channel_w[0].len();
    let mut p_x = vec![1.0 / (nx as f64); nx];
    let mut c_vals = vec![0.0; nx];

    for _ in 0..max_iters.max(1) {
        let p_y = push_forward(&p_x, channel_w);
        for i in 0..nx {
            let mut d_kl = 0.0;
            for j in 0..ny {
                let w_ij = channel_w[i][j];
                if w_ij > 1e-15 && p_y[j] > 1e-15 {
                    d_kl += w_ij * (w_ij / p_y[j]).ln();
                }
            }
            c_vals[i] = d_kl.exp();
        }
        let z: f64 = p_x
            .iter()
            .zip(c_vals.iter())
            .map(|(&pi, &ci)| pi * ci)
            .sum::<f64>()
            .max(1e-15);
        let max_c = c_vals.iter().copied().fold(0.0_f64, f64::max).max(1e-15);
        let gap = (max_c / z).ln();
        for (pi, &ci) in p_x.iter_mut().zip(c_vals.iter()) {
            *pi = (*pi * ci) / z;
        }
        if gap.abs() <= tol.max(1e-14) {
            break;
        }
    }
    let cap = channel_mutual_information_nats(&p_x, channel_w);
    (cap, p_x)
}

/// Estimates an empirical discrete memoryless channel $W(y_{\text{bin}} \mid x_{\text{bin}})$ and input marginal $P_X$
/// from paired continuous observations `(x, y)` via empirical quantile discretization (`num_bins`) and Laplace smoothing (`laplace_alpha`).
pub fn estimate_empirical_channel(
    x: &[f64],
    y: &[f64],
    num_bins: usize,
    laplace_alpha: f64,
) -> (Vec<f64>, Vec<Vec<f64>>) {
    assert_eq!(x.len(), y.len());
    assert!(!x.is_empty() && num_bins >= 2);
    let b = num_bins;
    let alpha = laplace_alpha.max(1e-9);
    let mut sx = x.to_vec();
    let mut sy = y.to_vec();
    sx.sort_by(|a, b_val| a.total_cmp(b_val));
    sy.sort_by(|a, b_val| a.total_cmp(b_val));

    let bin_of = |val: f64, sorted: &[f64]| -> usize {
        let mut cut = 0usize;
        for k in 1..b {
            let q_k = stochology_core::quantile_sorted(sorted, (k as f64) / (b as f64));
            if val > q_k {
                cut = k;
            }
        }
        cut.min(b - 1)
    };

    let mut counts = vec![vec![alpha; b]; b];
    let mut row_totals = vec![alpha * (b as f64); b];
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let bi = bin_of(xi, &sx);
        let bj = bin_of(yi, &sy);
        counts[bi][bj] += 1.0;
        row_totals[bi] += 1.0;
    }

    let grand_total: f64 = row_totals.iter().sum();
    let p_x: Vec<f64> = row_totals.iter().map(|&r| r / grand_total).collect();
    let w: Vec<Vec<f64>> = counts
        .into_iter()
        .zip(row_totals.iter())
        .map(|(row, &tot)| row.into_iter().map(|c| c / tot).collect())
        .collect();
    (p_x, w)
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

    #[test]
    fn mckellips_amplitude_bound_and_asymmetric_z_channel_golden_section() {
        // Peak-amplitude McKellips bound is strictly tighter than unconstrained variance bound 0.5 * ln(1 + A^2/sigma^2)
        let a = 2.0;
        let sigma = 1.0;
        let naive_var_cap = 0.5 * (1.0_f64 + (a / sigma) * (a / sigma)).ln();
        let mck_bound = amplitude_constrained_mi_bound(a, sigma, 0.0);
        assert!(mck_bound < naive_var_cap && mck_bound > 0.0);

        // Asymmetric binary channel: W(0|0) = 0.9, W(0|1) = 0.2 (so a = 0.9, b = 0.2)
        // Unique interior maximizer p0* = 4/7 (not on 1/50 grid), with exact
        // sup_{p0} g(p0) = (sqrt(a*(1-b)) - sqrt(b*(1-a)))^2 = (sqrt(0.72) - sqrt(0.02))^2 = 0.50
        let w_asym = vec![vec![0.9, 0.1], vec![0.2, 0.8]];
        let h_asym = SdpiHierarchy::compute(&[0.5, 0.5], &w_asym);
        assert!((h_asym.eta_kl - 0.50).abs() < 1e-12);
        assert!(h_asym.hierarchy_valid);
    }

    #[test]
    fn dpi_three_step_markov_chain_monotonically_contracts_mutual_information() {
        let p_w = [0.5, 0.5];
        let w_to_x = vec![vec![0.90, 0.10], vec![0.10, 0.90]];
        let x_to_y = vec![vec![0.85, 0.15], vec![0.15, 0.85]];
        let y_to_z = vec![vec![0.80, 0.20], vec![0.20, 0.80]];
        let dpi = DpiMarkovChain::compute(&p_w, &w_to_x, &x_to_y, &y_to_z);
        assert!(dpi.dpi_satisfied);
        assert!(dpi.i_w_x_nats > dpi.i_w_y_nats && dpi.i_w_y_nats > dpi.i_w_z_nats);
        assert!(dpi.i_w_z_nats > 0.0);

        let (cap_bsc, p_opt) = blahut_arimoto_capacity(&w_to_x, 200, 1e-12);
        let expected_cap = std::f64::consts::LN_2 - binary_entropy_nats(0.10);
        assert!((cap_bsc - expected_cap).abs() < 1e-10);
        assert!((p_opt[0] - 0.5).abs() < 1e-9);
    }
}
