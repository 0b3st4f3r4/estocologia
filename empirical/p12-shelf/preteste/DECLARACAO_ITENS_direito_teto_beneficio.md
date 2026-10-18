# Declaration of items of the pre-test — `h2g/direito_teto_beneficio`

**Frozen on 2026-10-16**, before the expert answered anything.

| what | value |
| --- | --- |
| pool of origin | `empirical/p12-shelf/preteste/direito_teto_beneficio.yaml`, 30 items (17 `sim` and 13 `nao`), 16 discarded, SHA-256 `965faff6503d0a6348851fefd7a5f124e18bf4917af41d449ca74cf6e719db37` |
| seed of the draw | `2026` |
| rule of the draw | **stratified**: shuffles each stratum of resolution with the `SplitMix64` of the repository and takes 10 from each. The checked ones gave **exactly 10 `sim` and 10 `nao`**, so there was no discarding for imbalance |
| answer key | it stays with the facilitator; the expert sees **only the statements** of this declaration |

## The filter of the answer key, declared

Of the 30 items of the pool, **20 were checked by the facilitator** and they are the ones that enter: all 14 of Eurostat, the 2 of IBGE/SIDRA, the 2 of the BLS API v1, the 1 of the World Bank, and the one of the ILO page.

- **exact equality (19 items)**: the value was read by machine in the API and compared with the declared one — Eurostat (14: T09 to T21 and T24), IBGE/SIDRA (2: T26 and T27, series 202312 = 39.2 and 202412 = 38.6), BLS API v1 (2: T22 = 0.42 in `CMU2270000000000D` and T23 = 0.44 in `CMU1270000000000D`, fourth quarter of 2024) and World Bank (1: T28 = 27.0083134719641).
- **presence on the cited page (1 item)**: T30, the 47.6% of the ILO, located in the text of the page `world-social-protection-report-2024-26-figures`.

**The 10 items that I did not reproduce stayed out of the draw**, and do not enter on trust in the executor: T01 to T08 (the eight pages of the BLS, which without the complete set of browser headers answer with a block), T25 (the page of the SSA, same case), and T29 (the PDF of the ILO, which downloaded with 37 KB and does not carry the text of the report). It stays declared, and not hidden.

## The 20 items, in the order in which the expert sees them

| # | id | statement | declared difficulty |
| --- | --- | --- | --- |
| 1 | T11 | In the base year 2022, was the number of fatal work accidents in France greater than 700? | difficult |
| 2 | T23 | In the base year 2024, was the cost of work accident insurance per hour worked among the civilian workers of the United States, in the quarter ended in December 2024, greater than US$ 0.40? | difficult |
| 3 | T13 | In the base year 2022, was the number of fatal work accidents in Spain greater than 350? | medium |
| 4 | T15 | In the base year 2022, was the rate of fatal work accidents per 100 000 workers in France greater than 3.0? | difficult |
| 5 | T30 | In the base year 2023, was the proportion of the world population without any social protection greater than 55%? | medium |
| 6 | T14 | In the base year 2022, was the number of fatal work accidents in the Netherlands greater than 50? | easy |
| 7 | T19 | In the base year 2022, was the number of non-fatal work accidents with at least 4 days of absence in Italy greater than 300 000? | difficult |
| 8 | T21 | In the base year 2022, was the number of work accidents in the European Union (27 countries) that resulted in permanent incapacity or absence of 183 days or more greater than 150 000? | difficult |
| 9 | T28 | In the base year 2023, was vulnerable employment as a proportion of total employment in Brazil greater than 35%? | medium |
| 10 | T12 | In the base year 2024, was the number of fatal work accidents in France greater than 1 000? | difficult |
| 11 | T10 | In the base year 2022, was the number of fatal work accidents in Germany greater than 350? | medium |
| 12 | T27 | In the base year 2024, was the informality rate of people aged 14 or over in employment in Brazil, in the moving quarter ended in December 2024, greater than 45%? | medium |
| 13 | T22 | In the base year 2024, was the cost of work accident insurance per hour worked in the private industry of the United States, in the quarter ended in December 2024, greater than US$ 0.50? | medium |
| 14 | T09 | In the base year 2023, was the number of fatal work accidents in Germany greater than 500? | medium |
| 15 | T24 | In the base year 2024, was the proportion of employees aged 16 to 64 in the European Union (27 countries) entitled to sickness benefit greater than 80%? | medium |
| 16 | T26 | In the base year 2023, was the informality rate of people aged 14 or over in employment in Brazil, in the moving quarter ended in December 2023, greater than 35%? | easy |
| 17 | T18 | In the base year 2022, was the number of non-fatal work accidents with at least 4 days of absence in Germany greater than 1 million? | medium |
| 18 | T17 | In the base year 2022, was the number of non-fatal work accidents with at least 4 days of absence in the European Union (27 countries) greater than 2.5 million? | easy |
| 19 | T16 | In the base year 2022, was the rate of fatal work accidents per 100 000 workers in Germany greater than 1.0? | medium |
| 20 | T20 | In the base year 2022, was the number of work accidents in the European Union (27 countries) that resulted in an absence of 3 to 6 months greater than 150 000? | difficult |

## What the expert receives and what he returns

- He receives: the 20 statements and the request for `p in (0, 1)` for each one, from memory and without consultation.
- He returns: 20 probabilities.
- The facilitator runs: Brier, climatology of the pool (0.50 — balanced by construction), skill, ECE in 5 bins of equal mass, observed curve, and Murphy decomposition, with **criterion 3 by the binomial test** (declaration 3).

## Limits declared of this list

- **The expert is the author of the program**: panel of 1 of 3, without the outside control, as in the three previous domains.
- **The pair of the same base of this domain is `h1-real/b`** (NSW/LaLonde), which E1 did not serve as a SHELF expert — for that reason this domain is eligible for him, contrary to `h2g/educacao_star_turmas`.
- **No item touches the elicited domain**: nothing of elasticity of the duration with respect to the benefit, nothing of the benefit ceiling, nothing of Kentucky/Michigan of the 1980s.
