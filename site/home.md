# Stochology: from ancient conjecture to the audit of judgement

**Stochology** is the science (or art) of judgement and decision under uncertainty. The name comes from the Greek
*stókhos*, the target an archer aims at and, by extension, a well-aimed guess. In 1713, eight years after his death,
Jacob Bernoulli's *Ars Conjectandi* was published: the art of conjecturing, which he called *stochastice*. Over time
"stochastic" came to mean only "random" and lost the interesting part: **aiming well at a target you cannot see**.

The purpose of the discipline is not to optimise a metric. It is to say, before anyone acts, what a piece of
quantitative evidence authorises one to conclude, and what it forbids.

[Read the book](00-opening.md)

## 1. The roots

Stochology does not oppose the tradition. It joins traditions that grew up separately and uses the tools of each.

- **The philosophy of induction and demarcation.** Hume showed that no finite series of observations proves a
  general law. Popper answered that science does not prove theories, it tries to refute them. Duhem and Quine
  recalled that a failed prediction refutes the theory together with its auxiliary hypotheses, never the theory
  alone. Lakatos separated the progressive repair, which predicts something new, from the degenerative one, which
  only hides the error. Deborah Mayo demanded the severe test: only a test that could have failed counts.
- **Statistical inference.** From it come strictly proper scoring rules (Brier and log) out of sample, the
  correction for multiple comparisons, extreme value theory for heavy tails, Kelly's bet (1956) for multiplicative
  growth and Knight's distinction (1921) between measurable risk and uncertainty.
- **Stochastic processes.** Markov and Langevin studied how chance accumulates over time. Many decisions are
  sequences, and what matters is the path, not one step.
- **Judgement and decision.** The discipline talks to Herbert Simon's bounded rationality, to the heuristics and
  biases programme of Kahneman and Tversky, and to Gigerenzer's ecological rationality. Many biases in the classical
  catalogue are a forgotten model of chance: the ignored base rate, the filtered sample taken for the population,
  memory attributed to a process without memory.

## 2. The method

### Three layers and the bridge thesis

Every claim is examined in three layers: the **World**, which generates the data and is never seen whole; the
**Model**, the formal and simulable representation; and the **Agent**, who decides, with limited time and attention.
The three form a loop: the World generates data, the Model interprets them, the Agent acts, and the action changes
the World.

The **bridge thesis** says that a fallacy is almost never an arithmetic error but a misalignment between layers. The
prosecutor's fallacy is the classic case: the chance of the evidence given innocence, a property of the Model, is
swapped for the chance of innocence given the evidence, the answer about the World. The thesis says where to look
first, and it is a thesis, not a theorem. Today it stands **unsupported**: the pilot tied at the ceiling (both groups
found every planted defect), and a tie at the ceiling decides nothing. The fair test needs harder cases chosen by
someone outside, human analysts and a blind judge.

### Three kinds of uncertainty, independent of the layer

**Aleatory** uncertainty is born from the draw and does not shrink with more data. **Epistemic** uncertainty is born
from ignorance about the parameter and shrinks when the parameter is identifiable. **Knightian** uncertainty belongs
to a process outside the hypothesis space, and it deceives, because the interval narrows around the wrong answer.

Layer and kind of uncertainty are two independent questions, and every claim answers both. An unlisted mechanism in
the World is Knightian; the decision-maker's doubt about their own calibration is epistemic and lives in the Agent.
What the kind fixes is the treatment: frequency and margins for the aleatory, belief charged by calibration out of
sample for the epistemic, scope and an exit clause for the Knightian. The foundation is neither purely Bayesian nor
purely frequentist: Bayesian belief is charged by frequentist calibration.

### The four prohibitions

1. **No free natural language between agents.** Communication passes through structured cards (YAML) with an
   identifier, layer, kind of uncertainty, definition, simulable artifact, seed, independent unit, null model, scope,
   who pays for the error and an exit clause.
2. **No solitary mean** where the function is nonlinear, the dynamics multiplicative or the tail heavy: report the
   function at the mean, the mean of the function, the median, the quantiles and the cumulative curve.
3. **No confusing learning with performativity.** A small error does not prove that a model is true; run with and
   without the intervention and declare the feedback loop.
4. **No dogmatic extremes and no omitted attempts.** No probability is 0% or 100%, every forecast is scored by a
   strictly proper rule out of sample, and the number of specifications tried is declared (the counter $K$).

### The five invariants

| Invariant | What it requires |
| --- | --- |
| Φ1, simulability | A fixed seed and an executable artifact: whoever claims delivers the code that runs the claim |
| Φ2, independent unit | The margin of error on the true independent unit, never on pseudoreplicates |
| Φ3, severe test | The null model simulated before reading any pattern |
| Φ4, scope and exit | Who pays for the error, and an objective exit clause |
| Φ5, resolution of the question | When the unit is a day or coarser, the aggregate resolution fails |

### Seals, vault and diary

Three mechanisms contain overfitting at scale. **Pre-registration with a hash**: the hypothesis is frozen before
anyone touches the test partition, and every later change is recorded and counts in $K$. The **vault outside
time**: the future partition is opened once, and a correction is worth something only when tested in a new vault.
The **forecast diary** on a hash chain.

The seal has a limit, and the discipline declares it: a hash proves **integrity**, not provenance and not truth. An
invented record, sealed, carries a fingerprint as valid as an honest one; in a rehearsal, seven fabricated records
opened the listening gate. What contains the garden of forking paths is the hash written **before** the data are
seen, together with the counter $K$.

### The panel, the orchestrator and the two systems

Instead of a lone analyst, eight specialists form a safeguard panel: **code, data, lifecycle, inference,
governance, fairness, domain and decision** (A1 to A8). A claim is approved only if every one of them finds its
safeguard declared and anchored on the card; the panel is a checklist, and the attacks that run live in the gates. An orchestrator drives the six-stage cycle: translation and scope, ingestion and grain, simulation and null
model, projection onto the five lenses, panel and gate, decision and closure.

Knowledge lives in a typed hypergraph, and five **lenses** cross every problem as questions: Fluxonomy (is
information conserved?), Chrononomy (does the mean across many coincide with the mean over time of a single one?),
Scalonomy (is the tail so heavy that the mean does not exist?), Morphonomy (does the structure survive when the
environment changes?) and Crisonomy (is the information worth its cost, and are the forecasts calibrated?).
Non-ergodicity is the answer "no" to the question of Chrononomy; in most additive processes the answer is "yes", and
the mean across many is legitimate.

**System 1** does fast triage, with a funnel of filters. **System 2** deliberates: tree search, a belief state,
Metropolis inference and deliberative stress of the panel.

When a claim fails a gate, the verdict is **"unsupported under this test"**, never "null".

### The apparatus

All of this runs in Rust: nine crates, 395 tests, seven gates able to fail a claim and 21 laboratories with a fixed
seed, one per chapter of the book. Every output block quoted in the book is checked against the instrument that
produces it.

## 3. What the real data taught

- **The health post and the bank.** The same mean demand gives 9.3 or 20.7 minutes of wait, because waiting is
  convex in demand. In 2.65 million real calls, the industry's standard schedule fails on 26 of every 100 days, and
  the right margin is computed on the day, not on the call.
- **The leveraged market.** Leverage of 3 times promises 0.20 a year and delivers 0.06: time charges the drag.
- **The Danish fires.** With a heavy tail, a single month decides the fate of the insurer, and in the vault no model
  beat the simple historical frequency. Failing one's own model is a result.
- **The invariant causes.** The gate asked for invariant causes in three areas and passed only in the sixth round,
  with the five failures counted in $K$. With dependent rows corrected, education left and the gate stood on physics,
  pharmacology and genetics. The effect transports better than the list of causes.
- **The live clock.** The 2023 flights and the Rhode Island traffic stops passed in new vaults; the 2020–2021 arrests
  failed. The performative loop declared at 0.6 is four times the measured one, 0.145.

## 4. The open frontiers

The discipline declares what code alone cannot settle. Four fronts remain open, and they are not of the same nature:

- **Expert elicitation (SHELF)** with a full human panel.
- **The voice of those who pay for the error.** In the seven decisions that involve people, no affected group has
  been heard (0 of 7, deadline 2026-12-15). Without listening, the decisions remain sensitivity analyses and do not
  become public policy; once the deadline passes, they are archived as a methodological exercise. It is a governance
  rule, not a theorem.
- **The bridge thesis at scale**, with 12 cases chosen from outside, human analysts and a blind judge (0 of 12,
  window until 2027-01-14). If the window closes without the fair test, the thesis stays permanently unsupported.
- **The bridge to language models.** The local runner exists; what is missing is a frozen file of open weights
  with its hash declared before execution. Saying that the language model plays the role of System 1 would be a
  hypothesis not yet tested.

The first three depend on people outside the programme: consent, listening and blind judgement, which no line of
code replaces. The fourth depends on a file and an operator.

## 5. Where the discipline applies

Wherever a number authorises a decision that someone will pay for:

- **In health**, on the bridge between the trial and the protocol, against base-rate neglect and against the
  correlation that does not transport across environments.
- **In economics and public policy**, against the ensemble mean applied to people who live a multiplicative
  trajectory, and against the queue that collapses because waiting grows on a curve.
- **In engineering**, against trust in thin tails where the tail is heavy.
- **In law and in the governance of algorithms**, against the prosecutor's fallacy, against the metric that becomes a
  target (Goodhart's law), and for declaring where a system can fail and who bears the cost.

In every case, Stochology turns a conjecture about the uncertain into a public commitment: traceable, refutable and
answerable to those who live with the consequences of the decision.
