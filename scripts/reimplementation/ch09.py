"""Chapter 9: Simpson's paradox in the Charig counts, and a minimal ICP built from the chapter's description.

Simpson: exact accounts on the counts the chapter prints.

ICP: the chapter states the method (for each subset S, test whether the regression of Y on X_S is the same in every
site; keep the sets that pass; return their intersection) and the world (three sites of 4,000 rows; X -> Y with
coefficient 2; Z a child of X with noise that differs by site; W pure noise; in the unstable world the coefficient
of X is 0.5, 2 and 3.5 by site). It does not state how X is distributed in each site, the noise of Z, the test or
its level. This module fixes them as follows and checks the conclusions the chapter draws, not the decimals: X has a
different mean in each site (an intervention on X), the test is a Chow F test of equal intercept and slopes across
sites at level 0.05. The decimals of the printed F statistics belong to the book's own draws and are not compared.
"""

from __future__ import annotations

import math
import random

from common import check, f_sf, solve

# ------------------------------------------------------------------ Simpson
counts = {"small": {"open": (81, 87), "new": (234, 270)}, "large": {"open": (192, 263), "new": (55, 80)}}
rate = lambda s, a: counts[s][a][0] / counts[s][a][1]  # noqa: E731
for s, a, book in [("small", "open", 0.9310), ("small", "new", 0.8667), ("large", "open", 0.7300),
                   ("large", "new", 0.6875)]:
    check("9", f"Simpson: {s} stones, {a} surgery", book, rate(s, a), 5e-5)
raw = {a: sum(counts[s][a][0] for s in counts) / sum(counts[s][a][1] for s in counts) for a in ("open", "new")}
check("9", "Simpson: raw rate, open", 0.7800, raw["open"], 5e-5)
check("9", "Simpson: raw rate, new", 0.8257, raw["new"], 5e-5)
total = sum(counts[s][a][1] for s in counts for a in ("open", "new"))
weight = {s: sum(counts[s][a][1] for a in ("open", "new")) / total for s in counts}
check("9", "Simpson: weight of small stones", 0.5100, weight["small"], 5e-5)
std = {a: sum(weight[s] * rate(s, a) for s in counts) for a in ("open", "new")}
check("9", "Simpson: standardised rate, open", 0.8325, std["open"], 5e-5)
check("9", "Simpson: standardised rate, new", 0.7789, std["new"], 5e-5)
check("9", "Simpson: difference, small stones (points)", 6.44, 100 * (rate("small", "open") - rate("small", "new")),
      5e-3)
check("9", "Simpson: difference, large stones (points)", 4.25, 100 * (rate("large", "open") - rate("large", "new")),
      5e-3)
check("9", "Simpson: standardised difference (points)", 5.37, 100 * (std["open"] - std["new"]), 5e-3)
check("9", "Simpson: hard cases in the open surgery", 0.7514, 263 / 350, 5e-5)
check("9", "Simpson: hard cases in the new surgery", 0.2286, 80 / 350, 5e-5)

# ------------------------------------------------------------------ ICP


def moments(rows):
    """Sufficient statistics of a regression on [1, x, z, w] -> y: X'X, X'y, y'y, n."""
    k = 4
    xx = [[0.0] * k for _ in range(k)]
    xy = [0.0] * k
    yy = 0.0
    for x, z, w, y in rows:
        v = (1.0, x, z, w)
        for i in range(k):
            xy[i] += v[i] * y
            for j in range(k):
                xx[i][j] += v[i] * v[j]
        yy += y * y
    return xx, xy, yy, len(rows)


def rss(mom, cols):
    xx, xy, yy, _ = mom
    sub = [[xx[i][j] for j in cols] for i in cols]
    b = solve(sub, [xy[i] for i in cols])
    return yy - sum(bi * xy[i] for bi, i in zip(b, cols))


def add(m1, m2):
    return ([[a + b for a, b in zip(r1, r2)] for r1, r2 in zip(m1[0], m2[0])], [a + b for a, b in zip(m1[1], m2[1])],
            m1[2] + m2[2], m1[3] + m2[3])


NAMES = {1: "x", 2: "z", 3: "w"}
SUBSETS = [(), (1,), (2,), (3,), (1, 2), (1, 3), (2, 3), (1, 2, 3)]


def chow_p(site_moments, subset) -> float:
    cols = [0, *subset]
    k, g = len(cols), len(site_moments)
    pooled = site_moments[0]
    for m in site_moments[1:]:
        pooled = add(pooled, m)
    rss_sep = sum(rss(m, cols) for m in site_moments)
    n = pooled[3]
    f = ((rss(pooled, cols) - rss_sep) / (k * (g - 1))) / (rss_sep / (n - g * k))
    return f_sf(f, k * (g - 1), n - g * k)


def simulate(rng: random.Random, coef=(2.0, 2.0, 2.0), rows=4000):
    sites = []
    for s in range(3):
        mean_x, sd_z = (-1.0, 0.0, 1.0)[s], (0.5, 1.5, 3.0)[s]
        data = []
        for _ in range(rows):
            x = rng.gauss(mean_x, 1.0)
            y = coef[s] * x + rng.gauss(0.0, 1.0)
            z = x + rng.gauss(0.0, sd_z)
            data.append((x, z, rng.gauss(0.0, 1.0), y))
        sites.append(moments(data))
    return sites


def icp(sites, alpha=0.05):
    accepted = [s for s in SUBSETS if chow_p(sites, s) > alpha]
    if not accepted:
        return accepted, set()
    inter = set(accepted[0])
    for s in accepted[1:]:
        inter &= set(s)
    return accepted, inter


rng = random.Random(2016)
accepted, s_hat = icp(simulate(rng))
check("9", "minimal ICP: number of accepted sets (of 8)", 4, len(accepted), 0, "simulated")
check("9", "minimal ICP: every accepted set contains x (1 if so)", 1, int(all(1 in s for s in accepted)), 0,
      "simulated")
check("9", "minimal ICP: intersection is {x} (1 if so)", 1, int(s_hat == {1}), 0, "simulated")

stable = [icp(simulate(rng))[1] for _ in range(60)]
check("9", "stable mechanism: share of rounds with S_hat = {x}", 0.960, sum(s == {1} for s in stable) / len(stable),
      0.06, "simulated")
check("9", "stable mechanism: share of rounds containing z", 0.015, sum(2 in s for s in stable) / len(stable), 0.05,
      "simulated")
unstable = [icp(simulate(rng, coef=(0.5, 2.0, 3.5)))[1] for _ in range(20)]
check("9", "unstable mechanism: share of rounds with an empty S_hat", 1.0,
      sum(s == set() for s in unstable) / len(unstable), 0.0, "simulated")
