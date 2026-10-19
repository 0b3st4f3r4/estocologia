"""Chapters 2 to 5: the prosecutor's fallacy, the mammography tree, the coin that drifts, and the mean that lies.

Chapters 2 and 3 are exact accounts. Chapter 4 and the health centre of Chapter 5 are re-simulated with Python's own
generator and compared within tolerances that cover the seed.

Choices the text fixes and this module follows. Chapter 4: the bias climbs linearly from 0.2 to 0.6 over 6,400
tosses; the fixed model is the running frequency with a 95% normal interval; the model with forgetting discounts
the past by 0.99 per toss, and its interval is that of the standard error of the discounted mean (Appendix E,
Chapter 4), which this module takes with the effective sample size (sum of weights)^2 / sum of squared weights.
Chapter 5: one attendant at 12 per hour, 10-hour days that start empty, 4,000 days per world; the reinforcement
"doubles the capacity of the busy day" (Exercise 6), that is, one attendant at 24 per hour on the days of 11; the
R horizon staffs each plan for a utilisation of 0.8, the coarse plan at the mean demand (capacity 10) and the fine
plan at each day's own demand.

Choice the text does not give: the distribution of the class of 100,000 grades (open).
"""

from __future__ import annotations

import math
import random
import statistics as st

from common import check, open_choice

# ------------------------------------------------------------------ Chapter 2
for prior, b_post, b_swap in [(1e-2, 0.0001, 99.0), (1e-3, 0.0010, 998.0), (1e-6, 0.5000, 499999.7)]:
    like = 1e-6
    post = (1 - prior) * like / ((1 - prior) * like + prior * 1.0)
    check("2", f"P(innocent | match), prior guilt {prior:g}", b_post, post, 5e-5)
    check("2", f"swap factor, prior guilt {prior:g}", b_swap, post / like, 0.06)

# ------------------------------------------------------------------ Chapter 3
sick, healthy = 100, 9900
tp, fp = 0.90 * sick, 0.09 * healthy
check("3", "sick positives", 90, tp, 1e-9)
check("3", "false positives", 891, fp, 1e-9)
check("3", "P(sick | positive)", 0.0917, tp / (tp + fp), 5e-5)
check("3", "one in every ... positives", 10.9, (tp + fp) / tp, 0.05)
check("3", "second test, independent (dB)", 10.0, 10 * math.log10(0.90 / 0.09), 1e-9)
check("3", "second test, correlated ratio 4.2 (dB)", 6.23, 10 * math.log10(4.2), 5e-3)
odds = 1 / 99
check("3", "posterior after two positives, independent", 0.5025, odds * 100 / (1 + odds * 100), 5e-5)
check("3", "posterior after two positives, correlated", 0.2979, odds * 42 / (1 + odds * 42), 5e-5)

# ------------------------------------------------------------------ Chapter 4
rng = random.Random(1921)
steps, reps, disc = 6400, 200, 0.99
cover_fixed = cover_forget = total = 0
width_fixed_100, width_fixed_end, width_forget_end = [], [], []
for _ in range(reps):
    heads = 0
    s_w = s_wy = s_w2 = 0.0
    for t in range(1, steps + 1):
        bias = 0.2 + 0.4 * (t - 1) / (steps - 1)
        y = 1.0 if rng.random() < bias else 0.0
        heads += y
        p = heads / t
        half = 1.96 * math.sqrt(max(p * (1 - p), 1e-12) / t)
        s_w = disc * s_w + 1.0
        s_wy = disc * s_wy + y
        s_w2 = disc * disc * s_w2 + 1.0
        q = s_wy / s_w
        n_eff = s_w * s_w / s_w2
        half_f = 1.96 * math.sqrt(max(q * (1 - q), 1e-12) / n_eff)
        cover_fixed += abs(p - bias) <= half
        cover_forget += abs(q - bias) <= half_f
        total += 1
        if t == 100:
            width_fixed_100.append(2 * half)
    width_fixed_end.append(2 * half)
    width_forget_end.append(2 * half_f)
check("4", "coverage of the fixed model", 0.1199, cover_fixed / total, 0.02, "simulated")
check("4", "coverage of the model with forgetting", 0.9398, cover_forget / total, 0.015, "simulated")
check("4", "width of the fixed interval at step 100", 0.1564, st.mean(width_fixed_100), 0.004, "simulated")
check("4", "width of the fixed interval at step 6400", 0.0240, st.mean(width_fixed_end), 5e-4, "simulated")
check("4", "width with forgetting at step 6400", 0.1360, st.mean(width_forget_end), 0.003, "simulated")
check("4", "ratio of the widths at step 6400", 5.7, st.mean(width_forget_end) / st.mean(width_fixed_end), 0.15,
      "simulated")

# ------------------------------------------------------------------ Chapter 5
W = lambda lam: lam / (12.0 * (12.0 - lam)) * 60.0  # noqa: E731
check("5", "W(5) (min)", 3.57, W(5), 5e-3)
check("5", "W(8) (min)", 10.00, W(8), 5e-3)
check("5", "W(11) (min)", 55.00, W(11), 5e-3)
check("5", "E[W] for 5 or 11 (min)", 29.29, (W(5) + W(11)) / 2, 5e-3)
uniform_mean = sum(W(5 + 6 * (i + 0.5) / 100000) for i in range(100000)) / 100000
check("5", "E[W] for uniform 5..11 (min)", 14.46, uniform_mean, 5e-3)
second = lambda var: 0.5 * 2 / (12 - 8) ** 3 * 60 * var  # noqa: E731  f''(8) = 2 / (mu - 8)^3 hours, in minutes
check("5", "second-order toll, 5 or 11 (min)", 8.44, second(9.0), 5e-3)
check("5", "second-order toll, uniform 5..11 (min)", 2.81, second(3.0), 5e-3)


def day(rng, lam, mu, hours=10.0):
    t, free, waits = 0.0, 0.0, []
    while True:
        t += rng.expovariate(lam)
        if t > hours:
            return waits
        start = max(t, free)
        waits.append((start - t) * 60.0)
        free = start + rng.expovariate(mu)


def world(rng, draw, mu_of=lambda lam: 12.0, days=4000):
    out = []
    for _ in range(days):
        lam = draw(rng)
        out.append((lam, day(rng, lam, mu_of(lam))))
    return out


def stats(per_day):
    w = sorted(x for _, d in per_day for x in d)
    n = len(w)
    return st.mean(w), w[n // 2], w[int(0.9 * n)], 100 * sum(x > 30 for x in w) / n


rng = random.Random(1905)
worlds = {
    "8 every day": world(rng, lambda r: 8.0),
    "5 or 11": world(rng, lambda r: r.choice((5.0, 11.0))),
    "uniform 5..11": world(rng, lambda r: r.uniform(5.0, 11.0)),
    "5 or 11, surge reinforcement": world(rng, lambda r: r.choice((5.0, 11.0)), lambda lam: 24.0 if lam > 8 else 12.0),
}
for name, book in [("8 every day", (9.1, 3.7, 26.2, 7.6)), ("5 or 11", (19.6, 8.6, 56.1, 23.7)),
                   ("uniform 5..11", (13.8, 5.5, 39.8, 15.0)), ("5 or 11, surge reinforcement", (2.5, 0.0, 8.1, 0.4))]:
    mean, med, p90, over = stats(worlds[name])
    check("5", f"{name}: mean wait (min)", book[0], mean, max(0.07 * book[0], 0.25), "simulated")
    if book[1] > 0:
        check("5", f"{name}: median (min)", book[1], med, 0.12 * book[1], "simulated")
    check("5", f"{name}: p90 (min)", book[2], p90, 0.07 * book[2], "simulated")
    check("5", f"{name}: above 30 min per 100", book[3], over, 1.2, "simulated")
surge = [x for lam, d in worlds["5 or 11"] if lam > 8 for x in d]
mild = [x for lam, d in worlds["5 or 11"] if lam < 8 for x in d]
check("5", "share of patients on the surge days (%)", 68.8, 100 * len(surge) / (len(surge) + len(mild)), 0.6,
      "simulated")
check("5", "mean wait on the surge days (min)", 27.0, st.mean(surge), 1.8, "simulated")
check("5", "mean wait on the mild days (min)", 3.4, st.mean(mild), 0.3, "simulated")


def r_horizon(rng, low, high, days=2000):
    agg, fine = [], []
    for _ in range(days):
        lam = rng.choice((low, high))
        seed = rng.random()
        a = day(random.Random(seed), lam, (low + high) / 2 / 0.8)
        f = day(random.Random(seed), lam, lam / 0.8)
        agg.extend(a)
        fine.extend(f)
    return st.mean(agg), st.mean(fine), sum(x > 30 for x in agg) / len(agg), sum(x > 30 for x in fine) / len(fine)


for low, high, b_agg, b_fine, b_loss, b_o_agg, b_o_fine in [(5.0, 11.0, 43.73, 18.20, 25.54, 0.4488, 0.2081),
                                                            (7.0, 9.0, 21.15, 18.38, 2.77, 0.2518, 0.2198),
                                                            (8.0, 8.0, 18.52, 18.52, 0.00, 0.2209, 0.2209)]:
    agg, fine, o_agg, o_fine = r_horizon(random.Random(int(low * 100 + high)), low, high)
    label = f"R horizon, demand {low:g} or {high:g}"
    check("5", f"{label}: aggregate wait (min)", b_agg, agg, 0.08 * b_agg, "simulated")
    check("5", f"{label}: fine wait (min)", b_fine, fine, 0.08 * b_fine, "simulated")
    check("5", f"{label}: loss of the coarse question (min)", b_loss, agg - fine, max(0.12 * b_loss, 0.6), "simulated")
    check("5", f"{label}: share above 30 min, aggregate", b_o_agg, o_agg, 0.02, "simulated")
    check("5", f"{label}: share above 30 min, fine", b_o_fine, o_fine, 0.02, "simulated")

open_choice("5", "class of 100,000 students: share failing (%)", 45.1, "the text does not give the distribution of grades")
