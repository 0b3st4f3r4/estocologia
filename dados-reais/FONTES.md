# Dados reais da Estocologia (H0 e H1 em domínio real)

Baixados em 2026-10-14 pelo espelho público Rdatasets (raw.githubusercontent.com/vincentarelbundock/Rdatasets)
e pelo espelho CRAN no GitHub (raw.githubusercontent.com/cran/...). Os portais oficiais (DATASUS, dados.gov.br,
FRED, UCI, Kaggle) estão bloqueados pela rede deste ambiente. Somas SHA-256 em `SHA256SUMS`.

| Arquivo | Origem | O que é | Uso |
|---|---|---|---|
| bank_calls.csv | Rdatasets `fpp3::bank_calls` | Chamadas a cada 5 min na central de um grande banco comercial norte-americano, 3 mar–24 out 2003, dias úteis, 07:00–21:05 (164 dias × 169 intervalos) | H0 |
| djia_1885_2023.csv | Rdatasets `stevedata::DJIA` | Fechamento diário do Dow Jones Industrial Average, 1885-02-16 a 2023-11-21 (37 931 pregões; sem dividendos) | H1·A |
| crsp_day_1989_1998.csv *(fora da árvore)* | Rdatasets `Ecdat::CRSPday` | Retornos diários de GE, IBM, Mobil e do índice CRSP, 1989–1998 (declarado fora da árvore em `SHA256SUMS`; não consumido pelos instrumentos vigentes) | H1·A (conferência) |
| nsw_experimental.csv | Rdatasets `causaldata::nsw_mixtape` | NSW (LaLonde 1986), amostra Dehejia–Wahba: 185 tratados e 260 controles sorteados | H1·B |
| cps_observational.csv | Rdatasets `causaldata::cps_mixtape` | Controles observacionais do CPS (15 992) usados contra os tratados do NSW | H1·B |
| danish_fire.csv | CRAN `fitdistrplus::danishuni` (espelho GitHub) | Perdas de incêndio da Copenhagen Re acima de 1 M DKK, com data, 1980-01-03 a 1990-12-31 (2 167 sinistros, em milhões de DKK, ajustados pela inflação a 1985) | H1·C |

Referências clássicas (citadas de memória, conferir): Shen & Huang (2008) para a série de chamadas;
LaLonde (1986) e Dehejia & Wahba (1999) para o NSW; McNeil (1997) e Embrechts, Klüppelberg & Mikosch (1997)
para as perdas dinamarquesas.
