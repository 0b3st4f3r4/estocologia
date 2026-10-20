"""Chapters 15, 16, 18, 20 and 21: the accounts that the text states in full.

Chapter 15: the EVPI of the sensor, as E_state[max_act payoff] - max_act E_state[payoff] (Chapter 11's definition).
Chapter 16: the closed form of the Beta(4, 8) mean. Chapter 18: the SHA-256 test vectors. Chapter 20: the arithmetic
that ties the printed Brier scores, rates and skills together. Chapter 21: the count of the verifier's rejections.

Since 2026-10-19 the chapters state the recipes of the Hill series (12), the System 1 triage (13), the property test
(14), the hash chain (15) and the canary (20), and this module follows them: exactly where the recipe is deterministic,
within the sampling error of its own draws where it is a simulation.
"""

from __future__ import annotations

import hashlib

from common import check


def evpi(actions: list[list[float]], probs: list[float]) -> tuple[float, float, float]:
    without = max(sum(x * p for x, p in zip(row, probs)) for row in actions)
    with_info = sum(p * max(row[s] for row in actions) for s, p in enumerate(probs))
    return without, with_info, with_info - without


probs = [0.7, 0.3]
without, with_info, value = evpi([[10, -4], [0, 0]], probs)
check("15", "sensor: best act without information", 5.8, without, 1e-9)
check("15", "sensor: value with perfect information", 7.0, with_info, 1e-9)
check("15", "sensor: EVPI (act or not act)", 1.2, value, 1e-9)
check("15", "sensor: EVPI with only half the stake", 0.6, evpi([[5, -2], [0, 0]], probs)[2], 1e-9)
check("15", "useless sensor: EVPI", 0.0, evpi([[10, 10], [0, 0]], probs)[2], 1e-9)

check("16", "Beta(4, 8) mean, closed form", 0.3333, 4 / 12, 5e-5)

check("18", 'sha256("") prefix as integer', int("e3b0c44298fc1c14", 16),
      int(hashlib.sha256(b"").hexdigest()[:16], 16), 0)
check("18", 'sha256("abc") prefix as integer', int("ba7816bf8f01cfea", 16),
      int(hashlib.sha256(b"abc").hexdigest()[:16], 16), 0)

vault_rate, dev_rate, train_rate = 0.0220, 0.0222, 0.0352
clim = (dev_rate - vault_rate) ** 2 + vault_rate * (1 - vault_rate)
check("20", "raw model predicted mean over the vault rate (% above)", 60, 100 * (0.0351 / vault_rate - 1), 1.0)
check("20", "climatology at the development rate: Brier on the vault", 0.02151, clim, 3e-5)
check("20", "p(1 - p) at the development rate", 0.02171, dev_rate * (1 - dev_rate), 3e-5)
check("20", "skill of the raw model", -0.0020, 1 - 0.02155 / 0.02151, 3e-4)
check("20", "skill of the model with Platt", 0.0098, 1 - 0.02130 / 0.02151, 3e-4)

rejections = [1, 1, 1, 4, 1, 1, 2]
check("21", "cards rejected = sum of rejections per rule", 11, sum(rejections), 0)
check("21", "rules that rejected at least one card", 7, sum(r > 0 for r in rejections), 0)

# ------------------------------------------------------------------ the recipes of Step 3 (since 2026-10-19)
import math  # noqa: E402
import random  # noqa: E402
import statistics as st  # noqa: E402

from common import logistic_irls, sigmoid  # noqa: E402

# Chapter 15: Hill on 20,000 draws of a Pareto with alpha = 1.81 (this module's own draws)
rng = random.Random(1909)
series = sorted(((1 - rng.random()) ** (-1 / 1.81) for _ in range(20_000)), reverse=True)
for k, book in [(250, 1.8444), (500, 1.7901), (1000, 1.7949), (2000, 1.8032)]:
    hill = 1 / (sum(math.log(series[i] / series[k]) for i in range(k)) / k)
    # three standard errors of Hill, alpha / sqrt(k), cover the difference between two independent draws
    check("15", f"Hill of the Pareto series, k = {k}", book, hill, 3 * 1.81 / math.sqrt(k) * math.sqrt(2), "simulated")

# Chapter 16: the deterministic triage of System 1, exactly
MASK = (1 << 64) - 1


def splitmix64(state: int) -> tuple[int, int]:
    state = (state + 0x9E3779B97F4A7C15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return state, z ^ (z >> 31)


state = 0
for byte in b"h3.voos":
    state = (state * 1_099_511_628_211 + byte) & MASK
_, value = splitmix64(state)
first = (value % 99 + 1) / 100
check("16", "System 1 triage of h3.voos: first probability", 0.06, first, 1e-12)
check("16", "System 1 triage of h3.voos: second probability", 0.94, 1 - first, 1e-12)

# Chapter 17: the bounds of the property test
check("17", "property test: smallest possible margin", 0.00617, 1.96 * math.sqrt(0.01 * 0.99 / 999), 5e-6)
check("17", "property test: largest possible margin (n = 100, p = 0.5)", 0.098, 1.96 * math.sqrt(0.25 / 100), 1e-12)
rng = random.Random(2026)
margins = [1.96 * math.sqrt(p * (1 - p) / n) for n, p in
           ((rng.randrange(100, 1000), rng.randrange(1, 100) / 100) for _ in range(10_000))]
check("17", "property test: violations of [0; 0.5]", 0, sum(not 0 <= m <= 0.5 for m in margins), 0)
check("17", "property test: observed maximum near the bound", 0.09798, max(margins), 0.002, "simulated")

# Chapter 18: the chain and the Merkle root, exactly
sha = lambda s: hashlib.sha256(s.encode()).hexdigest()  # noqa: E731
events = [("h3.voos", "ECE=0.0048"), ("h3.bolsa", "mediana=2.1x"), ("h3.energia", "canario=ok"),
          ("h3.prisoes", "ECE=0.036"), ("h3.ri", "ECE=0.0048 habilidade=0.019")]
previous, hashes = "0" * 64, []
for i, (card, content) in enumerate(events):
    previous = sha(f"sessao-h3|{i}|{card}|{content}|{previous}")
    hashes.append(previous)


def merkle(hex_leaves: list[str]) -> str:
    # As the text writes it since 2026-10-20: leaf 0x00 || text, node 0x01 || left || right, lone node moves up.
    level = [hashlib.sha256(b"\x00" + h.encode()).digest() for h in hex_leaves]
    while len(level) > 1:
        level = [hashlib.sha256(b"\x01" + level[i] + level[i + 1]).digest() if i + 1 < len(level) else level[i]
                 for i in range(0, len(level), 2)]
    return level[0].hex()


check("18", "event 0 hash (first 8 hex digits as integer)", int("902e82d6", 16), int(hashes[0][:8], 16), 0)
check("18", "Merkle root of the textbook chain (first 8 hex digits as integer)", int("e81457d5", 16),
      int(merkle(hashes)[:8], 16), 0)
corpus = ["ficha=h0.posto-saude|decisao=abrir uma segunda sala", "ficha=h0.banco-2003|decisao=reforcar o atendimento",
          "ficha=h1.a-multiplicativo|decisao=reduzir a alavancagem", "ficha=h1.b-lalonde|decisao=manter o programa",
          "ficha=h1.c-incendios|decisao=triplicar a aposta", "ficha=h3.voos23|decisao=manter o modelo",
          "ficha=h3.prisoes|decisao=trocar o cofre"]
leaves = [sha(c) for c in corpus]
check("18", "Merkle root of the batch of 7 cards (first 8 hex digits as integer)", int("ed9aa4bb", 16),
      int(merkle(leaves)[:8], 16), 0)
check("18", "repeating the last card changes the root (1 if so)", 1,
      int(merkle(leaves) != merkle(leaves + leaves[-1:])), 0)

# Chapter 20: the canary, with the covariates grouped into cells (they are binary)
rng = random.Random(2021)


def period(n: int, b0: float):
    rows = []
    for _ in range(n):
        x = (rng.random() < 0.18, rng.random() < 0.28, rng.random() < 0.12)
        y = rng.random() < sigmoid(b0 + 0.8 * x[0] + 1.0 * x[1] + 0.5 * x[2])
        rows.append((x, y))
    return rows


train, dev, vault = period(120_000, -4.005), period(58_000, -4.42), period(24_466, -4.42)


def grouped_fit(rows, canary_flip=None):
    cells = {}
    for x, y in rows:
        c = canary_flip(y) if canary_flip else None
        key = (x, c)
        n1, n0 = cells.get(key, (0, 0))
        cells[key] = (n1 + y, n0 + (not y))
    design, outcome, weight = [], [], []
    for (x, c), (n1, n0) in cells.items():
        base = [1.0, float(x[0]), float(x[1]), float(x[2])] + ([float(c)] if canary_flip else [])
        for yv, cnt in ((1.0, n1), (0.0, n0)):
            if cnt:
                design.append(base)
                outcome.append(yv)
                weight.append(float(cnt))
    return logistic_irls(design, outcome, ridge=1e-4, weights=weight)


raw = grouped_fit(train)
dev_rate = st.mean(float(y) for _, y in dev)
vault_y = [float(y) for _, y in vault]
predict = lambda b, x, c=None: sigmoid(sum(bj * xj for bj, xj in zip(b, [1.0, *map(float, x)] + ([float(c)] if c is not None else []))))  # noqa: E731
p_raw = [predict(raw, x) for x, _ in vault]
skill = lambda ps: 1 - st.mean((p - y) ** 2 for p, y in zip(ps, vault_y)) / st.mean((dev_rate - y) ** 2 for y in vault_y)  # noqa: E731
# The skills move with the realised rates of a 24,466-stop vault: the book's own instrument, run with seeds 2022 to
# 2025 instead of 2021, gives raw skills from +0.001 to +0.008 and canary gains from 0.668 to 0.694 (2026-10-19).
check("20", "raw model: skill against the development rate", -0.0020, skill(p_raw), 0.012, "simulated")
flip_rng = random.Random(0xCA11A710)
flip = lambda y: (not y) if flip_rng.random() < 0.01 else y  # noqa: E731
with_canary = grouped_fit(train, flip)
p_canary = [predict(with_canary, x, flip(y)) for x, y in vault]
check("20", "canary with the clock switched off: skill gain", 0.6521, skill(p_canary) - skill(p_raw), 0.08, "simulated")
check("20", "canary with the clock switched off: its coefficient", 9.23, with_canary[4], 0.15, "simulated")
