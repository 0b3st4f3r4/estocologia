# Rereading of the risk opinions (form)

**Blank on purpose: nothing was reread, nothing was confirmed and nothing was signed here.** This file is the
instrument of the rereading of the 21 opinions that sil signed on 2026-10-15T03:53:28Z (record in
[`ASSINATURAS.md`](ASSINATURAS.md)). Whoever prepared this form did not read any opinion as someone who agrees: they only
copied number, file and hash.

The 21 SHA-256 below are those of `ASSINATURAS.md` and **were checked one by one with `sha256sum`** against the file
cited, on 2026-10-15: **21 of 21 match**, no file was missing. The check is reproducible through the excerpt of
"How to check", at the end.

## How to fill it in

- **`date`** — the date on which the rereading was done (ISO or as in `ASSINATURAS.md`).
- **`decision`** — exactly one of the three:
  - **`confirm`** — the opinion stays as it is, with the hash of this table;
  - **`withdraw`** — the signature is withdrawn; the opinion ceases to be signed and that is written down;
  - **`withdraw and reissue`** — the signature is withdrawn and the opinion comes back **with another one** (or with the same content
    reissued), and the new hash enters the "Rereading" section of `ASSINATURAS.md`, next to the old one, which is **not**
    deleted.
- **`note`** — what the rereading saw. It is not a place to rewrite the opinion.
- **Nothing is rewritten.** A row already filled in does not change: a new rereading enters as a new dated row, and
  `ASSINATURAS.md` is **not** edited by whoever prepares this form (it is outside the writing scope of this task).

### Rule of the four opinions that involve people

In the **four** opinions with "yes" in this table (2, 3, 17, 20), the note is mandatory and says **two things**:

1. **what the opinion did NOT look at** — the declared limit: what was left outside the opinion, what it did not evaluate, what
   has no source; and
2. **which affected groups exist** — who is hit by the decision of the opinion (groups, populations, categories),
   said by name.

Each of the two may be **"none"** when there is none: `did not look at: none` / `affected groups: none`. In the 17
opinions with "no", the note may stay empty or say "none"; writing a note about them is allowed, it is just not
required.

The material gathered from the repository itself for those notes (data subjects, quasi-identifiers, k-anonymity,
fairness groups and the scope of each decision) is in the section "Material for the note", at the end of this file. It is
**material**, it is not the note: whoever rereads writes their own.

The sibling file [`GRUPOS.md`](GRUPOS.md) — prepared on the same date, with the affected groups of the seven decisions that
involve people and the section "What the audit could not look at" — is the other half of that material: the note of the four
opinions must **cite** `GRUPOS.md` instead of repeating what it already raises.

## Table of the rereading

| # | opinion | involves people | file | SHA-256 | date | decision | note |
|---|---|---|---|---|---|---|---|
| 1 | Biology (cell signaling): biologia_sachs | no | `h2g/saida/biologia_sachs/resultados.json` | `c4586baf485c2edf1b77f2c2718df317e246e4729f05601f2a3be0fa11f5168b` | 2026-10-16 | confirm | none |
| 2 | Law (workers' compensation rule): direito_teto_beneficio | yes | `h2g/saida/direito_teto_beneficio/resultados.json` | `ba8a63e9924224a16c539ce7b00db3814a91d6fd2389721bd1f9679cf3d11d30` | 2026-10-16 | withdraw and reissue | new opinion in `pareceres/2-direito_teto_beneficio.md`; note confirmed and signed by sil on 2026-10-16 |
| 3 | Education: educacao_star_turmas | yes | `h2g/saida/educacao_star_turmas/resultados.json` | `f5b4307e4cbd103b96e7e0750c24e1758ec29995ae4eb7fa2bca28bc39477994` | 2026-10-16 | withdraw and reissue | new opinion in `pareceres/3-educacao_star_turmas.md`; note confirmed and signed by sil on 2026-10-16 |
| 4 | Engineering (aerodynamics): engenharia_tunel_vento | no | `h2g/saida/engenharia_tunel_vento/resultados.json` | `fbe8d0731fc3e7ab1a35c7a4fef134c5687e4026f7e8955130c46b8ee2e444fe` | 2026-10-16 | confirm | none |
| 5 | Clinical pharmacology (cardiology): farmacologia_qt | no | `h2g/saida/farmacologia_qt/resultados.json` | `d0042974a80fb3166d6e0f601c4b5facec8d6e66eb0547eb6e60fc9fce3c979e` | 2026-10-16 | confirm | none |
| 6 | Physics (optics and electronics): fisica_tunel_luz | no | `h2g/saida/fisica_tunel_luz/resultados.json` | `9bfa0f14b512611fc142a726505054925022767cac385bfb8dcfcd6f63c86ec0` | 2026-10-16 | confirm | none |
| 7 | Genetics (gene expression in yeast): genetica_y01 | no | `h2g/saida/genetica_y01/resultados.json` | `e14275f56b8e42f52d872c1782c16591d9fede8cb72f95488095e911758326b3` | 2026-10-16 | confirm | none |
| 8 | Genetics (gene expression in yeast): genetica_y02 | no | `h2g/saida/genetica_y02/resultados.json` | `5877dc7fc993b0a0f87f0c809e132dd61a8ef0e87eca352c22087bc9faa75cc8` | 2026-10-16 | confirm | none |
| 9 | Genetics (gene expression in yeast): genetica_y03 | no | `h2g/saida/genetica_y03/resultados.json` | `28f77502bf7ce135925874262b476b92eb60fb4a6a79912a479c0dca2611aac7` | 2026-10-16 | confirm | none |
| 10 | Genetics (gene expression in yeast): genetica_y04 | no | `h2g/saida/genetica_y04/resultados.json` | `e7543937f2c7c95c7781c5318aab1add0d5d1afef6195fa76489a9ed28d4f587` | 2026-10-16 | confirm | none |
| 11 | Genetics (gene expression in yeast): genetica_y05 | no | `h2g/saida/genetica_y05/resultados.json` | `620b251f38ebe1e4dac86fca07e02fc613230d5b617c0e6befb215bae098a24a` | 2026-10-16 | confirm | none |
| 12 | Genetics (gene expression in yeast): genetica_y06 | no | `h2g/saida/genetica_y06/resultados.json` | `c346f8a4d0b3a4be07e8d9e51e4ebae5723b223d23ee541e5b6bfdf4ffbc051e` | 2026-10-16 | confirm | none |
| 13 | Genetics (gene expression in yeast): genetica_y07 | no | `h2g/saida/genetica_y07/resultados.json` | `65666d206376fa73c826ccb7aef1b42dee9152556fe398f015d240f62297ed68` | 2026-10-16 | confirm | none |
| 14 | Genetics (gene expression in yeast): genetica_y08 | no | `h2g/saida/genetica_y08/resultados.json` | `86f19427ef3057afa25ed3da0f02c5cd700ab1529c7eced343ed062b843dacc5` | 2026-10-16 | confirm | none |
| 15 | Genetics (gene expression in yeast): genetica_y09 | no | `h2g/saida/genetica_y09/resultados.json` | `b4733257657015481c0f608e7c391e39c8b545d658a80d61e919d1112391aab2` | 2026-10-16 | confirm | none |
| 16 | Genetics (gene expression in yeast): genetica_y10 | no | `h2g/saida/genetica_y10/resultados.json` | `7fcfc653b550d0ae71e8491086683b4d2aaf286150be6961a891c7b26dc405e2` | 2026-10-16 | confirm | none |
| 17 | Health (insurance and use of services): saude_rand | yes | `h2g/saida/saude_rand/resultados.json` | `4e9c87b011fc1e01b8faea4a66ed1bbd5454504082435a5b62a9eb5bc44c7d6c` | 2026-10-16 | withdraw and reissue | new opinion in `pareceres/17-saude_rand.md`; note confirmed and signed by sil on 2026-10-16 |
| 18 | Transport (road safety): transporte_cinto | no | `h2g/saida/transporte_cinto/resultados.json` | `390b12022336021768a6febe9976f6faca46fc1540aab02b04d5289e682d5347` | 2026-10-16 | confirm | none |
| 19 | H1·A Dow Jones, additive against multiplicative (1885–2023) | no | `h1-real/saida/a/resultados.json` | `2b95a0b4832289c221f05c03c3d1136649c0d0e13c571e01bfe5f51255e4d73a` | 2026-10-16 | confirm | none |
| 20 | H1·B NSW (LaLonde): vocational training against CPS controls | yes | `h1-real/saida/b/resultados.json` | `5aaa954e56ede16cd7d38360bd923b3c75dea201f46464e7bb398c36f436aa58` | 2026-10-16 | withdraw and reissue | new opinion in `pareceres/20-nsw-lalonde.md`; note confirmed and signed by sil on 2026-10-16 |
| 21 | H1·C tail of the Danish fires, Layer 100 vs 50 | no | `h1-real/saida/c/resultados.json` | `0ecd25593905e8bf021262a1735538d27d574f58a75b64474d25c7b2c4116f38` | 2026-10-16 | confirm | none |

## Material for the note (gathered from the repository; it is not the note)

Fields copied from the `resultados.json` of the four opinions that involve people. They serve as a starting point for the two
parts of the note ("did not look at" and "affected groups") and **do not** replace the rereading. The
[`GRUPOS.md`](GRUPOS.md) brings the same list with the data-subject counts per group, and the part of "what the audit
could not look at" already gathered; this block remains as the field-by-field reading of each `resultados.json`.

- **2 · Law (workers' compensation rule), `direito_teto_beneficio`.** `governanca`: `tem_titulares: true`,
  quasi-identifiers `["homem", "casado", "kentucky"]`, `k_anonimato_bruto: 63`, real data
  (`wooldridge_injury.csv`), human responsible party with signature pending. Decision of the opinion: "cut the weekly
  benefit by 10% to shorten absences", with "adopt" and P = 1.00. Declared scope: it holds for compensation
  for work accidents in US states in the early 1980s with moderate changes of the cap; **it does not hold**
  for occupational diseases of long latency, large cuts, other countries and total duration without benefit.
  Caveat that already accompanies the opinion in `ASSINATURAS.md`: the recommendation of the 10% cut comes from a decision and a
  cost **invented** in F1, and is not policy advice.
- **3 · Education, `educacao_star_turmas`.** `governanca`: `tem_titulares: true`, quasi-identifiers
  `["menina", "etnia", "almoco_gratuito"]`, `k_anonimato_bruto: 9`, real data (`AER_STAR.csv`). Fairness
  block with group `etnia` and base rate by group: `afam` n = 4 524, `cauc` n = 8 632, `outra` n = 80. Decision:
  "reduce the class sizes of the early grades to ~15 students", with "keep" and P(adopting pays off) = 0.47275. Scope: it holds
  for early grades of US public schools with classes of 13 to 17 against 22 to 25; **it does not hold** for later
  grades, reduction at state scale with less experienced teachers and other countries.
- **17 · Health (insurance and use of services), `saude_rand`.** `governanca`: `tem_titulares: true`,
  quasi-identifiers `["mulher", "local"]`, `k_anonimato_bruto: 1103`, real data
  (`sampleSelection_RandHIE.csv`). Decision: "replace free care with 25% copayment", with "adopt" and
  P = 0.74125. Scope: it holds for non-elderly US families in the 1970s, plans with an out-of-pocket cap and covered
  medical spending; **it does not hold** for the elderly (Medicare), plans without an out-of-pocket cap, other countries and health
  outcomes (the opinion only looks at spending).
- **20 · H1·B NSW (LaLonde), `h1-real/b`.** `f8`: recommendation "fund the training", causal ATT effect with
  median 0.0963 (CI90 0.0088 to 0.1879), "fund if the causal ATT > C/L" with C per participant **illustrative**.
  Declared reference class: men eligible for NSW, 1976–77, 10 US cities, 84% black. The declared circuit breaker
  says to stop if the black fraction of a batch departs more than 0.25 from 0.84. There is no
  `governanca` block with data subjects nor quasi-identifiers in this `resultados.json` (the one of the real H1 does not have this field):
  it is one of the points the note has to state.

## How to check the 21 SHA-256

Runs from the root of the repository, without writing anything:

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

Expected output: `linhas: 21 | divergentes: 0`.

## Block for `governance/ASSINATURAS.md` — **applied on 2026-10-16**

The rereading was decided, and the section went to the end of `ASSINATURAS.md` with the fields filled in: the seventeen
confirmed, the four withdrawn and reissued with the new hash of each, and the four notes in two parts. The
skeleton below remains as a record of what was to be done.

````markdown
## Rereading

**Reread on <DATE> by sil.** The rereading used the form `governance/RELEITURA.md` (table of 21 rows, with the
same SHA-256 checked by `sha256sum`). The result of each opinion is below; `confirm` keeps the signature
of 2026-10-15T03:53:28Z, `withdraw` removes it, `withdraw and reissue` removes it and records the new hash.

| # | opinion | SHA-256 in the rereading | decision | note |
|---|---|---|---|---|
| 1 | ... | `<sha256>` | <confirm / withdraw / withdraw and reissue> | <note> |
| ... | (the 21 rows of `RELEITURA.md`) | | | |

**The four opinions that involve people (2, 3, 17 and 20).** In each one, the note says what the opinion **did not look at** and
which **affected groups** exist (or "none", when there is none):

- **2 · Law (workers' compensation rule)** — did not look at: <...>; affected groups: <... or none>.
- **3 · Education (STAR)** — did not look at: <...>; affected groups: <... or none>.
- **17 · Health (insurance and use of services)** — did not look at: <...>; affected groups: <... or none>.
- **20 · H1·B NSW (LaLonde)** — did not look at: <...>; affected groups: <... or none>.

**Nothing was rewritten.** The record of 2026-10-15T03:53:28Z continues in the file; where the decision was "withdraw and
reissue", the new opinion enters as a new row (or as a new `resultados.json` in a new folder) and the old hash stays
cited alongside, so that the clean reproduction remains comparable.
````

## Result of the rereading of 2026-10-16

**Who reread:** sil, the signatory of 2026-10-15T03:53:28Z, with the facilitator preparing the material. **Date:**
2026-10-16, in the working tree.

**The 21 files are not in the tree.** The pruning of 2026-10-16 took the fronts `h2g/` and `h1-real/` to commit
`1bfec905`. The check of the 21 SHA-256 was redone today in that commit:

~~~text
git archive 1bfec905 h2g/saida h1-real/saida | tar -x -C /tmp/releitura
linhas: 21 | conferidos: 21 | divergentes: 0 | ausentes: 0
~~~

In the tree, without restoring, the same check block gives `linhas: 21 | ausentes: 21 | divergentes: 0`. The
record keeps both numbers: the files left the tree, their content is safe in the commit and the
21 hashes match there.

**Decisions.** Seventeen opinions **confirm** (1, 4 to 16, 18, 19 and 21): the signature stands and the note is
"none", as the instrument allows for those that do not involve people. Four opinions **withdraw and reissue**
(2, 3, 17 and 20, the ones that involve people): the old signature leaves and the opinion comes back with another one, in
`governance/pareceres/`.

| # | new opinion | what the reissue adds |
| --- | --- | --- |
| 2 | `pareceres/2-direito_teto_beneficio.md` | governance block (data subjects, quasi-identifiers, k = 63), scope and the mandatory note |
| 3 | `pareceres/3-educacao_star_turmas.md` | fairness block by ethnicity with the base rates, scope, the absence of the SHELF panel and the note |
| 17 | `pareceres/17-saude_rand.md` | the limit "only looks at spending" in evidence, the elderly as excluded from the scope and the note |
| 20 | `pareceres/20-nsw-lalonde.md` | the governance block that was missing, the illustrative cost declared and the note |

**The note of the four is proposed, not signed.** The instrument says that whoever rereads writes their own note; the
signatory chose to withdraw and reissue, and the facilitator wrote the proposed note from the material
gathered (`GRUPOS.md` and the `resultados.json` of the commit). Each of the four documents carries the note marked as
**pending confirmation by the signatory**. While it is not confirmed, the new opinion does **not** enter
`ASSINATURAS.md`: its hash is only recorded with the note closed, and that is why `ASSINATURAS.md` remains
untouched.

*(Closed on the same day: the four notes were confirmed, with two wording micro-adjustments that the signatory
wrote — see "Close of the confirmation", below —, the four hashes entered `ASSINATURAS.md` and the pending
label left the four opinions.)*

**Close of the confirmation, on the same day.** The signatory reviewed the four new opinions and: confirmed the **3** and the **17**
in full; confirmed the **2** with a wording micro-adjustment in the first point of the note ("the effect of the change on
those below the cap", and not "the effect of the cut"); and confirmed the **20** with a micro-adjustment in the final clause
("the declared circuit breaker disarms if the fraction of black participants of a batch departs more than 0.25 from 0.84", in place of
"to depart"). The notes stayed as the signatory signed them, the pending label left the four files, and
the four SHA-256 entered the "Rereading" section of `ASSINATURAS.md`, next to the old ones.

**Count.** K of the rereading: the 21 rereadings are a single act (K = 1 of the rereading). The four reissues do not add
K, because they are not a new specification: it is the same opinion reissued with what was missing declared.
