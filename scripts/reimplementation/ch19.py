"""Chapter 19: the gate of the invariant set (the second ruler and the saturation rule), from the text alone.

The chapter states the method: one pooled least-squares model of Y on the columns of S; the residual standardised by
a scale estimated without looking at the environment; Kruskal-Wallis on its location and Brown-Forsythe on its scale
between environments; Bonferroni over the two tests; a candidate with 10 values or fewer enters as a factor; a set is
saturated when the 1-NN leaving the row out predicts the environment from the columns of S with a hit rate of 0.99 or
more, and saturated sets are excluded before the intersection. It states the worlds: three environments of 600 rows;
Y = x1 + 0.8 x2 + noise; x3 pure noise; x4 shifted by the environment without acting on Y; world A shifts the means of
x1 and x2; world B adds a shift of its own to Y and has three candidates; world C is world A plus x5, the environment
itself.

It does not state the sizes of the shifts, nor the noise scales. This module fixes them (below) and checks the
conclusions the chapter draws: which sets are accepted, how many are saturated, and the returned set with and without
the rule. Two declared departures from the laboratory, for speed in the standard library: the p-values are the
asymptotic ones (chi-square with 2 degrees of freedom for Kruskal-Wallis, F for Brown-Forsythe) instead of 999
permutations, and the 1-NN hit rate is measured on 300 rows (100 per environment) instead of 1,800.

One choice recovered by trying the readings the text allows: in the 1-NN, a factor column must match exactly, as the
regression treats it as a factor. With the factor read as a number at distance 1, only 11 of the 16 sets that contain
x5 come out saturated here, and the chapter's 16 are not reproduced; the text should say how the 1-NN measures
distance on factors and whether it scales the numeric columns.
"""

from __future__ import annotations

import itertools
import math
import random
import statistics as st

from common import check, f_sf, solve

ALPHA = 0.05


def make_world(kind: str, rng: random.Random):
    rows = []
    for e in range(3):
        for _ in range(600):
            x1 = rng.gauss((0.0, 1.0, 2.0)[e], 1.0)
            x2 = rng.gauss((0.0, -1.0, 1.0)[e], 1.0)
            x3 = rng.gauss(0.0, 1.0)
            x4 = rng.gauss((0.0, 1.5, 3.0)[e], 1.0)
            y = x1 + 0.8 * x2 + rng.gauss(0.0, 1.0) + ((0.0, 1.0, -1.0)[e] if kind == "B" else 0.0)
            row = {"x1": x1, "x2": x2, "x3": x3, "x4": x4, "y": y, "e": e}
            if kind == "C":
                row["x5"] = float(e)
            rows.append(row)
    return rows


def design(row: dict, subset: tuple[str, ...], levels: dict[str, list[float]]) -> list[float]:
    v = [1.0]
    for c in subset:
        if c in levels:  # factor: one dummy per level after the first
            v.extend(1.0 if row[c] == lv else 0.0 for lv in levels[c][1:])
        else:
            v.append(row[c])
    return v


def residuals(rows, subset):
    levels = {c: sorted({r[c] for r in rows}) for c in subset if len({r[c] for r in rows}) <= 10}
    xs = [design(r, subset, levels) for r in rows]
    k = len(xs[0])
    xtx = [[sum(x[i] * x[j] for x in xs) for j in range(k)] for i in range(k)]
    xty = [sum(x[i] * r["y"] for x, r in zip(xs, rows)) for i in range(k)]
    b = solve(xtx, xty)
    res = [r["y"] - sum(bi * xi for bi, xi in zip(b, x)) for x, r in zip(xs, rows)]
    scale = st.pstdev(res)
    return [x / scale for x in res]


def kruskal_wallis_p(values, groups) -> float:
    order = sorted(range(len(values)), key=values.__getitem__)
    ranks = [0.0] * len(values)
    for pos, i in enumerate(order):
        ranks[i] = pos + 1.0  # continuous data: no ties
    n = len(values)
    h = 0.0
    for g in set(groups):
        rs = [ranks[i] for i in range(n) if groups[i] == g]
        h += len(rs) * (st.mean(rs) - (n + 1) / 2) ** 2
    h *= 12 / (n * (n + 1))
    return math.exp(-h / 2)  # chi-square survival with 2 degrees of freedom


def brown_forsythe_p(values, groups) -> float:
    gs = sorted(set(groups))
    med = {g: st.median([v for v, gg in zip(values, groups) if gg == g]) for g in gs}
    z = [abs(v - med[g]) for v, g in zip(values, groups)]
    grand = st.mean(z)
    between = sum(len([1 for g2 in groups if g2 == g]) * (st.mean([zz for zz, g2 in zip(z, groups) if g2 == g]) - grand) ** 2
                  for g in gs)
    means = {g: st.mean([zz for zz, g2 in zip(z, groups) if g2 == g]) for g in gs}
    within = sum((zz - means[g]) ** 2 for zz, g in zip(z, groups))
    k, n = len(gs), len(values)
    f = (between / (k - 1)) / (within / (n - k))
    return f_sf(f, k - 1, n - k)


def one_nn_hit_rate(rows, subset) -> float:
    if not subset:
        return 1 / 3
    sample = [r for e in range(3) for r in [r for r in rows if r["e"] == e][:100]]
    factors = {c for c in subset if len({r[c] for r in rows}) <= 10}
    hits = 0
    for i, a in enumerate(sample):
        best, label = math.inf, None
        for j, b in enumerate(sample):
            if i == j:
                continue
            # A factor (10 values or fewer) must match exactly; numeric columns enter by squared distance.
            if any(a[c] != b[c] for c in factors):
                continue
            d = sum((a[c] - b[c]) ** 2 for c in subset if c not in factors)
            if d < best:
                best, label = d, b["e"]
        hits += label == a["e"]
    return hits / len(sample)


def gate(rows, candidates):
    groups = [r["e"] for r in rows]
    accepted, saturated, saturated_accepted, saturated_sets = [], 0, 0, []
    for size in range(len(candidates) + 1):
        for subset in itertools.combinations(candidates, size):
            res = residuals(rows, subset)
            p_set = min(1.0, 2 * min(kruskal_wallis_p(res, groups), brown_forsythe_p(res, groups)))
            ok = p_set > ALPHA
            if one_nn_hit_rate(rows, subset) >= 0.99:
                saturated += 1
                saturated_accepted += ok
                if ok:
                    saturated_sets.append(set(subset))
                continue
            if ok:
                accepted.append(set(subset))
    with_rule = set.intersection(*accepted) if accepted else set()
    family = accepted + saturated_sets
    without_rule = set.intersection(*family) if family else set()
    return accepted, saturated, saturated_accepted, with_rule, without_rule


rng = random.Random(2019)
for kind, candidates, n_sets, n_acc, n_sat, s_hat in [
    ("A", ("x1", "x2", "x3", "x4"), 16, 4, 0, {"x1", "x2"}),
    ("B", ("x1", "x2", "x3"), 8, 0, 0, set()),
    ("C", ("x1", "x2", "x3", "x4", "x5"), 32, 4, 16, {"x1", "x2"}),
]:
    rows = make_world(kind, rng)
    accepted, saturated, saturated_accepted, with_rule, without_rule = gate(rows, candidates)
    check("19", f"world {kind}: sets in the family", n_sets, 2 ** len(candidates), 0)
    check("19", f"world {kind}: accepted non-saturated sets", n_acc, len(accepted), 0, "simulated")
    check("19", f"world {kind}: saturated sets tested", n_sat, saturated, 0, "simulated")
    check("19", f"world {kind}: saturated sets accepted", n_sat, saturated_accepted, 0, "simulated")
    check("19", f"world {kind}: S_hat with the saturation rule matches (1 if so)", 1, int(with_rule == s_hat), 0,
          "simulated")
    without_book = set() if kind in ("B", "C") else s_hat
    check("19", f"world {kind}: S_hat without the rule matches (1 if so)", 1, int(without_rule == without_book), 0,
          "simulated")
