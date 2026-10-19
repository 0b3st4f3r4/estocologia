# Declaration 2 of point 23: the two refinements, K = 2

**Written on 2026-10-19, after the single run of declaration 1 and before the instrument it declares first read a
dataset.** Its SHA-256 and that of `crates/stochology-book/src/wave2.rs` are appended to `DECLARACAO.sha256` and
committed before `cargo run --release --bin labs -- --wave2` is first run. By the owner's decision of the same day to
proceed with the refinement that the report of declaration 1 proposed. Written with an AI assistant.

## 0. What this is, and what it costs

A second look at the same data, with changes chosen **after** seeing the first verdicts. That is what K counts:
each test now stands at K = 2, and a pass here is weaker evidence than a pass at K = 1 would have been, because the
encoding was adjusted to what the first run showed. Declaration 1, its sealed code (`wave.rs`) and its verdicts stay
as they are: T1 not supported, T2 not testable. Nothing in this declaration rewrites them, and both readings will be
reported side by side.

## 1. T1 at K = 2

Everything of declaration 1, §1 (data, anomaly, weeks as the sealed code cuts them, z-scores, spike shape, phase as
the time of day, the engine's bus, the null pairing at +13 weeks, rules P1, P2, P3 and "adds", the secondary margin),
with three changes:

- spikes open at excursions at or above **1.0** instead of 1.5;
- the STDP memory is **τ+ = τ− = 12 half-hours** (six hours) instead of 6;
- the horizon is **48 half-hours** (a day) instead of 12, and the rival's lags go to **48** to keep the same horizon.

The reason, from the first run: the engine abstained in 63 of 155 weeks because no pair of spikes fell within six
hours, and thermal inertia in buildings acts over hours to a day. The report of declaration 1 also proposed learning
from pairs inside the same half-hour; that is dropped, because a pair with no order carries no direction, and the
STDP rule needs one.

## 2. T2 at K = 2

Everything of declaration 1, §2 (data, split, sources, eligibility, Laplace log-odds, hours with at least 10
flights, weights, first harmonic over 24 hours, median normalisation, the engine's thresholds, System 1 and System 2,
the rival, the bootstrap, the rules, the minimum strata), with one change: the profile reduced to a phasor is the
**deviation** of the level's hourly log-odds from the **common hourly log-odds of all training flights**, hour by
hour. The common evening peak of delays, which put every source in phase in the first run, is taken out before the
phase is read, as the anomaly of T1 takes out the daily cycle. System 1 uses the harmonic of this deviation as its
per-source covariate, so that it still carries exactly what the phasors carry.

## 3. The synthetic controls, run before this declaration was sealed

- T1, planted arrow: engine 142 of 156 correct, null 61, rival 134; supported, does not add (0.05 ahead, 0.10
  needed). Independent: engine 70 against 86; not supported.
- T2, the synthetic flights of declaration 1 with a common evening peak added to every flight, the shape the real data
  showed. With the phase of the raw profile (declaration 1): every one of 60,000 vault flights in resonance, not
  testable; the first run's failure reproduced. With the phase of the deviation: conflict 13,324, resonance 19,250;
  with the planted interaction Δ +0.00264, interval [+0.00156; +0.00365], Δ_rival +0.00273; without it Δ −0.00005,
  interval [−0.00105; +0.00089]. The deviation brings the conflict back and detects the interaction; the rival
  without phase still matches it.

## 4. What will be reported, whatever comes out

Both verdicts of this declaration beside both of declaration 1, in `README.md` and in the registry. A pass at K = 2
is reported as a pass at K = 2. Any further change is a third declaration, sealed before its run.
