# An Audit From Outside

Declared on 2026-10-18 (item 17 of `docs/DECLARED_PROPOSALS.md`).

## Why this file exists

On 2026-10-18 a reading from outside the program found errors that every internal safeguard had passed: a Kelly
fraction computed with the wrong chance, a value of information that chose the best act inside each branch of a
test, a test of invariance that let the environment move the outcome, a market calibration that dropped a factor of
252, a review board whose verdict a seed could decide, and figures in the prose that no instrument produced any more.
Each of them was reproduced, bit for bit, by a fixed-seed instrument, and each passed the gates, because the gates
check form, presence and reproducibility, not correctness (Chapter 17 now says so in the table of gates). The full list,
with both values, is the rectification table of Appendix E.

The inward audit of Chapter 18 had concluded that no number fell by an error of arithmetic. That conclusion was
wrong, and the way it was wrong is the argument of this file: the authors of a claim are a weak auditor of it,
however honest they are. Chapter 18 already said this about signed opinions; it holds for the mathematics too.

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
- Priority: the laboratories whose numbers carry a decision (Chapters 10, 11, 12 and 13), then the methods
  (Chapters 8, 9 and 19).

### 3. A red team for the claims

- People outside the program choose hard cases for the bridge thesis (item 9) and for the coding of layers
  (item 14), as Chapter 2 already requires.
- The red team also receives the safeguard panel as it is: a checklist of declared and anchored safeguards that
  prints one pair of bits per agent. Its task is to find a card that passes the panel and is wrong.

## What is not claimed

None of the three activities has been run by people outside the program. Every entry below was made with an AI
assistant and stands as an entry in the errata, not as the audit. Agreement between a reimplementation and the
instrument is weak evidence wherever the reimplementer could have absorbed the author's choices from the text;
only a divergence found from the text alone is strong evidence.

## The first reimplementation (2026-10-19)

Activity 2 was run once, on the laboratories that carry decisions in Chapters 10 and 11, and its limits are declared
first: it was written by an AI assistant of the same kind as the readings of 2026-10-18, in Python, from the text of
the book alone, without reading `crates/stochology-book/src/ch10.rs` or `ch11.rs`. It recomputes 61 quantities,
exact accounts where the chapter states one and the exact value behind each simulation otherwise, and it now runs
in `scripts/verify.sh` (now `scripts/reimplementation/ch10_ch11.py`, run by `run.py`).

It found two values that the instruments printed faithfully and that were wrong, both in Chapter 10: the share of
twice-Kelly trajectories above the start, where floating-point rounding decided the exact ties (0.485 printed,
0.377 exact after 10 rounds), and the share of the true mean of a Pareto above the maximum of a sample of one
million (25.12% printed, 27.55% exact), where the laboratory put the maximum at $n$. Both are corrected, with tests,
in Appendix E. The other 59 quantities agreed. Agreement here is weak evidence where the reimplementer could have
absorbed the author's choices from the text; the two divergences are strong evidence, because the text alone was
enough to find them.

## The reimplementation of the chapters (2026-10-19)

The same day the reimplementation was extended to twenty chapters, in the priority of activity 2: the chapters that
carry decisions (12 and 13), then the methods (8, 9 and 19), then the rest. 421 quantities; 402 agree, 19 are open,
none diverges after the corrections. The Rust of a chapter was read only after a divergence, to locate it. The run
found six wrong values in the instruments (the R horizon's day, the mean of W over a uniform demand, the EVPI of
Chapter 15, the reinforcement of the health post, the bank's halves null, and a rounding) and one gap in the text
(the 1-NN of the saturation rule); Appendix E lists them with both values.

The 19 open quantities are the honest remainder: the text names a procedure (a propensity fit, the four forecasters of
Chapter 8, the gardens of Chapter 7, a posterior interval, the canary model) without the recipe a reader would need
to recompute it. Each is a place where the book is still its own source (item 16), and the remedy is to write the
recipe into the chapter, not to copy it from the code. The same limits hold as before: an AI assistant wrote this,
and agreement is weak evidence where the reimplementer could have absorbed the author's choices from the text.

## The recipes (2026-10-19)

The remedy was then applied: the recipes of 16 of the 19 open quantities were written into the chapters, beside the
run of each laboratory, and the reimplementation redoes them from those paragraphs. 462 quantities; 459 agree, none
diverges, 3 are open (the upper end of the Kelly interval, the skill of the model in the vault and the false negative
rate of Hispanics in Chapter 13, whose recipes are longer than a paragraph and stay cited from the research record at
commit `1bfec905`). Writing the recipes found one more wrong value (the exclusion freedom of the garden of paths
compared mixed groups: 21.5% and 47.0% became 20.3% and 42.3%), one statement wrong in the text (Chapter 13 read the
observed +9.0 as the result of the inert treatment, whose band is centred near +0.9), and a known weakness of the
Merkle root, now declared with a test. Where a quantity is a simulation, the tolerance is the spread of the book's own
instrument across seeds, measured and written beside the check, not a number chosen to make it pass.

This file does not make the program audited. It records how an audit from outside would run and what it would need:
people who are not the authors. Until they exist, item 17 stays open.
