# Reissued opinion 20 — H1·B NSW (LaLonde): vocational training against CPS controls

**Reissue of the rereading of 2026-10-16.** The original opinion is `h1-real/saida/b/resultados.json`, with SHA-256 `5aaa954e56ede16cd7d38360bd923b3c75dea201f46464e7bb398c36f436aa58`,
signed on 2026-10-15T03:53:28Z and **withdrawn in this rereading** (`governance/RELEITURA.md`). The original file
is not rewritten: it comes back from commit `1bfec905` and its hash remains that of the old signature. This
document is the new opinion; its hash enters the "Rereading" section of `governance/ASSINATURAS.md`, next to the
old one.

## Decision (unchanged in the reissue)
- action: **fund the training, with the rule "fund if the causal ATT > C/L"**
- P(adopting pays off): median causal ATT 0.0963, CI90 from 0.0088 to 0.1879
- scope, what **holds**: men eligible for NSW, 1976–77, 10 US cities, reference class with 84% black; declared circuit breaker: stop if the black fraction of a batch departs more than 0.25 from 0.84
- scope, what **does not hold**: other training programs, other cohorts and other countries

## What the reissue adds
- the governance block that **was missing** in the original: reference class, groups by declared race and the 0.25 circuit breaker on the black fraction;
- the declaration that the illustrative cost blocks the field recommendation;
- the mandatory note.

## Governance block (from the original `resultados.json`)
- data subjects: **the `resultados.json` of the real H1 has no `governanca` block** — there is no `tem_titulares`, there are no quasi-identifiers, there is no k-anonymity in this file; what exists is the `f6` block (groups by declared race and base rates) and `f5.aplicavel_a_titulares: true`.

## Mandatory note — two parts
*Proposal written by the facilitator from the material gathered (`governance/GRUPOS.md` and the
`resultados.json` itself); **confirmed and signed by sil on 2026-10-16**, with a micro-adjustment in the final clause.*

- **did not look at:** the real cost per participant: the `C` is **illustrative**, so that the decision rule does not close a field recommendation; the governance block, which simply did not exist; and the long-term employment outcome, which the opinion does not measure.
- **affected groups:** 185 NSW training participants and 15 992 CPS controls, by declared race (`negro`, `hispanico`, `outro`); **no** group heard (`GRUPOS.md`, row of the decision `h1-real/saida/b`).
