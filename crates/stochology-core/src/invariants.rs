//! The Five Boundary Invariants ($\Phi_1$ to $\Phi_5$), Design Effect for Quantile Exceedance
//! (`ExceedanceDesignEffect`), Moulton-Campbell Correction for Unequal Clusters, Operational
//! Weighting Gap, and Chouldechova's Algebraic Impossibility Audit (§2.2).

use crate::rng::quantile;

/// Design Effect for Quantile Exceedance and Moulton-Campbell Correction for Unequal Clusters
/// (Invariant $\Phi_2$ — §2.2):
/// $$\tilde{m} = \frac{\sum_{j=1}^C m_j^2}{\sum_{j=1}^C m_j} = \bar{m}\left(1 + \mathrm{CV}^2(m_j)\right)$$
/// $$\mathrm{DEFF}_{\text{exceedance}}(p) = 1 + (\tilde{m} - 1)\rho_I(p), \qquad n_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}_{\text{exceedance}}(p)}$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExceedanceDesignEffect {
    /// Nominal transactional row count $n = \sum_{j=1}^C m_j$.
    pub n_nominal: usize,
    /// Number of independent clusters $C$ (days, hospitals, stores, batches).
    pub num_clusters_c: usize,
    /// Simple arithmetic mean cluster size $\bar{m} = n / C$.
    pub m_bar: f64,
    /// Coefficient of variation of cluster sizes $\mathrm{CV}(m_j)$.
    pub cv_clusters: f64,
    /// Moulton-Campbell weighted cluster size $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2(m_j))$.
    pub m_tilde_moulton: f64,
    /// Quantile level $p \in (0, 1)$ at which exceedance $I_{ij}(p) = \mathbb{I}(Y_{ij} > q_p)$ is audited.
    pub quantile_level_p: f64,
    /// Intraclass correlation of binary threshold exceedance $\rho_I(p)$.
    pub rho_exceedance_p: f64,
    /// Exceedance Design Effect $\mathrm{DEFF}_{\text{exceedance}}(p) = 1 + (\tilde{m} - 1)\rho_I(p)$.
    pub deff_exceedance: f64,
    /// Effective sample size at quantile $p$: $n_{\mathrm{eff}}(p) = n / \mathrm{DEFF}_{\text{exceedance}}(p)$.
    pub n_effective_p: f64,
    /// Naive Kish effective sample size ignoring cluster size dispersion ($\bar{m}$ instead of $\tilde{m}$).
    pub n_effective_naive_kish: f64,
}

impl ExceedanceDesignEffect {
    /// Computes the Moulton-Campbell design effect from cluster sizes $m_j$ and exceedance ICC $\rho_I(p)$.
    pub fn from_cluster_sizes(
        cluster_sizes: &[usize],
        quantile_level_p: f64,
        rho_exceedance_p: f64,
    ) -> Self {
        assert!(!cluster_sizes.is_empty(), "requires at least 1 cluster");
        let c = cluster_sizes.len();
        let n_nom: usize = cluster_sizes
            .iter()
            .copied()
            .fold(0usize, usize::saturating_add);
        assert!(n_nom > 0);

        let sum_m: f64 = cluster_sizes.iter().map(|&m| m as f64).sum();
        let sum_m2: f64 = cluster_sizes.iter().map(|&m| (m as f64) * (m as f64)).sum();
        let m_bar = sum_m / (c as f64);
        let m_tilde = sum_m2 / sum_m;
        let cv2 = ((m_tilde / m_bar) - 1.0).max(0.0);
        let cv = cv2.sqrt();

        let rho_clamped = rho_exceedance_p.clamp(0.0, 1.0);
        let deff = 1.0 + (m_tilde - 1.0).max(0.0) * rho_clamped;
        let deff_naive = 1.0 + (m_bar - 1.0).max(0.0) * rho_clamped;

        Self {
            n_nominal: n_nom,
            num_clusters_c: c,
            m_bar,
            cv_clusters: cv,
            m_tilde_moulton: m_tilde,
            quantile_level_p,
            rho_exceedance_p: rho_clamped,
            deff_exceedance: deff,
            n_effective_p: sum_m / deff,
            n_effective_naive_kish: sum_m / deff_naive,
        }
    }

    /// Computes $\tilde{m}$, ANOVA exceedance ICC $\rho_I(p)$, and $n_{\mathrm{eff}}(p)$ directly from clustered observations.
    pub fn from_clustered_observations(clusters: &[Vec<f64>], quantile_level_p: f64) -> Self {
        assert!(clusters.len() >= 2, "requires at least 2 clusters");
        let mut all = Vec::new();
        let mut sizes = Vec::with_capacity(clusters.len());
        for cl in clusters {
            if !cl.is_empty() {
                sizes.push(cl.len());
                all.extend_from_slice(cl);
            }
        }
        let q_p = quantile(&all, quantile_level_p);

        // Binary exceedance indicator I_{ij}(p) = 1 if Y_{ij} > q_p else 0
        let n_total = all.len() as f64;
        let k_clusters = sizes.len() as f64;
        let mut grand_sum = 0.0;
        let mut cluster_rates = Vec::with_capacity(sizes.len());
        for cl in clusters.iter().filter(|c| !c.is_empty()) {
            let s: f64 = cl.iter().map(|&y| if y > q_p { 1.0 } else { 0.0 }).sum();
            grand_sum += s;
            cluster_rates.push((cl.len() as f64, s / (cl.len() as f64)));
        }
        let p_bar = grand_sum / n_total;

        // One-way random-effects ANOVA estimator for binary intraclass correlation rho_I(p)
        let mut ssb = 0.0;
        let mut ssw = 0.0;
        for &(m_j, p_j) in &cluster_rates {
            ssb += m_j * (p_j - p_bar).powi(2);
            ssw += m_j * p_j * (1.0 - p_j);
        }
        let msb = ssb / (k_clusters - 1.0).max(1.0);
        let msw = ssw / (n_total - k_clusters).max(1.0);
        let sum_m2: f64 = sizes.iter().map(|&m| (m as f64).powi(2)).sum();
        let m0 = (n_total - (sum_m2 / n_total)) / (k_clusters - 1.0).max(1.0);
        let rho_hat = if msb > msw && m0 > 0.0 {
            (msb - msw) / (msb + (m0 - 1.0) * msw)
        } else {
            0.0
        };

        Self::from_cluster_sizes(&sizes, quantile_level_p, rho_hat)
    }

    /// Protocol calibration benchmark #1 (§2.2):
    /// Nominal $n = 25,028$ transactional rows across $C = 420$ unequal clusters ($\bar{m} \approx 59.59$,
    /// $\mathrm{CV} \approx 0.999$, $\tilde{m} \approx 119.05$) with $\rho_I(0.90) \approx 0.155$ collapses to
    /// $n_{\mathrm{eff}}(0.90) \approx 1,297$ independent units ($\mathrm{DEFF}_{\text{exceedance}} \approx 19.30$).
    pub fn benchmark_calibration_25028() -> Self {
        // 78 large clusters of size 182 (14,196) + 341 regular clusters of size 31 (10,571) + 1 cluster of 261 = 25,028
        let mut sizes = Vec::with_capacity(420);
        sizes.extend(std::iter::repeat_n(182, 78));
        sizes.extend(std::iter::repeat_n(31, 341));
        sizes.push(261);
        Self::from_cluster_sizes(&sizes, 0.90, 0.155)
    }

    /// Protocol calibration benchmark #2 (§2.2):
    /// Nominal $n = 500$ observations in $C = 10$ clusters of size $m = 50$ with $\rho_I \approx 0.098$
    /// collapses to $n_{\mathrm{eff}} \approx 86$.
    pub fn benchmark_pseudoreplication_500_to_86() -> Self {
        let sizes = vec![50; 10];
        Self::from_cluster_sizes(&sizes, 0.90, 0.098_2)
    }

    /// Profiles Moulton-Campbell exceedance design effect $\mathrm{DEFF}_{\text{exceedance}}(p)$ and $n_{\mathrm{eff}}(p)$
    /// across a grid of quantile levels $p \in \text{quantile\_levels}$ (e.g. $[0.50, 0.90, 0.95, 0.99]$).
    pub fn profile_across_quantiles(clusters: &[Vec<f64>], quantile_levels: &[f64]) -> Vec<Self> {
        quantile_levels
            .iter()
            .map(|&p| Self::from_clustered_observations(clusters, p))
            .collect()
    }
}

/// Operational Weighting Gap and Variance Inflation Factor (Invariant $\Phi_2$ — §2.2):
/// Separates cluster average $\bar{\mu}_C = \frac{1}{C}\sum_{j=1}^C \hat{\mu}_j$ from transactional
/// average $\bar{\mu}_{\text{op}} = \frac{\sum_{j=1}^C m_j \hat{\mu}_j}{\sum_{j=1}^C m_j}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperationalWeightingGap {
    pub unweighted_cluster_mean: f64,
    pub operational_transactional_mean: f64,
    /// Exact covariance gap $\bar{\mu}_{\text{op}} - \bar{\mu}_C = \frac{1}{\bar{m}}\mathrm{Cov}(m_j, \mu_j)$.
    pub weighting_gap: f64,
    /// Variance inflation factor $1 + \mathrm{CV}^2(m_j)$ paid by the operational estimator.
    pub variance_inflation_factor: f64,
}

impl OperationalWeightingGap {
    pub fn compute(cluster_sizes: &[usize], cluster_means: &[f64]) -> Self {
        assert_eq!(cluster_sizes.len(), cluster_means.len());
        assert!(!cluster_sizes.is_empty());
        let c = cluster_sizes.len() as f64;
        let mu_c = cluster_means.iter().sum::<f64>() / c;
        let sum_m: f64 = cluster_sizes.iter().map(|&m| m as f64).sum();
        let sum_m2: f64 = cluster_sizes.iter().map(|&m| (m as f64).powi(2)).sum();
        let m_bar = sum_m / c;
        let m_tilde = sum_m2 / sum_m;
        let vif = m_tilde / m_bar;

        let weighted_sum: f64 = cluster_sizes
            .iter()
            .zip(cluster_means.iter())
            .map(|(&m, &mu)| (m as f64) * mu)
            .sum();
        let mu_op = weighted_sum / sum_m;

        Self {
            unweighted_cluster_mean: mu_c,
            operational_transactional_mean: mu_op,
            weighting_gap: mu_op - mu_c,
            variance_inflation_factor: vif,
        }
    }

    /// Protocol benchmark (§2.2): ignoring intra-cluster correlation and cluster size variance
    /// understates sampling variance by a factor of $69\times$.
    pub fn variance_amplification_benchmark_69x() -> f64 {
        let m_tilde = 137.0;
        let rho = 0.50;
        1.0 + (m_tilde - 1.0) * rho
    }
}

/// Invariant $\Phi_5$: Question Resolution vs. Independent Unit compatibility check (§2.2).
///
/// Forbids answering a high-resolution operational decision (daily, shift, hourly, 5-min) with
/// coarse annual, monthly, or quarterly aggregates ($R_0$).
pub fn verify_resolution_phi5(independent_unit: &str, altered_action: &str) -> bool {
    let unit = independent_unit.trim().to_ascii_lowercase();
    let action = altered_action.trim().to_ascii_lowercase();
    if unit.is_empty() || action.is_empty() {
        return false;
    }

    let fine_unit = unit.contains("minute")
        || unit.contains("5-min")
        || unit.contains("hour")
        || unit.contains("shift")
        || unit.contains("day")
        || unit.contains("daily");

    let coarse_horizon = action.contains("annual")
        || action.contains("yearly")
        || action.contains("monthly")
        || action.contains("quarterly")
        || action.contains("decadal")
        || action.contains("multi-year");

    let coarse_aggregation = action.contains("aggregate")
        || action.contains("average")
        || action.contains("mean")
        || action.contains("total")
        || action.contains("rollup")
        || action.contains("sum")
        || action.contains("single");

    let coarse_aggregate_action =
        (coarse_horizon && coarse_aggregation) || action.contains("across all shifts");

    !(fine_unit && coarse_aggregate_action)
}

/// Audit of Chouldechova's Algebraic Impossibility Trilemma (Invariant $\Phi_4$ — §2.2):
/// $$\mathrm{FPR}_g = \frac{p_g}{1 - p_g} \cdot \frac{1 - \mathrm{PPV}_g}{\mathrm{PPV}_g} \cdot (1 - \mathrm{FNR}_g)$$
/// Whenever base rates differ ($p_A \neq p_B$), no imperfect binary classifier ($\mathrm{PPV} < 1$, $\mathrm{FNR} < 1$)
/// can simultaneously equalize Predictive Parity ($\mathrm{PPV}$), False Negative Rate ($\mathrm{FNR}$),
/// and False Positive Rate ($\mathrm{FPR}$) across groups $A$ and $B$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChouldechovaAudit {
    pub prevalence_a: f64,
    pub prevalence_b: f64,
    pub ppv_a: f64,
    pub ppv_b: f64,
    pub fnr_a: f64,
    pub fnr_b: f64,
    pub fpr_a: f64,
    pub fpr_b: f64,
    /// Exact algebraic verification error of Chouldechova's identity across both groups.
    pub identity_max_error: f64,
    /// Required ratio $\mathrm{FPR}_A / \mathrm{FPR}_B = \frac{p_A / (1 - p_A)}{p_B / (1 - p_B)}$ if $\mathrm{PPV}$ and $\mathrm{FNR}$ were forced equal.
    pub fpr_ratio_under_equal_ppv_and_fnr: f64,
    /// Confirms that simultaneous equality of $(\mathrm{PPV}, \mathrm{FNR}, \mathrm{FPR})$ is algebraically impossible.
    pub simultaneous_equality_impossible: bool,
}

impl ChouldechovaAudit {
    /// Evaluates Chouldechova's exact identity for group prevalence $p \in (0, 1)$, $\mathrm{PPV} \in (0, 1)$, and $\mathrm{FNR} \in [0, 1)$.
    #[inline]
    pub fn chouldechova_fpr(prevalence_p: f64, ppv: f64, fnr: f64) -> f64 {
        if !prevalence_p.is_finite()
            || !ppv.is_finite()
            || !fnr.is_finite()
            || prevalence_p <= 0.0
            || prevalence_p >= 1.0
            || ppv <= 0.0
            || ppv >= 1.0
            || !(0.0..1.0).contains(&fnr)
        {
            return 0.0;
        }
        let p_safe = prevalence_p.clamp(1e-12, 1.0 - 1e-12);
        let ppv_safe = ppv.clamp(1e-12, 1.0);
        let fnr_safe = fnr.clamp(0.0, 1.0);
        (p_safe / (1.0 - p_safe)) * ((1.0 - ppv_safe) / ppv_safe) * (1.0 - fnr_safe)
    }

    /// Computes $(\mathrm{FPR}_A, \mathrm{FPR}_B, \mathrm{FPR}_A / \mathrm{FPR}_B)$ when $\mathrm{PPV}$ and $\mathrm{FNR}$ are equalized across groups with different base rates $p_A \neq p_B$.
    pub fn fpr_ratio_under_equal_ppv_and_fnr(
        p_a: f64,
        p_b: f64,
        target_ppv: f64,
        target_fnr: f64,
    ) -> (f64, f64, f64) {
        let fpr_a = Self::chouldechova_fpr(p_a, target_ppv, target_fnr);
        let fpr_b = Self::chouldechova_fpr(p_b, target_ppv, target_fnr);
        (fpr_a, fpr_b, fpr_a / fpr_b.max(1e-15))
    }

    /// Audits two empirical confusion matrices `(tp, fp, tn, fn_count)` for groups $A$ and $B$.
    pub fn from_confusion_matrices(
        group_a: (usize, usize, usize, usize),
        group_b: (usize, usize, usize, usize),
    ) -> Self {
        let metrics = |(tp, fp, tn, fn_c): (usize, usize, usize, usize)| {
            let tp_f = tp as f64;
            let fp_f = fp as f64;
            let tn_f = tn as f64;
            let fn_f = fn_c as f64;
            let total = (tp_f + fp_f + tn_f + fn_f).max(1.0);
            let p = (tp_f + fn_f) / total;
            let ppv = tp_f / (tp_f + fp_f).max(1.0);
            let fnr = fn_f / (tp_f + fn_f).max(1.0);
            let fpr = fp_f / (fp_f + tn_f).max(1.0);
            let fpr_id = Self::chouldechova_fpr(p, ppv, fnr);
            (p, ppv, fnr, fpr, (fpr - fpr_id).abs())
        };

        let (p_a, ppv_a, fnr_a, fpr_a, err_a) = metrics(group_a);
        let (p_b, ppv_b, fnr_b, fpr_b, err_b) = metrics(group_b);
        let p_a_safe = p_a.clamp(1e-12, 1.0 - 1e-12);
        let p_b_safe = p_b.clamp(1e-12, 1.0 - 1e-12);
        let odds_a = p_a_safe / (1.0 - p_a_safe);
        let odds_b = p_b_safe / (1.0 - p_b_safe);

        Self {
            prevalence_a: p_a,
            prevalence_b: p_b,
            ppv_a,
            ppv_b,
            fnr_a,
            fnr_b,
            fpr_a,
            fpr_b,
            identity_max_error: err_a.max(err_b),
            fpr_ratio_under_equal_ppv_and_fnr: odds_a / odds_b.max(1e-15),
            simultaneous_equality_impossible: (p_a - p_b).abs() > 1e-6,
        }
    }

    /// Kleinberg, Mullainathan & Raghavan (KMR 2017) continuous risk-score impossibility identity:
    /// For any calibrated risk score $S \in [0, 1]$ ($\mathbb{E}[Y \mid S = s, G = g] = s$), the total Score Variance
    /// satisfies $\mathrm{Var}(S \mid G = g) = \mu_g(1 - \mu_g)\big(1 - \text{BrierRatio}_g\big)$.
    /// If a calibrated risk score also satisfies Balance for the Negative Class ($\mathbb{E}[S \mid Y=0, A] = \mathbb{E}[S \mid Y=0, B]$)
    /// and Balance for the Positive Class ($\mathbb{E}[S \mid Y=1, A] = \mathbb{E}[S \mid Y=1, B]$), then either $\mu_A = \mu_B$
    /// or the classifier is deterministic ($\mathrm{Var}(S) = \mu(1 - \mu)$).
    /// Returns `(expected_score_neg_gap, expected_score_pos_gap, calibration_variance_gap)`.
    pub fn kmr_continuous_impossibility_gap(&self) -> (f64, f64, f64) {
        let neg_score_a = self.fpr_a * self.ppv_a;
        let neg_score_b = self.fpr_b * self.ppv_b;
        let pos_score_a = (1.0 - self.fnr_a) * self.ppv_a;
        let pos_score_b = (1.0 - self.fnr_b) * self.ppv_b;
        let base_var_a = self.prevalence_a * (1.0 - self.prevalence_a);
        let base_var_b = self.prevalence_b * (1.0 - self.prevalence_b);
        (
            (neg_score_a - neg_score_b).abs(),
            (pos_score_a - pos_score_b).abs(),
            (base_var_a - base_var_b).abs(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moulton_campbell_and_exceedance_benchmarks_reproduce_protocol() {
        let b1 = ExceedanceDesignEffect::benchmark_calibration_25028();
        assert_eq!(b1.n_nominal, 25_028);
        assert!(b1.m_tilde_moulton > b1.m_bar * 1.8);
        assert!(b1.n_effective_p > 1_200.0 && b1.n_effective_p < 1_400.0);
        assert!(b1.n_effective_p < b1.n_effective_naive_kish);

        let b2 = ExceedanceDesignEffect::benchmark_pseudoreplication_500_to_86();
        assert_eq!(b2.n_nominal, 500);
        assert!((b2.n_effective_p - 86.0).abs() < 1.0);

        let amp = OperationalWeightingGap::variance_amplification_benchmark_69x();
        assert!((amp - 69.0).abs() < 1e-12);
    }

    #[test]
    fn phi5_rejects_coarse_annual_aggregate_against_daily_unit() {
        assert!(!verify_resolution_phi5(
            "operational day",
            "Set a single annual aggregate average for the entire year"
        ));
        assert!(verify_resolution_phi5(
            "operational day",
            "Allocate 26 agents to the daily peak shift"
        ));
    }

    #[test]
    fn chouldechova_trilemma_prevents_simultaneous_equality() {
        let audit =
            ChouldechovaAudit::from_confusion_matrices((180, 90, 630, 100), (70, 55, 825, 50));
        assert!(audit.identity_max_error < 1e-12);
        assert!(audit.simultaneous_equality_impossible);
        assert!((audit.fpr_ratio_under_equal_ppv_and_fnr - 1.0).abs() > 0.5);
    }
}
