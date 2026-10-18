# Declaration of point 15: predictions 3 and 5 of the R axis, measured

**Frozen on 2026-10-16T22:17:44Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `DECLARACAO.md` (this one) | — | the five declarations; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `1f97cd7c0b79ade4f264f2e1b08588602e27968f454c64d7b5ff534e125a7c91` | the result in one page |
| `DECLARACAO.sha256` | — | the two hashes, in `sha256sum` form |

## Declaration 1 — the criterion, and where it was written

The two predictions are in `book/horizontes-de-resolucao-da-pergunta.md` since 2026-10-16, written **before**
an instrument existed for them:

> 3. **Oscillation threshold.** There is a point beyond which a decision taken in R0 fails in more than one
> quarter of the days, and below it refining ceases to pay off.
>
> 5. **Transfer.** A result that survives R1 survives R0; the converse is not true.

The document fixes the **quarter** (more than 25% of the days) and does not define what a failed day is. The criterion of failed day
is declared here, before the second specification: **the day fails when more than 10% of the customers wait more than
30 min** — the same threshold that Chapter 16 uses to speak of bad days. The staffing of R0 is sized by
the Erlang C formula with a target of 80% (at most 20% of the customers enter the queue), which is what H0 does.

## Declaration 2 — the two specifications of the R0 plan

**Specification 1.** The staffing of R0 is that of the average day, by the Erlang C formula (target of 80%). It is the natural reading
of asking about the average day, and the one that the instrument of the axis already used.

**Specification 2.** After seeing the first, the staffing of R0 came to be **calibrated on flat demand**: the smallest
number of agents at which the demand of the average day, without any oscillation, makes at most 10% of the days fail. The
calibration entered because specification 1 leaves the prediction **impossible to measure**: with 12 agents flat demand
already fails 1 day in 5 (0.205), so the point beyond which it fails more than a quarter marks nothing about the
oscillation — it marks the plan.

This is a **new specification**, not a fix, and it enters the **K of this front: K = 2**. The two stay measured and
published side by side; the first is not rewritten, and its result is negative.

## Declaration 3 — the numbers of the two specifications

~~~bash
cargo run --release --bin labs -- --resolucao
~~~

Design common to the two: 200 days per amplitude, 10 h per day, average demand of 8 arrivals per hour, the odd day
strong and the even day weak, seed 1909, 12 or 13 agents. The loss is the average wait of the aggregate minus that of the fine grain, in
minutes.

**Specification 1 (Erlang plan, 12 agents).**

| amplitude | failed days | fraction | loss (min) |
| --- | --- | --- | --- |
| 0.0 | 41 | 0.205 | −0.26 |
| 0.5 | 52 | 0.260 | −1.89 |
| 1.0 | 51 | 0.255 | +1.97 |
| 1.5 | 64 | 0.320 | +2.60 |
| 2.0 | 66 | 0.330 | +2.27 |
| 2.5 | 73 | 0.365 | +7.04 |
| 3.0 | 80 | 0.400 | +8.93 |

Crossing at 0.5, with a loss of **−1.89 min**, and the fraction **is not monotonic** (0.260 at 0.5 against 0.255 at 1.0).
**Prediction 3 does not hold under this specification.**
**Specification 2 (calibrated plan, 13 agents).**

| amplitude | failed days | fraction | loss (min) |
| --- | --- | --- | --- |
| 0.0 | 19 | 0.095 | −2.24 |
| 0.5 | 30 | 0.150 | −4.08 |
| 1.0 | 33 | 0.165 | −0.66 |
| 1.5 | 39 | 0.195 | −0.65 |
| 2.0 | 42 | 0.210 | −1.12 |
| 2.5 | 53 | 0.265 | +2.87 |
| 3.0 | 66 | 0.330 | +3.36 |

Monotonic, crossing at **2.5**, loss at the crossing of **+2.87 min**, and **a negative loss at every amplitude
below it** (−2.24 to −0.65 min). The two halves of the prediction hold: the point exists, and below it refining
does not pay — in the measured design, refining comes to **cost** 4.08 min of average wait, because the coarse plan is
oversized on the weak days.

## Declaration 4 — prediction 5

Six demand profiles (mean 8, amplitudes from 0.5 to 3.0), 200 days each, with the R0 plan of the average day:

| what | value |
| --- | --- |
| profiles in which R1 survives and R0 does not (**violations of the prediction**) | **0** |
| profiles in which R0 survives and R1 does not (**the converse, which the prediction says exists**) | **3** |
| conservation of the aggregate checked (the sum of the days reconstructs the aggregate) | yes |
| threshold of the day | 0.10 |

**Prediction 5 holds.** And it is a theorem, not an accident: the aggregate is the weighted mean of the days, so
if every day stays below the threshold the aggregate stays below it too. The instrument checks that arithmetic in each profile
(`conservacao_do_agregado`), rejects an aggregate that does not match the sum of the days, and counts **how many** profiles
prove that the converse does not hold — three, in the measured design.

## Declaration 5 — the limits

1. **It is declared simulation.** The queue is that of one server per agent, exponential service of mean 1 h, 10 h per day,
   200 days, a single average profile (8 arrivals per hour). The threshold of 2.5 is a property of **this** design, and
   not a constant of the world.
2. **Prediction 3 still needs what the document asked for:** a real operation with the same decision taken at
   two grains. Here it was measured in the laboratory, which is less than that.
3. **The negative loss below the threshold was not foreseen** and is the most interesting finding of the round: refining can
   cost when the coarse plan is oversized on the weak days. It stays recorded as a reading, without a new prediction.
4. **The criterion of failed day was declared in this front**, and not in the document of the axis: whoever compares with another
   number has to use the same one (more than 10% above 30 min).

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-core` | 59 tests passed, among them the six of the predictions: the threshold with the calibrated plan, the Erlang plan that fails prediction 3, the transfer without violation and with a counterexample, the conservation that rejects an aggregate that does not match, and the checking of the implication (0) |
| `labs --resolucao` | the resolution loss of the three cases of the document (46.24 / 5.97 / 0.00 min), prediction 3 under the two specifications, and prediction 5 (0) |
| `python3 scripts/book/generate_results.py --check` | `docs/RUST_RESULTS.md` updated, with the block of the R axis (0) |
