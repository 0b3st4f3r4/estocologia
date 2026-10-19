#!/usr/bin/env python3
"""Chapter 5, "A fourth answer: the multiverse", recomputed from the text alone.

The recipe is the paragraph of Chapter 5 and the header of its block: W = 60 lambda (1 + c^2) / (2 mu (mu - lambda))
minutes with c in {0.5, 1, 1.5}; the rates of mean 8 and 12 per hour and standard deviations 0.6 and 0.8, normal or
lognormal with the same mean and standard deviation; the correlation -0.5, 0 or +0.5 between the underlying normals;
P(W > 20) counted with a day that never clears as above; the same draws for every specification; the
between-specification variance split into main effects with equal weights. This module did not read
crates/stochology-book/src/multiverse.rs. It draws 100,000 pairs with Python's own generator, so its shares agree
with the book's within their sampling error, not to the digit.

Declared limit: written by an AI assistant, like the instrument it checks; a cross-check, not the outside audit of
item 17. Standard library only.
"""

from __future__ import annotations

import math
import random

from common import check

BOOK = {  # (c, law, rho) -> P as printed
    (0.5, "normal", -0.5): 0.0241, (0.5, "normal", 0.0): 0.0081, (0.5, "normal", 0.5): 0.0004,
    (0.5, "lognormal", -0.5): 0.0234, (0.5, "lognormal", 0.0): 0.0075, (0.5, "lognormal", 0.5): 0.0003,
    (1.0, "normal", -0.5): 0.0884, (1.0, "normal", 0.0): 0.0499, (1.0, "normal", 0.5): 0.0107,
    (1.0, "lognormal", -0.5): 0.0875, (1.0, "lognormal", 0.0): 0.0487, (1.0, "lognormal", 0.5): 0.0096,
    (1.5, "normal", -0.5): 0.3210, (1.5, "normal", 0.0): 0.2849, (1.5, "normal", 0.5): 0.2131,
    (1.5, "lognormal", -0.5): 0.3211, (1.5, "lognormal", 0.0): 0.2855, (1.5, "lognormal", 0.5): 0.2131,
}
BOOK_SHARES = {"service cv": 0.93, "rate law": 0.00, "correlation": 0.06, "interaction": 0.02}
N = 100_000


def rate(law: str, mean: float, sd: float, z: float) -> float:
    if law == "normal":
        return mean + sd * z
    s2 = math.log(1 + sd * sd / (mean * mean))
    return math.exp(math.log(mean) - s2 / 2 + math.sqrt(s2) * z)


def wait(lam: float, mu: float, c: float) -> float:
    if lam >= mu or mu <= 0 or lam < 0:
        return math.inf
    return 60 * lam * (1 + c * c) / (2 * mu * (mu - lam))


rng = random.Random(20261019)
draws = [(rng.gauss(0, 1), rng.gauss(0, 1)) for _ in range(N)]
ours = {}
for (c, law, rho), book in BOOK.items():
    root = math.sqrt(1 - rho * rho)
    above = sum(
        1 for z1, z2 in draws if wait(rate(law, 8, 0.6, z1), rate(law, 12, 0.8, rho * z1 + root * z2), c) > 20
    ) / N
    ours[(c, law, rho)] = above
    # two independent counts, of 400,000 and of 100,000 draws: four standard errors of their difference
    tol = 4 * math.sqrt(max(book, 1e-4) * (1 - book) * (1 / N + 1 / 400_000))
    check("5", f"multiverse c {c}, {law}, rho {rho:+.1f}: P(W > 20)", book, above, tol, "sampling")

values = list(ours.values())
mean = sum(values) / len(values)
total = sum((v - mean) ** 2 for v in values) / len(values)


def effect(index: int) -> float:
    levels: dict = {}
    for key, v in ours.items():
        levels.setdefault(key[index], []).append(v)
    return sum(len(v) * (sum(v) / len(v) - mean) ** 2 for v in levels.values()) / len(values) / total


shares = {"service cv": effect(0), "rate law": effect(1), "correlation": effect(2)}
shares["interaction"] = 1 - sum(shares.values())
for name, book in BOOK_SHARES.items():
    check("5", f"multiverse: share of the between-specification variance, {name}", book, shares[name], 0.01, "sampling")
check("5", "multiverse: specifications that ask for the second nurse", 8, sum(1 for v in values if v > 0.05))
