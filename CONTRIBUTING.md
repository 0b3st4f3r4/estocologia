# Contributing

All contributions must comply with the governance and verification contract defined in [`AGENTS.md`](AGENTS.md) and [`docs/PROTOCOL.md`](docs/PROTOCOL.md).

## 1. Epistemological Contract (`contract_version: "1.0"`)

- Every inter-module or inter-agent claim must be represented as a structured [`EpistemologicalCard`](crates/stochology-core/src/card.rs) (`cards/*.yaml`) and pass `EpistemologicalCard::audit()`.
- Zero external crate dependencies are permitted across the 5 workspace crates (`stochology-core`, `stochology-lenses`, `stochology-siga`, `stochology-governance`, `stochology-cli`).
- Deterministic simulations must use [`SplitMix64`](crates/stochology-core/src/rng.rs) with `DEFAULT_SEED = 2026`.

## 2. Mandatory Verification

Before submitting any change, run the full verification suite:

```bash
scripts/verify.sh
```

It enforces `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, all 6 laboratories (`labs`), the 7 positive epistemological gates (`gates`), and all 9 negative defect proofs (`gates --defect <...>`).

## 3. Licensing

Contributions to documentation are licensed under CC BY 4.0 ([`LICENSE-TEXT`](LICENSE-TEXT)) and contributions to code under MIT ([`LICENSE`](LICENSE)). Datasets in [`data/`](data/) retain their original licenses documented in [`data/SOURCES.md`](data/SOURCES.md).
