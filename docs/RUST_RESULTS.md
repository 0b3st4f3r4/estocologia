# Rust Results

Record harvested directly from the Rust instruments on 2026-10-17. Nothing here is manually transcribed: every block is the literal stdout of the command printed above it.

| Instrument | What it measures |
| --- | --- |
| `labs --chapter N` | Chapter N laboratory with fixed seed |
| `labs --all` | All 21 textbook laboratories in sequence |
| `labs --bank` | Real H0: 2.65 million bank calls, Erlang C schedule, independent unit, and split-half null |
| `labs --resolution` | Horizon R: cost of coarse questions under identical total resources |
| `labs --s1` | Deterministic System 1 triage |
| `labs --s2` | System 2: hypergraph search, board belief state, Metropolis inference, and R-axis |
| `labs --pretest` | Calibration pretest null for the SHELF panel |
| `labs --genetics` | Interventional validation resolution: what the second cell line changes |
| `labs --streams` | The real streams of Chapter 20, rebuilt on public data with a declared specification |
| `labs --eprocess` | The H2 gate under the e-process, over the 38 recorded tests, as declared before the run |
| `labs --mc-error` | The Monte Carlo error of every simulated number a reading of the book depends on |
| `labs --propagate` | Three answers to the same question: plug-in, delta method and Monte Carlo, with the uncertainty budget |
| `labs --listening` | Seal of the seven decisions affecting people: who was heard |
| `labs --h3` | H3 chain: what is re-measured, what is re-derived, and what remains a record |
| `labs --bridge` | Bridge thesis at scale: re-derived pilot and fair-test gate |
| `labs --schema` | Card contract: schema version and file-backed migration chain |
| `labs --second-opinion` | Second opinion on results marked P |
| `labs --jev` | Jev-LLM bridge: bridge gates and what requires frozen weights |
| `labs --queue` | Queue with abandonment and the four staffing rules (R0 to R3) |
| `gates` | Seven deterministic gates (G1-G7) and safeguard panel A1-A8 |
| `labs --rehearsal-shelf` | Synthetic floor of the SHELF panel: four declared respondents on real items |
| `labs --rehearsal-listening` | Coverage arithmetic and gate boundary for affected-group listening |
| `labs --rehearsal-bridge` | Roles, cases, and status in the scale bridge gate |
| `cargo test --workspace` | Test suite across all workspace crates |

## 1. Real H0 and Empirical Instruments in Rust

### Bank call center (Chapter 12)

Reads `data/bank_calls.csv` (5-minute public dataset), builds the Erlang C schedule over the mean training profile, and measures holdout performance.

~~~bash
./target/release/labs --bank
~~~

~~~text
== Real H0 :: the bank call centre, in Rust ==
training 83 days (Mar-Jun) | vault 81 days (Jul-Oct) | 2649354 calls in the vault
roster of the mean profile: 234 agents at the peak, 2259 agent-hours per day
service level on the mean profile = 0.8193 | mean of the vault days = 0.5683 | days below the goal of 80% = 0.6543
dispersion between days: Var/mean = 246.7 in the vault, 272.6 in the training window, 261.1 over all | deviation between days = 2841 | Poisson would predict 181
halves null: per day = 0.0743 | per call = 0.0006
~~~

### Queue with abandonment and staffing rules (R0 to R3)

Reads the bank call dataset, reconstructs the queue with abandonment via two independent engines, and measures the four staffing rules.

~~~bash
./target/release/labs --queue
~~~

~~~text
== Real H0 :: the queue with abandonment and the four scaling rules (R0 to R3) ==
training 83 days (Mar-Jun) | vault 81 days (Jul-Oct) | 2654950 calls in the vault
exponential service 240 s | exponential patience 180 s | target 20 s | goal 80% | seed 2003 | 3 replicas per day
residual CV of the day 0.0598 | margin of R2 = 1 + 1.2816 x CV = 1.0767 | R3 = R0 + 12 agents across the board
days in the training window per weekday [16, 17, 17, 17, 16, 0, 0]

Exact engine (observed arrivals, FCFS with abandonment) — the reconstruction
rule    agent-hours/day   peak     service level    days below the goal         abandon/100     mean wait s median wait s  p90 wait s
R0                 2264    234             83.9%                  30.9%                 8.4              14            0           0
R1                 2261    267             84.4%                  19.8%                 8.1              14            0          44
R2                 2428    287             91.6%                   4.9%                 4.2               7            0           0
R3                 2433    246             90.3%                   9.9%                 5.0               8            0           0

Events engine (M/M/c+M event by event, Poisson arrivals) — independent verification
rule    agent-hours/day   peak     service level    days below the goal         abandon/100     mean wait s median wait s  p90 wait s
R0                 2264    234             84.4%                  28.4%                 4.3               7            0           6
R1                 2261    267             87.5%                  16.0%                 3.7               6            0          14
R2                 2428    287             95.0%                   4.9%                 1.6               3            0           1
R3                 2433    246             92.4%                   8.6%                 2.2               4            0           0
~~~

### System 1: deterministic card triage

Closed vocabulary of 6 categories and 35 themes, suffix tree of 136 entries, probabilities clamped to [0.01, 0.99], and normalized sum checked on every pass.

~~~bash
./target/release/labs --s1
~~~

~~~text
== System 1 :: deterministic triage of cards ==
suffix tree: 129 suffixes indexed over 6 declared themes
category=queue probability=0.9117 products=[0.9117, 0.0177, 0.0177, 0.0177, 0.0177, 0.0177] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9117 | sum 1.0000 | outside [0.01;0.99] 0
category=causal probability=0.9297 products=[0.0141, 0.9297, 0.0141, 0.0141, 0.0141, 0.0141] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9297 | sum 1.0000 | outside [0.01;0.99] 0
category=tail probability=0.9117 products=[0.0177, 0.0177, 0.9117, 0.0177, 0.0177, 0.0177] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9117 | sum 1.0000 | outside [0.01;0.99] 0
category=calibration probability=0.9415 products=[0.0117, 0.0117, 0.0117, 0.9415, 0.0117, 0.0117] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9415 | sum 1.0000 | outside [0.01;0.99] 0
category=fairness probability=0.9217 products=[0.0157, 0.0157, 0.0157, 0.0157, 0.9217, 0.0157] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.9217 | sum 1.0000 | outside [0.01;0.99] 0
category=other probability=0.8718 products=[0.0256, 0.0256, 0.0256, 0.0256, 0.0256, 0.8718] sum=1.0000 out_of[0.01;0.99]=0 | probability 0.8718 | sum 1.0000 | outside [0.01;0.99] 0
~~~

### SHELF panel calibration pretest

Declared null: a perfectly calibrated expert with uniform prior under the protocol design.

~~~bash
./target/release/labs --pretest
~~~

~~~text
== SHELF pre-test :: panel calibration ==
protocol design: 20 items, 10 buckets of equal mass; ECE threshold 0.1
measured limits of the design: criterion 3 (bucket with n >= 4) becomes vacuous, because 20 items in 10 buckets give 2 per bucket; criterion 4, as the protocol writes it, is inert: the buckets are cut by ordered p and the mean forecast of the upper one exceeds that of the lower one by construction
calibrated null, 20 items in 5 buckets, ECE threshold 0.25: ECE p50 0.1584 | p90 0.2316 | p95 0.2559 | maximum 0.4487 | skill p50 0.3076 | p10 0.0370 | Brier p50 0.1645
    verdict of the gate in the calibrated world: approved 0.2935 | rejected by criterion 3 0.7052 | by corrected criterion 3 (binomial) 0.0031 | by criterion 2 0.0592 | by some criterion 0.7065
calibrated null, 20 items in 10 buckets, ECE threshold 0.1: ECE p50 0.2272 | p90 0.3002 | p95 0.3218 | maximum 0.5474 | skill p50 0.3076 | p10 0.0370 | Brier p50 0.1645
    verdict of the gate in the calibrated world: approved 0.0008 | rejected by criterion 3 0.0000 | by corrected criterion 3 (binomial) 0.0007 | by criterion 2 0.9992 | by some criterion 0.9992
calibrated null, 200 items in 10 buckets, ECE threshold 0.1: ECE p50 0.0694 | p90 0.0932 | p95 0.1002 | maximum 0.1470 | skill p50 0.3316 | p10 0.2583 | Brier p50 0.1663
    verdict of the gate in the calibrated world: approved 0.8729 | rejected by criterion 3 0.0995 | by corrected criterion 3 (binomial) 0.0179 | by criterion 2 0.0514 | by some criterion 0.1271
~~~

### Deliberative System 2

LATS versus greedy search on the sample hypergraph, board belief state, Beta(4, 8) posterior versus closed form (1/3), and R-axis resolution selection.

~~~bash
./target/release/labs --s2
~~~

~~~text
== System 2 :: deliberation over the hypergraph ==
declared graph: 8 nodes, 6 tree edges, 2 graph edges, 5 hyperedges | validation ok
nodes that no lens covers: ["f1.code-synthesis"]
greedy: stops at h0.queue (value 0.600) | LATS: root.stochology -> r.resolution -> f1.code-synthesis (value 1.000, node without lens true)
board belief: most likely band A5-governance with probability 0.9205 | entropy 0.4324
stress: deliberated stopped at A5-governance with 1 attack(s) | declared order stopped at A5-governance with 5 attacks | same rejection true
inference: Beta(4, 8) chain mean 0.3346 against the closed form 0.3333 | acceptance 0.676
R axis: loss of the aggregate 25.54 min against a margin of 12.00 min -> R1-dia (the loss of the aggregate pays for the margin of the day) | out of measured reach: R2-hora, R3-subpergunta
R axis without oscillation: loss 0.00 min -> R0-agregada
~~~

### Interventional validation resolution (yeast genetics)

Replicate design: 40 mutants, comparing single-line versus two-line validation.

~~~bash
./target/release/labs --genetics
~~~

~~~text
== Chapter 19 :: the fine resolution of validation by intervention ==
design: 40 mutants | 60 individuals per line | 999 draws per test | alpha = 0.05 | Bonferroni alpha/40 = 0.001250
planted by the instrument: true mover = mutant 7 | line artifact = mutant 23

-- today's validation: every mutant is a single-line environment --
  40 mutants x 60 individuals | design floor = 0.001000 | threshold = 0.001250
  mutant  7 | effect  +0.940 | p  0.001000 | passes at the floor <- true mover (effect on both lines)
  mutant 14 | effect  -0.450 | p  0.001000 | passes at the floor
  mutant 23 | effect  +0.813 | p  0.001000 | passes at the floor <- line artifact (effect on a single line)
  mutant 29 | effect  +0.496 | p  0.001000 | passes at the floor
  passing: 4 of 40

-- the same world, with the second line: the effect must be on both --
  40 mutants x 2 lines x 60 individuals | design floor = 0.000001 | threshold = 0.001250
  mutant  7 | effect l1  +0.456 | effect l2  +1.016 | span  0.560 | p  0.000001 | passes <- true mover (effect on both lines)
  mutant 23 | effect l1  +0.607 | effect l2  +0.028 | span  0.579 | p  0.000831 | rejected <- line artifact (effect on a single line)
  passing: 1 of 40
  effect on the second line: artifact  +0.028 (does not reproduce) | mover  +1.016 (does reproduce) | in single-line validation the estimate is a point and the span is zero

reading: with one line the floor of 0.001000 and the threshold of 0.001250 let 4 of the 40 mutants through — the true mover, the line artifact and two mutants that are just noise. All of them sit on the design floor, and the p-value distinguishes none of the three from what we are looking for. With the second line the artifact leaves the list, because its effect on the second line is zero, and the floor drops to 0.000001000: 1 of the 40 mutants pass. The p was never the problem: the problem was there being a single line.
~~~

### The real streams of Chapter 20, rebuilt

Energy, bank, market, flights of 2013 and 2023, cyclones and Toronto 1997-2002, from public data with a specification declared before the run; the canary under both clocks; the fairness audit by colour.

~~~bash
./target/release/labs --streams
~~~

~~~text
== Chapter 20 :: the real streams, rebuilt beside the book ==
no step draws a random number | gates: |CITL| <= 0.02, ECE <= 0.03, skill > 0 | coverage 0.86 to 0.94 | VaR 0.5% to 1.5% and Kupiec p > 0.05
energy (Victoria 2014, day ahead, 90% interval):  n =  17424 | coverage 0.886 | gate PASSES
bank (Sep-Oct 2003, next working day, 90%):       n =   6422 | coverage 0.884 | gate PASSES
market (Dow Jones 2000-2023, 99% VaR, 250 days):  n =   6011 | violations 1.53% | Kupiec p = 0.00 | Christoffersen p = 0.02 | gate FAILS
flights 2013 (Oct-Dec), trained on 54835 flights of the 60 days before:
   raw model:             n =  82609 | rate 0.2214 | CITL -0.0171 | ECE 0.0260 (null q95 0.0046) | skill +0.0898 | gate PASSES
   intercept correction:  n =  82609 | rate 0.2214 | CITL -0.0015 | ECE 0.0322 (null q95 0.0046) | skill +0.0908 | gate FAILS
   canary: honest clock available in 0 emissions, skill gain +0.0000 | clock switched off, skill gain +0.9102
flights 2023 (Oct-Dec), trained on 68257 flights of the 60 days before:
   raw model:             n = 104091 | rate 0.1273 | CITL +0.0633 | ECE 0.0646 (null q95 0.0040) | skill +0.0282 | gate FAILS
   intercept correction:  n = 104091 | rate 0.1273 | CITL +0.0025 | ECE 0.0177 (null q95 0.0034) | skill +0.0747 | gate PASSES
   canary: honest clock available in 0 emissions, skill gain +0.0000 | clock switched off, skill gain +0.9718
cyclones (2015-2022, +30 kt in 24 h), trained on 12585 observations of 1975-2014:
   model:                 n =   3813 | rate 0.0462 | CITL +0.0002 | ECE 0.0121 (null q95 0.0107) | skill +0.1000 | gate PASSES
Toronto (2001-2002, chance of release, model blind to colour), trained on 3738 cases of 1997-2000:
   model:                 n =   1488 | rate 0.8253 | CITL +0.0165 | ECE 0.0354 (null q95 0.0315) | skill +0.0812 | gate FAILS
   canary: clock switched off, skill gain +0.9188
   marker (chance of release below 0.70): false positive and false negative rates by perceived colour
   2001 Black  FPR 0.155  FNR 0.667  (311 cases)
   2001 White  FPR 0.070  FNR 0.772  (900 cases)
   2002 Black  FPR 0.164  FNR 0.500  (85 cases)
   2002 White  FPR 0.077  FNR 0.838  (192 cases)
~~~

### The H2 gate under the e-process, retroactively

One ledger per area over the 38 recorded tests, the mean of the e-values, two readings as declared in empirical/p22-parada/DECLARACAO_2.md.

~~~bash
./target/release/labs --eprocess
~~~

~~~text
== The H2 gate under the e-process, retroactively ==
38 distinct tests as recorded, 2 clustered rows admitted by the declaration | alpha = 0.05 | threshold 1/alpha = 20 | merge: mean within each area | gate: 3 areas

-- reading: as recorded --
  area             tests claims         mean e  verdict
  Agronomia            1      0         0.5000  does not
  Biologia             2      0         0.5000  does not
  Clima                1      0         0.5000  does not
  Crédito              2      0         0.5000  does not
  Desenvolvimento      1      0         0.5000  does not
  Direito              5      3         2.3749  does not
  Educação             5      2      4414.5557  crosses 1/alpha
  Energia              1      0         0.5000  does not
  Engenharia           1      0         0.5000  does not
  Epidemiologia        2      0         0.5000  does not
  Farmacologia         1      1 1309803451.4320  crosses 1/alpha
  Física               1      1 1309803451.4320  crosses 1/alpha
  Genética             5      1         6.9919  does not
  Logística            1      0         0.5000  does not
  Saúde                5      0         0.5000  does not
  Trabalho             2      0         0.5000  does not
  Transporte           2      1         2.9927  does not
  areas that cross: 3 (Educação, Farmacologia, Física) | gate under the e-process: PASSES

-- reading: with dependent rows corrected --
  area             tests claims         mean e  verdict
  Agronomia            1      0         0.5000  does not
  Biologia             2      0         0.5000  does not
  Clima                1      0         0.5000  does not
  Crédito              2      0         0.5000  does not
  Desenvolvimento      1      0         0.5000  does not
  Direito              5      3         2.3749  does not
  Educação             6      3      3678.8880  crosses 1/alpha
  Energia              1      0         0.5000  does not
  Engenharia           1      0         0.5000  does not
  Epidemiologia        2      0         0.5000  does not
  Farmacologia         1      1 1309803451.4320  crosses 1/alpha
  Física               1      1 1309803451.4320  crosses 1/alpha
  Genética             5      1         6.9919  does not
  Logística            1      0         0.5000  does not
  Saúde                5      0         0.5000  does not
  Trabalho             2      0         0.5000  does not
  Transporte           2      1         2.9927  does not
  areas that cross: 3 (Educação, Farmacologia, Física) | gate under the e-process: PASSES
~~~

### The Monte Carlo error of the simulated numbers

Each simulated quantile a reading depends on, rerun under twenty other seeds; each simulated share with its exact Clopper-Pearson interval (Appendix C.3).

~~~bash
./target/release/labs --mc-error
~~~

~~~text
== The Monte Carlo error of the simulated numbers ==
each simulated quantile rerun under 20 seeds other than the published one; each simulated share with its exact 95% interval

-- the null q95 of the ECE in the rebuilt binary vaults (Chapters 8 and 20; 2000 simulations each) --
  flights 2013, raw        ECE 0.0260 | published q95 0.0046, order-statistic 95% interval [0.0045; 0.0047] | 20 seeds: mean 0.0046, sd 0.00005, range [0.0045; 0.0047] | ECE above q95 in 20 of 20
  flights 2023, corrected  ECE 0.0177 | published q95 0.0034, order-statistic 95% interval [0.0034; 0.0035] | 20 seeds: mean 0.0035, sd 0.00003, range [0.0034; 0.0035] | ECE above q95 in 20 of 20
  cyclones 2015-2022       ECE 0.0121 | published q95 0.0107, order-statistic 95% interval [0.0105; 0.0110] | 20 seeds: mean 0.0109, sd 0.00010, range [0.0107; 0.0111] | ECE above q95 in 20 of 20
  Toronto 2001-2002        ECE 0.0354 | published q95 0.0315, order-statistic 95% interval [0.0307; 0.0322] | 20 seeds: mean 0.0318, sd 0.00029, range [0.0313; 0.0323] | ECE above q95 in 20 of 20

-- the simulated null at Toronto's size in the laboratory of Chapter 8 (2,000 simulations each) --
  published q95 0.0425 | 20 seeds: mean 0.0416, sd 0.00040, range [0.0409; 0.0424] | Toronto's real ECE 0.0354 below the simulated q95 in 20 of 20

-- the bootstrap interval of the tail index in Chapter 14 (300 replicas each) --
  published [1.37; 2.61] | 20 seeds: lower mean 1.395 sd 0.030 range [1.34; 1.46], upper mean 2.535 sd 0.040 range [2.43; 2.60] | the interval contains 2 in 20 of 20

-- shares counted over independent draws, with the exact 95% interval --
  Chapter  7: garden of paths, Simmons' four freedoms: false positives: 0.2030 of 4000 (about 812) | Monte Carlo sd 0.0064 | exact 95% interval [0.1906; 0.2158]
  Chapter  7: garden of paths, the large family: false positives: 0.4230 of 4000 (about 1692) | Monte Carlo sd 0.0078 | exact 95% interval [0.4076; 0.4385]
  Chapter  7: pre-registered test: false positives: 0.0520 of 4000 (about 208) | Monte Carlo sd 0.0035 | exact 95% interval [0.0453; 0.0593]
  Chapter  8: null ECE above 0.03 at Toronto's size: 0.4510 of 2000 (about 902) | Monte Carlo sd 0.0111 | exact 95% interval [0.4290; 0.4731]
  Chapter 10: investors with no loss in 250 weeks (all-in under the raw frequency): 0.3680 of 10000 (about 3680) | Monte Carlo sd 0.0048 | exact 95% interval [0.3585; 0.3775]
  Chapter 10: investors ruined in 1,000 weeks under the raw frequency: 0.0194 of 10000 (about 194) | Monte Carlo sd 0.0014 | exact 95% interval [0.0168; 0.0223]
  Chapter 13: replicas of the claim world as bad as the real index (p): 0.0140 of 2000 (about 28) | Monte Carlo sd 0.0026 | exact 95% interval [0.0093; 0.0202]
~~~

### Three answers to the same question

The wait of a day at the health post with uncertain, correlated rates: plug-in, delta method with its second-order mean, Monte Carlo, the uncertainty budget, and the rule on when the linear answer may decide (Chapter 5).

~~~bash
./target/release/labs --propagate
~~~

~~~text
== Three answers to the same question: the wait of a day at the health post ==
W = 60 lambda / (mu (mu - lambda)) min | lambda ~ N(8, 0.6^2) per hour | mu ~ N(12, 0.8^2) per hour | question: P(W > 20 min), promise <= 0.05 | 400000 draws, seed 2026
sensitivities at the mean: dW/dlambda = +3.750 min per arrival/h, dW/dmu = -3.333 min per service/h

-- correlation between arrivals and pace: -0.5 --
  plug-in:     W = 10.00 min | no probability | decision: promise kept
  delta:       mean 10.00 (second order 11.37) | sd 4.26 | P(W > 20) = 0.0095 | decision: promise kept
  Monte Carlo: mean 12.82 | median 9.99 | sd 165.93 | q95 24.47 | P(W > 20) = 0.0884 (MC sd 0.0004) | days that never clear 199 | decision: second nurse
  budget of the delta variance 18.17 min^2: arrivals 5.06 (28%), pace 7.11 (39%), covariance +6.00 (+33%)
  the linear answer may decide: false (delta and Monte Carlo differ by 0.0789, more than half the promise, 0.025)

-- correlation between arrivals and pace: +0.0 --
  plug-in:     W = 10.00 min | no probability | decision: promise kept
  delta:       mean 10.00 (second order 10.92) | sd 3.49 | P(W > 20) = 0.0021 | decision: promise kept
  Monte Carlo: mean 11.22 | median 9.99 | sd 9.08 | q95 19.99 | P(W > 20) = 0.0499 (MC sd 0.0003) | days that never clear 14 | decision: promise kept
  budget of the delta variance 12.17 min^2: arrivals 5.06 (42%), pace 7.11 (58%), covariance -0.00 (-0%)
  the linear answer may decide: false (delta and Monte Carlo differ by 0.0478, more than half the promise, 0.025)

-- correlation between arrivals and pace: +0.5 --
  plug-in:     W = 10.00 min | no probability | decision: promise kept
  delta:       mean 10.00 (second order 10.47) | sd 2.48 | P(W > 20) = 0.0000 | decision: promise kept
  Monte Carlo: mean 10.52 | median 9.99 | sd 2.98 | q95 15.90 | P(W > 20) = 0.0107 (MC sd 0.0002) | days that never clear 0 | decision: promise kept
  budget of the delta variance 6.17 min^2: arrivals 5.06 (82%), pace 7.11 (115%), covariance -6.00 (-97%)
  the linear answer may decide: true (delta and Monte Carlo differ by 0.0107, within half the promise, 0.025)
~~~

### H3 chain re-executed from the repository tree

Reads `empirical/p14-h3/registro.tsv`, classifies each of the four H3 rows, re-derives the performative loop from `empirical/p06-laco/resultados.json`, and runs negative proofs.

~~~bash
./target/release/labs --h3
~~~

~~~text
== Point 14 :: the H3 chain, re-executed as far as the tree allows ==
record stamp: 2026-10-16
line             class     mark versioned record                     what the instrument does
h3.fluxos        record    P    -                                    declares: the raw data is not in the tree
h3.ri            record    D    -                                    declares: the raw data is not in the tree
h3.laco          derived   V    empirical/p06-laco/resultados.json   re-derives the account and the verdict
h3.hipergrafo    record    P    -                                    declares: the raw data is not in the tree

-- the performative loop, re-derived from the record in the tree --
2SLS estimate: r = 0.1446378859 | 90% CI from 0.1319811884 to 0.1568385251 | first-stage F = 5523.46
ratio declared r / measured r: 4.1482907207 (the record brings 4.1482907207) | declared band: 0.3333 to 3.0000
sign matches: true | order of magnitude matches: false | compatible in the strict sense: false (the record says false)
bias of the naive forecaster against exogenous demand: 0.2912
divergences between the account and the record: none
verdict: the H3 loop overestimates the measured one: the sign matches and the order does not (ratio 4.1483, band from 0.3333 to 3.0000)

-- negative proof: a record in which the ratio falls inside the declared band --
  with measured r 0.4: order matches = true | verdict: the H3 loop has the sign and the order of the measured one (ratio 1.5000 inside the band)
-- negative proof 2: a record in which the ratio field does not match the account --
  divergences: ["the record's ratio (2.000000) does not match the account (4.148291)", "the record's containment (true) does not match the account (false)"]
~~~

### Jev-LLM bridge

Weight gate, schema-constrained decoding with declared abstention, G1 determinism proof, and hypothesis-space adequacy.

~~~bash
./target/release/labs --jev
~~~

~~~text
== Point 20 :: the Jev-LLM bridge ==

-- the gate of the weights --
  REFUSED: HashNotDeclared("empirical/p20-jev/pesos/SHA256") | the operator declares the sha256 in empirical/p20-jev/pesos/SHA256 before running, and no number comes out

-- the restricted decoding --
  the rehearsal model answers in the schema: card jev.bridge.1 | layer L2 | words of the document: 93
  free text refused: OutOfSchema("LineWithoutSeparator(\"this is free text, not a card\")") | abstention Others
  margin at 100% refused: DogmaticExtreme("margin 1") | abstention Others

-- the determinism proof, in the form of G1 --
  rehearsal model: run 1 5b39f6cee7710ff1 | run 2 5b39f6cee7710ff1 | identical: true
  unstable model: run 1 e20d9cbb82f57cff | run 2 7395445653a74393 | identical: false
  reading: gate G1 closes with a frozen model and does NOT close with a model that changes its answer — which is what docs/JEV_INTEGRATION.md declares about the free LLM

-- the cost per call and the adequacy of the space --
  rehearsal model: 8 calls | 42.0 tokens per call | 0.0 ms per call
  unstable model: 4 requests | 4 abstentions (100.0%) | the card schema does NOT serve this model: 100.0% of abstentions, above the declared limit of 30%
  rehearsal model: 4 requests | 0 abstentions (0.0%) | the card schema serves this model: 0.0% of abstentions, inside the declared limit of 30%

-- the local runner of the frozen weights --
  REFUSED: hash not declared in empirical/p20-jev/pesos/SHA256 | the operator declares the sha256 before running

-- the calibration: the Brier of the frozen model --
  without a labeled set: hash not declared in empirical/p20-jev/calibracao/SHA256 | the Brier demands pairs with provenance declared before running
  declared rule: 0.0 and 1.0 refuse, an id out of order refuses, an outcome outside 0 and 1 refuses, below 30 pairs the score is not published, abstention leaves the denominator
  rehearsal of the runner (these are not the real weights): d5ecae88ce7e5ecc | 4 answers | G1 identical: true | abstentions 0.0% | 88.0 tokens per call | 0.0 ms per call
  reading: the path artifact -> declared hash -> frozen table -> gate runs end to end; what is missing is the model file

-- the declared state --
  local runner: exists in crates/stochology-s2/src/runner.rs, with artifact, hash declared before the run and abstention per request with no answer
  frozen weights: absent from the tree | the operator declares the sha256 in empirical/p20-jev/pesos/SHA256 and leaves the model in empirical/p20-jev/pesos/modelo.congelado; until then, the gate refuses and no real calibration number is published
  labeled set: without a published score | the reader exists and the set is what is missing
  what already closes: the gate of the weights, the restricted decoding with abstention, the determinism proof, the cost per call, the adequacy rule of the space, the runner and the reader of the labeled set
~~~

### Second opinion on results marked P

Reads point 19 registry and verifies independent re-implementations and derivations.

~~~bash
./target/release/labs --second-opinion
~~~

~~~text
== Point 19 :: the second opinion on the results marked P ==
registry stamp: 2026-10-16
item                       class       second opinion
h1.coortes                 registro    declara: o dado saiu com o andaime; nao ha segunda implementacao possivel
h1.brier                   registro    declara: o teste fora do tempo e do roteiro antigo
h2.luz                     registro    declara: o metodo e reproduzido em mundos simulados no capitulo 19
h2.fda                     registro    declara: o dado do ensaio nao esta na arvore
h2.levedura                derivada    o desenho da replica e medido de novo no ponto 14; os 40 genes reais seguem sem instrumento
h2.star                    registro    declara: e o segundo teste da unidade independente, e o dado saiu
h2.k                       conferida   os limiares alfa/M gravados sao recalculados aqui como 0,05/M, na forma exata do arquivo
h3.fluxos                  registro    declara: o mecanismo do canario e medido no capitulo 20
h3.goodhart                conferida   o vies do previsor ingenuo aparece em duas fontes independentes e as duas concordam digito por digito
h3.hipergrafo              registro    declara: o grafo saiu com o andaime; o hipergrafo do Rust e o validador
aud.piloto                 derivada    o ponto 16 re-deriva o empate no teto das notas do juiz
aud.prioris                conferida   a reproducao e lida do registro e contada de novo; o desvio maximo nao esta em maquina e fica como registro
aud.pareceres              conferida   as linhas da releitura e os arquivos reemitidos sao contados
h0.real                    conferida   a base e recontada em Rust por um caminho proprio, com a dispersao por uma passada so

-- what the second opinion measured --
  multiplicity: M = 32 | alpha/M expected 0.0015625 | recorded in the registry: matches
  multiplicity: M = 38 | alpha/M expected 0.0013157894736842105 | recorded in the registry: matches
  multiplicity: M = 72 | alpha/M expected 0.0006944444444444445 | recorded in the registry: matches
  priors: reproduction of p05 read from the registry: 8 identical of 33 domains
  performative loop: bias of the naive forecaster in two independent sources: 0.29124674633711956 (p03) and 0.29124674633711956 (p06) | they agree digit by digit
  real H0: recounted in Rust: 164 days | vault 2654950 calls (published: 2 649 354) | Var/mean between days 261.7 over all and 247.4 in the vault (published: 261.1 and 246.7)
  opinions: 30 lines in the rereading | 4 opinions reissued in the folder

lines checked by a second implementation: 5 | lines that remain registry only: 7
~~~

### Card contract: file-backed migration chain

Reads `crates/stochology-core/migracoes.tsv` and demonstrates migrating a v1.0 card to the current schema.

~~~bash
./target/release/labs --schema
~~~

~~~text
== The contract :: the card on file, the version and the migration chain ==
card version: 1.3 | chain declared in crates/stochology-core/migracoes.tsv
  step 1: from 1.0 to 1.1: adds resolucao = R0-agregada | a resolucao da pergunta entrou no contrato com o Phi5; a migracao nao inventa pergunta mais fina que a declarada no documento
  step 2: from 1.1 to 1.2: adds priori_elicitada = false | o campo nasceu com a frente SHELF; um documento anterior nao tem priori elicitada de painel
  step 3: from 1.2 to 1.3: introduces orcamento_de_incerteza (optional, absent until declared); introduces evento_dominante (optional, absent until declared) | de onde vem a variancia da saida, por entrada e covariancia, fracoes que fecham em um; um documento anterior nao declarou orcamento e a migracao nao inventa um / o evento que carrega o custo esperado, com probabilidade e fracao do custo; frequencia e severidade sao eixos distintos, e a migracao nao inventa o evento
  versions of the chain: 1.0 -> 1.1 -> 1.2 -> 1.3

-- demonstration: a version 1.0 document migrates up to the card version --
  the 1.0 document does not declare the two fields the chain adds
  migrated: resolution = R0-agregada (the declared addition value) | elicited_prior = false | definition intact = true
  the migrated card validated: id lab.schema.1 | layer L2 | action "size the roster of the day"
  the card at the current version does not go through the chain: true

-- the two fields of contract 1.3: the uncertainty budget and the dominant event --
  declared and read back: budget "arrivals=0.42; pace=0.58; covariance=0.00" | dominant event "the ship is lost; p=0.05; share=1.0"
  a budget that does not close is refused: invalid value in orcamento_de_incerteza: arrivals=0.42 (the shares add up to 0.420, not to one within 0.05)
~~~

### Bridge thesis at scale

Re-derives pilot P01 from blind judge scores and checks the scale bridge gate.

~~~bash
./target/release/labs --bridge
~~~

~~~text
== Point 16 :: the bridge thesis at scale ==
-- pilot P01, re-derived from the blind judge notes --
cases 12 | defective 8 | clean 4 | discordant pairs 0
flagged the defect: arm X 8 of 8 | arm Y 8 of 8
false alarm on the clean ones: arm X 0 of 4 | arm Y 0 of 4
verdict: tie at the ceiling: both arms flagged 8 of 8 defects and raised 0 false alarms on 4 clean cases; by the declared rule, the thesis is NOT supported

-- the fair test --
window deadline: 2027-01-14 | minimum cases: 12
if the window lapses without the roles: nao sustentada em definitivo, aceito pelo dono em 2026-10-16
  escolhedor_dos_casos   nao designado de fora do programa, escolhe os casos antes de existir resultado
  analistas_braco_A      nao designado dois humanos, um de fora do programa
  analistas_braco_B      nao designado dois humanos, um de fora, cegos de qualquer texto da Estocologia
  juiz                   nao designado um humano de fora do programa, cego ao braco, com o gabarito
  casos                  0 de 12    casos mais dificeis que a lista de 2019, escolhidos pelo de fora
  status_da_tese         nao sustentada nao sustentada ate o teste justo (o empate no teto conta como nao sustentada)
designated: [] | pending: ["escolhedor_dos_casos", "analistas_braco_A", "analistas_braco_B", "juiz", "casos (0 de 12)"] | cases missing: 12
divergences: none — no line claims what the fair test does not yet support
verdict: THESIS NOT SUPPORTED and box closed — with 5 pending, the thesis does not become "supported" by any path other than the fair test, and only by 2027-01-14

-- negative proof: a registry that says the thesis is supported without the fair test --
  divergences: ["declared status \"sustentada\" with the fair test still to be done: the thesis only changes state after the judge's result"]
  box closed: true
-- positive proof: designated roles and cases at the minimum --
  divergences: [] | box closed: false

-- thesis (c′): the advantage at the seams (DECLARACAO_3.md) --
per stratum: 12 cases (8 defective, 4 clean) | machine runs per arm: 5 (reliability, descriptive)
seam kinds: resolucao_da_pergunta, dinamica_nao_ergodica, laco_performativo, selecao_dentro_da_decisao, cauda_sem_media, quem_paga_o_erro
  stratum S  0 of 12 cases  not scored
  stratum C  0 of 12 cases  not scored
divergences: none
verdict: (c′) NOT SUPPORTED and box closed — stratum S at 0 of 12 cases; stratum C at 0 of 12 cases; the fair test of thesis (c) has 5 role(s) pending
-- negative proof: an advantage of A equal in both strata is (c), not (c′) --
  computed Some("nao sustentada") | divergences ["declared status \"sustentada\" but the scores give \"nao sustentada\""]
-- positive proof: advantage at the seams, parity in the classical stratum --
  computed Some("sustentada") | divergences []
~~~

### Seal of the seven decisions affecting people

Reads `empirical/p13-escuta/registro.tsv` and checks affected-group listening coverage.

~~~bash
./target/release/labs --listening
~~~

~~~text
== Point 13 :: listening to those who pay for the error ==
decisions: 7 | registry stamp: 2026-10-16 | conduct: protocolo cego | window deadline: 2026-12-15 | minimum coverage: 2 of 7
decision                               holders heard        record     may issue
h2g/saida/direito_teto_beneficio             - nenhum       -          nao
h2g/saida/educacao_star_turmas           11598 nenhum       -          nao
h2g/saida/saude_rand                         - nenhum       -          nao
h1-real/saida/b                          16177 nenhum       -          nao
h3/prisoes                                5226 nenhum       -          nao
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

listening coverage: 0 of 7 groups heard
divergences: none — no decision claims to be issued with nobody heard
verdict: BOX SEALED — the coverage of 0 stays below the minimum of 2; the decisions remain as sensitivity analysis (n = 1) and do not become public-policy guidance nor an F8 gate decision until 2026-12-15 or until the minimum coverage

without listening, one by one:
  h2g/saida/direito_teto_beneficio   | trabalhadores acidentados cobertos pelo teto (Kentucky 1980, Michigan 1982)
  h2g/saida/educacao_star_turmas     | criancas do jardim a 3a serie do STAR no Tennessee
  h2g/saida/saude_rand               | familias nao idosas do RAND HIE (anos 1970)
  h1-real/saida/b                    | participantes do NSW e controles do CPS
  h3/prisoes                         | detidos de Toronto 1997-2002
  h3/prisoes20                       | prisoes de Toronto 2020-2021
  h3/prisoes_ri                      | paradas de Rhode Island 2005-2015

-- negative proof: the decision that claims to be issued with nobody heard --
decisions: 3 | registry stamp: 2026-10-16 | conduct: protocolo cego | window deadline: 2026-12-15 | minimum coverage: 2 of 3
decision                               holders heard        record     may issue
h3/prisoes                                5226 nenhum       -          sim
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

listening coverage: 0 of 3 groups heard
divergences: REJECTED on ["h3/prisoes"]
verdict: BOX SEALED — the coverage of 0 stays below the minimum of 2; the decisions remain as sensitivity analysis (n = 1) and do not become public-policy guidance nor an F8 gate decision until 2026-12-15 or until the minimum coverage

-- positive proof: two groups heard, with a signed record --
decisions: 2 | registry stamp: 2026-10-16 | conduct: protocolo cego | window deadline: 2026-12-15 | minimum coverage: 2 of 2
decision                               holders heard        record     may issue
h3/prisoes                                5226 associacao de defesa publica de TORONTO registro-01.md sim
h3/prisoes20                             65276 coletivo de ex-detentos registro-02.md sim

listening coverage: 2 of 2 groups heard
divergences: none — no decision claims to be issued with nobody heard
verdict: BOX OPEN for the covered decisions — the coverage of 2 reached the minimum of 2
~~~

### Horizon R: question resolution

Paired arms under identical total resources measuring resolution loss.

~~~bash
./target/release/labs --resolution
~~~

~~~text
== R horizon :: resolution of the question ==
paired arms: same arrivals and same service bases; only the service rate changes
demand 5 or 11 (the station of Chapter 12): mean capacity 10.0 | aggregate wait 43.73 min, fine wait 18.20 min, loss 25.54 min
   above 30 min: aggregated 0.4488, fine 0.2081
demand 7 or 9 (light oscillation): mean capacity 10.0 | aggregate wait 21.15 min, fine wait 18.38 min, loss 2.77 min
   above 30 min: aggregated 0.2518, fine 0.2198
demand 8 or 8 (no oscillation): mean capacity 10.0 | aggregate wait 18.52 min, fine wait 18.52 min, loss 0.00 min
   above 30 min: aggregated 0.2209, fine 0.2209
resolution ladder: R0-agregada -> R1-dia -> R2-hora -> R3-subpergunta

-- prediction 3: the oscillation threshold (failed day: more than 10% above 30 min; threshold: more than 1/4 of the days) --
  200 days per amplitude | 10 h per day | specification 1: the Erlang plan of the average day, 12 agents
   amplitude  failed days       fraction   loss (min) above30 R0
         0.0           41          0.205        -0.26     0.0612
         0.5           52          0.260        -1.89     0.0776
         1.0           51          0.255         1.97     0.0954
         1.5           64          0.320         2.60     0.1267
         2.0           66          0.330         2.27     0.1365
         2.5           73          0.365         7.04     0.1634
         3.0           80          0.400         8.93     0.2314
  threshold: amplitude 0.5 | resolution loss at the threshold -1.89 min | monotone: false

-- prediction 3, specification 2: the R0 scale calibrated on flat demand --
  the calibrated scale has 13 agents (the flat one fails on at most 10% of the days)
   amplitude  failed days       fraction   loss (min) above30 R0
         0.0           19          0.095        -2.24     0.0339
         0.5           30          0.150        -4.08     0.0467
         1.0           33          0.165        -0.66     0.0554
         1.5           39          0.195        -0.65     0.0739
         2.0           42          0.210        -1.12     0.0854
         2.5           53          0.265         2.87     0.1027
         3.0           66          0.330         3.36     0.1458
  threshold: amplitude 2.5 | resolution loss at the threshold 2.87 min | monotone: true

-- prediction 5: the transfer (surviving R1 implies surviving R0; the converse does not) --
  6 profiles x 200 days | day threshold 0.10 | violations of the implication: 0 | counterexamples: 3 | aggregate conservation checked: true
  reading: no profile survives R1 and fails at R0; 3 profiles pass at R0 and fail on some day

-- the R axis, chapter by chapter (the table is in book/r-axis.tsv; the reading is computed here) --
chapter  1 | no loss measured in this chapter | the chapter presents the target and the thesis; there is no scale decision
chapter  2 | no loss measured in this chapter | the three layers; the grain of the question does not enter the account here
chapter  3 | no loss measured in this chapter | probability without a scale decision
chapter  4 | no loss measured in this chapter | the three kinds of uncertainty; the R axis is orthogonal to them
chapter  5 | resolution loss 25.54 min (demand from 5 to 11) | the mean lies: the chapter is the post that oscillates between 5 and 11
chapter  6 | no loss measured in this chapter | the independent unit is Phi2; the R axis is its cousin in the grain of the question
chapter  7 | no loss measured in this chapter | severe tests and multiplicity; no scale decision
chapter  8 | resolution loss 2.77 min (demand from 7 to 9) | the calibration tolerance depends on the n of the vault: the oscillation from 7 to 9 is the light case
chapter  9 | no loss measured in this chapter | the ladder of cause; the grain enters as the unit of the environments
chapter 10 | resolution loss 0.00 min (flat demand) | drag and ruin are measured per day; without oscillation, refining buys nothing
chapter 11 | no loss measured in this chapter | the trilemma is the subquestion of R3: every hour, and for whom
chapter 12 | resolution loss 25.54 min (demand from 5 to 11) | the chapter brings the R0 to R3 table of the post and of the bank
chapter 13 | no loss measured in this chapter | the independent unit is the person; the R axis appears as the grain of the comparison
chapter 14 | resolution loss 0.00 min (flat demand) | the fires: the number changes with the window, and the window is the grain
chapter 15 | no loss measured in this chapter | the map of knowledge; no scale decision
chapter 16 | no loss measured in this chapter | the adversarial panel judges the card, not its grain
chapter 17 | no loss measured in this chapter | the contract carries the resolution field (R0 to R3) and Phi5; the lock only bites when the unit is a day or coarser
chapter 18 | no loss measured in this chapter | every vault is a new grain: the ruler of the R axis when the correction changes the question
chapter 19 | no loss measured in this chapter | grouping by school brings the area down; by student it does not: the independent unit in the vocabulary of the axis
chapter 20 | resolution loss 25.54 min (demand from 5 to 11) | the live vault: laboratory 0.0022 and real vault 0.0048 are two grains of test
chapter 21 | oscillation threshold at amplitude 2.5 (calibrated plan) | the chapter names the axis and predictions 3 and 5
~~~

## 2. The 21 Textbook Laboratories

### All laboratories

Single run, fixed seed, offline.

~~~bash
./target/release/labs --all
~~~

~~~text
== Chapter 1 :: the invisible target ==
two machines with the seed 1713 agree in 10000 of 10000 draws: true
draws coinciding with the next seed: 0
seed 1713: frequency of heads in 100 thousand tosses = 0.5000
seed 1909: frequency of heads in 100 thousand tosses = 0.4992
seed 2026: frequency of heads in 100 thousand tosses = 0.4987
== Chapter 2 :: the fallacy lives on the bridge ==
prevalence 1e-2: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.0001  swap factor=99.0
prevalence 1e-3: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.0010  swap factor=998.0
prevalence 1e-6: P(evidence|innocent)=0.00000100  P(innocent|evidence)=0.5000  swap factor=499999.7
== Chapter 3 :: trees, odds and decibans ==
mammography: 100 sick, 90 sick positives, 9900 healthy, 891 false positives, 981 positives
P(sick|positive) = 0.0917  (one in every 10.9 positives)
second test: independent = 10.0 dB, correlated (effective ratio 4.2) = 6.23 dB
posterior with two positives: independent = 0.5025, correlated = 0.2979
== Chapter 4 :: the coin whose bias changes ==
bias climbs from 0.2 to 0.6 in 6400 steps, 200 repetitions, discount 0.99
coverage of the 95% interval: fixed model = 0.1199, model with forgetting = 0.9398
width of the fixed interval at step 100 = 0.1564, at step 6400 = 0.0240
width of the interval with forgetting at step 6400 = 0.1360 (ratio 5.7x the final fixed one)
== Chapter 5 :: when the mean lies ==
W(5) = 3.57 min, W(8) = 10.00 min, W(11) = 55.00 min
Jensen: f(E[X]) = 10.00 min | E[f(X)] = 29.29 (5 or 11) and 14.46 (uniform 5..11)
second order: half f''(8) Var(X) = 8.44 min (5 or 11) and 2.81 min (uniform)
                 8 every day:  319405 patients | mean   9.1 min | median   3.7 | p90  26.2 | >30 min:   7.6 per 100
                     5 or 11:  319115 patients | mean  19.6 min | median   8.6 | p90  56.1 | >30 min:  23.7 per 100
               uniform 5..11:  319843 patients | mean  13.8 min | median   5.5 | p90  39.8 | >30 min:  15.0 per 100
5 or 11, surge reinforcement:  319115 patients | mean   2.5 min | median   0.0 | p90   8.1 | >30 min:   0.4 per 100
on surge days: 27.0 min in 219458 patients (68.8% of the total); on mild days: 3.4 min
halves null in the 8 every day world: measured deviation 0.201 min | margin per day 0.185 | per patient 0.046 | wait variance 168 min^2
halves null in the     5 or 11 world: measured deviation 0.742 min | margin per day 0.563 | per patient 0.097 | wait variance 747 min^2
difference between the worlds = 10.6 min = 52.7 deviations of the null
median of the 5 or 11 world = 8.6 min, against the mean wait of 9.1 min of the constant world
class of 100000 students: mean = 6.21, failing (score < 6.0) = 45.1 per 100
== Chapter 6 :: the invisible filter ==
Literary Digest: 2400031 of 10000000 voters answered (f = 0.240); Roosevelt among them = 0.4259, Landon = 0.5741
measured error = -0.1821 (-18.2 points); Meng identity with rho = -0.21: -0.1824
the error is worth the standard error of a random sample of 7.2 people; the poll's margin was 0.0276 points, and the error was 18.2 points = 661 margins
deviation of 2000 random samples of 7 people = 0.1836 (18.4 points); of 50 000 interviews = 0.0022 (0.22 points)
Berkson with cutoff 1.0: correlation in the population = +0.0012, among the selected (24.0% of the population) = -0.6168
Berkson with cutoff 1.5: correlation in the population = +0.0012, among the selected (14.4% of the population) = -0.6790
Berkson with cutoff 2.0: correlation in the population = +0.0012, among the selected (7.9% of the population) = -0.7315
Simpson: small 0.931 x 0.867; large 0.730 x 0.688; raw 0.780 x 0.826 (reverses)
weighted by a half-and-half population: 0.831 x 0.777 (the open surgery wins again)
two-state chain with persistence 0.500: variance of the mean of 100 letters = 0.002487 (independent: 0.002500), effective size = 101
two-state chain with persistence 0.800: variance of the mean of 100 letters = 0.009899 (independent: 0.002500), effective size = 25
two-state chain with persistence 0.235: variance of the mean of 100 letters = 0.000765 (independent: 0.002500), effective size = 327
== Chapter 7 :: hunting ghosts in the noise ==
model garden: 200 replicas, 500 candidates over 50 pure-noise predictors, 25 training cases
the validation winner beats predicting zero in 79.5% of the replicas; on fresh data, in 30.0%
mean squared error of the winner: 0.814 on validation (predicting zero: 1.000) and 1.191 on fresh data
hot hand: mean of the proportions per sequence = 0.4167 (5/12 = 0.4167); pooled proportion = 0.5000
100 people tossing 20 coins: in 100.0% of the experiments someone strings 6 heads in a row; the longest run averages 9.5
garden of paths (4000 experiments with no effect at all): with Simmons' four freedoms, someone finds p < 0.05 in 20.3% of them; with the large family, in 42.3%; the pre-registered test flags 5.2%
Mayo severity: 1 - p = 0.96 in the pre-registered test with p = 0.04; 1 - false-positive rate = 0.58 in the garden
Ioannidis with R = 1/1000, power 0.80: PPV = 0.016 with alpha 0.05 and 0.002 with the garden alpha
Ioannidis with R = 1/10, power 0.80: PPV = 0.615 with alpha 0.05 and 0.159 with the garden alpha
Ioannidis with R = 1/3, power 0.80: PPV = 0.842 with alpha 0.05 and 0.387 with the garden alpha
safeguard panel A1-A8, card with the eight safeguards: bits [1, 1, 1, 1, 1, 1, 1, 1], approved = true
the same card without one band's safeguard: 7 of 8 bits set, approved = false
== Chapter 8 :: the forecaster's thermometer ==
4000 days, observed base rate = 0.243, mean chance declared by the expert = 0.252
Platt fitted on days 1-2000 and scored out of the fit on days 2001-4000 (intercept a, slope b):
      lazy (0.20 always): Brier 0.1858 | REL 0.0018 RES 0.0000 UNC 0.1840 | ECE 0.0430 | out of fit: Brier 0.1831 -> 0.1817 with Platt, ECE 0.0385 -> 0.0090 (a = -1.11, b = 0.00)
                  expert: Brier 0.1569 | REL 0.0012 RES 0.0267 UNC 0.1840 | ECE 0.0281 | out of fit: Brier 0.1558 -> 0.1547 with Platt, ECE 0.0345 -> 0.0204 (a = -0.18, b = 0.82)
           overconfident: Brier 0.1712 | REL 0.0150 RES 0.0275 UNC 0.1840 | ECE 0.1109 | out of fit: Brier 0.1708 -> 0.1555 with Platt, ECE 0.1119 -> 0.0129 (a = -0.21, b = 0.37)
                   noisy: Brier 0.2119 | REL 0.0363 RES 0.0083 UNC 0.1840 | ECE 0.1449 | out of fit: Brier 0.2147 -> 0.1741 with Platt, ECE 0.1544 -> 0.0200 (a = -0.78, b = 0.28)
Murphy identity on the expert: REL - RES + UNC = 0.1584339, Brier of the forecasts pooled into the boxes = 0.1584339, difference = 3.47e-15
the raw forecast has Brier 0.1569462; pooling into 10 boxes costs 1.49e-3, and the identity closes
 Toronto prisons 1997-2002: n =    1488 | null q50 0.0292 | null q95 0.0425 | null q99 0.0481 | mean 0.0296 | null ECE above 0.03 in 45.1% of the simulations
    Rhode Island 2013-2015: n =   24466 | null q50 0.0028 | null q95 0.0043 | null q99 0.0051 | mean 0.0029 | null ECE above 0.03 in 0.0% of the simulations
           NY flights 2023: n =  104489 | null q50 0.0025 | null q95 0.0037 | null q99 0.0042 | mean 0.0026 | null ECE above 0.03 in 0.0% of the simulations
n = 1 488, spread 0.4: null q50 0.0320, null q95 0.0456
n = 1 488, spread 1.0: null q50 0.0291, null q95 0.0419
n = 1 488, spread 2.0: null q50 0.0243, null q95 0.0349
== Chapter 9 :: Simpson's paradox in kidney-stone surgery ==
small: open 81/87 = 0.9310  new 234/270 = 0.8667
large: open 192/263 = 0.7300  new 55/80 = 0.6875
raw:   open 273/350 = 0.7800  new 289/350 = 0.8257
weight of the strata in the whole population: small 0.5100, large 0.4900
weighted by the whole population: open 0.8325  new 0.7789  -> the advantage flips back
difference in points: small stones 6.44, large stones 4.25, standardised 5.37 (a weighted mean of the two)
fraction of hard cases in the open surgery = 0.7514, in the new one = 0.2286

== Chapter 9 :: the three steps of Pearl ==
  [  1. association] neighbourhoods with more police have more crime
  [ 2. intervention] if we double policing, does crime fall?
  [3. counterfactual] would this robbery have happened with a patrol car on the corner?

== Chapter 9 :: a minimal ICP, three sites, three candidates ==
rows = 12000 (4000 per site), X -> Y with coefficient 2, X -> Z (child) with noise different per site, W pure noise
naive correlation with Y: corr(X,Y) = 0.9314  corr(Z,Y) = 0.7208  corr(W,Y) = 0.0055
naive slope (all sites together): X = 1.9885  Z = 0.6003  W = 0.0152
  candidate {empty}  F = 3081.848  p = 0.000e0  intercepts only  rejected
  candidate {    x}  F =    0.633  p = 6.390e-1  common = 1.988  per site = [1.970, 1.996, 1.990]  ACCEPTED
  candidate {    z}  F = 1079.841  p = 0.000e0  common = 0.600  per site = [0.928, 0.496, 0.299]  rejected
  candidate {    w}  F = 1541.630  p = 0.000e0  common = 0.015  per site = [0.056, -0.030, 0.008]  rejected
  candidate {  x+z}  F =    0.579  p = 7.471e-1  common = 1.994  per site = [1.918, 1.998, 1.999]  ACCEPTED
  candidate {  x+w}  F =    0.640  p = 6.984e-1  common = 1.988  per site = [1.970, 1.996, 1.990]  ACCEPTED
  candidate {  z+w}  F =  719.620  p = 0.000e0  common = 0.600  per site = [0.928, 0.496, 0.299]  rejected
  candidate {x+z+w}  F =    0.597  p = 7.815e-1  common = 1.994  per site = [1.919, 1.998, 1.999]  ACCEPTED
accepted sets: 4 of 8 -> intersection S_hat = {x}

== Chapter 9 :: 200 repetitions, stable mechanism: Y = 2X + noise at every site
mean naive slope (all sites together): X = 2.0004  Z = 0.6015
  {empty}  mean F =  3191.16  rejection rate = 1.000
  {    x}  mean F =     1.06  rejection rate = 0.065
  {    z}  mean F =  1111.23  rejection rate = 1.000
  {    w}  mean F =  1596.08  rejection rate = 1.000
  {  x+z}  mean F =     1.02  rejection rate = 0.065
  {  x+w}  mean F =     1.05  rejection rate = 0.080
  {  z+w}  mean F =   741.13  rejection rate = 1.000
  {x+z+w}  mean F =     1.03  rejection rate = 0.055
  intersection = {x} in 0.960 of the rounds | contains z in 0.015 | contains w in 0.010 | empty in 0.025

== Chapter 9 :: 200 repetitions, unstable mechanism: the coefficient of X in Y changes from site to site
mean naive slope (all sites together): X = 2.0001  Z = 0.6013
  {empty}  mean F =  2921.60  rejection rate = 1.000
  {    x}  mean F =  6001.61  rejection rate = 1.000
  {    z}  mean F =   772.13  rejection rate = 1.000
  {    w}  mean F =  1461.37  rejection rate = 1.000
  {  x+z}  mean F =  4001.00  rejection rate = 1.000
  {  x+w}  mean F =  4001.05  rejection rate = 1.000
  {  z+w}  mean F =   515.10  rejection rate = 1.000
  {x+z+w}  mean F =  3000.74  rejection rate = 1.000
  intersection = {x} in 0.000 of the rounds | contains z in 0.000 | contains w in 0.000 | empty in 1.000

== Chapter 10 :: the Peters coin (heads +50%, tails -40%) ==
ensemble mean per round = 1.0500  (+5.00% per round)
time mean per round      = -0.052680  (-5.268% per round)
product of one head and one tail: 1.5 x 0.6 = 0.90
full Kelly (heads +50%, tails -40%): f* = p/0.4 - (1-p)/0.5 = 0.2500 of the wealth; even-odds bet with p = 0.6: 2p - 1 = 0.20
time growth per round: f* = +0.006211 | 2f* = +0.000000 | f = 0.70 -0.014200 | f = 1 -0.052680

all-in (f = 1.00)          above the start: 10 rounds 0.374  100 rounds 0.128  1000 rounds 0.00000
                           ruin (< 1e-6 of the capital) = 0.9971  final median = 1.322e-23  final mean = 2.725e-6
twice Kelly (f = 0.50)     above the start: 10 rounds 0.374  100 rounds 0.450  1000 rounds 0.49110
                           ruin (< 1e-6 of the capital) = 0.0244  final median = 1.000e0  final mean = 7.375e6
full Kelly (f = 0.25)      above the start: 10 rounds 0.618  100 rounds 0.684  1000 rounds 0.96120
                           ruin (< 1e-6 of the capital) = 0.0000  final median = 4.983e2  final mean = 1.127e5
half Kelly (f = 0.125)     above the start: 10 rounds 0.618  100 rounds 0.811  1000 rounds 0.99650
                           ruin (< 1e-6 of the capital) = 0.0000  final median = 1.062e2  final mean = 4.954e2

all-in: log10 of the final median = -22.9, log10 of the final mean = -5.6
part of the mean that comes from the richest trajectory (richest 1%): 1.000

== Chapter 10 :: the insurance seller (earns 1% per week, loses 95% with q = 0.004) ==
time growth of the all-in: -0.00207 per week (negative: the typical all-in trajectory decays)
Kelly fraction with the true q: 0.648 | with Laplace after 250 clean weeks: 0.652 | with the raw frequency 0/250: capped at 1
observed frequency + full Kelly  no loss = 0.368  all-in = 0.368  ruin in 1000 weeks = 0.0194  final median = 3.026e0  final mean = 1.984e2
Laplace + full Kelly             no loss = 0.368  all-in = 0.000  ruin in 1000 weeks = 0.0000  final median = 4.077e0  final mean = 2.238e1
Laplace + half Kelly             no loss = 0.368  all-in = 0.000  ruin in 1000 weeks = 0.0000  final median = 2.097e0  final mean = 3.789e0
theoretical probability of 250 weeks with no loss = 0.3671
Laplace with zero losses in 250 weeks: q_hat = 1/252 = 0.003968

== Chapter 10 :: the wild tail: Pareto with alpha = 1.1 ==
true mean (alpha/(alpha-1)) = 11.0
n =      1000: mean of the sample means =     7.76  median =    6.22  below 11 in 0.94  95% CI covers in 0.24
n =   1000000: mean of the sample means =     9.53  median =    8.39  below 11 in 0.95  95% CI covers in 0.26
share of each sample's sum that comes from its single largest value (n = 1 000 000): 0.088
in the median, 27.55% of the true mean comes from values above the maximum of a sample of one million (median maximum 397420)

Hill estimator with alpha = 1.1 and n = 200 000:
  k =    100  alpha_hat = 1.083  95% CI approx [0.870; 1.295]
  k =    500  alpha_hat = 1.063  95% CI approx [0.970; 1.157]
  k =   1000  alpha_hat = 1.100  95% CI approx [1.032; 1.168]
  k =   5000  alpha_hat = 1.101  95% CI approx [1.070; 1.131]
  k =  20000  alpha_hat = 1.095  95% CI approx [1.080; 1.111]
reading: a Gaussian model fitted to this tail underestimates the loss in every repetition

== Chapter 10 :: volatility drag at 1x, 2x and 3x ==
L = 1x: arithmetic mean = +0.000299/session, measured growth = +0.000237/session, drag = +0.000062/session (+0.0156/year), wealth in 100000 sessions = 1.903e10
L = 2x: arithmetic mean = +0.000597/session, measured growth = +0.000350/session, drag = +0.000247/session (+0.0623/year), wealth in 100000 sessions = 1.551e15
L = 3x: arithmetic mean = +0.000896/session, measured growth = +0.000339/session, drag = +0.000556/session (+0.1402/year), wealth in 100000 sessions = 5.325e14
horizon of 2000 sessions:
  L = 1x: measured growth = +0.000198/session, final wealth = 1.485e0
  L = 2x: measured growth = +0.000273/session, final wealth = 1.726e0
  L = 3x: measured growth = +0.000225/session, final wealth = 1.569e0
horizon of 10000 sessions:
  L = 1x: measured growth = +0.000313/session, final wealth = 2.276e1
  L = 2x: measured growth = +0.000501/session, final wealth = 1.496e2
  L = 3x: measured growth = +0.000565/session, final wealth = 2.835e2
the 1987 crash (-22.6% in one day) takes 67.8% of the capital at 3x, 22.6% at 1x; ruin starts at L = 4.4
with a -22.6% crash in one day and 40 days of -3% (20 thousand sessions):
  L = 1x: measured growth = +0.000142/session, final wealth = 1.720e1
  L = 2x: measured growth = +0.000155/session, final wealth = 2.222e1
  L = 3x: measured growth = +0.000033/session, final wealth = 1.944e0

== Chapter 11 :: the Bernoulli ship (C = 1,800, L = 34,000, p = 0.05) ==
threshold p > C/L: p* = 0.05294 (5.294%)  ->  p = 0.050 does not pass the threshold
in expectation: expected loss without insurance = 1700, insurance result = +100 per month
shipowner growth in time: 1 ship without insurance = -0.0208, 1 ship with insurance = -0.0182, 3 ships without insurance = -inf
insurer holding 50,000 of capital per contract (exact binomial account):
insurer with     1 contracts: growth in time = -0.018042 per month
insurer with    10 contracts: growth in time = +0.000854 per month
insurer with   100 contracts: growth in time = +0.001888 per month
insurer with  1000 contracts: growth in time = +0.001987 per month

== Chapter 11 :: the EVPI (treating costs 1, leaving a sick patient costs 10) ==
action threshold: p > 0.10  ->  with p = 0.3 the best act without information treat (-1.00)
gain of whoever knows the state before acting = -0.30
EVPI = +0.70
  test sensitive (0.90; 0.70): EVSI = +0.2200 | mutual information = 0.241 bit
  test specific  (0.30; 0.95): EVSI = +0.0000 | mutual information = 0.079 bit
  with prevalence 0.05: best act without information = do not treat, EVPI = +0.450
  with prevalence 0.30: best act without information = treat, EVPI = +0.700
  with prevalence 0.60: best act without information = treat, EVPI = +0.400

== Chapter 11 :: the van Dantzig dikes, four tail models ==
  model     xi = -0.1  xi = +0.0  xi = +0.1  xi = +0.2   optimal height
  T = 100          4.07       4.76       5.71       7.01
  T = 10000        5.84       8.27      12.78      21.49
  optimal height with xi = -0.1: 5.839 m  (cost 4.143)
  optimal height with xi = +0.0: 7.564 m  (cost 6.327)
  optimal height with xi = +0.1: 9.809 m  (cost 9.426)
  optimal height with xi = +0.2: 12.584 m  (cost 13.613)
  expected value with the Gumbel: 7.56 m
  mean of the models: 8.95 m
  Wald maximin: 12.58 m
  Savage minimax regret: 9.81 m (largest regret 3.67)
  precaution: 1 in 10 000 in the worst model = 21.49 m
  spread between precaution and the Gumbel expected value = 13.92 m
  EVPI of knowing the tail model (equal weights, cost units) = +1.919

== Chapter 11 :: the Chouldechova trilemma ==
group     base rate        FNR        FPR  false positives  false negatives        PPV
White         0.069      0.240      0.105              467               79     0.3491
Black         0.111      0.360      0.196              832              191     0.2896
observed cases: White 329 marked by the facts, Black 530

-- FNR matched downwards, to the FNR of White at 0.240 --
White         0.069      0.240      0.105              467               79
Black         0.111      0.240      0.309             1312              127
total false positives: 1299 -> 1778 (+479)
total false negatives: 270 -> 206 (-64)

-- FNR matched upwards, to the FNR of Black at 0.360 --
White         0.069      0.360      0.052              230              119
Black         0.111      0.360      0.196              832              191
total false positives: 1299 -> 1062 (-237)
total false negatives: 270 -> 309 (+40)

Chouldechova identity: FPR = p/(1-p) * (1-PPV)/PPV * (1-FNR)
group    observed FPR  FPR of identity     difference
White          0.1050           0.1050       -2.8e-17
Black          0.1960           0.1960       -2.8e-17
the trilemma: with PPV 0.3491 and FNR 0.240 in BOTH groups, the identity forces FPR 0.1050 for White and 0.1769 for Black

== Chapter 12 :: the same mean, two waits ==
2000 days of 10 h per world, one attendant, mu = 12 per hour, seed 1909 (+ world index)
world (patients per hour)    patients    wait  median     p90  >30min   margin/day     margin/pat.
8 every day (the claim)        159716     9.1     3.7    26.1      8%         0.14            0.03
7 or 9                         159057    10.1     4.0    29.1      9%         0.20            0.04
uniform between 5 and 11       160475    12.8     5.0    37.1     14%         0.32            0.05
5 or 11                        158656    19.3     8.1    55.8     24%         0.51            0.07
5 or 11, reinforcement in the surge   163683     1.9     0.0     6.8      0%         0.04            0.01

halves null: 4000 days of demand 8, 2000 splits of 2000 days against 2000
measured sd of the differences = 0.2062 min
predicted by the day unit     = 0.1928 min
predicted by the patient unit = 0.0473 min  (319704 patients)
null: max |difference| in 2000 splits = 0.664 min

severe test: observed difference = 10.2 min, 49.4 sd of the null, null fraction >= observed = 0.0000
tail: 8 in 100 patients wait more than 30 min in world 8; 24 in 100 in world 5 or 11
== Chapter 13 :: volatility drag and the selection bias ==
training 1885-1999: 31919 sessions in 115 years; whole series 1885-2023: 37930 sessions
daily arithmetic mean = 0.000240669 (0.0668 per year), daily sd = 0.010496 (0.1749 per year)

time growth per year and drag (cohorts and trajectories of the training set)
  L=1: the arithmetic sum would give 0.0668 per year; the realised growth is 0.0515; drag 0.0153
  L=2: the arithmetic sum would give 0.1336 per year; the realised growth is 0.0719; drag 0.0617
  L=3: the arithmetic sum would give 0.2004 per year; the realised growth is 0.0601; drag 0.1403
worst session: -0.2261 (-22.61%) on 1987-10-19; loss at L=1 0.226, L=2 0.452, L=3 0.678; daily ruin at L >= 4.42
  calendar year 1929-1932 at L=1: the capital ends at 0.19521 of the initial one (80.5% destroyed)
  calendar year 1929-1932 at L=3: the capital ends at 0.00088 of the initial one (99.9% destroyed)
  peak to trough, 1929-09-03 to 1932-07-08 at L=1: the capital ends at 0.10814 of the initial one (89.2% destroyed)
  peak to trough, 1929-09-03 to 1932-07-08 at L=3: the capital ends at 0.00025 of the initial one (100.0% destroyed)

20-year cohorts starting on the first trading session of each month (training set):
L           cohorts     median (x)       mean (x)  above the start
1              1134           2.13            2.9           0.9577
2              1134           2.67            7.8           0.7848
3              1134           2.10           19.1           0.6429

cohorts per unit-year (115 years, moving sum of 20): above the start at 1x = 0.9583, at 3x = 0.6458 (96 cohorts)
claim world (training years resampled, growth 3*annual arithmetic sum, without drag):
  2000 replicas: mean fraction of the cohorts above the start = 0.9286, 1% quantile = 0.6354, p(fraction <= 0.6458) = 0.0140

LaLonde: NSW with 185 treated and 260 randomised controls; CPS with 15992 observations
comparison (1978 outcome)                employment   income (US$)
randomised experiment (NSW treat. - NSW ctrl)      0.111           1794
raw comparison (NSW treat. - CPS)            -0.107          -8498
implied selection bias (raw - experiment)     -0.218         -10292
employment levels in 1978: NSW treated 0.757, NSW controls 0.646, CPS 0.864; in 100 people: 76, 65 and 86
mean income of 1978 (US$): NSW treated 6349, NSW controls 4555, CPS 14847
1975 placebo (employment): NSW treated 0.400, NSW controls 0.315, CPS 0.891; raw difference treated - CPS = -0.491
== Chapter 14 :: the tail, the clean window and the information ==
Copenhagen Re fires above 1 M DKK, 1980-1990: 2167 claims in 132 months

period        months        months hit  largest loss
1980-1982         36                 4         263.3
1983-1985         36                 1          57.4
1986-1988         36                 0          47.0
1989-1990         24                 2         152.4
months hit and value: 1980-07 263.3; 1981-05 56.2; 1981-12 50.1; 1982-10 65.7; 1985-08 57.4; 1989-08 152.4; 1990-10 144.7;

training 1980-1988: 1714 claims in 108 months
Hill on the training set: k=25 alpha=1.9625 k=50 alpha=1.8109 k=100 alpha=1.4100
Hill k=50, 95% CI by bootstrap of monthly blocks (300 replicas): [1.37; 2.61]
simulated Pareto with alpha = 1.81, n = 1500, 200 repetitions: mean Hill k=50 = 1.900 (sd 0.265), 90% of the repetitions between 1.51 and 2.39

window of 36 months without a claim on the layer:
Laplace on the window: p = (0+1)/(36+2) = 0.02632, or 1 chance in 38 per month
monthly frequency measured in the training set: p = 5/108 = 0.04630
monthly p of the training tail (published POT/GPD fit: xi = 0.406, sigma = 7.387, 83 excesses above 10): p = 0.04296
  measured frequency p = 0.04630: P(clean window | dangerous) = 0.1815; likelihood ratio = 5.35 (2.42 bits); clean months for 95% = 64
                     mutual information window x (dangerous vs negligible) = 0.5455 bits
  POT/GPD tail       p = 0.04296: P(clean window | dangerous) = 0.2058; likelihood ratio = 4.72 (2.24 bits); clean months for 95% = 69
                     mutual information window x (dangerous vs negligible) = 0.5147 bits
reference: under the rate that the vault showed (2 months hit in 24, p = 1/12), the same clean window would be worth 4.48 bits (ratio 22.3)
== Chapter 15 :: the map of knowledge ==
graph: nodes=3 (L1=1 L2=1 L3=1) E_tree=2 E_graph=4 hyperedges=3 validation=[]
topological order = ["L3_manager", "L2_erlang_c", "L1_demand"] | leaves=1 roots=2 | nodes without lens = []
proof 1 (edge to an absent node): ["edge to absent node: L2_erlang_c -> L2_ghost_node"]
proof 2 (deductive cycle): ["cycle in E_tree: [\"L1_demand\", \"L2_erlang_c\", \"L3_manager\"]"]
proof 3 (hyperedge with one member): ["hyperedge with fewer than two members: H4"]
proof 4 (node outside every lens): ["L2_orphan"] validation=[]
lenses triggered = ["Chrononomy", "Crisonomy", "Scalonomy"] | not triggered = ["Fluxonomy", "Morphonomy"]
Scalonomy: hill_alpha(k=250) = 1.8444 wild_tail=true values_above_1000=0
Scalonomy: hill_alpha(k=500) = 1.7901 wild_tail=true values_above_1000=0
Scalonomy: hill_alpha(k=1000) = 1.7949 wild_tail=true values_above_1000=0
Scalonomy: hill_alpha(k=2000) = 1.8032 wild_tail=true values_above_1000=0
Crisonomy: act (+10 in a good year, -4 in a bad one) or not act: best without information = 5.8000 | with perfect information = 7.0000 | EVPI = 1.2000
Crisonomy: only half the stake available: best without information = 2.9000 | with perfect information = 3.5000 | EVPI = 0.6000
Crisonomy: useless sensor (same payoff in both states): EVPI = 0.0000
Morphonomy: rule 1 -> marginal slope = 24.37 (treated fraction = 0.474) | by regime = [9.96, 10.09, 10.15] | invariant(tolerance 0.5) = true
Morphonomy: rule 1 with the allocation swapped -> marginal slope = -4.60 | by regime = [10.34, 9.92, 10.15] | invariant(tolerance 0.5) = true
Morphonomy: rule 2, middle regime with effect 40 -> marginal slope = 40.03 | by regime = [9.96, 40.09, 10.15] | invariant(tolerance 0.5) = false | step within the regime = 30.13
== Chapter 16 :: the A1-A8 safeguard panel ==
complete card (card h3.voos): A1-code=1 A2-data=1 A3-lifecycle=1 A4-inference=1 A5-governance=1 A6-fairness=1 A7-domain=1 A8-decision=1 | passed=8 of 8 approved=true
card with A5 and A6 removed (card h3.voos-com-defeito): A1-code=1 A2-data=1 A3-lifecycle=1 A4-inference=1 A5-governance=0 A6-fairness=0 A7-domain=1 A8-decision=1 | passed=6 of 8 approved=false
without A1-code: rejecting=["A1-code"] count_rejecting=1
without A2-data: rejecting=["A2-data"] count_rejecting=1
without A3-lifecycle: rejecting=["A3-lifecycle"] count_rejecting=1
without A4-inference: rejecting=["A4-inference"] count_rejecting=1
without A5-governance: rejecting=["A5-governance"] count_rejecting=1
without A6-fairness: rejecting=["A6-fairness"] count_rejecting=1
without A7-domain: rejecting=["A7-domain"] count_rejecting=1
without A8-decision: rejecting=["A8-decision"] count_rejecting=1
sweep: cards rejected=8 of 8 | rejections per agent=[1, 1, 1, 1, 1, 1, 1, 1]
seed read by the panel: none (the verdict is a function of the eight bits)
system 1: label=card-in-order probabilities=[0.06, 0.94] sum=1.0000 outside_[0.01;0.99]=0
== Chapter 17 :: the iron contract ==
card h0.posto-saude: layer=L1 uncertainty=aleatory header=5 fields | failures=[] total=0
declared taxonomy: layers=["L1", "L2", "L3"] uncertainties=["aleatory", "epistemic", "knightian", "not-applicable"]
defect NoSeed: failures=1 gates=Phi1-simulability detail="missing fixed seed or simulable artifact"
defect NoDefinition: failures=1 gates=Phi3-layers-and-null-model detail="no null model or no one-line definition"
defect NoMarginNoUnit: failures=1 gates=Phi2-independent-unit detail="margin without a declared independent unit"
defect NoNullModel: failures=1 gates=Phi3-layers-and-null-model detail="no null model or no one-line definition"
defect NoScopeNoPayerNoExit: failures=1 gates=Phi4-scope-and-exit detail="ecological scope, who pays for the error or exit clause missing"
defect QuestionWithoutMark: failures=1 gates=type-III-decider-question detail="decider_question must end in ? and cite the action of the package"
defect QuestionWithoutAction: failures=1 gates=type-III-decider-question detail="decider_question must end in ? and cite the action of the package"
intact graph: validation=[]
graph with a ghost edge: validation=["edge to absent node: L2_erlang -> L2_ghost"]
graph with a deductive cycle: validation=["cycle in E_tree: Some(\"L1_queue\")"]
graph with a one-member hyperedge: validation=["hyperedge with fewer than two members: H2"]
property test (10.000 simulated cards): violations=0 margin_min=0.00617 margin_max=0.09798
same seed, two validators: runs that diverge in 0 of 32
== Chapter 18 :: the vault, the seal and the audit ==
sha256("")=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
sha256("abc")=ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
chain: events=5 intact=true merkle_root=92f2ab76eb6485ab
  event 0 card=h3.voos content=ECE=0.0048 hash=902e82d6ca6cf280
  event 1 card=h3.bolsa content=mediana=2.1x hash=a9c54b2c488f0397
  event 2 card=h3.energia content=canario=ok hash=fbbcfbdbd5970a6c
  event 3 card=h3.prisoes content=ECE=0.036 hash=4248815bc8f679d9
  event 4 card=h3.ri content=ECE=0.0048 habilidade=0.019 hash=8059d78760a22282
tampering in event 2 (canario=ok -> canario=nok, event hash recalculated): intact=false root_changed=true
tampering + rewrite of 3 following events: intact=true but it changes the published root (true)
one extra character in each event: 5 of 5 alterations invalidate the chain
batch of cards: n=7 merkle_root=17679ce182647dbb
swapping one card of the batch changes the root in 7 of 7 cases
== Chapter 19 :: the gate of the invariant set ==
seed 2019 | 3 environments x 600 rows | 999 permutations per test | alpha = 0.05

world A: the environment intervenes in X (3 schools, 4 candidates)
  set                                p_location  p_scale    p_set  verdict       1-NN
  {}                                  0.0010   0.8500   0.0020  rejected     0.333
  {x1 study}                          0.0010   0.4260   0.0020  rejected     0.477
  {x2 sleep}                          0.0010   0.0610   0.0020  rejected     0.702
  {x3 noise}                          0.0010   0.8350   0.0020  rejected     0.343
  {x4 transport}                      0.0010   0.9010   0.0020  rejected     0.725
  {x1 study, x2 sleep}                0.9920   0.6480   1.0000  accepted     0.743
  {x1 study, x2 sleep, x4 transport}  0.9910   0.6150   1.0000  accepted     0.868
  accepted in the family: 4 of 16 (with the saturated ones it would be 4) | saturated tested: 0 | saturated accepted: 0
  S_hat with the saturation rule = {x1 study, x2 sleep}  | without the rule = {x1 study, x2 sleep}  | gate: PASSES (S_hat not empty)

world B: the environment acts directly on Y (the year, 3 candidates)
  set                                p_location  p_scale    p_set  verdict       1-NN
  {}                                  0.0010   0.8460   0.0020  rejected     0.333
  {x1 study}                          0.0010   0.6370   0.0020  rejected     0.477
  {x2 sleep}                          0.0010   0.3730   0.0020  rejected     0.702
  {x3 noise}                          0.0010   0.8280   0.0020  rejected     0.343
  {x1 study, x2 sleep}                0.0010   0.0350   0.0020  rejected     0.743
  accepted in the family: 0 of 8 (with the saturated ones it would be 0) | saturated tested: 0 | saturated accepted: 0
  S_hat with the saturation rule = {}  | without the rule = {}  | gate: FAILS (S_hat empty)

world C: world A plus x5, the column that is the environment itself (5 candidates)
  set                                p_location  p_scale    p_set  verdict       1-NN
  {}                                  0.0010   0.8500   0.0020  rejected     0.333
  {x1 study}                          0.0010   0.4260   0.0020  rejected     0.477
  {x2 sleep}                          0.0010   0.0610   0.0020  rejected     0.702
  {x3 noise}                          0.0010   0.8350   0.0020  rejected     0.343
  {x4 transport}                      0.0010   0.9010   0.0020  rejected     0.725
  {x1 study, x2 sleep}                0.9920   0.6480   1.0000  accepted     0.743
  {x1 study, x2 sleep, x4 transport}  0.9910   0.6150   1.0000  accepted     0.868
  {x5 class}                          0.9640   0.9990   1.0000  accepted     1.000
  {x1 study, x2 sleep, x5 class}      0.9410   1.0000   1.0000  accepted     1.000
  accepted in the family: 4 of 32 (with the saturated ones it would be 20) | saturated tested: 16 | saturated accepted: 16
  S_hat with the saturation rule = {x1 study, x2 sleep}  | without the rule = {}  | gate: PASSES (S_hat not empty)
== Chapter 20 :: the clock, the canary and the vault ==
seed 2021 | training 120000 stops (rate 0.0352) | development 58000 (rate 0.0222) | vault 24466 (rate 0.0220)
raw model (trained in the old period of higher rate): predicted mean 0.0351 (60% above the vault rate)
  CITL = +0.0132 | ECE = 0.0132 | Brier = 0.02155 | skill = -0.0020
official model with Platt (slope = 0.98, intercept = -0.54): predicted mean 0.0222
  CITL = +0.0002 | ECE = 0.0022 | Brier = 0.02130 | skill = +0.0098
climatology fixed at the past rate: Brier = 0.02151 (p(1-p) = 0.02171)

canary: a column equal to the label, published at the instant of the label
  clock honest: canary available in 0 of 24466 emissions | beta_canary = +0.00 | skill gain = +0.0000
  clock switched off: canary available in 24466 of 24466 emissions | beta_canary = +9.23 | skill gain = +0.6521
== Chapter 21 :: the verifier of the final project card ==
seven rules, and each one has to reject at least one card:

  1. R1 generative process declared
  2. R2 independent unit declared
  3. R3 null model with the same seed
  4. R4 decider question (who decides, which action, question)
  5. R5 fixed numeric seed
  6. R6 pre-registration hash with 64 hexadecimal digits
  7. R7 exit clause with an objective trigger

01 complete                  ACCEPTED (who decides: the municipal education secretariat)

02 without the decider question REJECTED
    R4 field decider_question missing

03 generic who decides       REJECTED
    R4 who decides is generic

04 the action does not appear literally REJECTED
    R4 the action of the decision does not appear literally

05 without a question mark   REJECTED
    R4 the text does not end in a question

06 without a seed            REJECTED
    R5 no fixed numeric seed

07 without an exit clause    REJECTED
    R7 no exit clause

08 clause without a trigger  REJECTED
    R7 exit clause without an objective trigger

09 without pre-registration  REJECTED
    R6 pre-registration hash missing or out of format

10 without a null model      REJECTED
    R3 null model missing or without the same seed

11 without a generator       REJECTED
    R1 generator field empty or too short

12 without a unit            REJECTED
    R2 unit field empty

12 cards: 1 accepted, 11 rejected
rules that rejected at least one card: 7 of 7 | rejections per rule: [1, 1, 1, 4, 1, 1, 2]
~~~

## 3. Deterministic Gates

### Seven gates and the safeguard panel

Seven gates (G1 to G7), identical reproduction, and Merkle root of the audit trail.

~~~bash
./target/release/gates --horizon all --reproduction
~~~

~~~text
== Stochology :: deterministic gates ==
horizon=all cards=12 defect=none
  card h0.posto-saude checks_passed=8/8 failing=[] approved=true
  card h0.banco-2003 checks_passed=8/8 failing=[] approved=true
  card h1.a-multiplicativo checks_passed=8/8 failing=[] approved=true
  card h1.b-lalonde checks_passed=8/8 failing=[] approved=true
  card h1.c-incendios checks_passed=8/8 failing=[] approved=true
  card h2.icp-tunel-de-luz checks_passed=8/8 failing=[] approved=true
  card h2.icp-fda-herg checks_passed=8/8 failing=[] approved=true
  card h2.icp-estrela-levedura checks_passed=8/8 failing=[] approved=true
  card h3.voos23 checks_passed=8/8 failing=[] approved=true
  card h3.bolsa checks_passed=8/8 failing=[] approved=true
  card h3.energia checks_passed=8/8 failing=[] approved=true
  card h3.prisoes checks_passed=8/8 failing=[] approved=true
  lenses: hill_alpha=1.7901 wild_tail=true median_log_wealth=-10.5361 ruined=0.892 murphy_rel=0.0375 res=0.1719 unc=0.2344
merkle_root=9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81
reproduction: second run identical = true (root 9716e28a87bec9cf)
G1 ok       root determinism 9716e28a87bec9cf
G2 ok       fixed seed in 12 cards
G3 ok       declared scope: simulated artifact only
G4 ok       Phi1-Phi5 and type III (failures=0)
G5 ok       SHA-256 chain intact and tampering detected
G6 ok       panel A1-A8: 96 of 96 checks declared and anchored
G7 ok       declared exit clause and A8 declared and anchored
report_hash=bb7edf1318bf68edb01b585c544c72f7975fabbb46824b4870ab2924363ae5d9
verdict: 7 gates ok
~~~

### Negative proof: gate rejection

With a planted defect, G6 rejects and exits with code 1.

~~~bash
./target/release/gates --horizon h3 --defect h3.voos23
~~~

~~~text
== Stochology :: deterministic gates ==
horizon=h3 cards=4 defect=h3.voos23
  card h3.voos23 checks_passed=6/8 failing=["A5-governance", "A6-fairness"] approved=false
  card h3.bolsa checks_passed=8/8 failing=[] approved=true
  card h3.energia checks_passed=8/8 failing=[] approved=true
  card h3.prisoes checks_passed=8/8 failing=[] approved=true
  lenses: hill_alpha=1.7901 wild_tail=true median_log_wealth=-10.5361 ruined=0.892 murphy_rel=0.0375 res=0.1719 unc=0.2344
merkle_root=802e84b64e432c259582d4d1322d602ea9634b8a05f8421e8205f3f9672252d2
G1 ok       root determinism 802e84b64e432c25
G2 ok       fixed seed in 4 cards
G3 ok       declared scope: simulated artifact only
G4 ok       Phi1-Phi5 and type III (failures=0)
G5 ok       SHA-256 chain intact and tampering detected
G6 failed   panel A1-A8: 30 of 32 checks declared and anchored
G7 ok       declared exit clause and A8 declared and anchored
report_hash=93565e652be85d38a1283f19bcaa2af7352a6057af866cc46e47c75360afeccd
verdict: 1 gates failed
~~~

### Synthetic trial: SHELF panel

Four synthetic respondents scored on the five real pretest pools.

~~~bash
./target/release/labs --rehearsal-shelf
~~~

~~~text
== synthetic rehearsal :: the SHELF panel, with declared respondents ==
label: SYNTHETIC. No line below counts panel coverage; what is measured is the floor of a
fluent and uncalibrated respondent, so that the real panel has something to be compared against.

respondent             item pool                    n    brier    skill      ece  approved
coin p=0.50            desenvolvimento_preston     22   0.2500  -0.0083   0.2727        no
fluent p=0.75          desenvolvimento_preston     22   0.2443  -0.0558   0.2500        no
yes-bias p=0.90        desenvolvimento_preston     22   0.4100  -0.6400   0.4000        no
expert p=0.85/0.50     desenvolvimento_preston     22   0.1156   0.3419   0.1750        no
oracle p=0.99          desenvolvimento_preston     22   0.0001   0.0000   0.0100        no
coin p=0.50            direito_prisoes             38   0.2500  -0.0256   0.2895        no
fluent p=0.75          direito_prisoes             38   0.2072  -0.0076   0.0921        no
yes-bias p=0.90        direito_prisoes             38   0.3679  -0.4881   0.4263        no
expert p=0.85/0.50     direito_prisoes             38   0.0644  -0.2917   0.1618        no
oracle p=0.99          direito_prisoes             38   0.0001   0.0000   0.0100        no
coin p=0.50            direito_teto_beneficio      30   0.2500  -0.0181   0.2667        no
fluent p=0.75          direito_teto_beneficio      30   0.3458  -0.4084   0.3667        no
yes-bias p=0.90        direito_teto_beneficio      30   0.3567  -0.4525   0.3333        no
expert p=0.85/0.50     direito_teto_beneficio      30   0.1135   0.1828   0.1567        no
oracle p=0.99          direito_teto_beneficio      30   0.0001   0.0000   0.0100        no
coin p=0.50            educacao_star               30   0.2500  -0.0766   0.2667        no
fluent p=0.75          educacao_star               30   0.3125  -0.2500   0.3000        no
yes-bias p=0.90        educacao_star               30   0.4100  -0.6400   0.4000        no
expert p=0.85/0.50     educacao_star               30   0.0832   0.4802   0.1767        no
oracle p=0.99          educacao_star               30   0.0001   0.0000   0.0100        no
coin p=0.50            h1_real_c                   30   0.2500  -0.2784   0.3000        no
fluent p=0.75          h1_real_c                   30   0.3292  -0.3225   0.2833        no
yes-bias p=0.90        h1_real_c                   30   0.4100  -0.6400   0.4000        no
expert p=0.85/0.50     h1_real_c                   30   0.0528   0.1509   0.1433        no
oracle p=0.99          h1_real_c                   30   0.0001   0.0000   0.0100        no

respondent             all pools                    n    brier    skill      ece  approved
coin p=0.50            all pools                  150   0.2500  -0.0310   0.1200        no
fluent p=0.75          all pools                  150   0.2858  -0.1565   0.2067        no
yes-bias p=0.90        all pools                  150   0.3887  -0.5591   0.3733        no
expert p=0.85/0.50     all pools                  150   0.0832   0.2803   0.1200        no
oracle p=0.99          all pools                  150   0.0001   0.0000   0.0100        no

Reading: the fluent respondent declares 0.75 knowing nothing. If it passes the same threshold the
panel uses, the threshold measures declared confidence and not knowledge. The expert shows the floor a real
panel needs to beat: positive skill with 80% of the items known and the rest guessed.
Declared divergence: the perfect oracle appears with skill 0 and rejected, because with accuracy 1 the
climatological variance is zero and the 0/0 ratio is read as 0 by the evaluator guard.
~~~

### Synthetic trial: affected-group listening

Coverage arithmetic on synthetic records and gate provenance boundary.

~~~bash
./target/release/labs --rehearsal-listening
~~~

~~~text
== synthetic rehearsal :: listening to those who pay for the error ==
label: SYNTHETIC. The registries below are built in code; none of them counts coverage.

real registry: 7 decisions | coverage 0 of 7 | minimum 2 | BOX SEALED

coverage arithmetic, with a synthetic registry:
  0 group(s) heard | coverage 0 of 7 | minimum 2 | BOX SEALED
  1 group(s) heard | coverage 1 of 7 | minimum 2 | BOX SEALED
  2 group(s) heard | coverage 2 of 7 | minimum 2 | BOX OPEN
  3 group(s) heard | coverage 3 of 7 | minimum 2 | BOX OPEN

-- the limit of the gate: it checks the field, not the provenance --
  seven fabricated records, with a plausible name and a transcript file: coverage 7 of 7 | BOX OPEN
  reading: the gate does not tell a signed transcript from an invented transcript, because it only looks at
the fields. Real coverage needs provenance -- the hash of the signed transcript, chained into the
governance registry -- and that does not exist today. It stays declared as the next field of the registry:
transcript_hash, checked like the seals, and not a new stamp of confidence.

-- the frozen script against the forbidden vocabulary --
  What is your worst scenario that is still plausible?                   no leak
  What would make you refuse this intervention?                          no leak
  When this policy fails, who in fact bears the cost?                    no leak
  questions with vocabulary leaks: 0 of 3
~~~

### Synthetic trial: scale bridge

Four synthetic records testing role assignment, case count, and declared status.

~~~bash
./target/release/labs --rehearsal-bridge
~~~

~~~text
== synthetic rehearsal :: the bridge thesis at scale ==
label: SYNTHETIC. No registry below counts a chosen case or a designated role.

real registry: 6 roles | pending ["escolhedor_dos_casos", "analistas_braco_A", "analistas_braco_B", "juiz", "casos (0 de 12)"] | cases missing 12 | status "nao sustentada" | THESIS NOT SUPPORTED

synthetic registries:
  no role, no case                       roles 0 of 4 | cases  0 of 12 | status "nao sustentada": box closed
  roles complete, cases missing          roles 4 of 4 | cases  0 of 12 | status "em teste": box closed | DIVERGENCE: status em teste without the designated roles and the minimum cases
  fair test assembled                    roles 4 of 4 | cases 12 of 12 | status "em teste": box open
  status forged without the fair test    roles 0 of 4 | cases  0 of 12 | status "sustentada": box closed | DIVERGENCE: declared status "sustentada" with the fair test still to be done: the thesis only changes state after the judge's result

Reading: the gate separates three things an opinion conflates -- a filled role, a chosen case
and a declared status. A "supported" status with zero cases is rejected, as the real registry already rejects.
~~~

## 4. Workspace Tests

### All crates

Unit and integration tests across the workspace.

~~~bash
cargo test --workspace
~~~

~~~text
running 109 tests
test banco::tests::erlang_c_grows_with_load ... ok
test banco::tests::one_more_agent_improves_the_level ... ok
test banco::tests::scaling_by_the_goal_returns_the_goal ... ok
test banco::tests::the_halves_null_is_the_sd_of_signed_differences ... ok
test ch01::tests::different_seeds_almost_never_match ... ok
test ch01::tests::honest_coin_concentrates_near_half ... ok
test ch01::tests::two_machines_agree_draw_by_draw ... ok
test ch02::tests::rare_coincidence_does_not_become_guilt_in_a_small_population ... ok
test ch02::tests::the_swap_factor_grows_when_prevalence_falls ... ok
test ch02::tests::when_prevalence_is_high_the_bridge_barely_lies ... ok
test ch03::tests::a_ratio_of_ten_is_ten_decibans ... ok
test ch03::tests::mammography_gives_one_in_ten ... ok
test ch03::tests::two_independent_tests_against_correlated_tests ... ok
test ch04::tests::the_bias_moves_from_two_tenths_to_six_tenths ... ok
test ch04::tests::the_fixed_interval_shrinks_and_the_discounted_one_stays_wide ... ok
test ch04::tests::the_fixed_model_is_almost_always_wrong_and_the_forgetting_model_covers ... ok
test ch05::tests::jensen_in_the_queue_varies_above_the_constant ... ok
test ch05::tests::the_uniform_jensen_mean_is_exact ... ok
test ch06::tests::berkson_induces_negative_correlation_and_persistence_reduces_effective_n ... ok
test ch07::tests::garden_inflates_false_positive_and_hot_hand_has_negative_bias ... ok
test ch07::tests::the_garden_of_paths_without_any_effect_stays_inside_its_bounds ... ok
test ch08::tests::constant_forecaster_has_zero_resolution ... ok
test ch08::tests::murphy_identity_closes_and_platt_reduces_rel ... ok
test ch09::tests::icp_recovers_the_cause_and_rejects_the_empty_set ... ok
test ch09::tests::icp_rejects_the_cause_when_the_mechanism_changes_by_site ... ok
test ch09::tests::pearl_steps_and_icp_intersection_work ... ok
test ch10::tests::drag_grows_with_the_square_of_leverage ... ok
test ch10::tests::kelly_fraction_is_the_argmax_of_growth ... ok
test ch10::tests::peters_coin_grows_in_the_ensemble_and_decays_in_time ... ok
test ch10::tests::the_share_above_the_maximum_uses_the_median_maximum_not_n ... ok
test ch10::tests::twice_kelly_ties_do_not_count_as_above_the_start ... ok
test ch11::tests::chouldechova_identity_closes_on_any_matrix ... ok
test ch11::tests::dike_gev_underestimates_with_gumbel_and_chouldechova_closes ... ok
test ch11::tests::evsi_matches_the_hand_count_and_never_exceeds_evpi ... ok
test ch11::tests::gumbel_level_grows_with_the_period ... ok
test ch11::tests::insurer_growth_rises_with_the_number_of_contracts ... ok
test ch11::tests::lowering_the_fnr_raises_the_fpr ... ok
test ch12::tests::day_margin_is_larger_than_patient_pseudoreplication ... ok
test ch12::tests::two_nurses_share_one_line_and_one_nurse_is_lindley ... ok
test ch13::tests::leverage_drag_and_selection_bias_match_stochology_horizons_h1 ... ok
test ch14::tests::laplace_avoids_zero_and_hill_recovers_tail ... ok
test ch15::tests::valid_graph_passes_and_lenses_agree_with_hypergraph ... ok
test ch16::tests::panel_approves_complete_card_and_rejects_when_a_concern_is_missing ... ok
test ch17::tests::complete_card_passes_and_a_defect_triggers_a_gate ... ok
test ch18::tests::ledger_detects_tampering_in_the_past ... ok
test ch19::tests::ols_recovers_a_line_and_kruskal_is_zero_on_identical_groups ... ok
test ch20::tests::sigmoid_inverts_logit_and_brier_measures_squared_error ... ok
test ch21::tests::verifier_accepts_a_complete_card_and_rejects_a_generic_decider ... ok
test costura::tests::a_changed_list_of_seam_kinds_is_rejected ... ok
test costura::tests::a_general_advantage_is_not_c_prime ... ok
test costura::tests::a_supported_c_prime_with_the_box_closed_is_rejected ... ok
test costura::tests::an_advantage_paid_in_false_alarms_is_not_c_prime ... ok
test costura::tests::complete_strata_without_the_roles_keep_the_box_closed ... ok
test costura::tests::partial_scores_and_unknown_strata_are_not_read ... ok
test costura::tests::the_located_advantage_is_c_prime ... ok
test costura::tests::the_record_in_the_tree_keeps_the_box_closed_and_does_not_lie ... ok
test costura::tests::the_tie_at_the_ceiling_in_s_is_not_c_prime ... ok
test ensaio::tests::fluent_and_yes_bias_stay_below_the_expert ... ok
test ensaio::tests::the_coin_has_no_skill ... ok
test ensaio::tests::the_five_real_pools_have_items ... ok
test ensaio::tests::the_real_registry_is_sealed_and_the_synthetic_one_with_seven_opens ... ok
test ensaio::tests::the_synthetic_bridge_only_opens_with_everything_and_rejects_a_forged_status ... ok
test ensaio::tests::the_three_rehearsals_run_and_label_themselves_synthetic ... ok
test eprocess::tests::a_saturated_set_makes_no_claim ... ok
test eprocess::tests::a_test_without_a_set_makes_no_claim_and_contributes_one_half ... ok
test eprocess::tests::one_strong_claim_among_empty_rounds_can_carry_an_area_and_a_weak_one_cannot ... ok
test eprocess::tests::the_clustered_row_replaces_the_recorded_one_only_in_the_second_reading ... ok
test eprocess::tests::the_sealed_table_has_the_declared_shape ... ok
test escuta::tests::a_decision_issued_with_nobody_heard_is_rejected ... ok
test escuta::tests::listening_without_a_signed_record_does_not_count ... ok
test escuta::tests::the_deadline_and_the_minimum_coverage_come_from_the_registry ... ok
test escuta::tests::the_declared_conduct_is_protocolo_cego ... ok
test escuta::tests::the_minimum_coverage_opens_the_box ... ok
test escuta::tests::the_program_registry_has_seven_decisions_and_none_heard ... ok
test fila::tests::grid_is_the_cut_of_the_instrument ... ok
test fila::tests::instrument_runs_with_a_fixed_seed ... ok
test fila::tests::monotonicity_r2_better_than_r0 ... ok
test fila::tests::queue_with_abandonment_stabilises ... ok
test fila::tests::registry_bands_and_declared_divergence ... ok
test fila::tests::the_r2_margin_is_the_p90_of_the_residual_cv ... ok
test fila::tests::weekday_of_known_dates ... ok
test fila::tests::working_print ... ok
test mc_error::tests::clopper_pearson_matches_known_values ... ok
test mc_error::tests::the_order_interval_brackets_the_quantile ... ok
test mc_error::tests::the_seeds_are_twenty_and_none_is_the_published_one ... ok
test ponte::tests::a_supported_thesis_without_a_fair_test_is_rejected ... ok
test ponte::tests::the_choice_is_mechanical_deterministic_and_in_proportion ... ok
test ponte::tests::the_fair_test_registry_declares_what_is_missing ... ok
test ponte::tests::the_list_that_does_not_support_the_proportion_is_rejected ... ok
test ponte::tests::the_pilot_re_derives_the_tie_at_the_ceiling ... ok
test ponte::tests::the_registry_declares_the_verdict_if_the_window_lapses ... ok
test ponte::tests::the_status_em_teste_without_the_roles_is_rejected ... ok
test ponte::tests::with_the_roles_designated_the_test_may_run ... ok
test propagate::tests::the_covariance_moves_the_spread_and_not_the_plug_in ... ok
test propagate::tests::the_normal_tail_is_right_at_known_points ... ok
test propagate::tests::the_wait_at_the_mean_is_ten_minutes_and_the_gradient_matches_differences ... ok
test segunda::tests::the_bank_is_recounted_in_rust_and_matches_the_published_numbers ... ok
test segunda::tests::the_bias_of_the_loop_agrees_in_two_sources ... ok
test segunda::tests::the_multiplicity_thresholds_match_alpha_over_m ... ok
test segunda::tests::the_priors_reproduction_has_33_domains_and_one_declared_divergence ... ok
test segunda::tests::the_registry_declares_the_classes ... ok
test segunda::tests::the_rereading_and_the_opinions_are_counted ... ok
test streams::tests::christoffersen_rejects_clustered_violations ... ok
test streams::tests::ece_is_zero_for_boxes_that_match_their_rate ... ok
test streams::tests::kupiec_accepts_the_nominal_rate_and_rejects_ten_times_it ... ok
test streams::tests::logistic_recovers_a_known_coefficient ... ok
test streams::tests::the_chi_square_tail_is_right_at_its_quantiles ... ok
test streams::tests::the_civil_calendar_is_right ... ok
test streams::tests::the_null_of_the_ece_shrinks_with_the_size_of_the_vault ... ok

test result: ok. 109 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 93 tests
test agents::tests::a_missing_safeguard_fails_the_agent_of_that_band ... ok
test agents::tests::eight_agents_approve_a_card_with_safeguards ... ok
test agents::tests::safeguard_without_structural_anchor_does_not_pass ... ok
test agents::tests::the_report_is_a_function_of_the_bits_only ... ok
test audit::tests::repeating_the_last_hash_keeps_the_root_a_declared_limit ... ok
test audit::tests::sha256_matches_the_known_vectors ... ok
test audit::tests::the_chain_detects_tampering ... ok
test audit::tests::the_merkle_root_is_reproducible ... ok
test audit::vault::tests::a_single_opening_and_the_second_failure_is_recorded ... ok
test audit::vault::tests::degenerate_cut_empty_series_and_unordered_series_are_refused ... ok
test audit::vault::tests::erring_after_the_opening_requires_a_new_vault_and_not_an_adjustment ... ok
test audit::vault::tests::splitting_by_index_and_by_time_do_not_overlap ... ok
test audit::vault::tests::tampering_with_the_record_invalidates_the_verification ... ok
test audit::vault::tests::the_merkle_root_changes_when_the_content_changes ... ok
test budget::tests::a_budget_that_closes_is_read_with_a_negative_covariance ... ok
test budget::tests::a_budget_that_does_not_close_is_refused ... ok
test budget::tests::a_dominant_event_needs_a_probability_inside_the_open_interval ... ok
test erlang::tests::erlang_b_and_erlang_c_stay_in_range ... ok
test erlang::tests::minimal_scale_meets_the_target_and_one_less_does_not ... ok
test funnel::tests::a_channel_without_information_zeroes_the_evsi_and_forces_an_immediate_stop ... ok
test funnel::tests::a_complete_card_passes_the_four_filters ... ok
test funnel::tests::a_more_accurate_exam_is_not_worth_less ... ok
test funnel::tests::f1_refuses_a_card_without_a_decision_maker_question ... ok
test funnel::tests::f2_also_refuses_a_card_without_a_seed ... ok
test funnel::tests::f2_refuses_a_card_without_a_generative_process ... ok
test funnel::tests::f3_refuses_a_card_without_a_null_model ... ok
test funnel::tests::f3_refuses_a_card_without_an_independent_unit ... ok
test funnel::tests::f4_refuses_a_card_without_a_declared_action ... ok
test funnel::tests::f4_refuses_a_card_without_an_exit_clause ... ok
test funnel::tests::strict_dominance_invariant_in_the_hidden_probability ... ok
test funnel::tests::the_counter_adds_up_by_gate_and_keeps_the_sequence ... ok
test funnel::tests::the_funnel_counts_the_attempts_up_to_the_final_version ... ok
test funnel::tests::the_funnel_stops_at_the_first_gate ... ok
test funnel::tests::the_measured_evsi_matches_the_exact_one ... ok
test funnel::tests::the_pruning_declares_no_uncertainty_at_the_extreme ... ok
test funnel::tests::the_pruning_does_not_fire_when_the_evsi_covers_it ... ok
test funnel::tests::the_pruning_fires_when_the_evsi_does_not_cover_the_threshold ... ok
test funnel::tests::the_upper_bound_limits_the_evsi_of_any_exam ... ok
test invariants::tests::a_complete_card_at_r1_day_passes_phi5 ... ok
test invariants::tests::a_complete_card_passes ... ok
test invariants::tests::a_resolution_finer_than_the_day_also_passes ... ok
test invariants::tests::a_unit_finer_than_the_day_stays_outside_the_declared_phi5 ... ok
test invariants::tests::a_week_unit_also_fails_with_r0_aggregated ... ok
test invariants::tests::an_aggregated_action_against_a_day_unit_fails_phi5 ... ok
test invariants::tests::declared_limit_the_type_iii_lock_does_not_read_meaning ... ok
test invariants::tests::the_phi5_gate_code_appears_in_the_failure_report ... ok
test invariants::tests::the_type_iii_lock_names_the_missing_part ... ok
test invariants::tests::without_a_decision_maker_question_type_iii_fails ... ok
test invariants::tests::without_a_seed_phi1_fails ... ok
test invariants::tests::without_a_unit_phi5_does_not_fire_and_phi2_does ... ok
test layer::tests::new_brings_r1_day_as_the_default_resolution ... ok
test layer::tests::with_resolution_swaps_the_resolution_and_complete_preserves_it ... ok
test orchestrator::tests::a_single_attack_runs_and_refuses_a_corrupted_trail_hash ... ok
test orchestrator::tests::panel_rejects_an_incomplete_card ... ok
test resolution::tests::below_the_threshold_refining_buys_less_than_above ... ok
test resolution::tests::the_conservation_fails_an_aggregate_that_does_not_add_up ... ok
test resolution::tests::the_implication_check_finds_the_counterexample_and_not_the_violation ... ok
test resolution::tests::the_order_of_resolutions_is_the_order_of_fineness ... ok
test resolution::tests::the_transfer_has_no_violation_and_has_a_counterexample ... ok
test resolution::tests::with_the_calibrated_plan_the_threshold_exists_and_the_fraction_grows ... ok
test resolution::tests::with_the_erlang_plan_prediction_3_does_not_hold ... ok
test resolution::tests::with_the_same_resource_the_fine_question_waits_less ... ok
test resolution::tests::without_variation_there_is_no_resolution_loss ... ok
test rng::tests::different_seeds_diverge ... ok
test rng::tests::same_seed_same_sequence ... ok
test s1::tests::case_that_must_fail_a_card_with_thematic_evidence_does_not_fall_into_others ... ok
test s1::tests::case_that_must_fail_a_short_match_is_not_evidence ... ok
test s1::tests::ceiling_reprojection_preserves_the_sum ... ok
test s1::tests::determinism_same_input_same_output ... ok
test s1::tests::happy_paths_every_theme_finds_its_category ... ok
test s1::tests::long_suffix_beats_short_theme ... ok
test s1::tests::no_probability_at_zero_or_one ... ok
test s1::tests::normalization_is_stable ... ok
test s1::tests::reading_of_the_record ... ok
test s1::tests::sum_one_after_the_adjustment ... ok
test s1::tests::the_tie_break_is_the_declared_order ... ok
test s1::tests::unknown_input_falls_into_others ... ok
test schema::tests::a_budget_that_does_not_close_is_refused_when_read ... ok
test schema::tests::a_chain_with_a_hole_fails ... ok
test schema::tests::a_chain_with_a_hole_or_an_unknown_operation_fails ... ok
test schema::tests::a_document_of_version_1_0_migrates_with_the_declared_defaults ... ok
test schema::tests::a_document_without_a_version_fails ... ok
test schema::tests::a_line_break_in_the_definition_survives ... ok
test schema::tests::a_missing_field_becomes_none ... ok
test schema::tests::a_missing_step_fails_with_the_version_of_the_hole ... ok
test schema::tests::a_version_1_2_document_migrates_with_the_new_fields_absent ... ok
test schema::tests::an_invalid_value_fails ... ok
test schema::tests::from_text_fails_on_free_text ... ok
test schema::tests::text_and_from_text_preserve_the_card ... ok
test schema::tests::the_budget_and_the_dominant_event_survive_the_round_trip ... ok
test schema::tests::the_chain_applies_rename_drop_and_addition ... ok
test schema::tests::the_chain_does_not_touch_a_document_already_at_the_card_version ... ok
test schema::tests::the_chain_of_the_file_is_continuous_and_ends_at_the_card_version ... ok

test result: ok. 93 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 3 tests
test tests::defect_in_h3_voos23_fails_gate_g6 ... ok
test tests::each_individual_horizon_closes_the_seven_gates ... ok
test tests::seven_gates_pass_the_all_horizon_with_an_identical_reproduction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 68 tests
test h0::tests::erlang_c_falls_with_more_servers_and_stays_between_b_and_one ... ok
test h0::tests::erlang_matches_the_exact_reference ... ok
test h0::tests::heavy_tail_bursts_the_dispersion_index ... ok
test h0::tests::minimal_scale_meets_target_and_one_less_does_not ... ok
test h0::tests::null_series_is_deterministic ... ok
test h0::tests::poisson_null_matches_mean_and_variance ... ok
test h0::tests::quantile_interpolates_and_stays_inside_the_sample ... ok
test h0::tests::recurrence_and_log_sum_agree ... ok
test h0::tests::summary_brings_median_and_quantiles_alongside_mean ... ok
test h1::tests::drag_grows_with_leverage ... ok
test h1::tests::drag_of_symmetric_series_has_exact_value ... ok
test h1::tests::hill_recovers_the_alpha_of_a_pareto ... ok
test h1::tests::median_and_mean_tell_different_stories ... ok
test h1::tests::selection_bias_flips_the_sign_of_the_gain ... ok
test h1::tests::the_1987_day_brings_down_whoever_is_at_three_times ... ok
test h2::genetics::tests::floor_falls_with_the_power_of_the_lines ... ok
test h2::genetics::tests::the_design_is_deterministic ... ok
test h2::genetics::tests::the_line_artifact_passes_without_the_second_line ... ok
test h2::genetics::tests::the_second_line_separates_the_artifact_from_the_mover ... ok
test h2::genetics::tests::with_the_second_line_every_effect_has_an_interval ... ok
test h2::genetics::tests::without_the_second_line_the_interval_is_a_point ... ok
test h2::tests::bonferroni_drops_one_area_and_counts_k ... ok
test h2::tests::k_counter_joins_repeated_specifications ... ok
test h2::tests::round_keeps_the_area_and_the_subset_count ... ok
test h2::tests::shuffled_null_shows_the_power_of_the_test ... ok
test h2::tests::ten_candidates_give_one_thousand_and_twenty_four_subsets ... ok
test h2::tests::unit_grain_changes_the_accepted_family ... ok
test h2::tests::without_area_the_round_returns_an_empty_area ... ok
test h3::beta::tests::incomplete_beta_matches_the_closed_form_of_beta_2_2 ... ok
test h3::beta::tests::incomplete_beta_matches_the_uniform ... ok
test h3::beta::tests::log_gamma_matches_factorials ... ok
test h3::beta::tests::the_fit_is_deterministic ... ok
test h3::beta::tests::the_fit_of_the_uniform_stays_symmetric ... ok
test h3::beta::tests::the_fit_recovers_the_beta_that_generated_the_quartiles ... ok
test h3::beta::tests::the_quantile_recovers_the_known_quartiles ... ok
test h3::chain::tests::a_derived_line_without_a_versioned_record_fails ... ok
test h3::chain::tests::the_chain_has_four_lines_with_the_marks_of_the_record ... ok
test h3::chain::tests::the_json_reader_reads_the_real_record ... ok
test h3::chain::tests::the_loop_is_rederived_from_the_versioned_record ... ok
test h3::chain::tests::the_record_that_lies_in_the_ratio_fails ... ok
test h3::chain::tests::the_verdict_flips_when_the_ratio_enters_the_band ... ok
test h3::chain::tests::the_weak_instrument_is_undecided ... ok
test h3::pretest::tests::a_calibrated_pretest_passes_the_four_criteria ... ok
test h3::pretest::tests::inverted_probability_fails_by_monotonicity ... ok
test h3::pretest::tests::p_is_the_unit_of_the_protocol ... ok
test h3::pretest::tests::protocol_criterion_4_cannot_fail ... ok
test h3::pretest::tests::the_binomial_test_does_not_fail_the_calibrated_bucket ... ok
test h3::pretest::tests::the_binomial_test_fails_the_impossible_bucket ... ok
test h3::pretest::tests::the_bucket_at_the_mean_has_p_near_one ... ok
test h3::pretest::tests::the_murphy_identity_closes_exactly ... ok
test h3::pretest::tests::the_null_is_deterministic ... ok
test h3::pretest::tests::the_overshooting_pretest_fails_by_ece_and_by_bucket ... ok
test h3::pretest::tests::the_pretest_null_grows_when_the_sample_shrinks ... ok
test h3::pretest::tests::the_protocol_design_puts_two_items_per_bucket ... ok
test h3::shelf::tests::asymmetric_quartiles_leave_a_visible_error ... ok
test h3::shelf::tests::narrow_extremes_fail_in_the_control ... ok
test h3::shelf::tests::phi_matches_the_table ... ok
test h3::shelf::tests::recovers_the_normal_that_generated_the_quartiles ... ok
test h3::shelf::tests::the_anchor_and_the_shift_have_declared_limits ... ok
test h3::shelf::tests::the_anchor_predicts_what_the_prior_says ... ok
test h3::shelf::tests::the_implied_width_matches_the_elicited_one_when_the_extremes_are_from_the_normal ... ok
test h3::shelf::tests::the_symmetric_fit_of_the_mean_at_q50 ... ok
test h3::tests::brier_skill_rewards_the_good_and_punishes_the_bad ... ok
test h3::tests::ece_is_zero_for_the_calibrated_and_high_for_the_overshooting ... ok
test h3::tests::ece_tolerance_depends_on_n ... ok
test h3::tests::the_canary_catches_the_reading_of_the_future ... ok
test h3::tests::the_loop_measures_what_the_action_does_to_the_indicator ... ok
test h3::tests::the_passive_correlation_follows_the_exact_account ... ok

test result: ok. 68 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 13 tests
test lenses::tests::brier_and_murphy_close_the_account ... ok
test lenses::tests::deterministic_chain_respects_processing ... ok
test lenses::tests::evpi_of_the_example_is_three ... ok
test lenses::tests::hill_recovers_pareto_alpha ... ok
test lenses::tests::invariant_regime_approves_broken_regime ... ok
test lenses::tests::peters_coin_is_not_ergodic ... ok
test lenses::tests::simple_slope ... ok
test lenses::tests::the_sensor_of_chapter_12_is_worth_what_it_saves_in_bad_years ... ok
test lenses::tests::volatility_drag_discounts_variance ... ok
test tests::complete_graph_validates ... ok
test tests::deductive_cycle_fails ... ok
test tests::edge_to_missing_node_fails ... ok
test tests::node_without_lens_is_flagged ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 26 tests
test drift::tests::kl_is_zero_against_itself_and_grows_with_distance ... ok
test drift::tests::surveillance_is_deterministic ... ok
test drift::tests::the_downward_jump_also_fires ... ok
test drift::tests::the_jump_fires_the_alarm_after_it_happens ... ok
test drift::tests::the_null_does_not_fire_in_one_hundred_thousand_points ... ok
test fairness::tests::calibration_finds_the_miscalibrated_region ... ok
test fairness::tests::equal_groups_have_no_amplitude ... ok
test fairness::tests::parity_measures_the_injected_gap ... ok
test fairness::tests::the_ece_tolerance_falls_with_the_square_root_of_n ... ok
test icp::tests::collinear_predictors_fail ... ok
test icp::tests::empty_area_when_nothing_is_invariant ... ok
test icp::tests::ks_knows_its_own_threshold ... ok
test icp::tests::the_group_unit_shrinks_the_family ... ok
test icp::tests::the_invariant_predictor_is_accepted_and_the_confounder_is_rejected ... ok
test icp::tests::the_set_count_matches_the_enumeration ... ok
test icp::tests::the_target_as_predictor_fails ... ok
test icp::tests::the_threshold_falls_with_the_sample_size ... ok
test sequential::tests::a_failed_round_costs_evidence_in_the_product_and_not_in_the_bound ... ok
test sequential::tests::a_zero_p_value_is_clamped_not_trusted ... ok
test sequential::tests::calibrators_are_decreasing_and_never_negative ... ok
test sequential::tests::calibrators_have_expectation_at_most_one_under_the_null ... ok
test sequential::tests::invalid_e_values_are_recorded_as_zero_and_the_ledger_is_deterministic ... ok
test sequential::tests::real_signal_accumulates_and_crosses ... ok
test sequential::tests::the_mean_is_valid_under_dependence_and_slower ... ok
test sequential::tests::the_naive_rule_does_not_survive_the_same_path ... ok
test sequential::tests::ville_bounds_the_product_under_optional_stopping ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 83 tests
test belief::tests::invalid_evidence_does_not_zero_the_state ... ok
test belief::tests::the_belief_state_never_hits_zero_nor_one ... ok
test belief::tests::the_card_evidence_moves_belief_to_the_layer_of_the_card ... ok
test belief::tests::the_ranking_breaks_ties_by_declared_order ... ok
test bridge::tests::a_non_finite_score_does_not_become_a_record ... ok
test bridge::tests::absence_returns_absent_and_nan_is_refused ... ok
test bridge::tests::an_output_outside_the_schema_becomes_an_abstention ... ok
test bridge::tests::the_cost_per_call_and_the_adequacy_come_out_of_the_rehearsal ... ok
test bridge::tests::the_declared_quantization_closes_one_third ... ok
test bridge::tests::the_determinism_proof_fails_the_unstable_model ... ok
test bridge::tests::the_determinism_proof_passes_on_the_rehearsal_model ... ok
test bridge::tests::the_margin_at_the_extreme_is_refused ... ok
test bridge::tests::the_measure_goes_and_returns_through_the_card ... ok
test bridge::tests::the_record_refuses_half_a_measure ... ok
test bridge::tests::weights_with_a_different_hash_refuse ... ok
test bridge::tests::without_verified_weights_the_bridge_does_not_run ... ok
test calibration::tests::a_probability_at_the_extreme_refuses ... ok
test calibration::tests::a_sample_below_the_minimum_does_not_publish_a_score ... ok
test calibration::tests::an_id_out_of_order_refuses ... ok
test calibration::tests::an_outcome_that_is_neither_0_nor_1_is_an_unreadable_artifact ... ok
test calibration::tests::the_abstention_leaves_the_denominator_and_enters_the_rate ... ok
test calibration::tests::the_ece_uses_the_h3_rule_and_the_declared_buckets ... ok
test calibration::tests::the_hash_gate_holds_for_the_labeled_set ... ok
test calibration::tests::the_labeled_set_reads_and_the_brier_matches ... ok
test calibration::tests::without_data_there_is_no_brier ... ok
test hypergraph::tests::a_node_absent_from_the_graph_is_worth_nothing ... ok
test hypergraph::tests::coverage_counts_distinct_lenses ... ok
test hypergraph::tests::deliberation_finds_the_lens_free_node_where_the_greedy_agent_stops_earlier ... ok
test hypergraph::tests::the_action_declares_the_type_of_the_edge ... ok
test hypergraph::tests::the_sample_graph_is_valid_and_has_one_uncovered_node ... ok
test hypergraph::tests::the_search_in_the_sample_graph_reaches_the_lens_free_front ... ok
test hypergraph::tests::the_search_is_deterministic_in_the_test_graph ... ok
test inference::tests::a_large_step_drops_the_acceptance ... ok
test inference::tests::a_target_outside_the_support_accepts_nothing ... ok
test inference::tests::the_chain_is_deterministic_in_the_seed ... ok
test inference::tests::the_chain_recovers_the_conjugate_posterior_of_the_beta ... ok
test inference::tests::the_chain_recovers_the_mean_of_a_normal ... ok
test lats::tests::a_world_without_actions_does_not_decide ... ok
test lats::tests::deliberation_beats_the_greedy_agent ... ok
test lats::tests::the_decision_is_deterministic_in_the_seed ... ok
test panel::tests::g8_1_a_different_seed_changes_the_matrix ... ok
test panel::tests::g8_1_a_short_workbench_refuses ... ok
test panel::tests::g8_1_lens_preserved_and_distinct_bodies ... ok
test panel::tests::g8_1_the_assignment_is_reproducible ... ok
test panel::tests::g8_2_a_reading_without_an_artifact_refuses ... ok
test panel::tests::g8_2_a_veto_with_an_unknown_artifact_refuses ... ok
test panel::tests::g8_3_a_duplicate_veto_refuses ... ok
test panel::tests::g8_3_a_full_window_quota_zeroes_the_power_of_veto ... ok
test panel::tests::g8_3_an_exhausted_quota_refuses ... ok
test panel::tests::g8_4_a_full_quorum_upholds ... ok
test panel::tests::g8_4_an_insufficient_quorum_archives ... ok
test panel::tests::g8_4_silence_after_the_deadline_archives ... ok
test panel::tests::g8_4_silence_before_the_deadline_is_pending ... ok
test panel::tests::g8_5_a_dogmatic_bet_refuses ... ok
test panel::tests::g8_6_a_valid_veto_brings_it_down_and_consumes ... ok
test panel::tests::g8_6_the_veto_wins_even_without_quorum ... ok
test panel::tests::g8_7_a_lens_outside_the_assignment_refuses ... ok
test panel::tests::g8_8_apply_outside_the_pure_function ... ok
test panel::tests::g8_8_the_seal_is_deterministic_and_sensitive ... ok
test panel::tests::phi4_a_card_without_whoever_pays_refuses ... ok
test panel::tests::phi4_a_piece_without_a_unit_refuses ... ok
test panel::tests::the_receipt_produces_a_line_without_an_opinion ... ok
test planning::tests::an_expensive_margin_returns_to_the_aggregate ... ok
test planning::tests::the_evaluation_declares_the_steps_out_of_reach ... ok
test planning::tests::the_resolution_ladder_comes_from_the_core ... ok
test planning::tests::with_volatility_the_loss_pays_for_the_margin ... ok
test planning::tests::without_volatility_the_loss_is_zero_and_the_aggregate_suffices ... ok
test runner::tests::a_malformed_or_misplaced_model_line_is_an_unreadable_artifact ... ok
test runner::tests::a_request_without_a_frozen_answer_becomes_an_abstention ... ok
test runner::tests::a_wrong_header_is_an_unreadable_artifact ... ok
test runner::tests::one_swapped_byte_in_the_artifact_refuses_the_bridge ... ok
test runner::tests::the_brier_is_the_own_rule_outside_the_fit ... ok
test runner::tests::the_model_line_goes_inside_the_seal_and_comes_back ... ok
test runner::tests::the_operator_writes_the_artifact_and_the_runner_reads_it ... ok
test runner::tests::the_rehearsal_artifact_loads_and_the_hash_matches ... ok
test runner::tests::the_runner_closes_determinism_in_the_form_of_g1 ... ok
test runner::tests::the_v2_artifact_stores_the_request_text_and_v1_keeps_reading ... ok
test runner::tests::without_a_declared_hash_the_bridge_does_not_run ... ok
test stress::tests::a_card_with_all_eight_cares_fails_in_no_order ... ok
test stress::tests::belief_learns_the_weak_band_and_attacks_it_first ... ok
test stress::tests::the_attack_is_deterministic ... ok
test stress::tests::the_panel_belief_respects_the_floor_and_the_ceiling ... ok
test stress::tests::without_learning_the_deliberative_order_is_the_declared_one ... ok

test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 9 tests
test a_malformed_model_line_aborts_and_writes_nothing ... ok
test a_probability_at_the_extreme_aborts_and_writes_nothing ... ok
test the_check_mode_passes_with_thirty_pairs ... ok
test the_check_mode_refuses_an_answer_outside_the_schema ... ok
test the_check_mode_validates_without_touching_a_destination ... ok
test the_escape_of_the_request_does_not_break_the_key_of_the_artifact ... ok
test the_ingestor_writes_the_artifact_and_the_set_with_the_seals ... ok
test the_model_line_of_the_source_enters_the_sealed_artifact ... ok
test without_arguments_the_ingestor_prints_the_usage_and_exits_with_1 ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 3 tests
test tests::coin_matches_the_probability_and_does_not_depend_on_the_clock ... ok
test tests::probability_never_hits_the_extremes ... ok
test tests::the_bridge_delivers_the_same_numbers_as_the_pure_function ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs
~~~

## 5. State Manifest

Every empirical point declaration with its file hash. The full chain is verified via `python3 scripts/check_seals.py`, and the table below is generated rather than transcribed.

| Point | Declaration | SHA-256 |
| --- | --- | --- |
| p01-tese | DECLARACAO.md | 7b9dc704e6a908857de25014941d4df55126ec6ba7bda6904c63fdb80d2b5873 |
| p02-independencia | DECLARACAO.md | 832610ff76a9aaa4198372f5573e93c0988621bf395ab87fb50d47e6e5eb03ca |
| p03-multiplicidade | DECLARACAO.md | 25577c99a5d949e79d62a1600274595952a2dec8f32680284a3b7eff109f0f6b |
| p05-prioris | DECLARACAO.md | fafd2c78203e587ed303592a047a14c01de63b90d76d4d997b544bd0f59be744 |
| p05-prioris | DECLARACAO_2.md | b35993c1dc8952681adaffc9de541ea9cd2e2555d27882dc23ac789e2ae9e5a3 |
| p06-laco | DECLARACAO.md | db599d55dd87a775ead7c0bf17daa231780d64c0a35a55852b15e8c302657b1c |
| p07-icp | DECLARACAO.md | 95fc79a046c27d1c0003f6785aec9e4088e8d445b9beaee4979842abfd1e1f48 |
| p07-icp | DECLARACAO_2.md | 46ba8d7a377c50f07b01a5a6ed48b677548c886f03efc3e31b82d347f7d220f6 |
| p08-calibracao | DECLARACAO.md | bdcab5e29cf4de4ad175cdafcdeacfffb4c5c6c0b799ecf12cddee130ddc761f |
| p12-shelf | DECLARACAO.md | ae53776270b4d0bc51cb0d710c371637a04f36d9d7826242536a2b4c15a0f01f |
| p13-escuta | DECLARACAO.md | 6f3bc267cb013074379b92a9eb3d6226d303f5c0db5c03519fd32a79dd0e3e76 |
| p13-escuta | DECLARACAO_2.md | 8c841d2cccee9ba143dfce813428c424e83d10a51d9d0ea10c667d1e7725a11a |
| p14-h3 | DECLARACAO.md | e27d1982b167885abf2779ff3fcf23c61efaa42b8abab9fc91641c90962b387a |
| p15-previsoes-r | DECLARACAO.md | fafe4472e1000f452fcb9907c21c277370a0469d2dad0ba6ed173e876feecb0d |
| p16-ponte | DECLARACAO.md | 9c0b7b372e298b4a8b86059b1b24716450315f7a285a2464b993a9dd6e63ea46 |
| p16-ponte | DECLARACAO_2.md | e011c590d9797c3ddf078c492590f911b084b35b2ca9345dd5d841976b3831b6 |
| p16-ponte | DECLARACAO_3.md | ebc18d4bdfe42b39b9d150305ad93a998d5e43b97d019b8d2cf30b60f7bfc547 |
| p17-schema | DECLARACAO.md | 86d583d6db7071f1e5106ada4a8b21b5204f9dd617227f6cbac432b16d5e857a |
| p17-schema | DECLARACAO_2.md | 84e6e1ce14273d409dba7740a9e8a203d802ca3a3f3842ef630533b47dfc3d22 |
| p18-eixo-r | DECLARACAO.md | aa6f52d52ab39cc9a5373b8f4c81e2327b8ab9b1b1d7d34ad08ea4821f2acc4d |
| p19-segunda-opiniao | DECLARACAO.md | 65b15402a12d96c866a471cc75154b9fda8ec753649b903bc984e059e9c60453 |
| p20-jev | DECLARACAO.md | 57905643db48e491085e695e87ff19d02e4af432ca660066f5106f0e09091828 |
| p21-camadas | DECLARACAO.md | 92d6539ca4fe163b38796ab669cc8972edf6896d0d04bcd5a6d5a57469f8f7a5 |
| p22-parada | DECLARACAO.md | 7e2b0295279be3fba1b5722d914fdb92194d6732c5c905ff75a57e65f8fa843b |
| p22-parada | DECLARACAO_2.md | db71d78b1c669833b39db53056b2cdfeee4ae671dffcdcf04ee35fe0aa07c288 |
| p23-onda | DECLARACAO.md | 0dff3c93f89a33a2ea2ac765416572b1a2efcfc459d1dac983eb4cbdf0622615 |
| p23-onda | DECLARACAO_2.md | 62a41e44ecab70006e3bbebd92c91e368f2f8e594c456d3c8ef648dd378b4994 |
| p24-ressonancia | DECLARACAO.md | 2eee5af36122558ce06b917abd7095b2154e2db10c79676aa9bea61505d8fe19 |

Point declarations: 28, of which 8 are declaration 2 or later (declared rectification with both values) | empirical points: 20 | `labs` instruments: 23 (--all,--bank,--bridge,--chapter,--elicitation,--eprocess,--genetics,--h3,--jev,--listening,--mc-error,--pretest,--propagate,--queue,--rehearsal-bridge,--rehearsal-listening,--rehearsal-shelf,--resolution,--s1,--s2,--schema,--second-opinion,--streams; plus 5 modifier flags: --answer-key,--answers,--measure,--scale,--threshold) | chapters: 21 | crates: 8

Assembled book: `book/stochology-textbook.md` with **6654 lines** and **92188 words**, sha256 `f57f6f3c2760bc883ff0d4edb9864f1295f1e07378e01658d4ae501a9306ae40`. The file on disk must match what `scripts/book/build.py` assembles, verified character by character by `python3 scripts/book/build.py --check`.
