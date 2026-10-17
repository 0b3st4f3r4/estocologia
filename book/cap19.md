## Capítulo 19. Horizonte H2: sete rodadas para encontrar causas verdadeiras na ciência

**Objetivos.** Ao fim deste capítulo você deve saber como se define um ambiente que *intervém* numa variável, por que uma
causa só merece esse nome se sobrevive à troca de ambiente, por que o portão do H2 falhou cinco vezes antes de
passar, e como se lê um resultado aprovado com ressalva. E também um resultado que passou por pouco e depois encolheu.

---

### Degrau 1 — O mundo real e o paradoxo

**A pergunta que continua valendo em toda parte.** Uma equipe quer saber quais causas continuam valendo quando o mundo muda.
Não "a causa nesta escola", nem "a causa neste ano". A causa aparece na escola A, na escola B e na escola C, no
ano bom e no ano ruim, no hospital de Toronto e no de Rhode Island. Esse é o horizonte H2 da pesquisa, e o seu portão é
único e exigente. Ele pede um conjunto não vazio de causas invariantes, em pelo menos três áreas
diferentes da ciência.

**Primeira tentativa: usar o ano como "ambiente".** Doze áreas entraram de uma vez: educação, clima, crédito,
energia, saúde, direito, logística e outras. Em cada área, o ambiente era algo fácil de achar. Servia o ano, a região, o
aeroporto, o tipo de escola. Em cada 12 áreas, o resultado foi assim:

| O que aconteceu | Em quantas das 12 áreas |
| --- | --- |
| O método rejeitou todos os conjuntos candidatos | 6 |
| O método aceitou tantos conjuntos que a interseção ficou vazia | 4 |
| Deu um conjunto não vazio — no direito, e *contra* a alegação | 1 |
| Foi podado antes, por outro filtro | 1 |

A única área que deu resposta apontou cidadania e emprego como causas invariantes da decisão de libertar um detido,
e deixou a cor da pele de fora. O resultado não diz que a cor não importa. O método só promete devolver um
subconjunto das causas verdadeiras, e tal promessa só vale sob condições que o ano e a região violam. O ano age
**direto** no resultado, e não só nas variáveis de entrada. Um ambiente que muda o mundo inteiro não é um
experimento.

**Da segunda à sétima tentativa: buscar ambientes que sejam intervenções.** A equipe decidiu manter o portão
forte em vez de afrouxá-lo, e foi buscar dados em que o ambiente fosse uma intervenção documentada sobre uma
variável: uma lei que muda um benefício, um sorteio que muda um plano de saúde, uma câmara física onde um
pesquisador gira um botão. Sete rodadas foram necessárias, e cada correção foi escrita e travada com hash **antes**
de rodar. O contador de tentativas, chamado $K$, é o placar honesto dessa busca.

| Rodada | O que mudou | Áreas com conjunto invariante não vazio | Portão |
| --- | --- | --- | --- |
| H2 | 12 domínios de conveniência: ano, região, aeroporto como ambiente | 1 (direito, e contra a alegação) | Falhou |
| H2b | ambientes que são intervenções documentadas ($K = 1$) | 1 (direito) | Falhou |
| H2c | teste de invariância não paramétrico ($K = 2$) | 2 (saúde e educação; a saúde só por saturação) | Falhou |
| H2d | regra que descarta conjuntos sem poder de teste ($K = 3$) | 1 (educação) | Falhou |
| H2e | câmaras físicas com o grafo causal conhecido | 2 (educação e física) | Falhou |
| H2f | deleções de genes em levedura | 3 (educação, física e genética) | Passou pela regra, com ressalva |
| H2g | ensaio clínico de cardiologia da FDA | 4 (as três anteriores e farmacologia) | Passou |

**O túnel de luz.** Numa câmara física construída na Suíça para estudar causalidade (Gamella, Peters e Bühlmann,
2025), um pesquisador gira um botão que controla uma corrente elétrica, três LEDs de cores diferentes acendem, e
sensores de luz medem a resposta. De 1.024 combinações possíveis de causas candidatas, o método aceitou 128, e
a interseção delas (o conjunto que todas têm em comum) foi $\{\text{vermelho},\ \text{verde},\ \text{azul}\}$. São os
três LEDs. Os sensores de luz ficaram de fora, apesar de estarem fortemente correlacionados com
a corrente, porque são **filhos** dos LEDs. Foi a primeira vez no projeto que o método foi conferido contra uma
verdade conhecida, e ele acertou.

**O túnel de vento, nesta bateria, reprovou tudo.** O conjunto dos pais verdadeiros caiu junto. As causas
prováveis foram declaradas antes: os barômetros medem pressão absoluta e o clima muda o resultado entre regimes; a
autocorrelação que sobra depois do desbaste; e um ajuste aditivo onde a física é quadrática, com interação entre os
dois ventiladores. O melhor p-valor entre os conjuntos ficou em $7\times10^{-5}$. Nem com a pressão ambiente como
proxy algum conjunto se sustentou. Um método que falha num caso e acerta no outro, desde que as causas plausíveis
estejam declaradas, ensina mais que um método que sempre acerta.

**O coração.** Num ensaio do FDA em que voluntários receberam quatro drogas e tiveram o eletrocardiograma medido 16
vezes cada um, o método apontou como causas do alongamento do intervalo QT as concentrações das três
drogas que bloqueiam o canal hERG: dofetilida, quinidina e ranolazina. Todos os 35 conjuntos aceitos, de 512,
continham as três, e medidas do próprio eletrocardiograma ficaram de fora. A inclinação do QTcF na dofetilida foi
de $25{,}2$ ms por ng/mL, com intervalo de 90% da posterior de $23{,}0$ a $27{,}5$. E o teste mais duro veio
depois. Na droga que o modelo **nunca viu**, o verapamil, o modelo invariante previu bem (habilidade $+0{,}57$). O
modelo que usava *todas* as variáveis explodiu, com perda da ordem de $10^{15}$, porque o verapamil empurra uma
medida do eletrocardiograma para fora de tudo o que havia no treino.

**A levedura.** Com dados de 2014 sobre deleções de genes em levedura, o método deu um conjunto invariante num
domínio entre dez sorteados. Ele rejeitou 1.022 dos 1.024 conjuntos, aceitou dois, e o melhor p-valor ficou em
$0{,}0072$, logo acima do limiar de $0{,}005$.
A validação por intervenção, que confere gene por gene, não confirmou dois dos três genes do conjunto; só o
primeiro movia o alvo na direção prevista. A saída foi aprovada **com ressalva registrada**. Uma réplica declarada
depois sorteou 40 genes novos de um conjunto de 4.681, com nível de $0{,}05/40 = 0{,}00125$ por gene, e achou um
gene que passa na validação estrita, com taxa nula de $p = 0{,}001$. É o piso de 999 sorteios. A réplica mostra
*um gene num mutante*, e não um efeito medido com resolução. Cada mutante é um ambiente de uma linha só, e o menor
p-valor possível ali é $1/1\,000$.

O laboratório deste capítulo mede o que a **segunda linha** muda, no mesmo desenho (40 mutantes, $lpha/40 =
0{,}00125$ por gene) e com dois casos plantados: um mutante cujo efeito aparece nas **duas** linhas e um artefato
cujo efeito aparece em **uma** linha só. Com uma linha por mutante, a validação aprova **4 dos 40** — o mover
verdadeiro, o artefato de linha e dois que são só ruído, todos encostados no piso de $0{,}001$. O p não separa o
que se procura do que não se procura, porque todos batem no chão ao mesmo tempo. Com a segunda linha, exigindo o
efeito nas duas, passa **1 dos 40**: o mover, com $p = 10^{-6}$, que é o piso novo; o artefato cai fora porque o
efeito dele na segunda linha é $+0{,}028$, contra $+1{,}016$ do mover. Faltava linha, não p-valor menor
(`labs --genetica`).

**A queda da educação.** A educação era a área mais confortável. Eram turmas pequenas sorteadas no experimento STAR do
Tennessee, 11.598 alunos. Quando os alunos deixaram de ser contados como 11.598 observações independentes e
passaram a ser contados como **80 escolas**, os conjuntos aceitos não tiveram nenhuma variável em comum, e o
conjunto invariante virou vazio. Isso não quer dizer que turma pequena não tenha efeito. Quer dizer que, com 80
unidades de verdade, o teste perdeu o poder de apontar um conjunto, que é o único produto que ele entrega.

**Cinco falhas, três causas nomeadas.** O portão falhou cinco vezes seguidas. Três dessas falhas têm nome — usar o
ano ou a região como ambiente invalida a premissa; um erro de código ficou escondido no teste de invariância da
rodada H2b; e a regra de saturação, que só apareceu na rodada H2d, tirou do placar um conjunto que se sustentava
por construção. As outras duas foram a troca de régua da rodada H2c, que perdeu o direito e ganhou a saúde só por
saturação, e a rodada H2e, que chegou a duas áreas e não a três porque o túnel de vento rejeitou tudo.

---

### Degrau 2 — A regra do jogo

**O pacote de domínio, e o teste de plugabilidade.** Cada área entrou por um pacote padronizado: pergunta
substantiva, alegação, glossário de dez termos, fontes, unidade independente, ambientes e cofre, grafo causal,
conjunto de ajuste, elicitação, decisão e escopo. O validador recusa o pacote incompleto **antes** de ler qualquer
dado, e dez dos doze primeiros pacotes foram escritos por agentes que só tinham lido o contrato e dois exemplos.
A máquina se mostrou plugável.

**Definição de ambiente.** Ambiente é a etiqueta que diz de onde veio cada linha da tabela: a escola, o ano, o regime
de intervenção. Escrevemos $E$ para essa etiqueta, $X$ para o conjunto de variáveis candidatas e $Y$ para o
desfecho que se quer explicar.

**A condição de invariância.** Um conjunto $S$ de candidatas é **invariante** se a distribuição de $Y$, uma vez
conhecidas as variáveis de $S$, for igual em todos os ambientes:

$$Y \perp E \mid X_S .$$

Dito de outro modo, depois de olhar as variáveis do conjunto, saber de que ambiente veio a linha não muda mais nada na
previsão. Quem testa essa condição para todos os $2^p$ subconjuntos é a Predição Causal Invariante, ou ICP, e
ele devolve a **interseção** dos subconjuntos aceitos, chamada $\hat{S}$.

**O que o método garante e sob quais condições.** Se o modelo estatístico estiver correto, se as linhas dentro de
cada ambiente forem independentes e se houver poder suficiente, então $\hat{S} \subseteq \text{pa}(Y)$ com
probabilidade de pelo menos $1 - \alpha$. O conjunto devolvido só contém causas verdadeiras. O método **não**
garante que todas as causas apareçam, e não diz nada sobre o tamanho dos efeitos.

**Onde ele falha.** Três situações apareceram na pesquisa, e cada uma virou uma regra.

1. **O ambiente age direto em $Y$.** Se o ano muda o resultado por si, então $Y \perp E \mid X_S$ é falso para todo
   $S$. O método rejeita tudo, e $\hat{S} = \emptyset$. Foi esse o caso nas 6 áreas da primeira rodada.
2. **O ambiente é função determinística das variáveis de $S$.** Nesse caso a condição vale *por construção*.
   Condicionado a $X_S$, o ambiente é constante, e o teste tem poder zero. Um teste sem poder não certifica nada;
   aceitar um conjunto saturado não é evidência. A **regra de saturação** passou a excluir esses conjuntos da
   família. Eles não contam como aceitos nem como rejeitados. A medida é o acerto do vizinho mais próximo,
   deixando a linha de fora (1-NN LOO), ao prever o ambiente pelas colunas de $S$; acerto médio $\ge 0{,}99$
   significa saturado. No experimento de seguro da RAND, a coparticipação é constante dentro de cada regime de
   plano. O conjunto que dependia dela saiu no H2d, como a declaração previa.
3. **As linhas não são independentes.** 16 instantes por voluntário no ensaio cardíaco, até 403 alunos por escola
   no STAR, 225 mil voos em 364 dias. Tratar linhas dependentes como independentes estreita a margem. A correção
   foi o Wald agrupado por voluntário ou por escola e a permutação de blocos inteiros.

**A falha de código que a leitura encontrou.** No H2b, antes de qualquer execução, a leitura do código achou um
erro no teste de invariância. Os graus de liberdade eram contados pelo número de **colunas** da matriz de ajuste em
vez do seu **posto**. Com ambientes que são intervenções, um candidato costuma ser constante dentro do ambiente, e
essas colunas colineares somavam graus de liberdade sem reduzir a soma dos quadrados. O teste ficava permissivo e
aceitava demais. A correção passou a usar o posto e ganhou um teste próprio. Sob a hipótese nula de invariância o
teste rejeita 7% das vezes (a meta era menos de 9%), e sob a hipótese de quebra, 100%. A reexecução mostrou que o
erro **não** tinha mudado nenhum resultado do H2.

**As duas réguas.** A primeira régua era um teste linear gaussiano: F aninhado contra coeficientes por ambiente,
mais Levene dos resíduos. Com amostras grandes ele rejeita qualquer desvio do modelo linear, como massa em zero,
resposta côncava, cauda pesada, e em simulação rejeitava de 72% a 100% das vezes mesmo quando a lei era
invariante. A segunda régua (a partir do H2c) ajusta um único modelo agrupado, padroniza o resíduo pela escala
estimada sem olhar o ambiente, e testa locação (Kruskal–Wallis) e escala (Brown–Forsythe) do resíduo padronizado
entre ambientes, com Bonferroni sobre dois testes. Candidato numérico com mais de 10 valores distintos entra como
termo linear mais spline cúbica restrita com 4 nós nos quantis 5, 35, 65 e 95%; candidato com até 10 valores
distintos entra como fator. A troca de régua mudou o veredito de três domínios: o direito perdeu o conjunto, a
educação ganhou, a saúde ganhou só por saturação.

**A validação por intervenção, e o seu limite.** A conferência externa de um gene é direta. Deleta-se o gene $x$ e
vê-se se o alvo $Y$ se move na direção que o coeficiente prevê. O critério estrito pede duas coisas: que a deleção
tenha funcionado (o próprio gene deletado é o menor valor da sua coluna) e que $Y$ caia fora da faixa das outras
1.740 amostras. Nos 27 genes dos 10 conjuntos com gene da réplica, a deleção funcionou em 26 e **um** gene passou
no critério estrito. É o par do domínio n34, com $Y = $ YLR354C e $x_6 = $ YIL033C, cujo mutante põe $Y$ no mínimo
das 1.741 linhas, na direção do coeficiente positivo $+0{,}053$. A taxa nula foi fixada depois, por permutação do
rótulo do mutante entre as 1.741 linhas, com 999 sorteios e semente 20261015. O observado é 1 par em 27, a média
nula é 0, e $p = 0{,}001$, o piso do teste. A leitura tem duas metades, e elas apontam para lados opostos. Contra o
acaso do rótulo a evidência é forte. Contra ler isso como efeito medido, a evidência é nula. Quase todo o p vem da
metade que é fato dos dados. Deletar um gene tira a expressão dele, então o mínimo da coluna está na linha do
mutante, e permutar o rótulo destrói exatamente essa metade. A metade que sobra, o alvo fora da faixa das outras
1.740 linhas, sozinha dá $p = 0{,}031$, marginal.

**O que a invariância compra.** No verapamil, a droga que o modelo nunca viu, o modelo invariante previu com
habilidade $+0{,}57$ e cobertura conforme $0{,}89$. O modelo ajustado pela chave fez um pouco melhor, $+0{,}65$.
Prever bem dentro do treino e falhar sob uma intervenção nova é o modo de falha que a invariância existe para
evitar.

**O que ficou registrado e entrou no contador $K$.** Além das sete rodadas, o placar guarda o arcabouço v2 com as suas
quatro correções, o critério do G2 (o erro do ajuste da elicitação era pedido abaixo de $10^{-6}$, impossível para
quartis assimétricos, e passou a ser 10% do desvio padrão ajustado, com erro máximo medido de 5%), a mudança de
unidade do experimento de sinalização (a célula, e não a condição, passou a ser a unidade, mudança que levou aquele
domínio até o teste de invariância e derrubou o efeito de desenho de 67 para 1,13) e o efeito fixo de estado do
fluxo de cintos de segurança, que cortou o efeito de desenho de 6,1 para 1,85.

**A multiplicidade, contada.** O contador honesto do portão do H2 é de **38 testes distintos**, entre 32 e 72 conforme
o que se conta. O inventário do caminho conta ainda **onze aberturas de cofre**, isto é, onze vezes em que um
resultado foi visto antes de uma decisão de desenho. Com Bonferroni sobre 38, o nível por teste cai para $\alpha = 0{,}05/38 = 0{,}0013$, ao custo de
9,6 bits de parcimônia, e o portão ainda passa em quatro áreas. Os conjuntos encolhem: no STAR,
$\{\text{anos\_pequena\_antes}, \text{turma\_pequena}\}$ vira $\{\text{turma\_pequena}\}$; na levedura,
$\{\text{plate}, x_1, x_5, x_9\}$ vira $\{\text{plate}, x_5\}$, e $x_1$ sai, o único gene que a validação por
intervenção confirmava.

**Duas ressalvas que pesam mais que a correção.** A primeira: dividir $\alpha$ não é monótono no ICP. Com um nível
menor, domínios que rejeitavam tudo passam a aceitar algum conjunto, e aparecem conjuntos invariantes em modelos
quase rejeitados. A segunda: a aprovação depende da versão do arcabouço. Com o arcabouço v1 reconstruído, o STAR é
podado antes do teste de invariância, pois a chave é constante dentro do ambiente, e o H2g cai para três áreas, uma
delas a genética frágil. E se valesse só a primeira versão declarada de cada escolha, o H2 não estaria aprovado, e a
rodada 1 dá uma área só.

**O resultado final, depois da auditoria de fundo.** A farmacologia sobreviveu intacta às duas correções de
agrupamento. O conjunto continua sendo $\{c_{\text{dof}}, c_{\text{qui}}, c_{\text{ran}}\}$, e encolhe a
evidência contra o conjunto vazio, de $6{,}3\times10^{-104}$ para $2{,}2\times10^{-15}$ no teste agrupado e
$0{,}002$ na permutação de blocos, de cerca de 340 bits para 49 e 9. A educação não sobreviveu. Com as 80 escolas
como unidades, são 266 conjuntos aceitos de 512 no teste agrupado e 512 na permutação, sem nenhuma variável em
comum, e o conjunto publicado $\{\text{anos\_pequena\_antes}, \text{turma\_pequena}\}$ tem $p = 0{,}76$. Mesmo
antes da correção de agrupamento, a educação já era a área frágil, com o maior p entre os conjuntos não saturados
em $0{,}075$, perto do limiar, e $\{\text{turma\_pequena}\}$ sozinho era rejeitado ($p = 0{,}010$). A
genética se replicou. A contagem de áreas fecha assim. A educação sai, a física fica como estava, a farmacologia
fica e a genética se replica. O portão fica de pé com três áreas, agora com a terceira sustentada
por um domínio novo, e não pela área que já era frágil.

**O efeito transporta melhor que o conjunto.** Ainda com o ICP rejeitado, o modelo ajustado previu o ambiente
nunca visto com habilidade positiva em **9 domínios**, de $0{,}04$ a $0{,}88$. A alegação forte ("vale em qualquer
escola, aeroporto ou região") cai; a direção e o tamanho do efeito sobrevivem. Para decidir, é essa versão fraca
que importa.

**A arquitetura também errou.** A revisão dos agentes de domínio achou **quatro** falhas no arcabouço, corrigidas
de uma vez na versão 2: o nulo do filtro F3 permutava num nível diferente daquele em que a chave varia, e passou a
permutar entre unidades, dentro da unidade quando o sorteio é declarado; a verossimilhança tratava as linhas como
independentes e saía estreita demais, e passou a ser temperada por $1/\text{efeito de desenho}$; a predição
conforme era calibrada num quinto sorteado do treino, e passou a deixar um ambiente de fora; e os rótulos do ICP
não separavam "rejeitado" de "vazio aceito". A versão 2 mudou o status de quatro domínios. As quatro correções
vieram depois de ver resultados, e contam no $K$.

Na primeira rodada, com ambientes de conveniência, o efeito de desenho chegou a $11{,}5$ nos voos (225 mil voos
valem 364 dias), $22{,}8$ nos ciclones e $9{,}8$ no estudo de desenvolvimento; e a literatura errou para os dois
lados: a priori dos voos ficou abaixo do dado (razão de chances de $1{,}08$ por hora contra $1{,}14$), a dos
ciclones ficou acima (15 nós por raiz de hectopascal contra $11{,}8$), e a dos currículos previa "um terço a menos"
onde o dado deu $2{,}3$ pontos percentuais. O filtro F2 barrou a primeira especificação dos voos por incluir a
temperatura, descendente da hora no próprio grafo causal ($K = 2$), e o F3 podou o imposto sobre a cerveja, que com
efeito fixo de estado não se distingue do nada ($p = 0{,}41$).

---

### Degrau 3 — Mão na massa

O laboratório deste capítulo é um ICP mínimo, com as duas réguas do livro em versão reduzida: mínimos quadrados
por equações normais, resíduo padronizado pela escala estimada sem olhar o ambiente, Kruskal–Wallis da locação e
Brown–Forsythe da escala, p-valor por 999 permutações dos rótulos de ambiente, Bonferroni sobre os dois testes, e a
regra de saturação medida pelo 1-NN deixando a linha de fora. O código está em `crates/stochology-book/src/cap19.rs` —
autocontido, com função principal própria, sem dependências, com o acaso embutido em dez linhas de SplitMix64 (rodado pelo
binário `labs` com `--cap` dezenove, ou direto no módulo):

```bash
cargo run --release
```

São três mundos simulados, todos com três ambientes de 600 linhas cada, semente 2019, e quatro candidatas: $x_1$
(horas de estudo, causa verdadeira), $x_2$ (horas de sono, causa verdadeira), $x_3$ (ruído puro) e $x_4$
(transporte até a escola, uma variável que o ambiente desloca e que **não** age na nota). O desfecho é
$Y = x_1 + 0{,}8\,x_2 + \text{ruído}$.

| Mundo | O que o ambiente faz | O que se espera |
| --- | --- | --- |
| A | desloca a média de $x_1$ e $x_2$ entre as escolas (intervenção em X) | $\hat{S} = \{x_1, x_2\}$ |
| B | soma um deslocamento próprio ao desfecho (o ano age direto em Y) | $\hat{S} = \emptyset$ |
| C | o mundo A mais $x_5$, que é o próprio ambiente (uma turma por escola) | $\hat{S} = \{x_1, x_2\}$, e nada mais |

```text
== Capítulo 19 :: o portão do conjunto invariante ==
semente 2019 | 3 ambientes x 600 linhas | 999 permutacoes por teste | alfa = 0.05

mundo A: o ambiente intervem em X (3 escolas, 4 candidatos)
  conjunto                           p_local p_escala   p_conj  veredito      1-NN
  {}                                  0.0010   0.8500   0.0020  rejeitado    0.333
  {x1 estudo}                         0.0010   0.4260   0.0020  rejeitado    0.477
  {x2 sono}                           0.0010   0.0610   0.0020  rejeitado    0.702
  {x3 ruido}                          0.0010   0.8350   0.0020  rejeitado    0.343
  {x4 transporte}                     0.0010   0.9010   0.0020  rejeitado    0.725
  {x1 estudo, x2 sono}                0.9920   0.6480   1.0000  aceito       0.743
  {x1 estudo, x2 sono, x4 transporte}  0.9910   0.6150   1.0000  aceito       0.868
  aceitos na familia: 4 de 16 (com os saturados seriam 4) | saturados testados: 0 | aceitos saturados: 0
  S_hat com a regra de saturacao = {x1 estudo, x2 sono}  | sem a regra = {x1 estudo, x2 sono}  | portao: PASSA (S_hat nao vazio)

mundo B: o ambiente age direto em Y (o ano, 3 candidatos)
  conjunto                           p_local p_escala   p_conj  veredito      1-NN
  {}                                  0.0010   0.8460   0.0020  rejeitado    0.333
  {x1 estudo}                         0.0010   0.6370   0.0020  rejeitado    0.477
  {x2 sono}                           0.0010   0.3730   0.0020  rejeitado    0.702
  {x3 ruido}                          0.0010   0.8280   0.0020  rejeitado    0.343
  {x1 estudo, x2 sono}                0.0010   0.0350   0.0020  rejeitado    0.743
  aceitos na familia: 0 de 8 (com os saturados seriam 0) | saturados testados: 0 | aceitos saturados: 0
  S_hat com a regra de saturacao = {}  | sem a regra = {}  | portao: FALHA (S_hat vazio)

mundo C: mundo A mais x5, a coluna que e o proprio ambiente (5 candidatos)
  conjunto                           p_local p_escala   p_conj  veredito      1-NN
  {}                                  0.0010   0.8500   0.0020  rejeitado    0.333
  {x1 estudo}                         0.0010   0.4260   0.0020  rejeitado    0.477
  {x2 sono}                           0.0010   0.0610   0.0020  rejeitado    0.702
  {x3 ruido}                          0.0010   0.8350   0.0020  rejeitado    0.343
  {x4 transporte}                     0.0010   0.9010   0.0020  rejeitado    0.725
  {x1 estudo, x2 sono}                0.9920   0.6480   1.0000  aceito       0.743
  {x1 estudo, x2 sono, x4 transporte}  0.9910   0.6150   1.0000  aceito       0.868
  {x5 turma}                          0.9640   0.9990   1.0000  aceito       1.000
  {x1 estudo, x2 sono, x5 turma}      0.9410   1.0000   1.0000  aceito       1.000
  aceitos na familia: 4 de 32 (com os saturados seriam 20) | saturados testados: 16 | aceitos saturados: 16
  S_hat com a regra de saturacao = {x1 estudo, x2 sono}  | sem a regra = {}  | portao: PASSA (S_hat nao vazio)
```

Leia linha por linha. **No mundo A**, o conjunto vazio e todos os conjuntos de uma variável só são rejeitados. O
p-valor de locação bate no piso da permutação, $0{,}0010 = 1/1\,000$, porque o deslocamento que a escola faz em
$x_1$ e $x_2$ deixa marca na média do resíduo. Só os conjuntos que contêm **as duas** causas verdadeiras são
aceitos, 4 dos 16, e a interseção deles é $\{x_1, x_2\}$. A coluna 1-NN mostra por que nenhum deles é saturado.
O acerto do vizinho mais próximo ao adivinhar a escola pelas variáveis do conjunto fica entre $0{,}74$ e $0{,}87$,
muito abaixo do limiar $0{,}99$.

**No mundo B**, o ambiente soma um deslocamento direto ao desfecho, e **nenhum** dos 8 conjuntos passa. A
invariância não existe para ser achada, e o portão falha honestamente. É a simulação da primeira rodada do H2.

**No mundo C**, o acréscimo de $x_5$, uma coluna constante dentro de cada escola e diferente entre elas, como a
coparticipação de um plano de saúde, muda tudo. Dezesseis dos 32 conjuntos são saturados, e dezesseis desses são
aceitos. Aceitar não é evidência, porque o teste não tem poder nenhum ali, e o acerto 1-NN de $x_5$ é $1{,}000$.
Sem a regra de saturação, o conjunto $\{x_5\}$ entra na família, a interseção com $\{x_1, x_2\}$ fica vazia e o
método devolve $\hat{S} = \emptyset$, o oposto da verdade, com cara de rigor. Com a regra, a família volta a ter 4
conjuntos aceitos e o $\hat{S}$ é $\{x_1, x_2\}$. Foi isso que aconteceu no experimento de seguro da RAND no
H2d, onde a saúde saiu do placar por saturação.

**A segunda linha, medida.** O mesmo laboratório traz o desenho da validação por intervenção da levedura: os dois
casos plantados — o mutante que se move nas duas linhas e o artefato que se move numa só — e os dois arranjos
rodados sobre o mesmo mundo. O instrumento está em `crates/stochology-h2/src/genetica.rs`, a leitura em
`crates/stochology-book/src/genetica.rs`:

```bash
cargo run --release --bin labs -- --genetica
```

```text
== Capítulo 19 :: a resolução fina da validação por intervenção ==
desenho: 40 mutantes | 60 individuos por linha | 999 sorteios por teste | alfa = 0.05 | Bonferroni alfa/40 = 0.001250
plantados pelo instrumento: mover verdadeiro = mutante 7 | artefato de linha = mutante 23

-- a validacao de hoje: cada mutante e um ambiente de uma linha so --
  40 mutantes x 60 individuos | piso do desenho = 0.001000 | limiar = 0.001250
  mutante  7 | efeito  +0.940 | p  0.001000 | passa no piso <- mover verdadeiro (efeito nas duas linhas)
  mutante 14 | efeito  -0.450 | p  0.001000 | passa no piso
  mutante 23 | efeito  +0.813 | p  0.001000 | passa no piso <- artefato de linha (efeito numa linha so)
  mutante 29 | efeito  +0.496 | p  0.001000 | passa no piso
  passam: 4 de 40

-- o mesmo mundo, com a segunda linha: o efeito tem de estar nas duas --
  40 mutantes x 2 linhas x 60 individuos | piso do desenho = 0.000001 | limiar = 0.001250
  mutante  7 | efeito l1  +0.456 | efeito l2  +1.016 | faixa  0.560 | p  0.000001 | passa <- mover verdadeiro (efeito nas duas linhas)
  mutante 23 | efeito l1  +0.607 | efeito l2  +0.028 | faixa  0.579 | p  0.000831 | rejeitado <- artefato de linha (efeito numa linha so)
  passam: 1 de 40
  efeito na segunda linha: artefato  +0.028 (nao se reproduz) | mover  +1.016 (se reproduz) | na validacao de uma linha a estimativa e um ponto e a faixa e zero

leitura: com uma linha o piso de 0.001000 e o limiar de 0.001250 deixam passar 4 dos 40 mutantes — o mover verdadeiro, o artefato de linha e dois mutantes que sao so ruido. Todos encostam no chao do desenho, e o p nao distingue nenhum dos tres do que se procura. Com a segunda linha o artefato sai da lista, porque o efeito dele na segunda linha e zero, e o piso cai para 0.000001000: passa 1 dos 40 mutantes. O p nunca foi o problema: o problema era a linha ser uma so.
```

Leia os dois blocos juntos. No arranjo de **uma linha**, a lista dos que passam tem quatro nomes, e só dois deles
são o que se procura: o mover verdadeiro e o artefato de linha chegam ao piso de $0{,}001$ pela mesma porta, e dois
mutantes sem efeito nenhum entram com eles, porque no piso o teste não tem resolução para separar. É a medida exata
do que a réplica já dizia em prosa: *um gene num mutante*. No arranjo de **duas linhas**, o efeito tem de aparecer
nas duas, e a lista encolhe para um nome. O artefato não cai por p-valor grande — o efeito dele na primeira linha é
$+0{,}607$, maior que o do mover na mesma linha —, cai porque na segunda linha o efeito é $+0{,}028$, que é nada.
O piso do desenho, que era o chão de $1/1\,000$, passa a ser o produto dos dois, $10^{-6}$, e a estimativa de
cada efeito deixa de ser um ponto. **O p nunca foi o problema; a linha é que era uma só.**

**Declaração de divergência.** Os números acima são de dados **simulados** por este laboratório, e não do
repositório da pesquisa; as três simulações mostram um mecanismo único, em escala de brinquedo. Os valores do
repositório citados neste capítulo (128 de 1.024 conjuntos aceitos no túnel de luz com $p = 0{,}29$ para
$\{$vermelho, verde, azul$\}$; 35 de 512 no ensaio do FDA; 57 conjuntos aceitos no STAR no H2d; 2 de 1.024 na
levedura com melhor $p = 0{,}0072$) não são reproduzidos aqui, e não podem ser: eles exigem os dados reais e o
arcabouço em Python dos horizontes, cujos comandos estão no Apêndice C. O laboratório mede o que pode com
dados próprios: que o método acha as duas causas no mundo A, que devolve vazio no mundo B e que a regra de
saturação é a diferença entre $\emptyset$ e a verdade no mundo C. O desenho da validação por intervenção da
levedura é a exceção: a resolução da réplica é medida de novo, em Rust, por `labs --genetica`, com o mover
verdadeiro e o artefato de linha plantados pelo instrumento — o que o instrumento mede ali é o **desenho**, e não
os 40 genes reais, que continuam sem instrumento.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 19 | sem perda medida neste capitulo | agrupar por escola derruba a área; por aluno, não: a unidade independente no vocabulário do eixo
```

**Exercícios.**

1. *(Conceito)* Explique, com o mundo B do laboratório, por que usar o ano como ambiente faz o conjunto invariante
   sair vazio. Que premissa do ICP o ano viola?
2. *(Conceito)* No mundo C, sem a regra de saturação o método devolve o conjunto vazio. Escreva em três frases por
   que aceitar um conjunto saturado não é evidência de invariância.
3. *(Conta)* No laboratório, o piso da permutação com 999 sorteios é $1/1\,000 = 0{,}001$. Qual é o menor p-valor
   possível com 9.999 sorteios, e por que isso importa para a validação da levedura, onde cada mutante é um
   ambiente de uma linha? Confira a conta no arranjo de duas linhas: se cada linha tem o piso $1/1\,000$ e o p
   combinado é o produto, onde fica o piso do desenho com duas linhas?
4. *(Laboratório)* Troque, no laboratório, o deslocamento de $Y$ do mundo B de $[0; 6; -4]$ para $[0; 1; -0{,}5]$
   e rode de novo. O portão continua falhando? O que isso diz sobre o poder do teste?
5. *(Conceito)* A correção de multiplicidade encolhe os conjuntos e ainda deixa quatro áreas. Por que ela também
   pode criar conjuntos invariantes em domínios que rejeitavam tudo, e por que isso é uma ressalva?
6. *(Conta)* Com 38 testes corrigidos por Bonferroni, o nível por teste é $0{,}0013$ e o custo de parcimônia é de
   9,6 bits. A evidência da levedura vale 10,9 bits. Por quantos bits ela passa? E se o custo subisse para 12 bits,
   quantas áreas sobrariam — e o portão continuaria de pé?

**Resumo.**

- O portão do H2 pedia um conjunto invariante não vazio em três áreas distintas; a primeira rodada deu uma, e
  contra a alegação, porque o ano e a região agem direto no resultado.
- O portão só passou na sexta rodada, com ressalva registrada na genética, e as cinco falhas ficaram no contador
  $K$: ambientes de conveniência, um erro de código no teste, a troca de régua e a regra de saturação.
- Duas rodadas conferiram o método contra verdade conhecida. O túnel de luz recuperou os três LEDs e o ensaio do
  FDA recuperou as três drogas que bloqueiam o hERG, com o verapamil previsto a $+0{,}57$.
- Depois da auditoria de fundo, a educação sai, a genética se replica e o portão fica de pé com três áreas:
  física, farmacologia e genética. O efeito transporta melhor que o conjunto. Os 9 domínios previram o ambiente novo
  com habilidade de $0{,}04$ a $0{,}88$.
