# Declaration 3 of point 16: the reformulated thesis (c′) and its dissociation test

**Written on 2026-10-18, before any case, opinion, or score exists for what it declares.** Its SHA-256 enters
`DECLARACAO.sha256` alongside declarations 1 and 2. A change to what follows is a new declaration and counts in the K
of the front.

## 0. Provenance, and what this declaration is not

It comes from an outside review of the whole thesis requested by the owner on 2026-10-18. The reviewer is a language
model (Claude, Anthropic), plausibly of the same family as the models that played the planter, the two arms, and the
judge of P01. It is therefore **not** the outside specialist that the decision of 2026-10-18 in
`docs/DECLARED_PROPOSALS.md` §3 seeks, and nothing here counts as independent judgment. It enters because the owner
asked for it to be applied, and it binds only after the owner merges it.

## 1. What does not change

Declarations 1 to 4 of `DECLARACAO.md`, the answers of `DECLARACAO_2.md`, `registro.tsv`, `regra_de_escolha.md`, the
window until **2027-01-14**, and the verdict on thesis (c) as written: **not sustained**, definitively so if the window
expires without the fair test. This declaration does not rescue thesis (c), does not move a deadline, and does not give
`labs --bridge` a new path to the word `sustentada`.

## 2. Why a second thesis

Thesis (c) says that the architecture catches errors that ordinary competent practice lets through. As written, it predicts
only "more", and P01 showed what "more" costs: on a canonical list of manuscript errors, a competent arm finds
everything, and the test cannot discriminate. The comparative advantage that the paper claims is not on that list. It is
at the **seams** between World, Model, and Agent, which is where the paper says the errors live. A thesis that
locates its advantage predicts an **interaction**, and an interaction can fail in more ways than a difference can.

**Thesis (c′).** The advantage of the architecture over competent ordinary analysis is concentrated in seam errors and
is absent in classical errors.

## 3. The two strata

- **Classical (C):** the ten errors of Makin and Orban de Xivry (2019), the list of P01.
- **Seam (S):** a closed list of six kinds, each one an error that the arithmetic can get right while the bridge
  between layers is wrong:
  1. **resolution of the question** — an aggregate answer to a question whose unit of decision is the day or finer
     ($\Phi_5$);
  2. **non-ergodic dynamics** — an ensemble mean read as the fate of one multiplicative trajectory;
  3. **performative loop** — a forecast scored on outcomes that the forecast itself changed;
  4. **selection inside a decision** — a collider or a filter that sits between the data and the action asked;
  5. **a tail without a usable mean** — a decision priced at a sample mean that the tail does not let converge;
  6. **who pays for the error** — a fairness or cost metric chosen without declaring where the error migrates.

Each seam case carries one planted defect of exactly one kind, written so that the arithmetic of the client's claim is
correct. The selection keeps the mechanical rule of `regra_de_escolha.md` for C; for S, the outside selector draws the
kinds with the same seed, 20270114, and writes or chooses the cases before any arm exists.

## 4. The prediction and the rule

With $\text{sens}_X(T)$ the fraction of planted defects that arm $X$ points out in stratum $T$, and
$\Delta_T = \text{sens}_A(T) - \text{sens}_B(T)$:

- **(c′) is sustained** if $\Delta_S > \Delta_C$, **and** $|\Delta_C|$ is at most one case of the stratum, **and** the
  false-alarm rate of A is not greater than that of B in either stratum.
- **(c′) is not sustained** otherwise, including a tie at the ceiling in S, an advantage of A that is equal in both
  strata (that would be a general advantage, which is thesis (c), not (c′)), and any advantage that A pays for in false
  alarms.

**Minimum:** 12 cases in S and 12 in C, each with 8 defective and 4 clean, chosen by the outside selector. With fewer,
the interaction is not read and the result is reported as descriptive. The power of the interaction test at this size
is computed by the instrument before the run, not declared here, because no number enters without a command.

## 5. Reliability, a second axis the pilot never measured

The value of an architecture may lie in how often it gives the same verdict, not only in whether it can find the defect
once. Each machine arm runs **5 times** per case, with the same instructions and different seeds; the agreement among
the five verdicts is measured by Fleiss' κ per arm and per stratum. The declared reading: if $\kappa_A > \kappa_B$ with
the same sensitivity, the architecture buys consistency and not accuracy, and the paper must say so. Human arms run once.

## 6. The monoculture rehearsal, before any human

The ceiling tie of P01 may be partly a property of one model family reading itself. Before the fair test, a rehearsal
runs the same design with **three different model families**: one for arm A, one for arm B, and one for the judge, none
from the family of the planter. It carries the label `SYNTHETIC`, counts no case, assigns no role, and changes no row of
`registro.tsv`. Its only purpose is to measure whether the tie survives the change of family.

## 7. What is declared and not enforced

`labs --bridge` reads `registro.tsv`, which has no stratum column; the instrument does not yet know S from C, nor the
five repetitions. Enforcing (c′) needs a stratum field in the record and a test that rejects a `sustentada` for (c′)
without both strata at the minimum. Until that exists, (c′) is a declaration and nothing more.

## 8. K

**K = 1 for (c′)**, its first specification. Thesis (c) keeps its own count and its own verdict.
