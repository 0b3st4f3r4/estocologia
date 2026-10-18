# Declaration of point 7 (ICP): clustering-robust version and replication of genetics

Written on 2026-10-15, before running any ICP in this folder. Up to here I had only seen the H2g code and the results already
published in `h2g/saida/` (Ŝ, number of accepted sets and best p of STAR, of pharmacology and of y08), and I checked the structure
of the data without looking at Y: 80 schools in STAR, 446 of 11,598 students go through more than one school, and the ECGRDVQ has
109 volunteer × arm blocks with 16 instants each. The SHA-256 of this file lives in `DECLARACAO.sha256`.

## The audit point

"The test assumes independent rows (16 measures per volunteer in the FDA trial, H2g); Education (STAR) is fragile
(largest p 0.075); Genetics (H2f) was approved with reservation."

## What does not change

- The code runs from a copy of `h2g/` in `empirical/p07-icp/codigo/`. The originals are not touched.
- The data, the packages, the training environments (the vault stays outside), the candidates, the encoding (`Desenho`), the α of
  each domain and the saturation rule K = 3 stay as in H2g.
- The pooled fit of the test also stays the same: the same expansion (linear + restricted spline), the same residual
  r, the same scale ŝ and the same u = r / ŝ of `p_invariancia_np`. Only the way of testing whether u has the same law across
  environments changes.
- First I reproduce, with the original test, the published Ŝ of each domain. If it does not reproduce, I stop and report.

## Main robust test (decides): Wald with clustered error, in the same two parts of the original test

For each S, with u computed as above:

1. **Location.** z = rank of u among all rows (the Kruskal–Wallis is the ANOVA of ranks). I regress z on the
   environment indicators (intercept + m − 1 indicators) by OLS and test that the m − 1 coefficients are zero with
   a Wald of clustered covariance CR1 (factor G/(G − 1) · (n − 1)/(n − k)). Statistic F = W/q, reference
   F(q, G − 1), with q = rank of the restricted covariance.
2. **Scale.** d = |u − median of u in the environment| (the Brown–Forsythe transformation). The same clustered Wald.
3. p = min(1, 2 · min(p_loc, p_esc)), Bonferroni of the two parts, as in the original.

Clusters (the clustering):

- **Pharmacology (hERG, `farmacologia_qt`):** the volunteer (`unidade`, 22). The cluster crosses the arms; the clustered Wald
  takes into account the correlation within the volunteer, between instants and between arms.
- **Education (STAR, `educacao_star_turmas`):** the school (`unidade` = schoolid of the grade). The 446 students who change
  school link two clusters; that stays as a declared limit.
- **Genetics (Kemmeren):** the sample is a culture of its own and there is no batch label besides `plate` (2 levels, which
  do not serve as clusters). The clustered test does not apply; the correction of genetics is the replication below.

## Secondary robust test (informs, does not decide): block permutation

Same statistics of the original test (Kruskal–Wallis H on u and Brown–Forsythe F on u), with the null given by
permuting the environment labels of whole blocks within the cluster:

- pharmacology: block = volunteer × arm (the 16 instants together), permuted among the training arms of the same
  volunteer (the trial is crossover, every volunteer goes through all of them);
- STAR: block = school × environment, permuted among the training environments present in the same school.

B = 999 permutations, `np.random.default_rng(7)`, the same set of permutations for all S. p of each part =
(1 + #{T* ≥ T}) / (B + 1); then the same Bonferroni. STAR has a caveat: the blocks of a school have different
sizes and divide students between grades, so the exchange is only approximately valid.

## Survival criteria

- **Pharmacology survives** if the Ŝ of the main test is not empty and does not contain `pr` nor `tamp`. It is "solid" if the
  Ŝ of the secondary one is also not empty.
- **Education survives** if the Ŝ of the main test is not empty. "Solid" if the secondary one also gives Ŝ ≠ ∅.
- **Genetics: replication with more Y genes.**
  - New draw of 40 Y genes among the never deleted, taking out the 10 of H2f:
    `np.random.default_rng(7007).choice(sorted(nunca_deletados − 10_do_H2f), 40, replace=False)`.
  - For each Y: same pre-selection (9 deleted genes with the largest |correlation| in the wild-type, x1 oriented), `plate`,
    the same environments (obs, del_EJ, del_KP; vault del_AD out) and the original test of H2g (residuals + saturation).
  - α per Y = 0.05 / 40 = 0.00125 (Bonferroni in the new family).
  - The ICP runs on all 40, without the F1–F3 filters of the funnel (the Ŝ does not depend on them).
  - **Validation by intervention** of each gene X of each Ŝ (outside `plate`), in the mutant ΔX of the training samples and
    of the vault: (i) the deletion worked if X in the mutant row ΔX is the smallest value of X among all rows;
    (ii) **strict validation** (Meinshausen et al. 2016): Y in the mutant ΔX stays outside the range of Y in all other
    rows; (iii) **weak coherence**: the quantile of Y in the mutant is in the 5% of the tail predicted by the sign of the
    coefficient of X in the pooled fit of Ŝ (coefficient > 0 predicts low Y, quantile ≤ 0.05; < 0 predicts ≥ 0.95).
  - **Genetics survives** if at least one of the 40 new Y gives an Ŝ with some gene and at least one gene of those Ŝ
    passes the strict validation. If there is an Ŝ with a gene but only weak coherence, genetics stays "fragile" and does **not**
    count. If no new Y gives an Ŝ with a gene, genetics does not replicate and does **not** count (y08 alone remains what it already was:
    approved with reservation).
- I also apply the validation to the genes of the Ŝ of the original y08, only to check the H2f table.

## Verdict of the H2 gate (Ŝ ≠ ∅ in 3 distinct areas)

- Physics (tunnel of light) is not part of this point and enters as it is in H2g.
- The gate **survives** the correction if Physics + the areas that survive above add up to at least 3.
- I also report the count with only the "solid" areas.

## What counts as a failure

Not reproducing the original Ŝ; any change of test, cluster, draw, α or criterion after seeing a result.
A change like that enters as a new declaration and counts in K.
