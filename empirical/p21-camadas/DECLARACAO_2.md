# Declaration 2 of point 21: source list, drawing rule, and seed for the 60 documented errors

**Written on 2026-10-20, before any human coder is recruited or scored.** Its SHA-256 enters `DECLARACAO.sha256` alongside the sealed 60-error catalogue (`erros.tsv`), the governance registry (`registro.tsv`), and the synthetic model-coder rehearsal matrix (`codificacoes_ensaio.tsv`). Because `DECLARACAO.md` §2 explicitly required this second declaration to freeze the source list, selection rule, and seed before any draw, **K remains 1**.

## 1. External source list

All 60 errors in `erros.tsv` come from documented cases outside the Stochology programme and outside the datasets analysed in the textbook (`bank_calls.csv`, `danish_fire.csv`, `atlantic_storms.csv`, `nyc_flights_*`, `toronto_arrests.csv`, `nsw_*`, STAR, RAND, Kemmeren, FDA trial, Meyer/Viscusi, Preston). They are drawn from four public external corpora:

1. **Published replications, retractions, and methodological error catalogues (15 cases, `E-01` to `E-15`)**: documented statistical, coding, and design failures in economics, biomedicine, genetics, neuroscience, and psychology (including spreadsheet range omission, microarray row-offset shifts, dead-salmon multiple-testing without correction, immortal-time bias, collider stratification in hospitalised cohorts, and p-value difference-of-significance errors).
2. **Engineering, safety-critical software, and metrological post-mortems (15 cases, `E-16` to `E-30`)**: public accident and incident investigation reports (unit-conversion mismatch, 64-bit to 16-bit integer overflow, 24-bit fixed-point clock drift, race conditions in radiation therapy, Pitot tube icing sensor failure, wind-oscillation resonance, and floating-point division lookup defects).
3. **Clinical, epidemiological, and algorithmic deployment post-mortems (15 cases, `E-31` to `E-45`)**: documented failures in clinical prediction rules, triage scores, pulse oximetry calibration across skin pigmentation, healthcare cost-as-need proxies, sepsis alert drift, and hospital readmission penalties.
4. **Financial, operational, and public-policy decision post-mortems (15 cases, `E-46` to `E-60`)**: public post-mortems of risk management, market microstructure, inventory/supply-chain forecasting, and automated pricing/dispatch systems (VaR sign/denominator formula errors, deployment flag misrouting, correlation breakdown under liquidity feedback, census sampling undercount, and automated tax/benefit adjudication errors).

## 2. Selection and neutral-description rule

1. **Quota and seed**: 15 documented cases per external corpus ($4 \times 15 = 60$ errors, `E-01` to `E-60`), ordered and resampled in the bootstrap evaluator with the fixed seed `20261021` (`stochology_core::Rng`, 1,000 bootstrap replicas over the 60 errors).
2. **Neutrality constraint (enforced by lexical gate in `labs --layers`)**: each row of `erros.tsv` states only the system, what entered, what came out, and what went wrong according to the external source. The instrument scans every row (`fonte`, `dominio`, `descricao`) and rejects the catalogue if any word from the layer vocabulary (`camada`, `camadas`, `layer`, `layers`, `L1`, `L2`, `L3`, `costura`, `costuras`, `seam`, `seams`, `WM`, `MA`, `WA`, `WMA`, `ARIT`, `FORA`, `NC`, `estocologia`, `stochology`, `desalinhamento`, `desalinhamentos`, `misalignment`, `misalignments`, `d(W`, `d(M`, `d(A` including spaced variants) appears in `erros.tsv`.
3. **Presence of the falsifiers (`ARIT` and `FORA`)**: the 60 cases include documented errors of pure calculation, unit conversion, spreadsheet indexing, and hardware/software procedure (`ARIT`), as well as physical/hardware/organisational failures outside the representational frame (`FORA`), so that the falsifiers of `DECLARACAO.md` §3 and §6 are genuinely reachable.

## 3. Rehearsal and gate

- `labs --layers` reads `empirical/p21-camadas/registro.tsv` and `empirical/p21-camadas/erros.tsv`, verifies the 60 descriptions against the lexical leak filter, locks the verdict (`BOX CLOSED`) while fewer than 3 human coders (at least 2 external) have completed coding, and tests the decision rule of `DECLARACAO.md` §6 through negative and positive proofs.
- `labs --rehearsal-layers` runs the §7 synthetic rehearsal over the 60 errors using `codificacoes_ensaio.tsv` (three model-family coders, labelled `SYNTHETIC`), reporting the floor of Fleiss' $\kappa$, per-code $\kappa$, 90% bootstrap intervals, and the falsifier checks without counting any human coder coverage.

