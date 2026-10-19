"""Chapter 18: the Danish fires, the clean window and its information, from the text and data/danish_fire.csv.

Choices the text fixes: a month is "hit" when a single claim exceeds the retention of 50 M DKK; the training set is
1980 to 1988 and the vault 1989 to 1990; Hill as written, with the threshold at the (k+1)-th largest loss; Laplace
as (s+1)/(n+2); the "negligible" world at 1/1200 per month; 95% severity as the smallest number of clean months m with
(1 - p)^m <= 0.05.

Choices recovered by trying the readings the text allows: the POT tail turns into a monthly chance as
1 - exp(-rate * S(50)), with the rate of excesses above 10 M per month of the training set and S the generalised
Pareto survival above the threshold; the mutual information takes the two hypotheses as equally likely a priori.
The simulated Pareto check uses Python's generator and is compared within its sampling error.
"""

from __future__ import annotations

import collections
import csv
import math
import random
import statistics as st
from pathlib import Path

from common import check, open_choice

ROOT = Path(__file__).resolve().parent.parent.parent
with open(ROOT / "data" / "danish_fire.csv", newline="") as fh:
    claims = [(r["Date"], float(r["Loss"])) for r in csv.DictReader(fh)]

months = sorted({d[:7] for d, _ in claims})
check("18", "claims above 1 M DKK, 1980-1990", 2167, len(claims), 0)
check("18", "months in the series", 132, len(months), 0)
hit: dict[str, float] = collections.OrderedDict()
for d, x in claims:
    if x > 50:
        hit[d[:7]] = max(hit.get(d[:7], 0.0), x)
for label, first, last, n_book, big_book in [("1980-1982", "1980", "1982", 4, 263.3), ("1983-1985", "1983", "1985", 1, 57.4),
                                             ("1986-1988", "1986", "1988", 0, 47.0), ("1989-1990", "1989", "1990", 2, 152.4)]:
    in_block = lambda d: first <= d[:4] <= last  # noqa: E731
    check("18", f"months hit, {label}", n_book, sum(in_block(m) for m in hit), 0)
    check("18", f"largest loss, {label}", big_book, max(x for d, x in claims if in_block(d)), 0.05)

train = sorted((x for d, x in claims if d < "1989"), reverse=True)
check("18", "claims in the training set", 1714, len(train), 0)


def hill(xs_desc: list[float], k: int) -> float:
    return 1 / (sum(math.log(xs_desc[i] / xs_desc[k]) for i in range(k)) / k)


for k, book in [(25, 1.9625), (50, 1.8109), (100, 1.4100)]:
    check("18", f"Hill on the training set, k = {k}", book, hill(train, k), 5e-5)

rng = random.Random(1810)
sims = []
for _ in range(400):
    xs = sorted(((1 - rng.random()) ** (-1 / 1.81) for _ in range(1500)), reverse=True)
    sims.append(hill(xs, 50))
# 0.06 covers three standard errors of the book's 200-draw mean (sd 0.265) plus those of this 400-draw mean.
check("18", "simulated Pareto 1.81, n = 1500: mean Hill k = 50", 1.900, st.mean(sims), 0.06, "simulated")
check("18", "simulated Pareto 1.81, n = 1500: sd of Hill k = 50", 0.265, st.stdev(sims), 0.03, "simulated")
check("18", "exact mean of Hill k = 50 on a Pareto 1.81 (alpha k/(k-1))", 1.900, 1.81 * 50 / 49, 0.06)

p_laplace = 1 / 38
check("18", "Laplace on 36 clean months", 0.02632, (0 + 1) / (36 + 2), 5e-6)
train_months = 108
p_freq = sum(1 for m in hit if m < "1989") / train_months
check("18", "monthly frequency in the training set (5/108)", 0.04630, p_freq, 5e-6)
xi, sigma, excesses = 0.406, 7.387, 83
survival = (1 + xi * (50 - 10) / sigma) ** (-1 / xi)
p_pot = 1 - math.exp(-excesses / train_months * survival)
# The published fit is printed to three decimals (xi = 0.406, sigma = 7.387); the tolerances of the POT rows cover
# that rounding.
check("18", "monthly p of the POT/GPD tail", 0.04296, p_pot, 1e-4)

negligible = 1 / 1200


def window(p: float):
    clean_dangerous = (1 - p) ** 36
    clean_negligible = (1 - negligible) ** 36
    ratio = clean_negligible / clean_dangerous
    months_95 = math.ceil(math.log(0.05) / math.log(1 - p))
    joint = {("d", 1): 0.5 * clean_dangerous, ("d", 0): 0.5 * (1 - clean_dangerous),
             ("n", 1): 0.5 * clean_negligible, ("n", 0): 0.5 * (1 - clean_negligible)}
    pc = {c: joint[("d", c)] + joint[("n", c)] for c in (0, 1)}
    mi = sum(v * math.log2(v / (0.5 * pc[c])) for (h, c), v in joint.items() if v > 0)
    return clean_dangerous, ratio, math.log2(ratio), months_95, mi


for name, p, b_clean, b_ratio, b_bits, b_months, b_mi in [
    ("measured frequency", p_freq, 0.1815, 5.35, 2.42, 64, 0.5455),
    ("POT/GPD tail", p_pot, 0.2058, 4.72, 2.24, 69, 0.5147),
]:
    clean, ratio, bits, months_95, mi = window(p)
    check("18", f"{name}: P(clean window | dangerous)", b_clean, clean, 5e-4)
    check("18", f"{name}: likelihood ratio", b_ratio, ratio, 0.02)
    check("18", f"{name}: bits", b_bits, bits, 0.01)
    check("18", f"{name}: clean months for 95%", b_months, months_95, 0)
    check("18", f"{name}: mutual information (bits)", b_mi, mi, 1e-3)
check("18", "P(clean window | negligible)", 0.9704, (1 - negligible) ** 36, 5e-5)
_, ratio_vault, bits_vault, _, _ = window(1 / 12)
check("18", "vault rate 1/12: bits of the clean window", 4.48, bits_vault, 0.01)
check("18", "vault rate 1/12: likelihood ratio", 22.3, ratio_vault, 0.05)
check("18", "insurance seller: weeks of severe test, in years", 14, math.log(0.05) / math.log(1 - 0.004) / 52, 0.5)

open_choice("18", "trend of the claim frequency per year (%)", 4.6, "the text does not give the model of the trend")
open_choice("18", "Brier skill of the model in the vault", -0.015, "the text does not give the forecasters' recipe")
open_choice("18", "insurance seller: bits of 'I never lost'", 1.3, "the text does not give the negligible weekly rate")
