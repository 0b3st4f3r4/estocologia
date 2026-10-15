# Ponto 8: calibração (tolerância do ECE, deriva de rótulo, correção dos voos)

## Resultado

**(a) Com uma tolerância de ECE que depende de n, nenhum cofre binário do H3 passa, nem os aprovados.** Pela regra
declarada (ECE observado ≤ q95 do ECE sob calibração perfeita, com as mesmas previsões e o mesmo n), ciclones, voos
2023 e Rhode Island deixam de passar: o ECE deles é 1,4 a 2,3 vezes o q95 nulo, com valor-p ≤ 0,0005. A reprovação
de Toronto 1997–2002 também não era ruído: o ECE de 0,036 fica acima do q95 nulo de 0,032 (valor-p 0,008). Pela regra
secundária, max(0,03; q95), o quadro do H3 fica igual: os três aprovados continuam aprovados e os dois de Toronto
continuam reprovados. A lição é que 0,03 não é "o ruído": nos cofres grandes o ruído é de 0,002 a 0,006, e 0,03 é uma
tolerância prática que deixa passar descalibração detectável. A nula supõe rótulos independentes, o que subestima o
ruído nos voos (choques por dia e aeroporto).

**(b) Os testes de deriva de rótulo não teriam visto 2013 antes de forma útil.** Pelo critério declarado (um alarme
novo no cofre antes de 2013-10-16, data do primeiro alarme do sensor original), a resposta é não: o teste da média
prevista contra a taxa realizada (R1) liga no cofre só em 2013-11-06, e o BBSE (R2) em 2013-11-21. Mas o R1 já
estava ligado desde 2013-07-22 e ficou ligado na virada do corte; o problema é que ele fica ligado em 95% das
verificações do desenvolvimento de 2013 e em 85% das de 2023. Com a janela do disjuntor (3 000 voos, uns três dias),
a taxa de atraso oscila muito mais que 0,02 de um dia para outro, e o teste vira um alarme permanente que não separa
o 4º trimestre de 2013 de nenhum outro período. Nos fluxos de prisões e ciclones o R1 fica ligado em 0% a 19% das
verificações do cofre. O BBSE fica ligado em 99% a 100% do cofre nos voos, em 71% a 82% no Rhode Island e em Toronto
2021 e em cerca de 20% nos demais: a hipótese dele (classificador fixo) não vale com o modelo reajustado todo dia.

**(c) A correção aguentou o único desvio real que achei, mas não houve prova no cofre, e o estresse não discriminou.**
Achei dois anos novos de voos com o mesmo esquema: NY 2017 e Portland/Seattle 2014. Em nenhum dos dois o 4º trimestre
repetiu o desvio de 2013 no modelo cru (|p̄ − ȳ| de 0,008 e 0,018, abaixo de 0,02), então o teste principal não se
aplica; a previsão oficial passa no G2 nos dois cofres. No teste secundário, o único trimestre em que o modelo cru
desviou como em 2013 foi o 3º trimestre de 2017 (cru +0,028), e a correção o levou a +0,001. No estresse declarado
(injetar no cofre de 2023 a deriva de rótulo de 2013), a correção passa, mas o modelo cru também passa, porque o
reajuste diário em 60 dias absorve a deriva sozinho no nível do trimestre. Pela regra declarada, o estresse não foi
forte o bastante para discriminar. Isto é estresse, não prova.

## O que foi feito

A declaração foi escrita antes de qualquer análise: [DECLARACAO.md](DECLARACAO.md), SHA-256 em `DECLARACAO.sha256`
(13dbcd38…). Todo o código roda em cópias do H3 em `codigo/` (copiadas de `h3/estocologia`, `h3/fluxos` e
`h3/rodar.py`); os originais não foram editados. Os diários dos cofres vêm de
`/mnt/project-files/estocologia/h3/saida/<fluxo>/diario.parquet`, e o código confere cada um contra
`FORA_DO_GIT.sha256` antes de ler.

Arquivos:
- `ece_n.py`, saída `saida/ece_n.json` (item a);
- `rotulo.py`, saídas `saida/rotulo.json` e `saida/r1_serie_*.csv`, `saida/r2_serie_*.csv` (item b);
- `preparar_dados.py` (conversão dos voos novos), `parametros_estresse.py` (saída `saida/parametros_estresse.json`),
  `codigo/fluxos/voos_p08.py` (fluxos novos), `correcao.py`, saída `saida/correcao.json` (item c);
- `saida/replays/<fluxo>/`: resultados, barramento e especificações dos 4 replays. Os diários (> 2 MB) estão em
  `/mnt/project-files/estocologia/dados-reais/fundo/p08-calibracao/replays/`, com somas no `SHA256SUMS` de lá.

## (a) Tolerância de ECE que depende de n

Para cada cofre, 2 000 simulações de y* ~ Bernoulli(p̂) com as previsões oficiais do cofre, semente 20261015, e o ECE
calculado com a mesma função do H3 (`calibracao.ece`, 10 caixas de massa igual). Os ECE observados batem com os
publicados no H3. Fonte: `saida/ece_n.json`.

| cofre | n | ECE observado | q50 nulo | q95 nulo | q99 nulo | valor-p | 0,03 (H3) | T95 (principal) | max(0,03; q95) |
|---|---|---|---|---|---|---|---|---|---|
| voos 2013 (NY) | 84 301 | 0,0267 | 0,0032 | 0,0046 | 0,0052 | < 0,0005 | passa | **falha** | passa |
| ciclones | 5 766 | 0,0123 | 0,0058 | 0,0087 | 0,0099 | 0,0005 | passa | **falha** | passa |
| prisões Toronto 1997–2002 | 1 488 | 0,0363 | 0,0225 | 0,0320 | 0,0359 | 0,008 | falha | **falha** | falha |
| prisões Toronto 2020–2021 | 16 933 | 0,0668 | 0,0080 | 0,0115 | 0,0132 | < 0,0005 | falha | **falha** | falha |
| voos 2023 (NY) | 104 489 | 0,0079 | 0,0025 | 0,0035 | 0,0039 | < 0,0005 | passa | **falha** | passa |
| Rhode Island | 24 466 | 0,0048 | 0,0023 | 0,0034 | 0,0038 | < 0,0005 | passa | **falha** | passa |

"< 0,0005" quer dizer que nenhuma das 2 000 simulações chegou ao ECE observado. Os voos 2013 já falhavam no G2 pelo
|p̄ − ȳ|; aqui a coluna "0,03" olha só o ECE.

**Veredito (a).** Os cofres aprovados no H3 (ciclones, voos 2023, Rhode Island) **não** continuam aprovados pela
regra principal declarada: a calibração deles não é perfeita, e com n de 5 a 100 mil a diferença é detectável. Pela
regra secundária, continuam. O de Toronto 1997–2002 tinha ruído da ordem da tolerância, como dizia o H3 (q95 nulo
0,032), mas o ECE observado passa dele: a reprovação se mantém nas duas regras. O ECE menos a média nula (um ECE
"sem o viés do ruído") vai de 0,0024 (Rhode Island) a 0,0587 (Toronto 2021), em `saida/ece_n.json`.

## (b) Teste de deriva de rótulo

Dois testes rodados fora da linha sobre os diários, na ordem do replay, com a janela W e o intervalo m do disjuntor de
cada fluxo (voos 3 000/300, ciclones 1 500/100, prisões 400/50):
- **R1:** d = ȳ − p̄ nas W últimas previsões resolvidas; liga com |d| > 0,02 e |z| > 3 em 3 verificações seguidas,
  desliga com |d| < 0,01 em 3 seguidas;
- **R2 (BBSE, Lipton, Wang e Smola 2018):** taxa do alvo estimada sem rótulo, q̂ = (μ − FPR)/(TPR − FPR), contra a
  média prevista no alvo; mesma histerese.

Fonte: `saida/rotulo.json`; os alarmes do sensor original vêm de `h3/saida/<fluxo>/resultados.json`.

| fluxo | original: alarmes dev / cofre (1º no cofre) | R1: dev / cofre (1º no cofre) | R1 ligado no corte | R1 % ligado dev / cofre | R2: dev / cofre (1º no cofre) | R2 % ligado dev / cofre |
|---|---|---|---|---|---|---|
| voos 2013 | 0 / 1 (2013-10-16) | 7 / 3 (2013-11-06) | sim, desde 2013-07-22 | 95% / 96% | 2 / 1 (2013-11-21) | 97% / 99% |
| voos 2023 | 4 / 1 (2023-11-14) | 21 / 5 (2023-10-13) | sim | 85% / 75% | 1 / 0 | 100% / 100% |
| ciclones | 1 / 0 | 2 / 0 | sim | 41% / 19% | 2 / 0 | 41% / 21% |
| prisões 1997–2002 | 1 / 1 (2001-04-05) | 0 / 0 | não | 0% / 0% | 2 / 0 | 36% / 20% |
| prisões 2020–2021 | 3 / 0 | 4 / 0 | não | 16% / 0% | 9 / 6 (2021-07-06) | 87% / 82% |
| Rhode Island | 0 / 1 (2013-02-03) | 3 / 1 (2013-05-27) | não | 6% / 1% | 13 / 6 (2013-06-01) | 74% / 71% |

O R2 teve verificações degeneradas (TPR − FPR < 0,05 ou janela sem um dos rótulos), que foram puladas: 28 (voos 2013),
53 (voos 2023), 0, 0, 54 e 150. Essa regra de pular não estava na declaração; está relatada aqui.

**Veredito (b).** Pelo critério declarado, **não**: nenhum dos dois testes liga um alarme novo no cofre de 2013 antes
do sensor original (2013-10-16). O R1 estava ligado em setembro e na virada do corte, com d = +0,040 quando ligou em
2013-07-22, o que a declaração previa relatar. Mas ele fica ligado quase o tempo todo nos voos (95% das verificações
do desenvolvimento de 2013), e por isso não é um aviso: com a janela de 3 000 voos, ele mede a variação de um dia de
mau tempo, não uma deriva de trimestre. O desvio de 2013 também não foi um degrau: por mês, a média prevista menos a
taxa realizada (previsão oficial, `saida/correcao.json`, `retro_2013.meses`, coluna `citl_cru`) foi −0,014 em
outubro, +0,001 em novembro e −0,067 em dezembro. Quase todo o desvio do trimestre veio de dezembro. Um teste com
janela de três dias não separa isso do ruído de agosto (+0,046) ou de setembro (+0,070), que não reprovariam nada.

Sobre a premissa do ponto: o sensor original tem um canal de resíduo padronizado, (y − p)/√p(1−p), comparado por KS a
uma referência. Esse canal reage ao rótulo, e foi ele que disparou nos voos 2013 e 2023. O que falta nele é comparar
a taxa com a média prevista; o R1 faz isso e, com a janela do disjuntor, não ajudou.

## (c) A correção dos voos posta à prova

A correção é a do H3b: p' = σ(logit p + δ), δ ← δ + 0,002 (y − p'). Os fluxos novos herdam `Voos23` sem mudar modelo,
features, reserva, disjuntor, sensores ou correção.

**Dados novos achados na rede** (`/mnt/project-files/estocologia/dados-reais/fundo/p08-calibracao/FONTES.md`):
- NY 2017, pacote `nycflights` (github.com/jayleetx/nycflights): 303 748 voos de EWR, JFK e LGA, com tempo horário;
- Portland e Seattle 2014, pacote `pnwflights14` (github.com/ismayc/pnwflights14): 162 049 voos de PDX e SEA, com
  tempo horário;
- Houston 2011, pacote `hflights` (espelho cran/hflights): baixado e não usado, porque não tem horário programado dos
  cancelados nem tempo meteorológico.

Não achei `nycflights14` nem outros anos de NY no GitHub; o `anyflights` só baixa da BTS, que está bloqueada.

**Correção de leitura nos dados do noroeste, feita antes de rodar.** A declaração diz que o instante programado seria
montado de year, month, day, hour e minute. Ao converter vi que, no `pnwflights14`, hour e minute são da partida
real. O instante programado passou a ser (partida real − dep_delay) mod 24 h na data do voo. Os 857 voos cancelados
(0,53%) não têm horário nenhum e saíram; como cancelado conta como atraso, a taxa-base do noroeste fica um pouco
menor do que seria. Em NY 2017 o tempo meteorológico tem `precip` vazio em muitas horas; o fluxo trata vazio como 0.

### Cofres novos (teste principal)

Fonte: `saida/correcao.json`. CITL = p̄ − ȳ (o sinal negativo quer dizer mais atrasos que o previsto, como em 2013).

| cofre out–dez | n | ȳ | oficial: CITL | oficial: ECE | oficial: habilidade | cru: CITL | desvio como 2013 no cru? | G2 do oficial |
|---|---|---|---|---|---|---|---|---|
| NY 2017 | 75 166 | 0,197 | −0,0091 | 0,022 | +0,107 | −0,0081 | não | passa |
| PDX/SEA 2014 | 38 608 | 0,161 | +0,0004 | 0,016 | +0,053 | −0,0176 | não | passa |
| NY 2023 (H3b) | 104 489 | 0,130 | −0,0015 | 0,008 | +0,149 | +0,0121 | não | passa |

Nenhum cofre novo repetiu o desvio de 2013 no modelo cru, então **o teste principal não se aplica** a nenhum ano.

### Trimestres de 2014, 2017 e 2023 (teste secundário)

Em 12 trimestres, só um teve o cru com |CITL| ≥ 0,02: **NY 2017, 3º trimestre**, cru +0,0277, oficial +0,0012
(passa). O 3º trimestre de 2023 ficou perto (cru +0,0192, oficial +0,0053). Todos os 12 têm o oficial com
|CITL| ≤ 0,02. A tabela completa está em `saida/correcao.json` (`trimestres`).

**Informativo, por mês (não era critério).** Os meses de dezembro repetem o desvio de dezembro de 2013 (cru −0,067)
nos três anos:
- NY 2017: cru −0,050, oficial −0,024, ainda acima de 0,02;
- PDX/SEA 2014: cru −0,051, oficial +0,011;
- NY 2023: cru −0,011, oficial −0,001.

Outros meses com o cru acima de 0,04: 2017-09 (+0,043, oficial −0,017), 2023-05 (+0,046, oficial +0,008), 2023-06
(−0,059, oficial −0,001), 2023-08 (+0,075, oficial +0,008).

### Teste de estresse no cofre de 2023 (é estresse, não prova)

Deriva pura de rótulo, só nos voos com partida programada a partir de 2023-10-01: troca de y = 0 para y = 1 com
probabilidade f, semente 20261015 (`saida/parametros_estresse.json`). O desvio de 2013 vem do diário de 2013:
ȳ − p̄ = +0,026691, o mesmo publicado no H3.
- **S1, degrau:** f = 0,0307 em todo o cofre; 2 785 rótulos trocados de 104 570 voos.
- **S2, perfil de 2013:** f = 0,0165 em outubro, 0 em novembro (o desvio de novembro de 2013 foi negativo) e 0,0777 em
  dezembro; 2 677 trocados.

| cofre estressado | ȳ | oficial: CITL | oficial: ECE | oficial: habilidade | cru: CITL | cru: ECE | G2 oficial | G2 cru |
|---|---|---|---|---|---|---|---|---|
| S1 | 0,157 | −0,0018 | 0,014 | +0,093 | +0,0046 | 0,013 | passa | passa |
| S2 | 0,156 | −0,0029 | 0,015 | +0,104 | −0,0030 | 0,012 | passa | passa |

No S2, em dezembro, o cru fica em −0,055 (perto do −0,067 de dezembro de 2013) e o oficial em −0,006. No trimestre
inteiro, o reajuste diário do modelo em 60 dias já aprende a taxa nova, e o cru passa sozinho.

**Controle.** A parte do desenvolvimento dos dois replays de estresse (307 246 previsões antes do corte) é idêntica ao
diário do H3b em todas as colunas de previsão (modelo, modelo_cru, delta, origem). Só 2 rótulos diferem no S1, de voos
emitidos antes do corte e programados depois dele. Os replays rodaram com G3 limpo (0 leituras do futuro, 0 rótulos
já conhecidos, 0 estados do futuro) e no máximo 2,14 varreduras por tabela, exceto PDX/SEA 2014, com 3,13 varreduras
na pior tabela, acima do limite de 3 do G5. Isso não afeta as previsões. O K de cada fluxo novo é 1.

### Retrospectiva de 2013 (informativa; a correção foi desenhada depois de ver 2013)

Primeiro validei a réplica fora da linha da recursão de δ contra o diário de 2023: a diferença máxima de δ é 0,0031,
a média é 2,7e-5, e a diferença máxima da previsão é 0,00076. Aplicada a 2013, com as origens do diário mantidas, a
correção levaria o cofre de CITL −0,0267 para −0,0024 e o ECE de 0,0267 para 0,0238, passando no G2. Dezembro iria
de −0,067 para −0,001.

**Veredito (c).** A correção ainda não foi posta à prova num cofre que repetisse 2013, porque nenhum dos dois anos
novos repetiu o desvio no 4º trimestre. Nos trimestres e meses que repetiram, ela segurou o trimestre (2017-T3) e a
maior parte dos dezembros, mas não dezembro de 2017 (−0,024). O estresse declarado não discrimina: o modelo cru também
passa.

## Uma divergência no H3

O `h3/README.md` diz que no H3b "o δ variou de −0,52 a +0,43 ao longo do ano". No diário de 2023, esse é o intervalo
dentro do cofre; no ano inteiro, o δ vai de −1,01 a +1,25.

## Como reproduzir

A partir de `fundo/p08-calibracao/`, com pyarrow, duckdb e pyreadr instalados:

```
python3 ece_n.py                 # (a), segundos
python3 rotulo.py                # (b), segundos
D=/mnt/project-files/estocologia/dados-reais/fundo/p08-calibracao
python3 -I preparar_dados.py $D $D/csv
python3 parametros_estresse.py
# pasta de dados com h3b -> dados-reais/h3b e p08 -> $D/csv
mkdir -p /tmp/dados && ln -sfn /mnt/project-files/estocologia/dados-reais/h3b /tmp/dados/h3b && ln -sfn $D/csv /tmp/dados/p08
cd codigo && for n in voos17 voospnw14 voos23e1 voos23e2; do \
  ESTOCOLOGIA_DADOS=/tmp/dados ESTOCOLOGIA_SAIDA=../saida/replays python3 -W ignore rodar.py $n & done; wait; cd ..
python3 correcao.py              # (c)
```

Os quatro replays em paralelo levam cerca de 2 h em 4 núcleos.

**Reprodução feita.** `ece_n.py` e `rotulo.py` rodaram duas vezes, e os JSON têm o mesmo SHA-256. A segunda execução
dos quatro replays está relatada em "Segunda execução", abaixo.

## Limites

- A nula do ECE supõe rótulos independentes. Nos voos há choques comuns por dia e aeroporto, e a nula subestima o
  ruído. A regra T95 é, por isso, severa demais nos voos. Uma nula por blocos exigiria um modelo da dependência, que
  não fiz.
- R1 e R2 rodaram fora da linha e só detectam: não liguei nenhuma ação a eles. Os limiares (0,02 e 0,01) e as janelas
  (as do disjuntor) foram declarados e não ajustados. Uma janela de um mês talvez separasse dezembro de 2013, mas isso
  seria escolhido depois de ver o resultado, e não rodei.
- O BBSE supõe p(x|y) fixo e o classificador fixo; aqui o modelo se reajusta todo dia.
- Os anos novos não são o mesmo aeroporto em todos os casos (PDX/SEA), e o noroeste perdeu os cancelados.
- O estresse é uma deriva de rótulo pura, sem sinal nas features. É o caso mais difícil para o modelo e o mais fácil
  para uma correção de intercepto, e mesmo assim o reajuste diário o absorveu no trimestre.
