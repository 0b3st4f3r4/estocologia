# Closure of item 13: the protocol, and what is still missing

This file is the checklist of the closure, written **before** the data enter. It obeys the house rule: the
criterion is declared beforehand, the input enters through the strict ramp, the instrument measures, and the registry receives the number with
the provenance alongside. While the four criteria of phase 3 are not attested with the real input, item 13
**does not leave** the list of live pending items of `docs/DECLARED_PROPOSALS.md` — and this file says why, instead of
letting the list change out of enthusiasm.

## Phase 1 — the sources, outside the tree

| input | format | what the source has to fulfil |
| --- | --- | --- |
| `inferencias_reais.tsv` | `pedido<TAB>resposta`, with the internal break of the card escaped as `\n` | answer in the formal schema of the card, without free text and without a loose tag |
| `desfechos_auditados.tsv` | `caso_id<TAB>prob<TAB>desfecho<TAB>fonte` | N ≥ 30, p in the strict (0, 1), outcome 0 or 1, exogenous provenance in the column `fonte` |

The catalog of cases can be assembled by hand or with `tools/gerar_fontes_incendios.py`, which has two decoupled
stages: `conferir-casos` validates the catalog without calling any model, and `inferir` submits each prompt to the runner
— `llama-cli` by subprocess, `vllm` by `/v1/completions`, `api` by `/v1/chat/completions` with the key read from
`JEV_API_KEY`, `arquivo` for answers already gathered, and `demo` to attest the plumbing with a deterministic card —,
validates the schema, and writes the two sources with the break escaped.

**Strict decoding, and what it costs.** `grammar/ficha_jev.gbnf` restricts the output to the canonical document: 23
fields in the right order and `margem` in the format `0.<digits with at least one non-zero>`, which excludes 0.0 and 1.0 **by
syntax** instead of by validation. The generator uses `--gramatica <arquivo>`: with `llama-cli` it becomes `--grammar-file`,
with `llama-server` it goes in the body of the request. Attention to the price: with the grammar on, the form ceases to be measured and
comes to be guaranteed, so criterion 3 (abstention rate) becomes 1 by construction. The adequacy of the schema is only
truly measured **without** the grammar; the round with the grammar measures another thing — the distribution of the margin and the
calibration. The two rounds have to be declared as what they are.

The grammar of the draft that originated this front used `--- INICIO FICHA ---`, `CATEGORIA`, and `PROBABILIDADE`. That
schema is not the one of the repository: the canonical parser is `campo = valor` with `margem`. A grammar that produces the wrong
schema guarantees the wrong thing — which is why the one that is here follows the card of `stochology-core`.

**Two stages, and why.** With a small model, a complete example in the prompt makes the model copy the margin of the
example and all the predictions come out equal — constant calibration measures nothing. The protocol then elicits in two
stages: `--duas-etapas` asks first only the number ("answer only with a number between 0.0100 and 0.9900") and
then asks for the complete document with that margin already inside, so that the model fills in only `definicao` and
`acao_do_pacote`. The declared probability is that of stage 1, and it is the one that enters the Brier.

**The source escapes both columns.** Request and answer come with the break escaped as `\n`, and the ingestor unescapes
both. Unescaping only the answer leaves the key of the artifact pointing to the escaped text: the bridge looks, does not
find, and abstains in 100% of the cases, with the checking saying 32 of 32 valid. It happened in the first round with a
real model, and the test `a_escape_do_pedido_nao_quebra_a_chave_do_artefato` exists so that it does not repeat.

**The API route does not deliver frozen weight.** A remote provider does not publish a `sha256` of the weights and may swap the model
without warning, so two criteria of phase 3 change nature: the weights gate becomes the **declared revision of the
model** (version identifier, date, and, when there is one, the `sha256` that the provider publishes), and G1 ceases to be a
guarantee and comes to be **measured** — `--repetir 3` asks the same question three times and counts how many answers are
identical to the first. A repetition that does not close is a result, not noise: it means that provider does not serve as
frozen weight, and the bridge reports that instead of pretending determinism. The `demo` and the `exemplo` exist to prove the path: what comes out of them carries the label
`SINTETICO-...` and `sintetico-demo` in the provenance column, and does not serve for calibrating.

The checking of phase 1 is mechanical and writes nothing:

~~~bash
cargo run --release --bin ingestar-jev -- --conferir \
  caminho/para/inferencias_reais.tsv \
  caminho/para/desfechos_auditados.tsv
~~~

It reads the two sources, applies the rules field by field, runs the rehearsal in a temporary directory, prints the volume
against the minimum of 30 and the reading of the calibration, and **does not touch any destination**. A volume below the minimum exits with
code 1.

## Phase 2 — the ingestion and the seals

~~~bash
cargo run --release --bin ingestar-jev -- \
  empirical/p20-jev/pesos \
  caminho/para/inferencias_reais.tsv \
  empirical/p20-jev/calibracao \
  caminho/para/desfechos_auditados.tsv
(cd empirical/p20-jev/pesos && sha256sum -c SHA256)
(cd empirical/p20-jev/calibracao && sha256sum -c SHA256)
~~~

Expected output: `modelo.congelado: OK` and `desfechos.tsv: OK`. The ingestor only persists after the rehearsal passes
the same gates of the bridge; if it refuses, the destination stays untouched.

## Phase 3 — the four criteria, measured by the instrument

~~~bash
cargo run --release --bin labs -- --jev
~~~

| criterion | what the output has to show | what fails it |
| --- | --- | --- |
| 1. Weights gate | `pesos verificados: <sha256>` | `RECUSADO` for an absent or divergent hash, or an illegible artifact |
| 2. Determinism G1 | `identicas: true` with the same `sha256` in the two passes | `identicas: false` |
| 3. Adequacy of the space | `abstencoes (x%)` with x ≤ 30, measured **over the cases of the artifact itself** | x > 30: the item closes with the formal verdict that the schema does **not** serve the model, with the refusals counted, without imputation |
| 4. Brier and ECE | `Brier <valor> e ECE <valor> em 10 baldes sobre N pares`, with N ≥ 30 and abstentions outside the denominator | N < 30: `AmostraInsuficiente`, no score |

Criterion 3 admits two outcomes and both close the item: the schema serves, or the schema does not serve. What does not
close is hiding the abstention rate or imputing 0.5 in place of the refusal. Since 2026-10-17 the adequacy is measured over
the cases that the artifact keeps, and not over a fixed probe of the instrument; an artifact without the text of the request (v1) falls
on the probe, and the instrument declares which of the two it used.

The ECE uses the same function of `stochology-horizons` — `ece(confianca, acerto, 10)` —, reused instead of rewritten, so that the
repository does not have two definitions of calibration.

## Phase 4 — registry, seals, and the movement of the list

The line of the round is not typed: the instrument prints the measurement as a card (`labs --jev --medida`), and the ingestor
writes the line in `rodadas.tsv` with the same gate — `ingestar-jev --registrar <medida> empirical/p20-jev/rodadas.tsv
--data AAAA-MM-DD`. It refuses half a measurement: without verified weight, without G1 closed, below the minimum sample, or without
Brier and ECE, there is no line, and a repeated round is refused instead of duplicated.

The `registro.tsv` of this folder gains one line per real round, with the format already declared:

| field | where it comes from |
| --- | --- |
| `data` | the date of the execution, not that of the declaration |
| `N` | the number of pairs of the verified set |
| `abstencoes` | the count that the instrument printed |
| `custo_por_chamada` | tokens and milliseconds per call, as the instrument printed |
| `brier` | the conditional score, with six decimals |

After that:

~~~bash
python3 scripts/check_seals.py
python3 scripts/book/generate_results.py && python3 scripts/book/generate_results.py --check
python3 scripts/book/build.py --check
python3 scripts/book/check_labs.py
python3 scripts/style/conferir-passada.py
cargo test --workspace
~~~

Item 13 only passes to the closed section of `docs/DECLARED_PROPOSALS.md` when the line of `registro.tsv` exists
with the four criteria attested and the output of the instrument pasted in `ENSAIO_SINTETICO.md` (or in the registry of the
real round). Before that, the state is that of today:

| phase | state on 2026-10-17 |
| --- | --- |
| 1. sources | **both are missing**: there is no inference of a real model nor audited outcome in the tree |
| 2. ingestion | ready and proved: the ingestor writes, refuses, and does not touch the destination when it refuses |
| 3. criteria | measured in the synthetic rehearsal of 30 pairs (`ENSAIO_SINTETICO.md`); **not measured on the model** |
| 4. registry | format declared; **no line of a real round**, because there was no real round |

The synthetic rehearsal proves the path, not the model. Its number is that of the synthetic set and is labeled as such;
publishing it as calibration would be trading the declaration for the stamp.
