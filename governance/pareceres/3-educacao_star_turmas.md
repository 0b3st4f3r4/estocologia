# Reissued opinion 3 — Education (smaller classes in STAR)

**Reissue of the rereading of 2026-10-16.** The original opinion is `h2g/saida/educacao_star_turmas/resultados.json`, with SHA-256 `f5b4307e4cbd103b96e7e0750c24e1758ec29995ae4eb7fa2bca28bc39477994`,
signed on 2026-10-15T03:53:28Z and **withdrawn in this rereading** (`governance/RELEITURA.md`). The original file
is not rewritten: it comes back from commit `1bfec905` and its hash remains that of the old signature. This
document is the new opinion; its hash enters the "Rereading" section of `governance/ASSINATURAS.md`, next to the
old one.

## Decision (unchanged in the reissue)
- action: **keep — **do not** adopt the reduction of the class sizes of the early grades to ~15 students**
- P(adopting pays off): 0.47275 (below 0.50, the opinion recommends not adopting)
- scope, what **holds**: early grades (kindergarten to 3rd) of US public schools, classes of 13 to 17 against 22 to 25
- scope, what **does not hold**: later grades, reduction at state scale with less experienced teachers and other countries

## What the reissue adds
- the fairness block by ethnicity with the base rates by group, which the decision of the original did not bring;
- the declared scope, in the letter of the original;
- the declaration that the SHELF panel of this domain remains absent;
- the mandatory note.

## Governance block (from the original `resultados.json`)
- data subjects: **yes**; quasi-identifiers: `menina`, `etnia`, `almoco_gratuito`; raw k-anonymity: 9; real data (`AER_STAR.csv`); fairness block by `etnia` with base rates: `afam` n = 4 524, `cauc` n = 8 632, `outra` n = 80.

## Mandatory note — two parts
*Proposal written by the facilitator from the material gathered (`governance/GRUPOS.md` and the
`resultados.json` itself); **confirmed and signed by sil on 2026-10-16, in full**, without adjustment.*

- **did not look at:** what happens to the `outra` group (n = 80) when the class shrinks — it is too small to sustain an interval; the implementation at state scale with less experienced teachers; and the elicited prior, which **does not exist** in this domain (no educator was consulted; the prior is the one of the literature from p05).
- **affected groups:** children from kindergarten to 3rd grade in Tennessee public schools in STAR (11 598 students, 80 schools), by declared ethnicity: `afam` 4 524, `cauc` 8 632, `outra` 80; **no** group heard (`GRUPOS.md`, row of the decision `educacao_star_turmas`).
