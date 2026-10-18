# Background point 2: independence (N-version reimplementation from the specification)

## Result

The four results that support the approved outputs were reimplemented from scratch, without reading the code or the
outputs of the horizons, and **all agree within the tolerances declared beforehand**. The ICP of the STAR and the one of the hERG
came out identical to the published ones (Ŝ, count of accepted, saturated and largest p). The LaLonde and the Rhode Island agree
in the conclusion and in the main numbers, but not exactly, and the differences come from choices that **are not in the
README nor in the declarations**: the covariates of the propensity score and the fact that the "+0.096" is the median of a
Bayesian posterior (LaLonde), and the row filter, the emission hour and the parameters of the circuit breaker (Rhode Island).
I found zero errors of the original and zero errors of mine that change a conclusion. The most serious limit: the "independent
team" was me, a model of the same family that wrote the original, and several choices that the specification
does not fix coincided with those of the original without being written down. That is correlated failure, not independence; see
"Limits".

## What was done

1. **Contract.** I read only READMEs, declarations, criteria, YAML packages and the `datacontract.yaml` of H1·B, plus the
   CSV headers. The exact list is in `DECLARACAO.md`.
2. **Declaration first.** `DECLARACAO.md` carries each published number with file:line, the tolerance and the choices
   I made where the specification is silent. SHA-256 and time in `DECLARACAO.sha256`
   (`5cd635e3…f1665a`, 2026-10-15T12:11:34Z).
3. **Implementation.** `comum.py` (Newton logistic, test K = 2, 1-NN rule K = 3, ICP), `r1_lalonde.py`,
   `r2_star.py`, `r3_herg.py`, `r4_ri.py`. Fixed seed 20261015 (only the bootstrap of R1 uses the generator).
4. **Freeze.** Hashes of the `.py` files and of `rodar.sh` in `CONGELAMENTO.sha256`, at 2026-10-15T12:18:39Z. None
   of those files changed afterwards.
5. **Only then** did I read the original code (`h1-real/projetos/b_real.py`, `h2d/dominios/educacao_star_turmas/preparo.py`,
   `h3/fluxos/prisoes_ri.py`, `h3/fluxos/base.py`, `h3/estocologia/calibracao.py`, `h3/avaliar_h3d.py`) and the
   JSON outputs, to explain each difference. Two post-freeze diagnostics, separate from the frozen version, are in
   `diagnostico/`.

## Table

| result | published | my reimplementation | agrees? | cause of the difference |
|---|---|---|---|---|
| R1 · experiment, ΔR of employment | +0.111 [0.026; 0.196] (`h1-real/README.md:58`) | +0.1106 [0.0257; 0.1955] | yes | none (CI: variance with ddof = 1 in the original, binomial in mine) |
| R1 · experiment, Δ income | +1,794 | +1,794.3 | yes | none |
| R1 · raw against CPS, ΔR | −0.107 [−0.170; −0.045] (`:56`) | −0.1074 [−0.1695; −0.0454] | yes | none |
| R1 · raw, Δ income | −8,498 | −8,497.5 | yes | none |
| R1 · propensity score, ΔR | +0.096 [−0.006; 0.208] (`:57`) | +0.088 [−0.027; 0.215] (weighting, bootstrap) | yes (difference 0.008 ≤ 0.03) | **(iii) not specified.** The published +0.096 is the median and the 95% CI of an MCMC posterior of 4 cells (arm × black) with Kish pseudo-counts and standardization by race (`b_real.py:485-500`, `:770-773`). The weighting point in the original is 0.0905. The README calls this only "adjustment by propensity score". |
| R1 · score, Δ income | +1,570 [218; 3,265] | +1,680 [178; 3,203] | yes (Δ = 110 ≤ 500) | **(iii).** The covariates of the score (with re74² and re75², without educ·re74) are only in the docstring of `b_real.py:22-24`. With them, my code gives 1,570.2 (`diagnostico/diag_r1_spec_original.py`). |
| R1 · effective controls | ≈ 103 (`:61`) | 87.9 | yes (−15%, tolerance ±30%) | **(iii)**, the same cause: with the specification of the original, my code gives 102.5. |
| R1 · CPS below the lowest score | 72% (`:61`) | 73.2% | yes | (iii), the same cause: 71.6% with the specification of the original. |
| R2 · STAR Ŝ | {anos_pequena_antes, turma_pequena} (`h2d/README.md:76`) | {anos_pequena_antes, turma_pequena} | yes, identical | none |
| R2 · accepted non-saturated | 57 | 57 | yes, identical | none |
| R2 · accepted saturated / saturated tested | 152 / 256 | 152 / 256 | yes, identical | none |
| R2 · largest non-saturated p | 0.075 in the set of 6 (`:76`) | 0.0751 in the same set | yes, identical | none |
| R2 · p of {turma_pequena} | 0.010 (`:99`) | 0.0101 | yes | none |
| R3 · hERG Ŝ | {c_dof, c_qui, c_ran} (`h2g/README.md:6`) | {c_dof, c_qui, c_ran} | yes, identical | none |
| R3 · accepted / saturated | 35 / 0 (`:33`) | 35 / 0 | yes, identical | none |
| R3 · QTcF slope under dofetilide | 25.2 [23.0; 27.5] (`:38`) | 25.21 [22.99; 27.43] (normal conjugacy) | yes | none of substance: the original uses MCMC (mean 25.211) and pooled SE 1.355; mine, 1.359 (small-sample correction) |
| R4 · n in the vault | 24,466 (`h3/README.md:227`) | 24,502 | yes (+0.15%) | **(iii).** The original also removes rows without sex or without age (`prisoes_ri.py:72`); the declaration only speaks of the rows without an outcome. And the forecast is emitted at the end of the tick, so 5 stops of 31/12/2012 after 18 h enter the vault. 24,461 + 5 = 24,466 (`diagnostico/diag_r4_filtro_original.py`). |
| R4 · ȳ in the vault | 2.26% | 2.265% | yes | the same |
| R4 · \|p̄ − ȳ\| official | 0.0020 | 0.0007 | yes (passes at both limits; Δ ≤ 0.003) | **(iii).** The circuit breaker (window 400, margin 0.03, k = 3, interval 50; `prisoes_ri.py:33`) is not published and I did not implement it: the fallback was official in 12% of the vault in the original and in 0% in mine. Also age is standardized by fixed constants (34; 12.7) and the Platt step uses p' of the emission. |
| R4 · official ECE | 0.0048 | 0.0051 | yes (passes at both limits) | the same ones |
| R4 · official skill | +0.019 | +0.022 | yes | the same ones |
| R4 · raw model | 0.013; 0.013; +0.004 (`:233`) | 0.013; 0.013; +0.005 | yes | none of substance |
| R4 · Platt at the end | a = −0.30, b = 1.07 (`:236`) | a = −0.297, b = 1.071 | yes | none |
| R4 · criterion (a) and (b) | passes | passes | yes | none |

Classification of the causes: (i) my error: none that changes a number outside the tolerance; (ii) error of the original:
none found; (iii) unspecified choice: five, all in the LaLonde and in the Rhode Island.

## What the divergences say

- **The specification is not enough to reproduce the LaLonde.** The README gives the number, but does not say that it comes from a
  posterior, nor which score. An independent team reaches +0.088 instead of +0.096, and 88 instead of 103
  effective controls. The conclusion ("the adjustment hits the point, but rests on few controls") survives.
- **The specification is not enough to reproduce the Rhode Island exactly.** The circuit breaker of the stream is part of the
  official forecast, but the parameters are only in the code. Without it, the calibration comes out even better (|p̄ − ȳ| 0.0007
  against 0.0020), so the approval of the H3d does not depend on the circuit breaker.
- **STAR and hERG reproduce exactly from the declarations.** The declarations K = 2, K = 3 and H2g, with the YAML package,
  were enough. But see the first limit below.

## The signed opinions

What sil would need to do: reread in full the 21 opinions listed in `governance/ASSINATURAS.md` (the files
with the SHA-256 from there, not the summary) and then confirm or withdraw the signature, recording that the full text was read.
Nothing was done here about that.

## How to reproduce

```
cd empirical/p02-independencia
./history/rodar-p02-independencia-2026-10-17.sh   # ~4 min from the root; the script left the folder on 2026-10-17
python3 -I diagnostico/diag_r1_spec_original.py     # post-freeze: score with the covariates of the original
python3 -I diagnostico/diag_r4_filtro_original.py   # post-freeze: row filter of the original
sha256sum -c <(head -6 CONGELAMENTO.sha256)         # checks that the code is the frozen one
```

I ran the reproduction twice after the first execution; the four JSONs had identical hashes in all three:
`r1_lalonde.json 8461cca1…`, `r2_star.json d758e7b0…`, `r3_herg.json f0fd0d1e…`, `r4_ri.json 14102f54…`.

## Limits

- **It is not author independence.** The reimplementation was done by a model of the same family as the one that wrote the
  original. Several choices that no document fixes coincided with those of the original: in the STAR, "mestrado =
  degree different from bachelor", "etnia = cauc, afam or other" and the standardization by the regular classes of the grade
  (`preparo.py:23-38`); the exact coincidence of 57 accepted depends on them. That is exactly the failure mode that
  background point 2 points to: versions that agree because they share the author. An agreement from here is weak
  evidence of correctness; a divergence would be strong. Real independence asks for a person or team from outside, with the
  same contract.
- **The contract was not only the README.** For STAR and hERG I also used the declarations and the YAML packages, as the request
  allowed. The README alone is not enough for any of the four.
- **R4 without the circuit breaker.** I did not reimplement the circuit breaker nor the drift sensors, because the parameters are not
  published. The comparison of R4 is therefore that of the predictor without the circuit breaker against the official one with the circuit breaker.
- **R1 without the MCMC.** I did not reimplement the 4-cell posterior of the original, which I had no way of knowing
  existed; I compared the weighting estimator with bootstrap.
- **Scope.** Only the four requested results. Placebo, severity, fairness and vault of the LaLonde, the F2/F3/F8 of the
  domains of the H2 and the F6/G3/G5 of the H3d were not reimplemented.
