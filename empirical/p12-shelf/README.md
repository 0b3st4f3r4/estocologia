# Point 12 (SHELF): the two artifacts that depend on people

## Result

**Five domains elicited by the first expert ($n = 1$, preliminary sensitivity analysis) and 21 rereading lines filled in and signed; the other two experts per domain and the one from outside the program are missing.** The panel protocol is frozen with hash (`DECLARACAO.sha256`), the first expert (E1, author of the program) concluded the five pre-tests of 20 resolved predictions (Brier skills of 0.6500, 0.9730, 0.7425, 0.8585, and 0.4240) and the five elicitations of quantiles (`h2/desenvolvimento_preston`, `h2/direito_prisoes`, `h2/educacao_star`, `h2g/direito_teto_beneficio`, and `h1-real/c`, with `h2g/educacao_star_turmas` reserved for the same base), and the rereading of the 21 opinions was concluded in `governance/RELEITURA.md` (17 confirmed, 4 withdrawn and reissued with a signed note). Since no expert from outside the program has been interviewed yet ($n = 1$), no elicited prior substitutes the prior of the package in the official round.

| artifact | content | state |
|---|---|---|
| `protocolo.md` | complete protocol: calibration pre-test of 20 resolved predictions (Brier in 10 bins of equal mass), the rule that whoever does not pass does not enter as a source, elicitation by median/quartiles/extremes with return, controls of anchor, order, and overconfidence, three experts per domain with one from outside the area of the program, and the list of the 6 domains with the starting quartiles of p05 | frozen in `DECLARACAO.sha256` |
| `formularios/` and `preteste/` | 6 forms (5 filled in block E1 and 1 reserved; blocks E2 and E3 in blank), 5 files `preteste/*.yaml` with measured skill, `prioris_elicitadas.yaml`, and `comparacao.md` (§6–§10 filled for $n = 1$) | E1 concluded ($n = 1$); E2, E3, and the external expert are missing |
| `../../governance/RELEITURA.md` | 21 lines of the opinions with file and SHA-256, date, decision (17 confirmed, 4 withdrawn and reissued), and signed note, plus the section "Releitura" applied in `ASSINATURAS.md` | 21 lines filled; 21 of 21 hashes checked |

The two numbers that this point uses as a starting point are a checked copy of `empirical/p05-prioris/prioris/`: the
literature quartiles of the 6 domains match the file of p05 at the 15th decimal place, and the 21 SHA-256 of `RELEITURA.md`
match the files cited by `sha256sum`.

## What was done

1. **Protocol.** `protocolo.md` fixes, before any elicitation: the 20 items resolved per expert and the
   four approval criteria of the pre-test (skill > 0, ECE ≤ 0.10, no bin with n ≥ 4 outside 0.25,
   weakly monotone curve); the composition of the panel (three per domain, one from outside the program **and** the area, plus the
   exclusions for declared double counting); the order of the elicitation (median → quartiles → extremes) with return and at
   most two rounds; the consensus by **point-by-point median** (declaring the trade against the mean of
   `h1/estocologia/agentes.py`); the controls of order (0.20 SD), anchor (0.50 ± 0.10 and 0.20 SD), and overconfidence
   (ratio ≥ 0.75); and the list of the 6 domains with the starting quartiles of p05 and what p05 already observed with
   them (for whoever compares, not for whoever elicits).
2. **Forms.** Six files, one per domain, with the elicited quantity, the unit, the decision at stake, and the current K of
   the domain; and 18 expert blocks in blank with pre-test, elicitation, controls, return, and quartiles
   confirmed. No number of the domain appears in the forms: the starting quartiles stay in
   `prioris_elicitadas.yaml` and only go to the table in the return (anchor control).
3. **Example and skeleton.** `prioris_elicitadas.yaml` carries the quartiles of the literature prior of p05 of the 6
   domains, marked as an **example** and not as elicitation, with the quartiles of the package alongside;
   `comparacao.md` carries the declared criterion of K = 2, the priors side by side (package, literature, and the blank column
   of the elicited one), the decision before and after, and the verdict to be written.
4. **Rereading.** `governance/RELEITURA.md`: 21 lines with number, opinion, file, and SHA-256; date, decision, and
   note in blank; the rule that in the four opinions that involve people (2, 3, 17, and 20) the note says what the
   opinion **did not** look at and which affected groups exist, or "none"; the field-by-field material of the
   `resultados.json` of those four, pointing to the sibling file `governance/GRUPOS.md`; the checking excerpt of
   the 21 hashes; and, at the end, in a block not applied, the section "Releitura" that `ASSINATURAS.md` is to
   gain later.

## Exact numbers

- **21 of 21** SHA-256 of `RELEITURA.md` match the `sha256sum` of the cited files (0 divergent),
  running the checking block itself that is in the file: `linhas: 21 | divergentes: 0`.
- **6 of 6** domains with starting quartiles identical to those of `empirical/p05-prioris/prioris/{h2,h2g,h1-real}/literatura.yaml`
  (maximum difference 0 at the 15th decimal place; `h1-real/c` has q05/q95 derived from the declared Beta(1,1), 0.05 and 0.95, and this
  is marked in the file).
- **Adjustment of the priors** (descriptive arithmetic with the functions of the repository, not elicitation), package → literature
  p05, implied SD: Preston 2.223903 → 2.603390; law in log(RC) 0.283331 → 0.104582; STAR 5.930409 → 3.666596;
  STAR classes 0.088956 → 0.050133; benefit ceiling 0.222390 → 0.074130; `h1-real/c` Beta(1; 1) in both.
  In five of the six domains the literature prior is **narrower** than that of the package.
- **Current K** of the elicited domains, from the field `K` of the `resultados.json`: `h2/desenvolvimento_preston` 1,
  `h2/direito_prisoes` 1, `h2/educacao_star` 1, `h2g/educacao_star_turmas` 3,
  `h2g/direito_teto_beneficio` 3; `h1-real/c` does not have the field.

## Limits

- **The elicitation of E1 is a sensitivity analysis ($n = 1$), not a full panel.** Blocks E2 and E3 of the forms stay in blank until the two additional experts per domain (including the one from outside the program and the area) go through the pre-test and the return.
- **The protocol is frozen in `DECLARACAO.sha256`.** The five pre-tests of 20 resolved items and the item declarations are sealed in `DECLARACAO.sha256`.
- **The numbers of "Adjustment of the priors" above are the descriptive baseline**, computed over the starting quartiles of p05; the comparison with the five priors elicited by E1 is in `formularios/comparacao.md` (§6–§10).
- **The label of the K is the program's.** The `K = 2` of `comparacao.md` is the number of this front (protocol = 1,
  comparison = 2) and is added to the K of each domain (1 → 2 and 3 → 4); if the count of the program uses another number, the
  label is corrected by a new declaration, without rewriting a file or a hash.
- **One expert from outside per domain is the declared minimum, not the ideal**: with only one, the control against the language
  of the program is left without a pair.
- **Three domains are "same base"** (`h2/educacao_star`, `h2g/educacao_star_turmas`,
  `h2g/direito_teto_beneficio`): their literature prior is not independent evidence, and the panel cannot
  have the same person on both sides of a pair (which is why `h2g/educacao_star_turmas` was reserved for E1).
- **The rereading of the 21 opinions is applied in `governance/RELEITURA.md` and `governance/ASSINATURAS.md`**, with 17 opinions confirmed and 4 withdrawn and reissued; the limit that whoever signs is an interested party still holds.

## How to reproduce / how to check

Checking of the 21 hashes (runs from the repository, writes nothing):

```bash
cd "$(git rev-parse --show-toplevel)"
python3 - <<'EOF'
import re, subprocess, pathlib
root = pathlib.Path(".")
txt = (root/"governance/RELEITURA.md").read_text()
rows = re.findall(r'^\|\s*(\d+)\s*\|.*?\|\s*`(.+?)`\s*\|\s*`([0-9a-f]{64})`\s*\|.*$', txt, re.M)
assert len(rows) == 21, len(rows)
ruins = []
for n, arq, h in rows:
    got = subprocess.run(["sha256sum", arq], capture_output=True, text=True).stdout.split()[0]
    if got != h:
        ruins.append((n, arq, h, got))
print("linhas:", len(rows), "| divergentes:", len(ruins))
for r in ruins:
    print("DIFERE", r)
EOF
```

Output: `linhas: 21 | divergentes: 0` (the same output as the block that is inside `RELEITURA.md`).

Checking of the starting quartiles against p05 and validation of the forms:

```bash
cd "$(git rev-parse --show-toplevel)/empirical/p12-shelf/formularios"
OMP_NUM_THREADS=1 python3 -c "
import yaml, glob
for f in sorted(glob.glob('*.yaml')):
    m = yaml.safe_load(open(f))
    print(f, 'OK', len(m.get('dominios', m.get('especialistas', {}))))
"
```

Resources: any script of this front runs with `OMP_NUM_THREADS=1` and at most 2 processes in parallel. Nothing
of this front was committed or pushed.
