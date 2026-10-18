# Background Point 6: the H3 performative loop measured in real data

## Result

There is no real operation within reach from here. The H3 loop remains unmeasured in an operation that Stochology
controls. What could be done was to measure the same mechanism in a real call center, that of AnonymousBank (Israeli
bank, all calls of 1999), with a quasi-experimental design and not with a randomization. The design uses the
queue found upon entering, within the same date and hour, as an instrument for abandonment. By that design,
abandoning the queue instead of being served increases by **0.145** (90% CI by day bootstrap from 0.132 to 0.157) the
probability that the same customer calls again within 15 min. H3 declared **r = 0.6** in that same window.
**The sign checks out and the order of magnitude does not: the H3 loop overestimates the measured one about 4 times** (0.6 / 0.145
= 4.1, and the declared criterion asked for a ratio between 1/3 and 3). The excess of arrivals that the measured loop produces in this
call center is 2.3% of the arrivals, against the 29% bias that the declared loop generated in H3. That means the
performative correction of G4 was tested against a loop much stronger than the one observed in a real case. There are two
weighty caveats. A new call is only visible when it enters the queue, so 0.145 is probably a lower
bound. A crude correction, made after seeing the result, takes the value to 0.16, still far from 0.6. And the
call center is from another bank, another country and another year than those of the H3 call center.

## What H3 declared

`h3/estocologia/performativo.py` builds the loop over the real arrivals of `bank_calls`. Staffing comes from the forecast
by Erlang C. Abandonment follows M/M/c+M with patience of 180 s, assumed. **Whoever gives up redials with
probability r = 0.6**, 60% in the following 5-min interval and 40% in the other (`DIV = (0.6, 0.4)`, line 36).
Whoever is served does not return. In the simulated loop, the 2SLS with the randomized shocks recovered r̂ = 0.563, with 90% CI of
0.51 to 0.61, and the naive forecaster overestimated exogenous demand by 29.1% (`h3/saida/portoes.json`, lines
276 and 283). The H3 README itself says it is "a declared loop over real arrivals, not a measurement of the world".

The parameter measured here is the same r: the causal effect of abandoning, versus being served, on the chance of a
new call shortly after. In H3 it equals P(new call | abandoned), because P(new call | served) = 0.

## Search for the data

What I searched, with what the network reaches (raw.githubusercontent.com, GitHub code search, the Rdatasets
index):

- **Rdatasets index (3,648 data sets).** I searched call, queue, wait, abandon, redial, energy, electric,
  nudge, disclos, experiment, field, recommend, lottery and random. I found field experiments with information or
  intervention (for example `causaldata/thornton_hiv` and several from `experimentr`) and aggregate call
  series (`fpp2/calls`, the same family as the H3 `bank_calls`). None has a published forecast or action
  that changes the behavior predicted in the H3 channel, which is staffing → wait → abandonment → new call.
- **GitHub code search** for `vru_entry q_time outcome HANG AGENT`. It found mirrors of the Technion SEE Lab
  AnonymousBank 1999, the classic call-center data set with a per-call record and customer identification.
  That is the only reachable data with the exact mechanism of H3.
- **I did not try** recommendation logs with randomized exposure (Open Bandit Dataset, KuaiRand) nor
  social-norm experiments in energy (Opower, which is not public). They measure another loop, which is not the one
  declared in H3.
- **I did not find** any call-center data with staffing varied by randomization, nor with randomized announced
  waiting time.

The data was downloaded from two independent mirrors and checked against each other (`carregar.py`). Each has 444,448
rows, and 444,436 pair by key. After normalizing the leading zeros, the fields coincide in more than
99.98%. The origin, the hashes and the warnings are in
`/mnt/project-files/estocologia/data/empirical/p06-laco/FONTES.md`.

## Design (pre-registered in [DECLARACAO.md](DECLARACAO.md), SHA-256 in `DECLARACAO.sha256`)

- **Index call:** identified call that entered the queue, outcome AGENT or HANG, type different from TT.
  There are 198,980 calls, 36,658 of them abandonments, from 12,511 customers over 360 days.
- **Treatment:** abandoned, versus was served.
- **Instrument:** number of other calls in the queue at the instant the index call enters it. The mean is
  3.1, the median 2 and the q90 7.
- **Fixed effects:** date × hour × priority × service type. Within the cell, the queue found comes from the
  arrivals and the services of third parties.
- **Outcome:** another call from the same customer within 15 min after the end of the index call. The 60 min
  and same-day windows are secondary.
- **Margin:** the day is the independent unit. Bootstrap of 1,000 days with `default_rng(2026)`.

This is a do(abandonment) approximated by quasi-random variation, not a randomization. Alongside goes the passive
reading, OLS with the same fixed effects, which is the comparison that AGENTS.md forbids confusing with effect.

## Numbers (`resultados.json`)

| | effect on P(new call) | 90% CI (days) | Passive OLS | First-stage F |
|---|---|---|---|---|
| **main, 15 min** | **0.145** | 0.132 to 0.157 | 0.129 | 5,523 |
| robustness 1: control for the queue in the 15 min after | 0.134 | 0.121 to 0.146 | 0.126 | 4,413 |
| robustness 2: instrument log(1 + queue) | 0.142 | 0.129 to 0.153 | 0.129 | 3,128 |
| secondary, 60 min | 0.218 | 0.200 to 0.235 | 0.165 | 5,523 |
| secondary, same day | 0.262 | 0.241 to 0.281 | 0.151 | 5,523 |
| **H3 declared** | **0.6** | (simulated r̂ 0.563, 0.51 to 0.61) | | |

**Declared criterion:**
- **Sign.** The CI lies entirely above 0, so the sign checks out.
- **Order of magnitude.** The ratio 0.6 / 0.145 is 4.1, outside the 1/3 to 3 band, so the order does not check out.
- **Strict compatibility.** The value 0.6 lies outside the 90% CI.
- **Robustness.** Robustness 1 changes neither the sign nor the band, so the result is not fragile.
- **Declared verdict:** "the H3 loop overestimates the measured one".

**Raw rates.** In 15 min, 21.2% of the abandoners call again, against 7.2% of the served. In 60 min it is
33.8% against 15.4%, and until the end of the day 49.8% against 32.9%. The served also return, which H3 assumes
does not happen. The raw difference in 15 min (0.140) stays close to the estimate by instrument (0.145). Here
selection by patience weighs little in that window, but it weighs in the same day, where OLS gives 0.151 and IV 0.262.

**Declared secondaries:**
1. Of the abandoners' redials within 60 min, 63% arrive within 15 min. H3 assumes 100% in two intervals.
2. With r = 0.145, the 63,203 abandonments in the queue over 405,358 attendant requests generate an excess of 2.3%
   of observed arrivals (CI from 2.1% to 2.5%). In H3, the declared loop produced a bias of 29.1% of the naive
   forecaster. The two numbers do not measure exactly the same thing, because the H3 one also depends on the simulated
   abandonment rate. Even so, the distance is one order of magnitude.

**Informative, done after seeing the result (`posthoc.py`, `posthoc.json`; does not decide).** Requests served
without a queue almost never have an identifier: the fraction is 0.00002, against 0.84 for those who went through the queue. In the 15 min
after an abandonment, 88.8% of the requests enter the queue. If the new calls entered the queue in that same
proportion, the corrected effect would be 0.145 / 0.888 ≈ 0.16.

## How to reproduce

```
python3 -I empirical/p06-laco/carregar.py   # checks the two mirrors
python3 -I empirical/p06-laco/estimar.py    # resultados.json (~35 s)
python3 -I empirical/p06-laco/posthoc.py    # posthoc.json (informative)
sha256sum -c empirical/p06-laco/DECLARACAO.sha256   # run from inside empirical/p06-laco
```

The data lives in `/mnt/project-files/estocologia/data/empirical/p06-laco/`, with `SHA256SUMS`. The
`estimar.py` ran twice, and the two `resultados.json` outputs are identical byte for byte.

## Limits

- **It is not a randomization.** Exclusion assumes that the queue found changes the chance of a redial only through abandonment.
  There are at least two paths outside it. Whoever is served after a long wait may return more. And the queue
  persists, which makes a new call be seen more often when the queue was long. Robustness 1
  controls the second path in part and lowers the estimate a little, from 0.145 to 0.134.
- **Measurement.** Only a new call that enters the queue is seen, and only from identified customers. Unidentified
  customers (customer_id = 0) and the 300 corrupted identifiers stay out.
- **External validity.** It is another bank (Israel, 1999, about 8 service positions) and not the call center of the
  `bank_calls` (USA, 2003). That informs the plausible order of magnitude of r, but not the r of that call center.
- **Local estimate.** The 2SLS measures the effect among those who abandon because of the longer queue. It is the group that
  matters for staffing, but it is not every abandonment.
- **Patience and timing.** The patience of 180 s assumed in H3 was not tested here.

## What to measure in a real operation (what it would need to have)

To measure the loop for real, as AGENTS.md asks, with do(a) against passive observation:

1. **Write access to staffing**, or to an equivalent parameter such as routing, overflow or announced waiting
   time. With that, randomize, in 5% to 10% of the 15-min intervals, a deviation of ±15% to ±30% in the
   planned staffing. The randomization is recorded beforehand, with seed and hash, as in the simulated G4.
2. **Per-call record with a stable caller identifier** (ANI or customer) in all calls,
   served with or without a queue, because the AnonymousBank defect is precisely not having that. Also record the
   queue entry and exit and the outcome.
3. **Record of the system actions:** planned staffing, effective staffing and forecast issued, with the instant of
   each one, for each interval.
4. **Estimation.** The 2SLS of `performativo.py` (v4) with the randomized shocks as instrument and the day as
   unit, alongside the same r measured at the individual level. The comparison of that r with the one of this measurement, 0.15 to 0.26
   depending on the window, and with the declared 0.6 says which loop holds for that call center.
5. **Size.** In AnonymousBank, with about 550 index calls per day (198,980 in 360 days), the 90% CI of the effect is about
   ±0.013 with 360 days. A staffing randomization directly in the operation should be stronger than the natural variation
   of the queue. The power for a given number of days has to be computed in the pre-registration of the operation,
   not assumed here.
