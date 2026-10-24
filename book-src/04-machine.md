# 4. The Almost-Perfect Machine and First-Class Authentic Model Form

> **Learning Objectives.** By the end of this chapter, you will be able to explain why high predictive accuracy can hide a "Clever Hans" model; define the $(\varepsilon, \delta)$-Almost-Perfect Machine $M = (T, \Phi, \theta)$ and derive its five mathematical properties (`P1` Barron universal density, `P2` DPI $\delta$-sufficiency, `P3` Fano/minimax rate optimality, `P4` Telgarsky depth-$D$ compositional efficiency, and `P5` Gram effective dimension $d\_{\mathrm{eff}}$); construct the First-Class Authentic Model Form ($\delta(x) + \hat{S}^{\ast}$); and verify simultaneous closure of the six numerical identities (`IdentityClosureAudit`) and the five-part authenticity criterion $A(\text{model})$.

---

## Step 1 — Intuition and the Real-World Paradox

### Clever Hans, the Horse That Could Count

In Berlin at the turn of the twentieth century, a horse named **Clever Hans** became an international sensation. If his trainer asked, *"What is $7 + 5$?"*, Hans tapped his hoof twelve times and stopped. Commissions of veterinarians, circus trainers, and zoologists inspected the stable for hidden wires or trick signals and found none; Hans performed even when strangers asked the questions.

Finally, psychologist **Oskar Pfungst** ran an experiment that changed only one thing: he asked Hans questions **whose answers the human questioner did not know**. Instantly, Hans's accuracy collapsed to random guessing.

What had Hans actually learned? He had not learned arithmetic. He had learned to watch the tiny, unconscious forward lean of the questioner's posture as the hoof taps approached the expected count, and the micro-relaxation of the questioner's eyebrows on the final tap!

Modern machine learning is filled with Clever Hans horses:
- A deep convolutional network trained to detect pneumonia on chest X-rays achieves an area under the curve ($\mathrm{AUC}$) of $0.96$—by reading the portable metal hospital token in the corner of the image rather than the lungs. When deployed at a second hospital across town, it fails catastrophically.
- A tropical cyclone rapid-intensification model fits historical storm tracks closely by latching onto a basin-specific latitude-season correlation that shifts whenever the multi-decadal ocean oscillation flips regime.
- A massive over-parameterized model uses $100$ colinear dictionary features that merely duplicate one another, hiding structural model error inside tweaked weights.

This raises two foundational questions for the Stochology Protocol:
1. **When is a representation and approximation architecture $M = (T, \Phi, \theta)$ mathematically well-conditioned (`AlmostPerfectMachine`)?**
2. **When does a fitted model deserve to be called *authentic* (`AuthenticModelForm`) rather than a Clever Hans illusion?**

---

## Step 2 — Foundations and Mathematical Formalization

### 4.1 The $(\varepsilon, \delta)$-Almost-Perfect Machine $M = (T, \Phi, \theta)$

#### Foundations: Anatomy of a Learning Machine

Every parametric or semi-parametric model in statistics and machine learning can be written as a triple:

$$M = (T,  \Phi,  \theta), \qquad M(x) = \sum\_{j=1}^W \theta\_j  \phi\_j\bigl(T(x)\bigr) = \theta^\top \Phi\bigl(T(x)\bigr)$$

where:
- **$T: \mathcal{X} \to \mathcal{Z}$ is the Representation Map (Encoder):** It extracts a compressed state $z = T(x)$ from the raw high-dimensional observation $x \in \mathbb{R}^d$, stripping away irrelevant sensor noise.
- **$\Phi = [\phi\_1, \dots, \phi\_W]^\top$ is the Atomic Dictionary (Basis):** A collection of $W$ nonlinear basis functions or compositional feature atoms evaluated on $T(x)$.
- **$\theta \in \mathbb{R}^W$ is the Parameter Vector:** The linear readout weights combining the $W$ atoms into a final prediction.

In `crates/stochology-lenses/src/machine.rs`, `AlmostPerfectMachine` audits whether $M = (T, \Phi, \theta)$ satisfies five rigorous mathematical laws (`P1` through `P5`) simultaneously:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1080" height="220" viewBox="0 0 1080 220" fill="none" role="img" aria-labelledby="title-fig-04-machine">
  <title id="title-fig-04-machine">Architecture Pipeline of the (epsilon, delta)-Almost-Perfect Machine M = (T, Phi, theta)</title>
  <defs>
  <marker id="m-sky" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="m-violet" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--violet)"/>
  </marker>
  <marker id="m-emerald" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--emerald)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1078" height="218" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="14" y="48" width="136" height="124" rx="10" fill="var(--surface)" stroke="var(--border)" stroke-width="2"/>
  <text x="82" y="86" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11" font-weight="700">OBSERVATION</text>
  <text x="82" y="114" text-anchor="middle" fill="var(--ink)" font-size="16" font-weight="800">Input X</text>
  <text x="82" y="142" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">x ∈ ℝᵈ</text>
  <line x1="150" y1="110" x2="168" y2="110" stroke="var(--sky)" stroke-width="2.5" marker-end="url(#m-sky)"/>
  <rect x="172" y="28" width="282" height="164" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.2"/>
  <text x="313" y="60" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11" font-weight="800">PROPERTY P2 · ENCODER</text>
  <text x="313" y="92" text-anchor="middle" fill="var(--ink)" font-size="15.5" font-weight="800">Representation T(X)</text>
  <text x="313" y="126" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">• DPI Loss Δ_DPI ≤ δ</text>
  <text x="313" y="152" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">• SDPI: η_χ² ≤ η_KL ≤ η_TV</text>
  <line x1="454" y1="110" x2="472" y2="110" stroke="var(--violet)" stroke-width="2.5" marker-end="url(#m-violet)"/>
  <rect x="476" y="28" width="282" height="164" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2.2"/>
  <text x="617" y="60" text-anchor="middle" fill="var(--violet)" class="mono" font-size="11" font-weight="800">PROPERTIES P4 &amp; P5 · BASIS</text>
  <text x="617" y="92" text-anchor="middle" fill="var(--ink)" font-size="15.5" font-weight="800">Dictionary Φ(T(X))</text>
  <text x="617" y="126" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">• Telgarsky 2^D Regions</text>
  <text x="617" y="152" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">• Gram d_eff ≥ 0.80 W</text>
  <line x1="758" y1="110" x2="776" y2="110" stroke="var(--emerald)" stroke-width="2.5" marker-end="url(#m-emerald)"/>
  <rect x="780" y="28" width="286" height="164" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2.2"/>
  <text x="923" y="60" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">PROPERTIES P1 &amp; P3 · READOUT</text>
  <text x="923" y="92" text-anchor="middle" fill="var(--ink)" font-size="15.5" font-weight="800">Readout θᵀΦ(T(X))</text>
  <text x="923" y="126" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">• Barron Rate ≤ C_f² / W</text>
  <text x="923" y="152" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">• Minimax MSE ≈ σ²W / n</text>
</svg>
  <figcaption><strong>Figure 4.1 — Architecture Pipeline of the Almost-Perfect Machine M = (T, Φ, θ)</strong></figcaption>
</figure>

#### Property P1 — Universal Barron Density and Escaping the Curse of Dimensionality

If you try to approximate an arbitrary smooth function $f(x)$ in $d$ dimensions using a classic grid of local splines or polynomials, the number of grid cells needed to achieve accuracy $\varepsilon$ grows exponentially as $(1/\varepsilon)^d$. In $d = 50$ dimensions, that **curse of dimensionality** requires more parameters than there are atoms in the universe.

How is it possible for neural and dictionary expansions of width $W$ to approximate high-dimensional functions at all? In 1993, Yale information theorist **Andrew Barron** proved a remarkable theorem: if the Fourier transform $\tilde{f}(\omega)$ of the target function $f$ has a finite first spectral moment $C\_f \triangleq \int \lVert\omega\rVert\_1 |\tilde{f}(\omega)| d\omega < \infty$ (the **Barron norm**), then a dictionary of $W$ ridge atoms approximates $f$ at a **dimension-independent rate**:

$$\text{P1 (Barron Rate):} \qquad \sup\_{x \in \mathcal{X}} \bigl|M(x) - f(x)\bigr| \le \varepsilon \quad \text{and} \quad \lVert M - f \rVert\_{L\_2(\mu)}^2 \le \frac{C\_f^2}{W}$$

Notice that the ambient dimension $d$ does not appear in the denominator $W$! Property `P1` audits both the uniform supremum error ($\le \varepsilon$) and the empirical $L\_2^2$ error against the exact Barron ceiling $C\_f^2 / W$.

#### Property P2 — Informational $\delta$-Sufficiency (DPI Compression)

A representation $T(X)$ must compress raw inputs $X$ without throwing away the signal that predicts the target state of the world $W$. By the Data Processing Inequality (Lens 1), $I(W; T(X)) \le I(W; X)$ always holds. If $I(W; T(X)) = I(W; X)$, the statistic $T(X)$ is a **sufficient statistic** in the classical Fisher-Neyman sense.

In real noisy pipelines, we demand **$(\delta)$-Approximate Sufficiency**:

$$\text{P2 ($\delta$-Sufficiency):} \qquad \Delta\_{\mathrm{DPI}} \triangleq I(W; X) - I(W; T(X)) \le \delta \qquad (\text{with } \Delta\_{\mathrm{DPI}} \ge 0)$$

coupled with a valid Strong Data Processing Inequality hierarchy ($\eta\_{\chi^2} \le \eta\_{\mathrm{KL}} \le \eta\_{\mathrm{TV}}$). This guarantees that $T(X)$ sheds nuisance entropy $H(X \mid W)$ while retaining almost every nat of predictive mutual information about $W$.

#### Property P3 — Fano / Minimax Statistical Rate Optimality

Suppose we fit $W$ parameters from $n$ noisy observations with irreducible noise variance $\sigma^2$. Information-theoretic minimax theory (Fano's method and Assouad's lemma) proves that the optimal mean squared estimation error scales as:

$$R\_{\text{minimax}} = \frac{\sigma^2 W}{n}$$

Property `P3` audits the empirical ratio $\frac{\mathrm{MSE}}{\sigma^2 W / n}$:
- If $\mathrm{MSE} \gg 2.5 \frac{\sigma^2 W}{n}$, the estimator is **statistically inefficient or underfitting**;
- If $\mathrm{MSE} < 0.25 \frac{\sigma^2 W}{n}$ in a noisy world where Fano's floor $P\_e^{\min} > 0$, the pipeline is **memorizing noise or suffering from target leakage**!

Thus `P3` requires $0.25 \le \frac{\mathrm{MSE}}{\sigma^2 W / n} \le 2.5$ alongside a positive Fano lower bound $P\_e^{\min} > 0$.

#### Property P4 — Telgarsky Depth-$D$ Compositional Efficiency

Why use compositional depth $D$ instead of a single flat layer? In 2016, **Matus Telgarsky** constructed an explicit family of sawtooth compositions (related to the tent map $g(u) = 1 - |2u - 1|$) proving that composing $D$ affine-nonlinear stages folds the input space in half at every layer, generating **$2^D$ distinct affine linear regions**:

$$\text{P4 (Telgarsky Efficiency):} \qquad N\_{\text{regions}}(D) = 2^D, \qquad \frac{2^D}{2D} \ge 4.0$$

At depth $D = 6$, a composition of only $2D = 12$ elementary parameters creates $2^6 = 64$ linear pieces (efficiency ratio $64 / 12 = 5.33\times \ge 4.0$), whereas a shallow 1-layer network would need $64$ separate units to match the same oscillatory resolution.

#### Property P5 — Atomic Completeness and Gram Effective Dimension ($d\_{\mathrm{eff}}$)

Finally, what if a model declares $W = 8$ basis atoms $\phi\_1, \dots, \phi\_8$, but $6$ of them are nearly identical copies of one another? Then the dictionary is bloated and ill-conditioned: parameter estimates $\hat{\theta}$ will swing wildly under tiny perturbations of the data.

To audit whether the $W$ atoms are genuinely independent, we compute the empirical **Gram Matrix** $G \in \mathbb{R}^{W \times W}$ of inner products across the sample:

$$G\_{ij} = \frac{1}{n}\sum\_{k=1}^n \tilde{\phi}\_i(x\_k) \tilde{\phi}\_j(x\_k)$$

and measure its **Participation Ratio / Effective Dimension ($d\_{\mathrm{eff}}$)**:

$$\text{P5 (Gram Completeness):} \qquad d\_{\mathrm{eff}}(G) \triangleq \frac{(\mathrm{tr} G)^2}{\lVert G \rVert\_F^2} = \frac{\left(\sum\_{j=1}^W \lambda\_j\right)^2}{\sum\_{j=1}^W \lambda\_j^2} \ge 0.80  W, \qquad \max\_{i \neq j} |G\_{ij}| < 0.15$$

Notice why $d\_{\mathrm{eff}}(G)$ is such a clean geometric diagnostic:
- If all $W$ atoms are mutually orthonormal ($G = I\_W$), then $\mathrm{tr} G = W$ and $\lVert G \rVert\_F^2 = W$, so **$d\_{\mathrm{eff}} = \frac{W^2}{W} = W$** (full rank efficiency).
- If all $W$ atoms are collinear clones of a single feature ($\lambda\_1 = W, \lambda\_2 = \dots = \lambda\_W = 0$), then **$d\_{\mathrm{eff}} = 1$** regardless of how large $W$ is!

---

### 4.2 First-Class Authentic Model Form ($\delta(x) + \hat{S}^{\ast}$)

Even when a machine $M = (T, \Phi, \theta)$ is well-conditioned (`P1`–`P5`), it can still act like Clever Hans if one of its atoms is a regime-specific spurious correlation rather than an invariant causal mechanism.

Therefore, Gate `G8` requires every model promoted to production to take the **First-Class Authentic Model Form (`AuthenticModelForm`)**:

$$y(x) = \underbrace{f\_{\hat{S}^{\ast}}\bigl(x\_{\hat{S}^{\ast}}, \theta\_{\hat{S}^{\ast}}\bigr)}\_{\text{Surviving Invariant Causal Core ($\hat{S}^{\ast}$)}} + \underbrace{\delta(x)}\_{\text{Anchored Kennedy-O'Hagan GP}} + \varepsilon$$

Let us unpack the four mandatory pillars of `AuthenticModelForm`:

1. **Invariant Causal Core ($\hat{S}^{\ast}$) Plus Anchored Discrepancy ($\delta(x)$):**
   Only predictors that survive multi-environment nonlinear ICP and NILE/IRM invariance testing enter the parametric core $\hat{S}^{\ast}$. Any remaining smooth discrepancy between the causal core and reality is quarantined inside an anchored Kennedy-O'Hagan Gaussian Process $\delta(x) \sim \mathcal{GP}(0, k\_{\text{anchored}})$ so that missing physics never corrupts $\theta\_{\hat{S}^{\ast}}$.

2. **Explicit Naming of Discarded Spurious Atoms (`DiscardedAtomRecord`):**
   In standard academic papers, when a researcher tries ten features and drops four, the four dropped features vanish into the wastebasket without a trace. In Stochology, **every candidate atom rejected by invariance testing must be recorded explicitly by name** in `discarded_atoms` alongside its cross-environment invariance penalty ratio and the exact reason it was discarded (for example, `atom_1_regime_shifted_lat_proxy` on `atlantic_storms.csv`, whose NILE/IRM penalty jumps $97\times$ higher than the physical thermodynamic intensity core!).

3. **Simultaneous Closure of the Six Exact Numerical Identities (`IdentityClosureAudit`):**
   The protocol checks that six mathematical identities close simultaneously to floating-point tolerance (`all_six_closed() == true`):
   - **Identity 1 (Jensen's Gap Closure):** $|\Delta\_J - (\mathbb{E}[f(X)] - f(\mathbb{E}[X]))| \le 10^{-6}$;
   - **Identity 2 (CORP-Murphy PAV Closure):** $|\mathrm{BS} - (\mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0)| \le 10^{-9}$;
   - **Identity 3 (Meng's Selection Bias Closure):** $|\Delta\_{\text{sel}} - \rho\_{R,Y}\sqrt{\frac{N-n}{n}}\sigma\_Y| \le 10^{-6}$;
   - **Identity 4 (Cryptographic Pre-Registration Seal):** The recomputed FIPS 180-4 SHA-256 digest of the design payload matches the 64-character hex seal character-for-character;
   - **Identity 5 (Info-Gap Zero-ing & Global Wald Floor):** $|\hat{\alpha}(q, R(q, \tilde{u}))| \le 10^{-12}$ and the global Wald floor holds outside $\mathcal{U}(\hat{\alpha}, \tilde{u})$;
   - **Identity 6 (Findings Conservation Law):** $\text{registered} = \text{used} + \text{unused}$ with $K \ge 1$ and zero undeclared cherry-picking.

4. **The Five-Part Operational Authenticity Criterion $A(\text{model})$ (`AuthenticityCriterionA`):**
   Finally, the boolean certificate `authentic == true` is granted if and only if all five operational gates hold simultaneously:
   $$A(\text{model}) = c\_1(\text{Invariance of } \hat{S}^{\ast}) \land c\_2(\text{Anchored GP } \delta(x)) \land c\_3(\text{Named Discarded Atoms}) \land c\_4(\text{6 Identities Closed}) \land c\_5(\text{OOT/OOD Survival})$$

---

## Step 3 — Auditable Laboratory in Rust and Exercises

Both `AlmostPerfectMachine` and `AuthenticModelForm` are audited in `labs --lenses` and enforced as a blocking check in **Gate `G8`** (`crates/stochology-cli/src/bin/gates.rs`). Run `gates` and its negative defect proof `--defect machine` from your terminal:

~~~bash
cargo run --release --bin gates
cargo run --release --bin gates -- --defect machine
~~~

~~~text
  [Almost-Perfect Machine M=(T,Phi,theta)] sup_err=0.0234 (Barron C_f=5.60), DPI_gap=0.0297 nats, regions=64 (ratio=5.3x), d_eff=8.00/8, P1..P5=true
  [Lens 4: Invariance] ICP S*=[0], env_OLS_penalty=["0.0001", "15.8059"] (ChowF=[0.18, 23437.0]), Storms_nonlinear_NILE_IRM=["0.0011", "0.1072"], Lambda*=4.752 (RV=0.511) | AuthenticForm(S*=[0], discarded='atom_1_regime_shifted_lat_proxy', A=true)
~~~

Notice how the numbers tie the theory together:
- The 8-atom orthonormal Hermite-Fourier dictionary achieves uniform supremum error $0.0234$ under Barron norm $C\_f = 5.60$ (`P1`), loses only $\Delta\_{\mathrm{DPI}} = 0.0297\text{ nats}$ in representation compression (`P2`), matches the minimax noise floor (`P3`), generates $2^6 = 64$ linear regions ($5.3\times$ shallow efficiency, `P4`), and attains perfect Gram effective dimension $d\_{\mathrm{eff}} = 8.00 / 8$ (`P5`).
- On `atlantic_storms.csv`, `AuthenticModelForm` retains only the invariant thermodynamic core $\hat{S}^{\ast} = [0]$ (`Storms_nonlinear_NILE_IRM = 0.0011`, Chow $F = 0.18$) and explicitly quarantines and logs the non-invariant proxy `atom_1_regime_shifted_lat_proxy` (`0.1072`, Chow $F = 23\text{,}437.0$). When `--defect machine` injects a spurious non-invariant atom into $\hat{S}^{\ast}$ or breaks Gram orthogonality, Gate `G8` immediately halts execution with exit code `1`.

### Exercises

1. **(Concept — Clever Hans vs. Authentic Form)** A credit-scoring startup adds "phone battery percentage at the moment of loan application" to its gradient-boosted tree and sees validation $\mathrm{AUC}$ rise by $+0.015$.
   - *(a)* Using Pearl's Causal Ladder and ICP across environments, explain what happens the moment applicants read on social media that plugging in their phone before applying boosts approval odds.
   - *(b)* How does `AuthenticModelForm` handle this feature?
2. **(Calculation — Gram Effective Dimension)** Suppose a $W = 4$ feature dictionary has Gram matrix eigenvalues $\lambda = (3.7, 0.1, 0.1, 0.1)$. Compute $\mathrm{tr} G$, $\lVert G \rVert\_F^2 = \sum \lambda\_j^2$, and the effective dimension $d\_{\mathrm{eff}} = \frac{(\mathrm{tr} G)^2}{\lVert G \rVert\_F^2}$. Does this dictionary pass Property `P5` ($d\_{\mathrm{eff}} \ge 0.80 W = 3.20$)? Why or why not?
3. **(Calculation — Telgarsky Regions)** How many affine linear regions $2^D$ does a sawtooth network of depth $D = 8$ generate, and what is its efficiency ratio $2^D / (2D)$ compared to a shallow network?
4. **(Laboratory — Defect Proof)** Run `cargo run --release --bin gates -- --defect machine` and inspect why Gate `G8` rejects a machine whose discarded atoms are hidden or whose Gram matrix is rank-deficient.

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Clever Hans vs. Authentic Form:**
   - *(a)* Phone battery percentage is a non-causal proxy ($X\_{\text{proxy}}$) rather than a direct causal parent of loan repayment ($Y$). Once applicants intervene ($\mathrm{do}(\text{Battery} = 100\%)$), the conditional distribution $P^e(Y \mid \text{Battery})$ shifts across environments $e \in \mathcal{E}$ (Goodhart/Lucas performative breakdown).
   - *(b)* `AuthenticModelForm` rejects the battery atom during multi-environment ICP/NILE testing, excludes it from $\hat{S}^{\ast}$, and logs it by name in `discarded_atoms` (`DiscardedAtomRecord`) with its invariance penalty ratio.
2. **Gram Effective Dimension Calculation:**
   - $\mathrm{tr} G = 3.7 + 0.1 + 0.1 + 0.1 = 4.0$.
   - $\lVert G \rVert\_F^2 = 3.7^2 + 0.1^2 + 0.1^2 + 0.1^2 = 13.69 + 0.03 = 13.72$.
   - $d\_{\mathrm{eff}} = \frac{4.0^2}{13.72} = \frac{16.0}{13.72} \approx 1.166 < 3.20$.
   - It **fails Property `P5`** because almost all variance is concentrated along a single collinear direction ($\lambda\_1 = 3.7$).
3. **Telgarsky Regions Calculation:**
   At depth $D = 8$, $N\_{\text{regions}} = 2^8 = 256$ affine linear pieces using $2D = 16$ elementary parameters, giving an efficiency ratio of $\frac{256}{16} = 16.0\times \ge 4.0$.
4. **Machine Defect Proof:** Running `cargo run --release --bin gates -- --defect machine` triggers `[G8 BLOCKED]` when `P1`–`P5` or `AuthenticModelForm` fails and exits with code `1`.

</details>

### Chapter Summary

- An **$(\varepsilon, \delta)$-Almost-Perfect Machine $M = (T, \Phi, \theta)$** requires universal Barron density (`P1`), DPI $\delta$-sufficiency (`P2`), Fano/minimax rate optimality (`P3`), Telgarsky $2^D$ compositional depth efficiency (`P4`), and Gram effective dimension $d\_{\mathrm{eff}} \ge 0.80 W$ (`P5`).
- The **First-Class Authentic Model Form ($\delta(x) + \hat{S}^{\ast}$)** pairs the invariant causal core $\hat{S}^{\ast}$ with an anchored Kennedy-O'Hagan discrepancy $\delta(x)$, explicitly names every discarded spurious atom (`DiscardedAtomRecord`), verifies simultaneous closure of the **6 numerical identities**, and enforces the 5-part criterion $A(\text{model})$.
