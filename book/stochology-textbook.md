# Stochology: A Textbook

**Edition in 21 chapters** · five parts · a three-step ramp per chapter

Oct 16, 2026 · Silvano Neto

---

## Preface: how to use this book

This book teaches Stochology from end to end. The text presents the concepts of the field, the way of operating
in an auditable architecture and the behaviour of that architecture when put to the test on real data, from
horizon H0 to H3. The research that sustains it was closed on October 15, 2026.

**What changed in this edition.** The previous edition had 14 chapters and assumed university-level calculus and
probability already mastered. This edition unfolds that content into 21 chapters, grouped into five parts, and
builds a **continuous pedagogical ramp**. The first step of each chapter uses only High School mathematics
(proportions, sets, means, counting trees, graphs of first- and second-degree functions). Nothing from the
previous edition was thrown away. Every idea, every theorem and every number of the research remains here,
redistributed into smaller steps.

**Who it is for.** The book serves four readers. The first is the High School student who wants to understand
decision under uncertainty without waiting for university. The second is the undergraduate or graduate student
who has already seen basic probability. The third is the data professional, who needs to know why a model that
passed the test fails in production. The fourth is the decision-maker, who reads a number and needs to know how
far it authorizes them to conclude.

**What is assumed.** In this edition, for Steps 1 and 2, only arithmetic, percentages, proportions and graph
reading. Calculus and linear algebra appear only where the text warns, and always after the geometric
intuition. The laboratories are written in **Rust**, with a fixed seed, in the crate
`crates/stochology-book`; whoever does not program can read the output of the laboratories, which is quoted in
the text, and skip the exercises marked as *Laboratory*.

**One rule that holds for the whole book.** Every number quoted here comes from an instrument that the reader
can run. When a number in this edition diverges from the number published in the previous edition or in a
course outline, the divergence is **declared in the text**, with both values, instead of hidden. It is the rule
that the research applies to itself (section 9.7 of the previous edition, today Part V, Chapter 21).

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
this edition returns another, both appear, with the probable cause.

The exercises come in three types: *(Concept)* requires words; *(Count)* requires a number; *(Laboratory)*
requires code that reproduces the result.

**Notation.** $L_1$ is the World, $L_2$ is the Model, $L_3$ is the Agent. $\operatorname{do}(x)$ is the
intervention. $\mathbb{E}$ is the long-run mean, $\operatorname{Var}$ is the dispersion around it. Appendix B
translates every symbol of the book into English and into code.

# Part I. The Art of Aiming in the Dark

## Chapter 1. The Invisible Target: What Is Stochology?

**Objectives.** By the end of this chapter you should be able to say what Stochology is dedicated to, where the
name comes from, which three traditions it stitches together, and why checking a result depends on the draw
being repeatable.

---

### Step 1 — The real world and the paradox

**The decision before the answer.** On a Tuesday morning, a doctor sees a 54-year-old man with chest pain,
and does not know how that case will end. She has in her hands an electrocardiogram, his age, his history,
and fifteen minutes. The decision is between admitting him and sending him home. If she admits him, the
hospital spends a bed that another patient will lack; if she sends him home, and the worst happens, nobody
will forget her name.

Notice what has just happened. **The decision was taken before the answer existed.** This is not negligence
of the health system. It is the condition of practically every decision that matters. The judge decides
without having seen the crime. The manager hires staff for a demand that has not arrived yet. You choose a
course before knowing whether you will like it.

We call **Stochology** the science (and the art) that studies this kind of judgement: **judgement and
decision under uncertainty**.

**In each 100 decisions.** To make the conversation concrete, the whole book uses natural frequencies.
Instead of "the probability of error is 9%", say: *in each 100 patients like this, about 9 receive the wrong
diagnosis*. Instead of "the prevalence is 1%", say: *in each 1,000 people who join the queue, 10 are sick*.
And instead of "the confidence interval covers the truth in 95% of cases", say: *in each 100 repeated
experiments, 95 give an interval that contains the real value*. The numbers do not change, but the second way
of saying it already shows who pays the bill.

**Where the name comes from.** *Stókhos* is the Greek word for the target one aims at, and also for the
well-oriented conjecture, the guess of someone who aims at what they do not see. In 1713, three years after
his death, *Ars Conjectandi* by Jacob Bernoulli was published: the art of conjecturing, which he called
*stochastice*. Over time, "stochastic" came to mean only "random", and lost precisely the interesting part,
**aiming well when you do not see the target**. This book wants to recover that meaning.

**The "art" part.** The definition says "science (or art)" on purpose. There is a part of judgement under
uncertainty that is formalized, and the whole book shows how: the coherence of beliefs (Chapter 3), updating
by evidence (Chapter 3), the margin of error on the right unit (Chapter 6), calibration (Chapter 8), the cost
of the error (Chapter 11). And there is a part that is not fully formalized: which model is worth taking
seriously, when to stop collecting data, who pays if the error happens. An honest discipline about
uncertainty has to be honest about the limits of its own formalism. That is why every chapter of this book,
at the end, declares where it can fail.

**Why two computers have to draw the same numbers.** A scientific experiment has to be **repeatable**, and
another person, on another machine, next year, has to obtain an identical result. If the program draws
numbers at random on every run, nothing can be checked, and every round gives a different answer; the
discussion becomes "on my computer it gave another value".

The solution is simple, and **the seed** is the basis of every laboratory in this book. A pseudo-random
number generator is a deterministic recipe. Given the same starting seed, the recipe produces the same
sequence of numbers, always, on any machine. The "chance" of the experiment is a *reproducible* chance,
random for whoever looks at the result and auditable for whoever knows the seed.

---

### Step 2 — The rule of the game

**Definition.** Stochology is the science (or art) that studies judgement and decision under uncertainty.
Formally, it studies how an agent turns incomplete information into an action and answers for the
consequences.

**The stitching of three traditions.** The discipline stitches three traditions that grew up separately and
rarely cite each other; it is not a new branch of statistics.

- **The philosophy of induction.** Hume showed that every defence of induction presupposes that the future
  will resemble the past. Popper replaced confirmation with refutation. Lakatos showed how theories react
  when refutation arrives.
- **Statistical inference.** Pascal and Fermat, Bernoulli, Bayes, Laplace and Kolmogorov built probability;
  Fisher, Neyman and the Bayesian school built inference.
- **The mechanics of stochastic processes.** Markov and Langevin studied how chance accumulates over time.

**Lakatos's adjustments.** Duhem and Quine recalled that a frustrated prediction refutes the theory together
with its auxiliary hypotheses, never alone. Lakatos separated two ways of saving the theory. The
**progressive** adjustment predicts something new that is later confirmed; in 1846, the anomaly in the orbit
of Uranus did not bring Newton down, because Le Verrier postulated a hidden planet and Neptune was found.
The **degenerative** adjustment only protects the theory, and the equivalent manoeuvre, with the planet
Vulcan, failed to explain Mercury. Every change of specification after a failure (and this book reports
several) has to answer one question. Did the repair predict something new, or did it only hide the error?

**The question that changes.** Instead of asking "what is the probability?", Stochology formulates three
others. **In which layer can this claim fail? What kind of uncertainty threatens it? What test would it have
to face to deserve trust?** The rest of the book answers those three questions.

---

### Step 3 — Hands on

The laboratory of this chapter computes nothing interesting, but it shows *why* everything that comes later
is checkable. The code is in `crates/stochology-book/src/ch01.rs`, and the output below was measured with
`cargo run --release --bin labs -- --chapter 1`.

```text
== Chapter 1 :: the invisible target ==
two machines with the seed 1713 agree in 10000 of 10000 draws: true
draws coinciding with the next seed: 0
seed 1713: frequency of heads in 100 thousand tosses = 0.5000
seed 1909: frequency of heads in 100 thousand tosses = 0.4992
seed 2026: frequency of heads in 100 thousand tosses = 0.4987
```

Read line by line. **10000 of 10000**. Two independent "machines", with the starting seed in common, produce
identical sequences, and that is what makes the experiment auditable. **0**. When the seed changes, the
sequences have nothing in common (zero matches in ten thousand draws). The seed was not chosen to force a
result; it is only the starting point. **0.5000, 0.4992 and 0.4987**. Three different seeds, three honest
coins, head frequencies less than 0.0013 away from one half in 100 thousand tosses. Chance, when it
accumulates, behaves.

**The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it
costs to ask coarsely. The line of this chapter, from the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Explain, in your own words, why the definition says "science (or art)". Give an example of a
   decision in which the "art" part is unavoidable.
2. *(Concept)* A newspaper writes: "the draw was audited and nobody knows the result beforehand". If the
   program that performs the draw has a fixed seed, does whoever knows the seed know the result? Why does
   that not invalidate the draw?
3. *(Count)* You repeat the experiment and get 0.53 heads in 100 tosses. In 100 thousand tosses, would you
   expect a similar deviation? Justify with one sentence about how chance behaves when it accumulates.
4. *(Laboratory)* Run `labs --chapter 1` and then change the seed from 1909 to 1909 in the file. Does the result
   change? Change it to 1910 and measure how many matches appear in 100 thousand draws.
5. *(Concept)* Name a decision of your own life in which the answer is only known months later, and say
   which information you would have liked to have had and did not.

**Summary.**

- Stochology is the science (or art) that studies judgement and decision under uncertainty.
- The name comes from Bernoulli's *stókhos* (1713). Aiming well when the target cannot be seen.
- It stitches the philosophy of induction, statistical inference and stochastic processes.
- Checking a result depends on the chance of the experiment being reproducible. That requires a fixed seed.

## Chapter 2. World, Model and Agent: where do errors live?

**Objectives.** By the end of this chapter you should be able to separate the three layers of reality, state the
bridge thesis, apply the three forms of Peirce's inference, distinguish the three senses of the word "bias"
and recognize the prosecutor's fallacy.

---

### Step 1 — The real world and the paradox

**The territory, the map and the driver.** You are in a car in an unknown city. There is the real city:
streets, potholes, roadworks, the truck stopped at the corner. There is the map, a sheet of paper that
represents the city, always incomplete and always out of date somewhere. And there is the driver, you,
hungry, in a hurry, with divided attention, deciding at every crossing.

The three are different things, and the map is never the territory. But the driver only acts through the
map. Almost every error this book studies is born there. **Not in the map, but in the passage from the world
to the map, and from the map to the driver's hand.**

**The queue at the health centre.** In each 100 days, the centre receives patients in two ways. On half the
days 5 people per hour arrive; on the other half, 11 arrive. On average, 8 people per hour. The centre
serves 12 per hour.

The manager's claim is this: *"average demand is 8, the centre serves 12, so the wait is about 10 minutes and
there is no reason to hire"*. The arithmetic is right for a world in which exactly 8 people per hour arrive,
every hour.

Except that the world is the other one. Simulated with the same seed, the two worlds give:

| World | Mean wait per patient | In each 100 patients, how many wait more than 30 min |
| --- | --- | --- |
| Always 8 per hour | 9.3 minutes | 8 |
| Half the days with 5, half with 11 | 20.7 minutes | 25 |

No arithmetic was wrong. Mean demand is 8 in both cases. The **world** changed. When demand oscillates, the
queue on the busy day grows far more than the queue on the empty day shrinks, because the wait grows **on a
curve**. Whoever looked only at the mean, with the phrase "the average patient waits 10 minutes", decided
for a world that does not exist. Chapter 5 explains the curve; Chapter 16 redoes the whole case with the
repository numbers.

---

### Step 2 — The rule of the game

**The three layers.**

| Layer | Code | What it is | What is audited in it |
| --- | --- | --- | --- |
| World | $L_1$ | The reality that generates the data and that we never see whole | Physics of the noise, causal mechanisms, path dependence, heavy tails |
| Model | $L_2$ | The formal, simplified and simulable representation | Hidden assumptions, hypothesis space, likelihood, overfitting |
| Agent | $L_3$ | Who decides: a person, an institution or an algorithm, with limited time and attention | Heuristic shortcuts, calibration, utility, loops in which the action changes the World |

The three form a loop. The World generates data, the Model interprets it, the Agent acts, and the action
changes the World again. Each arrow is a **bridge** where the error enters. The data arrive filtered between
World and Model, the prediction is misread between Model and Agent, and the action deforms the World one
wanted to measure.

**The bridge thesis.** *A fallacy is almost always not an arithmetic error, but a misalignment between
layers.* It is worth saying what this thesis does **not** claim. It does not say that wrong arithmetic does
not exist, nor that whoever uses the architecture does not err. It says where to look first.

**The three forms of inference (Peirce).** Peirce obtained deduction, induction and abduction by permuting
the same three propositions: rule, case and result.

| Form | Goes from | To | Preserves the truth? |
| --- | --- | --- | --- |
| Deduction | Rule and case | Result | Yes |
| Induction | Case and result | Rule | No: it amplifies and can fail |
| Abduction | Rule and result | Case (the best explanation) | No: it amplifies and can fail |

Abduction chooses the hypothesis that makes the data least surprising, but only among the hypotheses
considered. It is the precursor of maximum likelihood and inherits its limit, which Chapter 4 will call
Knightian uncertainty.

**The three senses of "bias".** The word is used for three things that the discipline separates in order to
reunite later. They are the statistical bias of an estimator (Model), the bias of the process that produced
the data (World) and the cognitive bias of whoever reasons (Agent). In the three cases the error is not noise
that cancels with more data, but a distortion that accumulates.

**The thesis has an operational version.** Most of the biases in the classical catalogue correspond to a
model of chance that was ignored.

| Bias | Ignored model of chance |
| --- | --- |
| Base-rate neglect | The likelihood taken for the posterior |
| Availability, survival | A filtered sample taken for the population |
| Pattern illusion | The null hypothesis of chance forgotten |
| Gambler's fallacy | Memory attributed to a process without memory |
| Neglect of rare events | A heavy tail modelled as Gaussian |
| Overconfidence | Miscalibration |
| Motivated reasoning | The model chosen by the desired result |

**The prosecutor's fallacy.** The Agent reads the Model's likelihood, $P(\text{evidence} \mid
\text{innocent})$, as if it were the answer about the World, $P(\text{innocent} \mid \text{evidence})$.
The two numbers are different, and the difference depends entirely on layer $L_1$, that is, on how many
innocent people one is talking about.

**The inspection paradox.** The Agent imposes a stable Model on a World that oscillates, like the manager of
the centre, who projected the queue of the average day onto every day.

**The thesis was tested once.** Twelve cases were built from an external list of ten common statistical
errors (Makin and Orban de Xivry, 2019). Eight cases had a planted defect and four were clean; two
independent arms analysed them blind, one with this book's architecture, the other with the instruction to
analyse rigorously, without agents or gates. Both pointed to the 8 defects and raised zero false alarms in the
4 clean cases. The rule fixed in advance said that **a tie at the ceiling does not support the thesis**, and
that is what the result said. The tie does not refute it either. It shows that, for the ten errors on that
list, a competent analyst finds everything the other one finds. If the thesis holds, it holds for errors the
list does not cover, and a fair test of that needs harder cases, chosen outside the program, human analysts
and a human judge. Chapter 21 returns to this point.

---

### Step 3 — Hands on

Two tasks: represent the layers as data types, so that the program cannot confuse them, and measure the
prosecutor's fallacy in three different worlds.

**The layers as types.** In Rust, the separation ceases to be a stylistic recommendation and becomes a
compiler rule (`crates/stochology-core/src/camada.rs`):

```rust
pub enum Camada { Mundo, Modelo, Agente }

pub enum TipoIncerteza { Aleatoria, Epistemica, Knightiana }
```

Whoever writes a function that takes `Camada::Modelo` cannot pass a `Camada::Agente` without writing that
conversion by hand, and the manual conversion is the place where the reader of the code stops to think.

**The bridge measured.** The chapter's laboratory asks the following. A DNA coincidence happens by chance in
1 person in each 1 million. In three different worlds, in which the prior chance that the defendant is guilty
is 1%, 0.1% and 0.0001%, what is the correct answer about the defendant?

```text
== Chapter 2 :: the fallacy lives on the bridge ==
prevalence 1e-2: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.0001  swap factor=99.0
prevalence 1e-3: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.0010  swap factor=998.0
prevalence 1e-6: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.5000  swap factor=499999.7
```

The first column does not change across the three worlds, **0.000001**. It is the likelihood, and it is the
number the prosecutor quotes. The second column is the answer. With identical evidence and likelihood, the
chance of innocence goes from **0.0001** (0.01%) to **0.5** (50%). The third column shows how much the swap
lies: 99 times in the first world, 499,999.7 times in the last. The error is not in the arithmetic; it is in
answering about the World with a number from the Model.

**The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it
costs to ask coarsely. The line of this chapter, from the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Classify each item as World, Model or Agent: the real arrival rate of patients; the queue
   formula used by the manager; the decision to hire two nurses; the patients giving up when they see the
   queue.
2. *(Concept)* A news item says: "the chance of the DNA coinciding by chance is minuscule, therefore the
   chance of the defendant being innocent is equally minuscule". On which bridge is the error? Which
   quantity is missing to get to $P(\text{innocent} \mid \text{evidence})$?
3. *(Count)* In the laboratory, with prevalence 0.0001% ($10^{-6}$), the result is 0.5. Compute by hand which
   prevalence would give 0.09 and explain in one sentence why the answer depends so much on layer $L_1$.
4. *(Laboratory)* Modify the laboratory to use a coincidence of 1 in 1,000 instead of 1 in 1,000,000 and redo
   the table of the three worlds.
5. *(Concept)* Choose three biases from the table of the previous section and describe, for each one, an
   environment in which the shortcut would cease to be an error (Chapter 7 returns to this idea with
   Kahneman and Gigerenzer).

**Summary.**

- World ($L_1$), Model ($L_2$) and Agent ($L_3$) are distinct layers, and most errors live on the bridges
  between them.
- Peirce: deduction preserves the truth; induction and abduction amplify and can fail.
- "Bias" has three senses, from the data, from the estimator and from the mind, and all of them accumulate.
- The prosecutor's fallacy swaps $P(E \mid I)$ for $P(I \mid E)$; the swap lies by a factor that depends on
  the prevalence in the World.

## Chapter 3. Probability without mystery: bets, trees and Bayes' theorem

**Objectives.** By the end of this chapter you should be able to read the five interpretations of probability,
recognize a Dutch book, compute a posterior with natural-frequency trees and with odds, measure the strength
of an item of evidence in decibans, and separate Royall's three questions.

---

### Step 1 — The real world and the paradox

**"There is a 70% chance of rain tomorrow."** What is the meaning of that sentence?

- It may mean that *of the 10 days on which the sky looked like this, it rained on 7*. A count of the world.
- It may mean that *the numerical model, run 10 times with small perturbations, gave rain in 7*. A
  product of the model.
- It may mean that *I would bet 7 against 3 on it*. A disposition of whoever decides.
- It may mean that *of the equally possible cases, 7 favour rain*. A count of the model.
- It may mean that *given my knowledge, the rational degree of support for rain is 0.7*. A relation between
  propositions.

They are five different things with the same number on top. When someone reads one as if it were the other,
the error is not in the arithmetic. It is on the bridge.

**The friend who offers two tickets.** Your friend says the chance of rain is 60% and the chance of no rain
is 50%. You can buy two tickets from him. One pays 1 real if it rains, the other pays 1 real if it does not
rain. He charges 0.60 for the first, 0.50 for the second, 1.10 in total. Whether it rains or not, **one** of
the two tickets pays 1 real. You receive 1.00 and paid 1.10. You lost 0.10 with no risk at all.

That is a **Dutch book**: a combination of bets in which one loses money with certainty. The sum rule exists
to prevent that combination. Beliefs that obey probability have no Dutch book against them.

**In each 1,000 women.** A test detects a disease. In each 10,000 women, 100 have the disease and 9,900 do
not have it. The test is right in 90 of the 100 disease cases (90%) and gives a positive in 9% of the healthy
women. Let us count who tests positive:

- 90 sick women test positive.
- Of the 9,900 healthy women, 9% test positive. That is 891 women.
- Total positives: 90 + 891 = 981.

**Of the 981 women with a positive test, only 90 are sick**, about 1 in every 11. That is the answer that
matters to whoever has just received the test, and not the "90%" that the laboratory announces.

When Gigerenzer and colleagues asked 160 gynaecologists this question, **only 21% chose "about 1 in 10"**. It
is not stupidity; it is the bridge between the likelihood the test declares and the posterior the patient
wants to know.

**The court's arithmetic.** In the Sally Clark case, the prosecution presented the number "1 in 73 million",
obtained by squaring 1/8,543, as if the two deaths were independent. The prosecutor treated that number as
the chance that the mother was innocent. That number was the chance of a coincidence; the answer about
innocence required the prevalence in the World. That is the bridge of Chapter 2, now with fatal consequences.

---

### Step 2 — The rule of the game

**Why belief obeys probability.** Two independent arguments. Cox (1946) showed that any consistent measure of
plausibility obeys, after a rescaling, the rules of sum and product. Ramsey and de Finetti measured belief
by the price of a bet. A set of prices admits a Dutch book if and only if it violates the axioms.

**The five interpretations.** Each of the senses of "70%" has a name, and each lives in a layer.

| Interpretation | The probability is | Layer |
| --- | --- | --- |
| Classical (Laplace) | Favourable cases over equally possible cases | Model |
| Frequentist | The limit of the relative frequency in a long series | World |
| Propensity (Popper) | A physical disposition of the experimental arrangement | World |
| Logical (Keynes, Carnap) | A degree of partial support between propositions | Model |
| Subjective (Ramsey, de Finetti) | The degree of belief of an agent, measured by the price of a bet | Agent |

Stochology does not choose one. **It types every number by the layer it came from**. Many errors of Chapter
2 are swaps of reading.

**Coherence is cheap.** An agent who commits the conjunction fallacy loses 0.05 per round to the Dutch
bookmaker and 0.60 to whoever knows the world. Making the agent coherent reduces the loss only to 0.55. The
most visible error was not the most expensive one. **What punishes disagreement with the World is
calibration** (hence Chapter 8).

**Bayes in odds.** Written in odds, Bayes' rule separates the likelihood ratio from the prior odds:

$$\frac{P(H\mid E)}{P(\neg H\mid E)}=\frac{P(E\mid H)}{P(E\mid \neg H)}\times\frac{P(H)}{P(\neg H)}$$

*Posterior odds = likelihood ratio × prior odds.* Evidence is strong when the ratio moves away from 1; the
value of $P(E \mid H)$ does not measure that strength.

**Maximum entropy.** When information is incomplete, Jaynes (1957) tells us to choose, among the
distributions compatible with the available information, the one of greatest entropy. A six-sided die with
mean 4.5 then receives about 5% on face 1 and 35% on face 6; any other choice assumes something that was not
declared.

**Decibans.** Turing and Good measured the strength of evidence in decibans, ten times the decimal logarithm
of the likelihood ratio. On that scale, independent items of evidence **add up**. Independence is the
condition most often forgotten. Two positive tests with a common cause of error are not worth double.

**Royall in three questions.** Royall separated three questions that are usually answered with a single
number: *what do these data say about H against H′?* (likelihood ratio); *what should I believe now?* (it
requires a prior); *what should I do?* (it requires costs and losses, Chapter 11). The p-value answers none
of the three well.

**How much data is enough.** The expected value of the weight of evidence is the Kullback-Leibler
divergence, which is never negative:

$$D(P_1\,\|\,P_0)=\sum_x P_1(x)\log\frac{P_1(x)}{P_0(x)}\ \ge 0$$

To separate a 60% coin from an honest one with odds of 100 to 1, about **229 tosses** are needed. And the
evidence rarely deceives. If the null hypothesis is true, the chance that the likelihood ratio ever reaches
$k$ is at most $1/k$, whatever the sample size:

$$P_{H_0}\Big(\sup_n \mathrm{RV}_n \ge k\Big)\le \frac{1}{k}$$

In simulation, only **0.85%** of the paths under the null reached 20 decibans (ratio 100), against a bound
of 1%.

---

### Step 3 — Hands on

The laboratory builds the tree, measures the strength of the second test and shows how much the dependence
between tests costs. Code in `crates/stochology-book/src/ch03.rs`; output measured with
`cargo run --release --bin labs -- --chapter 3`:

```text
== Chapter 3 :: trees, odds and decibans ==
mammography: 100 sick, 90 sick positives, 9900 healthy, 891 false positives, 981 positives
P(sick|positive) = 0.0917  (one in every 10.9 positives)
second test: independent = 10.0 dB, correlated (effective ratio 4.2) = 6.23 dB
posterior with two positives: independent = 0.5025, correlated = 0.2979
```

Three lessons in three lines. **(1)** The tree gives 981 positives, of which 90 are sick. That is 0.0917, or
1 in every 10.9, and not 90%. **(2)** The second test is worth 10 decibans if it is independent of the first
and 6.23 decibans if there is a common cause of false positives (dense tissue, for instance), with the
effective ratio falling from 10 to 4.2. **(3)** With two positives, the belief goes from 1% to 50.25% in the
independent case and to 29.79% in the correlated case. Independence was not a detail. It is almost half of the
conclusion. The repository number for the correlated case is 30%, the rounded value that appears in the
course outline.

**The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it
costs to ask coarsely. The line of this chapter, from the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Count)* Redo the mammography tree for 1,000 women (instead of 10,000) and check that the answer is the
   same. How many positives, how many sick?
2. *(Count)* A woman tests positive on the second mammography. How many decibans is the second test worth if
   it is independent, and how many with the effective ratio of 4.2? What is the posterior in each case?
3. *(Concept)* Why did "1 in 73 million" commit two different errors? Name the bridge or the layer of each
   one.
4. *(Count)* How many tosses are needed to separate, with odds of 10 to 1, a 0.55 coin from an honest one?
   Use the Kullback-Leibler divergence and say in what sense the answer is an average.
5. *(Laboratory)* Change the effective ratio from 4.2 to 2.0 in the laboratory and redo the third line. At
   what point does the dependence between tests destroy the conclusion?
6. *(Concept)* Give a professional example in which you saw someone answer the question "what should I
   believe?" with the number that answers "what do the data say?".

**Summary.**

- Five interpretations of probability, typed by layer; Stochology does not choose one.
- Coherence avoids the Dutch book; calibration avoids losing to the World and costs more.
- Bayes in odds separates the strength of the evidence (likelihood ratio) from the prior belief; in natural
  frequencies, the same arithmetic becomes visible in a tree.
- Decibans add up when the items of evidence are independent; independence is the most fragile and the most
  forgotten hypothesis.

## Chapter 4. Luck, ignorance and the Christmas turkey: the three types of uncertainty

**Objectives.** By the end of this chapter you should be able to distinguish aleatory, epistemic and Knightian
uncertainty, use the law of total variance to separate them, place a problem in the Stirling matrix and
recognize a model that ages in silence.

---

### Step 1 — The real world and the paradox

**The Christmas turkey.** A turkey is fed every day, punctually, at 9 in the morning. On day 1 it knows
nothing; on day 10 it suspects that the food has something to do with the henhouse. On day 60 it already has a
theory. On day 100 it does the arithmetic of its days. *In 100 mornings, the food arrived in 100.* On the eve
of day 101, the turkey is **99% confident** that tomorrow, at 9 o'clock, the food arrives.

Day 101 is Christmas, and Christmas was never on the turkey's list of hypotheses. No extra datum would have
saved it. **More observations only increased confidence in the wrong place.** (Bertrand Russell told the
story with a chicken; Nassim Taleb retold it with a turkey.)

**In each 100 experiments.** Imagine two types of uncertainty living together:

- **The coin.** You toss an honest coin 100 times. In each 100 experiments of 100 tosses, the proportion of
  heads stays near 50%, but never equal to 50%. Tossing more times does **not** make the proportion turn into
  50%. The draw remains a draw.
- **The unknown coin.** You toss a coin without knowing whether it is honest. After 10 tosses you do not know
  yet; after 10,000 tosses you know almost for certain. Here, more data solves it.

These two cases have different names, and confusing them is expensive. There is also a third one, the
turkey's.

---

### Step 2 — The rule of the game

**The law of total variance.** The total dispersion of a result decomposes into two parcels:

$$\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{aleatória (Mundo)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistêmica (Modelo)}}$$

The first parcel is born from the draw and persists when the parameter is known. The second is born from
ignorance about the parameter and disappears when you identify it and have data.

| Uncertainty | Layer | What more data does | How it is mitigated |
| --- | --- | --- | --- |
| Aleatory | World | Nothing | Safety margins, redundancy |
| Epistemic | Model | It reduces them to zero, if the parameter is identifiable | Sampling, Bayesian updating |
| Knightian | Model and Agent | Nothing — or worse: false confidence | Scenarios, robustness, stress on assumptions, widening the hypothesis space |

Frank Knight (1921) separated **risk**, with known probabilities, from **uncertainty**, with probabilities
that cannot be measured. In the extreme case, which this book calls Knightian uncertainty or specification
error, the real process is **outside** the hypothesis space. That uncertainty is the most dangerous of the
three, because it does not appear in the equation. The interval narrows around the wrong answer.

**The Stirling matrix.** Two questions organize the diagnosis. Do we know which outcomes are possible? Do we
know their probabilities?

|  | Probabilities known | Probabilities unknown |
| --- | --- | --- |
| **Well-defined outcomes** | Risk: compute | Uncertainty: scenarios, sensitivity |
| **Ill-defined or disputed outcomes** | Ambiguity: deliberate on what counts as harm | Ignorance: vigilance, reversibility, diversity of options |

Russell's turkey is in the bottom right corner, and no calculation from the top left corner saves it.

**The typology of Walker and colleagues (2003).** Every uncertainty is classified along three dimensions.
*Location* says where it enters: context, model structure, parameters, inputs or results. *Level* goes from
deterministic certainty to total ignorance. *Nature* separates variability (aleatory) from lack of knowledge
(epistemic). In this book's architecture, location corresponds to the layer and to the field of the card
(Chapter 14), and the level says whether a margin is enough or whether a stress test is needed.

**Laplace's rule of succession.** On the eve of day 101, the turkey uses:

$$P(\text{success tomorrow}\mid s,n)=\frac{s+1}{n+2}$$

The rule is not neutral either, because it derives from a uniform prior. With seven successes in ten, the
uniform prior gives 0.667, a uniform prior over $\theta^2$ gives 0.692 and one over $\sqrt{\theta}$ gives
0.652. **Even ignorance has a shape**, and that is why the choice of prior has to be declared (Chapter 13,
band F7).

---

### Step 3 — Hands on

The laboratory places a world that changes underneath a model that does not change. A coin whose bias
**climbs from 0.2 to 0.6** over 6,400 tosses, 200 repetitions, with model 1 assuming a fixed coin and model 2
discounting the past with factor 0.99. Code in `crates/stochology-book/src/ch04.rs`; output measured with
`cargo run --release --bin labs -- --chapter 4`:

```text
== Chapter 4 :: the coin whose bias changes ==
bias climbs from 0.2 to 0.6 in 6400 steps, 200 repetitions, discount 0.99
coverage of the 95% interval: fixed model = 0.1199, model with forgetting = 0.9398
width of the fixed interval at step 100 = 0.1564, at step 6400 = 0.0240
width of the interval with forgetting at step 6400 = 0.1360 (ratio 5.7x the final fixed one)
```

The fixed model declares "95% confidence" and **covers the truth in 12% of the steps**. The interval shrinks
from 0.1564 to 0.0240 while the bias walks out of it. The model that forgets the past, with an interval
**5.7 times wider**, covers 93.98%, close to the nominal value. The price of honesty is the width of the
interval; the price of stubbornness is false confidence.

**Divergence declaration.** The previous edition of this book (and the course outline) records 11% coverage
for the fixed model and 99% for the model with forgetting, with widths from 0.140 to 0.035. This instrument,
with 6,400 steps and discount 0.99, measures **12%** and **93.98%**, with widths from 0.1564 to 0.0240 and a
ratio of 5.7×. The coverage of the fixed model agrees within one percentage point; the coverage of the model
with forgetting is lower because the width of the interval used here is that of the standard error of the
EWMA, and a wider interval (or a smaller discount) raises coverage above 97% — the parameter grid in
`crates/stochology-book/examples/grade_ch04.rs` shows the effect. Both values remain recorded; neither was
erased.

**The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it
costs to ask coarsely. The line of this chapter, from the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Classify each uncertainty: the result of the next toss of a coin; the true bias of that coin;
   the chance that the government changes the rule of the game next year; the measurement error of a
   calibrated scale.
2. *(Count)* Apply Laplace's rule with 7 successes in 10 trials and compare with the two alternative priors
   (0.667, 0.692, 0.652). Which of the three would you declare before seeing the data?
3. *(Count)* How many standard deviations away is the true bias at step 6,400, if the interval of the fixed
   model has width 0.0240 and the bias is at 0.6 while the fixed estimate points to near 0.4?
4. *(Laboratory)* Change the discount from 0.99 to 0.98 and to 0.995 in the laboratory. Explain, looking at
   the two coverages and the two widths, the trade-off between memory window and coverage.
5. *(Concept)* Describe a real case (from your area) in which the process changed regime and the model
   remained confident. In which corner of the Stirling matrix does it fall?

**Summary.**

- Aleatory uncertainty does not shrink with more data; epistemic shrinks; Knightian deceives.
- The law of total variance separates the first two precisely.
- The Stirling matrix and the Walker typology organize the diagnosis before the calculation.
- A world that changes slowly defeats a fixed model in silence: the nominal confidence survives the error.

# Part II. The traps of the world and of the data

## Chapter 5. When the mean lies: curves and Jensen's inequality

**Objectives.** By the end of this chapter you should be able to look at a graph and say whether the curve is
convex or concave; state Jensen's inequality and the second-order approximation that measures the variance
toll; recognize when the average case ceases to be the average result; and compute every margin on the unit
that is in fact independent.

---

### Step 1 — The real world and the paradox

**The health centre.** In each 100 days, the centre receives patients in two ways. On 50 days 5 people per
hour arrive; on the other 50, 11 arrive. The mean is 8 people per hour. The centre manages to serve 12 per
hour.

The manager looks at the spreadsheet and says: *"average demand is 8, the centre serves 12, there is slack;
the mean wait has to stay around 10 minutes, and there is no reason to hire"*. No arithmetic was done wrong.
And yet the world the spreadsheet describes does not exist.

The table below returns to the two worlds of Chapter 1, repeated here to measure the toll, and mean demand
does not change from one row to the other.

| World | Mean wait per patient | In each 100 patients, how many wait more than 30 min |
| --- | --- | --- |
| Always 8 per hour | 9.3 minutes | 8 |
| Half the days with 5, half with 11 | 20.7 minutes | 25 |

The mean changed in nothing. The mean wait more than doubled, and the number of people waiting more than half
an hour tripled. A decision about hiring staff, taken in the first world, is applied in the second.

**The curve.** Draw the function $y = x^2$ on a piece of paper. It is a parabola opening upwards. Mark two
points, $x = 0$ and $x = 2$. Their mean is 1, and the height of the curve at 1 is 1. But the mean of the
heights is $(0^2 + 2^2)/2 = 2$. Look at the drawing. The point of the curve in the middle (1) lies **below**
the chord that joins the two points (2). That is the asymmetry this chapter measures.

In a **convex** function (opening upwards, like $x^2$), the mean of the heights is always greater than or
equal to the height of the mean. In the **concave** one (opening downwards, like $-x^2$ or the square root),
the reverse holds, and the mean of the heights lies below the height of the mean.

**The queue is a curve opening upwards.** When demand comes close to service capacity, the wait does not grow
in a straight line; it grows faster and faster. In the world of the centre, that means the busy day is not
merely the mirror of the empty day. The busy day charges far more than the empty day gives back.

**The class with mean 6.2.** In an exam with a pass mark of 6.0, the class mean was 6.2. Whoever reads only
the mean concludes that "the average student passed". The conclusion would require a student who does not
exist, and the count contradicts the summary. **In each 100 students, 45 fell below 6.0**. A mean of 6.2
coexists without any problem with 45% of failures. As at the centre, the mean is not a wrong summary; it is an
**insufficient** summary for the question that matters.

---

### Step 2 — The rule of the game

**Convexity, in one line.** A function $f$ is convex when, for any two points, the chord that joins them lies
**above** the graph. It is concave when the chord lies **below**. The straight line is both things at the same
time, and it is the only case in which the two coincide.

**Jensen's inequality.** For a convex function $f$ and a random quantity $X$:

$$\mathbb{E}[f(X)] \;\ge\; f(\mathbb{E}[X])$$

In words, the mean of the result is **not** the result of the average case. Equality holds in two cases, and
only in them, when $f$ is a straight line, or when $X$ does not vary. Outside those two, a difference is left
over, the toll.

For a concave function, the inequality reverses: $\mathbb{E}[f(X)] \le f(\mathbb{E}[X])$.

**What the toll costs.** If $X$ stays near its mean $\mu$, the difference is approximately proportional to
the **curvature** and to the **variance**:

$$\mathbb{E}[f(X)] - f(\mathbb{E}[X]) \;\approx\; \tfrac{1}{2}\, f''(\mu)\,\operatorname{Var}(X)$$

The formula reads like a price, **curvature times dispersion**. The condition under which it holds is that the
variance be small enough for the curve to look like a parabola in that neighbourhood. It **fails** when the
dispersion is large, and the centre is exactly that case.

**The centre's queue.** The long-run mean wait of a queue with one attendant, with arrivals of rate
$\lambda$ per hour and service of rate $\mu$ per hour, is

$$W(\lambda)=\frac{\lambda}{\mu(\mu-\lambda)}$$

With $\mu = 12$ and $\lambda = 8$, that gives 10 minutes. Applied to the two days of the oscillating world,
$W(5) = 3.57$ min and $W(11) = 55$ min, and the mean of the two is 29.3 minutes, almost three times the 10
minutes of the claim. The second derivative of $W$ at $\lambda = 8$ is
$f''(8) = 120/(\mu-8)^3 = 1.875$ minute per (patient/hour)². With the variance of $X$ equal to 9, the
second-order toll is $\tfrac{1}{2}\cdot 1.875\cdot 9 = 8.44$ minutes. That is a lot, but it is less than
the 19.3 minutes that are missing. **The second-order approximation underestimates the toll when the
dispersion is large**, because the curve departs from the parabola before the day ends.

**The mean grade and the threshold.** Not every crooked function is a second-degree curve. Failing is a
**threshold**, and each student's outcome becomes a step at 6.0. The class mean answers the question "what is
the typical grade?" and does not answer the question "how many fail?". Whoever needs the second question and
uses the first is swapping the summary for the result, and the swap cannot be repaired by adding more
students.

**The unit of the margin.** Every number in this book comes with a margin, and the margin only makes sense if
it is computed on the **unit that is in fact independent**. At the centre, the question is this. Are the 320
thousand patients 320 thousand independent observations, or are the 4 thousand days 4 thousand independent
observations? Patients of one day share the demand of the day, the server and the queue. Treating them as
independent is **pseudoreplication**, and it makes the margin look five times smaller than it is. The test
that decides, with no convention at all, is the **halves null**: split the days at random into two halves and
measure the deviation of the difference between the two means. The null measures what the margin must predict.

**The prohibition of the solitary mean.** When the function is nonlinear, the dynamics multiplicative or the
tail heavy, reporting only the mean is prohibited. One reports the average case $f(\mathbb{E}[X])$, the
simulated mean $\mathbb{E}[f(X)]$, the median, the quantiles and the cumulative curve. Chapter 14 formalizes
the prohibition; here it is born from the toll we have just measured.

---

### Step 3 — Hands on

The laboratory runs four worlds of the centre, all with mean demand 8, measures the halves null in two of them
and counts the failures of the class. Code in `crates/stochology-book/src/ch05.rs`; output measured with
`cargo run --release`:

```text
== Chapter 5 :: when the mean lies ==
W(5) = 3.57 min, W(8) = 10.00 min, W(11) = 55.00 min
Jensen: f(E[X]) = 10.00 min | E[f(X)] = 29.29 (5 or 11) and 14.10 (uniform 5..11)
second order: half f''(8) Var(X) = 8.44 min (5 or 11) and 2.81 min (uniform)
                 8 every day:  319405 patients | mean   9.1 min | median   3.7 | p90  26.2 | >30 min:   7.6 per 100
                     5 or 11:  319115 patients | mean  19.6 min | median   8.6 | p90  56.1 | >30 min:  23.7 per 100
               uniform 5..11:  319843 patients | mean  13.8 min | median   5.5 | p90  39.8 | >30 min:  15.0 per 100
5 or 11, surge reinforcement:  319115 patients | mean   2.5 min | median   0.0 | p90   8.1 | >30 min:   0.4 per 100
on surge days: 27.0 min in 219458 patients (68.8% of the total); on mild days: 3.4 min
halves null in the 8 every day world: measured deviation 0.201 min | margin per day 0.185 | per patient 0.046 | wait variance 168 min^2
halves null in the     5 or 11 world: measured deviation 0.742 min | margin per day 0.563 | per patient 0.097 | wait variance 747 min^2
difference between the worlds = 10.6 min = 52.7 deviations of the null
median of the 5 or 11 world = 8.6 min, against the mean wait of 9.1 min of the constant world
class of 100000 students: mean = 6.21, failing (score < 6.0) = 45.1 per 100
```

Five readings. **(1)** Mean demand is 8 in the four worlds and the mean wait goes from 2.5 to 19.6 minutes.
Variability, not the mean, decides. **(2)** In the oscillating world, **68.8% of the patients** arrive on the
surge days, and that is why the mean per patient weighs the bad days. That is 27.0 minutes on surge days
against 3.4 on mild days. **(3)** The second-order toll (8.44 min) captures less than half of the true
difference (19.29 min between 10.00 and 29.29). The formula is local, and the centre is a case of large
dispersion. **(4)** The halves null gives 0.201 minute in the constant world, and the margin computed per day
predicts 0.185, almost equal; the margin per patient predicts 0.046, four times smaller. In the oscillating
world the difference between the two readings is 5.8 times, with 0.563 against 0.097. **(5)** The median of
the oscillating world is 8.6 minutes, practically the mean of 9.1 min that the claim promised. Whoever looks
at "the typical patient" sees the promise kept, and the whole difference is in the tail. The difference
between the worlds, 10.6 minutes, sits 52.7 null deviations away.

**Divergence declaration.** The course material and the H0 README publish, for these worlds, 9.3 min and 20.7
min of mean wait, **8** and **25** patients in each 100 above 30 minutes, a margin of ±0.51 minute per day
against ±0.07 per patient, and a halves null of 0.216 minute with predicted margins of 0.227 (per day) and
0.047 (per patient). This instrument measures **9.1** and **19.6** minutes, **7.6** and **23.7** in each 100,
a null of **0.201** minute with margins of **0.185** (per day) and **0.046** (per patient) in the constant
world, and, in the oscillating world, a null of **0.742** with margins of **0.563** and **0.097**. The
uniform world gives 13.8 min here against 12.3 in the material. The numbers are all in a close neighbourhood.
The difference has two probable causes: the generator (the material uses NumPy's PCG64 with
`SeedSequence(1909)`; here, SplitMix64 with seed 1909) and the start of each day, which here is always an
empty queue. Jensen's identity, the sign of the toll and the conclusion about the unit of the margin depend on
neither of them. No old value was erased.

**The grain of this question.** Every chapter has a decision with a grain, and the R axis measures what it
costs to ask coarsely. The line of this chapter, from the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* For each case, say whether the mean hides a toll and of which sign. (a) The waiting time in a
   queue; (b) the balance of an account with fixed yield; (c) the wealth of whoever bets 100% of the capital
   every round; (d) the grade of a class with a pass cut-off.
2. *(Count)* With $W(\lambda)$ and $\mu = 12$, compute $W(6)$, $W(8)$ and $W(10)$. Then compute
   $f(\mathbb{E}[X])$ and $\mathbb{E}[f(X)]$ for a world alternating between 6 and 10, and compare the true
   toll with $\tfrac{1}{2}f''(8)\operatorname{Var}(X)$.
3. *(Count)* In the laboratory, the margin per day in the oscillating world is 0.563 minute and the margin per
   patient is 0.097. By how many times does one underestimate the other? If a report published the margin per
   patient to claim that the 10.6 minute difference is significant, what would it have got wrong?
4. *(Laboratory)* Replace the "5 or 11" world with "4 or 12" and then with "7 or 9", keeping the mean at 8.
   Measure the mean wait and the fraction above 30 minutes in the three cases. Draw the three points and say
   whether the mean wait grows like the variance or faster.
5. *(Concept)* Write the two-sentence memo that the manager of the centre should receive, using natural
   frequencies and without citing any mean without the rest of the distribution.
6. *(Laboratory)* In the laboratory, reinforcement only on surge days doubles the capacity of the busy day
   and brings the mean wait down from 19.6 to 2.5 minutes. Measure that world with reinforcement of 1.5 times
   and 3 times the capacity and say, in natural frequencies, how many patients in each 100 still wait more
   than 30 minutes.

**Summary.**

- The mean of the input is not the mean of the output when the function is curved. The result is
  $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$ for convex $f$.
- The toll grows with the curvature and with the variance, and the second-order formula **underestimates** the
  toll when the dispersion is large.
- Mean demand does not move, and the waits do not look alike. Whoever simulates the world sees what the
  spreadsheet hides.
- The margin is only worth what the independent unit is worth, and the halves null decides which unit is the
  right one.
- A threshold (failing, overflowing, going bankrupt) is not summarized by any mean.

## Chapter 6. The invisible filter: why do 2.4 million votes count as 7 people?

**Objectives.** By the end of this chapter you should know how to separate the three factors of the error of a
sample (quality, quantity and difficulty); name the mechanism that produces a missing datum; compute the
effective size of a sample in the unit that is in fact independent; and recognise the selection filter that
manufactures relations that do not exist in the world.

---

### Step 1 — The real world and the paradox

**The magazine that heard 2.4 million people and got the president wrong.** In 1936 the American magazine
*Literary Digest* ran the largest election poll in history up to then. It received **2.38 million** ballots
from readers and announced, with fanfare, that Alfred Landon would beat Franklin Roosevelt with
**57%** of the vote. Roosevelt won with **60.8%**. In that election George Gallup got the winner right
with about **50 thousand** interviews.

In every 100 ballots that reached the newsroom, 57 said Landon. In every 100 real voters, 61
said Roosevelt. The magazine did not get the arithmetic wrong, it got the **door** wrong. The ballots did not
come from a draw of the electorate, they came from those who had a telephone, from those who were subscribers, from those
willing to fill in and return a form. Each of those stages is a screen, with a different effect
for Roosevelt and for Landon voters.

**In every 100 people.** The count is worth seeing in natural frequencies. Imagine 100 voters. If the
chance of answering the poll were the same for all of them, the 100 would answer in the proportion of each
one's vote. That is not what happened. Running the world of 1936 with 10 million voters and a chance of
answering that depends on the vote, this book measures as follows:

- of Roosevelt's supporters, **16.8 in every 100** answered;
- of Landon's supporters, **35.2 in every 100** answered;
- taken together, 24 in every 100 answered — 2.4 million ballots.

The result is simple arithmetic, 0.608 × 0.168 = 10.2% for Roosevelt against 0.392 × 0.352 = 13.8% for
Landon among the respondents. Roosevelt gets 10.2 out of 24, that is, **42.6%**, and Landon 57.4%.
The poll received millions of votes and measured the magazine's audience; it did not measure the country.

**The question that gives the chapter its name.** How many people *drawn in the street* would be equivalent to those
2.4 million ballots? The measured answer is **7**, seven people. The deviation of a random sample of 7
people in a world with 60.8% for Roosevelt is 18.4 points, practically the size of the poll's
error.

More biased data do not give a smaller error. They give more certainty about the wrong answer. The margin of error that
the magazine could compute was 0.03 point, and the error was 18.2 points, or **661 margins**.

**The independent unit.** This poison shows up with no ill will at all, in "internal" data. A call
centre logs 2.65 million calls in 81 working days. An analyst treats each call as an
observation and computes the margin. The calls of one day, however, share the demand, the staffing
roster and the weather. They repeat, they do not add up. The independent unit there is the **day**, and 2.65
million calls are worth **81** observations. That is why a report that treats the calls
as independent promises a precision that the real unit does not sustain.

---

### Step 2 — The rule of the game

**Meng's identity.** Meng (2018) showed that the error of any sample mean decomposes into
three factors, whose product is exact:

$$\bar Y_n - \bar Y_N \;=\; \underbrace{\rho_{R,Y}}_{\text{qualidade}}\times
\underbrace{\sqrt{\tfrac{1-f}{f}}}_{\text{quantidade}}\times \underbrace{\sigma_Y}_{\text{dificuldade}}$$

| Factor | What it is | What it does |
| --- | --- | --- |
| Quality $\rho_{R,Y}$ | Correlation between **entering the sample** and **the answer** | It is the poison: it does not shrink with $n$ |
| Quantity $\sqrt{(1-f)/f}$ | $f$ is the fraction of the population sampled | It shrinks with more data, but slowly |
| Difficulty $\sigma_Y$ | How much the answer varies in the population | It is the physics of the problem |

In a **random** sample, $\rho$ is of the order of $1/\sqrt N$ — tiny — and the identity returns the
usual standard error. When entering the sample depends on what is being measured, $\rho$ does **not** shrink with
$n$. The first factor stands still while the second falls. With $\rho = -0.21$, $f = 0.24$ and
$\sigma_Y = 0.488$, the identity predicts an error of $-0.1824$, or 18.2 points, the value that the simulation
measures. More biased data fix nothing; they **narrow the interval around the wrong answer**.

**How to handle a missing datum.** Rubin (1976) classified missingness
by the mechanism that produces it, and the class decides the legitimate treatment.

| Mechanism | The chance of missing depends on | Example | What can be done |
| --- | --- | --- | --- |
| MCAR, completely at random | Nothing that matters | A form lost in the mail | Analysing the complete cases loses precision — no bias enters |
| MAR, at random given the observed | Variables that were measured | Young people answer less, and age is in the base | Weight or impute conditioning on the observed |
| MNAR, not at random | The very value that is missing | Those who earn more report less income | No correction comes out of the data alone: it requires a model of the missingness and a sensitivity analysis |

The distinction between MAR and MNAR **cannot be tested** with the observed data. It is an assumption,
and it has to be declared as such. In Meng's identity, MNAR is the case in which $\rho$ remains
different from zero after conditioning on what was observed. The sentinel value, the missing value recorded as
0 or 9999, is worse than the other three, because nobody knows that the datum is missing.

**The filter manufactures relations.** Berkson (1946) showed that two independent diseases appear
associated among inpatients, because each one raises the chance of admission. Looking only at the selected is
conditioning on a **collider**. In the simulation of this chapter, talent and likeability have correlation $+0.001$ in the
population and $-0.679$ among the selected. The relation was not hidden in the data; the filter
**created** it.

**The table that reverses.** Simpson showed the mirrored case. Charig and colleagues (1986) compared two
kidney-stone surgeries.

| Group | Open surgery | Other technique |
| --- | --- | --- |
| Small stones | 93% | 87% |
| Large stones | 73% | 69% |
| Raw total | 78% | 83% |
| Weighted by the whole population | 83.3% | 77.9% |

Open surgery wins in both strata and loses in the total, because the hard cases went to it.
Conditioning on a confounder closes a spurious path; conditioning on a collider opens one. "Controlling for
everything" is not neutral, and the causal graph decides which count to make, not the table (Chapter 9).

**The independent unit, formalised.** The effective size of a sample is a property of the
**process**, not of the count. In 1913 Markov counted vowels and consonants in the first 20 000 letters
of *Eugene Onegin* and showed that the law of large numbers survives dependence, but that
dependence changes what the sample is worth. Since letters tend to alternate, a block of 100 letters is worth
about 330 independent letters; in a "sticky" chain, with each state tending to repeat, 100
observations are worth much less than 100.

$$n_{\text{ef}} \;=\; \frac{p(1-p)}{\operatorname{Var}(\bar X)}$$

In words, compare the spread that the sample mean **actually has** with the one it would have if the
observations were independent. In real data, 225 thousand New York flights are worth, in independent
information, 364 days; 2.65 million calls are worth 81 days. Treating the
patients of one day, or the calls of one hour, as independent observations is
**pseudoreplication**: the margin comes out too small and the test approves what it should not.

**The call-centre case, in full.** The sector claimed the following. Staffing each half hour by
Erlang C on the historical mean rate would guarantee 82% of calls answered within 20 seconds, every
day. In the 81 days of the vault, the standard rule fails on **26 out of every 100 days** and delivers 86%. The variation
between days accounts for most of it, since even the world that keeps only the real total of each day, with the
mean profile, already loses 17% of the days. Erlang C, applied to the real rate of each day, gives **57%**, instead of the claimed 82%, and 65% of the days fall below the target; it is Jensen inside the formula (Chapter 5), without
needing simulation. The unit comes out of the halves null. The deviations are 0.0474, against 0.0470 predicted
by the day and 0.0033 predicted by the call. The per-call reading promises a margin fourteen times smaller.
In 400 days simulated in the world of the claim, none fell below the target; in the real one, about one in four.
The decision that comes out of this costs **+7.2% of agent-hours**, 2,428 per day against 2,264, and takes the failure
from 26 down to **less than 4 in every 100 days**. The sensitivity grid, declared before running in
`h0-real/DECLARACAO_PACIENCIA.md`, sweeps service from 3 to 6 minutes against patience from 1 to 10
minutes, 24 points. The winning rule is right in 20 of them; in the four remaining, all with 1 minute of
patience, the four rules tie because none leaves more than 1 day below the target. There is no turning
point. In the worst corner of the grid, not even the best rule holds the target. It is 14.2% of the days below it, against
37.7% for the rule of the claim.

**How this is checked.** None of this is a matter of opinion about who is right. The traces of the
poll (logs, hypergraphs, results) stay outside Git for size, with the SHA-256 sums
published in `FORA_DO_GIT.md` and in each front's `SHA256SUMS`; whoever wants to redo the count downloads the
raw data, regenerates the trace and checks it byte by byte. The filter is invisible in the report and in the file.

---

### Step 3 — Hands on

The laboratory rebuilds the 1936 poll, measures the correlation that the filter creates, redoes the
Simpson table and measures the effective size of a chain with memory. Code in `crates/stochology-book/src/ch06.rs`, output
measured with `cargo run --release`:

```text
== Chapter 6 :: the invisible filter ==
Literary Digest: 2400031 of 10000000 voters answered (f = 0.240); Roosevelt among them = 0.4259, Landon = 0.5741
measured error = -0.1821 (-18.2 points); Meng identity with rho = -0.21: -0.1824
the error is worth the standard error of a random sample of 7.2 people; the poll's margin was 0.0276 points, and the error was 18.2 points = 661 margins
deviation of 2000 random samples of 7 people = 0.1836 (18.4 points); of 50 000 interviews = 0.0022 (0.22 points)
Berkson with cutoff 1.0: correlation in the population = +0.0012, among the selected (24.0% of the population) = -0.6168
Berkson with cutoff 1.5: correlation in the population = +0.0012, among the selected (14.4% of the population) = -0.6790
Berkson with cutoff 2.0: correlation in the population = +0.0012, among the selected (7.9% of the population) = -0.7315
Simpson: small 0.931 x 0.867; large 0.730 x 0.688; raw 0.780 x 0.826 (reverses)
weighted by a half-and-half population: 0.831 x 0.777 (the open surgery wins again)
two-state chain with persistence 0.500: variance of the mean of 100 letters = 0.002487 (independent: 0.002500), effective size = 101
two-state chain with persistence 0.800: variance of the mean of 100 letters = 0.009899 (independent: 0.002500), effective size = 25
two-state chain with persistence 0.235: variance of the mean of 100 letters = 0.000765 (independent: 0.002500), effective size = 327
```

Four readings. **(1)** The measured error (−18.2 points) coincides with Meng's identity (−18.24) and is worth
the standard deviation of a sample of **7.2 people**. The simulation is not a story, it is a closed
count. **(2)** The margin of error of the poll (0.0276 point) is no margin at all against a poison of
18 points; they are 661 margins apart. **(3)** The correlation between talent and likeability goes from $+0.001$
in the population to **−0.679** among those selected with cutoff 1.5. The filter **manufactures** the negative
association, and it gets stronger the narrower the door (−0.617 with 24% selected, −0.732 with
7.9%). **(4)** The effective size follows the process. There are 100 letters that are worth 101 observations when there is no
memory, 25 when the chain is sticky (persistence 0.8) and 327 when the letters alternate (persistence
0.235), the number Markov found in *Onegin*.

**Divergence declaration.** The previous edition of this book records, for the same Berkson simulation,
a correlation of $+0.002$ in the population and $−0.669$ among the selected; this instrument measures
$+0.0012$ and $−0.679$ with cutoff 1.5 (and $−0.617$ with cutoff 1.0). The difference is one hundredth and comes
from the selection cutoff, which the old text does not declare. The 1936 numbers converge without adjustment. They are 2.4
million ballots, 57.4% for Landon against the published 57%, and the equivalence of 7 people. No old
value was deleted.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarse. The line of this chapter, by the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* A company says: "we have millions of users, so our satisfaction survey is
   precise". Which factor of Meng's identity did it forget, and what does it have to do with the *Literary
   Digest*?
2. *(Count)* In the Simpson table, recompute the raw rate of each surgery as the mean of the strata
   weighted by the number of cases (81, 234, 192 and 55) and explain why it reverses the strata. Which is
   the confounding variable?
3. *(Count)* With $\rho = -0.21$ and $\sigma_Y = 0.488$, which $f$ makes Meng's error worth 10 points?
   And 5 points? Say in one sentence why the 1936 poll could not get there even with 50
   million ballots.
4. *(Laboratory)* In the laboratory, change 0.168 and 0.3517 to a pair in which the chance of answering is
   the same in both groups and confirm that the error disappears. Then choose a pair with correlation
   $−0.05$ and say how many people the whole poll would be worth.
5. *(Laboratory)* Run the two-state chain with persistence 0.9 and with 0.1 and measure the effective
   size in both cases. Explain, in one sentence for each case, what this does to the margin of a
   report that treats the 100 letters as independent.
6. *(Concept)* Give an example from your field in which the missing datum is MNAR and one in which it is MAR, and
   say what changes in the conclusion of each.

**Summary.**

- The error of a mean is quality × quantity × difficulty, and size does not make up for selection.
- MCAR loses precision, MAR is corrected by conditioning on the observed, MNAR is corrected only with a model of the
  missingness declared as an assumption.
- The filter does not only distort the mean: it creates relations that do not exist in the population.
- The effective size of a sample is a property of the process; the margin is always computed in the independent
  unit.
- 2.4 million biased ballots are worth 7 people, and the margin of error of a biased poll measures
  nothing at all.

## Chapter 7. Hunting ghosts in the noise: severe tests and false discoveries

**Objectives.** By the end of this chapter you should know how to build the exact null model of an experiment and
compare against it; recognise the researcher freedoms that inflate false positives; compute the predictive
value of a finding with Ioannidis' formula; name the five types of error; and distinguish a shortcut that is an error from a shortcut that is an adaptation to the environment.

---

### Step 1 — The real world and the paradox

**One hundred people tossing coins.** Gather 100 people. Each one tosses an honest coin 20 times and writes down the
longest run of heads obtained. Nobody cheats. Even so, in **100 out of every 100 experiments** that this book
simulates, someone strings 6 heads in a row or more, and the longest run is, on average, **9.5 heads**. If the reporter chooses to tell the story of the person with the longest run,
tomorrow's paper carries a talent that does not exist. There was no measurement error, only a forgotten
**null model**.

**The hot hand.** The most famous case is the "hot hand" in basketball. Gilovich, Vallone and Tversky (1985)
analysed the shots of professional players and treated the belief that whoever hits tends to
hit again as a pattern illusion. The proportion of hits after a hit was equal to the general
proportion. For three decades the result was cited as the canonical example of illusion.

In 2018, Miller and Sanjurjo showed that the **test** was biased. The original count pooled all
shots and compared the proportion of "hit after hit" with one half. But that proportion, in a
process with no memory at all, **is not one half**. In three tosses of an honest coin, the mean of the
proportions per sequence is $5/12 = 0.4167$. The target was in the wrong place. With the count corrected, the
difference in the Cornell data goes from $+3$ to $+13$ percentage points. The ghost was not in the
players. It was in the denominator.

**The garden of forking paths.** Simmons, Nelson and Simonsohn (2011) asked
researchers to declare the plan before collecting the data. Four common decisions are:
*when to stop collecting*, *which variables to use*, *which cases to exclude* and *which comparisons
to report*. Each one looks reasonable on its own. Together, in an experiment in which **there is no effect
at all**, they take the false-positive rate from 5% to **60.7%**. In every 100 studies done
this way, 61 show a "significant effect" that does not exist.

**Fifty cases, fifty predictors, nothing.** An analyst has 50 cases and 50 measured variables, with
no relation to the response. They fit 500 different models and keep the one with the smallest validation
error. The winner looks better than "predicting zero" in **73% of the repetitions**, and in the face of fresh
data the advantage vanishes, because there is no wrong count and no invented datum: only 500 shots at a target that
does not exist, and the best shot chosen after seeing where it landed.

---

### Step 2 — The rule of the game

**The exact null model.** The first step of any analysis is not to look at the data, but to write, in
code, the world in which **nothing happens**, and to measure the results it produces. The null model must
reproduce the structure of the data, with the number of cases, the number of comparisons and the freedom of
choice of the experiment, because the right target comes out of it.

The hot hand is the cleanest example. In three tosses of an honest coin, the eight possible sequences
are equally likely. The estimator the literature used is the **mean of the proportions per sequence**,
and the sequences with no hit at all have no defined proportion and drop out of the count. Six sequences
remain, with proportions 1, 1/2, 0, 0, 1 and 0. The mean is $2.5/6 = 5/12$. The **pooled** estimator instead,
total of hits after hits over total transitions after a hit, gives $4/8 = 1/2$. The two
estimators answer different questions, and comparing the first with 1/2 is comparing against the wrong
target. The error is 0.0833, eight percentage points of invented "hot hand".

**Mayo's severity.** Deborah Mayo proposed the question that organises everything: *a result is only
evidence if the procedure would probably have gone wrong had the claim been false.* For a pre-registered
test that produces $p = 0.04$, the severity of the result is $1 - p = 0.96$. If the hypothesis
were false, the test would have produced a less extreme result in 96 out of 100 times. When the
"$p = 0.04$" comes out of a garden of paths, the severity that matters is no longer $1 - p$, but $1$
minus the false-positive rate **of the procedure**, because what produced the number was the whole
procedure.

**Ioannidis' theorem.** The predictive value of a positive finding depends on the power $1-\beta$, on the
odds $R$ that a hypothesis in the field is true and on the significance level $\alpha$:

$$\text{VPP}=\frac{(1-\beta)R}{R-\beta R+\alpha}$$

In words: even a well-done study can have little chance of being right, if the field tests many
false hypotheses. With power 0.80 and $\alpha = 0.05$, if one in every ten hypotheses is true
($R = 1/10$), the PPV is 0.615; if one in every thousand ($R = 1/1000$), it falls to 0.016. If $\alpha$ is the garden's
instead of 0.05, both values collapse. The Open Science Collaboration (2015) replicated 100
psychology studies, and **97% of the originals** had $p < 0.05$, against **36% of the replications**.

**A fit is only evidence if the model could have failed.** In a two-dimensional space, a
half-plane separates 50 points at random with probability $2 \times 10^{-12}$, about **39 bits** of
evidence. A table that memorises every label also separates the 50 points, and brings not a single bit. It
would separate **any** set of labels, with or without signal. The separation is identical, and the evidence is
zero. The test has to be able to fail for success to mean anything.

**The five errors.** Classical statistics names two, and practice added three more.

| Error | What happens | Source |
| --- | --- | --- |
| Type I | Rejecting a true null hypothesis: the false alarm | Neyman and Pearson |
| Type II | Not rejecting a false null: the lost effect | Neyman and Pearson |
| Type III | Giving the right answer to the wrong problem | Kimball (1957) |
| Type S | Estimating an effect with the sign flipped | Gelman and Tuerlinckx (2000); Gelman and Carlin (2014) |
| Type M | Exaggerating the magnitude of a true effect | Gelman and Carlin (2014) |

Types S and M explain why small and "significant" studies disappoint in replication. With
little power, only the estimates that chance inflated pass the $p < 0.05$ filter, and some have even the
wrong sign. Type III is the most stochological of all, because it is not a counting error but a
misalignment between the Agent's question and the question the Model answers. Chapter 5 shows the
example of the manager who asked about the mean wait and received the answer of a world that does not exist.

**Shortcut against environment.** Kahneman and Tversky catalogued how people depart from the rules of
probability; Simon and Gigerenzer answered that a shortcut is an error only **relative to an environment**. The
Stochology proposes a criterion instead of choosing a side. A departure is an **error** when the right
information was available and the combination rule was violated; it is a **candidate adaptation** when the norm
assumes information that the environment does not supply reliably.

The two numbers that separate the cases. In a screening with prevalence 1%, whoever ignores the base rate
has a Brier score of **0.083**, against **0.009** for whoever uses it. In a referral clinic, where the
real prevalence is 33%, whoever insists on 1% has **0.28**, against **0.084** for whoever ignores it. A single
heuristic is an error in one environment and a success in the other. The Brier score is the subject of Chapter 8.

**The adversarial board.** In a research architecture, the defence against the garden is a formal attack, not
a good intention. Eight agents try to break the artefact before approval, each one in a band:

| Band attacked | The attack |
| --- | --- |
| Code | A new battery of properties, with a seed the author did not choose |
| Data | A batch with corrupted records, which has to be isolated |
| Model in operation | A batch with strong drift, live |
| Inference | A pathological data set, to see whether the uncertainty denounces the inadequacy |
| Governance | Tampering with the audit trail, which the chain verifier has to detect |
| Fairness | Inverting the chosen metric and showing who would start paying |
| Domain | An atypical scenario of the domain, with an alternative prior, to see whether the decision holds |
| Decision | A crisis that violates the exit clause itself |

Approval requires that **all** eight reach the minimum severity:

$$\text{Aprovado}(X) = \bigwedge_{i=1}^{8}\Big(A_i(X) \ge \text{Severity}_{\min}\Big)$$

with $\text{Severity}_{\min} = 6$ on a scale from 0 to 10, and the sheet repeated with the fixed seed always
returns the same severity. The rule is hard on purpose. The board's average grade approves nothing, and one weak
band declared is worth more than uniform work without an admitted gap. No agent approves the
work it produced. Three failures eliminate the work before any discussion of merit: temporal leakage, a data
contract violated without treatment, and the absence of a named human owner.

---

### Step 3 — Hands on

The laboratory measures four things: the garden of models on pure noise, the exact null model of the hot hand,
the garden of paths and the severity of the board. Code in `crates/stochology-book/src/ch07.rs`; output measured with
`cargo run --release`:

```text
== Chapter 7 :: hunting ghosts in the noise ==
model garden: 200 replicas, 500 candidates over 50 pure-noise predictors, 25 training cases
the validation winner beats predicting zero in 79.5% of the replicas; on fresh data, in 30.0%
mean squared error of the winner: 0.814 on validation (predicting zero: 1.000) and 1.191 on fresh data
hot hand: mean of the proportions per sequence = 0.4167 (5/12 = 0.4167); pooled proportion = 0.5000
100 people tossing 20 coins: in 100.0% of the experiments someone strings 6 heads in a row; the longest run averages 9.5
garden of paths (4000 experiments with no effect at all): with Simmons' four freedoms, someone finds p < 0.05 in 21.5% of them; with the large family, in 47.0%; the pre-registered test flags 5.2%
Mayo severity: 1 - p = 0.96 in the pre-registered test with p = 0.04; 1 - false-positive rate = 0.53 in the garden
Ioannidis with R = 1/1000, power 0.80: PPV = 0.016 with alpha 0.05 and 0.002 with the garden alpha
Ioannidis with R = 1/10, power 0.80: PPV = 0.615 with alpha 0.05 and 0.146 with the garden alpha
Ioannidis with R = 1/3, power 0.80: PPV = 0.842 with alpha 0.05 and 0.362 with the garden alpha
board A1-A8 with seed 2026, card with the eight safeguards: severities [9, 8, 10, 9, 8, 9, 8, 10], floor 6, approved = true
the same card without one band's safeguard: the lowest severity drops to 2 (rejected)
```

Five readings. **(1)** The winner of the garden of models has error 0.814 on validation, against 1.000 for
"predicting zero". It looks great, and the error rises to **1.191 on fresh data**. It is worse than zero, and that is why
it "beats zero" only 30% of the time when the judge is new. Validation was used as a target, and the target turned into
poison. **(2)** The exact null model of the hot hand is 0.4167, not 0.5; the mean of the proportions per
sequence coincides with 5/12 to four decimal places, while the pooled estimator sits at 0.5000.
The ghost is the difference between the two. **(3)** The four classical freedoms take the false
positive from 5.2% (pre-registered test) to **21.5%**, and the large family, with optional stopping at
four points, three sets of covariates, exclusion of the extreme case and two subgroups, takes it to
**47.0%**. **(4)** The Mayo severity of the pre-registered "$p = 0.04$" is 0.96; that of the number that comes
out of the garden is 0.53, because what produces the number is the procedure. **(5)** The A1–A8 board with
seed 2026 gives severities 9, 8, 10, 9, 8, 9, 8 and 10 on a sheet with the eight safeguards, approved, and
one band without its safeguard is enough for the lowest severity to fall to 2 and the approval to vanish. The AND of the eight does not
forgive.

**Divergence declaration.** The previous edition of this book records 73% of winners that "look
better than predicting zero", the 60.7% rate of the Simmons garden and a severity of 0.42 for the
$p = 0.04$ of the garden. This instrument measures **79.5%**, **21.5%** with the four freedoms (and **47.0%**
with the large family) and **0.53**. The numbers differ because the full protocol was not published: the
old garden swept more combinations — two outcomes, two sexes, more covariates, more optional stopping
points —, and every extra combination pushes the false positive up; in the garden of models, the
size of the validation and the range of predictors per candidate change the result, and here the winner still
fails on fresh data (error 1.191 against 1.000). What does not change is the direction: the pre-registered
test flags 5.2%, close to the nominal 5%, and the garden multiplies that by four or by nine. The
two values are recorded; none was deleted.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarse. The line of this chapter, by the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Explain, without any count, why the mean of the proportions per sequence ($5/12$) is smaller than
   the pooled proportion ($1/2$). Which sequences enter one count and not the other?
2. *(Count)* With $R = 1/20$, power 0.80 and $\alpha = 0.05$, compute the PPV. Redo it with the power falling
   to 0.40 and say, in one sentence, how the value of a positive finding changes when the study is
   small.
3. *(Laboratory)* In the laboratory, reduce the candidates from 500 to 10 and then to 5000. Measure the
   fraction of winners that beat zero on validation and on fresh data. Draw the two points and say what the
   number of attempts does to the appearance of the winner.
4. *(Laboratory)* Add to the garden the freedom "use the mean or the median of the outcome, whichever gives the
   smallest p" and measure the new false-positive rate. What consequence does that have for declaring the freedoms
   before running?
5. *(Count)* On the adversarial board, the severity of each agent is drawn between 8 and 10 when the safeguard
   exists and between 2 and 4 when it does not. Compute the probability of approval of a sheet with the eight
   safeguards and of a sheet with a single band without its safeguard. Then explain why the design chose
   those two intervals, and not a draw from 0 to 10.
6. *(Concept)* Give an example from your field in which ignoring the base rate is an adaptation to the environment, with
   the two Brier numbers that support your answer.

**Summary.**

- Every pattern needs an exact null model, with the same structure of the data and of the procedure.
- The hot hand target is $5/12$, not $1/2$: the estimator, compared against the wrong target, invents the
  effect.
- Four researcher freedoms take the false positive from 5% to tens of percent with no effect
  in the world.
- Ioannidis: the predictive value of a finding falls with $\alpha$ and with the chances that the hypothesis is false.
- Five errors, not two: I (false alarm), II (blindness), III (right answer to the wrong question), S
  (flipped sign) and M (exaggeration). The defence is institutional: the A1–A8 board and Mayo's severity
  trade good intention for a test that could have failed.

## Chapter 8. The forecaster's thermometer: calibration, resolution and the Brier score

**Objectives.** By the end of this chapter you should know why not every accuracy metric is honest;
compute the Brier score and decompose it into calibration, resolution and uncertainty; explain why
calibration can be repaired afterwards and resolution cannot; and decide the calibration tolerance by the size of the
sample, and not by a round number.

---

### Step 1 — The real world and the paradox

**Two forecasters.** In a city it rains, on average, 20 days in every 100. The first forecaster,
called here the *lazy* one, says the same sentence every day: *"chance of rain: 20%"*. The second,
called the *expert*, looks at the sky and says: *"today, 70%"*.

Who is better?

Neither of the two ready-made answers does. The lazy one is, over the long run, **right on average**. In
every 100 days on which he announced 20%, it rained about 20 times. He is calibrated, and useless for carrying an
umbrella. In every 100 of his warnings, 80 dry days and 20 wet ones, exactly the rate of the place. The
expert **separates** the days. On a day on which he says 70%, it rains far more than on a day on which he
says 10%. On one side, a calibrated forecaster with no resolution; on the other, a calibrated one with resolution.

Measured in a world of 4 thousand days, the numbers are these:

| Forecaster | Brier score | Calibration error (REL) | Resolution (RES) |
| --- | --- | --- | --- |
| Lazy (0.20 always) | 0.1858 | 0.0021 | 0.0002 |
| Expert | 0.1569 | 0.0012 | 0.0267 |

The lazy one has almost no calibration error (0.0021) and no resolution at all (0.0002). He
gets the frequency right and distinguishes nothing. The expert has an even smaller calibration error and a
resolution **133 times larger**. That is why he is worth more, and that is what the accuracy rate would not see.

**The question the accuracy rate does not answer.** A third forecaster is evaluated by accuracy
rate. He says "it will rain" when the chance passes 50% and is right 80% of the time. That metric does not
distinguish the forecaster who believes in 51% from the one who believes in 99%. The action is the same for both, and the
grade is the same. Such a metric does not punish whoever lies about the declared belief, and therefore it is no good for
auditing anyone.

---

### Step 2 — The rule of the game

**Proper scoring rule.** A scoring rule is **proper** (or *strictly proper*) when
whoever really believes in $q$ minimises the expected penalty by declaring that probability. The best known
rule is the **Brier score**:

$$\text{BS} = \frac{1}{n}\sum_{i=1}^{n}(p_i - y_i)^2$$

with $p_i$ the declared probability and $y_i$ the outcome (1 if it happened, 0 if not). It is the mean of the squared
error of the probability, and it is that property that makes it a metric in which **lying about the
belief always makes the grade worse**. The accuracy rate is not proper.

**Passing on average is not passing.** The predictive check of Box, Gelman and Shalizi compares the real data with replicas of the fitted model. A Poisson fitted to the demand of the call centre passes on the mean and fails on the variance. The observed
variance/mean ratio is **273**, and the model underestimates by tens of times the chance of a peak day (**180 against 2,900** calls, `h0-real/README.md`). A model that gets the mean value right and the dispersion wrong gets the day that matters wrong.

**Murphy's decomposition.** Murphy (1973) split the Brier score into three parts:

$$\text{BS} = \underbrace{\text{REL}}_{\text{calibration error}} -
\underbrace{\text{RES}}_{\text{resolution}} + \underbrace{\text{UNC}}_{\text{uncertainty of events}}$$

- **REL** (calibration): how far the declared probabilities depart from the observed frequencies.
  Zero means "when I say 30%, it happens 30% of the time".
- **RES** (resolution): how far the observed frequencies depart from the base rate. Zero means "all
  my warnings are the same warning".
- **UNC** (uncertainty): $\bar y(1-\bar y)$, how unpredictable the outcome is for someone who knows only the
  base rate.

The identity closes when the forecasts are pooled into boxes (ten boxes, for instance) and the
Brier score is computed over the mean of each box. The **ECE** (*expected calibration error*) is the non-squared
relative of REL, the mean of the absolute distances between predicted and observed per box.

**The practical asymmetry: calibration is repaired afterwards, resolution is not.** **Platt recalibration**
adjusts two numbers, $a$ and $b$, and turns $p$ into $\sigma(a + b\,\text{logit}(p))$. It repairs
monotone distortions, a forecaster systematically too confident or systematically too loose, and that is
why it is cheap. What it does **not** do is create information. A noisy forecaster remains
noisy after recalibration, because the noise was not in the scale, it was in the signal.

Measured in the laboratory, an overconfident forecaster goes from Brier 0.1712 to **0.1559**, practically the
value of the honest expert (0.1560), and all of the difference was scale distortion. A noisy forecaster
goes from 0.2119 to **0.1755**, an improvement that still does not reach the expert. The recalibrated lazy
one gives **0.1840**, which is the UNC of the world. Recalibrating a constant forecaster returns only the
base rate.

**How much calibration can be charged.** The ECE tolerance is the practical point. The rule of the
$\mathrm{ECE} \le 0.03$ type looks austere, but it is not. The ECE of a **perfectly calibrated** forecaster
is not zero, but a random number whose scale moves with $1/\sqrt n$. The honest tolerance is the 95% quantile
of that null, computed with those forecasts at the $n$ of the vault, and it shrinks as $n$ grows. In a
vault of 1,488 cases the noise is 0.03; in one of 104,489, 0.0035. A fixed tolerance is, then, two
things. **Too tight** in the small vaults, where it fails an honest forecaster, and
**too loose** in the large ones, where it lets through perfectly detectable miscalibration.

H3 measured this in six binary vaults:

| vault | n | observed ECE | null q95 | fixed tolerance 0.03 |
| --- | --- | --- | --- | --- |
| NY flights 2013 | 84,301 | 0.0267 | 0.0046 | passes |
| cyclones | 5,766 | 0.0123 | 0.0087 | passes |
| Toronto prisons 1997–2002 | 1,488 | 0.0363 | 0.0320 | fails |
| Toronto prisons 2020–2021 | 16,933 | 0.0668 | 0.0115 | fails |
| NY flights 2023 | 104,489 | 0.0079 | 0.0035 | passes |
| Rhode Island 2013–2015 | 24,466 | 0.0048 | 0.0034 | passes |

Applied the tolerance that depends on $n$, **no** binary vault of H3 passes, not even the three that the
fixed rule approved. The Toronto 1997–2002 case is the mirror. There the ECE of 0.0363 was noise of the order
of the tolerance, because the null q95 is 0.0320, but the observed value passes it, and the failure holds.

**Resolution is the hard part.** The Rhode Island flow ended with ECE 0.0048 and skill
$+0.019$, that is, good calibration and low resolution. Prison at a traffic stop is a rare event and
not very predictable from the available variables, and the model makes up for that with Platt (intercept and
slope) and with the circuit breaker, which puts the reserve as the official forecast in 12% of the vault. A forecast
can be honest and uninformative at the same time, and the two things are reported **separately**, because
the first is repaired and the second is not.

**Putting calibration to the test requires a new vault.** The intercept correction of H3b ($\delta \leftarrow
\delta + 0.002(y - p')$) was designed after seeing the 2013 deviation. To be honest, it needed a
vault nobody had seen, the 435 thousand New York flights in 2023, in the quarter that had
failed. It passed, with CITL 0.0015 and ECE 0.008, but the model without the correction would also have passed, because
2023 did not repeat the 2013 deviation. It is declared that the correction improved the calibration and **was not
decisive** for the gate.

**Two tools that come for free.** **Conformal prediction** gives guaranteed coverage without assuming the shape
of the distribution, $P(Y \in C(X)) \ge 1-\alpha$, and it is the one H3 used in an adaptive version under drift.
One rule holds for both. Every forecast is scored by a proper rule **outside the fit**, and no
forecast declares 0% or 100%.

---

### Step 3 — Hands on

The laboratory scores four forecasters, recalibrates all of them with Platt and measures the ECE tolerance at three
vault sizes. The code is in `crates/stochology-book/src/ch08.rs`, and the output was measured with
`cargo run --release`:

```text
== Chapter 8 :: the forecaster's thermometer ==
4000 days, observed base rate = 0.243, mean chance declared by the expert = 0.252
      lazy (0.20 always): Brier 0.1858 | REL 0.0021 RES 0.0002 UNC 0.1840 | ECE 0.0430 | after Platt: Brier 0.1840, ECE 0.0130 (a = -1.14, b = 0.00)
                  expert: Brier 0.1569 | REL 0.0012 RES 0.0267 UNC 0.1840 | ECE 0.0281 | after Platt: Brier 0.1560, ECE 0.0115 (a = -0.22, b = 0.81)
           overconfident: Brier 0.1712 | REL 0.0150 RES 0.0275 UNC 0.1840 | ECE 0.1109 | after Platt: Brier 0.1559, ECE 0.0120 (a = -0.25, b = 0.37)
                   noisy: Brier 0.2119 | REL 0.0363 RES 0.0083 UNC 0.1840 | ECE 0.1449 | after Platt: Brier 0.1755, ECE 0.0166 (a = -0.82, b = 0.27)
Murphy identity on the expert: REL - RES + UNC = 0.1584339, Brier of the forecasts pooled into the boxes = 0.1584339, difference = 3.47e-15
the raw forecast has Brier 0.1569462; pooling into 10 boxes costs 1.49e-3, and the identity closes
 Toronto prisons 1997-2002: n =    1488 | null q50 0.0292 | null q95 0.0425 | null q99 0.0481 | mean 0.0296 | null ECE above 0.03 in 45.1% of the simulations
    Rhode Island 2013-2015: n =   24466 | null q50 0.0028 | null q95 0.0043 | null q99 0.0051 | mean 0.0029 | null ECE above 0.03 in 0.0% of the simulations
           NY flights 2023: n =  104489 | null q50 0.0025 | null q95 0.0037 | null q99 0.0042 | mean 0.0026 | null ECE above 0.03 in 0.0% of the simulations
n = 1 488, spread 0.4: null q50 0.0320, null q95 0.0456
n = 1 488, spread 1.0: null q50 0.0291, null q95 0.0419
n = 1 488, spread 2.0: null q50 0.0243, null q95 0.0349
```

Five readings. **(1)** Murphy's identity closes to $3.5\times10^{-15}$ when the forecasts are
pooled into the ten boxes; the raw forecast has Brier 0.1569, and the pooling costs 0.0015. **(2)** The
lazy one has REL 0.0021 and RES 0.0002. It is calibrated and remains useless, and recalibrating a
constant forecaster returns exactly the UNC (0.1840). **(3)** The overconfident one falls from 0.1712 to
0.1559, below the expert (0.1560), because it was only scale. The noisy one falls from 0.2119 to 0.1755 and
**does not reach** the expert, because it was signal, and lost signal does not come back. **(4)** The tolerance of
0.03 is failed by an honest forecaster in **45.1%** of the simulations when $n = 1,488$, and in **0.0%**
when $n$ is 24,466 or 104,489. In the large vaults it almost prohibits nothing. **(5)** The null depends on the
forecasts, not only on $n$. With $n = 1,488$, the q50 of the null ECE goes from 0.0320 with concentrated forecasts
to 0.0243 with spread forecasts.

**Divergence declaration.** The previous edition records, for an example of 4,000 events, an overconfident
forecaster that goes from Brier 0.1956 to 0.1851 with Platt, an honest expert at 0.1846 and a noisy forecaster
that goes from 0.2154 to 0.2089. This instrument measures **0.1712 → 0.1559**, **0.1560** and
**0.2119 → 0.1755**. The world is not the same — here the observed base rate is 0.243, against the 0.5 of the
old example —, and the order of magnitudes is the same: recalibration takes the confident one to the level of the
honest one and leaves the noisy one behind. The ECE tolerance also diverges in level: H3 publishes a null q95 of
0.0320 for $n = 1,488$, 0.0034 for 24,466 and 0.0035 for 104,489; here they come out **0.0425**, **0.0043** and
**0.0037**. The difference is the distribution of the forecasts, which the README does not publish — with concentrated
forecasts ($n = 1,488$, spread 0.4) the null q50 of this instrument is 0.0320, the number of the
report. What matters does not change: the ECE noise moves with $1/\sqrt n$, and that is why a fixed tolerance
is tight in a small vault and loose in a large one. No old value was deleted.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarse. The line of this chapter, by the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Why is the accuracy rate not a proper rule? Give an example of two forecasters
   with the same accuracy rate and very different Brier scores.
2. *(Count)* A forecaster declares 0.30 on 200 days and it rains on 48 of them. Compute REL and RES in that
   box, assuming base rate 0.24, and say what that forecaster should do with the scale.
3. *(Count)* The null ECE moves with $1/\sqrt n$. If the null q95 is 0.0320 with $n = 1,488$, what do you
   expect with $n = 148,800$? Compare with the measured q95 of 104,489 and say whether the growth matches the
   prediction.
4. *(Laboratory)* In the laboratory, replace Platt recalibration with a simple intercept shift
   ($\sigma(\text{logit}(p) + a)$, fitting only $a$) and redo the four lines. In which
   forecaster does the slope matter?
5. *(Laboratory)* Change the world seed from 2026 to 1909 and to 2027 and measure the expert's Brier
   in the three. How far apart do the three sit, and what does that say about comparing two
   forecasters in a world of 4 thousand days?
6. *(Concept)* Write, in four lines, the memorandum that explains to the board why the ECE of 0.0363
   in Toronto 1997–2002 cannot be read with the ruler of the ECE of 0.0079 of the 2023 flights.

**Summary.**

- Only a proper rule scores belief: the Brier score punishes whoever lies about what they believe.
- $\text{BS} = \text{REL} - \text{RES} + \text{UNC}$, and the identity closes exactly with the forecasts
  pooled into boxes.
- Calibrated and without resolution is the lazy one: right on average and useless on each day.
- Calibration is repaired afterwards (Platt); resolution is not: lost information does not come back.
- The calibration tolerance is computed from the sample size: 0.03 is too hard in a thousand five hundred
  cases and too loose in a hundred thousand.

# Part III. Cause, time and survival

## Chapter 9. The ladder of cause and effect: graphs, confounders and invariance (ICP)

**Objectives.** By the end of this chapter you should know how to place a question on one of the three steps of
Pearl, draw a simple causal graph, distinguish a confounder from a collider, decompose a
naive difference into effect plus selection bias, and apply the invariant causal prediction (ICP) to
choose between two variables that correlation treats as equivalent.

---

### Step 1 — The real world and the paradox

**Two surgeries and a table that contradicts itself.** In 1986, Charig and colleagues compared two
surgeries for kidney stones. Open surgery won in the two strata in which patients are usually separated, and lost in the total:

| Group | Open surgery | Other technique |
| --- | --- | --- |
| Small stones | 93% success (81 of 87) | 87% (234 of 270) |
| Large stones | 73% (192 of 263) | 69% (55 of 80) |
| Raw total | 78% (273 of 350) | 83% (289 of 350) |

The explanation is in the left column of the table. The hard cases went to open surgery.
**In every 100 patients operated on openly, 75 had large stones; in the other technique, 23.**
Comparing the totals is comparing two different groups of patients. If the two procedures
had served cases with the **same mix, 51% small and 49% large, the one of the whole
population**, the numbers would be 83.3% for the open surgery and 77.9% for the other. The advantage goes back to the
open surgery, now by a difference larger than that of any single stratum.

Nothing in that table is wrong. Between the second and the fourth row what changes is **the question**.
*Among those operated on, which technique did better?* and *for the next patient, which technique to choose?*
are two questions, and the answer to the first does not serve the second. That distinction has a name and a
ladder.

**The three steps.** In every 100 headlines with the word "cause", nine out of ten sit on the
first step. Pearl separated the steps by the question that is asked:

| Step | Question | Example |
| --- | --- | --- |
| 1. See | Who has more of what? | Neighbourhoods with more police have more crime |
| 2. Act | If I intervene, what changes? | If we double policing, does crime fall? |
| 3. Imagine | What would have happened? | Would this robbery have happened with a patrol car on the corner? |

The first step is the step of correlation; the second requires a **do(x)**, the intervention; the third requires
a parallel world. The data alone only support the first. The bridge to the other two is a causal graph,
**a human declaration about the world, and never a software output**.

**The alarm that looks like a cause.** In an intensive care unit, the team's dashboard lights up an
alarm when the estimated bacterial resistance passes a threshold. The alarm predicts the severe
outcome better than any other variable in the system. In every 100 patients the alarm flags,
many do badly, and the team starts treating the alarm as if it were the disease.

The catch is that the alarm is a **child** of the resistance, not its parent. What triggers it is the resistance, and it
is the resistance that causes the outcome. Moving the alarm, switching it off or calibrating it differently changes the
outcome of not a single patient, because touching the child is not touching the parent. The correlation between the
alarm and the outcome is very high; the causal effect of the alarm is zero. When the data come from three
different hospitals, what separates the two is **invariance**. The effect of resistance repeats
in every hospital, the alarm's does not.

---

### Step 2 — The rule of the game

**Graphs.** A causal graph has nodes (variables) and arrows (direct relations). $X \to Y$ says that $X$
is in the equation that generates $Y$. Three arrangements receive names worth memorising:

| Arrangement | Shape | What conditioning does | Name |
| --- | --- | --- | --- |
| Confounder | $X \leftarrow Z \to Y$ | Closes a false path | Adjusting helps |
| Mediator | $X \to Z \to Y$ | Erases the effect we want to measure | Adjusting erases |
| Collider | $X \to Z \leftarrow Y$ | Opens a false path | Adjusting makes it worse |

Berkson (1946) showed the third case with two independent diseases. If each one raises the
chance of admission, looking only at the admitted makes them look associated. In a simulation of the course,
talent and likeability have correlation $+0.002$ in the population and $-0.669$ among those above
a threshold. **"Controlling for everything that was measured" is not neutral**. Adjusting a mediator erases the
effect, adjusting a collider manufactures it and adjusting an instrument amplifies the bias that is left.

**The back door.** Pearl's criterion says what to adjust. A set $Z$ closes the back door when
it blocks all the paths that enter $X$ from behind, without containing any descendant
of $X$. When that holds, the observed distribution answers the intervention question:

$$P(y \mid \operatorname{do}(x)) = \sum_z P(y \mid x, z)\,P(z)$$

The canonical worked case is hormone therapy. In 1991, the cohort of the Nurses' Health Study, with
about 48 thousand women, reported a relative risk of **0.56** of serious coronary disease among
users of oestrogen. In 2002, the randomised trial of the Women's Health Initiative, with 16,608
women, found a hazard ratio of **1.29**. The users in the cohort had other habits,
other income and other access to doctors, and a back door remained open, until the draw closed it.

**Potential outcomes.** Each person has two possible outcomes, $Y(1)$ with the treatment and
$Y(0)$ without it, and only one is observed. The comparison between the treated and the untreated gives:

$$\mathbb{E}[Y\mid D{=}1]-\mathbb{E}[Y\mid D{=}0] = \underbrace{\mathbb{E}[Y(1)-Y(0)\mid D{=}1]}_{\text{effect on the treated}} + \underbrace{\mathbb{E}[Y(0)\mid D{=}1]-\mathbb{E}[Y(0)\mid D{=}0]}_{\text{selection bias}}$$

When the treatment is **randomised**, the second term is zero by construction, because whoever was drawn
for the treatment has the same expected $Y(0)$ as whoever was not. Chapter 17 finds this in the
LaLonde data, where selection bias flips the sign of the effect. The raw comparison gives $-8,498$
dollars of income in 1978 and the experiment gives $+1,794$; the difference, $-10,292$, is the middle term.

**The naive difference is not only effect.** The decomposition gives a name to the third element, and it has no
statistical repair. Nothing in the observed data says how much it is worth, because one of the two
potential outcomes of each person is missing by construction. Only the draw zeroes that term.

**Invariance: what survives the change of environment.** A causal relation survives the change of
environment; a correlation of convenience does not. The Invariant Causal Prediction, by Peters,
Bühlmann and Meinshausen (2016), turns that into a test. If $S$ is the set of direct causes of
$Y$, $P(Y \mid X_S, E = e)$ is the identical distribution in every environment $e$ that intervenes on the
other variables without interfering in the equation of $Y$.

The procedure has three steps. (1) For each candidate subset $S$, test whether the relation of
$Y$ with $X_S$ is invariant across the environments; (2) keep every $S$ that passes; (3) return the
intersection of them. **The guarantee is asymmetric**. $\hat S$ contains only direct causes, with
probability of at least $1-\alpha$, but nothing guarantees that it contains all of them. Abstaining,
returning the empty set, is a result, not a failure. Abstention happens in two ways. The test
rejects every set, or accepts so many that the intersection comes out empty.

The test treats each row as an independent unit. When one volunteer gives 16 measurements, or one
school gives 403 pupils, the honest unit is the volunteer or the school, and the verdict changes from
field to field. In H2 this appeared in full:

| Reading | What happened |
| --- | --- |
| 6 of the 12 domains | ICP rejected every set: the environments act directly on $Y$ |
| 4 domains | it accepted so much that $\hat S$ came out empty: the environments move $X$ only a little |
| 1 domain | $\hat S$ not empty, and against the claim: citizenship and employment, not skin colour |

**A lesson that H2 added: the effect transports better than the set.** When ICP
rejects every set, the model fitted in the training environments predicted well an environment never
seen in 9 domains, with skill from 0.04 to 0.88. Finding the exact set of causes is more
demanding than predicting under change.

**Why this matters for deciding.** Invariance is also the answer to Lucas' critique
(Chapter 11). Only what is invariant across regimes deserves use when the regime changes. In H2g, with the
reference truth declared beforehand, ICP recovered the true parents of the electrical current
in a physical chamber, $\hat S = \{red, green, blue\}$, and left out light sensors strongly
correlated with them, because they were children of the LEDs.

---

### Step 3 — Hands on

Three tasks: redo Charig's count with the counts, classify questions into the three steps and
build a minimal ICP that chooses the cause and rejects the false alarm. Code verified in
`crates/stochology-book/src/ch09.rs`; output measured with `cargo run --release --bin labs -- --chapter 09`:

```text
== Chapter 9 :: Simpson's paradox in kidney-stone surgery ==
small: open 81/87 = 0.9310  new 234/270 = 0.8667
large: open 192/263 = 0.7300  new 55/80 = 0.6875
raw:   open 273/350 = 0.7800  new 289/350 = 0.8257
weight of the strata in the whole population: small 0.5100, large 0.4900
weighted by the whole population: open 0.8325  new 0.7789  -> the advantage flips back
fraction of hard cases in the open surgery = 0.7514, in the new one = 0.2286

== Chapter 9 :: the three steps of Pearl ==
  [  1. association] neighbourhoods with more police have more crime
  [ 2. intervention] if we double policing, does crime fall?
  [3. counterfactual] would this robbery have happened with a patrol car on the corner?

== Chapter 9 :: a minimal ICP, three sites, three candidates ==
rows = 12000 (4000 per site), X -> Y with coefficient 2, X -> Z (child) with noise different per site, W pure noise
naive correlation with Y: corr(X,Y) = 0.9314  corr(Z,Y) = 0.7208  corr(W,Y) = 0.0055
naive slope (all sites together): X = 1.9885  Z = 0.6003  W = 0.0152
  candidate {    x}  F =   0.755  p = 4.702e-1  common = 1.985  per site = [1.970, 1.996, 1.990]  ACCEPTED
  candidate {    z}  F = 976.809  p = 0.000e0  common = 0.477  per site = [0.928, 0.496, 0.299]  rejected
  candidate {    w}  F =   1.474  p = 2.290e-1  common = 0.011  per site = [0.056, -0.030, 0.008]  ACCEPTED
  candidate {  x+z}  F =   0.612  p = 6.541e-1  common = 1.991  per site = [1.918, 1.998, 1.999]  ACCEPTED
  candidate {  x+w}  F =   0.704  p = 5.890e-1  common = 1.985  per site = [1.970, 1.996, 1.990]  ACCEPTED
  candidate {  z+w}  F = 488.188  p = 0.000e0  common = 0.477  per site = [0.928, 0.496, 0.299]  rejected
  candidate {x+z+w}  F =   0.624  p = 7.111e-1  common = 1.991  per site = [1.919, 1.998, 1.999]  ACCEPTED
  candidate {empty}  F = 0.000  p = 1.0  (intercepts only)
accepted sets: 5 -> intersection S_hat = {}

== Chapter 9 :: 200 repetitions, stable mechanism: Y = 2X + noise at every site
mean naive slope (all sites together): X = 2.0004  Z = 0.6015
  {    x}  mean F =     1.04  rejection rate = 0.060
  {    z}  mean F =   973.87  rejection rate = 1.000
  {    w}  mean F =     1.14  rejection rate = 0.085
  {  x+z}  mean F =     0.99  rejection rate = 0.050
  {  x+w}  mean F =     1.04  rejection rate = 0.075
  {  z+w}  mean F =   487.45  rejection rate = 1.000
  {x+z+w}  mean F =     1.01  rejection rate = 0.055
  {x} accepted in 0.940 of the rounds
  intersection = {x} in 0.080 of the rounds
  intersection = {z} in 0.000 of the rounds
  intersection = {w} in 0.025 of the rounds
  intersection contains x and does not contain w in 0.080 of the rounds
  empty intersection in 0.895 (of which 0.025 because only the noise W was left, 0.005 because the cause itself was not left, and 0.890 because W entered some sets and not others)

== Chapter 9 :: 200 repetitions, unstable mechanism: the coefficient of X in Y changes from site to site
mean naive slope (all sites together): X = 4.0004  Z = 1.2029
  {    x}  mean F =     1.04  rejection rate = 0.060
  {    z}  mean F =  1352.54  rejection rate = 1.000
  {    w}  mean F =     1.10  rejection rate = 0.070
  {  x+z}  mean F =     0.99  rejection rate = 0.050
  {  x+w}  mean F =     1.04  rejection rate = 0.075
  {  z+w}  mean F =   676.76  rejection rate = 1.000
  {x+z+w}  mean F =     1.01  rejection rate = 0.055
  {x} accepted in 0.940 of the rounds
  intersection = {x} in 0.065 of the rounds
  intersection = {z} in 0.000 of the rounds
  intersection = {w} in 0.025 of the rounds
  intersection contains x and does not contain w in 0.065 of the rounds
  empty intersection in 0.910 (of which 0.025 because only the noise W was left, 0.005 because the cause itself was not left, and 0.905 because W entered some sets and not others)
```

**What each line says.**

**(1) Charig's count closes.** Open surgery wins on the small ones (0.9310 against 0.8667) and on the
large ones (0.7300 against 0.6875), and loses in the raw total (0.7800 against 0.8257). Weighting the two
strata by the whole population, with weights 0.5100 and 0.4900, the open surgery gets 0.8325 and the other
0.7789. The advantage flips back, now to the side of the open surgery, with the difference enlarged. What
produced the paradox was the mix of cases. **75% of the patients of the open surgery had large stones,
against 23% in the other technique**. The lesson of the 12 cases of Chapter 2 repeats here, now with numbers
from a real trial.

**(2) Naive correlation cannot separate the cause from the symptom.** With 12,000 rows across three
sites, the outcome $Y$ was generated by $Y = 2X + \text{noise}$, the symptom $Z$ by $Z = 2X +
\text{ruído do sítio}$ and $W$ is pure noise. Looking at everything together, $\operatorname{corr}(X,Y) =
0.9314$ and $\operatorname{corr}(Z,Y) = 0.7208$. The cause and the symptom look like two explanatory
variables, and the naive regression gives slope **1.9885** for $X$ and **0.6003** for $Z$. If the
analyst stopped here, they would attribute to the alarm an effect of 0.6.

**(3) ICP separates the two.** In the invariance test, fitting a common slope across the three
sites against free slopes:

| Candidate | F | p | Common slope | Per site | Verdict |
| --- | --- | --- | --- | --- | --- |
| $\{X\}$ | 0.755 | 0.470 | 1.985 | 1.970 · 1.996 · 1.990 | accepted |
| $\{Z\}$ | 976.809 | ~0 | 0.477 | 0.928 · 0.496 · 0.299 | rejected |

The slope of $X$ is practically constant across the three sites (1.970, 1.996, 1.990); that of $Z$ goes from
0.928 to 0.299, almost tripling from one site to the other. The mechanism that links the cause to the outcome
did not change; what links the symptom to the outcome changed in every place. The p of $Z$ is indistinguishable from
zero (F near a thousand) and that of $X$ is 0.470. In other words, the data have no way of distinguishing the
relation of $X$ from a relation that holds everywhere.

**(4) What the test gets right and where it abstains.** In 200 repetitions with the stable mechanism, the
set $\{X\}$ is accepted in **94.0%** of the rounds and $\{Z\}$ is rejected in **100%** of them. The
intersection of all accepted sets, however, returns $\{X\}$ only in **8.0%** of the cases, and comes out
empty in 89.5%. The set of the cause did not survive in 0.5% of the times, and in 2.5% only the noise
$W$ was left, because the test accepted the sets that contained it and refused the ones that contained $X$. In the other
86.5% the noise $W$ entered some sets and stayed out of others, and the intersection lost the common
denominator. **Abstaining is a result, not a defect of the instrument**, and the test never
returned $\hat S = \{Z\}$.

**(5) When the mechanism is unstable, the cause falls too.** In the second battery, the coefficient of
$X$ in $Y$ changes from site to site. The data now tell the truth about an incoherent world. The
naive slope of $X$ rises from 2.0004 to **4.0004**, because the estimator is mixing the true
effect with the variation across sites; $Z$ goes from 0.6015 to 1.2029, and $\{X\}$ stops
being a stable relation. When the sites intervene in the mechanism, no set is the right answer,
and the test rejects, as it should.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarse. The line of this chapter, by the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Classify on Pearl's step: "neighbourhoods with more police have more crime"; "if
   we double policing, does crime fall?"; "would this robbery have happened with a patrol car on the
   corner?".
2. *(Concept)* Draw a graph in which adjusting for $Z$ creates an association between $X$ and $Y$ that did not
   exist. What is the name of $Z$? Give an example from your area.
3. *(Count)* Recompute the raw rates of the Charig table as the mean of the strata weighted by the
   number of cases, and explain why it reverses the strata. Which is the confounding variable?
4. *(Count)* In the laboratory, the naive slope of $Z$ was 0.6003 and the slopes per site
   were 0.928, 0.496 and 0.299. Explain in one sentence why the naive mean is not the mean of those
   three, and what it reveals about reading the coefficient of a single regression.
5. *(Laboratory)* Make the noise of $Z$ equal across the three sites in the laboratory and show that
   $\{Z\}$ becomes accepted. What does that say about the strength of ICP? Does it find the cause or merely
   reject what varies?
6. *(Concept)* Why can ICP return the empty set when it has too much data? Relate it to
   the sensitivity of the test to departures from the model.

**Summary.**

- Association, intervention and counterfactual are different questions; the graph makes the bridge, and the data
  alone do not.
- Simpson's paradox is born of a mix of cases. Open surgery wins in every stratum and
  loses in the raw total because 75% of its patients were the hard cases.
- Confounder, mediator and collider call for three opposite reactions; adjusting everything is not neutral.
- Naive difference = effect on the treated + selection bias, and only the draw zeroes the second term.
- ICP separates the cause from the symptom through invariance across environments, and abstaining is a legitimate
  result. In the laboratory it never returned the child as the cause.

## Chapter 10. Time and ruin: the Peters coin, Kelly and wild tails

**Objectives.** By the end of this chapter you should know how to distinguish the ensemble mean from the time mean,
compute the volatility drag without advanced calculus, apply the Kelly criterion with Laplace
smoothing and recognise when the mean of a distribution does not exist or misleads.

---

### Step 1 — The real world and the paradox

**The coin that enriches the crowd and ruins the person.** A bet offers a 50% gain against a 40% loss,
with equal chances; heads multiplies the staked capital by 1.5 and tails by 0.6. Let us suppose that
someone stakes the whole wealth, round after round, starting with 1 unit.

- The count of one average round. Half the time it gains 0.5 and half the time it loses 0.4. On average, $+0.05$ per
  round, or five per cent per round.
- The count of one head **and** one tail: $1.5 \times 0.6 = 0.90$. Whoever lives through both rounds
  has lost 10%. One head and one tail do not return to the starting point, because 50% up and 40%
  down **are different numbers**.

In every 100 bettors who stake everything, at the end of **10 rounds** 37 finish above the start, at the end
of **100 rounds** 13, and at the end of **1,000 rounds** none. The mean wealth, that one, explodes. In a
simulation measured with 10,000 trajectories of 1,000 rounds, the final median of whoever stakes everything is
the probability that the cohort ends at that value is $1.322 \times 10^{-23}$; in the following modules we will see why the sample mean of that cohort
still grows.

The name of this is **non-ergodicity**, and it forces a simple question. *Whose mean is it?* The mean of 10,000 people at an instant is not the mean of one person over 10,000
rounds. Whoever lives one trajectory needs the second; whoever manages a fund with many holders
can live off the first, and the first is precisely the one that does not pay the bill of whoever bets.

**The same bet, two answers.** That bet is not bad in itself. It is good for the
crowd (the expectation grows 5% per round) and terrible for the typical trajectory (the median falls). What
separates the two readings is neither optimism nor pessimism. It is **in which unit one is counting**.

---

### Step 2 — The rule of the game

**Ensemble mean against time mean.** If the wealth of each round multiplies by $M_t$,

$$X_t = X_0 \prod_{s=1}^{t} M_s$$

the ensemble mean grows when $\mathbb{E}[M] > 1$, but along one trajectory the logarithm of the
product accumulates, and by the law of large numbers it goes to $\mathbb{E}[\ln M]$. When
$$\mathbb{E}[M] > 1 \quad \text{e} \quad \mathbb{E}[\ln M] < 0,$$
the crowd grows rich and almost every trajectory decays. In the Peters coin:

$$\ln \mathbb{E}[M] \approx +0.049, \qquad \mathbb{E}[\ln M] = \tfrac12\ln 1.5 + \tfrac12\ln 0.6 \approx -0.053$$

The instrument of the chapter measures $-0.052680$ per round, which gives $-5.268\%$ of time growth.
The difference between the two counts, $+5.00\%$ and $-5.27\%$, measures the distance between the line of a
spreadsheet and the life of whoever bets.

| Rounds | Trajectories above the starting point |
| --- | --- |
| 10 | 37.7% |
| 100 | 13.6% |
| 1,000 | 0.01% |

In 1,000 rounds the expectation is worth $1.55 \times 10^{21}$ and the median $10^{-23}$. The factors are
independent and identically distributed; **what is not ergodic is the product**. Even whoever simulates falls into the trap, because the
mean of $N$ trajectories follows the expectation only for a time that grows like $\ln N$, about 12
rounds with 10 trajectories and 112 with a million.

**How much to expose: Kelly's criterion (1956).** Instead of staking everything, one chooses the fraction $f$ of the
wealth:

$$f^* = \arg\max_f \; \mathbb{E}[\ln(1 + fR)]$$

In an even-odds bet with chance $p$, the optimal fraction is $f^* = 2p - 1$, that is, stake
20% with chance 0.6. In a bet that pays 5 to 4, like the Peters coin, $f^* = p|b| - (1-p)a =
0.9p - 0.4$, that is, 14% of the wealth. Staking everything would take the expectation, in 100 rounds, to
$8.3 \times 10^7$ times the capital, with a chance of surviving of $0.6^{100}$, about $10^{-22}$.
Twice the optimal fraction already impoverishes with that same edge, and the instrument measures $g(0.28)$ already below
zero and $g(0.70) = -0.0075$ per round.

**Epistemic uncertainty is an argument for staking less.** With the probability estimated over 25
rounds, full Kelly has a negative growth rate in 16% of the estimates, while half Kelly
guarantees 75% of the growth with a quarter of the variance.

**Laplace smoothing.** The simplest estimator of a probability is the observed frequency
$k/n$. It has a fatal defect in rare events. With zero losses in 250 weeks, it says that the
probability is **zero**, and no leverage is too much for a null chance. The Laplace succession rule
gives the corrected value.

$$P(\text{success tomorrow} \mid s, n) = \frac{s+1}{n+2}$$

With 250 clean weeks, the chance of a loss is not zero. It is $1/252 = 0.003968$. The rule is not neutral,
because it comes from a uniform prior, but it prevents the absurd conclusion that the risk is over.

**Insurance and cooperation.** In Bernoulli's ship, taken up by Peters and Adamou, a shipowner with wealth
of 100,000 pays 1,800 to cover a loss of 34,000 that occurs with chance 5%. In expectation, he
loses 100 per month and the insurer gains 100. In time growth, both come out ahead, and the shipowner goes
from 1.94% to 2.18% per month, and the insurer, which holds many independent trajectories, from zero to
+0.007%. In the Peters coin, three players who split the wealth every round grow 1.6% per
round, when each one alone would decay. **The question that decides the case is who holds the trajectories.**

**When the mean ceases to exist.** In a Pareto distribution, with $P(X > x) = x^{-\alpha}$, the
moment of order $k$ exists if and only if $k < \alpha$. With $\alpha = 1.1$, the mean exists and is worth
11, but the mean of a million observations has median 8.3, falls below 11 in 91% of the samples, and
the usual 95% interval contains the true value in only 26% of them. The fraction of the mean that comes from
values above $m$ is $m^{1-\alpha}$; in the median, 28% of the true mean comes from values larger than
the largest the sample of one million saw.

**The three states of chance (Mandelbrot).** Mandelbrot classified chance by the way sums behave.

| State | Example of distribution | What happens to the sum |
| --- | --- | --- |
| Mild | Gaussian, exponential | No observation dominates; the mean converges quickly |
| Slow | Lognormal | Every moment exists, but convergence misleads in real samples |
| Wild | Pareto with $\alpha \le 2$ | One observation can dominate the sum; the variance, or the mean, does not exist |

The name of this is **Fisher–Tippett–Gnedenko**. The maximum of many draws can only converge to three
families, Gumbel, Fréchet or Weibull, and that is why the tail is estimated from the maximum, not from the mean. In the course registry, the 1-in-100-year loss comes out at 17.4% by the GEV against 3.9% for the Gaussian: more than four times.

Taleb called the first Mediocristan and the last Extremistan, and added the type of
consequence. The "fourth quadrant", where the tail is wild and the loss depends on the size of the event, is
where models fail most and cost most. The Hill estimator estimates the tail exponent from the
$k$ largest values, $\hat\alpha = k / \sum \ln(x_i / x_{(k)})$. The Danish fires, with
$\alpha$ estimated at 1.81 (interval from 1.41 to 2.56), are in that territory (Chapter 18).

---

### Step 3 — Hands on

Four tasks make up the laboratory. Measure the Peters coin with the optimal fraction and with the all-in
stake; measure 10,000 insurance sellers with and without Laplace; estimate the tail with Hill; and measure the drag of
volatility at $1\times$, $2\times$ and $3\times$. Code verified in `crates/stochology-book/src/ch10.rs`; output measured with
`cargo run --release --bin labs -- --chapter 10`:

```text
== Chapter 10 :: the Peters coin (heads +50%, tails -40%) ==
ensemble mean per round = 1.0500  (+5.00% per round)
time mean per round      = -0.052680  (-5.268% per round)
product of one head and one tail: 1.5 x 0.6 = 0.90
full Kelly (5:4): f* = 0.1400 of the wealth; with an even-odds payoff, 2p - 1 = 0.20
time growth with full Kelly: +0.0175 per round

all-in (f = 1.00)          above the start: 10 rounds 0.374  100 rounds 0.128  1000 rounds 0.00000
                           ruin (< 1e-6 of the capital) = 0.9971  final median = 1.322e-23  final mean = 2.725e-6
full Kelly (f = 0.70)      above the start: 10 rounds 0.374  100 rounds 0.300  1000 rounds 0.07830
                           ruin (< 1e-6 of the capital) = 0.5089  final median = 6.810e-7  final mean = 4.982e4
half Kelly (f = 0.35)      above the start: 10 rounds 0.618  100 rounds 0.611  1000 rounds 0.85620
                           ruin (< 1e-6 of the capital) = 0.0000  final median = 1.854e2  final mean = 2.119e6
quarter Kelly (f = 0.14)   above the start: 10 rounds 0.618  100 rounds 0.811  1000 rounds 0.99560
                           ruin (< 1e-6 of the capital) = 0.0000  final median = 1.506e2  final mean = 1.016e3

all-in: log10 of the final median = -22.9, log10 of the final mean = -5.6
part of the mean that comes from the richest trajectory (richest 1%): 1.000

== Chapter 10 :: the insurance seller (earns 1% per week, loses 50% with q = 0.004) ==
time growth of the all-in: +0.00714 per week (negative: ruin is certain)
observed frequency + full Kelly  no loss = 0.368  all-in = 0.368  ruin in 1000 weeks = 0.0000  final median = 3.531e2  final mean = 1.255e3
Laplace + full Kelly             no loss = 0.368  all-in = 0.000  ruin in 1000 weeks = 0.0000  final median = 8.983e1  final mean = 2.529e2
Laplace + half Kelly             no loss = 0.368  all-in = 0.000  ruin in 1000 weeks = 0.0000  final median = 1.097e1  final mean = 1.371e1
theoretical probability of 250 weeks with no loss = 0.3671
Laplace with zero losses in 250 weeks: q_hat = 1/252 = 0.003968

== Chapter 10 :: the wild tail: Pareto with alpha = 1.1 ==
true mean (alpha/(alpha-1)) = 11.0
n =      1000: mean of the sample means =     7.76  median =    6.22  below 11 in 0.94  95% CI covers in 0.24
n =   1000000: mean of the sample means =     9.53  median =    8.39  below 11 in 0.95  95% CI covers in 0.26
fraction of the mean that comes from values above the largest seen: 0.000
in the median, 25.12% of the true mean comes from values above the maximum of a sample of one million

Hill estimator with alpha = 1.1 and n = 200 000:
  k =    100  alpha_hat = 1.083  95% CI approx [0.870; 1.295]
  k =    500  alpha_hat = 1.063  95% CI approx [0.970; 1.157]
  k =   1000  alpha_hat = 1.100  95% CI approx [1.032; 1.168]
  k =   5000  alpha_hat = 1.101  95% CI approx [1.070; 1.131]
  k =  20000  alpha_hat = 1.095  95% CI approx [1.080; 1.111]
gregarine: a Gaussian model fitted to this tail underestimates the loss in every repetition

== Chapter 10 :: volatility drag at 1x, 2x and 3x ==
L = 1x: arithmetic mean = +0.000317/session, measured growth = +0.000249/session, drag = +0.000068/session (+0.0172/year), wealth in 100000 sessions = 6.363e10
L = 2x: arithmetic mean = +0.000634/session, measured growth = +0.000267/session, drag = +0.000367/session (+0.0926/year), wealth in 100000 sessions = 3.805e11
L = 3x: arithmetic mean = +0.000951/session, measured growth = +0.000053/session, drag = +0.000898/session (+0.2264/year), wealth in 100000 sessions = 1.955e2
horizon of 2000 sessions:
  L = 1x: measured growth = +0.000196/session, final wealth = 1.480e0
  L = 2x: measured growth = +0.000162/session, final wealth = 1.384e0
  L = 3x: measured growth = -0.000101/session, final wealth = 8.168e-1
horizon of 10000 sessions:
  L = 1x: measured growth = +0.000352/session, final wealth = 3.385e1
  L = 2x: measured growth = +0.000472/session, final wealth = 1.126e2
  L = 3x: measured growth = +0.000360/session, final wealth = 3.649e1
the 1987 crash (-22.6% in one day) takes 67.8% of the capital at 3x, 22.6% at 1x; ruin starts at L = 4.4
with a -22.6% crash in one day and 40 days of -3% (20 thousand sessions):
  L = 1x: measured growth = +0.000147/session, final wealth = 1.890e1
  L = 2x: measured growth = +0.000058/session, final wealth = 3.167e0
  L = 3x: measured growth = -0.000274/session, final wealth = 4.188e-3
```

**What each line says.**

**(1) The two means.** The ensemble mean of the Peters coin is $1.0500$ per round, or $+5.00\%$.
The time mean is $-0.052680$, or $-5.268\%$. The product of one head and one tail is
$1.5 \times 0.6 = 0.90$. The Kelly fraction is $0.14$ of the wealth; with it, the growth in
time is $+0.0175$ per round, positive for the first time.

**(2) The cohort of 10,000 trajectories.** Staking everything, the wealth falls to $< 10^{-6}$ of the capital in
$99.71\%$ of the trajectories, the final median is $1.322 \times 10^{-23}$ and the $\log_{10}$ of the final
mean is $-5.6$. **The richest trajectory alone accounts for 100% of the cohort mean**, that is, the richest 1%
explains everything the mean has of wealth. With **full Kelly** $(f = 0.70)$ ruin appears in
half of the trajectories ($50.89\%$) and the final median collapses to $6.810 \times 10^{-7}$; with
**half Kelly** $(f = 0.35)$ ruin is zero and the final median is $1.854 \times 10^2$; with
**quarter Kelly** $(f = 0.14)$ ruin is zero and the final median is $1.506 \times 10^2$. The
contrast between $f = 0.70$ and $f = 0.35$ is the point of the chapter. **Twice the optimal fraction is no longer
optimal for anything**, because the bet loses on most trajectories, with the mean still exploding.
**(3) The insurance seller.** In 10,000 histories of 250 weeks, **36.8%** see no loss at all —
exactly the theoretical probability $0.996^{250} = 0.3671$, and those are the ones who stake everything ($f = 1$).
Whoever reads the probability from the raw frequency has a final median of $3.531 \times 10^2$ and mean of
$1.255 \times 10^3$; whoever uses Laplace's rule estimates $1/252 = 0.003968$ and works with
$f = 0.484$, with median $8.983 \times 10^1$. Both finish above the starting point, and
the difference does not show in the gain, but in the **tail**. Whoever ignored the risk, in a bad history, has
nowhere to come back from.

**(4) The wild tail.** With a Pareto of $\alpha = 1.1$, whose true mean is 11, the sample mean
of a million observations has median 8.39 and falls below 11 in 95% of the samples, and the 95%
interval covers the true value in 26% of them. The Hill estimator recovers the exponent. With $k = 1,000$
it gives $\hat\alpha = 1.100$, with approximate interval $[1.032; 1.168]$; with $k = 20,000$ it gives
$1.095$ with $[1.080; 1.111]$. A Gaussian model fitted to that tail underestimates the loss in
every repetition.

**(5) The volatility drag.** Calibrating the model by the H1 of the repository (drag of 0.14 per year
at $3\times$ and growth of 0.051 per year at $1\times$), in 100,000 sessions the measured growth
per session is $+0.000249$ at $1\times$, $+0.000267$ at $2\times$ and $+0.000053$ at $3\times$.
The measured drag rises from $0.0172$ per year at $1\times$ to $0.0926$ at $2\times$ and $0.2264$ at
$3\times$; the drag **grows with the square of the leverage**, which is the signature of
$\mu - \tfrac12 \sigma^2 L^2$. The 1987 crash, alone, takes **67.8%** of the capital at $3\times$ and
22.6% at $1\times$; ruin starts at $L = 4.4$. With a crash planted in the middle of 20,000 sessions,
the growth of $3\times$ sits at $-0.000274$ per session, below $1\times$ and $2\times$. **A single day changes the order**.

**Divergence declaration.** The previous edition records 35.8% of ruin for whoever stakes by their own
history and 0% with Laplace. This instrument measures 36.8% of "all-in" and **0.00% of ruin** with
any of the rules. A declared lock explains why. Kelly is limited to $f \le 1$ (one does not
stake more than one has), and with $f = 1$ a trader who earns 1% per week and loses at most
half per loss needs seven consecutive extreme events to fall below $10^{-6}$, and those
events do not happen in 1,000 weeks with chance 0.004. The old value is recorded. The two numbers measure
the same thing, with different leverage ceilings. And the drag also diverges from the course script.
The table of five rules in Chapter 11 uses a tail model of its own, and the two sets of heights
do not coincide; both are reported.

The repository's time-growth table (5.1% at $1\times$, 7.2% at $2\times$ and 6.0% at
$3\times$, plus the drag of 0.14 per year at $3\times$; the same three portfolios appear in decimal
in Chapter 17, which measures 0.0515, 0.0719 and 0.0601) **is not reproduced** by this instrument, because, with
the parameters calibrated by those two numbers, the model gives $3\times$ larger than $2\times$, and not
smaller. The probable reason is the real tail. The Dow from 1885 to 2023 has 19/10/1987 and the 1929-32 run,
which the Gaussian model does not have. The crash row, measured, is the evidence that the effect exists; the
exact value of the growth at $3\times$ depends on which tail enters the count.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarse. The line of this chapter, by the axis instrument (`labs --resolution`):

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

**Exercises.**

1. *(Count)* In the Peters coin, compute $\mathbb{E}[\ln M]$ by hand and explain why the median falls
   even with the expectation growing. Check it against the measured $-0.052680$.
2. *(Count)* For an even-odds bet with chance $p$, show that $f^* = 2p - 1$. Then
   compute $f^*$ for the Peters coin and check it against the 0.14 of the laboratory.
3. *(Concept)* In the Peters coin, three players who split the wealth every round grow 1.6%
   per round, when each one alone would decay. Why does splitting change the growth without changing the
   expectation of any of them?
4. *(Laboratory)* Run 10,000 investors on the strategy that sells insurance, with $q$ estimated by the
   raw frequency and by Laplace. Compare the fractions of "all-in" and the results, and report
   what changes when the leverage ceiling is removed.
5. *(Count)* In Bernoulli's ship, the insurance costs 1,800 and covers a loss of 34,000 with chance
   5%. By the cost/loss rule, should the shipowner insure? What does the time growth say?
6. *(Concept)* Give an example from your area in which the number the spreadsheet shows is an ensemble
   mean and the decision depends on a time mean. Who is on the side of the crowd and who is on the
   side of the trajectory?

**Summary.**

- In multiplicative dynamics, whoever lives one trajectory must maximise $\mathbb{E}[\ln M]$; the ensemble
  mean answers a different question.
- $+50\%$ against $-40\%$ gives an ensemble mean of $+5\%$ per round and a drift of $-5.27\%$; the product
  of one head and one tail is 0.90.
- Kelly gives the fraction that maximises growth; uncertainty about the parameter orders less staking, and
  Laplace's rule prevents zero losses from turning into zero risk.
- With a heavy tail, the sample mean misleads or does not even exist; with $\alpha = 1.1$, the mean of a million
  observations falls below the true value in 95% of the samples.
- The volatility drag grows with the square of the leverage. Measured per session, 0.000068 at
  $1\times$ and 0.000898 at $3\times$.

## Chapter 11. The price of error, Goodhart's law and who pays the bill

**Objectives.** By the end of this chapter you should know how to decide by the cost/loss threshold, compute the
expected value of perfect information, use the robust criteria when there is no probability worth
trusting, recognise a loop in which the forecast changes the forecast and declare who pays for the error.

---

### Step 1 — The real world and the paradox

**The shipowner who asks whether insurance is worth it.** A shipowner has 100 000 in the account. The insurance
contract costs 1 800 per month and covers a loss of 34 000 that happens with chance 5%. Is it worth it?

- **In expectation**, the shipowner pays 1 800 and receives, on average, $0{,}05 \times 34\,000 = 1\,700$. He loses
  100 per month. The insurer earns 100 per month. Whoever looks only at that account concludes that insurance is expensive.
- **In time**, each month multiplies wealth by a factor. Without insurance, in 95% of the months it is
  multiplied by 1,0 and in 5% by 0,66. With insurance, the multiplication is 0,982 every month, without
  exception. The two accounts give different numbers.

The criterion that settles the impasse is a one-line rule: **pay to protect yourself when the chance
of the event is greater than the cost divided by the loss**, that is, $p > C/L$. Here $C/L = 1\,800/34\,000 =
5{,}294\%$. The chance is 5%, therefore below the threshold by 0,294 percentage point, and the rule
says **not** to insure. It is one of the cases in which the answer of expectation and the answer of time do not
coincide, and the chapter shows both.

**In every 100 weeks.** The insurance salesman of the previous chapter has the opposite situation. He
earns 1% per week and loses half the capital with chance 0,4%. In 100 weeks, 67 end with
profit; the danger lives in the other 33. A history of 250 weeks shows no loss at all in
$0{,}996^{250} = 37\%$ of the cases, and whoever estimates the chance from the observed frequency concludes that it is
zero and leverages. It is the **rare event that does not appear in the sample**, and Chapter 10 has already measured the cost.

**The forecast that changes what it forecasts.** A published forecast is not a thermometer, because people
react to it. A traffic app that sends every user to yesterday's fastest route
moves the traffic jam to the route it recommended. With half of the drivers following the
app, those who use it take 41,0 minutes and those who do not, 36,8. The forecaster who splits the users to
equalise the times is wrong by 0,3 minute, and everybody takes 35,0. **The metric became a target and stopped
measuring.** That is Goodhart's law and the inverse of the version of the chapter. Here the forecaster *designed* the
loop and stabilised it.

**Who pays when the error rate is different for different groups.** A triage system
decides who enters a programme. It gets the probability right within each group (it is calibrated) and,
even so, it errs in different ways for each one: the group with more real cases is flagged more
often than necessary, and the group with fewer cases is missed more often. The easy way out
("do not use colour in the model") does not work, because colour enters through the correlates.

---

### Step 2 — The rule of the game

**The threshold and the value of information.** If protecting costs $C$ and not protecting costs $L$ when the event
occurs, one protects when $p > C/L$. A piece of information is worth as much as it can move the probability to
the other side of that threshold. The bits it carries do not enter that account. The value of knowing the state of the
world before acting, the EVPI, is the ceiling of the value of any information:

$$\mathrm{EVPI} = \mathbb{E}_\theta\big[\max_a U(a,\theta)\big] - \max_a \mathbb{E}_\theta\big[U(a,\theta)\big]$$

The value of any real test (EVSI) lies between zero and it. **Bits are not value.** In one example of the
course, a disease has prevalence 0,3; treating costs 1 and leaving a sick person untreated costs 10,
so that one treats when the chance passes 0,10. Two tests carry the same mutual information,
0,319 bit. The sensitive test is worth 0,419, against a ceiling (the EVPI) of 0,70. The specific one is worth zero, because even
its negative result leaves the chance above the threshold and does not change the action.

**Not all information is welcome.** Good (1967) proved that free information never makes the decision
of someone with preferences over states of the world worse. But whoever has preferences over future
observations may pay not to know. Among people at risk of Huntington's disease, only a minority
take the genetic test before the symptoms, 17,4% in the United Kingdom between 1994 and 2014 (Baig et al., 2016).

**Criteria that dispense with probabilities.** The threshold $C/L$ and the EVPI presuppose a distribution over the
states of the world. In the uncertainty corner of the Stirling matrix (Chapter 4), it does not exist in a
trustworthy way, and decision theory offers robust criteria. The invariant $\Phi_4$ (Chapter 14) requires
declaring which one was used:

| Criterion | Rule | Posture |
| --- | --- | --- |
| Wald (maximin) | Choose the action whose worst case is the least bad | Maximum pessimism |
| Savage (minimum regret) | Choose the action whose largest regret is the smallest | Avoiding "if I had known" |
| Laplace | Treat the states as equally likely | Ignorance as uniform |
| Hurwicz | Weight the worst and the best case by an optimism index | Declared middle ground |

**The dikes of Holland.** After the 1953 flood, the Delta Commission adopted for the central Netherlands
a standard of 1 in 10 000 years, and van Dantzig (1956) treated the height of the dike as a decision
problem, balancing the cost of the works and the expected damage. The laboratory redoes the problem with a
model. The annual maxima follow a GEV of unknown shape $\xi$, between $-0{,}1$ and $0{,}2$, and the
annual cost of a height $h$ is the works, $h - 2$, plus the expected damage, $3000 \times P(\text{maximum} > h)$.
Five decision rules give five different heights, and the criterion and the list of models have to be
declared before seeing the answer. Chernoff (1954) showed that Savage's criterion can change its
choice merely by adding an alternative that nobody would choose; here one adds a model.
**The robust criteria are not more "correct" than the expected one: they are the honest choice when the
probability the account depends on does not deserve trust.**

**The forecast that changes what it forecasts.** When a model is published, people react to it. Lucas
(1976) showed the phenomenon in economics. The parameters of a model estimated under a policy depend
on the rules the agents expect, and they change when the policy changes. In one example of the course, the old
Phillips curve promises 4,59% unemployment and the new regime delivers 5,88%. The answer of Kuhn and
Perdomo and colleagues (2020) is conceptual: **performatively stable** is the model that is optimal for the
distribution it induces.

**To learn or to deform?** An agent that minimises the variational free energy can lower surprise in
three ways: changing the belief, changing the world or ceasing to look.

$$F[q] = D\big(q \,\|\, p(s \mid o)\big) - \ln p(o) \;\ge\; -\ln p(o)$$

A thermostat that "believes" in 22 °C in a room at 18 °C reaches an identical $F$ when it learns that the
room is cold and when it heats it to 22 °C. Only a counterfactual from outside separates the two cases, and for that
reason the architecture requires running the model with and without the action.

**The H3 loop, measured.** H3 assumed that whoever abandons the queue calls back with probability 0,6.
That parameter was measured in another real support desk, from 1999, with a quasi-experimental design. The queue
found on arrival, at an identical date and time, serves as an instrument for the abandonment. Under that
design, abandoning instead of being served increases by **0,145** the chance that that client calls back
within fifteen minutes, with a 90% interval from 0,132 to 0,157. The sign checks out and the order of
magnitude does not. The declared loop is about four times the observed one, outside the band from one third to three
that had been fixed in advance. The excess of arrivals that the measured loop generates in that support desk is 2,3%
of the arrivals, against the 29% bias that the declared loop produced in H3; the comparison is one of order of
magnitude, because the H3 bias also depends on the simulated abandonment rate, and not only on the loop. And the
measured value is probably a lower bound, because a new call is only visible when it enters the
queue.

In H3, the control of the loop was made with a small experiment embedded in the operation. In 10% of the
intervals, a draw multiplied the staffing level by 0,7 or 1,3. As the draw is independent of
the demand, the regression of the following arrivals on the drawn deficit identifies the effect of the action. The
naive forecaster, trained on the observed arrivals, **overestimates the exogenous demand by 29%** and even
so hits the observed values with 1% error. The target metric approves a forecaster that is wrong about
the world. With the instrument, $\hat r$ comes out at 0,56 (90% interval from 0,51 to 0,61) and the error of the
corrected forecast falls to **1,7%**, inside the limit of 3%.

**Who pays for the error.** When the base rates differ between groups, a single threshold produces different
error rates. The identity of Chouldechova (2017) shows why:

$$\text{FPR}=\frac{p}{1-p}\cdot\frac{1-\text{VPP}}{\text{VPP}}\cdot(1-\text{FNR})$$

With predictive value (PPV) and false negatives equal between groups, different prevalences $p$ force
different false positive rates. **Equalising approvals, equalising errors and equalising calibration by group
are incompatible requirements when the prevalences differ and the predictor is not perfect.** "I want
fairness" is not yet a decision. One has to declare which equality was chosen, what it costs, where
the error migrated to and who comes to pay for it.

**Communicating is part of the decision.** A badly communicated result failed in the Agent layer. "30%
chance" does not say 30% of what; "of every 100 people like you, 30" does. The rules are fixed
denominators, absolute risk before relative and an explicit reference class. Hypothetical outcome plots
(HOPs), which show simulated outcomes one after another, make the reader feel the variation
that an error bar hides. The recommendation declares the point at which the action changes, act if $p > C/L$,
and an objective clause for when it stops holding.

---

### Step 3 — Hands on

Four tasks: the cost/loss calculator on the Bernoulli ship with the growth in time, the EVPI
and the EVSI in a two-act case, the four tail models of the dike with five decision rules, and the
trilemma table with the Chouldechova identity. Code verified in `crates/stochology-book/src/ch11.rs`; output
measured with `cargo run --release --bin labs -- --chapter 11`:

```text
== Chapter 11 :: the Bernoulli ship (C = 1.800, L = 34.000, p = 0,05) ==
threshold p > C/L: p* = 0.05294 (5.294%)  ->  p = 0,050 does not pass the threshold
in expectation: expected loss without insurance = 1700, insurance result = +100 per month
shipowner growth in time: 1 ship without insurance = -0.0208, 1 ship with insurance = -0.0182, 3 ships without insurance = -inf
insurer with     1 contracts: growth in time = -0.000186 per month
insurer with    10 contracts: growth in time = -0.001808 per month
insurer with   100 contracts: growth in time = -0.013466 per month
insurer with  1000 contracts: growth in time = +0.100410 per month

== Chapter 11 :: the EVPI (treating costs 1, leaving a sick patient costs 10) ==
action threshold: p > 0.10  ->  with p = 0,3 the best act without information treat (-1.00)
gain of whoever knows the state before acting = -0.30
EVPI = +0.70
  test sensitive  (0,90; 0,70): EVSI = +0.2400
  test specific (0,30; 0,95): EVSI = +0.2450
  with prevalence 0.05: best act without information = do not treat, EVPI = +0.450
  with prevalence 0.30: best act without information = treat, EVPI = +0.700
  with prevalence 0.60: best act without information = treat, EVPI = +0.400

== Chapter 11 :: the van Dantzig dikes, four tail models ==
  model     xi = -0.1  xi = +0.0  xi = +0.1  xi = +0.2   optimal height
  T = 100          4.07       1.26       5.71       7.01
  T = 10000        5.84       1.26      12.78      21.49
  optimal height with xi = -0.1: 5.839 m  (cost 4.143)
  optimal height with xi = +0.0: 3.059 m  (cost 1.130)
  optimal height with xi = +0.1: 9.809 m  (cost 9.426)
  optimal height with xi = +0.2: 12.584 m  (cost 13.613)
  expected value with the Gumbel: 3.06 m
  mean of the models: 7.82 m
  Wald maximin: 12.58 m
  Savage minimax regret: 9.81 m (largest regret 6.68)
  precaution: 1 in 10 000 in the worst model = 21.49 m
  EVPI of the model set = +18.427 m

== Chapter 11 :: the Chouldechova trilemma ==
group     base rate        FNR        FPR  false positives  false negatives        PPV
White         0.069      0.240      0.105              467               79     0.9408
Black         0.111      0.360      0.196              832              191     0.9096
observed cases: White 329 marked by the facts, Black 530

-- FNR matched downwards, to the FNR of White at 0.240 --
White         0.069      0.240      0.105              467               79
Black         0.111      0.240      0.045              192              127
total false positives: 1299 -> 659 (-640)
total false negatives: 270 -> 206 (-64)

-- FNR matched upwards, to the FNR of Black at 0.360 --
White         0.069      0.360      0.246             1095              119
Black         0.111      0.360      0.196              832              191
total false positives: 1299 -> 1927 (+628)
total false negatives: 270 -> 309 (+40)

Chouldechova identity: FPR = p/(1-p) * (1-PPV)/PPV * (1-FNR)
group    PPV of the matrix PPV of the identity          ratio
White              0.3491             0.0592            0.2x
Black              0.2896             0.0904            0.3x
reading: with the observed FNR and FPR, the score would only be calibrated by group
if the positive predictive value were the one in the middle column. It is much larger: the score
separates reasonably and is not read as a probability of risk.
```

**What each line says.**

**(1) The Bernoulli ship.** The threshold is $C/L = 0{,}05294$, or 5,294%, and the chance of 5% stays **below**
it. The cost/loss rule says not to insure. In expectation, the loss without insurance is 1 700 and the
result of the insurance is $+100$ per month for the insurer. The growth in time reverses the reading for
the shipowner. With one ship, $-0{,}0208$ per month without insurance and $-0{,}0182$ with insurance, both negative,
and insurance **improves** the result in spite of expectation saying the opposite. With **three** ships, the
growth without insurance is $\mathbf{-\infty}$, because three simultaneous losses leave the wealth without
cover for the damage; ruin stops being a small probability and becomes an absorbing
state.

**(2) On the other side of the counter.** The insurer improves when it accumulates independent contracts. With 1
contract the growth in time is $-0{,}000186$ per month; with 10, $-0{,}001808$; with 100,
$-0{,}013466$; with **1 000 contracts, $+0{,}100410$**. The premium does not change in any of those cases;
what changes is the number of independent trajectories it holds. The first lines go down because the
formula used is the mean minus variance divided by two approximation, and with few contracts the
variance is too large for the approximation to hold; from a few hundred contracts the law
works and the growth becomes positive. **Whoever shares the risk charges the same and grows; whoever carries
the trajectory alone sinks.** It is the answer of Chapter 10, now with an insurance company in
place of a gambler.
**(3) The EVPI.** With prevalence 0,3, treating costs 1 and leaving a sick person untreated costs 10, the
action threshold is $p > 0{,}10$ and the best action without information is to treat, with gain $-1{,}00$. Whoever knows
the state before acting has $-0{,}30$, and the difference is the **EVPI $= +0{,}70$**. The two tests of the text
deliver close values, EVSI of $0{,}2400$ for the sensitive one $(0{,}90; 0{,}70)$ and $0{,}2450$ for the
specific one $(0{,}30; 0{,}95)$, both below the ceiling. With prevalence 0,05 the EVPI falls to 0,450; with
0,30, it rises to 0,700; with 0,60, it falls again to 0,400. **The value of information is maximal when the
probability is close to the action threshold**, and that is where information changes the decision.

**(4) The dikes.** The four models disagree little at the **100-year** level (from 1,26 to 7,01 m) and
a lot at the **10 000-year** level (from 1,26 to 21,49 m). The tail is where the models separate. The five
rules give five heights: expected value with the Gumbel, **3,06 m**; mean of the models, **7,82 m**;
Wald maximin, **12,58 m**; Savage minimax regret, **9,81 m**; precaution of 1 in 10 000
in the worst model, **21,49 m**. The expected value of a single model chooses the cheapest and the most
fragile height; precaution chooses the most expensive; Savage limits how much one can regret. The EVPI of the set
of models, the distance between precaution and the expected value, is worth **18,427 m**, and it is the measure of
how much it costs not to know which is the right tail.

**(5) The trilemma table.** The two groups are White, with base rate 0,069, FNR 0,24 and FPR 0,105, and
Black, with base rate 0,111, FNR 0,36 and FPR 0,196. At the system threshold, there are 467 and 832 false
positives and 79 and 191 false negatives. Equalising the FNR **downwards**, to the level of White, the false
positives fall from 1 299 to 659 and the false negatives from 270 to 206. Equalising **upwards**, to the
level of Black, the false positives rise from 1 299 to 1 927 ($+628$) and the false negatives from 270
to 309. **The choice of which equality to pursue is not neutral: each direction moves the two errors in
opposite directions and in different magnitudes.**

**(6) The identity closes and accuses.** The positive predictive value that the confusion matrix delivers is
0,3491 (White) and 0,2896 (Black). The identity of Chouldechova requires, given the base rate, the FNR and the
FPR observed, 0,0592 and 0,0904, four to six times smaller. **The score separates reasonably and is not
read as a probability of risk.** It is the numerical translation of what the identity says. With the three
observed rates, the predictor cannot be calibrated by group and keep the error rates equal in the
two.

**Divergence declaration.** The previous edition records 35,8% of ruin for whoever bets by their own
history and 0% with Laplace (Chapter 10), and a table of dikes with heights of 4,76 m (expected
value with Gumbel), 6,20 m (mean of the models), 7,46 m (Wald), 5,93 m (Savage) and 9,96 m (precaution),
with 100-year levels from 3,11 to 4,26 m and 10 000-year levels from 3,81 to 9,96 m. This instrument, with
$\mu = 1{,}2567$ and $\sigma = 0{,}762$ calibrated so that the 100-year level of the model with $\xi = -0{,}1$
reaches 4,26 m, measures 3,06 m, 7,82 m, 12,58 m, 9,81 m and 21,49 m. The old heights come from a
set of models calibrated in another way, which this laboratory does not reproduce; the two sets
stay recorded. In the trilemma, the old value of 1 273 false positives for equalising the FNR is from the
control table of H1; this laboratory measures $+628$ (with $+274$ in the opposite direction), because the computation here
starts from the rates by group instead of the units. The divergences are declared and no old value
was erased.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarsely. The line of this chapter, from the instrument of the axis (`labs --resolution`):

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

**Exercises.**

1. *(Computation)* In the Bernoulli ship, insurance costs 1 800 and covers a loss of 34 000 with chance of
   5%. By the cost/loss rule, should the shipowner insure? How does the growth in time answer?
   Explain the two answers in one sentence each.
2. *(Computation)* Redo the EVPI with prevalence 0,10 (exactly the threshold). Does the value of perfect
   information increase or decrease? How does that guide the spending on tests?
3. *(Concept)* In the example of the two-route app, who errs more, the naive forecaster or the
   forecaster that equalises the times? In what sense is the second one "performatively stable"?
4. *(Computation)* With the rates of the laboratory (White with 0,069, 0,24, 0,105), check the identity of
   Chouldechova and say by how much it errs. Is the error one of rounding or of modelling?
5. *(Laboratory)* In the laboratory, change the base rate of the second group from 0,111 to 0,20 and redo the
   trilemma table. Which equality becomes more expensive, and why?
6. *(Concept)* Rule 14.2 of the book forbids 0% and 100% probability. Which problem of the ship
   laboratory does that prohibition avoid?

**Summary.**

- One decides by the threshold $C/L$. With $C = 1\,800$ and $L = 34\,000$, the threshold is 5,294% and the chance of 5%
  stays below it. The growth in time can say the opposite, and the two numbers have to be
  declared.
- Information is worth as much as it moves the decision. The EVPI is the ceiling, and bits are not value.
- When the probability does not deserve trust, the criteria of Wald, Savage and precaution replace the
  expected value; the criterion and the list of models are declared choices, never deductions.
- Published forecasts change the world. The Goodhart measured in H3 brings the 29% bias down to 1,7%, and the
  real loop measured is four times smaller than the declared one.
- Fairness requires choosing between incompatible equalities. Equalising the FNR upwards costs $+628$ false
  positives, downwards it saves 640, and who pays is declared before running.

### Who pays, in the research

The research closed the seven governance opinions and the three prison streams. On 15 October
2026, `governance/GRUPOS.md` records that **none of the seven affected groups was heard, in any of the
seven decisions.** The file is the answer of the front to the objection "the auditor is a stakeholder".
The signature of the officer is not the listening of the affected group. The officer is on the side of whoever decides
to use the artifact; whoever pays for the error stays on the other side. The list includes the 5 226 detainees of Toronto in
1997–2002, the 65 276 of 2020–2021 and the 86 113 traffic stops in Rhode Island 2005–2015, all
flagged by perceived colour. The audit could not look at three things: who authorises, the human panel
that was never done and the decisions that never reached anyone. Chapter 21 returns to this point.

# Part IV. Building the auditable machine

## Chapter 12. The map of knowledge: the typed hypergraph and the five lenses

**Objectives.** By the end of this chapter you should know how to read the definition $\mathcal{G} = (V, E_{\mathrm{tree}},
E_{\mathrm{graph}}, H, \Phi)$, list the five mandatory fields of a node, recognise a pathology edge
and use the five lenses (Fluxonomy, Chrononomy, Scalonomy, Morphonomy and Crisonomy) as
questions that one asks of any problem.

---

### Step 1 — The real world and the paradox

**The conclusion that lives in the middle of the paragraph.** In a support desk, a report begins like this:
*"Considering the two-year history, the mean arrival rate of 312 calls per hour, the mean
service time of 6,2 minutes and the application of the Erlang C formula with 36 agents, the estimated
level of service is 82% of the calls answered within 20 seconds, and it is recommended to keep the current
staffing."* It is a long sentence, correct in its arithmetic and almost impossible to audit. If the conclusion is wrong, the error
is hidden somewhere between "considering" and "it is recommended", and nobody knows where.

The first measure of an auditable machine is not to compute better. It is **to forbid the long sentence** and
to force every claim to become a card with fixed fields. A card that does not fill every field does not
enter the library. In every 100 reports written in prose, a good part hides the fragile point in an
adjective. In 100 cards with those five fields, the fragile point shows up in the empty field.

**The empty field that denounces.** The report of the support desk uses three things at once: the calls that
arrived, the formula that forecasts the wait and the decision on staffing. If each one becomes a separate card, three
questions are visible at once. *Where does the demand number come from?* *Under what conditions does the formula
hold?* *Who pays if it fails?* The sentence hid all three. The card shows the three that are answered
and the ones that are not.

**There are three kinds of link, and one of them is an alarm.** First come the dependency links. The
service time depends on the demand, and the wait depends on the service time. Then come the
transversal links. The Jensen curve appears in the queue, in the computation of information and in the stock market. Last,
the **pathology links**, which are not dependencies but misalignments. The most common pathology of this
story is the **fallacy of the mean**. The formula is applied to the mean rate of the history when it should be
applied to each day and then the mean of the results taken. In the support desk, the two computations give very different
numbers, 82% against 57%.

**The five questions that one can always ask.** After building the map, five ways of looking at it
remain. *Does the account close?* (information is not created out of nothing by reprocessing). *Which time is being spoken of: the
process, the observer or the simulation?* *Is the tail so heavy that the mean does not even exist?* *Does the structure
found survive when the environment changes?* *Is the information worth its cost, and are the forecasts
calibrated?* Each question has a quantity and a number that can be measured. The five together are the **five
lenses**.

---

### Step 2 — The rule of the game

**The formal map.** Knowledge is not kept in ambiguous text. It is kept as a **typed
hypergraph**, that is, a set of cards (nodes) linked by three families of lines.

$$\mathcal{G} = (V,\; E_{\mathrm{tree}},\; E_{\mathrm{graph}},\; H,\; \Phi)$$

| Component | What it keeps | Example from the support desk |
| --- | --- | --- |
| $V$, nodes | Concepts, hypotheses, variables, data sets | `L1_demanda_real`: call counts every 5 minutes |
| $E_{\mathrm{tree}}$, tree | What depends on what: deductive prerequisites | `L1_demanda` $\to$ `L2_erlang_c`, with the label "input of the model" |
| $E_{\mathrm{graph}}$, transversal | Mathematical bridges, causal edges $\operatorname{do}(x)$ and pathology edges | Jensen links the mean function to information and to ergodicity |
| $H$, hyperedges | The five lenses, which wrap whole groups of nodes | Chrononomy over `L1_demanda` and `L2_erlang_c`: "the demand has a calendar" |
| $\Phi$, invariants | The invariants that every claim fulfils to enter the graph | $\Phi_2$: the margin on the independent unit |

**Every claim becomes executable code.** The central node of the map, `N_Core`, carries the one-line definition and the pointer to the artifact that executes it. Whoever asserts delivers the code that runs the assertion. The sentence of the programme, every empirical claim becomes executable code, is literal.

**The five mandatory fields of a node.** A node without any of them is refused on writing.

| Field | What it declares |
| --- | --- |
| `id` | Unique identifier |
| `camada` | $L_1$ World, $L_2$ Model or $L_3$ Agent |
| `tipo_incerteza` | Aleatory, epistemic or knightian |
| `definicao_formal` | The mathematical definition, in one line |
| `artefato_simulavel` | The pointer to the code that reproduces it with a fixed seed |

A real node, copied from the card of $H_0$ in a real domain:

```yaml
- id: L2_erlang_c
  camada: L2
  tipo_incerteza: Knightiana_Especificacao
  definicao_formal: SL(c, λ) = 1 − C(c, λ·AHT) e^{−(c − λ·AHT) t / AHT}, estacionária, sem abandono
  artefato_simulavel: estocologia/central.py::nivel_servico
```

Notice that the Erlang C formula is typed as a **knightian uncertainty**. It assumes stationarity and
absence of abandonment, and the World guarantees neither. The card says, in one field, where the formula
can fail. The long sentence did not say.

**The pathology edges.** These are not dependencies but structural misalignments, the moment at
which the Agent applies a Model incompatible with the World.

| Pathology | What the edge marks |
| --- | --- |
| Fallacy of the mean | $\mathrm{SL}(\mathbb{E}[\Lambda])$ in place of $\mathbb{E}[\mathrm{SL}(\Lambda)]$ |
| Inspection paradox | The Agent imposes a stable Model on a World that oscillates |
| Transposition of the conditional | $P(E \mid I)$ read as $P(I \mid E)$ — the prosecutor's fallacy |
| Russell's turkey | The real process is outside the hypothesis space, and more data only increase confidence in the wrong place |
| Illusion of the dark room | One looks for the cause of the case, not the cause of the frequency |

**The mathematical bridges.** The transversal edges link distant modules by an explicit relation,
navigable in both directions. The contract of the agents fixes four.

| Bridge | Relation | What it links |
| --- | --- | --- |
| Jensen | $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$ for convex $f$ | The mean function to information and to ergodicity |
| Prior as regularisation | $-\ln P(\theta) \leftrightarrow \lambda\Omega(\theta)$ | Bayes to learning with a penalty and to the SHELF elicitation of $F_7$ |
| Information as growth | $\Delta W \le I(X; Y)$ | Mutual information to the Kelly growth gain |
| Noise as temperature | Minibatch noise $\leftrightarrow$ temperature $T$ | Stochastic mechanics to active inference and to learning |

The second bridge says that penalising large coefficients is equivalent to declaring a prior that finds them
unlikely. Whoever regularises is already computing a maximum a posteriori, without saying which prior was used. The third says
that a gambler with side information $Y$ about the outcome $X$ gains, in growth rate, at most
$I(X; Y)$ bits per round, and those bits become growth only for whoever bets by Kelly. The fourth says
that the noise of estimating the gradient on a minibatch acts as a temperature, and helps to escape shallow
minima.

**The five lenses.** The hyperedges work as questions that one asks of any problem. Each lens
has a quantity that it measures and a critical habit, and it crosses the concepts without adding new objects.

| Lens | Question | What it measures | Critical habit |
| --- | --- | --- | --- |
| Fluxonomy | How do probability and information flow and get lost? | Currents, mutual information | Does the account close? Reprocessing does not create information |
| Chrononomy | Does the mean across many coincide with the mean over time of a single one? | Mixing, relaxation and escape times; $\mathbb{E}[\ln M]$ | Which time is being spoken of: the process, the observer or the simulation? |
| Scalonomy | Is the tail heavy to the point that the mean does not even exist? | Tail exponent, effective size, capacity of the model | Declare whether the theoretical mean exists |
| Morphonomy | Does the structure found survive when the environment changes? | Invariance (ICP), compressibility (MDL) | Rewrite the model in two forms and see what is conserved |
| Crisonomy | Is the information worth its cost, and are the forecasts calibrated? | Loss, proper score, value of information | Charge every number the record of hits at the frequency it promised |

Two examples of how a lens becomes an account. In **Fluxonomy**, the data-processing inequality
says that, if $Z$ is obtained from $Y$, then $I(X; Z) \le I(X; Y)$. Three newspapers that publish the same research
are worth one. In **Morphonomy**, two representations can give identical forecasts and still authorise
different conclusions under intervention, and only the conclusion that is conserved across environments serves outside them.

**How a new domain triggers the lenses.** The practical rule is in the plan. Is it a financial, career or
insurance process? It triggers **Chrononomy**. Does it involve claims, incomes, virality or the size of
companies? It triggers **Scalonomy**. Does it involve recommendation, regulation or a published forecast? It triggers
**Fluxonomy** and **Morphonomy**, with simulation with and without the action.

**Where the data live.** The hypergraph keeps only the light topology: nodes, edges, YAML contracts and hashes.
Microdata and series stay in partitioned Parquet, queried with explicit column projection, partition
pruning and a rigid budget of bytes scanned. The plan also fixes the rule of scale. A model that
requires a full scan per forecast does not scale, however good its test error.

The size of the graph grew with the research, but below the targets of the plan. There are 85 nodes in the real $H_1$ and
**454 178** nodes in $H_3$, of which 453 973 are forecasts and only 205 are topology. $H_3$ reached 45% of the
million that the plan foresaw.

The number of $H_2$ deserves a correction, because it shows how easy it is to inflate that account. The "1 538 nodes" that the
first version of this chapter cited are from an old run of the graph generator. Running the
`grafo.py` of $H_{2g}$ over the results of $H_{2g}$, the graph has **12 797 nodes**, and
12 288 of them, 96%, are ICP hypotheses, one per subset tested, which grow like $2^k$ and are not new
knowledge.

The scale test of the background audit ran the funnel in batch over **329** domains generated by rule from
a public catalogue (327 up to the end, at 3,8 seconds of processor per domain) and took the graph to
**26 634 nodes**, 28 619 tree edges, 26 795 bridges and 4 628 hyperedges. Only **6 494** of the nodes are
concept nodes (domain, layers, variables, environments, filters, decision), and only **554** come from curated packages;
the other 20 140 belong to that family of hypotheses. The bottleneck is not in the machine but in semantic
curation, which does not scale by rule, in the supply of data sets with identifiable environments (9% of the catalogue
pass), in the bridges, which grow with the square of the number of domains, and in keeping everything in memory.

---

### Step 3 — Hands on

Three tasks: build a mini-hypergraph with three nodes and three lenses, prove that the validator bites in four
cases, and run the lenses with a measured number. **Scalonomy** enters through the Hill estimator,
**Crisonomy** through the expected value of perfect information (EVPI), and **Morphonomy** through the test of
invariance across regimes. Code in `crates/stochology-book/src/ch12.rs`; output measured with
`cargo run --release --bin labs -- --chapter 12` (self-contained module, without external
dependencies):

```text
== Chapter 12 :: the map of knowledge ==
graph: nodes=3 (L1=1 L2=1 L3=1) E_tree=2 E_graph=4 hyperedges=3 validation=[]
topological order = ["L3_manager", "L2_erlang_c", "L1_demand"] | leaves=1 roots=2 | nodes without lens = []
proof 1 (edge to an absent node): ["edge to absent node: L2_erlang_c -> L2_ghost_node"]
proof 2 (deductive cycle): ["cycle in E_tree: [\"L1_demand\", \"L2_erlang_c\", \"L3_manager\"]"]
proof 3 (hyperedge with one member): ["hyperedge with fewer than two members: H4"]
proof 4 (node outside every lens): ["L2_orphan"] validation=[]
lenses triggered = ["Chrononomy", "Crisonomy", "Scalonomy"] | not triggered = ["Fluxonomy", "Morphonomy"]
Scalonomy: hill_alpha(k=250) = 1.8444 wild_tail=true values_above_1000=0
Scalonomy: hill_alpha(k=500) = 1.7901 wild_tail=true values_above_1000=0
Scalonomy: hill_alpha(k=1000) = 1.7949 wild_tail=true values_above_1000=0
Scalonomy: hill_alpha(k=2000) = 1.8032 wild_tail=true values_above_1000=0
Crisonomy: EVPI = 0.0000 | best act without information = 5.8000 | value with perfect information = 7.0000
Crisonomy: without the option of acting (only betting half = 3.0000 or not betting), the best without information is 2.1000 and the EVPI rises to 3.7000
Crisonomy: useless sensor (same payoff in both states): EVPI = 0.0000
Morphonomy: rule 1 -> marginal slope = 24.37 (treated fraction = 0.474) | by regime = [9.96, 10.09, 10.15] | invariant(0,5) = true
Morphonomy: rule 1 with the allocation swapped -> marginal slope = -4.60 | by regime = [10.34, 9.92, 10.15] | invariant(0,5) = true
Morphonomy: rule 2, middle regime with effect 40 -> marginal slope = 40.03 | by regime = [9.96, 40.09, 10.15] | invariant(0,5) = false | step within the regime = 30.13
```

Four readings of the output, in the order in which they matter.

**The validator refuses.** The valid graph passes with zero problems. The four proofs show that the refusal is not
decorative: an edge to a node that does not exist, a deductive cycle (the "return" that turns the tree into a wheel),
a hyperedge with a single member, and an orphan node outside every lens. Proof 4 shows the structural validator
letting the orphan node through: the validation `[]` stays empty, while `nos_sem_lente()` points at `L2_orfao`.
The two screens take care of distinct things; one takes care of the form, the other of coverage.

**Not every lens is triggered.** The mini-hypergraph triggers three lenses (Chrononomy, Scalonomy and
Crisonomy) and leaves Fluxonomy and Morphonomy without a hyperedge. The absence is not an error of the validator. The error
would be if some **node** stayed outside every lens, and none does. A lens without use is a question that
has not yet been asked.

**Scalonomy reproduces the number of the instrument of the repository.** With the same seed (1909), 20 000
values of a Pareto with $\alpha = 1{,}81$ and the cutoff at $k = 500$, the Hill estimator returns
**1,7901**, and the gate of the repository publishes 1,7901 in that account. The tail is wild
($\hat\alpha < 2$), and for that reason the theoretical variance does not exist. At the edges of the cutoff the number moves, with
1,8444 at $k = 250$ and 1,8032 at $k = 2\,000$, and the sensitivity is the lesson. The tail exponent is an
estimate, and not a constant of nature.

**Crisonomy depends on which actions exist.** The perfect sensor says whether the year will be good or bad, with
probability 0,70 of good. Profiting yields 10, losing costs 4. Whoever can *act or not act* already acts without the
sensor, because the expected value of acting is 5,80, greater than zero, and so the sensor is worth **0,0000**, because
it never changes the decision. Without the option of acting, leaving *bet half* (value 3,00) or *not bet*
(0,00), the best without information falls to 2,1000 and the sensor comes to be worth **3,7000**. If the payoff is
equal in both states, it is worth 0,0000 again. **The value of information is not a property of the datum: it is a
property of the set of available actions.**

**Morphonomy prefers the map to the number.** Three regimes, bases 100, 140 and 120, true effect 10 in all
three. In the first rule, the slope computed over all the data together gives **24,37**, more than double
the true effect, because those who received the treatment are concentrated in the high-base regime. Swapping
only the allocation, and keeping the effect of 10, the marginal slope becomes **−4,60**, negative, when every
effect within each regime is positive. It is the classic misalignment that Chapter 9 called
confounding. The slopes by regime, those, stay at 9,96 / 10,09 / 10,15 and 10,34 / 9,92 / 10,15. The
two rules pass the invariance test with tolerance 0,5. When the middle regime really responds
40 instead of 10, the invariance **fails** (9,96 against 40,09, step of 30,13). The marginal slope is a
number; the slope by regime is a structure.

**Divergence declaration.** The previous edition of this book, in Chapter 6, presents the three samples of
Hill with values close to $1{,}81$; this instrument measures 1,8444 ($k = 250$), 1,7901 ($k = 500$), 1,7949
($k = 1\,000$) and 1,8032 ($k = 2\,000$). The difference is one of sampling and of cutoff, not of formula — the value
published in the repository for $k = 500$ with seed 1909 reproduces exactly. It is recorded as well that the
previous edition and the plan of the repository count gates in different ways: the old book describes
**five** automatic gates (contracts and seeds; schema and idempotence; convergence; temporal cutoff;
governance, fairness and exit), while the `gates` binary implements **seven** (G1 determinism,
G2 fixed seed, G3 declared scope, G4 invariants and type III, G5 integrity of the trail, G6 panel
$A_1$–$A_8$, G7 exit clause), with exit code 0 or 1. Chapter 14 details the seven; no old value
was erased.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarsely. The line of this chapter, from the instrument of the axis (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Write an $L_1$ node and an $L_2$ node for the claim "with this mean demand, the mean
   wait will be about 9 minutes". Which pathology edge links them, and in which sense does it point?
2. *(Concept)* Which lenses must a credit-granting model published to clients trigger?
   Justify each one in one sentence.
3. *(Computation)* In the output above, the EVPI is 0,0000 with the option of acting and 3,7000 without it. Explain why the same
   sensor is worth zero in one case and 3,70 in the other, and compute the EVPI for a third option whose best value
   without information is 4,00.
4. *(Laboratory)* Add a fourth node `L3_auditor` to the mini-hypergraph and check what the
   validator does before and after putting it into a hyperedge. Then create a second cycle in the tree and
   confirm that the message points at the cycle.
5. *(Laboratory)* Change the tolerance of the invariance test from 0,5 to 2,0 and to 0,05 in the two
   first rules. In which of them does the structure by regime stop being distinguishable from noise?
6. *(Concept)* Why was the Erlang C formula typed as knightian and not as epistemic?

**Summary.**

- The typed hypergraph keeps claims as nodes with layer, uncertainty type, formal definition and code.
- Pathology edges mark misalignments between layers; the validator refuses incomplete nodes, absent
  endpoints, deductive cycles and hyperedges with fewer than two members.
- The five lenses are transversal questions with their own quantities: information, time, tail, invariance
  and cost of information.
- The same graph gives different numbers according to the lens: Hill returns 1,7901 for $k = 500$; the EVPI returns
  0,0000 or 3,7000 depending on the available actions; the marginal slope changes sign while the
  slopes by regime stay equal.
- The node count inflates easily: of the 26 634 nodes of the scale audit, only 6 494 are concept nodes.

## Chapter 13. The team of specialists, the adversarial panel and AI under control

**Objectives.** By the end of this chapter you should know the blind spot of each of the eight tracks Track 1 to Track 8,
go through the six-stage cycle, apply the approval condition of the adversarial panel
$\mathrm{Aprovado}(X) = \bigwedge_{i=1}^{8} \left( A_i(X) \ge \mathrm{Severidade}_{\min} \right)$ and
distinguish System 1 (fast triage, probabilities trapped between 1% and 99%) from System 2
(deliberation over the graph).

---

### Step 1 — The real world and the paradox

**The group that approves itself.** A student hands in a statistics assignment. The student chose the
sample, cleaned the data, ran the test, looked at the result, changed the test, ran it again and wrote
the conclusion. When the conclusion finally pleased him, he stopped. Nobody cheated. Each step seemed
reasonable at the moment it was taken.

Now split the work among eight colleagues, each responsible for one single thing, and hand a
ninth colleague the **artifacts** in place of the report: the code, the data file, the saved model, the
audit trail, and their task is to break them. In every 100 assignments made by one person alone, the
most fragile part is the one nobody looked at from outside. In 100 assignments with a team that attacks, the fragile
part is the one that survived the attack.

The ninth colleague has one rule: **do not read the report**. They run the code with a seed that the author did not
choose, inject a corrupted batch, tamper with a line of the trail to see whether the verifier notices. Whoever
only reads reports agrees with them.

**The eight tracks and the blind spot of each one.** Each track exists because there is a way of erring that it
sees better than the others, a blind spot that the others do not cover.

| Track | What it does | The blind spot it covers | What it delivers |
| --- | --- | --- | --- |
| Track 1 Code | Turns hypotheses into executable generating processes | "The code runs" is not "the code is the hypothesis" | Fixed seed, pre- and post-conditions, property tests, `cProfile` and `tracemalloc` |
| Track 2 Data | Audits the entry of the World | What a row is, and why the datum is missing | `datacontract.yaml`, quarantine of defective records, idempotent ingestion, byte budget |
| Track 3 Lifecycle | Tests the model in time | The world changes and the model does not warn | Temporal cutoff without leakage, drift sensors, policy of ignorance, post-mortem |
| Track 4 Frontier | Runs the heavy inference | One sample is not an answer | Diagnostics of chains (the several chains must agree among themselves), calibration by simulation, conformal prediction |
| Track 5 Governance | Takes care of legality, privacy and trail | Personal data is a combination of fields, not a file type | Impact opinion, differential privacy, named human officer, SHA-256 |
| Track 6 Fairness | Asks who pays for the error | The chosen metric decides who pays | Audit of error rates between groups, fairness trilemma |
| Track 7 Domain | Translates problems from other areas | The parameter that does not come from the data comes from someone | Glossary, SHELF elicitation, scope term |
| Track 8 Decision | Converts the result into action | A number without a natural frequency decides nothing | Natural frequencies, HOPs, C/L tree, exit clause |
| Orchestrator | Coordinates and commands the panel | No step approves its own result | Card of the model critique and attack on the artifact before approval |

**The craft of each track, in one case.** Track 2 begins with a decision that nobody sees: **what
a row is**. One row per person, per day or per event changes the mean, the variance and the margin, and for that
reason the grain is declared and proved with a count. That agent hunts the **sentinel value**, the
absent recorded as 0, the −1 or the 9 999 that enters the sum as if it were data, and requires idempotent
ingestion. Running the load twice cannot change the number.

Track 5 recalls that personal data is not a file type: it is the **combination of fields** that decides.
Postcode, date of birth and sex, together, probably identify 87% of the people in the United
States, by the 1990 census. Differential privacy limits how much the presence of one person can
change any answer. In two databases that differ in the record of a single person, the result of
any query cannot change by more than a fixed factor. That factor is a budget, spent on each
query. The cheapest measure remains not collecting.

It falls to Track 3 to require that every model in operation declares its behaviour in three cases: input outside the
domain, missing field and low confidence. The options are to refuse, to activate a safe deterministic
fallback or to return calibrated uncertainty. A model in use degrades for different reasons, which ask for different
answers.

| Situation | What changes | The right answer |
| --- | --- | --- |
| Covariate drift | The distribution of the inputs changes, and the relation between attribute and outcome still holds | Reweight, with weights by the ratio of the densities |
| Label drift | The frequency of the outcomes changes, and the distribution of the attribute within each outcome still holds | Readjust the base rates (intercept correction) |
| Concept drift | The relation between attribute and outcome itself changes | No reweighting saves it: rebuild the model |
| Served population changed | The metric falls because the audience changed | It is not a degradation of the model: read the metric against the base rate |

The correction of label drift has a closed form. If the distribution of the attribute within each
class does not change, the corrected probability is the old one multiplied by the ratio between the new base rate
and the old one, then renormalised. With two outcomes, the correction adds a constant to the logit. It is an
**intercept correction**. The hierarchy of Van Calster and colleagues (2016) separates two levels that the
third horizon found. One of them is calibration in the mean, measured by the intercept; the other is
**weak calibration**, which requires intercept 0 and slope 1. In the third horizon, the 2023 flights passed with
the intercept correction and the Toronto prisons did not, because their error was one of slope.

Track 7 does not ask the specialist for a mean. The Sheffield protocol (SHELF) asks for the **median**,
the **quartiles** and the **plausible extremes**, fits a distribution to those quantiles and gives it back
so that the specialist confirms it, controlling the interviewer's anchor, the order of the scenarios and overconfidence.
In the research, no elicitation was done with a human specialist. The audit managed to
measure the **sensitivity to the choice of the prior**. In 33 domains, the prior of the package was replaced by the
literature prior and by a deliberately weak one, and five domains change status or action with the
literature one and two with the weak one. The conclusion is not that the simulated priors were wrong. It is that
in five of 33 domains the decision does not survive the replacement of the prior.

**An honest gap.** The architecture lacks a question that Track 2 should ask. **Why is the
datum missing?** The data contract limits the nullity but does not classify the mechanism of the
absence (MCAR, MAR or MNAR, Chapter 6), and it is that class that decides whether discarding the incomplete
cases is harmless. The audit later added an `ausencia` block to the contract, and in the three
contracts of the real horizon 1 and in the 18 tables of the second horizon the rule is **vacuous today**, since
no column has a single missing value. The lock starts to bite on the day one appears.

**System 1 and System 2.** The computer that helps thinking has two modes. **System 1** does
fast and categorical triage and closes the card with a deterministic suffix tree (same input,
same answer), emitting probabilities **trapped in the interval from 1% to 99%**. **System 2**
deliberates: tree search over the hypergraph, code synthesis, heavy inference, adversarial
stress. The rule of the interval is not aesthetics. A probability of 0% or of 100% cannot be
scored by a proper rule, never loses and never learns.

---

### Step 2 — The rule of the game

**The six-stage cycle.** A question goes through six stages, each with its output cards. The path
to the decision passes through all of them.

1. **Translation and elicitation (Track 7).** Card of the transfer, SHELF elicitation, scope term.
2. **Ingestion, grain, causality and ethics (Track 2, Track 5, Track 6).** Data contract, card of the sample, causal
   graph with the Pearl step, privacy opinion, fairness audit.
3. **Simulation, null model and frontier (Track 1, Track 4, Track 3).** Executable generating process, null model
   simulated **before** reading any pattern, inference diagnostics, partition outside time, count
   of attempts.
4. **Projection onto the five lenses (the orchestrator).** Sweep of Fluxonomy, Chrononomy, Scalonomy,
   Morphonomy and Crisonomy.
5. **Adversarial panel (the orchestrator).** Rejected goes back to the stage that failed; approved moves on.
6. **Decision and closure (Track 8).** Memorandum in natural frequencies, HOPs, C/L tree and exit clause.

The only path to the decision passes through the panel. A rejected assignment goes back to the **stage in which
it failed**, instead of going back to the beginning.

**The adversarial panel.** Three failures eliminate the assignment before any discussion of merit: temporal
leakage, a violated data contract without treatment and the absence of a named human officer. After that, each
track receives its attack, eight in total.

| Track under attack ($A_i$) | The attack | The concern the card must have |
| --- | --- | --- |
| $A_1$ Code | A new battery of properties, with a seed the author did not choose | property test |
| $A_2$ Data | A batch with corrupted records, which must be isolated | handling of missing values |
| $A_3$ Lifecycle | A batch with strong drift, live | declared drift |
| $A_4$ Inference | A pathological data set, to see whether the uncertainty denounces the inadequacy | pathologies covered |
| $A_5$ Governance | Tampering with the audit trail, which the chain verifier has to detect | intact trail |
| $A_6$ Fairness | Inverting the chosen metric and showing who would come to pay | audited subgroups |
| $A_7$ Domain | An atypical scenario of the domain, with an alternative prior | elicited prior |
| $A_8$ Decision | A crisis that violates the exit clause itself | declared exit clause |

The eight attacks appear in old records as **MB1–MB8**. The new name is $A_1$–$A_8$, and the list is one only.

**Calibration by simulation (track 4).** $\mathrm{rank}(\tilde\theta; \theta^{(1)}, \dots, \theta^{(L)}) \sim \mathrm{Uniforme}\{0, \dots, L\}$. If the posterior is calibrated, the rank of the true value among the $L$ replicates is uniform. That is how track 4 checks whether the heavy inference returns uncertainty of the right size, instead of just a number.

**The approval condition.** Each agent $A_i$ returns a **severity** on the scale from 0 to 10, and the artifact
$X$ is only approved if **all** eight reach the floor. The condition is this:

$$\mathrm{Aprovado}(X) = \bigwedge_{i=1}^{8} \Big( A_i(X) \ge \mathrm{Severidade}_{\min} \Big),\qquad
\mathrm{Severidade}_{\min} = 6.$$

The $\bigwedge$ is the logical "and". One single agent below the floor is enough for the assignment to go back. The severity is
deterministic. It comes from the corresponding concern in the card and from an explicit seed, and repeating the
card with the declared seed always returns the same number. The condition **fails** when a concern
was declared absent, and that is what makes it auditable. There is no subjective judgment hidden in the
score.

**The card of the model critique.** The orchestrator only approves after filling five items: predictive
check against replicates of the fitted model; comparison with at least one rival model; separation between
parameters identified by the data and parameters brought from outside; analysis of performativity, Goodhart and Lucas; and
defence of the premises that are undecidable by the **cost of the error**.

**An assignment with a declared gap is worth more than a uniform assignment.** The complete artifact with one
admitted weak track beats the uniform artifact that admits no gap at all. In the real horizon 1,
it was the panel that rejected the Danish fire project. The claim lost, but the rival model **did not win the right to decide**.

---

### Step 3 — Hands on

Two measurements: run the complete panel over a card with the eight concerns and over a card with concerns
removed, printing the eight severities; and check that the triage of System 1 never leaves the interval
between 1% and 99%. Code in `crates/stochology-book/src/ch13.rs`; output measured with `cargo run --release --bin labs -- --chapter 13`:

```text
== Chapter 13 :: the A1-A8 adversarial panel ==
complete card (card h3.voos): A1-code=9 A2-data=8 A3-lifecycle=10 A4-inference=9 A5-governance=8 A6-fairness=9 A7-domain=8 A8-decision=10 | lowest=8 floor=6 approved=true
card with A5 and A6 removed (card h3.voos-com-defeito): A1-code=9 A2-data=8 A3-lifecycle=10 A4-inference=9 A5-governance=2 A6-fairness=3 A7-domain=8 A8-decision=10 | lowest=2 floor=6 approved=false
without A1-code: rejecting=["A1-code"] count_rejecting=1 lowest=3
without A2-data: rejecting=["A2-data"] count_rejecting=1 lowest=2
without A3-lifecycle: rejecting=["A3-lifecycle"] count_rejecting=1 lowest=4
without A4-inference: rejecting=["A4-inference"] count_rejecting=1 lowest=3
without A5-governance: rejecting=["A5-governance"] count_rejecting=1 lowest=2
without A6-fairness: rejecting=["A6-fairness"] count_rejecting=1 lowest=3
without A7-domain: rejecting=["A7-domain"] count_rejecting=1 lowest=2
without A8-decision: rejecting=["A8-decision"] count_rejecting=1 lowest=4
sweep: cards rejected=8 of 8 | lowest global severity=2 | severities < floor per agent=[1, 1, 1, 1, 1, 1, 1, 1]
complete card with seed 7: lowest=8 approved=true
complete card with seed 2026: lowest=8 approved=true
complete card with seed 999983: lowest=8 approved=true
system 1: label=card-in-order probabilities=[0.06, 0.94] sum=1.0000 outside_[0.01;0.99]=0
```

The laboratory above uses a triage of two probabilities, enough to show the lock. The **real System 1** is
implemented in `crates/stochology-core/src/s1.rs`, with a closed vocabulary of six categories and
35 themes, a canonical suffix tree of 136 entries and the sum checked at every run. The output measured with
`cargo run --release --bin labs -- --s1`:

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

Six lines, and the contract repeats in all of them. The sum is 1,0000 and no probability falls outside
[0,01; 0,99]. The unknown input (`xyzzy plugh abracadabra`) falls into `outros` with 0,8718. That
value is the declared prior, and not a guess. The limits stay recorded. The vocabulary is closed,
the matching is lexical and does not understand negation, and this is triage, and not judgement. Judging belongs to System
2.

Four readings.

**The complete card passes, and passes with room to spare.** The eight severities stay between 8 and 10, and the smallest is
8, two points above the floor 6. The gate of the repository, in the horizon `todos` with 12 cards, also
publishes worst = 8, and changing the seed of the laboratory (7, 2026, 999 983) does not alter the verdict. The
rejection comes from a missing concern, and not from luck of the seed.

**Each concern removed brings down exactly the agent that charges for it.** In the eight lines of the sweep, the
set of rejected agents always has **one** element, and it is always the corresponding agent. Without an intact
trail $A_5$ is rejected (severity 2), without an exit clause $A_8$ is rejected (4), without a property
test $A_1$ is rejected (3). The other seven stay intact. The panel is not a mean score that
dilutes, but a logical "and" that locates the failure.

**Removing two concerns rejects the whole card.** With A5 and A6 removed, the severities fall to 2
and 3 and the card is rejected, with 8 rejections in 8 defective cards and the smallest global severity at
2. Compare with the real gate. The command `gates --horizon h3 --defect h3.voos23` returns
`G6 reprovado banca A1-A8 com piso 6 e pior 2`. The laboratory reproduces the worst = 2 by removing the
governance concern and the fairness one.

**System 1 respects the interval.** The triage returns 0,06 and 0,94, sum 1,0000, and zero values
outside between 1% and 99%. The interval exists for a practical reason. A probability of 0% would not be
scored by a proper rule nor would it be corrected by the scoreboard. The limit separates a forecaster from a
dogmatic assertion.

**Divergence declaration.** The previous edition of this book speaks of "eight tracks and an orchestrator M15". The
text of the repository (`docs/PLAN.md`) calls the orchestrator the orchestrator and the implementation
(`OrquestradorM`) as well; the label M15, like the MB labels of the panel, is historical and is declared in
`AGENTS.md` §VIII, along with the ME numbering of the script. They are the same object, and this book uses the orchestrator
because that is the name of the code. The laboratory of this edition measures the
severities from the **eight concerns of the card** (true/false flags) and not from the attacks themselves:
running the attack is the machinery of the repository, and the number the panel publishes is the one of the severity. For that reason the
values 8, 9 and 10 here reproduce the worst = 8 of the gate, and not a report of an executed attack.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarsely. The line of this chapter, from the instrument of the axis (`labs --resolution`):

```text
chapter 13 | no loss measured in this chapter | the adversarial panel judges the card, not its grain
```

**Exercises.**

1. *(Concept)* A database has one row per medical appointment, and the question is about patients. Which track must
   block the analysis, and with which test?
2. *(Concept)* Draw the attack of the panel for a credit triage model. Which crisis would violate the
   exit clause? Which group would come to pay if the metric were inverted?
3. *(Computation)* The complete card has smallest severity 8 and floor 6. What is the maximum number of agents that can
   fall to the low track (2 to 4) without rejecting the card? Justify with the definition of $\bigwedge$.
4. *(Laboratory)* Implement calibration by simulation (SBC) for a Beta-Binomial model: draw the
   parameter from the prior, simulate the data with it and run the sampler. Check that the rank of the true
   value among the posterior samples is uniform. Then introduce an error in the sampler and see the
   histogram stop being flat.
5. *(Concept)* Give one example of each of the three modes of degradation of a model in use: covariate
   drift, label drift and concept drift; give also an example of the fourth situation, which
   is not a degradation. Say the right answer for each one.
6. *(Concept)* Why must the answer of System 1 be deterministic (repeated input,
   equal probability)? Relate it to the fixed seed of Chapter 1.

**Summary.**

- Eight tracks, each linked to a way of erring, plus an orchestrator that attacks; the agents exchange
  structured cards, never free text, and nobody approves the assignment they produced.
- The cycle has six stages, and the return loop leads to the stage that failed, instead of the beginning.
- Approval is a logical "and" of the eight severities against the floor 6; each missing concern brings down the
  agent that charges for it.
- System 1 deterministic triage with probabilities between 1% and 99%; System 2 deliberates over the
  hypergraph.
- An artifact with a declared weak track is worth more than a uniform artifact without an admitted gap.

## Chapter 14. The iron contract: cards, prohibitions, invariants and gates

**Objectives.** By the end of this chapter you should know how to read and write an epistemological card, fill the
five questions of the header, state the four prohibitions and the five invariants $\Phi_1$–$\Phi_5$,
go through the funnel of four filters and say which check each automatic gate makes.

---

### Step 1 — The real world and the paradox

**The form that refuses.** Everybody has filled in a form that accepts anything. You write
"yes" in the remarks field, the system accepts, and three weeks later nobody knows who authorised what.
The opposite exists and it is boring, and that is what this chapter is about. The form here **refuses** the submission when a
mandatory field is empty.

The epistemological card is a closed list of questions. It is the **contract** of the work, and not its summary.
In every 100 cards filled in a hurry, the ones that pass without complaint are the ones that had every
field answered with something checkable. A card without the field of the generating process does not say "I do not know". It says
"nobody wrote that", and that is where the error enters.

**The question nobody asks the decider.** There is a specific way of erring that has a name and is almost never
checked, the **type III error**. In ordinary tests, the type I error is accusing someone who is innocent and the type
II error is acquitting someone who is guilty. The type III error is more banal and more frequent, and consists in **answering
brilliantly the wrong question**. The study measures effect A with impeccable precision, and the manager needed to
know about effect B.

Imagine a city hall that has to decide whether to open a second service room on the busy days. The
report answers, rigorously, the question *"what is the mean level of service of the year?"* The answer is 82%, and the
manager reads it as the answer to his question, *"should I open a second room?"*. Nobody wrote the manager's question
anywhere. In 100 hurried decisions, a respectable part dies from a right account on the
wrong question, and not from a wrong account.

**How the contract locks that.** The domain package comes to carry a mandatory field, the
`pergunta_do_decisor`, with three parts in order: **who decides**, **which action the decision changes**
(copied literally from the decision field of the package) and **the question that decision uses**,
ending in an interrogation mark. The validator is the code, which refuses generic text without who decides and rejects an action
different from the one of the package. After that, "the question the model answers is the one the decider asked"
stopped being an assumption and became a checkable sentence.

**The four things the contract forbids.** The contract of the agents begins with four absolute prohibitions, and
each one patches a known hole:

1. **Free natural language between agents.** Communication is only through structured cards; free text is
   ambiguous and invents.
2. **The solitary mean.** When the function is non-linear, the dynamics multiplicative or the tail heavy, it is
   forbidden to report only the mean: one reports the result of the mean case, the simulated mean of the
   results, the median, the quantiles and the cumulative curve.
3. **Confusing learning with performativity.** Small prediction error does not prove that the model is
   true; one runs the model with and without the action.
4. **Dogmatic extremes and omission of trials.** No forecast declares 0% or 100%; every forecast is
   scored by a proper rule outside the fit; and one declares how many specifications were tried.

---

### Step 2 — The rule of the game

**The card and the five questions of the header.** Every payload that circulates opens with five mandatory
questions.

| Field | Question |
| --- | --- |
| `processo_gerador` | Which hidden mechanism generates the data, written in a simulable way, and in which layer can the claim fail? |
| `nao_linearidade` | Is the result a non-linear function of something that varies? Compare $f(\mathbb{E}[X])$ with $\mathbb{E}[f(X)]$; is the dynamics additive or multiplicative? |
| `unidade_e_margem` | What is the de facto independent unit, and was the margin computed with it? |
| `fisica_do_ruido` | What noise is this (gaussian or with jumps)? Are there absorbing states? Is the agent passive or does it deform the world? |
| `escada_causal` | At which step of Pearl is the statement, with which identification strategy and which untestable premise? |

The real header of $H_0$ in a real domain, abbreviated:

```yaml
alegacao: >
  "Escalando cada meia hora pelo Erlang C sobre a taxa média do histórico, a central atende
  82% das chamadas em até 20 s, com espera média de 10 s, todo dia."
cabecalho:
  nao_linearidade: >
    O nível de serviço do Erlang C é côncavo e despenca perto da saturação: aplicada à taxa
    real de cada dia do cofre, a própria fórmula da alegação dá em média 57%, não 82%.
  unidade_e_margem: >
    A unidade independente é o DIA: as chamadas de um dia compartilham a taxa e a fila.
    n_efetivo = 81 dias de cofre, não 2,65 milhões de chamadas.
  fisica_do_ruido: >
    A demanda diária NÃO é Poisson: dispersão Var/média = 273 (Poisson: 1).
  escada_causal: >
    Degrau 2 (intervenção): as regras de escala são do(escala) simuladas sobre a demanda real.
    Premissa não testável: a demanda não reage à escala.
```

Each module of the course adds questions to the five of the header. The card of the evidence asks for the base rate, the
two likelihoods and the posterior computed in two ways, in odds and in a tree of 10 000 cases, which
have to coincide. The card of the causal graph begins with the step. The card of the model critique closes the series and says in which
item each previous card enters.

**The physics of noise, in detail.** The fourth field asks for three classifications of the noise.

| Question | Options | Why it matters |
| --- | --- | --- |
| What shape does the noise have? | Gaussian (continuous) or with jumps | A gaussian model does not have 19 October 1987 in its hypothesis space |
| How does the noise integrate? | Itô or Stratonovich | When the noise depends on the state, the two readings give different drifts |
| Are there absorbing states? | Yes (ruin, death, bankruptcy) or no | In an absorbing state, the ensemble mean stops describing any trajectory |

The second question looks technical and is not. In a multiplicative growth with proportional noise,
$dX = \mu X\,dt + \sigma X\,dW$, read in the manner of Itô, the expectation grows at rate $\mu$, but the
typical trajectory grows at rate

$$\frac{d}{dt}\,\mathbb{E}[\ln X_t] = \mu - \tfrac{1}{2}\sigma^2.$$

It is the volatility drag of leveraged Dow Jones, and it is Jensen again. Read in the manner of Stratonovich, the
same symbol describes another process, in which the term $-\sigma^2/2$ does not appear. The choice depends on the physics.
Idealised white noise of a discrete system asks for Itô; the limit of a real noise with short correlation asks for
Stratonovich. The card also asks for a test with a smaller time step. If the result changes when $\Delta t$
shrinks, the simulation was measuring the integrator instead of the World. That test does **not** decide between Itô and
Stratonovich, because the Euler scheme converges to the Itô reading, and the Stratonovich one asks for another integrator.

Noise also makes the system jump between stable states. In a system with two wells, the mean time until the jump
follows the Kramers formula and grows exponentially with the height of the barrier $\Delta U$ with respect to the
temperature $T$:

$$\tau \approx \frac{2\pi}{\sqrt{U''(x_{\min})\,\lvert U''(x_{\max})\rvert}}\; e^{\Delta U/T}.$$

For high barriers, that time has almost no memory. Whoever waited a long time is not closer to the jump.
Looked at only in the jumps, however, the noise looks like a push. In 591 jumps simulated in the course, the noise just
before the jump pointed in its direction in 99% of the cases, with no external force at all. Looking for the cause of a
jump is a selection error; looking for the cause of a change in the frequency of the jumps is not.

**The five invariants.** Nothing enters the graph without passing the invariants, which work as database
locks. The first four are required from the beginning; the fifth entered with the axis of question resolution
(Chapter 21) and bites in the case in which the misalignment is most expensive.

| Invariant | What it requires | What it blocks |
| --- | --- | --- |
| $\Phi_1$ Simulability | Every claim becomes code with a declared seed and property tests | An argument only in prose |
| $\Phi_2$ Margin and unit | Every number comes with a margin computed on the de facto independent unit | Pseudoreplication, duplicates, sentinel values |
| $\Phi_3$ Layers and severe test | Audit the filter of the sample, draw the causal graph and confront the hypothesis with a simulated null model | Pattern read before the null, collider, Simpson |
| $\Phi_4$ Ecological scope | Judge the decision in the environment in which it will operate, with a scope term and an objective exit clause | A recommendation without a validity date |
| $\Phi_5$ Resolution of the question | The declared action has to be at the same grain as the independent unit: with the datum at the day or coarser, aggregate resolution (R0) is rejected | An action by day with an account at the aggregate. A unit finer than the day stays outside this lock, and that is declared |

**The funnel of four filters.** A system that simulates every hypothesis it meets collapses from cost and from
inflation of false discoveries. The funnel, the pruning by value of information and the counter of attempts live in
Rust, in `crates/stochology-core/src/funil.rs`, with 17 tests. The four filters refuse by the header (without a
question, without a generating process, without a unit or null, without an action with an exit). The pruning cuts when the quota of the value
of information cannot cross the threshold: with $C/L = 0{,}1$ the quota is worth 0,0090 at $p = 0{,}001$ and the test of
0,05 is pruned, while at the threshold $p = 0{,}10$ the quota is 0,9000. Finally, the counter closes the account in the form
`K = 3 tentativas (F1 1, F2 0, F3 0, F4 1); aprovadas 1; versao final: v3`. The declared limit is that the quota
is the expected value of perfect information, exact in the model of two acts and two states. Each front organises
itself as a funnel, and in it most of the weak hypotheses fall before touching heavy simulation.

1. **Value of information.** The hypothesis that would not change any decision even with a perfect answer is cut. The
   standard log is "pruned by Crisonomy: curiosity without operational use".
2. **Topological pruning and unit.** By d-separation, every node that stays outside the Markov blanket of the
   variable of interest is cut, including the colliders that, if conditioned on, would create Berkson's bias. The claim
   that the data do not identify is also cut, or the one whose effective size, counted on the independent unit
   by $F_2$, stays too small to conclude. The data contract also checks an `ausencia` block for the
   mechanism of the absence: MCAR, MAR, MNAR or not declared, with the columns that the classification covers and the analysis
   of sensitivity required when the absence may depend on the value of the variable. Absence without classification
   is refused; in the three contracts of the real $H_1$ and in the 18 tables of $H_{2g}$ the rule is vacuous today, and it starts
   to bite on the day one appears.
3. **Severe test against the null.** If the alleged pattern already appears in the null model, or in a reference with a single
   parameter such as the base rate, the research closes there. When the simulation is expensive, $F_4$ makes that test
   first in a cheap emulator.
4. **Heavy inference, lenses and panel.** Only the remainder receives the expensive treatment.

**Worked case: the identifiability of $R_0$.** At the beginning of an epidemic, the cases grow like
$\exp((\beta - \gamma)t)$, and the data determine the **difference** $\beta - \gamma$, not the ratio
$R_0 = \beta/\gamma$. With 25 days of simulated cases, in which the truth is $R_0 = 2$, the compatible values
go from **1,38 to 5,90**, and the predicted final size goes from **50% to 100%** of the population. With 45 days, $R_0$
stays between **2,00 and 2,24**, but by then the curve has already doubled. Taking the duration of the infection from other studies narrows
$R_0$ to **1,81 to 2,11**. That is not a defect, as long as one declares that the margin became the one of the source.

**The automatic gates.** No delivery is consolidated without passing the gates on a clean machine. The
agents and the $\Phi$ have already been formalised; the gates **repeat the work and compare**. A repeated input has
to produce an identical output, and a planted defect has to be detected. The binary
`gates` implements seven.

| Gate | Track | What it verifies |
| --- | --- | --- |
| G1 | $\mathcal{M}$ | Determinism: the Merkle root of the second run is identical to the one of the first |
| G2 | $F_1$ | Fixed seed declared in every card |
| G3 | $F_1$ | Declared scope: only a simulable artifact |
| G4 | $F_1$/$F_7$ | Invariants $\Phi_1$–$\Phi_4$ and the type III lock |
| G5 | $F_5$ | SHA-256 trail intact **and** tampering detected |
| G6 | $\mathcal{M}$ | Adversarial panel $A_1$–$A_8$ with the floor |
| G7 | $F_8$ | Declared exit clause and $A_8$ above the floor |

Two details make the set worth more than the sum. G5 does not verify only that the chain is intact. It
**tampers on purpose** and requires the verification to fail with the tampered chain. G1, in turn,
compares two runs of the declared horizon. A gate that never fails is not a gate (Chapter 15).

---

### Step 3 — Hands on

The laboratory validates a complete card and a card with one planted defect at a time, printing the failures by
gate, in the spirit of `gates --defect`. Code in `crates/stochology-book/src/ch14.rs`; output measured with
`cargo run --release --bin labs -- --chapter 14`:

```text
== Chapter 14 :: the iron contract ==
card h0.posto-saude: layer=L1 uncertainty=aleatory header=5 fields | failures=[] total=0
declared taxonomy: layers=["L1", "L2", "L3"] uncertainties=["aleatory", "epistemic", "knightian", "not-applicable"]
defect NoSeed: failures=1 gates=Phi1-simulability detail="missing fixed seed or simulable artifact"
defect NoDefinition: failures=1 gates=Phi3-layers-and-null-model detail="no null model or no one-line definition"
defect NoMarginNoUnit: failures=1 gates=Phi2-independent-unit detail="margin without a declared independent unit"
defect NoNullModel: failures=1 gates=Phi3-layers-and-null-model detail="no null model or no one-line definition"
defect NoScopeNoPayerNoExit: failures=1 gates=Phi4-scope-and-exit detail="ecological scope, who pays for the error or exit clause missing"
defect QuestionWithoutMark: failures=1 gates=type-III-decider-question detail="decider_question must end in ? and cite the action of the package"
defect QuestionWithoutAction: failures=1 gates=type-III-decider-question detail="decider_question must end in ? and cite the action of the package"
intact graph: validation=[]
graph with a ghost edge: validation=["edge to absent node: L2_erlang -> L2_ghost"]
graph with a deductive cycle: validation=["cycle in E_tree: Some(\"L1_queue\")"]
graph with a one-member hyperedge: validation=["hyperedge with fewer than two members: H2"]
property test (10.000 simulated cards): violations=0 margin_min=0.00617 margin_max=0.09798
same seed, two validators: runs that diverge in 0 of 32
```

Four readings.

**The complete card passes without a single failure.** Five fields of the header in the required order, seed
declared, simulable artifact, margin with independent unit, null model, ecological scope, who pays
for the error, exit clause and the question of the decider citing the action. The total is zero. The type III
lock is **literal**, and the question has to end in an interrogation mark **and** contain the action of the package.

**Each defect falls at the right gate.** Seven planted defects, seven rejections, one per gate, without
contamination. Without a seed it falls at $\Phi_1$; without a margin or unit it falls at $\Phi_2$; without a definition or without a
null model it falls at $\Phi_3$; without scope, without who pays or without an exit clause it falls at $\Phi_4$. The two
defects of the question of the decider both fall at the type III lock, but for different reasons. One loses the
interrogation mark, the other loses the action. **The message is identical in the two cases.** The validator
states the requirement of the rule, without saying which part was missing. It is a small detail that exercise 4 asks
to improve.

**The structural validator crosses the chapters.** The three refusals of Chapter 12 reappear here:
an edge to a ghost node, a deductive cycle and a hyperedge with a single member. That is on purpose. A lock that only
exists in one chapter is a lock that nobody uses.

**The property test and the seed.** Ten thousand simulated cards with margin $1{,}96\sqrt{p(1-p)/n}$ for
$n$ between 100 and 999 and $p$ between 0,01 and 0,99 give **zero violations**, with margins between 0,00617 and 0,09798. The
confidence interval is never negative nor greater than a half. The two validators run over the declared
seed diverge in **0 of 32** runs, the reproducibility that Chapter 1 requires.

**The pipeline by horizon.** In the repository each horizon has its own pipeline. $H_0$ applies $\Phi_1$–$\Phi_4$, the runs $H_{2b}$ to $H_{2g}$ apply five criteria and $H_3$ applies seven. Each `portoes.py` repeats the run, compares the cryptographic digests and exits with 0 or 1; the seven of this chapter
are the ones of the `gates` binary. (The `portoes.py` of each front was retired on 2026-10-16 and lives in the commit
`1bfec905`; the gate stays in the binary.)

**Divergence declaration.** Two accounts of this chapter diverge between the sources, and both stay
recorded. (1) The previous book describes **five** automatic gates — contracts and seeds; declarative schema
and idempotence; convergence of the chains; temporal cutoff; governance, fairness and exit —, while
the `gates` binary implements **seven** (G1 to G7). The difference is one of grouping: gate 3 of the old
book ("convergence of the chains") disappears from the binary, and gate 5 splits into G5, G6 and G7. The two texts
count the same pipeline in different ways; no list was erased. (2) The previous book and the plan of the
repository say that "in $H_0$ the pipeline applies $\Phi_1$–$\Phi_4$", and this laboratory measures exactly
that: a complete card generates zero invariant failures, and each defect generates one. The number "failures = 0" of
gate G4, in the horizon `todos`, reproduces here.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarsely. The line of this chapter, from the instrument of the axis (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Fill the five questions of the header for the claim "students who study with music
   get higher grades".
2. *(Concept)* Which filter of the funnel must cut the question "what is the favourite colour of the clients who buy
   insurance?" in an insurer that is not going to change the product?
3. *(Computation)* Why do 25 days of exponential growth identify $\beta - \gamma$ and not $\beta/\gamma$?
   Show that two pairs $(\beta, \gamma)$ with equal differences generate identical initial curves.
4. *(Laboratory)* Improve the message of the type III lock to distinguish "the interrogation mark is missing"
   from "the question does not cite the action". Make the two defects fall into different messages and reproduce the
   table.
5. *(Laboratory)* Write a property test that verifies that the simulated wait of a queue is always
   non-negative and does not decrease when the service time increases, keeping the arrivals fixed.
6. *(Concept)* Explain, in three sentences, why G5 is stronger than a test that only checks whether the chain
   is intact, since it tampers on purpose with the chain it produced.

**Summary.**

- The card is a closed list of questions; five are mandatory in every payload, and the field
  `pergunta_do_decisor` locks the type III error.
- The four prohibitions and the five invariants are locks, and not recommendations; $\Phi_5$ is the newest and the narrowest.
- The funnel cuts cheaply before spending expensively: value of information, topological and unit pruning, severe test
  against the null, and only then heavy inference and panel.
- The `gates` binary implements seven gates, with exit code 0 or 1, and a gate that never
  fails is not a gate.

## Chapter 15. The vault, the cryptographic seal and the audit against itself

**Objectives.** By the end of this chapter you should know how to explain why pre-registration with a SHA-256 hash is the
only seal against adjustment after seeing the result, what the counter of attempts $K$ measures, why the
vault outside time opens a single time, how the forecast diary and the signed opinions work, and
why the auditor being a stakeholder is a limit that no signature resolves.

---

### Step 1 — The real world and the paradox

**Painting the target after the shot.** A marksman fires ten shots at the wall of a barn. Then he looks at the
holes, picks up a brush and draws a target around the tightest group. Then he shows the target and says: "five
hits in the centre". The count of hits is right, and the target is fake. The only thing that separates science
from this marksman is **when** the target was drawn.

There is no way to undo that by looking at the result. The seal consists in writing and **freezing** the
target before the shot, in a way that nobody can alter afterwards without it being noticed. In every 100
published results, the ones that look most like cheating are precisely the clean ones, adjusted a little bit
after seeing the data. The difference between "I adjusted the model" and "I adjusted the model after seeing the result"
is not in the model, but in the **order of the facts**.

**The seal is a number.** How to freeze? By publishing the **hash** of the file. The hash is a fingerprint. It takes
the whole content, from the first to the last character, and returns a line of 64 characters. Changing one comma
changes the whole fingerprint. Publishing the hash before the shot is like sealing the envelope at a notary. You do not have to
show the content now, but afterwards nobody opens the envelope without it showing.

**The vault.** Imagine a bank that receives a payment slip from each client. One day the bank decides to build a
"vault outside time": a folder with the data of the next three months, which no analyst may open. The
model is built with the past; when it is finished, someone opens the vault and runs the model against data it
never saw. Opening the vault costs the one thing it had: surprise. In the research, when a vault has already been
opened and the model needs a correction, the correction is only worth anything tested in a **new vault**, which nobody saw. There were
three vaults until the calibration of the Toronto prisons was right.

**The forecast diary.** Every probabilistic forecast issued stays recorded in a continuous ledger, with
the source, the filter, whether the system can influence the outcome, how many factors the justification has and the
step of Pearl. And with a limit: **the forecast is never 0% nor 100%**, but something between "one in a hundred" and
"ninety-nine in a hundred". Without that limit, whoever bets on certainty never loses points and never learns; with the
limit, each forecaster accumulates a record that can be charged.

---

### Step 2 — The rule of the game

**The risk of scale is epistemological.** When a system tests hundreds of hypotheses and specifications in
parallel, the greatest risk is not technical; it is overfitting at industrial scale, the garden of forking
paths walked by indefatigable machines. The plan imposes three mechanisms of containment.

**Pre-registration with hash and counter of attempts.** Before any agent accesses the test partition, the
hypothesis is frozen with a hash. Every change of criterion, of test or of data is written in a declaration
file, with its SHA-256 recorded **before** running. No rule was changed after seeing the result
of the run to which it applied, but each new run was written knowing about the previous ones, and the corrections made
after seeing results stayed recorded and **count in $K$**.

The system keeps a counter $K$ of specifications tried per question. Every failed attempt stays
recorded, and the final report is obliged to declare $K$. The plan goes further and asks that $K$ penalise
complexity, by the minimum description length (MDL) or by the Occam factor, together with the correction for
multiple comparisons. In the research, $K$ was declared, but **none of those penalties was applied**. It was
because of that counter that the respecification of the $H_2$ domains stopped at $K = 3$, and the next step was to seek
new data. The counter per domain does not record, however, that the gate was tried in seven runs, and
no correction for that was applied. The background audit after that step measured the **honest** $K$ of the
$H_2$ gate: **38 distinct tests** (with the band from 32 to 72 according to the counting criterion), and with
Bonferroni $H_{2g}$ keeps the four areas of then, with the sets $\hat S$ shrinking.

**The vault outside time.** The future temporal partition stays under the exclusive custody of the lifecycle agent
($F_3$). No modelling agent reads it during development; it is opened a single time to score
calibration and verify the absence of leakage. The rule is hard and has a consequence. When a vault has already been
opened and the model needs a correction, the correction is only worth anything tested in a new vault. $H_3$ did that three
times: with the 2023 flights, with the Toronto prisons of 2020–2021 and with the traffic stops of Rhode
Island.

**The forecast diary and the bus.** Each agent that issues probabilistic forecasts records them in a
continuous ledger, restricted to the interval $[1\%, 99\%]$. Agents whose Brier score loses to the
base rate, or whose Kelly capital collapses, have their weight reduced by the orchestrator. The card bus is
**chained by hash**: each message carries the hash of the previous one, and the trail cannot be rewritten without breaking
the chain. An implementation lesson came early. In $H_1$, the score of the diary added into a single one the forecaster
that appeared in different projects, and the error was corrected and declared in the README; the numbers of $H_1$
published after that already come from the corrected version.

**Risk opinions and the named officer.** The governance agent ($F_5$) issues a risk opinion for
each project evaluated, including the ones that do not reach the decision. In the research there were **21**: 18 from $H_2$ and 3 from the
real $H_1$. Four of them involve decisions about people: Law (rule of labour indemnity),
Education (the STAR classes), Health (the RAND insurance experiment) and the LaLonde training programme.

The 21 opinions were signed on 15 October 2026, after reading a summary of them that highlighted the
four that involve people. The signature stays in a separate record, in which each opinion is identified by the
SHA-256 of the file at the moment of signing. The result files **were not rewritten**, so as not to change
the outputs that the clean reproduction compares. The opinion of Law carries a factual caveat. The recommendation to
adopt a 10% cutoff comes from a decision and a cost **invented** in $F_1$, and it is not policy advice.

The signatures are nominal. Each opinion carries the short name of the officer (**sil**) and the cryptographic digest of the file, in `governance/ASSINATURAS.md`.

**The limit: the auditor is a stakeholder.** The opinions were signed by whoever runs the programme. An
audit authorised by whoever is audited has limited value. The named officer answers the question that
an audit has to answer (who decided to use it, who approved it, who watches the scoreboard, who switches it off), but
naming an officer **does not make the audit independent**. In the direction that an audit requires, the programme
recorded the points that the audit **could not look at** and which affected groups were heard. The record
declares that, in the seven decisions that involve people (cutting the benefit by 10%, reducing the STAR
classes, replacing free care with copayment, comparing the NSW training with CPS controls and the
three review marks of the prison streams), **none of the seven affected groups was heard**. There is no record in the
repository of consultation, interview, panel or consent. The cheapest and
most obvious step of the list is missing as well: the elicitation with human specialists.

**The background audit: what was assumed and what was measured.** Governance also serves to mark the points
**not** measured, and to measure them afterwards. The research turns against itself in a background audit. Ten
points that the text sustained and that had never been put to the test became fronts of work, to which were
added, later, the extensions (rare event, health, card and contract) and the front of human elicitation, which
waits for people. Each front repeats the rule of the horizons: the declaration with hash comes **before** running, the
result is reported with the limits, and a change of criterion after seeing the result enters as a new
declaration and counts in the number of attempts.

| Point | The assumption | What was measured | Result |
| --- | --- | --- | --- |
| 1. Central thesis | The architecture catches errors that common practice lets through | Blind pilot of 12 cases, two arms | Tie at the ceiling: 8 of 8 defects in both, 0 false alarm in the 4 clean cases — the thesis does not stay supported |
| 2. Independence | The approved outputs do not depend on who wrote them | Four results reimplemented only from the specification | All agree; zero errors in the original; several choices that the specification does not fix coincide with those of the author, three of them only in STAR (weak evidence) |
| 3 and 4. Multiplicity | The $H_2$ gate was tried few times, and nothing was adjusted afterwards | Inventory of the attempts and of the vault openings | $K = 38$ distinct tests (32 to 72); with Bonferroni $H_{2g}$ keeps the 4 areas of then and the sets shrink |
| 5. Priors | The literature priors were not checked with specialists; the control run had no declared tolerance | Three runs in 33 domains (package, literature and weak) and the equivalence band of 0,05 sd | Five domains change status or action with the literature one and two with the weak one; the verdict stops being conditional — maximum deviation of **0,0071 sd** and **33 of 33** domains identical |
| 6. Performative loop | Whoever abandons the queue calls back with probability 0,6 | Quasi-experimental design in a real support desk, 1999 | **0,145** [0,132; 0,157] — four times less than the declared one |
| 7. Dependent rows | ICP assumes independent rows, and there are 16 instants per volunteer and 80 schools | Clustered Wald and block permutation; replication of Genetics with 40 new genes and null rate by permutation ($B = 999$) | Pharmacology survives intact, Education drops out; Genetics replicates in one validated gene, with $p = 0{,}001$ of the null rate |
| 8. Calibration | The tolerance of the ECE can be fixed at 0,03 | Null simulated with the same forecasts and the same $n$ | None of the six binary vaults of $H_3$ passes the rule that depends on $n$; by the secondary rule, the picture does not change |
| 9 and 10. Scale and data | The funnel scales and the graph measures knowledge | 329 domains in batch; two new health domains | Two pruned in the severe test; 327 up to the end; 26 634 nodes, of which 6 494 are concept nodes |
| Rare event | A forecaster of a rare event is useful for deciding when it has good calibration | Brier decomposition in three vaults: indomethacin, COVID and Rhode Island | No pathway wins in the three: only in Rhode Island does RES − REL stay above zero (+2,74643e-4 in the logistic, +3,11655e-4 in the ranking), and the Brier skill stays at +0,018629 and +0,017952, below the declared +0,05; in COVID the AUC is 0,7685 and the skill is −0,020853 |
| Health | The circle that Lab 0 opened closes with real health data, in an open mirror | PhysioNet/CinC Challenge 2019 (early sepsis) in the PhysioNet mirror; the funnel of $H_{2g}$ ran end to end without a single line changed, with the vault in the second hospital system | The effect exists and is precise ($\beta = 0{,}0388$ in log-odds ratio per breath per minute, 90% CI 0,0257 to 0,0523), but the invariance does not: 0 of 512 accepted subsets, $\hat S = \emptyset$, and G3 fails; the skill in the vault is +0,00568 and the recorded decision rate is 8,4% in the admissions of one system and 3,9% in those of the other |
| Card and contract | The question the model answers is the one the decider asked; the data contract does not say why the datum is missing | Mandatory field `pergunta_do_decisor` in the domain package and `ausencia` block in the contract of $F_2$ | The two fields enter validated in the code; in the three contracts of the real $H_1$ and in the 18 tables of $H_{2g}$ the rule of the absence is vacuous today and no published number changes |
| Human elicitation (SHELF) | The simulated prior would be replaced by a panel of specialists | Protocol of the panel and six written forms; criterion of $K = 2$ fixed | Nothing was elicited: no interview, no corrected forecast and no signed decision line — the front has only the instrument |

**Retirement note.** The artifacts of the rare event, health and scale extensions left the tree on 2026-10-16 for not
sustaining a central result of the theory; the numbers above stay recorded here, and the material comes back with `git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros`.

Three of those results tighten the reach authorised by the previous chapters. The loop of 0,6 that $H_3$
corrected is about four times the measurement of a real case. The tolerance of 0,03 of the calibration error is greater
than the noise in all the large vaults, and none of them passes the rule that depends on $n$. And the transfer gate of
$H_2$ depends on which version of each choice counts. The area of Education does not survive the
correction of clustering, and what sustains the third area now is the replicated Genetics. **No published number fell because of an error of account; the audit narrows the authorised reach of those numbers.**

The open questions have names. The elicitation with real human specialists, because the audit
measured the sensitivity to the priors and that does not replace a panel. The protocol and the six forms are
written, with the criterion of $K = 2$ ready, and **no interview was done**. The null rate of the validation by
intervention of Genetics, which decides with a single gene in 27, came out at $p = 0{,}001$, the floor of 999
draws; the resolution stays open there, and not the $p$. And the service time and the patience of the
clients of the support desk, which the database does not bring and which remain assumed.

The limit of the auditor being a stakeholder also stays, and the audit did not resolve it. One of the steps it
proposes is that whoever signed rereads the 21 signed opinions in full, one by one, and confirms or withdraws
the signature recording that they read the whole text. The form of rereading of the 21 lines exists, with the
hashes checked one by one (**21 of 21 match**), and it is **blank on purpose**. Nothing was reread, nothing was
confirmed and nothing was signed there. While that does not happen, the signature is worth what it is: the
declaration of whoever runs the programme that they read a summary.

---

### Step 3 — Hands on

Two tasks: build a ledger chained by SHA-256 and prove that changing **one character** in the past
brings the chain down; and compute the Merkle root of a batch of cards. The code brings SHA-256 written by hand,
without dependencies, and checks the implementation against the known vectors. Code in `crates/stochology-book/src/ch15.rs`;
output measured with `cargo run --release --bin labs -- --chapter 15`:

```text
== Chapter 15 :: the vault, the seal and the audit ==
sha256("")=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
sha256("abc")=ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
chain: events=5 intact=true merkle_root=92f2ab76eb6485ab
  event 0 card=h3.voos content=ECE=0.0048 hash=902e82d6ca6cf280
  event 1 card=h3.bolsa content=mediana=2.1x hash=a9c54b2c488f0397
  event 2 card=h3.energia content=canario=ok hash=fbbcfbdbd5970a6c
  event 3 card=h3.prisoes content=ECE=0.036 hash=4248815bc8f679d9
  event 4 card=h3.ri content=ECE=0.0048 habilidade=0.019 hash=8059d78760a22282
tampering in event 2 (canario=ok -> canario=nok, event hash recalculated): intact=false root_changed=true
tampering + rewrite of 3 following events: intact=true but it changes the published root (true)
one extra character in each event: 5 of 5 alterations invalidate the chain
batch of cards: n=7 merkle_root=17679ce182647dbb
swapping one card of the batch changes the root in 7 of 7 cases
```

Four readings.

**The implementation is right, and that is the basis of everything.** The two known vectors of SHA-256 match
character by character. The empty string gives `e3b0c442...b855` and `abc` gives `ba7816bf...15ad`. Without that
check, all the rest of the trail would be a number invented with the appearance of security.

**One character in the past brings the chain down.** The chain of five events is intact. Changing
`canario=ok` to `canario=nok` in event 2 and **recomputing the hash of that event** (the most
obvious attack, in which content and hash agree with each other), the verification fails. Event 3 keeps the old hash of
event 2 as the "previous" one, and the incompatibility appears. Of the five alterations tested, one per event,
**5 of 5** invalidate the chain.

**The chain alone is not enough: the root is the proof.** If the attacker also rewrites the three following events,
the chain closes locally again (the verification passes), but the **published root** changes. That line is the
heart of the chapter. The trail becomes inviolable by the hash published **before**, and not by the chain. Whoever
keeps only the chain keeps a document that can be rewritten in its entirety; whoever keeps the root keeps the
seal.

**The Merkle root always changes.** The batch of seven cards has root `17679ce1...`. Swapping any one of the
seven for an altered version, the root changes in **7 of 7** cases. That is what allows sealing a whole batch with
64 characters instead of 64 per file.

**Divergence declaration.** Two points stay recorded. (1) The plan of the repository publishes, for the
horizon `todos`, the Merkle root `9716e28a87bec9cf...` over the 12 cards of the four
horizons, and a hash of the report `08e7c8f0...`; this laboratory uses a **own** chain of
five textbook events, and for that reason the root measured here is `92f2ab76...`, not that one. The two
accounts are correct for their contents; comparing the two only makes sense if the input is the same, and it is
exactly in that way that the clean reproduction of G1 works. (2) The previous book says that the first run of the
support desk hung because the Erlang C written with factorials overflowed at large loads; the swap for the stable
recursion of Erlang B only affects the decision, not the forecasts, but it changes the hash of the module and **counts in $K$ (3)**.
No old value was erased.

**The grain of this question.** Every chapter has a decision with grain, and the R axis measures what it costs to ask
coarsely. The line of this chapter, from the instrument of the axis (`labs --resolution`):

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

**Exercises.**

1. *(Concept)* Why does opening the vault twice destroy its value? Relate it to Mayo's severity,
   that is, to the requirement that a test could have given another result.
2. *(Concept)* A colleague proposes rewriting the result files with the signature inside. Which property
   of the clean reproduction would that break?
3. *(Computation)* The counter of the $H_2$ gate stopped at $K = 3$ per domain; the background audit measured $K = 38$
   distinct tests in the set. Explain, in three sentences, why the two numbers coexist without either of them
   being wrong.
4. *(Laboratory)* Extend the laboratory to a bus in which each message carries the hash of the previous one
   **and** the index, and write a verifier that detects not only the alteration of content but also the exchange of
   order of two messages.
5. *(Laboratory)* Compute the Merkle root of the batch of seven cards down to the intermediate roots and confirm
   how many hashes are recomputed when only the last card changes.
6. *(Concept)* Propose a procedure to make the audit of the four opinions that involve people
   less dependent on whoever runs the programme.

**Summary.**

- Pre-registration with a hash is the only seal against adjustment after seeing the result. The hypothesis is frozen
  before, and the published hash is the seal.
- The counter $K$ makes the attempts visible; the background audit measured $K = 38$ at the $H_2$ gate, against
  the $K = 3$ per domain.
- The vault outside time opens only once; corrections ask for a new vault, and there were three until the calibration was right.
- The diary scores each forecaster with a proper rule, inside $[1\%, 99\%]$, and weighs the agents by the scoreboard; the
  chain of cards is chained by SHA-256.
- Signed opinions name the officer of record, but they do not make the audit independent. None of the seven affected
  groups was heard, and the form of rereading of the 21 opinions is blank on purpose.

# Part V. The machine in the real world

## Chapter 16. Horizon H0: the health post and the 2.65 million calls of the bank

**Objectives.** By the end of this chapter you must know why the same mean demand can produce two very
different waits, how much the convexity of the queue charges whoever decides, why the independent unit of a margin
can be the patient or the day and what the industry formula promised to a call centre that it did not
deliver.

---

### Step 1 — The real world and the paradox

**The health post.** In a post with a single nurse, the consultation takes on average 5 minutes per patient, and the account
closes at 12 consultations per hour. Demand is what oscillates. On half of the days 5 patients arrive per hour, and on
the other half 11 arrive. The mean is 8, and 8 is less than 12, so the queue does not grow without end.

The manager looks at the spreadsheet and writes the usual sentence: *"mean demand is 8 and the post serves 12; the wait is about
10 minutes and there is no reason to hire"*. The arithmetic behind that sentence is right, and it is worth 10 minutes, but only in a
world in which exactly 8 patients arrive per hour, every hour. The problem is that this world is not the world.

Simulated with the same seed, the two worlds (2,000 days of 10 hours each, one attendant, seed 1909), the
difference shows up whole.

| World (patients per hour) | Mean wait per patient | The typical patient waits | 90 in 100 wait less than | In every 100 patients, more than 30 min |
| --- | --- | --- | --- | --- |
| Always 8, every day | 9.3 min | 3.8 min | 26.7 min | 8 |
| Half of the days 5, half 11 | 20.7 min | 9.3 min | 59.3 min | 25 |

No mean changed. It is 8 in both cases. What changed was the world, and the wait is **curved** in demand. Whoever arrives on a
calm day waits little, but whoever arrives on a surge day waits a lot, and the gain of the calm day does not compensate the
damage of the full day. The surge day accounts for about 70 in every 100 patients (it is 110 patients against 50),
so that the mean per patient weights precisely the bad days.

**The median trap.** Read the second row of the table again. In the world that oscillates, **the typical patient
waits 9.3 minutes**. It is the same number that the claim promised as a mean. Whoever looks at the "typical patient" sees the
promise kept; the whole difference is in the tail, where 25 in every 100 patients pass half an hour of wait.

**Reinforcement only on surge days.** With two nurses on the
days of 11 patients and one on the days of 5, the mean wait falls to 2.5 minutes and fewer than 1 patient in every 100
passes 30 minutes. The slack does not need to be larger; it needs to be where the risk is.

**The hidden question is how many patients a margin is worth.** Every margin of error needs a unit. If the
post measures the mean wait and treats each patient as an independent observation, the margin stays around
±0.07 minute; if it treats each *day* as the unit, it stays at ±0.51 minute, seven times larger. Which of the two holds
is not a matter of taste. It is the world that decides. The test is direct. Take 4,000 days of the claim world, split them
at random into two halves and measure the difference of mean wait between them. If the patients were independent, that
difference should walk around 0.047 minute. It walks at 0.216, and the prediction by the *day* unit, 0.227, hits
the nail. The patients of the same day share the same demand, and sharing is not being independent.

**The real case is the call centre of a bank.** The post was an assumption; the centre is observation. It is 2.65 million
real calls of a North American bank call centre, counted every 5 minutes, over 164 working days of
2003. The schedule is built on the 83 training days (March to June), with a peak of 234 attendants and 2,264
agent-hours per day, and applied to the 81 vault days (July to October). The claim is the standard practice of the
industry: *staffing each half hour by the Erlang C formula on the mean rate of the history guarantees 82% of the calls
answered within 20 seconds, every day.*

The dataset showed this.

- **Demand is not Poisson.** Between days, the sd is about 2,900 calls, where the Poisson model would predict
  180; the ratio variance/mean is 273. Part of that is calendar (Monday is +15%), and the residual after the day
  of the week still has a coefficient of variation of 6%.
- **Jensen lives inside the formula itself.** Applied to the real rate of each day, the Erlang C of the claim gives 57% on
  average, and not 82%. The service level is concave in load.
- **The unit is the day.** The halves null gives an sd of 0.0474, against 0.0470 predicted by the day and 0.0033 predicted
  by the call.
- **Severe test.** In 400 days simulated in the world that the claim describes, none fell below the target. In the real
  world, about one in every four days did.
- **Out of time.** The conformal 90% interval for the total calls of the day, built on the mean of the day of the
  week, covered 86% of the vault days, and the drift between training and vault is mild (KS 0.19, p = 0.08).

The decision is also measured in the vault, against the real arrivals:

| Staffing rule | Agent-hours per day | In every 100 days, days below the target | Service level |
| --- | --- | --- | --- |
| The claim: mean profile | 2,264 | 26 | 86% |
| Profile of the day of the week (zero cost) | 2,261 | 12 | 90% |
| Profile of the day of the week + margin for the residual variation | 2,428 | 4 (3.7) | 96% |
| The claim + 12 attendants equally | 2,433 | 7 (7.4) | 94% |

The last two rules cost almost the same (about 7% more agent-hours) and put the slack in different
places. The one that puts the slack where the risk is brings the failure from 26 down to fewer than 4 days in every 100; the one that
spreads attendants equally brings it down only to 7.

---

### Step 2 — The rule of the game

**The queue, in words.** A post has an arrival rate (patients per hour) and a service rate
(patients per hour). If the arrival stays below the service, the queue does not explode to infinity; it oscillates around
an average size. The long-run mean wait of a queue with one attendant, memoryless arrivals and
memoryless service is

$$W(\lambda)=\frac{\lambda}{\mu\,(\mu-\lambda)},$$

in which $\lambda$ is the arrival rate and $\mu$ the service rate, both in the same unit of time. With $\lambda = 8$,
$\mu = 12$ and the hour as the unit, $W = 8/(12 \cdot 4) = 1/6$ of an hour, or 10 minutes. The condition in which the formula
holds is written in the expression: $0 \le \lambda < \mu$, demand that does not change and a long horizon. When
$\lambda$ approaches $\mu$, the denominator goes to zero and the wait explodes; it is the curvature that Step 1 showed,
now in a formula.

**Where the formula fails.** Applied to days that alternate between 5 and 11 patients per hour, it would give
$(W(5)+W(11))/2 = 29.3$ minutes. The simulation gives 20.7. The difference teaches three things. First, a day of 10 hours
is not the long run of the formula. On the surge days the wait stays at 28.2 minutes, not at the 55 that $W(11)$ predicts,
because the queue starts empty and has only 10 hours to grow. Second, about 70% of the patients arrive on the surge
days, and that is why the mean *per patient* weights the bad days more. Third, the formula is not wrong; it answers
the long-run wait with constant demand when the manager asked how many patients wait too much on the bad
days. It is a type III error. The answer is right for the wrong question (Chapter 7).

**Jensen in the queue.** The wait is convex in demand, and Jensen's inequality (Chapter 5) says what happens with
any convex function:

$$\mathbb{E}\big[W(\lambda)\big] \;\ge\; W\big(\mathbb{E}[\lambda]\big), \qquad
\mathbb{E}[W(\lambda)] - W(\mathbb{E}[\lambda]) \approx \tfrac{1}{2} W''(\mu)\,\operatorname{Var}(\lambda).$$

The left-hand side is the wait that whoever arrives actually feels; the right-hand side is the wait of the average day. The larger the
oscillation of demand, the larger the bill, because the gain and the loss are not symmetric. Five worlds with the same mean 8
give very different waits:

| World (patients per hour) | Mean wait | In every 100 patients, more than 30 min |
| --- | --- | --- |
| 8 every day (the one of the claim) | 9.3 min | 8 |
| 7 or 9 | 10.5 min | 10 |
| uniform between 5 and 11 | 12.3 min | 13 |
| 5 or 11 | 20.7 min | 25 |
| 5 or 11, with reinforcement only on surge days | 2.5 min | fewer than 1 |

**The independent unit decides the margin.** The effective size of a sample is a property of the process, not of the
count (Chapter 6). Calls of the same day share the rate and the queue; treating them as independent observations
is **pseudoreplication**, and the margin comes out too small. The 2.65 million calls of the vault are worth,
in independent information, what 81 days are worth. The halves null model arbitrates the question without convention. It
splits the days of a known world at random into two halves, measures the difference between them and says which of the two
predicted margins describes what was seen. The margin per day describes; the data refute the margin per patient.

**The severe test (Mayo).** A difference between two worlds is evidence only if the procedure would have accused the
difference had it not existed. The difference between the world of 8 and the world of 5 or 11 is 11.4 minutes, and it stays
53 sd from the null; in 2,000 random splits, the null never passed 0.81 minute. It is the definition of a
result that could have come out wrong.

**What was assumed in the real case, and how the size of the assumption was measured.** Neither the service time nor the
patience of the clients is in the dataset; the file brings counts of calls, nothing more. The service was assumed
exponential with mean 4 minutes, and the patience exponential with mean 3. The audit turned the two
assumptions into a declared grid of 4 × 6 points (service of 3, 4, 5 and 6 minutes against patience of 1, 2, 3,
5, 7 and 10 minutes) and rescaled the four rules point by point. That is why the rule of the profile of the day of the week with
margin wins in 20 of the 24 points, and in the other four (all with patience of 1 minute) the four rules tie
without any of them leaving more than 1 day below the target. **There is no turning point.** The corner of the grid is the worst case,
with 5 minutes of service and 10 of patience. Not even the best of the four rules holds the target, with 14.2% of the days below
it against 37.7% of the rule of the claim. In every 100 days, 38 fail with the schedule of the claim and 14 with the rule
that is right, which costs 7.2% more agent-hours. Patience governs the size of the effect; the service time
barely moves it (with patience fixed, the fraction of the winning rule walks at most 0.9 percentage point across the whole
range). What the grid does **not** measure is the family of distributions (both stay exponential, with coefficient
of variation 1) and the dependence between patience and service. And the health domain of the original example stayed
out, because the public data portals were out of reach of the network.

**The reproduction.** The whole cycle went through the invariants in force at that moment and reproduced the six numbers
that the course material published, with identical counts of patients in the two worlds (161,867 and 160,220) and the
same hash in the two executions, and that makes each number above a verifiable measure, and not a narrative.

---

### Step 3 — Hands on

The laboratory rebuilds Lab 0 in Rust, with a fixed seed and no dependencies: embedded SplitMix64, one day of the
post with Poisson arrivals spread over the day, exponential service and the Lindley recursion for the wait,
which is the rule "I wait for what is left of the queue, or nothing, whichever is larger". The code is in `crates/stochology-book/src/ch16.rs`;
the output below was measured with `cargo run --release --bin labs -- --chapter 16`.

```text
== Chapter 16 :: the same mean, two waits ==
2000 days of 10 h per world, one attendant, mu = 12 per hour, seed 1909 (+ world index)
world (patients per hour)    patients    wait  median     p90  >30min   margin/day     margin/pat.
8 every day (the claim)        159716     9.1     3.7    26.1      8%         0.14            0.03
7 or 9                         159057    10.1     4.0    29.1      9%         0.20            0.04
uniform between 5 and 11       160475    12.8     5.0    37.1     14%         0.32            0.05
5 or 11                        158656    19.3     8.1    55.8     24%         0.51            0.07
5 or 11, reinforcement in the surge   163683     2.5     0.0     8.2      0%         0.03            0.01

halves null: 4000 days of demand 8, 2000 splits of 2000 days against 2000
measured sd of the differences = 0.2062 min
predicted by the day unit     = 0.1928 min
predicted by the patient unit = 0.0473 min  (319704 patients)
null: max |difference| in 2000 splits = 0.664 min

severe test: observed difference = 10.2 min, 49.4 sd of the null, null fraction >= observed = 0.0000
tail: 8 in 100 patients wait more than 30 min in world 8; 24 in 100 in world 5 or 11
```

Read it line by line. **9.1 against 19.3.** The mean demand is the same, 8 patients per hour, and the wait more than
doubles only because the world oscillates. **8% against 24%** and **8.1 against 55.8** at the 90th percentile. The mean separates the two
worlds, and the tail separates far more. **Reinforcement in the surge is 2.5 min and 0%**, and the slack where the risk is zeroes the tail.
**0.2062, 0.1928 and 0.0473.** The sd measured in the halves null is five times larger than the margin of the unit
"patient". Whoever measured the wait treating each patient as independent would declare a precision that the world does not
have. **10.2 min, 49.4 sd, 0.0000.** The difference between the worlds is too large to come from chance, and the
procedure could have come out wrong and did not.

**Divergence declaration.** The previous edition of this book and `h0-lab0/README.md` record 9.3 min and 20.7 min
for the two worlds, 8 and 25 patients in every 100 above 30 minutes, medians of 3.8 and 9.3, 90th percentiles of 26.7
and 59.3, and, in the halves null, a measured sd of 0.216 against 0.227 predicted by the day and 0.047 by the call (severe
test: 11.4 min, 53 sd, ceiling of 0.81 in 2,000 splits). This instrument measures **9.1 and 19.3**, **8 and 24**
in every 100, medians of **3.7 and 8.1**, 90th percentiles of **26.1 and 55.8**, and, in the null, **0.2062** measured against
**0.1928** by the day and **0.0473** by the call (severe test: **10.2 min**, **49.4** sd, ceiling of **0.664**). The
margin per patient coincides up to the third place, because it is a standard deviation account over the number of patients.

**The real dataset also runs in Rust.** The laboratory above is the simulated world; the bank call centre is measured by the
`labs` crate, reading `data/bank_calls.csv` with `cargo run --release --bin labs -- --bank`. Measured output.

```text
== Real H0 :: the bank call centre, in Rust ==
training 83 days (Mar-Jun) | vault 81 days (Jul-Oct) | 2649354 calls in the vault
roster of the mean profile: 234 agents at the peak, 2259 agent-hours per day
service level on the mean profile = 0.8193 | mean of the vault days = 0.5683 | days below the goal of 80% = 0.6543
dispersion between days: Var/mean = 246.7 in the vault, 272.6 in the training window, 261.1 over all | deviation between days = 2841 | Poisson would predict 181
halves null: per day = 0.0441 | per call = 0.0006
```

The **queue with abandonment** and the four staffing rules (R0 to R3) also run in Rust, in
`crates/stochology-book/src/fila.rs`, with exponential service of 240 s, exponential patience of 180 s, target
of 20 s, goal of 80% and seed 2003. Output measured with `cargo run --release --bin labs -- --queue`.

```text
== Real H0 :: the queue with abandonment and the four scaling rules (R0 to R3) ==
training 83 days (Mar-Jun) | vault 81 days (Jul-Oct) | 2654950 calls in the vault
exponential service 240 s | exponential patience 180 s | target 20 s | goal 80% | seed 2003 | 3 replicas per day
residual CV of the day 0.0598 | margin of R2 = 1 + 1.2816 x CV = 1.0767 | R3 = R0 + 12 agents across the board
days in the training window per weekday [16, 17, 17, 17, 16, 0, 0]

Exact engine (observed arrivals, FCFS with abandonment) — the reconstruction
rule    agent-hours/day   peak     service level    days below the goal         abandon/100     mean wait s median wait s  p90 wait s
R0                 2264    234             83.9%                  30.9%                 8.4              14            0           0
R1                 2261    267             84.4%                  19.8%                 8.1              14            0          44
R2                 2428    287             91.6%                   4.9%                 4.2               7            0           0
R3                 2433    246             90.3%                   9.9%                 5.0               8            0           0

Events engine (M/M/c+M event by event, Poisson arrivals) — independent verification
rule    agent-hours/day   peak     service level    days below the goal         abandon/100     mean wait s median wait s  p90 wait s
R0                 2264    234             84.4%                  28.4%                 4.3               7            0           6
R1                 2261    267             87.5%                  16.0%                 3.7               6            0          14
R2                 2428    287             95.0%                   4.9%                 1.6               3            0           1
R3                 2433    246             92.4%                   8.6%                 2.2               4            0           0
```

Four readings, with the two divergences declared. **(1)** The agent-hours match the record:
2,264, 2,261, 2,428 and 2,433 per day in the four rules. **(2)** The order is the one of the record: R2 is the best, R3 loses
to R2 while spending the same, and the claim (R0) is the worst. **(3)** The service levels come out a little below the
published ones (83.9% against 86% in R0, 91.6% against 96% in R2) and the failures per day a little above: 30.9% against
26% in R0, 19.8% against 12% in R1, 4.9% against 3.7% in R2, 9.9% against 7.4% in R3. **(4)** The independent
verification, with Poisson arrivals event by event, reaches R2 with 95.0% and 4.9% of failing days, touching the
published figure. The record measured with another simulation engine and another seed; the difference stays declared, and what
matters for the decision, R2 winning at the same cost, holds in both engines. The instrument was
rebuilt from the retired code, which stays in the history (`git show HEAD:h0-real/estocologia/central.py`),
with two extra verifications proving the engine: the mean wait without abandonment of R0 comes out at **1,040 s** against the
956 s published, and the claim world gives **96.3%** and zero failing day in 81, against 97% and zero. A grid difference
stays on the record. This instrument includes the 21:00 interval, and therefore counts 2,654,950 calls in the
vault against the 2,649,354 of the Erlang C instrument, which cuts that row.

The instrument of the Erlang C Table reproduces in Rust the central numbers of the real $H_0$. The mean profile stays at
**82%** (0.8193), the schedule of the claim meets the real day at **57%** (0.5683) and **65%** of the days stay below the
target (0.6543); the sd between days of
**2,841** against the **181** that Poisson would predict; and the `Var/mean` of **272.6 in the training set**, which is the published 273.
In the vault it is 246.7 and in the whole series 261.1, and the reason is the cut of the window. The halves null per day comes out at
0.0441 against the 0.0474 published (different splitting scheme), and the null per call of this account is 0.0006
by the binomial, against the 0.0033 of the script, which used another design of intraday correlation. The two
divergences stay declared, together with the Rust reconstruction of the decision table of the schemes R0 to
R3 by the queue simulation with abandonment (`fila.rs`, measured just above with `labs --queue`).
The other values differ because the number generator is another (SplitMix64 here, PCG64 in the original material) and
because the mean wait per patient in the oscillating world is a ratio of heavy-tailed sums. Running the instrument
with six different seeds, the wait of the world 5 or 11 goes from **19.3 to 20.6 minutes**. The difference
is inside the variation of the instrument, and the two sets of numbers stay on the record.

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

**Exercises.**

1. *(Account)* Use $W(\lambda)=\lambda/(\mu(\mu-\lambda))$ with $\mu = 12$ to find the long-run wait with
   $\lambda = 8$, $\lambda = 5$ and $\lambda = 11$; then compute the mean of the last two and compare it with the 20.7
   minutes of the simulation. Why does the formula give more?
2. *(Concept)* Why does the mean per patient weight the surge days more? Compute, in every 100 patients, how many
   arrive on days of 11 and how many on days of 5.
3. *(Laboratory)* Run `crates/stochology-book/src/ch16.rs` with seed 1909 and confirm the wait of the world 5 or 11 and the
   margin per day. Then change the surge reinforcement from 24 to 18 consultations per hour: does the mean wait improve?
   Does the tail change size?
4. *(Account)* In every 100 patients of the claim world, 8 wait more than 30 minutes; in the real world, 25. How many
   more patients are they, if the post serves 8,000 patients per year?
5. *(Concept)* Write, in four sentences and in natural frequencies, the memo of the real H0 for the board of the
   bank, with an exit clause (which new fact would make the decision be reviewed).
6. *(Concept)* Explain why "the Erlang C formula is not wrong, but the answer is of type III". Which question did
   the manager ask and which question did the formula answer?

**Summary.**

- The same mean demand can give 9 or 20 minutes of wait: the wait is convex in demand, and whoever looks only at the mean
  chooses a world that does not exist.
- The median deceives. In the world that oscillates, the typical patient waits exactly the number that the claim promised as a
  mean, and the whole difference stays in the tail.
- The margin is computed on the independent unit, which is the day, and not the patient; the halves null model arbitrates the
  choice and refutes the pseudoreplication.
- In 2.65 million real calls, the standard schedule of the industry fails in 26 of every 100 days, and spending about 7%
  more agent-hours where the risk is brings that down to fewer than 4.

## Chapter 17. Horizon H1, part 1: the illusion of multiplying by 3 in the market and the employment programme

**Objectives.** By the end of this chapter you must know how to measure the volatility drag, to distinguish the mean of
the ensemble from the typical trajectory, to compute the leverage at which a single day destroys the capital and to recognise a selection
bias, including the uncomfortable case in which the fit hits the number and still does not count as evidence.

---

### Step 1 — The real world and the paradox

**The promise of leverage.** An investor opens 115 years of Dow Jones quotes (from 1885 to the end of 1999,
31,919 sessions) and computes the mean of the daily returns, 0.0668 per year as an arithmetic sum. The conclusion seems
inevitable: *"the index yields 0.07 per year; leveraged 3 times, it yields 0.20 per year, and whoever holds for decades
always wins."*

Except that 0.20 is the number of the average day seen from outside. Over time, a **trajectory** lives another
thing. Measured in those 31,919 sessions, with daily rebalancing:

| Leverage | The arithmetic sum promises | The growth realised in time | What volatility charges |
| --- | --- | --- | --- |
| 1× | 0.0668 per year | 0.0515 per year | 0.0153 per year |
| 2× | 0.1336 per year | 0.0719 per year | 0.0617 per year |
| 3× | 0.2004 per year | 0.0601 per year | 0.1403 per year |

Leverage of 3 times promises 20% per year and delivers 6%. The difference, 14 points per year, is the **volatility
drag**, a toll paid every day, without exception, and it grows faster than leverage
(later on, with its square). Leveraging 3 times yields **less** than leveraging 2. There is an
optimal point, and passing it destroys growth.

**The Friday that erased 68%.** On 19 October 1987 the index fell 22.61% in a single day. In every 100 reais
of capital:

- without leverage, 77 reais remain;
- with 2 times, 55 reais remain;
- with 3 times, **32 reais** remain, 68 reais destroyed in one session;
- with 4.42 times, zero remains. Above that leverage a single day is enough, and the capital goes to zero without return,
  because nothing is gained from zero.

**Whoever holds for 20 years.** The most generous test for the claim is the long horizon. In every 100 cohorts of 20
years (the investor buys on the first session of a month and sells 20 years later):

| Leverage | End above the start | Median wealth | Wealth summed ÷ number of cohorts |
| --- | --- | --- | --- |
| 1× | 96 | 2.13× | 2.9× |
| 2× | 78 | 2.67× | 7.8× |
| 3× | 64 | 2.10× | **19.1×** |

The mean of 19 times coexists with a median of 2.1. The mean number is pulled by a handful of lucky starts, and
the typical cohort that the investor holds barely doubles. About 35.7 in every 100 leveraged cohorts
end below the starting point, the exact complement of the 64.3% that end above it.

**How many independent observations is that?** The cohorts overlap, since one year enters dozens of
20-year windows, and that is why the 1,134 cohorts of the training set are worth only about **5.8 blocks
independent** (`h1-real/README.md`). Counting cohorts is not counting observations. A margin computed
over 1,134 would be about 14 times smaller than the honest one ($\sqrt{1\,134/5{,}8}$). It is the lesson of the independent
unit of Chapter 6, now in time.

**The Great Depression, in natural frequencies.** From the peak of 3 September 1929 to the trough of 8 July 1932,
every 100 reais invested become **11 reais** without leverage, and **2.5 cents** with 3 times. None of these numbers
comes out of a simulation. The series is real, and the mean drag does not register a day like that.

**The employment programme that looked like a disaster.** In 1976 and 1977, an American job training
programme (NSW) enrolled workers in a difficult situation. To know whether it worked, the obvious route is
to compare the participants with workers from a national survey (CPS), in 1978:

| Group | In every 100, employed in 1978 | Mean income in 1978 |
| --- | --- | --- |
| Participants of the programme (185 people) | 76 | 6,349 dollars |
| Randomised: 260 controls, also applicants, out of the programme | 65 | 4,555 dollars |
| National survey (15,992 people) | 86 | 14,847 dollars |

Read raw, the table condemns the programme. The participants are 10.7 percentage points **less** employed than the
population and earn 8,498 dollars less. Read through the randomisation, the table absolves the programme. Among similar
applicants, those who entered stayed 11.1 points more employed and earned 1,794 dollars more. Between the two
readings there is a distance of 10,292 dollars per person, and that distance is not an effect of the training. It is the difference
between whoever applies to a training programme and the average worker of the country.

**The placebo that reveals the difference.** Before the programme existed, in 1975, in every 100 participants 40 had
a job, against 89 in every 100 of the national survey. The whole difference that the fit will try to repair was already
there, three years before the programme started.

---

### Step 2 — The rule of the game

**Rebalanced leverage.** Keeping leverage constant $L$ means multiplying the wealth by $1 + L r_t$
at each session, in which $r_t$ is the simple return of the index. Two quantities stay apart:

- the **arithmetic mean** of the return, $\mu = \mathbb{E}[r]$, which the sum of the returns promises;
- the **time growth rate**, $g_L = \mathbb{E}\big[\ln(1 + L r)\big]$, which a trajectory
  experiences in the long run.

They do not coincide in any world with volatility:

$$g_L \;\approx\; L\mu \;-\; \frac{L^2\sigma^2}{2}, \qquad
\text{drag}_L \;=\; L\mu - g_L \;\approx\; \frac{L^2\sigma^2}{2}.$$

The drag grows with the **square** of the leverage. Measured in the training set (daily sd $\sigma = 0.010496$, or
0.1749 per year), the observed drags are 0.0153, 0.0617 and 0.1403, against 0.0153, 0.0612 and 0.1377 predicted by the
formula. The second-order account already catches the essential, and little is left for the higher moments of the distribution.
The approximation holds for small returns and no large jump; on a day of −22.6% it stops
describing what happened and only underestimates the damage.

**Non-ergodicity, in one sentence.** The ensemble mean of all investors is not the time mean of
any of them. The mean of 19 times at 3× is an ensemble mean, made of a handful of very
lucky starts and of many cohorts that barely doubled, and nobody lives the mean. When there is an **absorbing state** (the
ruin, from which one does not leave), the ensemble mean stops being a useful description of real life.

**Ruin in one day.** In a world of daily returns, the leverage that zeroes the wealth in a single session is

$$L_{\text{ruin}} = \frac{1}{\min_t |r_t|},$$

and in the training set it is worth 4.42. Above that, ruin stops being a matter of probability and becomes a date marked on the calendar.

**Kelly.** The leverage that maximises the time rate is $L^* = \arg\max_L g_L$. In the posterior of the project, the
90% interval goes from **1.26 to 3.44**, with median **2.35**; in the day-to-day, the empirical Kelly of the series is 2.14.
Leveraging 3 times is inside the interval, but at its upper end; project A decided for much less than
that, a leverage of 0.26 in the simulated exercise of that lens.

**The severe test.** The claim world states that wealth grows 3 times the annual arithmetic sum, without drag. In it,
the fraction of the 20-year cohorts that end above the starting point has mean **0.933** (with the 1% quantile at
0.656). In the real index, the fraction is **0.646**, and would the procedure have accused that value?
Only in 0.95% of the replicas (p = 0.0095). The result is severe.

**What the refutation does not reach.** Delimiting the target that fell is part of the result. The refuted claim is "yields 3
times the mean" and "always wins". "Leveraging never pays" survives the refutation. Without financing cost, 2× and 3× still
grow more than 1× in time (0.0719 and 0.0601 against 0.0515), and the vault from 2000 to 2023 even favoured the claim.
The forecast without drag had skill 0.44, against 0.40 of the model. Two absences in the dataset favour the claim and
are declared. The series is price only (no dividends, and that underestimates the total return by about 3% to 4% per
year in the twentieth century), and there is no financing cost of the leverage.

**Who pays for the error.** The 65.5% of the summed wealth of the cohorts come from the best 10% of starts; the worst entry
dates are all from 1929 (3 September, 1 July, 1 August). In every 100 cohorts of 20 years, 4 end
below the start without leverage, 22 with 2× and 36 with 3×. Whoever entered in the worst month had a mean return equal to that of the
market and did not survive to see it.

**The LaLonde decomposition.** Comparing participants with non-participants gives:

$$\underbrace{\bar Y_{\text{treat}} - \bar Y_{\text{non treat}}}_{\text{raw difference}}
= \underbrace{Y_1 - Y_0}_{\text{causal effect}}
+ \underbrace{\mathbb{E}[Y_0 \mid T=1] - \mathbb{E}[Y_0 \mid T=0]}_{\text{selection bias}}.$$

The vault, the 260 randomised controls of the NSW, stays out of the sample **by construction**, and that distance is not
temporal. Those 260 people were out of the fit from the first day. The randomisation zeroes the second term by
construction. The controls were drawn among the applicants, so $\mathbb{E}[Y_0 \mid T=1] = \mathbb{E}[Y_0 \mid T=0]$. The fit by
propensity score does not zero anything. It
**estimates** that term from what is observable, and the 1975 placebo shows that the estimate errs. With the
employment of 1975 as the outcome (an outcome that the programme could not have caused), the fit gives −12.3 points
(p = 0.04), whereas the experiment gives +8.5, with an interval from −0.6 to +17.5, compatible with zero.

**Hitting the number is not having evidence.** The three numbers bring each one its 95% interval. The raw comparison gives
−10.7 points [−17.0; −4.5]; the propensity fit gives +9.6 [−0.6; +20.8]; the randomised experiment gives +11.1
[+2.6; +19.6]. The fit comes close to the experiment and still does not count as evidence. But an **inert** treatment would have given +9.0, inside the band of the null, from −10.3 to +11.9 (p = 0.08). The
procedure could not have come out grossly wrong, and that is why the hit does not count as evidence (Mayo). The
fit relies on about 103 effective controls, and 72% of the CPS is below the smallest score of the treated. In
practice, it compares the treated with a handful of CPS people who look like them; those people are
different in things that the model does not see.

**What the vault showed.** With the 260 randomised controls, the skill of the model is +0.18 (it predicted 0.655 of
employment, the observed was 0.646); the logistic regression stays at +0.09 and the raw counterfactual of the CPS, at −0.002. The
k-anonymity of the rows goes from 1 to 11 after the generalisation of quasi-identifiers.

**Fairness, measured.** The false negative rates differ by group. They are 0.72 for Hispanics, 0.50 for Blacks and 0.57
for the others. Equalising them captures 22 more unemployed people and costs 87 false positives. It is the trilemma of Chouldechova
measured in numbers. There is no way to get the three things right at once, and someone will pay for the choice (Chapter 11).

**The two projects of this chapter passed.** The one of the market and the one of employment crossed the five gates of H1 in
a real domain and were approved by the adversarial board, with the numbers above recorded at the moment in which they were
produced. The third project of the horizon, the Danish fires, is Chapter 18, and it was failed.

---

### Step 3 — Hands on

The laboratory reads the three public datasets and measures all the quantities cited above: the time growth and the drag,
the 20-year cohorts with a monthly start, the 1987 shock, the Depression, the claim world by resampling of
years and the raw decomposition of the NSW against the CPS. The code is in `crates/stochology-book/src/ch17.rs`; the output below was
measured with `cargo run --release --bin labs -- --chapter 17`.

```text
== Chapter 17 :: volatility drag and the selection bias ==
training 1885-1999: 31919 sessions in 115 years; whole series 1885-2023: 37930 sessions
daily arithmetic mean = 0.000240669 (0.0668 per year), daily sd = 0.010496 (0.1749 per year)

time growth per year and drag (cohorts and trajectories of the training set)
  L=1: the arithmetic sum would give 0.0668 per year; the realised growth is 0.0515; drag 0.0153
  L=2: the arithmetic sum would give 0.1336 per year; the realised growth is 0.0719; drag 0.0617
  L=3: the arithmetic sum would give 0.2004 per year; the realised growth is 0.0601; drag 0.1403
worst session: -0.2261 (-22.61%) on 1987-10-19; loss at L=1 0.226, L=2 0.452, L=3 0.678; daily ruin at L >= 4.42
  calendar year 1929-1932 at L=1: the capital ends at 0.19521 of the initial one (80.5% destroyed)
  calendar year 1929-1932 at L=3: the capital ends at 0.00088 of the initial one (99.9% destroyed)
  peak to trough, 1929-09-03 to 1932-07-08 at L=1: the capital ends at 0.10814 of the initial one (89.2% destroyed)
  peak to trough, 1929-09-03 to 1932-07-08 at L=3: the capital ends at 0.00025 of the initial one (100.0% destroyed)

20-year cohorts starting on the first trading session of each month (training set):
L           cohorts     median (x)       mean (x)  above the start
1              1134           2.13            2.9           0.9577
2              1134           2.67            7.8           0.7848
3              1134           2.10           19.1           0.6429

cohorts per unit-year (115 years, moving sum of 20): above the start at 1x = 0.9583, at 3x = 0.6458 (96 cohorts)
claim world (training years resampled, growth 3*annual arithmetic sum, without drag):
  2000 replicas: mean fraction of the cohorts above the start = 0.9286, 1% quantile = 0.6354, p(fraction <= 0.6458) = 0.0140

LaLonde: NSW with 185 treated and 260 randomised controls; CPS with 15992 observations
comparison (1978 outcome)                employment   income (US$)
randomised experiment (NSW treat. - NSW ctrl)      0.111           1794
raw comparison (NSW treat. - CPS)            -0.107          -8498
implied selection bias (raw - experiment)     -0.218         -10292
employment levels in 1978: NSW treated 0.757, NSW controls 0.646, CPS 0.864; in 100 people: 76, 65 and 86
mean income of 1978 (US$): NSW treated 6349, NSW controls 4555, CPS 14847
1975 placebo (employment): NSW treated 0.400, NSW controls 0.315, CPS 0.891; raw difference treated - CPS = -0.491
```

Read it line by line. **0.2004, 0.0601, 0.1403**: the arithmetic sum of 3× pays 20% per year and volatility keeps
14, more than the gain. **0.678** and **4.42**: one day is enough to destroy 68% of the capital
leveraged 3 times, and above 4.42 times it is enough to zero it. **2.10 against 19.1**: at 3×, the typical cohort barely
doubles while the ensemble mean reaches 19 times. **0.6429 and 0.9286**: in the real index, 64 in every 100 cohorts
of 20 years end above the starting point; in the world in which the claim is true, 93 in every 100, and the
difference is the drag. **−0.107 and +0.111**: the question about the programme gives two opposite signs, and the
difference of −10,292 dollars is the selection bias, measured and not assumed.

**Divergence declaration.** The `h1-real/README.md` records 0.051, 0.072 and 0.060 of growth per year;
this instrument measures **0.0515, 0.0719 and 0.0601** — the coincidence is of three places, because the account is the same. The
published cohorts with a monthly start are 1,135 and the ones here are 1,134 (the difference is the border criterion: one cohort
whose end falls on the last session of the training set). The published values of the fraction above the start are 0.9586 / 0.7841 /
0.6405; the ones here are **0.9577 / 0.7848 / 0.6429**, and the published median wealths (2.13 / 2.68 / 2.08) match
the ones here (2.13 / 2.67 / 2.10). In the claim world, the published material brings mean 0.9332, 1% quantile of
0.6563 and p = 0.0095, against **0.9286**, **0.6354** and **0.0140** here: the difference is in the draw of the
resamplings (2,000 replicas in each case, different generators), and the p-value changes in the third place. One different
case deserves a note: the previous edition cites "from 1929 to 1932, the capital falls to 0.11 with 1× and to 0.00025 with
3×". With the window of the **calendar year** (1 January 1929 to the end of 1932), this instrument measures **0.19521** and
**0.00088**; with the window of the **peak to the trough** (3 September 1929 to 8 July 1932), it measures **0.10814** and
**0.00025** — exactly the numbers of the material. The divergence was not of method: it was of window, and the two
appear now, with the date of each one.

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

**Exercises.**

1. *(Account)* Use the second-order drag, $\tfrac{1}{2}L^2\sigma^2$, with $\sigma = 0{,}1749$ per year to
   predict the drag of 1×, 2× and 3× and compare with 0.0153, 0.0617 and 0.1403 measured. At which leverage does the drag
   start to exceed the arithmetic gain?
2. *(Account)* Which leverage zeroes the capital on the day of −22.61%? And on the worst day of 1929? Explain why "ruin"
   is an absorbing state and the effect of that on the ensemble mean.
3. *(Concept)* Explain the sentence "whoever holds for decades always wins" and distinguish what the refutation
   brings down from what it leaves standing.
4. *(Laboratory)* Run `crates/stochology-book/src/ch17.rs` and redo the table of cohorts with a 10-year horizon
   (change the constant `HORIZON`). What happens to the fraction of 3× cohorts above the start? And to the
   median?
5. *(Concept)* Why does the placebo with the employment of 1975 measure the selection bias? Write the answer using the
   decomposition of the raw difference.
6. *(Account)* With the measured levels of employment in 1978 (76% of the treated, 65% of the randomised controls), compute the
   effect in percentage points and explain why the raw comparison gives the opposite sign.

**Summary.**

- Leverage multiplies the arithmetic mean, but time charges the drag $\tfrac{1}{2}L^2\sigma^2$. The 3× promises
  0.20 per year and delivers 0.06.
- A single day can destroy 68% of the capital leveraged 3 times; above $1/\min|r|$ (4.42 here) ruin is
  a matter of calendar, not of probability.
- Ensemble mean and typical trajectory separate. The 19.1× of mean coexist with 2.10× of median, and about 35.7 in every 100
  3× cohorts end below the start.
- A refutation must declare the target: "yields 3 times" and "always wins" fall; "leveraging never pays" does not.
- Hitting the number is not enough. In LaLonde, the propensity fit gives +9.6 points, and an inert treatment would have given
  +9.0; the 1975 placebo shows that the selection bias is still there, worth 10,292 dollars per person.

## Chapter 18. Horizon H1, part 2: the Danish fires and the art of failing one's own model

**Objectives.** By the end of this chapter you must know why a clean window is weak evidence, what the effect of
a tail with $\alpha$ below 2 is on any mean, how the Laplace rule turns "never happened" into "one
chance in 38" and why failing your own model counts as a result.

---

### Step 1 — The real world and the paradox

**The reinsurer that saw three calm years.** At the end of 1988, a reinsurer looks at the series of fire
claims of Copenhagen Re and at the stretch that interests it, the layer that it sells, "100 above 50". In words, it
pays the part of each claim that stays **between 50 and 150 million Danish kroner** of 1985. The retention of 50
million stays at the 99.7th percentile of the losses, at the tip of the distribution.

The analyst sees **36 months in a row, from 1986 to 1988, without a single claim even touching the layer.** The conclusion that
he writes repeats the usual one: *"the risk is negligible (something like one month hit per century) and we can sell three
times our capital."*

**What the whole series says.** The history of the layer, told in months hit:

| Period | Months | Months in which the layer was hit | Largest loss of the period |
| --- | --- | --- | --- |
| 1980-1982 | 36 | 4 | 263.3 M |
| 1983-1985 | 36 | 1 | 57.4 M |
| 1986-1988 | 36 | **0** | 47.0 M |
| 1989-1990 (the vault) | 24 | 2 | 152.4 M |

The two months of the vault have a name and a date: **August 1989, 152.4 million**, and October 1990, 144.7 million. The
first falls eight months after the decision to leverage.

**In every 100 windows.** Here is the central error of the claim, and it is an error of counting. In the whole training set, the layer
was hit in 5 of the 108 months, about 4.6 months in every 100. In that world, **about 18 in every 100 windows
of three years come out clean** by pure chance. Seeing a clean window is not seeing a safe world. It is seeing the expected
result in a dangerous world, more often than one imagines.

**The Laplace rule.** How many chances per month remain after 36 clean months? The rule of succession answers:

$$\frac{0+1}{36+2} = \frac{1}{38} \approx 0{,}0263.$$

In every 100 months, 2.6 months hit, never zero. "It never happened" is not "it cannot happen". It is "it has not
happened yet".

**The tail.** The Hill estimator, applied to the 1,714 losses of the training set, gives for the tail index
$\hat\alpha = 1{,}81$, with a 95% interval from 1.41 to 2.56 (bootstrap by monthly blocks). That matters because an
index below $2$ means **undefined variance**, and the long-run mean of the loss does not converge. A single
claim can be so large that it dominates all the others added together, and August 1989 is the case.

**The ruin.** With the exposure that the claim would justify (selling a limit equivalent to 3 times the capital), a
claim that consumes the whole limit takes the capital to **1 − 3 = −2**, that is, to zero. It happened in August 1989:
the reinsurer is ruined, and ruin is an absorbing state, from which one does not leave. Whoever used the Laplace rule
on the clean window (a much smaller exposure) ends 1989-1990 with **19% of the capital**; whoever used the rate of the adjusted tail
The tail fit (POT/GPD) covers 78%. The Kelly of the posterior, at the optimal point, is $f^* = 0{,}46$.

**The art of failing one's own model.** The project took the claim to the gates and the result was uncomfortable for
everyone, because in the vault **no forecaster beat the simple historical frequency**. The Brier skill of the model
(which is the difference between predicting well and predicting the historical constant, Chapter 8) stayed at **−0.015**; that of the POT,
at −0.003; that of the Laplace rule, at −0.024; and that of the claim, at −0.052. Besides, the frequency of claims
was growing **4.6% per year** (p = 2·10⁻⁶), and the world was not stationary. The elicited (simulated) prior dominated the
posterior.

The adversarial board failed the project, and the tests were not adjusted so that it would pass. One sentence summarises
the verdict: **the claim loses, but the model does not win the right to decide.** The F8 issued no decision, the
gates 4 and 5 failed because of that project, and that is what the horizon publishes.

**The clean window in the whole series.** The project tested the rule on all the clean windows of the history. Whenever the
last 36 months had not touched the layer, the decision would have been to leverage at the ceiling. There were 5 such decisions, and the
**5 ended ruined within 24 months**.

---

### Step 2 — The rule of the game

**The wild tail.** A distribution has a heavy tail when the probability of extreme values falls like a
power, $P(X > x) \propto x^{-\alpha}$, and not like an exponential. The Hill estimator uses the $k$ largest
observations:

$$\hat\alpha = \left[\frac{1}{k}\sum_{i=1}^{k}\ln\frac{x_{(i)}}{x_{(k+1)}}\right]^{-1}.$$

The reading is direct. The more slowly the largest observations decrease, the smaller $\hat\alpha$. It holds when
the tail is really of power and there is a plateau in $k$ where the estimate does not change. It fails in small samples and
when the choice of $k$ decides the answer (here, $\hat\alpha = 1{,}96$ with $k = 25$, $1{,}81$ with $k = 50$ and
$1{,}41$ with $k = 100$). The interval by bootstrap of monthly blocks
(1.41 to 2.56) is the honest way of saying: "we know that the tail is heavy; we do not know how much". An alternative
fit of the tail (POT, with a generalised Pareto distribution above 10 M) gives $\xi = 0{,}41$ [0.02; 0.72],
that is, $\alpha \approx 2{,}46$, lighter than the Hill. The two methods disagree, and the disagreement is in the
result.

**Why $\alpha < 2$ changes everything.** The variance of a variable with a power tail is finite only if
$\alpha > 2$. With $\alpha = 1{,}81$, the variance does not exist, and there is no "standard deviation" of the annual loss, so that
any margin computed from an estimated variance will be small. The case repeats the lesson of Chapter 4. A
process can be outside the hypothesis space of the model, and the data do not warn with a pretty chart.

**The Laplace rule of succession.** After $s$ successes in $n$ trials, the probability of the next
success is

$$P(\text{success} \mid s, n) = \frac{s+1}{n+2}.$$

It comes from a uniform prior over the probability and treats each month as exchangeable with the others. It holds when the
months are exchangeable, without memory and without trend, and it fails when the world has drift. And it has. The frequency of
claims grows 4.6% per year, with a p-value of $2 \times 10^{-6}$. Laplace, therefore, delivers a **floor**. After
36 clean months, the defensible minimum is 1 in 38 per month, never zero.

**How much a clean window is worth.** Here the account stops being about the frequency and becomes about the evidence.
Compare two hypotheses: "the risk is negligible" (1 month hit per century, $p = 1/1200$) and "the risk is the one of the
tail of the training set" ($p \approx 1/23$ per month). The probability of seeing 36 clean months under each hypothesis:

$$P(\text{clean window} \mid \text{negligible}) = \left(1 - \tfrac{1}{1200}\right)^{36} = 0{,}9704, \qquad
P(\text{clean window} \mid \text{dangerous}) = \left(1 - p\right)^{36} = 0{,}2058.$$

The likelihood ratio is 4.72, or **2.2 bits** of evidence. Two bits is very little to decide
to sell three times the capital. The severity criterion says how much would be needed. For the procedure "the layer
was not touched" to have 95% chance of accusing the danger, **69 clean months** would be necessary, almost six years.
The evidence is also measured the other way, as mutual information between the result of the window and the hypothesis
dangerous-against-negligible, **0.51 bits**, practically nothing. And, for comparison, if the real world were the one of the vault
(2 months hit in 24, $p = 1/12$), that clean window would be worth 4.5 bits. Three clean years are reasonable evidence
against a monthly rate of 8% and ridiculous evidence against one of 4%.

**Brier skill.** The grade of a forecaster comes from a scoring rule, and the reference is the historical
constant:

$$\text{skill} = 1 - \frac{\text{BS}_{\text{forecaster}}}{\text{BS}_{\text{climatology}}}.$$

Positive skill means that the forecaster beats the constant; **negative, that it loses to the constant**. The model
(−0.015), the POT (−0.003) and the Laplace (−0.024) lost; the claim (−0.052) lost more. No forecaster won the
right to decide, and the governance gate records that.

**The analogue of the course.** The pattern reappears in a sister lens, a strategy that sells insurance. In 37% of
the simulated histories of 250 weeks not a single loss appears. "I never lost" is worth 1.3 bits of evidence, and a
severe test would require 14 clean years. Of the investors who bet by the Kelly estimated from their history,
35.8% are ruined. The story repeats with another asset, and the clean window remains the most seductive and
weakest argument there is.

**A declared limit of the design.** The retention of 50 million was chosen by looking at the whole series, and not before
seeing it, and that is why the vault **is not blind** for the analyst. The horizon brings a sensitivity table for retentions
from 20 to 50 million, and the design stays on the record as the analysis of a case chosen with the outcome in view, useful to
learn about the method and not to estimate the risk of the insurance.

**Lakatos, again.** An adjustment after a failure can be **progressive** (it predicts something new that is confirmed) or
**degenerative** (it only protects the theory). Project C had every reason to choose the degenerative path.
It was enough to change the retention, to touch the prior or to change the period of the vault. None of that was done. The numbers stayed
as they were, the project was failed, and the horizon records the failure with the care with which it records the
two approvals.

---

### Step 3 — Hands on

The laboratory reads the real series, measures Hill on the tail of the training set and on a simulated tail, counts the months in which the layer
was hit and computes Laplace, the likelihood ratio and the bits of the clean window. The code is in
`crates/stochology-book/src/ch18.rs`; the output below was measured with `cargo run --release --bin labs -- --chapter 18`.

```text
== Chapter 18 :: the tail, the clean window and the information ==
Copenhagen Re fires above 1 M DKK, 1980-1990: 2167 claims in 132 months

period        months        months hit  largest loss
1980-1982         36                 4         263.3
1983-1985         36                 1          57.4
1986-1988         36                 0          47.0
1989-1990         24                 2         152.4
months hit and value: 1980-07 263.3; 1981-05 56.2; 1981-12 50.1; 1982-10 65.7; 1985-08 57.4; 1989-08 152.4; 1990-10 144.7;

training 1980-1988: 1714 claims in 108 months
Hill on the training set: k=25 alpha=1.9625 k=50 alpha=1.8109 k=100 alpha=1.4100
Hill k=50, 95% CI by bootstrap of monthly blocks (300 replicas): [1.37; 2.61]
simulated Pareto with alpha = 1.81, n = 1500, 200 repetitions: mean Hill k=50 = 1.900 (sd 0.265), 90% of the repetitions between 1.51 and 2.39

window of 36 months without a claim on the layer:
Laplace on the window: p = (0+1)/(36+2) = 0.02632, or 1 chance in 38 per month
monthly frequency measured in the training set: p = 5/108 = 0.04630
monthly p of the training tail (published POT/GPD fit: xi = 0.406, sigma = 7.387, 83 excesses above 10): p = 0.04296
  measured frequency p = 0.04630: P(clean window | dangerous) = 0.1815; likelihood ratio = 5.35 (2.42 bits); clean months for 95% = 64
                     mutual information window x (dangerous vs negligible) = 0.5455 bits
  POT/GPD tail       p = 0.04296: P(clean window | dangerous) = 0.2058; likelihood ratio = 4.72 (2.24 bits); clean months for 95% = 69
                     mutual information window x (dangerous vs negligible) = 0.5147 bits
reference: under the rate that the vault showed (2 months hit in 24, p = 1/12), the same clean window would be worth 4.48 bits (ratio 22.3)
```

Read it line by line. **4, 1, 0**: the months hit in each three-year block of the training set; the clean window that the claim
read was real, surrounded by years with four and one month hit. **1.9625 / 1.8109 / 1.4100**: the series gives
three tail indices according to the number of observations used; the tail is heavy with room to spare, and its exact value
is not stable. **1.900 with sd 0.265**: in a simulated tail with $\alpha = 1{,}81$, the Hill estimator
has that uncertainty in samples of that size, the honest translation of the interval [1.41; 2.56]. **0.1815**: in every
100 windows of three years, 18 come out clean in the dangerous world. **0.02632**: the Laplace rule puts the floor at 1 in 38.
**2.24 bits and 69 months**: the clean window is 4.7 times more likely in the negligible world, that is, 2.2 bits of
evidence, and almost six clean years for a severe test. **4.48 bits**: against the rate of the vault, that window
would be worth double.

**Divergence declaration.** The `h1-real/README.md` records $\hat\alpha = 1{,}81$ with interval [1.41;
2.56] by bootstrap in blocks; this instrument measures the same **1.8109** at the point (k = 50) and the interval **[1.37;
2.61]**, besides 1.9625 and 1.4100 for k = 25 and k = 100 — the three points coincide with the published ones up to the fourth
place, and the difference of the interval comes from the resampling (300 replicas, different generator). The divergence that deserves
a paragraph is another one: the previous edition and the README say that 36 clean months are worth **2.2 bits** and that a severe test
would need **69 months**. This instrument confirms the two numbers **exactly** (2.2373 bits and 69 months)
when it uses the $p$ that the tail of the training set implies, $p = 0{,}04296$, the value of the published POT/GPD fit
$\xi = 0{,}406$, $\sigma = 7{,}387$, 83 excesses above 10 — and measures **2.42 bits and 64 months** when it uses the
simple monthly frequency of the training set, $p = 5/108 = 0{,}04630$. The difference is not of account: it is of which world
"dangerous" one places on the other side. The adjusted tail is more pessimistic than the raw count of months, and that is why it
requires more clean months. The two values stay on the record; none was erased.

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

**Exercises.**

1. *(Account)* With the Laplace rule, compute the monthly chance after 12, 36 and 69 clean months. How many clean months
   would be needed for the monthly chance to fall below 1 in 100?
2. *(Concept)* Explain why $\alpha = 1{,}81$ prevents computing an honest margin of error for the annual loss,
   and why the interval [1.41; 2.56] crosses the boundary at which the variance exists.
3. *(Account)* Using $P(\text{clean} \mid p) = (1-p)^{36}$, compute the likelihood ratio between
   $p = 1/1200$ and $p = 1/12$. In bits, and in words: what do those bits authorise one to conclude?
4. *(Laboratory)* Run `crates/stochology-book/src/ch18.rs` with (k = 50) and then change the window from 36 to 72 clean
   months: how many bits is the larger window worth? How many months would be needed for 95%?
5. *(Concept)* "The claim loses, but the model does not win the right to decide." Explain that sentence to a
   risk manager who only wants to know whether or not to sell the layer.
6. *(Laboratory)* Simulate 100 years of i.i.d. months with $p = 1/1200$ and count how many windows of 36 months come out
   clean. Compare with the real history of the Danish series and say what that does to the argument "negligible
   risk".

**Summary.**

- 36 clean months between 50 and 150 million kroner are not a safe world. With the rate of the training set, 18 in every 100
  windows of three years come out clean by chance.
- The tail is heavy ($\hat\alpha = 1{,}81$, interval 1.41 to 2.56). With $\alpha < 2$ the variance does not exist, and a
  single month (August 1989, 152 million) decides the fate of the company.
- The clean window is worth 2.2 bits of evidence; the Laplace rule gives 1 chance in 38 per month; a severe test
  would need 69 clean months.
- In the vault, no forecaster beat the simple historical frequency: model −0.015, POT −0.003, Laplace −0.024,
  claim −0.052, and the claim loses without the model winning the right to decide.
- Failing the model that one wrote, without touching the tests so that it passes, is a result of the programme.

## Chapter 19. Horizon H2: seven rounds to find true causes in science

**Objectives.** By the end of this chapter you must know how one defines an environment that *intervenes* in a variable, why a
cause only deserves that name if it survives the exchange of environments, why the gate of H2 failed five times before
passing, and how one reads a result approved with a caveat. And also a result that passed by a narrow margin and then shrank.

---

### Step 1 — The real world and the paradox

**The question that keeps holding everywhere.** A team wants to know which causes keep holding when the world changes.
Not "the cause in this school", nor "the cause in this year". The cause appears in school A, in school B and in school C, in
the good year and in the bad year, in the hospital of Toronto and in the one of Rhode Island. That is the horizon H2 of the research, and its gate is
single and demanding. It asks for a non-empty set of invariant causes, in at least three different
areas of science.

**First attempt: use the year as "environment".** Twelve areas entered at once: education, climate, credit,
energy, health, law, logistics and others. In each area, the environment was something easy to find. The year, the region, the
airport, the type of school served. In each of the 12 areas, the result was like this:

| What happened | In how many of the 12 areas |
| --- | --- |
| The method rejected all the candidate sets | 6 |
| The method accepted so many sets that the intersection came out empty | 4 |
| It gave a non-empty set — in law, and *against* the claim | 1 |
| It was pruned earlier, by another filter | 1 |

The only area that gave an answer pointed to citizenship and employment as invariant causes of the decision to release a detainee,
and left skin colour out. The result does not say that colour does not matter. The method only promises to return a
subset of the true causes, and such a promise only holds under conditions that the year and the region violate. The year acts
**directly** on the outcome, and not only on the input variables. An environment that changes the whole world is not an
experiment.

**From the second to the seventh attempt: look for environments that are interventions.** The team decided to keep the gate
strong instead of loosening it, and went looking for data in which the environment was a documented intervention on a
variable: a law that changes a benefit, a lottery that changes a health plan, a physical chamber where a
researcher turns a knob. Seven rounds were needed, and each correction was written and locked with a hash **before**
running. The counter of attempts, called $K$, is the honest scoreboard of that search.

| Round | What changed | Areas with a non-empty invariant set | Gate |
| --- | --- | --- | --- |
| H2 | 12 convenience domains: year, region, airport as environment | 1 (law, and against the claim) | Failed |
| H2b | environments that are documented interventions ($K = 1$) | 1 (law) | Failed |
| H2c | non-parametric invariance test ($K = 2$) | 2 (health and education; health only by saturation) | Failed |
| H2d | rule that discards sets without testing power ($K = 3$) | 1 (education) | Failed |
| H2e | physical chambers with the causal graph known | 2 (education and physics) | Failed |
| H2f | gene deletions in yeast | 3 (education, physics and genetics) | Passed the rule, with a caveat |
| H2g | FDA cardiology clinical trial | 4 (the three previous ones and pharmacology) | Passed |

**The light tunnel.** In a physical chamber built in Switzerland to study causality (Gamella, Peters and Bühlmann,
2025), a researcher turns a knob that controls an electric current, three LEDs of different colours light up, and
light sensors measure the response. Of 1,024 possible combinations of candidate causes, the method accepted 128, and
their intersection (the set that all of them have in common) was $\{\text{red},\ \text{green},\ \text{azul}\}$. They are the
three LEDs. The light sensors stayed out, despite being strongly correlated with
the current, because they are **children** of the LEDs. It was the first time in the project that the method was checked against a
known truth, and it got it right.

**The wind tunnel, in this battery, failed everything.** The set of the true parents fell along with the rest. The probable
causes were declared before: the barometers measure absolute pressure and the weather changes the result between regimes; the
autocorrelation that remains after the thinning; and an additive fit where the physics is quadratic, with interaction between the
two fans. The best p-value among the sets stayed at $7\times10^{-5}$. Not even with the ambient pressure as a
proxy did any set hold. A method that fails in one case and gets another right, as long as the plausible causes
are declared, teaches more than a method that always gets it right.

**The heart.** In an FDA trial in which volunteers received four drugs and had the electrocardiogram measured 16
times each, the method pointed as causes of the lengthening of the QT interval to the concentrations of the three
drugs that block the hERG channel: dofetilide, quinidine and ranolazine. All 35 accepted sets, out of 512,
contained the three, and measures of the electrocardiogram itself stayed out. The slope of QTcF under dofetilide was
$25{,}2$ ms per ng/mL, with a 90% posterior interval of $23{,}0$ to $27{,}5$. And the hardest test came
later. On the drug that the model **never saw**, verapamil, the invariant model predicted well (skill $+0{,}57$). The
model that used *all* the variables exploded, with a loss of the order of $10^{15}$, because verapamil pushes one
measure of the electrocardiogram outside everything that was in the training set.

**The yeast.** With 2014 data on gene deletions in yeast, the method gave an invariant set in one
domain out of ten drawn. It rejected 1,022 of the 1,024 sets, accepted two, and the best p-value stayed at
$0{,}0072$, just above the threshold of $0{,}005$.
The validation by intervention, which checks gene by gene, did not confirm two of the three genes of the set; only the
first moved the target in the predicted direction. The output was approved **with a caveat on the record**. A replica declared
later drew 40 new genes from a set of 4,681, with a level of $0{,}05/40 = 0{,}00125$ per gene, and found one
gene that passes the strict validation, with a null rate of $p = 0{,}001$. It is the floor of 999 draws. The replica shows
*one gene in one mutant*, and not an effect measured with resolution. Each mutant is an environment of a single row, and the smallest
possible p-value there is $1/1\,000$.

The laboratory of this chapter measures what the **second row** changes, in the same design (40 mutants, $lpha/40 =
0{,}00125$ per gene) and with two planted cases: a mutant whose effect appears in the **two** rows and an artefact
whose effect appears in **one** row only. With one row per mutant, the validation approves **4 of 40** — the true mover, the
row artefact and two that are pure noise, all touching the floor of $0{,}001$. The p does not separate what
one looks for from what one does not look for, because all of them hit the ground at the same time. With the second row, requiring the
effect in both, **1 of 40** passes: the mover, with $p = 10^{-6}$, which is the new floor; the artefact falls out because the
effect of it in the second row is $+0{,}028$, against $+1{,}016$ of the mover. What was missing was a row, not a smaller p-value
(`labs --genetics`).

**The fall of education.** Education was the most comfortable area. They were small classes drawn in the STAR experiment of
Tennessee, 11,598 students. When the students stopped being counted as 11,598 independent observations and
started being counted as **80 schools**, the accepted sets had no variable in common, and the
invariant set became empty. That does not mean that a small class has no effect. It means that, with 80
true units, the test lost the power to point to a set, which is the only product that it delivers.

**Five failures, three named causes.** The gate failed five times in a row. Three of those failures have a name — using the
year or the region as environment invalidates the premise; a code error stayed hidden in the invariance test of round H2b; and the saturation rule,
which only appeared in the Hou tudo round.

---

### Step 2 — The rule of the game

**The domain package, and the pluggability test.** Each area entered through a standardised package: substantive question,
claim, glossary of ten terms, sources, independent unit, environments and vault, causal graph, adjustment set, elicitation,
decision and scope. The validator rejects the incomplete package **before** reading any data, and ten of the twelve first
packages were written by agents that had read only the contract and two examples.
The machine proved to be pluggable.

**Definition of environment.** An environment is the label that says where each row of the table came from: the school, the year, the
intervention regime. We write $E$ for that label, $X$ for the set of candidate variables and $Y$ for the
outcome that one wants to explain.

**The invariance condition.** A set $S$ of candidates is **invariant** if the distribution of $Y$, once the
variables of $S$ are known, is the same in all environments:

$$Y \perp E \mid X_S .$$

Put another way, after looking at the variables of the set, knowing from which environment the row came changes nothing else in
the prediction. Whoever tests that condition for all the $2^p$ subsets is Invariant Causal Prediction, or ICP, and
it returns the **intersection** of the accepted subsets, called $\hat{S}$.

**What the method guarantees and under which conditions.** If the statistical model is correct, if the rows inside
each environment are independent and if there is enough power, then $\hat{S} \subseteq \text{pa}(Y)$ with
probability of at least $1 - \alpha$. The returned set contains only true causes. The method does **not**
guarantee that all the causes appear, and it says nothing about the size of the effects.

**Where it fails.** Three situations appeared in the research, and each one became a rule.

1. **The environment acts directly on $Y$.** If the year changes the outcome by itself, then $Y \perp E \mid X_S$ is false for every
   $S$. The method rejects everything, and $\hat{S} = \emptyset$. That was the case in the 6 areas of the first round.
2. **The environment is a deterministic function of the variables of $S$.** In that case the condition holds *by construction*.
   Conditioned on $X_S$, the environment is constant, and the test has zero power. A test without power certifies nothing;
   accepting a saturated set is not evidence. The **saturation rule** started to exclude those sets from the
   family. They count neither as accepted nor as rejected. The measure is the hit rate of the nearest neighbour,
   leaving the row out (1-NN LOO), when predicting the environment from the columns of $S$; mean hit rate $\ge 0{,}99$
   means saturated. In the RAND insurance experiment, the copayment is constant inside each regime of
   plan. The set that depended on it left in H2d, as the declaration foresaw.
3. **The rows are not independent.** 16 instants per volunteer in the cardiac trial, up to 403 students per school
   in STAR, 225 thousand flights in 364 days. Treating dependent rows as independent narrows the margin. The correction
   was the Wald clustered by volunteer or by school and the permutation of whole blocks.

**The code failure that the reading found.** In H2b, before any execution, the reading of the code found an
error in the invariance test. The degrees of freedom were counted by the number of **columns** of the design matrix in
place of its **rank**. With environments that are interventions, a candidate is usually constant inside the environment, and
those collinear columns added degrees of freedom without reducing the sum of squares. The test became permissive and
accepted too much. The correction started to use the rank and gained a test of its own. Under the null hypothesis of invariance the
test rejects 7% of the times (the goal was less than 9%), and under the hypothesis of a break, 100%. The re-execution showed that the
error had **not** changed any result of H2.

**The two rulers.** The first ruler was a linear gaussian test: nested F against coefficients per environment,
plus Levene of the residuals. With large samples it rejects any deviation from the linear model, such as mass at zero,
concave response, heavy tail, and in simulation it rejected from 72% to 100% of the times even when the law was
invariant. The second ruler (from H2c on) fits a single pooled model, standardises the residual by the scale
estimated without looking at the environment, and tests location (Kruskal–Wallis) and scale (Brown–Forsythe) of the standardised residue
between environments, with Bonferroni over two tests. A numeric candidate with more than 10 distinct values enters as a linear
term plus a restricted cubic spline with 4 knots at the quantiles 5, 35, 65 and 95%; a candidate with up to 10 distinct
values enters as a factor. The exchange of ruler changed the verdict of three domains: law lost the set, education
gained, health gained only by saturation.

**The validation by intervention, and its limit.** The external check of a gene is direct. One deletes the gene $x$ and
sees whether the target $Y$ moves in the direction that the coefficient predicts. The strict criterion asks for two things: that the deletion
worked (the deleted gene itself is the smallest value of its column) and that $Y$ falls outside the range of the other
1,740 samples. In the 27 genes of the 10 sets with a gene of the replica, the deletion worked in 26 and **one** gene passed
the strict criterion. It is the pair of the domain n34, with $Y = $ YLR354C and $x_6 = $ YIL033C, whose mutant puts $Y$ at the minimum
of the 1,741 rows, in the direction of the positive coefficient $+0{,}053$. The null rate was fixed afterwards, by permutation of the
label of the mutant among the 1,741 rows, with 999 draws and seed 20261015. The observed is 1 pair in 27, the null mean is 0, and $p = 0{,}001$, the floor of the test. The
reading has two halves, and they point to opposite sides. Against the chance of the label the evidence is strong. Against reading that as a measured effect, the evidence is null. Almost all of the p comes from the
half that is a fact of the data. Deleting a gene removes its expression, so the minimum of the column is on the row of the
mutant, and permuting the label destroys exactly that half. The half that remains, the target outside the range of the other
1,740 rows, on its own gives $p = 0{,}031$, marginal.

**What invariance buys.** In verapamil, the drug that the model never saw, the invariant model predicted with
skill $+0{,}57$ and conformal coverage $0{,}89$. The model fitted by the key did a little better, $+0{,}65$.
Predicting well inside the training set and failing under a new intervention is the failure mode that invariance exists to
avoid.

**What stayed on the record and entered the counter $K$.** Besides the seven rounds, the scoreboard keeps framework v2 with its
four corrections, the criterion of G2 (the fit error of the elicitation was asked below $10^{-6}$, impossible for
asymmetric quartiles, and it became 10% of the adjusted standard deviation, with a maximum measured error of 5%), the change of
unit of the signalling experiment (the cell, and not the condition, became the unit, a change that took that
domain to the invariance test and brought the design effect down from 67 to 1.13) and the state fixed effect of the
seat-belt flow, which cut the design effect from 6.1 to 1.85.

**The multiplicity, counted.** The honest counter of the gate of H2 is of **38 distinct tests**, between 32 and 72 according to
what one counts. The inventory of the path counts still **eleven vault openings**, that is, eleven times in which a
result was seen before a design decision. With Bonferroni over 38, the level per test falls to $\alpha = 0{,}05/38 = 0{,}0013$, at the cost of
9.6 bits of parsimony, and the gate still passes in four areas. The sets shrink: in STAR,
$\{\text{small\_years\_before}, \text{small\_class}\}$ becomes $\{\text{small\_class}\}$; in yeast,
$\{\text{plate}, x_1, x_5, x_9\}$ becomes $\{\text{plate}, x_5\}$, and $x_1$ leaves, the only gene that the validation by
intervention confirmed.

**Two caveats that weigh more than the correction.** The first: dividing $\alpha$ is not monotone in the ICP. With a lower
level, domains that rejected everything start to accept some set, and invariant sets appear in models
almost rejected. The second: the approval depends on the version of the framework. With framework v1 reconstructed, STAR is
pruned before the invariance test, since the key is constant inside the environment, and H2g drops to three areas, one
of them the fragile genetics. And if only the first declared version of each choice counted, H2 would not be approved, and
round 1 gives a single area.

**The final result, after the background audit.** Pharmacology survived intact the two clustering corrections. The set
continues to be $\{c_{\text{dof}}, c_{\text{qui}}, c_{\text{ran}}\}$, and it shrinks the
evidence against the empty set, from $6{,}3\times10^{-104}$ to $2{,}2\times10^{-15}$ in the clustered test and
$0{,}002$ in the block permutation, from about 340 bits to 49 and 9. Education did not survive. With the 80 schools
as units, they are 266 accepted sets of 512 in the clustered test and 512 in the permutation, without any variable in
common, and the published set $\{\text{small\_years\_before}, \text{small\_class}\}$ has $p = 0{,}76$. Even
before the clustering correction, education was already the fragile area, with the largest p among the non-saturated sets
at $0{,}075$, close to the threshold, and $\{\text{small\_class}\}$ alone was rejected ($p = 0{,}010$). Genetics
replicated. The counting of areas closes like this. Education leaves, physics stays as it was, pharmacology
stays and genetics replicates. The gate stays standing with three areas, now with the third supported
by a new domain, and not by the area that was already fragile.

**The effect transports better than the set.** Still with the ICP rejected, the fitted model predicted the environment
never seen with positive skill in **9 domains**, from $0{,}04$ to $0{,}88$. The strong claim ("holds in any
school, airport or region") falls; the direction and the size of the effect survive. To decide, it is that weak version
that matters.

**The architecture also erred.** The review of the domain agents found **four** failures in the framework, corrected at
once in version 2: the null of filter F3 permuted at a level different from the one in which the key varies, and it started to
permute between units, inside the unit when the draw is declared; the likelihood treated the rows as
independent and came out too narrow, and it started to be tempered by $1/\text{design effect}$; the conformal prediction
was calibrated on a fifth drawn from the training set, and it started to leave one environment out; and the labels of the ICP
did not separate "rejected" from "empty accepted". Version 2 changed the status of four domains. The four corrections
came after seeing results, and count in $K$.

In the first round, with convenience environments, the design effect reached $11{,}5$ in the flights (225 thousand flights
are worth 364 days), $22{,}8$ in the cyclones and $9{,}8$ in the development study; and the literature erred in both
directions: the prior of the flights stayed below the data (odds ratio of $1{,}08$ per hour against $1{,}14$), the one of the
cyclones stayed above (15 knots per root of hectopascal against $11{,}8$), and the one of the curricula predicted "a third less"
where the data gave $2{,}3$ percentage points. Filter F2 stopped the first specification of the flights for including
the temperature, a descendant of the hour in the causal graph itself ($K = 2$), and F3 pruned the tax on beer, which with
a state fixed effect is not distinguishable from nothing ($p = 0{,}41$).

---

### Step 3 — Hands on

The laboratory of this chapter is a minimal ICP, with the two rulers of the book in a reduced version: least squares
by normal equations, residual standardised by the scale estimated without looking at the environment, Kruskal–Wallis of the location and
Brown–Forsythe of the scale, p-value by 999 permutations of the environment labels, Bonferroni over the two tests, and the
saturation rule measured by the 1-NN leaving the row out. The code is in `crates/stochology-book/src/ch19.rs` —
self-contained, with its own main function, without dependencies, with the chance embedded in ten lines of SplitMix64 (run by the
`labs` binary with `--chapter` nineteen, or directly in the module):

```bash
cargo run --release
```

There are three simulated worlds, all with three environments of 600 rows each, seed 2019, and four candidates: $x_1$
(hours of study, true cause), $x_2$ (hours of sleep, true cause), $x_3$ (pure noise) and $x_4$
(transport to school, a variable that the environment shifts and that **does not** act on the score). The outcome is
$Y = x_1 + 0{,}8\,x_2 + \text{noise}$.

| World | What the environment does | What is expected |
| --- | --- | --- |
| A | shifts the mean of $x_1$ and $x_2$ between the schools (intervention in X) | $\hat{S} = \{x_1, x_2\}$ |
| B | adds a shift of its own to the outcome (the year acts directly on Y) | $\hat{S} = \emptyset$ |
| C | world A plus $x_5$, which is the environment itself (one class per school) | $\hat{S} = \{x_1, x_2\}$, and nothing else |

```text
== Chapter 19 :: the gate of the invariant set ==
seed 2019 | 3 environments x 600 rows | 999 permutations per test | alpha = 0.05

world A: the environment intervenes in X (3 schools, 4 candidates)
  set                                p_location  p_scale    p_set  verdict       1-NN
  {}                                  0.0010   0.8500   0.0020  rejected     0.333
  {x1 study}                          0.0010   0.4260   0.0020  rejected     0.477
  {x2 sleep}                          0.0010   0.0610   0.0020  rejected     0.702
  {x3 noise}                          0.0010   0.8350   0.0020  rejected     0.343
  {x4 transport}                      0.0010   0.9010   0.0020  rejected     0.725
  {x1 study, x2 sleep}                0.9920   0.6480   1.0000  accepted     0.743
  {x1 study, x2 sleep, x4 transport}  0.9910   0.6150   1.0000  accepted     0.868
  accepted in the family: 4 of 16 (with the saturated ones it would be 4) | saturated tested: 0 | saturated accepted: 0
  S_hat with the saturation rule = {x1 study, x2 sleep}  | without the rule = {x1 study, x2 sleep}  | gate: PASSES (S_hat not empty)

world B: the environment acts directly on Y (the year, 3 candidates)
  set                                p_location  p_scale    p_set  verdict       1-NN
  {}                                  0.0010   0.8460   0.0020  rejected     0.333
  {x1 study}                          0.0010   0.6370   0.0020  rejected     0.477
  {x2 sleep}                          0.0010   0.3730   0.0020  rejected     0.702
  {x3 noise}                          0.0010   0.8280   0.0020  rejected     0.343
  {x1 study, x2 sleep}                0.0010   0.0350   0.0020  rejected     0.743
  accepted in the family: 0 of 8 (with the saturated ones it would be 0) | saturated tested: 0 | saturated accepted: 0
  S_hat with the saturation rule = {}  | without the rule = {}  | gate: FAILS (S_hat empty)

world C: world A plus x5, the column that is the environment itself (5 candidates)
  set                                p_location  p_scale    p_set  verdict       1-NN
  {}                                  0.0010   0.8500   0.0020  rejected     0.333
  {x1 study}                          0.0010   0.4260   0.0020  rejected     0.477
  {x2 sleep}                          0.0010   0.0610   0.0020  rejected     0.702
  {x3 noise}                          0.0010   0.8350   0.0020  rejected     0.343
  {x4 transport}                      0.0010   0.9010   0.0020  rejected     0.725
  {x1 study, x2 sleep}                0.9920   0.6480   1.0000  accepted     0.743
  {x1 study, x2 sleep, x4 transport}  0.9910   0.6150   1.0000  accepted     0.868
  {x5 class}                          0.9640   0.9990   1.0000  accepted     1.000
  {x1 study, x2 sleep, x5 class}      0.9410   1.0000   1.0000  accepted     1.000
  accepted in the family: 4 of 32 (with the saturated ones it would be 20) | saturated tested: 16 | saturated accepted: 16
  S_hat with the saturation rule = {x1 study, x2 sleep}  | without the rule = {}  | gate: PASSES (S_hat not empty)
```

Read it line by line. **In world A**, the empty set and all the sets of a single variable are rejected. The
p-value of location touches the floor of the permutation, $0{,}0010 = 1/1\,000$, because the shift that the school makes in
$x_1$ and $x_2$ leaves a mark on the mean of the residual. Only the sets that contain **the two** true causes are
accepted, 4 of the 16, and the intersection of them is $\{x_1, x_2\}$. The 1-NN column shows why none of them is saturated.
The hit rate of the nearest neighbour when guessing the school from the variables of the set stays between $0{,}74$ and $0{,}87$,
far below the threshold $0{,}99$.

**In world B**, the environment adds a direct shift to the outcome, and **none** of the 8 sets passes. The
invariance does not exist to be found, and the gate fails honestly. It is the simulation of the first round of H2.

**In world C**, the addition of $x_5$, a column constant inside each school and different between them, like the
copayment of a health plan, changes everything. Sixteen of the 32 sets are saturated, and sixteen of those are
accepted. Accepting is not evidence, because the test has no power at all there, and the 1-NN hit rate of $x_5$ is $1{,}000$.
Without the saturation rule, the set $\{x_5\}$ enters the family, the intersection with $\{x_1, x_2\}$ comes out empty and the
method returns $\hat{S} = \emptyset$, the opposite of the truth, with a face of rigour. With the rule, the family goes back to having 4
accepted sets and the $\hat{S}$ is $\{x_1, x_2\}$. That is what happened in the RAND insurance experiment in
H2d, where health left the scoreboard by saturation.

**The second row, measured.** The same laboratory brings the design of the validation by intervention of the yeast: the two
planted cases — the mutant that moves in the two rows and the artefact that moves in only one — and the two arrangements
run over the same world. The instrument is in `crates/stochology-horizons/src/h2/genetica.rs`, the reading in
`crates/stochology-book/src/genetica.rs`:

```bash
cargo run --release --bin labs -- --genetics
```

```text
== Chapter 19 :: the fine resolution of validation by intervention ==
design: 40 mutants | 60 individuals per line | 999 draws per test | alpha = 0.05 | Bonferroni alpha/40 = 0.001250
planted by the instrument: true mover = mutant 7 | line artifact = mutant 23

-- today's validation: every mutant is a single-line environment --
  40 mutants x 60 individuals | design floor = 0.001000 | threshold = 0.001250
  mutant  7 | effect  +0.940 | p  0.001000 | passes at the floor <- true mover (effect on both lines)
  mutant 14 | effect  -0.450 | p  0.001000 | passes at the floor
  mutant 23 | effect  +0.813 | p  0.001000 | passes at the floor <- line artifact (effect on a single line)
  mutant 29 | effect  +0.496 | p  0.001000 | passes at the floor
  passing: 4 of 40

-- the same world, with the second line: the effect must be on both --
  40 mutants x 2 lines x 60 individuals | design floor = 0.000001 | threshold = 0.001250
  mutant  7 | effect l1  +0.456 | effect l2  +1.016 | span  0.560 | p  0.000001 | passes <- true mover (effect on both lines)
  mutant 23 | effect l1  +0.607 | effect l2  +0.028 | span  0.579 | p  0.000831 | rejected <- line artifact (effect on a single line)
  passing: 1 of 40
  effect on the second line: artifact  +0.028 (does not reproduce) | mover  +1.016 (does reproduce) | in single-line validation the estimate is a point and the span is zero

reading: with one line the floor of 0.001000 and the threshold of 0.001250 let 4 of the 40 mutants through — the true mover, the line artifact and two mutants that are just noise. All of them sit on the design floor, and the p-value distinguishes none of the three from what we are looking for. With the second line the artifact leaves the list, because its effect on the second line is zero, and the floor drops to 0.000001000: 1 of the 40 mutants pass. The p was never the problem: the problem was there being a single line.
```

Read the two blocks together. In the arrangement of **one row**, the list of those that pass has four names, and only two of them
are what one looks for: the true mover and the row artefact reach the floor of $0{,}001$ through the same door, and two
mutants without any effect enter with them, because at the floor the test has no resolution to separate. It is the exact measure
of what the replica already said in prose: *one gene in one mutant*. In the arrangement of **two rows**, the effect has to appear
in both, and the list shrinks to one name. The artefact does not fall because of a large p-value — the effect of it in the first row is
$+0{,}607$, larger than the one of the mover in the same row —, it falls because in the second row the effect is $+0{,}028$, which is nothing.
The floor of the design, which was the ground of $1/1\,000$, becomes the product of the two, $10^{-6}$, and the estimate of
each effect stops being a point. **The p was never the problem; the row was the one that was single.**

**Divergence declaration.** The numbers above are of data **simulated** by this laboratory, and not of the
research repository; the three simulations show a single mechanism, at toy scale. The values of the
repository cited in this chapter (128 of 1,024 accepted sets in the light tunnel with $p = 0{,}29$ for
$\{$vermelho, verde, azul$\}$; 35 of 512 in the FDA trial; 57 accepted sets in STAR in H2d; 2 of 1,024 in
the yeast with best $p = 0{,}0072$) are not reproduced here, and cannot be: they require the real data and the
framework in Python of the horizons, whose commands are in Appendix C. The laboratory measures what it can with
its own data: that the method finds the two causes in world A, that it returns empty in world B and that the rule of
saturation is the difference between $\emptyset$ and the truth in world C. The design of the validation by intervention of the
yeast is the exception: the resolution of the replica is measured again, in Rust, by `labs --genetics`, with the true mover
and the row artefact planted by the instrument — what the instrument measures there is the **design**, and not
the 40 real genes, which continue without an instrument.

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

**Exercises.**

1. *(Concept)* Explain, with world B of the laboratory, why using the year as environment makes the invariant set
   come out empty. Which premise of the ICP does the year violate?
2. *(Concept)* In world C, without the saturation rule the method returns the empty set. Write in three sentences why
   accepting a saturated set is not evidence of invariance.
3. *(Account)* In the laboratory, the floor of the permutation with 999 draws is $1/1\,000 = 0{,}001$. What is the smallest p-value
   possible with 9,999 draws, and why does that matter for the validation of the yeast, where each mutant is an
   environment of one row? Check the account in the two-row arrangement: if each row has the floor $1/1\,000$ and the
   combined one is the product, where does the floor of the design with two rows sit?
4. *(Laboratory)* Change, in the laboratory, the shift of $Y$ of world B from $[0; 6; -4]$ to $[0; 1; -0{,}5]$
   and run again. Does the gate keep failing? What does that say about the power of the test?
5. *(Concept)* The multiplicity correction shrinks the sets and still leaves four areas. Why can it also
   create invariant sets in domains that rejected everything, and why is that a caveat?
6. *(Account)* With 38 tests corrected by Bonferroni, the level per test is $0{,}0013$ and the cost of parsimony is
   9.6 bits. The evidence of the yeast is worth 10.9 bits. By how many bits does it pass? And if the cost rose to 12 bits,
   how many areas would remain — and would the gate stay standing?

**Summary.**

- The gate of H2 asked for a non-empty invariant set in three distinct areas; the first round gave one, and
  against the claim, because the year and the region act directly on the outcome.
- The gate only passed in the sixth round, with a caveat recorded in genetics, and the five failures stayed in the counter
  $K$: convenience environments, a code error in the test, the exchange of ruler and the saturation rule.
- Two rounds checked the method against known truth. The light tunnel recovered the three LEDs and the trial of the
  FDA recovered the three drugs that block hERG, with verapamil predicted at $+0{,}57$.
- After the background audit, education leaves, genetics replicates and the gate stays standing with three areas:
  physics, pharmacology and genetics. The effect transports better than the set. The 9 domains predicted the new environment
  with skill from $0{,}04$ to $0{,}88$.

## Chapter 20. Horizon H3: the live clock, the leakage canary and the police in the vault

**Objectives.** By the end of this chapter you must know what a datum that does not exist yet is, why a test that
never found anything needs a canary to be worth something, how the Goodhart law is measured with a
randomised instrument, and why fixing the calibration of a model does not create the resolution that it does not have.

---

### Step 1 — The real world and the paradox

**Six clocks switched on at the same time.** Horizon H3 ran six real data streams as if they were
happening now: the electricity demand of the Victoria region, every half hour; the calls of a
bank, every five minutes; the daily return of the New York stock exchange; the delays of aircraft in New York; the
chance of a hurricane gaining strength; and the decision to release a detainee in Toronto. In each stream, each fact is an
**event with a timestamp**, that is, the time at which it becomes knowable. A flight that departs at 10h and arrives late only
"exists" for the system when the delay is published; a forecast issued at 8h cannot use that datum, because
it had not happened yet.

The warehouse refuses four things, and refusing is its job: reading the future, opening a vault before the written
pre-registration, opening the same vault twice and turning the clock back. In the whole replay there were 453,973 forecasts, and none
of them used future state; **zero violations**, with the count made forecast by forecast.

**The canary paradox.** "Zero violations" looks great, and it is almost nothing. It is only worth something if the test *could* have
found a violation. That is why the system planted a **canary** on purpose. It is a column identical to the label,
published at that instant, when the label becomes knowable. With the honest clock, that column appears in
no emission, and a forecaster that uses it gains nothing, with **gain 0.000**. Clock switched off, the positive
control, the column yields gains from $+0{,}86$ to $+0{,}92$ in the flights stream, $+0{,}85$ in the new year of flights,
$+0{,}55$ in the one of arrests of 2020 and $+0{,}97$ in the one of Rhode Island. It is the severe test of Mayo applied to the test
itself. The alarm rings when there is leakage, and stays quiet when there is not.

**The paradox of the good forecaster who errs about the world.** In a call centre, whoever is not served calls again. The
forecast starts to measure the effect of itself; if the model predicts a queue and the manager staffs more people, the
observed calls change. A naive forecaster, trained on what is observed, hit the observed with **1%** of error and
**overestimated the true demand by 29%**. The target metric approved a model that was wrong about the world.
That is the Goodhart loop in a pure state, and the only exit was to plant an experiment inside the loop. In 10% of the
intervals, a draw changed the staffing to 0.7 times or 1.3 times. With the randomised shocks as an instrument, the
estimated effect fell to 0.56 and the corrected forecast missed the true demand by **1.7%**, inside the limit of
3% declared before.

**The vault paradox.** Calibration is the promise that "when the model says 30%, it happens in about 30% of the
cases". It was tested in vaults out of time, opened a single time, and the expected calibration error is the
mean distance between the predicted and the observed, measured bin by bin. The table below tells the story of the three
attempts.

| Stream | What it predicts | Vault | Calibration in the vault | Gate |
| --- | --- | --- | --- | --- |
| Energy (Victoria) | demand of each half hour of tomorrow | 2014, 17,376 | coverage of 90% = 0.887 (official reserve 48% of the time) | Passes |
| Call centre (bank, 2003) | calls per 5 minutes of the next working day | Sep–Oct, 6,084 | coverage 0.900 | Passes |
| Market (Dow Jones) | 99% value at risk of the next session | 2000–2023, 6,010 | violations 0.98%; Kupiec p = 0.89; clustered (Christoffersen p = 0.02) | Passes |
| Flights (New York, 2013) | chance of a delay above 15 minutes, 2 h before | Oct–Dec, 84,301 | bias $+2{,}7$ points; ECE 0.027 | Fails |
| Cyclones | chance of gaining 30 knots in 24 hours | 2015–2022, 5,766 | bias 0.001; ECE 0.012 | Passes |
| Prisons (Toronto, 1997–2002) | chance of release | 2001–2002, 1,488 | bias 0.016; ECE 0.036 | Fails |

**The design lesson of the first failure.** In the flights, the last quarter of 2013 had more delays than the model of
60 days expected, and the bias stayed 2.7 points above the predicted one, for a tolerance of 2. In the prisons, the
calibration error was 0.036, for a tolerance of 0.03, with only 1,488 cases. And the cause became clear. **The circuit breaker
compared the model with the reserve, and not with the calibration**. The model kept beating the reserve, and that is why the
circuit breaker almost did not open, except that it was miscalibrated on average. Being better than the rival and being right are
different things.

**Second attempt: fix the level.** The correction was a continuous recalibration of the intercept, declared
before and tested in a new vault. The vault was the quarter from October to December repeated, now in 2023, with
104,489 forecasts. The flights passed, with a bias of 0.0015 and an ECE of 0.008. But the quarter of 2023 did not
repeat the deviation of 2013. The raw model, without any correction, would also have passed, with ECE 0.014. The correction
improved the calibration and **was not put to the test**.

In the prisons, the new Toronto data, from 2020 and 2021, with 65,276 arrests and a vault in the second half of 2021
over 16,933 cases, changed the question. It became registering at the police station instead of releasing. The model
hit the mean (0.0002) and separated the cases well, but it was **underconfident**. In the lower bins it predicted 0.12
where the observed rate was 0.04, and in the upper ones it predicted 0.82 to 0.90 where the rate was 0.91 to 0.94. It is an error of
**slope**, and touching the intercept does not reach that.

**Third attempt: Rhode Island.** The programme declared before a new specification, a Platt recalibration
that adjusts intercept *and* slope, and tested it in a new vault, with 24,466 stops of the state police of Rhode
Island, between 2013 and 2015, and an arrest rate of 2.26%. The question changed from "release after arrest" to
"arrest in a traffic stop", and that exchange was confirmed by a human.

| Vault 2013–2015 | Bias | ECE | Skill against the base rate |
| --- | --- | --- | --- |
| Official forecast (with Platt) | 0.0020 | **0.0048** | $+0{,}019$ |
| Raw model, in shadow | 0.013 | 0.013 | $+0{,}004$ |

It passed, with an ECE of $0{,}21\,\bar{y}$ against the limit of $0{,}3\,\bar{y}$. The vault had that criterion relative to the
rate, and the raw model, which overestimated the rate by **58%**, would have failed. That is what shows the value of Platt. The
recalibration ended with slope 1.07 and intercept $-0{,}30$, and the circuit breaker put the reserve as official in 12%
of the vault. Two caveats remain, and the first is the most important. **The skill is small** ($+0{,}019$). Arrest
in a traffic stop is a rare event (2 in every 100 stops) and little predictable from the available variables.
The calibration is good, and the **resolution** is not fixed by recalibration. It is the decomposition of Murphy in real data.
The recalibration fixed the calibration, and could not create the resolution absent from the data.

**Who pays for the error.** The fairness audit was informative in the three prison streams, and the pattern was not
the same in the three. In Toronto, from 1997 to 2002, the false positive rate of Blacks was about **double** the one of
Whites in all the years (0.16 to 0.23 against 0.06 to 0.11), in a model that **does not use colour**; it enters through the
correlates. In Toronto, in 2020 and 2021, the base rates stayed close and the calibration, the false positives and the
false negatives stayed under 0.05 between the groups. In Rhode Island, the arrest rate of Black drivers
was double the one of Whites (3.0–3.8% against 1.4–1.7%), and the false positive rate was almost double in 2013 and 2014
(0.019 against 0.011; 0.025 against 0.013), again in a model blind to colour. In Toronto, in 2002, the rates of non
release were close (21% against 19%) and the false positive of Blacks continued about double. Not using colour in the
model does not give parity, because colour enters through the correlates.

---

### Step 2 — The rule of the game

**The clock.** Each fact carries its instant of availability, written $t_{\mathrm{disp}}$. The warehouse only
delivers a row if $t_{\mathrm{disp}} \le$ instant of the request, and each agent reads only the window between the last
tick and now. Before accessing the vault, the specification is frozen in a pre-registration hash, and the vault opens one
time per stream and does not open again. The row groups of the file on disk have the size of a tick, and that is why
each table is swept at most 3 times in the whole replay, measured at 2.03 times in H3, 1.8 in H3b, 2.35 in H3c
and 1.53 in H3d.

**The rules of the calibration gate.** For a binary forecaster, in the vault:

$$|\bar{p} - \bar{y}| \le 0{,}02, \qquad \mathrm{ECE} \le 0{,}03, \qquad \text{Brier skill} > 0 .$$

The Brier skill compares the score of the model with the one of the climatologist, who always repeats the rate of the past:
$1 - \text{Brier}_{\text{model}}/\text{Brier}_{\text{climatology}}$. Zero means that the model knows nothing
beyond the base rate; the reference is a **moving** climatology, that is, the base rate of the past available at the instant of the
emission. The expected calibration error is the mean distance between the predicted and the observed, weighted by the size
of each bin:

$$\mathrm{ECE} = \sum_{k=1}^{10} \frac{n_k}{n}\, \left| \bar{p}_k - \bar{y}_k \right| ,$$

with 10 bins of equal mass. For the quantile streams, the criterion is the coverage of the 90% interval staying between
0.86 and 0.94; for the value at risk of the market, the frequency of violations staying between 0.5% and 1.5% and the Kupiec test
giving $p > 0{,}05$. Abstaining does not take the forecast out of the scoreboard, because when the model is refused the reserve is scored in
its place. Without that rule, a model could improve the score simply by giving up on the difficult days.

**The Murphy decomposition, in one sentence.** The mean squared error separates into reliability (how much the
forecasts missed the frequency), resolution (how much they knew how to separate the cases) and uncertainty of the outcome
(how rare and unpredictable the event is). The Platt recalibration only moves the **reliability**. The resolution lives
in the data and in the variables, and no output correction invents it. That is why the verdict of Rhode Island is
"good calibration, low resolution", and that is why it is honest. $\mathrm{REL} - \mathrm{RES}$ comes out positive, but the
final skill is $+0{,}019$, far from the $+0{,}05$ that the programme had asked for.

**The canary as a severe test.** Mayo asks that a test be capable of erring. The canary is the operational version
of that request, a column that *is* the label, but only becomes available when the label becomes available. With the
honest clock it enters no emission, and the gain is 0.000; with the clock switched off, the gain is from 0.55 to
0.97. The value of the leakage test is in its positive control.

**The performative loop, declared and then measured.** None of the real streams records the actions of the system,
so the loop was simulated over the real arrivals of the call centre. The staffing comes out of the forecast, whoever abandons redials with
probability $r$, and the redials enter the observed count. With $r = 0{,}6$ declared, the sensor detects the
dependence ($t = -6{,}4$, $p = 1{,}5\times10^{-10}$); with $r = 0$, the sensor does not detect it ($p = 0{,}70$). The parameter
declared was a guess. Measured later in a real call centre of 1999, the same loop is worth
$0{,}145$ [0.132; 0.157], **four times less** than the declared one. The correction was, therefore, tested in a world
more hostile than the observed world, and that asymmetry weakens the correction as proof. And the estimator of the loop had
**four versions** before passing, all recorded: bias of $+4{,}7\%$, $-5{,}9\%$, $-8{,}4\%$ and $+1{,}7\%$,
for a limit of 3%. Only the fourth passes. The first fails by design, and not by account, because staffing by the corrected demand
feeds the spiral of abandonments. It is estimator adjustment after seeing the result, the weakest point of
that gate, said without decoration.

**The board against the scoreboard.** Three attacks were tried and neutralised. The forecaster that abstains in the uncertain
zone receives the score of the reserve and stays behind the honest forecaster in the three binary streams; the forecaster constant
at the base rate receives weight 0 in the diary; and the second opening of the vault is refused. The Goodhart index, skill in the
development minus skill in the vault, was reported per stream, and it is small: $+0{,}14$ in energy, $+0{,}02$ in the
flights, and negative in the others, where the vault came out above the development.

**What the mesh produced, in operating numbers.** Drift alarms in the vault: 1 in energy, 2 in the call centre, 1 in the
flights, 1 in the prisons. Circuit breaker openings: 10 in development and 6 in the vault in energy; 4 and 1 in the flights. The
cost per loss threshold changed the decision relative to the reserve in 21% of the instants in the flights, 18% in the market and 13%
in energy. The k-anonymity of the prison dataset is 1. In that dataset, 11% of the rows are unique in the quasi-identifiers, and 2%
stay unique with age in bands of 5 years; no released cohort exceeded the budget $\varepsilon = 2$ per
person, and the privacy circuit breaker removed the cohort of 1997 from the publication of 2001. The hypergraph of the horizon has
454,178 nodes, 453,973 forecasts and 205 of topology, 51 hyperedges and 7 bridges, that is, 45% of the goal of $10^6$ that the
plan asked for.

**The calibration tolerance, measured afterwards.** The value 0.03 of the tolerance was a choice. The background audit
asked what the noise really is. For each vault, 2,000 simulations of labels with the forecasts and the $n$ of the
vault, and the 95% quantile of the expected calibration error under perfect calibration. The noise grows when the vault shrinks.

| Vault | $n$ | Observed ECE | 95% quantile of the noise | Verdict with a tolerance that depends on $n$ |
| --- | --- | --- | --- | --- |
| Flights 2013 | 84,301 | 0.0267 | 0.0046 | Fails |
| Cyclones | 5,766 | 0.0123 | 0.0087 | Fails |
| Prisons Toronto 1997–2002 | 1,488 | 0.0363 | 0.0320 | Fails ($p = 0{,}008$) |
| Prisons Toronto 2020–2021 | 16,933 | 0.0668 | 0.0115 | Fails |
| Flights 2023 | 104,489 | 0.0079 | 0.0035 | Fails |
| Rhode Island | 24,466 | 0.0048 | 0.0034 | Fails |

No binary vault passes by that rule, **not even the three approved ones**. The lesson is that 0.03 is not "the noise". In the
large vaults the noise is from 0.002 to 0.006, and 0.03 is a practical tolerance that lets detectable miscalibration
through. By the secondary rule, which requires $\max(0{,}03;\ \text{95% quantile})$, the picture of H3 goes back to what it was
before. The three approved ones continue approved and the two from Toronto continue failed. And the failure of Toronto
of 1997–2002 stops looking like noise. It was signal.

**The label drift that did not warn.** The audit also asked whether two classical tests of label drift
would have seen the deviation of 2013 before the original sensor. They did not. The test that compares the realised rate with the predicted
mean switched on in the vault on 6 November 2013, and the original sensor had already switched on 16 October; worse, it
stayed on in 95% of the checks of the 2013 development, because with a window of three thousand flights the delay rate oscillates more than
0.02 from one day to the next. A permanent alarm warns nothing. The correction held the only real
deviation that appeared, in the third quarter of 2017, when the raw model marked $+0{,}028$ and the corrected one,
$+0{,}001$. But the declared stress did not discriminate, because the daily readjustment absorbs the drift alone at the level of the
quarter. The raw model also passed. That is stress, not proof.

**The inverted audit, and who pays.** The question of the fourth invariant, "who pays for the error?", was run
inside out in the three prison streams. The **chosen** metric is the false positive rate of the marker,
$P(\text{marked} \mid \text{does not suffer the outcome})$; the **inverted** one is the false negative rate,
$P(\text{not marked} \mid \text{suffers the outcome})$. The least favoured group comes out of the largest base rate, the payer is
the one with the largest rate under the metric in force, and a tie stays with the cases with overlapping confidence intervals. The sweep of
thresholds goes from 0.05 to 0.95, plus the declared threshold of each stream: 0.70 in the two Toronto streams and 0.90 in
Rhode Island.

In Toronto, from 1997 to 2002, the least favoured group is the one with the **smallest** false negative rate. At the declared
threshold of 0.70, the Black person loses 0.64 of the cases in 1998 against 0.70 of the White person, while by the chosen metric
who pays is the Black person (0.196 of false positive against 0.105) and by the inverted one the Black person leaves, leaving the tie or the
White person. Whoever is marked more often without suffering the outcome is the Black person; whoever is missed more often is the
White person. Changing the metric changes who pays, and in that stream that happens at one threshold of the grid per year, all **above**
the declared one: 0.85 in 1998, 0.50 in 2000 and 0.80 in 2001, and in none in 1999 and 2002. In the stream of 2020 and 2021 the
exchange appears in the second half of 2020, in 5 of the 19 thresholds swept (0.35 to 0.55), all below the declared one;
at the threshold of 0.70 of the stream, the four halves tie under the two metrics. In Rhode Island there is no exchange at
any threshold of the sweep. The least favoured group pays under the chosen metric in 2014 and ties in 2013 and 2015,
and its false negative rate stays between 0.850 and 0.927 at the threshold of 0.90.

Two design caveats close the chapter. The first: in Rhode Island the question changed from "release after
arrest" to "arrest in a traffic stop", and the exchange was confirmed by whoever runs the programme, and not by the
group that became the target of the decision. The second: the record of the affected groups is short and is not good, because in
seven decisions that involve people **no affected group was heard**. Chapter 21 returns to that point.

**Divergence declaration (the Platt balance).** The record of H3 says that, in the flights stream of 2023, the
intercept of the correction varied from $-0{,}52$ to $+0{,}43$ along the year. The diary of 2023 shows that this is the
interval **inside the vault**; in the whole year, the same intercept goes from $-1{,}01$ to $+1{,}25$. The two values
are right and measure different things: one is the vault, the other is the year. It stays on the record instead of hidden.

**Divergence declaration (the numpy case).** The published number of the performative loop — estimated effect
$0{,}5632$ with 2,375 shocks — was contested once, and it is worth telling how it ended. The code, with the seed
declared, gave $0{,}5529$ and 2,395 shocks when it ran with the standard `python3` of the machine. There were not two versions
of the code. The file is one, and the four copies of the tree bring the identical cryptographic digest. What there was were
two numeric packages inside a single interpreter: numpy 2.5.1 of the record and numpy 2.4.6 that the standard path
imports first. The exact point of the divergence was found: with the state of the generator identical, a binomial draw
of 248 trials with probability 0.6 returns 171 in one version and 150 in the other, because version 2.5 corrected the
Stirling series of the acceptance test of BTPE — the third and the fourth terms were added instead of subtracted, and the
main coefficient said 13680 where it should say 13860. From there on the two simulations decorrelate, and the
exploration draw changes pattern. **No published number changed**: the r̂ of 0.5632 with 2,375 shocks reproduces
in the record environment, and the other value is the one of the other package. The lesson for whoever goes to check: record the version of the
environment together with the number, because a number without an environment is a number that nobody can audit.

**What stays open.** The service time and the patience of the clients of the call centre continue assumed, because the
dataset brings neither. The performative loop is a declared loop over real arrivals, and not a measure of the
world. The latency of the cyclone bulletin (3 hours) and the one of the taxi of the flights (20 minutes) are assumptions. And the horizon
is not a service running: it is a replay under a clock.

---

### Step 3 — Hands on

The laboratory of this chapter assembles the three pieces on which H3 depends, the Platt scaling, the expected calibration
error and the canary, over simulated forecasts, with the chance embedded in ten lines of SplitMix64. The logistic regression is
fitted by reweighted least squares, which is deterministic and needs no library. The code is in
`crates/stochology-book/src/ch20.rs` (run by the `labs` binary with `--chapter` twenty, or directly in the module):

```bash
cargo run --release
```

The world is a traffic stop. The outcome is "there was an arrest", with three covariates (night, high speed,
plate of another state) and an additive model in log-odds. The training period (2005–2012) has a higher base
rate; the development (the period before the vault) and the vault (2013–2015, 24,466 stops) have the same distribution and
rate, about 2.2%. The raw model is trained in the old period and applied to the vault, and the Platt correction is
fitted in the development, where the labels are already known.

```text
== Chapter 20 :: the clock, the canary and the vault ==
seed 2021 | training 120000 stops (rate 0.0352) | development 58000 (rate 0.0222) | vault 24466 (rate 0.0220)
raw model (trained in the old period of higher rate): predicted mean 0.0351 (60% above the vault rate)
  CITL = +0.0132 | ECE = 0.0132 | Brier = 0.02155 | skill = -0.0020
official model with Platt (a = 0.98, b = -0.54): predicted mean 0.0222
  CITL = +0.0002 | ECE = 0.0022 | Brier = 0.02130 | skill = +0.0098
climatology fixed at the past rate: Brier = 0.02151 (p(1-p) = 0.02171)

canary: a column equal to the label, published at the instant of the label
  clock honest: canary available in 0 of 24466 emissions | beta_canary = +0.00 | skill gain = +0.0000
  clock switched off: canary available in 24466 of 24466 emissions | beta_canary = +9.23 | skill gain = +0.6521
```

Read it line by line. **The raw model** predicts, on average, 0.0351 where the rate of the vault is 0.0220, that is, 60% above.
That excess appears in the two metrics at the same time, since the bias is worth $+0{,}0132$ and the expected
calibration error is worth 0.0132, and the deviation is almost all of **level**. The model learned the base rate of the old
period and did not notice that the world changed. The Brier skill comes out negative ($-0{,}0020$), since the model loses
to the climatologist that only repeats the rate of the past. **The official model**, with Platt ($a = 0{,}98$, $b =
-0{,}54$), goes back to the right level, with bias $+0{,}0002$ and ECE 0.0022, and skill $+0{,}0098$. Note the point at which
the correction did **nothing**. The skill rose from $-0{,}0020$ to $+0{,}0098$, and not to $+0{,}5$. The slope
hardly changed (the fit stayed at 0.98), and the resolution continues to be the one of the data. The calibration is fixed;
the resolution is not. **The canary** closes the laboratory. With the honest clock, the canary column becomes available in
**0** of the 24,466 emissions, its coefficient stays at zero and the skill gain is 0.0000; with the clock
switched off, it is available in the 24,466 emissions and the gain is $+0{,}6521$.

**The H3 chain, re-executed as far as the tree allows.** What happens to the four rows of this horizon
when the Python scaffolding leaves the tree? Each one receives a class, and the instrument **knows how to reject** a row that
declares itself of a class that the data does not support: **measured**, when the raw data is in the tree and the thing is measured
again; **derived**, when the number is in a versioned file and the instrument redoes the account and the verdict; and
**record**, when the raw data is not here and all that remains is to declare what was measured. The code is in
`crates/stochology-horizons/src/h3/cadeia.rs` and the record in `empirical/p14-h3/registro.tsv`:

```bash
cargo run --release --bin labs -- --h3
```

```text
== Point 14 :: the H3 chain, re-executed as far as the tree allows ==
record stamp: 2026-10-16
line             class     mark versioned record                     what the instrument does
h3.fluxos        record    P    -                                    declares: the raw data is not in the tree
h3.ri            record    D    -                                    declares: the raw data is not in the tree
h3.laco          derived   V    empirical/p06-laco/resultados.json   re-derives the account and the verdict
h3.hipergrafo    record    P    -                                    declares: the raw data is not in the tree

-- the performative loop, re-derived from the record in the tree --
2SLS estimate: r = 0.1446378859 | 90% CI from 0.1319811884 to 0.1568385251 | first-stage F = 5523.46
ratio declared r / measured r: 4.1482907207 (the record brings 4.1482907207) | declared band: 0.3333 to 3.0000
sign matches: true | order of magnitude matches: false | compatible in the strict sense: false (the record says false)
bias of the naive forecaster against exogenous demand: 0.2912
divergences between the account and the record: none
verdict: the H3 loop overestimates the measured one: the sign matches and the order does not (ratio 4.1483, band from 0.3333 to 3.0000)

-- negative proof: a record in which the ratio falls inside the declared band --
  with measured r 0.4: order matches = true | verdict: the H3 loop has the sign and the order of the measured one (ratio 1.5000 inside the band)
-- negative proof 2: a record in which the ratio field does not match the account --
  divergences: ["the record's ratio (2.000000) does not match the account (4.148291)", "the record's containment (true) does not match the account (false)"]
```

Read the table header of each row. **The six streams** and **the hypergraph** stay in the record class: the
diaries and the graph left the tree with the scaffolding, and what exists in Rust for them is the **mechanism** — the canary
of this chapter, which measures the clock, and not the six streams. **The vault of Rhode Island** is also record, and that is why it
continues with the **D** mark: the laboratory measures the mechanism, and the divergence against the real vault is declared three
paragraphs above. **The performative loop** is the only derivable row, because the record of its measure is
versioned: the instrument reads the `resultados.json`, redoes the account of the criterion that was declared before running and
reaches the same verdict — the loop declared at 0.6 is **4.1483 times** the measured one, outside the band from 1/3 to 3, with zero
divergence between the account and the record. It is the difference between "trust the number that is in the chapter" and "run the
command and see the account redone".

**Divergence declaration.** The experiment above is simulated and was calibrated to have the structure of the vault of
Rhode Island — rate of 2.26%, 24,466 stops, correction fitted in the previous period —, not to reproduce its
numbers. The values measured here and the ones of the repository, side by side: bias of the raw model, $+0{,}0132$
here against $0{,}013$ there; bias with Platt, $+0{,}0002$ here against $0{,}0020$ there; ECE with Platt, 0.0022
here against **0.0048** there; skill with Platt, $+0{,}0098$ here against **$+0{,}019$** there; excess of the rate by the
raw model, 60% here against 58% there. The order of magnitude and the sign of all of them coincide, and the reason for the
differences is the data: the one here is generated by a simple additive model, with three covariates and without the noise of a
real police record. The gain of the canary with the clock switched off, $+0{,}6521$, falls inside the range measured in the
real vaults, which goes from $+0{,}55$ (arrests of 2020–2021) to $+0{,}97$ (Rhode Island).

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

**Exercises.**

1. *(Concept)* Why does a canary that gives no gain with the honest clock prove little if nobody tests it with the
   clock switched off? Give an example of a column that would be a bad canary.
2. *(Concept)* Explain the failure of the circuit breaker in the flights of 2013 and propose a second criterion of opening based
   on calibration, saying what price it charges in number of forecast changes.
3. *(Account)* In the laboratory, the fixed climatology has Brier 0.02171 and the official model, 0.02130. Compute the
   skill and check it against the measured number of $+0{,}0098$ (the measured reference uses the rate of the past, 0.0222, and
   not the rate of the vault).
4. *(Laboratory)* Change, in the laboratory, the rate of the training period bringing it close to the rate of the vault (change the
   old intercept from $-4{,}005$ to $-4{,}30$) and run again. Does the raw model become well calibrated? What does
   that say about the value of Platt in that design?
5. *(Concept)* Why does the recalibration of the intercept not correct a model that is underconfident at the extremes?
   Describe the calibration curve of the two cases, with the axis of the forecasts against the one of the frequencies.
6. *(Concept)* In Toronto from 1997 to 2002, changing the metric changes who pays. Write, in four sentences, why
   that is not an error of account and rather a decision of value.

**Summary.**

- Zero leakage is only worth something with a canary that proves that the test would find the leakage: 0.000 with the honest clock,
  from $+0{,}55$ to $+0{,}97$ with the clock switched off. And of the four rows of this horizon, only the performative loop
  can have the account redone today: the other three lost the raw data with the scaffolding, and what remains of them is the
  mechanism measured here.
- A forecaster can hit what it observes (1% of error) and err about the world (29% of excess); the instrument randomised
  inside the loop undoes the spiral, and the loop declared at 0.6 is four times the measured one, 0.145.
- Each calibration correction was only accepted in a new vault: flights of 2023 with ECE 0.008, prisons of 2020–2021
  failed by slope (0.067) and Rhode Island approved with Platt, ECE 0.0048 and skill $+0{,}019$.
- Calibration is fixed; resolution stays where it is. And with a tolerance that depends on the size of the vault, none of the six
  binary vaults passes — not even the three approved ones.
- Changing the metric of fairness changes who pays for the error; and none of the affected groups was heard in the seven
  decisions that involve people.

## Chapter 21. The seven lessons, the ten points of the audit and what remains to be done

**Objectives.** By the end of this chapter you must know how to summarise the seven lessons of the programme in everyday situations,
to read the balance of the ten points of the background audit with the result of each one in hand, to name aloud the
questions that remain open and without answer, and to conduct the final project. To choose a problem of the school or of the
city, to fill the card, to simulate the world and to defend the conclusion.

---

### Step 1 — The real world and the paradox

**The question that closes the course.** In the last meeting, someone asks what the course was worth in real life. Not
what it was worth for whoever is going to do statistics, but for whoever is going to decide. The answer of the programme has seven sentences, and
each one was born from a concrete case. The seven come told the way they happened.

1. **The mean hides the essential.** In the health post, the mean demand is identical in the two worlds, and the wait
   jumps from 9.3 minutes in the constant world to 20.7 minutes in the world that oscillates. In every 100 patients, 25 wait
   more than half an hour in the world that oscillates, against 8 in the constant world. In a call centre, the schedule
   dimensioned by the mean fails in **26 of every 100 days**. And the mean wealth of whoever bet 3 times in the market
   appears as 19 times, when the median is 2.1.
2. **The independent unit decides the margin.** 2.65 million calls are worth **81 days**; 225 thousand flights are worth
   **364 days**. Counting row by row, where the variation repeats, separates an honest margin from a margin that only
   looks narrow.
3. **Hitting is not enough; one must have been able to err.** The propensity fit in the LaLonde experiment hit the
   number, and an innocuous treatment would have reached a similar value. In the 1975 placebo it invented a phantom
   effect of $-12{,}3$ percentage points. A number that hits without being able to err is not evidence.
4. **Failing one's own model is a result.** In the Danish fires, 36 months without a single giant fire
   are worth only **2.2 bits** of information. The monthly chance stays near $1/38$ by the Laplace rule, while the
   insurer tripled the bet. When the model of the programme was tested in the vault, it lost to the simple historical
   mean (skill $-0{,}015$) and **did not win the right to decide**. The claim of the insurer fell.
5. **The effect transports better than the list of causes.** Exact invariance is rare. When it appears and is
   checked against a known truth (the light tunnel and the hERG blockers), it protects against the new
   intervention, as verapamil showed. When it does not appear, the direction and the size of the effect still usually survive.
   9 domains predicted the never-seen environment with skill from 0.04 to 0.88.
6. **Calibration is fixed, resolution is not.** Three vaults were needed for the calibration to pass the
   tolerance of 0.03, and the audit showed that this tolerance was loose in the large vaults. The final skill
   stayed at $+0{,}019$. Saying "it is well calibrated" does not ensure that the model separates the cases well.
7. **Declaring before running makes the failure useful.** Seven rounds in H2 and three attempts in H3 only count as
   science because each change was written with a hash **before** running, and because each adjustment made after seeing a
   result stayed on the record and counted in the counter $K$.

**The programme became the target.** A discipline that demands a severe test of the claims of others needed to go through the
test that it applies to others. The background audit did that. Ten assumptions of the programme, things that the text
affirmed and that had never been put to the test, became fronts of work, each one submitted to the house rule:
declaration with a hash before running, result reported with the limits, and every change of criterion after seeing the
result entering as a new declaration and counting in $K$.

The general result is uncomfortable and worth the reading. **No published number fell by an error of account**, and the
audit shrank the authorisation of those numbers. The central thesis of the book tied in the direct test. The tolerance of
calibration was loose. The declared Goodhart loop was four times larger than the measured one. And the gate of H2 depends
on which version of each choice counts.

**Whom the programme did not hear.** In each one of the seven decisions that involve people (cutting the benefit of
work accident victims in Kentucky, reducing classes in Tennessee, changing the copayment in the health insurance,
comparing the job training of the NSW with controls of the CPS, marking the detainee in Toronto in the two periods and marking the
driver in Rhode Island), **no affected group was heard**. There is no consultation, interview, panel
nor consent in the repository. The list is large and has name and number: 11,598 students in 80 schools; 5,226 detainees; 65,276 arrests
of 2020 and 2021; 86,113 traffic stops; 185 participants of the NSW training and 15,992 controls. Signing is not
listening. Whoever signs an opinion stands on the side of whoever decides to use the artifact; whoever pays for the error stands on the other side.

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

---

### Step 2 — The rule of the game

**The seven lessons, with the condition in which they hold.** The first three lessons are arithmetic of decision. The mean of
a function is not the function of the mean; the margin depends on the unit counted as independent; and a number is only
evidence if the procedure that produced it could have come out wrong. The two following ones deal with the gain that comes from
the loss: failing the model that one built is a publishable result, and invariance protects against the new
intervention, while the effect, alone, only promises to transport on average. The last two concern the process. The
Murphy decomposition separates reliability from resolution, and the previous declaration separates science from cheating. None
of the seven holds without its condition. The sixth only holds for forecasters that cannot be improved by more data, and the
seventh only holds if the declaration is written before and the counter $K$ is published afterwards.

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

**The seven objections, and the answer to each one.** A discipline that demands a severe test needs to survive the objections
that are made to it.

| Objection | Answer |
| --- | --- |
| "This is just statistics under another name." | In part, yes: almost all the tools have an owner for decades. What Stochology adds is the organisation — the layers, the typing of uncertainty and the same gates for every claim. The first direct test of that, the blind pilot, **tied at the ceiling**: the two arms pointed to the same 8 defects and the same 0 false alarms. A tie at the ceiling neither supports the thesis nor refutes it |
| "The data are old and public." | It was a choice: known datasets, verifiable by hash and with a known outcome allow auditing the method, but do not replace a real operation. Several pieces remain assumed — the service time, the patience of the clients, the panel that was never consulted. Others stopped being assumptions: the performative loop, the tolerance of the calibration and the fragility of the genetics were measured |
| "Formalising judgement is eliminating judgement." | The architecture does not decide what is worth asking nor how much an error costs for whoever suffers it. Those values enter through the elicitation and through the ratio between cost and loss, declared and not computed. The ambition is to make judgement visible |
| "The apparatus is too heavy." | Eight agent bands, five lenses and five invariants are a lot for a simple question. The funnel answers: most of the hypotheses fall cheaply, before touching the data. If the cost of the machine exceeds the value of the decisions that it improves, the lens of judgement itself orders it switched off |
| "Bayes solves the problem of Hume." | It does not. Bayesian updating is the coherent rule inside a space of hypotheses, but induction moves to the choice of the prior and of the space, as the Laplace rule showed. The Cox theorem itself depends on technical hypotheses (Halpern, 1999) |
| "Ergodicity became doctrine." | The lens of time relies on the programme of Ole Peters, which is contested, and the discipline teaches it as a debate. By the criterion of growth, the loss aversion of about 2.25 measured in the laboratory is only justified when 56% of the wealth is at stake: the thesis explains the refusal to risk half a patrimony, not the aversion to small bets. Something similar holds for the tails: Taleb asks that a heavy tail be assumed until proof to the contrary; Clauset, Shalizi and Newman ask for proof before declaring a power law. The question is whose is the burden of proof, and the answer depends on how much it costs to err |
| "The auditor is a party with an interest." | The 21 opinions were signed by whoever runs the programme, and naming someone responsible does not make the audit independent. The background audit added a cheaper and weaker check: reimplementing from zero four results that support the approved outputs, from the specifications. The four agreed and no error was found — but the test has a declared defect, because several choices that the specification does not fix came out equal to the ones of the original code, three of them only in STAR. Real independence asks for people from outside, with the same contract |

**The card demands five questions and four prohibitions.** The instrument that turns a doubt into an auditable
claim asks five header questions. What is the **generative process** of the data, with seed; where does the
**non-linearity** live; what is the **independent unit**; what is the **physics of the noise**, whether the noise enters added
(Itô) or whether the variable is the one dragged (Stratonovich); and at which step of the **causal ladder** the claim lives.
Four prohibitions close the list: no free natural language between agents, no solitary mean where there is
curvature or heavy tail, no confusion between learning and performativity without a counterfactual, and no dogmatic extreme
outside the interval from 1% to 99% nor omission of attempts. Five invariants and seven automatic
gates verify the card, and the gate binary exits with code 0 or 1.

**The script of the final project.** The project is worth as much as the whole course. It is four weeks of work for a
problem of the school or of the city. The script has eight steps, and the final product is a memo in natural
frequencies, with an exit clause, defended before a board.

1. **Choose the problem.** A problem in which someone decides something and there is a concrete action serves, such as reducing
   classes, changing the time of the break, installing a drinking fountain, fining double-parked cars or changing the
   bus stop. "Understanding the violence in the city" does not serve: there is no decider and there is no action.
2. **Write the decider question**, in three parts: *Who decides*: name the person or the body; *Action that the
   decision changes*: copy the action literally; *The decider question is*: end with a question mark. Without those three
   parts the card is refused, and the laboratory of this chapter shows the verifier that refuses it.
3. **Fill the card.** The five header questions, the claim in one sentence, the scope term (where it holds and
   where it does not hold), the quantity measured and the unit.
4. **Fix the generative process** with a numeric seed. Which elements are drawn, which are fixed, how many
   rows, which environments. The seed allows another person to see that world.
5. **Build the null model.** What chance alone would produce in that design, with the seed repeated. Without a
   null there is no contrast.
6. **Pre-register with a hash** and declare the vault. The partition of data that nobody saw stays guarded, and opens one time
   only. If the criterion changes after opening, the new criterion is a new declaration and enters the counter of attempts.
7. **Run, measure and declare.** All the numbers of the memo come from code that another person runs; every attempt
   stays on the scoreboard; every mental arithmetic is forbidden.
8. **Write the memo and defend it.** Natural frequencies ("in every 100 students"), the chart of the hypothetical
   results, the objective exit clause, the one that says when the recommendation stops holding, and the answer to the
   final question, when a model is good enough. The book answers with three conditions. A model is good
   enough when it declares the failure modes, when someone from outside manages to reproduce every number, and when the
   decision that it authorises does not depend on any assumption that has not been written.

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

**Four horizons remain named.** **Resolution of the question:** the decider question has a grain, and the grain is a variable of the model. R0 asks about the average day, R1 about the day that exists, R2 about the hour and R3 about the subgroup. With the identical total resource and paired arms, asking coarse costs **46.24 min** of mean wait where the demand oscillates from 5 to 11, **5.97 min** where it oscillates from 7 to 9 and **nothing** where it does not oscillate (`cargo run --release --bin labs -- --resolution`, in `crates/stochology-core/src/resolucao.rs`). It is the new axis, orthogonal to H0–H3. Its falsifiable predictions are measured: the oscillation threshold appears at amplitude 2.5 with the calibrated plan — loss of $+2{,}87$ min at the crossing and negative below it — and **does not** appear with the Erlang plan; the transfer has no violation in 6 profiles, with 3 counterexamples that pass in R0 and fail in some day (`labs --resolution`). **From the argument to the artifact:** the unit of knowledge of this programme is the executable artifact, and not the paragraph. Claim, generative process, null model and test stay side by side. **Artificial agents with epistemic discipline:** language models enter with an evaluation rubric ($\kappa = (p_o - p_e)/(1 - p_e)$), a record of the version and of the date of the supplier, and the prohibition of free text between agents that already holds in the card. **Expiry date for the goal of the book:** every metric of this book is declared with the date on which it starts to hold, and the measured Goodhart (Chapter 20) is the reason to declare the deadline before.

---

### Step 3 — Hands on

The laboratory of this chapter is the **verifier of the final project card**: a small gate that refuses a card
without the decider question, without a seed and without an exit clause, and proves that it knows how to reject, because each one of the seven
rules brings down at least one card. The code is in `crates/stochology-book/src/ch21.rs`, self-contained and without
dependencies:

```bash
cargo run --release
```

```text
== Chapter 21 :: the verifier of the final project card ==
seven rules, and each one has to reject at least one card:

  1. R1 generative process declared
  2. R2 independent unit declared
  3. R3 null model with the same seed
  4. R4 decider question (who decides, which action, question)
  5. R5 fixed numeric seed
  6. R6 pre-registration hash with 64 hexadecimal digits
  7. R7 exit clause with an objective trigger

01 complete                  ACCEPTED (who decides: the municipal education secretariat)

02 without the decider question REJECTED
    R4 field decider_question missing

03 generic who decides       REJECTED
    R4 who decides is generic

04 the action does not appear literally REJECTED
    R4 the action of the decision does not appear literally

05 without a question mark   REJECTED
    R4 the text does not end in a question

06 without a seed            REJECTED
    R5 no fixed numeric seed

07 without an exit clause    REJECTED
    R7 no exit clause

08 clause without a trigger  REJECTED
    R7 exit clause without an objective trigger

09 without pre-registration  REJECTED
    R6 pre-registration hash missing or out of format

10 without a null model      REJECTED
    R3 null model missing or without the same seed

11 without a generator       REJECTED
    R1 generator field empty or too short

12 without a unit            REJECTED
    R2 unit field empty

12 cards: 1 accepted, 11 rejected
rules that rejected at least one card: 7 of 7 | rejections per rule: [1, 1, 1, 4, 1, 1, 2]
```

Read it line by line. **Card 01 is accepted** with who decides named, the action copied literally from the decision, the
question ending in a question mark, the seed 2026 in the generative process and in the null model, the cryptographic digest of
64 digits and the exit clause anchored in a number ("above 0.10 sd for three consecutive years"). **The other eleven
are refused, and each refusal is a distinct defect.** R4 appears four times because the decider question is the
rule with most parts: missing, generic ("the decider" names nobody), without the literal action and without the question mark.
R7 appears twice: one card without any clause and another with a clause without a trigger ("while the model
keeps being good and nobody complains"), which does not say when the recommendation stops holding. The last line is the count that
matters. **7 of 7 rules rejected at least one card**. A gate that never rejects is not a gate; it is a
rubber stamp.

**Divergence declaration.** The verifier of the laboratory is a reduced reimplementation of the validator of the
repository. The validator of the research demands, besides the rules above, at least 60 characters in the text of the question,
refuses a larger list of generic names and runs over 18 packages of H2g, 12 of H2 and the card of H0, without any
published number changing; the one of the laboratory runs over 12 test cards and touches nobody's data. What
the two have in common is the essential: the decider question is only worth something if it names who decides, copies the action
literally and ends in a question.

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

**Exercises.**

1. *(Concept)* Choose a problem of your school and write the decider question in the three required parts. Then
   change the decider for "the decider" and explain, in two sentences, why the card would stop being auditable.
2. *(Concept)* Write an exit clause with an objective trigger for a decision of your city and a clause
   without any trigger. Which of the two survives a change of management?
3. *(Laboratory)* Run the verifier and add a card 13 with a correct decider question and a seed that is
   not numeric ("any"). How many cards are refused now, and by which rule?
4. *(Account)* In the balance of the audit, five of the 33 domains change status or action when the prior of
   the literature replaces the one of the package. Write that number as a natural frequency ("in every 100 domains...") and
   say why it is the size of what remains to be measured.
5. *(Concept)* The blind pilot tied at the ceiling: each arm pointed to 8 defects, and the two lists came out identical.
   Explain, in four sentences, why a tie at the ceiling neither supports the thesis nor refutes it.
6. *(Essay)* Answer the question that closes the course: when is a model good enough? Use at least three
   numbers of this book.

**Summary.**

- The seven lessons summarise the programme: the mean hides; the independent unit decides the margin; hitting is not enough
  if the test could not fail; failing the model that one built is a result; the effect transports better than the
  list of causes; calibration is fixed and resolution is not; declaring before running makes the failure useful.
- The background audit tested ten assumptions of the programme and the extensions. No published number fell by an error
  of account; the audit narrowed the authorisation of those numbers. The central thesis tied, the tolerance of calibration
  was loose, the declared loop was four times the measured one.
- They remain open, with a name: the elicitation with human experts (5 of 33 domains depend on the prior), the
  second row in the 40 real genes of genetics (the design already has an instrument: 4 of 40 pass with one row, 1 of
  40 with two) and the voice of whoever pays for the error (no affected group heard in the seven decisions that involve people, with
  the box sealed by instrument and a deadline on 2026-12-15).
- The final project closes the course: choose a problem of the school or of the city, write the decider question,
  fill the card, fix the seed, build the null, pre-register, run and defend a memo in natural
  frequencies with an exit clause.

# Appendices

## Appendix A. Visual dictionary of Stochology

The dictionary gathers every term of the book, each defined in one simple sentence and illustrated with an everyday
example. The order is alphabetical.

| Term | In one sentence | Everyday example |
| --- | --- | --- |
| **Abduction** | Choosing, among the hypotheses considered, the one that makes what was seen least surprising | The noise in the engine wakes the neighbour: the simplest explanation is the car, not a drone |
| **Agent ($L_3$)** | Whoever decides, with limited time, attention and money: a person, an institution or an algorithm | You choosing the way home at 7 a.m., with the tank on reserve |
| **Filtered sample** | A sample that contains only the cases that survived some filter, taken for the whole population | Counting how many famous musicians dropped out of college and concluding that college gets in the way |
| **Pathology edge** | The mark, on the map of knowledge, of a misalignment between the layers of the world, the model and the agent | Two folders of the same client with different phone numbers, and nobody knows which one counts |
| **Background audit** | A front that tests an assumption of the program itself, with a declaration written before running | The teacher who redoes the exam before applying it to the students, with the answer key sealed |
| **Adversarial panel** | The stage in which someone attacks the work live, looking for where it breaks, before approving | The practice in which the coach makes the reserve team play against the starters, without mercy |
| **Sheet bus** | The channel through which the pieces of the work exchange sheets, chained by cryptographic digest, so that nothing can be rewritten without breaking the chain | The minutes of a meeting signed page by page: changing one page invalidates the signature of the following ones |
| **Bayes (rule of)** | The rule that says how today's belief is updated with the evidence of now, starting from yesterday's belief | A positive pregnancy test in a low-risk person can still be a false positive most of the time |
| **Brier (score of)** | The mean of the squared error of a probabilistic forecast: lower is better, and 0 is perfect | Betting 90% on a game and losing costs more on the scoreboard than betting 55% and losing |
| **Calibration** | The property of whoever says "30%" and gets it right in about 30% of the cases | A meteorologist who says "chance of rain" 100 times and it rains in 62 of them is miscalibrated |
| **Canary** | A leak planted on purpose to prove that the leakage test works | The test camera the shop triggers on purpose to check whether the alarm rings |
| **Heavy tail** | A distribution in which extreme events are far more frequent than the bell curve would suppose | Days of −10% on the stock market exist; in a Gaussian world they would almost never happen |
| **Exit clause** | The objective trigger that says when the recommendation stops holding | "I review the medicine if the fever stays above 39 °C for two days in a row" |
| **Vault** | The part of the data that stays closed until pre-registration, and opens a single time | The sealed envelope of the exam, opened only after the student hands in the answer |
| **Confounder** | A common cause of two variables that makes it look as if one causes the other | Ice cream and drowning rise together: both rise with the heat |
| **Counter $K$** | The tally of the number of specifications tried in order to answer the same question | The number of attempts written in the margin of the notebook to get a password right |
| **Crisonomy** | The lens of judgement: value of information, proper scores, calibration and cost of the error | Paying for one more exam before deciding whether the surgery is worth it |
| **Chrononomy** | The lens of time: the average of many people against the average of one life, ruin and growth | Whoever bets half the estate every month goes broke before harvesting the pretty average |
| **Design (effect of)** | How many dependent rows are worth one independent row | The same 30 people answering 10 times are not worth 300 people |
| **Circuit breaker** | The mechanism that swaps the model for a safe reserve when it loses over a moving window | The autopilot that switches itself off when the sensor disagrees for too long |
| **$\operatorname{do}(x)$ (intervention)** | Fixing a variable by decision, and not merely observing who already has it | Drawing lots for who receives the medicine, instead of comparing those who were already taking it with those who were not |
| **ECE (expected calibration error)** | The mean distance between predicted and observed, computed by confidence bands | Estimating the error of a thermometer band by band, from 35 to 42 degrees |
| **Erlang C** | The classical formula that estimates how many attendants are needed for a queue | How many checkouts to open at the supermarket so that the queue does not exceed five people |
| **Type I error** | Raising the alarm when there was nothing | The smoke detector that goes off while the bread is toasting |
| **Type II error** | Not raising the alarm when there was something | The detector with a weak battery that stays silent in the fire |
| **Type III error** | Answering with precision a question that was not the decision-maker's | Delivering the average grade when the question was how many students stay for remedial classes |
| **Type M error** | Getting the size of the effect wrong, too large or too small | "3 centimetres of rain" when it was 30 millimetres |
| **Type S error** | Getting the sign of the effect wrong: saying it improves when it worsens | Recommending a medicine that gets in the way, because of a signal noise |
| **Scalonomy** | The lens of scale: tail exponent, existence of the mean, effective size | Measuring whether a flood can swallow the historical average of floods |
| **Proper score** | The score in which declaring one's own sincere belief is the best move | The pool in which guessing the most likely outcome, and not your heart's team, is what scores |
| **EVPI and EVSI** | How much perfect information is worth, and how much the information from one sample is worth, in expected money | How much you would pay to know today tomorrow's demand, before buying the stock |
| **Sheet** | The closed list of questions asked of a text, a number or a model | The mechanic's checklist: without it, the same item is always forgotten |
| **Fluxonomy** | The lens of the current: how probability and information flow, and where they are lost | The telephone game: the message loses bits at every repetition |
| **Natural frequencies** | Stating the risk as "x in every 100", with a fixed denominator | "In every 100 patients, 25 wait more than half an hour" instead of "25% risk" |
| **Goodhart (law of)** | When the measure becomes the goal, it stops measuring | The school that trains the students for the exam and the grade rises without learning rising |
| **Brier skill** | How much the model improves, in percentage, over the climatologist who only repeats the base rate | How much the rain forecast gains over "it rains as often as always" |
| **Typed hypergraph** | A map of knowledge in which the links have a type and can join more than two things | The subway map with named lines, and not only points joined by strokes |
| **ICP (invariant causal prediction)** | The method that returns the set of variables whose relation to the outcome does not change across environments | The recipe that works in your house and in your grandmother's, with a different oven |
| **Meng's identity** | The equation that says how much of the final answer depends on the data that are missing | Assessing the class by measuring only those who came to the exam: the absent ones change the average |
| **Aleatory uncertainty** | The variability of the world, which more data do not reduce | The outcome of the honest die: rolling ten thousand times does not make the die predictable |
| **Epistemic uncertainty** | The model's ignorance about the world, which shrinks with data | Not knowing the average height of the class; measuring everyone solves it |
| **Knightian uncertainty** | A real process outside the hypothesis space, in which more data give false confidence | The turkey that fattens for a thousand days until Christmas Day |
| **Invariants $\Phi_1$ to $\Phi_5$** | The five boundary conditions every claim must satisfy: simulability ($\Phi_1$), independent unit ($\Phi_2$), severe test ($\Phi_3$), scope and exit clause ($\Phi_4$) and resolution of the question ($\Phi_5$) | The five locks of the sheet that reject a work before the argument |
| **$\Phi_5$ (resolution of the question)** | The invariant that rejects aggregated decision ($R_0$) when the independent unit is the day or coarser | Scaling by the average day when the clinic alternates between calm and surge |
| **Jensen (inequality of)** | The mean of a curved function is not the function of the mean | The average waiting time grows when arrivals oscillate, even with the same average arrival |
| **Kelly (criterion of)** | The fraction of the estate that maximizes growth over the long run | Betting 20% when the chance is 60% at 1 to 1; betting everything leads to ruin |
| **Kupiec (test of)** | The test that checks whether the number of violations of the value at risk is the expected one | Checking whether the flood insurance triggered at the promised frequency |
| **Laplace (rule of)** | The rule that adds one imaginary success and one imaginary failure before estimating a probability | "I never saw that happen" in 36 months is not the same as "it cannot happen" |
| **Lenses (the five)** | The five fixed ways of looking of the program: scale, current, form, time and judgement | Five specialists examining the same house: plumber, electrician, mason, architect and the resident |
| **Model ($L_2$)** | The formal, simplified and simulable representation of the world | The scale model of the building, with paper walls |
| **Null model** | The simulation of what chance alone would produce, to compare with what was observed | Drawing groups by an honest lottery a thousand times before believing in the effect of the medicine |
| **Morphonomy** | The lens of form: what is preserved when the environment changes | The accent that survives the move to another city |
| **World ($L_1$)** | The reality that generates the data and that we never see in full | The real neighbourhood, with the holes the map does not show |
| **Murphy (decomposition of)** | The account that separates the error of a forecast into reliability, resolution and difficulty of the event | A good student who fails the exam because of difficulty, not because of not knowing the subject |
| **Non-ergodicity** | When the average of one trajectory in time is not the average of many trajectories | The average of the people who crossed the river on foot does not describe whoever was carried away by the current |
| **$p$-value** | The frequency with which a world without effect would produce a result as extreme as the one seen | Flipping the coin 100 times and seeing 62 heads: how strange is that in the honest world? |
| **Pearl (ladder of)** | The three rungs: see, act and imagine what did not happen | Watching the price of tomatoes rise, deciding to buy pineapple, and imagining how things would be without the frost |
| **Decision-maker's question** | The question the decision uses, with who decides, which action changes and the question mark at the end | "Does reducing the class to 15 students improve reading one year later?" |
| **Gate** | The automatic test that says yes or no, with exit code 0 or 1 | The subway turnstile: it does not open for whoever has not gone through |
| **Pre-registration with hash** | Freezing the criterion before seeing the data, with the cryptographic digest recorded | Writing the guess and sealing the envelope before opening the result |
| **Pseudoreplication** | Treating dependent observations as independent, shrinking the margin for no reason | Counting ten photos of the same person as ten people in the crowd |
| **Resolution** | The capacity to separate the cases that have the event from those that do not | Telling the days of heavy rain from the drizzle, and not only getting the month's average right |
| **Ruin** | Reaching zero and no longer being able to play, regardless of the average of the bets | Whoever uses the whole credit card at the limit loses the right to stay in the game |
| **Saturation** | When the environment is a function of the variables of the set, and the test has no way of rejecting | Counting the traffic light's colour to forecast the traffic light's colour |
| **Fixed seed** | The starting point of the draw, declared so that two machines produce the same world | The recipe with the exact amount, so that the cake comes out the same at the neighbour's house |
| **Severity** | The chance that the procedure would have gone wrong if the claim were false | The exam that is only useful if it detects the disease when it exists |
| **SHELF** | The protocol for eliciting experts by median, quartiles and extremes | Asking the doctor not for a guess, but for a range and a most probable value |
| **$t_{\mathrm{disp}}$ (clock)** | The instant at which each datum becomes knowable | The exam result that only exists after it leaves the laboratory |
| **Scope term** | The declaration of where the conclusion holds and where it does not | "This holds for the morning shift; at the weekend the queue is another one" |
| **Chouldechova's trilemma** | With different base rates between groups, calibration and equal error rates are incompatible | Two classes with different difficulties cannot have, at the same time, a fair average grade and the same number of failures |
| **Independent unit** | The smallest unit whose observations do not share the same source of variation | The school, and not the student, when students of the same school resemble one another |
| **Likelihood** | The chance of seeing what was seen, supposing each hypothesis true | "How likely is this stain on the roof if it is rain? And if it is seepage?" |
| **Bias (three senses)** | Distortion of the datum, of the estimator and of the mind, and the three accumulate | An uncalibrated scale, an average that mixes weights and the haste of whoever reads the number |

---

## Appendix B. Pocket guide to the formulas

### B.1 How to read the symbols

| Symbol | Read in English | In Rust code |
| --- | --- | --- |
| $\sum_{k=1}^{n} a_k$ | "Add up everything that comes after, from $k$ equal to 1 to $n$" | `let soma: f64 = a.iter().sum();` |
| $\mathbb{E}[X]$ | "The long-run mean of $X$" | `let media = x.iter().sum::<f64>() / x.len() as f64;` |
| $\operatorname{Var}(X)$ | "The mean dispersion of $X$ around its mean, squared" | `let var = x.iter().map(\|v\| (v - media).powi(2)).sum::<f64>() / x.len() as f64;` |
| $\sigma$ or $\mathrm{dp}$ | "Standard deviation: the square root of the variance, in the same unit as $X$" | `let dp = var.sqrt();` |
| $P(A \mid B)$ | "Probability of $A$ **given that** $B$ happened" | `if b { p_a_dado_b } else { p_a_dado_nao_b }` |
| $\operatorname{do}(x)$ | "Fixing $x$ by decision, and not merely observing who already has $x$" | `let y = funcao(x_fixo);` — instead of `let y = funcao(x_observado);` |
| $\ln$ | "Natural logarithm: the exponent of Euler's number that returns the value" | `let z = valor.ln();` |
| $\log_{10}$ | "Logarithm in base 10" | `let d = 10.0 * razao.log10();` (decibans) |
| $\ge$ | "Is greater than or equal to" | `a >= b` |
| $\approx$ | "Approximately equal", within a declared tolerance | `(a - b).abs() < 1e-9` |
| $\sqrt{x}$ | "The square root of $x$" | `x.sqrt()` |
| $x^2$ | "The square of $x$" | `x.powi(2)` |
| $\alpha$ | "The false-alarm level that is accepted: 5% is the usual one" | `const ALFA: f64 = 0.05;` |
| $1 - \alpha$ | "The promised coverage of the procedure: 95% in the usual case" | `let cobertura = 1.0 - ALFA;` |
| $\hat{S}$ | "The set returned by the method, with the hat of estimated" | `let s_hat: Vec<usize> = ...;` |
| $\emptyset$ | "The empty set: the method pointed at nothing" | `Vec::new()` |
| $\cap$ and $\subseteq$ | "The intersection of two sets" and "is contained in" | `a.iter().filter(\|x\| b.contains(x)).count()` |
| $\Phi_1$ to $\Phi_5$ | "The invariants every claim must satisfy" | one gate per invariant, with exit code 0 or 1 |
| $\kappa$ | "The agreement between raters, already discounted what chance would give" | `(po - pe) / (1.0 - pe)` |
| $\theta$ | "The parameter one wants to estimate: the effect, the slope, the rate" | `let theta = 0.148;` |
| $\bar{y}$ and $\bar{p}$ | "The observed rate" and "the mean of the forecasts" | `let citl = p.iter().sum::<f64>() / n as f64 - y.iter().sum::<f64>() / n as f64;` |

### B.2 The formulas the book uses

| Formula | In words | In code |
| --- | --- | --- |
| $P(H \mid E) = \frac{P(E \mid H)P(H)}{P(E)}$ | Today's belief is yesterday's multiplied by the strength of the evidence | posterior odds $=$ prior odds $\times$ likelihood ratio |
| $10\log_{10}\text{RV}$ | The strength of the evidence in decibans: every 10 dB is a factor of 10 | `10.0 * rv.log10()` |
| $\operatorname{Brier} = \frac{1}{n}\sum (p_i - y_i)^2$ | The mean of the squared error of a probabilistic forecast | `p.iter().zip(y).map(\|(a,b)\| (a-b).powi(2)).sum::<f64>() / n as f64` |
| $\text{habilidade} = 1 - \operatorname{Brier}/\operatorname{Brier}_{\text{clima}}$ | How much the model gains over the climatologist | `1.0 - brier / brier_ref` |
| $\mathrm{ECE} = \sum_k \frac{n_k}{n}\lvert \bar{p}_k - \bar{y}_k\rvert$ | The mean distance between predicted and observed, band by band | 10 boxes of equal mass, summing the weighted absolute difference |
| $f^* = 2p - 1$ | The Kelly fraction for a bet that pays the same as it risks | `let f = 2.0 * p - 1.0;` |
| $\frac{s+1}{n+2}$ | The rule of Laplace: one imaginary success and one imaginary failure | `(s + 1.0) / (n + 2.0)` |
| $W_{M/M/1} = \frac{\rho}{\mu - \lambda}$ | The average wait in a queue with one server grows on a curve | Do not use the mean to decide the scale: simulate the day |
| $\mathrm{CITL} = \bar{p} - \bar{y}$ | The level deviation: how much the average forecast misses the observed rate | `media(p) - media(y)` |

**A reading warning.** In Rust, `f64` holds about 15 to 16 digits. Therefore $\approx$ is never
equality, and every comparison of measured numbers needs a **declared** tolerance, such as the band of 0.05
standard deviations that the background audit fixed for comparing two rounds of the same domain. A band chosen
after seeing the result counts as a new declaration and enters the counter of attempts.

---

## Appendix C. Reproduction guide

### C.1 The crates in Rust

| Command | What it proves |
| --- | --- |
| `cargo test --workspace` | That the types and the gates of the architecture work: measured in this tree, 373 tests pass in the nine crates |
| `cargo run --release --bin gates -- --horizon all --reproduction` | That the gate runs twice and gives the same result: measured, "7 portões ok", with the Merkle root `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81` |
| `cargo run --release --bin gates -- --horizon h3 --defect h3.voos23` | That a gate **knows how to reject**: with the defect planted, the command exits with code 1 and says which gate fell |
| `cargo run --release --bin labs -- --chapter 19` | That the numbers cited in Step 3 of chapter 19 come out of code, with a fixed seed. It is worth replacing 19 with any chapter; `--all` runs them all |
| `rustc -O crates/stochology-book/src/ch21.rs -o /tmp/ch21 && /tmp/ch21` | That every laboratory of this edition is self-contained: one file, no dependency, the same result |

The laboratories of this edition live directly in the crate, in `crates/stochology-book/src/capNN.rs`. They are
tested modules (`ch01` to `ch04` carry tests in the file) and **all 21** are served by the `labs` binary. The
intermediate copies were deleted after integration, because a working file does not stay in the repository.
No laboratory uses the system clock or an unseeded random number, and that is why verification is
possible.

### C.2 The empirical registry (the Python code was retired)

| Folder | What it reproduces | Main command |
| --- | --- | --- |
| `h0-lab0/` | The simulated health clinic (Chapter 16) | `python3 portoes.py` |
| `h0-real/` | The bank call centre (Chapter 16) | `python3 portoes.py` |
| `h1/` | The simulated H1, with the panel (Chapter 17) | `python3 rodar.py` and `python3 portoes.py --reproduction` |
| `h1-real/` | Dow Jones, LaLonde and the Danish fires (Chapters 17 and 18) | `python3 rodar.py` and `python3 portoes.py --reproduction` |
| `h2/` to `h2g/` | The seven rounds of ICP (Chapter 19) | `python3 rodar.py` and `python3 portoes.py --reproduction` in each folder |
| `h3/` | The six streams and the revisions H3b to H3d (Chapter 20) | `python3 rodar.py` and `python3 portoes.py --reproduction` |
| `governance/` | The record of the signatures of the 21 opinions (Chapters 15 and 21) | Reading |
| `empirical/` | The background audit and the extensions (Chapter 21) | Reading; `rodar.sh` in p02 and p05, a script of its own in the others; p12 has only the protocol and the forms |

The commands that tied the registry together, copied from the research plan, today demand the code back:

```bash
git checkout 1bfec905 -- h1/          # the Python code of the fronts lives in the history
cd h1 && python3 rodar.py && python3 portoes.py --reproduction && python3 grafo.py
cd h3 && python3 rodar.py && python3 grafo.py && python3 figura.py && python3 portoes.py --reproduction
cd empirical/p07-icp/codigo && python3 nulo_validacao.py          # the null rate of the genetics, 0.5 s
```

**The Python simulation code was retired on 2026-10-16** (576 files, about 2.9 MB), because the engineering
apparatus of the project became the Rust workspace. What stays in the tree is the record. They are the data and the sums, the
`README.md` of each front with the published numbers, the declarations, the opinions and the checked outputs. The
manifest `history/python-removido.txt` carries the blob hash of each removed file, and the complete declaration is in
`docs/declarations/REMOVED_PYTHON.md`.

On that day the artifacts that do not support a central result left. They are the extensions of scale, health and rare event (points
9 and 10 of the background audit) and the previous edition of the book. The numbers of those extensions remain in Chapters 15 and 21, with
the retirement note in the table. Afterwards the whole Python measurement scaffolding left. The fronts `h0` to `h3` also left, with their READMEs, and the data
that only they used. The numbers stayed in `docs/EMPIRICAL_REGISTRY.md`, with commit `1bfec905` as the anchor
(`docs/declarations/RUST_CONSOLIDATION.md`).

Three checks that keep running without Python, and that cost seconds:

```bash
python3 scripts/book/check_labs.py       # checks every output block against the instrument
python3 scripts/book/build.py --check       # checks the three-step ramp of the 21 chapters
cargo test --workspace               # the book crates and the architecture
```

### C.3 The recording environment, and how to read a divergence

The H3 record is: **Python 3.13.7**, **numpy 2.5.1** (obtained with `PYTHONNOUSERSITE=1 python3`), pandas 3.0.5,
scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0, duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1 and hypothesis 6.165.7, with
`export OMP_NUM_THREADS=1` and at most two processes in parallel. Two dependencies are missing on this machine, and the
environment file says how to install them. The first is `statsmodels`, which only the answer key of point 1 uses, and the second is `pyreadr`, which only
one step of point 8 uses.

The numpy case shows why the environment is part of the number. The estimated effect of the performative loop was
published as 0.5632 with 2,375 shocks, and the identical code, run with the same seed, gave 0.5529 and 2,395
shocks. There were no two versions of the code. There were two numpys in the interpreter, and the default path imported
2.4.6 before 2.5.1. The point of the divergence was isolated. In the two versions, the generator started from the same state,
and a binomial draw of 248 trials with probability 0.6 returns 171 in one and 150 in the other, because 2.5
fixed the Stirling series of the acceptance test of the BTPE. If you meet a divergence like this, the path is
always this one. **First the environment, then the code**, because no published number is exchanged without a written declaration
before running.

---

## Appendix D. Annotated bibliography for young scientists

### D.0 How this list was checked

This bibliography follows three rules, and all three are declarations.

**1. The check was remote, and not by downloaded file.** On 2026-10-16 the author checked the metadata
of the links in this list against the Crossref API, the publishers' pages, PubMed, the PMLR proceedings and library
catalogues. No third-party PDF exists in the tree, and none is claimed as `[file]`: what the tree keeps is
the check of the metadata, recorded line by line. The five documents of the project were **integrated** into the
repository (architecture, scalable plan, foundation and the article) and the loose copies were deleted, as the
rule of new files demands. The integrated material is in `docs/PLAN.md` §9.13, in `paper/` and in this appendix. Of them
what remains is the citation, the DOI and the number that the book uses.

**2. Every item has a state of checking.**

| Mark | Means |
| --- | --- |
| **[checked]** | the number cited in the book was checked against the article itself, against the DOI or against the publisher's page |
| **[file]** | there is a downloaded file for consultation, still without item-by-item checking against the text of the book. No line of this list uses this mark: no third-party PDF is on disk, and the check is of the metadata |
| **[historical]** | retired material, which stays only as a record of where a number came from |

**The state of each line is in the line.** After the check of 2026-10-16, every line of D.2 to D.4 carries
the mark **[checked]**, and the source of each check is in `references/CONFERENCIA.md`. The route and the date of
each pass are in the table below, because the mark alone does not say when it was applied.

| pass | date | lines | route | what stayed in the line |
| --- | --- | --- | --- | --- |
| first | 2026-10-15 | the 23 of D.3 | Crossref, IDEAS/RePEc, RAND, NeurIPS proceedings and the author's page | checked, **without** the mark applied |
| second | 2026-10-16 | the 27 of D.2 and D.4 | Crossref, publishers' pages, PubMed and PMLR | checked and marked |
| third | 2026-10-17 | the 23 of D.3, again | the same sources of the first, line by line | mark applied: **50 of 50** |

The third pass exists because the first left the check recorded outside the line, and reading the table
gave less than the total. It re-ran the metadata of that section against the same sources, one by one, and the result is
what the table above declares. Divergence of content in that re-run: none.

**3. Every experiment in this book is Rust.** Python appears only as a retired historical record
(see Appendix C). No number of Step 3 depends on Python, and none depends on the course.

**Warning: the course was retired.** The course *Pensamento Crítico e Modelos Estocásticos*, from which
several simulation numbers and a good part of the internal numbering (M0 to M15, ME1 to ME24, MB1 to MB8) came, was
**made obsolete on 2026-10-16**. The mentions of "course script" and "course material" that still appear in the
chapters are **historical**. They say where a number came from, and they are not current material. What holds is what this
repository keeps. It is the checked empirical registry, the declarations and the laboratories in Rust.

### D.1 Documents of the project

- [Architecture](https://docs.google.com/document/d/1l0YQHrV_UrKisAjQcz2BqT9KpwhcT0s0WVoXm_muxtE/edit),
  [Master plan (`docs/PLAN.md`)](https://docs.google.com/document/d/1hXvDuJyckooL1_qx9gspkSkBqgjyhAUVQvQ3F-Crinc/edit),
  [AGENTS.md](https://docs.google.com/document/d/1hniAsjLqgNVGqR1wZ1845GVkS5lZTEjhlH7MaI3U2Xk/edit) and
  [Estocologia: Fundamentação, Três Camadas e Inferência sob Incerteza](https://docs.google.com/document/d/1L5ittj7-RiSacEvNrkR4IS6yrQUyhusFpcDIWazINLI/edit)
  are the design of the machine, the master plan and the foundation.
- [Estocologia: Velhas Fronteiras, Novos Horizontes](https://claude.ai/code/artifact/69dbc377-114b-4cb7-a3c1-322387abc674)
  is the article, with the complete list of references.
- **[historical]** The course *Pensamento Crítico e Modelos Estocásticos*, **retired on 2026-10-16**. It stays cited because several numbers of the book came from its simulations, with the source now declared in the chapters; nothing in the book depends on it.
- `references/CONFERENCIA.md`: the check of the references, made on October 15, 2026 against the Crossref
  API, the publishers' pages, PubMed and RePEc. It is the model of what to do with your bibliography. They are
  25 references checked one by one, one correction applied (the article is by **Meyer**, Viscusi and Durbin, and not by
  Viscusi and Durbin), one journal swapped (Baltagi and Levin came out in *Structural Change and Economic Dynamics*), and the
  note that SHELF is a protocol, not the book by O'Hagan and colleagues.

### D.2 Classics to begin with

| Reference | Why read it now |
| --- | --- |
| **[checked]** Bernoulli, *Ars Conjectandi* (1713) | The book that turned "probably" into an account. The rule of Laplace of this book is its daughter |
| **[checked]** Hume, *Enquiry* (1748; the title *Enquiry* is from the 1758 edition) | The question that no statistics solves: why does the future resemble the past? |
| **[checked]** Knight, *Risk, Uncertainty and Profit* (1921) | The distinction between measurable risk and uncertainty that cannot be measured, in Chapter 4 |
| **[checked]** Savage, *The Foundations of Statistics* (1954) and Savage, *JASA* 46:55–67 (1951) | Where the idea of a proper rule comes from: betting against one's own belief is losing money |
| **[checked]** Wald, *Statistical Decision Functions* (1950) and Chernoff, *Econometrica* 22(4) (1954) | Deciding with loss, and not only with probability: the basis of Chapter 11 |
| **[checked]** Royall, *Statistical Evidence: A Likelihood Paradigm* (1997) | Likelihood as a measure of evidence, and not as a final answer |
| **[checked]** Tversky and Kahneman, "Judgment under Uncertainty" (1974) | The catalogue of mental shortcuts that Chapter 2 organizes by ignored model of chance |
| **[checked]** Mayo, *Error and the Growth of Experimental Knowledge* (1996) | Severity: a test is worth only what its power to err is worth |
| **[checked]** Pearl, *Causality* (2000) | The ladder of seeing, acting and imagining, and the back door |
| **[checked]** Gigerenzer, *Calculated Risks* (2002) | How to write risk for whoever needs to decide: natural frequencies, without jargon |
| **[checked]** Mandelbrot, *Fractals and Scaling in Finance* (1997) | Wild tails counted by whoever saw them first in prices |

### D.3 Methods and data used in the research

| Reference | Where it enters |
| --- | --- |
| **[checked]** Shen, H.; Huang, J. Z. *Annals of Applied Statistics* 2(2):601–623, 2008. DOI: 10.1214/08-AOAS164 | The bank call centre, in 2003 (Chapters 16 and 20) |
| **[checked]** LaLonde, R. J. *American Economic Review* 76(4):604–620, 1986 (JSTOR 1806062) | The NSW job-training experiment and the raw difference that deceives (Chapter 17) |
| **[checked]** Dehejia, R. H.; Wahba, S. *JASA* 94(448):1053–1062, 1999. DOI: 10.1080/01621459.1999.10473858 | The answer to LaLonde: matching hits the number, and why that is still not enough |
| **[checked]** McNeil, A. J. *ASTIN Bulletin* 27(1):117–137, 1997. DOI: 10.2143/AST.27.1.563210 | The Danish fires and the wild tail (Chapter 18) |
| **[checked]** Embrechts, P.; Klüppelberg, C.; Mikosch, T. *Modelling Extremal Events for Insurance and Finance*, Springer, 1997. DOI: 10.1007/978-3-642-33483-2 | The theory of the tail that the Hill estimator uses |
| **[checked]** Peters, J.; Bühlmann, P.; Meinshausen, N. *JRSS B* 78(5):947–1012, 2016. DOI: 10.1111/rssb.12167 | ICP, the method of the seven rounds (Chapter 19). Read the discussion: the objections are there |
| **[checked]** Heinze-Deml, C.; Peters, J.; Meinshausen, N. *Journal of Causal Inference* 6(2):20170016, 2018. DOI: 10.1515/jci-2017-0016 | The nonlinear ICP, which inspired the second ruler of H2c |
| **[checked]** Gamella, J. L.; Peters, J.; Bühlmann, P. *Nature Machine Intelligence* 7:107–118, 2025. DOI: 10.1038/s42256-024-00964-x | The physical chambers, the light tunnel and the wind tunnel |
| **[checked]** Kemmeren, P. et al. *Cell* 157(3):740–752, 2014. DOI: 10.1016/j.cell.2014.02.054 | The gene deletions in yeast and the replication of 40 genes |
| **[checked]** Meinshausen, N.; Hauser, A.; Mooij, J. M.; Peters, J.; Versteeg, P.; Bühlmann, P. *PNAS* 113(27):7361–7368, 2016. DOI: 10.1073/pnas.1510493113 | The validation by intervention that decides the replication of the genetics |
| **[checked]** Johannesen, L.; Vicente, J.; Mason, J. W. et al. *Clinical Pharmacology & Therapeutics* 96(5):549–558, 2014. DOI: 10.1038/clpt.2014.155 | The FDA trial with dofetilide, quinidine, ranolazine and verapamil |
| **[checked]** Vicente, J. et al. *Journal of the American Heart Association* 4(4):e001615, 2015. DOI: 10.1161/JAHA.114.001615 | The morphology of the T wave in the same trial |
| **[checked]** Sachs, K.; Perez, O.; Pe'er, D.; Lauffenburger, D. A.; Nolan, G. P. *Science* 308(5721):523–529, 2005. DOI: 10.1126/science.1105809 | The signalling network that H2 pruned in the first filter |
| **[checked]** Krueger, A. B. *Quarterly Journal of Economics* 114(2):497–532, 1999. DOI: 10.1162/003355399556052 | STAR: small classes drawn by lot in Tennessee |
| **[checked]** Finn, J. D.; Achilles, C. M. *American Educational Research Journal* 27(3):557–577, 1990. DOI: 10.3102/00028312027003557 | The design of STAR, told by whoever applied it |
| **[checked]** Card, D.; Krueger, A. B. *American Economic Review* 84(4):772–793, 1994 (JSTOR 2118030) | Minimum wage and employment, the differences-in-differences model |
| **[checked]** Meyer, B. D.; Viscusi, W. K.; Durbin, D. L. *American Economic Review* 85(3):322–340, 1995 (JSTOR 2118177) | The cap on the workers' compensation benefit, in Kentucky and in Michigan |
| **[checked]** Thornton, R. L. *American Economic Review* 98(5):1829–1863, 2008. DOI: 10.1257/aer.98.5.1829 | The lottery of the HIV test result, an environment that is an intervention |
| **[checked]** Newhouse, J. P.; Insurance Experiment Group. *Free for All?*, Harvard University Press, 1993 | The RAND insurance experiment, whose copayment saturates the test |
| **[checked]** Baltagi, B. H.; Levin, D. *Structural Change and Economic Dynamics* 3(2):321–335, 1992. DOI: 10.1016/0954-349X(92)90010-4 | The beer tax, pruned in the third filter |
| **[checked]** Stock, J. H.; Watson, M. W. *Introduction to Econometrics*, 4th ed., Pearson, 2019, ch. 12 | Instrumental variables, the cigarettes example |
| **[checked]** Gibbs, I.; Candès, E. *NeurIPS 34*:1660–1672, 2021 (arXiv:2106.00170) | Conformal adaptive inference, which gives the quantiles of H3 |
| **[checked]** O'Hagan, A. et al. *Uncertain Judgements: Eliciting Experts' Probabilities*, Wiley, 2006. DOI: 10.1002/0470033312 | The elicitation that remained open in Chapter 21. Note: SHELF is a protocol, not this book |

### D.4 Typologies of error, uncertainty and calibration

| Reference | Where it enters |
| --- | --- |
| **[checked]** Rubin, D. B. *Biometrika* 63(3):581–592, 1976. DOI: 10.1093/biomet/63.3.581 | MCAR, MAR and MNAR: why the data are missing (Chapter 6) |
| **[checked]** Kimball, A. W. *JASA* 52(278):133–142, 1957. DOI: 10.1080/01621459.1957.10501374 | The type III error: answering the wrong question |
| **[checked]** Gelman, A.; Tuerlinckx, F. *Computational Statistics* 15(3):373–390, 2000. DOI: 10.1007/s001800000040 | The type S (sign) and type M (magnitude) errors |
| **[checked]** Gelman, A.; Carlin, J. *Perspectives on Psychological Science* 9(6):641–651, 2014. DOI: 10.1177/1745691614551642 | How small power produces exaggerations of size |
| **[checked]** Walker, W. E. et al. *Integrated Assessment* 4(1):5–17, 2003. DOI: 10.1076/iaij.4.1.5.16466 | The typology of uncertainty, in the language of public decision |
| **[checked]** Stirling, A. *Nature* 468(7327):1029–1031, 2010. DOI: 10.1038/4681029a | Uncertainty, ambiguity and ignorance, and why simplifying costs |
| **[checked]** Moreno-Torres, J. G. et al. *Pattern Recognition* 45(1):521–530, 2012. DOI: 10.1016/j.patcog.2011.06.019 | The unified view of distribution shifts, in Chapter 4 |
| **[checked]** Ellsberg, D. *Quarterly Journal of Economics* 75(4):643–669, 1961. DOI: 10.2307/1884324 | Ambiguity aversion, in Stirling's matrix |
| **[checked]** Sweeney, L. *Simple Demographics Often Identify People Uniquely*, Carnegie Mellon, 2000 | Quasi-identifiers and the k-anonymity of H3 |
| **[checked]** Baig, S. S. et al. *European Journal of Human Genetics* 24(10):1396–1402, 2016. DOI: 10.1038/ejhg.2016.36 | The predictive test for Huntington's disease, and what "17.4%" means |
| **[checked]** Makin, T. R.; Orban de Xivry, J.-J. *eLife* 8:e48175, 2019. DOI: 10.7554/eLife.48175 | The ten most common statistical errors, used in the blind pilot |
| **[checked]** van Dantzig, D. *Econometrica* 24(3):276–287, 1956 | The dike problem: how much to spend against an improbable flood (Chapter 11) |
| **[checked]** Saerens, M.; Latinne, P.; Decaestecker, C. *Neural Computation* 14:21–41, 2002 | Adjusting the outputs of a classifier to new base rates |
| **[checked]** Lipton, Z.; Wang, Y.-X.; Smola, A. *ICML* 80:3122–3130, 2018 | The label-drift test that did not warn of the 2013 shift (Chapter 20) |
| **[checked]** Janssen, K. J. M. et al. *Canadian Journal of Anesthesia* 56(3):194–201, 2009 | Adjusting a risk model to local circumstances |
| **[checked]** Van Calster, B. et al. *Journal of Clinical Epidemiology* 74:167–176, 2016 | The hierarchy of calibration, from utopia to empirical data |

### D.5 How to use this list

Read in the order that curiosity asks for, and distrust whoever presents any of these works as a
final answer. Three practical pieces of advice. First, read the data before the method, because almost every error of
reading is born from not asking where the table came from. Second, when an article cites a number, look for the
number in the code or in the appendix, and not in the abstract. Third, keep the version of the environment next to the number you
reproduced; this book has a whole chapter that exists only because someone did that.

### D.6 What has already been checked and what remains to be checked

The check of the references of the project is in `references/CONFERENCIA.md`. They are 25 references checked
one by one against the Crossref API, the publishers' pages, PubMed and RePEc, with one correction applied (the
article on the benefit cap is by **Meyer**, Viscusi and Durbin, and not by Viscusi and Durbin) and one journal
swapped (Baltagi and Levin came out in *Structural Change and Economic Dynamics*).

**The check ended on 2026-10-16.** The 50 lines of D.2 to D.4 were checked item by item against the Crossref
API, the publishers' pages, PubMed, the PMLR proceedings and library catalogues: the 23 of D.3 in the first
pass (2026-10-15) and the 27 of D.2 and D.4 in the second. Every line of this appendix carries the mark **[checked]** and every
check is in its line of `references/CONFERENCIA.md`, with the source beside it.

The route of each line of the methods and data section, and what it proves:

| route | lines of D.3 | what it proves |
| --- | --- | --- |
| Crossref API, with the deposited DOI | 17 | author, year, journal, volume, number and pages |
| IDEAS/RePEc, citation metadata | 3 | the three AER articles before 1999, which have no DOI |
| RAND, catalogue DOI | 1 | the book of the insurance experiment, with publisher and year |
| NeurIPS proceedings | 1 | the conformal adaptive inference paper, with the pages |
| author's page, at Princeton | 1 | the textbook and the chapter on instrumental variables |

What appeared: **no divergence of content** in the 27 new lines — authors, years, titles, journals and
pages check out —, **eleven completions** (the DOIs that were missing in ten lines and the working paper number of
Sweeney) and **two source reservations declared**: the final page of Chernoff 1954 and that of van Dantzig 1956
did not come out in the sources consulted, and the year of Sweeney (2000) comes from the CMU record.

Rule 1 of this appendix still holds, and now with the exact sense: the check is of the metadata, and what the
book keeps of each one is the citation, the DOI and the number that it uses. What does not enter the repository are the third-party
files, for copyright, and none of them is claimed to be present on disk.
