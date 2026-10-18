# Declaration (background Point 6): the H3 performative loop measured in real data

Written before any estimate of the effect. The SHA-256 hash of this file lives in `DECLARACAO.sha256`.
Any change after seeing results enters as a new declaration and counts in K.

## What H3 declared

In `h3/estocologia/performativo.py`, the call center's loop is declared, not measured. Staffing comes from the forecast by
Erlang C, whoever waits too long gives up (exponential patience of 180 s, assumed), and **whoever gives up redials with
probability r = 0.6**, 60% in the following 5-min interval and 40% in the other (`DIV = (0.6, 0.4)`, line 36).
Whoever is served does not call again. The r̂ estimated by 2SLS in the simulated loop was 0.563, 90% CI by day
bootstrap from 0.51 to 0.61 (`h3/saida/portoes.json`, lines 270–277). The naive forecaster overestimated exogenous
demand by 29.1% (`portoes.json`, line 283).

So the loop parameter is: **the causal effect of abandoning the queue, versus being served, on the
probability of a new call from the same customer shortly after**. In H3 that effect is r = 0.6, because whoever is
served has probability 0 of returning.

## Data

AnonymousBank, the telephone call center of an Israeli bank, all calls of 1999 (Technion SEE Lab, Guedj and
Mandelbaum). Each row is a call with `customer_id`, instants of queue entry, queue exit and
service, and the outcome (AGENT, HANG, PHANTOM). Downloaded from two independent mirrors on GitHub, checked
against each other (`carregar.py`): 444,448 rows in each one, 444,436 paired by key, and all fields equal
after normalizing the leading zeros. Details in
`/mnt/project-files/estocologia/data/empirical/p06-laco/FONTES.md`.

### What I looked at before declaring (structure, not the effect)

- Outcomes: AGENT 353,143, HANG 87,706, PHANTOM 3,599.
- Owing to a system defect documented in the documentation, `customer_id` is almost never recorded for those who did not
  enter the queue: only 5,632 of 157,137 served without a queue have an identifier. Among those who entered the queue,
  163,920 of 196,006 served and 37,138 of 64,006 abandoners have an identifier.
- 300 rows have `customer_id` in scientific notation (for example 3.27E+11), corrupted in both mirrors.
- Queued calls, identified, with outcome AGENT or HANG: 201,058, from 12,525 customers, over 360 days.
- I computed no redial rate, neither by group nor overall.

## Design

**There is no randomization.** The call center did not vary staffing at random, and the data does not record staffing. The best design
available is quasi-experimental: the queue the customer finds upon entering. Within the same date and the same hour,
the number of people ahead in the queue depends on how many other customers arrived in the previous minutes and on
how long the services in progress lasted. That is third-party Poisson noise and should not depend on this
customer's patience or need. The longer queue increases the chance of abandoning. It is the same channel as the
H3 action: fewer attendants than the load demands means a longer queue.

- **Unit of analysis:** the index call, an identified call (`customer_id` different from 0 and without
  scientific notation) that entered the queue (`q_start` > 0), with outcome AGENT or HANG, type different from TT
  (TT is a callback initiated by the bank).
- **Treatment D:** abandoned (HANG) = 1, served (AGENT) = 0.
- **Instrument Z:** number of other calls in the queue at the instant `q_start` of the index call (counted across
  all calls in the data, identified or not, with `q_start` ≤ t < `q_exit`), excluding the call itself.
- **Outcome Y_W:** there is another call from the same `customer_id`, not TT and not PHANTOM, with `vru_entry` in the
  interval (end of the index call, end + W], where the end is `q_exit` for whoever abandoned and `ser_exit` for
  whoever was served. Main W = 15 min, which covers the two 5-min intervals of H3 counted from any
  point of the abandonment interval. Secondary W: 60 min and until the end of the same day.
- **Controls (fixed effects):** date × hour of `q_start`, priority (0/1 versus 2) and service type.
- **Main estimator:** 2SLS with the fixed effects absorbed by centering within cells
  (date × hour × priority × type), one equation, one instrument.
- **Passive comparison (what AGENTS.md forbids confusing):** the OLS difference in Y_W between abandoners and
  served with the same fixed effects. It mixes the effect of the action with selection (whoever abandons may be
  whoever had less urgency or more haste).
- **Margin:** the day is the independent unit, as in H3. 90% CI by day bootstrap (1,000 replicates,
  `np.random.default_rng(2026)`), percentile.
- **Instrument strength:** first-stage F with robust error by day; if F < 10, the 2SLS is reported as
  weak and the criterion does not decide by it.
- **Declared robustness (does not decide):**
  1. the same 2SLS with one more control: the mean queue of other calls in the 15 min after the end of the
     index call. It serves the measurement problem below. The queue afterwards is posterior to the treatment, but a
     single call changes the queue of everyone little;
  2. the instrument swapped for log(1 + Z).

## Declared measurement problem

A new call is only seen if it is identified, and it is almost only identified if it enters the queue. A new
call served without waiting stays invisible. That is why Y_W measures "new call that found a queue", and the measured
rate is a lower bound of the real redial rate in both groups. Since the queue persists over time, a high Z
also increases the chance that the new call is seen, for abandoners and served alike. That can violate
exclusion. Robustness 1 attempts to control this path, but does not eliminate it.

## Metrics and criterion

Question: **does the loop declared in H3 have the sign and the order of magnitude of the measured one?**

With r_IV the main 2SLS estimate (W = 15 min) and 90% CI by day bootstrap:

- **Sign checks out** if the 90% CI of r_IV lies entirely above 0.
- **Order of magnitude checks out** if 0.6 / r_IV lies between 1/3 and 3, that is, if r_IV lies between 0.2 and 1.8.
- **Compatible in the strict sense** if 0.6 lies inside the 90% CI of r_IV (reported, does not decide).
- **Verdict:**
  - "the H3 loop has the sign and the order of the measured one" if the first two conditions hold and F ≥ 10;
  - "the H3 loop overestimates (or underestimates) the measured one" if the sign checks out but the order does not;
  - "does not check out" if the 90% CI includes 0 or lies below 0;
  - "undecided" if F < 10.

## Secondary metric (reported, does not decide)

1. The fraction of the redials within 60 min that arrive within 15 min, among abandoners. H3 assumes 100%.
2. The excess of observed arrivals caused by the loop: r_IV × (abandonments in the queue) / (attendant requests
   − r_IV × abandonments in the queue), with attendant requests = calls that entered the queue or were served
   without a queue. In H3, the analogue is the bias of the naive forecaster against exogenous demand, 29.1%.
3. The raw rates of Y_W by group and the OLS difference (the passive reading).

## What counts as a failure of this attempt

- If the first-stage F is less than 10.
- If robustness 1 changes the sign of r_IV or takes it out of the 0.2–1.8 band in which the main one falls, the main
  result is reported as fragile.
- This design is quasi-experimental. Even if it passes, it does not replace the measurement in a real operation with
  randomized variation of staffing, which stays proposed in the README.
