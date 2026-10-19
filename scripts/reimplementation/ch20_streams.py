#!/usr/bin/env python3
"""Chapter 20: three of the rebuilt real streams, recomputed from the recipe in the text alone.

The recipe is the paragraph "The recipe of the rebuild" of Chapter 20 and the ECE of Chapter 8 (ten boxes of equal
mass that never split a run of equal forecasts). This module did not read crates/stochology-book/src/streams.rs.
Toronto, the market and the energy interval are short enough to redo here; the flights and the cyclones are not.

Declared limit: written by an AI assistant, like the instrument it checks; a cross-check, not the outside audit of
item 17. Standard library only.
"""

from __future__ import annotations

import csv
import datetime as dt
import math
from pathlib import Path

from common import check, logistic_irls, sigmoid

DATA = Path(__file__).resolve().parent.parent.parent / "data"


def ece(p: list[float], y: list[float]) -> float:
    order = sorted(range(len(p)), key=lambda i: p[i])
    n, total, start = len(p), 0.0, 0
    for k in range(1, 11):
        end = k * n // 10
        if k < 10:
            while 0 < end < n and p[order[end]] == p[order[end - 1]]:
                end += 1
        else:
            end = n
        if end <= start:
            continue
        box = order[start:end]
        total += len(box) / n * abs(sum(p[i] for i in box) / len(box) - sum(y[i] for i in box) / len(box))
        start = end
    return total


# ---------------------------------------------------------------- Toronto, 1997 to 2002

rows = list(csv.DictReader(open(DATA / "toronto_arrests.csv")))


def features(r: dict) -> list[float]:
    a = (float(r["age"]) - 25) / 10
    return [1.0, r["employed"] == "Yes", r["citizen"] == "Yes", float(r["checks"]), a, a * a, r["sex"] == "Male"]


train = [r for r in rows if int(r["year"]) <= 2000]
vault = [r for r in rows if int(r["year"]) >= 2001]
b = logistic_irls([[float(v) for v in features(r)] for r in train], [float(r["released"] == "Yes") for r in train], ridge=1e-3)
p = [sigmoid(sum(bj * float(xj) for bj, xj in zip(b, features(r)))) for r in vault]
y = [float(r["released"] == "Yes") for r in vault]
check("20", "rebuilt Toronto: vault cases", 1488, len(vault))
check("20", "rebuilt Toronto: CITL", 0.0165, sum(p) / len(p) - sum(y) / len(y), 5e-4)
check("20", "rebuilt Toronto: ECE", 0.0354, ece(p, y), 5e-4)
for year, colour, book in [(2001, "Black", 0.155), (2001, "White", 0.070), (2002, "Black", 0.164), (2002, "White", 0.077)]:
    released = [pi for r, pi in zip(vault, p) if int(r["year"]) == year and r["colour"] == colour and r["released"] == "Yes"]
    check("20", f"rebuilt Toronto: FPR of the marker, {year} {colour}", book, sum(pi < 0.70 for pi in released) / len(released), 5e-4)

# ---------------------------------------------------------------- the market

series = [(r["date"], float(r["value"])) for r in csv.DictReader(open(DATA / "djia_1885_2023.csv"))]
returns = [(series[k][0], series[k][1] / series[k - 1][1] - 1) for k in range(1, len(series))]


def quantile(v: list[float], q: float) -> float:
    v = sorted(v)
    pos = q * (len(v) - 1)
    lo, hi = math.floor(pos), math.ceil(pos)
    return v[lo] + (pos - lo) * (v[hi] - v[lo])


hits = [returns[k][1] < quantile([r for _, r in returns[k - 250 : k]], 0.01) for k in range(250, len(returns)) if returns[k][0] >= "2000-01-01"]
check("20", "rebuilt market: sessions in the vault", 6011, len(hits))
check("20", "rebuilt market: violations of the 99% VaR (%)", 1.53, 100 * sum(hits) / len(hits), 0.005)
x, n = sum(hits), len(hits)
lr = -2 * ((n - x) * math.log(0.99) + x * math.log(0.01) - (n - x) * math.log(1 - x / n) - x * math.log(x / n))
check("20", "rebuilt market: Kupiec p-value", 0.00, math.erfc(math.sqrt(lr / 2)), 0.005)

# ---------------------------------------------------------------- energy, Victoria 2014

days: dict[dt.date, list] = {}
for r in csv.DictReader(open(DATA / "vic_elec.csv")):
    d = dt.date.fromisoformat(r["Date"])
    days.setdefault(d, [d.weekday() < 5 and r["Holiday"] != "TRUE", []])[1].append(float(r["Demand"]))
ordered = [(d, w, v) for d, (w, v) in sorted(days.items()) if len(v) == 48]


def point(k: int):
    same = [v for _, w, v in reversed(ordered[:k]) if w == ordered[k][1]][:4]
    return [sum(s[i] for s in same) / 4 for i in range(48)] if len(same) == 4 else None


errors = []
for k in range(len(ordered)):
    pt = point(k)
    errors.append([yv / pv - 1 for yv, pv in zip(ordered[k][2], pt)] if pt else [])
inside = total = 0
for k, (d, _, values) in enumerate(ordered):
    if d.year != 2014:
        continue
    pt = point(k)
    past = [e for es in errors[max(0, k - 56) : k] for e in es]
    if not pt or not past:
        continue
    lo, hi = quantile(past, 0.05), quantile(past, 0.95)
    for yv, pv in zip(values, pt):
        total += 1
        inside += pv * (1 + lo) <= yv <= pv * (1 + hi)
check("20", "rebuilt energy: half hours in the vault", 17424, total)
check("20", "rebuilt energy: coverage of the 90% interval", 0.886, inside / total, 5e-4)
