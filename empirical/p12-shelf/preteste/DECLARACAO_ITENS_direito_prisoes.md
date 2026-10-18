# Declaration of items of the pre-test — \`h2/direito_prisoes\`

**Frozen on 2026-10-16**, before the expert answered anything. It is the "list with hash" that the
protocol (section 2.1) requires, per domain.

| what | value |
| --- | --- |
| pool of origin | `empirical/p12-shelf/preteste/direito_prisoes.yaml`, 38 items (21 `sim` and 17 `nao`), 15 discarded, SHA-256 `080ef700cba064e7c8d5fe11d6d002b4b82b3584773e52821e08cd61a7e84686` |
| seed of the draw | `2026` |
| rule of the draw | **stratified**: shuffles each stratum of resolution with the `SplitMix64` of the repository and takes 10 items from each, then shuffles the 20 for the order of presentation |
| stayed out of the stratum `nao` | `D25` (the stratum had 11 checked items and the design asks for 10 of each) |
| answer key | it stays with the facilitator; the expert sees **only the statements** of this declaration |

## Declaration 2 of the domain — the design of the draw, and why it changed after seeing the pool

The protocol (section 2.1) speaks of a draw with a declared seed, and does not fix stratification. The first pool of this
domain came out with **21 items `sim` against 3 `nao`** — base rate 0.875 —, and that was **seen** before any
administration. With that base rate the climatology of the pool itself would go to 0.875 and whoever answered 0.9 to everything
would end up with skill near zero: the pre-test would measure almost nothing.

Declared, then: the pool was rebalanced (14 items of resolution `nao` were added, ids D25 to D38, without
touching the first 24) and the draw becomes **stratified, 10 `sim` and 10 `nao`**. This is a change of design
**after seeing a result of the instrument itself**, so it counts: the K of the front goes from **3** to **4**.

## The filter of the answer key, declared

The 20 items below did not come from the whole pool: they came from the **21 items whose answer key the facilitator reproduced in the
primary source** (10 `sim` and 11 `nao`), with a real request and reading of the number on the page or in the downloaded PDF
(`pdftotext` when it is a PDF; browser headers when the BJS served a blocking stub). An item whose answer key the
facilitator **could not** reproduce stayed out of the draw, and does not enter on trust in the executor. The evidence
item by item:

| item checked | what the primary source says, in the letter |
| --- | --- |
| D05 | PPI 2024: 'The U.S. incarcerates over 1.9 million people' |
| D06 | Eurostat: 'around 509 000 [prisoners] in 2024' |
| D07 | INEGI: 'había 233 277 personas privadas de la libertad' |
| D09 | UNODC: '11.2 millones de personas privadas de libertad' |
| D13 | Eurostat: '113 prisoners per 100 000 inhabitants in the EU in 2024' |
| D15 | INEGI: '37.3 % ... no contó con una sentencia' |
| D16 | UNODC: '142 personas privadas de libertad por cada 100,000 habitantes' |
| D18 | Sentencing Project: 'In 12 states, more than half the prison population is Black' |
| D21 | Eurostat occupancy: 'highest overcrowding EU rates in 2024 ... Cyprus (227.6)' |
| D23 | INEGI: 'para las mujeres este porcentaje fue de 46.9 %' |
| D20 | Eurostat: 'detainees with a foreign citizenship (21.4%) ... than in 2023 (20.7%)' |
| D25 | BJS ji22st: 'local jails spent an average of 32 days in custody' |
| D26 | BJS p22st: '62 per 100,000 female adult U.S. residents were serving' |
| D27 | BJS cpus22st: '5,407,300 ... (1,827,600) were incarcerated in state or federal' |
| D28 | Eurostat: '1 out of 18 adult prisoners were women' (5.5%) |
| D30 | BJS ji22st, table: 76.7% of the prisoners for felony in 2022 |
| D31 | INEGI: '43 553 993 799.0 pesos' for 233 277 people |
| D32 | BJS fjs22: 'median number of days from the receipt of a matter ... was 60 days' |
| D34 | BJS msfp0119st: 'a rate of 259 per 100,000' |
| D36 | Eurostat occupancy: 'France (129.3)' |
| D38 | INEGI: '100 fallecieron' in incidents in the penitentiary centres |

The other 17 items of the pool (13 old and 4 new) were **not** checked by the facilitator — the executor checked them,
with the URL cited — and for that reason they do not enter this pre-test. It stays declared, and not hidden.

## The 20 items, in the order in which the expert sees them

| # | id | statement | declared difficulty |
| --- | --- | --- | --- |
| 1 | D05 | In the base year 2024, was the total number of incarcerated people in the United States, adding state and federal prisons, local jails, immigrant detention, juvenile units, and the other forms of confinement, greater than 1.5 million? | easy |
| 2 | D32 | In the fiscal base year 2022, was the median time between the receipt of the investigation and the decision of the federal prosecutor's office of the United States to charge or decline the case greater than 90 days? | medium |
| 3 | D27 | In the base year 2022, were more than two thirds of the people under adult correctional supervision in the United States incarcerated in prisons or local jails? | easy |
| 4 | D16 | In the base year 2021, was the world rate of people deprived of liberty lower than 150 per 100 000 inhabitants? | medium |
| 5 | D31 | In the base year 2023, was the annual expenditure per person deprived of liberty in the state and federal penitentiary centres of Mexico greater than 300 000 pesos? | medium |
| 6 | D18 | In the base year 2019, did more than 10 states of the United States have more than half of the state prison population made up of black people? | difficult |
| 7 | D06 | In the base year 2024, was the number of prisoners in the European Union greater than 400 000? | easy |
| 8 | D21 | In the base year 2024, was the country of the European Union with the highest prison occupancy rate above 200% of the official capacity of the prisons? | difficult |
| 9 | D13 | In the base year 2024, was the rate of prisoners per 100 000 inhabitants in the European Union lower than 150? | medium |
| 10 | D28 | In the base year 2024, were women more than 10% of the prisoners of the European Union? | easy |
| 11 | D34 | In the base year 2019, was the mortality rate in the federal prisons operated by the Bureau of Prisons of the United States greater than 300 deaths per 100 000 prisoners? | difficult |
| 12 | D30 | In the base year 2022, were fewer than half of the people held in the local jails of the United States charged with a serious crime (felony)? | medium |
| 13 | D09 | In the base year 2021, was the world population deprived of liberty greater than 10 million people? | easy |
| 14 | D07 | In the base year 2023, was the number of people deprived of liberty in the state and federal penitentiary centres of Mexico greater than 200 000? | easy |
| 15 | D38 | In the base year 2023, did more than 200 people deprived of liberty die in incidents in the penitentiary centres and specialized centres of Mexico? | difficult |
| 16 | D23 | In the base year 2023, were more than 45% of the women deprived of liberty in the state and federal penitentiary centres of Mexico without a sentence? | difficult |
| 17 | D36 | In the base year 2024, did France have a prison occupancy rate above 150% of the official capacity? | difficult |
| 18 | D26 | In the base year 2022, was the incarceration rate of adult women in the United States greater than 100 inmates per 100 000 adult residents? | easy |
| 19 | D15 | In the base year 2023, were more than one third of the people deprived of liberty in the state and federal penitentiary centres of Mexico without a sentence? | medium |
| 20 | D20 | In the base year 2024, were more than 25% of the prisoners of the European Union of foreign nationality in the country where they were detained? | difficult |

## What the expert receives and what he returns

- He receives: the 20 statements, the request for `p in (0, 1)` for each one (exact 0 and 1 are rejected) and the instruction to
  answer from memory, without consultation, in 20 minutes.
- He returns: 20 probabilities.
- What the facilitator does: Brier item by item, climatology of the pool (0.553), skill, ECE in 5 bins of equal
  mass (the adjusted gate of declaration 1), the observed curve, and the Murphy decomposition, with the item-by-item detail
  alongside the aggregates.

## Limits declared of this list

- **The expert of this table is the author of the program**, as in the first domain: he is not from outside the program nor from
  outside the area, the panel has 1 of 3, and no domain closes a consensus like that.
- **The cut-off window of the answer key costs representativeness**: the items whose source I did not reach stayed out
  (ABS, part of the BJS), so that the 20 are honest as to the answer key and are not a random sample of the pool.
- **The pool has 38 items and 15 discarded**, all with a reason in the file itself.
