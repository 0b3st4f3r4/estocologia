# Declaration of items of the pre-test — `h1-real/c`

**Frozen on 2026-10-16**, before the expert answered anything.

| what | value |
| --- | --- |
| pool of origin | `empirical/p12-shelf/preteste/h1_real_c.yaml`, 30 items (15 `sim` and 15 `nao`), 10 discarded, SHA-256 `fb5a8d87a206546427edf2b62c0ec68a53f15349c6374c0e13115c058bf7046f` |
| seed of the draw | `2026` |
| rule of the draw | **stratified**: shuffles each stratum of resolution with the `SplitMix64` of the repository and takes 10 from each. The pool is 15 `sim` × 15 `nao`, so there was no discarding for imbalance |
| answer key | it stays with the facilitator; the expert sees **only the statements** of this declaration |

## The filter of the answer key: **30 of 30 checked**, the best of the five domains

The facilitator downloaded the **ten sources** on his own (four PDFs read with `pdftotext` and six pages converted to text) and located the declared number of each item. Result: **30 of 30**, no item excluded.

- **24** appeared in the first pass, with the passage checked in view (Swiss Re sigma 1/2025, sigma 1/2024, and sigma 3/2024; Munich Re 2023 and 2024; NOAA/NCEI 2023 and 2024; EIOPA; OECD).
- **6** only appeared after correcting the **reading of the separator**: the pool writes the numbers in Portuguese (`74.000`, `31,9`, `182,7`, `1.543,2`, `96,2`, `6,2`) and the sources in English (`74,000`, `31.9`, `182.7`, `1,543.2`, `96.2`, `6.2 billion`). Located the six, with the passage checked.

The passages that anchor the fragile items stay recorded: `more than 74,000 fatalities` (C05), the table of III with `2016 $31.9 $43.1` and `2017 96.2 127.2` (C08 and C17), `approximately $182.7 billion` (C14), `$1,543.2 billion` in cyclones (C16), and `insured losses of USD 6.2 billion` (C23).

## The 20 items, in the order in which the expert sees them

| # | id | statement | declared difficulty |
| --- | --- | --- | --- |
| 1 | C23 | In the base year 2023, were the insured losses of the earthquake in Turkey and Syria greater than 20 billion dollars? | difficult |
| 2 | C27 | In the base year 2024, considering the series from 1980 to 2024, were more than 70% of the economic losses from natural catastrophes in Europe covered by insurance? | difficult |
| 3 | C01 | In the base year 2024, were the insured losses from natural catastrophes in the world greater than 100 billion dollars? | easy |
| 4 | C28 | In the base year 2024, considering the series from 1980 to 2024, did the earthquake in Italy account for more than 30% of the uninsured losses from natural catastrophes in the European economic space? | difficult |
| 5 | C15 | In the base year 2024, was the number of billion-dollar disasters catalogued by NOAA in the United States greater than the number recorded in 2023? | medium |
| 6 | C29 | In the base year 2023, was the penetration of non-life insurance in the Netherlands higher than 6% of GDP? | difficult |
| 7 | C19 | In the base year 2023, was insurance penetration in the world, measured as direct premiums as a percentage of GDP, higher than 6%? | medium |
| 8 | C14 | In the base year 2024, was the total cost of the billion-dollar disasters catalogued by NOAA in the United States greater than 200 billion dollars? | medium |
| 9 | C20 | In the base year 2023, was insurance penetration in Mexico higher than 5% of GDP? | medium |
| 10 | C08 | In the base year 2016, were the insured losses from natural catastrophes in the United States greater than 50 billion dollars? | easy |
| 11 | C22 | In the base year 2023, were the insured losses from severe convective storms in the world greater than 50 billion dollars? | difficult |
| 12 | C21 | In the base year 2023, was the number of natural catastrophes that generated insured losses in the world greater than 130? | difficult |
| 13 | C13 | In the base year 2024, did weather catastrophes account for more than 90% of the insured losses from natural catastrophes in the world? | medium |
| 14 | C12 | In the base year 2024, were the insured losses from secondary perils (floods, wildfires, and severe storms) in the world greater than those recorded in 2023? | medium |
| 15 | C04 | In the base year 2023, were the economic losses from natural catastrophes in the world greater than 200 billion dollars? | easy |
| 16 | C26 | In the base year 2024, was the number of billion-dollar disasters that occurred in the United States in the decade from 2015 to 2024 greater than 150? | difficult |
| 17 | C16 | In the base year 2024, was the accumulated cost of the billion-dollar disasters caused by tropical cyclones in the United States since 1980 greater than 1 trillion dollars? | medium |
| 18 | C06 | In the base year 2023, were the insured losses from natural catastrophes in the world, in the count of Munich Re, greater than 80 billion dollars? | easy |
| 19 | C03 | In the base year 2024, were more than 70% of the economic losses from natural catastrophes in the world left without insurance coverage? | easy |
| 20 | C30 | In the base year 2023, was the penetration of non-life insurance in most of the jurisdictions that report to the OECD higher than 8% of GDP? | difficult |

## What the expert receives and what he returns

- He receives: the 20 statements and the request for `p in (0, 1)` for each one, from memory and without consultation.
- He returns: 20 probabilities.
- The facilitator runs: Brier, climatology of the pool (0.50), skill, ECE in 5 bins of equal mass, observed curve, Murphy decomposition, and criterion 3 by the **binomial test** (declaration 3).

## Limits declared

- **The expert is the author of the program**: panel of 1 of 3. This is the **last domain of his queue**; `h2g/educacao_star_turmas` stays reserved for a second expert, by the exclusion of the pair of the same base.
- **The quantity of this domain is a frequency in (0, 1)**, and for that reason the adjustment of the prior is **Beta** — the instrument entered in code on 2026-10-16 (`stochology-horizons::beta`), with the weak prior of p05 (Beta(1; 1)) recovered exactly in the test.
- **No item touches the domain**: nothing of the Danish base of fires, nothing of the scheme "100 xs 50", nothing of the reinsurance layer of that scheme.
