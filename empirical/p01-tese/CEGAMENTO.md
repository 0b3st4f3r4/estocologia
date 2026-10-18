# Blinding of the judge (recorded before opening the opinions)

Written by the coordinator on 2026-10-15, after launching arms A and B and before reading any opinion.

- For each case, the X or Y label of each arm comes from `np.random.default_rng(20261017).permutation(2)` applied
  case by case, in the order caso_01 to caso_12 (0 = A becomes X; 1 = A becomes Y).
- The coordinator erases from the opinions, before the judge, the mentions of the arm and the terms proper to the architecture: L1, L2,
  L3, World, Model, Agent (as a layer), card, gate, invariant, Φ1–Φ4, F1–F8, M0–M15, Stochology,
  Fluxonomy, Chrononomy, Scalonomy, Morphonomy, Crisonomy, panel, orchestrator, hypergraph.
- The judge first receives only the final block (VEREDITO and MOTIVO PRINCIPAL) of X and of Y, then the clean text.
- The arms analysed copies of the cases in separate folders of the scratchpad (bracoA/ and bracoB/), with the instruction
  not to read the repository, the shared disk, the web nor other folders. The isolation is by instruction, not by
  file permission: it is a declared limitation.

## Addendum (before the judge saw any material)

While preparing the material, the full text of the arm A opinions remained recognizable after the cleaning (cards
in YAML and marks of erased terms), which would break the blinding. Therefore the judge received only the final blocks
(VEREDITO and MOTIVO PRINCIPAL), which came out with no erased term, and the answer key. The X/Y draw was the one recorded
above; the map stays in `scratchpad/mapa_XY.json` until the judge delivers the scores.
