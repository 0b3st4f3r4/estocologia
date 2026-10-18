# Point 7 (ICP): clustering-robust version and replication of genetics

## Result

The H2g ICP test treats each row as independent. In the FDA trial there are 16 instants per volunteer and in
STAR there are 80 schools with up to 403 students. **In the two areas that depend on clustered rows, the result splits:**

- **Pharmacology survives, and is solid.** The Ŝ is {c_dof, c_qui, c_ran} in the three tests — original, Wald clustered by
  volunteer (CR1) and block permutation — and it contains neither `pr` nor `tamp`, as the reference truth declared
  beforehand asks. The intersection of the accepted sets is not empty in any of them. What shrinks is the evidence: the p of the empty
  set goes from 6.3e-104 in the original test to 2.2e-15 in the clustered one and 0.002 in the permutation, that is, from about 340
  bits against the empty set to 49 and 9. Taking the within-volunteer correlation seriously does not erase the finding; it cuts
  the account that announced it by seven orders of magnitude.
- **Education does not survive.** With the Wald clustered by school (80 clusters, 446 of the 11,598 students change school), the
  ICP accepts 266 sets out of 512, but the **intersection is empty**: Ŝ = ∅. The block permutation (school × environment,
  513 blocks, B = 999) accepts 512 and also gives Ŝ = ∅. The published set, {anos_pequena_antes, turma_pequena},
  has p = 0.76 in the clustered test: it ceases to be accepted. By the declared rule — "Education survives if the Ŝ of the
  main test is not empty" — Education **drops out**.

**Consequence for the H2g gate.** Physics (tunnel of light) is not part of this point and enters as it is, with the
Ŝ = {blue, green, red}. The replication of genetics ran (its own section below): the ICP of the 40 new Y genes hits one gene
in the strict validation by intervention — with null rate by permutation of the mutant label of **p = 0.001**, the floor of the
test with B = 999 (its own section below) —, and by the declared rule Genetics **replicates**. With Education out and
Genetics replicated, the areas are **three: Physics, Pharmacology and Genetics**. The PLAN criterion asks for Ŝ ≠ ∅ in 3
distinct areas, so the **H2g gate stands with 3 areas** — without Education and without the set that sustained it, and with
genetics sustained by a new domain, no longer by y08 alone. That was the case at stake: if no new Y had
given an Ŝ with a validated gene, the gate **would fall from 4 to 2 areas and the transfer between areas would cease to be
approved** by that criterion.

| area | test | Ŝ | accepted | p of the published Ŝ | p of the empty |
|---|---|---|---|---|---|
| Pharmacology | original (H2g) | {c_dof, c_qui, c_ran} | 35 | 0.304 | 6.3e-104 |
| Pharmacology | **CR1, cluster = volunteer (22)** | {c_dof, c_qui, c_ran} | 61 | 1.0 | 2.2e-15 |
| Pharmacology | block permutation (B = 999) | {c_dof, c_qui, c_ran} | 64 | 0.876 | 0.002 |
| Education (STAR) | original (H2g) | {anos_pequena_antes, turma_pequena} | 57 | 0.039 | 1.6e-37 |
| Education (STAR) | **CR1, cluster = school (80)** | **∅** | 266 | 0.760 | 1.6e-06 |
| Education (STAR) | block permutation (B = 999) | **∅** | 512 | 1.0 | 0.002 |

What "Ŝ = ∅" means here: it is not that the small class has no effect. It is that, with 80 schools as de facto independent
units, the sets that the test cannot reject have no variable in common — the invariance
test loses the power to point to a set, which is the only product it delivers.

## Replication of genetics: 40 new Y genes (α = 0.00125 per Y)

Genetics has no clusters — the sample is a culture of its own and the only available label is `plate`, with two levels —
so its correction is not the clustered Wald but the declared replication: 40 new Y genes drawn among the never
deleted ones, apart from the 10 of H2f (`default_rng(7007)`, pool of 4,681), each with the same pre-selection of the 9 genes
deleted with the largest |correlation| in the wild-type, the same environments (obs, del_EJ, del_KP; vault del_AD out) and the
original test of H2g. The α per Y is 0.05/40 = 0.00125. It ran on 2026-10-15, `OMP_NUM_THREADS=1`, in 310.2 s of wall time.

| status of the 40 new Y | n |
|---|---|
| Ŝ ≠ ∅ | 11 |
| — Ŝ with at least one gene (outside `plate`) | **10** |
| — Ŝ with only `plate` (n36, YLR023C) | 1 |
| Ŝ = ∅, with the empty set accepted (low power) | 11 |
| Ŝ = ∅ because the accepted sets are disjoint | 13 |
| no non-saturated set accepted | 5 |

The 11 Y whose Ŝ is not empty (genes of the Ŝ by the pre-selection index, in the Y of that domain):

| Y | label | Ŝ | genes of the Ŝ | accepted |
|---|---|---|---|---|
| YER170W | n01 | {x1, x4, x6} | x1 YPL123C, x4 YNL186W, x6 YBR201W | 6 |
| YBR063C | n03 | {plate, x1, x4, x5, x6, x8} | x1 YDR359C, x4 YGL131C, x5 YDR422C, x6 YBR175W, x8 YHR167W | 2 |
| YNL146W | n04 | {x9} | x9 YGL216W | 222 |
| YKR062W | n09 | {plate, x3} | x3 YMR191W | 11 |
| YJR118C | n12 | {plate, x3, x5, x9} | x3 YHR032W, x5 YDR392W, x9 YGR178C | 4 |
| YBR229C | n27 | {plate, x2, x7} | x2 YMR042W (the deletion did not work), x7 YMR304W | 36 |
| YJR098C | n29 | {plate, x1, x3, x4, x6, x8} | x1 YBR082C, x3 YBR062C, x4 YNL273W, x6 YMR255W, x8 YML099C | 1 |
| YLR354C | n34 | {x1, x4, x6} | x1 YBR225W, x4 YDR001C, **x6 YIL033C** | 66 |
| YLR023C | n36 | {plate} | — | 17 |
| YLR057W | n37 | {plate, x3, x4, x6} | x3 YGR271C-A, x4 YJL098W, x6 YMR269W | 23 |
| YAR007C | n40 | {plate, x9} | x9 YDL101C | 66 |

**Validation by intervention** (Meinshausen et al. 2016, the declared criterion), in the 27 genes of the 10 Ŝ with a gene:

- The deletion worked in 26 of the 27. The exception is n27, x2 = YMR042W: in the mutant ΔYMR042W YMR042W itself is not the smallest
  value among all rows.
- **One gene passes the strict validation: n34 (Y = YLR354C), x6 = YIL033C.** The deletion worked and, in the mutant
  ΔYIL033C, the expression of YLR354C is the **minimum of the 1,741 rows** (quantile 0.000, that is, outside the range of all the
  other samples), in the direction predicted by the positive coefficient (+0.053).
- Weak coherence, which alone does **not** count: n03 x4 = YGL131C (quantile 0.016, coherent sign, but inside the range) and
  n34 x6 = YIL033C itself. No other gene comes close: the remaining quantiles lie between 0.055 (n34, x1 =
  YBR225W) and 0.964 (n03, x6 = YBR175W).

**y08 control (the H2f domain, with α = 0.005).** Reproduces the published one: Ŝ = {plate, x1 = YKL009W, x5 = YNL023C,
x9 = YPL193W}, 2 accepted sets out of 1,024, best p = 0.00724 (H2f: 0.0072). The validation repeats the H2f table
entirely: the deletion works in the three, none stays outside the range (quantiles 0.0557 / 0.855 / 0.983) and **0 of the 3** genes
passes the strict validation.

**Null rate of the strict validation (declaration 2).** The number that decides the replication was 1 gene in 27, without a
companion p. [DECLARACAO_2.md](DECLARACAO_2.md) (hash in `DECLARACAO_2.sha256`, `dbcb13f8…`), written before
running, fixes the null: permute the mutant label among the 1,741 rows, per pair (Y, x) of the 27, B = 999, seed
20261015, statistic = number of pairs that pass the whole strict criterion (deletion worked **and** Y outside the range
of the other rows), p = (1 + #{null ≥ observed})/(B + 1), α = 0.05. It **does not change the replication criterion**. Source:
`saida/nulo_validacao.json`, 0.5 s.

| statistic | observed | null mean | null q95 | null maximum | p |
|---|---|---|---|---|---|
| pairs that pass the strict criterion (**decides**) | 1 | 0.00000 | 0 | 0 (0 of 999 draws) | **0.001** |
| only "the deletion worked" (informs) | 26 | 0.01502 | 0 | 1 | 0.001 |
| only "Y outside the range" (informs) | 1 | 0.03003 | 0 | 1 (30 of 999 draws) | 0.031 |

The p of the row that decides is the **floor** of 999 draws, 1/1,000: none of the 999 draws reached 1 pair, and the expected
count under the null is 27 · (1/1,741) · (2/1,741) ≈ 1.8e-5. With α = 0.05, the number that sustains the replication survives:
1 pair in 27 is not what the randomness of the label produces. What the p does **not** authorize is reading that as an effect measured in one
row: the first half of the criterion is a fact of the data — deleting X removes the expression of X, so the minimum of column X
is in the mutant row — and permuting the label **destroys** that half, which is where almost all the p comes from. The second
half, alone, stops at p = 0.031, just below α: the "Y at the minimum of the 1,741" is marginal by itself. The strong evidence
is against "the label fell there by chance"; the evidence about the effect remains that of 1 mutant row.

**Verdict by the declared rule.** There was an Ŝ with a gene (10 of the 40 new Y) and one gene of those — n34, x6 = YIL033C — passes
the strict validation, with p = 0.001 against the randomness of the label. Genetics **replicates** and counts as an area. It is not the
"fragile" case: weak coherence alone decides nothing here, and the gene that decides passes the strict criterion, not
the weak one.

## What was done

1. **Declaration before running.** [DECLARACAO.md](DECLARACAO.md), with the hash in `DECLARACAO.sha256`, written before
   any ICP in this folder. It fixes what does not change (code copied from `h2g/`, data, training environments,
   candidates, encoding, α, saturation K = 3), the main test (CR1 Wald in two parts — location by ranks
   and Brown–Forsythe scale — with Bonferroni of the two), the secondary one (permutation of whole blocks), the clusters
   of each area (volunteer, school, own sample in genetics), the survival criteria and what counts as
   failure: not reproducing the published Ŝ, or changing the test after seeing the result.
2. **Reproduction first.** The original test reproduces, in the two areas, the Ŝ, the number of accepted sets and the best p
   published in `h2g/` (fields `original_funcao_h2g` of `saida/farmacologia.json` and `saida/star.json`).
3. **Only then** the two robust tests, over the same residual u = r / ŝ of `p_invariancia_np`.
4. **Genetics came last**, with `Kemmeren.RData` and the derived parquet in place: the ICP of the 40 new Y with the
   original test of H2g, α = 0.00125 per Y, the validation by intervention of each gene of the Ŝ and y08 as control.
5. **The null rate of the number that decides came afterwards, as a new declaration.** [DECLARACAO_2.md](DECLARACAO_2.md)
   (`dbcb13f8…`, written before running, without rewriting the first one) fixes the permutation of the mutant label, B = 999,
   seed 20261015 and the counting statistic, and keeps the replication criterion; `codigo/nulo_validacao.py` runs that
   in 0.5 s over `saida/genetica.json` and the parquet, and writes `saida/nulo_validacao.json`. Two runs gave the same
   file, apart from the `segundos` field.

## Reproduction check

I ran the two areas again on 2026-10-15 (OMP_NUM_THREADS=1, `P07_SAIDA` outside the repository) and compared with the
recorded outputs: Ŝ, number of accepted sets, verdict of each test, best set and p of the empty match in the two areas.
The only two differences are the last digit of the p of the empty of CR (2.1569841841513456e-15 against
2.1569841841514194e-15 in pharmacology, and 1.6350118112824874e-06 against 1.63501181128208e-06 in STAR), which is
floating-point rounding. The wall time, however, dropped a lot: 12.4 s and 213.0 s, against the 419.4 s and
2,333.1 s of the first run, which ran on a machine shared with other jobs. The numbers do not change; the
cost changes.
## Limits

- **Genetics ran, but the derived parquet does not match the reference byte for byte.** `Kemmeren.RData` matches
  the expected one (86,328,652 bytes, sha256 `08e95f39…`), and the conversion to
  `data/h2f/kemmeren2014_obs_int.parquet` matches in **content**: 1741 rows × 6175 columns (5 of metadata
  in the order `sample_id`, `kind`, `deleted_gene`, `deleted_gene_col_index_1based`, `plate`, plus the 6170 genes by
  ORF), observational (262 × 6170) and interventional (1479 × 6170) blocks identical bit for bit to the `RData`, genes in
  float64 and without transformation. The sha256 of the file, however, is `f152937f…`, and not the reference `0921bda4…`: the byte
  for byte of the Parquet carries the writer version (`parquet-cpp-arrow 26.0.0`, pandas 3.0.5), and none of the alternative
  encodings tested for the metadata columns (index as NA or −1; `plate` as bool or integer; `kind` as
  text or category; genes in file order or sorted; dataframe index written or not) reproduces that
  hash. The replication ran over the checked content, not over the hash.
- **The strict validation of genetics decides with one row.** Each mutant is an environment of n = 1, so the criterion
  "ΔX worked and Y falls outside the range of all the other rows" is a comparison of one observation against 1,740. The
  gene that sustains the replication (n34, x6 = YIL033C, with Y = YLR354C at the minimum of the 1,741) passes that criterion, and the
  original declaration did not fix a null rate for it: 1 gene in 27 is the number. Declaration 2 fixed it: p = 0.001, which is the
  **floor** of the test with B = 999, because no draw reached 1 pair. The p buys no resolution and does not remove the limit: with
  n = 1 per mutant, the smallest possible p here is worth 1/1,000, and the null by label permutation destroys the half "the deletion
  worked" — which is a fact of the data, not a hypothesis —, so almost all the p comes from it (the half "Y outside the range" alone
  gives 0.031). The null is also conditional on the 27 pairs already obtained from the ICP: it does not carry the selection of the 40 Y nor the
  choice of the Ŝ. `plate` is not validatable by intervention (it is not a deleted gene).
- **STAR has clusters that intersect.** 446 students go through more than one school, which links two clusters; and the
  blocks of a school have different sizes, so the exchange is only approximately valid, as the declaration says.
- **The test that decides is the main one.** The permutation enters as information, not as decision.
- **Genetics has no clusters.** The sample is a culture of its own, and the only label is `plate`, with two levels, which
  do not serve as clusters; that is why its correction is the replication, and not the clustered Wald.

## How to reproduce

With the data of `data/h2b` in place (the FDA trial and STAR are in the repository). Genetics comes from
`data/h2f/Kemmeren.RData` (outside Git; origin and sum in `data/h2f/FONTES.md` and `SHA256SUMS`) to the
derived parquet, with `rdata` 1.1.0 and pandas 3.0.5:

```
cd empirical/p07-icp/codigo
python3 construir_parquet_h2f.py                                    # RData → data/h2f/kemmeren2014_obs_int.parquet
ESTOCOLOGIA_DADOS=<repo>/data/h2b OMP_NUM_THREADS=1 python3 icp_robusto.py farmacologia star genetica
OMP_NUM_THREADS=1 python3 nulo_validacao.py          # null rate of the "1 gene in 27", 0,5 s
```

On a machine shared with other jobs, the first run took 419.4 s (pharmacology) and 2,333.1 s (STAR),
which are the values of the `segundos` field of the outputs; with the machine free, 12.4 s and 213.0 s. Genetics took 310.2 s
with the machine free. It depends on numpy, pandas, scipy and pyarrow — and, only for the conversion of the RData, on the `rdata` package. Without an
argument, the script tries the three areas.
