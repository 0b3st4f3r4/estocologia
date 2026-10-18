# Point 8: calibration (ECE tolerance, label drift, flights correction)

## Result

**(a) With an ECE tolerance that depends on n, no binary H3 vault passes, not even the approved ones.** By the
declared rule (observed ECE ≤ q95 of the ECE under perfect calibration, with the same forecasts and the same n), cyclones, flights
2023 and Rhode Island stop passing: their ECE is 1.4 to 2.3 times the null q95, with p-value ≤ 0.0005. The failure
of Toronto 1997–2002 was not noise either: the ECE of 0.036 lies above the null q95 of 0.032 (p-value 0.008). By the secondary
rule, max(0.03; q95), the H3 picture stays the same: the three approved ones remain approved and the two Toronto
ones remain failed. The lesson is that 0.03 is not "the noise": in the large vaults the noise is 0.002 to 0.006, and 0.03 is a
practical tolerance that lets detectable miscalibration through. The null assumes independent labels, which underestimates the
noise in the flights (shocks by day and airport).

**(b) The label drift tests would not have seen 2013 earlier in a useful way.** By the declared criterion (a new alarm
in the vault before 2013-10-16, the date of the first alarm of the original sensor), the answer is no: the test of predicted mean
against realized rate (R1) turns on in the vault only on 2013-11-06, and the BBSE (R2) on 2013-11-21. But R1 was already
on since 2013-07-22 and stayed on at the crossing of the cutoff; the problem is that it is on in 95% of the
2013 development checks and in 85% of the 2023 ones. With the circuit breaker window (3,000 flights, some three days),
the delay rate swings much more than 0.02 from one day to the next, and the test becomes a permanent alarm that does not separate
the 4th quarter of 2013 from any other period. In the prisons and cyclones flows R1 is on in 0% to 19% of the
vault checks. The BBSE is on in 99% to 100% of the vault in the flights, in 71% to 82% in Rhode Island and in Toronto
2021 and in about 20% in the others: its hypothesis (fixed classifier) does not hold with the model readjusted every day.

**(c) The correction withstood the only real deviation I found, but there was no proof in the vault, and the stress did not discriminate.**
I found two new years of flights with the same schema: NY 2017 and Portland/Seattle 2014. In neither of the two did the 4th quarter
repeat the 2013 deviation in the raw model (|p̄ − ȳ| of 0.008 and 0.018, below 0.02), so the main test does not
apply; the official forecast meets G2 in both vaults. In the secondary test, the only quarter in which the raw model
deviated as in 2013 was the 3rd quarter of 2017 (raw +0.028), and the correction took it to +0.001. In the declared stress
(injecting into the 2023 vault the label drift of 2013), the correction passes, but the raw model also passes, because the
daily readjustment over 60 days absorbs the drift on its own at the quarter level. By the declared rule, the stress was not
strong enough to discriminate. This is stress, not proof.

## What was done

The declaration was written before any analysis: [DECLARACAO.md](DECLARACAO.md), SHA-256 in `DECLARACAO.sha256`
(13dbcd38…). All the code runs on copies of H3 in `codigo/` (copied from `h3/estocologia`, `h3/fluxos` and
`h3/rodar.py`); the originals were not edited. The vault journals come from
`/mnt/project-files/estocologia/h3/saida/<fluxo>/diario.parquet`, and the code checks each one against
`FORA_DO_GIT.sha256` before reading.

Files:
- `ece_n.py`, output `saida/ece_n.json` (item a);
- `rotulo.py`, outputs `saida/rotulo.json` and `saida/r1_serie_*.csv`, `saida/r2_serie_*.csv` (item b);
- `preparar_dados.py` (conversion of the new flights), `parametros_estresse.py` (output `saida/parametros_estresse.json`),
  `codigo/fluxos/voos_p08.py` (new flows), `correcao.py`, output `saida/correcao.json` (item c);
- `saida/replays/<fluxo>/`: results, bus and specifications of the 4 replays. The journals (> 2 MB) are in
  `/mnt/project-files/estocologia/data/empirical/p08-calibracao/replays/`, with sums in the `SHA256SUMS` there.

## (a) ECE tolerance that depends on n

For each vault, 2,000 simulations of y* ~ Bernoulli(p̂) with the official forecasts of the vault, seed 20261015, and the ECE
computed with the same function as H3 (`calibracao.ece`, 10 equal-mass bins). The observed ECEs match the ones
published in H3. Source: `saida/ece_n.json`.

| vault | n | observed ECE | null q50 | null q95 | null q99 | p-value | 0.03 (H3) | T95 (main) | max(0.03; q95) |
|---|---|---|---|---|---|---|---|---|---|
| flights 2013 (NY) | 84,301 | 0.0267 | 0.0032 | 0.0046 | 0.0052 | < 0.0005 | passes | **fails** | passes |
| cyclones | 5,766 | 0.0123 | 0.0058 | 0.0087 | 0.0099 | 0.0005 | passes | **fails** | passes |
| Toronto prisons 1997–2002 | 1,488 | 0.0363 | 0.0225 | 0.0320 | 0.0359 | 0.008 | fails | **fails** | fails |
| Toronto prisons 2020–2021 | 16,933 | 0.0668 | 0.0080 | 0.0115 | 0.0132 | < 0.0005 | fails | **fails** | fails |
| flights 2023 (NY) | 104,489 | 0.0079 | 0.0025 | 0.0035 | 0.0039 | < 0.0005 | passes | **fails** | passes |
| Rhode Island | 24,466 | 0.0048 | 0.0023 | 0.0034 | 0.0038 | < 0.0005 | passes | **fails** | passes |

"< 0.0005" means that none of the 2,000 simulations reached the observed ECE. The flights 2013 were already failing G2 on
|p̄ − ȳ|; here the "0.03" column looks only at the ECE.

**Verdict (a).** The vaults approved in H3 (cyclones, flights 2023, Rhode Island) **do not** remain approved by the
declared main rule: their calibration is not perfect, and with n from 5 to 100 thousand the difference is detectable. By the
secondary rule, they remain. The Toronto 1997–2002 one had noise of the order of the tolerance, as H3 said (null q95
0.032), but the observed ECE goes beyond it: the failure holds under both rules. The ECE minus the null mean (an ECE
"without the noise bias") goes from 0.0024 (Rhode Island) to 0.0587 (Toronto 2021), in `saida/ece_n.json`.

## (b) Label drift test

Two tests run off-line over the journals, in the order of the replay, with the window W and the interval m of the circuit breaker of
each flow (flights 3,000/300, cyclones 1,500/100, prisons 400/50):
- **R1:** d = ȳ − p̄ in the last W resolved forecasts; turns on with |d| > 0.02 and |z| > 3 in 3 consecutive checks,
  turns off with |d| < 0.01 in 3 consecutive ones;
- **R2 (BBSE, Lipton, Wang and Smola 2018):** target rate estimated without label, q̂ = (μ − FPR)/(TPR − FPR), against the
  predicted mean in the target; same hysteresis.

Source: `saida/rotulo.json`; the alarms of the original sensor come from `h3/saida/<fluxo>/resultados.json`.

| flow | original: dev / vault alarms (1st in the vault) | R1: dev / vault (1st in the vault) | R1 on at the cutoff | R1 % on dev / vault | R2: dev / vault (1st in the vault) | R2 % on dev / vault |
|---|---|---|---|---|---|---|
| flights 2013 | 0 / 1 (2013-10-16) | 7 / 3 (2013-11-06) | yes, since 2013-07-22 | 95% / 96% | 2 / 1 (2013-11-21) | 97% / 99% |
| flights 2023 | 4 / 1 (2023-11-14) | 21 / 5 (2023-10-13) | yes | 85% / 75% | 1 / 0 | 100% / 100% |
| cyclones | 1 / 0 | 2 / 0 | yes | 41% / 19% | 2 / 0 | 41% / 21% |
| prisons 1997–2002 | 1 / 1 (2001-04-05) | 0 / 0 | no | 0% / 0% | 2 / 0 | 36% / 20% |
| prisons 2020–2021 | 3 / 0 | 4 / 0 | no | 16% / 0% | 9 / 6 (2021-07-06) | 87% / 82% |
| Rhode Island | 0 / 1 (2013-02-03) | 3 / 1 (2013-05-27) | no | 6% / 1% | 13 / 6 (2013-06-01) | 74% / 71% |

R2 had degenerate checks (TPR − FPR < 0.05 or a window without one of the labels), which were skipped: 28 (flights 2013),
53 (flights 2023), 0, 0, 54 and 150. This rule of skipping was not in the declaration; it is reported here.

**Verdict (b).** By the declared criterion, **no**: neither of the two tests turns on a new alarm in the 2013 vault before
the original sensor (2013-10-16). R1 was on in September and at the crossing of the cutoff, with d = +0.040 when it turned on on
2013-07-22, which the declaration foresaw reporting. But it is on almost all the time in the flights (95% of the checks
of the 2013 development), and therefore it is not a warning: with the window of 3,000 flights, it measures the variation of one day of
bad weather, not a quarter drift. The 2013 deviation was also not a step: by month, the predicted mean minus the
realized rate (official forecast, `saida/correcao.json`, `retro_2013.meses`, column `citl_cru`) was −0.014 in
October, +0.001 in November and −0.067 in December. Almost all of the deviation of the quarter came from December. A test with a
three-day window does not separate that from the noise of August (+0.046) or of September (+0.070), which would have failed nothing.

On the premise of the point: the original sensor has a standardized residual channel, (y − p)/√p(1−p), compared by KS to
a reference. That channel reacts to the label, and it was the one that fired in the 2013 and 2023 flights. What is missing in it is comparing
the rate with the predicted mean; R1 does that and, with the circuit breaker window, did not help.

## (c) The flights correction put to the test

The correction is the H3b one: p' = σ(logit p + δ), δ ← δ + 0.002 (y − p'). The new flows inherit `Voos23` without changing model,
features, fallback, circuit breaker, sensors or correction.

**New data found on the network** (`/mnt/project-files/estocologia/data/empirical/p08-calibracao/FONTES.md`):
- NY 2017, package `nycflights` (github.com/jayleetx/nycflights): 303,748 flights from EWR, JFK and LGA, with hourly
  weather;
- Portland and Seattle 2014, package `pnwflights14` (github.com/ismayc/pnwflights14): 162,049 flights from PDX and SEA, with
  hourly weather;
- Houston 2011, package `hflights` (mirror cran/hflights): downloaded and not used, because it has no scheduled time for the
  cancelled ones and no weather.

I did not find `nycflights14` nor other NY years on GitHub; `anyflights` only downloads from the BTS, which is blocked.

**Reading correction in the northwest data, made before running.** The declaration says that the scheduled instant would be
assembled from year, month, day, hour and minute. On converting I saw that, in `pnwflights14`, hour and minute are those of the actual
departure. The scheduled instant became (actual departure − dep_delay) mod 24 h on the date of the flight. The 857 cancelled flights
(0.53%) have no time at all and left; since cancelled counts as delayed, the base rate of the northwest ends up a little
lower than it would be. In NY 2017 the weather has `precip` empty in many hours; the flow treats empty as 0.

### New vaults (main test)

Source: `saida/correcao.json`. CITL = p̄ − ȳ (the negative sign means more delays than predicted, as in 2013).

| Oct–Dec vault | n | ȳ | official: CITL | official: ECE | official: skill | raw: CITL | deviation like 2013 in the raw? | G2 of the official |
|---|---|---|---|---|---|---|---|---|
| NY 2017 | 75,166 | 0.197 | −0.0091 | 0.022 | +0.107 | −0.0081 | no | passes |
| PDX/SEA 2014 | 38,608 | 0.161 | +0.0004 | 0.016 | +0.053 | −0.0176 | no | passes |
| NY 2023 (H3b) | 104,489 | 0.130 | −0.0015 | 0.008 | +0.149 | +0.0121 | no | passes |

No new vault repeated the 2013 deviation in the raw model, so **the main test does not apply** to any year.

### Quarters of 2014, 2017 and 2023 (secondary test)

In 12 quarters, only one had the raw one with |CITL| ≥ 0.02: **NY 2017, 3rd quarter**, raw +0.0277, official +0.0012
(passes). The 3rd quarter of 2023 came close (raw +0.0192, official +0.0053). All 12 have the official one with
|CITL| ≤ 0.02. The complete table is in `saida/correcao.json` (`trimestres`).

**Informative, by month (it was not a criterion).** The December months repeat the December 2013 deviation (raw −0.067)
in the three years:
- NY 2017: raw −0.050, official −0.024, still above 0.02;
- PDX/SEA 2014: raw −0.051, official +0.011;
- NY 2023: raw −0.011, official −0.001.

Other months with the raw one above 0.04: 2017-09 (+0.043, official −0.017), 2023-05 (+0.046, official +0.008), 2023-06
(−0.059, official −0.001), 2023-08 (+0.075, official +0.008).

### Stress test in the 2023 vault (it is stress, not proof)

Pure label drift, only in flights with scheduled departure from 2023-10-01: swapping y = 0 for y = 1 with
probability f, seed 20261015 (`saida/parametros_estresse.json`). The 2013 deviation comes from the 2013 journal:
ȳ − p̄ = +0.026691, the same published in H3.
- **S1, step:** f = 0.0307 in the whole vault; 2,785 labels swapped out of 104,570 flights.
- **S2, 2013 profile:** f = 0.0165 in October, 0 in November (the November 2013 deviation was negative) and 0.0777 in
  December; 2,677 swapped.

| stressed vault | ȳ | official: CITL | official: ECE | official: skill | raw: CITL | raw: ECE | official G2 | raw G2 |
|---|---|---|---|---|---|---|---|---|
| S1 | 0.157 | −0.0018 | 0.014 | +0.093 | +0.0046 | 0.013 | passes | passes |
| S2 | 0.156 | −0.0029 | 0.015 | +0.104 | −0.0030 | 0.012 | passes | passes |

In S2, in December, the raw one stays at −0.055 (close to the −0.067 of December 2013) and the official one at −0.006. In the whole
quarter, the daily readjustment of the model over 60 days already learns the new rate, and the raw one passes on its own.

**Control.** The development part of the two stress replays (307,246 forecasts before the cutoff) is identical to the
H3b journal in all forecast columns (modelo, modelo_cru, delta, origem). Only 2 labels differ in S1, from flights
emitted before the cutoff and scheduled after it. The replays ran with G3 clean (0 readings of the future, 0 labels
already known, 0 states of the future) and at most 2.14 scans per table, except PDX/SEA 2014, with 3.13 scans
in the worst table, above the G5 limit of 3. This does not affect the forecasts. The K of each new flow is 1.

### Retrospective of 2013 (informative; the correction was designed after seeing 2013)

First I validated the off-line replica of the recursion of δ against the 2023 journal: the maximum difference of δ is 0.0031,
the mean is 2.7e-5, and the maximum difference of the forecast is 0.00076. Applied to 2013, with the origins of the journal kept, the
correction would take the vault from CITL −0.0267 to −0.0024 and the ECE from 0.0267 to 0.0238, passing G2. December would go
from −0.067 to −0.001.

**Verdict (c).** The correction has not yet been put to the test in a vault that repeated 2013, because neither of the two new
years repeated the deviation in the 4th quarter. In the quarters and months that repeated it, it held the quarter (2017-Q3) and
most of the Decembers, but not December 2017 (−0.024). The declared stress does not discriminate: the raw model also
passes.

## A divergence in H3

`h3/README.md` says that in H3b "the δ varied from −0.52 to +0.43 over the year". In the 2023 journal, that is the interval
inside the vault; over the whole year, the δ goes from −1.01 to +1.25.

## How to reproduce

From `empirical/p08-calibracao/`, with pyarrow, duckdb and pyreadr installed:

```
python3 ece_n.py                 # (a), seconds
python3 rotulo.py                # (b), seconds
D=/mnt/project-files/estocologia/data/empirical/p08-calibracao
python3 -I preparar_dados.py $D $D/csv
python3 parametros_estresse.py
# data folder with h3b -> data/h3b and p08 -> $D/csv
mkdir -p /tmp/dados && ln -sfn /mnt/project-files/estocologia/data/h3b /tmp/dados/h3b && ln -sfn $D/csv /tmp/dados/p08
cd codigo && for n in voos17 voospnw14 voos23e1 voos23e2; do \
  ESTOCOLOGIA_DADOS=/tmp/dados ESTOCOLOGIA_SAIDA=../saida/replays python3 -W ignore rodar.py $n & done; wait; cd ..
python3 correcao.py              # (c)
```

The four replays in parallel take about 2 h on 4 cores.

**Reproduction done.** `ece_n.py` and `rotulo.py` ran twice, and the JSONs have the same SHA-256. The second execution
of the four replays is reported in "Second execution", below.

## Limits

- The null of the ECE assumes independent labels. In the flights there are common shocks per day and airport, and the null underestimates the
  noise. The T95 rule is, therefore, too severe in the flights. A block null would require a model of the dependence, which
  I did not do.
- R1 and R2 ran off-line and only detect: I attached no action to them. The thresholds (0.02 and 0.01) and the windows
  (those of the circuit breaker) were declared and not tuned. A window of one month might have separated December 2013, but that
  would be chosen after seeing the result, and I did not run it.
- The BBSE assumes p(x|y) fixed and the classifier fixed; here the model readjusts every day.
- The new years are not the same airport in all cases (PDX/SEA), and the northwest lost the cancelled ones.
- The stress is a pure label drift, with no signal in the features. It is the hardest case for the model and the easiest
  for an intercept correction, and even so the daily readjustment absorbed it in the quarter.
