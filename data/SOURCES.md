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
