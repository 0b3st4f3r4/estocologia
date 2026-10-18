# H1 in a real domain

This is the same framework as the simulated H1 (`../h1/`): bus with hash chain, Brier/Kelly journal, F1–F8, M15 and the
MB1–MB8 panel. The three simulated worlds gave way to real public datasets, described with hash in `../data/SOURCES.md`.

```
python3 rodar.py                    # ~3 min; writes saida/
python3 portoes.py --reproducao     # the 5 gates, with a second clean run
```

## Gate results

| Gate | Result |
|---|---|
| 1 · invariants and reproduction | **passes**: 40 properties; identical reproduction across the three projects |
| 2 · contract and data | **passes** |
| 3 · convergence, SBC and conformal | **passes**: R-hat ≤ 1.007; ESS ≥ 547; coverage 96% / 100% / 92% |
| 4 · temporal cut and skill | **fails on C**: skill −0.015 in the vault |
| 5 · governance and decision | **fails on C**: M15 failed it, so F8 issued no decision |
| human | the three risk opinions were signed by sil on 2026-10-15T03:53:28Z (`../governance/ASSINATURAS.md`, items 19 to 21) |

A and B were approved by M15. C failed. The tests were not adjusted to pass: C's failure is
the result of the project.

## A · Additive vs. multiplicative in the Dow Jones (1885–2023)

**Claim:** "the Dow yields μ per day; leveraged 3× (rebalanced daily), it yields 3μ, and whoever holds for decades always
wins."

**Data and unit:** the periods are 1885–1929, 1930–69, 1970–99 and the vault 2000–23. The unit is the calendar year: there are 115 years of
training, and 20-year cohorts are worth only about 5.8 independent blocks.

- **Temporal growth.** The arithmetic sum 3μ would give about 0.20 per year. The realized temporal growth is
  0.051 with 1×, 0.072 with 2× and 0.060 with 3×. The volatility drag at 3× is 0.14 per year.
- **Kelly.** L\* of the posterior is 2.35 [1.26; 3.44].
- **20-year cohorts above the starting point.** They are 96% with 1×, 78% with 2× and 64% with 3×. At 3×, mean wealth
  is 19× the initial capital, but the median is only 2.1×.
- **Absorbing days.**
  - 19/10/1987 (−22.6%) takes 68% of the capital at 3×; ruin starts at L = 4.4.
  - From 1929 to 1932, capital falls to 0.11 with 1× and to 0.00025 with 3×.
- **Severe test.** The claim's world predicts 93% of the 3× cohorts above the starting point; the real one gives 65%
  (p = 0.01).
- **What the claim keeps.** Without financing cost, 2–3× still grows more than 1× over time. The vault
  2000–23 favors the claim: the forecaster without drag had skill 0.44, against 0.40 for the model. The refutation holds
  for "3μ" and for "always wins", not for "leveraging never pays".

## B · LaLonde: job training, NSW against CPS

**Claim:** comparing NSW participants with CPS workers, raw or adjusted, recovers the effect of the training.

**Outcome and vault:** the outcome is being employed in 1978. The vault is the 260 randomized NSW controls. It is out of
sample by construction, not out of time.

| | employment ΔR | 1978 income (US$) |
|---|---|---|
| raw comparison | −0.107 [−0.170; −0.045] | −8,498 |
| propensity-score adjustment | +0.096 [−0.006; 0.208] | +1,570 [218; 3,265] |
| **experiment** | **+0.111 [0.026; 0.196]** | **+1,794** |

- **The adjustment gets the point right, but it is not severe.** An inert treatment would give +0.090 inside the range [−0.103;
  0.119] (p = 0.08). The adjustment leans on about 103 effective controls, and 72% of the CPS lies below the lowest
  score of the treated.
- **Placebo.** With 1975 employment as the outcome, the adjustment gives −0.123 (p = 0.04), against +0.085 in the experiment.
  This indicates selection through 1975 shocks.
- **Vault.** The model's skill is +0.18 (it predicted 0.655 employment and the observed value was 0.646). The logistic
  regression has +0.09, and the raw CPS counterfactual, −0.002.
- **Fairness.** False-negative rates differ by group: 0.72 for Hispanics, 0.50 for Blacks and 0.57 for
  others. Equalizing them catches 22 more unemployed and costs 87 false positives.
- **Privacy.** k-anonymity goes from 1 to 11 after generalization.

## C · The tail of the Danish fires (1980–1990)

**Claim:** at the end of 1988, a reinsurer sees 36 months without a claim in the "100 xs 50" layer (from 50 to 150 M DKK). It
concludes that the risk is negligible and writes 3× the capital.

**Data and unit:** the unit is the month. The 50 M DKK retention sits at the 99.7th percentile of losses, but it was chosen with the
whole series in view. Therefore the vault is not blind for the analyst, and there is a sensitivity table for retentions from
20 to 50.

- **Tail.** The Hill estimator gives α = 1.81 [1.41; 2.56] (block bootstrap). The GPD fit gives ξ = 0.41
  [0.02; 0.72], that is, variance close to infinite.
- **Months with the layer hit.** There were 4, 1 and 0 in each 36-month period, and 2 in the 24 vault months
  (152 M DKK in Aug/1989 and 145 M DKK in Oct/1990).
- **The clean window as evidence.** It is worth 2.2 bits. Laplace's rule gives 1/38 per month, not zero. A severe
  test would need 69 clean months.
- **Ruin.** With f = 3 (k/n) and with half Kelly, the reinsurer is ruined in Aug/1989. With Laplace it ends with
  0.19 of the capital; with POT, 0.78. The Kelly of the posterior is f\* = 0.46.
- **Why M15 failed it.** In the vault, no forecaster beats climatology: model −0.015, POT −0.003,
  Laplace −0.024 and the claim −0.052. The vault rate (1/12) came out above the training one, and the frequency of
  claims grows 4.6% per year (p = 2·10⁻⁶). The posterior is also dominated by the simulated elicited prior
  (posterior sd / prior sd = 0.87). The claim loses, but the model does not win the right to decide.

## Changes in the framework

- **Fix.** `Diario.placar(projeto=...)`: the score summed the same forecaster across the three projects. The simulated H1
  numbers were corrected in `../h1/README.md`: B becomes +0.014 and C +0.97, and the gates remain approved.
- **F5.** F5 now reads `dados_simulados`, `origem_dos_dados` and `tem_titulares` from the project. In A and C, which have no
  people, the LGPD is marked as not applicable, and k-anonymity stays informative (commercial secrecy).
- **Gate 4.** Gate 4 now accepts the invariance key of real B.

## Declared limits

- **Still simulated.** The SHELF elicitations (F7) remain simulated and labeled as such. The drift and
  pathological scenarios of the panel are declared perturbations of real rows.
- **No costs and no dividends.** The Dow is price only and there is no financing cost. Both absences favor the
  claim in A.
- **Hypergraph.** The hypergraph has 85 nodes, far from the target of about 10³.
