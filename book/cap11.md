## Capítulo 11. O preço do erro, a lei de Goodhart e quem paga a conta

**Objetivos.** Ao fim deste capítulo você deve saber decidir pelo limiar custo/perda, calcular o valor
esperado da informação perfeita, usar os critérios robustos quando não há probabilidade em que
confiar, reconhecer um laço em que a previsão muda o que prevê e declarar quem paga pelo erro.

---

### Degrau 1 — O mundo real e o paradoxo

**O armador que se pergunta se vale segurar.** Um armador tem 100 000 na conta. O contrato de seguro
custa 1 800 por mês e cobre uma perda de 34 000 que acontece com chance de 5%. Vale a pena?

- **Em esperança**, o armador paga 1 800 e recebe, em média, $0{,}05 \times 34\,000 = 1\,700$. Perde
  100 por mês. A seguradora ganha 100 por mês. Quem olha só essa conta conclui que o seguro é caro.
- **No tempo**, cada mês multiplica a riqueza por um fator. Sem seguro, em 95% dos meses ela é
  multiplicada por 1,0 e em 5% por 0,66. Com seguro, é sempre multiplicada por 0,982 — o mesmo, todo
  mês. As duas contas dão números diferentes.

O critério que resolve o impasse é uma regra de uma linha: **pague para se proteger quando a chance
do evento é maior que o custo dividido pela perda**, isto é, $p > C/L$. Aqui $C/L = 1\,800/34\,000 =
5{,}294\%$, e a chance é de 5%: a regra do limiar manda **não** segurar, porque a chance está abaixo do
limiar por 0,294 ponto percentual. É um dos casos em que a resposta da esperança e a resposta do tempo
não coincidem, e o capítulo mostra as duas.

**Em cada 100 semanas.** O vendedor de seguro do capítulo anterior tem a situação invertida: ganha 1%
por semana e perde metade do capital com chance de 0,4%. Em 100 semanas, 67 terminam com lucro; o
perigo mora nas outras 33. Um histórico de 250 semanas não mostra nenhuma perda em
$0{,}996^{250} = 37\%$ das vezes, e quem estima a chance pela frequência observada conclui que ela é
zero e alavanca. É o **evento raro que não aparece na amostra**, e o Capítulo 10 já mediu o custo.

**A previsão que muda o que prevê.** Uma previsão publicada não é um termômetro: as pessoas reagem a
ela. Um aplicativo de trânsito que manda todos os usuários para a rota mais rápida de ontem transfere
o engarrafamento para a rota que recomendou. Com metade dos motoristas seguindo o aplicativo, quem
usa leva 41,0 minutos e quem não usa, 36,8. O previsor que divide os usuários para igualar os tempos
erra por 0,3 minuto, e todos levam 35,0. **A métrica virou meta e deixou de medir.** Essa é a lei de
Goodhart, e o inverso da versão do capítulo: aqui o previsor *desenhou* o laço e o estabilizou.

**Quem paga quando a taxa de erro é diferente para grupos diferentes.** Um sistema de triagem
decide quem entra num programa. Ele acerta a probabilidade dentro de cada grupo — é calibrado — e, ao
mesmo tempo, erra de modos diferentes para cada um: o grupo com mais casos reais é marcado com mais
frequência sem precisar, e o grupo com menos casos é perdido com mais frequência. A saída fácil —
"não use a cor no modelo" — não resolve, porque a cor entra pelos correlatos.

---

### Degrau 2 — A regra do jogo

**O limiar e o valor da informação.** Se proteger custa $C$ e não proteger custa $L$ quando o evento
ocorre, protege-se quando $p > C/L$. Uma informação vale pelo quanto pode mover a probabilidade para
o outro lado desse limiar, não pelos bits que carrega. O valor de saber o estado do mundo antes de
agir — o EVPI — é o teto do valor de qualquer informação:

$$\mathrm{EVPI} = \mathbb{E}_\theta\big[\max_a U(a,\theta)\big] - \max_a \mathbb{E}_\theta\big[U(a,\theta)\big]$$

O valor de qualquer exame real (EVSI) fica entre zero e ele. **Bits não são valor**: num exemplo do
curso, uma doença tem prevalência de 0,3; tratar custa 1 e deixar um doente sem tratamento custa 10,
de modo que se trata quando a chance passa de 0,10. Dois exames carregam a mesma informação mútua,
0,319 bit. O exame sensível vale 0,419, num teto (o EVPI) de 0,70. O específico vale zero, porque
mesmo o seu resultado negativo deixa a chance acima do limiar e não muda a ação.

**Nem toda informação é bem-vinda.** Good (1967) provou que informação gratuita nunca piora a decisão
de quem tem preferências sobre estados do mundo. Mas quem tem preferências sobre as próprias
observações pode pagar para não saber: entre pessoas com risco de doença de Huntington, só uma
minoria faz o exame genético antes dos sintomas — 17,4% no Reino Unido entre 1994 e 2014
(Baig et al., 2016).

**Critérios que dispensam probabilidades.** O limiar $C/L$ e o EVPI supõem uma distribuição sobre os
estados do mundo. No canto da incerteza da matriz de Stirling (Capítulo 4), ela não existe de modo
confiável, e a teoria da decisão oferece critérios robustos. O invariante $\Phi_4$ (Capítulo 14) exige
declarar qual deles foi usado:

| Critério | Regra | Postura |
| --- | --- | --- |
| Wald (maximin) | Escolher a ação cujo pior caso é o menos ruim | Pessimismo máximo |
| Savage (arrependimento mínimo) | Escolher a ação cujo maior arrependimento é o menor | Evitar o "se eu soubesse" |
| Laplace | Tratar os estados como igualmente prováveis | Ignorância como uniforme |
| Hurwicz | Ponderar o pior e o melhor caso por um índice de otimismo | Meio-termo declarado |

**Os diques da Holanda.** Depois da enchente de 1953, a Comissão Delta adotou para a Holanda central
um padrão de 1 em 10 000 anos, e van Dantzig (1956) tratou a altura do dique como um problema de
decisão, equilibrando o custo da obra e o dano esperado. O laboratório refaz o problema com um modelo
próprio: as máximas anuais seguem uma GEV de forma $\xi$ desconhecida, entre $-0{,}1$ e $0{,}2$, e o
custo anual de uma altura $h$ é a obra, $h - 2$, mais o dano esperado, $3000 \times P(\text{máxima} > h)$.
Cinco regras de decisão dão cinco alturas diferentes, e o critério e a lista de modelos têm de ser
declarados antes de ver a resposta. Chernoff (1954) mostrou que o critério de Savage pode mudar de
escolha só por acrescentar uma alternativa que ninguém escolheria; aqui o que se acrescenta é um
modelo. **Os critérios robustos não são mais "corretos" que o esperado: são a escolha honesta quando
a probabilidade de que a conta depende não merece confiança.**

**A previsão que muda o que prevê.** Quando um modelo é publicado, as pessoas reagem a ele. Lucas
(1976) mostrou o mesmo na economia: os parâmetros de um modelo estimado sob uma política dependem das
regras que os agentes esperam, e mudam quando a política muda. Num exemplo do curso, a curva de
Phillips antiga promete 4,59% de desemprego e o regime novo entrega 5,88%. A resposta de Kuhn e
Perdomo e colegas (2020) é conceitual: **performativamente estável** é o modelo que é ótimo para a
distribuição que ele mesmo induz.

**Aprender ou deformar?** Um agente que minimiza a energia livre variacional pode baixar a surpresa de
três modos: mudando a crença, mudando o mundo ou deixando de olhar.

$$F[q] = D\big(q \,\|\, p(s \mid o)\big) - \ln p(o) \;\ge\; -\ln p(o)$$

Um termostato que "acredita" em 22 °C numa sala a 18 °C termina com o mesmo $F$ quando aprende que a
sala está fria e quando a aquece até 22 °C. Só um contrafactual de fora separa os dois casos, e por
isso a arquitetura exige rodar o modelo com e sem a ação.

**O laço do H3, medido.** O H3 supôs que quem desiste da fila volta a chamar com probabilidade 0,6. O
mesmo parâmetro foi medido em outra central real, de 1999, com um desenho quase experimental: a fila
encontrada ao entrar, dentro da mesma data e hora, serve de instrumento para a desistência. Por esse
desenho, desistir em vez de ser atendido aumenta em **0,145** a chance de o mesmo cliente ligar de novo
em até quinze minutos, com intervalo de 90% de 0,132 a 0,157. O sinal confere e a ordem de grandeza
não: o laço declarado é cerca de quatro vezes o observado, fora da faixa de um terço a três que estava
fixada de antemão. O excesso de chegadas que o laço medido gera nessa central é de 2,3% das chegadas,
contra o viés de 29% que o laço declarado produziu no H3 — comparação de ordem de grandeza, porque o
viés do H3 depende também da taxa de desistência simulada, e não só do laço. E o valor medido é
provavelmente um limite inferior, porque uma chamada nova só é visível quando entra na fila.

No H3, o controle do laço foi feito com um pequeno experimento embutido na operação: em 10% dos
intervalos, um sorteio multiplicava a escala de atendentes por 0,7 ou 1,3. Como o sorteio independe da
demanda, a regressão das chegadas seguintes no déficit sorteado identifica o efeito da ação. O
previsor ingênuo, treinado nas chegadas observadas, **superestima a demanda exógena em 29%** e mesmo
assim acerta o que observa com 1% de erro: a métrica do próprio alvo aprova um previsor errado sobre o
mundo. Com o instrumento, $\hat r$ sai em 0,56 (intervalo de 90% de 0,51 a 0,61) e o erro da previsão
corrigida cai para **1,7%**, dentro do limite de 3%.

**Quem paga pelo erro.** Quando as taxas-base diferem entre grupos, o mesmo limiar produz taxas de
erro diferentes. A identidade de Chouldechova (2017) mostra por quê:

$$\text{FPR}=\frac{p}{1-p}\cdot\frac{1-\text{VPP}}{\text{VPP}}\cdot(1-\text{FNR})$$

Com valor preditivo (VPP) e falsos negativos iguais entre grupos, prevalências $p$ diferentes forçam
taxas de falso positivo diferentes. **Igualar aprovações, igualar erros e igualar calibração por grupo
são exigências incompatíveis quando as prevalências diferem e o preditor não é perfeito.** "Quero
justiça" ainda não é uma decisão: é preciso declarar que igualdade se escolheu, quanto ela custa, para
onde o erro migrou e quem passa a pagá-lo.

**Comunicar é parte da decisão.** Um resultado mal comunicado falhou na camada do Agente. "30% de
chance" não diz 30% de quê; "de cada 100 pessoas como você, 30" diz. As regras: denominadores fixos,
risco absoluto antes do relativo e classe de referência explícita. Gráficos de resultados hipotéticos
(HOPs), que mostram desfechos simulados um após o outro, fazem o leitor sentir a variação que uma
barra de erro esconde. A recomendação declara o ponto em que a ação muda — agir se $p > C/L$ — e uma
cláusula objetiva de quando deixa de valer.

---

### Degrau 3 — Mão na massa

Quatro tarefas: a calculadora de custo/perda no navio de Bernoulli com o crescimento no tempo, o EVPI
e o EVSI em um caso de dois atos, os quatro modelos de cauda do dique com cinco regras de decisão, e a
tabela do trilema com a identidade de Chouldechova. Código verificado em `crates/stochology-book/src/cap11.rs`; saída
medida com o crate de rascunho `labs_cap11`:

```text
== Capítulo 11 :: o navio de Bernoulli (C = 1.800, L = 34.000, p = 0,05) ==
limiar p > C/L: p* = 0.05294 (5.294%)  ->  p = 0,050 nao passa o limiar
em esperança: perda esperada sem seguro = 1700, resultado do seguro = +100 por mês
crescimento no tempo do armador: 1 navio sem seguro = -0.0208, 1 navio com seguro = -0.0182, 3 navios sem seguro = -inf
seguradora com     1 contratos: crescimento no tempo = -0.000186 por mês
seguradora com    10 contratos: crescimento no tempo = -0.001808 por mês
seguradora com   100 contratos: crescimento no tempo = -0.013466 por mês
seguradora com  1000 contratos: crescimento no tempo = +0.100410 por mês

== Capítulo 11 :: o EVPI (tratar custa 1, deixar um doente custa 10) ==
limiar de ação: p > 0.10  ->  com p = 0,3 o melhor ato sem informação tratar (-1.00)
ganho de quem sabe o estado antes de agir = -0.30
EVPI = +0.70
  exame sensivel  (0,90; 0,70): EVSI = +0.2400
  exame especifico (0,30; 0,95): EVSI = +0.2450
  com prevalência 0.05: melhor ato sem informação = nao tratar, EVPI = +0.450
  com prevalência 0.30: melhor ato sem informação = tratar, EVPI = +0.700
  com prevalência 0.60: melhor ato sem informação = tratar, EVPI = +0.400

== Capítulo 11 :: os diques de van Dantzig, quatro modelos de cauda ==
  modelo    xi = -0.1  xi = +0.0  xi = +0.1  xi = +0.2   altura ótima
  T = 100          4.07       1.26       5.71       7.01
  T = 10000        5.84       1.26      12.78      21.49
  altura ótima com xi = -0.1: 5.839 m  (custo 4.143)
  altura ótima com xi = +0.0: 3.059 m  (custo 1.130)
  altura ótima com xi = +0.1: 9.809 m  (custo 9.426)
  altura ótima com xi = +0.2: 12.584 m  (custo 13.613)
  valor esperado com o Gumbel: 3.06 m
  media dos modelos: 7.82 m
  maximin de Wald: 12.58 m
  arrependimento minimax de Savage: 9.81 m (maior arrependimento 6.68)
  precaucao: 1 em 10 000 no pior modelo = 21.49 m
  EVPI do conjunto de modelos = +18.427 m

== Capítulo 11 :: o trilema de Chouldechova ==
grupo     taxa-base        FNR        FPR falsos positivos falsos negativos        VPP
White         0.069      0.240      0.105              467               79     0.9408
Black         0.111      0.360      0.196              832              191     0.9096
casos observados: White 329 marcados pelos fatos, Black 530

-- FNR igualado para baixo, ao FNR do White em 0.240 --
White         0.069      0.240      0.105              467               79
Black         0.111      0.240      0.045              192              127
falsos positivos totais: 1299 -> 659 (-640)
falsos negativos totais: 270 -> 206 (-64)

-- FNR igualado para cima, ao FNR do Black em 0.360 --
White         0.069      0.360      0.246             1095              119
Black         0.111      0.360      0.196              832              191
falsos positivos totais: 1299 -> 1927 (+628)
falsos negativos totais: 270 -> 309 (+40)

identidade de Chouldechova: FPR = p/(1-p) * (1-VPP)/VPP * (1-FNR)
grupo       VPP da matriz  VPP da identidade          razao
White              0.3491             0.0592            0.2x
Black              0.2896             0.0904            0.3x
leitura: com o FNR e o FPR observados, o escore so seria calibrado por grupo
se o valor preditivo positivo fosse o da coluna do meio. Ele e muito maior: o escore
separa razoavelmente e nao se le como probabilidade de risco.
```

**O que cada linha diz.**

**(1) O navio de Bernoulli.** O limiar é $C/L = 0{,}05294$, ou 5,294%: a chance de 5% fica **abaixo**
dele, e a regra do custo/perda manda não segurar. Em esperança a perda sem seguro é 1 700 e o
resultado do seguro é $+100$ por mês para a seguradora. O crescimento no tempo inverte a leitura para o
armador: com um navio, $-0{,}0208$ por mês sem seguro e $-0{,}0182$ com seguro — os dois negativos, e
o seguro **melhora** o resultado apesar de a esperança dizer o contrário. Com **três** navios, o
crescimento sem seguro é $\mathbf{-\infty}$: com três perdas simultâneas a riqueza não cobre o
prejuízo, e a ruína não é uma probabilidade pequena, é um estado absorvente.

**(2) Do outro lado do balcão.** A seguradora melhora quando acumula contratos independentes: com 1
contrato o crescimento no tempo é $-0{,}000186$ por mês; com 10, $-0{,}001808$; com 100, $-0{,}013466$;
com **1 000 contratos, $+0{,}100410$**. O prêmio é o mesmo em todos os casos — o que muda é o número de
trajetórias independentes que ela detém. As primeiras linhas descem porque a fórmula usada é a
aproximação média menos variância dividida por dois, e com poucos contratos a variância é grande demais
para a aproximação valer; a partir de algumas centenas de contratos a lei funciona e o crescimento vira
positivo. **Quem reparte o risco cobra o mesmo e cresce; quem carrega a trajetória sozinho afunda.** É a
mesma resposta do Capítulo 10, agora com uma companhia de seguros no lugar de um apostador.
**(3) O EVPI.** Com prevalência de 0,3, tratar custa 1 e deixar um doente sem tratamento custa 10, o
limiar de ação é $p > 0{,}10$ e a melhor ação sem informação é tratar, com ganho $-1{,}00$. Quem sabe
o estado antes de agir tem $-0{,}30$, e a diferença é o **EVPI $= +0{,}70$**. Os dois exames do texto
medem quase o mesmo: EVSI de $0{,}2400$ para o sensível $(0{,}90; 0{,}70)$ e $0{,}2450$ para o
específico $(0{,}30; 0{,}95)$ — ambos abaixo do teto. Com prevalência 0,05 o EVPI cai para 0,450; com
0,30, sobe a 0,700; com 0,60, cai de novo para 0,400. **O valor da informação é máximo quando a
probabilidade está perto do limiar de ação**, e é aí que a informação muda a decisão.

**(4) Os diques.** Os quatro modelos discordam pouco no nível de **100 anos** (de 1,26 a 7,01 m) e
muito no de **10 000 anos** (de 1,26 a 21,49 m): a cauda é onde os modelos se separam. As cinco regras
dão cinco alturas: valor esperado com o Gumbel, **3,06 m**; média dos modelos, **7,82 m**; maximin de
Wald, **12,58 m**; arrependimento minimax de Savage, **9,81 m**; precaução de 1 em 10 000 no pior
modelo, **21,49 m**. O valor esperado de um só modelo escolhe a altura mais barata e a mais frágil; a
precaução escolhe a mais cara; Savage limita o quanto se pode lamentar. O EVPI do conjunto de modelos
— a distância entre a precaução e o valor esperado — vale **18,427 m**, e é a medida de quanto custa
não saber qual é a cauda certa.

**(5) A tabela do trilema.** Os dois grupos: White com taxa-base 0,069, FNR 0,24 e FPR 0,105; Black
com taxa-base 0,111, FNR 0,36 e FPR 0,196. No limiar do sistema, são 467 e 832 falsos positivos e 79 e
191 falsos negativos. Igualando o FNR **para baixo**, ao nível do White, os falsos positivos caem de
1 299 para 659 e os falsos negativos de 270 para 206. Igualando **para cima**, ao nível do Black, os
falsos positivos sobem de 1 299 para 1 927 ($+628$) e os falsos negativos de 270 para 309. **A
escolha de qual igualdade perseguir não é neutra: cada direção move os dois erros em direções opostas
e em magnitudes diferentes.**

**(6) A identidade fecha e acusa.** O valor preditivo positivo que a matriz de confusão entrega é
0,3491 (White) e 0,2896 (Black). O que a identidade de Chouldechova exige, dadas a taxa-base, o FNR e
o FPR observados, é 0,0592 e 0,0904 — de quatro a seis vezes menor. **O escore separa razoavelmente e
não se lê como probabilidade de risco.** É a tradução numérica do que a identidade diz: com as três
taxas observadas, o preditor não pode ser calibrado por grupo e ter os mesmos erros ao mesmo tempo.

**Declaração de divergência.** A edição anterior registra 35,8% de ruína para quem aposta pelo próprio
histórico e 0% com Laplace (Capítulo 10), e uma tabela de diques com alturas de 4,76 m (valor
esperado com Gumbel), 6,20 m (média dos modelos), 7,46 m (Wald), 5,93 m (Savage) e 9,96 m (precaução),
com níveis de 100 anos de 3,11 a 4,26 m e de 10 000 anos de 3,81 a 9,96 m. Este instrumento, com
$\mu = 1{,}2567$ e $\sigma = 0{,}762$ calibrados para o nível de 100 anos do modelo de $\xi = -0{,}1$
chegar a 4,26 m, mede 3,06 m, 7,82 m, 12,58 m, 9,81 m e 21,49 m. As alturas antigas vêm de um
conjunto de modelos calibrado de outro modo, que este laboratório não reproduz; os dois conjuntos
ficam registrados. No trilema, o valor antigo de 1 273 falsos positivos para igualar o FNR é da tabela
de controle do H1; este laboratório mede $+628$ (com $+274$ na direção inversa), porque o cálculo aqui
parte das taxas por grupo em vez das unidades. As divergências estão declaradas e nenhum valor antigo
foi apagado.

**Exercícios.**

1. *(Conta)* No navio de Bernoulli, o seguro custa 1 800 e cobre uma perda de 34 000 com chance de
   5%. Pela regra do custo/perda, o armador deveria se segurar? O que diz o crescimento temporal?
   Explique as duas respostas numa frase cada.
2. *(Conta)* Refaça o EVPI com prevalência de 0,10 (exatamente o limiar). O valor da informação
   perfeita aumenta ou diminui? O que isso diz sobre onde gastar o orçamento de exames?
3. *(Conceito)* No exemplo do aplicativo de duas rotas, quem erra mais: o previsor ingênuo ou o
   previsor que iguala os tempos? Em que sentido o segundo é "performativamente estável"?
4. *(Conta)* Com as taxas do laboratório (White: 0,069, 0,24, 0,105), verifique a identidade de
   Chouldechova e diga em quanto ela erra. O erro é de arredondamento ou de modelagem?
5. *(Laboratório)* No laboratório, troque a taxa-base do segundo grupo de 0,111 para 0,20 e refaça a
   tabela do trilema. Que igualdade fica mais cara e por quê?
6. *(Conceito)* A Regra 14.2 do livro proíbe 0% e 100% de probabilidade. Que problema do laboratório
   do navio essa proibição evita?

**Resumo.**

- Decide-se pelo limiar $C/L$: com $C = 1\,800$ e $L = 34\,000$, o limiar é 5,294% e a chance de 5%
  fica abaixo dele. O crescimento no tempo pode dizer o contrário, e os dois números precisam ser
  declarados.
- A informação vale o quanto move a decisão: o EVPI é o teto, e bits não são valor.
- Quando a probabilidade não merece confiança, os critérios de Wald, Savage e a precaução substituem o
  valor esperado; o critério e a lista de modelos são escolhas, não deduções.
- Previsões publicadas mudam o mundo: o Goodhart medido no H3 derruba o viés de 29% para 1,7%, e o
  laço real medido é quatro vezes menor que o declarado.
- Justiça exige escolher entre igualdades incompatíveis: igualar o FNR para cima custa $+628$ falsos
  positivos, para baixo economiza 640, e quem paga é declarado antes de rodar.

### Quem paga, na pesquisa

A pesquisa fechou os sete pareceres de governança e os três fluxos de prisões. Em 15 de outubro de
2026, `governanca/GRUPOS.md` registra: **nenhum dos sete grupos afetados foi ouvido, em nenhuma das
sete decisões.** O arquivo é a resposta da frente à objeção "o auditor é parte interessada".
Assinatura do responsável não é escuta do grupo afetado: o responsável está do lado de quem decide
usar o artefato, não de quem paga o erro. A lista inclui os 5 226 detidos de Toronto em 1997–2002, os
65 276 de 2020–2021 e as 86 113 paradas de trânsito em Rhode Island 2005–2015, todas marcadas por cor
percebida. A auditoria não pôde olhar três coisas: quem autoriza, o painel humano que nunca foi feito
e as decisões que não chegaram a ninguém. O Capítulo 21 volta a esse ponto.
