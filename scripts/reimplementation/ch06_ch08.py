"""Chapters 6 to 8: the filter, the ghosts in the noise, and the forecaster's thermometer.

Exact accounts where the chapter states one, and small simulations with Python's own generator where it states the
process: Berkson with two independent standard normals selected on their sum above the cutoff (the selected shares
24.0%, 14.4% and 7.9% follow from that reading and from nothing else); a symmetric two-state chain whose persistence
is the chance of repeating the previous letter; the hot hand on sequences of three tosses (the exact null of 5/12);
100 people tossing 20 coins, with the longest run of heads.

The gardens of Chapter 7 and the forecasters and null ECE of Chapter 8 follow the recipes that Step 3 of each chapter
states since 2026-10-19, with fewer replicas than the book; Chapter 8 is also checked through the identities that hold
among the printed numbers.
"""

from __future__ import annotations

import itertools
import math
import random
import statistics as st

from common import check

# ------------------------------------------------------------------ Chapter 6: the Literary Digest
share_r, resp_r, resp_l = 0.608, 0.168, 0.352
f = share_r * resp_r + (1 - share_r) * resp_l
roosevelt = share_r * resp_r / f
sigma = math.sqrt(share_r * (1 - share_r))
check("6", "response rate f", 0.240, f, 5e-4)
check("6", "Roosevelt among respondents (expected)", 0.4259, roosevelt, 1e-3, "simulated")
check("6", "error of the poll", -0.1821, roosevelt - share_r, 1e-3, "simulated")
meng = -0.21 * math.sqrt((1 - f) / f) * sigma
check("6", "Meng identity with rho = -0.21", -0.1824, meng, 5e-4)
check("6", "equivalent random sample (people)", 7.2, (sigma / 0.1821) ** 2, 0.05)
n = 2_400_031
margin = sigma / math.sqrt(n) * math.sqrt(1 - n / 10_000_000)
check("6", "poll margin (1 se with finite population, points)", 0.0276, 100 * margin, 5e-4)
check("6", "error in margins", 661, 0.1821 / margin, 2)
check("6", "deviation of samples of 7 people (points)", 18.4, 100 * sigma / math.sqrt(7), 0.15)
check("6", "deviation of 50,000 interviews (points)", 0.22, 100 * sigma / math.sqrt(50_000), 5e-3)

rng = random.Random(1946)
pairs = [(rng.gauss(0, 1), rng.gauss(0, 1)) for _ in range(200_000)]
check("6", "Berkson: correlation in the population", 0.0012, st.correlation(*zip(*pairs)), 0.006, "simulated")
for cutoff, b_share, b_corr in [(1.0, 0.240, -0.6168), (1.5, 0.144, -0.6790), (2.0, 0.079, -0.7315)]:
    sel = [(a, b) for a, b in pairs if a + b > cutoff]
    check("6", f"Berkson cutoff {cutoff}: share selected", b_share, len(sel) / len(pairs), 0.003, "simulated")
    check("6", f"Berkson cutoff {cutoff}: correlation among the selected", b_corr, st.correlation(*zip(*sel)), 0.01,
          "simulated")
    exact_share = 0.5 * math.erfc(cutoff / math.sqrt(2) / math.sqrt(2))
    check("6", f"Berkson cutoff {cutoff}: exact share P(N(0,2) > c)", b_share, exact_share, 5e-4)

check("6", "Simpson, half-and-half population: open", 0.831, 0.5 * (81 / 87 + 192 / 263), 5e-4)
check("6", "Simpson, half-and-half population: new", 0.777, 0.5 * (234 / 270 + 55 / 80), 5e-4)

for persistence, b_var, b_eff in [(0.500, 0.002487, 101), (0.800, 0.009899, 25), (0.235, 0.000765, 327)]:
    r = 2 * persistence - 1  # lag-1 autocorrelation of the symmetric chain
    nn = 100
    var = 0.25 / nn * (1 + 2 * sum((1 - k / nn) * r**k for k in range(1, nn)))
    check("6", f"chain persistence {persistence}: variance of the mean of 100", b_var, var, 0.06 * var, "simulated")
    check("6", f"chain persistence {persistence}: effective size", b_eff, 0.25 / var, 0.06 * 0.25 / var, "simulated")

# ------------------------------------------------------------------ Chapter 7
props = []
heads_after_heads = after_heads = 0
for seq in itertools.product((0, 1), repeat=3):
    nxt = [seq[i + 1] for i in range(2) if seq[i] == 1]
    if nxt:
        props.append(sum(nxt) / len(nxt))
        heads_after_heads += sum(nxt)
        after_heads += len(nxt)
check("7", "hot hand: mean of the proportions per sequence", 0.4167, st.mean(props), 5e-5)
check("7", "hot hand: 5/12", 0.4167, 5 / 12, 5e-5)
check("7", "hot hand: pooled proportion", 0.5000, heads_after_heads / after_heads, 1e-9)


def longest_heads(seq):
    best = cur = 0
    for x in seq:
        cur = cur + 1 if x else 0
        best = max(best, cur)
    return best


rng = random.Random(20)
maxima = [max(longest_heads([rng.random() < 0.5 for _ in range(20)]) for _ in range(100)) for _ in range(1000)]
check("7", "100 people x 20 coins: share with a run of 6 heads (%)", 100.0, 100 * sum(m >= 6 for m in maxima) / 1000,
      0.5, "simulated")
check("7", "100 people x 20 coins: mean longest run", 9.5, st.mean(maxima), 0.15, "simulated")
check("7", "Mayo severity of the pre-registered p = 0.04", 0.96, 1 - 0.04, 1e-12)
check("7", "Mayo severity of the garden (1 - 0.423)", 0.58, 1 - 0.423, 5e-3)
ppv = lambda power, r, alpha: power * r / (power * r + alpha)  # noqa: E731
for r, b5, bg in [(1 / 1000, 0.016, 0.002), (1 / 10, 0.615, 0.159), (1 / 3, 0.842, 0.387)]:
    check("7", f"Ioannidis PPV, R = {r:.4g}, alpha 0.05", b5, ppv(0.8, r, 0.05), 5e-4)
    # the garden alpha is printed as 42.3%; the tolerance covers its rounding.
    check("7", f"Ioannidis PPV, R = {r:.4g}, garden alpha 0.423", bg, ppv(0.8, r, 0.423), 1e-3)
# the gardens, from the recipe of Step 3 (since 2026-10-19), with fewer replicas than the book
from common import normal_cdf, sigmoid, solve  # noqa: E402


def model_garden(rng: random.Random, replicas: int, candidates: int = 500, predictors: int = 50):
    wins_val = wins_test = 0
    sum_val = sum_test = 0.0
    for _ in range(replicas):
        rows = [[1.0] + [rng.gauss(0, 1) for _ in range(predictors)] for _ in range(75)]
        y = [rng.gauss(0, 1) for _ in range(75)]
        train = range(25)
        gram = [[sum(rows[r][i] * rows[r][j] for r in train) for j in range(predictors + 1)] for i in range(predictors + 1)]
        xy = [sum(rows[r][i] * y[r] for r in train) for i in range(predictors + 1)]
        best_val, best_test = math.inf, math.inf
        for _ in range(candidates):
            cols = [0] + rng.sample(range(1, predictors + 1), rng.randint(1, 5))
            beta = solve([[gram[i][j] for j in cols] for i in cols], [xy[i] for i in cols])
            pred = lambda r: sum(b * rows[r][c] for b, c in zip(beta, cols))  # noqa: E731
            mse_val = sum((y[r] - pred(r)) ** 2 for r in range(25, 50)) / 25
            if mse_val < best_val:
                best_val = mse_val
                best_test = sum((y[r] - pred(r)) ** 2 for r in range(50, 75)) / 25
        wins_val += best_val < 1.0
        wins_test += best_test < 1.0
        sum_val += best_val
        sum_test += best_test
    return 100 * wins_val / replicas, 100 * wins_test / replicas, sum_val / replicas, sum_test / replicas


reps = 150
v_win, t_win, v_err, t_err = model_garden(random.Random(2026), reps)
se = lambda p: 100 * math.sqrt(p / 100 * (1 - p / 100) / reps)  # noqa: E731
check("7", "garden of models: winners that beat zero on validation (%)", 79.5, v_win, 4 * se(79.5), "simulated")
check("7", "garden of models: winners that beat zero on fresh data (%)", 30.0, t_win, 4 * se(30.0), "simulated")
check("7", "garden of models: winner's validation error", 0.814, v_err, 0.05, "simulated")
check("7", "garden of models: winner's error on fresh data", 1.191, t_err, 0.08, "simulated")


def t_p(a, b):
    ma, mb = st.mean(a), st.mean(b)
    se_ = math.sqrt(st.variance(a) / len(a) + st.variance(b) / len(b))
    return 2 * (1 - normal_cdf(abs(ma - mb) / se_))


def reg_p(y, group, covs):
    rows = [[1.0, g] + [c[i] for c in covs] for i, g in enumerate(group)]
    k = len(rows[0])
    xtx = [[sum(r[i] * r[j] for r in rows) for j in range(k)] for i in range(k)]
    xty = [sum(r[i] * yi for r, yi in zip(rows, y)) for i in range(k)]
    beta = solve(xtx, xty)
    res = [yi - sum(b * x for b, x in zip(beta, r)) for r, yi in zip(rows, y)]
    s2 = sum(e * e for e in res) / (len(y) - k)
    inv11 = solve(xtx, [0.0, 1.0] + [0.0] * (k - 2))[1]
    return 2 * (1 - normal_cdf(abs(beta[1]) / math.sqrt(s2 * inv11)))


def garden_of_paths(rng: random.Random, experiments: int):
    n = 40
    small = large = pre = 0
    for _ in range(experiments):
        a = [rng.gauss(0, 1) for _ in range(2 * n)]
        bb = [rng.gauss(0, 1) for _ in range(2 * n)]
        c1 = [rng.gauss(0, 1) for _ in range(2 * n)]
        c2 = [rng.gauss(0, 1) for _ in range(2 * n)]
        c3 = [rng.gauss(0, 1) for _ in range(2 * n)]
        sex = [rng.random() < 0.5 for _ in range(2 * n)]
        dv1 = [a[i] + 0.4 * c1[i] for i in range(2 * n)]
        dv2 = [0.7 * a[i] + 0.7 * bb[i] + 0.4 * c2[i] for i in range(2 * n)]
        grp = [0.0] * n + [1.0] * n

        def sub(v, m):
            return v[:m] + v[n:n + m]

        def analyse(y, m, covs):
            return reg_p(sub(y, m), sub(grp, m), [sub(c, m) for c in covs])

        def exclude(y, m):
            treated = y[n:n + m]
            drop = treated.index(max(treated))
            return t_p(y[:m], [v for i, v in enumerate(treated) if i != drop])

        pre += t_p(dv1[:30], dv1[n:n + 30]) < 0.05
        p_small = 1.0
        for y in (dv1, dv2):
            for m in (20, 30):
                p_small = min(p_small, t_p(y[:m], y[n:n + m]), analyse(y, m, [c1]))
            p_small = min(p_small, exclude(y, 30))
        small += p_small < 0.05
        p_large = 1.0
        for y in (dv1, dv2):
            for m in (10, 20, 30, 40):
                p = t_p(y[:m], y[n:n + m])
                p_large = min(p_large, p)
                if p < 0.05:
                    break
            for m in (20, 30, 40):
                for covs in ([c1], [c1, c2], [c1, c2, c3]):
                    p_large = min(p_large, analyse(y, m, covs))
                p_large = min(p_large, exclude(y, m))
            for target in (False, True):
                g0 = [y[i] for i in range(n) if sex[i] == target]
                g1 = [y[i] for i in range(n, 2 * n) if sex[i] == target]
                if len(g0) > 3 and len(g1) > 3:
                    p_large = min(p_large, t_p(g0, g1))
        large += p_large < 0.05
    return 100 * small / experiments, 100 * large / experiments, 100 * pre / experiments


exps = 1500
small, large, pre = garden_of_paths(random.Random(7), exps)
se_p = lambda p: 100 * math.sqrt(p / 100 * (1 - p / 100) / exps)  # noqa: E731
check("7", "garden of paths: pre-registered test (%)", 5.2, pre, 4 * se_p(5.2), "simulated")
check("7", "garden of paths: Simmons' four freedoms (%)", 20.3, small, 4 * se_p(20.3), "simulated")
check("7", "garden of paths: the large family (%)", 42.3, large, 4 * se_p(42.3), "simulated")

# ------------------------------------------------------------------ Chapter 8: identities among the printed numbers
base = 0.243
unc = base * (1 - base)
check("8", "UNC = ybar (1 - ybar)", 0.1840, unc, 2e-4)
check("8", "lazy: REL = (0.20 - ybar)^2", 0.0018, (0.20 - base) ** 2, 1e-4)
check("8", "lazy: Brier = REL + UNC", 0.1858, (0.20 - base) ** 2 + unc, 2e-4)
check("8", "lazy: ECE = |0.20 - ybar|", 0.0430, abs(0.20 - base), 5e-4)
for name, brier, rel, res in [("expert", 0.1569, 0.0012, 0.0267), ("overconfident", 0.1712, 0.0150, 0.0275),
                              ("noisy", 0.2119, 0.0363, 0.0083)]:
    # The identity closes on the forecasts pooled into boxes; the pooling costs a few thousandths.
    check("8", f"{name}: REL - RES + UNC against the raw Brier", brier, rel - res + 0.1840, 0.002)
check("8", "expert: pooled Brier = raw Brier + pooling cost", 0.1584, 0.1569462 + 1.49e-3, 1e-4)
for vault, ece, q95, passes in [("NY flights 2013", 0.0267, 0.0046, 1), ("cyclones", 0.0123, 0.0087, 1),
                                ("Toronto 1997-2002", 0.0363, 0.0320, 0), ("Toronto 2020-2021", 0.0668, 0.0115, 0),
                                ("NY flights 2023", 0.0079, 0.0035, 1), ("Rhode Island", 0.0048, 0.0034, 1)]:
    check("8", f"{vault}: fixed tolerance 0.03 verdict (1 passes)", passes, int(ece <= 0.03), 0)
    check("8", f"{vault}: detectable against the null q95 (1 if so)", 1, int(ece > q95), 0)


# the four forecasters and the null ECE, from the recipe of Step 3 (since 2026-10-19)
def decompose(p, y):
    n = len(p)
    base = sum(y) / n
    order = sorted(range(n), key=p.__getitem__)
    bounds, start = [], 0
    for k in range(1, 11):
        end = max(k * n // 10, start)
        while 0 < end < n and p[order[end]] == p[order[end - 1]]:
            end += 1
        if end > start:
            bounds.append((start, end))
            start = end
    rel = res = ece = 0.0
    for s, e in bounds:
        idx = order[s:e]
        pm = sum(p[i] for i in idx) / len(idx)
        ym = sum(y[i] for i in idx) / len(idx)
        rel += len(idx) / n * (pm - ym) ** 2
        res += len(idx) / n * (ym - base) ** 2
        ece += len(idx) / n * abs(pm - ym)
    brier = sum((a - b) ** 2 for a, b in zip(p, y)) / n
    return brier, rel, res, base * (1 - base), ece


logit = lambda q: math.log(min(max(q, 1e-6), 1 - 1e-6) / (1 - min(max(q, 1e-6), 1 - 1e-6)))  # noqa: E731


def platt(p, y):
    raw = [logit(q) for q in p]
    if max(raw) - min(raw) < 1e-9:
        return logit(sum(y) / len(y)), 0.0
    # standardised for Newton's sake, mapped back at the end
    m, s = st.mean(raw), st.pstdev(raw)
    xs = [(x - m) / s for x in raw]
    a, b = 0.0, 1.0
    for _ in range(60):
        g0 = g1 = h00 = h01 = h11 = 0.0
        for x, yi in zip(xs, y):
            q = sigmoid(a + b * x)
            w = q * (1 - q)
            g0 += yi - q
            g1 += (yi - q) * x
            h00 += w
            h01 += w * x
            h11 += w * x * x
        det = h00 * h11 - h01 * h01
        da, db = (h11 * g0 - h01 * g1) / det, (h00 * g1 - h01 * g0) / det
        a, b = a + da, b + db
        if abs(da) + abs(db) < 1e-10:
            break
    return a - b * m / s, b / s


rng = random.Random(2026)
days = 4000
fc = {"lazy": [], "expert": [], "overconfident": [], "noisy": []}
y = []
for _ in range(days):
    z = -1.386 + 1.1 * rng.gauss(0, 1)
    y.append(1.0 if rng.random() < sigmoid(z) else 0.0)
    fc["lazy"].append(0.20)
    fc["expert"].append(sigmoid(z + 0.6 * rng.gauss(0, 1)))
    fc["overconfident"].append(sigmoid(2.2 * (z + 0.6 * rng.gauss(0, 1))))
    fc["noisy"].append(sigmoid(z + 1.6 * rng.gauss(0, 1)))
check("8", "observed base rate of the 4,000 days", 0.243, sum(y) / days, 0.02, "simulated")
half = days // 2
for name, b_brier, b_after in [("lazy", 0.1858, 0.1817), ("expert", 0.1569, 0.1547), ("overconfident", 0.1712, 0.1555),
                               ("noisy", 0.2119, 0.1741)]:
    p = fc[name]
    brier = decompose(p, y)[0]
    a, b = platt(p[:half], y[:half])
    after = decompose([sigmoid(a + b * logit(q)) for q in p[half:]], y[half:])[0]
    # The overconfident forecaster's Brier is the most seed-sensitive: the book's own instrument, run with seeds
    # 2022 to 2025 and 2027 instead of 2026, gives 0.1706 to 0.1914 (2026-10-19), so its tolerance is that spread.
    check("8", f"{name}: Brier", b_brier, brier, 0.022 if name == "overconfident" else 0.012, "simulated")
    check("8", f"{name}: Brier out of the fit after Platt", b_after, after, 0.012, "simulated")
check("8", "lazy: RES of a constant forecaster", 0.0, decompose(fc["lazy"], y)[2], 1e-12)


def null_ece(rate, spread, n, sims, rng):
    p = [sigmoid(logit(rate) + spread * rng.gauss(0, 1)) for _ in range(n)]
    eces = sorted(decompose(p, [1.0 if rng.random() < q else 0.0 for q in p])[4] for _ in range(sims))
    return eces[len(eces) // 2]


rng = random.Random(20261015)
for name, rate, spread, n, sims, b_q50 in [("Toronto 1997-2002", 0.50, 1.0, 1488, 300, 0.0292),
                                           ("Rhode Island", 0.0226, 1.6, 24466, 40, 0.0028)]:
    check("8", f"null ECE median, {name}", b_q50, null_ece(rate, spread, n, sims, rng), 0.15 * b_q50, "simulated")
