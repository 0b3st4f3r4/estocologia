# 14. Verification and Algorithmic Gates (`G1`–`G8` and Negative Proofs)

> **Learning Objectives.** By the end of this chapter, you will understand how the Stochology Protocol enforces end-to-end verification via `scripts/verify.sh`, what each of the **Eight Epistemological Gates (`G1`–`G8`)** checks, and why **Ten Negative Defect Proofs (`--defect`)** are required to prove that the gates actually catch and block methodological errors.

---

## 14.1 The Philosophy of Positive and Negative Verification

In ordinary software engineering, a test suite checks that valid inputs produce the expected output (*positive testing*). In an **auditable epistemological machine**, positive testing alone is never enough.

Suppose a gate is supposed to reject dogmatic $0\%$ probabilities or Moulton-Campbell pseudoreplication, but a programmer accidentally wrote `if true { return Ok(()); }` inside the validator. Every positive test will pass with flying colors! The only way to prove that a gate has genuine refutation power (Deborah Mayo's Severity Principle, Chapter 7) is to **deliberately inject each forbidden methodological pathology into the system and verify that the binary halts and exits with non-zero status code `1`**.

A single command at the root of the repository runs the complete positive and negative verification suite:

~~~bash
scripts/verify.sh
~~~

---

## 14.2 The Six Verification Stages of `scripts/verify.sh`

1. **Formatting Check (`cargo fmt --all --check`):** Enforces canonical Rust formatting across all 5 crates.
2. **Zero-Warning Static Analysis (`cargo clippy --workspace --all-targets -- -D warnings`):** Rejects any compiler or Clippy lint warning anywhere in library, binary, or test targets.
3. **Unit and Integration Tests (`cargo test --workspace`):** Executes all unit tests across `stochology-core`, `stochology-lenses`, `stochology-siga`, `stochology-governance`, and the end-to-end integration suite in `crates/stochology-cli/tests/protocol_integration.rs`.
4. **The Seven Methodological and Empirical Laboratories (`labs`):**
   - `cargo run --release --bin labs -- --all`
   - `cargo run --release --bin labs -- --layers` (Lab 1: Meng, Manski, Jensen, Performativity)
   - `cargo run --release --bin labs -- --uncertainty` (Lab 2: Kennedy-O'Hagan GP, Info-Gap + Sniedovich)
   - `cargo run --release --bin labs -- --lenses` (Lab 3: The 5 Lenses, Machine $M=(T,\Phi,\theta)$, and 6-Axis Relational Ontology)
   - `cargo run --release --bin labs -- --invariants` (Lab 4: $\Phi\_1$–$\Phi\_5$, Moulton-Campbell, Exceedance, Chouldechova)
   - `cargo run --release --bin labs -- --siga` (Lab 5: 4-Filter Funnel and 8-Step SIGA on the 3 YAML Cards)
   - `cargo run --release --bin labs -- --governance` (Lab 6: Bitemporal Clock, Canary, Vault, and 4 Drift Regimes)
   - `cargo run --release --bin labs -- --real-data` (Lab 7: Transversal Synthesis across all 13 CSVs & 3 YAML Cards)
5. **The Eight Epistemological Gates (`gates`, exit code `0`) and Ten Negative Proofs (`--defect`, exit code `1`).**
6. **Repository Tag Hygiene (`git tag --list`):** Verifies that no git tags pollute the tree.

---

## 14.3 The Eight Epistemological Gates (`G1` to `G8`)

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1140" height="260" viewBox="0 0 1140 260" fill="none" role="img" aria-labelledby="title-fig-14-gates">
  <title id="title-fig-14-gates">The Eight Epistemological Gates (G1 to G8) and Ten Negative Defect Proofs</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1138" height="258" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="18" y="20" width="262" height="100" rx="10" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="34" y="46" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">G1 · SIMULABILITY (Φ₁)</text>
  <text x="34" y="70" fill="var(--ink)" font-size="13.5" font-weight="750">seed=2026 &amp; 13 CSV SHA-256</text>
  <text x="34" y="94" fill="var(--muted)" class="mono" font-size="11">SplitMix64 + Null Model H₀</text>
  <rect x="298" y="20" width="262" height="100" rx="10" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="314" y="46" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">G2 · INDEPENDENT UNIT (Φ₂)</text>
  <text x="314" y="70" fill="var(--ink)" font-size="13.5" font-weight="750">Moulton m̃ &amp; Exceedance n_eff</text>
  <text x="314" y="94" fill="var(--rose)" class="mono" font-size="11">--defect moulton → exit 1</text>
  <rect x="578" y="20" width="262" height="100" rx="10" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="594" y="46" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">G3 · SEVERE TEST &amp; SEAL (Φ₃)</text>
  <text x="594" y="70" fill="var(--ink)" font-size="13.5" font-weight="750">Rubin · Meng · Manski · Lee</text>
  <text x="594" y="94" fill="var(--muted)" class="mono" font-size="11">64-Hex SHA-256 Pre-Reg Seal</text>
  <rect x="858" y="20" width="264" height="100" rx="10" fill="var(--elevated)" stroke="var(--amber)" stroke-width="2"/>
  <text x="874" y="46" fill="var(--amber)" class="mono" font-size="11.5" font-weight="800">G4 · FAIRNESS &amp; EXIT (Φ₄)</text>
  <text x="874" y="70" fill="var(--ink)" font-size="13.5" font-weight="750">p* = C/(ηL) · Chouldechova</text>
  <text x="874" y="94" fill="var(--rose)" class="mono" font-size="11">--defect chouldechova → exit 1</text>
  <rect x="18" y="138" width="262" height="100" rx="10" fill="var(--elevated)" stroke="var(--rose)" stroke-width="2"/>
  <text x="34" y="164" fill="var(--rose)" class="mono" font-size="11.5" font-weight="800">G5 · RESOLUTION (Φ₅)</text>
  <text x="34" y="188" fill="var(--ink)" font-size="13.5" font-weight="750">R_data ≥ R_decision Match</text>
  <text x="34" y="212" fill="var(--rose)" class="mono" font-size="11">--defect phi5 → exit 1</text>
  <rect x="298" y="138" width="262" height="100" rx="10" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="314" y="164" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">G6 · PROHIBITIONS 1–4 &amp; WALD</text>
  <text x="314" y="188" fill="var(--ink)" font-size="13.5" font-weight="750">Cromwell · RRM · Wald · CORP</text>
  <text x="314" y="212" fill="var(--rose)" class="mono" font-size="10.5">cromwell/performativity/sniedovich</text>
  <rect x="578" y="138" width="262" height="100" rx="10" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="594" y="164" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">G7 · CLOCK, VAULT &amp; DRIFT</text>
  <text x="594" y="188" fill="var(--ink)" font-size="13.5" font-weight="750">Canary · HMAC Vault · Breaker</text>
  <text x="594" y="212" fill="var(--rose)" class="mono" font-size="10.5">clock / vault / concept_drift</text>
  <rect x="858" y="138" width="264" height="100" rx="10" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="874" y="164" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">G8 · MACHINE &amp; AUTHENTIC FORM</text>
  <text x="874" y="188" fill="var(--ink)" font-size="13.5" font-weight="750">P1–P5 · δ(x)+S* · 6 IDs · 6 Axes</text>
  <text x="874" y="212" fill="var(--rose)" class="mono" font-size="11">--defect machine → exit 1</text>
  </svg>
  <figcaption><strong>Figure 14.1 — The Eight Epistemological Gates (G1–G8) and Ten Negative Defect Proofs</strong></figcaption>
</figure>

When you run `cargo run --release --bin gates`, the binary executes eight sequential gates:

| Gate | Name | What It Audits at Runtime |
| --- | --- | --- |
| **`G1`** | **Simulability & Cryptographic Integrity ($\Phi\_1$)** | Verifies `seed = 2026` (`SplitMix64`), executable null models, and FIPS 180-4 SHA-256 digests of all 13 CSV datasets against `data/SHA256SUMS` and `cards/*.yaml`. |
| **`G2`** | **Independent Unit & Exceedance Design Effect ($\Phi\_2$)** | Verifies Moulton-Campbell unequal-cluster $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$, tail-exceedance ICC $\rho\_I(p)$, and $n\_{\mathrm{eff}}(p)$ across all 3 canonical cards and empirical benchmarks. |
| **`G3`** | **Severe Testing, Missingness & Pre-Registration ($\Phi\_3$)** | Verifies Rubin missingness (`MCAR`, `MAR`, `MNAR`), Meng's Identity, Manski bounds, Pearl's Ladder rung, and 64-hex SHA-256 pre-registration seals. |
| **`G4`** | **Scope, Fairness & Exit Clause ($\Phi\_4$)** | Verifies cost-loss threshold $p^{\ast} = \frac{C}{\eta L}$, action consistency (`protect` iff $p > p^{\ast}$), Chouldechova's algebraic fairness identity, and numeric exit clauses. |
| **`G5`** | **Question Resolution Alignment ($\Phi\_5$)** | Verifies $R\_{\text{data}} \ge R\_{\text{decision}}$ so coarse annual/monthly aggregates never answer daily, shift, or 6-hour operational questions. |
| **`G6`** | **Prohibitions 1–4, Performativity, Info-Gap & Lenses** | Verifies Cromwell's Rule ($p \in [\varepsilon\_{\text{floor}}, 1-\varepsilon\_{\text{floor}}]$), 7-number `NonlinearSummary`, Stackelberg $\theta\_{\mathrm{PO}}$ vs. $\theta\_{\mathrm{PS}}$ ($r\_{\mathrm{RRM}} < 1$), Info-Gap + Sniedovich Wald floor, SDPI/DPI hierarchy, and exact PAV CORP-Murphy closure. |
| **`G7`** | **Bitemporal Governance, Vault & Drift Breaker** | Verifies Bitemporal Clock ($t\_{\text{disp}} \le \tau$), Synthetic Canary ($I(\text{Canary}; Y) = 0$), Single-Opening Temporal Vault (`0 -> 1`), `ExTRA-WCP-T` Lei & Candès bounds, 4 Label Shift estimators, and the Murphy-CORP Circuit Breaker. |
| **`G8`** | **Almost-Perfect Machine, Authentic Form & Relational Synthesis** | Verifies $(\varepsilon, \delta)$-Almost-Perfect Machine $M=(T,\Phi,\theta)$ (`P1`–`P5`), First-Class Authentic Model Form ($\delta(x)+\hat{S}^{\ast}$, named discarded atoms, 6 closed identities, $A(\text{model}) = \text{true}$), Fair-Coin Benchmark, and the 6-Axis Relational & Complex Systems Synthesis (Schelling & *Daisyworld*). |

---

## 14.4 The Ten Negative Defect Proofs (`--defect`)

Each command below injects a targeted methodological defect and **must exit with code `1`**:

| Command (`cargo run --release --bin gates -- ...`) | Injected Pathology | Blocking Gate & Mechanism |
| --- | --- | --- |
| `--defect cromwell` | Sets a forecast probability to dogmatic $0\%$ or $100\%$ outside $[\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$. | **Gate `G6` (Prohibition 4):** Rejects violation of Cromwell's Rule. |
| `--defect phi5` | Attempts to answer a 5-minute shift-staffing decision using coarse monthly aggregate data ($R\_{\text{data}} < R\_{\text{decision}}$). | **Gate `G5` (Invariant $\Phi\_5$):** Rejects temporal resolution mismatch. |
| `--defect moulton` | Claims nominal sample size $n$ as effective sample size $n\_{\mathrm{eff}}(p)$, ignoring Moulton-Campbell cluster inflation ($\mathrm{DEFF} > 1$). | **Gate `G2` (Invariant $\Phi\_2$):** Rejects pseudoreplicated margin of error. |
| `--defect performativity` | Deploys a performative retraining loop with contraction ratio $r\_{\mathrm{RRM}} = \frac{\varepsilon\beta}{\gamma} \ge 1$ or conflates $\theta\_{\mathrm{PS}}$ with $\theta\_{\mathrm{PO}}$. | **Gate `G6` (Prohibition 3):** Rejects divergent performative feedback. |
| `--defect sniedovich` | Selects a fragile "Christmas Turkey" option with positive local $\hat{\alpha}$ that collapses below the global Wald Maximin floor in "No-Man's Land." | **Gate `G6` (Chapter 2):** Rejects Info-Gap No-Man's Land collapse. |
| `--defect clock` | Disables the bitemporal filter ($t\_{\text{disp}} \le \tau$), allowing future records to leak into features and spiking canary mutual information $I(\text{Canary}; Y) > 0$. | **Gate `G7` (Chapter 10):** Trips the Bitemporal Canary Intrusion Alarm. |
| `--defect vault` | Attempts to call `open_once()` a second time (`opened_count: 1 -> 2`) on an already-opened `TemporalVault`. | **Gate `G7` (Chapter 10):** Blocks vault reopening (`VaultReopenError`). |
| `--defect concept_drift` | Injects a structural breakdown where resolution collapses ($\Delta\mathrm{RES}\_{\mathrm{CORP}} < -\tau\_{\mathrm{RES}}$) while attempting to continue automated predictions. | **Gate `G7` (Chapter 10):** Trips the Murphy-CORP Concept Drift Circuit Breaker. |
| `--defect chouldechova` | Claims simultaneous equality of $\mathrm{FPR}, \mathrm{PPV},$ and $\mathrm{FNR}$ across groups with unequal base rates ($p\_A \neq p\_B$). | **Gate `G4` (Invariant $\Phi\_4$):** Rejects algebraic violation of Chouldechova's identity. |
| `--defect machine` | Injects a non-invariant spurious proxy into $\hat{S}^{\ast}$, hides a discarded atom, or breaks Gram effective dimension $d\_{\mathrm{eff}} < 0.80 W$. | **Gate `G8` (Chapter 4):** Rejects inauthentic model form ($A(\text{model}) = \text{false}$). |
