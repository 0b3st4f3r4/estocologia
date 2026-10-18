# H3b: declaration before running (H3 pending items)

This declaration was written on 2026-10-14, before any replay with the 2023 data. Until then, of these data
only the header and the column types had been seen. The SHA-256 hash of this file enters the pre-registration of the
new stream.

## What failed in H3 and what is tested here

**Flights.** The 2013 stream failed G2 with |p̄ − ȳ| = 0.027 in the Oct–Dec vault, and the tolerance was 0.02. The
design had no mechanism that corrected a deviation of the mean: the circuit breaker compares with the fallback, not with
the calibration.

The new mechanism, declared here, is the **online recalibration of the intercept (F4/F3)**:
- the official forecast of the model becomes p' = σ(logit p + δ);
- at each resolved label, δ ← δ + η (y − p'), with η = 0.002 and δ initially 0;
- the fallback does not change.

The mechanism cannot be evaluated honestly in the 2013 vault, which has already been seen. The test is done in a
new vault: **nycflights23**, all the NY flights in 2023 (CRAN package nycflights23, cran/ mirror on GitHub).
- The model, the features, the fallback, the circuit breaker and the sensors are those of the `voos` stream, with no other change.
- The start is 2023-01-22.
- The vault cut is **2023-10-01**: the vault is Oct–Dec 2023, the same quarter that failed in 2013.

**Arrests.** The stream failed with ECE 0.036, in a tolerance of 0.03 and with 1,488 cases. There are no Toronto arrest data after
2002 within reach of this network. The pending item stays open and declared. It is not "resolved"
by loosening the tolerance after seeing the result.

## Criterion (the same as G2 of H3, for binary streams)

In the 2023 vault, with the official forecast (recalibrated model, or fallback when the circuit breaker or the scope so dictate):
- |p̄ − ȳ| ≤ 0.02;
- ECE (10 equal-mass bins) ≤ 0.03;
- Brier skill > 0 against the moving climatology.

The following also hold, for this stream:
- **G3, zero leakage:** reads, state, label, pre-registration, and the canary with the positive control;
- **G5, scan containment:** at most 3 scans per table.

G1 is pytest plus an identical second clean run.

The result without recalibration (the raw model in shadow, which stays in the diary) is reported alongside. That shows whether it was
the mechanism that made the difference.

## Rule

H3 comes to have the gates G2–G5 approved for the flights if the 2023 stream passes. The arrests pending item
remains open and blocks the H3 exit, unless sil decides otherwise.
