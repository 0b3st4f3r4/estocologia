# Stochology: Judgement and Decision under Uncertainty

**21 chapters** · five parts · a three-step ramp per chapter

A working text, not yet a textbook: its central thesis has had one direct test and its outside audit is still
open (Preface).

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

**What the reader cannot run.** The rule above holds for the simulations and for ten real datasets: the bank calls,
the Dow Jones, the two files of the job-training experiment, the Danish fires, and the five public sources on which
Chapter 20 rebuilds its streams (the flights of 2013 and 2023, the electricity of Victoria, the Atlantic cyclones and
the Toronto arrests of 1997 to 2002). It does not hold for everything the book cites. The real cases of Chapter 19 (the
physical chambers, the cardiac trial, the small classes and the yeast), the arrests of 2020 and 2021 and the traffic
stops of Chapter 20, the propensity fit and the Kelly interval of
Chapter 13, the skills of the forecasters of Chapter 14, and a handful of worked examples in Chapters 3, 7, 10, 11, 16, 17 and 21 come from records of the research, made with code that does not run beside this book. Appendix E lists them.
They carry the weight of a citation, not of a check, and the lessons that lean on them lean on a citation.

**Who wrote what.** The theory, the architecture, the research and the first text are the author's. From 2026-10-18
on, an AI assistant read the book critically, reimplemented its laboratories from the text, rebuilt the real streams
of Chapter 20 and wrote the corrections those readings found; Appendix E lists each change with both values, and the
dated records of the repository say which pass made it. Those readings are outside the author's own audit and inside
the author's tools, and no person outside the programme has checked them yet. Chapter 21 says what an audit from
outside would need; until it exists, read every correction here as one more claim to check.

**The thesis that organises the book.** The bridge thesis of Chapter 2 (most fallacies are a misalignment between the
world, the model and the agent, not an error of arithmetic) has had one direct test, and it tied at the ceiling
(Chapter 21). Until a fair test, with cases chosen and judged by people outside the work, is run, the thesis is a
working hypothesis about where to look first, not a result, and the book should be read that way.

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
| **IV. The method on real data** | 12 to 14 | What do the first three parts change on a queue, a market and a fire insurer? |
| **V. The auditable machine, built and put to work** | 15 to 21 | How do you turn that thinking into a machine that audits itself, and what happens when it meets the data? |

**Two books in one.** Parts I to IV are the method: the theory and the three real cases where it decides something.
Part V is the machine: a particular architecture built to audit that method, and its record on real data, failures
included. The first can be read without the second; the second leans on the first.

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
| 12 | The health post and the bank | the same mean, two waits; Erlang C against the real queue |
| 13 | The stock market and the job | 3x leverage; LaLonde; severity that fails the method itself |
| 14 | The Danish fires | wild tail; Laplace; failing your own model is a victory |
| 15 | The map of knowledge | the typed hypergraph; the five lenses; pathology edges |
| 16 | The team of specialists, the safeguard panel and the two systems | eight specialists; the safeguard panel A1–A8; base-rate correction and SBC; System 1 and System 2 |
| 17 | The iron contract | cards; four prohibitions; invariants Φ1–Φ5; property tests; the funnel; gates |
| 18 | The vault and the seal | pre-registration with hash; counter K; the vault; auditing one's own claims |
| 19 | Seven rounds of ICP | five honest failures; light, heart and yeast; a gate that its first declared version does not pass |
| 20 | The live clock | zero leakage with a canary; Goodhart measured; three vaults |
| 21 | The seven lessons and the final project | the lessons with their conditions; objections; the resolution axis; the final project |

---

## Reading tracks

| Reader | Path | Suggested time |
| --- | --- | --- |
| High School | chapters 1 to 8, only Step 1 and the *Concept* exercises | 8 meetings |
| Undergraduate | chapters 1 to 11, the three steps, laboratories in Rust | one semester |
| Data professional | chapters 1, 2, 5, 6 and 8, Part IV, and chapters 17 to 21 | four weeks |
| Whoever wants only the architecture | chapters 2, 15, 16, 17 and 18 | two weeks |
| Whoever wants to see the method in action | Part IV and chapters 19 to 21, returning to the foundations when needed | two weeks |

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

# Part I. The Art of Aiming in the Dark

## Chapter 1. The Invisible Target: What Is Stochology?

**Objectives.** By the end of this chapter you should be able to say what Stochology studies, where the name
comes from, which three traditions it joins, how a theory can be repaired without becoming dishonest, and why
checking a result requires a draw that can be repeated.

---

### Step 1 — The real world and the paradox

**The decision before the answer.** On a Tuesday morning, a doctor sees a 54-year-old man with chest pain. She
has an electrocardiogram, his age, his history, and fifteen minutes. She must choose between admitting him and
sending him home. If she admits him, the hospital spends a bed that another patient will lack; if she sends him
home and the worst happens, the cost falls on him.

Notice what has just happened. **The decision was taken before the answer existed.** This is not a flaw of the
health system. It is the condition of almost every decision that matters. The judge decides without having seen
the crime. The manager hires staff for a demand that has not arrived yet. You choose a course before knowing
whether you will like it.

**Stochology** is the science (and the art) that studies this kind of judgement: **judgement and decision under
uncertainty**.

**Counting instead of percentages.** A percentage hides who is counted. A natural frequency shows it. "A
probability of 9%" becomes *in each 100 cases like this one, 9*. "A prevalence of 1%" becomes *in each 1,000
people, 10*. "A 95% confidence interval" becomes *in each 100 repetitions of the experiment, 95 intervals
contain the real value*. The number is the same; the second form already shows how many people stand on each
side of the decision.

**Where the name comes from.** *Stókhos* is the Greek word for the target an archer aims at, and also for a
well-aimed guess. In 1713, eight years after his death, Jacob Bernoulli's *Ars Conjectandi* was published: the art
of conjecturing, which he called *stochastice*. Over time "stochastic" came to mean only "random", and it lost the
interesting part: **aiming well at a target you cannot see**.

**Science and art.** Part of judgement under uncertainty is formal: keeping beliefs coherent, updating them with
evidence, measuring a margin of error on the right unit, checking calibration, pricing the error. Part of it is
not: deciding which model deserves to be taken seriously, when to stop collecting data, and who pays if the
decision goes wrong. A discipline about uncertainty must say where its own formalism ends.

**Why two computers must draw the same numbers.** A computation can be checked only if someone else, on another
machine, can repeat it and obtain the same result. A program that draws different random numbers on every run
cannot be checked: each run answers differently, and every disagreement ends in "on my computer it gave another
value".

The way out is the **seed**. A pseudo-random number generator is a deterministic recipe. Given the same starting
value, it produces the same sequence, always, on any machine. A tiny example shows the idea. Take the rule
"multiply by 5, add 3, keep the remainder of the division by 16", and start from 7:

| Step | Calculation | Result |
| --- | --- | --- |
| 1 | 5 × 7 + 3 = 38, remainder by 16 | 6 |
| 2 | 5 × 6 + 3 = 33, remainder by 16 | 1 |
| 3 | 5 × 1 + 3 = 8, remainder by 16 | 8 |
| 4 | 5 × 8 + 3 = 43, remainder by 16 | 11 |

The sequence 6, 1, 8, 11 looks irregular, yet anyone who starts from 7 gets exactly the same numbers. Real
generators use the same idea with much larger numbers. The chance in the experiment is **reproducible chance**:
random to whoever looks at the result, auditable to whoever knows the seed.

**What the seed does not prove.** Three words are easy to confuse. A result is **reproducible** when the same code on
the same data gives the same number on another machine; the seed buys exactly that. It is **replicable** when a new
study, with new data, reaches the same conclusion; no seed buys that. It is **valid** when the account itself is
right. The seed reproduces a wrong formula as faithfully as a right one: in this book, a wrong Kelly fraction and a
wrong flood model passed every test and every gate, identical on every machine, until someone read the formula
(Appendix E). Reproducibility is the floor of checking, not its roof.

---

### Step 2 — The rule of the game

**Definition.** Stochology studies how an agent turns incomplete information into an action, and how that agent
answers for the consequences.

**Three traditions.** The discipline joins three traditions that grew up separately and rarely cite each other.

- **The philosophy of induction.** David Hume showed that every defence of induction already assumes that the
  future will resemble the past: the sun rising every morning so far does not prove, by logic alone, that it will
  rise tomorrow. Karl Popper answered that science does not prove theories, it tries to refute them. A thousand
  white swans do not prove "all swans are white"; one black swan refutes it.
- **Statistical inference.** Pascal and Fermat, Bernoulli, Bayes, Laplace and Kolmogorov built probability, the
  language of degrees of belief and of frequencies. Fisher, Neyman and the Bayesian school built inference, the
  rules for learning from data with a declared margin of error.
- **Stochastic processes.** Markov and Langevin studied how chance accumulates over time. A single coin toss is
  unpredictable; the fraction of heads in a long run is not. Many decisions are sequences, and what matters is the
  path, not one step.

**Repairing a theory honestly.** Pierre Duhem and Willard Quine pointed out that a failed prediction never
refutes a theory alone: it refutes the theory together with the auxiliary hypotheses used to derive the
prediction. So there is always some way to save the theory. Imre Lakatos separated two kinds of rescue.

- A **progressive** repair predicts something new that is later confirmed. In 1846 the orbit of Uranus disagreed
  with Newton's laws. Urbain Le Verrier did not abandon Newton; he postulated an unseen planet, computed where it
  should be, and Neptune was found there.
- A **degenerative** repair only protects the theory. It adds an auxiliary hypothesis that predicts nothing
  new, or keeps adding one after another as each new prediction fails. The diet whose author blames the
  participants, without measuring whether they followed it, is the typical case.

The orbit of Mercury shows why the verdict is harder than it looks. In 1859 the same Le Verrier made the same
move: a hidden planet, Vulcan, inside the orbit of Mercury. It was not an empty rescue. It predicted transits
across the Sun that astronomers then searched for, and they did not find them. Vulcan was a progressive *proposal*
whose prediction **failed**; the anomaly was explained only by general relativity, in 1915. For Lakatos,
degeneration is a trajectory of a whole programme, not the fate of a single hypothesis, and it is often visible
only in hindsight.

The test is one question, asked **at the moment of the repair**: **did the repair predict something new that could
fail, or did it only hide the error?** Every change of method after a failure must answer it, and must say in
advance which observation would count against it.

**The three questions.** Instead of asking only "what is the probability?", Stochology asks three questions of
any claim:

1. **In which layer can this claim fail?** In the world, in the model, or in the person who decides.
2. **What kind of uncertainty threatens it?** Chance, ignorance that more data can reduce, or a possibility no
   one has listed.
3. **What test would it have to face to deserve trust?** A test it could have failed.

Chapters 2 to 4 develop each question in turn.

---

### Step 3 — Hands on

The laboratory checks the two properties of a seed: the same seed gives the same sequence, and a different seed
gives an unrelated one. Run it with `cargo run --release --bin labs -- --chapter 1`.

```text
== Chapter 1 :: the invisible target ==
two machines with the seed 1713 agree in 10000 of 10000 draws: true
draws coinciding with the next seed: 0
seed 1713: frequency of heads in 100 thousand tosses = 0.5000
seed 1909: frequency of heads in 100 thousand tosses = 0.4992
seed 2026: frequency of heads in 100 thousand tosses = 0.4987
```

Read it line by line.

- **10000 of 10000.** Two independent generators started from the same seed produce identical sequences. Anyone
  can repeat the experiment and check it.
- **0.** With the next seed, the sequences share nothing: zero matches in ten thousand draws. The seed is a
  starting point, not a way of choosing the result.
- **0.5000, 0.4992 and 0.4987.** Three seeds, three fair coins, and head frequencies within 0.0013 of one half
  after 100 thousand tosses. Each toss is unpredictable; their average is not. That is the law of large numbers,
  and it is why a long run of chance can be trusted when a single draw cannot.

**Exercises.**

1. *(Concept)* Why does the definition say "science (or art)"? Give an example of a decision in which the "art"
   part cannot be avoided.
2. *(Concept)* A newspaper says: "the draw was audited and nobody knew the result beforehand". If the program uses
   a fixed seed, does whoever knows the seed know the result? Why does that not make the draw dishonest, as long as
   the seed is published before the draw and cannot be changed after it?
3. *(Count)* Continue the table of Step 1 for four more steps. After how many steps does the sequence return
   to 7? What does that say about using a small remainder in a real generator?
4. *(Count)* You toss a coin 100 times and get 53 heads. In 100 thousand tosses, would you expect the
   fraction to stay 0.03 away from one half? Justify in one sentence.
5. *(Laboratory)* Change the seed 1909 to 1910 in the laboratory and run it again. How many draws coincide with
   the original sequence?
6. *(Concept)* Classify each repair as progressive or degenerative, and justify: (a) a diet fails, and its author
   says the participants did not follow it, without measuring whether they did; (b) a vaccine trial fails in older
   adults, and the researchers predict, before a new trial, that a higher dose will work in that group, and it
   does.

**Summary.**

- Stochology is the science (or art) of judgement and decision under uncertainty.
- The name comes from *stókhos*, the target, and from Bernoulli's *Ars Conjectandi* (1713): aiming well at what
  cannot be seen.
- It joins the philosophy of induction, statistical inference and the study of stochastic processes.
- A repair of a theory is honest when it predicts something new that could fail (Neptune, and also Vulcan, whose
  prediction failed) and dishonest when it only hides the error.
- A computation can be checked only if its chance can be repeated, which requires a fixed seed; a fixed seed makes a
  result reproducible, not replicable and not right.

## Chapter 2. World, Model and Agent: where do errors live?

**Objectives.** By the end of this chapter you should be able to separate the three layers of reality, state the
bridge thesis and what would refute it, apply Peirce's three forms of inference, distinguish the three senses of
the word "bias" and recognise the prosecutor's fallacy.

---

### Step 1 — The real world and the paradox

**The territory, the map and the driver.** You are driving in an unknown city. There is the real city: streets,
potholes, roadworks, the truck stopped at the corner. There is the map, a sheet of paper that represents the city,
always incomplete and always out of date somewhere. And there is the driver, hungry, in a hurry, with divided
attention, deciding at every crossing.

The three are different things, and the map is never the territory. Yet the driver acts only through the map. Most
errors are born in the passage: **from the world to the map, and from the map to the driver's hand.**

**The queue at the health centre.** A health centre serves 12 patients per hour. On half the days 5 patients per
hour arrive; on the other half, 11. On average, 8 per hour.

The manager reasons: *"average demand is 8, we serve 12, so the wait is short and there is no reason to hire"*.
The arithmetic is right for a world in which exactly 8 patients arrive every hour.

But the world is the other one. Simulated with the same seed, the two worlds give:

| World | Mean wait per patient | In each 100 patients, how many wait more than 30 min |
| --- | --- | --- |
| Always 8 per hour | 9.1 minutes | 8 |
| Half the days with 5, half with 11 | 19.6 minutes | 24 |

No arithmetic was wrong, and the mean demand is 8 in both worlds. What changed is the **world**. When demand
oscillates, the queue of a busy day grows much more than the queue of a quiet day shrinks, because waiting time
grows **on a curve**. The manager decided for a world that does not exist. Chapter 5 explains the curve, and
Chapter 12 follows the same queue in real data.

---

### Step 2 — The rule of the game

**The three layers.**

| Layer | Code | What it is | What can go wrong in it |
| --- | --- | --- | --- |
| World | $L_1$ | The reality that generates the data, never seen whole | The physics of the noise, causal mechanisms, path dependence, heavy tails |
| Model | $L_2$ | The formal, simplified, simulable representation | Hidden assumptions, a hypothesis space that is too small, the wrong likelihood, overfitting |
| Agent | $L_3$ | Who decides: a person, an institution or an algorithm, with limited time and attention | Mental shortcuts, miscalibration, utility, actions that change the World |

The three form a loop. The World generates data, the Model interprets them, the Agent acts, and the action changes
the World again. Each arrow is a **bridge**, and each bridge has its typical error:

- **World to Model:** the data arrive filtered. Only the patients who did not give up are counted; only the
  companies that survived appear in the database.
- **Model to Agent:** the prediction is misread. A likelihood is taken for an answer; a mean is taken for a
  typical case.
- **Agent to World:** the action deforms what it wanted to measure. A forecast of a bank run causes the run; a
  target turns into a goal to be gamed.

**The bridge thesis.** *A fallacy is almost always not an arithmetic error but a misalignment between layers.* The
thesis does **not** claim that wrong arithmetic never happens, nor that anyone who follows it stops erring. It says
where to look first.

**What would refute the thesis.** A claim deserves trust only if a test could have failed it. Here is a fair test.
Take a set of analyses, some with a planted error and some clean. Give them, blind, to two groups: one that checks
every bridge systematically, and one told only to "analyse rigorously". An external judge, who does not know which
group wrote what, scores how many planted errors each group finds and how many false alarms it raises. If the
bridge-checking group finds no more errors than the other, the thesis gains nothing.

The design has a trap. If the planted errors come from a list of well-known mistakes, both groups find all of them,
and the result is a **tie at the ceiling**: every defect found, no false alarm, in both groups. A tie at the
ceiling does not support the thesis, and it does not refute it either. It says the test was too easy to tell the
groups apart. A fair test needs errors harder than the textbook ones, chosen by someone outside the group being
tested, and human analysts and judges.

**The three forms of inference (Peirce).** Charles Sanders Peirce obtained deduction, induction and abduction by
permuting three propositions: a rule ("all beans in this bag are white"), a case ("these beans come from this bag")
and a result ("these beans are white").

| Form | Goes from | To | Preserves the truth? |
| --- | --- | --- | --- |
| Deduction | Rule and case | Result | Yes |
| Induction | Case and result | Rule | No: it amplifies and can fail |
| Abduction | Rule and result | Case (the best explanation) | No: it amplifies and can fail |

Abduction chooses the hypothesis that makes the data least surprising, but only among the hypotheses considered.
It is the ancestor of maximum likelihood and inherits its limit: the true explanation may not be on the list.
Chapter 4 calls that limit Knightian uncertainty.

**The three senses of "bias".** The word names three different things, one per layer.

- In the **World**, the bias of the process that produced the data: a survey answered only by those who wanted to
  answer.
- In the **Model**, the statistical bias of an estimator: a formula whose average result misses the true value.
- In the **Agent**, the cognitive bias of whoever reasons: seeing a pattern where there is only chance.

In all three, the error is not noise that cancels out with more data. It is a distortion that persists, and more data
only measure it more precisely.

**Biases as forgotten models of chance.** Many biases in the classical catalogue are a model of chance that was
ignored.

| Bias | The model of chance that was ignored |
| --- | --- |
| Base-rate neglect | The likelihood taken for the posterior |
| Availability, survivorship | A filtered sample taken for the population |
| Pattern illusion | The null hypothesis of pure chance |
| Gambler's fallacy | Memory attributed to a process without memory |
| Neglect of rare events | A heavy tail modelled as a normal curve |
| Overconfidence | Miscalibration |
| Motivated reasoning | The model chosen for the desired result |

**The prosecutor's fallacy.** In court, an expert says: "the chance of this DNA match occurring by coincidence is
one in a million". The jury hears: "the chance that the defendant is innocent is one in a million". These are two
different numbers. The first is $P(\text{evidence} \mid \text{innocent})$, a property of the Model. The second is
$P(\text{innocent} \mid \text{evidence})$, the answer about the World. Going from one to the other requires a
third number: how many people could have left that trace. The fallacy crosses the Model-to-Agent bridge without
paying the toll.

**The fallacy of the stable world.** The manager of the health centre made the opposite crossing: imposing a stable
Model on a World that oscillates, and projecting the queue of the average day onto every day. A second error hides in
the same queue, and it has a classical name. Asked "how long did *you* wait?", the patients answer from the busy days,
because most of them arrived on busy days. Counted per patient, the bad days weigh more than counted per day. That is
the **inspection paradox**, or length-biased sampling: whoever samples by arrival over-represents the long intervals.
Chapter 5 measures both errors.

**A model good enough for the decision.** No Model is true everywhere, and it does not need to be. It needs to be
right where the decision depends on it. Two errors of the same size on the map can cost very differently: a wrong
waiting time on the quiet days changes nothing, because nobody hires for them, while the same error on the busy
days decides the second nurse. In reinforcement learning this has a name, **value equivalence** (Grimm and
colleagues, 2020): a model of the world is good enough when the decisions it recommends are worth what they would be
worth in the real world, whatever else it gets wrong. The converse is the trap. An Agent that optimises against a
Model finds the places where the Model is wrong in its favour, and goes there. Agents trained inside a learned model
of a game learned to exploit its glitches, winning in the dream and losing in the game (Ha and Schmidhuber, 2018).
It is Goodhart's law one layer down: the Model became the target. The defence is to test the Model where the Agent
will act, not on average.

**The modeller is an Agent too.** Whoever builds the Model chooses which data to keep, which variables, which
formula, and when to stop. Each choice is an action, and the World of that Agent is the space of possible Models.
The same three bridges reappear one level up: the data the modeller sees are filtered; the fit is misread as a
fact; and the modeller's choices bend toward the answer that was hoped for. How large that layer is was measured
once, directly. Seventy-three teams received the same survey data and the same hypothesis, and reached results of
opposite signs; even after every analytic choice the teams made was coded, more than 95% of the variance of their
estimates stayed unexplained (Breznau and colleagues, 2022). That spread is a part of the uncertainty of every
published number, and no standard error contains it. Counting the attempts, and sealing the plan before looking at
the data, are the bridges of this chapter applied to the modeller.

---

### Step 3 — Hands on

**The layers as types.** A program can be made unable to confuse the layers. In Rust, each layer and each kind of
uncertainty is a type:

```rust
pub enum Layer { World, Model, Agent }

pub enum UncertaintyType { Aleatory, Epistemic, Knightian }
```

A function that expects `Layer::Model` cannot receive `Layer::Agent` by accident. Any conversion has to be written
by hand, and that line is where the reader of the code stops to think.

**The fallacy measured.** A DNA match happens by coincidence in 1 person in each 1 million. In three worlds, the
prior chance that the defendant is guilty is 1%, 0.1% and 0.0001%. What is the chance of innocence given the match?
Run `cargo run --release --bin labs -- --chapter 2`.

```text
== Chapter 2 :: the fallacy lives on the bridge ==
prevalence 1e-2: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.0001  swap factor=99.0
prevalence 1e-3: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.0010  swap factor=998.0
prevalence 1e-6: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.5000  swap factor=499999.7
```

- **The first column does not move: 0.000001.** It is the likelihood, the number the prosecutor quotes.
- **The second column is the answer.** With the same evidence, the chance of innocence goes from **0.0001** (one in
  ten thousand) to **0.5** (one in two).
- **The third column measures the lie.** Swapping the two numbers misleads 99 times in the first world and
  499,999.7 times in the last.

The arithmetic is never wrong. The error is answering a question about the World with a number from the Model.

**Exercises.**

1. *(Concept)* Classify each item as World, Model or Agent: the real arrival rate of patients; the queue formula
   used by the manager; the decision to hire two nurses; the patients who leave when they see the queue.
2. *(Concept)* A news item says: "the chance of the DNA matching by coincidence is tiny, so the chance of the
   defendant being innocent is equally tiny". On which bridge is the error? Which quantity is missing?
3. *(Count)* With a coincidence of 1 in a million, which prior chance of guilt makes the chance of innocence
   given the match equal to 0.09? Explain in one sentence why the answer depends so much on layer $L_1$.
4. *(Laboratory)* Change the coincidence from 1 in 1,000,000 to 1 in 1,000 and redo the table of the three worlds.
5. *(Concept)* Choose three biases from the table and describe, for each, an environment in which the shortcut
   stops being an error.
6. *(Concept)* In the fair test of the bridge thesis, why must the person who chooses the cases be outside the group
   being tested? What would go wrong if the same person planted the errors and analysed them?
7. *(Concept)* A route planner uses a traffic model that is accurate on average and wrong on one avenue, where it
   underestimates the delay. Why will a planner that minimises the predicted time send more and more cars down that
   avenue? Which layer failed, and where should the model be tested?
8. *(Concept)* Two analysts with the same data report effects of opposite signs. Name, for each of the three
   bridges, one choice of the modeller that could produce the difference.

**Summary.**

- World ($L_1$), Model ($L_2$) and Agent ($L_3$) are distinct layers, and most errors live on the bridges between
  them.
- The bridge thesis says where to look first; a fair test of it needs hard cases, outside selection and a blind
  judge, and a tie at the ceiling decides nothing.
- Peirce: deduction preserves the truth; induction and abduction amplify it and can fail.
- "Bias" has three senses, in the data, in the estimator and in the mind, and all of them accumulate.
- The prosecutor's fallacy swaps $P(E \mid I)$ for $P(I \mid E)$; the size of the lie depends on the World.
- A Model needs to be right where the decision depends on it; an Agent that optimises against a Model finds where
  it is wrong, so the Model is tested where the Agent will act.
- The modeller is an Agent whose World is the space of Models: the spread between analysts with the same data is
  uncertainty that no standard error contains.

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
is 50%. You can buy two tickets from him. One pays 1 dollar if it rains, the other pays 1 dollar if it does not
rain. He charges 0.60 for the first, 0.50 for the second, 1.10 in total. Whether it rains or not, **one** of
the two tickets pays 1 dollar. You receive 1.00 and paid 1.10. You lost 0.10 with no risk at all.

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

**The court's arithmetic.** In the Sally Clark case, an expert witness for the prosecution presented the number
"1 in 73 million", obtained by squaring 1/8,543, as if the two deaths were independent. The court heard that number
as the chance that the mother was innocent. That number was the chance of a coincidence; the answer about
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

Stochology does not choose one. **It types every number by the layer it came from**. The column of layers is a
convention of this book, not a theorem: a frequentist limit is also an idealisation of the Model, and a propensity is
a hypothesis about the World that only a Model can state. The typing is useful because it forces the question "where
did this number come from?", not because each interpretation belongs to one layer by nature. Many errors of Chapter
2 are swaps of reading: a frequency of the World read as a degree of belief of the Agent, or a count of equally
possible cases in the Model read as a property of the World.

**Coherence is cheap.** An agent who commits the conjunction fallacy can be Dutch-booked, but what the
bookmaker takes is bounded by the size of the incoherence, which is usually small. What a bettor who knows the world
takes grows with the distance between the agent's numbers and the world's frequencies, and making the agent coherent
does not shrink that distance. The most visible error is not the most expensive one. **What punishes disagreement with the World is
calibration** (hence Chapter 8).

**Bayes in odds.** Written in odds, Bayes' rule separates the likelihood ratio from the prior odds:

$$\frac{P(H\mid E)}{P(\neg H\mid E)}=\frac{P(E\mid H)}{P(E\mid \neg H)}\times\frac{P(H)}{P(\neg H)}$$

*Posterior odds = likelihood ratio × prior odds.* Evidence is strong when the ratio moves away from 1; the
value of $P(E \mid H)$ does not measure that strength.

**Maximum entropy.** When information is incomplete, Jaynes (1957) tells us to choose, among the
distributions compatible with the available information, the one of greatest entropy. Entropy measures how
spread out a distribution is, how little it commits to any outcome. With no information at all about a die, the
answer is the uniform distribution, one sixth per face. If the only information is that the mean is 4.5, the
answer leans towards the high faces without excluding the low ones: about 5% on face 1 and 35% on face 6. Any
other choice assumes something that was not declared.

**Decibans.** Turing and Good measured the strength of evidence in decibans, ten times the decimal logarithm
of the likelihood ratio. A ratio of 10 is worth 10 decibans; a ratio of 100, 20 decibans. On that scale,
independent items of evidence **add up**, because the logarithm turns the product of ratios into a sum. Belief
becomes a balance on which each item of evidence places its weight. Independence is the condition most often
forgotten. Two positive tests with a common cause of error are not worth double.

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

$$P_{H_0}\Big(\sup_n \mathrm{LR}_n \ge k\Big)\le \frac{1}{k}$$

For a coin of 0.6 against an honest one, with paths of 500 tosses, **0.87%** of the paths under the null reach 20
decibans (ratio 100) at some point, against a bound of 1%. The number is exact: it counts, toss by toss, the chance
of the paths that have not yet crossed.

---

### Step 3 — Hands on

The laboratory builds the tree, measures the strength of the second test and shows how much the dependence
between tests costs. Run it with `cargo run --release --bin labs -- --chapter 3`.

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
conclusion.

The second line deserves a closer look. Two tests on the same breast share the tissue that fooled the first one.
When the first test is a false positive because the tissue is dense, the second is more likely to be a false
positive for the same reason. The second result then carries less news than its nominal ratio promises, and the
decibans must be computed with the ratio that remains after the shared cause is taken into account.

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

$$\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{aleatory (World)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistemic (Model)}}$$

The first parcel is born from the draw and persists when the parameter is known. The second is born from
ignorance about the parameter and disappears when you identify it and have data.

| Uncertainty | Where it is usually first met | What more data does | How it is mitigated |
| --- | --- | --- | --- |
| Aleatory | World | Nothing | Safety margins, redundancy |
| Epistemic | Model | It reduces them to zero, if the parameter is identifiable | Sampling, Bayesian updating |
| Knightian | Model and Agent | Nothing — or worse: false confidence | Scenarios, robustness, stress on assumptions, widening the hypothesis space |

Frank Knight (1921) separated **risk**, with known probabilities, from **uncertainty**, with probabilities
that cannot be measured. This book stretches Knight's word to a neighbouring case, the one statisticians call
**misspecification** or the *M-open* setting: the real process is **outside** the hypothesis space. The two are
related, since a process nobody listed has no measurable probability, but they are not the same, and the label
"Knightian" here means misspecification first and Knight's unmeasurable odds second. That uncertainty is the most dangerous of the
three, because it does not appear in the equation. The interval narrows around the wrong answer.

The second column says where each kind is usually met first, not where it is confined. Layer and kind of
uncertainty are two independent questions, and every claim answers both. An unlisted mechanism in the World is
Knightian; the decision-maker's doubt about their own calibration is epistemic, and it lives in the Agent. What
the kind fixes is the treatment: frequency and margins for the aleatory, belief charged by calibration for the
epistemic, scope and an exit clause for the Knightian.

**The ceiling a person sets.** The law of total variance also says how well a person can be predicted. Ask the
same people the same survey twice, two weeks apart, and a share of the answers changes: that share is aleatory from
the outside, because nothing about the person that a model could learn decides it. No model of a person can predict
them better than they predict themselves, so the honest score of such a model is its accuracy **divided by the
person's own consistency**. Agents built from two-hour interviews with a thousand people reached 83% of that ceiling
on held-out survey questions, and agents built from demographics alone 74% (Park and colleagues, 2024): the first
number measures the Model, the gap to 100% is what is left to learn, and the ceiling itself is the World. The same
law warns against the opposite error. A simulated respondent asked to speak as "a voter of group X" tends to return
the most common answer of the group, so the first parcel, the variance inside the group, vanishes: in one study of
such simulated samples, most subgroups came out with almost no internal disagreement on questions where the real
survey showed plenty (Li, Li and Qiu, 2025). A sample without its aleatory parcel is not a cheaper sample; it is
another population.

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
(epistemic). Location corresponds to the layer in which the uncertainty lives, and the level says whether a
safety margin is enough or whether the assumptions themselves must be stressed. A parameter that is merely
unknown calls for more data; a model structure that may be wrong calls for rival models.

**Laplace's rule of succession.** On the eve of day 101, the turkey uses:

$$P(\text{success tomorrow}\mid s,n)=\frac{s+1}{n+2}$$

The rule is not neutral either, because it derives from a uniform prior. With seven successes in ten, the
uniform prior gives 0.667, a uniform prior over $\theta^2$ gives 0.692 and one over $\sqrt{\theta}$ gives
0.652. **Even ignorance has a shape**, and that is why the choice of prior has to be declared and its effect on
the conclusion measured.

---

**The forgetting factor.** A model that assumes a fixed world gives the same weight to the first observation
and to the last one. When the world drifts, that is a mistake: old data describe a world that no longer
exists. The remedy is to weigh each observation by a factor that decays with its age. With factor $\lambda$, an
observation of $k$ steps ago weighs $\lambda^k$, and the model remembers, in effect, about $1/(1-\lambda)$
recent observations. With $\lambda = 0.99$, that is about 100. The model gives up the precision of a long
memory in exchange for following a moving target. The choice of $\lambda$ is itself a bet on how fast the world
changes, and it must be declared like any prior.

### Step 3 — Hands on

The laboratory places a world that changes underneath a model that does not change. A coin whose bias
**climbs from 0.2 to 0.6** over 6,400 tosses, 200 repetitions, with model 1 assuming a fixed coin and model 2
discounting the past with factor 0.99. Run it with `cargo run --release --bin labs -- --chapter 4`.

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
6. *(Concept)* A model predicts a person's answers with accuracy 0.70, and the person repeats their own answers with
   consistency 0.80. What is the normalised accuracy? Which parcel of the law of total variance does the missing
   0.20 of the person belong to, and which parcel the gap between 0.70 and 0.80?

**Summary.**

- Aleatory uncertainty does not shrink with more data; epistemic shrinks; Knightian deceives.
- The law of total variance separates the first two precisely.
- A model of a person is scored against the person's own consistency, the aleatory ceiling; a simulated sample that
  loses the variance inside its groups is another population.
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

The table below returns to the two worlds of Chapter 2, and mean demand does not change from one row to the
other.

| World | Mean wait per patient | In each 100 patients, how many wait more than 30 min |
| --- | --- | --- |
| Always 8 per hour | 9.1 minutes | 8 |
| Half the days with 5, half with 11 | 19.6 minutes | 24 |

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

In words, the mean of the result is **not** the result of the average case. Equality holds when $f$ is a
straight line over the values that $X$ can take, or when $X$ does not vary. For a strictly convex curve, such as
the wait of a queue, only the second case remains: any variation leaves a difference over, the toll.

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
the 19.3 minutes that are missing. **Here the second-order approximation underestimates the toll**, and the
direction is not a law: it comes from the third derivative of $W$, which is positive, so the curve rises faster
than its parabola on the busy side. A curve with a negative third derivative would be overestimated instead.

The 29.3 minutes are themselves a long-run figure. A day of 10 hours starts with an empty queue and does not reach
the long run of $W(11) = 55$ minutes, so the simulated wait (Step 3) stays below the formula. Two corrections pull
in opposite directions: the short day lowers the wait of the surge day, and counting per patient, rather than per
day, raises the weight of the surge day, because more patients arrive on it. That second effect is the inspection
paradox of Chapter 2.

**The mean grade and the threshold.** Not every crooked function is a second-degree curve. Failing is a
**threshold**, and each student's outcome becomes a step at 6.0. The class mean answers the question "what is
the typical grade?" and does not answer the question "how many fail?". Whoever needs the second question and
uses the first is swapping the summary for the result, and the swap cannot be repaired by adding more
students.
The class of the laboratory is a mixture of two groups: 45 students in every 100 draw their grade from a normal
curve centred on 4 and 55 from one centred on 8, both with standard deviation 1. The mean is 0.45 × 4 + 0.55 × 8 = 6.2,
and the share below 6.0 is 0.45 Φ(2) + 0.55 Φ(−2) = 45.2%, which the laboratory measures as 45.1 in a draw of 100,000.

**The unit of the margin.** Every estimate comes with a margin, and the margin only makes sense if
it is computed on the **unit that is in fact independent**. At the centre, the question is this. Are the 320
thousand patients 320 thousand independent observations, or are the 4 thousand days 4 thousand independent
observations? Patients of one day share the demand of the day, the server and the queue. Treating them as
independent is **pseudoreplication**, and it makes the margin look five times smaller than it is. A test that
checks the choice is the **halves null**: split the days at random into two halves and measure the deviation of
the difference between the two means. The null measures what the margin must predict. It is not free of
convention: by splitting days, it already assumes that the days are independent of one another. A weekly calendar
or a drift across weeks would escape it, and the honest version then splits whole weeks.

**The prohibition of the solitary mean.** When the function is nonlinear, the dynamics multiplicative or the
tail heavy, reporting only the mean is prohibited. One reports the average case $f(\mathbb{E}[X])$, the
simulated mean $\mathbb{E}[f(X)]$, the median, the quantiles and the cumulative curve. Each of them answers a
different question, and the toll we have just measured is the distance between the first two. Which one is "the"
answer is not a matter of taste but of the loss. The mean is the number that minimises the expected squared error;
the median, the expected absolute error; a quantile, a loss that charges a miss on one side more than on the other,
which is the loss of a promise ("no more than 1 day in 20"). Choosing the summary is choosing whose error counts.

---

### Step 3 — Hands on

The laboratory runs four worlds of the centre, all with mean demand 8, measures the halves null in two of them
and counts the failures of the class. Run it with `cargo run --release --bin labs -- --chapter 5`.

```text
== Chapter 5 :: when the mean lies ==
W(5) = 3.57 min, W(8) = 10.00 min, W(11) = 55.00 min
Jensen: f(E[X]) = 10.00 min | E[f(X)] = 29.29 (5 or 11) and 14.46 (uniform 5..11)
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

**The resolution of the question.** The manager's error has a name that outlives the health centre. The
question "does the service hold?" can be asked at different grains. Asked of the average day, it is the coarse
question, R0. Asked of each day, it is R1. Asked of each hour, R2. Asked of each hour and of each group of
patients, R3. Each step refines the one before. When the failure is a threshold on a mean, a finer question can
only see more failures, never fewer, because an average above the line needs at least one part above it.

The cost of asking coarsely can be measured with two plans that spend the **same total resource**. The coarse
plan staffs every day for the mean demand. The fine plan staffs each day for that day's demand. Both aim at the
same utilisation, a capacity of the demand divided by 0.8, so that neither plan runs a queue at the critical
point where arrivals equal service. Both face the same arrivals and the same service draws, so the only difference
between them is the grain of the question.

```text
== R horizon :: resolution of the question ==
paired arms: same arrivals and same service bases; only the service rate changes
demand 5 or 11 (the station of Chapter 12): mean capacity 10.0 | aggregate wait 43.73 min, fine wait 18.20 min, loss 25.54 min
   above 30 min: aggregated 0.4488, fine 0.2081
demand 7 or 9 (light oscillation): mean capacity 10.0 | aggregate wait 21.15 min, fine wait 18.38 min, loss 2.77 min
   above 30 min: aggregated 0.2518, fine 0.2198
demand 8 or 8 (no oscillation): mean capacity 10.0 | aggregate wait 18.52 min, fine wait 18.52 min, loss 0.00 min
   above 30 min: aggregated 0.2209, fine 0.2209
```

With demand of 5 or 11, the coarse plan makes patients wait 43.73 minutes on average and the fine plan 18.20: the
coarse question costs **25.54 minutes** per patient. With a light oscillation, from 7 to 9, it costs 2.77
minutes. Without oscillation it costs nothing, 0.00, because then the average day is every day. On the surge day
the coarse plan offers a capacity of 10 to a demand of 11, and that queue has no long run at all. The loss of
resolution is the Jensen toll seen from the side of the decision: it exists only when the world varies, and it
grows with the variation.

**Three answers to the same question.** So far the demand varied and the formula stood still. Now let the rates
themselves be uncertain: from day to day the arrivals move around 8 per hour and the pace of the nurse around 12, and
the post has promised that the mean wait of a day passes 20 minutes on no more than 1 day in 20. Three methods answer.
The **plug-in** puts the mean rates into the formula and returns one number, with no probability attached. The
**delta method** draws the tangent plane of the formula at the mean rates: the variance of the wait is the sum of
each rate's variance times its sensitivity squared, plus twice the covariance times both sensitivities, and read as
a normal it gives a probability. Its second-order correction moves the mean by half the curvature times the variance,
Jensen again. **Monte Carlo** draws the two rates from their joint distribution, correlation included, applies the
formula to each draw and counts the days above the line. The laboratory runs the three under three correlations
between arrivals and pace (`cargo run --release --bin labs -- --propagate`):

```text
== Three answers to the same question: the wait of a day at the health post ==
W = 60 lambda / (mu (mu - lambda)) min | lambda ~ N(8, 0.6^2) per hour | mu ~ N(12, 0.8^2) per hour | question: P(W > 20 min), promise <= 0.05 | 400000 draws, seed 2026
sensitivities at the mean: dW/dlambda = +3.750 min per arrival/h, dW/dmu = -3.333 min per service/h

-- correlation between arrivals and pace: -0.5 --
  plug-in:     W = 10.00 min | no probability | decision: promise kept
  delta:       mean 10.00 (second order 11.37) | sd 4.26 | P(W > 20) = 0.0095 | decision: promise kept
  Monte Carlo: mean 12.82 | median 9.99 | sd 165.93 | q95 24.47 | P(W > 20) = 0.0884 (MC sd 0.0004) | days that never clear 199 | decision: second nurse
  budget of the delta variance 18.17 min^2: arrivals 5.06 (28%), pace 7.11 (39%), covariance +6.00 (+33%)
  the linear answer may decide: false (delta and Monte Carlo differ by 0.0789, more than half the promise, 0.025)

-- correlation between arrivals and pace: +0.0 --
  plug-in:     W = 10.00 min | no probability | decision: promise kept
  delta:       mean 10.00 (second order 10.92) | sd 3.49 | P(W > 20) = 0.0021 | decision: promise kept
  Monte Carlo: mean 11.22 | median 9.99 | sd 9.08 | q95 19.99 | P(W > 20) = 0.0499 (MC sd 0.0003) | days that never clear 14 | decision: promise kept
  budget of the delta variance 12.17 min^2: arrivals 5.06 (42%), pace 7.11 (58%), covariance -0.00 (-0%)
  the linear answer may decide: false (delta and Monte Carlo differ by 0.0478, more than half the promise, 0.025)

-- correlation between arrivals and pace: +0.5 --
  plug-in:     W = 10.00 min | no probability | decision: promise kept
  delta:       mean 10.00 (second order 10.47) | sd 2.48 | P(W > 20) = 0.0000 | decision: promise kept
  Monte Carlo: mean 10.52 | median 9.99 | sd 2.98 | q95 15.90 | P(W > 20) = 0.0107 (MC sd 0.0002) | days that never clear 0 | decision: promise kept
  budget of the delta variance 6.17 min^2: arrivals 5.06 (82%), pace 7.11 (115%), covariance -6.00 (-97%)
  the linear answer may decide: true (delta and Monte Carlo differ by 0.0107, within half the promise, 0.025)
```

Read it line by line. **10.00 in every block**: the plug-in cannot see the correlation, nor the variance, nor the
line; it answers a question nobody asked. **The delta probability against Monte Carlo**: with no correlation the
tangent promises 0.0021 and the curve delivers 0.0499; the formula climbs ever more steeply towards the day when
arrivals reach the pace, and a tangent cannot know that. **The covariance in the budget**: when busy days slow the
nurse (correlation −0.5), the two sensitivities have opposite signs and the covariance term adds to the variance
instead of cancelling it; 199 of the 400000 days never clear their queue, and Monte Carlo asks for the second nurse
that the tangent and the plug-in would refuse. When the nurse speeds up on busy days (+0.5), the same term cancels
most of the spread and the three answers agree. **sd 165.93**: the mean and the standard deviation of a curve that
blows up are not summaries; the median, 9.99 in every block, and the q95 are. **0.0499 against 0.05**: with no
correlation the promise is kept by a margin smaller than the Monte Carlo error of the count, and the honest answer is
that the post sits on its own line.

Three rules come out of the block. The output of a model under uncertainty is a distribution, not a number, and the
decision reads the part of it that the promise names, here a tail. The covariance is not a detail of the inputs: it
can double the variance or erase it, and leaving it out is a structural assumption about the world. And when the
linear answer and the simulated one disagree about the probability that decides, by more than the decision can bear,
the linear answer is not allowed to decide: the laboratory writes that rule into its last line.

**A fourth answer: the multiverse.** The three answers share a specification, and the specification hides three
choices that the sentence of the question did not make. The service time was taken as exponential, which fixes its
coefficient of variation at 1; a nurse whose consultations are more regular has a smaller one, and Pollaczek and
Khinchine give the mean wait of any service law as $W = 60\,\lambda\,(1 + c^2) / (2\,\mu\,(\mu - \lambda))$
minutes, which is the formula of this chapter at $c = 1$. The rates were taken as normal; a lognormal with the same
mean and spread is as reasonable, and never negative. And the correlation could be any of the three. Every
combination is a model someone could defend. The laboratory runs all eighteen with the same 400,000 draws, so that the
differences between them are differences of choice and not of chance, and splits the variance of the answer between
the choices (`cargo run --release --bin labs -- --multiverse`):

```text
== The multiverse of the health post: how much of the answer is the modeller's choice ==
question: P(W > 20 min), promise <= 0.05 | lambda mean 8 sd 0.6, mu mean 12 sd 0.8 per hour | 18 specifications, 400000 common draws, seed 2026
  service cv | rate law  | correlation | P(W > 20) | MC sd  | decision
        0.5 | normal    |        -0.5 |       0.0241 | 0.0002 | promise kept
        0.5 | normal    |        +0.0 |       0.0081 | 0.0001 | promise kept
        0.5 | normal    |        +0.5 |       0.0004 | 0.0000 | promise kept
        0.5 | lognormal |        -0.5 |       0.0234 | 0.0002 | promise kept
        0.5 | lognormal |        +0.0 |       0.0075 | 0.0001 | promise kept
        0.5 | lognormal |        +0.5 |       0.0003 | 0.0000 | promise kept
        1.0 | normal    |        -0.5 |       0.0884 | 0.0004 | second nurse
        1.0 | normal    |        +0.0 |       0.0499 | 0.0003 | promise kept
        1.0 | normal    |        +0.5 |       0.0107 | 0.0002 | promise kept
        1.0 | lognormal |        -0.5 |       0.0875 | 0.0004 | second nurse
        1.0 | lognormal |        +0.0 |       0.0487 | 0.0003 | promise kept
        1.0 | lognormal |        +0.5 |       0.0096 | 0.0002 | promise kept
        1.5 | normal    |        -0.5 |       0.3210 | 0.0007 | second nurse
        1.5 | normal    |        +0.0 |       0.2849 | 0.0007 | second nurse
        1.5 | normal    |        +0.5 |       0.2131 | 0.0006 | second nurse
        1.5 | lognormal |        -0.5 |       0.3211 | 0.0007 | second nurse
        1.5 | lognormal |        +0.0 |       0.2855 | 0.0007 | second nurse
        1.5 | lognormal |        +0.5 |       0.2131 | 0.0006 | second nurse

the 18 specifications: 8 say second nurse, 10 say promise kept | P from 0.0003 to 0.3211 | between-specification sd 0.1201, largest Monte Carlo sd 0.0007 (ratio 163)
between-specification variance of P, by choice: service cv 0.93 | rate law 0.00 | correlation 0.06 | interaction 0.02
  as a budget of the specification: service-cv=0.93; rate-law=0.00; correlation=0.06; interaction=0.02
the book's specification (M/M/1, normal rates, no correlation): P = 0.0499, promise kept

-- the rules declared before the run --
  R1, the 18 specifications do not agree on the decision: holds
  R2, the service cv carries more of the between-specification variance than the rate law and the correlation together: holds
  R3, the between-specification sd is more than ten times the largest Monte Carlo sd: holds
```

Three readings. **The decision is a choice of the modeller.** Eight specifications ask for the second nurse and ten
keep the promise, and the answer runs from 0.0003 to 0.3211: the spread between reasonable models is 163 times the
largest error of the simulation. **One choice carries almost all of it.** The coefficient of variation of the service
explains 0.93 of the variance between specifications; the law of the rates, which looked like a serious decision,
explains nothing to the second decimal. That is the share the delta budget cannot see, because it lives above the
formula, not inside it. **The question it asks is cheap to answer.** Nobody measured how regular the consultations
are, and that one measurement, a stopwatch on a week of consultations, decides the nurse; it is the value of
information of Chapter 11 pointed at the model instead of at the world. A budget of the uncertainty that stops at the
inputs reports the uncertainty of one model; the uncertainty of the answer includes the choice of the model, and the
honest card declares both.

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
7. *(Count)* With $\lambda = 8$ and $\mu = 12$, compute the mean wait for $c = 0.5$, $1$ and $1.5$. Which value of
   $c$ makes the mean wait at the mean rates equal to 20 minutes? What would you measure first at the post, and why?

**Summary.**

- The mean of the input is not the mean of the output when the function is curved. The result is
  $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$ for convex $f$.
- The toll grows with the curvature and with the variance; the second-order formula is local, and in the queue,
  whose third derivative is positive, it **underestimates** the toll when the dispersion is large.
- Mean demand does not move, and the waits do not look alike. Whoever simulates the world sees what the
  spreadsheet hides.
- The margin is only worth what the independent unit is worth, and the halves null decides which unit is the
  right one.
- A threshold (failing, overflowing, going bankrupt) is not summarized by any mean.
- The uncertainty of an answer includes the choice of the model. Run the reasonable specifications with the same
  draws and split the spread between the choices: one unmeasured quantity can carry almost all of it.

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
answering that depends on the vote, the simulation gives:

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
roster and the weather. They repeat, they do not add up. The independent unit there is the **day**: there are
**81** independent units, and the margin has to be computed over them. That does not make 2.65 million calls worth
exactly 81 calls; it makes them worth much less than 2.65 million. That is why a report that treats the calls
as independent promises a precision that the real unit does not sustain.

---

### Step 2 — The rule of the game

**Meng's identity.** Meng (2018) showed that the error of any sample mean decomposes into
three factors, whose product is exact:

$$\bar Y_n - \bar Y_N \;=\; \underbrace{\rho_{R,Y}}_{\text{quality}}\times
\underbrace{\sqrt{\tfrac{1-f}{f}}}_{\text{quantity}}\times \underbrace{\sigma_Y}_{\text{difficulty}}$$

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
conditioning on a **collider**. In a simulation, talent and likeability have correlation $+0.001$ in the
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
observations were independent. Two different numbers hide in that sentence and must not be confused. The
**number of independent units** is the count of clusters: 364 days of New York flights, 81 days of calls. The
**effective size** is how many independent rows would give the same margin, and with clusters it follows the
design effect, $1 + (m - 1)\,\rho$, where $m$ is the size of a cluster and $\rho$ the correlation between rows of
the same cluster. Only when the rows of a day are identical ($\rho = 1$) does the effective size fall to the number
of days. At the bank the margin per call comes out about fourteen times narrower than the margin per day, so the
design effect is about two hundred and the 2.65 million calls weigh like some thousands of independent calls, far
fewer than 2.65 million and far more than 81. Treating the
patients of one day, or the calls of one hour, as independent observations is
**pseudoreplication**: the margin comes out too small and the test approves what it should not.

**One formula behind all of them.** The variance of a sum is the sum of the variances **plus twice every covariance**:

$$\operatorname{Var}\Big(\sum_i X_i\Big) \;=\; \sum_i \operatorname{Var}(X_i) \;+\; 2\sum_{i<j} \operatorname{Cov}(X_i, X_j)$$

Every effective size in this book is this line. The patients of a day share its demand, so their covariances are
positive and the margin per patient forgets them (Chapter 5); the letters of *Onegin* alternate, so theirs are
negative and the sample is worth more than its count; the respondents of the *Literary Digest* share a reason to
answer, which is Meng's $\rho$; the players who pool in Chapter 10 cancel each other's swings; two sources of error
moved by the same cause, the same rain on two sensors or the same evening on every route, err together. Independence
is sufficient for the cross terms to vanish, not necessary, and it is never free: leaving the covariance out is not a
simplification, it is a claim about the world, and Chapter 5 shows a case where the claim is worth a second nurse.

**The call-centre case, in full.** The sector claimed the following. Staffing each half hour by
Erlang C on the historical mean rate would guarantee 82% of calls answered within 20 seconds, every
day. In the 81 days of the vault, the standard rule fails on **26 out of every 100 days** and delivers 86%. The variation
between days accounts for most of it, since even the world that keeps only the real total of each day, with the
mean profile, already loses 17% of the days. Erlang C, applied to the real rate of each day, gives **57%**, instead of the claimed 82%, and 65% of the days fall below the target; it is Jensen inside the formula (Chapter 5), without
needing simulation. The unit comes out of the halves null. The deviations are 0.0474, against 0.0470 predicted
by the day and 0.0033 predicted by the call. The per-call reading promises a margin fourteen times smaller.
In 400 days simulated in the world of the claim, none fell below the target; in the real one, about one in four.
The decision that comes out of this costs **+7.2% of agent-hours**, 2,428 per day against 2,264, and takes the failure
from 26 down to **less than 4 in every 100 days**. The sensitivity grid, declared before running, sweeps service from 3 to 6 minutes against patience from 1 to 10
minutes, 24 points. The winning rule is right in 20 of them; in the four remaining, all with 1 minute of
patience, the four rules tie because none leaves more than 1 day below the target. There is no turning
point. In the worst corner of the grid, not even the best rule holds the target. It is 14.2% of the days below it, against
37.7% for the rule of the claim.

---

### Step 3 — Hands on

The laboratory rebuilds the 1936 poll, measures the correlation that the filter creates, redoes the
Simpson table and measures the effective size of a chain with memory. Run it with
`cargo run --release --bin labs -- --chapter 6`.

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
the standard deviation of a sample of **7.2 people**. The agreement to the third decimal is not evidence: Meng's
identity is algebraic, so it holds for any sample, and the two response rates were chosen to reproduce the
known result of 1936. What the simulation shows is the mechanism, not that it was the cause; historians still
weigh the frame (telephones and subscribers) against the non-response of whoever did not mail the ballot back. **(2)** The margin of error of the poll (0.0276 point) is no margin at all against a poison of
18 points; they are 661 margins apart. **(3)** The correlation between talent and likeability goes from $+0.001$
in the population to **−0.679** among those selected with cutoff 1.5. The filter **manufactures** the negative
association, and it gets stronger the narrower the door (−0.617 with 24% selected, −0.732 with
7.9%). **(4)** The effective size follows the process. There are 100 letters that are worth 101 observations when there is no
memory, 25 when the chain is sticky (persistence 0.8) and 327 when the letters alternate (persistence
0.235), the number Markov found in *Onegin*.

**Exercises.**

1. *(Concept)* A company says: "we have millions of users, so our satisfaction survey is
   precise". Which factor of Meng's identity did it forget, and what does it have to do with the *Literary
   Digest*?
2. *(Count)* In the Simpson table, recompute the raw rate of each surgery as the mean of the strata
   weighted by the number of cases (87, 270, 263 and 80; the successes were 81, 234, 192 and 55) and explain why it
   reverses the strata. Which is
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
  unit, and the number of independent units is a floor for the effective size, not the effective size itself.
- 2.4 million biased ballots are worth 7 people, and the margin of error of a biased poll measures
  nothing at all.

## Chapter 7. Hunting ghosts in the noise: severe tests and false discoveries

**Objectives.** By the end of this chapter you should know how to build the exact null model of an experiment and
compare against it; recognise the researcher freedoms that inflate false positives; compute the predictive
value of a finding with Ioannidis' formula; name the five types of error; and distinguish a shortcut that is an error from a shortcut that is an adaptation to the environment.

---

### Step 1 — The real world and the paradox

**One hundred people tossing coins.** Gather 100 people. Each one tosses an honest coin 20 times and writes down the
longest run of heads obtained. Nobody cheats. Even so, in **100 out of every 100 experiments** of a simulation,
someone strings 6 heads in a row or more, and the longest run is, on average, **9.5 heads**. If the reporter chooses to tell the story of the person with the longest run,
tomorrow's paper carries a talent that does not exist. There was no measurement error, only a forgotten
**null model**.

**The hot hand.** The most famous case is the "hot hand" in basketball. Gilovich, Vallone and Tversky (1985)
analysed the shots of professional players and treated the belief that whoever hits tends to
hit again as a pattern illusion. The proportion of hits after a hit was equal to the general
proportion. For three decades the result was cited as the canonical example of illusion.

In 2018, Miller and Sanjurjo showed that the **test** was biased. The original analysis computed, player by
player, the proportion of hits after a run of hits and compared it with the proportion after a run of misses,
expecting no difference under pure chance. But the proportion of "hit after hit", averaged over finite sequences
of a process with no memory at all, **is below the hit rate**: with a fair coin it is not one half. In three tosses of an honest coin, the mean of the
proportions per sequence is $5/12 = 0.4167$. The target was in the wrong place. With the count corrected, the
difference in the Cornell data goes from $+3$ to $+13$ percentage points. The ghost was not in the
players. It was in the denominator.

**The garden of forking paths.** Simmons, Nelson and Simonsohn (2011) asked
researchers to declare the plan before collecting the data. Four common decisions are:
*when to stop collecting*, *which variables to use*, *which cases to exclude* and *which comparisons
to report*. Each one looks reasonable on its own. Together, in an experiment in which **there is no effect
at all**, they took the false-positive rate from 5% to **60.7%** in their simulations. In every 100 studies
done this way, 61 show a "significant effect" that does not exist. The exact figure depends on how many
variants of each decision are tried; what does not depend on it is the direction, because every extra path is
one more chance for noise to cross the line.

**Fifty predictors, nothing.** An analyst has 25 training cases and 50 measured variables, with no
relation to the response. They fit 500 different models and keep the one with the smallest validation error.
On validation, the winner looks better than "predicting zero" in **79.5% of the repetitions**; on fresh data,
in only 30.0%. The advantage vanishes because there is no wrong count and no invented datum: only 500 shots at a target that
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
one-sided test that produces $p = 0.04$, the severity of the claim "there is a positive effect" is $1 - p = 0.96$:
had there been no effect, the test would have produced a less extreme result in 96 out of 100 times. When the
"$p = 0.04$" comes out of a garden of paths, the probability that matters is no longer $1 - p$, but $1$
minus the false-positive rate **of the procedure**, because what produced the number was the whole
procedure. That second number is an analogue at the level of the procedure, in the spirit of Mayo, not her
formula, which is defined claim by claim.

**Ioannidis' theorem.** The predictive value of a positive finding depends on the power $1-\beta$, on the
pre-study odds $R$ of true against false hypotheses in the field and on the significance level $\alpha$:

$$\text{PPV}=\frac{(1-\beta)R}{R-\beta R+\alpha}$$

In words: even a well-done study can have little chance of being right, if the field tests many
false hypotheses. With power 0.80 and $\alpha = 0.05$, if there is one true hypothesis for every ten false ones
($R = 1/10$), the PPV is 0.615; if one for every thousand ($R = 1/1000$), it falls to 0.016. If $\alpha$ is the garden's
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
probability; Simon and Gigerenzer answered that a shortcut is an error only **relative to an environment**. Stochology
proposes a criterion instead of choosing a side. A departure is an **error** when the right
information was available and the combination rule was violated; it is a **candidate adaptation** when the norm
assumes information that the environment does not supply reliably.

The two numbers that separate the cases. Take the mammography test of Chapter 3 (90 in 100 sick women test
positive, 9 in 100 healthy ones too). Whoever ignores the base rate forecasts as if the prior were one half: 0.909
after a positive, 0.099 after a negative. In a screening with prevalence 1%, that forecaster has an expected Brier
score of **0.083**, against **0.009** for whoever uses the base rate. In a referral clinic, where the real prevalence
is 33%, whoever insists on 1% has **0.28**, against **0.085** for whoever ignores it. A single
heuristic is an error in one environment and a success in the other. The Brier score is the subject of Chapter 8.

**The safeguard panel.** In a research process, the defence against the garden is a formal attack, not
a good intention. Eight bands name the attack that each part of the artefact should survive, and demand the
safeguard that answers it:

| Band | The attack it names |
| --- | --- |
| Code | A new battery of properties, with a seed the author did not choose |
| Data | A batch with corrupted records, which has to be isolated |
| Model in operation | A batch with strong drift, live |
| Inference | A pathological data set, to see whether the uncertainty denounces the inadequacy |
| Governance | Tampering with the audit trail, which the chain verifier has to detect |
| Fairness | Inverting the chosen metric and showing who would start paying |
| Domain | An atypical scenario of the domain, with an alternative prior, to see whether the decision holds |
| Decision | A crisis that violates the exit clause itself |

Approval requires that **all** eight bands pass, and a band passes when its safeguard is declared and anchored
on the specification sheet:

$$\text{Approved}(X) = \bigwedge_{i=1}^{8} C_i(X),\qquad C_i(X) = \text{safeguard}_i(X) \wedge \text{anchor}_i(X).$$

One warning about the word "attack". In the machine of this book no band runs the attack in the table. Each band
reads two bits from the sheet, and the panel is therefore a **checklist of safeguards**. It shows that a safeguard
was declared and anchored, not that it works, and it is not Mayo's severity. Until 2026-10-19 each band printed a
"severity" from 0 to 10, drawn by a seed inside a range the bits selected; the number added nothing to the bits and
borrowed the name of Mayo's concept, so it was removed (Appendix E). The attacks that do run (the audit trail
tampered on purpose, the planted defects) live in the gates of Chapter 17. The rule is hard on purpose. No average of the bands approves anything, and one weak
band declared is worth more than uniform work without an admitted gap. No agent approves the
work it produced. Three failures eliminate the work before any discussion of merit: temporal leakage, a data
contract violated without treatment, and the absence of a named human owner.

---

### Step 3 — Hands on

The laboratory measures four things: the garden of models on pure noise, the exact null model of the hot hand,
the garden of paths and the verdict of the safeguard panel. Run it with `cargo run --release --bin labs -- --chapter 7`.

**The recipe of the laboratory.** *The garden of models*: in each of 200 replicas, 50 predictors and an outcome are
drawn as independent standard normals for 75 rows, 25 to fit, 25 to validate and 25 fresh; each of 500 candidates
takes between 1 and 5 predictors at random and is fitted by least squares with an intercept; the winner is the one
with the smallest mean squared error on validation, and "beating zero" means an error below 1.0, the error of
predicting zero. *The garden of paths*: in each of 4,000 experiments, two groups of 40 with no effect; two outcomes
built from correlated normals plus 0.4 of a covariate; tests are two-sample t tests with the normal approximation, or
a regression of the outcome on the group and covariates. The pre-registered test compares the first 30 of each group
on the first outcome. Simmons' four freedoms take the smallest p over both outcomes, 20 or 30 per group, with or
without the first covariate, and the exclusion of the largest value of the treated group. The large family adds
optional stopping at 10, 20, 30 and 40 per group, up to three covariates, and the two subgroups of a coin-flip sex.

```text
== Chapter 7 :: hunting ghosts in the noise ==
model garden: 200 replicas, 500 candidates over 50 pure-noise predictors, 25 training cases
the validation winner beats predicting zero in 79.5% of the replicas; on fresh data, in 30.0%
mean squared error of the winner: 0.814 on validation (predicting zero: 1.000) and 1.191 on fresh data
hot hand: mean of the proportions per sequence = 0.4167 (5/12 = 0.4167); pooled proportion = 0.5000
100 people tossing 20 coins: in 100.0% of the experiments someone strings 6 heads in a row; the longest run averages 9.5
garden of paths (4000 experiments with no effect at all): with Simmons' four freedoms, someone finds p < 0.05 in 20.3% of them; with the large family, in 42.3%; the pre-registered test flags 5.2%
Mayo severity: 1 - p = 0.96 in the pre-registered test with p = 0.04; 1 - false-positive rate = 0.58 in the garden
Ioannidis with R = 1/1000, power 0.80: PPV = 0.016 with alpha 0.05 and 0.002 with the garden alpha
Ioannidis with R = 1/10, power 0.80: PPV = 0.615 with alpha 0.05 and 0.159 with the garden alpha
Ioannidis with R = 1/3, power 0.80: PPV = 0.842 with alpha 0.05 and 0.387 with the garden alpha
safeguard panel A1-A8, card with the eight safeguards: bits [1, 1, 1, 1, 1, 1, 1, 1], approved = true
the same card without one band's safeguard: 7 of 8 bits set, approved = false
```

Five readings. **(1)** The winner of the garden of models has error 0.814 on validation, against 1.000 for
"predicting zero". It looks great, and the error rises to **1.191 on fresh data**. It is worse than zero, and that is why
it "beats zero" only 30% of the time when the judge is new. Validation was used as a target, and the target turned into
poison. **(2)** The exact null model of the hot hand is 0.4167, not 0.5; the mean of the proportions per
sequence coincides with 5/12 to four decimal places, while the pooled estimator sits at 0.5000.
The ghost is the difference between the two. **(3)** The four classical freedoms take the false
positive from 5.2% (pre-registered test) to **20.3%**, and the large family, with optional stopping at
four points, three sets of covariates, exclusion of the extreme case and two subgroups, takes it to
**42.3%**. **(4)** The Mayo severity of the pre-registered "$p = 0.04$" is 0.96; that of the number that comes
out of the garden is 0.58, because what produces the number is the procedure. **(5)** The A1–A8 panel sets the
eight bits on a sheet with the eight safeguards and approves it; one band without its safeguard leaves 7 of 8 bits
set, and the approval vanishes. The AND of the eight does not forgive. Read the contrast with reading (4): Mayo's
severity is a probability about a procedure, and it can be 0.96 or 0.58; the panel's verdict is a bit about a
declaration, and it carries no probability at all.

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
5. *(Concept)* Until 2026-10-19 the panel printed, for each band, a "severity" drawn between 8 and 10 when the
   safeguard existed and between 2 and 4 when it did not, against a floor of 6. Show that the verdict was the same
   as the AND of the eight bits under every seed. What did the number add, and why is calling it "severity" a
   misuse of Mayo's word?
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
  (flipped sign) and M (exaggeration). The defence is institutional: Mayo's severity and a test that could have
  failed, with the A1–A8 safeguard panel as a checklist of bits that makes the missing safeguard visible.

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
every 100 days on which he announced 20%, it rained about 24 times in the simulated city, close to what he said. He is
nearly calibrated, and useless for carrying an umbrella: every one of his warnings is the same warning, the rate of
the place. The
expert **separates** the days. On a day on which he says 70%, it rains far more than on a day on which he
says 10%. On one side, a calibrated forecaster with no resolution; on the other, a calibrated one with resolution.

Measured in a world of 4 thousand days, the numbers are these:

| Forecaster | Brier score | Calibration error (REL) | Resolution (RES) |
| --- | --- | --- | --- |
| Lazy (0.20 always) | 0.1858 | 0.0018 | 0.0000 |
| Expert | 0.1569 | 0.0012 | 0.0267 |

The lazy one has almost no calibration error (0.0018) and no resolution at all (0.0000, and exactly zero by
construction: a constant forecast cannot separate anything). He gets the frequency nearly right and distinguishes
nothing. The expert has an even smaller calibration error and a resolution of 0.0267 where the lazy one has none.
That is why he is worth more, and that is what the accuracy rate would not see.

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
variance/mean ratio is **273**, and the model underestimates by tens of times the chance of a peak day (**180 against 2,900** calls). A model that gets the mean value right and the dispersion wrong gets the day that matters wrong.

**A proper score also combines.** A proper rule judges one forecaster. It can also choose how much to trust each of
several. Bayesian model averaging weights each model by its posterior probability, which answers the question
"which of these models is true?". When one of them is, the weight goes to it, as it should. When none is, the case
that Chapter 4 calls misspecification, the question has no right answer, and the average still picks one model,
because the evidence for one always ends up overwhelming the other. It also counts what should not count: list the
same model twice and its prior mass doubles. **Stacking** (Yao, Vehtari, Simpson and Gelman, 2018) asks a different
question, "which mixture of these predictions predicts best?", and answers it with a proper score: the weights that
maximise the logarithmic score of each point predicted by the models fitted without it. A copy of a model adds no
prediction, so it changes nothing.

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
noisy after recalibration, because the noise was not in the scale, it was in the signal. The asymmetry is
about **recalibration**, not about the model: resolution cannot be bought afterwards by a map from $p$ to $p'$,
but it can be bought before, with new information (a better variable, a finer unit, a shorter horizon).

Measured in the laboratory, with Platt fitted on the first 2,000 days and scored on the last 2,000, which it
never saw, an overconfident forecaster goes from Brier 0.1708 to **0.1555**, practically the value of the
recalibrated expert (0.1547), and almost all of the difference was scale distortion. A noisy forecaster goes from
0.2147 to **0.1741**, an improvement that still does not reach the expert. Recalibrating the constant forecaster
only replaces his 20% by the base rate of the fitting half; it cannot give him a second number.

**How much calibration can be charged.** The ECE tolerance is the practical point. The rule of the
$\mathrm{ECE} \le 0.03$ type looks austere, but it is not. The ECE of a **perfectly calibrated** forecaster
is not zero, but a random number whose scale moves with $1/\sqrt n$. The honest tolerance is the 95% quantile
of that null, computed with those forecasts at the $n$ of the vault, and it shrinks as $n$ grows. In a
vault of 1,488 cases the median noise is 0.0292 and its 95% quantile 0.0425; in one of 104,489, the 95% quantile
is 0.0037. The quantile is itself drawn at random, and it moves with the seed: under twenty other seeds the 0.0425
ranges from 0.0409 to 0.0424, so the seed printed here happened to give the largest value, and the honest report is
"about 0.042". A fixed seed makes a simulated number reproducible, not exact. A fixed tolerance is, then, two things. **Too tight** in the small vaults, where it fails an honest
forecaster, and **too loose** in the large ones, where it lets through perfectly detectable miscalibration.

Two different questions hide here, and Royall's distinction of Chapter 3 separates them. *Can the miscalibration
be told apart from noise?* is answered by the null quantile, which shrinks with $n$. *Is the miscalibration large
enough to matter for the decision?* is answered by a tolerance fixed by the cost of the error, which does not
shrink with $n$. In a vault of a hundred thousand cases almost any deviation is detectable, including deviations
too small to change any action. The honest report gives both verdicts: detectable or not, and relevant or not.

Six real binary vaults show it. Four are rebuilt beside the book in Chapter 20; the two marked come from the record of
the research.

| vault | n | observed ECE | null q95 | fixed tolerance 0.03 |
| --- | --- | --- | --- | --- |
| NY flights 2013 | 82,609 | 0.0260 | 0.0046 | passes |
| cyclones | 3,813 | 0.0121 | 0.0107 | passes |
| Toronto prisons 1997–2002 | 1,488 | 0.0354 | 0.0315 | fails |
| Toronto prisons 2020–2021 (record) | 16,933 | 0.0668 | 0.0115 | fails |
| NY flights 2023, with the correction | 104,091 | 0.0177 | 0.0034 | passes |
| Rhode Island 2013–2015 (record) | 24,466 | 0.0048 | 0.0034 | passes |

The null q95 of the table was computed with the real forecasts of each vault. Against it, all six vaults show
**detectable** miscalibration, including the four that the fixed rule approved. Detectable is not yet relevant:
Rhode Island misses by 0.0048, a deviation that the null detects and that changes no decision taken on an
arrest forecast. The Toronto 1997–2002 case is the fragile one. Its ECE of 0.0354 sits just above the q95 of 0.0315
computed with the real forecasts, and below the q95 of 0.0425 that the laboratory measures with simulated
forecasts of the same size. The verdict on that vault depends on the spread of its forecasts, and it is reported as
borderline, not as a failure.

**Resolution is the hard part.** The Rhode Island forecast of arrests at traffic stops ended with ECE 0.0048 and skill
$+0.019$, that is, good calibration and low resolution. Prison at a traffic stop is a rare event and
not very predictable from the available variables, and the model makes up for that with Platt (intercept and
slope) and with the circuit breaker, which puts the reserve as the official forecast in 12% of the vault. A forecast
can be honest and uninformative at the same time, and the two things are reported **separately**, because
the first is repaired and the second is not.

**Putting calibration to the test requires a new vault.** In the flight forecast, an intercept correction
($\delta \leftarrow \delta + 0.002(y - p')$) moves the level after each known arrival. In the quarter of 2013 it fixed
the bias and made the ECE worse. Its honest test needed a vault nobody had seen, the 435 thousand New York flights of
2023, in the same quarter: there the raw model missed the level and failed, with ECE 0.0646, and the correction took it
to 0.0177 and passed (Chapter 20). The correction was decisive, and it had to be shown in a vault that it had not seen.

**A guarantee that comes for free.** **Conformal prediction** gives guaranteed coverage without assuming the
shape of the distribution, $P(Y \in C(X)) \ge 1-\alpha$. It needs only that the new cases be exchangeable with
the ones used to set the interval; under drift, an adaptive version widens or narrows the interval as the
recent misses accumulate. One rule holds for every forecast. Every forecast is scored by a proper rule **outside the fit**, and no
forecast declares 0% or 100%.

---

### Step 3 — Hands on

The laboratory scores four forecasters, recalibrates all of them with Platt and measures the ECE tolerance at three
vault sizes. Run it with `cargo run --release --bin labs -- --chapter 8`.

**The recipe of the laboratory.** Each of 4,000 days has a real chance of rain $\sigma(z)$ with
$z = -1.386 + 1.1\,\varepsilon$, $\varepsilon$ a standard normal (a median of 20%), and it rains with that chance.
The lazy forecaster says 0.20; the expert says $\sigma(z + 0.6\,\varepsilon_1)$; the overconfident one
$\sigma(2.2\,(z + 0.6\,\varepsilon_2))$; the noisy one $\sigma(z + 1.6\,\varepsilon_3)$, each $\varepsilon_i$ a fresh
standard normal. REL, RES and the ECE use ten boxes of equal mass that never split a run of equal forecasts. Platt is
the logistic regression of the outcome on the logit of the forecast, fitted on days 1 to 2,000 and scored on days
2,001 to 4,000. The null ECE of a vault draws its forecasts as $\sigma(\operatorname{logit}(r) + s\,\varepsilon)$, with the
rate $r$ and spread $s$ of each vault (Toronto 0.50 and 1.0, Rhode Island 0.0226 and 1.6, the 2023 flights 0.130 and
1.3), draws each outcome with the chance its forecast states, and repeats 2,000 times.

```text
== Chapter 8 :: the forecaster's thermometer ==
4000 days, observed base rate = 0.243, mean chance declared by the expert = 0.252
Platt fitted on days 1-2000 and scored out of the fit on days 2001-4000 (intercept a, slope b):
      lazy (0.20 always): Brier 0.1858 | REL 0.0018 RES 0.0000 UNC 0.1840 | ECE 0.0430 | out of fit: Brier 0.1831 -> 0.1817 with Platt, ECE 0.0385 -> 0.0090 (a = -1.11, b = 0.00)
                  expert: Brier 0.1569 | REL 0.0012 RES 0.0267 UNC 0.1840 | ECE 0.0281 | out of fit: Brier 0.1558 -> 0.1547 with Platt, ECE 0.0345 -> 0.0204 (a = -0.18, b = 0.82)
           overconfident: Brier 0.1712 | REL 0.0150 RES 0.0275 UNC 0.1840 | ECE 0.1109 | out of fit: Brier 0.1708 -> 0.1555 with Platt, ECE 0.1119 -> 0.0129 (a = -0.21, b = 0.37)
                   noisy: Brier 0.2119 | REL 0.0363 RES 0.0083 UNC 0.1840 | ECE 0.1449 | out of fit: Brier 0.2147 -> 0.1741 with Platt, ECE 0.1544 -> 0.0200 (a = -0.78, b = 0.28)
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
lazy one has REL 0.0018 and RES 0.0000. It is nearly calibrated and remains useless. **(3)** Out of the fit, the
overconfident one falls from 0.1708 to 0.1555, close to the recalibrated expert (0.1547), because it was mostly
scale. The noisy one falls from 0.2147 to 0.1741 and **does not reach** the expert, because it was signal, and lost
signal does not come back. **(4)** The tolerance of
0.03 is failed by an honest forecaster in **45.1%** of the simulations when $n = 1,488$, and in **0.0%**
when $n$ is 24,466 or 104,489. In the large vaults it almost prohibits nothing. **(5)** The null depends on the
forecasts, not only on $n$. With $n = 1,488$, the q50 of the null ECE goes from 0.0320 with concentrated forecasts
to 0.0243 with spread forecasts.

**Calibration has a grain too.** The resolution of the question, met in Chapter 5, raises the same problem
of sample size. A light oscillation of demand, between 7 and 9, costs little to ignore:

```text
demand 7 or 9 (light oscillation): mean capacity 10.0 | aggregate wait 21.15 min, fine wait 18.38 min, loss 2.77 min
   above 30 min: aggregated 0.2518, fine 0.2198
```

The coarse plan loses 2.77 minutes per patient, and the share of patients waiting more than 30 minutes moves
from 0.2198 to 0.2518, about three points. A difference of that size is of the order of the calibration noise of a
small vault. Whether a coarse question can be caught costing something depends, like the calibration
tolerance, on how many cases there are to see it.

**Combining, measured.** Two confident experts each model one side of a quantity: the left one is nearly sure its
centre is at −2, the right one at +2, both with spread 1. In one world half the points come from each side, so neither
expert is right; in the other every point comes from the left. The laboratory fits both on 40 points, combines them
by model averaging and by stacking, scores the mixture on 2,000 fresh points, and repeats on 1,000 datasets
(`cargo run --release --bin labs -- --stacking`):

```text
-- M-open: half the points from each side, neither candidate is true --
  log score: BMA -4.8251 | stacking -2.0811 | best single model -4.9817 | stacking - BMA +2.7441 [+2.7007; +2.7874]
  weight on the left model: BMA mean 0.485 (above 0.99 or below 0.01 in 926 of 1000) | stacking mean 0.499
  left model listed twice: stacking score moves by 0.00000 on average, its left family weighs 0.499 (was 0.499) | BMA score moves by 0.03748, its left family weighs 0.491 (was 0.485)

-- M-closed: every point from the left, the left candidate is true --
  log score: BMA -1.4189 | stacking -1.4202 | best single model -1.4189 | stacking - BMA -0.0013 [-0.0016; -0.0010]
  weight on the left model: BMA mean 1.000 (above 0.99 or below 0.01 in 1000 of 1000) | stacking mean 0.998
  left model listed twice: stacking score moves by 0.00000 on average, its left family weighs 0.998 (was 0.998) | BMA score moves by 0.00000, its left family weighs 1.000 (was 1.000)
```

**When no model is true, averaging bets on one.** In 926 of the 1,000 datasets the average puts more than 0.99 on one
expert, and which one depends on the luck of the draw; its score, −4.8251 per point, is barely better than the best
single expert's. Stacking keeps both near one half and scores −2.0811, a gain of 2.7 units of log score per point,
with an interval far from zero. **A copy changes the average and not the stack.** Listed twice, the left expert moves
the average's score by 0.03748 per point and the stack's by nothing. **When one model is true, the two agree.** In the
second world both put everything on the left expert, and the stack pays a small price for not knowing that in advance,
0.0013 per point. Model averaging is the right tool for the question it asks; the question of the decision is
usually the other one.

**Exercises.**

1. *(Concept)* Why is the accuracy rate not a proper rule? Give an example of two forecasters
   with the same accuracy rate and very different Brier scores.
2. *(Count)* A forecaster declares 0.30 on 200 days and it rains on 48 of them. Compute REL and RES in that
   box, assuming base rate 0.24, and say what that forecaster should do with the scale.
3. *(Count)* The null ECE moves with $1/\sqrt n$. If the null q95 is 0.0425 with $n = 1,488$, what do you
   expect with $n = 148,800$? Compare with the measured q95 of 104,489 and say whether the growth matches the
   prediction.
4. *(Laboratory)* In the laboratory, replace Platt recalibration with a simple intercept shift
   ($\sigma(\text{logit}(p) + a)$, fitting only $a$) and redo the four lines. In which
   forecaster does the slope matter?
5. *(Laboratory)* Change the world seed from 2026 to 1909 and to 2027 and measure the expert's Brier
   in the three. How far apart do the three sit, and what does that say about comparing two
   forecasters in a world of 4 thousand days?
6. *(Concept)* Write, in four lines, the memorandum that explains to the board why the ECE of 0.0354
   in Toronto 1997–2002 cannot be read with the ruler of the ECE of 0.0177 of the 2023 flights.
7. *(Concept)* A committee weighs three forecasting models by how probable each is given the data, and two of the
   three are the same model fitted by two teams. What happens to the weights? What would stacking do, and why?

**Summary.**

- Only a proper rule scores belief: the Brier score punishes whoever lies about what they believe.
- $\text{BS} = \text{REL} - \text{RES} + \text{UNC}$, and the identity closes exactly with the forecasts
  pooled into boxes.
- Calibrated and without resolution is the lazy one: right on average and useless on each day.
- Calibration is repaired afterwards (Platt); resolution is not: lost information does not come back.
- The calibration tolerance is computed from the sample size: 0.03 is too hard in a thousand five hundred
  cases and too loose in a hundred thousand.
- A proper score also combines: stacking weights the predictions by their out-of-sample log score; model averaging
  asks which model is true, bets on one when none is, and counts a listed copy twice.

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
open surgery. Its size, 5.37 points, is a weighted mean of the differences inside the strata, 6.44 and 4.25 points, and
so it sits between them: standardising never produces a gap larger than every stratum.

Nothing in that table is wrong. Between the second and the fourth row what changes is **the question**.
*Among those operated on, which technique did better?* and *for the next patient, which technique to choose?*
are two questions, and the answer to the first does not serve the second. That distinction has a name and a
ladder.

**The three steps.** Most headlines with the word "cause" sit on the first step. Pearl separated the steps by the
question that is asked:

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
chance of admission, looking only at the admitted makes them look associated. In the simulation of Chapter 6,
talent and likeability have correlation $+0.001$ in the population and $-0.679$ among those above
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
other income and other access to doctors, and a back door remained open, until the draw closed it. The case also
teaches that confounding is not the whole story. Later reanalyses of the same cohort (Hernán and colleagues, 2008),
designed to imitate the trial (women who start the hormone, compared from the day they start, at the same time since
menopause), came close to the trial's answer. Much of the gap came from comparing women who had used the hormone for
years with women who never did: a problem of design and timing, which no adjustment for income or habits repairs.

**Potential outcomes.** Each person has two possible outcomes, $Y(1)$ with the treatment and
$Y(0)$ without it, and only one is observed. The comparison between the treated and the untreated gives:

$$\mathbb{E}[Y\mid D{=}1]-\mathbb{E}[Y\mid D{=}0] = \underbrace{\mathbb{E}[Y(1)-Y(0)\mid D{=}1]}_{\text{effect on the treated}} + \underbrace{\mathbb{E}[Y(0)\mid D{=}1]-\mathbb{E}[Y(0)\mid D{=}0]}_{\text{selection bias}}$$

When the treatment is **randomised**, the second term is zero by construction, because whoever was drawn
for the treatment has the same expected $Y(0)$ as whoever was not. Chapter 13 finds this in the
LaLonde data, where selection bias flips the sign of the effect. The raw comparison gives $-8,498$
dollars of income in 1978 and the experiment gives $+1,794$; the difference, $-10,292$, is the middle term.

**The naive difference is not only effect.** The decomposition gives a name to the third element, and the data
alone do not repair it. Nothing in the observed data says how much it is worth, because one of the two
potential outcomes of each person is missing by construction. The draw zeroes that term by design. Without a draw,
the term can still be removed, but only under a premise that the data cannot test: the back door of the formula
above (every confounder measured), an instrument that moves the treatment and nothing else, a threshold that
assigns the treatment, or two groups that would have moved in parallel. Each strategy trades the draw for a
declared assumption.

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
field to field. Twelve real domains, each with environments that intervened on the predictors, show the whole
range of outcomes:

| Reading | What happened |
| --- | --- |
| 6 of the 12 domains | ICP rejected every set: the environments act directly on $Y$, or the linear model was wrong for them |
| 4 domains | it accepted so much that $\hat S$ came out empty: the environments move $X$ only a little |
| 1 domain | $\hat S$ not empty: citizenship and employment entered, skin colour did not |

**The effect transports better than the set.** The same twelve domains taught a second lesson. When ICP
rejects every set, the model fitted in the training environments predicted well an environment never
seen in 9 domains, with skill from 0.04 to 0.88. Finding the exact set of causes is more
demanding than predicting under change. The single non-empty set, in police data, is the one to read with most care:
ICP only promises a subset of the direct causes, under premises that observational records of who was released do
not guarantee, and a variable left out of $\hat S$ is not shown to have no effect.

**Why this matters for deciding.** Invariance is also the answer to Lucas' critique
(Chapter 11). Only what is invariant across regimes deserves use when the regime changes. In a physical
experiment whose true causal structure was known and declared beforehand, ICP recovered the true parents of the electrical current
in a physical chamber, $\hat S = \{red, green, blue\}$, and left out light sensors strongly
correlated with them, because they were children of the LEDs.

---

### Step 3 — Hands on

Three tasks: redo Charig's count with the counts, classify questions into the three steps and
build a minimal ICP that chooses the cause and rejects the false alarm. Run it with
`cargo run --release --bin labs -- --chapter 9`.

```text
== Chapter 9 :: Simpson's paradox in kidney-stone surgery ==
small: open 81/87 = 0.9310  new 234/270 = 0.8667
large: open 192/263 = 0.7300  new 55/80 = 0.6875
raw:   open 273/350 = 0.7800  new 289/350 = 0.8257
weight of the strata in the whole population: small 0.5100, large 0.4900
weighted by the whole population: open 0.8325  new 0.7789  -> the advantage flips back
difference in points: small stones 6.44, large stones 4.25, standardised 5.37 (a weighted mean of the two)
fraction of hard cases in the open surgery = 0.7514, in the new one = 0.2286

== Chapter 9 :: the three steps of Pearl ==
  [  1. association] neighbourhoods with more police have more crime
  [ 2. intervention] if we double policing, does crime fall?
  [3. counterfactual] would this robbery have happened with a patrol car on the corner?

== Chapter 9 :: a minimal ICP, three sites, three candidates ==
rows = 12000 (4000 per site), X -> Y with coefficient 2, X -> Z (child) with noise different per site, W pure noise
naive correlation with Y: corr(X,Y) = 0.9314  corr(Z,Y) = 0.7208  corr(W,Y) = 0.0055
naive slope (all sites together): X = 1.9885  Z = 0.6003  W = 0.0152
  candidate {empty}  F = 3081.848  p = 0.000e0  intercepts only  rejected
  candidate {    x}  F =    0.633  p = 6.390e-1  common = 1.988  per site = [1.970, 1.996, 1.990]  ACCEPTED
  candidate {    z}  F = 1079.841  p = 0.000e0  common = 0.600  per site = [0.928, 0.496, 0.299]  rejected
  candidate {    w}  F = 1541.630  p = 0.000e0  common = 0.015  per site = [0.056, -0.030, 0.008]  rejected
  candidate {  x+z}  F =    0.579  p = 7.471e-1  common = 1.994  per site = [1.918, 1.998, 1.999]  ACCEPTED
  candidate {  x+w}  F =    0.640  p = 6.984e-1  common = 1.988  per site = [1.970, 1.996, 1.990]  ACCEPTED
  candidate {  z+w}  F =  719.620  p = 0.000e0  common = 0.600  per site = [0.928, 0.496, 0.299]  rejected
  candidate {x+z+w}  F =    0.597  p = 7.815e-1  common = 1.994  per site = [1.919, 1.998, 1.999]  ACCEPTED
accepted sets: 4 of 8 -> intersection S_hat = {x}

== Chapter 9 :: 200 repetitions, stable mechanism: Y = 2X + noise at every site
mean naive slope (all sites together): X = 2.0004  Z = 0.6015
  {empty}  mean F =  3191.16  rejection rate = 1.000
  {    x}  mean F =     1.06  rejection rate = 0.065
  {    z}  mean F =  1111.23  rejection rate = 1.000
  {    w}  mean F =  1596.08  rejection rate = 1.000
  {  x+z}  mean F =     1.02  rejection rate = 0.065
  {  x+w}  mean F =     1.05  rejection rate = 0.080
  {  z+w}  mean F =   741.13  rejection rate = 1.000
  {x+z+w}  mean F =     1.03  rejection rate = 0.055
  intersection = {x} in 0.960 of the rounds | contains z in 0.015 | contains w in 0.010 | empty in 0.025

== Chapter 9 :: 200 repetitions, unstable mechanism: the coefficient of X in Y changes from site to site
mean naive slope (all sites together): X = 2.0001  Z = 0.6013
  {empty}  mean F =  2921.60  rejection rate = 1.000
  {    x}  mean F =  6001.61  rejection rate = 1.000
  {    z}  mean F =   772.13  rejection rate = 1.000
  {    w}  mean F =  1461.37  rejection rate = 1.000
  {  x+z}  mean F =  4001.00  rejection rate = 1.000
  {  x+w}  mean F =  4001.05  rejection rate = 1.000
  {  z+w}  mean F =   515.10  rejection rate = 1.000
  {x+z+w}  mean F =  3000.74  rejection rate = 1.000
  intersection = {x} in 0.000 of the rounds | contains z in 0.000 | contains w in 0.000 | empty in 1.000
```

**What each line says.**

**(1) Charig's count closes.** Open surgery wins on the small ones (0.9310 against 0.8667) and on the
large ones (0.7300 against 0.6875), and loses in the raw total (0.7800 against 0.8257). Weighting the two
strata by the whole population, with weights 0.5100 and 0.4900, the open surgery gets 0.8325 and the other
0.7789. The advantage flips back, now to the side of the open surgery, by 5.37 points, between the 6.44 and the 4.25
of the strata. What
produced the paradox was the mix of cases. **75% of the patients of the open surgery had large stones,
against 23% in the other technique**.

**(2) Naive correlation cannot separate the cause from the symptom.** With 12,000 rows across three
sites, the outcome $Y$ was generated by $Y = 2X + \text{noise}$, the symptom $Z$ by $Z = 2X +
\text{site noise}$ and $W$ is pure noise. Looking at everything together, $\operatorname{corr}(X,Y) =
0.9314$ and $\operatorname{corr}(Z,Y) = 0.7208$. The cause and the symptom look like two explanatory
variables, and the naive regression gives slope **1.9885** for $X$ and **0.6003** for $Z$. If the
analyst stopped here, they would attribute to the alarm an effect of 0.6.

**(3) ICP separates the two.** The invariance test fits ONE line (one intercept and one slope) for the three sites
together and compares it with a line per site. Under the null of invariance, the site must not move the
intercept either, because an environment that shifts the outcome by itself acts directly on $Y$:

| Candidate | F | p | Common slope | Per site | Verdict |
| --- | --- | --- | --- | --- | --- |
| $\emptyset$ | 3081.848 | ~0 | — | — | rejected |
| $\{X\}$ | 0.633 | 0.639 | 1.988 | 1.970 · 1.996 · 1.990 | accepted |
| $\{Z\}$ | 1079.841 | ~0 | 0.600 | 0.928 · 0.496 · 0.299 | rejected |
| $\{W\}$ | 1541.630 | ~0 | 0.015 | 0.056 · −0.030 · 0.008 | rejected |

The slope of $X$ is practically constant across the three sites (1.970, 1.996, 1.990); that of $Z$ goes from
0.928 to 0.299, almost tripling from one site to the other. The mechanism that links the cause to the outcome
did not change; what links the symptom to the outcome changed in every place. The empty set and the noise $W$ fall
for another reason: the sites shift the cause $X$, so the mean of $Y$ moves from site to site, and only a set that
contains $X$ can explain that move. Four sets pass, all of them containing $X$, and their intersection is
$\hat S = \{X\}$.

**(4) What the test gets right and how often it errs.** In 200 repetitions with the stable mechanism, the empty set,
$\{Z\}$ and $\{W\}$ are rejected in **100%** of the rounds, and $\{X\}$ only in **6.5%**, close to the level of 5%.
The intersection returns exactly $\{X\}$ in **96%** of the rounds (0.960). It contains the child $Z$ in 1.5% and the
noise $W$ in 1.0%, which together stay below the 5% that the guarantee of ICP allows, and it comes out empty in
2.5%. **Abstaining is a result, not a defect of the instrument**: when the true cause is rejected by chance, the
method says nothing rather than something false.

**(5) When the mechanism is unstable, the cause falls too.** In the second battery, the coefficient of
$X$ in $Y$ changes from site to site (0.5, 2 and 3.5). The pooled slope of $X$ stays near 2 (2.0001), the average
of the three, and hides that no site has the slope 2 of the middle one except by coincidence. Every set is now
rejected in **100%** of the rounds and the intersection is empty in all of them. When the sites intervene in the
mechanism, no set is invariant, and the test says so, as it should.

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
5. *(Laboratory)* Make the noise of $Z$ equal across the three sites **and** remove the shift of $X$ between the
   sites. Show that $\{Z\}$, $\{W\}$ and the empty set become accepted and that $\hat S$ comes out empty. What does
   that say about ICP? Does it find the cause, or does it need environments that move the cause?
6. *(Concept)* Why can ICP return the empty set when it has too much data? Relate it to
   the sensitivity of the test to departures from the model.

**Summary.**

- Association, intervention and counterfactual are different questions; the graph makes the bridge, and the data
  alone do not.
- Simpson's paradox is born of a mix of cases. Open surgery wins in every stratum and
  loses in the raw total because 75% of its patients were the hard cases.
- Confounder, mediator and collider call for three opposite reactions; adjusting everything is not neutral.
- Naive difference = effect on the treated + selection bias; the draw zeroes the second term by design, and every
  other strategy zeroes it only under a declared, untestable premise.
- ICP separates the cause from the symptom through invariance across environments that move the cause, and abstaining
  is a legitimate result. In the laboratory it returned exactly the cause in 96% of the rounds.

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
simulation with 10,000 trajectories of 1,000 rounds, the final median of whoever stakes everything is
$1.322 \times 10^{-23}$ of the starting capital. Step 2 shows why the mean of that cohort still grows.

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
$$\mathbb{E}[M] > 1 \quad \text{and} \quad \mathbb{E}[\ln M] < 0,$$
the crowd grows rich and almost every trajectory decays. In the Peters coin:

$$\ln \mathbb{E}[M] \approx +0.049, \qquad \mathbb{E}[\ln M] = \tfrac12\ln 1.5 + \tfrac12\ln 0.6 \approx -0.053$$

Measured by simulation, it is $-0.052680$ per round, which gives $-5.268\%$ of time growth.
The difference between the two counts, $+5.00\%$ and $-5.27\%$, measures the distance between the line of a
spreadsheet and the life of whoever bets.

| Rounds | Trajectories above the starting point |
| --- | --- |
| 10 | 37.7% |
| 100 | 13.6% |
| 1,000 | 0.01% |

In 1,000 rounds the expectation is worth $1.55 \times 10^{21}$ and the median $10^{-23}$. The factors are
independent and identically distributed; **what is not ergodic is the product**. Even whoever simulates falls into the trap, because the
mean of $N$ trajectories follows the expectation only for a time that grows like $\ln N$. A one-line estimate is
$\ln N / (\ln \mathbb{E}[M] - \mathbb{E}[\ln M])$, the time at which the gap between the two growth rates has eaten
the head start that the luckiest of $N$ trajectories can give: about 23 rounds with 10 trajectories and 136 with a
million.

**How much to expose: Kelly's criterion (1956).** Instead of staking everything, one chooses the fraction $f$ of the
wealth:

$$f^* = \arg\max_f \; \mathbb{E}[\ln(1 + fR)]$$

In an even-odds bet with chance $p$, the optimal fraction is $f^* = 2p - 1$, that is, stake
20% with chance 0.6. Staking everything in that bet would take the expectation, in 100 rounds, to
$8.3 \times 10^7$ times the capital, with a chance of surviving of $0.6^{100}$, about $10^{-22}$.
In a bet that gains $b$ with chance $p$ and loses $a$ with chance $1-p$, maximising
$p\ln(1+fb) + (1-p)\ln(1-fa)$ gives

$$f^* = \frac{p}{a} - \frac{1-p}{b},$$

the edge $pb - (1-p)a$ divided by the product $ab$. For the Peters coin, $p = 0.5$, $b = 0.5$ and $a = 0.4$, so
$f^* = 1.25 - 1 = 0.25$: a quarter of the wealth, with a growth of $+0.006211$ per round. Twice that fraction,
$0.50$, has growth exactly zero, and above it the trajectory decays: at $f = 0.70$ the growth is $-0.014200$ per
round. Overbetting does not merely lower the growth; past twice the optimum it turns a favourable bet into a
losing one.

**Epistemic uncertainty is an argument for staking less.** Take the even-odds bet with chance 0.6 and estimate the
chance by the frequency of 25 rounds. Full Kelly on the estimate, $2\hat p - 1$, has a negative growth rate under the
true chance whenever $\hat p$ reaches 18 of 25, which happens in 15% of the estimates (the binomial count is exact).
Half Kelly keeps 75% of the growth of full Kelly with a quarter of the variance of the stake.

**Laplace smoothing.** The simplest estimator of a probability is the observed frequency
$k/n$. It has a fatal defect in rare events. With zero losses in 250 weeks, it says that the
probability is **zero**, and no leverage is too much for a null chance. The Laplace succession rule
gives the corrected value.

$$P(\text{success tomorrow} \mid s, n) = \frac{s+1}{n+2}$$

With 250 clean weeks, the chance of a loss is not zero. It is $1/252 = 0.003968$. The rule is not neutral,
because it comes from a uniform prior, but it prevents the absurd conclusion that the risk is over.

**Insurance and cooperation.** In Bernoulli's ship, taken up by Peters and Adamou, a shipowner with wealth
of 100,000 pays 1,800 to cover a loss of 34,000 that occurs with chance 5%. In expectation, he
loses 100 per month and the insurer gains 100. In time growth, the shipowner gains by insuring: his growth goes
from $-0.0208$ to $-0.0182$ per month. The insurer, which holds many independent contracts, turns the same premium
into positive growth once the contracts are many enough (Chapter 11 measures it). In the Peters coin, three players who split the wealth every round grow 1.6% per
round (the mean of the logarithm of the average of their three factors, over the eight outcomes), when each one
alone would decay. **The question that decides the case is who holds the trajectories.**

**When the mean ceases to exist.** In a Pareto distribution, with $P(X > x) = x^{-\alpha}$, the
moment of order $k$ exists if and only if $k < \alpha$. With $\alpha = 1.1$, the mean exists and is worth
11, but the mean of a million observations has median 8.39, falls below 11 in 95% of the samples, and
the usual 95% interval contains the true value in only 26% of them. The fraction of the mean that comes from
values above $m$ is $m^{1-\alpha}$; in the median, 27.55% of the true mean comes from values larger than
the largest the sample of one million saw.

**The three states of chance (Mandelbrot).** Mandelbrot classified chance by the way sums behave.

| State | Example of distribution | What happens to the sum |
| --- | --- | --- |
| Mild | Gaussian, exponential | No observation dominates; the mean converges quickly |
| Slow | Lognormal | Every moment exists, but convergence misleads in real samples |
| Wild | Pareto with $\alpha \le 2$ | One observation can dominate the sum; the variance, or the mean, does not exist |

The theorem of **Fisher, Tippett and Gnedenko** says that the normalised maximum of many draws can only
converge to three families, Gumbel, Fréchet or Weibull, and that is why the tail is estimated from the maximum,
not from the mean. A Gaussian fitted to daily returns puts the 1-in-100-year loss where the bulk of the data is,
and a heavy tail puts it far beyond: the distribution of the maxima is fitted to the extremes themselves.

Taleb called the first Mediocristan and the last Extremistan, and added the type of
consequence. The "fourth quadrant", where the tail is wild and the loss depends on the size of the event, is
where models fail most and cost most. The Hill estimator estimates the tail exponent from the
$k$ largest values, $\hat\alpha = k / \sum \ln(x_i / x_{(k)})$. The Danish fires, with
$\alpha$ estimated at 1.81 (interval from 1.41 to 2.56), are in that territory (Chapter 14).

---

### Step 3 — Hands on

Four tasks make up the laboratory. Measure the Peters coin with the optimal fraction and with the all-in
stake; measure 10,000 insurance sellers with and without Laplace; estimate the tail with Hill; and measure the drag of
volatility at $1\times$, $2\times$ and $3\times$. Run it with `cargo run --release --bin labs -- --chapter 10`.

```text
== Chapter 10 :: the Peters coin (heads +50%, tails -40%) ==
ensemble mean per round = 1.0500  (+5.00% per round)
time mean per round      = -0.052680  (-5.268% per round)
product of one head and one tail: 1.5 x 0.6 = 0.90
full Kelly (heads +50%, tails -40%): f* = p/0.4 - (1-p)/0.5 = 0.2500 of the wealth; even-odds bet with p = 0.6: 2p - 1 = 0.20
time growth per round: f* = +0.006211 | 2f* = +0.000000 | f = 0.70 -0.014200 | f = 1 -0.052680

all-in (f = 1.00)          above the start: 10 rounds 0.374  100 rounds 0.128  1000 rounds 0.00000
                           ruin (< 1e-6 of the capital) = 0.9971  final median = 1.322e-23  final mean = 2.725e-6
twice Kelly (f = 0.50)     above the start: 10 rounds 0.374  100 rounds 0.450  1000 rounds 0.49110
                           ruin (< 1e-6 of the capital) = 0.0244  final median = 1.000e0  final mean = 7.375e6
full Kelly (f = 0.25)      above the start: 10 rounds 0.618  100 rounds 0.684  1000 rounds 0.96120
                           ruin (< 1e-6 of the capital) = 0.0000  final median = 4.983e2  final mean = 1.127e5
half Kelly (f = 0.125)     above the start: 10 rounds 0.618  100 rounds 0.811  1000 rounds 0.99650
                           ruin (< 1e-6 of the capital) = 0.0000  final median = 1.062e2  final mean = 4.954e2

all-in: log10 of the final median = -22.9, log10 of the final mean = -5.6
part of the mean that comes from the richest trajectory (richest 1%): 1.000

== Chapter 10 :: the insurance seller (earns 1% per week, loses 95% with q = 0.004) ==
time growth of the all-in: -0.00207 per week (negative: the typical all-in trajectory decays)
Kelly fraction with the true q: 0.648 | with Laplace after 250 clean weeks: 0.652 | with the raw frequency 0/250: capped at 1
observed frequency + full Kelly  no loss = 0.368  all-in = 0.368  ruin in 1000 weeks = 0.0194  final median = 3.026e0  final mean = 1.984e2
Laplace + full Kelly             no loss = 0.368  all-in = 0.000  ruin in 1000 weeks = 0.0000  final median = 4.077e0  final mean = 2.238e1
Laplace + half Kelly             no loss = 0.368  all-in = 0.000  ruin in 1000 weeks = 0.0000  final median = 2.097e0  final mean = 3.789e0
theoretical probability of 250 weeks with no loss = 0.3671
Laplace with zero losses in 250 weeks: q_hat = 1/252 = 0.003968

== Chapter 10 :: the wild tail: Pareto with alpha = 1.1 ==
true mean (alpha/(alpha-1)) = 11.0
n =      1000: mean of the sample means =     7.76  median =    6.22  below 11 in 0.94  95% CI covers in 0.24
n =   1000000: mean of the sample means =     9.53  median =    8.39  below 11 in 0.95  95% CI covers in 0.26
share of each sample's sum that comes from its single largest value (n = 1 000 000): 0.088
in the median, 27.55% of the true mean comes from values above the maximum of a sample of one million (median maximum 397420)

Hill estimator with alpha = 1.1 and n = 200 000:
  k =    100  alpha_hat = 1.083  95% CI approx [0.870; 1.295]
  k =    500  alpha_hat = 1.063  95% CI approx [0.970; 1.157]
  k =   1000  alpha_hat = 1.100  95% CI approx [1.032; 1.168]
  k =   5000  alpha_hat = 1.101  95% CI approx [1.070; 1.131]
  k =  20000  alpha_hat = 1.095  95% CI approx [1.080; 1.111]
reading: a Gaussian model fitted to this tail underestimates the loss in every repetition

== Chapter 10 :: volatility drag at 1x, 2x and 3x ==
L = 1x: arithmetic mean = +0.000299/session, measured growth = +0.000237/session, drag = +0.000062/session (+0.0156/year), wealth in 100000 sessions = 1.903e10
L = 2x: arithmetic mean = +0.000597/session, measured growth = +0.000350/session, drag = +0.000247/session (+0.0623/year), wealth in 100000 sessions = 1.551e15
L = 3x: arithmetic mean = +0.000896/session, measured growth = +0.000339/session, drag = +0.000556/session (+0.1402/year), wealth in 100000 sessions = 5.325e14
horizon of 2000 sessions:
  L = 1x: measured growth = +0.000198/session, final wealth = 1.485e0
  L = 2x: measured growth = +0.000273/session, final wealth = 1.726e0
  L = 3x: measured growth = +0.000225/session, final wealth = 1.569e0
horizon of 10000 sessions:
  L = 1x: measured growth = +0.000313/session, final wealth = 2.276e1
  L = 2x: measured growth = +0.000501/session, final wealth = 1.496e2
  L = 3x: measured growth = +0.000565/session, final wealth = 2.835e2
the 1987 crash (-22.6% in one day) takes 67.8% of the capital at 3x, 22.6% at 1x; ruin starts at L = 4.4
with a -22.6% crash in one day and 40 days of -3% (20 thousand sessions):
  L = 1x: measured growth = +0.000142/session, final wealth = 1.720e1
  L = 2x: measured growth = +0.000155/session, final wealth = 2.222e1
  L = 3x: measured growth = +0.000033/session, final wealth = 1.944e0
```

**What each line says.**

**(1) The two means.** The ensemble mean of the Peters coin is $1.0500$ per round, or $+5.00\%$.
The time mean is $-0.052680$, or $-5.268\%$. The product of one head and one tail is
$1.5 \times 0.6 = 0.90$. The Kelly fraction is $0.25$ of the wealth; with it, the growth in
time is $+0.006211$ per round, positive for the first time. At twice that fraction it is $+0.000000$.

**(2) The cohort of 10,000 trajectories.** Staking everything, the wealth falls to $< 10^{-6}$ of the capital in
$99.71\%$ of the trajectories, the final median is $1.322 \times 10^{-23}$ and the $\log_{10}$ of the final
mean is $-5.6$. **The richest trajectory alone accounts for 100% of the cohort mean**, that is, the richest 1%
explains everything the mean has of wealth. With **twice Kelly** $(f = 0.50)$ the growth is zero: the final median
is $1.000$, just under half of the trajectories end strictly above the start (0.49110; the rest end at it or below, because 1.25 and 0.8 multiply to exactly 1 and every equal number of heads and tails returns to the start), ruin touches 0.0244 of them, and the mean
still explodes to $7.375 \times 10^6$. With **full Kelly** $(f = 0.25)$ ruin is zero, a fraction 0.96120 of the trajectories ends
above the start and the final median is $4.983 \times 10^2$; with **half Kelly** $(f = 0.125)$ the median is
$1.062 \times 10^2$ and 0.99650 end above the start. The contrast is the point of the chapter. **Twice the optimal
fraction is no longer optimal for anything**: it keeps the exploding mean and gives the typical trajectory nothing;
half the optimum gives up most of the median to buy safety.
**(3) The insurance seller.** He earns 1% per week and, with chance 0.004, loses 95% of what is staked. Staking
everything, his growth is $-0.00207$ per week: the typical all-in trajectory decays. In 10,000 histories of 250 weeks,
**36.8%** see no loss at all, close to the theoretical probability $0.996^{250} = 0.3671$, and those are the ones who
stake everything ($f = 1$), because the raw frequency $0/250$ says there is no risk. Kelly with the true chance
stakes 0.648; Laplace's rule estimates $1/252 = 0.003968$ and stakes 0.652, almost the same. The difference shows
in the **tail**: whoever read the raw frequency is ruined in 1000 weeks in $1.94\%$ of the histories (0.0194),
whoever used Laplace in none, and the median of the Laplace investors ($4.077$) beats that of the all-in ones
($3.026$).

**(4) The wild tail.** With a Pareto of $\alpha = 1.1$, whose true mean is 11, the sample mean
of a million observations has median 8.39 and falls below 11 in 95% of the samples, and the 95%
interval covers the true value in 26% of them. The Hill estimator recovers the exponent. With $k = 1,000$
it gives $\hat\alpha = 1.100$, with approximate interval $[1.032; 1.168]$; with $k = 20,000$ it gives
$1.095$ with $[1.080; 1.111]$. A Gaussian model fitted to that tail underestimates the loss in
every repetition.

**(5) The volatility drag.** Calibrating a Gaussian model by two numbers measured on the Dow Jones index from 1885
to 2023 (drag of 0.14 per year at $3\times$ and growth of 0.051 per year at $1\times$, with 252 sessions per year),
in 100,000 sessions the measured growth per session is $+0.000237$ at $1\times$, $+0.000350$ at $2\times$ and
$+0.000339$ at $3\times$: $2\times$ beats $3\times$, as in the real index. The measured drag rises from
$0.0156$ per year at $1\times$ to $0.0623$ at $2\times$ and $0.1402$ at $3\times$, in the proportion 1 to 4 to 9:
the drag **grows with the square of the leverage**, which is the signature of $\mu - \tfrac12 \sigma^2 L^2$, and
the three values reproduce the drags of the real index of Chapter 13. The 1987 crash, alone, takes **67.8%** of the capital at $3\times$ and
22.6% at $1\times$; ruin starts at $L = 4.4$. With a crash planted in the middle of 20,000 sessions,
the growth of $3\times$ falls to $+0.000033$ per session, far below $1\times$ and $2\times$. **A single day changes
the order**.

**When refining buys nothing.** The volatility drag and the ruin of the coin are measured per day, per round,
per session: at the grain at which the trajectory is lived. The question of resolution from Chapter 5 has a
limiting case that is worth seeing once. When the world does not oscillate, asking each day gives the same
answer as asking the average day:

```text
demand 8 or 8 (no oscillation): mean capacity 10.0 | aggregate wait 18.52 min, fine wait 18.52 min, loss 0.00 min
   above 30 min: aggregated 0.2209, fine 0.2209
```

The loss of resolution is exactly zero. A finer question is worth its cost only when the world varies at the
grain of that question; the drag is large in this chapter because daily returns vary, and the queue loss is
zero here because demand does not.

**Exercises.**

1. *(Count)* In the Peters coin, compute $\mathbb{E}[\ln M]$ by hand and explain why the median falls
   even with the expectation growing. Check it against the measured $-0.052680$.
2. *(Count)* For an even-odds bet with chance $p$, show that $f^* = 2p - 1$. Then
   derive $f^* = p/a - (1-p)/b$ for a bet that gains $b$ or loses $a$, and check the 0.25 of the Peters coin.
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
- Kelly for the Peters coin is a quarter of the wealth; twice that has zero growth, and more than that decays.
- The volatility drag grows with the square of the leverage. Measured per session, 0.000062 at
  $1\times$ and 0.000556 at $3\times$.

## Chapter 11. The price of error, Goodhart's law and who pays the bill

**Objectives.** By the end of this chapter you should know how to decide by the cost/loss threshold, compute the
expected value of perfect information, use the robust criteria when there is no probability worth
trusting, recognise a loop in which the forecast changes the forecast and declare who pays for the error.

---

### Step 1 — The real world and the paradox

**The shipowner who asks whether insurance is worth it.** A shipowner has 100,000 in the account. The insurance
contract costs 1,800 per month and covers a loss of 34,000 that happens with chance 5%. Is it worth it?

- **In expectation**, the shipowner pays 1,800 and receives, on average, $0.05 \times 34,000 = 1,700$. He loses
  100 per month. The insurer earns 100 per month. Whoever looks only at that account concludes that insurance is expensive.
- **In time**, each month multiplies wealth by a factor. Without insurance, in 95% of the months it is
  multiplied by 1.0 and in 5% by 0.66. With insurance, the multiplication is 0.982 every month, without
  exception. The two accounts give different numbers.

The criterion that settles the impasse is a one-line rule: **pay to protect yourself when the chance
of the event is greater than the cost divided by the loss**, that is, $p > C/L$. Here $C/L = 1,800/34,000 =
5.294\%$. The chance is 5%, therefore below the threshold by 0.294 percentage point, and the rule
says **not** to insure. It is one of the cases in which the answer of expectation and the answer of time do not
coincide, and the chapter shows both.

**In every 100 weeks.** The insurance salesman of the previous chapter has the opposite situation. He
earns 1% per week and loses 95% of what he stakes with chance 0.4%. In 100 weeks, 67 end with
profit; the danger lives in the other 33. A history of 250 weeks shows no loss at all in
$0.996^{250} = 37\%$ of the cases, and whoever estimates the chance from the observed frequency concludes that it is
zero and leverages. It is the **rare event that does not appear in the sample**, and Chapter 10 has already measured the cost.

**The forecast that changes what it forecasts.** A published forecast is not a thermometer, because people
react to it. A traffic app that sends every user to yesterday's fastest route
moves the traffic jam to the route it recommended. Once enough drivers follow the app, those who use it take
longer than those who do not. A forecaster who splits the users so as to equalise the times ends with everybody on
the same time, and its forecast is close to right because it built the world it forecasts. **The metric became a target and stopped
measuring.** That is Goodhart's law. The second forecaster did not escape the loop: he *designed* it and
stabilised it.

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

The value of any real test (EVSI) lies between zero and it. **Bits are not value.** In one example, a disease has prevalence 0.3; treating costs 1 and leaving a sick person untreated costs 10,
so that one treats when the chance passes 0.10. A sensitive test (sensitivity 0.90, specificity 0.70) carries
0.241 bit about the state and is worth 0.2200, against a ceiling (the EVPI) of 0.70. A specific test (0.30, 0.95)
still carries 0.079 bit and is worth exactly zero, because even its negative result leaves the chance of disease at
0.24, above the threshold, and does not change the action. The value of a test is computed branch by branch: after
each result the decider picks one act for that result, and the test is worth only what that choice gains.

**Where the expected cost is born.** An expected cost is a sum of probability times consequence, term by term, and
the terms are worth writing out, because frequency and severity are different axes. A failure that happens once in a
hundred cases and costs a hundred times more than the common nuisance contributes as much as the nuisance on every
case; a list of events sorted by frequency then hides the event that carries the risk. Two systems can also share the
same expected cost and have nothing else in common: one loses a little every week, the other loses everything once.
The expectation is neutral to how the loss is spread, which is why a promise about the bad cases is stated as a tail:
the conditional value at risk (Rockafellar and Uryasev, 2000) is the mean of the worst fraction of outcomes, and it
moves when the tail moves while the expectation barely notices. Rarity does not neutralise a grave consequence;
small times large is not necessarily small.

**Not all information is welcome.** Good (1967) proved that free information never makes the decision
of someone with preferences over states of the world worse. But whoever has preferences over future
observations may pay not to know. Among people at risk of Huntington's disease, only a minority
take the genetic test before the symptoms, 17.4% in the United Kingdom between 1994 and 2014 (Baig et al., 2016).

**Criteria that dispense with probabilities.** The threshold $C/L$ and the EVPI presuppose a distribution over the
states of the world. In the uncertainty corner of the Stirling matrix (Chapter 4), it does not exist in a
trustworthy way, and decision theory offers robust criteria. Whoever uses one must declare which, and why:

| Criterion | Rule | Posture |
| --- | --- | --- |
| Wald (maximin) | Choose the action whose worst case is the least bad | Maximum pessimism |
| Savage (minimum regret) | Choose the action whose largest regret is the smallest | Avoiding "if I had known" |
| Laplace | Treat the states as equally likely | Ignorance as uniform |
| Hurwicz | Weight the worst and the best case by an optimism index | Declared middle ground |

**The dikes of Holland.** After the 1953 flood, the Delta Commission adopted for the central Netherlands
a standard of 1 in 10,000 years, and van Dantzig (1956) treated the height of the dike as a decision
problem, balancing the cost of the works and the expected damage. The laboratory redoes the problem with a
model. The annual maxima follow a GEV of unknown shape $\xi$, between $-0.1$ and $0.2$, and the
annual cost of a height $h$ is the works, $h - 2$, plus the expected damage, $3000 \times P(\text{maximum} > h)$.
Five decision rules give five different heights, and the criterion and the list of models have to be
declared before seeing the answer. Chernoff (1954) showed that Savage's criterion can change its
choice merely by adding an alternative that nobody would choose; here one adds a model.
**The robust criteria are not more "correct" than the expected one: they are the honest choice when the
probability the account depends on does not deserve trust.**

**The forecast that changes what it forecasts.** When a model is published, people react to it. Lucas
(1976) showed the phenomenon in economics. The parameters of a model estimated under a policy depend
on the rules the agents expect, and they change when the policy changes. A Phillips curve
estimated under one policy promises a lower unemployment than the new regime delivers, because the agents have
learnt the new rule. The answer of Perdomo and
colleagues (2020) is conceptual: **performatively stable** is the model that is optimal for the
distribution it induces.

**To learn or to deform?** An agent that minimises the variational free energy can lower surprise in
three ways: changing the belief, changing the world or ceasing to look.

$$F[q] = D\big(q \,\|\, p(s \mid o)\big) - \ln p(o) \;\ge\; -\ln p(o)$$

A thermostat that "believes" in 22 °C in a room at 18 °C reaches an identical $F$ when it learns that the
room is cold and when it heats it to 22 °C. Only a counterfactual from outside separates the two cases, and for that
reason an honest audit runs the model with and without the action.

**A loop, measured.** A simulated call centre assumed that whoever abandons the queue calls back with
probability 0.6.
That parameter was measured in another real support desk, from 1999, with a quasi-experimental design. The queue
found on arrival, at an identical date and time, serves as an instrument for the abandonment. Under that
design, abandoning instead of being served increases by **0.145** the chance that that client calls back
within fifteen minutes, with a 90% interval from 0.132 to 0.157. The two numbers are not the same quantity, and the
comparison has to say so. The simulation declared a probability of calling back after abandoning, 0.6; the
measurement is a difference between abandoning and being served, inside a window of fifteen minutes. A client who
was served may also call back, and a call after the window is not counted, so the measured difference is not a
measure of the declared probability. What it does show is that the loop exists, with the right sign, and that the
order of magnitude of the declared value was not supported by the only real data available. The excess of arrivals that the measured loop generates in that support desk is 2.3%
of the arrivals, against the 29% bias that the declared loop produced in the simulation; the comparison is one of order of
magnitude, because that bias also depends on the simulated abandonment rate, and not only on the loop. And the
measured value is probably a lower bound, because a new call is only visible when it enters the
queue.

In the simulated operation, the control of the loop was made with a small experiment embedded in the operation. In 10% of the
intervals, a draw multiplied the staffing level by 0.7 or 1.3. As the draw is independent of
the demand, the regression of the following arrivals on the drawn deficit identifies the effect of the action. The
naive forecaster, trained on the observed arrivals, **overestimates the exogenous demand by 29%** and even
so hits the observed values with 1% error. The target metric approves a forecaster that is wrong about
the world. With the instrument, $\hat r$ comes out at 0.56 (90% interval from 0.51 to 0.61) and the error of the
corrected forecast falls to **1.7%**, inside the limit of 3%.

**Who pays for the error.** When the base rates differ between groups, a single threshold produces different
error rates. The identity of Chouldechova (2017) shows why:

$$\text{FPR}=\frac{p}{1-p}\cdot\frac{1-\text{PPV}}{\text{PPV}}\cdot(1-\text{FNR})$$

The formula is an identity of the confusion matrix: it holds for any predictor, so it cannot fail on a single
group. Its force is comparative. With predictive value (PPV) and false negatives equal between groups, different
prevalences $p$ force different false positive rates. **Equal predictive value, equal false positive rates and equal false
negative rates by group are incompatible requirements when the prevalences differ and the predictor is not perfect.** Equal
predictive value is one form of calibration; calibration within each group (Kleinberg, Mullainathan and Raghavan, 2016)
is a neighbouring one and meets the same wall, and equal approval rates are yet another demand, with its own cost. "I want
fairness" is not yet a decision. One has to declare which equality was chosen, what it costs, where
the error migrated to and who comes to pay for it.

**Signing is not listening.** A governance officer who signs a risk opinion stands on the side of whoever
decides to use the model. Whoever pays for the error stands on the other side. In three real streams of police
data, the 5,226 detainees of Toronto in 1997–2002, the 65,276 of 2020–2021 and the 86,113 traffic stops in
Rhode Island 2005–2015, all recorded with a perceived colour, the opinions were signed and none of the seven
affected groups was heard in any of the seven decisions. A signature answers the question "who authorised
this?"; it does not answer "who was asked?". Chapter 21 returns to this point.

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
trilemma table with the Chouldechova identity. Run it with `cargo run --release --bin labs -- --chapter 11`.

```text
== Chapter 11 :: the Bernoulli ship (C = 1,800, L = 34,000, p = 0.05) ==
threshold p > C/L: p* = 0.05294 (5.294%)  ->  p = 0.050 does not pass the threshold
in expectation: expected loss without insurance = 1700, insurance result = +100 per month
shipowner growth in time: 1 ship without insurance = -0.0208, 1 ship with insurance = -0.0182, 3 ships without insurance = -inf
insurer holding 50,000 of capital per contract (exact binomial account):
insurer with     1 contracts: growth in time = -0.018042 per month
insurer with    10 contracts: growth in time = +0.000854 per month
insurer with   100 contracts: growth in time = +0.001888 per month
insurer with  1000 contracts: growth in time = +0.001987 per month

== Chapter 11 :: the EVPI (treating costs 1, leaving a sick patient costs 10) ==
action threshold: p > 0.10  ->  with p = 0.3 the best act without information treat (-1.00)
gain of whoever knows the state before acting = -0.30
EVPI = +0.70
  test sensitive (0.90; 0.70): EVSI = +0.2200 | mutual information = 0.241 bit
  test specific  (0.30; 0.95): EVSI = +0.0000 | mutual information = 0.079 bit
  with prevalence 0.05: best act without information = do not treat, EVPI = +0.450
  with prevalence 0.30: best act without information = treat, EVPI = +0.700
  with prevalence 0.60: best act without information = treat, EVPI = +0.400

== Chapter 11 :: the van Dantzig dikes, four tail models ==
  model     xi = -0.1  xi = +0.0  xi = +0.1  xi = +0.2   optimal height
  T = 100          4.07       4.76       5.71       7.01
  T = 10000        5.84       8.27      12.78      21.49
  optimal height with xi = -0.1: 5.839 m  (cost 4.143)
  optimal height with xi = +0.0: 7.564 m  (cost 6.327)
  optimal height with xi = +0.1: 9.809 m  (cost 9.426)
  optimal height with xi = +0.2: 12.584 m  (cost 13.613)
  expected value with the Gumbel: 7.56 m
  mean of the models: 8.95 m
  Wald maximin: 12.58 m
  Savage minimax regret: 9.81 m (largest regret 3.67)
  precaution: 1 in 10 000 in the worst model = 21.49 m
  spread between precaution and the Gumbel expected value = 13.92 m
  EVPI of knowing the tail model (equal weights, cost units) = +1.919

== Chapter 11 :: the Chouldechova trilemma ==
group     base rate        FNR        FPR  false positives  false negatives        PPV
White         0.069      0.240      0.105              467               79     0.3491
Black         0.111      0.360      0.196              832              191     0.2896
observed cases: White 329 marked by the facts, Black 530

-- FNR matched downwards, to the FNR of White at 0.240 --
White         0.069      0.240      0.105              467               79
Black         0.111      0.240      0.309             1312              127
total false positives: 1299 -> 1778 (+479)
total false negatives: 270 -> 206 (-64)

-- FNR matched upwards, to the FNR of Black at 0.360 --
White         0.069      0.360      0.052              230              119
Black         0.111      0.360      0.196              832              191
total false positives: 1299 -> 1062 (-237)
total false negatives: 270 -> 309 (+40)

Chouldechova identity: FPR = p/(1-p) * (1-PPV)/PPV * (1-FNR)
group    observed FPR  FPR of identity     difference
White          0.1050           0.1050       -2.8e-17
Black          0.1960           0.1960       -2.8e-17
the trilemma: with PPV 0.3491 and FNR 0.240 in BOTH groups, the identity forces FPR 0.1050 for White and 0.1769 for Black
```

**What each line says.**

**(1) The Bernoulli ship.** The threshold is $C/L = 0.05294$, or 5.294%, and the chance of 5% stays **below**
it. The cost/loss rule says not to insure. In expectation, the loss without insurance is 1,700 and the
result of the insurance is $+100$ per month for the insurer. The growth in time reverses the reading for
the shipowner. With one ship, $-0.0208$ per month without insurance and $-0.0182$ with insurance, both negative,
and insurance **improves** the result in spite of expectation saying the opposite. With **three** ships, the
growth without insurance is $\mathbf{-\infty}$, because three simultaneous losses leave the wealth without
cover for the damage; ruin stops being a small probability and becomes an absorbing
state.

**(2) On the other side of the counter.** The insurer holds 50,000 of capital for each contract, and the account is
exact: the number of claims is binomial, and the growth is the mean of the logarithm over every possible number of
claims. With 1 contract the growth in time is $-0.018042$ per month; with 10, $+0.000854$; with 100, $+0.001888$; with
**1,000 contracts, $+0.001987$**, close to the expected margin of the premium over the capital. The premium does not
change in any of those cases; what changes is the number of independent contracts it holds, which shrinks the
relative variance of its result. **Whoever shares the risk charges the same and grows; whoever carries
the trajectory alone sinks.** It is the answer of Chapter 10, now with an insurance company in
place of a gambler.
**(3) The EVPI.** With prevalence 0.3, treating costs 1 and leaving a sick person untreated costs 10, the
action threshold is $p > 0.10$ and the best action without information is to treat, with gain $-1.00$. Whoever knows
the state before acting has $-0.30$, and the difference is the **EVPI $= +0.70$**. The sensitive test is worth
$0.2200$ and carries $0.241$ bit; the specific one carries $0.079$ bit and is worth $0.0000$, because no result of
it changes the act. Bits and value part ways. With prevalence 0.05 the EVPI falls to 0.450; with
0.30, it rises to 0.700; with 0.60, it falls again to 0.400. **The value of information is maximal when the
probability is close to the action threshold**, and that is where information changes the decision.

**(4) The dikes.** The four models already disagree at the **100-year** level (from 4.07 to 7.01 m) and disagree
far more at the **10,000-year** level (from 5.84 to 21.49 m). The tail is where the models separate. The five
rules give five heights: expected value with the Gumbel, **7.56 m**; mean of the models, **8.95 m**;
Savage minimax regret, **9.81 m**; Wald maximin, **12.58 m**; precaution of 1 in 10,000
in the worst model, **21.49 m**. The expected value of a single model chooses a cheap and fragile height;
precaution chooses the most expensive; Savage limits how much one can regret. Two different numbers measure what
the ignorance of the tail costs. The spread of heights between precaution and the Gumbel is 13.92 m, a distance in
metres. The EVPI of knowing which model is right, with the four models equally weighted, is $+1.919$ in units of
annual cost: what a decider who could learn the tail before building would save on average.

**(5) The trilemma table.** The two groups are White, with base rate 0.069, FNR 0.24 and FPR 0.105, and
Black, with base rate 0.111, FNR 0.36 and FPR 0.196. At the system threshold, there are 467 and 832 false
positives and 79 and 191 false negatives. Moving a threshold moves both errors, in opposite directions: in the
binormal model of the laboratory, lowering the FNR of a group lowers its threshold and raises its FPR. Equalising the
FNR **downwards**, to the level of White, lowers the threshold for Black: the false negatives fall from 270 to 206
and the false positives rise from 1,299 to 1778 ($+479$). Equalising **upwards**, to the level of Black, raises the
threshold for White: the false positives fall to 1,062 ($-237$) and the false negatives rise to 309 ($+40$).
**The choice of which equality to pursue is not neutral: each direction trades one error for the other, in
different magnitudes, and falls on different people.**

**(6) The identity closes, and its force is comparative.** The positive predictive value that the confusion matrix
delivers is 0.3491 (White) and 0.2896 (Black). Fed with those values, the identity of Chouldechova returns the
observed FPR of each group to the last decimal: it is algebra, and it holds for any predictor. The trilemma appears
when the two groups are asked to share PPV and FNR: with the PPV 0.3491 and the FNR 0.240 of White in both, the
identity forces an FPR of 0.1769 on Black against 0.1050 on White. With different base rates, equal predictive
value and equal miss rates cannot coexist with equal false-alarm rates.

**Exercises.**

1. *(Count)* In the Bernoulli ship, insurance costs 1,800 and covers a loss of 34,000 with chance of
   5%. By the cost/loss rule, should the shipowner insure? How does the growth in time answer?
   Explain the two answers in one sentence each.
2. *(Count)* Redo the EVPI with prevalence 0.10 (exactly the threshold). Does the value of perfect
   information increase or decrease? How does that guide the spending on tests?
3. *(Concept)* In the example of the two-route app, who errs more, the naive forecaster or the
   forecaster that equalises the times? In what sense is the second one "performatively stable"?
4. *(Count)* With the rates of the laboratory (White with 0.069, 0.24, 0.105), compute the PPV of the matrix
   and check that the identity of Chouldechova returns the observed FPR. Then fix the PPV and the FNR of White for
   Black, with base rate 0.111, and compute the FPR the identity forces.
5. *(Laboratory)* In the laboratory, change the base rate of the second group from 0.111 to 0.20 and redo the
   trilemma table. Which equality becomes more expensive, and why?
6. *(Concept)* No forecast may declare a probability of 0% or 100%. Which problem of the ship
   laboratory does that prohibition avoid?

**Summary.**

- One decides by the threshold $C/L$. With $C = 1,800$ and $L = 34,000$, the threshold is 5.294% and the chance of 5%
  stays below it. The growth in time can say the opposite, and the two numbers have to be
  declared.
- Information is worth as much as it moves the decision. The EVPI is the ceiling, and bits are not value.
- When the probability does not deserve trust, the criteria of Wald, Savage and precaution replace the
  expected value; the criterion and the list of models are declared choices, never deductions.
- Published forecasts change the world. A drawn intervention identifies the loop and brings the 29% bias down to
  1.7%; the only real measurement of the loop, of a related but different quantity, did not support the size assumed.
- Fairness requires choosing between incompatible equalities. Equalising the FNR downwards saves 64 misses and costs
  479 false alarms; upwards it saves 237 false alarms and costs 40 misses; who pays is declared before running.

# Part IV. The method on real data

## Chapter 12. The health post and the 2.65 million calls of the bank

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
| Always 8, every day | 9.1 min | 3.7 min | 26.1 min | 8 |
| Half of the days 5, half 11 | 19.3 min | 8.1 min | 55.8 min | 24 |

No mean changed. It is 8 in both cases. What changed was the world, and the wait is **curved** in demand. Whoever arrives on a
calm day waits little, but whoever arrives on a surge day waits a lot, and the gain of the calm day does not compensate the
damage of the full day. The surge day accounts for about 70 in every 100 patients (it is 110 patients against 50),
so that the mean per patient weights precisely the bad days.

**The median trap.** Read the second row of the table again. In the world that oscillates, **the typical patient
waits 8.1 minutes**, less than the 10 minutes that the claim promised as a mean. Whoever looks at the "typical patient"
sees the promise kept; the whole difference is in the tail, where 24 in every 100 patients pass half an hour of wait.

**Reinforcement only on surge days.** With two nurses on the
days of 11 patients and one on the days of 5, the mean wait falls to 1.9 minutes and fewer than 1 patient in every 100
passes 30 minutes. The slack does not need to be larger; it needs to be where the risk is.

**The hidden question is how many patients a margin is worth.** Every margin of error needs a unit. If the
post measures the mean wait and treats each patient as an independent observation, the margin stays around
±0.07 minute; if it treats each *day* as the unit, it stays at ±0.51 minute, seven times larger. Which of the two holds
is not a matter of taste. It is the world that decides. The test is direct. Take 4,000 days of the claim world, split them
at random into two halves and measure the difference of mean wait between them. If the patients were independent, that
difference should walk around 0.0473 minute. It walks at 0.2062, and the prediction by the *day* unit, 0.1928, comes
close. The patients of the same day share the same demand, and sharing is not being independent.

**The real case is the call centre of a bank.** The post was an assumption; the centre is observation. It is 2.65 million
real calls of a North American bank call centre, counted every 5 minutes, over 164 working days of
2003. The schedule is built on the 83 training days (March to June), with a peak of 234 attendants and 2,264
agent-hours per day, and applied to the 81 vault days (July to October): the vault is the stretch of data kept closed
until the method is fixed, and opened once (Chapter 18). The claim is the standard practice of the
industry: *staffing each half hour by the Erlang C formula on the mean rate of the history guarantees 82% of the calls
answered within 20 seconds, every day.*

The dataset showed this.

- **Demand is not Poisson.** Between days, the sd is about 2,900 calls, where the Poisson model would predict
  180; the ratio variance/mean is 273. Part of that is calendar (Monday is +15%), and the residual after the day
  of the week still has a coefficient of variation of 6%.
- **Jensen lives inside the formula itself.** Applied to the real rate of each day, the Erlang C of the claim gives 57% on
  average, and not 82%. The service level is concave in load.
- **The unit is the day.** Splitting the days at random, the halves null gives an sd of 0.0743 for the service level,
  where the binomial margin that treats every call as independent predicts 0.0006.
- **Severe test.** In 400 days simulated in the world that the claim describes, none fell below the target. In the real
  vault, replayed call by call, about three days in every ten did.
- **Out of time.** The conformal 90% interval for the total calls of the day, built on the mean of the day of the
  week, covered 86% of the vault days, and the drift between training and vault is mild (KS 0.19, p = 0.08).

The decision is also measured in the vault, against the real arrivals:

| Staffing rule | Agent-hours per day | In every 100 days, days below the target | Service level |
| --- | --- | --- | --- |
| The claim: mean profile | 2,264 | 31 (30.9) | 83.9% |
| Profile of the day of the week (zero cost) | 2,261 | 20 (19.8) | 84.4% |
| Profile of the day of the week + margin for the residual variation | 2,428 | 5 (4.9) | 91.6% |
| The claim + 12 attendants equally | 2,433 | 10 (9.9) | 90.3% |

The last two rules cost almost the same (about 7% more agent-hours) and put the slack in different
places. The one that puts the slack where the risk is brings the failure from about 31 down to about 5 days in every 100; the
one that spreads attendants equally brings it down only to about 10. The figures are those of the exact replay of the vault
(Step 3); an independent engine, with simulated arrivals, keeps the same order.

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
$(W(5)+W(11))/2 = 29.3$ minutes. The simulation gives 19.3. The difference teaches three things. First, a day of 10 hours
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
| 8 every day (the one of the claim) | 9.1 min | 8 |
| 7 or 9 | 10.1 min | 9 |
| uniform between 5 and 11 | 12.8 min | 14 |
| 5 or 11 | 19.3 min | 24 |
| 5 or 11, with reinforcement only on surge days | 1.9 min | fewer than 1 |

**The independent unit decides the margin.** The effective size of a sample is a property of the process, not of the
count (Chapter 6). Calls of the same day share the rate and the queue; treating them as independent observations
is **pseudoreplication**, and the margin comes out too small. The 2.65 million calls of the vault come from 81
independent days, and the margin is computed over the days; their effective size, in calls, lies between 81 and 2.65
million, much nearer the first than the second. The halves null model checks the choice, under its own premise that
the days are independent of one another. It
splits the days of a known world at random into two halves, measures the difference between them and says which of the two
predicted margins describes what was seen. The margin per day describes; the data refute the margin per patient.

**The severe test (Mayo).** A difference between two worlds is evidence only if the procedure would have accused the
difference had it not existed. The difference between the world of 8 and the world of 5 or 11 is 10.2 minutes, and it stays
49.4 sd from the null; in 2,000 random splits, the null never passed 0.664 minute. Both worlds are simulated, so the test
does not confirm anything about a real post; it confirms that the simulation is precise enough for its difference to
be read, which is what a severe test can say about a model.

**What was assumed in the real case, and how the size of the assumption was measured.** Neither the service time nor the
patience of the clients is in the dataset; the file brings counts of calls, nothing more. The service was assumed
exponential with mean 4 minutes, and the patience exponential with mean 3. A sensitivity analysis, declared before
running, turned the two assumptions into a grid of 4 × 6 points (service of 3, 4, 5 and 6 minutes against patience of 1, 2, 3,
5, 7 and 10 minutes) and rescaled the four rules point by point. That is why the rule of the profile of the day of the week with
margin wins in 20 of the 24 points, and in the other four (all with patience of 1 minute) the four rules tie
without any of them leaving more than 1 day below the target. **There is no turning point.** The corner of the grid is the worst case,
with 5 minutes of service and 10 of patience. Not even the best of the four rules holds the target, with 14.2% of the days below
it against 37.7% of the rule of the claim. In every 100 days, 38 fail with the schedule of the claim and 14 with the rule
that is right, which costs 7.2% more agent-hours. Patience governs the size of the effect; the service time
barely moves it (with patience fixed, the fraction of the winning rule walks at most 0.9 percentage point across the whole
range). What the grid does **not** measure is the family of distributions (both stay exponential, with coefficient
of variation 1) and the dependence between patience and service. A sensitivity grid answers only the questions
it was built to ask.

---

### Step 3 — Hands on

The laboratory rebuilds the health post with a fixed seed: one day of the post with Poisson arrivals spread over
the day, exponential service and the Lindley recursion for the wait, which is the rule "I wait for what is left
of the queue, or nothing, whichever is larger". Run it with `cargo run --release --bin labs -- --chapter 15`.

```text
== Chapter 12 :: the same mean, two waits ==
2000 days of 10 h per world, one attendant, mu = 12 per hour, seed 1909 (+ world index)
world (patients per hour)    patients    wait  median     p90  >30min   margin/day     margin/pat.
8 every day (the claim)        159716     9.1     3.7    26.1      8%         0.14            0.03
7 or 9                         159057    10.1     4.0    29.1      9%         0.20            0.04
uniform between 5 and 11       160475    12.8     5.0    37.1     14%         0.32            0.05
5 or 11                        158656    19.3     8.1    55.8     24%         0.51            0.07
5 or 11, reinforcement in the surge   163683     1.9     0.0     6.8      0%         0.04            0.01

halves null: 4000 days of demand 8, 2000 splits of 2000 days against 2000
measured sd of the differences = 0.2062 min
predicted by the day unit     = 0.1928 min
predicted by the patient unit = 0.0473 min  (319704 patients)
null: max |difference| in 2000 splits = 0.664 min

severe test: observed difference = 10.2 min, 49.4 sd of the null, null fraction >= observed = 0.0000
tail: 8 in 100 patients wait more than 30 min in world 8; 24 in 100 in world 5 or 11
```

Read it line by line. **9.1 against 19.3.** The mean demand is the same, 8 patients per hour, and the wait more than
doubles only because the world oscillates. **8% against 24%** and **26.1 against 55.8** at the 90th percentile. The mean separates the two
worlds, and the tail separates far more. **Reinforcement in the surge is 1.9 min and 0%**, and the slack where the risk is zeroes the tail.
**0.2062, 0.1928 and 0.0473.** The sd measured in the halves null is five times larger than the margin of the unit
"patient". Whoever measured the wait treating each patient as independent would declare a precision that the world does not
have. **10.2 min, 49.4 sd, 0.0000.** The difference between the worlds is too large to come from chance, and the
procedure could have come out wrong and did not.

Run with six different seeds, the wait of the oscillating world goes from 19.3 to 20.6 minutes. The mean wait per
patient in the oscillating world is a ratio of heavy-tailed sums, and its second decimal belongs to the seed; the
doubling belongs to the world.

**The real calls.** The bank's 2.65 million calls are read and measured with
`cargo run --release --bin labs -- --bank`.

```text
== Real H0 :: the bank call centre, in Rust ==
training 83 days (Mar-Jun) | vault 81 days (Jul-Oct) | 2649354 calls in the vault
roster of the mean profile: 234 agents at the peak, 2259 agent-hours per day
service level on the mean profile = 0.8193 | mean of the vault days = 0.5683 | days below the goal of 80% = 0.6543
dispersion between days: Var/mean = 246.7 in the vault, 272.6 in the training window, 261.1 over all | deviation between days = 2841 | Poisson would predict 181
halves null: per day = 0.0743 | per call = 0.0006
```

The industry formula, applied to the mean profile, promises 82% (0.8193). Applied to the real rate of each vault
day, the same formula gives 57% on average (0.5683), and 65% of the days fall below the goal (0.6543). Between days
the counts deviate by 2841 calls where a Poisson model would predict 181. The concave formula and the overdispersed
demand together are the whole story of Step 1, measured.

**The four rules, with abandonment.** The staffing rules are tested by simulating the queue of each vault day,
with clients who give up after an exponential patience. Run it with `cargo run --release --bin labs -- --queue`.

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

Two readings. **(1)** The agent-hours are those of the decision table: 2,264, 2,261, 2,428 and 2,433 per day.
**(2)** The order holds in both engines: R2, the profile of the weekday with a margin for the residual variation, is
the best; R3 spends the same and loses to R2, because it spreads the slack evenly; the claim, R0, is the worst. The
levels differ a little between the exact engine and the event engine, and the decision does not.

**The grain of the bank's question.** Three of the four rules are resolutions of one question: R0 asks of the
average day, R1 of each weekday, R2 of each day with its residual variation. R3 is the control, the same money as
R2 spread without refining the question. The health post shows the price of
the coarse question in its purest form:

```text
== R horizon :: resolution of the question ==
paired arms: same arrivals and same service bases; only the service rate changes
demand 5 or 11 (the station of Chapter 12): mean capacity 10.0 | aggregate wait 43.73 min, fine wait 18.20 min, loss 25.54 min
   above 30 min: aggregated 0.4488, fine 0.2081
```

With the same total capacity, staffing every day for the mean costs 25.54 minutes of wait per patient more than
staffing each day for its own demand. At the bank, the same refinement is worth about 31 failing days in 100 against
about 5.

**Exercises.**

1. *(Count)* Use $W(\lambda)=\lambda/(\mu(\mu-\lambda))$ with $\mu = 12$ to find the long-run wait with
   $\lambda = 8$, $\lambda = 5$ and $\lambda = 11$; then compute the mean of the last two and compare it with the 19.3
   minutes of the simulation. Why does the formula give more?
2. *(Concept)* Why does the mean per patient weight the surge days more? Compute, in every 100 patients, how many
   arrive on days of 11 and how many on days of 5.
3. *(Laboratory)* Run the laboratory with seed 1909 and confirm the wait of the world 5 or 11 and the
   margin per day. Then replace the second nurse of the surge days by one nurse working at twice the pace, 24 consultations per hour, as Chapter 5 does: why do two nurses sharing one line wait less (1.9 against 2.5 minutes)?
   Does the tail change size?
4. *(Count)* In every 100 patients of the claim world, 8 wait more than 30 minutes; in the real world, 24. How many
   more patients are they, if the post serves 8,000 patients per year?
5. *(Concept)* Write, in four sentences and in natural frequencies, the memo of the call-centre case for the board
   of the bank, with an exit clause (which new fact would make the decision be reviewed).
6. *(Concept)* Explain why "the Erlang C formula is not wrong, but the answer is of type III". Which question did
   the manager ask and which question did the formula answer?

**Summary.**

- The same mean demand can give 9 or 20 minutes of wait: the wait is convex in demand, and whoever looks only at the mean
  chooses a world that does not exist.
- The median deceives. In the world that oscillates, the typical patient waits exactly the number that the claim promised as a
  mean, and the whole difference stays in the tail.
- The margin is computed on the independent unit, which is the day, and not the patient; the halves null model arbitrates the
  choice and refutes the pseudoreplication.
- In 2.65 million real calls, the standard schedule of the industry fails in about 31 of every 100 days, and spending
  about 7% more agent-hours where the risk is brings that down to about 5.

## Chapter 13. The illusion of multiplying by 3 in the market and the employment programme

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

**The Monday that erased 68%.** On Monday, 19 October 1987, the index fell 22.61% in a single day. In every 100 dollars
of capital:

- without leverage, 77 dollars remain;
- with 2 times, 55 dollars remain;
- with 3 times, **32 dollars** remain, 68 dollars destroyed in one session;
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
independent**. Counting cohorts is not counting observations. A margin computed
over 1,134 would be about 14 times smaller than the honest one ($\sqrt{1134/5.8}$). It is the lesson of the independent
unit of Chapter 6, now in time.

**The Great Depression, in natural frequencies.** From the peak of 3 September 1929 to the trough of 8 July 1932,
every 100 dollars invested become **11 dollars** without leverage, and **2.5 cents** with 3 times. None of these numbers
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
0.1749 per year), the observed drags are 0.0153, 0.0617 and 0.1403, against 0.0153, 0.0612 and 0.1376 predicted by the
formula. The second-order account already catches the essential, and little is left for the higher moments of the distribution.
The approximation holds for small returns and no large jump; on a day of −22.6% it stops
describing what happened and only underestimates the damage.

**Non-ergodicity, in one sentence.** The ensemble mean of all investors is not the time mean of
any of them. The mean of 19 times at 3× is an ensemble mean, made of a handful of very
lucky starts and of many cohorts that barely doubled, and nobody lives the mean. When there is an **absorbing state** (the
ruin, from which one does not leave), the ensemble mean stops being a useful description of real life.

**Ruin in one day.** In a world of daily returns, the leverage that zeroes the wealth in a single session is

$$L_{\text{ruin}} = \frac{1}{\lvert \min_t r_t \rvert},$$

the inverse of the size of the worst daily fall, and in the training set it is worth 4.42. Above that, ruin stops being a matter of probability and becomes a date marked on the calendar.

**Kelly.** The leverage that maximises the time rate is $L^* = \arg\max_L g_L$. With the uncertainty about the
parameters of the series carried into a posterior, the 90% interval goes from **1.26 to 3.44**, with median
**2.35**; the empirical Kelly of the series is 2.14. Leveraging 3 times is inside the interval, but at its upper
end, and Chapter 10 showed what epistemic uncertainty says about the upper end: stake less.

**The severe test.** The claim world states that wealth grows 3 times the annual arithmetic sum, without drag. In it,
the fraction of the 20-year cohorts that end above the starting point has mean **0.9286** (with the 1% quantile at
0.6354). In the real index, the fraction is **0.6458**. Had the claim been true, a fraction as low as the real one
would have appeared in a fraction 0.0140 of the replicas. The procedure could have spared the claim and did not:
the refutation is severe.

**What the refutation does not reach.** Delimiting the target that fell is part of the result. The refuted claim is "yields 3
times the mean" and "always wins". "Leveraging never pays" survives the refutation. Without financing cost, 2× and 3× still
grow more than 1× in time (0.0719 and 0.0601 against 0.0515), and the vault from 2000 to 2023 even favoured the claim.
The forecast without drag had skill 0.44, against 0.40 of the model. Two absences in the dataset favour the claim and
are declared. The series is price only (no dividends, and that underestimates the total return by about 3% to 4% per
year in the twentieth century), and there is no financing cost of the leverage.

**Who pays for the error.** The 65.5% of the summed wealth of the cohorts come from the best 10% of starts; the worst entry
dates are all from 1929 (3 September, 1 July, 1 August). In every 100 cohorts of 20 years, 4 end
below the start without leverage, 22 with 2× and 36 with 3×. Whoever entered in the worst month faced, over the 20 years, the same kind of daily returns as
everyone else; what ruined the cohort was the order in which they came, with the worst sessions first, and a mean
says nothing about order.

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
[+2.6; +19.6]. The fit comes close to the experiment and still does not count as evidence. Run a treatment with no
effect at all through the same weighting, keeping who was treated and drawing each person's employment from a model
fitted on the controls: the weighted difference lands between −10.3 and +11.9 points in 95% of the replicas, centred
near +0.9. The weighted estimate of the real data, +9.0, sits inside that band (p = 0.08). The
procedure could not have told a real effect of that size from no effect, and that is why the hit does not count as
evidence (Mayo). (Until 2026-10-19 this paragraph said that the inert treatment "would have given +9.0"; +9.0 is the
observed estimate, and the inert one is centred near +0.9.) The
fit relies on about 103 effective controls, and 72% of the CPS is below the smallest score of the treated. In
practice, it compares the treated with a handful of CPS people who look like them; those people are
different in things that the model does not see.

**Where the numbers of the propensity fit come from.** They are not printed by the laboratory; they come from the
research record, computed by code that no longer runs beside the book (Appendix E says where it is kept). The recipe, so that a reader can redo it:
the propensity score is a logistic regression of being treated on the Dehejia–Wahba covariates (age and its square,
centred at 30 and scaled by 10; education and its square, centred at 11 and scaled by 3; black, Hispanic, married,
no degree; income of 1974 and of 1975 in tens of thousands, their squares and an indicator of zero income), with a
ridge of $10^{-3}$ outside the intercept, over the 185 treated and the 15,992 CPS controls; each control weighs
$e/(1-e)$; the effective number of controls is Kish's $(\sum w)^2 / \sum w^2$. The +9.6 [−0.6; +20.8] is the
posterior of the effect on the treated, standardised to their racial composition (156 of 185 black), from four
independent Beta posteriors, one per arm and race, with the weighted controls entering as Kish pseudo-counts and
Beta(1, 1) priors except for untreated black men, whose prior is a Beta fitted to the quartiles of a simulated
elicitation (0.433, 0.533, 0.633). The weighted point estimate is +9.0. The placebo removes the 1975 terms from the
score and uses employment in 1975 as the outcome. The Kelly interval of the market case, the skill in the vault and the
false negative rates below come from the same record, and their
recipes are longer than a paragraph; they stay cited, not reproduced beside the book.

**What the vault showed.** With the 260 randomised controls, the skill of the model is +0.18 (it predicted 0.655 of
employment, the observed was 0.646); the logistic regression stays at +0.09 and the raw counterfactual of the CPS, at −0.002. The
k-anonymity of the rows goes from 1 to 11 after the generalisation of quasi-identifiers.

**Fairness, measured.** The false negative rates differ by group. They are 0.72 for Hispanics, 0.50 for Blacks and 0.57
for the others. Equalising them captures 22 more unemployed people and costs 87 false positives. It is the trilemma of Chouldechova
measured in numbers. There is no way to get the three things right at once, and someone will pay for the choice (Chapter 11).

**Approval is not truth.** Both analyses of this chapter went through the whole cycle and passed the safeguard
panel that Part V builds (Chapters 16 and 17). Passing means that the cards declared and anchored every safeguard, not that the analyses were right, and
much less that the claims were true: the market claim was refuted, and
the programme's effect stood only under randomisation. The third real case of this part, the Danish fire losses
of Chapter 14, met a different end: the data rejected it.

---

### Step 3 — Hands on

The laboratory reads the three public datasets and measures all the quantities cited above: the time growth and the drag,
the 20-year cohorts with a monthly start, the 1987 shock, the Depression, the claim world by resampling of
years and the raw decomposition of the NSW against the CPS. Run it with
`cargo run --release --bin labs -- --chapter 16`.

```text
== Chapter 13 :: volatility drag and the selection bias ==
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

**Exercises.**

1. *(Count)* Use the second-order drag, $\tfrac{1}{2}L^2\sigma^2$, with $\sigma = 0.1749$ per year to
   predict the drag of 1×, 2× and 3× and compare with 0.0153, 0.0617 and 0.1403 measured. At which leverage does the drag
   start to exceed the arithmetic gain?
2. *(Count)* Which leverage zeroes the capital on the day of −22.61%? And on the worst day of 1929? Explain why "ruin"
   is an absorbing state and the effect of that on the ensemble mean.
3. *(Concept)* Explain the sentence "whoever holds for decades always wins" and distinguish what the refutation
   brings down from what it leaves standing.
4. *(Laboratory)* Run the laboratory and redo the table of cohorts with a 10-year horizon (change the horizon
   constant). What happens to the fraction of 3× cohorts above the start? And to the
   median?
5. *(Concept)* Why does the placebo with the employment of 1975 measure the selection bias? Write the answer using the
   decomposition of the raw difference.
6. *(Count)* With the measured levels of employment in 1978 (76% of the treated, 65% of the randomised controls), compute the
   effect in percentage points and explain why the raw comparison gives the opposite sign.

**Summary.**

- Leverage multiplies the arithmetic mean, but time charges the drag $\tfrac{1}{2}L^2\sigma^2$. The 3× promises
  0.20 per year and delivers 0.06.
- A single day can destroy 68% of the capital leveraged 3 times; above $1/\min|r|$ (4.42 here) ruin is
  a matter of calendar, not of probability.
- Ensemble mean and typical trajectory separate. The 19.1× of mean coexist with 2.10× of median, and about 35.7 in every 100
  3× cohorts end below the start.
- A refutation must declare the target: "yields 3 times" and "always wins" fall; "leveraging never pays" does not.
- Hitting the number is not enough. In LaLonde, the propensity fit gives +9.6 points, inside the band that a treatment
  with no effect produces through the same weighting; the 1975 placebo (−12.3 points) shows that selection is still
  there, and the raw comparison of 1978 carries a selection bias of 10,292 dollars per person.

## Chapter 14. The Danish fires and the art of failing one's own model

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

$$\frac{0+1}{36+2} = \frac{1}{38} \approx 0.0263.$$

In every 100 months, 2.6 months hit, never zero. "It never happened" is not "it cannot happen". It is "it has not
happened yet".

**The tail.** The Hill estimator, applied to the 1,714 losses of the training set, gives for the tail index
$\hat\alpha = 1.81$, with a 95% interval from 1.41 to 2.56 (bootstrap by monthly blocks). That matters because an
index below $2$ means **infinite variance** of the losses. The mean still exists, because the index is above 1, and the
running mean does converge, but slowly and by jumps: the usual margin, built on a variance, does not apply. A single
claim can be so large that it dominates all the others added together, and August 1989 is the case.

The reinsurer, however, does not sell the whole tail. Its layer pays at most 100 million per claim, so what it pays
is bounded, and its annual result has a finite variance whatever the tail of the losses. The heavy tail matters to the
layer through the **frequency** with which claims reach 50 million, not through an infinite variance of what the
layer pays. That frequency is what the rest of this chapter estimates.

**The ruin.** With the exposure that the claim would justify (selling a limit equivalent to 3 times the capital), a
claim that consumes the whole limit takes the capital to **1 − 3 = −2**, that is, to zero. It happened in August 1989:
the reinsurer is ruined, and ruin is an absorbing state, from which one does not leave. Whoever used the Laplace rule
on the clean window (a much smaller exposure) ends 1989-1990 with **19% of the capital**; whoever used the rate of the adjusted tail
(POT/GPD) keeps 78%. The Kelly of the posterior, at the optimal point, is $f^* = 0.46$.

**The art of failing one's own model.** The analysis took the claim to a vault it had not seen, and the result was uncomfortable for
everyone, because in the vault **no forecaster beat the simple historical frequency**. The Brier skill of the model
(which is the difference between predicting well and predicting the historical constant, Chapter 8) stayed at **−0.015**; that of the POT,
at −0.003; that of the Laplace rule, at −0.024; and that of the claim, at −0.052. With 24 months in the vault and 2
months hit, those skills are noise of the size of their differences (the lesson of Chapter 8 on small vaults): the
vault does not rank the forecasters, it only fails to show that any of them beats the constant. Besides, the frequency of claims
was growing **4.6% per year** (p = 2·10⁻⁶), and the world was not stationary. The elicited (simulated) prior dominated the
posterior.

The safeguard panel (Chapter 16) rejected the analysis, and the tests were not adjusted so that it would pass. One sentence
summarises the verdict: **the claim loses, but the model does not win the right to decide.** No decision was
issued, and the rejection is the published result.

**The clean window in the whole series.** The rule was tested on all the clean windows of the history. Whenever the
last 36 months had not touched the layer, the decision would have been to leverage at the ceiling. There were 5 such decisions, and the
**5 ended ruined within 24 months**. The five are not five independent trials: the clean windows overlap, and their
ruins fall on the same two hits of 1989 and 1990. The honest count is one or two episodes, not five, which is the
independent unit of Chapter 6 again.

---

### Step 2 — The rule of the game

**The wild tail.** A distribution has a heavy tail when the probability of extreme values falls like a
power, $P(X > x) \propto x^{-\alpha}$, and not like an exponential. The Hill estimator uses the $k$ largest
observations:

$$\hat\alpha = \left[\frac{1}{k}\sum_{i=1}^{k}\ln\frac{x_{(i)}}{x_{(k+1)}}\right]^{-1}.$$

The reading is direct. The more slowly the largest observations decrease, the smaller $\hat\alpha$. It holds when
the tail is really of power and there is a plateau in $k$ where the estimate does not change. It fails in small samples and
when the choice of $k$ decides the answer (here, $\hat\alpha = 1.96$ with $k = 25$, $1.81$ with $k = 50$ and
$1.41$ with $k = 100$). The interval by bootstrap of monthly blocks
(1.41 to 2.56) is the honest way of saying: "we know that the tail is heavy; we do not know how much". An alternative
fit of the tail (POT, with a generalised Pareto distribution above 10 M) gives $\xi = 0.41$ [0.02; 0.72],
that is, $\alpha \approx 2.46$, lighter than the Hill. The two methods disagree, and the disagreement is in the
result.

**Why $\alpha < 2$ changes everything.** The variance of a variable with a power tail is finite only if
$\alpha > 2$. With $\alpha = 1.81$, the variance of the ground-up loss does not exist, and any margin computed from an
estimated variance will be too small. For the layer, which caps each payment, the variance exists and the danger is
the frequency of the hits. The case repeats the lesson of Chapter 4. A
process can be outside the hypothesis space of the model, and the data do not warn with a pretty chart.

**The higher the moment, the louder one point.** A moment of order $k$ weighs each observation by the $k$-th power of
its distance from the centre, so one observation ten times farther out weighs a hundred times more in the variance
and ten thousand times more in the kurtosis. The higher moments are the first to become unstable as the tail grows
heavy, and with $\alpha$ below 4 the kurtosis of the population does not even exist, whatever the sample prints. That
is why this chapter reads the tail through quantiles, the months hit and the Hill index at several $k$, and not
through a sample kurtosis, and why the three Hill values above differ so much. It is also why an extreme observation
is never removed because it is extreme: in this series the month that decides the fate of the insurer is exactly the
one an automatic cleaning would throw away.

**The Laplace rule of succession.** After $s$ successes in $n$ trials, the probability of the next
success is

$$P(\text{success} \mid s, n) = \frac{s+1}{n+2}.$$

It comes from a uniform prior over the probability and treats each month as exchangeable with the others. It holds when the
months are exchangeable, without memory and without trend, and it fails when the world has drift. And it has. The frequency of
claims grows 4.6% per year, with a p-value of $2 \times 10^{-6}$. Laplace, therefore, delivers an **optimistic**
estimate, not a lower bound: with a uniform prior and a world that drifts upwards, the true rate is probably higher.
After 36 clean months, 1 in 38 per month is the least one can defend, never zero.

**How much a clean window is worth.** Here the account stops being about the frequency and becomes about the evidence.
Compare two hypotheses: "the risk is negligible" (1 month hit per century, $p = 1/1200$) and "the risk is the one of the
tail of the training set" ($p \approx 1/23$ per month). The probability of seeing 36 clean months under each hypothesis:

$$P(\text{clean window} \mid \text{negligible}) = \left(1 - \tfrac{1}{1200}\right)^{36} = 0.9704, \qquad
P(\text{clean window} \mid \text{dangerous}) = \left(1 - p\right)^{36} = 0.2058.$$

The likelihood ratio is 4.72, or **2.2 bits** of evidence. Two bits is very little to decide
to sell three times the capital. The severity criterion says how much would be needed. For the procedure "the layer
was not touched" to have 95% chance of accusing the danger, **69 clean months** would be necessary, almost six years.
The evidence is also measured the other way, as mutual information between the result of the window and the hypothesis
dangerous-against-negligible, **0.51 bits**, about half of the one bit that a window can carry between two equally
likely hypotheses: the window leaves the question open in either direction. And, for comparison, if the real world were the one of the vault
(2 months hit in 24, $p = 1/12$), that clean window would be worth 4.5 bits. Three clean years are reasonable evidence
against a monthly rate of 8% and ridiculous evidence against one of 4%.

**Brier skill.** The grade of a forecaster comes from a scoring rule, and the reference is the historical
constant:

$$\text{skill} = 1 - \frac{\text{BS}_{\text{forecaster}}}{\text{BS}_{\text{climatology}}}.$$

Positive skill means that the forecaster beats the constant; **negative, that it loses to the constant**. The model
(−0.015), the POT (−0.003) and the Laplace (−0.024) lost; the claim (−0.052) lost more. No forecaster won the
right to decide, and the record of the analysis says so. The recipe: each forecaster states one monthly chance for
the 24 months of the vault, two of which were hit; the climatology is the frequency of the training set, 5/108; the
claim's zero is held at 1%, the floor that the forecast diary of Chapter 18 imposes; Laplace is 1/38; the POT is the chance of the tail above;
the model is the mean of the Beta posterior of the monthly chance, with 5 hits in 108 months and a Beta prior fitted to
the quartiles of a simulated elicitation (0.011, 0.0187 and 0.0283 per month). The trend of the claims is a Poisson
regression of the monthly count of all claims above 1 M DKK in the training set on time in years, with a likelihood
ratio test against a constant rate. These come from the research record, made with code that no longer runs beside the
book (Appendix E says where it is kept).

**The same pattern elsewhere.** The insurance seller of Chapter 10 lives the same story. In 37% of the simulated
histories of 250 weeks not a single loss appears. "I never lost" is worth 1.3 bits of evidence against a negligible
world of one loss in 2,500 weeks (0.0004 per week): $0.9996^{250} / 0.996^{250} = 2.46$. A severe test would require
14 clean years. The story repeats with another asset, and the clean window remains the most seductive and
weakest argument there is.

**A declared limit of the design.** The retention of 50 million was chosen by looking at the whole series, and not before
seeing it, and that is why the vault **is not blind** for the analyst. An honest report adds a sensitivity table for
retentions from 20 to 50 million, and the design stays on the record as the analysis of a case chosen with the outcome in view, useful to
learn about the method and not to estimate the risk of the insurance.

**Calibrating is not understanding.** Every model is wrong by some structure it does not contain. Kennedy and O'Hagan
(2001) wrote the honest equation: what is observed is the simulator at the right parameter, **plus a discrepancy**,
plus noise. Leave the discrepancy out and the parameter does its work: it bends to absorb the structure the model
lacks, the fit stays good, and the parameter, the quantity with a physical meaning, is learnt wrong, with an
interval narrow enough to exclude the truth. Put the discrepancy in with no restriction and the opposite happens: the
data cannot tell the parameter from the part of the discrepancy that looks like it, and the parameter is not learnt at
all. Brynjarsdóttir and O'Hagan (2014) showed the way between: say what the discrepancy **may not** do, which is
knowledge about the model and not about the data. A simulator that is right at the origin has a discrepancy that is
zero there and flat there, and that sentence is enough to give the parameter back.

**Lakatos, again.** An adjustment after a failure can be **progressive** (it predicts something new that is confirmed) or
**degenerative** (it only protects the theory). This analysis had every reason to choose the degenerative path. It was enough to change the retention, to touch
the prior or to change the period of the vault. None of that was done. The numbers stayed as they were, the
analysis was rejected, and the failure deserves to be recorded with the same care as the two approvals of
Chapter 13.

---

### Step 3 — Hands on

The laboratory reads the real series, measures Hill on the tail of the training set and on a simulated tail, counts the months in which the layer
was hit and computes Laplace, the likelihood ratio and the bits of the clean window. Run it with
`cargo run --release --bin labs -- --chapter 17`.

```text
== Chapter 14 :: the tail, the clean window and the information ==
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
is not stable. **1.900 with sd 0.265**: in a simulated tail with $\alpha = 1.81$, the Hill estimator
has that uncertainty in samples of that size, the honest translation of the interval [1.41; 2.56]. The bootstrap
interval printed above, [1.37; 2.61], is drawn too: under twenty other seeds its upper bound ranges from 2.43 to 2.60,
and the reading does not move, since the interval contains 2 under every seed. **0.1815**: in every
100 windows of three years, 18 come out clean in the dangerous world. **0.02632**: the Laplace rule puts the estimate at 1 in 38, never zero.
**2.24 bits and 69 months**: the clean window is 4.7 times more likely in the negligible world, that is, 2.2 bits of
evidence, and almost six clean years for a severe test. **4.48 bits**: against the rate of the vault, that window
would be worth double.

**Failing a parameter, measured.** A simulator says $y = \theta x$; the world bends away from it, as
$y = 0.65\,x / (1 + x/20)$, with the same slope at the origin. Thirty measurements in the range from 0 to 4 learn
$\theta$ three ways: without discrepancy, with a discrepancy that is zero and flat at the origin, and with a
discrepancy free to do anything (`cargo run --release --bin labs -- --discrepancy`):

```text
== Calibrating is not understanding: a parameter learnt with and without the discrepancy ==
world y = 0.65 x / (1 + x / 20) | simulator y = theta x | 30 points on (0, 4], noise sd 0.05 | priors N(0, 1) | 1000 datasets, seed 2026
  model                               | theta mean | theta sd | 95% interval covers 0.65 | fit rmse in range | at x = 8: mean error, rms error
  without discrepancy                 |     0.5647 |   0.0039 |                     0.000 |            0.0677 |       +0.803, 0.804
  discrepancy with d(0) = d'(0) = 0   |     0.6444 |   0.0767 |                     0.953 |            0.0461 |       +0.236, 6.988
  discrepancy free at the origin      |     0.3232 |   0.7081 |                     1.000 |            0.0461 |       +0.101, 6.998
  the world at x = 8: 3.714; the noise sd: 0.05
```

**Without discrepancy, the fit hides the error.** The fit misses the data by 0.0677, little more than the noise of
0.05, and $\theta$ comes out at 0.5647 with a standard deviation of 0.0039: its interval never contains 0.65 in a
thousand datasets, and its prediction at twice the range is off by 0.803, every time by about the same amount (the
root mean square error, 0.804, is the bias). **With what the discrepancy may not do, the parameter comes back.** The
mean is 0.6444, the interval covers the truth in 0.953 of the datasets, and it is twenty times wider, the price of
admitting that the model is wrong. **With a discrepancy free to do anything, the parameter is lost.** Its interval
always covers 0.65, because it is almost as wide as the prior; and its mean, 0.3232, is the slope shared half and half
with the linear term of the discrepancy. **Outside the range, nobody knows.** The two fits with a discrepancy have a
small mean error at $x = 8$ and a root mean square error near 7: a fourth power learnt between 0 and 4 says almost
anything at 8, and only on average does it say the truth. The fit without discrepancy is wrong with confidence; the
fits with one are honest about not knowing. Neither extrapolates, and a small mean error is no evidence of a good
prediction until its spread is printed beside it.

**Exercises.**

1. *(Count)* With the Laplace rule, compute the monthly chance after 12, 36 and 69 clean months. How many clean months
   would be needed for the monthly chance to fall below 1 in 100?
2. *(Concept)* Explain why $\alpha = 1.81$ prevents computing an honest margin of error for the annual loss,
   and why the interval [1.41; 2.56] crosses the boundary at which the variance exists.
3. *(Count)* Using $P(\text{clean} \mid p) = (1-p)^{36}$, compute the likelihood ratio between
   $p = 1/1200$ and $p = 1/12$. In bits, and in words: what do those bits authorise one to conclude?
4. *(Laboratory)* Run the laboratory with (k = 50) and then change the window from 36 to 72 clean
   months: how many bits is the larger window worth? How many months would be needed for 95%?
5. *(Concept)* "The claim loses, but the model does not win the right to decide." Explain that sentence to a
   risk manager who only wants to know whether or not to sell the layer.
6. *(Laboratory)* Simulate 100 years of i.i.d. months with $p = 1/1200$ and count how many windows of 36 months come out
   clean. Compare with the real history of the Danish series and say what that does to the argument "negligible
   risk".
7. *(Concept)* An engineer calibrates the friction coefficient of a model of a braking car on tests at low speed,
   and the fit is excellent. Why may the coefficient still be wrong? What would you have to know about the model's
   error to learn the coefficient, and why would a fully flexible correction not help?

**Summary.**

- 36 clean months between 50 and 150 million kroner are not a safe world. With the rate of the training set, 18 in every 100
  windows of three years come out clean by chance.
- The tail is heavy ($\hat\alpha = 1.81$, interval 1.41 to 2.56). With $\alpha < 2$ the variance of the losses does not
  exist; the layer caps what it pays, so its danger is the frequency of the hits, and a single month (August 1989,
  152 million) decides the fate of the company.
- The clean window is worth 2.2 bits of evidence; the Laplace rule gives 1 chance in 38 per month; a severe test
  would need 69 clean months.
- In the vault, no forecaster beat the simple historical frequency: model −0.015, POT −0.003, Laplace −0.024,
  claim −0.052, and the claim loses without the model winning the right to decide.
- Failing the model that one wrote, without touching the tests so that it passes, is a result.
- A parameter calibrated without the discrepancy absorbs the model's error and is learnt wrong with a narrow interval; a
  discrepancy free to do anything loses it; what the discrepancy may not do is the knowledge that gives it back.

# Part V. The auditable machine, built and put to work

## Chapter 15. The map of knowledge: the typed hypergraph and the five lenses

**Objectives.** By the end of this chapter you should know how to read the definition $\mathcal{G} = (V, E_{\mathrm{tree}},
E_{\mathrm{graph}}, H, \Phi)$, list the five mandatory fields of a node, recognise a pathology edge
and use the five lenses (information, time, tail, invariance and decision) as
questions that one asks of any problem.

---

### Step 1 — The real world and the paradox

**The conclusion that lives in the middle of the paragraph.** In a support desk, a report begins like this:
*"Considering the two-year history, the mean arrival rate of 312 calls per hour, the mean
service time of 6.2 minutes and the application of the Erlang C formula with 36 agents, the estimated
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
| $V$, nodes | Concepts, hypotheses, variables, data sets | `L1_demand`: call counts every 5 minutes |
| $E_{\mathrm{tree}}$, tree | What depends on what: deductive prerequisites | `L1_demanda` $\to$ `L2_erlang_c`, with the label "input of the model" |
| $E_{\mathrm{graph}}$, transversal | Mathematical bridges, causal edges $\operatorname{do}(x)$ and pathology edges | Jensen links the mean function to information and to ergodicity |
| $H$, hyperedges | The five lenses, which wrap whole groups of nodes | The time lens over `L1_demanda` and `L2_erlang_c`: "the demand has a calendar" |
| $\Phi$, invariants | The invariants that every claim fulfils to enter the graph | $\Phi_2$: the margin on the independent unit |

**Every claim becomes executable code.** Each node carries a one-line definition and a pointer to the artifact
that executes it with a fixed seed. Whoever asserts delivers the code that runs the assertion, and a claim that
nobody else can run cannot be checked by anybody else.

**The five mandatory fields of a node.** A node without any of them is refused on writing.

| Field | What it declares |
| --- | --- |
| `id` | Unique identifier |
| `layer` | $L_1$ World, $L_2$ Model or $L_3$ Agent |
| `uncertainty` | Aleatory, epistemic or knightian |
| `definition` | The mathematical definition, in one line |
| `artifact` | The pointer to the code that reproduces it with a fixed seed |

The node of the call-centre formula reads:

```yaml
- id: L2_erlang_c
  layer: L2
  uncertainty: epistemic (specification)
  definition: SL(c, λ) = 1 − C(c, λ·AHT) e^{−(c − λ·AHT) t / AHT}, stationary, no abandonment
  artifact: queue::service_level
```

Notice that the Erlang C formula is typed as an **epistemic uncertainty of specification**. It assumes
stationarity and absence of abandonment, and the World guarantees neither. Those two failures are **listed**, and
each can be tested against the data, so they are not Knightian in the sense of Chapter 4: Knightian is the mechanism
nobody wrote down. A field that called every violated assumption "Knightian" would empty the word, and the turkey
would share a label with a queue that only needs its stationarity checked. The card says, in one field, where the
formula can fail. The long sentence did not say.

**The pathology edges.** These are not dependencies but structural misalignments, the moment at
which the Agent applies a Model incompatible with the World.

| Pathology | What the edge marks |
| --- | --- |
| Fallacy of the mean | $\mathrm{SL}(\mathbb{E}[\Lambda])$ in place of $\mathbb{E}[\mathrm{SL}(\Lambda)]$ |
| Fallacy of the stable world | The Agent imposes a stable Model on a World that oscillates |
| Inspection paradox | Sampling by arrival over-represents the long intervals: the patient-weighted wait counts the busy days more |
| Transposition of the conditional | $P(E \mid I)$ read as $P(I \mid E)$ — the prosecutor's fallacy |
| Russell's turkey | The real process is outside the hypothesis space, and more data only increase confidence in the wrong place |
| Case for population | One looks for the cause of the case, not the cause of the frequency (Rose's sick individuals and sick populations) |

**The mathematical bridges.** The transversal edges link distant modules by an explicit relation,
navigable in both directions. Four of them recur across the whole theory.

| Bridge | Relation | What it links |
| --- | --- | --- |
| Jensen | $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$ for convex $f$ | The mean function to information and to ergodicity |
| Prior as regularisation | $-\ln P(\theta) \leftrightarrow \lambda\Omega(\theta)$ | Bayes to learning with a penalty and to the elicitation of expert priors |
| Information as growth | $\Delta W \le I(X; Y)$ | Mutual information to the Kelly growth gain |
| Noise as temperature (heuristic) | Minibatch noise $\leftrightarrow$ temperature $T$ | Stochastic mechanics to active inference and to learning |

The second bridge says that penalising large coefficients is equivalent to declaring a prior that finds them
unlikely. Whoever regularises is already computing a maximum a posteriori, without saying which prior was used. The third says
that a gambler with side information $Y$ about the outcome $X$ gains, in growth rate, at most
$I(X; Y)$ bits per round, and those bits become growth only for whoever bets by Kelly. The fourth says
that the noise of estimating the gradient on a minibatch acts as a temperature, and helps to escape shallow
minima. The fourth bridge does not have the rank of the other three. Jensen, the prior as penalty and the bound on
growth are theorems under stated conditions; the temperature of the minibatch is an analogy that holds only
approximately, because the noise of the gradient is neither isotropic nor independent of the point where it is
measured, and whether it helps generalisation is still disputed. It is kept as a question to test, not as a link to
lean on.

**The five lenses.** The hyperedges work as questions that one asks of any problem. Each lens
has a quantity that it measures and a critical habit, and it crosses the concepts without adding new objects.

| Lens | Question | What it measures | Critical habit |
| --- | --- | --- | --- |
| Information | How do probability and information flow and get lost? | Currents, mutual information | Does the account close? Reprocessing does not create information |
| Time | Does the mean across many coincide with the mean over time of a single one? | Mixing, relaxation and escape times; $\mathbb{E}[\ln M]$ | Which time is being spoken of: the process, the observer or the simulation? |
| Tail | Is the tail heavy to the point that the mean does not even exist? | Tail exponent, effective size, capacity of the model | Declare whether the theoretical mean exists |
| Invariance | Does the structure found survive when the environment changes? | Invariance (ICP), compressibility (MDL) | Rewrite the model in two forms and see what is conserved |
| Decision | Is the information worth its cost, and are the forecasts calibrated? | Loss, proper score, value of information | Charge every number the record of hits at the frequency it promised |

Two examples of how a lens becomes an account. Under the **information** lens, the data-processing inequality
says that, if $Z$ is obtained from $Y$, then $I(X; Z) \le I(X; Y)$. Three newspapers that publish the same research
are worth one. Under the **invariance** lens, two representations can give identical forecasts and still authorise
different conclusions under intervention, and only the conclusion that is conserved across environments serves outside them.

**The lenses, their fields and their labels.** The tools under each lens belong to a field with its own name, and the
name of the field is the one that leads to the literature. The laboratory and the code label the lenses with five
coined words, and those words appear in the printed outputs below; the table says which is which.

| Lens | Label in the outputs | What the field calls it | Where to read more |
| --- | --- | --- | --- |
| Information | Fluxonomy | information theory: mutual information, the data-processing inequality | Cover and Thomas, *Elements of Information Theory* |
| Time | Chrononomy | ergodicity, mixing and growth-optimal (Kelly) betting | Peters (2019); Kelly (1956) |
| Tail | Scalonomy | extreme-value theory, tail estimation, effective sample size | Embrechts, Klüppelberg and Mikosch (1997) |
| Invariance | Morphonomy | invariance and causal identification; minimum description length | Peters, Bühlmann and Meinshausen (2016) |
| Decision | Crisonomy | decision theory, value of information and proper scoring rules | Raiffa and Schlaifer; Gneiting and Raftery (2007) |

The labels add a habit of asking the five questions in order; they add no mathematics of their own.

Non-ergodicity is not a sixth lens. It is the answer "no" to the question of the **time** lens: the mean across many
and the mean over time of a single one come apart, as in the multiplicative coin of Chapter 10. The lens asks the
question of every process; most additive processes answer "yes", and there the mean across many is legitimate.

**How a new domain triggers the lenses.** A practical rule: Is it a financial, career or
insurance process? It triggers the **time** lens. Does it involve claims, incomes, virality or the size of
companies? It triggers the **tail** lens. Does it involve recommendation, regulation or a published forecast? It
triggers the **information** and **invariance** lenses, with simulation with and without the action.

**Where the data live.** The hypergraph keeps only the light topology: nodes, edges, YAML contracts and hashes.
Microdata and series stay in partitioned Parquet, queried with explicit column projection, partition
pruning and a rigid budget of bytes scanned. A rule of scale follows. A model that
requires a full scan per forecast does not scale, however good its test error.

**Counting nodes inflates easily.** A scale test ran the filter funnel in batch over **329** domains generated by rule from
a public catalogue (327 up to the end, at 3.8 seconds of processor per domain) and took the graph to
**26,634 nodes**, 28,619 tree edges, 26,795 bridges and 4,628 hyperedges. Only **6,494** of the nodes are
concept nodes (domain, layers, variables, environments, filters, decision), and only **554** come from curated packages;
the other 20,140 are ICP hypotheses, one per subset of predictors tested, which grow like $2^k$ and are not new
knowledge. The bottleneck is not in the machine but in semantic
curation, which does not scale by rule, in the supply of data sets with identifiable environments (9% of the catalogue
pass), in the bridges, which grow with the square of the number of domains, and in keeping everything in memory.

---

### Step 3 — Hands on

Three tasks: build a mini-hypergraph with three nodes and three lenses, prove that the validator bites in four
cases, and run the lenses with a measured number. The **tail** lens (Scalonomy) enters through the Hill estimator,
the **decision** lens (Crisonomy) through the expected value of perfect information (EVPI), and the **invariance** lens
(Morphonomy) through the test of
invariance across regimes. Run it with `cargo run --release --bin labs -- --chapter 18`.

**The recipe of the laboratory.** The series of the tail lens is 20,000 draws of a Pareto with $\alpha = 1.81$,
$(1 - u)^{-1/1.81}$ with $u$ uniform, on which Hill is computed at $k$ = 250, 500, 1,000 and 2,000; Hill is the formula
of Chapter 14. The EVPI of the decision lens is that of Chapter 11, $\mathbb{E}_\theta[\max_a U] - \max_a \mathbb{E}_\theta[U]$, on the
payoff table that the paragraph below states.

```text
== Chapter 15 :: the map of knowledge ==
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
Crisonomy: act (+10 in a good year, -4 in a bad one) or not act: best without information = 5.8000 | with perfect information = 7.0000 | EVPI = 1.2000
Crisonomy: only half the stake available: best without information = 2.9000 | with perfect information = 3.5000 | EVPI = 0.6000
Crisonomy: useless sensor (same payoff in both states): EVPI = 0.0000
Morphonomy: rule 1 -> marginal slope = 24.37 (treated fraction = 0.474) | by regime = [9.96, 10.09, 10.15] | invariant(tolerance 0.5) = true
Morphonomy: rule 1 with the allocation swapped -> marginal slope = -4.60 | by regime = [10.34, 9.92, 10.15] | invariant(tolerance 0.5) = true
Morphonomy: rule 2, middle regime with effect 40 -> marginal slope = 40.03 | by regime = [9.96, 40.09, 10.15] | invariant(tolerance 0.5) = false | step within the regime = 30.13
```

Four readings of the output, in the order in which they matter.

**The validator refuses.** The valid graph passes with zero problems. The four proofs show that the refusal is not
decorative: an edge to a node that does not exist, a deductive cycle (the "return" that turns the tree into a wheel),
a hyperedge with a single member, and an orphan node outside every lens. Proof 4 shows the structural validator
letting the orphan node through: the validation `[]` stays empty, while the coverage check points at
`L2_orphan`.
The two screens take care of distinct things; one takes care of the form, the other of coverage.

**Not every lens is triggered.** The mini-hypergraph triggers three lenses (time, tail and decision,
printed as Chrononomy, Scalonomy and Crisonomy) and leaves information and invariance without a hyperedge. The absence is not an error of the validator. The error
would be if some **node** stayed outside every lens, and none does. A lens without use is a question that
has not yet been asked.

**The tail lens measures the tail.** With seed 1909, 20,000 values of a Pareto with $\alpha = 1.81$ (the
exponent estimated for the Danish fire losses) and the cutoff at $k = 500$, the Hill estimator returns
**1.7901**. The tail is wild
($\hat\alpha < 2$), and for that reason the theoretical variance does not exist. At the edges of the cutoff the number moves, with
1.8444 at $k = 250$ and 1.8032 at $k = 2,000$, and the sensitivity is the lesson. The tail exponent is an
estimate, and not a constant of nature.

**The decision lens depends on which actions exist.** The perfect sensor says whether the year will be good or bad, with
probability 0.70 of good. Acting yields 10 in a good year and costs 4 in a bad one; not acting yields 0. Without the
sensor the best is to act, with an expected value of 5.80. With the sensor one acts only in the good years and keeps
out of the bad ones, which is worth 0.70 × 10 = 7.0000, so the sensor is worth **1.2000**: exactly what it saves in
the bad years, 0.30 × 4. If only half the stake can be placed (+5 or −2), the best without information is 2.9000,
with the sensor 3.5000, and the sensor is worth **0.6000**. If the payoff of acting is the same in both states, the sensor
can never change the decision and is worth 0.0000. (Until 2026-10-19 this paragraph and its laboratory gave
0.0000 for the first case, as if the sensor never changed the decision; it changes it in every bad year. Appendix E.) **The value of information is not a property of the datum: it is a
property of the set of available actions.**

**The invariance lens prefers the map to the number.** Three regimes, bases 100, 140 and 120, true effect 10 in all
three. In the first rule, the slope computed over all the data together gives **24.37**, more than double
the true effect, because those who received the treatment are concentrated in the high-base regime. Swapping
only the allocation, and keeping the effect of 10, the marginal slope becomes **−4.60**, negative, when every
effect within each regime is positive. It is the classic misalignment that Chapter 9 called
confounding. The slopes by regime, those, stay at 9.96 / 10.09 / 10.15 and 10.34 / 9.92 / 10.15. The
two rules pass the invariance test with tolerance 0.5. When the middle regime really responds
40 instead of 10, the invariance **fails** (9.96 against 40.09, step of 30.13). The marginal slope is a
number; the slope by regime is a structure.

**Exercises.**

1. *(Concept)* Write an $L_1$ node and an $L_2$ node for the claim "with this mean demand, the mean
   wait will be about 9 minutes". Which pathology edge links them, and in which sense does it point?
2. *(Concept)* Which lenses must a credit-granting model published to clients trigger?
   Justify each one in one sentence.
3. *(Count)* In the output above, the EVPI is 1.2000 with the full stake and 0.6000 with half of it. Explain why
   halving the stake halves the value of the same sensor, and compute the EVPI when acting costs 1 in a bad year
   instead of 4.
4. *(Laboratory)* Add a fourth node `L3_auditor` to the mini-hypergraph and check what the
   validator does before and after putting it into a hyperedge. Then create a second cycle in the tree and
   confirm that the message points at the cycle.
5. *(Laboratory)* Change the tolerance of the invariance test from 0.5 to 2.0 and to 0.05 in the two
   first rules. In which of them does the structure by regime stop being distinguishable from noise?
6. *(Concept)* Why was the Erlang C formula typed as epistemic (specification) and not as Knightian? Which
   observation would move it to Knightian?

**Summary.**

- The typed hypergraph keeps claims as nodes with layer, uncertainty type, formal definition and code.
- Pathology edges mark misalignments between layers; the validator refuses incomplete nodes, absent
  endpoints, deductive cycles and hyperedges with fewer than two members.
- The five lenses are transversal questions with their own quantities: information, time, tail, invariance
  and cost of information.
- The same graph gives different numbers according to the lens: Hill returns 1.7901 for $k = 500$; the EVPI returns
  1.2000, 0.6000 or 0.0000 depending on the available actions; the marginal slope changes sign while the
  slopes by regime stay equal.
- The node count inflates easily: of the 26,634 nodes of the scale audit, only 6,494 are concept nodes.

## Chapter 16. The team of specialists, the safeguard panel and the two systems of thought

**Objectives.** By the end of this chapter you should know the blind spot of each of eight specialists, go
through the six-stage cycle, apply the approval condition of the safeguard panel
$\mathrm{Approved}(X) = \bigwedge_{i=1}^{8} C_i(X)$, correct a forecast
when the base rate moves, check a posterior by simulation, and distinguish System 1 (fast triage, probabilities
held between 1% and 99%) from System 2 (deliberation: tree search, belief state, heavy inference and directed
stress).

---

### Step 1 — The real world and the paradox

**The group that approves itself.** A student hands in a statistics assignment. The student chose the sample,
cleaned the data, ran the test, looked at the result, changed the test, ran it again and wrote the conclusion.
When the conclusion finally pleased them, they stopped. Nobody cheated. Each step seemed reasonable at the moment
it was taken.

Now split the work among eight colleagues, each responsible for one single thing, and hand a ninth colleague the
**artifacts** in place of the report: the code, the data file, the saved model, the audit trail. The ninth
colleague's task is to break them. In an assignment made by one person alone, the most fragile part is the one
nobody looked at from outside. In an assignment with a team that attacks, the fragile part is the one that
survived the attack.

The ninth colleague has one rule: **do not read the report**. They run the code with a seed that the author did
not choose, inject a corrupted batch, tamper with a line of the trail to see whether the verifier notices.
Whoever only reads reports agrees with them.

**The eight specialists and the blind spot of each one.** Each specialist exists because there is a way of erring
that they see better than the others.

| Specialist | What they do | The blind spot they cover | What they deliver |
| --- | --- | --- | --- |
| Code | Turns hypotheses into executable generating processes | "The code runs" is not "the code is the hypothesis" | Fixed seed, pre- and post-conditions, property tests, profiles of time and memory |
| Data | Audits the entry of the World | What a row is, and why the datum is missing | A data contract, quarantine of defective records, idempotent ingestion, a byte budget |
| Lifecycle | Tests the model in time | The world changes and the model does not warn | Temporal cutoff without leakage, drift sensors, a policy for ignorance, post-mortem |
| Inference | Runs the heavy inference | One sample is not an answer | Chain diagnostics, calibration by simulation, conformal prediction |
| Governance | Takes care of legality, privacy and trail | Personal data is a combination of fields, not a file type | Impact opinion, differential privacy, a named human officer, a hash chain |
| Fairness | Asks who pays for the error | The chosen metric decides who pays | Audit of error rates between groups, the fairness trilemma |
| Domain | Translates problems from other areas | The parameter that does not come from the data comes from someone | Glossary, structured elicitation, scope term |
| Decision | Converts the result into action | A number without a natural frequency decides nothing | Natural frequencies, HOPs, the C/L tree, exit clause |
| Orchestrator | Coordinates and convenes the panel | No step approves its own result | The model-critique card and the attack on the artifact before approval |

**The craft of each specialist, in one case.** The data specialist begins with a decision that nobody sees:
**what a row is**. One row per person, per day or per event changes the mean, the variance and the margin, and for
that reason the grain is declared and proved with a count. The same specialist hunts the **sentinel value**, the
absence recorded as 0, −1 or 9 999 that enters the sum as if it were data, and requires idempotent ingestion:
running the load twice cannot change the number. And they ask the question that a limit on nulls does not ask:
**why is the datum missing?** The mechanism (MCAR, MAR or MNAR, Chapter 6) decides whether discarding the
incomplete cases is harmless, and a contract that only counts the nulls is silent on the day one appears.

The governance specialist recalls that personal data is not a file type: it is the **combination of fields**
that decides. Postcode, date of birth and sex, together, probably identify 87% of the people in the United
States, by the 1990 census. Differential privacy limits how much the presence of one person can change any
answer. In two databases that differ in the record of a single person, the result of any query cannot change by
more than a fixed factor. That factor is a budget, spent on each query. The cheapest measure remains not
collecting.

The lifecycle specialist requires that every model in operation declares its behaviour in three cases: input
outside the domain, missing field and low confidence. The options are to refuse, to fall back on a safe
deterministic rule or to return calibrated uncertainty. A model in use degrades for different reasons, which ask
for different answers.

| Situation | What changes | The right answer |
| --- | --- | --- |
| Covariate drift | The distribution of the inputs changes, and the relation between attribute and outcome still holds | Reweight, with weights by the ratio of the densities |
| Label drift | The frequency of the outcomes changes, and the distribution of the attribute within each outcome still holds | Readjust the base rates (intercept correction) |
| Concept drift | The relation between attribute and outcome itself changes | No reweighting saves it: rebuild the model |
| Served population changed | The metric falls because the audience changed | It is not a degradation of the model: read the metric against the base rate |

The domain specialist does not ask an expert for a mean. The Sheffield protocol (SHELF) asks for the **median**,
the **quartiles** and the **plausible extremes**, fits a distribution to those quantiles and gives it back so that
the expert confirms it, controlling the interviewer's anchor, the order of the scenarios and overconfidence. When
no expert can be heard, the least one can do is measure **how much the decision depends on the prior**. In 33
domains, the prior in use was replaced by a prior from the literature and by a deliberately weak one: five domains
change status or action with the literature prior and two with the weak one. The lesson is not that the first
priors were wrong. It is that in five of 33 domains the decision does not survive the replacement.

**System 1 and System 2.** A machine that helps thinking has two modes. **System 1** does fast, categorical
triage: the same input always gives the same answer, and every probability it emits is **held inside the interval
from 1% to 99%**. **System 2** deliberates: it searches a tree of options, keeps a belief about where the weak
point is, runs heavy inference and directs the stress of the panel. The rule of the interval is not aesthetics. A
probability of 0% or of 100% can be scored, and when it is wrong it is punished hard: the Brier score charges it
the maximum of 1, and the logarithmic score charges it an infinite penalty. The real defect is in learning. By
Bayes' rule a prior of 0 or 1 stays at 0 or 1 whatever the evidence, so a system that emits certainties cannot be
moved by what it sees next (Lindley called this Cromwell's rule).

Cromwell's rule forbids 0 and 1; it does not choose the floor. The interval from 1% to 99% is a **convention of
System 1's triage**, where the categories are few and none is rare, not a law for every forecast. Applied to a
forecaster of rare events it would do damage: an event with a base rate of 0.2% forecast at no less than 1% is
over-forecast five times on every quiet day, and the Brier and logarithmic scores charge for it. For forecasts,
the floor $\varepsilon$ is declared from the base rate and from the size of the data (Laplace's rule,
$1/(n+2)$, is one honest choice), and is never wider than the rarest outcome the model must predict.

---

### Step 2 — The rule of the game

**The six-stage cycle.** A question goes through six stages, each with its output cards. The path to the decision
passes through all of them.

1. **Translation and elicitation (Domain).** Card of the transfer, elicitation, scope term.
2. **Ingestion, grain, causality and ethics (Data, Governance, Fairness).** Data contract, card of the sample,
   causal graph with the Pearl step, privacy opinion, fairness audit.
3. **Simulation, null model and frontier (Code, Inference, Lifecycle).** Executable generating process, null model
   simulated **before** reading any pattern, inference diagnostics, partition outside time, count of attempts.
4. **Projection onto the five lenses (Orchestrator).** Sweep of the information, time, tail,
   invariance and decision lenses.
5. **Safeguard panel (Orchestrator).** Rejected goes back to the stage that failed; approved moves on.
6. **Decision and closure (Decision).** Memorandum in natural frequencies, HOPs, the C/L tree and exit clause.

A rejected assignment goes back to the **stage in which it failed**, instead of going back to the beginning.

**The safeguard panel.** Three failures eliminate the assignment before any discussion of merit: temporal
leakage, a violated data contract without treatment and the absence of a named human officer. After that, each
specialist's work receives its attack, eight in total.

| Attack ($A_i$) | The attack | The safeguard the card must carry |
| --- | --- | --- |
| $A_1$ Code | A new battery of properties, with a seed the author did not choose | property test |
| $A_2$ Data | A batch with corrupted records, which must be isolated | handling of missing values |
| $A_3$ Lifecycle | A batch with strong drift, live | declared drift |
| $A_4$ Inference | A pathological data set, to see whether the uncertainty denounces the inadequacy | pathologies covered |
| $A_5$ Governance | Tampering with the audit trail, which the chain verifier has to detect | intact trail |
| $A_6$ Fairness | Inverting the chosen metric and showing who would come to pay | audited subgroups |
| $A_7$ Domain | An atypical scenario of the domain, with an alternative prior | elicited prior |
| $A_8$ Decision | A crisis that violates the exit clause itself | declared exit clause |

**The approval condition.** For each attack $A_i$ the panel reads two bits from the card: whether the safeguard
of that row was declared, and whether the structural field that anchors it is present (a property test anchored
by a runnable artifact and a seed, an exit clause anchored by a named payer of the error). The artifact $X$ is
approved only if **all** eight rows pass:

$$\mathrm{Approved}(X) = \bigwedge_{i=1}^{8} C_i(X),\qquad
C_i(X) = \mathrm{safeguard}_i(X) \wedge \mathrm{anchor}_i(X).$$

The $\bigwedge$ is the logical "and". One single row that fails is enough for the assignment to go back. The
condition reads no seed and no judgement: the same card always gets the same sixteen bits, and it **fails** when a
safeguard was declared absent or left without its anchor, which is what makes it auditable. The panel is a
checklist. It answers "was the safeguard declared, with the field that anchors it?", not "does the safeguard hold
under attack?". The second question belongs to tests that run the attack, such as the tampering of the trail in
the gates of Chapter 17.

**Why the panel prints bits.** Until 2026-10-19 each row printed a "severity" from 0 to 10, drawn by a seed inside
a range that the two bits selected, against a floor of 6. No range crossed the floor, so the verdict was always the
AND of the bits; whether a declared safeguard scored 8, 9 or 10 was decided by the seed and said nothing about the
artifact. A decimal that only encodes a yes or a no is the false precision this book warns against elsewhere, and
it borrowed the name of Mayo's severity (Chapter 7), which is a probability about a procedure. The panel now prints
the bits, and the name was changed with it: it was called the "adversarial panel", which described what it **asks
for**, an attack on each specialist's work, and not what it **runs**. Both changes are recorded in Appendix E.

**How teams of agents fail, and what the panel sees.** A study of more than two hundred traces of systems of
language-model agents sorted their failures into fourteen modes and three classes (Cemri and colleagues, 2025): the
specification (an agent disobeys its task or its role, repeats steps, loses the history, does not know when to stop),
the misalignment between agents (one withholds what it found, ignores what another said, acts against its own
reasoning), and the verification (it stops early, checks too little, or checks wrongly). Most of them came from the
design of the system, not from the model inside it. Held against the fourteen, the panel and the cycle of this
chapter catch four, the ones that are **declared before**: repeated attempts, which the counter of attempts counts; the
lost history, which the trail keeps; and the stopping, which a rule written in advance fixes twice over. They catch
seven only in part, because they check that something was declared, not that it was done. Three they do not see at
all: a specialist that leaves its role, one that withholds a finding, and an action that departs from the reasoning
written beside it. All three happen **between** the specialists, and a checklist of cards cannot see what passes
between them; it takes a record of the exchanges, and a reader of that record who is not one of them.

**The model-critique card.** The orchestrator approves only after filling five items: a predictive check against
replicates of the fitted model; comparison with at least one rival model; separation between parameters
identified by the data and parameters brought from outside; analysis of performativity, Goodhart and Lucas; and a
defence of the undecidable premises by the **cost of the error**.

**An assignment with a declared gap is worth more than a uniform one.** The complete artifact with one admitted
weak point beats the uniform artifact that admits no gap at all. In the Danish fire-insurance case, the panel
rejected the claim; the claim lost, but the rival model **did not win the right to decide** either.

**Correcting a forecast when the base rate moves.** Label drift has a closed-form correction. If the distribution
of the attribute within each class does not change, the corrected probability is the old one multiplied by the
ratio between the new base rate and the old one, then renormalised:

$$P_{\text{new}}(y \mid x) \;\propto\; P_{\text{old}}(y \mid x)\,\frac{\pi_{\text{new}}(y)}{\pi_{\text{old}}(y)}$$

With two outcomes, the correction adds a constant to the logit: it is an **intercept correction**. When the new
base rate is unknown, Saerens, Latinne and Decaestecker (2002) estimate it from the unlabelled new cases
themselves, alternating two steps until they agree: correct the forecasts with the current guess of the base
rate, then take the mean of the corrected forecasts as the next guess. The hierarchy of Van Calster and colleagues
(2016) separates two levels of calibration. One is calibration in the mean, measured by the intercept; the other
is **weak calibration**, which requires intercept 0 and slope 1. A flight-delay forecast for 2023 passed with the
intercept correction alone; an arrest forecast for Toronto did not, because its error was one of slope, and no
shift of base rate repairs a slope.

**Checking a posterior by simulation.** Heavy inference returns a cloud of samples, and the cloud can be the wrong
size without any warning. Calibration by simulation (SBC) tests the machinery itself. Draw a parameter
$\tilde\theta$ from the prior, simulate data with it, run the inference on those data and record where the true
value falls among the $L$ posterior samples. Repeat many times. If the inference is right, the rank is uniform:

$$\mathrm{rank}(\tilde\theta; \theta^{(1)}, \dots, \theta^{(L)}) \sim \mathrm{Uniform}\{0, \dots, L\}$$

A histogram of ranks with a hump in the middle means posteriors that are too wide; a U shape means posteriors that
are too narrow, the overconfident case; a slope means a biased sampler. The check needs no real data, because the
truth is known by construction.

**System 2, in four tools.** Deliberation is not a mood; it is a set of procedures, each with a null to beat.

- **Tree search with estimate (upper confidence bound, the search behind LATS).** A greedy agent takes the option of
  greatest immediate value and stops.
  Tree search expands the options, estimates the value of each branch by simulation and balances exploring
  untried branches against exploiting good ones, by the upper confidence bound. It can find a path that begins
  with a worse step and ends somewhere better, which the greedy agent never sees.
- **The belief state.** System 2 keeps a probability for each hypothesis about where the weak point is, updated
  with every card the panel has rejected. The belief has a floor and a ceiling: no hypothesis falls to 0% or rises
  to 100%, so that a surprise can always move it. Its entropy says how concentrated it is.
- **Directed stress.** The ordinary panel attacks the eight points in a fixed order. Directed stress attacks first
  where the belief points and stops at the first rejection. The null is the fixed order: if learning adds nothing,
  both orders find the same rejection with the same number of attacks.
- **Heavy inference (Metropolis).** When the posterior has no closed form, a random walk proposes a new value near
  the current one and accepts it with probability $\min\{1, p(\text{new})/p(\text{current})\}$. The chain spends time
  in each region in proportion to its posterior probability. Its honesty is tested where the answer is known: on a
  Beta distribution, the mean of the chain must meet the closed form.

The same discipline decides the grain of a question. Refining from the average day to each day is worth it only
when the measured loss of the coarse question pays for the margin the fine one requires; finer grains that the
instrument cannot measure stay declared, never guessed.

**When to wake System 2: the value of computation.** Deliberation is not free: it costs time, attention and, in a
machine, calls and money. The right comparison is therefore not between System 1 and a reasoner without limits, but
between System 1 and System 2 **net of what System 2 costs**. Cognitive science has a name for this reading,
**resource rationality** (Lieder and Griffiths, 2020): a shortcut that looks like a bias can be the best use of a
limited budget of computation, and it stops being one only when thinking more would change the act by more than the
thinking costs. The value of computation is to thinking what the value of information of Chapter 11 is to measuring:
the expected loss of the act chosen now, minus the expected loss of the act chosen after deliberating, minus the cost
of deliberating. It is zero when no result of the deliberation could change the act, which is the case of a triage
far from its threshold. So System 2 is woken where a small revision flips the decision, near the threshold, and where
the dominant event of a wrong act is severe; everywhere else, the fast answer is the rational one.

**Reliability is not average success.** A machine that acts is judged, almost always, by the share of tasks it
completes, its average success. Whoever delegates asks something else: if I hand it this task every week, will it
succeed every week? Run each task $n$ times and call $p_i$ the chance of success on task $i$. The average success is
$\text{pass}^1 = \mathbb{E}[p]$; the chance that $k$ runs of the same task all succeed is
$\text{pass}^k = \mathbb{E}[p^k]$. If every task had the same chance, $\text{pass}^k$ would be $(\text{pass}^1)^k$.
They do not, and by Jensen's inequality $\mathbb{E}[p^k] \ge (\mathbb{E}p)^k$, with equality only when the chances
do not vary. The distance between the two is the variance of the chances across tasks at work, the same term that
the variance of a sum carries in Chapter 6. It has two faces. The good one: an agent that solves some tasks always
and others never is more reliable, on the tasks it solves, than its average suggests. The bad one: its average says
nothing about which tasks those are, and a single run with a fixed seed cannot say how far its own number would move.
The card of an agent therefore declares $k$ with $\text{pass}^k$, not only the average, and reads the delegation by
the tail: the dominant event of a failure, not the task length it completes half the time.

---

### Step 3 — Hands on

Three measurements: run the complete panel over a card with the eight safeguards and over cards with safeguards
removed; check that the triage of System 1 never leaves the interval between 1% and 99%; and let System 2
deliberate. Run the first with `cargo run --release --bin labs -- --chapter 12`.

**The recipe of the laboratory.** The panel reads the eight bits of each card and nothing else. The triage of
System 1 is a deterministic stand-in, not a model: the bytes $b$ of the card's identifier are folded into a state
$s \leftarrow 1{,}099{,}511{,}628{,}211\,s + b$ (modulo $2^{64}$, starting at 0), one step of the SplitMix64 generator is
taken from that state, and the probability is $(v \bmod 99 + 1)/100$, held inside $[0.01; 0.99]$. Its only claims are the
two the line checks: the same card gives the same number, and no number leaves the interval.

```text
== Chapter 16 :: the A1-A8 safeguard panel ==
complete card (card h3.voos): A1-code=1 A2-data=1 A3-lifecycle=1 A4-inference=1 A5-governance=1 A6-fairness=1 A7-domain=1 A8-decision=1 | passed=8 of 8 approved=true
card with A5 and A6 removed (card h3.voos-com-defeito): A1-code=1 A2-data=1 A3-lifecycle=1 A4-inference=1 A5-governance=0 A6-fairness=0 A7-domain=1 A8-decision=1 | passed=6 of 8 approved=false
without A1-code: rejecting=["A1-code"] count_rejecting=1
without A2-data: rejecting=["A2-data"] count_rejecting=1
without A3-lifecycle: rejecting=["A3-lifecycle"] count_rejecting=1
without A4-inference: rejecting=["A4-inference"] count_rejecting=1
without A5-governance: rejecting=["A5-governance"] count_rejecting=1
without A6-fairness: rejecting=["A6-fairness"] count_rejecting=1
without A7-domain: rejecting=["A7-domain"] count_rejecting=1
without A8-decision: rejecting=["A8-decision"] count_rejecting=1
sweep: cards rejected=8 of 8 | rejections per agent=[1, 1, 1, 1, 1, 1, 1, 1]
seed read by the panel: none (the verdict is a function of the eight bits)
system 1: label=card-in-order probabilities=[0.06, 0.94] sum=1.0000 outside_[0.01;0.99]=0
```

Four readings.

**The complete card passes.** The eight bits are set, and the verdict reads no seed: there is nothing to vary.
A rejection comes from a missing safeguard, never from a draw.

**Each safeguard removed brings down exactly the row that charges for it.** In the eight lines of the sweep, the
set of rejecting rows always has **one** element, and it is always the corresponding one. Without an intact trail
$A_5$ rejects, without an exit clause $A_8$ rejects, without a property test $A_1$ rejects, and the other seven stay
set. The panel is not a mean score that dilutes, but a logical "and" that locates the failure.

**Removing two safeguards rejects the card and names both.** With the governance and fairness safeguards removed,
6 of 8 bits stay set, the card is rejected, and the line says which two rows fell.

**The triage respects the interval.** The last line returns 0.06 and 0.94, sum 1.0000, and zero values outside
1% to 99%.

A fuller System 1 sorts a card into one of six declared themes with a suffix tree, and the same contract holds on
every line. Run it with `cargo run --release --bin labs -- --s1`.

```text
== System 1 :: deterministic triage of cards ==
suffix tree: 129 suffixes indexed over 6 declared themes
category=queue probability=0.9117 products=[0.9117, 0.0177, 0.0177, 0.0177, 0.0177, 0.0177] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9117 | sum 1.0000 | outside [0.01;0.99] 0
category=causal probability=0.9297 products=[0.0141, 0.9297, 0.0141, 0.0141, 0.0141, 0.0141] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9297 | sum 1.0000 | outside [0.01;0.99] 0
category=tail probability=0.9117 products=[0.0177, 0.0177, 0.9117, 0.0177, 0.0177, 0.0177] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9117 | sum 1.0000 | outside [0.01;0.99] 0
category=calibration probability=0.9415 products=[0.0117, 0.0117, 0.0117, 0.9415, 0.0117, 0.0117] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9415 | sum 1.0000 | outside [0.01;0.99] 0
category=fairness probability=0.9217 products=[0.0157, 0.0157, 0.0157, 0.0157, 0.9217, 0.0157] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9217 | sum 1.0000 | outside [0.01;0.99] 0
category=other probability=0.8718 products=[0.0256, 0.0256, 0.0256, 0.0256, 0.0256, 0.8718] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.8718 | sum 1.0000 | outside [0.01;0.99] 0
```

The sum is 1.0000 and no probability falls outside [0.01; 0.99]. The last line is an input that matches no theme:
it falls into `other` with 0.8718, the declared prior, not a guess. The limits are worth stating. The vocabulary is
closed, the matching is lexical and does not understand negation, and this is triage, not judgement. The numbers it
emits are normalised scores that obey the rules of probability; nobody has checked that a card triaged at 0.91 lands
in its theme 91 times in 100, so they are not yet calibrated probabilities, and the honest name for them is scores. Judging
belongs to System 2. Run it with `cargo run --release --bin labs -- --s2`.

```text
== System 2 :: deliberation over the hypergraph ==
declared graph: 8 nodes, 6 tree edges, 2 graph edges, 5 hyperedges | validation ok
nodes that no lens covers: ["f1.code-synthesis"]
greedy: stops at h0.queue (value 0.600) | LATS: root.stochology -> r.resolution -> f1.code-synthesis (value 1.000, node without lens true)
board belief: most likely band A5-governance with probability 0.9205 | entropy 0.4324
stress: deliberated stopped at A5-governance with 1 attack(s) | declared order stopped at A5-governance with 5 attacks | same rejection true
inference: Beta(4, 8) chain mean 0.3346 against the closed form 0.3333 | acceptance 0.676
R axis: loss of the aggregate 25.54 min against a margin of 12.00 min -> R1-dia (the loss of the aggregate pays for the margin of the day) | out of measured reach: R2-hora, R3-subpergunta
R axis without oscillation: loss 0.00 min -> R0-agregada
```

- **The greedy agent stops at the queue** with value 0.600. Tree search goes further and reaches value 1.000, at a
  node that no lens covers, which is itself a finding: the deliberation found the hole in the map.
- **The belief** points at governance with probability 0.9205, and entropy 0.4324 says the belief is concentrated.
- **Directed stress** reaches the rejection with 1 attack; the fixed order needs 5 to reach the same one. Learning
  saved four attacks and changed no verdict.
- **Metropolis** on a Beta(4, 8) gives a chain mean of 0.3346 against the closed form 0.3333, with acceptance 0.676.
- **The grain:** with oscillating demand, the coarse question loses 25.54 minutes, more than the 12.00 minutes of
  margin the fine question costs, so System 2 refines to the day; without oscillation, it stays at the aggregate.

**Reliability, measured.** The laboratory of reliability draws the chances of 200 tasks from four worlds, runs each
task 8 times, estimates $\text{pass}^k$ per task as $\binom{c}{k}/\binom{8}{k}$ from its $c$ successes (the
unbiased estimator of $p_i^k$), and repeats the whole experiment 2,000 times to see how far one run of the laboratory
moves. Every world but one has average success near one half. Run it with
`cargo run --release --bin labs -- --passk`; the world where most tasks sit near 0 or near 1:

```text
-- world C, Beta(0.3, 0.3) --
   k | first run: pass^k  (pass^1)^k | 95% band of pass^k  | exact E[p^k] | 95% band of the gap
   1 |          0.5150      0.5150  | [0.4419; 0.5587]  |     0.5000   | [+0.0000; +0.0000]
   2 |          0.4325      0.2652  | [0.3471; 0.4659]  |     0.4062   | [+0.1406; +0.1708]
   3 |          0.3920      0.1366  | [0.2996; 0.4192]  |     0.3594   | [+0.2075; +0.2590]
   4 |          0.3661      0.0703  | [0.2716; 0.3899]  |     0.3294   | [+0.2292; +0.3027]
   5 |          0.3471      0.0362  | [0.2494; 0.3687]  |     0.3079   | [+0.2305; +0.3218]
   6 |          0.3321      0.0187  | [0.2334; 0.3518]  |     0.2914   | [+0.2238; +0.3275]
   7 |          0.3200      0.0096  | [0.2188; 0.3400]  |     0.2782   | [+0.2138; +0.3261]
   8 |          0.3100      0.0049  | [0.2050; 0.3300]  |     0.2672   | [+0.2037; +0.3213]
  agent card: reliability k = 8; pass^1 0.5150; pass^8 0.3100; read as independent 0.004948; ratio 62.6
  exact moment inside the band for every k: true | gap band contains 0 for every k >= 2: false | gap band above 0 for every k >= 2: true
```

Three readings.

- **Same average, a factor of sixty.** With $\text{pass}^1$ near one half, the independent reading promises that
  8 runs in a row succeed on 0.0049 of the tasks; the laboratory finds 0.3100. The tasks the agent solves, it solves
  almost always, and the tasks it fails, it fails almost always. In the world where every task sits at one half the
  two readings agree (0.0050 against 0.0041), and the band of their gap contains zero at every $k$.
- **One run of the laboratory is a draw.** The band of $\text{pass}^8$ over the 2,000 repetitions goes from 0.2050
  to 0.3300: a benchmark of this size, published once, fixes the reliability within a factor of about one and a half,
  not to the fourth decimal it prints.
- **The estimator is honest.** The exact $\mathbb{E}[p^k]$ of the Beta law lies inside the band at every $k$, in all
  four worlds.

The three rules were written down before the first run, and all three hold:

```text
-- the rules declared before the run --
  R1, world A: the gap band contains 0 for every k >= 2: holds
  R2, worlds B, C and D: the gap band lies above 0 for every k >= 2: holds
  R3, every world: the exact E[p^k] lies inside the band of the estimator for every k: holds
```

**Exercises.**

1. *(Concept)* A database has one row per medical appointment, and the question is about patients. Which
   specialist must block the analysis, and with which test?
2. *(Concept)* Draw the attack of the panel for a credit triage model. Which crisis would violate the exit clause?
   Which group would come to pay if the metric were inverted?
3. *(Count)* A card has 7 of its 8 safeguards declared and anchored, and the eighth declared without its anchor.
   How many of the sixteen bits are set, and is the card approved? Justify with the definition of $\bigwedge$.
4. *(Laboratory)* Implement calibration by simulation (SBC) for a Beta-Binomial model: draw the parameter from the
   prior, simulate the data with it and run the sampler. Check that the rank of the true value among the posterior
   samples is uniform. Then introduce an error in the sampler and watch the histogram stop being flat.
5. *(Count)* A classifier was trained where the base rate was 10% and now serves a population where it is
   30%. A case receives 0.20 from the old model. What is the corrected probability? Which assumption must hold for
   the correction to be right?
6. *(Concept)* Give one example of each mode of degradation of a model in use (covariate drift, label drift and
   concept drift) and an example of the fourth situation, which is not a degradation. Say the right answer for
   each one.
7. *(Concept)* Why must System 1 be deterministic (repeated input, equal probability)? Relate it to the fixed seed
   of Chapter 1. Why, in contrast, must the belief of System 2 never reach 0% or 100%?
8. *(Count)* A triage sends a case to "protect" when its probability passes 0.30. Deliberating costs 2, and the
   loss of a wrong act is 100. For a case at 0.05 and for a case at 0.29, say whether deliberation could change the
   act, and whether its value can exceed its cost. Where is System 2 worth waking?
9. *(Count)* Two agents have average success 0.5 on the same 200 tasks. The first succeeds on every task with chance
   one half; the second solves half of the tasks always and the other half never. What is $\text{pass}^8$ for each?
   Which one would you trust with a weekly task, and which question would you ask before choosing?

**Summary.**

- Eight specialists, each linked to a way of erring, plus an orchestrator that attacks; nobody approves the work
  they produced.
- The cycle has six stages, and a rejection returns to the stage that failed, not to the beginning.
- Approval is a logical "and" of eight checks, each a safeguard declared and anchored; each missing safeguard
  brings down the row that charges for it, and no seed enters the verdict.
- A moved base rate is corrected on the logit; a wrong slope is not. SBC checks the inference where the truth is
  known by construction.
- System 1 triages deterministically with probabilities between 1% and 99%. System 2 deliberates by tree search, a
  bounded belief state, directed stress and Metropolis, each tested against a null.
- Teams of agents fail by specification, by misalignment between agents and by verification; a checklist of cards
  catches what is declared before and misses what passes between the specialists.
- Reliability is $\text{pass}^k = \mathbb{E}[p^k] \ge (\text{pass}^1)^k$; the gap is the spread of the chances
  across tasks, and the card of an agent declares $k$ with it.
- Thinking more is an act with a value: System 2 is woken where its result could change the decision by more than it
  costs, near the threshold and where a wrong act is severe.

## Chapter 17. The iron contract: cards, prohibitions, invariants and gates

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

**How the contract locks that.** The domain package carries a mandatory field, the
`decision_maker_question`, with three parts in order: **who decides**, **which action the decision changes**
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
| `generating_process` | Which hidden mechanism generates the data, written in a simulable way, and in which layer can the claim fail? |
| `nonlinearity` | Is the result a non-linear function of something that varies? Compare $f(\mathbb{E}[X])$ with $\mathbb{E}[f(X)]$; is the dynamics additive or multiplicative? |
| `unit_and_margin` | What is the de facto independent unit, and was the margin computed with it? |
| `noise_physics` | What noise is this (gaussian or with jumps)? Are there absorbing states? Is the agent passive or does it deform the world? |
| `causal_ladder` | At which step of Pearl is the statement, with which identification strategy and which untestable premise? |

The header of a real claim about a bank call centre, abbreviated:

```yaml
claim: >
  "Staffing each half hour by Erlang C on the historical mean rate, the centre answers
  82% of the calls within 20 s, with a mean wait of 10 s, every day."
header:
  nonlinearity: >
    The Erlang C service level is concave and collapses near saturation: applied to the real
    rate of each day of the vault, the claim's own formula gives 57% on average, not 82%.
  unit_and_margin: >
    The independent unit is the DAY: the calls of one day share the rate and the queue.
    Effective n = 81 days of vault, not 2.65 million calls.
  noise_physics: >
    Daily demand is NOT Poisson: dispersion Var/mean = 273 (Poisson: 1).
  causal_ladder: >
    Step 2 (intervention): the staffing rules are do(staffing) simulated on the real demand.
    Untestable premise: demand does not react to staffing.
```

Other cards add questions to the five of the header. The card of the evidence asks for the base rate, the
two likelihoods and the posterior computed in two ways, in odds and in a tree of 10,000 cases, which
have to coincide. The card of the causal graph begins with the step. The card of the model critique closes the series and says in which
item each previous card enters.

**Two fields for the uncertainty that reaches the decision.** Since version 1.3 of the contract, a card can declare
where the uncertainty of its answer comes from and which event carries its cost. The **uncertainty budget** splits
the variance of the output among the inputs and their covariances, as shares that must add up to one, so that nothing
is left unaccounted for; for the health post of Chapter 5 with independent rates it reads
`arrivals=0.42; pace=0.58; covariance=0.00`, and it says that a better measure of the nurse's pace buys more than a
better count of arrivals. The **dominant event** names the event that carries the expected cost, with its probability
and its share; for the shipowner of Chapter 11 it reads `the ship is lost; p=0.05; share=1.0`. The budget says where
to measure; the value of information of Chapter 11 says whether measuring would change the decision; the two are
different questions, and the card asks both. The fields are optional: a card that leaves them out is valid, and the
migration from 1.2 brings them in absent rather than inventing a value. A card that declares a budget that does not
close, or an event with a probability of 0 or 1, is refused when it is read.

**Two more fields: the choice of the model and the tail of an agent.** Version 1.4 adds two. The **multiverse** says
under how many reasonable specifications the answer was computed, how many of them reach the card's decision, and how
the variance between them splits by choice; for the health post it reads
`specifications=18; agree=10; shares=service-cv:0.93, rate-law:0.00, correlation:0.06, interaction:0.02`, and it says
that the card's decision is kept by ten models of eighteen and turns on a quantity nobody measured. A budget may then
carry a share for the specification beside its inputs, and a budget that names that share without a multiverse is
refused, because that share is measured there and nowhere else. The **reliability** is for the card of an agent: the
number of runs $k$, the average success and the chance that all $k$ runs succeed, as in
`k=8; pass1=0.5312; passk=0.0700`; a card that declares a $\text{pass}^k$ above its $\text{pass}^1$ is refused,
because $k$ runs cannot all succeed more often than one does. Both fields are optional and enter absent in the
migration from 1.3, for the reason of the earlier two.

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
same symbol describes another process: the typical trajectory grows at $\mu$, and the expectation at
$\mu + \tfrac12\sigma^2$. Notice what changes and what does not. The gap between the growth of the expectation and
that of the typical trajectory is $\tfrac12\sigma^2$ **in both readings**; the drag is Jensen, and no convention
removes it. What the convention changes is **which of the two rates the parameter $\mu$ names**, and therefore the
number one obtains when a $\mu$ estimated under one reading is plugged into the other. With additive noise,
$dX = \mu\,dt + \sigma\,dW$, the two readings coincide and the question does not arise. The choice depends on the physics.
Idealised white noise of a discrete system asks for Itô; the limit of a real noise with short correlation asks for
Stratonovich. The card also asks for a test with a smaller time step. If the result changes when $\Delta t$
shrinks, the simulation was measuring the integrator instead of the World. That test does **not** decide between Itô and
Stratonovich, because the Euler scheme converges to the Itô reading, and the Stratonovich one asks for another integrator.

Noise also makes the system jump between stable states. In a system with two wells, the mean time until the jump
follows the Kramers formula and grows exponentially with the height of the barrier $\Delta U$ with respect to the
temperature $T$:

$$\tau \approx \frac{2\pi}{\sqrt{U''(x_{\min})\,\lvert U''(x_{\max})\rvert}}\; e^{\Delta U/T}.$$

For high barriers, that time has almost no memory. Whoever waited a long time is not closer to the jump.
Looked at only in the jumps, however, the noise looks like a push: just before a jump, the noise almost always
points in its direction, with no external force at all, because a jump is selected precisely by the noise that made it. Looking for the cause of a
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

**Testing a property, not an example.** An ordinary test checks one case: with these inputs, this output. A
**property test** states a rule that must hold for every input, and lets the machine search for a counterexample.
"The margin of a proportion is never negative and never larger than one half", "the wait of a queue never falls
when the service gets slower", "running the load twice gives the same table". A generator draws thousands of
random inputs with a declared seed; when one breaks the rule, the tool shrinks it to the smallest input that still
breaks it, and that small case is usually the bug. The property is a claim about the code, written in the form of
a severe test: it could fail on any of the inputs, and it is worth something only because it did not.

**The funnel of four filters.** A system that simulates every hypothesis it meets collapses from cost and from
inflation of false discoveries. The funnel works with two companions: pruning by value of information and a counter
of attempts. The four filters refuse by the header (without a
question, without a generating process, without a unit or null, without an action with an exit). The pruning cuts when the quota of the value
of information cannot cross the threshold: with $C/L = 0.1$ the quota is worth 0.0090 at $p = 0.001$ and the test of
0.05 is pruned, while at the threshold $p = 0.10$ the quota is 0.9000. Finally, the counter closes the account: how many specifications were tried, at which filter each one fell, how
many were approved and which one is final. Its limit is declared: the quota is the expected value of perfect
information, exact only in the model of two acts and two states. Each front organises
itself as a funnel, and in it most of the weak hypotheses fall before touching heavy simulation.

1. **Value of information.** The hypothesis that would not change any decision even with a perfect answer is cut. The
   standard log is "pruned by Crisonomy: curiosity without operational use", Crisonomy being the label of the decision lens.
2. **Topological pruning and unit.** By d-separation, every node that stays outside the Markov blanket of the
   variable of interest is cut, including the colliders that, if conditioned on, would create Berkson's bias. The claim
   that the data do not identify is also cut, or the one whose effective size, counted on the independent unit, stays too small to conclude. The data contract also classifies the mechanism of
   every absence (MCAR, MAR, MNAR or not declared), with the columns the classification covers and a sensitivity
   analysis when the absence may depend on the value itself. An absence without classification is refused.
3. **Severe test against the null.** If the alleged pattern already appears in the null model, or in a reference with a single
   parameter such as the base rate, the research closes there. When the simulation is expensive, that test runs first in a
   cheap emulator.
4. **Heavy inference, lenses and panel.** Only the remainder receives the expensive treatment.

**Worked case: the identifiability of $R_0$.** At the beginning of an epidemic, the cases grow like
$\exp((\beta - \gamma)t)$, and the data determine the **difference** $\beta - \gamma$, not the ratio
$R_0 = \beta/\gamma$. With a few weeks of noisy cases, a wide range of $R_0$ fits the curve equally well, and the
predicted final size ranges from about half the population to all of it. More days narrow the range, but by then the
curve has already grown. Taking the duration of the infection from other studies fixes $\gamma$ and narrows $R_0$
at once. That is not a defect, as long as one declares that the margin became the one of the source.

**The automatic gates.** No delivery is consolidated without passing the gates on a clean machine. The
agents and the $\Phi$ have already been formalised; the gates **repeat the work and compare**. A repeated input has
to produce an identical output, and a planted defect has to be detected. Seven gates make a complete set.

| Gate | Who answers | What it verifies | What it does not guarantee |
| --- | --- | --- | --- |
| G1 | Orchestrator | Determinism: the Merkle root of the second run is identical to the one of the first | That the result is right: a wrong account reproduces as faithfully as a right one |
| G2 | Code | Fixed seed declared in every card | That the seed was chosen before the result was seen |
| G3 | Code | Declared scope: only a simulable artifact | That the artifact simulates the claim it is attached to |
| G4 | Code and Domain | Invariants $\Phi_1$–$\Phi_4$ and the type III lock | That the fields are true, or that the decider question is the decider's: the lock reads a question mark and a quoted action, not meaning |
| G5 | Governance | SHA-256 trail intact **and** tampering detected | Provenance or truth: a fabricated record sealed before publication passes |
| G6 | Orchestrator | Safeguard panel $A_1$–$A_8$: every check declared and anchored | That any safeguard works: the panel checks declared and anchored safeguards, it runs no attack |
| G7 | Decision | Declared exit clause and $A_8$ above the floor | That the exit clause will be honoured when its trigger fires |

Two details make the set worth more than the sum. G5 does not verify only that the chain is intact. It
**tampers on purpose** and requires the verification to fail with the tampered chain. G1, in turn,
compares two runs of the declared horizon. A gate that never fails is not a gate (Chapter 18). The last column is
there for the same reason. A gate that passes is evidence only of what it checks, and every one of the seven checks
form, presence or reproducibility; none of them checks that a number is correct. That job stays with the tests that
compare a result with a known answer, and with readers who redo the account.

---

### Step 3 — Hands on

The laboratory validates a complete card and a card with one planted defect at a time, printing the failures by
gate. Run it with `cargo run --release --bin labs -- --chapter 13`.

**The recipe of the laboratory.** The property test draws 10,000 cards with a sample size $n$ uniform between
100 and 999 and a rate $p$ uniform in $\{0.01, 0.02, \ldots, 0.99\}$, and computes the margin $1.96\sqrt{p(1-p)/n}$. It
checks one property only, that the margin lies in $[0; 0.5]$; by construction it lies between
$1.96\sqrt{0.01 \cdot 0.99/999} = 0.00617$ and $1.96\sqrt{0.25/100} = 0.098$. That the margin shrinks when $n$ grows is
true of the formula and is not what this test checks.

```text
== Chapter 17 :: the iron contract ==
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

**The structural validator crosses the chapters.** The three refusals of Chapter 15 reappear here:
an edge to a ghost node, a deductive cycle and a hyperedge with a single member. That is on purpose. A lock that only
exists in one chapter is a lock that nobody uses.

**The property test and the seed.** Ten thousand simulated cards with margin $1.96\sqrt{p(1-p)/n}$ for
$n$ between 100 and 999 and $p$ between 0.01 and 0.99 give **zero violations**, with margins between 0.00617 and 0.09798. The
confidence interval is never negative nor greater than a half. The two validators run over the declared
seed diverge in **0 of 32** runs, the reproducibility that Chapter 1 requires.

**Exercises.**

1. *(Concept)* Fill the five questions of the header for the claim "students who study with music
   get higher grades".
2. *(Concept)* Which filter of the funnel must cut the question "what is the favourite colour of the clients who buy
   insurance?" in an insurer that is not going to change the product?
3. *(Count)* Why do 25 days of exponential growth identify $\beta - \gamma$ and not $\beta/\gamma$?
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
  `decision_maker_question` locks the type III error.
- The four prohibitions and the five invariants are locks, and not recommendations; $\Phi_5$ is the newest and the narrowest.
- The funnel cuts cheaply before spending expensively: value of information, topological and unit pruning, severe test
  against the null, and only then heavy inference and panel.
- Seven gates repeat the work and compare, each with a pass or a fail and no third outcome, and a gate that
  never fails is not a gate.
- A property test states a rule that must hold for every input and lets the machine hunt for the
  counterexample.
- Version 1.4 adds the multiverse of the answer and the reliability of an agent; a share for the specification needs
  the multiverse that measured it, and no $\text{pass}^k$ may exceed its $\text{pass}^1$.

## Chapter 18. The vault, the cryptographic seal and the audit against itself

**Objectives.** By the end of this chapter you should know how to explain why pre-registration with a hash published
before the data is a seal against adjustment after seeing the result, what the counter of attempts $K$ measures, why the
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
never saw. Opening the vault costs the one thing it had: surprise. When a vault has already been opened and the
model needs a correction, the correction is worth something only when tested in a **new vault**, which nobody saw.

**The forecast diary.** Every probabilistic forecast issued stays recorded in a continuous ledger, with
the source, the filter, whether the system can influence the outcome, how many factors the justification has and the
step of Pearl. And with a limit: **the forecast is never 0% nor 100%**, but something between "one in a hundred" and
"ninety-nine in a hundred". A certainty that fails is punished by the score, but it cannot be revised by the
evidence; with the limit, each forecaster keeps a belief that can still move and a record that can be charged.

---

### Step 2 — The rule of the game

**The risk of scale is epistemological.** When a system tests hundreds of hypotheses and specifications in
parallel, the greatest risk is not technical; it is overfitting at industrial scale, the garden of forking paths
walked by indefatigable machines. Three mechanisms contain it: the sealed pre-registration with a counter of
attempts, the vault outside time, and the forecast diary on a chained bus.

**Pre-registration with hash and counter of attempts.** Before anyone touches the test partition, the hypothesis
is frozen with a hash. Every change of criterion, of test or of data is written in a declaration, with its SHA-256
recorded **before** running. The rule does not forbid learning from a failure: a new run may be written knowing
the previous ones. What it forbids is hiding that it was. Every correction made after seeing a result stays on
record and **counts in $K$**, the number of specifications tried for the question, and the final report is obliged
to declare it.

Counting honestly is harder than it looks. In a real search for causal structure across twelve domains, the
counter per domain stopped at $K = 3$, and the search stopped there because of it. But the same transfer gate had
been tried across seven runs, and the counter per domain did not see that. Recounted across the whole gate, the
honest number was **38 distinct tests**, within a band from 32 to 72 according to the counting criterion. Both
numbers are right; they count different things. The lesson is that $K$ must be counted at the level at which the
claim is made. With a Bonferroni correction at $K = 38$, the conclusion survived, with smaller sets of causes.

A counter alone only makes the attempts visible. To make them cost something, $K$ should also penalise complexity,
by the minimum description length (MDL) or by the Occam factor, together with the correction for multiple
comparisons. A model that needed many tries and many parameters to fit has said less about the world than one that
fitted at the first attempt with few.

**What a seal proves, and what it does not.** A hash proves **integrity**: the file is the same, character by
character, as the file that was sealed. It does not prove **provenance**, who produced the content and how, and
much less **truth**. An invented record, sealed, carries a fingerprint as valid as that of an honest one. A gate
for the listening of affected groups showed the limit: it checked whether each record had a name and a transcript
file, and seven fabricated records, each with a plausible name and a transcript, opened it. Provenance needs
something else, such as a transcript signed by whoever was heard, with the hash of the signed file chained into
the record. Even then the signature says who spoke, not that what was said is true. The same holds for the garden
of forking paths: the hash alone does not stop it. What stops it is the hash written **before** the data are
seen, together with the counter $K$. Sealed after the result, the most selective analysis gets an impeccable
fingerprint.

**The vault outside time.** The future temporal partition stays in the exclusive custody of the lifecycle
specialist. No one who builds models reads it during development; it is opened a single time, to score calibration
and to verify the absence of leakage. The rule is hard and has a consequence: when a vault has been opened and the
model needs a correction, the correction is worth something only when tested in a **new** vault. One real study
opened three new vaults this way: the 2023 flights, the Toronto arrests of 2020–2021 and the Rhode Island traffic
stops.

**The forecast diary and the bus.** Each forecaster records every probabilistic forecast in a continuous ledger,
restricted to the interval $[1\%, 99\%]$. Forecasters whose Brier score loses to the base rate, or whose Kelly
capital collapses, have their weight reduced. The bus that carries the cards is **chained by hash**: each message
carries the hash of the previous one, and the trail cannot be rewritten without breaking the chain. The chain has
its own failure modes. One real diary added into a single score a forecaster that appeared in different projects
under the same name; the error was invisible in any single line and showed only when the scores were recomputed
by project.

**Risk opinions and the named officer.** The governance specialist issues a risk opinion for each project
evaluated, including the ones that never reach a decision, and a **named human officer** signs it. The signature
is recorded apart from the result files, with each opinion identified by the SHA-256 of the file at the moment of
signing, so that signing does not rewrite the outputs a clean reproduction compares. An opinion may carry a factual
caveat that the signature does not remove: a recommendation derived from a decision and a cost that were
**assumed** for the exercise is not policy advice, and the opinion has to say so.

**The limit: the auditor is a stakeholder.** An audit authorised by whoever is audited has limited value. The named
officer answers the questions an audit has to answer (who decided to use it, who approved it, who watches the
scoreboard, who switches it off), but naming an officer **does not make the audit independent**. What an honest
audit can do from inside is record what it **could not look at** and which affected groups were heard. In seven
real decisions that involved people (cutting a benefit by 10%, reducing class sizes, replacing free care with a
copayment, judging a training programme against survey controls, and three review marks on police data streams),
**none of the seven affected groups was heard**, and an elicitation with human experts was never done. A signature
on a summary is worth what it is: the declaration of whoever runs the work that they read a summary.

**Auditing your own claims.** Governance also serves to mark the points **not** measured, and then to measure them.
The method is the one of every severe test, turned inward: list the claims the work sustains but never tested,
write for each one a declaration with a hash **before** measuring, measure, and report the result with its limits.
A change of criterion after seeing the result enters as a new declaration and counts in $K$. Four such checks show
what the method finds.

| The assumption | What was measured | Result |
| --- | --- | --- |
| Whoever abandons a queue calls back with probability 0.6 | A quasi-experimental design in a real support desk, 1999 | **0.145** [0.132; 0.157], four times less than assumed |
| The tolerance of the calibration error can be fixed at 0.03 | The null simulated with the same forecasts and the same $n$ | All six binary vaults show detectable miscalibration, one of them only borderline; detectable is not the same as relevant (Chapter 8) |
| ICP may treat every row as independent | Clustered tests and block permutation, with 16 instants per volunteer and 80 schools | Pharmacology survives intact; Education drops out |
| A method that checks every bridge catches errors that common practice lets through | A blind pilot of 12 cases, two arms | A tie at the ceiling: 8 of 8 defects found in both arms, so the claim is not supported |

**The inward audit found no error of arithmetic and narrowed the reach the numbers were allowed to claim.** A later
reading from outside found what the inward audit had missed: a Kelly fraction computed with the wrong chance, a
value of information that chose the best act inside each branch, a test of invariance that let the environment move
the outcome, a market calibration that dropped a factor of 252 (Appendix E lists every one, with both values).
That is the strongest argument of this chapter, measured on the book itself: an audit run by the authors of the
claims is weaker than one run from outside, however honest the authors. One limit stays declared: the outside
readings of 2026-10-18 were made with an AI assistant, outside the authors' audit but not by people outside the
program. They are the first entries of a public errata, not the independent audit that Chapter 21 asks for.

### Step 3 — Hands on

Two tasks: build a ledger chained by SHA-256 and prove that changing **one character** in the past
brings the chain down; and compute the Merkle root of a batch of cards. The code brings SHA-256 written by hand,
without dependencies, and checks the implementation against the known vectors. Run it with
`cargo run --release --bin labs -- --chapter 14`.

**The recipe of the laboratory.** Each event of the chain is hashed as
SHA-256 of the text `session|index|card|content|previous`, with the session `sessao-h3`, the index from 0, and the
previous hash of the first event written as 64 zeros. The Merkle root pairs the hexadecimal hashes of a level in order,
hashes the concatenated text of each pair, and repeats the last hash of a level when the level has an odd count, up to a
single hash. That last rule has a known weakness, declared here: a list and the same list with its last element
repeated have the same root, so the root alone does not prove how many cards there were (the gates record the count
beside it).

```text
== Chapter 18 :: the vault, the seal and the audit ==
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

**Exercises.**

1. *(Concept)* Why does opening the vault twice destroy its value? Relate it to Mayo's severity,
   that is, to the requirement that a test could have given another result.
2. *(Concept)* A colleague proposes rewriting the result files with the signature inside. Which property
   of the clean reproduction would that break?
3. *(Count)* The counter per domain stopped at $K = 3$; the recount across the whole gate measured $K = 38$
   distinct tests. Explain, in three sentences, why the two numbers coexist without either of them
   being wrong.
4. *(Laboratory)* Extend the laboratory to a bus in which each message carries the hash of the previous one
   **and** the index, and write a verifier that detects not only the alteration of content but also the exchange of
   order of two messages.
5. *(Laboratory)* Compute the Merkle root of the batch of seven cards down to the intermediate roots and confirm
   how many hashes are recomputed when only the last card changes.
6. *(Concept)* Propose a procedure to make the audit of decisions that involve people less dependent on whoever
   runs the work.

**Summary.**

- Pre-registration with a hash is a seal against adjustment after seeing the result, as long as the hash is published
  to someone outside before the data are seen. The hypothesis is frozen before, and the published hash is the seal.
- The counter $K$ makes the attempts visible, and it must be counted at the level of the claim: 38 tests at the
  gate against 3 per domain.
- The vault outside time opens only once; a correction asks for a new vault.
- The diary scores each forecaster with a proper rule, inside $[1\%, 99\%]$, and weighs the agents by the scoreboard; the
  chain of cards is chained by SHA-256.
- Signed opinions name the officer of record, but they do not make the audit independent. None of the seven affected
  groups was heard.
- A self-audit applies the severe test to the work's own assumptions, and usually narrows the reach of its numbers.

## Chapter 19. Seven rounds in search of invariant causes, and what they found

**Objectives.** By the end of this chapter you must know how one defines an environment that *intervenes* in a variable, why a
cause only deserves that name if it survives the exchange of environments, why a demanding gate failed five times before
passing, and how one reads a result approved with a caveat. And also a result that passed by a narrow margin and then shrank.

---

### Step 1 — The real world and the paradox

**The question that keeps holding everywhere.** A team wants to know which causes keep holding when the world changes.
Not "the cause in this school", nor "the cause in this year". The cause appears in school A, in school B and in school C, in
the good year and in the bad year, in one laboratory and in another. That is the question of this chapter, and its gate is single and demanding. It asks for a non-empty set of invariant causes, in at least three different
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
and left skin colour out. The result does not say that colour does not matter, and in observational police records,
where colour can act through every other variable that was recorded, it is the reading to treat with most care. The method only promises to return a
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
| 1 | 12 convenience domains: year, region, airport as environment | 1 (law, and against the claim) | Failed |
| 2 | environments that are documented interventions ($K = 1$) | 1 (law) | Failed |
| 3 | non-parametric invariance test ($K = 2$) | 2 (health and education; health only by saturation) | Failed |
| 4 | rule that discards sets without testing power ($K = 3$) | 1 (education) | Failed |
| 5 | physical chambers with the causal graph known | 2 (education and physics) | Failed |
| 6 | gene deletions in yeast | 3 (education, physics and genetics) | Passed the rule, with a caveat |
| 7 | FDA cardiology clinical trial | 4 (the three previous ones and pharmacology) | Passed |

Read the last column together with the end of Step 2. "Passed" is the verdict of the rule as it stood in each round,
after the corrections of the rounds before it. Under the first declared version of each choice the gate does not pass,
and once the dependent rows are corrected it stands with three areas only by counting a marginal replica in genetics.

**The light tunnel.** In a physical chamber built in Switzerland to study causality (Gamella, Peters and Bühlmann,
2025), a researcher turns a knob that controls an electric current, three LEDs of different colours light up, and
light sensors measure the response. Of 1,024 possible combinations of candidate causes, the method accepted 128, and
their intersection (the set that all of them have in common) was $\{\text{red},\ \text{green},\ \text{blue}\}$. They are the
three LEDs. The light sensors stayed out, despite being strongly correlated with
the current, because they are **children** of the LEDs. It was a check of the method against a known truth, and the method got it right.

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
$25.2$ ms per ng/mL, with a 90% posterior interval of $23.0$ to $27.5$. And the hardest test came
later. On the drug that the model **never saw**, verapamil, the invariant model predicted well (skill $+0.57$). The
model that used *all* the variables exploded, with a loss of the order of $10^{15}$, because verapamil pushes one
measure of the electrocardiogram outside everything that was in the training set.

**The yeast.** With 2014 data on gene deletions in yeast, the method gave an invariant set in one
domain out of ten drawn. It rejected 1,022 of the 1,024 sets, accepted two, and the best p-value stayed at
$0.0072$, just above the threshold of $0.005$.
The validation by intervention, which checks gene by gene, did not confirm two of the three genes of the set; only the
first moved the target in the predicted direction. The output was approved **with a caveat on the record**. A replica declared
later drew 40 new genes from a set of 4,681, with a level of $0.05/40 = 0.00125$ per gene, and found one
gene that passes the strict validation, with a null rate of $p = 0.001$. It is the floor of 999 draws. The replica shows
*one gene in one mutant*, and not an effect measured with resolution. Each mutant is an environment of a single row, and the smallest
possible p-value there is $1/1,000$.

The laboratory of this chapter measures what the **second row** changes, in the same design (40 mutants, $\alpha/40 =
0.00125$ per gene) and with two planted cases: a mutant whose effect appears in the **two** rows and an artefact
whose effect appears in **one** row only. With one row per mutant, the validation approves **4 of 40** — the true mover, the
row artefact and two that are pure noise, all touching the floor of $0.001$. The p does not separate what
one looks for from what one does not look for, because all of them hit the ground at the same time. With the second row, requiring the
effect in both, **1 of 40** passes: the mover, with $p = 10^{-6}$, which is the new floor; the artefact falls out because the
effect of it in the second row is $+0.028$, against $+1.016$ of the mover. What was missing was a row, not a smaller p-value.

**The fall of education.** Education was the most comfortable area. They were small classes drawn in the STAR experiment of
Tennessee, 11,598 students. When the students stopped being counted as 11,598 independent observations and
started being counted as **80 schools**, the accepted sets had no variable in common, and the
invariant set became empty. That does not mean that a small class has no effect. It means that, with 80
true units, the test lost the power to point to a set, which is the only product that it delivers.

**Five failures, three named causes.** The gate failed five times in a row, and three of those failures have a
name: using the year or the region as environment invalidates the premise; a code error was hidden in the
invariance test of round 2; and saturated sets passed as evidence until the saturation rule of round 4 removed
them.

---

### Step 2 — The rule of the game

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

**Where it fails.** Three situations recur, and each one became a rule.

1. **The environment acts directly on $Y$.** If the year changes the outcome by itself, then $Y \perp E \mid X_S$ is false for every
   $S$. The method rejects everything, and $\hat{S} = \emptyset$. That was the case in the 6 areas of the first round.
2. **The environment is a deterministic function of the variables of $S$.** In that case the condition holds *by construction*.
   Conditioned on $X_S$, the environment is constant, and the test has zero power. A test without power certifies nothing;
   accepting a saturated set is not evidence. The **saturation rule** started to exclude those sets from the
   family. They count neither as accepted nor as rejected. The measure is the hit rate of the nearest neighbour,
   leaving the row out (1-NN LOO), when predicting the environment from the columns of $S$; mean hit rate $\ge 0.99$
   means saturated. The distance is Euclidean on the columns standardised to mean 0 and sd 1, and a row whose
   columns are tied with other rows scores the share of the tied rows that share its environment. (The text did not
   say this until 2026-10-19; a reimplementation that read a factor as a plain number at distance 1 found only 11 of
   the 16 saturated sets of the laboratory.) In the RAND insurance experiment, the copayment is constant inside each regime of
   plan. The set that depended on it left in round 4, as the declaration foresaw.
3. **The rows are not independent.** 16 instants per volunteer in the cardiac trial, up to 403 students per school
   in STAR, 225 thousand flights in 364 days. Treating dependent rows as independent narrows the margin. The correction
   was the Wald clustered by volunteer or by school and the permutation of whole blocks.

**The code failure that a reading found.** In round 2, before any execution, a reading of the code found an
error in the invariance test. The degrees of freedom were counted by the number of **columns** of the design matrix in
place of its **rank**. With environments that are interventions, a candidate is usually constant inside the environment, and
those collinear columns added degrees of freedom without reducing the sum of squares. The test became permissive and
accepted too much. The correction started to use the rank and gained a test of its own. Under the null hypothesis of invariance the
test rejects 7% of the times (the goal was less than 9%), and under the hypothesis of a break, 100%. The re-execution showed that the
error had **not** changed any result of the first round.

**The two rulers.** The first ruler was a linear gaussian test: nested F against coefficients per environment,
plus Levene of the residuals. With large samples it rejects any deviation from the linear model, such as mass at zero,
concave response, heavy tail, and in simulation it rejected from 72% to 100% of the times even when the law was
invariant. The second ruler (from round 3 on) fits a single pooled model, standardises the residual by the scale
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
of the 1,741 rows, in the direction of the positive coefficient $+0.053$. The null rate was fixed afterwards, by permutation of the
label of the mutant among the 1,741 rows, with 999 draws and seed 20261015. The observed is 1 pair in 27, the null mean is 0, and $p = 0.001$, the floor of the test. The
reading has two halves, and they point to opposite sides. Against the chance of the label the evidence is strong. Against reading that as a measured effect, the evidence is null. Almost all of the p comes from the
half that is a fact of the data. Deleting a gene removes its expression, so the minimum of the column is on the row of the
mutant, and permuting the label destroys exactly that half. The half that remains, the target outside the range of the other
1,740 rows, on its own gives $p = 0.031$, marginal.

**What invariance buys.** In verapamil, the drug that the model never saw, the invariant model predicted with
skill $+0.57$ and conformal coverage $0.89$. The model fitted by the key did a little better, $+0.65$.
Predicting well inside the training set and failing under a new intervention is the failure mode that invariance exists to
avoid.

**Invariance as a test of understanding.** The same logic tests a model of another mind. A system that answers
false-belief questions correctly (where will Sally look for the marble that Anne moved while she was out?) may have
the mechanism, an account of what Sally saw, or only the form of the stories it has met. The test is an environment
that changes a detail the mechanism says is irrelevant and one it says is decisive. Language models that pass the
classical versions failed when the container was made transparent, so that Sally could see the move, or when she was
told about it (Ullman, 2023): the answer stayed the same where the mechanism says it must change. A model that keeps
its prediction under an irrelevant alteration and changes it under a relevant one has passed an invariance test; one
that does the opposite has learned the surface. Average accuracy on the familiar versions cannot tell the two apart,
which is why the failures on the altered cases weigh more than the successes on the usual ones.

**The multiplicity, counted.** The honest counter of the gate is of **38 distinct tests**, between 32 and 72 according to
what one counts. The inventory of the path counts still **eleven vault openings**, that is, eleven times in which a
result was seen before a design decision. With Bonferroni over 38, the level per test falls to $\alpha = 0.05/38 = 0.0013$, at the cost of
9.6 bits of parsimony, and the gate still passes in four areas. The sets shrink: in STAR,
$\{\text{small\_years\_before}, \text{small\_class}\}$ becomes $\{\text{small\_class}\}$; in yeast,
$\{\text{plate}, x_1, x_5, x_9\}$ becomes $\{\text{plate}, x_5\}$, and $x_1$ leaves, the only gene that the validation by
intervention confirmed.

**The stopping rule, declared after the path.** Bonferroni over 38 divides $\alpha$ by a number the path chose, and
"run another round until three areas pass" is a stopping rule that was never declared. An **e-process** closes that
gap: each test contributes an e-value (here, a calibrated form of its per-domain p-value), the evidence accumulates
by a rule fixed before looking, and by Ville's inequality the chance that it ever reaches $1/\alpha$ under the null is
at most $\alpha$, whatever the stopping rule. Applied to the 38 recorded tests under a declaration written before the
computation (one ledger per area, the mean of the e-values because the rounds reuse data, a test with no non-empty set
counting as a p-value of 1), the reading is this:

```text
== The H2 gate under the e-process, retroactively ==
38 distinct tests as recorded, 2 clustered rows admitted by the declaration | alpha = 0.05 | threshold 1/alpha = 20 | merge: mean within each area | gate: 3 areas

-- reading: as recorded --
  area             tests claims         mean e  verdict
  Agronomia            1      0         0.5000  does not
  Biologia             2      0         0.5000  does not
  Clima                1      0         0.5000  does not
  Crédito              2      0         0.5000  does not
  Desenvolvimento      1      0         0.5000  does not
  Direito              5      3         2.3749  does not
  Educação             5      2      4414.5557  crosses 1/alpha
  Energia              1      0         0.5000  does not
  Engenharia           1      0         0.5000  does not
  Epidemiologia        2      0         0.5000  does not
  Farmacologia         1      1 1309803451.4320  crosses 1/alpha
  Física               1      1 1309803451.4320  crosses 1/alpha
  Genética             5      1         6.9919  does not
  Logística            1      0         0.5000  does not
  Saúde                5      0         0.5000  does not
  Trabalho             2      0         0.5000  does not
  Transporte           2      1         2.9927  does not
  areas that cross: 3 (Educação, Farmacologia, Física) | gate under the e-process: PASSES

-- reading: with dependent rows corrected --
  area             tests claims         mean e  verdict
  Agronomia            1      0         0.5000  does not
  Biologia             2      0         0.5000  does not
  Clima                1      0         0.5000  does not
  Crédito              2      0         0.5000  does not
  Desenvolvimento      1      0         0.5000  does not
  Direito              5      3         2.3749  does not
  Educação             6      3      3678.8880  crosses 1/alpha
  Energia              1      0         0.5000  does not
  Engenharia           1      0         0.5000  does not
  Epidemiologia        2      0         0.5000  does not
  Farmacologia         1      1 1309803451.4320  crosses 1/alpha
  Física               1      1 1309803451.4320  crosses 1/alpha
  Genética             5      1         6.9919  does not
  Logística            1      0         0.5000  does not
  Saúde                5      0         0.5000  does not
  Trabalho             2      0         0.5000  does not
  Transporte           2      1         2.9927  does not
  areas that cross: 3 (Educação, Farmacologia, Física) | gate under the e-process: PASSES
```

Three areas cross: education, pharmacology and physics; genetics does not, with a mean e-value of 7 against the 20
required. Replacing education's published set by its clustered p-value of 0.76, as the declaration admits, does not
move its verdict, because the mean also carries the two earlier rounds on the same data, which the declaration left
as recorded. The e-process answers a different question from the clustering correction: it asks whether the path of
rounds was allowed to stop where it stopped, and says yes for three areas, not for genetics. It does not repair the
rows of a test, and the clustered reading of the paragraph above stands beside it.

**Two caveats that weigh more than the correction.** The first: dividing $\alpha$ is not monotone in the ICP. With a lower
level, domains that rejected everything start to accept some set, and invariant sets appear in models
almost rejected. The second: the approval depends on the version of the procedure. With its first version reconstructed, STAR is
pruned before the invariance test, since the key is constant inside the environment, and the last round drops to
three areas, one of them the fragile genetics. And if only the first declared version of each choice counted, the gate would not pass, and
round 1 gives a single area.

**The final result, with dependent rows corrected.** Pharmacology survived intact the two clustering corrections. The set
continues to be $\{c_{\text{dof}}, c_{\text{qui}}, c_{\text{ran}}\}$, and it shrinks the
evidence against the empty set, from $6.3\times10^{-104}$ to $2.2\times10^{-15}$ in the clustered test and
$0.002$ in the block permutation, from about 340 bits to 49 and 9. Education did not survive. With the 80 schools
as units, they are 266 accepted sets of 512 in the clustered test and 512 in the permutation, without any variable in
common, and the published set $\{\text{small\_years\_before}, \text{small\_class}\}$ has $p = 0.76$. Even
before the clustering correction, education was already the fragile area, with the largest p among the non-saturated sets
at $0.075$, close to the threshold, and $\{\text{small\_class}\}$ alone was rejected ($p = 0.010$). The counting of
areas closes like this. Education leaves, physics stays as it was, pharmacology stays, and genetics stays only by the
replica of one gene in one mutant, whose evidence, once the half of the p-value that the deletion produces by itself is
removed, is $p = 0.031$, marginal, and one pair among 27. The gate stays standing with three areas only if that
marginal replica is counted.

Read the three areas for what they are. Physics and pharmacology recovered causes that were known before the test:
the chamber was built with the LEDs as the parents of the current, and the trial gave the drugs whose block of hERG
was established. That is a check of the method against a known truth, which is valuable and is not a discovery.
The only area where the method could have found something new is the fragile one. And the gate itself was reached
after seven rounds, eleven openings of vaults and 38 counted tests, with the first declared version of each choice
failing it. Every step was sealed before it ran, which makes the path visible; it does not make the path
innocent. A gate fixed in advance ("three areas") becomes a target, and seven rounds aimed at it are a garden of
forking paths walked in public.

**The effect transports better than the set.** Still with the ICP rejected, the fitted model predicted the environment
never seen with positive skill in **9 domains**, from $0.04$ to $0.88$. The strong claim ("holds in any
school, airport or region") falls; the direction and the size of the effect survive. To decide, it is that weak version
that matters.

---

### Step 3 — Hands on

The laboratory of this chapter is a minimal ICP, with the two rulers of Step 2 in a reduced version: least squares
by normal equations, residual standardised by the scale estimated without looking at the environment, Kruskal–Wallis of the location and
Brown–Forsythe of the scale, p-value by 999 permutations of the environment labels, Bonferroni over the two tests, and the
saturation rule measured by the 1-NN leaving the row out. Run it with
`cargo run --release --bin labs -- --chapter 19`.

There are three simulated worlds, all with three environments of 600 rows each, seed 2019, and four candidates: $x_1$
(hours of study, true cause), $x_2$ (hours of sleep, true cause), $x_3$ (pure noise) and $x_4$
(transport to school, a variable that the environment shifts and that **does not** act on the score). The outcome is
$Y = x_1 + 0.8\,x_2 + \text{noise}$.

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
p-value of location touches the floor of the permutation, $0.0010 = 1/1,000$, because the shift that the school makes in
$x_1$ and $x_2$ leaves a mark on the mean of the residual. Only the sets that contain **the two** true causes are
accepted, 4 of the 16, and the intersection of them is $\{x_1, x_2\}$. The 1-NN column shows why none of them is saturated.
The hit rate of the nearest neighbour when guessing the school from the variables of the set stays between $0.74$ and $0.87$,
far below the threshold $0.99$.

**In world B**, the environment adds a direct shift to the outcome, and **none** of the 8 sets passes. The
invariance does not exist to be found, and the gate fails honestly. It is the simulation of the first round.

**In world C**, the addition of $x_5$, a column constant inside each school and different between them, like the
copayment of a health plan, changes everything. Sixteen of the 32 sets are saturated, and sixteen of those are
accepted. Accepting is not evidence, because the test has no power at all there, and the 1-NN hit rate of $x_5$ is $1.000$.
Without the saturation rule, the set $\{x_5\}$ enters the family, the intersection with $\{x_1, x_2\}$ comes out empty and the
method returns $\hat{S} = \emptyset$, the opposite of the truth, with a face of rigour. With the rule, the family goes back to having 4
accepted sets and the $\hat{S}$ is $\{x_1, x_2\}$. That is what happened in the RAND insurance experiment in
round 4, where health left the scoreboard by saturation.

**The second row, measured.** The same laboratory brings the design of the validation by intervention of the yeast: the two
planted cases — the mutant that moves in the two rows and the artefact that moves in only one — and the two arrangements
run over the same world. Run it with `cargo run --release --bin labs -- --genetics`.

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
are what one looks for: the true mover and the row artefact reach the floor of $0.001$ through the same door, and two
mutants without any effect enter with them, because at the floor the test has no resolution to separate. It is the exact measure
of what the replica already said in prose: *one gene in one mutant*. In the arrangement of **two rows**, the effect has to appear
in both, and the list shrinks to one name. The artefact does not fall because of a large p-value — the effect of it in the first row is
$+0.607$, larger than the one of the mover in the same row —, it falls because in the second row the effect is $+0.028$, which is nothing.
The floor of the design, which was the ground of $1/1,000$, becomes the product of the two, $10^{-6}$, and the estimate of
each effect stops being a point. **The p was never the problem; the row was the one that was single.**

**Exercises.**

1. *(Concept)* Explain, with world B of the laboratory, why using the year as environment makes the invariant set
   come out empty. Which premise of the ICP does the year violate?
2. *(Concept)* In world C, without the saturation rule the method returns the empty set. Write in three sentences why
   accepting a saturated set is not evidence of invariance.
3. *(Count)* In the laboratory, the floor of the permutation with 999 draws is $1/1,000 = 0.001$. What is the smallest p-value
   possible with 9,999 draws, and why does that matter for the validation of the yeast, where each mutant is an
   environment of one row? Check the account in the two-row arrangement: if each row has the floor $1/1,000$ and the
   combined one is the product, where does the floor of the design with two rows sit?
4. *(Laboratory)* Change, in the laboratory, the shift of $Y$ of world B from $[0; 6; -4]$ to $[0; 1; -0.5]$
   and run again. Does the gate keep failing? What does that say about the power of the test?
5. *(Concept)* The multiplicity correction shrinks the sets and still leaves four areas. Why can it also
   create invariant sets in domains that rejected everything, and why is that a caveat?
6. *(Count)* With 38 tests corrected by Bonferroni, the level per test is $0.0013$ and the cost of parsimony is
   9.6 bits. The evidence of the yeast is worth 10.9 bits. By how many bits does it pass? And if the cost rose to 12 bits,
   how many areas would remain — and would the gate stay standing?
7. *(Concept)* Design four versions of a false-belief question: the classical one, one with an irrelevant change
   (the marble becomes a key), and two with a relevant change (the box is transparent; Sally is told). Which answers
   must stay the same and which must change? Which pattern of answers would show the surface without the mechanism?

**Summary.**

- The gate asked for a non-empty invariant set in three distinct areas; the first round gave one, and
  against the claim, because the year and the region act directly on the outcome.
- The gate only passed in the sixth round, with a caveat recorded in genetics, and the five failures stayed in the counter
  $K$: convenience environments, a code error in the test, the exchange of ruler and the saturation rule. Under the first
  declared version of each choice it does not pass at all: seven rounds aimed at a gate fixed in advance are a garden of
  forking paths walked in public, visible because it was sealed, not innocent because of it.
- Two rounds checked the method against known truth. The light tunnel recovered the three LEDs and the trial of the
  FDA recovered the three drugs that block hERG, with verapamil predicted at $+0.57$.
- With dependent rows corrected, education leaves; the gate stays standing with three areas only by counting a marginal
  genetic replica, and two of the three areas recovered causes that were already known. The effect transports better than the set. The 9 domains predicted the new environment
  with skill from $0.04$ to $0.88$.
- A model of another mind is tested like a cause: by alterations that its mechanism declares irrelevant and others it
  declares decisive.

## Chapter 20. The live clock, the leakage canary and the police in the vault

**Objectives.** By the end of this chapter you must know what a datum that does not exist yet is, why a test that
never found anything needs a canary to be worth something, how the Goodhart law is measured with a
randomised instrument, and why fixing the calibration of a model does not create the resolution that it does not have.

---

### Step 1 — The real world and the paradox

**Six clocks switched on at the same time.** A replay ran six real data streams as if they were
happening now: the electricity demand of the Victoria region, every half hour; the calls of a
bank, every five minutes; the daily return of the New York stock exchange; the delays of aircraft in New York; the
chance of a hurricane gaining strength; and the decision to release a detainee in Toronto. In each stream, each fact is an
**event with a timestamp**, that is, the time at which it becomes knowable. A flight that departs at 10h and arrives late only
"exists" for the system when the delay is published; a forecast issued at 8h cannot use that datum, because
it had not happened yet.

The warehouse refuses four things, and refusing is its job: reading the future, opening a vault before the written
pre-registration, opening the same vault twice and turning the clock back. In the replay of the research, over
453,973 forecasts, none used future state; **zero violations**, with the count made forecast by forecast. Six of the
streams are rebuilt in Step 3 from public data, and the numbers of this step are theirs wherever they exist; the arrests
of 2020 and 2021 and the traffic stops of Rhode Island have no copy beside the book, and their numbers come from the
record of the research, said each time.

**The canary paradox.** "Zero violations" looks great, and it is almost nothing. It is only worth something if the test *could* have
found a violation. That is why the system planted a **canary** on purpose. It is a column identical to the label,
published at that instant, when the label becomes knowable. With the honest clock, that column appears in
no emission, and a forecaster that uses it gains nothing, with **gain 0.000**. Clock switched off, the positive
control, the column yields a skill gain of $+0.91$ in the flights of 2013, $+0.97$ in those of 2023 and $+0.92$ in the
arrests of 1997 to 2002; the record measured $+0.55$ in the arrests of 2020 and $+0.97$ in Rhode Island. It is the severe test of Mayo applied to the test
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
mean distance between the predicted and the observed, measured bin by bin. The table below gives the six rebuilt
streams; the story of the three attempts follows it.

| Stream | What it predicts | Vault | Calibration in the vault | Gate |
| --- | --- | --- | --- | --- |
| Energy (Victoria) | demand of each half hour of tomorrow | 2014, 17,424 | coverage of 90% = 0.886 | Passes |
| Call centre (bank, 2003) | calls per 5 minutes of the next working day | Sep–Oct, 6,422 | coverage 0.884 | Passes |
| Market (Dow Jones) | 99% value at risk of the next session | 2000–2023, 6,011 | violations 1.53%; Kupiec p = 0.00; clustered (Christoffersen p = 0.02) | Fails |
| Flights (New York, 2013) | chance of a delay above 15 minutes, 2 h before | Oct–Dec, 82,609 | bias −0.0171; ECE 0.0260 | Passes |
| Cyclones | chance of gaining 30 knots in 24 hours | 2015–2022, 3,813 | bias +0.0002; ECE 0.0121 | Passes |
| Prisons (Toronto, 1997–2002) | chance of release | 2001–2002, 1,488 | bias +0.0165; ECE 0.0354 | Fails |

**The design lesson of the first failures.** In the prisons, the calibration error was 0.0354, for a tolerance of 0.03,
with only 1,488 cases, and yet the model beat the climatology (skill $+0.0812$). Being better than the rival and being
right are different things, and a gate that only compares the model with the rival lets the miscalibration through.
In the market, the value at risk was crossed in 1.53% of the sessions, just above the band, and the crossings came in
clusters; the frequency alone would hide the second failure, which is the more dangerous one.

**Second attempt: fix the level.** The correction was a continuous recalibration of the intercept, declared before:
after each arrival becomes known, $\delta \leftarrow \delta + 0.002(y - p')$. In the flights of 2013 it did what it
promises on the level, taking the bias from $-0.0171$ to $-0.0015$, and it cost the gate: the ECE rose to 0.0322. A
correction of the mean is not a correction of the shape. Its real test was a new vault, the same quarter of 2023. There
the model, trained on August and September, overestimated a quarter with fewer delays (bias $+0.0633$, ECE 0.0646) and
failed; with the correction it ended at $+0.0025$ and 0.0177, and passed. The correction was decisive, and it was
tested where nobody had looked.

In the prisons, the new Toronto data (from the record of the research), from 2020 and 2021, with 65,276 arrests and a vault in the second half of 2021
over 16,933 cases, changed the question. It became registering at the police station instead of releasing. The model
hit the mean (0.0002) and separated the cases well, but it was **underconfident**. In the lower bins it predicted 0.12
where the observed rate was 0.04, and in the upper ones it predicted 0.82 to 0.90 where the rate was 0.91 to 0.94. It is an error of
**slope**, and touching the intercept does not reach that.

**Third attempt: Rhode Island** (from the record of the research). A new specification was declared before opening the vault: a Platt recalibration
that adjusts intercept *and* slope, and tested it in a new vault, with 24,466 stops of the state police of Rhode
Island, between 2013 and 2015, and an arrest rate of 2.26%. The question changed from "release after arrest" to
"arrest in a traffic stop", and that exchange was confirmed by a human.

| Vault 2013–2015 | Bias | ECE | Skill against the base rate |
| --- | --- | --- | --- |
| Official forecast (with Platt) | 0.0020 | **0.0048** | $+0.019$ |
| Raw model, in shadow | 0.013 | 0.013 | $+0.004$ |

It passed, with an ECE of $0.21\,\bar{y}$ against the limit of $0.3\,\bar{y}$. The vault had that criterion relative to the
rate, and the raw model, which overestimated the rate by **58%**, would have failed. That is what shows the value of Platt. The change of ruler, from an absolute tolerance to one
relative to the rate, came after two failures and counts as one more attempt; it is a tightening, not a loosening. With
the rate near 2 in 100, the absolute tolerance of 0.03 is larger than the rate itself, and under it the raw model would
have passed too. The
recalibration ended with slope 1.07 and intercept $-0.30$, and the circuit breaker put the reserve as official in 12%
of the vault. Two caveats remain, and the first is the most important. **The skill is small** ($+0.019$). Arrest
in a traffic stop is a rare event (2 in every 100 stops) and little predictable from the available variables.
The calibration is good, and the **resolution** is not fixed by recalibration. It is the decomposition of Murphy in real data.
The recalibration fixed the calibration, and could not create the resolution absent from the data.

**Who pays for the error.** The fairness audit was informative in the three prison streams, and the pattern was not
the same in the three. In Toronto, from 1997 to 2002, the marker flagged released Black detainees about **twice** as
often as released White ones in both years of the vault (0.155 and 0.164 against 0.070 and 0.077), in a model that
**does not use colour**; it enters through the correlates. In the record, in Toronto in 2020 and 2021, the base rates stayed close and the calibration, the false positives and the
false negatives stayed under 0.05 between the groups. In Rhode Island, the arrest rate of Black drivers
was double the one of Whites (3.0–3.8% against 1.4–1.7%), and the false positive rate was almost double in 2013 and 2014
(0.019 against 0.011; 0.025 against 0.013), again in a model blind to colour. Not using colour in the
model does not give parity, because colour enters through the correlates.

---

### Step 2 — The rule of the game

**The clock.** Each fact carries its instant of availability, written $t_{\mathrm{disp}}$. The warehouse only
delivers a row if $t_{\mathrm{disp}} \le$ instant of the request, and each agent reads only the window between the last
tick and now. Before accessing the vault, the specification is frozen in a pre-registration hash, and the vault opens one
time per stream and does not open again. The row groups of the file on disk have the size of a tick, and that is why
each table is swept at most 3 times in the whole replay, measured at between 1.53 and 2.35 times across the
replays.

**The rules of the calibration gate.** For a binary forecaster, in the vault:

$$|\bar{p} - \bar{y}| \le 0.02, \qquad \mathrm{ECE} \le 0.03, \qquad \text{Brier skill} > 0 .$$

The Brier skill compares the score of the model with the one of the climatologist, who always repeats the rate of the past:
$1 - \text{Brier}_{\text{model}}/\text{Brier}_{\text{climatology}}$. Zero means that the model knows nothing
beyond the base rate; the reference is a **moving** climatology, that is, the base rate of the past available at the instant of the
emission. The expected calibration error is the mean distance between the predicted and the observed, weighted by the size
of each bin:

$$\mathrm{ECE} = \sum_{k=1}^{10} \frac{n_k}{n}\, \left| \bar{p}_k - \bar{y}_k \right| ,$$

with 10 bins of equal mass. For the quantile streams, the criterion is the coverage of the 90% interval staying between
0.86 and 0.94; for the value at risk of the market, the frequency of violations staying between 0.5% and 1.5% and the Kupiec test
giving $p > 0.05$. Abstaining does not take the forecast out of the scoreboard, because when the model is refused the reserve is scored in
its place. Without that rule, a model could improve the score simply by giving up on the difficult days.

**The Murphy decomposition, in one sentence.** The mean squared error separates into reliability (how much the
forecasts missed the frequency), resolution (how much they knew how to separate the cases) and uncertainty of the outcome
(how rare and unpredictable the event is). The Platt recalibration only moves the **reliability**. The resolution lives
in the data and in the variables, and no output correction invents it. That is why the verdict of Rhode Island is
"good calibration, low resolution", and that is why it is honest. $\mathrm{REL} - \mathrm{RES}$ comes out positive, but the
final skill is $+0.019$, far from the $+0.05$ that had been asked for. The stream passed the calibration gate and
missed its own target of skill, and the two verdicts are reported side by side; neither cancels the other.

**The canary as a severe test.** Mayo asks that a test be capable of erring. The canary is the operational version
of that request, a column that *is* the label, but only becomes available when the label becomes available. With the
honest clock it enters no emission, and the gain is 0.000; with the clock switched off, the gain is from 0.55 to
0.97. The value of the leakage test is in its positive control.

**The performative loop, declared and then measured.** None of the real streams records the actions of the system,
so the loop was simulated over the real arrivals of the call centre. The staffing comes out of the forecast, whoever abandons redials with
probability $r$, and the redials enter the observed count. With $r = 0.6$ declared, the sensor detects the
dependence ($t = -6.4$, $p = 1.5\times10^{-10}$); with $r = 0$, the sensor does not detect it ($p = 0.70$). The parameter
declared was a guess. Measured later in a real call centre of 1999, a related quantity, the increase in the chance of
calling back within fifteen minutes when a client abandons instead of being served, is worth $0.145$ [0.132; 0.157].
It is not the same parameter as the declared redial probability (Chapter 11), but it does not support its size. The correction was, therefore, tested in a world
more hostile than the observed world, and that asymmetry weakens the correction as proof. And the estimator of the loop had
**four versions** before passing, all recorded: bias of $+4.7\%$, $-5.9\%$, $-8.4\%$ and $+1.7\%$,
for a limit of 3%. Only the fourth passes. The first fails by design, and not by account, because staffing by the corrected demand
feeds the spiral of abandonments. It is estimator adjustment after seeing the result, the weakest point of
that gate, said without decoration.

**The board against the scoreboard.** Three attacks were tried and neutralised. The forecaster that abstains in the uncertain
zone receives the score of the reserve and stays behind the honest forecaster in the three binary streams; the forecaster constant
at the base rate receives weight 0 in the diary; and the second opening of the vault is refused. The Goodhart index, skill in the
development minus skill in the vault, was reported per stream in the record of the research, and it is small: $+0.14$ in energy, $+0.02$ in the
flights, and negative in the others, where the vault came out above the development.

**Privacy in operation.** The k-anonymity of the prison dataset is 1. In that dataset, 11% of the rows are unique
in the quasi-identifiers, and 2% stay unique with age in bands of 5 years; no released cohort exceeded the budget
$\varepsilon = 2$ per person, and the privacy circuit breaker removed the cohort of 1997 from the publication of 2001.

**The calibration tolerance, again.** Chapter 8 sets the ECE of each binary vault beside the null of its own
forecasts: all of them show detectable miscalibration, because in the large vaults the noise is a few thousandths and
0.03 lets detectable miscalibration through. Detectable is not the same as relevant, and the gate keeps both readings.
By the secondary rule, which requires $\max(0.03;\ \text{95\% quantile})$, the picture goes back to what it was: the
vaults approved by the fixed tolerance stay approved and the two from Toronto stay failed. The failure of Toronto of
1997–2002 sits on the edge: above the null of its real forecasts, below the null that simulated forecasts of the same
size give, so it is reported as a borderline failure, not as signal.

**The label drift that did not warn** (from the record of the research). Would two classical tests of label drift have seen the deviation of 2013 before the original sensor. They did not. The test that compares the realised rate with the predicted
mean switched on in the vault on 6 November 2013, and the original sensor had already switched on 16 October; worse, it
stayed on in 95% of the checks of the 2013 development, because with a window of three thousand flights the delay rate oscillates more than
0.02 from one day to the next. A permanent alarm warns nothing. The correction held the only real
deviation that appeared, in the third quarter of 2017, when the raw model marked $+0.028$ and the corrected one,
$+0.001$. But the declared stress did not discriminate, because the daily readjustment absorbs the drift alone at the level of the
quarter. The raw model also passed. That is stress, not proof.

**The inverted audit, and who pays** (from the record of the research; the rebuild of Step 3 audits only the two
years of its vault). The question "who pays for the error?" was run inside out in the three prison streams. The **chosen** metric is the false positive rate of the marker,
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
arrest" to "arrest in a traffic stop", and the exchange was confirmed by whoever ran the analysis, and not by the
group that became the target of the decision. The second: the record of the affected groups is short and is not good, because in
seven decisions that involve people **no affected group was heard**. Chapter 21 returns to that point.

---

### Step 3 — Hands on

The laboratory of this chapter assembles the three pieces on which the replay depends, the Platt scaling, the
expected calibration error and the canary, over simulated forecasts. The logistic regression is fitted by
reweighted least squares, which is deterministic and needs no library. Run it with
`cargo run --release --bin labs -- --chapter 20`.

**The recipe of the laboratory.** Each stop has three binary covariates, night (chance 0.18), speed (0.28) and
plate (0.12), and is an arrest with chance $\sigma(\beta_0 + 0.8\,\text{night} + 1.0\,\text{speed} + 0.5\,\text{plate})$, with
$\beta_0 = -4.005$ in the 120,000 training stops and $-4.42$ in the 58,000 development and the 24,466 vault stops. The
raw model is a logistic regression fitted by reweighted least squares on the training stops, with a ridge of $10^{-4}$
outside the intercept; Platt is the logistic regression of the development outcome on the logit of the raw forecast;
the skill is against the development rate, held fixed. The canary is the label itself with 1% of its values flipped,
added as a fourth covariate: absent (all zeros) under the honest clock, present in training and vault with the clock
switched off; the skill gain is that of the model with the canary over the raw model.

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
official model with Platt (slope = 0.98, intercept = -0.54): predicted mean 0.0222
  CITL = +0.0002 | ECE = 0.0022 | Brier = 0.02130 | skill = +0.0098
climatology fixed at the past rate: Brier = 0.02151 (p(1-p) = 0.02171)

canary: a column equal to the label, published at the instant of the label
  clock honest: canary available in 0 of 24466 emissions | beta_canary = +0.00 | skill gain = +0.0000
  clock switched off: canary available in 24466 of 24466 emissions | beta_canary = +9.23 | skill gain = +0.6521
```

Read it line by line. **The raw model** predicts, on average, 0.0351 where the rate of the vault is 0.0220, that is, 60% above.
That excess appears in the two metrics at the same time, since the bias is worth $+0.0132$ and the expected
calibration error is worth 0.0132, and the deviation is almost all of **level**. The model learned the base rate of the old
period and did not notice that the world changed. The Brier skill comes out negative ($-0.0020$), since the model loses
to the climatologist that only repeats the rate of the past. **The official model**, with Platt (slope 0.98, intercept
$-0.54$), goes back to the right level, with bias $+0.0002$ and ECE 0.0022, and skill $+0.0098$. Note the point at which
the correction did **nothing**. The skill rose from $-0.0020$ to $+0.0098$, and not to $+0.5$. The slope
hardly changed (the fit stayed at 0.98), and the resolution continues to be the one of the data. The calibration is fixed;
the resolution is not. **The canary** closes the laboratory. With the honest clock, the canary column becomes available in
**0** of the 24,466 emissions, its coefficient stays at zero and the skill gain is 0.0000; with the clock
switched off, it is available in the 24,466 emissions and the gain is $+0.6521$.

**The real streams.** The six streams of the table of Step 1 run beside the book, on public data, with a specification
declared before the run; the instrument prints its own numbers, and those are the numbers Step 1 teaches with. Run it
with
`cargo run --release --bin labs -- --streams`.

**The recipe of the rebuild.** Nothing is drawn at random. *Energy* and *bank*: the point forecast of a slot is the
mean of the same slot over the last four days of the same type (working days, and the rest), and the 90% interval
multiplies it by the 5% and 95% quantiles of the relative errors of that rule over the previous 56 days (energy) or 20
working days (bank). *Market*: the 99% value at risk of a session is the 1% quantile of the last 250 daily returns.
*Flights*: a logistic regression, with a small ridge, on the carrier, the airport, five bands of departure hour, the
weekday, the log distance and the share of delayed arrivals from the same airport known in the three hours before
the emission, fitted on the 60 days before 1 October and issued two hours before each departure of October to
December; a flight that was cancelled or diverted has no arrival and leaves the count; the corrected forecast moves its
intercept after each known arrival by $\delta \leftarrow \delta + 0.002(y - p')$. *Cyclones*: a logistic regression on
the wind, its change over the last 12 and 24 hours, the pressure, the position and the season, at every observation
with another one 24 hours later. *Toronto*: a logistic regression blind to colour (employment, citizenship, prior
checks, age and sex), fitted on 1997 to 2000; the marker flags a forecast of release below 0.70. The climatology is
always the rate of the labels knowable at the emission.

```text
== Chapter 20 :: the real streams, rebuilt beside the book ==
no step draws a random number | gates: |CITL| <= 0.02, ECE <= 0.03, skill > 0 | coverage 0.86 to 0.94 | VaR 0.5% to 1.5% and Kupiec p > 0.05
energy (Victoria 2014, day ahead, 90% interval):  n =  17424 | coverage 0.886 | gate PASSES
bank (Sep-Oct 2003, next working day, 90%):       n =   6422 | coverage 0.884 | gate PASSES
market (Dow Jones 2000-2023, 99% VaR, 250 days):  n =   6011 | violations 1.53% | Kupiec p = 0.00 | Christoffersen p = 0.02 | gate FAILS
flights 2013 (Oct-Dec), trained on 54835 flights of the 60 days before:
   raw model:             n =  82609 | rate 0.2214 | CITL -0.0171 | ECE 0.0260 (null q95 0.0046) | skill +0.0898 | gate PASSES
   intercept correction:  n =  82609 | rate 0.2214 | CITL -0.0015 | ECE 0.0322 (null q95 0.0046) | skill +0.0908 | gate FAILS
   canary: honest clock available in 0 emissions, skill gain +0.0000 | clock switched off, skill gain +0.9102
flights 2023 (Oct-Dec), trained on 68257 flights of the 60 days before:
   raw model:             n = 104091 | rate 0.1273 | CITL +0.0633 | ECE 0.0646 (null q95 0.0040) | skill +0.0282 | gate FAILS
   intercept correction:  n = 104091 | rate 0.1273 | CITL +0.0025 | ECE 0.0177 (null q95 0.0034) | skill +0.0747 | gate PASSES
   canary: honest clock available in 0 emissions, skill gain +0.0000 | clock switched off, skill gain +0.9718
cyclones (2015-2022, +30 kt in 24 h), trained on 12585 observations of 1975-2014:
   model:                 n =   3813 | rate 0.0462 | CITL +0.0002 | ECE 0.0121 (null q95 0.0107) | skill +0.1000 | gate PASSES
Toronto (2001-2002, chance of release, model blind to colour), trained on 3738 cases of 1997-2000:
   model:                 n =   1488 | rate 0.8253 | CITL +0.0165 | ECE 0.0354 (null q95 0.0315) | skill +0.0812 | gate FAILS
   canary: clock switched off, skill gain +0.9188
   marker (chance of release below 0.70): false positive and false negative rates by perceived colour
   2001 Black  FPR 0.155  FNR 0.667  (311 cases)
   2001 White  FPR 0.070  FNR 0.772  (900 cases)
   2002 Black  FPR 0.164  FNR 0.500  (85 cases)
   2002 White  FPR 0.077  FNR 0.838  (192 cases)
```

Read it line by line. **The interval streams pass**: 0.886 of the half hours of Victoria and 0.884 of the five-minute
slots of the bank fall inside their 90% intervals. **The market fails**, with 1.53% of crossings and a Kupiec p-value
of 0.00, and the crossings cluster (Christoffersen p = 0.02). **The flights tell the story of the correction**: in 2013
the raw model passes and the correction fixes the bias but fails on the ECE; in 2023 the raw model fails and the
correction passes. **The cyclones pass** with ECE 0.0121, and **Toronto fails** with 0.0354. The null in brackets is
the 95% quantile of the ECE of a perfectly calibrated forecaster with the same forecasts (Chapter 8): every binary
vault sits above it, so every miscalibration here is detectable, and the gate of 0.03 decides which of them is
relevant. **The canary** is available in 0 emissions under the honest clock and gives no gain; with the clock switched
off it gives $+0.91$ or more. **The marker blind to colour** flags released Black detainees about twice as often as
released White ones, and misses detained Black detainees less often (0.667 and 0.500 against 0.772 and 0.838).

**What could not be rebuilt.** The arrests of 2020 and 2021 in Toronto and the traffic stops of Rhode Island have no
copy that can sit beside the book; their numbers in Step 1 come from the record of the research, and Appendix E sets the
record's numbers for the six rebuilt streams beside these.

**Exercises.**

1. *(Concept)* Why does a canary that gives no gain with the honest clock prove little if nobody tests it with the
   clock switched off? Give an example of a column that would be a bad canary.
2. *(Concept)* In the prisons of 1997–2002 the model beats the climatology and still fails calibration. Explain why a
   circuit breaker that compares the model with the reserve would not open, and propose a second criterion of opening
   based on calibration, saying what price it charges in number of forecast changes.
3. *(Count)* In the laboratory, the fixed climatology has Brier 0.02171 and the official model, 0.02130. Compute the
   skill and check it against the measured number of $+0.0098$ (the measured reference uses the rate of the past, 0.0222, and
   not the rate of the vault).
4. *(Laboratory)* Change, in the laboratory, the rate of the training period bringing it close to the rate of the vault (change the
   old intercept from $-4.005$ to $-4.30$) and run again. Does the raw model become well calibrated? What does
   that say about the value of Platt in that design?
5. *(Concept)* Why does the recalibration of the intercept not correct a model that is underconfident at the extremes?
   Describe the calibration curve of the two cases, with the axis of the forecasts against the one of the frequencies.
6. *(Concept)* In Toronto from 1997 to 2002, changing the metric changes who pays. Write, in four sentences, why
   that is not an error of account and rather a decision of value.

**Summary.**

- Zero leakage is only worth something with a canary that proves that the test would find the leakage: 0.000 with the honest clock,
  from $+0.55$ to $+0.97$ with the clock switched off.
- A forecaster can hit what it observes (1% of error) and err about the world (29% of excess); the instrument randomised
  inside the loop undoes the spiral, and the loop declared at 0.6 is four times the measured one, 0.145.
- Each calibration correction was only accepted in a new vault: the correction of the level failed the flights of 2013
  and passed those of 2023, from ECE 0.0646 to 0.0177; in the record, the prisons of 2020–2021 failed by slope (0.067)
  and Rhode Island passed with Platt, ECE 0.0048 and skill $+0.019$.
- Calibration is fixed; resolution stays where it is. Against a tolerance that depends on the size of the vault, all six
  binary vaults show detectable miscalibration, one only borderline; detectable is not yet relevant.
- Changing the metric of fairness changes who pays for the error; and none of the affected groups was heard in the seven
  decisions that involve people.
- Six streams run beside the book on public data, under a specification declared before the run; the market fails
  its value at risk, and the prisons of 1997–2002 fail calibration while beating the climatology.

## Chapter 21. The seven lessons and the final project

**Objectives.** By the end of this chapter you must know how to state the seven lessons in everyday situations, with
the condition under which each one holds; answer the main objections to the discipline; read the two falsifiable
predictions of the resolution axis; and conduct the final project: choose a problem of the school or of the city,
fill the card, simulate the world and defend the conclusion.

---

### Step 1 — The real world and the paradox

**The question that closes the course.** In the last meeting, someone asks what the course was worth in real life. Not
what it was worth for whoever is going to do statistics, but for whoever is going to decide. The answer has seven
sentences, and each one was born from a concrete case.

1. **The mean hides the essential.** In the health post, the mean demand is identical in the two worlds, and the wait
   jumps from 9.1 minutes in the constant world to 19.3 minutes in the world that oscillates. In every 100 patients, 24 wait
   more than half an hour in the world that oscillates, against 8 in the constant world. In a call centre, the schedule
   dimensioned by the mean fails in **about 31 of every 100 days**. And the mean wealth of whoever bet 3 times in the market
   appears as 19 times, when the median is 2.1.
2. **The independent unit decides the margin.** 2.65 million calls come from **81 independent days**; 225 thousand
   flights from **364**. The margin is computed over those units, and the calls weigh far less than their count.
   Counting row by row, where the variation repeats, separates an honest margin from a margin that only looks narrow.
3. **Hitting is not enough; one must have been able to err.** The propensity fit in the LaLonde experiment hit the
   number, and an innocuous treatment would have reached a similar value. In the 1975 placebo it invented a phantom
   effect of $-12.3$ percentage points. A number that hits without being able to err is not evidence.
4. **Failing one's own model is a result.** In the Danish fires, 36 months without a single giant fire
   are worth only **2.2 bits** of information. The monthly chance stays near $1/38$ by the Laplace rule, while the
   insurer tripled the bet. When the analyst's own model was tested in the vault, it lost to the simple historical
   mean (skill $-0.015$) and **did not win the right to decide**. The claim of the insurer fell.
5. **The effect transports better than the list of causes.** Exact invariance is rare. When it appears and is
   checked against a known truth (the light tunnel and the hERG blockers), it protects against the new
   intervention, as verapamil showed. When it does not appear, the direction and the size of the effect still usually survive.
   9 domains predicted the never-seen environment with skill from 0.04 to 0.88. Of the areas where exact invariance
   held, two recovered causes known before the test; the search found no new cause that survived without a caveat.
6. **Calibration is fixed afterwards, resolution is not.** Recalibration cannot add what only new information adds. Three vaults were needed for the calibration to pass the
   tolerance of 0.03, and a later measurement showed that this tolerance was loose in the large vaults, where a
   detectable miscalibration is not always a relevant one. The final skill
   stayed at $+0.019$. Saying "it is well calibrated" does not ensure that the model separates the cases well.
7. **Declaring before running makes the failure useful.** Seven rounds in the search for invariant causes and three vaults in the arrest forecasts only count as
   science because each change was written with a hash **before** running, and because each adjustment made after seeing a
   result stayed on the record and counted in the counter $K$. Declaring makes the path visible; it does not make it
   innocent. Under the first declared version of each choice, the gate of the causal search does not pass.

**Turning the test inward.** A discipline that demands a severe test of the claims of others has to pass the same
test. When ten of its own assumptions, things the text affirmed and had never tested, were each put to a declared
test, the result was uncomfortable and worth the reading. The inward audit found no error of arithmetic and shrank
the reach of the numbers; a later reading from outside, made with an AI assistant, found several errors that the
inward audit had passed over, and they are corrected in this text; no person outside the program has yet checked
either reading. The central thesis tied in its direct test. The calibration tolerance was loose.
The only real measurement of the performative loop, of a related quantity, did not support the size assumed. And one of the three areas of the causal
search depended on which version of each choice counted.

**Whom nobody heard.** In each one of the seven decisions that involve people (cutting the benefit of
work accident victims in Kentucky, reducing classes in Tennessee, changing the copayment in the health insurance,
comparing the job training of the NSW with controls of the CPS, marking the detainee in Toronto in the two periods and marking the
driver in Rhode Island), **no affected group was heard**. The list is large and has name and number: 11,598 students in 80 schools; 5,226 detainees; 65,276 arrests
of 2020 and 2021; 86,113 traffic stops; 185 participants of the NSW training and 15,992 controls. Signing is not
listening. Whoever signs an opinion stands on the side of whoever decides to use the artifact; whoever pays for the error stands on the other side.

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

**The seven objections, and the answer to each one.** A discipline that demands a severe test needs to survive the objections
that are made to it.

| Objection | Answer |
| --- | --- |
| "This is just statistics under another name." | In part, yes: almost all the tools have an owner for decades. What Stochology adds is the organisation — the layers, the typing of uncertainty and the same gates for every claim. The first direct test of that, the blind pilot, **tied at the ceiling**: the two arms pointed to the same 8 defects and the same 0 false alarms. A tie at the ceiling neither supports the thesis nor refutes it |
| "The data are old and public." | It is a choice: known datasets, verifiable by hash and with a known outcome, allow auditing the method, but do not replace a real operation. Some pieces remain assumed, such as the service time and the patience of the clients; others can stop being assumptions once measured, as the performative loop and the calibration tolerance did |
| "Formalising judgement is eliminating judgement." | The architecture does not decide what is worth asking nor how much an error costs for whoever suffers it. Those values enter through the elicitation and through the ratio between cost and loss, declared and not computed. The ambition is to make judgement visible |
| "The apparatus is too heavy." | Eight agent bands, five lenses and five invariants are a lot for a simple question. The funnel answers: most of the hypotheses fall cheaply, before touching the data. If the cost of the machine exceeds the value of the decisions that it improves, the lens of judgement itself orders it switched off |
| "Bayes solves the problem of Hume." | It does not. Bayesian updating is the coherent rule inside a space of hypotheses, but induction moves to the choice of the prior and of the space, as the Laplace rule showed. The Cox theorem itself depends on technical hypotheses (Halpern, 1999) |
| "Ergodicity became doctrine." | The lens of time relies on the research programme of Ole Peters, which is contested, and the discipline teaches it as a debate. By the criterion of growth, the loss aversion of about 2.25 measured in the laboratory is only justified when 56% of the wealth is at stake: the thesis explains the refusal to risk half a patrimony, not the aversion to small bets. Something similar holds for the tails: Taleb asks that a heavy tail be assumed until proof to the contrary; Clauset, Shalizi and Newman ask for proof before declaring a power law. The question is whose is the burden of proof, and the answer depends on how much it costs to err |
| "The auditor is a party with an interest." | True whenever the opinions are signed by whoever runs the work: naming someone responsible does not make the audit independent. A cheaper and weaker check is to reimplement results from zero, from the specification alone. In one such check four results agreed and no error was found, but several choices that the specification does not fix came out equal to the original, which weakens the test. Real independence asks for people from outside, with the same contract |

**The neighbours, and what is borrowed from each.** The first objection deserves a longer answer than a table row,
because the pieces of the machine have owners, and a discipline that did not name them would be claiming what it only
borrowed. The card of Chapter 17, with its fixed fields, is a cousin of the **model card** (Mitchell and colleagues,
2019) and of the **datasheet for datasets** (Gebru and colleagues, 2021): both force a model or a dataset to declare
its intended use, its provenance and where it fails. The card adds what those two leave out, the generating process,
the independent unit, the null model and who pays for the error, and it is refused by code, not by a reviewer. The
seal of Chapter 18 is **pre-registration** (Nosek and colleagues, 2018), with a hash in place of a registry and the
counter $K$ in place of the honour system; the registered report, which commits a journal before the data are seen,
is the institutional form of the same idea, and this book has no journal. The forecast diary and the proper score out
of sample are the discipline of the **forecasting tournament** (Tetlock and colleagues, 2014), where calibration and
resolution are charged on the forecaster and not on the forecast. The natural frequencies, the fixed denominator and
the absolute risk before the relative are **risk communication** as Gigerenzer and Spiegelhalter (2017) teach it. The
predictive check, the simulation-based calibration and the habit of fitting a rival model are the **Bayesian
workflow** (Gelman and colleagues, 2020), and the refusal to choose between belief and frequency, charging the one by
the other, is close to what Gelman and Hennig (2017) call going beyond the subjective and the objective. The open
research culture of the replication movement (Nosek and colleagues, 2015) is the ground all of this stands on.

What is not borrowed is the joint: that a claim about an uncertain world is checked in three layers at once, that its
uncertainty is typed before it is computed, and that the same gates apply to the author's claims as to anyone else's,
with the failures published. Whether that joint is worth more than its parts is the question the bridge thesis asks,
and it has not been answered.

**The card demands five questions and four prohibitions.** The instrument that turns a doubt into an auditable
claim asks five header questions. What is the **generative process** of the data, with seed; where does the
**non-linearity** live; what is the **independent unit**; what is the **physics of the noise**: its shape (Gaussian or with
jumps), whether there are absorbing states, and, when the noise depends on the state, whether it is read as idealised white
noise (Itô) or as the limit of a real noise with short correlation (Stratonovich), two readings that coincide when the
noise is additive; and at which step of the **causal ladder** the claim lives.
Four prohibitions close the list: no free natural language between agents, no solitary mean where there is
curvature or heavy tail, no confusion between learning and performativity without a counterfactual, and no dogmatic extreme
(no forecast of 0% or 100%, with a floor declared from the base rate) nor omission of attempts. Five invariants and seven automatic
gates verify the card, each with a pass or a fail and no third outcome.

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
   final question, when a model is good enough. Three conditions answer it. A model is good
   enough when it declares the failure modes, when someone from outside manages to reproduce every number, and when the
   decision that it authorises does not depend on any assumption that has not been written.

**The resolution of the question, and two predictions that could fail.** The decider question has a grain, and the
grain is a variable of the model. R0 asks about the average day, R1 about each day, R2 about the hour and R3 about
the subgroup. With the same total resource, paired arms and a staffing that aims at a utilisation of 0.8, asking
coarsely costs 25.54 minutes of mean wait where demand oscillates from 5 to 11, 2.77 where it oscillates from 7 to 9
and nothing where it does not oscillate (Chapters 5, 8 and 10). The axis states two claims; only one of them is an
empirical prediction. Prediction 3: there is an amplitude of
oscillation above which a plan made for the average day fails on more than a quarter of the days. Prediction 5: a
plan that survives every day also survives the aggregate, while the converse does not hold. Run it with
`cargo run --release --bin labs -- --resolution`.

```text
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
```

With the plan calibrated on flat demand, the threshold appears at amplitude 2.5, where the coarse question starts
to cost (2.87 minutes); the fraction of failed days grows monotonically. Below the threshold the loss column is
negative, and that does not contradict Chapter 5. In this specification the two arms do not spend the same resource:
the coarse plan keeps its 13 agents every day, calibrated so that flat demand fails on at most 10% of the days, while
the fine plan staffs each day with the smallest Erlang scale for that day, which near flat demand is fewer agents. The
coarse plan waits less there because it pays for more people, and the column is a comparison of two plans, not the
toll of the coarse question at equal resource that Chapter 5 measures.
(With the uncalibrated Erlang plan of the average day the fraction is not monotone, and the threshold reading is
weaker.) Across six profiles, no profile survives each day and fails the aggregate, and three pass the aggregate
while failing some day.

The two results are not of the same kind. Prediction 5 is a consequence of arithmetic, not of the world: when the day
fails on a threshold of a share, the aggregate share is a weighted mean of the daily shares, and a mean cannot exceed
the threshold if no day does. The instrument confirms that the code respects that arithmetic (it checks the
conservation of the aggregate), which is a test of the code, not of the axis. Prediction 3 is empirical, but it was
found under a second specification, after the first one (the Erlang plan of the average day) gave a fraction that
did not grow monotonically; the second specification counts in $K$, and the threshold of 2.5 is a result of that
specification, not a law. Prediction 3 stands as a supported regularity of one simulated post, and prediction 5 as a
theorem with a test of its implementation.

**Three more horizons.** *From the argument to the artifact:* the unit of knowledge is the executable artifact, not
the paragraph; claim, generative process, null model and test stay side by side. *Artificial agents with epistemic
discipline:* language models enter with an evaluation rubric ($\kappa = (p_o - p_e)/(1 - p_e)$), a record of the
version and of the date of the supplier, and the same prohibition of free text between agents. *An expiry date for
every target:* every metric is declared with the date on which it starts to hold, and the measured Goodhart effect
(Chapter 20) is the reason to declare the deadline before.

---

### Step 3 — Hands on

The laboratory of this chapter is the **verifier of the final project card**: a small gate that refuses a card
without the decider question, without a seed and without an exit clause, and proves that it knows how to reject, because each one of the seven
rules brings down at least one card. Run it with `cargo run --release --bin labs -- --chapter 21`.

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

**Exercises.**

1. *(Concept)* Choose a problem of your school and write the decider question in the three required parts. Then
   change the decider for "the decider" and explain, in two sentences, why the card would stop being auditable.
2. *(Concept)* Write an exit clause with an objective trigger for a decision of your city and a clause
   without any trigger. Which of the two survives a change of management?
3. *(Laboratory)* Run the verifier and add a card 13 with a correct decider question and a seed that is
   not numeric ("any"). How many cards are refused now, and by which rule?
4. *(Count)* Five of 33 domains changed status or action when a prior from the literature replaced the prior in
   use. Write that number as a natural frequency ("in every 100 domains...") and
   say why it is the size of what remains to be measured.
5. *(Concept)* The blind pilot tied at the ceiling: each arm pointed to 8 defects, and the two lists came out identical.
   Explain, in four sentences, why a tie at the ceiling neither supports the thesis nor refutes it.
6. *(Essay)* Answer the question that closes the course: when is a model good enough? Use at least three
   numbers from the chapters.

**Summary.**

- The seven lessons: the mean hides; the independent unit decides the margin; hitting is not enough if the test
  could not fail; failing the model that one built is a result; the effect transports better than the list of
  causes; calibration is fixed and resolution is not; declaring before running makes the failure useful.
- Each lesson holds under a condition, and a discipline that tests others must test itself: turned inward, the
  severe test narrowed the reach of its own numbers without finding an error of arithmetic.
- Signing is not listening: in seven decisions that touched people, none of the affected groups was heard.
- The resolution axis stated two claims: one is a theorem whose implementation was checked, the other an empirical
  regularity found under a second specification.
- The final project closes the course: choose a problem of the school or of the city, write the decider question,
  fill the card, fix the seed, build the null, pre-register, run and defend a memo in natural frequencies with an
  exit clause.

# Appendices

## Appendix A. Visual dictionary of Stochology

The dictionary gathers every term of the book, each defined in one simple sentence and illustrated with an everyday
example. The order is alphabetical.

| Term | In one sentence | Everyday example |
| --- | --- | --- |
| **Abduction** | Choosing, among the hypotheses considered, the one that makes what was seen least surprising | The noise in the engine wakes the neighbour: the simplest explanation is the car, not a drone |
| **Agent ($L_3$)** | Whoever decides, with limited time, attention and money: a person, an institution or an algorithm | You choosing the way home at 7 a.m., with the tank on reserve |
| **Aleatory uncertainty** | The variability of the world, which more data do not reduce | The outcome of the honest die: rolling ten thousand times does not make the die predictable |
| **Background audit** | A front that tests an assumption of the program itself, with a declaration written before running | The teacher who redoes the exam before applying it to the students, with the answer key sealed |
| **Bayes (rule of)** | The rule that says how today's belief is updated with the evidence of now, starting from yesterday's belief | A positive pregnancy test in a low-risk person can still be a false positive most of the time |
| **Bias (three senses)** | Distortion of the datum, of the estimator and of the mind, and the three accumulate | An uncalibrated scale, an average that mixes weights and the haste of whoever reads the number |
| **Brier (score of)** | The mean of the squared error of a probabilistic forecast: lower is better, and 0 is perfect | Betting 90% on a game and losing costs more on the scoreboard than betting 55% and losing |
| **Brier skill** | How much the model improves, in percentage, over the climatologist who only repeats the base rate | How much the rain forecast gains over "it rains as often as always" |
| **Calibration** | The property of whoever says "30%" and gets it right in about 30% of the cases | A meteorologist who says "chance of rain" 100 times and it rains in 62 of them is miscalibrated |
| **Canary** | A leak planted on purpose to prove that the leakage test works | The test camera the shop triggers on purpose to check whether the alarm rings |
| **Chouldechova's trilemma** | With different base rates between groups and an imperfect classifier, equal predictive value (PPV) and equal false-positive and false-negative rates cannot hold together. Equal PPV is one kind of calibration; calibration within groups (Kleinberg and colleagues) is a related but distinct one | Two classes with different difficulties cannot have, at the same time, a fair average grade and the same number of failures |
| **Circuit breaker** | The mechanism that swaps the model for a safe reserve when it loses over a moving window | The autopilot that switches itself off when the sensor disagrees for too long |
| **Confounder** | A common cause of two variables that makes it look as if one causes the other | Ice cream and drowning rise together: both rise with the heat |
| **Counter $K$** | The tally of the number of specifications tried in order to answer the same question | The number of attempts written in the margin of the notebook to get a password right |
| **Decision lens (Crisonomy)** | The lens of judgement: value of information, proper scores, calibration and cost of the error | Paying for one more exam before deciding whether the surgery is worth it |
| **Decision-maker's question** | The question the decision uses, with who decides, which action changes and the question mark at the end | "Does reducing the class to 15 students improve reading one year later?" |
| **Design (effect of)** | How many dependent rows are worth one independent row | The same 30 people answering 10 times are not worth 300 people |
| **ECE (expected calibration error)** | The mean distance between predicted and observed, computed by confidence bands | Estimating the error of a thermometer band by band, from 35 to 42 degrees |
| **Epistemic uncertainty** | The model's ignorance about the world, which shrinks with data | Not knowing the average height of the class; measuring everyone solves it |
| **Erlang C** | The classical formula that estimates how many attendants are needed for a queue | How many checkouts to open at the supermarket so that 80% of the customers wait less than 20 seconds |
| **EVPI and EVSI** | How much perfect information is worth, and how much the information from one sample is worth, in expected money | How much you would pay to know today tomorrow's demand, before buying the stock |
| **Exit clause** | The objective trigger that says when the recommendation stops holding | "I review the medicine if the fever stays above 39 °C for two days in a row" |
| **Filtered sample** | A sample that contains only the cases that survived some filter, taken for the whole population | Counting how many famous musicians dropped out of college and concluding that college gets in the way |
| **Fixed seed** | The starting point of the draw, declared so that two machines produce the same world | The recipe with the exact amount, so that the cake comes out the same at the neighbour's house |
| **Gate** | The automatic test that says yes or no, with exit code 0 or 1 | The subway turnstile: it does not open for whoever has not gone through |
| **Goodhart (law of)** | When the measure becomes the goal, it stops measuring | The school that trains the students for the exam and the grade rises without learning rising |
| **Heavy tail** | A distribution in which extreme events are far more frequent than the bell curve would suppose | Days of −10% on the stock market exist; in a Gaussian world they would almost never happen |
| **ICP (invariant causal prediction)** | The method that returns the variables common to every set whose relation to the outcome does not change across environments: a conservative estimate of the direct causes, wrong with at most the declared chance | The recipe that works in your house and in your grandmother's, with a different oven |
| **Independent unit** | The smallest unit whose observations do not share the same source of variation | The school, and not the student, when students of the same school resemble one another |
| **Information lens (Fluxonomy)** | The lens of the current: how probability and information flow, and where they are lost | The telephone game: the message loses bits at every repetition |
| **Inspection paradox** | Sampling by arrival over-represents the long intervals, so the experience of the average arrival is worse than the average interval | Asked how long they waited, most patients answer from the busy days, because most of them came on busy days |
| **Invariance lens (Morphonomy)** | The lens of form: what is preserved when the environment changes | The accent that survives the move to another city |
| **Invariants $\Phi_1$ to $\Phi_5$** | The five boundary conditions every claim must satisfy: simulability ($\Phi_1$), independent unit ($\Phi_2$), severe test ($\Phi_3$), scope and exit clause ($\Phi_4$) and resolution of the question ($\Phi_5$) | The five locks of the sheet that reject a work before the argument |
| **Jensen (inequality of)** | The mean of a curved function is not the function of the mean | The average waiting time grows when arrivals oscillate, even with the same average arrival |
| **Kelly (criterion of)** | The fraction of the estate that maximizes growth over the long run: $p/a - (1-p)/b$ for a bet that gains $b$ or loses $a$ | Betting 20% when the chance is 60% at 1 to 1, a quarter in the Peters coin; twice that has zero growth |
| **Knightian uncertainty** | In this book, a real process outside the hypothesis space that nobody listed, in which more data give false confidence (Knight's own sense, probabilities that cannot be measured, is its neighbour, Chapter 4) | The turkey that fattens for a thousand days until Christmas Day |
| **Kupiec (test of)** | The test that checks whether the number of violations of the value at risk is the expected one | Checking whether the flood insurance triggered at the promised frequency |
| **Laplace (rule of)** | The rule that adds one imaginary success and one imaginary failure before estimating a probability | "I never saw that happen" in 36 months is not the same as "it cannot happen" |
| **Lenses (the five)** | The five fixed ways of looking of the program: information, time, tail, invariance and decision, labelled in the code Fluxonomy, Chrononomy, Scalonomy, Morphonomy and Crisonomy | Five specialists examining the same house: plumber, electrician, mason, architect and the resident |
| **Likelihood** | The chance of seeing what was seen, supposing each hypothesis true | "How likely is this stain on the roof if it is rain? And if it is seepage?" |
| **Meng's identity** | The equation that says how much of the final answer depends on the data that are missing | Assessing the class by measuring only those who came to the exam: the absent ones change the average |
| **Model ($L_2$)** | The formal, simplified and simulable representation of the world | The scale model of the building, with paper walls |
| **Murphy (decomposition of)** | The account that separates the error of a forecast into reliability, resolution and difficulty of the event | A good student who fails the exam because of difficulty, not because of not knowing the subject |
| **Natural frequencies** | Stating the risk as "x in every 100", with a fixed denominator | "In every 100 patients, 25 wait more than half an hour" instead of "25% risk" |
| **Non-ergodicity** | When the average of one trajectory in time is not the average of many trajectories | The average of the people who crossed the river on foot does not describe whoever was carried away by the current |
| **Null model** | The simulation of what chance alone would produce, to compare with what was observed | Drawing groups by an honest lottery a thousand times before believing in the effect of the medicine |
| **$\operatorname{do}(x)$ (intervention)** | Fixing a variable by decision, and not merely observing who already has it | Drawing lots for who receives the medicine, instead of comparing those who were already taking it with those who were not |
| **$p$-value** | The frequency with which a world without effect would produce a result at least as extreme as the one seen | Flipping the coin 100 times and seeing 62 heads: how strange is that in the honest world? |
| **Pathology edge** | The mark, on the map of knowledge, of a misalignment between the layers of the world, the model and the agent | Two folders of the same client with different phone numbers, and nobody knows which one counts |
| **Pearl (ladder of)** | The three rungs: see, act and imagine what did not happen | Watching the price of tomatoes rise, deciding to buy pineapple, and imagining how things would be without the frost |
| **$\Phi_5$ (resolution of the question)** | The invariant that rejects aggregated decision ($R_0$) when the independent unit is the day or coarser | Scaling by the average day when the clinic alternates between calm and surge |
| **Pre-registration with hash** | Freezing the criterion before seeing the data, with the cryptographic digest recorded | Writing the guess and sealing the envelope before opening the result |
| **Proper score** | The score in which declaring one's own sincere belief is the best move | The pool that pays an honest 60% better than a bluffed 100%, so that saying what you believe is what scores |
| **Pseudoreplication** | Treating dependent observations as independent, shrinking the margin for no reason | Counting ten photos of the same person as ten people in the crowd |
| **Resolution** | The capacity to separate the cases that have the event from those that do not | Telling the days of heavy rain from the drizzle, and not only getting the month's average right |
| **Ruin** | Reaching zero and no longer being able to play, regardless of the average of the bets | Whoever uses the whole credit card at the limit loses the right to stay in the game |
| **Safeguard panel** | The checklist of eight safeguards that a card must declare and anchor before approval; it prints one pair of bits per safeguard and runs no attack itself | The pre-flight checklist: it proves that every item was looked at, not that the plane will fly |
| **Saturation** | When the environment is a function of the variables of the set, and the test has no way of rejecting | Counting the traffic light's colour to forecast the traffic light's colour |
| **Scope term** | The declaration of where the conclusion holds and where it does not | "This holds for the morning shift; at the weekend the queue is another one" |
| **Severity** | The chance that the procedure would have given a less favourable result if the claim were false | The fire drill that counts only because the alarm would have rung had there been smoke |
| **Sheet** | The closed list of questions asked of a text, a number or a model | The mechanic's checklist: without it, the same item is always forgotten |
| **Sheet bus** | The channel through which the pieces of the work exchange sheets, chained by cryptographic digest, so that nothing can be rewritten without breaking the chain | The minutes of a meeting signed page by page: changing one page invalidates the signature of the following ones |
| **SHELF** | The protocol for eliciting experts by median, quartiles and extremes | Asking the doctor not for a guess, but for a range and a most probable value |
| **$t_{\mathrm{disp}}$ (clock)** | The instant at which each datum becomes knowable | The exam result that only exists after it leaves the laboratory |
| **Tail lens (Scalonomy)** | The lens of scale: tail exponent, existence of the mean, effective size | Measuring whether a flood can swallow the historical average of floods |
| **Time lens (Chrononomy)** | The lens of time: the average of many people against the average of one life, ruin and growth | Whoever bets half the estate every month goes broke before harvesting the pretty average |
| **Type I error** | Raising the alarm when there was nothing | The smoke detector that goes off while the bread is toasting |
| **Type II error** | Not raising the alarm when there was something | The detector with a weak battery that stays silent in the fire |
| **Type III error** | Answering with precision a question that was not the decision-maker's | Delivering the average grade when the question was how many students stay for remedial classes |
| **Type M error** | Getting the size of the effect wrong, too large or too small, typically by a study too small for the effect it seeks | A small trial that only reaches significance when chance doubles the effect, and then reports the doubled effect |
| **Type S error** | Getting the sign of the effect wrong: saying it improves when it worsens | Recommending a medicine that gets in the way, because of a signal noise |
| **Typed hypergraph** | A map of knowledge in which the links have a type and can join more than two things | The subway map with named lines, and not only points joined by strokes |
| **Vault** | The part of the data that stays closed until pre-registration, and opens a single time | The sealed envelope of the exam, opened only after the student hands in the answer |
| **World ($L_1$)** | The reality that generates the data and that we never see in full | The real neighbourhood, with the holes the map does not show |

---

## Appendix B. Pocket guide to the formulas

### B.1 How to read the symbols

| Symbol | Read in English | In Rust code |
| --- | --- | --- |
| $\sum_{k=1}^{n} a_k$ | "Add up everything that comes after, from $k$ equal to 1 to $n$" | `let sum: f64 = a.iter().sum();` |
| $\mathbb{E}[X]$ | "The long-run mean of $X$" | `let mean = x.iter().sum::<f64>() / x.len() as f64;` |
| $\operatorname{Var}(X)$ | "The mean dispersion of $X$ around its mean, squared" | `let var = x.iter().map(\|v\| (v - mean).powi(2)).sum::<f64>() / x.len() as f64;` |
| $\sigma$ or $\mathrm{dp}$ | "Standard deviation: the square root of the variance, in the same unit as $X$" | `let dp = var.sqrt();` |
| $P(A \mid B)$ | "Probability of $A$ **given that** $B$ happened" | `if b { p_a_given_b } else { p_a_given_not_b }` |
| $\operatorname{do}(x)$ | "Fixing $x$ by decision, and not merely observing who already has $x$" | `let y = f(x_fixed);` — instead of `let y = f(x_observed);` |
| $\ln$ | "Natural logarithm: the exponent of Euler's number that returns the value" | `let z = value.ln();` |
| $\log_{10}$ | "Logarithm in base 10" | `let d = 10.0 * ratio.log10();` (decibans) |
| $\ge$ | "Is greater than or equal to" | `a >= b` |
| $\approx$ | "Approximately equal", within a declared tolerance | `(a - b).abs() < 1e-9` |
| $\sqrt{x}$ | "The square root of $x$" | `x.sqrt()` |
| $x^2$ | "The square of $x$" | `x.powi(2)` |
| $\alpha$ | "The false-alarm level that is accepted: 5% is the usual one" | `const ALPHA: f64 = 0.05;` |
| $1 - \alpha$ | "The promised coverage of the procedure: 95% in the usual case" | `let coverage = 1.0 - ALPHA;` |
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
| $\text{skill} = 1 - \operatorname{Brier}/\operatorname{Brier}_{\text{clim}}$ | How much the model gains over the climatologist | `1.0 - brier / brier_ref` |
| $\mathrm{ECE} = \sum_k \frac{n_k}{n}\lvert \bar{p}_k - \bar{y}_k\rvert$ | The mean distance between predicted and observed, band by band | 10 boxes of equal mass, summing the weighted absolute difference |
| $f^* = 2p - 1$ | The Kelly fraction for a bet that pays the same as it risks | `let f = 2.0 * p - 1.0;` |
| $\frac{s+1}{n+2}$ | The rule of Laplace: one imaginary success and one imaginary failure | `(s + 1.0) / (n + 2.0)` |
| $W_{M/M/1} = \frac{\rho}{\mu - \lambda}$ | The average wait in a queue with one server grows on a curve | Do not use the mean to decide the scale: simulate the day |
| $\mathrm{CITL} = \bar{p} - \bar{y}$ | The level deviation: how much the average forecast misses the observed rate | `mean(p) - mean(y)` |

**A reading warning.** In Rust, `f64` holds about 15 to 16 digits. Therefore $\approx$ is never
equality, and every comparison of measured numbers needs a **declared** tolerance, such as the band of 0.05
standard deviations that the background audit fixed for comparing two rounds of the same domain. A band chosen
after seeing the result counts as a new declaration and enters the counter of attempts.

---

## Appendix C. Reproduction guide

### C.1 Running the laboratories

Every number in Step 3 of a chapter comes out of a command that anyone can run on another machine and obtain the
same output, because no laboratory reads the system clock or draws an unseeded random number.

| Command | What it shows |
| --- | --- |
| `cargo run --release --bin labs -- --chapter 19` | The output block of a chapter; replace 19 with any chapter, or use `--all` |
| `cargo run --release --bin labs -- --bank` | The real call-centre data measured (Chapter 12) |
| `cargo run --release --bin labs -- --queue` | The queue with abandonment and the four staffing rules (Chapter 12) |
| `cargo run --release --bin labs -- --s1` and `--s2` | System 1 triage and System 2 deliberation (Chapter 16) |
| `cargo run --release --bin labs -- --resolution` | The resolution axis and its two predictions (Chapters 5, 8, 10, 12 and 21) |
| `cargo run --release --bin labs -- --genetics` | The design of the second row in the yeast validation (Chapter 19) |
| `cargo run --release --bin labs -- --streams` | The real streams of Chapter 20, rebuilt on public data |
| `cargo run --release --bin labs -- --eprocess` | The H2 gate under the e-process, over the 38 recorded tests (Chapter 19) |
| `cargo run --release --bin labs -- --mc-error` | How far each simulated number the book reads moves with the seed (C.3) |
| `cargo run --release --bin labs -- --propagate` | Three answers to the same question, with the uncertainty budget (Chapter 5) |
| `cargo run --release --bin labs -- --multiverse` | The same question under eighteen reasonable specifications, and how much of its spread each choice carries (Chapter 5) |
| `cargo run --release --bin labs -- --stacking` | Combining models when none is true: model averaging against stacking, with a duplicated model (Chapter 8) |
| `cargo run --release --bin labs -- --discrepancy` | A physical parameter learnt with and without a model of the discrepancy (Chapter 14) |
| `cargo run --release --bin labs -- --passk` | Reliability against average success: pass^k over the tasks of four worlds, with the band of 2,000 repetitions (Chapter 16) |
| `cargo test --workspace` | That the types and the gates of the architecture work: 426 tests pass |
| `python3 -I scripts/reimplementation/run.py` | That 573 quantities of 20 chapters, recomputed from the text and the data files alone in another language, agree with the book (570), or are marked open where the recipe is longer than a paragraph and stays cited (3) |
| `cargo run --release --bin gates -- --horizon all --reproduction` | That the gates run twice and give the same result, with the Merkle root `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81` |
| `cargo run --release --bin gates -- --horizon h3 --defect h3.voos23` | That a gate **knows how to reject**: with the defect planted, the command exits with code 1 and says which gate fell |

### C.2 The environment is part of the number

The numpy case shows why. The estimated effect of the performative loop was published as 0.5632 with 2,375
shocks, and the identical code, run with the same seed, gave 0.5529 and 2,395 shocks. There were no two versions of
the code. There were two numerical libraries in the interpreter, and the default path imported 2.4.6 before 2.5.1.
The point of the divergence was isolated. In the two versions, the generator started from the same state, and a
binomial draw of 248 trials with probability 0.6 returns 171 in one and 150 in the other, because 2.5 fixed the
Stirling series of the acceptance test of the BTPE algorithm. The fix is upstream and documented: the NumPy 2.5.0
release notes, under "Corrections to the BTPE binomial sampler" (gh-31238), state that the third and fourth terms
were added instead of subtracted, an error inherited from the 1988 paper of Kachitvichyanukul and Schmeiser, that the
leading coefficient read 13680 instead of 13860, and that `Generator.binomial` may therefore return different samples
for the same seed, while the legacy `RandomState` keeps the old stream. If you meet a divergence like this, the path is always
the same. **First the environment, then the code**: record the version of every library next to the number, because
a number without its environment is a number nobody can audit.

### C.3 The seed is part of the number too

A fixed seed makes a simulated number reproducible on any machine; it does not make it exact. Another seed gives
another draw, and the distance between the two is the Monte Carlo error of the number, which shrinks like
$1/\sqrt N$ in the number of draws and faster for a mean than for a tail quantile. `labs --mc-error` measures it for
every simulated number on which a reading of the book depends: each simulated quantile is rerun under twenty other
seeds, and each simulated share gets its exact Clopper-Pearson interval.

No reading moves. In the four binary vaults of Chapters 8 and 20, the ECE stays above the null q95 under every seed,
including the cyclones, where the margin is the narrowest (ECE 0.0121 against seeds from 0.0107 to 0.0111). The
bootstrap interval of the tail index in Chapter 14 contains 2 under every seed. The share of 0.0140 in Chapter 13 has
the exact interval [0.0093; 0.0202], and the 20.3% of Chapter 7 the interval [19.06%; 21.58%].

Two published values sit at the edge of their spread, and the chapters now say so. The null q95 of 0.0425 in Chapter 8
is the largest of twenty-one seeds (the others range from 0.0409 to 0.0424), and the upper bound 2.61 of the interval in
Chapter 14 is above every other seed's (from 2.43 to 2.60). Neither changes a conclusion. Both are the reason a
simulated number is reported with the precision its draws allow, and not with the four decimals the printer allows.

The audit does not cover every draw. Of the 27 modules of the laboratories that draw random numbers, 4 print their own
spread (the propagation, the multiverse, the reliability and the stacking laboratories), 6 are audited by `--mc-error`,
3 are of another kind (Chapter 1, which is about seeds; the discrepancy, which averages over a thousand datasets; the
choice of cases of the bridge, a selection and not an estimate), and **14 are not audited**: their numbers are
reproducible, and how far they would move under another seed has not been measured. A register of the 27 is kept in
the code, and a test refuses a new module that draws random numbers without saying where its spread is measured, or
that it is not.

---

## Appendix D. Annotated bibliography for young scientists

### D.1 How this list was checked

The metadata of every line of D.2 to D.4 was checked against the Crossref API, the publishers' pages, PubMed, the
PMLR proceedings, IDEAS/RePEc and library catalogues, and every line carries the mark **[checked]**: 50 of 50. The
check is of the metadata (authors, year, title, journal, volume, pages and DOI) and of the number the text uses;
third-party files are not reproduced. The check found no divergence of content, completed the DOIs that were
missing, and corrected two earlier citations: the article on the benefit cap is by **Meyer**, Viscusi and Durbin,
and Baltagi and Levin came out in *Structural Change and Economic Dynamics*. Two source reservations remain: the
final page of Chernoff 1954 and that of van Dantzig 1956 did not come out in the sources consulted. SHELF is a
protocol, not a book. The line on proper scoring rules in D.2 entered with the review of 2026-10-18, which found that they
had been credited to Savage 1951, an article on decision criteria; its three references were checked against Crossref
on 2026-10-19. A reading of 2026-10-19 found that the text cites, by author and year, many works that
the list did not carry; they are gathered in D.5. The same day 24 of the 26 were checked against the Crossref API (and
two against arXiv and the PMLR proceedings): no divergence of content; the issue of Peters 2019 and the volume and
article number of Kleinberg and colleagues were completed. Two source reservations, as for Chernoff and van Dantzig:
Crossref gives only the first page of Berkson 1946 and no pages for Hill 1975. Cover and Thomas, refused once by a rate limit, was checked later
the same day by its DOI (Crossref dates the online edition 2005; the second print edition is 2006). Platt 1999 is a
book chapter that Crossref does not index; its full title and pages were checked against a secondary source, which is
weaker than the others and is declared as such.

### D.2 Classics to begin with

| Reference | Why read it now |
| --- | --- |
| **[checked]** Bernoulli, *Ars Conjectandi* (1713) | The book that turned "probably" into an account. The rule of Laplace of this book is its daughter |
| **[checked]** Hume, *Enquiry* (1748; the title *Enquiry* is from the 1758 edition) | The question that no statistics solves: why does the future resemble the past? |
| **[checked]** Knight, *Risk, Uncertainty and Profit* (1921) | The distinction between measurable risk and uncertainty that cannot be measured, in Chapter 4 |
| **[checked]** Savage, *The Foundations of Statistics* (1954) and Savage, *JASA* 46:55–67 (1951) | Belief measured by betting, and the minimax-regret criterion of Chapter 11 |
| **[checked]** Brier, *Monthly Weather Review* 78(1):1–3 (1950); Good, *JRSS B* 14(1):107–114 (1952); Savage, *JASA* 66(336):783–801 (1971) | Where the proper scoring rules come from: the quadratic score, the logarithmic score, and the theory that ties them to honest elicitation |
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
| **[checked]** Shen, H.; Huang, J. Z. *Annals of Applied Statistics* 2(2):601–623, 2008. DOI: 10.1214/08-AOAS164 | The bank call centre, in 2003 (Chapters 12 and 20) |
| **[checked]** LaLonde, R. J. *American Economic Review* 76(4):604–620, 1986 (JSTOR 1806062) | The NSW job-training experiment and the raw difference that deceives (Chapter 13) |
| **[checked]** Dehejia, R. H.; Wahba, S. *JASA* 94(448):1053–1062, 1999. DOI: 10.1080/01621459.1999.10473858 | The answer to LaLonde: matching hits the number, and why that is still not enough |
| **[checked]** McNeil, A. J. *ASTIN Bulletin* 27(1):117–137, 1997. DOI: 10.2143/AST.27.1.563210 | The Danish fires and the wild tail (Chapter 14) |
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

### D.5 Cited in the text and missing from the list until 2026-10-19

| Reference | Where it enters |
| --- | --- |
| **[checked]** Kelly, J. L. *Bell System Technical Journal* 35(4):917–926, 1956. DOI: 10.1002/j.1538-7305.1956.tb03809.x | The growth-optimal fraction (Chapters 10 and 13) |
| **[checked]** Peters, O. *Nature Physics* 15(12):1216–1221, 2019. DOI: 10.1038/s41567-019-0732-0 | The ergodicity problem and the Peters coin (Chapter 10) |
| **[checked]** Peters, O.; Adamou, A. "Insurance makes wealth grow faster", arXiv:1507.04655, 2015 | Bernoulli's ship in time (Chapters 10 and 11) |
| **[checked]** Meng, X.-L. *Annals of Applied Statistics* 12(2):685–726, 2018 | The identity of quality, quantity and difficulty (Chapter 6) |
| **[checked]** Berkson, J. *Biometrics Bulletin* 2(3):47–53, 1946 | The filter that manufactures relations (Chapters 6 and 9) |
| **[checked]** Charig, C. R. et al. *BMJ* 292(6524):879–882, 1986 | The kidney-stone table (Chapters 6 and 9) |
| **[checked]** Cox, R. T. *American Journal of Physics* 14(1):1–13, 1946 | Why belief obeys probability (Chapter 3) |
| **[checked]** Jaynes, E. T. *Physical Review* 106(4):620–630, 1957 | Maximum entropy (Chapter 3) |
| **[checked]** Halpern, J. Y. *Journal of Artificial Intelligence Research* 10:67–85, 1999 | The counterexample to Cox (Chapter 21) |
| **[checked]** Gilovich, T.; Vallone, R.; Tversky, A. *Cognitive Psychology* 17(3):295–314, 1985 | The hot hand (Chapter 7) |
| **[checked]** Miller, J. B.; Sanjurjo, A. *Econometrica* 86(6):2019–2047, 2018 | The biased test of the hot hand (Chapter 7) |
| **[checked]** Simmons, J. P.; Nelson, L. D.; Simonsohn, U. *Psychological Science* 22(11):1359–1366, 2011 | The garden of forking paths (Chapter 7) |
| **[checked]** Ioannidis, J. P. A. *PLoS Medicine* 2(8):e124, 2005 | The predictive value of a finding (Chapter 7) |
| **[checked]** Open Science Collaboration. *Science* 349(6251):aac4716, 2015 | The replication of 100 psychology studies (Chapter 7) |
| **[checked]** Murphy, A. H. *Journal of Applied Meteorology* 12(4):595–600, 1973 | The decomposition of the Brier score (Chapters 8 and 20) |
| **[checked]** Gneiting, T.; Raftery, A. E. *JASA* 102(477):359–378, 2007 | Proper scoring rules (Chapter 8) |
| **[checked]** Platt, J. C. "Probabilistic outputs for support vector machines and comparisons to regularized likelihood methods", in *Advances in Large Margin Classifiers*, MIT Press, 1999, pp. 61–74 | Platt recalibration (Chapters 8 and 20) |
| **[checked]** Hernán, M. A. et al. *Epidemiology* 19(6):766–779, 2008 | Hormone therapy reanalysed as a trial (Chapter 9) |
| **[checked]** Hill, B. M. *Annals of Statistics* 3(5):1163–1174, 1975. DOI: 10.1214/aos/1176343247 | The tail estimator (Chapters 10, 14 and 15) |
| **[checked]** Clauset, A.; Shalizi, C. R.; Newman, M. E. J. *SIAM Review* 51(4):661–703, 2009 | Proof before declaring a power law (Chapter 21) |
| **[checked]** Good, I. J. *British Journal for the Philosophy of Science* 17(4):319–321, 1967 | Free information never hurts (Chapter 11) |
| **[checked]** Lucas, R. E. *Carnegie-Rochester Conference Series on Public Policy* 1:19–46, 1976 | The critique of policy evaluation (Chapters 9 and 11) |
| **[checked]** Perdomo, J.; Zrnic, T.; Mendler-Dünner, C.; Hardt, M. *ICML*, PMLR 119:7599–7609, 2020 | Performative stability (Chapter 11) |
| **[checked]** Chouldechova, A. *Big Data* 5(2):153–163, 2017 | The identity of the trilemma (Chapters 11 and 20) |
| **[checked]** Kleinberg, J.; Mullainathan, S.; Raghavan, M. *ITCS 2017*, LIPIcs 67:43:1–43:23. DOI: 10.4230/LIPIcs.ITCS.2017.43 | Calibration within groups (Chapter 11) |
| **[checked]** Cover, T. M.; Thomas, J. A. *Elements of Information Theory*, 2nd ed., Wiley, 2006. DOI: 10.1002/047174882X | The information lens under its usual name (Chapter 15) |

### D.6 The neighbours of the discipline

Cited in Chapter 21 and, the next three, in Chapters 5, 11 and 14; metadata checked against Crossref, arXiv and the publishers on 2026-10-19. The last eleven entered the same day with the theory that the research on models of the world, of agents and of modelling brought to Chapters 2, 4, 8, 14, 16 and 19; their metadata were checked against Crossref (the five DOIs of journals; Crossref gives no pages for Yao and colleagues) and the arXiv pages.

| Reference | Where it enters |
| --- | --- |
| **[checked]** Mitchell, M.; Wu, S.; Zaldivar, A. et al. "Model Cards for Model Reporting", *FAT\* '19*, pp. 220–229, 2019. DOI: 10.1145/3287560.3287596 | The card as a cousin of the model card (Chapter 21) |
| **[checked]** Gebru, T.; Morgenstern, J.; Vecchione, B. et al. "Datasheets for datasets", *Communications of the ACM* 64(12):86–92, 2021. DOI: 10.1145/3458723 | The card as a cousin of the datasheet (Chapter 21) |
| **[checked]** Nosek, B. A.; Ebersole, C. R.; DeHaven, A. C.; Mellor, D. T. "The preregistration revolution", *PNAS* 115(11):2600–2606, 2018. DOI: 10.1073/pnas.1708274114 | The seal as pre-registration (Chapters 18 and 21) |
| **[checked]** Nosek, B. A.; Alter, G.; Banks, G. C. et al. "Promoting an open research culture", *Science* 348(6242):1422–1425, 2015. DOI: 10.1126/science.aab2374 | The open research culture (Chapter 21) |
| **[checked]** Tetlock, P. E.; Mellers, B. A.; Rohrbaugh, N.; Chen, E. "Forecasting Tournaments", *Current Directions in Psychological Science* 23(4):290–295, 2014. DOI: 10.1177/0963721414534257 | The forecast diary as a tournament (Chapters 18 and 21) |
| **[checked]** Spiegelhalter, D. "Risk and Uncertainty Communication", *Annual Review of Statistics and Its Application* 4:31–60, 2017. DOI: 10.1146/annurev-statistics-010814-020148 | Natural frequencies and absolute risk (Chapters 11 and 21) |
| **[checked]** Gelman, A.; Hennig, C. "Beyond Subjective and Objective in Statistics", *JRSS A* 180(4):967–1033, 2017. DOI: 10.1111/rssa.12276 | Belief charged by frequency (Chapter 21) |
| **[checked]** Gelman, A.; Vehtari, A.; Simpson, D. et al. "Bayesian Workflow", arXiv:2011.01808, 2020 | Predictive checks, SBC and rival models (Chapters 16 and 21) |
| **[checked]** JCGM 100:2008. *Evaluation of measurement data — Guide to the expression of uncertainty in measurement* (GUM). Joint Committee for Guides in Metrology, BIPM, 2008. DOI: 10.59161/JCGM100-2008E | The propagation of uncertainty, the first-order law and its limits (Chapter 5) |
| **[checked]** NIST/SEMATECH. *e-Handbook of Statistical Methods* (NIST Handbook 151). National Institute of Standards and Technology. DOI: 10.18434/M32189 | Moments, correlation and the analysis of uncertainty, for reference (Chapters 5, 6 and 14) |
| **[checked]** Rockafellar, R. T.; Uryasev, S. "Optimization of conditional value-at-risk", *Journal of Risk* 2(3), 2000 | The conditional value at risk, the mean of the worst cases (Chapter 11) |
| **[checked]** Grimm, C.; Barreto, A.; Singh, S.; Silver, D. "The Value Equivalence Principle for Model-Based Reinforcement Learning", arXiv:2011.03506, 2020 | A model good enough for the decision (Chapter 2) |
| **[checked]** Ha, D.; Schmidhuber, J. "World Models", arXiv:1803.10122, 2018. DOI: 10.5281/zenodo.1207631 | An agent that exploits the errors of its own model (Chapter 2) |
| **[checked]** Breznau, N.; Rinke, E. M.; Wuttke, A. et al. "Observing many researchers using the same data and hypothesis reveals a hidden universe of uncertainty", *PNAS* 119(44):e2203150119, 2022. DOI: 10.1073/pnas.2203150119 | Seventy-three teams, one hypothesis: the modeller as a source of variance (Chapter 2) |
| **[checked]** Park, J. S.; Zou, C. Q.; Kamphorst, J. et al. "Generative Agent Simulations of 1,000 People", arXiv:2411.10109, 2024 (revised in 2026 as "LLM Agents Grounded in Self-Reports Enable General-Purpose Simulation of Individuals") | Accuracy of a model of a person against the person's own consistency (Chapter 4) |
| **[checked]** Li, D.; Li, L.; Qiu, H. S. "ChatGPT is not A Man but Das Man: Representativeness and Structural Consistency of Silicon Samples Generated by Large Language Models", arXiv:2507.02919, 2025 | Simulated samples that lose the variance inside their groups (Chapter 4) |
| **[checked]** Lieder, F.; Griffiths, T. L. "Resource-rational analysis: Understanding human cognition as the optimal use of limited computational resources", *Behavioral and Brain Sciences* 43:e1, 2020. DOI: 10.1017/S0140525X1900061X | The value of computation, and when to wake System 2 (Chapter 16) |
| **[checked]** Ullman, T. "Large Language Models Fail on Trivial Alterations to Theory-of-Mind Tasks", arXiv:2302.08399, 2023 | Invariance as a test of understanding (Chapter 19) |
| **[checked]** Yao, Y.; Vehtari, A.; Simpson, D.; Gelman, A. "Using Stacking to Average Bayesian Predictive Distributions (with Discussion)", *Bayesian Analysis* 13(3), 2018. DOI: 10.1214/17-BA1091 | Stacking against model averaging when no candidate is true (Chapter 8) |
| **[checked]** Cemri, M.; Pan, M. Z.; Yang, S. et al. "Why Do Multi-Agent LLM Systems Fail?", arXiv:2503.13657, 2025 | The failures of teams of agents, against the panel (Chapter 16) |
| **[checked]** Kennedy, M. C.; O'Hagan, A. "Bayesian Calibration of Computer Models", *JRSS B* 63(3):425–464, 2001. DOI: 10.1111/1467-9868.00294 | Simulator, discrepancy and noise (Chapter 14) |
| **[checked]** Brynjarsdóttir, J.; O'Hagan, A. "Learning about physical parameters: the importance of model discrepancy", *Inverse Problems* 30(11):114007, 2014. DOI: 10.1088/0266-5611/30/11/114007 | What the discrepancy may not do, and why the parameter needs it (Chapter 14) |

### D.7 How to use this list

Read in the order that curiosity asks for, and distrust whoever presents any of these works as a
final answer. Three practical pieces of advice. First, read the data before the method, because almost every error of
reading is born from not asking where the table came from. Second, when an article cites a number, look for the
number in the code or in the appendix, and not in the abstract. Third, keep the version of the environment next to the number you
reproduced; the numpy case of Appendix C shows why.

## Appendix E. Provenance of the numbers

The chapters teach with numbers measured by a fixed-seed instrument. Where a measured number differs from the
value an earlier text or the course outline recorded, both values and the cause are kept here, chapter by
chapter, so that the chapters can teach without carrying the comparison.

**How to read this appendix.** The two rectification tables are for every reader: they show errors that passed
every gate. The sections by chapter that follow record smaller differences against earlier texts, and the sections
of Chapters 15 to 18 record differences of naming and of counting between the documents of the repository. Those are
for whoever audits the repository, and a reader of the book can skip them.

**The numbering of the chapters.** On 2026-10-19 the chapters were reordered, so that the book reads as two: the
method (Parts I to IV, Chapters 1 to 14, with the three real cases right after the theory) and the auditable machine
(Part V, Chapters 15 to 21). Every chapter number in this book, this appendix included, follows the new numbering.
Records outside the book (the dated state records, the declarations and the empirical points) keep the numbering of
their date; this table translates.

| Before 2026-10-19 | After | Chapter |
| --- | --- | --- |
| 12 | 15 | The map of knowledge |
| 13 | 16 | The team of specialists, the safeguard panel and the two systems |
| 14 | 17 | The iron contract |
| 15 | 18 | The vault and the seal |
| 16 | 12 | The health post and the bank |
| 17 | 13 | The market and the employment programme |
| 18 | 14 | The Danish fires |

Chapters 1 to 11 and 19 to 21 keep their numbers.

**Who read from outside.** The rectifications of 2026-10-18 were applied by an AI assistant (the rectification commit
of that day is authored by "Claude"), and the second table below comes from a later review of the same kind. Both
readings stand outside the authors' own audit of Chapter 18, but neither meets the independence that Chapter 21 asks
for: no person outside the program has yet checked them. They count as entries in the errata of
`docs/OUTSIDE_AUDIT.md`, not as its completion.

### Rectifications of 2026-10-18

A reading from outside, made after the inward audit of Chapter 18, found errors that the instruments reproduced
faithfully, because the seed fixes a wrong account as well as a right one. Each was corrected in the code, given a test
that would have caught it, and corrected in the text. The earlier value stays here, beside the corrected one.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 10 | Kelly fraction of the Peters coin | $f^* = 0.9p - 0.4 = 0.14$, growth $+0.0175$ | $f^* = p/a - (1-p)/b = 0.25$, growth $+0.006211$ | the formula omitted the division by $ab$ and used $p = 0.6$ for a fair coin |
| 10 | "Full Kelly" in the cohort | $f = 0.70$, ruin 50.89% | $f = 0.25$, no ruin; $f = 0.70$ is 2.8 times Kelly, growth $-0.014200$ | consequence of the line above |
| 10 | "Twice Kelly impoverishes" | $g(0.28)$ below zero | $g(0.50) = 0$ exactly; $g(0.28) > 0$ | idem |
| 10 | Insurance seller | loss of 50%, all-in growth $+0.00714$ labelled "negative: ruin is certain" | loss of 95%, all-in growth $-0.00207$; Kelly 0.648 (true chance), 0.652 (Laplace) | with a 50% loss the all-in grew, so the example did not show what it claimed; the Kelly formula divided the edge by the gain only |
| 10 | Volatility drag of the synthetic market | 0.0172, 0.0926, 0.2264 per year, $3\times$ above $2\times$ | 0.0156, 0.0623, 0.1402 per year, $2\times$ above $3\times$, as in the real index | the calibration dropped the factor of 252 sessions ($\sigma = 0.01518$ instead of 0.01111), and the drag was measured against the population mean instead of the sample mean |
| 10 | "Fraction of the mean above the largest seen" | 0.000 | replaced by the share of each sample's sum held by its largest value, 0.088 | the old quantity is zero by definition |
| 10 | Bernoulli's ship in time | shipowner from 1.94% to 2.18% per month | from $-0.0208$ to $-0.0182$, as in Chapter 11 | the two chapters used different accounts |
| 11 | EVSI of the two tests | text 0.419 and 0; laboratory 0.2400 and 0.2450 | 0.2200 and 0.0000 | the laboratory chose the best act per state inside each branch (perfect information) and swapped the predictive values of the negative branch |
| 11 | Mutual information of the two tests | "the same, 0.319 bit" | 0.241 and 0.079 bit | never measured; now measured |
| 11 | Gumbel model of the dikes | levels 1.26 m at 100 and at 10,000 years; height 3.06 m | 4.76 and 8.27 m; height 7.56 m; mean of models 8.95 m | the $\xi \to 0$ branch returned $\mu$ and used $e^{-e^{+z}}$ for the exceedance |
| 11 | "EVPI of the model set" | 18.427 m | spread of heights 13.92 m; EVPI 1.919 in cost units | a distance in metres is not a value of information |
| 11 | Equalising the FNR | downwards: false positives 1,299 to 659 and negatives 270 to 206 | false positives to 1,778 ($+479$), negatives to 206 ($-64$); upwards: positives to 1,062, negatives to 309 | the old map lowered both errors at once, which no threshold can do; now a binormal ROC |
| 11 | Chouldechova identity | "PPV of the identity" 0.0592 and 0.0904, "the identity accuses" | the identity returns the observed FPR exactly; the trilemma forces FPR 0.1769 on Black under the PPV and FNR of White | the formula used $1 - \mathrm{FPR}$ for FPR; an identity cannot fail on one group |
| 11 | Column "PPV" of the trilemma table | 0.9408 and 0.9096 | 0.3491 and 0.2896 | the column printed another quantity |
| 11 | Insurer with many contracts | $+0.100410$ per month with 1,000 contracts (mean minus half variance, failing for few contracts) | exact binomial account with 50,000 of capital per contract: $-0.018042$ with 1, $+0.001987$ with 1,000 | the approximation was outside its range |
| 9 | Simpson standardised difference | "larger than that of any single stratum" | 5.37 points, between 6.44 and 4.25 | a standardised difference is a weighted mean of the stratum differences |
| 9 | ICP laboratory | intercept free per environment under the null; empty set never tested; $\hat S = \{X\}$ in 8.0% of the rounds | one intercept under the null; empty set tested and rejected; $\hat S = \{X\}$ in 96% | the old test let the environment act directly on $Y$ |
| 9 | Unstable mechanism | coefficient doubled at every site (naive slope 4.0004), $\{X\}$ accepted in 94% | coefficient 0.5, 2 and 3.5 by site, every set rejected | the instability term did not depend on the site |
| 8 | Resolution of the constant forecaster | 0.0002 | 0.0000 | the boxes of equal mass split runs of identical forecasts |
| 8 | Platt recalibration | fitted and scored on the same 4,000 days | fitted on the first 2,000, scored on the last 2,000 | the book's own rule asks for scoring out of the fit |
| 8, 18, 20, 21 | Calibration tolerance by $n$ | "none of the six vaults passes" | all six detectable, Toronto 1997–2002 borderline (q95 0.0320 with its real forecasts, 0.0425 with simulated ones) | the verdict depends on the spread of the forecasts |
| 5, 8, 10, 12, 16, 21 | Loss of resolution of the post | 46.24 min (5 or 11), 5.97 min (7 or 9) | 22.99 and 2.50 min | both arms staffed at capacity equal to demand, a critical queue without steady state; now a utilisation of 0.8 |
| 16 | Panel band of a safeguard without anchor | severity 5 or 6, drawn by the seed (the seed decided the verdict) | 4 or 5, below the floor under every seed | a band that crosses the floor makes the gate a lottery |
| 12 | Health post, Step 1 and decision table | 9.3 and 20.7 min; failing days 26, 12, 3.7, 7.4 | 9.1 and 19.3 min; 30.9, 19.8, 4.9, 9.9 | the old figures came from the retired generator and engine; the chapter now cites its own instrument |
| 13 | Ruin leverage | $1/\min_t\lvert r_t\rvert$ | $1/\lvert\min_t r_t\rvert$ | the minimum of absolute values is near zero |
| 13 | 19 October 1987 | Friday | Monday | date |
| 14 | Tail index below 2 | "the long-run mean does not converge" | the mean exists and converges slowly; the variance is infinite; the layer caps payments | statement |
| 6 | Exercise on Simpson | cases 81, 234, 192 and 55 | cases 87, 270, 263 and 80 | successes printed as cases |
| 20 | Platt parameters | "a = 0.98, b = -0.54" | slope 0.98, intercept $-0.54$ | labels swapped against Chapter 8 |
| 18, 21 | Inward audit | "no published number fell by an error of arithmetic" | several did; this table lists them | the audit was run by the authors of the claims |
| C | Test count | 373 | 406 | stale count, plus the tests added with these corrections |

### Rectifications of the text, 2026-10-18 (third review)

A third reading of the same kind, made on the assembled book, found statements that no instrument produces and
therefore no gate could catch: definitions, attributions and readings of a formula. They are corrected in the text;
the earlier wording is kept here.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| Preface, 1 | What the seed proves | "an experiment is scientific only if someone else... obtains the same result"; the seed as the basis of the check | a fixed seed makes a computation reproducible, not replicable and not right; the lesson of the first table opens the book | reproducibility, replicability and validity were one word |
| 1 | Vulcan | the example of a degenerative repair | a progressive proposal whose prediction (transits) failed; degeneration is a trajectory of a programme | Le Verrier's Vulcan predicted observable transits |
| 8, 21 | "Resolution is not repaired" | stated of the forecaster | stated of recalibration; new information can raise resolution | the asymmetry belongs to the map from $p$ to $p'$ |
| 11, A | Chouldechova | approvals, errors and calibration "incompatible" | equal PPV, equal FPR and equal FNR incompatible for an imperfect predictor; calibration within groups is a neighbour | approval parity is not part of the identity |
| 15 | Type of the Erlang C node | Knightian (specification) | epistemic (specification) | its failures are listed and testable; Knightian is the unlisted mechanism |
| 15 | Pathology "illusion of the dark room" | name | "case for population" | the dark room is another problem of active inference (Chapter 11) |
| 15 | Bridge "noise as temperature" | a bridge of the same rank as Jensen | marked heuristic | the gradient noise is anisotropic and state-dependent |
| 16 | Panel score | a severity from 0 to 10 | read as one bit per safeguard | the band is drawn by the seed and never crosses the floor |
| 16, 21 | Floor of 1% to 99% | a prohibition on every forecast | a convention of System 1's triage; forecasts declare $\varepsilon$ from the base rate | Cromwell's rule forbids 0 and 1, not 0.5% |
| 17 | Stratonovich and the drag | "the term $-\sigma^2/2$ does not appear" | the gap of $\tfrac12\sigma^2$ appears in both readings; the convention changes what $\mu$ names | Jensen does not depend on the integral |
| 21 | Physics of the noise | "added (Itô) or dragged (Stratonovich)" | the two readings coincide for additive noise and differ for state-dependent noise | definition |
| A | Type M error | "3 centimetres of rain when it was 3 millimetres" | a small study that reports a doubled effect | a unit error is not a magnitude error |
| A | ICP | returns "the set" of invariant variables | returns the intersection of the accepted sets, with a declared error rate | definition |
| A | Erlang C | "the queue does not exceed five people" | a share of customers served within a time | Erlang C gives the chance of waiting, not a bound on the queue |
| 1, 2 | Exercise type | *(Calculation)* | *(Count)* | the preface declares three types |

### Rectifications of 2026-10-19

Two kinds of change. The safeguard panel stopped printing a number it did not measure, and a reimplementation of the
decision laboratories from the text alone (now `scripts/reimplementation/ch10_ch11.py`, in Python, written without
reading the Rust of Chapters 10 and 11) found two values that the instruments printed faithfully and that were
wrong. Each was corrected in the code, given a test that fails on the old code, and corrected in the text. The
reimplementation now runs in `scripts/verify.sh`; it was written by an AI assistant, so it is a cross-check, not
the independent reimplementation that `docs/OUTSIDE_AUDIT.md` asks of people.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 7, 16, 17 | Output of the panel A1–A8 | a severity from 0 to 10 per agent, drawn by a seed inside a range that the bits selected; floor 6; G6 "worst 8" | two bits per agent (safeguard, anchor) and their AND; no seed; G6 "96 of 96 checks declared and anchored" | the number carried nothing beyond the bits and borrowed Mayo's word |
| 7, 16, A, and the repository | Name of the panel | "adversarial panel", "adversarial board", "review board" | "safeguard panel" | it names attacks and checks safeguards; it runs no attack |
| 10 | Twice Kelly above the start | 0.485, 0.527 and 0.51960 after 10, 100 and 1000 rounds | 0.374, 0.450 and 0.49110 (exact: 0.377, 0.460 and 0.487) | 1.25 and 0.8 multiply to exactly 1; the comparison `> 1.0` let the rounding of each product decide the ties |
| 10 | Share of the true mean above the maximum of a sample of one million, in the median | 25.12% | 27.55% (median maximum 397,420) | the laboratory printed $n^{1-\alpha}$, which puts the maximum at $n$; the median maximum solves $(1 - m^{-\alpha})^n = 1/2$ |
| C, E | The numpy case | the upstream change "still owed" | NumPy 2.5.0 release notes, gh-31238 | checked on 2026-10-19 |
| 10–21 | Exercise types | *(Computation)*, *(Account)* | *(Count)* | the preface declares three types |
| C | Test count | 406 | 409 | the count of 2026-10-18 was already stale (408 on the base of these changes); one panel test retired, two tests of Chapter 10 added |

### Rectifications of the reimplementation of the chapters, 2026-10-19

The reimplementation was extended, the same day, from Chapters 10 and 11 to twenty chapters (`scripts/reimplementation/`,
run by `run.py` in `scripts/verify.sh`): 421 quantities, recomputed in Python from the text and the data files without
reading the Rust of each chapter, 402 in agreement and 19 marked open because the text names a procedure without
giving its recipe. The Rust was read only after a divergence, to say where it came from. Six values were wrong in the
instruments, and two statements in the text; each is corrected, with a test where there is code.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 5, 8, 12, 16, 21 | R horizon: wait of the coarse and the fine plan, and the loss | 5 or 11: 38.94, 15.95 and 22.99 min; 7 or 9: 19.17, 16.67 and 2.50; 8 or 8: 16.72 | 43.73, 18.20 and 25.54; 21.15, 18.38 and 2.77; 18.52 | the day had a fixed count of arrivals, each wait included the client's own service, and 200 days left a spread of about 2 minutes under two printed decimals; now a Poisson day of 10 hours, the Lindley recursion in its right order, 4,000 days. The decision (refine to the day) does not change |
| 5 | Mean of W over a uniform demand between 5 and 11 | 14.10 min | 14.46 min | a six-point midpoint rule; the integral has a closed form |
| 15 | EVPI of the sensor | 0.0000 with the option of acting, 3.7000 without it | 1.2000 with the full stake, 0.6000 with half of it, 0.0000 for a payoff that does not depend on the state | the payoffs of one act were passed where the best payoff of each state was due; the text said the sensor never changes the decision, and it changes it in every bad year |
| 12 | Reinforcement of the surge days | 2.5 min, p90 8.2 | 1.9 min, p90 6.8 | the text says two nurses; the instrument ran one nurse at twice the pace. Chapter 5, whose text says the capacity doubles, keeps the 2.5 |
| 12 | Halves null of the bank, per day | 0.0441 | 0.0743 | the sd of the absolute differences, about 0.6 of the sd of the differences |
| 13 | Drag predicted at 3x | 0.1377 | 0.1376 | rounding in the text |
| 19 | The 1-NN of the saturation rule | the measure was named, not specified | standardised columns and a rule for ties, written into the text | a reading with factors at distance 1 found 11 of the 16 saturated sets |
| 5 (R axis) | The day of predictions 3 and 5 | — | declared, not changed | `day_with_agents` keeps the fixed count and the order of the recursion with which predictions 3 and 5 were declared and measured; correcting it is a new specification, counted in K |

### Rectifications of the recipes, 2026-10-19

Of the 19 quantities that the reimplementation left open, the text now states the recipe of 16, in a paragraph beside
the run of each laboratory (Chapters 5, 7, 8, 13, 14, 15, 16, 17, 18 and 20), and the reimplementation redoes them from
that paragraph: 462 quantities, 459 in agreement, none divergent and 3 open (the upper end of the Kelly interval, the
skill of the model in the vault and the false negative rate of Hispanics in Chapter 13, whose recipes are longer than a
paragraph and stay cited from the research record at commit `1bfec905`). Writing the recipes found one more value
wrong in an instrument and one statement wrong in the text.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 7 | Garden of paths: Simmons' four freedoms, the large family, the Mayo severity of the garden | 21.5%, 47.0% and 0.53; PPV with the garden alpha 0.146 ($R = 1/10$) and 0.362 ($R = 1/3$) | 20.3%, 42.3% and 0.58; PPV 0.159 and 0.387 | the "exclusion of the extreme case" took the first $2m$ rows of the file, all of one group and part of the other, and split them at $m - 1$; it compared mixed groups. Now the first $m$ of each group, without the largest value of the treated. The direction (the garden multiplies the false positive) does not change |
| 13 | The inert treatment of LaLonde | "an **inert** treatment would have given +9.0" | a treatment with no effect, through the same weighting, falls between −10.3 and +11.9 in 95% of the replicas, centred near +0.9; +9.0 is the weighted estimate of the real data, inside that band | the text read the observed estimate as the result of the placebo |
| 13 | Selection bias of 1978 | 10.292 dollars per person, in three places | 10,292 dollars per person, as in Chapter 9 | a decimal point where a thousands separator was due |
| 18 | Merkle root | — | declared, not changed | repeating the last hash of an odd level gives a list and the same list with its last element repeated the same root; the gates record the count beside the root, and a test fixes the limit |
| C | Test count | 413 | 415 | the bounds of the garden of paths and the declared limit of the Merkle root |
| 8 | Brier of the overconfident forecaster | — | declared, not changed | the instrument's own seed spread (seeds 2022 to 2027) is 0.1706 to 0.1914; the printed 0.1712 is the low end, and the reimplementation compares within that spread |

### Rectifications of a critical reading, 2026-10-19

A critical reading of the assembled book, made with an AI assistant, compared each chapter with its own output blocks,
with the rest of the book and with what the reader can run. Like the readings above it is outside the authors' audit
and is not the independent audit that Chapter 21 asks for. No instrument changed; every change is in the text.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 2, 5 | Table of the two worlds in Step 1 | 9.3 and 20.7 min; 8 and 25 in every 100 above 30 min | 9.1 and 19.6 min; 8 and 24 | the table kept the figures of the retired generator while the block of Chapter 5 prints the instrument's (Chapter 5 section below) |
| 15 | Exercise 6 | "why was Erlang C typed as knightian and not as epistemic?" | asks why it is epistemic and not Knightian, and what would move it | the node was retyped on 2026-10-18 and the exercise was not |
| 13 | The worst entry month | "had a mean return equal to that of the market and did not survive to see it" | the order of the returns, not their mean, ruined the cohort | the sentence named no quantity |
| 14 | Mutual information of the clean window | 0.51 bit, "practically nothing" | about half of the one bit that two equally likely hypotheses allow | 0.51 of a ceiling of 1 is not nothing; the likelihood ratio, 2.2 bits, carries the lesson |
| 13, 14, 19, 20 | Thousands separator in running prose | 31.919 sessions, 1.488 cases, 11.598 students and others, 37 places | 31,919, 1,488, 11,598 | a decimal point where a thousands separator was due, as in the row of 1978 above |
| 19 | Verdict of the causal gate | table and summary said "Passed" | the table and the summary add that under the first declared version of each choice the gate does not pass | the end of Step 2 said it; the headline did not |
| 20 | Calibration criterion of Rhode Island | relative tolerance stated without comment | declared as a change of ruler after two failures, counted as an attempt; it tightens, since 0.03 exceeds the rate | a change of criterion is an attempt whatever its direction |
| 21 | Lessons 5 and 7 | invariance and pre-registration as successes | two of the areas recovered known causes; declaring makes the path visible, not innocent | the lessons carried less than Chapter 19 concedes |
| 21 | Negative loss below the threshold of prediction 3 | "after costing less than nothing" | the two arms do not spend the same resource: 13 agents every day against the per-day Erlang scale | the column compares two plans, not the equal-resource toll of Chapter 5 |
| Preface | What the reader cannot run; status of the bridge thesis | the rule "every number comes from an instrument" without its exceptions | the exceptions listed, and the thesis named a working hypothesis | the rule did not hold for the real cases of Chapters 13, 14, 19 and 20 |
| D | References cited and absent from the list | 26 works cited by author and year | listed in D.5, marked [to check] | the check of 50 of 50 covered the list, not the text |
| D | Check of D.5 | 26 lines to check | 24 checked against Crossref, arXiv and PMLR; Cover and Thomas, and Platt 1999, still to check | the metadata were written from the reading |
| 15, 16, 17, A | Names of the five lenses | Fluxonomy, Chrononomy, Scalonomy, Morphonomy and Crisonomy as the names | information, time, tail, invariance and decision, with the coined words kept as the labels the outputs print | a name that needs a translation table in its own chapter costs the reader and adds no mathematics |
| 12 to 18 | Order of the chapters | the machine (then 12 to 15) before the three real cases (then 16 to 18) | the cases right after the theory (12 to 14), the machine after them (15 to 18); see the numbering above | the method can be read without the machine; the old order made the reader cross the machine to reach the cases |
| Title | Title of the book | "Stochology: A Textbook" | "Stochology: Judgement and Decision under Uncertainty", a working text | a textbook is what the outside audit of Chapter 21 would certify; it has not been done |

### Rectifications of the remaining points, 2026-10-19

A last pass of the same day, on the points the critical reading had left open. Text, the reimplementation and the
registry; no instrument changed.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 3 | Null paths reaching 20 decibans | 0.85% "in simulation", no horizon | 0.87% over paths of 500 tosses, exact | the simulation had no declared horizon; the exact count needs one |
| 3 | Dutch book and calibration | 0.05, 0.60 and 0.55 | the argument without the numbers | no recipe in any source |
| 7 | Brier of the base-rate heuristic in a referral clinic | 0.084 | 0.085, with the recipe (mammography test, prior one half) | the recipe gives 0.0851 |
| 10 | Horizon of the ensemble mean | about 12 rounds for 10 trajectories and 112 for a million | 23 and 136, by ln N over the gap of the two growth rates | the earlier pair does not grow like ln N, which the sentence claimed |
| 10 | Full Kelly on an estimated chance | 16% | 15%, from 18 of 25, exact binomial | the recipe gives 15.4% |
| 10 | Extremes against Gaussian | 17.4% against 3.9% | the argument without the numbers | no recipe in any source |
| 11 | Two-route app; Phillips curve | 41.0, 36.8, 35.0 and 0.3 min; 4.59% and 5.88% | the argument without the numbers | no recipe in any source |
| 17 | Kramers jumps; identifiability of R0 | 591 jumps, 99%; 1.38 to 5.90, 2.00 to 2.24, 1.81 to 2.11 | the argument without the numbers | no recipe in any source |
| 13, 14 | Paths of the research record | written in the chapter | moved to the sections of Chapters 13 and 14 below | the chapters carry content only |
| 14 | Floor of the claim's forecast | "the floor of the forecast diary" | points to Chapter 18, where the diary is defined | the reordering put the diary after the chapter |
| 19 | Title | "Seven rounds to find true causes in science" | "Seven rounds in search of invariant causes, and what they found" | the title promised what the chapter concedes it did not deliver |
| A | Order of the dictionary | the order of the Portuguese original | alphabetical in English | the table says it is alphabetical |
| D | The last references to check | Cover and Thomas, Platt 1999, and the D.2 line on scoring rules | all checked; Platt against a secondary source | 77 of 77 lines checked |

### The real streams, 2026-10-19

A new instrument: six streams of Chapter 20 rebuilt on public data, with the record and the rebuild side by side in
the section of Chapter 20 below. Following the rule of the Preface, the chapters teach with the instrument's numbers
and this appendix keeps the record's: Chapters 8 and 20 were brought into line the same day.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 20 | Table of the six streams in Step 1 | the record's numbers (coverage 0.887 and 0.900; VaR 0.98%, passes; flights 2013 bias +2.7 points, fails; cyclones ECE 0.012; Toronto ECE 0.036) | the instrument's (0.886 and 0.884; VaR 1.53%, fails; flights 2013 CITL −0.0171 and ECE 0.0260, passes; cyclones 0.0121; Toronto 0.0354) | the chapters teach with what the reader can run |
| 20 | The first failures | the circuit breaker of the flights of 2013, which compared the model with the reserve | Toronto, which beats the climatology and fails calibration; the market, whose crossings cluster | in the rebuild the flights of 2013 pass and the market fails |
| 20 | The 2023 correction | "was not put to the test": the raw model would also have passed, ECE 0.014 | decisive: the raw model fails (ECE 0.0646) and the corrected one passes (0.0177); in 2013 the correction fixes the bias and fails on the ECE | the instrument's reading |
| 20 | Canary, fairness of Toronto 1997–2002, summary, exercise 2 | the record's gains and rates | the instrument's, with the record's kept where nothing was rebuilt (2020–2021, Rhode Island) | the same rule |
| 20 | Step 2: the replay count, the Goodhart index, the label drift, the inverted audit | stated as measured | marked as the record of the research | no instrument redoes them |
| 8 | Table of the six binary vaults | the record's ECE and null q95 (0.0267/0.0046, 0.0123/0.0087, 0.0363/0.0320, 0.0079/0.0035) | the instrument's (0.0260/0.0046, 0.0121/0.0107, 0.0354/0.0315, 0.0177/0.0034 with the correction); Toronto 2020–2021 and Rhode Island marked as record | the same rule |
| 8 | The correction in a new vault; exercise 6 | "was not decisive"; 0.0363 against 0.0079 | decisive; 0.0354 against 0.0177 | the instrument's reading |
| C | Test count | 415 | 422 | the seven tests of the rebuilt streams |
| Preface | What the reader cannot run | the flights, the cyclones and the arrests among the citations | rebuilt by `labs --streams`; the arrests of 2020–2021 and the traffic stops stay cited | the data now sit beside the book |

### The desk work before the public, 2026-10-19

No number changed. What entered was the record of who wrote what (Preface), the neighbours of the discipline and
what is borrowed from each (Chapter 21 and D.6), the stopping rule of the transfer gate read under an e-process
(Chapter 19, `labs --eprocess`, with the declaration sealed before the run in `empirical/p22-parada/DECLARACAO_2.md`),
and an answer key for the 126 exercises, published on the site beside the chapters and not assembled into the book.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| Preface | Who wrote what | not stated | a paragraph: the theory, the architecture, the research and the first text are the author's; from 2026-10-18 an AI assistant read, reimplemented, rebuilt and wrote the corrections, inside the author's tools and checked by no one outside | a reader has the right to know |
| 19 | The stopping rule of the seven rounds | Bonferroni over 38, chosen after the path | the e-process over the 38 recorded tests: three areas cross, genetics does not, education stays in by the mean of its earlier rounds | the declaration admitted the clustered rows; the mean carries the rounds left as recorded |
| 21, D | The neighbours of the discipline | the first objection answered in a table row | model cards, datasheets, pre-registration, forecasting tournaments, risk communication, the Bayesian workflow and the open research culture, each with what is borrowed; eight references checked | a discipline that did not name its owners would be claiming what it borrowed |
| C | Instruments and tests | 21 instruments; 422 tests | `labs --eprocess` added; the five tests of `eprocess.rs` | the new instrument |
| Exercises | Answers | none published | `book/answers.md`: every Count answer with its computation, every Laboratory answer with what to change and what to see, numbers only where measured | the course had a key; the book did not |

### Uncertainty propagated to the decision, 2026-10-19

Text added after a reading of the propagation of uncertainty, rebuilt with the book's own instruments and sources.
No number changed.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 5 | Three answers to the same question | the toll of Jensen measured; no comparison of methods | plug-in, delta method and Monte Carlo on the health post with uncertain, correlated rates; the uncertainty budget; the rule on when the linear answer may decide (`labs --propagate`) | a tangent does not see a tail, and the decision read the tail |
| 5 | Which summary answers | mean, median and quantiles listed | each tied to the loss it minimises | choosing the summary is choosing whose error counts |
| 6 | The effective sizes | several, each with its own story | one formula, the variance of a sum with its covariances | leaving the covariance out is a claim about the world |
| 11 | The expected cost | the threshold and the value of information | written term by term, with the conditional value at risk | frequency and severity are different axes |
| 14 | The higher moments | the tail read through Hill and quantiles | why: the higher the moment, the louder one point; why an extreme is never removed for being extreme | the instability of the higher moments |
| D | References | — | GUM, the NIST/SEMATECH e-Handbook, Rockafellar and Uryasev | checked against the publishers |

### The card contract 1.3, 2026-10-19

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 17 | The fields of the card | the header and the fields of contract 1.2 | two optional fields, the uncertainty budget and the dominant event, validated when present; the migration from 1.2 introduces them absent | the uncertainty that reaches the decision has a source and a carrier, and the card did not ask for either |

### The research on models of the world, of agents and of modelling, 2026-10-19

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 2 | The layers | the Model judged by its errors everywhere; the modeller outside the loop | a Model good enough for the decision (value equivalence), an Agent that exploits its Model's errors, and the modeller as an Agent whose World is the space of Models | the research on world models and on many analysts with the same data |
| 4 | The law of total variance | the two parcels of a parameter | also the ceiling a person sets (a model of a person scored against the person's own consistency) and the simulated sample that loses its within-group parcel | the research on agent models that simulate people |
| 5 | The health post | three answers under one specification | a fourth: eighteen specifications with common draws, of which eight ask for the second nurse; the service cv carries 0.93 of the spread (`labs --multiverse`, sealed in point 26 before its run) | the uncertainty of an answer includes the choice of the model |
| 8 | Proper scores | rules that judge a forecast | also a rule that combines models: stacking against model averaging, with the laboratory of two confident experts (`labs --stacking`, sealed in point 27 before its run) | the research on models of modelling and the M-open setting |
| 14 | Failing one's own model | the tail model failed on purpose | also a parameter failed by its own model: calibrated without discrepancy it is learnt wrong with a narrow interval; with a discrepancy that is zero and flat at the origin it comes back (`labs --discrepancy`, sealed in point 28 before its run; a second declaration added the rms error at twice the range, after a drafted reading took a small mean error for a good prediction, never published) | the research on models of modelling and on emulators |
| 16 | The machine | System 2 woken by margin; agents judged by average success; the panel without a map of how teams fail | the value of computation; the fourteen failure modes of teams of agents held against the panel (four caught, seven in part, three not seen; `docs/MAST_AUDIT.md`); reliability as pass^k, with the laboratory of four worlds (`labs --passk`, sealed in point 25 before its run) | the research on resource rationality and on agent benchmarks |
| 17 | The card | contract 1.3 | contract 1.4: the multiverse and the reliability, optional, validated when present | the two laboratories measured what the card had no place for |
| 19 | Invariance | causes across environments | also models of other minds, tested by alterations their mechanism declares irrelevant or decisive | the research on theory of mind in language models |
| E | Chapter 8 | "the instrument measures 0.1712 → 0.1559, 0.1560 and 0.2119 → 0.1755" | measured until 2026-10-18 in sample; out of the fit it measures 0.1708 → 0.1555, 0.1547 and 0.2147 → 0.1741 | the provenance check that closed item 16 found the paragraph speaking of the old reading in the present |

### The seed of the simulations, 2026-10-19

A new instrument, `labs --mc-error`, measures how far each simulated number that a reading depends on moves with the
seed (Appendix C.3). No number changed and no reading moved; two published values were found at the edge of their
seed spread, and the chapters now say so.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 8 | The null q95 at 1,488 cases | 0.0425, as if exact | 0.0425, the largest of twenty-one seeds; about 0.042 | a simulated quantile carries the error of its draws |
| 14 | The bootstrap interval of the tail index | [1.37; 2.61], as if exact | the same, with its upper bound above every other seed's | the same |
| C | Reproduction | the environment is part of the number | also the seed: C.3 | the same |

### Numbers cited without an instrument in this book

The preface promises that every number comes from an instrument the reader can run or from a cited source. These do
not come from an instrument beside the book; most come from worked examples of the paper and of the obsolete course,
others from the research records. The critical reading of 2026-10-19 recomputed three of them from the text, and a
later pass the same day wrote a recipe for five more, reproduced in `scripts/reimplementation/recipes.py`, and cut the
numbers of the six that could not be reconstructed, keeping the argument.

| Chapter | Number | Where it comes from | Status |
| --- | --- | --- | --- |
| 3 | Dutch book: 0.05 per round, 0.60 to the world, 0.55 after coherence | course example in the paper | not reconstructable from the text; the numbers were cut and the argument kept |
| 3 | 229 tosses to separate a 0.6 coin with odds of 100 to 1 | Kullback-Leibler | recomputed: 228.7 |
| 3 | Maximum entropy with mean 4.5: about 5% on face 1 and 35% on face 6 | Jaynes | recomputed: 5.4% and 34.7% |
| 3 | 0.85% of null paths reach 20 decibans | course simulation in the paper | recipe written: paths of 500 tosses, exact count 0.87% |
| 7 | Brier 0.083 and 0.009, 0.28 and 0.084 of the base-rate heuristic | course example in the paper | recipe written: the mammography test with a prior of one half; 0.083, 0.009, 0.28 and 0.085 |
| 10 | Mean of N trajectories follows the expectation for about 12 rounds (N = 10) and 112 (a million) | course example | recipe written: ln N over the gap of the two growth rates, 23 and 136; a simulation of the median ensemble mean agrees |
| 10 | Full Kelly negative in 16% of estimates from 25 rounds | course example | recipe written: even odds, chance 0.6, exact binomial, 15% |
| 10 | Three players who pool grow 1.6% per round | course example | recipe written: exact over the eight outcomes, 1.6% |
| 10 | 1-in-100-year loss 17.4% by extremes against 3.9% Gaussian | course example in the paper | not reconstructable; the numbers were cut and the argument kept |
| 11 | Two-route app: 41.0, 36.8 and 35.0 minutes, error 0.3 | course example in the paper | not reconstructable; the numbers were cut and the argument kept |
| 11 | Phillips curve: 4.59% promised, 5.88% delivered | course example in the paper | not reconstructable; the numbers were cut and the argument kept |
| 13 | Propensity fit, Kelly interval, skill in the vault, false negative rates | research record, retired code | recipe in the text for the propensity fit; the rest cited |
| 14 | Skills of the four forecasters in the vault; trend of 4.6% per year | research record, retired code | recipe in the text; data of the vault not beside the book |
| 15 | Scale audit: 329 domains, 26,634 nodes, 6,494 concept nodes | research record | not reproduced here |
| 16 | Five of 33 domains change with the literature prior | research record | not reproduced here |
| 17 | R0 between 1.38 and 5.90 with 25 days; Kramers, 591 jumps, 99% | course examples in the paper | not reconstructable; the numbers were cut and the argument kept |
| 19 | Every result on real data: chambers, cardiac trial, small classes, yeast | research record | not reproduced here; the laboratory is a simulation |
| 20 | Every stream, vault and fairness audit | research record | six streams rebuilt on public data by `labs --streams` (Chapter 20, Step 3), with the divergences in the section of Chapter 20 below; the arrests of 2020–2021 and the traffic stops stay cited |
| 21 | Loss aversion of 2.25 justified only with 56% of the wealth at stake | paper | recomputed: a loss of a share x needs a gain of x/(1 − x) to break even in growth, and 1/(1 − x) = 2.25 at x = 0.556 |

### Chapter 4

The earlier text (and the course outline) records 11% coverage
for the fixed model and 99% for the model with forgetting, with widths from 0.140 to 0.035. The instrument,
with 6,400 steps and discount 0.99, measures **12%** and **93.98%**, with widths from 0.1564 to 0.0240 and a
ratio of 5.7×. The coverage of the fixed model agrees within one percentage point; the coverage of the model
with forgetting is lower because the width of the interval used here is that of the standard error of the
EWMA, and a wider interval (or a smaller discount) raises coverage above 97% — the parameter grid in
`crates/stochology-book/examples/grade_ch04.rs` shows the effect. Both values remain recorded; neither was
erased.

### Chapter 5

The course material and the H0 README publish, for these worlds, 9.3 min and 20.7
min of mean wait, **8** and **25** patients in each 100 above 30 minutes, a margin of ±0.51 minute per day
against ±0.07 per patient, and a halves null of 0.216 minute with predicted margins of 0.227 (per day) and
0.047 (per patient). The instrument measures **9.1** and **19.6** minutes, **7.6** and **23.7** in each 100,
a null of **0.201** minute with margins of **0.185** (per day) and **0.046** (per patient) in the constant
world, and, in the oscillating world, a null of **0.742** with margins of **0.563** and **0.097**. The
uniform world gives 13.8 min here against 12.3 in the material. The numbers are all in a close neighbourhood.
The difference has two probable causes: the generator (the material uses NumPy's PCG64 with
`SeedSequence(1909)`; here, SplitMix64 with seed 1909) and the start of each day, which here is always an
empty queue. Jensen's identity, the sign of the toll and the conclusion about the unit of the margin depend on
neither of them. No old value was erased.

### Chapter 6

The earlier text records, for the same Berkson simulation,
a correlation of $+0.002$ in the population and $−0.669$ among the selected; the instrument measures
$+0.0012$ and $−0.679$ with cutoff 1.5 (and $−0.617$ with cutoff 1.0). The difference is one hundredth and comes
from the selection cutoff, which the old text does not declare. The 1936 numbers converge without adjustment. They are 2.4
million ballots, 57.4% for Landon against the published 57%, and the equivalence of 7 people. No old
value was deleted.

### Chapter 7

The earlier text records 73% of winners that "look
better than predicting zero", the 60.7% rate of the Simmons garden and a severity of 0.42 for the
$p = 0.04$ of the garden. The instrument measures **79.5%**, **20.3%** with the four freedoms (and **42.3%**
with the large family) and **0.58**. The numbers differ because the full protocol was not published: the
old garden swept more combinations — two outcomes, two sexes, more covariates, more optional stopping
points —, and every extra combination pushes the false positive up; in the garden of models, the
size of the validation and the range of predictors per candidate change the result, and here the winner still
fails on fresh data (error 1.191 against 1.000). What does not change is the direction: the pre-registered
test flags 5.2%, close to the nominal 5%, and the garden multiplies that by four or by eight. The
two values are recorded; none was deleted.

### Chapter 8

The earlier text records, for an example of 4,000 events, an overconfident
forecaster that goes from Brier 0.1956 to 0.1851 with Platt, an honest expert at 0.1846 and a noisy forecaster
that goes from 0.2154 to 0.2089. The instrument measured **0.1712 → 0.1559**, **0.1560** and
**0.2119 → 0.1755** until 2026-10-18, fitting and scoring the recalibration on the same days; scored out of the fit, it
measures **0.1708 → 0.1555**, **0.1547** and **0.2147 → 0.1741** (the provenance check of 2026-10-19 found the
paragraph still speaking of the old reading in the present). The world is not the same — here the observed base rate is 0.243, against the 0.5 of the
old example —, and the order of magnitudes is the same: recalibration takes the confident one to the level of the
honest one and leaves the noisy one behind. The ECE tolerance also diverges in level: H3 publishes a null q95 of
0.0320 for $n = 1,488$, 0.0034 for 24,466 and 0.0035 for 104,489; here they come out **0.0425**, **0.0043** and
**0.0037**. The difference is the distribution of the forecasts, which the README does not publish — with concentrated
forecasts ($n = 1,488$, spread 0.4) the null q50 of this instrument is 0.0320, the number of the
report. What matters does not change: the ECE noise moves with $1/\sqrt n$, and that is why a fixed tolerance
is tight in a small vault and loose in a large one. No old value was deleted.

### Chapter 10

The earlier text records 35.8% of ruin for whoever stakes by their own
history and 0% with Laplace. The instrument measures 36.8% of "all-in" and **0.00% of ruin** with
any of the rules. A declared lock explains why. Kelly is limited to $f \le 1$ (one does not
stake more than one has), and with $f = 1$ a trader who earns 1% per week and loses at most
half per loss needs seven consecutive extreme events to fall below $10^{-6}$, and those
events do not happen in 1,000 weeks with chance 0.004. The old value is recorded. The two numbers measure
the same thing, with different leverage ceilings. And the drag also diverges from the course script.
The table of five rules in Chapter 11 uses a tail model of its own, and the two sets of heights
do not coincide; both are reported.

The time-growth table of the empirical registry (5.1% at $1\times$, 7.2% at $2\times$ and 6.0% at
$3\times$, plus the drag of 0.14 per year at $3\times$; the same three portfolios appear in decimal
in Chapter 13, which measures 0.0515, 0.0719 and 0.0601) was recorded here until 2026-10-18 as not reproduced, with
$3\times$ above $2\times$, and the cause was attributed to the real tail. The cause was a calibration that dropped the
factor of 252 sessions per year. With it restored, the Gaussian model puts $2\times$ above $3\times$ and gives drags
that match those of the real index to the second decimal (see the rectifications above). The earlier attribution is recorded and withdrawn.

### Chapter 11

The earlier text records 35.8% of ruin for whoever bets by their own
history and 0% with Laplace (Chapter 10), and a table of dikes with heights of 4.76 m (expected
value with Gumbel), 6.20 m (mean of the models), 7.46 m (Wald), 5.93 m (Savage) and 9.96 m (precaution),
with 100-year levels from 3.11 to 4.26 m and 10,000-year levels from 3.81 to 9.96 m. The instrument, with
$\mu = 1.2567$ and $\sigma = 0.762$ calibrated so that the 100-year level of the model with $\xi = -0.1$
reaches 4.26 m, measured 3.06 m, 7.82 m, 12.58 m, 9.81 m and 21.49 m until 2026-10-18, when the Gumbel branch was
corrected (see the rectifications above). The old heights come from a
set of models calibrated in another way, which this laboratory does not reproduce; the two sets
stay recorded. In the trilemma, the old value of 1 273 false positives for equalising the FNR is from the
control table of H1; this laboratory measured $+628$ until 2026-10-18, with a map from FNR to FPR that moved both errors
the same way, and now measures the trade-off of the rectifications above, because the computation here starts from the
rates by group instead of the units. The divergences are declared and no old value
was erased.

### Chapter 12

The earlier text and the README of the first simulation record 9.3 min and 20.7 min
for the two worlds, 8 and 25 patients in every 100 above 30 minutes, medians of 3.8 and 9.3, 90th percentiles of 26.7
and 59.3, and, in the halves null, a measured sd of 0.216 against 0.227 predicted by the day and 0.047 by the call (severe
test: 11.4 min, 53 sd, ceiling of 0.81 in 2,000 splits). The instrument measures **9.1 and 19.3**, **8 and 24**
in every 100, medians of **3.7 and 8.1**, 90th percentiles of **26.1 and 55.8**, and, in the null, **0.2062** measured against
**0.1928** by the day and **0.0473** by the call (severe test: **10.2 min**, **49.4** sd, ceiling of **0.664**). The
margin per patient coincides up to the third place, because it is a standard deviation account over the number of patients.

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
0.0441 against the 0.0474 published (different splitting scheme; since 2026-10-19 it is 0.0743, because the 0.0441
was the sd of the absolute differences, see the rectifications of 2026-10-19), and the null per call of this account is 0.0006
by the binomial, against the 0.0033 of the script, which used another design of intraday correlation. The two
divergences stay declared, together with the Rust reconstruction of the decision table of the schemes R0 to
R3 by the queue simulation with abandonment (`fila.rs`, measured just above with `labs --queue`).
The other values differ because the number generator is another (SplitMix64 here, PCG64 in the original material) and
because the mean wait per patient in the oscillating world is a ratio of heavy-tailed sums. Running the instrument
with six different seeds, the wait of the world 5 or 11 goes from **19.3 to 20.6 minutes**. The difference
is inside the variation of the instrument, and the two sets of numbers stay on the record.

### Chapter 13

**Where the cited numbers of this chapter live.** The numbers that the text attributes to the research record come
from `empirical/p05-prioris/h1-real/README.md`, computed by code retired from the tree on 2026-10-16 and kept at
commit `1bfec905` (`fundo/p05-prioris/h1-real/projetos/`, `a_real.py` and `b_real.py`). Until 2026-10-19 these paths were written in the
chapter; they moved here because the chapters carry content only.

The report of the market case records 0.051, 0.072 and 0.060 of growth per year;
the instrument measures **0.0515, 0.0719 and 0.0601** — the coincidence is of three places, because the account is the same. The
published cohorts with a monthly start are 1,135 and the ones here are 1,134 (the difference is the border criterion: one cohort
whose end falls on the last session of the training set). The published values of the fraction above the start are 0.9586 / 0.7841 /
0.6405; the ones here are **0.9577 / 0.7848 / 0.6429**, and the published median wealths (2.13 / 2.68 / 2.08) match
the ones here (2.13 / 2.67 / 2.10). In the claim world, the published material brings mean 0.9332, 1% quantile of
0.6563 and p = 0.0095, against **0.9286**, **0.6354** and **0.0140** here: the difference is in the draw of the
resamplings (2,000 replicas in each case, different generators), and the p-value changes in the third place. One different
case deserves a note: the earlier text cites "from 1929 to 1932, the capital falls to 0.11 with 1× and to 0.00025 with
3×". With the window of the **calendar year** (1 January 1929 to the end of 1932), the instrument measures **0.19521** and
**0.00088**; with the window of the **peak to the trough** (3 September 1929 to 8 July 1932), it measures **0.10814** and
**0.00025** — exactly the numbers of the material. The divergence was not of method: it was of window, and the two
appear now, with the date of each one.

### Chapter 14

**Where the cited numbers of this chapter live.** The numbers that the text attributes to the research record come
from `empirical/p05-prioris/h1-real/README.md`, computed by code retired from the tree on 2026-10-16 and kept at
commit `1bfec905` (`fundo/p05-prioris/h1-real/projetos/`, `c_real.py`). Until 2026-10-19 these paths were written in the
chapter; they moved here because the chapters carry content only.

The report of the fire case records $\hat\alpha = 1.81$ with interval [1.41;
2.56] by bootstrap in blocks; the instrument measures the same **1.8109** at the point (k = 50) and the interval **[1.37;
2.61]**, besides 1.9625 and 1.4100 for k = 25 and k = 100 — the three points coincide with the published ones up to the fourth
place, and the difference of the interval comes from the resampling (300 replicas, different generator). The divergence that deserves
a paragraph is another one: the earlier text and the README say that 36 clean months are worth **2.2 bits** and that a severe test
would need **69 months**. The instrument confirms the two numbers **exactly** (2.2373 bits and 69 months)
when it uses the $p$ that the tail of the training set implies, $p = 0.04296$, the value of the published POT/GPD fit
$\xi = 0.406$, $\sigma = 7.387$, 83 excesses above 10 — and measures **2.42 bits and 64 months** when it uses the
simple monthly frequency of the training set, $p = 5/108 = 0.04630$. The difference is not of account: it is of which world
"dangerous" one places on the other side. The adjusted tail is more pessimistic than the raw count of months, and that is why it
requires more clean months. The two values stay on the record; none was erased.

### Chapter 15

The earlier text, in Chapter 6, presents the three samples of
Hill with values close to $1.81$; the instrument measures 1.8444 ($k = 250$), 1.7901 ($k = 500$), 1.7949
($k = 1,000$) and 1.8032 ($k = 2,000$). The difference is one of sampling and of cutoff, not of formula — the value
published in the repository for $k = 500$ with seed 1909 reproduces exactly. It is recorded as well that the
earlier text and the plan of the repository count gates in different ways: the old book describes
**five** automatic gates (contracts and seeds; schema and idempotence; convergence; temporal cutoff;
governance, fairness and exit), while the `gates` binary implements **seven** (G1 determinism,
G2 fixed seed, G3 declared scope, G4 invariants and type III, G5 integrity of the trail, G6 panel
$A_1$–$A_8$, G7 exit clause), with exit code 0 or 1. Chapter 17 details the seven; no old value
was erased.

### Chapter 16

The earlier text speaks of "eight tracks and an orchestrator M15". The
text of the repository (`docs/PLAN.md`) calls the orchestrator the orchestrator and the implementation
(`OrquestradorM`) as well; the label M15, like the MB labels of the panel, is historical and is declared in
`AGENTS.md` §VIII, along with the ME numbering of the script. They are the same object, and this book uses the orchestrator
because that is the name of the code. The laboratory of this book measures the
severities from the **eight concerns of the card** (true/false flags) and not from the attacks themselves, and the
machinery of the repository does the same (the review of 2026-10-18 states it in the module itself):
running the attack is the machinery of the repository, and the number the panel publishes is the one of the severity. For that reason the
values 8, 9 and 10 here reproduce the worst = 8 of the gate, and not a report of an executed attack. Since
2026-10-19 neither prints a severity: the laboratory and the gate print the bits (rectifications of 2026-10-19).

### Chapter 17

Two accounts of this chapter diverge between the sources, and both stay
recorded. (1) The previous book describes **five** automatic gates — contracts and seeds; declarative schema
and idempotence; convergence of the chains; temporal cutoff; governance, fairness and exit —, while
the `gates` binary implements **seven** (G1 to G7). The difference is one of grouping: gate 3 of the old
book ("convergence of the chains") disappears from the binary, and gate 5 splits into G5, G6 and G7. The two texts
count the same pipeline in different ways; no list was erased. (2) The previous book and the plan of the
repository say that "in $H_0$ the pipeline applies $\Phi_1$–$\Phi_4$", and the laboratory measures exactly
that: a complete card generates zero invariant failures, and each defect generates one. The number "failures = 0" of
gate G4, in the horizon `todos`, reproduces here.

### Chapter 18

Two points stay recorded. (1) The plan publishes, for the
horizon `todos`, the Merkle root `9716e28a87bec9cf...` over the 12 cards of the four
horizons, and a hash of the report `08e7c8f0...`; the laboratory uses its **own** chain of
five textbook events, and for that reason the root measured here is `92f2ab76...`, not that one. The two
accounts are correct for their contents; comparing the two only makes sense if the input is the same, and it is
exactly in that way that the clean reproduction of G1 works. (2) The previous book says that the first run of the
support desk hung because the Erlang C written with factorials overflowed at large loads; the swap for the stable
recursion of Erlang B only affects the decision, not the forecasts, but it changes the hash of the module and **counts in $K$ (3)**.
No old value was erased.

### Chapter 19

The numbers above are of data **simulated** by this laboratory, and not of the
research repository; the three simulations show a single mechanism, at toy scale. The values of the
repository cited in this chapter (128 of 1,024 accepted sets in the light tunnel with $p = 0.29$ for
$\{$red, green, blue$\}$; 35 of 512 in the FDA trial; 57 accepted sets in STAR in H2d; 2 of 1,024 in
the yeast with best $p = 0.0072$) are not reproduced here, and cannot be: they require the real data and the
framework in Python of the horizons, whose commands are in Appendix C. The laboratory measures what it can with
its own data: that the method finds the two causes in world A, that it returns empty in world B and that the rule of
saturation is the difference between $\emptyset$ and the truth in world C. The design of the validation by intervention of the
yeast is the exception: the resolution of the replica is measured again, in Rust, by `labs --genetics`, with the true mover
and the row artefact planted by the instrument — what the instrument measures there is the **design**, and not
the 40 real genes, which continue without an instrument.

### Chapter 20

**The streams rebuilt on 2026-10-19.** `labs --streams` redoes six streams of the record on public data under a
specification written in the chapter before the run. Side by side, record and rebuild:

| Stream | Record | Rebuild | Reading |
| --- | --- | --- | --- |
| Energy, 2014 | coverage 0.887, passes | coverage 0.886 (17 424 half hours), passes | the same |
| Bank, Sep–Oct 2003 | coverage 0.900 (6 084), passes | coverage 0.884 (6 422 five-minute slots), passes | the same verdict; the vault counts 38 days here |
| Market, 2000–2023 | violations 0.98%, Kupiec p = 0.89, Christoffersen p = 0.02 | violations 1.53%, Kupiec p = 0.00, Christoffersen p = 0.02 | **diverges**: the record does not state its method; historical simulation over 250 sessions fails |
| Flights, Q4 2013 | bias +2.7 points, ECE 0.027, fails | raw: CITL −0.0171, ECE 0.0260, passes; corrected: ECE 0.0322, fails | the sign of the bias agrees (observed above forecast); the verdict depends on the model |
| Flights, Q4 2023 | corrected ECE 0.008, raw ECE 0.014, both pass | raw: CITL +0.0633, ECE 0.0646, fails; corrected: ECE 0.0177, passes | **diverges**: in the rebuild the correction is decisive |
| Cyclones, 2015–2022 | bias 0.001, ECE 0.012 (5 766), passes | CITL +0.0002, ECE 0.0121 (3 813), passes | the same; the rebuild keeps only observations with another one 24 hours later |
| Toronto, 2001–2002 | bias 0.016, ECE 0.036 (1 488), fails | CITL +0.0165, ECE 0.0354 (1 488), fails | the same |
| Toronto, marker by colour | FPR 0.16–0.23 Black against 0.06–0.11 White | 0.155 and 0.164 against 0.070 and 0.077 | the same pattern |
| Canary | 0.000 honest; +0.55 to +0.97 switched off | 0 emissions, +0.0000 honest; +0.91 to +0.97 switched off | the same |

The reimplementation from the text (`scripts/reimplementation/ch20_streams.py`) redoes Toronto, the market and the
energy interval from the recipe alone and agrees with every number of the rebuild.

**The Platt balance.** The record of the live replay says that, in the flights stream of 2023, the
intercept of the correction varied from $-0.52$ to $+0.43$ along the year. The diary of 2023 shows that this is the
interval **inside the vault**; in the whole year, the same intercept goes from $-1.01$ to $+1.25$. The two values
are right and measure different things: one is the vault, the other is the year. It stays on the record instead of hidden.

**The numpy case.** The published number of the performative loop — estimated effect
$0.5632$ with 2,375 shocks — was contested once, and it is worth telling how it ended. The code, with the seed
declared, gave $0.5529$ and 2,395 shocks when it ran with the standard `python3` of the machine. There were not two versions
of the code. The file is one, and the four copies of the tree bring the identical cryptographic digest. What there was were
two numeric packages inside a single interpreter: numpy 2.5.1 of the record and numpy 2.4.6 that the standard path
imports first. The exact point of the divergence was found: with the state of the generator identical, a binomial draw
of 248 trials with probability 0.6 returns 171 in one version and 150 in the other, because version 2.5 corrected the
Stirling series of the acceptance test of BTPE — the third and the fourth terms were added instead of subtracted, and the
main coefficient said 13680 where it should say 13860 (NumPy 2.5.0 release notes, gh-31238, checked on 2026-10-19). From there on the two simulations decorrelate, and the
exploration draw changes pattern. **No published number changed**: the r̂ of 0.5632 with 2,375 shocks reproduces
in the record environment, and the other value is the one of the other package. The lesson for whoever goes to check: record the version of the
environment together with the number, because a number without an environment is a number that nobody can audit.

The experiment above is simulated and was calibrated to have the structure of the vault of
Rhode Island — rate of 2.26%, 24,466 stops, correction fitted in the previous period —, not to reproduce its
numbers. The values measured here and the ones of the real vault, side by side: bias of the raw model, $+0.0132$
here against $0.013$ there; bias with Platt, $+0.0002$ here against $0.0020$ there; ECE with Platt, 0.0022
here against **0.0048** there; skill with Platt, $+0.0098$ here against **$+0.019$** there; excess of the rate by the
raw model, 60% here against 58% there. The order of magnitude and the sign of all of them coincide, and the reason for the
differences is the data: the one here is generated by a simple additive model, with three covariates and without the noise of a
real police record. The gain of the canary with the clock switched off, $+0.6521$, falls inside the range measured in the
real vaults, which goes from $+0.55$ (arrests of 2020–2021) to $+0.97$ (Rhode Island).

### Chapter 21

The verifier of the laboratory is a reduced reimplementation of the validator of the
repository. The validator of the research demands, besides the rules above, at least 60 characters in the text of the question,
refuses a larger list of generic names and runs over 18 packages of H2g, 12 of H2 and the card of H0, without any
published number changing; the one of the laboratory runs over 12 test cards and touches nobody's data. What
the two have in common is the essential: the decider question is only worth something if it names who decides, copies the action
literally and ends in a question.
