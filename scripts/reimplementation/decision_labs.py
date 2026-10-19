#!/usr/bin/env python3
"""Reimplementation of the decision laboratories of Chapters 10 and 11, from the text of the book alone.

Protocol (docs/OUTSIDE_AUDIT.md, activity 2). Every quantity below is recomputed from the specification that the
chapter states in words and formulas, without reading the Rust of the laboratory, and compared with the value the
book prints. Where the book's quantity is a closed-form or exact account, the comparison is to 1e-3 or tighter;
where it is a simulation, the reimplementation computes the exact value the simulation estimates and accepts the
printed value within four binomial standard errors of 10,000 trajectories.

Declared limit: this reimplementation was written by the same kind of AI assistant that made the readings of
2026-10-18, on 2026-10-19. It did not read crates/stochology-book/src/ch10.rs or ch11.rs before writing these
accounts, but it is not independent in the sense of Chapter 21. It is a cross-check, not an outside audit.

Standard library only. Usage: python3 scripts/reimplementation/decision_labs.py   (exit 1 on any divergence)
"""

from __future__ import annotations

import math
import sys
from math import comb, exp, log

ROWS: list[tuple[str, str, float, float, float, str]] = []


def check(chapter: str, what: str, book: float, ours: float, tol: float, kind: str = "exact") -> None:
    ROWS.append((chapter, what, book, ours, tol, kind))


# ---------------------------------------------------------------- Chapter 10: the Peters coin
# Spec: heads (p = 0.5) multiplies the staked part by 1.5, tails by 0.6; a fraction f is staked each round.


def growth(f: float, p: float = 0.5, up: float = 0.5, down: float = 0.4) -> float:
    return p * log(1 + f * up) + (1 - p) * log(1 - f * down)


def kelly(p: float, b: float, a: float) -> float:
    """f* = p/a - (1-p)/b for a bet that gains b with chance p and loses a otherwise (Chapter 10)."""
    return p / a - (1 - p) / b


def above_start(n: int, f: float) -> float:
    """Exact share of trajectories strictly above the start after n rounds."""
    lu, ld = log(1 + 0.5 * f), log(1 - 0.4 * f)
    return sum(comb(n, k) for k in range(n + 1) if k * lu + (n - k) * ld > 1e-12) / 2**n


def sim_tol(p: float, trajectories: int = 10_000) -> float:
    return 4 * math.sqrt(max(p * (1 - p), 1e-6) / trajectories)


check("10", "ensemble mean per round", 1.0500, 0.5 * 1.5 + 0.5 * 0.6, 1e-4)
check("10", "time mean per round", -0.052680, growth(1.0), 1e-6)
check("10", "Kelly fraction of the Peters coin", 0.25, kelly(0.5, 0.5, 0.4), 1e-12)
check("10", "even-odds Kelly with p = 0.6", 0.20, kelly(0.6, 1, 1), 1e-12)
check("10", "growth at f*", 0.006211, growth(0.25), 1e-6)
check("10", "growth at 2f*", 0.0, growth(0.5), 1e-12)
check("10", "growth at f = 0.70", -0.014200, growth(0.70), 1e-6)
check("10", "all-in expectation after 1000 rounds (x1e21)", 1.55, 1.05**1000 / 1e21, 0.01)
check("10", "all-in median after 1000 rounds (x1e-23)", 1.322, exp(1000 * growth(1.0)) / 1e-23, 0.002)
check("10", "even-odds all-in, expectation after 100 rounds (x1e7)", 8.3, 1.2**100 / 1e7, 0.05)

printed_above = {  # Step 3 of Chapter 10, after the rectification of 2026-10-19
    (1.0, 10): 0.374, (1.0, 100): 0.128, (1.0, 1000): 0.00000,
    (0.5, 10): 0.374, (0.5, 100): 0.450, (0.5, 1000): 0.49110,
    (0.25, 10): 0.618, (0.25, 100): 0.684, (0.25, 1000): 0.96120,
    (0.125, 10): 0.618, (0.125, 100): 0.811, (0.125, 1000): 0.99650,
}
for (f, n), book in printed_above.items():
    exact = above_start(n, f)
    check("10", f"above the start, f = {f}, {n} rounds", book, exact, sim_tol(exact), "simulated")

# The insurance seller: earns 1% per week, loses 95% with chance 0.004.
q = 0.004
check("10", "seller all-in growth per week", -0.00207, (1 - q) * log(1.01) + q * log(0.05), 1e-5)
check("10", "seller Kelly with the true q", 0.648, kelly(1 - q, 0.01, 0.95), 1e-3)
check("10", "seller Kelly with Laplace 1/252", 0.652, kelly(1 - 1 / 252, 0.01, 0.95), 1e-3)
check("10", "250 weeks without a loss", 0.3671, (1 - q) ** 250, 1e-4)

# The wild tail: Pareto with P(X > x) = x^(-alpha), alpha = 1.1.
alpha, n = 1.1, 1_000_000
median_max = (-math.expm1(-math.log(2) / n)) ** (-1 / alpha)  # (1 - m^-alpha)^n = 1/2
check("10", "true mean alpha/(alpha-1)", 11.0, alpha / (alpha - 1), 1e-9)
check("10", "share of the mean above the median maximum (%)", 27.55, 100 * median_max ** (1 - alpha), 0.01)

# Volatility drag: L^2 sigma^2 / 2 per session, sigma = 0.01111, 252 sessions per year.
for lev, book in [(1, 0.0156), (2, 0.0623), (3, 0.1402)]:
    check("10", f"drag per year at {lev}x", book, lev**2 * 0.01111**2 / 2 * 252, 0.0005, "calibrated")
check("10", "loss of the 1987 crash at 3x (%)", 67.8, 300 * 0.2261, 0.05)
check("10", "ruin leverage of the 1987 crash", 4.42, 1 / 0.2261, 0.005)

# ---------------------------------------------------------------- Chapter 11
# Bernoulli's ship: wealth 100,000, premium 1,800, loss 34,000 with chance 0.05.
W, C, L, p = 100_000, 1_800, 34_000, 0.05
check("11", "threshold C/L", 0.05294, C / L, 1e-5)
check("11", "shipowner growth without insurance", -0.0208, p * log((W - L) / W), 1e-4)
check("11", "shipowner growth with insurance", -0.0182, log((W - C) / W), 1e-4)


def insurer_growth(contracts: int, capital: float = 50_000) -> float:
    """Exact binomial account: mean of log wealth over every possible number of claims."""
    total = 0.0
    for k in range(contracts + 1):
        prob = comb(contracts, k) * p**k * (1 - p) ** (contracts - k)
        wealth = contracts * (capital + C) - k * L
        if prob > 0:
            total += prob * (log(wealth / (contracts * capital)) if wealth > 0 else -math.inf)
    return total


for contracts, book in [(1, -0.018042), (10, 0.000854), (100, 0.001888), (1000, 0.001987)]:
    check("11", f"insurer growth with {contracts} contracts", book, insurer_growth(contracts), 1e-6)

# EVPI and EVSI: prevalence 0.3, treating costs 1, a sick patient untreated costs 10.


def best_cost(prev: float) -> float:
    return min(1.0, 10 * prev)


def evpi(prev: float) -> float:
    return best_cost(prev) - prev * 1.0


def evsi(prev: float, sens: float, spec: float) -> float:
    pos = prev * sens + (1 - prev) * (1 - spec)
    neg = 1 - pos
    post_pos = prev * sens / pos
    post_neg = prev * (1 - sens) / neg
    return best_cost(prev) - (pos * best_cost(post_pos) + neg * best_cost(post_neg))


def mutual_information(prev: float, sens: float, spec: float) -> float:
    joint = {
        (1, 1): prev * sens, (1, 0): prev * (1 - sens),
        (0, 1): (1 - prev) * (1 - spec), (0, 0): (1 - prev) * spec,
    }
    px = {1: prev, 0: 1 - prev}
    py = {r: joint[(1, r)] + joint[(0, r)] for r in (0, 1)}
    return sum(v * math.log2(v / (px[d] * py[r])) for (d, r), v in joint.items() if v > 0)


check("11", "EVPI at prevalence 0.3", 0.70, evpi(0.3), 1e-9)
check("11", "EVSI of the sensitive test", 0.2200, evsi(0.3, 0.90, 0.70), 1e-4)
check("11", "EVSI of the specific test", 0.0000, evsi(0.3, 0.30, 0.95), 1e-4)
check("11", "mutual information, sensitive test (bit)", 0.241, mutual_information(0.3, 0.90, 0.70), 1e-3)
check("11", "mutual information, specific test (bit)", 0.079, mutual_information(0.3, 0.30, 0.95), 1e-3)
check("11", "post-test chance after a negative specific test", 0.24, 0.3 * 0.7 / (0.3 * 0.7 + 0.7 * 0.95), 1e-3)
for prev, book in [(0.05, 0.450), (0.60, 0.400)]:
    check("11", f"EVPI at prevalence {prev}", book, evpi(prev), 1e-9)

# The dikes: annual maxima GEV(mu = 1.2567, sigma = 0.762, xi), cost h - 2 + 3000 P(max > h).
MU, SIGMA = 1.2567, 0.762


def exceed(h: float, xi: float) -> float:
    z = (h - MU) / SIGMA
    if abs(xi) < 1e-12:
        return -math.expm1(-exp(-z))
    t = 1 + xi * z
    if t <= 0:
        return 0.0 if xi < 0 else 1.0
    return -math.expm1(-(t ** (-1 / xi)))


def level(T: float, xi: float) -> float:
    y = -math.log1p(-1 / T)
    if abs(xi) < 1e-12:
        return MU - SIGMA * log(y)
    return MU + SIGMA / xi * (y ** (-xi) - 1)


def optimal_height(xi: float) -> float:
    lo, hi = 2.0, 40.0
    cost = lambda h: h - 2 + 3000 * exceed(h, xi)  # noqa: E731
    for _ in range(200):  # golden-section search on a unimodal cost
        a, b = lo + 0.382 * (hi - lo), lo + 0.618 * (hi - lo)
        lo, hi = (lo, b) if cost(a) < cost(b) else (a, hi)
    return (lo + hi) / 2


for xi, t100, t10k, hopt in [(-0.1, 4.07, 5.84, 5.839), (0.0, 4.76, 8.27, 7.564),
                             (0.1, 5.71, 12.78, 9.809), (0.2, 7.01, 21.49, 12.584)]:
    check("11", f"100-year level, xi = {xi}", t100, level(100, xi), 0.005)
    check("11", f"10,000-year level, xi = {xi}", t10k, level(10_000, xi), 0.005)
    check("11", f"optimal height, xi = {xi}", hopt, optimal_height(xi), 0.002)

# Chouldechova: FPR = p/(1-p) * (1-PPV)/PPV * (1-FNR)
fpr = lambda prev, ppv, fnr: prev / (1 - prev) * (1 - ppv) / ppv * (1 - fnr)  # noqa: E731
check("11", "identity forces FPR on Black (PPV and FNR of White)", 0.1769, fpr(0.111, 0.3491, 0.240), 5e-4)


def main() -> int:
    bad = 0
    print(f"{'ch':<3} {'quantity':<58} {'book':>12} {'reimpl.':>12}  verdict")
    for ch, what, book, ours, tol, kind in ROWS:
        ok = abs(book - ours) <= tol
        bad += not ok
        print(f"{ch:<3} {what:<58} {book:>12.6g} {ours:>12.6g}  {'agrees' if ok else 'DIVERGES'} ({kind})")
    print(f"\n{len(ROWS)} quantities, {len(ROWS) - bad} agree, {bad} diverge")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
