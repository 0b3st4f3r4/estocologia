# Resolution horizons of the question

A proposal for a new axis for the theory, written on 2026-10-16. The current empirical axis (H0 to H3) says **where** the
question is asked: seed, complete cycle, transfer between domains, live mesh. This document
proposes a second axis, orthogonal to the first: **how fine** the decision-maker's question is, and how much it costs
to ask coarsely.

## Why this axis belongs to the theory, and not to the method

The invariants of the program all hang on the question. $\Phi_2$ demands the margin on the independent unit; $\Phi_4$
demands scope and who pays; the type III error is answering another question. Until now, however, the question entered
the account as binary, declared or not. The thesis that is proposed is stronger: **the resolution of the question is a
variable of the model, with a measurable effect on the number and on the action.**

## The ladder of resolution

| Level | The question | What it decides | Where it already appears in the book |
| --- | --- | --- | --- |
| **R0 aggregated** | "On average, does the service cope?" | sizing by the average day | Erlang C over the average rate (Chapter 16) |
| **R1 day** | "Each day, does the service cope?" | sizing by the day that exists | the clinic that oscillates between 5 and 11 (Chapters 5 and 16) |
| **R2 hour** | "Each hour, does the service cope?" | sizing by the peak | the reinforcement at the bank's peaks (Chapter 16) |
| **R3 sub-question** | "Each hour, and for whom?" | sizing by subgroup, with a declared cost | Chouldechova's trilemma (Chapter 11) |

Comparing resolutions only makes sense when one **refines** the other. R1 answers the question of R0 at a finer grain,
and so on.

## The instrument

```bash
cargo run --release --bin labs -- --resolution
```

The arms come paired. The arrivals and the service bases do not change from one arm to the other; only the service
rate changes, and the **same total resource** holds in both arms (the aggregate uses the average capacity every
day; the fine one uses the capacity of that day). Measured output:

```text
demanda 5 ou 11 (o posto do Capitulo 16): capacidade media 8.0 | espera agregada 86.48 min, espera fina 40.24 min, perda 46.24 min
demanda 7 ou 9 (oscilacao leve): capacidade media 8.0 | espera agregada 47.70 min, espera fina 41.73 min, perda 5.97 min
demanda 8 ou 8 (sem oscilacao): capacidade media 8.0 | espera agregada 41.56 min, espera fina 41.56 min, perda 0.00 min
```

The resolution loss is **46.24 min** where demand oscillates from 5 to 11, **5.97 min** where it oscillates from 7 to 9 and
**zero** where it does not oscillate. The two arms operate near saturation, and that is why the absolute waits are
large; what is measured here is the **difference**, which does not depend on the calibration of the rate.

## What the registry already shows, with no new data

| Case | R0 | R1 or finer | Source |
| --- | --- | --- | --- |
| Call centre | Erlang C promises 82% | 57% measured; 26 failed days in 100; with +7.2% of agent-hours at the peaks, fewer than 4 in 100 | `h0-real/README.md` |
| Health clinic | constant day: 9.1 min | day that oscillates: 19.3 min, with the same mean | `labs --chapter 16` |
| STAR (education) | grouping by student keeps the area | grouping by the 80 schools brings it down to $\hat S = \emptyset$ | `governance/GRUPOS.md`, Chapter 19 |
| hERG (pharmacology) | 16 instants per volunteer | grouping by volunteer keeps the area, with skill +0.57 | `h2g/README.md` |
| Multiplicity | 4 areas before the correction | 3 areas after the grouping | Chapters 15 and 19 |
| Fires | 36-month window | calendar year 1929 to 1932: the number changes with the window | Chapter 18 |
| H3 vault | laboratory: ECE 0.0022 | real vault: ECE 0.0048 | Chapter 20 |

In every case, changing the resolution **did not change the arithmetic**; it changed the question, and with it the number and the
action. The lesson of the independent unit reappears here, now on the axis of the grain of the question.

## Falsifiable predictions

1. **Monotonicity.** With the same total resource, refining does not increase the average wait ($R_{k+1} \le R_k$).
2. **Convexity of the loss.** The resolution loss grows faster than the oscillation of demand (0.00 → 5.97 →
   46.24 in the three measured cases).
3. **Oscillation threshold.** There is a point beyond which a decision taken at R0 fails on more than one
   quarter of the days, and below which refining stops paying off. **Measured** (`labs --resolution`, background point 15):
   it depends on how the R0 scale is sized, and both readings remain published. With the Erlang C scale of the
   average day (12 agents) the prediction **does not hold** — flat demand already fails 1 day in 5, the fraction is not
   monotone and at the crossing refining costs. With the scale calibrated on flat demand (13 agents, at most 10% of
   failed days with no oscillation at all) it **does hold**: the fraction grows from 0.095 to 0.330, crosses the quarter at the
   amplitude **2.5**, the resolution loss is negative in every amplitude below the threshold and becomes **+2.87 min**
   at the crossing. The criterion of a failed day is declared: more than 10% of the clients above 30 min.
4. **Cost of refining.** The gain of R1 over R0 is limited by the available data, and refining beyond what the data
   support becomes the inflation of $K$ that the counter of attempts exists to contain.
5. **Transfer.** A result that survives R1 survives R0; the converse is not true. **Measured**
   (`labs --resolution`, background point 15): in 6 demand profiles × 200 days, **0 violations** of the implication and
   **3 counterexamples** — profiles that pass at R0 and fail on some day. The conservation of the aggregate (the sum of the days
   has to reconstruct the aggregate) is checked in every profile, and an aggregate that does not add up is rejected.

## What the architecture needed to gain

- **New field in the sheet.** The resolution of the question (R0 to R3) sits beside `pergunta_do_decisor` and the independent
  unit. In the code the name is `resolucao`; the plan called it `resolucao_da_pergunta`.
- **Coupling rule (a candidate for $\Phi_5$).** The declared action has to be at the grain of the independent
  unit; action by day with a margin by patient is a misalignment, even if the number is right.
- **Cheap empirical test.** The six fronts of the table above already allow the resolution loss to be measured
  retroactively, with no new data. Only prediction 3 (oscillation threshold) demanded new data, and it asked for a front
  with the same decision taken at two grains. It was measured in the laboratory (point 15, with the criterion of a failed day
  declared before the second specification of the plan), and what is missing remains exactly what this document
  asked for: the real operation with the same decision at two grains.
- **State of the code.** The type `Resolucao` and the instrument `medir_perda_de_resolucao` are in
  `crates/stochology-core/src/resolucao.rs`, with 3 tests. The field in the sheet and the coupling rule left the
  proposal and became a charged invariant. The field entered as `Ficha.resolucao`
  (`crates/stochology-core/src/camada.rs:52`, with the resolution swap in `camada.rs:98`) and the rule became
  $\Phi_5$ in `crates/stochology-core/src/invariantes.rs:110`, counted by the gates binary
  (`crates/stochology-gates/src/main.rs:316`). Declared limit: the lock only bites when the independent unit
  is the day or coarser, and the test `unidade_mais_fina_que_o_dia_fica_fora_do_phi5_declarado` guards that
  boundary.
