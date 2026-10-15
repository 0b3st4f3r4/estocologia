# Ponto 7 (ICP): versão robusta ao agrupamento e réplica da genética

## Resultado

O teste do ICP do H2g trata cada linha como independente. No ensaio do FDA há 16 instantes por voluntário e no
STAR há 80 escolas com até 403 alunos. **Nas duas áreas que dependem de linha agrupada, o resultado se separa:**

- **Farmacologia sobrevive, e é sólida.** O Ŝ é {c_dof, c_qui, c_ran} nos três testes — original, Wald agrupado por
  voluntário (CR1) e permutação em blocos — e não contém `pr` nem `tamp`, como a verdade de referência declarada
  antes pede. A interseção dos aceitos não é vazia em nenhum deles. O que encolhe é a evidência: o p do conjunto
  vazio vai de 6,3e-104 no teste original para 2,2e-15 no agrupado e 0,002 na permutação, isto é, de cerca de 340
  bits contra o vazio para 49 e 9. Levar a correlação dentro do voluntário a sério não apaga o achado; corta a
  conta que o anunciava por sete ordens de grandeza.
- **Educação não sobrevive.** Com o Wald agrupado por escola (80 grupos, 446 dos 11 598 alunos trocam de escola), o
  ICP aceita 266 conjuntos de 512, mas a **interseção é vazia**: Ŝ = ∅. A permutação em blocos (escola × ambiente,
  513 blocos, B = 999) aceita 512 e também dá Ŝ = ∅. O conjunto publicado, {anos_pequena_antes, turma_pequena},
  tem p = 0,76 no teste agrupado: ele deixa de ser aceito. Pela regra declarada — "Educação sobrevive se o Ŝ do
  teste principal for não vazio" — a Educação **sai**.

**Consequência para o portão do H2g.** A Física (túnel de luz) não faz parte deste ponto e entra como está, com o
Ŝ = {blue, green, red}. A réplica da genética rodou (seção própria abaixo): o ICP dos 40 genes Y novos acerta um gene
na validação estrita por intervenção, e pela regra declarada a Genética **se replica**. Com a Educação fora e a
Genética replicada, as áreas são **três: Física, Farmacologia e Genética**. O critério do PLAN pede Ŝ ≠ ∅ em 3 áreas
distintas, então o **portão do H2g segue de pé com 3 áreas** — sem a Educação e sem o conjunto que a sustentava, e com
a genética sustentada por um domínio novo, não mais pelo y08 sozinho. Era esse o caso em jogo: se nenhum Y novo tivesse
dado Ŝ com gene validado, o portão **cairia de 4 para 2 áreas e a transferência entre áreas deixaria de estar
aprovada** por esse critério.

| área | teste | Ŝ | aceitos | p do Ŝ publicado | p do vazio |
|---|---|---|---|---|---|
| Farmacologia | original (H2g) | {c_dof, c_qui, c_ran} | 35 | 0,304 | 6,3e-104 |
| Farmacologia | **CR1, grupo = voluntário (22)** | {c_dof, c_qui, c_ran} | 61 | 1,0 | 2,2e-15 |
| Farmacologia | permutação em blocos (B = 999) | {c_dof, c_qui, c_ran} | 64 | 0,876 | 0,002 |
| Educação (STAR) | original (H2g) | {anos_pequena_antes, turma_pequena} | 57 | 0,039 | 1,6e-37 |
| Educação (STAR) | **CR1, grupo = escola (80)** | **∅** | 266 | 0,760 | 1,6e-06 |
| Educação (STAR) | permutação em blocos (B = 999) | **∅** | 512 | 1,0 | 0,002 |

O que "Ŝ = ∅" quer dizer aqui: não é que a turma pequena não tenha efeito. É que, com 80 escolas como unidades de
fato independentes, os conjuntos que o teste não consegue rejeitar não têm nenhuma variável em comum — o teste de
invariância perde o poder de apontar um conjunto, que é o único produto que ele entrega.

## Réplica da genética: 40 genes Y novos (α = 0,00125 por Y)

A genética não tem grupos — a amostra é uma cultura própria e o único rótulo disponível é `plate`, com dois níveis —
então a correção dela não é o Wald agrupado e sim a réplica declarada: 40 genes Y novos sorteados entre os nunca
deletados, fora os 10 do H2f (`default_rng(7007)`, pool de 4 681), cada um com a mesma pré-seleção dos 9 genes
deletados de maior |correlação| nas selvagens, os mesmos ambientes (obs, del_EJ, del_KP; cofre del_AD fora) e o teste
original do H2g. O α por Y é 0,05/40 = 0,00125. Rodou em 2026-10-15, `OMP_NUM_THREADS=1`, em 310,2 s de parede.

| situação dos 40 Y novos | n |
|---|---|
| Ŝ ≠ ∅ | 11 |
| — Ŝ com pelo menos um gene (fora `plate`) | **10** |
| — Ŝ só com `plate` (n36, YLR023C) | 1 |
| Ŝ = ∅, com o conjunto vazio aceito (pouco poder) | 11 |
| Ŝ = ∅ por conjuntos aceitos disjuntos | 13 |
| nenhum conjunto não saturado aceito | 5 |

Os 11 Y cujo Ŝ não é vazio (genes do Ŝ pelo índice da pré-seleção, no Y daquele domínio):

| Y | rótulo | Ŝ | genes do Ŝ | aceitos |
|---|---|---|---|---|
| YER170W | n01 | {x1, x4, x6} | x1 YPL123C, x4 YNL186W, x6 YBR201W | 6 |
| YBR063C | n03 | {plate, x1, x4, x5, x6, x8} | x1 YDR359C, x4 YGL131C, x5 YDR422C, x6 YBR175W, x8 YHR167W | 2 |
| YNL146W | n04 | {x9} | x9 YGL216W | 222 |
| YKR062W | n09 | {plate, x3} | x3 YMR191W | 11 |
| YJR118C | n12 | {plate, x3, x5, x9} | x3 YHR032W, x5 YDR392W, x9 YGR178C | 4 |
| YBR229C | n27 | {plate, x2, x7} | x2 YMR042W (a deleção não funcionou), x7 YMR304W | 36 |
| YJR098C | n29 | {plate, x1, x3, x4, x6, x8} | x1 YBR082C, x3 YBR062C, x4 YNL273W, x6 YMR255W, x8 YML099C | 1 |
| YLR354C | n34 | {x1, x4, x6} | x1 YBR225W, x4 YDR001C, **x6 YIL033C** | 66 |
| YLR023C | n36 | {plate} | — | 17 |
| YLR057W | n37 | {plate, x3, x4, x6} | x3 YGR271C-A, x4 YJL098W, x6 YMR269W | 23 |
| YAR007C | n40 | {plate, x9} | x9 YDL101C | 66 |

**Validação por intervenção** (Meinshausen et al. 2016, o critério declarado), nos 27 genes dos 10 Ŝ com gene:

- A deleção funcionou em 26 dos 27. A exceção é n27, x2 = YMR042W: no mutante ΔYMR042W o próprio YMR042W não é o menor
  valor entre todas as linhas.
- **Um gene passa na validação estrita: n34 (Y = YLR354C), x6 = YIL033C.** A deleção funcionou e, no mutante
  ΔYIL033C, a expressão de YLR354C é o **mínimo das 1 741 linhas** (quantil 0,000, isto é, fora da faixa de todas as
  outras amostras), na direção prevista pelo coeficiente positivo (+0,053).
- Coerência fraca, que sozinha **não** conta: n03 x4 = YGL131C (quantil 0,016, sinal coerente, mas dentro da faixa) e
  o próprio n34 x6 = YIL033C. Nenhum outro gene chega perto: os demais quantis ficam entre 0,055 (n34, x1 =
  YBR225W) e 0,964 (n03, x6 = YBR175W).

**Controle y08 (o domínio do H2f, com α = 0,005).** Reproduz o publicado: Ŝ = {plate, x1 = YKL009W, x5 = YNL023C,
x9 = YPL193W}, 2 conjuntos aceitos de 1 024, melhor p = 0,00724 (H2f: 0,0072). A validação repete a tabela do H2f
inteira: deleção funciona nos três, nenhum fica fora da faixa (quantis 0,0557 / 0,855 / 0,983) e **0 dos 3** genes
passa na validação estrita.

**Veredito pela regra declarada.** Houve Ŝ com gene (10 dos 40 Y novos) e um gene desses — n34, x6 = YIL033C — passa
na validação estrita. A genética **se replica** e conta como área. Não é o caso "frágil": a coerência fraca sozinha
não decide nada aqui, e o gene que decide passa pelo critério estrito, não pelo fraco.

## O que foi feito

1. **Declaração antes de rodar.** [DECLARACAO.md](DECLARACAO.md), com o hash em `DECLARACAO.sha256`, escrita antes
   de qualquer ICP nesta pasta. Ela fixa o que não muda (código copiado de `h2g/`, dados, ambientes de treino,
   candidatos, codificação, α, saturação K = 3), o teste principal (Wald CR1 em duas partes — locação pelos postos
   e escala de Brown–Forsythe — com Bonferroni das duas), o secundário (permutação de blocos inteiros), os grupos
   de cada área (voluntário, escola, amostra própria na genética), os critérios de sobrevivência e o que conta como
   falha: não reproduzir o Ŝ publicado, ou mudar o teste depois de ver o resultado.
2. **Reprodução primeiro.** O teste original reproduz, nas duas áreas, o Ŝ, o número de aceitos e o melhor p
   publicados no `h2g/` (campos `original_funcao_h2g` de `saida/farmacologia.json` e `saida/star.json`).
3. **Só então** os dois testes robustos, sobre o mesmo resíduo u = r / ŝ de `p_invariancia_np`.
4. **A genética veio por último**, com `Kemmeren.RData` e o parquet derivado no lugar: o ICP dos 40 Y novos com o
   teste original do H2g, α = 0,00125 por Y, a validação por intervenção de cada gene dos Ŝ e o y08 como controle.

## Conferência da reprodução

Rodei as duas áreas de novo em 2026-10-15 (OMP_NUM_THREADS=1, `P07_SAIDA` fora do repositório) e comparei com as
saídas gravadas: Ŝ, número de aceitos, veredito de cada teste, melhor conjunto e p do vazio batem nas duas áreas.
As duas únicas diferenças são o último dígito do p do vazio do CR (2,1569841841513456e-15 contra
2,1569841841514194e-15 na farmacologia, e 1,6350118112824874e-06 contra 1,63501181128208e-06 no STAR), que é
arredondamento de ponto flutuante. O tempo de parede, porém, caiu muito: 12,4 s e 213,0 s, contra os 419,4 s e
2 333,1 s da primeira execução, que rodou numa máquina dividida com outros trabalhos. Os números não mudam; o
custo muda.
## Limites

- **A genética rodou, mas o parquet derivado não bate byte a byte com a referência.** `Kemmeren.RData` confere com
  o esperado (86 328 652 bytes, sha256 `08e95f39…`), e a conversão para
  `dados-reais/h2f/kemmeren2014_obs_int.parquet` confere no **conteúdo**: 1741 linhas × 6175 colunas (5 de metadados
  na ordem `sample_id`, `kind`, `deleted_gene`, `deleted_gene_col_index_1based`, `plate`, mais os 6170 genes por
  ORF), blocos observacional (262 × 6170) e intervencional (1479 × 6170) idênticos bit a bit ao `RData`, genes em
  float64 e sem transformação. O sha256 do arquivo, porém, é `f152937f…`, e não o de referência `0921bda4…`: o byte a
  byte do Parquet carrega a versão do escritor (`parquet-cpp-arrow 26.0.0`, pandas 3.0.5), e nenhuma das codificações
  alternativas testadas para as colunas de metadados (índice em NA ou −1; `plate` como bool ou inteiro; `kind` como
  texto ou categoria; genes na ordem do arquivo ou ordenados; índice do dataframe escrito ou não) reproduz aquele
  hash. A réplica rodou sobre o conteúdo conferido, não sobre o hash.
- **A validação estrita da genética decide com uma linha.** Cada mutante é um ambiente de n = 1, então o critério
  "ΔX funcionou e Y cai fora da faixa de todas as outras linhas" é uma comparação de uma observação contra 1 740. O
  gene que sustenta a réplica (n34, x6 = YIL033C, com Y = YLR354C no mínimo das 1 741) passa nesse critério, e a
  declaração não fixou taxa nula para ele: 1 gene em 27 é o número, sem p de acompanhamento. `plate` não é validável
  por intervenção (não é gene deletado).
- **O STAR tem grupos que se cruzam.** 446 alunos passam por mais de uma escola, o que liga dois grupos; e os
  blocos de uma escola têm tamanhos diferentes, então a troca é só aproximadamente válida, como a declaração diz.
- **O teste que decide é o principal.** A permutação entra como informação, não como decisão.
- **A genética não tem grupos.** A amostra é uma cultura própria, e o único rótulo é `plate`, com dois níveis, que
  não servem como grupos; por isso a correção dela é a réplica, e não o Wald agrupado.

## Como reproduzir

Com os dados de `dados-reais/h2b` no lugar (o ensaio do FDA e o STAR estão no repositório). A genética sai de
`dados-reais/h2f/Kemmeren.RData` (fora do Git; origem e soma em `dados-reais/h2f/FONTES.md` e `SHA256SUMS`) para o
parquet derivado, com `rdata` 1.1.0 e pandas 3.0.5:

```
cd fundo/p07-icp/codigo
python3 construir_parquet_h2f.py                                    # RData → dados-reais/h2f/kemmeren2014_obs_int.parquet
ESTOCOLOGIA_DADOS=<repo>/dados-reais/h2b OMP_NUM_THREADS=1 python3 icp_robusto.py farmacologia star genetica
```

Numa máquina dividida com outros trabalhos, a primeira execução levou 419,4 s (farmacologia) e 2 333,1 s (STAR),
que são os valores do campo `segundos` das saídas; com a máquina livre, 12,4 s e 213,0 s. A genética levou 310,2 s
com a máquina livre. Depende de numpy, pandas, scipy e pyarrow — e, só para a conversão do RData, do pacote `rdata`. Sem
argumento, o script tenta as três áreas.