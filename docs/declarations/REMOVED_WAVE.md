# Declaration: the wave engine has been removed

On 2026-10-19, by the owner's decision, the wave engine (`crates/stochology-wave`, binary `wave`) left the working
tree, together with the three instruments that depended on it (`labs --wave`, `labs --wave2`, `labs --resonance`).

## Why

The engine entered on 2026-10-17 as an evolution of the research: a phasor superposition with a coherence index, a
magnetic Laplacian with a unitary propagator, and a spike bus that learns a causal arrow by STDP. Its six gates
proved that its mathematics was sound; none of them could fail because the engine misjudged the world. On 2026-10-19
it was refined so that it could be tested (`channel.rs`, `cycle.rs`) and put to three sealed tests on catalogued
data, each run once after its seal:

| Point | Claim | Verdict |
| --- | --- | --- |
| 23, declaration 1 | The STDP arrow finds temperature → demand in Victoria; deliberation pays where the phasors conflict | arrow not supported; conflict not testable (every flight in resonance) |
| 23, declaration 2 (K = 2) | The same, with a day of horizon and the phase of each source's deviation from the common profile | arrow not supported (77 against 78 in 155 weeks); conflict refuted in the opposite direction |
| 24 | The reverse: deliberation pays where the phasors are in phase, on three vaults never used for it | not supported (negative in two vaults, below zero in one) |

The arrow found no direction in the one dataset with a known one, and lost to the lagged cross-correlation; the
regime of the superposition did not mark, in either direction, where a model with interactions gains over an additive
one. The magnetic Laplacian never had a dataset with a truth for it. No chapter, no other crate and no gate used the
engine. Kept in the tree, it would carry 47 tests, a binary with gates named like the governance gates, and the
suggestion of a theory the evidence did not support.

## What left

| What | Where it was |
| --- | --- |
| The crate, 13 files, 38 tests | `crates/stochology-wave/` (phasor, magnetic, plasticity, spike, seal, complex, gate, channel, cycle, the `wave` binary, the integration tests) |
| The instruments of points 23 and 24, 9 tests | `crates/stochology-book/src/wave.rs`, `wave2.rs`, `resonance.rs`; the flags `--wave`, `--wave2`, `--resonance` |
| The steps of the verification | `wave --check` and its two negative proofs in `scripts/verify.sh`; the three instruments in its loop, in `scripts/book/check_labs.py` and in `scripts/book/generate_results.py` |

## What remained

| Remains | Why |
| --- | --- |
| `empirical/p23-onda/` and `empirical/p24-ressonancia/`: the declarations, the outputs (`saida.txt`, `saida_2.txt`), the reports | the record of what was tested, with every number; a negative result is a result |
| Their seals, with the hashes of the removed sources kept as lines declared out of the tree | anyone can restore a source and check it against the hash sealed before its run |
| `data/nyc_flights_2013_h1.csv`, `data/nyc_flights_2023_h1.csv`, `data/houston_flights_2011.csv` | the sealed data of point 24; read by no instrument now, kept so the record can be redone |
| The registry rows and the state records of 2026-10-19 | history, not versions |

## How to recover

Everything lives in the Git history at commit **`72169954d87b089cebd5c765c0c1da291db7ad2a`**, the last commit with
the engine in the tree:

```bash
git show 72169954:crates/stochology-book/src/wave2.rs        # one source
git checkout 72169954 -- crates/stochology-wave/              # the whole crate
git checkout 72169954 -- crates/stochology-book/src/wave.rs crates/stochology-book/src/wave2.rs \
                         crates/stochology-book/src/resonance.rs  # the three instruments
```

The restored sources hash to the values sealed in `empirical/p23-onda/DECLARACAO.sha256` and
`empirical/p24-ressonancia/DECLARACAO.sha256` before their runs.
