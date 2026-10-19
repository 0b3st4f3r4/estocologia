# Contributing

Three ways in, in increasing order of cost. None requires permission; all go through the verification in `AGENTS.md`.

## 1. Report an error

Open an issue with the **errata** template. The protocol is `docs/OUTSIDE_AUDIT.md`, activity 1: a report is
confirmed by a test that fails on the old code and passes on the corrected one, or by a source when there is no
executable account, and it enters Appendix E of the book with both values. Every number in the book comes from a
command you can run (Appendix C), and the ones that do not are listed in Appendix E, "Numbers cited without an
instrument".

## 2. Reimplement a laboratory without reading the Rust

Pick a chapter, read only its text (the recipe paragraph beside each laboratory states the choices), and recompute
its printed quantities in a language of your choice. `scripts/reimplementation/` holds the ones done so far, in
Python with the standard library only; a new module registers its checks through `common.py` and is added to
`run.py`. A divergence beyond the tolerance is an errata; agreement is weak evidence where the text leaves a choice
open, so say which choices you had to make.

## 3. Change the tree

- Read `AGENTS.md` first. It is the contract: the four prohibitions, the invariants, and the mandatory verification.
- Run `scripts/verify.sh` before opening a pull request. CI runs the same file.
- No number enters the book without an instrument that prints it; no path, repository reference or metacommentary
  enters a chapter (they belong in Appendix E).
- A correction keeps both values: the earlier one goes to Appendix E with the cause, and its retired value to the
  guard in `scripts/book/check_book.py`.
- The project keeps history, not versions: no tags, no "v2" files. A change of state is a dated record in `history/`.
- Rust 1.97.1 is pinned in `rust-toolchain.toml`; Python tooling is standard library only (Python 3.13).

## Licence of what you contribute

What goes in comes out under the same terms: a contribution to the text (the book, the paper, the documents) is
offered under CC BY 4.0 (`LICENSE-TEXT`), and a contribution to the code under MIT (`LICENSE`). A dataset enters
only with its licence declared in `data/SOURCES.md`.

## What outside participation is waiting for

Four items depend on people who are not the authors, and no code closes them (`docs/DECLARED_PROPOSALS.md`): the
listening sessions with the groups affected by the seven decisions on people, the blind test of the bridge thesis
with cases and judges chosen from outside, independent coders for the layers, and an audit from outside. If you can be
one of those people, open an issue saying which.
