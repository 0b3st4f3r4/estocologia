# 3. The Five Transversal Lenses

> **Learning Objectives.** By the end of this chapter, you will be able to audit any stochastic model through five universal lenses: **(1) Information** (Shannon entropy, mutual information, the Data Processing Inequality, the Strong DPI contraction hierarchy $\eta\_{\chi^2} \le \eta\_{\mathrm{KL}} \le \eta\_{\mathrm{TV}}$, and Fano's error floor); **(2) Time** (non-ergodicity, Itô's variance drain $g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2$, the asymmetric Kelly criterion, and Log-Sobolev mixing times); **(3) Scale and Tail** (power-law moment regimes and the Hill tail-index estimator $\hat{\alpha}\_{k,n}$); **(4) Causal Structure and Invariance** (Pearl's three-rung Causal Ladder, confounders vs. Berkson colliders, Invariant Causal Prediction $\hat{S}^{\ast}$, and nonlinear NILE/IRM over an orthonormal Hermite basis); and **(5) Decision and Calibration** (the cost-loss threshold $p^{\ast} = \frac{C}{\eta L}$, $\mathrm{EVPI}$, and the exact isotonic CORP-Murphy decomposition via Pool-Adjacent-Violators).

---

## Step 1 — Intuition and the Real-World Paradox

Suppose five domain specialists—a telecom engineer, a quantitative trader, a reinsurance actuary, an epidemiologist, and a hurricane forecaster—sit around a table. On the surface, their fields look completely unrelated. Yet when their models fail in the real world, they fail along the **same five universal axes**:

1. **The Illusion of Creating Information by Reprocessing (Lens 1 — Information):** An analyst takes a noisy survey table, runs it through three neural feature layers and two clustering algorithms, and believes the polished dashboard contains "more signal" than the raw sensors. In reality, **no mathematical transformation can ever create information about the physical world that was not already captured at the sensor.** Every stage of processing can only preserve or leak signal.
2. **Confusing the Crowd at One Instant with One Person Across Time (Lens 2 — Time):** Imagine a casino game where a fair coin is tossed every minute: on Heads your wealth increases by $+50\%$ ($\times 1.50$); on Tails your wealth drops by $-40\%$ ($\times 0.60$). If $10\text{,}000$ people play a single round, the average wealth of the room grows by $\frac{1.50 + 0.60}{2} = 1.05$ ($+5\%$). It looks like a gold mine! Now play that exact same game **yourself** for $100$ consecutive rounds: half the time you multiply by $1.50$, half the time by $0.60$, and since $1.50 \times 0.60 = 0.90$, **you lose $-10\%$ of your wealth every two rounds** until you are bankrupt! The *average across the crowd* ($g\_{\text{ensemble}}$) and the *growth rate of a single trajectory through time* ($g\_{\text{time}}$) are not the same number.
3. **Treating a Giant Outlier as a "Six-Sigma Fluke" (Lens 3 — Scale and Tail):** If you measure the height of $1\text{,}000$ randomly chosen adults, the tallest person might be $2.05\text{ m}$—barely $1.2\times$ the average—and changes the sample mean by a millimeter. If you measure the net worth of $1\text{,}000$ people or the size of $2\text{,}167$ fire insurance claims (`danish_fire.csv`), a single observation can be **hundreds of times larger than the median** and can outweigh half the dataset combined. In a heavy-tailed world, the sample variance does not converge: standard textbook $\pm 1.96\sigma/\sqrt{n}$ error bars are fiction.
4. **Mistaking a Weathervane for the Wind (Lens 4 — Structure and Invariance):** Looking at historical data, every time the barometer needle drops, rain follows. Correlation is high! What happens if we tape the barometer needle to "Sunny" (an intervention, $\mathrm{do}(\text{Barometer} = \text{High})$)? It rains anyway—because the atmospheric pressure caused both the needle and the rain. Worse still, what happens when we train a model inside a hospital and condition on hospitalization (a **Berkson collider**)? Two completely independent diseases suddenly appear correlated in our dataset simply because either one suffices to put a patient in a hospital bed.
5. **Confusing a Bold Forecast with an Useful Decision (Lens 5 — Decision and Calibration):** A weather app says "$80\%$ chance of hurricane winds" when the true frequency on such days is only $25\%$. If a town evacuates whenever the risk exceeds $p^{\ast} = 12\%$, both $80\%$ and $25\%$ lie above $12\%$, so the mayor orders the right evacuation today. Does that mean the $80\%$ forecast is safe? The moment a neighbouring port uses the same app to decide whether to scuttle a cargo schedule whose break-even threshold is $p^{\ast} = 50\%$, the overconfident $80\%$ forecast triggers a ruinous false alarm!

These five universal audits are **the Five Transversal Lenses** (`crates/stochology-lenses`). Let us build each one from its foundations.

---

## Step 2 — Foundations and Mathematical Formalization

### 3.1 Lens 1 — Information (`stochology_lenses::information`)

#### Foundations: Surprise, Entropy, KL Divergence, and Mutual Information

How do we measure "information" with a number? In 1948, **Claude Shannon** started from a simple intuition: observing an event that was almost certain ($p \approx 1$) tells you almost nothing, whereas observing a rare event ($p \ll 1$) carries high **surprise**, measured as $\ln\frac{1}{p(x)} = -\ln p(x)$ (in natural units, **nats**, or base-$2$ **bits**).

1. **Shannon Entropy $H(W)$:** The average uncertainty (expected surprise) of a random variable $W$ before we observe it:
   $$H(W) \triangleq -\sum\_{w \in \mathcal{W}} P(w)\ln P(w) \ge 0$$
2. **Kullback-Leibler (KL) Divergence $D\_{\mathrm{KL}}(P \parallel Q)$:** How distinguishable a distribution $P$ is from a reference distribution $Q$:
   $$D\_{\mathrm{KL}}(P \parallel Q) \triangleq \sum\_x P(x)\ln\frac{P(x)}{Q(x)} \ge 0$$
3. **Mutual Information $I(W; X)$:** How much observing a noisy signal $X$ reduces our uncertainty about the hidden state of the world $W$:
   $$I(W; X) \triangleq H(W) - H(W \mid X) = \sum\_{w, x} P(w, x)\ln\frac{P(w, x)}{P(w)P(x)} \ge 0$$
   If $X$ and $W$ are independent, $I(W; X) = 0$ (knowing $X$ tells you nothing about $W$).

#### The Data Processing Inequality (DPI) and Strong Contraction (SDPI)

Suppose the true state of the world $W$ generates raw physical data $X$, which an ingestion pipeline summarizes into features $Y$, which a model finally compresses into a decision score $Z$. Because each step depends only on the immediately preceding step, they form a **3-step Markov chain**:

$$W \longrightarrow X \longrightarrow Y \longrightarrow Z$$

Shannon's **Data Processing Inequality (DPI)** (`DpiMarkovChain`) proves that information can never increase along a Markov chain:

$$I(W; Z) \le I(W; Y) \le I(W; X) \le H(W)$$

Even stronger: when a noisy physical or computational channel $W(y \mid x)$ maps an input distribution $P\_X$ to $P\_Y$ and $Q\_X$ to $Q\_Y$, it strictly shrinks the distance between any two distributions! The **Strong Data Processing Inequality (SDPI)** measures the worst-case contraction factor $\eta\_D(W) = \sup\_{P \neq Q} \frac{D(P\_Y \parallel Q\_Y)}{D(P\_X \parallel Q\_X)} < 1$ across three divergence geometries:
- **$\chi^2$-divergence contraction $\eta\_{\chi^2}(W) = \rho\_m^2(X; Y)$** (the squared Hirschfeld-Gebelein-Rényi maximal correlation);
- **KL-divergence contraction $\eta\_{\mathrm{KL}}(W)$**;
- **Total Variation contraction $\eta\_{\mathrm{TV}}(W) = \sup\_{x, x'} \lVert W(\cdot \mid x) - W(\cdot \mid x') \rVert\_{\mathrm{TV}}$** (Dobrushin's coefficient of ergodicity).

The protocol audits the exact **Ahlswede-Gács-Raginsky SDPI Hierarchy**:

$$\eta\_{\chi^2}(W) = \rho\_m^2(X; Y) \le \eta\_{\mathrm{KL}}(W) \le \eta\_{\mathrm{TV}}(W) \le 1$$

#### Fano's Error Floor

What happens when a pipeline leaks mutual information so that $I(W; Z)$ becomes small compared to the entropy $H(W)$ of the $|\mathcal{W}|$ possible states of the world? **Robert Fano's Inequality** (1952) establishes an inescapable lower bound on the probability of classification error $P\_e = P(\hat{W}(Z) \neq W)$ of *any* classifier, no matter how sophisticated:

$$H\_2(P\_e) + P\_e \ln(|\mathcal{W}| - 1) \ge H(W \mid Z) = H(W) - I(W; Z)$$

where $H\_2(p) = -p\ln p - (1-p)\ln(1-p)$ is binary entropy. In plain words: **if your features $Z$ do not carry enough nats of mutual information $I(W; Z)$ to cover the entropy $H(W)$ of the target, no deep neural network can push error below Fano's floor $P\_e^{\min}$.**

#### Blahut-Arimoto Exact Channel Capacity (`blahut_arimoto_capacity`)

While $I(W; X)$ measures the mutual information for a *fixed* input prior $P\_W$, what is the absolute maximum information rate that a noisy physical channel $W(x \mid w)$ can transmit under *any* possible input distribution? In 1972, **Richard Blahut** and **Suguru Arimoto** independently derived an alternating convex-optimization algorithm that converges monotonically to Shannon's **Channel Capacity**:

$$C(W) \triangleq \max\_{P\_W \in \Delta\_{|\mathcal{W}|}} I(P\_W, W)$$

By alternating between the variational posterior $Q^{(t)}(w \mid x) \propto P\_W^{(t)}(w) W(x \mid w)$ and the tilted prior update $P\_W^{(t+1)}(w) \propto P\_W^{(t)}(w) \exp\bigl(\sum\_x W(x \mid w)\ln\frac{W(x \mid w)}{\sum\_{w'} P\_W^{(t)}(w')W(x \mid w')}\bigr)$, `blahut_arimoto_capacity` (together with Laplace-smoothed contingency estimation in `estimate_empirical_channel`) computes the exact capacity $C(W)$ and capacity-achieving input prior $P\_W^{\ast}$ in nats.

---

### 3.2 Lens 2 — Time and Survival (`stochology_lenses::time`)

#### Foundations: Ergodicity Breaking and Itô's Variance Drain

In physics and probability, a process is called **ergodic** if the average across a large population of parallel trajectories at a single time $T$ (*the ensemble average*, $g\_{\text{ensemble}}$) equals the average experienced by a single trajectory followed over a long period of time (*the time average*, $g\_{\text{time}}$).

Additive games with stationary independent increments are ergodic. **Multiplicative compounding is non-ergodic** (Ole Peters, 2019).

Let us see why using Kiyosi Itô's stochastic calculus. Suppose an asset, population, or capital reserve $S\_t$ evolves via geometric Brownian motion with instantaneous drift $\mu$ and volatility $\sigma$:

$$\frac{dS\_t}{S\_t} = \mu dt + \sigma dW\_t$$

1. **The Ensemble Growth Rate ($g\_{\text{ensemble}}$):** If you average $S\_t$ across infinitely many parallel universes, $\mathbb{E}[S\_t] = S\_0 e^{\mu t}$, so the expected wealth grows at rate:
   $$g\_{\text{ensemble}} = \frac{1}{t}\ln \mathbb{E}\left[\frac{S\_t}{S\_0}\right] = \mu$$
2. **The Time-Average Growth Rate ($g\_{\text{time}}$):** You do not live across a million parallel universes; you live in *one* universe over time! By **Itô's Lemma** (which is simply the stochastic version of Jensen's Inequality applied to the concave function $\ln S\_t$), $d(\ln S\_t) = \left(\mu - \frac{1}{2}\sigma^2\right)dt + \sigma dW\_t$. Therefore, along almost every single path over time, the compounded growth rate is:
   $$g\_{\text{time}} = \lim\_{T \to \infty} \frac{1}{T}\ln\left(\frac{S\_T}{S\_0}\right) = \mu - \frac{1}{2}\sigma^2 < g\_{\text{ensemble}}$$

The missing piece, $\frac{1}{2}\sigma^2$, is **Itô's Variance Drain** (or *volatility drag*). Even if the average return $\mu$ is positive, if volatility is high enough that $\frac{1}{2}\sigma^2 > \mu$, the ensemble average $\mathbb{E}[S\_t] \to \infty$ explodes to infinity while **almost every individual player goes bankrupt ($S\_t \to 0$)!** Across $37\text{,}930$ daily trading sessions of the Dow Jones Industrial Average (`djia_1885_2023.csv`, 1885–2023), the annualized ensemble growth rate is $g\_{\text{ensemble}} = 6.12\%$, but Itô's variance drain steals $1.45\%$ per year, leaving a true compounded time-growth rate of $g\_{\text{time}} = 4.68\%$.

#### The Optimal Asymmetric Kelly Fraction and Absorbing Ruin

Because over-betting amplifies $\sigma^2$ quadratically while increasing $\mu$ only linearly, betting too large a fraction $f$ of your capital guarantees ruin. In 1956, Bell Labs physicist **John L. Kelly Jr.** derived the exact fraction $f^{\ast}$ of capital to risk on a repeatable gamble with win probability $p$, fractional loss $a$ on failure, and fractional gain $b$ on success so as to maximize long-run logarithmic time growth $\mathbb{E}[\ln(1 + R\_t(f))]$:

$$\max\_{f} \bigl[  p\ln(1 + b f) + (1 - p)\ln(1 - a f)  \bigr] \implies f^{\ast} = \frac{p}{a} - \frac{1 - p}{b}$$

Notice two crucial boundaries of Kelly's curve:
- At $f = f^{\ast}$, long-run compounded time-growth reaches its unique peak.
- At $f > 2 f^{\ast}$, time-average growth turns **strictly negative** even though every single bet has positive expected value!
- At $f \ge 1/a$, a single bad draw wipes out $100\%$ of capital ($W\_t \le 0$), hitting an **absorbing ruin barrier** from which no future recovery is possible.

Finally, Lens 2 audits how long a stochastic simulation or Markov chain takes to explore its state space via the **Log-Sobolev Inequality (LSI)** constant $\alpha\_{\mathrm{LSI}} = -\frac{1}{2}\ln \eta\_{\mathrm{KL}}$, Penalizing the mixing time whenever heavy-tailed mass $H\_R$ traps the sampler in local modes.

#### Continuous-Return Empirical Kelly vs. the Gaussian Merton Ratio (`empirical_kelly_optimal_fraction`)

In continuous-time Gaussian finance, Robert Merton (1969) approximated the optimal leverage fraction by maximizing the second-order Taylor expansion $g(f) \approx f\hat{\mu} - \frac{1}{2}f^2\hat{\sigma}^2$, which yields the **Merton Ratio** $f\_{\text{Merton}} = \frac{\hat{\mu}}{\hat{\sigma}^2}$. However, real empirical returns $r\_1, \dots, r\_n$ exhibit severe skewness and excess kurtosis (on `djia_1885_2023.csv`, excess kurtosis is $17.8$ and the Jarque-Bera statistic is $\mathrm{JB} = 502\text{,}551$, computed online via `OnlineMoments`). Because the logarithmic utility $\ln(1 + f r\_t)$ diverges to $-\infty$ as $1 + f r\_{\min} \to 0^+$, `empirical_kelly_optimal_fraction` solves the exact concave optimization problem via damped Newton-Raphson:

$$f\_{\text{emp}}^{\ast} = \arg\max\_{0 \le f < f\_{\max}} \frac{1}{n}\sum\_{t=1}^n \ln(1 + f r\_t)$$

Whenever negative tail jumps exist, $f\_{\text{emp}}^{\ast} < f\_{\text{Merton}}$ (`2.08 < 2.12` on DJIA), protecting the agent against Gaussian over-leverage.

---

### 3.3 Lens 3 — Scale and Tail (`stochology_lenses::tail`)

#### Foundations: Thin Tails vs. Power Laws and Moment Existence Regimes

In a Gaussian or exponential distribution (*thin-tailed* / "Mediocristan"), the probability of seeing an event larger than $x$ decays exponentially fast ($e^{-c x^2}$ or $e^{-c x}$). Extreme outliers are practically impossible.

In financial losses, reinsurance claims, city sizes, electrical grid spikes, and casualty counts (*heavy-tailed* / "Extremistan"), the upper tail decays slowly as a **Pareto power law**:

$$P(X > x) \sim C  x^{-\alpha} \qquad \text{as } x \to \infty$$

The positive exponent $\alpha > 0$ is called the **tail index**. Why is $\alpha$ the single most important number in risk management? Because the $m$-th moment $\mathbb{E}[X^m] = \int\_{x\_0}^\infty m x^{m-1} P(X > x) dx \propto \int\_{x\_0}^\infty x^{m - 1 - \alpha} dx$ is finite **if and only if $m < \alpha$**! This splits all positive random variables into three sharply different physical worlds:

| Tail Index Regime | Protocol Classification | Mathematical Consequence | Operational Meaning |
| --- | --- | --- | --- |
| **$\alpha > 2$** | `FiniteMeanAndVariance` | Both $\mathbb{E}[X] < \infty$ and $\mathrm{Var}(X) < \infty$ exist. | Classical Central Limit Theorem holds, though kurtosis may still be infinite if $\alpha \le 4$. |
| **$1 < \alpha \le 2$** | `FiniteMeanInfiniteVariance` | The true mean $\mathbb{E}[X] < \infty$ exists, but **the population variance is infinite ($\mathrm{Var}(X) = \infty$)**. | Sample standard deviation $s\_n$ grows forever with $n$; Gaussian $\pm 1.96 s\_n / \sqrt{n}$ intervals are invalid; single jumps dominate. |
| **$\alpha \le 1$** | `InfiniteMeanAndVariance` | Even the **population mean is infinite ($\mathbb{E}[X] = \infty$)**. | The Law of Large Numbers fails completely: the sample average $\bar{X}\_n$ never converges! |

#### Bruce Hill's (1975) Tail-Index Estimator

How do we estimate $\alpha$ from empirical data without parametric wishful thinking? Sort the sample of size $n$ in ascending order: $X\_{(1)} \le X\_{(2)} \le \dots \le X\_{(n)}$. If the top $k$ observations above threshold $X\_{(n-k)}$ follow a Pareto tail, then by a logarithmic transformation their log-spacings $\ln(X\_{(n-i+1)} / X\_{(n-k)})$ behave like exponential random variables with rate $\alpha$. Maximum likelihood estimation yields **Hill's Estimator** (`HillTailDiagnostic`):

$$\hat{\alpha}\_{k,n} = \left(\frac{1}{k}\sum\_{i=1}^k \ln\frac{X\_{(n-i+1)}}{X\_{(n-k)}}\right)^{-1}, \qquad \mathrm{SE}(\hat{\alpha}\_{k,n}) \approx \frac{\hat{\alpha}\_{k,n}}{\sqrt{k}}$$

Applied to the $n = 2\text{,}167$ industrial fire insurance losses of Copenhagen Re (`danish_fire.csv`), Hill's estimator yields **$\hat{\alpha}\_{\mathrm{Hill}} = 1.6009 \pm 0.1601$**, placing fire claims squarely inside the **`FiniteMeanInfiniteVariance` ($1 < \alpha \le 2$)** regime, with $99\text{th}$ percentile $q\_{99} = 26.04\text{M DKK}$ and an effective tail sample size of only `tail_n_eff` $= 13.9$.

#### Dekkers-Einmahl-de Haan Moment Estimator, Peaks-Over-Threshold GPD, and Expected Shortfall ($\mathrm{ES}\_{0.99}$)

To cross-check Hill's Pareto estimator across all three Extreme Value Theory (EVT) domains of attraction ($\xi > 0$ Fréchet, $\xi = 0$ Gumbel, and $\xi < 0$ Weibull, where $\xi = 1/\alpha$), `crates/stochology-lenses/src/tail.rs` implements three complementary EVT diagnostics:

1. **Dekkers-Einmahl-de Haan (1989) Moment Estimator (`dedh_moment_tail_index`):** Using the first two log-spacing moments $M\_n^{(j)} = \frac{1}{k}\sum\_{i=1}^k \bigl(\ln X\_{(n-i+1)} - \ln X\_{(n-k)}\bigr)^j$ ($j \in \lbrace 1, 2 \rbrace$), the DEdH generalized extreme-value shape estimator is:
   $$\hat{\xi}\_{\mathrm{DEdH}} = M\_n^{(1)} + 1 - \frac{1}{2}\left(1 - \frac{(M\_n^{(1)})^2}{M\_n^{(2)}}\right)^{-1} \in \mathbb{R}$$
2. **Pickands-Balkema-de Haan Peaks-Over-Threshold GPD (`gpd_pot_fit`):** By the Pickands-Balkema-de Haan Theorem (1974/1975), excesses $Y = X - u \mid X > u$ above a high threshold $u$ converge to a **Generalized Pareto Distribution** $G\_{\xi, \beta}(y) = 1 - (1 + \xi y / \beta)^{-1/\xi}$, calibrated via Hosking-Wallis probability-weighted moments (L-moments).
3. **Coherent Expected Shortfall ($\mathrm{ES}\_{0.99}$, `expected_shortfall_99`):** Unlike Value-at-Risk ($q\_{99}$), which ignores the severity of losses beyond the $99\text{th}$ percentile and violates Artzner et al.'s (1999) subadditivity axiom, **Expected Shortfall** $\mathrm{ES}\_{0.99} = \mathbb{E}[X \mid X \ge q\_{0.99}]$ is a coherent spectral risk measure. On `danish_fire.csv`, while $q\_{99} = 26.04\text{M DKK}$, the tail expectation jumps more than twofold to **$\mathrm{ES}\_{0.99} = 58.59\text{M DKK}$** (and $\mathrm{ES}\_{0.99}^{\mathrm{POT}} = 58.88\text{M DKK}$)!

---

### 3.4 Lens 4 — Structure and Invariance (`stochology_lenses::invariance`)

#### Foundations: Pearl's Causal Ladder, Confounders, and Berkson Colliders

Standard regression and machine learning live entirely on the first rung of **Judea Pearl's Three-Rung Causal Ladder**:

| Rung | Name | Formal Query | Canonical Question |
| --- | --- | --- | --- |
| **1. `1-See`** | **Association** (Passive Observation) | $P(Y \mid X = x)$ | *"Seeing that a patient took aspirin, what is the probability their headache is gone?"* |
| **2. `2-Act`** | **Intervention** (Doing) | $P(Y \mid \mathrm{do}(X = x))$ | *"If we administer aspirin to this patient, what is the probability their headache will vanish?"* |
| **3. `3-Imagine`** | **Counterfactual** (Retrospection) | $P(Y\_{x'} \mid X = x, Y = y)$ | *"Given that the patient took aspirin and the headache vanished, would it still have vanished had they not taken it?"* |

Why is $P(Y \mid X = x)$ (Rung 1) so often completely different from $P(Y \mid \mathrm{do}(X = x))$ (Rung 2)? Consider the two elementary causal graphs that trap analysts every day:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1120" height="310" viewBox="0 0 1120 310" fill="none" role="img" aria-labelledby="title-fig-03-dags">
  <title id="title-fig-03-dags">Comparison of Confounder (Common Cause) versus Berkson Collider (Common Effect)</title>
  <defs>
  <marker id="dag-arrow-sky" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="dag-arrow-rose" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--rose)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1118" height="308" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="24" y="18" width="520" height="274" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="284" y="48" text-anchor="middle" fill="var(--sky)" class="mono" font-size="12" font-weight="800" letter-spacing="0.04em">1. CONFOUNDER (COMMON CAUSE: X ← C → Y)</text>
  <circle cx="284" cy="100" r="25" fill="var(--surface)" stroke="var(--sky)" stroke-width="2.6"/>
  <text x="284" y="106" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">C</text>
  <circle cx="170" cy="188" r="25" fill="var(--surface)" stroke="var(--muted)" stroke-width="2.2"/>
  <text x="170" y="194" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">X</text>
  <circle cx="398" cy="188" r="25" fill="var(--surface)" stroke="var(--muted)" stroke-width="2.2"/>
  <text x="398" y="194" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">Y</text>
  <line x1="263" y1="117" x2="195" y2="167" stroke="var(--sky)" stroke-width="2.6" marker-end="url(#dag-arrow-sky)"/>
  <line x1="305" y1="117" x2="373" y2="167" stroke="var(--sky)" stroke-width="2.6" marker-end="url(#dag-arrow-sky)"/>
  <text x="284" y="244" text-anchor="middle" fill="var(--ink)" font-size="12.5" font-weight="750">MUST condition on C to block back-door path</text>
  <text x="284" y="266" text-anchor="middle" fill="var(--muted)" font-size="11.5">Unconditioned X and Y share spurious common cause C</text>
  <rect x="576" y="18" width="520" height="274" rx="12" fill="var(--elevated)" stroke="var(--rose)" stroke-width="2"/>
  <text x="836" y="48" text-anchor="middle" fill="var(--rose)" class="mono" font-size="12" font-weight="800" letter-spacing="0.04em">2. BERKSON COLLIDER (COMMON EFFECT: X → B ← Y)</text>
  <circle cx="722" cy="100" r="25" fill="var(--surface)" stroke="var(--muted)" stroke-width="2.2"/>
  <text x="722" y="106" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">X</text>
  <circle cx="950" cy="100" r="25" fill="var(--surface)" stroke="var(--muted)" stroke-width="2.2"/>
  <text x="950" y="106" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">Y</text>
  <circle cx="836" cy="188" r="25" fill="var(--surface)" stroke="var(--rose)" stroke-width="2.6"/>
  <text x="836" y="194" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">B</text>
  <line x1="743" y1="117" x2="811" y2="167" stroke="var(--rose)" stroke-width="2.6" marker-end="url(#dag-arrow-rose)"/>
  <line x1="929" y1="117" x2="861" y2="167" stroke="var(--rose)" stroke-width="2.6" marker-end="url(#dag-arrow-rose)"/>
  <line x1="752" y1="100" x2="920" y2="100" stroke="var(--rose)" stroke-width="1.8" stroke-dasharray="5 4"/>
  <text x="836" y="88" text-anchor="middle" fill="var(--rose)" class="mono" font-size="11" font-weight="700">spurious link if B conditioned</text>
  <text x="836" y="244" text-anchor="middle" fill="var(--rose)" font-size="12.5" font-weight="750">NEVER condition on B! (Banned by Filter 2)</text>
  <text x="836" y="266" text-anchor="middle" fill="var(--muted)" font-size="11.5">Conditioning on common effect B opens Berkson bias</text>
</svg>
  <figcaption><strong>Figure 3.1 — Confounder (Common Cause) vs. Berkson Collider (Common Effect)</strong></figcaption>
</figure>

1. **The Confounder ($X \leftarrow C \to Y$):** Variable $C$ (say, outdoor summer temperature) causes both ice-cream sales ($X$) and swimming-pool drownings ($Y$). In passive data, $X$ and $Y$ are strongly correlated. If you ban ice cream ($\mathrm{do}(X = 0)$), drownings do not drop at all! To identify the causal effect of $X$ on $Y$, you **must condition on the confounder $C$** to block the back-door path.
2. **The Berkson Collider ($X \to B \leftarrow Y$):** Now flip the arrows! Suppose $X$ (locomotor disease) and $Y$ (respiratory disease) are completely independent in the general population, and either condition can cause a patient to be admitted to a hospital ($B = 1$). If an analyst restricts their dataset only to hospitalized patients (conditioning on $B = 1$), learning that a hospitalized patient does *not* have respiratory disease ($Y = 0$) immediately raises the probability that they must be in the hospital because of locomotor disease ($X = 1$)! **Conditioning on a common effect (collider) creates a mirage of correlation out of thin air** (Joseph Berkson, 1946). That is why Filter 2 of the Stochology Funnel strictly bans conditioning on a Berkson collider.

#### Invariant Causal Prediction (ICP) and Nonlinear NILE/IRM

How can we discover which predictors $X\_S$ are true causal parents of $Y$ rather than spurious proxies or descendants when we observe data across multiple environments or regimes $e \in \mathcal{E}$ (for example, different decades, climate regimes, or geographical basins)?

**Jonas Peters, Peter Bühlmann, and Nicolai Meinshausen (2016)** introduced a profound principle called **Invariant Causal Prediction (ICP)**:

> **If $S^{\ast}$ is the set of true direct causal parents of $Y$, then the conditional mechanism $P^e(Y \mid X\_{S^{\ast}})$ remains invariant across all environments $e \in \mathcal{E}$ that do not directly intervene on $Y$ itself.**

By contrast, any subset $S$ that relies on a confounder, a collider, or a regime-specific proxy will exhibit changing regression coefficients or changing residual distributions when the environment $e$ shifts! ICP identifies the identifiable causal core by taking the **intersection** of all statistically invariant subsets:

$$\hat{S}^{\ast} = \bigcap\_{S  :  H\_{0, S} \text{ not rejected across } \mathcal{E}} S$$

In `crates/stochology-lenses/src/invariance.rs`, the protocol extends linear ICP (`env_ols_invariance_penalties`) to **Nonlinear ICP and NILE/IRM** (`run_nonlinear_icp_nile`) by expanding standardized predictors onto the **3-atom orthonormal probabilists' Hermite basis**:

$$\Phi(z) = \left[  z ,\quad \frac{z^2 - 1}{\sqrt{2}} ,\quad \frac{z^3 - 3z}{\sqrt{6}}  \right], \qquad \mathbb{E}\_{Z \sim \mathcal{N}(0,1)}[\Phi\_j(Z)\Phi\_k(Z)] = \delta\_{jk}$$

and auditing the **Invariant Risk Minimization (IRM / NILE)** risk-gradient penalty $\sum\_{e \in \mathcal{E}} \lVert \nabla\_{w \mid w=1.0} R^e(w \cdot \Phi) \rVert^2$ alongside Paul Rosenbaum's hidden-confounding sensitivity threshold $\Lambda^{\ast}$ (the minimum odds ratio an unobserved confounder would need to overturn the conclusion).

In addition, `crates/stochology-lenses/src/invariance.rs` provides two classic econometric invariance and sensitivity audits:
- **Gregory Chow's (1960) Structural Break $F$-Statistic (`chow_invariance_f_stat`):** Tests $H\_{0, S}: \beta^{(e)} = \beta^{\text{pooled}}$ across $E$ environments via $F\_S = \frac{(\mathrm{RSS}\_{\text{pooled}} - \sum\_{e=1}^E \mathrm{RSS}\_e) / ((E - 1)p)}{\sum\_{e=1}^E \mathrm{RSS}\_e / (N\_{\text{total}} - E p)}$. For the true causal parent $X\_0$, $F\_{\lbrace 0 \rbrace} = 0.18$ (invariant), whereas for the spurious proxy $X\_1$, $F\_{\lbrace 1 \rbrace} = 23\text{,}437.0$ (structural breakdown!).
- **Cinelli & Hazlett (2020) Omitted-Variable Bias Robustness Value (`cinelli_hazlett_robustness_value`):** Computes the exact partial-$R^2$ strength $RV\_q = \frac{1}{2}\bigl(\sqrt{f\_q^4 + 4 f\_q^2} - f\_q^2\bigr)$ (where $f\_q = q\cdot |t| / \sqrt{\mathrm{df}}$) that an unobserved confounder $U$ would need with *both* treatment $X$ and outcome $Y$ to reduce the estimated effect by fraction $q$.

---

### 3.5 Lens 5 — Decision and Calibration (`stochology_lenses::decision`)

#### Foundations: The Cost-Loss Boundary ($p^{\ast}$) and Value of Perfect Information ($\mathrm{EVPI}$)

A probability forecast $p \in (0, 1)$ is not yet an action. To cross from belief to action on Bridge $L\_2 \to L\_3$, we must price the errors.

Consider a decision-maker who must choose between two acts—**`protect`** (e.g., evacuate a coast, staff extra nurses, hold a flight) or **`do_not_protect`**—before a binary adverse event $Y \in \lbrace 0, 1 \rbrace$ occurs with forecast probability $p = P(Y = 1)$:

| Action \ State of the World ($L\_1$) | Event Occurs ($Y = 1$, prob. $p$) | No Event ($Y = 0$, prob. $1 - p$) | Expected Cost |
| --- | --- | --- | --- |
| **`protect`** (Pay cost $C$, mitigate fraction $\eta$ of loss $L$) | $C + (1 - \eta)L$ | $C$ | $C + p(1 - \eta)L$ |
| **`do_not_protect`** (Pay $0$ upfront, suffer full loss $L$ if $Y=1$) | $L$ | $0$ | $p L$ |

Comparing the two expected costs shows that **`protect` is strictly better than `do_not_protect`** whenever $C + p(1 - \eta)L < pL$, which simplifies algebraically to the **Cost-Loss Boundary**:

$$p > p^{\ast} \triangleq \frac{C}{\eta L}$$

Look at the intuition of $p^{\ast} = \frac{C}{\eta L}$:
- If protecting costs $C = USD 12\text{,}000$, a storm surge causes loss $L = USD 125\text{,}000$, and early evacuation mitigates $\eta = 0.80$ ($80\%$) of the avoidable damage ($\eta L = USD 100\text{,}000$), then the critical threshold is **$p^{\ast} = \frac{12\text{,}000}{100\text{,}000} = 0.1200$ ($12\%$)**.
- You do *not* wait until a hurricane is "$50\%$ likely" to evacuate! Because the damage $\eta L$ is more than $8\times$ the cost of protection $C$, it is rational to protect the moment the calibrated probability passes $12\%$.

Furthermore, before spending money on a complex predictive project, we compute the **Expected Value of Perfect Information ($\mathrm{EVPI}$)**—the maximum economic gain an infallible oracle could ever deliver over acting on the base rate $p\_0$ alone:

$$\mathrm{EVPI} = \min\bigl(p\_0 \eta L,  C\bigr) - \max\bigl(0,  p\_0 \eta L - C\bigr) = \begin{cases} p\_0(\eta L - C) & \text{if } p\_0 \le p^{\ast} \text{ (or } p\_0 \eta L \text{ net)}, \\\\ C(1 - p\_0) & \text{if } p\_0 > p^{\ast}. \end{cases}$$

Specifically, under the cost table above, an oracle saves $C$ whenever $Y=0$ (if you would otherwise protect) or saves $\eta L - C$ whenever $Y=1$ (if you would otherwise not protect). If the total engineering and operational cost of building the model exceeds $\mathrm{EVPI}$, **Filter 1 of the Stochology Funnel kills the project immediately.**

#### Calibration vs. Resolution: The Exact Isotonic CORP-Murphy Decomposition

How do we audit whether a forecaster's probabilities $p\_1, \dots, p\_n \in (0, 1)$ against realized binary outcomes $y\_1, \dots, y\_n \in \lbrace 0, 1 \rbrace$ are honest?

We start with Glenn Brier's (1950) strictly proper **Brier Score**:

$$\mathrm{BS} = \frac{1}{n}\sum\_{i=1}^n (p\_i - y\_i)^2$$

In 1973, Allan Murphy showed that the Brier Score splits into three meaningful physical components:
1. **Miscalibration / Reliability ($\mathrm{REL}$):** When the model says $p = 30\%$, how far is the true empirical frequency $\hat{\pi}(p)$ from $0.30$? Small $\mathrm{REL}$ is good ($0$ is perfect calibration).
2. **Resolution / Discrimination ($\mathrm{RES}$):** How strongly do the calibrated frequencies $\hat{\pi}(p)$ separate high-risk days from low-risk days around the overall base rate $\bar{y}$? Large $\mathrm{RES}$ is good ($0$ means a useless constant forecast).
3. **Base Uncertainty ($\mathrm{UNC}\_0 = \bar{y}(1 - \bar{y})$):** The intrinsic variance of the binary outcome in the World ($L\_1$).

Historically, analysts estimated $\mathrm{REL}$ and $\mathrm{RES}$ by chopping $[0, 1]$ into $10$ arbitrary histogram bins—a procedure that breaks the exact algebraic identity and changes its answer whenever someone shifts the bin boundaries!

In 2021, **Timo Dimitriadis, Tilmann Gneiting, and Alexander Jordan** solved this fifty-year problem with the **CORP (*Consistent, Optimally Binned, Reproducible*) Decomposition**. Instead of arbitrary bins, CORP computes the unique non-parametric isotonic regression $\hat{\pi}\_1 \le \hat{\pi}\_2 \le \dots \le \hat{\pi}\_n$ of the sorted outcomes $y\_{(i)}$ on the sorted forecasts $p\_{(i)}$ via the **Pool-Adjacent-Violators (PAV)** algorithm (`CorpPavDecomposition` in `crates/stochology-lenses/src/decision.rs`).

Because PAV partitions the sorted data into optimal monotone blocks where the block average equals the conditional frequency, the **CORP-Murphy Identity** closes to machine precision ($\sim 10^{-15}$):

$$\mathrm{BS} = \underbrace{\frac{1}{n}\sum\_{i=1}^n (p\_{(i)} - \hat{\pi}\_i)^2}\_{\mathrm{REL}\_{\mathrm{CORP}} \text{ (Miscalibration)}} - \underbrace{\frac{1}{n}\sum\_{i=1}^n (\hat{\pi}\_i - \bar{y})^2}\_{\mathrm{RES}\_{\mathrm{CORP}} \text{ (Resolution)}} + \underbrace{\bar{y}(1 - \bar{y})}\_{\mathrm{UNC}\_0 \text{ (Uncertainty)}}$$

Remember the fundamental operational asymmetry of this equation:
- **$\mathrm{REL}\_{\mathrm{CORP}}$ (Miscalibration) can be repaired post-hoc** by monotone recalibration (such as PAV isotonic mapping or Platt scaling).
- **$\mathrm{RES}\_{\mathrm{CORP}}$ (Resolution) cannot be created post-hoc by any monotone rescaling**—because any strictly increasing map $p \mapsto g(p)$ produces the *exact same* PAV isotonic blocks $\hat{\pi}\_i$! To increase $\mathrm{RES}\_{\mathrm{CORP}}$, you must go back to $L\_1$ and collect better causal predictors.
- **Richardson's (2000) Relative Economic Value (`CostLossRule::relative_economic_value`) and Good's (1952) Logarithmic Score (`logarithmic_score`):** Alongside the Brier decomposition, Lens 5 evaluates the normalized economic value of a forecast system relative to climatology and a perfect oracle at threshold $p^{\ast} = \frac{C}{\eta L}$:
  $$V(p^{\ast}) = \frac{\mathbb{E}[\text{Cost}\_{\text{climatology}}] - \mathbb{E}[\text{Cost}\_{\text{forecast}}]}{\mathbb{E}[\text{Cost}\_{\text{climatology}}] - \mathbb{E}[\text{Cost}\_{\text{oracle}}]} \in (-\infty, 1]$$
  as well as I. J. Good's local strictly proper **Logarithmic (Ignorance) Score** $\mathrm{LogS}(p, y) = -\frac{1}{n}\sum\_{i=1}^n \bigl[y\_i \ln \tilde{p}\_i + (1 - y\_i)\ln(1 - \tilde{p}\_i)\bigr]$ safeguarded by Cromwell's floor $\tilde{p}\_i \in [\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$.

---

## Step 3 — Auditable Laboratory in Rust and Exercises

Run Laboratory 3 from your terminal to execute all five transversal lenses on both synthetic benchmarks and real empirical data:

~~~bash
cargo run --release --bin labs -- --lenses
~~~

~~~text
=== LAB 3: THE FIVE TRANSVERSAL LENSES (SDPI/DPI, TIME/KELLY, HILL, ICP, CORP-PAV) ===
  [Lens 1: Information] SDPI: eta_chi2=0.4900 <= eta_KL=0.4900 <= eta_TV=0.7000 (valid=true, C_BA=0.2704 nats) | DPI W->X->Y->Z: 0.3681 >= 0.1662 >= 0.0576 (monotonic=true) | I_amp=0.5185 | Fano P_e >= 0.1156
  [Lens 2: Time] DJIA (37930 trading days, kurtosis=17.8, JB=502551): g_ensemble=0.0612 vs g_time_Ito=0.0468 (drain=0.0145) | Kelly f*=0.8600 (emp_DJIA=2.08 vs Merton=2.12) | Mixing steps=13
  [Lens 3: Tail] Danish Fire (n=2167): Hill alpha_k=1.6009 (+/- 0.1601, DEdH_xi=0.538, GPD_xi=0.483), regime=FiniteMeanInfiniteVariance, median=1.778, MAD=0.574, q99=26.04, ES99=58.59 (POT=58.88), tail_n_eff=13.9 | VicElec (n=52608): q99=6981.5 MW
  [Lens 4: Invariance] ICP S*=[0], env_OLS_penalty=["0.0001", "15.8059"] (ChowF=[0.18, 23437.0]), Storms_nonlinear_NILE_IRM=["0.0011", "0.1072"], Lambda*=4.752 (RV=0.511) | AuthenticForm(S*=[0], discarded='atom_1_regime_shifted_lat_proxy', A=true)
  [Lens 5: Decision] CORP-PAV (Atlantic Storms, n=20778): BS=0.26906 = REL(0.21135) - RES(0.13102) + UNC0(0.18873) [id_err=1.53e-14, LogS_PAV=0.1849, REV_PAV=0.719] | FairCoin(p0=0.50): agree@0.30=100%, REL_over=0.1104 vs REL_cal=0.0005, regret@0.70=+18.20
  [Almost-Perfect Machine M=(T,Phi,theta)] sup_err=0.0234 (Barron C_f=5.60), DPI_gap=0.0297 nats, regions=64 (ratio=5.3x), d_eff=8.00/8, P1..P5=true
  [Relational Ontology & Complex Systems] MI_rel=0.3681 vs MI_iso=0.0003 nats | Prigogine dS=-0.55 (turnover=100.0%, closure=100.0%) | Schelling(tau=0.125->50.2% vs tau=0.375->82.4%) | Daisyworld(MAE=1.52C vs dead=9.57C, dT/dL=-11.93) | verified=true
~~~

Look at what each lens reveals:
1. **Lens 1 (Information):** Across the 3-step Markov chain $W \to X \to Y \to Z$, mutual information decays monotonically ($0.3681 \ge 0.1662 \ge 0.0576\text{ nats}$), the SDPI contraction constants obey $\eta\_{\chi^2}(0.4900) \le \eta\_{\mathrm{KL}}(0.4900) \le \eta\_{\mathrm{TV}}(0.7000)$ with Blahut-Arimoto channel capacity $C\_{\mathrm{BA}} = 0.2704\text{ nats}$, and Fano's inequality forces at least $11.56\%$ classification error.
2. **Lens 2 (Time):** Over $37\text{,}930$ trading sessions of the Dow Jones (`djia_1885_2023.csv`, excess kurtosis $17.8$, Jarque-Bera $\mathrm{JB} = 502\text{,}551$), Itô's variance drain reduces the ensemble growth rate of $6.12\%$ down to a time-average growth rate of $4.68\%$, while exact continuous empirical Kelly leverage ($2.08$) is strictly more conservative than the Gaussian Merton ratio ($2.12$).
3. **Lens 3 (Tail):** On `danish_fire.csv` ($n = 2\text{,}167$), the Hill estimator finds $\hat{\alpha} = 1.6009 \pm 0.1601$ (`FiniteMeanInfiniteVariance`), corroborated by Dekkers-Einmahl-de Haan $\hat{\xi}\_{\mathrm{DEdH}} = 0.538$ and Peaks-Over-Threshold GPD $\hat{\xi}\_{\mathrm{GPD}} = 0.483$, with $99\%$ Expected Shortfall $\mathrm{ES}\_{0.99} = 58.59\text{M DKK}$ (`POT = 58.88M`).
4. **Lens 4 (Invariance):** Across shifting regimes, ICP isolates the true causal parent $\hat{S}^{\ast} = [0]$ with tiny invariance penalty ($0.0001$ linear, Chow $F = 0.18$; $0.0011$ nonlinear NILE/IRM on `atlantic_storms.csv`) while exposing the spurious proxy ($15.8059$ linear, Chow $F = 23\text{,}437.0$; $0.1072$ nonlinear), with Rosenbaum sensitivity $\Lambda^{\ast} = 4.752$ and Cinelli-Hazlett Robustness Value $RV = 0.511$.
5. **Lens 5 (Decision):** On $n = 20\text{,}778$ NOAA Atlantic storm records, the PAV isotonic decomposition splits $\mathrm{BS} = 0.26906$ into $\mathrm{REL}\_{\mathrm{CORP}} = 0.21135$, $\mathrm{RES}\_{\mathrm{CORP}} = 0.13102$, and $\mathrm{UNC}\_0 = 0.18873$ with an identity closure error of $1.53 \times 10^{-14}$, PAV-calibrated Logarithmic Score $\mathrm{LogS} = 0.1849$, and Relative Economic Value $\mathrm{REV}\_{\mathrm{PAV}} = 0.719$ ($71.9\%$ of an infallible oracle's economic gain). Meanwhile, the `FairCoin` benchmark proves that an overconfident forecaster (`REL = 0.1104`) who agrees $100\%$ of the time with a calibrated forecaster (`REL = 0.0005`) at threshold $p^{\ast}\_A = 0.30$ inflicts $+18.20$ units of excess regret the moment the decision threshold moves to $p^{\ast}\_B = 0.70$!

### Exercises

1. **(Calculation — Non-Ergodicity and Itô's Drain)** An investment strategy has expected annual arithmetic return $\mu = 0.12$ ($12\%$) and annual volatility $\sigma = 0.30$ ($30\%$).
   - *(a)* What is its compounded time-average growth rate $g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2$?
   - *(b)* If a fund applies $3\times$ leverage (tripling both $\mu$ to $0.36$ and $\sigma$ to $0.90$), what happens to $g\_{\text{ensemble}}$, and what happens to $g\_{\text{time}}$? Why does the $3\times$ leveraged investor lose money over time while the expected value grows?
2. **(Concept — Confounder vs. Collider)** A tech company notices that among employees it hired, programming test scores ($X$) and interview charisma scores ($Y$) are *negatively* correlated. Given that a candidate is hired ($B = 1$) only if the sum of $X + Y$ exceeds a high cutoff, draw the causal graph connecting $X$, $Y$, and $B$, and explain why conditioning on hired employees created the negative correlation.
3. **(Calculation — Cost-Loss Threshold)** Protecting a power substation against a geomagnetic storm costs $C = USD 45\text{,}000$. An unmitigated transformer failure costs $L = USD 1\text{,}000\text{,}000$, and protective switching prevents $\eta = 0.90$ of the damage. Compute $p^{\ast}$. Should the operator protect if the calibrated probability is $p = 0.065$?
4. **(Concept — CORP-Murphy Asymmetry)** Why is it impossible to increase $\mathrm{RES}\_{\mathrm{CORP}}$ by applying a strictly increasing function $p\_i \mapsto g(p\_i)$ to your model's predicted probabilities?

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Non-Ergodicity and Itô's Drain:**
   - *(a)* Unleveraged time-growth rate: $g\_{\text{time}} = 0.12 - \frac{1}{2}(0.30)^2 = 0.12 - 0.045 = +0.075$ ($+7.5\%$ per year).
   - *(b)* At $3\times$ leverage, $g\_{\text{ensemble}} = 3 \times 0.12 = +0.36$ ($+36\%$), but volatility triples to $\sigma\_3 = 0.90$, quadrupling the variance drain by $3^2 = 9\times$:
     $$g\_{\text{time}}(3) = 0.36 - \frac{1}{2}(0.90)^2 = 0.36 - 0.405 = -0.045\ (-4.5\%\text{ per year!})$$
     Because $f = 3$ exceeds twice the Merton-Kelly optimum ($f^{\ast} = \mu/\sigma^2 = 0.12/0.09 = 1.333$, so $2f^{\ast} = 2.667$), the variance drag $\frac{1}{2}f^2\sigma^2$ overwhelms the linear drift $f\mu$.
2. **Confounder vs. Berkson Collider:**
   - Causal graph: $X \to B \leftarrow Y$, where $B = \mathbb{I}\lbrace X + Y \ge \tau \rbrace$ is a **common effect (Berkson collider)**. Conditioning on $B = 1$ (being hired) means any hired employee with a low programming score $X$ *must* have had a very high charisma score $Y$ to clear the bar $X + Y \ge \tau$, inducing a spurious negative correlation between $X$ and $Y$.
3. **Cost-Loss Threshold:**
   $$p^{\ast} = \frac{C}{\eta L} = \frac{45\text{,}000}{0.90 \times 1\text{,}000\text{,}000} = \frac{45\text{,}000}{900\text{,}000} = 0.050\ (5.0\%)$$
   Since $p = 0.065 > p^{\ast} = 0.050$, the operator **must `protect`** (expected cost of protecting is $45\text{,}000 + 0.065 \times 100\text{,}000 = USD 51\text{,}500$, versus $0.065 \times 1\text{,}000\text{,}000 = USD 65\text{,}000$ without protection).
4. **CORP-Murphy Asymmetry:** The Pool-Adjacent-Violators (PAV) algorithm depends only on the **rank order** of the forecasts $p\_{(1)} \le \dots \le p\_{(n)}$ and the realized outcomes $y\_{(1)}, \dots, y\_{(n)}$. Any strictly increasing transformation $g(p)$ preserves the exact sorting order of $p\_i$, producing the exact same isotonic conditional event frequencies $\hat{\pi}\_i$ and therefore leaving $\mathrm{RES}\_{\mathrm{CORP}} = \frac{1}{n}\sum (\hat{\pi}\_i - \bar{y})^2$ completely unchanged.

</details>

### Chapter Summary

- **Lens 1 (Information):** Post-processing cannot create signal ($I(W; Z) \le I(W; Y) \le I(W; X)$), channel noise contracts distributions according to $\eta\_{\chi^2} = \rho\_m^2 \le \eta\_{\mathrm{KL}} \le \eta\_{\mathrm{TV}}$, and lost information enforces Fano's error floor.
- **Lens 2 (Time):** Multiplicative dynamics are non-ergodic ($g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2 < g\_{\text{ensemble}}$), and survival requires respecting the asymmetric Kelly fraction $f^{\ast} = \frac{p}{a} - \frac{1-p}{b}$ and absorbing ruin barriers.
- **Lens 3 (Tail):** Power-law tails $P(X > x) \propto x^{-\alpha}$ estimated via Hill's $\hat{\alpha}\_{k,n}$ lose finite variance when $\alpha \le 2$ and lose finite mean when $\alpha \le 1$.
- **Lens 4 (Invariance):** Pearl's Causal Ladder separates seeing, doing, and imagining; conditioning on a Berkson collider is forbidden; and Invariant Causal Prediction ($\hat{S}^{\ast}$) + NILE/IRM isolate mechanisms that survive across environments.
- **Lens 5 (Decision):** Rational protection occurs at $p > p^{\ast} = \frac{C}{\eta L}$, project viability requires $\mathrm{EVPI} > C\_{\text{total}}$, and PAV isotonic regression yields the exact CORP-Murphy decomposition $\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$.
