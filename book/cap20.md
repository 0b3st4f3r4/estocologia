## Capítulo 20. Horizonte H3: o relógio ao vivo, o canário de vazamento e a polícia no cofre

**Objetivos.** Ao fim deste capítulo você deve saber o que é um dado que ainda não existe, por que um teste que
nunca encontrou nada precisa de um canário para valer alguma coisa, como se mede a lei de Goodhart com um
instrumento sorteado, e por que corrigir a calibração de um modelo não cria a resolução que ele não tem.

---

### Degrau 1 — O mundo real e o paradoxo

**Seis relógios ligados ao mesmo tempo.** O horizonte H3 rodou seis fluxos de dados reais como se estivessem
acontecendo agora: a demanda de energia elétrica da região de Victoria, de meia em meia hora; as chamadas de um
banco, de cinco em cinco minutos; o retorno diário da bolsa de Nova York; os atrasos de aviões em Nova York; a
chance de um furacão ganhar força; e a decisão de liberar um detento em Toronto. Em cada fluxo, cada fato é um
**evento com carimbo**, isto é, a hora em que ele se torna conhecível. Um voo que parte às 10h e chega atrasado só
"existe" para o sistema quando o atraso é publicado; uma previsão emitida às 8h não pode usar esse dado, porque
ele ainda não tinha acontecido.

O armazém recusa quatro coisas, e recusar é o trabalho dele: ler o futuro, abrir um cofre antes do pré-registro
escrito, abrir o mesmo cofre duas vezes e voltar o relógio. Em todo o replay foram 453.973 previsões, e nenhuma
delas usou estado do futuro; **zero violações**, com a contagem feita previsão por previsão.

**O paradoxo do canário.** "Zero violações" parece ótimo, e é quase nada. Só vale se o teste *podia* ter
encontrado uma violação. Por isso o sistema plantou de propósito um **canário**. É uma coluna idêntica ao rótulo,
publicada nesse instante, quando o rótulo se torna conhecível. Com o relógio honesto, essa coluna não aparece em
nenhuma emissão, e um previsor que a usa não ganha nada, com **ganho 0,000**. Desligado o relógio, o controle
positivo, a coluna rende ganhos de $+0{,}86$ a $+0{,}92$ no fluxo de voos, $+0{,}85$ no ano novo de voos,
$+0{,}55$ no de prisões de 2020 e $+0{,}97$ no de Rhode Island. É o teste severo de Mayo aplicado ao próprio
teste. O alarme toca quando há vazamento, e fica quieto quando não há.

**O paradoxo do bom previsor que erra o mundo.** Numa central de chamadas, quem não é atendido liga de novo. A
previsão passa a medir o efeito dela mesma; se o modelo prevê fila e o gestor escala mais gente, as chamadas
observadas mudam. Um previsor ingênuo, treinado naquilo que se observa, acertou o observado com **1%** de erro e
**superestimou a demanda verdadeira em 29%**. A métrica do alvo aprovava um modelo errado sobre o mundo.
Esse é o laço de Goodhart em estado puro, e a única saída foi plantar um experimento dentro do laço. Em 10% dos
intervalos, um sorteio mudava a escala para 0,7 vezes ou 1,3 vezes. Com os choques sorteados como instrumento, o
efeito estimado caiu para 0,56 e a previsão corrigida errou a demanda verdadeira em **1,7%**, dentro do limite de
3% declarado antes.

**O paradoxo do cofre.** Calibração é a promessa de que "quando o modelo diz 30%, acontece em cerca de 30% dos
casos". Ela foi testada em cofres fora do tempo, abertos uma única vez, e o erro esperado de calibração é a
distância média entre o previsto e o observado, medida faixa por faixa. A tabela abaixo conta a história das três
tentativas.

| Fluxo | O que prevê | Cofre | Calibração no cofre | Portão |
| --- | --- | --- | --- | --- |
| Energia (Victoria) | demanda de cada meia hora de amanhã | 2014, 17.376 | cobertura de 90% = 0,887 (reserva oficial em 48% do tempo) | Passa |
| Central (banco, 2003) | chamadas por 5 minutos do próximo dia útil | set–out, 6.084 | cobertura 0,900 | Passa |
| Bolsa (Dow Jones) | valor em risco de 99% do próximo pregão | 2000–2023, 6.010 | violações 0,98%; Kupiec p = 0,89; aglomeradas (Christoffersen p = 0,02) | Passa |
| Voos (Nova York, 2013) | chance de atraso acima de 15 minutos, 2 h antes | out–dez, 84.301 | erro na média de $+2{,}7$ pontos; ECE 0,027 | Falha |
| Ciclones | chance de ganhar 30 nós em 24 horas | 2015–2022, 5.766 | erro na média 0,001; ECE 0,012 | Passa |
| Prisões (Toronto, 1997–2002) | chance de liberação | 2001–2002, 1.488 | erro na média 0,016; ECE 0,036 | Falha |

**A lição de desenho da primeira falha.** Nos voos, o último trimestre de 2013 teve mais atrasos que o modelo de
60 dias esperava, e o erro na média ficou 2,7 pontos acima do previsto, para uma tolerância de 2. Nas prisões, o
erro de calibração foi 0,036, para uma tolerância de 0,03, com só 1.488 casos. E a causa ficou clara. **O disjuntor
comparava o modelo com o reserva, e não com a calibração**. O modelo continuava vencendo o reserva, e por isso o
disjuntor quase não abriu, só que estava descalibrado na média. Ser melhor que o rival e estar certo são coisas
diferentes.

**Segunda tentativa: consertar o nível.** A correção foi uma recalibração contínua do intercepto, declarada
antes e testada num cofre novo. O cofre foi o trimestre de outubro a dezembro repetido, agora em 2023, com
104.489 previsões. Os voos passaram, com erro na média de 0,0015 e ECE de 0,008. Mas o trimestre de 2023 não
repetiu o desvio de 2013. O modelo cru, sem correção nenhuma, também teria passado, com ECE 0,014. A correção
melhorou a calibração e **não foi posta à prova**.

Nas prisões, os dados novos de Toronto, de 2020 e 2021, com 65.276 prisões e cofre no segundo semestre de 2021
sobre 16.933 casos, mudaram a pergunta. Ela passou a ser registrar na delegacia em vez de liberar. O modelo
acertou a média (0,0002) e separou bem os casos, mas era **pouco confiante**. Nas faixas de baixo previa 0,12
onde a taxa observada era 0,04, e nas de cima previa 0,82 a 0,90 onde a taxa era 0,91 a 0,94. É um erro de
**inclinação**, e mexer no intercepto não alcança isso.

**Terceira tentativa: Rhode Island.** O programa declarou antes uma especificação nova, uma recalibração de Platt
que ajusta intercepto *e* inclinação, e a testou num cofre novo, com 24.466 paradas da polícia estadual de Rhode
Island, entre 2013 e 2015, e taxa de prisão de 2,26%. A pergunta mudou de "liberação depois da prisão" para
"prisão numa parada de trânsito", e essa troca foi confirmada por um humano.

| Cofre 2013–2015 | Erro na média | ECE | Habilidade contra a taxa-base |
| --- | --- | --- | --- |
| Previsão oficial (com Platt) | 0,0020 | **0,0048** | $+0{,}019$ |
| Modelo cru, em sombra | 0,013 | 0,013 | $+0{,}004$ |

Passou, com ECE de $0{,}21\,\bar{y}$ contra o limite de $0{,}3\,\bar{y}$. O cofre tinha esse critério relativo à
taxa, e o modelo cru, que superestimava a taxa em **58%**, teria falhado. É isso que mostra o valor de Platt. A
recalibração terminou com inclinação 1,07 e intercepto $-0{,}30$, e o disjuntor pôs o reserva como oficial em 12%
do cofre. Ficam duas ressalvas, e a primeira é a mais importante. **A habilidade é pequena** ($+0{,}019$). Prisão
numa parada de trânsito é um evento raro (2 em cada 100 paradas) e pouco previsível pelas variáveis disponíveis.
A calibração é boa, e a **resolução** não se conserta por recalibração. É a decomposição de Murphy em dados reais.
A recalibração consertou a calibração, e não podia criar a resolução ausente nos dados.

**Quem paga pelo erro.** A auditoria de justiça foi informativa nos três fluxos de prisões, e o padrão não foi
igual nos três. Em Toronto, de 1997 a 2002, a taxa de falso positivo dos negros foi cerca do **dobro** da dos
brancos em todos os anos (0,16 a 0,23 contra 0,06 a 0,11), num modelo que **não usa a cor**; ela entra pelos
correlatos. Em Toronto, em 2020 e 2021, as taxas-base ficaram próximas e a calibração, os falsos positivos e os
falsos negativos ficaram a menos de 0,05 entre os grupos. Em Rhode Island, a taxa de prisão dos motoristas negros
foi o dobro da dos brancos (3,0–3,8% contra 1,4–1,7%), e a taxa de falso positivo foi quase o dobro em 2013 e 2014
(0,019 contra 0,011; 0,025 contra 0,013), de novo num modelo cego à cor. Em Toronto, em 2002, as taxas de não
liberação eram próximas (21% contra 19%) e o falso positivo dos negros continuou cerca do dobro. Não usar a cor no
modelo não dá paridade, porque a cor entra pelos correlatos.

---

### Degrau 2 — A regra do jogo

**O relógio.** Cada fato carrega o seu instante de disponibilidade, escrito $t_{\mathrm{disp}}$. O armazém só
entrega uma linha se $t_{\mathrm{disp}} \le$ instante do pedido, e cada agente lê apenas a janela entre o último
tique e agora. Antes de acessar o cofre, a especificação é congelada num hash de pré-registro, e o cofre abre uma
vez por fluxo e não abre de novo. Os grupos de linhas do arquivo em disco têm o tamanho de um tique, e é por isso
que cada tabela é varrida no máximo 3 vezes no replay inteiro, medido em 2,03 vezes no H3, 1,8 no H3b, 2,35 no H3c
e 1,53 no H3d.

**As regras do portão de calibração.** Para um previsor binário, no cofre:

$$|\bar{p} - \bar{y}| \le 0{,}02, \qquad \mathrm{ECE} \le 0{,}03, \qquad \text{habilidade de Brier} > 0 .$$

A habilidade de Brier compara o escore do modelo com o do climatologista, que sempre repete a taxa do passado:
$1 - \text{Brier}_{\text{modelo}}/\text{Brier}_{\text{climatologia}}$. Zero significa que o modelo nada sabe
além da taxa-base; a referência é uma climatologia **móvel**, isto é, a taxa-base do passado disponível no instante da
emissão. O erro esperado de calibração é a distância média entre o previsto e o observado, ponderada pelo tamanho
de cada faixa:

$$\mathrm{ECE} = \sum_{k=1}^{10} \frac{n_k}{n}\, \left| \bar{p}_k - \bar{y}_k \right| ,$$

com 10 faixas de massa igual. Para os fluxos de quantis, o critério é a cobertura do intervalo de 90% ficar entre
0,86 e 0,94; para o valor em risco da bolsa, a frequência de violações ficar entre 0,5% e 1,5% e o teste de Kupiec
dar $p > 0{,}05$. Abster-se não tira a previsão do placar, porque quando o modelo é recusado pontua-se o reserva no
lugar dele. Sem essa regra, um modelo poderia melhorar o escore simplesmente desistindo nos dias difíceis.

**A decomposição de Murphy, em uma frase.** O erro quadrático médio se separa em confiabilidade (o quanto as
previsões erraram a frequência), resolução (o quanto elas souberam separar os casos) e incerteza do desfecho
(quanto o evento é raro e imprevisível). A recalibração de Platt só move a **confiabilidade**. A resolução mora
nos dados e nas variáveis, e nenhuma correção de saída a inventa. Por isso o veredito de Rhode Island é
"calibração boa, resolução baixa", e por isso ele é honesto. $\mathrm{REL} - \mathrm{RES}$ fica positivo, mas a
habilidade final é $+0{,}019$, longe do $+0{,}05$ que o programa havia pedido.

**O canário como teste severo.** Mayo pede que um teste seja capaz de errar. O canário é a versão operacional
desse pedido, uma coluna que *é* o rótulo, mas só se torna disponível quando o rótulo se torna disponível. Com o
relógio honesto ela não entra em nenhuma emissão, e o ganho é 0,000; com o relógio desligado, o ganho é de 0,55 a
0,97. O valor do teste de vazamento está no controle positivo dele.

**O laço performativo, declarado e depois medido.** Nenhum dos fluxos reais registra as ações do sistema,
então o laço foi simulado sobre as chegadas reais da central. A escala sai da previsão, quem desiste redisca com
probabilidade $r$, e as rediscagens entram na contagem observada. Com $r = 0{,}6$ declarado, o sensor detecta a
dependência ($t = -6{,}4$, $p = 1{,}5\times10^{-10}$); com $r = 0$, o sensor não detecta ($p = 0{,}70$). O parâmetro
declarado era um chute. Medido depois numa central real de 1999, o mesmo laço vale
$0{,}145$ [0,132; 0,157], **quatro vezes menos** que o declarado. A correção foi, portanto, testada num mundo
mais hostil que o mundo observado, e essa assimetria enfraquece a correção como prova. E o estimador do laço teve
**quatro versões** antes de passar, todas registradas: viés de $+4{,}7\%$, $-5{,}9\%$, $-8{,}4\%$ e $+1{,}7\%$,
para um limite de 3%. Só a quarta passa. A primeira falha por desenho, e não por conta, porque escalar pela demanda
corrigida alimenta a espiral de desistências. É ajuste de estimador depois de ver resultado, o ponto mais fraco
desse portão, dito sem enfeite.

**A banca contra o placar.** Três ataques foram tentados e neutralizados. O previsor que se abstém na zona
incerta recebe o escore do reserva e fica atrás do previsor honesto nos três fluxos binários; o previsor constante
na taxa-base recebe peso 0 no diário; e a segunda abertura do cofre é recusada. O índice de Goodhart, habilidade no
desenvolvimento menos habilidade no cofre, foi relatado por fluxo, e é pequeno: $+0{,}14$ na energia, $+0{,}02$ nos
voos, e negativo nos outros, onde o cofre ficou acima do desenvolvimento.

**O que a malha produziu, em números de operação.** Alarmes de deriva no cofre: 1 na energia, 2 na central, 1 nos
voos, 1 nas prisões. Aberturas do disjuntor: 10 no desenvolvimento e 6 no cofre na energia; 4 e 1 nos voos. O
limiar de custo por perda mudou a decisão em relação ao reserva em 21% dos instantes nos voos, 18% na bolsa e 13%
na energia. O k-anonimato da base de prisões é 1. Nessa base, 11% das linhas são únicas nos quase-identificadores, e 2%
continuam únicas com a idade em faixas de 5 anos; nenhuma coorte liberada passou de orçamento $\varepsilon = 2$ por
pessoa, e o disjuntor de privacidade retirou a coorte de 1997 da publicação de 2001. O hipergrafo do horizonte tem
454.178 nós, 453.973 previsões e 205 de topologia, 51 hiperarestas e 7 pontes, isto é, 45% da meta de $10^6$ que o
plano pedia.

**A tolerância de calibração, medida depois.** O valor 0,03 da tolerância era uma escolha. A auditoria de fundo
perguntou qual é o ruído de verdade. Para cada cofre, 2.000 simulações de rótulos com as previsões e o $n$ do
cofre, e o quantil 95% do erro esperado de calibração sob calibração perfeita. O ruído cresce quando o cofre encolhe.

| Cofre | $n$ | ECE observado | Quantil 95% do ruído | Veredito com tolerância que depende de $n$ |
| --- | --- | --- | --- | --- |
| Voos 2013 | 84.301 | 0,0267 | 0,0046 | Falha |
| Ciclones | 5.766 | 0,0123 | 0,0087 | Falha |
| Prisões Toronto 1997–2002 | 1.488 | 0,0363 | 0,0320 | Falha ($p = 0{,}008$) |
| Prisões Toronto 2020–2021 | 16.933 | 0,0668 | 0,0115 | Falha |
| Voos 2023 | 104.489 | 0,0079 | 0,0035 | Falha |
| Rhode Island | 24.466 | 0,0048 | 0,0034 | Falha |

Nenhum cofre binário passa por essa regra, **nem os três aprovados**. A lição é que 0,03 não é "o ruído". Nos
cofres grandes o ruído é de 0,002 a 0,006, e 0,03 é uma tolerância prática que deixa passar descalibração
detectável. Pela regra secundária, que exige $\max(0{,}03;\ \text{quantil 95\%})$, o quadro do H3 volta ao que era
antes. Os três aprovados continuam aprovados e os dois de Toronto continuam reprovados. E a reprovação de Toronto
de 1997–2002 deixa de parecer ruído. Ela era sinal.

**A deriva de rótulo que não avisou.** A auditoria também perguntou se dois testes clássicos de deriva de rótulo
teriam visto o desvio de 2013 antes do sensor original. Não viram. O teste que compara a taxa realizada com a média
prevista ligou no cofre em 6 de novembro de 2013, e o sensor original já havia ligado em 16 de outubro; pior, ele
ficou ligado em 95% das verificações do desenvolvimento de 2013, porque com uma janela de três mil voos a taxa de
atraso oscila mais de 0,02 de um dia para o outro. Um alarme permanente não avisa nada. A correção segurou o único
desvio real que apareceu, no terceiro trimestre de 2017, quando o modelo cru marcava $+0{,}028$ e o corrigido,
$+0{,}001$. Mas o estresse declarado não discriminou, porque o reajuste diário absorve a deriva sozinho no nível do
trimestre. O modelo cru também passou. Isto é estresse, não prova.

**A auditoria invertida, e quem paga.** A pergunta do quarto invariante, "quem paga pelo erro?", foi rodada ao
avesso nos três fluxos de prisões. A métrica **escolhida** é a taxa de falso positivo do marcador,
$P(\text{marcado} \mid \text{não sofre o desfecho})$; a **invertida** é a taxa de falso negativo,
$P(\text{não marcado} \mid \text{sofre o desfecho})$. O grupo menos favorecido sai da maior taxa-base, o pagador é
o de maior taxa sob a métrica em vigor, e empate fica para os casos com intervalos de confiança sobrepostos. A varredura de
limiares vai de 0,05 a 0,95, mais o limiar declarado de cada fluxo: 0,70 nos dois fluxos de Toronto e 0,90 em
Rhode Island.

Em Toronto, de 1997 a 2002, o grupo menos favorecido é o de **menor** taxa de falso negativo. No limiar
declarado de 0,70, o negro perde 0,64 dos casos em 1998 contra 0,70 do branco, enquanto pela métrica escolhida
quem paga é o negro (0,196 de falso positivo contra 0,105) e pela invertida o negro sai, ficando o empate ou o
branco. Quem é marcado com mais frequência sem sofrer o desfecho é o negro; quem é perdido com mais frequência é o
branco. Trocar a métrica troca quem paga, e nesse fluxo isso acontece num limiar do grid por ano, todos **acima**
do declarado: 0,85 em 1998, 0,50 em 2000 e 0,80 em 2001, e em nenhum em 1999 e 2002. No fluxo de 2020 e 2021 a
troca aparece no segundo semestre de 2020, em 5 dos 19 limiares varridos (0,35 a 0,55), todos abaixo do declarado;
no limiar de 0,70 do fluxo, os quatro semestres empatam sob as duas métricas. Em Rhode Island não há troca em
limiar nenhum da varredura. O grupo menos favorecido paga sob a métrica escolhida em 2014 e empata em 2013 e 2015,
e a taxa de falso negativo dele fica entre 0,850 e 0,927 no limiar de 0,90.

Duas ressalvas de desenho fecham o capítulo. A primeira: em Rhode Island a pergunta mudou de "liberação depois da
prisão" para "prisão numa parada de trânsito", e a troca foi confirmada por quem conduz o programa, e não pelo
grupo que passou a ser o alvo da decisão. A segunda: o registro dos grupos afetados é curto e não é bom, porque em
sete decisões que envolvem pessoas **nenhum grupo afetado foi ouvido**. O Capítulo 21 volta a esse ponto.

**Declaração de divergência (o balanço de Platt).** O registro do H3 diz que, no fluxo de voos de 2023, o
intercepto da correção variou de $-0{,}52$ a $+0{,}43$ ao longo do ano. O diário de 2023 mostra que esse é o
intervalo **dentro do cofre**; no ano inteiro, o mesmo intercepto vai de $-1{,}01$ a $+1{,}25$. Os dois valores
estão certos e medem coisas diferentes: um é o cofre, o outro é o ano. Fica registrado em vez de escondido.

**Declaração de divergência (o caso do numpy).** O número publicado do laço performativo — efeito estimado
$0{,}5632$ com 2.375 choques — foi contestado uma vez, e vale contar como terminou. O código, com a semente
declarada, dava $0{,}5529$ e 2.395 choques quando rodava com o `python3` padrão da máquina. Não havia duas versões
do código. O arquivo é um só, e as quatro cópias da árvore trazem o resumo criptográfico idêntico. O que havia eram
dois pacotes numéricos dentro de um só interpretador: o numpy 2.5.1 do registro e o numpy 2.4.6 que o caminho padrão
importa antes. O ponto exato da divergência foi encontrado: com o estado do gerador idêntico, um sorteio binomial
de 248 tentativas com probabilidade 0,6 devolve 171 em uma versão e 150 na outra, porque a versão 2.5 corrigiu a
série de Stirling do teste de aceitação do BTPE — o terceiro e o quarto termos eram somados em vez de subtraídos, e o
coeficiente principal dizia 13680 onde devia dizer 13860. Daí em diante as duas simulações descorrelacionam, e o
sorteio de exploração muda de padrão. **Nenhum número publicado mudou**: o r̂ de 0,5632 com 2.375 choques reproduz
no ambiente de registro, e o outro valor é o do outro pacote. A lição para quem for conferir: registre a versão do
ambiente junto com o número, porque um número sem ambiente é um número que ninguém consegue auditar.

**O que fica aberto.** O tempo de atendimento e a paciência dos clientes da central continuam supostos, porque a
base não traz nenhum dos dois. O laço performativo é um laço declarado sobre chegadas reais, e não uma medida do
mundo. A latência do boletim de ciclones (3 horas) e a do táxi dos voos (20 minutos) são suposições. E o horizonte
não é um serviço rodando: é um replay sob relógio.

---

### Degrau 3 — Mão na massa

O laboratório deste capítulo monta as três peças de que o H3 depende, o escalonamento de Platt, o erro esperado de
calibração e o canário, sobre previsões simuladas, com o acaso embutido em dez linhas de SplitMix64. A logística é
ajustada por mínimos quadrados reponderados, que é determinístico e não precisa de biblioteca. O código está em
`crates/stochology-book/src/cap20.rs`, e no rascunho de escrita rodou assim:

```bash
cargo run --release
```

O mundo é uma parada de trânsito. O desfecho é "houve prisão", com três covariáveis (noite, velocidade alta,
placa de outro estado) e um modelo aditivo em log-razão de chances. O período de treino (2005–2012) tem taxa-base
mais alta; o desenvolvimento (o período antes do cofre) e o cofre (2013–2015, 24.466 paradas) têm distribuição e
taxa iguais, cerca de 2,2%. O modelo cru é treinado no período antigo e aplicado ao cofre, e a correção de Platt é
ajustada no desenvolvimento, onde os rótulos já são conhecidos.

```text
== Capítulo 20 :: o relogio, o canario e o cofre ==
semente 2021 | treino 120000 paradas (taxa 0.0352) | desenvolvimento 58000 (taxa 0.0222) | cofre 24466 (taxa 0.0220)
modelo cru (treinado no periodo antigo de taxa mais alta): media prevista 0.0351 (60% acima da taxa do cofre)
  CITL = +0.0132 | ECE = 0.0132 | Brier = 0.02155 | habilidade = -0.0020
modelo oficial com Platt (a = 0.98, b = -0.54): media prevista 0.0222
  CITL = +0.0002 | ECE = 0.0022 | Brier = 0.02130 | habilidade = +0.0098
climatologia fixa na taxa do passado: Brier = 0.02151 (p(1-p) = 0.02171)

canario: uma coluna igual ao rotulo, publicada no instante do rotulo
  relogio honesto: canario disponivel em 0 de 24466 emissoes | beta_canario = +0.00 | ganho de habilidade = +0.0000
  relogio desligado: canario disponivel em 24466 de 24466 emissoes | beta_canario = +9.23 | ganho de habilidade = +0.6521
```

Leia linha por linha. **O modelo cru** prevê, em média, 0,0351 onde a taxa do cofre é 0,0220, isto é, 60% acima.
Esse excesso aparece nas duas métricas a um só tempo, já que o erro na média vale $+0{,}0132$ e o erro esperado de
calibração vale 0,0132, e o desvio é quase todo de **nível**. O modelo aprendeu a taxa-base do período
antigo e não percebeu que o mundo mudou. A habilidade de Brier fica negativa ($-0{,}0020$), pois o modelo perde
para o climatologista que só repete a taxa do passado. **O modelo oficial**, com Platt ($a = 0{,}98$, $b =
-0{,}54$), volta ao nível certo, com erro na média $+0{,}0002$ e ECE 0,0022, e habilidade $+0{,}0098$. Note o ponto em que
a correção **não** fez nada. A habilidade subiu de $-0{,}0020$ para $+0{,}0098$, e não para $+0{,}5$. A inclinação
quase não mudou (o ajuste ficou em 0,98), e a resolução continua sendo a dos dados. A calibração se conserta;
a resolução não. **O canário** fecha o laboratório. Com o relógio honesto, a coluna do canário fica disponível em
**0** das 24.466 emissões, o coeficiente dela fica em zero e o ganho de habilidade é 0,0000; com o relógio
desligado, ela está disponível nas 24.466 emissões e o ganho é $+0{,}6521$.

**A cadeia do H3, re-executada até onde a árvore deixa.** O que acontece com as quatro linhas deste horizonte
quando o andaime Python sai da árvore? Cada uma recebe uma classe, e o instrumento **sabe reprovar** uma linha que
se declare de uma classe que o dado não sustenta: **medida**, quando o dado bruto está na árvore e a coisa é medida
de novo; **derivada**, quando o número está num arquivo versionado e o instrumento refaz a conta e o veredito; e
**registro**, quando o dado bruto não está aqui e só resta declarar o que se mediu. O código está em
`crates/stochology-h3/src/cadeia.rs` e o registro em `fundo/p14-h3/registro.tsv`:

```bash
cargo run --release --bin labs -- --h3
```

```text
== Ponto 14 :: a cadeia do H3, re-executada ate onde a arvore deixa ==
carimbo do registro: 2026-10-16
linha            classe    marca registro versionado                o que o instrumento faz
h3.fluxos        registro  P    -                                  declara: o dado bruto nao esta na arvore
h3.ri            registro  D    -                                  declara: o dado bruto nao esta na arvore
h3.laco          derivada  V    fundo/p06-laco/resultados.json     re-deriva a conta e o veredito
h3.hipergrafo    registro  P    -                                  declara: o dado bruto nao esta na arvore

-- o laco performativo, re-derivado do registro em arvore --
estimativa 2SLS: r = 0.1446378859 | IC90 de 0.1319811884 a 0.1568385251 | F do primeiro estagio = 5523.46
razao r declarado / r medido: 4.1482907207 (o registro traz 4.1482907207) | banda declarada: 0.3333 a 3.0000
sinal confere: true | ordem de grandeza confere: false | compativel no sentido estrito: false (o registro diz false)
vies do previsor ingenuo contra a demanda exogena: 0.2912
divergencias entre a conta e o registro: nenhuma
veredito: o laco do H3 superestima o medido: o sinal confere e a ordem nao (razao 4.1483, banda de 0.3333 a 3.0000)

-- prova negativa: um registro em que a razao cai dentro da banda declarada --
  com r medido 0,4: ordem confere = true | veredito: o laco do H3 tem o sinal e a ordem do medido (razao 1.5000 dentro da banda)
-- prova negativa 2: um registro em que o campo da razao nao bate com a conta --
  divergencias: ["a razao do registro (2.000000) nao bate com a conta (4.148291)", "a contencao do registro (true) nao bate com a conta (false)"]
```

Leia o cabeçalho da tabela de cada linha. **Os seis fluxos** e **o hipergrafo** ficam na classe registro: os
diários e o grafo saíram da árvore com o andaime, e o que existe em Rust para eles é o **mecanismo** — o canário
deste capítulo, que mede o relógio, e não os seis fluxos. **O cofre de Rhode Island** também é registro, e por isso
segue com a marca **D**: o laboratório mede o mecanismo, e a divergência contra o cofre real está declarada três
parágrafos acima. **O laço performativo** é a única linha derivável, porque o registro da medida dele está
versionado: o instrumento lê o `resultados.json`, refaz a conta do critério que foi declarado antes de rodar e
chega ao mesmo veredito — o laço declarado em 0,6 é **4,1483 vezes** o medido, fora da banda de 1/3 a 3, com zero
divergência entre a conta e o registro. É a diferença entre "confie no número que está no capítulo" e "rode o
comando e veja a conta refeita".

**Declaração de divergência.** O experimento acima é simulado e foi calibrado para ter a estrutura do cofre de
Rhode Island — taxa de 2,26%, 24.466 paradas, correção ajustada no período anterior —, não para reproduzir os
números dele. Os valores medidos aqui e os do repositório, lado a lado: erro na média do modelo cru, $+0{,}0132$
aqui contra $0{,}013$ lá; erro na média com Platt, $+0{,}0002$ aqui contra $0{,}0020$ lá; ECE com Platt, 0,0022
aqui contra **0,0048** lá; habilidade com Platt, $+0{,}0098$ aqui contra **$+0{,}019$** lá; excesso da taxa pelo
modelo cru, 60% aqui contra 58% lá. A ordem de grandeza e o sinal de todas elas coincidem, e o motivo das
diferenças é o dado: o de cá é gerado por um modelo aditivo simples, com três covariáveis e sem o ruído de um
registro policial real. O ganho do canário com o relógio desligado, $+0{,}6521$, cai dentro da faixa medida nos
cofres reais, que vai de $+0{,}55$ (prisões de 2020–2021) a $+0{,}97$ (Rhode Island).

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 20 | perda de resolucao 46.24 min (demanda de 5 a 11) | o cofre ao vivo: laboratório 0,0022 e cofre real 0,0048 são dois grãos de teste
```

**Exercícios.**

1. *(Conceito)* Por que um canário que não dá ganho com o relógio honesto prova pouco se ninguém o testa com o
   relógio desligado? Dê um exemplo de coluna que seria um canário ruim.
2. *(Conceito)* Explique a falha do disjuntor nos voos de 2013 e proponha um segundo critério de abertura baseado
   em calibração, dizendo qual preço ele cobra em número de trocas de previsão.
3. *(Conta)* No laboratório, a climatologia fixa tem Brier 0,02171 e o modelo oficial, 0,02130. Calcule a
   habilidade e confira com o número medido de $+0{,}0098$ (a referência medida usa a taxa do passado, 0,0222, e
   não a taxa do cofre).
4. *(Laboratório)* Troque, no laboratório, a taxa do período de treino aproximando-a da taxa do cofre (mude o
   intercepto antigo de $-4{,}005$ para $-4{,}30$) e rode de novo. O modelo cru passa a ser bem calibrado? O que
   isso diz sobre o valor do Platt nesse desenho?
5. *(Conceito)* Por que a recalibração de intercepto não corrige um modelo pouco confiante nos extremos?
   Descreva a curva de calibração dos dois casos, com o eixo das previsões contra o das frequências.
6. *(Conceito)* Em Toronto de 1997 a 2002, trocar a métrica troca quem paga. Escreva, em quatro frases, por que
   isso não é um erro de cálculo e sim uma decisão de valor.

**Resumo.**

- Zero vazamento só vale com um canário que prove que o teste acharia o vazamento: 0,000 com o relógio honesto,
  de $+0{,}55$ a $+0{,}97$ com o relógio desligado. E das quatro linhas deste horizonte, só o laço performativo
  pode ter a conta refeita hoje: as outras três perderam o dado bruto com o andaime, e o que sobra delas é o
  mecanismo medido aqui.
- Um previsor pode acertar o que observa (1% de erro) e errar o mundo (29% de excesso); o instrumento sorteado
  dentro do laço desfaz a espiral, e o laço declarado em 0,6 é quatro vezes o medido, 0,145.
- Cada correção de calibração só foi aceita num cofre novo: voos de 2023 com ECE 0,008, prisões de 2020–2021
  reprovadas por inclinação (0,067) e Rhode Island aprovada com Platt, ECE 0,0048 e habilidade $+0{,}019$.
- Calibração se conserta; resolução fica onde está. E com uma tolerância que depende do tamanho do cofre, nenhum dos seis
  cofres binários passa — nem os três aprovados.
- Trocar a métrica de justiça troca quem paga pelo erro; e nenhum dos grupos afetados foi ouvido nas sete
  decisões que envolvem pessoas.
