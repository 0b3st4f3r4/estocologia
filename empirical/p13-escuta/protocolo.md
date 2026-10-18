# Listening protocol — background point 13 of the research

**Front:** `p13-escuta`. **Owner:** `sil`. **Frozen before the first attempt at contact.**

This protocol deals with the only thing that the program has not yet done: **listening to those who pay for the error**. The book asks for this in
section 14.2, objection "The auditor is an interested party"; the record of who was heard is in
[`governance/GRUPOS.md`](../../governance/GRUPOS.md), and it says **none** in the seven decisions that involve people.

## 1. Object

The seven decisions of the program that affect real people, with the groups that pay for the error of each one:

| decision | what it does | who pays for the error |
| --- | --- | --- |
| `h2g/saida/direito_teto_beneficio` | cuts the weekly benefit by 10% to shorten absences from work | workers injured on the job covered by the ceiling, Kentucky (1980) and Michigan (1982); the group above the ceiling |
| `h2g/saida/educacao_star_turmas` | reduces the classes of the early grades to about 15 pupils | children from kindergarten to 3rd grade of the public schools of Tennessee in STAR (11 598 pupils, 80 schools), by declared ethnicity |
| `h2g/saida/saude_rand` | swaps free care for a 25% copayment | non-elderly families of the RAND HIE (1970s), in the plans with an out-of-pocket ceiling |
| `h1-real/saida/b` (NSW/LaLonde) | compares the NSW job training with CPS controls | 185 participants of the NSW training and 15 992 CPS controls, by declared race |
| `h3` flow `prisoes` | flags the detainee for release review (Toronto 1997–2002) | 5 226 detainees, by perceived colour |
| `h3` flow `prisoes20` | flags the arrested person for review of the record at the police station (Toronto 2020–2021) | 65 276 arrests, 37 thousand distinct people, by perceived colour |
| `h3` flow `prisoes_ri` | flags the driver for review of the arrest at the stop (Rhode Island 2005–2015) | 86 113 stops, by perceived colour |

The machine record of the seven lines is `registro.tsv`, read by the instrument `labs --escuta`.

## 2. The declared fact: none of the historical cohorts is reachable

Declared by the owner of the program on 2026-10-16: **none of the seven historical groups is within reach.** The cohorts
are from decades ago — STAR children in the 1980s, RAND HIE families, Toronto detainees in 1997, Rhode Island drivers
in 2005 — and direct contact is unfeasible because of the temporal, legal, ethical, and anonymity barrier. The
program does not have, and will not invent, a record of a consultation that did not happen.

That does not close the front: it changes what is sought. What is sought is a **structural equivalent** — who today occupies the
same position as those who paid for the error then.

## 3. Who conducts

The listening **is not conducted by the owner of the program**. The adjustments and the priors came from the same hand that would ask the
questions, and that is structural confirmation bias. One of the two forms is worth, declared beforehand:

1. **neutral or institutional facilitator**, with no link to the program; or
2. **blind protocol**: the interviewer receives only the script of section 4, without the numbers of the funnel, without the prior, without
   the hypothesis of the program, and does not know which decision is behind the conversation.

The choice between the two forms, and the name of the facilitator, enter `DECLARACAO.md` **before** the first conversation.

## 4. The script, in the words of the group

Three questions, without a technical sheet and without the vocabulary of the program — no quantile, Beta, Brier, EVPI, F1 to F8:

1. *What is your worst scenario that is still plausible?*
2. *What would make you refuse this intervention?*
3. *When this policy fails, who in fact bears the cost?*

The interviewer records the answers in natural frequencies ("out of every 100 people, how many"), notes the date and the
place, and asks for the signature or the recorded consent of whoever spoke. What the group said is **not** translated
into the vocabulary of the program by whoever conducts: the translation, if there is one, is made later, separately, and stays marked as a
reading of the program.

## 5. Rule of no substitution

Whoever is not of the group does not speak for the group. It holds for the owner of the program, for the facilitator, for the researcher
heir of the base, and for any opinion-giver. A structural equivalent (professional association, public defenders' office,
sectoral representative, researcher heir of the original bases) enters **with the label of equivalent**, never as
the group. Declaring the label is part of the record, and the coverage of section 6 counts only what was truly heard.

## 6. Exit clause

- **Window:** 60 calendar days from the stamp of this declaration, seeking structural equivalents of the seven
  groups.
- **Minimum coverage:** **2 of the 7** groups, each one with a signed record or recorded consent.
- **If the coverage is not reached by the deadline:** the artifact is **archived as a methodological exercise of
  sensitivity (n = 1)**, prevented from becoming a public policy guideline or an F8 gate decision. The box is
  sealed and the seal has a hash.
- **If the coverage is reached:** the records enter `governance/GRUPOS.md`, the column "heard" ceases to say
  none in the covered lines, and the F8 gate is reopened only for them.

## 7. The voice of the operator does not count

The voice of whoever models is already entirely in the priors and in the adjustments, and the operator projects the rule while the group suffers
the consequence. Recording the operator in the column "heard" would be a structural conflict of interest and would annul the
objective of the front. **The operator does not count as a group heard, in any of the seven decisions.**

## 8. What stays in the box and what stays out

**In the box, without use in the field:** the seven decisions of section 1. None of them becomes a public policy guideline or an
issued decision of a gate while the coverage of section 6 is not reached.

**Outside the box, as it stands:** the five elicited rounds of E1 (`empirical/p12-shelf/formularios/`) remain as a
**sensitivity analysis with n = 1**, which is what the protocol of SHELF already declared. The official funnel continues
with the prior of the package.

## 9. Change of criterion

Script, deadline, minimum coverage, who conducts: none of that is rewritten after seeing a result. A change of criterion
is a **new declaration**, with its own hash, and counts in the K of the program.

## How to reproduce

```bash
cargo run --release --bin labs -- --escuta
```

The instrument reads `registro.tsv`, counts the coverage, checks whether any decision claims to be issued with nobody heard, and
gives the verdict of the seal. The negative proof is in the tests of the module: a line that claims to be issued with the column
"heard" as none is rejected.
