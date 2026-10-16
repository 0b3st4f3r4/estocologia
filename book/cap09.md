# Parte III. Causa, tempo e sobrevivência

## Capítulo 9. A escada de causa e efeito: grafos, confundidores e invariância (ICP)

**Objetivos.** Ao fim deste capítulo você deve saber situar uma pergunta num dos três degraus de
Pearl, desenhar um grafo causal simples, distinguir um confundidor de um colisor, decompor uma
diferença ingênua em efeito mais viés de seleção e aplicar a predição causal invariante (ICP) para
escolher entre duas variáveis que a correlação trata como equivalentes.

---

### Degrau 1 — O mundo real e o paradoxo

**Duas cirurgias e uma tabela que se contradiz.** Em 1986, Charig e colegas compararam duas
cirurgias para cálculos renais. A cirurgia aberta venceu nos dois estratos em que os pacientes
costumam ser separados, e perdeu no total:

| Grupo | Cirurgia aberta | Outra técnica |
| --- | --- | --- |
| Cálculos pequenos | 93% de sucesso (81 de 87) | 87% (234 de 270) |
| Cálculos grandes | 73% (192 de 263) | 69% (55 de 80) |
| Total bruto | 78% (273 de 350) | 83% (289 de 350) |

A explicação está na coluna esquerda da tabela. Os casos difíceis iam para a cirurgia aberta.
**Em cada 100 pacientes operados abertamente, 75 tinham cálculos grandes; na outra técnica, 23.**
Comparar os totais é comparar dois grupos de pacientes diferentes. Se os dois procedimentos
tivessem atendido à mesma mistura de casos, **51% pequenos e 49% grandes, a mistura da população
inteira**, os números seriam 83,3% para a aberta e 77,9% para a outra. A vantagem volta para a
aberta, agora por uma diferença maior do que a de qualquer estrato isolado.

Nada nessa tabela está errado. Entre a segunda e a quarta linha muda **a pergunta**.
*Entre os operados, qual técnica foi melhor?* e *para o próximo paciente, qual técnica escolher?*
são duas perguntas, e a resposta da primeira não serve à segunda. Essa distinção tem nome e tem
uma escada.

**Os três degraus.** Em cada 100 manchetes com a palavra "causa", nove em cada dez estão no
primeiro degrau. Pearl separou os degraus pela pergunta que se faz:

| Degrau | Pergunta | Exemplo |
| --- | --- | --- |
| 1. Ver | Quem tem mais de quê? | Bairros com mais policiais têm mais crimes |
| 2. Agir | Se eu mexer, o que muda? | Se dobrarmos o policiamento, o crime cai? |
| 3. Imaginar | O que teria acontecido? | Esse assalto teria acontecido com uma viatura na esquina? |

O primeiro degrau é o da correlação; o segundo exige um **do(x)**, a intervenção; o terceiro exige
um mundo paralelo. Os dados sozinhos só sustentam o primeiro. A ponte para os outros dois é um
grafo causal, **uma declaração humana sobre o mundo, não uma saída de software**.

**O alarme que parece causa.** Numa unidade de terapia intensiva, o quadro da equipe acende um
alarme quando a resistência bacteriana estimada passa de um limite. O alarme prevê o desfecho
grave melhor do que qualquer outra variável do sistema. Em cada 100 pacientes que o alarme marca,
muitos evoluem mal, e a equipe começa a tratar o alarme como se ele fosse a doença.

Só que o alarme é **filho** da resistência, não pai. Quem o aciona é a resistência, e é ela que
causa o desfecho. Trocar o alarme de lugar, desligá-lo ou calibrá-lo de outro jeito não muda o
desfecho de um paciente sequer, porque mexer no filho não é mexer no pai. A correlação entre o
alarme e o desfecho é altíssima; o efeito causal do alarme é zero. Quando os dados vêm de três
hospitais diferentes, o que separa os dois é a **invariância**. O efeito da resistência se repete
em todo hospital, o do alarme não.

---

### Degrau 2 — A regra do jogo

**Grafos.** Um grafo causal tem nós (variáveis) e setas (relações diretas). $X \to Y$ diz que $X$
está na equação que gera $Y$. Três arranjos recebem nomes que vale a pena decorar:

| Arranjo | Forma | O que condicionar faz | Nome |
| --- | --- | --- | --- |
| Confundidor | $X \leftarrow Z \to Y$ | Fecha um caminho falso | Ajustar ajuda |
| Mediador | $X \to Z \to Y$ | Apaga o efeito que se quer medir | Ajustar apaga |
| Colisor | $X \to Z \leftarrow Y$ | Abre um caminho falso | Ajustar piora |

Berkson (1946) mostrou o terceiro caso com duas doenças independentes. Se cada uma aumenta a
chance de internação, olhar só os internados as faz parecer associadas. Numa simulação do curso,
talento e simpatia têm correlação de $+0{,}002$ na população e de $-0{,}669$ entre os que passam de
um limiar. **"Controlar por tudo o que foi medido" não é neutro**. Ajustar um mediador apaga o
efeito, ajustar um colisor o fabrica e ajustar um instrumento amplifica o viés que sobra.

**A porta dos fundos.** O critério de Pearl lista o que ajustar. Um conjunto $Z$ fecha a porta dos
fundos quando bloqueia todos os caminhos que entram em $X$ por trás, sem conter nenhum descendente
de $X$. Quando isso vale, a distribuição observada responde à pergunta de intervenção:

$$P(y \mid \operatorname{do}(x)) = \sum_z P(y \mid x, z)\,P(z)$$

O caso trabalhado canônico é a terapia hormonal. Em 1991, a coorte do Nurses' Health Study, com
cerca de 48 mil mulheres, relatou risco relativo de **0,56** de doença coronariana grave entre as
usuárias de estrogênio. Em 2002, o ensaio sorteado da Women's Health Initiative, com 16 608
mulheres, encontrou razão de riscos de **1,29**. As usuárias da coorte tinham outros hábitos,
outra renda e outro acesso a médicos, e havia uma porta dos fundos aberta, que o sorteio fechou.

**Resultados potenciais.** Cada pessoa tem dois desfechos possíveis, $Y(1)$ com o tratamento e
$Y(0)$ sem ele, e só um é observado. Comparar os tratados com os não tratados dá, exatamente:

$$\mathbb{E}[Y\mid D{=}1]-\mathbb{E}[Y\mid D{=}0] = \underbrace{\mathbb{E}[Y(1)-Y(0)\mid D{=}1]}_{\text{efeito nos tratados}} + \underbrace{\mathbb{E}[Y(0)\mid D{=}1]-\mathbb{E}[Y(0)\mid D{=}0]}_{\text{viés de seleção}}$$

Quando o tratamento é **sorteado**, o segundo termo é zero por construção, porque quem foi sorteado
para o tratamento tem o mesmo $Y(0)$ esperado que quem não foi. O Capítulo 17 encontra isso nos
dados de LaLonde, onde o viés de seleção troca o sinal do efeito: a comparação crua dá $-8\,498$
dólares de renda em 1978 e o experimento dá $+1\,794$; a diferença, $-10\,292$, é o termo do meio.

**A diferença ingênua não é só efeito.** A decomposição dá um nome ao terceiro elemento, e ele não
tem conserto estatístico. Nada nos dados observados diz quanto ele vale, porque um dos dois
desfechos potenciais de cada pessoa está faltando por construção. Só o sorteio zera esse termo.

**Invariância: o que sobrevive à troca de ambiente.** Uma relação causal sobrevive à troca de
ambiente; uma correlação de conveniência, não. A Predição Causal Invariante, de Peters, Bühlmann e
Meinshausen (2016), transforma isso num teste. Se $S$ é o conjunto das causas diretas de $Y$,
$P(Y \mid X_S, E = e)$ é a mesma distribuição em todo ambiente $e$ que intervenha nas outras
variáveis sem interferir na equação de $Y$.

O procedimento tem três passos. (1) Para cada subconjunto candidato $S$, testar se a relação de
$Y$ com $X_S$ é invariante entre os ambientes; (2) guardar todos os $S$ que passam; (3) devolver a
interseção deles. **A garantia é assimétrica**. $\hat S$ só contém causas diretas, com
probabilidade de pelo menos $1-\alpha$, mas nada garante que as contenha todas. Abster-se,
devolvendo o conjunto vazio, é resultado, não falha. A abstenção acontece de dois modos. O teste
rejeita todos os conjuntos, ou aceita tantos que a interseção fica vazia.

O teste trata cada linha como unidade independente. Quando o mesmo voluntário dá 16 medidas, ou a
mesma escola dá 403 alunos, a unidade honesta é o voluntário ou a escola, e o veredito muda de
área para área. No H2 isso apareceu inteiro:

| Leitura | O que aconteceu |
| --- | --- |
| 6 dos 12 domínios | o ICP rejeitou todos os conjuntos: os ambientes agem direto em $Y$ |
| 4 domínios | aceitou tanto que $\hat S$ ficou vazio: os ambientes mexem pouco em $X$ |
| 1 domínio | $\hat S$ não vazio, e contra a alegação: cidadania e emprego, não a cor da pele |

**Uma lição que o H2 acrescentou: o efeito transporta melhor que o conjunto.** Mesmo quando o ICP
rejeita todos os conjuntos, o modelo ajustado nos ambientes de treino previu bem um ambiente nunca
visto em 9 domínios, com habilidade de 0,04 a 0,88. Encontrar o conjunto exato de causas é mais
exigente do que prever sob mudança.

**Por que isso importa para decidir.** A invariância é também a resposta à crítica de Lucas
(Capítulo 11): só o que é invariante entre regimes merece uso quando o regime muda. No H2g, com a
verdade de referência declarada antes, o ICP recuperou os pais verdadeiros da corrente elétrica
numa câmara física, $\hat S = \{red, green, blue\}$, e deixou de fora sensores de luz fortemente
correlacionados, porque eram filhos dos LEDs.

---

### Degrau 3 — Mão na massa

Três tarefas: refazer a conta de Charig com as contagens, classificar perguntas nos três degraus e
construir um ICP mínimo que escolhe a causa e rejeita o alarme falso. Código verificado em
`crates/stochology-book/src/cap09.rs`; saída medida com o crate de rascunho `labs_cap09`:

```text
== Capítulo 9 :: o paradoxo de Simpson nas cirurgias de pedra nos rins ==
pequenas: aberta 81/87 = 0.9310  nova 234/270 = 0.8667
grandes:  aberta 192/263 = 0.7300  nova 55/80 = 0.6875
cru:      aberta 273/350 = 0.7800  nova 289/350 = 0.8257
peso dos estratos na populacao inteira: pequenas 0.5100, grandes 0.4900
ponderado pela populacao inteira: aberta 0.8325  nova 0.7789  -> a vantagem inverte de novo
fracao de casos dificeis na aberta = 0.7514, na nova = 0.2286

== Capítulo 9 :: os tres degraus de Pearl ==
  [   1. associação] bairros com mais policia tem mais crimes
  [  2. intervenção] se dobrarmos o policiamento, o crime cai?
  [3. contrafactual] esse assalto teria acontecido com uma viatura na esquina?

== Capítulo 9 :: um ICP minimo, tres sitios, tres candidatos ==
linhas = 12000 (4000 por sitio), X -> Y com coeficiente 2, X -> Z (filho) com ruido diferente por sitio, W puro ruido
correlacao ingenua com Y: corr(X,Y) = 0.9314  corr(Z,Y) = 0.7208  corr(W,Y) = 0.0055
inclinacao ingenua (todos os sitios juntos): X = 1.9885  Z = 0.6003  W = 0.0152
  candidato {    x}  F =   0.755  p = 4.702e-1  comum = 1.985  por sitio = [1.970, 1.996, 1.990]  ACEITO
  candidato {    z}  F = 976.809  p = 0.000e0  comum = 0.477  por sitio = [0.928, 0.496, 0.299]  rejeitado
  candidato {    w}  F =   1.474  p = 2.290e-1  comum = 0.011  por sitio = [0.056, -0.030, 0.008]  ACEITO
  candidato {  x+z}  F =   0.612  p = 6.541e-1  comum = 1.991  por sitio = [1.918, 1.998, 1.999]  ACEITO
  candidato {  x+w}  F =   0.704  p = 5.890e-1  comum = 1.985  por sitio = [1.970, 1.996, 1.990]  ACEITO
  candidato {  z+w}  F = 488.188  p = 0.000e0  comum = 0.477  por sitio = [0.928, 0.496, 0.299]  rejeitado
  candidato {x+z+w}  F =   0.624  p = 7.111e-1  comum = 1.991  por sitio = [1.919, 1.998, 1.999]  ACEITO
  candidato {vazio}  F = 0.000  p = 1.0  (so interceptos)
conjuntos aceitos: 5 -> intersecao S_hat = {}

== Capítulo 9 :: 200 repetições, mecanismo estavel: Y = 2X + ruido em todo sitio
inclinação ingênua média (todos os sítios juntos): X = 2.0004  Z = 0.6015
  {    x}  F médio =     1.04  taxa de rejeição = 0.060
  {    z}  F médio =   973.87  taxa de rejeição = 1.000
  {    w}  F médio =     1.14  taxa de rejeição = 0.085
  {  x+z}  F médio =     0.99  taxa de rejeição = 0.050
  {  x+w}  F médio =     1.04  taxa de rejeição = 0.075
  {  z+w}  F médio =   487.45  taxa de rejeição = 1.000
  {x+z+w}  F médio =     1.01  taxa de rejeição = 0.055
  {x} aceito em 0.940 das rodadas
  interseção = {x} em 0.080 das rodadas
  interseção = {z} em 0.000 das rodadas
  interseção = {w} em 0.025 das rodadas
  interseção contém x e não contém w em 0.080 das rodadas
  interseção vazia em 0.895 (dos quais 0.025 porque só o ruído W sobrou, 0.005 porque a própria causa não sobrou, e 0.890 porque W entrou em alguns conjuntos e não em outros)

== Capítulo 9 :: 200 repetições, mecanismo instavel: o coeficiente de X em Y muda de sitio para sitio
inclinação ingênua média (todos os sítios juntos): X = 4.0004  Z = 1.2029
  {    x}  F médio =     1.04  taxa de rejeição = 0.060
  {    z}  F médio =  1352.54  taxa de rejeição = 1.000
  {    w}  F médio =     1.10  taxa de rejeição = 0.070
  {  x+z}  F médio =     0.99  taxa de rejeição = 0.050
  {  x+w}  F médio =     1.04  taxa de rejeição = 0.075
  {  z+w}  F médio =   676.76  taxa de rejeição = 1.000
  {x+z+w}  F médio =     1.01  taxa de rejeição = 0.055
  {x} aceito em 0.940 das rodadas
  interseção = {x} em 0.065 das rodadas
  interseção = {z} em 0.000 das rodadas
  interseção = {w} em 0.025 das rodadas
  interseção contém x e não contém w em 0.065 das rodadas
  interseção vazia em 0.910 (dos quais 0.025 porque só o ruído W sobrou, 0.005 porque a própria causa não sobrou, e 0.905 porque W entrou em alguns conjuntos e não em outros)
```

**O que cada linha diz.**

**(1) A conta de Charig fecha.** A cirurgia aberta vence nos pequenos (0,9310 contra 0,8667) e nos
grandes (0,7300 contra 0,6875), e perde no bruto (0,7800 contra 0,8257). Ponderando os dois
estratos pela população inteira, com pesos 0,5100 e 0,4900, a aberta fica com 0,8325 e a outra com
0,7789. A vantagem inverte de novo, agora para o lado da aberta, com a diferença ampliada. Quem
produziu o paradoxo foi a mistura de casos: **75% dos pacientes da aberta tinham cálculos grandes,
contra 23% na outra técnica**. É a mesma lição dos 12 casos do Capítulo 2, agora com números de um
ensaio real.

**(2) A correlação ingênua não consegue separar a causa do sintoma.** Com 12 000 linhas em três
sítios, o desfecho $Y$ foi gerado por $Y = 2X + \text{ruído}$, o sintoma $Z$ por $Z = 2X +
\text{ruído próprio do sítio}$ e $W$ é ruído puro. Olhando tudo junto, $\operatorname{corr}(X,Y) =
0{,}9314$ e $\operatorname{corr}(Z,Y) = 0{,}7208$. A causa e o sintoma parecem duas variáveis
explicativas, e a regressão ingênua dá inclinação **1,9885** para $X$ e **0,6003** para $Z$. Se o
analista parasse aqui, concluiria que o alarme tem efeito próprio de 0,6.

**(3) O ICP separa os dois.** No teste de invariância, ajustando a mesma inclinação nos três sítios
contra inclinações livres:

| Candidato | F | p | Inclinação comum | Por sítio | Veredito |
| --- | --- | --- | --- | --- | --- |
| $\{X\}$ | 0,755 | 0,470 | 1,985 | 1,970 · 1,996 · 1,990 | aceito |
| $\{Z\}$ | 976,809 | ~0 | 0,477 | 0,928 · 0,496 · 0,299 | rejeitado |

A inclinação de $X$ é praticamente a mesma nos três sítios (1,970, 1,996, 1,990); a de $Z$ vai de
0,928 a 0,299, quase triplicando de um sítio para o outro. O mecanismo que liga a causa ao desfecho
não mudou; o que liga o sintoma ao desfecho mudou em cada lugar. A p de $Z$ é indistinguível de
zero (F perto de mil) e a de $X$ é 0,470. Em outras palavras, os dados não têm como distinguir a
relação de $X$ de uma relação que valha em todo lugar.

**(4) O que o teste acerta e onde ele se abstém.** Em 200 repetições com o mecanismo estável, o
conjunto $\{X\}$ é aceito em **94,0%** das rodadas e $\{Z\}$ é rejeitado em **100%** delas. A
interseção de todos os conjuntos aceitos, porém, só dá exatamente $\{X\}$ em **8,0%** dos casos, e
sai vazia em 89,5%. A própria causa não sobreviveu em 0,5% das vezes, e em 2,5% sobrou só o ruído
$W$, porque o teste aceitou os conjuntos que o continham e não os que continham $X$. Nos outros
86,5% o ruído $W$ entrou em alguns conjuntos e não em outros, e a interseção ficou sem denominador
comum. **Abster-se é resultado, não defeito do instrumento**, e o teste nunca devolveu
$\hat S = \{Z\}$.

**(5) Quando o mecanismo é instável, a causa também cai.** Na segunda bateria, o coeficiente de
$X$ em $Y$ muda de sítio para sítio. Os dados agora dizem a verdade sobre um mundo incoerente. A
inclinação ingênua de $X$ sobe de 2,0004 para **4,0004**, porque o estimador está misturando o
efeito verdadeiro com a variação entre sítios; $Z$ vai de 0,6015 para 1,2029, e $\{X\}$ deixa de
ser uma relação estável. Quando os sítios intervêm no mecanismo, nenhum conjunto é a resposta
certa, e o teste deve, e faz, rejeitar.

**Exercícios.**

1. *(Conceito)* Classifique no degrau de Pearl: "bairros com mais polícia têm mais crimes"; "se
   dobrarmos o policiamento, o crime cai?"; "esse assalto teria acontecido com uma viatura na
   esquina?".
2. *(Conceito)* Desenhe um grafo em que ajustar por $Z$ cria uma associação entre $X$ e $Y$ que não
   existia. Que nome tem $Z$? Dê um exemplo da sua área.
3. *(Conta)* Recalcule as taxas brutas da tabela de Charig como média dos estratos ponderada pelo
   número de casos, e explique por que ela inverte os estratos. Qual é a variável confundidora?
4. *(Conta)* No laboratório, a inclinação ingênua de $Z$ foi 0,6003 e as inclinações por sítio
   foram 0,928, 0,496 e 0,299. Explique em uma frase por que a média ingênua não é a média dessas
   três, e o que isso diz sobre a leitura do coeficiente de uma regressão só.
5. *(Laboratório)* Faça o ruído de $Z$ ser igual nos três sítios no laboratório e mostre que
   $\{Z\}$ passa a ser aceito. O que isso diz sobre a força do ICP: ele encontra a causa ou apenas
   rejeita o que varia?
6. *(Conceito)* Por que o ICP pode devolver o conjunto vazio quando tem dados demais? Relacione com
   a sensibilidade do teste a desvios do modelo.

**Resumo.**

- Associação, intervenção e contrafactual são perguntas diferentes; o grafo faz a ponte, e os dados
  sozinhos não a fazem.
- O paradoxo de Simpson nasce de uma mistura de casos: a cirurgia aberta vence em cada estrato e
  perde no bruto porque 75% dos seus pacientes eram os casos difíceis.
- Confundidor, mediador e colisor pedem três reações opostas; ajustar tudo não é neutro.
- Diferença ingênua = efeito nos tratados + viés de seleção, e só o sorteio zera o segundo termo.
- O ICP separa a causa do sintoma pela invariância entre ambientes, e abster-se é um resultado
  legítimo: no laboratório ele nunca devolveu o filho como causa.
