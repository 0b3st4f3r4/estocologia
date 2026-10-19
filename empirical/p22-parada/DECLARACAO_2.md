# Declaration 2 of point 22: the H2 gate under the e-process, retroactively

**Written on 2026-10-19, before the recomputation it declares.** Its SHA-256 enters `DECLARACAO.sha256` before the
instrument that applies it is run. It closes item 15 of `docs/DECLARED_PROPOSALS.md` as desk work; it is not an
audit from outside (item 17), because it was written with an AI assistant, as the readings of 2026-10-18 and 10-19.

## 0. What enters

The **38 distinct tests** of the H2 gate, exactly as `empirical/p03-multiplicidade/resultados/multiplicidade.json`
records them (the 62 rows whose `repete` is empty), copied without change into `testes.tsv` beside this file with
their round, domain, area, declared null test, number of subsets, published Ŝ and the per-domain p-value `p_dom`
defined by point 3: $p_{\text{dom}} = \min_j \max_{S \not\ni j} p_S$, a valid p-value for the error "Ŝ contains a
non-parent". Nothing is recomputed from raw data, part of which left the tree on 2026-10-16; the ledger reads the
recorded p-values.

## 1. The null, per ledger

One ledger per **area** (17 areas in the record). Its null is: *in this area, the method returns no non-empty
invariant set made of parents only*. The gate's own rule, declared before round 1 of H2, asked for three areas with a
non-empty set; under the e-process the gate passes when **three or more area ledgers** have crossed $1/\alpha$,
with $\alpha = 0.05$ as declared in the first declaration.

## 2. The e-value of a round

- A test whose published Ŝ is **non-empty** makes the claim the null denies; its e-value is the mixture calibrator of
  its $p_{\text{dom}}$, $e = (1 - p + p\ln p)/(p(\ln p)^2)$, with $p$ clamped to $[10^{-12}, 1]$ as `sequential.rs` does.
- A test whose published Ŝ is **empty** or that **rejected every set** makes no claim; its p-value is 1 and its
  e-value is the calibrator at 1, which is $1/2$. A round that found nothing costs evidence; it does not add any.
- A test whose non-empty Ŝ was **accepted only by saturation** (the rule of round 4 of H2: a set whose columns
  predict the environment has no power) makes no claim either. The record marks one such test, `saude_rand` in
  round H2c, and no other.

## 3. The merge

Within an area the rounds reuse data (the same dataset tested with a new ruler, a new rule or a new target), so the
ledger of every area uses the **mean** of its e-values, which is valid under any dependence. The product, which
would be valid only over fresh data, is not used anywhere in this recomputation. Rounds enter in the order of the
record (H2, H2b, …, H2g); the verdict is read at the last round, and the mean makes the order immaterial.

## 4. Two readings reported side by side, declared here

- **As recorded**: the 38 tests with their recorded $p_{\text{dom}}$.
- **With dependent rows corrected**: the same ledgers, with the published set of Education replaced by its
  clustered-by-school p-value, $0.76$, and the set of Pharmacology by its clustered-by-volunteer p-value,
  $2.2 \times 10^{-15}$, both from `empirical/p07-icp/README.md`; the other areas have no clustered test on record and
  keep their values. These two numbers enter the table as two extra rows, flagged `clustered`, and the instrument
  prints both readings.

## 5. What this does not do

It does not change the gate's verdict in the book, which stays as Chapter 19 and Appendix E say; it adds the reading
that the stopping rule, declared after the path, would have given. It does not test a new null, so **K does not move**:
the recomputation applies a rule already declared to tests already recorded. It does not decide which reading is the
right one; the book reports both.

## 6. Proof

`labs --eprocess` reads `testes.tsv` and prints, per area, the number of tests, the mean e-value, the path of the
mean, and whether it crosses $1/\alpha$; then the count of areas under each reading. The instrument is deterministic
and reads no seed.
