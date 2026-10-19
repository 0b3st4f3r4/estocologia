# Point 24: resonance, not conflict

**Status on 2026-10-19: run once, after the seal. The claim is not supported.** The declaration (`DECLARACAO.md`), the
instrument and the three data files were sealed in `DECLARACAO.sha256` and committed (`6320388`, 18:39 UTC) before
`cargo run --release --bin labs -- --resonance` was first run. The output of that run is `saida.txt`, verbatim.

## Verdict, by the sealed rules

Testable in all three vaults. Δ is negative in two of the three, not in all three, and its interval lies entirely
below zero in one (Houston), where two were required: **not supported**. The criterion "adds over the rival" is met in
two of the three, which carries no weight when the claim itself fails, and the rival is reported below for what it
shows.

## The output

```text
== Point 24: resonance, not conflict (empirical/p24-ressonancia/DECLARACAO.md) ==
controls on synthetic data, seed 2026:

-- control, interaction in resonance --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 18148 beating 28223 conflict 13629
  Brier System 1 (additive) 0.13832 | System 2 (cells) 0.13353
  gain of System 2: conflict +0.00447 | resonance +0.00614 | delta -0.00167, 95% CI by day [-0.00282; -0.00054]
  rival delta (level agreement, no phase) -0.00090 | resonance beats the rival here: true

-- control, no interaction --
  training 60000 | vault flights with three eligible sources 60000 | quiescent 0 resonance 18192 beating 27661 conflict 14147
  Brier System 1 (additive) 0.13809 | System 2 (cells) 0.13424
  gain of System 2: conflict +0.00403 | resonance +0.00408 | delta -0.00005, 95% CI by day [-0.00105; +0.00089]
  rival delta (level agreement, no phase) -0.00019 | resonance beats the rival here: true

the three fresh vaults:

-- New York 2013, April to June --
  training 52354 | vault flights with three eligible sources 77251 | quiescent 0 resonance 18533 beating 44488 conflict 14230
  Brier System 1 (additive) 0.18685 | System 2 (cells) 0.18585
  gain of System 2: conflict -0.00002 | resonance +0.00040 | delta -0.00042, 95% CI by day [-0.00130; +0.00053]
  rival delta (level agreement, no phase) -0.00204 | resonance beats the rival here: true

-- New York 2023, April to June --
  training 74136 | vault flights with three eligible sources 101457 | quiescent 0 resonance 35519 beating 53217 conflict 12721
  Brier System 1 (additive) 0.15992 | System 2 (cells) 0.15865
  gain of System 2: conflict +0.00175 | resonance +0.00138 | delta +0.00038, 95% CI by day [-0.00047; +0.00125]
  rival delta (level agreement, no phase) +0.00005 | resonance beats the rival here: false

-- Houston 2011, October to December --
  training 37148 | vault flights with three eligible sources 49328 | quiescent 0 resonance 28283 beating 14964 conflict 6081
  Brier System 1 (additive) 0.13235 | System 2 (cells) 0.13179
  gain of System 2: conflict -0.00023 | resonance +0.00095 | delta -0.00118, 95% CI by day [-0.00221; -0.00019]
  rival delta (level agreement, no phase) -0.00149 | resonance beats the rival here: true

== verdict, by the rules sealed before the run ==
testable in all three: true | delta negative in all three: false | intervals entirely below zero: 1 of 3 (need 2)
the resonance claim: NOT SUPPORTED | adds over level agreement (in 2 of 3): yes (2 of 3)
```

## Reading

| Vault | Δ (conflict − resonance) | 95% interval | Δ of the rival |
| --- | --- | --- | --- |
| New York, April to June 2013 | −0.00042 | [−0.00130; +0.00053] | −0.00204 |
| New York, April to June 2023 | +0.00038 | [−0.00047; +0.00125] | +0.00005 |
| Houston, October to December 2011 | −0.00118 | [−0.00221; −0.00019] | −0.00149 |

The pattern that suggested the claim (Δ −0.00093 and −0.00284 in the two exploratory vaults of October to December)
does not hold in fresh months of the same city: in spring 2013 it is about half as large and within noise, and in
spring 2023 it has the other sign. It holds in Houston, a different city and year. Three readings, one clear, one
null, one reversed, is what a small effect that depends on the season and the network looks like, or what no effect
looks like after a pattern was picked from two vaults; this test cannot tell those apart, and by its rule it does not
support the claim.

The rival changed too. In the exploratory vaults the least-agreeing levels were where deliberation paid (+0.00146,
+0.00335); in the fresh vaults the sign is reversed in New York 2013 and Houston and null in New York 2023. Neither
the phase coherence of the engine nor the level agreement of its rival marks, reliably across seasons and cities,
where a model with interactions gains over an additive one.

## What this closes

The wave engine's superposition was given three readings on flights: its own claim (conflict calls for deliberation)
was untestable with the raw phase and refuted with the phase of the deviation; the reverse claim (resonance calls for
deliberation), drawn from that refutation, is not supported on fresh data. On these data the regime of the
superposition does not mark where deliberation pays, in either direction. The refinements of the code (`channel.rs`,
`cycle.rs`, the deviation encoding) remain the tools that made these tests possible.
