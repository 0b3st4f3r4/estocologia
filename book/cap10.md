## Capítulo 10. O tempo e a ruína: a moeda de Peters, Kelly e caudas selvagens

**Objetivos.** Ao fim deste capítulo você deve saber distinguir a média de conjunto da média no tempo,
calcular o arrasto da volatilidade sem cálculo avançado, aplicar o critério de Kelly com a suavização
de Laplace e reconhecer quando a média de uma distribuição não existe ou engana.

---

### Degrau 1 — O mundo real e o paradoxo

**A moeda que enriquece a multidão e arruína a pessoa.** Uma aposta oferece 50% de ganho contra 40% de
perda, com chances iguais; a cara multiplica o capital apostado por 1,5 e a coroa por 0,6. Vamos supor que
alguém aposte toda a riqueza, rodada após rodada, começando com 1 real.

- A conta de uma rodada média. Metade das vezes ganha 0,5 e metade perde 0,4. Em média, $+0{,}05$ por
  rodada, ou cinco por cento por rodada.
- A conta de uma cara **e** uma coroa: $1{,}5 \times 0{,}6 = 0{,}90$. Quem vive as duas rodadas
  perdeu 10%. Uma cara e uma coroa não devolvem o ponto de partida, porque 50% para cima e 40% para
  baixo **são números diferentes**.

Em cada 100 apostadores que apostam tudo, ao fim de **10 rodadas** 37 terminam acima do início, ao fim
de **100 rodadas** 13, e ao fim de **1 000 rodadas** nenhum. A riqueza média, essa, explode. Numa
simulação medida com 10 000 trajetórias de 1 000 rodadas, a mediana final de quem aposta tudo é
A probabilidade de a coorte terminar naquele valor é $1{,}322 \times 10^{-23}$; nos módulos seguintes veremos por que a média amostral daquela coorte
ainda assim cresce.

O nome disso é **não-ergodicidade**, e ela obriga a fazer uma pergunta simples. *De quem é a
média?* A média de 10 000 pessoas num instante não é a média de uma pessoa ao longo de 10 000
rodadas. Quem vive uma trajetória precisa da segunda; quem administra um fundo com muitos cotistas
pode viver da primeira, e a primeira é justamente a que não paga a conta de quem aposta.

**A mesma aposta, duas respostas.** Essa aposta não é ruim por si. Ela é boa para a
multidão (a esperança cresce 5% por rodada) e péssima para a trajetória típica (a mediana cai). Aquilo que
separa as duas leituras não é otimismo nem pessimismo. É **em que unidade se está contando**.

---

### Degrau 2 — A regra do jogo

**Média de conjunto contra média no tempo.** Se a riqueza de cada rodada multiplica por $M_t$,

$$X_t = X_0 \prod_{s=1}^{t} M_s$$

a média de conjunto cresce quando $\mathbb{E}[M] > 1$, mas numa trajetória acumula-se o
logaritmo do produto, e pela lei dos grandes números ele vai para $\mathbb{E}[\ln M]$. Quando
$$\mathbb{E}[M] > 1 \quad \text{e} \quad \mathbb{E}[\ln M] < 0,$$
a multidão enriquece e quase toda trajetória decai. Na moeda de Peters:

$$\ln \mathbb{E}[M] \approx +0{,}049, \qquad \mathbb{E}[\ln M] = \tfrac12\ln 1{,}5 + \tfrac12\ln 0{,}6 \approx -0{,}053$$

O instrumento do capítulo mede $-0{,}052680$ por rodada, o que dá $-5{,}268\%$ de crescimento temporal.
A diferença entre as duas contas, $+5{,}00\%$ e $-5{,}27\%$, mede a distância entre a linha de uma
planilha e a vida de quem aposta.

| Rodadas | Trajetórias acima do ponto de partida |
| --- | --- |
| 10 | 37,7% |
| 100 | 13,6% |
| 1 000 | 0,01% |

Em 1 000 rodadas a esperança vale $1{,}55 \times 10^{21}$ e a mediana $10^{-23}$. Os fatores são
independentes e idênticos; **o produto é que não é ergódico**. Até quem simula cai na armadilha, porque a
média de $N$ trajetórias acompanha a esperança só por um tempo que cresce como $\ln N$, cerca de 12
rodadas com 10 trajetórias e 112 com um milhão.

**Quanto expor: o critério de Kelly (1956).** Em vez de apostar tudo, escolhe-se a fração $f$ da
riqueza:

$$f^* = \arg\max_f \; \mathbb{E}[\ln(1 + fR)]$$

Numa aposta paga de igual para igual com chance $p$, a fração ótima é $f^* = 2p - 1$, ou seja, apostar
20% com chance de 0,6. Numa aposta que paga 5 contra 4, como a moeda de Peters, $f^* = p|b| - (1-p)a =
0{,}9p - 0{,}4$, isto é, 14% da riqueza. Apostar tudo levaria a esperança, em 100 rodadas, a
$8{,}3 \times 10^7$ vezes o capital, com chance de sobreviver de $0{,}6^{100}$, cerca de $10^{-22}$.
O dobro da fração ótima já empobrece com essa mesma vantagem, e o instrumento mede $g(0{,}28)$ já abaixo de
zero e $g(0{,}70) = -0{,}0075$ por rodada.

**A incerteza epistêmica é um argumento para apostar menos.** Com a probabilidade estimada em 25
rodadas, o Kelly cheio tem taxa de crescimento negativa em 16% das estimativas, enquanto meio Kelly
garante 75% do crescimento com um quarto da variância.

**A suavização de Laplace.** O estimador mais simples de uma probabilidade é a frequência observada
$k/n$. Ele tem um defeito fatal em eventos raros. Com zero perdas em 250 semanas, ele diz que a
probabilidade é **zero**, e nenhuma alavancagem é demais para uma chance nula. A regra de sucessão de
Laplace dá o valor corrigido.

$$P(\text{sucesso amanhã} \mid s, n) = \frac{s+1}{n+2}$$

Com 250 semanas limpas, a chance de perda não é zero. Ela é $1/252 = 0{,}003968$. A regra não é neutra,
porque vem de uma priori uniforme, mas impede a conclusão absurda de que o risco acabou.

**Seguro e cooperação.** No navio de Bernoulli, retomado por Peters e Adamou, um armador com riqueza
de 100 000 paga 1 800 para cobrir uma perda de 34 000 que ocorre com chance de 5%. Em esperança, ele
perde 100 por mês e a seguradora ganha 100. Em crescimento temporal, os dois saem ganhando, e o armador passa
de 1,94% para 2,18% ao mês, e a seguradora, que detém muitas trajetórias independentes, de zero para
+0,007%. Na moeda de Peters, três jogadores que repartem a riqueza a cada rodada crescem 1,6% por
rodada, quando cada um sozinho decairia. **A pergunta que decide o caso é quem detém as trajetórias.**

**Quando a média deixa de existir.** Numa distribuição de Pareto, com $P(X > x) = x^{-\alpha}$, o
momento de ordem $k$ existe se e somente se $k < \alpha$. Com $\alpha = 1{,}1$, a média existe e vale
11, mas a média de um milhão de observações tem mediana 8,3, fica abaixo de 11 em 91% das amostras, e
o intervalo de 95% usual contém o valor verdadeiro em só 26% delas. A fração da média que vem de
valores acima de $m$ é $m^{1-\alpha}$; em mediana, 28% da média verdadeira vêm de valores maiores que
o maior que a amostra de um milhão viu.

**Os três estados do acaso (Mandelbrot).** Mandelbrot classificou o acaso pelo modo como as somas se
comportam.

| Estado | Exemplo de distribuição | O que acontece com a soma |
| --- | --- | --- |
| Brando | Gaussiana, exponencial | Nenhuma observação domina; a média converge depressa |
| Lento | Lognormal | Todos os momentos existem, mas a convergência engana em amostras reais |
| Selvagem | Pareto com $\alpha \le 2$ | Uma observação pode dominar a soma; a variância, ou a média, não existe |

O nome disso é **Fisher–Tippett–Gnedenko**. O máximo de muitos sorteios só pode convergir para três famílias, Gumbel, Fréchet ou Weibull, e é por isso que a cauda se estima pelo máximo, não pela média. No registro do curso, a perda de 1 em 100 anos sai em 17,4% pela GEV contra 3,9% da gaussiana: mais de quatro vezes.

Taleb chamou o primeiro de Mediocristão e o último de Extremistão, e acrescentou o tipo de
consequência. O "quarto quadrante", onde a cauda é selvagem e a perda depende do tamanho do evento, é
onde os modelos falham mais e custam mais. O estimador de Hill estima o expoente da cauda a partir dos
$k$ maiores valores, $\hat\alpha = k / \sum \ln(x_i / x_{(k)})$. Os incêndios dinamarqueses, com
$\alpha$ estimado em 1,81 (intervalo de 1,41 a 2,56), estão nesse território (Capítulo 18).

---

### Degrau 3 — Mão na massa

Quatro tarefas compõem o laboratório. Medir a moeda de Peters com a fração ótima e com a aposta total;
medir 10 000 vendedores de seguro com e sem Laplace; estimar a cauda com Hill; e medir o arrasto da
volatilidade em $1\times$, $2\times$ e $3\times$. Código verificado em `crates/stochology-book/src/cap10.rs`; saída medida com o
crate de rascunho `labs_cap10`:

```text
== Capítulo 10 :: a moeda de Peters (cara +50%, coroa -40%) ==
média de conjunto por rodada = 1.0500  (+5.00% por rodada)
média no tempo por rodada  = -0.052680  (-5.268% por rodada)
produto de uma cara e uma coroa: 1,5 x 0,6 = 0.90
Kelly cheio (5:4): f* = 0.1400 da riqueza; com pagamento de igual para igual, 2p - 1 = 0.20
crescimento no tempo com Kelly cheio: +0.0175 por rodada

aposta total (f = 1,00)    acima do início: 10 rodadas 0.374  100 rodadas 0.128  1000 rodadas 0.00000
                           ruína (< 1e-6 do capital) = 0.9971  mediana final = 1.322e-23  média final = 2.725e-6
Kelly cheio (f = 0,70)     acima do início: 10 rodadas 0.374  100 rodadas 0.300  1000 rodadas 0.07830
                           ruína (< 1e-6 do capital) = 0.5089  mediana final = 6.810e-7  média final = 4.982e4
meio Kelly (f = 0,35)      acima do início: 10 rodadas 0.618  100 rodadas 0.611  1000 rodadas 0.85620
                           ruína (< 1e-6 do capital) = 0.0000  mediana final = 1.854e2  média final = 2.119e6
quarto de Kelly (f = 0,14) acima do início: 10 rodadas 0.618  100 rodadas 0.811  1000 rodadas 0.99560
                           ruína (< 1e-6 do capital) = 0.0000  mediana final = 1.506e2  média final = 1.016e3

aposta total: log10 da mediana final = -22.9, log10 da média final = -5.6
parte da média que vem da trajetória mais rica (1% mais rico): 1.000

== Capítulo 10 :: o vendedor de seguro (ganha 1% por semana, perde 50% com q = 0,004) ==
crescimento no tempo da aposta total: +0.00714 por semana (negativo: a ruína é certa)
frequência observada + Kelly pleno sem perda = 0.368  aposta total = 0.368  ruina em 1000 semanas = 0.0000  mediana final = 3.531e2  média final = 1.255e3
Laplace + Kelly pleno            sem perda = 0.368  aposta total = 0.000  ruina em 1000 semanas = 0.0000  mediana final = 8.983e1  média final = 2.529e2
Laplace + meio Kelly             sem perda = 0.368  aposta total = 0.000  ruina em 1000 semanas = 0.0000  mediana final = 1.097e1  média final = 1.371e1
probabilidade teórica de 250 semanas sem perda = 0.3671
Laplace com zero perdas em 250 semanas: q_est = 1/252 = 0.003968

== Capítulo 10 :: a cauda selvagem: Pareto com alfa = 1,1 ==
média verdadeira (alfa/(alfa-1)) = 11.0
n =      1000: média amostral média =     7.76  mediana =    6.22  abaixo de 11 em 0.94  IC95 cobre em 0.24
n =   1000000: média amostral média =     9.53  mediana =    8.39  abaixo de 11 em 0.95  IC95 cobre em 0.26
fração da média que vem de valores acima do maior visto: 0.000
em mediana, 25.12% da média verdadeira vêm de valores acima do máximo de uma amostra de um milhão

estimador de Hill com alfa = 1,1 e n = 200 000:
  k =    100  alfa_hat = 1.083  IC95 aprox [0.870; 1.295]
  k =    500  alfa_hat = 1.063  IC95 aprox [0.970; 1.157]
  k =   1000  alfa_hat = 1.100  IC95 aprox [1.032; 1.168]
  k =   5000  alfa_hat = 1.101  IC95 aprox [1.070; 1.131]
  k =  20000  alfa_hat = 1.095  IC95 aprox [1.080; 1.111]
gregarina: um modelo gaussiano ajustado a essa cauda subestima a perda em todas as repetições

== Capítulo 10 :: o arrasto da volatilidade em 1x, 2x e 3x ==
L = 1x: media aritmetica = +0.000317/sessao, crescimento medido = +0.000249/sessao, arrasto = +0.000068/sessao (+0.0172/ano), riqueza em 100000 sessoes = 6.363e10
L = 2x: media aritmetica = +0.000634/sessao, crescimento medido = +0.000267/sessao, arrasto = +0.000367/sessao (+0.0926/ano), riqueza em 100000 sessoes = 3.805e11
L = 3x: media aritmetica = +0.000951/sessao, crescimento medido = +0.000053/sessao, arrasto = +0.000898/sessao (+0.2264/ano), riqueza em 100000 sessoes = 1.955e2
horizonte de 2000 sessões:
  L = 1x: crescimento medido = +0.000196/sessao, riqueza final = 1.480e0
  L = 2x: crescimento medido = +0.000162/sessao, riqueza final = 1.384e0
  L = 3x: crescimento medido = -0.000101/sessao, riqueza final = 8.168e-1
horizonte de 10000 sessões:
  L = 1x: crescimento medido = +0.000352/sessao, riqueza final = 3.385e1
  L = 2x: crescimento medido = +0.000472/sessao, riqueza final = 1.126e2
  L = 3x: crescimento medido = +0.000360/sessao, riqueza final = 3.649e1
o crash de 1987 (-22,6% num dia) tira 67.8% do capital em 3x, 22.6% em 1x; a ruína comeca em L = 4.4
com um crash de -22,6% num dia e 40 dias de -3% (20 mil sessões):
  L = 1x: crescimento medido = +0.000147/sessão, riqueza final = 1.890e1
  L = 2x: crescimento medido = +0.000058/sessão, riqueza final = 3.167e0
  L = 3x: crescimento medido = -0.000274/sessão, riqueza final = 4.188e-3
```

**O que cada linha diz.**

**(1) As duas médias.** A média de conjunto da moeda de Peters é $1{,}0500$ por rodada, ou $+5{,}00\%$.
A média no tempo é $-0{,}052680$, ou $-5{,}268\%$. O produto de uma cara e uma coroa é
$1{,}5 \times 0{,}6 = 0{,}90$. A fração de Kelly é $0{,}14$ da riqueza; com ela, o crescimento no
tempo é $+0{,}0175$ por rodada, positivo pela primeira vez.

**(2) A coorte de 10 000 trajetórias.** Apostando tudo, a riqueza cai para $< 10^{-6}$ do capital em
$99{,}71\%$ das trajetórias, a mediana final é $1{,}322 \times 10^{-23}$ e o $\log_{10}$ da média
final é $-5{,}6$. **A trajetória mais rica sozinha responde por 100% da média da coorte**, ou seja, o 1%
mais rico explica tudo o que a média tem de riqueza. Com **Kelly cheio** $(f = 0{,}70)$ a ruína aparece em
metade das trajetórias ($50{,}89\%$) e a mediana final despenca para $6{,}810 \times 10^{-7}$; com
**meio Kelly** $(f = 0{,}35)$ a ruína é zero e a mediana final é $1{,}854 \times 10^2$; com
**quarto de Kelly** $(f = 0{,}14)$ a ruína é zero e a mediana final é $1{,}506 \times 10^2$. O
contraste entre $f = 0{,}70$ e $f = 0{,}35$ é o ponto do capítulo. **Duas vezes a fração ótima já não
é ótima de nada**, porque a aposta perde na maioria das trajetórias, com a média ainda explodindo.
**(3) O vendedor de seguro.** Em 10 000 históricos de 250 semanas, **36,8%** não veem perda alguma —
exatamente a probabilidade teórica $0{,}996^{250} = 0{,}3671$, e são esses que apostam tudo ($f = 1$).
Quem lê a probabilidade pela frequência crua tem mediana final de $3{,}531 \times 10^2$ e média de
$1{,}255 \times 10^3$; quem usa a regra de Laplace estima $1/252 = 0{,}003968$ e trabalha com
$f = 0{,}484$, com mediana de $8{,}983 \times 10^1$. As duas terminam acima do ponto de partida, e
a diferença não aparece no ganho, e sim na **cauda**. Quem ignorou o risco, num histórico ruim, não
tem de onde voltar.

**(4) A cauda selvagem.** Com Pareto de $\alpha = 1{,}1$, cuja média verdadeira é 11, a média amostral
de um milhão de observações tem mediana 8,39 e fica abaixo de 11 em 95% das amostras e o intervalo de
95% cobre o valor verdadeiro em 26% delas. O estimador de Hill recupera o expoente. Com $k = 1\,000$
dá $\hat\alpha = 1{,}100$, com intervalo aproximado $[1{,}032; 1{,}168]$; com $k = 20\,000$ dá
$1{,}095$ com $[1{,}080; 1{,}111]$. Um modelo gaussiano ajustado a essa cauda subestima a perda em
todas as repetições.

**(5) O arrasto da volatilidade.** Calibrando o modelo pelo H1 do repositório (arrasto de 0,14 ao ano
em $3\times$ e crescimento de 0,051 ao ano em $1\times$), em 100 000 sessões o crescimento medido
por sessão é $+0{,}000249$ em $1\times$, $+0{,}000267$ em $2\times$ e $+0{,}000053$ em $3\times$.
O arrasto medido sobe de $0{,}0172$ ao ano em $1\times$ para $0{,}0926$ em $2\times$ e $0{,}2264$ em
$3\times$; o arrasto **cresce com o quadrado da alavancagem**, que é a assinatura de
$\mu - \tfrac12 \sigma^2 L^2$. O crash de 1987, sozinho, tira **67,8%** do capital em $3\times$ e
22,6% em $1\times$; a ruína começa em $L = 4{,}4$. Com um crash plantado no meio de 20 000 sessões,
o crescimento de $3\times$ fica em $-0{,}000274$ por sessão, abaixo de $1\times$ e de $2\times$. **Um único dia muda a ordem**.

**Declaração de divergência.** A edição anterior registra 35,8% de ruína para quem aposta pelo próprio
histórico e 0% com Laplace. Este instrumento mede 36,8% de "aposta tudo" e **0,00% de ruína** com
qualquer uma das regras. Uma trava declarada explica o motivo. O Kelly é limitado a $f \le 1$ (não se
aposta mais do que se tem), e com $f = 1$ um comerciante que ganha 1% por semana e perde no máximo
metade por perda precisa de sete eventos extremos consecutivos para cair abaixo de $10^{-6}$, e esses
eventos não acontecem em 1 000 semanas com chance de 0,004. O valor antigo fica registrado. Os dois números medem
a mesma coisa, com tetos de alavancagem diferentes. E o arrasto também diverge do roteiro do curso.
A tabela de cinco regras do Capítulo 11 usa um modelo de cauda que lhe é próprio, e os dois conjuntos de alturas
não coincidem; os dois ficam relatados.

Já a tabela de crescimento temporal do repositório (5,1% com $1\times$, 7,2% com $2\times$ e 6,0% com
$3\times$, mais o arrasto de 0,14 ao ano em $3\times$; os mesmos três portfólios aparecem em decimal
no Capítulo 17, que mede 0,0515, 0,0719 e 0,0601) **não é reproduzida** por este instrumento, porque, com
os parâmetros calibrados por aqueles dois números, o modelo dá $3\times$ maior que $2\times$, e não
menor. O motivo provável é a cauda real. O Dow de 1885 a 2023 tem 19/10/1987 e a corrida de 1929-32,
que o modelo gaussiano não tem. A linha do crash, medida, é a evidência de que o efeito existe; o
valor exato do crescimento com $3\times$ depende de qual cauda entra na conta.

**Exercícios.**

1. *(Conta)* Na moeda de Peters, calcule $\mathbb{E}[\ln M]$ à mão e explique por que a mediana cai
   mesmo com a esperança crescendo. Confira com o $-0{,}052680$ medido.
2. *(Conta)* Para uma aposta de igual para igual com chance $p$, mostre que $f^* = 2p - 1$. Depois
   calcule $f^*$ para a moeda de Peters e confira com os 0,14 do laboratório.
3. *(Conceito)* Na moeda de Peters, três jogadores que repartem a riqueza a cada rodada crescem 1,6%
   por rodada, quando cada um sozinho decairia. Por que repartir muda o crescimento sem mudar a
   esperança de nenhum deles?
4. *(Laboratório)* Rode 10 000 investidores na estratégia que vende seguro, com $q$ estimado pela
   frequência crua e por Laplace. Compare as frações de "aposta tudo" e os resultados, e relate o
   que muda quando o teto de alavancagem é removido.
5. *(Conta)* No navio de Bernoulli, o seguro custa 1 800 e cobre uma perda de 34 000 com chance de
   5%. Pela regra do custo/perda, o armador deveria se segurar? O que diz o crescimento temporal?
6. *(Conceito)* Dê um exemplo da sua área em que o número que a planilha mostra é uma média de
   conjunto e a decisão depende de uma média no tempo. Quem está do lado da multidão e quem está do
   lado da trajetória?

**Resumo.**

- Em dinâmica multiplicativa, quem vive uma trajetória deve maximizar $\mathbb{E}[\ln M]$; a média de
  conjunto responde a uma pergunta diferente.
- $+50\%$ contra $-40\%$ dá média de conjunto de $+5\%$ por rodada e deriva de $-5{,}27\%$; o produto
  de uma cara e uma coroa é 0,90.
- Kelly dá a fração que maximiza o crescimento; a incerteza sobre o parâmetro manda apostar menos, e
  a regra de Laplace impede que zero perdas vire risco zero.
- Com cauda pesada, a média amostral engana ou nem existe; com $\alpha = 1{,}1$, a média de um milhão
  de observações fica abaixo do valor verdadeiro em 95% das amostras.
- O arrasto da volatilidade cresce com o quadrado da alavancagem. Medido por sessão, 0,000068 em
  $1\times$ e 0,000898 em $3\times$.
