# Declaration of point 18: the R axis in the 21 chapters

**Frozen on 2026-10-16T23:19:00Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `book/eixo-r.tsv` | `5e0c7db5d931b70610fb0cd4c99718ae9f2d830f502baaef140402303c3ea04d` | the table per chapter: the declared case and the editorial sentence |
| `DECLARACAO.md` (this one) | — | the four declarations; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `3e705914f6eef17dc6f2e532fe12b549a15ccebe7288dc57a39746ac5843c4fe` | the result in one page |
| `DECLARACAO.sha256` | — | the three hashes, in `sha256sum` form |

## Declaration 1 — the form of the block, the same in the 21 chapters

Each chapter gains, in Step 3, the same short paragraph and **one line** of the instrument of the axis:

> **The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it costs to ask
> coarse. The line of this chapter, by the instrument of the axis (`labs --resolucao`):
>
> ```text
> capítulo 16 | perda de resolucao 46.24 min (demanda de 5 a 11) | o capítulo traz a tabela R0 a R3 do posto e do banco
> ```

The line is **literal output** of the instrument, and the block checker of the book charges it: `conferir_labs.py` went from
27 to **48 blocks checked**, with zero divergence. Single form, single placement (before the exercises), and no
number written by hand in the chapter.

## Declaration 2 — what the instrument computes and what the table declares

The table `book/eixo-r.tsv` declares, per chapter, **one case** and **one editorial sentence**. The instrument reads the table,
**computes** the reading of the case, and assembles the line; the sentence carries no number at all. The cases are five, and all of them come
from measurement:

| case | what the instrument computes |
| --- | --- |
| `posto` | the resolution loss in demand that oscillates from 5 to 11: **46.24 min** |
| `posto-leve` | the loss in the oscillation from 7 to 9: **5.97 min** |
| `plano` | the loss in flat demand: **0.00 min** |
| `previsoes` | the oscillation threshold at amplitude **2.5**, with the calibrated plan |
| `teoria` | no number: the chapter does not have two resolutions to compare, and the line says so |

Five chapters carry a number (5, 8, 10, 16, 18, and 20, with `posto`, `posto-leve`, and `plano` repeated) and chapter 21
carries the threshold. The other fourteen carry the honest declaration that the loss **is not measured there** — because there is not, in the
example of that chapter, the same decision taken at two grains. It is the difference between filling in a table and measuring.

## Declaration 3 — the editorial reading, per chapter

Where the chapter decides scale, the sentence names the grain: the post that oscillates (5), the tolerance that depends on the n of the
vault (8), the drag and the ruin measured by day (10), the table R0 to R3 of the post and of the bank (16), the window of the
fires (18), the live vault with two grains of test (20). Where the chapter does not decide scale, the sentence says what
it does and how the axis enters: the layers (2), the three types of uncertainty (4), the independent unit as Φ2 (6 and 17),
the ladder of cause (9), the trilemma as a subquestion of R3 (11), the map of knowledge (12), the panel (13), the contract
that carries the field `resolucao` and Φ5 (14), the vault as a new grain (15), and the aggregation by school (19).

## Declaration 4 — the limit and the K

**The limit.** The R axis enters the Step 3 of each chapter as a line, and not as a new laboratory: the
laboratory of the axis remains a single one, the `labs --resolucao`, and the resolution loss is measured only where the
two resolutions exist. The seven documented cases of the axis remain in the table of the document (`book/horizontes-de-
resolucao-da-pergunta.md`), each one with its source.

**The K.** The assignment of one case to each of the 21 chapters is a choice of specification, made once and declared
here: **K = 1**. Changing a case after seeing the line of a chapter is a new declaration, with its own hash, and counts in the
K.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `labs --resolucao` | the loss of the three cases, predictions 3 and 5, and the **21 lines** of the axis, one per chapter, with the reading computed by the declared case (0) |
| `python3 scripts/book/check_labs.py` | **48 cited blocks, 0 divergent** — the 21 new lines check in `labs --resolucao`, and the block of the axis is the only one of each chapter that came from the instrument of the axis (0) |
| `python3 scripts/book/build.py --check` | the three-step ramp of the 21 chapters checks, with 5 903 lines and 74 847 words (0) |
| `python3 scripts/book/check_book.py` | parts, chapters, fences, and order check (0) |

**Declaration of divergence (2026-10-17T08:40Z).** The record above also ceased to reproduce on this date, and
for the same declared reason: the third rectification of the edition touched `book/apendices.md` and the assembled book
grew. The `python3 scripts/book/build.py --check` now prints **5 927 lines and 75 209 words**, with code 0. The
difference is the tables of the passes and of the routes of Appendix D and the paragraphs that corrected the claim of physical
possession of the files; no number of this point changes, and the R axis continues with the 21 lines measured by the same
instrument. The file was re-sealed on this date, and the previous hash of the declaration (`a768543c…`) stays recorded
here instead of vanishing with the line of the seal.

**Declaration of divergence (2026-10-17T08:10Z).** The line of `gerar.py --check` above is a record dated
2026-10-16 and stays as it is: on that day the assembled book did have **5 903 lines and 74 847 words**. The
record ceased to reproduce on 2026-10-17, when `book/cap13.md` gained two lines in the rectification of the passage
about the label M15 — the same command prints today **5 904 lines and 74 861 words**, with code 0. The difference is
of two lines of prose, not of measurement: no number of this point changes. The file was re-sealed on this date, and the
previous hash of the declaration (`4ac060b4…`) stays recorded here instead of vanishing with the line of the seal.
