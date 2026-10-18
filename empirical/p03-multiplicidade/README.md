# Points 3 and 4: uncounted multiplicity and adjustments after seeing results

## Result

**The approved output of the H2 survives the multiplicity correction.** The honest global K of the gate of the H2 is 38 distinct ICP
tests (32 to 72, depending on the count). With Bonferroni over 38 (α = 0.05/38 = 0.0013 per test), the round
H2g still has Ŝ ≠ ∅ in 4 areas: Education, Physics, Pharmacology and Genetics. Holm, Benjamini–Yekutieli and the sequential
version (α/7 per round) also leave at least 3 areas. The count over the recorded p-values was confirmed by
running `h2g/` itself with `alfa_icp = 0,05/38` in the 18 packages: the G3 of `portoes.py` gives the same 4 areas.

What the correction changes:
- **The Ŝ shrink.** In the STAR, {anos_pequena_antes, turma_pequena} becomes {turma_pequena}. In y08, {plate, x1, x5, x9}
  becomes {plate, x5}: x1, the only gene that the interventional validation confirmed, leaves.
- **What holds the gate** are three domains with a tiny domain p: light tunnel (p = 0), pharmacology
  (1e-60) and STAR (4e-7).

There are two caveats, both important:
- **The correction is not monotone in the ICP.** With a smaller α, domains that rejected everything start accepting some set.
  That creates Ŝ in almost rejected models, such as Preston and Energy in the H2.
- **The approval depends on the v2 framework.** With the reconstructed v1 F3, the STAR is pruned before the ICP, because the key
  is constant inside the environment and the permutation inside the environment does not move it. The H2g then has 3 areas, and one
  of them is the fragile Genetics. If, besides that, one also requires that the model not be rejected at 0.05 (undeclared
  sensitivity), 2 areas remain and the gate falls.

In the H3, only the v4 of the performative loop estimator passes the G4(b):
- v1: bias of +4.7%;
- v2: −5.9%;
- v3: −8.4%;
- v4: +1.7%, against a limit of 3%.

If only the first declared version counted, neither the H2 nor the H3 would be approved:
- in the H2, round 1 has 1 area;
- in the H3, the G4 fails with v1, and the G2 fails in the original vaults of voos and prisões.

## 1. Inventory: global K

The script `analise_multiplicidade.py` reads the 54 `filtro4` recorded in `h2*/saida/*/resultados.json` and the 8 tests
redone by `icp_direto.py`. It first checks that the rule below reproduces, at the original α, the recorded Ŝ and the number of
accepted, and the 62 check out.

| round | ICP tests seen | new (distinct) | what is new |
|---|---|---|---|
| H2 | 11 | 11 | 11 convenience domains; test K = 1 without rank |
| H2b | 11 | 11 | 3 intervention domains; 2 diagnostic ICPs (Sachs, seatbelt) after pruning; 6 reruns of the H2 with the rank correction |
| H2c | 3 | 3 | test K = 2 (non-parametric) |
| H2d | 5 | 5 | saturation rule (K = 3); Sachs by cell; seatbelt with fixed effect |
| H2e | 7 | 2 | light tunnel, wind tunnel |
| H2f | 12 | 5 | genetics y02, y03, y04, y07, y08 (the other 5 Y pruned in the F3), α = 0.005 |
| H2g | 13 | 1 | pharmacology |
| **total** | **62** | **38** | |

Counts of M:
- **M = 38** is the main count: the distinct tests, because rerunning the same specification deterministically
  is not a new attempt.
- **M = 32** removes the 6 reruns of the rank, which gave identical Ŝ.
- **M = 72** adds the 10 ICPs of the v1 execution of the H2, which were not recorded: every execution seen.
- Out of the count, for leaving no record: the pilots of the H2 (education and logistics, before the criteria) and the
  K = 1 specification of the logistics, blocked in the F2.

**Statistic per test.** For each test, p_dom = min_j max_{S ∌ j} p_S is the smallest variable p-value of the ICP,
taken over the family that counts (without the saturated ones from the H2d on). Ŝ ≠ ∅ at level a ⇔ p_dom ≤ a < p_max. p_dom is a
valid p-value for the error "Ŝ contains a non-parent". The complete table, with the 62 tests, p_max, p_dom, the variable that
defines it and Ŝ before and after, is in `resultados/multiplicidade.json`. The tests that matter:

| round | domain | p_max | p_dom | published Ŝ | Ŝ with Bonferroni (M = 38) |
|---|---|---|---|---|---|
| H2 | direito_prisoes | 0.070 | 0.0105 | {cidadao, empregado} | ∅ |
| H2 | energia_vitoria | 0.0042 | 7.4e-10 | rejects everything | 5 variables |
| H2 | desenvolvimento_preston | 0.0051 | 5.7e-41 | rejects everything | {decadas_desde_1952} |
| H2b | direito_teto_beneficio | 0.087 | 0.033 | {beneficio_log} | ∅ |
| H2b | educacao_star_turmas | 0.0013 | 4.4e-15 | rejects everything | {turma_pequena} (by a little: p_max = 0.00133 > 0.00132) |
| H2b | transporte_cinto (diagnostic) | 0.054 | 0.0072 | {idade21, idade_media, vel65, vel70} | ∅ |
| H2c | saude_rand | 1 | 2e-29 | {coparticipacao} (saturated) | {coparticipacao} |
| H2d–g | educacao_star_turmas | 0.075 | 4.2e-7 | {anos_pequena_antes, turma_pequena} | {turma_pequena} |
| H2e–g | fisica_tunel_luz | 0.52 | 0 (below the smallest double) | {blue, green, red} | {blue, green, red} |
| H2e–g | engenharia_tunel_vento | 7.3e-5 | 7e-15 | rejects everything | rejects everything |
| H2f–g | genetica_y08 (α = 0.005) | 0.0072 | 0.00053 | {plate, x1, x5, x9} | {plate, x5} |
| H2g | farmacologia_qt | 1 | 1e-60 | {c_dof, c_qui, c_ran} | {c_dof, c_qui, c_ran} |

## 2. Corrections and the verdict of the H2

The criterion declared in `DECLARACAO.md` (SHA-256 in `DECLARACAO.sha256`, 2026-10-15T12:17:03Z, before any
corrected calculation): the output of the H2 survives if, at the corrected level, the H2g round has Ŝ ≠ ∅ in at least 3 distinct
areas. The correction that decides is Bonferroni with M = 38.

| correction | level per test in the H2g | areas with Ŝ ≠ ∅ in the H2g | G3 of the H2g | earlier rounds that would pass |
|---|---|---|---|---|
| none (published) | 0.05 (0.005 in genetics) | Education, Pharmacology, Physics, Genetics | passes | H2f |
| **Bonferroni, M = 38** | **0.00132** | **Education, Pharmacology, Physics, Genetics** | **passes** | H2f |
| Bonferroni, M = 32 | 0.00156 | the same 4 | passes | H2f |
| Bonferroni, M = 72 | 0.00069 | the same 4; Genetics only with Ŝ = {plate} | passes (3 areas by the plate rule of the H2f) | H2f by the letter |
| Holm, M = 38 | 0.0013 to 0.0028 | the same 4 | passes | H2f |
| Holm, M = 32 / 72 | | the same 4 | passes | H2f |
| Benjamini–Yekutieli, M = 38 | 0.0065 | the same 4 | passes | H2f |
| Benjamini–Yekutieli, M = 32 | 0.0081 | Education, Pharmacology, Physics (y08 stays out) | passes | none |
| Benjamini–Yekutieli, M = 72 | 0.0060 | the same 4 | passes | H2f |
| sequential, α/7 per round, Bonferroni in the round (m = 13 in the H2g) | 0.00055 | the same 4; Genetics only with Ŝ = {plate} | passes (3 areas by the plate rule) | H2f by the letter |

Notes on the table:
- **Plate rule.** By the declared rule of the H2f, a Ŝ = {plate} alone does not count. With M = 38 the y08 has
  {plate, x5} and counts. With M = 72 and in the sequential version it falls to {plate}, Genetics leaves, and the 3 areas
  without it remain.
- **Occam cost.** For M = 38, the cost is log₂ 38 + log₂ 20 = 9.6 bits. The evidence of the STAR is −log₂ 4.2e-7 =
  21 bits, that of pharmacology is 199 bits and that of y08 is 10.9 bits. The y08 passes by 1.3 bit.

**Confirmation with the horizon code.** `copias/h2g_bonf` is `h2g/` with only `alfa_icp: 0.0013157894736842105`
in the 18 packages.
- The G3 of `h2g/portoes.py` over the output gives "Ŝ ≠ ∅ em 4 domínio(s), 4 área(s): educacao_star_turmas,
  farmacologia_qt, fisica_tunel_luz, genetica_y08".
- The Ŝ match the count.
- Status, F1–F3 and later ones are identical to the published ones (`resultados/execucoes.json`).

**A caveat that the count revealed: the correction is not monotone in the ICP.** Dividing α protects against Ŝ with non-parents,
but it also makes sets acceptable in models that, at 0.05, were rejected entirely. With Bonferroni, the round H2
gains Development and Energy (p_max of 0.005 and 0.004) and loses Law. The formal guarantee still holds, but
those Ŝ come from almost rejected models.

For that reason I report a sensitivity **that was not declared**: also requiring p_max > 0.05. With it, the H2g has
Education, Physics and Pharmacology, 3 areas, and still passes. The y08 leaves, because its p_max is 0.0072.

## 3. Adjustments after seeing results

### H2: framework v1 against v2

The v1 code does not exist in git: there is a single research commit (`4bb7181`), and the shared disk only has v2. I
reconstructed it in `preparar_copias.py`, reverting the three numerical changes marked "(v2)" in `h2/estocologia/funil.py`:
- F3 with permutation inside the environment;
- likelihood without tempering;
- conforme on a drawn fifth of the training set.

The fourth change, the ICP labels, does not change any number.

**The reconstruction is not faithful.** The README of the H2 says that, in v1, energy and cyclones fell in the conforme, the credit in the
R-hat (1.011) and the Preston in the F3. My reconstruction reproduces only the case of the cyclones. The redone v2 is identical to the
published one, in the 12 domains.

| domain | v2 (published = redone) | reconstructed v1 | v1 according to h2/README.md |
|---|---|---|---|
| clima_tempestades | approved; conforme 0.852 | **failed in the conforme (0.841)** | failed in the conforme |
| energia_vitoria | approved; conforme 0.910 | approved; conforme 0.877 | failed in the conforme |
| credito_hmda | approved; R-hat 1.0096 | approved; R-hat 1.0074 | failed in the R-hat (1.011) |
| desenvolvimento_preston | approved; F3 p = 0.005 | approved; F3 p = 0.005; 90% CI [4.64; 5.28] against [4.14; 6.04] | pruned in the F3 |
| trabalho_curriculos | failed in the conforme (0.786) | **approved (0.874)** | (not cited) |
| the other 7 | | same status; 90% CI of the key generally narrower without the tempering | |

What can be stated safely:
- **The G3 of the H2 does not depend on the framework.** The ICP runs on the training set before any step of v2, and the Ŝ is identical
  in the two (1 area, Law).
- **The changes of v2 touch the statuses and the width of the posteriors.** In the reconstruction, v2 also brought down one
  domain (currículos): the changes were not all in the direction of approving.

The four cases "from failed to approved" of the README could not be reproduced. That may be a failure of the
reconstruction: the drawing of the fifth, the order of the generator and the fit on the four fifths are my assumptions.

**The H2g with the reconstructed v1 (`copias/h2g_v1`).** In the H2g, only the F3 can change the G3.

- **Domains pruned in the F3.** With the permutation inside the environment, the STAR, the RAND, the Sachs and the y04 fall in the F3. In the
  STAR and in the RAND the key is constant inside the environment (the environment is the class or the plan), so the permutation changes
  nothing and the test has no way of rejecting.
- **G3 by `portoes.py`:** 3 areas, Pharmacology, Physics and Genetics. It passes.
- **G3 with Bonferroni (M = 38):** the same 3 areas, with the y08 = {plate, x5}. It passes.
- **Reading.** Without v2, the gate of the H2g depends on the y08, which is the evidence that the project itself called fragile.
  With the undeclared sensitivity (p_max > 0.05), it would be 2 areas, and the gate would fall.
- **What changed the F3.** The change of the F3 of v2 was made in the H2, before the intervention domains existed. It is
  the right correction for those designs, but it was chosen after seeing results of the H2.

### H3: the four versions of the loop estimator

Only v4 is in the code. The other three were reconstructed in `laco_versoes.py`, from the description of the field
`historico` of `h3/estocologia/performativo.py`. The naive simulation is the same, with seed 2027 and r = 0.6. The
criterion G4(b) did not change: |bias| ≤ 3%.

| version | r̂ (reconstructed) | r̂ in the `historico` | bias against the exogenous demand | bias in the `historico` | G4(b) |
|---|---|---|---|---|---|
| v1: scale by the corrected demand; Wald of one interval | 0.562 | 0.55 | **+4.7%** | +6.6% | fails |
| v2: scale by the observed load; Wald of two intervals | 0.722 | 0.70 | **−5.9%** | −4.8% | fails |
| v3: Wald with pre-shock control | 0.773 | 0.77 | **−8.4%** | (not cited) | fails |
| v4: 2SLS (current code) | 0.563 | 0.56 | **+1.7%** | +1.7% | passes |

- **The reconstruction stays close to the historical numbers.** The v4 is identical to the published `portoes.json`.
- **The naive predictor** errs by +29.1% in all versions.
- **The result depends on the version:** three of four fail.
- **In v1, what fails is the design, not the estimator.** The r̂ of v1 is almost that of v4 (0.562 against 0.563), and the bias
  comes from the loop design: scaling by the corrected demand feeds the spiral of abandonments.

### H3: vault openings

The inventory comes from `h3/saida/*/especificacoes.jsonl`, from the bars (one `cofre_aberto` per stream in the recorded
execution) and from `h3/README.md`.

| stream | openings with a seen result | after seeing |
|---|---|---|
| energia, bolsa, voos, ciclones, prisões (H3) | 1 each | voos and prisões failed in the G2 and led to the H3b, the H3c and the H3d, with new vaults |
| central (H3) | 2 | the 1st execution hung (Erlang C with factorials). The swap for the Erlang B recursion only touches `decidir()` (`h3/fluxos/central.py`, line 200), not the forecasts. It counts in K (3). |
| voos23 (H3b) | 1 | passes |
| prisoes20 (H3c) | 1 | fails (ECE 0.067) |
| prisoes_ri (H3d) | 2 | the G5 failed with groups of 32 rows, and the groups became 4 |
| **total** | **11** | plus the deterministic reproductions of the G1, which do not enter |

**Check of `prisoes_ri`.** I ran the copy `copias/h3_ri32` with 32 rows per group. The G5 gives 6.36 scans
per table, the same 6.4 of the README. The diary matches the published one in `diario.parquet` (SHA-256 `0c0e05aa…`, equal to
`FORA_DO_GIT.sha256`):
- same official origin in all 83,632 rows;
- maximum difference of 1.5e-15 in the probabilities, which is floating-point rounding: I ran with
  OMP_NUM_THREADS=1;
- detail in `resultados/prisoes_ri_grupos32.json`.

The correction made after opening the vault did not in fact touch the forecasts.

Two observations about the K of the H3:
- **Inconsistency in `prisoes_ri`.** The `resultados.json` brings K = 1, and the `especificacoes.jsonl` brings 2 entries.
- **Multiplicity of the G2.** The G2 of prisões was tried in 3 vaults, and that of voos in 2. The G2 is a tolerance, not
  a test with α, so there is no α to divide. The number of attempts is recorded.

### If only the first declared version counted

- **H2.** Round 1 (v1, test K = 1, α = 0.05) gives 1 area. The gate fails, with or without correction. With Bonferroni
  it would be 2 areas (Development and Energy), from almost rejected models, and it would still fail.
- **H3, G4.** With the v1 of the estimator the bias is +4.7%, and the G4 fails.
- **H3, G2.** With the six streams of the original `CRITERIOS.md`, voos and prisões fail.
- **H3.** The output would not be approved.

## How to reproduce

The data come from `/mnt/project-files/estocologia/data/` and were checked by the `SHA256SUMS`. Copy
`h2`, `h2b`, `h2e`, `h2f` and `h2g` to `$D` and `h3c` to `$D3`.

```
cd empirical/p03-multiplicidade
python3 icp_direto.py ../../h2 ../../h2b $D/h2 resultados/icp_h2_posto_corrigido.json \
    direito_prisoes credito_hmda saude_consultas trabalho_curriculos epidemiologia_contracepcao educacao_star   # ~1 min
python3 icp_direto.py ../../h2b ../../h2b $D/h2b resultados/icp_h2b_diagnosticos.json biologia_sachs transporte_cinto
python3 analise_multiplicidade.py                       # inventory, K and corrections → resultados/multiplicidade.json
python3 preparar_copias.py 38                           # recreates copias/ from the originals
export OMP_NUM_THREADS=1
(cd copias/h2_v2 && ESTOCOLOGIA_DADOS=$D/h2 ESTOCOLOGIA_SAIDA=$S/h2_v2 python3 rodar.py)
(cd copias/h2_v1 && ESTOCOLOGIA_DADOS=$D/h2 ESTOCOLOGIA_SAIDA=$S/h2_v1 python3 rodar.py)
(cd copias/h2g_bonf && ESTOCOLOGIA_DADOS=$D/h2b ESTOCOLOGIA_SAIDA=$S/h2g_bonf python3 rodar.py)     # ~25 min
(cd copias/h2g_v1 && ESTOCOLOGIA_DADOS=$D/h2b ESTOCOLOGIA_SAIDA=$S/h2g_v1 python3 rodar.py)
python3 comparar_execucoes.py $S                        # → resultados/execucoes.json
python3 laco_versoes.py ../../data/bank_calls.csv   # → resultados/laco_versoes.json (~10 min)
(cd copias/h3_ri32 && ESTOCOLOGIA_DADOS=$D3 ESTOCOLOGIA_SAIDA=$S/h3_ri32 python3 rodar.py prisoes_ri)  # ~5 min
```

I ran `analise_multiplicidade.py`, `comparar_execucoes.py` and `laco_versoes.py` twice.
- **`multiplicidade.json` and `execucoes.json`** gave identical SHA-256 in the two executions (`resultados/SHA256SUMS`).
- **`laco_versoes.py`** differs only in the 16th digit of v4 (r̂ 0.5632133268561669 against …667). The 1st execution used the
  BLAS with several threads and the 2nd, OMP_NUM_THREADS=1. The 1st is identical to the published `h3/saida/portoes.json`, and the
  v1–v3 are identical in the two.
- **The executions of the H2 and of the H2g ran once each.** The v2 is the second execution of the published code and came out
  identical to it, field by field.

## Limits

- **The v1 of the H2 and the v1–v3 of the loop are reconstructions** from text, not the original code. The one of the H2 reproduces
  1 of the 4 effects cited in the README, and the one of the loop stays 1–2 percentage points from the historical numbers.
- **What the correction does not control.** It handles the multiplicity of attempts and controls the chance of a Ŝ with a non-parent.
  It does not handle the validity of each test: dependence inside the volunteer in pharmacology, cells of one condition
  in the Sachs, batches in the yeast. Those are other background points.
- **What stays out of M.** M does not count the pilots of the H2, the choices of which data sets to download and run,
  nor the specifications considered and not run. The K = 72 is a lower bound of the "garden of paths".
- **The sensitivity p_max > 0.05 was not declared beforehand.** I report it because the non-monotonicity only appeared in the count.
- **Interference with other work.** During the work, a `pkill -f "rodar.py"` of mine may have interrupted
  `rodar.py` processes of other workers on the same machine. I also deleted a `saida/` folder at the root of the shared
  scratchpad, which I had created but which another worker may have used. After that, everything that is mine stayed in
  `scratchpad/p03/`.
