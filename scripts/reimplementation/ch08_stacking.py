#!/usr/bin/env python3
"""Chapter 8, "Combining, measured", recomputed from the text alone.

The recipe is the paragraph of Chapter 8 and the header of its block: two candidates N(c, 1) with c ~ N(-2, 0.05^2)
and c ~ N(+2, 0.05^2); model averaging with equal prior probability per listed model; stacking as the weights that
maximise the log score of each training point predicted by the models fitted without it; the score of a mixture as the
mean log density of fresh points under the full-data predictives; 40 training and 2,000 test points per dataset. The
book runs 1,000 datasets per world; this module runs 300 with Python's own generator and its own optimiser (a fine grid
over the weight on the left model), so its means agree within their sampling error, not to the digit. It did not read
crates/stochology-book/src/stacking.rs.

Declared limit: written by an AI assistant, like the instrument it checks; a cross-check, not the outside audit of
item 17. Standard library only.
"""

from __future__ import annotations

import math
import random

from common import check

S0 = 0.05
CENTRES = (-2.0, 2.0)
N_TRAIN, N_TEST, DATASETS = 40, 2000, 300


def logpred(y: float, centre: float, total: float, n: int) -> float:
    prec = 1 / S0**2 + n
    v = 1 / prec
    m = v * (centre / S0**2 + total)
    var = 1 + v
    return -0.5 * (math.log(2 * math.pi * var) + (y - m) ** 2 / var)


def logmarg(ys: list[float], centre: float) -> float:
    total, out = 0.0, 0.0
    for i, y in enumerate(ys):
        out += logpred(y, centre, total, i)
        total += y
    return out


def mix_score(w: float, train: list[float], test: list[float]) -> float:
    t = sum(train)
    n = len(train)
    return sum(
        math.log(max(w * math.exp(logpred(y, CENTRES[0], t, n)) + (1 - w) * math.exp(logpred(y, CENTRES[1], t, n)), 1e-300))
        for y in test
    ) / len(test)


def stack(train: list[float]) -> float:
    t, n = sum(train), len(train)
    dens = [(math.exp(logpred(y, CENTRES[0], t - y, n - 1)), math.exp(logpred(y, CENTRES[1], t - y, n - 1))) for y in train]
    best, best_w = -math.inf, 0.5
    for i in range(0, 1001):
        w = i / 1000
        s = sum(math.log(max(w * a + (1 - w) * b, 1e-300)) for a, b in dens)
        if s > best:
            best, best_w = s, w
    return best_w


def world(left: float, seed: int) -> tuple[float, float, float]:
    rng = random.Random(seed)
    draw = lambda n: [(-2.0 if rng.random() < left else 2.0) + rng.gauss(0, 1) for _ in range(n)]
    bma, stk, certain = [], [], 0
    for _ in range(DATASETS):
        train, test = draw(N_TRAIN), draw(N_TEST)
        la, lb = logmarg(train, CENTRES[0]), logmarg(train, CENTRES[1])
        top = max(la, lb)
        wb = math.exp(la - top) / (math.exp(la - top) + math.exp(lb - top))
        certain += wb < 0.01 or wb > 0.99
        bma.append(mix_score(wb, train, test))
        stk.append(mix_score(stack(train), train, test))
    return sum(bma) / DATASETS, sum(stk) / DATASETS, certain / DATASETS


ob, os_, oc = world(0.5, 20261019)
check("8", "stacking, M-open: mean log score of model averaging", -4.8251, ob, 0.25, "sampling")
check("8", "stacking, M-open: mean log score of stacking", -2.0811, os_, 0.02, "sampling")
check("8", "stacking, M-open: share of datasets where averaging is certain", 926 / 1000, oc, 0.04, "sampling")
cb, cs, _ = world(1.0, 20261020)
check("8", "stacking, M-closed: mean log score of model averaging", -1.4189, cb, 0.01, "sampling")
check("8", "stacking, M-closed: stacking minus averaging", -0.0013, cs - cb, 0.002, "sampling")
