# AGENTS.md — Governance, Engineering, and Orchestration Contract (Stochology, in Rust)

Binding contract for any agent or contributor operating in this repository. The machine described here is
an **auditable epistemological machine**: claim, generative process, null model, and severe test in the same place, with
a fixed seed and a gate capable of failing a claim. The reader of the output is **human** — student, instructor, or decision-maker.

This file replaces the Python version of the contract, integrated on 2026-10-16. What changed: the apparatus is
**Rust**; the Python simulation code has been retired; the course that provided the internal numbering has been obsoleted; and
everything that remains declared and not yet enforced lives in `docs/DECLARED_PROPOSALS.md`.

---

## I. Golden Rules — The Four Prohibitions

1. **No free-form natural language between agents.** Communication passes through structured specification sheets
   (`Ficha`, in `crates/stochology-core/src/layer.rs`), carrying an identifier, layer, uncertainty type,
   one-line definition, simulable artifact, seed, independent unit, null model, scope, who pays
   for the error, and an exit clause.
2. **No solitary mean.** Wherever the function is nonlinear, the dynamics are multiplicative, or the tail is
   heavy, reporting only the mean is prohibited: report the evaluation at the mean $f(E[X])$, the simulated expectation $E[f(X)]$, the
   median, the quantiles, and the cumulative distribution curve. Chapter 5 of the textbook measures the Jensen toll; the instrument
   `labs --chapter 5` reproduces it.
3. **No confusing learning with performativity.** Small prediction error does not prove that a model is
   true. Run with and without the intervention (`do(a)` versus passive observation) and declare the feedback loop: the measured Goodhart effect
   is in Chapter 20 and equals 0.145, four times smaller than the declared 0.6.
4. **No dogmatic extremes and no omission of trials.** No probability may be 0% or 100%;
   every forecast is scored by a strictly proper scoring rule (Brier or log) out of sample; and the number of specifications
   tried must be declared — formerly the counter K, now recorded in the registry (`K = 38` at the H2 gate).

## II. Boundary Invariants

| Invariant | Requirement | Where it lives in code |
| --- | --- | --- |
| Phi1 — simulability | fixed seed and executable artifact | `crates/stochology-core/src/invariants.rs` and `rng.rs` |
| Phi2 — independent unit | margin on the true independent unit, never pseudoreplication | validated in the sheet; measured in `labs --chapter 16` and `labs --bank` |
| Phi3 — severe test | null model simulated before reading any pattern | validated in the sheet; `labs --chapter 7` and `labs --chapter 14` |
| Phi4 — scope and exit | who pays for the error and an objective exit clause | validated in the sheet; review board agent A8 |
| Phi5 — question resolution | enforced when the unit is a day or coarser: aggregate resolution (R0) fails; finer units remain exempt, as declared | `crates/stochology-core/src/invariants.rs` and `src/resolution.rs` |

The `gates` binary enforces the invariants and the review board across seven gates (G1 to G7, distinct from the six gates G1 to G6 of the wave engine `wave`) and exits with code 0 or 1.

## III. Ontology and the Five Lenses

Knowledge is represented as a typed hypergraph G = (V, E_tree, E_graph, H, Phi). Each node has a layer (L1 World, L2 Model,
L3 Agent), an uncertainty type (aleatory, epistemic, Knightian), a definition, and a pointer to its artifact.
Pathology edges mark misalignments across layers. The five lenses — **Fluxonomy, Chrononomy,
Scalonomy, Morphonomy, Crisonomy** — live in `crates/stochology-hypergraph/src/lenses.rs`, with tests.

## IV. Adversarial Review Board, Orchestrator, and Cycle

The eight agents of the adversarial review board are A1 to A8 (code, data, lifecycle, inference, governance, fairness, domain,
decision), in `crates/stochology-core/src/agents.rs`, governed by the condition

    Approved(X) = AND_{i=1..8} ( A_i(X) >= Severity_min ), Severity_min = 6.

Legacy records use the labels MB1–MB8: it is the same list. The orchestrator M
(`crates/stochology-core/src/orchestrator.rs`) drives the six-stage cycle: translation and scope, ingestion
and grain, simulation and null model, projection onto the five lenses, review board and gate, decision and closure. Of the two
modes, **System 1** lives in `crates/stochology-core/src/s1.rs`, with the filter funnel alongside it, and
**System 2** lives in `crates/stochology-s2`: LATS, belief-state engine, Metropolis inference, and
deliberative board stress-testing, with code synthesis (F1) declared and not implemented. See
`docs/DECLARED_PROPOSALS.md`.

## V. Mandatory Verification Before Consolidation

One command, and it is the list:

~~~bash
scripts/verify.sh            # all of §V: formatting, lints, tests, instruments, and verifiers
scripts/verify.sh --rapido   # the same without the heavy laboratories, for the editing loop
~~~

The list lives in `scripts/verify.sh` and CI invokes **this file**, not a copy. While it lived in two
places, the contract required 29 commands and CI ran 11 — the divergence that this paragraph closes. Below is the
reading of the list, with what each line enforces:

~~~bash
cargo fmt --all --check                             # formatting belongs to rustfmt, not personal taste
cargo clippy --workspace --all-targets -- -D warnings   # zero warnings tolerated across all nine crates
cargo test --workspace                              # all nine crates
cargo run --release --bin labs -- --all           # the 21 textbook laboratories
cargo run --release --bin labs -- --bank           # real H0, reading data/bank_calls.csv
cargo run --release --bin labs -- --resolution       # the R horizon
cargo run --release --bin labs -- --s1              # System 1 triage
cargo run --release --bin labs -- --s2              # System 2: hypergraph, belief, inference, and R-axis
cargo run --release --bin labs -- --queue            # queue with abandonment and the R0-to-R3 table
cargo run --release --bin gates -- --horizon all --reproduction   # the seven gates
cargo run --release --bin gates -- --horizon h3 --defect h3.voos23  # negative proof: exits with 1
cargo run --release --bin wave -- --check            # the six gates of the wave engine
cargo run --release --bin wave -- --defect mutation  # negative proof of the seal chain: exits with 1
cargo run --release --bin wave -- --defect inversion # negative proof of the causal arrow: exits with 1
cargo run --release --bin labs -- --rehearsal-shelf      # synthetic floor of the SHELF panel
cargo run --release --bin labs -- --rehearsal-listening     # coverage arithmetic and boundary of the listening gate
cargo run --release --bin labs -- --rehearsal-bridge      # roles, cases, and status in the bridge gate
python3 scripts/book/build.py --check               # the ramp AND the disk: the file must match the assembled book
python3 scripts/book/check_labs.py                  # every cited block checked against its instrument
python3 scripts/book/check_book.py                  # parts, chapters, fences, and ordering
python3 scripts/book/generate_results.py --check    # the docs/RUST_RESULTS.md registry
python3 scripts/check_seals.py                      # the 28 SHA-256 seals, paths relative to root
python3 scripts/style/check_pass.py                 # prose pass against HEAD:book/, file by file
git tag --list                                      # must be empty: history only, no versions (§VI)
~~~

The instruments that prove what the items in `docs/DECLARED_PROPOSALS.md` §2 closed also enter the
verification suite, because every row in that table cites one of them:

~~~bash
cargo run --release --bin labs -- --genetics    # fine resolution in genetics
cargo run --release --bin labs -- --h3          # the H3 chain
cargo run --release --bin labs -- --second-opinion     # second opinion on the registry
cargo run --release --bin labs -- --bridge       # the bridge thesis at scale
cargo run --release --bin labs -- --listening      # the voice of those who pay for the error
cargo run --release --bin labs -- --schema      # the contract migration chain
cargo run --release --bin labs -- --jev         # the Jev-LLM bridge
cargo run --release --bin labs -- --resolution   # the R-axis and predictions 3 and 5
~~~

The wave engine enters the first block because it is an instrument, not a chapter: it produces no
published number and replaces no laboratory. Its two divergences from the draft that
originated it are declared in `crates/stochology-wave/src/lib.rs` — the frustration `<Psi, L Psi>` cannot serve as a
conflict trigger because in symmetric antiphase it equals zero precisely when conflict exists, and the
plasticity gate enforces target resonance gain rather than monotonic frustration decay.

The three dry-run rehearsals (`--rehearsal-*`) are rehearsals, not coverage: they run the instruments of points 12, 13, and 16 over synthetic
records to measure the floor of a fluent uncalibrated respondent and to expose what the gate cannot
distinguish. No line in them counts a heard group, a selected case, or an assigned role, and their output carries the
`SYNTHETIC` label. The panel rehearsal measured two things that remain declared: the fluent respondent (p = 0.75
without information) has negative skill, and the perfect oracle fails the skill guard because with
perfect accuracy the climatological variance is zero and the 0/0 ratio evaluates to 0. The listening rehearsal measured the boundary of the gate:
it checks fields rather than provenance, so seven fabricated records open the box — true coverage requires a
signed transcript hash, which does not exist today.

No number enters the book without coming from one of these commands. Whenever a value diverges from the legacy registry, a
**divergence declaration** states both values and the cause in Appendix E (provenance of the numbers), chapter by chapter.
The chapters themselves carry content only: no path, no reference to the book, the repository or the program, and the R-axis
block, or the slice of it that teaches, only in the chapters that measure a resolution loss or a threshold (5, 8, 10, 16, and 21). This is the owner's rule of 2026-10-18,
which moved the declarations out of the chapters and rewrote the book part by part (`docs/declarations/BOOK_REWRITE.md`); since then,
`scripts/style/check_pass.py` checks each rewritten file for the ramp markers, the word floor, and number provenance (every number
in running prose must exist in a source or in an output block of the same file), and `check_labs.py` keeps every block equal to
its instrument.

## VI. Rule on New Files

New working files — intermediate copies, one-off scripts, drafts — are **deleted after
integration**. Only artifacts with an ongoing function (`scripts/book/build.py`, the two book verifiers,
the results generator, the seal verifier, the style tooling, the crates) or permanent records
(chapters, declarations, manifests, reports) remain in the repository.

**History, not versions** (owner's rule, 2026-10-18). The project has no versions: no numbered editions, no
version tags or releases, no "v2" of a document, and no second copy of a file kept alongside the first. What
changes is recorded as history only — the git log and the dated records in `history/` — and a correction is a
dated rectification in the file it corrects, with both values. `scripts/verify.sh` fails if the repository
carries any tag. Three things keep a number because they are not versions of the project: the Rust language
edition and the crate `version` fields that Cargo requires; the card schema version (1.0 to 1.2), which is a data
contract whose migration chain is itself its history; and the labels v1 to v4 in `empirical/p03-multiplicidade/`,
which name the specifications that were tried and are the record that the counter $K$ counts.

## VII. Project State

| Layer | Location | State |
| --- | --- | --- |
| Book | `book/stochology-textbook.md`, authored in `book/` | 21 chapters, 5 parts, assembled and verified |
| Site | `site/` (the Docsify page and `build.sh`), published by `.github/workflows/pages.yml` at obestafera.com | assembled from `book/` on each push to `main`, with the sidebar generated from the headings; the site keeps no copy of the text in the tree |
| Apparatus | `crates/stochology-*` | nine live crates after the consolidation of 2026-10-17: `-core` (with the `audit` and `schema` modules), `-hypergraph`, `-methods` (ICP, drift, fairness), `-horizons` (H0 to H3), `-s2`, `-gates`, `-book`, `-wasm`, and `-wave`; no name from the roadmap was left behind, and every name that disappeared lives on as a module of the crate that absorbed it (`docs/declarations/CRATE_CONSOLIDATION.md`) |
| Results | `docs/RUST_RESULTS.md` | registry generated directly from the instruments |
| Empirical registry | `docs/EMPIRICAL_REGISTRY.md`, `empirical/`, `governance/`, `data/` | every number tagged with the instrument that reproduces it; the Python scaffolding left on 2026-10-16 |
| Language | every layer | **English migration of 2026-10-17**: crates, binaries, tooling, book, archives, and paper; the declared contract changes and what stayed frozen in Portuguese are in `docs/declarations/ENGLISH_MIGRATION.md` |
| Retired | commit `1bfec905` | the Python simulation code and artifacts not significant to the theory |
| Declared and not enforced | `docs/DECLARED_PROPOSALS.md` | **four live items, held at their synthetic rehearsals by the owner's decision of 2026-10-18** (the next step is sharing with outside specialists), and they are not of the same nature: **3** human SHELF elicitation (the full panel), **5** the voice of those who pay for the error (0 of 7 groups heard, deadline 2026-12-15), and **9** the bridge thesis at scale (0 of 12 cases, window 2027-01-14) depend on **people outside the program** — consent, listening, and blind judgment, which no line of code can replace; **13** the Jev-LLM bridge depends on **artifact and code** — open weights frozen with a declared `sha256` and a local runner to load them —, and the runner has existed in `crates/stochology-s2/src/runner.rs` since 2026-10-17, with the operator manual in `empirical/p20-jev/RUNNER.md`; what is missing is the weights file and the hash declaration, which is a machine and operator operation rather than a panel. Items 1, 2, 4, 6, 7, 8, 10, 11, and 12 left the list on 2026-10-16, with proof in §2 of the same file; Phi5, the resolution field, System 1, the funnel, the holdout vault, the queue, and System 2 left earlier upon implementation |

## VIII. What Does Not Enter This Repository

- **Simulation Python:** retired on 2026-10-16 (`docs/declarations/REMOVED_PYTHON.md`). What remains in Python
  are the verification tools for the book and the registry, and nothing else: `scripts/book/build.py`, the two book
  verifiers, `scripts/book/generate_results.py`, `scripts/check_seals.py`, and the style tooling in `scripts/style/`. The
  style tooling is authorized here because the prose pass is part of the continuous verification cycle,
  is declared in `scripts/style/decisions.md`, and is part of the mandatory verification in §V. Any verification tool
  that does not enter §V does not stay in the tree: the four tools remaining outside the direct command list either run through it
  (`scripts/style/scan.py` is imported by `scripts/style/check_pass.py`) or serve the rereading declared in
  `scripts/style/decisions.md`, and what did not serve — the three manifests and two transcripts — moved to `history/`.
- **The legacy course and its numbering:** the course *Critical Thinking and Stochastic Models* has been obsoleted; the labels
  M0–M15, ME1–ME24, and MB1–MB8 are historical and are not current nomenclature.
- **Python operator tool:** `scripts/generate_fire_sources.py` is authorized here because it prepares the
  raw inputs for item 13 outside the tree — case catalog, frozen inference, and newline-escaped
  serialization —, and the input it produces enters through the ramp of `ingest-jev`, which is Rust and enforces the schema. It
  computes no published number: what it writes is a raw source, and a source only becomes a number after the seals and the
  instrument. Any synthetic source it generates is labeled as synthetic.
- **Artifacts without significance for the theory:** the criterion lives in `book/significant-results.md`, and its
  latest application is recorded in `docs/declarations/REMOVED_NON_SIGNIFICANT.md`.
