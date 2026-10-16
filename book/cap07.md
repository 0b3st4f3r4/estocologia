## Capítulo 7. Caçando fantasmas no ruído: testes severos e falsas descobertas

**Objetivos.** Ao fim deste capítulo você deve saber construir o modelo nulo exato de um experimento e
comparar com ele; reconhecer as liberdades de pesquisador que inflam falsos positivos; calcular o valor
preditivo de um achado com a fórmula de Ioannidis; nomear os cinco tipos de erro; e distinguir um
atalho que é erro de um atalho que é adaptação ao ambiente.

---

### Degrau 1 — O mundo real e o paradoxo

**Cem pessoas jogando moedas.** Reúna 100 pessoas. Cada uma joga 20 vezes uma moeda honesta e anota a
maior sequência de caras que conseguiu. Ninguém trapaceia. Ainda assim, em **100 de cada 100
experimentos** que este livro simula, alguém emenda 6 caras seguidas ou mais, e a maior sequência tem,
em média, **9,5 caras**. Se o repórter escolher contar a história da pessoa com a sequência mais longa,
o jornal de amanhã traz um talento que não existe. Não houve erro de medida: houve um **modelo nulo**
esquecido.

**A mão quente.** O caso mais famoso é o da "mão quente" no basquete. Gilovich, Vallone e Tversky (1985)
analisaram os arremessos de jogadores profissionais e concluíram que a crença de que quem acerta tende a
acertar de novo era ilusão de padrão: a proporção de acertos depois de um acerto era igual à proporção
geral. Por três décadas o resultado foi citado como o exemplo canônico de ilusão.

Em 2018, Miller e Sanjurjo mostraram que o **teste** estava viciado. A conta original agrupava todos os
arremessos e comparava a proporção de "acerto depois de acerto" com um meio. Mas essa proporção, num
processo sem memória nenhuma, **não vale um meio**: em três lances de uma moeda honesta, a média das
proporções por sequência é $5/12 = 0{,}4167$. O alvo estava no lugar errado. Corrigida a conta, a
diferença nos dados de Cornell passa de $+3$ para $+13$ pontos percentuais. O fantasma não estava nos
jogadores: estava no denominador.

**O jardim dos caminhos que se bifurcam.** Simmons, Nelson e Simonsohn (2011) pediram a
pesquisadores que declarassem, antes de coletar, o que fariam. Quatro decisões comuns são:
*quando parar de coletar*, *quais variáveis usar*, *quais casos excluir* e *quais comparações
relatar*. Cada uma parece razoável isolada. Juntas, num experimento em que **não existe efeito
nenhum**, elas levam a taxa de falsos positivos de 5% para **60,7%**. Em cada 100 estudos feitos
assim, em 61 aparece um "efeito significativo" que não existe.

**Cinquenta casos, cinquenta preditores, nada.** Um analista tem 50 casos e 50 variáveis medidas. Nada
tem relação com a resposta. Ele ajusta 500 modelos diferentes e fica com o de menor erro de validação.
O vencedor parece melhor que "prever zero" em **73% das repetições** — e, diante de dados novos, não é
melhor. Nenhuma conta errada, nenhum dado inventado: apenas 500 tiros num alvo que não existe, e o
melhor tiro escolhido depois de ver onde ele caiu.

---

### Degrau 2 — A regra do jogo

**O modelo nulo exato.** O primeiro passo de qualquer análise não é olhar os dados: é escrever, em
código, o mundo em que **nada acontece** e medir o que ele produz. O modelo nulo precisa ter a mesma
estrutura dos dados — o mesmo número de casos, o mesmo número de comparações, a mesma liberdade de
escolha —, porque é ele que diz qual é o alvo certo.

A mão quente é o exemplo mais limpo. Em três lances de uma moeda honesta, as oito sequências possíveis
são igualmente prováveis. O estimador que a literatura usava é a **média das proporções por sequência**,
e as sequências sem nenhum acerto não têm proporção definida e saem da conta. Sobram seis sequências,
com proporções 1, 1/2, 0, 0, 1 e 0: a média é $2{,}5/6 = 5/12$. Já o estimador **agrupado** — total de
acertos depois de acertos sobre total de transições depois de acerto — dá $4/8 = 1/2$. Os dois
estimadores respondem a perguntas diferentes, e comparar o primeiro com 1/2 é comparar com o alvo
errado. O erro é de 0,0833: oito pontos percentuais de "mão quente" inventada.

**A severidade de Mayo.** Deborah Mayo propôs a pergunta que organiza tudo: *um resultado só é
evidência se o procedimento provavelmente teria dado errado caso a afirmação fosse falsa.* Para um teste
pré-registrado que produz $p = 0{,}04$, a severidade do resultado é $1 - p = 0{,}96$: se a hipótese
fosse falsa, o teste teria dado um resultado menos extremo em 96 de cada 100 vezes. Quando o mesmo
"$p = 0{,}04$" sai de um jardim de caminhos, a severidade que importa não é $1 - p$: é $1$ menos a taxa
de falso positivo **do procedimento**, porque foi o procedimento inteiro que produziu o número.

**O teorema de Ioannidis.** O valor preditivo de um achado positivo depende do poder $1-\beta$, das
chances $R$ de que uma hipótese da área seja verdadeira e do nível de significância $\alpha$:

$$\text{VPP}=\frac{(1-\beta)R}{R-\beta R+\alpha}$$

Em palavras: mesmo um estudo bem-feito pode ter pouca chance de estar certo, se a área testa muitas
hipóteses falsas. Com poder 0,80 e $\alpha = 0{,}05$: se uma em cada dez hipóteses é verdadeira
($R = 1/10$), o VPP é 0,615; se uma em cada mil ($R = 1/1000$), cai para 0,016. Se $\alpha$ for o do
jardim em vez de 0,05, os mesmos números desabam. A Open Science Collaboration (2015) replicou 100
estudos de psicologia: **97% dos originais** tinham $p < 0{,}05$, contra **36% das replicações**.

**Um ajuste só é evidência se o modelo podia ter falhado.** Num espaço de duas dimensões, um
semiplano separa ao acaso 50 pontos com probabilidade $2 \times 10^{-12}$ — cerca de **39 bits** de
evidência. Uma tabela que guarda cada rótulo também separa os 50 pontos, e não traz bit nenhum: ela
separaria **qualquer** conjunto de rótulos, com ou sem sinal. Mesma separação, evidência zero. O teste
tem de poder errar para que acertar signifique algo.

**Os cinco erros.** A estatística clássica nomeia dois, e a prática acrescentou outros três.

| Erro | O que acontece | Fonte |
| --- | --- | --- |
| Tipo I | Rejeitar uma hipótese nula verdadeira: o falso alarme | Neyman e Pearson |
| Tipo II | Não rejeitar uma nula falsa: o efeito perdido | Neyman e Pearson |
| Tipo III | Dar a resposta certa ao problema errado | Kimball (1957) |
| Tipo S | Estimar um efeito com o sinal trocado | Gelman e Tuerlinckx (2000); Gelman e Carlin (2014) |
| Tipo M | Exagerar a magnitude de um efeito verdadeiro | Gelman e Carlin (2014) |

Os tipos S e M explicam por que estudos pequenos e "significativos" decepcionam na replicação: com
pouco poder, só passam no filtro de $p < 0{,}05$ as estimativas que o acaso inflou, e algumas têm até o
sinal errado. O tipo III é o mais estocológico de todos, porque não é erro de conta: é um desalinhamento
entre a pergunta do Agente e a pergunta que o Modelo responde. O Capítulo 5 mostra o exemplo do gestor
que perguntou pela espera média e recebeu a resposta de um mundo que não existe.

**Atalho contra ambiente.** Kahneman e Tversky catalogaram como as pessoas se afastam das regras da
probabilidade; Simon e Gigerenzer responderam que um atalho só é erro **relativo a um ambiente**. A
Estocologia propõe um critério em vez de escolher um lado. Um desvio é **erro** quando a informação
certa estava disponível e a regra de combinação foi violada; é **adaptação candidata** quando a norma
supõe uma informação que o ambiente não fornece de modo confiável.

Os dois números que separam os casos. Num rastreamento com prevalência de 1%, quem ignora a taxa-base
tem escore de Brier de **0,083**, contra **0,009** de quem a usa. Num ambulatório de referência, onde a
prevalência real é de 33%, quem insiste em 1% tem **0,28**, contra **0,084** de quem a ignora. A mesma
heurística é erro num ambiente e acerto no outro. O escore de Brier é o assunto do Capítulo 8.

**A banca adversarial.** Numa arquitetura de pesquisa, a defesa contra o jardim não é a boa intenção: é
um ataque formal. Oito agentes tentam quebrar o artefato antes da aprovação, cada um numa faixa:

| Faixa atacada | O ataque |
| --- | --- |
| Código | Uma bateria nova de propriedades, com semente que o autor não escolheu |
| Dados | Um lote com registros corrompidos, que precisa ser isolado |
| Modelo em operação | Um lote com deriva forte, ao vivo |
| Inferência | Um conjunto de dados patológico, para ver se a incerteza denuncia a inadequação |
| Governança | Adulteração da trilha de auditoria, que o verificador da cadeia tem de detectar |
| Justiça | Inverter a métrica escolhida e mostrar quem passaria a pagar |
| Domínio | Um cenário atípico do domínio, com priori alternativa, para ver se a decisão resiste |
| Decisão | Uma crise que viola a própria cláusula de saída |

A aprovação exige que **todos** os oito atinjam severidade mínima:

$$\text{Aprovado}(X) = \bigwedge_{i=1}^{8}\Big(A_i(X) \ge \text{Severidade}_{\min}\Big)$$

com $\text{Severidade}_{\min} = 6$ numa escala de 0 a 10, e mesma ficha e mesma semente devolvendo
sempre a mesma severidade. A regra é dura de propósito: a nota média da banca não aprova nada, e uma
faixa fraca declarada vale mais que um trabalho uniforme sem lacuna admitida. Nenhum agente aprova o
próprio trabalho. Três falhas eliminam o trabalho antes de qualquer discussão de mérito: vazamento
temporal, contrato de dados violado sem tratamento e ausência de um responsável humano nomeado.

---

### Degrau 3 — Mão na massa

O laboratório mede quatro coisas: o jardim de modelos em ruído puro, o modelo nulo exato da mão quente,
o jardim dos caminhos e a severidade da banca. Código em `crates/stochology-book/src/cap07.rs`; saída medida com
`cargo run --release`:

```text
== Capítulo 7 :: caçando fantasmas no ruído ==
jardim de modelos: 200 replicas, 500 candidatos em 50 preditores de puro ruido, 25 casos de treino
o vencedor da validacao bate prever zero em 79.5% das replicas; diante de dados novos, em 30.0%
erro quadratico medio do vencedor: 0.814 na validacao (prever zero: 1,000) e 1.191 em dados novos
mao quente: media das proporcoes por sequencia = 0.4167 (5/12 = 0.4167); proporcao agrupada = 0.5000
100 pessoas jogando 20 moedas: em 100.0% dos experimentos alguem emenda 6 caras seguidas; a maior sequencia tem media 9.5
jardim dos caminhos (4000 experimentos sem efeito nenhum): com as quatro liberdades de Simmons, alguem acha p < 0,05 em 21.5% deles; com a familia grande, em 47.0%; o teste pre-registrado acusa 5.2%
severidade de Mayo: 1 - p = 0.96 no teste pre-registrado com p = 0,04; 1 - taxa de falso positivo = 0.53 no jardim
Ioannidis com R = 1/1000, poder 0,80: VPP = 0.016 com alfa 0,05 e 0.002 com o alfa do jardim
Ioannidis com R = 1/10, poder 0,80: VPP = 0.615 com alfa 0,05 e 0.146 com o alfa do jardim
Ioannidis com R = 1/3, poder 0,80: VPP = 0.842 com alfa 0,05 e 0.362 com o alfa do jardim
banca A1-A8 com a semente 2026, ficha com os oito cuidados: severidades [9, 8, 10, 9, 8, 9, 8, 10], piso 6, aprovada = true
a mesma ficha sem o cuidado de uma faixa: a menor severidade cai para 2 (reprovada)
```

Cinco leituras. **(1)** O vencedor do jardim de modelos tem erro 0,814 na validação, contra 1,000 de
"prever zero" — parece ótimo — e **1,191 em dados novos**: ele é pior que o zero, e por isso "bate
zero" apenas 30% das vezes quando o juiz é novo. A validação foi usada como alvo, e o alvo virou
veneno. **(2)** O modelo nulo exato da mão quente é 0,4167, e não 0,5: a média das proporções por
sequência coincide com 5/12 na casa dos quatro decimais, enquanto o estimador agrupado fica em 0,5000.
O fantasma é exatamente a diferença entre os dois. **(3)** As quatro liberdades clássicas levam o falso
positivo de 5,2% (teste pré-registrado) para **21,5%**, e a família grande — com parada opcional em
quatro pontos, três conjuntos de covariáveis, exclusão do caso extremo e dois subgrupos — leva para
**47,0%**. **(4)** A severidade de Mayo do "$p = 0{,}04$" pré-registrado é 0,96; a do mesmo número
saindo do jardim é 0,53, porque quem produz o número é o procedimento. **(5)** A banca A1–A8 com a
semente 2026 dá severidades 9, 8, 10, 9, 8, 9, 8 e 10 numa ficha com os oito cuidados — aprovada —, e
basta uma faixa sem o cuidado para a menor severidade cair a 2 e a aprovação sumir. O AND das oito não
perdoa.

**Declaração de divergência.** A edição anterior deste livro registra 73% de vencedores que "parecem
melhores que prever zero", a taxa de 60,7% do jardim de Simmons e uma severidade de 0,42 para o
$p = 0{,}04$ do jardim. Este instrumento mede **79,5%**, **21,5%** com as quatro liberdades (e **47,0%**
com a família grande) e **0,53**. Os números diferem porque o protocolo completo não foi publicado: o
jardim antigo varria mais combinações — dois desfechos, dois sexos, mais covariáveis, mais pontos de
parada opcional —, e cada combinação a mais empurra o falso positivo para cima; no jardim de modelos, o
tamanho da validação e a faixa de preditores por candidato mudam o resultado, e aqui o vencedor ainda
falha diante de dados novos (erro 1,191 contra 1,000). O que não muda é a direção: o teste
pré-registrado acusa 5,2%, perto dos 5% nominais, e o jardim multiplica isso por quatro ou por nove. Os
dois valores ficam registrados; nenhum foi apagado.

**Exercícios.**

1. *(Conceito)* Explique, sem conta, por que a média das proporções por sequência ($5/12$) é menor que
   a proporção agrupada ($1/2$). Que sequências entram numa conta e não na outra?
2. *(Conta)* Com $R = 1/20$, poder 0,80 e $\alpha = 0{,}05$, calcule o VPP. Refaça com o poder caindo
   para 0,40 e diga, em uma frase, o que acontece com o valor de um achado positivo quando o estudo é
   pequeno.
3. *(Laboratório)* No laboratório, reduza os candidatos de 500 para 10 e depois para 5000. Meça a
   fração de vencedores que batem zero na validação e em dados novos. Desenhe os dois pontos e diga o
   que o número de tentativas faz com a aparência do vencedor.
4. *(Laboratório)* Acrescente ao jardim a liberdade "usar a média ou a mediana do desfecho, o que der o
   menor p" e meça a nova taxa de falso positivo. O que isso diz sobre declarar as liberdades antes de
   rodar?
5. *(Conta)* Na banca adversarial, a severidade de cada agente é sorteada entre 8 e 10 quando o cuidado
   existe e entre 2 e 4 quando não existe. Calcule a probabilidade de aprovação de uma ficha com os oito
   cuidados e de uma ficha com uma única faixa sem o cuidado. Depois explique por que o desenho escolheu
   esses dois intervalos, e não um sorteio de 0 a 10.
6. *(Conceito)* Dê um exemplo do seu campo em que ignorar a taxa-base seja adaptação ao ambiente, com
   os dois números de Brier que sustentam a sua resposta.

**Resumo.**

- Todo padrão precisa de um modelo nulo exato, com a mesma estrutura dos dados e do procedimento.
- O alvo da mão quente é $5/12$, não $1/2$: o mesmo estimador, comparado com o alvo errado, inventa o
  efeito.
- Quatro liberdades de pesquisador levam o falso positivo de 5% a dezenas por cento sem nenhum efeito
  no mundo.
- Ioannidis: o valor preditivo de um achado cai com $\alpha$ e com as chances de a hipótese ser falsa.
- Cinco erros, não dois: I (alarme falso), II (cegueira), III (resposta certa à pergunta errada), S
  (sinal trocado) e M (exagero) — e a defesa é institucional: a banca A1–A8 e a severidade de Mayo
  trocam a boa intenção por um teste que podia ter falhado.
