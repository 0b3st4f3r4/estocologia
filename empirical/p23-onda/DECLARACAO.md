# Declaration of point 23: the wave engine against data

**Written on 2026-10-19, before the instrument it declares first read a dataset.** Its SHA-256, together with the
SHA-256 of the three source files it fixes, enters `DECLARACAO.sha256` and is committed before
`cargo run --release --bin labs -- --wave` is run for the first time. By the owner's decision of the same day: the
wave engine (`crates/stochology-wave`) was meant as an evolution of the research, and it is to be refined and put to
a test on data already catalogued, not archived. Written with an AI assistant; it is not an audit from outside.

## 0. Why a test, and what had to change first

Until today the engine produced no published number. Its six gates (G1 to G6 of `wave --check`) prove that its
mathematics is sound (a Hermitian Laplacian, a unitary propagator, a seal chain, determinism, a learnt asymmetry on a
synthetic cascade); none of them could fail because the engine misjudged a claim about the world. Two of its three
layers make claims that the catalogued data can refute. The third (the magnetic Laplacian and its dissonant
vertices) has no catalogued dataset with a known truth for it, and is not tested here.

Two refinements were needed before any test, and both are in the sealed code:

- `channel.rs`: the phasor STDP rule over N channels on the data's own clock. The engine's bus was the five lenses on
  a microsecond clock. A test pins that five channels in lens order reproduce the old bus number for number.
- `cycle.rs`: the phase of a phasor as a time, the position in a cycle where a profile peaks (its first harmonic).
  With a phase that only says "for" or "against" (0 or π), the coherence of `phasor.rs` is a signed sum normalised,
  and the superposition adds nothing a sum does not.

## 1. T1, the arrow (layer 3)

**The truth.** In `data/vic_elec.csv` (Victoria, half-hourly, 2012-01-01 to 2014-12-31), temperature drives
electricity demand, and demand does not drive temperature.

**The encoding.** Each series becomes an anomaly: the value minus the mean of the same half-hour on the three days
before and the three after (fewer at the edges). This removes the daily cycle that both series share, which is a
common cause of their co-movement and not the causal path. The series is cut into 156 weeks of 336 half-hours
from 2012-01-01; the last four days are dropped. In each week the anomalies become z-scores. Temperature spikes open
at the first half-hour of each excursion of |z| at or above 1.5 (heat or cold, both raise demand); demand spikes open
at each excursion of z at or above 1.5 (surges only). Amplitude min(1, peak / 3); **phase the time of day**,
2π × (half-hour of the day) / 48.

**The engine.** A fresh two-channel bus per week, initial conductance 0.25, with the rule of `StdpConfig` on the
half-hour clock: q 0.5, A+ 0.18, A− 0.12, τ+ = τ− = 6 half-hours, learning rate of the phase 0.30, Oja 0.04, horizon
12 half-hours. The week's arrow is the sign of η(temperature → demand) − η(demand → temperature); zero is no arrow.

**The rival.** The lagged cross-correlation on the same z-scores over the same horizon:
Σₖ₌₁¹² corr(|z_T|ₜ, z_D,ₜ₊ₖ) − Σₖ₌₁¹² corr(|z_T|ₜ₊ₖ, z_D,ₜ), its sign the arrow.

**The null.** Each week's demand paired with the temperature of the week thirteen weeks later (modulo 156): the time
of day and the weekday are kept, the link is broken. Both the engine and the rival are read on the null pairing.

**The rules** (rates are over the 156 weeks; "correct" is temperature → demand):

- P1: correct > wrong, the exact one-sided binomial P(X ≥ correct | correct + wrong, ½) below 0.01, and the correct
  rate at least 0.60.
- P2: the correct rate minus the null's temperature → demand rate at least 0.20.
- P3: the rival's correct rate minus the engine's at most 0.10 (the engine is not worse than the rival by more than
  ten weeks in a hundred).
- **T1 is supported** if P1, P2 and P3 all hold. **The engine adds over the rival** if its correct rate exceeds the
  rival's by at least 0.10 and its null rate does not exceed the rival's null rate.

Reported but not ruled on: the same tally with a margin of 0.05 on the asymmetry, and the median phase lag the bus
learns, in hours. The synthetic control below shows that the learnt lag does not recover a planted lag of two hours
(it gives 0.17 h), so the lag is **not** a claim of this test.

## 2. T2, the conflict (layer 1, phase as time)

**The truth.** Whether a flight out of New York arrived more than 15 minutes late (`data/nyc_flights_2013.csv`,
`data/nyc_flights_2023.csv`), read with the loader of Chapter 20 (`streams.rs`, `load_flights`).

**The split.** As in Chapter 20, by the instant of emission (two hours before the scheduled departure): the 60 days
before 1 October to learn, October to December as the vault.

**The encoding.** Three sources per flight: carrier, origin, destination. A level is eligible with at least 200
training flights. For each eligible level, the log-odds of delay by scheduled departure hour, Laplace (k + 1)/(n + 2),
over the hours with at least 10 flights, weighted by their counts, is reduced to its first harmonic over 24 hours:
amplitude A (how much the risk swings over the day) and **phase φ, the hour of peak risk**. The amplitudes are divided
by the median amplitude over all eligible levels of the three sources. A vault flight enters only if its three levels
are eligible.

**The engine.** The three phasors superpose in `phasor::superpose` with the engine's own thresholds, unchanged since
2026-10-17 (activation 0.15, resonance 0.85, critical 0.35, critical sum 0.80). The regime says whether the fast
forecast suffices (resonance: System 1) or deliberation is needed (conflict: System 2).

**System 1 and System 2.** System 1 is the superposition read as an additive model: a logistic regression (ridge 1.0
outside the intercept) on the eligible-level dummies of the three sources, five departure bands (to 8 h, 9–12, 13–16,
17–20, later), and for each source the covariate A cos(2πh/24 − φ) of the flight's level at its hour. Nothing in it
crosses two sources. System 2 is the cell (carrier, origin, departure hour) of the training flights, shrunk toward
System 1 with 50 imaginary flights: p₂ = (k + 50 p₁)/(n + 50). The gain of a flight is its Brier under System 1
minus its Brier under System 2.

**The claim.** The gain of deliberation is larger where the engine says conflict than where it says resonance:
Δ = mean gain in conflict − mean gain in resonance > 0.

**The rival.** The same flights ordered by the agreement of their three sources' levels without any phase,
|Σ dₛ| / Σ |dₛ| with dₛ the level's log-odds minus the global log-odds of the training flights; the least agreeing
flights, as many as the engine put in conflict, against the most agreeing, as many as it put in resonance.
Δ_rival is computed the same way.

**The rules.**

- **Testable** only if both vaults have at least 500 flights in conflict and at least 500 in resonance. Otherwise the
  verdict is "not testable under this encoding", which is not a pass.
- **T2 is supported** if testable, the 95% interval of Δ in 2013 lies above zero, and Δ in 2023 is positive. The
  interval is a bootstrap of whole vault days, 2,000 draws, SplitMix64 seed 2026, percentile.
- **The engine adds over the rival** if testable and Δ > Δ_rival in both vaults.

Reported but not ruled on: the counts in each regime, the Brier of both systems, and the gains in each stratum.

## 3. The synthetic controls, run before this declaration was sealed

The development touched only synthetic data, generated with seed 2026 inside the instrument; the controls are
printed before the real readings on every run, and two unit tests hold them.

- T1, an arrow planted two hours deep: engine 141 of 156 weeks correct, 15 wrong; null 57; rival 155; **supported**,
  does not add. T1, independent series: engine 80 correct, 64 wrong, 12 none (binomial p 0.11); **not supported**.
- T2, an interaction planted in the three of 36 carrier-origin pairs whose peaks lie twelve hours apart, in the
  evening: Δ +0.00083, interval [+0.00003; +0.00166]; Δ_rival +0.00099. T2, no interaction: Δ −0.00052,
  interval [−0.00107; +0.00004]. The test detects a concentrated interaction, with little margin; in a world built
  for the engine, the rival without phase matched it.

Three changes were made during development, all before any real data were read, all on synthetic evidence, and all
are counted here so that K does not hide them: System 1 first had no per-source daily profile, which made System 2
gain most in resonance even in a world without interactions (System 1 was missing what the phasors carry); the first
two synthetic worlds were rebuilt because their marginal profiles lost the antiphase the interaction lived in. K = 1
for each test on real data: this is the first declared version, and no other will replace it silently.

## 4. Prior exposure of the data

None of the three datasets is a fresh vault. The vaults of 2014 (energy) and October to December of 2013 and 2023
(flights) were opened for the forecasts of Chapter 20, for other questions: interval coverage and calibration. No
reading of the engine, of the temperature-demand link, of the daily risk profiles or of any cell interaction has been
made on them in this programme. The test is a first look for these questions, not a fresh vault, and is reported as
such.

## 5. What will be reported, whatever comes out

Both verdicts, with every count, rate and interval the instrument prints, in `empirical/p23-onda/README.md` and in
the registry, beside these rules. A failure of T1 or T2 is a result of the engine on data, not a reason to change the
encoding: any later change is a new declaration, sealed before its run, with K raised.

## 6. What is sealed

`DECLARACAO.sha256` holds the SHA-256 of this file and of `crates/stochology-book/src/wave.rs`,
`crates/stochology-wave/src/channel.rs` and `crates/stochology-wave/src/cycle.rs` as they stand when sealed. A run
whose sources do not match those hashes is not a run of this declaration.
