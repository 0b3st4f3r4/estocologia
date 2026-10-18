# Point 19 — the second opinion on the results marked P

**State: closed with the limit inside. 5 lines checked by a second implementation, 2 with the verdict re-derived, and 7
that remain only as a record.**

The empirical registry has fourteen lines with the **P** mark — results that came from the retired Python scaffolding, with
no Rust instrument to measure them again. This front gives the second opinion where it is possible: where the data or the
machine record stayed in the tree, a second implementation redoes the arithmetic by a different path and checks it.

**The ones that checked out.** The `α/M` thresholds of p03 are exactly 0.05 divided by M. The bias of the naive
predictor of the loop (0.29124674633711956) appears in **two independent files** and agrees digit by digit. The base of
the bank was recounted in Rust and matches the published one, with a difference of 0.2% that is the laboratory's window
of hours — attributed and quantified, not a disagreement. The opinions were counted: 30 lines in the rereading and 4 files.

**The divergence, declared and unresolved.** The published registry says "33 of the 33 identical" by the criterion of the
0.05 SD band; the machine field `reproducao_rodada_a` has 33 domains and marks `true` in **8**. The field does not declare
its own meaning, so the second opinion records both values instead of guessing. The decision is human.

**The ones without a second opinion.** Seven lines — the cohorts, the Brier of the fires, the tunnel of light, the
pharmacology, STAR, the six flows, and the hypergraph — depend on data that left the tree with the scaffolding. They stay
declared, with the reason, instead of becoming a promise.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --segunda
~~~

**The limit.** There is no second opinion for what has no data. The K of the front is 1.
