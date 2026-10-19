# P22 · A stopping rule that survives the path

**Status: rule declared and implemented; retroactive application declared (DECLARACAO_2.md, 2026-10-19) and run by `labs --eprocess`.**

From 2026-10-18 on, a gate attempted in more than one round keeps an e-process ledger
(`crates/stochology-methods/src/sequential.rs`): α fixed before round 1, one e-value per round, the running product over
fresh data or the running mean over reused data, and the gate passes at the first round whose accumulated evidence
reaches 1/α. Ville's inequality makes that valid at any stopping time, including "stop when it passes", which is the
rule the seven rounds of H2 actually followed without declaring it.

Proof: `cargo test -p stochology-methods` (nine tests in `sequential.rs`). The recomputation of the 38 tests of H2 was
declared in `DECLARACAO_2.md` (sealed before the run) and is printed by `labs --eprocess` from `testes.tsv`, the 38
distinct tests copied from the multiplicity record plus the two clustered rows the declaration admits: one ledger per
area, the mean of the e-values, two readings. Three areas cross 1/α (education, pharmacology, physics); genetics does
not (mean e 7 against 20); the clustered row of education does not move its verdict, because the mean also carries the
earlier rounds on the same data. Item 15 of `docs/DECLARED_PROPOSALS.md` closed with it; the limits are in §5 of the
declaration.
