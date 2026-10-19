"""Chapter 13: leverage on the Dow Jones and the LaLonde decomposition, from the text and the data files.

Every number here is a deterministic account on data/djia_1885_2023.csv, data/nsw_experimental.csv and
data/cps_observational.csv, except the claim world, which resamples years with Python's own generator.

Choices the text fixes: simple daily returns of the closing value; training 1885 to 1999; "per year" as the
number of sessions per year of the training set (31,919 sessions in 115 years); constant leverage rebalanced each
session, wealth multiplied by 1 + L r; employment as positive income.

Choices the text leaves implicit, recovered by trying the readings it allows and declared here:
- A window "from day A" starts at the close of the first session of A, so the move of that session is not held;
  "to day B" ends at the close of the session of B (the peak to trough of 1929 to 1932, and the calendar years).
- A 20-year cohort ends at the first session on or after the same date 20 years later, and is kept only if that
  session is inside the training set. The published material counted 1,135 cohorts, the instrument 1,134; the
  fractions differ in the third decimal between the two conventions, which is the tolerance used.
- The 95% intervals of Step 2 are Wald intervals for a difference of proportions.

The propensity fit follows the recipe that Step 2 now states (since 2026-10-19). The posterior interval of Kelly,
the skill in the vault and the fairness rates stay open: the chapter cites them from the research record, with the
commit where their code lives.
"""

from __future__ import annotations

import bisect
import csv
import math
import random
import statistics as st
from pathlib import Path

from common import check, open_choice

ROOT = Path(__file__).resolve().parent.parent.parent

with open(ROOT / "data" / "djia_1885_2023.csv", newline="") as fh:
    prices = [(r["date"], float(r["value"])) for r in csv.DictReader(fh)]
returns = [(prices[i][0], prices[i][1] / prices[i - 1][1] - 1) for i in range(1, len(prices))]
train = [(d, r) for d, r in returns if d < "2000-01-01"]
years = sorted({d[:4] for d, _ in train})
per_year = len(train) / len(years)
mu = st.mean(r for _, r in train)
sigma = st.pstdev(r for _, r in train)

check("13", "sessions in the whole series", 37930, len(returns), 0)
check("13", "sessions in the training set", 31919, len(train), 0)
check("13", "daily arithmetic mean", 0.000240669, mu, 5e-10)
check("13", "arithmetic mean per year", 0.0668, mu * per_year, 5e-5)
check("13", "daily sd", 0.010496, sigma, 5e-7)
check("13", "sd per year", 0.1749, sigma * math.sqrt(per_year), 5e-5)

growth = {L: st.mean(math.log(1 + L * r) for _, r in train) * per_year for L in (1, 2, 3)}
for L, g_book, drag_book, pred_book in [(1, 0.0515, 0.0153, 0.0153), (2, 0.0719, 0.0617, 0.0612),
                                        (3, 0.0601, 0.1403, 0.1376)]:
    check("13", f"time growth per year at {L}x", g_book, growth[L], 5e-5)
    check("13", f"observed drag per year at {L}x", drag_book, L * mu * per_year - growth[L], 5e-5)
    check("13", f"drag predicted by L^2 sigma^2 / 2 at {L}x", pred_book, L * L * sigma**2 / 2 * per_year, 5e-5)

worst_day, worst = min(train, key=lambda x: x[1])
check("13", "worst session (return)", -0.2261, worst, 5e-5)
check("13", "worst session is 1987-10-19 (1 if so)", 1, int(worst_day == "1987-10-19"), 0)
check("13", "loss of the worst session at 3x", 0.678, 3 * abs(worst), 5e-4)
check("13", "daily ruin leverage 1/|min r|", 4.42, 1 / abs(worst), 5e-3)


def held(first: str, last: str, L: float) -> float:
    """Wealth from the close of the first session on or after `first` to the close of `last`."""
    dates = [d for d, _ in returns]
    start = bisect.bisect_left(dates, first)
    w = 1.0
    for d, r in returns[start + 1:]:
        if d > last:
            break
        w *= 1 + L * r
    return w


check("13", "calendar 1929-1932 at 1x (capital left)", 0.19521, held("1929-01-01", "1932-12-31", 1), 5e-6)
check("13", "calendar 1929-1932 at 3x (capital left)", 0.00088, held("1929-01-01", "1932-12-31", 3), 5e-6)
check("13", "peak to trough 1929-09-03 to 1932-07-08 at 1x", 0.10814, held("1929-09-03", "1932-07-08", 1), 5e-6)
check("13", "peak to trough 1929-09-03 to 1932-07-08 at 3x", 0.00025, held("1929-09-03", "1932-07-08", 3), 5e-6)

kelly = max((st.mean(math.log(1 + L * r) for _, r in train), L) for L in [i / 100 for i in range(100, 400)])[1]
check("13", "empirical Kelly leverage of the series", 2.14, kelly, 0.005)

# 20-year cohorts on the first session of each month
dates = [d for d, _ in train]
firsts, seen = [], set()
for i, d in enumerate(dates):
    if d[:7] not in seen:
        seen.add(d[:7])
        firsts.append(i)


def cohorts(L: float) -> list[float]:
    out = []
    for i in firsts:
        d = dates[i]
        j = bisect.bisect_left(dates, f"{int(d[:4]) + 20}{d[4:]}")
        if j >= len(dates):
            continue
        w = 1.0
        for k in range(i + 1, j + 1):
            w *= 1 + L * train[k][1]
        out.append(w)
    return out


for L, n_book, med, mean, above in [(1, 1134, 2.13, 2.9, 0.9577), (2, 1134, 2.67, 7.8, 0.7848),
                                     (3, 1134, 2.10, 19.1, 0.6429)]:
    c = cohorts(L)
    check("13", f"cohorts at {L}x: count", n_book, len(c), 1)
    check("13", f"cohorts at {L}x: median wealth", med, st.median(c), 0.02)
    check("13", f"cohorts at {L}x: mean wealth", mean, st.mean(c), 0.15)
    check("13", f"cohorts at {L}x: share above the start", above, sum(x > 1 for x in c) / len(c), 0.003)

# cohorts per unit-year and the claim world
by_year: dict[str, list[float]] = {}
for d, r in train:
    by_year.setdefault(d[:4], []).append(r)


def above_share(annual: list[float]) -> float:
    sums = [sum(annual[i:i + 20]) for i in range(len(annual) - 19)]
    return sum(s > 0 for s in sums) / len(sums)


for L, book in [(1, 0.9583), (3, 0.6458)]:
    check("13", f"unit-year cohorts above the start at {L}x", book,
          above_share([sum(math.log(1 + L * r) for r in v) for v in by_year.values()]), 5e-5)
arith3 = [3 * sum(v) for v in by_year.values()]
rng = random.Random(1885)
shares = sorted(above_share([rng.choice(arith3) for _ in arith3]) for _ in range(2000))
check("13", "claim world: mean share above the start", 0.9286, st.mean(shares), 0.01, "simulated")
check("13", "claim world: 1% quantile", 0.6354, shares[20], 0.03, "simulated")
check("13", "claim world: p(share <= 0.6458)", 0.0140, sum(s <= 0.6458 for s in shares) / len(shares), 0.008,
      "simulated")

# LaLonde
with open(ROOT / "data" / "nsw_experimental.csv", newline="") as fh:
    nsw = list(csv.DictReader(fh))
with open(ROOT / "data" / "cps_observational.csv", newline="") as fh:
    cps = list(csv.DictReader(fh))
treated = [r for r in nsw if r["treat"] == "1"]
controls = [r for r in nsw if r["treat"] == "0"]
emp = lambda g, k: st.mean(float(r[k]) > 0 for r in g)  # noqa: E731
inc = lambda g, k: st.mean(float(r[k]) for r in g)  # noqa: E731

check("13", "NSW treated", 185, len(treated), 0)
check("13", "NSW randomised controls", 260, len(controls), 0)
check("13", "CPS observations", 15992, len(cps), 0)
for name, group, b_emp, b_inc in [("NSW treated", treated, 0.757, 6349), ("NSW controls", controls, 0.646, 4555),
                                  ("CPS", cps, 0.864, 14847)]:
    check("13", f"employment 1978, {name}", b_emp, emp(group, "re78"), 5e-4)
    check("13", f"mean income 1978, {name} (US$)", b_inc, inc(group, "re78"), 0.5)
experiment = emp(treated, "re78") - emp(controls, "re78")
raw = emp(treated, "re78") - emp(cps, "re78")
check("13", "experiment: employment difference", 0.111, experiment, 5e-4)
check("13", "experiment: income difference (US$)", 1794, inc(treated, "re78") - inc(controls, "re78"), 0.5)
check("13", "raw comparison: employment difference", -0.107, raw, 5e-4)
check("13", "raw comparison: income difference (US$)", -8498, inc(treated, "re78") - inc(cps, "re78"), 0.5)
check("13", "implied selection bias: employment", -0.218, raw - experiment, 5e-4)
check("13", "implied selection bias: income (US$)", -10292,
      (inc(treated, "re78") - inc(cps, "re78")) - (inc(treated, "re78") - inc(controls, "re78")), 0.5)
for name, group, book in [("NSW treated", treated, 0.400), ("NSW controls", controls, 0.315), ("CPS", cps, 0.891)]:
    check("13", f"1975 placebo employment, {name}", book, emp(group, "re75"), 5e-4)
check("13", "1975 placebo raw difference treated - CPS", -0.491, emp(treated, "re75") - emp(cps, "re75"), 5e-4)


def wald(a: list, b: list, key: str) -> tuple[float, float, float]:
    p1, p2 = emp(a, key), emp(b, key)
    se = math.sqrt(p1 * (1 - p1) / len(a) + p2 * (1 - p2) / len(b))
    return p1 - p2, p1 - p2 - 1.96 * se, p1 - p2 + 1.96 * se


for what, (est, lo, hi), book in [
    ("raw comparison", wald(treated, cps, "re78"), (-10.7, -17.0, -4.5)),
    ("randomised experiment", wald(treated, controls, "re78"), (11.1, 2.6, 19.6)),
    ("1975 placebo, experiment", wald(treated, controls, "re75"), (8.5, -0.6, 17.5)),
]:
    check("13", f"{what}: estimate (points)", book[0], 100 * est, 0.05)
    check("13", f"{what}: 95% interval, lower (points)", book[1], 100 * lo, 0.1)
    check("13", f"{what}: 95% interval, upper (points)", book[2], 100 * hi, 0.1)

# ------------------------------------------------------------------ the propensity fit, from the recipe of Step 2
from common import beta_from_quantiles, logistic_irls, normal_cdf, sigmoid  # noqa: E402


def covariates(r: dict, placebo: bool = False) -> list[float]:
    a, e = float(r["age"]), float(r["educ"])
    r74, r75 = float(r["re74"]) / 1000, float(r["re75"]) / 1000
    x = [1.0, (a - 30) / 10, ((a - 30) / 10) ** 2, (e - 11) / 3, ((e - 11) / 3) ** 2, float(r["black"]), float(r["hisp"]),
         float(r["marr"]), float(r["nodegree"]), r74 / 10, 1.0 if r74 == 0 else 0.0, (r74 / 10) ** 2]
    if not placebo:
        x += [r75 / 10, 1.0 if r75 == 0 else 0.0, (r75 / 10) ** 2]
    return x


sample = treated + cps
t_obs = [float(r["treat"]) for r in sample]
X = [covariates(r) for r in sample]
beta = logistic_irls(X, t_obs, ridge=1e-3)
score = [sigmoid(sum(b * x for b, x in zip(beta, row))) for row in X]
w = [e / (1 - e) for e in score]
ctrl = [i for i, ti in enumerate(t_obs) if ti == 0]
trt = [i for i, ti in enumerate(t_obs) if ti == 1]
ess = sum(w[i] for i in ctrl) ** 2 / sum(w[i] ** 2 for i in ctrl)
check("13", "propensity fit: effective controls (Kish)", 103, ess, 0.6)
check("13", "propensity fit: CPS below the smallest treated score (%)", 72,
      100 * sum(score[i] < min(score[j] for j in trt) for i in ctrl) / len(ctrl), 0.6)
y78 = [1.0 if float(r["re78"]) > 0 else 0.0 for r in sample]
weighted = lambda y, idx: sum(w[i] * y[i] for i in idx) / sum(w[i] for i in idx)  # noqa: E731
att_ipw = st.mean(y78[i] for i in trt) - weighted(y78, ctrl)
check("13", "propensity fit: weighted point estimate (points)", 9.0, 100 * att_ipw, 0.06)

# the four-cell posterior, standardised to the treated's racial composition (156 of 185 black)
prior_a, prior_b = beta_from_quantiles({0.25: 0.43333, 0.5: 0.53333, 0.75: 0.63333})
cells = {}
for arm in (0, 1):
    for black in (1, 0):
        idx = [i for i in range(len(sample)) if t_obs[i] == arm and float(sample[i]["black"]) == black]
        if arm == 1:
            k, n = sum(y78[i] for i in idx), len(idx)
        else:
            n = sum(w[i] for i in idx) ** 2 / sum(w[i] ** 2 for i in idx)
            k = n * weighted(y78, idx)
        a0, b0 = (prior_a, prior_b) if (arm, black) == (0, 1) else (1.0, 1.0)
        cells[(arm, black)] = (k + a0, n - k + b0)
rng_post = random.Random(1986)
w_black = 156 / 185
draws = sorted(w_black * (rng_post.betavariate(*cells[(1, 1)]) - rng_post.betavariate(*cells[(0, 1)]))
               + (1 - w_black) * (rng_post.betavariate(*cells[(1, 0)]) - rng_post.betavariate(*cells[(0, 0)]))
               for _ in range(40000))
check("13", "propensity fit: posterior mean of the effect (points)", 9.6, 100 * st.mean(draws), 0.25, "simulated")
check("13", "propensity fit: 95% interval, lower (points)", -0.6, 100 * draws[1000], 0.4, "simulated")
check("13", "propensity fit: 95% interval, upper (points)", 20.8, 100 * draws[39000], 0.9, "simulated")

# a treatment with no effect, run through the same weighting (normal approximation of its replicas)
outcome_beta = logistic_irls([X[i] for i in ctrl], [y78[i] for i in ctrl], ridge=1e-3)
p_null = [sigmoid(sum(b * x for b, x in zip(outcome_beta, row))) for row in X]
null_mean = st.mean(p_null[i] for i in trt) - weighted(p_null, ctrl)
w_ctrl = sum(w[i] for i in ctrl)
null_sd = math.sqrt(sum(p_null[i] * (1 - p_null[i]) for i in trt) / len(trt) ** 2
                    + sum(w[i] ** 2 * p_null[i] * (1 - p_null[i]) for i in ctrl) / w_ctrl ** 2)
check("13", "inert treatment: centre of the band (points)", 0.9, 100 * null_mean, 0.1)
check("13", "inert treatment: band, lower (points)", -10.3, 100 * (null_mean - 1.96 * null_sd), 0.5)
check("13", "inert treatment: band, upper (points)", 11.9, 100 * (null_mean + 1.96 * null_sd), 0.5)
check("13", "inert treatment: one-sided p of the observed", 0.08, 1 - normal_cdf((att_ipw - null_mean) / null_sd), 0.015)

# the 1975 placebo: the score without the 1975 terms, employment in 1975 as the outcome
Xp = [covariates(r, placebo=True) for r in sample]
beta_p = logistic_irls(Xp, t_obs, ridge=1e-3)
score_p = [sigmoid(sum(b * x for b, x in zip(beta_p, row))) for row in Xp]
wp = [e / (1 - e) for e in score_p]
y75 = [1.0 if float(r["re75"]) > 0 else 0.0 for r in sample]
placebo = st.mean(y75[i] for i in trt) - sum(wp[i] * y75[i] for i in ctrl) / sum(wp[i] for i in ctrl)
check("13", "propensity placebo, 1975 employment (points)", -12.3, 100 * placebo, 0.06)

missing = "recipe longer than a paragraph; cited from the research record (commit 1bfec905)"
open_choice("13", "posterior 90% interval of Kelly, upper end", 3.44, missing)
open_choice("13", "skill of the model in the vault", 0.18, missing)
open_choice("13", "false negative rate, Hispanics", 0.72, missing)
