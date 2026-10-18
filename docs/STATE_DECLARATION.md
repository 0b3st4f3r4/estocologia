# Declaration of the State of the Tree

**This is a state, not an edition.** The research and the textbook have not been published. There is no numbered
edition or version, no "freeze" with an ordinal number, and no second or third edition, before or after publication: there is a **state of the tree**,
which this file declares, and there is a **dated record** of the corrections that brought the tree to this point, in `history/`
— `estado-2026-10-17T0810Z.md` and `estado-2026-10-17T0840Z.md`, each recording the before-and-after hashes and the
file names as they existed at that moment. Numbering editions stamps as final what is still being corrected, and the project keeps only history
(the git log and the dated records in `history/`), by the owner's rule of 2026-10-18 (`AGENTS.md` §VI).

What this file declares: what the tree contains, what it rejected against itself, what it could not do, what
would change each verdict, and how everything is reproduced. The manifest with the hash of every point declaration lives in
`docs/RUST_RESULTS.md` §5, **generated** by `python3 scripts/book/generate_results.py` and verified by `--check`. The seal
of this file is `docs/STATE_DECLARATION.sha256`.

The path to this state, across three dated corrections, with each fix declared in its proper place: (1) the assembled book fell
353 lines behind `book/` and `scripts/book/build.py --check` was updated to compare the file on disk against what `montar()` produces (§6); the
seal of `CEGAMENTO.md` had been stale since 2026-10-15 and the seal path convention had two competing rules (§3);
the reproduction recipe did not run the instruments proving the closed items (§5). (2) A hand-written line count
survived the growth of the book, and `--check` gained its third check, parsing the "N lines and N words" pair
across the four canonical documents. (3) Two documentary claims fell upon measurement — that all 50 rows of D.2 through
D.4 carried the verification tag, when 27 carried it, and that the three reports could be recovered from `1bfec905` via `git checkout`,
when `book/` does not exist in that commit —, and the record of each is in `references/VERIFICATION.md` and
`docs/declarations/LOST_REPORTS.md`.

## 1. The Nine Closed Items, with Proof and Command

| Item | Proof | Command |
| --- | --- | --- |
| Deliberative System 2 | `crates/stochology-s2`, ten tested modules (81 unit tests and 7 integration tests) | `cargo test --workspace` and `labs --s2` |
| Roadmap crates | all ten live crate names, each with its own test suite | `cargo test --workspace` |
| Fine resolution in genetics | `stochology-horizons/src/genetica.rs`: 4 of 40 pass with one line, 1 of 40 with two, floor tightened from 0.001 to 0.000001 | `labs --genetics` |
| Bibliography verification | all 50 rows of D.2 through D.4 marked with **[conferido]**, 50 verified against DOI, publisher, catalog, or proceedings, 0 content divergences. **Correction of 2026-10-17:** this row previously stated "50 rows with the tag" and "27 verified", and both could not be true simultaneously — the tag was present on 27, while the 23 in D.3 had been verified on 2026-10-15 without the tag. The 2026-10-17 re-execution re-verified all 23 and applied the tag | `references/VERIFICATION.md` (sections "As 27 linhas de D.2 e D.4" and "As 23 linhas de D.3") and `book/appendices.md` |
| H3 chain | `stochology-horizons/src/cadeia.rs`: one derived row, three declared with their rationale | `labs --h3` |
| R-axis predictions 3 and 5 | threshold at amplitude 2.5 with the calibrated plan (+2.87 min loss, negative below it); transfer with 0 violations and 3 counterexamples | `labs --resolution` |
| Contract migrations | chain in `migracoes.tsv`, embedded at compile time, 1.0 → 1.1 → 1.2 | `cargo test -p stochology-core` and `labs --schema` |
| R-axis across chapters | the 21 R-axis rows, computed by the instrument, in Tier 3 of each chapter | `labs --resolution` and `python3 scripts/book/check_labs.py` |
| Registry second opinion | five rows verified by a second implementation, two derived, seven declared as registry records | `labs --second-opinion` |

## 2. The Four That Remain, and Why

| Item | What is missing | Who resolves it | Deadline |
| --- | --- | --- | --- |
| SHELF elicitation (3) | the other two participants per domain and the external specialist | people | no date; the first specialist's track is closed across five domains |
| Voice of those who pay (5) | listening sessions with the seven affected groups, with signed records | people | **2026-12-15** |
| Bridge thesis at scale (9) | external case selector, human analysts, and human judge | people | **2027-01-14**; without them, the thesis becomes definitively unsupported |
| Jev-LLM bridge (13) | **the weights**, with their `sha256` declared prior to execution, and **the labeled outcome set** in `empirical/p20-jev/calibracao/desfechos.tsv`. The runner has existed since 2026-10-17 (`crates/stochology-s2/src/runner.rs`) as has the Brier reader (`calibration.rs`, minimum sample of 30, abstentions excluded from the denominator); the operator manual is `empirical/p20-jev/RUNNER.md`, the `ingest-jev` binary prepares both files with a dry run before persisting, and closure has a pre-data protocol declared in `empirical/p20-jev/FECHAMENTO.md` | the operator, with two files | no date |

**What the four do not have in common.** Three depend on **people outside the program** and cannot be resolved with code:
what is missing is consent, listening, and blind judgment, which is why the instruments lock the box rather than
filling the gap. The fourth depends on **artifact and code**: the weights with `sha256` and the runner that loads them, and
the runner is already written; what is missing is the file. The criterion for each is declared prior to inspecting any number.

## 3. What This State Rejected Against Itself

| What fell | What took its place | Instrument |
| --- | --- | --- |
| the bridge thesis at scale | **unsupported**: ceiling tie, 8 of 8 in both arms and 0 false alarms | `labs --bridge` |
| the loop declared at 0.6 | is **4.1483 times** the measured value, outside the 1/3 to 3 band fixed by pre-registration | `labs --h3` |
| the 0.10 ECE threshold | was a wall, not a gate: it passed 0.08% of calibrated models under the protocol design | `labs --preteste` |
| single-line validation in genetics | a floor of 1/1 000 lets 4 of 40 through, whereas the second line separates them | `labs --genetics` |
| fixed calibration tolerance | with an n-dependent tolerance, none of the six binary vaults pass | `empirical/p08-calibracao` |
| the machine field for priors | marks `true` in 8 of the 33 domains, while the published sentence states 33 of 33 | `labs --second-opinion` |
| listening to those who pay | **0 of 7** groups heard, and the box sealed by the instrument | `labs --listening` |
| the seal of `p01-tese/CEGAMENTO.md` | **re-sealed**: the hash recorded at 12:42 on 2026-10-15 (`fcf8da5a…`) ceased to hold when the addendum of commit `935807e` at 15:36 modified the file while the seal still pointed to the earlier text; it now equals `d92afa7c…` | `python3 scripts/check_seals.py` |
| the seal path convention | **unified**: all 28 files now use paths relative to the repository root (including `data/SHA256SUMS` and `governance/DECLARACAO.sha256`), the 9 lines pointing to removed Python code, to the case folder that left the tree, or to `crsp_day_1989_1998.csv` became declared comments, and `sha256sum -c` no longer depends on the working directory from which it is invoked | `python3 scripts/check_seals.py` |
| tree hygiene | **measured and corrected**: `cargo fmt --check` and `clippy -D warnings` pass across all nine crates (down from 615 formatting diffs and 35 warnings), the toolchain is pinned in `rust-toolchain.toml`, `scripts/verify.sh` is now the single list of §V — CI previously ran 11 of the 29 commands —, and dead root files moved to `history/`. No published number changed: **373 tests** and `docs/RUST_RESULTS.md` identical byte for byte | `scripts/verify.sh` |
| the per-instrument registry | **was machine-bound**: each block cited the absolute binary path (`/home/silvano-neto/…/target/release/labs`), so `scripts/book/generate_results.py --check` could only pass on this machine and failed in CI from 2026-10-17T16:23Z onward. The command is now recorded relative to the root (`./target/release/labs`), and `--check` now prints the diff divergence instead of merely failing | `python3 scripts/book/generate_results.py --check` |
| the English migration of 2026-10-18 | **broke CI and left stale pointers**: renaming `verificar.sh` to `scripts/verify.sh` dropped the executable bit (mode `100644`), so CI run 8 on `85d3ade` failed with `Permission denied` before any check ran; the registry also captured cargo's `has been running for over 60 seconds` notice on a slow machine, making `--check` depend on the clock. The bit is restored, `normalize` drops that notice, and `docs/DECLARED_PROPOSALS.md` and this file now point to the nine crates and the English file and test names (`bridge.rs`, `calibration.rs`, `vault.rs`, `layer.rs`, `invariants.rs`). No published number changed | `scripts/verify.sh` |
| versions in the prose | **replaced by history** at the owner's rule of 2026-10-18: "edition" no longer names a state of this project. The book calls the 14-chapter text of 2026-10-15 "the earlier text", the docs say "migration" and "state manifest", and `scripts/verify.sh` fails if any tag exists. The assembled book lost one word (96 018 to 96 017) and kept its 7 178 lines; no number in it changed | `scripts/verify.sh` |

## 4. What Would Change Each Verdict

- **Listening:** two of the seven groups with signed records by 2026-12-15 open the box for the covered decisions; without
  that, the artifact remains archived as a sensitivity exercise.
- **The bridge:** the external case selector, the human analysts, and the human judge by 2027-01-14 put the fair
  test into motion; without them, the thesis remains definitively unsupported, as the project owner accepted in writing.
- **SHELF:** the other two participants per domain and the external specialist make the elicitation valid in the official
  round; today it stands as a sensitivity run with n = 1.
- **Jev:** the weights matching their `sha256` and the local runner unlock the calibrated Brier score, true cost, and
  abstention rate of the actual model.
- **The p05 divergence:** a human decision on the semantics of the `reproducao_rodada_a` field resolves what the
  second opinion recorded without guessing.

## 5. Reproduction

~~~bash
cargo test --workspace                                  # 373 tests passed, none failed
cargo run --release --bin labs -- --all               # the 21 laboratories
cargo run --release --bin gates -- --horizon all --reproduction
cargo run --release --bin gates -- --horizon h3 --defect h3.voos23   # exits with 1, as declared
python3 scripts/book/build.py --check                   # the 21-chapter ramp AND the file on disk
python3 scripts/book/check_labs.py                      # 48 cited blocks, 0 divergent
python3 scripts/book/check_book.py
python3 scripts/book/generate_results.py --check        # the registry and the state manifest
python3 scripts/check_seals.py                          # the 28 seals: 82 targets, 9 outside the tree, 0 issues
python3 scripts/style/check_pass.py                     # prose pass against HEAD:book/
~~~

The instruments proving the closed items in §1 are also part of the recipe, because the table in §1 cites each
of them and a recipe that omits them fails to verify its own table:

~~~bash
cargo run --release --bin labs -- --genetics    # fine resolution in genetics: 4 of 40 with one line, 1 of 40 with two
cargo run --release --bin labs -- --h3          # the H3 chain: one derived row, three declared
cargo run --release --bin labs -- --second-opinion     # second opinion: five verified, two derived, seven registry
cargo run --release --bin labs -- --bridge       # bridge thesis: re-derived pilot and fair-test gate
cargo run --release --bin labs -- --listening      # voice of those who pay: 0 of 7 heard, minimum 2
cargo run --release --bin labs -- --schema      # contract migration chain: 1.0 -> 1.1 -> 1.2
cargo run --release --bin labs -- --jev         # Jev-LLM bridge: gates close, weights missing
cargo run --release --bin labs -- --resolution   # R-axis and predictions 3 and 5
~~~

Each point folder has its own `DECLARACAO.sha256`, and all 28 seals in the tree are verified by a single command
run from the root: `python3 scripts/check_seals.py` returns **82 verified targets**, **9 lines declared outside the
tree**, and **0 issues**. Two seals have no live target and appear as declared rather than passed:
`p01-tese/CASOS.sha256` (the 12-case bank left the tree) and `p01-tese/GERADORES.sha256` (the generator was
Python).

## 6. The Manifest

In `docs/RUST_RESULTS.md` §5, generated: **20 point declarations**, of which 4 are second-or-later declarations (corrections
with both values side by side), **16 foundational empirical points**, **19 instruments** (plus 5 modifier flags) in `labs`, **21 chapters**, and
**9 crates**. Each row of the manifest carries the `sha256` of the declaration file.

The assembled textbook enters the same manifest with its identifying metrics: **7 178 lines**, **96 017 words**, and
the `sha256` of the file. The state of 2026-10-17 had a defect that this paragraph closes: the earlier
`scripts/book/build.py --check` assembled the book in memory and checked only the ramp without inspecting the file on
disk — while the file on disk was **353 lines behind** `book/`, missing the 21 R-axis blocks and the
rewrites of items 3 through 13. The gate covered the engine rather than the deliverable. Now `--check` compares
`montar()` against `book/stochology-textbook.md` **character by character**, prints the diff, and exits with 1
whenever the disk is not the assembled book — and enforces a third requirement: that the count declared in `README.md`,
`docs/PLAN.md`, this declaration, and the registry matches the count measured by `montar()`. The hand-written number
now has an instrument, because the correction of 2026-10-17T08:10Z arose from four documents claiming 5 903 lines
while the file on disk had 5 904.

## 7. What Remains Written

This state leaves no promises. What it measured is in the registry, with the instrument beside it; what it rejected
stands rejected, with the number and the criterion that rejected it; what it could not do is declared, with who
resolves it and by when. The tree is clean, and the transcript that originated the Jev bridge audit
(`history/transcricao-jev-2026-10-16.md`) has been **versioned** since commit `5393515` — the previous version
of this sentence described it as the sole unversioned file, which was false on both counts: the file is tracked in Git and the
tree had six modified files when the correction of 2026-10-17T08:10Z began.

Anyone wishing to verify need not take anything on faith: run the eighteen commands in Section 5 — ten in the reproduction block and eight
for the instruments of the closed items —, plus `sha256sum -c` in each
point folder.


## The English migration (2026-10-17)

On 2026-10-17 the repository migrated to English: the nine crates, the four binaries, the verification tooling, the
textbook, the empirical audit, the governance record, the paper, and the bibliography. No number moved:
`cargo test --workspace` reports **373 passed, 0 failed**, the `gates` merkle root over the twelve registered cards is
still `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81`, and the negative proof still exits 1 with
root `802e84b6...`. The assembled book is **7 178 lines and 96 017 words**, and its 48 cited blocks are the literal
stdout of the English instruments. What moved by construction -- the two wave seals that hash translated text and the
two `gates` report hashes -- and what stayed frozen in Portuguese because it is hashed or read on disk are declared in
`docs/declarations/ENGLISH_MIGRATION.md`. The 23 seal files whose declarations were translated were re-issued at the
end of the pass.
