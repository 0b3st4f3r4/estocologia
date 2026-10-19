# Point 28: calibrating is not understanding

**Status on 2026-10-19: run twice, each after its seal. All three rules hold in both runs.** Declaration 1 and the
instrument were sealed and committed (`879cd24`, 21:34 UTC) before the first run (`saida.txt`). Declaration 2, which
adds a column and changes no rule, was sealed and committed (`087fadd`, 21:39 UTC) before the second run
(`saida_2.txt`). K = 2.

## Verdict, by the sealed rules

- **R1 holds.** Without discrepancy θ comes out at 0.5647 with standard deviation 0.0039; its interval covers 0.65 in
  none of the 1,000 datasets, while the fit misses the data by 0.0677 against a noise of 0.05.
- **R2 holds.** With the discrepancy that is zero and flat at the origin, θ is 0.6444 with standard deviation 0.0767,
  and the interval covers the truth in 0.953 of the datasets.
- **R3 holds.** With the discrepancy free at the origin, the standard deviation of θ is 0.7081, nine times the
  constrained one, and its mean, 0.3232, is half of the slope.

## Why there is a second run

The first run printed only the mean error of the prediction at x = 8. A drafted reading took the small mean errors of
the fits with a discrepancy (+0.236 and +0.101) for good predictions; the reimplementation found +0.516 and +0.382, and
exact rational arithmetic showed that both instruments were right and that the difference is sampling: those
predictions have a spread of about 6.9 per dataset. The second run prints the root mean square error beside the mean:
0.804 without discrepancy (all of it bias) and 6.988 and 6.998 with it (almost all of it spread). The drafted reading
was never published; the one in Chapter 14 reads the second run.
