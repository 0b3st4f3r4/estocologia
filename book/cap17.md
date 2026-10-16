## Capítulo 17. Horizonte H1, parte 1: a ilusão de multiplicar por 3 na bolsa e o programa de emprego

**Objetivos.** Ao fim deste capítulo você deve saber medir o arrasto da volatilidade, distinguir a média do
conjunto da trajetória típica, calcular a alavancagem em que um único dia destrói o capital e reconhecer um viés de
seleção, inclusive no caso incômodo em que o ajuste acerta o número e continua sem valer como evidência.

---

### Degrau 1 — O mundo real e o paradoxo

**A promessa da alavancagem.** Um investidor abre 115 anos de cotações do Dow Jones (de 1885 ao fim de 1999,
31 919 pregões) e calcula a média dos retornos diários, 0,0668 ao ano em soma aritmética. A conclusão parece
inevitável: *"o índice rende 0,07 por ano; alavancado 3 vezes, rende 0,20 por ano, e quem segura por décadas
sempre ganha."*

Só que 0,20 é o número do dia médio visto de fora. Ao longo do tempo, uma **trajetória** vive outra
coisa. Medidos nesses 31 919 pregões, com rebalanceamento diário:

| Alavancagem | A soma aritmética promete | O crescimento realizado no tempo | O que a volatilidade cobra |
| --- | --- | --- | --- |
| 1× | 0,0668 ao ano | 0,0515 ao ano | 0,0153 ao ano |
| 2× | 0,1336 ao ano | 0,0719 ao ano | 0,0617 ao ano |
| 3× | 0,2004 ao ano | 0,0601 ao ano | 0,1403 ao ano |

A alavancagem de 3 vezes promete 20% ao ano e entrega 6%. A diferença, 14 pontos por ano, é o **arrasto da
volatilidade**, um pedágio que se paga todos os dias, sem exceção, e cresce mais rápido que a alavancagem
(mais adiante, com o quadrado dela). Alavancar 3 vezes rende **menos** que alavancar 2. Existe um
ponto ótimo, e passar dele destrói crescimento.

**A sexta-feira que apagou 68%.** Em 19 de outubro de 1987 o índice caiu 22,61% num único dia. Em cada 100 reais
de capital:

- sem alavancagem, restam 77 reais;
- com 2 vezes, restam 55 reais;
- com 3 vezes, restam **32 reais**, 68 reais destruídos em uma sessão;
- com 4,42 vezes, restam zero. Acima dessa alavancagem basta um único dia, e o capital vai a zero sem volta,
  porque não se ganha nada de zero.

**Quem segura por 20 anos.** O teste mais generoso para a alegação é o horizonte longo. Em cada 100 coortes de 20
anos (o investidor compra no primeiro pregão de um mês e vende 20 anos depois):

| Alavancagem | Terminam acima do início | Riqueza mediana | Riqueza somada ÷ número de coortes |
| --- | --- | --- | --- |
| 1× | 96 | 2,13× | 2,9× |
| 2× | 78 | 2,67× | 7,8× |
| 3× | 64 | 2,10× | **19,1×** |

A média de 19 vezes convive com uma mediana de 2,1. O número médio é puxado por um punhado de inícios felizes, e
a coorte típica que o investidor tem na mão mal dobra. Cerca de 35,7 em cada 100 coortes alavancadas
acabam abaixo do ponto de partida, o complemento exato dos 64,3% que terminam acima.

**Quantas observações independentes são?** As coortes se sobrepõem, já que um ano entra em dezenas de
janelas de 20 anos, e por isso as 1 134 coortes do treino valem só cerca de **5,8 blocos
independentes** (`h1-real/README.md`). Contar coortes não é contar observações. Uma margem calculada
sobre 1 134 seria cerca de 14 vezes menor que a honesta ($\sqrt{1\,134/5{,}8}$). É a lição de unidade
independente do Capítulo 6, agora no tempo.

**A Grande Depressão, em frequências naturais.** Do pico de 3 de setembro de 1929 ao fundo de 8 de julho de 1932,
cada 100 reais investidos viram **11 reais** sem alavancagem, e **2,5 centavos** com 3 vezes. Nenhum desses números
sai de simulação. A série é real, e o arrasto médio não registra um dia assim.

**O programa de emprego que parecia um desastre.** Em 1976 e 1977, um programa americano de treinamento
profissional (NSW) inscreveu trabalhadores em situação difícil. Para saber se funcionou, o caminho óbvio é
comparar os participantes com trabalhadores de uma pesquisa nacional (CPS), em 1978:

| Grupo | Em cada 100, empregados em 1978 | Renda média em 1978 |
| --- | --- | --- |
| Participantes do programa (185 pessoas) | 76 | 6 349 dólares |
| Sorteio: 260 controles, também candidatos, fora do programa | 65 | 4 555 dólares |
| Pesquisa nacional (15 992 pessoas) | 86 | 14 847 dólares |

Lida crua, a tabela condena o programa. Os participantes estão 10,7 pontos percentuais **menos** empregados que a
população e ganham 8 498 dólares menos. Lida pelo sorteio, a tabela absolve o programa. Entre candidatos
parecidos, quem entrou ficou 11,1 pontos mais empregado e ganhou 1 794 dólares a mais. Entre as duas
leituras há uma distância de 10 292 dólares por pessoa, e essa distância não é efeito do treino. É a diferença
entre quem se candidata a um programa de treinamento e o trabalhador médio do país.

**O placebo que revela a diferença.** Antes de existir o programa, em 1975, em cada 100 participantes 40 tinham
emprego, contra 89 em cada 100 da pesquisa nacional. Toda a diferença que o ajuste vai tentar consertar já estava
lá, três anos antes de o programa começar.

---

### Degrau 2 — A regra do jogo

**Alavancagem rebalanceada.** Manter alavancagem constante $L$ significa multiplicar a riqueza por $1 + L r_t$
a cada pregão, em que $r_t$ é o retorno simples do índice. Duas quantidades ficam separadas:

- a **média aritmética** do retorno, $\mu = \mathbb{E}[r]$, que a soma dos retornos promete;
- a **taxa de crescimento temporal**, $g_L = \mathbb{E}\big[\ln(1 + L r)\big]$, que uma trajetória
  experimenta no longo prazo.

Elas não coincidem em nenhum mundo com volatilidade:

$$g_L \;\approx\; L\mu \;-\; \frac{L^2\sigma^2}{2}, \qquad
\text{arrasto}_L \;=\; L\mu - g_L \;\approx\; \frac{L^2\sigma^2}{2}.$$

O arrasto cresce com o **quadrado** da alavancagem. Medido no treino (desvio diário $\sigma = 0,010496$, ou
0,1749 ao ano), os arrastos observados são 0,0153, 0,0617 e 0,1403, contra 0,0153, 0,0612 e 0,1377 previstos pela
fórmula. A conta de segunda ordem já pega o essencial, e sobra pouco para os momentos mais altos da distribuição.
A aproximação vale para retornos pequenos e nenhum salto grande; num dia de −22,6% ela deixa de
descrever o acontecido e passa só a subestimar o estrago.

**Não ergodicidade, em uma frase.** A média do conjunto de todos os investidores não é a média do tempo de
nenhum deles. A média de 19 vezes no 3× é uma média de conjunto, feita de um punhado de inícios muito
felizes e de muitas coortes que mal dobraram, e ninguém vive a média. Quando existe um **estado absorvente** (a
ruína, de onde não se sai), a média do conjunto deixa de ser uma descrição útil da vida real.

**Ruína em um dia.** Num mundo de retornos diários, a alavancagem que zera a riqueza num único pregão é

$$L_{\text{ruína}} = \frac{1}{\min_t |r_t|},$$

e no treino ela vale 4,42. Acima disso, a ruína deixa de ser questão de probabilidade e passa a ser uma data marcada no calendário.

**Kelly.** A alavancagem que maximiza a taxa temporal é $L^* = \arg\max_L g_L$. Na posterior do projeto, o
intervalo de 90% vai de **1,26 a 3,44**, com mediana **2,35**; no dia a dia, o Kelly empírico da série é 2,14.
Alavancar 3 vezes está dentro do intervalo, mas no seu extremo superior; o projeto A decidiu por bem menos que
isso, uma alavancagem de 0,26 no exercício simulado dessa lente.

**O teste severo.** O mundo da alegação afirma que a riqueza cresce 3 vezes a soma aritmética anual, sem arrasto. Nele,
a fração das coortes de 20 anos que terminam acima do ponto de partida tem média **0,933** (com o quantil 1% em
0,656). No índice real, a fração é **0,646**, e o procedimento teria acusado esse valor? Só em 0,95% das réplicas
(p = 0,0095). O resultado é severo.

**O que a refutação não alcança.** Delimitar o alvo que caiu é parte do resultado. A alegação refutada é "rende 3
vezes a média" e "sempre ganha". "Alavancar nunca compensa" sobrevive à refutação. Sem custo de financiamento, 2× e 3× ainda
crescem mais que 1× no tempo (0,0719 e 0,0601 contra 0,0515), e o cofre de 2000 a 2023 até favoreceu a alegação.
O previsor sem arrasto teve habilidade 0,44, contra 0,40 do modelo. Duas ausências da base favorecem a alegação e
estão declaradas. A série é só de preço (sem dividendos, e isso subestima o retorno total em cerca de 3% a 4% ao
ano no século XX), e não há custo de financiamento da alavancagem.

**Quem paga o erro.** Os 65,5% da riqueza somada das coortes vêm dos 10% melhores inícios; as piores datas de
entrada são todas de 1929 (3 de setembro, 1º de julho, 1º de agosto). Em cada 100 coortes de 20 anos, terminam
abaixo do início 4 sem alavancagem, 22 com 2× e 36 com 3×. Quem entrou no pior mês teve retorno médio igual ao do
mercado e não sobreviveu para vê-lo.

**A decomposição do LaLonde.** Comparar participantes com não participantes dá:

$$\underbrace{\bar Y_{\text{trat}} - \bar Y_{\text{não trat}}}_{\text{diferença crua}}
= \underbrace{Y_1 - Y_0}_{\text{efeito causal}}
+ \underbrace{\mathbb{E}[Y_0 \mid T=1] - \mathbb{E}[Y_0 \mid T=0]}_{\text{viés de seleção}}.$$

O cofre, os 260 controles sorteados do NSW, fica fora da amostra **por construção**, e esse afastamento não é
temporal. Essas 260 pessoas estiveram fora do ajuste desde o primeiro dia. O sorteio zera o segundo termo por
construção. Os controles foram sorteados entre os candidatos, então $\mathbb{E}[Y_0 \mid T=1] = \mathbb{E}[Y_0 \mid T=0]$. O ajuste por
escore de propensão não zera nada. Ele
**estima** esse termo a partir do que é observável, e o placebo de 1975 mostra que a estimativa erra. Com o
emprego de 1975 como desfecho (um desfecho que o programa não podia ter causado), o ajuste dá −12,3 pontos
(p = 0,04), enquanto o experimento dá +8,5, com intervalo de −0,6 a +17,5, compatível com zero.

**Acertar o número não é ter evidência.** Os três números trazem cada um o seu intervalo de 95%. A comparação crua dá
−10,7 pontos [−17,0; −4,5]; o ajuste por propensão dá +9,6 [−0,6; +20,8]; o experimento sorteado dá +11,1
[+2,6; +19,6]. O ajuste chega perto do experimento e continua sem valer como evidência. Mas um tratamento **inerte** teria dado +9,0, dentro da faixa do nulo, de −10,3 a +11,9 (p = 0,08). O
procedimento não podia ter dado errado de forma grosseira, e por isso o acerto não conta como evidência (Mayo). O
ajuste se apoia em cerca de 103 controles efetivos, e 72% do CPS está abaixo do menor escore dos tratados. Na
prática, ele compara os tratados com um punhado de pessoas do CPS que se parecem com eles; essas pessoas são
diferentes em coisas que o modelo não vê.

**O que o cofre mostrou.** Com os 260 controles sorteados, a habilidade do modelo é +0,18 (previu 0,655 de
emprego, o observado foi 0,646); a regressão logística fica em +0,09 e o contrafactual cru do CPS, em −0,002. O
k-anonimato das linhas vai de 1 a 11 depois da generalização de quase identificadores.

**Justiça, medida.** As taxas de falso negativo diferem por grupo. São 0,72 para hispânicos, 0,50 para negros e 0,57
para os demais. Igualá-las capta mais 22 desempregados e custa 87 falsos positivos. É o trilema de Chouldechova
medido em números. Não há como acertar as três coisas de uma vez, e alguém vai pagar a escolha (Capítulo 11).

**Os dois projetos deste capítulo passaram.** O da bolsa e o do emprego atravessaram os cinco portões do H1 em
domínio real e foram aprovados pela banca adversarial, com os números acima registrados no momento em que foram
produzidos. O terceiro projeto do horizonte, os incêndios dinamarqueses, é o Capítulo 18, e ele foi reprovado.

---

### Degrau 3 — Mão na massa

O laboratório lê as três bases públicas e mede todas as quantidades citadas acima: o crescimento temporal e o arrasto,
as coortes de 20 anos de início mensal, o choque de 1987, a Depressão, o mundo da alegação por reamostragem de
anos e a decomposição crua do NSW contra o CPS. O código está em `crates/stochology-book/src/cap17.rs`; a saída abaixo foi
medida com `cargo run --release` no crate de rascunho `~/tmp/labs-cap17`.

```text
== Capítulo 17 :: arrasto da volatilidade e o viés de seleção ==
treino 1885-1999: 31919 pregoes em 115 anos; serie inteira 1885-2023: 37930 pregoes
media aritmetica diaria = 0.000240669 (0.0668 ao ano), desvio diario = 0.010496 (0.1749 ao ano)

crescimento temporal por ano e arrasto (coortes e trajetorias do treino)
  L=1: a soma aritmetica daria 0.0668 ao ano; o crescimento realizado e 0.0515; arrasto 0.0153
  L=2: a soma aritmetica daria 0.1336 ao ano; o crescimento realizado e 0.0719; arrasto 0.0617
  L=3: a soma aritmetica daria 0.2004 ao ano; o crescimento realizado e 0.0601; arrasto 0.1403
pior pregao: -0.2261 (-22.61%) em 1987-10-19; perda com L=1 0.226, L=2 0.452, L=3 0.678; ruina diaria em L >= 4.42
  ano civil 1929-1932 com L=1: o capital termina em 0.19521 do inicial (80.5% destruido)
  ano civil 1929-1932 com L=3: o capital termina em 0.00088 do inicial (99.9% destruido)
  pico a fundo, 1929-09-03 a 1932-07-08 com L=1: o capital termina em 0.10814 do inicial (89.2% destruido)
  pico a fundo, 1929-09-03 a 1932-07-08 com L=3: o capital termina em 0.00025 do inicial (100.0% destruido)

coortes de 20 anos com inicio no primeiro pregao de cada mes (treino):
L           coortes    mediana (x)      media (x)  acima do inicio
1              1134           2.13            2.9           0.9577
2              1134           2.67            7.8           0.7848
3              1134           2.10           19.1           0.6429

coortes por ano-unidade (115 anos, soma movel de 20): acima do inicio em 1x = 0.9583, em 3x = 0.6458 (96 coortes)
mundo da alegacao (anos do treino reamostrados, crescimento 3*soma aritmetica anual, sem arrasto):
  2000 replicas: fracao media das coortes acima do inicio = 0.9286, quantil 1% = 0.6354, p(fracao <= 0.6458) = 0.0140

LaLonde: NSW com 185 tratados e 260 controles sorteados; CPS com 15992 observacoes
comparacao (desfecho de 1978)               emprego    renda (US$)
experimento sorteado (NSW trat. - NSW ctrl)      0.111           1794
comparacao crua (NSW trat. - CPS)            -0.107          -8498
vies de selecao implicado (cru - experimento)     -0.218         -10292
niveis de emprego em 1978: NSW tratados 0.757, NSW controles 0.646, CPS 0.864; em 100 pessoas: 76, 65 e 86
renda media de 1978 (US$): NSW tratados 6349, NSW controles 4555, CPS 14847
placebo de 1975 (emprego): NSW tratados 0.400, NSW controles 0.315, CPS 0.891; diferenca crua tratados - CPS = -0.491
```

Leia linha por linha. **0,2004, 0,0601, 0,1403**: a soma aritmética do 3× paga 20% ao ano e a volatilidade fica com
14, mais do que o ganho. **0,678** e **4,42**: um dia basta para destruir 68% do capital
alavancado 3 vezes, e acima de 4,42 vezes basta para zerá-lo. **2,10 contra 19,1**: no 3×, a coorte típica mal
dobra enquanto a média do conjunto chega a 19 vezes. **0,6429 e 0,9286**: no índice real, 64 em cada 100 coortes
de 20 anos terminam acima do ponto de partida; no mundo em que a alegação é verdadeira, 93 em cada 100, e a
diferença é o arrasto. **−0,107 e +0,111**: a pergunta sobre o programa dá dois sinais opostos, e a
diferença de −10 292 dólares é o viés de seleção, medido e não suposto.

**Declaração de divergência.** O `h1-real/README.md` registra 0,051, 0,072 e 0,060 de crescimento ao ano;
este instrumento mede **0,0515, 0,0719 e 0,0601** — a coincidência é de três casas, porque a conta é a mesma. As
coortes de início mensal publicadas são 1 135 e as daqui são 1 134 (a diferença é o critério de borda: uma coorte
cujo fim cai no último pregão do treino). Os valores publicados de fração acima do início são 0,9586 / 0,7841 /
0,6405; os daqui são **0,9577 / 0,7848 / 0,6429**, e as riquezas medianas publicadas (2,13 / 2,68 / 2,08) batem
com as daqui (2,13 / 2,67 / 2,10). No mundo da alegação, o material publicado traz média 0,9332, quantil 1% de
0,6563 e p = 0,0095, contra **0,9286**, **0,6354** e **0,0140** aqui: a diferença está no sorteio das
reamostragens (2 000 réplicas em cada caso, geradores diferentes), e o p-valor muda na terceira casa. Um caso
diferente merece registro: a edição anterior cita "de 1929 a 1932, o capital cai a 0,11 com 1× e a 0,00025 com
3×". Com a janela do **ano civil** (1º de janeiro de 1929 ao fim de 1932), este instrumento mede **0,19521** e
**0,00088**; com a janela do **pico ao fundo** (3 de setembro de 1929 a 8 de julho de 1932), mede **0,10814** e
**0,00025** — exatamente os números do material. A divergência não era de método: era de janela, e as duas
aparecem agora, com a data de cada uma.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 17 | sem perda medida neste capitulo | a unidade independente é a pessoa; o eixo R aparece como o grão da comparação
```

**Exercícios.**

1. *(Conta)* Use o arrasto de segunda ordem, $\tfrac{1}{2}L^2\sigma^2$, com $\sigma = 0{,}1749$ ao ano para
   prever o arrasto de 1×, 2× e 3× e compare com 0,0153, 0,0617 e 0,1403 medidos. Em que alavancagem o arrasto
   começa a superar o ganho aritmético?
2. *(Conta)* Qual alavancagem zera o capital no dia de −22,61%? E no pior dia de 1929? Explique por que "ruína"
   é um estado absorvente e o efeito disso sobre a média do conjunto.
3. *(Conceito)* Explique a frase "quem segura por décadas sempre ganha" e distinga o que a refutação
   derruba daquilo que ela deixa de pé.
4. *(Laboratório)* Rode `crates/stochology-book/src/cap17.rs` e refaça a tabela de coortes com horizonte de 10 anos
   (troque a constante `HORIZONTE`). Como fica a fração de coortes 3× acima do início? E a
   mediana?
5. *(Conceito)* Por que o placebo com o emprego de 1975 mede o viés de seleção? Escreva a resposta usando a
   decomposição da diferença crua.
6. *(Conta)* Com os níveis medidos de emprego em 1978 (76% dos tratados, 65% dos controles sorteados), calcule o
   efeito em pontos percentuais e explique por que a comparação crua dá o sinal contrário.

**Resumo.**

- A alavancagem multiplica a média aritmética, mas o tempo cobra o arrasto $\tfrac{1}{2}L^2\sigma^2$. O 3× promete
  0,20 ao ano e entrega 0,06.
- Um único dia pode destruir 68% do capital alavancado 3 vezes; acima de $1/\min|r|$ (4,42 aqui) a ruína é
  questão de calendário, não de probabilidade.
- Média de conjunto e trajetória típica se separam. Os 19,1× de média convivem com 2,10× de mediana, e cerca de 35,7 em cada 100
  coortes 3× terminam abaixo do início.
- Uma refutação precisa declarar o alvo: "rende 3 vezes" e "sempre ganha" caem; "alavancar nunca compensa" não.
- Acertar o número não basta. No LaLonde, o ajuste por propensão dá +9,6 pontos, e um tratamento inerte teria dado
  +9,0; o placebo de 1975 mostra que o viés de seleção continua lá, valendo 10 292 dólares por pessoa.
