# Declaration K = 3: saturation rule, the Sachs unit and state fixed effect in the seat belt

Written on 2026-10-14T22:32Z (UTC), **before** any run of the H2d code, in simulation or on real data.
The SHA-256 of this file and the UTC time of the record stay in `DECLARACAO_K3.sha256`. All the work stays in `h2d/`,
a copy of `h2c/`; `h2/`, `h2b/`, `h2c/`, `h3/` and `data/` are not touched.

## Situation and reason

- **The request.** sil said "Proceed with the pending items". For H2, that is to attempt, honestly, a third
  area without weakening the gate.
- **Where things stand.** At K = 2 (H2c), Ŝ ≠ ∅ in education (STAR, solid) and in health (RAND, only by saturation). The
  law lost the Ŝ. Sachs was pruned at F3 and the seat belt at F2, both before ICP.
- **What this K = 3 does.** Three changes, each declared here before running and each counted as K = 3 in the
  affected domain. The G3 criterion **does not change**: the sentence of `portoes.py` stays the same, letter for letter.
- **What is already known and can contaminate the choices.** I declare what I know of the previous results, because the
  choices below were made knowing it:
  - the K = 1 and K = 2 result of the three domains that reached F4 (H2b and H2c READMEs);
  - the H2b diagnostic ICP on Sachs (K = 1, after pruning): rejects everything, largest p = 1e-211;
  - the H2b diagnostic ICP on the seat belt (K = 1, after pruning): accepts only {idade21, idade_media, vel65, vel70},
    p = 0.054, without seat belt use;
  - the H2b F2 of the seat belt: SE per state 0.140 against prior sd 0.133; naive SE 0.057; design effect 6.1;
  - the H2b F3 of Sachs: z = 3.76 against null q95 5.20 (p = 0.10) with 11 conditions; design effect 67.

  None of the choices below was made looking at the K = 2 ICP in these two domains, which does not exist (ICP did not
  run there in H2c), nor by running any alternative.

## (a) Saturation rule (all domains)

- **Principle.** If the environment E is a deterministic function of the columns of S, then Y ⊥ E | X_S holds by
  construction: conditional on X_S, E is constant. An invariance test on such an S has zero power against any
  alternative, and a test with zero power cannot certify invariance. Accepting a saturated S is not evidence;
  rejecting it would also not be informative about invariance (only about the form of the adjustment).
- **The rule.** Saturated sets are **excluded from the family**: they count neither as accepted nor as rejected. Ŝ is the
  intersection of the accepted **non-saturated** sets. If no non-saturated set is accepted, ICP is treated
  as "nothing accepted" (`modelo_rejeitado = true`) and Ŝ = ∅. The rest of the ICP logic does not change: α = 0.05, all
  2^p subsets (p ≤ 10), the K = 2 residual test (KW + Brown–Forsythe, Bonferroni) with no change at all.
- **How "E is a deterministic function of X_S" is measured (replaces the K = 2 diagnostic).**
  1. The columns of S are those of the block that the funnel passes to ICP (standardized numerics, categoricals as indicators),
     each column rescaled to sd 1 on the training set; constant columns are dropped.
  2. Each row receives a nearest-neighbour hit score, leaving it out (1-NN LOO):
     - if there are other rows with exactly the same S values (a "cell" with m ≥ 2 rows), the score is the
       fraction of the other m − 1 rows of the cell that are in the same environment (ties resolved exactly);
     - if the row is unique in S, the score is 1 if the nearest row (Euclidean, `scipy.spatial.cKDTree`) is in the
       same environment, and 0 if not.
     - S = ∅: one cell with all the rows.
  3. **S is saturated if the mean hit rate ≥ 0.99** (the same threshold as the K = 2 diagnostic).
  - **Why this form.** It covers discrete X (the pure cell of K = 2: the RAND coparticipation, the STAR class × grade)
    and also continuous X that separates the environments into bands (a plan value, an incentive step). With
    continuous X that overlaps between environments, the nearest neighbour misses the environment often and S is not
    saturated.
  - **Declared limit.** With 1-NN, environments almost separable in X_S (hit ≥ 99% without being exactly
    deterministic) also count as saturated. It is deliberate: in that case the test also has almost null power.
- **Report.** By domain: number of saturated accepted, non-saturated accepted, saturated tested, and the Ŝ that the
  K = 2 logic would give (intersection of all accepted, saturated included), for comparison.
- **Unit test.** Before the real data: (i) discrete X that determines E is saturated; (ii) continuous X with
  bands disjoint by environment is saturated; (iii) overlapping continuous X and discrete X independent of E are not;
  (iv) in an SCM where the only accepted set containing the parent is saturated, Ŝ does not return it; (v) the
  H2c coverage tests keep passing.
- **Expected effect, said beforehand.** It may remove the Ŝ of RAND (all 512 accepted of K = 2 were saturated in the
  old diagnostic). It will be reported as it comes out.
- **K.** It counts as K = 3 in the 5 domains.

## (b) biologia_sachs: the cell becomes the unit

- **Change.** The `unidade` column becomes the cell (condition + row index), not the condition.
- **Justification.** The interventions are applied per condition and the cells are the replicates; this is how the
  classical ICP analyses of these data treat the cell (Meinshausen, Hauser, Mooij, Peters, Versteeg & Bühlmann
  2016, PNAS 113:7361, "Methods for causal inference from gene perturbation experiments and validation" — quoted from
  memory, verify; and Peters, Bühlmann & Meinshausen 2016, JRSS-B — quoted from memory, verify). In ICP the unit
  that matters is the row, and ICP already uses the cell; the change aligns F2/F3 with that.
- **What changes, said clearly.** F2 changes (SE clustered by cell, almost the robust SE) and F3 changes (the permutation
  null swaps pmek between cells, not between conditions), and therefore Sachs should reach F4. The likelihood
  weight in F4 (1/design effect) now uses the per-cell design effect, close to 1. **This is a
  change that loosens the dependence within the condition**; it was pointed out in the H2b README as a "choice after
  seeing the failure" and it is made here as such, declared.
- **Mandatory caveat.** The design effect at the condition level (67 in H2b) will be reported alongside: the cells
  of a condition share tube, staining and run, and the F3 margin and the posterior come out optimistic. G3 only looks
  at the ICP, whose guarantee (Ŝ ⊆ pa(Y) with prob. ≥ 1 − α) assumes independent rows within the environment; the
  dependence within the condition is dependence within the environment, because environment = condition. It stays declared as a limit.
- **Nothing else changes.** Y (log p44/42), the 10 candidates, `ajuste`, DAG, environments (12 conditions, without 6 and 13),
  vault (14_cd3cd28icam2_ly), prior, decision and seeds stay as in H2b.
- **K.** Sachs becomes K = 3 (K = 2 was the test change; K = 3 joins (a) and (b)).

## (c) transporte_cinto: state fixed effect ("within" transformation) in place of the 2-year baseline

- **The known diagnostic.** F2 pruned the seat belt because the SE clustered by state (0.140) passed the prior sd
  (0.133). The naive SE is 0.057: what inflates the SE is the design effect 6.1, that is, residuals highly correlated
  within the state.
- **The probable cause.** The Y of H2b is the log of mortality minus the mean of the state itself in 1983–1984. That
  baseline uses only 2 years, and in small states it has large Poisson noise; that noise enters as a constant
  per state in all the years, which is exactly a residual component common to the state. Besides that, the baseline
  only removes the level of Y, not that of the candidates, and the state-level confounding (opinion on safety,
  roads, fleet) stays in the X.
- **The declared correction (a single one).** State fixed effect through the within transformation, as in the panel design
  with state fixed effects of Cohen & Einav (2003), REStat 85:828 (quoted from memory, verify):
  - for each state, means computed **only on the training rows** (regimes `no` and `secondary`, with observed seat belt
    use) of log(deaths per mile), seat belt use, year, log income, mean age, vel65, vel70, idade21,
    alcool008 and log of the miles;
  - y = log(deaths per mile) − state training mean; each candidate = value − state training mean;
    `trafego_log` becomes log of the miles − state training mean;
  - the vault rows (`primary`) are centered with the training means of the state itself, never with vault rows; vault
    rows of states with no training row at all are dropped (the number will be reported).
- **Why this one and not another.**
  - **Unit.** Replacing the state with state-year as the unit would ignore the serial correlation; it would be loosening F2,
    not correcting it. Rejected.
  - **Vault.** Changing the vault does not attack the design effect. `no` in the vault would break the baseline (which uses years
    `no`) and `secondary` in the vault would leave 201 training rows. Rejected.
  - The within transformation attacks the diagnosed cause (component common to the state) and is the standard panel
    specification for this question.
- **What may happen.** The SE may rise or fall: removing the state component reduces the design effect, but
  using only the variation within the state reduces the variation of seat belt use. I did not run it to find out. If F2 still prunes,
  that is the result.
- **Nothing else changes.** Candidates (the same 9, now centered on the state), `ajuste`, DAG, environments (`no`,
  `secondary` in training; `primary` in the vault), unit (state), prior, decision and seeds. The question and the prior
  remain about the variation of log mortality per unit of use.
- **Centered binary values.** vel65, vel70, idade21 and alcool008 centered on the state stop being 0/1; the
  K = 2 expansion treats them by the number of distinct values, with no new rule.
- **K.** The seat belt becomes K = 3.

## (d) Additional dataset: none

- **Decision.** I do not add a sixth dataset. The reserves of `../data/h2b/FONTES.md`:
  - `AER_Fatalities` (Transport, same area as the seat belt; laws in package and confounded with the year: medium/high);
  - `AER_CigarettesSW` (n = 96; taxes come with anti-smoking campaigns: medium) and `Ecdat_Cigar` (no tax: high);
  - `poe5_njmin3` (it does not have the wage, the X intervened on);
  - `causaldata_thornton_hiv` (Health, an area already counted; the environment is the X itself, saturated by construction);
  - `causaldata_nsw_mixtape` (the treatment acts on Y by construction);
  - discretized Sachs (same source as Sachs).
- None has a clean intervention on X in a new area. Running one of them would be looking for an area, not testing invariance.

## What does not change

- The K = 2 test, its parameters, the F1–F4 filters, the G1–G5 criteria and the G3 sentence.
- The law, STAR and RAND packages change only in `tentativas_de_especificacao: 3` and in the history (rule (a)).
- `portoes.py` gains only a note in the header.

## After running

Nothing will be adjusted after seeing the K = 3 results. Any later change is K = 4 and will be reported as
a proposal, not executed.
