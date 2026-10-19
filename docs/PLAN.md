# docs/PLAN.md — Master Plan, Technical Architecture, Pedagogy, and Consolidated Content of Stochology (Rust Edition 2024)

This document establishes the exhaustive and expository master plan of **Stochology**, unifying its
epistemological foundations, progressive didactic structure, hybrid cognitive architecture,
cryptographic governance, and modular engineering ecosystem in Rust.

Two layers coexist here, and the distinction is honest:

- **Live engineering layer (Rust, Edition 2024):** the crates in `crates/`, with deterministic tests and
  gates that run in this repository today (Sections 7 and 12).
- **Empirical registry layer (Python):** the fronts `h0-lab0/`, `h0-real/`, `h1/`, `h1-real/`,
  `h2/…h2g/`, `h3/`, `empirical/`, `governance/`, `data/`, `paper/`, and the book
  `book/stochology-textbook.md`. This is the trail of the research concluded on 2026-10-15, described in
  Sections 6 and 11.

---

## 1. Overview and Epistemological Foundations

**Stochology** is formally defined as the science (and the art) of judgment and decision-making under
uncertainty. It weaves together three historical traditions that previously operated in isolation: the
philosophy of induction (Hume, Popper, Lakatos), statistical inference, and the mechanics of stochastic
processes over time.

The central axiom of the discipline posits that **almost every fallacy, analytical illusion, or design
flaw stems not from failures of internal arithmetic, but from misalignments in the bridges between the
three fundamental layers of reality**:

- **World ($\mathcal{L}_1$):** the data-generating physical or social reality, fraught with noise,
  path dependence, and heavy tails.
- **Model ($\mathcal{L}_2$):** the formal, simplified, and simulable representation constructed to
  approximate the world.
- **Agent ($\mathcal{L}_3$):** the decision-maker (person, institution, or algorithm), bounded by time,
  attention, utilities, and performative loops where the action itself undermines the model.

In code, this separation is a type:

```rust
pub enum Camada { Mundo, Modelo, Agente }        // crates/stochology-core/src/layer.rs
pub enum TipoIncerteza { Aleatoria, Epistemica, Knightiana }
```

Mixing layers ceases to be a textual oversight and becomes a compilation error or a rejected card (`Ficha`).

---

## 2. Pedagogical Ramp and Structure of the Book

So that any student (from high school to graduate school) can traverse the learning curve without abrupt
leaps, the work adopts a progressive scaffolding ramp of **3 steps per chapter**:

1. **Phenomenological Intuition and Decision:** natural frequencies, counting trees, and visual charts
   to expose the paradox in natural language.
2. **Theoretical-Formal Treatment:** explicit definitions, identification conditions, and canonical
   theorems (Jensen, Meng, Murphy, Pearl, Kelly, Chouldechova).
3. **Rust Laboratory:** self-contained implementation with a fixed seed, strict typing of invariants,
   and property-based tests.

The complete breakdown into 21 chapters and 5 parts is provided in Section 9.

---

## 3. The Epistemological Machine and the Typed Hypergraph

Knowledge is not maintained in ambiguous prose, but structured as an executable typed hypergraph:

$$\mathcal{G} = (V, E_{\text{tree}}, E_{\text{graph}}, H, \Phi)$$

- **Vertices ($V$):** nodes storing ID, layer, uncertainty type (*aleatory*, *epistemic*, or
  *Knightian*), a one-line formal definition, and a pointer to the simulable artifact.
- **Trees and edges ($E_{\text{tree}}, E_{\text{graph}}$):** deductive prerequisites, mathematical
  bridges, causal edges $\operatorname{do}(x)$, and pathology edges that expose misalignments (flaw of
  averages, inspection paradox, Russell's turkey).
- **Hyperedges / the five lenses ($H$):** cross-cutting tools that inspect the problem from specific
  perspectives:
  1. *Fluxonomy:* information flow and loss across data streams ($I(X;Z) \le I(X;Y)$).
  2. *Chrononomy:* ergodicity, time average versus ensemble average, absorbing states.
  3. *Scalonomy:* heavy tails, Hill exponent, existence of theoretical moments.
  4. *Morphonomy:* structural invariance under regime shifts and interventions (ICP).
  5. *Crisonomy:* expected value of perfect information (EVPI), proper scoring rules, and out-of-time
     calibration.

The validator is executable: `Grafo::validar()` rejects duplicate or incomplete nodes, edges pointing to
missing nodes, deductive cycles in $E_{\text{tree}}$, and hyperedges with fewer than two members;
`Grafo::nos_sem_lente()` enforces that every node is covered by at least one of the five lenses.

---

## 4. Hybrid Cognitive Architecture: System 1 and System 2

The machine's cognition operates in coupled modes under central governance:

- **System 1 Mode ($O(1)$):** fast triage and closed categorical classification of YAML cards via
  deterministic suffix trees, emitting probabilities restricted to the epistemological interval
  $p \in [0.01,\ 0.99]$. The name *LLM-as-Jev* covers two recipes, and their status is not the same: the
  **deterministic Jev** exists in `crates/stochology-core/src/s1.rs` (877 lines, 12 tests, `labs --s1`),
  with the LLM's slot occupied by a silent judge because an unconstrained LLM fails gate G1; the
  frozen open-weights **LLM-as-Jev** remains declared and unimplemented, as item 13 of
  `docs/DECLARED_PROPOSALS.md`, and the verification of the transcript that proposed it is in
  `docs/JEV_INTEGRATION.md`.
- **System 2 Mode (deliberative):** tree search (*LATS* + *Belief-State Engine* over the hypergraph
  $\mathcal{G}$) for deep code synthesis ($F_1$), MCMC inference, and adversarial stress testing. It
  lives in `crates/stochology-s2`: search over the hypergraph, the belief-state engine, Metropolis
  inference, and the panel's deliberative stress testing are up and running with tests and CLI command
  (`labs --s2`); code synthesis ($F_1$) remains declared and unimplemented.
- **Orchestrator ($\mathcal{M}$) and lane agents ($F_1$ to $F_8$):** the orchestrator drives the
  six-stage cycle and submits artifacts to the specialist lanes: $F_1$ (code and simulation),
  $F_2$ (data grain and missingness), $F_3$ (clock, drift, and "I don't know"), $F_4$ (statistics and
  SBC), $F_5$ (privacy and SHA-256 trail), $F_6$ (fairness and who pays for the error), $F_7$ (SHELF
  priors), and $F_8$ (communication and exit clause).

The **safeguard panel** (called the adversarial panel until 2026-10-19) is the orchestrator's checklist: each
lane has an agent $A_i$ that names the attack the artifact should survive and checks that the card declares and
anchors the safeguard against it (Section 8). It runs no attack; the attacks that run live in the gates.

---

## 5. Governance, Invariants, and Deterministic Gates

- **Four structural invariants ($\Phi_1$–$\Phi_4$):** $\Phi_1$ (simulability with a fixed seed),
  $\Phi_2$ (margin computed at the true independent unit), $\Phi_3$ (explicit layers and severe test
  against the null model), and $\Phi_4$ (ecological scope, audit of who pays for the error, and exit
  clause).
- **Type III error prevention:** mandatory field `pergunta_do_decisor` (decision-maker's question),
  validated in code, requiring a trailing question mark and literal match with the package's action.
- **Cryptographic trail:** ledger chained by SHA-256 hashes and a Merkle DAG, out-of-time vault, and
  trial counter $K$.
- **Seven automated gates:** G1 determinism, G2 fixed seed, G3 declared scope, G4 invariants and
  Type III error, G5 trail integrity, G6 safeguard panel $A_1$–$A_8$, G7 exit clause.

---

## 6. Empirical Horizons ($H_0$ to $H_3$) and Background Audit

- **$H_0$ (epistemological seed):** simulated and real pilot (health clinic and call center), testing
  nonlinear queues and the convexity of waiting times.
- **$H_1$ (full cycle):** additive versus multiplicative processes (Dow Jones and volatility drag
  $\mu - \tfrac{1}{2}\sigma^2$), causality (LaLonde), and heavy tails (Danish fire losses).
- **$H_2$ (cross-domain transfer):** invariant causal prediction (ICP) across multiple rounds and
  heterogeneous domains (light tunnel, FDA cardiac trial, yeast genetics).
- **$H_3$ (continuous mesh under a strict clock):** operation under strict time constraints
  ($t_{\text{disp}}$), leakage canaries, Goodhart control, and continuous Platt recalibration.
- **Background audit:** the severe testing of 10 premises of the program itself, including the
  performative loop, $n$-dependent calibration, and real clinical data from PhysioNet.

---

## 7. Technical Workspace in Rust (Edition 2024)

The ecosystem is modularized into dedicated, deterministic crates. **Nine exist and pass all tests today**
(2026-10-17, 373 tests), and no name from the roadmap was left behind: the ten missing crates were born with
their own tests, the last of them is the wave engine (`stochology-wave`, binary `wave`), and System 1 triage
remains as a module inside `core`. The consolidation of 2026-10-17 folded the sixteen crates of the first
pass into nine without dropping a test: `-audit` and `-schema` became modules of `-core`, `-icp`,
`-drift`, and `-fairness` became modules of `-methods`, and `-h0` to `-h3` became modules of
`-horizons` (`docs/declarations/CRATE_CONSOLIDATION.md`).

| Crate | Status | Contents |
|---|---|---|
| `crates/stochology-core` | live | layers, card (`Ficha`), invariants $\Phi_1$–$\Phi_5$, Type III, panel $A_1$–$A_8$, orchestrator $\mathcal{M}$, SplitMix64 RNG, System 1 triage, and filter funnel |
| `crates/stochology-book` | live | chapter-by-chapter book laboratories, with binary `labs --chapter N` and `--all` |
| `crates/stochology-hypergraph` | live | hypergraph $\mathcal{G}$, structural validator, topological sort, and the five lenses |
| `crates/stochology-core/src/audit` | live | zero-dependency SHA-256, hash-chained ledger, and Merkle root (module of `-core`) |
| `crates/stochology-gates` | live | binary `gates`: 7 deterministic governance gates per horizon (G1 to G7) |
| `crates/stochology-core/src/schema.rs` | live | the contract on disk: canonical card text, version 1.2, and the **migration chain in `migracoes.tsv`**, embedded at compile time (module of `-core`) |
| `crates/stochology-s2` | live | deliberative engine (10 modules, 81 unit tests and 7 integration tests): LATS (`lats`), belief state (`crenca`), hypergraph (`hipergrafo`), Metropolis (`inferencia`), panel stress (`estresse`), R-axis (`planejamento`), orchestrator-free FSM bench G8.1–G8.4 (`banca`), Jev-LLM bridge (`ponte`), local runner (`runner`), and Brier calibration (`calibracao`), plus binary `ingest-jev` |
| `crates/stochology-horizons` | live | the four horizons as modules: `h0` (Erlang B and C, minimum staffing, Poisson null, mandatory summary), `h1` (leverage drag, selection bias, Hill tail estimator), `h2` (causal invariance rounds, counter $K$ and Bonferroni, permutation null, fine-grained genetics resolution), and `h3` (strict clock with canary, ECE, Brier skill score, performative loop) |
| `crates/stochology-methods` | live | the three field methods as modules: `icp` (invariant causal prediction: environments, ordinary least squares, Kolmogorov-Smirnov, surviving intersection), `drift` (two-sided CUSUM with measured stationary null and Kullback-Leibler divergence), and `fairness` (demographic parity, equal opportunity, subgroup calibration) |
| `crates/stochology-wasm` | live | WebAssembly playground: minimal C ABI bridge |
| `crates/stochology-wave` | live | wave engine (binary `wave`): local phasor superposition, magnetic Laplacian with Cayley propagator, Kuramoto-Oja plasticity, and spike bus with STDP, with six dedicated gates |

**Disambiguation of gates `G1`–`G7` (`gates`) and `G1`–`G6` (`wave`).** The two binaries use the `G` prefix in
distinct scopes:
- In `gates` (`stochology-gates`), **G1 to G7** enforce card governance per horizon: G1 determinism
  (identical Merkle root across two passes), G2 fixed seed, G3 declared scope, G4 invariants $\Phi_1$–$\Phi_5$
  and Type III error prevention, G5 SHA-256 trail integrity, G6 safeguard panel $A_1$–$A_8$ (every check declared and anchored), and G7
  exit clause.
- In `wave` (`stochology-wave`), **G1 to G6** enforce the physics of the wave engine: G1 Hermiticity of the
  magnetic Laplacian $L^{(q)}$, G2 unitarity of the Cayley propagator, G3 seal chain over the phasor state, G4
  determinism of phase evolution, G5 target resonance gain under Kuramoto-Oja plasticity, and G6 causal arrow
  of the spike bus under STDP.

Verification of the deterministic gates on a clean machine:

```bash
cargo test --workspace
cargo run --release --bin gates -- --horizon all --reproduction
cargo run --release --bin wave -- --check
```

Actual output measured in this repository (12 cards, 4 horizons):

```text
horizonte=todos fichas=12 defeito=nenhum
  lentes: hill_alfa=1.7901 cauda_selvagem=true mediana_log_riqueza=-10.5361 arruinadas=0.892 ...
raiz_merkle=9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81
reproducao: segunda execucao identica = true (raiz 9716e28a87bec9cf)
G1 ok       determinismo da raiz 9716e28a87bec9cf
G2 ok       semente fixa em 12 fichas
G3 ok       escopo declarado: so artefato simulado
G4 ok       Phi1-Phi5 e tipo III (falhas=0)
G5 ok       corrente SHA-256 integra e adulteracao detectada
G6 ok       banca A1-A8 com piso 6 e pior 8
G7 ok       clausula de saida declarada e A8 acima do piso
hash_do_relatorio=08e7c8f0e8e706f9bf95286dd5f6b265c965125e5f496c83b8d9417f53d0e5c2
veredito: 7 gates ok
```

A gate that never fails is not a gate. The defect injector proves that the panel has teeth:

```bash
cargo run --release --bin gates -- --horizon h3 --defect h3.voos23
# G6 reprovado  banca A1-A8 com piso 6 e pior 2
# veredito: 1 gates reprovados  (exit 1)
```

---

## 8. Safeguard Panel: Formalization of $A_1$ to $A_8$

- **The orchestrator ($\mathcal{M}$):** coordinates the hypergraph and manages acceptance gates.
- **The safeguard panel agents ($A_1 \dots A_8$):** each $A_i$ names the stress test of its lane and checks the safeguard against it:
  - $A_1$: property-based tests on code.
  - $A_2$: noise and sentinel value injection into data.
  - $A_3$: live temporal drift simulation.
  - $A_4$: inferential pathology tests.
  - $A_5$: tampering and auditing of the hash trail.
  - $A_6$: subgroup metric inversion (fairness).
  - $A_7$: prior shocks and domain scenarios.
  - $A_8$: crisis simulation violating the exit clause.

The approval condition for artifact $X$ by the ensemble of agents is:

$$\text{Approved}(X) = \bigwedge_{i=1}^{8} C_i(X),\qquad C_i(X) = \text{safeguard}_i(X) \wedge \text{anchor}_i(X)$$

Each $A_i$ reads two bits from the card: the corresponding safeguard (property test, missing-data handling,
declared drift, covered pathologies, intact trail, audited subgroups, elicited prior, exit clause) and the
structural field that anchors it. No seed enters the verdict. Until 2026-10-19 the two bits were printed as a
severity from 0 to 10 drawn by a seed against a floor of 6; the number added nothing to the bits and was removed
(Appendix E of the book). The sketch below is the original design draft, in its original names; the live code is
`crates/stochology-core/src/agents.rs` and `orchestrator.rs`.

```rust
use stochology_core::{AgenteA, Ficha, OrquestradorM, Severidade, SEVERIDADE_MINIMA};

/// Orchestrator M: drives the six-stage cycle and the adversarial panel.
pub struct OrquestradorM {
    pub id_sessao: String,
    pub hash_trilha: String,
}

impl OrquestradorM {
    /// Drives adversarial stress using the specified agent A_i.
    pub fn conduzir_ataque(&self, agente: AgenteA, ficha: &Ficha) -> Result<Severidade, String> {
        if ficha.id.trim().is_empty() {
            return Err("ficha sem identificador".to_string());
        }
        Ok(stochology_core::estressar(
            agente,
            ficha,
            ficha.semente.unwrap_or(2026),
        ))
    }

    /// Runs the entire panel: logical AND of all eight severities against the floor.
    pub fn banca(&self, ficha: &Ficha) -> stochology_core::Aprovacao {
        stochology_core::aprovado(ficha, ficha.semente.unwrap_or(2026))
    }
}
```

The canonical agent enum:

```rust
pub enum AgenteA {
    A1Codigo,
    A2Dados,
    A3CicloVida,
    A4Inferencia,
    A5Governanca,
    A6Justica,
    A7Dominio,
    A8Decisao,
}
```

---

## 9. Pedagogy: The Book Restructured into 21 Chapters

### 9.1 Pedagogical philosophy of the 21-chapter book

In the previous 14-chapter version, dense topics (Bayes, entropy, three types of uncertainty, Jensen's
inequality) were compressed into a single chapter, requiring university-level prerequisites in calculus
and probability. So that **any high school student** can traverse the entire learning curve without
losing a single idea or result from the actual research, the work was unfolded into **21 chapters
grouped into 5 parts**, following the 3-step architecture:

1. **Step 1 — the real world and the paradox (High School level):** every chapter begins with a
   concrete story, counts in natural frequencies ("out of every 100 people..."), and visual mathematics
   (tables, possibility trees, plots of linear and quadratic functions). Differential calculus is
   replaced by geometric intuition (curvature and areas) before introducing the general formula.
2. **Step 2 — the rules of the game (accessible formalization):** formal definitions, theorems, and
   identification conditions, translating every symbol into plain words before the equation.
3. **Step 3 — hands-on (laboratory and exercises):** step-by-step simulations in Rust/Python with a
   fixed seed, where the student watches the theorem unfold by running data, followed by *Concept*,
   *Calculation*, and *Laboratory* exercises.

### 9.2 General map

| Part | Chapters | Pedagogical objective |
|---|---|---|
| **I. The art of aiming in the dark** | 1 to 4 | Separate World, Model, and Agent; master counting trees (Bayes) and distinguish luck, ignorance, and blind spots |
| **II. The traps of the world and of data** | 5 to 8 | Why the average lies (curves and Jensen), how massive samples deceive (Meng), and how to measure good forecasters (Brier) |
| **III. Cause, time, and survival** | 9 to 11 | Climb the ladder of causation (Pearl/ICP), compound interest with random draws (Kelly, non-ergodicity), and Goodhart |
| **IV. Building the auditable machine** | 12 to 15 | Turn science into engineering: hypergraph and 5 lenses, agents $F_1$–$F_8$, panel $A_1$–$A_8$, cards, vault, and hash |
| **V. The machine in the real world** | 16 to 21 | The real tests of the research, from $H_0$ to $H_3$, through the final audit and the student's project |

### 9.3 Part I — The art of aiming in the dark

**Chapter 1. The invisible target: what is Stochology?** From Jacob Bernoulli's *stókhos* (1713) to
deciding before the outcome is known; weaving together the philosophy of doubt (Hume, Popper, Lakatos —
when patching an idea is progress like discovering Neptune, and when it is an excuse like inventing
Vulcan), statistics, and stochastic processes in time. *Laboratory:* fixed-seed random draw simulator
and why two computers must draw the exact same numbers.

**Chapter 2. World, Model, and Agent: where do errors live?** The difference between the territory, the
paper map, and the driver. The epistemic bridge thesis: errors live on the bridges ($L_1 \to L_2$,
$L_2 \to L_3$, $L_3 \to L_1$). Peirce's three modes of inference (deduction, induction, abduction) and
the three meanings of "bias" (in the data, in the formula, in the mind). *Case:* the prosecutor's
fallacy in court and the blind test of the 12 cases. *Laboratory:* `enum Camada` preventing the
programmer from mixing layers.

**Chapter 3. Probability without mystery: bets, trees, and Bayes.** The 5 interpretations of
probability (classical, frequentist, propensity, logical, subjective) linked to each layer; coherence
and the Dutch book; natural frequencies and trees of 1,000 people; the odds form (likelihood ratio and
Turing's *decibans*) and Royall's three questions. *Laboratory:* calculator for trees of 10,000 cases,
two independent tests (10 dB) versus dependent tests (6.2 dB).

**Chapter 4. Luck, ignorance, and the Christmas turkey: the three types of uncertainty.** Russell's
turkey fed for 100 days at 9 a.m., 99% certain on Christmas Eve, which was never on its list.
Aleatory ($L_1$), epistemic ($L_2$), and Knightian ($L_2/L_3$, where more data only inflates false
confidence) uncertainty. Law of total variance, maximum entropy (Jaynes's honest die), and Stirling's
matrix. *Laboratory:* a coin whose bias shifts from 0.2 to 0.6 — the fixed-coin model covers the truth
in 12% of steps (missing 88% of the time), whereas the model with a 0.99 forgetting factor covers
93.98%, with an interval 5.7 times wider. The coverage measured by `cargo run --release --bin labs -- --chapter 4`
is 0.1199 and 0.9398; the 14-chapter text recorded 11% and 99%, and Chapter 4 declares the divergence
along with the parameter grid that explains the effect.

### 9.4 Part II — The traps of the world and of data

**Chapter 5. When the average lies: curves and Jensen.** A 6.2 average with a 6.0 cutoff does not save
the class; the queue that doubles on busy days and never goes negative on empty ones.
$\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$ on parabolas, and the variance toll proportional to curvature.
*Laboratory:* Jensen's gap function on simple curves and on queue waiting times.

**Chapter 6. The invisible filter: why are 2.4 million votes worth 7 people?** *Literary Digest*
1936: 2.4 million responses calling the wrong president while 50 thousand get it right. Meng's identity
$\text{Error} = \text{Quality} \times \text{Quantity} \times \text{Difficulty}$; MCAR, MAR, and MNAR
(Rubin); pseudoreplication and the effective independent sample size $n_{\text{ef}}$ (1,000 students
from the same school, 2 million calls from the same 81 days). *Laboratory:* a massive biased poll erring
as badly as interviewing 7 random people on the street.

**Chapter 7. Hunting ghosts in the noise: severe tests and false discoveries.** 100 people flipping
coins generate a "hot hand" by chance; Miller & Sanjurjo (2018): after heads in 3 flips, the chance is
$5/12$, not one half. Mental shortcuts versus environment (Kahneman and Gigerenzer); the garden of
forking paths, Mayo's severity, Ioannidis's theorem, and the five errors — Type I (false alarm),
Type II (blindness), Type III (right answer to the wrong question), Type S (wrong sign), and Type M
(exaggeration). *Laboratory:* 500 models fit on pure noise and counting the accidental "discoverers".

**Chapter 8. The fortune-teller's thermometer: calibration, resolution, and Brier.** Two
meteorologists: one who says 20% every day and one who warns of the storm. Proper scoring rules (Brier)
where lying about one's own belief always worsens the score; Murphy's decomposition
($\text{REL} - \text{RES} + \text{UNC}$): calibration can be fixed with Platt scaling, lack of
resolution cannot. The sample-size rule ($\text{ECE} \le 0.03$ is too strict in a small test and too
loose across 100 thousand cases). *Laboratory:* 10-bin Brier score evaluator.

### 9.5 Part III — Cause, time, and survival

**Chapter 9. The ladder of cause and effect: graphs, confounders, and ICP.** Do cities with more
hospitals have more deaths? Simpson's paradox in kidney stone surgeries. Pearl's 3 rungs (seeing,
doing, imagining), causal graphs, confounder versus collider (Berkson's bias), the decomposition
$\text{naive difference} = \text{true effect} + \text{selection bias}$, and Peters's invariant causal
prediction. *Laboratory:* true cause and false alarm across 3 environments, with ICP discovering which
is which.

**Chapter 10. Time and ruin: Peters's coin, Kelly, and wild tails.** $+50\%$ versus $-40\%$: $+5\%$
average per round, certain ruin over 100 rounds ($1.5 \times 0.6 = 0.90$). Ensemble average versus time
average (non-ergodicity), volatility drag $\mu - \tfrac{1}{2}\sigma^2$ without advanced calculus, the
Kelly criterion with Laplace smoothing, and the insurance seller who goes 5 years without an accident.
*Laboratory:* 10,000 catastrophe insurers — 35.8% go bankrupt using the simple historical average, 0%
using Laplace + Kelly.

**Chapter 11. The price of error, Goodhart, and who pays the bill.** When to pay for insurance or a
dike; the cost/loss rule ($p > C/L$) and EVPI; van Dantzig's Dutch dikes (Wald, Savage, precaution);
performativity (Goodhart and the Lucas critique); Chouldechova's trilemma and the necessity of declaring
**who pays for the error**; honest communication with natural frequencies, HOPs, and exit clauses.
*Laboratory:* cost/loss calculator on Bernoulli's ship problem and the trilemma in a decision table.

### 9.6 Part IV — Building the auditable machine

**Chapter 15. The map of knowledge: the typed hypergraph and the five lenses.** Long texts hide errors;
the card (node) required to declare identifier, layer, uncertainty type, formula, and executable code;
pathology edges in red; the five lenses (Fluxonomy, Chrononomy, Scalonomy, Morphonomy, Crisonomy).
*Laboratory:* the first mini-hypergraph, with the validator rejecting incomplete cards.

**Chapter 16. The team of specialists ($F_1$ to $F_8$), the safeguard panel, and AI under control.**
How to use AI in science without being fooled: each lane covers a blind spot ($F_1$ simulation, $F_2$
grain and missingness, $F_3$ clock and drift, $F_4$ statistics and SBC, $F_5$ privacy and SHA-256,
$F_6$ fairness, $F_7$ SHELF priors, $F_8$ communication and exit); the six-stage cycle and the panel
$A_1$–$A_8$; System 1 and System 2 (*LLM-as-Jev*, deterministic suffix tree). *Laboratory:* Saerens's
prior-shift corrector ($F_3$) and simulation-based calibration (SBC).

**Chapter 17. The iron contract: cards, prohibitions, invariants, and gates.** The pilot's checklist as
`ficha.yaml`; the 5 header questions and the Type III error lock (`pergunta_do_decisor` ending with "?");
the 4 prohibitions (no free text between agents, no mental math by AI, nobody approves their own work,
0% and 100% forbidden); invariants $\Phi_1$–$\Phi_4$, the 4-filter funnel, and the 5 gates.
*Laboratory:* `ficha.yaml` validator and property-based test attempting to break a function with
thousands of random inputs.

**Chapter 18. The vault, the cryptographic password, and auditing oneself.** Painting the bullseye after
the shot; the three locks (pre-registration with SHA-256 hash, trial counter $K$, out-of-time vault);
the Brier and Kelly logbook and the 21 signed opinions; the spirit of the background audit (10 hard
tests against the machine itself). *Laboratory:* SHA-256 hash chain where altering a single comma in the
past invalidates the report — exactly what `LivroRazao::verificar()` performs.

### 9.7 Part V — The machine in the real world ($H_0$ to $H_3$ and stocktaking)

**Chapter 12. $H_0$: the health clinic and the 2.65 million bank calls.** Clinic with 12 consultations/hour
and average arrival of 8: a constant day yields a 9.3-minute wait; a day alternating between 5 and 11
yields 20.7 minutes and triples the share waiting over half an hour (from 8 to 25 in 100). The split-half
test shows that the independent unit is the **day** ($\pm 0.51$ min), not the patient ($\pm 0.07$ min).
In the real case, 2.65 million calls amount to **81 vault days**: the Erlang C formula promises 82% fast
service, delivers 57%, and fails on 26 out of every 100 days; adding $+7\%$ agents only during peak hours
cuts failures to fewer than 4 in 100 days. *Laboratory:* Lindley recursion with seed `1909`.

**Chapter 13. $H_1$, part 1: the illusion of $3\times$ stock leverage and the job training program.**
Project A — $3\times$ leveraged Dow Jones (1885–2023): real compound growth rises from 5.1%/yr ($1\times$)
to 7.2%/yr ($2\times$) and **drops to 6.0%/yr ($3\times$)**; the 1987 crash ($-22.6\%$) wipes out 68% of
$3\times$ holders, leaving median wealth at $2.1\times$ while the deceptive mean reports $19\times$.
Project B — LaLonde (NSW versus CPS): raw difference of $-\$8,498$ versus the randomized trial's
$+\$1,794$ (selection bias of $-\$10,292$); matching hits the right number ($+9.6$ pp versus $+11.1$ pp)
but fails severity because, on the 1975 placebo, it fabricated a phantom effect of $-12.3$ pp.
*Laboratory:* the 1987 shock on $1\times$, $2\times$, and $3\times$ portfolios and bias decomposition.

**Chapter 14. $H_1$, part 2: the Danish fires and the art of rejecting one's own model.** 36 months
without a mega-fire (50 to 150 million kroner) led the insurer to triple its exposure; the tail is wild
($\hat{\alpha} = 1.81$, near-infinite variance) and 36 clean months carry only $2.2$ bits of evidence
(Laplace: monthly hazard is still $1/38$). In August 1989, a 152-million-krone fire bankrupts the
reckless insurer. The panel tested the agent's own model in the out-of-time vault: it lost to the simple
historical mean (Brier skill score $-0.015$) because fire losses were growing 4.6% per year — **the
insurer's claim collapsed, and the agent's model was rejected and barred from making decisions**.
*Laboratory:* Hill estimator and Laplace's $1/38$ rule.

**Chapter 19. $H_2$: seven rounds to find true causes in science.** The goal is to find
$\hat{S} \neq \emptyset$ in at least 3 domains. The honest story of 5 consecutive failures (using year or
region as an environment invalidates the test; a hidden code bug in round $H_{2b}$; saturation rule in
$H_{2d}$), each recorded with SHA-256 in counter $K$. The three surviving successes after the background
audit: physics (the light tunnel, with the three LEDs `{red, green, blue}` emerging out of 1,024
combinations), pharmacology (FDA/hERG, 16 measurements per volunteer, the 3 true ion channels, and
verapamil predicted in the vault with skill $+0.57$), and yeast genetics (40 genes, $p = 0.001$),
alongside education (STAR) collapsing to $\hat{S} = \emptyset$ once 11,598 students are clustered by
their 80 actual schools.

**Chapter 20. $H_3$: the live clock, the leakage canary, and policing in the vault.** Six streams under a
strict clock ($t_{\text{disp}}$) that forbids peeking a single second into the future; the canary
planted on purpose to prove the alarm rings; Goodhart's Law measured by instrument (4 times smaller than
the initial 0.6 guess); three vaults until calibration succeeded (Toronto burned, Platt scaling, Rhode
Island), reaching $\text{ECE} = 0.0048$ with a modest skill of $+0.019$ due to low resolution, and the
inverted audit of who pays for the error across the three streams.

**Chapter 21. The seven lessons, the ten audit points, and what remains to be done.** The 7 lessons: the
average hides what matters; the independent unit decides the margin; hitting the right number is not
enough if the test could not fail; rejecting one's own model is a scientific victory; prediction travels
better than the list of causes, but only the cause protects against the unseen; calibration can be fixed,
resolution cannot; declaring before running is what separates science from cheating. Taking stock of the
10 points of the background audit. What remains open, honestly stated: (1) in the SHELF protocol
(`empirical/p12-shelf/`), the first expert (E1) elicited five domains with pre-test skill measured
(0.4240 to 0.9730) and reviewed all 21 opinions, but **the other two experts per domain and the external
expert are still missing** (and 5 out of 33 domains flip their decision depending on the prior); (2) fine
resolution in genetics closed in code on 2026-10-16 (`labs --genetics`: 4 of 40 with one line, 1 of 40
with two lines), leaving outside the program only a potential wet-lab validation with a second real
strain; (3) **the voice of those who pay for the error** (`governance/GRUPOS.md`): across the 7 decisions
that affect real people, no member of the affected groups has been heard (deadline 2026-12-15,
`labs --listening`) — signing names the accountable party, listening is something else entirely; (4) **the
bridge thesis at scale** (`labs --bridge`, 0 of 12 cases with an external case selector and judge,
deadline 2027-01-14); and (5) the frozen weights and labeled outcomes for the Jev-LLM bridge
(`labs --jev`). *Final project:* guided walkthrough for the student to choose a school or city problem,
fill out `ficha.yaml`, simulate the world, and defend the conclusion.

### 9.8 Pedagogical appendices

- **Appendix A — visual dictionary of Stochology:** every term (from *abduction* and *Brier* to
  *severity* and *independent unit*) in a single plain sentence, with an everyday example.
- **Appendix B — pocket guide to formulas:** how to read every symbol
  ($\sum, \mathbb{E}, \operatorname{Var}, \mid, \operatorname{do}(x), \ln$) without fear, translated into
  plain language and code.
- **Appendix C — reproduction guide (`stochology-rs` and Python):** exact commands to run the 21
  laboratories and verify the SHA-256 hashes on one's own machine.
- **Appendix D — annotated bibliography for young scientists:** from Jacob Bernoulli (1713) to
  2025/2026 papers.

### 9.9 Names: folder, crates, and binaries

| Before | After |
| --- | --- |
| folder `livro/` | folder `book/` |
| `crates/estocologia-core` | `crates/stochology-core` (lib `stochology_core`) |
| `crates/estocologia-hypergraph` | `crates/stochology-hypergraph` |
| `crates/estocologia-audit` | `crates/stochology-core/src/audit` |
| `crates/estocologia-livro` | `crates/stochology-book` |
| `crates/gates-cli` | `crates/stochology-gates` |
| roadmap crates (`estocologia-schema`, `-s1`, `-s2`, `-h0` … `-h3`, `-icp`, `-drift`, `-fairness`, `-wasm`) | same `stochology-` prefix, and all ten names came to life on 2026-10-16 |

**The binaries did not change names**: `labs` and `gates`. Reason: the book, this plan, and the
`portoes.py` scripts in the Python fronts invoke them by those names, keeping the command typed by the
reader identical regardless of folder language. The assembled book file is now at
`book/stochology-textbook.md`.

### 9.10 Book status and how to build

The book is authored in `book/`, one file per chapter, and assembled into
`book/stochology-textbook.md` once all 21 chapters are ready.

| Part | Chapters | Status |
| --- | --- | --- |
| I. The art of aiming in the dark | 1 to 4 | written, with measured laboratories |
| — | 21 chapters | book assembled in `book/stochology-textbook.md` (6 265 lines, 85 797 words, verified character by character against `book/` and against the declared count by `scripts/book/build.py --check`) |
| II. The traps of the world and of data | 5 to 8 | written, with measured laboratories |
| III. Cause, time, and survival | 9 to 11 | written, with measured laboratories |
| IV. Building the auditable machine | 12 to 15 | written, with measured laboratories |
| V. The machine in the real world | 16 to 21 | written, with measured laboratories (17 and 18 read the datasets from `data/`) |
| Appendices A to D | — | written (A with 73 entries) |

```bash
python3 scripts/book/build.py --check    # requires, per chapter: title, three steps, exercises, and summary
python3 scripts/book/build.py            # assembles the book when all 21 exist
cargo run --release --bin labs -- --all    # runs all laboratories with a fixed seed
```

---

## 9.11 Question Resolution Horizons (new axis)

Second axis of the theory, orthogonal to the empirical one (H0 to H3): **how fine-grained is the
decision-maker's question**, and how much it costs to ask coarsely. Ladder R0 (aggregate) → R1 (day) →
R2 (hour) → R3 (sub-question), with its own dedicated instrument:

```bash
cargo run --release --bin labs -- --resolution
```

Measured with the same total resource budget, paired arms and a staffing that aims at a utilisation of 0.8:
resolution loss is **25.54 min** where demand oscillates between 5 and 11, **2.77 min** where it oscillates
between 7 and 9, and **zero** where it does not oscillate (rectified 2026-10-18; the earlier 46.24 and 5.97 min
were measured at utilisation 1, a critical queue; rectified again 2026-10-19, from 22.99 and 2.50 min, when the day
became a Poisson day of 10 hours with the Lindley recursion in its right order, over 4,000 days instead of 200). The `Resolucao` type and `medir_perda_de_resolucao` live in
`crates/stochology-core/src/resolution.rs`; the full specification, the five falsifiable predictions, and
the status of the rule are in `book/resolution-horizons.md`. The field entered the card
as `Ficha.resolucao` and the rule became $\Phi_5$, enforced by the gates binary; predictions 3 and 5
were measured on 2026-10-16 (item 8 of `docs/DECLARED_PROPOSALS.md` §2): the threshold holds only at
amplitude 2.5 under the calibrated plan, and transfer yielded 0 violations and 3 counterexamples. The
complete inventory of what remains as a declared proposal — theory, method, open research, and limits —
is in `docs/DECLARED_PROPOSALS.md`.

---

## 9.12 Rule for New Files

New working files — intermediate laboratory copies, one-off scripts, drafts — are **deleted after
integration**. Only artifacts with a continuous function (`scripts/book/build.py`,
`scripts/book/check_labs.py`, `scripts/book/check_book.py`, the crates) or permanent records (chapters,
declarations, manifests `history/python-removido.txt` and `history/nao-significante-removido.txt`,
reports) remain in the repository. The latest application of this rule removed `book/labs/` (17
intermediate copies already integrated into `crates/stochology-book/src/capNN.rs`) and the one-off
scripts used for name migration, Python removal, and updating Appendix D.

---

## 9.13 Integrated Documents (architecture, scaling, foundations, and paper)

External project documents were downloaded for reference on 2026-10-16 and **integrated here**, as
mandated by the rule for new files. What each document became:

| Downloaded document | What was integrated | Where |
| --- | --- | --- |
| *Architecture* | data layers (hypergraph, columnar, vector index), seed-isolated runtime, lane agents, card bus | Sections 3, 4, and 7 of this plan; `crates/stochology-hypergraph` and `crates/stochology-core` |
| *Scalable research plan* (5 axes) | maturity axes, funnel with EVSI pruning, horizontal scaling, anti-overfitting governance, and infrastructure | table below |
| *Foundations: three layers and inference* | uncertainty taxonomy and the three layers | Chapters 2 and 4 of the book; Sections 1 and 3 of this plan |
| Paper *Old Frontiers, New Horizons* | was already the canonical text of `paper/README.md` (newer and longer); the downloaded copy was an earlier draft | `paper/` |
| *Integrating Jev Models into Stochology* (Gemini transcript) | verification of every claim against the codebase: what already exists with file and command, the divergences — legacy roadmap numbering ($M_{15}$ and $MB_1$–$MB_8$) versus current nomenclature ($\mathcal{M}$ and $A_1$–$A_8$), four filters versus five, $\Phi_1$–$\Phi_4$ versus $\Phi_1$–$\Phi_5$, *LLM-as-Jev* — and the Jev-LLM bridge as a declared proposal | `docs/JEV_INTEGRATION.md`; item 13 of `docs/DECLARED_PROPOSALS.md` |

| Scaling axis | Requirement | Status in the repository |
| --- | --- | --- |
| 1. Horizon maturity ($H_0 \to H_3$) | exit gates per horizon | fulfilled in the empirical registry; the pipeline became the `gates` binary |
| 2. Funnel with EVSI pruning | do not spend compute without value of information | **implemented in code**: `crates/stochology-core/src/funnel.rs` (878 lines, 17 tests) implements value-of-information pruning and counter $K$, verified by `cargo test --workspace` and `labs --s1`. This row previously read "declared, not implemented", and the **divergence declaration** is recorded here with both old and new states, because the criterion moved from Chapter 11 into code after initial drafting |
| 3. `Plug-and-play` horizontal scaling (F7) | new domain enters via standardized card | partial: the card exists and is validated; the domain engine does not exist |
| 4. Anti-overfitting governance | declare trials and correct for multiplicity | fulfilled in the registry ($K = 38$; Bonferroni applied across the 4 domains of $H_{2g}$) |
| 5. Infrastructure and costs | versioned environment, repeatable execution | fulfilled in Rust: `cargo test --workspace`, fixed seed, and `docs/RUST_RESULTS.md`; the Python portion was retired |

---

## 10. Cards, Prohibitions, and the Iron Contract

- **The 5 header questions:** generating process; nonlinearity; independent unit; noise physics (Itô
  versus Stratonovich); causal ladder.
- **Type III lock:** `pergunta_do_decisor` must end with "?" and literally cite the package's action
  (validated in `crates/stochology-core/src/invariants.rs`).
- **4 prohibitions:** free text between agents; mental math by AI; self-approval; probabilities outside
  $[0.01, 0.99]$.
- **4 invariants and 7 gates:** verified by the `gates` binary, exiting with code 0 or 1.

---

## 11. Bridge to the Empirical Registry (consolidated in Rust)

| Plan front | Where it lives today |
|---|---|
| $H_0$ simulated and real | `docs/EMPIRICAL_REGISTRY.md`; measured by `labs --chapter 15` and `labs --bank` |
| $H_1$ simulated and real | `docs/EMPIRICAL_REGISTRY.md`; measured by `labs --chapter 16` and `labs --chapter 17` over `data/` |
| $H_2$ and re-runs $K=1..3$ | `docs/EMPIRICAL_REGISTRY.md`; the method runs in simulation via `labs --chapter 19` |
| $H_3$ (mesh under strict clock) | `docs/EMPIRICAL_REGISTRY.md`; the mechanism runs in simulation via `labs --chapter 20` |
| Datasets and checksums | `data/` with `SOURCES.md` and `SHA256SUMS` |
| $F_5$ risk opinions | `governance/` |
| Background audit (10 points) | `empirical/p01-tese/`, `p02-independencia/`, `p03-multiplicidade/`, `p05-prioris/`, `p06-laco/`, `p07-icp/`, `p08-calibracao/`, `p12-shelf/`; points 9 and 10 (scaling and data), along with rare-event and clinical extensions, left the working tree on 2026-10-16 because they did not support a central result, and can be recovered from commit `1bfec905` |
| Book (21 chapters in 5 parts, with Appendices A to D) | `book/stochology-textbook.md`, authored in `book/` |
| Paper | `paper/` |

Reproduction commands for the empirical registry (Python 3.13, fixed seed). **The Python simulation code
was retired on 2026-10-16** and lives in commit `1bfec905`: run `git checkout 1bfec905 -- <folder>`
before executing the commands below; see `docs/declarations/REMOVED_PYTHON.md`.

```bash
cd h1 && python3 rodar.py && python3 portoes.py --reproduction && python3 grafo.py
cd h3 && python3 rodar.py && python3 grafo.py && python3 figura.py && python3 portoes.py --reproduction
```

Measurement taken prior to retirement: `cd h1 && python3 -m pytest -q` returned `24 passed in 17.51s`.
The book in `book/stochology-textbook.md` is already the 21-chapter edition: Section 9 of this plan
ceased to be a specification and now describes what is on disk. The 14-chapter text lives in the git
history at commit `1bfec905`.

---

## 12. How to Verify Everything

```bash
# Deterministic gates on a clean machine
cargo test --workspace
cargo run --release --bin gates -- --horizon all --reproduction

# A gate must know how to reject
cargo run --release --bin gates -- --horizon h3 --defect h3.voos23   # exit 1

# Empirical registry (Python), front by front
git checkout 1bfec905 -- h1/          # the Python code lives in git history
cd h1 && python3 -m pytest -q
cd h1 && python3 portoes.py --reproduction
```

State measured in this session (2026-10-17): `cargo test --workspace` passes with **373 tests across the
nine crates**, zero failures; all **twenty-four instruments** of `labs` exit with code 0;
`--horizon all --reproduction` returns "7 gates ok" with an identical second pass and Merkle root
`9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81`; and the negative proof
`--horizon h3 --defect h3.voos23` exits with 1, failing at G6.
