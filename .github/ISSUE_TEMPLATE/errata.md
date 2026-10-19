---
name: Errata
about: Report a number, formula, attribution or statement in the book or the code that you believe is wrong
title: "errata: "
labels: errata
---

<!-- The protocol is in docs/OUTSIDE_AUDIT.md, activity 1. A report is confirmed when a test that fails on the old code
and passes on the corrected code enters the tree, or by a source when there is no executable account. Confirmed reports
enter Appendix E with both values and, if you wish, your name. -->

**Where.** Chapter and line of `book/stochology-textbook.md` (or file and line of the code), at commit `…`.

**What it says.**

**What you believe it should say, and the account that gives it.** A formula, a computation, a source.

**How you checked.** Ran the laboratory (`cargo run --release --bin labs -- --chapter N`), the reimplementation
(`python3 -I scripts/reimplementation/run.py --all`), a recomputation by hand, a source.

**May we name you in Appendix E?** yes / no
