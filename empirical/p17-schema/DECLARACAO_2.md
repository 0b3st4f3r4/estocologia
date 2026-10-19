# Declaration 2 of point 17: the step 1.2 -> 1.3 of the chain

**Written on 2026-10-19**, by the owner's go-ahead on the two card fields proposed after the propagation of
uncertainty entered the book. Declaration 1 said how a new field enters: as a new declaration, with the value of
addition and the reason, and the version of the card goes up. This is that declaration. Written with an AI assistant.

## The step

| step | field | operation | value | why |
| --- | --- | --- | --- | --- |
| 1.2 → 1.3 | `orcamento_de_incerteza` | `introduz` | none | where the variance of the output comes from, by input and covariance, shares closing to one; an earlier document declared no budget, and the migration does not invent one |
| 1.2 → 1.3 | `evento_dominante` | `introduz` | none | the event that carries the expected cost, with its probability and its share; frequency and severity are different axes, and the migration does not invent the event |

## A new operation, and why not a value of addition

The two earlier steps added a value (`R0-agregada`, `false`) because their fields are mandatory and the value chosen
was the honest reading of a document that did not speak of them. These two fields are **optional**: a card that does
not declare them is valid. Any value of addition would be invented, so the step uses a new operation, `introduz`, which
records the field in the contract and leaves the document as it was. The reading of the chain (`chain_from_text`)
accepts it; the notes of a step with several lines are joined, so that each field keeps its reason in the report.

## What is checked, and what is not demanded

When present, the budget must be `name=share; ...` with the shares adding up to one within 0.05 (covariances may be
negative), and the dominant event `name; p=probability; share=fraction`, with the probability strictly between 0 and 1
and the share in (0, 1]; a card that declares either in a form that does not read or does not close is refused when it
is read (`crates/stochology-core/src/budget.rs`). No gate demands the fields: demanding them would fail retroactively
every card already approved, and that is not this declaration's to decide.

## K

The format of the two fields and the tolerance of 0.05 are a choice of specification, made once and declared here:
K = 1 for this step. A later change is a new declaration.

## The seal

`DECLARACAO.sha256` keeps the hash of the chain as frozen on 2026-10-16 as a line declared superseded, and holds the
hash of the chain at 1.3 and of this declaration as live lines.
