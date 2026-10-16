# Declaração de itens do pré-teste — \`h2/direito_prisoes\`

**Congelada em 2026-10-16**, antes de o especialista responder qualquer coisa. É a "lista com hash" que o
protocolo (seção 2.1) exige, por domínio.

| o que | valor |
| --- | --- |
| pool de origem | \`fundo/p12-shelf/preteste/direito_prisoes.yaml\`, 38 itens (21 \`sim\` e 17 \`nao\`), 15 descartados, SHA-256 \`080ef700cba064e7c8d5fe11d6d002b4b82b3584773e52821e08cd61a7e84686\` |
| semente do sorteio | \`2026\` |
| regra do sorteio | **estratificado**: embaralha cada estrato de resolução com o \`SplitMix64\` do repositório e toma 10 itens de cada, depois embaralha os 20 para a ordem de apresentação |
| ficou fora do estrato \`nao\` | \`D25\` (o estrato tinha 11 itens conferidos e o desenho pede 10 de cada) |
| gabarito | fica com o facilitador; o especialista vê **só os enunciados** desta declaração |

## Declaração 2 do domínio — o desenho do sorteio, e por que ele mudou depois de ver o pool

O protocolo (seção 2.1) fala em sorteio com semente declarada, e não fixa estratificação. O primeiro pool deste
domínio saiu com **21 itens \`sim\` contra 3 \`nao\`** — taxa-base 0,875 —, e isso foi **visto** antes de qualquer
administração. Com aquela taxa-base a climatologia do próprio pool iria a 0,875 e quem respondesse 0,9 em tudo
ficaria com habilidade perto de zero: o pré-teste mediria quase nada.

Declarado, então: o pool foi reequilibrado (foram acrescentados 14 itens de resolução \`nao\`, ids D25 a D38, sem
tocar nos 24 primeiros) e o sorteio passa a ser **estratificado, 10 \`sim\` e 10 \`nao\`**. Isto é mudança de desenho
**depois de ver um resultado do próprio instrumento**, então conta: o K da frente passa de **3** para **4**.

## O filtro de gabarito, declarado

Os 20 itens abaixo não saíram do pool inteiro: saíram dos **21 itens cujo gabarito o facilitador reproduziu na
fonte primária** (10 \`sim\` e 11 \`nao\`), com requisição real e leitura do número na página ou no PDF baixado
(\`pdftotext\` quando é PDF; cabeçalho de navegador quando o BJS servia um stub de bloqueio). Item cujo gabarito o
facilitador **não** conseguiu reproduzir ficou fora do sorteio, e não entra por confiança no executor. A evidência
item a item:

| item conferido | o que a fonte primária diz, na letra |
| --- | --- |
| D05 | PPI 2024: 'The U.S. incarcerates over 1.9 million people' |
| D06 | Eurostat: 'around 509 000 [prisoners] in 2024' |
| D07 | INEGI: 'había 233 277 personas privadas de la libertad' |
| D09 | UNODC: '11.2 millones de personas privadas de libertad' |
| D13 | Eurostat: '113 prisoners per 100 000 inhabitants in the EU in 2024' |
| D15 | INEGI: '37.3 % ... no contó con una sentencia' |
| D16 | UNODC: '142 personas privadas de libertad por cada 100,000 habitantes' |
| D18 | Sentencing Project: 'In 12 states, more than half the prison population is Black' |
| D21 | Eurostat occupancy: 'highest overcrowding EU rates in 2024 ... Cyprus (227.6)' |
| D23 | INEGI: 'para las mujeres este porcentaje fue de 46.9 %' |
| D20 | Eurostat: 'detainees with a foreign citizenship (21.4%) ... than in 2023 (20.7%)' |
| D25 | BJS ji22st: 'local jails spent an average of 32 days in custody' |
| D26 | BJS p22st: '62 per 100,000 female adult U.S. residents were serving' |
| D27 | BJS cpus22st: '5,407,300 ... (1,827,600) were incarcerated in state or federal' |
| D28 | Eurostat: '1 out of 18 adult prisoners were women' (5,5%) |
| D30 | BJS ji22st, tabela: 76,7% dos presos por felony em 2022 |
| D31 | INEGI: '43 553 993 799.0 pesos' para 233 277 pessoas |
| D32 | BJS fjs22: 'median number of days from the receipt of a matter ... was 60 days' |
| D34 | BJS msfp0119st: 'a rate of 259 per 100,000' |
| D36 | Eurostat occupancy: 'France (129.3)' |
| D38 | INEGI: '100 fallecieron' em incidentes nos centros penitenciários |

Os outros 17 itens do pool (13 antigos e 4 novos) **não** foram conferidos pelo facilitador — o executor os
conferiu, com URL citada — e por isso não entram neste pré-teste. Fica declarado, e não escondido.

## Os 20 itens, na ordem em que o especialista os vê

| # | id | enunciado | dificuldade declarada |
| --- | --- | --- | --- |
| 1 | D05 | No ano-base 2024, o número total de pessoas presas nos Estados Unidos, somando prisões estaduais e federais, cadeias locais, detenção de imigrantes, unidades juvenis e as demais formas de confinamento, era maior que 1,5 milhão? | facil |
| 2 | D32 | No ano-base fiscal de 2022, o tempo mediano entre o recebimento da investigação e a decisão do Ministério Público federal dos Estados Unidos de acusar ou declinar do caso era maior que 90 dias? | media |
| 3 | D27 | No ano-base 2022, mais de dois terços das pessoas sob supervisão correcional adulta nos Estados Unidos estavam encarceradas em prisões ou cadeias locais? | facil |
| 4 | D16 | No ano-base 2021, a taxa mundial de pessoas privadas de liberdade era menor que 150 por 100.000 habitantes? | media |
| 5 | D31 | No ano-base 2023, o gasto anual por pessoa privada de liberdade nos centros penitenciários estaduais e federais do México era maior que 300.000 pesos? | media |
| 6 | D18 | No ano-base 2019, mais de 10 estados dos Estados Unidos tinham mais da metade da população carcerária estadual formada por pessoas negras? | dificil |
| 7 | D06 | No ano-base 2024, o número de prisioneiros na União Europeia era maior que 400.000? | facil |
| 8 | D21 | No ano-base 2024, o país da União Europeia com a maior taxa de ocupação carcerária estava acima de 200% da capacidade oficial das prisões? | dificil |
| 9 | D13 | No ano-base 2024, a taxa de prisioneiros por 100.000 habitantes na União Europeia era menor que 150? | media |
| 10 | D28 | No ano-base 2024, as mulheres eram mais de 10% dos prisioneiros da União Europeia? | facil |
| 11 | D34 | No ano-base 2019, a taxa de mortalidade nas prisões federais operadas pelo Bureau of Prisons dos Estados Unidos era maior que 300 óbitos por 100.000 presos? | dificil |
| 12 | D30 | No ano-base 2022, menos da metade das pessoas custodiadas nas cadeias locais dos Estados Unidos respondia por crime grave (felony)? | media |
| 13 | D09 | No ano-base 2021, a população mundial privada de liberdade era maior que 10 milhões de pessoas? | facil |
| 14 | D07 | No ano-base 2023, o número de pessoas privadas de liberdade nos centros penitenciários estaduais e federais do México era maior que 200.000? | facil |
| 15 | D38 | No ano-base 2023, mais de 200 pessoas privadas de liberdade morreram em incidentes nos centros penitenciários e centros especializados do México? | dificil |
| 16 | D23 | No ano-base 2023, mais de 45% das mulheres privadas de liberdade nos centros penitenciários estaduais e federais do México estavam sem sentença? | dificil |
| 17 | D36 | No ano-base 2024, a França tinha taxa de ocupação das prisões acima de 150% da capacidade oficial? | dificil |
| 18 | D26 | No ano-base 2022, a taxa de encarceramento de mulheres adultas nos Estados Unidos era maior que 100 presas por 100.000 residentes adultas? | facil |
| 19 | D15 | No ano-base 2023, mais de um terço das pessoas privadas de liberdade nos centros penitenciários estaduais e federais do México não tinha sentença? | media |
| 20 | D20 | No ano-base 2024, mais de 25% dos prisioneiros da União Europeia tinham nacionalidade estrangeira no país onde estavam detidos? | dificil |

## O que o especialista recebe e o que ele devolve

- Recebe: os 20 enunciados, o pedido de \`p em (0, 1)\` para cada um (0 e 1 exatos são recusados) e a instrução de
  responder de cabeça, sem consulta, em 20 minutos.
- Devolve: 20 probabilidades.
- O que o facilitador faz: Brier item a item, climatologia do pool (0,553), habilidade, ECE em 5 caixas de massa
  igual (o portão ajustado da declaração 1), a curva observada e a decomposição de Murphy, com o detalhe item a item
  ao lado dos agregados.

## Limites declarados desta lista

- **O especialista desta mesa é o autor do programa**, como no primeiro domínio: não é de fora do programa nem de
  fora da área, o painel tem 1 de 3 e nenhum domínio fecha consenso assim.
- **A janela de corte de gabarito custa representatividade**: ficaram fora os itens cuja fonte eu não alcancei
  (ABS, parte do BJS), de modo que os 20 são honestos quanto ao gabarito e não são uma amostra aleatória do pool.
- **O pool tem 38 itens e 15 descartados**, todos com motivo no próprio arquivo.
