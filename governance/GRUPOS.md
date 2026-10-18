# Affected groups heard

The book asks the program to record **what the audit could not look at and which affected groups were heard,
or to declare that none was** (section 14.2, objection "The auditor is an interested party"). This file is that record.
One row per decision that involves people: the four of the 21 opinions and the three arrest flows of H3.

| decision | affected groups | heard | by whom | when | where the record is |
|---|---|---|---|---|---|
| `h2g/saida/direito_teto_beneficio` — cut the weekly benefit by 10% to shorten absences | workers injured on the job covered by the cap, Kentucky (1980) and Michigan (1982); the cut reaches the `alta_renda` group (above the cap) | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h2g/direito_teto_beneficio`), `governance/pareceres/h2g-direito_teto_beneficio.md`, `governance/RELEITURA.md` and `empirical/p12-shelf/elicitacoes/direito_teto_beneficio.yaml` (historical origin in `h2f/` and `h2g/`, retired in commit `1bfec905`, `docs/declarations/REMOVED_PYTHON.md`) |
| `h2g/saida/educacao_star_turmas` — reduce the class sizes of the early grades to ~15 students | children from kindergarten to 3rd grade in Tennessee public schools in STAR (11 598 students, 80 schools), by declared ethnicity: `afam` 4 524, `cauc` 8 632, `outra` 80 | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h2g/educacao_star_turmas`), `governance/pareceres/h2g-educacao_star_turmas.md` and `governance/RELEITURA.md` (historical origin in `h2f/` and `h2g/`, retired in `1bfec905`) |
| `h2g/saida/saude_rand` — replace free care with 25% copayment | non-elderly US families in the RAND HIE (1970s), in the plans with an out-of-pocket cap | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h2g/saude_rand`), `governance/pareceres/h2g-saude_rand.md`, `governance/RELEITURA.md` and `empirical/p12-shelf/elicitacoes/saude_rand.yaml` (historical origin in `h2f/` and `h2g/`, retired in `1bfec905`) |
| `h1-real/saida/b` (NSW/LaLonde) — compare the NSW vocational training with CPS controls | 185 NSW training participants and 15 992 CPS controls, by declared race (`negro`, `hispanico`, `outro`) | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h1-real/b`), `governance/pareceres/h1-real-b.md`, `governance/RELEITURA.md` and `crates/stochology-book/src/cap17.rs` (historical origin in `h1-real/`, retired in `1bfec905`) |
| `h3` flow `prisoes` — flag the detainee for release review (Toronto 1997–2002) | 5 226 detainees, by perceived colour (`colour`, protected attribute of the flow contract) | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h3/prisoes`), `governance/pareceres/h3-prisoes.md`, `governance/RELEITURA.md` and `empirical/p14-h3/registro.tsv` (historical origin in `h3/`, retired in `1bfec905`) |
| `h3` flow `prisoes20` — flag the arrested person for record review at the police station (Toronto 2020–2021) | 65 276 arrests, by perceived colour; 37 thousand distinct people | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h3/prisoes20`), `governance/pareceres/h3d-prisoes20.md`, `governance/RELEITURA.md` and `empirical/p14-h3/registro.tsv` (historical origin in `h3/`, retired in `1bfec905`) |
| `h3` flow `prisoes_ri` — flag the driver for arrest review at the stop (Rhode Island 2005–2015, vault 2013–2015) | 86 113 stops, by perceived colour; no driver or officer identifier | **none** | — | — | `empirical/p13-escuta/registro.tsv` (`h3/prisoes_ri`), `governance/pareceres/h3d-prisoes_ri.md`, `governance/RELEITURA.md` and `empirical/p14-h3/registro.tsv` (historical origin in `h3/`, retired in `1bfec905`) |

**None of the seven affected groups was heard, in any of the seven decisions.** There is no record in the repository of
consultation, interview, panel of affected groups or consent.

## The seal (2026-10-16)

The program stopped treating listening as a pending opinion. The protocol of point 13 is frozen with a hash
([`empirical/p13-escuta/`](../empirical/p13-escuta/DECLARACAO.md)), the machine record has the seven decisions and the
instrument `labs --escuta` gives the verdict: with **0 of 7** groups heard and minimum coverage of **2 of 7**, the box
ends up **sealed**. The seven decisions remain as a sensitivity analysis ($n = 1$) and do **not** become public-policy
guidance nor an F8 gate decision until the minimum coverage is reached, or until **2026-12-15**; once the deadline passes without
coverage, the artifact is archived as a methodological exercise of sensitivity, with the seal in hash.

The "heard" column of this table remains **none** in the seven rows, and that is what the instrument checks: no
decision may call itself issued without someone heard — the synthetic case that lies is rejected in the gate report.

## What the audit could not look at

- **Who authorizes.** The 21 opinions were signed by sil on 2026-10-15T03:53:28Z
  (`governance/ASSINATURAS.md`) and reread one by one on 2026-10-16 (`governance/RELEITURA.md`), and sil confirmed the change
  of question of H3d on 2026-10-15T04:08Z (recorded in `docs/EMPIRICAL_REGISTRY.md` §4 and `empirical/p14-h3/README.md`).
  The signature of the responsible person is not listening to the affected group: the responsible person is on the side of whoever decides to use the artifact,
  not on the side of whoever pays for the error.
- **The human panel of outside experts was never completed.** The three domains of `h2g` were born declaring
  `shelf.painel: "literatura publicada; nenhum especialista humano consultado"` (`empirical/p05-prioris/README.md`;
  book, section 9.7); on 2026-10-16 the first internal expert (`E1`) closed five domains in
  `empirical/p12-shelf/elicitacoes/` ($n = 1$), but the other two people per domain and the expert from outside the
  program are missing, and no member of the affected groups was heard.
- **The decisions reached no one.** In the three arrest flows the artifact issues a flag for review; whoever
  would be flagged — the detainee, the arrested person, the driver stopped — never saw the artifact nor its error rate.
- **No consent and no committee.** The data are public and old (Toronto 1997–2002 and 2020–2021, Rhode Island
  2005–2015, STAR, RAND HIE, NSW, Kentucky and Michigan of the 1980s). F5 does k-anonymity and ε budget per
  person in two flows, which measures re-identification risk, and not consultation.
- **Replacement question.** In Rhode Island the question changed from "release after arrest" to "arrest in a
  traffic stop". The change was confirmed by sil, and not by the group that became the target of the decision.

## How this was established

There is no completed consultation protocol in the program, so the record came from a search for any trace of listening
in the whole repository. The only findings tied to people are the `shelf.painel` fields, the `tem_titulares` and the
quasi-identifiers — declarations about data, not about groups heard.

```
grep -rni "grupos afetados|grupo afetado|ouvidos|partes afetadas" --include=*.md .
grep -rni "painel de especialistas|entrevista|consulta pública|stakeholder|grupo de interesse" --include=*.md .
grep -rn "titular|tem_titulares" --include=*.md --include=*.tsv --include=*.yaml .
```

## Block G: the inversion of the metric

The Φ4 question — invert the chosen metric and show who would start paying — was run in the three arrest
flows. In the historical record of `h3/saida/<fluxo>/justica_invertida.json` (preserved in the manifest of commit
`1bfec905` and consolidated in this table and in `docs/EMPIRICAL_REGISTRY.md`), the **chosen** metric is the false
positive rate of the flag, P(flagged | does not suffer the outcome of interest), and the **inverted** one is the false
negative rate, P(not flagged | suffers the outcome). The least favored group comes out of the highest base rate of the outcome, and the payer is the
one with the highest rate under the metric; a tie is what has overlapping 95% Wilson intervals. The threshold sweep is
0.05 to 0.95 in steps of 0.05, plus the declared threshold of the flow.

| flow | threshold | FNR of the least favored group (at the flow threshold) | who pays under the chosen metric | under the inverted metric | change at the threshold? |
|---|---|---|---|---|---|
| `prisoes` (1997–2002, by year) | 0.70 | 0.64 / 0.68 / 0.66 / 0.67 / 0.44 (1998 to 2002) | `Black` in 1998–2001, tie in 2002 | tie in all years | no |
| `prisoes20` (by semester) | 0.70 | 0.062 / 0.056 / 0.048 / 0.071 | tie in the four semesters | tie in the four | no |
| `prisoes_ri` (vault 2013–2015) | 0.90 | 0.927 / 0.906 / 0.850 | `Black` in 2014, tie in 2013 and 2015 | tie in the three | no |

The finding that matters: **in Toronto 1997–2002, the least favored group is the one with the SMALLEST false
negative rate**. With the declared threshold of 0.70, the FNR of `Black` is 0.64 in 1998 against 0.70 of `White`; the chosen
metric (FPR) puts `Black` paying (0.196 against 0.105) and the inverted one (FNR) removes `Black` and leaves the tie or
`White`. Whoever is flagged more often without suffering the outcome is `Black`; whoever is missed more
often is `White`. Changing the metric changes who pays, and in 1998–2002 that happens at a threshold of the grid in
1998 (0.85), in 2000 (0.50) and in 2001 (0.80), and in none in 1999 and 2002 — always above the declared threshold of
0.70. In Toronto 2020–2021 the change appears in semester 2020-S2, in 5 of the 20 thresholds (0.35 to 0.55), and in Rhode
Island in none: there the Wilson intervals overlap in almost the whole sweep, and the inversion does not change the
payer. The background pattern is the same in the three flows: `Black` has the higher FPR and the lower FNR.

**K = 2 in this block.** Declaration 1 named "tie" when the base-rate intervals crossed, and that
left Toronto 2001 and 2002 without the requested number (the base rates are 21% and 19%). Declaration 2 names the group by the
point base rate and carries the tie alongside. Both were recorded, side by side, with the hashes below.

| flow | declaration 1 | declaration 2 | `governanca.py` in the declaration | in the run |
|---|---|---|---|---|
| `prisoes` | `dc2624f85a2cbbef09f326f9ab2ad84d45da1faa0075f26063d9e2f8a56ca7e4` | `ace45ccd919323a93ae0f5ad62a8c782644726cc6d38147e632d70eb5343f2e2` | `a44d14010b6c2235df7d5a7acee1ed8339b093da0ac56b528b9f14e37f28729b` | `81f231e78228fa6c932f609c97da96abc2d7bac7936c877c95d5a3c7da11ff70` |
| `prisoes20` | `449bfa97459eb1971eb196030c7e121c2bd5367c4f902562736310f1ef29f9e8` | `2be0152924dc2ee4d61cc2eb33e2050dd9a95cfc212b3e65964048f67d7122c6` | idem | idem |
| `prisoes_ri` | `a48c8899cee9d1a962102cd31aa7974af1c6306c88d318e816e900fa1a289251` | `7fb1cfbfd57edd956a9de8f4f4debbdd40ebb20b0f6f8732113cb35b23436b17` | idem | idem |

Verification in force in the tree: `cargo run --release --bin labs -- --escuta` checks `empirical/p13-escuta/registro.tsv`
against the minimum coverage, and `crates/stochology-methods/src/fairness.rs` implements the Chouldechova trilemma and the
inversion of metric in Rust. The original Python scaffolding (`h3/estocologia/governanca.py`) left the tree on 2026-10-16 and remains
recoverable through commit `1bfec905` (`docs/declarations/REMOVED_PYTHON.md`).
