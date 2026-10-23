# Declaration of point 20: the Jev-LLM bridge

**Frozen on 2026-10-16T23:57:11Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `empirical/p20-jev/registro.tsv` | `b97ef5032c93479dc20ae28112089809788ab5cce40e230c912fba41bf0c78c7` (historical: `27596bdf…`, `b777a213…`, `909c1ad7…`, `92c0d1b0…`) | the state of the bridge, item by item, in machine form |
| `DECLARACAO.md` (this one) | — | the five declarations; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `daaf0fc853924680e09cf65a1835d2fa5f35d06cb60684adaba746c3b09639ef` (historical: `e4c3fe9f…`, `c18ddc5e…`, `c6b2798e…`) | the result in one page |
| `DECLARACAO.sha256` | — | the seven hashes of the folder and of the grammar, in `sha256sum` form |

## Declaration 1 — what the item asks and what the tree has

Item 13 asks six things: the local runner of frozen weights, the `sha256` of the weights, the decoding restricted to the
schema of the card, the proof of determinism in the form of G1, the calibration against the Brier of the deterministic Jev, the cost
per call, and the parallel question of adequacy of the hypothesis space. **The weights are not in this tree, and there is no
local runner here.** So the bridge stays declared, and this module delivers the whole framework: what closes without the
weights, closes; what only closes with them stays with the gate armed and the number in blank.

## Declaration 2 — the gates that already close, with the proof

| gate | how it closes | the negative proof |
| --- | --- | --- |
| **weights** | the file must exist and the `sha256` must match the declared one; without that the bridge does not run and no number comes out | an absent path returns `PesosAusentes` with abstention `Outros`, and a different hash returns `PesosComHashDiferente` with the two values |
| **restricted decoding** | the output must be the canonical document of the card, and the margin cannot touch 0% nor 100% | free text returns `ForaDoEsquema` and the margin at 100% returns `ExtremoDogmatico`; both abstain in `Categoria::Outros`, which is the auditable "I do not know" that already existed in `s1.rs` |
| **determinism (G1)** | two executions, the same `sha256` of the sequence of outputs | the test model closes (identical hashes); the **unstable model does not close** (two different hashes) — which is exactly what `docs/JEV_INTEGRATION.md` declares about the free LLM, now measured |
| **cost per call** | the accounting sums calls, tokens, and time, and divides | it runs in the rehearsal: 42 tokens and the time per call; with the weights, the same code measures the real cost |
| **adequacy of the space** | the fraction of abstentions over the requests, against the declared limit of **30%** | the test model abstains in 0% ("the schema serves") and the unstable one in 100% ("the schema does NOT serve") |

## Declaration 3 — the limit, declared as a limit

**Without the verified weights, no number of the bridge is published**: neither the Brier calibrated against the deterministic Jev,
nor the real cost per call, nor the fraction of abstentions of the real model. What the instrument publishes is the
mechanism, with a test model declared as such, and the gate armed for when the weights exist. It is the same
rule of points 13, 16, and 19: no future measurement is promised, the state of today is declared.

## Declaration 4 — the parallel question, with the criterion declared beforehand

"Is the schema of the card an adequate hypothesis space for this model?" The answer stays falsifiable by the criterion
declared now, before seeing any real model: **if the fraction of requests in which the bridge has to abstain exceeds
30%, the schema does not serve the model**, and the bridge reports that instead of forcing an answer outside the schema. The limit
of 30% is a choice of specification declared once.

## Declaration 5 — the K and what does not change

**K = 1**: the abstention limit and the rules of refusal are a choice of specification, declared once. It changes
nothing of what is already published: the bridge is a new framework, and no number of the book, of the empirical registry, or of the
horizons passes through it.

**Note of 2026-10-17: the runner exists.** §1 of this declaration and the `registro.tsv` said that the local runner did not
exist in the tree. It came to exist in `crates/stochology-s2/src/runner.rs` (397 lines): it loads the artifact, reads the hash
declared in `pesos/SHA256` **before** running, checks it, and answers through `ModeloCongelado`; a request without a frozen
answer abstains and enters the count of adequacy. There are **9 tests** in `runner.rs`, **12** in `ponte.rs`, **9** in
`calibracao.rs`, and **7** integration tests in the `ingestar-jev` binary, and the operator's manual is `RUNNER.md`.
What remains absent is the **artifact**: `pesos/modelo.congelado` and the declared `sha256`. The gate keeps
refusing, and that is the correct behavior. The seal of this folder was redone: `registro.tsv` and `RUNNER.md` enter
it, and the previous hashes of the record (`27596bdf…`, `b777a213…`) and of the `README.md` (`e4c3fe9f…`) stay recorded
here instead of vanishing with the line of the seal.

**Note of 2026-10-18: the operator's manual follows the English migration.** `RUNNER.md`, `FECHAMENTO.md`, and
`README.md` still named the pre-migration binary (`ingestar-jev`, with `--conferir` and `--registrar`), the generator
at `tools/gerar_fontes_incendios.py`, the example `rodada_sintetica`, the modules `ponte.rs` and `calibracao.rs`, and the
Portuguese refusal names; and `RUNNER.md` §4 gave the header of `desfechos.tsv` as `id caso_id probabilidade_declarada
desfecho_observado fonte_desfecho`, which `calibration::read_outcomes` refuses, because the reader expects
`id case_id declared_probability observed_outcome outcome_source`. An operator following the manual literally would have
been stopped at the first command. The three files now name what exists in the tree; `ENSAIO_SINTETICO.md` keeps its
2026-10-17 commands as the record of that run and gains a dated rectification with today's. No gate, rule, threshold, or
number changed, so the K stays 1. The seal of this folder was redone; the previous hashes were `README.md`
`6c9aa1c9…`, `RUNNER.md` `2a6459e4…`, `ENSAIO_SINTETICO.md` `d96e2074…`, `FECHAMENTO.md` `a99888f2…`, and
`DECLARACAO.md` `cae8cf4a…`.

**Note of 2026-10-18, later the same day: the artifact names the model file it came from (owner's request).** The
`sha256` declared in `pesos/SHA256` is that of the table of answers: it proves that the table did not change after the
declaration, and it does not prove which model wrote it, and `scripts/generate_fire_sources.py` did not record the hash of
the weights file. The artifact now accepts one `@model <sha256> <revision>` line before the first block, inside the sealed
text; the generator measures the hash of the GGUF with `llama-cli` and takes `--model-sha256` with the other runners (and
refuses it with `demo`, which is never the model); the ingestor carries the line from the inference source into the artifact;
`labs --jev` prints the declared model file or says it is missing; and `bridge::registry_line` refuses a round without it,
with a `model_hash` column in `rodadas.tsv`. Tests: 83 unit and 9 integration in `stochology-s2` (81 and 7 before), the new
ones being the line inside the seal, the four malformed or misplaced lines, the refusal of the record without the model, and
the ingestor carrying the line and aborting on a bad one. The line names the file and does not prove the operator ran it; that
stays with whoever repeats the round with the same file. No threshold of the four criteria changed, and the record gained a
condition of provenance, not of decision, so the K stays 1. The seal of this folder was redone again.

**Note of 2026-10-20: 32 real outcomes and a deterministic tensor baseline are sealed; the open-weights LLM round remains open.**
`crates/stochology-s2/src/runner.rs` gained `TensorWeights` (revision `stochology-s2/tensor-v1 2026-10-20`, serialized in
`pesos/pesos.tensor` with `sha256` `4ab811adb5f0a07dfe01f61d43a852ca68acdf34a9e17536740f8988f912aaf3`) and `ingest-jev`
gained `--from-data`, which reads the 16 five-year windows of `data/danish_fire.csv` and the 16 ten-year seasons of
`data/atlantic_storms.csv`, runs the frozen tensor forward pass, stages and verifies both gates, and writes
`pesos/modelo.congelado` (`sha256` `e2b55a3cf5a245caa03bf5007a2ffddea1882e34496fbdbb5f09fd02953ec20b`), `pesos/SHA256`,
`calibracao/desfechos.tsv` (`sha256` `265fcf6e91573b97f9c9aceb2eb02947af58c2720fd15f96a690ed826d38e5ab`), `calibracao/SHA256`,
and the round in `rodadas.tsv` (32 requests, 0 abstentions, 32 pairs, 88.0 tokens/call, 0.0% abstention rate, Brier
`0.293988`, ECE `0.338131`, `K = 1`). **Declared boundary (audit of 2026-10-20):** `pesos.tensor` is a 388-byte, 96-parameter
numerical MLP whose `decode_case` method formats the card via a Rust template, **not** an open-weights language model (GGUF):
its `0.0%` abstention rate holds by template construction rather than measuring unconstrained LLM output. The 32 real outcomes
and the tensor baseline are sealed in this folder (13 files), while item 13 remains open in `docs/DECLARED_PROPOSALS.md` §1
until an open-weights LLM (GGUF) is run with and without `grammar/ficha_jev.gbnf`.

## Verification

Run on 2026-10-16, after the freezing, and updated on 2026-10-17. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-s2` | 81 unit tests and 7 integration tests passed (36 at the freezing of 2026-10-16), among them the 12 of `ponte`, the 9 of `runner`, the 9 of `calibracao`, and the 7 of `ingestar-jev`: without weights it does not run, a different hash refuses, free text abstains, margin at 100% refuses, G1 closes on the test model and **does not** close on the unstable one, and the cost and the adequacy come from the rehearsal (0) |
| `labs --jev` | the weights gate refusing with the weights absent, the restricted decoding with the two refusals, the proof of determinism in the two models, the cost per call, and the adequacy in the two models (0) |
| `python3 scripts/book/generate_results.py --check` | `docs/RUST_RESULTS.md` updated, with the block of the bridge (0) |
