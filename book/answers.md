# Answer key

**How to read it.** Every *Count* answer carries its computation, so that a reader can redo it; every number here that
also appears in a chapter is the chapter's. A *Laboratory* answer says what to change, what to look at, and what the
reader should see; where it gives a number, the number was measured, and where it gives only a direction, the
direction is what the theory predicts and the reader measures the size. A *Concept* answer is one acceptable answer,
not the only one: it names what a complete answer has to contain. The key was written on 2026-10-19, with an AI
assistant, from the chapters as they stand; it is part of the book's errata like any other page.

---

## Chapter 1. The Invisible Target

1. *(Concept)* The formal part keeps beliefs coherent, updates them with evidence, measures margins on the right
   unit and scores forecasts; the part that is not formal decides which model deserves attention, when to stop
   collecting data and who pays if the decision is wrong. The doctor of Step 1 cannot avoid the second part: no
   formula tells her how much a free bed is worth against a missed infarction.
2. *(Concept)* Yes, whoever knows the seed knows the result, because the generator is deterministic. The draw is
   honest when the seed is published before the draw and cannot be changed afterwards, because then nobody can
   choose the seed knowing the result it gives; what the seed buys is that anyone can check that the published
   result is the one the published seed produces.
3. *(Count)* 6, 1, 8, 11, 10, 5, 12, 15, 14, 9, 0, 3, 2, 13, 4, 7: the sequence returns to 7 after 16 steps, which is
   every residue once. A real generator with a small modulus repeats after at most that many draws, so a long
   simulation would reuse the same numbers; real generators use moduli near $2^{64}$ for that reason.
4. *(Count)* No. The standard deviation of the fraction after $n$ tosses is $\sqrt{0.25/n}$: 0.05 at 100 tosses,
   so 0.03 is within one deviation; 0.0016 at 100 thousand tosses, so 0.03 would be about 19 deviations away. The
   law of large numbers shrinks the fluctuation like $1/\sqrt n$.
5. *(Laboratory)* The number of coinciding draws stays at 0, as with seed 1909 against 1713. Two different seeds
   give unrelated sequences; the coincidence count does not depend on how close the seeds are.
6. *(Concept)* (a) is degenerative: the rescue predicts nothing that could fail, since compliance was not measured
   and cannot be checked. (b) is progressive: the repair states, before the new trial, an observation that could
   refute it (a higher dose working in older adults), and the observation was made.

## Chapter 2. World, Model and Agent

1. *(Concept)* The real arrival rate is World; the queue formula is Model; the decision to hire is Agent; the
   patients who leave on seeing the queue are the Agent-to-World bridge: an action of the people in the system
   that changes the World the Model describes.
2. *(Concept)* On the Model-to-Agent bridge: $P(\text{match} \mid \text{innocent})$, a property of the Model, is read
   as $P(\text{innocent} \mid \text{match})$, the answer about the World. The missing quantity is the prior: how many
   people could have left the trace, the prevalence of guilt in layer $L_1$.
3. *(Count)* With coincidence $c = 10^{-6}$ and prior guilt $g$, $P(I \mid E) = (1-g)c / ((1-g)c + g)$. Setting it to
   0.09 gives $g = 0.91c / (0.09 + 0.91c) \approx 1.01 \times 10^{-5}$, about one in 99 thousand. The answer depends
   on $L_1$ because the same evidence ratio is multiplied by prior odds that can span many orders of magnitude.
4. *(Laboratory)* With coincidence 1 in 1,000, $P(I \mid E)$ becomes about 0.09 at prevalence $10^{-2}$, 0.50 at
   $10^{-3}$ and 0.999 at $10^{-6}$; the swap factor falls from 499,999.7 to about 999 in the last world, because the
   evidence is a thousand times weaker. The order of the lesson does not change: the lie grows as the prior shrinks.
5. *(Concept)* Base-rate neglect stops being an error in a referral clinic where the base rate is near the
   likelihood's implicit prior (Chapter 7 measures it). The gambler's fallacy stops being one when the process has
   memory, such as a deck dealt without replacement. Availability stops being one when the memorable cases are the
   frequent ones, as with common illnesses in a general practice.
6. *(Concept)* Whoever plants the errors knows where they are and what kind they are, and an analyst who shares that
   knowledge cannot be fooled by them; the test then measures memory, not method. If the same person planted and
   analysed, every defect would be found by both arms, and the ceiling tie of Chapter 21 would be built in.

## Chapter 3. Probability without mystery

1. *(Count)* 10 sick women, 9 of whom test positive; 990 healthy, of whom 9% is 89.1 test positive; 98.1 positives in
   all. $9 / 98.1 = 0.0917$, the same as with 10,000 women: the tree scales, the proportion does not change.
2. *(Count)* Independent: 10 decibans; posterior odds $0.101 \times 10 = 1.01$, posterior $0.502$. With the effective
   ratio 4.2: $10 \log_{10} 4.2 = 6.23$ decibans; posterior odds $0.424$, posterior $0.298$. The chapter's laboratory
   prints 0.5025 and 0.2979.
3. *(Concept)* The first error is in the Model: squaring $1/8{,}543$ assumes two independent deaths, which the World
   does not grant to siblings who share genes and a home. The second is on the Model-to-Agent bridge: the chance of
   the coincidence was read as the chance of innocence, without the prevalence of double infant deaths and of
   murder in the World.
4. *(Count)* The Kullback-Leibler divergence of a 0.55 coin from a fair one is $0.55 \ln 1.1 + 0.45 \ln 0.9 =
   0.00501$ per toss; odds of 10 to 1 need $\ln 10 = 2.30$, so about 460 tosses. It is an average: the expected
   weight of evidence per toss, so a given run can reach the odds earlier or later.
5. *(Laboratory)* With an effective ratio of 2.0, the second test is worth 3 decibans and the posterior with two
   positives falls to about 0.17 (odds $0.101 \times 2 = 0.20$). The conclusion "more likely sick than not" is
   already gone at 4.2; at 2.0 the second test barely moves the belief from the first.
6. *(Concept)* Any case in which a likelihood, a sensitivity, or a $p$-value was reported as the probability of the
   hypothesis: "the test is 95% accurate, so you are 95% likely to be ill" is the common form. The complete answer
   names the missing prior.

## Chapter 4. Luck, ignorance and the Christmas turkey

1. *(Concept)* The next toss is aleatory; the true bias is epistemic; the chance that the government changes the
   rule is Knightian, a possibility outside the model's hypothesis space; the measurement error of a calibrated
   scale is aleatory (its spread) with an epistemic part (the calibration constant), both small by construction.
2. *(Count)* Uniform prior: $(7+1)/(10+2) = 0.667$. A uniform prior on $\theta^2$ is a Beta(2, 1) on $\theta$ and gives
   $(7+2)/(10+3) = 0.692$; a uniform prior on $\sqrt\theta$ is a Beta(1/2, 1) and gives $(7.5)/(11.5) = 0.652$. Declare
   the one whose shape matches what is known before the data; with nothing known, the uniform, and say so.
3. *(Count)* A 95% interval of width 0.0240 has a standard error of about $0.0240 / 3.92 = 0.0061$; a bias at 0.6
   against an estimate near 0.4 is 0.2 away, about 33 standard errors. The interval is precise and wrong.
4. *(Laboratory)* Discount 0.98 remembers about 50 observations: the interval widens and coverage rises toward the
   nominal 95%; discount 0.995 remembers about 200: the interval narrows and coverage falls, because the model
   averages over a longer stretch of a bias that keeps moving. The trade-off is memory against coverage, and the
   right discount depends on how fast the world drifts, which must be declared.
5. *(Concept)* A complete answer names a regime change the model did not list (a rule, a market, a protocol) and
   places it in the bottom-right corner of the Stirling matrix: outcomes not well defined and probabilities not
   known, where the remedy is vigilance and reversibility, not a narrower interval.

## Chapter 5. When the mean lies

1. *(Concept)* (a) convex: the mean wait exceeds the wait at the mean demand. (b) linear: no toll. (c) multiplicative:
   the mean wealth exceeds the typical wealth, the toll of Chapter 10. (d) a step: the share below the cut-off is not
   a function of the mean at all.
2. *(Count)* $W(6) = 5$, $W(8) = 10$, $W(10) = 25$ minutes. $f(\mathbb{E}[X]) = 10$; $\mathbb{E}[f(X)] = (5 + 25)/2 =
   15$; the toll is 5 minutes. The second-order formula gives $\tfrac12 \cdot 1.875 \cdot 4 = 3.75$: it underestimates,
   as the chapter says, because the third derivative is positive.
3. *(Count)* $0.563 / 0.097 = 5.8$ times. A report using the margin per patient would claim a precision the data do
   not have: the patients of one day share its demand, and the honest unit is the day. The 10.6-minute difference
   stays significant on either margin, but the report would have declared the wrong uncertainty.
4. *(Laboratory)* With "4 or 12" the variance of demand is 16 and the surge day runs at capacity, so the mean wait
   rises well above the 19.6 minutes of "5 or 11" and the share above 30 minutes climbs; with "7 or 9" (variance 1)
   both fall close to the constant world. The three points rise faster than the variance, which is the sign of a
   convex toll with a positive third derivative.
5. *(Concept)* "In every 100 patients, about 24 wait more than half an hour on the days we have, against 8 if demand
   were flat; the median wait is 8.6 minutes and the mean 19.6. Putting a second nurse only on the busy days takes
   the 24 down to fewer than 1 in 100."
6. *(Laboratory)* With 1.5 times the capacity on surge days the wait falls part of the way and some patients in
   each 100 still pass 30 minutes; with 3 times it falls further than with 2 times, toward none. The reader
   measures the two shares; the lesson is that the tail responds to the slack placed on the busy day, not to the
   mean.

## Chapter 6. The invisible filter

1. *(Concept)* The quality factor $\rho_{R,Y}$: the correlation between answering and the answer. Millions of users
   shrink the quantity factor, not the quality one; the *Literary Digest* had 2.4 million ballots and the precision
   of 7 people because the people who answered differed from those who did not.
2. *(Count)* Open surgery: $(81 + 192)/(87 + 263) = 273/350 = 0.78$; other technique: $(234 + 55)/(270 + 80) =
   289/350 = 0.826$. The raw rate weights each stratum by the number of cases, and open surgery treated 263 large
   stones against 80; the stone size is the confounder, which decides both the treatment and the success.
3. *(Count)* $|\text{error}| = |\rho| \sqrt{(1-f)/f}\, \sigma_Y$. For 10 points, $(1-f)/f = (0.10 / (0.21 \cdot 0.488))^2 =
   0.952$, so $f = 0.51$: half the electorate. For 5 points, $f = 0.81$. Even 50 million ballots are not a sample of 81%
   of the voters, and the error does not fall below 10 points until the sample is most of the population.
4. *(Laboratory)* With equal response rates the respondents' share for Roosevelt equals the electorate's and the
   measured error is zero: $\rho$ is zero by construction. With $\rho = -0.05$ and $f = 0.24$ the error is
   $0.05 \sqrt{0.76/0.24} \times 0.488 = 0.043$, about 4.3 points, and the poll is worth a random sample of
   $f/((1-f)\rho^2) = 0.316 / 0.0025 \approx 126$ people, against 7.2 with the correlation of 1936.
5. *(Laboratory)* The variance of the mean of a two-state chain with persistence $q$ is that of independent letters
   times $(1 + r)/(1 - r)$ with $r = 2q - 1$: persistence 0.8 gives the factor 4 and the effective size 25 the chapter
   prints. Persistence 0.9 gives the factor 9 and an effective size near 11 letters in 100: a report treating them as
   independent understates its margin by about 3. Persistence 0.1 gives the factor $1/9$ and an effective size near
   900: the alternating chain is worth more than independent letters, and the independent margin is too wide.
6. *(Concept)* MNAR: income not reported more often by those who earn more; the mean of the reported incomes is
   biased downward and no weighting on observed variables repairs it. MAR: young people answer less and age is
   recorded; weighting by age recovers the population mean. The complete answer says which conclusion survives.

## Chapter 7. Hunting ghosts in the noise

1. *(Concept)* The pooled estimator counts every transition after a hit, so long runs of hits weigh more. The mean
   of per-sequence proportions gives each sequence one vote, and the sequences with no hit at all drop out; among
   those that remain, a sequence whose only hit is the last toss contributes 0, and such sequences are common. The
   two counts answer different questions, and only one of them has 1/2 as its target.
2. *(Count)* $\text{PPV} = 0.8 \cdot 0.05 / (0.8 \cdot 0.05 + 0.05) = 0.44$. With power 0.40, $0.02 / 0.07 = 0.29$. A
   small study lowers the chance that a positive finding is true, because it finds fewer of the true effects and
   the same share of false ones.
3. *(Laboratory)* With 10 candidates, the winner beats zero on validation less often and the gap to fresh data
   shrinks; with 5,000, the winner looks better on validation and worse on fresh data. More attempts improve the
   appearance and worsen the reality.
4. *(Laboratory)* One more freedom raises the false-positive rate above 20.3%, by less than doubling it, because the
   two statistics are correlated. The consequence: every freedom must be declared before running, since each one
   added afterwards is one more chance for noise to cross the line.
5. *(Concept)* With a safeguard present the draw gave 8 to 10, all above the floor 6; absent, 2 to 4, all below. So the
   verdict (every band above the floor) equalled the AND of the eight bits under every seed, and the decimal added
   nothing but a seed-dependent number. Mayo's severity is a probability about a procedure's capacity to err; a
   number drawn from a range selected by a bit has no such meaning.
6. *(Concept)* Any environment whose base rate is near one half, where the forecast of whoever ignores the base rate
   is right. The two numbers are the Brier of whoever ignores the base rate and of whoever insists on the wrong one:
   in the chapter's referral clinic, 0.085 for the heuristic against 0.28 for the forecaster who keeps the 1% of the
   screening, where the same heuristic scored 0.083 against 0.009.

## Chapter 8. The forecaster's thermometer

1. *(Concept)* The accuracy rate scores the action, not the belief, so a forecaster who believes 51% and one who
   believes 99% get the same grade for the same "it will rain"; a forecaster can raise it by lying about the belief
   near the threshold. Two forecasters right 80% of the time, one saying 0.55 and one saying 0.95 on every rainy
   day, have Brier scores far apart.
2. *(Count)* Observed frequency in the box $48/200 = 0.24$. REL contribution $(0.30 - 0.24)^2 = 0.0036$; RES
   contribution $(0.24 - 0.24)^2 = 0$. The forecaster is overconfident on this box and carries no information in it:
   the scale should come down to the base rate there.
3. *(Count)* $0.0425 \sqrt{1{,}488 / 148{,}800} = 0.0043$. For $n = 104{,}489$ the rule predicts 0.0051, against the
   measured 0.0037: the growth is in the right direction and the measured value is smaller, because the null also
   depends on the spread of the forecasts, not only on $n$.
4. *(Laboratory)* The intercept shift fixes the level and leaves the slope at 1. The lazy forecaster's distortion is
   one of level, so the shift recovers nearly all that Platt recovers; the overconfident one's is a slope problem, so
   its Brier falls little under the shift and the ECE stays high. The slope matters where the forecaster is too sharp
   or too flat, and only there.
5. *(Laboratory)* The three Briers of the expert differ in the third decimal, by an amount of the order of the
   difference between the expert and the overconfident forecaster after Platt. Comparing two forecasters in 4,000 days
   needs a margin, and the seed spread is a measure of it.
6. *(Concept)* The noise of the ECE of a perfectly calibrated forecaster shrinks like $1/\sqrt n$: with 1,488 cases
   its 95% quantile is about 0.03, so an ECE of 0.0354 is barely above noise; with 104,091 cases the quantile is
   0.003, so 0.0177 is five times the noise. The same ruler would call the small vault worse than the large one when
   the large one's miscalibration is the one that is detectable beyond doubt.

## Chapter 9. The ladder of cause and effect

1. *(Concept)* Association (seeing), intervention (acting, a $\operatorname{do}$), counterfactual (imagining a
   parallel world).
2. *(Concept)* $X \to Z \leftarrow Y$: $Z$ is a collider. Conditioning on it (looking only at the admitted, the hired,
   the selected) opens an association between two independent causes; Berkson's inpatients and the chapter's
   talent and likeability are the examples.
3. *(Count)* The same table as Chapter 6: $273/350 = 0.78$ against $289/350 = 0.826$. Open surgery took 75% of the large
   stones and the other technique 23%, so the raw rate of open surgery is pulled toward the hard stratum. Stone
   size is the confounder.
4. *(Count)* The pooled slope weights each site by the spread of $Z$ within it and by the covariance across sites,
   not equally, so $0.6003$ is not $(0.928 + 0.496 + 0.299)/3 = 0.574$. A single regression coefficient over mixed
   environments is a weighted average of slopes that may differ, and it hides that none of them is the coefficient.
5. *(Laboratory)* With equal noise of $Z$ and no shift of $X$, nothing moves between sites, so every set is invariant,
   including the empty one, and the intersection is empty. ICP needs environments that move the cause; it does not
   find the cause in data where nothing intervened.
6. *(Concept)* With enough data, the invariance test rejects any deviation from the model (a slight nonlinearity, a
   heavier tail), so every set is rejected and the intersection is empty. Abstention then says "the model is wrong
   somewhere", not "there are no causes".

## Chapter 10. Time and ruin

1. *(Count)* $\mathbb{E}[\ln M] = \tfrac12 \ln 1.5 + \tfrac12 \ln 0.6 = 0.2027 - 0.2554 = -0.0527$ per round, which
   is the measured $-0.052680$. The median follows the time average, which is negative, while the expectation follows
   the ensemble average, $+5\%$; the mean is carried by a vanishing share of trajectories.
2. *(Count)* Maximise $p \ln(1 + f) + (1 - p) \ln(1 - f)$: the derivative $p/(1+f) - (1-p)/(1-f) = 0$ gives $f = 2p - 1$.
   In general, maximise $p \ln(1 + fb) + (1-p) \ln(1 - fa)$: $pb/(1 + fb) = (1-p)a/(1 - fa)$ gives
   $f = p/a - (1-p)/b$. For the Peters coin, $0.5/0.4 - 0.5/0.5 = 0.25$.
3. *(Concept)* Splitting averages the factors before taking the logarithm; by Jensen, the logarithm of an average is
   larger than the average of the logarithms. The expectation of each player's wealth is unchanged (the average of
   expectations), but the time growth, which is the expected logarithm, rises because pooling cuts the variance that
   the logarithm punishes.
4. *(Laboratory)* The raw frequency makes 36.8% of the investors go all-in and 1.9% of them are ruined in 1,000 weeks;
   Laplace keeps every stake near Kelly and ruins none. Without the ceiling of $f \le 1$, the raw-frequency investors
   leverage against an estimated zero risk, and ruin rises toward the 35.8% the earlier version of this laboratory
   measured (Appendix E).
5. *(Count)* $C/L = 1{,}800 / 34{,}000 = 5.29\%$ and $p = 5\%$ is below it, so the cost/loss rule says do not insure.
   The time growth says insure: $-0.0208$ per month without, $-0.0182$ with.
6. *(Concept)* A complete answer names a spreadsheet mean (a fund's average return, a portfolio's expected value, an
   average salary trajectory) that is an ensemble average, and the person who lives one path. The crowd is the fund or
   the insurer with many independent exposures; the trajectory is the individual.

## Chapter 11. The price of error

1. *(Count)* As Chapter 10, exercise 5: the threshold is 5.294% and 5% is below it, so expectation says do not
   insure; the time growth, $-0.0208$ against $-0.0182$ per month, says insure, because a 34% loss hurts a single
   trajectory more than its expectation shows.
2. *(Count)* At prevalence 0.10, treating costs 1 and not treating costs $10 \times 0.10 = 1$: the two acts tie at $-1$.
   Knowing the state gives $-0.10$. EVPI $= 0.90$, higher than at 0.3 (0.70): information is worth most exactly at the
   threshold, so that is where testing pays.
3. *(Concept)* The naive forecaster errs more about the times, because it sends everyone to the fastest route and
   moves the jam there. The second forecaster's forecast comes true because it built the world it forecast; it is
   performatively stable, optimal for the distribution it induces, which is a property of the loop, not of the
   world.
4. *(Count)* True positives $0.069 \times 0.76 = 0.0524$; false positives $0.931 \times 0.105 = 0.0978$; PPV $= 0.349$.
   The identity returns $0.069/0.931 \times 0.651/0.349 \times 0.76 = 0.105$, the observed FPR. With PPV 0.349 and FNR
   0.24 imposed on Black at base rate 0.111: $0.111/0.889 \times 1.866 \times 0.76 = 0.177$.
5. *(Laboratory)* With base rate 0.20 for the second group, equalising the FNR downwards costs more false positives
   than before, because a larger share of the group is positive and the threshold moves on more people; equalising
   upwards costs more false negatives in the first group. The gap between base rates sets the price of every
   equality.
6. *(Concept)* The insurance seller who estimates the chance of a loss as exactly 0 from 250 clean weeks: a
   probability of 0% cannot be updated by any evidence, and it licenses unlimited leverage. Laplace's floor
   prevents the certainty.

## Chapter 12. The health post and the bank

1. *(Count)* $W(8) = 10$, $W(5) = 3.57$, $W(11) = 55$ minutes; the mean of the last two is 29.3, against 19.3 in the
   simulation. The formula gives the long-run wait of a queue that never empties; a day of 10 hours starts empty and
   does not reach the long run of the surge day.
2. *(Concept)* In every 100 patients, about 69 arrive on days of 11 and 31 on days of 5 ($11/16$ and $5/16$), so the
   mean per patient weights the bad days by the number of people who lived them.
3. *(Laboratory)* The wait of "5 or 11" and the margin per day reproduce with seed 1909. Two nurses on one line serve
   two patients at once, so a patient who arrives finds a free nurse more often than with one nurse at twice the
   pace, who serves one at a time and still makes everyone queue behind the current patient; the mean wait is lower
   (1.9 against 2.5) and the tail shorter.
4. *(Count)* $8{,}000 \times (0.24 - 0.08) = 1{,}280$ more patients a year waiting more than half an hour.
5. *(Concept)* "In every 100 days, the mean-profile schedule leaves about 31 below the service target; the weekday
   profile with a margin for the residual variation leaves about 5, for 7% more agent-hours. The slack goes where the
   risk is, not everywhere. We review the decision if the share of failed days under the new rule exceeds 10 in 100
   over a quarter, or if the day-to-day dispersion of calls falls below half of today's."
6. *(Concept)* The manager asked "how many patients wait too long on the bad days?"; the formula answered "what is
   the long-run wait at the mean demand?". The arithmetic is right and the question is the wrong one, which is the
   type III error.

## Chapter 13. The market and the employment programme

1. *(Count)* $\tfrac12 L^2 (0.1749)^2$ gives 0.0153, 0.0612 and 0.1376 against 0.0153, 0.0617 and 0.1403 measured. The
   drag $\tfrac12 L^2 \sigma^2$ exceeds the gain $L \mu$ when $L > 2\mu/\sigma^2 = 2 \times 0.0668 / 0.0306 = 4.4$.
2. *(Count)* $1/0.2261 = 4.42$. On the worst day of 1929 the fall was smaller, so the leverage that zeroes the capital
   is higher. Ruin is absorbing because nothing is gained from zero; the ensemble mean keeps growing on the
   surviving trajectories and stops describing the one that was ruined.
3. *(Concept)* The sentence claims that a long horizon turns the arithmetic mean into the realised growth. The
   refutation brings down "yields 3 times the mean" and "always wins" (36 in 100 cohorts end below the start); it
   leaves standing that 2× and 3× grow more than 1× in time without financing costs, and that the vault favoured
   the claim.
4. *(Laboratory)* With a 10-year horizon the fraction of 3× cohorts above the start falls and the median falls,
   because a shorter window gives the drag less time to be offset by growth and gives a single crash more weight.
5. *(Concept)* Employment in 1975 is an outcome the programme could not have caused, so its $Y(1) - Y(0)$ is zero;
   the naive difference in 1975 is therefore pure selection bias, $\mathbb{E}[Y(0) \mid D{=}1] - \mathbb{E}[Y(0) \mid
   D{=}0]$. The propensity fit gives $-12.3$ points there, so the selection it leaves behind is measured.
6. *(Count)* $76 - 65 = 11$ percentage points. The raw comparison with the national survey gives $76 - 86 = -10$,
   the opposite sign, because applicants to the programme were far less employed than the average worker before it
   began.

## Chapter 14. The Danish fires

1. *(Count)* $1/14 = 0.071$, $1/38 = 0.026$, $1/71 = 0.014$. Below 1 in 100 needs $n + 2 > 100$: 99 clean months.
2. *(Concept)* A margin built on a variance needs the variance to exist, and with $\alpha = 1.81 < 2$ the variance of
   the losses is infinite: the sample variance grows with the sample instead of converging. The interval [1.41; 2.56]
   straddles 2, so the data do not even say whether the variance exists.
3. *(Count)* $(1 - 1/1200)^{36} / (1 - 1/12)^{36} = 0.970 / 0.0436 = 22.3$, about 4.5 bits. Against a world with a
   monthly rate of 1 in 12, three clean years are reasonable evidence; against the training-set rate of about 1 in 23
   they were worth 2.2 bits, and the bits authorise no more than "the risk is probably not as high as one in twelve".
4. *(Laboratory)* With $k = 50$ the laboratory measures $p = 0.0463$ on the training set, a likelihood ratio of 5.35
   and 2.42 bits for 36 clean months. With 72 the ratio squares, $5.35^2 = 28.6$, about 4.8 bits: twice the window,
   twice the bits. The number of clean months for 95% does not change with the window, because it depends on $p$
   alone, $(1 - p)^m \le 0.05$: 64 months at this $p$, 69 at the 1-in-23 of the text.
5. *(Concept)* "No forecaster, including ours, predicted the months of the vault better than the historical
   frequency, so we have no model that earns the right to set the price. The claim that the risk is negligible lost:
   three clean years are two bits of evidence. The honest answer is to price by the historical frequency, with the
   capital that a 1-in-23 month requires, and to refuse the leverage the claim asked for."
6. *(Laboratory)* With $p = 1/1200$ almost every window of 36 months is clean (about 97 in 100), so a clean window
   carries no information against the negligible world. In the real Danish series, clean windows and hit windows
   alternate within a decade, which is what a rate near 1 in 23 produces; the argument "negligible risk" is built
   on an observation that both worlds produce.

## Chapter 15. The map of knowledge

1. *(Concept)* `L1_demand`: the arrival process, aleatory, with the counts as artifact. `L2_mean_wait`: the formula
   $W(\mathbb{E}[\lambda])$, epistemic (specification). The pathology edge is the fallacy of the mean, pointing from
   the Model node to the World node: the formula is applied to the mean rate where the mean of the formula over
   the rates was due.
2. *(Concept)* Decision (the cost of a wrong grant and the value of more information), invariance (does the score
   transport to new applicants and new years?), information and invariance together because the model is published
   and the applicants react to it, and tail (default losses are heavy-tailed). Time applies if the client's
   repayment is a multiplicative trajectory.
3. *(Count)* Halving the stake halves every payoff, so every expected value halves, including the difference that is
   the EVPI. If acting costs 1 in a bad year, acting without information yields $0.7 \times 10 - 0.3 \times 1 = 6.7$,
   with perfect information $7.0$, so EVPI $= 0.3$: the sensor is worth what it saves in the bad years.
4. *(Laboratory)* The structural validation passes with the new node, because form is intact, while the coverage
   check names `L3_auditor` as a node outside every lens until it joins a hyperedge. A second cycle produces a
   second "cycle in E_tree" message naming the nodes of that cycle.
5. *(Laboratory)* With tolerance 2.0 rule 1 stays invariant and rule 2, whose middle regime has slope 40.09 against
   9.96 and 10.15, still fails: a step of 30 is not hidden by a tolerance of 2. With 0.05, rule 1's slopes 9.96, 10.09
   and 10.15 differ by more than the tolerance and the test calls the structure non-invariant though the effect is 10
   everywhere: in rule 1 the structure stops being distinguishable from noise, because the tolerance went below the
   noise of the slopes.
6. *(Concept)* Its two failure modes, non-stationarity and abandonment, are listed and each can be tested against
   the data, so the uncertainty is about a specification, which is epistemic. It would become Knightian if the
   formula failed for a mechanism nobody wrote down, which no test of the listed assumptions would detect.

## Chapter 16. The team of specialists

1. *(Concept)* The data specialist, with the question "what is a row?": a row per appointment counts a patient with
   ten visits ten times. The test is the count of rows against the count of distinct patients, and the margin must
   be computed on patients.
2. *(Concept)* Code: a property test with a seed the author did not choose. Data: a batch with corrupted incomes.
   Lifecycle: a batch from a recession. Inference: a pathological applicant set. Governance: a tampered trail.
   Fairness: inverting the metric from false positives to false negatives shows who pays: the group denied credit
   it would have repaid. Domain: a prior from another lender. Decision: a crisis in which defaults double is the one
   that violates an exit clause written for calm times.
3. *(Count)* Seven rows with both bits give 14; the eighth has the safeguard bit and not the anchor, 1 more: 15 of 16
   set. Not approved: $\bigwedge_i C_i$ is false if one $C_i = \text{safeguard}_i \wedge \text{anchor}_i$ is false.
4. *(Laboratory)* Draw $\theta \sim \text{Beta}(a, b)$, simulate $k \sim \text{Binomial}(n, \theta)$, sample the
   posterior Beta$(a + k, b + n - k)$ $L$ times and record the rank of $\theta$; over many repetitions the histogram
   of ranks is flat. A sampler that, say, uses $a + k$ and $b + k$ produces a histogram with a slope.
5. *(Count)* The logit of 0.20 is $-1.386$; the base-rate shift adds $\ln\big((0.3/0.7)/(0.1/0.9)\big) = 1.350$; the
   corrected logit is $-0.036$ and the probability $0.49$. The assumption is label drift alone: the distribution of
   the attributes within each class did not change.
6. *(Concept)* Covariate drift: the applicants are older than at training, and the relation age-to-default holds;
   reweight. Label drift: defaults became more common at every age; shift the intercept. Concept drift: a new law
   changed what default means; rebuild. Not a degradation: the model is now applied to a lower-risk product, so its
   accuracy falls while its calibration holds; read the metric against the base rate.
7. *(Concept)* System 1 must give the same answer to the same card so that its triage can be audited, as a fixed seed
   lets two machines agree; a triage that varies cannot be checked. System 2's belief must stay inside (0, 1) so that
   evidence can move it: a belief at 0 or 1 is frozen by Bayes' rule.

## Chapter 17. The iron contract

1. *(Concept)* Generating process: students choose to study with music; grades depend on study time, ability and
   environment; the claim can fail in the World (selection) and in the Model (a correlation read as an effect).
   Nonlinearity: grades are bounded, so compare the mean grade with the grade at the mean. Unit: the student, unless
   classes share a teacher. Noise: Gaussian with a ceiling; no absorbing state; students may change behaviour if
   told. Causal ladder: rung 1, observational; identification would need a random assignment of music, and the
   untestable premise is that no unmeasured trait drives both.
2. *(Concept)* The first, the value of information: no answer would change any decision of an insurer that will not
   change the product. "Pruned by Crisonomy: curiosity without operational use."
3. *(Count)* Cases grow like $\exp((\beta - \gamma)t)$, so two pairs with the same difference, such as $(2, 1)$ and
   $(3, 2)$, give the same curve for any $t$: the data see $\beta - \gamma = 1$ and cannot tell $R_0 = 2$ from $R_0 =
   1.5$. The ratio needs one of the two from elsewhere.
4. *(Laboratory)* Split the check into two: one for the terminal question mark, one for the literal action. The two
   planted defects then fall on different messages, and the table shows seven distinct messages for seven defects.
5. *(Laboratory)* Generate random arrival sequences and two service times $s_1 < s_2$; with the same arrivals, the
   Lindley recursion gives waits that are never negative and never smaller with $s_2$ than with $s_1$ at every
   position. A property test asserts both over thousands of seeds.
6. *(Concept)* A verifier that only checks that the chain is intact passes an intact chain that was rewritten in
   full, and it also passes a verifier that always returns true. G5 tampers with a record and requires the
   verification to fail, so it checks that the verifier can detect a change, not only that no change was detected.

## Chapter 18. The vault and the seal

1. *(Concept)* The vault is worth the surprise it holds: a model tested against data it never saw could have failed.
   Opened twice, the second model knows the vault's answer, so the test could no longer fail, and by Mayo's criterion
   its passing is no evidence.
2. *(Concept)* The clean reproduction compares the outputs of two runs byte for byte; a signature inside the result
   file changes the file after the run, so two honest runs would differ and determinism (G1) would fail. Signatures
   are stored apart, identified by the hash of the file they sign.
3. *(Count)* $K = 3$ counts the specifications tried per domain and stopped at three; $K = 38$ counts the distinct
   tests across all domains and rounds of the gate. Both are correct counts of different objects, and the claim was
   made at the level of the gate, so the correction uses 38.
4. *(Laboratory)* Hash `index|content|previous` and have the verifier check both that each hash matches and that the
   indices run 0, 1, 2, …; swapping two messages then breaks the index sequence even if the hashes are recomputed,
   and the verifier reports which position is out of order.
5. *(Laboratory)* Seven leaves give four hashes at the first level (the last repeated), two at the second, one root.
   Changing the last card recomputes its leaf, one hash at each level above it: three recomputations plus the root,
   not seven.
6. *(Concept)* A complete answer has three parts: the affected groups are heard and their transcripts, signed by
   them, are hashed into the record; the cases and the judges of any test come from outside the programme; and
   the opinion is signed by someone who does not decide whether the model is used.

## Chapter 19. Seven rounds in search of invariant causes

1. *(Concept)* In world B the environment adds its own shift to $Y$, so $Y \perp E \mid X_S$ fails for every $S$:
   no set makes the environment irrelevant, and the intersection of an empty family is empty. The year violates the
   premise that the environment intervenes only on the candidates, not on the outcome.
2. *(Concept)* A saturated set predicts the environment from its own columns, so conditioned on it the environment is
   constant. The invariance test then has nothing to reject and no power. Acceptance under zero power is not
   evidence that anything is invariant.
3. *(Count)* With 9,999 draws the floor is $1/10{,}000 = 0.0001$. With one row per mutant, every true effect and
   every artefact hits the same floor, so the $p$ cannot separate them. With two rows and a product, the floor is
   $10^{-3} \times 10^{-3} = 10^{-6}$, which is what the laboratory prints.
4. *(Laboratory)* With shifts $[0; 1; -0.5]$ the environment still acts on $Y$, but weakly; some sets may now pass by
   chance and the gate may pass or fail depending on the noise. The test's power to see a direct effect of the
   environment falls with the size of the shift, and a pass there is weak evidence.
5. *(Concept)* Lowering $\alpha$ makes the invariance test accept more sets, including in domains where every set was
   rejected at 0.05; some accepted sets then intersect in a non-empty $\hat S$ that did not exist before. The caveat
   is that the "invariant set" appeared because the test became more lenient, not because the data changed.
6. *(Count)* $10.9 - 9.6 = 1.3$ bits. At 12 bits the yeast fails and three areas remain, physics, pharmacology and
   education; the gate stands on them, and only until the clustering correction removes education.

## Chapter 20. The live clock

1. *(Concept)* A canary that gives no gain may be silent because there is no leakage or because the canary itself
   is broken; only the positive control, with the clock switched off, shows that the alarm rings when it should. A
   bad canary is a column that is knowable before the label, such as the scheduled departure time: it would give
   gain under both clocks and prove nothing.
2. *(Concept)* The circuit breaker compares the model's score with the reserve's; the model beats the climatology
   (skill $+0.08$), so the breaker stays shut while the forecasts miss the frequency by 0.035. A second criterion
   opens the breaker when the running ECE over the last $n$ cases exceeds its null quantile; its price is the number
   of times the official forecast switches to the reserve, which the chapter measured at 12% of a vault in Rhode
   Island.
3. *(Count)* $1 - 0.02130 / 0.02171 = 0.019$ against the fixed climatology. The measured $+0.0098$ uses the rate of
   the past, 0.0222, which is closer to the vault's 0.0220 and therefore a stronger reference; the skill is smaller
   against a better climatology.
4. *(Laboratory)* With the old intercept near the new rate, the raw model's level is already right, its bias and ECE
   fall near the Platt model's, and Platt changes little. Platt's value in this design is the level correction;
   where the level is right, it has nothing to repair.
5. *(Concept)* An underconfident model predicts too close to the middle: its calibration curve (observed frequency
   against forecast) is steeper than the diagonal, below it at low forecasts and above at high ones. Shifting the
   intercept moves the whole curve up or down and cannot change its slope; only a slope correction, as Platt's $b$,
   straightens it.
6. *(Concept)* Both metrics are computed correctly from the same confusion matrix; they measure different errors
   falling on different people. Choosing the false-positive rate makes those wrongly flagged the payers, and
   choosing the false-negative rate makes those wrongly missed the payers; with different base rates no threshold
   equalises both. The choice of which error to count is a choice of who bears it, which is a value, declared and not
   computed.

## Chapter 21. The seven lessons and the final project

1. *(Concept)* A complete answer names a person or body, copies an action literally and ends in a question mark:
   "The head of school decides; the action is moving the lunch break from 12:00 to 12:30; does moving the break
   reduce late arrivals to the first afternoon class?" With "the decider", nobody can be asked whether the question
   is theirs, and nobody owns the error if the answer is wrong.
2. *(Concept)* With a trigger: "the bus-stop change is reversed if boardings at the new stop stay below 80% of the
   old stop's for three months". Without: "while the new stop works well". The first survives a change of
   management because anyone can read the number; the second is whatever the new manager says it is.
3. *(Laboratory)* Thirteen cards, 1 accepted and 12 refused: the new card falls at R5, "no fixed numeric seed", and at
   no other rule, so the rejections per rule become [1, 1, 1, 4, 2, 1, 2].
4. *(Count)* In every 100 domains, about 15 change status or action when the prior changes. That is the size of what
   remains to be measured because those decisions depend on a number that no datum fixed and no expert has yet been
   asked for.
5. *(Concept)* Both arms found all eight planted defects and raised no false alarm, so the scores are equal at their
   maximum. An equal score could mean the methods are equally good or that the test was too easy to separate them,
   and the ceiling makes the second reading unavoidable. The thesis gains nothing because its prediction (bridge
   checking finds more) was not given a chance to fail; it loses nothing because it was not contradicted. A fair
   test needs defects hard enough that at least one arm misses some.
6. *(Essay)* One acceptable answer uses the three conditions of Chapter 21: the model declares its failure modes
   (the Erlang C card lists non-stationarity and abandonment, and the real rate gives 57% where the formula
   promised 82%); someone outside reproduces every number (the 39 cited blocks match their instruments, and a
   reimplementation from the text agrees on 570 of 573 quantities); and no decision depends on an unwritten
   assumption (the Danish analysis was rejected rather than rescued, with skill $-0.015$ against the climatology).
   The essay should end by saying which of the three the reader's own project has not yet met.
