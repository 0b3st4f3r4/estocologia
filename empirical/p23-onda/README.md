# Point 23: the wave engine against data

**Status on 2026-10-19: run once, after the seal.** The declaration (`DECLARACAO.md`) and the three sources of the
instrument were sealed in `DECLARACAO.sha256` and committed (`bf6353a`, 17:41 UTC) before
`cargo run --release --bin labs -- --wave` was first run on data. The output of that run is `saida.txt`, verbatim.

## Verdicts, by the sealed rules

- **T1, the arrow (layer 3, phasor STDP): not supported.** P1, P2 and P3 all fail.
- **T2, the conflict (layer 1, phase as the hour of peak risk): not testable under this encoding.** The engine put
  no flight in conflict in either vault.
- The engine adds over its rival in neither test.

## The output

```text
== The wave engine against data (empirical/p23-onda/DECLARACAO.md) ==
controls on synthetic data, seed 2026:

-- T1 control, planted arrow: 156 weeks --
  engine                             temperature -> demand 141 | demand -> temperature  15 | none   0 | rate 0.904
  engine, null pairing (+13 weeks)   temperature -> demand  57 | demand -> temperature  85 | none  14 | rate 0.365
  rival (lagged cross-correlation)   temperature -> demand 155 | demand -> temperature   1 | none   0 | rate 0.994
  rival, null pairing                temperature -> demand  74 | demand -> temperature  82 | none   0 | rate 0.474
  engine, margin 0.05 (secondary)    temperature -> demand 128 | demand -> temperature   7 | none  21 | rate 0.821
  binomial P(X >= 141 | n = 156) = 3.681e-27 | median learnt lag 0.17 h
  P1 true | P2 true | P3 true | works: true | adds over the rival: false

-- T1 control, independent: 156 weeks --
  engine                             temperature -> demand  80 | demand -> temperature  64 | none  12 | rate 0.513
  engine, null pairing (+13 weeks)   temperature -> demand  69 | demand -> temperature  76 | none  11 | rate 0.442
  rival (lagged cross-correlation)   temperature -> demand  70 | demand -> temperature  86 | none   0 | rate 0.449
  rival, null pairing                temperature -> demand  76 | demand -> temperature  80 | none   0 | rate 0.487
  engine, margin 0.05 (secondary)    temperature -> demand  51 | demand -> temperature  42 | none  63 | rate 0.327
  binomial P(X >= 80 | n = 144) = 1.056e-1 | median learnt lag 0.09 h
  P1 false | P2 false | P3 true | works: false | adds over the rival: false

-- T2 control, planted interaction --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 19804 beating 27088 conflict 13108
  Brier System 1 (additive) 0.15893 | System 2 (cells) 0.15760
  gain of System 2: conflict +0.00193 | resonance +0.00109 | delta +0.00083, 95% CI by day [+0.00003; +0.00166]
  rival delta (level agreement, no phase) +0.00099 | testable: true

-- T2 control, no interaction --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 21198 beating 25391 conflict 13411
  Brier System 1 (additive) 0.15502 | System 2 (cells) 0.15498
  gain of System 2: conflict -0.00013 | resonance +0.00039 | delta -0.00052, 95% CI by day [-0.00107; +0.00004]
  rival delta (level agreement, no phase) -0.00154 | testable: true

real data:

-- T1 Victoria 2012-2014, temperature -> demand: 155 weeks --
  engine                             temperature -> demand  53 | demand -> temperature  39 | none  63 | rate 0.342
  engine, null pairing (+13 weeks)   temperature -> demand  26 | demand -> temperature  36 | none  93 | rate 0.168
  rival (lagged cross-correlation)   temperature -> demand  92 | demand -> temperature  63 | none   0 | rate 0.594
  rival, null pairing                temperature -> demand  85 | demand -> temperature  70 | none   0 | rate 0.548
  engine, margin 0.05 (secondary)    temperature -> demand  31 | demand -> temperature  23 | none 101 | rate 0.200
  binomial P(X >= 53 | n = 92) = 8.749e-2 | median learnt lag 0.03 h
  P1 false | P2 false | P3 false | works: false | adds over the rival: false

-- T2 New York 2013, vault October to December --
  training 54835 | vault flights with three eligible sources 77184 | quiescent 0 resonance 77184 beating 0 conflict 0
  Brier System 1 (additive) 0.16645 | System 2 (cells) 0.16597
  gain of System 2: conflict NaN | resonance +0.00048 | delta NaN, 95% CI by day [NaN; NaN]
  rival delta (level agreement, no phase) NaN | testable: false

-- T2 New York 2023, vault October to December --
  training 68257 | vault flights with three eligible sources 98509 | quiescent 0 resonance 98146 beating 363 conflict 0
  Brier System 1 (additive) 0.11771 | System 2 (cells) 0.11737
  gain of System 2: conflict NaN | resonance +0.00038 | delta NaN, 95% CI by day [NaN; NaN]
  rival delta (level agreement, no phase) NaN | testable: false

== verdicts, by the rules sealed before the run ==
T1 the arrow: NOT SUPPORTED | adds over the cross-correlation: no
T2 the conflict: NOT TESTABLE under this encoding | adds over level agreement: no
```

## Reading

**T1.** Over 155 weeks the engine drew an arrow in 92: 53 temperature → demand, 39 the other way, and in 63 weeks
none, because no thermal-stress spike and demand surge fell within six hours of each other. The tally does not beat
a fair coin by the declared margin (P(X ≥ 53 | 92) = 0.087, against 0.01), the gap to the null pairing is 0.174
against 0.20, and the lagged cross-correlation on the same z-scores finds the right arrow in 92 weeks against the
engine's 53, a loss of 0.25 against the 0.10 allowed. The rival itself is weak here (0.594 against its own null of
0.548): under this encoding, the lead of temperature over demand inside a week is faint at half-hour resolution. The
engine's sparse code makes it worse, not better: it abstains in 41% of the weeks because it learns only from pairs of
events, while the correlation uses every half-hour. On the synthetic arrow the engine worked (141 of 156) and still
lost to the rival (155); on real data it does not work.

**T2.** Every eligible carrier, origin and destination has its daily delay risk peaking at the same time of day,
the evening, where the delays of the day accumulate; their first harmonics align, and every one of the 77,184 vault
flights of 2013, and 98,146 of the 98,509 of 2023 (the other 363 in beating), came out in resonance. The phase as the
hour of peak risk cannot express conflict among these sources: the accumulation of the day is a common cause that
puts every source in phase. The gain of System 2 over System 1 is small everywhere (+0.00048 and +0.00038 in Brier).
The rule says "not testable", and that is the verdict; it is not a pass and it is not a refutation of the claim, which
was never put at risk.

**What the two have in common.** Both failures come from a cycle shared by every source. In T1 the encoding removed
the daily cycle before the spikes were drawn, and what was left was too faint for the engine; in T2 it did not, and the
shared evening peak swallowed every phase. A phase means something only after the common cycle is taken out.

## A deviation, reported

The declaration speaks of 156 weeks with the last four days dropped. The sealed code keeps only the dates with 48
half-hours, and Victoria's series has six dates with 46 or 50 (the changes of daylight saving time, in April and
October of each year), so the run read 1,090 dates and 155 weeks, and the weeks after each dropped date are shifted
by one day. The code is the sealed specification; the text was imprecise. The run is not repeated with a fix,
because that would be a second run of a changed rule.

## What is proposed, not run

A second declaration, with K = 2 for each test, sealed before its run as this one was:

- T2 with the phase of each source's **deviation** from the global daily profile of the training flights, which
  takes the common evening peak out, exactly as the anomaly of T1 took the daily cycle out; the same rules,
  rival and minimum strata.
- T1 with the engine allowed to learn from coincidences within the same half-hour (today an event pair with Δt = 0 is
  skipped) and a horizon of a day, against the same rival and null.

Neither is run here: whether to spend a second look on these data is the owner's decision.
