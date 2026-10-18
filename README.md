# Stochology

Stochology is the science (or art) of studying judgment and decision-making under uncertainty.

This repository holds the research conducted under the project architecture (typed hypergraph, World/Model/Agent
layers, adversarial review board A1–A8, orchestrator M, structured specification sheets, invariants Φ1–Φ5), from H0 to
H3, on real-world data. The research concluded on 2026-10-15 and the engineering apparatus is **Rust** (Edition 2024):
**nine crates**, **395 tests**, **nineteen instruments** (plus 5 modifier flags) in the `labs` binary, and **seven gates** (G1 to G7) in the `gates` binary (distinct from the six gates G1 to G6 of the wave engine `wave`)
that are capable of failing a claim.

## Repository Contents

| Path | Contents |
|---|---|
| `book/stochology-textbook.md` | The textbook in **21 chapters and 5 parts**, three tiers per chapter, with laboratory outputs cited directly in the text. It is the human-reader artifact, and it is **generated**: 5 619 lines and 71 931 words assembled from `book/` |
| `book/` | The book source, one file per chapter, assembled and verified by `scripts/book/build.py`, alongside the two book verifiers and the results generator in `scripts/book/` |
| `crates/` | The Rust workspace, **nine crates**: `-core` (layers, cards, invariants Phi1 to Phi5, review board A1 to A8, orchestrator M, System 1, funnel, SHA-256 audit trail, and the file-backed contract with its migration chain), `-hypergraph` (typed hypergraph and the five lenses), `-methods` (invariant causal prediction, CUSUM drift, subgroup fairness, and the e-process ledger for gates attempted in rounds), `-horizons` (the horizons H0 to H3), `-s2` (deliberative System 2), `-gates` (binary `gates`), `-book` (textbook laboratories, binary `labs`), `-wasm` (WebAssembly playground), and `-wave` (binary `wave`: local phasor superposition, magnetic Laplacian, Kuramoto-Oja plasticity, and STDP spike bus) |
| `docs/EMPIRICAL_REGISTRY.md` | The research numbers, with the Rust instrument that reproduces each one and the declared divergence markers |
| `docs/RUST_RESULTS.md` | The registry generated directly from the instruments themselves: each block is the literal output of the command above it. §5 is the state manifest, with the `sha256` of every point declaration |
| `data/` | The five datasets read by the instruments (`bank_calls.csv`, `djia_1885_2023.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `danish_fire.csv`), with provenance in `SOURCES.md` and checksums in `SHA256SUMS` |
| `empirical/` | The foundational empirical points: central thesis, independence, multiplicity, priors, performative loop, ICP, calibration, genetics, H3, R-axis predictions, bridge at scale, SHELF, affected-party listening, second opinion, Jev, whether the layers can fail, and the stopping rule. Each folder contains its declaration and `*.sha256` seal |
| `governance/` | Risk opinions and the audit of who pays for the error |
| `AGENTS.md` | The governance, engineering, and orchestration contract: the four prohibitions, the invariants, the review board, the cycle, and the **mandatory verification of §V** |
| `docs/PLAN.md` | Master plan: epistemology, pedagogy, architecture, adversarial review board A1–A8, and governance |
| `docs/DECLARED_PROPOSALS.md` | The inventory of what is declared and not yet enforced or measured: **six live items** — SHELF (3), the voice of those who pay for the error (5), the bridge thesis at scale (9), the Jev-LLM bridge (13), whether the layers can fail (14), and the H2 gate under an e-process (15). The nine closed items are in §2, alongside their proofs |
| `docs/STATE_DECLARATION.md` | **The state of the tree, not an edition.** There are no numbered editions or versions, before or after publication: there is a live state recording what it measured, what it rejected against itself, what it could not do, and how it is reproduced. Its seal is `docs/STATE_DECLARATION.sha256`. The corrections of 2026-10-17 live in `history/` as dated records with before-and-after hashes |
| `references/` | Verification of the bibliographic references |
| `paper/` | The paper "Stochology: Old Frontiers, New Horizons", with figures |
| `scripts/style/` | The prose-pass tooling: `scripts/style/check_pass.py` compares each file against its version in `HEAD` |
| `scripts/check_seals.py` | The verifier for the 30 SHA-256 seals across the tree, executed in a single command from a single directory |
| `scripts/verify.sh` | **The mandatory verification of §V in a single command**: formatting, lints, tests, eight gate suites, nineteen instruments, and all verifiers. CI invokes this exact file, not a copy of the list |
| `docs/JEV_INTEGRATION.md` | The audit of the transcript *Integration of Jev Models into Stochology*: every claim checked against the code, with declared divergences — the legacy syllabus numbering versus the current nomenclature ($M_{15}$ and $MB_1$–$MB_8$ on one side, $\mathcal{M}$ and $A_1$–$A_8$ on the other), four filters versus five, $\Phi_1$–$\Phi_4$ versus $\Phi_1$–$\Phi_5$, and *LLM-as-Jev* — plus the Jev-LLM bridge as a declared proposal |
| `docs/declarations/REMOVED_PYTHON.md`, `docs/declarations/REMOVED_NON_SIGNIFICANT.md` | What left the tree on 2026-10-16, with the recovery commit and the blob-hash manifests |

## Measured State

Measurement of this tree on **2026-10-18**, each row with the command that produces it:

| Metric | Value | Command |
|---|---|---|
| Tests | **395 passed**, 0 failed, across the nine crates (the 373 written here before 2026-10-18 was already stale: the tree measured 377 before the e-process ledger added nine and the gate of thesis (c′) another nine) | `cargo test --workspace` |
| Instruments | **19** in `labs` (plus 5 modifier flags), all exiting with code 0 | `labs --all`, `--bank`, `--queue`, `--s1`, `--s2`, `--resolution`, `--genetics`, `--h3`, `--second-opinion`, `--bridge`, `--listening`, `--schema`, `--jev` and the others |
| Gates | **7 ok**, Merkle root `9716e28a87bec9cf…` and identical second run | `gates --horizon all --reproduction` |
| Negative proof | **1 gate failed**, as declared, with the board at worst score 2 against floor 6 | `gates --horizon h3 --defect h3.voos23` |
| Assembled book | **5 619 lines**, **71 931 words**, 21 chapters, 5 parts | `scripts/book/build.py --check` |
| Cited blocks | **48**, none divergent | `scripts/book/check_labs.py` |
| Seals | **30 files**, **90 verified targets**, 9 lines declared outside the tree, 0 issues | `scripts/check_seals.py` |

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

The six open items are not of the same nature. Items 3, 5, 9, and 14 depend on **people outside the program** and cannot be closed with
more code: what is missing is consent, listening, blind judgment, and independent coding. Item 13 depends on **artifact and code**: frozen
weights with a declared `sha256` and a runner that loads them, which closes with a download, a hash declaration, and an
implementation. Item 15 is **desk work**: a declaration written before a recomputation. Items 14 and 15, and the second
thesis (c′) of item 9, came from an outside review on 2026-10-18 by a language model plausibly of the same family as the
program's own agents, so they are refinements, not independent judgment.

| Item | Dependency | What is missing | Deadline |
|---|---|---|---|
| 3. SHELF elicitation | **people outside the program** | the other two participants per domain and the external control; the first specialist's track is closed across five domains | no date |
| 5. The voice of those who pay for the error | **people outside the program** | listening sessions with affected groups: **0 of 7** heard, minimum coverage of 2 | **2026-12-15** |
| 9. The bridge thesis at scale | **people outside the program** | external case selector, two human analysts, and a blind human judge: **0 of 12** cases; for thesis (c′), 0 of 12 in each of the seam and classical strata (`registro_c_linha.tsv`, enforced by `labs --bridge`) | **2027-01-14** |
| 13. The Jev-LLM bridge | **artifact and code** | the weights with a declared `sha256`; the local runner has existed since 2026-10-17 (`crates/stochology-s2/src/runner.rs`) | no date |
| 14. Can the layers fail? | **people outside the program** | three coders, two from outside, over at least 60 documented errors (`empirical/p21-camadas/`) | no date |
| 15. The H2 gate under an e-process | **desk work** | the declaration of which null each of the 38 tests tested and which data it used, before recomputing (`empirical/p22-parada/`) | no date |

Once the listening window expires without minimum coverage, the seven decisions are archived as a sensitivity
exercise and do not become public policy guidelines. Once the bridge window expires, the thesis becomes **definitively
unsupported**, as the project owner accepted in writing.

## What Left the Tree

On 2026-10-16 the Python measurement scaffolding left the tree — the code, the `h0` through `h3` fronts with their
READMEs, and the datasets used exclusively by them. The numbers remained: in `docs/EMPIRICAL_REGISTRY.md` and in the book, with commit
`1bfec905` as the recovery anchor and the manifests `history/python-removido.txt`, `history/registro-python-removido.txt`, and
`history/nao-significante-removido.txt` recording the blob hash of every file. What remains in Python are the verification tools for
the book, the registry, and the seals — and nothing else.
