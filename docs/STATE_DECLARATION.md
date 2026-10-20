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
| the per-instrument registry | **was machine-bound**: each block cited the absolute binary path (an absolute path under the author's home directory, ending in `target/release/labs`), so `scripts/book/generate_results.py --check` could only pass on this machine and failed in CI from 2026-10-17T16:23Z onward. The command is now recorded relative to the root (`./target/release/labs`), and `--check` now prints the diff divergence instead of merely failing | `python3 scripts/book/generate_results.py --check` |
| the English migration of 2026-10-18 | **broke CI and left stale pointers**: renaming `verificar.sh` to `scripts/verify.sh` dropped the executable bit (mode `100644`), so CI run 8 on `85d3ade` failed with `Permission denied` before any check ran; the registry also captured cargo's `has been running for over 60 seconds` notice on a slow machine, making `--check` depend on the clock. The bit is restored, `normalize` drops that notice, and `docs/DECLARED_PROPOSALS.md` and this file now point to the nine crates and the English file and test names (`bridge.rs`, `calibration.rs`, `vault.rs`, `layer.rs`, `invariants.rs`). No published number changed | `scripts/verify.sh` |
| versions in the prose | **replaced by history** at the owner's rule of 2026-10-18: "edition" no longer names a state of this project. The book calls the 14-chapter text of 2026-10-15 "the earlier text", the docs say "migration" and "state manifest", and `scripts/verify.sh` fails if any tag exists. The assembled book lost one word (96 018 to 71 931) and kept its 5 619 lines; no number in it changed | `scripts/verify.sh` |

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
cargo test --workspace                                  # 434 tests passed, none failed
cargo run --release --bin labs -- --all               # the 21 laboratories
cargo run --release --bin gates -- --horizon all --reproduction
cargo run --release --bin gates -- --horizon h3 --defect h3.voos23   # exits with 1, as declared
python3 scripts/book/build.py --check                   # the 21-chapter ramp AND the file on disk
python3 scripts/book/check_labs.py                      # 39 cited blocks, 0 divergent
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
cargo run --release --bin labs -- --schema      # contract migration chain: 1.0 -> 1.1 -> 1.2 -> 1.3 -> 1.4 -> 1.5
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
with both values side by side), **16 foundational empirical points**, **21 instruments** (plus 5 modifier flags) in `labs`, **21 chapters**, and
**8 crates**. Each row of the manifest carries the `sha256` of the declaration file.

The assembled textbook enters the same manifest with its identifying metrics: **7 097 lines**, **100 405 words**, and
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
root `802e84b6...`. The assembled book then had 5 619 lines; since the critical reading of 2026-10-19 and its follow-up it has 7 097 lines and 100 405 words, and its 39 cited blocks are the literal
stdout of the English instruments. What moved by construction -- the two wave seals that hash translated text and the
two `gates` report hashes -- and what stayed frozen in Portuguese because it is hashed or read on disk are declared in
`docs/declarations/ENGLISH_MIGRATION.md`. The 23 seal files whose declarations were translated were re-issued at the
end of the pass.

## The outside review (2026-10-18)

On 2026-10-18 the owner asked for an outside review of the thesis as a whole and then for its proposals to be applied.
The reviewer is a language model (Claude, Anthropic), plausibly of the same family as the models that played every
role of P01, so what it brought is a refinement proposal and not the independent judgment that §2 still waits for. The
dated record, with the hashes before and after, is `history/estado-2026-10-18T2014Z.md`. What changed in this state,
with both values:

| What | Before | After |
| --- | --- | --- |
| `cargo test --workspace` | **373** written here and in `README.md`; the tree actually measured **377** before this change | **386**: nine tests of the e-process ledger in `crates/stochology-methods/src/sequential.rs` |
| point declarations in the manifest | 20, of which 4 second-or-later | **23**, of which 5: `p16-ponte/DECLARACAO_3.md`, `p21-camadas/DECLARACAO.md`, `p22-parada/DECLARACAO.md` |
| empirical points | 16 | **18** |
| seals | 28 files, 82 verified targets | **30 files, 88 verified targets**, 9 declared outside the tree, 0 issues |
| live declared proposals | 4 (items 3, 5, 9, 13) | **6**: item 14, whether the layers can fail (people), and item 15, the H2 gate under an e-process (desk work) |
| the thesis in `paper/README.md` | one sentence carrying three claims | claims (a), (b), and (c) separated, with their status; the Knightian and specification-error rows of the table no longer say the same thing, by a dated rectification with both definitions |

What did not change: every verdict of §3, both frozen windows of §4, `empirical/p16-ponte/registro.tsv`, and every
number of the book. Thesis (c) stays not sustained under its own rule; thesis (c′) is declared and not enforced.

## Thesis (c′) enforced (2026-10-18, later the same day)

The second thesis of point 16 left the "declared and not enforced" column on the same day it was declared. The dated
record is `history/estado-2026-10-18T2036Z.md`.

| What | Before | After |
| --- | --- | --- |
| `cargo test --workspace` | 386 | **395**: nine tests of the (c′) gate in `crates/stochology-book/src/costura.rs` |
| `labs --bridge` | two sections: the pilot re-derived and the fair test of (c) | **three**: the third reads `empirical/p16-ponte/registro_c_linha.tsv` and prints **(c′) not supported, box closed**, with a negative and a positive proof on synthetic records |
| seals | 30 files, 88 verified targets | 30 files, **90** verified targets: the (c′) record and this record |
| `empirical/p16-ponte/README.md` | reproduction command `--ponte`, refused since the English migration | `--bridge`, by a dated rectification |

What did not change: `registro.tsv`, the verdict on thesis (c), and every number of the book.

## The rectifications of 2026-10-18 (second outside review)

Later the same day the owner asked for a critical review of the textbook and then for its corrections to be applied
end to end. The reviewer is again a language model (Claude, Anthropic), of the same family as the first, so this is
not the independent audit that item 17 of `docs/DECLARED_PROPOSALS.md` now declares. The dated record, with the hashes
before and after, is `history/estado-2026-10-18T2227Z.md`. Unlike the earlier records, this one **changes numbers of
the book**: every change is listed with both values in Appendix E, rectifications of 2026-10-18.

| What | Before | After |
| --- | --- | --- |
| `cargo test --workspace` | 395 | **406**: eleven tests that fail on the old code (Kelly as the argmax of growth, drag as $L^2\sigma^2/2$, EVSI by hand, Gumbel level, FNR against FPR, Chouldechova identity, exact insurer, ICP rejects the empty set and the child, ICP under an unstable mechanism, RES of a constant forecaster, a board verdict that no seed can change), plus two that pin the type III lock |
| laboratories | Chapters 8, 9, 10, 11 and 12 printed values from wrong accounts | corrected accounts; every cited block still equals its instrument (31 of 31) |
| R axis | resolution loss 46.24 and 5.97 min at utilisation 1 | 22.99 and 2.50 min at a declared utilisation of 0.8 |
| review board A1–A8 | a safeguard without anchor drew 5 or 6, so the seed decided the verdict | 4 or 5, below the floor under every seed; the module states that the board runs no attack |
| type III lock | one message for two defects | three messages; a test pins its declared limit (it reads no meaning) |
| `scripts/book/check_book.py` | parts, chapters, fences | also refuses 20 retired values in the chapters |
| assembled book, line count | 5 619 | 5 831 (with 76 919 words) |
| live declared proposals | 6 | **8**: item 16, the book as its own source, and item 17, an audit from outside (`docs/OUTSIDE_AUDIT.md`) |

What did not change: the data, the seals of the empirical points, the `gates` merkle root, and every verdict of §3 that
does not rest on a corrected laboratory. The conclusions of Chapters 15 and 21 that no published number had fallen by an
error of arithmetic are withdrawn in the text, with both statements kept in Appendix E.

## The text rectifications (third outside review, 2026-10-18)

A third reading, made with an AI assistant like the two before it, read the assembled book rather than the code and
found statements that no instrument produces, so no gate could have caught them: what a fixed seed proves, Vulcan as
a degenerative repair, Stratonovich as the reading without drag, the 1%-to-99% floor stated as a prohibition on every
forecast, the Erlang C node typed as Knightian, approval parity inside Chouldechova's identity, and five glossary
definitions. Each is corrected in the chapter, kept with both wordings in the second rectification table of Appendix E,
and its old wording enters the guard of `scripts/book/check_book.py`. The book now also states, in Chapters 15 and 21,
in Appendix E and in `docs/OUTSIDE_AUDIT.md`, that the outside readings of 2026-10-18 were made with an AI assistant
and do not close item 17. The dated record is `history/estado-2026-10-19T0052Z.md`.

| What | Before | After |
| --- | --- | --- |
| laboratories, data, seals of the empirical points, `gates` merkle root, tests | unchanged | unchanged: no Rust file moved |
| `scripts/book/check_book.py` | refuses 20 retired values | refuses 27 |
| assembled book, line count | 5 831 (with 76 919 words) | 5 925 (with 78 717 words) |
| Appendix E | one rectification table | two; a reading note; the declared limit of the outside readings |

## The panel as bits and the first reimplementation (2026-10-19)

The safeguard panel A1–A8 (called the adversarial panel or review board until this day) stopped printing a severity
from 0 to 10 drawn by a seed. It prints the two bits it always decided on, the safeguard and its anchor, and their
AND; no seed enters the verdict. The `gates` merkle roots do not move (`9716e28a...` and `802e84b6...`), because the
ledger records the verdict, which did not change; the report hashes move with the text of G6 and G7
(`8279ae1c...` → `bb7edf13...` for `--horizon all`, `88b7214b...` → `93565e65...` for the negative proof).

A reimplementation of the decision laboratories of Chapters 10 and 11 from the text alone, in Python, by an AI
assistant (`scripts/reimplementation/decision_labs.py`, now in `scripts/verify.sh`), recomputed 61 quantities and
found two wrong values in Chapter 10, both corrected with tests: the twice-Kelly share above the start, decided by
the rounding of exact ties, and the share of a Pareto mean above the median maximum, computed with the maximum at
$n$. The numpy citation of Appendix C is checked (NumPy 2.5.0 release notes, gh-31238). The dated record is
`history/estado-2026-10-19T0340Z.md`.

| What | Before | After |
| --- | --- | --- |
| `cargo test --workspace` | 408 (406 declared, stale) | **409**: one panel test retired with the seed it tested, two tests of Chapter 10 added |
| panel A1–A8 | a severity per agent, drawn by a seed; floor 6 | two bits per agent and their AND; G6 "96 of 96 checks declared and anchored" |
| laboratories | Chapters 7, 10 and 13 | bits in 7 and 13; ties and the median maximum in 10; every cited block equals its instrument |
| `scripts/verify.sh` | no reimplementation | runs `decision_labs.py`: 61 of 61 agree |
| `scripts/book/check_book.py` | refuses 27 retired values | refuses 32 |
| assembled book, line count | 5 925 (with 78 717 words) | 5 944 (with 79 179 words) |

## The reimplementation of the chapters (2026-10-19, later the same day)

The reimplementation from the text was extended from Chapters 10 and 11 to twenty chapters
(`scripts/reimplementation/`, run by `run.py` in `scripts/verify.sh`; `decision_labs.py` became `ch10_ch11.py`).
421 quantities: 402 agree, 19 are open because the text names a procedure without its recipe, none diverges after the
corrections. It found six wrong values in the instruments and one gap in the text, all listed with both values in
Appendix E (rectifications of the reimplementation of the chapters): the R horizon's day (a fixed count of arrivals,
the Lindley recursion off by one client, 200 days), the mean of W over a uniform demand, the EVPI of Chapter 12, the
reinforcement of the health post (one nurse at twice the pace where the text says two nurses), the bank's halves
null (the sd of absolute differences), a rounding in Chapter 17, and the unspecified 1-NN of Chapter 19. The day of
predictions 3 and 5 keeps its old form, declared, because the predictions were declared and measured with it. The
dated record is `history/estado-2026-10-19T0752Z.md`.

| What | Before | After |
| --- | --- | --- |
| reimplementation | 61 quantities, Chapters 10 and 11 | 421 quantities, 20 chapters: 402 agree, 19 open |
| `cargo test --workspace` | 409 | **413**: four tests that fail on the old code (signed halves null, closed-form Jensen mean, two nurses on one line, EVPI from the payoff table) |
| R horizon | 5 or 11: loss 22.99 min; 7 or 9: 2.50; 200 days | 25.54 and 2.77; 4,000 days; the decision of System 2 (refine to the day) unchanged |
| `scripts/book/check_book.py` | refuses 32 retired values | refuses 37 |
| assembled book, line count | 5 944 (with 79 179 words) | 5 969 (with 79 884 words) |

## The recipes of the open quantities (2026-10-19, later the same day)

The recipes of 16 of the 19 open quantities were written into the chapters, beside the run of each laboratory, and the
reimplementation redoes them from those paragraphs: 462 quantities, 459 agree, none diverges, 3 open (Chapter 17's
Kelly interval, skill in the vault and false negative rate of Hispanics, cited from the research record). The writing
found one wrong value (the exclusion freedom of the garden of paths compared mixed groups: 21.5% and 47.0% became
20.3% and 42.3%, the Mayo severity of the garden 0.53 became 0.58), one wrong statement (Chapter 17 read the observed
+9.0 as the result of the inert treatment), and a known weakness of the Merkle root, declared with a test. The dated
record is `history/estado-2026-10-19T0845Z.md`.

| What | Before | After |
| --- | --- | --- |
| reimplementation | 421 quantities: 402 agree, 19 open | 462 quantities: 459 agree, 3 open |
| `cargo test --workspace` | 413 | **415**: the bounds of the garden of paths, the declared limit of the Merkle root |
| `scripts/book/check_book.py` | refuses 37 retired values | refuses 41 |
| assembled book, line count | 5 969 (with 79 884 words) | 6 075 (with 81 961 words) |

## A critical reading of the text (2026-10-19, later the same day)

A critical reading of the assembled book compared each chapter with its own output blocks, with the rest of the book
and with what the reader can run. It changed text only: no instrument, no test and no gate moved. It found two
tables that kept the figures of a retired generator (Chapters 2 and 5), an exercise left behind by a retyping
(Chapter 12), two sentences that misread a quantity (Chapters 17 and 18), 37 thousands separators written as decimal
points (Chapters 17 to 20), headlines that said more than the text concedes (Chapters 19 and 21), a change of ruler
left without comment (Chapter 20), a negative loss left without its cause (Chapter 21), a preface whose rule did not
list its exceptions, and 26 works cited and absent from the bibliography. Every change is listed with both values in
Appendix E, rectifications of a critical reading, beside a new table of the numbers cited without an instrument. The
dated record is `history/estado-2026-10-19T1235Z.md`.

| What | Before | After |
| --- | --- | --- |
| reimplementation, `cargo test --workspace`, gates | unchanged | unchanged |
| bibliography | 51 lines (50 checked, 1 to check) | 77 lines (50 checked, 27 to check) |
| assembled book, line count | 6 075 (with 81 961 words) | 6 188 (with 84 273 words) |

## Names, title, references and the order of the chapters (2026-10-19, later the same day)

The follow-up of the critical reading, by the owner's decisions of the same day. The chapters were reordered so that
the book reads as two: the method (Parts I to IV, Chapters 1 to 14, the three real cases now Chapters 12 to 14) and the
auditable machine (Part V, Chapters 15 to 21). Every chapter number in the book, in the `labs` flags, in the modules of
`stochology-book`, in the verifiers, in the reimplementation and in the living documents follows the new numbering;
the dated records, the declarations and the sealed empirical points keep the numbering of their date, and Appendix E
carries the table that translates. The R-axis table (`book/r-axis.tsv`) is sealed by `empirical/p18-eixo-r` and stays
as frozen; the instrument translates its numbers when it prints. The five lenses take the names of their fields, with
the coined words kept as the labels the outputs print. The title becomes "Stochology: Judgement and Decision under
Uncertainty", a working text until the outside audit. 24 of the 26 references of D.5 were checked against Crossref,
arXiv and PMLR. The dated record is `history/estado-2026-10-19T1307Z.md`.

| What | Before | After |
| --- | --- | --- |
| order of the chapters | 1 to 11, machine 12 to 15, cases 16 to 21 | 1 to 11, cases 12 to 14, machine 15 to 21 |
| bibliography | 77 lines (50 checked, 27 to check) | 77 lines (74 checked, 3 to check) |
| cited blocks, reimplementation | 31 blocks, 462 quantities | unchanged, under the new numbers |
| assembled book, line count | 6 188 (with 84 273 words) | 6 222 (with 84 921 words) |

## The remaining points (2026-10-19, later the same day)

A last pass on the points the critical reading had left open, by the owner's decision of the same day. Five worked
examples of the course gained a recipe and a check (`scripts/reimplementation/recipes.py`, recorded in
`docs/EMPIRICAL_REGISTRY.md`); four of them corrected the printed value (0.85% to 0.87%, 0.084 to 0.085, 16% to 15%,
12 and 112 rounds to 23 and 136). The six that could not be reconstructed lost their numbers and kept their argument.
The paths of the research record left Chapters 13 and 14 for Appendix E; "textbook" left the living documents (the
file name stays, because the build, the seals and the site read it); Chapter 19 was retitled; the dictionary was put
in alphabetical order; the last three references were checked, so all 77 lines of the bibliography are. The real
datasets of Chapters 19 and 20 were not brought into `data/`: what they need is not the data but an instrument that
redoes the retired analyses in Rust. The dated record is `history/estado-2026-10-19T1335Z.md`.

| What | Before | After |
| --- | --- | --- |
| reimplementation | 462 quantities: 459 agree, 3 open | 475 quantities: 472 agree, 3 open |
| course examples without a recipe | 11 | none with a number in the text: 5 recipes written, 6 cut |
| bibliography | 77 lines (74 checked, 3 to check) | 77 lines, all checked (Platt 1999 against a secondary source) |
| `scripts/book/check_book.py` | refuses 41 retired values | refuses 49 |
| assembled book, line count | 6 222 (with 84 921 words) | 6 265 (with 85 797 words) |

## The real streams, rebuilt (2026-10-19, later the same day)

Six streams of H3 rebuilt in Rust on public data, by the owner's decision of the same day. Five datasets entered
`data/` (`data/SOURCES.md`, `data/SHA256SUMS`): the New York flights of 2013 and 2023, the electricity demand of
Victoria, the Atlantic storms and the Toronto arrests of 1997 to 2002. A new instrument, `labs --streams`
(`crates/stochology-book/src/streams.rs`, six tests), rebuilds energy, bank, market, the two flights vaults, the
cyclones and Toronto under a specification written in Chapter 20 before the run, with the clock, the canary and the
fairness audit; Chapter 20 gained the recipe, the block and its reading. Where the specification is the same, the
rebuild lands on the record; the value at risk and the 2023 correction disagree, and Appendix E keeps both. A
reimplementation from the text (`scripts/reimplementation/ch20_streams.py`) redoes Toronto, the market and the energy
interval and agrees with every number. Not rebuilt, and why, in `data/SOURCES.md`: Toronto 2020–2021 and Rhode Island
(portals not reachable), the chambers, the cardiac trial and the yeast of Chapter 19, and STAR with school identifiers.
The dated record is `history/estado-2026-10-19T1429Z.md`.

| What | Before | After |
| --- | --- | --- |
| datasets in `data/` | 5 | 10 |
| instruments in `labs` | 19 | 20 (`--streams`) |
| `cargo test --workspace` | 415 | **421** |
| cited blocks | 31 | 32 |
| reimplementation | 475 quantities: 472 agree, 3 open | 487 quantities: 484 agree, 3 open |
| assembled book, line count | 6 265 (with 85 797 words) | 6 365 (with 87 427 words) |

## The chapters teach with the instrument (2026-10-19, later the same day)

The rebuild of the streams left the book saying two things: Chapter 20 taught the record's numbers in Step 1 and the
instrument's in Step 3, and Chapter 8 kept the record's six vaults. By the rule of the Preface (the chapters teach
with what the reader can run; Appendix E keeps both values), Chapters 8 and 20 now teach with `labs --streams`, which
also prints the null q95 of the ECE with the real forecasts of each vault; the record's numbers stay in Appendix E,
and what nothing rebuilds (Toronto 2020–2021, Rhode Island, the replay count, the Goodhart index, the label drift and
the inverted audit) is marked as the record of the research. Three readings changed with the numbers: the first
failures are Toronto and the market, not the flights of 2013; the 2023 correction is decisive; and in 2013 the
correction fixes the bias and fails on the ECE. The dated record is `history/estado-2026-10-19T1524Z.md`.

| What | Before | After |
| --- | --- | --- |
| `cargo test --workspace` | 421 | **422** |
| `scripts/book/check_book.py` | refuses 49 retired values | refuses 52 |
| assembled book, line count | 6 365 (with 87 427 words) | 6 371 (with 87 807 words) |


## The desk work before the public (2026-10-19, later the same day)

By the owner's request to review the project from the start and name what still stands between the discipline and
its public, and then to start Phase A, the part that is desk work. The living documents (`docs/PLAN.md`,
`docs/EMPIRICAL_REGISTRY.md`, `docs/DECLARED_PROPOSALS.md`, `docs/OUTSIDE_AUDIT.md`, `site/home.md`, `README.md`) now
say what the book says, with the instrument's numbers and the renumbered chapters. The repository gained
`CITATION.cff` (cite the commit; no version), `CONTRIBUTING.md`, an errata template and the licences of the ten
datasets in `data/SOURCES.md`; the README says who wrote what and what this is not yet, and the Preface says the same.
Item 15 closed: the 38 recorded tests of the transfer gate were read under an e-process declared before the run
(`empirical/p22-parada/DECLARACAO_2.md`, `labs --eprocess`); three areas cross at any stopping time, genetics does
not, and education stays in by the mean of its earlier rounds, which Chapter 19 reports beside the clustering
correction that removes it. Chapter 21 and Appendix D.6 name the neighbours of the discipline and what is borrowed
from each. The 126 exercises have an answer key, `book/answers.md`, on the site and outside the assembled book. The
paper was aligned with the book. The dated record is `history/estado-2026-10-19T1635Z.md`. What stays open is for the
owner and for people outside the programme, and the record lists it.

| What | Before | After |
| --- | --- | --- |
| instruments in `labs` | 20 | 21 (`--eprocess`) |
| `cargo test --workspace` | 422 | **427** |
| cited blocks | 32 | 33 |
| declared proposals open | eight | seven (item 15 closed) |
| assembled book, line count | 6 371 (with 87 807 words) | 6 493 (with 89 505 words) |

## The licence of the text (2026-10-19, later the same day)

By the owner's decision, the text (the book, the paper, the site and the prose documents) is under CC BY 4.0
(`LICENSE-TEXT`); the code stays under MIT and the datasets under the licences of their sources. No number and no
instrument changed. The dated record is `history/estado-2026-10-19T1655Z.md`.

## The wave engine against data (2026-10-19, later the same day)

By the owner's decision, the wave engine was refined (`channel.rs`, `cycle.rs`) and two of its layers were tested
on catalogued data under a declaration sealed and committed before the first run (`empirical/p23-onda/`,
`labs --wave`). The arrow of its spike bus was not supported in Victoria, and lost to the lagged cross-correlation;
the conflict of its phasors never fired on the New York flights, so it was not testable under its encoding. Both are
reported as results, with the deviation of the weeks read. The dated record is `history/estado-2026-10-19T1744Z.md`.

| What | Before | After |
| --- | --- | --- |
| instruments in `labs` | 21 | 22 (`--wave`) |
| `cargo test --workspace` | 427 | **438** |
| seals | 30 | 31 |
| assembled book, line count | 6 493 (with 89 505 words) | 6 494 (with 89 541 words) |

## Declaration 2 of point 23, K = 2 (2026-10-19, later the same day)

By the owner's decision, the two refinements the first report proposed were declared, sealed with their module and
committed before their single run (`DECLARACAO_2.md`, `labs --wave2`). The arrow of the spike bus is still not
supported; the conflict of the phasors, now able to fire, points away from where deliberation pays, and is refuted
by its own rules in both years. The dated record is `history/estado-2026-10-19T1803Z.md`.

| What | Before | After |
| --- | --- | --- |
| instruments in `labs` | 22 | 23 (`--wave2`) |
| `cargo test --workspace` | 438 | **440** |
| assembled book, line count | 6 494 (with 89 541 words) | 6 495 (with 89 580 words) |

## Point 24, resonance, not conflict (2026-10-19, later the same day)

By the owner's decision, the claim drawn from declaration 2 of point 23 (deliberation pays where the sources are in
phase) was declared, sealed with its instrument and three fresh vaults, and run once (`empirical/p24-ressonancia/`,
`labs --resonance`). It is not supported: negative in two vaults, with an interval below zero in one, against three
and two required. The dated record is `history/estado-2026-10-19T1840Z.md`.

| What | Before | After |
| --- | --- | --- |
| datasets in `data/` | 10 | 13 |
| instruments in `labs` | 23 | 24 (`--resonance`) |
| `cargo test --workspace` | 440 | **442** |
| seals | 31 | 32 |
| assembled book, line count | 6 495 (with 89 580 words) | 6 496 (with 89 607 words) |

## The wave engine removed (2026-10-19, later the same day)

By the owner's decision, the wave engine left the tree with its three instruments after its sealed tests on data
(points 23 and 24) did not support its claims. The records, the outputs and the sealed data stay; the removed sources
stay in the seals as lines declared out of the tree, and `docs/declarations/REMOVED_WAVE.md` gives the recovery
commit. The dated record is `history/estado-2026-10-19T1859Z.md`.

| What | Before | After |
| --- | --- | --- |
| crates | 9 | 8 |
| instruments in `labs` | 24 | 21 |
| `cargo test --workspace` | 442 | **395** |
| assembled book, line count | 6 496 (with 89 607 words) | 6 493 (with 89 505 words) |

## Uncertainty propagated to the decision, and the seed audited (2026-10-19, later the same day)

By the owner's request, what a lecture on the propagation of uncertainty teaches entered the book with the book's own
instruments: `labs --mc-error` measures how far each simulated number moves with the seed (no reading moves; two
published values sit at the edge of their spread, and the chapters say so), and `labs --propagate` gives three answers
to the same question on the health post, with the uncertainty budget and the rule on when the linear answer may decide
(Chapter 5). Chapters 5, 6, 11 and 14 gained the loss behind a summary, the variance of a sum, the expected cost by
term with the conditional value at risk, and the instability of the higher moments. The dated record is
`history/estado-2026-10-19T1925Z.md`.

| What | Before | After |
| --- | --- | --- |
| instruments in `labs` | 21 | 23 (`--mc-error`, `--propagate`) |
| `cargo test --workspace` | 395 | **401** |
| cited blocks | 33 | 34 |
| reimplementation | 487 quantities: 484 agree, 3 open | 513 quantities: 510 agree, 3 open |
| assembled book, line count | 6 493 (with 89 505 words) | 6 636 (with 91 896 words) |

## The card contract 1.3 (2026-10-19, later the same day)

The card may now declare the uncertainty budget and the dominant event of the expected cost; both are validated when
present and not demanded by any gate, and the migration from 1.2 brings them in absent. The chain frozen by point 17
changes by its declaration 2, with the frozen hash kept as a superseded line of its seal. The dated record is
`history/estado-2026-10-19T1948Z.md`.

| What | Before | After |
| --- | --- | --- |
| card contract | 1.2 | 1.3 |
| `cargo test --workspace` | 401 | **407** |
| assembled book, line count | 6 636 (with 91 896 words) | 6 654 (with 92 188 words) |

## The research on models, and item 16 (2026-10-19, at the end of the day)

The previous book no longer certifies its own numbers (item 16): instrument outputs, code and records are sources, and
the 29 numbers no source carries have a checked row in `book/provenance.tsv`; the search found one stale passage in
Appendix E. Item 18 came from the research on models of the world, of agents and of modelling and closed the same
day: four laboratories, each declared and sealed before its first run (points 25 to 28; `--passk`, `--multiverse`,
`--stacking`, `--discrepancy`), with every rule holding; the card contract 1.4 (declaration 3 of point 17); the
authorship of every check by family; the safeguard panel held against the fourteen failure modes of teams of agents
(`docs/MAST_AUDIT.md`); and a register of the 27 modules that draw random numbers, with a test. Point 28 needed a
second declaration after its first run (K = 2). Item 19 holds what the audits left open. The dated record is
`history/estado-2026-10-19T2150Z.md`.

| What | Before | After |
| --- | --- | --- |
| instruments in `labs` | 23 | 27 (`--passk`, `--multiverse`, `--stacking`, `--discrepancy`) |
| card contract | 1.3 | 1.4 |
| `cargo test --workspace` | 407 | **426** |
| cited blocks | 34 | 39 |
| reimplementation | 513 quantities: 510 agree, 3 open | 573 quantities: 570 agree, 3 open; all 573 checked by the family that wrote the instrument |
| numbers whose only source was the previous book | about 140 measured on 2026-10-18; 314 measured on 2026-10-19 without the old clause | 0; 29 in the provenance ledger |
| assembled book, line count | 6 654 (with 92 188 words) | 7 004 (with 98 402 words) |

## Item 19: contract 1.5 and the seed over whole laboratories (2026-10-19, later)

The three failures of teams of agents that nothing in the program saw get a place in the card (contract 1.5,
declaration 4 of point 17), checked when present: the authorship of each field, the findings used and not used, and the
decision rule recomputed from p against C/L. Every generator of the laboratories adds an audit offset to its declared
seed (none when unset, so the published outputs do not change), and `labs --seed-spread` reruns the thirteen
instruments that the register listed as not audited under twenty other seeds: three of 312 lines carry a verdict that
moves with the draw, and Chapters 9 and 15 say so. The dated record is `history/estado-2026-10-19T2335Z.md`.

| What | Before | After |
| --- | --- | --- |
| card contract | 1.4 | 1.5 |
| instruments in `labs` | 27 | 28 (`--seed-spread`, recorded in `docs/SEED_SPREAD.md`) |
| `cargo test --workspace` | 426 | **433** |
| modules that draw random numbers without an audit of their seed | 14 | 0 |
| failure modes of teams of agents that nothing sees | 3 | 0 (moved to "partly") |
| assembled book, line count | 7 004 (with 98 402 words) | 7 050 (with 99 294 words) |

## The repository renamed (2026-10-20)

The repository was renamed from `0b3st4f3r4/estocologia` to `0b3st4f3r4/stochology`. The URLs in `CITATION.cff`, the
README and the paper follow it; the C bridge of the WebAssembly crate exports `stochology_*` and keeps the historical
`estocologia_*` symbols as aliases, so nothing that linked before breaks (one test); the tool package is
`stochology-tools`. No number of the book, no seal of an empirical point and no verdict changed. The site is served
from its own domain and did not move. Not run here: the `wasm32-unknown-unknown` build, whose target could not be
installed in this session (the CI does not build it either); the bridge compiles and its tests pass on the host.

| What | Before | After |
| --- | --- | --- |
| repository | `0b3st4f3r4/estocologia` | `0b3st4f3r4/stochology` |
| paths of the author's machine in the tree | 10 lines in 6 files | none: replaced by paths relative to the repository root or to the home directory (`~`); the earlier values remain in the git history |
| `cargo test --workspace` | 433 | **434** |

## The analytic oracles (2026-10-20)

A review made with AI assistants proposed oracles at limit cases for the formulas of the decision laboratories. Its own
reference for the M/M/1 wait omitted the division by $\mu$, so the oracles enter with the mutants of Appendix E: each
recorded error rewritten with the signature of its instrument, and an oracle counts only if a mutant fails it.
`labs --oracles` (`crates/stochology-book/src/oracles.rs`, five tests) runs 18 identities: all pass on the instruments
and 16 of 16 mutants die. It found one divergence, declared and not changed: Chapter 10 takes $x_{(k)}$ as the
threshold of Hill and Chapters 14 and 15 take $x_{(k+1)}$, a bias of $k/(k-1)$. No published number changed. The dated
record is `history/estado-2026-10-20T1518Z.md`.

| What | Before | After |
| --- | --- | --- |
| instruments in `labs` | 28 | 29 (`--oracles`, run by `scripts/verify.sh`) |
| identities checked at limit cases | 0 as a suite | 18 |
| recorded errors that a suite would have caught | not measured | 16 of 16 mutants killed |
| `cargo test --workspace` | 434 | **439** |
| assembled book, line count | 7 050 (with 99 294 words) | 7 066 (with 99 675 words) |

## The threshold of Hill in Chapter 10 (2026-10-20, later)

By the owner's go-ahead, the divergence that the oracles found is corrected: Chapter 10 now computes Hill over the
threshold $x_{(k+1)}$, as Chapter 14, the horizon H1 and the tail lens do, and its text writes the same definition. Four
printed estimates move in the third decimal (1.083, 1.063, 1.100 and 1.101 become 1.080, 1.059, 1.099 and 1.100); the
value at $k = 20,000$ and the reading do not change. The old threshold enters `labs --oracles` as a mutant, killed by a
new oracle that asks one definition of Hill across the chapters, and a test fixes that it reproduces the value the
chapter printed before. The dated record is `history/estado-2026-10-20T1828Z.md`.

| What | Before | After |
| --- | --- | --- |
| definitions of Hill in the book | 2 ($x_{(k)}$ in Chapter 10, $x_{(k+1)}$ elsewhere) | 1 |
| oracles / mutants killed | 18 / 16 of 16 | 19 / 17 of 17 |
| `cargo test --workspace` | 439 | 439 (one test replaced) |
| assembled book, line count | 7 066 (with 99 675 words) | 7 066 (with 99 741 words) |

## The oracles of Chapter 5, the calibration of Chapter 10 and the separated root (2026-10-20, later)

`labs --oracles` grows to 23 identities and 21 mutants, all killed, and a mutant now has to reproduce the value it
stands for. The formulas of Chapter 5 (the mean wait over two demands and over a uniform demand, the curvature) and
the calibration of the synthetic market of Chapter 10 move into named functions, with outputs unchanged byte for byte.
Applied to the calibration, the rule found that the cause Appendix E recorded (the factor of 252 dropped) does not
produce the earlier sigma of 0.01518, which solves the drag equation with 135 sessions; Appendix E now says so. The
audit module gains a Merkle root with domain separation, checked against the definition of RFC 6962; the gates still
publish the legacy root. The dated record is `history/estado-2026-10-20T1907Z.md`.

| What | Before | After |
| --- | --- | --- |
| oracles / mutants killed | 19 / 17 of 17 | 23 / 21 of 21 |
| Merkle roots in the audit module | 1 (legacy, odd level repeats its last hash) | 2 (legacy, published; separated, RFC 6962) |
| `cargo test --workspace` | 439 | **444** |
| assembled book, line count | 7 066 (with 99 741 words) | 7 080 (with 100 122 words) |

## The root with domain separation in force (2026-10-20, later)

By the owner's decision, `audit::merkle_root` and `Ledger::merkle_root` compute the root with domain separation
(RFC 6962), and the earlier rule stays as `merkle_root_legacy`. The gates print both roots and G1 checks that both
repeat; the verdicts do not change. The gates root for the horizon `todos` moves from `9716e28a87bec9cf...` to
`0a9636fac5eb1c5e...`, the negative proof from `802e84b6...` to `8dfb949d...`, and the two roots of Chapter 18 from
`92f2ab76...` and `17679ce1...` to `e81457d5...` and `ed9aa4bb...`; Chapter 18 writes the new recipe and the
reimplementation recomputes it from the text. The declared limit of the Merkle root (a list and its last element
repeated) is closed. The dated record is `history/estado-2026-10-20T1930Z.md`.

| What | Before | After |
| --- | --- | --- |
| root published by the gates, `todos` | `9716e28a87bec9cf...` | `0a9636fac5eb1c5e...` (legacy printed beside it) |
| declared limits of the Merkle root | 1 | 0 |
| `cargo test --workspace` | 444 | **444** |
| assembled book, line count | 7 080 (with 100 122 words) | 7 097 (with 100 405 words) |
