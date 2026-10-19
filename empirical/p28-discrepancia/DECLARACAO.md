# Declaration of point 28: calibrating is not understanding

**Written on 2026-10-19, before the instrument it declares was first run on the configuration below.** Its SHA-256 and
that of `crates/stochology-book/src/discrepancy.rs` enter `DECLARACAO.sha256` and are committed before
`cargo run --release --bin labs -- --discrepancy` is first run. The unit tests ran before the seal: the matrix inverse
returns the identity; a noiseless line is recovered without discrepancy; with the free discrepancy, theta and the
linear term of the discrepancy get equal posterior means that add up to the slope. One computation was made before the
seal, on the noiseless world and not with the instrument: the least-squares slope through the origin of the true curve
on the 30 design points is 0.5647, against the true 0.65, with a root mean square misfit of 0.047. On reading it, the
noise standard deviation was set at 0.05 (it had been 0.01 when the code was first written), so that the misfit is of
the order of the noise, the case where a fit "looks fine". By the owner's go-ahead of the same day on item 18 of
`docs/DECLARED_PROPOSALS.md`, part (d). Written with an AI assistant.

## 0. Where the question comes from

Kennedy and O'Hagan (2001) wrote reality as simulator plus discrepancy plus noise. Brynjarsdóttir and O'Hagan (2014)
showed what happens without the discrepancy term (the parameters absorb the structural error and are learnt wrong,
with intervals that do not cover the truth), and what it takes for the term to help: prior knowledge of what the
discrepancy may not do; a discrepancy free to do anything makes the parameters non-identifiable.

## 1. What is frozen

- **World:** y = 0.65 x / (1 + x / 20) + noise, noise N(0, 0.05²); 30 points at x = 4i/30, i = 1 to 30.
- **Simulator:** y = θ x.
- **Three fits**, each a Bayesian regression with known noise and independent N(0, 1) priors on every coefficient:
  without discrepancy; with d(x) = b2 x² + b3 x³ + b4 x⁴ (d(0) = d′(0) = 0); with d(x) = b1 x + b2 x² + b3 x³ + b4 x⁴.
- **Readings per fit**, averaged over 1,000 datasets, seed 2026: the posterior mean and standard deviation of θ, the
  share of datasets whose 95% interval (mean ± 1.96 sd) covers 0.65, the root mean square error of the posterior-mean
  fit on the data, and the error of its prediction at x = 8, twice the range.

## 2. The rules

- **R1.** Without discrepancy, the interval of θ covers 0.65 in fewer than half of the datasets, while the root mean
  square error of the fit in range is below 1.5 noise standard deviations.
- **R2.** With the discrepancy that knows d(0) = d′(0) = 0, the coverage is at least 0.90.
- **R3.** With the discrepancy free at the origin, the mean posterior standard deviation of θ is at least five times the
  constrained one.

Each rule holds or fails as printed. Any change after the run is a new declaration, counted in K.

## 3. What will be reported

The block and the three verdicts, verbatim, in `saida.txt` and in the registry; the reading in Chapter 14, where a
model is failed on purpose.
