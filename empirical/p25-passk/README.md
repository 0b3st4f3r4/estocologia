# Point 25: reliability is not average success

**Status on 2026-10-19: run once, after the seal. All three rules hold.** The declaration (`DECLARACAO.md`) and the
instrument were sealed in `DECLARACAO.sha256` and committed (`efe2b6a`, 21:05 UTC) before
`cargo run --release --bin labs -- --passk` was first run. The output of that run is `saida.txt`, verbatim.

## Verdict, by the sealed rules

- **R1 holds.** In world A, where every task has chance 0.5, the band of the gap pass^k − (pass^1)^k contains zero at
  every k: without spread in the chances, the laboratory invents no gap.
- **R2 holds.** In worlds B, C and D the band of the gap lies above zero at every k from 2 to 8, including k = 2 in
  world B, where the gap is smallest (band [+0.0369; +0.0619]).
- **R3 holds.** The exact Beta moment E[p^k] lies inside the band of the estimator at every k in all four worlds.

## What the run adds that the rules did not ask

The bands are wide. In world C, one run of a laboratory of 200 tasks and 8 runs puts pass^8 anywhere from 0.2050 to
0.3300, and pass^1 from 0.4419 to 0.5587; in world B, pass^8 goes from 0.0250 to 0.0850, a factor of more than three.
The first replicate, the one a benchmark would publish, sits above the exact value in B, C and D (0.0700 against
0.0545, 0.3100 against 0.2672, 0.3200 against 0.2976), by the luck of the draw: a single published pass^k is a draw
whose spread nobody printed.

K = 1. Nothing was changed after the run.
