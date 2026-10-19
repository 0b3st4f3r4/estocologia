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


def normal_cdf(z: float) -> float:
    return 0.5 * math.erfc(-z / math.sqrt(2))


def betainc(a: float, b: float, x: float) -> float:
    """Regularised incomplete beta function I_x(a, b)."""
    if x <= 0:
        return 0.0
    if x >= 1:
        return 1.0
    lbeta = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b)
    front = math.exp(lbeta + a * math.log(x) + b * math.log(1 - x))
    if x < (a + 1) / (a + b + 2):
        return front * betacf(a, b, x) / a
    return 1 - front * betacf(b, a, 1 - x) / b


def beta_ppf(q: float, a: float, b: float) -> float:
    lo, hi = 0.0, 1.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if betainc(a, b, mid) < q:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def nelder_mead(f, x0: list[float], step: float = 0.5, iters: int = 4000, tol: float = 1e-14) -> list[float]:
    n = len(x0)
    pts = [list(x0)] + [[x0[j] + (step if j == i else 0.0) for j in range(n)] for i in range(n)]
    vals = [f(p) for p in pts]
    for _ in range(iters):
        order = sorted(range(n + 1), key=vals.__getitem__)
        pts, vals = [pts[i] for i in order], [vals[i] for i in order]
        if abs(vals[-1] - vals[0]) < tol:
            break
        cen = [sum(p[j] for p in pts[:-1]) / n for j in range(n)]
        ref = [cen[j] + (cen[j] - pts[-1][j]) for j in range(n)]
        fr = f(ref)
        if fr < vals[0]:
            exp_ = [cen[j] + 2 * (cen[j] - pts[-1][j]) for j in range(n)]
            fe = f(exp_)
            pts[-1], vals[-1] = (exp_, fe) if fe < fr else (ref, fr)
        elif fr < vals[-2]:
            pts[-1], vals[-1] = ref, fr
        else:
            con = [cen[j] + 0.5 * (pts[-1][j] - cen[j]) for j in range(n)]
            fc = f(con)
            if fc < vals[-1]:
                pts[-1], vals[-1] = con, fc
            else:
                pts = [pts[0]] + [[pts[0][j] + 0.5 * (p[j] - pts[0][j]) for j in range(n)] for p in pts[1:]]
                vals = [vals[0]] + [f(p) for p in pts[1:]]
    return pts[min(range(n + 1), key=vals.__getitem__)]


def beta_from_quantiles(quantiles: dict[float, float]) -> tuple[float, float]:
    """Least squares on log quantiles, from (log a, log b) = (0, log 100), as the research code did."""
    def loss(lab):
        a, b = math.exp(lab[0]), math.exp(lab[1])
        return sum((math.log(beta_ppf(q, a, b)) - math.log(v)) ** 2 for q, v in quantiles.items())
    lab = nelder_mead(loss, [0.0, math.log(100.0)])
    return math.exp(lab[0]), math.exp(lab[1])


def sigmoid(z: float) -> float:
    return 1 / (1 + math.exp(-z)) if z >= 0 else math.exp(z) / (1 + math.exp(z))


def logistic_irls(rows: list[list[float]], y: list[float], ridge: float = 0.0, weights=None, iters: int = 80):
    """Logistic regression by Newton-Raphson, with a ridge outside the intercept (column 0). Optional case weights."""
    k = len(rows[0])
    b = [0.0] * k
    w_case = weights or [1.0] * len(y)
    for _ in range(iters):
        h = [[0.0] * k for _ in range(k)]
        g = [0.0] * k
        for x, yi, wc in zip(rows, y, w_case):
            eta = sum(bj * xj for bj, xj in zip(b, x))
            p = sigmoid(eta)
            w = wc * p * (1 - p)
            r = wc * (yi - p)
            for i in range(k):
                xi = x[i]
                if xi == 0.0:
                    continue
                g[i] += r * xi
                wi = w * xi
                hi = h[i]
                for j in range(i, k):
                    hi[j] += wi * x[j]
        for i in range(k):
            for j in range(i):
                h[i][j] = h[j][i]
        for i in range(1, k):
            h[i][i] += ridge
            g[i] -= ridge * b[i]
        step = solve(h, g)
        b = [bj + sj for bj, sj in zip(b, step)]
        if max(abs(s) for s in step) < 1e-9:
            break
    return b
