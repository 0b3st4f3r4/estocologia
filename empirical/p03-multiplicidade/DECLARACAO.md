# Declaration: points 3 and 4 (uncounted multiplicity and adjustments after seeing results)

Written on 2026-10-15, before calculating any corrected p-value and before running any copy of a horizon.
The SHA-256 of this file and the UTC time are in `DECLARACAO.sha256`.

## What I already knew when writing

I read the READMEs, the declarations and the criteria of h2 … h2g and h3, and I listed, from `h2*/saida/*/resultados.json`,
which domains reached F4 in each round, with which α, which test and which Ŝ. I therefore know the published Ŝ and the
"largest p" cited in the READMEs (for example, 0.075 in the STAR of H2d; 0.0072 in y08; 0.29 in {red, green, blue} in the
light tunnel). I did not calculate, before this declaration, the domain-level p-value defined below, nor any corrected
result, nor any v1 execution.

## Point 3: multiplicity

### Counting unit

An **ICP test** is one run of the ICP on a domain with a specification (data, unit, environments, candidates,
invariance test, saturation rule, α) that produced p-values per subset. I count three versions of M:

- **M = 38 (main): distinct tests.** 11 of the H2; 6 reruns of the domains of the H2 with the rank correction
  (H2b); 3 of the H2b plus 2 diagnostic ICPs (Sachs and seatbelt, after pruning); 3 of the H2c (K = 2); 5 of the H2d (K = 3); 2
  new ones in the H2e; 5 new ones in the H2f (genetics that reached F4); 1 new one in the H2g. A deterministic, identical rerun
  of the same specification is not a new attempt and does not enter.
- **M = 32 (sensitivity, downwards):** without the 6 reruns with the rank correction, which gave identical Ŝ and counts of
  accepted.
- **M = 72 (sensitivity, upwards): every run whose result was seen.** The 54 runs of the F4 recorded in
  `h2*/saida`, plus 2 diagnostics, plus 6 reruns of the rank, plus 10 ICPs of the v1 execution of the H2 (the Preston was
  pruned in the F3 in v1).

The pilots of the H2 (education and logistics, before the criteria) and the second executions of `portoes.py --reproducao`
do not enter: of the pilots there is no ICP record, and the reproductions are deterministic.

### Statistic per test

For each test, with F = the family that counts (the non-saturated ones, when the saturation rule of the H2d is active; all
subsets before that) and p_S the invariance p-value of subset S:

- p_max = max_{S ∈ F} p_S;
- p_j = max_{S ∈ F, j ∉ S} p_S, the variable p-value of the ICP (Peters, Bühlmann & Meinshausen 2016);
- **p_dom = min_j p_j.**

Since the ICP accepts S when p_S > α, it holds that Ŝ ≠ ∅ at level a if and only if p_dom ≤ a < p_max. p_dom is a valid
p-value for the error "Ŝ brings a non-parent": for every non-parent j, p_j ≥ p_{S*}, with S* = pa(Y). Running the ICP with α
divided amounts to replacing a by α/M in that condition; the saturation does not depend on α. Before any correction, I
check that this rule reproduces, at the original α, the recorded Ŝ of each test. If it does not reproduce, the count stops and I
report it.

The p-values per subset come from `filtro4.icp.p_por_subconjunto` and `filtro4.icp.saturados` of the recorded
results. For the 8 tests without a recorded output (6 reruns of the rank, 2 diagnostics), I run the ICP with the code of the
horizon itself (H2b) on the training set that the funnel of the horizon assembles. If that does not work, they enter M with p_dom = 1
for Holm and BY, and I say so.

### Corrections (α = 0.05)

1. **Bonferroni:** level α/M per test.
2. **Holm:** descending step by step over the p_dom of the M tests. The test of rank i (from the smallest to the largest) is
   compared to α/(M − i + 1).
3. **Benjamini–Yekutieli** (FDR under arbitrary dependence): ascending step by step, with threshold i·α/(M·c_M) and
   c_M = Σ_{k=1}^{M} 1/k.
4. **Sequential, with the 7 rounds as looks.** The α spend is uniform, α/7 per round: H2, H2b, H2c, H2d, H2e,
   H2f and H2g. In each round, Bonferroni over the m_r tests that the round evaluates, including the inherited ones: level
   α/(7·m_r). By the union inequality, the chance of some Ŝ with a non-parent in some look stays ≤ α. With m_r:
   H2 11; H2b 11 (3 + 2 diagnostics + 6 of the rank); H2c 3; H2d 5; H2e 7; H2f 12; H2g 13.

In Holm and BY, a test counts as Ŝ ≠ ∅ when it is rejected by the procedure and p_max > the threshold of its rank
(there is still an accepted set at that level).

As a note, without deciding anything: the Occam cost of choosing among M attempts is log₂ M bits. The evidence −log₂ p_dom
has to exceed log₂ M + log₂(1/α), and that is equivalent to Bonferroni.

### Criterion

- **Main question:** does the approved output of the H2 survive? The approved gate is the G3 of the H2g: Ŝ ≠ ∅ in at least 3
  domains of distinct areas. It survives a correction if, at the corrected level, at least 3 distinct areas
  of the H2g round have Ŝ ≠ ∅.
- **Correction that decides: Bonferroni with M = 38.** Holm, BY, the sequential version and M = 32 or 72 are reported
  alongside, and I say where they disagree.
- **Areas.** The area is the field `area`, by the first word (Education, Physics, Genetics, Pharmacology, Law,
  Health, …). I also apply, alongside, the rule of the H2f: a Ŝ = {plate} alone does not count.
- **Grouped reading, informative only:** the union of the corrected discoveries of all rounds, counted by area.
- **What counts as failure of the output of the H2:** fewer than 3 areas with Ŝ ≠ ∅ at the Bonferroni level with M = 38.

### Confirmation with the horizon code

I copy `h2g/` to a working folder and change only the α. The funnel already reads `alfa_icp` from the package, so I write
`alfa_icp: α/38` in the 18 packages and nothing else. I run `rodar.py` and the G3 of `portoes.py`, without `--reproducao`, and the
result has to match the count made over the recorded p-values. If it does not match, I report both.

## Point 4: adjustments after seeing results

### H2, framework v1 against v2

The v1 code is not in git: the history has a single research commit (`4bb7181`) and it only brings v2. I reconstruct
v1 in a copy of `h2/`, reverting the four changes described in `h2/README.md` and in the "(v2)" comments of
`h2/estocologia/funil.py`:

1. **F3:** the key is permuted row by row inside each environment. That is what the header of `funil.py` says: "the
   permutation null inside the environment".
2. **Likelihood without tempering:** weight = 1.
3. **Conforme:** calibration on a fifth of the training set drawn with the generator of the funnel. The maximum likelihood fit
   uses the other four fifths, and the scores are the same as in v2 (residual in the Gaussian, 1 − p of the observed class
   in the binomial).
4. **ICP labels:** no numerical effect; I do not reconstruct.

**Validation of the reconstruction.** The README says that, in v1, energy and cyclones fell in the conforme, the credit in the
R-hat (1.011) and the Preston in the F3. If the reconstruction reproduces the four, I treat it as faithful. If not, I report which
ones match and treat the rest as sensitivity to an uncertain reconstruction.

I run the 12 domains of the H2 with the reconstructed v1 and with v2 (intact copy) and show them side by side. Since v2 was also
inherited by the H2b … H2g, I run the H2g with the reconstructed v1: only the F3 can change the G3, because the G3 counts every
domain that reaches the F4, approved or not in the M15. Applying the same three reversals to `funil.py` of the H2g is the only
change.

### H3, the four versions of the performative loop estimator

Only v4 (2SLS) exists in the code. I reconstruct the other three in a copy of `h3/estocologia/performativo.py`, from the
description in the `historico` field of the module itself. The shocks are z_j = expl[d, j] ∈ {−0.3; 0; +0.3}, the excess is
exc_j = obs_j − previsão_j and a_j are the abandonments:

- **v1:** the scale of the controlled predictor comes from the forecast of the corrected demand, not from the observed load. The r̂ uses
  a single interval: r̂ = Σ z_j·exc_{j+1} / (0.6·Σ z_j·a_j).
- **v2:** the scale serves the observed load, as in v4. r̂ = Σ z_j·(exc_{j+1} + exc_{j+2}) / Σ z_j·a_j.
- **v3:** the ratio of v2 with pre-shock control: numerator Σ z_j·[(exc_{j+1} + exc_{j+2}) − (exc_{j−1} + exc_{j−2})]
  and denominator Σ z_j·(a_j − a_{j−1}).
- **v4:** the current code, unchanged.

The sums run over the days with a forecast and the intervals with the defined indices. The seed is that of the module (2027).

**Metric:** the relative bias of the controlled predictor against the exogenous demand, with r = 0.6, on days 60 onward.
**Criterion G4(b) of the H3, unchanged:** |bias| ≤ 3%.

**Validation:** the `historico` gives the reference numbers: v1 r̂ = 0.55 and bias 6.6%; v2 r̂ = 0.70 and bias −4.8%;
v3 r̂ = 0.77; v4 r̂ = 0.56 and bias 1.7%. I report how far the reconstruction departs from them.

### H3, vault openings

I inventory the openings from `h3/saida/*/especificacoes.jsonl`, from the bars and from the READMEs. For the two
corrections after opening the vault, the one of the central (Erlang C) and the one of the row groups of `prisoes_ri`, I say from the code
whether they touch the forecasts. If so, I check the one of `prisoes_ri` by running the copy with groups of 32 rows and comparing the
diary.

### "If only the first declared version counted"

I report the verdict of each gate with the first declared specification:

- **H2:** the H2 round, with v1, test K = 1 and α = 0.05.
- **H3 G4:** the v1 of the estimator.
- **H3 G2:** the six streams of the original CRITERIOS.md, without the new vaults.
