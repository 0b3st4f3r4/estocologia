# Point 20 — the Jev-LLM bridge

**State: the framework closes and the weights are missing. No number of the bridge is published without them.**

Item 13 asks for the local runner of frozen weights, the `sha256` of the weights, the decoding restricted to the schema of
the card, the proof of determinism in the form of G1, the calibration against the Brier of the deterministic Jev, the cost
per call, and the parallel question of adequacy of the hypothesis space. **The local runner (`crates/stochology-s2/src/runner.rs`,
397 lines, 9 tests), the calibration reader (`calibration.rs`, 9 tests), the bridge (`bridge.rs`, 12 tests), and the binary
`ingest-jev` (7 integration tests) already exist in this tree** — what is missing are the two operator artifacts
(`empirical/p20-jev/pesos/modelo.congelado` with `pesos/SHA256` and `empirical/p20-jev/calibracao/desfechos.tsv` with
`calibracao/SHA256`), with the gates armed.

**What already closes, with negative proof.** The weights gate: without the file, or with the hash different from the
declared one, the bridge **does not run**. The restricted decoding: free text and margin at 100% are rejected, and the
rejection is the abstention `Category::Others` — the auditable "I do not know" that already existed in System 1. The proof
of determinism in the form of G1: two executions and the same `sha256` closes on the test model and **does not close** on
the unstable model, which is the declaration of `docs/JEV_INTEGRATION.md` about the free LLM, now measured. The cost per
call and the fraction of abstentions come from the rehearsal.

**The criterion declared beforehand.** If the fraction of requests in which the bridge has to abstain exceeds **30%**, the
schema of the card is not adequate for that model — and the bridge reports this instead of forcing an answer outside the
schema.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --jev
~~~

**The limit.** Without the verified weights and the labeled set, no published number: neither calibrated Brier, nor real
cost, nor abstention of the real model. The K of the front is 1.
