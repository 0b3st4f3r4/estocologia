# `book/` — the book in 21 chapters

This folder keeps the book being unfolded into **21 chapters and 5 parts**, with **three steps
per chapter** (high-school intuition, accessible formalization, laboratory in Rust). The plan of the
unfolding is §9 of `docs/PLAN.md`.

| File | Content |
| --- | --- |
| `00-opening.md` | preface, three-step ramp, map of the 21 chapters, reading tracks and conventions |
| `ch01.md` … `ch21.md` | one file per chapter |
| `appendices.md` | appendices A to D |
| — | the laboratories live directly in the crate: `crates/stochology-book/src/capNN.rs` |
| `scripts/book/build.py` | assembles the book and checks the ramp |

## Commands

```bash
python3 scripts/book/build.py --check      # checks chapters, three steps, exercises, summary AND the file on disk
python3 scripts/book/check_labs.py     # checks each chapter's output block against the instrument
python3 scripts/book/generate_results.py  # writes docs/RUST_RESULTS.md with the literal output of the instruments
python3 scripts/book/generate_results.py --check   # checks whether the registry is up to date
python3 scripts/book/build.py              # writes book/stochology-textbook.md when the 21 are ready
cargo run --release --bin labs -- --all    # runs every laboratory with a fixed seed
cargo test --workspace                       # tests of the book crates and of the architecture
```

## Real data in the laboratories

The laboratories of chapters 13 and 14 read the bases kept in the repository
(~data/djia_1885_2023.csv~, ~nsw_experimental.csv~, ~cps_observational.csv~,
~danish_fire.csv~). The default path is relative to the root of the repository; to point to another place,
use the environment variable:

```bash
ESTOCOLOGIA_DADOS=/caminho/para/dados cargo run --release --bin labs -- --chapter 16
```

## Rule of new files

A new working file (intermediate copy, one-off script, laboratory draft) is **deleted after
integration**. What stays in the repository is an artifact with an ongoing function (the generator, the two checkers, the crate)
or a record (chapters, declarations, manifests, reports).

## Rule of the numbers

Every number of Step 3 comes out of code that runs:

```bash
cargo run --release --bin labs -- --chapter 4
```

When a number of this book diverges from the number recorded in the previous book or in a README of the
research, the chapter carries a paragraph **Divergence declaration** with both values and the probable
reason. No old value is deleted.

## State

| Part | Chapters | State |
| --- | --- | --- |
| I. The art of aiming in the dark | 1 to 4 | written; laboratories in the crate and output blocks checked |
| II. The traps of the world and of data | 5 to 8 | written; laboratories in the crate and output blocks checked |
| III. Cause, time, and survival | 9 to 11 | written; laboratories in the crate and output blocks checked |
| IV. Building the auditable machine | 12 to 15 | written; laboratories in the crate and output blocks checked |
| V. The machine in the real world | 16 to 21 | written; laboratories in the crate and output blocks checked |
| Appendices A to D | — | written; checked against the earlier text (the old Appendix C, selected answers, became material embedded in Step 3) |

The edition in `book/stochology-textbook.md` is already the 21-chapter one, assembled from here, and `scripts/book/build.py --check`
demands two things: that the 21 chapters be on the ramp and that the file on disk be, character by
character, the text that `montar()` produces. The assembly on disk fell 353 lines behind `book/` between the
commits `a59803c` and `5393515` without any gate noticing — `scripts/book/build.py --check` compared only the ramp. The
text of 14 chapters lives in the history, in commit `1bfec905`.
