# Integration of Jev Material into Stochology

Record of the integration of the transcript *Integração de Modelos Jev à Estocologia* (*Integration of Jev Models into Stochology*). The transcript answers twelve
questions on how to couple Jev-type models — single-step closed categorical classification — to the
architecture of this repository tree. Every claim in the transcript was verified against the code rather than against prior prose: what
already exists is listed with its file and command, what has been retired appears as a **divergence declaration**
with both values, and what is new appears as a declared proposal in `docs/DECLARED_PROPOSALS.md` (item 13).

Pass date: **2026-10-16** (session execution context, per `AGENTS.md`).

Legend, identical to `docs/EMPIRICAL_REGISTRY.md`:

| Tag | Meaning |
| --- | --- |
| **V** | verified in the code of this tree, with file and command |
| **D** | declared divergence, stating both the transcript's value and the tree's value |
| **P** | declared without an instrument: exists in prose but not in code |

## 1. Source and Provenance

| Field | Value |
| --- | --- |
| Title | *Integração de Modelos Jev à Estocologia* (*Integration of Jev Models into Stochology*) |
| Origin | Gemini conversation, with the URL in the header of the file itself |
| Local file | `history/transcricao-jev-2026-10-16.md`, 470 lines, 6,263 words, 12 turns |
| Raw `sha256` | `cf223807b4ac578b4fc68bf5180703dff4246a6f09ee1d209b34dd894da2276b` |
| Git status | versioned since `5393515`; since 2026-10-17 located at `history/transcricao-jev-2026-10-16.md`, outside the repository root. The raw file is the source; the integrated form is this record (`docs/PLAN.md` §9.12) |

## 2. Verdict for Each Claim

| Transcript Claim | Tag | Where It Lives, and the Reproducing Command |
| --- | --- | --- |
| System 1 is typed triage in `O(1)`, with a deterministic suffix tree | **V** | `crates/stochology-core/src/s1.rs` (877 lines, 12 tests), with the tree materialized in a `BTreeMap` (`s1.rs:275-328`) — `labs --s1` |
| System 1 emits a closed-set label and a bounded probability | **V** | `Categoria::TODAS` with six labels (`s1.rs:127`), `PISO` and `TETO` in `s1.rs:52-55`, affine transfer `p = 0.01 + 0.94 * escore` in `s1.rs:84` |
| Abstention is a valid and auditable outcome | **V** | `Categoria::Outros` is the declared "I do not know" (`s1.rs:22`, `s1.rs:122`, `s1.rs:403`), proven in `s1.rs:772` (empty text returns `Outros`) |
| System 2 is deliberative, featuring tree search, belief state, and an orchestrator-free workbench | **V** | `crates/stochology-s2` (ten modules, 81 unit tests and 7 integration tests: `lats`, `crenca`, `hipergrafo`, `inferencia`, `estresse`, `planejamento`, `banca`, `ponte`, `runner`, `calibracao`, plus `lib`) — `labs --s2` and `labs --jev` |
| The adversarial panel approves by conjunction of all eight severities | **V** | `agentes.rs:111` implements `Aprovado(X) = AND A_i(X) >= Severidade_min`, with `SEVERIDADE_MINIMA = 6` in `agentes.rs:11` |
| The orchestrator drives the cycle and submits the artifact to the panel | **V** | `orquestrador.rs:7` (`OrquestradorM`), `orquestrador.rs:21` (`conduzir_ataque`), `orquestrador.rs:30` (`banca`) |
| The four replacements for orchestrator $M_{15}$ (open proposal constrained by schema, deterministic lens assignment, finite veto quota requiring a falsifying artifact, and mechanical closure via FSM without central signature) | **V** | `crates/stochology-s2/src/banca.rs` (866 lines, 25 tests) implements the finite-state machine of the orchestrator-free workbench with the four compilation rules **G8.1–G8.4**: G8.1 deterministic lens $\to$ reviewer assignment from the seed and artifact; G8.2 veto or review without an artifact returns `Err(Recusa)`; G8.3 finite quota (`VETOS_POR_JANELA = 2`) and prohibition of repeated vetoes on the same lens; G8.4 silence does not decide (`Pendente` before deadline, `Arquivada` without quorum of `LEITURA_MINIMA = 6`), with pure closure returning `Recibo` and the `sha256` of the canonical transcript |
| The orchestrator is named $M_{15}$ and the panel is $MB_1$–$MB_8$ | **D** | the tree uses $\mathcal{M}$ and $A_1$–$A_8$; see Declaration 1 |
| The funnel has four filters, with value-of-information pruning | **D** | `crates/stochology-core/src/funnel.rs` (878 lines, 17 tests); see Declaration 2 |
| The structural invariants are $\Phi_1$–$\Phi_4$ | **D** | the tree enforces $\Phi_1$–$\Phi_5$ (`invariantes.rs:12`, `invariantes.rs:110`); see Declaration 3 |
| System 1 is implemented as `LLM-as-Jev` over frozen open weights | **D** | the LLM's place is occupied by a deterministic **Jev**, without free generation (`s1.rs:10-22`); see Declaration 4 |
| `temperature=0` over the suffix tree yields identical logits and the same `sha256` | **D** | no instrument measures this here, and the argument in `s1.rs` states the opposite; see Declaration 5 |
| The latency budget for Jev-LLM triage is between "80 and 300 ms" per card | **D** | transcript claim for local GPU/CPU inference over open weights; in the tree, the deterministic Jev and the test frozen model run in a fraction of a millisecond (`ponte.rs`, measured in `labs --jev`), whereas the real latency of the open model depends on the still-missing frozen weights (item 13); see Declaration 9 |
| The mandatory header has five questions, with `pergunta_do_decisor` inside | **D** | five questions in `docs/PLAN.md` §10 and the Type III lock as its own dedicated field (`camada.rs:59`); see Declaration 6 |
| "0% syntactic schema failure" | **D** | holds for the deterministic Jev, which does not generate text; for an unconstrained LLM there is no measurement in this tree; see Declaration 7 |
| The Rust snippets in the transcript (`OrquestradorM`, `AgenteA`, `AgenteAtaqueAs`) | **D** | the first two identifiers exist with different signatures, and the third does not exist; see Declaration 8 |
| Tracks $F_2$, $F_5$, $F_6$, and $F_7$ gain batch rubric classification | **P** | declared in prose; there is no rubric classifier in code. What exists is card validation and the funnel |
| The parallel hypothesis-space adequacy question accompanies every System 1 call | **P** / **R** | abstention exists (`s1.rs`) and the parallel question entered the codebase in point 20 (`ponte.rs`, 12 tests): `adequacao_do_espaco` measures the fraction of abstentions per batch and derives the verdict from the pre-declared 30% threshold; the real model's number awaits the verified weights |
| Truncating the LLM probability to $[1\%, 99\%]$ feeds the Brier and Kelly log | **P** / **R** | the interval exists in the Jev scale, and point 20 implemented restricted decoding (`ponte.rs`), the local runner (`runner.rs`, 9 tests), and the calibration reader (`calibracao.rs`, 9 tests) that reject margins at 0% or 100% (`ExtremoDogmatico`); the bridge to an actual LLM remains declared, awaiting only the weights and outcomes files |

## 3. Divergence Declarations

1. **$M_{15}$ and $MB_1$–$MB_8$, and the four orchestrator-free replacements.** The transcript starts from the legacy roadmap numbering and replaces it mid-conversation
   with $\mathcal{M}$ and $A_1$–$A_8$, further proposing to replace the central orchestrator with four workbench mechanisms. The two values: the transcript opens with $M_{15}$ and $MB_1$–$MB_8$ and closes
   with $\mathcal{M}$, $A_1$–$A_8$, and the decentralized workbench; the tree uses $\mathcal{M}$ (`orquestrador.rs`), $A_1$–$A_8$
   (`agentes.rs:15`), and implements the orchestrator-free workbench in `crates/stochology-s2/src/banca.rs` (rules G8.1 to G8.4, 25 tests). Labels $M0$–$M15$ and $MB_1$–$MB_8$ have been retired since 2026-10-16, as recorded in
   Chapter 16 and in `docs/DECLARED_PROPOSALS.md` §4.
2. **Four filters.** The transcript discusses four filters and describes value-of-information pruning as future work. The
   two values: four filters in the transcript; a funnel with **17 tests** in `funil.rs` (878 lines), including
   `o_funil_conta_as_tentativas_ate_a_versao_final`. A state divergence also existed in the tree, in row 2
   of the axes table in `docs/PLAN.md` §9.13, which read "declared, not implemented"; that row was corrected in this
   pass.
3. **$\Phi_1$–$\Phi_4$.** The transcript enumerates four invariants. The two values: four in the transcript; five in the
   tree, with $\Phi_5$ (question resolution) in `invariantes.rs:12` and `invariantes.rs:110`, counted by the
   `gates` binary. The transcript was written prior to the addition of the `resolucao` field to the card.
4. **`LLM-as-Jev`.** The transcript treats the LLM as the engine of System 1. The two values: frozen open weights
   with `temperature=0` in the transcript; a deterministic Jev implemented without an LLM in the tree. The reason is documented in the
   header of `s1.rs`: an unconstrained LLM generates new text on every call, two runs diverge, and gate G1
   (determinism) fails to close. Consequently, this material becomes a declared proposal (item 13) rather than an active implementation.
5. **Determinism of `temperature=0`.** The transcript asserts that successive runs produce identical logits,
   identical probabilities, and the same `sha256`. The two values: an unmeasured assertion in the transcript; no measurement
   in this tree. What the tree measures is the determinism of the Jev: `gates --horizon all --reproduction` passes
   G1 and G2 with an identical second execution.
6. **The header.** The transcript writes the payload with six fields while calling the set "five questions." The two
   values: six fields in the transcript; five questions in `docs/PLAN.md` §10 (generative process, nonlinearity, independent
   unit, noise physics, causal ladder), with `pergunta_do_decisor` serving as the Type III lock in its own dedicated
   field (`camada.rs:59`).
7. **Schema failure.** The transcript attributes "0% syntactic schema failure" to the Jev model. The two values:
   zero percent in the transcript; zero percent measured solely for the deterministic Jev, which selects among six labels and
   never generates free prose. For an unconstrained LLM the tree has no instrument, and the tag is **P**.
8. **The Rust snippets.** The transcript proposes `OrquestradorM`, `AgenteAtaqueAs`, and `AgenteA`, with a
   `conduzir_ataque` method returning `Ok(())`. The two values: the sketches in the transcript; `OrquestradorM`
   (`orquestrador.rs:7`) and `AgenteA` (`agentes.rs:15`) exist with the same names and a different signature —
   `conduzir_ataque(&self, agente: AgenteA, ficha: &Ficha) -> Result<Severidade, String>` —, while
   `AgenteAtaqueAs` does not exist. The transcript did not return a severity score, whereas the tree does.
9. **Latency of "80 to 300 ms".** The transcript estimates between 80 and 300 ms per Jev-LLM triage call under a
   constrained grammar. The two values: 80 to 300 ms declared in the transcript; less than 0.05 ms in the synthetic rehearsal of
   `crates/stochology-s2/src/ponte.rs` (`labs --jev`), leaving the real inference latency dependent on the frozen
   weights file (`empirical/p20-jev/pesos/modelo.congelado`).

## 4. What Remains as a Declared Proposal

The bridge between the deterministic Jev and a locally executed language model is item **13** of
`docs/DECLARED_PROPOSALS.md`: *Jev-LLM Bridge (System 1 over frozen open weights)*. The code framework is
implemented in `crates/stochology-s2`: `ponte.rs` (12 tests), `runner.rs` (9 tests), `calibracao.rs` (9 tests),
`banca.rs` (25 tests, FSM G8.1–G8.4), and the `ingest-jev` binary (7 integration tests). What remains missing are the two
operator files: `empirical/p20-jev/pesos/modelo.congelado` (with `pesos/SHA256`) and
`empirical/p20-jev/calibracao/desfechos.tsv` (with `calibracao/SHA256`).

## 5. Verification of This Integration

Run on 2026-10-16, in this tree, with the codebase as is. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test --workspace` | 201 tests passed, none failed (0) |
| `cargo run --release --bin labs -- --s1` | triage runs: tree of 136 suffixes, six categories, row sum 1.0000 on every row, and **0** probabilities outside $[0.01, 0.99]$ (0) |
| `cargo run --release --bin labs -- --s2` | System 2 runs: 8 nodes, 2 graph edges, and 5 hyperedges validated, LATS reaches `f1.sintese-de-codigo` where greedy search stops at `h0.fila`, the Beta(4, 8) posterior yields 0.3346 against the closed form 0.3333, and the R-axis selects R1-dia with a loss of 46.24 min (0) |
| `cargo run --release --bin gates -- --horizon all --reproduction` | all seven gates passed, with panel $A_1$–$A_8$ worst score 8 against the floor of 6, identical second execution, and Merkle root `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81` (0) |
| `python3 scripts/book/build.py --check` | the textbook builds and verifies: 5,593 lines and 71,318 words (0) |
| `python3 scripts/book/check_labs.py` | 24 cited blocks, 0 divergent (0) |
| `python3 scripts/book/check_book.py` | 5,593 lines, 21 chapters, 5 parts, and balanced code fences (0) |
| `python3 scripts/book/generate_results.py --check` | **was out of date (1)** when the pass began: the new SHELF panel tests, from another front, were not yet in the registry. After running `python3 scripts/book/generate_results.py`, the file verified cleanly (0). The difference was instrument output — 26 tests in `shelf` and `preteste` where the registry previously listed 15, plus the gate verdict in the calibrated pre-test world — rather than this integration |

This integration touches no Rust code: `cargo test` and both the System 1 and System 2 instruments execute what already
existed. The only state updated was the registry `docs/RUST_RESULTS.md`, which had lagged behind the code due to the
SHELF panel front.
