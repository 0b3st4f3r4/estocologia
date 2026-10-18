# Declaration: Consolidation in Rust (the Python Measurement Scaffolding Left the Tree)

On 2026-10-16, after the review and reports had been read, the repository completed its final consolidation: the
**Python measurement scaffolding** left the tree, and what remains is the textbook, the crates, the registry of numbers, the datasets
read by the instruments, the governance records, and the foundational empirical audit.

## What Left

**715 versioned files and 422.0 MB**, across folders and measurements:

| Item | What it was |
| --- | --- |
| `h0-lab0/`, `h0-real/` | the $H_0$ fronts, with READMEs, declarations, outputs, and figure |
| `h1/`, `h1-real/` | the $H_1$ fronts (Dow Jones, LaLonde, fires) |
| `h2/` to `h2g/` | the seven rounds of ICP, with re-executions and declarations |
| `h3/` | the continuous mesh, criteria, and revisions H3b through H3d |
| `data/h2*/`, `h3b/`, `h3c/`, `crsp_day_1989_1998.csv` | the datasets used exclusively by those fronts |
| `book/revisao.md`, `book/resposta-a-revisao.md`, `book/resultados-que-dependem-de-python.md` | generated reports, already read. **Correction of 2026-10-17:** they have no blob in `history/registro-python-removido.txt`, do not exist in `1bfec905`, and do not exist in any of the 94 commits of the tree. The record of their disappearance is in `docs/declarations/LOST_REPORTS.md` |
| `FORA_DO_GIT.md` and companion checksums | described large files that left along with the fronts |

## What Remains, and Why

| Remains | Role |
| --- | --- |
| `docs/RUST_RESULTS.md` | literal output of the instruments, with the command above each block |
| `docs/EMPIRICAL_REGISTRY.md` | the research numbers, with the R, D, or P tag for each |
| `data/` | the five datasets read by the instruments, `SOURCES.md`, and `SHA256SUMS` |
| `governance/`, `empirical/`, `references/` | sign-offs, foundational empirical audit, and bibliography verification |
| `paper/` | the paper, which is a distinct work rather than scaffolding |
| `book/`, `book/stochology-textbook.md`, `crates/` | the textbook and the apparatus |

## How to Recover

~~~bash
git checkout 1bfec905 -- h0-real/ h1-real/ h2/ h2g/ h3/ fundo/
~~~

The manifest `history/registro-python-removido.txt` contains the blob hash of every removed versioned file.

**Boundary declared on 2026-10-17.** The recovery line above previously also requested `book/revisao.md`, which never
worked: the `book/` directory does not exist in `1bfec905` (the commit has 1 314 files, and the six folders above
are among them), and none of the 94 commits in the tree contain the three reports. The command dropped the phantom
path, and the record of what cannot be recovered lives in `docs/declarations/LOST_REPORTS.md`.

## Declared Consequence

The chapters cite the fronts by name (`h0-real/README.md`, `h1-real/README.md`, `h2g/README.md`, and
others): those references are now **historical**, and the current path to each number is
`docs/EMPIRICAL_REGISTRY.md` together with the recovery note above. No number was changed: what changed is where it
lives.
