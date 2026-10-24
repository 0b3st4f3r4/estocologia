# 8. The Epistemological Card (`EpistemologicalCard`)

> **Learning Objectives.** By the end of this chapter, you will be able to read, author, and audit a complete **Epistemological Card** (`cards/*.yaml`); explain why each of its five structured sections exists; trace the internal cross-field mathematical consistency checks enforced by `EpistemologicalCard::audit()`; and understand how the card replaces ambiguous prose reports with a self-verifying contract.

---

## Step 1 — Intuition and the Real-World Paradox

### Why Airline Pilots Use Checklists Instead of Essays

Before taking off in a modern commercial airliner, the captain and first officer do not sit in the cockpit and write an expressive literary essay about how confident they feel in the weather and the engines. They execute a **structured pre-flight checklist**:
- Fuel quantity and cross-feed valves: *checked against flight plan.*
- Flap setting and stabilizer trim: *verified in green band.*
- Engine failure abort speed ($V\_1$): *computed and called out explicitly.*

Why did aviation adopt rigid checklists after the 1935 Boeing Model 299 crash? Because under cognitive load, time pressure, and institutional incentives, even brilliant experts skip uncomfortable checks when left to free-form narration.

An **Epistemological Card** (`EpistemologicalCard` in `crates/stochology-core/src/card.rs`) is the pre-flight checklist of quantitative science and decision-making. Every technical claim in the Stochology Protocol lives as a self-contained YAML document (`cards/*.yaml`) with `contract_version: "1.0"`.

Crucially, an Epistemological Card is **not a passive form where an analyst can type arbitrary numbers to make the parser happy.** When `EpistemologicalCard::audit()` reads a card from disk, it acts as an adversarial mathematical auditor: it recomputes Jensen's Gap, recomputes the Moulton-Campbell design effect, checks that the four variance-budget shares sum to $1.0$, verifies the CORP-Murphy PAV identity $\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$, checks the cost-loss threshold $p^{\ast} = \frac{C}{\eta L}$ against the recommended act (`protect` vs. `do_not_protect`), verifies that the modeler and adversarial verifier are distinct agents, and recomputes the SHA-256 cryptographic digest of the underlying CSV dataset on disk!

---

## Step 2 — Anatomy and Mathematical Audit of the Five Sections

Every Epistemological Card begins with top-level routing metadata (`contract_version`, `id`, `layer`, `uncertainty`) followed by **five mandatory sections**:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1100" height="420" viewBox="0 0 1100 420" fill="none" role="img" aria-labelledby="title-fig-08-card">
  <title id="title-fig-08-card">Structural Anatomy of the EpistemologicalCard and its Adversarial Mathematical Audit</title>
  <defs>
  <marker id="c-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1098" height="418" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="20" y="18" width="690" height="384" rx="14" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.4"/>
  <text x="44" y="48" fill="var(--sky)" class="mono" font-size="12.5" font-weight="800" letter-spacing="0.06em">EPISTEMOLOGICAL CARD · contract_version: "1.0"</text>
  <text x="44" y="70" fill="var(--muted)" class="mono" font-size="11.5">id: "card-..."  |  layer: L1/L2/L3  |  uncertainty: aleatory/...</text>
  <line x1="20" y1="84" x2="710" y2="84" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="40" y="96" width="650" height="50" rx="8" fill="var(--surface)" stroke="var(--border)" stroke-width="1.4"/>
  <text x="58" y="117" fill="var(--ink)" class="mono" font-size="12.5" font-weight="800">1. decision_maker_question</text>
  <text x="58" y="137" fill="var(--muted)" font-size="11.5">Who decides? Concrete action &amp; Resolution R_data ≥ R_decision (Φ₅)</text>
  <rect x="40" y="156" width="650" height="50" rx="8" fill="var(--surface)" stroke="var(--border)" stroke-width="1.4"/>
  <text x="58" y="177" fill="var(--ink)" class="mono" font-size="12.5" font-weight="800">2. epistemological_header</text>
  <text x="58" y="197" fill="var(--muted)" font-size="11.5">Process, Jensen Δ_J, Moulton-Campbell n_eff (Φ₂), Rubin, Pearl Rung</text>
  <rect x="40" y="216" width="650" height="50" rx="8" fill="var(--surface)" stroke="var(--border)" stroke-width="1.4"/>
  <text x="58" y="237" fill="var(--ink)" class="mono" font-size="12.5" font-weight="800">3. uncertainty_budget_and_authorship</text>
  <text x="58" y="257" fill="var(--muted)" font-size="11.5">4 Variance Shares = 1.0, Info-Gap α̂ + Wald Floor, Cross-Authorship</text>
  <rect x="40" y="276" width="650" height="50" rx="8" fill="var(--surface)" stroke="var(--border)" stroke-width="1.4"/>
  <text x="58" y="297" fill="var(--ink)" class="mono" font-size="12.5" font-weight="800">4. data_and_audit_contract</text>
  <text x="58" y="317" fill="var(--muted)" font-size="11.5">64-Hex SHA-256, seed = 2026 Null (Φ₁), K ≥ 1, e-process, CORP PAV</text>
  <rect x="40" y="336" width="650" height="50" rx="8" fill="var(--surface)" stroke="var(--border)" stroke-width="1.4"/>
  <text x="58" y="357" fill="var(--ink)" class="mono" font-size="12.5" font-weight="800">5. decision_rule_and_scope</text>
  <text x="58" y="377" fill="var(--muted)" font-size="11.5">Cromwell p, p* = C/(ηL), Act, Chouldechova Audit &amp; Exit Clause (Φ₄)</text>
  <line x1="710" y1="121" x2="772" y2="174" stroke="var(--sky)" stroke-width="2" marker-end="url(#c-arrow)"/>
  <line x1="710" y1="181" x2="772" y2="192" stroke="var(--sky)" stroke-width="2" marker-end="url(#c-arrow)"/>
  <line x1="710" y1="241" x2="772" y2="210" stroke="var(--sky)" stroke-width="2" marker-end="url(#c-arrow)"/>
  <line x1="710" y1="301" x2="772" y2="228" stroke="var(--sky)" stroke-width="2" marker-end="url(#c-arrow)"/>
  <line x1="710" y1="361" x2="772" y2="246" stroke="var(--sky)" stroke-width="2" marker-end="url(#c-arrow)"/>
  <rect x="780" y="120" width="296" height="184" rx="12" fill="var(--emerald-bg)" stroke="var(--emerald)" stroke-width="2.4"/>
  <text x="928" y="152" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">ADVERSARIAL GATEKEEPER</text>
  <text x="928" y="180" text-anchor="middle" fill="var(--ink)" class="mono" font-size="13.5" font-weight="800">Card::audit()</text>
  <text x="928" y="208" text-anchor="middle" fill="var(--muted)" font-size="11.5">• Recomputes Δ_J &amp; n_eff</text>
  <text x="928" y="230" text-anchor="middle" fill="var(--muted)" font-size="11.5">• Verifies CORP Identity</text>
  <text x="928" y="252" text-anchor="middle" fill="var(--muted)" font-size="11.5">• Checks CSV SHA-256</text>
  <text x="928" y="280" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Gates G1–G5 Enforced</text>
</svg>
  <figcaption><strong>Figure 8.1 — Structural Anatomy of the Epistemological Card and its Adversarial Audit</strong></figcaption>
</figure>

Let us walk through each section and see the exact mathematical invariants it locks in place.

---

### 8.1 Section 1: `decision_maker_question`

~~~yaml
decision_maker_question:
  who_decides: "Director of Retail Banking Operations (North American Call Center)"
  altered_action: "Schedule 18 overflow customer-service agents on high-load business days"
  question: "Does the calibrated daily probability of 5-minute call-arrival queue overload exceed the economic protection threshold p* = 0.080?"
~~~

- **Why it exists (Defeating Type III Error and $\Phi\_5$):** A model that changes no concrete human or algorithmic action has an Expected Value of Perfect Information ($\mathrm{EVPI}$) of zero.
- **What `audit()` enforces:**
  1. `who_decides` and `altered_action` must be non-empty concrete declarations;
  2. `question` must be a genuine interrogative sentence ending with `'?'`;
  3. The temporal/spatial resolution of `question` and `altered_action` is cross-checked against the resolution of the dataset under Invariant $\Phi\_5$.

---

### 8.2 Section 2: `epistemological_header`

This section captures the physical and causal mechanics across Layers $L\_1, L\_2,$ and $L\_3$:

1. **`generating_process`:** Explicit description of the physical data-generating mechanism in $L\_1$ (for instance, inhomogeneous Poisson/negative-binomial customer arrivals or thermodynamic tropical cyclone intensification).
2. **`nonlinearity` (Enforcing Prohibition 2):**
   Reports all seven mandatory numbers of `NonlinearSummary`:
   - `payoff_function`, `f_at_mean` ($f(\mathbb{E}[X])$), `mean_of_f` ($\mathbb{E}[f(X)]$), `jensen_gap` ($\Delta\_J$), `median`, `q05`, `q95`, and `crps`.
   - **Audit Check:** Verifies $|\Delta\_J - (\mathbb{E}[f(X)] - f(\mathbb{E}[X]))| \le 10^{-3}$, strict quantile ordering $q\_{05} < \text{median} < q\_{95}$, and $\mathrm{CRPS} > 0$.
3. **`unit_and_margin` (Enforcing Invariant $\Phi\_2$):**
   Reports:
   - `independent_unit` (e.g., `"164 calendar business days"` or `"named Atlantic storm tracks"`),
   - `nominal_sample_size_n` ($n$),
   - `weighted_cluster_mean_m_tilde` ($\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$),
   - `threshold_exceedance_icc` ($\rho\_I(p)$),
   - `effective_sample_size_n_eff` ($n\_{\mathrm{eff}}(p)$), and `unit_margin_of_error`.
   - **Audit Check:** Recomputes $\frac{n}{1 + (\tilde{m} - 1)\rho\_I(p)}$ from the declared fields and blocks the card if `effective_sample_size_n_eff` deviates from the Moulton-Campbell formula!
4. **`noise_physics` (Enforcing Lens 2 & Prohibition 3):**
   Declares `stochastic_regime` (e.g., sub-exponential vs. Pareto infinite variance), `absorbing_states` (ruin or saturation barriers), and `bridge_L3_L1_performativity` (auditing $\theta\_{\mathrm{PS}}, \theta\_{\mathrm{PO}},$ and $r\_{\mathrm{RRM}} < 1$).
5. **`causal_ladder` (Enforcing Lens 4 & $\Phi\_3$):**
   Declares `pearl_rung` (`"1-See"`, `"2-Act"`, or `"3-Imagine"`), `identification_strategy` (e.g., multi-environment NILE/IRM invariance), and `untestable_assumption` (forcing the author to state openly what could break causal identification).

---

### 8.3 Section 3: `uncertainty_budget_and_authorship`

This section enforces honest accounting of where uncertainty comes from and who authored each part of the card:

1. **`variance_budget` (Decomposition of Total Predictive Uncertainty):**
   Splits predictive variance into four non-negative shares:
   - `inputs` (measurement and covariate noise in $L\_1$),
   - `parameters` (finite-sample estimation variance in $L\_2$),
   - `covariance` (cross-term dependence across clusters/factors),
   - `structural_discrepancy_gp` (the share absorbed by the anchored Kennedy-O'Hagan Gaussian Process $\delta(x)$).
   - **Audit Check:** The four shares must be non-negative, `structural_discrepancy_gp` must be strictly positive ($> 0$, acknowledging that no parametric model is structurally perfect), and their sum must equal unity:
     $$\text{inputs} + \text{parameters} + \text{covariance} + \text{structural\_discrepancy\_gp} = 1.0000$$
2. **`dominant_event` (Cromwell's Rule on Tail Risk):**
   Names the single tail event carrying the largest share of expected loss, together with `probability_p`, `base_rate_floor_epsilon` ($\varepsilon\_{\text{floor}}$), and `cost_share`, verifying $\varepsilon\_{\text{floor}} \le p \le 1 - \varepsilon\_{\text{floor}}$.
3. **`info_gap_robustness` (Knightian Safeguard — Chapter 2):**
   Declares `critical_requirement_rc` ($r\_c$), `robustness_horizon_alpha_hat` ($\hat{\alpha} > 0$), `opportuneness_horizon_beta_hat` ($\hat{\beta} > 0$), and `global_maximin_safeguard` (requiring explicit verification of the global Wald floor outside $\mathcal{U}(\hat{\alpha}, \tilde{u})$).
4. **`field_authorship` (Adversarial Cross-Authorship — Filter 4):**
   Declares the identifiers of three distinct roles:
   - `null_model` (who designed the generative null benchmark),
   - `decision_rule` (who proposed the predictive model and action rule),
   - `adversarial_reviewer` (who audited the bridges and tried to break the card).
   - **Audit Check:** Enforces `decision_rule != adversarial_reviewer` and `null_model != decision_rule` so that no single agent grades their own homework.

---

### 8.4 Section 4: `data_and_audit_contract`

This section binds the YAML card to the physical bytes of the repository and closes the anti-p-hacking contract:

1. **`dataset_path` and `dataset_sha256`:**
   Points to the empirical CSV file in `data/*.csv` and records its 64-character lowercase hexadecimal SHA-256 hash. At runtime, `stochology-cli` computes the FIPS 180-4 SHA-256 digest of the file on disk and cross-checks it against both `dataset_sha256` and `data/SHA256SUMS`.
2. **`missingness_mechanism` and `seed_and_environment`:**
   Must declare `MCAR`, `MAR`, or `MNAR` (with Meng/Manski justification) and must include `seed = 2026` and `SplitMix64`.
3. **`null_model` and `preregistration_hash`:**
   Describes the executable null model and provides the 64-character lowercase hexadecimal SHA-256 pre-registration seal.
4. **`attempt_counter_K` and `e_process_multiplicity` (Prohibition 4):**
   Requires $K \ge 1$ and $E\_t / K \ge 1.0$.
5. **`murphy_corp_decomposition` (Lens 5):**
   Reports `total_score_brier_or_crps` ($\mathrm{BS}$), `miscalibration_REL_MCB` ($\mathrm{REL}\_{\mathrm{CORP}}$), `resolution_RES_DSC` ($\mathrm{RES}\_{\mathrm{CORP}}$), and `base_uncertainty_UNC0` ($\mathrm{UNC}\_0$).
   - **Audit Check:** Verifies the exact PAV identity closure:
     $$\bigl|\mathrm{BS} - (\mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0)\bigr| \le 10^{-4}$$
6. **`findings` (`registered`, `used`, `unused`):**
   - **Audit Check:** Enforces $\text{registered} = \text{used} + \text{unused}$ with $\text{registered} \ge 1$ and $\text{used} \ge 1$.

---

### 8.5 Section 5: `decision_rule_and_scope`

The final section governs Bridge $L\_2 \to L\_3$ and Invariant $\Phi\_4$:

1. **Cost-Loss Parameters and Threshold Verification:**
   Declares `event_probability_p` ($p$), `base_rate_floor_epsilon` ($\varepsilon\_{\text{floor}}$), `protection_cost_C` ($C > 0$), `event_loss_L` ($L > 0$), `mitigation_efficacy_eta` ($\eta \in (0, 1]$), `critical_threshold_p_star` ($p^{\ast}$), and `declared_act` (`"protect"` or `"do_not_protect"`).
   - **Audit Check:** Recomputes $\frac{C}{\eta L}$ and verifies:
     $$\left|p^{\ast} - \frac{C}{\eta L}\right| \le 10^{-4}$$
     and verifies that `declared_act` is `"protect"` **if and only if** $p > p^{\ast}$ (and `"do_not_protect"` if and only if $p \le p^{\ast}$)!
2. **`scope_statement`, `who_pays_for_error`, and `exit_clause`:**
   Enforces explicit operational boundaries, Chouldechova error-bearer attribution, and a numeric circuit-breaker exit rule.

---

## Step 3 — Auditable Laboratory in Rust and Exercises

In `crates/stochology-core/src/card.rs`, `EpistemologicalCard::from_yaml_str` parses the YAML card using a zero-dependency parser written in pure Rust, and `EpistemologicalCard::audit()` runs all cross-field algebraic checks. You can see all three canonical cards (`card_01`, `card_02`, `card_03`) audited against their real CSV datasets by running:

~~~bash
cargo run --release --bin labs -- --siga
cargo run --release --bin gates
~~~

### Exercises

1. **(Calculation — Auditing a Card by Hand)** Suppose an analyst submits a YAML card with `protection_cost_C: 15000.0`, `event_loss_L: 250000.0`, `mitigation_efficacy_eta: 0.75`, `critical_threshold_p_star: 0.08000`, `event_probability_p: 0.06500`, and `declared_act: "protect"`.
   - *(a)* Verify whether `critical_threshold_p_star` matches $\frac{C}{\eta L}$.
   - *(b)* Will `EpistemologicalCard::audit()` approve or reject `declared_act: "protect"`? Why?
2. **(Calculation — Auditing CORP-Murphy Closure)** A card declares `total_score_brier_or_crps: 0.03210`, `miscalibration_REL_MCB: 0.00578`, `resolution_RES_DSC: 0.06183`, and `base_uncertainty_UNC0: 0.08815` (the exact numbers from `card_01_atlantic_storms_evacuation.yaml`). Check whether $\mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$ equals $\mathrm{BS}$.
3. **(Concept — Adversarial Cross-Authorship)** Why does `field_authorship` require `null_model`, `decision_rule`, and `adversarial_reviewer` to be distinct agents? What cognitive and institutional bias on Layer $L\_3$ does this rule prevent?

### Chapter Summary

- The **Epistemological Card (`EpistemologicalCard`)** is the universal unit of communication in the Stochology Protocol (`contract_version: "1.0"`).
- Its five sections (`decision_maker_question`, `epistemological_header`, `uncertainty_budget_and_authorship`, `data_and_audit_contract`, `decision_rule_and_scope`) enforce Prohibitions 1–4 and Invariants $\Phi\_1$–$\Phi\_5$.
- `EpistemologicalCard::audit()` verifies internal algebraic closure across Jensen's Gap, Moulton-Campbell $n\_{\mathrm{eff}}(p)$, variance-budget unity, CORP-Murphy PAV decomposition, findings conservation, SHA-256 integrity, and the cost-loss decision rule $p \gtrless p^{\ast} = \frac{C}{\eta L}$.
