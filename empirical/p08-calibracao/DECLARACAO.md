# Point 8 (calibration): declaration before running

Written on 2026-10-15, before any analysis that produces the results below. The SHA-256 of this file is in
`DECLARACAO.sha256`. Changes after that enter as a new declaration and count in K.

**What was already visible before writing.** The H3 results published in `h3/README.md` and in `h3/saida/*.json`
(CITL and ECE of each vault, drift sensor alarms). Of the new flight data (NY 2017, Portland and Seattle
2014, Houston 2011) I saw only the schema: columns, types, number of rows, months and airports present. I computed
no delay rate, no mean and nothing per period. The 2013 deviation used as reference is the published one:
ȳ − p̄ = +0.026691 in the Oct–Dec 2013 vault (`h3/saida/portoes.json`, key `G2 ... voos.citl` = −0.026690800).

All the code runs on copies, inside `empirical/p08-calibracao/`. The vault journals are those of
`/mnt/project-files/estocologia/h3/saida/<fluxo>/diario.parquet`, checked against `FORA_DO_GIT.sha256`.
Fixed seeds with `np.random.default_rng`.

## (a) ECE tolerance that depends on n

**Hypothesis.** The fixed tolerance of 0.03 does not take into account the noise of the ECE, which depends on n and on the
distribution of the forecasts. In small vaults the noise can come close to 0.03; in large vaults it is much smaller, and 0.03 lets
real miscalibration through.

**Method.** For each binary H3 vault (flights 2013, cyclones, Toronto prisons 1997–2002, Toronto prisons
2020–2021, flights 2023, Rhode Island), I take the official forecasts p̂ of the vault (`calibracao.oficial_binario`, rows
with t_emissao ≥ cutoff) and simulate B = 2000 times y* ~ Bernoulli(p̂), independent, seed 20261015. In each
simulation I compute the ECE with the same function as H3 (`calibracao.ece`, 10 equal-mass bins). This gives the
distribution of the ECE under perfect calibration with the same forecasts and the same n.

**Declared criterion.**
- **Main rule (T95):** the vault passes on ECE if ECE_obs ≤ q95 of the null distribution. It is a test of perfect
  calibration with level 5%.
- **Secondary rule (T95+):** it passes if ECE_obs ≤ max(0.03; q95). It keeps the practical tolerance and only widens it when
  the noise goes beyond it.
- I also report q50, q99, the p-value (fraction of simulations with ECE* ≥ ECE_obs) and the ECE minus the null mean.
- The other G2 conditions (|p̄ − ȳ| ≤ 0.02, skill > 0, and in Rhode Island the relative criterion) do not change and are
  reported as they are.

**What counts as failure of the point.** If any vault approved in H3 (cyclones, flights 2023, Rhode Island) has
ECE_obs > q95, its approval does not hold under the T95 rule, and I say so. If the Toronto 1997–2002 vault falls
below q95, its failure was noise under the T95 rule.

**Declared limit.** The simulation assumes independent labels given the forecast. In the flights there are common shocks per day and
airport (design effect of 11.5 in H2), and so the independent null underestimates the real noise.

## (b) Label drift test

**Hypothesis.** The original sensor (KS and Wasserstein with hysteresis on an input and on the standardized residual)
compares distributions with a reference; it does not test whether the realized rate moves away from the predicted mean. A direct
label drift test would see the 2013 deviation earlier or more clearly.

**Two tests, run off-line over the H3 journals** (the temporal order of the replay is reconstructed: the forecast
exists at t_emissao and the label at t_rotulo). The window parameters come from the circuit breaker of each flow (window W and
interval m of `fluxos/*.py`), with no tuning.

- **R1, predicted mean against realized rate (rolling CITL, with window and hysteresis).** The resolved forecasts are
  ordered by t_rotulo. Every m resolutions, over the W most recent ones, I compute d = ȳ − p̄ (p̄ of the official forecast)
  and z = (Σy − Σp) / √Σp(1−p). It turns on when |d| > 0.02 and |z| > 3 in 3 consecutive checks. It turns off when
  |d| < 0.01 in 3 consecutive checks.
- **R2, BBSE (Lipton, Wang and Smola 2018), without label.** Every m emissions, at instant t of the last one: the source is the W
  most recent forecasts with t_rotulo ≤ t; the target is the W most recent emissions, without label. Hard classification
  ŷ = 1{p > π_fonte}, with π_fonte the rate of the source. With TPR and FPR of the source and μ = mean of ŷ in the target, the
  estimated target rate is q̂ = (μ − FPR)/(TPR − FPR), clipped to [0, 1]. The statistic is d_B = q̂ − p̄_alvo. It turns on when
  |d_B| > 0.02 in 3 consecutive checks and turns off when |d_B| < 0.01 in 3 consecutive ones. The BBSE hypothesis (p(x|y)
  fixed and fixed classifier) does not hold here, because the model readjusts; that is a declared limit.

**Flows:** the six binary ones (flights 2013, flights 2023, cyclones, prisons 1997–2002, prisons 2020–2021, Rhode Island).

**Measure.** For each flow: the date (and the month) of the first alarm in the vault and of each alarm in development, of R1,
R2 and of the original sensor (events `deriva` in `h3/saida/<fluxo>/resultados.json`). The alarms in development are the
price (alarms that did not result in vault failure).

**Decision question: "would the label test have seen 2013 earlier?"** Yes, if R1 or R2 turns on in the 2013 vault before
2013-10-16 (the first alarm of the original sensor in the vault, `residuo_padronizado`, t = 1381881600), or if it turns on in the
vault when the original does not. I also report whether it would turn on still in September (before the cutoff), when the correction
would have arrived in time. The tests only detect; I attach no action to them in this analysis.

## (c) The flights correction put to the test

The correction is the online recalibration of the intercept from H3b: p' = σ(logit p + δ), δ ← δ + 0.002 (y − p'). It was
declared before seeing any 2023 data, and it has no parameter tuned on data after 2013.

**Definition of "a deviation like that of 2013".** In a period, the raw model (without the correction; the fallback where the origin is not the
model) has |p̄ − ȳ| ≥ 0.02.

**(c1) New real data.** I found, on GitHub, two years of flights with the same schema and weather:
- **NY 2017**, package `nycflights` by jayleetx (github.com/jayleetx/nycflights), 303,748 flights from EWR, JFK and LGA;
- **Portland and Seattle 2014**, package `pnwflights14` by ismayc (github.com/ismayc/pnwflights14), 162,049 flights.

I run the `voos23` flow with no change in the model or in the correction, with a cutoff on October 1 and a start on January 22
of the respective year. The only adaptations are of reading: the file map, the conversion of local time to UTC
(America/New_York and America/Los_Angeles) and, in the northwest data, the airport list (PDX, SEA) in place of
(EWR, JFK, LGA) and the scheduled instant assembled from year, month, day, hour and minute. The `hflights` package (Houston
2011) stays out: it has no scheduled time for cancelled flights and no weather.

**Criterion (c1).** For each new year:
- **Main test:** if the raw model has |p̄ − ȳ| ≥ 0.02 in the Oct–Dec vault, the correction passes if the official forecast
  meets G2 (|p̄ − ȳ| ≤ 0.02, ECE ≤ 0.03, skill > 0). If the raw one does not have that deviation, the year does not test
  the correction, and I say so.
- **Secondary test:** in every calendar quarter, of 2017, 2014 and 2023, including those of development (which are also
  out of sample for the correction, declared beforehand), in which the raw one has |p̄ − ȳ| ≥ 0.02, the correction passes in the
  quarter if the official one has |p̄ − ȳ| ≤ 0.02. I report all quarters that meet the condition, not only the
  favourable ones. The 1st quarter starts on January 22.

**(c2) Stress test in the 2023 vault (it is stress, not proof).** In the `voos23` flow, only in flights with scheduled
departure ≥ 2023-10-01, I swap y = 0 for y = 1 with probability f, drawn per flight with seed 20261015, without
changing the instant of the label or any other table. This is pure label drift, with no signal in the features, the case
hardest for the model.
- **S1, step:** f = 0.026691 / (1 − ȳ₂₀₂₃), with ȳ₂₀₂₃ the rate observed in the 2023 vault (already opened in H3b), so that
  the expected rise of the rate is the 2013 deviation.
- **S2, 2013 profile:** one f per month (Oct, Nov, Dec), with the deviation ȳ − p̄ measured in each month of the 2013 vault in the
  journal `h3/saida/voos/diario.parquet` (official forecast), divided by (1 − ȳ of the month in 2023). If the deviation of a
  month is negative, f = 0 in that month.

I do the full replay (engine, circuit breaker, sensors, readjustments) over the copies. **Criterion (c2):** the correction
withstands the stress if the official forecast meets G2 in the stressed vault (|p̄ − ȳ| ≤ 0.02, ECE ≤ 0.03, skill
> 0). I report the raw alongside; if the raw one also passes, the stress was not strong enough to discriminate, and I say
so.

**(c3) Informative: 2013 retrospective off-line.** I apply the recursion of δ over the 2013 journal, in the order
of the replay, keeping the origins (model or fallback) of the journal. Before that, I validate this off-line replica against the δ
recorded in the 2023 journal. The correction was designed after seeing 2013, so this is not a test.

## What counts in the K of this point

This is declaration 1. The replays of items (c1) and (c2) are 4 (2017, 2014, S1, S2). Reading corrections that do not
change forecasts will be reported; any change of criterion after seeing a result enters as a new declaration.
