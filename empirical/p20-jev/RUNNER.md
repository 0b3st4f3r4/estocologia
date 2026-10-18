# The local runner of the frozen weights — operator's manual

Item 13 has two halves, and they are not confused with each other: **the runner is code and already exists**; **the weights are artifact and
only the operator brings them**. This file is the contract of whoever operates. It asks for no human judgment, no panel, and
no listening: it asks for three files, an order of operations, and a record.

## 1. What exists today

| piece | where | state |
| --- | --- | --- |
| the weights gate | `crates/stochology-s2/src/bridge.rs`, `verify_weights` | closes: without the file or with a different hash, the bridge does not run |
| the runner | `crates/stochology-s2/src/runner.rs`, `LocalRunner` | closes: loads the artifact, checks the declared hash, reads the blocks, and answers |
| the ingestor | `crates/stochology-s2/src/bin/ingest_jev.rs`, binary `ingest-jev` | closes: reads the two sources of the operator, validates field by field, charges the schema of the card through the real parser, rehearses in a temporary directory, reads back through the gates, and only then persists; five integration tests |
| the source generator | `scripts/generate_fire_sources.py` | closes: validates the catalog without calling a model, runs the runner through `llama-cli`, `vllm`, `api`, `file`, or `demo`, measures determinism with `--repeat`, and writes the two sources with the break escaped |
| the reading of the declared hash | `runner.rs`, `declared_hash` | closes: without `SHA256` in the directory, the runner refuses instead of guessing |
| the proof of determinism | `bridge.rs`, `prove_determinism` | closes in the runner: two executions, the same `sha256` |
| the adequacy of the space | `bridge.rs`, `space_adequacy` | closes: a request without a frozen answer abstains and enters the count |
| the scoring rule | `runner.rs`, `brier` | implemented; the number comes out when there is a labeled set |
| the reader of the labeled set | `calibration.rs`, `read_outcomes` and `load` | closes: strict format, sequential id, minimum sample of 30, abstention outside the denominator |
| **the labeled set** | `empirical/p20-jev/calibracao/desfechos.tsv` | **absent**: it is the observed outcome that only the operator brings |
| **the weights** | `empirical/p20-jev/pesos/modelo.congelado` | **absent**: it is what is missing, and it is what only the operator resolves |

The instrument proves the whole path with a rehearsal artifact that it writes in a temporary directory and erases:
`labs --jev` prints the hash of the rehearsal, the number of answers, the determinism, the fraction of abstentions, and the cost per
call. A rehearsal is not model weight: the calibration verdict remains without a number.

## 2. What the operator does, in order

The order matters: **the hash is declared before the execution**. Declaring after seeing the result is choosing the
specification while looking at the number, and that counts in the K.

1. **Choose the revision of the model.** One only, named and dated: repository, identifier of the revision, and date of the
   freezing. Changing revision afterwards is a new declaration. Hash the weights file itself at this step: with
   `llama-cli`, `scripts/generate_fire_sources.py infer --model M.gguf --revision "<repository> <identifier> <date>"`
   measures it and writes it as the first line of the inference source (`# model-sha256<TAB><hex><TAB><revision>`);
   with `vllm`, `api`, or `file`, the script does not read the weights, and the operator passes the hash measured where
   they are with `--model-sha256`. The ingestor carries that line into the artifact as `@model` (§3).
2. **Produce the artifact and the labeled set, in one go.** The format is in §3 and §4. The operator gathers the
   answers of the model that it froze and the observed outcome, and runs the ingestor:

   ```bash
   cargo run --release --bin ingest-jev -- \
     empirical/p20-jev/pesos \
     dados_brutos/inferencias_congeladas.tsv \
     empirical/p20-jev/calibracao \
     dados_brutos/desfechos_auditados.tsv
   ```

   The two sources are simple TSV: `pedido<TAB>resposta` and `caso_id<TAB>prob<TAB>desfecho<TAB>fonte`. Since the
   canonical document of the card has more than one line and the source is line by line, the break comes **escaped** as the
   sequence of two characters `\n`, and the ingestor undoes the escape; without the escape there are no two ambiguous lines. The
   ingestor **validates before persisting**: it checks every field, writes in a temporary directory, reads back through the same
   gates of the bridge (hash, format, sequential `id`, open interval of the probability), and only then writes to the
   destinations, with the two `SHA256`. If it refused, the destination is not touched. The writers of the library
   (`runner::write_artifact` and `calibration::write_outcomes`) do the same inside; the ingestor is the
   command-line path. There is no tensor in this format, and that is declared: a tensor loader is a next
   step, not half a path.
3. **Declare the hash before running:**
   ```bash
   cd empirical/p20-jev/pesos
   sha256sum modelo.congelado > SHA256      # the operator declares it, now, before the first execution
   ```
4. **Run the bridge**:
   ```bash
   cargo run --release --bin labs -- --jev
   ```
   With the hash matching, the output changes from `RECUSADO` to `pesos verificados: <hash> | N respostas congeladas`, and the
   determinism and the adequacy come to be measured on the real model.
5. **Fill in the record.** `empirical/p20-jev/registro.tsv` gains the line of the artifact: revision, hash, date, number of
   answers. The record is the proof; the file alone is not.
6. **Measure the Brier** against the deterministic Jev. It requires the set of pairs (probability, outcome) outside the
   fit; without it, `runner::brier` exists and has no input. This is the only step that asks for labeled data, and not for
   new code.

## 3. The format of the artifact

Text, first line the format, an optional line naming the model file, one block per frozen answer:

```text
# p20-jev/runner-v1
@model <sha256 do arquivo do modelo, 64 hex> <revisao: repositorio, identificador, data>
@pedido <sha256 do texto do pedido, 64 hex> <texto do pedido>
<documento canonico da ficha, uma linha por campo>
@end
```

The text of the request after the hash is the **v2** format, and it exists for a measurement reason: without it the instrument cannot
repeat the cases of the artifact itself, and the adequacy of the space would fall on a fixed probe unrelated to the data
of the operator. Artifact **v1**, with the hash only, remains readable, and the instrument declares that it is using its own probe.

The `sha256` of the request is that of the text that the bridge sends, which is `bridge::request(pergunta)` — not that of the raw question.
A request without a block does **not** invent an answer: it returns `fora do esquema`, which the bridge converts into abstention
(`Category::Others`) and counts in the adequacy.

The `@model` line exists because the `sha256` in `SHA256` covers the table, so it proves that the table did not change after
the declaration, and it does not prove which model wrote it. The line carries the `sha256` of the weights file itself and
the revision, inside the sealed text, so swapping it also invalidates the artifact. It is optional for the bridge: without it,
`labs --jev` runs and prints `model file: not declared`, and `bridge::registry_line` refuses the round, so no row of a real
round enters the record without the model it came from. The line names the file; it does not prove that the operator ran it.
That last step stays with whoever repeats the round with the same file and compares the tables.

What invalidates the artifact, and the runner refuses instead of tolerating:

| defect | what happens |
| --- | --- |
| the file is missing | `WeightsMissing` — the bridge does not run |
| the `SHA256` is missing | `HashNotDeclared` — the bridge does not run |
| one byte swapped in the artifact | `WeightsHashMismatch` — the bridge does not run |
| a `@model` line that is not 64 hex plus a revision, a second `@model` line, or one after the first block | `ArtifactUnreadable` — the bridge does not run |
| wrong header, block without `@end`, key that is not `sha256`, empty block | `ArtifactUnreadable` — the bridge does not run |

## 4. The labeled set that feeds the Brier

The Brier is not measured against the rehearsal: it needs an observed outcome. The canonical file is
`empirical/p20-jev/calibracao/desfechos.tsv`, TSV in UTF-8, with this exact header:

```text
id	case_id	declared_probability	observed_outcome	outcome_source
1	C-001	0.7500	1	registro-hospitalar-2026
2	C-002	0.2000	0	registro-hospitalar-2026
```

| field | what the reader charges |
| --- | --- |
| `id` | integer, sequential from 1 to N; out of order is `ArtifactUnreadable` |
| `case_id` | not empty; it is the join key with the request of the artifact |
| `declared_probability` | `f64` in the **open** interval (0, 1); strict 0.0 and 1.0 are `DogmaticExtreme` |
| `observed_outcome` | strictly `0` or `1`; another value is `ArtifactUnreadable` |
| `outcome_source` | not empty; it names the exogenous record from which the outcome came |

The hash of the file is declared in `calibracao/SHA256`, in the same form and with the same time rule as the artifact
of the weights: **before** the first execution. Swapping a line afterwards invalidates the set, and the instrument refuses instead of
recomputing.

The three rules of the score, declared before looking at the number:

1. **No pair, no score.** N = 0 returns `f64::NAN` and the reading says "Brier nao publicado".
2. **Minimum sample of 30 pairs.** Below that the instrument returns `InsufficientSample` and does not publish a score.
3. **Abstention does not become 0.5.** The case in which the model abstained leaves the denominator of the conditional Brier and enters the
   abstention rate, reported alongside the score.

Writing the file with the code, instead of by hand: `calibration::write_outcomes(dir, &[(caso_id, p, desfecho, fonte), …])`
writes the TSV and the `SHA256` in the same step. Reading and checking: `calibration::load(dir, abstentions, MIN_SAMPLE)`.

## 5. What stays declared, and is not a pending item of code

- **Tensor is not read.** The artifact is the table; the loader of raw weight stays as a next step, with the format
  declared before it exists.
- **The cost is counted in words of the returned document**, not in the tokenizer of the model. The number is declared,
  and the comparison with the real cost enters when the real artifact brings the count of the model itself.
- **The Brier requires a labeled set.** The reader exists (§4) with the three rules; what is missing is the file with
  observed outcome. Without it the bridge says so instead of publishing a rehearsal number as if it were calibration.

## 6. Synthetic rehearsal, to attest the path

Before there is real weight, the whole path is attested with 30 synthetic pairs, generated by
`crates/stochology-s2/examples/synthetic_round.rs` with a deterministic rule and the label `rodada-sintetica`:

```bash
cargo run --release --example synthetic_round -- /tmp/sint
cargo run --release --bin ingest-jev -- empirical/p20-jev/pesos /tmp/sint/frozen_inferences.tsv \
  empirical/p20-jev/calibracao /tmp/sint/audited_outcomes.tsv
cargo run --release --bin labs -- --jev
```

The literal output of that round, with the hashes and the reading of the instrument, is in `ENSAIO_SINTETICO.md`. The two
synthetic files were deleted after the rehearsal, and the instrument went back to refusing — a rehearsal number does not stay
where the real number should be.

## 7. The closure of the item

The closure protocol, with the four criteria, the format of the registry line, and the rule for when the item leaves the
list of pending items, is in `FECHAMENTO.md`. The checking of phase 1 is the `--check` mode of the ingestor: it reads the
two sources, runs the rehearsal, and writes to no destination — a volume below 30 exits with code 1.

## 8. Verification of what is asserted here

```bash
cargo test -p stochology-s2      # the tests of the runner and of the calibration: loading, hash, tampering, format, G1, abstention, Brier, minimum sample
cargo test -p stochology-s2 --test ingest   # the ingestor: writes, refuses, checks without writing, and does not touch the destination when it refuses
cargo run --release --bin labs -- --jev
```
