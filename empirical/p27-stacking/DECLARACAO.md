# Declaration of point 27: combining models when none is true

**Written on 2026-10-19, before the instrument it declares was first run on the configuration below.** Its SHA-256 and
that of `crates/stochology-book/src/stacking.rs` enter `DECLARACAO.sha256` and are committed before
`cargo run --release --bin labs -- --stacking` is first run. The unit tests ran before the seal: the sequential
predictives give the closed-form marginal likelihood and do not depend on the order of the data; the stacking fixed
point reaches the maximum of the leave-one-out score over a grid of weights; a model listed twice doubles its prior mass
in BMA, exactly. One development change is declared: the prior standard deviation of the candidates' centres was 0.5
when the code was first written, and was set to 0.05 before any run, on reading that with 40 points a prior of 0.5
lets the data pull both centres toward the middle, so that neither candidate would be a confident expert on one side.
By the owner's go-ahead of the same day on item 18 of `docs/DECLARED_PROPOSALS.md`, part (c). Written with an AI
assistant.

## 0. Where the question comes from

Yao, Vehtari, Simpson and Gelman (2018) argue that when the true process is not among the candidates (the M-open
setting, which Chapter 4 calls Knightian in its first sense), Bayesian model averaging is the wrong tool for combining
predictions: it asks which model is true, and converges to one of them; and its weights depend on how many times a
model is listed. Stacking chooses the weights by a proper scoring rule on leave-one-out predictions. This point tests
the two claims in the smallest setting where both can show.

## 1. What is frozen

- **Candidates:** normal with spread 1 and centre c, with c ~ N(−2, 0.05²) (left) or c ~ N(+2, 0.05²) (right);
  conjugate, so every predictive density is exact.
- **Worlds:** M-open, each point from N(−2, 1) or N(+2, 1) with chance one half; M-closed, every point from N(−2, 1).
- **Sizes:** 40 training points and 2,000 test points per dataset; 1,000 datasets per world; seed 2026 (M-open) and
  2027 (M-closed).
- **BMA:** equal prior probability per listed model, weights by marginal likelihood.
- **Stacking:** the weights that maximise the sum of the log leave-one-out predictive densities, by the fixed point
  w_k ← mean_i w_k p_ki / Σ_j w_j p_ji, 500 iterations from equal weights.
- **Score:** the mean log predictive density per test point of the weighted mixture of full-data predictives.
- **The duplicate:** the left model listed twice, three candidates, everything else equal.

## 2. The rules

- **R1.** In the M-open world, stacking scores above BMA, with the 95% interval of the mean paired difference over the
  1,000 datasets entirely above zero.
- **R2.** In both worlds, listing the left model twice moves stacking's mean score by less than 0.001 per dataset on
  average, and the total weight of the left family by less than 0.01.
- **R3.** In the M-closed world, the mean log scores of the two differ by less than 0.01.

Each rule holds or fails as printed. Any change after the run is a new declaration, counted in K.

## 3. What will be reported

The two blocks and the three verdicts, verbatim, in `saida.txt` and in the registry; the reading in Chapter 8, where
proper scores move from judging forecasts to combining them.
