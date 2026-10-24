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
- **Canonical Linear Benchmark:** Under $Y(\theta) = \mu + \varepsilon \theta + \xi$ and loss $\ell(z; \phi) = \frac{\gamma}{2}(y - \phi)^2 - \alpha \phi$, passive learning yields $\theta_{\mathrm{SL}} = \mu + \frac{\alpha}{\gamma}$, retraining converges to $\theta_{\mathrm{PS}} = \frac{\mu + \alpha / \gamma}{1 - \varepsilon}$ (reducing to $\frac{\mu}{1 - \varepsilon}$ when $\alpha = 0$), whereas the true Stackelberg optimum minimizing $\mathrm{PR}(\theta) = \frac{\gamma}{2}[(1 - \varepsilon)\theta - \mu]^2 + \frac{\gamma}{2}\sigma^2 - \alpha\theta$ is $\theta_{\mathrm{PO}} = \frac{\mu}{1 - \varepsilon} + \frac{\alpha}{\gamma(1 - \varepsilon)^2}$. Under Wasserstein-1 sensitivity $W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|_2$, $\gamma$-strong convexity, and $\beta$-smoothness, $\mathrm{RRM}$ contracts linearly if and only if $\frac{\varepsilon \beta}{\gamma} < 1$.

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

### 2.3. Epistemological Card Schema (`EpistemologicalCard`)
Every claim is serialized in YAML (`cards/*.yaml`) and validated by `EpistemologicalCard::audit()`, carrying top-level scalars (`id`, `contract_version: "1.0"`, `layer`, `uncertainty`) and five structured sections:
- `decision_maker_question`: `who_decides`, `altered_action`, and `question` (ending in `'?'`).
- `epistemological_header`: `generating_process`, `nonlinearity` ($f(\mathbb{E}[X]), \mathbb{E}[f(X)], \Delta_J, \text{median}, q_{05}, q_{95}, \mathrm{CRPS}$), `unit_and_margin` (`independent_unit`, `nominal_sample_size_n`, `weighted_cluster_mean_m_tilde`, `threshold_exceedance_icc`, `effective_sample_size_n_eff`, `unit_margin_of_error`), `noise_physics` (`stochastic_regime`, `absorbing_states`, `bridge_L3_L1_performativity`), and `causal_ladder` (`pearl_rung`, `identification_strategy`, `untestable_assumption`).
- `uncertainty_budget_and_authorship`: `variance_budget` (`inputs`, `parameters`, `covariance`, `structural_discrepancy_gp`), `dominant_event` (`description`, `probability_p`, `base_rate_floor_epsilon`, `cost_share`), `info_gap_robustness` (`critical_requirement_rc`, `robustness_horizon_alpha_hat`, `opportuneness_horizon_beta_hat`, `global_maximin_safeguard`), and `field_authorship` (`null_model`, `decision_rule`, `adversarial_reviewer`).
- `data_and_audit_contract`: `missingness_mechanism`, `seed_and_environment`, `null_model`, `preregistration_hash` (64-char SHA-256), `attempt_counter_K`, `e_process_multiplicity`, `murphy_corp_decomposition` (`total_score_brier_or_crps`, `miscalibration_REL_MCB`, `resolution_RES_DSC`, `base_uncertainty_UNC0`), and `findings` (`registered`, `used`, `unused`).
- `decision_rule_and_scope`: `event_probability_p`, `base_rate_floor_epsilon`, `protection_cost_C`, `event_loss_L`, `mitigation_efficacy_eta`, `critical_threshold_p_star`, `declared_act`, `scope_statement`, `who_pays_for_error` (Chouldechova audit), and `exit_clause`.

---

## 3. Eliminatory Funnel and 8-Step SIGA Operational Roadmap

### 3.1. The 4 Eliminatory Filters (`stochology_siga::funnel`)
Evaluated in strict sequence with immediate short-circuiting on failure:
1. **Filter 1 (Economic & Resolution):** Requires $\mathrm{EVPI} > 0$, non-degenerate $p^* = \frac{C}{\eta L} \in (0, 1)$, and $\Phi_5$ temporal resolution alignment ($R_{\text{data}} \ge R_{\text{decision}}$).
2. **Filter 2 (Cluster Independence & Causal Structure):** Requires Moulton-Campbell tail-exceedance effective sample size $n_{\mathrm{eff}}(p) \ge n_{\min}$, mapped Markov Blanket, and zero conditioning on a Berkson collider.
3. **Filter 3 (Simulation-Based Calibration & Elicitation):** Requires Simulation-Based Calibration (SBC) rank uniformity ($\chi^2 \le \chi^2_{\text{crit}}$) and SHELF quartile consistency ($L_{\text{phys}} < q_{25} < q_{50} < q_{75} < U_{\text{phys}}$).
4. **Filter 4 (Pre-Registration, Cross-Authorship & Exit):** Requires valid 64-character lowercase SHA-256 pre-registration hash, distinct modeler and verifier IDs, and an explicit numeric exit clause.

### 3.2. The 8-Step SIGA Roadmap, Evidence Processes, and Natural Frequencies (`stochology_siga::roadmap`)
- **8-Step SIGA Execution:** Executes **S**urvey the Layer & Bridge (Steps 1–2), **I**nterrogate the Decision-Maker (Steps 3–4), **G**enerate the Null & Noise Physics (Steps 5–6), and **A**ct with Threshold, Memory & Exit (Steps 7–8).
- **Anytime-Valid Evidence Process (*e-process*):** Computes $E_t = \prod_{i=1}^t L_i$ with optional Ville's inequality threshold $E_t / K \ge 1 / \alpha$.
- **Gigerenzer Natural Frequencies:** Translates prevalence, sensitivity, and false-positive rate into integer natural trees ($N = 1000$ or $10000$) to eliminate base-rate neglect.

---

## 4. Bitemporal Production Governance, Vault, and Drift Adaptation

### 4.1. Bitemporal Clock, Synthetic Canary, and Single-Opening Vault (`stochology_governance::{clock, vault}`)
- **Bitemporal Asymmetry ($t_{\text{disp}} \ge t_{\text{ev}}$):** Filters feature records strictly by system availability timestamp $t_{\text{disp}} \le \tau$ rather than event timestamp $t_{\text{ev}}$, and injects a high-cardinality synthetic canary column ($I(\text{Canary}; Y) = 0$) whose non-zero importance triggers an immediate look-ahead intrusion alarm.
- **Single-Opening Temporal Vault (`TemporalVault`):** Enforces a cryptographic SHA-256 pre-registration seal and an immutable single-opening lock (`opened_count: 0 -> 1`). Any non-positive confirmation outcome automatically downgrades the study to `ExploratoryHypothesisGenerating`, and any second opening attempt is blocked.

### 4.2. Murphy-CORP Circuit Breaker, Covariate Drift (ExTRA-WCP-T), and Label Shift (`stochology_governance::drift`)
- **Murphy-CORP Circuit Breaker:** Separates pure Covariate/Target Shift ($\Delta\mathrm{REL}_{\mathrm{CORP}} > \tau_{\mathrm{REL}}, \Delta\mathrm{RES}_{\mathrm{CORP}} \approx 0$) from Concept Drift ($\Delta\mathrm{RES}_{\mathrm{CORP}} < -\tau_{\mathrm{RES}}$), halting automated predictions under concept breakdown.
- **Covariate Drift Adaptation (`ExTRA-WCP-T`):** Fits exponential tilt density ratios $\hat{w}(x) = \exp(\hat{\theta}^\top \phi(x) - \hat{\psi})$ and computes weighted conformal intervals with Lei & Candès coverage lower bound $1 - \alpha - d_{\mathrm{TV}}(P_X, Q_X)$.
- **Label Shift Adaptation (4 Canonical Estimators):** Simultaneously estimates target class priors $q_Y$ via **BBSE** ($q = C^{-1}\mu$), **MLLS/SLD** (EM fixed point), **DFM** (characteristic-function kernel matching with condition number $\kappa(M)$), and **OTTER** (Sinkhorn-Knopp optimal transport scaling).

---

## 5. Module 7 & Lab 7 (`labs --real-data`): Transversal Real-Data Synthesis Across All 13 Empirical Datasets and 3 Canonical Cards

Module 7 (`cargo run --release --bin labs -- --real-data`) consolidates end-to-end empirical verification across all **13 real datasets (`data/*.csv`)** and **3 canonical Epistemological Cards (`cards/*.yaml`)**:

| Real Dataset (`data/*.csv`) | Empirical Domain & Sample ($n$) | Audited Bridge / Lens / Invariant | Applied Equation / Mechanism | Audited Empirical Benchmark (`labs --real-data`) |
| --- | --- | --- | --- | --- |
| **`nsw_experimental.csv`** vs. **`cps_observational.csv`** | LaLonde job training program: Experimental (NSW, $n=445$) vs. Observational (CPS, $n=15\,992$) | **Bridge $L_1 \to L_2$** & **Invariant $\Phi_3$** (`MengAudit`, `ManskiBounds` — Lab 1, Lab 4, Lab 7) | Meng's Identity $\bar{Y}_n - \bar{Y}_N = \rho_{R,Y} \sqrt{\frac{N-n}{n}} \sigma_Y$ & Manski MNAR bounds | Selection correlation $\rho_{R,Y} = +0.1334$ reduces $n = 15\,992$ to $n_{\mathrm{eff}} = 2842.70$; Manski bounds $[5463.2,\ 13835.3]$ |
| **`bank_calls.csv`** ($\to$ `card_02`) | North American retail bank call-center 5-min arrival windows ($n = 27\,716$, $164$ days) | **Bridge $L_2 \to L_3$** (Prohibition 2) & **Invariants $\Phi_2, \Phi_5$** (`NonlinearSummary`, `ExceedanceDesignEffect` — Lab 1, Lab 4, Lab 5, Lab 7) | Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, $\mathrm{CRPS}$, and Moulton-Campbell $n_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho_I(p)}$ | $\Delta_J = +9.03 \times 10^6$, $\mathrm{OOS\_CRPS} = 9.06 \times 10^7$; at $p=0.90$, $\mathrm{DEFF} = 28.26 \implies n_{\mathrm{eff}}(0.90) = 980.7$; `card_02` act `protect` ($p = 0.145 > p^* = 0.08000$) |
| **`djia_1885_2023.csv`** ($\to$ `card_03`) | Daily historical closes of the *Dow Jones Industrial Average* ($37\,930$ trading days, 1885–2023) | **Lens 2 — Time** & **Bridge $L_3 \to L_1$** (`TimeKellyDiagnostic`, `PerformativeSystem` — Lab 1, Lab 3, Lab 5, Lab 7) | Itô variance drain $g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$, asymmetric Kelly $f^* = \frac{p}{a} - \frac{1-p}{b}$, and Stackelberg $\theta_{\mathrm{PO}}$ | $g_{\text{time}} = 0.0468 < g_{\text{ensemble}} = 0.0612$ (drain $= 0.0145$); Kelly $f^* = 0.8600$; LSI mixing $= 13$ steps; $\theta_{\mathrm{PS}} = 1.8750$ vs. $\theta_{\mathrm{PO}} = 2.0313$ ($r_{\mathrm{RRM}} = 0.24 < 1$) |
| **`danish_fire.csv`** & **`vic_elec.csv`** | Copenhagen Re fire claims ($n = 2167$) & Victoria half-hourly electricity demand ($n = 52\,608$) | **Lens 3 — Scale and Tail** (`HillTailDiagnostic` — Lab 3, Lab 7) | Hill estimator $\hat{\alpha}_{k,n} = \left(\frac{1}{k}\sum_{i=1}^k \ln \frac{X_{(n-i+1)}}{X_{(n-k)}}\right)^{-1}$ & moment regime classification | `danish_fire.csv`: $\hat{\alpha}_{\mathrm{Hill}} = 1.6009 \pm 0.1601$, `FiniteMeanInfiniteVariance` ($1 < \alpha \le 2$), $q_{99} = 26.04\text{M DKK}$, `tail_n_eff` $= 13.9$; `vic_elec.csv`: $q_{99} = 6981.5\text{ MW}$ |
| **`atlantic_storms.csv`** & **`toronto_arrests.csv`** ($\to$ `card_01`) | NOAA Atlantic storm tracks ($n = 20\,778$) & Toronto police arrest records ($n = 5226$) | **Lens 5 — Decision** (`CorpPavDecomposition`) & **Invariant $\Phi_4$** (`ChouldechovaAudit` — Lab 3, Lab 4, Lab 5, Lab 7) | Isotonic PAV decomposition $\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$ & Chouldechova's identity | `atlantic_storms.csv`: $\mathrm{BS} = 0.26906 = \mathrm{REL}(0.21135) - \mathrm{RES}(0.13102) + \mathrm{UNC}_0(0.18873)$ (err $= 1.53 \times 10^{-14}$); `toronto_arrests.csv`: $\mathrm{FPR}_A / \mathrm{FPR}_B = 5.13\times$ |
| **`nyc_flights_2013.csv`**, **`nyc_flights_2023.csv`**, **`nyc_flights_2013_h1.csv`**, **`nyc_flights_2023_h1.csv`**, **`houston_flights_2011.csv`** | NYC flight operations (2013 vs. 2023 + H1 out-of-time reserves) & Houston out-of-domain flights (2011) | **Governance, Temporal Vault & Drift** (`fit_covariate_drift_extra_wcp`, `TemporalVault`, `estimate_label_shift_four_methods` — Lab 6, Lab 7) | Exponential tilt *ExTRA-WCP-T*, Lei & Candès bound $1 - \alpha - d_{\mathrm{TV}}(P_X, Q_X)$, SHA-256 single-opening vault, and 4 Label Shift estimators | Lei & Candès bounds: NYC $13 \to 23 \ge 0.846$, H1 OOT $13 \to 23 \ge 0.869$, NYC $\to$ Houston OOD $\ge 0.737$; H1 Vault `OpenedApproved` ($\mathrm{BS} = 0.1783$) |

---

## 6. Architectural Mapping to Rust Crates

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
| §5 Module 7 / Lab 7 | Transversal Real-Data Synthesis across all 13 `data/*.csv` datasets and 3 `cards/*.yaml` | `stochology_cli::{load_*, build_v1_card_suite}` & `labs --real-data` | `cargo run --release --bin labs -- --real-data` |

