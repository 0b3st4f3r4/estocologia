# 12. Architectural Mapping of the Pure-Rust Apparatus

> **Learning Objectives.** By the end of this chapter, you will understand why the Stochology Protocol is implemented in **pure Rust with zero external dependencies**, how the workspace is factored into five crates (`stochology-core`, `stochology-lenses`, `stochology-siga`, `stochology-governance`, and `stochology-cli`), and where every equation and audit in this textbook lives in the source tree.

---

## 12.1 Why Pure Rust with Zero External Dependencies?

Most scientific computing projects depend on hundreds of transitive third-party packages (`pip`, `npm`, or external crates). Over a five-year horizon, transitive dependencies suffer from **supply-chain drift**: an upstream linear-algebra routine changes its tie-breaking rule, a random-number library alters its normal-distribution sampler, or a YAML parser changes how it handles floating-point precision—silently breaking reproducibility.

To guarantee that `seed = 2026` produces the **exact same bit-for-bit mathematical audit today and twenty years from today**, the entire Stochology Protocol is implemented in **100% pure Rust (2021 edition) with zero external crate dependencies**:
- **Deterministic Pseudo-Randomness & Online Moments (`stochology_core::rng`):** Implements Sebastiano Vigna's 64-bit `SplitMix64` generator, exact Box-Muller normal, Marsaglia-Tsang (2000) Gamma, Beta, Student-$t$, Exponential, and Knuth/Atkinson Poisson samplers, Welford-Terriberry 4th-order streaming moments (`OnlineMoments` with skewness, excess kurtosis, and Jarque-Bera test), and Künsch (1989) moving-block bootstrap (`block_bootstrap_ci`) directly from IEEE-754 primitives.
- **Cryptographic Hashing & Keyed Authentication (`stochology_core::sha256`):** Implements the full NIST **FIPS 180-4 SHA-256** digest and **RFC 2104 `HMAC-SHA256`** authentication (`hmac_sha256_hex`) from scratch to seal pre-registrations (`preregistration_hash`), temporal vaults (`TemporalVault`), and empirical datasets (`data/SHA256SUMS`).
- **Structured Card Parser (`stochology_core::card`):** Parses and validates `cards/*.yaml` directly into strongly typed Rust structs (`EpistemologicalCard`) without external serialization macros.

---

## 12.2 The Five-Crate Workspace Architecture

The repository is organized as a layered Cargo workspace where dependency arrows flow strictly downward from the CLI to the foundational core:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1060" height="420" viewBox="0 0 1060 420" fill="none" role="img" aria-labelledby="title-fig-12-crates">
  <title id="title-fig-12-crates">Five-Crate Rust Workspace Dependency Architecture</title>
  <defs>
  <marker id="cr-sky" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="cr-violet" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--violet)"/>
  </marker>
  <marker id="cr-emerald" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--emerald)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1058" height="418" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="270" y="16" width="520" height="86" rx="12" fill="var(--elevated)" stroke="var(--amber)" stroke-width="2.4"/>
  <text x="530" y="45" text-anchor="middle" fill="var(--ink)" class="mono" font-size="16" font-weight="800">stochology-cli</text>
  <text x="530" y="68" text-anchor="middle" fill="var(--amber)" class="mono" font-size="11.5" font-weight="700">Binaries: labs (7 Labs) &amp; gates (8 Gates G1–G8)</text>
  <text x="530" y="88" text-anchor="middle" fill="var(--muted)" font-size="11.5">Empirical CSV loaders &amp; SHA-256 verification</text>
  <line x1="380" y1="102" x2="188" y2="134" stroke="var(--sky)" stroke-width="2.2" marker-end="url(#cr-sky)"/>
  <line x1="530" y1="102" x2="530" y2="134" stroke="var(--violet)" stroke-width="2.2" marker-end="url(#cr-violet)"/>
  <line x1="680" y1="102" x2="872" y2="134" stroke="var(--emerald)" stroke-width="2.2" marker-end="url(#cr-emerald)"/>
  <rect x="24" y="140" width="320" height="128" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.2"/>
  <text x="184" y="168" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">stochology-lenses</text>
  <text x="184" y="191" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11" font-weight="700">5 Transversal Lenses + Machine</text>
  <text x="184" y="213" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">information · time · tail</text>
  <text x="184" y="233" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">invariance · decision (CORP)</text>
  <text x="184" y="253" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">machine (P1–P5) · relational</text>
  <rect x="370" y="140" width="320" height="128" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2.2"/>
  <text x="530" y="168" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">stochology-siga</text>
  <text x="530" y="191" text-anchor="middle" fill="var(--violet)" class="mono" font-size="11" font-weight="700">Funnel &amp; Operational Roadmap</text>
  <text x="530" y="213" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">funnel (4 Eliminatory Filters)</text>
  <text x="530" y="233" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">roadmap (8-Step SIGA, SHELF,</text>
  <text x="530" y="253" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">SBC, e-process, Gigerenzer)</text>
  <rect x="716" y="140" width="320" height="128" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2.2"/>
  <text x="876" y="168" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">stochology-governance</text>
  <text x="876" y="191" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11" font-weight="700">Production Audit &amp; Drift</text>
  <text x="876" y="213" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">clock (Bitemporal + Canary)</text>
  <text x="876" y="233" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">vault (Single-Opening SHA-256)</text>
  <text x="876" y="253" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">drift (CORP, ExTRA, 4 Shift)</text>
  <line x1="188" y1="268" x2="365" y2="300" stroke="var(--sky)" stroke-width="2.2" marker-end="url(#cr-sky)"/>
  <line x1="530" y1="268" x2="530" y2="300" stroke="var(--violet)" stroke-width="2.2" marker-end="url(#cr-violet)"/>
  <line x1="872" y1="268" x2="695" y2="300" stroke="var(--emerald)" stroke-width="2.2" marker-end="url(#cr-emerald)"/>
  <rect x="40" y="306" width="980" height="94" rx="12" fill="var(--surface)" stroke="var(--sky)" stroke-width="2.5"/>
  <text x="530" y="336" text-anchor="middle" fill="var(--ink)" class="mono" font-size="16" font-weight="800">stochology-core (Zero External Dependencies)</text>
  <text x="530" y="360" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">layers (L₁–L₃, Meng, Manski, Δ_J, CRPS, RRM) · uncertainty (GP, Info-Gap + Wald)</text>
  <text x="530" y="383" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">invariants (Φ₁–Φ₅, Moulton, Chouldechova) · card (YAML &amp; audit) · rng (SplitMix64) · sha256</text>
</svg>
  <figcaption><strong>Figure 12.1 — Five-Crate Rust Workspace Dependency Architecture</strong></figcaption>
</figure>

---

## 12.3 Complete Mapping from Textbook Chapters to Rust Modules

| Textbook Chapter & Protocol Section | Mathematical Concept / Governing Equation | Rust Module & Primary Types | Verification Command |
| --- | --- | --- | --- |
| **Ch. 1 (§1.1)** — Bridge $L\_1 \to L\_2$ | Meng's Identity ($\rho\_{R,Y}$), Rubin (`MCAR`/`MAR`/`MNAR`), Stratified Manski Bounds, Lee (2009) Trimming Bounds, and $W\_1$ | `stochology_core::layers::{MengAudit, ManskiBounds, MissingnessMechanism, empirical_wasserstein_1}` | `cargo run --release --bin labs -- --layers` |
| **Ch. 1 (§1.1)** — Bridge $L\_2 \to L\_3$ | Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, quantiles, and exact $O(n\log n)$ LOO $\mathrm{CRPS}$ | `stochology_core::layers::{NonlinearSummary, exact_loo_empirical_crps}` | `cargo run --release --bin labs -- --layers` |
| **Ch. 1 (§1.1)** — Bridge $L\_3 \to L\_1$ | Performative Prediction ($\theta\_{\mathrm{SL}}, \theta\_{\mathrm{PS}}, \theta\_{\mathrm{PO}}$), `PerfGD`, and RRM contraction $\frac{\varepsilon\beta}{\gamma} < 1$ | `stochology_core::layers::{PerformativeSystem, PerformativeDiagnostic}` | `labs --layers` & `gates -- --defect performativity` |
| **Ch. 2 (§1.2)** — Uncertainty Typing | Cholesky Empirical Bayes Kennedy-O'Hagan GP $\delta(x)$ and Info-Gap ($\hat{\alpha}, \hat{\beta}$) + Sniedovich/Wald Safeguard | `stochology_core::uncertainty::{KennedyOHaganDiscrepancy, cholesky_decompose, InfoGapOption}` | `labs --uncertainty` & `gates -- --defect sniedovich` |
| **Ch. 3 (§1.3)** — Lens 1 (Information) | 3-step DPI Markov chain, SDPI hierarchy ($\eta\_{\chi^2} \le \eta\_{\mathrm{KL}} \le \eta\_{\mathrm{TV}}$), Blahut-Arimoto $C(W)$, and Fano's floor | `stochology_lenses::information::{DpiMarkovChain, SdpiHierarchy, blahut_arimoto_capacity, fano_error_lower_bound}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 3 (§1.3)** — Lens 2 (Time) | Itô variance drain $g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2$, asymmetric & empirical Kelly $f^{\ast}$ vs. Merton, and LSI mixing | `stochology_lenses::time::{TimeKellyDiagnostic, empirical_kelly_optimal_fraction, MixingTimeAudit}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 3 (§1.3)** — Lens 3 (Scale & Tail) | Hill $\hat{\alpha}\_{k,n}$, Dekkers-Einmahl-de Haan $\hat{\xi}$, POT GPD, $\mathrm{ES}\_{0.99}$, and `tail_n_eff` | `stochology_lenses::tail::{HillTailDiagnostic, dedh_moment_tail_index, gpd_pot_fit, expected_shortfall_99}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 3 (§1.3)** — Lens 4 (Invariance) | Pearl's Ladder, linear ICP $\hat{S}^{\ast}$, Chow $F$-test, nonlinear Hermite NILE/IRM, Rosenbaum $\Lambda^{\ast}$ & Cinelli-Hazlett $RV\_q$ | `stochology_lenses::invariance::{run_icp, chow_invariance_f_stat, run_nonlinear_icp_nile, cinelli_hazlett_robustness_value}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 3 (§1.3)** — Lens 5 (Decision) | Cost-Loss $p^{\ast} = \frac{C}{\eta L}$, $\mathrm{EVPI}$, Richardson $V(p^{\ast})$, Log Score, isotonic PAV CORP-Murphy, and Fair-Coin Benchmark | `stochology_lenses::decision::{CostLossRule, CorpPavDecomposition, logarithmic_score, ExactCoinCalibrationBenchmark}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 4 (§1.3)** — Machine & Authentic Form | $(\varepsilon, \delta)$-Almost-Perfect Machine $M=(T,\Phi,\theta)$ (`P1`–`P5`) and `AuthenticModelForm` ($\delta(x)+\hat{S}^{\ast}$) | `stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm, IdentityClosureAudit}` | `gates` (`G8`) & `gates -- --defect machine` |
| **Ch. 5 (§1.3.7)** — Relational Ontology | *Śūnyatā*/RQM/OSR, Prigogine Brusselator RK4 + Schnakenberg $d\_i S$, Schelling tipping, *Homo clausus*, and *Daisyworld* RK4 | `stochology_lenses::relational::{RelationalComplexSystemsSynthesis, DissipativeAutopoieticAudit, DaisyworldSystem}` | `labs --lenses` & `gates` (`G8`) |
| **Ch. 6–8 (§2)** — Contract, $\Phi\_1$–$\Phi\_5$ & Card | Prohibitions 1–4, Moulton-Campbell $\tilde{m}$, exceedance profile $\rho\_I(p)$, Chouldechova + KMR, and YAML Card | `stochology_core::invariants::{ExceedanceDesignEffect, ChouldechovaAudit}` & `stochology_core::card::EpistemologicalCard` | `labs --invariants`, `gates`, `--defect cromwell/phi5/moulton/chouldechova` |
| **Ch. 9 (§3)** — Funnel & SIGA Roadmap | 4 Eliminatory Filters, Lentz Incomplete Beta & SHELF $\mathrm{Beta}(\hat{\alpha},\hat{\beta})$, 8-Step SIGA, SBC $\chi^2$, GRAPA *e-processes*, and Gigerenzer trees | `stochology_siga::funnel::{evaluate_four_filter_funnel, fit_shelf_beta_prior}` & `stochology_siga::roadmap::{EvidenceProcess, NaturalFrequencyMemo}` | `cargo run --release --bin labs -- --siga` |
| **Ch. 10 (§4)** — Governance, Vault & Drift | Bitemporal Clock ($t\_{\text{disp}}$), Canary, `HMAC-SHA256` Vault (`SealTampered`), CORP Breaker, `ExTRA-WCP-T`, Online `ACI`, and 4 Label Shift Estimators | `stochology_governance::{clock::*, vault::TemporalVault, drift::{AdaptiveConformalInference, *}}` | `labs --governance`, `gates -- --defect clock/vault/concept_drift` |
| **Ch. 11 (§5)** — Empirical Synthesis (Lab 7) | End-to-end audit across all 13 empirical CSV datasets and 3 canonical YAML cards | `stochology_cli` (`src/lib.rs` & `src/bin/labs.rs`) | `cargo run --release --bin labs -- --real-data` |
