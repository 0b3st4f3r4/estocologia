# Stochology — Auditable Epistemological Machine (Protocol v1.0)

Pure-Rust implementation (**zero external dependencies**) of the **Stochology Protocol: Enriched Methodological Architecture for Judgment and Decision-Making under Uncertainty (`contract_version: "1.0"`)**.

The repository unifies within a single verifiable apparatus:
1. **Structural Diagnosis of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$):** Meng's Identity ($\rho_{R,Y}$), Manski's bounds under MNAR missingness, Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, $\mathrm{CRPS}$ evaluation, and **Performative Prediction** ($\theta_{\mathrm{SL}}, \theta_{\mathrm{PS}}, \theta_{\mathrm{PO}}$ of Stackelberg and contraction radius $\frac{\varepsilon \beta}{\gamma} < 1$).
2. **Uncertainty Typing and Severe Uncertainty:** **Kennedy-O'Hagan** Gaussian Process Structural Discrepancy ($\delta(x) \sim \mathcal{GP}(0, k(x, x'))$) and **Info-Gap Decision Theory** ($\hat{\alpha}, \hat{\beta}$, *zero-ing*) coupled with **Sniedovich's Global Maximin Safeguard** (Wald).
3. **The Five Transversal Lenses:**
   - **Information:** Data Processing Inequality (DPI), SDPI hierarchy ($\eta_{\chi^2} = \rho_m^2 \le \eta_{\mathrm{KL}} \le \eta_{\mathrm{TV}}$), and Fano's error floor.
   - **Time:** Itô variance drain, asymmetric optimal Kelly fraction ($f^* = \frac{p}{a} - \frac{1-p}{b}$), absorbing ruin states, and mixing time under Log-Sobolev Inequality.
   - **Scale and Tail:** Hill estimator ($\hat{\alpha}_{k,n}$) and moment existence regimes.
   - **Structure and Invariance:** Pearl's Causal Ladder, Invariant Causal Prediction (ICP), NILE score, 2nd-order worst-case loss, and hidden confounding threshold $\Lambda^*$.
   - **Decision and Calibration:** Cost-loss rule $p^* = \frac{C}{\eta L}$, $\mathrm{EVPI}$, quantile score, and isotonic **CORP-Murphy** decomposition via the *Pool-Adjacent-Violators* (PAV) algorithm.
4. **Structured Communication Contract (`contract_version: "1.0"`):** The 4 Prohibitions, the 5 Boundary Invariants ($\Phi_1$–$\Phi_5$), **Moulton-Campbell** unequal-cluster correction ($\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$), quantile exceedance design effect ($\rho_I(p)$ and $n_{\mathrm{eff}}(p)$), and **Chouldechova's** algebraic impossibility audit.
5. **4-Filter Eliminatory Funnel and 8-Step SIGA Roadmap:** Executed end-to-end over the Epistemological Cards in [`cards/`](cards/) with sequential evidence processes (*e-processes*) and executive memoranda in integer natural frequencies.
6. **Bitemporal Governance, Single-Opening Vault, and 4 Regimes of Drift:** Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$), canary column, SHA-256 Temporal Vault, Murphy-CORP circuit breaker, *Covariate Drift* adaptation (*ExTRA-WCP-T* + Lei & Candès bound), and *Label Shift* adaptation via the 4 canonical estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**).
7. **Transversal Real-Data Synthesis (`labs --real-data`, Lab 7):** End-to-end empirical audit across all **13 real datasets (`data/*.csv`)** — including the out-of-time (`nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`) and out-of-domain (`houston_flights_2011.csv`) flight reserves — and the **3 canonical Epistemological Cards (`cards/*.yaml`)**.

---

## Repository Structure

```text
.
├── AGENTS.md                                      # Binding governance and verification contract (v1.0)
├── docs/
│   └── PROTOCOL.md                                # Full Protocol v1.0 specification and Rust mapping
├── cards/                                         # Canonical Epistemological Cards (contract_version: "1.0")
│   ├── card_01_sepsis_triage.yaml
│   ├── card_02_bank_queue_staffing.yaml
│   └── card_03_performative_credit_scoring.yaml
├── data/                                          # Real empirical datasets (CSV)
├── crates/
│   ├── stochology-core/                           # Layers, Meng, Manski, Jensen, Performativity, GP, Info-Gap, Phi1-Phi5, and YAML Card
│   ├── stochology-lenses/                         # The 5 Lenses: Information, Time, Tail, Invariance, and Decision/CORP-PAV
│   ├── stochology-siga/                           # 4-Filter Eliminatory Funnel and 8-Step SIGA Roadmap
│   ├── stochology-governance/                     # Bitemporal Clock, Single-Opening Vault, and Drift (ExTRA-WCP-T, BBSE, MLLS, DFM, OTTER)
│   └── stochology-cli/                            # CLI binaries `labs` and `gates`
├── web/                                           # Interactive static site published at https://obestafera.com (GitHub Pages CI)
└── scripts/
    └── verify.sh                                  # Complete positive and negative verification suite
```

---

## Quick Start

### 1. Full Verification (Formatting, Clippy, Tests, Laboratories, and Gates)

```bash
scripts/verify.sh
```

### 2. Methodological and Empirical Laboratories (`labs`)

```bash
cargo run --release --bin labs -- --all
cargo run --release --bin labs -- --layers        # Lab 1: Meng, Manski, Jensen, and Performativity
cargo run --release --bin labs -- --uncertainty   # Lab 2: Kennedy-O'Hagan GP and Info-Gap + Sniedovich
cargo run --release --bin labs -- --lenses        # Lab 3: The Five Transversal Lenses
cargo run --release --bin labs -- --invariants    # Lab 4: Phi1-Phi5, Moulton-Campbell, Exceedance, and Chouldechova
cargo run --release --bin labs -- --siga          # Lab 5: 4-Filter Funnel and SIGA Roadmap on the 3 YAML Cards
cargo run --release --bin labs -- --governance    # Lab 6: Bitemporal Clock, Vault, and 4 Regimes of Drift
cargo run --release --bin labs -- --real-data     # Lab 7: Transversal Synthesis across all 13 real CSVs & 3 YAML Cards
```

### 3. Epistemological Gates and Negative Proofs (`gates`)

```bash
# Positive audit of all 7 gates (exits with code 0)
cargo run --release --bin gates

# Negative blocking proofs (each command must block the pathology and exit with code 1)
cargo run --release --bin gates -- --defect cromwell
cargo run --release --bin gates -- --defect phi5
cargo run --release --bin gates -- --defect moulton
cargo run --release --bin gates -- --defect performativity
cargo run --release --bin gates -- --defect sniedovich
cargo run --release --bin gates -- --defect clock
cargo run --release --bin gates -- --defect vault
cargo run --release --bin gates -- --defect concept_drift
cargo run --release --bin gates -- --defect chouldechova
```
