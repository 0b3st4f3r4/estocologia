# Stochology: Judgement and Decision under Uncertainty

Stochology is the science (or art) of judgement and decision under uncertainty: what a number authorises one to
conclude, what it forbids, and who pays when the decision is wrong. This repository holds a book and the apparatus
that produces every number in it.

**Start here.**

- **Read the book:** [obestafera.com](https://obestafera.com), or `book/stochology-textbook.md`. It reads as two
  books in one: Parts I to IV are the method (the mean that lies, the filtered sample, the severe test, calibration,
  causes that survive a change of environment, time and ruin, the price of error) and three real cases; Part V is a
  machine built to audit that method, with its record on real data, failures included. Reading tracks for four
  readers are in the Preface.
- **Run a laboratory.** Every number in a chapter's Step 3 comes from a fixed-seed instrument; the same seed gives
  the same number on your machine:

  ~~~bash
  git clone https://github.com/0b3st4f3r4/stochology && cd stochology
  cargo run --release --bin labs -- --chapter 5      # the health post: when the mean lies
  cargo run --release --bin labs -- --streams        # six real streams on public data
  ~~~

- **Check a number.** Appendix C lists every command; Appendix E lists every number that has no instrument, every
  error found so far with both values, and where each dataset comes from (`data/SOURCES.md`).
- **Report an error:** open an issue with the *errata* template (`CONTRIBUTING.md`, `docs/OUTSIDE_AUDIT.md`).

**What this is not yet.** A working text, not a textbook. Its central thesis (most fallacies are misalignments
between the world, the model and the agent, not errors of arithmetic) has had one direct test, which tied at the
ceiling. Seven decisions on people were analysed without hearing the people affected. Every audit from outside the
authors' own so far was made with an AI assistant. These are the open items of `docs/DECLARED_PROPOSALS.md`, with
their deadlines, and they depend on people outside the programme.

**Use of AI, declared.** The theory, the architecture and the research are the author's. From 2026-10-18 on, an AI
assistant (Claude, Anthropic) made the critical readings of the book, the reimplementation of its laboratories from
the text, the rebuild of the real streams in Rust, and the corrections those found; each pass is a dated record in
`history/`, with its commits attributed, and Appendix E lists what each one changed. Those readings stand outside
the author's own audit, but not outside the tools the author used, and no person outside the programme has checked
them. They do not count as the independent audit the book asks for.

**Licences.** Two, by kind of work, decided by the owner on 2026-10-19. The **text** is under the Creative Commons
Attribution 4.0 International licence (CC BY 4.0, `LICENSE-TEXT`): the book (`book/`), the paper with its figures
(`paper/`), the site's text (`site/home.md`) and the prose documents of the repository (the Markdown files outside
`crates/` and `scripts/`). Anyone may copy, adapt, translate and reuse it, commercially or not, crediting
"Silvano Neto, *Stochology*, github.com/0b3st4f3r4/stochology", linking the licence and saying what was changed.
The **code** (`crates/`, `scripts/`, `site/index.html`, `site/build.sh`) is under the MIT licence (`LICENSE`). The
**datasets** in `data/` keep the licences of their sources, declared one by one in `data/SOURCES.md`; neither
licence above covers them.

---

The research concluded on 2026-10-15 and the apparatus is **Rust** (Edition 2024): **eight crates**, **444 tests**,
**twenty-nine instruments** (plus 5 modifier flags) in the `labs` binary, and **seven gates** (G1 to G7) in the `gates`
binary that are capable of failing a claim. The
gates check form, presence and reproducibility, not correctness: on 2026-10-18 an outside reading found errors in
mathematics and code that every gate had passed, corrected them, and recorded both values in Appendix E of the book
(see `docs/OUTSIDE_AUDIT.md`). The architecture: a typed hypergraph, the World/Model/Agent layers, the safeguard
panel A1–A8, the orchestrator M, structured specification sheets and the invariants Φ1–Φ5, run from H0 to H3 on
real-world data.

## Repository Contents

| Path | Contents |
|---|---|
| `book/stochology-textbook.md` | The book in **21 chapters and 5 parts**, three tiers per chapter, with laboratory outputs cited directly in the text. It is the human-reader artifact, and it is **generated**: 7 080 lines and 100 122 words assembled from `book/` |
| `book/` | The book source, one file per chapter, assembled and verified by `scripts/book/build.py`, alongside the two book verifiers and the results generator in `scripts/book/` |
| `crates/` | The Rust workspace, **eight crates**: `-core` (layers, cards, invariants Phi1 to Phi5, safeguard panel A1 to A8, orchestrator M, System 1, funnel, SHA-256 audit trail, and the file-backed contract with its migration chain), `-hypergraph` (typed hypergraph and the five lenses), `-methods` (invariant causal prediction, CUSUM drift, subgroup fairness, and the e-process ledger for gates attempted in rounds), `-horizons` (the horizons H0 to H3), `-s2` (deliberative System 2), `-gates` (binary `gates`), `-book` (book laboratories, binary `labs`), and `-wasm` (WebAssembly playground); the wave engine left on 2026-10-19 (`docs/declarations/REMOVED_WAVE.md`) |
| `docs/EMPIRICAL_REGISTRY.md` | The research numbers, with the Rust instrument that reproduces each one and the declared divergence markers |
| `docs/RUST_RESULTS.md` | The registry generated directly from the instruments themselves: each block is the literal output of the command above it. §5 is the state manifest, with the `sha256` of every point declaration |
| `data/` | Thirteen datasets: ten read by the instruments (`bank_calls.csv`, `djia_1885_2023.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `danish_fire.csv`, and, since 2026-10-19, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `vic_elec.csv`, `atlantic_storms.csv`, `toronto_arrests.csv`) and three kept as the sealed data of point 24 (`nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, `houston_flights_2011.csv`), with provenance in `SOURCES.md` and checksums in `SHA256SUMS` |
| `empirical/` | The foundational empirical points: central thesis, independence, multiplicity, priors, performative loop, ICP, calibration, genetics, H3, R-axis predictions, bridge at scale, SHELF, affected-party listening, second opinion, Jev, whether the layers can fail, and the stopping rule. Each folder contains its declaration and `*.sha256` seal |
| `governance/` | Risk opinions and the audit of who pays for the error |
| `AGENTS.md` | The governance, engineering, and orchestration contract: the four prohibitions, the invariants, the safeguard panel, the cycle, and the **mandatory verification of §V** |
| `docs/PLAN.md` | Master plan: epistemology, pedagogy, architecture, safeguard panel A1–A8, and governance |
| `docs/DECLARED_PROPOSALS.md` | The inventory of what is declared and not yet enforced or measured: **seven live items** — SHELF (3), the voice of those who pay for the error (5), the bridge thesis at scale (9), the Jev-LLM bridge (13), whether the layers can fail (14), the book as its own source (16), and an audit from outside (17); item 15 (the H2 gate under an e-process) closed on 2026-10-19. The ten closed items are in §2, alongside their proofs |
| `docs/OUTSIDE_AUDIT.md` | Why an audit by the authors is not enough, measured on 2026-10-18, and the protocol for one from outside: public errata, blind reimplementation, red team |
| `docs/STATE_DECLARATION.md` | **The state of the tree, not an edition.** There are no numbered editions or versions, before or after publication: there is a live state recording what it measured, what it rejected against itself, what it could not do, and how it is reproduced. Its seal is `docs/STATE_DECLARATION.sha256`. The corrections of 2026-10-17 live in `history/` as dated records with before-and-after hashes |
| `references/` | Verification of the bibliographic references |
| `paper/` | The paper "Stochology: Old Frontiers, New Horizons", with figures |
| `scripts/style/` | The prose-pass tooling: `scripts/style/check_pass.py` compares each file against its version in `HEAD` |
| `scripts/check_seals.py` | The verifier for the 32 SHA-256 seals across the tree, executed in a single command from a single directory |
| `scripts/verify.sh` | **The mandatory verification of §V in a single command**: formatting, lints, tests, eight gate suites, twenty-seven instruments (all but the slow `--seed-spread`), and all verifiers. CI invokes this exact file, not a copy of the list |
| `docs/JEV_INTEGRATION.md` | The audit of the transcript *Integration of Jev Models into Stochology*: every claim checked against the code, with declared divergences — the legacy syllabus numbering versus the current nomenclature ($M_{15}$ and $MB_1$–$MB_8$ on one side, $\mathcal{M}$ and $A_1$–$A_8$ on the other), four filters versus five, $\Phi_1$–$\Phi_4$ versus $\Phi_1$–$\Phi_5$, and *LLM-as-Jev* — plus the Jev-LLM bridge as a declared proposal |
| `docs/declarations/REMOVED_PYTHON.md`, `docs/declarations/REMOVED_NON_SIGNIFICANT.md` | What left the tree on 2026-10-16, with the recovery commit and the blob-hash manifests |

## Measured State

Measurement of this tree on **2026-10-19**, each row with the command that produces it:

| Metric | Value | Command |
|---|---|---|
| Tests | **444 passed**, 0 failed, across the eight crates (439 before the four tests of the separated Merkle root and one of the oracles of Chapter 5, on 2026-10-20; 434 before the five tests of the analytic oracles, `oracles.rs`, on 2026-10-20; 433 before the test of the WebAssembly symbols renamed with the repository on 2026-10-20; 426 before the seven tests of item 19: three of `conduct.rs` and two of `schema.rs` for contract 1.5, and two of `seed_spread.rs`; 407 before the nineteen tests of 2026-10-19 for item 18: three of `passk.rs`, four of `multiverse.rs`, three of `stacking.rs`, four of `discrepancy.rs`, two of `budget.rs` and two of `schema.rs` for contract 1.4, and one of the seed register in `mc_error.rs`; 401 before the six tests of contract 1.3, `budget.rs` and `schema.rs`; 395 before the three tests of `mc_error.rs` and the three of `propagate.rs`; 442 before the wave engine left with its 38 tests and the 9 of its three instruments; 440 before the two tests of point 24, `resonance.rs`; 438 before the two tests of declaration 2 of point 23, `wave2.rs`; 427 before the refinement and the test of the wave engine, point 23: six tests of `channel.rs` and `cycle.rs`, five of `wave.rs`; 422 before the five tests of the e-process reading of the H2 gate, `eprocess.rs`; 421 before the null of the ECE entered `streams.rs`; 415 before the rebuilt streams of 2026-10-19 added six, the tests of `streams.rs`; 413 before the recipes of 2026-10-19 added two: the bounds of the garden of paths and the declared limit of the Merkle root; 409 before the reimplementation of the chapters on 2026-10-19 added four tests that fail on the old code; the 406 declared on 2026-10-18 was stale: the base of the rectifications of 2026-10-19 measured 408, which retired one panel test and added two tests of Chapter 10; 395 before the rectifications of 2026-10-18, which added eleven tests that fail on the old code; the 373 written here earlier was already stale: the tree measured 377 before the e-process ledger added nine and the gate of thesis (c′) another nine) | `cargo test --workspace` |
| Instruments | **29** in `labs` (one, `--seed-spread`, takes minutes and is recorded in `docs/SEED_SPREAD.md` rather than run by the suite) (plus 5 modifier flags), all exiting with code 0 | `labs --all`, `--bank`, `--queue`, `--s1`, `--s2`, `--resolution`, `--genetics`, `--streams`, `--eprocess`, `--mc-error`, `--propagate`, `--passk`, `--multiverse`, `--oracles`, `--stacking`, `--discrepancy`, `--h3`, `--second-opinion`, `--bridge`, `--listening`, `--schema`, `--jev` and the others |
| Gates | **7 ok**, Merkle root `9716e28a87bec9cf…` and identical second run | `gates --horizon all --reproduction` |
| Negative proof | **1 gate failed**, as declared, with the board at worst score 2 against floor 6 | `gates --horizon h3 --defect h3.voos23` |
| Assembled book | **7 080 lines**, **100 122 words**, 21 chapters, 5 parts | `scripts/book/build.py --check` |
| Cited blocks | **39**, none divergent (48 counted the R-axis lines before the book was rewritten as content only) | `scripts/book/check_labs.py` |
| Seals | **36 files**, **143 verified targets**, 18 lines declared outside the tree, 0 issues | `scripts/check_seals.py` |

## How to Verify

~~~bash
# the machine
cargo test --workspace
cargo run --release --bin labs -- --all
cargo run --release --bin labs -- --s2
cargo run --release --bin gates -- --horizon all --reproduction
cargo run --release --bin gates -- --horizon h3 --defect h3.voos23   # exits with 1

# the book: the ramp, the disk, the cited blocks, and the prose
python3 scripts/book/build.py --check          # compares with disk character by character and enforces the declared count
python3 scripts/book/check_labs.py             # checks every cited block against the instrument that generated it
python3 scripts/book/check_book.py             # parts, chapters, fences, and ordering
python3 scripts/book/generate_results.py --check
python3 scripts/style/check_pass.py            # prose pass against HEAD:book/, file by file

# the seals, in a single command, from any directory
python3 scripts/check_seals.py

# the playground
cargo build --release --target wasm32-unknown-unknown -p stochology-wasm
~~~

The binding list is in `AGENTS.md` §V: no number enters the book without coming from one of those commands.

## The Rule the Tree Learned

A gate that does not inspect its target is a blind gate. Previously, `scripts/book/build.py --check` assembled the book in memory and
checked only the 21-chapter ramp: it exited with code 0 while the file on disk was **353 lines behind**
`book/`. Today `--check` compares `montar()` against `book/stochology-textbook.md` character by character,
prints the diff, and exits with 1 whenever the disk does not match the assembled book. The same principle applies to the seals: one
command, one path convention (relative to the repository root), and any seal without a live target is reported as **declared**, never as
passed. And it applies to the line/word count: the book gained two lines in `book/cap13.md`, the manifest was regenerated, and three
documents continued claiming 5 903 lines — while `--check` passed because it only inspected the text. Since the
rectification of 2026-10-17, it parses the "N lines ... N words" pairs in `README.md`, `docs/PLAN.md`, the state declaration,
and the results registry, compares them against what `montar()` measures, and exits with 1 if they diverge.

## What Remains Open

The open items are not of the same nature. Items 3, 5, 9, and 14 depend on **people outside the program** and cannot be closed with
more code: what is missing is consent, listening, blind judgment, and independent coding. Item 13 depends on **artifact and code**: frozen
weights with a declared `sha256` and a runner that loads them, which closes with a download, a hash declaration, and an
implementation. Item 15, desk work, closed on 2026-10-19 with a declaration sealed before the run and `labs --eprocess`;
item 16 closed the same day: the previous book no longer certifies its own numbers, and the 29 that no instrument,
code or record carries have a checked row in `book/provenance.tsv`. Item 18, from the research on models of the world, of agents
and of modelling, closed the same day with four laboratories sealed before their runs, the card contract 1.4 and three
audits; item 19, what those audits found open, closed the same day with contract 1.5 and the audit of the seed over whole laboratories. What remains open needs people or an artifact. Items 14 and 15, and the second
thesis (c′) of item 9, came from an outside review on 2026-10-18 by a language model plausibly of the same family as the
program's own agents, so they are refinements, not independent judgment.

| Item | Dependency | What is missing | Deadline |
|---|---|---|---|
| 3. SHELF elicitation | **people outside the program** | the other two participants per domain and the external control; the first specialist's track is closed across five domains | no date |
| 5. The voice of those who pay for the error | **people outside the program** | listening sessions with affected groups: **0 of 7** heard, minimum coverage of 2 | **2026-12-15** |
| 9. The bridge thesis at scale | **people outside the program** | external case selector, two human analysts, and a blind human judge: **0 of 12** cases; for thesis (c′), 0 of 12 in each of the seam and classical strata (`registro_c_linha.tsv`, enforced by `labs --bridge`) | **2027-01-14** |
| 13. The Jev-LLM bridge | **artifact and code** | the weights with a declared `sha256`; the local runner has existed since 2026-10-17 (`crates/stochology-s2/src/runner.rs`) | no date |
| 14. Can the layers fail? | **people outside the program** | three coders, two from outside, over at least 60 documented errors (`empirical/p21-camadas/`) | no date |

Once the listening window expires without minimum coverage, the seven decisions are archived as a sensitivity
exercise and do not become public policy guidelines. Once the bridge window expires, the thesis becomes **definitively
unsupported**, as the project owner accepted in writing.

## What Left the Tree

On 2026-10-16 the Python measurement scaffolding left the tree — the code, the `h0` through `h3` fronts with their
READMEs, and the datasets used exclusively by them. The numbers remained: in `docs/EMPIRICAL_REGISTRY.md` and in the book, with commit
`1bfec905` as the recovery anchor and the manifests `history/python-removido.txt`, `history/registro-python-removido.txt`, and
`history/nao-significante-removido.txt` recording the blob hash of every file. What remains in Python are the verification tools for
the book, the registry, and the seals — and nothing else.
