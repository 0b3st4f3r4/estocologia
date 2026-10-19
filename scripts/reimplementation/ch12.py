"""Chapter 12: the health post and the bank's call centre, from the text of the book and data/bank_calls.csv.

The health post is re-simulated with Python's own generator (seed 1909), not the book's SplitMix64, so its numbers
are compared within tolerances that cover the seed: the book itself reports the oscillating world between 19.3 and
20.6 minutes across six seeds. The bank's numbers are deterministic accounts on the data file and are compared to the
printed precision.

Choices the text fixes and this module follows: Poisson arrivals at a constant rate within each 10-hour day, the day
starting empty; exponential service at 12 per hour; first come, first served; "7 or 9" and "5 or 11" drawn with equal
chance per day, "uniform between 5 and 11" as a continuous uniform rate per day; reinforcement as two nurses on the
days of 11 ("two nurses on the days of 11 patients and one on the days of 5", Step 1). The bank: half-hour slots of
the 5-minute counts, training March to June and vault July to October, Erlang C with 240 s of handling, a target of
20 s and a goal of 80%.

Choice the text did not fix, recovered by trying the readings it allows: the "margin" of the table is one standard
error (not 1.96), and the margin per day is that of a ratio of sums (the mean wait per patient is total wait over
total patients, and the days are the units).
"""

from __future__ import annotations

import collections
import csv
import datetime as dt
import math
import random
import statistics as st
from pathlib import Path

from common import check

ROOT = Path(__file__).resolve().parent.parent.parent

# ------------------------------------------------------------------ the health post


def one_day(rng: random.Random, lam: float, servers: int, hours: float = 10.0, mu: float = 12.0) -> list[float]:
    t, waits, free = 0.0, [], [0.0] * servers
    while True:
        t += rng.expovariate(lam)
        if t > hours:
            return waits
        i = min(range(servers), key=free.__getitem__)
        start = max(t, free[i])
        waits.append((start - t) * 60.0)
        free[i] = start + rng.expovariate(mu)


def world(rng, draw, servers=lambda lam: 1, days=2000):
    out = []
    for _ in range(days):
        lam = draw(rng)
        out.append(one_day(rng, lam, servers(lam)))
    return out


def ratio_se(per_day: list[list[float]]) -> float:
    s = [sum(d) for d in per_day]
    n = [len(d) for d in per_day]
    r = sum(s) / sum(n)
    k = len(s)
    return math.sqrt(k / (k - 1) * sum((a - r * b) ** 2 for a, b in zip(s, n))) / sum(n)


def summary(per_day):
    w = sorted(x for d in per_day for x in d)
    n = len(w)
    return {
        "mean": sum(w) / n,
        "median": w[n // 2],
        "p90": w[int(0.9 * n)],
        "over30": sum(x > 30 for x in w) / n,
        "margin_day": ratio_se(per_day),
        "margin_patient": st.pstdev(w) / math.sqrt(n),
    }


rng = random.Random(1909)
WORLDS = [
    ("8 every day", lambda r: 8.0, None, (9.1, 3.7, 26.1, 0.08, 0.14, 0.03)),
    ("7 or 9", lambda r: r.choice((7.0, 9.0)), None, (10.1, 4.0, 29.1, 0.09, 0.20, 0.04)),
    ("uniform between 5 and 11", lambda r: r.uniform(5.0, 11.0), None, (12.8, 5.0, 37.1, 0.14, 0.32, 0.05)),
    ("5 or 11", lambda r: r.choice((5.0, 11.0)), None, (19.3, 8.1, 55.8, 0.24, 0.51, 0.07)),
    ("5 or 11, two nurses on the days of 11", lambda r: r.choice((5.0, 11.0)), lambda lam: 2 if lam > 8 else 1,
     (1.9, 0.0, 6.8, 0.0, 0.04, 0.01)),
]
results = {}
for name, draw, servers, book in WORLDS:
    s = summary(world(rng, draw, servers or (lambda lam: 1)))
    results[name] = s
    mean, median, p90, over30, mday, mpat = book
    check("12", f"post, {name}: mean wait (min)", mean, s["mean"], max(0.08 * mean, 0.3), "simulated")
    if median > 0:
        check("12", f"post, {name}: median wait (min)", median, s["median"], 0.12 * median, "simulated")
    check("12", f"post, {name}: 90th percentile (min)", p90, s["p90"], 0.08 * p90, "simulated")
    check("12", f"post, {name}: share above 30 min", over30, s["over30"], 0.015, "simulated")
    check("12", f"post, {name}: margin per day (1 se)", mday, s["margin_day"], max(0.15 * mday, 0.01), "simulated")
    check("12", f"post, {name}: margin per patient (1 se)", mpat, s["margin_patient"], max(0.25 * mpat, 0.006),
          "simulated")

W = lambda lam: lam / (12.0 * (12.0 - lam)) * 60.0  # noqa: E731  minutes
check("12", "W(8) of the formula (min)", 10.0, W(8.0), 1e-9)
check("12", "(W(5) + W(11)) / 2 (min)", 29.3, (W(5.0) + W(11.0)) / 2, 0.05)
check("12", "W(11) of the formula (min)", 55.0, W(11.0), 1e-9)
check("12", "share of patients on the surge days", 0.70, 11 / 16, 0.02)
surge = summary(world(random.Random(11), lambda r: 11.0, days=1000))
check("12", "post, surge days only: mean wait (min)", 28.2, surge["mean"], 2.0, "simulated")
check("12", "severe test: difference between the worlds (min)", 10.2,
      results["5 or 11"]["mean"] - results["8 every day"]["mean"], 1.2, "simulated")

# halves null: 4000 days of demand 8, 2000 random splits into two halves of 2000 days
null_rng = random.Random(2026)
days8 = world(null_rng, lambda r: 8.0, days=4000)
sums = [sum(d) for d in days8]
counts = [len(d) for d in days8]
diffs = []
for _ in range(2000):
    idx = list(range(4000))
    null_rng.shuffle(idx)
    a, b = idx[:2000], idx[2000:]
    diffs.append(sum(sums[i] for i in a) / sum(counts[i] for i in a) - sum(sums[i] for i in b) / sum(counts[i] for i in b))
all_waits = [x for d in days8 for x in d]
half_a, half_b = days8[:2000], days8[2000:]
pred_day = math.hypot(ratio_se(half_a), ratio_se(half_b))
pred_pat = 2 * st.pstdev(all_waits) / math.sqrt(len(all_waits))
check("12", "halves null: measured sd of the differences (min)", 0.2062, st.stdev(diffs), 0.03, "simulated")
check("12", "halves null: predicted by the day unit (min)", 0.1928, pred_day, 0.03, "simulated")
check("12", "halves null: predicted by the patient unit (min)", 0.0473, pred_pat, 0.005, "simulated")
check("12", "halves null: max |difference| in 2000 splits (min)", 0.664, max(abs(x) for x in diffs), 0.15, "simulated")

# ------------------------------------------------------------------ the bank

AHT, TARGET, SLOT, GOAL = 240.0, 20.0, 1800.0, 0.80


def load(cut_2100: bool):
    days: dict[str, collections.Counter] = collections.OrderedDict()
    with open(ROOT / "data" / "bank_calls.csv", newline="") as fh:
        for row in csv.DictReader(fh):
            day, t = row["DateTime"][:10], row["DateTime"][11:16]
            if cut_2100 and t == "21:00":
                continue
            key = f"{t[:2]}:{(int(t[3:]) // 30) * 30:02d}"
            days.setdefault(day, collections.Counter())[key] += int(row["Calls"])
    training = [d for d in days if d[5:7] in ("03", "04", "05", "06")]
    vault = [d for d in days if d not in training]
    return days, training, vault


def erlang_c(c: int, a: float) -> float:
    b = 1.0
    for k in range(1, c + 1):  # Erlang B recursion, stable at large loads
        b = a * b / (k + a * b)
    return c * b / (c - a * (1 - b))


def service_level(c: int, calls: float, length: float = SLOT) -> float:
    a = calls / length * AHT
    if calls <= 0:
        return 1.0
    if c <= a:
        return 0.0
    return 1 - erlang_c(c, a) * math.exp(-(c - a) * TARGET / AHT)


def staff(calls: float, length: float = SLOT) -> int:
    c = 1
    while service_level(c, calls, length) < GOAL:
        c += 1
    return c


days, training, vault = load(cut_2100=True)
slots = sorted(days[training[0]])
totals = lambda ks: [sum(days[k].values()) for k in ks]  # noqa: E731
v_tot, t_tot = totals(vault), totals(training)
check("12", "bank: training days", 83, len(training))
check("12", "bank: vault days", 81, len(vault))
check("12", "bank: calls in the vault (21:00 cut)", 2649354, sum(v_tot), 0)
check("12", "bank: Var/mean between days, vault", 246.7, st.variance(v_tot) / st.mean(v_tot), 0.05)
check("12", "bank: Var/mean between days, training", 272.6, st.variance(t_tot) / st.mean(t_tot), 0.05)
check("12", "bank: Var/mean between days, all", 261.1, st.variance(t_tot + v_tot) / st.mean(t_tot + v_tot), 0.05)
check("12", "bank: deviation between vault days", 2841, st.stdev(v_tot), 0.5)
check("12", "bank: Poisson deviation sqrt(mean)", 181, math.sqrt(st.mean(v_tot)), 0.5)

mean_profile = {s: st.mean(days[d][s] for d in training) for s in slots}
roster = {s: staff(mean_profile[s]) for s in slots}
check("12", "bank: peak of the mean-profile roster", 234, max(roster.values()), 0)
check("12", "bank: agent-hours per day, mean profile (21:00 cut)", 2259, sum(roster.values()) * SLOT / 3600, 0.5)
weight = sum(mean_profile.values())
check("12", "bank: service level on the mean profile", 0.8193,
      sum(mean_profile[s] * service_level(roster[s], mean_profile[s]) for s in slots) / weight, 5e-5)
levels = [sum(days[d][s] * service_level(roster[s], days[d][s]) for s in slots) / sum(days[d].values()) for d in vault]
check("12", "bank: mean service level of the vault days", 0.5683, st.mean(levels), 5e-5)
check("12", "bank: share of vault days below the goal", 0.6543, sum(x < GOAL for x in levels) / len(levels), 5e-5)

# halves null on the bank: the sd of the difference between the means of two random halves of the vault days
bank_rng = random.Random(1909)
half = len(levels) // 2
bank_diffs = []
for _ in range(20000):
    shuffled = levels[:]
    bank_rng.shuffle(shuffled)
    bank_diffs.append(st.mean(shuffled[:half]) - st.mean(shuffled[half:]))
calls_total = sum(v_tot)
p = st.mean(levels)
check("12", "bank: halves null per day (sd of the difference)", 0.0743, st.stdev(bank_diffs), 0.006, "simulated")
check("12", "bank: halves null per call (binomial)", 0.0006, math.sqrt(2 * p * (1 - p) / (calls_total / 2)), 5e-5)

# the four staffing rules, with the 21:00 interval as a 5-minute slot (the queue instrument keeps it)
days_q, training_q, vault_q = load(cut_2100=False)
slots_q = sorted(days_q[training_q[0]])
length = lambda s: 300.0 if s == "21:00" else SLOT  # noqa: E731
weekday = lambda d: dt.date.fromisoformat(d).weekday()  # noqa: E731
agent_hours = lambda r: sum(r[s] * length(s) for s in slots_q) / 3600  # noqa: E731
r0 = {s: staff(st.mean(days_q[d][s] for d in training_q), length(s)) for s in slots_q}
by_weekday = collections.defaultdict(list)
for d in training_q:
    by_weekday[weekday(d)].append(d)
profile = {w: {s: st.mean(days_q[d][s] for d in ds) for s in slots_q} for w, ds in by_weekday.items()}
residual = [sum(days_q[d].values()) / sum(profile[weekday(d)].values()) for d in training_q]
cv = st.stdev(residual)
margin = 1 + 1.2816 * cv
r1 = {w: {s: staff(profile[w][s], length(s)) for s in slots_q} for w in profile}
r2 = {w: {s: staff(profile[w][s] * margin, length(s)) for s in slots_q} for w in profile}
r3 = {s: r0[s] + 12 for s in slots_q}
vault_mean = lambda rule: st.mean(agent_hours(rule[weekday(d)]) for d in vault_q)  # noqa: E731
peak = lambda rule: max(max(v.values()) for v in rule.values())  # noqa: E731
check("12", "bank: calls in the vault (21:00 kept)", 2654950, sum(sum(days_q[d].values()) for d in vault_q), 0)
check("12", "bank: residual CV of the day", 0.0598, cv, 5e-5)
check("12", "bank: margin of R2", 1.0767, margin, 5e-5)
check("12", "bank: R0 agent-hours per day", 2264, agent_hours(r0), 0.5)
check("12", "bank: R1 agent-hours per day", 2261, vault_mean(r1), 0.5)
check("12", "bank: R2 agent-hours per day", 2428, vault_mean(r2), 0.5)
check("12", "bank: R3 agent-hours per day", 2433, agent_hours(r3), 0.5)
check("12", "bank: R0 peak", 234, max(r0.values()), 0)
check("12", "bank: R1 peak", 267, peak(r1), 0)
check("12", "bank: R2 peak", 287, peak(r2), 0)
check("12", "bank: R3 peak", 246, max(r3.values()), 0)
