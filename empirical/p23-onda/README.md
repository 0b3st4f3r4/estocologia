# Point 23: the wave engine against data

**Status on 2026-10-19: two declarations, each run once after its seal.** Declaration 1 (K = 1): the arrow not
supported, the conflict not testable. Declaration 2 (K = 2, the refinements below, sealed in `c93e0a3` before its run):
the arrow not supported, the conflict **refuted in the opposite direction**. The section on declaration 1 follows as
written after its run; the section on declaration 2 is at the end.

## Declaration 1

**Run once, after the seal.** The declaration (`DECLARACAO.md`) and the three sources of the
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

## What was proposed after declaration 1

A second declaration, with K = 2 for each test, sealed before its run as this one was: T2 with the phase of each
source's deviation from the common daily profile; T1 with a longer memory and horizon. (The idea of learning from
pairs inside the same half-hour was dropped when declaration 2 was written: a pair with no order carries no
direction.) The owner decided to proceed; that is declaration 2.

## Declaration 2 (K = 2)

`DECLARACAO_2.md`, sealed with `crates/stochology-book/src/wave2.rs` and committed (`c93e0a3`, 18:01 UTC) before
`cargo run --release --bin labs -- --wave2` was first run. Output: `saida_2.txt`, verbatim.

### Verdicts, by the sealed rules

- **T1 at K = 2 (spikes at 1.0, six hours of memory, a day of horizon): not supported.** P1 and P2 fail; P3 holds.
- **T2 at K = 2 (the phase of each level's deviation from the common daily profile): testable, and not supported.**
  Δ is negative in both vaults, with the 2013 interval entirely below zero.
- Neither adds over its rival.

### The output

```text
== The wave engine against data, declaration 2, K = 2 (empirical/p23-onda/DECLARACAO_2.md) ==
controls on synthetic data, seed 2026:

-- T1 (K = 2) control, planted arrow: 156 weeks --
  engine                             temperature -> demand 142 | demand -> temperature  14 | none   0 | rate 0.910
  engine, null pairing (+13 weeks)   temperature -> demand  61 | demand -> temperature  95 | none   0 | rate 0.391
  rival (lagged cross-correlation)   temperature -> demand 134 | demand -> temperature  22 | none   0 | rate 0.859
  rival, null pairing                temperature -> demand  80 | demand -> temperature  76 | none   0 | rate 0.513
  engine, margin 0.05 (secondary)    temperature -> demand 124 | demand -> temperature   6 | none  26 | rate 0.795
  binomial P(X >= 142 | n = 156) = 3.855e-28 | median learnt lag 0.38 h
  P1 true | P2 true | P3 true | works: true | adds over the rival: false

-- T1 (K = 2) control, independent: 156 weeks --
  engine                             temperature -> demand  70 | demand -> temperature  86 | none   0 | rate 0.449
  engine, null pairing (+13 weeks)   temperature -> demand  81 | demand -> temperature  75 | none   0 | rate 0.519
  rival (lagged cross-correlation)   temperature -> demand  80 | demand -> temperature  76 | none   0 | rate 0.513
  rival, null pairing                temperature -> demand  77 | demand -> temperature  79 | none   0 | rate 0.494
  engine, margin 0.05 (secondary)    temperature -> demand  47 | demand -> temperature  59 | none  50 | rate 0.301
  binomial P(X >= 70 | n = 156) = 9.134e-1 | median learnt lag 0.27 h
  P1 false | P2 false | P3 true | works: false | adds over the rival: false

-- T2 (K = 2) control, common evening peak, planted interaction, phase of the raw profile (declaration 1) --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 60000 beating 0 conflict 0
  Brier System 1 (additive) 0.13189 | System 2 (cells) 0.13095
  gain of System 2: conflict NaN | resonance +0.00094 | delta NaN, 95% CI by day [NaN; NaN]
  rival delta (level agreement, no phase) NaN | testable: false

-- T2 (K = 2) control, common evening peak, planted interaction, phase of the deviation --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 19250 beating 27426 conflict 13324
  Brier System 1 (additive) 0.13603 | System 2 (cells) 0.13136
  gain of System 2: conflict +0.00666 | resonance +0.00403 | delta +0.00264, 95% CI by day [+0.00156; +0.00365]
  rival delta (level agreement, no phase) +0.00273 | testable: true

-- T2 (K = 2) control, common evening peak, no interaction, phase of the raw profile (declaration 1) --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 60000 beating 0 conflict 0
  Brier System 1 (additive) 0.13407 | System 2 (cells) 0.13385
  gain of System 2: conflict NaN | resonance +0.00022 | delta NaN, 95% CI by day [NaN; NaN]
  rival delta (level agreement, no phase) NaN | testable: false

-- T2 (K = 2) control, common evening peak, no interaction, phase of the deviation --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 18192 beating 27661 conflict 14147
  Brier System 1 (additive) 0.13809 | System 2 (cells) 0.13424
  gain of System 2: conflict +0.00403 | resonance +0.00408 | delta -0.00005, 95% CI by day [-0.00105; +0.00089]
  rival delta (level agreement, no phase) -0.00019 | testable: true

real data:

-- T1 (K = 2) Victoria 2012-2014, temperature -> demand: 155 weeks --
  engine                             temperature -> demand  77 | demand -> temperature  78 | none   0 | rate 0.497
  engine, null pairing (+13 weeks)   temperature -> demand  59 | demand -> temperature  95 | none   1 | rate 0.381
  rival (lagged cross-correlation)   temperature -> demand  90 | demand -> temperature  65 | none   0 | rate 0.581
  rival, null pairing                temperature -> demand  80 | demand -> temperature  75 | none   0 | rate 0.516
  engine, margin 0.05 (secondary)    temperature -> demand  48 | demand -> temperature  33 | none  74 | rate 0.310
  binomial P(X >= 77 | n = 155) = 5.638e-1 | median learnt lag 0.04 h
  P1 false | P2 false | P3 true | works: false | adds over the rival: false

-- T2 (K = 2) New York 2013, vault October to December --
  training 54835 | vault flights with three eligible sources 77184 | quiescent 0 resonance 25196 beating 40352 conflict 11636
  Brier System 1 (additive) 0.16676 | System 2 (cells) 0.16604
  gain of System 2: conflict +0.00024 | resonance +0.00117 | delta -0.00093, 95% CI by day [-0.00165; -0.00022]
  rival delta (level agreement, no phase) +0.00146 | testable: true

-- T2 (K = 2) New York 2023, vault October to December --
  training 68257 | vault flights with three eligible sources 98509 | quiescent 0 resonance 35965 beating 52851 conflict 9693
  Brier System 1 (additive) 0.11771 | System 2 (cells) 0.11736
  gain of System 2: conflict -0.00029 | resonance +0.00256 | delta -0.00284, 95% CI by day [-0.00365; -0.00193]
  rival delta (level agreement, no phase) +0.00335 | testable: true

== verdicts of declaration 2 (K = 2), by the rules sealed before the run ==
T1 the arrow: NOT SUPPORTED | adds over the cross-correlation: no
T2 the conflict: NOT SUPPORTED | adds over level agreement: no
```

### Reading

**T1.** The wider horizon removed the abstentions (an arrow in every week, against 63 weeks without one at K = 1),
and the engine's arrow became a coin:
77 one way, 78 the other, in 155 weeks (P(X ≥ 77 | 155) = 0.56). The null pairing gives 59. The rival, at the same
horizon, finds the right arrow in 90 weeks against its null's 80, so the lead it sees is weak, and the engine sees
none. The synthetic control at the same settings found a planted arrow in 142 of 156 weeks, so the instrument is not
blind; in Victoria's anomalies, the STDP rule finds no direction. With two declared looks, the layer-3 claim stands
not supported on these data.

**T2.** Taking the common evening peak out did what the synthetic control said it would: the conflict regime now
fires, on 11,636 flights of 2013 and 9,693 of 2023, and the test became testable. And it came out the other way.
System 2, the model with interactions, gains **more** where the engine says resonance than where it says conflict:
Δ −0.00093 [−0.00165; −0.00022] in 2013 and −0.00284 [−0.00365; −0.00193] in 2023. Where the three sources' daily
deviations agree, the additive reading misses more than where they cancel. The rival without any phase points the
right way in both years (+0.00146 and +0.00335): disagreement among the sources' levels does mark where deliberation
pays, and the engine's phase coherence marks the opposite. The claim that the conflict regime is where System 2
should be opened is refuted on these data, at K = 2, by its own rules.

**What stands.** Of the engine's three layers, the arrow of the spike bus found no direction in the one dataset with
a known one, and the conflict of the superposition, once it could fire, pointed away from where deliberation pays.
The magnetic Laplacian is untested. Both refinements of the code stay (`channel.rs`, `cycle.rs`): they are what made
the engine testable, and a failure measured is the engine's first result on data. What the result suggests, and does
not establish, is that resonance, not conflict, is where sources compound beyond an additive model; testing that
would be a new claim, sealed before its run on data not yet used for it.
