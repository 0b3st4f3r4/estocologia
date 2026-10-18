# Declaration of point 16: the bridge thesis at scale, with the fair test still to be done

**Frozen on 2026-10-16T22:37:13Z.**

| file | SHA-256 | what it is |
| --- | --- | --- |
| `registro.tsv` | `fb547bcd6de4fc7f047c10f0b095b551fb5c1b3ace09784a2f25f235ee0c23c8` | the roles of the fair test, the deadline, and the minimum of cases |
| `DECLARACAO.md` (this one) | — | the four declarations; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `c204219a96e412398df83da135422821cbefc930da6afed2b85ee46b711a3ff5` | the result in one page |
| `DECLARACAO.sha256` | — | the three hashes, in `sha256sum` form |

## Declaration 1 — the pilot, re-derived and not repeated

The P01 pilot gave a **tie at the ceiling**, and this instrument does not repeat the number of the README: it reads the scores of the blind
judge (`empirical/p01-tese/juiz/notas.yaml`) and redoes the arithmetic. What it returns:

| what | re-derived value |
| --- | --- |
| cases | 12, of which 8 with a defect and 4 clean |
| pointed out the defect | arm X: 8 of 8; arm Y: 8 of 8 |
| false alarm in the clean ones | arm X: 0 of 4; arm Y: 0 of 4 |
| discordant pairs | 0 |
| verdict by the declared rule | **tie at the ceiling: the thesis is not sustained** |

The judge coded each opinion with a score (1 = pointed out, in a case with a defect; 1 = false alarm, in a clean case).
Without a discordant pair, the McNemar test has nothing to test — the pilot **does not discriminate**, and that is what the
tie at the ceiling means. The rule that turns the tie into a verdict is in the `DECLARACAO.md` of the pilot, written
before any opinion; it is not rewritten here.

## Declaration 2 — what a fair test requires

The README of the pilot names what is missing: harder cases chosen by someone **from outside the program**,
**human** analysts, and a **human** judge. This front turns that into a verifiable requirement, in the
`registro.tsv`:

| role | requirement |
| --- | --- |
| selector of the cases | from outside the program, and chooses the cases **before** any result exists |
| analysts of arm A (architecture) | two humans, one from outside the program |
| analysts of arm B (ordinary analysis) | two humans, one from outside, blind to any text of Stochology |
| judge | one human from outside the program, blind to the arm, with the answer key |
| cases | at least 12, harder than the 2019 list, chosen by the outsider |

## Declaration 3 — the gate of the thesis

While the roles are not designated and the cases not at the minimum, the status of the thesis is **not sustained**, and
no line of the record can say otherwise. The instrument rejects whoever states the contrary: the proof is
in the tests — a record that says `sustentada` without the fair test is **REJECTED** with the divergence named, and a
status `em teste` without the designated roles as well. It is the difference between a thesis that is open and a thesis that
someone promoted by lapse of the deadline.

## Declaration 4 — the exit clause

- **Window:** 90 calendar days from this stamp, until **2027-01-14**.
- **Minimum coverage:** the four roles designated (with at least one analyst from outside in each arm and the judge from
  outside) and **12 cases** chosen by the outsider.
- **If the coverage is not reached by the deadline:** the thesis stays **not sustained definitively** as the verdict of the
  program, and the pilot stays as what it is — a test that tied at the ceiling against the 2019 list, without power
  to discriminate. No other front promotes the thesis.
- **If it is reached:** the test runs, the judge scores, and the verdict comes out of his scores by the rule of declaration 1.

A change of requirement, deadline, or minimum coverage is a **new declaration**, with its own hash, and counts in the K of the program.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-book` | 34 tests passed, among them the five of `ponte`: the re-derivation of the tie at the ceiling, the record that declares what is missing, the thesis sustained without a fair test **rejected**, the designated roles that let the test run, and the status `em teste` without the roles, rejected (0) |
| `labs --ponte` | the pilot re-derived (8 of 8 in both arms, 0 of 4 false alarms, 0 discordant pairs, tie at the ceiling), the five pending roles, and the two synthetic proofs (0) |
| `python3 scripts/book/generate_results.py --check` | `docs/RUST_RESULTS.md` updated, with the block of point 16 (0) |
