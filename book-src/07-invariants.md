# 7. The Five Boundary Invariants ($\Phi\_1$ to $\Phi\_5$)

> **Learning Objectives.** By the end of this chapter, you will be able to state and apply the Five Boundary Invariants ($\Phi\_1$–$\Phi\_5$) that guard every Epistemological Card: **($\Phi\_1$) Simulability and the Generative Null Model**; **($\Phi\_2$) The True Independent Unit, Pseudoreplication, Moulton-Campbell Unequal-Cluster Correction ($\tilde{m}$), and Tail-Exceedance Design Effect ($\mathrm{DEFF}\_p$)**; **($\Phi\_3$) Severe Testing, Error Taxonomy (Types I, II, III, S, M), Missingness, and SHA-256 Pre-Registration**; **($\Phi\_4$) Economic Scope, Alexandra Chouldechova's Algebraic Fairness Trilemma, and the Automatic Exit Clause**; and **($\Phi\_5$) Question Resolution Alignment**.

---

## Step 1 — Intuition and the Real-World Paradox

### Five Hundred Fish in Two Tanks, and One Hundred Coin Tossers

In 1984, ecologist **Stuart Hurlbert** published a famous review of biological experiments that shocked the scientific community. Suppose a researcher wants to test whether a new water additive makes trout grow faster. She places $250$ trout into Tank A (with the additive) and $250$ trout into Tank B (without the additive). Six months later, she weighs all $500$ fish, runs a two-sample $t$-test with sample size $n = 500$ ($498$ degrees of freedom), and publishes a tiny $p$-value ($p = 0.0001$).

What is wrong with her calculation?
If Tank A happened to sit two meters closer to the window, or had a slightly warmer heater, or had a single dominant fish that stressed the others, **all $250$ fish in Tank A shared that exact same tank environment!** Measuring $250$ fish inside one tank does not give you $250$ independent experiments on the additive; it gives you a very precise measurement of **one tank**. The true independent unit of the experiment is the **tank ($n\_{\text{true}} = 2$)**, not the fish ($n\_{\text{nominal}} = 500$). Hurlbert named this ubiquitous error **pseudoreplication**.

The exact same illusion happens every day in data science:
- In a hospital dataset of $50\text{,}000$ hourly vital-sign readings taken from $400$ patients, the hourly readings of the same patient are not independent—the patient (or the hospital ward) is the cluster!
- In `bank_calls.csv`, there are $n = 27\text{,}716$ five-minute arrival windows recorded across $164$ working days. If a promotional campaign or a Monday morning surge hits the bank, all $169$ five-minute windows of that day surge together. Treating $n = 27\text{,}716$ as the independent sample size understates the tail variance by **$28.26$-fold**!

Now consider a second paradox: **the Hot Hand and the Forgotten Null Model ($\Phi\_1, \Phi\_3$)**.
Ask $100$ people in an auditorium to toss a fair coin $20$ times each and write down their longest streak of consecutive Heads. Even though no one has "coin-tossing talent," in $100\%$ of simulated auditoriums someone will toss at least $6$ Heads in a row, and the longest streak in the room will average **$9.5$ consecutive Heads**! If a journalist interviews only the winner after the fact, tomorrow's headline announces a miracle. Even more subtly, when Joshua Miller and Adam Sanjurjo (2018) wrote down the exact finite-sample generative null model for basketball shooting streaks, they discovered that in $3$ tosses of a fair coin, the average proportion of Heads *immediately following a Head* across sequences is not $\frac{1}{2} = 0.50$, but **$\frac{5}{12} \approx 0.4167$**! For thirty years, psychologists had compared basketball players against $0.50$ instead of simulating the actual null model on a computer.

To prevent these errors from ever entering production, the Stochology Protocol enforces **Five Boundary Invariants ($\Phi\_1$ to $\Phi\_5$)** as a strict boolean product $\Phi = \prod\_{j=1}^5 \Phi\_j$: if even one invariant fails, the gate drops to $0$ and blocks the claim.

---

## Step 2 — Foundations and Mathematical Formalization

### Overview of the Five Boundary Invariants

| Invariant | Name | Formal Requirement | Rust Implementation |
| --- | --- | --- | --- |
| **$\Phi\_1$** | **Simulability** | Fixed seed (`seed = 2026`, `SplitMix64`) and an executable generative null model before any parameter fitting. | `stochology_core::rng::Rng` & `card.rs` |
| **$\Phi\_2$** | **Independent Unit & Exceedance Design Effect** | Margin computed over the true independent unit; Moulton-Campbell unequal-cluster correction $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2(m\_j))$, threshold exceedance ICC $\rho\_I(p)$, and $n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho\_I(p)}$. | `stochology_core::invariants::{ExceedanceDesignEffect, OperationalWeightingGap}` |
| **$\Phi\_3$** | **Severe Test, Missingness & Pre-Registration** | Explicit Rubin missingness (`MCAR`, `MAR`, `MNAR`), Meng's Identity ($\rho\_{R,Y}$), Manski bounds, 64-hex SHA-256 pre-registration seal, and Pearl's Ladder rung. | `stochology_core::{layers, sha256, card}` |
| **$\Phi\_4$** | **Scope, Fairness & Automatic Exit** | Economic threshold $p^{\ast} = \frac{C}{\eta L}$, Chouldechova's algebraic fairness audit ($\mathrm{FPR}\_A / \mathrm{FPR}\_B$), declared scope, and machine-actionable numeric exit clause. | `stochology_core::invariants::ChouldechovaAudit` & `card.rs` |
| **$\Phi\_5$** | **Question Resolution Alignment** | Ban on answering a fine-grained operational decision (5-min shift, 6-hr synoptic track) with coarse annual or monthly aggregates ($R\_{\text{data}} \ge R\_{\text{decision}}$). | `stochology_core::invariants::verify_resolution_phi5` |

---

### 7.1 Invariant $\Phi\_1$ — Simulability and the Generative Null Model

Why must every Epistemological Card declare both `seed_and_environment` (`seed = 2026`, `SplitMix64`) and an executable `null_model` *before* fitting parameters?

1. **Defeating Denominator Illusions:** As the Miller-Sanjurjo hot-hand discovery proved, intuitive mental guesses about "what pure chance looks like" under complex stopping rules, sliding windows, or cluster structures are frequently biased. Writing an executable null generator in code and running it with `seed = 2026` reveals the true distribution of the test statistic when the claim is false.
2. **Synthetic Injection Recovery:** Before trusting an estimator on messy real data where the truth is unknown, $\Phi\_1$ requires knowing that the estimator can recover a known parameter $\theta\_{\text{true}}$ injected into simulated data (formalized via Simulation-Based Calibration in Chapter 9).

---

### 7.2 Invariant $\Phi\_2$ — The Independent Unit, Moulton-Campbell Correction, and Tail Exceedance

#### Step-by-Step Derivation: From Intraclass Correlation to Tail Exceedance $n\_{\mathrm{eff}}(p)$

Suppose our dataset contains $n = \sum\_{j=1}^C m\_j$ elementary observations grouped into $C$ clusters (for instance, $C = 164$ days in `bank_calls.csv`, each containing $m\_j$ five-minute intervals, or $C$ hospitals each containing $m\_j$ patients).

Inside cluster $j$, any two observations share a common cluster shock, inducing an **intraclass correlation coefficient (ICC)** $\rho \in [0, 1]$.
1. **Equal Cluster Sizes (Leslie Kish's Classic Formula, 1965):**
   If every cluster has the exact same size $\bar{m} = n / C$, the variance of the sample mean is inflated relative to simple random sampling by the **Design Effect ($\mathrm{DEFF}$)**:
   $$\mathrm{DEFF}\_{\text{Kish}} = 1 + (\bar{m} - 1)\rho$$

2. **Unequal Cluster Sizes (Moulton 1986 & Campbell et al. 2001):**
   In the real world, clusters are rarely equal in size: one hospital treats $400$ patients while a rural clinic treats $15$; one turbulent storm system has $80$ track points while a short storm has $12$. When cluster sizes $m\_j$ vary with coefficient of variation $\mathrm{CV}(m\_j) = \frac{\mathrm{sd}(m\_j)}{\bar{m}}$, the larger clusters dominate the sum of squared errors! **Brent Moulton and Mike Campbell** proved that the average cluster size $\bar{m}$ in the design effect must be replaced by the **effective cluster size $\tilde{m}$**:

   $$\tilde{m} = \frac{\sum\_{j=1}^C m\_j^2}{\sum\_{j=1}^C m\_j} = \bar{m}\Bigl(1 + \mathrm{CV}^2(m\_j)\Bigr) \ge \bar{m}$$

   Look at what happens when cluster sizes are skewed: if $\bar{m} = 59.59$ and $\mathrm{CV}(m\_j) \approx 1.0$, then $\tilde{m} = 119.05$—**doubling the cluster multiplier before we even multiply by $\rho$!**

3. **Threshold Exceedance ICC ($\rho\_I(p)$) in the Tails:**
   Standard textbooks compute $\rho$ on the *mean* of $Y$. Yet operational decisions (staffing for peak call surges, evacuating for category-2 hurricane winds, holding flights during severe delays) depend on **tail quantiles $q\_p$** (such as $p = 0.90$ or $p = 0.95$).
   Even when everyday fluctuations have weak correlation across a day, **extreme tail events cluster fiercely inside surge days!** Therefore, `ExceedanceDesignEffect` (`crates/stochology-core/src/invariants.rs`) defines the binary tail-exceedance indicator $I\_{ij}(p) = \mathbb{I}\lbrace Y\_{ij} > q\_p \rbrace$ (which has variance $p(1-p)$) and computes the **ANOVA Threshold Exceedance ICC ($\rho\_I(p)$)** across the $C$ clusters:

   $$\mathrm{DEFF}\_{\text{exceedance}}(p) = 1 + (\tilde{m} - 1) \rho\_I(p), \qquad n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m} - 1) \rho\_I(p)}$$

   Let us check the numbers on `bank_calls.csv` ($n = 27\text{,}716$ five-minute windows across $C = 164$ days, $\tilde{m} = 169.00$):
   - At the $90\text{th}$ percentile ($p = 0.90$), the tail-exceedance ICC is **$\rho\_I(0.90) = 0.1623$**.
   - The resulting design effect is **$\mathrm{DEFF}(0.90) = 1 + (169 - 1) \times 0.1623 = 28.26$**.
   - Consequently, the true effective sample size for tail staffing decisions is not $27\text{,}716$, but **$n\_{\mathrm{eff}}(0.90) = \frac{27\text{,}716}{28.26} = 980.7$**! Any confidence interval computed on $n = 27\text{,}716$ is more than five times too narrow (`--defect moulton`).

#### The Operational Weighting Gap (The Inspection Paradox)

Invariant $\Phi\_2$ also audits a cousin of cluster correlation (`OperationalWeightingGap`): **who are you averaging over—the calendar day or the arriving customer?**
Remember the health clinic from Chapter 1: on calm days, $5$ patients/hour arrive and wait $3.6\text{ min}$; on surge days, $11$ patients/hour arrive and wait $55\text{ min}$.
- Averaged **by day**, calm and surge days weigh $50\%$ each.
- Averaged **by patient**, $\frac{11}{5 + 11} = 68.75\%$ of all human beings arrive on the surge day!
This is the **Inspection Paradox** (length-biased sampling): sampling by arriving user always weights high-volume clusters by $m\_j / \bar{m}$, opening a positive gap whenever cluster outcome $Y\_j$ correlates with cluster volume $m\_j$.

---

### 7.3 Invariant $\Phi\_3$ — Severe Testing, Error Taxonomy, Missingness, and Pre-Registration

#### Deborah Mayo's Severity Principle and the Five Types of Statistical Error

Philosopher of statistics **Deborah Mayo** formulated the foundational criterion that distinguishes a genuine empirical test from a hollow confirmation ritual:

> **The Severity Principle:** Data $x\_0$ provide genuine evidence for a claim $H$ *only if* $H$ passes a test $T$ that had a very high probability of refuting $H$ if $H$ were false.

If a bridge inspector tests whether a steel cable can support a $40$-ton truck by placing a $2$-kilogram brick on it and announcing *"The cable did not snap! Hypothesis confirmed!"*, the cable passed a test of **zero severity**.

To audit severity comprehensively, Stochology expands the textbook Neyman-Pearson error table (`Type I` and `Type II`) to **five distinct statistical failure modes**:

| Error Type | Classic Author | What Goes Wrong | Protocol Defense |
| --- | --- | --- | --- |
| **Type I ($\alpha$)** | Neyman & Pearson | **False Positive:** Claiming an effect exists when the null hypothesis ($H\_0$) is actually true. | Executable Null Model ($\Phi\_1$), Attempt Counter $K$, and anytime-valid *e-process* $E\_t / K \ge 1/\alpha$. |
| **Type II ($\beta$)** | Neyman & Pearson | **False Negative:** Failing to detect a real effect because the sample size or test power is too small. | Moulton-Campbell $n\_{\mathrm{eff}}(p) \ge n\_{\min}$ (Filter 2) and cost-loss threshold $p^{\ast} = \frac{C}{\eta L}$. |
| **Type III** | Kimball (1957), Mosteller | **Wrong Question:** Giving the right mathematical answer to the wrong operational question or wrong ontological layer. | Decision-Maker Question header (`who_decides`, `altered_action`) and $\Phi\_5$ resolution check. |
| **Type S (Sign)** | Gelman & Carlin (2014) | **Wrong Direction:** Claiming with "statistical significance" that an intervention helps when it actually harms. | Anchored Kennedy-O'Hagan GP ($\delta(x)$), Manski bounds, and multi-environment ICP ($\hat{S}^{\ast}$). |
| **Type M (Magnitude)** | Gelman & Carlin (2014) | **Exaggeration Ratio (Winner's Curse):** Conditioning on crossing a noisy significance threshold inflates the estimated effect size by $2\times$ to $10\times$. | Out-of-sample CORP-Murphy PAV decomposition and Single-Opening Temporal Vault (`TemporalVault`). |

To seal $\Phi\_3$ cryptographically, every Epistemological Card must carry:
1. An explicit Rubin missingness classification (`MCAR`, `MAR`, or `MNAR`) paired with Meng's Identity and Manski bounds;
2. A declared rung on Pearl's Causal Ladder (`1-See`, `2-Act`, or `3-Imagine`) alongside its `identification_strategy` and explicit `untestable_assumption` (remembering Nancy Cartwright's dictum: *"No causes in, no causes out"*);
3. A **64-character lowercase hexadecimal SHA-256 pre-registration digest** (`preregistration_hash`, verified in pure Rust by `stochology_core::sha256` under FIPS 180-4) sealing the pre-analysis design payload before the test vault is opened.

---

### 7.4 Invariant $\Phi\_4$ — Scope, Algorithmic Fairness (Chouldechova's Trilemma), and the Exit Clause

#### Step-by-Step Derivation of Alexandra Chouldechova's (2017) Impossibility Identity

Whenever a model triggers consequential actions across demographic, geographic, or institutional groups $g \in \lbrace A, B \rbrace$ (such as pretrial detention, loan approval, medical triage, or storm evacuation), committees naturally ask that the algorithm satisfy three fairness properties simultaneously:
1. **Predictive Parity (Equal $\mathrm{PPV}$):** When the model raises an alarm ($S = 1$), the probability that the person truly has the outcome ($Y = 1$) is the same in both groups: $\mathrm{PPV}\_A = \mathrm{PPV}\_B$, where $\mathrm{PPV}\_g = P(Y = 1 \mid S = 1, G = g)$.
2. **Equal False Negative Rate (Equal $\mathrm{FNR}$):** True cases have the same probability of being missed in both groups: $\mathrm{FNR}\_A = \mathrm{FNR}\_B$, where $\mathrm{FNR}\_g = P(S = 0 \mid Y = 1, G = g)$.
3. **Equal False Positive Rate (Equal $\mathrm{FPR}$):** Innocent or event-free individuals have the same probability of suffering a false alarm in both groups: $\mathrm{FPR}\_A = \mathrm{FPR}\_B$, where $\mathrm{FPR}\_g = P(S = 1 \mid Y = 0, G = g)$.

In 2017, statistician **Alexandra Chouldechova** (and independently Jon Kleinberg, Sendhil Mullainathan, and Manish Raghavan) proved a stunning mathematical fact: **this is not an engineering shortcoming that a cleverer neural network can fix; it is an exact algebraic impossibility whenever the two groups have different base rates ($p\_A \neq p\_B$)!**

Let us derive **Chouldechova's Identity** in three lines directly from Bayes' rule. For any group $g$ with base rate $p\_g = P(Y = 1 \mid G = g) \in (0, 1)$, write Bayes' rule for $\mathrm{PPV}\_g$:

$$\mathrm{PPV}\_g = \frac{p\_g (1 - \mathrm{FNR}\_g)}{p\_g (1 - \mathrm{FNR}\_g) + (1 - p\_g) \mathrm{FPR}\_g}$$

Multiply both sides by the denominator and solve for $\mathrm{FPR}\_g$:

$$\mathrm{FPR}\_g = \frac{p\_g}{1 - p\_g} \frac{1 - \mathrm{PPV}\_g}{\mathrm{PPV}\_g} (1 - \mathrm{FNR}\_g)$$

Look at that exact algebraic equation (`ChouldechovaAudit` in `crates/stochology-core/src/invariants.rs`):
- Suppose you force Predictive Parity ($\mathrm{PPV}\_A = \mathrm{PPV}\_B$) and Equal Opportunity ($\mathrm{FNR}\_A = \mathrm{FNR}\_B$).
- Then the second and third factors on the right-hand side are identical for Group A and Group B!
- Dividing $\mathrm{FPR}\_A$ by $\mathrm{FPR}\_B$ immediately yields:

$$\frac{\mathrm{FPR}\_A}{\mathrm{FPR}\_B} = \frac{p\_A / (1 - p\_A)}{p\_B / (1 - p\_B)}$$

The ratio of false positive rates is locked algebraically to the **ratio of prior base-rate odds**!
In `toronto_arrests.csv` ($n = 5\text{,}226$ police arrest records), two demographic strata exhibit empirical base rates $p\_A = 0.3926$ and $p\_B = 0.1119$. If a classifier equalizes $\mathrm{PPV} = 0.65$ and $\mathrm{FNR} = 0.25$ across both groups, Chouldechova's identity forces $\mathrm{FPR}\_A = 0.2610$ versus $\mathrm{FPR}\_B = 0.0509$—meaning innocent members of Group A suffer false alarms at **$5.13\times$ the rate of Group B**!

Because no algorithm can repeal Bayes' theorem, Invariant $\Phi\_4$ requires every Epistemological Card to:
1. Audit Chouldechova's identity explicitly and declare **`who_pays_for_error`** (naming the human or institutional bearer of False Positives and False Negatives);
2. Declare an explicit **`scope_statement`** (where the model is forbidden from being applied);
3. Declare a machine-actionable numeric **`exit_clause`** (for instance: *"Immediately suspend automated shift scheduling if $\mathrm{REL}\_{\mathrm{CORP}} > 0.020$ or $\mathrm{RES}\_{\mathrm{CORP}} < 0.025$ over a 10-business-day rolling window"*).

---

### 7.5 Invariant $\Phi\_5$ — Question Resolution Alignment

Finally, **Invariant $\Phi\_5$ (`verify_resolution_phi5`)** guards the temporal and spatial scale of the data against the scale of the action.

Suppose a hospital director needs to staff nurses for **5-minute arrival surges** on Tuesday mornings, or an emergency manager needs to issue **6-hour hurricane evacuation advisories**, and an analyst hands them a regression fitted on **annual or monthly averages** ($R\_0$). Monthly aggregation averages out the exact intra-day spikes that destroy the queue!

In `crates/stochology-core/src/invariants.rs`, every resolution level is mapped to an ordered scale:
$$\text{Sub-Hourly / 5-Min } (4) > \text{Hourly / 6-Hr Synoptic / Shift } (3) > \text{Daily } (2) > \text{Weekly / Monthly / Annual } (1)$$

Invariant $\Phi\_5$ enforces:

$$R\_{\text{data}} \ge R\_{\text{decision}}$$

Any attempt to answer a shift, hourly, or daily operational question with coarse monthly or annual aggregates is immediately blocked by Gate `G5` (`--defect phi5`).

---

## Step 3 — Auditable Laboratory in Rust and Exercises

Run Laboratory 4 from your terminal to see $\Phi\_1$–$\Phi\_5$, Moulton-Campbell tail-exceedance design effects, and Chouldechova's fairness audit on real data:

~~~bash
cargo run --release --bin labs -- --invariants
~~~

~~~text
=== LAB 4: INVARIANTS PHI1-PHI5, MOULTON-CAMPBELL, EXCEEDANCE, AND CHOULDECHOVA ===
  [Benchmarks Phi2] n=25028 -> m_bar=59.59, m_tilde=119.05, rho_I(0.90)=0.155, n_eff=1297.0 (naive Kish=2482.6) | n=500 -> n_eff=86.0 | var_amp=69.0x
  [Real Data: bank_calls.csv] n=27716, clusters=164, m_bar=169.00, m_tilde=169.00, rho_I(0.90)=0.1623, DEFF=28.26, n_eff(0.90)=980.7
  [Phi4: Chouldechova on toronto_arrests.csv] p_A=0.3926 vs p_B=0.1119 -> equalizing PPV=0.65 & FNR=0.25 forces FPR_A=0.2610 vs FPR_B=0.0509 (ratio=5.13x)
~~~

Notice how the three negative defect proofs verify the teeth of these invariants:
- `cargo run --release --bin gates -- --defect phi5` injects a monthly aggregate dataset against a daily/shift decision and verifies that Gate `G5` halts with exit code `1`.
- `cargo run --release --bin gates -- --defect moulton` injects a pseudoreplicated card that ignores cluster inflation and verifies that Gate `G2` halts with exit code `1`.
- `cargo run --release --bin gates -- --defect chouldechova` injects a card claiming simultaneous equality of $\mathrm{FPR}, \mathrm{PPV}, \mathrm{FNR}$ under unequal base rates and verifies that Gate `G4` halts with exit code `1`.

### Exercises

1. **(Calculation — Moulton-Campbell $\Phi\_2$)** A retail chain collects $n = 10\text{,}000$ customer satisfaction scores across $C = 50$ stores. The mean cluster size is $\bar{m} = 200$, but because flagship stores are much larger than rural stores, the coefficient of variation of store sizes is $\mathrm{CV}(m\_j) = 1.0$.
   - *(a)* Compute the Moulton-Campbell effective cluster size $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$.
   - *(b)* If the threshold exceedance ICC at $p = 0.90$ is $\rho\_I(0.90) = 0.10$, compute the Kish design effect (using $\bar{m}$), the Moulton-Campbell design effect (using $\tilde{m}$), and the true effective sample size $n\_{\mathrm{eff}}(0.90)$.
2. **(Calculation — Chouldechova's Trilemma $\Phi\_4$)** In a medical triage algorithm, Group A has disease prevalence $p\_A = 0.25$ and Group B has prevalence $p\_B = 0.05$. If the triage score is calibrated so that $\mathrm{PPV}\_A = \mathrm{PPV}\_B = 0.50$ and $\mathrm{FNR}\_A = \mathrm{FNR}\_B = 0.20$, compute $\mathrm{FPR}\_A$, $\mathrm{FPR}\_B$, and the ratio $\mathrm{FPR}\_A / \mathrm{FPR}\_B$.
3. **(Concept — Severity and Error Types $\Phi\_3$)** An analyst runs a noisy pilot study ($n = 20$) and finds that a new teaching method raises test scores by $+35\%$ ($p = 0.048$). When the ministry of education rolls the method out to $50\text{,}000$ students, the true effect turns out to be $+2\%$. Which of the five error types (`Type I`, `Type II`, `Type III`, `Type S`, `Type M`) did the pilot study commit, and why did conditioning on $p < 0.05$ in a small noisy sample guarantee it?
4. **(Concept — Question Resolution $\Phi\_5$)** Why can an electric utility whose annual generation exceeds annual demand by $+25\%$ still suffer rolling blackouts on a hot August afternoon? Which invariant blocks using annual totals to plan peaking turbine capacity?

### Chapter Summary

- **$\Phi\_1$ (Simulability):** Every claim requires `seed = 2026` (`SplitMix64`) and an executable generative null model.
- **$\Phi\_2$ (Independent Unit):** Discounts nominal sample size $n$ for unequal cluster sizes ($\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$) and tail clustering ($\rho\_I(p)$), yielding $n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho\_I(p)}$.
- **$\Phi\_3$ (Severe Testing & Missingness):** Enforces Rubin's missingness taxonomy, Meng's Identity, Manski bounds, Pearl's Causal Ladder rung, and a 64-hex SHA-256 pre-registration seal.
- **$\Phi\_4$ (Scope, Fairness & Exit):** Audits Chouldechova's algebraic identity $\mathrm{FPR}\_g = \frac{p\_g}{1-p\_g}\frac{1-\mathrm{PPV}\_g}{\mathrm{PPV}\_g}(1-\mathrm{FNR}\_g)$, names who pays for false positives and false negatives, bounds scope, and sets a numeric exit clause.
- **$\Phi\_5$ (Resolution Alignment):** Blocks coarse temporal or spatial aggregates from answering fine-grained operational decisions ($R\_{\text{data}} \ge R\_{\text{decision}}$).
