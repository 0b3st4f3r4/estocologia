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
| **$\mathrm{do}(x)$ (intervention)** | Fixing a variable by decision, and not merely observing who already has it | Drawing lots for who receives the medicine, instead of comparing those who were already taking it with those who were not |
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
| $\mathrm{Var}(X)$ | "The mean dispersion of $X$ around its mean, squared" | `let var = x.iter().map(\|v\| (v - mean).powi(2)).sum::<f64>() / x.len() as f64;` |
| $\sigma$ or $\mathrm{dp}$ | "Standard deviation: the square root of the variance, in the same unit as $X$" | `let dp = var.sqrt();` |
| $P(A \mid B)$ | "Probability of $A$ **given that** $B$ happened" | `if b { p_a_given_b } else { p_a_given_not_b }` |
| $\mathrm{do}(x)$ | "Fixing $x$ by decision, and not merely observing who already has $x$" | `let y = f(x_fixed);` — instead of `let y = f(x_observed);` |
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
| $\mathrm{Brier} = \frac{1}{n}\sum (p_i - y_i)^2$ | The mean of the squared error of a probabilistic forecast | `p.iter().zip(y).map(\|(a,b)\| (a-b).powi(2)).sum::<f64>() / n as f64` |
| $\text{skill} = 1 - \mathrm{Brier}/\mathrm{Brier}_{\text{clim}}$ | How much the model gains over the climatologist | `1.0 - brier / brier_ref` |
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
| `cargo run --release --bin labs -- --seed-spread` | Thirteen laboratories rerun under twenty other seeds, line by line: which numbers move, and whether a verdict moves with them (C.3; minutes long) |
| `cargo run --release --bin labs -- --discrepancy` | A physical parameter learnt with and without a model of the discrepancy (Chapter 14) |
| `cargo run --release --bin labs -- --passk` | Reliability against average success: pass^k over the tasks of four worlds, with the band of 2,000 repetitions (Chapter 16) |
| `cargo run --release --bin labs -- --oracles` | Analytic oracles at limit cases and the 21 mutants of Appendix E they kill (Appendix C) |
| `cargo run --release --bin labs -- --jev` | The Jev-LLM bridge on the 32 sealed real outcomes, with the 96-parameter numerical baseline `tensor-v1` and what awaits an open-weights GGUF run (Chapter 16) |
| `cargo run --release --bin labs -- --layers` | Can the layers fail: the 60 external errors, the lexical gate, Fleiss' kappa, and the falsifier rule (Chapter 2) |
| `cargo test --workspace` | That the types and the gates of the architecture work: 449 tests pass |
| `python3 -I scripts/reimplementation/run.py` | That 573 quantities of 20 chapters, recomputed from the text and the data files alone in another language, agree with the book (570), or are marked open where the recipe is longer than a paragraph and stays cited (3) |
| `cargo run --release --bin gates -- --horizon all --reproduction` | That the gates run twice and give the same result, with the Merkle root `0a9636fac5eb1c5e1313dc73c709f2d5b0729703f004ab9cb9764b60fda4e12a` (and, by the rule in force until 2026-10-20, `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81`) |
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

The audit did not cover every draw, and on 2026-10-19 the rest was audited by brute force. Of the 27 modules of the
laboratories that draw random numbers, 4 print their own spread (the propagation, the multiverse, the reliability and
the stacking laboratories), 6 are audited number by number by `--mc-error`, 4 are of another kind (Chapter 1, which
is about seeds; the discrepancy, which averages over a thousand datasets; the choice of cases of the bridge, a
selection and not an estimate; the triage of Chapter 16, a hash and not a draw), and the other 13 are rerun whole by
`labs --seed-spread` under twenty other seeds, line by line against the published run. Numbers move, as they should;
a word that moves is a verdict that depends on the draw. Of 312 lines, three have words that move, and their
chapters now say so: one set of Chapter 9 accepted under one seed and rejected under another (the intersection the
method reports does not move), the tail of Chapter 15 read as not wild at the edge of the cutoff under 2 of 20
seeds, and an invariance test of Chapter 15 that fails its tolerance under 2 of 20 seeds although the effect does not
change. A register of the 27 is kept in the code, and a test refuses a new module that draws random numbers without
saying where its spread is measured. The run of `--seed-spread` takes minutes, so it is recorded with its commit and
not repeated by the verification suite.

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
| 16 | The machine | System 2 woken by margin; agents judged by average success; the panel without a map of how teams fail | the value of computation; the fourteen failure modes of teams of agents held against the panel (four caught, seven in part, three not seen, until contract 1.5 moved the three to in part; `docs/MAST_AUDIT.md`); reliability as pass^k, with the laboratory of four worlds (`labs --passk`, sealed in point 25 before its run) | the research on resource rationality and on agent benchmarks |
| 17 | The card | contract 1.3 | contract 1.4: the multiverse and the reliability, optional, validated when present | the two laboratories measured what the card had no place for |
| 19 | Invariance | causes across environments | also models of other minds, tested by alterations their mechanism declares irrelevant or decisive | the research on theory of mind in language models |
| E | Chapter 8 | "the instrument measures 0.1712 → 0.1559, 0.1560 and 0.2119 → 0.1755" | measured until 2026-10-18 in sample; out of the fit it measures 0.1708 → 0.1555, 0.1547 and 0.2147 → 0.1741 | the provenance check that closed item 16 found the paragraph speaking of the old reading in the present |

### The card contract 1.5, 2026-10-19

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 16 | The failures of teams of agents | four caught, seven in part, three not seen | four caught, ten in part, none unseen, with the three that passed between the specialists now declared in the card | the audit of the panel found three failures nothing saw |
| 17 | The fields of the card | contract 1.4 | contract 1.5: authorship by role, the findings used and not used, and the decision rule recomputed from the card | the same |

### The seed over whole laboratories, 2026-10-19

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 9 | The single run of the ICP | the verdicts of one seed | the same, and under twenty other seeds the set {X, W} is rejected once while the intersection stays {X} | `labs --seed-spread` |
| 15 | The tail lens | "the tail is wild" at every cutoff | the same at the published seed; at k = 250 the estimate passes 2 under 2 of 20 other seeds | the same |
| 15 | The invariance lens | "the two rules pass the invariance test with tolerance 0.5" | the same at the published seed; the first rule fails the tolerance under 2 of 20 other seeds, and a tolerance is set from the noise of the estimate | the same |
| C.3 | The audit of the seed | 14 modules not audited | none: 13 rerun whole by `--seed-spread`, and the triage of Chapter 16 reclassified as a hash | the same |

### The seed of the simulations, 2026-10-19

A new instrument, `labs --mc-error`, measures how far each simulated number that a reading depends on moves with the
seed (Appendix C.3). No number changed and no reading moved; two published values were found at the edge of their
seed spread, and the chapters now say so.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| 8 | The null q95 at 1,488 cases | 0.0425, as if exact | 0.0425, the largest of twenty-one seeds; about 0.042 | a simulated quantile carries the error of its draws |
| 14 | The bootstrap interval of the tail index | [1.37; 2.61], as if exact | the same, with its upper bound above every other seed's | the same |
| C | Reproduction | the environment is part of the number | also the seed: C.3 | the same |

### The analytic oracles, 2026-10-20

A review made with AI assistants proposed that every formula of a decision pass identities known in closed form before
it runs on data. The review's own reference for the M/M/1 wait left out the division by $\mu$, the same kind of error as
the Kelly row of the first table, so the oracles were built with a second rule: each recorded error of this appendix is
rewritten as a mutant with the signature of the instrument it lived in, and an oracle counts only if a mutant fails it.
`labs --oracles` runs both. The oracles found one wrong definition, in Chapter 10, corrected in the last row. Like the readings above, this one is outside the authors'
audit and is not the independent audit of Chapter 21.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| C | The check of a formula | one test per error, written after it was found | 19 identities at limit cases (Pollaczek–Khinchine at $c_v = 1$ and $c_v = 0$, $C(1, \rho) = \rho$, $g'(f^*) = 0$, $F(\text{level}) = 1 - 1/T$, the round trip of the Chouldechova identity, $0 \le \text{EVSI} \le \text{EVPI}$, the scale invariance of Hill, RES of a constant forecaster, one definition of Hill across chapters, and others); all pass on the instruments, and 17 of 17 mutants die | a test written after an error catches that error; an oracle is measured by the errors it would have caught |
| C | An oracle with a wrong reference | — | the review's reference $60\lambda/(\mu - \lambda)$ rejects the right wait by up to 1265.00 min | an oracle is a claim too; when it fails on an instrument, the closed form decides which of the two is wrong |
| 10 | The threshold of Hill | $\hat\alpha = k / \sum \ln(x_i / x_{(k)})$, and the laboratory used $x_{(k)}$: at $k$ = 100, 500, 1,000 and 5,000 the estimate was 1.083, 1.063, 1.100 and 1.101, with interval $[1.032; 1.168]$ at $k = 1,000$ | $x_{(k+1)}$, as in Chapter 14, the horizon H1 and the tail lens: 1.080, 1.059, 1.099 and 1.100, with $[1.031; 1.167]$ at $k = 1,000$; at $k = 20,000$ the value 1.095 does not move | with $x_{(k)}$ the $k$-th term is $\ln 1 = 0$, so $k$ divided a sum of $k - 1$ terms; the oracles found the two definitions, and the old threshold is now a mutant they kill. The reading (Hill recovers the exponent) does not change |
| C | Test count | 434 | 439 | the five tests of the oracles |

### The oracles of Chapter 5, the calibration of Chapter 10 and the separated root, 2026-10-20

The same review was carried three steps further. A mutant now has to reproduce the value it stands for: the six-point
rule of Chapter 5 gives 14.10 min against the closed form's 14.46, as recorded, and the threshold $x_{(k)}$ gives the
1.083 that Chapter 10 printed. Applied to the calibration of the synthetic market, the rule found that the cause
recorded above did not produce the value recorded above. No published number changed.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| C | The oracles | 19 identities, 17 of 17 mutants killed | 23 identities (the exact toll of Jensen with two demands, the uniform mean as an integral, the curvature of the wait, the round trip of the market's calibration) and 21 of 21 mutants killed | the formulas of Chapter 5 and the calibration of Chapter 10 had no identity of their own |
| 10 | Cause of the earlier calibration | "the calibration dropped the factor of 252 sessions" | the earlier $\sigma = 0.01518$ solves $4.5\,\sigma^2 d = 0.14$ with $d = 135.0$ sessions, not 252, and gives a drag at $3\times$ of 0.2613 per year; the equation without $d$ gives 0.1764, and with 252 gives 0.01111 | the recorded cause did not reproduce the recorded value; the research record also counts 135 calendar days of the closure of 1914, and whether the two are related is not known |
| 18 | Merkle root | a level with an odd count repeats its last hash; a list and the same list with its last element repeated share a root | the legacy root stays, and every root published is still computed by it; beside it, a root with domain separation (leaf $\mathrm{SHA256}(0\mathrm{x}00 \Vert x)$, node $\mathrm{SHA256}(0\mathrm{x}01 \Vert l \Vert r)$, the lone node moved up), checked against the definition of RFC 6962 for 1 to 33 leaves | the declared limit of 2026-10-19; switching the gates to the new root is the owner's decision, since it changes the root that Appendix C cites |
| C | Test count | 439 | 444 | four tests of the separated root and one of the oracles |

### The root with domain separation in force, 2026-10-20

By the owner's decision, the root with domain separation replaces the legacy rule wherever a root is computed: the
gates, the ledger of the vault and the laboratory of Chapter 18. The legacy rule stays as a function, and the gates print
both roots, so that every root published before can still be reproduced. No verdict changed: seven gates pass, the
negative proof still fails G6, and G1 now checks that both roots repeat.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| C, 17 | Root of the gates, horizon `todos` | `9716e28a87bec9cf...` | `0a9636fac5eb1c5e...`; the legacy root is printed beside it | the rule changed; the cards did not |
| C | Root of the negative proof | `802e84b6...` | `8dfb949d...` | the same |
| C | Hash of the report | `bb7edf13...` (`todos`), `93565e65...` (negative proof) | `49427021...`, `fbbbf27c...` | the report carries both roots |
| 18 | Root of the textbook chain | `92f2ab76eb6485ab` | `e81457d50006a7c8` | the same |
| 18 | Root of the batch of seven cards | `17679ce1...` | `ed9aa4bb...`; the swap of any card still changes it in 7 of 7 cases | the same |
| 18 | The declared limit | a list and the same list with its last element repeated share a root; the gates record the count beside it | they no longer share a root | the lone node moves up instead of being paired with itself |

### The Jev-LLM baseline and the external error catalogue, 2026-10-20

Two open items of the research record received their sealed machine layers, audited by four subagents and recorded with their exact scope. On the Jev-LLM bridge (`empirical/p20-jev/`), 32 real outcomes from the Danish fire series and the Atlantic storm series were sealed before the run (`calibracao/desfechos.tsv`), together with a 96-parameter, 2-layer numerical MLP baseline (`tensor-v1`, in `pesos/pesos.tensor` and `pesos/modelo.congelado`) whose Rust template decoder emits valid cards by construction (`Brier = 0.293988`, `ECE = 0.338131`, `G1 identical: true`, `0` abstentions); the open-weights GGUF comparison with and without the GBNF grammar remains open. On whether the three layers can fail (`empirical/p21-camadas/`), 60 documented analytical and institutional errors across 45 domains were sealed (`erros.tsv`) with the lexical gate (`27` forbidden tokens), the instrument `labs --layers` (`0` human coders), and the synthetic rehearsal `labs --rehearsal-layers` (`Fleiss' kappa = 0.6405`, 90% CI `[0.5447; 0.7255]`); the item remains open until three human coders, two from outside, code the 60 errors.

| Chapter | What | Earlier | Corrected | Cause |
| --- | --- | --- | --- | --- |
| C | Test count | 444 | 449 | one test in `runner.rs` (`tensor-v1` round-trip and card decoding) and four tests in `camadas.rs` (catalogue seal, lexical gate, Fleiss' kappa, and synthetic rehearsal) |

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
five textbook events, and for that reason the root measured here is `92f2ab76...`, not that one (since 2026-10-20, with the separated root, `0a9636fa...` and `e81457d5...`). The two
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
