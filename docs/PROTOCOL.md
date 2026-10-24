# Stochology Protocol: Enriched Methodological Architecture for Judgment and Decision-Making under Uncertainty

This document establishes the consolidated, mathematically formalized specification of the **Stochology Protocol** for data architecture, stochastic modeling, causal inference, and structured inter-agent communication. It unifies operational decision-making with explicit algorithmic gates for performative prediction, decision theory under severe uncertainty, strong information contraction, tail design effects in unequal clusters, and formal production drift adaptation.

---

## 1. Deep Structural Diagnosis: Bridge Mechanics, Severe Uncertainty, and the Five Transversal Lenses

Before fitting any parameter, selecting predictive architectures, or estimating probability distributions, every technical claim, scientific hypothesis, or operational model must auditably answer three structural questions:
1. In which ontological layer can the system fail (**World $L\_1$**, **Model $L\_2$**, or **Agent $L\_3$**)?
2. Which class of uncertainty threatens inference validity (**Aleatory**, **Parametric/Structural Epistemic**, or **Knightian**)?
3. Which **severe test** — endowed with genuine empirical or logical refutation power — supports the confidence placed in the claim?

### 1.1. Dynamics of the Three Ontological Layers ($L\_1 \to L\_2 \to L\_3 \to L\_1$)

| Layer | Code | Ontological Definition | Typical Outbound Bridge Error | Formal Bridge Audit |
| --- | --- | --- | --- | --- |
| **World** | $L\_1$ | Physical, biological, or socioeconomic reality generating the data, never observed in its entirety. | **Bridge $L\_1 \to L\_2$ (Invisible Filter):** Selection, censoring, or survivorship bias. | Meng's Identity ($\rho\_{R,Y} \neq 0$), Rubin's missingness taxonomy (`MCAR`, `MAR`, `MNAR`), and Manski's non-parametric bounds. |
| **Model** | $L\_2$ | Formal, reduced, computationally simulable mathematical representation of the process. | **Bridge $L\_2 \to L\_3$ (Misreading):** Treating $\mathbb{E}[X]$ as a sufficient summary of a nonlinear payoff/loss $f(X)$. | Quantification of Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ and evaluation via strictly proper scoring rules ($\mathrm{CRPS}$, Brier). |
| **Agent** | $L\_3$ | Human decision-maker, institutional committee, or algorithmic controller acting under constraints. | **Bridge $L\_3 \to L\_1$ (Performativity):** Decisions based on the model deform the very distribution $\mathcal{D}(\theta)$ being measured. | Induced distribution map $\mathcal{D}(\theta)$, separation between Performative Stability ($\theta\_{\mathrm{PS}}$) and Performative Optimality ($\theta\_{\mathrm{PO}}$), and RRM contraction $\frac{\varepsilon\beta}{\gamma} < 1$. |

On **Bridge $L\_3 \to L\_1$**, when a model parameterized by $\theta \in \Theta$ supports consequential decisions, deploying $\theta$ causally alters the environment and induces $Z = (X, Y) \sim \mathcal{D}(\theta)$. Performance is governed by Performative Risk $\mathrm{PR}(\theta)$ and Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi)$ (Perdomo, Zrnic, Mendler-Dünner & Hardt):

$$\mathrm{PR}(\theta) \triangleq \mathbb{E}\_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \theta)], \qquad \mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}\_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$$

- **Performative Stability ($\theta\_{\mathrm{PS}}$):** $\theta\_{\mathrm{PS}} = \arg\min\_{\phi \in \Theta} \mathrm{DPR}(\theta\_{\mathrm{PS}}, \phi)$ — fixed point of Repeated Risk Minimization ($\mathrm{RRM}$).
- **Performative Optimality ($\theta\_{\mathrm{PO}}$):** $\theta\_{\mathrm{PO}} = \arg\min\_{\theta \in \Theta} \mathrm{PR}(\theta)$ — Stackelberg equilibrium anticipating the strategic reaction of the population.
- **Canonical Linear Benchmark:** Under $Y(\theta) = \mu + \varepsilon \theta + \xi$ and loss $\ell(z; \phi) = \frac{\gamma}{2}(y - \phi)^2 - \alpha \phi$, passive learning yields $\theta\_{\mathrm{SL}} = \mu + \frac{\alpha}{\gamma}$, retraining converges to $\theta\_{\mathrm{PS}} = \frac{\mu + \alpha / \gamma}{1 - \varepsilon}$ (reducing to $\frac{\mu}{1 - \varepsilon}$ when $\alpha = 0$), whereas the true Stackelberg optimum minimizing $\mathrm{PR}(\theta) = \frac{\gamma}{2}[(1 - \varepsilon)\theta - \mu]^2 + \frac{\gamma}{2}\sigma^2 - \alpha\theta$ is $\theta\_{\mathrm{PO}} = \frac{\mu}{1 - \varepsilon} + \frac{\alpha}{\gamma(1 - \varepsilon)^2}$. Under Wasserstein-1 sensitivity $W\_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \lVert\theta - \theta'\rVert\_2$, $\gamma$-strong convexity, and $\beta$-smoothness, $\mathrm{RRM}$ contracts linearly if and only if $\frac{\varepsilon \beta}{\gamma} < 1$.

### 1.2. Uncertainty Typing: Kennedy-O'Hagan Discrepancy and Info-Gap + Sniedovich Safeguard

1. **Aleatory Uncertainty (`aleatory`, $L\_1$):** Irreducible variability; demands safety margins and exposure control.
2. **Parametric Epistemic Uncertainty (`epistemic_parameter`, $L\_2$):** Reducible ignorance about finite parameters $\theta$; uses Laplace's rule of succession $\hat{p} = \frac{s+1}{n+2}$ in small samples.
3. **Structural Epistemic Uncertainty (`epistemic_specification`, $L\_2$):** Formulated via **Kennedy-O'Hagan Structural Discrepancy**:
   $$y(x) = \eta(x, \theta) + \delta(x) + \varepsilon, \qquad \delta(x) \sim \mathcal{GP}(m(x), k(x, x'))$$
   with anchoring constraints to prevent parameter pseudo-convergence and extrapolation overconfidence.
4. **Knightian Uncertainty (`knightian`, $L\_3$):** Formulated via **Info-Gap Decision Theory** (Ben-Haim) coupled with **Sniedovich's Global Maximin Safeguard** (Wald):
   $$\hat{\alpha}(q, r\_c) = \max\left\lbrace\alpha \ge 0 : \min\_{u \in \mathcal{U}(\alpha, \tilde{u})} R(q, u) \ge r\_c\right\rbrace, \qquad \hat{\beta}(q, r\_w) = \min\left\lbrace\alpha \ge 0 : \max\_{u \in \mathcal{U}(\alpha, \tilde{u})} R(q, u) \ge r\_w\right\rbrace$$
   Enforces the *zero-ing* property ($\hat{\alpha}(q, R(q, \tilde{u})) = 0$) and requires a global Wald Maximin floor outside $\mathcal{U}(\hat{\alpha}, \tilde{u})$ to prevent catastrophic collapse in "No-Man's Land".

### 1.3. Mathematical Formalism of the Five Transversal Lenses, Almost-Perfect Machine, and Authentic Model Form

1. **Lens 1 — Information (`stochology_lenses::information`):**
   - Data Processing Inequality ($I(W; Z) \le I(W; Y) \le I(W; X)$ across a 3-step Markov chain $W \to X \to Y \to Z$ via `DpiMarkovChain`) and **Strong Data Processing Inequality (SDPI)** contraction hierarchy:
     $$\eta\_{\chi^2}(W) = \rho\_m^2(X; Y) \le \eta\_{\mathrm{KL}}(W) \le \eta\_{\mathrm{TV}}(W)$$
   - Fano's Error Floor: $H(P\_e) + P\_e \ln(|\mathcal{W}| - 1) \ge H(W) - I(W; Z)$.
   - Meng's Fundamental Identity: $\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y} \times \sqrt{\frac{N-n}{n}} \times \sigma\_Y$.
2. **Lens 2 — Time (`stochology_lenses::time`):**
   - Itô Variance Drain: $g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2 < \mu = g\_{\text{ensemble}}$.
   - Optimal Asymmetric Kelly Fraction: $f^{\ast} = \frac{p}{a} - \frac{1 - p}{b}$.
   - Heavy-Tailed Mixing Time under Log-Sobolev Inequality ($\alpha\_{\mathrm{LSI}} = -\frac{1}{2}\ln \eta\_{\mathrm{KL}}$) and tail mass $H\_R$.
3. **Lens 3 — Scale and Tail (`stochology_lenses::tail`):**
   - Hill Estimator: $\hat{\alpha}\_{k,n} = \left(\frac{1}{k}\sum\_{i=1}^k \ln \frac{X\_{(n-i+1)}}{X\_{(n-k)}}\right)^{-1}$ and moment regimes ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$).
4. **Lens 4 — Structure and Invariance (`stochology_lenses::invariance`):**
   - Pearl's Causal Ladder (`1-See`, `2-Act`, `3-Imagine`), Invariant Causal Prediction ($\hat{S}^{\ast} = \bigcap\_{S:\text{invariant}} S$), environment OLS invariance penalty (`env_ols_invariance_penalties`), nonlinear ICP and NILE/IRM risk-gradient penalty over the 3-atom orthonormal Hermite basis $\Phi(z) = \bigl[z, \frac{z^2-1}{\sqrt{2}}, \frac{z^3-3z}{\sqrt{6}}\bigr]$ (`run_nonlinear_icp_nile`), 2nd-order worst-case loss, and hidden confounding threshold $\Lambda^{\ast}$.
5. **Lens 5 — Decision and Calibration (`stochology_lenses::decision`):**
   - Cost-Loss Boundary: $p > p^{\ast} \triangleq \frac{C}{\eta L}$ and Expected Value of Perfect Information ($\mathrm{EVPI}$).
   - Exact Isotonic **CORP-Murphy Decomposition** via *Pool-Adjacent-Violators* (PAV):
     $$\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$$
   - Exact Fair-Coin Calibration vs. Decision Benchmark (`ExactCoinCalibrationBenchmark`, $p\_0 = 0.50$), proving that decision agreement at a fixed threshold $p^{\ast}\_A$ does not imply calibrated probabilities when the threshold shifts to $p^{\ast}\_B$.
6. **Almost-Perfect Machine and First-Class Authentic Model Form (`stochology_lenses::machine`):**
   - **$(\varepsilon, \delta)$-Almost-Perfect Machine $M = (T, \Phi, \theta)$ (`AlmostPerfectMachine`):** Audits (`P1`) universal Barron density ($\sup\_x |M(x) - f(x)| \le \varepsilon$ and $L\_2^2 \le C\_f^2 / W$), (`P2`) informational $\delta$-sufficiency ($\Delta\_{\mathrm{DPI}} = I(W; X) - I(W; T(X)) \le \delta$ and valid SDPI hierarchy), (`P3`) Fano/minimax rate optimality ($0.25 \le \mathrm{MSE} / (\sigma^2 W / n) \le 2.5$ and $P\_e^{\min} > 0$), (`P4`) Telgarsky depth-$D$ compositional efficiency ($2^D$ affine linear regions with $2^D / (2D) \ge 4.0$), and (`P5`) atomic completeness (Gram effective dimension $d\_{\mathrm{eff}} = \frac{(\mathrm{tr} G)^2}{\lVert G\rVert\_F^2} \ge 0.80 W$ and off-diagonal coherence $< 0.15$).
   - **First-Class Authentic Model Form (`AuthenticModelForm`, $\delta(x) + \hat{S}^{\ast}$):** Pairs the surviving invariant causal core $\hat{S}^{\ast}$ with an anchored Kennedy-O'Hagan GP discrepancy $\delta(x) \sim \mathcal{GP}(0, k\_{\text{anchored}})$, explicitly named discarded spurious atoms (`DiscardedAtomRecord`), simultaneous closure of the 6 exact numerical identities (`IdentityClosureAudit`: Jensen, CORP-Murphy, Meng, SHA-256 pre-registration, Wald Maximin, and Findings accounting), and the 5-part operational authenticity criterion $A(\text{model})$ (`AuthenticityCriterionA`), enforced by Gate `G8` and `--defect machine`.
7. **Relational Ontology and Complex Systems (`stochology_lenses::relational`):**
   - Formalizes the 6-axis transdisciplinary synthesis (`RelationalComplexSystemsSynthesis`): (`Axis 1 & 4`) relational vacuity (*Śūnyatā* / RQM / OSR, $I\_{\text{isolated}} \to 0$ vs. $I\_{\text{relational}} > 0$) and *Homo clausus* vs. *Homines aperti* figuration audit (`RelationalFigurationAudit`); (`Axis 2`) Prigogine open-system dissipative entropy balance $dS = d\_i S + d\_e S \le 0$ and Maturana-Varela autopoietic organizational closure (`DissipativeAutopoieticAudit`); (`Axis 3`) heteropathic emergence (*More Is Different*) and Thomas Schelling's (1971) 2D dynamic segregation phase transition (`SchellingSegregationModel`, `SchellingPhaseTransitionAudit`); (`Axis 5`) Andrew Watson & James Lovelock's (1983) *Daisyworld* planetary cybernetic homeostasis (`DaisyworldSystem`, `DaisyworldHomeostasisAudit`); and (`Axis 6`) the 6-layer Ontological Stack (Morin, Mirowski, Arthur, Beinhocker), audited in Lab 3 (`labs --lenses`) and Gate `G8`.
   - **Audited numeric thresholds (`seed = 2026`):** Axis 1&4 — $I\_{\text{rel}} > 0.30$ nats vs. $I\_{\text{iso}} < 0.005$ nats, Moulton-Campbell $\mathrm{DEFF} = 5.81\times$, Stackelberg $\theta\_{\mathrm{PO}} > \theta\_{\mathrm{PS}}$; Axis 2 — dissipative entropy balance $d\_i S = +0.42,\ d\_e S = -0.45 \Rightarrow dS = -0.03 \le 0$, material turnover $>99.9\%$, autopoietic fidelity $>99.9\%$; Axis 3 — Schelling phase transition $\tau = 0.125 \to$ integrated ($<60\%$) vs. $\tau = 0.375 \to$ segregation cascade ($\ge 78\%$), emergent gap $\Delta\_{\text{emerg}} > 0.38$; Axis 5 — Daisyworld homeostasis $\approx 22.5 ^\circ\mathrm{C}$ ($\mathrm{MAE} < 3.0 ^\circ\mathrm{C}$ with biosphere vs. $>10 ^\circ\mathrm{C}$ dead planet), $dT\_e/dL \le 0$ in coexistence, extinction at $L = 1.60$; Axis 6 — `all_six_axes_verified == true`.
   - **6-layer Ontological Stack (substancialist vs. relational):** (1) physics/metaphysics — atomism + *svabhāva* vs. *Śūnyatā*/RQM/OSR; (2) thermodynamics/time — closed static equilibrium vs. block-universe/*Uji* + dissipative-autopoietic; (3) biology/ecology — unilateral adaptation vs. Lewontin co-evolution + Daisyworld homeostasis; (4) psychology — *Homo clausus* vs. bundle-theory/*Anattā*/*Homines aperti*; (5) sociology/emergence — methodological individualism vs. *More is Different* + duality of structure; (6) political economy/epistemology — neoclassical equilibrium vs. complexity economics + complex thought.

---

## 2. Structured Communication Contract, Prohibitions, and Boundary Invariants

### 2.1. The Four Prohibitions
- **Prohibition 1:** Ban on unstructured free prose between modules/agents; all claims transit via `EpistemologicalCard` (`cards/*.yaml`).
- **Prohibition 2:** Ban on the lone mean under nonlinearity or heavy tails; mandatory reporting of $f(\mathbb{E}[X]), \mathbb{E}[f(X)], \Delta\_J, \text{median}, q\_{05}, q\_{95}, \mathrm{CRPS}$.
- **Prohibition 3:** Ban on confusing predictive fit with causal/performative validity; mandatory audit of $\theta\_{\mathrm{PS}}$ vs. $\theta\_{\mathrm{PO}}$ and $\frac{\varepsilon\beta}{\gamma} < 1$.
- **Prohibition 4:** Ban on dogmatic extremes ($0\%$ or $100\%$, Cromwell's Rule $p \in [\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$) and omitted attempts ($K \ge 1$, *e-process* $E\_t$, and CORP-Murphy decomposition).

### 2.2. The Five Boundary Invariants ($\Phi\_1$ to $\Phi\_5$)
- **$\Phi\_1$ (Simulability):** Fixed seed (`seed = 2026`, `SplitMix64`) and executable generative null model.
- **$\Phi\_2$ (Independent Unit & Exceedance Design Effect):** Moulton-Campbell unequal-cluster correction $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2(m\_j))$, threshold exceedance ICC $\rho\_I(p)$, and $n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m} - 1)\rho\_I(p)}$.
- **$\Phi\_3$ (Severe Test & Missingness):** Rubin classification (`MCAR`, `MAR`, `MNAR`), SHA-256 pre-registration seal, and Pearl's Ladder rung.
- **$\Phi\_4$ (Scope, Fairness & Exit):** Chouldechova's algebraic impossibility identity $\mathrm{FPR}\_g = \frac{p\_g}{1-p\_g}\frac{1-\mathrm{PPV}\_g}{\mathrm{PPV}\_g}(1-\mathrm{FNR}\_g)$, scope statement, and numeric exit clause.
- **$\Phi\_5$ (Question Resolution):** Ban on answering daily/shift operational decisions with coarse annual/monthly aggregates.

### 2.3. Epistemological Card Schema (`EpistemologicalCard`)
Every claim is serialized in YAML (`cards/*.yaml`) and validated by `EpistemologicalCard::audit()`, carrying top-level scalars (`id`, `layer`, `uncertainty`) and five structured sections:
- `decision_maker_question`: `who_decides`, `altered_action`, and `question` (ending in `'?'`).
- `epistemological_header`: `generating_process`, `nonlinearity` ($f(\mathbb{E}[X]), \mathbb{E}[f(X)], \Delta\_J, \text{median}, q\_{05}, q\_{95}, \mathrm{CRPS}$), `unit_and_margin` (`independent_unit`, `nominal_sample_size_n`, `weighted_cluster_mean_m_tilde`, `threshold_exceedance_icc`, `effective_sample_size_n_eff`, `unit_margin_of_error`), `noise_physics` (`stochastic_regime`, `absorbing_states`, `bridge_L3_L1_performativity`), and `causal_ladder` (`pearl_rung`, `identification_strategy`, `untestable_assumption`).
- `uncertainty_budget_and_authorship`: `variance_budget` (`inputs`, `parameters`, `covariance`, `structural_discrepancy_gp`), `dominant_event` (`description`, `probability_p`, `base_rate_floor_epsilon`, `cost_share`), `info_gap_robustness` (`critical_requirement_rc`, `robustness_horizon_alpha_hat`, `opportuneness_horizon_beta_hat`, `global_maximin_safeguard`), and `field_authorship` (`null_model`, `decision_rule`, `adversarial_reviewer`).
- `data_and_audit_contract`: `dataset_path`, `dataset_sha256` (64-char SHA-256 verified against `data/SHA256SUMS`), `missingness_mechanism`, `seed_and_environment`, `null_model`, `preregistration_hash` (64-char SHA-256 over pre-analysis design payload), `attempt_counter_K`, `e_process_multiplicity`, `murphy_corp_decomposition` (`total_score_brier_or_crps`, `miscalibration_REL_MCB`, `resolution_RES_DSC`, `base_uncertainty_UNC0`), and `findings` (`registered`, `used`, `unused`).
- `decision_rule_and_scope`: `event_probability_p`, `base_rate_floor_epsilon`, `protection_cost_C`, `event_loss_L`, `mitigation_efficacy_eta`, `critical_threshold_p_star`, `declared_act`, `scope_statement`, `who_pays_for_error` (Chouldechova audit), and `exit_clause`.

---

## 3. Eliminatory Funnel and 8-Step SIGA Operational Roadmap

### 3.1. The 4 Eliminatory Filters (`stochology_siga::funnel`)
Evaluated in strict sequence with immediate short-circuiting on failure:
1. **Filter 1 (Economic & Resolution):** Requires $\mathrm{EVPI} > C\_{\text{total}} \ge 0$, non-degenerate $p^\ast  = \frac{C}{\eta L} \in (0, 1)$, and $\Phi\_5$ temporal resolution alignment ($R\_{\text{data}} \ge R\_{\text{decision}}$).
2. **Filter 2 (Cluster Independence & Causal Structure):** Requires Moulton-Campbell tail-exceedance effective sample size $n\_{\mathrm{eff}}(p) \ge n\_{\min}$, mapped Markov Blanket, and zero conditioning on a Berkson collider.
3. **Filter 3 (Simulation-Based Calibration & Elicitation):** Requires Simulation-Based Calibration (SBC) rank uniformity ($\chi^2 \le \chi^2\_{\text{crit}}$) and SHELF quartile consistency ($L\_{\text{phys}} < q\_{25} < q\_{50} < q\_{75} < U\_{\text{phys}}$).
4. **Filter 4 (Pre-Registration, Cross-Authorship & Exit):** Requires valid 64-character lowercase SHA-256 pre-registration hash, distinct modeler and verifier IDs, and an explicit numeric exit clause.

### 3.2. The 8-Step SIGA Roadmap, Evidence Processes, and Natural Frequencies (`stochology_siga::roadmap`)
- **8-Step SIGA Execution:** Executes **S**urvey the Layer & Bridge (Steps 1–2), **I**nterrogate the Decision-Maker (Steps 3–4), **G**enerate the Null & Noise Physics (Steps 5–6), and **A**ct with Threshold, Memory & Exit (Steps 7–8).
- **Anytime-Valid Evidence Process (*e-process*):** Computes $E\_t = \prod\_{i=1}^t L\_i$ with optional Ville's inequality threshold $E\_t / K \ge 1 / \alpha$.
- **Gigerenzer Natural Frequencies:** Translates prevalence, sensitivity, and false-positive rate into integer natural trees ($N = 1000$ or $10000$) to eliminate base-rate neglect.

---

## 4. Bitemporal Production Governance, Vault, and Drift Adaptation

### 4.1. Bitemporal Clock, Synthetic Canary, and Single-Opening Vault (`stochology_governance::{clock, vault}`)
- **Bitemporal Asymmetry ($t\_{\text{disp}} \ge t\_{\text{ev}}$):** Filters feature records strictly by system availability timestamp $t\_{\text{disp}} \le \tau$ rather than event timestamp $t\_{\text{ev}}$, and injects a high-cardinality synthetic canary column ($I(\text{Canary}; Y) = 0$) whose non-zero importance triggers an immediate look-ahead intrusion alarm.
- **Single-Opening Temporal Vault (`TemporalVault`):** Enforces a cryptographic SHA-256 pre-registration seal and an immutable single-opening lock (`opened_count: 0 -> 1`). Any non-positive confirmation outcome automatically downgrades the study to `ExploratoryHypothesisGenerating`, and any second opening attempt is blocked.

### 4.2. Murphy-CORP Circuit Breaker, Covariate Drift (ExTRA-WCP-T), and Label Shift (`stochology_governance::drift`)
- **Murphy-CORP Circuit Breaker:** Separates pure Covariate/Target Shift ($\Delta\mathrm{REL}\_{\mathrm{CORP}} > \tau\_{\mathrm{REL}}, \Delta\mathrm{RES}\_{\mathrm{CORP}} \approx 0$) from Concept Drift ($\Delta\mathrm{RES}\_{\mathrm{CORP}} < -\tau\_{\mathrm{RES}}$), halting automated predictions under concept breakdown.
- **Covariate Drift Adaptation (`ExTRA-WCP-T`):** Fits exponential tilt density ratios $\hat{w}(x) = \exp(\hat{\theta}^\top \phi(x) - \hat{\psi})$ and computes weighted conformal intervals with Lei & Candès coverage lower bound $1 - \alpha - d\_{\mathrm{TV}}(P\_X, Q\_X)$.
- **Label Shift Adaptation (4 Canonical Estimators):** Simultaneously estimates target class priors $q\_Y$ via **BBSE** ($q = C^{-1}\mu$), **MLLS/SLD** (EM fixed point), **DFM** (characteristic-function kernel matching with condition number $\kappa(M)$), and **OTTER** (Sinkhorn-Knopp optimal transport scaling).

---

## 5. Module 7 & Lab 7 (`labs --real-data`): Transversal Real-Data Synthesis Across All 13 Empirical Datasets and 3 Canonical Cards

Module 7 (`cargo run --release --bin labs -- --real-data`) consolidates end-to-end empirical verification across all **13 real datasets (`data/*.csv`)** and **3 canonical Epistemological Cards (`cards/*.yaml`)**:

| Real Dataset (`data/*.csv`) | Empirical Domain & Sample ($n$) | Audited Bridge / Lens / Invariant | Applied Equation / Mechanism | Audited Empirical Benchmark (`labs --real-data`) |
| --- | --- | --- | --- | --- |
| **`nsw_experimental.csv`** vs. **`cps_observational.csv`** | LaLonde job training program: Experimental (NSW, $n=445$) vs. Observational (CPS, $n=15\text{,}992$) | **Bridge $L\_1 \to L\_2$** & **Invariant $\Phi\_3$** (`MengAudit`, `ManskiBounds` — Lab 1, Lab 4, Lab 7) | Meng's Identity $\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y} \sqrt{\frac{N-n}{n}} \sigma\_Y$ & Manski MNAR bounds | Selection correlation $\rho\_{R,Y} = +0.1334$ reduces $n = 15\text{,}992$ to $n\_{\mathrm{eff}} = 2842.70$; Manski bounds $[5463.2,\ 13835.3]$ |
| **`bank_calls.csv`** ($\to$ `card_02`) | North American retail bank call-center 5-min arrival windows ($n = 27\text{,}716$, $164$ days) | **Bridge $L\_2 \to L\_3$** (Prohibition 2) & **Invariants $\Phi\_2, \Phi\_5$** (`NonlinearSummary`, `ExceedanceDesignEffect` — Lab 1, Lab 4, Lab 5, Lab 7) | Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, $\mathrm{CRPS}$, and Moulton-Campbell $n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho\_I(p)}$ | $\Delta\_J = +9.03 \times 10^6$, $\mathrm{OOS\_CRPS} = 9.06 \times 10^7$; at $p=0.90$, $\mathrm{DEFF} = 28.26 \implies n\_{\mathrm{eff}}(0.90) = 980.7$; `card_02` act `protect` ($p = 0.09911 > p^{\ast} = 0.08000$) |
| **`djia_1885_2023.csv`** | Daily historical closes of the *Dow Jones Industrial Average* ($37\text{,}930$ trading days, 1885–2023) | **Lens 2 — Time** & **Bridge $L\_3 \to L\_1$** (`TimeKellyDiagnostic`, `PerformativeSystem` — Lab 1, Lab 3, Lab 7) | Itô variance drain $g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2$, asymmetric Kelly $f^{\ast} = \frac{p}{a} - \frac{1-p}{b}$, and Stackelberg $\theta\_{\mathrm{PO}}$ | $g\_{\text{time}} = 0.0468 < g\_{\text{ensemble}} = 0.0612$ (drain $= 0.0145$); Kelly $f^{\ast} = 0.8600$; LSI mixing $= 13$ steps; $\theta\_{\mathrm{PS}} = 1.8750$ vs. $\theta\_{\mathrm{PO}} = 2.0313$ ($r\_{\mathrm{RRM}} = 0.24 < 1$) |
| **`danish_fire.csv`** & **`vic_elec.csv`** | Copenhagen Re fire claims ($n = 2167$) & Victoria half-hourly electricity demand ($n = 52\text{,}608$) | **Lens 3 — Scale and Tail** (`HillTailDiagnostic` — Lab 3, Lab 7) | Hill estimator $\hat{\alpha}\_{k,n} = \left(\frac{1}{k}\sum\_{i=1}^k \ln \frac{X\_{(n-i+1)}}{X\_{(n-k)}}\right)^{-1}$ & moment regime classification | `danish_fire.csv`: $\hat{\alpha}\_{\mathrm{Hill}} = 1.6009 \pm 0.1601$, `FiniteMeanInfiniteVariance` ($1 < \alpha \le 2$), $q\_{99} = 26.04\text{M DKK}$, `tail_n_eff` $= 13.9$; `vic_elec.csv`: $q\_{99} = 6981.5\text{ MW}$ |
| **`atlantic_storms.csv`** ($\to$ `card_01`) & **`toronto_arrests.csv`** | NOAA Atlantic storm tracks ($n = 20\text{,}778$) & Toronto police arrest records ($n = 5226$) | **Lenses 4 & 5** (`run_nonlinear_icp_nile`, `AuthenticModelForm`, `CorpPavDecomposition`) & **Invariant $\Phi\_4$** (`ChouldechovaAudit` — Lab 3, Lab 4, Lab 5, Lab 7, Gate G8) | Nonlinear NILE/IRM over Hermite basis $\Phi$, isotonic PAV decomposition $\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$, & Chouldechova's identity | `atlantic_storms.csv`: $\hat{S}^{\ast} = [0]$ (`ratio` $= 0.0103$, $A = \text{true}$), Lab 7 64-kt $\mathrm{BS} = 0.26906 = \mathrm{REL}(0.21135) - \mathrm{RES}(0.13102) + \mathrm{UNC}\_0(0.18873)$, `card_01` 85-kt $\mathrm{BS} = 0.03210 = \mathrm{REL}(0.00578) - \mathrm{RES}(0.06183) + \mathrm{UNC}\_0(0.08815)$; `toronto_arrests.csv`: $\mathrm{FPR}\_A / \mathrm{FPR}\_B = 5.13\times$ |
| **`nyc_flights_2013.csv`** ($\to$ `card_03`), **`nyc_flights_2023.csv`**, **`nyc_flights_2013_h1.csv`**, **`nyc_flights_2023_h1.csv`**, **`houston_flights_2011.csv`** | NYC flight operations (2013 vs. 2023 + H1 out-of-time reserves) & Houston out-of-domain flights (2011) | **Governance, Temporal Vault & Drift** (`fit_covariate_drift_extra_wcp`, `TemporalVault`, `estimate_label_shift_four_methods` — Lab 5, Lab 6, Lab 7) | Exponential tilt *ExTRA-WCP-T*, Lei & Candès bound $1 - \alpha - d\_{\mathrm{TV}}(P\_X, Q\_X)$, SHA-256 single-opening vault, and 4 Label Shift estimators | `card_03` ($n=138\text{,}375, n\_{\mathrm{eff}}=1364$); Lei & Candès bounds: NYC $13 \to 23 \ge 0.802$, H1 OOT $13 \to 23 \ge 0.828$, NYC $\to$ Houston OOD $\ge 0.693$; H1 Vault `OpenedApproved` ($\mathrm{BS} = 0.1783$) |

---

## 6. Architectural Mapping to Rust Crates

| Protocol Section | Concept / Equation | Rust Module & Symbol | Verification Command |
| --- | --- | --- | --- |
| §1.1 Bridge $L\_1 \to L\_2$ | Meng's Identity ($\rho\_{R,Y}$) and Manski MNAR Bounds | `stochology_core::layers::{MengAudit, ManskiBounds}` | `cargo run --release --bin labs -- --layers` |
| §1.1 Bridge $L\_2 \to L\_3$ | Jensen's Gap $\Delta\_J$ and empirical $\mathrm{CRPS}$ | `stochology_core::layers::NonlinearSummary` | `cargo run --release --bin labs -- --layers` |
| §1.1 Bridge $L\_3 \to L\_1$ | Performative Prediction ($\theta\_{\mathrm{SL}}, \theta\_{\mathrm{PS}}, \theta\_{\mathrm{PO}}$) and RRM contraction | `stochology_core::layers::PerformativeSystem` | `cargo run --release --bin labs -- --layers` & `gates -- --defect performativity` |
| §1.2 Structural & Knightian | Kennedy-O'Hagan GP $\delta(x)$ and Info-Gap + Sniedovich Safeguard | `stochology_core::uncertainty::{KennedyOHaganDiscrepancy, InfoGapOption}` | `cargo run --release --bin labs -- --uncertainty` & `gates -- --defect sniedovich` |
| §1.3 Lenses 1–5 | SDPI/DPI/Fano, Itô/Kelly/Mixing, Hill, Pearl/ICP/`env_ols_invariance_penalties`/`run_nonlinear_icp_nile`/$\Lambda^\ast $, Cost-Loss/EVPI/CORP-PAV/`ExactCoinCalibrationBenchmark` | `stochology_lenses::{information, time, tail, invariance, decision}` | `cargo run --release --bin labs -- --lenses` |
| §1.3 Almost-Perfect Machine & Authentic Form | $M = (T, \Phi, \theta)$ (`P1`–`P5`), Barron rate $C\_f^2/W$, Telgarsky $2^D$, Gram $d\_{\mathrm{eff}}$, $\delta(x) + \hat{S}^\ast $, named discarded atoms, and 6 identities | `stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm}` | `cargo run --release --bin gates` (G8) & `gates -- --defect machine` |
| §2.1–2.3 Contract | $\Phi\_1$–$\Phi\_5$, Moulton-Campbell, Exceedance ICC, Chouldechova, and `EpistemologicalCard` | `stochology_core::{invariants, card}` & `cards/*.yaml` | `cargo run --release --bin labs -- --invariants` & `cargo run --release --bin gates` |
| §3.1–3.2 Funnel & SIGA | 4 Eliminatory Filters, 8-Step SIGA Roadmap, *e-processes*, and Natural Frequencies | `stochology_siga::{funnel, roadmap}` | `cargo run --release --bin labs -- --siga` |
| §4.1–4.2 Governance & Drift | Bitemporal Clock ($t\_{\text{disp}}$), Canary, Single-Opening Vault, Murphy-CORP Breaker, ExTRA-WCP-T, and Label Shift (BBSE, MLLS/SLD, DFM, OTTER) | `stochology_governance::{clock, vault, drift}` | `cargo run --release --bin labs -- --governance` & `gates -- --defect clock` |
| §5 Module 7 / Lab 7 | Transversal Real-Data Synthesis across all 13 `data/*.csv` datasets and 3 `cards/*.yaml` | `stochology_cli::{load_*, evaluate_empirical_card_severe_test, verify_card_against_empirical_data}` & `labs --real-data` | `cargo run --release --bin labs -- --real-data` |

