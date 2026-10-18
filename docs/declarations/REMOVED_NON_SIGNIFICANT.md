# Declaration: What Left the Tree as Non-Significant to the Theory

On 2026-10-16, alongside the retirement of the Python simulation code (`docs/declarations/REMOVED_PYTHON.md`), the project
removed artifacts that served as **illustration, scale demonstration, or lateral extension** without supporting any
central result. The triage governing this decision lives in `book/significant-results.md`: what remains is what
supports a Tier A result (what the theory cannot afford to lose) or a Tier B result (a known principle
measured by this program), plus the textbook, the crates, and the provenance of the numbers.

## What Left

| Artifact | Why it left |
| --- | --- |
| `book/edicao-14-capitulos.md` | 14-chapter text superseded by the 21-chapter book; the text lives in commit `1bfec905` |
| `empirical/p09-escala-dados/` | scale demonstration (329 domains in batch, 26 634 nodes): shows the funnel runs in batch, does not advance the theory |
| `empirical/p10-saude/` | healthcare extension (early sepsis): standalone result, outside the core |
| `empirical/p11-eventos-raros/` | rare-event extension (indomethacin, COVID): standalone result, outside the core |
| `data/p10/` (219 MB, unversioned) | raw dataset for the healthcare extension, downloadable from the sources in `SOURCES.md` |

**Total: 181 versioned files and 258.89 MB.** The numbers cited in the textbook from these extensions remain in the
text (Chapters 15 and 21), with a retirement note beside the table.

## What Remained, and Why

| Remains | Role in the theory |
| --- | --- |
| `book/`, `book/stochology-textbook.md`, `crates/stochology-*` | the work and the apparatus |
| `h0-lab0/`, `h0-real/`, `h1/`, `h1-real/` | Tier A: independent unit, severity, the gate that fails the agent |
| `h2/`, `h2b/` … `h2g/` | Tier A: the three successes of ICP and the five honest failures with counter $K$ |
| `h3/` | Tier A: measured Goodhart effect, calibration versus resolution, the three vaults |
| `empirical/p01-tese/` | the blind pilot of 12 cases (negative result that bounds the thesis) |
| `empirical/p02-independencia/`, `p03-multiplicidade/`, `p05-prioris/`, `p06-laco/`, `p07-icp/`, `p08-calibracao/` | foundational audit points that affect the verdict |
| `empirical/p12-shelf/` | the declared ethical boundary: protocol ready, zero interviews conducted |
| `governance/` | who signs off and who pays for the error |
| `data/` (excluding `p10`) | provenance of every number cited in the textbook |
| `references/`, `paper/` | the verified bibliography and the paper presenting the theory |
| `docs/declarations/REMOVED_*.md`, `history/python-removido.txt`, `history/nao-significante-removido.txt` | the audit trail of both removals |

## How to Recover

```bash
git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros book/edicao-14-capitulos.md
git show 1bfec905:book/stochology-textbook.md | head -50   # the 14-chapter edition
```

The manifest `history/nao-significante-removido.txt` contains the blob hash of every removed versioned file.

## Declared Consequence

The numbers from the removed extensions — scale (327 of 329 domains, 26 634 nodes, 6 494 concept nodes), rare
events (skill of +0.018629 and +0.017952, AUC 0.7685 in COVID), and healthcare ($\beta = 0.0388$, 0 of 512
subsets, skill of +0.00568) — remain in the textbook as **cited registry records**, rather than as
re-executable results. The significance triage did not change as a consequence: none of them belong to
Tier A or Tier B.
