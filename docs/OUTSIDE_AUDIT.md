# An Audit From Outside

Declared on 2026-10-18 (item 17 of `docs/DECLARED_PROPOSALS.md`).

## Why this file exists

On 2026-10-18 a reading from outside the program found errors that every internal safeguard had passed: a Kelly
fraction computed with the wrong chance, a value of information that chose the best act inside each branch of a
test, a test of invariance that let the environment move the outcome, a market calibration that dropped a factor of
252, a review board whose verdict a seed could decide, and figures in the prose that no instrument produced any more.
Each of them was reproduced, bit for bit, by a fixed-seed instrument, and each passed the gates, because the gates
check form, presence and reproducibility, not correctness (Chapter 14 now says so in the table of gates). The full list,
with both values, is the rectification table of Appendix E.

The inward audit of Chapter 15 had concluded that no number fell by an error of arithmetic. That conclusion was
wrong, and the way it was wrong is the argument of this file: the authors of a claim are a weak auditor of it,
however honest they are. Chapter 15 already said this about signed opinions; it holds for the mathematics too.

The readings of 2026-10-18 themselves have a declared limit. They were made with an AI assistant: the first
rectification commit is authored by "Claude", and a second review of the same kind added the second table of
Appendix E (definitions, attributions and readings of formulas that no instrument produces). They stand outside the
authors' audit, but not outside the tools the authors used, and no person outside the program has checked them.
They enter the public errata below as its first entries; they do not close item 17.

## The protocol

Three activities, in increasing order of cost. None of them is run by the program on itself.

### 1. A public errata

- Anyone may report an error by opening an issue with the label `errata`, citing the chapter, the line of the
  assembled book, the value printed, and the value they believe is right, with the account that gives it.
- A report is **confirmed** only when a test that fails on the old code and passes on the corrected code enters the
  tree. Rows with no executable account (a date, an attribution) are confirmed by a source.
- Every confirmed report enters the rectification table of Appendix E with both values and the name of who reported
  it, if they wish, and its retired value enters the guard in `scripts/book/check_book.py`.

### 2. A blind reimplementation

- A reader who has not seen the Rust code receives only the specification of one laboratory: the generating
  process, the seed policy, the quantities to print. They implement it in a language of their choice.
- The two outputs are compared number by number. A difference beyond the declared tolerance opens an `errata`
  issue. Agreement is weak evidence where the specification leaves choices open (Chapter 21 notes this about an
  earlier reimplementation), so the specification records which choices it fixes.
- Priority: the laboratories whose numbers carry a decision (Chapters 10, 11, 16 and 17), then the methods
  (Chapters 8, 9 and 19).

### 3. A red team for the claims

- People outside the program choose hard cases for the bridge thesis (item 9) and for the coding of layers
  (item 14), as Chapter 2 already requires.
- The red team also receives the review board as it is: a checklist of declared safeguards with a reproducible
  score. Its task is to find a card that passes the board and is wrong.

## What is not claimed

This file does not make the program audited. It records how an audit from outside would run and what it would need:
people who are not the authors. Until they exist, item 17 stays open.
