# Declaration of point 5 (priors without a specialist), written before rerunning

Date: 2026-10-15. Written after assembling the alternative priors (`prioris.py`, `prioris/*/*.yaml`) and before
any execution of the funnel or of the cycle with them. No execution of the copies was made up to here.

## Limit that is not resolved from here

No human specialist is available. No prior of this round is a real SHELF elicitation: the literature priors
are readings of published sources, checked online, converted to the quantity of the domain by
fixed rules written below. This tests whether the conclusions depended on the simulated priors; it does not replace a panel.

## Question and criterion

**Question:** do the F8 decision and the status of each domain change with the literature prior, with the weak prior, or with
neither of the two?

**Scope.** All the Bayesian analyses with a SHELF prior of the key:
- H2 (12 domains, `h2/`) and H2g (18 domains, `h2g/`). The domains of H2b to H2f have the same priors and decisions
  as the final versions in `h2g/` (checked: quartiles, `taxa_base` and `decisao` identical), so H2g covers them.
- Real H1 (projects a, b, c, `h1-real/`), with the simulated SHELF panel swapped.
- H3: does not use a SHELF prior (`estat.py` has the function, but no flow calls it). It stays out, and that is said.

**Three rounds per horizon**, from copies in `empirical/p05-prioris/` (the originals are not edited); the only
difference between them is the table indicated in `$P05_PRIORI`:
- (a) original: no table (the priors of the packages). It also serves to check that the copy reproduces the original.
- (b) literature: `prioris/<h>/literatura.yaml`.
- (c) weak: `prioris/<h>/fraca.yaml`.
Seeds, data, code, decisions (costs, benefits) and `taxa_base` stay equal.

**Rules of the priors (fixed before running):**
- R1: source with estimate e and standard error s: median e, sd = 2s (transport factor 2 between study and domain).
- R2: source with range [a, b]: median (a + b)/2, quartiles a and b.
- R3: source with a single point: median e, sd = |e|/2.
- Binomials: the computation is done on the log of the odds ratio.
- No verified source: the literature prior is the weak prior, and that is written down.
- Weak: center 0 (odds ratio 1), sd = 10 times the sd of the original prior. In real H1, Beta(1, 1).
- Sources from the same database as the domain are used, but marked "same base" (double counting).

**What is measured, per domain and round:** status (category: pruned at F1/F2/F3, failed at F4 with the list of the
gates that failed, approved), F8 action (adopt/keep, or the equivalent action in real H1) and P(adopting pays off),
mean and 90% CI of the key posterior, posterior sd / prior sd ratio. The sensitivity is the shift of the posterior
mean in units of the posterior sd of the round (a): |m_b − m_a| / sd_a and |m_c − m_a| / sd_a.

**Change.** A domain "changes" in a round if the status category changes (including the list of gates that fail)
or if the F8 action changes (including going from an emitted decision to none, or the opposite). Changes whose only reason
is a computational gate (R-hat, ESS, SBC) are counted as a change and also listed separately.

**Verdict (fixed rule):** let N_b and N_c be the numbers of domains that change in (b) and in (c), over the 33 (12 + 18 + 3).
- N_b = 0 and N_c = 0: the conclusions did not depend on the simulated priors.
- N_b = 0 and N_c > 0: did not depend on the simulated priors in the content, but did depend on there being some informative
  prior in the domains that change in (c).
- N_b > 0: depended on the simulated priors in the domains that change in (b); the verdict names each one and says whether the source
  of (b) was independent or from the same base.
It is also said whether the G3 of H2g (ICP with Ŝ ≠ ∅ in at least 3 areas) changes; the ICP does not use the prior, so it would only change if
a domain were pruned before F4.

**What counts as a failure of this attempt:** the copy not reproducing the original round (status or posterior different
from the original `saida/` in (a)). In that case rounds (b) and (c) are not interpreted.
