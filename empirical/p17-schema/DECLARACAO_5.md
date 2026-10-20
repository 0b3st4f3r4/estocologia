# Declaration 5 of point 17: the step 1.5 -> 1.6 of the chain

**Written on 2026-10-20**, from the item "card fields and contract 1.6" of the checklist of the catalogue and the
literature review, executed on the owner's instruction to proceed down that checklist. Written with an AI assistant.
It has not had a separate review by the owner; the owner's reading of this declaration is still due.

## The step

Six fields enter, all with the operation `introduz` and no value; a document at 1.5 migrates with them absent.

| step | field | what it declares |
| --- | --- | --- |
| 1.5 → 1.6 | `correlacao_de_selecao` | Meng's selection correlation by estimand, with the error and the effective size recomputed from rho, f and sigma |
| 1.5 → 1.6 | `corte_de_ece` | the ECE cut above the null's 95th percentile for the declared n, with the origin of the null's predictions |
| 1.5 → 1.6 | `modo_de_deriva` | the mode of the declared drift: covariate, label, concept or audience; label drift requires its detection |
| 1.5 → 1.6 | `mecanismo_de_ausentes` | MCAR, MAR or MNAR; MNAR requires a sensitivity analysis |
| 1.5 → 1.6 | `auditoria_de_subgrupos` | the rate equalised, the family of subgroups and who pays |
| 1.5 → 1.6 | `grafo_causal` | the causal graph, acyclic, from which the colliders can be read |

Two fields already in the contract get a backwards-compatible extension that the chain file cannot express, because it
adds no field: `autoria` accepts `role@model/persona` (the persona and the model are validated for form, not for truth),
and `regra_de_decisao` accepts `faixa=lo:hi`, the range of C/L over which the net benefit was read, which must contain the
C/L of the rule.

## What is checked (`crates/stochology-core/src/contract16.rs`)

- `correlacao_de_selecao`: the error `rho * sqrt((1-f)/f) * sigma` and the size `f / ((1-f) rho^2)` are recomputed, and a
  declared value more than 1% away is refused. The oracle is the one of the checklist: rho = -0.21, f = 0.24,
  sigma = 0.488 give -0.1824 and 7.16.
- `corte_de_ece`: the cut below `q95_nulo` is refused; the source of the predictions must be `reais` or `simuladas`.
- `modo_de_deriva`, `mecanismo_de_ausentes`, `auditoria_de_subgrupos`: the closed vocabularies and the companion
  requirement of each.
- `grafo_causal`: edges `A->B`, no self-loop, no repeated edge, no cycle.

## What this does not do

The fields are optional and no gate demands them. The recomputation of the margin of Phi2 against the null of the
halves needs data that the card does not carry, and the test of the independences the graph implies (Phi3) needs data
too: both stay out of this step. Nothing here changes the Merkle root of the gates (`0a9636fac5eb1c5e…`), which was
checked after the change.

## The seal

`DECLARACAO.sha256` keeps the hash of the chain at 1.5 as a line declared superseded, and holds the hash of the chain at
1.6 and of this declaration as live lines.
