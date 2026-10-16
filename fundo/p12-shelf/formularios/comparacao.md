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

---

# Preenchimento de 2026-10-16 (parcial): o domínio \`h2/desenvolvimento_preston\`

O esqueleto acima continua valendo e não foi reescrito. O que segue é o que já pode ser preenchido, o que não pode,
e por quê.

## 1. As três prioris, lado a lado

| prior | q05 | q25 | q50 | q75 | q95 | média | dp | fonte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **pacote** | 3 | 5 | 6,5 | 8 | 10,5 | 6,5 | 2,223903 | \`fundo/p05-prioris/h2/dominios/desenvolvimento_preston/pacote.yaml\`, bloco \`shelf\`: literatura de Preston "citada de memória e por conferir; nenhum especialista humano consultado" |
| **literatura (p05)** | −2,746195205080443 | −0,21995975644268073 | 1,536 | 3,291959756442681 | 5,818195205080443 | 1,536 | 2,603390 | p05, Pritchett & Summers (1996), regra R1 |
| **elicitada (E1, n = 1)** | −0,50 | 1,75 | 3,25 | 4,75 | 7,00 | 3,25 | 2,223903 | \`formularios/h2__desenvolvimento_preston.yaml\`, rodada 1 confirmada; ajuste e controles em \`labs --elicitacao\` |

A priori elicitada tem a **mesma forma** do pacote — dp 2,223903 nos dois — sem ter visto o pacote: o especialista
não recebeu nenhum número do domínio antes de escrever os cinco quantis. O que muda é o **centro**: 3,25, isto é,
2,75 anos acima da literatura e 3,25 abaixo do pacote. A largura 90% elicitada é 7,5 contra 7,3160 implicada
(razão 1,0252), no limite de cima do controle de excesso de confiança, e as pontas do especialista são mais largas
do que os próprios quartis exigiriam.

## 2. A leitura no plano da própria priori

O limiar da decisão deste domínio está no próprio \`pacote.yaml\`: a via da renda compensa se \`theta\` passa de
**5 anos** (benefício 1,0 por unidade contra custo 5,0).

| prior | \`P(theta > 5)\` na própria priori |
| --- | --- |
| pacote | **0,7500** |
| literatura | **0,0917** |
| elicitada | **0,2157** |

\`\`\`bash
cargo run --release --bin labs -- --elicitacao "<q05,q25,q50,q75,q95>" --limiar 5
\`\`\`

Leitura: das duas prioris informativas (literatura e elicitada), as duas põem \`P(theta > 5)\` bem abaixo de 0,50 —
a elicitada em 0,2157, quase o dobro da literatura e menos de um terço do pacote. É o mesmo lado do limiar para as
duas, e é o lado que o p05 já tinha visto: com a literatura, a ação do F8 deste domínio virou **manter** (era
\`adotar\` com o pacote).

## 3. O que **não** é computável hoje, e por quê

O p05 mediu a passagem completa pelo funil com cada priori. Para este domínio, a tabela dele traz: status
\`aprovado\` nas três rodadas, ação do F8 \`adotar / manter / manter\`, média posterior 5,09 / 4,81 / 4,98 e
\`P(adotar compensa)\` 0,565 / 0,366 / 0,495 — a última é a priori fraca, não a elicitação.

Refazer essa passagem com a priori elicitada exige a cadeia que transforma priori em ação: o \`preparo.py\` do
domínio e a maquinaria do \`h2f/estocologia\`, **aposentados em 2026-10-16**
(\`DECLARACAO_REMOVIDO_PYTHON.md\`). O que sobrou na árvore é o \`pacote.yaml\` do domínio, não o código que roda.
Logo a coluna **(c) elicitada** fica **vazia**, marcada como vazia, e não preenchida por aproximação.

Dois caminhos declarados, e nenhum decidido aqui:

1. rodar a cópia do p05 que sobrou na árvore, aceitando a marca **P** do \`REGISTRO_EMPIRICO.md\` (número que só
   existe em Python) — o que mantém a comparação maçã com maçã, porque as colunas (a) e (b) saíram do mesmo código;
2. reimplementar o funil do domínio em Rust e preencher a coluna com instrumento — é o caminho do item 12 do
   inventário, que já pedia a segunda opinião dos resultados marcados **P**.

## 4. O que este preenchimento não pode dizer

- **se a priori elicitada muda a ação do F8** deste domínio: depende da seção 3, e hoje não fecha;
- **o consenso do painel**: este domínio tem **1** especialista aprovado, e a seção 2.4 do protocolo diz que com
  menos de três o domínio fica **não elicitado**. O que está aqui é a elicitação do E1, mantida como
  **sensibilidade**, e ela não substitui a priori do pacote na rodada oficial;
- **o controle de fora**: o E1 é o autor do programa, então o painel não tem o especialista de fora que o
  protocolo exige como controle principal;
- **o controle de ordem**: com um só especialista não há metade do painel na ordem inversa, e isso fica declarado
  em vez de simulado.

## 5. K

A frente tinha **K = 2** (protocolo = 1, comparação = 2) e a declaração de ajuste do pré-teste levou a **K = 3**.
Este preenchimento **não** soma K novo: enquanto a coluna (c) não fechar, o que está escrito aqui é aritmética
descritiva sobre prioris já publicadas, com um critério que já foi declarado.


---

## 6. A coluna (c) preenchida, com a marca **P** — decisão declarada do operador

**Decisão de 2026-10-16:** rodar a **cópia aposentada do p05** (o mesmo código que gerou as colunas (a) e (b)),
e não reimplementar o funil em Rust agora. Motivo declarado: a tabela compara **sensibilidade da decisão** a três
prioris, e usar o mesmo código é o que garante comparação maçã com maçã; a reimplementação do funil em Rust fica no
escopo próprio, o item 12 do inventário. O número que sai daqui leva a marca **P** do `REGISTRO_EMPIRICO.md`.

### Como rodar, e por que fora da árvore

O código do p05 saiu da árvore em 2026-10-16 e vive no commit `1bfec905`. A cópia foi restaurada **fora da
árvore** e rodada lá, para o repositório continuar só Rust (§VIII do contrato):

~~~bash
rm -rf /tmp/p05run && mkdir -p /tmp/p05run
git archive 1bfec905 fundo/p05-prioris dados-reais/h2 | tar -x -C /tmp/p05run
# a priori elicitada entra no formato do p05 (fonte, painel e quartis)
# /tmp/p05run/fundo/p05-prioris/prioris/h2/elicitada.yaml
cd /tmp/p05run/fundo/p05-prioris/h2
for v in original literatura elicitada; do
  tab=""; [ "$v" != original ] && tab="/tmp/p05run/fundo/p05-prioris/prioris/h2/$v.yaml"
  P05_PRIORI="$tab" ESTOCOLOGIA_DADOS=/tmp/p05run/dados-reais/h2 ESTOCOLOGIA_SAIDA=/tmp/p05run/saida/$v \
    python3 p05_rodar.py agronomia_nitrogenio clima_tempestades credito_hmda desenvolvimento_preston
done
~~~

**A lista de domínios não é enfeite.** O `p05_rodar.py` dá a semente `2027 + 100 i`, com `i` a posição do domínio
na lista pedida. `desenvolvimento_preston` é o quarto na ordem alfabética dos dez domínios, então os três primeiros
entram na chamada só para a semente dele continuar sendo **2327**, a mesma da rodada do p05. Rodar só o preston
mudaria a semente e o número não seria comparável.

### A conferência que autoriza a terceira coluna

As colunas (a) e (b) do p05 saem **iguais** quando o mesmo código roda de novo. A tabela do p05 traz média posterior
5,09 / 4,81 e `P(adotar compensa)` 0,565 / 0,366; a rodada de hoje deu 5,087198 / 4,806199 e 0,565375 / 0,366375.
Quem duvidar roda o bloco acima e compara.

### As três rodadas, lado a lado

| rodada | status | ação do F8 | P(adotar compensa) | média posterior | IC90 | razão dp post/priori | desloc. em dp da (a) | P(theta > 5) na priori (F1) | EVPI |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **pacote** | aprovado no F4 | adotar | **0,565375** | 5,087198 | [4,142920; 6,040198] | 0,2638 | 0 | 0,75285 | 0,32340 |
| (b) **literatura** | aprovado no F4 | manter | **0,366375** | 4,806199 | [3,828045; 5,781023] | 0,2293 | 0,4790 | 0,09210 | 0,11246 |
| (c) **elicitada (E1)** | aprovado no F4 | **manter** | **0,418875** | 4,867977 | [3,913771; 5,836887] | 0,2656 | 0,3737 | 0,21245 | 0,27330 |

O dp posterior da rodada (a) é 0,586673, e é nele que o deslocamento está medido.

### O que a linha nova diz

- **A ação do F8 muda de `adotar` para `manter`** com a priori elicitada, como já mudava com a de literatura. O
  status não muda: `aprovado no F4`, decisão emitida.
- **`P(adotar compensa)` cai de 0,565375 para 0,418875** e cruza o limiar de 0,50. A literatura leva a 0,366375: a
  elicitada fica **entre o pacote e a literatura**, mais perto da literatura.
- **O deslocamento é 0,3737 dp da rodada (a)** — menor que o da literatura (0,4790), como esperado, porque a média
  posterior da elicitada (4,867977) está mais perto da do pacote (5,087198) do que a da literatura (4,806199).
- **A razão dp posterior / dp priori é 0,2656**, praticamente a do pacote (0,2638): a priori elicitada tem a mesma
  forma do pacote, então o dado aperta a posterior quase igualmente.
- **A priori elicitada é a única das três com preço de pesquisa ainda alto** (EVPI 0,27330 contra 0,11246 da
  literatura), porque é mais larga que ela — e o F1 passa nas três.

### Limites desta linha, declarados

- **Marca P.** O número sai da cópia aposentada do Python, não de instrumento Rust. É o mesmo caminho das colunas
  (a) e (b), e é por isso que a comparação vale; a segunda opinião em Rust é o item 12 do inventário.
- **A rodada (c) é sensibilidade, não rodada oficial.** O painel do domínio tem **1** especialista aprovado, e é o
  autor do programa; a seção 2.4 do protocolo diz que com menos de três o domínio fica **não elicitado**.
- **O que esta linha não mede:** o efeito de uma priori elicitada *por três especialistas com um de fora*. Com n = 1
  e sem o controle de fora, o que se mediu foi a sensibilidade do funil à cabeça de uma pessoa — a mesma que já
  tinha escrito o pacote.


---

## 7. Segundo domínio preenchido, com a marca **P**: \`h2/direito_prisoes\`

Mesmo caminho do primeiro (decisão do operador de 2026-10-16): cópia aposentada do p05 restaurada fora da árvore, no commit \`1bfec905\`, e as três prioris rodadas pelo **mesmo código**. A priori elicitada entra no formato do p05 em \`prioris/h2/elicitada_direito.yaml\`.

**A semente de novo preservada.** \`direito_prisoes\` é o quinto domínio na ordem alfabética dos dez, então a chamada leva os quatro primeiros antes dele para a semente continuar \`2027 + 100 × 4 = 2427\`, a mesma da rodada do p05.

### As três prioris, lado a lado (escala de razão de chances, e o log que o ajuste usa)

| prior | q05 | q25 | q50 | q75 | q95 | média no log | dp no log |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **pacote** | 0,42 | 0,58 | 0,70 | 0,85 | 1,05 | −0,354640 | 0,283331 |
| **literatura (p05)** | 0,5944060189976956 | 0,6578947368421053 | 0,7059781184061313 | 0,7575757575757576 | 0,8384926931067868 | −0,348171 | 0,104582 |
| **elicitada (E1)** | 0,20 | 0,45 | 0,65 | 0,85 | 1,15 | −0,463937 | 0,471459 |

A elicitada tem a **maior largura das três** no log (0,471459 contra 0,283331 do pacote e 0,104582 da literatura) e a mediana mais pessimista (0,65; pacote 0,70 e literatura 0,706). O especialista declarou o motivo: ceticismo metodológico com intervalos de meta-análise artificialmente estreitos.

### A conferência que autoriza a terceira coluna

| rodada | o que a tabela do p05 diz | o que a rodada de hoje deu |
| --- | --- | --- |
| (a) pacote | reprovado: modelo_vence_climatologia; média posterior −0,73; razão dp 0,28 | reprovado no F4 (M15): modelo_vence_climatologia; média −0,729791; razão 0,275878 |
| (b) literatura | podado no F1: a pesquisa não muda a decisão (EVPI < custo) | podado no F1: a pesquisa não muda a decisão (EVPI < custo); priori ajustada com média −0,348171 e dp 0,104582 |

### As três rodadas

| rodada | status | EVPI na priori (F1) | P(adotar compensa) na priori | ação do F8 | média posterior | IC90 | razão dp post/priori |
| --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **pacote** | reprovado no F4 (M15): modelo_vence_climatologia | 0,541955 | 0,79195 | — | −0,729791 | [−0,859514; −0,603526] | 0,275878 |
| (b) **literatura** | podado no F1: EVPI < custo | 0,009989 | 0,98290 | — | — | — | — |
| (c) **elicitada (E1)** | reprovado no F4 (M15): modelo_vence_climatologia | **1,054148** | 0,76510 | — | **−0,752532** | [−0,885605; −0,623203] | **0,169906** |

### O que a linha nova diz, e é diferente do primeiro domínio

- **A ação não muda em nenhuma das três**: com o pacote, com a literatura e com a elicitada, o domínio fica
  **reprovado no F4** — o modelo não vence a climatologia — e nenhuma decisão é emitida. Aqui a controvérsia sobre
  a priori **não decide nada**, porque quem barra é outro portão.
- **O EVPI explode com a elicitada**: 0,541955 (pacote) → 0,009989 (literatura) → **1,054148** (elicitada). A
  priori mais larga das três é a que dá mais valor à pesquisa — e é a única das informativas que sobrevive ao F1,
  onde a de literatura foi podada.
- **A posterior anda pouco**: −0,729791 (pacote) contra −0,752532 (elicitada), 2,3% de diferença, com a razão
  dp post/priori caindo de 0,275878 para 0,169906 — com priori larga, o dado puxa mais.
- **A leitura de fundo**: em \`h2/desenvolvimento_preston\` a priori elicitada cruzou o limiar do F8 e virou a ação;
  aqui não vira nada, e o que ela move é o **valor da pesquisa**. Duas faces da mesma elicitação.

### Limites, os mesmos do primeiro domínio

- **Marca P**: o número sai do Python aposentado, o mesmo que gerou as colunas (a) e (b).
- **Sensibilidade, não rodada oficial**: o painel tem 1 especialista e é o autor do programa.
- **A largura elicitada é a variável dominante**, e ela veio de ceticismo declarado, não de dados: se um painel
  de três especialistas estreitar essa distribuição, o EVPI cai junto e o F1 volta a podar — é o que o p05 já
  tinha mostrado com a literatura.


---

## 8. Terceiro domínio preenchido, com a marca **P**: \`h2/educacao_star\`

Mesmo caminho (cópia do p05 fora da árvore, commit \`1bfec905\`, três prioris pelo mesmo código) e a semente de novo preservada: \`educacao_star\` é o sexto na ordem alfabética, então a chamada leva os cinco primeiros antes dele para a semente continuar \`2027 + 100 × 5 = 2527\`.

### As três prioris (pontos SAT)

| prior | q05 | q25 | q50 | q75 | q95 | média | dp |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **pacote** | 2,2453 | 8,0000 | 12,0000 | 16,0000 | 21,7547 | 12,0000 | 5,930409 |
| **literatura (p05)** | 1,3021781198967624 | 4,860110016547187 | 7,333191149998485 | 9,806272283449783 | 13,364204180100208 | 7,333191 | 3,666596 |
| **elicitada (E1)** | −2,5 | 6,0 | 12,0 | 18,0 | 26,5 | 12,0000 | 8,895613 |

**A literatura deste domínio carrega a marca \`mesma base\`**: ela nasceu da própria \`AER_STAR\` que o domínio usa, e por isso não é evidência independente. A elicitada tem a mesma mediana do pacote (12,0) e o **maior desvio das três** (8,895613).

### A conferência que autoriza a terceira coluna

| rodada | o que a tabela do p05 diz | o que a rodada de hoje deu |
| --- | --- | --- |
| (a) pacote | aprovado; adotar; média posterior 13,5; razão dp 0,61 | aprovado no F4; adotar; média 13,518436; razão 0,609555 |
| (b) literatura | podado no F2: dados na unidade independente não informam mais que a priori | podado no F2, com a mesma justificativa; efeito de desenho 3,343892 |

### As três rodadas

| rodada | status | EVPI (F1) | F2 informativo | ação do F8 | P(adotar compensa) | média posterior | IC90 | razão dp post/priori |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **pacote** | aprovado no F4 | 1,502145 | sim | adotar | **0,839625** | 13,518436 | [7,485530; 19,505214] | 0,609555 |
| (b) **literatura** | **podado no F2** | 0,497782 | **não** | — | — | — | — | — |
| (c) **elicitada (E1)** | aprovado no F4 | **2,640820** | sim | **adotar** | **0,836500** | **13,879785** | [7,140456; 20,576858] | **0,454279** |

### O que a linha nova diz, e é o resultado mais informativo dos três domínios

- **A ação não muda**: \`adotar\`, com P(adotar compensa) **0,836500** contra **0,839625** do pacote — diferença de 0,4%. A eliciteda tem a mesma mediana do pacote e alarga a dispersão, sem mexer na decisão.
- **O F2 muda, e é o achado**: a priori de **mesma base** foi **podada no F2** — "dados na unidade independente não informam mais que a priori" —, e a elicitada passa. O motivo é direto: uma priori construída da própria base do domínio, e estreita, **torna o dado redundante**; o F2 pega isso. Alargar a priori devolve valor ao dado. É o pedágio da circularidade, medido.
- **O EVPI sobre com a elicitada**: 2,640820 contra 1,502145 do pacote e 0,497782 da literatura.
- **A posterior quase não anda** (13,879785 contra 13,518436, +2,7%), mas a razão dp post/priori cai de 0,609555 para 0,454279.
- **Comparação com o \`h2g/educacao_star_turmas\`**, que é o outro lado do par de mesma base: lá a literatura também foi podada no F1. Os dois lados do par se comportam igual, e é o que se espera de uma priori que não acrescenta informação nova.

### Limites, os mesmos dos outros domínios

- **Marca P**: o número sai do Python aposentado.
- **Sensibilidade, não rodada oficial**: painel de 1 de 3, e o especialista é o autor do programa.
- **A exclusão do par de mesma base continua valendo** para quando o painel existir: a mesma pessoa não pode servir \`h2/educacao_star\` e \`h2g/educacao_star_turmas\`.


---

## 9. Quarto domínio preenchido, com a marca **P**: \`h2g/direito_teto_beneficio\`

Mesmo caminho, com uma diferença de pasta: este domínio vive no **\`h2g\`** do p05 e lê a base **\`dados-reais/h2b\`**. A semente de novo preservada — \`direito_teto_beneficio\` é o **segundo** na ordem alfabética dos dezoito domínios do h2g, então a chamada leva \`biologia_sachs\` antes dele para a semente continuar \`2027 + 100 × 1 = 2127\`.

### As três prioris (coeficiente log-log)

| prior | q05 | q25 | q50 | q75 | q95 | média | dp |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **pacote** | **0,00** | 0,20 | 0,35 | 0,50 | **0,75** | 0,350000 | 0,222390 |
| **literatura (p05)** | 0,22806681817823804 | 0,30 | 0,35 | 0,40 | 0,4719331818217619 | 0,350000 | 0,074130 |
| **elicitada (E1)** | **0,05** | 0,20 | 0,35 | 0,50 | **0,65** | 0,350000 | 0,222390 |

### A conferência que autoriza a terceira coluna

| rodada | o que a tabela do p05 diz | o que a rodada de hoje deu |
| --- | --- | --- |
| (a) pacote | aprovado; adotar; média posterior 0,579; razão dp 0,24 | aprovado no F4; adotar; média 0,578765; razão 0,242860 |
| (b) literatura | aprovado; adotar; média 0,506; razão 0,6 | aprovado no F4; adotar; média 0,505514; razão 0,602756 |

### As três rodadas

| rodada | status | EVPI (F1) | ação do F8 | P(adotar compensa) | média posterior | IC90 | razão dp post/priori |
| --- | --- | --- | --- | --- | --- | --- | --- |
| (a) **pacote** | aprovado no F4 | 0,087187 | adotar | 1,0 | 0,578765 | [0,491437; 0,665412] | 0,242860 |
| (b) **literatura** | aprovado no F4 | 0,029062 | adotar | 0,99875 | 0,505514 | [0,433538; 0,576837] | 0,602756 |
| (c) **elicitada (E1)** | aprovado no F4 | **0,087187** | adotar | **1,0** | **0,578765** | **[0,491437; 0,665412]** | **0,242860** |

### O achado: a elicitação reproduziu o pacote, e o funil saiu idêntico

A linha (c) é **igual à linha (a) até o último dígito** — status, ação, P(adotar compensa), média posterior, IC90 e razão dp. Não é coincidência de arredondamento: os **três quartis que o ajuste usa** (q25, q50, q75) são os mesmos no pacote e na elicitação — 0,20, 0,35 e 0,50 —, e o ajuste da Normal é por mínimos quadrados **nesses três**. O \`priori\` das duas rodadas sai com média 0,350000 e dp 0,222390, e daí tudo o mais é idêntico por construção.

O que difere são as **pontas**, e o ajuste as ignora: q05 0,05 contra 0,00 do pacote, e q95 0,65 contra 0,75. Elas aparecem só nos controles, e aí sim há diferença — a razão da largura 90% é **0,8201** na elicitada e **1,0000** no pacote.

**A leitura.** Neste domínio a elicitação formal **não trouxe priori nova: ela documentou a priori que já existia**. O pacote declara, na própria letra, "Meyer, Viscusi & Durbin (1995)… citados de memória, conferir; **nenhum especialista humano consultado**". Agora há um especialista, um pré-teste aprovado (habilidade 0,8585), uma justificativa escrita e duas notas assinadas — e os três quartis centrais que ele produziu são os mesmos que o pacote trazia. É a primeira **validação** de uma priori de pacote desta árvore por elicitação formal, e ela vale o que vale: mede consistência do autor consigo mesmo ao longo do tempo, e não concordância com gente de fora.

**Correção de uma frase da devolução.** Ao devolver os números, o facilitador escreveu que "o p05 já registrou que os dois lados desse par foram podados no funil". Isso **não** é o caso deste domínio: com a priori de literatura ele continuou **aprovado no F4**, e a tabela do p05 mostra \`adotar\` nas três rodadas. O que foi podado, no p05, foram \`h2/direito_prisoes\` (no F1) e \`h2g/educacao_star_turmas\` (no F1) — e a marca \`mesma base\` deste domínio continua valendo pelo motivo certo: a evidência não é independente. A frase fica corrigida aqui, e o especialista foi avisado.

### Limites

- **Marca P** e **sensibilidade, não rodada oficial**: painel de 1 de 3, e o especialista é o autor do programa.
- **O achado tem teto**: reproduzir o pacote não é confirmar a estimativa contra o mundo. É o autor chegando ao mesmo lugar por dois caminhos — o que é útil contra erro de transcrição e inútil contra erro de leitura da literatura.
- **O ajuste descarta as pontas**: as diferenças de q05 e q95 entre pacote e elicitação não entram no funil, porque a Normal é ajustada nos três quartis centrais. Fica registrado como limite declarado do ajuste, não como escolha deste domínio.


---

## 10. Quinto domínio preenchido, com a marca **P**: \`h1-real/c\`

O pipeline vive no **\`h1-real\`** do p05 e roda os três projetos (\`a\`, \`b\`, \`c\`) por \`p05_rodar.py\`, com semente \`2026 + 100 × i\`. Como o projeto \`c\` é o terceiro, a chamada tem de ser a completa (sem argumentos) para a semente dele continuar **2226**. As bases do H1 real são os cinco CSVs de \`dados-reais/\`, copiados para fora da árvore para o p05 rodar.

### As três prioris (frequência mensal da camada "100 xs 50")

| prior | q05 | q25 | q50 | q75 | q95 | Beta ajustada (a, b) | média implicada |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **(a) painel simulado do p05** | — | — | — | — | — | 2,327958 / 106,930083 | 0,021307 |
| **(b) fraca Beta(1; 1)** | 0,05 (derivado) | 0,25 | 0,50 | 0,75 | 0,95 (derivado) | 1,000000 / 1,000000 | 0,500000 |
| **(c) elicitada (E1)** | 0,01 | 0,04 | 0,08 | 0,15 | 0,30 | **1,168188 / 9,911852** | **0,105432** |

A elicitada tem mediana **seis vezes menor** que a fraca e 2,5 vezes maior que o painel simulado. E o ajuste que o Python aposentado faz dela — \`a = 1,168188\`, \`b = 9,911852\` — **confere com o ajuste Beta que eu escrevi em Rust**: \`a = 1,171038\`, \`b = 9,919921\`, uma diferença de 0,24% e 0,08%. Duas implementações independentes, mesmo resultado: é a segunda opinião do ajuste, e ela fecha.

### A conferência que autoriza a terceira coluna

| rodada | o que a tabela do p05 diz | o que a rodada de hoje deu |
| --- | --- | --- |
| (a) painel simulado | reprovado: modelo_vence_climatologia; razão dp post/priori 0,87 | reprovado (\`modelo_vence_climatologia\` falso no portão); razão **0,872232** |
| (b) fraca Beta(1; 1) | aprovado; razão 0,072 | aprovado; razão **0,072434** |

### As três rodadas

| rodada | portão \`modelo_vence_climatologia\` | aprovado | razão dp posterior / dp priori | média da priori |
| --- | --- | --- | --- | --- |
| (a) **painel simulado** | falso | **não** | 0,872232 | 0,021307 |
| (b) **fraca Beta(1; 1)** | verdadeiro | **sim** | 0,072434 | 0,500000 |
| (c) **elicitada (E1)** | verdadeiro | **sim** | **0,221084** | **0,105432** |

### O que a linha nova diz

- **A elicitação resgata o projeto**, como a fraca já resgatava: \`h1-real/c\` é reprovado pelo painel simulado do p05 e passa com a priori elicitada. O que muda é a **qualidade da priori**: a fraca passava com uma uniforme declarada sem fonte; a elicitada passa com uma Beta de mediana 0,08, construída por um especialista, com pré-teste aprovado e justificativa escrita.
- **A razão dp post/priori fica entre as duas**: 0,221084 contra 0,872232 do painel simulado e 0,072434 da fraca. Isto é a medida da informação: a priori elicitada é mais informativa que a uniforme e menos que o painel simulado, e o dado a aperta na proporção correspondente.
- **A prioridade de literatura deste domínio não existe** — o p05 declara, com todas as letras, \`SEM FONTE CONFERIDA\` e usa a fraca. Então este é o único dos cinco domínios em que a elicitação não compara com literatura nenhuma: ela **ocupa o lugar vazio** que o p05 tinha preenchido com uma uniforme, e o ocupou com um número.

### Limites

- **Marca P**: o número do funil sai do Python aposentado.
- **Sensibilidade, não rodada oficial**: painel de 1 de 3, e o especialista é o autor do programa. Este é o **último domínio da fila dele**; \`h2g/educacao_star_turmas\` fica reservado.
- **O que este domínio não resolve**: a elicitada passa o portão, mas isso **não** valida a frequência de 0,08 contra o mundo — a série dinamarquesa tem 108 meses e o próprio projeto declara as premissas indecidíveis (prêmio como múltiplo do \`burning cost\`, ξ incerto, meses tratados como independentes embora a frequência cresça na década). O que ficou provado é que a decisão muda com a priori, e que agora essa priori é uma declaração com nome e data em vez de uma uniforme.

