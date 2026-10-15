# Estocologia: livro didático

Oct 15, 2026 · @Silvano Neto

## Prefácio: como usar este livro

Este livro ensina a Estocologia de ponta a ponta: o que ela é, com que conceitos trabalha, como se opera numa arquitetura auditável e como essa arquitetura se comporta quando é posta à prova em dados reais, do horizonte H0 ao H3. A pesquisa que o sustenta foi encerrada em 15 de outubro de 2026, e todo número citado aqui vem da literatura indicada no texto, dos roteiros e laboratórios do curso *Pensamento Crítico e Modelos Estocásticos* ou das saídas do repositório 0b3st4f3r4/estocologia.

**Para quem é.** O livro serve a três leitores. O estudante de graduação ou pós que já viu probabilidade básica e quer aprender a raciocinar sob incerteza. O profissional de dados que quer saber por que um modelo que acerta no teste falha em produção. E o decisor que precisa ler um número e saber o que ele autoriza concluir.

**O que se supõe.** Probabilidade elementar (eventos, probabilidade condicional, esperança e variância), um pouco de cálculo e familiaridade com uma linguagem de programação. Os laboratórios estão em Python com semente fixa. Quem não programa pode ler os capítulos e pular os exercícios marcados como laboratório.

**Como o livro está organizado.** São quatro partes, e cada uma depende da anterior.

| Parte | Capítulos | Pergunta que responde |
| --- | --- | --- |
| I. Fundamentos | 1 a 5 | Como se pensa sob incerteza, e onde o pensamento costuma falhar? |
| II. Arquitetura | 6 a 9 | Como transformar esse pensamento numa máquina que se audita? |
| III. A pesquisa real | 10 a 13 | Como a máquina enfrenta dados reais, horizonte por horizonte? |
| IV. Balanço | 14 e apêndices | O que se aprende, o que fica em aberto e como continuar? |

**Trilhas de leitura.** Um curso de um semestre lê o livro inteiro, na ordem. Um seminário de quatro semanas pode ler os capítulos 1, 2, 4, 6 e 10. Quem só quer a arquitetura lê os capítulos 1, 6, 7, 8 e 9. Quem quer ver o método em ação vai direto à Parte III e volta aos fundamentos quando precisar.

**Como cada capítulo funciona.** Cada capítulo abre com os objetivos de aprendizagem, desenvolve as ideias com um caso trabalhado e fecha com um resumo e exercícios. Os exercícios têm três tipos: de conceito (responder com palavras), de conta (responder com um número) e de laboratório (responder com código que outra pessoa possa rodar e obter o mesmo resultado).

**Uma regra que vale para o livro inteiro.** A Estocologia exige que toda alegação declare onde pode falhar. Este livro também declara as suas falhas, e depois as testa: a seção 9.7 recolhe dez pontos de fundo que a auditoria do próprio programa levantou — da tese central, que nunca havia sido posta à prova, ao laço performativo que o H3 declarou sem medir — e diz o que cada um respondeu, com os números. As prioris que não foram conferidas com especialistas, os dados antigos e públicos e os portões que só passaram depois de várias rodadas continuam no texto, não escondidos em nota de rodapé.

# Parte I. Fundamentos

## Capítulo 1. O que é a Estocologia

**Objetivos.** Ao fim deste capítulo, você deve saber enunciar a definição da Estocologia, distinguir Mundo, Modelo e Agente, e explicar por que uma falácia é, quase sempre, um desalinhamento entre camadas e não um erro de conta.

### 1.1 A definição

**Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.** Toda decisão que importa é tomada antes de sabermos o que vai acontecer. O médico prescreve sem conhecer a resposta daquele paciente, o juiz condena sem ter visto o crime, o gestor contrata pessoal para uma demanda que ainda não chegou. Em todos esses casos alguém transforma informação incompleta numa ação concreta e responde pelas consequências.

O nome vem do grego *stókhos*, o alvo para onde se mira, a conjectura bem orientada. Jacob Bernoulli, na obra publicada postumamente em 1713, *Ars Conjectandi*, a arte de conjecturar, chamou essa arte de *stochastice*. Com o tempo, "estocástico" virou sinônimo de "aleatório" e perdeu o sentido de mirar bem quando não se vê o alvo. A Estocologia quer recuperar esse sentido.

O "ou arte" é deliberado. Julgar sob incerteza tem uma parte que se formaliza, como a coerência das crenças, a atualização pela evidência e a margem de erro. E tem uma parte que não se formaliza por completo: decidir qual modelo vale a pena levar a sério, quando parar de coletar dados, quem paga se o erro acontecer. Uma disciplina honesta sobre incerteza precisa ser honesta sobre os limites do próprio formalismo.

### 1.2 Uma costura, não um ramo novo

A Estocologia não é um ramo novo da estatística. É uma costura entre três tradições que cresceram separadas e raramente se citam.

- **A filosofia da indução.** Hume mostrou que toda defesa da indução pressupõe que o futuro se parecerá com o passado. Popper trocou a confirmação pela refutação; Lakatos mostrou como as teorias reagem quando a refutação chega, como se verá logo adiante.
- **A inferência estatística.** Pascal e Fermat, Bernoulli, Bayes, Laplace e Kolmogorov construíram a probabilidade; Fisher, Neyman e a escola bayesiana construíram a inferência.
- **A mecânica dos processos estocásticos.** Markov e Langevin estudaram como o acaso se acumula no tempo.

Duas classificações da filosofia atravessam o livro inteiro.

**As três formas de inferência (Peirce).** Peirce obteve dedução, indução e abdução permutando as mesmas três proposições: regra, caso e resultado.

| Forma | Vai de | Para | Preserva a verdade? |
| --- | --- | --- | --- |
| Dedução | Regra e caso | Resultado | Sim |
| Indução | Caso e resultado | Regra | Não: amplia e pode falhar |
| Abdução | Regra e resultado | Caso (a melhor explicação) | Não: amplia e pode falhar |

A abdução escolhe a hipótese que torna os dados menos surpreendentes, mas só entre as hipóteses consideradas. É a precursora da máxima verossimilhança e herda o mesmo limite, que o Capítulo 2 chamará de incerteza knightiana.

**Os ajustes de Lakatos.** Duhem e Quine lembraram que uma previsão frustrada refuta a teoria junto com suas hipóteses auxiliares, nunca sozinha. Lakatos separou dois modos de salvar a teoria. O ajuste **progressivo** prevê algo novo que depois se confirma: em 1846, a anomalia na órbita de Urano não derrubou Newton, porque Le Verrier postulou um planeta oculto e Netuno foi achado. O ajuste **degenerativo** só protege a teoria: a mesma manobra com o planeta Vulcano, para explicar Mercúrio, fracassou. Toda mudança de especificação depois de uma falha (Capítulos 9 e 12) precisa responder a essa pergunta.

Cada uma das três tradições acerta sobre o seu pedaço e erra ao generalizar. A aposta da Estocologia é que os erros vivem justamente nas costuras. A disciplina não busca teoremas novos: usa os existentes para julgar argumentos e pressupostos, e pergunta o que um modelo autoriza concluir, não como otimizá-lo.

### 1.3 Três camadas

O primeiro gesto da Estocologia é separar três coisas que o senso comum mistura.

| Camada | Código | O que é | O que se audita nela |
| --- | --- | --- | --- |
| Mundo | L1 | A realidade que gera os dados e que nunca vemos inteira | Física do ruído, mecanismos causais, dependência de trajetória, caudas pesadas |
| Modelo | L2 | A representação formal, simplificada e simulável que construímos para aproximá-lo | Premissas ocultas, espaço de hipóteses, verossimilhança, sobreajuste |
| Agente | L3 | Quem decide: pessoa, instituição ou algoritmo, com tempo, atenção e computação limitados | Atalhos heurísticos, calibração, utilidade, laços em que a ação muda o Mundo |

As três camadas formam um laço. O Mundo gera dados, o Modelo os interpreta, o Agente age, e a ação volta a mudar o Mundo.

&#91;embedded content: as três camadas e o laço da ação\]

Cada seta é uma ponte em que o erro pode entrar: dados filtrados entre Mundo e Modelo, previsão mal lida entre Modelo e Agente, e ação que deforma o Mundo que se queria medir.

### 1.4 A tese central: a falácia mora na ponte

A separação tem uma consequência forte: **uma falácia, quase sempre, não é um erro de aritmética, é um desalinhamento entre camadas.** Dois exemplos curtos e um caso trabalhado.

- **A falácia do promotor.** O Agente lê a verossimilhança do Modelo, P(evidência | inocente), como se fosse a resposta sobre o Mundo, P(inocente | evidência).
- **O paradoxo da inspeção.** O Agente impõe um Modelo estável a um Mundo que oscila.

**Caso trabalhado: o posto de saúde.** A alegação diz: "o posto recebe em média 8 pacientes por hora e atende 12; a espera média é de 10 minutos, então não há por que reforçar o atendimento". A conta da fila com demanda constante está certa. Mas no Mundo do Lab 0 do curso a demanda oscila: metade dos dias tem 5 chegadas por hora e metade tem 11, com a mesma média de 8. Simulados com a mesma semente, os dois mundos dão esperas muito diferentes. A espera média por paciente vai de 9,3 minutos no mundo da alegação para 20,7 no mundo que oscila, e os pacientes que esperam mais de 30 minutos vão de 8 para 25 em cada 100 (h0-lab0/README.md). Nenhuma conta errou. O Modelo supôs um Mundo estável, e o Agente decidiu pelo caso médio. A razão é que a espera cresce de forma convexa com a demanda, a desigualdade de Jensen do Capítulo 2; o Capítulo 10 refaz o caso por inteiro.

**A tese foi testada uma vez.** Doze casos foram montados a partir de uma lista externa de dez erros estatísticos comuns (Makin e Orban de Xivry, 2019): oito com um defeito plantado, quatro limpos, analisados às cegas por dois braços independentes — um com a arquitetura, outro com a instrução de analisar com rigor, sem agentes nem portões. Os dois apontaram os 8 defeitos e deram zero alarme falso nos 4 casos limpos. A regra fixada de antemão dizia que um empate no teto não sustenta a tese, e foi isso que o resultado disse. Ele também não a refuta: mostra que, para os dez erros dessa lista, um analista competente acha tudo o que o outro acha. Se a tese vale, vale para erros que a lista não cobre — e um teste justo disso precisa de casos mais difíceis, escolhidos fora do programa, de analistas humanos e de um juiz humano. A seção 9.7 volta a esse ponto.

Nos três casos as contas podem estar certas. O que está errado é a ponte entre camadas. A tese tem uma versão operacional: a maior parte dos vieses do catálogo clássico corresponde a um modelo ignorado.

| Viés | Modelo de acaso ignorado |
| --- | --- |
| Negligência da taxa-base | A verossimilhança tomada pela posterior |
| Disponibilidade, sobrevivência | Uma amostra filtrada tomada pela população |
| Ilusão de padrão | A hipótese nula de acaso esquecida |
| Falácia do apostador | Memória atribuída a um processo sem memória |
| Negligência de eventos raros | Uma cauda pesada modelada como gaussiana |
| Excesso de confiança | Má calibração |
| Raciocínio motivado | O modelo escolhido pelo resultado desejado |

A palavra "viés" ganha três sentidos que a disciplina separa para depois reunir: o viés estatístico de um estimador, o viés do processo que produziu os dados e o viés cognitivo de quem raciocina. Nos três, o erro não é ruído que se cancela com mais dados; é uma distorção que se acumula.

### 1.5 A pergunta que muda

Em vez de perguntar "qual é a probabilidade?", a Estocologia pergunta três coisas. Em que camada esta afirmação pode falhar? Que tipo de incerteza a ameaça? Que teste ela teria de enfrentar para merecer confiança? O resto do livro é a resposta a essas três perguntas.

### Resumo

- Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.
- Ela costura filosofia da indução, inferência estatística e processos estocásticos.
- Mundo, Modelo e Agente são camadas distintas; a maior parte dos erros mora nas pontes entre elas.

### Exercícios

1. *(Conceito)* Explique com suas palavras por que a definição diz "ciência (ou arte)". Dê um exemplo de decisão em que a parte "arte" é inevitável.
2. *(Conceito)* Uma notícia diz: "a chance de o DNA coincidir por acaso é minúscula, logo a chance de o réu ser inocente é igualmente minúscula". Em que ponte, entre que camadas, está o erro? Que quantidade falta para chegar a P(inocente | evidência)?
3. *(Conceito)* Escolha três vieses da tabela da seção 1.4 e descreva, para cada um, um ambiente em que o atalho deixaria de ser erro.
4. *(Conceito)* Classifique cada item como Mundo, Modelo ou Agente: a taxa real de chegada de pacientes; a fórmula de fila usada pelo gestor; a decisão de contratar dois enfermeiros; a desistência de pacientes quando veem a fila.

## Capítulo 2. Probabilidade como linguagem do julgamento

**Objetivos.** Justificar por que graus de crença devem obedecer às regras da probabilidade; separar coerência de calibração; aplicar Bayes em chances e em frequências naturais; distinguir incerteza aleatória, epistêmica e knightiana; e usar a desigualdade de Jensen para ver quando o caso médio engana.

### 2.1 Por que a crença obedece à probabilidade

Antes de justificar as regras, convém saber de que "probabilidade" se fala. Há cinco leituras clássicas, e cada uma responde a uma pergunta diferente sobre o mesmo número.

| Interpretação | A probabilidade é | Camada em que mora |
| --- | --- | --- |
| Clássica (Laplace) | Casos favoráveis sobre casos igualmente possíveis | Modelo |
| Frequentista | O limite da frequência relativa numa série longa | Mundo |
| Propensão (Popper) | Uma disposição física do arranjo experimental | Mundo |
| Lógica (Keynes, Carnap) | Um grau de apoio parcial entre proposições | Modelo |
| Subjetiva (Ramsey, de Finetti) | O grau de crença de um agente, medido pelo preço de uma aposta | Agente |

A Estocologia não escolhe uma: tipa cada número pela camada de onde veio. Muitos erros do Capítulo 1 são trocas de leitura, como tomar a crença de um agente por uma frequência do Mundo.

Por que graus de crença devem obedecer às regras da probabilidade? Dois argumentos independentes respondem. Cox (1946) mostrou que qualquer medida de plausibilidade consistente, que acompanhe o senso comum, obedece, depois de uma reescala, às regras da soma e do produto. Ramsey e de Finetti mediram a crença pelo preço de uma aposta. Quem declara 60% para chuva e 50% para não chuva aceita pagar 1,10 por dois bilhetes dos quais só um paga 1, e perde 0,10 com certeza. Um conjunto de preços admite esse "livro holandês" se e somente se violar os axiomas.

**Coerência é barata.** No laboratório do curso, o agente que comete a falácia da conjunção perde 0,05 por rodada para o apostador holandês e 0,60 para quem conhece o mundo. Torná-lo coerente reduz a perda só para 0,55. O erro mais visível não era o mais caro: quem pune o desacordo com o Mundo é a calibração, não a coerência.

**Máxima entropia.** Quando a informação é incompleta, Jaynes (1957) manda escolher, entre as distribuições compatíveis com o que se sabe, a de maior entropia. Um dado de seis faces com média 4,5 recebe então cerca de 5% na face 1 e 35% na face 6; qualquer outra escolha supõe algo que não foi declarado.

### 2.2 Bayes em chances

Escrita em chances, a regra de Bayes separa o que a evidência diz do que já se sabia:

```latex
\frac{P(H\mid E)}{P(\neg H\mid E)}=\frac{P(E\mid H)}{P(E\mid \neg H)}\times\frac{P(H)}{P(\neg H)}
```

Chance posterior = razão de verossimilhança × chance anterior. Uma evidência é forte quando a razão está longe de 1, não quando P(E | H) é alta.

**Caso trabalhado: a mamografia.** A doença atinge 1% das mulheres, o exame detecta 90% dos casos e dá positivo em 9% das saudáveis. Em frequências naturais a conta fica visível.

1. De 10 000 mulheres, 100 estão doentes; 90 delas dão positivo.
2. Das 9 900 saudáveis, 891 também dão positivo.
3. Das 981 positivas, só 90 estão doentes: cerca de uma em dez.

Quando Gigerenzer e colegas fizeram essa pergunta a 160 ginecologistas, só 21% escolheram "cerca de 1 em 10". A mesma troca levou ao tribunal o "1 em 73 milhões" do caso Sally Clark, obtido multiplicando 1/8 543 por si mesmo, como se as duas mortes fossem independentes, e lido como a chance de inocência.

### 2.3 A força da evidência

Turing e Good mediram a força da evidência em decibans, dez vezes o logaritmo decimal da razão de verossimilhança. Nessa escala, evidências independentes se somam. A independência é a condição que mais se esquece: dois exames positivos com uma causa comum de erro dão 51% pela soma ingênua e 30% na população simulada. O exemplo é a Parte 1 do Lab 3 do curso, com os números da mamografia acima: numa população em que o tecido denso produz falso positivo nos dois exames, a razão efetiva do segundo cai de 10 para 4,2.

O valor esperado do peso da evidência é a divergência de Kullback-Leibler, que nunca é negativa:

```latex
D(P_1\,\|\,P_0)=\sum_x P_1(x)\log\frac{P_1(x)}{P_0(x)}\ \ge 0
```

Ela diz quantas observações bastam. Para separar uma moeda de 60% de uma honesta com chances de 100 para 1, são precisos cerca de 229 lances. E a evidência engana pouco: Royall mostrou que, se a hipótese nula é verdadeira, a chance de a razão de verossimilhança alguma vez chegar a k é no máximo 1/k, qualquer que seja o tamanho da amostra.

```latex
P_{H_0}\Big(\sup_n \mathrm{RV}_n \ge k\Big)\le \frac{1}{k}
```

Em simulação, só 0,85% dos caminhos sob a nula chegaram a 20 decibans (razão 100), contra um limite de 1%.

**As três perguntas de Royall.** Royall separou três perguntas que costumam ser respondidas com o mesmo número. O que estes dados dizem sobre H contra H′? No que devo acreditar agora? O que devo fazer? A primeira é respondida pela razão de verossimilhança; a segunda exige uma priori; a terceira exige também custos e perdas (Capítulo 5). O p-valor não responde bem a nenhuma das três, e o promotor do Capítulo 1 respondeu à segunda com a resposta da primeira.

### 2.4 Três tipos de incerteza

Os três tipos de incerteza se distribuem pelas camadas, e confundi-los é um erro clássico. A lei da variância total separa os dois primeiros com precisão:

```latex
\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{aleatória (Mundo)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistêmica (Modelo)}}
```

| Incerteza | Camada | O que mais dados fazem | Como se mitiga |
| --- | --- | --- | --- |
| Aleatória | Mundo | Nada | Margens de segurança e redundância |
| Epistêmica | Modelo | Reduzem até zero, se o parâmetro for identificável (Capítulo 8) | Amostragem e atualização bayesiana |
| Knightiana | Modelo e Agente | Nada, ou pior: falsa confiança | Cenários, robustez, estresse de premissas; ampliar o espaço de hipóteses |

Frank Knight (1921) separou o risco, de probabilidades conhecidas, da incerteza, de probabilidades que não se podem medir. No caso extremo, que este livro chama de incerteza knightiana ou erro de especificação, o processo real está fora do espaço de hipóteses. É a mais perigosa porque não aparece na equação. Aí mais dados estreitam o intervalo em volta da resposta errada.

**Duas classificações mais finas.** A divisão em três tipos basta para tipar um nó, mas duas tipologias da análise de risco ajudam a ver o que fazer em cada caso.

A **matriz de Stirling** cruza duas perguntas: sabemos quais desfechos são possíveis? E sabemos as probabilidades deles?

|  | Probabilidades conhecidas | Probabilidades desconhecidas |
| --- | --- | --- |
| **Desfechos bem definidos** | Risco: calcular | Incerteza: cenários, sensibilidade |
| **Desfechos mal definidos ou disputados** | Ambiguidade: deliberar sobre o que conta como dano | Ignorância: vigilância, reversibilidade, diversidade de opções |

O peru de Russell está no canto de baixo à direita, e nenhum cálculo do canto de cima à esquerda o salva.

A **tipologia de Walker e colegas (2003)**, usada em modelagem para políticas públicas, classifica cada incerteza em três dimensões. A *localização* diz onde ela entra: no contexto, na estrutura do modelo, nos parâmetros, nas entradas ou nos resultados. O *nível* vai da certeza determinística à ignorância total, passando pela incerteza estatística e pelos cenários. A *natureza* separa o que é variabilidade (aleatória) do que é falta de conhecimento (epistêmica). Na arquitetura, a localização corresponde à camada e ao campo da ficha, e o nível diz se basta uma margem ou se é preciso um teste de estresse.

**Caso trabalhado: o peru de Russell.** Russell contou a história com uma galinha; Taleb a recontou com o peru. Um peru alimentado pontualmente raciocina pela regra de sucessão de Laplace:

```latex
P(\text{sucesso amanhã}\mid s,n)=\frac{s+1}{n+2}
```

Na véspera do centésimo primeiro dia, está 99% confiante. Um processo que muda de regime numa data marcada nunca esteve no seu espaço de hipóteses. A regra também não é neutra, porque deriva de uma priori uniforme: com sete sucessos em dez, a uniforme dá 0,667, uma uniforme sobre θ² dá 0,692 e uma sobre √θ dá 0,652. Até a ignorância tem forma.

Quando o mundo muda devagar, o modelo fixo falha em silêncio. Se o viés de uma moeda sobe de 0,2 para 0,6 e o modelo supõe moeda fixa, o intervalo de 95% encolhe de 0,140 para 0,035 e cobre a verdade em só 11% do tempo. Esquecer o passado, com desconto de 0,99 por passo, eleva a cobertura a 99%, acima dos 95% nominais, ao preço de intervalos três vezes mais largos.

### 2.5 Jensen: o resultado médio não é o resultado do caso médio

A ferramenta que atravessa as três camadas é modesta. Para uma função convexa f:

```latex
\mathbb{E}[f(X)] \;\ge\; f(\mathbb{E}[X]), \qquad \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \tfrac{1}{2} f''(\mu)\,\operatorname{Var}(X)
```

Quando o sistema reage de forma não linear, a variabilidade do Mundo não se cancela na média: cobra um preço proporcional à variância. A troca do caso médio pelo resultado médio nem precisa de curvatura para morder. Numa prova com média 6,2 e nota mínima 6, o "aluno médio" passa, mas 45% reprovam. O Capítulo 10 mostra quanto essa conta custa num posto de saúde.

### 2.6 Norma e comportamento

Kahneman e Tversky catalogaram como as pessoas se afastam das regras da probabilidade; Simon e Gigerenzer responderam que um atalho só é erro relativo a um ambiente. A Estocologia propõe um critério em vez de um lado. Um desvio é **erro** quando a informação certa estava disponível e a regra de combinação foi violada. É **adaptação candidata** quando a norma supõe uma informação que o ambiente não fornece de modo confiável.

Num rastreamento com prevalência de 1%, quem ignora a taxa-base tem escore de Brier de 0,083, contra 0,009 de quem a usa. Num ambulatório de referência, onde a prevalência real é de 33%, quem insiste em 1% tem 0,28, contra 0,084 de quem a ignora. A mesma heurística é erro num ambiente e acerto no outro.

Acusar uma ilusão também exige um modelo nulo correto. Por três décadas, o estudo da "mão quente" de Gilovich, Vallone e Tversky (1985) foi o exemplo canônico de ilusão de padrão, até que Miller e Sanjurjo (2018) mostraram que o teste era viciado: em três lances de uma moeda honesta, a proporção média de cara depois de cara é 5/12, não 1/2. Corrigida, a diferença nos dados de Cornell passa de +3 para +13 pontos.

### Resumo

- Coerência evita o livro holandês; calibração evita perder para o Mundo, e custa mais.
- Bayes em chances separa a força da evidência (razão de verossimilhança) da crença anterior.
- Incerteza aleatória não encolhe, epistêmica encolhe, knightiana engana.
- Sob não linearidade, o caso médio não é o resultado médio.

### Exercícios

1. *(Conta)* Com os números da mamografia, calcule P(doente | positivo) pela regra das chances, sem frequências naturais, e confira que dá o mesmo que 90/981.
2. *(Conta)* Uma mulher dá positivo na segunda mamografia. Quantos decibans vale o segundo exame se ele for independente do primeiro, e quantos com a razão efetiva de 4,2 do Lab 3? Qual é a posterior no segundo caso?
3. *(Conceito)* Por que "1 em 73 milhões" cometeu dois erros diferentes? Nomeie a ponte ou a camada de cada um.
4. *(Laboratório)* Simule a regra de Laplace sobre uma moeda cujo viés sobe de 0,2 para 0,6 e reproduza a cobertura de cerca de 11%. Depois aplique desconto de 0,99 e meça a nova cobertura.
5. *(Conceito)* Dê um exemplo profissional seu em que f(E\[X\]) e E\[f(X)\] diferem, e diga o sinal da diferença.

## Capítulo 3. Dados, aprendizado e calibração

**Objetivos.** Reconhecer o filtro que existe antes de todo dado; calcular o tamanho efetivo de uma amostra; explicar por que um ajuste só é evidência se o modelo podia ter falhado; e decompor o escore de Brier em calibração e resolução.

### 3.1 O filtro antes do dado

Os dados quase nunca chegam crus: passaram por telas institucionais, mecânicas ou comportamentais. Meng (2018) mostrou que o erro de qualquer média amostral é o produto exato de três fatores, qualidade, quantidade e dificuldade:

```latex
\bar Y_n - \bar Y_N = \rho_{R,Y}\times\sqrt{\tfrac{1-f}{f}}\times\sigma_Y
```

Aqui ρ é a correlação entre entrar na amostra e a resposta, e f é a fração da população amostrada. Numa amostra sorteada, ρ é minúsculo, da ordem de 1/√N. Quando a entrada depende do que se mede, o tamanho não salva.

**Caso trabalhado: a *Literary Digest*.** Em 1936, a revista recebeu 2,38 milhões de cédulas e previu a vitória de Landon com 57%. Roosevelt teve 60,8%, e Gallup acertou o vencedor com cerca de 50 mil entrevistas. Numa simulação na escala de 1936, uma correlação de −0,21 basta para que 2,4 milhões de cédulas errem por 18 pontos e valham o mesmo que uma amostra aleatória de cerca de 7 pessoas. Mais dados enviesados dão mais certeza sobre a resposta errada.

**O filtro fabrica relações.** Berkson (1946) mostrou que duas doenças independentes aparecem associadas entre internados, porque cada uma aumenta a chance de internação. Olhar só os selecionados é condicionar num colisor. Numa simulação do curso, talento e simpatia têm correlação +0,002 na população e −0,669 entre os que passam de um limiar.

**O caso inverso: Simpson.** Charig e colegas (1986) compararam duas cirurgias de cálculo renal.

| Grupo | Cirurgia aberta | Outra técnica |
| --- | --- | --- |
| Cálculos pequenos | 93% | 87% |
| Cálculos grandes | 73% | 69% |
| Total bruto | 78% | 83% |
| Ponderado pela população inteira | 83,3% | 77,9% |

A cirurgia aberta vence nos dois estratos e perde no total, porque os casos difíceis iam para ela. Condicionar num confundidor fecha um caminho espúrio; condicionar num colisor abre um. "Controlar por tudo" não é neutro, e quem decide qual conta fazer é o grafo, não a tabela (Capítulo 4).

**O dado que falta também passou por um filtro.** Rubin (1976) classificou a ausência de dados pelo mecanismo que a produz, e a classe decide o que se pode fazer.

| Mecanismo | A chance de faltar depende de | Exemplo | O que se pode fazer |
| --- | --- | --- | --- |
| MCAR, completamente ao acaso | Nada que importe | Um formulário perdido no correio | Analisar os casos completos perde precisão, não introduz viés |
| MAR, ao acaso dado o observado | Variáveis que foram medidas | Jovens respondem menos, e a idade está na base | Ponderar ou imputar condicionando no observado |
| MNAR, não ao acaso | O próprio valor que falta | Quem ganha mais declara menos a renda | Nenhuma correção sai só dos dados; exige um modelo da ausência e análise de sensibilidade |

A distinção entre MAR e MNAR não pode ser testada com os dados observados: é uma premissa, e precisa ser declarada como tal. Na identidade de Meng, MNAR é o caso em que ρ continua diferente de zero mesmo depois de condicionar no que foi observado, porque a entrada na amostra depende do próprio Y. O valor sentinela (o ausente gravado como 0 ou 9999, Capítulo 7) é pior que os três, porque nem se sabe que o dado falta.

### 3.2 A unidade independente

Em 1913, Markov contou vogais e consoantes nas 20 000 primeiras letras do *Ievguêni Oniéguin* e mostrou que a lei dos grandes números sobrevive à dependência. Mas a dependência muda quanto a amostra vale. Como as letras tendem a alternar, um bloco de 100 delas vale cerca de 330 letras independentes; numa cadeia "pegajosa", em que cada estado tende a se repetir, 100 observações valem bem menos que 100.

**O tamanho efetivo de uma amostra é uma propriedade do processo, não da contagem.** Pacientes de um mesmo dia compartilham a mesma demanda e não contam como observações independentes. Tratá-los como independentes é pseudorreplicação: a margem sai pequena demais. Nos voos de Nova York (Capítulo 12), 225 mil voos valem, em informação independente, o mesmo que 364 dias.

### 3.3 Um ajuste só é evidência se podia ter falhado

Se um semiplano separa perfeitamente 50 pontos, isso teria chance de 2 × 10⁻¹² caso os rótulos fossem acaso: cerca de 39 bits de evidência. Uma tabela que guarda cada rótulo também os separa e não traz bit nenhum, porque separaria qualquer coisa. É a falseabilidade de Popper em forma de conta, que a teoria de Vapnik e Chervonenkis mede pela capacidade da família de modelos.

**O jardim dos caminhos que se bifurcam.** Simmons, Nelson e Simonsohn (2011) simularam quatro liberdades comuns num experimento sem efeito: quando parar de coletar, quais variáveis usar, quais casos excluir, quais comparações relatar. A taxa de falsos positivos subiu de 5% para 60,7%. Nem a validação cruzada escapa. Num mundo de puro ruído, com 50 casos e 50 preditores sem relação com nada, sortear 500 modelos e ficar com o de menor erro de validação produz um vencedor que parece melhor que prever zero em 73% das repetições e não é melhor em nenhuma diante de dados novos.

Ioannidis (2005) mostrou que o valor preditivo de um achado significativo depende do poder 1 − β e das chances R de uma hipótese da área ser verdadeira:

```latex
\text{VPP}=\frac{(1-\beta)R}{R-\beta R+\alpha}
```

A Open Science Collaboration (2015) replicou 100 estudos de psicologia: 97% dos originais tinham p < 0,05, contra 36% das replicações. Por isso vale a regra de Deborah Mayo: um resultado só é evidência se o procedimento provavelmente teria dado errado caso a afirmação fosse falsa. Numa simulação, o mesmo "p = 0,04" tem severidade 0,96 quando vem de um teste pré-registrado e 0,42 quando vem do jardim.

**Os tipos de erro.** A estatística clássica nomeia dois erros, e a prática acrescentou outros.

| Erro | O que acontece | Fonte |
| --- | --- | --- |
| Tipo I | Rejeitar uma hipótese nula verdadeira: o falso alarme | Neyman e Pearson |
| Tipo II | Não rejeitar uma nula falsa: o efeito perdido | Neyman e Pearson |
| Tipo III | Dar a resposta certa ao problema errado | Kimball (1957) |
| Tipo S | Estimar um efeito com o sinal trocado | Gelman e Tuerlinckx (2000); Gelman e Carlin (2014) |
| Tipo M | Exagerar a magnitude de um efeito verdadeiro | Gelman e Carlin (2014) |

Os tipos S e M explicam por que estudos pequenos e "significativos" decepcionam na replicação: com pouco poder, só passam no filtro de p < 0,05 as estimativas que o acaso inflou, e algumas têm até o sinal errado. O tipo III é o mais estocológico de todos, porque não é um erro de conta: é um desalinhamento entre a pergunta do Agente e a pergunta que o Modelo responde. O Capítulo 10 mostra um exemplo exato.

### 3.4 A checagem preditiva

De Box a Gelman e Shalizi, a crítica de um modelo começa simulando réplicas do modelo ajustado e vendo em que estatística os dados reais caem fora delas. Num exemplo do curso, um modelo Poisson passa na média, falha na variância (nenhuma réplica chega perto) e subestima em 42 vezes a chance de um dia de pico. Um modelo pode servir para dimensionar a equipe média e ser perigoso para o plantão de pico. A pergunta certa não é se ele é verdadeiro, mas para que decisão é bom o bastante.

### 3.5 Calibração e resolução

Uma previsão probabilística se pontua com uma **regra própria**, em que quem acredita em q minimiza a penalidade declarando q. O escore de Brier é próprio. A taxa de acerto não é: quem acredita em 70% fica indiferente entre declarar 51% e 99%. A decomposição de Murphy (1973) separa o Brier em três partes:

```latex
BS = \underbrace{\text{REL}}_{\text{erro de calibração}} - \underbrace{\text{RES}}_{\text{resolução}} + \underbrace{\text{UNC}}_{\text{incerteza dos eventos}}
```

O climatologista, que diz sempre a taxa histórica, é perfeitamente calibrado e não tem resolução nenhuma: calibrado e inútil. A assimetria prática é que **a calibração se conserta depois, a resolução não.** Num exemplo do curso com 4 000 eventos, recalibrar um previsor confiante demais leva seu Brier de 0,1956 a 0,1851, quase o do perito honesto (0,1846). Um previsor ruidoso melhora pouco, de 0,2154 a 0,2089, porque a informação perdida não volta. O Capítulo 13 encontra exatamente isso em dados reais.

Para modelos de caixa-preta, a **predição conforme** dá uma cobertura garantida sem supor a forma da distribuição, P(Y ∈ C(X)) ≥ 1 − α. É a ferramenta que o H3 usou, em versão adaptativa, sob deriva.

**A tolerância da calibração depende do tamanho da amostra.** Simulem-se 2 000 previsões perfeitamente calibradas com as probabilidades de um cofre e calcule-se o erro esperado de calibração, o ECE de dez caixas de massa igual: o quantil 95% desse nulo é a tolerância honesta, e ele encolhe com n. Num cofre de 1 488 casos o ruído chega a 0,032; num de 104 489, a 0,0035. Uma tolerância fixa de 0,03 é então duas coisas ao mesmo tempo: apertada demais para os cofres pequenos e frouxa demais para os grandes, onde deixa passar descalibração perfeitamente detectável. Aplicada a tolerância que depende de n, nenhum dos seis cofres binários do H3 passa — nem os três que a regra fixa aprovava. O Capítulo 13 mostra o que isso faz com a leitura daquele horizonte.

### Resumo

- O erro de uma média é qualidade × quantidade × dificuldade; tamanho não compensa seleção.
- A margem se calcula na unidade independente, que é uma propriedade do processo.
- Um ajuste vale o quanto o modelo podia ter falhado; toda tentativa precisa ser contada.
- Calibração é barata de consertar; resolução, não.

### Exercícios

1. *(Conceito)* Uma empresa diz: "temos milhões de usuários, então nossa pesquisa de satisfação é precisa". Que fator da identidade de Meng ela esqueceu?
2. *(Conta)* Na tabela de Charig, recalcule a taxa bruta de cada cirurgia como média dos estratos ponderada pelo número de casos e explique por que ela inverte os estratos. Que variável é confundidora?
3. *(Laboratório)* Simule 50 casos com 50 preditores de puro ruído, sorteie 500 modelos e confira a fração de vencedores que parecem melhores que prever zero.
4. *(Conceito)* Por que o climatologista é calibrado e inútil? Que termo da decomposição de Murphy ele zera?
5. *(Conceito)* Dê um exemplo de pseudorreplicação no seu campo e diga qual seria a unidade independente.

## Capítulo 4. Causalidade, intervenção e invariância

**Objetivos.** Situar uma pergunta num dos três degraus de Pearl; aplicar o critério da porta dos fundos; decompor uma diferença ingênua em efeito e viés de seleção; e entender a Predição Causal Invariante (ICP), o teste que decidiu o horizonte H2.

### 4.1 Os três degraus

Pearl distingue três degraus de pergunta, e cada um pede mais do modelo que o anterior.

| Degrau | Notação | Pergunta típica |
| --- | --- | --- |
| 1. Associação | P(y \| x) | Quem toma o remédio tem menos infarto? |
| 2. Intervenção | P(y \| do(x)) | Se eu tomar, meu risco cai? |
| 3. Contrafactual | P(yₓ \| x′, y′) | Eu teria infartado sem tomar? |

A ponte entre os dois primeiros é um grafo causal, que os dados sozinhos não fornecem. A ficha do grafo causal (Capítulo 8) começa por declarar em que degrau a afirmação está.

### 4.2 A porta dos fundos

Quando um conjunto Z bloqueia todos os caminhos que entram no tratamento por trás, sem conter nenhum descendente dele, vale:

```latex
P(y \mid do(x)) = \sum_z P(y \mid x, z)\,P(z)
```

**Caso trabalhado: a terapia hormonal.** Em 1991, a coorte do Nurses' Health Study, com cerca de 48 mil mulheres, relatou risco relativo de 0,56 de doença coronariana grave entre as usuárias de estrogênio. Em 2002, o ensaio sorteado da Women's Health Initiative, com 16 608 mulheres, encontrou razão de riscos de 1,29. As usuárias da coorte tinham outros hábitos, outra renda e outro acesso a médicos. A comparação não é perfeita (estrogênio isolado num caso, combinado com progestina no outro), mas a lição ficou.

Ajustar "tudo o que foi medido" também não resolve. Ajustar um mediador apaga o efeito, ajustar um colisor fabrica associação e ajustar um instrumento amplifica o viés que sobra.

### 4.3 Resultados potenciais

Na linguagem de Neyman e Rubin, cada pessoa tem dois desfechos possíveis, Y(1) com o tratamento e Y(0) sem ele, e só um é observado. A diferença ingênua entre tratados e não tratados se decompõe exatamente assim:

```latex
\mathbb{E}[Y\mid D{=}1]-\mathbb{E}[Y\mid D{=}0] = \underbrace{\mathbb{E}[Y(1)-Y(0)\mid D{=}1]}_{\text{efeito nos tratados}} + \underbrace{\mathbb{E}[Y(0)\mid D{=}1]-\mathbb{E}[Y(0)\mid D{=}0]}_{\text{viés de seleção}}
```

Sortear o tratamento zera o segundo termo. O Capítulo 11 encontra exatamente isso nos dados de LaLonde, onde o viés de seleção troca o sinal do efeito.

### 4.4 Invariância: o que sobrevive à troca de ambiente

Uma relação causal sobrevive à troca de ambiente; uma correlação de conveniência, não. A Predição Causal Invariante (Peters, Bühlmann e Meinshausen, 2016) transforma essa ideia num teste.

**A afirmação formal.** Se S é o conjunto das causas diretas de Y, a distribuição P(Y | X\_S, E = e) é a mesma em todo ambiente e que intervém nas outras variáveis sem intervir na equação de Y.

**O procedimento.**

1. Para cada subconjunto candidato S, teste se a relação de Y com X\_S é invariante entre os ambientes.
2. Guarde todos os S que passam.
3. Devolva a interseção deles, Ŝ.

**A garantia é assimétrica.** Com probabilidade de pelo menos 1 − α, Ŝ só contém causas diretas, mas nada garante que contenha todas. Abster-se, devolvendo o conjunto vazio, é resultado, não falha. Isso acontece de dois modos: o teste rejeita todos os conjuntos (os ambientes agem direto em Y, ou o modelo está mal especificado) ou aceita tantos que a interseção fica vazia (os dados não têm poder para distinguir). Com amostras grandes, o teste padrão rejeita qualquer desvio do modelo linear gaussiano, e foi isso que o H2 encontrou na primeira rodada (Capítulo 12).
O teste trata cada linha como uma unidade independente; quando o mesmo voluntário dá 16 medidas ou a mesma escola
dá 403 alunos, a unidade honesta é o voluntário ou a escola, e corrigido para isso o mesmo teste muda de veredito
entre áreas (Capítulo 12).

**Por que importa para decidir.** A invariância é também uma resposta à crítica de Lucas (Capítulo 5): só o que é invariante entre regimes merece uso quando o regime muda. No H2 (Capítulo 12), um modelo invariante previu bem uma droga que nunca tinha visto, enquanto o modelo com todas as variáveis explodiu.

**Uma lição que o H2 acrescentou.** O efeito transporta melhor que o conjunto. Mesmo quando o ICP rejeitou todos os conjuntos, o modelo ajustado nos ambientes de treino previu bem o ambiente nunca visto em 9 domínios (habilidade de 0,04 a 0,88). Encontrar o conjunto exato de causas é mais exigente do que prever sob mudança.

### Resumo

- Associação, intervenção e contrafactual são perguntas diferentes; o grafo faz a ponte.
- A porta dos fundos diz o que ajustar; ajustar tudo pode piorar.
- Diferença ingênua = efeito + viés de seleção; o sorteio zera o viés.
- ICP devolve só causas diretas com alta probabilidade, e pode se abster.

### Exercícios

1. *(Conceito)* Classifique no degrau de Pearl: "bairros com mais polícia têm mais crimes"; "se dobrarmos o policiamento, o crime cai?"; "esse assalto teria acontecido com uma viatura na esquina?".
2. *(Conceito)* Desenhe um grafo em que ajustar por Z cria uma associação entre X e Y que não existia. Que nome tem Z?
3. *(Conta)* No projeto LaLonde (Capítulo 11; os valores em dólares estão em h1-real/README.md), a comparação crua entre participantes do NSW e trabalhadores do CPS dá −8 498 dólares de renda em 1978, e o experimento dá +1 794. Quanto vale o viés de seleção da comparação crua?
4. *(Conceito)* Por que o ICP pode devolver o conjunto vazio quando tem dados demais? Relacione com a sensibilidade do teste a desvios do modelo.
5. *(Laboratório)* Simule Y = 2X₁ + ε e X₂ = Y + η, com três ambientes que mudam a média de X₁. Rode o ICP linear e verifique que Ŝ = {X₁}. Depois faça o ambiente agir também em Y e veja o que acontece.

## Capítulo 5. Decidir: valor da informação, tempo, caudas e Goodhart

**Objetivos.** Decidir pelo limiar custo/perda e medir o valor de uma informação; distinguir média de conjunto de média temporal e aplicar o critério de Kelly; reconhecer quando a média deixa de existir; identificar laços em que a previsão muda o que prevê; perguntar quem paga pelo erro; e comunicar uma decisão em frequências naturais, com o ponto em que ela muda.

### 5.1 O limiar e o valor da informação

Se proteger custa C e não proteger custa L quando o evento ocorre, protege-se quando p > C/L. Uma informação vale pelo quanto pode mover a probabilidade para o outro lado desse limiar, não pelos bits que carrega. O valor de saber o estado do mundo antes de agir, o EVPI, é o teto do valor de qualquer informação:

```latex
\mathrm{EVPI} = \mathbb{E}_\theta\big[\max_a U(a,\theta)\big] - \max_a \mathbb{E}_\theta\big[U(a,\theta)\big]
```

O valor de qualquer exame real (EVSI) fica entre zero e ele.

**Caso trabalhado: dois exames com a mesma informação.** No Lab 14 do curso, uma doença tem prevalência de 0,3; tratar custa 1 e deixar um doente sem tratamento custa 10, de modo que se trata quando a chance passa de 0,10. Dois exames carregam a mesma informação mútua, 0,319 bit. O exame sensível vale 0,419, num teto (o EVPI) de 0,70. O específico vale zero, porque mesmo o seu resultado negativo deixa a chance acima do limiar e não muda a ação. Bits não são valor.

E nem toda informação é bem-vinda. Good (1967) provou que informação gratuita nunca piora a decisão de quem tem preferências sobre estados do mundo. Mas quem tem preferências sobre as próprias observações pode pagar para não saber: entre pessoas com risco de doença de Huntington, só uma minoria faz o exame genético antes dos sintomas, 17,4% no Reino Unido entre 1994 e 2014 (Baig et al., 2016).

**Quando não há probabilidade em que confiar.** O limiar C/L e o EVPI supõem uma distribuição sobre os estados do mundo. No canto da incerteza da matriz de Stirling (Capítulo 2), ela não existe de modo confiável, e a teoria da decisão oferece critérios robustos que dispensam probabilidades. Pelo espírito do invariante Φ4 (Capítulo 8), o termo de escopo deve declarar qual deles foi usado.

| Critério | Regra | Postura |
| --- | --- | --- |
| Wald (maximin) | Escolher a ação cujo pior caso é o menos ruim | Pessimismo máximo |
| Savage (arrependimento mínimo) | Escolher a ação cujo maior arrependimento é o menor | Evitar o "se eu soubesse" |
| Laplace | Tratar os estados como igualmente prováveis | Ignorância como uniforme |
| Hurwicz | Ponderar o pior e o melhor caso por um índice de otimismo | Meio-termo declarado |

**Exemplo do curso: a altura do dique.** Depois da enchente de 1953, a Comissão Delta adotou para a Holanda central um padrão de 1 em 10 000 anos, e van Dantzig (1956) tratou a altura do dique como um problema de decisão, equilibrando o custo da obra e o dano esperado. O Lab 14 do curso (Exercício 5) refaz o problema com um modelo próprio: as máximas anuais seguem uma GEV de forma ξ desconhecida, entre −0,1 e 0,2, e o custo anual de uma altura h é a obra, h − 2, mais o dano esperado, 3 000 × P(máxima > h). Os quatro modelos (ξ = −0,1, 0, 0,1 e 0,2) discordam pouco no nível de 100 anos (de 3,11 a 4,26 m) e muito no de 10 000 anos (de 3,81 a 9,96 m), e cem anos de dados gerados com ξ = 0,1 não decidem entre eles: dão estimativas de ξ entre −0,02 e 0,22. Os números são do modelo do laboratório, não os de van Dantzig; dele vem a forma do problema. Cinco regras de decisão dão cinco alturas: o valor esperado com um só modelo, a média dos modelos (o critério de Laplace aplicado aos modelos), Wald, Savage e a precaução:

| Critério | Altura | Custo anual se ξ = −0,1 | ξ = 0 | ξ = 0,1 | ξ = 0,2 | Maior arrependimento | Com ξ = 0,4 na lista |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Valor esperado com Gumbel (ξ = 0) | 4,76 m | 2,76 | 3,06 | 7,14 | 18,90 | 12,05 | 4,76 m |
| Média dos modelos | 6,20 m | 4,20 | 4,20 | 4,67 | 7,99 | 2,17 | 8,09 m |
| Maximin de Wald | 7,46 m | 5,46 | 5,46 | 5,56 | 6,85 | 3,43 | 11,67 m |
| Arrependimento minimax de Savage | 5,93 m | 3,93 | 3,94 | 4,62 | 8,75 | 1,90 | 8,12 m |
| Precaução: 1 em 10 000 no pior modelo | 9,96 m | 7,96 | 7,96 | 7,97 | 8,26 | 5,93 | 31,11 m |

O arrependimento é o custo menos o menor custo possível naquele modelo. O valor esperado com um único modelo, o de Gumbel, escolhe 4,76 m, que é exatamente o nível de 10 000 anos desse modelo, e custa de 2,76 a 18,90 conforme a cauda. Wald fica com o ótimo do pior modelo, 7,46 m; Savage limita o quanto se pode lamentar em qualquer mundo, 5,93 m; a precaução paga caro em todos. A lista de modelos também é uma escolha: quando alguém propõe ξ = 0,4, Wald sobe a 11,67 m e a precaução a 31,11 m, enquanto o modelo único não se mexe, e o arrependimento minimax passa de 5,93 a 8,12 m. Chernoff (1954) mostrou que esse critério pode mudar de escolha até quando se acrescenta uma alternativa que ninguém escolheria; aqui o que se acrescentou foi um modelo. Os critérios robustos não são mais "corretos" que o esperado: são a escolha honesta quando a probabilidade de que a conta depende não merece confiança. O critério e a lista de modelos têm de ser declarados antes de ver a resposta.

### 5.2 O tempo e a ruína

A média entre muitas pessoas não é a média ao longo da vida de uma. Numa dinâmica multiplicativa,

```latex
X_{t+1} = X_t M_t, \qquad \mathbb{E}[M] > 1 \;\text{ e }\; \mathbb{E}[\ln M] < 0,
```

a média do conjunto cresce enquanto quase toda trajetória decai.

**Caso trabalhado: a moeda de Peters.** Aposta-se toda a riqueza; cara aumenta 50%, coroa tira 40%. A esperança cresce 5% por rodada, mas o que se acumula numa trajetória é o logaritmo:

```latex
\ln \mathbb{E}[M] \approx +0{,}049, \qquad \mathbb{E}[\ln M] = \tfrac12\ln 1{,}5 + \tfrac12 \ln 0{,}6 \approx -0{,}053
```

| Rodadas | Trajetórias acima do ponto de partida |
| --- | --- |
| 10 | 37,7% |
| 100 | 13,6% |
| 1 000 | 0,01% |

Em 1 000 rodadas, a esperança vale 1,55 × 10²¹ e a mediana, 10⁻²³. Os fatores são independentes e idênticos; o que não é ergódico é o produto. Até quem simula cai na armadilha: a média de N trajetórias acompanha a esperança só por um tempo que cresce como ln N, cerca de 12 rodadas com 10 trajetórias e 112 com um milhão.

**Quanto expor: o critério de Kelly (1956).**

```latex
f^* = \arg\max_f \; \mathbb{E}[\ln(1 + fR)]
```

Numa aposta paga de igual para igual com chance 0,6, a fração ótima é 0,2 da riqueza. Apostar tudo levaria a esperança, em 100 rodadas, a 8,3 × 10⁷ vezes o capital, com chance de sobreviver de 0,6¹⁰⁰, cerca de 10⁻²². O dobro da fração ótima, 0,4, já empobrece com a mesma vantagem. E a incerteza sobre o parâmetro pesa: com a probabilidade estimada em 25 rodadas, o Kelly cheio tem taxa de crescimento negativa em 16% das estimativas, enquanto meio Kelly garante 75% do crescimento com um quarto da variância. **A incerteza epistêmica é um argumento para apostar menos.**

**Seguro e cooperação.** No navio de Bernoulli, retomado por Peters e Adamou, um armador com riqueza de 100 000 paga 1 800 para cobrir uma perda de 34 000 que ocorre com chance de 5%. Em esperança, ele perde 100 por mês e a seguradora ganha 100. Em crescimento temporal, os dois ganham: o armador passa de 1,94% para 2,18% ao mês, e a seguradora, que detém muitas trajetórias independentes, de zero para +0,007%. Na moeda de Peters, três jogadores que repartem a riqueza a cada rodada crescem 1,6% por rodada, quando cada um sozinho decairia. A pergunta que decide o caso é quem detém as trajetórias.

### 5.3 Quando a média deixa de existir

Numa distribuição de Pareto, com P(X > x) = x^(−α), o momento de ordem k existe se e somente se k < α. Com α = 1,1, a média existe e vale 11, mas a média de um milhão de observações tem mediana 8,3, fica abaixo de 11 em 91% das amostras, e o intervalo de 95% usual contém o valor verdadeiro em só 26% delas. A fração da média que vem de valores acima de m é m^(1−α); em mediana, 28% da média verdadeira vêm de valores maiores que o maior que a amostra de um milhão viu.

Quando a cauda manda, a gaussiana falha justamente onde importa. Pelo teorema de Fisher, Tippett e Gnedenko, o máximo normalizado de uma amostra só pode tender a uma de três famílias: Gumbel, Fréchet ou Weibull. Num exemplo do curso com 50 anos de retornos de cauda pesada, a perda de 1 em 100 anos vale 17,4%. Uma gaussiana ajustada aos retornos diários dá 3,9% e subestima a perda em todas as repetições; a distribuição de extremos acerta em mediana, com intervalo de 11% a 53%.

**Os três estados do acaso (Mandelbrot).** Mandelbrot classificou o acaso pelo modo como as somas se comportam.

| Estado | Exemplo de distribuição | O que acontece com a soma |
| --- | --- | --- |
| Brando | Gaussiana, exponencial | Nenhuma observação domina; a média converge depressa |
| Lento | Lognormal | Todos os momentos existem, mas a convergência é tão lenta que amostras reais enganam |
| Selvagem | Pareto com α ≤ 2 | Uma única observação pode dominar a soma; a variância, ou até a média, não existe |

Taleb chamou o primeiro de Mediocristão e o último de Extremistão, e acrescentou o tipo de consequência: o "quarto quadrante", onde a cauda é selvagem e a perda depende do tamanho do evento, é onde os modelos falham mais e custam mais. O Pareto com α = 1,1 acima é acaso selvagem; os incêndios dinamarqueses, com α estimado em 1,81 (intervalo de 1,41 a 2,56), provavelmente também (Capítulo 11).

**Caso trabalhado: vender seguro.** Uma estratégia ganha 1% por semana e perde 50% com chance de 0,4%. Um histórico de 250 semanas não mostra nenhuma perda em 37% das vezes, e quem estima a chance pela frequência observada conclui que ela é zero e alavanca. Entre 10 000 investidores simulados, 35,8% dos que apostam pelo critério de Kelly com a probabilidade estimada se arruínam. Com a regra de sucessão de Laplace, ninguém se arruína.

### 5.4 A previsão que muda o que prevê

Quando um modelo é publicado, as pessoas reagem a ele. Uma métrica vira meta e deixa de medir: é a lei de Goodhart. Em 1976, Lucas mostrou o mesmo na economia: os parâmetros de um modelo estimado sob uma política dependem das regras que os agentes esperam, e mudam quando a política muda. Num exemplo do curso, a curva de Phillips antiga promete 4,59% de desemprego e o regime novo entrega 5,88%.

**Caso trabalhado: o aplicativo de duas rotas.** O previsor ingênuo manda todos para a rota mais rápida de ontem e erra sempre que mais de 2/7 dos motoristas o seguem. Com metade deles, quem usa o aplicativo leva 41,0 minutos e quem não usa, 36,8. O previsor que divide os usuários para igualar os tempos erra por 0,3 minuto, e todos levam 35,0. Perdomo e colegas (2020) chamaram de performativamente estável o modelo que é ótimo para a distribuição que ele mesmo induz.

**Aprender ou deformar?** Um agente que minimiza a energia livre variacional pode baixar a surpresa de três modos: mudando a crença, mudando o mundo ou deixando de olhar.

```latex
F[q] = D\big(q \,\|\, p(s \mid o)\big) - \ln p(o) \;\ge\; -\ln p(o)
```

Um termostato que "acredita" em 22 °C numa sala a 18 °C termina com o mesmo F quando aprende que a sala está fria e quando a aquece até 22 °C. Só um contrafactual de fora separa os dois casos, e por isso a arquitetura exige rodar o modelo com e sem a ação.

**O laço do H3, medido.** O H3 supôs que quem desiste da fila volta a chamar com probabilidade 0,6. O mesmo parâmetro foi medido em outra central real, de 1999, com um desenho quase experimental: a fila encontrada ao entrar, dentro da mesma data e hora, serve de instrumento para a desistência. Por esse desenho, desistir em vez de ser atendido aumenta em 0,145 a chance de o mesmo cliente ligar de novo em até quinze minutos, com intervalo de 90% de 0,132 a 0,157. O sinal confere e a ordem de grandeza não: o laço declarado é cerca de quatro vezes o observado, fora da faixa de um terço a três que estava fixada de antemão. O excesso de chegadas que o laço medido gera nessa central é de 2,3% das chegadas, contra o viés de 29% que o laço declarado produziu no H3 — comparação de ordem de grandeza, porque o viés do H3 depende também da taxa de desistência simulada, e não só do laço. Ou seja: a correção performativa do H3 foi testada contra um laço bem mais forte que o de um caso real. E o valor medido é provavelmente um limite inferior, porque uma chamada nova só é visível quando entra na fila.

### 5.5 Quem paga pelo erro

Quando as taxas-base diferem entre grupos, o mesmo limiar produz taxas de erro diferentes. A identidade de Chouldechova (2017) mostra por quê:

```latex
\text{FPR}=\frac{p}{1-p}\cdot\frac{1-\text{VPP}}{\text{VPP}}\cdot(1-\text{FNR})
```

Com valor preditivo e falsos negativos iguais entre grupos, prevalências p diferentes forçam taxas de falso positivo diferentes. Igualar aprovações, igualar erros e igualar calibração por grupo são exigências incompatíveis quando as prevalências diferem e o preditor não é perfeito. "Quero justiça" ainda não é uma decisão: é preciso declarar que igualdade se escolheu, quanto ela custa, para onde o erro migrou e quem passa a pagá-lo.

### 5.6 Comunicar é parte da decisão

Um resultado mal comunicado falhou na camada do Agente. "30% de chance" não diz 30% de quê; "de cada 100 pessoas como você, 30" diz. As regras: denominadores fixos, risco absoluto antes do relativo e classe de referência explícita. Gráficos de resultados hipotéticos (HOPs), que mostram desfechos simulados um após o outro, fazem o leitor sentir a variação que uma barra de erro esconde. A recomendação declara o ponto em que a ação muda e uma cláusula objetiva de quando deixa de valer.

### Resumo

- Decide-se pelo limiar C/L; a informação vale o quanto move a decisão.
- Em dinâmica multiplicativa, quem vive uma trajetória deve maximizar E\[ln M\], e a incerteza manda apostar menos.
- Com cauda pesada, a média amostral engana ou nem existe.
- Previsões publicadas mudam o mundo; só o contrafactual separa aprender de deformar.
- Justiça exige escolher entre igualdades incompatíveis e declarar quem paga.

### Exercícios

1. *(Conta)* No navio de Bernoulli, o seguro custa 1 800 e cobre uma perda de 34 000 com chance de 5%. Pela regra do custo-perda, que manda proteger quando p > C/L, o armador deveria se segurar? O que diz o crescimento temporal?
2. *(Conta)* Na moeda de Peters, calcule E\[ln M\] e explique por que a mediana cai mesmo com a esperança crescendo.
3. *(Conta)* Para uma aposta de igual para igual com chance p, mostre que f\* = 2p − 1.
4. *(Conceito)* Na moeda de Peters, três jogadores que repartem a riqueza a cada rodada crescem 1,6% por rodada, quando cada um sozinho decairia. Por que repartir muda o crescimento sem mudar a esperança de nenhum deles?
5. *(Laboratório)* Simule 10 000 investidores na estratégia que vende seguro, com p estimado pela frequência e pela regra de Laplace. Compare as taxas de ruína.
6. *(Conceito)* A Women's Health Initiative relatou razão de riscos de 1,29 (seção 4.2). Que número falta para reescrevê-la como "de cada 1 000 mulheres, tantas"? Onde ele deveria ser procurado?

# Parte II. Arquitetura

## Capítulo 6. O hipergrafo tipado e as cinco lentes

**Objetivos.** Descrever a máquina epistemológica auditável; ler a definição G = (V, E\_tree, E\_graph, H, Φ); listar os campos obrigatórios de um nó; e usar as cinco lentes como perguntas sobre qualquer problema.

### 6.1 Uma máquina epistemológica auditável

Uma disciplina só vira prática quando ganha instrumentos. O projeto propõe operar a Estocologia como uma **máquina epistemológica auditável**: um sistema em que cada alegação empírica vira código executável, e em que nenhuma conclusão é aceita sem passar por testes automáticos. Não é um buscador de textos que resume a literatura. É um ambiente em que as hipóteses são simuladas e atacadas, e em que pensar sobre o mundo empírico exige inferir explicitamente um processo gerador de dados oculto, o nó N\_Core.

### 6.2 O hipergrafo

O conhecimento fica guardado como um grafo tipado:

```latex
\mathcal{G} = (V,\; E_{\text{tree}},\; E_{\text{graph}},\; H,\; \Phi)
```

| Componente | O que guarda | Exemplo |
| --- | --- | --- |
| V, nós | Conceitos, hipóteses, variáveis, conjuntos de dados | L1\_demanda\_real: contagens de chamadas a cada 5 minutos |
| E\_tree, árvore | O que depende do quê: pré-requisitos entre módulos do curso (M0 a M15, especiais e bancas) | Na ficha do H0 real, L2\_modelo\_nulo → L2\_media\_por\_chamada, com o rótulo "Teste severo (M5)" |
| E\_graph, transversais | Pontes matemáticas, arestas causais do(x) e arestas de patologia | Jensen liga M0 a M6 e M8 |
| H, hiperarestas | As cinco lentes, que envolvem grupos inteiros de nós | Na ficha do H0 real, Crononomia sobre L1\_demanda\_real e L2\_erlang\_c: "a demanda tem calendário" |
| Φ, invariantes | Os quatro invariantes que toda alegação cumpre para entrar no grafo | Φ2: margem na unidade independente |

**O esquema de um nó.** Todo nó declara obrigatoriamente cinco campos. Um nó sem algum deles é recusado na gravação.

| Campo | O que declara |
| --- | --- |
| id | Identificador único |
| camada | L1 Mundo, L2 Modelo ou L3 Agente; o nó raiz, que guarda o processo gerador oculto, usa raiz ou core |
| tipo\_incerteza | Aleatoria, Epistemica ou Knightiana\_Especificacao; nao\_se\_aplica no nó raiz |
| definicao\_formal | A definição matemática, em uma linha |
| artefato\_simulavel | O ponteiro para o código que o reproduz com semente fixa |

Um nó real, copiado da ficha do H0 em domínio real (h0-real/ficha.yaml):

```yaml
- id: L2_erlang_c
  camada: L2
  tipo_incerteza: Knightiana_Especificacao
  definicao_formal: SL(c, λ) = 1 − C(c, λ·AHT) e^{−(c − λ·AHT) t / AHT}, estacionária, sem abandono
  artefato_simulavel: estocologia/central.py::nivel_servico
```

Repare que a fórmula de Erlang C está tipada como incerteza knightiana: ela supõe estacionariedade e ausência de abandono, que o Mundo não garante.

**As arestas de patologia.** Marcam desalinhamentos estruturais quando o Agente aplica um Modelo incompatível com o Mundo: falácia da média, paradoxo da inspeção, transposição do condicional (falácia do promotor), problema do peru de Russell e ilusão do quarto escuro. Na ficha do H0 real, a aresta L2\_erlang\_c → L1\_fila\_do\_dia leva o rótulo "Falácia da Média: SL(E\[Λ\]) no lugar de E\[SL(Λ)\]", e a aresta L3\_gestor\_de\_escala → L2\_erlang\_c, "Decisão pelo dia médio sem a cauda dos dias".

**As pontes matemáticas.** As arestas transversais do primeiro tipo ligam módulos distantes por uma relação matemática explícita, navegável nos dois sentidos. O contrato dos agentes (AGENTS.md) fixa quatro.

| Ponte | Relação | O que liga |
| --- | --- | --- |
| Jensen | E\[f(X)\] ≥ f(E\[X\]) para f convexa | A função da média (M0) à informação (M6) e à ergodicidade (M8) |
| Priori como regularização | −ln P(θ) ↔ λΩ(θ) | Bayes (M3) ao aprendizado com penalidade (M12) e à elicitação SHELF do F7 |
| Informação como crescimento | ΔW ≤ I(X; Y) | A informação mútua (M6) ao ganho de crescimento de Kelly (M8) e à inferência ativa (M11) |
| Ruído como temperatura | Ruído de minilote ↔ temperatura T | A mecânica estocástica (M10) à inferência ativa (M11) e ao aprendizado (M12) |

A segunda ponte diz que penalizar coeficientes grandes é o mesmo que declarar uma priori que os acha improváveis: quem regulariza já calcula uma moda a posteriori, sem dizer qual priori usou. A terceira diz que um apostador com uma informação lateral Y sobre o resultado X ganha, em taxa de crescimento, no máximo I(X; Y): bits viram crescimento, mas só para quem aposta por Kelly. A quarta diz que o ruído de estimar o gradiente num minilote age como uma temperatura, que ajuda a escapar de mínimos rasos, como na fórmula de Kramers do Capítulo 8.

### 6.3 As cinco lentes

As hiperarestas funcionam como perguntas que se fazem a qualquer problema. Cada lente tem uma grandeza que mede e um hábito crítico, e atravessa os conceitos sem acrescentar objetos.

| Lente | Pergunta | O que mede | Hábito crítico |
| --- | --- | --- | --- |
| Fluxonomia | Como a probabilidade e a informação fluem e se perdem? | Correntes, informação mútua | A conta fecha? Reprocessar não cria informação |
| Crononomia | A média entre muitos coincide com a média ao longo do tempo de um só? | Tempos de mistura, relaxação e escape; E\[ln M\] | De que tempo se fala: do processo, do observador ou da simulação? |
| Escalonomia | A cauda é pesada a ponto de a média nem existir? | Expoente de cauda, tamanho efetivo, capacidade do modelo | Declarar se a média teórica existe |
| Morfonomia | A estrutura encontrada sobrevive quando o ambiente muda? | Invariância (ICP), compressibilidade (MDL) | Reescrever o modelo em duas formas e ver o que se conserva |
| Crisonomia | A informação vale o seu custo, e as previsões estão calibradas? | Perda, escore próprio, valor da informação | Cobrar de cada número o histórico de acertos na frequência que ele prometeu |

Dois exemplos de como uma lente vira conta. Na Fluxonomia, a desigualdade do processamento de dados diz que, se Z é obtido de Y, então I(X; Z) ≤ I(X; Y): três jornais que publicam a mesma pesquisa valem um. Na Morfonomia, duas representações podem dar as mesmas previsões e ainda assim autorizar conclusões diferentes sob intervenção, e só o que se conserva entre ambientes serve fora deles.

**Como um domínio novo aciona as lentes.** O PLAN.md dá a regra prática. É um processo financeiro, de carreira ou de seguro? Aciona a Crononomia. Envolve sinistros, rendas, viralização ou tamanho de empresas? Aciona a Escalonomia. Envolve recomendação, regulação ou previsão publicada? Aciona Fluxonomia e Morfonomia, com simulação com e sem a ação.

### 6.4 Onde ficam os dados

O hipergrafo guarda só a topologia leve: nós, arestas, contratos YAML e hashes. Microdados e séries ficam em Parquet particionado, consultado por DuckDB com projeção explícita de colunas, poda de partição e um orçamento rígido de bytes varridos. A regra de escala é do próprio plano: um modelo que exige uma varredura completa por previsão não escala, por melhor que seja o erro de teste.

O tamanho do grafo cresceu com a pesquisa, mas abaixo das metas do plano: 85 nós no H1 real e 454 178 no H3, dos quais 453 973 são previsões e só 205 são topologia. O H3 chegou a 45% do milhão que o plano previa.

O número do H2 merece uma correção, porque mostra como é fácil inflar essa conta. Os "1 538 nós" que a primeira versão deste capítulo citava são de uma rodada antiga do gerador do grafo. Rodando o `grafo.py` do H2g sobre os resultados do próprio H2g, o grafo tem 12 797 nós — e 12 288 deles, 96%, são hipóteses do ICP, uma por subconjunto testado, que crescem como 2 elevado ao número de variáveis e não são conhecimento novo. O teste de escala da auditoria rodou o funil em lote sobre 329 domínios gerados por regra a partir de um catálogo público (327 até o fim, a 3,8 segundos de processador por domínio) e levou o grafo a 26 634 nós, 28 619 arestas de árvore, 26 795 pontes e 4 628 hiperarestas. Só 6 494 dos nós são de conceito — domínio, camadas, variáveis, ambientes, filtros, decisão — e só 554 vêm de pacotes curados; os outros 20 140 são a mesma família de hipóteses. O gargalo não é a máquina: é a curadoria semântica, que não escala por regra, a oferta de conjuntos com ambientes identificáveis (9% do catálogo passa), as pontes, que crescem com o quadrado do número de domínios, e manter tudo em memória.

### Resumo

- O hipergrafo tipado guarda alegações como nós com camada, tipo de incerteza, definição formal e código.
- Arestas de patologia marcam desalinhamentos entre camadas.
- As cinco lentes são perguntas transversais com grandezas próprias.

### Exercícios

1. *(Conceito)* Escreva um nó L1 e um nó L2 para a alegação do Lab 0, "com essa média de demanda, a espera média será de uns 9 minutos" (Capítulo 10). Que aresta de patologia os liga?
2. *(Conceito)* Que lentes um modelo de concessão de crédito publicado aos clientes deve acionar? Justifique cada uma.
3. *(Conceito)* Por que a fórmula de Erlang C foi tipada como knightiana e não como epistêmica?

## Capítulo 7. Os agentes F1–F8, o orquestrador M15 e o ciclo de seis estágios

**Objetivos.** Saber o papel e a entrega obrigatória de cada agente; percorrer o ciclo de seis estágios; e entender por que a banca adversarial ataca o artefato em vez de ler o relatório.

### 7.1 Por que agentes especializados

O trabalho é dividido entre oito faixas coordenadas por um orquestrador. A divisão não é burocrática: cada faixa corresponde a um modo próprio de errar, e um especialista que só olha aquele modo o vê melhor. Os agentes não conversam em linguagem livre, que é ambígua e alucina; trocam fichas estruturadas em YAML (Capítulo 8). E nenhum aprova o próprio trabalho. Atenção ao nome: "Agente", com maiúscula, é a camada L3 do hipergrafo, quem decide no Mundo; os agentes F1–F8 são os módulos do programa que fazem a pesquisa. A tabela abrevia as entregas. O AGENTS.md pede ainda, entre outras, cProfile e tracemalloc no F1, a linhagem dos dados no F2, o versionamento triplo no F3, R̂ < 1,01 e ESS no F4, k-anonimato no F5, perguntas abertas antes das fechadas no F7 e o mapa de resistências organizacionais no F8. Na pesquisa, porém, os agentes são módulos de um mesmo programa, escritos pelo mesmo autor: a separação impede que um passo aprove o próprio resultado, mas não substitui revisores independentes.

| Agente | Papel | Entrega obrigatória |
| --- | --- | --- |
| F1 Código | Transforma hipóteses em processos geradores executáveis | Semente fixa, contratos de pré e pós-condição, testes de propriedades com Hypothesis |
| F2 Dados | Audita a entrada do Mundo | datacontract.yaml, quarentena de registros defeituosos, ingestão idempotente, orçamento de bytes |
| F3 Ciclo de vida | Testa o modelo no tempo | Corte temporal sem vazamento, sensores de deriva, política de ignorância, post-mortem |
| F4 Fronteira | Roda a inferência pesada | Diagnóstico de cadeias, calibração por simulação (SBC), predição conforme |
| F5 Governança | Cuida de legalidade e privacidade | Parecer de impacto (EU AI Act, LGPD), privacidade diferencial, responsável humano nomeado |
| F6 Justiça | Pergunta quem paga pelo erro | Auditoria de taxas de erro entre grupos e o trilema da justiça |
| F7 Domínio | Traduz problemas de outras áreas | Glossário, elicitação SHELF, termo de escopo |
| F8 Decisão | Converte o resultado em ação | Frequências naturais, HOPs, árvore C/L, cláusula de saída |
| M15 Orquestrador | Coordena e faz a banca adversarial | Ficha da crítica de modelos e ataque ao artefato antes da aprovação |

### 7.2 O ofício de cada faixa

**F2, o grão.** O agente de dados começa por uma decisão que ninguém vê: o que é uma linha. Uma linha por pessoa, por dia ou por evento muda a média, a variância e a margem, e por isso o grão é declarado e provado com uma contagem. O mesmo agente caça o valor sentinela, o ausente gravado como 0, −1 ou 9999 que entra na soma como se fosse dado, e exige ingestão idempotente: rodar a carga duas vezes tem de dar o mesmo número.

Falta à arquitetura uma pergunta que o F2 deveria fazer: **por que o dado está ausente?** O contrato de dados limita a nulidade, mas não classifica o mecanismo da ausência (MCAR, MAR ou MNAR, Capítulo 3), e é essa classe que decide se o descarte dos casos incompletos é inofensivo. Fica registrada como lacuna (Capítulo 14).

**F4, a amostra honesta.** Quando a posterior não tem forma fechada, o computador devolve uma amostra, não uma resposta. O diagnóstico de convergência mede se várias cadeias concordam entre si, não se o modelo está certo. O teste mais forte é a calibração por simulação: sorteia-se um parâmetro da priori, simulam-se dados com ele e roda-se o amostrador. Se o algoritmo é honesto, a posição do valor verdadeiro entre as L amostras da posterior é uniforme:

```latex
\operatorname{rank}\big(\tilde\theta;\,\theta^{(1)},\dots,\theta^{(L)}\big)\sim \operatorname{Uniforme}\{0,\dots,L\}
```

**F5, o dado pessoal.** Dado pessoal não é um tipo de arquivo: é a combinação de campos que decide, e CEP, data de nascimento e sexo, juntos, identificam provavelmente 87% das pessoas nos Estados Unidos, pelo censo de 1990 (Sweeney, 2000). A privacidade diferencial limita quanto a presença de uma pessoa pode mudar qualquer resposta; para bases D e D′ que diferem em uma pessoa,

```latex
\Pr[M(D)\in S]\ \le\ e^{\varepsilon}\,\Pr[M(D')\in S]
```

e o ε é o orçamento, gasto a cada consulta. A medida mais barata continua sendo não coletar.

**F3, a política de ignorância.** Todo modelo em operação declara o que faz em três casos: entrada fora do domínio, campo ausente e confiança baixa. As opções são recusar, ativar um fallback determinístico seguro ou devolver incerteza calibrada. Um modelo em uso piora por razões diferentes que pedem respostas diferentes: deriva de dado (o mundo mudou), deriva de conceito (mudou a relação entre atributo e desfecho) e performatividade (o modelo mudou o mundo). Há ainda uma quarta situação, que não é piora: a métrica cai porque a população atendida mudou.

A deriva de dado tem ela mesma duas formas, e a correção muda conforme a forma. Na **deriva de covariável**, muda a distribuição das entradas P(X), e a relação P(Y | X) continua valendo: basta reponderar. Na **deriva de rótulo**, muda a frequência dos desfechos P(Y), e P(X | Y) continua valendo: basta reajustar as taxas-base. Na **deriva de conceito**, muda P(Y | X), e nenhuma reponderação salva o modelo (Moreno-Torres e colegas, 2012).

A correção da deriva de rótulo tem forma fechada. Saerens, Latinne e Decaestecker (2002) mostraram que, se as densidades P(X | Y) de cada classe não mudam, a probabilidade corrigida é a antiga multiplicada pela razão entre a taxa-base nova e a antiga, depois renormalizada. Com dois desfechos, segue da fórmula que a correção soma uma constante ao logito: é uma correção de intercepto. A literatura clínica usa a mesma correção quando a incidência do desfecho difere entre a população em que o modelo foi feito e aquela em que é aplicado, caso em que todos os riscos saem sistematicamente altos ou baixos (Janssen e colegas, 2009). A hierarquia de Van Calster e colegas (2016) separa os dois níveis que o H3 encontrou: a calibração na média, medida pelo intercepto de calibração, e a calibração fraca, que exige intercepto 0 e inclinação 1.

No H3, os voos de 2023 passaram com a correção do intercepto, e as prisões de Toronto, com erro de inclinação, não (Capítulo 13). Mas o trimestre de 2023 não repetiu o desvio de 2013, e o modelo cru, sem correção, também teria passado (h3/README.md): a passagem dos voos não é evidência a favor da correção nem de uma deriva de rótulo. A leitura do desvio de 2013 como deriva de rótulo fica em aberto, porque o intercepto conserta qualquer deslocamento da média, venha ele de onde vier. Um teste ajudaria: Lipton, Wang e Smola (2018) estimam P(Y) no período novo pela matriz de confusão do modelo, sem rótulos novos e supondo P(X | Y) fixo, e testam se ela mudou. A auditoria rodou os dois. Pelo critério fixado antes — um alarme novo no cofre antes do primeiro alarme do sensor original, em 16 de outubro de 2013 — a resposta é não: o teste da média ligou no cofre em 6 de novembro de 2013 e o BBSE em 21 de novembro. Pior que chegar tarde, o teste da média fica ligado em 95% das verificações daquele ano: com a janela de três dias do disjuntor, ele mede a variação de um dia ruim, não a deriva do trimestre. E a hipótese do BBSE, um classificador fixo, não vale onde o modelo é reajustado todo dia: ele fica ligado em 99% a 100% das verificações do cofre nos voos, em 71% a 82% no Rhode Island e em Toronto de 2020 e 2021 e em cerca de 20% nos outros fluxos.

**F7, pedir bem.** Quando um parâmetro não vem dos dados, vem de alguém. O protocolo de Sheffield (SHELF) não pede ao especialista uma média: pede a mediana, os quartis e os extremos plausíveis, ajusta uma distribuição a esses quantis e a devolve para que ele a confirme, controlando a âncora do entrevistador, a ordem dos cenários e o excesso de confiança. Na pesquisa, nenhuma elicitação foi feita com especialista humano. O que a auditoria mediu, e é o que se pode medir sem painel, foi a sensibilidade à escolha da priori: em 33 domínios, a priori do pacote foi trocada pela de literatura e por uma deliberadamente fraca; cinco domínios mudam de status ou de ação com a de literatura e dois com a fraca, e entre eles está a aprovação do projeto de cauda que o H1 havia reprovado. Duas das cinco vêm de fonte construída sobre a própria base do domínio, o caso de dupla contagem que a declaração previa marcar. A conclusão não é que as prioris simuladas estivessem erradas; é que em cinco de 33 domínios a decisão não sobrevive à troca da priori. O painel humano continua em aberto (seções 9.7 e 14.4).

### 7.3 O ciclo de seis estágios

Uma pergunta percorre seis estágios, cada um com suas fichas de saída. A lista abaixo é abreviada: o AGENTS.md pede também a ficha de reconstrução no primeiro estágio e o previsor reserva no terceiro.

1. **Tradução e elicitação (F7).** Ficha da transferência, elicitação SHELF, termo de escopo.
2. **Ingestão, grão, causalidade e ética (F2, F5, F6).** Contrato de dados, ficha da amostra, grafo causal com o degrau de Pearl, parecer de privacidade, auditoria de justiça.
3. **Simulação, modelo nulo e fronteira (F1, F4, F3).** Processo gerador executável, modelo nulo simulado antes de ler qualquer padrão, diagnósticos de inferência, partição fora do tempo e contagem de tentativas.
4. **Projeção nas cinco lentes (M15).** Varredura de Fluxonomia, Crononomia, Escalonomia, Morfonomia e Crisonomia.
5. **Banca adversarial (M15).** Reprovado volta ao estágio que falhou; aprovado segue.
6. **Decisão e fechamento (F8).** Memorando em frequências naturais, HOPs, árvore C/L e cláusula de saída.

&#91;embedded content: ciclo de pesquisa · 6 estágios, 1 laço de volta\]

O único caminho até a decisão passa pela banca; um trabalho reprovado volta ao estágio em que falhou, não ao começo.

### 7.4 A banca adversarial

A banca não lê relatórios: ataca o artefato ao vivo. Três falhas eliminam o trabalho antes de qualquer discussão de mérito: vazamento temporal, contrato de dados violado sem tratamento e ausência de um responsável humano nomeado. Depois, cinco faixas recebem ataques específicos; as de código, governança e domínio não têm ataque próprio definido.

| Faixa atacada | O ataque |
| --- | --- |
| Dados | Um lote com registros corrompidos, que precisa ser isolado |
| Modelo em operação | Um lote com deriva forte, ao vivo |
| Inferência | Um conjunto de dados patológico, para ver se a incerteza denuncia a inadequação |
| Justiça | Inverter a métrica escolhida e mostrar quem passaria a pagar |
| Decisão | Uma crise que viola a própria cláusula de saída |

Um trabalho completo com uma faixa fraca declarada vale mais que um trabalho uniforme sem lacuna admitida. No H1 real, foi a banca que reprovou o projeto dos incêndios dinamarqueses (Capítulo 11): a alegação perdeu, mas o modelo rival não ganhou o direito de decidir.

**A ficha da crítica de modelos.** O orquestrador só aprova depois de preencher cinco itens, na versão do contrato dos agentes (o AGENTS.md; o PLAN.md fala em seis perguntas, e o livro segue o AGENTS.md): checagem preditiva contra réplicas do modelo ajustado; comparação com pelo menos um modelo rival; separação entre parâmetros identificados pelos dados e trazidos de fora; análise de performatividade, Goodhart e Lucas; e defesa das premissas indecidíveis pelo custo do erro.

### Resumo

- Oito faixas, cada uma ligada a um modo de errar, mais um orquestrador que ataca.
- O ciclo tem seis estágios e um laço de volta a partir da banca.
- Nenhum agente aprova o próprio trabalho; a banca ataca o artefato.

### Exercícios

1. *(Conceito)* Uma base tem uma linha por consulta médica, e a pergunta é sobre pacientes. Que agente deve barrar a análise e com que teste?
2. *(Conceito)* Desenhe o ataque da banca para um modelo de triagem de crédito. Que crise violaria a sua cláusula de saída?
3. *(Laboratório)* Implemente a calibração por simulação para um modelo Beta-Binomial e verifique o histograma de postos. Depois introduza um erro no amostrador e veja o histograma deixar de ser plano.
4. *(Conceito)* Dê um exemplo de cada um dos três modos de piora de um modelo em uso e da quarta situação, que não é piora, e diga a resposta certa para cada.

## Capítulo 8. Fichas, invariantes Φ1–Φ4 e portões

**Objetivos.** Ler e escrever uma ficha epistemológica; enunciar os quatro invariantes e as quatro proibições; percorrer o funil de quatro filtros; e saber o que cada um dos cinco portões automáticos verifica.

### 8.1 A ficha: uma lista fechada de perguntas

Uma ficha não é um resumo: é uma lista fechada de perguntas que se faz a um texto, a um número ou a um modelo. Todo payload que circula no barramento abre com um cabeçalho de cinco perguntas obrigatórias.

| Campo | Pergunta |
| --- | --- |
| processo\_gerador | Qual mecanismo oculto gera os dados, escrito de modo simulável, e em que camada a alegação pode falhar? |
| nao\_linearidade | O resultado é função não linear de algo que varia? Compare f(E\[X\]) com E\[f(X)\]; a dinâmica é aditiva ou multiplicativa? |
| unidade\_e\_margem | Qual é a unidade de fato independente, e a margem foi calculada com ela? |
| fisica\_do\_ruido | Que ruído é esse (gaussiano ou com saltos)? Há estados absorventes? O agente é passivo ou deforma o mundo? |
| escada\_causal | Em que degrau de Pearl está a afirmação, com que estratégia de identificação e que premissa não testável? |

O cabeçalho real do H0 em domínio real, abreviado (h0-real/ficha.yaml):

```yaml
alegacao: >
  "Escalando cada meia hora pelo Erlang C sobre a taxa média do histórico, a central atende
  82% das chamadas em até 20 s, com espera média de 10 s, todo dia."
cabecalho:
  nao_linearidade: >
    O nível de serviço do Erlang C é côncavo e despenca perto da saturação: aplicada à taxa
    real de cada dia do cofre, a própria fórmula da alegação dá em média 57%, não 82%.
  unidade_e_margem: >
    A unidade independente é o DIA: as chamadas de um dia compartilham a taxa e a fila.
    n_efetivo = 81 dias de cofre, não 2,65 milhões de chamadas.
  fisica_do_ruido: >
    A demanda diária NÃO é Poisson: dispersão Var/média = 273 (Poisson: 1).
  escada_causal: >
    Degrau 2 (intervenção): as regras de escala são do(escala) simuladas sobre a demanda real.
    Premissa não testável: a demanda não reage à escala.
```

Cada módulo do curso acrescenta perguntas às cinco do cabeçalho. A ficha da evidência pede a taxa-base, as duas verossimilhanças e a posterior calculada de dois modos, em chances e numa árvore de 10 000 casos, que precisam coincidir. A do grafo causal começa pelo degrau. A da crítica de modelos fecha a série e diz em que item cada ficha anterior entra.

Uma pergunta a mais entrou depois da auditoria de fundo, e ela não é do cabeçalho: todo pacote de domínio passou a
trazer, obrigatório, o campo `pergunta_do_decisor`, com três partes na ordem — quem decide, qual ação a decisão muda
(copiada literalmente do campo `decisao.acao` do próprio pacote) e a pergunta que essa decisão usa, terminada em
interrogação. A forma é validada no código, que recusa texto genérico sem quem decide e ação que não seja a do pacote;
os 18 domínios do H2g, os 12 do h2 e a ficha do H0, que não tem bloco de decisão e copia a ação do termo de escopo,
carregam o campo. É o que ataca o erro de tipo III: que a pergunta que o modelo responde seja a que o decisor fez deixa
de ser suposição e passa a ser uma frase conferível.

**A física do ruído, em detalhe.** O quarto campo do cabeçalho pede três classificações do ruído.

| Pergunta | Opções | Por que importa |
| --- | --- | --- |
| Que forma tem o ruído? | Gaussiano (contínuo) ou com saltos | Um modelo gaussiano não tem o 19 de outubro de 1987 no seu espaço de hipóteses |
| Como o ruído se integra? | Itô ou Stratonovich | Quando o ruído depende do estado, as duas leituras dão derivas diferentes |
| Há estados absorventes? | Sim (ruína, morte, falência) ou não | Num estado absorvente, a média de conjunto deixa de descrever qualquer trajetória |

A segunda pergunta parece técnica e não é. Num crescimento multiplicativo com ruído proporcional, dX = μX dt + σX dW, lido à maneira de Itô, a esperança cresce à taxa μ, mas a trajetória típica cresce à taxa

```latex
\frac{d}{dt}\,\mathbb{E}[\ln X_t] = \mu - \tfrac{1}{2}\sigma^2
```

É o arrasto da volatilidade do Dow Jones alavancado (Capítulo 11), e é Jensen outra vez. Lido à maneira de Stratonovich, o mesmo símbolo descreve outro processo, em que o termo −σ²/2 não aparece. A escolha depende da física: ruído branco idealizado de um sistema discreto pede Itô; o limite de um ruído real com correlação curta pede Stratonovich. A ficha pede também, à parte, um teste com passo de tempo menor: se o resultado muda quando Δt encolhe, a simulação estava medindo o integrador, não o Mundo. Esse teste não decide entre Itô e Stratonovich: o esquema de Euler converge para a leitura de Itô, e a de Stratonovich pede outro integrador.

O ruído também faz saltar entre estados estáveis. Num sistema com dois poços, o tempo médio até o salto segue a fórmula de Kramers e cresce exponencialmente com a altura da barreira ΔU em relação à temperatura T:

```latex
\tau \approx \frac{2\pi}{\sqrt{U''(x_{\min})\,\lvert U''(x_{\max})\rvert}}\; e^{\Delta U/T}
```

Para barreiras altas, esse tempo quase não tem memória: quem esperou muito não está mais perto do salto. Olhado só nos saltos, porém, o ruído parece um empurrão: em 591 saltos simulados no curso, o ruído logo antes do salto apontou no sentido dele em 99% dos casos, sem nenhuma força externa. Procurar a causa de um salto é um erro de seleção; procurar a causa de uma mudança na frequência dos saltos, não.

### 8.2 As quatro proibições

O contrato dos agentes (AGENTS.md) começa por quatro proibições absolutas.

1. **Linguagem natural livre entre agentes.** A comunicação é só por fichas estruturadas.
2. **A média solitária.** Quando a função é não linear, a dinâmica multiplicativa ou a cauda pesada, é proibido reportar só a média. Reportam-se o caso médio f(E\[X\]), a média simulada E\[f(X)\], a mediana, os quantis e a curva acumulada.
3. **Confundir aprendizado com performatividade.** Erro de previsão pequeno não prova que o modelo é verdadeiro; roda-se o modelo com e sem a ação.
4. **Extremos dogmáticos e omissão de tentativas.** Nenhuma previsão declara 0% ou 100%; toda previsão é pontuada por regra própria fora do ajuste; e declara-se quantas especificações foram tentadas.

### 8.3 Os quatro invariantes

Nada entra no grafo sem passar por quatro invariantes, que funcionam como travas de banco de dados.

| Invariante | O que exige | O que barra |
| --- | --- | --- |
| Φ1 Simulabilidade | Toda alegação vira código com semente declarada e testes de propriedades | Argumento só em prosa |
| Φ2 Margem e unidade | Todo número vem com margem calculada na unidade de fato independente | Pseudorreplicação, duplicatas, valores sentinela |
| Φ3 Diagnóstico e teste severo | Auditar o filtro da amostra, desenhar o grafo causal e confrontar a hipótese com um modelo nulo simulado | Padrão lido antes do nulo, colisor, Simpson |
| Φ4 Escopo ecológico | Julgar a decisão no ambiente em que vai operar, com termo de escopo e cláusula objetiva de saída | Recomendação sem prazo de validade |

### 8.4 O funil de quatro filtros

Um sistema que simula tudo o que encontra colapsa por custo e por inflação de falsas descobertas. O plano organiza cada frente como um funil em que a maior parte das hipóteses fracas cai antes de tocar em simulação pesada.

1. **Valor da informação.** Corta-se o que, mesmo com resposta perfeita, não mudaria nenhuma decisão. O log padrão é "podado por Crisonomia: curiosidade sem uso operacional".
2. **Poda topológica e unidade.** Corta-se, por d-separação (M13), o que fica fora do cobertor de Markov da variável de interesse (M11), inclusive os colisores que, se condicionados, criariam o viés de Berkson. Corta-se também o que os dados não identificam ou cujo tamanho efetivo, contado na unidade independente pelo F2, não permite conclusão. O contrato de dados que esse filtro confere ganhou, depois da auditoria, um bloco `ausencia` para o mecanismo da falta: `MCAR`, `MAR`, `MNAR` ou não declarado, com as colunas que a classificação cobre e a análise de sensibilidade exigida quando a ausência pode depender do próprio valor. Ausência sem classificação é recusada; nos três contratos do H1 real e nas 18 tabelas do H2g a regra é vacuosa hoje — nenhuma coluna tem um único valor ausente —, e passa a morder no dia em que aparecer.
3. **Teste severo contra o nulo.** Se o padrão alegado já aparece no modelo nulo, ou numa referência de um parâmetro só, como a taxa-base (M12), a pesquisa encerra ali. Quando a simulação é cara, o F4 faz esse teste primeiro num emulador barato.
4. **Inferência pesada, lentes e banca.** Só o que sobra recebe o tratamento caro.

**Caso trabalhado: a identificabilidade de R₀.** No início de uma epidemia, os casos crescem como exp((β − γ)t), e os dados determinam a diferença β − γ, não a razão R₀ = β/γ. Com 25 dias de casos simulados, em que a verdade é R₀ = 2, os valores compatíveis vão de 1,38 a 5,90, e o tamanho final previsto vai de 50% a 100% da população. Com 45 dias, R₀ fica entre 2,00 e 2,24, mas aí a curva já dobrou. Tomar a duração da infecção de outros estudos estreita R₀ para 1,81 a 2,11. Isso não é defeito, desde que se declare que a margem passou a ser a da fonte.

### 8.5 Os cinco portões automáticos

Nenhuma entrega é consolidada sem passar em cinco portões numa máquina limpa.

| Portão | Faixa | O que verifica |
| --- | --- | --- |
| 1 | F1 | Contratos, semente declarada, invariantes estocásticos (probabilidades em \[0, 1\], somas 1, monotonia) |
| 2 | F2 | Esquema declarativo, chave primária, idempotência, limite de bytes |
| 3 | F4 | Convergência das cadeias, calibração por simulação, cobertura conforme |
| 4 | F3 | Corte temporal sem vazamento, insensibilidade ao passo de tempo, fallback, invariância |
| 5 | F5, F6, F8 | Parecer de governança, trilema da justiça, cláusula de saída C/L |

Na prática, cada horizonte da pesquisa tem um `portoes.py` que repete a execução, compara os hashes dos resultados e aplica os critérios do horizonte. Os cinco portões acima foram implementados como tais no H1; no H0 a esteira aplica Φ1–Φ4, e no H2 e no H3 cada portoes.py aplica os critérios de saída do horizonte (G1–G5 ou G1–G7), com a segunda execução pedida por --reproducao. O H0 simulado, por exemplo, grava um `resultados.json` com o hash SHA-256 das saídas.

### Resumo

- A ficha é uma lista fechada de perguntas; cinco são obrigatórias em todo payload.
- Quatro proibições e quatro invariantes são travas, não recomendações.
- O funil corta barato antes de gastar caro.
- Cinco portões automáticos decidem se algo entra.

### Exercícios

1. *(Conceito)* Preencha as cinco perguntas do cabeçalho para a alegação "alunos que estudam com música tiram notas maiores".
2. *(Conceito)* Que filtro do funil deve cortar a pergunta "qual é a cor preferida dos clientes que compram seguro?" numa seguradora que não vai mudar o produto?
3. *(Conta)* Por que 25 dias de crescimento exponencial identificam β − γ e não β/γ? Mostre que dois pares (β, γ) com a mesma diferença geram a mesma curva inicial.
4. *(Laboratório)* Escreva um teste com Hypothesis que verifique que a espera simulada de uma fila é sempre não negativa e não diminui quando o tempo de atendimento aumenta, com as mesmas chegadas.

## Capítulo 9. Governança: pré-registro, cofre, diário e assinaturas

**Objetivos.** Explicar como a arquitetura impede o sobreajuste em escala; descrever o pré-registro com hash, o contador de tentativas, o cofre fora do tempo e o diário de previsões; e entender o papel e os limites dos pareceres de risco assinados.

### 9.1 O risco de escala é epistemológico

Quando um sistema testa centenas de hipóteses e especificações em paralelo, o maior risco não é técnico. É o sobreajuste em escala industrial: o jardim dos caminhos que se bifurcam (Capítulo 3) percorrido por máquinas incansáveis. O plano impõe três mecanismos de contenção.

### 9.2 Pré-registro com hash e contador de tentativas

Antes que qualquer agente acesse a partição de teste, a hipótese é congelada com hash. Na pesquisa, toda mudança de critério, de teste ou de dados foi escrita num arquivo de declaração, com o seu SHA-256 gravado antes de rodar; exemplos são `h2b/DECLARACAO_K2.md` e `h3/CRITERIOS_H3d.md`. Nenhuma regra foi mudada depois de ver o resultado da rodada a que se aplicava, mas cada rodada nova foi escrita sabendo das anteriores, e as correções feitas depois de ver resultados ficaram registradas e contam no K (Capítulos 12 e 13).

O sistema mantém um contador K de especificações tentadas por pergunta. Toda tentativa falha fica registrada, e o relatório final é obrigado a declarar K. O PLAN.md pede mais: que K penalize a complexidade, pelo comprimento mínimo de descrição (MDL) ou pelo fator de Occam, junto com a correção para comparações múltiplas do M5. Na pesquisa, K foi declarado, mas nenhuma dessas penalidades foi aplicada (Capítulo 12). Foi por esse contador que a respecificação dos domínios do H2 parou em K = 3, e o passo seguinte foi buscar dados novos (Capítulo 12). O contador por domínio não registra, porém, que o portão foi tentado em sete rodadas, e nenhuma correção por isso foi aplicada.

### 9.3 O cofre fora do tempo

A partição temporal futura fica sob custódia exclusiva do agente de ciclo de vida (F3). Nenhum agente de modelagem a lê durante o desenvolvimento; ela é aberta uma única vez para pontuar calibração e verificar ausência de vazamento. Quando um cofre já foi aberto e o modelo precisa de correção, a correção só vale testada num cofre novo, que ninguém viu. Foi o que o H3 fez três vezes: com os voos de 2023, com as prisões de Toronto de 2020–2021 e com as paradas de trânsito de Rhode Island (Capítulo 13).

### 9.4 O diário de previsões e o barramento

Cada agente que emite previsões probabilísticas as grava num livro-razão contínuo, restrito ao intervalo \[1%, 99%\]. Para cada previsão o diário guarda a fonte e o filtro, se o próprio sistema pode influir no resultado, o número de fatores da justificativa e o degrau de Pearl. Agentes cujo escore de Brier perde para a taxa-base, ou cujo capital de Kelly colapsa, têm o peso reduzido pelo orquestrador.

O barramento de fichas é encadeado por hash: cada mensagem carrega o hash da anterior, e a trilha não pode ser reescrita sem quebrar a cadeia. Uma lição de implementação veio cedo. No H1, o placar do diário somava o mesmo previsor entre projetos diferentes, e o erro foi corrigido e declarado no README; os números do H1 publicados depois disso já vêm da versão corrigida.

### 9.5 Pareceres de risco e o responsável nomeado

O agente de governança (F5) emite um parecer de risco para cada projeto avaliado, inclusive os que não chegam à decisão. Na pesquisa foram 21: 18 do H2 e 3 do H1. Quatro deles envolvem decisões sobre pessoas: Direito (regra de indenização trabalhista), Educação (turmas do STAR), Saúde (o experimento de seguro da RAND) e o programa de treinamento de LaLonde.

Os 21 pareceres foram assinados por sil em 15 de outubro de 2026, depois de ler um resumo deles que destacava os quatro que envolvem pessoas. A assinatura fica num registro separado (`governanca/ASSINATURAS.md`), em que cada parecer é identificado pelo SHA-256 do arquivo no momento da assinatura. Os arquivos de resultado não foram reescritos, para não mudar as saídas que a reprodução limpa compara. O parecer do Direito carrega uma ressalva factual: a recomendação de adotar um corte de 10% vem de uma decisão e de um custo inventados no F1, e não é conselho de política.

### 9.6 O limite: o auditor é parte interessada

Os pareceres foram assinados por quem conduz o programa. Uma auditoria autorizada por quem é auditado tem valor limitado. O responsável nomeado responde à pergunta que uma auditoria precisa responder (quem decidiu usar, quem aprovou, quem olha o placar, quem desliga), mas nomear um responsável não torna a auditoria independente. O que o programa pode fazer, e ainda não fez, é registrar o que a auditoria não pôde olhar e quais grupos afetados foram ouvidos, ou declarar que nenhum foi.

### 9.7 A auditoria de fundo: o que foi suposto e o que se mediu

A governança também serve para marcar o que não foi medido, e para depois medir. A pesquisa se vira contra si mesma
numa auditoria de fundo: dez pontos que o texto sustentava e que nunca haviam sido postos à prova viraram dez frentes de
trabalho na pasta `fundo/` do repositório. Cada frente segue a mesma regra dos horizontes, e é ela que faz a prática
funcionar — a declaração com hash vem antes de rodar, o resultado é relatado com os limites, e mudança de critério
depois de ver o resultado entra como declaração nova e conta no número de tentativas. A tabela diz o que cada ponto
perguntou e o que ele respondeu.

| Ponto | A suposição | O que foi medido | Resultado |
| --- | --- | --- | --- |
| 1. Tese central | A arquitetura pega erros que a prática comum deixa passar | Piloto cego de 12 casos, dois braços | Empate no teto: 8 de 8 defeitos nos dois, 0 alarme falso nos 4 casos limpos — a tese não fica sustentada |
| 2. Independência | As saídas aprovadas não dependem de quem as escreveu | Quatro resultados reimplementados só a partir da especificação | Todos concordam; zero erros no original; várias escolhas que a especificação não fixa coincidem com as do autor, três delas só no STAR (evidência fraca) |
| 3 e 4. Multiplicidade | O portão do H2 foi tentado poucas vezes, e nada foi ajustado depois | Inventário das tentativas e das aberturas de cofre | K = 38 testes distintos (32 a 72); com Bonferroni o H2g mantém as 4 áreas de então — antes da correção de agrupamento do ponto 7 — e os conjuntos encolhem |
| 5. Prioris | As prioris de literatura não foram conferidas com especialistas; a rodada de controle não tinha tolerância declarada | Três rodadas em 33 domínios (pacote, literatura e fraca) e a banda de equivalência de 0,05 dp da `DECLARACAO_2` | Cinco domínios mudam de status ou ação com a de literatura e dois com a fraca; o veredito deixa de ser condicional — desvio máximo de 0,0071 dp e **33 dos 33** domínios idênticos |
| 6. Laço performativo | Quem desiste da fila volta a chamar com probabilidade 0,6 | Desenho quase experimental numa central real, 1999 | 0,145 [0,132; 0,157] — quatro vezes menos que o declarado |
| 7. Linhas dependentes | O ICP supõe linhas independentes, e há 16 instantes por voluntário e 80 escolas | Wald agrupado e permutação em blocos; réplica da Genética com 40 genes novos e taxa nula por permutação do rótulo do mutante (B = 999) | A Farmacologia sobrevive intacta, a Educação sai; a Genética se replica em um gene validado, com p = 0,001 da taxa nula |
| 8. Calibração | A tolerância do ECE pode ser fixa em 0,03 | Nulo simulado com as mesmas previsões e o mesmo n | Nenhum dos seis cofres binários do H3 passa pela regra que depende de n; pela regra secundária, o quadro não muda |
| 9 e 10. Escala e dados | O funil escala e o grafo mede conhecimento | 329 domínios em lote; dois domínios novos de saúde | Dois podados no teste severo; 327 até o fim; 26 634 nós, dos quais 6 494 de conceito |
| Evento raro | Um previsor de evento raro serve para decidir quando tem boa calibração | Decomposição de Brier em três cofres: indometacina, COVID e Rhode Island | Nenhum caminho vence nos três: só em Rhode Island RES − REL fica acima de zero (+2,74643e-4 na logística, +3,11655e-4 no ranking), e a habilidade de Brier fica em +0,018629 e +0,017952, abaixo do +0,05 declarado; no COVID a AUC é 0,7685 e a habilidade é −0,020853 |
| Saúde | O círculo que o Lab 0 abriu se fecha com dado real de saúde, em espelho aberto | PhysioNet/CinC Challenge 2019 (sepse precoce) no espelho do PhysioNet: o funil do H2g rodou de ponta a ponta, sem uma linha mudada, com o cofre no segundo sistema hospitalar | O efeito existe e é preciso (β = 0,0388 em log-razão de chances por respiração por minuto, IC90 0,0257 a 0,0523), mas a invariância não: 0 de 512 subconjuntos aceitos, Ŝ = ∅, e o G3 falha; a habilidade no cofre é +0,00568 e a taxa de decisão registrada é 8,4% nas internações de um sistema e 3,9% nas do outro |
| Ficha e contrato | A pergunta que o modelo responde é a que o decisor fez; o contrato de dados não diz por que o dado falta | Campo obrigatório `pergunta_do_decisor` no pacote de domínio (18 do H2g, 12 do h2 e a ficha do H0) e bloco `ausencia` (MCAR, MAR, MNAR ou não declarado) no contrato do F2 | Os dois campos entram validados no código; nos três contratos do H1 real e nas 18 tabelas do H2g a regra da ausência é vacuosa hoje — nenhuma coluna tem um único valor ausente — e nenhum número publicado muda |

Três desses resultados apertam o que os capítulos anteriores autorizam. O laço de 0,6 que o H3 corrigiu é cerca de
quatro vezes o que se mediu num caso real (Capítulos 5 e 13). A tolerância de 0,03 do erro de calibração é maior que
o ruído em todos os cofres grandes, e nenhum deles passa na regra que depende de n (Capítulos 3 e 13). E o portão de
transferência do H2 depende de qual versão de cada escolha conta: a área da Educação não sobrevive à correção de
agrupamento, e quem sustenta a terceira área agora é a Genética replicada, não a Educação (Capítulo 12). Nenhum número
publicado caiu por erro de conta; o que a auditoria estreita é o que esses números autorizam.

O que continua em aberto tem nome. A elicitação com especialistas humanos de verdade, porque a auditoria mediu a
sensibilidade às prioris e isso não substitui um painel — o protocolo e os seis formulários estão escritos em
`fundo/p12-shelf/`, com o critério de K = 2 pronto, e nenhuma entrevista foi feita. A taxa nula da validação por
intervenção da Genética, que decide com um único gene em 27: ela saiu em p = 0,001, o piso de 999 sorteios, e o que
segue aberto ali é a resolução, não o p. E o tempo de atendimento e a paciência dos clientes da central (Capítulo 10),
que a base não traz e que continuam supostos.

Fica também o limite da seção 9.6, que a auditoria não resolveu: um dos passos que ela mesma propõe é que sil releia
por inteiro os 21 pareceres assinados, um a um, e confirme ou retire a assinatura registrando que leu o texto integral.
Enquanto isso não acontece, a assinatura vale pelo que ela é: a declaração de quem conduz o programa de que leu um
resumo.

### Resumo

- Pré-registro com hash e contador K tornam as tentativas visíveis.
- O cofre fora do tempo só se abre uma vez; correções pedem cofre novo.
- O diário pontua cada previsor com regra própria e pesa os agentes pelo placar.
- Pareceres assinados nomeiam o responsável, mas não tornam a auditoria independente.
- A auditoria de fundo testou dez suposições do próprio programa, declaradas antes de rodar; a seção 9.7 diz o que cada uma respondeu.

### Exercícios

1. *(Conceito)* Por que abrir o cofre duas vezes destrói o seu valor? Relacione com a severidade de Mayo.
2. *(Conceito)* Um colega propõe reescrever os `resultados.json` com a assinatura dentro. Que propriedade da reprodução limpa isso quebraria?
3. *(Conceito)* Proponha um procedimento para tornar a auditoria dos quatro pareceres que envolvem pessoas menos dependente de quem conduz o programa.
4. *(Laboratório)* Implemente um barramento mínimo em que cada mensagem carrega o SHA-256 da anterior, e um verificador que detecta a alteração de qualquer mensagem.

# Parte III. A pesquisa real

Os quatro capítulos seguintes põem a arquitetura diante de dados reais, horizonte por horizonte, conforme o PLAN.md: o leitor acompanha o trabalho sendo feito, com os números que ele produziu. A pesquisa atravessou os quatro portões de saída, os dois últimos só depois de várias rodadas de falha, e foi encerrada em 15 de outubro de 2026.

| Horizonte | Escala prevista | Portão de saída | Resultado |
| --- | --- | --- | --- |
| H0 Semente | 1 domínio-piloto, cerca de 10² nós | Φ1 a Φ4 passando e reprodução determinística em máquina limpa | Passou, no simulado e no real |
| H1 Ciclo completo | 3 projetos, cerca de 10³ nós | Um projeto completo aprovado nos 5 portões e defendido contra a banca adversarial | Passou: A e B aprovados pela banca; os portões 4 e 5, calculados sobre os três projetos juntos, falharam por causa de C |
| H2 Transferência | 10 ou mais domínios | ICP com conjunto não vazio em 3 áreas | Passou na sexta rodada, com ressalva; a auditoria de fundo confirmou o portão com a Genética replicada e sem a Educação |
| H3 Malha contínua | Fluxos sob relógio | Calibração fora do tempo, zero vazamento, controle de Goodhart | Passou na terceira tentativa do fluxo mais difícil |

## Capítulo 10. H0: o posto de saúde e a central de chamadas

**Objetivos.** Acompanhar um ciclo completo dos invariantes num caso pequeno; ver Jensen cobrar sua conta numa fila; e entender como o modelo nulo arbitra a margem de erro.

### 10.1 O Lab 0: a mesma média, duas esperas

O primeiro teste foi deliberadamente modesto: um posto de saúde simulado, o Lab 0 do curso. O posto atende 12 pacientes por hora e recebe, em média, 8. A alegação é a de qualquer planilha de gestão: "com essa média de demanda, a espera média será de uns 9 minutos". O sistema traduziu a alegação num processo gerador com semente fixa (1909) e rodou cinco mundos com a mesma demanda média.

| Mundo (pacientes por hora) | Espera média | Pacientes com espera acima de 30 min |
| --- | --- | --- |
| 8 todo dia (o da alegação) | 9,3 min | 8 em 100 |
| 7 ou 9 | 10,5 min | 10 em 100 |
| uniforme entre 5 e 11 | 12,3 min | 13 em 100 |
| 5 ou 11 | 20,7 min | 25 em 100 |
| 5 ou 11, com reforço só nos dias de surto | 2,5 min | menos de 1 em 100 |

Nada mudou na média; mudou a variabilidade, e a fila é uma função convexa da demanda. A fórmula clássica de uma fila com um atendente,

```latex
W(\lambda)=\frac{\lambda}{\mu(\mu-\lambda)}
```

dá 10 minutos com λ = 8 e μ = 12. Aplicada a dias que alternam entre 5 e 11, daria 29,3 minutos; a simulação dá 20,7. A diferença ensina três coisas. Um dia de 10 horas não é o longo prazo da fórmula: nos dias de surto, a espera fica em 28,2 minutos, não nos 55 previstos. Cerca de 70% dos pacientes chegam nos dias de surto, de modo que a média por paciente pesa mais os dias ruins. E a fórmula não está errada: responde à espera de longo prazo com demanda constante, quando o gestor perguntava quantos esperam demais nos dias ruins. É um erro de tipo III (Capítulo 3): a resposta certa à pergunta errada.

**Os três resultados que vão além do exemplo.**

1. **O nulo arbitra a margem (Φ2).** Tratando cada paciente como independente, a margem é ±0,07 minuto; tratando o dia, ±0,51. O sistema repartiu ao acaso 4 000 dias simulados do mundo da alegação em duas metades e mediu a diferença entre elas: desvio de 0,216. A margem por dia previa 0,227; a por paciente, 0,047. A pseudorreplicação foi refutada pelos dados.
2. **O teste severo (Φ3).** A diferença entre os mundos, 11,4 minutos, fica a 53 desvios do nulo; em 2 000 repartições, o nulo nunca passou de 0,81.
3. **A armadilha da mediana.** No mundo com surtos, a mediana é 9,3 minutos, o mesmo número que a alegação promete como média. Quem olha "o paciente típico" vê a promessa cumprida. A diferença está toda na cauda.

Tudo passou pelos quatro portões e reproduziu os seis números que o material do curso publicava, com a mesma contagem de pacientes (161 867 e 160 220) e o mesmo hash nas duas execuções.

### 10.2 O H0 real: a central de chamadas de um banco

O Lab 0 supunha uma demanda que alterna entre 5 e 11. No H0 real, a demanda é observada: 164 dias úteis de 2003 de uma central bancária norte-americana, contados a cada 5 minutos. A escala sai de 83 dias de treino e é aplicada a 81 dias de cofre, com 2,65 milhões de chamadas. A alegação é a prática do setor: escalar cada meia hora pela fórmula de Erlang C sobre a taxa média do histórico garante 82% das chamadas atendidas em até 20 segundos, todo dia.

**O que a base mostrou.**

- **A demanda não é Poisson.** Entre dias, o desvio é de cerca de 2 900 chamadas, onde Poisson preveria 180; a razão variância/média é 273. Parte é calendário (segunda-feira tem +15%), parte é ruído residual com coeficiente de variação de 6%.
- **Jensen dentro da própria fórmula.** Aplicado à taxa real de cada dia, o Erlang C da alegação dá em média 57%, não 82%, porque o nível de serviço é côncavo na carga.
- **A unidade é o dia.** O nulo de metades deu desvio de 0,0474, contra 0,0470 previsto pelo dia e 0,0033 pela chamada.
- **Teste severo.** Em 400 dias simulados no mundo da alegação, nenhum ficou abaixo da meta. No real, cerca de um em quatro ficou.

**A decisão (Φ4).** O F8 comparou regras de escala no cofre, com as chegadas reais.

| Regra de escala | Agente-horas por dia | Dias abaixo da meta | Nível de serviço |
| --- | --- | --- | --- |
| R0 alegação: perfil médio | 2 264 | 26% | 86% |
| R1 perfil do dia da semana | 2 261 | 12% | 90% |
| R3 R0 + 12 atendentes por igual | 2 433 | 7,4% | 94% |
| R2 dia da semana + margem para a variação residual | 2 428 | 3,7% | 96% |

R2 e R3 custam quase o mesmo, mas R2 põe a folga onde está o risco. Em frequências naturais: de cada 100 dias, 26 falham com a escala da alegação, nenhum no mundo que ela descreve e cerca de 4 com R2.

**O que ficou suposto.** O tempo de atendimento (exponencial, média de 4 minutos) e a paciência dos clientes (exponencial, média de 3 minutos) não estão na base. A auditoria de fundo transformou essas duas suposições numa malha declarada de 4 × 6 pontos — atendimento de 3, 4, 5 e 6 minutos contra paciência de 1, 2, 3, 5, 7 e 10 minutos —, e o resultado é que a decisão não depende da escolha dentro dessa faixa: o R2 é a regra vencedora em 20 dos 24 pontos, e nos outros quatro, todos com 1 minuto de paciência, as quatro regras empatam, sem nenhuma delas deixar mais de 1 dia abaixo da meta. Não há ponto de virada, e a margem mais apertada fora do empate é de 2 dias do cofre, em 6 minutos de atendimento e 2 de paciência. O pior caso é o canto da malha, 5 minutos de atendimento com 10 de paciência: nem a melhor das quatro regras segura a meta, com 14,2% dos dias abaixo dela, contra 37,7% da regra da alegação no mesmo ponto — de cada 100 dias, 38 falham com a escala da alegação e 14 com o R2, que custa 7,2% a mais de agente-horas. A paciência governa o tamanho do efeito e o atendimento quase não: fixada a paciência, a fração do R2 anda no máximo 0,9 ponto percentual em todo o intervalo de atendimento, e a linha da alegação vai de 0,6% a 1,2% dos dias abaixo da meta com 1 minuto de paciência e de 37,7% a 38,6% com 10. O que a malha não mede é a família das distribuições, porque as duas seguem exponenciais com CV 1, nem a dependência entre paciência e atendimento. O domínio de saúde ficou de fora do Lab 0 porque os portais do DATASUS e de dados.gov.br estavam fora do alcance da rede; o círculo se fecha por outro caminho, no ponto 10 da auditoria de fundo (seção 9.7).

### Resumo

- A mesma demanda média pode dar 9 ou 21 minutos de espera; só quem simula o mundo vê a diferença.
- O modelo nulo de metades decide qual margem vale, sem discutir convenção.
- Em dados reais, a regra padrão falha em 26 de cada 100 dias, e gastar cerca de 7% a mais de agente-horas, onde está o risco, reduz isso a menos de 4.

### Exercícios

1. *(Conta)* Use W(λ) para conferir 10 minutos com λ = 8 e 29,3 minutos como média de W(5) e W(11).
2. *(Conceito)* Por que a média por paciente pesa mais os dias de surto? Calcule a fração de pacientes que chegam neles.
3. *(Laboratório)* Reproduza o Lab 0 com a recursão de Lindley e a semente 1909, e confira a espera média de 20,7 minutos no mundo 5 ou 11.
4. *(Conceito)* Escreva o memorando de decisão do H0 real para a diretoria em quatro frases, em frequências naturais, com cláusula de saída.

## Capítulo 11. H1: Dow Jones, LaLonde e os incêndios dinamarqueses

**Objetivos.** Ver o enxame completo F1–F8 trabalhar em três alegações reais; reconhecer a quebra de ergodicidade, o viés de seleção e a janela limpa enganosa; e entender por que uma alegação pode perder sem que o modelo rival ganhe.

O H1 ativou as oito faixas, o barramento com cadeia de hash, o diário de Brier e Kelly e a banca adversarial, que é o papel do orquestrador M15. Três projetos concorreram, cada um pensado para uma lente: A, aditivo contra multiplicativo (Crononomia); B, causal (Morfonomia); C, cauda pesada (Escalonomia). O portão de saída pedia um projeto aprovado. A e B foram aprovados; C foi reprovado pela banca, e os testes não foram ajustados para que passasse.

### 11.1 Projeto A: alavancar o Dow Jones (1885–2023)

**A alegação.** "Alavancado 3 vezes, o índice rende 3 vezes mais, e quem segura por décadas sempre ganha."

**A unidade.** O ano civil. Coortes de 20 anos se sobrepõem, e por isso as coortes do treino valem só cerca de 5,8 blocos independentes.

| Alavancagem | Crescimento temporal realizado (ao ano) | Coortes de 20 anos acima do ponto de partida |
| --- | --- | --- |
| 1× | 0,051 | 96% |
| 2× | 0,072 | 78% |
| 3× | 0,060 | 64% |

A soma aritmética daria cerca de 0,20 ao ano com 3×. O arrasto da volatilidade come 0,14 ao ano. Com 3×, a riqueza média das coortes chega a 19 vezes o capital, mas a mediana fica em 2,1: média de conjunto e trajetória típica se separam, como o Capítulo 5 previa. Os dias absorventes mostram por quê. Só o 19 de outubro de 1987 (−22,6%) tira 68% do capital alavancado 3 vezes. De 1929 a 1932, o capital cai a 0,11 com 1× e a 0,00025 com 3×.

**O teste severo.** O mundo da alegação prevê 93% das coortes 3× acima do ponto de partida; o real, com coortes de início anual, dá 65% (p = 0,01).

**O que a alegação conserva.** Sem custo de financiamento, 2× e 3× ainda crescem mais que 1× no tempo, e o Kelly da posterior é 2,35 \[1,26; 3,44\], intervalo de 90% da posterior. A refutação vale para "rende 3 vezes" e "sempre ganha", não para "alavancar nunca compensa". O cofre de 2000 a 2023 até favoreceu a alegação: o previsor sem arrasto teve habilidade 0,44, contra 0,40 do modelo. E a série é só de preço, sem dividendos e sem custo de financiamento, duas ausências que favorecem a alegação. Delimitar o que se refutou é parte do resultado.

### 11.2 Projeto B: o experimento de LaLonde

**A alegação.** Comparar participantes do programa de treinamento NSW com trabalhadores da pesquisa nacional CPS, crua ou ajustada, recupera o efeito do treino.

**O desfecho e o cofre.** Estar empregado em 1978. O cofre são os 260 controles sorteados do NSW, fora da amostra por construção.

| Método | Efeito no emprego (pontos) | Intervalo de 95% |
| --- | --- | --- |
| Comparação crua com o CPS | −10,7 | \[−17,0; −4,5\] |
| Ajuste por escore de propensão | +9,6 | \[−0,6; +20,8\] |
| Experimento sorteado | +11,1 | \[+2,6; +19,6\] |

**O ajuste acerta o ponto, mas não é severo.** Um tratamento inócuo daria +9,0, dentro da faixa do nulo, de −10,3 a +11,9 (p = 0,08). O ajuste se apoia em cerca de 103 controles efetivos, e 72% do CPS fica abaixo do menor escore dos tratados.

**O placebo.** Usando o emprego de 1975, anterior ao programa, como desfecho, o ajuste dá −12,3 pontos (p = 0,04), contra +8,5 no experimento, com intervalo de −0,6 a +17,5, compatível com zero. O placebo é uma estimativa direta do termo de viés de seleção da decomposição do Capítulo 4, e ele não deu zero. Quem procura um programa de treinamento estava em pior situação; o sorteio zera esse termo, o ajuste só tenta estimá-lo pelas variáveis observadas.

**Justiça (F6).** As taxas de falso negativo diferem por grupo: 0,72 para hispânicos, 0,50 para negros e 0,57 para os demais. Igualá-las capta mais 22 desempregados e custa 87 falsos positivos. É o trilema do Capítulo 5 com números.

### 11.3 Projeto C: a cauda dos incêndios dinamarqueses (1980–1990)

**A alegação.** No fim de 1988, uma resseguradora vê 36 meses sem sinistro na camada de 50 a 150 milhões de coroas, conclui que o risco é desprezível e escreve 3 vezes o capital. A retenção de 50 milhões foi escolhida com a série inteira à vista, e por isso o cofre não é cego para o analista; há uma tabela de sensibilidade para retenções de 20 a 50 milhões.

**A cauda.** O estimador de Hill dá α = 1,81 \[1,41; 2,56\] (intervalo de 95% por bootstrap em blocos): variância perto de infinita.

**A janela limpa como evidência.** Os 36 meses limpos valem só 2,2 bits. A regra de Laplace dá 1 chance em 38 por mês, não zero. Um teste severo precisaria de 69 meses limpos.

**A ruína.** Com a alavancagem que a alegação justificaria, a resseguradora é arruinada em agosto de 1989, por um sinistro de 152 milhões. Com a regra de Laplace, termina com 0,19 do capital.

**Por que a banca reprovou o projeto.** No cofre, nenhum previsor venceu a frequência histórica simples: o modelo teve habilidade −0,015, e a alegação, −0,052. A frequência de sinistros crescia 4,6% ao ano, e a posterior era dominada pela priori elicitada, que era simulada. **A alegação perde, mas o modelo não ganha o direito de decidir.** O F8 não emitiu decisão, e os portões 4 e 5 falharam por C.

Há um análogo exato no curso: a estratégia que vende seguro (Capítulo 5), em que 37% dos históricos de 250 semanas não mostram perda e 35,8% dos investidores que apostam pelo Kelly estimado se arruínam.

### 11.4 O que o H1 ensinou

- Uma refutação precisa dizer exatamente o que refuta (projeto A).
- Acertar o número não basta; o procedimento precisa poder ter errado (projeto B).
- Reprovar o próprio modelo é um resultado, não uma falha do programa (projeto C).

### Exercícios

1. *(Conta)* Com crescimento temporal de 0,060 ao ano, em quanto o capital se multiplica em 20 anos? Compare com a riqueza média de 19 vezes e com a mediana de 2,1, e explique as diferenças.
2. *(Conceito)* Por que o placebo com o emprego de 1975 é uma estimativa do viés de seleção?
3. *(Conta)* Mostre que 36 meses limpos dão 1/38 pela regra de Laplace.
4. *(Conceito)* Explique a frase "a alegação perde, mas o modelo não ganha o direito de decidir" para um gestor de risco.

## Capítulo 12. H2: invariância causal em domínios reais

**Objetivos.** Acompanhar um portão exigente que falhou cinco vezes antes de passar; ver como cada mudança foi declarada antes de rodar; e distinguir um resultado forte, conferido contra uma verdade externa, de um resultado que passa pela regra mas é frágil.

### 12.1 O portão

O H2 testou se o método transfere entre áreas. Cada domínio entra por um **pacote padronizado** do F7: pergunta substantiva, alegação, glossário de 10 termos, fontes, unidade independente, ambientes e cofre, grafo causal, conjunto de ajuste, elicitação, decisão e escopo. O validador recusa o pacote incompleto antes de ler qualquer dado. Dez dos doze primeiros pacotes foram escritos por agentes que só leram o contrato e dois exemplos: foi o teste de plugabilidade.

O portão escolhido pelo plano era exigente: o ICP (Capítulo 4) deveria encontrar um conjunto invariante não vazio, Ŝ ≠ ∅, em pelo menos três áreas distintas. sil decidiu manter esse portão forte quando ele falhou, em vez de relaxá-lo, e buscar dados com intervenções conhecidas.

### 12.2 Sete rodadas

Cada rodada foi declarada num arquivo com hash antes de rodar. As correções feitas depois de ver resultados, como o arcabouço v2 do H2, o critério do G2 e a unidade do Sachs, ficaram registradas e contam no K.

| Rodada | O que mudou | Áreas com Ŝ ≠ ∅ | Portão |
| --- | --- | --- | --- |
| H2 | 12 domínios de conveniência (ano, região, aeroporto como ambientes) | 1: Direito | Falhou |
| H2b | Ambientes que são intervenções documentadas (K = 1) | 1: Direito | Falhou |
| H2c | Teste de invariância não paramétrico (K = 2) | 2: Saúde e Educação; a Saúde por saturação | Falhou |
| H2d | Regra que descarta conjuntos sem poder de teste (K = 3) | 1: Educação | Falhou |
| H2e | Câmaras físicas com o grafo causal conhecido | 2: Educação e Física | Falhou |
| H2f | Deleções de genes em levedura (Kemmeren et al., 2014) | 3: Educação, Física e Genética | Passou pela regra; aprovado com ressalva |
| H2g | Ensaio clínico de cardiologia do FDA | 4: as três anteriores e Farmacologia | Passou; 3 áreas mesmo sem a Genética |

**Por que a primeira rodada falhou.** Nos domínios de conveniência, o ICP ou rejeitou todos os conjuntos, porque o ambiente (ano, região, aeroporto) age direto no resultado, ou aceitou tantos que a interseção ficou vazia. Além disso, o ICP linear gaussiano, com amostras grandes, rejeita qualquer desvio do modelo.

**Uma falha achada lendo o código.** No H2b, antes de qualquer execução, apareceu um erro no teste de invariância: os graus de liberdade contavam colunas em vez do posto da matriz. Com ambientes que são intervenções, um candidato costuma ser constante dentro do ambiente, e o teste aceitava demais. A correção ganhou um teste próprio, e a reexecução mostrou que o erro não tinha mudado nenhum resultado do H2.

**A regra de saturação.** Se o ambiente é função determinística de X\_S, a invariância vale por construção e o teste tem poder zero. Um teste sem poder não certifica nada. Foi por isso que o Ŝ da Saúde (o experimento de seguro da RAND, em que a coparticipação sorteada define o ambiente) saiu no H2d, como a declaração previa.

### 12.3 O momento mais limpo: o túnel de luz

Numa câmara física construída para estudar causalidade (Gamella, Peters e Bühlmann, 2025), o ICP recuperou exatamente os pais verdadeiros da corrente elétrica, os três LEDs, Ŝ = {red, green, blue}. Deixou de fora sensores de luz fortemente correlacionados com a corrente, porque são filhos dos LEDs. Foi a primeira vez no projeto que o método foi conferido contra uma verdade conhecida, e acertou.

O túnel de vento, na mesma bateria, rejeitou até os pais verdadeiros, em parte por deriva atmosférica, declarada antes; a autocorrelação e um ajuste aditivo onde a física é quadrática são causas plausíveis, não testadas. Um método que falha num caso e acerta no outro, com causas plausíveis declaradas, ensina mais que um método que sempre acerta.

### 12.4 A ressalva da Genética

A sexta rodada chegou às três áreas com Educação (turmas sorteadas do STAR), Física (túnel de luz) e Genética (deleções de genes em levedura). Mas a Genética era frágil por três razões.

1. Vinha de um único gene entre dez sorteados.
2. O ICP rejeitou 1 022 dos 1 024 conjuntos, e o melhor tinha p = 0,007, logo acima do limiar de 0,005: é a situação em que o próprio ICP avisa que o modelo está quase rejeitado.
3. A validação por intervenção contradiz dois dos três genes do conjunto: deletar o gene aumenta Y, ao contrário do que o coeficiente invariante prevê.

sil aprovou a saída do H2 com essa ressalva registrada.

A ressalva foi posta à prova depois. A réplica foi declarada antes de rodar, em `fundo/p07-icp/DECLARACAO.md`: 40
genes Y novos sorteados com `default_rng(7007)` num pool de 4 681 — os nunca deletados, fora os 10 do H2f —, cada
um com a mesma pré-seleção e o mesmo teste original do H2g, e α = 0,05/40 = 0,00125 por gene. Onze dos 40 deram
Ŝ ≠ ∅; em dez deles o Ŝ tinha pelo menos um gene, 27 genes em todos, e a deleção funcionou em 26 dos 27. Um único
par passa na validação estrita por intervenção: n34, Y = YLR354C, x6 = YIL033C — no mutante ΔYIL033C a expressão
do alvo é o menor valor das 1 741 amostras (quantil 0,000, fora da faixa de todas as outras), na direção que o
coeficiente positivo (+0,053) prevê. Só coerência fraca, que não conta, apareceu em n03, x4 = YGL131C. O controle,
o domínio y08 original do H2f, reproduz o publicado: Ŝ = {plate, x1 = YKL009W, x5 = YNL023C, x9 = YPL193W}, 2
conjuntos aceitos e melhor p = 0,00724 contra os 0,0072 publicados, e os mesmos três genes de novo não passam na
validação estrita.

O limite fica dito sem enfeite: a validação estrita decide sobre uma linha, porque cada mutante é um ambiente de
n = 1, e a declaração original não fixou taxa nula para ela; 1 gene em 27 era o número, sem p-valor de acompanhamento.
A declaração seguinte fixou a nula — permutar o rótulo do mutante entre as 1 741 linhas, por par de gene e candidato,
B = 999, semente 20261015 — e o resultado foi p = 0,001, o piso de 999 sorteios: nenhum deles chegou a um par.

A leitura tem duas metades, e elas não dizem a mesma coisa. Contra o acaso do rótulo a evidência é forte: o observado
é 1 par em 27 contra uma média nula de 0, e o p está no piso do teste. Contra ler isso como efeito medido, a evidência
é nula: quase todo o p vem da metade que é fato dos dados — deletar um gene tira a expressão dele, então o mínimo da
coluna está na linha do mutante —, e permutar o rótulo destrói exatamente essa metade. A metade que sobra, o alvo fora
da faixa das outras 1 740 linhas, sozinha dá p = 0,031, marginal. O veredito não muda: a Genética se replica, e o que ela
replica é um gene num mutante, não um efeito medido com resolução.

### 12.5 A sétima rodada: os bloqueadores de hERG

Num ensaio do FDA em que voluntários receberam quatro drogas, o ICP apontou como causas invariantes do alongamento do intervalo QT exatamente as concentrações das três drogas que bloqueiam o canal hERG (dofetilida, quinidina e ranolazina). Todos os 35 conjuntos aceitos, de 512, continham as três, e medidas do próprio eletrocardiograma ficaram de fora. A inclinação do QTcF na dofetilida foi de 25,2 ms por ng/mL \[23,0; 27,5\]. Como todos os intervalos do H2, este é de 90% da posterior.

**O que a invariância protege.** No verapamil, a droga que o modelo nunca viu, o modelo invariante previu bem (habilidade +0,57, cobertura conforme 0,89). O modelo ajustado pela chave fez ainda melhor (+0,65); quem explodiu foi o modelo com todas as variáveis, porque o verapamil empurra o intervalo PR para fora de tudo o que havia no treino. O PR prevê bem dentro do treino e falha sob uma intervenção nova.

Sem a Genética restam três áreas, Educação, Física e Farmacologia, duas delas conferidas contra uma verdade externa. Ficam duas ressalvas. O teste supõe medidas independentes, e o ensaio tem 16 medidas por voluntário. E a Educação, a única das três sem verdade externa, também é frágil: o maior p dos conjuntos não saturados é 0,075, perto do limiar, e {turma\_pequena} sozinho é rejeitado (p = 0,010).

As duas ressalvas foram medidas depois, no mesmo ponto da auditoria de fundo, e as unidades de fato não são
independentes: 16 medidas por voluntário no ensaio do FDA e 80 escolas no STAR. Refazendo o ICP com um Wald de
covariância agrupada (CR1) — por voluntário na Farmacologia, por escola na Educação — e com permutação de blocos
inteiros (B = 999), a Farmacologia mantém Ŝ = {c\_dof, c\_qui, c\_ran} nos dois testes e continua sólida, embora a
evidência contra o conjunto vazio caia de 6,3e-104 para 2,2e-15 e 0,002, isto é, de cerca de 340 bits para 49 e 9.
A Educação não mantém: com as 80 escolas como unidades de fato independentes, os conjuntos aceitos — 266 de 512 no
teste agrupado e 512 na permutação — não têm nenhuma variável em comum, e Ŝ = ∅; o conjunto publicado
{anos\_pequena\_antes, turma\_pequena} tem p = 0,76. Ŝ = ∅ aqui não quer dizer que turma pequena não tenha efeito:
quer dizer que o teste perde o poder de apontar um conjunto, que é o único produto que ele entrega.

A contagem de áreas fecha assim: a Educação sai, a Física fica como estava, Ŝ = {blue, green, red}, a Farmacologia
fica e a Genética se replica. O portão fica de pé com exatamente três áreas, e a terceira área passa a ser
sustentada por um domínio novo, e não pela que já era frágil.

### 12.6 As lições positivas da falha

- **O efeito transporta melhor que o conjunto.** Mesmo com o ICP rejeitado, o modelo ajustado previu o ambiente nunca visto com habilidade positiva em 9 domínios, de 0,04 a 0,88.
- **A pseudorreplicação estava em toda parte.** Nos voos de Nova York, 225 mil voos valem o mesmo que 364 dias; no STAR do H2, os 5 250 alunos do recorte de escolas urbanas se agrupam em 72 escolas (o recorte maior do H2g, usado neste capítulo, tem 11 598 alunos em 80 escolas).
- **Abster-se é resultado.** Um Ŝ vazio honesto vale mais que um Ŝ fabricado por saturação.

A arquitetura também errou, e foi a auditoria dos próprios agentes que o viu. No H2, a revisão dos agentes de domínio achou quatro falhas no arcabouço, corrigidas de uma vez para todos os domínios na versão 2. O nulo do F3 permutava num nível diferente daquele em que a chave varia; agora permuta entre unidades, dentro da unidade quando o sorteio é declarado, ou médias e desvios quando a chave é observacional. A posterior tratava as linhas como independentes e saía estreita demais; agora a verossimilhança é temperada por 1/efeito de desenho. A predição conforme era calibrada num quinto sorteado do treino, que não carregava a mudança entre ambientes; agora deixa um ambiente de fora. E os rótulos do ICP não separavam "rejeitado" de "vazio aceito". A versão 2 mudou o status de 4 domínios: com a primeira, energia e ciclones caíam na cobertura conforme, o crédito no R-hat (1,011) e o Preston no F3 (h2/README.md). As quatro correções vieram depois de ver os resultados e contam no K do programa. O controle de qualidade de uma arquitetura de agentes precisa olhar para o arcabouço, não só para os domínios.

Fica também uma multiplicidade que o K não contava, e que a auditoria contou. Depois do H2d, a própria declaração registrou que rodar um domínio reserva numa área nova "seria procurar área, não testar invariância" (h2d/README.md). As rodadas seguintes trouxeram dados novos com intervenções conhecidas, e não reservas, mas acrescentaram áreas até o portão passar, e o contador registra especificações por domínio, não domínios tentados.

O K honesto do portão do H2 é de 38 testes ICP distintos — 32 a 72, conforme o que se conta, e as sete rodadas
acima estão dentro dessa conta. Com Bonferroni sobre 38 (α = 0,0013 por teste), o H2g ainda tem Ŝ ≠ ∅ em quatro
áreas, ao custo de 9,6 bits de Occam, e os conjuntos invariantes encolhem: no STAR, {anos\_pequena\_antes,
turma\_pequena} vira {turma\_pequena}; na levedura, {plate, x1, x5, x9} vira {plate, x5}, e x1, o único gene que a
validação por intervenção confirmava, sai.

Duas ressalvas pesam mais que a correção. A primeira é que dividir α não é monótono no ICP: com α menor, domínios
que rejeitavam tudo passam a aceitar algum conjunto, e aparecem Ŝ em modelos quase rejeitados. A segunda é que a
aprovação depende do arcabouço v2: com o v1 reconstruído, o STAR é podado antes do teste de invariância — a chave
é constante dentro do ambiente —, e o H2g cai para três áreas, uma delas a Genética frágil. O inventário do
caminho conta ainda onze aberturas de cofre, isto é, onze vezes em que um resultado foi visto antes de uma decisão
de desenho. E se valesse só a primeira versão declarada de cada escolha, o H2 não estaria aprovado: a rodada 1 dá
uma área só (o lado do H3 dessa conta fica no Capítulo 13).

### Exercícios

1. *(Conceito)* Por que um ambiente que é o ano ou a região costuma violar a premissa do ICP?
2. *(Conceito)* Explique a regra de saturação com o exemplo da RAND. Por que um teste sem poder não certifica invariância?
3. *(Conceito)* Liste as três razões da fragilidade da Genética e diga qual delas pesa mais para você.
4. *(Conceito)* O verapamil é um caso de mudança de regime. Relacione com a crítica de Lucas do Capítulo 5.
5. *(Laboratório)* Reproduza o H2e a partir do repositório e confira que o túnel de luz aceita 128 de 1 024 conjuntos e devolve {red, green, blue}.

## Capítulo 13. H3: vazamento, Goodhart e o cofre

**Objetivos.** Entender como se prova zero vazamento temporal; ver um laço de Goodhart medido e desfeito; e acompanhar três tentativas de calibração fora do tempo, cada uma num cofre novo.

### 13.1 A malha contínua

O H3 rodou seis fluxos reais como se fossem ao vivo, sob um relógio que só libera cada dado no instante em que ele se tornaria conhecível. Cada fato é um evento com um carimbo `t_disp`. Os agentes só leem a janela entre o último tique e agora, e o armazém recusa ler o futuro, ler o cofre antes do pré-registro, abrir o cofre duas vezes e voltar o relógio.

Dois mecanismos operam o modelo. O sensor de deriva (F3) usa os testes de Kolmogorov-Smirnov e Wasserstein com histerese, com limiares tirados da variação do próprio fluxo. O disjuntor (F8), que implementa a cláusula objetiva de saída do Φ4, troca a previsão oficial por um reserva determinístico quando o modelo perde para ele numa janela móvel; o modelo continua rodando em sombra, e é a vitória dele em sombra que fecha o disjuntor. Abster-se não tira a previsão do placar: quando o modelo é recusado, pontua-se o reserva.

| Fluxo | O que prevê | Calibração no cofre | Portão |
| --- | --- | --- | --- |
| Energia (Victoria) | Demanda de cada meia hora de amanhã | Cobertura de 90% = 0,887 (reserva oficial em 48% do tempo) | Passa |
| Central (banco, 2003) | Chamadas por 5 minutos do próximo dia útil | Cobertura 0,900 | Passa |
| Bolsa (Dow Jones) | Valor em risco de 99% do próximo pregão | Violações 0,98% (aglomeradas: Christoffersen p = 0,02) | Passa |
| Voos (Nova York, 2013) | Chance de atraso acima de 15 minutos | Erro na média de 2,7 pontos | Falha |
| Ciclones | Chance de ganhar 30 nós em 24 horas | ECE 0,012 | Passa |
| Prisões (Toronto) | Chance de liberação | ECE 0,036 | Falha |

### 13.2 Zero vazamento, com prova

Em 453 973 previsões, nenhuma leitura devolveu evento além do relógio, nenhuma previsão usou estado do futuro e nenhum rótulo já era conhecido na emissão. Mas "nenhuma violação" só vale se o teste era capaz de achar uma. Por isso o sistema plantou um **canário**: uma coluna idêntica ao rótulo, publicada no instante do rótulo. Nos três fluxos binários, com o relógio honesto, ela não deu ganho nenhum; com o relógio desligado, deu ganhos de +0,86 a +0,92. É o teste severo de Mayo aplicado ao próprio teste.

### 13.3 Goodhart medido

Na central, quem não é atendido liga de novo, e a previsão passa a medir o próprio efeito. Um previsor ingênuo acerta o que observa com 1% de erro e, ainda assim, superestima a demanda verdadeira em 29%: a métrica aprova um modelo errado sobre o mundo. Usando choques sorteados na escala como instrumento, o erro cai para 1,7%. O estimador teve quatro versões registradas antes de passar, ajustadas depois de ver os resultados do laço simulado, que são o ponto mais fraco desse portão, e o laço foi declarado sobre as chegadas reais do H3, sem ter sido medido na operação daquela central — o parâmetro foi medido em outra central, de 1999 (seção 5.4). É a proibição 3 do Capítulo 8 em ação: só o experimento embutido separa aprender de deformar.

A auditoria completou essa conta. Das quatro versões do estimador, só a quarta passa no critério declarado — na reconstrução da auditoria, vieses de +4,7%, −5,9%, −8,4% e +1,7% (o histórico do H3 registra +6,6% na v1 e −4,8% na v2), para um limite de 3% — e a primeira falha por desenho, não por conta: escalar pela demanda corrigida alimenta a espiral de desistências. E o laço que a correção enfrenta é bem mais forte que o de um caso real: 0,6 declarado contra 0,145 medido numa central de 1999 (Capítulo 5). A correção, portanto, foi testada num mundo mais hostil que o mundo observado, e essa assimetria é o que a torna frágil como prova.

### 13.4 Calibração: três tentativas

**Primeira tentativa: duas falhas.** Nos voos, os atrasos do último trimestre de 2013 ficaram 2,7 pontos acima do previsto, para uma tolerância de 2. Nas prisões de Toronto, o erro de calibração (ECE) foi 0,036, para uma tolerância de 0,03, com só 1 488 casos. A lição de desenho é precisa: **o disjuntor comparava o modelo com o reserva, não com a calibração**, e por isso não viu a deriva da média. O modelo continuava vencendo o reserva, só que descalibrado.

**Segunda tentativa: os voos passam, as prisões não.** A correção, uma recalibração contínua do intercepto, foi declarada antes e testada num cofre novo: o mesmo trimestre em 2023. Os voos passaram (ECE 0,008), mas o trimestre de 2023 não repetiu o desvio de 2013, e o modelo cru, sem correção, também teria passado (ECE 0,014): a correção melhorou a calibração, mas não foi posta à prova. As prisões foram testadas com dados de Toronto de 2020 e 2021, já com outra pergunta, o registro na delegacia em vez da liberação: o modelo acerta a média, mas é pouco confiante nos extremos (ECE 0,067). Nas faixas de baixo prevê 0,12 onde a taxa observada é 0,04. É um erro de inclinação, que a correção do intercepto não alcança. Corrigi-lo seria especificação nova, e não havia dados de Toronto posteriores para um cofre cego.

**Terceira tentativa: Rhode Island.** A nova especificação, uma recalibração contínua de Platt que ajusta intercepto e inclinação, foi declarada e testada num cofre novo: 24 466 paradas de trânsito da polícia estadual de Rhode Island, de 2013 a 2015, com taxa de prisão de 2,26%.

| Cofre 2013–2015 | Erro na média | ECE | Habilidade contra a taxa-base |
| --- | --- | --- | --- |
| Previsão oficial (com Platt) | 0,0020 | 0,0048 | +0,019 |
| Modelo cru, em sombra | 0,013 | 0,013 | +0,004 |

Passou, com ECE de 0,21·ȳ contra o limite de 0,3·ȳ. O cofre tinha esse critério relativo à taxa, e o modelo cru o teria falhado: é isso que mostra o valor de Platt. Antes de declarar o cofre, só as contagens gerais da base tinham sido vistas, e uma correção de leitura em disco feita depois de abri-lo, com diário idêntico, conta como K = 2. Ficam duas ressalvas. A pergunta mudou, de liberação depois da prisão para prisão numa parada, e sil confirmou a substituição. E a habilidade é pequena, porque o evento é raro e pouco previsível pelas variáveis disponíveis. **A calibração é boa; a resolução, não.** É a decomposição de Murphy do Capítulo 3 em dados reais: a recalibração consertou a calibração, mas não podia criar a resolução que os dados não tinham.

Duas lacunas do desenho ficaram abertas, e a auditoria de fundo as fechou com respostas que não são as que o desenho esperava. A tolerância do ECE pelo tamanho da amostra foi construída (Capítulo 3): sob ela, nenhum dos seis cofres binários passa, nem os três aprovados, e a reprovação de Toronto de 1997–2002 deixa de parecer ruído — o ECE de 0,036 contra o quantil nulo de 0,032 dá valor-p de 0,008. Pela regra secundária, que combina o piso de 0,03 com o quantil nulo, o quadro volta ao que era: três aprovados, dois reprovados, e os cofres grandes com os seus 0,0079 e 0,0048 continuam passando. A segunda lacuna, o teste de deriva de rótulo, foi rodada nos dois desenhos do Capítulo 7 e não teria avisado antes do sensor original (seção 7.2). A conclusão de desenho é que a correção do intercepto continua sem prova no cofre: ela aguentou o único desvio real que apareceu em dois anos novos de voos (o terceiro trimestre de 2017, de +0,028 para +0,001), mas o estresse declarado não discriminou, porque o reajuste diário absorve a deriva sozinho no nível do trimestre.

### 13.5 Quem paga pelo erro, de novo

A auditoria de justiça (F6) foi informativa nos três fluxos de prisões, e o padrão não foi o mesmo nos três.

- **Toronto, 1997–2002.** A taxa de falso positivo dos negros foi cerca do dobro da dos brancos em todos os anos (0,16 a 0,23 contra 0,06 a 0,11), com um modelo que não usa a cor.
- **Toronto, 2020–2021.** As taxas-base ficaram próximas, e calibração, falsos positivos e falsos negativos ficaram a menos de 0,05 entre os grupos.
- **Rhode Island, 2013–2015.** A taxa de prisão dos motoristas negros foi o dobro da dos brancos, e a taxa de falso positivo foi quase o dobro em 2013 e 2014 (0,019 contra 0,011; 0,025 contra 0,013), de novo num modelo cego à cor.

Não usar a cor no modelo não dá paridade, porque a cor entra pelos correlatos. A identidade de Chouldechova diz que, com taxas-base diferentes, calibração e taxas de erro iguais são incompatíveis; com taxas-base próximas, a paridade fica possível, mas não automática. Em Toronto, em 2002, as taxas de não liberação eram próximas (21% contra 19%), e a taxa de falso positivo dos negros continuou cerca do dobro.

A auditoria de fundo repetiu a pergunta do Φ4 no avesso, nos três fluxos de prisões (`h3/saida/*/justica_invertida.json`).
A métrica escolhida é a taxa de falso positivo do flag, P(marcado | não sofre o desfecho de interesse); a invertida é a
taxa de falso negativo, P(não marcado | sofre o desfecho). O grupo menos favorecido sai da maior taxa-base, o pagador é o
de maior taxa sob a métrica em vigor, e empate é o que tem intervalos de Wilson de 95% sobrepostos. A varredura de
limiares vai de 0,05 a 0,95, de 0,05 em 0,05, mais o limiar declarado de cada fluxo: 0,70 em Toronto 1997–2002 e em
Toronto 2020–2021, e 0,90 em Rhode Island.

Em Toronto 1997–2002 o grupo menos favorecido é o que tem a MENOR taxa de falso negativo. No limiar declarado de 0,70, o
`Black` perde 0,64 dos casos em 1998 contra 0,70 do `White`, enquanto pela métrica escolhida quem paga é o `Black`
(0,196 de falso positivo contra 0,105) e pela invertida o `Black` sai e fica o empate ou o `White`. Quem é marcado com
mais frequência sem sofrer o desfecho é o `Black`; quem é perdido com mais frequência é o `White`. Trocar a métrica
troca quem paga, e nesse fluxo isso acontece em um a dois limiares do grid, todos acima do limiar declarado: 0,85 em
1998, 0,50 em 2000 e 0,80 em 2001. Em Toronto 2020–2021 e em Rhode Island os intervalos se sobrepõem em quase toda a
varredura e a inversão não troca pagador nenhum: os dois fluxos ficam em empate nos quatro semestres e nos três anos do
cofre, com a taxa de falso negativo do grupo menos favorecido em Rhode Island entre 0,85 e 0,93 no limiar de 0,90.

O registro dos grupos afetados é curto e não é bom. Em sete decisões que envolvem pessoas — os quatro pareceres que este
capítulo já citava e os três fluxos de prisões —, **nenhum grupo afetado foi ouvido** (`governanca/GRUPOS.md`). Não há
no repositório consulta, entrevista, painel nem consentimento; o que existe são declarações sobre os dados, como os
titulares e o atributo protegido, não sobre quem foi escutado. Em Rhode Island a pergunta ainda mudou de "liberação
depois da prisão" para "prisão numa parada de trânsito", troca confirmada por quem conduz o programa e não pelo grupo que
passou a ser o alvo da decisão. O outro lado do mesmo registro é que assinar não é escutar: o responsável fica do lado de
quem decide usar o artefato, não de quem paga o erro.

### Resumo

- Zero vazamento só vale com um canário que prove que o teste acharia o vazamento.
- Um previsor pode acertar o que observa e errar o mundo; o instrumento sorteado desfaz o laço.
- Cada correção de calibração só foi aceita num cofre novo.
- Calibração se conserta; resolução, não.

### Exercícios

1. *(Conceito)* Por que um canário que não dá ganho com o relógio honesto prova pouco se ninguém o testa com o relógio desligado?
2. *(Conceito)* Explique a falha do disjuntor nos voos de 2013 e proponha um segundo critério de abertura baseado em calibração.
3. *(Conceito)* Por que a recalibração de intercepto não corrige um modelo pouco confiante nos extremos? Desenhe a curva de calibração dos dois casos.
4. *(Conta)* Com taxa de 2,26%, quanto vale, aproximadamente, o Brier de um climatologista que diz sempre essa taxa? Compare com uma habilidade de +0,019, lembrando que a do H3d foi medida contra uma climatologia móvel, não contra a taxa fixa do cofre.
5. *(Conceito)* Por que mudar a pergunta de Toronto para Rhode Island exigiu a confirmação de um humano?

# Parte IV. Balanço

## Capítulo 14. Lições, limites e horizontes

**Objetivos.** Reunir o que a pesquisa ensinou; responder às objeções mais fortes contra a disciplina; e saber quais são os próximos passos abertos.

### 14.1 Sete lições

1. **A média esconde o que importa.** A mesma demanda média dá 9 ou 21 minutos de espera; a escala padrão de uma central falha em 26 de cada 100 dias; a riqueza média de 19 vezes convive com uma mediana de 2,1.
2. **A unidade independente decide a margem.** 2,65 milhões de chamadas valem 81 dias; 225 mil voos valem 364 dias. O modelo nulo de metades arbitra a questão sem convenção.
3. **Acertar não basta; é preciso ter podido errar.** O ajuste por propensão em LaLonde acertou o ponto, e um tratamento inócuo teria dado quase o mesmo.
4. **Reprovar o próprio modelo é resultado.** Nos incêndios dinamarqueses, a alegação perdeu e o modelo não ganhou o direito de decidir.
5. **O efeito transporta melhor que o conjunto.** A invariância exata é rara; quando aparece e é conferida (o túnel de luz, os bloqueadores de hERG), protege contra a intervenção nova.
6. **Calibração se conserta, resolução não.** Três cofres foram necessários para a calibração passar pela tolerância fixa de 0,03, que a auditoria mostrou frouxa nos cofres grandes (seções 9.7 e 13.4); a habilidade final ficou em +0,019.
7. **Declarar antes de rodar é o que torna a falha útil.** Sete rodadas no H2 e três tentativas no H3 só contam como ciência porque cada mudança foi declarada com hash antes de rodar e cada ajuste feito depois de ver um resultado ficou registrado e contado no K.

### 14.2 Objeções e respostas

Uma disciplina que exige teste severo das alegações alheias precisa aplicá-lo a si mesma.

**"Isso é só estatística com outro nome."** Em parte, sim: quase todas as ferramentas têm dono há décadas. O que a Estocologia acrescenta é a organização: as camadas, a tipagem da incerteza e os mesmos portões para toda alegação. A hipótese que o programa ainda precisa demonstrar é que essa organização pega erros que a prática comum deixa passar. O primeiro teste direto dela, o piloto cego da seção 1.4, empatou no teto: os dois braços apontaram os mesmos oito defeitos plantados e deram os mesmos zero alarmes falsos.

**"Os dados são antigos e públicos."** Foi escolha: bases conhecidas, verificáveis por hash e com desfecho sabido permitem auditar o método, mas não substituem uma operação de verdade. Várias peças continuam supostas, e estão declaradas como tal (seção 9.7): o tempo de atendimento e a paciência dos clientes da central, o painel de especialistas que nunca foi consultado e o que a auditoria não pôde olhar. Outras deixaram de ser suposição: o laço performativo, a tolerância da calibração e a fragilidade da Genética foram medidos, e a seção 9.7 traz os números.

**"Formalizar o julgamento é eliminar o julgamento."** A arquitetura não decide o que vale a pena perguntar nem quanto um erro custa para quem o sofre. Esses valores entram pela elicitação e pela razão C/L, declarados, não calculados. A ambição é tornar o julgamento visível, não substituí-lo; por isso a definição diz "ciência (ou arte)".

**"O aparato é pesado demais."** Oito agentes, cinco lentes e quatro invariantes são muito para uma pergunta simples. O funil responde: a maior parte das hipóteses cai barato. Se o custo da máquina superar o valor das decisões que ela melhora, a própria Crisonomia manda desligá-la.

**"Bayes resolve o problema de Hume."** Não resolve. A atualização bayesiana é a regra coerente dentro de um espaço de hipóteses, mas a indução se desloca para a escolha da priori e do espaço, como a regra de Laplace mostrou. O próprio teorema de Cox depende de hipóteses técnicas (Halpern, 1999).

**"A ergodicidade virou doutrina."** A Crononomia se apoia no programa de Ole Peters, que é contestado, e a disciplina o ensina como debate. Pelo critério do crescimento, a aversão a perdas de cerca de 2,25 estimada por Tversky e Kahneman só se justifica quando 56% da riqueza estão em jogo: a tese explica a recusa de arriscar meio patrimônio, não a aversão medida em laboratório com apostas pequenas. Algo parecido vale para as caudas: Taleb pede que se suponha cauda pesada até prova em contrário; Clauset, Shalizi e Newman pedem prova antes de declarar uma lei de potência. A questão é de quem é o ônus da prova, e a resposta depende de quanto custa errar.

**"O auditor é parte interessada."** Os 21 pareceres foram assinados por quem conduz o programa. Nomear um responsável não torna a auditoria independente. A auditoria de fundo acrescentou uma checagem mais barata e mais fraca: reimplementar do zero quatro resultados que sustentam as saídas aprovadas, a partir das especificações. Os quatro concordaram com o publicado e nenhum erro foi achado no original — mas o teste tem um defeito declarado, porque várias escolhas que a especificação não fixa saíram iguais às do código original, três delas só no STAR, que é o modo de falha de uma reimplementação feita pela mesma equipe. Independência real pede gente de fora, com o mesmo contrato. O que o programa ainda precisa registrar é o que a auditoria não pôde olhar e quais grupos afetados foram ouvidos.

### 14.3 Horizontes

**Do argumento ao artefato.** A Estocologia propõe que a unidade de conhecimento seja um artefato executável: uma alegação com seu processo gerador, seu modelo nulo e seu teste. Discordar deixa de ser confrontar duas narrativas e passa a ser localizar a premissa que muda o resultado.

**Agentes artificiais com disciplina epistêmica.** Modelos de linguagem escrevem com fluência e erram com confiança; são agentes da camada L3 com atalhos próprios. A arquitetura não tenta torná-los infalíveis: cerca-os de portões. Eles não fazem contas de cabeça, não conversam em linguagem livre e não aprovam o próprio trabalho. A pergunta deixa de ser "a IA está certa?" e passa a ser "o que ela afirmou sobreviveu a quais testes?". Avaliar um sistema desses exige rubricas com o desacordo entre avaliadores descontado do acaso, κ = (p\_o − p\_e)/(1 − p\_e), e registro de versão e data, porque o fornecedor pode trocar o modelo em silêncio.

**O tempo e a ruína.** Para quem decide uma única vez, com um único patrimônio, evitar a ruína vem antes de maximizar o esperado.

**A previsão que muda o que prevê.** Todo modelo publicado roda com e sem a ação, com métrica companheira, auditoria fora do laço e prazo de validade para a própria meta.

**A comunicação como parte da ciência.** Frequências naturais, gráficos de resultados hipotéticos e cláusula de saída são a saída do sistema, não um apêndice.

### 14.4 O que a auditoria respondeu e o que fica em aberto

A auditoria de fundo testou dez suposições do próprio programa, e a lista completa com os números está na seção 9.7.
Dez das lacunas que a primeira edição deste capítulo listava deixaram de ser lacuna.

- **Testes de invariância que respeitem medidas repetidas.** O teste agrupado por voluntário e por escola foi rodado,
  e ele muda veredito entre áreas: a Farmacologia sobrevive, a Educação não (Capítulo 12).
- **Uma tolerância de calibração que dependa do tamanho do cofre.** Foi construída, e nenhum cofre binário do H3
  passa por ela; pela regra secundária, o quadro daquele horizonte não muda (Capítulos 3 e 13).
- **Um teste de deriva de rótulo ao lado do sensor de entradas.** Os dois desenhos foram rodados e nenhum avisa antes
  do sensor original; o que compara a taxa realizada com a média prevista fica ligado quase o tempo todo (Capítulo 7).
- **Uma auditoria periódica do próprio arcabouço.** É literalmente o que a pasta `fundo/` é, e ela achou coisas que a
  auditoria dos domínios não tinha achado: o laço declarado quatro vezes maior que o medido, a tolerância de
  calibração frouxa nos cofres grandes, o portão do H2 dependendo de qual versão de cada escolha conta.
- **A tolerância da rodada de controle do ponto 5.** A condição de falha estava escrita sem tolerância e disparava por
  diferença numérica de ponto flutuante. A banda de equivalência de 0,05 dp foi declarada depois, com hash próprio e
  K = 2, e o veredito deixou de ser condicional: o desvio máximo é 0,0071 dp e os 33 domínios batem (seção 9.7).
- **Previsores de eventos raros.** Os três cofres foram medidos e nenhum caminho vence; só em Rhode Island a resolução
  bate a confiabilidade, e a habilidade de Brier fica em +0,019, longe do +0,05 exigido; no COVID a ordenação é boa
  (AUC 0,77) e a previsão perde para a constante por deslocamento de nível (seção 9.7).
- **A auditoria de quem paga pelo erro.** A pergunta do Φ4 foi rodada ao avesso nos três fluxos de prisões: em Toronto
  1997–2002 trocar a métrica troca quem paga, e nos outros dois fluxos o empate não muda (Capítulo 13).
- **Um campo no contrato de dados do F2 para o mecanismo da ausência** (MCAR, MAR ou MNAR), com análise de
  sensibilidade quando a ausência pode depender do próprio valor. O bloco entrou no contrato, com a regra validada no
  código; nos dados de hoje ele é vacuoso, e é isso que o mantém barato (Capítulo 8).
- **Uma pergunta explícita na ficha contra o erro de tipo III**: que a pergunta que o modelo responde seja a que o
  decisor fez virou campo obrigatório do pacote de domínio, com quem decide e qual ação muda, validado contra o
  próprio texto do pacote (Capítulo 8).
- **Dados reais de saúde**, para fechar o círculo que o Lab 0 abriu. O primeiro domínio real de saúde entrou, no espelho
  aberto da PhysioNet, e o que ele deu foi uma alegação que não se sustenta por falta de invariância (seção 9.7).

Ficam em aberto, agora com nome:

- **Elicitação com especialistas humanos de verdade.** A auditoria mediu a sensibilidade às prioris; isso não
  substitui um painel, e cinco de 33 domínios dependem da escolha. O protocolo do painel e os seis formulários estão
  escritos, com o critério de K = 2 pronto, e ninguém foi eliciado: nenhuma previsão foi corrigida, nada foi assinado.
- **A resolução da validação por intervenção da Genética.** A taxa nula saiu em p = 0,001, o piso de 999 sorteios, e
  com um mutante por ambiente o menor p possível ali é 1/1 000: o efeito que sustenta a réplica é o de uma linha.
- **O que a auditoria não pôde olhar.** Os 21 pareceres assinados por quem conduz o programa precisam ser relidos um a
  um; as 21 linhas estão prontas, com os 21 SHA-256 conferidos, e nenhuma decisão foi assinada. O registro dos grupos
  afetados existe e diz que nenhum foi ouvido, nas sete decisões que envolvem pessoas: quem paga o erro continua sem
  voz no programa.


### Exercícios finais

1. *(Ensaio)* Escolha uma das sete objeções e escreva a melhor réplica que conseguir contra a resposta dada aqui.
2. *(Projeto)* Escolha uma alegação do seu campo e leve-a pelo ciclo completo: ficha com as cinco perguntas, processo gerador com semente, modelo nulo, as lentes pertinentes, um ataque de banca e um memorando em frequências naturais com cláusula de saída.
3. *(Conceito)* Responda à pergunta que fecha o curso: quando um modelo é bom o bastante?

# Apêndices

## Apêndice A. Glossário

| Termo | Definição | Capítulo |
| --- | --- | --- |
| Agente (L3) | Quem decide, com recursos limitados: pessoa, instituição ou algoritmo | 1 |
| Aresta de patologia | Marca no hipergrafo de um desalinhamento entre camadas, como a falácia da média | 6 |
| Auditoria de fundo | Frente que testa uma suposição do próprio programa, com declaração antes de rodar | 9, 14 |
| Banca adversarial | Etapa em que o orquestrador ataca o artefato ao vivo antes de aprovar | 7 |
| Barramento de fichas | Canal por onde os agentes trocam fichas YAML, encadeado por hash | 7, 9 |
| Canário | Vazamento plantado de propósito para provar que o teste de vazamento funciona | 13 |
| Cláusula de saída | Gatilho objetivo que diz quando a recomendação deixa de valer | 5, 8 |
| Cofre | Partição de dados guardada até o pré-registro e aberta uma única vez | 9 |
| Crisonomia | Lente do juízo: valor da informação, escores próprios, calibração | 6 |
| Crononomia | Lente do tempo: média de conjunto contra média temporal, ruína, Kelly | 6 |
| Disjuntor | Mecanismo que troca o modelo por um reserva seguro quando ele perde | 13 |
| ECE | Erro esperado de calibração: distância média entre o previsto e o observado por faixa | 13 |
| Escalonomia | Lente da escala: expoente de cauda, existência da média, tamanho efetivo | 6 |
| EVPI, EVSI | Valor esperado da informação perfeita e da informação de uma amostra | 5 |
| Ficha | Lista fechada de perguntas feita a um texto, número ou modelo | 8 |
| Fluxonomia | Lente da corrente: como probabilidade e informação fluem e se perdem | 6 |
| Frequências naturais | Expressão de um risco como "x em cada 100", com denominador fixo | 2, 5 |
| ICP | Predição Causal Invariante: devolve as causas cuja relação com Y não muda entre ambientes | 4, 12 |
| Incerteza aleatória | Variabilidade do Mundo, que mais dados não reduzem | 2 |
| Incerteza epistêmica | Ignorância do Modelo sobre parâmetros, que encolhe com dados | 2 |
| Incerteza knightiana | Processo real fora do espaço de hipóteses; mais dados dão falsa confiança | 2 |
| Kelly, critério de | Fração de exposição que maximiza o crescimento logarítmico | 5 |
| Modelo (L2) | Representação formal, simplificada e simulável do Mundo | 1 |
| Modelo nulo | Simulação do que o acaso sozinho produziria, para confrontar um padrão | 3, 10 |
| Morfonomia | Lente da forma: o que se conserva quando o ambiente muda | 6 |
| Mundo (L1) | A realidade que gera os dados e que nunca vemos inteira | 1 |
| Pseudorreplicação | Tratar observações dependentes como independentes, encolhendo a margem | 3, 10 |
| Regra própria | Escore em que declarar a própria crença minimiza a penalidade (Brier, logarítmico) | 3 |
| Saturação | Quando o ambiente é função de X\_S e o teste de invariância não tem poder | 12 |
| Severidade | Chance de o procedimento ter dado errado caso a afirmação fosse falsa (Mayo) | 3 |
| SHELF | Protocolo de elicitação de especialistas por quantis | 7 |
| Termo de escopo | Declaração de onde uma conclusão vale e onde não vale | 8 |
| Unidade independente | A menor unidade cujas observações não compartilham a mesma fonte de variação | 3 |

## Apêndice B. Como reproduzir a pesquisa

Todo o código está no repositório 0b3st4f3r4/estocologia, com uma pasta por horizonte e um README.md em cada uma. As bases reais têm origem e SHA-256 em `dados-reais/FONTES.md`; os arquivos grandes ficaram fora do Git e estão listados, com hash, em `FORA_DO_GIT.md`.

| Pasta | O que reproduz | Comando principal |
| --- | --- | --- |
| h0-lab0/ | Lab 0, posto de saúde simulado (Cap. 10) | `python3 portoes.py` |
| h0-real/ | Central de chamadas do banco (Cap. 10) | `python3 portoes.py` |
| h1-real/ | Dow Jones, LaLonde, incêndios dinamarqueses (Cap. 11) | `python3 rodar.py` e `python3 portoes.py --reproducao` |
| h2/ a h2g/ | As sete rodadas do ICP (Cap. 12) | `python3 rodar.py` e `python3 portoes.py --reproducao` em cada pasta |
| h3/ | Os seis fluxos e as revisões H3b a H3d (Cap. 13) | `python3 rodar.py` e `python3 portoes.py --reproducao` |
| governanca/ | Registro das assinaturas dos 21 pareceres (Cap. 9) | Leitura |
| fundo/ | A auditoria de fundo: os dez pontos supostos (p01 a p09) e as extensões (p10-saude, p11-eventos-raros, p12-shelf) (seção 9.7 e Cap. 14) | Leitura; `rodar.sh` em p02 e p05, script próprio nas demais |
| fundo/AMBIENTE.md | O que roda e o que não roda nesta máquina, com as versões conferidas uma a uma | Leitura |

Cada `portoes.py` roda o horizonte duas vezes e compara os hashes. Se o seu hash sair diferente do publicado, a reprodução falhou, e esse é o primeiro dado a investigar.

O `fundo/AMBIENTE.md` é a foto do ambiente das frentes, e ela tem duas ausências declaradas. Nesta máquina, Python 3.13.7, estão instaladas as nove dependências que o `fundo/requirements.txt` lista — numpy 2.4.6, pandas 3.0.5, scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0, duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1 e hypothesis 6.165.7. Falta `statsmodels`, que só o gabarito do p01 usa, e falta `pyreadr`, que só um passo do p08 usa (a conversão dos `.rda` de voos novos); sem eles, essas duas partes não reproduzem aqui, e o arquivo diz como instalá-los.

## Apêndice C. Respostas selecionadas

- **2.1.** Chance anterior = 1/99; razão de verossimilhança = 0,90/0,09 = 10; chance posterior = 10/99. Logo P(doente | positivo) = 10/109 ≈ 9,2%, o mesmo que 90/981.
- **2.2.** Independente, o segundo exame vale 10 · log₁₀ 10 = 10 decibans. Com a razão efetiva de 4,2, vale 10 · log₁₀ 4,2 ≈ 6,2 decibans. As chances passam de 10/99 a 42/99, e P(doente | dois positivos) = 42/141 ≈ 30%, o valor da população simulada.
- **4.3.** Diferença crua − efeito = −8 498 − 1 794 = −10 292 dólares. Como o sorteio do NSW zera o viés, o experimento estima o efeito nos tratados, e o resto da diferença crua é viés de seleção (h1-real/README.md).
- **5.1.** C/L = 1 800/34 000 ≈ 0,053, acima de p = 0,05: pela esperança, não se segura, porque paga 1 800 por uma perda esperada de 1 700. Pelo crescimento temporal, segura-se: o armador passa de 1,94% para 2,18% ao mês. A regra C/L é uma regra de esperança e não vê que a perda é um terço da riqueza.
- **5.3.** E\[ln(1 + fR)\] = p ln(1 + f) + (1 − p) ln(1 − f). Derivando e igualando a zero, p/(1 + f) = (1 − p)/(1 − f), logo f\* = 2p − 1. Com p = 0,6, f\* = 0,2.
- **10.1.** W(8) = 8/(12 × 4) h = 10 min; W(5) = 5/(12 × 7) h ≈ 3,6 min; W(11) = 11/(12 × 1) h = 55 min. A média de W(5) e W(11) é cerca de 29,3 min.
- **11.3.** Pela regra de Laplace, (0 + 1)/(36 + 2) = 1/38.
- **13.4.** O Brier de um climatologista com taxa fixa é p(1 − p) = 0,0226 × 0,9774 ≈ 0,022. Uma habilidade de +0,019 significa um Brier cerca de 1,9% menor que o da referência. A comparação com esse número é aproximada, porque no H3d a referência é a climatologia móvel, não a taxa fixa do cofre.

## Apêndice D. Fontes e leituras

**Documentos do projeto.** [Arquitetura](https://docs.google.com/document/d/1l0YQHrV_UrKisAjQcz2BqT9KpwhcT0s0WVoXm_muxtE/edit), [PLAN.md](https://docs.google.com/document/d/1hXvDuJyckooL1_qx9gspkSkBqgjyhAUVQvQ3F-Crinc/edit), [AGENTS.md](https://docs.google.com/document/d/1hniAsjLqgNVGqR1wZ1845GVkS5lZTEjhlH7MaI3U2Xk/edit) e [Estocologia: Fundamentação, Três Camadas e Inferência sob Incerteza](https://docs.google.com/document/d/1L5ittj7-RiSacEvNrkR4IS6yrQUyhusFpcDIWazINLI/edit). O artigo [Estocologia: Velhas Fronteiras, Novos Horizontes](https://claude.ai/code/artifact/69dbc377-114b-4cb7-a3c1-322387abc674), com a lista completa de referências. O curso *Pensamento Crítico e Modelos Estocásticos* (repositório 0b3st4f3r4/pensamento-critico-modelos-estocasticos), de onde vêm os números atribuídos a simulações do curso.

**Bases e métodos usados na pesquisa** (conferidos em fonte autoritativa, ver `referencias/CONFERENCIA.md`).

- Shen, H.; Huang, J. Z. *Annals of Applied Statistics* 2(2):601–623, 2008 (central de chamadas).
- LaLonde, R. J. *American Economic Review* 76(4):604–620, 1986.
- McNeil, A. J. *ASTIN Bulletin* 27(1):117–137, 1997 (incêndios dinamarqueses).
- Peters, J.; Bühlmann, P.; Meinshausen, N. Causal inference by using invariant prediction. *JRSS B* 78(5):947–1012, 2016.
- Heinze-Deml, C.; Peters, J.; Meinshausen, N. Invariant causal prediction for nonlinear models. *Journal of Causal Inference* 6(2), 2018.
- Gamella, J. L.; Peters, J.; Bühlmann, P. Causal chambers. *Nature Machine Intelligence* 7:107–118, 2025.
- Kemmeren, P. et al. *Cell* 157(3):740–752, 2014.
- Johannesen, L. et al. *Clinical Pharmacology & Therapeutics* 96(5):549–558, 2014.
- Gibbs, I.; Candès, E. Adaptive conformal inference under distribution shift. *NeurIPS 34*, 2021.

**Acrescentadas na revisão das tipologias** (conferidas em 15 de outubro de 2026 nos registros da Crossref e das editoras). Rubin, D. B. Inference and missing data, *Biometrika* 63(3):581–592, 1976, doi:10.1093/biomet/63.3.581. Kimball, A. W. Errors of the third kind in statistical consulting, *Journal of the American Statistical Association* 52(278):133–142, 1957, doi:10.1080/01621459.1957.10501374. Gelman, A.; Tuerlinckx, F. Type S error rates for classical and Bayesian single and multiple comparison procedures, *Computational Statistics* 15(3):373–390, 2000, doi:10.1007/s001800000040. Gelman, A.; Carlin, J. Beyond power calculations: assessing type S (sign) and type M (magnitude) errors, *Perspectives on Psychological Science* 9(6):641–651, 2014, doi:10.1177/1745691614551642. Walker, W. E.; Harremoës, P.; Rotmans, J.; van der Sluijs, J. P.; van Asselt, M. B. A.; Janssen, P.; Krayer von Krauss, M. P. Defining uncertainty: a conceptual basis for uncertainty management in model-based decision support, *Integrated Assessment* 4(1):5–17, 2003, doi:10.1076/iaij.4.1.5.16466. Stirling, A. Keep it complex, *Nature* 468(7327):1029–1031, 2010, doi:10.1038/4681029a. Mandelbrot, B. B. *Fractals and Scaling in Finance: Discontinuity, Concentration, Risk* (Selecta, vol. E), Springer, 1997. Wald, A. *Statistical Decision Functions*, Wiley, 1950. Savage, L. J. The theory of statistical decision, *Journal of the American Statistical Association* 46(253):55–67, 1951, doi:10.1080/01621459.1951.10500768. Royall, R. M. *Statistical Evidence: A Likelihood Paradigm*, Chapman & Hall, 1997. Moreno-Torres, J. G.; Raeder, T.; Alaiz-Rodríguez, R.; Chawla, N. V.; Herrera, F. A unifying view on dataset shift in classification, *Pattern Recognition* 45(1):521–530, 2012, doi:10.1016/j.patcog.2011.06.019. Ellsberg, D. Risk, ambiguity, and the Savage axioms, *Quarterly Journal of Economics* 75(4):643–669, 1961, doi:10.2307/1884324. Sweeney, L. *Simple Demographics Often Identify People Uniquely*, Data Privacy Working Paper 3, Carnegie Mellon University, 2000. Baig, S. S. et al. 22 years of predictive testing for Huntington's disease: the experience of the UK Huntington's Prediction Consortium, *European Journal of Human Genetics* 24(10):1396–1402, 2016, doi:10.1038/ejhg.2016.36. Makin, T. R.; Orban de Xivry, J.-J. Ten common statistical mistakes to watch out for when writing or reviewing a manuscript, *eLife* 8:e48175, 2019, doi:10.7554/eLife.48175.

**Do roteiro do M14 e do Lab 14 do curso** (exemplo do dique, Capítulo 5; o roteiro avisa que a GEV e os custos do Lab são ilustrativos). van Dantzig, D. Economic decision problems for flood prevention, *Econometrica* 24(3):276–287, 1956. Chernoff, H. Rational selection of decision functions, *Econometrica* 22(4), 1954.

**Deriva de rótulo e recalibração** (Capítulo 7; conferidas na fonte em 15 de outubro de 2026). Saerens, M.; Latinne, P.; Decaestecker, C. Adjusting the outputs of a classifier to new a priori probabilities: a simple procedure, *Neural Computation* 14:21–41, 2002. Lipton, Z.; Wang, Y.-X.; Smola, A. Detecting and correcting for label shift with black box predictors, *Proceedings of the 35th ICML*, PMLR 80:3122–3130, 2018. Janssen, K. J. M. et al. A simple method to adjust clinical prediction models to local circumstances, *Canadian Journal of Anesthesia* 56(3):194–201, 2009. Van Calster, B. et al. A calibration hierarchy for risk models was defined: from utopia to empirical data, *Journal of Clinical Epidemiology* 74:167–176, 2016.

**Clássicos para ler a seguir.** Bernoulli, *Ars Conjectandi* (1713); Hume, *Enquiry* (1748); Knight, *Risk, Uncertainty and Profit* (1921); Savage, *The Foundations of Statistics* (1954); Tversky e Kahneman, "Judgment under Uncertainty" (1974); Mayo, *Error and the Growth of Experimental Knowledge* (1996); Pearl, *Causality* (2000); Gigerenzer, *Calculated Risks* (2002). Primeiras edições conferidas em 15 de outubro de 2026. A primeira edição de Hume (Londres, A. Millar, 1748) saiu como Philosophical Essays Concerning Human Understanding; o título Enquiry vem da edição de 1758. As demais obras citadas nos capítulos vêm do artigo e dos módulos do curso.
