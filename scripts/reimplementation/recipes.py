#!/usr/bin/env python3
"""Worked examples that had no recipe in the text, recomputed from the recipe the text now states (2026-10-19).

They came from the course and the paper without a laboratory. A critical reading of 2026-10-19 wrote a recipe for
each one that could be reconstructed, and this module recomputes it; the ones that could not be reconstructed were
cut from the text (Appendix E). Every account here is exact or analytic, so the tolerances only cover the rounding of
the printed value.

Declared limit: written by an AI assistant; a cross-check, not the outside audit of item 17.

Standard library only. Run through scripts/reimplementation/run.py.
"""

from __future__ import annotations

from math import comb, log

from common import check

# ---------------------------------------------------------------- Chapter 3: reaching 20 decibans under the null
# Spec: a 0.6 coin against an honest one, paths of 500 tosses of the honest coin; the share of paths whose likelihood
# ratio reaches 100 at some toss, counted exactly by carrying the not-yet-crossed paths toss by toss.

up, down, target = log(1.2), log(0.8), log(100.0)
alive = {0: 1.0}
crossed = 0.0
for n in range(1, 501):
    nxt: dict[int, float] = {}
    for heads, prob in alive.items():
        for h in (heads, heads + 1):
            if h * up + (n - h) * down >= target - 1e-12:
                crossed += prob / 2
            else:
                nxt[h] = nxt.get(h, 0.0) + prob / 2
    alive = nxt
check("3", "null paths of 500 tosses reaching 20 decibans (%)", 0.87, 100 * crossed, 0.005)

# ---------------------------------------------------------------- Chapter 7: the base rate as environment
# Spec: the mammography test of Chapter 3 (sensitivity 0.9, false positives 0.09). The forecaster who ignores the base
# rate forecasts with a prior of one half; expected Brier scores at prevalence 1% and 33%.

sens, fpr = 0.9, 0.09


def posterior(prior: float, positive: bool) -> float:
    if positive:
        return prior * sens / (prior * sens + (1 - prior) * fpr)
    return prior * (1 - sens) / (prior * (1 - sens) + (1 - prior) * (1 - fpr))


def brier(prevalence: float, prior: float) -> float:
    fpos, fneg = posterior(prior, True), posterior(prior, False)
    return (
        prevalence * sens * (1 - fpos) ** 2
        + (1 - prevalence) * fpr * fpos**2
        + prevalence * (1 - sens) * (1 - fneg) ** 2
        + (1 - prevalence) * (1 - fpr) * fneg**2
    )


check("7", "forecast of whoever ignores the base rate, after a positive", 0.909, posterior(0.5, True), 5e-4)
check("7", "forecast of whoever ignores the base rate, after a negative", 0.099, posterior(0.5, False), 5e-4)
check("7", "screening 1%: Brier of whoever ignores the base rate", 0.083, brier(0.01, 0.5), 5e-4)
check("7", "screening 1%: Brier of whoever uses it", 0.009, brier(0.01, 0.01), 5e-4)
check("7", "referral 33%: Brier of whoever insists on 1%", 0.28, brier(0.33, 0.01), 5e-3)
check("7", "referral 33%: Brier of whoever ignores the base rate", 0.085, brier(0.33, 0.5), 5e-4)

# ---------------------------------------------------------------- Chapter 10: Kelly on an estimated chance
# Spec: even-odds bet with chance 0.6; the chance estimated by the frequency of 25 rounds; full Kelly 2p - 1 on the
# estimate, growth under the true chance; half Kelly against full Kelly at the true chance.

p = 0.6


def g(f: float) -> float:
    if f >= 1:
        return float("-inf")  # staking everything at even odds: one loss is ruin
    return p * log(1 + f) + (1 - p) * log(1 - f)


negative = sum(comb(25, k) * p**k * (1 - p) ** (25 - k) for k in range(26) if 2 * k / 25 - 1 > 0 and g(2 * k / 25 - 1) < 0)
check("10", "full Kelly on 25 rounds: share of estimates with negative growth (%)", 15, 100 * negative, 0.5)
check("10", "full Kelly on 25 rounds: smallest count that turns growth negative", 18,
      min(k for k in range(26) if 2 * k / 25 - 1 > 0 and g(2 * k / 25 - 1) < 0))
check("10", "half Kelly: share of the growth of full Kelly (%)", 75, 100 * g(0.1) / g(0.2), 0.5)

# ---------------------------------------------------------------- Chapter 10: pooling and the horizon of the mean
# Spec: three players of the Peters coin split their wealth every round; growth is the mean of the log of the average
# of their three factors over the eight outcomes. The horizon of N trajectories is ln N / (ln E[M] - E[ln M]).

pooled = (log(1.5) + 3 * log(1.2) + 3 * log(0.9) + log(0.6)) / 8
check("10", "three players who pool: growth per round (%)", 1.6, 100 * pooled, 0.05)
gap = log(1.05) - (0.5 * log(1.5) + 0.5 * log(0.6))
check("10", "horizon of the mean, 10 trajectories (rounds)", 23, log(10) / gap, 0.5)
check("10", "horizon of the mean, a million trajectories (rounds)", 136, log(1e6) / gap, 0.5)
