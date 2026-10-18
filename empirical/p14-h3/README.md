# Point 14 — the H3 chain, re-executed as far as the tree allows

**State: the reproducible part is instrumented; two lines stay declared, with the reason.**

The empirical registry carries four H3 lines: the six flows under a strict clock with a canary (**P**), the vault of
Rhode Island (**D**), the measured performative loop (**P**), and the H3 hypergraph (**P**). This front re-executes what
the tree allows and declares the rest, instead of leaving the mark **P** without a reason.

| file | what it is |
| --- | --- |
| `registro.tsv` | the four lines, with the class, the mark, the versioned record, and the declared criterion of the loop |
| `DECLARACAO.md` | the five declarations, with the new mark **V** and the limit line by line |
| `DECLARACAO.sha256` | the hashes, in `sha256sum` form |

**The result.** The performative loop is the only derivable line, because `empirical/p06-laco/resultados.json` is in the
tree: the instrument redoes the arithmetic of the criterion declared before running (sign, order of magnitude, strict
compatibility, strength of the instrument) and reaches the same verdict as the front — **the H3 loop overestimates the measured
value**, with the ratio of 4.1483 outside the band from 1/3 to 3, zero divergence between the arithmetic and the record. The other three lines stay
as **record** because the raw data left the tree with the `h3/` scaffolding (`docs/declarations/REMOVED_NON_SIGNIFICANT.md`);
what exists in Rust for them is the **mechanism**, in Chapter 20 (the canary with the honest clock and the laboratory
vault), and a measured mechanism is not a measured chain.

**How to reproduce.**

```bash
cargo run --release --bin labs -- --h3
```

The instrument reads `registro.tsv` and the `resultados.json`, and prints the class of each line, the re-derivation of the loop, and
the two negative proofs: a record in which the ratio falls inside the declared band changes the verdict, and a record
in which the field of the ratio does not match the arithmetic is rejected.
