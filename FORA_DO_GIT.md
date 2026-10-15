# Arquivos fora do Git

Estes 24 arquivos passam de 10 MB (dados brutos) ou de 2 MB (rastros em `saida/`) e não estão no repositório.
Os dados brutos se baixam pelas origens descritas no `FONTES.md` de cada pasta de `dados-reais/` e se conferem
pelas somas abaixo (as mesmas dos `SHA256SUMS` locais). Os rastros (`diario.*`, `hipergrafo*`) são regenerados
por `python3 rodar.py` e `python3 grafo.py` no horizonte correspondente; as somas abaixo permitem conferir que a
reprodução bate byte a byte quando o rastro é determinístico.

Conferência, depois de baixar ou regenerar, a partir da raiz:

```
sha256sum -c FORA_DO_GIT.sha256
```

| Arquivo | Tamanho |
|---|---|
| `dados-reais/h2/nycflights13_flights.csv` | 31.7 MB |
| `dados-reais/h2e/causal_chamber_lt_interventions_standard_v1.csv` | 18.7 MB |
| `dados-reais/h2e/causal_chamber_wt_walks_v1_regime_jumps_single.csv` | 91.9 MB |
| `dados-reais/h2f/Kemmeren.RData` | 82.3 MB |
| `dados-reais/h2f/kemmeren2014_obs_int.parquet` | 92.5 MB |
| `dados-reais/h2g/ecgcipa/adeg.csv` | 25.6 MB |
| `dados-reais/h3b/nycflights23_flights.csv` | 45.8 MB |
| `h1/saida/diario.json` | 19.1 MB |
| `h2/saida/diario.json` | 11.2 MB |
| `h2b/saida/diario.json` | 4.3 MB |
| `h2c/saida/diario.json` | 4.3 MB |
| `h2d/saida/diario.json` | 5.4 MB |
| `h2d/saida/hipergrafo.json` | 2.3 MB |
| `h2e/saida/diario.json` | 7.0 MB |
| `h2f/saida/diario.json` | 8.9 MB |
| `h2g/saida/diario.json` | 9.2 MB |
| `h3/saida/bolsa/diario.parquet` | 3.2 MB |
| `h3/saida/energia/diario.parquet` | 4.3 MB |
| `h3/saida/hipergrafo_arestas_tree.parquet` | 2.2 MB |
| `h3/saida/hipergrafo_nos.parquet` | 4.9 MB |
| `h3/saida/prisoes20/diario.parquet` | 2.0 MB |
| `h3/saida/prisoes_ri/diario.parquet` | 3.4 MB |
| `h3/saida/voos/diario.parquet` | 7.4 MB |
| `h3/saida/voos23/diario.parquet` | 12.5 MB |
