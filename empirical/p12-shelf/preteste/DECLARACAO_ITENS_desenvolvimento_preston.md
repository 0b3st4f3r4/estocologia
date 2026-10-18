# Declaration of items of the pre-test — `h2/desenvolvimento_preston`

**Frozen on 2026-10-16**, before the expert answered anything. It is the "list with hash" that the
protocol (section 2.1) requires, per domain.

| what | value |
| --- | --- |
| pool of origin | `empirical/p12-shelf/preteste/desenvolvimento_preston.yaml`, 22 items, SHA-256 `e4bd218d16fc50064c2f10e55221af8b3321e8926d3857f5e239edc5a69cffbb` |
| rectification | the hash of the pool changed after the first freezing (the executor added the detail of the discarded ones); the list of the 20 drawn and the order did **not** change, and the rectification is in the `DECLARACAO.md`, declaration 2 |
| seed of the draw | `2026` |
| rule of the draw | Fisher-Yates shuffle over the 22 identifiers, in the order of the file, with the `SplitMix64` of the repository itself (`crates/stochology-core/src/rng.rs`); the first 20 enter |
| discarded | `P03`, `P16` |
| answer key | it stays in the file of the pool, item by item, with the URL of the source and the number; the expert sees **only the statements** of this declaration, and the facilitator keeps the answer key |

Rule of the draw, for whoever wants to reproduce it:

~~~python
MASK = (1 << 64) - 1
def proximo(est):
    est = (est + 0x9E3779B97F4A7C15) & MASK
    z = est
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return est, (z ^ (z >> 31)) & MASK
# shuffles with seed 2026 and takes the first 20
~~~

## The 20 items, in the order in which the expert sees them

| # | id | statement | declared difficulty |
| --- | --- | --- | --- |
| 1 | P05 | In the base year 2019, was the life expectancy at birth of Nigeria lower than 55 years? | easy |
| 2 | P01 | In the base year 2019, was the life expectancy at birth of Japan higher than 80 years? | easy |
| 3 | P06 | In the base year 2019, was the under-five mortality rate of India higher than 40 deaths per thousand live births? | easy |
| 4 | P18 | In the base year 2019, was the share of the population of Nigeria with at least basic drinking water services higher than 80%? | difficult |
| 5 | P10 | In the base year 2019, was the life expectancy at birth of Russia higher than 75 years? | medium |
| 6 | P07 | In the base year 2019, was the life expectancy at birth of Brazil higher than 75 years? | medium |
| 7 | P22 | In the base year 2019, was total health expenditure as a percentage of GDP in South Africa higher than 10%? | difficult |
| 8 | P09 | In the base year 2020, was the fall in life expectancy at birth of the United States relative to 2019 greater than 1 year? | medium |
| 9 | P17 | In the base year 2019, was the under-five mortality rate of Sierra Leone higher than 100 deaths per thousand live births? | difficult |
| 10 | P11 | In the base year 2019, was the life expectancy at birth of Bangladesh higher than 70 years? | medium |
| 11 | P21 | In the base year 2021, was world life expectancy at birth lower than 72 years? | difficult |
| 12 | P02 | In the base year 2019, was the GDP per capita in purchasing power parity of China higher than 20 000 international dollars? | easy |
| 13 | P14 | In the base year 2019, was the GDP per capita at current prices of Brazil higher than 10 000 dollars? | medium |
| 14 | P13 | In the base year 2019, was the GDP per capita in purchasing power parity of India lower than 9 000 international dollars? | medium |
| 15 | P08 | In the base year 2019, was the life expectancy at birth of India higher than 72 years? | medium |
| 16 | P20 | In the base year 2019, was the difference between the life expectancy of women and that of men in Russia greater than 11 years? | difficult |
| 17 | P19 | In the base year 2021, was the life expectancy at birth of India lower than 68 years? | difficult |
| 18 | P04 | In the base year 2019, was the share of the population aged 65 or over in Japan higher than 30%? | easy |
| 19 | P12 | In the base year 2019, was total health expenditure as a percentage of GDP in Brazil higher than 10%? | medium |
| 20 | P15 | In the base year 2019, was the GDP per capita in purchasing power parity of Qatar higher than 100 000 international dollars? | difficult |

## What the expert receives and what he returns

- He receives: the 20 statements above, the request for `p in (0, 1)` for each one (exact 0 and 1 are rejected, because without a
  prediction there is no Brier) and the instruction to answer from memory, without consultation.
- He returns: 20 probabilities. Nothing more.
- What the facilitator does with that: Brier item by item, Brier of the climatology of the pool itself, skill, ECE in
  the 5 bins of equal mass (the gate adjusted in the `DECLARACAO.md`), the observed curve, and the Murphy decomposition,
  with the item-by-item detail alongside the aggregates.

## Limits declared of this list

- **The pool is not from the vault of the domain, but it is on the same subject.** The items are facts of development, population
  health, and income; none of them leaves the vault of `h2/desenvolvimento_preston`, and none measures the elicited
  quantity (elasticity of life expectancy with respect to the log of GDP per capita).
- **One item uses 2021 and another compares 2020 with 2019.** They are published and stable data, and the resolution does not change
  with a subsequent revision of the series.
- **The expert of this table is the author of the program.** The protocol (section 3) requires, in the panel, an expert
  **from outside the program** and **from outside the area**; the author is neither one thing nor the other, and that stays declared:
  his pre-test measures the calibration of whoever wrote the book, not of whoever did not write it. The panel continues
  incomplete, and no domain closes a consensus with a single expert (section 2.4).
