# Point 27: combining models when none is true

**Status on 2026-10-19: run once, after the seal. All three rules hold.** The declaration (`DECLARACAO.md`) and the
instrument were sealed in `DECLARACAO.sha256` and committed (`ee45dc1`, 21:29 UTC) before
`cargo run --release --bin labs -- --stacking` was first run. The output of that run is `saida.txt`, verbatim.

## Verdict, by the sealed rules

- **R1 holds.** In the M-open world stacking scores −2.0811 per test point against −4.8251 for model averaging; the
  paired difference is +2.7441 with interval [+2.7007; +2.7874].
- **R2 holds.** Listing the left model twice leaves stacking's score unchanged to five decimals in both worlds and its
  left-family weight at 0.499 and 0.998; model averaging's score moves by 0.03748 per point in the M-open world.
- **R3 holds.** In the M-closed world the two differ by 0.0013 per point.

## What the run adds that the rules did not ask

The difference in the M-closed world has an interval entirely below zero, [−0.0016; −0.0010]: when one candidate is
true, stacking pays a small, measurable price for not assuming it. In the M-open world model averaging put more than
0.99 on one expert in 926 of the 1,000 datasets, and its score is barely above that of the best single expert
(−4.9817).

K = 1, with the development change before the run that the declaration names.
