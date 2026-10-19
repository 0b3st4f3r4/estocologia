"""Chapters 6 to 8: the filter, the ghosts in the noise, and the forecaster's thermometer.

Exact accounts where the chapter states one, and small simulations with Python's own generator where it states the
process: Berkson with two independent standard normals selected on their sum above the cutoff (the selected shares
24.0%, 14.4% and 7.9% follow from that reading and from nothing else); a symmetric two-state chain whose persistence
is the chance of repeating the previous letter; the hot hand on sequences of three tosses (the exact null of 5/12);
100 people tossing 20 coins, with the longest run of heads.

Open, because the text names the process but not its recipe: the garden of models and the garden of paths of
Chapter 7, and the four forecasters and the null ECE of Chapter 8. For Chapter 8 the module checks the identities that
hold among the printed numbers.
"""

from __future__ import annotations

import itertools
import math
import random
import statistics as st

from common import check, open_choice

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
check("7", "Mayo severity of the garden (1 - 0.470)", 0.53, 1 - 0.470, 1e-12)
ppv = lambda power, r, alpha: power * r / (power * r + alpha)  # noqa: E731
for r, b5, bg in [(1 / 1000, 0.016, 0.002), (1 / 10, 0.615, 0.146), (1 / 3, 0.842, 0.362)]:
    check("7", f"Ioannidis PPV, R = {r:.4g}, alpha 0.05", b5, ppv(0.8, r, 0.05), 5e-4)
    # the garden alpha is printed as 47.0%; the tolerance covers its rounding.
    check("7", f"Ioannidis PPV, R = {r:.4g}, garden alpha 0.47", bg, ppv(0.8, r, 0.47), 1e-3)
missing = "the text names the experiment but not its recipe (candidates, freedoms, stopping points)"
open_choice("7", "garden of models: winners that beat zero on validation (%)", 79.5, missing)
open_choice("7", "garden of paths, Simmons' four freedoms (%)", 21.5, missing)

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
open_choice("8", "the four forecasters and their Platt fits", 0.1555, "the text does not give how the forecasts are generated")
open_choice("8", "null ECE q95 with n = 1,488", 0.0425, "the text does not give the distribution of the null forecasts")
