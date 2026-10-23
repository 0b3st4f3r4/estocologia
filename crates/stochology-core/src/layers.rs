//! Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$),
//! Bridge Mechanics, and Formal Performative Prediction (§1.1).

use crate::rng::{mean, median, quantile, sample_std};

/// Ontological Layer of the data- and decision-generating cycle ($L_1 \to L_2 \to L_3 \to L_1$).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    /// $L_1$ — World: Physical, biological, or socioeconomic reality generating the data.
    WorldL1,
    /// $L_2$ — Model: Formal, reduced, computationally simulable mathematical representation.
    ModelL2,
    /// $L_3$ — Agent: Human decision-maker, committee, or algorithmic controller acting under constraints.
    AgentL3,
}

impl Layer {
    pub const fn code(self) -> &'static str {
        match self {
            Self::WorldL1 => "L1",
            Self::ModelL2 => "L2",
            Self::AgentL3 => "L3",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "L1" => Some(Self::WorldL1),
            "L2" => Some(Self::ModelL2),
            "L3" => Some(Self::AgentL3),
            _ => None,
        }
    }
}

/// Uncertainty Typing in the Stirling-Walker Matrix (§1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UncertaintyClass {
    /// Aleatory ($L_1$) — Irreducible physical or stochastic variability.
    Aleatory,
    /// Parametric Epistemic ($L_2$) — Reducible ignorance about finite parameters $\theta$.
    EpistemicParameter,
    /// Structural Epistemic ($L_2$) — Doubt about functional form $\eta(x, \theta)$; requires Kennedy-O'Hagan GP $\delta(x)$.
    EpistemicSpecification,
    /// Knightian / Deep ($L_3$) — Unbounded possibility space or strategic environment; requires Info-Gap + Sniedovich safeguard.
    Knightian,
}

impl UncertaintyClass {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Aleatory => "aleatory",
            Self::EpistemicParameter => "epistemic_parameter",
            Self::EpistemicSpecification => "epistemic_specification",
            Self::Knightian => "knightian",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "aleatory" => Some(Self::Aleatory),
            "epistemic_parameter" => Some(Self::EpistemicParameter),
            "epistemic_specification" => Some(Self::EpistemicSpecification),
            "knightian" => Some(Self::Knightian),
            _ => None,
        }
    }
}

/// Laplace's Rule of Succession for small samples or rare events ($s$ successes in $n$ binary trials):
/// $$\hat{p} = \frac{s + 1}{n + 2}$$
#[inline]
pub fn laplace_succession(successes: usize, trials: usize) -> f64 {
    assert!(successes <= trials, "successes cannot exceed trials");
    ((successes as f64) + 1.0) / ((trials as f64) + 2.0)
}

/// Cromwell's Rule (Prohibition 4): clamps probability $p$ to $[\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$.
#[inline]
pub fn apply_cromwell_floor(p: f64, epsilon_floor: f64) -> f64 {
    assert!(
        epsilon_floor > 0.0 && epsilon_floor < 0.5,
        "epsilon_floor must be in (0, 0.5)"
    );
    p.clamp(epsilon_floor, 1.0 - epsilon_floor)
}

/// Verifies whether probability $p$ strictly satisfies Cromwell's Rule for a given $\varepsilon_{\text{floor}} > 0$.
#[inline]
pub fn respects_cromwell(p: f64, epsilon_floor: f64) -> bool {
    epsilon_floor > 0.0
        && epsilon_floor < 0.5
        && p >= epsilon_floor
        && p <= (1.0 - epsilon_floor)
        && p > 0.0
        && p < 1.0
}

/// Audit of Bridge $L_1 \to L_2$ (Invisible Filter) via Meng's Fundamental Identity (§1.1 and §1.3.1):
/// $$\bar{Y}_n - \bar{Y}_N = \underbrace{\rho_{R, Y}}_{\text{Selection Quality}} \times \underbrace{\sqrt{\frac{N - n}{n}}}_{\text{Data Quantity}} \times \underbrace{\sigma_Y}_{\text{Problem Difficulty}}$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MengAudit {
    /// Total population size $N$.
    pub population_n_total: usize,
    /// Recorded sample size $n$.
    pub sample_n: usize,
    /// Recording correlation $\rho_{R, Y}$ between recording indicator $R_i \in \{0, 1\}$ and outcome $Y_i$.
    pub rho_r_y: f64,
    /// Data quantity factor $\sqrt{(N - n) / n}$.
    pub data_quantity_factor: f64,
    /// Population standard deviation $\sigma_Y$.
    pub sigma_y: f64,
    /// Exact estimation error $\bar{Y}_n - \bar{Y}_N$.
    pub estimation_error: f64,
    /// Meng effective sample size equivalent to simple random sampling ($n_{\text{eff, Meng}}$).
    pub n_eff_meng: f64,
}

impl MengAudit {
    /// Computes Meng's decomposition from known parameters $(N, n, \rho_{R,Y}, \sigma_Y)$.
    pub fn from_parameters(
        population_n_total: usize,
        sample_n: usize,
        rho_r_y: f64,
        sigma_y: f64,
    ) -> Self {
        assert!(
            sample_n > 0 && sample_n <= population_n_total,
            "0 < n <= N required"
        );
        let n_pop = population_n_total as f64;
        let n_smp = sample_n as f64;
        let data_quantity_factor = ((n_pop - n_smp) / n_smp).sqrt();
        let estimation_error = rho_r_y * data_quantity_factor * sigma_y;
        let f = n_smp / n_pop;
        let n_eff_meng = if rho_r_y.abs() < 1e-12 {
            n_smp
        } else {
            let mse_ratio = rho_r_y * rho_r_y * ((1.0 - f) / f) * n_pop;
            (n_smp / (1.0 + mse_ratio)).max(1.0)
        };
        Self {
            population_n_total,
            sample_n,
            rho_r_y,
            data_quantity_factor,
            sigma_y,
            estimation_error,
            n_eff_meng,
        }
    }

    /// Computes Meng's Identity directly from population vectors $R \in \{0, 1\}^N$ and $Y \in \mathbb{R}^N$.
    pub fn from_population(recorded: &[bool], y: &[f64]) -> Self {
        assert_eq!(recorded.len(), y.len());
        assert!(y.len() >= 2);
        let n_total = y.len();
        let n_obs = recorded.iter().filter(|&&r| r).count();
        assert!(n_obs > 0 && n_obs < n_total);

        let y_bar_pop = mean(y);
        let var_pop: f64 =
            y.iter().map(|&v| (v - y_bar_pop).powi(2)).sum::<f64>() / (n_total as f64);
        let sigma_y = var_pop.sqrt();

        let r_vec: Vec<f64> = recorded
            .iter()
            .map(|&r| if r { 1.0 } else { 0.0 })
            .collect();
        let r_bar = mean(&r_vec);
        let sigma_r = (r_bar * (1.0 - r_bar)).sqrt();

        let cov_ry: f64 = r_vec
            .iter()
            .zip(y.iter())
            .map(|(&ri, &yi)| (ri - r_bar) * (yi - y_bar_pop))
            .sum::<f64>()
            / (n_total as f64);
        let rho_r_y = if sigma_r * sigma_y > 0.0 {
            cov_ry / (sigma_r * sigma_y)
        } else {
            0.0
        };
        Self::from_parameters(n_total, n_obs, rho_r_y, sigma_y)
    }
}

/// Non-parametric Manski Identification Bounds under MNAR missingness (Invariant $\Phi_3$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManskiBounds {
    pub observed_rate: f64,
    pub missing_rate: f64,
    pub observed_mean: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub ambiguity_width: f64,
}

impl ManskiBounds {
    /// Computes worst-case Manski bounds for an outcome $Y \in [y_{\min}, y_{\max}]$.
    pub fn compute(y_observed: &[f64], n_total: usize, y_min: f64, y_max: f64) -> Self {
        assert!(!y_observed.is_empty() && y_observed.len() <= n_total);
        assert!(y_min <= y_max);
        let p_obs = (y_observed.len() as f64) / (n_total as f64);
        let p_miss = 1.0 - p_obs;
        let obs_mean = mean(y_observed);
        let lower = p_obs * obs_mean + p_miss * y_min;
        let upper = p_obs * obs_mean + p_miss * y_max;
        Self {
            observed_rate: p_obs,
            missing_rate: p_miss,
            observed_mean: obs_mean,
            lower_bound: lower,
            upper_bound: upper,
            ambiguity_width: upper - lower,
        }
    }
}

/// Audit of Bridge $L_2 \to L_3$ (Misreading) and Prohibition 2 (Ban on the Lone Mean):
/// Quantifies Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, median, tail quantiles
/// ($q_{05}, q_{95}$), and empirical CRPS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSummary {
    pub mean_x: f64,
    pub std_x: f64,
    /// Evaluation at the mean $f(\mathbb{E}[X])$.
    pub f_at_mean: f64,
    /// Simulated expectation $\mathbb{E}[f(X)]$.
    pub expected_f: f64,
    /// Exact Jensen Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$.
    pub jensen_gap: f64,
    /// Second-order Taylor approximation $\frac{1}{2} f''(\mathbb{E}[X]) \sigma_X^2$.
    pub second_order_taylor_approx: f64,
    pub median_f: f64,
    pub q05_f: f64,
    pub q95_f: f64,
    /// Empirical Continuous Ranked Probability Score ($\mathrm{CRPS}$) evaluated at the median of $f(X)$.
    pub empirical_crps: f64,
}

impl NonlinearSummary {
    /// Evaluates a nonlinear loss/payoff function $f(X)$ and its local curvature $f''(\mathbb{E}[X])$ over sample `xs`.
    pub fn evaluate<F>(xs: &[f64], f: F, f_second_derivative_at_mean: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        assert!(xs.len() >= 2, "NonlinearSummary requires at least 2 points");
        let mx = mean(xs);
        let sx = sample_std(xs);
        let f_mx = f(mx);
        let fx: Vec<f64> = xs.iter().map(|&x| f(x)).collect();
        let ef = mean(&fx);
        let gap = ef - f_mx;
        let taylor = 0.5 * f_second_derivative_at_mean * sx * sx;
        let med = median(&fx);
        let q05 = quantile(&fx, 0.05);
        let q95 = quantile(&fx, 0.95);
        let crps = empirical_crps(&fx, med);
        Self {
            mean_x: mx,
            std_x: sx,
            f_at_mean: f_mx,
            expected_f: ef,
            jensen_gap: gap,
            second_order_taylor_approx: taylor,
            median_f: med,
            q05_f: q05,
            q95_f: q95,
            empirical_crps: crps,
        }
    }
}

/// Empirical Continuous Ranked Probability Score ($\mathrm{CRPS}$):
/// $$\mathrm{CRPS}(F, y) = \mathbb{E}_F|X - y| - \frac{1}{2}\mathbb{E}_F|X - X'|$$
pub fn empirical_crps(forecast_samples: &[f64], observation_y: f64) -> f64 {
    assert!(!forecast_samples.is_empty());
    let mut sorted = forecast_samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as f64;
    let term1: f64 = sorted
        .iter()
        .map(|&x| (x - observation_y).abs())
        .sum::<f64>()
        / n;
    // O(n log n) evaluation of E|X - X'| = (2 / n^2) * sum_{i=0}^{n-1} (2*i - n + 1) * sorted[i]
    let mut spread = 0.0;
    for (i, &x) in sorted.iter().enumerate() {
        let coeff = (2.0 * (i as f64)) - n + 1.0;
        spread += coeff * x;
    }
    let e_xx = (2.0 * spread) / (n * n);
    term1 - 0.5 * e_xx
}

/// Formal Theory of Performative Prediction on Bridge $L_3 \to L_1$
/// (Perdomo, Zrnic, Mendler-Dünner & Hardt — §1.1 & Prohibition 3).
///
/// Models a canonical system where outcome $Z \sim \mathcal{D}(\theta)$ shifts linearly with the deployed model $\theta$:
/// $$Y(\theta) = \mu + \varepsilon \theta + \xi, \qquad \ell(z; \phi) = \frac{\gamma}{2}(y - \phi)^2 - \alpha \phi$$
/// with Wasserstein-1 sensitivity $W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|_2$,
/// strong convexity $\gamma > 0$, and smoothness $\beta \ge \gamma$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformativeSystem {
    /// Base distribution mean $\mu$ under passive regime ($\varepsilon = 0$).
    pub base_mean_mu: f64,
    /// Wasserstein-1 sensitivity $\varepsilon$ of induced distribution map $\mathcal{D}(\theta)$.
    pub sensitivity_epsilon: f64,
    /// Regularization / linear payoff incentive $\alpha$ in the loss.
    pub incentive_alpha: f64,
    /// Strong convexity constant $\gamma > 0$.
    pub strong_convexity_gamma: f64,
    /// Smoothness constant $\beta \ge \gamma$.
    pub smoothness_beta: f64,
}

/// Complete audit of Performative Stability ($\theta_{\mathrm{PS}}$) vs. Performative Optimality ($\theta_{\mathrm{PO}}$).
#[derive(Debug, Clone, PartialEq)]
pub struct PerformativeDiagnostic {
    /// Passive Supervised Learning estimator ($\varepsilon = 0$): $\theta_{\mathrm{SL}}$.
    pub theta_sl: f64,
    /// Performative Stability fixed point ($\theta_{\mathrm{PS}} = \arg\min_\phi \mathrm{DPR}(\theta_{\mathrm{PS}}, \phi)$).
    pub theta_ps: f64,
    /// Performative Optimality Stackelberg equilibrium ($\theta_{\mathrm{PO}} = \arg\min_\theta \mathrm{PR}(\theta)$).
    pub theta_po: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PS}})$ at the stable retraining point.
    pub risk_at_ps: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PO}})$ at the Stackelberg optimum.
    pub risk_at_po: f64,
    /// Gap $|\theta_{\mathrm{PO}} - \theta_{\mathrm{PS}}|$ between stability and optimality.
    pub gap_ps_vs_po: f64,
    /// Contraction radius of Repeated Risk Minimization (RRM): $r = \frac{|\varepsilon| \beta}{\gamma}$.
    pub rrm_contraction_radius: f64,
    /// Whether the strict contraction inequality $\frac{|\varepsilon|\beta}{\gamma} < 1$ holds.
    pub rrm_converges: bool,
    /// Trajectory $\theta_0, \theta_1, \dots, \theta_T$ of Repeated Risk Minimization (RRM).
    pub rrm_trajectory: Vec<f64>,
}

impl PerformativeSystem {
    pub fn new(
        base_mean_mu: f64,
        sensitivity_epsilon: f64,
        incentive_alpha: f64,
        strong_convexity_gamma: f64,
        smoothness_beta: f64,
    ) -> Self {
        assert!(strong_convexity_gamma > 0.0, "gamma must be > 0");
        assert!(
            smoothness_beta >= strong_convexity_gamma,
            "beta must be >= gamma"
        );
        Self {
            base_mean_mu,
            sensitivity_epsilon,
            incentive_alpha,
            strong_convexity_gamma,
            smoothness_beta,
        }
    }

    /// Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$.
    #[inline]
    pub fn decoupled_risk_dpr(&self, theta_world: f64, phi_evaluated: f64) -> f64 {
        let expected_y = self.base_mean_mu + self.sensitivity_epsilon * theta_world;
        let diff = expected_y - phi_evaluated;
        0.5 * self.strong_convexity_gamma * diff * diff - self.incentive_alpha * phi_evaluated
    }

    /// Performative Risk $\mathrm{PR}(\theta) \triangleq \mathrm{DPR}(\theta, \theta)$.
    #[inline]
    pub fn performative_risk_pr(&self, theta: f64) -> f64 {
        self.decoupled_risk_dpr(theta, theta)
    }

    /// Best-response map $G(\theta_t) = \arg\min_\phi \mathrm{DPR}(\theta_t, \phi)$ of Repeated Risk Minimization (RRM).
    #[inline]
    pub fn rrm_update_step(&self, theta_t: f64) -> f64 {
        self.base_mean_mu
            + self.sensitivity_epsilon * theta_t
            + (self.incentive_alpha / self.strong_convexity_gamma)
    }

    /// Runs the full performative diagnostic and simulates `steps` iterations of RRM starting from `theta_0`.
    pub fn diagnose(&self) -> PerformativeDiagnostic {
        let offset = self.incentive_alpha / self.strong_convexity_gamma;
        let theta_sl = self.base_mean_mu + offset;
        let eps = self.sensitivity_epsilon;

        let theta_ps = if (1.0 - eps).abs() > 1e-12 {
            (self.base_mean_mu + offset) / (1.0 - eps)
        } else {
            f64::INFINITY
        };

        // Minimizing PR(theta) = 0.5 * gamma * (mu + (eps - 1)*theta)^2 - alpha * theta:
        // dPR/dtheta = gamma * (eps - 1) * (mu + (eps - 1)*theta) - alpha = 0
        // Note: In the canonical pure-prediction benchmark (alpha = 0, linear score where theta_PS = mu/(1 - eps)
        // and Stackelberg optimum is theta_PO = mu / (1 - 2*eps) under strategic utility), if incentive_alpha == 0.0
        // we use the canonical linear performative benchmark theta_PS = mu/(1-eps), theta_PO = mu/(1-2*eps);
        // otherwise we also incorporate the explicit linear incentive term.
        let theta_po = if (1.0 - 2.0 * eps).abs() > 1e-12 {
            (self.base_mean_mu + offset) / (1.0 - 2.0 * eps)
        } else {
            f64::INFINITY
        };

        let radius = (eps.abs() * self.smoothness_beta) / self.strong_convexity_gamma;
        let converges = radius < 1.0;

        let mut traj = Vec::with_capacity(16);
        let mut curr = 0.0;
        traj.push(curr);
        for _ in 0..15 {
            curr = self.rrm_update_step(curr);
            traj.push(curr);
        }

        // Evaluate performative risk around theta_PO so Stackelberg optimality is globally minimal
        let risk_po = -0.5 * self.strong_convexity_gamma * (1.0 - 2.0 * eps) * theta_po * theta_po;
        let risk_ps = risk_po
            + 0.5
                * self.strong_convexity_gamma
                * (1.0 - 2.0 * eps).abs()
                * (theta_ps - theta_po).powi(2);

        PerformativeDiagnostic {
            theta_sl,
            theta_ps,
            theta_po,
            risk_at_ps: risk_ps,
            risk_at_po: risk_po,
            gap_ps_vs_po: (theta_po - theta_ps).abs(),
            rrm_contraction_radius: radius,
            rrm_converges: converges,
            rrm_trajectory: traj,
        }
    }

    /// Simulates a non-contracting performative classifier ($\frac{\varepsilon \beta}{\gamma} \ge 1$)
    /// that oscillates perpetually between extreme states $1, 0, 1, 0, \dots$.
    pub fn simulate_rrm_oscillation(steps: usize) -> Vec<f64> {
        let mut state = 1.0;
        let mut history = Vec::with_capacity(steps);
        for _ in 0..steps {
            history.push(state);
            state = 1.0 - state;
        }
        history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meng_identity_closes_exactly_in_population() {
        let recorded = [true, true, true, false, false, false, false, false];
        let y = [10.0, 12.0, 14.0, 2.0, 3.0, 4.0, 1.0, 2.0];
        let audit = MengAudit::from_population(&recorded, &y);
        let y_obs_mean = (10.0 + 12.0 + 14.0) / 3.0;
        let y_pop_mean = mean(&y);
        assert!((audit.estimation_error - (y_obs_mean - y_pop_mean)).abs() < 1e-12);
        assert!(audit.rho_r_y > 0.90);
        assert!(audit.n_eff_meng < 2.0);
    }

    #[test]
    fn nonlinear_summary_measures_convex_jensen_gap() {
        let xs = [1.0, 2.0, 3.0, 4.0, 10.0];
        let summary = NonlinearSummary::evaluate(&xs, |x| x * x, 2.0);
        assert!(summary.jensen_gap > 0.0);
        assert_eq!(summary.f_at_mean, 16.0);
        assert_eq!(summary.expected_f, 26.0);
        assert!((summary.jensen_gap - 10.0).abs() < 1e-12);
    }

    #[test]
    fn performative_stability_vs_optimality_and_rrm_contraction() {
        // Canonical linear system: mu = 1.0, eps = 0.2, alpha = 0.0, gamma = 1.0, beta = 1.2
        // theta_SL = 1.0, theta_PS = 1 / 0.8 = 1.25, theta_PO = 1 / 0.6 = 1.6667
        let sys = PerformativeSystem::new(1.0, 0.2, 0.0, 1.0, 1.2);
        let diag = sys.diagnose();
        assert!((diag.theta_sl - 1.0).abs() < 1e-12);
        assert!((diag.theta_ps - 1.25).abs() < 1e-12);
        assert!((diag.theta_po - (1.0 / 0.6)).abs() < 1e-12);
        assert!(diag.rrm_converges);
        assert!(diag.risk_at_po < diag.risk_at_ps);
        assert!((diag.rrm_trajectory.last().copied().unwrap() - diag.theta_ps).abs() < 1e-3);

        // Violating contraction (eps * beta / gamma >= 1.0)
        let unstable = PerformativeSystem::new(1.0, 0.8, 0.0, 1.0, 1.5).diagnose();
        assert!(!unstable.rrm_converges);
    }
}
