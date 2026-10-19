#!/usr/bin/env python3
"""Chapter 16, "Reliability, measured", recomputed from the text alone.

The recipe is the paragraph of Chapter 16: 200 tasks whose chances of success are drawn from Beta(0.3, 0.3), 8 runs of
each, pass^k per task as C(c, k) / C(8, k) from its c successes, averaged over the tasks, and the whole experiment
repeated 2,000 times for a 95% band. The exact column is the Beta moment, the product of (a + j) / (a + b + j) for j
below k. This module did not read crates/stochology-book/src/passk.rs. Its draws use Python's own generator, so the
bands agree with the book's within their sampling error, not to the digit.

Declared limit: written by an AI assistant, like the instrument it checks; a cross-check, not the outside audit of
item 17. Standard library only.
"""

from __future__ import annotations

import math
import random

from common import check

A = B = 0.3
TASKS, RUNS, REPLICATES = 200, 8, 2000

BOOK_EXACT = {1: 0.5000, 2: 0.4062, 3: 0.3594, 4: 0.3294, 5: 0.3079, 6: 0.2914, 7: 0.2782, 8: 0.2672}
BOOK_BAND = {1: (0.4419, 0.5587), 8: (0.2050, 0.3300)}

for k, book in BOOK_EXACT.items():
    exact = math.prod((A + j) / (A + B + j) for j in range(k))
    check("16", f"passk: exact E[p^{k}] under Beta(0.3, 0.3)", book, exact, 0.00005)

rng = random.Random(20261019)
est = {k: [] for k in (1, 8)}
for _ in range(REPLICATES):
    counts = []
    for _ in range(TASKS):
        p = rng.betavariate(A, B)
        counts.append(sum(1 for _ in range(RUNS) if rng.random() < p))
    for k in est:
        est[k].append(sum(math.comb(c, k) for c in counts) / math.comb(RUNS, k) / TASKS)

for k, (lo, hi) in BOOK_BAND.items():
    v = sorted(est[k])
    ours_lo, ours_hi = v[round(0.025 * (REPLICATES - 1))], v[round(0.975 * (REPLICATES - 1))]
    # the 2.5% and 97.5% points of 2,000 replicates move by about a hundredth between generators
    check("16", f"passk: 95% band of pass^{k}, lower", lo, ours_lo, 0.015, "sampling")
    check("16", f"passk: 95% band of pass^{k}, upper", hi, ours_hi, 0.015, "sampling")
    mean = sum(v) / REPLICATES
    check("16", f"passk: mean of the estimator of pass^{k} against E[p^{k}]", BOOK_EXACT[k], mean, 0.004, "sampling")
