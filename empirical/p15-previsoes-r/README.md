# Point 15 — predictions 3 and 5 of the R axis

**State: measured, with one prediction that holds only under the second specification of the plan, and that declared.**

The axis document (`book/horizontes-de-resolucao-da-pergunta.md`) wrote five falsifiable predictions on
2026-10-16. Predictions 1, 2, and 4 already had an instrument (`resolucao.rs`, the `labs --resolucao`). This front measures
the two that were missing.

**Prediction 3 — the oscillation threshold.** It depends on how the R0 plan is sized, and both readings stay
published. With the Erlang C staffing of the average day (12 agents), the prediction **does not hold**: flat
demand already fails 1 day in 5, the fraction is not monotonic, and at the crossing refining costs. With the staffing calibrated on flat demand
(13 agents), it **holds**: the fraction grows from 0.095 to 0.330, crosses one-quarter at amplitude **2.5**, and the
resolution loss is **negative at every amplitude below the threshold** and turns into **+2.87 min** at the crossing. The second
staffing is a new specification, declared, and enters the K of the front (K = 2).

**Prediction 5 — the transfer.** It holds: in 6 profiles × 200 days, **0 violations** of the implication (no profile
survives R1 and fails in R0) and **3 counterexamples** (profiles that pass in R0 and fail on some day). The instrument
checks the conservation of the aggregate — the sum of the days must reconstruct the aggregate — and rejects an aggregate that does not
match.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --resolucao
~~~

**The limit.** It is declared simulation: one average profile, 200 days, exponential service. The threshold of 2.5 is a
property of this design, and the real operation with the same decision at two grains remains what is missing.
