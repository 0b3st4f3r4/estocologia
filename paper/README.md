# "Stochology": Old Frontiers, New Horizons

Silvano Neto · October 14, 2026, updated on October 15, October 18 and October 19, 2026

*Text and figures under CC BY 4.0; the code that produces the numbers is under MIT (see the repository README).*

## Abstract

Stochology is the science (or art) that studies judgment and decision under uncertainty. This paper argues that it is not a new branch of statistics, but a seam: it joins three traditions that grew apart (the philosophy of induction, statistical inference, and the mechanics of stochastic processes) and organizes them into three layers, World, Model, and Agent. The central thesis is that most of the errors of reasoning under uncertainty are not errors of arithmetic, but misalignment between those layers.

That sentence carries three claims of different nature, and the paper keeps them apart: (a) that errors live in the seams between layers, which is a claim about where errors are; (b) that each classical bias is an ignored model of chance, which is a claim about the catalogue; and (c) that the architecture built to operate the discipline catches errors that competent ordinary practice lets through, which is a claim about an instrument. Only (c) has been tested, once, and it was not sustained. (a) and (b) give the program its name without yet being at risk, and section 9 says what would put them there.

The paper goes through the old frontiers that the discipline inherits, presents the architecture with which the project intends to operate it (a typed hypergraph, audited by specialized agents and by five invariants), and reports the first results. In a simulated health clinic, the same average demand produces an average wait of 9.1 or 19.3 minutes, depending on whether the world varies or not. In real data, the same logic shows that the standard staffing of a bank call center, which promises to meet the target every day, fails on about 31 out of every 100. The program passed the four horizons of its plan, but not on the first try: the third, the search for causal structures invariant across areas, required seven rounds, and the fourth, continuous calibration out of time, three attempts in the hardest flow. The paper treats the failures of the path as a result and ends with the horizons that open up and the objections that the discipline must face.

This version adds the background audit of the program, in section 8: ten points that the text assumed and that were tested against the data themselves, among them the blind pilot of the central thesis, which tied at the ceiling, and the performative loop, which measurement at a real call center puts around 0.15, and not at the declared 0.6. It also gives the exact form of the computations that before were only named (Bayes in odds, Meng's identity, the back door, Kelly's criterion, Murphy's decomposition) and the cases that make them concrete, from mammography to Bernoulli's ship. Three new objections also come in, among them the one that the program's auditor is an interested party.

The update of October 19 aligns the paper with the book as it stands after its critical reading: every number here that an instrument of the repository produces is the instrument's number, and the values the book retired (Appendix E of the book keeps each of them beside its replacement) were retired here too. Two results are new. The six real streams of the live clock were rebuilt in Rust on public data and the record and the rebuild diverge in two places, which the book keeps as a result (section 6). And the stopping rule of the transfer gate, which section 9 left open on October 18, was closed: the 38 recorded tests were re-read under an e-process declared before the computation, and three areas cross the threshold at any stopping time (section 8). Section 9 also gained the neighbours of the discipline, named one by one, with what is borrowed from each.

## 1. Introduction: deciding without knowing

Every decision that matters is made before we know what is going to happen. The physician prescribes without knowing that patient's answer, the judge convicts without having seen the crime, the manager hires staff for a demand that has not arrived yet. In all these cases, someone must turn incomplete information into a concrete action, and answer for the consequences.

There have been tools for this for centuries: probability, statistics, decision theory, the psychology of judgment. The problem is that they rarely talk to each other. The statistician computes intervals without asking where the data came from; the philosopher discusses induction without running a simulation; the psychologist catalogs biases without saying in which environment the shortcut would be optimal. Each one sees a piece of the problem.

**Stochology is the science (or art) that studies judgment and decision under uncertainty.** The name comes from the Greek *stókhos*, the target one aims at, the well-oriented conjecture. It is not an arbitrary choice: in 1713, Jacob Bernoulli called his work *Ars Conjectandi*, the art of conjecturing, and defined that art as *stochastice*. Three centuries later, the word "stochastic" became a synonym of "random" and lost the original sense of aiming well when one does not see the target. Stochology wants to recover that sense.

The "or art" of the definition is deliberate. Judging under uncertainty has a part that is formalized (the coherence of beliefs, updating by evidence, the margin of error) and a part that is not fully formalized (deciding which model is worth taking seriously, when to stop collecting data, who pays if the error happens). An honest discipline about uncertainty must also be honest about the limits of its own formalism.

The discipline has a teaching body, the book *Stochology* in the same repository, and this paper leans on it. The book is an arc, not a catalogue: twenty-one chapters in five parts. Part I gives the foundation (the invisible target; World, Model and Agent; probability without mystery; the three kinds of uncertainty). Part II gives the pathologies of the mean, of the sample filter, of the pattern read into noise, of the uncalibrated forecast, of the correlation taken for a cause, of the trajectory confused with the ensemble, and of the error whose price nobody declared (Chapters 5 to 11). Part III takes three real cases through the whole method, the health post and the bank, the market and the employment programme, and the Danish fires (Chapters 12 to 14). Part IV builds the machine that audits the method (Chapters 15 to 18), and Part V turns the machine on itself, with the seven rounds of the transfer gate, the live clock and the seven lessons (Chapters 19 to 21). Every chapter carries a laboratory with a fixed seed whose output is quoted in the text and checked against the instrument that produces it, and 126 exercises with a published answer key. One reflective question closes the journey: when is a model good enough? The course *Critical Thinking and Stochastic Models*, from which the book grew, remains the source of the simulations that this paper attributes to "the course"; those numbers come from its scripts and were not re-measured by the book's instruments.

## 2. Old frontiers

Stochology does not invent its problems; it inherits them. Three old frontiers organize that inheritance, and each of them separated people who should be talking.

**The frontier between disciplines.** Philosophy asked whether induction is justified. David Hume showed that any defence of it presupposes that the future will resemble the past, which is circular. Karl Popper answered by exchanging confirmation for refutation; Duhem, Quine, and Lakatos recalled that no hypothesis is tested alone. In parallel, mathematics built probability (Pascal and Fermat, Bernoulli, Bayes, Laplace, Kolmogorov) and statistics built inference (Fisher, Neyman, the Bayesian school). A third lineage, that of stochastic processes (Markov, Langevin), studied how chance accumulates in time. The three traditions rarely cite each other.

Seen up close, that frontier has texture. Peirce obtained the three forms of inference by permuting the same three propositions, rule, case, and result. Only deduction preserves truth; induction and abduction enlarge knowledge and for that reason can fail. Abduction, inference to the best explanation, chooses the hypothesis that makes the data least surprising, but only among the hypotheses considered. It is the precursor of maximum likelihood and inherits the same limit.

Popper faced a problem that the fame of falsificationism hides: "this coin is honest" is compatible with any finite sequence of tosses. To test it, he proposed a convention that treats sufficiently improbable observations as refuting, and every convention of that kind errs at a rate fixed in advance, which does not fall with more data. Duhem and Quine complete the picture: a frustrated prediction refutes the theory together with its auxiliary hypotheses, never alone. In 1846, the anomaly in the orbit of Uranus did not bring Newton down, because Le Verrier postulated a hidden planet and Neptune was found. The same manoeuvre with the planet Vulcan, to explain Mercury, failed. Lakatos called the first adjustment progressive and the second degenerative.

The Langevin lineage also has a lesson of its own. In 1827, Robert Brown saw grains of pollen stirring in water and thought of an activity of living matter; Einstein (1905) explained the motion by the collisions of molecules, and Langevin (1908) wrote the equation. In a system with two stable states, the average time until the jump from one to the other follows Kramers's formula and grows exponentially with the height of the barrier:

$$
\tau \approx \frac{2\pi}{\sqrt{U''(x_{\min})\,\lvert U''(x_{\max})\rvert}}\; e^{\Delta U/T}
$$

That time has no memory: whoever has waited long is no closer to the jump. Looked at only in the jumps, however, the noise looks like a push: conditioned on a jump having happened, the noise just before it almost always pointed in its direction, with no external force. Looking for the cause of a jump is a selection error; looking for the cause of a change in the frequency of jumps, not.

**The frontier between risk and uncertainty.** In 1921, Frank Knight separated risk, in which the probabilities are known, from uncertainty properly speaking, in which not even the list of possibilities is available. A good part of twentieth-century decision theory (von Neumann and Morgenstern, Savage) worked on the comfortable side of that line. Russell's turkey, fed punctually for a thousand days, shows the uncomfortable side: each day of data makes its model more confident and, at the same time, more wrong, because the slaughter was never in its hypothesis space.

The turkey can be simulated, and the simulation shows that the problem is not a lack of data. Bertrand Russell told the story with a chicken; Taleb retold it with a turkey. An animal that reasons by Laplace's rule of succession estimates, after s successes in n days,

$$
P(\text{success tomorrow}\mid s,n)=\frac{s+1}{n+2}
$$

and, on the eve of the hundred and first day, is 99% confident. The rule is not neutral, because it derives from a uniform prior over the rate: with seven successes in ten, the uniform prior gives 0.667, a uniform over θ² gives 0.692, and a uniform over √θ gives 0.652. Even ignorance has a shape. And when the world changes slowly the fixed model fails in silence. If the bias of a coin rises from 0.2 to 0.6 and the model supposes a fixed coin, the 95% interval shrinks from 0.140 to 0.035 and covers the truth only 11% of the time. Forgetting the past, with a discount of 0.99 per step, restores coverage to 99%, at the price of intervals three times wider.

**The frontier between norm and behaviour.** Kahneman and Tversky catalogued how people move away from the rules of probability: they ignore the base rate, they judge the conjunction more probable than its parts. Herbert Simon and, later, Gerd Gigerenzer answered that a shortcut is only an error relative to an environment; when the same information is given in natural frequencies ("8 in every 10") instead of conditional percentages, many of the "biases" disappear. The debate stayed stuck between two camps, each right about half of the problem.

The course proposes a way out of that impasse: a criterion instead of a side. A deviation is an error when the right information was available and the combination rule was violated; it is a candidate adaptation when the norm supposes an information that the environment does not supply reliably. In a screening with a prevalence of 1%, whoever ignores the base rate has a Brier score of 0.083, against 0.009 for whoever uses it. In a referral clinic, where the real prevalence is 33%, whoever insists on 1% has 0.28, against 0.085 for whoever ignores it. The same heuristic is an error in one environment and a hit in the other.

The verdict of "bias" also depends on the model of whoever judges. Lichtenstein and colleagues (1978) saw that people overestimate the causes of rare deaths and underestimate the common ones; Erev, Wallsten, and Budescu (1994) showed that noisy judgments look underconfident or overconfident depending on the axis on which one conditions. In a course simulation, judges with no bias at all, each with a noisy signal, reproduce Lichtenstein's slope of 0.72 on one axis and are perfectly calibrated on the other. The most celebrated case is that of the hot hand. For three decades, the study by Gilovich, Vallone, and Tversky (1985) was the canonical example of pattern illusion, until Miller and Sanjurjo (2018) showed that the test was rigged against the hot hand: in three tosses of an honest coin, the average proportion of heads after heads is 5/12, not 1/2. Corrected, the difference in the Cornell data goes from +3 to +13 points. Accusing an illusion requires a correct null model.

| Year | Milestone | Frontier it left |
| --- | --- | --- |
| 2000 | Judea Pearl, *Causality* | Formal causality, but far from ordinary statistical practice |
| 1996 | Deborah Mayo, severe test | Philosophy of error, little used by those who run the tests |
| 1974 | Kahneman and Tversky, heuristics and biases | Norm versus behaviour |
| 1955 | Herbert Simon, bounded rationality | The real agent, without the mathematics of inference |
| 1954 | Leonard Savage, subjective probability | Decision under risk, little under radical uncertainty |
| 1933 | Kolmogorov, axioms of probability | Mathematical rigour, silence about interpretation |
| 1921 | Frank Knight, risk and uncertainty | Measurable risk versus unmeasurable uncertainty |
| 1908 | Langevin, stochastic dynamics | Chance in time, separated from inference |
| 1763 | Bayes's essay (published by Price) | Belief as probability, a long dispute with the frequentists |
| 1748 | Hume, problem of induction | Empirical science without logical justification |
| 1713 | Bernoulli, *Ars Conjectandi* | The art of conjecturing is born together with the law of large numbers |

What these frontiers have in common is that each side is right about its piece and wrong when generalizing. Stochology starts from the bet that the errors live precisely in the seams.

## 3. Foundation: World, Model, and Agent

The first gesture of Stochology is to separate three things that common sense mixes. The **World** is the reality that generates the data and that we never see whole. The **Model** is the formal, simplified, simulable representation that we build to approximate it. The **Agent** is whoever decides: a person, an institution, or an algorithm, always with limited time, attention, and computation.

![The three layers of Stochology and the loop of action](figuras/camadas.png)

*The three layers of Stochology and the loop of action.*

The separation has a strong consequence: **a fallacy is not an error of arithmetic, it is a misalignment between layers.** The prosecutor's fallacy happens when the Agent reads the likelihood of the Model, P(evidence | innocent), as if it were the answer about the World, P(innocent | evidence). The inspection paradox happens when the Agent imposes a stable Model on a World that oscillates. In both cases the arithmetic can be right; what is wrong is the bridge.

This is claim (a), and it has a weakness that the paper must name before anyone else does. World, Model, and Agent cover by construction the reality, the representation, and whoever decides, so any error can be redescribed as a wrong bridge. A classification that absorbs every case is a vocabulary, not a thesis. What would make (a) a thesis is that independent coders, with the same codebook, put the same documented error in the same place, and that they can and sometimes do code an error as pure arithmetic with the bridges right. That test is declared, with its decision rule, and not yet run (section 9).

Claim (b) is the operational version: each bias of the classical catalogue corresponds to an ignored model of chance. Base-rate neglect is the likelihood taken for the posterior. Availability and survival are a filtered sample taken for the population. The pattern illusion is the null hypothesis of chance forgotten. The gambler's fallacy attributes memory to a process without memory. The neglect of rare events models a heavy tail as Gaussian. Overconfidence is bad calibration, and motivated reasoning is choosing the model by the desired result. The mapping fits well at the layer of the Model and is strained at the layer of the Agent. Motivated reasoning is not an ignored model of chance: it is the utility of the Agent contaminating the choice of the Model. And "overconfidence is bad calibration" redescribes the phenomenon in the language of scores without explaining it. Claim (b) is a good teaching map, and it holds as a map, not as a law. The word "bias" thus gains three senses that the discipline separates in order to reunite later: the statistical bias of an estimator, the bias of the process that produced the data, and the cognitive bias of whoever reasons. In all three, the error is not noise that cancels out with more data; it is a distortion that accumulates.

Each layer has its type of uncertainty, and confusing them is a classical error. The assignment of a type to a layer, however, is relative to a description, and the paper reads it that way. **Aleatory** uncertainty belongs to the World: it is the chance of the next toss of a known coin, and no amount of data eliminates it. **Epistemic** uncertainty belongs to the Model: it is our ignorance about the bias of the coin, and it shrinks with each toss. The law of total variance separates the two with precision, once θ is chosen:

$$
\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{random (World)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistemic (Model)}}
$$

The split depends on what is put inside θ. A tossed coin is a deterministic mechanical system: with the initial conditions measured, the face is predictable, and Diaconis, Holmes, and Montgomery (2007) found that a vigorously flipped coin lands on the side it started about 51% of the time. Whoever conditions only on the coin sees aleatory uncertainty; whoever conditions on the hand sees epistemic uncertainty. The line between the two is a choice of coarse-graining, like entropy in statistical mechanics. The layers are therefore roles in an inference, relative to a declared partition, and not regions of reality; the architecture asks that the partition be declared because the type of the uncertainty is not a property of the World alone.

The third, **Knightian** uncertainty, is the most dangerous because it does not appear in the equation. It arises when the space of outcomes itself is incomplete: the event that matters is not one the model can even name. There, more data do not help: they narrow the confidence interval around the wrong answer, as in the case of the turkey, whose slaughter was not a low-probability outcome but no outcome at all. Specification error is its milder neighbour: the outcomes are the right ones and the family of distributions is wrong, which a predictive check can detect and Knightian uncertainty, by definition, cannot.

| Uncertainty | Layer | What more data do | How it is mitigated |
| --- | --- | --- | --- |
| Aleatory | World | Nothing | Safety margins and redundancy |
| Epistemic | Model | Reduce it to zero | Sampling and Bayesian updating |
| Knightian | Model and Agent | Nothing, or worse: false confidence | Scenarios, robustness, stress-testing of premises |
| Specification error | Model | Precision around the error | Predictive checks, widening the family of models, and auditing premises |

*Rectification of 2026-10-18.* The text before this date defined Knightian uncertainty as "the real process is outside the hypothesis space", which is also the definition of specification error, so the two rows of the table said the same thing twice. The two are now separated by what is missing: the outcome space (Knightian) or the right family over a correct outcome space (specification). The mitigation of specification error gained the predictive check, which is the instrument section 6 already uses at the call center.

The mathematical tool that crosses the three layers is modest: Jensen's inequality. For a convex function f, the average result is not the result of the average case:

$$
\mathbb{E}[f(X)] \;\ge\; f(\mathbb{E}[X]), \qquad \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \tfrac{1}{2} f''(\mu)\,\operatorname{Var}(X)
$$

In words: when the system reacts nonlinearly, the variability of the World does not cancel out in the average; it charges a price proportional to its variance. Planning by the average case is ignoring that bill. Section 5 shows how much it costs in a health clinic.

### 3.1 Belief and evidence

Why should degrees of belief obey the rules of probability? Two independent arguments answer. Cox (1946) showed that any consistent measure of plausibility, one that follows common sense, obeys, after a rescaling, the rules of sum and product. Ramsey and de Finetti measured belief by the price of a bet. Whoever declares 60% for rain and 50% for no rain accepts paying 1.10 for two tickets of which only one pays 1, and loses 0.10 for certain. A set of prices admits that "Dutch book" if and only if it violates the axioms.

Coherence, however, is cheap. An agent who commits the conjunction fallacy loses a little per round to the Dutch bettor and much more to whoever knows the world, and making it coherent removes only the first loss. The most visible error is not the most expensive. What punishes disagreement with the World is calibration, not coherence. And when the information is incomplete, Jaynes's (1957) principle of maximum entropy commands choosing, among the distributions compatible with what is known, the one of greatest entropy. A six-faced die with mean 4.5 then receives about 5% on face 1 and 35% on face 6; any other choice supposes something that was not declared.

The prosecutor's fallacy, which the previous section only named, has an exact form. Written in odds, Bayes's rule separates what the evidence says from what was already known:

$$
\frac{P(H\mid E)}{P(\neg H\mid E)}=\frac{P(E\mid H)}{P(E\mid \neg H)}\times\frac{P(H)}{P(\neg H)}
$$

In breast-cancer screening, the disease strikes 1% of the women, the test detects 90% of the cases and gives a positive result in 9% of the healthy ones. In natural frequencies the arithmetic becomes visible: of 10,000 women, 100 are sick and 90 of them test positive; of the 9,900 healthy ones, 891 also test positive. Of the 981 positives, only 90 are sick, about one in ten. When Gigerenzer and colleagues asked that question to 160 gynecologists, only 21% chose "about 1 in 10". The same exchange took the "1 in 73 million" of the Sally Clark case to court, obtained by multiplying 1/8,543 by itself, as if the two deaths were independent, and read as the chance of innocence. An evidence is strong when the likelihood ratio is far from 1, not when P(E | H) is high.

Turing and Good measured that strength in decibans, ten times the decimal logarithm of the ratio, a scale in which independent evidences add up. Independence is the condition most often forgotten: two positive tests with a common cause of error give 51% by naive addition and 30% in the simulated population. The expected value of the weight of evidence is the Kullback-Leibler divergence, which is never negative for the true hypothesis:

$$
D(P_1\,\|\,P_0)=\sum_x P_1(x)\log\frac{P_1(x)}{P_0(x)}\ \ge 0
$$

It says how many observations are enough: to separate a 60% coin from an honest one at odds of 100 to 1, about 229 tosses are needed. It is with that ruler that the 2.2 bits of the Danish fires (section 6) turn out to be little. Evidence also deceives little. Royall showed that, if the null hypothesis is true, the chance that the likelihood ratio ever reaches k is at most 1/k, whatever the sample size:

$$
P_{H_0}\Big(\sup_n \mathrm{RV}_n \ge k\Big)\le \frac{1}{k}
$$

In the book's laboratory, with paths of 500 tosses of a 0.6 coin against an honest one, 0.87% of the paths under the null reach 20 decibans at some point, against a bound of 1%; the number is exact, counted toss by toss over the paths that have not yet crossed.

### 3.2 The filter before the data

The data almost never arrive raw: they went through institutional, mechanical, or behavioural screens. Meng (2018) showed that the error of any sample mean is the exact product of three factors, quality, quantity, and difficulty:

$$
\bar Y_n - \bar Y_N = \rho_{R,Y}\times\sqrt{\tfrac{1-f}{f}}\times\sigma_Y
$$

Here ρ is the correlation between entering the sample and the response, and f is the fraction of the population sampled. In a drawn sample, ρ is tiny, of the order of 1/√N. When entrance depends on what is being measured, size does not save it. In 1936, the *Literary Digest* received 2.38 million ballots and predicted a victory for Landon with 57%; Roosevelt had 60.8%, and Gallup got the winner right with about 50 thousand interviews. In a simulation on the scale of 1936, a correlation of −0.21 is enough for 2.4 million ballots to miss by 18 points and be worth the same as a random sample of about 7 people. More biased data give more certainty about the wrong answer: it is the turkey again, now in a ballot box.

The filter also fabricates relations. Berkson (1946) showed that two independent diseases appear associated among inpatients, because each one raises the chance of admission: looking only at the selected ones is conditioning on a collider. In a course simulation, talent and likeability have a correlation of +0.002 in the population and −0.669 among those who pass a threshold. The inverse case is Simpson's paradox. Charig and colleagues (1986) compared two kidney-stone surgeries. Open surgery wins on the small stones (93% against 87%) and on the large ones (73% against 69%), but loses overall (78% against 83%), because the difficult cases went to it. Weighting the strata by the whole population, it has 83.3% and the other one 77.9%. Conditioning on a confounder closes a spurious path; conditioning on a collider opens one. "Controlling for everything" is not neutral, and whoever decides which computation to make is the graph, not the table.

### 3.3 From association to intervention

Pearl distinguishes three steps of question. Association asks P(y | x): do those who take the medicine have fewer heart attacks? Intervention asks P(y | do(x)): if I take it, does my risk fall? The counterfactual asks whether I would have had a heart attack without taking it. Each step demands more of the model than the previous one, and the bridge between the first two is a causal graph, which the data alone do not supply. When a set Z blocks all the paths that enter the treatment from behind, without containing any descendant of it, the back-door criterion holds:

$$
P(y \mid do(x)) = \sum_z P(y \mid x, z)\,P(z)
$$

The classic case is that of hormone therapy. In 1991, the cohort of the Nurses' Health Study, with about 48 thousand women, reported a relative risk of 0.56 of serious coronary disease among estrogen users. In 2002, the randomized Women's Health Initiative trial, with 16,608 women, found a hazard ratio of 1.29. The users in the cohort had other habits, another income, and another access to physicians. The comparison is not perfect (isolated estrogen in one case, combined with progestin in the other), but the lesson stuck. Adjusting for "everything that was measured" does not solve it either: adjusting for a mediator erases the effect, adjusting for a collider fabricates association, and adjusting for an instrument amplifies the bias that remains.

The language of potential outcomes, from Neyman and Rubin, gives the same lesson in one line. Each person has two possible outcomes, Y(1) with the treatment and Y(0) without it, and only one is observed. The naive difference between treated and untreated decomposes exactly like this:

$$
\mathbb{E}[Y\mid D{=}1]-\mathbb{E}[Y\mid D{=}0] = \underbrace{\mathbb{E}[Y(1)-Y(0)\mid D{=}1]}_{\text{effect on the treated}} + \underbrace{\mathbb{E}[Y(0)\mid D{=}1]-\mathbb{E}[Y(0)\mid D{=}0]}_{\text{selection bias}}
$$

Drawing the treatment at random zeroes the second term. It is exactly what section 6 finds in the LaLonde data.

### 3.4 When the mean stops existing

Jensen is not an isolated trick; it is the common root of several pathologies. The positivity of the Kullback-Leibler divergence is a case of it, and so is the breaking of ergodicity. In a multiplicative dynamics,

$$
X_{t+1} = X_t M_t, \qquad \mathbb{E}[M] > 1 \;\text{ and }\; \mathbb{E}[\ln M] < 0,
$$

the ensemble mean grows while almost every trajectory decays (section 7).

In the limit, the mean stops existing. In a Pareto distribution, with P(X > x) = x^(−α), the moment of order k exists if and only if k < α. With α = 1.1, the mean exists and equals 11, but the mean of a million observations has median 8.3, stays below 11 in 91% of the samples, and the usual 95% interval contains the true value in only 26% of them. The reason is that the fraction of the mean coming from values above a level m is m^(1−α): in the median, 28% of the mean lies above the largest value that the million-observation sample saw. For a typical error of 1%, about 10²² observations would be needed. Taleb's κ measures that distance up to the regime in which the law of large numbers works in practice. A verification of the course's own showed that the published formula for "how much data" overestimates the number by factors of about 3 and 8, which changes the arithmetic, but not the message.

When the tail rules, the Gaussian fails precisely where it matters. By the Fisher, Tippett, and Gnedenko theorem, the normalized maximum of a sample can only tend to one of three families, Gumbel, Fréchet, or Weibull. A Gaussian fitted to heavy-tailed daily returns underestimates the 1-in-100-year loss in every repetition, by a factor of several; the extremes distribution gets it right in the median, with a wide interval that is the honest price of a tail estimated from few extremes. The Danish fires of section 6 put a number on that price: a tail index of 1.81, with an interval that crosses 2, so that the data do not even say whether the variance exists.

## 4. Architecture: an auditable epistemological machine

A discipline only becomes practice when it gains instruments. The project proposes to operate Stochology as an **auditable epistemological machine**: a system in which every empirical claim becomes a piece of executable code, and in which no conclusion is accepted without passing automatic tests. The idea is not a text searcher that summarizes the literature, but an environment in which hypotheses are simulated and attacked.

**The hypergraph.** Knowledge is stored as a typed graph, G = (V, E\_tree, E\_graph, H, Φ). The nodes (V) are concepts, hypotheses, variables, and data sets; each node mandatorily declares its layer (World, Model, or Agent), its type of uncertainty, a formal definition, and a pointer to reproducible code with a fixed seed. The tree edges (E\_tree) record what depends on what. The cross edges (E\_graph) are of three types: mathematical bridges (Jensen's inequality linking distant modules), causal edges of the do(x) type, and **pathology edges**, which mark misalignments: linking a nonlinear World to a Model that only looks at the average case triggers the fallacy-of-the-mean alert.

**The five lenses.** The hyperedges (H) involve whole groups of nodes and work as questions that are asked of any problem:

- **Fluxonomy**: how probability and information flow and are lost.
- **Chrononomy**: whether the average across many coincides with the average over time of a single one (ergodicity, ruin, Kelly's criterion).
- **Scalonomy**: whether the tail is heavy to the point that the mean does not even exist.
- **Morphonomy**: whether the structure found survives when the environment changes.
- **Crisonomy**: whether the information is worth its cost and whether the forecasts are calibrated.

Each lens is more than a question: it has a quantity that it measures and a critical habit, and it crosses the concepts without adding objects. Fluxonomy reads as a current everything that circulates between the layers, the probability that leaves one state and enters another, the information that passes through a channel. Its base law is bookkeeping: the probability of a state varies by what enters minus what leaves, ∂ₜp = −∂ₓJ, and the habit is to ask whether the books balance. One version of that bookkeeping is the data processing inequality: if Z is obtained from Y, then I(X; Z) ≤ I(X; Y). Reprocessing, summarizing, or copying a datum never creates information, and three newspapers that publish the same research are worth one. Chrononomy measures mixing, relaxation, and escape times, and asks which time is being spoken of: that of the process, that of the observer, or that of the simulation. Scalonomy measures the tail exponent, the effective size, the horizon, and the capacity of the model. Morphonomy rewrites the same model in two forms, because two representations can give the same predictions and still authorize different conclusions under intervention, and only what is conserved across environments serves outside them. Crisonomy measures loss, proper score, and value of information, and charges every number the record of hits at the frequency it promised.

**The agents.** The work is divided among specialized agents, eight tracks coordinated by an orchestrator (the book names the eight safeguard bands A1 to A8; the table keeps the F-labels of the plan). They do not converse in free language, which is ambiguous and hallucinates; they exchange structured **cards**, in YAML. A card is not a summary: it is a closed list of questions that are asked of a text, a number, or a model. All of them inherit four starting questions. What is the generative process, written in simulable form, and in which layer can the claim err? Is the result a nonlinear function of something that varies, which would make the average case misleading? What is the unit that is in fact independent, and was the margin computed with it? And, if the number came from data, from which likelihood did it come, and what does the p-value say and not say? Each module adds its own. The evidence card asks for the base rate, the two likelihoods, and the posterior computed in two ways, in odds and in a tree of 10,000 cases, which must coincide; the process card asks what the physics of the noise is; the causal graph card begins at the step of the ladder where the statement is; the model critique card closes the series and says in which item each previous card enters.

| Agent | Role | Guarantee it delivers |
| --- | --- | --- |
| F1 Code | Turns hypotheses into executable generative processes | Fixed seed and property tests |
| F2 Data | Audits the entry of the World | Data contract and the right independent unit |
| F3 Lifecycle | Tests the model in time | Validation on future data, without leakage |
| F4 Frontier | Runs the heavy inference | Calibrated samplers and guaranteed coverage |
| F5 Governance | Takes care of legality and privacy | Privacy budget in bits lost |
| F6 Fairness | Asks who pays for the error | Audit of error rates between groups |
| F7 Domain | Translates problems from other areas | Expert elicitation and scope term |
| F8 Decision | Converts the result into action | Natural frequencies and exit clause |
| Orchestrator | Coordinates and runs the safeguard panel | Checklist of declared and anchored safeguards before approval |

Each row of the table hides a craft. The data agent (F2) begins with a decision that nobody sees: what a row is. A row per person, per day, or per event changes the mean, the variance, and the margin, and that is why the grain is declared and proved with a count. It is the independent unit seen from the side of the table: when 225 thousand flights are worth the same as 364 days (section 6), what failed was the grain. The same agent hunts the sentinel value, the missing one recorded as 0, −1, or 9999 that enters the sum as if it were data, and demands that ingestion be idempotent, because every datum arrives more than once and running the load twice has to give the same number.

The frontier agent (F4) knows that, when the posterior has no closed form, the computer returns a sample, not an answer. The convergence diagnosis measures whether several chains agree among themselves, not whether the model is right. The strongest test is calibration by simulation: a parameter is drawn from the prior, data are simulated with it, and the sampler is run. If the algorithm is honest, the position of the true value among the L samples of the posterior is uniform:

$$
\operatorname{rank}\big(\tilde\theta;\,\theta^{(1)},\dots,\theta^{(L)}\big)\sim \operatorname{Uniform}\{0,\dots,L\}
$$

For black-box models, conformal prediction gives a guaranteed coverage without supposing the shape of the distribution, P(Y ∈ C(X)) ≥ 1 − α; it was an adaptive version of it that H3 used under drift.

The governance agent (F5) starts from the fact that personal data is not a type of file: it is the combination of fields that decides, and postal code, date of birth, and sex, together, identify almost any person. Before everything it asks on what legal ground and for what purpose the data is processed, and it recalls that those who consent are not a random sample, so that the inference inherits that selection bias. Aggregating is not enough. Differential privacy limits how much the presence of one person can change any answer. For databases D and D′ that differ in one person,

$$
\Pr[M(D)\in S]\ \le\ e^{\varepsilon}\,\Pr[M(D')\in S]
$$

and ε is the budget, spent on each query. The cheapest measure remains not collecting.

**The cycle.** A question goes through six stages: translation and elicitation (F7); audit of the data, of causality, and of ethics (F2, F5, F6); simulation against a null model (F1, F4, F3); passage through the five lenses; safeguard panel, which sends the work back to the stage that failed; and, only then, the communicated decision (F8).

The safeguard panel does not read reports: it reads the card, and for each track it checks that the safeguard against a named attack is declared and anchored. It runs no attack itself; the attacks that run (a tampered trail, a planted defect) live in the gates. Three failures eliminate the work before any discussion of merit: temporal leakage, a data contract violated without treatment, and the absence of a named human responsible. Each track names its attack. The data one should receive a batch with corrupted records and must isolate them. The model in operation receives, live, a batch with strong drift. The inference instrument receives a pathological data set, to see whether its uncertainty denounces the inadequacy. The fairness audit has to invert the chosen metric and show who would start paying. The decision document faces a crisis that violates its own exit clause. A complete work with one declared weak track is worth more than a uniform work without an admitted gap.

**The five invariants.** Nothing enters the graph without passing five gates, which work as database locks:

1. **Φ1, simulability**: every claim becomes code with a declared seed and tests. Prose is not enough.
2. **Φ2, margin and unit**: every number comes with a margin of error, computed on the unit that is in fact independent.
3. **Φ3, diagnosis and severe test**: before reading a pattern, the sample filter is audited, the causal graph is drawn, and the hypothesis is confronted with a simulated null model.
4. **Φ4, ecological scope**: the decision is judged by the environment in which it will operate, and leaves with a scope term and an objective clause for when to abandon it.
5. **Φ5, question resolution**: when the unit of decision is the day or coarser, aggregate resolution (R0) fails and the temporal band compatible with the question is required.

In order not to explode in cost or in false discoveries, the research passes through a funnel. First one cuts what, even with a perfect answer, would not change any decision (expected value of information). Then one cuts what is not identifiable with the available data. Then, what does not beat the null model. Only what remains receives expensive inference. Every attempt is recorded, including the ones that failed, so that the final report declares how many specifications were tried.

Each cut of the funnel has an arithmetic behind it. The first is the value of information. If protecting costs C and not protecting costs L when the event occurs, one protects when p > C/L, and a piece of information is worth what it can move the probability to the other side of that threshold, not the bits it carries. The value of knowing the state of the world before acting has a ceiling,

$$
\mathrm{EVPI} = \mathbb{E}_\theta\big[\max_a U(a,\theta)\big] - \max_a \mathbb{E}_\theta\big[U(a,\theta)\big]
$$

and the value of any real test lies between zero and it. In the book's example, a disease has a prevalence of 0.3, treating costs 1 and leaving a sick person untreated costs 10, so one treats when the chance passes 0.10. A sensitive test (sensitivity 0.90, specificity 0.70) carries 0.241 bit about the state and is worth 0.2200, against a ceiling, the EVPI, of 0.70. A specific test (0.30, 0.95) still carries 0.079 bit and is worth exactly zero, because even its negative result leaves the chance of disease at 0.24, above the threshold, and does not change the action. Bits are not value.

The second cut is identifiability: a parameter is identifiable when different values generate different data. At the start of an epidemic, the cases grow as exp((β − γ)t), and the data determine the difference β − γ, not the ratio R₀ = β/γ: two pairs with the same difference give the same curve, and the predicted final size of the epidemic, which depends on the ratio, is left open by the early data. Taking the duration of the infection from other studies narrows R₀. This is not a defect, as long as one declares that the margin of the conclusion has become that of the source.

The third cut, the null model, is the defence against the garden of forking paths. Simmons, Nelson, and Simonsohn (2011) simulated four common freedoms in an experiment without effect (when to stop collecting, which variables to use, which cases to exclude, which comparisons to report) and saw the false-positive rate rise from 5% to about 61%. The book's laboratory redoes the garden over 4,000 experiments with no effect at all: with the four freedoms, someone finds p < 0.05 in 20.3% of them, and in 42.3% with a large family of comparisons and optional stopping, against 5.2% for the pre-registered test. Ioannidis (2005) showed that the predictive value of a significant finding depends on the power 1 − β and on the odds R that a hypothesis in the area is true:

$$
\text{VPP}=\frac{(1-\beta)R}{R-\beta R+\alpha}
$$

The Open Science Collaboration (2015) replicated 100 psychology studies: 97% of the originals had p < 0.05, against 36% of the replications. Not even cross-validation escapes. In a world of pure noise, with 50 cases and 50 predictors unrelated to anything, drawing 500 models and keeping the one with the lowest validation error produces a winner that looks better than predicting zero in 73% of the repetitions and is not better in any of them in the face of new data. That is why Mayo's rule is that of the funnel: a result is only evidence if the procedure would probably have gone wrong had the claim been false. In the same laboratory, the same "p = 0.04" has severity 1 − p = 0.96 when it comes from a pre-registered test, while in the garden the procedure accuses an effect that does not exist 42% of the time, so the claim passed a test it would have passed anyway.

The reverse also holds: a fit is only evidence if the model could have failed. If a half-plane separates 50 points perfectly, that would have a chance of 2 × 10⁻¹² if the labels were chance, about 39 bits of evidence. A table that stores every label also separates them and brings no bit at all, because it would separate anything. It is Popper's falsifiability in the form of arithmetic, which the theory of Vapnik and Chervonenkis measures by the capacity of the family of models.

## 5. A first experiment: the queue at the health clinic

The first test of the architecture (the H0 horizon of the project plan) was deliberately modest: a simulated health clinic, which the book carries in Chapter 5 and returns to in Chapter 12. The clinic serves 12 patients per hour and receives, on average, 8. The claim to examine is that of any management spreadsheet: "with this average demand, the average wait will be about 9 minutes".

The system did what the invariants demand. It translated the claim into an executable generative process, with a fixed seed (Φ1), and ran five worlds that have exactly the same average demand, but different variabilities:

![Average wait per simulated world in Lab 0 (H0, h0-lab0/resultados.json)](figuras/lab0-espera-por-mundo.png)

*Average wait per simulated world in Lab 0 (H0, h0-lab0/resultados.json).*

If the demand is constant, the promise is kept: 9.1 minutes. If the days alternate between calm (5 patients per hour) and outbreak (11), the average wait more than doubles, to 19.3 minutes, and about one in every four patients waits more than half an hour. (The Python generator that preceded the Rust apparatus gave 9.3 and 20.7; the book keeps the pair in its Appendix E.) Nothing changed in the mean; the variability changed, and the queue is a convex function of the demand. It is Jensen's inequality collecting its bill. The last world shows the other side: reinforcing the staff only on outbreak days brings the wait down to 2.5 minutes.

It is worth looking at where the promise comes from. It comes from the classical formula of a queue with one server,

$$
W(\lambda)=\frac{\lambda}{\mu(\mu-\lambda)}
$$

which, with λ = 8 and μ = 12 per hour, gives 10 minutes of wait in the long run. The same formula, applied to days that alternate between 5 and 11 patients per hour, would give an average of 29.3 minutes, and the simulation gives 19.3. The difference has three causes that teach something. A 10-hour day is not the long run of the formula: on outbreak days, the wait stays at 28.2 minutes, not at the 55 that it predicts. Besides, 68.8% of the patients (11 in 16, approximated by 70%) arrive on outbreak days, so that the average per patient weighs the bad days more. And the formula is not wrong: it answers the long-run wait with constant demand, when the manager was asking how many wait too long on the bad days. Jensen does not even need curvature to bite. In an exam with mean 6.2 and a passing grade of 6, the "average student" passes, but 45.1% fail.

The experiment produced three results that go beyond the didactic example.

1. **The null model arbitrates the margin of error (Φ2).** Whoever treats each patient as an independent observation obtains a margin of ±0.07 minute; whoever treats the day as the unit obtains ±0.51, seven times larger. Which one is right? Instead of discussing the convention, the system randomly split 4,000 simulated days into two halves and measured the difference between them. The observed deviation was 0.216; the margin by day predicted 0.227 and the margin by patient predicted 0.047. Pseudoreplication stopped being a matter of taste: it was refuted by the data.
2. **The severe test (Φ3).** The difference between the constant world and the world with outbreaks, 10.2 minutes, lies dozens of deviations away from what chance would produce. In 2,000 random splits, the null model never went beyond 0.81 minute.
3. **The median trap.** In the world with outbreaks, the median wait is 8.1 minutes in H0 (8.6 in Chapter 5 of the book, whose day is Poisson), close to the 9 minutes that the claim promises as a mean. A manager who looks at "the typical patient" will see the promise kept. The whole difference is in the tail, and that is why the architecture prohibits reporting a mean alone: it demands the median, the quantiles, and the cumulative curve.

Pseudoreplication has a theory behind it. In 1913, Markov counted vowels and consonants in the first 20,000 letters of *Eugene Onegin* and showed that the law of large numbers survives dependence. But dependence changes how much the sample is worth. As the letters tend to alternate, a block of 100 of them is worth about 330 independent letters; in a "sticky" chain, in which each state tends to repeat, 100 observations are worth well less than 100. The effective size of a sample is a property of the process, not of the count, and that is why the patients of the same day, who share the same demand, do not count as independent observations.

All of this passed through the gates and reproduced, in the Rust apparatus (`labs --chapter 5` and `stochology-horizons`), the declared divergence against the old generator (161,867 and 160,220 patients). The result is small, but its value lies in the method: anyone can run the code again and obtain the same numbers, with the same hash.

## 6. From the simulated to the real: what the data said

After Lab 0, the program went through the four horizons of the plan with real public databases and passed all four, the last two only after several rounds of failure. The failures were kept as a result: each change of criterion, of test, or of data was declared, with a hash, before running, and nothing was adjusted after seeing the result. The research was closed on October 15, 2026; code, hashed data, and results are in the project repository.

| Horizon | Real data | Exit gate | Result |
| --- | --- | --- | --- |
| H0 Seed | 2.65 million calls from a bank call center (2003) | Φ1 to Φ4 and clean reproduction | Passed |
| H1 Complete cycle | Dow Jones 1885–2023; LaLonde (NSW and CPS); Danish fires 1980–1990 | A project approved in the safeguard panel | Passed: 2 of 3 projects approved; the heavy-tailed one failed |
| H2 Transfer | 12 convenience domains; then databases with known interventions, physical chambers, gene deletions in yeast, and an FDA cardiology trial | Non-empty causal invariant set (ICP) in 3 areas | Passed on the seventh round: 4 areas; the background audit (section 8) exchanges Education for replicated Genetics, and the gate stands with three areas |
| H3 Continuous mesh | 6 flows in replay under a clock, 453,973 forecasts; then 2023 flights, 2020–2021 arrests, and traffic stops in Rhode Island | Calibration out of time, zero leakage, Goodhart control | Passed on the third attempt of the arrests flow; low skill (+0.019) |

### Real H0: the call center

Lab 0 supposed a demand that alternates between 5 and 11 per hour. In the real H0, the demand is observed: 164 business days of a North American bank call center, counted every 5 minutes. The claim is the standard practice of the sector: staffing every half hour by the Erlang C formula over the average rate of the history guarantees 82% of the calls answered within 20 seconds, every day.

In the world that the claim describes, no day falls below the target. In the real arrivals, about 31 in every 100 do (30.9% of the vault days in the exact reconstruction, 28.4% in the event-by-event engine that verifies it). The demand is not Poisson: between days, the deviation is about 2,900 calls, where Poisson would predict 180. And Jensen's inequality shows up inside the formula itself: applied to the real rate of each day, the Erlang C of the claim gives on average 57%, not 82%, because the level of service is concave in the load. The independent unit, again, is the day: the null model of halves gave a deviation of 0.0474, against 0.0470 predicted by the day and 0.0033 by the call.

It is the textbook case of the predictive check, from Box to Gelman and Shalizi: simulating replicas of the fitted model and seeing in which statistic the real data fall outside them. In a course example, a Poisson model passes on the mean, fails on the variance (no replica comes close) and underestimates by 42 times the chance of a peak day. A model can serve to size the average staff and be dangerous for the peak shift; the right question is not whether it is true, but for which decision it is good enough.

![Days below the target by staffing rule at the bank call center (real H0, h0-real/README.md)](figuras/central-regras-de-escala.png)

*Days below the target by staffing rule at the bank call center (real H0, h0-real/README.md).*

The decision (Φ4) compares staffing rules of similar cost. Adding 12 attendants across the board, about 7% more agent-hours, brings the failing days from about 31 to about 10 in every 100 (9.9%); spending the same money where the risk is, by day of the week and by a margin for the residual variation, brings them to about 5 (4.9%). The figures are those of the exact replay of the vault; an independent engine with simulated arrivals (28.4%, 8.6% and 4.9%) keeps the same order, and the decision does not change with the engine. What remains assumed is the service time and the patience of the clients, which the database does not bring, and the background audit swept the whole mesh (service from 3 to 6 minutes against patience from 1 to 10): no point switches the winning rule, the allocation by risk wins in 20 of the 24 points and ties in the four of lowest patience, and in the worst case it holds 14.2% of the days below the target against 37.7% for the rule of the claim. The service axis barely moves the result; patience moves it. The health domain stayed out because the DATASUS portals were beyond the reach of the network — which the audit worked around through the open mirror of PhysioNet (section 8).

### Real H1: three claims, three destinies

**Leveraging the Dow Jones.** The claim: "leveraged 3 times, the index yields 3 times more, and whoever holds for decades always wins". The arithmetic sum would give about 0.20 per year; the growth over time was 0.051 without leverage and 0.060 with 3 times. In 20-year windows, 96% end above the starting point without leverage and 64% with 3 times. With 3 times, the average wealth reaches 19 times the capital, but the median stays at 2.1: the ensemble mean and the typical trajectory separate, as Chrononomy predicts. October 19, 1987 alone takes 68% of the leveraged capital. The refutation holds for "yields 3 times" and "always wins", not for "leveraging never pays".

**The LaLonde experiment.** Comparing participants of a training program with workers from a national survey gives a negative effect on employment (−10.7 points). The randomized experiment shows +11.1. Adjustment by propensity score reaches +9.6, close to the right value, but the severe test shows that it got it right without deserving it: an inert treatment, redrawn and weighted the same way, lands between −10.3 and +11.9 points in 95% of the replicas, centred near +0.9, and the weighted estimate of the real data, +9.0, sits inside that band (p = 0.08). A placebo, using 1975 employment as the outcome, reveals that the selection is still there.

The decomposition of section 3.3 explains the flipped sign. The comparison with the national survey adds to the effect of the program a selection term, the difference between what the participants and the non-participants would have earned without the program, and that term is large and negative, because those who seek a training program were in a worse situation. The draw zeroes that term, and the propensity score tries to estimate it from the observed variables. The placebo with 1975 employment, prior to the program, is a direct estimate of it, and it did not give zero.

**The Danish fires.** A reinsurer sees 36 months without a claim in a high band of losses and concludes that the risk is negligible. The clean window is worth only 2.2 bits of evidence, and Laplace's rule of succession gives 1 chance in 38 per month, not zero. With the leverage that the claim would justify, the reinsurer is ruined in August 1989. But the orchestrator failed the project: in the vault, no forecaster beat the simple historical frequency, because the claims grew 4.6% per year. The claim loses, but the model does not win the right to decide.

The course has an exact analogue. A strategy that sells insurance gains 1% per week and loses 50% with probability 0.4%. A history of 250 weeks shows no loss in 37% of the cases, and whoever estimates the chance by the observed frequency concludes that it is zero and leverages. Among 10,000 simulated investors who bet by Kelly's criterion with the observed frequency, 36.8% see no loss in their history and go all-in, and 1.9% are ruined within 1,000 weeks, although that rule has the highest average wealth; with Laplace's rule of succession, nobody goes all-in and nobody is ruined. (An earlier version of this laboratory, without the ceiling of one on the fraction, ruined 35.8%; the book keeps both numbers.)

### H2: the gate that cost seven rounds

H2 tested whether the method transfers between areas. Twelve real domains (education, logistics, agronomy, climate, credit, health, law, among others) were plugged in through the same standardized package; ten of those packages were written by agents that had read only the contract and two examples. The gate chosen by the plan was demanding: Invariant Causal Prediction (ICP), which looks for the causes whose relation with the outcome does not change from one environment to another, should find a non-empty set in at least three distinct areas.

ICP starts from a formal statement. If S is the set of the direct causes of Y, the distribution P(Y | X_S, E = e) is the same in every environment that intervenes on the other variables without intervening on the equation of Y. A causal relation survives the change of environment; a convenience correlation does not. The method returns the intersection of all the sets that pass the invariance test, and its guarantee is asymmetric: with probability of at least 1 − α, the set returned contains only direct causes, but nothing guarantees that it contains all of them. Abstaining, returning the empty set, is a result, not a failure. It is also an answer to Lucas's critique (section 7): only what is invariant between regimes deserves use when the regime changes.

On the first round, it found it in one, law. In the other domains, it either rejected all the sets, because the convenience environments (year, region, airport) act directly on the outcome, or accepted so many that the intersection became empty. The diagnosis was that the standard ICP, with large samples, rejects any deviation from the linear Gaussian model. Six rounds followed, each with the change declared and recorded with a hash before running: databases in which the environment is a documented intervention, a non-parametric invariance test, a rule that discards sets without testing power, physical chambers with the causal graph known, gene deletions in yeast, and, finally, a cardiology clinical trial.

The cleanest moment was the light tunnel. In a physical chamber built to study causality, ICP recovered exactly the true parents of the electric current, the three LEDs, and left out sensors strongly correlated with it. It was the first time in the project that the method was checked against a known truth, and it got it right. The wind tunnel, in the same battery, rejected even the true parents, probably because of atmospheric drift and an additive fit where the physics is quadratic.

**The Genetics caveat, and what came after.** The sixth round reached the three required areas with Education (randomized classes), Physics (light tunnel), and Genetics (gene deletions in yeast, Kemmeren et al., 2014). But Genetics was fragile: it came from a single gene among ten drawn, in an ICP that rejected 1,022 of the 1,024 sets, with the best p = 0.007, just above the threshold of 0.005; and validation by intervention contradicts two of the three genes of the set. The owner approved the exit of H2 with that caveat recorded.

The seventh round removed the weight from the caveat. In an FDA trial in which volunteers received four drugs, ICP pointed as invariant causes of the prolongation of the QT interval exactly the concentrations of the three drugs that block the hERG channel, which is what pharmacology expected, and left out measurements of the electrocardiogram itself. In verapamil, the drug that the model never saw, the invariant model predicted well (skill +0.57), while the model with all the variables exploded, because verapamil pushes the PR interval outside everything that was in the training. It is the textbook example of what invariance protects. Without Genetics, three areas remain, Education, Physics, and Pharmacology, two of them checked against an external truth. One caveat stays: the test supposes independent measurements, and the trial has 16 measurements per volunteer.

There is a positive lesson hidden in the failure: **the effect transports better than the set**. Even with ICP rejected, the model predicted the never-seen environment well in 9 domains. And pseudoreplication showed up everywhere: in the New York flights, 225 thousand flights are worth, in independent information, the same as 364 days.

### H3: the continuous mesh

H3 ran six real flows (energy, the bank call center, the stock exchange, flights, cyclones, and arrests) as if they were live, under a clock that only releases each datum at the instant in which it would become knowable. The three criteria ended up passing, the last one on the third attempt.

- **Zero temporal leakage.** In 453,973 forecasts, none used information from the future. A canary proved that the test would work: a column identical to the label, published at the instant of the label, gave no gain at all with the honest clock, and gave gains of +0.86 to +0.92 with the clock turned off.
- **Goodhart under control.** At the call center, whoever is not served calls again, and the forecast starts to measure its own effect. A naive forecaster gets what it observes right with 1% error and, even so, overestimates the true demand by 29%: the metric approves a model that is wrong about the world. Using randomly drawn shocks in staffing as an instrument, the error falls to 1.7%. That loop, however, is declared over real arrivals, not measured, and the estimator had four versions before passing.
- **Calibration out of time: passed on the third attempt.** In the flights, the delays of the last quarter came out 2.7 points above the predicted one, for a tolerance of 2. In the arrests, the calibration error was 0.036, for a tolerance of 0.03, with only 1,488 cases. The design lesson is precise: the circuit breaker compared the model with the reserve model, not with the calibration, and for that reason it did not see the drift of the mean. With a continuous recalibration of the intercept, declared beforehand, the flights passed in a new vault, the same quarter in 2023 (ECE 0.008). The arrests did not: with Toronto data from 2020 and 2021, the model gets the mean right, but is underconfident in the extremes (ECE 0.067), a slope error that the intercept correction does not reach. The correction, a continuous recalibration that also adjusts the slope, entered as a new specification and was tested in a vault that nobody had seen: 24,466 traffic stops in Rhode Island, from 2013 to 2015. It passed with room to spare (ECE 0.0048), with two caveats: the question changed, from release after arrest to arrest at a stop, and the skill is small (+0.019), because the event is rare. The calibration is good; the resolution is not. And, in a model blind to colour, the false-positive rate against Black drivers was almost double that of white drivers in 2013 and 2014.

**The streams, rebuilt.** The numbers above are the record of the research, kept in the repository with their hashes. On October 19 the six streams were rebuilt in Rust on the public data, with a specification declared before running (`labs --streams`), so that a reader can redo them; the arrests of 2020–2021 and the Rhode Island stops have no copy that can be rebuilt, and stay as recorded. The record and the rebuild agree on four streams and disagree on two, and the book teaches with the rebuilt numbers while its Appendix E keeps the record's. Energy covers 0.886 of the half hours at the 90% level (record 0.887) and the bank 0.884; the cyclones have ECE 0.0121 and the Toronto prisons of 1997–2002 ECE 0.0354 (record 0.036), failing the gate while beating the climatology (skill +0.0812). The two disagreements: the market's 99% value at risk is crossed in 1.53% of 6,011 sessions, with the crossings clustered (Kupiec p = 0.00, Christoffersen p = 0.02), and fails where the record had it passing at 0.98%; and the 2013 flights pass raw (bias −0.0171, ECE 0.0260) where the record failed them by 2.7 points of bias. The 2023 vault then reverses the reading of the correction: in the rebuild the raw model fails (ECE 0.0646) and the corrected one passes (0.0177), so the intercept correction, which the record said was not put to the test, is decisive. A rebuild that disagrees with the record in two places out of six is not a reason to discard either; it is the size of the dependence on choices the specification left open, and the book keeps both.

The sentence "the calibration is good; the resolution is not" has a precise arithmetic, Murphy's (1973) decomposition of the Brier score:

$$
BS = \underbrace{\text{REL}}_{\text{calibration error}} - \underbrace{\text{RES}}_{\text{resolution}} + \underbrace{\text{UNC}}_{\text{uncertainty of the events}}
$$

The climatologist, who always states the historical rate, is perfectly calibrated and has no resolution at all: it is calibrated and useless. The practical asymmetry is that calibration is fixed afterwards, with a history and a regression, and resolution is not. In a course example with 4,000 events, recalibrating an overconfident forecaster takes its Brier from 0.1956 to 0.1851, almost that of the honest expert (0.1846); a noisy forecaster improves little, from 0.2154 to 0.2089, because the lost information does not come back. It is what H3 found: continuous recalibration fixed the calibration of the flights and of the traffic stops, but could not create the resolution that the data did not have. The Brier belongs to a special family, that of the proper rules, in which whoever believes in q minimizes the penalty by declaring q. The hit rate is not proper: whoever believes in 70% is indifferent between declaring 51% and 99%.

The lesson of the circuit breaker also has a theory. A model in use gets worse for three reasons, and each one demands a different answer. In data drift, the world changed; in concept drift, the relation between the attribute and the outcome changed; in performativity, the model itself changed the world that it predicted. There is also a fourth situation, which is not getting worse: the metric falls because the population served changed. Without separating the four, the report orders a retraining of what was not wrong. That is why the watcher looks at the input distributions before the error appears, every alarm has a threshold, a window, and hysteresis, and every incident closes with a chronology, a root cause, and a named responsible.

The imbalance against Black drivers does not require intention to appear. When the base rates differ between groups, the same threshold produces different error rates, and Chouldechova's (2017) identity shows why:

$$
\text{FPR}=\frac{p}{1-p}\cdot\frac{1-\text{VPP}}{\text{VPP}}\cdot(1-\text{FNR})
$$

With predictive value and false negatives equal between groups, different prevalences p force different false-positive rates. Equalizing the approval rates, equalizing the error rates, and equalizing the calibration by group are incompatible demands when the prevalences differ and the predictor is not perfect. Saying "I want fairness" is not yet a decision: one must declare which equality was chosen, how much it costs in accuracy, where the error migrated to, and who starts paying for it.

What the failures have in common is that they were foreseen as a possibility and recorded as a result. The system declared how many specifications it tried at each stage and kept the "from the literature" priors marked as unverified, since no human expert was consulted. The 21 risk opinions were read and signed by the owner, four of them about decisions that involve people, and the scale stayed below the target: the hypergraph of H3 has 454,178 nodes, 45% of the predicted million.

## 7. New horizons

If the old frontiers separated disciplines, the new horizons of Stochology are born precisely from crossing them. Five of them seem the most promising.

**From argument to artifact.** Science publishes claims in prose and, at most, tables. Stochology proposes that the unit of knowledge be an executable artifact: a claim that comes with its generative process, its null model, and its test. This changes what disagreeing means. Instead of two narratives, we have two programs, and the divergence locates itself in a premise that can be altered and run again.

**Artificial agents with epistemic discipline.** Language models write fluently and err with confidence; they are, in a precise sense, agents of layer L3 with shortcuts of their own. The architecture does not try to make them infallible. It surrounds them with gates: they do not do arithmetic in their heads, they do not converse in free language, they do not approve their own work. The question stops being "is the AI right?" and becomes "what the AI claimed survived which tests?".

Evaluating one of those agents is, in itself, a stochological problem. A system that returns text does not get it right or wrong like an arithmetic; what one can have is a rubric, an explicit criterion that another person can apply, with the disagreement between evaluators measured and discounted for chance:

$$
\kappa=\frac{p_o-p_e}{1-p_e}
$$

A rubric in which trained evaluators disagree half the time measures the evaluator, not the system. There are two risks of its own: contamination, when the test cases have already appeared in the training, and the silent swap of the model by the supplier, which obliges every evaluation to record version and date.

**Time and ruin.** Chrononomy takes seriously an idea that classical economics left aside: the average across many people is not the average over the life of one. In a multiplicative game, the expected value can grow while almost all individual trajectories shrink. For whoever decides once, with a single patrimony, the right question is the growth rate in time, and avoiding ruin comes before maximizing the expected value. In the Dow Jones, this stopped being theory: with 3 times leverage, the average wealth reached 19 times the initial capital, and the median, 2.1 (section 6).

The canonical example is Peters's coin. One bets all the wealth; heads increases it 50%, tails takes 40%. The expectation grows 5% per round, but what accumulates in a trajectory is the logarithm:

$$
\ln \mathbb{E}[M] \approx +0.049, \qquad \mathbb{E}[\ln M] = \tfrac12\ln 1.5 + \tfrac12 \ln 0.6 \approx -0.053
$$

37.7% of the trajectories end above the starting point in 10 rounds, 13.6% in 100, and 0.01% in 1,000; at that point, the expectation is worth 1.55 × 10²¹ and the median, 10⁻²³. The factors are independent and identical; what is not ergodic is the product. Peters calls an observable ergodic when its time average equals the expectation, an operational version of Boltzmann's hypothesis that Birkhoff's theorem (1931) delimited. There is a consequence even for whoever simulates: the mean of N trajectories follows the expectation only for a time that grows as ln N, about 23 rounds with 10 trajectories and 136 with a million.

If the trajectory lives the logarithm, the question becomes how much to expose, and the answer is Kelly's criterion (1956):

$$
f^* = \arg\max_f \; \mathbb{E}[\ln(1 + fR)]
$$

In an even-money bet with probability 0.6, the optimal fraction is 0.2 of the wealth. Betting everything would take the expectation, in 100 rounds, to 8.3 × 10⁷ times the capital, with a chance of surviving of 0.6¹⁰⁰, about 10⁻²². Twice the optimal fraction, 0.4, already impoverishes with the same advantage. Breiman (1961) proved that Kelly's strategy ends up ahead of any other essentially different one, but only in the limit, and the uncertainty about the parameter weighs: with the probability estimated over 25 rounds, full Kelly has a negative growth rate in 15% of the estimates (the exact binomial count), while half Kelly guarantees 75% of the growth with a quarter of the variance. Epistemic uncertainty is an argument for betting less.

The same arithmetic explains contracts that the expectation declares useless. In Bernoulli's ship, taken up by Peters and Adamou, a shipowner with wealth of 100,000 pays 1,800 to cover a loss of 34,000 that occurs with probability 5%. In expectation, he loses 100 per month and the insurer gains 100. In temporal growth, both gain: the shipowner goes from 1.94% to 2.18% per month, and the insurer, which holds many independent trajectories, from zero to +0.007%. Pooling and sharing does the same. In Peters's coin, three players who share the wealth at each round grow 1.6% per round, when each one alone would decay. Cooperation does not change the ensemble mean; it changes how much of it each trajectory receives. The question that decides the case is who holds the trajectories: an insurer or a State have many, a person or a company have one.

**The forecast that changes what it forecasts.** When a model is published, people react to it. A forecast of a long queue makes patients give up; a metric becomes a target and stops measuring (Goodhart's law). The architecture demands running the model with and without the action, to separate what was learned about the world from what was deformed in it. In H3, a forecaster that got what it observed right with 1% error overestimated the real demand by 29%; a small experiment embedded in the operation was needed to undo the loop.

Goodhart has a brother in economics. In 1976, Lucas showed that the parameters of a model estimated under a policy depend on the rules that the agents expect, and for that reason change when the policy changes. The Phillips curve, read as a menu between inflation and unemployment, comes apart when one tries to use it: a curve estimated under one policy promises a lower unemployment than the new regime delivers, because the agents have learnt the new rule. Only the structural relation, between unemployment and unexpected inflation, remains. Simon (1954) and Grunberg and Modigliani (1954) showed that a public forecast can exist that already takes the reaction into account and confirms itself, and Perdomo and colleagues (2020) called performatively stable the model that is optimal for the distribution that it itself induces. In a two-route app, the naive forecaster, which sends everybody to yesterday's fastest route, moves the jam to the route it recommended, and once enough drivers follow it those who use the app take longer than those who do not. The forecaster that divides the users to equalize the times is close to right because it built the world it forecasts: it did not escape the loop, it designed it.

When each success increases the chance of the next, the chance of the beginning becomes destiny, and history gives us only one world. In the Pólya urn, each urn converges, but each one to a different place. Salganik, Dodds, and Watts (2006) tested this with 14,341 participants who downloaded songs in eight parallel worlds: social influence increased the inequality and the unpredictability of success. In a simulated market, unequal merit and cumulative advantage produce the same Gini index (0.63 against 0.64); only the repetition of the worlds separates them, and with cumulative advantage the unpredictability between worlds is eight times greater. Since we cannot repeat history, the arcsine law feeds hindsight bias: in 41% of fair games of a thousand rounds, someone leads for 90% of the time or more, and the leader seems to have deserved it.

Active inference gives the loop between forecast and action an exact form. An agent that minimizes the variational free energy,

$$
F[q] = D\big(q \,\|\, p(s \mid o)\big) - \ln p(o) \;\ge\; -\ln p(o)
$$

can lower surprise in three ways: by changing the belief, by changing the world, or by stopping looking. A thermostat that "believes" in 22 °C in a room at 18 °C ends with the same F when it learns that the room is cold and when it heats it to 22 °C. The number that it minimizes does not distinguish learning from deforming. Only an external counterfactual separates the two cases, and that is why the architecture demands running the model with and without the action.

**Communication as part of science.** A badly communicated result is a result that failed at the layer of the Agent. That is why the output of the system is not a p-value, but natural frequencies ("25 in every 100 patients wait more than half an hour"), visualizations that show possible results instead of means, and an explicit clause that says when the recommendation stops holding.

The rules are simple and rarely followed. "30% chance" does not say 30% of what; "out of every 100 people like you, 30" does. Fixed denominators, absolute risk before relative, and the explicit reference class: a relative increase of 30% over a base rate of 1% is another story. Static error bars lead the reader to reduce the interval to the mean; hypothetical outcome plots, which show simulated outcomes one after another, make the reader feel the variation. The recommendation declares the point at which the action changes (act if p > C/L), and whoever presents the number declares their own interest. Against Goodhart, the plan carries a companion metric, an audit outside the loop, and an expiry date for the target itself.

Part of communication runs in the opposite direction, from the expert to the model. When a parameter does not come from the data, it comes from someone, and asking well is a method. The Sheffield protocol does not ask the expert for a mean. It asks for the median, the quartiles, and the plausible extremes, fits a distribution to those quantiles and returns it for them to confirm, controlling the anchor given by the interviewer, the order of the scenarios, and overconfidence. Before adopting an informative prior, it is worth measuring the calibration of the expert on predictions already resolved. It was this step that the research did not take, and its "from the literature" priors stayed marked as unverified.

The four horizons of the plan have already been traversed, and section 6 shows what passed and what did not. The next steps come out of the caveats, and the background audit has already tested two of them: invariance tests that respect repeated measurements of the same person and the real health data, which entered and were pruned in the severe test for lack of power (section 8). What stays open is the resolution of forecasters of rare events, and not only their calibration; the audit of who pays for the error, which in the two arrests flows showed the same imbalance; and the panel of human experts.

## 8. The background audit: the program against itself

On closing the research, an audit of the book listed ten background points: statements that the text sustained and that
had never been tested. Each point became a work front in the `empirical/` folder of the repository, with the same
house rule of the horizons: the declaration with a hash comes before running, the result is reported with its limits, and
a change of criterion after seeing the result enters as a new declaration and counts in the number of attempts. What
follows is the aggregated result, and it is less comfortable than the previous sections.

**The central thesis, in a blind pilot.** The hypothesis that gives the program its name is that the architecture catches errors that
ordinary statistical practice lets through. It was tested once. Twelve cases were assembled by a planter
from an external list of ten common errors (Makin and Orban de Xivry, 2019): eight with one planted defect each,
four clean. Two independent arms analysed the same twelve cases: one with the architecture, the other with the
instruction to analyse rigorously, without the agents or the gates. Both pointed out 8 of 8 defects and gave 0 false
alarms in the 4 clean cases. The rule declared beforehand said that a tie at the ceiling does not sustain the thesis, and that is what the
result says. It does not refute it either: it shows that, for the ten errors of that list, a competent analyst finds
everything that the other one finds. If the thesis holds, it holds for errors that the list does not cover, and a fair test of that requires
harder cases chosen outside the program, human analysts, and a human judge. The design of the pilot also made the tie
likely: the list is canonical, every role was played by models of the same family, and the list is about errors of the
Model layer, while the advantage the paper claims is at the seams. Section 9 states the second thesis that follows from
that diagnosis.

**Independence.** Four results that sustain the approved exits were reimplemented from scratch, without reading the
code or the original outputs, from the READMEs and the declarations. All of them agreed with the published one within the
tolerances fixed beforehand. STAR and the FDA trial came out identical, down to the number of accepted sets. LaLonde and
Rhode Island agree in the conclusion, but not exactly in the numbers, and the difference comes from choices that no
document fixes: the covariates of the propensity score, which are only in a docstring, and the fact that the published
"+0.096" is the median of a four-cell Bayesian posterior, and not the weighting estimate; in Rhode
Island, the row filter and the parameters of the circuit breaker. No error was found in the original. The limit is what
matters: the "independent team" was a model of the same family, and three choices that the specification leaves
open came out equal to those of the original without being written down. An agreement like that is weak evidence; a divergence
would have been strong.

**Multiplicity and adjustments after seeing the result.** The transfer gate of H2 was attempted 38 times, if one counts the distinct
tests (32 to 72, depending on what is included). With Bonferroni over 38, H2g still has Ŝ ≠ ∅ in
four areas, at the cost of 9.6 bits of Occam, and the invariant sets shrink: in STAR, {small years before,
small class} becomes {small class}; in yeast, {plate, x1, x5, x9} becomes {plate, x5}, and the only gene that validation by
intervention confirmed drops out. Two caveats weigh more than the correction. The first is that dividing α is not
monotone in the method: with a smaller α, domains that rejected everything start to accept some set, and Ŝ appears in
models almost rejected. The second is that the approval depends on framework v2 of H2: with the previous version
reconstructed, STAR is pruned before the invariance test and H2g falls to three areas, one of them the fragile
genetics. And in H3, of the four versions of the loop estimator, only the fourth passes the criterion (biases of +4.7%, −5.9%,
−8.4%, and +1.7%, for a limit of 3%). If only the first declared version of each choice counted, neither H2 nor
H3 would be approved. The inventory of the path also counts eleven vault openings, that is, eleven times in which a
result was seen before a design decision.

**The stopping rule, closed.** The repair that section 9 names, an e-process whose running evidence obeys Ville's inequality at any stopping time, was applied on October 19 to the 38 recorded tests under a declaration written before the computation (`empirical/p22-parada/DECLARACAO_2.md`; the instrument is `labs --eprocess`). The rule: one ledger per area; each test contributes the mixture calibrator of its per-domain p-value when it published a non-empty set, and the calibrator at p = 1, one half, when it returned the empty set, rejected every set, or accepted a set only by saturation; areas merge their rounds by the mean, because the rounds reuse data; the gate passes with three areas at or above 1/α = 20. As recorded, three areas cross: Education (mean e-value 4,414), Pharmacology and Physics (both far above the threshold, from a single test each); Genetics stays at 7.0, under the 20 required; Law at 2.4 and Transport at 3.0. With the clustered rows of the declaration admitted, Education's published set is replaced by its clustered p-value of 0.76, and its verdict does not move, because the mean also carries the two earlier rounds on the same data, which the declaration left as recorded. The e-process answers the question the Bonferroni could not: whether the path of rounds was allowed to stop where it stopped. It says yes for three areas and not for Genetics, and it does not repair the rows of a test, so the clustering correction of the previous paragraphs stands beside it. The declaration was sealed before the instrument ran; nothing in the rule was tuned after the reading, and the Education row that stays in by the mean is reported, not repaired.

**The performative loop.** H3 supposed that whoever gives up on the queue calls again with probability 0.6. The same parameter
was measured in a real call center within reach of the data (AnonymousBank, 1999), with a quasi-experimental design:
the queue found on entering, within the same date and hour, serves as an instrument for abandonment. Under that design,
giving up instead of being served increases by 0.145 (90% CI from 0.132 to 0.157) the chance that the same client calls again
within fifteen minutes. The sign matches and the order of magnitude does not: the declared loop is about four times the
observed one, outside the range of one third to three that was declared. The excess of arrivals that the measured loop produces
in that call center is 2.3% of the arrivals, against the 29% bias that the declared loop generated in H3. That is: the
performative correction of H3 was tested against a loop far stronger than that of a real case. The measured value is probably
a lower bound, because a new call is only visible when it enters the queue, and the call center is from another bank, from
another country, and from another year.

**Calibration: the tolerance is not the noise.** H3 failed a vault when the calibration error passed 0.03.
With a tolerance that depends on n — the observed ECE against the 95% quantile of the ECE under perfect calibration, with the
same predictions — no binary vault passes, not even those that were approved: in the cyclones, in the 2023 flights, and in
Rhode Island, the observed ECE is from 1.4 to 2.3 times the null value, with a p-value of 0.0005 or less. The noise in those
vaults is from 0.002 to 0.006, and 0.03 is a practical tolerance that lets detectable miscalibration through. By the secondary
rule, max(0.03; 95% quantile), the picture of H3 does not change: the three approved ones stay approved and the two from
Toronto stay failed — the failure of Toronto 1997–2002, by the way, was not noise (ECE 0.036 against 0.032,
p-value 0.008). The two label-drift tests tested would not have warned before the original sensor in the 2013 vault, and the one that compares the realized rate
with the predicted mean stays on in 95% of the verifications of that year: with a three-day window, it measures the time of one bad day, not
the drift of the quarter. The correction of the flights withstood the only real deviation that appeared in two new years of data, but there was no proof in the vault, and the declared stress did not
discriminate because the daily readjustment absorbs the drift by itself.

**ICP depends on whose the unit is.** The invariance test supposes independent rows; in the FDA trial there are 16
instants per volunteer and in STAR there are 80 schools. Redoing the test with errors clustered by volunteer and by school, and
with permutation of whole blocks, the two areas separate. Pharmacology survives intact: the set
{c_dof, c_qui, c_ran} — the concentrations of the three drugs that block hERG — comes out in all the tests, although the
evidence against the empty set falls from about 340 bits to 49, and to 9 in the block permutation. Education does not survive: with 80 schools as
units that are in fact independent, the sets that the test cannot reject have no variable in common, and the
invariant set becomes empty in both corrections. Physics enters as it was. Genetics, which was the fragile area, was
replicated in a new draw of 40 genes: in ten of them the test returned an invariant set with a gene, and one pair passed
validation by intervention — in the mutant of the deleted gene, the expression of the target stays below all the other 1,740 samples. The
areas are then three, physics, pharmacology, and genetics, and the transfer gate of H2 stands: not because of the
area that fell, but because of a new domain in the place of the only one that was already fragile.

**Scale and data.** Two new health domains entered the standardized package, with the funnel unchanged: a randomized
trial of indomethacin against post-ERCP pancreatitis, 602 patients, and a country-month panel of vaccination and
covid deaths. Both were pruned in the permutation test (p = 0.060 and p = 0.28). In the trial, the signal is on the
expected side and the funnel simply has no power after reserving an entire centre as a vault. Scale was
tested with the funnel running in batch over 329 domains generated by rule from a public catalogue: 327
reached the end, at a cost of 3.8 seconds of processor per domain. The hypergraph jumped from 12,797 to 26,634
nodes, but only 6,494 are concept nodes; 20,140 are hypotheses of the invariance test, one per subset tried,
which grow as 2 to the number of variables and are not new knowledge. And of the 6,494, only 554 come from
curated packages. The bottleneck is not the machine: it is semantic curation, which does not scale by rule, the supply of
sets with identifiable environments (9% of the catalogue passes), the bridges of the graph, which grow with the square of the
number of domains, and keeping everything in memory. The count of "1,538 nodes" that the audit used was out of date, and
that shows the risk of the pretty number.

**The priors.** The informative priors of the program never went through a human expert: they were read from
the literature and converted into distributions by fixed rules. The front that measures whether the conclusions depended on them exchanged,
in 33 domains, the prior of the package for the one from the literature and for a deliberately weak one. Five domains change status
or action with the one from the literature, and two with the weak one; none of the changes is cosmetic. One failure becomes pruning in
the first filter, another becomes approval — the heavy-tailed project that H1 failed passes when the simulated panel
becomes a Beta(1,1) with almost no information. Two of the five come from a source built on the domain's own database, the
case of double counting that the declaration foresaw marking. The large shift, however, does not decide on its own: two domains
move more than one standard deviation of the posterior and do not cross the threshold of F8. The transfer gate of H2 keeps
passing, now with exactly three areas. And the control round, which should reproduce the published numbers, did not
change category, gate, or action in any of the 33 domains: it stayed under seven thousandths of a standard deviation in each
posterior, which is noise of the numerical environment. The front closed that point with a second declaration, written before the
recomputation with its own hash: the deviation is now compared in units of the posterior standard deviation of the round itself,
against an equivalence band of 0.05 sd, fixed between the worst noise already observed and the smallest shift
that has already changed a decision, 0.18 sd. The maximum deviation is 0.0071 sd, in `h2g/genetica_y07`, seven times inside the
band, and 33 of the 33 domains match field by field against the outputs of the repository: status category, list of
gates that fail, and action of F8. The conditional label is over. The old declaration was not rewritten and still holds,
so much so that without the band the same test would fail 25 of the 33 for equality of bits — which no floating-point
reimplementation satisfies; that is why the new reading counts as K = 2, and not as a repair of the old criterion.

**Rare events, health, and what depends on people.** Three fronts entered after the ten: two of machine, which
closed, and one of people, which stayed prepared. The first measured where the event is rare, and not only the calibration: by the
declared rule, resolution above reliability with the CI90 of (RES − REL) entirely above zero and Brier skill in the vault
of at least +0.05, **no path wins in the three cases**. In Rhode Island the first condition
passes — 553 arrests in 24,466 stops, RES − REL of +2.74643e-4, with CI90 from +1.708e-4 to +3.793e-4 in the logistic, and
+3.11655e-4, from +2.057e-4 to +4.067e-4, in the ranking score — and the second fails: the skill stays at +1.9%,
about 40 times below the +5% required. In the indomethacin trial the CI90 of (RES − REL) crosses zero (−0.0082881, from
−0.02520 to +0.001452) and in the covid panel it lies entirely below zero (−0.0181081, from −0.03339 to −0.006549),
with AUC of 0.714 and 0.768: the ordering exists, the level is what spoils it, and in covid the forecast loses to the constant
of the training. The lesson is that, in a rare event, "does it have resolution?" and "is it useful?" are different questions. The second extension
took the circle of Lab 0 to open data: the sepsis alert domain in the public mirror of the PhysioNet/CinC Challenge
2019, one row per admission and the second hospital system as the vault, passed F1, F2, and F3 — z of 4.981
against q95 of 1.970, p = 0.004975 — and fell at F4, with **0 of 512 subsets accepted** and Ŝ = ∅. The effect is precise
(β = 0.0388 per breath per minute, CI90 from 0.0257 to 0.0523); the regularity between hospitals is what does not exist, the
vault decides half the times of the training (3.9% against 8.4%) and the calibration gate of p08 fails the three arms. The
claim of the domain does not hold, and G3 fails: the exit of H2 does not end up approved by that criterion. The third
extension is not of machine: the protocol of the elicitation panel (SHELF) is frozen in `empirical/p12-shelf/DECLARACAO.md`, with the pre-test of 20 resolved
predictions, the six forms, the starting priors, and the criterion of K = 2; on 2026-10-16 the first internal expert (`E1`) closed five
domains ($n = 1$, skills from 0.4240 to 0.9730), with the other two people per domain and the expert from outside the program missing; at the other end, the 21 rows of the rereading of the opinions in `governance/RELEITURA.md` are with file, SHA-256, date, and decision confirmed by the owner (21 of 21 on 2026-10-16).

**What the audit changes in the rest of the text.** No number published in sections 5 to 7 fell because of an arithmetic error: the
independent reimplementation found none. What the audit narrows is what those numbers authorize. The loop of 0.6
that H3 corrected is four times what was measured in a real case. The tolerance of 0.03 of the calibration error is greater
than the noise in all the large vaults, and none of them passes the rule that depends on n. The informative priors also
are not neutral: five of the 33 domains change decision with the one from the literature, among them the tail project that H1
failed. The transfer gate of H2 depends on which version of each choice counts, and the education area does not
survive the clustering correction — what sustains the third area now is the replicated genetics, not education, and
it gained the p-value that was missing: the number that sustains it, 1 gene in 27 in the validation by intervention, was subjected
to a permutation of the mutant label, 999 draws, and gave p = 0.001, the floor of the test (and, in the fine resolution of two rows in `labs --genetica`, 1 of 40 genes passes with a floor of $10^{-6}$). The declared limit stays by its
side, because with each mutant being worth only one row the smallest p possible in this design is 1/1,000, and the half of the
criterion that is a fact of the data, the deletion that worked, is precisely the one that the permutation destroys. And the hypothesis that
gives the program its name still lacks a demonstration, because in the only direct test it had the two arms tied
at the ceiling. A survey of the references completed the picture: the works previously cited from memory were
checked one by one in an authoritative source, with DOI, and three citations changed form — the article by Meyer, Viscusi, and
Durbin, which the text attributed to two authors; the one by Baltagi and Levin, which came out in *Structural Change and Economic
Dynamics*, and not in *Empirical Economics*; and the work of Hume from 1748, whose title only became *Enquiry* in 1758.

## 9. Limits and objections

A discipline that demands a severe test of other people's claims must apply it to itself. Nine objections deserve a direct answer.

**"This is just statistics with another name."** In part, yes: almost all the tools have had an owner for decades, and the closest neighbours are not statistics in general but specific programs: Box's loop of criticism and estimation, Mayo's severe testing, the Bayesian workflow of Gelman and colleagues (2020), which already joins model building, predictive checks, and model criticism in a declared sequence, and the model and data cards of machine learning. Against the Bayesian workflow in particular, the layers, the cards, and the gates add little at the layer of the Model. What none of those neighbours makes central is the layer of the Agent: who decides, under which question resolution, and who pays for the error. That is where Stochology can be distinct, and it is also the part least executed: the seven groups affected by the decisions that involve people have not been heard. What Stochology adds is the organization: the separation into layers, the typing of uncertainty, and the requirement that each claim pass through the same gates. The hypothesis that the program needs to demonstrate, and has not demonstrated yet, is that this organization catches errors that ordinary statistical practice lets through. Its first direct test, the blind pilot of section 8, tied at the ceiling: the two arms pointed out the same eight planted defects and gave the same zero false alarms.

The course documentation gives the shortest answer when it compares the field with its neighbours: Stochology does not seek new theorems, it uses them to judge arguments and presuppositions, and it asks what a model authorizes one to conclude, not how to optimize it.

The neighbours deserve names, because the pieces of the machine have owners and a discipline that did not name them would be claiming what it only borrowed. The card, with its fixed fields, is a cousin of the model card (Mitchell and colleagues, 2019) and of the datasheet for datasets (Gebru and colleagues, 2021): both force a model or a dataset to declare its intended use, its provenance and where it fails. The card adds what those two leave out, the generating process, the independent unit, the null model and who pays for the error, and it is refused by code, not by a reviewer. The seal is pre-registration (Nosek and colleagues, 2018), with a hash in place of a registry and the counter K in place of the honour system; the registered report, which commits a journal before the data are seen, is the institutional form of the same idea, and this program has no journal. The forecast diary and the proper score out of sample are the discipline of the forecasting tournament (Tetlock and colleagues, 2014), where calibration and resolution are charged on the forecaster and not on the forecast. The natural frequencies, the fixed denominator and the absolute risk before the relative are risk communication as Gigerenzer and Spiegelhalter (2017) teach it. The predictive check, the simulation-based calibration and the habit of fitting a rival model are the Bayesian workflow (Gelman and colleagues, 2020), and the refusal to choose between belief and frequency, charging the one by the other, is close to what Gelman and Hennig (2017) call going beyond the subjective and the objective. The open research culture of the replication movement (Nosek and colleagues, 2015) is the ground all of this stands on. What is not borrowed is the joint: the same gates for every claim, the Agent layer with its question resolution and its payer, and the count of attempts kept against the program itself.

The pilot also suggests what a fairer form of the hypothesis would be. If the advantage lives at the seams, the architecture should beat ordinary analysis on seam errors (question resolution, non-ergodic dynamics, performative loops, selection inside a decision, tails without a usable mean, and the choice of who pays) and tie on classical errors. That is thesis (c′), declared in `empirical/p16-ponte/DECLARACAO_3.md`: it predicts an interaction between arm and stratum, which can fail in more ways than "more" can, and it adds the axis the pilot never measured, the agreement of an arm with itself over repeated runs. Thesis (c) keeps its rule and its window.

**"The real data are old and from public databases."** It is true, and it was a choice: calls from 2003, jobs from 1978, fires from 1980 to 1990. They are known databases, verifiable by hash and with an outcome already known, which allows auditing the method, but does not replace a real operation. Several pieces remain assumed: the service time and the patience of the call center, which the database does not bring and whose mesh of 4 × 6 points (section 6) shows that no point switches the winning rule, with the worst case — 5 min of service and 10 min of patience — leaving 14.2% of the days below the target against 37.7% for the rule of the claim; the decision costs, which are a declared assumption; and the human elicitation panel, with five domains elicited by the first internal expert ($n = 1$) and the other two people per domain and the expert from outside still missing, which is what is missing for the informative priors to stop being sensitivity with a single person. What the measurement has already covered leaves the list: the performative loop of H3, which the real call center puts at about 0.15 against the declared 0.6 (section 8), and the sensitivity of the priors, which closed with a firm verdict. What the audit could not look at stays recorded: none of the seven groups affected by the decisions that involve people was heard, and there is no record in the repository of consultation, interview, panel of the affected groups, or consent. And the last two gates only passed after several rounds, with declared changes along the way. The architecture records all of this as a declared pending item, which is exactly what it demands of any claim.

**"The layers cannot fail."** Claim (a) is the name of the program, and as stated it absorbs every case: any error can be told as a wrong bridge. The answer is a test that lets the frame lose. `empirical/p21-camadas/DECLARACAO.md` freezes it: at least sixty documented errors from outside the program, three coders of whom two from outside, seven codes that include pure arithmetic and outside the frame, Fleiss' κ, typing at κ ≥ 0.60 and vocabulary below 0.40. If nobody ever reaches for the arithmetic code, the sentence "most errors are not arithmetic" is not refuted, but it is not tested either, and the paper will say so.

**"The gates approved what the audit rejected."** H2 and H3 passed their gates, and the background audit showed afterwards that neither would have passed if only the first declared version of each choice counted. The instrument that caught the errors was the retrospective count of attempts, not the gate at the moment of passage. The diagnosis is precise: a hash on every round makes each step honest, but "run another round until it passes" is optional stopping, and the stopping rule was never declared; a Bonferroni chosen after the path is known divides α by a number the path chose. The repair is a rule that holds at any stopping time. Each round contributes an e-value, a statistic with expectation at most 1 under the null, and the running product of conditionally valid e-values obeys Ville's inequality,

$$
P_{H_0}\Big(\sup_n E_n \ge 1/\alpha\Big)\le \alpha ,
$$

which is Royall's bound of section 3.1 with the likelihood ratio replaced by any e-value (Vovk and Wang, 2021; Ramdas and colleagues, 2023). From 2026-10-18 on, gates attempted in rounds keep that ledger (`empirical/p22-parada/DECLARACAO.md`). Its application to the 38 past tests of H2 was declared and run on 2026-10-19 (section 8): three areas cross at any stopping time, Genetics does not, and the Education row that the clustering correction removes stays in by the mean of its earlier rounds. The retrospective answer is therefore the same as the gate's, with the stopping rule now written down, and the clustering caveat untouched.

**"Formalizing judgment is eliminating judgment."** The architecture does not decide what is worth asking, nor how much an error costs for whoever suffers it. Those values enter through elicitation and through the cost/loss ratio, which are declared, not computed. The ambition is to make judgment visible and countable, not to replace it; that is why the definition says "science (or art)".

**"The apparatus is too heavy."** Eight agents, five lenses, and five invariants are a lot for a simple question. The plan recognizes the risk and answers with the funnel: most of the hypotheses should be discarded cheaply, before any expensive simulation. If the cost of using the machine is greater than the value of the decisions that it improves, Crisonomy itself would order the machine to be switched off.

**"Bayes solves Hume's problem."** It does not, and the discipline must not sell that. Bayesian updating is the coherent rule inside a hypothesis space, but the problem of induction moves to the choice of the prior and of the space, as Laplace's rule of succession showed in section 2. Cox's theorem itself depends on technical hypotheses, as Halpern's (1999) counterexample shows. And not all information is welcome. Good (1967) proved that free information never makes the decision of whoever has preferences over states of the world worse, but whoever has preferences over their own observations may pay not to know: among people at risk of Huntington's disease, 17.4% took the genetic test before symptoms in the United Kingdom between 1994 and 2014 (Baig and colleagues, 2016).

**"Ergodicity became doctrine."** Chrononomy relies on the program of Ole Peters, which is contested, and the discipline teaches it as a debate. Peters proposed that loss aversion may not be a bias, but a rational answer to a multiplicative dynamics. The numbers show on what scale that holds. Tversky and Kahneman estimated that losses weigh about 2.25 times the gains, and, by the criterion of growth, a ratio of that order is only justified when 56% of the wealth is at stake. The thesis explains the refusal to risk half a patrimony, but not the aversion measured in the laboratory with small bets, in which any concave and smooth utility is almost neutral; Rabin's calibration theorem holds for the logarithm as well. In an experiment in Copenhagen (Meder and colleagues, 2021), the risk aversion of the participants increased when the game went from additive to multiplicative; Doctor, Wakker, and Wang (2020) answer that expected utility never supposed ergodicity. The debate becomes more precise, without a winner. Something similar holds for the tails: Taleb asks that a heavy tail be supposed until proof to the contrary, and Clauset, Shalizi, and Newman (2009) ask for proof before declaring a power law. The question is whose is the burden of proof, and the answer depends on how much it costs to be wrong.

**"The auditor is an interested party."** The 21 risk opinions were signed by whoever runs the program. An audit authorized by the one being audited has limited value, and the scope, in that case, is a concession of the audited party. The architecture answers in part with the named human responsible, because the question of an audit is not technical: who decided to use it, who approved it, who watches the scoreboard, who switches it off. But naming a responsible person does not make the audit independent. The record that was missing has been made: in the seven cases that involve people, `governance/GRUPOS.md` declares that no affected group was heard (and `labs --escuta` keeps the box sealed with 0 of 7), and the rereading of the 21 opinions in full, with date and decision confirmed in the 21 rows on 2026-10-16, is recorded in `governance/RELEITURA.md`.

## 10. Conclusion

Stochology proposes a change of question. Instead of "what is the probability?", it asks: in which layer can this statement fail, what type of uncertainty threatens it, and what test would it have to face to deserve trust? The old frontiers (between philosophy, statistics, and dynamics; between risk and uncertainty; between norm and behaviour) do not disappear. They become seams, and it is in the seams that the discipline works.

The real data confirmed that lesson and added another, less comfortable one. The mean hid 26 bad days in every 100 in a bank call center, and the typical trajectory of a leveraged investor stayed far below the average of all. But the program needed seven rounds to pass one of its own gates and three attempts to pass another, and an imprudent reinsurer was ruined without the rival model winning the right to decide. A discipline about uncertainty is also measured by what it admits not to have achieved.

The simulated health clinic summarizes the lesson in one sentence: the same mean can hide 9 or 19 minutes of wait (21 in the old generator), and only whoever simulates the world instead of computing the average case discovers the difference. Bernoulli called this the art of conjecturing. Three centuries later, with reproducible code, null models, and agents that do not approve their own work, that art can also become an auditable science, as long as it stays humble about what it does not yet know.

What the repository demonstrates today is narrower than the name of the program and, for that reason, firmer. It does not yet show that the layers are a typing, nor that the architecture catches what competent practice misses. It shows that pre-declaration, the count of attempts, and negative proofs can turn a program against itself and make it retract, in the open, what its own gates had approved. That procedural claim is the one the evidence supports; the others stay declared, with the tests that could make them fail.

## References and readings

**Project documents**

- [Architecture](https://docs.google.com/document/d/1l0YQHrV_UrKisAjQcz2BqT9KpwhcT0s0WVoXm_muxtE/edit): typed hypergraph, agents F1 to F8, six-stage cycle.
- [docs/PLAN.md](https://docs.google.com/document/d/1hXvDuJyckooL1_qx9gspkSkBqgjyhAUVQvQ3F-Crinc/edit): horizons H0 to H3 and research funnel.
- [AGENTS.md](https://docs.google.com/document/d/1hniAsjLqgNVGqR1wZ1845GVkS5lZTEjhlH7MaI3U2Xk/edit): contract of the agents and invariants Φ1 to Φ5.
- [Stochology: Foundation, Three Layers, and Inference under Uncertainty](https://docs.google.com/document/d/1L5ittj7-RiSacEvNrkR4IS6yrQUyhusFpcDIWazINLI/edit): layers, taxonomy of uncertainty, modules M0 to M5.
- Results: project repository, 0b3st4f3r4/estocologia (h0-lab0, h0-real, h1-real, h2 to h2g, and h3, each with README.md and outputs); real databases with SHA-256 in [data/SOURCES.md](../data/SOURCES.md).
- The book: `book/` in the repository, twenty-one chapters, appendices with the provenance of every number (Appendix E) and the answer key of the 126 exercises (`book/answers.md`); the instruments, `cargo run --release --bin labs -- --chapter N`, `--streams` and `--eprocess`.
- Course *Critical Thinking and Stochastic Models*: public repository 0b3st4f3r4/pensamento-critico-modelos-estocasticos, with the M0 to M16 scripts, the ME01 to ME25 special modules, the MB01 to MB08 panels, the laboratories, and documentacao.md. The numbers attributed to the course or to course simulations come from those scripts and laboratories.

**Databases and methods used in the research** (verified in an authoritative source; see [references/VERIFICATION.md](../references/VERIFICATION.md))

- Shen, H.; Huang, J. Z. Forecasting time series of inhomogeneous Poisson processes with application to call center workforce management. *Annals of Applied Statistics*, 2(2):601–623, 2008.
- LaLonde, R. J. Evaluating the econometric evaluations of training programs with experimental data. *American Economic Review*, 76(4):604–620, 1986.
- McNeil, A. J. Estimating the tails of loss severity distributions using extreme value theory. *ASTIN Bulletin*, 27(1):117–137, 1997.
- Meinshausen, N.; Hauser, A.; Mooij, J. M.; Peters, J.; Versteeg, P.; Bühlmann, P. Methods for causal inference from gene perturbation experiments and validation. *PNAS*, 113(27):7361–7368, 2016. DOI: 10.1073/pnas.1510493113.
- Peters, J.; Bühlmann, P.; Meinshausen, N. Causal inference by using invariant prediction: identification and confidence intervals. *JRSS B*, 78(5):947–1012, 2016.
- Heinze-Deml, C.; Peters, J.; Meinshausen, N. Invariant causal prediction for nonlinear models. *Journal of Causal Inference*, 6(2), 2018.
- Meyer, B. D.; Viscusi, W. K.; Durbin, D. L. Workers' compensation and injury duration: evidence from a natural experiment. *American Economic Review*, 85(3):322–340, 1995.
- Gamella, J. L.; Peters, J.; Bühlmann, P. Causal chambers as a real-world physical testbed for AI methodology. *Nature Machine Intelligence*, 7:107–118, 2025.
- Kemmeren, P. et al. Large-scale genetic perturbations reveal regulatory networks and an abundance of gene-specific repressors. *Cell*, 157(3):740–752, 2014.
- Johannesen, L. et al. Differentiating drug-induced multichannel block on the electrocardiogram: randomized study of dofetilide, quinidine, ranolazine, and verapamil. *Clinical Pharmacology & Therapeutics*, 96(5):549–558, 2014.
- Gibbs, I.; Candès, E. Adaptive conformal inference under distribution shift. *NeurIPS 34*, 2021.
- PhysioNet/Computing in Cardiology Challenge 2019 (early sepsis), Reynolds et al. Early prediction of sepsis from clinical data. *Critical Care Medicine*, 2020. DOI: 10.1097/CCM.0000000000004145 (data in the open PhysioNet mirror, ODC-By licence).

**Classical works cited** (verified in an authoritative source the seven that the verification records — Bernoulli, Knight, Savage, Tversky and Kahneman, Mayo, Pearl, Gigerenzer; the others follow as the sources give them; see [references/VERIFICATION.md](../references/VERIFICATION.md))

- Bernoulli, J. *Ars Conjectandi*, 1713.
- Hume, D. *Philosophical Essays Concerning Human Understanding*. London: A. Millar, 1748 (the title *An Enquiry* only appears in the 1758 edition).
- Bayes, T. "An Essay towards Solving a Problem in the Doctrine of Chances", 1763.
- Knight, F. *Risk, Uncertainty and Profit*, 1921.
- Kolmogorov, A. *Grundbegriffe der Wahrscheinlichkeitsrechnung*, 1933.
- Popper, K. *Logik der Forschung*, 1934.
- Savage, L. J. *The Foundations of Statistics*, 1954.
- Simon, H. "A Behavioral Model of Rational Choice", 1955.
- Tversky, A.; Kahneman, D. "Judgment under Uncertainty: Heuristics and Biases", *Science*, 1974.
- Mayo, D. *Error and the Growth of Experimental Knowledge*, 1996.
- Pearl, J. *Causality*, 2000.
- Gigerenzer, G. *Calculated Risks*, 2002.
- Meng, X.-L. "Statistical Paradises and Paradoxes in Big Data", 2018.


**Works cited from the course modules** (references as the modules give them; those that came later are verified, see [references/VERIFICATION.md](../references/VERIFICATION.md))

- Baig, S. S. et al. "22 years of predictive testing for Huntington's disease: the experience of the UK Huntington's Prediction Consortium", *European Journal of Human Genetics* 24(10):1396–1402, 2016. DOI: 10.1038/ejhg.2016.36.
- Berkson, J. "Limitations of the application of fourfold table analysis to hospital data", *Biometrics Bulletin* 2(3), 1946.
- Birkhoff, G. D. "Proof of the ergodic theorem", *PNAS* 17(12), 1931.
- Breiman, L. "Optimal gambling systems for favorable games", *4th Berkeley Symposium*, 1961.
- Charig, C. R. et al. Comparison of treatments for kidney stones, *BMJ* 292, 1986.
- Chouldechova, A. "Fair prediction with disparate impact", *Big Data*, 2017.
- Clauset, A.; Shalizi, C. R.; Newman, M. E. J. "Power-law distributions in empirical data", 2009.
- Cox, R. T. "Probability, frequency and reasonable expectation", *American Journal of Physics* 14, 1946.
- Diaconis, P.; Holmes, S.; Montgomery, R. "Dynamical bias in the coin toss", *SIAM Review* 49(2):211–235, 2007. DOI: 10.1137/S0036144504446436.
- Doctor, J. N.; Wakker, P. P.; Wang, T. V. "Economists' views on the ergodicity problem", *Nature Physics* 16, 2020.
- Dwork, C.; Roth, A. *The Algorithmic Foundations of Differential Privacy*, 2014.
- Erev, I.; Wallsten, T.; Budescu, D. *Psychological Review* 101(3), 1994.
- Gebru, T.; Morgenstern, J.; Vecchione, B. et al. "Datasheets for datasets", *Communications of the ACM* 64(12):86–92, 2021. DOI: 10.1145/3458723.
- Gelman, A.; Hennig, C. "Beyond Subjective and Objective in Statistics", *JRSS A* 180(4):967–1033, 2017. DOI: 10.1111/rssa.12276.
- Gelman, A.; Vehtari, A.; Simpson, D.; Margossian, C. C.; Carpenter, B.; Yao, Y.; Kennedy, L.; Gabry, J.; Bürkner, P.-C.; Modrák, M. "Bayesian workflow", arXiv:2011.01808, 2020.
- Gigerenzer, G. et al. "Helping doctors and patients make sense of health statistics", *Psychological Science in the Public Interest* 8, 2007.
- Gilovich, T.; Vallone, R.; Tversky, A. *Cognitive Psychology* 17(3), 1985.
- Good, I. J. "On the principle of total evidence", *BJPS* 17(4), 1967.
- Halpern, J. Y. *Journal of Artificial Intelligence Research* 10, 1999.
- Hullman, J.; Resnick, P.; Adar, E. "Hypothetical outcome plots outperform error bars", *PLOS ONE* 10(11), 2015.
- Ioannidis, J. P. A. "Why most published research findings are false", *PLoS Medicine* 2(8), 2005.
- Jaynes, E. T. "Information theory and statistical mechanics", *Physical Review* 106, 1957.
- Kelly, J. L. "A new interpretation of information rate", *Bell System Technical Journal* 35(4), 1956.
- Kramers, H. A. *Physica* 7(4), 1940.
- Lucas, R. E. "Econometric policy evaluation: a critique", *Carnegie-Rochester Conference Series on Public Policy* 1, 1976.
- Makin, T. R.; Orban de Xivry, J.-J. "Ten common statistical mistakes to watch out for when writing or reviewing a manuscript", *eLife* 8:e48175, 2019. DOI: 10.7554/eLife.48175.
- Meder, D. et al. "Ergodicity-breaking reveals time optimal decision making in humans", *PLoS Computational Biology* 17(9), 2021.
- Meng, X.-L. "Statistical paradises and paradoxes in big data (I)", *Annals of Applied Statistics* 12(2), 2018.
- Miller, J. B.; Sanjurjo, A. *Econometrica* 86(6), 2018.
- Mitchell, M.; Wu, S.; Zaldivar, A. et al. "Model Cards for Model Reporting", *FAT\* '19*, pp. 220–229, 2019. DOI: 10.1145/3287560.3287596.
- Murphy, A. H. "A new vector partition of the probability score", *Journal of Applied Meteorology* 12, 1973.
- Nosek, B. A.; Alter, G.; Banks, G. C. et al. "Promoting an open research culture", *Science* 348(6242):1422–1425, 2015. DOI: 10.1126/science.aab2374.
- Nosek, B. A.; Ebersole, C. R.; DeHaven, A. C.; Mellor, D. T. "The preregistration revolution", *PNAS* 115(11):2600–2606, 2018. DOI: 10.1073/pnas.1708274114.
- O'Hagan, A.; Buck, C. E.; Daneshkhah, A.; Eiser, J. R.; Garthwaite, P. H.; Jenkinson, D. J.; Oakley, J. E.; Rakow, T. *Uncertain Judgements: Eliciting Experts' Probabilities*, Wiley, 2006. DOI: 10.1002/0470033312 (the SHELF protocol, by Oakley and O'Hagan, is a separate instrument, not a synonym of the book).
- Open Science Collaboration. "Estimating the reproducibility of psychological science", *Science* 349, 2015.
- Pearl, J.; Glymour, M.; Jewell, N. *Causal Inference in Statistics: A Primer*, Wiley, 2016.
- Peirce, C. S. "Deduction, induction, and hypothesis", *Popular Science Monthly* 13, 1878.
- Perdomo, J.; Zrnic, T.; Mendler-Dünner, C.; Hardt, M. "Performative prediction", 2020.
- Peters, O. "The ergodicity problem in economics", *Nature Physics* 15, 2019.
- Peters, O.; Adamou, A. "Insurance makes wealth grow faster", arXiv:1507.04655, 2015.
- Rabin, M. "Risk aversion and expected-utility theory: a calibration theorem", *Econometrica* 68(5), 2000.
- Ramdas, A.; Grünwald, P.; Vovk, V.; Shafer, G. "Game-theoretic statistics and safe anytime-valid inference", *Statistical Science* 38(4), 2023. DOI: 10.1214/23-STS894.
- Rossouw, J. E. et al. (Women's Health Initiative), *JAMA* 288(3), 2002.
- Royall, R. *Statistical Evidence: A Likelihood Paradigm*, 1997.
- Rubin, D. B. *Journal of Educational Psychology* 66(5), 1974.
- Salganik, M.; Dodds, P.; Watts, D. *Science* 311, 2006.
- Simmons, J.; Nelson, L.; Simonsohn, U. "False-positive psychology", *Psychological Science* 22(11), 2011.
- Spiegelhalter, D. "Risk and Uncertainty Communication", *Annual Review of Statistics and Its Application* 4:31–60, 2017. DOI: 10.1146/annurev-statistics-010814-020148.
- Stampfer, M. J. et al. (Nurses' Health Study), *NEJM* 325(11), 1991.
- Talts, S. et al. "Validating Bayesian inference algorithms with simulation-based calibration", arXiv:1804.06788.
- Taleb, N. N. "How much data do you need?", *International Journal of Forecasting* 35(2), 2019.
- Tetlock, P. E.; Mellers, B. A.; Rohrbaugh, N.; Chen, E. "Forecasting Tournaments", *Current Directions in Psychological Science* 23(4):290–295, 2014. DOI: 10.1177/0963721414534257.
- Tversky, A.; Kahneman, D. *Journal of Risk and Uncertainty* 5(4), 1992.
- Vapnik, V.; Chervonenkis, A. *Theory of Probability and Its Applications* 16(2), 1971.
- Vovk, V.; Wang, R. "E-values: calibration, combination and applications", *Annals of Statistics* 49(3), 2021. DOI: 10.1214/20-AOS2020.
