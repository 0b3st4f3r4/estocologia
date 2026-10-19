# Declaration of point 25: reliability is not average success

**Written on 2026-10-19, before the instrument it declares was first run on the configuration below.** Its SHA-256 and
that of `crates/stochology-book/src/passk.rs` enter `DECLARACAO.sha256` and are committed before
`cargo run --release --bin labs -- --passk` is first run. The unit tests of the module ran before the seal; they check
the estimator on hand-made counts, the Beta moments in closed form and the mean of the Gamma sampler on another seed,
and none of them runs the four worlds. By the owner's go-ahead of the same day on item 18 of
`docs/DECLARED_PROPOSALS.md`, part (a). Written with an AI assistant.

## 0. Where the question comes from

The research of 2026-10-19 on agent models found that benchmarks of agents report the average success over tasks,
pass^1, and that the ones that ran each task several times (τ-bench, Yao and colleagues, 2024) found the chance that
all of k runs succeed, pass^k, far below what the average promised. Under independence and equal chances, pass^k would
be (pass^1)^k. It is not, and the reason is arithmetic: pass^k = E[p^k] over the tasks, and E[p^k] ≥ (E p)^k with
equality only when every task has the same chance (Jensen). The question of this point is whether a laboratory of the
size such benchmarks use can see the difference, and whether the estimator they use is honest about it.

## 1. What is frozen

- **Tasks and runs:** N = 200 tasks, n = 8 runs of each.
- **Estimator:** per task, C(c, k) / C(8, k) from c successes; pass^k is its mean over the tasks, for k = 1 to 8.
- **Worlds:** the chances of the tasks drawn from
  - A: every task at 0.5;
  - B: Beta(2, 2), mean 0.5;
  - C: Beta(0.3, 0.3), mean 0.5, most tasks near 0 or near 1;
  - D: Beta(1.4, 0.6), mean 0.7.
- **Replicates:** 2,000 repetitions of the whole experiment, new chances and new runs each time; the 95% band is the
  2.5% and 97.5% order statistics. The first replicate is reported as "the first run", the one a single benchmark
  would publish.
- **Generator:** SplitMix64, seed 2026; Beta as the ratio of two Gamma draws (Marsaglia and Tsang).

## 2. What the arithmetic expects, before the run

From the closed form of the Beta moments (no simulation): pass^8 is 0.0039 in A; 0.0545 in B, 14 times the independent
reading 0.0039; 0.2672 in C, 68 times; 0.2976 in D, against the independent 0.0576. These are the targets, not the
results.

## 3. The rules

- **R1.** In world A, the band of the gap pass^k − (pass^1)^k contains zero for every k from 2 to 8: without spread in
  the chances, the laboratory does not invent a gap.
- **R2.** In worlds B, C and D, the band of the gap lies entirely above zero for every k from 2 to 8: with this many
  tasks and runs, the spread of the chances is seen at every k, including k = 2, where it is smallest in B (0.05).
- **R3.** In every world, the exact E[p^k] lies inside the band of the estimator for every k: the estimator is
  unbiased in practice, not only on paper.

Each rule holds or fails as printed by the instrument. A failure is a result and is reported as such; any change to
the configuration after the run is a new declaration, counted in K.

## 4. What will be reported

The full output, verbatim, in `saida.txt` and in the registry; the reading in the book, with the line the agent card
would carry (k, pass^1, pass^k, the independent reading and their ratio).
