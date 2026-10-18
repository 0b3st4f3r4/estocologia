# Criteria of the H3 exit gate (declared before opening any vault)

Declared on 2026-10-14, before any development replay. The SHA-256 hash of this file enters
every pre-registration, so changing one line afterwards changes the hash that opens the vault, and the swap becomes visible.

docs/PLAN.md, H3: "empirical calibration proven out of the sample (out-of-time), zero temporal leakage and control
of performativity/Goodhart", under continuous operation with F3 (KS/Wasserstein drift), F5/F6 (privacy and
fairness) and the F8 circuit breakers.

## Streams

Six real streams with a timestamp, each with a vault cut out of time fixed here:

| stream | data | forecast | type | vault (t_disp ≥) |
|---|---|---|---|---|
| energy | vic_elec, demand every 30 min, 2012–2014 | demand of the 48 half-hours of the next day, issued at midnight UTC | quantiles 5/50/95% | 2014-01-01 |
| call centre | bank_calls, calls every 5 min, 2003 | calls of the 169 intervals of the next business day, issued at the end of the day | quantiles 5/50/95% | 2003-09-01 |
| stock market | DJIA daily, 1885–2023 | return of the next session | quantiles 1/5/50% (VaR) | 2000-01-01 |
| flights | nycflights13, 2013 | P(arrival delayed > 15 min or cancelled), issued 2 h before the scheduled departure | binary | 2013-10-01 |
| cyclones | storms (NOAA HURDAT2), 1975–2022 | P(wind rises ≥ 30 kt in 24 h), issued at every observation | binary | 2015-01-01 |
| arrests | Toronto Arrests, 1997–2002 | P(release) of each detainee, model trained only on the previous years | binary | 2001-01-01 |

The official forecaster of each instant is the model, or the deterministic fallback when the F8 circuit breaker is open
or the input leaves scope. **The official forecast is the one that gets scored.** Refusing or abstaining does not remove the forecast from the
scoreboard: the fallback's takes its place.

## G1. Simulability (inherited)

pytest passes, and a second clean run reproduces the results identically.

## G2. Out-of-time calibration (PLAN)

It holds for every official forecaster, in the vault of **all** six streams.

- **Binary (flights, cyclones, arrests):**
  - |p̄ − ȳ| ≤ 0.02;
  - ECE (10 equal-mass bins) ≤ 0.03;
  - Brier skill > 0 against the moving climatology, which is the base rate of the past available at the instant of
    issue.
- **Quantiles (energy, call centre):**
  - coverage of the 5–95% interval between 0.86 and 0.94;
  - interval score skill > 0 against the naive seasonal fallback.
- **Stock market:**
  - violation frequency of the 99% VaR (return below the 1% quantile) between 0.5% and 1.5%;
  - Kupiec test with p > 0.05.

  Christoffersen independence is reported, without being required.

## G3. Zero temporal leakage (PLAN)

All the conditions below have to hold:

- **(a) Reads.** No read returns an event with t_disp above the request or above the clock.
- **(b) Forecasts.** Every forecast was issued with a state formed only by events with t_disp ≤ t_emissão, and the
  event that resolves it has t_disp > t_emissão. Zero violations in both counts.
- **(c) Vault.** No read of the vault before the pre-registration, and no second opening.
- **(d) Panel.** All the attacks are refused:
  - reading the future;
  - reading the vault early;
  - opening the vault with the wrong agent;
  - opening the vault twice;
  - running SELECT *;
  - setting the clock back.
- **(e) Canary.** In each binary stream, a column equal to the label is published at the instant the label
  becomes knowable.
  - With the honest clock, the forecaster that uses it gains no more than 0.02 of skill over the forecaster without it.
  - With the clock switched off (positive control), it gains more than 0.2. The positive control shows that the canary
    would detect a leak.

## G4. Performativity and Goodhart (PLAN)

None of the real streams records the actions of the system itself, so performativity is not measurable in the
data. The test is a declared loop: the real arrivals of the call centre are the first exogenous attempts. The scale
comes out of the forecast. Those who give up re-dial with probability r, and the re-dials enter the observed count.

All the conditions below have to hold:

- **(a) Detection.** With random exploration of the scale in 10% of the intervals, the sensor of lens M11 detects the
  dependence of the observed arrivals on the scale when r = 0.6 (p < 0.01). When r = 0, it does not
  detect it (p > 0.05).
- **(b) Control.** With r = 0.6, the forecast corrected by the estimated effect errs the exogenous demand by at most
  3% (mean relative bias in the evaluation period). The bias of the naive forecast, trained on the observed count,
  is reported.
- **(c) Goodhart: panel against the scoreboard.** All the attacks are neutralised:
  - The forecaster that abstains when it is uncertain receives exactly the fallback's score at those instants, and the
    abstention does not put it ahead of the honest forecaster.
  - The forecaster constant at the base rate receives weight 0 in the diary.
  - The second opening of the vault is refused.

  The Goodhart index (skill in development minus skill in the vault) is reported per stream.

## G5. Scan containment (inherited from H2)

All the conditions below hold:

- No SELECT * executed.
- Every form of query has the projection and the t_disp filter confirmed in the EXPLAIN.
- Each table is scanned at most 3 times in the whole replay (bytes scanned / bytes of the table ≤ 3).
- The spend per agent stays within the budget.

## Informative

- **G6, F5/F6.**
  - k-anonymity.
  - ε budget of the releases with differential privacy, with a privacy circuit breaker.
  - Bits lost.
  - Fairness trilemma per year in the arrests stream.
- **G7, operation.**
  - Drift alarms in development and in the vault.
  - Circuit-breaker openings.
  - Scope refusals.
  - Decisions altered by the C/L threshold.
  - Hypergraph nodes, with the PLAN target of 10⁶.
- **Trail.** The hash chain of the bus of each stream must be intact. This item is required.

## Exit rule

H3 exit = G1 ∧ G2 ∧ G3 ∧ G4 ∧ G5 ∧ Trail.

The H2 gate (ICP with Ŝ ≠ ∅ in 3 areas) remains open. H3 does not depend on it, and the report says so.

## Development and K

Before the cut, each stream may be adjusted at will. Each distinct specification run in development
is recorded (hash of the stream module) and counts in the K of the stream. After the pre-registration, nothing in the stream changes.
