# Ponto 12 (SHELF): protocolo de elicitação e formulário do painel

**Estado: escrito antes de qualquer elicitação. Nenhum especialista foi consultado, nenhuma previsão foi corrigida,
nenhum campo de formulário está preenchido e nada está assinado.** Este arquivo é instrumento: fixa o pré-teste, a
composição do painel, a forma da elicitação, os controles e a regra de entrada como fonte. O que os especialistas
responderem entra em `formularios/`; a comparação com a priori de literatura entra em
`formularios/comparacao.md`.

## Resultado

Não há resultado de análise nesta frente, e não há como haver: tudo aqui depende de pessoas. O que existe hoje:

| artefato | o que é | estado |
|---|---|---|
| `protocolo.md` (este) | protocolo completo: pré-teste de calibração, painel, elicitação, controles, regra de entrada, domínios e quartis de partida | escrito; critérios fixados |
| `formularios/h2__*.yaml`, `formularios/h2g__*.yaml`, `formularios/h1-real__c.yaml` | um formulário em branco por domínio (18 blocos de especialista, 6 domínios) | em branco |
| `formularios/prioris_elicitadas.yaml` | **exemplo** preenchido com os quartis da priori de **literatura** do p05, para fixar o formato e servir de linha de comparação | preenchido (não é elicitação) |
| `formularios/comparacao.md` | esqueleto da comparação: prioris lado a lado, decisão antes e depois, critério de K declarado | esqueleto |
| `../../governanca/RELEITURA.md` (Bloco K) | formulário de releitura dos 21 pareceres, com os 21 SHA-256 conferidos | em branco |

Os números de partida deste protocolo estão em "Domínios a elicitar" (seção 6), todos copiados de
`fundo/p05-prioris/prioris/h2/literatura.yaml`, `.../h2g/literatura.yaml` e `.../h1-real/literatura.yaml`,
com o caminho citado, e conferidos no arquivo.

## Regra de casa que rege esta frente

- **Declaração antes, com hash.** Antes do primeiro especialista responder qualquer coisa, este protocolo é congelado:
  `DECLARACAO.md` + `DECLARACAO.sha256` (SHA-256 **deste** arquivo e a hora UTC). Depois de congelado, este
  arquivo não é reescrito: qualquer mudança de critério entra como **declaração nova**, com hash próprio, e conta no K.
- **Este arquivo ainda não está congelado, e o hash não é preenchido por quem o escreveu.** Quem congela é quem
  assina. Hash: ______ (em branco).
- **Nada é real até estar escrito com nome e data.** O que não estiver no formulário não aconteceu: combinação verbal de
  quartis não entra na elicitação.
- **Falha de execução e desvio de desenho são resultado**, não motivo de nova tentativa silenciosa: pré-teste reprovado,
  especialista que desiste, controle estourado e devolução não confirmada são relatados como estão.

## 1. O que este ponto muda e o que não muda

Muda **uma** coisa: a distribuição a priori do coeficiente da chave (o bloco `shelf` do pacote de domínio, e a
priori correspondente no H1 real). Não muda absolutamente mais nada: dados, unidade independente, ambientes e cofre,
DAG, conjunto de ajuste, o código do funil (F1 a F4 e F8), os custos e benefícios da decisão, as sementes e os
limiares dos portões.

A comparação é a mesma do p05: categoria de status (podado no F1/F2/F3, reprovado no F4 com a lista dos portões que
falham, aprovado), ação do F8, média e IC90 da posterior da chave, razão dp posterior / dp priori, e o deslocamento da
média posterior em unidades do dp posterior da rodada (a) — `|m_elicitada − m_pacote| / dp_pacote`.

O que é diferente do p05: lá a priori era leitura de literatura conferida online ("nenhum especialista humano
consultado", dito no próprio `fundo/p05-prioris/DECLARACAO.md`); aqui é o consenso de um painel com pré-teste. Por
isso a elicitação só vale como fonte se (i) cada especialista passar no pré-teste da seção 2 e (ii) houver no painel um
especialista de fora da área do programa (seção 3).

## 2. Pré-teste de calibração: 20 previsões já resolvidas

Antes de elicitar o domínio, **cada** especialista faz o pré-teste de calibração dele. Sem passar, ele não entra como
fonte (seção 2.4).

### 2.1 Os 20 itens

- **20 itens binários, já resolvidos**, da mesma classe de pergunta do domínio (a mesma forma do desfecho: sim/não de um
  evento do tipo do domínio), com resultado conhecido **antes** do pré-teste acontecer.
- **Fora do cofre do domínio.** Os itens não podem sair do cofre (ambiente reservado) do domínio que vai ser elicitado,
  sob pena de gastar o cofre e de ensinar o resultado do domínio ao painel. Se não houver 20 itens resolvidos fora do
  cofre, isso é declarado na hora, o pool é montado com o que houver (mínimo 20 mesmo assim, mesmo que contaminado) e o
  domínio é marcado como **contaminado** no relatório.
- **Sorteio declarado**: semente e lista ficam na declaração com hash, antes de rodar. O especialista vê os 20 enunciados
  e nada mais; o gabarito fica com o facilitador.
- **Sem consulta.** É pré-teste de calibração: o especialista responde de cabeça, em 20 minutos, sem buscar fonte.
- **Cada item recebe p ∈ (0, 1)** — 0 e 1 exatos são recusados, porque não são previsão, e sem previsão não há Brier.
- **A unidade do item é a do pool, não a do domínio**: o pré-teste mede calibração, não conhecimento do domínio. O
  resultado dele não entra na priori.

### 2.2 As duas medidas: Brier e curva

- **Brier**, item a item: `Brier = média de (p − y)^2`, e a **habilidade** contra a climatologia do próprio pool,
  `habilidade = 1 − Brier / Brier_clima`, com `Brier_clima` pela taxa-base dos 20 gabaritos — a mesma
  definição que o H3 usa (`h3/estocologia/calibracao.py`, campo `habilidade_vs_climatologia`).
- **Curva de calibração** em **10 caixas de massa igual** (o `ece` do H3, mesmo arquivo), com a média prevista e a
  taxa observada de cada caixa, o **ECE** e a **decomposição de Murphy** (`REL`, `RES`, `UNC`, função
  `murphy`, mesmo arquivo). A curva é o que mostra a forma do erro; o Brier sozinho esconde se o erro é viés,
  excesso de confiança ou falta de resolução.
- **O que se relata do pré-teste**: os 20 pares `(p, y)`, o Brier, o `Brier_clima`, a habilidade, o ECE, a
  decomposição e as 10 caixas. Nada agregado sem o detalhe.

### 2.3 Critério de aprovação (declarado aqui, antes de rodar)

Passa quem cumpre os quatro:

1. **habilidade > 0** — Brier abaixo da climatologia do próprio pool (prever pior que a taxa-base é reprovação, mesmo
   que por pouco);
2. **ECE ≤ 0,10** nas 10 caixas de massa igual;
3. **nenhuma caixa com n ≥ 4 e |média prevista − taxa observada| > 0,25** (sem seção sistemática);
4. **curva fracamente monótona** — a média prevista da caixa de cima acima da média prevista da caixa de baixo.

Os quatro limiares são default do protocolo: 0,10, 0,25 e a monotonicidade fraca existem para não deixar o pré-teste
sem régua, e só mudam por declaração nova, com hash, contando no K. Os quatro vão escritos, com a semente e os 20 itens,
na declaração congelada antes de rodar.

### 2.4 Quem não passa não entra como fonte

- **Reprovado no pré-teste não elicita.** O especialista pode continuar na mesa como debatedor, mas os quartis dele
  **não** entram no consenso, e o formulário dele fica marcado `pre_teste: reprovado`, com os números que o
  reprovaram.
- **Domínio com menos de 3 aprovados fica sem painel.** O domínio é relatado como **não elicitado** — nunca completado
  com quem reprovou, nunca substituído pela priori de literatura do p05 sem dizer. Com 2 aprovados, o consenso deles é
  relatado com o n = 2 e a ressalva, e **não** substitui a priori do pacote na rodada oficial (fica como sensibilidade).
- **Domínio cujo painel reprovado em bloco** (por exemplo, os três abaixo do limiar) é resultado: diz que aquele
  desfecho não tem quem o preveja calibradamente, e é isso que se relata.

## 3. Painel

- **Três especialistas por domínio**, com **um de fora da área do programa**. "De fora" quer dizer duas coisas ao mesmo
  tempo, e as duas ficam escritas no formulário:
  - **de fora do programa**: não é autor do livro nem do artigo, não é revisor, não participou da auditoria nem das
    frentes do `fundo/` — alguém que não tem interesse no resultado;
  - **de fora da área do domínio**: a formação e o ofício não são os do domínio elicitado (para os domínios de direito,
    um jurista; para os de educação, um educador etc.), de modo que o de fora sirva de controle contra a linguagem e as
    convenções do próprio programa.
  O de fora é o controle principal: ele não sabe o que o programa espera e não reconhece a resposta "certa".
- **Facilitador**: conduz o pré-teste, guarda o gabarito, faz a devolução e não elicita. Não tem voto no consenso.
- **Exclusões declaradas** (quem está fora do painel daquele domínio, com motivo escrito): quem viu a saída do domínio
  (`h2/saida/<dom>`, `h2g/saida/<dom>`, `h1-real/saida/<projeto>`); quem escreveu o pacote
  (`<h>/dominios/<dom>/pacote.yaml`) ou o `preparo.py` dele; quem já emitiu previsão no diário daquele
  domínio; e quem serviu o outro lado de um par de **mesma base** — `h2/educacao_star` e
  `h2g/educacao_star_turmas` (Krueger 1999, a mesma base AER_STAR) e `h2g/direito_teto_beneficio` e
  `h1-real/b` (o mesmo esquema de base declarado no p05, "mesma base") não podem ter o mesmo especialista nos dois
  lados: a dupla contagem já é um problema declarado do p05, e o painel não a repete.
- **Registro**: cada especialista recebe um código fixo (E1, E2, E3) que vale para todos os domínios dele; o nome fica na
  folha de assinatura da declaração, e o código é o que circula nos formulários e na devolução (o consenso é anônimo
  entre eles).
- **Painel completo é 18 pessoas** (6 domínios × 3), menos as exclusões por acumulação: um mesmo especialista pode
  servir mais de um domínio desde que não haja conflito declarado.

## 4. Elicitação: mediana, quartis e extremos, com devolução

A elicitação é **na unidade do domínio**, com a quantidade escrita no formulário de cada domínio (seção 6). A ordem é
fixa, e cada passo é gravado antes do passo seguinte:

1. **Mediana primeiro.** `q50`: o valor tal que o especialista acha igualmente provável estar acima e abaixo.
2. **Depois os quartis.** `q25` e `q75`, definidos como as apostas em que os dois lados valem o mesmo
   (equivalente em apostas, não como "intervalo de confiança").
3. **Depois os extremos.** `q05` e `q95` — os 5% de cada ponta, ditos explicitamente como tais.
4. **Justificativa curta e fonte**, em uma a três linhas, escrita pelo especialista (é o que permite ler depois se o
   consenso ficou informado ou ficou plano).
5. **Devolução (primeira rodada).** O facilitador devolve ao especialista os **números dele**, a **faixa anônima do
   grupo** (mínimo, mediana e máximo dos q50, sem nome) e, se houver, a **priori de literatura do p05** como referência
   externa declarada — nunca a saída do funil nem o resultado do domínio.
6. **Confirmação.** O especialista responde uma de duas coisas, por escrito: **confirma** os números dele, ou **ajusta
   para X**, com o motivo. O que entra no consenso é o número confirmado, e a rodada 1 fica no formulário.
7. **No máximo duas rodadas de devolução.** Na segunda, os números já confirmados não voltam: só o que ficou em
   divergência (extremos estreitos demais, mediana que mudou de ordem de grandeza, controle da seção 5 estourado).
   Divergência que não se resolve fica relatada como divergência, com os dois números.

### 4.1 Consenso

- **Regra declarada: mediana ponto a ponto dos quartis** dos especialistas que passaram no pré-teste, na ordem
  `q05, q25, q50, q75, q95`, com os três valores individuais preservados ao lado do consenso, sempre.
- O H1 simulado do repositório usa **média** dos quartis ("como no RIO", `h1/estocologia/agentes.py`, função
  `f7_elicitar`). A troca da média pela mediana ponto a ponto está declarada **aqui, antes**, e é registrada como
  tal: mediana da amostra pequena é o que resiste a um especialista extremo, e a média não.
- **Ajuste da priori:**
  - **gaussiana** (`h2/desenvolvimento_preston`, `h2/educacao_star`, `h2g/educacao_star_turmas`,
    `h2g/direito_teto_beneficio`): Normal por mínimos quadrados nos quantis q25/q50/q75
    (`ajustar_normal`, `h2f/estocologia/dominio.py`);
  - **binomial** (`h2/direito_prisoes`): os quartis são razão de chances e viram log antes do ajuste
    (`priori_da_chave`, mesmo arquivo; `escala: razao_de_chances` e `taxa_base` 0,8 no pacote);
  - **H1 real** (`h1-real/c`): a quantidade é uma frequência, e o painel do H1 ajusta **Beta** aos três quartis
    (`ajustar_beta_quartis`, `h2f/estocologia/estat.py`), que é o que o H1 simulado já fazia.
- **Sempre relatar**: os quartis individuais, o consenso, o ajuste (média/mediana e dp, ou a e b), o
  `erro_max_quartis` e a razão largura 90% elicitada / implicada. Ajuste que não fecha é resultado, não motivo de
  reescolher os quartis.

## 5. Controles: âncora, ordem e excesso de confiança

**(a) Ordem.** Metade do painel recebe a ordem mediana → quartis → extremos e a outra metade a ordem inversa
(extremos → quartis → mediana), sorteio declarado antes. Lê-se a mediana dos dois subgrupos: diferença maior que
**0,20 dp** (em unidades do dp ajustado do grupo) é efeito de ordem relatado; se estourar, o consenso usa a **mediana
das duas ordens** e o efeito entra no relatório.

**(b) Âncora.** Depois do primeiro passe, o facilitador apresenta **um** valor sugerido `S` por domínio (sorteado
entre valores plausíveis, com semente declarada, fora da faixa dos quartis já emitidos) e pede
`P(θ > S)`. Duas leituras:
- com `S` = mediana do próprio especialista, `P(θ > S)` declarado deve ficar em **0,50 ± 0,10**; fora disso o
  especialista rediz a mediana;
- com `S` = mediana anônima do grupo, registra-se o **deslocamento da mediana** do especialista; acima de
  **0,20 dp**, o deslocamento é relatado e o número original fica como o confirmado.
- **Ancoragem proibida no primeiro passe**: nenhum número do domínio entra na mesa antes de o especialista escrever os
  cinco quantis — nem a priori do pacote, nem os quartis de literatura do p05, nem a saída do funil, nem a média
  posterior. Os quartis de partida ficam em `prioris_elicitadas.yaml` e só aparecem na devolução (passo 5 da
  seção 4), declarados como referência externa.

**(c) Excesso de confiança.** Razão **largura 90% elicitada / largura 90% implicada** pelos quartis ajustados
(`razao_largura_90_elicitada_sobre_implicada`, `ajustar_normal`). Declarado: razão **≥ 0,75**. Abaixo
disso, os extremos estão estreitos demais para a própria mediana e quartis do especialista; ele re-elicia **só
q05/q95** na devolução, e os dois conjuntos ficam no formulário. O mesmo controle é lido no consenso, e o valor fica no
relatório — é ele que diz se a priori elicitada é mais informativa do que os dados e a literatura sustentam.

## 6. Domínios a elicitar e quartis de partida

Os seis domínios abaixo são a lista fechada desta frente. "Quartis de partida" é a priori de **literatura do p05** — o
ponto de comparação, não a priori do painel. Ela **não** aparece nos formulários em branco e **não** é mostrada no
primeiro passe (controle de âncora, seção 5b).

| domínio | horizonte | área | família / escala | quantidade elicitada (unidade) | arquivo do pacote |
|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | H2 | Desenvolvimento | gaussiana | anos de expectativa de vida por unidade de log natural do PIB per capita | `h2/dominios/desenvolvimento_preston/pacote.yaml` |
| `h2/direito_prisoes` | H2 | Direito | binomial, razão de chances (log no ajuste; `taxa_base` 0,8) | razão de chances de liberação com notificação, presos negros contra brancos | `h2/dominios/direito_prisoes/pacote.yaml` |
| `h2/educacao_star` | H2 | Educação | gaussiana | efeito da turma pequena na soma de leitura + matemática (pontos SAT) | `h2/dominios/educacao_star/pacote.yaml` |
| `h2g/educacao_star_turmas` | H2g | Educação | gaussiana | efeito da turma pequena no escore padronizado (dp das turmas regulares) | `h2g/dominios/educacao_star_turmas/pacote.yaml` |
| `h2g/direito_teto_beneficio` | H2g | Direito (regra de indenização trabalhista) | gaussiana | elasticidade da duração do afastamento em relação ao benefício semanal (log-log) | `h2g/dominios/direito_teto_beneficio/pacote.yaml` |
| `h1-real/c` | H1 real, projeto C | Seguros / cauda | frequência (Beta), sem família de GLM | frequência mensal de um mês atingir a camada "100 xs 50" | `h1-real/projetos/c_real.py` |

**Quartis de partida (priori de literatura do p05), copiados e conferidos:**

| domínio | q05 | q25 | q50 | q75 | q95 | fonte no p05 |
|---|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | −2,746195205080443 | −0,21995975644268073 | 1,536 | 3,291959756442681 | 5,818195205080443 | `prioris/h2/literatura.yaml`, Pritchett & Summers (1996), R1 (e = 1,536; s = 1,302; dp = 2s) — independente |
| `h2/direito_prisoes` | 0,5944060189976956 | 0,6578947368421053 | 0,7059781184061313 | 0,7575757575757576 | 0,8384926931067868 | `prioris/h2/literatura.yaml`, Kochel, Wilson & Mastrofski (2011), R2 (faixa 1,32–1,52 na RC, invertida) — independente |
| `h2/educacao_star` | 1,3021781198967624 | 4,860110016547187 | 7,333191149998485 | 9,806272283449783 | 13,364204180100208 | `prioris/h2/literatura.yaml`, Krueger (1999), R3 (ponto 7,333; dp = 3,667) — **mesma base** (AER_STAR) |
| `h2g/educacao_star_turmas` | 0,017804398806321772 | 0,06645122940943404 | 0,10026513098524002 | 0,134079032561046 | 0,1827258631641583 | `prioris/h2g/literatura.yaml`, Krueger (1999), R3 (ponto 0,1003; dp = 0,05013) — **mesma base** (AER_STAR) |
| `h2g/direito_teto_beneficio` | 0,22806681817823804 | 0,3 | 0,35 | 0,4 | 0,4719331818217619 | `prioris/h2g/literatura.yaml`, Meyer, Viscusi & Durbin (1995), R2 (faixa [0,3; 0,4]) — **mesma base** |
| `h1-real/c` | 0,05 (derivado) | 0,25 | 0,5 | 0,75 | 0,95 (derivado) | `prioris/h1-real/literatura.yaml`: **sem fonte conferida**, usa a fraca Beta(1, 1), que publica só q25/q50/q75; q05 e q95 são **derivados** por mim da própria Beta(1,1) (uniforme) e estão marcados como derivados |

Notas de leitura da tabela:

- As regras R1, R2, R3 e a conversão de binomiais para o log da razão de chances são as fixadas na declaração do p05
  (`fundo/p05-prioris/DECLARACAO.md`); elas explicam a diferença entre estes quartis e os do pacote, que estão em
  outra conversão (o pacote de `h2/direito_prisoes`, por exemplo, traz 0,42–1,05 na RC).
- `h2/educacao_star`, `h2g/educacao_star_turmas` e `h2g/direito_teto_beneficio` são **mesma base** do
  domínio: o que eles medem não é evidência independente, e por isso o painel não pode ter a mesma pessoa dos dois
  lados do par (seção 3).
- `h2/direito_prisoes`: os quartis entram em razão de chances e o ajuste toma o log.

### 6.1 O que o p05 já observou com a priori de literatura

Isto **não** é resultado deste ponto: é o que a rodada (b) do p05 mediu, e está aqui porque é o que a comparação de
`formularios/comparacao.md` vai ler. Quem compara lê; quem elicita não vê antes de responder.

| domínio | status (a) pacote → (b) literatura | ação do F8 (a) → (b) | média posterior (a) → (b) | desloc. (b), em dp da (a) | K de hoje (`resultados.json`) |
|---|---|---|---|---|---|
| `h2/desenvolvimento_preston` | aprovado → aprovado | adotar → **manter** | 5,09 → 4,81 | 0,48 | K = 1 |
| `h2/direito_prisoes` | reprovado (modelo_vence_climatologia) → **podado no F1** (EVPI < custo) | — → — | −0,73 → — | — (podado antes do F4) | K = 1 |
| `h2/educacao_star` | aprovado → **podado no F2** (a unidade não informa mais que a priori) | adotar → — | 13,5 → — | — | K = 1 |
| `h2g/educacao_star_turmas` | aprovado → **podado no F1** (EVPI < custo) | manter → — | 0,148 → — | — | K = 3 |
| `h2g/direito_teto_beneficio` | aprovado → aprovado | adotar → adotar | 0,579 → 0,506 | **1,4** | K = 3 |
| `h1-real/c` | reprovado (modelo_vence_climatologia) → **aprovado** | exposição 0,4 → **0,0** | — | — (o `comparar.py` não calcula deslocamento no H1) | sem campo `K` no `resultados.json` |

Fonte: `fundo/p05-prioris/tabela.md` e `fundo/p05-prioris/README.md` (seção "As cinco mudanças de (b)"); a
coluna K vem do campo `K` de `h2/saida/<dom>/resultados.json`, `h2g/saida/<dom>/resultados.json`.
Duas dessas mudanças foram de **decisão**, não de número: `h2/desenvolvimento_preston` (adotar → manter) e
`h1-real/c` (exposição 0,4 → 0,0). É o tamanho do que está em jogo quando a priori passa a vir de gente.

## 7. Como registrar

- **Um formulário em branco por domínio**: `formularios/<horizonte>__<domínio>.yaml`, com três blocos de
  especialista (E1, E2, E3), o bloco de pré-teste de cada um, a elicitação, os controles, a devolução e o consenso.
- **Nada é reescrito**: ajuste de quartis depois da devolução entra como linha nova, datada e com autor, dentro do
  próprio bloco; o que ficou confirmado na rodada 1 permanece legível.
- **O consenso e o ajuste** vão para `formularios/prioris_elicitadas.yaml` (hoje esse arquivo traz o exemplo da
  literatura, marcado como exemplo) e a comparação é preenchida em `formularios/comparacao.md`.
- **Recursos**: qualquer script desta frente roda com `OMP_NUM_THREADS=1` e no máximo 2 processos em paralelo — a
  máquina é compartilhada. A parte de gente não consome máquina; a de comparação consome.
- **Quem prepara formulário não assina nada por antecipação**: nenhum campo é preenchido por antecipação, e nenhuma
  leitura do repositório é apresentada como resposta de especialista.
- **Commit e push**: quem prepara não commita nem empurra (regra da tarefa); o congelamento com hash e o commit são de
  quem assina.
