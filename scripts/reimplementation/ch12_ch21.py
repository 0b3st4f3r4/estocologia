"""Chapters 12, 13, 15, 20 and 21: the accounts that the text states in full.

Chapter 12: the EVPI of the sensor, as E_state[max_act payoff] - max_act E_state[payoff] (Chapter 11's definition).
Chapter 13: the closed form of the Beta(4, 8) mean. Chapter 15: the SHA-256 test vectors. Chapter 20: the arithmetic
that ties the printed Brier scores, rates and skills together. Chapter 21: the count of the verifier's rejections.

Open, because the text does not give the recipe: the hypergraph proofs and the Hill series of Chapter 12, the System 1
hash and the System 2 search of Chapter 13, the property test of Chapter 14, the hash chain format of Chapter 15 and
the canary model of Chapter 20.
"""

from __future__ import annotations

import hashlib

from common import check, open_choice


def evpi(actions: list[list[float]], probs: list[float]) -> tuple[float, float, float]:
    without = max(sum(x * p for x, p in zip(row, probs)) for row in actions)
    with_info = sum(p * max(row[s] for row in actions) for s, p in enumerate(probs))
    return without, with_info, with_info - without


probs = [0.7, 0.3]
without, with_info, value = evpi([[10, -4], [0, 0]], probs)
check("12", "sensor: best act without information", 5.8, without, 1e-9)
check("12", "sensor: value with perfect information", 7.0, with_info, 1e-9)
check("12", "sensor: EVPI (act or not act)", 1.2, value, 1e-9)
check("12", "sensor: EVPI with only half the stake", 0.6, evpi([[5, -2], [0, 0]], probs)[2], 1e-9)
check("12", "useless sensor: EVPI", 0.0, evpi([[10, 10], [0, 0]], probs)[2], 1e-9)

check("13", "Beta(4, 8) mean, closed form", 0.3333, 4 / 12, 5e-5)

check("15", 'sha256("") prefix as integer', int("e3b0c44298fc1c14", 16),
      int(hashlib.sha256(b"").hexdigest()[:16], 16), 0)
check("15", 'sha256("abc") prefix as integer', int("ba7816bf8f01cfea", 16),
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

missing = "the text does not give the recipe"
open_choice("12", "Hill of the hypergraph series, k = 500", 1.7901, missing)
open_choice("13", "System 1 triage probability of the card", 0.94, missing)
open_choice("14", "property test: minimum margin of 10,000 cards", 0.00617, missing)
open_choice("15", "Merkle root of the textbook chain (first 8 hex digits as integer)", int("92f2ab76", 16), missing)
open_choice("20", "canary with the clock switched off: skill gain", 0.6521, missing)
