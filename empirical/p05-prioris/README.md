# Point 5: the simulated priors (H2, H2g and real H1)

## Result

**N_b = 5 and N_c = 2, over the 33 domains (12 from H2 + 18 from H2g + 3 from real H1).** By the rule fixed in
[DECLARACAO.md](DECLARACAO.md), this is the branch "**depended on the simulated priors in the domains that change in (b)**":
five domains change status or F8 action with the literature prior, two with the weak prior, and the changes are not
cosmetic — one failure becomes pruning and one failure becomes approval. No change had a computational gate
(R-hat, ESS, SBC) as its only reason: `so_portao_computacional` is false in all seven.

**The gate of round (a) passes with the equivalence band declared in [`DECLARACAO_2.md`](DECLARACAO_2.md)
(0.05 sd of the round (a) posterior sd), and the verdict below is no longer conditional on that reading.** The maximum
deviation is in **0.0071 sd** (`h2g/genetica_y07`), **7.0 times inside the band**; of the 33 domains, 27 have both
posterior means to compare and 6 have no posterior emitted (pruned before F4). Field by field, against the repository
`saida/`: **33 of 33 identical** in status category, in list of failing gates and in F8 action.

The history of the check **was not erased**: without the band (the default of `comparar.py`, 1e-9 in bit equality) the gate
keeps failing **25 of the 33** — the 24 whose posterior mean on the two sides differs (relative from 4.1e-9 to 2.2e-3,
median 5.8e-7; at most 0.0071 sd) plus `h1-real/c`, whose posterior sd / prior sd ratio differs by 1.48e-9
(absolute — 1.7e-9 relative, which is the number this section carried before). The
difference is **numerical, not categorical**, and the cause is in the following section. By the house rule, changing a criterion after
seeing the result enters as a **new** declaration, with its own hash and counting in K: that is what `DECLARACAO_2.md` does.
**The K of this front goes from 1 to 2**, and no result of (b) or (c) is recomputed because of the band.

### The gate of round (a): the numerical deviation, and the band that closes it

| verification | result |
|---|---|
| status category, failing gates and F8 action, 33 domains | **33/33 identical** to the repository `saida/` |
| key posterior (`comparar.py`, tolerance **1e-9**, the historical one) | **25/33 failed**: 24 for the posterior mean (relative from 4.1e-9 to 2.2e-3, median 5.8e-7) and `h1-real/c` for the posterior sd / prior sd ratio (1.48e-9 absolute) |
| domains whose posterior mean on the two sides differs | **24 of 24** (the difference is numerical, not categorical) |
| domains without numerical comparison (pruned before F4, posterior not emitted) | **6**: `h2/transito_alcool` and `h2g/genetica_y01`, `y05`, `y06`, `y09`, `y10` — only the categorical check applies |
| deviation in units of the round (a) posterior sd | **≤ 0.0071 sd** (worst case: `h2g/genetica_y07`) |
| band declared in `DECLARACAO_2.md` | **0.05 sd** of the round (a) posterior sd |
| key posterior (`comparar.py --tolerancia-dp 0.05`) | **33/33 pass**; maximum deviation 0.0071 sd, **7.0 times inside** the band |
| posterior sd / prior sd ratio in real H1 (the band enters on the scale of that ratio) | `h1-real/a` 8.2e-14, `b` 1.0e-11, `c` **1.5e-9** — all inside 0.05 |
| deterministic quantities of F2/F3 (`beta_chave_mv`, `z_obs`, `nulo_q95`, ...) | differ in the 15th place (`~1e-15` relative) |
| copies against the originals (`diff -r`) | only the `$P05_PRIORI` hook (no-op with an empty table) and the path of the real H1 data |

The deviation is from the numerical environment of this machine against what generated the committed `saida/` (numpy 2.4.6, scipy-openblas
0.3.31), and not from the copies in `empirical/p05-prioris/`. The proof is the control: the **original** `h2g/` copied outside the
p05 tree and run with the seed of the sweep gave 25.210701694357784 in the key posterior of the pharmacology domain, our
number, and not the recorded 25.21079859117877. Running it twice here, the result comes out bit-identical (same seed, same
machine). To size it: the maximum environmental deviation is 0.0071 sd, and the smallest shift that changed a decision is
0.18 sd — twenty-five times larger.

### The equivalence band: `DECLARACAO_2.md`

`DECLARACAO.md` wrote the failure condition **without tolerance** ("the copy does not reproduce the original round (status or
posterior different from the original `saida/` in (a))"), and `comparar.py` implemented it as bit equality (1e-9 on the
posterior mean). By the letter it fires. None of that was touched: `DECLARACAO.md` is `167fbd6a…` as it was recorded. What
exists now is a **second declaration**, written before any recomputation with the band:

- [`DECLARACAO_2.md`](DECLARACAO_2.md), sha256 `6e2b59023b075d86fec4fce4a4b2be4ea395272ad65edc0fd930972a6835dd63`,
  UTC stamp `2026-10-15T19:16:37Z` (16:16 local), in `DECLARACAO_2.sha256`; `sha256sum -c DECLARACAO_2.sha256` gives OK.
- **The band: 0.05 sd of the round (a) posterior sd**, fixed between the two facts already known — **7.0 times above** the
  worst environmental deviation observed (0.05 / 0.0071 = 7.04) and **3.6 times below** the smallest shift that has already changed a
  decision (0.18 / 0.05 = 3.6).
- **Reading criterion, declared:** maximum deviation ≤ 0.05 sd ⇒ round (a) reproduces and the verdict of point 5 stops
  being conditional; above that the conditional label would stay. The maximum deviation observed is 0.0071 sd, so the criterion is
  met with a slack of 7 times. The categorical check (status, failing gates, F8 action) remains **without a band**,
  exact equality — no band forgives a status change.
- **The band does not reopen (b) and (c).** Both rounds had already been seen when the declaration was written, and it says
  so: no criterion of (b) or (c), no seed, no prior table and no decision is changed, and no
  number of (b) or (c) is recomputed. N_b = 5 and N_c = 2 remain the same, with the same seven changes named below.
- **K = 2.** The K of this front counts reading rules declared for the same result: the one in `DECLARACAO.md` (without
  tolerance, which fails 25 of the 33) and this one (with a band of 0.05 sd, which passes 33 of the 33). Both hold; the new one is an
  **additional** reading, published with its own hash.
- The maximum deviation comes out the same in both readings — the number does not depend on the band: **0.00710575674131663 sd**.

### The five changes of (b), the literature prior

| domain | status (a) → (b) | F8 action (a) → (b) | shift (b) in sd of (a) | source of (b) |
|---|---|---|---|---|
| `h2/desenvolvimento_preston` | approved → approved | adopt → **keep** | 0.48 | Pritchett & Summers (1996), income elasticity of life expectancy — **independent** |
| `h2/direito_prisoes` | failed at F4 (modelo_vence_climatologia) → **pruned at F1** (EVPI < cost) | — → — | — (pruned before F4) | Kochel, Wilson & Mastrofski (2011), effect of race on imprisonment — **independent** |
| `h2/educacao_star` | approved → **pruned at F2** (the unit no longer informs more than the prior) | adopt → — | — | Krueger (1999), 4 percentile points in year 1 — **same base** (`AER_STAR.csv`) |
| `h2g/educacao_star_turmas` | approved → **pruned at F1** (EVPI < cost) | keep → — | — | Krueger (1999) — **same base** (`AER_STAR.csv`) |
| `h1-real/c` | failed (modelo_vence_climatologia) → **approved** | 0.4 → **0.0** | — (`comparar.py` does not compute shift in H1) | **no verified source**: the weak Beta(1.1) entered |

Reading of each one:

- **`h2/desenvolvimento_preston`** is the only change that appears in both rounds: P(adopting pays off) falls from 0.565
  (a) to 0.366 in (b) and 0.495 in (c), crossing the F8 threshold. The source of (b) is independent of the domain base, but
  the weak one changes the action just the same, so what this change shows is that **the F8 action of that domain is fragile to any
  informative prior**, not to the literature one in particular.
- **`h2/direito_prisoes`** and **`h2g/educacao_star_turmas`** leave at F1: the literature prior is far narrower than
  the simulated panel (sd 0.105 against 0.283 in the first; 0.050 against 0.089 in the second) and the EVPI falls below the cost of the
  research. In both the status ceases to exist before F4.
- **`h2/educacao_star`** stops at F2: with the literature prior built from the **very base** of the domain, the data of the
  independent unit no longer informs more than the prior. It is the double-counting case that the declaration foresaw marking, and it is
  where it bites.
- **`h1-real/c`** is the project that M15 failed in real H1. Once the simulated SHELF panel is swapped for Beta(1.1), it passes,
  and the decision goes from 0.4 to 0.0. Since there is no verified source for the three projects of real H1, the entry of (b) is the
  same as that of (c): of the five changes of (b), this one is not a "literature" change.

Of the five, **three come from an independent source** (Preston, direito_prisoes, and the one of h1-real/c that has no source) and **two from
a source of the same base** (the two education ones, over `AER_STAR`). In none of them does the state of the domain improve with the
more informative prior: either it prunes, or it reverses towards not adopting (Preston), or — in real H1 — it approves what the simulated panel
failed.

### The two changes of (c), the weak prior

| domain | status (a) → (c) | F8 action (a) → (c) | shift (c) in sd of (a) |
|---|---|---|---|
| `h2/desenvolvimento_preston` | approved → approved | adopt → **keep** | 0.18 |
| `h1-real/c` | failed → **approved** | 0.4 → **0.0** | — |

With an sd ten times larger (and Beta(1.1) in real H1), the weak prior barely moves anything: **31 of the 33 domains stay
the same** in status and action. Where it moves is where the data are weak against the decision, and the two changes are the same two
that the literature had already moved. That is the branch the declaration called "did not depend on the simulated priors in the
content, but did depend on there being some informative prior".

### A larger shift is not a change

Two domains with a large shift **did not** change anything, and it is useful to see why: `h2g/direito_teto_beneficio` moves
**1.36 sd** in (b) (source of the same base, Meyer, Viscusi & Durbin 1995) and remains approved with the action `adopt`;
`h2/credito_hmda` moves **0.80 sd** in (b) and in (c) (no verified source, so the same table in both) and remains
approved with `adopt`. The shift measures the sensitivity of the posterior, not of the decision: F8 decides by P(adopting
pays off) against the threshold, and in these two the posterior moves without crossing the threshold.

The opposite case, `h2g/fisica_tunel_luz`, barely feels the swap (0.004 sd) because the literature prior is the weak one and the
posterior was already dominated by the data (posterior sd / prior sd ratio of 0.0084 in round (a)).

### The G3 of H2g

The ICP does not use the prior, so G3 could only change if a domain were pruned before F4. One was: education.

| round | domains with Ŝ ≠ ∅ | areas | G3 (Ŝ ≠ ∅ in at least 3 areas) |
|---|---|---|---|
| (a) original | 4: `educacao_star_turmas` (Education), `fisica_tunel_luz` (Physics), `genetica_y08` (Genetics), `farmacologia_qt` (Pharmacology) | 4 | passes |
| (b) literature | 3: Physics, Genetics, Pharmacology — `educacao_star_turmas` pruned at F1 | 3 | **passes, with no margin** |
| (c) weak | 4, the same as in (a) | 4 | passes |

**G3 passes in the three rounds, but in (b) it falls from 4 areas to exactly 3**, which is the minimum of the PLAN, because the
literature prior of education (same base as the domain) brings down the small class at F1. The Ŝ of each domain that
survives is the same in the three rounds: `{c_dof, c_qui, c_ran}` in pharmacology, `{blue, green, red}` in the light tunnel,
`{plate, x1, x5, x9}` in y08 and `{anos_pequena_antes, turma_pequena}` in education.

## What was done

1. **The declaration came first.** [DECLARACAO.md](DECLARACAO.md) was written before any execution of the funnel with
   the alternative priors, and `DECLARACAO.sha256` (9 entries: the declaration, `prioris.py` and the six tables
   `prioris/*/*.yaml`) checks out — `sha256sum -c` gives OK on all of them; the declaration is `167fbd6a…`. No criterion, seed,
   grouping or rule R1–R3 was changed afterwards. `h2g/DECLARACAO_H2g.sha256` verifies the `a5d879b3…` cited in the H2g
   README (the file has one line with a date stamp that `sha256sum -c` does not understand; the hash matches).
2. **The data were verified with `sha256sum -c` against the `SHA256SUMS` of each folder, before running.**

   | folder / file | result |
   |---|---|
   | `data/h2/` (12 files) | **OK** |
   | `data/h2b/` (H2g reads from here) | **OK** |
   | `data/h2g/` (includes `ecgcipa/adeg.csv`, `3ab7b3cd…`) | **OK** |
   | `data/h2e/causal_chamber_wt_walks_v1_regime_jumps_single.csv` | **OK** (`ae00d09a…`) |
   | `data/h2e/causal_chamber_lt_interventions_standard_v1.csv` | **no**: the file present has `a80bc1a6…`, and `SHA256SUMS` records `99b75516…` — see below |
   | `data/h2f/Kemmeren.RData` | **OK** (`08e95f39…`) |
   | `data/h2f/kemmeren2014_obs_int.parquet` | **no**: `f152937f…` against `0921bda4…` recorded; the deviation and the content check are in `data/h2f/CONVERSAO.md` (1741 × 6175, the `obs` and `int` blocks bit-identical to the `RData`) |
   | `data/h3b/nycflights23_flights.csv` | absent — outside the scope (H3 does not use a SHELF prior) |

   The deviation of `lt`: the file is the concatenation of the 59 CSVs of the zip of `lt_interventions_standard_v1`, in the order of the
   zip, with the `experiment` column appended at the end with the file name without extension — shape 68,000 × 47,
   `uniform_reference` with 10,000 rows and the other 58 with 1,000. It was built twice independently
   (here and by the parent process) and both gave `a80bc1a6…`. The two source zips check out against the MD5 declared in the
   `SOURCES.md`; what differs is the byte of the derived file, the same phenomenon as the H2f parquet (Parquet carries the version of the
   writer in the metadata). No domain of this point used the 8 derived columns beyond what `preparo.py`
   already read; no number was adjusted because of this.
3. **The three rounds, in the declared order, from the copies.** Round (a) ran alone and first; then (b) and
   (c). `rodar.sh` (with `P05_H="h2 h1-real"`) for H2 and real H1, `rodar_h2g.sh` for H2g. The combination used
   in H2g was the one of `rodar_h2g.sh`: domain by domain, with the sweep seed kept (`SEMENTE + 100 × posição`),
   (b) only in the 4 domains with a verified source and, in the other 14, the result of (c) copied into (b) — the copy was
   checked here: in the 22 domains where the literature entry is the weak one (14 of H2g, 5 of H2 and the 3 of real H1) the
   quartiles of the two tables are equal and, in the results, the prior and the posterior of (b) and (c) are identical.
   H2g **was not** run through the full sweep of 18 domains inside `rodar.sh` (it would be three sequential sweeps
   of ~13 min); the route of `rodar_h2g.sh` is the one that keeps the same seed and parallelizes. Both scripts
   left the executable tree on 2026-10-17, because they call the retired `p05_um.py`; the reference copy is in
   `history/rodar-p05-prioris-2026-10-17.sh` and `history/rodar-p05-prioris-h2g-2026-10-17.sh`.
4. **Outputs outside the repository**, in `~/p05-saida/` (the repository only received this README, the
   `resultados.json` and the `tabela.md` generated by `comparar.py`).
5. **The check of (a) against the originals** was done with `comparar.py` (tolerance of 1e-9) and field by field
   in the 33 domains (`comparar_33.py`, which separates what is category from what is moment); the diagnosis of the deviation has the
   control described above (`/tmp/ctrl_h2g.py`, the original `h2g/` copied to `/tmp/ctrl-h2g`).
6. **The equivalence band of round (a)**, after that: [`DECLARACAO_2.md`](DECLARACAO_2.md) was written before
   any recomputation with it (sha256 `6e2b5902…`, UTC stamp `2026-10-15T19:16:37Z` in `DECLARACAO_2.sha256`,
   `sha256sum -c` OK), `comparar.py` gained the option `--tolerancia-dp` with default 1e-9 (historical reading intact: the
   default fails the same 25 of 33) and the comparison was rerun with `--tolerancia-dp 0.05`, writing
   `resultados_2.json` and `tabela_2.md` — without touching `resultados.json` or `tabela.md`.

## Tables

The full rounds are in [`tabela.md`](tabela.md) (33 domains: status in the three rounds, F8 action, posterior
mean, posterior sd / prior sd ratio and shift in sd of round (a)), generated by `comparar.py`. The
[`tabela_2.md`](tabela_2.md), from the reading with the band, comes out **identical** to it (the band only moves the gate of round (a),
not the content of the rounds). The aggregates:

| round | domains that change | of those that change, independent source | of those that change, same base | of those that change, no source |
|---|---|---|---|---|
| (b) literature | **5** | 2 (`h2/desenvolvimento_preston`, `h2/direito_prisoes`) | 2 (`h2/educacao_star`, `h2g/educacao_star_turmas`) | 1 (`h1-real/c`) |
| (c) weak | **2** | 0 | 0 | 2 (`h2/desenvolvimento_preston`, `h1-real/c`) |

| domains with a verified source | how many | which |
|---|---|---|
| source read from outside the domain base | 6 | `h2/clima_tempestades`, `h2/desenvolvimento_preston`, `h2/agronomia_nitrogenio`, `h2/trabalho_curriculos`, `h2/direito_prisoes`, `h2g/farmacologia_qt` |
| source from the same base as the domain (double counting, marked) | 5 | `h2/transito_alcool`, `h2/educacao_star`, `h2g/direito_teto_beneficio`, `h2g/educacao_star_turmas`, `h2g/transporte_cinto` |
| no verified source (the literature entry is the weak one) | 22 | the 14 of H2g without a source, the 5 of H2 (`credito_hmda`, `epidemiologia_contracepcao`, `energia_vitoria`, `logistica_voos`, `saude_consultas`) and the 3 of real H1 |

## Limits

- **There is no human specialist.** No prior of this round is a SHELF elicitation: the literature ones are readings
  of published sources converted to the quantity of the domain by three fixed rules (R1–R3), and the weak ones are a
  declaration of ignorance. This tests **whether the conclusions of the funnel depended on the simulated priors**; it does not replace a
  elicitation panel and does not say what a real panel would have produced.
- **The gate of round (a) only passes through the second reading.** Status category, list of gates and F8 action match
  in 33/33; the posterior mean differs in 24 of the 24 domains that have one, from 4.1e-9 to 2.2e-3 of relative difference (≤ 0.0071
  sd), and `h1-real/c` differs by 1.48e-9 in the posterior sd / prior sd ratio. The control shows that the reason is the numerical
  environment, not the copies. By the failure condition written **without tolerance** — `DECLARACAO.md` — this is still a
  failure: `comparar.py` with the default of 1e-9 fails the same 25 of 33 today. The verdict of this README holds through the
  band of **0.05 sd** declared after seeing the result, in `DECLARACAO_2.md`, with its own hash and **K = 2**. The band
  is 7.0 times the worst deviation observed and 3.6 times smaller than the smallest shift that has already changed a decision, but it is a
  **new** rule, not the one written before, and that is how it must be read.
- **Six domains do not enter the numerical check of round (a)**, because the posterior is not emitted in them (pruned before
  F4): `h2/transito_alcool` and five Y of H2g. In them the gate of round (a) is categorical only, and the band applies
  to nothing.
- **`comparar.py` does not measure the shift in real H1**: the shape of the result of that horizon does not carry the posterior
  mean of the key (`post_media` is null), so the three lines of real H1 stay without the number that the declaration asked
  for all domains. What exists stayed: the posterior sd / prior sd ratio (0.87 → 0.072 in project c), the action and the
  status.
- **14 of the 18 domains of H2g have no verified source**, so round (b) of H2g is, in them, the weak one itself (copied,
  as the header of `rodar_h2g.sh` requires). The whole effect of (b) in H2g comes from a single domain: `educacao_star_turmas`.
- **None of the seven changes is from a computational gate**, but that is not a robustness test of the gates: R-hat, ESS
  and SBC are verified inside each execution and no domain was pruned by them in any of the rounds.
- **The scope of H2b–H2f was not rerun**: the declaration covers the 33 domains above and justifies that the final versions
  live in H2g, with quartiles, `taxa_base` and decision identical. **H3 stays out**: `estat.py` has the SHELF prior function,
  but no flow calls it.
- **Did not run for lack of data:** nothing in this scope. `data/h3b/nycflights23_flights.csv` is absent and is not
  read by any domain here; `h2f` and `h2e` derived are present with a hash diverging from the recorded one, which
  is reported above and in `data/h2*/CONVERSAO.md`.

## How to reproduce

Everything with `OMP_NUM_THREADS=1` and, outside the repository, `OUT=~/p05-saida`:

```
export P05_DADOS="$REPO/dados-reais"   # REPO: the root of the repository
export OUT=~/p05-saida
export OMP_NUM_THREADS=1
cd empirical/p05-prioris

# round (a), alone and first (the copy with the empty table is the original code)
(cd h2      && P05_PRIORI="" ESTOCOLOGIA_SAIDA=$OUT/h2/original      ESTOCOLOGIA_DADOS=$P05_DADOS/h2  python3 p05_rodar.py)
(cd h1-real && P05_PRIORI="" ESTOCOLOGIA_SAIDA=$OUT/h1-real/original ESTOCOLOGIA_DADOS=$P05_DADOS     python3 p05_rodar.py)
(cd h2g && (cd dominios && ls -d */ | tr -d / | grep -v '^_') | xargs -P 3 -I{} bash -c \
   'P05_PRIORI="" ESTOCOLOGIA_SAIDA=$OUT/h2g/original ESTOCOLOGIA_DADOS=$P05_DADOS/h2b python3 p05_um.py {}')

# rounds (b) and (c): H2g through rodar_h2g.sh, the rest through rodar.sh
P05_DADOS=$P05_DADOS P05_H="h2 h1-real" bash rodar.sh $OUT
P05_DADOS=$P05_DADOS bash rodar_h2g.sh $OUT 3

# comparison (writes resultados.json and tabela.md in this folder; default 1e-9 = strict reading of DECLARACAO.md)
python3 comparar.py $OUT
# the reading with the band of 0,05 sd of DECLARACAO_2.md (writes resultados_2.json and tabela_2.md)
python3 comparar.py $OUT --tolerancia-dp 0.05 --json resultados_2.json --tabela tabela_2.md
```

The raw `saida/` of the three rounds, the `fim.txt`/`fim_h2g.txt` with the exit code of each domain and the
`*-{original,literatura,fraca}.log` files stay in `$OUT`. The checks of this README that are not in `comparar.py`
stay there too: `checar_reproducao.py` and `comparar_33.py` (round (a) against the `saida/` of the repository, field by
field in the 33 domains), `drift_dp.py` (the deviation in units of the posterior sd), `g3.py`/`g3b.py` (Ŝ ≠ ∅ per round),
`verif_fraca.py` (the copy of (c) into (b) in the 22 domains without a source) and `numeros.py`. In `/tmp` stayed the control of the deviation: `/tmp/ctrl-h2g` (the original `h2g/`
copied) with `/tmp/ctrl_h2g.py`.

Wall time on this machine (16 cores, 3–4 workers): 34 min in total, of which 8 min for round (a) alone and
26 min for (b) and (c). What dominates are the heavy domains with the weak prior: `h2/logistica_voos`,
`h2g/fisica_tunel_luz`, `h2g/saude_rand`, `h2g/educacao_star_turmas` and `h2g/engenharia_tunel_vento`.
