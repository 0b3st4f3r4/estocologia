# AGENTS.md — Governance, Engineering, and Orchestration Contract (Stochology Protocol v1.0)

Binding contract for any agent or contributor operating in this repository. The system implemented here is an **auditable epistemological machine**: claim, generating process, executable null model, and severe test in the same place, with a fixed seed (`seed = 2026`), a structured Epistemological Card (`contract_version: "1.0"`), and algorithmic gates capable of rejecting a flawed claim. The final reader of the output is **human** — decision-maker, student, instructor, or auditor.

This repository implements the **Stochology Protocol v1.0** in **pure Rust (zero external dependencies)**:
- Full specification and architectural mapping: [`docs/PROTOCOL.md`](docs/PROTOCOL.md)

---

## I. Golden Rules — The Four Prohibitions

1. **Ban on unstructured free natural language between agents (`PROHIBITION_1`).** All communication between agents or modules transits exclusively through structured **Epistemological Cards** (`EpistemologicalCard`, in `crates/stochology-core/src/card.rs`, `contract_version: "1.0"`), serialized in YAML (`cards/*.yaml`) and validated by `EpistemologicalCard::audit()`.
2. **Ban on the lone mean (`PROHIBITION_2`).** Whenever the loss/payoff function is nonlinear, dynamics are multiplicative, or the tail is heavy, reporting only the mean is forbidden: one must report the evaluation at the mean $f(\mathbb{E}[X])$, the simulated expectation $\mathbb{E}[f(X)]$, Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, the median, tail quantiles ($q_{05}, q_{95}$), and the *Continuous Ranked Probability Score* ($\mathrm{CRPS}$, in `crates/stochology-core/src/layers.rs`).
3. **Ban on confusing learning with performativity (`PROHIBITION_3`).** Small predictive error does not prove that a model is true. On Bridge $L_3 \to L_1$, one must distinguish **Performative Stability** ($\theta_{\mathrm{PS}}$) from **Performative Optimality** ($\theta_{\mathrm{PO}}$, Stackelberg equilibrium), measure Wasserstein-1 sensitivity ($\varepsilon$), and audit the strict contraction condition of Repeated Risk Minimization ($\frac{\varepsilon \beta}{\gamma} < 1$, in `PerformativeSystem`).
4. **Ban on dogmatic extremes and omitted attempts (`PROHIBITION_4`).** No probability may be $0\%$ or $100\%$ (Cromwell's Rule: $p \in [\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$); every forecast is evaluated out-of-sample via the isotonic **CORP-Murphy** decomposition using *Pool-Adjacent-Violators* ($\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$, in `crates/stochology-lenses/src/decision.rs`); and the counter of attempted specifications $K \ge 1$ plus the evidence process (*e-process* $E_t$) and discarded findings must be declared and validated.

---

## II. Boundary Invariants ($\Phi_1$ to $\Phi_5$)

| Invariant | Formal Requirement | Code Location |
| --- | --- | --- |
| $\Phi_1$ — Simulability | Fixed seed (`seed = 2026`, `SplitMix64`) and executable null model before any fitting | `crates/stochology-core/src/rng.rs` & `card.rs` |
| $\Phi_2$ — Independent Unit | Margin over the true independent unit; Moulton-Campbell correction for unequal clusters $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$, threshold exceedance ICC $\rho_I(p)$, and $n_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho_I(p)}$ | `crates/stochology-core/src/invariants.rs` (`ExceedanceDesignEffect`, `OperationalWeightingGap`) |
| $\Phi_3$ — Severe Test & Missingness | Explicit Rubin missingness classification (`MCAR`, `MAR`, `MNAR`), Meng's Identity ($\rho_{R,Y}$), non-parametric Manski bounds, SHA-256 pre-registration, and Pearl's Ladder rung | `crates/stochology-core/src/layers.rs`, `sha256.rs` & `card.rs` |
| $\Phi_4$ — Scope, Fairness & Exit | Economic boundary $p^* = \frac{C}{\eta L}$, algebraic audit of Chouldechova's trilemma ($\mathrm{FPR}_A / \mathrm{FPR}_B$ under unequal base rates), and automatic numeric exit clause | `crates/stochology-core/src/invariants.rs` (`ChouldechovaAudit`) & `card.rs` |
| $\Phi_5$ — Question Resolution | Ban on answering a daily/shift operational decision with coarse annual/monthly aggregates ($R_0$) | `crates/stochology-core/src/invariants.rs` (`verify_resolution_phi5`) |

The `gates` binary enforces invariants $\Phi_1$–$\Phi_5$ and methodological locks across seven gates (`G1` to `G7`) and exits with code `0` (approved) or `1` (blocked).

---

## III. The Five Transversal Lenses and Severe Uncertainty

The five transversal lenses reside in `crates/stochology-lenses`:
1. **Information Lens (`information.rs`):** Data Processing Inequality (DPI), exact hierarchy of strong contraction constants ($\eta_{\chi^2}(W) = \rho_m^2 \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W)$), amplitude-constrained bound, and Fano's error floor.
2. **Time Lens (`time.rs`):** Itô variance drain ($g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$), optimal Kelly fraction under asymmetric returns ($f^* = \frac{p}{a} - \frac{1-p}{b}$), absorbing ruin barriers, and Langevin/MCMC mixing time under Log-Sobolev Inequality.
3. **Scale and Tail Lens (`tail.rs`):** Hill estimator ($\hat{\alpha}_{k,n}$), moment existence regime classification ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$), and quantiles with effective tail sample size.
4. **Structure and Invariance Lens (`invariance.rs`):** Invariant Causal Prediction (ICP by intersection across environments $e \in \mathcal{E}$), *Nonlinear Invariant Risk Minimization* (NILE) penalty, 2nd-order worst-case loss, and critical hidden confounding threshold $\Lambda^*$.
5. **Decision and Calibration Lens (`decision.rs`):** Cost-loss rule $p^* = \frac{C}{\eta L}$, Expected Value of Perfect Information ($\mathrm{EVPI}$), quantile score (pinball loss), and isotonic **CORP-Murphy** decomposition via the PAV algorithm.

Structural and severe uncertainty modeling resides in `crates/stochology-core/src/uncertainty.rs`:
- **Kennedy-O'Hagan Structural Discrepancy (`KennedyOHaganDiscrepancy`):** Gaussian Process $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$ with anchoring constraint to prevent parameter pseudo-convergence and extrapolation overconfidence.
- **Info-Gap Decision Theory with Sniedovich's Safeguard (`InfoGapOption`):** Robustness horizon $\hat{\alpha}(q, r_c)$, opportuneness horizon $\hat{\beta}(q, r_w)$, *zero-ing* property, and Wald's global Maximin safeguard against "No-Man's Land".

---

## IV. 4-Filter Funnel, 8-Step SIGA Roadmap, and Production Governance

- **Eliminatory Funnel and SIGA Roadmap (`crates/stochology-siga`):**
  - `funnel.rs`: Evaluates the **4 Eliminatory Filters** in strict sequence (Filter 1: $\mathrm{EVPI}$ and $\Phi_5$; Filter 2: Moulton-Campbell $n_{\mathrm{eff}}(p)$, Markov Blanket, and Berkson collider ban; Filter 3: Simulation-Based Calibration SBC $\chi^2$ and SHELF elicitation; Filter 4: SHA-256 pre-registration, independent cross-authorship, and exit clause).
  - `roadmap.rs`: Executes the **8-Step SIGA Roadmap** (**S**urvey the Layer & Bridge; **I**nterrogate the Decision-Maker; **G**enerate the Null & Noise Physics; **A**ct with Threshold, Memory & Exit), cumulative evidence processes (*e-processes* $E_t = \prod L_i$), and executive memoranda in integer natural frequencies.
- **Bitemporal Governance, Vault, and Drift (`crates/stochology-governance`):**
  - `clock.rs`: Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$) and synthetic canary column ($I(\text{Canary}; Y) = 0$).
  - `vault.rs`: Single-opening Temporal Vault (`TemporalVault`) with SHA-256 seal and automatic downgrade to exploratory status on failure.
  - `drift.rs`: Murphy-CORP Circuit Breaker (`MurphyCorpCircuitBreaker`), adaptation to *Covariate Drift* via exponential tilt and *ExTRA-WCP-T* with Lei & Candès bound ($1 - \alpha - d_{\mathrm{TV}}$), and adaptation to *Label Shift* via the 4 canonical estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**).

---

## V. Mandatory Verification Before Consolidation

A single command executes the entire mandatory verification suite of the repository:

~~~bash
scripts/verify.sh
~~~

The script `scripts/verify.sh` enforces the following list without exception:

~~~bash
cargo fmt --all --check                                         # rustfmt check
cargo clippy --workspace --all-targets -- -D warnings           # zero warnings tolerated across all 5 crates
cargo test --workspace                                          # all unit and integration tests
cargo run --release --bin labs -- --all                         # all 7 methodological and empirical laboratories
cargo run --release --bin labs -- --layers                      # Lab 1: Meng, Manski, Jensen, and Performativity
cargo run --release --bin labs -- --uncertainty                 # Lab 2: Kennedy-O'Hagan GP and Info-Gap + Sniedovich
cargo run --release --bin labs -- --lenses                      # Lab 3: The Five Transversal Lenses on real data
cargo run --release --bin labs -- --invariants                  # Lab 4: Phi1-Phi5, Moulton-Campbell, Exceedance, and Chouldechova
cargo run --release --bin labs -- --siga                        # Lab 5: 4-Filter Funnel and SIGA Roadmap on the 3 YAML Cards
cargo run --release --bin labs -- --governance                  # Lab 6: Bitemporal Clock, Vault, and 4 Regimes of Drift
cargo run --release --bin labs -- --real-data                   # Lab 7: Transversal Synthesis across all 13 CSVs & 3 YAML Cards
cargo run --release --bin gates                                 # The 7 Epistemological Gates (G1-G7): exits with 0
cargo run --release --bin gates -- --defect cromwell            # Negative proof (Cromwell's Rule): exits with 1
cargo run --release --bin gates -- --defect phi5                # Negative proof (Phi5 Resolution): exits with 1
cargo run --release --bin gates -- --defect moulton             # Negative proof (Moulton Pseudoreplication): exits with 1
cargo run --release --bin gates -- --defect performativity      # Negative proof (RRM Divergence): exits with 1
cargo run --release --bin gates -- --defect sniedovich          # Negative proof (Info-Gap No-Man's Land): exits with 1
cargo run --release --bin gates -- --defect clock               # Negative proof (Canary Temporal Intrusion): exits with 1
cargo run --release --bin gates -- --defect vault               # Negative proof (Vault Reopening): exits with 1
cargo run --release --bin gates -- --defect concept_drift       # Negative proof (Concept Drift Circuit Breaker): exits with 1
cargo run --release --bin gates -- --defect chouldechova        # Negative proof (Chouldechova Impossibility): exits with 1
git tag --list                                                  # Must be empty
~~~

---

## VI. Project Status (Protocol v1.0)

| Layer | Location | Status |
| --- | --- | --- |
| Protocol Specification | `docs/PROTOCOL.md` | Consolidated v1.0 specification and code mapping |
| Rust Apparatus | `crates/stochology-core`, `crates/stochology-lenses`, `crates/stochology-siga`, `crates/stochology-governance`, `crates/stochology-cli` | 5 pure-Rust crates (zero external dependencies), 2021 edition |
| Epistemological Cards | `cards/card_01_sepsis_triage.yaml`, `cards/card_02_bank_queue_staffing.yaml`, `cards/card_03_performative_credit_scoring.yaml` | 3 canonical cards (`contract_version: "1.0"`) audited in `gates` and `labs --siga` |
| Empirical Datasets | `data/*.csv` (`danish_fire.csv`, `bank_calls.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `djia_1885_2023.csv`, `atlantic_storms.csv`, `toronto_arrests.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `vic_elec.csv`; plus sealed out-of-time/out-of-domain reserves `nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, `houston_flights_2011.csv`) | Preserved and exercised by the laboratories |
| Verification | `scripts/verify.sh` | Complete positive and negative test suite |
