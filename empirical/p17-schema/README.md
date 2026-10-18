# Point 17 — the migration chain of the contract

**State: closed. The card contract is now a file, and the chain has two links with the declared values.**

Until this front, the migration chain was code and was **empty**: the `PADRAO` said in Rust that the card was born at
1.0 and that no migration existed. Now it is a file — `crates/stochology-core/migracoes.tsv` — embedded in the binary
at compile time, with two continuous links and the reason written in each one:

| step | field | value | why |
| --- | --- | --- | --- |
| 1.0 → 1.1 | `resolucao` | `R0-agregada` | the field was born with Φ5; the migration does not invent a finer question than the declared one |
| 1.1 → 1.2 | `priori_elicitada` | `false` | the field was born with the SHELF front |

The card version went from 1.0 to **1.2**, and the chain reader rejects whatever is not a chain: a gap between
versions, an unknown operation, a step that moves backwards, a short line.

**The demonstration, declared as a demonstration.** There is no real card at 1.0 in the tree — cards are built
in code, at the current version, and the SHELF forms are another format. The instrument reconstructs the old form
(removes the two fields that the chain adds, marks the document as 1.0) and shows the migration: the two fields enter
with the declared values and the rest of the document stays intact.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --schema
~~~

**The limit.** The history of the card file has a single commit, so the chain is not reconstructed from Git: the
two links are the two contract changes that the repository already declared. A new field enters as a new declaration,
with the value it adds and the reason, and the version goes up.
