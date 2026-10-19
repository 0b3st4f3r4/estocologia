#!/usr/bin/env python3
"""Chapter 14, "Failing a parameter, measured", recomputed from the text alone.

The recipe is the paragraph of Chapter 14 and the header of its block: world y = 0.65 x / (1 + x / 20), simulator
y = theta x, 30 points on (0, 4] (taken as 4i/30), noise sd 0.05; three Bayesian regressions with known noise and
N(0, 1) priors on every coefficient: theta alone; theta with b2 x^2 + b3 x^3 + b4 x^4; theta with b1 x + b2 x^2 + b3 x^3
+ b4 x^4. The readings are averaged over 1,000 datasets. This module did not read
crates/stochology-book/src/discrepancy.rs. Its noise comes from Python's own generator, so the averages agree within
their sampling error, not to the digit.

Declared limit: written by an AI assistant, like the instrument it checks; a cross-check, not the outside audit of
item 17. Standard library only.
"""

from __future__ import annotations

import math
import random

from common import check

SD = 0.05
XS = [4 * i / 30 for i in range(1, 31)]


def world(x: float) -> float:
    return 0.65 * x / (1 + x / 20)


def cols(model: int, x: float) -> list[float]:
    if model == 0:
        return [x]
    if model == 1:
        return [x, x**2, x**3, x**4]
    return [x, x, x**2, x**3, x**4]


def solve(a: list[list[float]], b: list[list[float]]) -> list[list[float]]:
    n = len(a)
    m = [row[:] + rb[:] for row, rb in zip(a, b)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(m[r][c]))
        m[c], m[p] = m[p], m[c]
        piv = m[c][c]
        m[c] = [v / piv for v in m[c]]
        for r in range(n):
            if r != c:
                f = m[r][c]
                m[r] = [v - f * w for v, w in zip(m[r], m[c])]
    return [row[n:] for row in m]


def fit(model: int, ys: list[float]) -> tuple[float, float, list[float]]:
    k = len(cols(model, 1.0))
    prec = [[(1.0 if i == j else 0.0) for j in range(k)] for i in range(k)]
    rhs = [0.0] * k
    for x, y in zip(XS, ys):
        c = cols(model, x)
        for i in range(k):
            rhs[i] += c[i] * y / SD**2
            for j in range(k):
                prec[i][j] += c[i] * c[j] / SD**2
    ident = [[(1.0 if i == j else 0.0) for j in range(k)] for i in range(k)]
    cov = solve(prec, ident)
    mean = [sum(cov[i][j] * rhs[j] for j in range(k)) for i in range(k)]
    return mean[0], math.sqrt(cov[0][0]), mean


BOOK = {  # model -> (theta mean, theta sd, coverage, rmse, mean error at 8, rms error at 8)
    0: (0.5647, 0.0039, 0.000, 0.0677, 0.803, 0.804),
    1: (0.6444, 0.0767, 0.953, 0.0461, 0.236, 6.988),
    2: (0.3232, 0.7081, 1.000, 0.0461, 0.101, 6.998),
}
# sampling tolerances: the prediction at x = 8 of the fits with a discrepancy has a spread of about 6.9 per dataset,
# so its mean over 1,000 datasets moves by about 0.22 between generators; three of those are allowed
TOL = {
    0: (0.001, 0.0001, 0.01, 0.002, 0.01, 0.01),
    1: (0.006, 0.001, 0.025, 0.002, 0.66, 0.35),
    2: (0.006, 0.001, 0.01, 0.002, 0.66, 0.35),
}
rng = random.Random(20261019)
acc = {m: [0.0] * 6 for m in BOOK}
N = 1000
for _ in range(N):
    ys = [world(x) + SD * rng.gauss(0, 1) for x in XS]
    for m in BOOK:
        t, sd, mean = fit(m, ys)
        f = lambda x: sum(c * b for c, b in zip(cols(m, x), mean))
        rmse = math.sqrt(sum((f(x) - y) ** 2 for x, y in zip(XS, ys)) / len(XS))
        e8 = f(8.0) - world(8.0)
        for i, v in enumerate((t, sd, float(abs(t - 0.65) <= 1.96 * sd), rmse, e8, e8 * e8)):
            acc[m][i] += v / N
for m in BOOK:
    acc[m][5] = math.sqrt(acc[m][5])
names = ("theta mean", "theta sd", "coverage", "fit rmse in range", "mean error at x = 8", "rms error at x = 8")
for m, book in BOOK.items():
    for i, what in enumerate(names):
        check("14", f"discrepancy, model {m}: {what}", book[i], acc[m][i], TOL[m][i], "sampling")
