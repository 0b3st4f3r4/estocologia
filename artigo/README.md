# "Estocologia": Velhas Fronteiras, Novos Horizontes

Silvano Neto · 14 de outubro de 2026, atualizado em 15 de outubro de 2026

## Resumo

Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza. Este artigo defende que ela não é um ramo novo da estatística, mas uma costura: junta três tradições que cresceram separadas (a filosofia da indução, a inferência estatística e a mecânica dos processos estocásticos) e as organiza em três camadas, Mundo, Modelo e Agente. A tese central é que a maior parte dos erros de raciocínio sob incerteza não é erro de conta, e sim desalinhamento entre essas camadas.

O artigo percorre as velhas fronteiras que a disciplina herda, apresenta a arquitetura com que o projeto pretende operá-la (um hipergrafo tipado, auditado por agentes especializados e por quatro invariantes) e relata os primeiros resultados. Num posto de saúde simulado, a mesma demanda média produz uma espera média de 9,3 ou de 20,7 minutos, conforme o mundo varie ou não. Em dados reais, a mesma lógica mostra que a escala padrão de uma central bancária, que promete cumprir a meta todo dia, falha em 26 de cada 100. O programa passou nos quatro horizontes do seu plano, mas não de primeira: o terceiro, a busca de estruturas causais invariantes entre áreas, exigiu sete rodadas, e o quarto, a calibração contínua fora do tempo, três tentativas no fluxo mais difícil. O artigo trata as falhas do caminho como resultado e termina com os horizontes que se abrem e as objeções que a disciplina precisa enfrentar.

## 1. Introdução: decidir sem saber

Toda decisão que importa é tomada antes de sabermos o que vai acontecer. O médico prescreve sem conhecer a resposta daquele paciente, o juiz condena sem ter visto o crime, o gestor contrata pessoal para uma demanda que ainda não chegou. Em todos esses casos, alguém precisa transformar informação incompleta em uma ação concreta, e responder pelas consequências.

Há séculos existem ferramentas para isso: a probabilidade, a estatística, a teoria da decisão, a psicologia do julgamento. O problema é que elas raramente conversam. O estatístico calcula intervalos sem perguntar de onde vieram os dados; o filósofo discute a indução sem rodar uma simulação; o psicólogo cataloga vieses sem dizer em que ambiente o atalho seria ótimo. Cada um enxerga um pedaço do problema.

**Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.** O nome vem do grego *stókhos*, o alvo para onde se mira, a conjectura bem orientada. Não é uma escolha arbitrária: em 1713, Jacob Bernoulli chamou a sua obra de *Ars Conjectandi*, a arte de conjecturar, e definiu essa arte como *stochastice*. Três séculos depois, a palavra "estocástico" virou sinônimo de "aleatório" e perdeu o sentido original de mirar bem quando não se vê o alvo. A Estocologia quer recuperar esse sentido.

O "ou arte" da definição é deliberado. Julgar sob incerteza tem uma parte que se formaliza (a coerência das crenças, a atualização pela evidência, a margem de erro) e uma parte que não se formaliza por completo (decidir qual modelo vale a pena levar a sério, quando parar de coletar dados, quem paga se o erro acontecer). Uma disciplina honesta sobre incerteza precisa ser honesta também sobre os limites do próprio formalismo.

## 2. Velhas fronteiras

A Estocologia não inventa seus problemas; herda-os. Três fronteiras antigas organizam essa herança, e cada uma delas separou pessoas que deveriam estar conversando.

**A fronteira entre disciplinas.** A filosofia perguntou se a indução se justifica. David Hume mostrou que qualquer defesa dela pressupõe que o futuro se parecerá com o passado, o que é circular. Karl Popper respondeu trocando a confirmação pela refutação; Duhem, Quine e Lakatos lembraram que nenhuma hipótese é testada sozinha. Em paralelo, a matemática construiu a probabilidade (Pascal e Fermat, Bernoulli, Bayes, Laplace, Kolmogorov) e a estatística construiu a inferência (Fisher, Neyman, a escola bayesiana). Uma terceira linhagem, a dos processos estocásticos (Markov, Langevin), estudou como o acaso se acumula no tempo. As três tradições raramente se citam.

**A fronteira entre risco e incerteza.** Em 1921, Frank Knight separou o risco, em que as probabilidades são conhecidas, da incerteza propriamente dita, em que nem a lista de possibilidades está disponível. Boa parte da teoria da decisão do século XX (von Neumann e Morgenstern, Savage) trabalhou do lado confortável dessa linha. O peru de Russell, alimentado pontualmente por mil dias, mostra o lado desconfortável: cada dia de dados torna o modelo dele mais confiante e, ao mesmo tempo, mais errado, porque o abate nunca esteve no seu espaço de hipóteses.

**A fronteira entre a norma e o comportamento.** Kahneman e Tversky catalogaram como as pessoas se afastam das regras da probabilidade: ignoram a taxa-base, julgam a conjunção mais provável que suas partes. Herbert Simon e, depois, Gerd Gigerenzer responderam que um atalho só é erro relativo a um ambiente; quando a mesma informação é dada em frequências naturais ("8 em cada 10") em vez de porcentagens condicionais, muitos dos "vieses" desaparecem. O debate ficou parado entre dois campos, cada um com razão sobre metade do problema.

| Ano | Marco | Fronteira que deixou |
| --- | --- | --- |
| 2000 | Judea Pearl, *Causality* | Causalidade formal, mas longe da prática estatística comum |
| 1996 | Deborah Mayo, teste severo | Filosofia do erro, pouco usada por quem roda os testes |
| 1974 | Kahneman e Tversky, heurísticas e vieses | Norma versus comportamento |
| 1955 | Herbert Simon, racionalidade limitada | O agente real, sem a matemática da inferência |
| 1954 | Leonard Savage, probabilidade subjetiva | Decisão sob risco, pouco sob incerteza radical |
| 1933 | Kolmogorov, axiomas da probabilidade | Rigor matemático, silêncio sobre interpretação |
| 1921 | Frank Knight, risco e incerteza | Risco mensurável versus incerteza não mensurável |
| 1908 | Langevin, dinâmica estocástica | O acaso no tempo, separado da inferência |
| 1763 | Ensaio de Bayes (publicado por Price) | Crença como probabilidade, longa disputa com os frequentistas |
| 1748 | Hume, problema da indução | Ciência empírica sem justificativa lógica |
| 1713 | Bernoulli, *Ars Conjectandi* | A arte de conjecturar nasce junto com a lei dos grandes números |

O que essas fronteiras têm em comum é que cada lado acerta sobre o seu pedaço e erra ao generalizar. A Estocologia parte da aposta de que os erros vivem justamente nas costuras.

## 3. Fundamentação: Mundo, Modelo e Agente

O primeiro gesto da Estocologia é separar três coisas que o senso comum mistura. O **Mundo** é a realidade que gera os dados e que nunca vemos inteira. O **Modelo** é a representação formal, simplificada e simulável que construímos para aproximá-lo. O **Agente** é quem decide: uma pessoa, uma instituição ou um algoritmo, sempre com tempo, atenção e computação limitados.

![As três camadas da Estocologia e o laço da ação](figuras/camadas.png)

*As três camadas da Estocologia e o laço da ação.*

A separação tem uma consequência forte: **uma falácia não é um erro de aritmética, é um desalinhamento entre camadas.** A falácia do promotor acontece quando o Agente lê a verossimilhança do Modelo, P(evidência | inocente), como se fosse a resposta sobre o Mundo, P(inocente | evidência). O paradoxo da inspeção acontece quando o Agente impõe um Modelo estável a um Mundo que oscila. Nos dois casos as contas podem estar certas; o que está errado é a ponte.

Cada camada tem o seu tipo de incerteza, e confundi-los é um erro clássico. A incerteza **aleatória** pertence ao Mundo: é o acaso do próximo lançamento de uma moeda conhecida, e nenhuma quantidade de dados a elimina. A incerteza **epistêmica** pertence ao Modelo: é a nossa ignorância sobre o viés da moeda, e encolhe com cada lançamento. A lei da variância total separa as duas com precisão:

$$
\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{aleatória (Mundo)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistêmica (Modelo)}}
$$

A terceira, a incerteza **knightiana**, é a mais perigosa porque não aparece na equação. Ela surge quando o processo real está fora do espaço de hipóteses. Aí, mais dados não ajudam: estreitam o intervalo de confiança em volta da resposta errada, como no caso do peru.

| Incerteza | Camada | O que mais dados fazem | Como se mitiga |
| --- | --- | --- | --- |
| Aleatória | Mundo | Nada | Margens de segurança e redundância |
| Epistêmica | Modelo | Reduzem até zero | Amostragem e atualização bayesiana |
| Knightiana | Modelo e Agente | Nada, ou pior: falsa confiança | Cenários, robustez, estresse de premissas |
| Erro de especificação | Modelo | Precisão em torno do erro | Ampliar o espaço de hipóteses e auditar premissas |

A ferramenta matemática que atravessa as três camadas é modesta: a desigualdade de Jensen. Para uma função convexa f, o resultado médio não é o resultado do caso médio:

$$
\mathbb{E}[f(X)] \;\ge\; f(\mathbb{E}[X]), \qquad \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \tfrac{1}{2} f''(\mu)\,\operatorname{Var}(X)
$$

Em palavras: quando o sistema reage de forma não linear, a variabilidade do Mundo não se cancela na média; ela cobra um preço proporcional à sua variância. Planejar pelo caso médio é ignorar essa conta. A seção 5 mostra quanto ela custa num posto de saúde.

## 4. Arquitetura: uma máquina epistemológica auditável

Uma disciplina só vira prática quando ganha instrumentos. O projeto propõe operar a Estocologia como uma **máquina epistemológica auditável**: um sistema em que cada alegação empírica vira um pedaço de código executável, e em que nenhuma conclusão é aceita sem passar por testes automáticos. A ideia não é um buscador de textos que resume a literatura, mas um ambiente em que as hipóteses são simuladas e atacadas.

**O hipergrafo.** O conhecimento fica guardado como um grafo tipado, G = (V, E\_tree, E\_graph, H, Φ). Os nós (V) são conceitos, hipóteses, variáveis e conjuntos de dados; cada nó declara obrigatoriamente a sua camada (Mundo, Modelo ou Agente), o seu tipo de incerteza, uma definição formal e um ponteiro para código reprodutível com semente fixa. As arestas de árvore (E\_tree) registram o que depende do quê. As arestas transversais (E\_graph) são de três tipos: pontes matemáticas (a desigualdade de Jensen ligando módulos distantes), arestas causais do tipo do(x), e **arestas de patologia**, que marcam desalinhamentos: ligar um Mundo não linear a um Modelo que só olha o caso médio dispara o alerta de falácia da média.

**As cinco lentes.** As hiperarestas (H) envolvem grupos inteiros de nós e funcionam como perguntas que se fazem a qualquer problema:

- **Fluxonomia**: como a probabilidade e a informação fluem e se perdem.
- **Crononomia**: se a média entre muitos coincide com a média ao longo do tempo de um só (ergodicidade, ruína, critério de Kelly).
- **Escalonomia**: se a cauda é pesada a ponto de a média nem existir.
- **Morfonomia**: se a estrutura encontrada sobrevive quando o ambiente muda.
- **Crisonomia**: se a informação vale o seu custo e se as previsões estão calibradas.

**Os agentes.** O trabalho é dividido entre agentes especializados, oito faixas coordenadas por um orquestrador. Eles não conversam em linguagem livre, que é ambígua e alucina; trocam **fichas** estruturadas, em YAML, cada uma abrindo com as mesmas perguntas: qual processo gera os dados, onde há não linearidade, qual é a unidade independente, qual é a física do ruído, em que degrau da escada causal estamos.

| Agente | Papel | Garantia que entrega |
| --- | --- | --- |
| F1 Código | Transforma hipóteses em processos geradores executáveis | Semente fixa e testes de propriedades |
| F2 Dados | Audita a entrada do Mundo | Contrato de dados e a unidade independente certa |
| F3 Ciclo de vida | Testa o modelo no tempo | Validação em dados futuros, sem vazamento |
| F4 Fronteira | Roda a inferência pesada | Amostradores calibrados e cobertura garantida |
| F5 Governança | Cuida de legalidade e privacidade | Orçamento de privacidade em bits perdidos |
| F6 Justiça | Pergunta quem paga pelo erro | Auditoria de taxas de erro entre grupos |
| F7 Domínio | Traduz problemas de outras áreas | Elicitação de especialistas e termo de escopo |
| F8 Decisão | Converte o resultado em ação | Frequências naturais e cláusula de saída |
| M15 Orquestrador | Coordena e faz a banca adversarial | Ataque ao modelo antes da aprovação |

**O ciclo.** Uma pergunta percorre seis estágios: tradução e elicitação (F7); auditoria dos dados, da causalidade e da ética (F2, F5, F6); simulação contra um modelo nulo (F1, F4, F3); passagem pelas cinco lentes; banca adversarial, que devolve o trabalho ao estágio que falhou; e, só então, a decisão comunicada (F8).

**Os quatro invariantes.** Nada entra no grafo sem passar por quatro portões, que funcionam como travas de banco de dados:

1. **Φ1, simulabilidade**: toda alegação vira código com semente declarada e testes. Prosa não basta.
2. **Φ2, margem e unidade**: todo número vem com margem de erro, calculada sobre a unidade que é de fato independente.
3. **Φ3, diagnóstico e teste severo**: antes de ler um padrão, audita-se o filtro da amostra, desenha-se o grafo causal e confronta-se a hipótese com um modelo nulo simulado.
4. **Φ4, escopo ecológico**: a decisão é julgada pelo ambiente em que vai operar, e sai com termo de escopo e uma cláusula objetiva de quando abandoná-la.

Para não explodir em custo nem em falsas descobertas, a pesquisa passa por um funil. Primeiro corta-se o que, mesmo com resposta perfeita, não mudaria nenhuma decisão (valor esperado da informação). Depois corta-se o que não é identificável com os dados disponíveis. Depois, o que não supera o modelo nulo. Só o que sobra recebe inferência cara. Toda tentativa fica registrada, inclusive as que falharam, para que o relatório final declare quantas especificações foram tentadas.

## 5. Um primeiro experimento: a fila do posto de saúde

O primeiro teste da arquitetura (o horizonte H0 do plano do projeto) foi deliberadamente modesto: um posto de saúde simulado, o Lab 0 do curso. O posto atende 12 pacientes por hora e recebe, em média, 8. A alegação a examinar é a de qualquer planilha de gestão: "com essa média de demanda, a espera média será de uns 9 minutos".

O sistema fez o que os invariantes exigem. Traduziu a alegação em um processo gerador executável, com semente fixa (Φ1), e rodou cinco mundos que têm exatamente a mesma demanda média, mas variabilidades diferentes:

![Espera média por mundo simulado no Lab 0 (H0, h0-lab0/resultados.json)](figuras/lab0-espera-por-mundo.png)

*Espera média por mundo simulado no Lab 0 (H0, h0-lab0/resultados.json).*

Se a demanda é constante, a promessa se cumpre: 9,3 minutos. Se os dias alternam entre calmos (5 pacientes por hora) e de surto (11), a espera média mais que dobra, para 20,7 minutos, e um em cada quatro pacientes espera mais de meia hora. Nada mudou na média; mudou a variabilidade, e a fila é uma função convexa da demanda. É a desigualdade de Jensen cobrando a sua conta. O último mundo mostra o outro lado: reforçar a equipe só nos dias de surto derruba a espera para 2,5 minutos.

O experimento produziu três resultados que vão além do exemplo didático.

1. **O modelo nulo arbitra a margem de erro (Φ2).** Quem trata cada paciente como observação independente obtém uma margem de ±0,07 minuto; quem trata o dia como unidade obtém ±0,51, sete vezes maior. Qual está certa? Em vez de discutir a convenção, o sistema repartiu ao acaso 4 000 dias simulados em duas metades e mediu a diferença entre elas. O desvio observado foi 0,216; a margem por dia previa 0,227 e a margem por paciente previa 0,047. A pseudorreplicação deixou de ser uma questão de gosto: foi refutada pelos dados.
2. **O teste severo (Φ3).** A diferença entre o mundo constante e o mundo com surtos, 11,4 minutos, fica a 53 desvios do que o acaso produziria. Em 2 000 repartições aleatórias, o modelo nulo nunca passou de 0,81 minuto.
3. **A armadilha da mediana.** No mundo com surtos, a mediana da espera é 9,3 minutos, o mesmo número que a alegação promete como média. Um gestor que olhe "o paciente típico" verá a promessa cumprida. Toda a diferença está na cauda, e é por isso que a arquitetura proíbe reportar uma média sozinha: exige mediana, quantis e a curva acumulada.

Tudo isso passou pelos quatro portões e reproduziu, com a mesma contagem de pacientes (161 867 e 160 220), os seis números que o material do curso já publicava. O resultado é pequeno, mas o seu valor está no método: qualquer pessoa pode rodar o código de novo e obter os mesmos números, com o mesmo hash.

## 6. Do simulado ao real: o que os dados disseram

Depois do Lab 0, o programa percorreu os quatro horizontes do plano com bases públicas reais e passou nos quatro, os dois últimos só depois de várias rodadas de falha. As falhas foram mantidas como resultado: cada mudança de critério, de teste ou de dados foi declarada, com hash, antes de rodar, e nada foi ajustado depois de ver o resultado. A pesquisa foi encerrada em 15 de outubro de 2026; código, dados com hash e resultados estão no repositório do projeto.

| Horizonte | Dados reais | Portão de saída | Resultado |
| --- | --- | --- | --- |
| H0 Semente | 2,65 milhões de chamadas de uma central bancária (2003) | Φ1 a Φ4 e reprodução limpa | Passou |
| H1 Ciclo completo | Dow Jones 1885–2023; LaLonde (NSW e CPS); incêndios dinamarqueses 1980–1990 | Um projeto aprovado na banca adversarial | Passou: 2 de 3 projetos aprovados; o de cauda pesada foi reprovado |
| H2 Transferência | 12 domínios de conveniência; depois bases com intervenções conhecidas, câmaras físicas, deleções de genes em levedura e um ensaio de cardiologia do FDA | Conjunto causal invariante (ICP) não vazio em 3 áreas | Passou na sétima rodada: 4 áreas; sem a Genética, frágil, restam Educação, Física e Farmacologia |
| H3 Malha contínua | 6 fluxos em replay sob relógio, 453 973 previsões; depois voos de 2023, prisões de 2020–2021 e paradas de trânsito em Rhode Island | Calibração fora do tempo, zero vazamento, controle de Goodhart | Passou na terceira tentativa do fluxo de prisões; habilidade baixa (+0,019) |

### H0 real: a central de chamadas

O Lab 0 supunha uma demanda que alterna entre 5 e 11 por hora. No H0 real, a demanda é observada: 164 dias úteis de uma central bancária norte-americana, contados a cada 5 minutos. A alegação é a prática padrão do setor: escalar cada meia hora pela fórmula de Erlang C sobre a taxa média do histórico garante 82% das chamadas atendidas em até 20 segundos, todo dia.

No mundo que a alegação descreve, nenhum dia fica abaixo da meta. Nas chegadas reais, cerca de um em cada quatro fica. A demanda não é Poisson: entre dias, o desvio é de cerca de 2 900 chamadas, onde Poisson preveria 180. E a desigualdade de Jensen aparece dentro da própria fórmula: aplicado à taxa real de cada dia, o Erlang C da alegação dá em média 57%, não 82%, porque o nível de serviço é côncavo na carga. A unidade independente, de novo, é o dia: o modelo nulo de metades deu desvio de 0,0474, contra 0,0470 previsto pelo dia e 0,0033 pela chamada.

![Dias abaixo da meta por regra de escala na central bancária (H0 real, h0-real/README.md)](figuras/central-regras-de-escala.png)

*Dias abaixo da meta por regra de escala na central bancária (H0 real, h0-real/README.md).*

A decisão (Φ4) compara regras de escala de custo parecido. Acrescentar 12 atendentes por igual leva a 7,4% de dias abaixo da meta; gastar a mesma folga onde está o risco, pelo dia da semana e por uma margem para a variação residual, leva a 3,7%. Ficam supostos o tempo de atendimento e a paciência dos clientes, que a base não traz; a paciência muda o tamanho do efeito (de 1% a 40% dos dias), mas nunca o sentido. O domínio de saúde ficou de fora porque os portais do DATASUS estavam fora do alcance da rede.

### H1 real: três alegações, três destinos

**Alavancar o Dow Jones.** A alegação: "alavancado 3 vezes, o índice rende 3 vezes mais, e quem segura por décadas sempre ganha". A soma aritmética daria cerca de 0,20 ao ano; o crescimento ao longo do tempo foi de 0,051 sem alavancagem e 0,060 com 3 vezes. Em janelas de 20 anos, 96% terminam acima do ponto de partida sem alavancagem e 64% com 3 vezes. Com 3 vezes, a riqueza média chega a 19 vezes o capital, mas a mediana fica em 2,1: a média de conjunto e a trajetória típica se separam, como a Crononomia prevê. Só o 19 de outubro de 1987 tira 68% do capital alavancado. A refutação vale para "rende 3 vezes" e "sempre ganha", não para "alavancar nunca compensa".

**O experimento de LaLonde.** Comparar participantes de um programa de treinamento com trabalhadores de uma pesquisa nacional dá um efeito negativo no emprego (−10,7 pontos). O experimento sorteado mostra +11,1. O ajuste por escore de propensão chega a +9,6, perto do valor certo, mas o teste severo mostra que acertou sem merecer: um tratamento inócuo daria +9,0 dentro da mesma faixa. Um placebo, usando o emprego de 1975 como desfecho, revela que a seleção ainda está lá.

**Os incêndios dinamarqueses.** Uma resseguradora vê 36 meses sem sinistro numa faixa alta de perdas e conclui que o risco é desprezível. A janela limpa vale apenas 2,2 bits de evidência, e a regra de sucessão de Laplace dá 1 chance em 38 por mês, não zero. Com a alavancagem que a alegação justificaria, a resseguradora é arruinada em agosto de 1989. Mas o orquestrador reprovou o projeto: no cofre, nenhum previsor venceu a frequência histórica simples, porque os sinistros cresciam 4,6% ao ano. A alegação perde, mas o modelo não ganha o direito de decidir.

### H2: o portão que custou sete rodadas

O H2 testou se o método transfere entre áreas. Doze domínios reais (educação, logística, agronomia, clima, crédito, saúde, direito, entre outros) foram plugados pelo mesmo pacote padronizado; dez desses pacotes foram escritos por agentes que só leram o contrato e dois exemplos. O portão escolhido pelo plano era exigente: a Predição Causal Invariante (ICP), que procura as causas cuja relação com o resultado não muda de um ambiente para outro, deveria encontrar um conjunto não vazio em pelo menos três áreas distintas.

Na primeira rodada, encontrou em uma, o direito. Nos outros domínios, ou rejeitou todos os conjuntos, porque os ambientes de conveniência (ano, região, aeroporto) agem direto no resultado, ou aceitou tantos que a interseção ficou vazia. O diagnóstico foi que o ICP padrão, com amostras grandes, rejeita qualquer desvio do modelo linear gaussiano. Seguiram-se seis rodadas, cada uma com a mudança declarada e registrada com hash antes de rodar: bases em que o ambiente é uma intervenção documentada, um teste de invariância não paramétrico, uma regra que descarta conjuntos sem poder de teste, câmaras físicas com o grafo causal conhecido, deleções de genes em levedura e, por fim, um ensaio clínico de cardiologia.

O momento mais limpo foi o túnel de luz. Numa câmara física construída para estudar causalidade, o ICP recuperou exatamente os pais verdadeiros da corrente elétrica, os três LEDs, e deixou de fora sensores fortemente correlacionados com ela. Foi a primeira vez no projeto que o método foi conferido contra uma verdade conhecida, e acertou. O túnel de vento, na mesma bateria, rejeitou até os pais verdadeiros, provavelmente por deriva atmosférica e por um ajuste aditivo onde a física é quadrática.

**A ressalva da Genética, e o que veio depois.** A sexta rodada chegou às três áreas exigidas com Educação (turmas sorteadas), Física (túnel de luz) e Genética (deleções de genes em levedura, Kemmeren et al., 2014). Mas a Genética era frágil: vinha de um único gene entre dez sorteados, num ICP que rejeitou 1 022 dos 1 024 conjuntos, com o melhor p = 0,007, logo acima do limiar de 0,005; e a validação por intervenção contradiz dois dos três genes do conjunto. sil aprovou a saída do H2 com essa ressalva registrada.

A sétima rodada tirou o peso da ressalva. Num ensaio do FDA em que voluntários receberam quatro drogas, o ICP apontou como causas invariantes do alongamento do intervalo QT exatamente as concentrações das três drogas que bloqueiam o canal hERG, que é o que a farmacologia esperava, e deixou de fora medidas do próprio eletrocardiograma. No verapamil, a droga que o modelo nunca viu, o modelo invariante previu bem (habilidade +0,57), enquanto o modelo com todas as variáveis explodiu, porque o verapamil empurra o intervalo PR para fora de tudo o que havia no treino. É o exemplo de livro do que a invariância protege. Sem a Genética, restam três áreas, Educação, Física e Farmacologia, duas delas conferidas contra uma verdade externa. Fica uma ressalva: o teste supõe medidas independentes, e o ensaio tem 16 medidas por voluntário.

Há uma lição positiva escondida na falha: **o efeito transporta melhor que o conjunto**. Mesmo com o ICP rejeitado, o modelo previu bem o ambiente nunca visto em 9 domínios. E a pseudorreplicação apareceu em toda parte: nos voos de Nova York, 225 mil voos valem, em informação independente, o mesmo que 364 dias.

### H3: a malha contínua

O H3 rodou seis fluxos reais (energia, a central bancária, a bolsa, voos, ciclones e prisões) como se fossem ao vivo, sob um relógio que só libera cada dado no instante em que ele se tornaria conhecível. Os três critérios acabaram passando, o último na terceira tentativa.

- **Zero vazamento temporal.** Em 453 973 previsões, nenhuma usou informação do futuro. Um canário comprovou que o teste funcionaria: uma coluna idêntica ao rótulo, publicada no instante do rótulo, não deu ganho nenhum com o relógio honesto, e deu ganhos de +0,86 a +0,92 com o relógio desligado.
- **Goodhart sob controle.** Na central, quem não é atendido liga de novo, e a previsão passa a medir o próprio efeito. Um previsor ingênuo acerta o que observa com 1% de erro e, ainda assim, superestima a demanda verdadeira em 29%: a métrica aprova um modelo errado sobre o mundo. Usando choques sorteados na escala como instrumento, o erro cai para 1,7%. Esse laço, porém, é declarado sobre chegadas reais, não medido, e o estimador teve quatro versões antes de passar.
- **Calibração fora do tempo: passou na terceira tentativa.** Nos voos, os atrasos do último trimestre ficaram 2,7 pontos acima do previsto, para uma tolerância de 2. Nas prisões, o erro de calibração foi 0,036, para uma tolerância de 0,03, com só 1 488 casos. A lição de desenho é precisa: o disjuntor comparava o modelo com o reserva, não com a calibração, e por isso não viu a deriva da média. Com uma recalibração contínua do intercepto, declarada antes, os voos passaram num cofre novo, o mesmo trimestre em 2023 (ECE 0,008). As prisões não: com dados de Toronto de 2020 e 2021, o modelo acerta a média, mas é pouco confiante nos extremos (ECE 0,067), um erro de inclinação que a correção do intercepto não alcança. A correção, uma recalibração contínua que ajusta também a inclinação, entrou como especificação nova e foi testada num cofre que ninguém tinha visto: 24 466 paradas de trânsito em Rhode Island, de 2013 a 2015. Passou com folga (ECE 0,0048), com duas ressalvas: a pergunta mudou, de liberação depois da prisão para prisão numa parada, e a habilidade é pequena (+0,019), porque o evento é raro. A calibração é boa; a resolução, não. E, num modelo cego à cor, a taxa de falsos positivos contra motoristas negros foi quase o dobro da dos brancos em 2013 e 2014.

O que as falhas têm em comum é que foram previstas como possibilidade e registradas como resultado. O sistema declarou quantas especificações tentou em cada etapa e manteve as prioris "de literatura" marcadas como não conferidas, já que nenhum especialista humano foi consultado. Os 21 pareceres de risco foram lidos e assinados por sil, quatro deles sobre decisões que envolvem pessoas, e a escala ficou abaixo da meta: o hipergrafo do H3 tem 454 178 nós, 45% do milhão previsto.

## 7. Novos horizontes

Se as velhas fronteiras separavam disciplinas, os novos horizontes da Estocologia nascem justamente de atravessá-las. Cinco deles parecem os mais promissores.

**Do argumento ao artefato.** A ciência publica alegações em prosa e, quando muito, tabelas. A Estocologia propõe que a unidade de conhecimento seja um artefato executável: uma alegação que vem com o seu processo gerador, o seu modelo nulo e o seu teste. Isso muda o que significa discordar. Em vez de duas narrativas, temos dois programas, e a divergência se localiza numa premissa que pode ser alterada e rodada de novo.

**Agentes artificiais com disciplina epistêmica.** Modelos de linguagem escrevem com fluidez e erram com confiança; são, num sentido preciso, agentes da camada L3 com atalhos próprios. A arquitetura não tenta torná-los infalíveis. Cerca-os de portões: eles não fazem contas de cabeça, não conversam em linguagem livre, não aprovam o próprio trabalho. A pergunta deixa de ser "a IA está certa?" e passa a ser "o que a IA afirmou sobreviveu a quais testes?".

**O tempo e a ruína.** A Crononomia leva a sério uma ideia que a economia clássica deixou de lado: a média entre muitas pessoas não é a média ao longo da vida de uma. Num jogo multiplicativo, o valor esperado pode crescer enquanto quase todas as trajetórias individuais encolhem. Para quem decide uma única vez, com um único patrimônio, a pergunta certa é a taxa de crescimento no tempo, e evitar a ruína vem antes de maximizar o esperado. No Dow Jones, isso deixou de ser teoria: com alavancagem de 3 vezes, a riqueza média chegou a 19 vezes o capital inicial, e a mediana, a 2,1 (seção 6).

**A previsão que muda o que prevê.** Quando um modelo é publicado, as pessoas reagem a ele. Uma previsão de fila longa faz pacientes desistirem; uma métrica vira meta e deixa de medir (lei de Goodhart). A arquitetura exige rodar o modelo com e sem a ação, para separar o que se aprendeu sobre o mundo do que se deformou nele. No H3, um previsor que acertava o que observava com 1% de erro superestimava a demanda real em 29%; foi preciso um pequeno experimento embutido na operação para desfazer o laço.

**A comunicação como parte da ciência.** Um resultado mal comunicado é um resultado que falhou na camada do Agente. Por isso a saída do sistema não é um p-valor, mas frequências naturais ("25 em cada 100 pacientes esperam mais de meia hora"), visualizações que mostram resultados possíveis em vez de médias, e uma cláusula explícita que diz quando a recomendação deixa de valer.

Os quatro horizontes do plano já foram percorridos, e a seção 6 mostra o que passou e o que não passou. Os próximos passos saem das ressalvas: testes de invariância que respeitem medidas repetidas da mesma pessoa; previsores de eventos raros com mais resolução, e não só boa calibração; a auditoria de quem paga pelo erro, que nos dois fluxos de prisões mostrou o mesmo desequilíbrio; e dados reais de saúde, para fechar o círculo que o Lab 0 abriu.

## 8. Limites e objeções

Uma disciplina que exige teste severo das alegações alheias precisa aplicá-lo a si mesma. Quatro objeções merecem resposta direta.

**"Isso é só estatística com outro nome."** Em parte, sim: quase todas as ferramentas têm dono há décadas. O que a Estocologia acrescenta é a organização: a separação em camadas, a tipagem da incerteza e a exigência de que cada alegação passe pelos mesmos portões. A hipótese que o programa precisa demonstrar, e ainda não demonstrou, é que essa organização pega erros que a prática estatística comum deixa passar.

**"Os dados reais são antigos e de bases públicas."** É verdade, e foi escolha: chamadas de 2003, empregos de 1978, incêndios de 1980 a 1990. São bases conhecidas, verificáveis por hash e com desfecho já sabido, o que permite auditar o método, mas não substitui uma operação de verdade. Várias peças continuam supostas: prioris de literatura citadas de memória, sem especialista consultado; tempos de atendimento e custos de decisão; o laço performativo do H3. E os dois últimos portões só passaram depois de várias rodadas, com mudanças declaradas no caminho. A arquitetura registra tudo isso como pendência declarada, que é exatamente o que ela exige de qualquer alegação.

**"Formalizar o julgamento é eliminar o julgamento."** A arquitetura não decide o que vale a pena perguntar, nem quanto um erro custa para quem o sofre. Esses valores entram pela elicitação e pela razão custo/perda, que são declarados, não calculados. A ambição é tornar o julgamento visível e contável, não substituí-lo; por isso a definição diz "ciência (ou arte)".

**"O aparato é pesado demais."** Oito agentes, cinco lentes e quatro invariantes são muito para uma pergunta simples. O plano reconhece o risco e responde com o funil: a maior parte das hipóteses deve ser descartada barato, antes de qualquer simulação cara. Se o custo de usar a máquina for maior que o valor das decisões que ela melhora, a própria Crisonomia mandaria desligar a máquina.

## 9. Conclusão

A Estocologia propõe uma mudança de pergunta. Em vez de "qual é a probabilidade?", pergunta: em que camada esta afirmação pode falhar, que tipo de incerteza a ameaça, e que teste ela teria de enfrentar para merecer confiança? As velhas fronteiras (entre filosofia, estatística e dinâmica; entre risco e incerteza; entre a norma e o comportamento) não desaparecem. Viram costuras, e é nas costuras que a disciplina trabalha.

Os dados reais confirmaram essa lição e acrescentaram outra, menos confortável. A média escondeu 26 dias ruins em cada 100 numa central bancária, e a trajetória típica de um investidor alavancado ficou muito abaixo da média de todos. Mas o programa precisou de sete rodadas para passar um dos seus próprios portões e de três tentativas para passar outro, e uma resseguradora imprudente perdeu sem que o modelo rival ganhasse o direito de decidir. Uma disciplina sobre incerteza se mede também pelo que admite não ter conseguido.

O posto de saúde simulado resume a lição em uma frase: a mesma média pode esconder 9 ou 21 minutos de espera, e só descobre a diferença quem simula o mundo em vez de calcular o caso médio. Bernoulli chamou isso de arte de conjecturar. Três séculos depois, com código reprodutível, modelos nulos e agentes que não aprovam o próprio trabalho, essa arte pode se tornar também uma ciência auditável, desde que continue humilde sobre o que ainda não sabe.

## Referências e leituras

**Documentos do projeto**

- [Arquitetura](https://docs.google.com/document/d/1l0YQHrV_UrKisAjQcz2BqT9KpwhcT0s0WVoXm_muxtE/edit): hipergrafo tipado, agentes F1 a F8, ciclo de seis estágios.
- [PLAN.md](https://docs.google.com/document/d/1hXvDuJyckooL1_qx9gspkSkBqgjyhAUVQvQ3F-Crinc/edit): horizontes H0 a H3 e funil de pesquisa.
- [AGENTS.md](https://docs.google.com/document/d/1hniAsjLqgNVGqR1wZ1845GVkS5lZTEjhlH7MaI3U2Xk/edit): contrato dos agentes e invariantes Φ1 a Φ4.
- [Estocologia: Fundamentação, Três Camadas e Inferência sob Incerteza](https://docs.google.com/document/d/1L5ittj7-RiSacEvNrkR4IS6yrQUyhusFpcDIWazINLI/edit): camadas, taxonomia da incerteza, módulos M0 a M5.
- Resultados: repositório do projeto, 0b3st4f3r4/estocologia (h0-lab0, h0-real, h1-real, h2 a h2g e h3, cada uma com README.md e saídas); bases reais com SHA-256 em [dados-reais/FONTES.md](../dados-reais/FONTES.md).

**Bases e métodos usados na pesquisa** (conferidos em fonte autoritativa; ver [referencias/CONFERENCIA.md](../referencias/CONFERENCIA.md))

- Shen, H.; Huang, J. Z. Forecasting time series of inhomogeneous Poisson processes with application to call center workforce management. *Annals of Applied Statistics*, 2(2):601–623, 2008.
- LaLonde, R. J. Evaluating the econometric evaluations of training programs with experimental data. *American Economic Review*, 76(4):604–620, 1986.
- McNeil, A. J. Estimating the tails of loss severity distributions using extreme value theory. *ASTIN Bulletin*, 27(1):117–137, 1997.
- Peters, J.; Bühlmann, P.; Meinshausen, N. Causal inference by using invariant prediction: identification and confidence intervals. *JRSS B*, 78(5):947–1012, 2016.
- Heinze-Deml, C.; Peters, J.; Meinshausen, N. Invariant causal prediction for nonlinear models. *Journal of Causal Inference*, 6(2), 2018.
- Meyer, B. D.; Viscusi, W. K.; Durbin, D. L. Workers' compensation and injury duration: evidence from a natural experiment. *American Economic Review*, 85(3):322–340, 1995.
- Gamella, J. L.; Peters, J.; Bühlmann, P. Causal chambers as a real-world physical testbed for AI methodology. *Nature Machine Intelligence*, 7:107–118, 2025.
- Kemmeren, P. et al. Large-scale genetic perturbations reveal regulatory networks and an abundance of gene-specific repressors. *Cell*, 157(3):740–752, 2014.
- Johannesen, L. et al. Differentiating drug-induced multichannel block on the electrocardiogram: randomized study of dofetilide, quinidine, ranolazine, and verapamil. *Clinical Pharmacology & Therapeutics*, 96(5):549–558, 2014.
- Gibbs, I.; Candès, E. Adaptive conformal inference under distribution shift. *NeurIPS 34*, 2021.

**Obras clássicas citadas** (referências de memória, sem conferência bibliográfica nesta versão)

- Bernoulli, J. *Ars Conjectandi*, 1713.
- Hume, D. *An Enquiry Concerning Human Understanding*, 1748.
- Bayes, T. "An Essay towards Solving a Problem in the Doctrine of Chances", 1763.
- Knight, F. *Risk, Uncertainty and Profit*, 1921.
- Kolmogorov, A. *Grundbegriffe der Wahrscheinlichkeitsrechnung*, 1933.
- Popper, K. *Logik der Forschung*, 1934.
- Savage, L. J. *The Foundations of Statistics*, 1954.
- Simon, H. "A Behavioral Model of Rational Choice", 1955.
- Tversky, A.; Kahneman, D. "Judgment under Uncertainty: Heuristics and Biases", *Science*, 1974.
- Mayo, D. *Error and the Growth of Experimental Knowledge*, 1996.
- Pearl, J. *Causality*, 2000.
- Gigerenzer, G. *Calculated Risks*, 2002.
- Meng, X.-L. "Statistical Paradises and Paradoxes in Big Data", 2018.
