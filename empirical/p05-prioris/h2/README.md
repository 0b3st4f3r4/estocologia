# H2: cross-domain transfer

**H2 output: NOT approved.** The reproduction, funnel, scan containment and track gates pass. The gate that
PLAN chose for H2, the Invariant Causal Prediction (ICP) between environments, fails: the criterion demanded Ŝ ≠ ∅ in at
least 3 domains of distinct areas, and only 1 (law) delivered.

There are 12 real domains, plugged in through the F7 standardized package (`dominios/<nome>/pacote.yaml` + `preparo.py`). The data
are in `../data/h2/` (Rdatasets, with SHA-256).

```
python3 rodar.py                    # funnel on the 12 domains (~4 min, the logistic one alone takes ~3)
python3 portoes.py --reproducao     # gates G1–G4, with a second clean run
python3 grafo.py                    # hypergraph: 1 538 nodes, 1 700 tree edges, 62 bridges, 630 hyperedges
```

## What was built

- **Plug-and-play domain package.** It is the F7 contract: substantive question, claim, glossary of 10 terms,
  sources, independent unit, environments and vault, DAG, adjustment set, SHELF elicitation, decision and scope. The
  validator refuses the incomplete package before any data read. Ten of the twelve packages were written by
  agents that only read the contract and two examples: it is the pluggability test.
- **4-filter funnel** (PLAN §2):
  - **F1:** EVPI under the prior against the cost of the research.
  - **F2:** back door in the DAG and informativeness in the unit.
  - **F3:** permutation null at the level at which the key was assigned.
  - **F4:** ICP, MCMC, SBC, pre-registration, vault from another environment, conformal, M15 and F8.
- **Adaptive sampler.** Haario Metropolis with the proposal frozen after warm-up, mixed with an independent
  t₅ step centered on the MAP with the numerical Hessian. R-hat stayed below 1.008 and ESS above 900 in the
  11 domains, with dimension up to 15. H1's fixed step failed in about 40% of seeds with d ≥ 5.
- **DuckDB warehouse.**
  - It reads Parquet partitioned by environment.
  - It refuses `SELECT *`.
  - It checks the projection in the `EXPLAIN` of every read.
  - It charges the bytes to a per-agent budget.
  - It keeps the vault locked until F3 presents the pre-registration hash.

## Results by domain

| domain | key → y | vault | n / units (design effect) | effect (90% CI) | ICP | vault skill | status |
|---|---|---|---|---|---|---|---|
| education (STAR) | small class → score | urban schools | 5,250 / 72 schools (3.3) | +13.5 points [7.5; 19.5] | rejected | 0.17 | approved, adopt (P = 0.84) |
| logistics (NY flights) | scheduled hour → delay | LaGuardia | 225,753 / 364 days (11.5) | +2.3 p.p. per hour [2.2; 2.4] | rejected | 0.09 | approved, adopt (P = 1.00); K = 2 |
| agronomy (maize, Argentina) | N → yield | lowland | 2,558 / 36 strips (1.4) | 0.059 q/ha per kg [0.046; 0.074] | rejected | 0.55 | approved, adopt (P = 0.86) |
| climate (cyclones) | √(1013 − p) → wind | Gulf of Mexico | 17,597 / 563 storms (22.8) | 11.8 knots [11.5; 12.1] | rejected | 0.79 | approved, keep (P = 0.14) |
| credit (HMDA Boston) | Black applicant → denial | high unemployment | 1,757 / 1,757 (1.0) | +6.7 p.p. [4.0; 9.3] | empty accepted | 0.16 | approved, adopt (P = 0.95) |
| development (Preston) | log GDP → life expectancy | 1997–2007 | 1,278 / 142 countries (9.8) | +5.1 years [4.1; 6.0] | rejected | 0.82 | approved, adopt (P = 0.57) |
| energy (Victoria) | cooling degree-days → demand | 2014 | 730 / 105 weeks (1.6) | 126 MW [118; 134] | rejected | 0.88 | approved, adopt (P = 1.00) |
| epidemiology (Bangladesh) | having children → contraception | g4 districts | 1,456 / 45 districts (1.5) | +21.7 p.p. [14.4; 29.2] | Ŝ = ∅ (disjoint accepted) | 0.04 | approved, adopt (P = 0.64) |
| health (NMES 1988) | private insurance → visits | West | 3,608 / 3,608 (1.1) | +0.31 log-visits [0.24; 0.37] | empty accepted | 0.14 | approved, adopt (P = 1.00) |
| law (Toronto) | being Black → release | 2002 | 4,949 / 4,949 (1.0) | −10.3 p.p. [−12.1; −8.5] | **Ŝ = {cidadão, empregado}** | −0.01 | failed: loses to climatology |
| labor (résumés) | Afro name → callback | Boston | 2,704 / 426 ads (0.7) | −2.3 p.p. [−3.5; −1.1] | empty accepted | 0.02 | failed: conformal coverage 0.79 |
| traffic (beer tax) | tax → deaths | Northeast | 273 / 39 states (2.4) | — | — | — | pruned at F3 (p = 0.41) |

## What H2 teaches

1. **ICP almost never finds an invariant set in these data.** In 6 domains it rejected every set: the
   available environments (school type, airport, relief, region of genesis, year) act directly on Y, or there is a hidden
   parent, or the relation is not linear at large n. In another 4 it accepted so much that Ŝ came out empty, because the environments
   move X very little. Only in law did Ŝ come out non-empty, and it came out against the claim: skin color is not in Ŝ; citizenship
   and employment are. At 95% coverage, this says that the invariant parents of release include citizenship and employment.
   It does not say that color does not matter, because ICP only guarantees Ŝ ⊆ parents.
2. **The effect transports better than the set.** Even with ICP rejected, the adjusted model predicted the never-seen
   environment well in 9 domains (skill from 0.04 to 0.88). The claim "holds in any school, airport or region"
   falls in the strong form (the conditional distribution changes), but the direction and the size of the key survive. For deciding,
   it is this weak version that matters, and the PLAN asked for the strong one.
3. **Pseudoreplication appears everywhere.** The design effect reaches 11.5 in the flights (225 thousand flights are worth 364
   days), 22.8 in the cyclones and 9.8 in Preston. The likelihood tempered by 1/design effect widens the posterior
   accordingly.
4. **The literature errs on both sides.**
   - In the flights, the prior (odds ratio of 1.08 per hour) came out below the data (1.14).
   - In the cyclones, the prior of 15 knots/√hPa came out above 11.8.
   - In the résumés, the prior of "one third less" became −2.3 p.p.
5. **The funnel pruned for the right reasons.**
   - F2 blocked the first specification of the flights: the adjustment included temperature, a descendant of the hour in the DAG
     itself. The package was fixed, and K = 2 was recorded.
   - F3 pruned the beer tax: with state fixed effect, the tax does not distinguish itself from the null.

## Changes after seeing results (the program's K)

- **Framework v2.** The audit of the domain agents found four flaws, fixed for all domains at
  once:
  - **F3 null.** It now permutes at the level at which the key varies: between units, within the unit when the
    draw is declared, or means and deviations when the key is observational.
  - **Likelihood tempered by 1/design effect.** The iid posterior was too narrow.
  - **Conformal by left-out environment.** Calibration on a random fifth of the training set did not carry the change
    between environments.
  - **ICP labels.** "Rejected" is now distinguished from "empty accepted".

  v2 changed the status of 4 domains. With v1, energy and cyclones fell into conformal coverage, credit into R-hat
  (1.011) and Preston into F3. The numbers above are from v2.
- **G2 criterion.** The declared criterion was "error < 1e-6 in the SHELF fit", impossible for asymmetric quantiles
  (odds ratio in log). On the first run, G2 failed only for that. The criterion became 10% of the adjusted sd;
  the largest error is 5%. The change is recorded in `portoes.py` and does not touch G3, which decides the output.

## Declared limits

- **Elicitation.** No human specialist was consulted. All SHELF priors come from literature quoted from
  memory and marked "to verify".
- **Costs.** Benefits and costs of the decisions are assumptions in the packages.
- **Clustering.** In 3 domains (credit, law, health) the dataset has no clustering identifier, so the
  unit is the row and the standard error may be optimistic. F2 flags these cases.
- **ICP power.** ICP with only 2 training environments (résumés) has little power.
- **M15 thresholds.** The M15 thresholds (R-hat < 1.01, coverage ≥ 0.85) are hard cuts.
- **Scale.** The hypergraph has 1,538 nodes, against the PLAN's target of 10⁴–10⁵. The data add up to 1.6 MB scanned, far
  from "Parquet databases" at scale.
- **Opinions.** The 12 risk opinions await sil's signature.

## The next step to pass the gate

The PLAN criterion demands environments that intervene on X without acting on Y. That requires data with known interventions:
several lottery draws, rule changes, or multicenter experiments. Convenience environments (year, region) do not
serve. There are two exits:

- **Change the environments.** One must look for datasets like this, and several are outside the reach of this environment's network.
- **Revise the criterion.** Accepting the weak form, invariance of the key's effect with a free intercept per environment, is
  a decision of yours about what H2 should demonstrate.
