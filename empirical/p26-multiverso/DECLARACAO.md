# Declaration of point 26: the multiverse of the health post

**Written on 2026-10-19, before the instrument it declares was first run on the configuration below.** Its SHA-256 and
that of `crates/stochology-book/src/multiverse.rs` enter `DECLARACAO.sha256` and are committed before
`cargo run --release --bin labs -- --multiverse` is first run. One irregularity of order is declared here: the source of
the instrument entered the tree in commit `8f52ec2`, before this declaration, swept in by the commit of point 25; it
had not been run, and the hash sealed below is of the file as it stands now. The unit tests ran before the seal: the
M/G/1 wait at unit coefficient of variation equals the M/M/1 wait, the lognormal keeps the declared mean and standard
deviation, the main effects of a balanced design add up to one, and the one specification already published (Chapter
5, `labs --propagate`, P = 0.0499) is reproduced draw for draw. None of the other seventeen was run. By the owner's
go-ahead of the same day on item 18 of `docs/DECLARED_PROPOSALS.md`, part (b). Written with an AI assistant.

## 0. Where the question comes from

Breznau and colleagues (2022) gave the same data and hypothesis to 73 teams; their estimates spread to opposite signs,
and more than 95% of the variance stayed unexplained after coding every analytic choice. The uncertainty budget of
Chapter 5 splits the variance of the wait between the arrivals, the pace and their covariance, under one
specification. It has no share for the choices that made the specification. This point measures that share on the
health post, where the choices can be enumerated.

## 1. What is frozen

- **Question:** P(W > 20 min), the share of days whose mean wait passes 20 minutes; decision "second nurse" when it
  passes the promise of 0.05. The rates as in Chapter 5: arrivals of mean 8 and standard deviation 0.6 per hour, pace
  of mean 12 and standard deviation 0.8.
- **The three choices and their levels** (3 × 2 × 3 = 18 specifications, all equally weighted):
  - service law, through its coefficient of variation c ∈ {0.5, 1.0, 1.5}, with the Pollaczek–Khinchine wait
    W = 60 λ (1 + c²) / (2 μ (μ − λ)) minutes (c = 1 is the M/M/1 of the book);
  - law of the rates: normal, or lognormal with the same mean and standard deviation;
  - correlation of the underlying normals: −0.5, 0, +0.5.
- **Draws:** 400,000 pairs of standard normals, seed 2026, shared by every specification (common random numbers),
  drawn in the order of the propagation laboratory; a day that never clears counts as above the limit.
- **Decomposition:** the between-specification variance of P, equal weights, split into the main effect of each choice
  (the variance of the means at its levels) and the interaction (the rest).

## 2. The rules

- **R1.** The 18 specifications do not agree on the decision: at least one says "second nurse" and at least one says
  "promise kept".
- **R2.** The coefficient of variation of the service, a choice the delta budget of Chapter 5 does not see, carries a
  larger main-effect share of the between-specification variance than the law of the rates and the correlation
  together.
- **R3.** The between-specification standard deviation of P is more than ten times the largest Monte Carlo standard
  deviation of a single specification: the spread is of choices, not of chance.

Each rule holds or fails as printed. A failure is reported as such; any change after the run is a new declaration,
counted in K.

## 3. What will be reported

The 18 rows, the decomposition and the three verdicts, verbatim, in `saida.txt` and in the registry; the reading in
Chapter 5, and the decomposition as the budget of the specification, the case for which contract 1.4 is declared.
