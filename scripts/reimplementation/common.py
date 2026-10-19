"""Shared registry of the reimplementation: every recomputed quantity, with the value the book prints.

Three verdicts. `agrees`: within the declared tolerance. `DIVERGES`: outside it, and the run fails. `open`: the text
of the book does not fix a choice that the number depends on, so the reimplementation cannot recompute it from the
text alone; the row records which choice is missing and does not fail the run, because the remedy is to write the
choice into the text, not to copy it from the code.
"""

from __future__ import annotations

import math

ROWS: list[dict] = []


def check(chapter: str, what: str, book: float, ours: float, tol: float = 0.0, kind: str = "exact") -> None:
    ROWS.append(dict(chapter=chapter, what=what, book=book, ours=ours, tol=tol, kind=kind, verdict=None))


def open_choice(chapter: str, what: str, book: float, missing: str) -> None:
    ROWS.append(dict(chapter=chapter, what=what, book=book, ours=float("nan"), tol=0.0, kind=missing, verdict="open"))


def verdict(row: dict) -> str:
    if row["verdict"] == "open":
        return "open"
    return "agrees" if abs(row["book"] - row["ours"]) <= row["tol"] else "DIVERGES"


# ---------------------------------------------------------------- numerical helpers, standard library only


def betacf(a: float, b: float, x: float) -> float:
    qab, qap, qam = a + b, a + 1, a - 1
    c, d = 1.0, 1 - qab * x / qap
    d = 1 / d if abs(d) > 1e-300 else 1e300
    h = d
    for m in range(1, 300):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1 + aa * d
        d = 1 / d if abs(d) > 1e-300 else 1e300
        c = 1 + aa / c if abs(1 + aa / c) > 1e-300 else 1e-300
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1 + aa * d
        d = 1 / d if abs(d) > 1e-300 else 1e300
        c = 1 + aa / c if abs(1 + aa / c) > 1e-300 else 1e-300
        delta = d * c
        h *= delta
        if abs(delta - 1) < 1e-12:
            break
    return h


def f_sf(f: float, d1: float, d2: float) -> float:
    """P(F > f) for an F(d1, d2) variable, by the regularised incomplete beta function."""
    if f <= 0:
        return 1.0
    x = d2 / (d2 + d1 * f)
    a, b = d2 / 2, d1 / 2
    lbeta = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b)
    front = math.exp(lbeta + a * math.log(x) + b * math.log(1 - x))
    if x < (a + 1) / (a + b + 2):
        return front * betacf(a, b, x) / a
    return 1 - front * betacf(b, a, 1 - x) / b


def solve(m: list[list[float]], v: list[float]) -> list[float]:
    n = len(v)
    a = [row[:] + [v[i]] for i, row in enumerate(m)]
    for i in range(n):
        p = max(range(i, n), key=lambda r: abs(a[r][i]))
        a[i], a[p] = a[p], a[i]
        for r in range(n):
            if r != i:
                f = a[r][i] / a[i][i]
                a[r] = [x - f * y for x, y in zip(a[r], a[i])]
    return [a[i][n] / a[i][i] for i in range(n)]
