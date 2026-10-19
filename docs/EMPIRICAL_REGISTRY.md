# Consolidated Empirical Registry

The research numbers, together with the Rust instrument that reproduces them today. The fronts that
originally measured them in Python (`h0-lab0/`, `h0-real/`, `h1/`, `h1-real/`, `h2/` through `h2g/`,
`h3/`) left the working tree on 2026-10-16: the measurement scaffolding was removed while the numbers
remained here and in the textbook. The original code can be restored via
`git checkout 1bfec905 -- <folder>`, and the manifest `history/registro-python-removido.txt` records the
blob hash of every removed file.

Legend: **R** the Rust instrument re-measures it from scratch; **D** divergence declared in the chapter,
reporting both values; **P** registry only (the number is recorded here and in the chapter, without a
Rust instrument); **V** the verdict is re-derived in Rust from the versioned registry (the raw
measurement remains without an instrument — declaration 2 of `empirical/p14-h3/`).

## H0 — the queue and the call center

| Result | Tag | Source / instrument |
| --- | --- | --- |
| Clinic: the same mean arrival rate of 8 per hour gives a 9.1 min wait in the constant world and 19.3 min in the world oscillating between 5 and 11; 8 and 24 out of every 100 patients wait over 30 min (the retired generator gave 9.3 and 20.7, 8 and 25; Appendix E) | **R** | `labs --chapter 12` |
| Independent unit of the queue: daily margin of 0.51 min versus 0.07 per patient | **R** | `labs --chapter 15` |
| Bank call center: 2.65 million calls, 83 training days and 81 vault days, peak of 234 agents | **R** | `labs --bank`, reading `data/bank_calls.csv`. **Second opinion (point 19): confirmed** — independent recount yields 164 days, 2 654 950 calls in the vault versus 2 649 354, and between-day Var/mean of 261.7 (all) and 247.4 (vault) versus 261.1 and 246.7; the 0.2% difference comes from the operating hour window (07:00 to 20:00) used by the laboratory |
| Erlang C promises 82% on the average profile and delivers 57% on the real day, with 65% of days below target | **R** | `labs --bank`: 0.8193, 0.5683, and 0.6543 |
| Between-day dispersion: Var/mean of 273 and standard deviation around 2 900, versus 180 under Poisson | **D** | `labs --bank` measures 272.6 in training (246.7 in the vault) and 2 841 versus 181 |
| Decision table R0 to R3: 26% of days fail under the claimed staffing, 12% with the day-of-week profile, 3.7% with the p90 margin, for +7.2% agent-hours | **R** / **D** | `labs --queue`: agent-hours match (2 264, 2 261, 2 428, 2 433) and the ranking holds; measured failure rates are 30.9%, 19.8%, 4.9%, and 9.9%, with the divergence declared in Chapter 12 |

## H1 — stocks, jobs, and fire

| Result | Tag | Source / instrument |
| --- | --- | --- |
| Dow Jones: 5.1% / 7.2% / 6.0% per year at 1×, 2×, and 3×; volatility drag of 0.1403; −22.6% in 1987 wipes out 68% of 3× holders; median 2.10× versus mean 19.1× | **R** | `labs --chapter 16`, reading `data/djia_1885_2023.csv` |
| LaLonde: raw comparison of −8 498 dollars versus randomized trial of +1 794, selection bias of −10 292, and 1975 placebo at −12.3 percentage points | **R** | `labs --chapter 16`, reading `nsw_experimental.csv` and `cps_observational.csv` |
| 20-year cohorts: 1 134 cohorts worth roughly 5.8 independent blocks; 64.3% finish above their starting point | **P** | registry; intra-cohort correlation has no Rust instrument |
| Danish fires: 2 167 claims, excess layer hit in 4/1/0 months per three-year block, 152.4 M in August 1989, Hill 1.81 [1.41; 2.56], 2.2 bits, 1/38 | **R** / **D** | `labs --chapter 17`, reading `data/danish_fire.csv`; the Hill confidence interval shifts with resampling |
| Brier skill score of −0.015 for the agent's model and 4.6% annual growth in fire losses | **P** | registry; the out-of-time test belongs to the legacy pipeline |

## H2 — the seven rounds of causal invariance

| Result | Tag | Source / instrument |
| --- | --- | --- |
| Light tunnel: 128 accepted sets out of 1 024, with the three LEDs red, green, and blue | **P** | `labs --chapter 19` reproduces the method on simulated worlds, not the original dataset |
| Pharmacology (FDA/hERG): 35 of 512 sets, three ion channels, and verapamil predicted with skill +0.57 | **P** | registry |
| Yeast genetics: 40 genes, p = 0.001, replication of a validated gene | **P** | registry; the 40 real genes remain without an instrument, while the replication **design** is measured in the row below |
| Resolution of interventional validation: with one line per mutant, 4 of 40 pass the 0.001 floor (the true mover, the line artifact, and two noise draws that the p-value cannot separate); with two lines, 1 of 40 passes, the floor drops to 0.000001, and the estimate ceases to be a point | **R** | \`labs --genetics\` |
| Education (STAR): 11 598 students across 80 schools; by student the intersection survives, by school it collapses to the empty set | **P** | registry; this is the second test of the independent unit |
| $K = 38$ distinct tests (32 to 72), with Bonferroni retaining 4 domains before correction and 3 after | **P** | registry; the trial counter is a declared proposal for Rust |

## H3 — the continuous mesh

| Result | Tag | Source / instrument |
| --- | --- | --- |
| Six streams under a strict clock, zero leakage with a deliberately planted canary | **P** | raw stream data left with the `h3/` scaffolding; the **mechanism** is measured in `labs --chapter 20` (canary with the honest clock and with the clock disabled) and its class is declared in `empirical/p14-h3/registro.tsv` |
| Rhode Island vault: 24 466 stops, ECE 0.0048, skill +0.019 | **D** | `labs --chapter 20` measures the mechanism in simulation (ECE 0.0022), the chapter declares the difference, and `labs --h3` declares the class: the Rhode Island logbook left the working tree |
| Instrument-measured Goodhart: 0.145 [0.132; 0.157], four times smaller than the declared 0.6 | **P** / **V** | the measurement is registry-only (field result whose raw data is not versioned); the verdict against the declared criterion and the ratio of 4.1483 are **re-derived** from `empirical/p06-laco/resultados.json` by `labs --h3`, with zero divergence |
| Hypergraph: 454 178 nodes in H3, of which 453 973 are predictions and 205 are topology | **P** | the H3 graph left with the scaffolding; the Rust hypergraph is the validator, not the H3 graph. Class declared in `empirical/p14-h3/registro.tsv` |

## Reimplementation from the text

| Result | Tag | Source / instrument |
| --- | --- | --- |
| 462 quantities of 20 chapters recomputed in Python from the text and the data files, without the Rust of each chapter: 459 agree, none diverges and 3 are open (recipes longer than a paragraph, cited from the research record at commit `1bfec905`). Until the recipes of 2026-10-19 the count was 421, with 402 in agreement and 19 open. Among them, the bank's halves null per day at 0.0743 (the sd of the signed differences) and the drags predicted by $L^2\sigma^2/2$ in the Dow Jones training set, 0.0153, 0.0612 and 0.1376 per year | **R** | `python3 -I scripts/reimplementation/run.py`; Appendix E, rectifications of the reimplementation of the chapters and of the recipes |
| Garden of paths, 4,000 experiments with no effect: Simmons' four freedoms find p < 0.05 in 20.3% of them, the large family in 42.3%, the pre-registered test in 5.2%; Mayo severity of the garden 0.58. Rectified 2026-10-19 (from 21.5%, 47.0% and 0.53: the exclusion of the extreme case compared mixed groups) | **R** | `labs --chapter 7`; Chapter 7 and Appendix E, rectifications of the recipes |
| Parameters of the recipes written into the text on 2026-10-19: the class of Chapter 5 (weights 0.45 and 0.55, exact share below 6.0 of 45.2%); the rain of Chapter 8 ($z = -1.386 + 1.1\varepsilon$) and the vaults of its null ECE (Rhode Island 0.0226, the 2023 flights 0.130); the FNV-1a prime 1,099,511,628,211 of the System 1 triage (Chapter 16); the largest margin of the property test, 0.098 (Chapter 17); the elicited quartiles of the LaLonde prior, 0.433 and 0.533, and the 156 black men among the 185 treated (Chapter 13); the quartiles of the Danish fire prior, 0.0187 and 0.0283 per month, and the negligible world of one loss in 2,500 weeks, 0.0004 per week, whose clean-window ratio is $0.9996^{250}/0.996^{250}$ (Chapter 14) | **R** / **P** | the laboratories (`labs --chapter N`) and the research record at commit `1bfec905` (`fundo/p05-prioris/h1-real/projetos/{a,b,c}_real.py`); Appendix E, rectifications of the recipes |

## R-Axis — question resolution

| Result | Tag | Source / instrument |
| --- | --- | --- |
| Resolution loss, both arms staffed for a utilisation of 0.8: 25.54 min where demand oscillates from 5 to 11, 2.77 min where it oscillates from 7 to 9, and 0.00 min where it does not oscillate. Rectified 2026-10-19 (from 22.99 and 2.50: a fixed count of arrivals, the Lindley recursion off by one client, and 200 days). Rectified 2026-10-18: the earlier 46.24 and 5.97 min were measured with capacity equal to demand (utilisation 1), a critical queue without steady state | **R** | `labs --resolution`; Chapters 5, 12 and 21 |
| Prediction 3 (oscillation threshold): under the plan calibrated on flat demand, the fraction of failed days grows from 0.095 to 0.330 and crosses one-quarter at amplitude 2.5, with resolution loss turning to +2.87 min at the crossing and negative below it; under the Erlang plan, the prediction **does not hold** (flat demand already fails 1 day in 5, the fraction is non-monotonic, and refining at the crossing incurs a cost) | **R** / **D** | `labs --resolution`; background point 15, with K = 2 declared |
| Prediction 5 (transfer): 0 violations of the implication (no profile survives R1 and fails R0) and 3 counterexamples across 6 profiles × 200 days, with conservation of the aggregate verified. Read on 2026-10-18 as a consequence of arithmetic (a weighted mean of daily shares cannot cross a threshold that no day crosses), so the run tests the implementation, not the world | **R** | `labs --resolution`; background point 15 |

## Background Audit

| Result | Tag | Source |
| --- | --- | --- |
| Blind pilot of 12 cases: ceiling tie, with 8 of 8 defects caught in both arms and 0 false alarms | **P** / **V** | the blind judge's scores (`empirical/p01-tese/juiz/notas.yaml`) are **re-derived** by `labs --bridge`, returning the exact same 8 of 8, 0 of 4 false alarms, and 0 discordant pairs; the gate for point 16 locks the thesis as **unsupported** until a fair test is run, with the reason declared (human case selector, analysts, and judge are missing) |
| Multiplicity: $K = 38$; under Bonferroni, H2g retains 4 domains before correction and 3 after | **P** | registry |
| Priors: three rounds across 33 domains, maximum deviation of 0.0071 SD, 33 of 33 identical | **P** | `empirical/p05-prioris/`. **Second opinion (point 19): declared divergence** — the machine field `reproducao_rodada_a` covers all 33 domains and marks `true` in **8**; the published statement relies on the 0.05 SD band from `DECLARACAO_2.md` in p05, whereas the field does not declare its own tolerance. Both readings remain on record, without resolution by guesswork |
| Performative loop: 0.145 measured versus 0.6 declared | **P** | `empirical/p06-laco/` |
| Scaling, rare events, and healthcare: points 9 and 10 and extensions | **retired** | left the working tree on 2026-10-16 (`docs/declarations/REMOVED_NON_SIGNIFICANT.md`) |
| Human elicitation (SHELF), fifth domain: `h1-real/c` — pre-test with 16 of 20 and skill 0.4240 (the weakest of the five, yet positive), confirmed quintiles (Beta with a = 1.168188 and b = 9.911852, median 0.08) | **R** / **P** | pre-test and fitting in Rust — and the retired Python Beta fit (`a = 1.168188`, `b = 9.911852`) **agrees with the Rust Beta fit** (`a = 1.171038`, `b = 9.919921`, differing by 0.24% and 0.08%), providing a second opinion across two independent implementations; running the funnel with the elicited prior uses the retired p05 copy (**P**, seed 2226) and the project **passes** (`modelo_vence_climatologia` true), just as it passed under the weak Beta(1, 1) prior and contrary to the simulated panel in p05; posterior/prior SD ratio of **0.221084**, between 0.872232 for the simulated panel and 0.072434 for the weak prior. The literature prior for this domain has **no verified source** in p05 itself, so the elicitation fills an empty slot | `empirical/p12-shelf/` |
| Human elicitation (SHELF), fourth domain: `h2g/direito_teto_beneficio` — pre-test with the highest skill among the four (0.8585, ECE 0.1375), confirmed quintiles (mean 0.35 and SD 0.222390) | **R** / **P** | pre-test and fitting in Rust; running the funnel with the elicited prior uses the retired p05 copy (**P**, base `h2b`, seed 2127) and comes out **identical to the package run down to the last digit** (action `adotar`, P 1.0, posterior mean 0.578765, SD ratio 0.242860), because the three quartiles used by the fit are identical: formal elicitation **reproduced the package prior** instead of introducing a new one. The tails differ (q05 0.05 versus 0.00; q95 0.65 versus 0.75) and only show up in the control checks (90% width ratio: 0.8201 versus 1.0000). Incomplete panel: 1 of 3, and the expert is the program author | `empirical/p12-shelf/`, `governance/RELEITURA.md` |
| Human elicitation (SHELF), third domain: `h2/educacao_star` — pre-test approved under the active rule (skill 0.7425 and ECE 0.1525), confirmed quintiles (mean 12.0 and SD 8.8956, versus 12.0 and 5.9304 for the package and 7.3332 and 3.6666 for the **same-dataset** literature prior) | **R** / **P** | pre-test and fitting in Rust; running the funnel with the elicited prior uses the retired p05 copy (**P**): the domain is **approved at F4** and the action remains `adotar`, with P(adopting pays off) 0.836500 versus 0.839625 for the package, whereas the **same-dataset prior is pruned at F2** ("data at the independent unit do not inform beyond the prior") while the elicited prior passes — the circularity of using the dataset itself as a prior, caught by F2. EVPI: 2.640820 (elicited), 1.502145 (package), and 0.497782 (literature) | `empirical/p12-shelf/` |
| Human elicitation (SHELF), second domain: `h2/direito_prisoes` — pre-test approved **at both gates** (skill 0.973 and ECE 0.0450), confirmed quintiles (mean −0.4639 and SD 0.4715 on the log odds-ratio scale, versus −0.3546/0.2833 for the package and −0.3482/0.1046 for the literature) | **R** / **P** | pre-test, fitting, and control checks in Rust (`labs --preteste` and `labs --elicitacao --escala log`); running the funnel with the elicited prior uses the retired p05 copy (**P**): the domain is **rejected at F4** under all three priors — elicitation does not change the action — and what it moves is EVPI, from 0.541955 (package) and 0.009989 (literature) to 1.054148 (elicited), with the posterior mean shifting from −0.729791 to −0.752532 and the posterior/prior SD ratio from 0.275878 to 0.169906. Incomplete panel: 1 of 3, and the expert is the program author | `empirical/p12-shelf/`, `governance/RELEITURA.md` |
| Human elicitation (SHELF): one expert interviewed in domain `h2/desenvolvimento_preston`; pre-test approved at the adjusted gate (skill 0.65 and ECE 0.2250) and five quantiles confirmed (mean 3.25 and SD 2.2239, versus 6.5 and 2.2239 for the package and 1.536 and 2.6034 for the literature) | **R** / **P** | pre-test, fitting, and control checks in Rust (`labs --preteste` and `labs --elicitacao`); passing through the funnel with the elicited prior runs the retired p05 copy, tagged **P** — `P(adotar compensa)` 0.418875 and F8 action `manter`, versus 0.565375 (`adotar`) for the package and 0.366375 (`manter`) for the literature, both reproduced by the same command. Incomplete panel: 1 of 3, and the expert is the program author | `empirical/p12-shelf/`, `governance/RELEITURA.md` |

## H3 rebuilt — the real streams on public data (2026-10-19)

Six streams of H3 rebuilt in Rust by `labs --streams` (`crates/stochology-book/src/streams.rs`) from public data added
to `data/` on 2026-10-19 (`data/SOURCES.md`), under a specification written in Chapter 20 before the run. These numbers
sit beside the H3 numbers above, not in their place; where they differ, Appendix E keeps both.

| Stream | Number | Instrument |
| --- | --- | --- |
| Energy, Victoria 2014 | coverage of the 90% interval 0.886, 17 424 half hours | `labs --streams` |
| Bank, Sep–Oct 2003 | coverage 0.884, 6 422 slots | `labs --streams` |
| Market, Dow Jones 2000–2023 | violations of the 99% VaR 1.53%, Kupiec p = 0.00, Christoffersen p = 0.02 | `labs --streams` |
| Flights, Q4 2013 | raw CITL −0.0171, ECE 0.0260, skill +0.0898; corrected CITL −0.0015, ECE 0.0322 | `labs --streams` |
| Flights, Q4 2023 | raw CITL +0.0633, ECE 0.0646; corrected CITL +0.0025, ECE 0.0177, skill +0.0747 | `labs --streams` |
| Cyclones, 2015–2022 | CITL +0.0002, ECE 0.0121, skill +0.1000, 3 813 observations | `labs --streams` |
| Toronto, 2001–2002 | CITL +0.0165, ECE 0.0354, 1 488 cases; FPR of the marker 0.155 and 0.164 (Black) against 0.070 and 0.077 (White) | `labs --streams` |
| Canary | honest clock 0 emissions, gain +0.0000; clock switched off, gain +0.9102, +0.9718, +0.9188 | `labs --streams` |
| Null q95 of the ECE with the real forecasts (2,000 simulations, seed 2026) | flights 2013 raw 0.0046; flights 2023 corrected 0.0034; cyclones 0.0107; Toronto 0.0315 | `labs --streams` |

## H2 under the e-process (2026-10-19)

The 38 recorded tests of the H2 gate re-read with the stopping rule of point 22, under `empirical/p22-parada/DECLARACAO_2.md`
(sealed before the run). Instrument: `labs --eprocess`, reading `empirical/p22-parada/testes.tsv`.

| Number | Instrument |
| --- | --- |
| One ledger per area, mean of the mixture-calibrated e-values; a test with no non-empty set counts as p = 1 (e = 1/2) | `labs --eprocess` |
| Areas that cross 1/α = 20, as recorded: 3 (education 4414.6, pharmacology and physics at the clamp of p = 10⁻¹²); genetics 7.0, law 2.4, transport 3.0 | `labs --eprocess` |
| With the clustered rows admitted by the declaration: education 3678.9, still crossing; the same three areas | `labs --eprocess` |

## The wave engine against data (2026-10-19)

Point 23: two layers of the wave engine tested on catalogued data under `empirical/p23-onda/DECLARACAO.md`, sealed
and committed before the first run. Instrument: `labs --wave`. Verdicts: the arrow not supported; the conflict not
testable under its encoding.

| Number | Instrument |
| --- | --- |
| T1, Victoria 2012–2014, 155 weeks (six dates of daylight-saving change dropped by the sealed code): engine temperature → demand 53, demand → temperature 39, none 63; binomial p 0.087 | `labs --wave` |
| T1 null pairing (+13 weeks): engine 26 correct; rival (lagged cross-correlation) 92 correct, its null 85 | `labs --wave` |
| T1 rules: P1 false, P2 false (gap 0.174 < 0.20), P3 false (rival ahead by 0.252 > 0.10) | `labs --wave` |
| T2, New York 2013 vault: 77,184 flights, all in resonance; 2023 vault: 98,509 flights, 98,146 in resonance and 363 in beating; no conflict in either | `labs --wave` |
| T2, gain of System 2 over System 1 in Brier: +0.00048 (2013), +0.00038 (2023) | `labs --wave` |
| Synthetic controls: planted arrow 141 of 156 (rival 155); independent 80 against 64; planted interaction Δ +0.00083 [+0.00003; +0.00166]; none Δ −0.00052 [−0.00107; +0.00004] | `labs --wave` |

## Recipes of the course examples (2026-10-19)

Worked examples that the book cited without a recipe, recomputed from the recipe the text now states. The instrument
is `scripts/reimplementation/recipes.py`, run by `scripts/reimplementation/run.py` inside `scripts/verify.sh`. Every
account is exact or analytic. The examples that could not be reconstructed were cut from the text, with their earlier
values in Appendix E.

| Number | Chapter | Recipe | Value |
| --- | --- | --- | --- |
| Null paths of a 0.6 coin that reach 20 decibans in 500 tosses | 3 | exact, toss by toss | 0.87% (the text said 0.85% in simulation) |
| Forecast of whoever ignores the base rate, after a positive and a negative | 7 | mammography test, prior one half | 0.909 and 0.099 |
| Brier at prevalence 1%: ignoring, using the base rate | 7 | same test | 0.083 and 0.009 |
| Brier at prevalence 33%: insisting on 1%, ignoring | 7 | same test | 0.28 and 0.085 (the text said 0.084) |
| Full Kelly on a chance estimated from 25 rounds: share with negative growth | 10 | even odds, chance 0.6, exact binomial | 15% from 18 of 25 (the text said 16%) |
| Half Kelly against full Kelly | 10 | even odds, chance 0.6 | 75% of the growth |
| Three players who pool, Peters coin | 10 | mean log of the average of three factors | 1.6% per round |
| Horizon of the ensemble mean of N trajectories | 10 | ln N / (ln E[M] − E[ln M]) | 23 rounds for 10, 136 for a million (the text said 12 and 112) |

## Where the Sources Live

| Source | Location |
| --- | --- |
| Datasets used by the instruments | `data/` (`bank_calls.csv`, `djia_1885_2023.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `danish_fire.csv`), with `SOURCES.md` and `SHA256SUMS` |
| Published numbers, with the measuring instrument | this file and `docs/RUST_RESULTS.md` |
| Background audit, governance, and ethics | `empirical/` and `governance/` (retained) |
| Verified bibliography | `references/VERIFICATION.md` and Appendix D of the textbook |
| Anchoring of removed files | `history/registro-python-removido.txt`, `history/python-removido.txt`, `history/nao-significante-removido.txt`, in commit `1bfec905` |
