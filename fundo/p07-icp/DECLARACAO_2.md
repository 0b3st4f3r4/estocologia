# Declaração 2 do ponto 7 (ICP): taxa nula da validação estrita da genética

Escrita em 2026-10-15T19:16:49Z, antes de rodar qualquer permutação desta declaração. O SHA-256 deste arquivo fica em
`DECLARACAO_2.sha256`. Ela **não** reescreve `DECLARACAO.md` nem o hash `05ab9ff1…`: acrescenta uma taxa nula ao
número que a réplica da genética já usou para decidir. É declaração nova e conta no K.

## O que já estava rodado

Quando esta declaração foi escrita, o ponto 7 já estava rodado e relatado em `README.md`: dos 40 genes Y novos, o ICP
deu 11 Ŝ não vazios, **10** deles com pelo menos um gene fora de `plate`, 27 pares (Y, x) ao todo; a deleção funcionou
em 26 dos 27 (a exceção é n27, x2 = YMR042W); **1 gene em 27 passa na validação estrita** — n34 (Y = YLR354C),
x6 = YIL033C — e o controle y08 tem 0 de 3. O limite já escrito é que a validação estrita decide com uma linha de
mutante (n = 1), e o número 1 em 27 estava **sem taxa nula**. É esse buraco que esta declaração fecha.

## Conferência de estrutura, feita antes de escrever isto

Sem olhar desfecho além do que já está publicado acima: o parquet `dados-reais/h2f/kemmeren2014_obs_int.parquet` tem
**1 741 linhas** (262 observacionais + 1 479 intervencionais), das quais 1 479 têm `deleted_gene` preenchido;
**nenhum dos 10 Y** dos 27 pares tem uma linha com `deleted_gene` igual a ele, então para todo par o conjunto de linhas
é o mesmo, as 1 741; cada um dos 27 genes X e cada um dos 10 Y tem **mínimo e máximo únicos** (nenhum empate no
extremo); cada par tem **exatamente uma** linha de mutante. Isso é o que fixa o sorteio abaixo.

## A nula (decide)

**Troca do rótulo de mutante.** Para cada par (Y, x) o rótulo de mutante deixa de estar na linha em que o gene X foi
deletado e passa a estar numa linha sorteada uniformemente entre as 1 741. Sob a nula o rótulo não carrega informação
sobre Y, então a linha rotulada é permutável com qualquer outra.

Sorteio, fixado aqui e não mudado depois:

```python
rng = np.random.default_rng(20261015)          # semente declarada
I = rng.choice(1741, size=(999, 27), replace=True)   # I[b, j] = linha rotulada mutante do par j no sorteio b
```

A coluna j é o par j; a linha b é o sorteio b. Os 27 pares ficam na ordem em que aparecem em `saida/genetica.json`:
os 10 rótulos de `resumo.y_com_S_hat_com_gene` na ordem de `por_y` (n01, n03, n04, n09, n12, n27, n29, n34, n37,
n40) e, dentro de cada um, a ordem da lista `validacao`. Sorteios independentes entre pares e entre sorteios.

**Critério de um par (Y, x) com gene X = `gene` na linha rotulada i*, idêntico ao critério estrito já declarado,
avaliado inteiro sobre o rótulo permutado:**

1. **deleção funcionou:** X[i*] ≤ mínimo de X entre as outras 1 740 linhas;
2. **Y fora da faixa:** Y[i*] < mínimo de Y entre as outras 1 740 linhas, ou Y[i*] > máximo de Y entre elas.

Nada do observado fica fixo: as duas partes usam a linha sorteada.

**Estatística**

- por sorteio b: T_b = número de pares j que passam o critério estrito em `I[b, j]` (0 a 27);
- observada: T_obs, o mesmo critério aplicado à linha real de cada par (a linha com `deleted_gene == gene`). T_obs é
  o número já publicado, 1. Se a contagem reproduzida de `saida/genetica.json` não der o mesmo T_obs, paro e relato.

**p e decisão**

p = (1 + #{b : T_b ≥ T_obs}) / (999 + 1), α = 0,05. Um único p decide; não há correção por par porque a estatística
já soma os 27 pares. T_obs é improvável sob a nula se p ≤ 0,05.

**Secundária (informa, não decide).** Da mesma matriz `I`, sem sorteio novo: T_deleção (só a parte 1) e T_faixa
(só a parte 2), com o mesmo p. Elas decompõem o critério estrito. Já se sabe disto, e fica escrito antes de rodar:
como a parte 1 é um fato dos dados — deletar X tira a expressão de X, o mínimo da coluna X está na linha do mutante —
permutar o rótulo **destrói** essa parte, e é de esperar que quase todo o p venha dela. A secundária também é o que
diz quanto a nula vale sem a parte 1.

## O que não muda

- O sorteio dos 40 Y, o α por Y, a pré-seleção, os ambientes, o ICP, os Ŝ, o critério estrito de validação e o
  **critério de réplica** ficam exatamente como em `DECLARACAO.md`: "a Genética se replica se pelo menos um dos 40 Y
  novos der um Ŝ com gene e pelo menos um gene desses passar na validação estrita". **O critério de réplica não muda.**
- Os 27 pares e o T_obs são **lidos** de `saida/genetica.json`, não recalculados; e nenhum arquivo é reescrito.
- O controle y08 (0 de 3) não entra: ele não pertence aos 27 pares dos 10 Y novos. A validação do y08 continua como o
  H2f a publicou.
- A nula não é reamostragem do ICP nem dos Ŝ: ela condiciona nos 27 pares já obtidos. Ela mede "quantos pares passam
  por acaso, sorteando a linha do mutante", não "quantos Ŝ teriam gene num mundo nulo".

## O que conta como falha

Mudar B, a semente, o sorteio, a estatística, o critério, a ordem dos pares ou o conjunto de linhas depois de ver
T_b; usar um pool de linhas diferente das 1 741; recalcular os 27 pares ou o T_obs de outro jeito. Qualquer mudança
assim entra como declaração nova e conta no K. Não reproduzir T_obs = 1 é falha de execução: paro e relato.

## Como reproduzir

```
cd fundo/p07-icp/codigo
OMP_NUM_THREADS=1 python3 nulo_validacao.py      # lê <repo>/dados-reais/h2f/kemmeren2014_obs_int.parquet
```

Saída: `../saida/nulo_validacao.json`, com T_obs, a média e o quantil 95 da nula, o p, as duas secundárias e a
semente. O script não escreve em nenhum outro lugar.
