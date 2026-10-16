# Parte II. As armadilhas do mundo e dos dados

## Capítulo 5. Quando a média mente: curvas e a desigualdade de Jensen

**Objetivos.** Ao fim deste capítulo você deve saber olhar um gráfico e dizer se a curva é convexa ou
côncava; enunciar a desigualdade de Jensen e a aproximação de segunda ordem que mede o pedágio da
variância; reconhecer quando o caso médio deixa de ser o resultado médio; e calcular toda margem na
unidade de fato independente.

---

### Degrau 1 — O mundo real e o paradoxo

**O posto de saúde.** Em cada 100 dias, o posto recebe pacientes assim: em 50 dias chegam 5 pessoas por
hora; nos outros 50, chegam 11. A média é 8 pessoas por hora. O posto consegue atender 12 por hora.

O gestor olha a planilha e diz: *"a demanda média é 8, o posto atende 12, sobra folga; a espera média
tem de ficar em torno de 10 minutos, e não há por que contratar"*. Nenhuma conta errada foi feita. E,
ainda assim, o mundo que a planilha descreve não existe.

A tabela abaixo retoma os dois mundos do Capítulo 1, repetidos aqui para medir o pedágio, e nos dois a
demanda média é de 8 pacientes por hora.

| Mundo | Espera média por paciente | Em cada 100 pacientes, quantos esperam mais de 30 min |
| --- | --- | --- |
| Sempre 8 por hora | 9,3 minutos | 8 |
| Metade dos dias com 5, metade com 11 | 20,7 minutos | 25 |

A média não mudou em nada. A espera média mais que dobrou, e o número de pessoas que esperam mais de meia hora
triplicou. Uma decisão sobre contratar pessoal, tomada no primeiro mundo, é aplicada no segundo.

**A curva.** Desenhe num papel a função $y = x^2$: uma parábola com a boca para cima. Marque dois
pontos, $x = 0$ e $x = 2$. A média deles é 1, e a altura da curva em 1 é 1. Mas a média das alturas é
$(0^2 + 2^2)/2 = 2$. Olhe o desenho. O ponto da curva no meio (1) fica **abaixo** da corda que liga os
dois pontos (2). É essa a assimetria que este capítulo mede.

Numa função **convexa** — boca para cima, como $x^2$ —, a média das alturas é sempre maior ou igual à
altura da média. Numa função **côncava** — boca para baixo, como $-x^2$ ou a raiz quadrada —, vale o
contrário: a média das alturas fica abaixo da altura da média.

**A fila é uma curva de boca para cima.** Quando a demanda chega perto da capacidade de atendimento, a
espera não cresce em linha reta; cresce cada vez mais rápido. No mundo do posto, isso significa que o
dia cheio não é apenas o espelho do dia vazio. O dia cheio cobra muito mais do que o dia vazio
devolve.

**A turma de média 6,2.** Numa prova com nota mínima 6,0, a média da turma foi 6,2. Quem lê só a média
conclui que "o aluno médio foi aprovado". A conclusão exigiria um aluno que não existe, e a contagem
desmente o resumo: **em cada 100 alunos, 45 ficaram abaixo de 6,0**. A média de 6,2 convive sem nenhum
problema com 45% de reprovação. Como no posto, a média não é um resumo errado; é um resumo **insuficiente** para a pergunta
que importa.

---

### Degrau 2 — A regra do jogo

**Convexidade, em uma linha.** Uma função $f$ é convexa quando, para quaisquer dois pontos, a corda que
os liga fica **acima** do gráfico. Ela é côncava quando a corda fica **abaixo**. A reta é as duas
coisas ao mesmo tempo, e é o único caso em que as duas coincidem.

**A desigualdade de Jensen.** Para uma função convexa $f$ e uma grandeza aleatória $X$:

$$\mathbb{E}[f(X)] \;\ge\; f(\mathbb{E}[X])$$

Em palavras, a média do resultado **não** é o resultado do caso médio. A igualdade vale em dois casos, e
só neles: quando $f$ é uma reta, ou quando $X$ não varia. Fora desses dois, sobra uma diferença — o
pedágio.

Para uma função côncava, a desigualdade se inverte: $\mathbb{E}[f(X)] \le f(\mathbb{E}[X])$.

**Quanto custa o pedágio.** Se $X$ fica perto de sua média $\mu$, a diferença é aproximadamente
proporcional à **curvatura** e à **variância**:

$$\mathbb{E}[f(X)] - f(\mathbb{E}[X]) \;\approx\; \tfrac{1}{2}\, f''(\mu)\,\operatorname{Var}(X)$$

A fórmula se lê como um preço, **curvatura vezes dispersão**. A condição em que ela vale é a de que a
variância seja pequena o bastante para a curva parecer uma parábola naquela vizinhança. Ela **falha**
quando a dispersão é grande, e o posto é exatamente esse caso.

**A fila do posto.** A espera média de longo prazo de uma fila com um atendente, com chegadas de taxa
$\lambda$ por hora e serviço de taxa $\mu$ por hora, é

$$W(\lambda)=\frac{\lambda}{\mu(\mu-\lambda)}$$

Com $\mu = 12$ e $\lambda = 8$, isso dá 10 minutos. Aplicada aos dois dias do mundo que oscila,
$W(5) = 3,57$ min e $W(11) = 55$ min, e a média dos dois é 29,3 minutos — quase o triplo dos 10
minutos da alegação. A derivada segunda de $W$ em $\lambda = 8$ é
$f''(8) = 120/(\mu-8)^3 = 1,875$ minuto por (paciente/hora)². Com a variância de $X$ igual a 9, o pedágio de segunda ordem é
$\tfrac{1}{2}\cdot 1,875\cdot 9 = 8,44$ minutos. É muito, mas é menos que os 19,3 minutos que faltam.
**A aproximação de segunda ordem subestima o pedágio quando a dispersão é grande**, porque a curva se
afasta da parábola antes de o dia terminar.

**A nota média e o limiar.** Nem toda função torta é uma curva do segundo grau. Reprovar é um
**limiar**: o desfecho de cada aluno é um degrau em 6,0. A média da turma responde a pergunta "qual é a
nota típica?" e não responde a pergunta "quantos reprovam?". Quem precisa da segunda pergunta e usa a
primeira está trocando o resumo pelo resultado, e a troca não tem conserto por mais alunos que se
incluam.

**A unidade da margem.** Todo número deste livro vem com uma margem, e a margem só tem sentido se for
calculada na **unidade de fato independente**. No posto, a pergunta é esta. Os 320 mil pacientes são 320
mil observações independentes, ou os 4 mil dias são 4 mil observações independentes? Pacientes de um
mesmo dia compartilham a mesma demanda, o mesmo servidor e a mesma fila. Tratá-los como independentes é
**pseudorreplicação**, e ela faz a margem parecer cinco vezes menor do que é. O teste que decide, sem
convenção nenhuma, é o **nulo de metades**: repartir os dias ao acaso em duas metades e medir o desvio
da diferença entre as duas médias. O nulo mede o que a margem deve prever.

**A proibição da média solitária.** Quando a função é não linear, a dinâmica multiplicativa ou a cauda
pesada, é proibido reportar só a média. Reportam-se o caso médio $f(\mathbb{E}[X])$, a média simulada
$\mathbb{E}[f(X)]$, a mediana, os quantis e a curva acumulada. O Capítulo 14 formaliza a proibição; aqui
ela nasce do pedágio que acabamos de medir.

---

### Degrau 3 — Mão na massa

O laboratório roda quatro mundos do posto, todos com a demanda média de 8, mede o nulo de metades em dois
deles e conta os reprovados da turma. Código em `crates/stochology-book/src/cap05.rs`; saída medida com
`cargo run --release`:

```text
== Capítulo 5 :: quando a média mente ==
W(5) = 3.57 min, W(8) = 10.00 min, W(11) = 55.00 min
Jensen: f(E[X]) = 10.00 min | E[f(X)] = 29.29 (5 ou 11) e 14.10 (uniforme 5..11)
segunda ordem: meia f''(8) Var(X) = 8.44 min (5 ou 11) e 2.81 min (uniforme)
                  8 todo dia:  319405 pacientes | media   9.1 min | mediana   3.7 | p90  26.2 | >30 min:   7.6 em 100
                     5 ou 11:  319115 pacientes | media  19.6 min | mediana   8.6 | p90  56.1 | >30 min:  23.7 em 100
              uniforme 5..11:  319843 pacientes | media  13.8 min | mediana   5.5 | p90  39.8 | >30 min:  15.0 em 100
   5 ou 11, reforco no surto:  319115 pacientes | media   2.5 min | mediana   0.0 | p90   8.1 | >30 min:   0.4 em 100
nos dias de surto: 27.0 min em 219458 pacientes (68.8% do total); nos dias leves: 3.4 min
nulo de metades no mundo  8 todo dia: desvio medido 0.201 min | margem por dia 0.185 | por paciente 0.046 | variancia da espera 168 min^2
nulo de metades no mundo     5 ou 11: desvio medido 0.742 min | margem por dia 0.563 | por paciente 0.097 | variancia da espera 747 min^2
diferenca entre os mundos = 10.6 min = 52.7 desvios do nulo
mediana do mundo 5 ou 11 = 8.6 min, contra a espera media de 9.1 min do mundo constante
turma de 100000 alunos: media = 6.21, reprovados (nota < 6,0) = 45.1 em 100
```

Cinco leituras. **(1)** A média da demanda é 8 nos quatro mundos e a espera média vai de 2,5 a 19,6
minutos. A variabilidade, não a média, decide. **(2)** No mundo que oscila, **68,8% dos pacientes**
chegam nos dias de surto, e é por isso que a média por paciente pesa os dias ruins: 27,0 minutos nos
dias de surto contra 3,4 nos dias leves. **(3)** O pedágio de segunda ordem (8,44 min) captura menos da
metade da diferença verdadeira (19,29 min entre 10,00 e 29,29). A fórmula é local, e o posto é um caso
de dispersão grande. **(4)** O nulo de metades dá 0,201 minuto no mundo constante, e a margem calculada
por dia prevê 0,185 — quase igual; a margem por paciente prevê 0,046, quatro vezes menor. No mundo que
oscila a diferença entre as duas leituras é de 5,8 vezes: 0,563 contra 0,097. **(5)** A mediana do
mundo que oscila é 8,6 minutos, praticamente a média de 9,1 min que a alegação prometia. Quem olha "o
paciente típico" vê a promessa cumprida, e a diferença está toda na cauda. A diferença entre os mundos,
10,6 minutos, fica a 52,7 desvios do nulo.

**Declaração de divergência.** O material do curso e o README do H0 publicam, para estes mesmos mundos,
9,3 min e 20,7 min de espera média, **8** e **25** pacientes em cada 100 acima de 30 minutos, margem de
±0,51 minuto por dia contra ±0,07 por paciente, e um nulo de metades de 0,216 minuto com margens
previstas de 0,227 (por dia) e 0,047 (por paciente). Este instrumento mede **9,1** e **19,6** minutos,
**7,6** e **23,7** em cada 100, um nulo de **0,201** minuto com margens de **0,185** (por dia) e
**0,046** (por paciente) no mundo constante, e, no mundo que oscila, **0,742** de nulo com margens de
**0,563** e **0,097**. O mundo uniforme dá 13,8 min aqui contra 12,3 no material. Os números estão
todos na mesma vizinhança, e a diferença tem duas causas prováveis: o gerador (o material usa o PCG64
do NumPy com `SeedSequence(1909)`; aqui, o SplitMix64 com a semente 1909) e o começo de cada dia, que
aqui é sempre uma fila vazia. A identidade de Jensen, o sinal do pedágio e a conclusão sobre a unidade
da margem não dependem de nenhuma delas. Nenhum valor antigo foi apagado.

**Exercícios.**

1. *(Conceito)* Para cada caso, diga se a média esconde um pedágio e de que sinal: (a) o tempo de
   espera numa fila; (b) o saldo de uma conta com rendimento fixo; (c) o patrimônio de quem aposta 100%
   do capital a cada rodada; (d) a nota de uma turma com corte de aprovação.
2. *(Conta)* Com $W(\lambda)$ e $\mu = 12$, calcule $W(6)$, $W(8)$ e $W(10)$. Depois calcule
   $f(\mathbb{E}[X])$ e $\mathbb{E}[f(X)]$ para um mundo que alterna entre 6 e 10, e compare o
   pedágio verdadeiro com $\tfrac{1}{2}f''(8)\operatorname{Var}(X)$.
3. *(Conta)* No laboratório, a margem por dia no mundo que oscila é 0,563 minuto e a margem por
   paciente é 0,097. Em quantas vezes uma subestima a outra? Se um relatório publicasse a margem por
   paciente para afirmar que a diferença de 10,6 minutos é significativa, o que ele teria errado?
4. *(Laboratório)* Troque o mundo "5 ou 11" por "4 ou 12" e depois por "7 ou 9", mantendo a média 8.
   Meça a espera média e a fração acima de 30 minutos nos três casos. Desenhe os três pontos e diga se
   a espera média cresce como a variância ou mais rápido.
5. *(Conceito)* Escreva o memorando de duas frases que o gestor do posto deveria receber, usando
   frequências naturais e sem citar nenhuma média sem o resto da distribuição.
6. *(Laboratório)* No laboratório, o reforço só nos dias de surto dobra a capacidade do dia cheio e
   derruba a espera média de 19,6 para 2,5 minutos. Meça esse mundo com reforço de 1,5 vez e de
   3 vezes a capacidade e diga, em frequências naturais, quantos pacientes em cada 100 ainda esperam
   mais de 30 minutos.

**Resumo.**

- A média da entrada não é a média da saída quando a função é curva: $\mathbb{E}[f(X)] \ge
  f(\mathbb{E}[X])$ para $f$ convexa.
- O pedágio cresce com a curvatura e com a variância, e a fórmula de segunda ordem **subestima** o
  pedágio quando a dispersão é grande.
- Mesma demanda média, esperas muito diferentes: quem simula o mundo vê o que a planilha esconde.
- A margem vale o que vale a unidade independente; o nulo de metades decide qual unidade é a certa.
- Um limiar (reprovar, transbordar, falir) não se resume por média nenhuma.
