# Point 13 — listening to those who pay for the error

**State: protocol frozen, listening not executed, box sealed.**

The program has seven decisions that touch real people and had heard none of them
([`governance/GRUPOS.md`](../../governance/GRUPOS.md), column "heard" as **none** in the seven). This front
freezes the protocol of the listening and seals the box until a recorded listening exists.

| file | what it is |
| --- | --- |
| `protocolo.md` | the protocol, frozen with hash: object, the declared fact, who conducts, the script, the rule of no substitution, the exit clause, the voice of the operator, change of criterion |
| `DECLARACAO.md` | the declaration with the hashes and the measurement of the seal |
| `DECLARACAO.sha256` | the hashes, in `sha256sum` form |
| `registro.tsv` | the machine record of the seven decisions, read by the instrument |

**What was measured.** Listening coverage: **0 of 7**. Minimum coverage declared to open the box: **2 of the 7**
groups with a signed record. Deadline of the window: **60 days**, until **2026-12-15**. Verdict: **box sealed** — the seven
decisions stay as a sensitivity analysis ($n = 1$) and do not become a public policy guideline or an
F8 gate decision until then or until the minimum coverage.

**What is missing.** The listening, and only it: reaching structural equivalents of the seven groups (professional associations,
public defenders' offices, sectoral representatives, heirs of the original bases), with an outside facilitator or a blind protocol, and
a signed record of whoever speaks. If the deadline expires without the minimum coverage, the artifact stays archived as a methodological
exercise of sensitivity, with the seal in hash.

**How to reproduce.**

```bash
cargo run --release --bin labs -- --escuta
```

The instrument reads `registro.tsv`, counts the coverage, checks whether any decision claims to be issued with nobody heard, and
gives the verdict of the seal. The two proofs — the decision that lies and the box that opens — come out in the report, and the tests of the
module hold both.
