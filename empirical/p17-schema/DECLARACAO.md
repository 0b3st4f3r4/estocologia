# Declaration of point 17: the chain of migrations of the contract

**Frozen on 2026-10-16T23:07:35Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `crates/stochology-core/migracoes.tsv` | `520f9e095c88e3693ab7d12bd8e7eff951f3e3ebd7910e5db792332c64d6f60b` | the chain declared in a file; the contract is this file |
| `DECLARACAO.md` (this one) | — | the five declarations; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `b73c9cc48712c863eb38785a45293644aa1130361df0653e7058f34493f78265` | the result in one page |
| `DECLARACAO.sha256` | — | the three hashes, in `sha256sum` form |

## Declaration 1 — the chain leaves the code and becomes a file

Until here the contract of the card was code: the `PADRAO` was an **empty** chain declared in Rust, with the note that
the card was born at 1.0 and no migration existed. From here on the chain is a versioned **file**,
`crates/stochology-core/migracoes.tsv`, embedded in the binary at compile time (`include_str!`). Whoever reviews the
contract reads the file; whoever runs the instrument loads the same file.

The version of the card goes from **1.0 to 1.2**, and the chain has two continuous links: 1.0 → 1.1 → 1.2. The reading of the
file is a declared function (`cadeia_de_texto`), and it rejects whatever is not a chain: a gap between versions,
an unknown operation, a step that moves backwards, and a short line.

## Declaration 2 — the two values of addition, with the reason

| step | field | declared value | why |
| --- | --- | --- | --- |
| 1.0 → 1.1 | `resolucao` | `R0-agregada` | the field was born with Φ5. The migration **does not invent a question finer than the one declared** in the document: a document that does not speak of resolution is read at the coarsest grain |
| 1.1 → 1.2 | `priori_elicitada` | `false` | the field was born with the SHELF front. An earlier document does not have a panel-elicited prior, and `false` is what it is |

The two values are the only ones added by the chain, and each one has the reason written in the file itself.

## Declaration 3 — the reconstruction, declared as a reconstruction

**There is no real card at version 1.0 in the tree.** The cards of the repository are built in code, at the current
version, and the forms of SHELF are another format (elicitation protocol, not a card document). What the
instrument demonstrates is the **reconstruction** of the old form: it takes a complete card, removes from it the two fields that the
chain adds, marks the document as 1.0, and orders it to migrate. The demonstration is honest about that — it shows the
mechanism of the chain, and not a historical artifact that does not exist.

**Limit of what the chain covers.** The history of the card file has a single commit (the consolidation in Rust, with
the pruning of Python), so the chain cannot be reconstructed field by field from Git: the two links are the two
contract changes that the repository itself declares (Φ5 and the SHELF front). If another new field appears, it
enters as a new declaration, with the value of addition and the reason, and the version of the card goes up.

## Declaration 4 — the K of this front

The two values of addition are a choice of specification, made once and declared here: **K = 1**. Changing one of
them after seeing any migrated document is a new declaration, with its own hash, and counts in the K.

## Declaration 5 — what this does not change

No card of the tree is earlier than the current version, and no published number changes: the 12 cards of the binary
`portoes` are built in code, already at the current version, and their validation does not pass through the chain. What the
front delivers is the **contract in a file** for whatever comes: an old external document, a new field, or a change of
form declared with a value of addition.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-core` | 13 tests passed, among them the four new ones: the chain of the file is continuous and ends at the version of the card, the chain with a gap or an unknown operation is rejected, the document of 1.0 migrates with the declared defaults and the rest intact, and the current version does not pass through the chain (0) |
| `labs --schema` | version 1.2, the two links with the note of each one, and the demonstration: a document of 1.0 migrates, gains `resolucao = R0-agregada` and `priori_elicitada = false`, and validates (0) |
| `python3 scripts/book/generate_results.py --check` | `docs/RUST_RESULTS.md` updated, with the block of the contract (0) |
