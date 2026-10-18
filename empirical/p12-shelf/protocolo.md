# Point 12 (SHELF): elicitation protocol and form of the panel

**State: written before any elicitation. No expert was consulted, no prediction was corrected,
no form field is filled in, and nothing is signed.** This file is an instrument: it fixes the pre-test, the
composition of the panel, the form of the elicitation, the controls, and the rule of entry as a source. What the experts
answer enters `formularios/`; the comparison with the literature prior enters
`formularios/comparacao.md`.

## Result

There is no result of analysis in this front, and there is no way for there to be one: everything here depends on people. What exists today:

| artifact | what it is | state |
|---|---|---|
| `protocolo.md` (this one) | complete protocol: calibration pre-test, panel, elicitation, controls, rule of entry, domains, and starting quartiles | written; criteria fixed |
| `formularios/h2__*.yaml`, `formularios/h2g__*.yaml`, `formularios/h1-real__c.yaml` | one blank form per domain (18 expert blocks, 6 domains) | blank |
| `formularios/prioris_elicitadas.yaml` | **example** filled with the quartiles of the **literature** prior of p05, to fix the format and serve as a line of comparison | filled (it is not elicitation) |
| `formularios/comparacao.md` | skeleton of the comparison: priors side by side, decision before and after, declared criterion of K | skeleton |
| `../../governance/RELEITURA.md` (Block K) | form of rereading of the 21 opinions, with the 21 SHA-256 checked | blank |

The starting numbers of this protocol are in "Domains to elicit" (section 6), all copied from
`empirical/p05-prioris/prioris/h2/literatura.yaml`, `.../h2g/literatura.yaml`, and `.../h1-real/literatura.yaml`,
with the path cited, and checked in the file.

## House rule that governs this front

- **Declaration first, with hash.** Before the first expert answers anything, this protocol is frozen:
  `DECLARACAO.md` + `DECLARACAO.sha256` (SHA-256 of **this** file and the UTC time). Once frozen, this
  file is not rewritten: any change of criterion enters as a **new declaration**, with its own hash, and counts in the K.
- **This file is not yet frozen, and the hash is not filled in by whoever wrote it.** Whoever freezes it is whoever
  signs. Hash: ______ (blank).
- **Nothing is real until it is written with name and date.** What is not in the form did not happen: a verbal agreement on
  quartiles does not enter the elicitation.
- **Failure of execution and deviation of design are a result**, not a reason for a silent new attempt: a rejected pre-test,
  an expert who gives up, a control that blows up, and an unconfirmed return are reported as they are.

## 1. What this point changes and what it does not change

It changes **one** thing: the prior distribution of the coefficient of the key (the `shelf` block of the domain package, and the
corresponding prior in the real H1). It changes absolutely nothing else: data, independent unit, environments and vault,
DAG, adjustment set, the code of the funnel (F1 to F4 and F8), the costs and benefits of the decision, the seeds, and the
thresholds of the gates.

The comparison is the same as that of p05: status category (pruned at F1/F2/F3, rejected at F4 with the list of the gates that
fail, approved), action of F8, mean and 90% CI of the posterior of the key, ratio posterior SD / prior SD, and the displacement of the
posterior mean in units of the posterior SD of round (a) — `|m_elicitada − m_pacote| / dp_pacote`.

What is different from p05: there the prior was a reading of literature checked online ("no human expert
consulted", said in the `empirical/p05-prioris/DECLARACAO.md` itself); here it is the consensus of a panel with a pre-test. For that
reason the elicitation is only worth as a source if (i) each expert passes the pre-test of section 2 and (ii) there is in the panel an
expert from outside the area of the program (section 3).

## 2. Calibration pre-test: 20 already resolved predictions

Before eliciting the domain, **each** expert does his own calibration pre-test. Without passing it, he does not enter as a
source (section 2.4).

### 2.1 The 20 items

- **20 binary items, already resolved**, of the same class of question as the domain (the same form of outcome: yes/no of an
  event of the type of the domain), with the result known **before** the pre-test happens.
- **Outside the vault of the domain.** The items cannot come from the vault (reserved environment) of the domain that is going to be elicited,
  under penalty of spending the vault and of teaching the result of the domain to the panel. If there are no 20 resolved items outside the
  vault, that is declared on the spot, the pool is assembled with what there is (minimum of 20 all the same, even if contaminated), and the
  domain is marked as **contaminated** in the report.
- **Declared draw**: seed and list stay in the declaration with hash, before running. The expert sees the 20 statements
  and nothing more; the answer key stays with the facilitator.
- **No consultation.** It is a calibration pre-test: the expert answers from memory, in 20 minutes, without seeking a source.
- **Each item receives p ∈ (0, 1)** — exact 0 and 1 are rejected, because they are not a prediction, and without a prediction there is no Brier.
- **The unit of the item is that of the pool, not that of the domain**: the pre-test measures calibration, not knowledge of the domain. Its
  result does not enter the prior.

### 2.2 The two measures: Brier and curve

- **Brier**, item by item: `Brier = mean of (p − y)^2`, and the **skill** against the climatology of the pool itself,
  `skill = 1 − Brier / Brier_clima`, with `Brier_clima` from the base rate of the 20 answer keys — the same
  definition that H3 uses (`h3/estocologia/calibracao.py`, field `habilidade_vs_climatologia`).
- **Calibration curve** in **10 bins of equal mass** (the `ece` of H3, same file), with the predicted mean and the
  observed rate of each bin, the **ECE**, and the **Murphy decomposition** (`REL`, `RES`, `UNC`, function
  `murphy`, same file). The curve is what shows the shape of the error; the Brier alone hides whether the error is bias,
  overconfidence, or lack of resolution.
- **What is reported from the pre-test**: the 20 pairs `(p, y)`, the Brier, the `Brier_clima`, the skill, the ECE, the
  decomposition, and the 10 bins. Nothing aggregated without the detail.

### 2.3 Approval criterion (declared here, before running)

Whoever fulfils the four passes:

1. **skill > 0** — Brier below the climatology of the pool itself (predicting worse than the base rate is a rejection, even
   if by a little);
2. **ECE ≤ 0.10** in the 10 bins of equal mass;
3. **no bin with n ≥ 4 and |predicted mean − observed rate| > 0.25** (no systematic section);
4. **weakly monotone curve** — the predicted mean of the top bin above the predicted mean of the bottom bin.

The four thresholds are the default of the protocol: 0.10, 0.25, and weak monotonicity exist so as not to leave the pre-test
without a ruler, and they only change by a new declaration, with hash, counting in the K. The four go written, with the seed and the 20 items,
in the declaration frozen before running.

### 2.4 Whoever does not pass does not enter as a source

- **Rejected in the pre-test does not elicit.** The expert may remain at the table as a discussant, but his quartiles
  **do not** enter the consensus, and his form stays marked `pre_teste: reprovado`, with the numbers that
  rejected him.
- **A domain with fewer than 3 approved stays without a panel.** The domain is reported as **not elicited** — never completed
  with whoever was rejected, never substituted by the literature prior of p05 without saying so. With 2 approved, their consensus is
  reported with n = 2 and the qualification, and **does not** substitute the prior of the package in the official round (it stays as sensitivity).
- **A domain whose panel is rejected in a block** (for example, the three below the threshold) is a result: it says that that
  outcome has nobody who predicts it in a calibrated way, and that is what is reported.

## 3. Panel

- **Three experts per domain**, with **one from outside the area of the program**. "From outside" means two things at the same
  time, and both are written in the form:
  - **from outside the program**: he is not an author of the book or of the article, he is not a reviewer, he did not participate in the audit or in the
    fronts of `empirical/` — someone who has no interest in the result;
  - **from outside the area of the domain**: his training and his trade are not those of the elicited domain (for the domains of law,
    a jurist; for those of education, an educator etc.), so that the outsider serves as a control against the language and the
    conventions of the program itself.
  The outsider is the main control: he does not know what the program expects and does not recognize the "right" answer.
- **Facilitator**: conducts the pre-test, keeps the answer key, makes the return, and does not elicit. He has no vote in the consensus.
- **Declared exclusions** (who is out of the panel of that domain, with the reason written): whoever saw the output of the domain
  (`h2/saida/<dom>`, `h2g/saida/<dom>`, `h1-real/saida/<projeto>`); whoever wrote the package
  (`<h>/dominios/<dom>/pacote.yaml`) or its `preparo.py`; whoever already issued a prediction in the log of that
  domain; and whoever served the other side of a pair of the **same base** — `h2/educacao_star` and
  `h2g/educacao_star_turmas` (Krueger 1999, the same base AER_STAR) and `h2g/direito_teto_beneficio` and
  `h1-real/b` (the same scheme of base declared in p05, "same base") cannot have the same expert on both
  sides: double counting is already a declared problem of p05, and the panel does not repeat it.
- **Record**: each expert receives a fixed code (E1, E2, E3) that holds for all his domains; the name stays on the
  signature sheet of the declaration, and the code is what circulates in the forms and in the return (the consensus is anonymous
  among them).
- **A full panel is 18 people** (6 domains × 3), minus the exclusions for accumulation: one same expert can
  serve more than one domain as long as there is no declared conflict.

## 4. Elicitation: median, quartiles, and extremes, with return

The elicitation is **in the unit of the domain**, with the quantity written in the form of each domain (section 6). The order is
fixed, and each step is recorded before the next step:

1. **Median first.** `q50`: the value such that the expert finds it equally likely to be above and below.
2. **Then the quartiles.** `q25` and `q75`, defined as the bets in which the two sides are worth the same
   (equivalent in bets, not as a "confidence interval").
3. **Then the extremes.** `q05` and `q95` — the 5% of each tail, said explicitly as such.
4. **Short justification and source**, in one to three lines, written by the expert (it is what allows reading afterwards whether the
   consensus stayed informed or stayed flat).
5. **Return (first round).** The facilitator returns to the expert **his own numbers**, the **anonymous range of the
   group** (minimum, median, and maximum of the q50, without names) and, if there is one, the **literature prior of p05** as an
   external reference declared — never the output of the funnel nor the result of the domain.
6. **Confirmation.** The expert answers one of two things, in writing: he **confirms** his numbers, or he **adjusts
   to X**, with the reason. What enters the consensus is the confirmed number, and round 1 stays in the form.
7. **At most two rounds of return.** In the second, the numbers already confirmed do not come back: only what stayed in
   divergence (extremes too narrow, a median that changed order of magnitude, a control of section 5 blown).
   A divergence that is not resolved stays reported as a divergence, with both numbers.

### 4.1 Consensus

- **Declared rule: point-by-point median of the quartiles** of the experts who passed the pre-test, in the order
  `q05, q25, q50, q75, q95`, with the three individual values preserved alongside the consensus, always.
- The simulated H1 of the repository uses the **mean** of the quartiles ("as in RIO", `h1/estocologia/agentes.py`, function
  `f7_elicitar`). The exchange of the mean for the point-by-point median is declared **here, beforehand**, and is recorded as
  such: the median of a small sample is what resists an extreme expert, and the mean does not.
- **Adjustment of the prior:**
  - **gaussian** (`h2/desenvolvimento_preston`, `h2/educacao_star`, `h2g/educacao_star_turmas`,
    `h2g/direito_teto_beneficio`): Normal by least squares on the quantiles q25/q50/q75
    (`ajustar_normal`, `h2f/estocologia/dominio.py`);
  - **binomial** (`h2/direito_prisoes`): the quartiles are an odds ratio and become log before the adjustment
    (`priori_da_chave`, same file; `escala: razao_de_chances` and `taxa_base` 0.8 in the package);
  - **real H1** (`h1-real/c`): the quantity is a frequency, and the panel of H1 adjusts a **Beta** to the three quartiles
    (`ajustar_beta_quartis`, `h2f/estocologia/estat.py`), which is what the simulated H1 already did.
- **Always report**: the individual quartiles, the consensus, the adjustment (mean/median and SD, or a and b), the
  `erro_max_quartis`, and the ratio elicited 90% width / implied. An adjustment that does not close is a result, not a reason to
  rechoose the quartiles.

## 5. Controls: anchor, order, and overconfidence

**(a) Order.** Half of the panel receives the order median → quartiles → extremes and the other half the reverse order
(extremes → quartiles → median), with the draw declared beforehand. The median of the two subgroups is read: a difference greater than
**0.20 SD** (in units of the adjusted SD of the group) is a reported order effect; if it blows up, the consensus uses the **median
of the two orders** and the effect enters the report.

**(b) Anchor.** After the first pass, the facilitator presents **one** suggested value `S` per domain (drawn
among plausible values, with a declared seed, outside the range of the quartiles already issued) and asks for
`P(θ > S)`. Two readings:
- with `S` = median of the expert himself, the declared `P(θ > S)` should stay at **0.50 ± 0.10**; outside that the
  expert restates the median;
- with `S` = anonymous median of the group, the **displacement of the median** of the expert is recorded; above
  **0.20 SD**, the displacement is reported and the original number stays as the confirmed one.
- **Anchoring forbidden in the first pass**: no number of the domain enters the table before the expert writes the
  five quantiles — neither the prior of the package, nor the literature quartiles of p05, nor the output of the funnel, nor the posterior
  mean. The starting quartiles stay in `prioris_elicitadas.yaml` and only appear in the return (step 5 of
  section 4), declared as an external reference.

**(c) Overconfidence.** Ratio **elicited 90% width / 90% width implied** by the adjusted quartiles
(`razao_largura_90_elicitada_sobre_implicada`, `ajustar_normal`). Declared: ratio **≥ 0.75**. Below
that, the extremes are too narrow for the median and quartiles of the expert himself; he re-elicits **only
q05/q95** in the return, and the two sets stay in the form. The same control is read in the consensus, and the value stays in the
report — it is what says whether the elicited prior is more informative than the data and the literature sustain.

## 6. Domains to elicit and starting quartiles

The six domains below are the closed list of this front. "Starting quartiles" is the **literature prior of p05** — the
point of comparison, not the prior of the panel. It **does not** appear in the blank forms and **is not** shown in the
first pass (anchor control, section 5b).

| domain | horizon | area | family / scale | elicited quantity (unit) | file of the package |
|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | H2 | Development | gaussian | years of life expectancy per unit of natural log of GDP per capita | `h2/dominios/desenvolvimento_preston/pacote.yaml` |
| `h2/direito_prisoes` | H2 | Law | binomial, odds ratio (log in the adjustment; `taxa_base` 0.8) | odds ratio of release with notification, black against white detainees | `h2/dominios/direito_prisoes/pacote.yaml` |
| `h2/educacao_star` | H2 | Education | gaussian | effect of the small class on the sum of reading + mathematics (SAT points) | `h2/dominios/educacao_star/pacote.yaml` |
| `h2g/educacao_star_turmas` | H2g | Education | gaussian | effect of the small class on the standardized score (SD of the regular classes) | `h2g/dominios/educacao_star_turmas/pacote.yaml` |
| `h2g/direito_teto_beneficio` | H2g | Law (rule of labour indemnity) | gaussian | elasticity of the duration of the absence from work with respect to the weekly benefit (log-log) | `h2g/dominios/direito_teto_beneficio/pacote.yaml` |
| `h1-real/c` | real H1, project C | Insurance / tail | frequency (Beta), without a GLM family | monthly frequency of a month reaching the layer "100 xs 50" | `h1-real/projetos/c_real.py` |

**Starting quartiles (literature prior of p05), copied and checked:**

| domain | q05 | q25 | q50 | q75 | q95 | source in p05 |
|---|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | −2.746195205080443 | −0.21995975644268073 | 1.536 | 3.291959756442681 | 5.818195205080443 | `prioris/h2/literatura.yaml`, Pritchett & Summers (1996), R1 (e = 1.536; s = 1.302; SD = 2s) — independent |
| `h2/direito_prisoes` | 0.5944060189976956 | 0.6578947368421053 | 0.7059781184061313 | 0.7575757575757576 | 0.8384926931067868 | `prioris/h2/literatura.yaml`, Kochel, Wilson & Mastrofski (2011), R2 (range 1.32–1.52 in the OR, inverted) — independent |
| `h2/educacao_star` | 1.3021781198967624 | 4.860110016547187 | 7.333191149998485 | 9.806272283449783 | 13.364204180100208 | `prioris/h2/literatura.yaml`, Krueger (1999), R3 (point 7.333; SD = 3.667) — **same base** (AER_STAR) |
| `h2g/educacao_star_turmas` | 0.017804398806321772 | 0.06645122940943404 | 0.10026513098524002 | 0.134079032561046 | 0.1827258631641583 | `prioris/h2g/literatura.yaml`, Krueger (1999), R3 (point 0.1003; SD = 0.05013) — **same base** (AER_STAR) |
| `h2g/direito_teto_beneficio` | 0.22806681817823804 | 0.3 | 0.35 | 0.4 | 0.4719331818217619 | `prioris/h2g/literatura.yaml`, Meyer, Viscusi & Durbin (1995), R2 (range [0.3; 0.4]) — **same base** |
| `h1-real/c` | 0.05 (derived) | 0.25 | 0.5 | 0.75 | 0.95 (derived) | `prioris/h1-real/literatura.yaml`: **no checked source**, it uses the weak Beta(1, 1), which publishes only q25/q50/q75; q05 and q95 are **derived** by me from the Beta(1,1) itself (uniform) and are marked as derived |

Reading notes of the table:

- The rules R1, R2, R3 and the conversion of binomials to the log of the odds ratio are those fixed in the declaration of p05
  (`empirical/p05-prioris/DECLARACAO.md`); they explain the difference between these quartiles and those of the package, which are in
  another conversion (the package of `h2/direito_prisoes`, for example, carries 0.42–1.05 in the OR).
- `h2/educacao_star`, `h2g/educacao_star_turmas`, and `h2g/direito_teto_beneficio` are **same base** as the
  domain: what they measure is not independent evidence, and for that reason the panel cannot have the same person on both
  sides of the pair (section 3).
- `h2/direito_prisoes`: the quartiles enter as an odds ratio and the adjustment takes the log.

### 6.1 What p05 already observed with the literature prior

This is **not** a result of this point: it is what round (b) of p05 measured, and it is here because it is what the comparison of
`formularios/comparacao.md` is going to read. Whoever compares reads; whoever elicits does not see before answering.

| domain | status (a) package → (b) literature | action of F8 (a) → (b) | posterior mean (a) → (b) | displ. (b), in SD of (a) | current K (`resultados.json`) |
|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | approved → approved | adopt → **maintain** | 5.09 → 4.81 | 0.48 | K = 1 |
| `h2/direito_prisoes` | rejected (modelo_vence_climatologia) → **pruned at F1** (EVPI < cost) | — → — | −0.73 → — | — (pruned before F4) | K = 1 |
| `h2/educacao_star` | approved → **pruned at F2** (the unit does not inform more than the prior) | adopt → — | 13.5 → — | — | K = 1 |
| `h2g/educacao_star_turmas` | approved → **pruned at F1** (EVPI < cost) | maintain → — | 0.148 → — | — | K = 3 |
| `h2g/direito_teto_beneficio` | approved → approved | adopt → adopt | 0.579 → 0.506 | **1.4** | K = 3 |
| `h1-real/c` | rejected (modelo_vence_climatologia) → **approved** | exposure 0.4 → **0.0** | — | — (the `comparar.py` does not compute displacement in H1) | without field `K` in the `resultados.json` |

Source: `empirical/p05-prioris/tabela.md` and `empirical/p05-prioris/README.md` (section "The five changes of (b)"); the
column K comes from the field `K` of `h2/saida/<dom>/resultados.json`, `h2g/saida/<dom>/resultados.json`.
Two of those changes were of **decision**, not of number: `h2/desenvolvimento_preston` (adopt → maintain) and
`h1-real/c` (exposure 0.4 → 0.0). It is the size of what is at stake when the prior comes to come from people.

## 7. How to record

- **One blank form per domain**: `formularios/<horizonte>__<domínio>.yaml`, with three blocks of
  expert (E1, E2, E3), the pre-test block of each one, the elicitation, the controls, the return, and the consensus.
- **Nothing is rewritten**: an adjustment of quartiles after the return enters as a new line, dated and with an author, inside the
  block itself; what stayed confirmed in round 1 remains readable.
- **The consensus and the adjustment** go to `formularios/prioris_elicitadas.yaml` (today that file carries the example of the
  literature, marked as an example) and the comparison is filled in in `formularios/comparacao.md`.
- **Resources**: any script of this front runs with `OMP_NUM_THREADS=1` and at most 2 processes in parallel — the
  machine is shared. The part of people does not consume machine; that of comparison does.
- **Whoever prepares a form signs nothing in advance**: no field is filled in in advance, and no
  reading of the repository is presented as an answer of an expert.
- **Commit and push**: whoever prepares does not commit or push (rule of the task); the freezing with hash and the commit belong to
  whoever signs.
