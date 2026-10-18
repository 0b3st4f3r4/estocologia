# P22 · A stopping rule that survives the path

**Status: rule declared and implemented; retroactive application declared and not done.**

From 2026-10-18 on, a gate attempted in more than one round keeps an e-process ledger
(`crates/stochology-methods/src/sequential.rs`): α fixed before round 1, one e-value per round, the running product over
fresh data or the running mean over reused data, and the gate passes at the first round whose accumulated evidence
reaches 1/α. Ville's inequality makes that valid at any stopping time, including "stop when it passes", which is the
rule the seven rounds of H2 actually followed without declaring it.

Proof: `cargo test -p stochology-methods` (nine tests in `sequential.rs`). The recomputation of the 38 tests of H2
waits for a declaration that fixes, before computing, which null each round tests and which data it used (item 15 of
`docs/DECLARED_PROPOSALS.md`).
