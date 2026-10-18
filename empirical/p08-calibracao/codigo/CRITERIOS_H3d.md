# H3d: declaration before running (arrests, second test with new data)

Written on 2026-10-15, before any replay of these data. The SHA-256 of this file enters the pre-registration of the
new stream.

## The request and what is already known

sil said "Continue with H3". The only pending item of H3 is the arrests.
- In 1997–2002, the ECE was 0.036.
- In H3c (Toronto 2020–2021), the ECE was 0.067, with |p̄ − ȳ| = 0.0002. The model was **little confident**: a
  slope error, which the intercept recalibration does not correct.

These results stay as they are. There are no Toronto arrests after 2021 within reach of this network, so the new
vault comes from another base: **Rhode Island, state police traffic stops, 2005–2015** (Stanford Open
Policing Project, ODC-By; mirror on GitHub; see `../data/h3c/FONTES.md`).

**What I saw of these data before declaring.** I saw the schema, the category levels and, unintentionally, the overall
counts of `stop_outcome` in the whole base. Those counts give a base rate of driver arrest around 3%.
I saw nothing by year, by race or by vault period.

## The lesson of H3c, turned into a specification (declared here, tested only in this vault)

- **Online Platt recalibration.** The official forecast is p' = σ(a + b · logit p). At each resolved label, (a, b)
  take a gradient step on the log-loss: a ← a + η (y − p'), b ← b + η (y − p') · logit p, with η = 0.002 and
  start a = 0, b = 1. It replaces the recalibration of the intercept alone and also corrects the slope.
- **Smaller penalty.** The `logistica` now uses λ = 1e-4 (in H3c it was 1e-3). The inferred cause of H3c was the
  excessive shrinkage.

## Design of the `prisoes_ri` stream (fixed here)

- **Question.** Will the stopped driver be arrested? It is the custody decision at the stop.
- **Label.** y = 1 if `stop_outcome` = "Arrest Driver". The rows without an outcome (the roughly 5,300 empty records)
  drop out before anything else. The arrest of a passenger does not count.
- **Time.** `stop_date` + `stop_time` (local time, treated as UTC; only the order matters). The label becomes known one
  day later. The clock advances every 6 h.
- **Features, all known at the stop:** sex, age (standardised, with the square), `violation_raw` (12
  categories), time of day (4 bands) and day of the week.
- **Out of the model:**
  - race (`driver_race`), because the model is race-blind; it enters only in F6;
  - `search_conducted`, `search_type`, `drugs_related_stop` and `stop_duration`, which are recorded during or after
    the decision (leakage);
  - `county_name`, which is empty.
- **Model.** Logistic regression, λ = 1e-4. It is refitted on 1 January only with the previous years, including in the
  refit by drift, and goes through the Platt recalibration above. The raw model stays in the diary.
- **Fallback.** The y rate of the previous year.
- **Scope.** The model holds if the `violation_raw` category appeared in training.
- **Circuit breaker and sensors.** The parameters of the `prisoes` stream. The sensors are age and the standardised residual.
- **Declared decision.** Flag the case for review if p(arrest) > 0.10.
- **Calendar.**
  - 2005 is training only; the start is 2006-01-01.
  - **The vault cut is 2013-01-01**: the vault is 2013–2015.

## Criterion

**(a) The G2 of H3, without change.** |p̄ − ȳ| ≤ 0.02, ECE (10 equal-mass bins) ≤ 0.03 and Brier skill > 0
against the moving climatology.

**(b) A relative criterion, stricter, which is also mandatory.** With a base rate near 3%, the absolute
tolerances of (a) test almost nothing; a constant forecaster would meet them. That is why, for this stream, I also
require:
- |p̄ − ȳ| ≤ 0.2 · ȳ;
- ECE ≤ 0.3 · ȳ;

with ȳ being the rate observed in the vault. This makes the test harder, not easier.

The following also hold: G1 (pytest and identical clean re-execution), G3 (zero leakage, with the canary and the positive control), G5
(at most 3 scans per table) and the trail.

F6 (Black against White trilemma, by vault year) and F5 (releases with differential privacy by race and year)
are reported as informative.

## Rule

- If `prisoes_ri` passes in (a), (b), G1, G3, G5 and the trail, the arrests pending item is closed and the H3 exit
  is approved. The question (arrest at a traffic stop) is not identical to the previous ones, and that is recorded
  for sil.
- If it fails, the pending item stays open and nothing is adjusted after seeing the vault.

This is the K = 1 of `prisoes_ri`. No alternative was run.
