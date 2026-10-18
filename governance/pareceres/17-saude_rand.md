# Reissued opinion 17 — Health (insurance and use of services, RAND HIE)

**Reissue of the rereading of 2026-10-16.** The original opinion is `h2g/saida/saude_rand/resultados.json`, with SHA-256 `4e9c87b011fc1e01b8faea4a66ed1bbd5454504082435a5b62a9eb5bc44c7d6c`,
signed on 2026-10-15T03:53:28Z and **withdrawn in this rereading** (`governance/RELEITURA.md`). The original file
is not rewritten: it comes back from commit `1bfec905` and its hash remains that of the old signature. This
document is the new opinion; its hash enters the "Rereading" section of `governance/ASSINATURAS.md`, next to the
old one.

## Decision (unchanged in the reissue)
- action: **adopt — replace free care with 25% copayment**
- P(adopting pays off): 0.74125
- scope, what **holds**: non-elderly US families in the 1970s, plans with an out-of-pocket cap and covered medical spending (without dentistry and psychotherapy)
- scope, what **does not hold**: the elderly (Medicare), plans without an out-of-pocket cap, health systems of other countries and **health outcomes** — the opinion only looks at spending

## What the reissue adds
- the limit **"only looks at spending"** in evidence, as part of the scope and not as a footnote;
- the mandatory note;
- the record that the excluded group (the elderly) was neither evaluated nor heard.

## Governance block (from the original `resultados.json`)
- data subjects: **yes**; quasi-identifiers: `mulher`, `local`; raw k-anonymity: 1 103; real data (`sampleSelection_RandHIE.csv`).

## Mandatory note — two parts
*Proposal written by the facilitator from the material gathered (`governance/GRUPOS.md` and the
`resultados.json` itself); **confirmed and signed by sil on 2026-10-16, in full**, without adjustment.*

- **did not look at:** any health outcome (the opinion measures spending, and the scope itself says that health is not covered); the elderly, whom the scope excludes by Medicare; and the budget sacrifice of the families that start paying 25% — there is no measure of income nor of treatment adherence in the opinion.
- **affected groups:** non-elderly US families in the RAND HIE (1970s), in the plans with an out-of-pocket cap; **no** group heard (`GRUPOS.md`, row of the decision `saude_rand`).
