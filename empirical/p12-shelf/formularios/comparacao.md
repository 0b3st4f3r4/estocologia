# Comparison: elicited prior × literature prior × package prior

**State on 2026-10-17: five elicitations by E1 done, panel not closed.** Sections 6 to 10 record the
five domains elicited end to end by the first expert (E1, the author of the program) — pre-test,
confirmed quartiles, controls read, and the third column of the funnel with the mark **P**. This is a **preliminary round of
sensitivity with $n = 1$**, and not a panel: the protocol of `empirical/p12-shelf/protocolo.md` asks for three experts
per domain, with one from outside the program, and one approved expert closes no panel at all. For that reason the
consolidated tables of this file (§"Priors side by side", §"Decision before and after", and the verdict) remain blank: they
wait for the panel, the `consenso` of each form, and `prioris_elicitadas.yaml`.
**Correction of 2026-10-17:** this header said "no elicitation happened", which contradicted the five filled lines
of §6 to §10 and the 21 lines of the rereading in `governance/RELEITURA.md`.

## Declared criterion — K = 2 of this front

This comparison is **not** a reading: it is a new criterion, and for that reason it enters as a **new declaration**, with its own hash
(`DECLARACAO_K2.md` + `DECLARACAO_K2.sha256`), written **before** comparing, and **counts in the K**. It is the
second declaration of this front — the first is the frozen protocol itself (K = 1 of the front) — and it is the second in the
count of the K = 2 that the program already uses for changes of criterion after seeing a result
(`empirical/p05-prioris/h2/README.md`, section "Changes after seeing results"; `empirical/p05-prioris/h2g/DECLARACAO_K3.md`,
"the K = 2 test of residuals").

**House rule respected:** no file of an earlier declaration is rewritten and no existing hash is swapped.
The K of this front **is added** to the K that each domain already has in the field `K` of its own `resultados.json`:

| domain | current K (source) | K after this declaration |
|---|---|---|
| `h2/desenvolvimento_preston` | 1 (`h2/saida/desenvolvimento_preston/resultados.json`) | 2 |
| `h2/direito_prisoes` | 1 (`h2/saida/direito_prisoes/resultados.json`) | 2 |
| `h2/educacao_star` | 1 (`h2/saida/educacao_star/resultados.json`) | 2 |
| `h2g/educacao_star_turmas` | 3 (`h2g/saida/educacao_star_turmas/resultados.json`) | 4 |
| `h2g/direito_teto_beneficio` | 3 (`h2g/saida/direito_teto_beneficio/resultados.json`) | 4 |
| `h1-real/c` | without field `K` in `h1-real/saida/c/resultados.json` | whoever signs opens the K of the real H1 in the declaration of the front |

If the count of the K of the program labels this with another number, the label is corrected by a **new declaration** — this
file is not rewritten to renumber.

### What counts as "changes" (fixed before comparing)

The definition is that of p05, letter by letter (`empirical/p05-prioris/DECLARACAO.md`, section "Change"): a domain **changes**
in a round if **the status category changes** (including the list of the gates that fail) **or if the action of F8 changes**
(including passing from an issued decision to none, or the contrary). A change whose only reason is a computational gate
(R-hat, ESS, SBC) counts as a change and is listed separately.

Numerically, the following are always reported, per domain and per round: mean and 90% CI of the posterior of the key, ratio posterior SD /
prior SD, P(adopting pays off), and the displacement `|m − m_(a)| / dp_(a)` in units of the posterior SD of round (a) —
the same that p05 measured. Two references for reading, measured by p05 itself and not invented here:

- the maximum **environmental deviation** of p05 is **0.0071 SD** (numpy/scipy of another machine); below that, the difference is
  numerical, not of content;
- the **smallest displacement that changed a decision** in p05 was **0.18 SD** — twenty-five times the environmental deviation.

That is: a displacement ≥ 0.18 SD with a change of category or of action is the case that matters; a displacement below
0.0071 SD is not a new reading.

### What is already known (and for that reason does not decide alone)

Round (b) of p05 **already** ran with the literature prior: three of the six domains changed status
(`h2/direito_prisoes`, `h2/educacao_star`, `h2g/educacao_star_turmas` pruned, `h1-real/c`
approved) and two changed action (`h2/desenvolvimento_preston` adopt → maintain; `h1-real/c` exposure 0.4 →
0.0). The table is in section 6.1 of `empirical/p12-shelf/protocolo.md`, with source in
`empirical/p05-prioris/tabela.md`. Two consequences for the reading of this comparison, declared here:

1. the elicited prior enters **after** the literature one was seen — whoever compares is not blind;
2. for that reason what matters is not "does the elicited one change?", but **where it falls between the package and the literature** (or outside
   both), and whether the decision with it is that of the package, that of the literature, or a third one.

## Priors side by side

Adjustment made with the functions of the repository itself: `ajustar_normal` (`h2f/estocologia/dominio.py`) for the
gaussian domains, with the quartiles of `h2/direito_prisoes` in **log of the odds ratio**
(`priori_da_chave`, same file), and `ajustar_beta_quartis` (`h2f/estocologia/estat.py`) for the real H1. The columns "package" and "literature" are a checked copy; the line "elicited" stays **blank in this consolidated
table**, and the value of E1 for each domain is in §6 to §10, on the same scale and with the same quartiles.

| domain | priority | q05 | q25 | q50 | q75 | q95 | adjustment | max. error in the quartiles | 90% width ratio |
|---|---|---|---|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | package (`pacote.yaml`) | 3 | 5 | 6.5 | 8 | 10.5 | N(6.5; SD 2.223903) | 5.2e-13 | 1.0252 |
| `h2/desenvolvimento_preston` | literature p05 (`prioris/h2/literatura.yaml`) | −2.746195 | −0.219960 | 1.536 | 3.291960 | 5.818195 | N(1.536; SD 2.603390) | 2.8e-13 | 1.0000 |
| `h2/desenvolvimento_preston` | **elicited (panel)** | — | — | — | — | — | — | — | — |
| `h2/direito_prisoes` | package (`pacote.yaml`) | 0.42 | 0.58 | 0.7 | 0.85 | 1.05 | log(OR): N(−0.354640; SD 0.283331) | 2.0e-03 | 0.9831 |
| `h2/direito_prisoes` | literature p05 (`prioris/h2/literatura.yaml`) | 0.594406 | 0.657895 | 0.705978 | 0.757576 | 0.838493 | log(OR): N(−0.348171; SD 0.104582) | 3.5e-14 | 1.0000 |
| `h2/direito_prisoes` | **elicited (panel)** | — | — | — | — | — | — | — | — |
| `h2/educacao_star` | package (`pacote.yaml`) | 2 | 8 | 12 | 16 | 24 | N(12; SD 5.930409) | 1.6e-12 | 1.1277 |
| `h2/educacao_star` | literature p05 (`prioris/h2/literatura.yaml`) | 1.302178 | 4.860110 | 7.333191 | 9.806272 | 13.364204 | N(7.333191; SD 3.666596) | 4.8e-13 | 1.0000 |
| `h2/educacao_star` | **elicited (panel)** | — | — | — | — | — | — | — | — |
| `h2g/educacao_star_turmas` | package (`pacote.yaml`) | 0.05 | 0.13 | 0.19 | 0.25 | 0.34 | N(0.19; SD 0.088956) | 4.1e-14 | 0.9910 |
| `h2g/educacao_star_turmas` | literature p05 (`prioris/h2g/literatura.yaml`) | 0.017804 | 0.066451 | 0.100265 | 0.134079 | 0.182726 | N(0.100265; SD 0.050133) | 1.7e-14 | 1.0000 |
| `h2g/educacao_star_turmas` | **elicited (panel)** | — | — | — | — | — | — | — | — |
| `h2g/direito_teto_beneficio` | package (`pacote.yaml`) | 0.0 | 0.2 | 0.35 | 0.5 | 0.75 | N(0.35; SD 0.222390) | 6.7e-14 | 1.0252 |
| `h2g/direito_teto_beneficio` | literature p05 (`prioris/h2g/literatura.yaml`) | 0.228067 | 0.3 | 0.35 | 0.4 | 0.471933 | N(0.35; SD 0.074130) | 3.2e-14 | 1.0000 |
| `h2g/direito_teto_beneficio` | **elicited (panel)** | — | — | — | — | — | — | — | — |
| `h1-real/c` | project, round (a): weak Beta(1,1) | 0.05 | 0.25 | 0.5 | 0.75 | 0.95 | Beta(1; 1) | 4.7e-12 | — |
| `h1-real/c` | literature p05 (`prioris/h1-real/literatura.yaml`) | 0.05 (deriv.) | 0.25 | 0.5 | 0.75 | 0.95 (deriv.) | Beta(1; 1) | 4.7e-12 | — |
| `h1-real/c` | **elicited (panel)** | — | — | — | — | — | — | — | — |

Reading of the two filled lines (descriptive arithmetic, without any decision): in five of the six domains the literature prior is
**narrower** than that of the package (SD of 2.60 against 2.22 in Preston; 3.67 against 5.93 in STAR; 0.050 against
0.089 in the classes; 0.074 against 0.222 in the ceiling; and, in log of the OR, 0.105 against 0.283 in law). It is that difference of
width that moves the posterior: the elicited prior that comes out wider than the literature one **shrinks** the displacement already
observed in round (b) of p05.

## Decision before and after

Per domain. "Before" = round (a) of p05, with the prior of the package, as it is in the `saida/` of the repository. "After"
= the round with the elicited prior, to be run exactly as p05 ran it (copies in `empirical/p05-prioris/`, the
table of the prior swapped for `formularios/prioris_elicitadas.yaml`, same seeds, data, costs, and benefits).

| domain | status (a) | status (after) | action (a) | action (after) | posterior mean (a) → (after) | ratio post/prior SD (a) → (after) | displ. in SD of (a) | changed? |
|---|---|---|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | approved | — | adopt | — | 5.09 → — | 0.26 → — | — | — |
| `h2/direito_prisoes` | rejected: modelo_vence_climatologia | — | — | — | −0.73 → — | 0.28 → — | — | — |
| `h2/educacao_star` | approved | — | adopt | — | 13.5 → — | 0.61 → — | — | — |
| `h2g/educacao_star_turmas` | approved | — | maintain | — | 0.148 → — | 0.38 → — | — | — |
| `h2g/direito_teto_beneficio` | approved | — | adopt | — | 0.579 → — | 0.24 → — | — | — |
| `h1-real/c` | rejected: modelo_vence_climatologia | — | 0.4 (exposure f) | — | — | 0.87 → — | — | — |

Columns "a" copied from `empirical/p05-prioris/tabela.md` (with `h2/desenvolvimento_preston`, the educations, and
law in `empirical/p05-prioris/README.md`, section of the five changes of (b)).

### Line of comparison with round (b) of p05, which already exists

| domain | status (b) literature | action (b) | displ. (b) in SD | reading to be made after the elicited one |
|---|---|---|---|---|
| `h2/desenvolvimento_preston` | approved | maintain | 0.48 | does the elicited one return to "adopt" or repeat the "maintain"? |
| `h2/direito_prisoes` | pruned at F1 (EVPI < cost) | — | — (pruned before F4) | does the elicited one also prune at F1, or does the domain reach F4? |
| `h2/educacao_star` | pruned at F2 | — | — | idem, at F2 |
| `h2g/educacao_star_turmas` | pruned at F1 (EVPI < cost) | — | — | idem, at F1 |
| `h2g/direito_teto_beneficio` | approved | adopt | 1.4 | does the largest displacement of p05 repeat itself? |
| `h1-real/c` | approved | 0.0 (exposure) | — (the `comparar.py` does not compute in H1) | does the elicited one repeat the 0.0 or return to 0.4? |

## Verdict, to be written later

Fixed rule (declared beforehand): let N_p and N_l be the numbers of domains that change status or action **with respect to the package**
(N_p) and **with respect to the literature of p05** (N_l), over the 6 domains.

- N_p = 0 and N_l = 0: the elicited prior changes nothing; the conclusions did not depend on who elicited.
- N_p > 0 and N_l = 0: the elicited one agrees with the literature and disagrees with the package — what changes is the informative prior,
  not the panel.
- N_l > 0: the panel says something **different from what the literature already said**; the verdict names each domain, the sign of the
  displacement, and whether the source is independent or of **same base** (`h2/educacao_star`,
  `h2g/educacao_star_turmas`, `h2g/direito_teto_beneficio`).
- In any case: to say whether the G3 of H2g (Ŝ ≠ ∅ in at least 3 areas) changes — the ICP does not use the prior, so it only changes if
  a domain is pruned before F4.

**Fields missing to be filled:** the consolidated value of each domain in the three tables above — the one of E1 exists per
domain in §6 to §10 —, the
`consenso` of each `formularios/<horizonte>__<dominio>.yaml`, the columns "after" of the two tables of
decision, the displacement in SD, the N_p, the N_l, and the verdict.

## How to reproduce / how to fill in

1. Elicit and record as `empirical/p12-shelf/protocolo.md` orders: each form per expert, with
   pre-test of 20 resolved predictions, controls of anchor, order, and overconfidence, and return.
2. Fill in the block `consenso` of each form and take it to
   `formularios/prioris_elicitadas.yaml` as a **new block** (nothing is rewritten).
3. Adjust with the functions of the repository — the same ones used in the table of priors of this file:

```python
import sys; sys.path.insert(0, "h2f")
from estocologia.dominio import ajustar_normal          # gaussians (and binomials, with the quartiles in log of the OR)
from estocologia.estat import ajustar_beta_quartis      # real H1 (frequency)
ajustar_normal({"q05": 3, "q25": 5, "q50": 6.5, "q75": 8, "q95": 10.5})
```

4. Run the comparison as p05 ran it (copies in `empirical/p05-prioris/`, the table of the prior through
   `$P05_PRIORI`, `OMP_NUM_THREADS=1`, at most 2 processes in parallel) and fill in the two tables of
   decision.
5. Write the verdict and freeze the declaration with hash (`DECLARACAO_K2.md` + `DECLARACAO_K2.sha256`).

---

# Filling of 2026-10-16 (partial): the domain \`h2/desenvolvimento_preston\`

The skeleton above still holds and was not rewritten. What follows is what can already be filled in, what cannot,
and why.

## 1. The three priors, side by side

| prior | q05 | q25 | q50 | q75 | q95 | mean | SD | source |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **package** | 3 | 5 | 6.5 | 8 | 10.5 | 6.5 | 2.223903 | \`empirical/p05-prioris/h2/dominios/desenvolvimento_preston/pacote.yaml\`, block \`shelf\`: literature of Preston "cited from memory and to be checked; no human expert consulted" |
| **literature (p05)** | −2.746195205080443 | −0.21995975644268073 | 1.536 | 3.291959756442681 | 5.818195205080443 | 1.536 | 2.603390 | p05, Pritchett & Summers (1996), rule R1 |
| **elicited (E1, n = 1)** | −0.50 | 1.75 | 3.25 | 4.75 | 7.00 | 3.25 | 2.223903 | \`formularios/h2__desenvolvimento_preston.yaml\`, round 1 confirmed; adjustment and controls in \`labs --elicitacao\` |

The elicited prior has the **same shape** as the package — SD 2.223903 in both — without having seen the package: the expert
received no number of the domain before writing the five quantiles. What changes is the **centre**: 3.25, that is,
2.75 years above the literature and 3.25 below the package. The elicited 90% width is 7.5 against 7.3160 implied
(ratio 1.0252), at the upper limit of the overconfidence control, and the tails of the expert are wider
than his own quartiles would require.

## 2. The reading in the plane of the prior itself

The decision threshold of this domain is in the `pacote.yaml` itself: the income channel pays off if `theta` exceeds
**5 years** (benefit 1.0 per unit against cost 5.0).

| prior | `P(theta > 5)` in the prior itself |
| --- | --- |
| package | **0.7500** |
| literature | **0.0917** |
| elicited | **0.2157** |

```bash
cargo run --release --bin labs -- --elicitacao "<q05,q25,q50,q75,q95>" --limiar 5
```

Reading: of the two informative priors (literature and elicited), both put `P(theta > 5)` well below 0.50 —
the elicited one at 0.2157, almost twice the literature and less than one third of the package. It is the same side of the threshold for
both, and it is the side that p05 had already seen: with the literature, the action of F8 of this domain became **maintain** (it was
`adotar` with the package).

## 3. What is **not** computable today, and why

p05 measured the complete passage through the funnel with each prior. For this domain, its table carries: status
`aprovado` in the three rounds, action of F8 `adotar / manter / manter`, posterior mean 5.09 / 4.81 / 4.98, and
`P(adotar compensa)` 0.565 / 0.366 / 0.495 — the last one is the weak prior, not the elicitation.

Redoing that passage with the elicited prior requires the chain that turns a prior into an action: the `preparo.py` of the
domain and the machinery of `h2f/estocologia`, **retired on 2026-10-16**
(`docs/declarations/REMOVED_PYTHON.md`). What was left in the tree is the `pacote.yaml` of the domain, not the code that runs.
Therefore the column **(c) elicited** stays **empty**, marked as empty, and not filled in by approximation.

Two declared paths, and neither decided here:

1. run the copy of p05 that was left in the tree, accepting the mark **P** of `docs/EMPIRICAL_REGISTRY.md` (a number that only
   exists in Python) — which keeps the comparison apple to apple, because columns (a) and (b) came out of the same code;
2. reimplement the funnel of the domain in Rust and fill in the column with an instrument — it is the path of item 12 of the
   inventory, which already asked for the second opinion on the results marked **P**.

## 4. What this filling cannot say

- **whether the elicited prior changes the action of F8** of this domain: it depends on section 3, and today it does not close;
- **the consensus of the panel**: this domain has **1** approved expert, and section 2.4 of the protocol says that with
  fewer than three the domain stays **not elicited**. What is here is the elicitation of E1, kept as
  **sensitivity**, and it does not substitute the prior of the package in the official round;
- **the outside control**: E1 is the author of the program, so the panel does not have the outside expert that the
  protocol requires as the main control;
- **the order control**: with a single expert there is no half of the panel in the reverse order, and that stays declared
  instead of simulated.

## 5. K

The front had **K = 2** (protocol = 1, comparison = 2) and the declaration of adjustment of the pre-test took it to **K = 3**.
This filling **does not** add new K: while column (c) does not close, what is written here is descriptive
arithmetic over already published priors, with a criterion that was already declared.


---

## 6. Column (c) filled in, with the mark **P** — declared decision of the operator

**Decision of 2026-10-16:** to run the **retired copy of p05** (the same code that generated columns (a) and (b)),
and not to reimplement the funnel in Rust now. Declared reason: the table compares the **sensitivity of the decision** to three
priors, and using the same code is what guarantees an apple-to-apple comparison; the reimplementation of the funnel in Rust stays in
its own scope, item 12 of the inventory. The number that comes out of here carries the mark **P** of `docs/EMPIRICAL_REGISTRY.md`.

### How to run, and why outside the tree

The code of p05 left the tree on 2026-10-16 and lives in commit `1bfec905`. The copy was restored **outside the
tree** and run there, so that the repository continues to be Rust only (§VIII of the contract):
~~~bash
rm -rf /tmp/p05run && mkdir -p /tmp/p05run
git archive 1bfec905 empirical/p05-prioris data/h2 | tar -x -C /tmp/p05run
# the elicited prior enters the format of p05 (source, panel, and quartiles)
# /tmp/p05run/empirical/p05-prioris/prioris/h2/elicitada.yaml
cd /tmp/p05run/empirical/p05-prioris/h2
for v in original literatura elicitada; do
  tab=""; [ "$v" != original ] && tab="/tmp/p05run/empirical/p05-prioris/prioris/h2/$v.yaml"
  P05_PRIORI="$tab" ESTOCOLOGIA_DADOS=/tmp/p05run/data/h2 ESTOCOLOGIA_SAIDA=/tmp/p05run/saida/$v \
    python3 p05_rodar.py agronomia_nitrogenio clima_tempestades credito_hmda desenvolvimento_preston
done
~~~

**The list of domains is not decoration.** The `p05_rodar.py` gives the seed `2027 + 100 i`, with `i` the position of the domain
in the requested list. `desenvolvimento_preston` is the fourth in the alphabetical order of the ten domains, so the first three
enter the call only so that its seed continues to be **2327**, the same as that of the round of p05. Running only preston
would change the seed and the number would not be comparable.

### The checking that authorizes the third column

Columns (a) and (b) of p05 come out **the same** when the same code runs again. The table of p05 carries posterior mean
5.09 / 4.81 and `P(adotar compensa)` 0.565 / 0.366; today's round gave 5.087198 / 4.806199 and 0.565375 / 0.366375.
Whoever doubts it runs the block above and compares.

### The three rounds, side by side

| round | status | action of F8 | P(adopting pays off) | posterior mean | 90% CI | ratio post/prior SD | displ. in SD of (a) | P(theta > 5) in the prior (F1) | EVPI |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **package** | approved at F4 | adopt | **0.565375** | 5.087198 | [4.142920; 6.040198] | 0.2638 | 0 | 0.75285 | 0.32340 |
| (b) **literature** | approved at F4 | maintain | **0.366375** | 4.806199 | [3.828045; 5.781023] | 0.2293 | 0.4790 | 0.09210 | 0.11246 |
| (c) **elicited (E1)** | approved at F4 | **maintain** | **0.418875** | 4.867977 | [3.913771; 5.836887] | 0.2656 | 0.3737 | 0.21245 | 0.27330 |

The posterior SD of round (a) is 0.586673, and it is in it that the displacement is measured.

### What the new line says

- **The action of F8 changes from `adotar` to `manter`** with the elicited prior, as it already changed with the literature one. The
  status does not change: `aprovado no F4`, decision issued.
- **`P(adotar compensa)` falls from 0.565375 to 0.418875** and crosses the threshold of 0.50. The literature takes it to 0.366375: the
  elicited one stays **between the package and the literature**, closer to the literature.
- **The displacement is 0.3737 SD of round (a)** — smaller than that of the literature (0.4790), as expected, because the posterior
  mean of the elicited one (4.867977) is closer to that of the package (5.087198) than that of the literature (4.806199).
- **The ratio posterior SD / prior SD is 0.2656**, practically that of the package (0.2638): the elicited prior has the same
  shape as the package, so the data tightens the posterior almost equally.
- **The elicited prior is the only one of the three with a still high price of research** (EVPI 0.27330 against 0.11246 of the
  literature), because it is wider than it — and F1 passes in the three.

### Limits of this line, declared

- **Mark P.** The number comes from the retired Python copy, not from a Rust instrument. It is the same path as columns
  (a) and (b), and that is why the comparison is worth something; the second opinion in Rust is item 12 of the inventory.
- **Round (c) is sensitivity, not an official round.** The panel of the domain has **1** approved expert, and he is the
  author of the program; section 2.4 of the protocol says that with fewer than three the domain stays **not elicited**.
- **What this line does not measure:** the effect of a prior elicited *by three experts with one from outside*. With n = 1
  and without the outside control, what was measured was the sensitivity of the funnel to the head of one person — the same one who had already
  written the package.


---

## 7. Second domain filled in, with the mark **P**: \`h2/direito_prisoes\`

Same path as the first (decision of the operator of 2026-10-16): retired copy of p05 restored outside the tree, in commit \`1bfec905\`, and the three priors run by the **same code**. The elicited prior enters the format of p05 in \`prioris/h2/elicitada_direito.yaml\`.

**The seed preserved again.** \`direito_prisoes\` is the fifth domain in the alphabetical order of the ten, so the call carries the first four before it so that the seed continues to be \`2027 + 100 × 4 = 2427\`, the same as that of the round of p05.

### The three priors, side by side (odds-ratio scale, and the log that the adjustment uses)

| prior | q05 | q25 | q50 | q75 | q95 | mean in the log | SD in the log |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **package** | 0.42 | 0.58 | 0.70 | 0.85 | 1.05 | −0.354640 | 0.283331 |
| **literature (p05)** | 0.5944060189976956 | 0.6578947368421053 | 0.7059781184061313 | 0.7575757575757576 | 0.8384926931067868 | −0.348171 | 0.104582 |
| **elicited (E1)** | 0.20 | 0.45 | 0.65 | 0.85 | 1.15 | −0.463937 | 0.471459 |

The elicited one has the **largest width of the three** in the log (0.471459 against 0.283331 of the package and 0.104582 of the literature) and the most pessimistic median (0.65; package 0.70 and literature 0.706). The expert declared the reason: methodological scepticism about artificially narrow meta-analysis intervals.

### The checking that authorizes the third column

| round | what the table of p05 says | what today's round gave |
| --- | --- | --- |
| (a) package | rejected: modelo_vence_climatologia; posterior mean −0.73; ratio SD 0.28 | rejected at F4 (M15): modelo_vence_climatologia; mean −0.729791; ratio 0.275878 |
| (b) literature | pruned at F1: the research does not change the decision (EVPI < cost) | pruned at F1: the research does not change the decision (EVPI < cost); adjusted prior with mean −0.348171 and SD 0.104582 |

### The three rounds

| round | status | EVPI in the prior (F1) | P(adopting pays off) in the prior | action of F8 | posterior mean | 90% CI | ratio post/prior SD |
| --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **package** | rejected at F4 (M15): modelo_vence_climatologia | 0.541955 | 0.79195 | — | −0.729791 | [−0.859514; −0.603526] | 0.275878 |
| (b) **literature** | pruned at F1: EVPI < cost | 0.009989 | 0.98290 | — | — | — | — |
| (c) **elicited (E1)** | rejected at F4 (M15): modelo_vence_climatologia | **1.054148** | 0.76510 | — | **−0.752532** | [−0.885605; −0.623203] | **0.169906** |

### What the new line says, and it is different from the first domain

- **The action does not change in any of the three**: with the package, with the literature, and with the elicited one, the domain stays
  **rejected at F4** — the model does not beat the climatology — and no decision is issued. Here the controversy about
  the prior **decides nothing**, because what blocks it is another gate.
- **The EVPI explodes with the elicited one**: 0.541955 (package) → 0.009989 (literature) → **1.054148** (elicited). The
  widest prior of the three is the one that gives the most value to the research — and it is the only one of the informative ones that survives F1,
  where the literature one was pruned.
- **The posterior moves little**: −0.729791 (package) against −0.752532 (elicited), 2.3% of difference, with the ratio
  post/prior SD falling from 0.275878 to 0.169906 — with a wide prior, the data pulls more.
- **The underlying reading**: in \`h2/desenvolvimento_preston\` the elicited prior crossed the threshold of F8 and turned into the action;
  here it turns into nothing, and what it moves is the **value of the research**. Two faces of the same elicitation.

### Limits, the same as those of the first domain

- **Mark P**: the number comes from the retired Python, the same that generated columns (a) and (b).
- **Sensitivity, not an official round**: the panel has 1 expert and he is the author of the program.
- **The elicited width is the dominant variable**, and it came from declared scepticism, not from data: if a panel
  of three experts narrows that distribution, the EVPI falls with it and F1 comes back to pruning — which is what p05 had already
  shown with the literature.


---

## 8. Third domain filled in, with the mark **P**: \`h2/educacao_star\`

Same path (copy of p05 outside the tree, commit \`1bfec905\`, three priors by the same code) and the seed preserved again: \`educacao_star\` is the sixth in the alphabetical order, so the call carries the first five before it so that the seed continues to be \`2027 + 100 × 5 = 2527\`.

### The three priors (SAT points)

| prior | q05 | q25 | q50 | q75 | q95 | mean | SD |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **package** | 2.2453 | 8.0000 | 12.0000 | 16.0000 | 21.7547 | 12.0000 | 5.930409 |
| **literature (p05)** | 1.3021781198967624 | 4.860110016547187 | 7.333191149998485 | 9.806272283449783 | 13.364204180100208 | 7.333191 | 3.666596 |
| **elicited (E1)** | −2.5 | 6.0 | 12.0 | 18.0 | 26.5 | 12.0000 | 8.895613 |

**The literature of this domain carries the mark `mesma base`**: it was born from the very `AER_STAR` that the domain uses, and for that reason it is not independent evidence. The elicited one has the same median as the package (12.0) and the **largest deviation of the three** (8.895613).

### The checking that authorizes the third column

| round | what the table of p05 says | what today's round gave |
| --- | --- | --- |
| (a) package | approved; adopt; posterior mean 13.5; ratio SD 0.61 | approved at F4; adopt; mean 13.518436; ratio 0.609555 |
| (b) literature | pruned at F2: data at the independent unit do not inform more than the prior | pruned at F2, with the same justification; design effect 3.343892 |

### The three rounds

| round | status | EVPI (F1) | F2 informative | action of F8 | P(adopting pays off) | posterior mean | 90% CI | ratio post/prior SD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **package** | approved at F4 | 1.502145 | yes | adopt | **0.839625** | 13.518436 | [7.485530; 19.505214] | 0.609555 |
| (b) **literature** | **pruned at F2** | 0.497782 | **no** | — | — | — | — | — |
| (c) **elicited (E1)** | approved at F4 | **2.640820** | yes | **adopt** | **0.836500** | **13.879785** | [7.140456; 20.576858] | **0.454279** |

### What the new line says, and it is the most informative result of the three domains

- **The action does not change**: `adotar`, with P(adopting pays off) **0.836500** against **0.839625** of the package — a difference of 0.4%. The elicited one has the same median as the package and widens the dispersion, without touching the decision.
- **F2 changes, and it is the finding**: the prior of **same base** was **pruned at F2** — "data at the independent unit do not inform more than the prior" —, and the elicited one passes. The reason is direct: a prior built from the very base of the domain, and narrow, **makes the data redundant**; F2 catches that. Widening the prior gives value back to the data. It is the toll of circularity, measured.
- **The EVPI rises with the elicited one**: 2.640820 against 1.502145 of the package and 0.497782 of the literature.
- **The posterior barely moves** (13.879785 against 13.518436, +2.7%), but the ratio post/prior SD falls from 0.609555 to 0.454279.
- **Comparison with \`h2g/educacao_star_turmas\`**, which is the other side of the pair of the same base: there the literature was also pruned at F1. The two sides of the pair behave the same, and that is what is expected of a prior that adds no new information.

### Limits, the same as those of the other domains

- **Mark P**: the number comes from the retired Python.
- **Sensitivity, not an official round**: panel of 1 of 3, and the expert is the author of the program.
- **The exclusion of the pair of the same base still holds** for when the panel exists: the same person cannot serve \`h2/educacao_star\` and \`h2g/educacao_star_turmas\`.


---

## 9. Fourth domain filled in, with the mark **P**: \`h2g/direito_teto_beneficio\`

Same path, with a difference of folder: this domain lives in the **\`h2g\`** of p05 and reads the base **\`data/h2b\`**. The seed preserved again — \`direito_teto_beneficio\` is the **second** in the alphabetical order of the eighteen domains of h2g, so the call carries \`biologia_sachs\` before it so that the seed continues to be \`2027 + 100 × 1 = 2127\`.

### The three priors (log-log coefficient)

| prior | q05 | q25 | q50 | q75 | q95 | mean | SD |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **package** | **0.00** | 0.20 | 0.35 | 0.50 | **0.75** | 0.350000 | 0.222390 |
| **literature (p05)** | 0.22806681817823804 | 0.30 | 0.35 | 0.40 | 0.4719331818217619 | 0.350000 | 0.074130 |
| **elicited (E1)** | **0.05** | 0.20 | 0.35 | 0.50 | **0.65** | 0.350000 | 0.222390 |

### The checking that authorizes the third column

| round | what the table of p05 says | what today's round gave |
| --- | --- | --- |
| (a) package | approved; adopt; posterior mean 0.579; ratio SD 0.24 | approved at F4; adopt; mean 0.578765; ratio 0.242860 |
| (b) literature | approved; adopt; mean 0.506; ratio 0.6 | approved at F4; adopt; mean 0.505514; ratio 0.602756 |

### The three rounds

| round | status | EVPI (F1) | action of F8 | P(adopting pays off) | posterior mean | 90% CI | ratio post/prior SD |
| --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **package** | approved at F4 | 0.087187 | adopt | 1.0 | 0.578765 | [0.491437; 0.665412] | 0.242860 |
| (b) **literature** | approved at F4 | 0.029062 | adopt | 0.99875 | 0.505514 | [0.433538; 0.576837] | 0.602756 |
| (c) **elicited (E1)** | approved at F4 | **0.087187** | adopt | **1.0** | **0.578765** | **[0.491437; 0.665412]** | **0.242860** |

### The finding: the elicitation reproduced the package, and the funnel came out identical

Line (c) is **equal to line (a) down to the last digit** — status, action, P(adopting pays off), posterior mean, 90% CI, and ratio SD. It is not a coincidence of rounding: the **three quartiles that the adjustment uses** (q25, q50, q75) are the same in the package and in the elicitation — 0.20, 0.35, and 0.50 —, and the adjustment of the Normal is by least squares **on those three**. The \`priori\` of the two rounds comes out with mean 0.350000 and SD 0.222390, and from there everything else is identical by construction.

What differs are the **tails**, and the adjustment ignores them: q05 0.05 against 0.00 of the package, and q95 0.65 against 0.75. They appear only in the controls, and there, yes, there is a difference — the ratio of the 90% width is **0.8201** in the elicited one and **1.0000** in the package.

**The reading.** In this domain the formal elicitation **brought no new prior: it documented the prior that already existed**. The package declares, in its own letter, "Meyer, Viscusi & Durbin (1995)… cited from memory, to be checked; **no human expert consulted**". Now there is an expert, an approved pre-test (skill 0.8585), a written justification, and two signed notes — and the three central quartiles that he produced are the same ones that the package carried. It is the first **validation** of a package prior of this tree by formal elicitation, and it is worth what it is worth: it measures the consistency of the author with himself over time, and not agreement with people from outside.

**Correction of a sentence of the return.** When returning the numbers, the facilitator wrote that "p05 already recorded that the two sides of this pair were pruned in the funnel". That is **not** the case of this domain: with the literature prior it continued **approved at F4**, and the table of p05 shows `adotar` in the three rounds. What was pruned, in p05, were \`h2/direito_prisoes\` (at F1) and \`h2g/educacao_star_turmas\` (at F1) — and the mark \`mesma base\` of this domain still holds for the right reason: the evidence is not independent. The sentence is corrected here, and the expert was warned.

### Limits

- **Mark P** and **sensitivity, not an official round**: panel of 1 of 3, and the expert is the author of the program.
- **The finding has a ceiling**: reproducing the package is not confirming the estimate against the world. It is the author arriving at the same place by two paths — which is useful against transcription error and useless against error of reading of the literature.
- **The adjustment discards the tails**: the differences of q05 and q95 between package and elicitation do not enter the funnel, because the Normal is adjusted on the three central quartiles. It stays recorded as a declared limit of the adjustment, not as a choice of this domain.


---

## 10. Fifth domain filled in, with the mark **P**: \`h1-real/c\`

The pipeline lives in the **\`h1-real\`** of p05 and runs the three projects (\`a\`, \`b\`, \`c\`) through \`p05_rodar.py\`, with seed \`2026 + 100 × i\`. Since project \`c\` is the third, the call has to be the complete one (without arguments) so that its seed continues to be **2226**. The bases of the real H1 are the five CSVs of \`data/\`, copied outside the tree so that p05 runs.

### The three priors (monthly frequency of the layer "100 xs 50")

| prior | q05 | q25 | q50 | q75 | q95 | adjusted Beta (a, b) | implied mean |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **(a) simulated panel of p05** | — | — | — | — | — | 2.327958 / 106.930083 | 0.021307 |
| **(b) weak Beta(1; 1)** | 0.05 (derived) | 0.25 | 0.50 | 0.75 | 0.95 (derived) | 1.000000 / 1.000000 | 0.500000 |
| **(c) elicited (E1)** | 0.01 | 0.04 | 0.08 | 0.15 | 0.30 | **1.168188 / 9.911852** | **0.105432** |

The elicited one has a median **six times smaller** than the weak one and 2.5 times larger than the simulated panel. And the adjustment that the retired Python makes of it — \`a = 1.168188\`, \`b = 9.911852\` — **matches the Beta adjustment that I wrote in Rust**: \`a = 1.171038\`, \`b = 9.919921\`, a difference of 0.24% and 0.08%. Two independent implementations, the same result: it is the second opinion of the adjustment, and it closes.

### The checking that authorizes the third column

| round | what the table of p05 says | what today's round gave |
| --- | --- | --- |
| (a) simulated panel | rejected: modelo_vence_climatologia; ratio post/prior SD 0.87 | rejected (\`modelo_vence_climatologia\` false in the gate); ratio **0.872232** |
| (b) weak Beta(1; 1) | approved; ratio 0.072 | approved; ratio **0.072434** |

### The three rounds

| round | gate \`modelo_vence_climatologia\` | approved | ratio posterior SD / prior SD | mean of the prior |
| --- | --- | --- | --- | --- |
| (a) **simulated panel** | false | **no** | 0.872232 | 0.021307 |
| (b) **weak Beta(1; 1)** | true | **yes** | 0.072434 | 0.500000 |
| (c) **elicited (E1)** | true | **yes** | **0.221084** | **0.105432** |

### What the new line says

- **The elicitation rescues the project**, as the weak one already rescued it: \`h1-real/c\` is rejected by the simulated panel of p05 and passes with the elicited prior. What changes is the **quality of the prior**: the weak one passed with a uniform declared without a source; the elicited one passes with a Beta of median 0.08, built by an expert, with an approved pre-test and a written justification.
- **The ratio post/prior SD stays between the two**: 0.221084 against 0.872232 of the simulated panel and 0.072434 of the weak one. This is the measure of the information: the elicited prior is more informative than the uniform and less than the simulated panel, and the data tightens it in the corresponding proportion.
- **The literature priority of this domain does not exist** — p05 declares, in so many words, \`SEM FONTE CONFERIDA\` and uses the weak one. So this is the only one of the five domains in which the elicitation does not compare with any literature: it **occupies the empty place** that p05 had filled with a uniform, and it occupied it with a number.

### Limits

- **Mark P**: the number of the funnel comes from the retired Python.
- **Sensitivity, not an official round**: panel of 1 of 3, and the expert is the author of the program. This is the **last domain of his queue**; \`h2g/educacao_star_turmas\` stays reserved.
- **What this domain does not resolve**: the elicited one passes the gate, but that does **not** validate the frequency of 0.08 against the world — the Danish series has 108 months and the project itself declares the undecidable premises (premium as a multiple of the \`burning cost\`, uncertain ξ, months treated as independent although the frequency grows in the decade). What was proved is that the decision changes with the prior, and that now that prior is a declaration with name and date instead of a uniform.
