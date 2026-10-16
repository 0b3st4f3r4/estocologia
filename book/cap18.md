## Capítulo 18. Horizonte H1, parte 2: os incêndios dinamarqueses e a arte de reprovar o próprio modelo

**Objetivos.** Ao fim deste capítulo você deve saber por que uma janela limpa é evidência fraca, qual é o efeito de
uma cauda com $\alpha$ abaixo de 2 sobre qualquer média, como a regra de Laplace transforma "nunca aconteceu" em "uma
chance em 38" e por que reprovar o seu modelo conta como resultado.

---

### Degrau 1 — O mundo real e o paradoxo

**A resseguradora que viu três anos calmos.** No fim de 1988, uma resseguradora olha a série de sinistros de
incêndio da Copenhagen Re e o trecho que lhe interessa, a camada que ela vende, "100 acima de 50". Em palavras, ela
paga a parte de cada sinistro que fica **entre 50 e 150 milhões de coroas dinamarquesas** de 1985. A retenção de 50
milhões fica no percentil 99,7 das perdas, na ponta da distribuição.

O analista vê **36 meses seguidos, de 1986 a 1988, sem um único sinistro sequer tocar a camada.** A conclusão que
ele escreve repete a de sempre: *"o risco é desprezível (algo como um mês atingido por século) e podemos vender três
vezes o nosso capital."*

**O que a série inteira diz.** A história da camada, contada em meses atingidos:

| Período | Meses | Meses em que a camada foi atingida | Maior perda do período |
| --- | --- | --- | --- |
| 1980-1982 | 36 | 4 | 263,3 M |
| 1983-1985 | 36 | 1 | 57,4 M |
| 1986-1988 | 36 | **0** | 47,0 M |
| 1989-1990 (o cofre) | 24 | 2 | 152,4 M |

Os dois meses do cofre têm nome e data: **agosto de 1989, 152,4 milhões**, e outubro de 1990, 144,7 milhões. O
primeiro cai oito meses depois da decisão de alavancar.

**Em cada 100 janelas.** Aqui está o erro central da alegação, e ele é de contagem. No treino inteiro, a camada
foi atingida em 5 dos 108 meses, cerca de 4,6 meses em cada 100. Nesse mundo, **cerca de 18 em cada 100 janelas
de três anos saem limpas** por puro acaso. Ver uma janela limpa não é ver um mundo seguro. É ver o resultado
esperado num mundo perigoso, com mais frequência do que se imagina.

**A regra de Laplace.** Quantas chances por mês sobram depois de 36 meses limpos? A regra de sucessão responde:

$$\frac{0+1}{36+2} = \frac{1}{38} \approx 0{,}0263.$$

Em cada 100 meses, 2,6 meses atingidos, nunca zero. "Nunca aconteceu" não é "não pode acontecer". É "não
aconteceu ainda".

**A cauda.** O estimador de Hill, aplicado às 1 714 perdas do treino, dá para o índice de cauda
$\hat\alpha = 1{,}81$, com intervalo de 95% de 1,41 a 2,56 (bootstrap por blocos mensais). Isso importa porque um
índice abaixo de $2$ significa **variância indefinida**, e a média de longo prazo do prejuízo não converge. Um único
sinistro pode ser tão grande que domina todos os outros somados, e agosto de 1989 é o caso.

**A ruína.** Com a exposição que a alegação justificaria (vender limite equivalente a 3 vezes o capital), um
sinistro que consome o limite inteiro leva o capital a **1 − 3 = −2**, isto é, a zero. Aconteceu em agosto de 1989:
a resseguradora está arruinada, e a ruína é um estado absorvente, do qual não se sai. Quem usou a regra de Laplace
na janela limpa (exposição bem menor) termina 1989-1990 com **19% do capital**; quem usou a taxa da cauda ajustada
O ajuste de cauda (POT/GPD) cobre 78%. O Kelly da posterior, no ponto ótimo, é $f^* = 0{,}46$.

**A arte de reprovar o próprio modelo.** O projeto levou a alegação aos portões e o resultado foi incômodo para
todos, porque no cofre **nenhum previsor venceu a frequência histórica simples**. A habilidade de Brier do modelo
(que é a diferença entre prever bem e prever a constante histórica, Capítulo 8) ficou em **−0,015**; a do POT,
em −0,003; a da regra de Laplace, em −0,024; e a da alegação, em −0,052. Além disso, a frequência de sinistros
crescia **4,6% ao ano** (p = 2·10⁻⁶), e o mundo não era estacionário. A priori elicitada (simulada) dominava a
posterior.

A banca adversarial reprovou o projeto, e os testes não foram ajustados para que ele passasse. Uma frase resume
o veredito: **a alegação perde, mas o modelo não ganha o direito de decidir.** O F8 não emitiu decisão, os
portões 4 e 5 falharam por causa desse projeto, e é isso que o horizonte publica.

**A janela limpa em toda a série.** O projeto testou a regra em todas as janelas limpas da história. Sempre que os
últimos 36 meses não tinham tocado a camada, a decisão teria sido alavancar no teto. Foram 5 decisões assim, e as
**5 terminaram arruinadas em até 24 meses**.

---

### Degrau 2 — A regra do jogo

**A cauda selvagem.** Uma distribuição tem cauda pesada quando a probabilidade de valores extremos cai como uma
potência, $P(X > x) \propto x^{-\alpha}$, e não como uma exponencial. O estimador de Hill usa as $k$ maiores
observações:

$$\hat\alpha = \left[\frac{1}{k}\sum_{i=1}^{k}\ln\frac{x_{(i)}}{x_{(k+1)}}\right]^{-1}.$$

A leitura é direta. Quanto mais lentamente as maiores observações decrescem, menor o $\hat\alpha$. Ele vale quando
a cauda é realmente de potência e existe um patamar em $k$ onde a estimativa não muda. Falha em amostras pequenas e
quando a escolha de $k$ decide a resposta (aqui, $\hat\alpha = 1{,}96$ com $k = 25$, $1{,}81$ com $k = 50$ e
$1{,}41$ com $k = 100$). O intervalo por bootstrap de blocos mensais
(1,41 a 2,56) é a forma honesta de dizer: "sabemos que a cauda é pesada; não sabemos quanto". Um ajuste
alternativo da cauda (POT, com distribuição de Pareto generalizada acima de 10 M) dá $\xi = 0{,}41$ [0,02; 0,72],
isto é, $\alpha \approx 2{,}46$, mais leve que o Hill. Os dois métodos discordam, e a discordância está no
resultado.

**Por que $\alpha < 2$ muda tudo.** A variância de uma variável com cauda de potência é finita apenas se
$\alpha > 2$. Com $\alpha = 1{,}81$, a variância não existe, e não há "desvio padrão" do prejuízo anual, de modo que
qualquer margem calculada a partir de uma variância estimada será pequena. O caso repete a lição do Capítulo 4. Um
processo pode estar fora do espaço de hipóteses do modelo, e os dados não avisam com um gráfico bonito.

**A regra de sucessão de Laplace.** Depois de $s$ sucessos em $n$ tentativas, a probabilidade do próximo
sucesso é

$$P(\text{sucesso} \mid s, n) = \frac{s+1}{n+2}.$$

Ela vem de uma priori uniforme sobre a probabilidade e trata cada mês como permutável com os outros. Vale quando os
meses são trocáveis, sem memória e sem tendência, e falha quando o mundo tem deriva. E tem. A frequência de
sinistros cresce 4,6% ao ano, com um p-valor de $2 \times 10^{-6}$. Laplace, portanto, entrega um **piso**. Depois
de 36 meses limpos, o mínimo defensável é 1 em 38 por mês, nunca zero.

**Quanto vale uma janela limpa.** Aqui a conta deixa de ser sobre a frequência e passa a ser sobre a evidência.
Compare duas hipóteses: "o risco é desprezível" (1 mês atingido por século, $p = 1/1200$) e "o risco é o da
cauda do treino" ($p \approx 1/23$ por mês). A probabilidade de ver 36 meses limpos sob cada hipótese:

$$P(\text{janela limpa} \mid \text{desprezível}) = \left(1 - \tfrac{1}{1200}\right)^{36} = 0{,}9704, \qquad
P(\text{janela limpa} \mid \text{perigoso}) = \left(1 - p\right)^{36} = 0{,}2058.$$

A razão de verossimilhança é 4,72, ou **2,2 bits** de evidência. Dois bits é muito pouco para decidir
vender três vezes o capital. O critério de severidade diz quanto seria preciso. Para que o procedimento "a camada
não foi tocada" tivesse 95% de chance de acusar o perigo, seriam necessários **69 meses limpos**, quase seis anos.
A evidência também se mede do outro jeito, como informação mútua entre o resultado da janela e a hipótese
perigoso-contra-desprezível, **0,51 bits**, praticamente nada. E, para comparar, se o mundo real fosse o do cofre
(2 meses atingidos em 24, $p = 1/12$), essa janela limpa valeria 4,5 bits. Três anos limpos são evidência
razoável contra uma taxa mensal de 8% e evidência ridícula contra uma de 4%.

**Habilidade de Brier.** A nota de um previsor vem de uma regra de pontuação, e a referência é a constante
histórica:

$$\text{habilidade} = 1 - \frac{\text{BS}_{\text{previsor}}}{\text{BS}_{\text{climatologia}}}.$$

Habilidade positiva significa que o previsor bate a constante; **negativa, que ele perde para a constante**. O modelo
(−0,015), o POT (−0,003) e o Laplace (−0,024) perderam; a alegação (−0,052) perdeu mais. Nenhum previsor ganhou o
direito de decidir, e o portão de governança registra isso.

**O análogo do curso.** O padrão reaparece numa lente irmã, uma estratégia que vende seguro. Em 37% dos
históricos de 250 semanas simulados não aparece perda nenhuma. "Nunca perdi" vale 1,3 bit de evidência, e um
teste severo exigiria 14 anos limpos. Dos investidores que apostam pelo Kelly estimado a partir do seu histórico,
35,8% se arruínam. A história se repete com outro ativo, e a janela limpa segue como o argumento mais sedutor e
mais fraco que existe.

**Um limite declarado do desenho.** A retenção de 50 milhões foi escolhida olhando a série inteira, e não antes de
vê-la, e por isso o cofre **não é cego** para o analista. O horizonte traz uma tabela de sensibilidade para retenções
de 20 a 50 milhões, e o desenho fica registrado como análise de um caso escolhido com o desfecho à vista, útil para
aprender sobre o método e não para estimar o risco do seguro.

**Lakatos, de novo.** Um ajuste depois de uma falha pode ser **progressivo** (prevê algo novo que se confirma) ou
**degenerativo** (só protege a teoria). O projeto C tinha todas as razões para escolher o caminho degenerativo.
Bastava trocar a retenção, mexer na priori ou mudar o período do cofre. Nada disso foi feito. Os números ficaram
como estavam, o projeto foi reprovado, e o horizonte registra a reprovação com o cuidado com que registra as
duas aprovações.

---

### Degrau 3 — Mão na massa

O laboratório lê a série real, mede Hill na cauda do treino e numa cauda simulada, conta os meses em que a camada
foi atingida e calcula Laplace, a razão de verossimilhança e os bits da janela limpa. O código está em
`crates/stochology-book/src/cap18.rs`; a saída abaixo foi medida com `cargo run --release` no crate de rascunho
`~/tmp/labs-cap18`.

```text
== Capítulo 18 :: a cauda, a janela limpa e a informação ==
incendios da Copenhagen Re acima de 1 M DKK, 1980-1990: 2167 sinistros em 132 meses

periodo        meses   meses atingidos   maior perda
1980-1982         36                 4         263.3
1983-1985         36                 1          57.4
1986-1988         36                 0          47.0
1989-1990         24                 2         152.4
meses atingidos e valor: 1980-07 263.3; 1981-05 56.2; 1981-12 50.1; 1982-10 65.7; 1985-08 57.4; 1989-08 152.4; 1990-10 144.7;

treino 1980-1988: 1714 sinistros em 108 meses
Hill no treino: k=25 alpha=1.9625 k=50 alpha=1.8109 k=100 alpha=1.4100
Hill k=50, IC 95% por bootstrap de blocos mensais (300 replicas): [1.37; 2.61]
Pareto simulado com alpha = 1.81, n = 1500, 200 repeticoes: Hill k=50 medio = 1.900 (dp 0.265), 90% das repeticoes entre 1.51 e 2.39

janela de 36 meses sem sinistro na camada:
Laplace na janela: p = (0+1)/(36+2) = 0.02632, ou 1 chance em 38 por mes
frequencia mensal medida no treino: p = 5/108 = 0.04630
p mensal da cauda do treino (ajuste POT/GPD publicado: xi = 0.406, sigma = 7.387, 83 excessos acima de 10): p = 0.04296
  frequencia medida  p = 0.04630: P(janela limpa | perigoso) = 0.1815; razao de verossimilhanca = 5.35 (2.42 bits); meses limpos para 95% = 64
                     informacao mutua janela x (perigoso vs desprezivel) = 0.5455 bits
  cauda POT/GPD      p = 0.04296: P(janela limpa | perigoso) = 0.2058; razao de verossimilhanca = 4.72 (2.24 bits); meses limpos para 95% = 69
                     informacao mutua janela x (perigoso vs desprezivel) = 0.5147 bits
referencia: sob a taxa que o cofre mostrou (2 meses atingidos em 24, p = 1/12), a mesma janela limpa valeria 4.48 bits (razao 22.3)
```

Leia linha por linha. **4, 1, 0**: os meses atingidos em cada triênio do treino; a janela limpa que a alegação
leu era real, cercada de anos com quatro e um mês atingidos. **1,9625 / 1,8109 / 1,4100**: a série dá
três índices de cauda conforme o número de observações usadas; a cauda é pesada com folga, e o valor exato dela
não é estável. **1,900 com desvio 0,265**: numa cauda simulada com $\alpha = 1{,}81$, o estimador de Hill
tem essa incerteza em amostras desse tamanho, a tradução honesta do intervalo [1,41; 2,56]. **0,1815**: em cada
100 janelas de três anos, 18 saem limpas no mundo perigoso. **0,02632**: a regra de Laplace põe o piso em 1 em 38.
**2,24 bits e 69 meses**: a janela limpa é 4,7 vezes mais provável no mundo desprezível, isto é, 2,2 bits de
evidência, e quase seis anos limpos para um teste severo. **4,48 bits**: contra a taxa do cofre, essa janela
valeria o dobro.

**Declaração de divergência.** O `h1-real/README.md` registra $\hat\alpha = 1{,}81$ com intervalo [1,41;
2,56] por bootstrap em blocos; este instrumento mede os mesmos **1,8109** no ponto (k = 50) e o intervalo **[1,37;
2,61]**, além de 1,9625 e 1,4100 para k = 25 e k = 100 — os três pontos coincidem com os publicados até a quarta
casa, e a diferença do intervalo vem da reamostragem (300 réplicas, gerador diferente). A divergência que merece
parágrafo é outra: a edição anterior e o README dizem que 36 meses limpos valem **2,2 bits** e que um teste severo
precisaria de **69 meses**. Este instrumento confirma os dois números **exatamente** (2,2373 bits e 69 meses)
quando usa o $p$ que a cauda do treino implica, $p = 0{,}04296$, o valor do ajuste POT/GPD publicado
$\xi = 0{,}406$, $\sigma = 7{,}387$, 83 excessos acima de 10 — e mede **2,42 bits e 64 meses** quando usa a
frequência mensal simples do treino, $p = 5/108 = 0{,}04630$. A diferença não é de conta: é de qual mundo
"perigoso" se coloca do outro lado. A cauda ajustada é mais pessimista que a contagem crua de meses, e por isso
exige mais meses limpos. Os dois valores ficam registrados; nenhum foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 18 | perda de resolucao 0.00 min (demanda plana) | os incêndios: o número muda de janela, e a janela é o grão
```

**Exercícios.**

1. *(Conta)* Com a regra de Laplace, calcule a chance mensal depois de 12, 36 e 69 meses limpos. Quantos meses
   limpos seriam precisos para que a chance mensal caia abaixo de 1 em 100?
2. *(Conceito)* Explique por que $\alpha = 1{,}81$ impede calcular uma margem de erro honesta para o prejuízo
   anual, e por que o intervalo [1,41; 2,56] atravessa a fronteira em que a variância existe.
3. *(Conta)* Usando $P(\text{limpa} \mid p) = (1-p)^{36}$, calcule a razão de verossimilhança entre
   $p = 1/1200$ e $p = 1/12$. Em bits, e com palavras: o que esses bits autorizam concluir?
4. *(Laboratório)* Rode `crates/stochology-book/src/cap18.rs` com (k = 50) e depois troque a janela de 36 para 72 meses
   limpos: quantos bits vale a janela maior? Quantos meses seriam precisos para 95%?
5. *(Conceito)* "A alegação perde, mas o modelo não ganha o direito de decidir." Explique essa frase para um
   gestor de risco que só quer saber se deve ou não vender a camada.
6. *(Laboratório)* Simule 100 anos de meses i.i.d. com $p = 1/1200$ e conte quantas janelas de 36 meses saem
   limpas. Compare com a história real da série dinamarquesa e diga o que isso faz com o argumento "risco
   desprezível".

**Resumo.**

- 36 meses limpos entre 50 e 150 milhões de coroas não são um mundo seguro. Com a taxa do treino, 18 em cada 100
  janelas de três anos saem limpas por acaso.
- A cauda é pesada ($\hat\alpha = 1{,}81$, intervalo 1,41 a 2,56). Com $\alpha < 2$ a variância não existe, e um
  único mês (agosto de 1989, 152 milhões) decide o destino da empresa.
- A janela limpa vale 2,2 bits de evidência; a regra de Laplace dá 1 chance em 38 por mês; um teste severo
  precisaria de 69 meses limpos.
- No cofre, nenhum previsor venceu a frequência histórica simples: modelo −0,015, POT −0,003, Laplace −0,024,
  alegação −0,052, e a alegação perde sem que o modelo ganhe o direito de decidir.
- Reprovar o modelo que se escreveu, sem mexer nos testes para que ele passe, é um resultado do programa.
