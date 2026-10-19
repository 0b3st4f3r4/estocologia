# Real data of Stochology (H0 and H1 in a real domain)

Downloaded on 2026-10-14 through the public Rdatasets mirror (raw.githubusercontent.com/vincentarelbundock/Rdatasets)
and the CRAN mirror on GitHub (raw.githubusercontent.com/cran/...). The official portals (DATASUS, dados.gov.br,
FRED, UCI, Kaggle) are blocked by the network of this environment. SHA-256 sums in `SHA256SUMS`.

| File | Source | What it is | Use |
|---|---|---|---|
| bank_calls.csv | Rdatasets `fpp3::bank_calls` | Calls every 5 min at the center of a large North American commercial bank, 3 Mar–24 Oct 2003, business days, 07:00–21:05 (164 days × 169 intervals) | H0 |
| djia_1885_2023.csv | Rdatasets `stevedata::DJIA` | Daily close of the Dow Jones Industrial Average, 1885-02-16 to 2023-11-21 (37 931 trading days; without dividends) | H1·A |
| crsp_day_1989_1998.csv *(outside the tree)* | Rdatasets `Ecdat::CRSPday` | Daily returns of GE, IBM, Mobil and of the CRSP index, 1989–1998 (declared outside the tree in `SHA256SUMS`; not consumed by the instruments in force) | H1·A (cross-check) |
| nsw_experimental.csv | Rdatasets `causaldata::nsw_mixtape` | NSW (LaLonde 1986), Dehejia–Wahba sample: 185 treated and 260 randomized controls | H1·B |
| cps_observational.csv | Rdatasets `causaldata::cps_mixtape` | Observational controls from the CPS (15 992) used against the NSW treated | H1·B |
| danish_fire.csv | CRAN `fitdistrplus::danishuni` (GitHub mirror) | Fire losses of Copenhagen Re above 1 M DKK, with date, 1980-01-03 to 1990-12-31 (2 167 claims, in millions of DKK, adjusted for inflation to 1985) | H1·C |

Classic references (cited from memory, to be checked): Shen & Huang (2008) for the call series;
LaLonde (1986) and Dehejia & Wahba (1999) for the NSW; McNeil (1997) and Embrechts, Klüppelberg & Mikosch (1997)
for the Danish losses.

## Added on 2026-10-19: the real streams of Chapter 20

Downloaded on 2026-10-19 by a sparse `git clone` of the Rdatasets repository (github.com/vincentarelbundock/Rdatasets,
commit `1dcc2bf5f955cc1224a3e1307256e1fe86b68dae`) and of the CRAN mirror of `nycflights23` (github.com/cran/nycflights23,
commit `48bca3116609b15fee8be2eb71b3f16383ca3ec6`, version 0.2.0, licence CC0); `raw.githubusercontent.com` and PyPI were not
reachable from the session, so the `.rda` file of `nycflights23` was read by a minimal reader of R's XDR serialization
written for this purpose and kept outside the tree. Read by `labs --streams` (`crates/stochology-book/src/streams.rs`).

| File | Source | What it is | Transformation |
|---|---|---|---|
| nyc_flights_2013.csv | Rdatasets `nycflights13::flights` | Departures from New York (EWR, JFK, LGA), 2013 | rows of August to December; columns month, day, sched_dep_time, sched_arr_time, carrier, origin, dest, distance, arr_delay |
| nyc_flights_2023.csv | CRAN `nycflights23::flights` | The same for 2023 (435 352 flights in the year) | the same rows and columns |
| vic_elec.csv | Rdatasets `tsibbledata::vic_elec` | Half-hourly electricity demand of Victoria, Australia, 2012–2014, with temperature and holidays | none |
| atlantic_storms.csv | Rdatasets `dplyr::storms` | NOAA best-track positions and winds of Atlantic storms, every 6 hours, 1975–2024 | none |
| toronto_arrests.csv | Rdatasets `carData::Arrests` | Arrests for simple possession of marijuana in Toronto, 1997–2002 (5 226), with release, perceived colour and covariates | none |

What could not be brought: the Toronto arrests of 2020–2021 and the Rhode Island traffic stops (portals not reachable
from the session), the causal chambers, the FDA cardiac trial and the yeast deletions of Chapter 19 (hosted outside the
reachable mirrors), and the STAR experiment with school identifiers (`AER::STAR` has no school identifier).

## Added on 2026-10-19, later: fresh flights for point 24

For the resonance claim of point 24, which needs flights no reading of this programme had used. The first seven months
of 2013 and of 2023 come from the same sources as the files above (Rdatasets `nycflights13::flights` at commit
`1dcc2bf5`, CRAN `nycflights23` at commit `48bca311`), rows that were cut out when those files were written and never
entered the tree. Houston comes from the CRAN mirror of `hflights` (github.com/cran/hflights, commit
`3f5a566f8b294da40e0abac2cfab4f7926eeb89d`, version 0.1, licence CC0), read by the same XDR reader kept outside the
tree. The conversion wrote rows and computed no statistic of the outcome. Since the wave engine left the tree later the
same day (`docs/declarations/REMOVED_WAVE.md`), no instrument reads these three files; they stay as the sealed data of
point 24.

| File | Source | What it is | Transformation |
|---|---|---|---|
| nyc_flights_2013_h1.csv | Rdatasets `nycflights13::flights` | Departures from New York, January to July 2013 (195 583 rows) | the columns of nyc_flights_2013.csv |
| nyc_flights_2023_h1.csv | CRAN `nycflights23::flights` | The same for January to July 2023 (258 613 rows) | the same columns |
| houston_flights_2011.csv | CRAN `hflights::hflights` | Departures from Houston (IAH, HOU), 2011 (224 591 rows with a departure time) | the same columns; scheduled departure and arrival recovered as the actual time minus its delay (the package has no scheduled time); flights with no departure time dropped (cancelled before departure, with no arrival delay either) |

## Licences, declared on 2026-10-19

The files in this folder are copies of datasets redistributed by R packages. The licence of a package covers its code
and documentation; whether it also covers the data it redistributes, and under what terms the original collector
released them, is not settled by the package field alone. The table gives both: the licence field of the package of
origin, read from its `DESCRIPTION` on the CRAN mirror on GitHub on 2026-10-19, and the original source of the data as
the package documents it. The repository's own MIT licence covers the code of this tree, not these files. Whoever
reuses a file should go by its original source.

| File | Package (licence field) | Original source as documented by the package |
|---|---|---|
| bank_calls.csv | `fpp3` (GPL-3) | Weinberg, Brown & Stroud (2007), call-centre data of a North American bank, as used by Shen & Huang (2008) |
| djia_1885_2023.csv | `stevedata` (GPL-2) | daily closes of the Dow Jones Industrial Average, compiled by the package author from public quotes |
| nsw_experimental.csv, cps_observational.csv | `causaldata` (MIT) | LaLonde (1986) and Dehejia & Wahba (1999), public research data |
| danish_fire.csv | `fitdistrplus` (GPL ≥ 2) | Copenhagen Re fire losses, made public by McNeil (1997) |
| nyc_flights_2013.csv | `nycflights13` (CC0) | US Bureau of Transportation Statistics, public records |
| nyc_flights_2023.csv | `nycflights23` (CC0) | US Bureau of Transportation Statistics, public records |
| nyc_flights_2013_h1.csv, nyc_flights_2023_h1.csv | `nycflights13`, `nycflights23` (CC0) | US Bureau of Transportation Statistics, public records |
| houston_flights_2011.csv | `hflights` (CC0) | US Bureau of Transportation Statistics, public records |
| vic_elec.csv | `tsibbledata` (GPL-3) | Australian Energy Market Operator, public half-hourly demand |
| atlantic_storms.csv | `dplyr` (MIT) | NOAA Atlantic hurricane database (HURDAT2), public records |
| toronto_arrests.csv | `carData` (GPL ≥ 2) | Toronto Star freedom-of-information request, as published by Fox & Weisberg |

The GPL packages are not linked, imported or executed by anything in this tree; only their data files were copied, and
the files are not derived works of the package code. The question of the data's own terms stays open for the files
whose original collector states none (the bank calls, the Dow Jones compilation and the Toronto arrests), and is
declared here rather than resolved.

