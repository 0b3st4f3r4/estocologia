# Declaration: Crate Consolidation and Tree Reorganization (2026-10-17)

On 2026-10-17 the tree was reorganized and the apparatus was consolidated. Nothing was retired: **no test, no
instrument, no gate, and no published number was removed**, and every name that disappeared from the crate list
lives on as a module of the crate that absorbed it.

## What Changed

| Before | After |
| --- | --- |
| root files in Portuguese (`PLAN.md`, `PROPOSTAS_DECLARADAS.md`, `REGISTRO_EMPIRICO.md`, `RESULTADOS_RUST.md`, `DECLARACAO_DO_ESTADO.md` and its seal, `INTEGRACAO_JEV.md`) | `docs/PLAN.md`, `docs/DECLARED_PROPOSALS.md`, `docs/EMPIRICAL_REGISTRY.md`, `docs/RUST_RESULTS.md`, `docs/STATE_DECLARATION.md` and `docs/STATE_DECLARATION.sha256`, `docs/JEV_INTEGRATION.md` |
| `DECLARACAO_*.md` at the root | `docs/declarations/RUST_CONSOLIDATION.md`, `LOST_REPORTS.md`, `REMOVED_NON_SIGNIFICANT.md`, `REMOVED_PYTHON.md` |
| `dados-reais/`, `fundo/`, `governanca/`, `historico/`, `artigo/`, `referencias/`, `estilo/` | `data/`, `empirical/`, `governance/`, `history/`, `paper/`, `references/`, `scripts/style/` |
| `book/stochology-textbook.md` at the root | `book/stochology-textbook.md` |
| `conferir_lacres.py`, `book/gerar.py`, `book/conferir_labs.py`, `book/conferir_livro.py`, `book/gerar_resultados.py` | `scripts/check_seals.py`, `scripts/book/build.py`, `scripts/book/check_labs.py`, `scripts/book/check_book.py`, `scripts/book/generate_results.py` |
| `scripts/verificar.sh` | `scripts/verify.sh` |

## The Sixteen Crates Became Nine

| Absorbed crate | Lives now as | Module |
| --- | --- | --- |
| `stochology-audit` | `stochology-core` | `audit` (SHA-256, hash-chained ledger, Merkle root) |
| `stochology-schema` | `stochology-core` | `schema`, with `migracoes.tsv` at the crate root |
| `stochology-icp` | `stochology-methods` | `icp` |
| `stochology-drift` | `stochology-methods` | `drift` |
| `stochology-fairness` | `stochology-methods` | `fairness` |
| `stochology-h0` | `stochology-horizons` | `h0` |
| `stochology-h1` | `stochology-horizons` | `h1` |
| `stochology-h2` | `stochology-horizons` | `h2` |
| `stochology-h3` | `stochology-horizons` | `h3` |

The nine crates that remain are `-core`, `-hypergraph`, `-methods`, `-horizons`, `-s2`, `-gates`, `-book`,
`-wasm`, and `-wave`. The binaries kept their names: `labs`, `gates`, `wave`, and `ingest-jev`.

## What This Cost, Measured

| Metric | Before | After |
| --- | --- | --- |
| Crates | 16 | **9** |
| Tests | 373 | **373** (`cargo test --workspace`: 22 suites, 0 failures) |
| Instruments in `labs` | 19 + 5 modifier flags | 19 + 5 modifier flags |
| Gates in `gates` | G1 to G7 | G1 to G7 |
| Gated numbers in `docs/RUST_RESULTS.md` | — | regenerated, byte-identical apart from the crate paths in the headers |

## Seals Re-Issued

The 28 seal files carry paths relative to the repository root, and some of them seal declarations whose prose
names the old directories. The renames changed that prose, so **47 target hashes were re-issued** on 2026-10-17:
the target list is unchanged, and what changed inside each sealed file is only the directory name in the text.
`docs/STATE_DECLARATION.md` was regenerated, and its seal covers the new text. `scripts/check_seals.py`
verifies 82 live targets with zero problems after the re-issue.

Historical commands are **not** renamed: every `git checkout 1bfec905 -- …` line still speaks the names the
commit uses (`fundo/`, `dados-reais/`, `governanca/`, `artigo/`, `referencias/`), because renaming a path inside a
command that reads a past commit would break the recovery it promises.

## How to Verify

~~~bash
cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
scripts/verify.sh                 # the whole §V list, in one command
python3 scripts/check_seals.py    # 28 seals, 82 live targets
~~~
