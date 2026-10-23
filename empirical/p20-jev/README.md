# Point 20 — the Jev-LLM bridge

**State: 32 real outcomes (`calibracao/desfechos.tsv`) and a deterministic numerical tensor baseline (`pesos/pesos.tensor`) are sealed and measured; open-weights LLM (GGUF) inference with and without grammar remains open.**

Item 13 asks for the local runner of frozen weights, the `sha256` of the weights, the decoding restricted to the schema of
the card, the proof of determinism in the form of G1, the calibration against the Brier of the deterministic Jev, the cost
per call, and the parallel question of adequacy of the hypothesis space. **The local runner (`crates/stochology-s2/src/runner.rs`,
10 tests), the calibration reader (`calibration.rs`, 9 tests), the bridge (`bridge.rs`, 12 tests), and the binary
`ingest-jev` (9 integration tests) live in this tree**, together with the 32 real audited outcomes
(`empirical/p20-jev/calibracao/desfechos.tsv` with `calibracao/SHA256`, `sha256`
`265fcf6e91573b97f9c9aceb2eb02947af58c2720fd15f96a690ed826d38e5ab`, 16 five-year windows of `data/danish_fire.csv` and
16 ten-year seasons of `data/atlantic_storms.csv`), a deterministic 2-layer numerical tensor baseline
(`empirical/p20-jev/pesos/pesos.tensor`, `sha256` `4ab811adb5f0a07dfe01f61d43a852ca68acdf34a9e17536740f8988f912aaf3`,
388 bytes, 96 parameters), and its sealed table (`empirical/p20-jev/pesos/modelo.congelado` with `pesos/SHA256`, `sha256`
`e2b55a3cf5a245caa03bf5007a2ffddea1882e34496fbdbb5f09fd02953ec20b`).

**What closes, and what must not be over-labeled.** With `pesos/SHA256` and `calibracao/SHA256` on disk, the weights gate
opens, verifies G1 determinism (`8bb0a10fc5cc24212caaecc7bd449c7b7d810c66c3399fa5bf4ba42419ebfa47` across two runs),
and computes the out-of-sample Brier score (`0.293988`) and 10-bucket ECE (`0.338131`) over the 32 real outcomes, recorded
in `rodadas.tsv`. However, `pesos.tensor` (`stochology-s2/tensor-v1`) is a **hand-specified numerical 2-layer MLP** whose
`decode_case` method formats the card via a Rust template, not a pretrained open-weights language model (LLM / GGUF): its
`0.0%` abstention rate is **guaranteed by template construction**, not a free-generation test of hypothesis-space adequacy.
As `FECHAMENTO.md` §1 states, closing item 13 as an **LLM** bridge still requires running a frozen open-weights LLM (GGUF)
over the 32 cases — without `grammar/ficha_jev.gbnf` to measure true abstention, and with `grammar/ficha_jev.gbnf` to measure
constrained calibration.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --jev
~~~

**The K of the front is 1**: the canonical tensor baseline was declared and executed once without post-hoc adjustment.
