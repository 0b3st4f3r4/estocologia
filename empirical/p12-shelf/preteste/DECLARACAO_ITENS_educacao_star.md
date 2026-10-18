# Declaration of items of the pre-test — `h2/educacao_star`

**Frozen on 2026-10-16**, before the expert answered anything.

| what | value |
| --- | --- |
| pool of origin | `empirical/p12-shelf/preteste/educacao_star.yaml`, 30 items (15 `sim` and 15 `nao`), 12 discarded, SHA-256 `5c4b6890ea73e845c603b3bb22c8ea3484a71229ae8be15b3b50c37588441c8a` |
| seed of the draw | `2026` |
| rule of the draw | **stratified**: shuffles each stratum of resolution with the `SplitMix64` of the repository, takes 10 from each, and shuffles the 20 for the order of presentation |
| answer key | it stays with the facilitator; the expert sees **only the statements** of this declaration |

## The filter of the answer key, declared, with **two degrees** of checking

The pool of this domain asked for balance from the start (15 `sim` and 15 `nao`), and the **30 items were checked by the facilitator** — no item was excluded for lack of an answer key, as happened in `h2/direito_prisoes`. The checking has two degrees, and each item carries its own:

- **exact equality**: the value of the source was read by machine and compared with the declared number — World Bank (13 items, API `api.worldbank.org`) and Eurostat (7 items, JSON-stat API). **20 items.**
- **presence on the cited page**: the declared number appears in the text of the page or of the downloaded PDF (`pdftotext` on the PDF of IBGE) — NCES (6), IBGE Census 2022 (2), and the PISA digest (2). **10 items.**

The second degree is weaker than the first, and it is written so as not to be confused with it.

## The 20 items, in the order in which the expert sees them

| # | id | statement | declared difficulty |
| --- | --- | --- | --- |
| 1 | E20 | In the base year 2022, was the proportion of people aged 25 to 34 with completed tertiary education in Ireland higher than 65%? | difficult |
| 2 | E26 | In the base year 2022, was the average mathematics score of 8th-grade students of the United States in the NAEP assessment higher than 280? | medium |
| 3 | E01 | In the base year 2019, was total public expenditure on education in Brazil, measured as a percentage of GDP, higher than 4%? | easy |
| 4 | E28 | In the base year 2022, was the literacy rate of people aged 15 or over in the Northeast Region of Brazil higher than 90%? | medium |
| 5 | E15 | In the base year 2022, was the pupil-teacher ratio in primary education in Greece higher than 10? | medium |
| 6 | E29 | In the base year 2018, was the average reading score of 15-year-old students of Estonia in the PISA assessment higher than 520? | difficult |
| 7 | E21 | In the school year 2021-22, was the average four-year adjusted cohort graduation rate (ACGR) of the public schools of the United States higher than 85%? | easy |
| 8 | E13 | In the base year 2019, was the completion rate of primary education in India higher than 99%? | difficult |
| 9 | E18 | In the base year 2022, was the proportion of people aged 25 to 34 with completed tertiary education in Italy higher than 35%? | medium |
| 10 | E07 | In the base year 2019, was the completion rate of primary education in South Africa higher than 95%? | difficult |
| 11 | E25 | In the base year 2019, was the expenditure per full-time equivalent (FTE) pupil at the primary and secondary level in South Korea higher than 15 000 dollars (constant 2021 dollars, adjusted by purchasing power parity)? | difficult |
| 12 | E23 | In the autumn of 2021, was the number of undergraduate enrolments in the higher education institutions of the United States higher than 15 million? | easy |
| 13 | E14 | In the base year 2022, was the pupil-teacher ratio in primary education in Romania higher than 15? | easy |
| 14 | E12 | In the base year 2019, was the gross enrolment rate in tertiary education in China higher than 70%? | easy |
| 15 | E04 | In the base year 2018, was public expenditure per primary pupil in Costa Rica, measured as a percentage of GDP per capita, higher than 15%? | easy |
| 16 | E27 | In the base year 2022, was the literacy rate of people aged 15 or over in Brazil higher than 90%? | easy |
| 17 | E16 | In the base year 2022, was the pupil-teacher ratio in primary education in Czechia higher than 16? | difficult |
| 18 | E06 | In the base year 2019, was the completion rate of primary education in Mexico higher than 100%? | medium |
| 19 | E02 | In the base year 2022, was total public expenditure on education in Mexico, measured as a percentage of GDP, higher than 6%? | easy |
| 20 | E30 | In the base year 2018, was the average reading score of 15-year-old students of Brazil in the PISA assessment higher than 450? | easy |

## What the expert receives and what he returns

- He receives: the 20 statements and the request for `p in (0, 1)` for each one, with the instruction to answer from memory, without consultation.
- He returns: 20 probabilities.
- The facilitator runs: Brier, climatology of the pool (0.50 — the pool is balanced by construction), skill, ECE in 5 bins of equal mass (adjusted gate), observed curve, and Murphy decomposition.

## Limits declared of this list

- **The expert is the author of the program**, as in the two previous domains: panel of 1 of 3, without the outside control.
- **Same base**: `h2/educacao_star` and `h2g/educacao_star_turmas` share the base `AER_STAR`; the panel cannot have the same person on both sides of the pair, and that holds for when the panel exists — with a single expert, it stays declared.
- **No item touches the elicited domain**: nothing of class size in the USA, nothing of STAR, Toronto, Rhode Island, or LaLonde/NSW.
