# Stochology Protocol (v1.0): Enriched Methodological Architecture for Judgment and Decision-Making under Uncertainty

This document establishes the consolidated, mathematically formalized specification of the **Stochology Protocol (`contract_version: "1.0"`)** for data architecture, stochastic modeling, causal inference, and structured inter-agent communication. It unifies operational decision-making with explicit algorithmic gates for performative prediction, decision theory under severe uncertainty, strong information contraction, tail design effects in unequal clusters, and formal production drift adaptation.

---

## 1. Deep Structural Diagnosis: Bridge Mechanics, Severe Uncertainty, and the Five Transversal Lenses

Before fitting any parameter, selecting predictive architectures, or estimating probability distributions, every technical claim, scientific hypothesis, or operational model must auditably answer three structural questions:
1. In which ontological layer can the system fail (**World $L_1$**, **Model $L_2$**, or **Agent $L_3$**)?
2. Which class of uncertainty threatens inference validity (**Aleatory**, **Parametric/Structural Epistemic**, or **Knightian**)?
3. Which **severe test** — endowed with genuine empirical or logical refutation power — supports the confidence placed in the claim?

### 1.1. Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$)

| Layer | Code | Ontological Definition | Typical Outbound Bridge Error | Formal Bridge Audit |
| --- | --- | --- | --- | --- |
| **World** | $L_1$ | Physical, biological, or socioeconomic reality generating the data, never observed in its entirety. | **Bridge $L_1 \to L_2$ (Invisible Filter):** Selection, censoring, or survivorship bias. | Meng's Identity ($\rho_{R,Y} \neq 0$), Rubin's missingness taxonomy (`MCAR`, `MAR`, `MNAR`), and Manski's non-parametric bounds. |
| **Model** | $L_2$ | Formal, reduced, computationally simulable mathematical representation of the process. | **Bridge $L_2 \to L_3$ (Misreading):** Treating $\mathbb{E}[X]$ as a sufficient summary of a nonlinear payoff/loss $f(X)$. | Quantification of Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ and evaluation via strictly proper scoring rules ($\mathrm{CRPS}$, Brier). |
| **Agent** | $L_3$ | Human decision-maker, institutional committee, or algorithmic controller acting under constraints. | **Bridge $L_3 \to L_1$ (Performativity):** Decisions based on the model deform the very distribution $\mathcal{D}(\theta)$ being measured. | Induced distribution map $\mathcal{D}(\theta)$, separation between Performative Stability ($\theta_{\mathrm{PS}}$) and Performative Optimality ($\theta_{\mathrm{PO}}$), and RRM contraction $\frac{\varepsilon\beta}{\gamma} < 1$. |

On **Bridge $L_3 \to L_1$**, when a model parameterized by $\theta \in \Theta$ supports consequential decisions, deploying $\theta$ causally alters the environment and induces $Z = (X, Y) \sim \mathcal{D}(\theta)$. Performance is governed by Performative Risk $\mathrm{PR}(\theta)$ and Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi)$ (Perdomo, Zrnic, Mendler-Dünner & Hardt):

$$\mathrm{PR}(\theta) \triangleq \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \theta)], \qquad \mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$$

- **Performative Stability ($\theta_{\mathrm{PS}}$):** $\theta_{\mathrm{PS}} = \arg\min_{\phi \in \Theta} \mathrm{DPR}(\theta_{\mathrm{PS}}, \phi)$ — fixed point of Repeated Risk Minimization ($\mathrm{RRM}$).
- **Performative Optimality ($\theta_{\mathrm{PO}}$):** $\theta_{\mathrm{PO}} = \arg\min_{\theta \in \Theta} \mathrm{PR}(\theta)$ — Stackelberg equilibrium anticipating the strategic reaction of the population.
- **Canonical Linear Benchmark:** Under $Y(\theta) = \mu + \varepsilon \theta + \xi$, passive learning yields $\theta_{\mathrm{SL}} = \mu$, retraining converges to $\theta_{\mathrm{PS}} = \frac{\mu}{1 - \varepsilon}$, whereas the true Stackelberg optimum is $\theta_{\mathrm{PO}} = \frac{\mu}{1 - 2\varepsilon}$. Under Wasserstein-1 sensitivity $W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|_2$, $\gamma$-strong convexity, and $\beta$-smoothness, $\mathrm{RRM}$ contracts linearly if and only if $\frac{\varepsilon \beta}{\gamma} < 1$.

### 1.2. Uncertainty Typing: Kennedy-O'Hagan Discrepancy and Info-Gap + Sniedovich Safeguard

1. **Aleatory Uncertainty (`aleatory`, $L_1$):** Irreducible variability; demands safety margins and exposure control.
2. **Parametric Epistemic Uncertainty (`epistemic_parameter`, $L_2$):** Reducible ignorance about finite parameters $\theta$; uses Laplace's rule of succession $\hat{p} = \frac{s+1}{n+2}$ in small samples.
3. **Structural Epistemic Uncertainty (`epistemic_specification`, $L_2$):** Formulated via **Kennedy-O'Hagan Structural Discrepancy**:
   $$y(x) = \eta(x, \theta) + \delta(x) + \varepsilon, \qquad \delta(x) \sim \mathcal{GP}(m(x), k(x, x'))$$
   with anchoring constraints to prevent parameter pseudo-convergence and extrapolation overconfidence.
4. **Knightian Uncertainty (`knightian`, $L_3$):** Formulated via **Info-Gap Decision Theory** (Ben-Haim) coupled with **Sniedovich's Global Maximin Safeguard** (Wald):
   $$\hat{\alpha}(q, r_c) = \max\left\{\alpha \ge 0 : \min_{u \in \mathcal{U}(\alpha, \tilde{u})} R(q, u) \ge r_c\right\}, \qquad \hat{\beta}(q, r_w) = \min\left\{\alpha \ge 0 : \max_{u \in \mathcal{U}(\alpha, \tilde{u})} R(q, u) \ge r_w\right\}$$
   Enforces the *zero-ing* property ($\hat{\alpha}(q, R(q, \tilde{u})) = 0$) and requires a global Wald Maximin floor outside $\mathcal{U}(\hat{\alpha}, \tilde{u})$ to prevent catastrophic collapse in "No-Man's Land".

### 1.3. Mathematical Formalism of the Five Transversal Lenses

1. **Lens 1 — Information (`stochology_lenses::information`):**
   - Data Processing Inequality ($I(W; Z) \le I(W; Y) \le I(W; X)$) and **Strong Data Processing Inequality (SDPI)** contraction hierarchy:
     $$\eta_{\chi^2}(W) = \rho_m^2(X; Y) \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W)$$
   - Fano's Error Floor: $H(P_e) + P_e \ln(|\mathcal{W}| - 1) \ge H(W) - I(W; Z)$.
   - Meng's Fundamental Identity: $\bar{Y}_n - \bar{Y}_N = \rho_{R,Y} \times \sqrt{\frac{N-n}{n}} \times \sigma_Y$.
2. **Lens 2 — Time (`stochology_lenses::time`):**
   - Itô Variance Drain: $g_{\text{time}} = \mu - \frac{1}{2}\sigma^2 < \mu = g_{\text{ensemble}}$.
   - Optimal Asymmetric Kelly Fraction: $f^* = \frac{p}{a} - \frac{1 - p}{b}$.
   - Heavy-Tailed Mixing Time under Log-Sobolev Inequality ($\alpha_{\mathrm{LSI}} = -\frac{1}{2}\ln \eta_{\mathrm{KL}}$) and tail mass $H_R$.
3. **Lens 3 — Scale and Tail (`stochology_lenses::tail`):**
   - Hill Estimator: $\hat{\alpha}_{k,n} = \left(\frac{1}{k}\sum_{i=1}^k \ln \frac{X_{(n-i+1)}}{X_{(n-k)}}\right)^{-1}$ and moment regimes ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$).
4. **Lens 4 — Structure and Invariance (`stochology_lenses::invariance`):**
   - Pearl's Causal Ladder (`1-See`, `2-Act`, `3-Imagine`), Invariant Causal Prediction ($\hat{S}^* = \bigcap_{S:\text{invariant}} S$), NILE score, 2nd-order worst-case loss, and hidden confounding threshold $\Lambda^*$.
5. **Lens 5 — Decision and Calibration (`stochology_lenses::decision`):**
   - Cost-Loss Boundary: $p > p^* \triangleq \frac{C}{\eta L}$ and Expected Value of Perfect Information ($\mathrm{EVPI}$).
   - Exact Isotonic **CORP-Murphy Decomposition** via *Pool-Adjacent-Violators* (PAV):
     $$\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$$

---

## 2. Structured Communication Contract, Prohibitions, and Boundary Invariants (`contract_version: "1.0"`)

### 2.1. The Four Prohibitions
- **Prohibition 1:** Ban on unstructured free prose between modules/agents; all claims transit via `EpistemologicalCard` (`cards/*.yaml`).
- **Prohibition 2:** Ban on the lone mean under nonlinearity or heavy tails; mandatory reporting of $f(\mathbb{E}[X]), \mathbb{E}[f(X)], \Delta_J, \text{median}, q_{05}, q_{95}, \mathrm{CRPS}$.
- **Prohibition 3:** Ban on confusing predictive fit with causal/performative validity; mandatory audit of $\theta_{\mathrm{PS}}$ vs. $\theta_{\mathrm{PO}}$ and $\frac{\varepsilon\beta}{\gamma} < 1$.
- **Prohibition 4:** Ban on dogmatic extremes ($0\%$ or $100\%$, Cromwell's Rule $p \in [\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$) and omitted attempts ($K \ge 1$, *e-process* $E_t$, and CORP-Murphy decomposition).

### 2.2. The Five Boundary Invariants ($\Phi_1$ to $\Phi_5$)
- **$\Phi_1$ (Simulability):** Fixed seed (`seed = 2026`, `SplitMix64`) and executable generative null model.
- **$\Phi_2$ (Independent Unit & Exceedance Design Effect):** Moulton-Campbell unequal-cluster correction $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2(m_j))$, threshold exceedance ICC $\rho_I(p)$, and $n_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m} - 1)\rho_I(p)}$.
- **$\Phi_3$ (Severe Test & Missingness):** Rubin classification (`MCAR`, `MAR`, `MNAR`), SHA-256 pre-registration seal, and Pearl's Ladder rung.
- **$\Phi_4$ (Scope, Fairness & Exit):** Chouldechova's algebraic impossibility identity $\mathrm{FPR}_g = \frac{p_g}{1-p_g}\frac{1-\mathrm{PPV}_g}{\mathrm{PPV}_g}(1-\mathrm{FNR}_g)$, scope statement, and numeric exit clause.
- **$\Phi_5$ (Question Resolution):** Ban on answering daily/shift operational decisions with coarse annual/monthly aggregates.

---

## 3. Architectural Mapping to Rust Crates

| Protocol Section | Concept / Equation | Rust Module & Symbol | Verification Command |
| --- | --- | --- | --- |
| §1.1 Bridge $L_1 \to L_2$ | Meng's Identity ($\rho_{R,Y}$) and Manski MNAR Bounds | `stochology_core::layers::{MengAudit, ManskiBounds}` | `cargo run --release --bin labs -- --layers` |
| §1.1 Bridge $L_2 \to L_3$ | Jensen's Gap $\Delta_J$ and empirical $\mathrm{CRPS}$ | `stochology_core::layers::NonlinearSummary` | `cargo run --release --bin labs -- --layers` |
| §1.1 Bridge $L_3 \to L_1$ | Performative Prediction ($\theta_{\mathrm{SL}}, \theta_{\mathrm{PS}}, \theta_{\mathrm{PO}}$) and RRM contraction | `stochology_core::layers::PerformativeSystem` | `cargo run --release --bin labs -- --layers` & `gates -- --defect performativity` |
| §1.2 Structural & Knightian | Kennedy-O'Hagan GP $\delta(x)$ and Info-Gap + Sniedovich Safeguard | `stochology_core::uncertainty::{KennedyOHaganDiscrepancy, InfoGapOption}` | `cargo run --release --bin labs -- --uncertainty` & `gates -- --defect sniedovich` |
| §1.3 Lenses 1–5 | SDPI/Fano, Itô/Kelly/Mixing, Hill, Pearl/ICP/NILE/$\Lambda^*$, Cost-Loss/EVPI/CORP-PAV | `stochology_lenses::{information, time, tail, invariance, decision}` | `cargo run --release --bin labs -- --lenses` |
| §2.1–2.3 Contract v1.0 | $\Phi_1$–$\Phi_5$, Moulton-Campbell, Exceedance ICC, Chouldechova, and `EpistemologicalCard` | `stochology_core::{invariants, card}` & `cards/*.yaml` | `cargo run --release --bin labs -- --invariants` & `cargo run --release --bin gates` |
| §3.1–3.2 Funnel & SIGA | 4 Eliminatory Filters, 8-Step SIGA Roadmap, *e-processes*, and Natural Frequencies | `stochology_siga::{funnel, roadmap}` | `cargo run --release --bin labs -- --siga` |
| §4.1–4.2 Governance & Drift | Bitemporal Clock ($t_{\text{disp}}$), Canary, Single-Opening Vault, Murphy-CORP Breaker, ExTRA-WCP-T, and Label Shift (BBSE, MLLS/SLD, DFM, OTTER) | `stochology_governance::{clock, vault, drift}` | `cargo run --release --bin labs -- --governance` & `gates -- --defect clock` |

