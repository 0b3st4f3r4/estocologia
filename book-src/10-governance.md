# 10. Bitemporal Governance, Vault, and the Four Regimes of Drift

> **Learning Objectives.** By the end of this chapter, you will be able to explain why models that pass academic backtests fail in live production through look-ahead leakage and distribution drift; enforce **Bitemporal Asymmetry ($t\_{\text{disp}} \ge t\_{\text{ev}}$)** and detect temporal intrusion via a **Synthetic Canary Column ($I(\text{Canary}; Y) = 0$)**; operate the **Single-Opening Temporal Vault (`TemporalVault`)**; distinguish the **Four Regimes of Production Drift** (Stationary, Covariate Drift, Label Shift, and Concept Drift) using the **Murphy-CORP Circuit Breaker**; adapt to Covariate Drift via **Exponential-Tilt Weighted Conformal Prediction (`ExTRA-WCP-T`)** with **Lei & Candès's coverage bound ($1 - \alpha - d\_{\mathrm{TV}}$)**; and estimate target priors under Label Shift via the **4 Canonical Estimators (`BBSE`, `MLLS/SLD`, `DFM`, `OTTER`)**.

---

## Step 1 — Intuition and the Real-World Paradox

### The Time-Traveler Backtest and the Overused Vault

Why do so many quantitative trading strategies, hospital early-warning scores, and flight-delay predictors boast $95\%$ accuracy in the research notebook and crash during their first week in live production?

Two structural failures explain almost every production collapse:

#### 1. The Time-Traveler in the Database (Bitemporal Leakage)

Suppose on **May 1st** an economist wants to train a model to predict whether the stock market will rise on **May 2nd**, using "April GDP growth" as a feature. In the historical SQL table downloaded three years later, the row labeled `event_date = 2023-04-30` contains the final revised April GDP number. The model uses that number to predict May 2nd and achieves spectacular accuracy!

Why did the model fail when deployed live on May 1st? Because in the real world, the statistical bureau does not publish the first estimate of April GDP until **May 28th**, and does not publish the final revised number until **July**! On May 1st, that number did not exist in the universe yet.

Every datum in an operational system actually has **two timestamps**:
- **Event Time ($t\_{\text{ev}}$):** When the physical event occurred in the World ($L\_1$).
- **Availability / Ingestion Time ($t\_{\text{disp}}$):** When the record was actually written, finalized, and available to the Agent ($L\_3$) in the computer system ($t\_{\text{disp}} \ge t\_{\text{ev}}$).

Filtering a training set by $t\_{\text{ev}} \le \tau$ instead of $t\_{\text{disp}} \le \tau$ secretly grants your model a time machine.

#### 2. Peeking at the Vault Until It Confesses

Even when timestamps are clean, suppose a research team reserves the last three months of 2023 as a "holdout test set."
- They run Model 1 on the test set; it fails.
- They tweak the learning rate and feature set (Model 2) and run it on the same test set; it still fails.
- After $15$ rounds of adjustments, Model 15 passes on the test set, and the team publishes that test score as an "out-of-sample confirmation."

As Cynthia Dwork and colleagues (2015) proved in adaptive data analysis, **every time you peek at a holdout set and use the result to choose your next modification, information leaks from the holdout set into your model.** After enough openings, the "test set" is just a second training set in disguise.

And once the model is finally live in production, the physical world itself begins to drift: passenger traffic shifts from 2013 to 2023 (`nyc_flights_2013.csv` $\to$ `nyc_flights_2023.csv`), a pandemic alters hospital baseline prevalence, or a regulatory change breaks the link between features and outcomes.

`crates/stochology-governance` builds the three defensive shields that protect live operations: **the Bitemporal Clock & Canary (`clock.rs`)**, **the Single-Opening Temporal Vault (`vault.rs`)**, and **Multi-Regime Drift Adaptation & Circuit Breaking (`drift.rs`)**.

---

## Step 2 — Foundations and Mathematical Formalization

### 10.1 Bitemporal Clock ($t\_{\text{disp}} \ge t\_{\text{ev}}$) and the Synthetic Canary (`stochology_governance::clock`)

#### Enforcing Physical Causality in Data Pipelines

Every feature record entering `BitemporalClock` carries $(t\_{\text{ev}}, t\_{\text{disp}})$ with the physical invariant $t\_{\text{disp}} \ge t\_{\text{ev}}$ (a database cannot record a measurement before the event happens, and network/reporting latency $\Delta\_{\text{lat}} = t\_{\text{disp}} - t\_{\text{ev}} \ge 0$ is non-negative).

When an agent makes a decision at operational wall-clock time $\tau$, the Bitemporal Clock admits a record **if and only if**:

$$t\_{\text{disp}} \le \tau$$

Any record with $t\_{\text{ev}} \le \tau < t\_{\text{disp}}$ (an event that already happened in the physical world, but whose measurement has not yet arrived in the database at time $\tau$) is strictly quarantined from the feature vector.

#### The Synthetic Canary Column ($I(\text{Canary}; Y) = 0$)

How can an automated gate prove that a complex feature-engineering pipeline has not accidentally leaked future target information $Y\_{t + h}$ into time $\tau$ through a buggy rolling window, a global normalization (`StandardScaler` fitted over the whole table), or a target-encoded join?

Borrowing the metaphor of the coal-mine canary, `BitemporalClock` injects a **Synthetic Canary Column** $C\_{\text{canary}}$ into the feature table:
- For every causal record properly available at $t\_{\text{disp}} \le \tau$, $C\_{\text{canary}}$ is drawn from independent `SplitMix64` noise that has **zero mutual information with the target $Y$**:
  $$I(C\_{\text{canary}}; Y) = 0$$
- However, inside the forbidden future horizon $t\_{\text{disp}} > \tau$, the canary is coupled to the future realization of $Y$.
- If the feature pipeline obeys strict bitemporal causality ($t\_{\text{disp}} \le \tau$), the model sees only the pre-$\tau$ canary noise, so empirical mutual information stays at $I(C\_{\text{canary}}; Y) \approx 0.0000$.
- The instant a buggy join, backfill, or look-ahead window reaches across $\tau$ into $t\_{\text{disp}} > \tau$, the canary's mutual information spikes ($I(C\_{\text{canary}}; Y) = 0.6931\text{ nats} > \tau\_{\text{canary}}$), tripping an immediate **Temporal Intrusion Alarm** (`Gate G7` and `--defect clock`)!

---

### 10.2 The Single-Opening Temporal Vault (`stochology_governance::vault`)

To prevent adaptive overfitting on holdout data, `TemporalVault` wraps out-of-time (`nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`) and out-of-domain (`houston_flights_2011.csv`) evaluation reserves inside a state machine governed by three irreversible rules:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1100" height="370" viewBox="0 0 1100 370" fill="none" role="img" aria-labelledby="title-fig-10-vault">
  <title id="title-fig-10-vault">State Machine of the Single-Opening TemporalVault</title>
  <defs>
  <marker id="v-sky" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="v-emerald" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--emerald)"/>
  </marker>
  <marker id="v-amber" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--amber)"/>
  </marker>
  <marker id="v-rose" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--rose)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1098" height="368" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="250" y="18" width="540" height="70" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.4"/>
  <text x="520" y="47" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">VaultSealed (opened_count = 0)</text>
  <text x="520" y="70" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">64-Hex SHA-256 Pre-Registration Seal Verified</text>
  <line x1="520" y1="88" x2="520" y2="128" stroke="var(--sky)" stroke-width="2.6" marker-end="url(#v-sky)"/>
  <text x="506" y="114" text-anchor="end" fill="var(--sky)" class="mono" font-size="12" font-weight="800">1st call: open_once()</text>
  <rect x="250" y="134" width="540" height="64" rx="10" fill="var(--surface)" stroke="var(--border)" stroke-width="2"/>
  <text x="520" y="161" text-anchor="middle" fill="var(--ink)" font-size="14" font-weight="800">Evaluate Pre-Registered Rule (opened_count = 1)</text>
  <text x="520" y="182" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Compare Out-of-Time Brier Score against Target</text>
  <line x1="395" y1="198" x2="276" y2="248" stroke="var(--emerald)" stroke-width="2.4" marker-end="url(#v-emerald)"/>
  <text x="315" y="224" text-anchor="end" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Score ≤ Target</text>
  <line x1="645" y1="198" x2="804" y2="248" stroke="var(--amber)" stroke-width="2.4" marker-end="url(#v-amber)"/>
  <text x="745" y="224" text-anchor="start" fill="var(--amber)" class="mono" font-size="11.5" font-weight="800">Score &gt; Target</text>
  <rect x="36" y="254" width="480" height="94" rx="12" fill="var(--emerald-bg)" stroke="var(--emerald)" stroke-width="2.4"/>
  <text x="276" y="285" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">OpenedApproved</text>
  <text x="276" y="309" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">Status: Confirmatory Pre-Registered OK</text>
  <text x="276" y="331" text-anchor="middle" fill="var(--muted)" font-size="11.5">Holdout generalization verified without tuning</text>
  <rect x="564" y="254" width="480" height="94" rx="12" fill="var(--amber-bg)" stroke="var(--amber)" stroke-width="2.4"/>
  <text x="804" y="285" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">DowngradedToExploratory</text>
  <text x="804" y="309" text-anchor="middle" fill="var(--amber)" class="mono" font-size="11.5" font-weight="700">Status: ExploratoryHypothesisGenerating</text>
  <text x="804" y="331" text-anchor="middle" fill="var(--muted)" font-size="11.5">Requires brand-new untouched future vault</text>
  <rect x="820" y="102" width="256" height="68" rx="10" fill="var(--rose-bg)" stroke="var(--rose)" stroke-width="2"/>
  <text x="948" y="130" text-anchor="middle" fill="var(--rose)" class="mono" font-size="11.5" font-weight="800">ANY 2ND open_once()</text>
  <text x="948" y="152" text-anchor="middle" fill="var(--ink)" class="mono" font-size="11" font-weight="700">VaultReopenError (G7)</text>
  <line x1="790" y1="154" x2="814" y2="142" stroke="var(--rose)" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#v-rose)"/>
</svg>
  <figcaption><strong>Figure 10.1 — State Machine of the Single-Opening Temporal Vault</strong></figcaption>
</figure>

1. **Cryptographic Pre-Registration Seal and Keyed `HMAC-SHA256` Authentication:** A vault can be constructed only with a valid 64-character lowercase hexadecimal SHA-256 hash sealing the model weights, feature pipeline, and acceptance threshold *before* the vault is touched, paired with an **RFC 2104 Keyed `HMAC-SHA256` authentication tag** (`hmac_sha256_hex` in `crates/stochology-core/src/sha256.rs`). On `open_once()`, `TemporalVault` recomputes the SHA-256 digest of the stored outcomes and pre-registration ID, immediately returning `Err(VaultError::SealTampered)` if a single bit was mutated after sealing.
2. **Immutable Single-Opening Lock (`opened_count: 0 -> 1`):** Calling `open_once()` increments `opened_count` from `0` to `1`. Any subsequent attempt to call `open_once()` on the same vault is immediately blocked (`--defect vault`).
3. **Automatic Epistemological Downgrade on Failure:** If the model's out-of-time Brier score or calibration error violates the pre-registered threshold upon opening, the study is **automatically and permanently downgraded** to `ExploratoryHypothesisGenerating`. The team may use what they learned to formulate a *new* hypothesis, but that new hypothesis must be tested on a **brand-new, untouched future vault**!

---

### 10.3 The Four Regimes of Production Drift and the Murphy-CORP Circuit Breaker

#### Foundations: Factoring the Joint Distribution $P(X, Y)$

Once a model is deployed in live operations, the joint distribution of features $X$ and outcomes $Y$ can change from the source training distribution $P(X, Y)$ to a new target production distribution $Q(X, Y)$.

By the product rule of probability, $P(X, Y)$ can be factored in two ways:
- **Causal / Covariate Factoring:** $P(X, Y) = P(X)  P(Y \mid X)$
- **Anti-Causal / Label Factoring:** $P(X, Y) = P(Y)  P(X \mid Y)$

This gives rise to **Four Distinct Regimes of Production Drift**, each requiring a completely different mathematical response:

| Regime | What Changes vs. What Stays Invariant | Physical Example | Protocol Action (`stochology_governance::drift`) |
| --- | --- | --- | --- |
| **1. Stationary** | $Q(X, Y) = P(X, Y)$ | Normal daily operation within expected variability. | Continue normal operation; monitor CORP components. |
| **2. Covariate Drift** | Covariates shift ($Q(X) \neq P(X)$), while causal mechanism stays invariant ($Q(Y \mid X) = P(Y \mid X)$). | Departure-hour schedule mix shifts between NYC 2013 and NYC 2023 flights. | Adapt via **Exponential-Tilt Weighted Conformal Prediction (`ExTRA-WCP-T`)** and verify **Lei & Candès bound**. |
| **3. Label / Target Shift** | Base rate shifts ($Q(Y) \neq P(Y)$), while symptom profile given state stays invariant ($Q(X \mid Y) = P(X \mid Y)$). | A winter flu wave doubles disease prevalence $q\_Y$, while fever given flu stays the same. | Re-estimate target prior $q\_Y$ without labels via the **4 Canonical Estimators (`BBSE`, `MLLS`, `DFM`, `OTTER`)** and apply Bayes prior-odds update. |
| **4. Concept Drift** | The fundamental conditional law breaks ($Q(Y \mid X) \neq P(Y \mid X)$ and $Q(X \mid Y) \neq P(X \mid Y)$). | A new air-traffic control regulation or structural market break invalidates the model's features. | **HALT AUTOMATED PREDICTIONS IMMEDIATELY** via the **Murphy-CORP Circuit Breaker** and trigger the card's `exit_clause`! |

#### The Murphy-CORP Circuit Breaker

How can an automated monitor tell the difference between a repairable shift (Regimes 2 & 3) and fatal **Concept Drift** (Regime 4)?
Look back at the PAV isotonic decomposition from Lens 5 (Chapter 3):

$$\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$$

- Under pure **Label Shift** or monotone **Covariate Shift**, the overall base rate or marginal scale moves, causing **Miscalibration to rise ($\Delta \mathrm{REL}\_{\mathrm{CORP}} > \tau\_{\mathrm{REL}}$)**, while the model's ability to rank high-risk cases above low-risk cases (**Resolution, $\mathrm{RES}\_{\mathrm{CORP}}$**) remains intact ($\Delta \mathrm{RES}\_{\mathrm{CORP}} \approx 0$)! This shift can be repaired mathematically without throwing away the model.
- Under **Concept Drift**, the features $X$ lose their causal link to $Y$: the model's ranking power collapses, causing **Resolution to plunge ($\Delta \mathrm{RES}\_{\mathrm{CORP}} < -\tau\_{\mathrm{RES}}$)**! No recalibration or reweighting can resurrect destroyed resolution. When $\Delta \mathrm{RES}\_{\mathrm{CORP}} < -\tau\_{\mathrm{RES}}$, `MurphyCorpCircuitBreaker` trips (`tripped = true`), halts automated action, and forces fallback to the conservative Wald Maximin policy (`--defect concept_drift`).

---

### 10.4 Adapting to Covariate Drift: `ExTRA-WCP-T` and the Lei & Candès Bound

When covariates shift ($Q\_X \neq P\_X$) while $P(Y \mid X)$ remains invariant, standard conformal prediction intervals lose their coverage guarantee because test points are drawn from $Q\_X$ rather than $P\_X$.

Ryan Tibshirani, Rina Foygel Barber, Emmanuel Candès, and Aaditya Ramdas (2019) and **Lihua Lei and Emmanuel Candès (2021)** showed that if we weight each calibration conformity score $V\_i = |Y\_i - \hat{\mu}(X\_i)|$ by the **density ratio** $w(x) = \frac{dQ\_X}{dP\_X}(x)$, exact finite-sample validity is restored!

In `fit_covariate_drift_extra_wcp` (`crates/stochology-governance/src/drift.rs`), we implement **Exponential-Tilt Weighted Conformal Prediction (`ExTRA-WCP-T`)**:
1. **Exponential-Tilt Density Ratio Estimation:** Rather than dividing two unstable high-dimensional density histograms, we model the log-density ratio linearly in a feature map $\phi(x)$:
   $$\hat{w}(x) = \exp\bigl(\hat{\theta}^\top \phi(x) - \hat{\psi}\bigr)$$
   fitting $\hat{\theta}$ by minimizing the convex Bregman / logistic classification objective between source samples $X \sim P\_X$ and target samples $X' \sim Q\_X$, and normalizing so that $\hat{Z}\_P = \frac{1}{n}\sum\_{i=1}^n \hat{w}(X\_i) \approx 1$.
2. **Weighted Conformal Quantile:** Given normalized weights $p\_i(x) = \frac{\hat{w}(X\_i)}{\sum\_{j=1}^n \hat{w}(X\_j) + \hat{w}(x)}$, `ExTRA-WCP-T` computes the $(1-\alpha)$ weighted quantile of the conformity scores.
3. **Lei & Candès (2021) Finite-Sample Coverage Floor:** Even when the density ratio $\hat{w}(x)$ is estimated imperfectly, Lei and Candès proved that actual target coverage under $Q$ satisfies the exact lower bound:
   $$P\_{Q}\bigl(Y\_{n+1} \in \widehat{C}(X\_{n+1})\bigr) \ge 1 - \alpha - d\_{\mathrm{TV}}(P\_X, Q\_X)$$
   where $d\_{\mathrm{TV}}(P\_X, Q\_X) = \frac{1}{2}\int |dP\_X - dQ\_X|$ is the Total Variation distance between source and target covariates.
4. **Gibbs & Candès (2021) Online Adaptive Conformal Inference (`AdaptiveConformalInference`):** What if the operational data stream is completely non-exchangeable—undergoing continuous, unmodeled time-series regime shifts where no static density ratio $w(x)$ suffices? In `AdaptiveConformalInference::evaluate_stream`, the effective miscoverage quantile level $\alpha\_t$ is updated online after each observed nonconformity score $S\_t$ via step size $\gamma > 0$:
   $$\alpha\_{t+1} = \mathrm{clamp}\Bigl(\alpha\_t + \gamma\bigl(\alpha - \mathrm{err}\_t\bigr),\ 0.001,\ 0.999\Bigr), \qquad \mathrm{err}\_t = \mathbb{I}\bigl\lbrace S\_t > \hat{Q}\_{1-\alpha\_t}(\mathcal{C}\_{\text{cal}})\bigr\rbrace$$
   By a telescoping-sum identity, **Isaac Gibbs and Emmanuel Candès (2021)** proved that long-run empirical miscoverage converges to $\alpha$ for *any* arbitrary sequence of observations without exchangeability:
   $$\left|\frac{1}{T}\sum\_{t=1}^T \mathrm{err}\_t - \alpha\right| \le \frac{1}{\gamma T}$$

---

### 10.5 Adapting to Label Shift: The Four Canonical Estimators (`BBSE`, `MLLS/SLD`, `DFM`, `OTTER`)

Now consider **Label Shift**: disease prevalence or delay base rate shifts from source prior $p\_Y = P(Y = 1)$ to an unknown target prior $q\_Y = Q(Y = 1)$, while $P(X \mid Y = y) = Q(X \mid Y = y)$ holds. How can we estimate the new target base rate $q\_Y$ in live production **before a single new ground-truth label $Y$ has arrived**?

Because $Q(X) = \sum\_{y \in \lbrace 0, 1 \rbrace} Q(X \mid Y = y)  q\_Y = \sum\_{y \in \lbrace 0, 1 \rbrace} P(X \mid Y = y)  q\_Y$, the distribution of our classifier's predictions on the unlabeled target stream shifts in an algebraically predictable way! To guard against estimator fragility, `estimate_label_shift_four_methods` computes **all four canonical estimators simultaneously**:

| Estimator | Authors | Mathematical Equation / Algorithm | Stability Diagnostic |
| --- | --- | --- | --- |
| **1. `BBSE`** (Black-Box Shift Estimation) | Lipton, Wang & Smola (2018) | Inverts the source $2 \times 2$ confusion matrix $C\_{ij} = P(\hat{Y}=i, Y=j)$ against target prediction rates $\mu\_i = Q(\hat{Y}=i)$: $\hat{w} = C^{-1}\mu \implies \hat{q}\_Y = \hat{w}\_1 p\_Y$. | Minimum singular value $\sigma\_{\min}(C) = |\mathrm{TPR} - \mathrm{FPR}| > 0$. |
| **2. `MLLS / SLD`** (Maximum Likelihood Label Shift) | Saerens, Latinne & Decaestecker (2002); Garg et al. (2020) | Iterates the Expectation-Maximization (EM) fixed point on target posterior probabilities $p\_i = P(Y=1 \mid X'\_i)$ until convergence: $q^{(k+1)} = \frac{1}{m}\sum\_{i=1}^m \frac{(q^{(k)}/p\_Y)p\_i}{(q^{(k)}/p\_Y)p\_i + ((1-q^{(k)})/(1-p\_Y))(1-p\_i)}$. | Strictly concave log-likelihood; guaranteed inside $[0, 1]$. |
| **3. `DFM`** (Distribution Feature Matching) | Gretton et al. / Characteristic Kernel Matching | Matches the target mean feature/score embedding $\bar{\phi}\_Q = \mathbb{E}\_Q[\phi(X)]$ to the convex combination $(1-q)\bar{\phi}\_{P \mid Y=0} + q\bar{\phi}\_{P \mid Y=1}$: $\hat{q}\_Y = \frac{\bar{\phi}\_Q - \bar{\phi}\_{P \mid 0}}{\bar{\phi}\_{P \mid 1} - \bar{\phi}\_{P \mid 0}}$. | Class-conditional separation $\Delta\_{\min} = |\bar{\phi}\_{P \mid 1} - \bar{\phi}\_{P \mid 0}| > 0$. |
| **4. `OTTER`** (Optimal Transport Entropic Reweighting) | Courty et al. / Sinkhorn-Knopp Scaling | Solves the entropic optimal-transport coupling between source class conditionals and unlabeled target distribution via Sinkhorn-Knopp matrix scaling and reads off the marginal mass $\hat{q}\_Y$. | Operator condition number $\kappa(M)$. |

Once $\hat{q}\_Y$ is estimated, every individual probability forecast $p(x) = P(Y=1 \mid X=x)$ is updated to the new target regime via Bayes' prior-odds ratio:

$$Q(Y = 1 \mid X = x) = \frac{\frac{\hat{q}\_Y}{p\_Y}  p(x)}{\frac{\hat{q}\_Y}{p\_Y}  p(x) + \frac{1 - \hat{q}\_Y}{1 - p\_Y}  (1 - p(x))}$$

---

## Step 3 — Auditable Laboratory in Rust and Exercises

Run Laboratory 6 from your terminal to see the Bitemporal Clock, Synthetic Canary, Single-Opening Vault, `ExTRA-WCP-T` on real NYC flight shifts (`2013 -> 2023`), the 4 Label Shift estimators, and the Concept Drift Circuit Breaker:

~~~bash
cargo run --release --bin labs -- --governance
~~~

~~~text
=== LAB 6: PRODUCTION GOVERNANCE: BITEMPORAL CLOCK, VAULT, AND 4 DRIFT REGIMES ===
  [Bitemporal Clock + Canary] enabled: MI_canary=0.0000, alarm=false | disabled (mutation): MI_canary=0.6931, alarm=true
  [Single-Opening Temporal Vault] sha256_seal=b451a7f0..8524795a, HMAC_tag=36cb9aff..ab71228a, Brier=0.0044, state=OpenedApproved
  [Covariate Drift: NYC Flights 2013->2023] beta_tilt=-0.1562, Z_P=0.9946, ExTRA_WCP_T_threshold=0.8178, Lei_Candes_bound >= 0.802 | Online_ACI_cov=0.891 (vs static=0.882)
  [Label Shift 4 Estimators (fit on 2013, TPR=0.620, FPR=0.249)] BBSE=0.0341 (sigma_min=0.094), MLLS/SLD=0.1161, DFM=0.0717 (Delta_min=0.6), OTTER=0.0691 (kappa=2.73) | Concept Drift Breaker=ConceptDriftCircuitBreaker (tripped=true)
~~~

And in Laboratory 7 (`labs --real-data`), the out-of-time H1 flight vaults (`nyc_flights_2013_h1.csv` $\to$ `nyc_flights_2023_h1.csv`) and the out-of-domain Houston reserve (`houston_flights_2011.csv`) verify:
- NYC $2013 \to 2023$ Lei & Candès bound $\ge 0.802$ (and online Gibbs-Candès ACI tracks $89.1\%$ target $90\%$ coverage vs. $88.2\%$ static);
- H1 out-of-time reserve $2013 \to 2023$ Lei & Candès bound $\ge 0.828$;
- NYC $\to$ Houston out-of-domain reserve Lei & Candès bound $\ge 0.693$;
- H1 Temporal Vault returns `OpenedApproved` ($\mathrm{BS} = 0.1783$).

### Exercises

1. **(Concept — Bitemporal Asymmetry)** A credit-card fraud model uses the feature `customer_30_day_chargeback_count` computed from the transactions table by `WHERE transaction_date <= current_date`. Given that banks often receive chargeback disputes from cardholders up to $45$ days *after* the `transaction_date`, explain why this SQL query leaks the future, and write the correct bitemporal condition using $t\_{\text{ev}}$ and $t\_{\text{disp}}$.
2. **(Calculation — BBSE Label Shift)** Suppose a binary classifier trained in 2023 has true positive rate $\mathrm{TPR} = P(\hat{Y}=1 \mid Y=1) = 0.80$ and false positive rate $\mathrm{FPR} = P(\hat{Y}=1 \mid Y=0) = 0.10$. In 2026, without any new ground-truth labels, you observe that the classifier predicts $\hat{Y} = 1$ on $\mu\_1 = Q(\hat{Y}=1) = 0.31$ ($31\%$) of incoming cases. Using the total probability identity $\mu\_1 = \mathrm{TPR}\cdot q\_Y + \mathrm{FPR}\cdot (1 - q\_Y)$, solve for the new target prevalence $\hat{q}\_Y$.
3. **(Calculation — Lei & Candès Coverage Floor)** If you construct a nominal $90\%$ conformal prediction interval ($\alpha = 0.10$) and the Total Variation distance between source and target covariate distributions is $d\_{\mathrm{TV}}(P\_X, Q\_X) = 0.072$, what is the guaranteed finite-sample coverage floor $1 - \alpha - d\_{\mathrm{TV}}$?
4. **(Laboratory — Governance Defect Proofs)** Run `cargo run --release --bin gates -- --defect clock`, `--defect vault`, and `--defect concept_drift` and verify that all three exit with code `1`.

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Bitemporal Asymmetry:**
   On `current_date` ($\tau$), a chargeback on a transaction that occurred $10$ days ago ($t\_{\text{ev}} = \tau - 10$) but is disputed $35$ days in the future ($t\_{\text{disp}} = \tau + 35$) was not known at time $\tau$. Filtering only by `transaction_date <= current_date` ($t\_{\text{ev}} \le \tau$) leaks future disputes. The valid bitemporal query requires **both** $t\_{\text{ev}} \le \tau$ and **`chargeback_reported_date <= current_date` ($t\_{\text{disp}} \le \tau$)**.
2. **BBSE Label Shift Calculation:**
   $$\mu\_1 = \mathrm{TPR}\cdot \hat{q}\_Y + \mathrm{FPR}\cdot(1 - \hat{q}\_Y) \implies 0.31 = 0.80\hat{q}\_Y + 0.10(1 - \hat{q}\_Y) = 0.10 + 0.70\hat{q}\_Y$$
   $$\implies \hat{q}\_Y = \frac{0.31 - 0.10}{0.80 - 0.10} = \frac{0.21}{0.70} = 0.300\ (30.0\%)$$
3. **Lei & Candès Coverage Floor:**
   $$1 - \alpha - d\_{\mathrm{TV}}(P\_X, Q\_X) = 1.0 - 0.10 - 0.072 = 0.828\ (82.8\%)$$
4. **Governance Defect Proofs:**
   Running `gates -- --defect clock`, `--defect vault`, and `--defect concept_drift` triggers `[G7 BLOCKED]` in each case and exits with code `1`.

</details>

### Chapter Summary

- **Bitemporal Governance (`BitemporalClock`)** filters strictly by availability time $t\_{\text{disp}} \le \tau$ ($t\_{\text{disp}} \ge t\_{\text{ev}}$) and injects a **Synthetic Canary Column** ($I(\text{Canary}; Y) = 0$) that triggers an immediate alarm if future leakage occurs.
- **The Single-Opening Temporal Vault (`TemporalVault`)** seals pre-registered evaluations with SHA-256, permits only a single opening (`0 -> 1`), and automatically downgrades failed confirmations to exploratory status.
- **The Murphy-CORP Circuit Breaker** separates repairable Covariate/Label shift ($\Delta\mathrm{REL}\_{\mathrm{CORP}} > \tau\_{\mathrm{REL}}$) from fatal Concept Drift ($\Delta\mathrm{RES}\_{\mathrm{CORP}} < -\tau\_{\mathrm{RES}}$).
- **Covariate Drift** is adapted via exponential-tilt **`ExTRA-WCP-T`** bounded by **Lei & Candès's** $1 - \alpha - d\_{\mathrm{TV}}(P\_X, Q\_X)$, while **Label Shift** is estimated across **`BBSE`**, **`MLLS/SLD`**, **`DFM`**, and **`OTTER`**.
