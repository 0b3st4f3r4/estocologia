# Declaration 2 of point 7 (ICP): null rate of the strict validation of genetics

Written on 2026-10-15T19:16:49Z, before running any permutation of this declaration. The SHA-256 of this file lives in
`DECLARACAO_2.sha256`. It does **not** rewrite `DECLARACAO.md` nor the hash `05ab9ff1…`: it adds a null rate to the
number that the replication of genetics already used to decide. It is a new declaration and counts in K.

## What was already run

When this declaration was written, point 7 was already run and reported in `README.md`: of the 40 new Y genes, the ICP
gave 11 non-empty Ŝ, **10** of them with at least one gene outside `plate`, 27 pairs (Y, x) in all; the deletion worked
in 26 of the 27 (the exception is n27, x2 = YMR042W); **1 gene in 27 passes the strict validation** — n34 (Y = YLR354C),
x6 = YIL033C — and the y08 control has 0 of 3. The limit already written is that the strict validation decides with one mutant
row (n = 1), and the number 1 in 27 stood **without a null rate**. It is that hole that this declaration closes.

## Structure check, made before writing this

Without looking at outcomes beyond what is already published above: the parquet `data/h2f/kemmeren2014_obs_int.parquet` has
**1,741 rows** (262 observational + 1,479 interventional), of which 1,479 have `deleted_gene` filled in;
**none of the 10 Y** of the 27 pairs has a row with `deleted_gene` equal to it, so for every pair the set of rows
is the same, the 1,741; each one of the 27 genes X and each one of the 10 Y has **unique minimum and maximum** (no tie at the
extreme); each pair has **exactly one** mutant row. That is what fixes the draw below.

## The null (decides)

**Swap of the mutant label.** For each pair (Y, x) the mutant label ceases to be in the row where gene X was
deleted and starts to be in a row drawn uniformly among the 1,741. Under the null the label carries no information
about Y, so the labelled row is exchangeable with any other.

Draw, fixed here and not changed afterwards:

```python
rng = np.random.default_rng(20261015)          # declared seed
I = rng.choice(1741, size=(999, 27), replace=True)   # I[b, j] = mutant-labelled row of pair j in draw b
```

Column j is pair j; row b is draw b. The 27 pairs stay in the order in which they appear in `saida/genetica.json`:
the 10 labels of `resumo.y_com_S_hat_com_gene` in the order of `por_y` (n01, n03, n04, n09, n12, n27, n29, n34, n37,
n40) and, within each one, the order of the `validacao` list. Draws independent between pairs and between draws.

**Criterion for a pair (Y, x) with gene X = `gene` in the labelled row i*, identical to the strict criterion already declared,
evaluated whole over the permuted label:**

1. **deletion worked:** X[i*] ≤ minimum of X among the other 1,740 rows;
2. **Y outside the range:** Y[i*] < minimum of Y among the other 1,740 rows, or Y[i*] > maximum of Y among them.

Nothing of the observed stays fixed: both parts use the drawn row.

**Statistic**

- per draw b: T_b = number of pairs j that pass the strict criterion in `I[b, j]` (0 to 27);
- observed: T_obs, the same criterion applied to the real row of each pair (the row with `deleted_gene == gene`). T_obs is
  the number already published, 1. If the reproduced count of `saida/genetica.json` does not give the same T_obs, I stop and report.

**p and decision**

p = (1 + #{b : T_b ≥ T_obs}) / (999 + 1), α = 0.05. A single p decides; there is no correction per pair because the statistic
already sums the 27 pairs. T_obs is improbable under the null if p ≤ 0.05.

**Secondary (informs, does not decide).** From the same matrix `I`, without a new draw: T_deleção (only part 1) and T_faixa
(only part 2), with the same p. They decompose the strict criterion. This is already known, and it stays written before running:
since part 1 is a fact of the data — deleting X removes the expression of X, the minimum of column X is in the mutant row —
permuting the label **destroys** that part, and it is to be expected that almost all the p comes from it. The secondary one is also what
says how much the null is worth without part 1.

## What does not change

- The draw of the 40 Y, the α per Y, the pre-selection, the environments, the ICP, the Ŝ, the strict validation criterion and the
  **replication criterion** stay exactly as in `DECLARACAO.md`: "Genetics replicates if at least one of the 40 new Y
  gives an Ŝ with a gene and at least one gene of those passes the strict validation". **The replication criterion does not change.**
- The 27 pairs and T_obs are **read** from `saida/genetica.json`, not recomputed; and no file is rewritten.
- The y08 control (0 of 3) does not enter: it does not belong to the 27 pairs of the 10 new Y. The validation of y08 continues as
  H2f published it.
- The null is not a resampling of the ICP nor of the Ŝ: it conditions on the 27 pairs already obtained. It measures "how many pairs pass
  by chance, drawing the mutant row", not "how many Ŝ would have a gene in a null world".

## What counts as a failure

Changing B, the seed, the draw, the statistic, the criterion, the order of the pairs or the set of rows after seeing
T_b; using a pool of rows different from the 1,741; recomputing the 27 pairs or T_obs in another way. Any change
like that enters as a new declaration and counts in K. Not reproducing T_obs = 1 is a failure of execution: I stop and report.

## How to reproduce

```
cd empirical/p07-icp/codigo
OMP_NUM_THREADS=1 python3 nulo_validacao.py      # reads <repo>/data/h2f/kemmeren2014_obs_int.parquet
```

Output: `../saida/nulo_validacao.json`, with T_obs, the mean and the 95 quantile of the null, the p, the two secondaries and the
seed. The script writes nowhere else.
