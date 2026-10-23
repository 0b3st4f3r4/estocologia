# P21 · Can the layers fail?

**Status: catalogue and instrument sealed (`labs --layers`, `labs --rehearsal-layers`); box closed awaiting external human coders.**

Thesis (a) of the paper says that errors under uncertainty are misalignments between World, Model, and Agent rather
than errors of arithmetic. Because the three layers cover everything by construction, the thesis is a typing only if
independent coders agree on where an error lives, and only refutable if they can and do code an error as pure
arithmetic (`ARIT`) or outside the frame (`FORA`). `DECLARACAO.md` freezes the seven codes, the minimum of 60 errors
from sources outside the program, three coders with two from outside, Fleiss' κ, and the decision rule (typing at
κ ≥ 0.60, vocabulary below 0.40).

`DECLARACAO_2.md` (2026-10-20) freezes the external source list, the mechanical selection rule, and the catalogue
`erros.tsv` (60 documented external errors across 45 domains, with zero leaks of layer vocabulary), together with
the governance registry `registro.tsv` and the synthetic model-family rehearsal `codificacoes_ensaio.tsv`.
The instrument `labs --layers` checks the 60-error catalogue, enforces the lexical gate, and keeps the box closed
while human coders are unassigned (`0 of 3 coders, 0 of 2 external`), with negative and positive proofs of the
Fleiss' κ and falsifier decision rule. The rehearsal `labs --rehearsal-layers` measures the synthetic floor across
three model-family coders (`κ = 0.6405`, 90% CI `[0.5447; 0.7255]`, `ARIT` mentions `38`, `κ(ARIT) = 0.8332`,
`ARIT + FORA` majority share `0.2667 [0.1833; 0.3667] < 0.50`), labeled `SYNTHETIC` and never counted as human coders.

What remains missing is people: three human coders, two from outside the program (item 14 of `docs/DECLARED_PROPOSALS.md`).
