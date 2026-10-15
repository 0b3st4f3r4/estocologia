# Dados do H3b

Baixados em 2026-10-14 do espelho CRAN no GitHub (raw.githubusercontent.com/cran/nycflights23/master/data/). Somas
SHA-256 em `SHA256SUMS`.

| arquivo | o que é | uso |
|---|---|---|
| nycflights23_flights.rda / .csv | todos os voos partindo de JFK, LGA e EWR em 2023 (435 352) | cofre novo do fluxo voos (H3b) |
| nycflights23_weather.rda / .csv | tempo meteorológico horário nos 3 aeroportos em 2023 (26 207; 3 horas ambíguas da troca de horário descartadas) | idem |

Conversão: no .rda, `time_hour` vem como hora local de Nova York sem fuso. O CSV a localiza em America/New_York e
grava em UTC (`...Z`), no mesmo formato do nycflights13 do Rdatasets usado no H3. Fora isso, as colunas são as
originais, mais `rownames`.
