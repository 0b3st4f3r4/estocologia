## Capítulo 3. Probabilidade sem mistério: apostas, árvores e o teorema de Bayes

**Objetivos.** Ao fim deste capítulo você deve saber ler as cinco interpretações da probabilidade,
reconhecer um livro holandês, calcular uma posterior com árvores de frequências naturais e com
chances, medir a força de uma evidência em decibans e separar as três perguntas de Royall.

---

### Degrau 1 — O mundo real e o paradoxo

**"Há 70% de chance de chuva amanhã."** Qual é o sentido dessa frase?

- Pode querer dizer que *dos 10 dias em que o céu esteve assim, choveu em 7*. Uma contagem do mundo.
- Pode querer dizer que *o modelo numérico, rodado 10 vezes com pequenas perturbações, deu chuva em 7*. Um
  produto do modelo.
- Pode querer dizer que *eu apostaria 7 contra 3 nisso*. Uma disposição de quem decide.
- Pode querer dizer que *dos casos igualmente possíveis, 7 favorecem a chuva*. Uma contagem do modelo.
- Pode querer dizer que *dado o meu conhecimento, o grau racional de apoio à chuva é 0,7*. Uma relação entre
  proposições.

São cinco coisas diferentes com um número igual em cima. Quando alguém lê uma como se fosse a outra, o
erro não está na aritmética. Está na ponte.

**O amigo que oferece dois bilhetes.** Seu amigo diz que a chance de chover é 60% e a chance de não
chover é 50%. Você pode comprar dele dois bilhetes. Um paga 1 real se chover, outro paga 1 real se não
chover. Ele cobra 0,60 pelo primeiro, 0,50 pelo segundo, 1,10 no total. Chova ou não chova, **um** dos
dois bilhetes paga 1 real. Você recebe 1,00 e pagou 1,10. Perdeu 0,10 sem risco nenhum.

Isso é um **livro holandês**: uma combinação de apostas em que se perde dinheiro com certeza. A regra
da soma existe para impedir essa combinação. Crenças que obedecem à probabilidade não têm livro
holandês contra elas.

**Em cada 1.000 mulheres.** Um exame detecta uma doença. Em cada 10.000 mulheres, 100 têm a doença e
9.900 não têm. O exame acerta em 90 dos 100 casos da doença (90%) e dá positivo em 9% das saudáveis.
Vamos contar quem dá positivo:

- 90 doentes dão positivo.
- Das 9.900 saudáveis, 9% dão positivo. São 891 mulheres.
- Total de positivos: 90 + 891 = 981.

**Das 981 mulheres com exame positivo, apenas 90 estão doentes**, cerca de 1 em cada 11. É essa a
resposta que interessa a quem acabou de receber o exame, e não o "90%" que o laboratório anuncia.

Quando Gigerenzer e colegas fizeram essa pergunta a 160 ginecologistas, **só 21% escolheram "cerca de 1
em 10"**. Não é burrice; é a ponte entre a verossimilhança que o exame declara e a posterior que a
paciente quer saber.

**A conta do tribunal.** No caso Sally Clark, a acusação apresentou o número "1 em 73 milhões", obtido
ao elevar 1/8.543 ao quadrado, como se as duas mortes fossem independentes. O promotor tratou esse
número como a chance de a mãe ser inocente. Aquele número era a chance de uma coincidência; a resposta
sobre a inocência exigia a prevalência no Mundo. Essa é a ponte do Capítulo 2, agora com consequências
fatais.

---

### Degrau 2 — A regra do jogo

**Por que a crença obedece à probabilidade.** Dois argumentos independentes. Cox (1946) mostrou que
qualquer medida de plausibilidade consistente obedece, depois de uma reescala, às regras da soma e do
produto. Ramsey e de Finetti mediram a crença pelo preço de uma aposta. Um conjunto de preços admite
livro holandês se e somente se viola os axiomas.

**As cinco interpretações.** Cada um dos sentidos do "70%" tem um nome, e cada um mora numa camada.

| Interpretação | A probabilidade é | Camada |
| --- | --- | --- |
| Clássica (Laplace) | Casos favoráveis sobre casos igualmente possíveis | Modelo |
| Frequentista | O limite da frequência relativa numa série longa | Mundo |
| Propensão (Popper) | Uma disposição física do arranjo experimental | Mundo |
| Lógica (Keynes, Carnap) | Um grau de apoio parcial entre proposições | Modelo |
| Subjetiva (Ramsey, de Finetti) | O grau de crença de um agente, medido pelo preço de uma aposta | Agente |

A Estocologia não escolhe uma. **Tipa cada número pela camada de onde veio**. Muitos erros do Capítulo
2 são trocas de leitura.

**Coerência é barata.** Um agente que comete a falácia da conjunção perde 0,05 por rodada para o
apostador holandês e 0,60 para quem conhece o mundo. Torná-lo coerente reduz a perda só para 0,55. O
erro mais visível não era o mais caro. **Quem pune o desacordo com o Mundo é a calibração** (daí o
Capítulo 8).

**Bayes em chances.** Escrita em chances, a regra de Bayes separa a razão de verossimilhança da chance
anterior:

$$\frac{P(H\mid E)}{P(\neg H\mid E)}=\frac{P(E\mid H)}{P(E\mid \neg H)}\times\frac{P(H)}{P(\neg H)}$$

*Chance posterior = razão de verossimilhança × chance anterior.* Uma evidência é forte quando a razão
se afasta de 1; o valor de $P(E \mid H)$ não mede essa força.

**Máxima entropia.** Quando a informação é incompleta, Jaynes (1957) manda escolher, entre as
distribuições compatíveis com a informação disponível, a de maior entropia. Um dado de seis faces com
média 4,5 recebe então cerca de 5% na face 1 e 35% na face 6; qualquer outra escolha supõe algo que não
foi declarado.

**Decibans.** Turing e Good mediram a força da evidência em decibans, dez vezes o logaritmo decimal da
razão de verossimilhança. Nessa escala, evidências independentes **se somam**. A independência é a
condição que mais se esquece. Dois exames positivos com uma causa comum de erro não valem o dobro.

**Royall em três perguntas.** Royall separou três perguntas que costumam ser respondidas com um único
número: *o que estes dados dizem sobre H contra H′?* (razão de verossimilhança); *no que devo acreditar
agora?* (exige uma priori); *o que devo fazer?* (exige custos e perdas, Capítulo 11). O p-valor não
responde bem a nenhuma das três.

**Quanto dado basta.** O valor esperado do peso da evidência é a divergência de Kullback-Leibler, que
nunca é negativa:

$$D(P_1\,\|\,P_0)=\sum_x P_1(x)\log\frac{P_1(x)}{P_0(x)}\ \ge 0$$

Para separar uma moeda de 60% de uma honesta com chances de 100 para 1, são precisos cerca de **229
lances**. E a evidência engana pouco. Se a hipótese nula é verdadeira, a chance de a razão de
verossimilhança alguma vez chegar a $k$ é no máximo $1/k$, qualquer que seja o tamanho da amostra:

$$P_{H_0}\Big(\sup_n \mathrm{RV}_n \ge k\Big)\le \frac{1}{k}$$

Em simulação, só **0,85%** dos caminhos sob a nula chegaram a 20 decibans (razão 100), contra um limite
de 1%.

---

### Degrau 3 — Mão na massa

O laboratório constrói a árvore, mede a força do segundo exame e mostra quanto a dependência entre
exames custa. Código em `crates/stochology-book/src/cap03.rs`; saída medida com
`cargo run --release --bin labs -- --cap 3`:

```text
== Capítulo 3 :: árvores, chances e decibans ==
mamografia: 100 doentes, 90 positivos doentes, 9900 saudaveis, 891 falsos positivos, 981 positivas
P(doente|positivo) = 0.0917  (uma em cada 10.9 positivas)
segundo exame: independente = 10.0 dB, correlacionado (razao efetiva 4,2) = 6.23 dB
posterior com dois positivos: independentes = 0.5025, correlacionados = 0.2979
```

Três lições em três linhas. **(1)** A árvore dá 981 positivas, das quais 90 doentes. Isso é 0,0917, ou
uma em cada 10,9, e não 90%. **(2)** O segundo exame vale 10 decibans se for independente do primeiro e
6,23 decibans se houver uma causa comum de falso positivo (tecida densa, por exemplo), com razão
efetiva caindo de 10 para 4,2. **(3)** Com dois positivos, a crença sai de 1% para 50,25% no caso
independente e para 29,79% no caso correlacionado. A independência não era um detalhe. É quase a
metade da conclusão. O número do repositório para o caso correlacionado é 30%, o valor arredondado que
aparece no roteiro do curso.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  3 | sem perda medida neste capitulo | probabilidade sem decisão de escala
```

**Exercícios.**

1. *(Conta)* Refaça a árvore da mamografia para 1.000 mulheres (em vez de 10.000) e confira que a
   resposta é a mesma. Quantas positivas, quantas doentes?
2. *(Conta)* Uma mulher dá positivo na segunda mamografia. Quantos decibans vale o segundo exame se ele
   for independente, e quantos com a razão efetiva de 4,2? Qual é a posterior em cada caso?
3. *(Conceito)* Por que "1 em 73 milhões" cometeu dois erros diferentes? Nomeie a ponte ou a camada de
   cada um.
4. *(Conta)* Quantos lances são necessários para separar, com chances de 10 para 1, uma moeda de 0,55 de
   uma honesta? Use a divergência de Kullback-Leibler e diga em que sentido a resposta é uma média.
5. *(Laboratório)* Troque a razão efetiva de 4,2 por 2,0 no laboratório e refaça a terceira linha. Em
   que ponto a dependência entre exames destrói a conclusão?
6. *(Conceito)* Dê um exemplo profissional em que você viu alguém responder à pergunta "no que devo
   acreditar?" usando o número que responde a "o que os dados dizem?".

**Resumo.**

- Cinco interpretações de probabilidade, tipadas por camada; a Estocologia não escolhe uma.
- Coerência evita o livro holandês; calibração evita perder para o Mundo e custa mais.
- Bayes em chances separa a força da evidência (razão de verossimilhança) da crença anterior; em
  frequências naturais, a mesma conta fica visível numa árvore.
- Decibans somam quando as evidências são independentes; a independência é a hipótese mais frágil e a
  mais esquecida.
