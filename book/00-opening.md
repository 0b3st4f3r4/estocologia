# Stochology: A Textbook

**21 chapters** · five parts · a three-step ramp per chapter

Oct 16, 2026 · Silvano Neto

---

## Preface: how to use this book

This book teaches Stochology from end to end. The text presents the concepts of the field, the way of operating
in an auditable architecture and the behaviour of that architecture when put to the test on real data, from
horizon H0 to H3. The research that sustains it was closed on October 15, 2026.

**What changed since the earlier text.** The earlier text had 14 chapters and assumed university-level calculus and
probability already mastered. This book unfolds that content into 21 chapters, grouped into five parts, and
builds a **continuous pedagogical ramp**. The first step of each chapter uses only High School mathematics
(proportions, sets, means, counting trees, graphs of first- and second-degree functions). Nothing from the
earlier text was thrown away. Every idea, every theorem and every number of the research remains here,
redistributed into smaller steps.

**Who it is for.** The book serves four readers. The first is the High School student who wants to understand
decision under uncertainty without waiting for university. The second is the undergraduate or graduate student
who has already seen basic probability. The third is the data professional, who needs to know why a model that
passed the test fails in production. The fourth is the decision-maker, who reads a number and needs to know how
far it authorizes them to conclude.

**What is assumed.** In this book, for Steps 1 and 2, only arithmetic, percentages, proportions and graph
reading. Calculus and linear algebra appear only where the text warns, and always after the geometric
intuition. The laboratories are written in **Rust**, with a fixed seed, in the crate
`crates/stochology-book`; whoever does not program can read the output of the laboratories, which is quoted in
the text, and skip the exercises marked as *Laboratory*.

**One rule that holds for the whole book.** Every number quoted here comes from an instrument that the reader
can run. When a number in this book diverges from the number recorded in the earlier text or in a
course outline, the divergence is **declared in the text**, with both values, instead of hidden. It is the rule
that the research applies to itself (section 9.7 of the earlier text, today Part V, Chapter 21).

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
| **V. The machine in the real world** | 16 to 21 | How does the machine face real data, horizon by horizon? |

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
| 13 | The team of specialists | bands F1–F8; the adversarial panel A1–A8; AI under contract |
| 14 | The iron contract | cards; four prohibitions; invariants Φ1–Φ4; gates |
| 15 | The vault and the password | pre-registration with hash; counter K; auditing against itself |
| 16 | H0: the health centre and the bank | the same mean, two waits; Erlang C against the real queue |
| 17 | H1, part 1: the stock market and the job | 3x leverage; LaLonde; severity that fails the method itself |
| 18 | H1, part 2: the fires | wild tail; Laplace; failing your own model is a victory |
| 19 | H2: seven rounds of ICP | five honest failures; light, heart and yeast |
| 20 | H3: the live clock | zero leakage with a canary; Goodhart measured; three vaults |
| 21 | Seven lessons and what is missing | balance, background audit and the student's final project |

---

## Reading tracks

| Reader | Path | Suggested time |
| --- | --- | --- |
| High School | chapters 1 to 8, only Step 1 and the *Concept* exercises | 8 meetings |
| Undergraduate | chapters 1 to 11, the three steps, laboratories in Rust | one semester |
| Data professional | chapters 1, 2, 5, 6, 8, 14, 15 and Part V | four weeks |
| Whoever wants only the architecture | chapters 2, 12, 13, 14, 15 | two weeks |
| Whoever wants to see the method in action | the whole of Part V, returning to the foundations when needed | two weeks |

---

## Conventions

**Fixed seed.** Every laboratory declares the seed. Two people on different machines have to obtain the same
numbers; that is what makes the check possible (Chapter 1, Step 3).

**Measured numbers.** The values quoted in Step 3 come from the instrument:

```bash
cargo run --release --bin labs -- --chapter 4     # um capítulo
cargo run --release --bin labs -- --all     # todos os laboratórios
```

**Consolidated registry.** On 2026-10-16 the Python measurement scaffolding left the tree. The fronts quoted in
the chapters (`h0-real/`, `h1-real/`, `h2*/`, `h3/`) were consolidated in `docs/EMPIRICAL_REGISTRY.md`, which
carries each number with the mark of the Rust instrument that reproduces it, and the original material lives
in commit `1bfec905`. Mentions of the old fronts in the text are **historical**, and say where the number came
from, not a live path.

**Divergence declaration.** When this book quotes a number from the research repository and the instrument of
this book returns another, both appear, with the probable cause.

The exercises come in three types: *(Concept)* requires words; *(Count)* requires a number; *(Laboratory)*
requires code that reproduces the result.

**Notation.** $L_1$ is the World, $L_2$ is the Model, $L_3$ is the Agent. $\operatorname{do}(x)$ is the
intervention. $\mathbb{E}$ is the long-run mean, $\operatorname{Var}$ is the dispersion around it. Appendix B
translates every symbol of the book into English and into code.
