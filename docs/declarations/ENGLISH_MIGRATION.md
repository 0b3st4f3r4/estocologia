# Declaration: The English Edition (2026-10-17)

On 2026-10-17 the repository was migrated to English. The apparatus, the tooling, the textbook, the empirical
audit, the governance record, the paper, and the bibliography now speak one language. **No number moved**, with two
declared exceptions that are consequences of the translation itself: the two seals that hash translated text
(the wave engine's topology seal and its report seal) and the report hashes of the `gates` binary.

## What Changed

| Layer | Before | After |
| --- | --- | --- |
| Crates | Portuguese identifiers, doc comments, string literals, test names | English throughout: `Layer`, `Card`, `UncertaintyType`, `AgentA::A1Code`…`A8Decision`, `Ledger`, `Vault`, `Resolution`, `Funnel`, `Lens`, `Environment`, `Options`, `Group`, `PoissonNull`, `Pretest`, `Chain`, `Workbench`, `BeliefState`, `HypergraphWorld`, `LocalRunner` |
| Modules | `camada`, `agentes`, `orquestrador`, `invariantes`, `funil`, `resolucao`, `audit/cofre`, `lentes`, `h2/genetica`, `h3/cadeia`, `h3/preteste`, `banca`, `crenca`, `hipergrafo`, `inferencia`, `planejamento`, `calibracao`, `complexo`, `fasor`, `magnetico`, `plasticidade`, `espicula`, `selo`, `portao` | `layer`, `agents`, `orchestrator`, `invariants`, `funnel`, `resolution`, `audit/vault`, `lenses`, `h2/genetics`, `h3/chain`, `h3/pretest`, `panel`, `belief`, `hypergraph`, `inference`, `planning`, `calibration`, `complex`, `phasor`, `magnetic`, `plasticity`, `spike`, `seal`, `gate` |
| Binaries | `portoes`, `onda`, `ingestar-jev`, `labs` | `gates`, `wave`, `ingest-jev`, `labs` |
| Flags | `--todos`, `--banco`, `--fila`, `--resolucao`, `--genetica`, `--segunda`, `--ponte`, `--escuta`, `--cap`, `--ensaio-*`, `--horizonte`, `--defeito`, `--reproducao`, `--conferir`, `--registrar` | `--all`, `--bank`, `--queue`, `--resolution`, `--genetics`, `--second-opinion`, `--bridge`, `--listening`, `--chapter`, `--rehearsal-*`, `--horizon`, `--defect`, `--reproduction`, `--check`, `--record` |
| Tooling | Portuguese messages in `scripts/` | English messages; `scripts/style/decisions.md`, `style-sheet.md`, `repetition-index.md`, `chapter-brief.md`, `colons-brief.md` |
| Book | `00-abertura.md`, `cap01`…`cap21`, `apendices.md`, `horizontes-de-resolucao-da-pergunta.md`, `resultados-significantes.md`, `eixo-r.tsv` | `00-opening.md`, `ch01`…`ch21`, `appendices.md`, `resolution-horizons.md`, `significant-results.md`, `r-axis.tsv`; every chapter, appendix, and the preface in English |
| Archives | Portuguese prose in `empirical/`, `governance/`, `paper/`, `references/`, `data/` | English prose; `data/SOURCES.md`, `references/VERIFICATION.md`, `empirical/ENVIRONMENT.md` |

## What Did Not Move, and Why

**The numbers, the seeds, the seeds' arithmetic, the thresholds, the tolerances, the exit codes, and the test
count.** `cargo test --workspace` reports **373 passed, 0 failed** before and after; each crate reports the count it
reported before the migration. The `gates` merkle root over the twelve registered cards is still
`9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81`, and the negative proof
`gates --horizon h3 --defect h3.voos23` still exits 1 with the same root `802e84b6…`.

**Frozen Portuguese inside the apparatus**, because it is data the outside world hashes, prints, or reads:

| Frozen | Where | Why |
| --- | --- | --- |
| Card file format keys and codes (`versao`, `camada`, `incerteza`, `margem`, `L1`–`L3`, `aleatoria`/`epistemica`/`knightiana`, `R0-agregada`…) and `migracoes.tsv` | `stochology-core` | the on-disk contract of the specification card and its declared migration chain |
| Ledger event ids `cofre.abertura`/`cofre.recusa` | `stochology-core` | hashed into the audit trail |
| Grain markers `semana`, `mes`, `dia`, `agregad`… | `stochology-core::invariants` | cards declare their unit in the Portuguese vocabulary |
| Accent transliteration table and one accented fixture | `stochology-core::s1` | the table is the mechanism the test exercises |
| The twelve registered card ids and one-line definitions | `stochology-gates` | hashed into the merkle root |
| Session prefix `sessao-` and the frozen session key `todos` | `stochology-gates` | hashed into every ledger event |
| TSV wire tags (`medida`/`derivada`/`registro`, `linha`, `papel`, `decisao`, `registros`, `prazo`…), role and status tokens, case-group tokens, `cap`/`posto`/`plano`/`previsoes` in `book/r-axis.tsv`, `caso_XX` YAML keys | `stochology-book` parsers, `empirical/` files | the parsers and the on-disk records agree on them byte for byte |
| `ESTOCOLOGIA_DADOS` | `stochology-book` | the data-path environment variable, shared by instruments |
| The retired Python fronts (`portoes.py`, `rodar.py`, `grafo.py`) | `book/appendices.md`, `documentation` | historical commands that read commit `1bfec905` |
| `history/` | `history/` | dated records of earlier states, superseded but not rewritten |

## Declared Contract Changes

The translation moves four surfaces that something outside the compiler reads:

1. **Instrument stdout.** Every `labs`, `gates`, `wave` and `ingest-jev` line is English. The 48 blocks cited in the
   textbook were re-measured from the instruments themselves, not retyped.
2. **Sample hypergraph ids** (`root.stochology`, `h0.queue`, `h1.market`, `h2.invariance`, `h3.mesh`,
   `r.resolution`, `s1.triage`, `f1.code-synthesis`) and the Jev runner artifact keyword `@pedido` → `@request`
   with its labeled-set, rounds-registry, and measure-card headers.
3. **Two seals that hash translated text** move by construction: the wave engine's topology seal
   (`0xd49601b61389b864 → 0xd0202112900b9a49` became `0x5e4542e991272c40 → 0x4d6f5f559457ebc9`) and its report
   seal (`0x4482ebf698cf0cf0` → `0x5cfb206fbd79b748`). The state seals `0x27a9cf307a6a5414`/`0x9d10bccc515646cd`,
   the synaptic seal `0x06d33ef213a8f4a8` and the spike-word seal `0x84a69a5eaa0d07a1` are unchanged.
4. **The report hashes of `gates`**, because the report transcript is the hashed object:
   `08e7c8f0…` → `8279ae1c…` for `--horizon all`, and `52e06952…` → `88b7214b…` for `--horizon h3 --defect`.

## Seals Re-Issued

The English pass changed the text of 23 seal files' targets — 53 hashes in total — because the declarations inside
`empirical/` and `governance/` were translated. The target list is unchanged. `docs/STATE_DECLARATION.md` and
`docs/RUST_RESULTS.md` were regenerated and re-sealed last, so the chain closes on the final text.
`scripts/check_seals.py` verifies **28 seals, 82 live targets, 9 declared out-of-tree lines, 0 problems**.

## The Book After the Migration

The assembled edition is **7 178 lines and 96 018 words** (it was 5 927 and 75 304 in Portuguese: English spells out
articles the Portuguese elides, and the line wrapping re-flowed). Every chapter keeps its three tiers
(`### Step 1/2/3`), its `**Exercises.**` and `**Summary.**` blocks, and its place in the five parts. The 48 cited
blocks are the literal stdout of the English instruments, checked by `scripts/book/check_labs.py`.

## How to Verify

~~~bash
cargo test --workspace            # 373 passed
scripts/verify.sh                 # the whole §V list, in one command
python3 scripts/book/check_labs.py  # 48 blocks, 0 divergent
python3 scripts/check_seals.py      # 28 seals, 82 targets, 0 problems
~~~
