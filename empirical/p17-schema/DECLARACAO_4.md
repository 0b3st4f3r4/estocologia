# Declaration 4 of point 17: the step 1.4 -> 1.5 of the chain

**Written on 2026-10-19**, by the owner's go-ahead on item 19 (a) of `docs/DECLARED_PROPOSALS.md`: the three failures of
teams of agents that `docs/MAST_AUDIT.md` found unseen get a place in the card. Written with an AI assistant.

## The step

| step | field | operation | value | why |
| --- | --- | --- | --- | --- |
| 1.4 → 1.5 | `autoria` | `introduz` | none | who wrote each field, each in the role of the specialist it belongs to (failure mode FM-1.2, a specialist outside its role) |
| 1.4 → 1.5 | `achados` | `introduz` | none | the findings logged, the ones used and the ones not used, each with its reason (FM-2.4, information withheld) |
| 1.4 → 1.5 | `regra_de_decisao` | `introduz` | none | the inequality the action rests on, p against C/L, recomputed from the card's own numbers (FM-2.6, an action that departs from its reasoning) |

## What is checked (`crates/stochology-core/src/conduct.rs`)

- `autoria`: `field:role, field:role`. The map of fields to roles is declared in `role_of`: the code writes the artifact,
  the seed and the property test; the data, the independent unit and the handling of missing values; the lifecycle, the
  drift; the inference, the null model, the margin, the pathologies, the budget, the multiverse and the reliability; the
  governance, the trail; the fairness, the subgroups and who pays for the error; the domain, the definition, the prior,
  the question and the scope; the decision, the action, the exit clause, the dominant event, the decision rule and the
  resolution. The identifier, the layer and the type of uncertainty are the orchestrator's frame and have no author
  among the specialists. A field outside the map, a role outside the eight, or a field named twice is refused.
- `achados`: `logged=N; used=U; unused=finding: reason | ...`. Used plus listed unused must equal logged, and every
  unused finding needs a reason.
- `regra_de_decisao`: `p=P; C=C; L=L; act=protect` or `act=do not protect`, with p strictly inside (0, 1) and C and L
  positive; the act must be the one p > C/L implies, and at p = C/L either is accepted.

## What this does not do

The three fields are optional, like those of 1.3 and 1.4, and no gate demands them. They check what a card declares
about the exchanges between specialists, not the exchanges themselves: a card that declares no withheld finding may
still have withheld one without logging it. In the terms of the audit, the three modes move from "not caught" to
"partly".

## K

The three formats and the map of roles are a choice of specification, made once and declared here: K = 1 for this step.

## The seal

`DECLARACAO.sha256` keeps the hash of the chain at 1.4 as a line declared superseded, and holds the hash of the chain at
1.5 and of this declaration as live lines.
