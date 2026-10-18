# H3c: declaration before running (arrests with new data)

Written on 2026-10-15, before any replay with the new data. Of these data only the schema was seen: columns,
types, row counts per quarter and the levels of the categorical variables (sex, age band, perceived race,
division, occurrence category). No outcome rate, calibration or cross-tabulation by race was computed. The
SHA-256 of this file enters the pre-registration of the new stream.

## The request and what stays open

sil asked to "find new data: arrests after 2002". The `prisoes` stream of H3 (Toronto, 1997–2002) failed G2
with ECE 0.036 against the tolerance of 0.03. That result stays as it is; nothing in it is redone.

The new test uses data nobody has seen: **Toronto Police Service, "Arrests and Strip Searches" (RBDC-ARR-TBL-001),
2020–2021**, 65,276 arrests, mirror on the GitHub of official data of the Toronto portal (see
`../data/h3c/FONTES.md`, with hashes). The official portals are blocked by this network.

**It is not the same question.** The 1997–2002 stream predicted the release with a summons of detainees for possession of
cannabis. The new one predicts whether the person arrested for any occurrence is booked at the station within 24 h. The
two are the same custody decision (release on the spot or take to the station), in the same city, but the population and the
period change. That is why the new stream is called `prisoes20` and is evaluated on its own account.

## Design of the `prisoes20` stream (fixed here)

- **Label.** y = 1 if Booked = 1 **or** StripSearch = 1. The official metadata warn that, due to a defect in the
  form, there are records with StripSearch = 1 and Booked = 0, and that in those cases one should presume that there was a
  booking at the station. The rule follows the warning.
- **Time.** The base only carries the year and the quarter. The instant of each arrest within the quarter is synthetic and
  deterministic, as in the 1997–2002 stream: the rows are spread uniformly over the quarter, in the order of the
  file (ObjectId). The label becomes known one day after the arrest.
- **Features, all known at the moment of the arrest:**
  - sex;
  - age band (the duplicated spellings are merged; a missing band becomes its own category);
  - occurrence category (missing becomes its own category);
  - police division;
  - the six behaviour flags at the arrest (Actions_at_arrest_*);
  - previous arrests of the same person (PersonID) in previous quarters, counted by the stream itself from
    what it has already consumed, as min(n, 5)/5 and the square of that.
- **Out of the model:**
  - perceived race: the model is race-blind, as before, and race enters only in F6;
  - StripSearch, SearchReason_* and ItemsFound, which are recorded after the decision (leakage);
  - EventID, ArrestID and ObjectId.
- **Model (F4).** Logistic regression with the `fluxos.base.logistica` penalty (λ = 1e-3). It is refitted at the
  start of each quarter only with the previous quarters, including in the refit requested by the drift.
- **Online recalibration of the intercept, the same as H3b.** p' = σ(logit p + δ), δ ← δ + 0.002 (y − p'), δ initially
  0. The raw model stays in the diary and is reported alongside.
- **Fallback.** The y rate of the last complete quarter.
- **Scope.** The model holds if the occurrence category and the division appeared in training. Outside that the
  fallback holds.
- **Circuit breaker and sensors.** The parameters of the `prisoes` stream do not change: window 400, margin 0.03, k_on 3, k_off 3,
  interval 50. The sensors are the previous arrests (in place of "checks", which does not exist here) and the standardised
  residual.
- **Declared decision.** Flag the case for review if p(booking) > 0.30, the mirror of the release threshold of 0.70 of the
  old stream.
- **Calendar.**
  - 2020-Q1 is training only; the start is 2020-04-01.
  - **The vault cut is 2021-07-01**, so that the vault is the 2nd half of 2021, about 16 thousand arrests.
  - The vault stays locked until the pre-registration, which carries the hash of this file.

## Criterion: the same as G2 of H3 for binary streams

In the vault, with the official forecast (the recalibrated model, or the fallback when the circuit breaker or the scope so dictate):
- |p̄ − ȳ| ≤ 0.02;
- ECE (10 equal-mass bins) ≤ 0.03;
- Brier skill > 0 against the moving climatology.

The following also hold for this stream:
- **G1:** pytest plus an identical second clean run;
- **G3, zero leakage:** reads, state, label, pre-registration, and the canary with the positive control (honest gain
  ≤ 0.02, leaked > 0.2);
- **G5:** at most 3 scans per table;
- intact bus trail.

F6 (trilemma by perceived race, Black against White, per half-year) and F5 (releases with differential privacy
by race and quarter) are reported as informative, as in H3.

**Declared caveat.** The same person appears in several rows (37,347 people in 65,276 arrests). The ECE and the
CITL are averages over arrests and do not get a CI here; the dependence is reported, not corrected.

## Rule

- If `prisoes20` passes G1, G2, G3, G5 and the trail, the arrests pending item of H3 is given as closed with new
  data, in the same way the flights were closed with 2023 in H3b. With that the H3 exit is approved. Since the
  question is not identical (see above), that reading is recorded for sil to confirm.
- If it fails, the pending item stays open and nothing is adjusted after seeing the vault.

This is the K = 1 of `prisoes20`. No design alternative was run before.
