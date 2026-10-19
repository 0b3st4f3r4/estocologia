# Declaration of point 24: resonance, not conflict

**Written on 2026-10-19, before the instrument it declares first read the data it names.** Its SHA-256, those of
`crates/stochology-book/src/resonance.rs` and of the three data files below, enter `DECLARACAO.sha256` and are
committed before `cargo run --release --bin labs -- --resonance` is first run. By the owner's decision of the same
day to pursue the claim that declaration 2 of point 23 suggested. Written with an AI assistant.

## 0. Where the claim comes from, and why it needs new data

Declaration 2 of point 23 refuted the wave engine's claim about its superposition: in both New York vaults of
October to December (2013 and 2023), the gain of deliberation (System 2, the cells with interactions) over the
additive reading of the phasors (System 1) was **larger in resonance than in conflict**, Δ −0.00093 [−0.00165;
−0.00022] and −0.00284 [−0.00365; −0.00193]. A pattern found by reading data is a hypothesis about those data, not
evidence for it. This declaration turns it into a claim and tests it once on vaults that no reading of the programme
has used, with everything else frozen.

## 1. The claim

Where the daily risk deviations of a flight's carrier, origin and destination are **in phase** (the engine's
resonance), the sources compound beyond what an additive model captures, more than where they cancel (conflict).
Operationally: Δ = mean gain of System 2 in conflict − mean gain in resonance is **negative**.

## 2. What is frozen

The encoding, the regimes and thresholds, System 1, System 2, the rival, the minimum strata and the bootstrap are those
of declaration 2 of point 23, called unchanged: `wave2::conflict_test_deviation`, whose source is sealed in
`empirical/p23-onda/DECLARACAO.sha256`. Seed 2026; 2,000 draws of whole vault days; 95% percentile interval.

## 3. The three fresh vaults

Split by the instant of emission (two hours before the scheduled departure), with the 60 days before the first day of
the vault to learn:

1. **New York, 1 April to 30 June 2013** (`data/nyc_flights_2013_h1.csv`): learnt on 31 January to 31 March 2013.
2. **New York, 1 April to 30 June 2023** (`data/nyc_flights_2023_h1.csv`): learnt on 31 January to 31 March 2023.
3. **Houston, 1 October to 31 December 2011** (`data/houston_flights_2011.csv`): learnt on 2 August to 30 September
   2011. Another city, other carriers, another year; the scheduled times are recovered as the actual time minus its
   delay, since the source has none.

None of these months entered the tree before this declaration; the two New York files are the rows that were cut out
when the files of Chapter 20 were written, and the Houston file is new. The conversion wrote rows and computed no
statistic of the outcome.

## 4. The rules

- **Testable** only if all three vaults have at least 500 flights in conflict and at least 500 in resonance;
  otherwise the verdict is "not testable", which is not a pass.
- **Supported** if testable, Δ < 0 in all three vaults, and the 95% interval lies entirely below zero in at least two.
- **Adds over the rival** if testable and, in at least two of the three, the resonance claim separates more than the
  phaseless rival does in its own direction: −Δ > Δ_rival. (In the two exploratory vaults it did not: 0.00093 against
  0.00146, and 0.00284 against 0.00335. The rival is expected to be hard to beat.)

## 5. The synthetic controls, run before this declaration was sealed

The synthetic world of declaration 2 (a common evening peak; twelve carriers peaking every two hours; three origins at
8, 14 and 20 hours), with the interaction placed in the three carrier-origin pairs that peak at the same hour:
Δ −0.00167, interval [−0.00282; −0.00054], Δ_rival −0.00090. Without the interaction: Δ −0.00005, interval
[−0.00105; +0.00089]. The test finds an interaction in resonance and finds nothing where there is none.

## 6. What will be reported, whatever comes out

The three readings and the verdict, verbatim, in `README.md` and in the registry. The exploratory vaults of point 23
are not part of the verdict and are not re-run here. A failure is a result: the claim is K = 1 on these data, and any
change to it is a new declaration, sealed before its run, on data not yet used for it.
