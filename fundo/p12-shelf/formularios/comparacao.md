# Comparação: priori elicitada × priori de literatura × priori do pacote

**Esqueleto. Nenhuma elicitação aconteceu: as colunas "elicitada" estão em branco de propósito.** Este arquivo é o
instrumento da comparação que só pode ser preenchido depois que o painel do
`fundo/p12-shelf/protocolo.md` terminar, cada formulário de `formularios/` estiver com os quartis
confirmados e o consenso estiver em `prioris_elicitadas.yaml`.

## Critério declarado — K = 2 desta frente

Esta comparação **não** é leitura: é um critério novo, e por isso entra como **declaração nova**, com hash próprio
(`DECLARACAO_K2.md` + `DECLARACAO_K2.sha256`), escrita **antes** de comparar, e **conta no K**. Ela é a
segunda declaração desta frente — a primeira é o próprio protocolo congelado (K = 1 da frente) — e é a segunda na
contagem do K = 2 que o programa já usa para mudanças de critério depois de ver resultado
(`fundo/p05-prioris/h2/README.md`, seção "Mudanças depois de ver resultados"; `fundo/p05-prioris/h2g/DECLARACAO_K3.md`,
"o teste K = 2 de resíduos").

**Regra de casa respeitada:** nenhum arquivo de declaração anterior é reescrito e nenhum hash existente é trocado.
O K desta frente **soma-se** ao K que cada domínio já tem no campo `K` do próprio `resultados.json`:

| domínio | K de hoje (fonte) | K depois desta declaração |
|---|---|---|
| `h2/desenvolvimento_preston` | 1 (`h2/saida/desenvolvimento_preston/resultados.json`) | 2 |
| `h2/direito_prisoes` | 1 (`h2/saida/direito_prisoes/resultados.json`) | 2 |
| `h2/educacao_star` | 1 (`h2/saida/educacao_star/resultados.json`) | 2 |
| `h2g/educacao_star_turmas` | 3 (`h2g/saida/educacao_star_turmas/resultados.json`) | 4 |
| `h2g/direito_teto_beneficio` | 3 (`h2g/saida/direito_teto_beneficio/resultados.json`) | 4 |
| `h1-real/c` | sem campo `K` no `h1-real/saida/c/resultados.json` | quem assinar abre o K do H1 real na declaração da frente |

Se a contagem do K do programa rotular isto de outro número, o rótulo é corrigido por **declaração nova** — este
arquivo não é reescrito para renumerar.

### O que conta como "muda" (fixado antes de comparar)

A definição é a do p05, letra por letra (`fundo/p05-prioris/DECLARACAO.md`, seção "Mudança"): um domínio **muda**
numa rodada se **a categoria de status muda** (inclusive a lista dos portões que falham) **ou se a ação do F8 muda**
(inclusive passar de decisão emitida a nenhuma, ou o contrário). Mudança cujo único motivo é portão computacional
(R-hat, ESS, SBC) conta como mudança e é listada à parte.

Numericamente, relatam-se sempre, por domínio e por rodada: média e IC90 da posterior da chave, razão dp posterior /
dp priori, P(adotar compensa) e o deslocamento `|m − m_(a)| / dp_(a)` em unidades do dp posterior da rodada (a) —
o mesmo que o p05 mediu. Duas referências de leitura, medidas pelo próprio p05 e não inventadas aqui:

- o **desvio ambiental** máximo do p05 é **0,0071 dp** (numpy/scipy de outra máquina); abaixo disso, a diferença é
  numérica, não de conteúdo;
- o **menor deslocamento que mudou uma decisão** no p05 foi **0,18 dp** — vinte e cinco vezes o desvio ambiental.

Ou seja: deslocamento ≥ 0,18 dp com mudança de categoria ou de ação é o caso que interessa; deslocamento abaixo de
0,0071 dp não é leitura nova.

### O que já se sabe (e por isso não decide sozinho)

A rodada (b) do p05 **já** rodou com a priori de literatura: três dos seis domínios mudaram de status
(`h2/direito_prisoes`, `h2/educacao_star`, `h2g/educacao_star_turmas` podados, `h1-real/c`
aprovado) e dois mudaram de ação (`h2/desenvolvimento_preston` adotar → manter; `h1-real/c` exposição 0,4 →
0,0). A tabela está na seção 6.1 do `fundo/p12-shelf/protocolo.md`, com fonte em
`fundo/p05-prioris/tabela.md`. Duas consequências para a leitura desta comparação, declaradas aqui:

1. a priori elicitada entra **depois** de a de literatura ter sido vista — quem compara não é cego;
2. por isso o que interessa não é "a elicitada muda?", e sim **onde ela cai entre o pacote e a literatura** (ou fora
   dos dois), e se a decisão com ela é a do pacote, a da literatura ou uma terceira.

## Prioris lado a lado

Ajuste feito com as funções do próprio repositório: `ajustar_normal` (`h2f/estocologia/dominio.py`) para os
domínios gaussianos, com os quartis de `h2/direito_prisoes` em **log da razão de chances**
(`priori_da_chave`, mesmo arquivo), e `ajustar_beta_quartis` (`h2f/estocologia/estat.py`) para o H1
real. As colunas "pacote" e "literatura" são cópia conferida; a linha "elicitada" está **em branco**.

| domínio | prioridade | q05 | q25 | q50 | q75 | q95 | ajuste | erro máx. nos quartis | razão largura 90 |
|---|---|---|---|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | pacote (`pacote.yaml`) | 3 | 5 | 6,5 | 8 | 10,5 | N(6,5; dp 2,223903) | 5,2e-13 | 1,0252 |
| `h2/desenvolvimento_preston` | literatura p05 (`prioris/h2/literatura.yaml`) | −2,746195 | −0,219960 | 1,536 | 3,291960 | 5,818195 | N(1,536; dp 2,603390) | 2,8e-13 | 1,0000 |
| `h2/desenvolvimento_preston` | **elicitada (painel)** | — | — | — | — | — | — | — | — |
| `h2/direito_prisoes` | pacote (`pacote.yaml`) | 0,42 | 0,58 | 0,7 | 0,85 | 1,05 | log(RC): N(−0,354640; dp 0,283331) | 2,0e-03 | 0,9831 |
| `h2/direito_prisoes` | literatura p05 (`prioris/h2/literatura.yaml`) | 0,594406 | 0,657895 | 0,705978 | 0,757576 | 0,838493 | log(RC): N(−0,348171; dp 0,104582) | 3,5e-14 | 1,0000 |
| `h2/direito_prisoes` | **elicitada (painel)** | — | — | — | — | — | — | — | — |
| `h2/educacao_star` | pacote (`pacote.yaml`) | 2 | 8 | 12 | 16 | 24 | N(12; dp 5,930409) | 1,6e-12 | 1,1277 |
| `h2/educacao_star` | literatura p05 (`prioris/h2/literatura.yaml`) | 1,302178 | 4,860110 | 7,333191 | 9,806272 | 13,364204 | N(7,333191; dp 3,666596) | 4,8e-13 | 1,0000 |
| `h2/educacao_star` | **elicitada (painel)** | — | — | — | — | — | — | — | — |
| `h2g/educacao_star_turmas` | pacote (`pacote.yaml`) | 0,05 | 0,13 | 0,19 | 0,25 | 0,34 | N(0,19; dp 0,088956) | 4,1e-14 | 0,9910 |
| `h2g/educacao_star_turmas` | literatura p05 (`prioris/h2g/literatura.yaml`) | 0,017804 | 0,066451 | 0,100265 | 0,134079 | 0,182726 | N(0,100265; dp 0,050133) | 1,7e-14 | 1,0000 |
| `h2g/educacao_star_turmas` | **elicitada (painel)** | — | — | — | — | — | — | — | — |
| `h2g/direito_teto_beneficio` | pacote (`pacote.yaml`) | 0,0 | 0,2 | 0,35 | 0,5 | 0,75 | N(0,35; dp 0,222390) | 6,7e-14 | 1,0252 |
| `h2g/direito_teto_beneficio` | literatura p05 (`prioris/h2g/literatura.yaml`) | 0,228067 | 0,3 | 0,35 | 0,4 | 0,471933 | N(0,35; dp 0,074130) | 3,2e-14 | 1,0000 |
| `h2g/direito_teto_beneficio` | **elicitada (painel)** | — | — | — | — | — | — | — | — |
| `h1-real/c` | projeto, rodada (a): fraca Beta(1,1) | 0,05 | 0,25 | 0,5 | 0,75 | 0,95 | Beta(1; 1) | 4,7e-12 | — |
| `h1-real/c` | literatura p05 (`prioris/h1-real/literatura.yaml`) | 0,05 (deriv.) | 0,25 | 0,5 | 0,75 | 0,95 (deriv.) | Beta(1; 1) | 4,7e-12 | — |
| `h1-real/c` | **elicitada (painel)** | — | — | — | — | — | — | — | — |

Leitura das duas linhas preenchidas (aritmética descritiva, sem decisão nenhuma): em cinco dos seis domínios a priori de
literatura é **mais estreita** que a do pacote (dp de 2,60 contra 2,22 no Preston; 3,67 contra 5,93 no STAR; 0,050 contra
0,089 nas turmas; 0,074 contra 0,222 no teto; e, em log da RC, 0,105 contra 0,283 no direito). É essa diferença de
largura que move a posterior: a priori elicitada que sair mais larga que a de literatura **encolhe** o deslocamento já
observado na rodada (b) do p05.

## Decisão antes e depois

Por domínio. "Antes" = rodada (a) do p05, com a priori do pacote, como está nos `saida/` do repositório. "Depois"
= a rodada com a priori elicitada, a ser rodada exatamente como o p05 rodou (cópias em `fundo/p05-prioris/`, a
tabela da priori trocada por `formularios/prioris_elicitadas.yaml`, sementes, dados, custos e benefícios iguais).

| domínio | status (a) | status (depois) | ação (a) | ação (depois) | média posterior (a) → (depois) | razão dp post/priori (a) → (depois) | desloc. em dp da (a) | mudou? |
|---|---|---|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | aprovado | — | adotar | — | 5,09 → — | 0,26 → — | — | — |
| `h2/direito_prisoes` | reprovado: modelo_vence_climatologia | — | — | — | −0,73 → — | 0,28 → — | — | — |
| `h2/educacao_star` | aprovado | — | adotar | — | 13,5 → — | 0,61 → — | — | — |
| `h2g/educacao_star_turmas` | aprovado | — | manter | — | 0,148 → — | 0,38 → — | — | — |
| `h2g/direito_teto_beneficio` | aprovado | — | adotar | — | 0,579 → — | 0,24 → — | — | — |
| `h1-real/c` | reprovado: modelo_vence_climatologia | — | 0,4 (exposição f) | — | — | 0,87 → — | — | — |

Colunas "a" copiadas de `fundo/p05-prioris/tabela.md` (com `h2/desenvolvimento_preston`, educações e
direito em `fundo/p05-prioris/README.md`, seção das cinco mudanças de (b)).

### Linha de comparação com a rodada (b) do p05, que já existe

| domínio | status (b) literatura | ação (b) | desloc. (b) em dp | leitura a fazer depois da elicitada |
|---|---|---|---|---|
| `h2/desenvolvimento_preston` | aprovado | manter | 0,48 | a elicitada volta a "adotar" ou repete o "manter"? |
| `h2/direito_prisoes` | podado no F1 (EVPI < custo) | — | — (podado antes do F4) | a elicitada também poda no F1, ou o domínio chega ao F4? |
| `h2/educacao_star` | podado no F2 | — | — | idem, no F2 |
| `h2g/educacao_star_turmas` | podado no F1 (EVPI < custo) | — | — | idem, no F1 |
| `h2g/direito_teto_beneficio` | aprovado | adotar | 1,4 | o maior deslocamento do p05 se repete? |
| `h1-real/c` | aprovado | 0,0 (exposição) | — (o `comparar.py` não calcula no H1) | a elicitada repete o 0,0 ou volta ao 0,4? |

## Veredito, a ser escrito depois

Regra fixa (declarada antes): sejam N_p e N_l os números de domínios que mudam de status ou ação **em relação ao pacote**
(N_p) e **em relação à literatura do p05** (N_l), sobre os 6 domínios.

- N_p = 0 e N_l = 0: a priori elicitada não muda nada; as conclusões não dependiam de quem elicitou.
- N_p > 0 e N_l = 0: a elicitada concorda com a literatura e discorda do pacote — o que muda é a priori informativa,
  não o painel.
- N_l > 0: o painel diz algo **diferente do que a literatura já dizia**; o veredito nomeia cada domínio, o sinal do
  deslocamento e se a fonte é independente ou de **mesma base** (`h2/educacao_star`,
  `h2g/educacao_star_turmas`, `h2g/direito_teto_beneficio`).
- Em qualquer caso: dizer se o G3 do H2g (Ŝ ≠ ∅ em pelo menos 3 áreas) muda — o ICP não usa a priori, então só muda se
  um domínio for podado antes do F4.

**Campos que faltam preencher (todos em branco agora):** os três blocos de quartis elicitados (tabela de prioris), o
`consenso` de cada `formularios/<horizonte>__<dominio>.yaml`, as colunas "depois" das duas tabelas de
decisão, o deslocamento em dp, o N_p, o N_l e o veredito.

## Como reproduzir / como preencher

1. Elicitar e registrar como manda o `fundo/p12-shelf/protocolo.md`: cada formulário por especialista, com
   pré-teste de 20 previsões resolvidas, controles de âncora, ordem e excesso de confiança, e devolução.
2. Preencher o bloco `consenso` de cada formulário e levá-lo para
   `formularios/prioris_elicitadas.yaml` como **bloco novo** (nada é reescrito).
3. Ajustar com as funções do repositório — as mesmas usadas na tabela de prioris deste arquivo:

```python
import sys; sys.path.insert(0, "h2f")
from estocologia.dominio import ajustar_normal          # gaussianas (e binomiais, com os quartis em log da RC)
from estocologia.estat import ajustar_beta_quartis      # H1 real (frequência)
ajustar_normal({"q05": 3, "q25": 5, "q50": 6.5, "q75": 8, "q95": 10.5})
```

4. Rodar a comparação como o p05 rodou (cópias em `fundo/p05-prioris/`, a tabela da priori por
   `$P05_PRIORI`, `OMP_NUM_THREADS=1`, no máximo 2 processos em paralelo) e preencher as duas tabelas de
   decisão.
5. Escrever o veredito e congelar a declaração com hash (`DECLARACAO_K2.md` + `DECLARACAO_K2.sha256`).
