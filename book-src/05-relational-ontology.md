# 5. Relational Ontology and Complex Systems

> **Learning Objectives.** By the end of this chapter, you will be able to contrast substantialist (atomistic) metaphysics with relational and process ontology across six transdisciplinary axes; explain how Buddhist vacuity (*Śūnyatā*), Relational Quantum Mechanics (RQM), and Ontic Structural Realism (OSR) map to mutual information collapse ($I\_{\text{iso}} \to 0$ vs. $I\_{\text{rel}} > 0$); formulate Ilya Prigogine's open-system entropy balance ($dS = d\_i S + d\_e S \le 0$) and Maturana and Varela's autopoietic organizational closure; analyze heteropathic emergence and Thomas Schelling's (1971) segregation phase transition; connect Norbert Elias's *Homo clausus* critique to Moulton-Campbell pseudoreplication ($\Phi\_2$) and Stackelberg performativity; and simulate planetary homeostasis in Andrew Watson and James Lovelock's (1983) *Daisyworld*.

---

## Step 1 — Intuition and the Real-World Paradox

### Peer Gynt's Onion, Cuvier's Whirlpool, and Douglas Adams's Puddle

Why do so many statistical models in economics, sociology, ecology, and psychology break down even when the data are clean and the equations are solved accurately? Because **before any mathematician writes down an equation, they implicitly adopt an ontology—a theory of what kinds of things exist in the World ($L\_1$).**

For three centuries, Western textbook science defaulted to **substantialism and methodological individualism**: the belief that the world is built out of self-contained, isolated bricks (atoms, selfish genes, isolated *Robinson Crusoe* consumers, independent rows in a CSV file) that possess intrinsic properties inside themselves prior to any interaction.

Three famous parables expose the illusion of the isolated brick:

1. **Peer Gynt's Onion (The Search for an Isolated Core):** In Act V of Henrik Ibsen's dramatic poem *Peer Gynt* (1867), the aging protagonist picks up a wild onion and decides to peel away its outer layers—his roles, relationships, habits, and social masks—to find the "true, intrinsic Peer Gynt" hiding at the center. He peels layer after layer until nothing is left in his hand: *"Nature is witty! To the innermost center, it's nothing but layers—smaller and smaller—no kernel at all!"* When you strip away all relations from a node in a coupled network, you do not uncover a pure essence; **the signal vanishes completely.**
2. **Cuvier's Whirlpool and the Knot in a Shoelace (Form That Persists Through Flow):** Look at a standing whirlpool in a mountain stream, or a metabolic cell in your body, or a knot tied in a long cord that you slide from left to right. Every drop of water in the whirlpool and almost every molecule in your body is replaced over time. If you freeze the river by building a solid wall around the whirlpool ("isolating the system"), the whirlpool dies immediately into flat, maximum-entropy water! The whirlpool is not a static lump of matter; it is a **dissipative, self-maintaining pattern of flow** that stays alive *only because* energy and matter pass continuously through its open boundary.
3. **Douglas Adams's Sentient Puddle (The Illusion of Static Adaptation):** Imagine a puddle waking up one morning in a depression in the asphalt and thinking: *"What an interesting world I find myself in—an interesting hole I find myself in—fits me rather neatly, doesn't it? In fact it fits me staggeringly well, must have been made to have me in it!"* And the puddle keeps believing that right up until the sun rises, heats the pavement, and evaporates the puddle. Passive supervised learning ($\theta\_{\mathrm{SL}}$) and myopic retraining ($\theta\_{\mathrm{PS}}$) act exactly like Douglas Adams's puddle: they assume the environment is a fixed mold to which the agent passively conforms, forgetting that in biological evolution (**Richard Lewontin's niche construction**) and institutional economics (**Stackelberg co-evolution, $\theta\_{\mathrm{PO}}$**), organisms and societies actively reshape the very environment that selects them.

In `crates/stochology-lenses/src/relational.rs`, the Stochology Protocol translates these philosophical, physical, biological, and sociological insights into **six executable, quantitatively audited axes** (`seed = 2026`). Let us examine the foundations and mathematics of each axis.

---

## Step 2 — Foundations and Mathematical Formalization

### 5.1 Axis 1 — Relational Vacuity (*Śūnyatā*), Indra's Net, RQM, and OSR

#### Philosophical and Physical Foundations

In classical metaphysics, Aristotle spoke of *substance* (*ousia*) and John Locke spoke of a *bare particular*—a hidden substrate supposed to carry properties all by itself. In classical Indian philosophy, substantialist schools called this hypothetical self-existent essence ***svabhāva*** ("own-being").

In the second century CE, the Buddhist logician and philosopher **Nāgārjuna** (*Mūlamadhyamakakārikā*, Chapter 24, verses 18–19) demonstrated that *svabhāva* is a logical contradiction: if a thing possessed an immutable intrinsic nature independent of causes, conditions, and relations (*pratītyasamutpāda*, dependent co-arising), it could never interact, change, or produce an effect. Therefore, all phenomena are **empty of intrinsic isolated essence (*Śūnyatā*)**—which does not mean they are nothingness, but rather that **to exist is to inter-exist relationally**. Seven centuries later, the Huayan philosopher **Fazang** illustrated this with **Indra's Net**: an infinite cosmic web holding a polished jewel at every vertex, where every jewel exists solely by reflecting all other jewels in the net.

In twentieth- and twenty-first-century physics, the exact same conclusion emerged independently from quantum mechanics and the philosophy of science:
- **Carlo Rovelli's Relational Quantum Mechanics (RQM, 1996):** Quantum states $\psi$ are not absolute properties carried by an isolated particle; a physical variable takes a definite value *only relative to an interaction with another physical system*.
- **James Ladyman and Don Ross's Ontic Structural Realism (OSR, *Every Thing Must Go*, 2007):** At the fundamental level of quantum field theory, there are no self-subsisting "little billiard balls"; **relational structure is all there is.**

#### Information-Theoretic Formalization (`RelationalFigurationAudit`)

How do we test Nāgārjuna's and Rovelli's thesis mathematically inside our Rust apparatus?
Consider two coupled binary subsystems $X\_A, X\_B \sim \mathrm{Bernoulli}(0.5)$ whose **relational interaction**—specifically their phase synchrony $R(X\_A, X\_B) = \mathbb{I}\lbrace X\_A = X\_B \rbrace$—drives the macroscopic observable state $Y$ across a high-fidelity channel ($90\%$ fidelity).
- If an analyst looks at node $X\_A$ in isolation (stripping away its relational edge to $X\_B$ in search of its intrinsic *svabhāva*), the marginal distribution of $X\_A$ is pure $50/50$ coin noise completely independent of $Y$:
  $$I(X\_{\text{isolated}}; Y) = 0 \qquad (\text{empirically } 0.0003\text{ nats under finite sample } n = 2\text{,}000)$$
- When the analyst measures the **relational coupling** $R(X\_A, X\_B)$, mutual information jumps by three orders of magnitude to:
  $$I\bigl(R(X\_A, X\_B);  Y\bigr) = 0.3681\text{ nats} > 0.30\text{ nats}$$

The predictive power lived **$100\%$ in the relation (the edge)** and **$0\%$ in the isolated node**.

---

### 5.2 Axis 2 — Process Ontology, Dissipative Structures, and Autopoiesis

#### Foundations: From Static Equilibrium to Open-System Thermodynamics

Five hundred years before Christ, **Heraclitus of Ephesus** observed that reality is a process of becoming (*"Everything flows; you cannot step twice into the same river"*), and in thirteenth-century Japan **Eihei Dōgen** formulated ***Uji* (Being-Time)**: time is not an empty container through which static objects travel; *being itself is dynamic temporal activity*.

In nineteenth-century physics, the **Second Law of Thermodynamics** stated that in an **isolated system** (closed to both matter and energy exchange with the outside world), entropy $S$ can only increase until all gradients flatten into dead thermal equilibrium:

$$d\_i S \ge 0 \qquad (\text{irreversible internal entropy production})$$

For decades, this created a famous paradox: *If entropy always increases toward disorder, how can living cells, ecosystems, cyclones, and human economies maintain highly ordered structures for decades or millennia?*

Nobel laureate **Ilya Prigogine** resolved the paradox by distinguishing **isolated systems** from **open far-from-equilibrium systems**. In an open system, the total change in entropy $dS$ is the sum of two terms:

$$dS = \underbrace{d\_i S}\_{\text{Internal Production }(\ge 0)} + \underbrace{d\_e S}\_{\text{Boundary Exchange Flux}}$$

While internal metabolism and friction inevitably produce positive entropy ($d\_i S > 0$), an open system imports high-grade free energy and matter from its environment and **exports heat and entropy across its boundary ($d\_e S < 0$)**. Whenever the boundary export exceeds internal production:

$$d\_e S \le -d\_i S < 0 \implies dS = d\_i S + d\_e S \le 0$$

the system sustains a stationary far-from-equilibrium ordered state that Prigogine named a **Dissipative Structure**.

#### Maturana and Varela's Autopoiesis

Chilean biologists **Humberto Maturana and Francisco Varela** (1972) added the topological law that distinguishes a living or institutional system from a mere physical eddy: **Autopoiesis** ("self-production"). An autopoietic network continuously regenerates the very boundary and catalytic processes that produce its components.

In `DissipativeAutopoieticAudit::simulate_vortex` (`crates/stochology-lenses/src/relational.rs`), we simulate an open metabolic vortex where $d\_i S / dt = +0.42$ and boundary export is $d\_e S / dt = -0.45$, yielding net open-system entropy change $dS / dt = -0.03 \le 0$. Over the trajectory:
- **$100.0\%$ of the original constituent matter is replaced** (`material_turnover_fraction > 0.999`), just like Cuvier's whirlpool or the knot sliding along a cord;
- Yet **autopoietic organizational closure** preserves the topological pattern at **$100.0\%$ fidelity** (`autopoietic_pattern_fidelity > 0.999`);
- Conversely, the moment the boundary is sealed ($d\_e S = 0$), internal entropy accumulates unchecked ($\Delta S\_{\text{iso}} > 10.0$) and the structure collapses.

---

### 5.3 Axis 3 — Heteropathic Emergence and Schelling's Segregation Cascade

#### Foundations: *More Is Different* and Nonlinear Composition

In Book III of *A System of Logic* (1843), **John Stuart Mill** distinguished two ways causes combine in nature:
1. **Mechanical (Homopathic) Composition:** If you push an object north with $3\text{ N}$ and east with $4\text{ N}$, the resultant is simply the vector sum ($5\text{ N}$ northeast). The whole is the linear sum of the parts.
2. **Chemical (Heteropathic) Emergence:** Combine sodium (a reactive metal that explodes in water) and chlorine (a lethal greenish-yellow gas), and you obtain **table salt ($\mathrm{NaCl}$)**—a crystalline solid essential for human life whose macro-properties cannot be deduced by summing the isolated properties of sodium and chlorine.

In 1972, Nobel physicist **Philip W. Anderson** universalized this principle in his landmark *Science* essay ***"More Is Different"***: at each new scale of complexity (from quarks to chemistry, from chemistry to cellular biology, from psychology to macro-sociology), **broken symmetries generate genuinely new emergent laws**.

#### Thomas Schelling's (1971) Dynamic Segregation Phase Transition

No model illustrates emergent macro-pathology from benign micro-intentions more starkly than **Thomas Schelling's (1971)** spatial neighborhood model (`SchellingSegregationModel`).

Imagine a city grid populated by two groups of residents ($A$ and $B$, plus empty houses). Every resident looks at their immediate $3 \times 3$ **Moore neighborhood** (up to $8$ adjacent neighbors) and asks a single question: *"Is the fraction of my neighbors who belong to my own group at least $\tau$?"*
- If yes, the resident is content and stays put.
- If no, the resident moves to the nearest vacant cell where that threshold $\tau$ is met.

Now look at the paradox of **non-hostile tolerance**:
- Suppose every single citizen in the city is happy to be in the minority, requiring only **$\tau = 0.375$ ($3/8$, i.e., gladly accepting $62.5\%$ out-group neighbors!)** and having zero desire for a segregated city.
- A linear, reductionist analyst who surveys individual attitudes ($L\_3$) concludes: *"Nobody in this city wants segregation; therefore, any observed segregation must be small or intentional."*

When we run `SchellingPhaseTransitionAudit::evaluate(2026)` on the 2D grid, a **nonlinear tipping cascade** unfolds:
- At subcritical tolerance **$\tau = 0.125$**, the city remains stably integrated (**$50.2\%$** same-group similarity, right at the random baseline).
- At mild, non-hostile tolerance **$\tau = 0.375$** (where everyone accepts $62.5\%$ out-group neighbors), a single local move tips a neighbor below $3/8$, who moves and tips two more neighbors, triggering a self-reinforcing avalanche that locks the city into **$82.4\%$ macro-segregation** (an emergent gap of $\Delta\_{\text{emerg}} = 0.449 > 0.38$ above individual intention)!

This mathematical theorem grounds modern structural sociology (**Eduardo Bonilla-Silva's** *Racism without Racists* and **Silvio Almeida's** *Structural Racism*): **macroscopic structural inequality and segregation are emergent topological properties of coupled networks and institutional rules ($L\_1$), not simple averages of individual hostility.**

---

### 5.4 Axis 4 — *Homo clausus* vs. *Homines aperti* and Figurational Coupling

In *The Civilizing Process* (1939) and *What Is Sociology?* (1970), sociologist **Norbert Elias** diagnosed the central blind spot of modern social and behavioral science as the myth of ***Homo clausus*** ("the closed human")—the image of the individual as a little sealed box standing "inside" while "society" stands outside like an external wall. Against *Homo clausus*, Elias proposed ***Homines aperti*** ("open, interdependent pluralities of humans") bound together in dynamic **figurations** (like dancers in a mobile court dance: you cannot understand the steps of one dancer by filming them in an empty room).

In the Stochology Protocol, the *Homo clausus* fallacy manifests in two exact mathematical errors that our invariants block:
1. **On Bridge $L\_1 \to L\_2$ (Pseudoreplication, Invariant $\Phi\_2$):** Treating $n = 500$ observations drawn from coupled social clusters as $500$ independent atoms ignores the Moulton-Campbell intraclass correlation. In `RelationalFigurationAudit`, a tail-exceedance design effect of **$\mathrm{DEFF} = 5.81\times$** shrinks $n = 500$ apparent individuals down to **$n\_{\mathrm{eff}}(0.90) = 86.0$** true independent degrees of freedom.
2. **On Bridge $L\_3 \to L\_1$ (Puddle Stability vs. Co-Creation, Prohibition 3):** Treating the agent as passively adapting to a fixed social hole yields Performative Stability $\theta\_{\mathrm{PS}} = 2.000$, whereas recognizing the reciprocal co-creation of agent and figuration yields the Stackelberg optimum $\theta\_{\mathrm{PO}} = 2.444 > \theta\_{\mathrm{PS}}$.

---

### 5.5 Axis 5 — Cybernetic Co-Evolution (*Triple Helix*) and *Daisyworld* Homeostasis

#### Foundations: Lewontin's Triple Helix, Giddens, and Lovelock's *Daisyworld*

In evolutionary biology, **Richard Lewontin** (*The Triple Helix: Gene, Organism, and Environment*, 2000) replaced the one-way Neo-Darwinian differential equation $\frac{dO}{dt} = f(O, E)$ (where a static environment $E$ filters passive organisms $O$) with a **coupled co-evolutionary system**:

$$\frac{dO}{dt} = f(O, E), \qquad \frac{dE}{dt} = g(O, E)$$

Beavers build dams that transform dry valleys into wetlands, altering the selection pressures on their own descendants; earthworms chemically rebuild terrestrial soil; in sociology, **Anthony Giddens's Duality of Structure** and **Roy Bhaskar's Transformational Model of Social Activity (TMSA)** show that social structure is both the medium and the continually reproduced outcome of human agency.

In 1983, **Andrew Watson and James Lovelock** created **Daisyworld** (`DaisyworldSystem` in `crates/stochology-lenses/src/relational.rs`) to prove mathematically that coupled organism-environment feedback ($f$ and $g$ together) automatically generates **planetary cybernetic homeostasis without any foresight, teleology, or central planner**.

#### Mathematical Equations of *Daisyworld*

Consider a planet seeded with two species of daisies covering fractions $\alpha\_b$ (**black daisies**, low albedo $A\_b = 0.25$ that absorb heat) and $\alpha\_w$ (**white daisies**, high albedo $A\_w = 0.75$ that reflect heat), with bare ground $\alpha\_g = 1 - \alpha\_b - \alpha\_w$ at intermediate albedo $A\_g = 0.50$:

1. **Planetary Albedo and Stefan-Boltzmann Radiative Balance:**
   The planet's effective albedo is the weighted sum $A\_p = \alpha\_g A\_g + \alpha\_b A\_b + \alpha\_w A\_w$. Under solar luminosity $L$ (which warms steadily over geological time from $L = 0.65$ to $L = 1.60$) and solar flux constant $S\_0$, the effective planetary temperature $T\_e$ satisfies the **Stefan-Boltzmann Law**:
   $$\sigma\_{\mathrm{SB}} T\_e^4 = S\_0 L (1 - A\_p)$$

2. **Local Microclimate Heating:**
   Because dark patches absorb more sunlight than average and bright patches reflect more, local temperatures $T\_b$ and $T\_w$ over black and white daisy beds satisfy:
   $$T\_i^4 = q(A\_p - A\_i) + T\_e^4 \qquad (i \in \lbrace b, w \rbrace)$$
   so black daisies warm their local microclimate ($T\_b > T\_e$) while white daisies cool theirs ($T\_w < T\_e$).

3. **Parabolic Growth Rate and Replicator ODEs:**
   Both species grow best at an optimal biological temperature $T\_{\text{opt}} = 22.5 ^\circ\mathrm{C}$ (vanishing below $5 ^\circ\mathrm{C}$ and above $40 ^\circ\mathrm{C}$):
   $$\beta(T\_i) = \max\bigl(0,  1 - 0.003265 (22.5 - T\_i)^2\bigr)$$
   and compete for bare fertile ground $\alpha\_g$ against mortality rate $\gamma = 0.30$:
   $$\frac{d\alpha\_b}{dt} = \alpha\_b(\alpha\_g \beta(T\_b) - \gamma), \qquad \frac{d\alpha\_w}{dt} = \alpha\_w(\alpha\_g \beta(T\_w) - \gamma)$$

Watch the cybernetic loop work across solar luminosity $L$:
- When the young sun is dim ($L \approx 0.70$), the planet is cold; black daisies thrive because they warm themselves ($T\_b > T\_e$), spreading across the globe, lowering planetary albedo $A\_p$, and **warming the entire planet up to $\approx 22.5 ^\circ\mathrm{C}$**!
- As the sun brightens ($L$ climbs from $0.80$ to $1.35$), white daisies gain the advantage because they cool themselves ($T\_w < T\_e$), gradually replacing black daisies, raising planetary albedo $A\_p$, and **cooling the planet against the brightening sun**!
- In the coexistence regime, the planetary derivative is actually negative (**$dT\_e / dL = -11.93 < 0$**), maintaining a mean absolute thermal deviation from $22.5 ^\circ\mathrm{C}$ of only **$\mathrm{MAE} = 1.52 ^\circ\mathrm{C}$** (compared to **$9.57 ^\circ\mathrm{C}$** on a dead lifeless planet)!
- Finally, at extreme solar forcing **$L = 1.60$**, solar radiation overwhelms even a $100\%$ white-daisy canopy, crossing an **absorbing catastrophe boundary** where the biosphere goes extinct and temperature jumps vertically to the dead-planet line.

---

### 5.6 Axis 6 — The Six-Layer Ontological Stack and Complexity Economics

Historian of economics **Philip Mirowski** (*More Heat than Light*, 1989) documented how late-nineteenth-century neoclassical economists (Walras, Jevons, Edgeworth) copied the equations of 1860s closed-system static potential energy physics line by line—substituting "utility" for potential energy and assuming a frictionless, history-free, atomistic equilibrium. **Edgar Morin**, **W. Brian Arthur**, and **Eric Beinhocker** contrasted this reductionist paradigm against the **Six-Layer Relational Ontological Stack** audited in Axis 6:

| Stack Layer | Substantialist / Reductionist Paradigm | Relational / Complex Systems Paradigm (Stochology) | Audited Protocol Mechanism |
| --- | --- | --- | --- |
| **1. Physics & Metaphysics** | Atomism, *bare particulars*, intrinsic *svabhāva*. | *Śūnyatā* (Nāgārjuna), Indra's Net, RQM (Rovelli), OSR (Ladyman). | $I(X\_{\text{rel}}; Y) = 0.3681\text{ nats}$ vs. $I(X\_{\text{iso}}; Y) = 0.0003\text{ nats}$. |
| **2. Thermodynamics & Time** | Closed static equilibrium ($dS = 0$), reversible clock time. | Dōgen's *Uji*, Prigogine dissipative open systems ($dS = d\_i S + d\_e S \le 0$), Maturana-Varela autopoiesis. | $d\_i S = +0.42, d\_e S = -0.45 \implies dS = -0.03 \le 0$; $100\%$ turnover & closure. |
| **3. Biology & Ecology** | Passive adaptation to a fixed niche (Douglas Adams's puddle). | Lewontin's *Triple Helix* niche construction & Watson-Lovelock *Daisyworld*. | Thermal homeostasis $\mathrm{MAE} = 1.52 ^\circ\mathrm{C}$ vs. $9.57 ^\circ\mathrm{C}$ dead planet; extinction at $L=1.60$. |
| **4. Psychology & Mind** | *Homo clausus* (Ibsen's onion core, isolated rational ego). | Hume's Bundle Theory, *Anattā*, Elias's *Homines aperti* figurations. | Moulton-Campbell $\mathrm{DEFF} = 5.81\times$ ($n=500 \to n\_{\mathrm{eff}}=86.0$). |
| **5. Sociology & Emergence** | Methodological individualism (macro $=$ sum of micro intentions). | Mill's heteropathic laws, Anderson's *More Is Different*, Schelling tipping cascades, Giddens's Duality. | Schelling $\tau = 0.375 \implies 82.4\%$ segregation ($\Delta\_{\text{emerg}} = 0.449 > 0.38$). |
| **6. Political Economy & Epistemology** | Neoclassical static equilibrium (Mirowski's copied energetics). | Complexity Economics (Arthur, Beinhocker), Morin's complex thought, Jensen's Gap & Stackelberg performativity. | $\Delta\_J > 0$ under endogenous volatility & Stackelberg $\theta\_{\mathrm{PO}} = 2.444 > \theta\_{\mathrm{PS}} = 2.000$. |

---

## Step 3 — Auditable Laboratory in Rust and Exercises

In `crates/stochology-lenses/src/relational.rs`, `RelationalComplexSystemsSynthesis::evaluate(2026)` executes all six axes and is verified in `labs --lenses` and Gate `G8`:

~~~bash
cargo run --release --bin labs -- --lenses
~~~

~~~text
  [Relational Ontology & Complex Systems] MI_rel=0.3681 vs MI_iso=0.0003 nats | Prigogine dS=-0.03 (turnover=100.0%, closure=100.0%) | Schelling(tau=0.125->50.2% vs tau=0.375->82.4%) | Daisyworld(MAE=1.52C vs dead=9.57C, dT/dL=-11.93) | verified=true
~~~

### Exercises

1. **(Concept — Relational Vacuity vs. Substantialism)** In `RelationalFigurationAudit`, why does each individual node $X\_A \sim \mathrm{Bernoulli}(0.5)$ have zero mutual information with the target $Y$ when inspected alone, yet high mutual information when paired with $X\_B$? Give a real-world example from neuroscience, finance, or ecology where measuring a single variable in isolation misses an interaction completely.
2. **(Calculation — Prigogine Entropy Balance)** An open industrial or biological system generates internal irreversible entropy at rate $d\_i S / dt = +0.65\text{ J/(K}\cdot\text{s)}$. What is the minimum magnitude of negative entropy flux $|d\_e S / dt|$ that the system must export across its boundary to avoid thermal degradation? What happens if its boundary is sealed ($d\_e S / dt = 0$) for $t = 20$ seconds?
3. **(Concept — Schelling Phase Transition)** In Schelling's segregation model with $\tau = 0.375$, every resident is happy living in a neighborhood where $62.5\%$ of their neighbors belong to the other group. Explain step by step why the city nevertheless tips into $82.4\%$ segregation, and why surveying individual attitudes fails to diagnose the macro-outcome.
4. **(Concept — Daisyworld Cybernetics)** In *Daisyworld*, neither black nor white daisies have brains, committees, or awareness of planetary temperature—each daisy simply grows faster when its local microclimate is near $22.5 ^\circ\mathrm{C}$. Why does $dT\_e / dL$ become negative in the two-species coexistence zone, and what lesson does the collapse at $L = 1.60$ teach about absorbing boundaries ($\Phi\_3 / \text{Lens 2}$)?

### Chapter Summary

- **Axis 1 (*Śūnyatā* / RQM / OSR):** Predictive and physical properties reside in relational couplings ($I\_{\text{rel}} = 0.3681\text{ nats}$), vanishing when nodes are stripped of their relations ($I\_{\text{iso}} = 0.0003\text{ nats}$).
- **Axis 2 (Dissipative Structures & Autopoiesis):** Open systems maintain far-from-equilibrium order via boundary entropy export ($dS = d\_i S + d\_e S \le 0$), surviving $100\%$ material turnover while preserving topological closure.
- **Axis 3 (Heteropathic Emergence & Schelling):** Non-hostile micro-preferences ($\tau = 0.375$) trigger nonlinear tipping cascades into macro-segregation ($82.4\%$), proving that *More Is Different*.
- **Axis 4 (*Homo clausus* vs. *Homines aperti*):** Treating interdependent humans as isolated atoms commits Moulton-Campbell pseudoreplication ($\mathrm{DEFF} = 5.81\times$) and ignores Stackelberg performativity ($\theta\_{\mathrm{PO}} > \theta\_{\mathrm{PS}}$).
- **Axis 5 (*Daisyworld* Co-Evolution):** Coupled replicator-radiative feedback stabilizes planetary temperature near $22.5 ^\circ\mathrm{C}$ ($\mathrm{MAE} = 1.52 ^\circ\mathrm{C}$) without teleology until the absorbing catastrophe boundary ($L = 1.60$).
- **Axis 6 (The 6-Layer Ontological Stack):** Aligns physical, thermodynamic, biological, psychological, sociological, and economic layers under a single auditable synthesis (`verified = true`).
