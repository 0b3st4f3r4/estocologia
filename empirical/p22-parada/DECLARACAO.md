# Declaration of point 22: a stopping rule that survives the path

**Written on 2026-10-18, before any recomputation of a past gate.** The SHA-256 of this file is in
`DECLARACAO.sha256`. A change after that enters as a new declaration and counts in the K.

## 0. Provenance

It comes from the outside review of 2026-10-18 requested by the owner (`empirical/p16-ponte/DECLARACAO_3.md` §0
describes the reviewer and why it is not an independent specialist).

## 1. The problem

Every round of the H2 gate was declared with a hash before running. That makes each step honest. It does not make the
path honest: "run another round until three areas pass" is optional stopping, and the stopping rule itself was never
declared. Bonferroni over 38, applied after the path was known, divides α by a number that the path chose. The
background audit already measured the consequence: if only the first declared version of each choice counted, neither
H2 nor H3 would be approved.

## 2. The rule, from this declaration on

1. A gate that is attempted in more than one round keeps a **ledger** (`crates/stochology-methods/src/sequential.rs`,
   type `Ledger`), and each round is recorded in it **before** the next round is designed.
2. **α is declared before round 1** and does not move. Default: 0.05.
3. Each round contributes an **e-value**: its own likelihood ratio when it has one, otherwise its p-value through the
   mixture calibrator $e = (1 - p + p\ln p)/(p\,(\ln p)^2)$, which needs no tuning parameter.
4. **Merge.** The running **product** when the round runs on data that no earlier round used, declared per round; the
   running **mean** when any data are reused, because the mean of e-values is valid under any dependence.
5. **The gate passes at the first round in which the accumulated evidence reaches 1/α.** By Ville's inequality this
   holds at any stopping time, so no correction is applied afterwards.
6. **K is still counted and reported.** The ledger makes the count harmless; it does not make it optional.

## 3. What the rule does not cover

A round that **changes the null** changes the object under test. The product is valid only over rounds that test the
same null hypothesis — for H2, "the method transfers to no area" — and a round that redefines the area, the
environment, or the outcome must say which null it tests. If two rounds test different nulls, they belong to different
ledgers.

## 4. The retroactive question, declared and not done

The 38 tests of the H2 gate were recorded in heterogeneous forms, and part of the raw data left the tree with the
Python scaffolding. Running the ledger over them now would mean choosing, while computing, which rounds are fresh and
which null each one tests. That is exactly the freedom this point removes. Recomputation requires first a second
declaration, with its own hash, listing for each of the 38 tests its p-value or e-value, the data it used, and the null
it tests. It enters `docs/DECLARED_PROPOSALS.md` as item 15, a desk task that needs no people.

## 5. Proof

`cargo test -p stochology-methods`, nine tests in `sequential.rs`: the calibrators integrate to at most 1 under the
null; they are monotone and never negative; a zero p-value is clamped and never trusted; over 4 000 null paths of 40
rounds each, the product crosses 1/α at most α of the time with three binomial deviations of margin; over the same
kind of path with seven rounds, the naive rule "stop when p < α" passes at about $1 - 0.95^7$ while the ledger stays
at or below α; a real signal accumulates and crosses; a weak round costs evidence in the product and not in the
bound; the mean resists duplicated rounds that the product does not; invalid e-values are recorded as zero and the
ledger is deterministic.

## 6. K

**K = 1.**
