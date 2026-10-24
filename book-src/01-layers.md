# 1. The Three Ontological Layers ($L\_1 \to L\_2 \to L\_3 \to L\_1$)

> **Learning Objectives.** By the end of this chapter, you will be able to distinguish the World ($L\_1$), the Model ($L\_2$), and the Agent ($L\_3$); locate analytical fallacies on the three bridges connecting them; apply Charles Sanders Peirce's three modes of inference; derive and compute Xiao-Li Meng's Fundamental Identity for selection bias and Charles Manski's non-parametric bounds on Bridge $L\_1 \to L\_2$; explain convexity, Jensen's Inequality, and the Continuous Ranked Probability Score ($\mathrm{CRPS}$) on Bridge $L\_2 \to L\_3$; and formalize Performative Prediction ($\theta\_{\mathrm{SL}}, \theta\_{\mathrm{PS}}, \theta\_{\mathrm{PO}}$) and Repeated Risk Minimization contraction on Bridge $L\_3 \to L\_1$.

---

## Step 1 — Intuition and the Real-World Paradox

### The Territory, the Map, and the Driver

Imagine driving through an unfamiliar city during a heavy rainstorm. Three distinct realities are operating at the same time:
1. **The Territory (The World, $L\_1$):** The physical streets of asphalt and concrete, the flooded underpass, the delivery truck double-parked around a blind corner, and the thousands of other drivers reacting to the storm. You never see the entire city at once.
2. **The Map (The Model, $L\_2$):** A simplified representation on your dashboard screen. By design, the map omits almost everything in the physical city—the texture of the pavement, the color of the buildings, the mood of the pedestrians—so that it can fit on a screen and compute routes quickly. Yet the map is always incomplete and potentially out of date.
3. **The Driver (The Agent, $L\_3$):** You—with limited time, finite attention, a specific destination, and an asymmetric cost function (arriving ten minutes late to dinner is a minor annoyance; driving into a flooded underpass is catastrophic).

Here is the central thesis of Chapter 1, which we call **the Bridge Thesis**:

> **Most analytical disasters in science, engineering, finance, and public policy are not errors of arithmetic inside the Map ($L\_2$). They are misalignments on the three bridges connecting the Territory ($L\_1$), the Map ($L\_2$), and the Driver ($L\_3$).**

Let us watch all three bridge failures happen in a single everyday institution: **a municipal health clinic**.

- **Failure on Bridge $L\_1 \to L\_2$ (The Invisible Filter):** The clinic director wants to know how satisfied patients are with waiting times. She places a tablet survey at the exit of the consultation room. The spreadsheet ($L\_2$) reports an average satisfaction of $92\%$. The arithmetic is flawless—but the data arrived **filtered**: the patients who waited three hours in the lobby and walked out in frustration never entered the consultation room and never touched the tablet.
- **Failure on Bridge $L\_2 \to L\_3$ (The Misread Summary):** The clinic can serve $\mu = 12$ patients per hour. Looking at the annual ledger, the manager observes that an average of $\bar{\lambda} = 8$ patients arrive per hour. Plugging $\lambda = 8$ and $\mu = 12$ into the standard queueing formula gives a comfortable average wait of $10$ minutes. The manager ($L\_3$) concludes there is plenty of slack and refuses to hire a relief nurse. What the average hid is that on half the days the clinic is calm ($\lambda = 5$ per hour, wait $\approx 3.6$ minutes) and on the other half it surges ($\lambda = 11$ per hour, wait $= 55$ minutes!). The average arrival rate is still $\frac{5 + 11}{2} = 8$, but the average wait across days is $\frac{3.6 + 55}{2} = 29.3$ minutes—and because $11$ patients per hour arrive on the bad days versus $5$ on the calm days, $\frac{11}{16} \approx 69\%$ of all human beings walk in on the $55$-minute day!
- **Failure on Bridge $L\_3 \to L\_1$ (Performativity):** Alarmed by emergency-room crowding, the regional health authority announces that any clinic whose recorded waiting time exceeds $30$ minutes will lose its quarterly budget bonus. Within two months, the recorded waiting time drops to $18$ minutes. Did patient care improve? No: receptionists simply stopped registering patients at the front desk until a doctor was ready to call them, shifting the queue from the waiting room onto the sidewalk outside. **The decision ($L\_3$) deformed the physical process ($L\_1$) it was trying to measure.**

---

## Step 2 — Foundations and Mathematical Formalization

### 1.1 Overview of the Closed Loop ($L\_1 \to L\_2 \to L\_3 \to L\_1$)

Every stochastic system in the protocol is mapped across three ontological layers and three directed bridges:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1200" height="460" viewBox="0 0 1200 460" fill="none" role="img" aria-labelledby="title-fig-01-layers">
  <title id="title-fig-01-layers">Closed-Loop Ontological Architecture: World (L1), Model (L2), Agent (L3), and the Three Audited Bridges</title>
  <defs>
  <marker id="arrow-sky" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="arrow-violet" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--violet)"/>
  </marker>
  <marker id="arrow-emerald" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--emerald)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1198" height="458" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="20" y="42" width="356" height="136" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.5"/>
  <text x="44" y="76" fill="var(--sky)" class="mono" font-size="12" font-weight="700" letter-spacing="0.06em">LAYER L₁ · REALITY</text>
  <text x="44" y="108" fill="var(--ink)" font-size="19" font-weight="800">WORLD (L₁)</text>
  <text x="44" y="136" fill="var(--muted)" font-size="12.5">Generating Process, Physical</text>
  <text x="44" y="156" fill="var(--muted)" font-size="12.5">Conservation &amp; Noise Physics</text>
  <rect x="824" y="42" width="356" height="136" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2.5"/>
  <text x="848" y="76" fill="var(--violet)" class="mono" font-size="12" font-weight="700" letter-spacing="0.06em">LAYER L₂ · FORMAL MAP</text>
  <text x="848" y="108" fill="var(--ink)" font-size="19" font-weight="800">MODEL (L₂)</text>
  <text x="848" y="136" fill="var(--muted)" font-size="12.5">Reduced Simulable Equations,</text>
  <text x="848" y="156" fill="var(--muted)" font-size="12">Executable Null &amp; Discrepancy δ(x)</text>
  <rect x="400" y="304" width="400" height="136" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2.5"/>
  <text x="424" y="338" fill="var(--emerald)" class="mono" font-size="12" font-weight="700" letter-spacing="0.06em">LAYER L₃ · DECISION &amp; ACTION</text>
  <text x="424" y="370" fill="var(--ink)" font-size="19" font-weight="800">AGENT (L₃)</text>
  <text x="424" y="398" fill="var(--muted)" font-size="12.5">Human, Committee, or Controller</text>
  <text x="424" y="418" fill="var(--muted)" font-size="12.5">Asymmetric Loss, Threshold p* &amp; Act</text>
  <path d="M 376 114 L 816 114" stroke="var(--sky)" stroke-width="3" fill="none" marker-end="url(#arrow-sky)"/>
  <rect x="390" y="16" width="420" height="70" rx="10" fill="var(--surface)" stroke="var(--border)" stroke-width="1.5"/>
  <text x="600" y="44" text-anchor="middle" fill="var(--sky)" font-size="12.5" font-weight="800">Bridge L₁ → L₂ (Observation &amp; Filtering)</text>
  <text x="600" y="67" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.8">Rubin (MCAR/MAR/MNAR) · Meng ρ(R,Y) · Manski</text>
  <path d="M 1002 178 C 1002 290, 900 360, 808 372" stroke="var(--violet)" stroke-width="3" fill="none" marker-end="url(#arrow-violet)"/>
  <rect x="760" y="208" width="420" height="70" rx="10" fill="var(--surface)" stroke="var(--border)" stroke-width="1.5"/>
  <text x="970" y="236" text-anchor="middle" fill="var(--violet)" font-size="12.5" font-weight="800">Bridge L₂ → L₃ (Reading &amp; Valuation)</text>
  <text x="970" y="259" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Jensen's Gap Δ_J · Quantiles · CRPS / Brier</text>
  <path d="M 400 372 C 300 360, 198 290, 198 186" stroke="var(--emerald)" stroke-width="3" fill="none" marker-end="url(#arrow-emerald)"/>
  <rect x="20" y="208" width="420" height="70" rx="10" fill="var(--surface)" stroke="var(--border)" stroke-width="1.5"/>
  <text x="230" y="236" text-anchor="middle" fill="var(--emerald)" font-size="12.5" font-weight="800">Bridge L₃ → L₁ (Performativity)</text>
  <text x="230" y="259" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Induced D(θ) · θ_PS vs. θ_PO · RRM &lt; 1</text>
</svg>
  <figcaption><strong>Figure 1.1 — The Closed Epistemological Loop (L₁ → L₂ → L₃ → L₁)</strong></figcaption>
</figure>

| Layer | Code | Ontological Definition | Typical Outbound Bridge Error | Formal Bridge Audit |
| --- | --- | --- | --- | --- |
| **World** | $L\_1$ | Physical, biological, or socioeconomic reality generating the data, never observed in its entirety. | **Bridge $L\_1 \to L\_2$ (Invisible Filter):** selection, censoring, or survivorship bias. | Meng's Identity ($\rho\_{R,Y} \neq 0$), Rubin's missingness taxonomy (`MCAR`, `MAR`, `MNAR`), and Manski's non-parametric bounds. |
| **Model** | $L\_2$ | Formal, reduced, computationally simulable mathematical representation. | **Bridge $L\_2 \to L\_3$ (Misreading):** treating $\mathbb{E}[X]$ as a sufficient summary of a nonlinear payoff/loss $f(X)$. | Quantification of Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ and strictly proper scoring rules ($\mathrm{CRPS}$, Brier). |
| **Agent** | $L\_3$ | Human decision-maker, institutional committee, or algorithmic controller. | **Bridge $L\_3 \to L\_1$ (Performativity):** decisions based on the model deform the very distribution $\mathcal{D}(\theta)$ being measured. | Induced distribution map $\mathcal{D}(\theta)$, separation between Performative Stability ($\theta\_{\mathrm{PS}}$) and Performative Optimality ($\theta\_{\mathrm{PO}}$), and RRM contraction $\frac{\varepsilon\beta}{\gamma} < 1$. |

### Peirce's Three Modes of Inference and the Three Senses of "Bias"

Before writing equations for the three bridges, two foundational distinctions clarify how reasoning travels around this loop.

First, the philosopher and logician **Charles Sanders Peirce** showed that all scientific reasoning consists of three distinct operations linking a *Rule*, a *Case*, and a *Result*:
1. **Deduction** (*Rule + Case $\to$ Result*): "All beans in this bag are white; these beans came from this bag; therefore, these beans are white." Deduction is **truth-preserving** and lives entirely inside the Model ($L\_2$). If the premises hold, the conclusion is guaranteed, but it adds no new empirical knowledge about the World.
2. **Induction** (*Case + Result $\to$ Rule*): "These beans came from this bag; these beans are white; therefore, all beans in this bag are white." Induction crosses Bridge $L\_1 \to L\_2$, generalizing from a finite sample to a population law. It is **ampliative**—it claims more than was observed, and can fail whenever the sample was filtered.
3. **Abduction** (*Rule + Result $\to$ Case*): "All beans in this bag are white; these beans on the table are white; perhaps these beans came from this bag." Abduction is the creative leap that proposes a candidate hypothesis or causal mechanism to explain a surprising observation. It is the birthplace of every model, and also the birthplace of the **Prosecutor's Fallacy** whenever an agent mistakes a plausible explanation ($P(\text{Evidence} \mid \text{Hypothesis})$) for a proven truth ($P(\text{Hypothesis} \mid \text{Evidence})$).

Second, notice how the word **"bias"** changes meaning depending on which layer you are standing in:
- In the **World ($L\_1$)**, *selection bias* means the recorded sample $n$ systematically over-represents some parts of the population $N$.
- In the **Model ($L\_2$)**, *estimator bias* means the expected value of a mathematical formula $\mathbb{E}[\hat{\theta}]$ differs from the true parameter $\theta$.
- In the **Agent ($L\_3$)**, *cognitive or strategic bias* means the decision-maker's mental shortcuts or institutional incentives distort how forecasts are interpreted or acted upon.

When an analyst says "we corrected for bias," your first question must always be: **in which layer?**

---

### 1.2 Bridge $L\_1 \to L\_2$: Rubin's Taxonomy, Meng's Identity, and Manski's Bounds

#### Foundations: What Happens When Data Are Missing or Filtered?

Suppose a target population consists of $N$ units (for instance, all $N$ workers in a labor market or all $N$ patients visiting an emergency department), each carrying an outcome of interest $Y\_i$. We want to know the true population average:

$$\bar{Y}\_N = \frac{1}{N}\sum\_{i=1}^N Y\_i$$

However, we only observe a subset of $n < N$ units. To make the filter visible in mathematics, we define a binary **recording indicator** $R\_i$ for every unit $i \in \lbrace 1, \dots, N \rbrace$:

$$R\_i = \begin{cases} 1, & \text{if unit } i \text{ is recorded in our dataset,} \\\\ 0, & \text{if unit } i \text{ is missing or excluded.} \end{cases}$$

The total sample size is $n = \sum\_{i=1}^N R\_i$, the sampling fraction is $f = \frac{n}{N}$, and the sample mean we actually compute on our computer is:

$$\bar{Y}\_n = \frac{\sum\_{i=1}^N R\_i Y\_i}{\sum\_{i=1}^N R\_i}$$

Donald Rubin (1976) classified the physical mechanism governing $R\_i$ into three regimes:
1. **`MCAR` (Missing Completely at Random):** Whether a record is observed ($R\_i$) is statistically independent of both the observed covariates $X\_i$ and the unseen outcome $Y\_i$ ($R \perp (X, Y)$). *Example:* A laboratory technician accidentally drops one out of twenty blood vials on the floor. Under `MCAR`, the sample mean $\bar{Y}\_n$ is an unbiased estimate of $\bar{Y}\_N$.
2. **`MAR` (Missing at Random):** Conditional on the covariates $X\_i$ that we *did* record, missingness no longer depends on the unseen outcome $Y\_i$ ($R \perp Y \mid X$). *Example:* Younger respondents are less likely to answer a telephone survey, but *within* each age bracket, respondents and non-respondents have the same income distribution. Under `MAR`, we can recover $\bar{Y}\_N$ by reweighting across the observed strata $X$.
3. **`MNAR` (Missing Not at Random):** Even after conditioning on all observed variables $X\_i$, the probability of being recorded still depends directly on the unobserved outcome $Y\_i$ itself ($R \not\perp Y \mid X$). *Example:* People with severely depressed earnings self-select into a government training program, or dissatisfied customers leave a platform without filling out the exit survey.

#### Xiao-Li Meng's Fundamental Identity (2018) and the "Big Data Paradox"

A widespread myth in data science holds that *"if the dataset is massive enough ($n$ in the hundreds of thousands or millions), selection bias no longer matters."* In 2018, Harvard statistician Xiao-Li Meng proved an exact algebraic identity that demolishes this myth.

Because $\bar{Y}\_n = \frac{\frac{1}{N}\sum\_{i=1}^N R\_i Y\_i}{\frac{1}{N}\sum\_{i=1}^N R\_i} = \frac{\mathbb{E}\_N[R Y]}{\mathbb{E}\_N[R]}$ and $\mathrm{Cov}\_N(R, Y) = \mathbb{E}\_N[RY] - \mathbb{E}\_N[R]\mathbb{E}\_N[Y]$, a three-line algebraic rearrangement yields **Meng's Fundamental Identity**:

$$\bar{Y}\_n - \bar{Y}\_N = \underbrace{\rho\_{R,Y}}\_{\text{Data Quality}} \times \underbrace{\sqrt{\frac{N - n}{n}}}\_{\text{Data Quantity}} \times \underbrace{\sigma\_Y}\_{\text{Problem Difficulty}}$$

Look closely at the three factors multiplying on the right-hand side:
1. **$\rho\_{R,Y} = \mathrm{Corr}\_N(R, Y)$ (Data Quality):** The population correlation between *being recorded* ($R\_i$) and *the outcome itself* ($Y\_i$). Under true simple random sampling (`MCAR`), $\mathbb{E}[\rho\_{R,Y}^2] = \frac{1}{N - 1}$, which shrinks rapidly with population size. Under self-selection or unadjusted observational filtering (`MNAR`), $\rho\_{R,Y}$ stays stuck at a non-zero constant (say, $+0.05$ or $+0.13$).
2. **$\sqrt{\frac{N - n}{n}} = \sqrt{\frac{1 - f}{f}}$ (Data Quantity):** A factor determined by the sampling fraction $f = n/N$.
3. **$\sigma\_Y$ (Problem Difficulty):** The true population standard deviation of the outcome $Y$.

Why is this identity so devastating for naive "Big Data"? Because if a massive unrepresentative sample of size $n$ has selection correlation $\rho\_{R,Y}$, its mean squared error is equal to that of a tiny, genuinely random sample of **effective sample size**:

$$n\_{\mathrm{eff}} \approx \frac{n}{1 + n \left(\frac{N - n}{N}\right) \rho\_{R,Y}^2 \cdot (\text{design scaling})}$$

In our empirical benchmark on Robert LaLonde's (1986) labor-market data (`nsw_experimental.csv`, the randomized National Supported Work experiment, versus `cps_observational.csv`, the observational Current Population Survey comparison group of $n = 15\text{,}992$ workers), the selection correlation is $\rho\_{R,Y} = +0.1334$. That seemingly modest correlation of $0.1334$ shrinks a nominal sample of $n = 15\text{,}992$ workers down to an effective sample size of **$n\_{\mathrm{eff}} = 2\text{,}842.70$**! Over $82\%$ of the apparent sample size was an illusion created by the invisible filter on Bridge $L\_1 \to L\_2$.

#### Charles Manski's Non-Parametric Worst-Case Bounds

When missingness or selection is `MNAR` and we do not know $\rho\_{R,Y}$ exactly, what can an honest scientist say without inventing untestable parametric assumptions?

Econometrician **Charles Manski** (1989, 1990) showed that whenever the outcome $Y$ is bounded by known physical or institutional limits $[Y\_{\min}, Y\_{\max}]$, the true population expectation $\mathbb{E}[Y]$ is strictly trapped inside a **non-parametric identification interval**:

$$\mathbb{E}[Y] = P(R = 1) \mathbb{E}[Y \mid R = 1] + P(R = 0) \mathbb{E}[Y \mid R = 0]$$

Since we observe the response rate $p\_{\text{obs}} = P(R = 1)$ and the observed mean $\bar{Y}\_{\text{obs}} = \mathbb{E}[Y \mid R = 1]$, and since the unobserved mean $\mathbb{E}[Y \mid R = 0]$ cannot be smaller than $Y\_{\min}$ or larger than $Y\_{\max}$, the truth must lie inside **Manski's Bounds**:

$$\bigl[  p\_{\text{obs}} \bar{Y}\_{\text{obs}} + (1 - p\_{\text{obs}}) Y\_{\min} ,\quad p\_{\text{obs}} \bar{Y}\_{\text{obs}} + (1 - p\_{\text{obs}}) Y\_{\max}  \bigr]$$

No matter how adversarial the missingness mechanism is, the population mean cannot escape this interval (`crates/stochology-core/src/layers.rs`, `ManskiBounds`). If a decision remains optimal at both endpoints of Manski's interval, the decision is immune to missing-data bias; if the optimal action flips between the lower and upper bound, the analyst must declare that the conclusion depends on an untestable assumption about the unseen records.

---

### 1.3 Bridge $L\_2 \to L\_3$: Nonlinearity, Jensen's Gap, and $\mathrm{CRPS}$

#### Foundations: Convexity and Why the Average Input Does Not Give the Average Output

Once data have crossed into the Model ($L\_2$), a second trap awaits on Bridge $L\_2 \to L\_3$: **reading the expected value $\mathbb{E}[X]$ as if it were the only number the decision-maker ($L\_3$) needs.**

Draw the curve $f(x) = x^2$ on a sheet of paper. It is a bowl opening upward—a **convex** function. Pick two equally likely outcomes: $x\_1 = 0$ and $x\_2 = 2$.
- What is the average of the inputs? $\mathbb{E}[X] = \frac{0 + 2}{2} = 1$.
- If you plug that average input into the formula, what do you get? $f(\mathbb{E}[X]) = 1^2 = 1$.
- Now compute the actual average of the outputs: $\mathbb{E}[f(X)] = \frac{0^2 + 2^2}{2} = \frac{0 + 4}{2} = 2$.

Look at the gap between the two numbers: $\mathbb{E}[f(X)] = 2$ is twice as large as $f(\mathbb{E}[X]) = 1$! Geometrically, for any convex function (a curve whose second derivative is positive, $f''(x) > 0$), the straight chord connecting any two points on the curve lies strictly **above** the curve itself.

This theorem is **Jensen's Inequality** (Johan Jensen, 1906): for any random variable $X$ and any convex function $f$,

$$\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$$

We define the exact difference as **Jensen's Gap ($\Delta\_J$)**:

$$\Delta\_J \triangleq \mathbb{E}[f(X)] - f(\mathbb{E}[X])$$

To see *why* $\Delta\_J$ grows when the world becomes more volatile, expand $f(X)$ around the mean $\mu = \mathbb{E}[X]$ using a second-order Taylor series:

$$f(X) \approx f(\mu) + f'(\mu)(X - \mu) + \frac{1}{2}f''(\mu)(X - \mu)^2$$

Taking the expectation of both sides (and noting that $\mathbb{E}[X - \mu] = 0$ while $\mathbb{E}[(X - \mu)^2] = \mathrm{Var}(X)$) yields the **Second-Order Variance Toll**:

$$\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \frac{1}{2}  f''(\mu)  \mathrm{Var}(X)$$

Read that equation in plain English:

> **Whenever a cost, waiting time, or damage function bends upward ($f''(\mu) > 0$), variability is not neutral noise that cancels out over time. Variability pays a positive tax equal to half the curvature times the variance.**

In `bank_calls.csv` ($164$ business days of a North American retail bank's call center), daily call volume fluctuates across days. Under a quadratic staffing congestion penalty, evaluating the cost at the mean daily volume gives $f(\mathbb{E}[X]) = 1.0669 \times 10^9$, whereas the true average daily cost is $\mathbb{E}[f(X)] = 1.0760 \times 10^9$—leaving a **Jensen's Gap of $\Delta\_J = +9\text{,}033\text{,}837.19$** that a spreadsheet using only the average day completely erases!

#### Evaluating Full Predictive Distributions: The Continuous Ranked Probability Score ($\mathrm{CRPS}$)

Because a single mean hides both Jensen's Gap and tail risk, Prohibition 2 requires every model on Bridge $L\_2 \to L\_3$ to report not only $f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, and $\Delta\_J$, but also the **median**, the **5th and 95th percentiles ($q\_{05}, q\_{95}$)**, and the **Continuous Ranked Probability Score ($\mathrm{CRPS}$)**.

What is the $\mathrm{CRPS}$, and why does Stochology mandate it?
Suppose a model issues a full predictive cumulative distribution function (CDF) $F(x) = P(X \le x)$, and reality subsequently materializes at the actual number $y$. A single point metric like Root Mean Squared Error ($\mathrm{RMSE}$) ignores whether the model's uncertainty bands were honest or delusional. The **$\mathrm{CRPS}$** scores the *entire predictive distribution* at once by integrating the squared distance between the forecast CDF $F(x)$ and the empirical step function $\mathbb{I}\lbrace x \ge y \rbrace$ (which is $0$ below $y$ and jumps to $1$ at $y$):

$$\mathrm{CRPS}(F, y) \triangleq \int\_{-\infty}^{\infty} \bigl(F(x) - \mathbb{I}\lbrace x \ge y \rbrace\bigr)^2 dx = \mathbb{E}\_F|X - y| - \frac{1}{2}\mathbb{E}\_F|X - X'|$$

where $X$ and $X'$ are independent draws from the predictive distribution $F$. Notice the two terms in the second expression (`NonlinearSummary` in `crates/stochology-core/src/layers.rs`):
- The first term, $\mathbb{E}\_F|X - y|$, rewards the distribution for placing its mass close to the realized truth $y$ (*accuracy*).
- The second term, $-\frac{1}{2}\mathbb{E}\_F|X - X'|$, rewards sharp, well-calibrated spread.
- Most importantly, $\mathrm{CRPS}$ is a **strictly proper scoring rule**: a forecaster minimizes expected $\mathrm{CRPS}$ if and only if they report their true predictive distribution $F$, with zero incentive to hedge or exaggerate confidence.

---

### 1.4 Bridge $L\_3 \to L\_1$: Performative Prediction and Repeated Risk Minimization

#### Foundations: When Deploying the Model Deforms the World

Standard supervised learning assumes that the data-generating distribution $\mathcal{D}$ is a passive photograph of nature: whether you deploy parameter vector $\theta\_1$ or $\theta\_2$, the world keeps drawing pairs $Z = (X, Y) \sim \mathcal{D}$.

In human and institutional systems, this assumption is false. As economist **Robert Lucas** pointed out in 1976 (*the Lucas Critique*), and as **Charles Goodhart** formulated in monetary policy, whenever an agent ($L\_3$) deploys a predictive model parameterized by $\theta \in \Theta$ to allocate credit, route traffic, set insurance premiums, or trigger police patrols, the population in the World ($L\_1$) reacts to the rule. **Deploying $\theta$ changes the distribution of the data from $\mathcal{D}$ to $\mathcal{D}(\theta)$.**

In 2020, Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt unified this feedback loop under the mathematical theory of **Performative Prediction**.

To reason cleanly about Bridge $L\_3 \to L\_1$, we must distinguish two risk functions:

$$\mathrm{PR}(\theta) \triangleq \mathbb{E}\_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \theta)], \qquad \mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}\_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$$

Look at the difference between the two formulas:
- In **Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi)$**, the first argument $\theta$ fixes the **World's distribution** $\mathcal{D}(\theta)$ (how the population is currently behaving under the previously deployed policy $\theta$), while the second argument $\phi$ is the **new candidate model** being evaluated on yesterday's data.
- In **Performative Risk $\mathrm{PR}(\theta) = \mathrm{DPR}(\theta, \theta)$**, both arguments move together: it measures the true loss you will experience *after* you deploy $\theta$ and the world shifts to $\mathcal{D}(\theta)$.

This distinction reveals two fundamentally different equilibria that analysts routinely conflate (Prohibition 3):

1. **Performative Stability ($\theta\_{\mathrm{PS}}$ — The Myopic Retraining Fixed Point):**
   What happens in almost every modern MLOps pipeline? Data scientists collect data under the current model $\theta\_t$, retrain a new model $\theta\_{t+1} = \arg\min\_\phi \mathrm{DPR}(\theta\_t, \phi)$ as if the distribution were static, deploy $\theta\_{t+1}$, and repeat. This loop is called **Repeated Risk Minimization (RRM)**. If RRM converges to a fixed point where retraining no longer changes the parameters, that point is **Performatively Stable**:
   $$\theta\_{\mathrm{PS}} = \arg\min\_{\phi \in \Theta} \mathrm{DPR}(\theta\_{\mathrm{PS}}, \phi)$$
   At $\theta\_{\mathrm{PS}}$, the model looks perfectly optimal on its own fresh validation data—yet it is completely blind to how its own deployment shifted $\mathcal{D}(\theta\_{\mathrm{PS}})$!

2. **Performative Optimality ($\theta\_{\mathrm{PO}}$ — The True Stackelberg Equilibrium):**
   An agent who understands Bridge $L\_3 \to L\_1$ acts as a **Stackelberg leader**: instead of taking $\mathcal{D}(\theta)$ as frozen, the agent differentiates through the environment's reaction map $\theta \mapsto \mathcal{D}(\theta)$ and minimizes total Performative Risk directly:
   $$\theta\_{\mathrm{PO}} = \arg\min\_{\theta \in \Theta} \mathrm{PR}(\theta)$$

#### Step-by-Step Derivation of the Canonical Linear Benchmark

Let us derive the exact gap between passive learning ($\theta\_{\mathrm{SL}}$), Performative Stability ($\theta\_{\mathrm{PS}}$), and Performative Optimality ($\theta\_{\mathrm{PO}})$ in the canonical linear-quadratic system implemented in `PerformativeSystem` (`crates/stochology-core/src/layers.rs`).

Suppose that when the institution deploys decision parameter $\theta$, the outcome in the world reacts linearly with performative sensitivity $\varepsilon$:

$$Y(\theta) = \mu + \varepsilon\theta + \xi, \qquad \mathbb{E}[\xi] = 0,\quad \mathrm{Var}(\xi) = \sigma^2$$

and the institution evaluates candidate parameter $\phi$ under the strongly convex regularized loss:

$$\ell(Y; \phi) = \frac{\gamma}{2}(Y - \phi)^2 - \alpha\phi \qquad (\gamma > 0)$$

1. **Passive Supervised Learning ($\theta\_{\mathrm{SL}}$, ignoring feedback $\varepsilon = 0$):**
   Before the model is deployed ($\theta = 0$), $\mathbb{E}[Y(0)] = \mu$. Minimizing $\mathbb{E}[\ell(Y(0); \phi)] = \frac{\gamma}{2}[(\mu - \phi)^2 + \sigma^2] - \alpha\phi$ with respect to $\phi$ sets the derivative to zero:
   $$-\gamma(\mu - \phi) - \alpha = 0 \implies \theta\_{\mathrm{SL}} = \mu + \frac{\alpha}{\gamma}$$

2. **Repeated Risk Minimization and Performative Stability ($\theta\_{\mathrm{PS}}$):**
   When $\theta\_t$ is deployed in round $t$, the environment shifts its mean to $\mathbb{E}[Y(\theta\_t)] = \mu + \varepsilon\theta\_t$. A myopic retrainer holds $\theta\_t$ fixed and minimizes $\mathrm{DPR}(\theta\_t, \phi)$ over $\phi$:
   $$-\gamma(\mu + \varepsilon\theta\_t - \phi) - \alpha = 0 \implies \theta\_{t+1} = \mu + \frac{\alpha}{\gamma} + \varepsilon\theta\_t$$
   Setting $\theta\_{t+1} = \theta\_t = \theta\_{\mathrm{PS}}$ gives the **Performative Stability fixed point**:
   $$\theta\_{\mathrm{PS}} = \frac{\mu + \alpha / \gamma}{1 - \varepsilon}$$

3. **True Performative Optimality ($\theta\_{\mathrm{PO}}$, Stackelberg Equilibrium):**
   Now substitute $\phi = \theta$ *before* differentiating! The true Performative Risk is:
   $$\mathrm{PR}(\theta) = \mathbb{E}\left[\frac{\gamma}{2}(\mu + \varepsilon\theta + \xi - \theta)^2 - \alpha\theta\right] = \frac{\gamma}{2}\bigl[(1 - \varepsilon)\theta - \mu\bigr]^2 + \frac{\gamma}{2}\sigma^2 - \alpha\theta$$
   Differentiating $\mathrm{PR}(\theta)$ with respect to $\theta$ (including the $(1 - \varepsilon)$ chain-rule factor from the environment's reaction!) and setting the derivative to zero yields:
   $$\gamma(1 - \varepsilon)\bigl[(1 - \varepsilon)\theta\_{\mathrm{PO}} - \mu\bigr] - \alpha = 0 \implies \theta\_{\mathrm{PO}} = \frac{\mu}{1 - \varepsilon} + \frac{\alpha}{\gamma(1 - \varepsilon)^2}$$

Compare $\theta\_{\mathrm{PS}}$ and $\theta\_{\mathrm{PO}}$: whenever $\alpha \neq 0$ and $\varepsilon \neq 0$, **retraining to stability ($\theta\_{\mathrm{PS}}$) converges to a strictly suboptimal policy ($\theta\_{\mathrm{PS}} \neq \theta\_{\mathrm{PO}}$)** because myopic retraining ignores the derivative $\frac{\partial \mathcal{D}(\theta)}{\partial \theta}$!

Even worse, what if the environment is highly reactive? Under Wasserstein-1 distribution sensitivity $W\_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon\lVert\theta - \theta'\rVert\_2$, $\gamma$-strong convexity, and $\beta$-smoothness of the loss, Perdomo et al. proved that Repeated Risk Minimization contracts via:

$$\lVert\theta\_{t+1} - \theta\_{\mathrm{PS}}\rVert\_2 \le \left(\frac{\varepsilon\beta}{\gamma}\right) \lVert\theta\_t - \theta\_{\mathrm{PS}}\rVert\_2$$

Thus, RRM converges **if and only if the contraction ratio satisfies $r\_{\mathrm{RRM}} \triangleq \frac{\varepsilon\beta}{\gamma} < 1$**. Whenever $\frac{\varepsilon\beta}{\gamma} \ge 1$, retraining the model on its own induced data causes violent self-reinforcing oscillations that destabilize the real world (`--defect performativity`).

---

## Step 3 — Auditable Laboratory in Rust and Exercises

In `crates/stochology-core/src/layers.rs`, the three bridges are implemented in `MengAudit`, `ManskiBounds`, `NonlinearSummary`, and `PerformativeSystem`. Run Laboratory 1 from your terminal:

~~~bash
cargo run --release --bin labs -- --layers
~~~

~~~text
=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===
  [Bridge L1->L2] Meng (CPS n=15992 vs NSW n=260): rho_R_Y=+0.1334, selection_err=+164.65, n_eff_Meng=2842.70 | Manski MNAR=[5463.2, 13835.3]
  [Bridge L2->L3] Jensen Gap (bank_calls, 164 days): f(E[X])=1066924927.55, E[f(X)]=1075958764.73, Delta_J=+9033837.19, median=1025216361.00, q05=830290416.50, q95=1481646385.00, OOS_CRPS=90551681.37
  [Bridge L3->L1] Performative Prediction: theta_SL=1.5000, theta_PS=1.8750, theta_PO=2.0312, PR(PS)=-0.8125 vs PR(PO)=-0.8203, r_RRM=0.300 (converges=true), oscillation_violated=[1.0, 0.0, 1.0, 0.0, 1.0]
~~~

Examine the three lines of output:
1. **On Bridge $L\_1 \to L\_2$**, comparing the observational CPS sample ($n = 15\text{,}992$) against the randomized NSW benchmark reveals $\rho\_{R,Y} = +0.1334$, collapsing the effective sample size to $n\_{\mathrm{eff}} = 2\text{,}842.70$, while the assumption-free Manski `MNAR` bounds trap the true earnings within $[5\text{,}463.2,\ 13\text{,}835.3]$.
2. **On Bridge $L\_2 \to L\_3$**, across the $164$ days of `bank_calls.csv`, the nonlinear congestion loss exhibits a positive Jensen's Gap of $\Delta\_J = +9\text{,}033\text{,}837.19$ above the naive plug-in mean evaluation $f(\mathbb{E}[X])$, and reports the full quantile profile ($q\_{05}$, median, $q\_{95}$) alongside the out-of-sample $\mathrm{CRPS} = 9.055 \times 10^7$.
3. **On Bridge $L\_3 \to L\_1$**, with $\mu = 1.0, \varepsilon = 0.20, \alpha = 0.50, \gamma = 1.0, \beta = 1.50$, passive learning chooses $\theta\_{\mathrm{SL}} = 1.5000$, Repeated Risk Minimization converges ($r\_{\mathrm{RRM}} = \frac{0.20 \times 1.50}{1.0} = 0.300 < 1$) to $\theta\_{\mathrm{PS}} = 1.8750$ ($\mathrm{PR} = -0.8125$), whereas the Stackelberg leader achieves $\theta\_{\mathrm{PO}} = 2.0312$ with strictly lower Performative Risk ($\mathrm{PR} = -0.8203$). When the contraction condition is violated, RRM oscillates endlessly (`[1.0, 0.0, 1.0, 0.0, 1.0]`).

### Exercises

1. **(Concept — Bridge Diagnosis)** Classify each of the following failures by its primary bridge ($L\_1 \to L\_2$, $L\_2 \to L\_3$, or $L\_3 \to L\_1$):
   - *(a)* A wartime air force armors the fuselage sections where returning bombers have the most bullet holes, forgetting that planes hit in the engines never returned to be counted (Abraham Wald's survivorship paradox).
   - *(b)* An agricultural planner computes the crop yield at the average annual rainfall, ignoring that a drought year followed by a flood year destroys the harvest in both years.
   - *(c)* A navigation app sends ten thousand drivers down a quiet residential side street because its historical travel-time model predicted zero congestion there, creating a two-hour gridlock.
2. **(Calculation — Meng's Identity)** Suppose a national health registry covers $n = 2\text{,}000\text{,}000$ self-enrolled citizens out of a population of $N = 10\text{,}000\text{,}000$, with outcome standard deviation $\sigma\_Y = 50$ and a tiny selection correlation $\rho\_{R,Y} = +0.04$. Using Meng's Identity, calculate the exact selection bias $\bar{Y}\_n - \bar{Y}\_N$. Why did a sample of two million people fail to make this error vanish?
3. **(Calculation — Jensen's Gap)** A hospital emergency room has capacity $\mu = 20$ patients/hour and waiting time proportional to $W(\lambda) = \frac{\lambda}{\mu - \lambda}$. Compare $W(\mathbb{E}[\lambda])$ against $\mathbb{E}[W(\lambda)]$ when arrival rate $\lambda$ is $10$ patients/hour on half the shifts and $18$ patients/hour on the other half. What is $\Delta\_J$?
4. **(Laboratory — Performativity Gate)** Run `cargo run --release --bin gates -- --defect performativity` and verify that the gate exits with code `1` when $\frac{\varepsilon\beta}{\gamma} \ge 1$.

### Chapter Summary

- Every model operates inside the closed feedback loop **World ($L\_1$) $\to$ Model ($L\_2$) $\to$ Agent ($L\_3$) $\to$ World ($L\_1$)**.
- **Bridge $L\_1 \to L\_2$** is governed by Rubin's missingness taxonomy (`MCAR`, `MAR`, `MNAR`), **Meng's Identity** ($\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y}\sqrt{\frac{N-n}{n}}\sigma\_Y$), and **Manski's non-parametric bounds**.
- **Bridge $L\_2 \to L\_3$** is governed by **Jensen's Inequality** ($\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \frac{1}{2}f''(\mu)\mathrm{Var}(X)$) and strictly proper distributional scoring via **$\mathrm{CRPS}$**.
- **Bridge $L\_3 \to L\_1$** is governed by **Performative Prediction**, separating passive fit ($\theta\_{\mathrm{SL}}$), myopic retraining stability ($\theta\_{\mathrm{PS}}$), and Stackelberg optimality ($\theta\_{\mathrm{PO}}$), subject to the strict RRM contraction barrier $\frac{\varepsilon\beta}{\gamma} < 1$.
