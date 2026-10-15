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

**Dois derivados não batem com a soma registrada, e o motivo é o escritor, não o dado.** O
`dados-reais/h2e/causal_chamber_lt_interventions_standard_v1.csv` — a concatenação dos 59 CSVs do zip de origem, com a
coluna `experiment` acrescentada — e o `dados-reais/h2f/kemmeren2014_obs_int.parquet` foram reconstruídos em
2026-10-15 a partir das origens, e as origens conferem (MD5 dos dois zips, SHA-256 do `Kemmeren.RData`). Os derivados
reconstruídos saem com `a80bc1a6…` e `f152937f…` no lugar de `99b75516…` e `0921bda4…`, porque o byte do derivado
depende de quem escreve — o Parquet, por exemplo, grava a versão do escritor nos metadados. O conteúdo foi conferido
campo a campo (forma, ambientes, blocos de amostras) e o desvio está relatado em `fundo/p05-prioris/README.md`,
`fundo/p07-icp/README.md` e `dados-reais/h2f/CONVERSAO.md`.

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

## As três classes, e o comando que confere cada uma

Os 24 arquivos acima não se conferem do mesmo jeito. São três classes, e cada uma tem o seu comando, a partir da
raiz do repositório:

| classe | o que é | comando | arquivos |
|---|---|---|---|
| **baixar** | dado bruto, com a origem no `FONTES.md` da pasta de `dados-reais/` | `sha256sum -c FORA_DO_GIT.sha256`, depois de baixar | 5 |
| **regerar rodando o horizonte** | rastro (`diario.*`, `hipergrafo*`), regerado pelo `rodar.py` (ou `grafo.py`) do horizonte | regerar e então `sha256sum -c FORA_DO_GIT.sha256` | 17 |
| **reconstruído** | derivado de dado bruto escrito nesta máquina | `sha256sum -c FORA_DO_GIT.reconstruido.sha256` | 2 |

Os dois derivados da terceira classe são o CSV do h2e e o Parquet do h2f — os mesmos da nota acima. As somas deles
estão em `FORA_DO_GIT.reconstruido.sha256`, com a pilha do escritor no cabeçalho, e as de referência continuam nas
duas listas antigas (`FORA_DO_GIT.sha256` e o `SHA256SUMS` de cada pasta). Nenhuma lista foi reescrita.

Arquivo por arquivo:

| arquivo | classe | origem, ou comando que regera | conferência |
|---|---|---|---|
| `dados-reais/h2/nycflights13_flights.csv` | baixar | Rdatasets `nycflights13`, ver `dados-reais/FONTES.md` | `FORA_DO_GIT.sha256` |
| `dados-reais/h2e/causal_chamber_lt_interventions_standard_v1.csv` | **reconstruído** | 59 CSVs do zip `lt_interventions_standard_v1` concatenados, mais a coluna `experiment`; origem no `dados-reais/h2e/FONTES.md` | `FORA_DO_GIT.reconstruido.sha256` |
| `dados-reais/h2e/causal_chamber_wt_walks_v1_regime_jumps_single.csv` | baixar | zip `wt_walks_v1`, ver `dados-reais/h2e/FONTES.md` | `FORA_DO_GIT.sha256` |
| `dados-reais/h2f/Kemmeren.RData` | baixar | espelho `longjp/causal-bias-code`, ver `dados-reais/h2f/FONTES.md` | `FORA_DO_GIT.sha256` |
| `dados-reais/h2f/kemmeren2014_obs_int.parquet` | **reconstruído** | do `Kemmeren.RData` por `fundo/p07-icp/codigo/construir_parquet_h2f.py` (rdata 1.1.0, pandas 3.0.5) | `FORA_DO_GIT.reconstruido.sha256` |
| `dados-reais/h2g/ecgcipa/adeg.csv` | baixar | PhysioNet `ecgcipa` v1.0.0 (espelho S3), ver `dados-reais/h2g/FONTES.md` | `FORA_DO_GIT.sha256` |
| `dados-reais/h3b/nycflights23_flights.csv` | baixar | ver a nota abaixo | `FORA_DO_GIT.sha256` |
| `h1/saida/diario.json` | regerar | `cd h1 && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2/saida/diario.json` | regerar | `cd h2 && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2b/saida/diario.json` | regerar | `cd h2b && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2c/saida/diario.json` | regerar | `cd h2c && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2d/saida/diario.json` | regerar | `cd h2d && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2d/saida/hipergrafo.json` | regerar | `cd h2d && python3 grafo.py` | `FORA_DO_GIT.sha256` |
| `h2e/saida/diario.json` | regerar | `cd h2e && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2f/saida/diario.json` | regerar | `cd h2f && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h2g/saida/diario.json` | regerar | `cd h2g && python3 rodar.py` | `FORA_DO_GIT.sha256` |
| `h3/saida/bolsa/diario.parquet` | regerar | `cd h3 && python3 rodar.py bolsa` | `FORA_DO_GIT.sha256` |
| `h3/saida/energia/diario.parquet` | regerar | `cd h3 && python3 rodar.py energia` | `FORA_DO_GIT.sha256` |
| `h3/saida/hipergrafo_arestas_tree.parquet` | regerar | `cd h3 && python3 grafo.py` | `FORA_DO_GIT.sha256` |
| `h3/saida/hipergrafo_nos.parquet` | regerar | `cd h3 && python3 grafo.py` | `FORA_DO_GIT.sha256` |
| `h3/saida/prisoes20/diario.parquet` | regerar | `cd h3 && python3 rodar.py prisoes20` | `FORA_DO_GIT.sha256` |
| `h3/saida/prisoes_ri/diario.parquet` | regerar | `cd h3 && python3 rodar.py prisoes_ri` | `FORA_DO_GIT.sha256` |
| `h3/saida/voos/diario.parquet` | regerar | `cd h3 && python3 rodar.py voos` | `FORA_DO_GIT.sha256` |
| `h3/saida/voos23/diario.parquet` | regerar | `cd h3 && python3 rodar.py voos23` | `FORA_DO_GIT.sha256` |

Sem nome de fluxo, o `h3/rodar.py` roda os seis do H3a e deixa de fora `voos23` (H3b), `prisoes20` (H3c) e
`prisoes_ri` (H3d), que por isso aparecem com o nome explícito na tabela.

### `dados-reais/h3b/nycflights23_flights.csv` não foi reconstruído aqui

A nota anterior (`FORA_DO_GIT.sha256`) traz a soma de referência `44415ddc…` para esse CSV, e ele não foi
reconstruído nesta máquina: fica na classe **baixar**, conferido pelo `FORA_DO_GIT.sha256`, e não entra nas somas
do `FORA_DO_GIT.reconstruido.sha256`. A origem está no `dados-reais/h3b/FONTES.md`: baixado em 2026-10-14 do
espelho CRAN no GitHub (`raw.githubusercontent.com/cran/nycflights23/master/data/`), com as duas somas no
`SHA256SUMS` da pasta, e o CSV é o `.rda` convertido — o `time_hour` localizado em `America/New_York` e gravado
em UTC, as demais colunas originais mais `rownames`. **O motivo de não reconstruir aqui: nenhuma frente usa o H3.**
O que ficou no lugar: os dois `.rda` e o `nycflights23_weather.csv`; o CSV de voos não está na pasta, e por isso
ele também entra na lista dos ausentes no `sha256sum -c FORA_DO_GIT.sha256`.

Quem aponta para essa pasta na árvore, conferido: `h3/fluxos/voos23.py` (o fluxo do H3b, cujo `self.base` é
`dados/h3b`) e a cópia dele em `fundo/p08-calibracao/codigo/fluxos/voos23.py`. Os dois leem o cofre pela pasta
apontada, não reescrevem o CSV.

### Estado da conferência em 2026-10-15

`sha256sum -c FORA_DO_GIT.reconstruido.sha256` → 2 OK. `sha256sum -c FORA_DO_GIT.sha256` → 4 OK, 2 FAILED (o CSV
do h2e e o Parquet do h2f, os dois da nota acima) e 18 ausentes (os 17 rastros, que não estão nesta máquina, e o CSV
do h3b). Nenhuma soma foi reescrita para fazer essa conferência passar.

**Rastros e dados novos das frentes de fundo.** Entraram fora do Git, pela mesma regra de 2 MB:

- `dados-reais/p10/` (219 MB): as 40 336 internações do PhysioNet/CinC 2019 baixadas pela frente `p10-saude`, com
  `FONTES.md` e `SHA256SUMS` na pasta; conferem com `sha256sum -c SHA256SUMS` (40 336 de 40 336).
- `fundo/p11-eventos-raros/saida/h3_ri/` (3,0 MB): o rastro do replay de Rhode Island da frente `p11-eventos-raros`,
  regenerado pelo comando do README da frente.

### O que é gerado dentro da árvore e não é versionado

Quatro coisas saíram do Git em 2026-10-15, porque um comando do próprio repositório as recria:

- `saida/armazem/` do `h2`, do `h2b` ao `h2g` e do `fundo/p10-saude` (27 MB, 591 arquivos): o armazém Parquet de
  cada execução, escrito pelo `rodar.py` da frente. Recriar: `python3 rodar.py` na pasta do horizonte.
- `fundo/p03-multiplicidade/copias/` (244 arquivos): as cópias do `h2` e do `h2g` da auditoria do K. Recriar:
  `python3 preparar_copias.py 38`, passo que já está na receita do README da frente.
- `fundo/p09-escala-dados/saida_base/` (18 arquivos): cópia dos `resultados.json` do `h2g/saida`, lida por
  `escala/analisar.py`. Recriar: o comando no README da frente.
- `dados-reais/h2f/kemmeren2014_obs_int.parquet` (100 MB): derivado do `Kemmeren.RData`, que continua na pasta,
  e **este fica**: as rodadas e os testes do `h2f` e do `h2g` leem o parquet direto. Se ele faltar, recriar com
  `python3 fundo/p07-icp/codigo/construir_parquet_h2f.py` (cerca de 2 s; sai com a soma `f152937f…` desta máquina,
  conforme `dados-reais/h2f/CONVERSAO.md`).
