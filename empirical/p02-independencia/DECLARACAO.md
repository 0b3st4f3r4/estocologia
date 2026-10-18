# Declaration of background point 2 (independence): N-version reimplementation

Written on 2026-10-15, before writing any code in this folder and before any calculation on the data.
The SHA-256 of this file is in `DECLARACAO.sha256`.

## What was read before this declaration (the "contract")

Only specification documents and data headers: `h1-real/README.md`, `h1-real/saida/b/datacontract.yaml`,
`h2b/README.md`, `h2b/DECLARACAO_K2.md`, `h2c/README.md`, `h2d/README.md`, `h2d/DECLARACAO_K3.md`,
`h2d/dominios/educacao_star_turmas/pacote.yaml`, `h2g/README.md`, `h2g/DECLARACAO_H2g.md`,
`h2g/dominios/farmacologia_qt/pacote.yaml`, `h3/README.md`, `h3/CRITERIOS.md`, `h3/CRITERIOS_H3d.md`, the
`SOURCES.md` of `data/h2g` and `data/h3c`, the first line of each CSV used and the column dictionary of
the ECGRDVQ. No `.py` of `h1-real/`, `h2*/`, `h3/` and no output JSON was opened.

## Hypothesis

The published specification (README, declarations, criteria, YAML packages) is enough for an independent team,
without the code, to reach the same numbers that support the approved outputs, within the tolerances below.

## Results, published numbers and tolerances

### R1. H1·B LaLonde

| quantity | published | source | agreement tolerance |
|---|---|---|---|
| experiment, ΔR of employment (y = re78 > 0) | +0.111 [0.026; 0.196] | `h1-real/README.md:58` | ±0.001 at the point, ±0.005 at the limits |
| experiment, Δ income 1978 | +1,794 | `h1-real/README.md:58` | ±US$ 2 |
| raw NSW treated − CPS, ΔR | −0.107 [−0.170; −0.045] | `h1-real/README.md:56` | ±0.001 at the point, ±0.01 at the limits |
| raw, Δ income | −8,498 | `h1-real/README.md:56` | ±US$ 2 |
| propensity score, ΔR | +0.096 [−0.006; 0.208] | `h1-real/README.md:57` | ±0.03 at the point; my interval covers the experiment |
| propensity score, Δ income | +1,570 [218; 3,265] | `h1-real/README.md:57` | ±US$ 500 at the point |
| effective controls | ≈ 103 | `h1-real/README.md:61` | ±30% |
| fraction of the CPS below the lowest score of the treated | 72% | `h1-real/README.md:61` | ±10 p.p. |

For the experiment and the raw comparison, which are differences of means, the tolerance is almost exact. For the
propensity score, whose specification is not published, the tolerance is wide and the conclusion agreement is: the
adjustment lands within 0.03 of the experiment (the README says it "hits the point").

### R2. H2 Education STAR (K = 3, test K = 2 with saturation rule)

| quantity | published | source | tolerance |
|---|---|---|---|
| Ŝ | {anos_pequena_antes, turma_pequena} | `h2d/README.md:76` | exact equality of sets |
| accepted non-saturated / 1,024 | 57 | `h2d/README.md:76` | ±20% (46 to 68) |
| largest non-saturated p | 0.075 in {anos_pequena_antes, menina, mestrado_prof, prof_negro, tipo_escola, turma_pequena} | `h2d/README.md:76`, `:99` | p in [0.05; 0.10]; the set need not be the same |
| accepted saturated / saturated tested | 152 / 256 | `h2d/README.md:76` | ±20% |
| p of {turma_pequena} alone | 0.010 | `h2d/README.md:99` | rejected (p < 0.05) |

### R3. H2g hERG (farmacologia_qt)

| quantity | published | source | tolerance |
|---|---|---|---|
| Ŝ | {c_dof, c_qui, c_ran} | `h2g/README.md:6`, `:35` | exact equality |
| accepted / 512, saturated | 35, none saturated | `h2g/README.md:33` | ±20% (28 to 42); zero saturated |
| QTcF slope under dofetilide (posterior) | 25.2 ms per ng/mL, 90% CI [23.0; 27.5] | `h2g/README.md:38` | ±2.0 at the point and a 90% CI that overlaps |

### R4. H3d Rhode Island (`prisoes_ri`, vault 2013–2015)

| quantity | published | source | tolerance |
|---|---|---|---|
| n in the vault | 24,466 | `h3/README.md:227` | ±1% |
| ȳ in the vault | 2.26% | `h3/README.md:227` | ±0.05 p.p. |
| \|p̄ − ȳ\| official | 0.0020 | `h3/README.md:232` | passes or fails at both limits as published; ±0.003 |
| official ECE (10 equal-mass bins) | 0.0048 | `h3/README.md:232` | passes or fails at both limits as published; ±0.003 |
| official Brier skill | +0.019 | `h3/README.md:232` | same sign; ±0.01 |
| raw model: \|p̄ − ȳ\|, ECE, skill | 0.013; 0.013; +0.004 | `h3/README.md:233` | ±0.005; ±0.005; ±0.01 |
| Platt at the end | a = −0.30, b = 1.07 | `h3/README.md:236` | ±0.15 in each |

The main agreement of R4 is the criterion decision: (a) and (b) of `h3/CRITERIOS_H3d.md:55-64` pass in my
version as they passed in the published one.

## What counts as a divergence

Any quantity outside the tolerance declared above. Each divergence will be classified, only after freezing
my version and reading the original code, into one of three causes: (i) my error; (ii) the error of the original; (iii) a
choice not specified in the README or in the declarations (the specification is not enough to reproduce).

## Choices I make where the specification is silent (fixed now, before running)

These choices are mine and will not be changed after seeing results. A divergence caused by one of them is,
by definition, of type (iii) if the original choice is also not written down.

- **R1.** Vault and training as in the README: the score is fitted on NSW treated (185) + CPS (15,992), without the
  NSW controls. Logit with the Dehejia & Wahba covariates: age, age², educ, educ², married, no diploma, black,
  hispanic, re74, re75, u74, u75, educ·re74 (re in thousands). Estimating: effect on the treated with odds weights
  e/(1 − e) in the CPS. Interval: 95% percentile bootstrap, 2,000 resamples stratified by group, seed
  20261015. Effective controls: (Σw)²/Σw².
- **R2.** One row per student × grade in which the class is "small" or "regular" (without the arm with an aide) and there is a reading
  and a mathematics score. Y = reading + mathematics, standardized by mean and sd of the regular classes of the same
  grade. anos_pequena_antes = number of previous grades in a "small" class. almoco_gratuito = lunch "free".
  menina = gender "female". etnia = cauc, afam or other. mestrado_prof = teacher degree different from
  "bachelor". prof_negro = tethnicity "afam". tipo_escola = school. Rows with any candidate missing drop out.
  Environments: grade × class; vault 3_small outside the ICP. Test K = 2 and rule K = 3 exactly as in
  `h2b/DECLARACAO_K2.md` and `h2d/DECLARACAO_K3.md`. Numerics are standardized on the training set before the expansion.
- **R3.** As `h2g/DECLARACAO_H2g.md`: mean of the triplicate per volunteer × arm × time point; Y = mean QTcF of each
  ECG; placebo with all concentrations 0; dofetilide in pg/mL divided by 1,000; post-dose without a measurement drops out.
  The slope: OLS of Y on c_dof + adjustment (linear) on the training set, SE clustered by volunteer, combined with a
  normal prior of median 28 and sd (36 − 20)/1.349, by normal-normal conjugacy. 90% CI of the posterior.
- **R4.** Rows without `stop_outcome` drop out. Features: sex (M/F; missing as its own level), age standardized on the
  training set and its square (missing imputed by the training mean), `violation_raw` as indicators, 4 bands of hour
  (0–6, 6–12, 12–18, 18–24) and day of week. Logistic with λ = 1e-4 on the mean log-loss (free intercept),
  refitted on January 1st with all previous years. Continuous online Platt since 2006-01-01, with the label
  available 24 h after the stop and updates applied at the start of each 6 h tick, in order of
  availability. Circuit breaker: the parameters of the `prisoes` stream are not published, so **I do not implement the
  circuit breaker**; the fallback only enters outside the scope. Rolling climatology: mean of all labels available at the
  instant of emission. ECE: sorted predictions, 10 equal-mass bins, Σ (n_b/N)·|p̄_b − ȳ_b|.

## Rule

The version is frozen (hash of the `.py` files in `CONGELAMENTO.sha256`) before opening the original code. After that, the
code in this folder does not change; corrections of my own error, if any, enter as a separate version 2 and are reported.
