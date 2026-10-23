# Point 20 — the Jev-LLM bridge

**State: closed on 2026-10-23 (`labs --jev`, `rodadas.tsv`).** A frozen open-weights language model (`Qwen/Qwen2.5-0.5B-Instruct-GGUF` `q4_k_m`, `sha256` `74a4da8c9fdbcd15bd1f6d01d621410d31c6fc00986f5eb687824e7b93d7a9db`) was run locally via `llama.cpp` (`llama-completion` `b11838`, `temperature = 0`, `seed = 2026`) both without and with `grammar/ficha_jev.gbnf` over the 32 real audited outcomes (`DF-01`..`DF-16` from `data/danish_fire.csv` and `AS-1990`..`AS-2005` from `data/atlantic_storms.csv`), alongside the preserved `2026-10-20` numerical tensor baseline (`stochology-s2/tensor-v1`).

Item 13 asks for the local runner of frozen weights, the `sha256` of the weights, the decoding restricted to the schema of
the card, the proof of determinism in the form of G1, the calibration against the Brier of the deterministic Jev, the cost
per call, and the parallel question of adequacy of the hypothesis space. **The local runner (`crates/stochology-s2/src/runner.rs`,
10 tests), the calibration reader (`calibration.rs`, 9 tests), the bridge (`bridge.rs`, 12 tests), and the binary
`ingest-jev` (9 integration tests) live in this tree**, together with the 32 real audited outcomes
(`empirical/p20-jev/calibracao/desfechos.tsv` with `calibracao/SHA256`, `sha256`
`36ac1b2a98c453e34150d2a0bf8aa81e7ada24c56310a4217635320bc002c466`, historical tensor-baseline hash
`265fcf6e91573b97f9c9aceb2eb02947af58c2720fd15f96a690ed826d38e5ab`), the preserved 2-layer numerical tensor baseline
(`empirical/p20-jev/pesos/pesos.tensor`, `sha256` `4ab811adb5f0a07dfe01f61d43a852ca68acdf34a9e17536740f8988f912aaf3`,
388 bytes, 96 parameters), and the sealed open-weights GGUF inference table (`empirical/p20-jev/pesos/modelo.congelado`
with `pesos/SHA256`, `sha256` `4dddcbe3d76d27035dcd9a11823b0deaa5877a80476a16700b07198c1b410f79`, historical tensor-baseline
hash `e2b55a3cf5a245caa03bf5007a2ffddea1882e34496fbdbb5f09fd02953ec20b`).

**What the three regimes of the frozen GGUF model proved.** Running the exact same frozen 0.5B open-weights model
(`74a4da8c9fdbcd15bd1f6d01d621410d31c6fc00986f5eb687824e7b93d7a9db`) across the three protocol regimes over the 32 real
cases isolated what prompt decomposition and GBNF grammar each contribute:

1. **Single-stage without `grammar/ficha_jev.gbnf`** (`artifact sha256` `9745dc2f7644160e95b2aaae92b2d9003a505e6c61889a3a582298f47f4eac1b`):
   G1 determinism closes (`1/1` identical repetition, `5156.3` ms/case on CPU), but **32 of 32 outputs (`100.0%`, above the
   `30%` limit)** are refused with `OutOfSchema` (conversational preamble `"Aqui está a resposta completa para o caso:"` and
   omitted mandatory fields `deriva_declarada` and `escopo_ecologico`), leaving `0` valid pairs.
2. **Two-stage (`--two-stage`) without `grammar/ficha_jev.gbnf`** (`artifact sha256` `0145e724b59a5a0607ec2f8e0b70b635f69f157dec1a1b67beb53def751b1335`):
   G1 determinism closes (`1/1` identical repetition, `5625.0` ms/case on CPU) and hypothesis-space adequacy passes with
   **3 of 32 abstentions (`9.4% <= 30.0%`)**, all three caused by `DogmaticExtreme("margin 0")` in stage 1 (`0.0000` on
   `DF-04`, `DF-06`, and `DF-10`); however, the remaining **29 valid pairs** fail the calibration volume gate
   (`InsufficientSample: 29 pairs, declared minimum of 30`), so the score is refused.
3. **Two-stage (`--two-stage`) with `grammar/ficha_jev.gbnf`** (`artifact sha256` `4dddcbe3d76d27035dcd9a11823b0deaa5877a80476a16700b07198c1b410f79`):
   excluding `0.0` and `1.0` by syntax converts the three stage-1 boundary cases to `0.0008` (`DF-04`, `DF-06`) and `0.0004`
   (`DF-10`), yielding **0 of 32 abstentions (`0.0%`)**, **32 of 32 valid cards (`N = 32 >= 30`)**, G1 determinism
   (`977af0333038326e92154652e4812edd9a0342eb89060baf61fc304146a697a4` across both passes), `95.1` tokens/call (`5531.3` ms/case
   on local CPU, `0.0` ms/call in frozen replay), **Brier `0.276413`**, and **10-bucket ECE `0.270562`** — improving on the
   `2026-10-20` `tensor-v1` baseline (Brier `0.293988`, ECE `0.338131`) while remaining worse than the deterministic Jev
   reference (Brier `0.219375`, ECE `0.212500`).

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --jev
~~~

**The K of the front is 2**: `rodadas.tsv` records both the `2026-10-20` `tensor-v1` baseline (`K = 1`) and the `2026-10-23`
`Qwen2.5-0.5B-Instruct-GGUF` `q4_k_m` round (`K = 2`), alongside the two unconstrained GGUF diagnostic runs documented above.
