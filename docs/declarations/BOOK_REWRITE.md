# Declaration: The Content-Only Rewrite of the Book (2026-10-18)

On 2026-10-18 the owner decided that the book spent too much prose on metacommentary (file paths, references to
the book, the repository and the program, divergence declarations, and the same 60-line R-axis block in all 21
chapters) and that it should be rewritten as content only: the whole theory of the research and nothing else,
teaching rather than only demonstrating. This file records what changed in the contract and where the sections
that left the chapters now live. **No number changed**: every number in running prose must have a source, and every
output block is held to its instrument by `scripts/book/check_labs.py`.

## What Changed

| Before | After |
| --- | --- |
| A divergence declaration inside each chapter that measured a value different from the legacy registry | Appendix E (provenance of the numbers), chapter by chapter, with both values and the cause |
| The full `labs --resolution` block in every chapter, with "no loss measured in this chapter" in fourteen of them | The slice that teaches, in Chapters 5, 8, 10, 16 and 21; the instrument still prints all 21 lines and `book/r-axis.tsv` is unchanged |
| File paths, crate names and `cargo run --release` without a target | One command per laboratory, `cargo run --release --bin labs -- --chapter N` or the named instrument |
| Horizon labels (H0 to H3, H2b to H2g, F1 to F8) as chapter vocabulary | The real cases taught as cases; rounds numbered 1 to 7; the eight specialists named by their craft |
| Chapter 13 introduced System 2 in one sentence | Chapter 13 teaches System 2 (tree search, the bounded belief state, directed stress and Metropolis) with the `labs --s2` block, the base-rate correction of Saerens, Latinne and Decaestecker, and calibration by simulation; `check_labs.py` gained `labs --s2` |
| Chapter 14 cited property tests without teaching them | Chapter 14 teaches property-based testing |
| Chapter 13 cited `labs --s1` over a block that was in fact the R-axis output | The real `labs --s1` output |
| Chapter 10 output read "gregarine:" | "reading:" (a typo in the instrument string, fixed in `crates/stochology-book/src/ch10.rs`) |
| Portuguese decimal commas and Portuguese field names in several chapters | English notation and the English field names of the code |
| Chapters 15 and 21 carried the stocktaking of the self-audit and the open fronts | The chapters keep the method and its lessons; the stocktaking is kept below, verbatim as it stood |

`scripts/style/check_pass.py` checks every rewritten file for the five ramp markers, the 700-word floor, and number
provenance (`scripts/style/decisions.md`, "Reescrita de 2026-10-18"). The live state of the open items is in
`docs/DECLARED_PROPOSALS.md`; the text below is the record of what the chapters said on 2026-10-18.

## Chapter 15, as it stood: the background audit

**The background audit: what was assumed and what was measured.** Governance also serves to mark the points
**not** measured, and to measure them afterwards. The research turns against itself in a background audit. Ten
points that the text sustained and that had never been put to the test became fronts of work, to which were
added, later, the extensions (rare event, health, card and contract) and the front of human elicitation, which
waits for people. Each front repeats the rule of the horizons: the declaration with hash comes **before** running, the
result is reported with the limits, and a change of criterion after seeing the result enters as a new
declaration and counts in the number of attempts.

| Point | The assumption | What was measured | Result |
|

## Chapter 21, as it stood: the balance, the open fronts and the seal of the listening

**Two loose ends stayed on paper.** The two remain open and both depend on people. The first is the elicitation
of experts. The panel protocol and six forms are written, with the criterion of $K = 2$ ready, and
**nobody was interviewed**. No forecast was corrected, nothing was signed. Without that panel, five of the 33
domains change status or action depending on the prior chosen. The second is the voice of whoever pays for the error. The
record of the affected groups exists and says that none was heard.

**And one loose end that was only about resolution.** In the yeast genetics, the validation that supported the replica decided with
**one mutant per environment**: the smallest possible p-value there was $1/1\,000$, and the result came out at $p = 0{,}001$, which
is the floor. The p was not the problem — a row was missing. The design of that absence has an instrument today: with one row per
mutant the test approves **4 of 40** (what one looks for, a row artefact and two that are pure noise, all at the floor
of $0{,}001$, without the p separating one from the others), and with the second row it approves **1 of 40**, with the floor dropping to
$10^{-6}$ and the estimate stopping being a point (`labs --genetics`, Chapter 19). What remains open is
running the 40 **real** genes with the second row: the instrument measures the design, not the data of the yeast.



**The balance of the ten points of the background audit.** The table brings the question and the answer of each point.

| Point | The assumption | What was measured | Result |
| --- | --- | --- | --- |
| 1. Central thesis | The architecture catches errors that common practice lets through | Blind pilot of 12 cases, two independent arms | Tie at the ceiling: 8 of 8 defects in both arms and 0 false alarm in the 4 clean cases — the thesis **does not hold** |
| 2. Independence | The approved outputs do not depend on who wrote them | Four results reimplemented only from the specification | All agree and zero errors in the original; several choices that the specification does not fix coincide with the ones of the author, three of them only in STAR (weak evidence) |
| 3 and 4. Multiplicity | The gate of H2 was tried few times and nothing was adjusted afterwards | Inventory of the attempts and of the vault openings | $K = 38$ distinct tests (32 to 72); with Bonferroni H2g keeps the 4 areas **before the clustering correction**, and after the correction it falls to three (Chapter 19) |
| 5. Priors | The literature priors were not checked with experts; the control round had no tolerance | Three rounds in 33 domains and the equivalence band of 0.05 sd | Five domains change status or action with the literature prior and two with the weak one; the maximum deviation is 0.0071 sd and **33 of the 33** match |
| 6. Performative loop | Whoever abandons the queue calls back with probability 0.6 | Quasi-experimental design in a real call centre, 1999 | $0{,}145$ [0.132; 0.157] — four times less than the declared one |
| 7. Dependent rows | The ICP assumes independent rows, and there are 16 instants per volunteer and 80 schools | Clustered Wald, block permutation and the genetics replica with 40 new genes | Pharmacology survives intact, education leaves, genetics replicates in one gene validated with $p = 0{,}001$ |
| 8. Calibration | The tolerance of the ECE can be fixed at 0.03 | Simulated null with the same forecasts and the same $n$ | None of the six binary vaults passes the rule that depends on $n$; by the secondary rule, the picture does not change |
| 9 and 10. Scale and data | The funnel scales and the graph measures knowledge | 329 domains in batch and two new health domains | Two pruned in the severe test and 327 to the end; 26,634 nodes in the hypergraph, of which 6,494 are of concept |
| Rare event | A rare event forecaster is useful to decide when it has good calibration | Brier decomposition in three vaults: indomethacin, COVID and Rhode Island | No path wins in the three: only in Rhode Island does the resolution beat the reliability (+2.74643e-4 in the logistic and +3.11655e-4 in the ranking), and the skill stays at +0.018629 and +0.017952, below the declared +0.05; in COVID the ordering is good (AUC 0.7685) and the skill is $-0{,}020853$ |
| Health | The circle that the health post opened closes with real health data | Early sepsis in the open mirror of PhysioNet, with the funnel of H2g run end to end, without a single line changed and with the vault in the second hospital system | The effect exists and is precise ($\beta = 0{,}0388$ in log-odds per breath per minute, 90% interval from 0.0257 to 0.0523), but the invariance does not: 0 of 512 subsets accepted, $\hat{S} = \emptyset$, and the gate fails; the skill in the vault is $+0{,}00568$ and the recorded decision rate is 8.4% in the admissions of one system and 3.9% in the ones of the other |
| Card and contract | The question that the model answers is the one that the decider asked; the data contract does not say why the datum is missing | Mandatory field `pergunta_do_decisor` in the domain package (18 of H2g, 12 of H2 and the card of H0) and the absence block with MCAR, MAR or MNAR in the contract | The two fields enter validated in the code; in today's contracts the rule of absence is vacuous — no column has a single missing value — and no published number changes |
| Human elicitation | The simulated prior would be replaced by a panel of experts | Panel protocol and six forms, with the criterion of $K = 2$ fixed | Nothing was elicited: no interview, no corrected forecast, no signed line of decision |

**Retirement note.** The artifacts of the extensions of rare event, health and scale (points 9 and 10) left the
tree on 2026-10-16. They do not support a central result of the theory, and the numbers continue recorded in the table above. The material returns with `git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros`.

Three of those results tighten the authorisation of the previous chapters. The loop of 0.6 that H3 corrected is about
four times the value measured in a real case. The tolerance of 0.03 of the calibration error is larger than the noise in all
the large vaults, and none of them passes the rule that depends on $n$. And the transfer gate of H2 depends on
which version of each choice counts. The area of education does not survive the clustering correction, and the third area
now stands on the replicated genetics.



**Three fronts of people remain open, and genetics closed in code.** The three that ask for people from outside are not solved with more code.

- **The elicitation with human experts.** The protocol and the six forms are ready, with five domains
  elicited by the first expert with skill measured in the pre-test; the other two per domain and the one from outside are missing. Five of the 33 domains depend on the prior chosen, and that is the size of what remains to be closed in the panel.
- **The resolution of the validation by intervention of genetics (closed in code in item four).** The design is measured: one row per mutant approves
  4 of 40 at the floor of $0{,}001$ and the second row approves 1 of 40, with the floor at $10^{-6}$ (`labs --genetics`).
  The instrument separated the row artefact from the true mover; only the wet bench of the 40 real genes with a second lineage would stay out of the programme, and the p-value was never the problem.
- **The bridge thesis at scale.** The blind pilot tied at the ceiling and the thesis remains **not supported**, now with a gate:
  a record that declares it supported without the fair test is rejected, and the pilot is re-derived from the notes of the judge
  (`labs --bridge`). The fair test needs a case selector from outside the programme, human analysts and a
  human judge, and the deadline is 2027-01-14 — expired without them, the thesis stays unsupported for good, and the owner already
  accepted that consequence in writing.
- **The voice of whoever pays for the error.** In the seven decisions that affect real people, no affected group was heard. The
  rereading of the 21 opinions was done (17 confirmed, 4 withdrawn and reissued), and it does not replace the listening:
  the signature of the responsible one is from whoever decides to use the artifact, not from whoever pays for the error. The front now has a
  protocol frozen with a hash (`empirical/p13-escuta/`) and the box sealed by instrument: the seven decisions stay
  as sensitivity analysis ($n = 1$) and do not become a public policy guideline nor an F8 gate decision until the
  minimum coverage of 2 of the 7 groups is reached, or until 2026-12-15 (`labs --listening`).



**The seal of the listening.** The second instrument of the chapter is the gate of whoever pays for the error. It reads the machine
record of the seven decisions that touch real people, counts the listening coverage, checks whether any decision claims to be
issued without anyone heard and gives the verdict of the seal. The protocol is in `empirical/p13-escuta/protocolo.md`, frozen
with a hash before the first attempt of contact:

```bash
cargo run --release --bin labs -- --listening
```

```text
== Point 13 :: listening to those who pay for the error ==
decisions: 7 | registry stamp: 2026-10-16 | conduct: protocolo cego | window deadline: 2026-12-15 | minimum coverage: 2 of 7
decision                               holders heard        record     may issue
h2g/saida/direito_teto_beneficio             - nenhum       -          nao
h2g/saida/educacao_star_turmas           11598 nenhum       -          nao
h2g/saida/saude_rand                         - nenhum       -          nao
h1-real/saida/b                          16177 nenhum       -          nao
h3/prisoes                                5226 nenhum       -          nao
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

listening coverage: 0 of 7 groups heard
divergences: none — no decision claims to be issued with nobody heard
verdict: BOX SEALED — the coverage of 0 stays below the minimum of 2; the decisions remain as sensitivity analysis (n = 1) and do not become public-policy guidance nor an F8 gate decision until 2026-12-15 or until the minimum coverage

without listening, one by one:
  h2g/saida/direito_teto_beneficio   | trabalhadores acidentados cobertos pelo teto (Kentucky 1980, Michigan 1982)
  h2g/saida/educacao_star_turmas     | criancas do jardim a 3a serie do STAR no Tennessee
  h2g/saida/saude_rand               | familias nao idosas do RAND HIE (anos 1970)
  h1-real/saida/b                    | participantes do NSW e controles do CPS
  h3/prisoes                         | detidos de Toronto 1997-2002
  h3/prisoes20                       | prisoes de Toronto 2020-2021
  h3/prisoes_ri                      | paradas de Rhode Island 2005-2015

-- negative proof: the decision that claims to be issued with nobody heard --
decisions: 3 | registry stamp: 2026-10-16 | conduct: protocolo cego | window deadline: 2026-12-15 | minimum coverage: 2 of 3
decision                               holders heard        record     may issue
h3/prisoes                                5226 nenhum       -          sim
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

listening coverage: 0 of 3 groups heard
divergences: REJECTED on ["h3/prisoes"]
verdict: BOX SEALED — the coverage of 0 stays below the minimum of 2; the decisions remain as sensitivity analysis (n = 1) and do not become public-policy guidance nor an F8 gate decision until 2026-12-15 or until the minimum coverage

-- positive proof: two groups heard, with a signed record --
decisions: 2 | registry stamp: 2026-10-16 | conduct: protocolo cego | window deadline: 2026-12-15 | minimum coverage: 2 of 2
decision                               holders heard        record     may issue
h3/prisoes                                5226 associacao de defesa publica de TORONTO registro-01.md sim
h3/prisoes20                             65276 coletivo de ex-detentos registro-02.md sim

listening coverage: 2 of 2 groups heard
divergences: none — no decision claims to be issued with nobody heard
verdict: BOX OPEN for the covered decisions — the coverage of 2 reached the minimum of 2
```

**None of the seven historical cohorts is within reach** — they are children of the STAR of the 1980s, families of the RAND HIE,
detainees of Toronto of 1997, drivers of Rhode Island of 2005 —, and the programme declares that instead of inventing a
consultation that did not happen. What the listening looks for is a structural equivalent: whoever today occupies the same position of whoever
paid for the error then. The script has three questions in plain language (worst plausible scenario, what would make one refuse,
who in fact bears the cost), whoever conducts it is from outside the programme or with a blind protocol, and the operator **does not count**
as a group heard. The deadline is 2026-12-15, the minimum coverage is of 2 of the 7 groups with a signed record, and what is not
covered stays filed as a sensitivity exercise. The negative proof is in the instrument itself: a
decision that claims to be issued with the column "heard" at none is rejected, and the box only opens when the coverage reaches
the minimum.

**The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it costs to ask
coarse. The row of this chapter, by the instrument of the axis (`labs --resolution`):

```text
== R horizon :: resolution of the question ==
paired arms: same arrivals and same service bases; only the service rate changes
demand 5 or 11 (the station of Chapter 16): mean capacity 8.0 | aggregate wait 86.48 min, fine wait 40.24 min, loss 46.24 min
   above 30 min: aggregated 0.6042, fine 0.4840
demand 7 or 9 (light oscillation): mean capacity 8.0 | aggregate wait 47.70 min, fine wait 41.73 min, loss 5.97 min
   above 30 min: aggregated 0.5168, fine 0.5063
demand 8 or 8 (no oscillation): mean capacity 8.0 | aggregate wait 41.56 min, fine wait 41.56 min, loss 0.00 min
   above 30 min: aggregated 0.5013, fine 0.5013
resolution ladder: R0-agregada -> R1-dia -> R2-hora -> R3-subpergunta

-- prediction 3: the oscillation threshold (failed day: more than 10% above 30 min; threshold: more than 1/4 of the days) --
  200 days per amplitude | 10 h per day | specification 1: the Erlang plan of the average day, 12 agents
   amplitude  failed days       fraction   loss (min) above30 R0
         0.0           41          0.205        -0.26     0.0612
         0.5           52          0.260        -1.89     0.0776
         1.0           51          0.255         1.97     0.0954
         1.5           64          0.320         2.60     0.1267
         2.0           66          0.330         2.27     0.1365
         2.5           73          0.365         7.04     0.1634
         3.0           80          0.400         8.93     0.2314
  threshold: amplitude 0.5 | resolution loss at the threshold -1.89 min | monotone: false

-- prediction 3, specification 2: the R0 scale calibrated on flat demand --
  the calibrated scale has 13 agents (the flat one fails on at most 10% of the days)
   amplitude  failed days       fraction   loss (min) above30 R0
         0.0           19          0.095        -2.24     0.0339
         0.5           30          0.150        -4.08     0.0467
         1.0           33          0.165        -0.66     0.0554
         1.5           39          0.195        -0.65     0.0739
         2.0           42          0.210        -1.12     0.0854
         2.5           53          0.265         2.87     0.1027
         3.0           66          0.330         3.36     0.1458
  threshold: amplitude 2.5 | resolution loss at the threshold 2.87 min | monotone: true

-- prediction 5: the transfer (surviving R1 implies surviving R0; the converse does not) --
  6 profiles x 200 days | day threshold 0.10 | violations of the implication: 0 | counterexamples: 3 | aggregate conservation checked: true
  reading: no profile survives R1 and fails at R0; 3 profiles pass at R0 and fail on some day

-- the R axis, chapter by chapter (the table is in book/r-axis.tsv; the reading is computed here) --
chapter  1 | no loss measured in this chapter | the chapter presents the target and the thesis; there is no scale decision
chapter  2 | no loss measured in this chapter | the three layers; the grain of the question does not enter the account here
chapter  3 | no loss measured in this chapter | probability without a scale decision
chapter  4 | no loss measured in this chapter | the three kinds of uncertainty; the R axis is orthogonal to them
chapter  5 | resolution loss 46.24 min (demand from 5 to 11) | the mean lies: the chapter is the post that oscillates between 5 and 11
chapter  6 | no loss measured in this chapter | the independent unit is Phi2; the R axis is its cousin in the grain of the question
chapter  7 | no loss measured in this chapter | severe tests and multiplicity; no scale decision
chapter  8 | resolution loss 5.97 min (demand from 7 to 9) | the calibration tolerance depends on the n of the vault: the oscillation from 7 to 9 is the light case
chapter  9 | no loss measured in this chapter | the ladder of cause; the grain enters as the unit of the environments
chapter 10 | resolution loss 0.00 min (flat demand) | drag and ruin are measured per day; without oscillation, refining buys nothing
chapter 11 | no loss measured in this chapter | the trilemma is the subquestion of R3: every hour, and for whom
chapter 12 | no loss measured in this chapter | the map of knowledge; no scale decision
chapter 13 | no loss measured in this chapter | the adversarial panel judges the card, not its grain
chapter 14 | no loss measured in this chapter | the contract carries the resolution field (R0 to R3) and Phi5; the lock only bites when the unit is a day or coarser
chapter 15 | no loss measured in this chapter | every vault is a new grain: the ruler of the R axis when the correction changes the question
chapter 16 | resolution loss 46.24 min (demand from 5 to 11) | the chapter brings the R0 to R3 table of the post and of the bank
chapter 17 | no loss measured in this chapter | the independent unit is the person; the R axis appears as the grain of the comparison
chapter 18 | resolution loss 0.00 min (flat demand) | the fires: the number changes with the window, and the window is the grain
chapter 19 | no loss measured in this chapter | grouping by school brings the area down; by student it does not: the independent unit in the vocabulary of the axis
chapter 20 | resolution loss 46.24 min (demand from 5 to 11) | the live vault: laboratory 0.0022 and real vault 0.0048 are two grains of test
chapter 21 | oscillation threshold at amplitude 2.5 (calibrated plan) | the chapter names the axis and predictions 3 and 5
```
