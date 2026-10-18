# Declaration of point 19: the second opinion on the results marked P

**Frozen on 2026-10-16T23:31:41Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `empirical/p19-segunda-opiniao/registro.tsv` | `e7034ec098d2b70f3344e3b426043a28ade28f1511027016c6c12a6e7683bb49` | the fourteen lines marked P, with the class and what the second opinion does |
| `DECLARACAO.md` (this one) | — | the five declarations; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `325928f7d3d25dc3949a0a4ebf16fef089097aff937b7a142a4cbbeb1cd94e17` | the result in one page |
| `DECLARACAO.sha256` | — | the three hashes, in `sha256sum` form |

## Declaration 1 — the three classes, and what makes a line checked

**A line is `conferida` only when a second implementation redoes the arithmetic by a path different from the first,
with the number recorded as a witness.** It is `derivada` when the verdict was re-derived from a record in the tree
(points 14 and 16), and `registro` when the raw data is no longer in the tree and there is no possible second opinion.
Nothing is promoted by will: the class comes out of what the instrument managed to do.

## Declaration 2 — the five checked, with the result

| line | what the second opinion redid | result |
| --- | --- | --- |
| multiplicity (K = 38) | the `α/M` thresholds recorded in the p03 registry, recalculated as 0.05/M in the exact form of the file | **checks** in the three values: 0.05/32 = 0.0015625, 0.05/38 = 0.0013157894736842105, 0.05/72 = 0.0006944444444444445 |
| performative loop (0.145) | the bias of the naive predictor, sought in **two independent files** | **checks digit by digit**: 0.29124674633711956 in p03 and in p06 |
| real H0 (2.65 M and Var/mean 273) | the base recounted in Rust by its own path, with the dispersion between days in a single pass | **checks with an attributed difference**: vault 2 654 950 against 2 649 354, Var/mean 261.7 (all) and 247.4 (vault) against 261.1 and 246.7. The difference of 0.2% is the **window of hours** that the laboratory uses (7 to 20 h) and the recount does not |
| opinions | the lines of the rereading and the reissued files, counted | **30 lines** in the table and **4 opinions** in the folder. The note: the registry speaks of 21 opinions, and the table has the 21 decisions plus the closing lines — the two numbers measure different things |
| priors (33 of 33) | the reproduction of p05, read from the machine record | **33 domains, of which 8 mark `true`** — see declaration 3 |

## Declaration 3 — the declared divergence, and it is not resolved here

The published registry says, in the background audit line, **"33 of the 33 identical"**, by the criterion of the
equivalence band of 0.05 SD fixed in the `DECLARACAO_2.md` of p05, with a maximum deviation of 0.0071 SD. The machine field
`reproducao_rodada_a` has **33 domains and marks `true` in 8 of them**.

**The field does not declare its own meaning** — whether `true` means "reproduced" or "diverged" —, and that is why the
second opinion records the two values and **does not resolve the divergence**: resolving would be guessing the meaning of a
field, and guessing is not a second opinion. What is known stays declared: the published sentence relies on the band of
0.05 SD, and the machine field **does not sustain that sentence on its own**. The decision is human, and while it does not come, the
line carries both readings.

This **does not bring down the verdict of p05** — the verdict is about the band, and the band is from the document —, but it shows one
thing that the program should record: a machine field without a declared meaning is not proof of anything.

## Declaration 4 — the seven that remain only as a record, and the two derived

Seven lines stay in `registro`, all for the same reason: **the raw data left the tree with the Python scaffolding**
(`docs/declarations/REMOVED_NON_SIGNIFICANT.md`). They are the 20-year cohorts, the Brier of the fires, the tunnel of light, the
pharmacology trial, the STAR education, the six flows of H3, and the hypergraph. Two stay in `derivada`: the
replica of genetics (the design is measured in point 14) and the blind pilot (the scores of the judge are re-derived in point
16). For none of them is there a possible second opinion without recovering the data — and that is what is written.

## Declaration 5 — the limit and the K

**The limit, declared like that of Φ5.** Fourteen lines marked **P**, five with a second implementation, two with the
verdict re-derived, and seven that remain a record. The item closes **with the limit inside**: there is no second opinion
for what has no data, and the promise of a future second opinion would be the eternal pending item that this program
rejects.

**The K.** The classes and the treatment of the divergence are a choice of specification, made once: **K = 1**.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-book` | 43 tests passed, among them the six of `segunda`: the α/M thresholds, the 33 domains with the declared divergence, the two sources of the loop, the recount of the bank against the published one, the count of the opinions, and the classes of the registry (0) |
| `labs --segunda` | the fourteen lines with the class, the five measured second opinions, and the seven that remain only as a record (0) |
| `python3 scripts/book/generate_results.py --check` | `docs/RUST_RESULTS.md` updated, with the block of the second opinion (0) |
