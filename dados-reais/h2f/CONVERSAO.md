# Conversão de `Kemmeren.RData` para o parquet derivado: conferência de integridade

O `kemmeren2014_obs_int.parquet` que está nesta pasta foi reconstruído em 2026-10-15 a partir de `Kemmeren.RData`
(86 328 652 bytes, sha256 `08e95f395ba00c55e188a9bacb6a4c01c3cfaa144e3bcd171665f04e57e33bdb`, igual ao esperado) com
`fundo/p07-icp/codigo/construir_parquet_h2f.py` — pacote `rdata` 1.1.0, pandas 3.0.5, pyarrow 26.0.0.

**O sha256 do arquivo presente é `f152937f6ab6974de5f65857a94a941926af07cad04da89c318bea4b9a2138e2`, e não o
`0921bda4…` registrado na `SHA256SUMS` desta pasta. A `SHA256SUMS` não foi alterada.**

O conteúdo, porém, confere com o que a `FONTES.md` documenta: 1741 linhas × 6175 colunas — as cinco colunas de
metadados na ordem `sample_id`, `kind`, `deleted_gene`, `deleted_gene_col_index_1based`, `plate`, seguidas dos
6170 genes nomeados pelos ORFs, todos em float64 e sem transformação; sem NaN. O bloco observacional (262 × 6170) e o
intervencional (1479 × 6170) são **idênticos bit a bit** aos objetos `obs` e `int` do `RData`; `deleted_gene` é nulo
nas 262 selvagens e igual a `mutantnames` (= `genenames[intpos − 1]`) nos 1479 mutantes; `plate` reproduz
`plateobs` (160/102) e `plateint` (633/846).

O byte a byte de um Parquet carrega a versão do escritor (`parquet-cpp-arrow 26.0.0`, pandas 3.0.5 gravados nos
metadados do arquivo), o que explica a diferença de hash com uma conversão feita em outro ambiente. Nenhuma das
alternativas testadas para as colunas de metadados — índice em NA ou em −1; `plate` como bool ou inteiro; `kind` como
texto ou categoria; genes na ordem do arquivo ou ordenados; índice do dataframe escrito ou não — reproduz o hash de
referência.

A réplica da genética de `fundo/p07-icp/` rodou sobre o conteúdo conferido, não sobre o hash; esse limite está
declarado no README daquela frente.
