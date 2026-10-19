#!/usr/bin/env python3
"""Chapter 5, "Three answers to the same question", recomputed from the text alone.

The recipe is the paragraph of Chapter 5 and the header line of its block: W = 60 lambda / (mu (mu - lambda)) minutes,
lambda ~ N(8, 0.6^2) and mu ~ N(12, 0.8^2) per hour with correlation -0.5, 0 or +0.5; the delta method is the tangent
plane at the mean read as a normal, with its second-order mean; Monte Carlo draws the two rates jointly and counts the
days above 20 minutes, a day that never clears counting as above. This module did not read
crates/stochology-book/src/propagate.rs. Its Monte Carlo uses Python's own generator, so its counts agree with the
book's within their sampling error, not to the digit.

Declared limit: written by an AI assistant, like the instrument it checks; a cross-check, not the outside audit of
item 17. Standard library only.
"""

from __future__ import annotations

import math
import random

from common import check

L, M, SL, SM, LIMIT = 8.0, 12.0, 0.6, 0.8, 20.0


def wait(lam: float, mu: float) -> float:
    return math.inf if lam >= mu or mu <= 0 or lam < 0 else 60 * lam / (mu * (mu - lam))


def grad(lam: float, mu: float) -> tuple[float, float]:
    h = 1e-6
    return (
        (wait(lam + h, mu) - wait(lam - h, mu)) / (2 * h),
        (wait(lam, mu + h) - wait(lam, mu - h)) / (2 * h),
    )


def hess(lam: float, mu: float) -> tuple[float, float, float]:
    h = 1e-4
    w = wait
    return (
        (w(lam + h, mu) - 2 * w(lam, mu) + w(lam - h, mu)) / h**2,
        (w(lam, mu + h) - 2 * w(lam, mu) + w(lam, mu - h)) / h**2,
        (w(lam + h, mu + h) - w(lam + h, mu - h) - w(lam - h, mu + h) + w(lam - h, mu - h)) / (4 * h * h),
    )


def normal_sf(z: float) -> float:
    return 0.5 * math.erfc(z / math.sqrt(2))


BOOK = {
    -0.5: dict(sd=4.26, second=11.37, p_delta=0.0095, p_mc=0.0884, median=9.99, q95=24.47, cov=6.00),
    0.0: dict(sd=3.49, second=10.92, p_delta=0.0021, p_mc=0.0499, median=9.99, q95=19.99, cov=0.00),
    0.5: dict(sd=2.48, second=10.47, p_delta=0.0000, p_mc=0.0107, median=9.99, q95=15.90, cov=-6.00),
}

gl, gm = grad(L, M)
check("5", "propagate: wait at the mean rates, min", 10.00, wait(L, M), 0.005)
check("5", "propagate: dW/dlambda", 3.750, gl, 0.0005)
check("5", "propagate: dW/dmu", -3.333, gm, 0.0005)
check("5", "propagate: budget, arrivals", 5.06, gl * gl * SL * SL, 0.005)
check("5", "propagate: budget, pace", 7.11, gm * gm * SM * SM, 0.005)

rng = random.Random(20261019)
N = 400_000
for rho, b in BOOK.items():
    cov = rho * SL * SM
    var = gl * gl * SL * SL + gm * gm * SM * SM + 2 * gl * gm * cov
    sd = math.sqrt(var)
    hll, hmm, hlm = hess(L, M)
    second = wait(L, M) + 0.5 * (hll * SL * SL + hmm * SM * SM + 2 * hlm * cov)
    check("5", f"propagate rho {rho:+.1f}: delta sd", b["sd"], sd, 0.005)
    check("5", f"propagate rho {rho:+.1f}: second-order mean", b["second"], second, 0.005)
    check("5", f"propagate rho {rho:+.1f}: delta P(W > 20)", b["p_delta"], normal_sf((LIMIT - wait(L, M)) / sd), 0.00005)
    check("5", f"propagate rho {rho:+.1f}: covariance term", b["cov"], 2 * gl * gm * cov, 0.005)
    root = math.sqrt(1 - rho * rho)
    ws = []
    for _ in range(N):
        z1, z2 = rng.gauss(0, 1), rng.gauss(0, 1)
        ws.append(wait(L + SL * z1, M + SM * (rho * z1 + root * z2)))
    ws.sort()
    p = sum(1 for w in ws if w > LIMIT) / N
    # the counts of two independent runs of 400 000 draws: four standard errors of their difference
    tol = 4 * math.sqrt(2 * max(p, 1e-4) * (1 - p) / N)
    check("5", f"propagate rho {rho:+.1f}: Monte Carlo P(W > 20)", b["p_mc"], p, tol, "sampling")
    check("5", f"propagate rho {rho:+.1f}: Monte Carlo median", b["median"], ws[round(0.5 * (N - 1))], 0.03, "sampling")
    check("5", f"propagate rho {rho:+.1f}: Monte Carlo q95", b["q95"], ws[round(0.95 * (N - 1))], 0.25, "sampling")
