# Stochology: A Textbook

**21 chapters** · five parts · a three-step ramp per chapter

Oct 16, 2026 · Silvano Neto

---

## Preface: how to use this book

This book teaches Stochology, the science (or art) of judgement and decision under uncertainty, from end to end.
It presents the concepts of the field, the way of operating in an auditable architecture, and what happens when
that architecture meets real data: a health post and a bank call centre, a stock index and a training programme,
fire losses, the search for invariant causes in four sciences, and live forecasts of flights and arrests.

**Who it is for.** The book serves four readers. The first is the High School student who wants to understand
decision under uncertainty without waiting for university. The second is the undergraduate or graduate student
who has already seen basic probability. The third is the data professional, who needs to know why a model that
passed the test fails in production. The fourth is the decision-maker, who reads a number and needs to know how
far it authorizes them to conclude.

**What is assumed.** For Steps 1 and 2, only arithmetic, percentages, proportions and graph reading. Calculus and
linear algebra appear only where the text warns, and always after the geometric intuition. The laboratories are
written in **Rust**, with a fixed seed; whoever does not program can read the output of the laboratories, which is
quoted in the text, and skip the exercises marked as *Laboratory*.

**One rule that holds for the whole book.** Every number quoted here comes from an instrument that the reader can
run, or from a cited source. Where a measured number differs from one recorded elsewhere, both values and the cause
are kept in Appendix E, so that the chapters can teach without carrying the comparison.

**What that rule does not promise.** A number that anyone can run is a number anyone can **check**, not a number
that is **right**. The fixed seed reproduces a wrong account as faithfully as a right one, and the gates of the
architecture check form, presence and reproducibility, not correctness. Appendix E lists the errors of formula and
of code that passed every test and every gate of this book until an outside reading found them. Read each number as
an invitation to check it, not as a certificate.

---

## The three-step ramp

Each chapter crosses three steps, always in this order. The names are fixed and appear as internal headings.

**Step 1 — The real world and the paradox.** A concrete story, with people, round numbers and natural
frequencies ("in each 100 people..."), counting tables and graphs. No mathematical symbol appears before it has
been said in words. When the chapter needs a curve, it appears drawn and described by its curvature before
becoming a formula.

**Step 2 — The rule of the game.** The formalization. Each symbol is translated into simple words before the
equation; each theorem comes with the condition under which it holds and the case in which it fails. This is
where Bayes, Jensen, Meng, Murphy, Pearl, Kelly and Chouldechova appear.

**Step 3 — Hands on.** A self-contained laboratory in Rust, with a fixed seed, followed by exercises of three
types: *Concept* (answer with words), *Count* (answer with a number) and *Laboratory* (answer with code that
someone else runs and obtains an identical result).

**Why three steps.** Because every idea in this book has two predictable learning failures. The student who
learns the formula without seeing the world applies the formula to the wrong world. The student who sees the
world without formalizing cannot audit anything. The third step exists to close the cycle. If the theorem is
right, it runs.

---

## Map of the five parts and of the 21 chapters

| Part | Chapters | Question it answers |
| --- | --- | --- |
| **I. The art of aiming in the dark** | 1 to 4 | What is it to judge under uncertainty, and where do errors live? |
| **II. The traps of the world and of the data** | 5 to 8 | Why does the mean lie, how do gigantic samples deceive and how is good forecasting measured? |
| **III. Cause, time and survival** | 9 to 11 | What is a cause, why does time ruin it and who pays for the error? |
| **IV. Building the auditable machine** | 12 to 15 | How do you turn that thinking into a machine that audits itself? |
| **V. The machine in the real world** | 16 to 21 | How does the machine face real data, case by case? |

| Ch. | Title | Central idea |
| --- | --- | --- |
| 1 | The Invisible Target | *stókhos*; the stitching of three traditions; why the fixed seed is the basis of the check |
| 2 | World, Model and Agent | the bridge thesis; Peirce; the three senses of bias; the prosecutor's fallacy |
| 3 | Probability without mystery | five readings of probability; Bayes in trees and in odds; decibans |
| 4 | Luck, ignorance and the Christmas turkey | the three types of uncertainty; Stirling matrix; the model that ages in silence |
| 5 | When the mean lies | Jensen on curves; convexity of the wait; variance charges a toll |
| 6 | The invisible filter | Meng's identity; MCAR, MAR, MNAR; the independent unit |
| 7 | Hunting ghosts in the noise | exact null model; garden of forking paths; type I, II, III, S and M errors; severity |
| 8 | The forecaster's thermometer | proper scores; Murphy decomposition; calibration is repaired, resolution is not |
| 9 | The ladder of cause and effect | Pearl's three steps; confounder against collider; ICP |
| 10 | Time and ruin | non-ergodicity; volatility drag; Kelly; wild tails |
| 11 | The price of error | cost/loss; EVPI; Goodhart; Chouldechova's trilemma; exit clause |
| 12 | The map of knowledge | the typed hypergraph; the five lenses; pathology edges |
| 13 | The team of specialists, the safeguard panel and the two systems | eight specialists; the safeguard panel A1–A8; base-rate correction and SBC; System 1 and System 2 |
| 14 | The iron contract | cards; four prohibitions; invariants Φ1–Φ5; property tests; the funnel; gates |
| 15 | The vault and the seal | pre-registration with hash; counter K; the vault; auditing one's own claims |
| 16 | The health post and the bank | the same mean, two waits; Erlang C against the real queue |
| 17 | The stock market and the job | 3x leverage; LaLonde; severity that fails the method itself |
| 18 | The Danish fires | wild tail; Laplace; failing your own model is a victory |
| 19 | Seven rounds of ICP | five honest failures; light, heart and yeast |
| 20 | The live clock | zero leakage with a canary; Goodhart measured; three vaults |
| 21 | The seven lessons and the final project | the lessons with their conditions; objections; the resolution axis; the final project |

---

## Reading tracks

| Reader | Path | Suggested time |
| --- | --- | --- |
| High School | chapters 1 to 8, only Step 1 and the *Concept* exercises | 8 meetings |
| Undergraduate | chapters 1 to 11, the three steps, laboratories in Rust | one semester |
| Data professional | chapters 1, 2, 5, 6, 8, 14, 15 and Part V | four weeks |
| Whoever wants only the architecture | chapters 2, 12, 13, 14, 15 | two weeks |
| Whoever wants to see the method in action | the whole of Part V, returning to the foundations when needed | two weeks |

**How hard each part is.** The promise to the High School reader holds for Step 1 of Parts I and II, and for the
*Concept* exercises. Step 2 asks for more as the book advances: probability and logarithms from Chapter 3,
derivatives from Chapter 5, regression from Chapter 9, and stochastic calculus (Itô and Stratonovich), variational
free energy and nonparametric tests in Parts III to V. Those passages are written for the undergraduate and the
professional, and a first reading can skip them without losing the thread of Step 1 and Step 3.

---

## Conventions

**Fixed seed.** Every laboratory declares the seed. Two people on different machines have to obtain the same
numbers; that is what makes the check possible (Chapter 1, Step 3).

**Measured numbers.** The values quoted in Step 3 come from the instrument:

```bash
cargo run --release --bin labs -- --chapter 4     # one chapter
cargo run --release --bin labs -- --all           # every laboratory
```

The exercises come in three types: *(Concept)* requires words; *(Count)* requires a number; *(Laboratory)*
requires code that reproduces the result.

**Notation.** $L_1$ is the World, $L_2$ is the Model, $L_3$ is the Agent. $\operatorname{do}(x)$ is the
intervention. $\mathbb{E}$ is the long-run mean, $\operatorname{Var}$ is the dispersion around it. Appendix B
translates every symbol of the book into English and into code.
