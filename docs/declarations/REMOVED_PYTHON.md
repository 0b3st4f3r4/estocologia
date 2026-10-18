# Declaration: The Python Simulation Code Has Been Retired

On 2026-10-16, the repository ceased maintaining the **Python simulation code** of the research fronts in the
working tree. The research has been concluded since 2026-10-15 and the project's engineering apparatus is now the
Rust workspace (`crates/stochology-*`), so the Python simulations have completed their lifecycle.

## What Left

**576 `.py` files** (approximately 2.9 MB), including:

| Category | Examples |
| --- | --- |
| Simulation packages | `h*/estocologia/{estat,icp,dsep,glm,armazem,diario,funil,amostrador,dominio,barramento}.py` |
| Front engines | `h*/rodar.py`, `h*/grafo.py`, `h*/figura.py`, `h3/fluxos/*.py`, `empirical/*/codigo/*.py` |
| Gates and tests | `h*/portoes.py`, `h*/conftest.py`, `h*/tests/*.py` |
| Working copies of foundational fronts | `empirical/p05-prioris/`, `empirical/p07-icp/codigo/`, `empirical/p08-calibracao/`, `empirical/p09-escala-dados/`, `empirical/p10-saude/`, `empirical/p11-eventos-raros/` |

## What Remained

| Remains | Why |
| --- | --- |
| `data/` (datasets, `SOURCES.md`, `SHA256SUMS`) | verifiable provenance of every number |
| Front `README.md` files, `DECLARACAO_*.md`, `CRITERIOS*.md`, `ficha.yaml` | registry of results, with the published numbers |
| Versioned `saida/` (307 files) | cryptographic summaries and reports verified by the audit |
| `governance/`, `empirical/*/README.md`, `empirical/p01-tese/pareceres/`, `references/`, `paper/` | governance and foundational empirical audit |
| `book/` and `scripts/book/` (book tooling) | structures the textbook: assembles and verifies |
| `empirical/p09-escala-dados/escala/baixar.py` and `empirical/p10-saude/codigo/baixar.py` | not simulations: raw-data provenance scripts — which **left subsequently**, along with the extensions they served, in the 2026-10-16 removal of material non-significant to the theory (`docs/declarations/REMOVED_NON_SIGNIFICANT.md`) |
| `crates/stochology-*` | the current engineering apparatus |

## How to Recover

The code was not lost: it lives in the Git history at commit **`1bfec9050bf046820787bdec8cb9765529658bdf`**.
The manifest `history/python-removido.txt` records the blob hash of every file for verification:

```bash
git show 1bfec905:h1/rodar.py           # view a single file
git checkout 1bfec905 -- h1/            # restore the entire front
git checkout 1bfec905 -- fundo/         # restore the entire foundational audit
```

## Declared Consequence

The Python commands cited in the textbook (Appendix C) and in `FORA_DO_GIT.md` — `python3 rodar.py`,
`python3 portoes.py --reproduction`, `python3 grafo.py`, `python3 figura.py`, `python3 -m pytest -q` —
**no longer run in the current working tree**. They run again after executing the `git checkout` above. What continues
to run without simulation Python is the textbook and crate pipeline: `cargo test --workspace`,
`cargo run --release --bin labs -- --all`, `cargo run --release --bin gates -- --horizon all
--reproduction`, `python3 scripts/book/build.py --check`, `python3 scripts/book/check_labs.py`, and
`python3 scripts/book/check_book.py`.

**Prior to retirement**, the independent review of the textbook verified 36 numbers against the front READMEs
(34 match, 2 are declared divergences) and ran `cd h1 && python3 -m pytest -q` (24 tests) and
`gates --horizon all --reproduction` (7 gates ok). The verified numbers are recorded in
`book/revisao.md`, along with the source of each.
