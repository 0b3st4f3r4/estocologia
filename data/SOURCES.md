# Empirical Datasets of Stochology (`data/*.csv`)

Downloaded on 2026-10-14 and 2026-10-19 through the public Rdatasets mirror (`github.com/vincentarelbundock/Rdatasets`, commit `1dcc2bf5f955cc1224a3e1307256e1fe86b68dae`) and the CRAN mirror on GitHub (`github.com/cran/...`). Cryptographic SHA-256 digests for all 13 in-tree CSV files are recorded in `data/SHA256SUMS` and verified at runtime by `stochology-cli` (`gates` and `labs`).

## 1. Core Benchmark Datasets (`data/*.csv`)

| File | Source | Description | Protocol Usage |
|---|---|---|---|
| `bank_calls.csv` | Rdatasets `fpp3::bank_calls` | Arrivals every 5 min at the call center of a large North American commercial bank, 3 Mar–24 Oct 2003, business days, 07:00–21:05 (164 days × 169 intervals = 27 716 rows) | Bridge $L_2 \to L_3$ (Jensen's Gap & OOS CRPS), $\Phi_2$ Moulton-Campbell/Exceedance DEFF, and `cards/card_02_bank_queue_staffing.yaml` |
| `djia_1885_2023.csv` | Rdatasets `stevedata::DJIA` | Daily close of the Dow Jones Industrial Average, 1885-02-16 to 2023-11-21 (37 931 trading days; without dividends) | Lens 2 (Time, Itô variance drain, asymmetric Kelly fraction) |
| `nsw_experimental.csv` | Rdatasets `causaldata::nsw_mixtape` | National Supported Work (NSW) randomized job-training experiment (LaLonde 1986; Dehejia–Wahba sample): 185 treated and 260 randomized controls | Bridge $L_1 \to L_2$ and $\Phi_3$ (Meng's Identity & Manski MNAR bounds) |
| `cps_observational.csv` | Rdatasets `causaldata::cps_mixtape` | Observational controls from the Current Population Survey (15 992 rows) compared against the NSW experimental arm | Bridge $L_1 \to L_2$ and $\Phi_3$ (Meng's selection bias audit) |
| `danish_fire.csv` | CRAN `fitdistrplus::danishuni` | Fire insurance claims of Copenhagen Re above 1M DKK, 1980-01-03 to 1990-12-31 (2 167 claims, in millions of 1985 DKK) | Lens 3 (Scale and Tail, Hill estimator $\hat{\alpha}_{k,n}$ and moment existence regime) |

### Verified Primary References
- **Call-Center Arrivals (`bank_calls.csv`):**
  - Weinberg, J., Brown, L. D., & Stroud, J. R. (2007). Bayesian forecasting of an inhomogeneous Poisson process with applications to call center data. *Journal of the American Statistical Association*, 102(480), 1185–1198.
  - Shen, H., & Huang, J. Z. (2008). Interday forecasting and intraday updating of call center arrivals. *Manufacturing & Service Operations Management*, 10(3), 391–410.
- **National Supported Work & CPS Comparison (`nsw_experimental.csv`, `cps_observational.csv`):**
  - LaLonde, R. J. (1986). Evaluating the econometric evaluations of training programs with experimental data. *The American Economic Review*, 76(4), 604–620.
  - Dehejia, R. H., & Wahba, S. (1999). Causal effects in nonexperimental studies: Reevaluating the evaluation of training programs. *Journal of the American Statistical Association*, 94(448), 1053–1062.
- **Danish Fire Reinsurance Losses (`danish_fire.csv`):**
  - McNeil, A. J. (1997). Estimating the tails of loss severity distributions using extreme value theory. *ASTIN Bulletin: The Journal of the IAA*, 27(1), 117–137.
  - Embrechts, P., Klüppelberg, C., & Mikosch, T. (1997). *Modelling Extremal Events for Insurance and Finance*. Springer-Verlag, Berlin.

---

## 2. Operational Streams, Calibration, and Fairness Datasets

Downloaded on 2026-10-19 from Rdatasets (commit `1dcc2bf5f955cc1224a3e1307256e1fe86b68dae`) and the CRAN mirror of `nycflights23` (commit `48bca3116609b15fee8be2eb71b3f16383ca3ec6`, version 0.2.0, licence CC0). Consumed by `crates/stochology-cli/src/lib.rs` (`labs` and `gates`).

| File | Source | Description | Transformation | Protocol Usage |
|---|---|---|---|---|
| `nyc_flights_2013.csv` | Rdatasets `nycflights13::flights` | Departures from New York (EWR, JFK, LGA), Aug–Dec 2013 | Columns: `month, day, sched_dep_time, sched_arr_time, carrier, origin, dest, distance, arr_delay` | Covariate & Label Shift source (Lab 6, Lab 7) and `cards/card_03_nyc_flights_holding.yaml` |
| `nyc_flights_2023.csv` | CRAN `nycflights23::flights` | Departures from New York (EWR, JFK, LGA), Aug–Dec 2023 | Same columns and calendar window | Covariate & Label Shift target (Lab 6, Lab 7) |
| `vic_elec.csv` | Rdatasets `tsibbledata::vic_elec` | Half-hourly electricity demand of Victoria, Australia, 2012–2014 (52 608 rows), with temperature and holidays | None | Lens 3 (Scale and Tail, Lab 3 & Lab 7) |
| `atlantic_storms.csv` | Rdatasets `dplyr::storms` | NOAA HURDAT2 best-track positions, central pressures, and maximum sustained winds of Atlantic storms, every 6 hours, 1975–2024 (20 778 rows, 693 storms) | None | Lens 5 (CORP-PAV isotonic calibration, Lab 3 & Lab 7) and `cards/card_01_atlantic_storms_evacuation.yaml` |
| `toronto_arrests.csv` | Rdatasets `carData::Arrests` | Arrests for simple possession of marijuana in Toronto, 1997–2002 (5 226 rows), with release decision, perceived colour, and covariates | None | $\Phi_4$ Chouldechova algebraic impossibility audit (Lab 4, Lab 7, and `gates --defect chouldechova`) |

---

## 3. Out-of-Time (OOT) and Out-of-Domain (OOD) Flight Reserves

Extracted from the January–July slices of `nycflights13::flights` (commit `1dcc2bf5`) and `nycflights23::flights` (commit `48bca311`), plus `hflights::hflights` (commit `3f5a566f8b294da40e0abac2cfab4f7926eeb89d`, version 0.1, licence CC0). Consumed by `load_oot_ood_flight_reserves` in `crates/stochology-cli/src/lib.rs` (`gates` G7 and `labs --real-data`).

| File | Source | Description | Transformation |
|---|---|---|---|
| `nyc_flights_2013_h1.csv` | Rdatasets `nycflights13::flights` | Departures from New York, January to July 2013 (195 583 rows) | Same columns as `nyc_flights_2013.csv` |
| `nyc_flights_2023_h1.csv` | CRAN `nycflights23::flights` | Departures from New York, January to July 2023 (258 613 rows) | Same columns; sealed in `TemporalVault` for out-of-time evaluation |
| `houston_flights_2011.csv` | CRAN `hflights::hflights` | Departures from Houston (IAH, HOU), 2011 (224 591 rows with a departure time) | Same columns; scheduled departure and arrival recovered as actual time minus delay; cancelled flights without departure time excluded |

---

## 4. Upstream Package Licences and Data Provenance

The files in this directory are copies of public research and government datasets redistributed by R packages. The repository's own MIT licence covers the Rust source code of this workspace.

| File | Package (licence field) | Original source as documented by the package |
|---|---|---|
| `bank_calls.csv` | `fpp3` (GPL-3) | Weinberg, Brown & Stroud (2007), call-centre data of a North American bank, as used by Shen & Huang (2008) |
| `djia_1885_2023.csv` | `stevedata` (GPL-2) | Daily closes of the Dow Jones Industrial Average, compiled from public market quotes |
| `nsw_experimental.csv`, `cps_observational.csv` | `causaldata` (MIT) | LaLonde (1986) and Dehejia & Wahba (1999), public research data |
| `danish_fire.csv` | `fitdistrplus` (GPL ≥ 2) | Copenhagen Re fire losses, published by McNeil (1997) |
| `nyc_flights_2013.csv`, `nyc_flights_2013_h1.csv` | `nycflights13` (CC0) | US Bureau of Transportation Statistics, public records |
| `nyc_flights_2023.csv`, `nyc_flights_2023_h1.csv` | `nycflights23` (CC0) | US Bureau of Transportation Statistics, public records |
| `houston_flights_2011.csv` | `hflights` (CC0) | US Bureau of Transportation Statistics, public records |
| `vic_elec.csv` | `tsibbledata` (GPL-3) | Australian Energy Market Operator (AEMO), public half-hourly demand |
| `atlantic_storms.csv` | `dplyr` (MIT) | NOAA Atlantic hurricane database (HURDAT2), public records |
| `toronto_arrests.csv` | `carData` (GPL ≥ 2) | Toronto Star freedom-of-information request, published by Fox & Weisberg |
