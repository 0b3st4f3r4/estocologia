# Declaration of point 14 (the H3 chain): what is redone, what is re-derived, and what is left only to declare

**Frozen on 2026-10-16T21:57:54Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `registro.tsv` | `b4774e1b19745acc84284c90e8855b1dab0687b996b41b4be8d40fb98fef0a36` | the four lines of H3, with the class, the mark, and the declared criterion of the loop |
| `DECLARACAO.md` (this one) | — | this document; its own hash comes out in `DECLARACAO.sha256`, computed after the writing |
| `README.md` | `b4dbd57a51e6e3af36b58be3c0571dbd9510437dde336ed2ed5e5d228d036502` | the result in one page |
| `DECLARACAO.sha256` | — | the three hashes, in `sha256sum` form |

## Declaration 1 — the three classes of the chain

The instrument `labs --h3` goes through the four lines of H3 and says, for each one, what it is possible to do with it.
No line can be declared of a class that the data does not sustain:

| class | what it requires | what the instrument does |
| --- | --- | --- |
| **measured** | the raw data is in the tree | measures again |
| **derived** | the number is in a versioned file of the tree | redoes the arithmetic and the verdict, and checks it against what the record carries |
| **record** | the raw data is not in the tree | declares what was measured and why it is not redone |

What took the raw data out of the tree is declared in `docs/declarations/REMOVED_NON_SIGNIFICANT.md`: the scaffolding
`h3/` left on 2026-10-16 by the significance criterion, and with it the logs of the flows and the graph of H3. The raw
data of the performative loop (AnonymousBank, 1999) was never versioned — the front of point 6 keeps only the
`resultados.json` and the `posthoc.json`.

## Declaration 2 — the new mark **V** in the empirical registry

The legend of `docs/EMPIRICAL_REGISTRY.md` had three marks: **R** (the Rust instrument measures again), **D** (divergence
declared in the chapter, with both values), and **P** (record only). This declaration adds the fourth:

**V** — the verdict is re-derived in Rust from the versioned record; the measurement remains without an instrument.

The reason is the honesty of the legend itself. Calling the line of the loop **R** would be a lie: the instrument does not measure
again the effect of giving up on the new call, because the raw data is not here. Leaving it as **P** would hide
what came to exist: the number that the book cites, and the verdict that the book issues, are now recalculated from a
versioned file, with the checking between the arithmetic and the record. It is not a change of criterion — no criterion of
decision of the program changes —, and for that reason **it does not enter the K**.

## Declaration 3 — what the re-derivation returned

~~~bash
cargo run --release --bin labs -- --h3
~~~

| what | re-derived value | what the record carried |
| --- | --- | --- |
| main 2SLS estimate (W = 15 min) | 0.1446378859145094 | the same, digit by digit |
| 90% CI by bootstrap of days | [0.131981188365084; 0.15683852512801463] | the same |
| F of the first stage | 5523.46 | the same |
| ratio r declared / r measured | 4.148290720694298 | 4.148290720694298 |
| 0.6 inside the 90% CI | no | no |
| bias of the naive predictor against the exogenous demand | 0.29124674633711956 | the same |
| verdict of the criterion declared in the pre-registration | **the H3 loop overestimates the measured value** (the sign matches, the order does not: the ratio falls outside the band from 1/3 to 3) | the same |

**Divergences between the arithmetic and the record: none.** The instrument knows how to reject and the proof is in the tests: a
record in which the ratio falls inside the declared band becomes "it has the sign and the order of the measured one"; a record in which
the field of the ratio does not match the arithmetic is rejected; a weak instrument (F below the declared minimum) is undecided.

## Declaration 4 — the limit, declared line by line

| line of H3 | class | what stays declared |
| --- | --- | --- |
| the six flows under a strict clock, with a canary | **record**, mark **P** | the logs of the flows left the tree. What exists in Rust is the **mechanism**, in Chapter 20: with the honest clock the canary is available in 0 of 24 466 emissions and the gain is +0.0000; with the clock off, in 24 466 of 24 466, with a gain of +0.6521. A measured mechanism is not a measured chain |
| the vault of Rhode Island | **record**, mark **D** | the logbook of Rhode Island left the tree. The laboratory measures the mechanism (ECE 0.0022) and the chapter declares the divergence against the real vault (0.0048), which remains without an instrument |
| the performative loop (Goodhart) | **derived**, mark **V** | it is the only line that is re-derived, and that is what this front delivers |
| the hypergraph of H3 | **record**, mark **P** | the graph left with the scaffolding; the hypergraph of Rust is the validator, not the graph of H3 |

**What this does not resolve.** The second opinion on the numbers of H3 that remain without raw data — the vault of
Rhode Island and the counts of the hypergraph — remains tied to simulation, and it is item 12 of the inventory. This instrument
does not produce a second opinion: it re-derives the arithmetic of a recorded measurement, and declares the rest.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-horizons` | 40 tests passed, among them the seven of `cadeia`: the four lines with the marks `P, D, V, P`, the re-derivation with the ten digits, the verdict that changes when the ratio enters the band, the record that lies in the ratio (two divergences), the weak instrument that is undecided, the derived line without a versioned record, and the JSON reader against the real file (0) |
| `labs --h3` | the four lines with the class and the mark, the re-derivation of the loop with zero divergence, and the two negative proofs (0) |
| `python3 scripts/book/check_labs.py` | the block of Chapter 20 checks in `labs --h3` (0) |
