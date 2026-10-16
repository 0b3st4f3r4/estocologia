# Parte IV. Construindo a máquina auditável

## Capítulo 12. O mapa do conhecimento: o hipergrafo tipado e as cinco lentes

**Objetivos.** Ao fim deste capítulo você deve saber ler a definição $\mathcal{G} = (V, E_{\mathrm{tree}},
E_{\mathrm{graph}}, H, \Phi)$, listar os cinco campos obrigatórios de um nó, reconhecer uma aresta de
patologia e usar as cinco lentes (Fluxonomia, Crononomia, Escalonomia, Morfonomia e Crisonomia) como
perguntas que se fazem a qualquer problema.

---

### Degrau 1 — O mundo real e o paradoxo

**A conclusão que mora no meio do parágrafo.** Numa central de atendimento, um relatório começa assim:
*"Considerando o histórico de dois anos, a taxa média de chegada de 312 chamadas por hora, o tempo médio
de atendimento de 6,2 minutos e a aplicação da fórmula de Erlang C com 36 atendentes, o nível de serviço
estimado é de 82% das chamadas atendidas em até 20 segundos, e recomenda-se manter a escala atual."* É uma
frase longa, correta na aritmética e quase impossível de auditar. Se a conclusão estiver errada, o erro
está escondido em algum lugar entre "considerando" e "recomenda-se", e ninguém sabe em qual.

A primeira providência de uma máquina auditável não é calcular melhor. É **proibir a frase longa** e
obrigar cada alegação a virar um cartão com campos fixos. Um cartão que não preenche todos os campos não
entra na biblioteca. Em cada 100 relatórios escritos em prosa, uma boa parte esconde o ponto frágil num
adjetivo. Em 100 cartões com esses cinco campos, o ponto frágil aparece no campo vazio.

**O campo vazio que denuncia.** O relatório da central usa três coisas de uma vez: as chamadas que
chegaram, a fórmula que prevê a espera e a decisão da escala. Se cada uma vira um cartão separado, três
perguntas ficam visíveis de imediato. *De onde vem o número da demanda?* *Em que condições a fórmula
vale?* *Quem paga se ela falhar?* A frase escondia as três. O cartão mostra as três que estão respondidas
e as que não estão.

**Existem três tipos de ligação, e uma delas é um alarme.** Primeiro vêm as ligações de dependência. O
tempo de atendimento depende da demanda, e a espera depende do tempo de atendimento. Depois vêm as
ligações transversais. A curva de Jensen aparece na fila, no cálculo de informação e na bolsa. Por último,
as **ligações de patologia**, que não são dependências e sim desalinhamentos. A patologia mais comum desta
história é a **falácia da média**. A fórmula é aplicada à taxa média do histórico quando deveria ser
aplicada a cada dia e depois tirada a média dos resultados. Na central, os dois cálculos dão números muito
diferentes, 82% contra 57%.

**As cinco perguntas que sempre se pode fazer.** Depois de montar o mapa, restam cinco jeitos de olhar
para ele. *A conta fecha?* (informação não se cria do nada ao reprocessar). *De que tempo se fala: do
processo, do observador ou da simulação?* *A cauda é tão pesada que a média nem existe?* *A estrutura
encontrada sobrevive quando o ambiente muda?* *A informação vale o seu custo, e as previsões estão
calibradas?* Cada pergunta tem uma grandeza e um número que se pode medir. As cinco juntas são as **cinco
lentes**.

---

### Degrau 2 — A regra do jogo

**O mapa formal.** O conhecimento não fica guardado em texto ambíguo. Fica guardado como um **hipergrafo
tipado**, isto é, um conjunto de cartões (nós) ligados por três famílias de linhas.

$$\mathcal{G} = (V,\; E_{\mathrm{tree}},\; E_{\mathrm{graph}},\; H,\; \Phi)$$

| Componente | O que guarda | Exemplo da central |
| --- | --- | --- |
| $V$, nós | Conceitos, hipóteses, variáveis, conjuntos de dados | `L1_demanda_real`: contagens de chamadas a cada 5 minutos |
| $E_{\mathrm{tree}}$, árvore | O que depende do quê: pré-requisitos dedutivos | `L1_demanda` $\to$ `L2_erlang_c`, com o rótulo "entrada do modelo" |
| $E_{\mathrm{graph}}$, transversais | Pontes matemáticas, arestas causais $\operatorname{do}(x)$ e arestas de patologia | Jensen liga a função da média à informação e à ergodicidade |
| $H$, hiperarestas | As cinco lentes, que envolvem grupos inteiros de nós | Crononomia sobre `L1_demanda` e `L2_erlang_c`: "a demanda tem calendário" |
| $\Phi$, invariantes | Os invariantes que toda alegação cumpre para entrar no grafo | $\Phi_2$: a margem na unidade independente |

**Cada alegação vira código executável.** O nó central do mapa, `N_Core`, carrega a definição em uma linha e o ponteiro para o artefato que a executa. Quem afirma entrega o código que roda a afirmação. A frase do programa, cada alegação empírica vira código executável, é literal.

**Os cinco campos obrigatórios de um nó.** Um nó sem algum deles é recusado na gravação.

| Campo | O que declara |
| --- | --- |
| `id` | Identificador único |
| `camada` | $L_1$ Mundo, $L_2$ Modelo ou $L_3$ Agente |
| `tipo_incerteza` | Aleatória, epistêmica ou knightiana |
| `definicao_formal` | A definição matemática, em uma linha |
| `artefato_simulavel` | O ponteiro para o código que o reproduz com semente fixa |

Um nó real, copiado da ficha do $H_0$ em domínio real:

```yaml
- id: L2_erlang_c
  camada: L2
  tipo_incerteza: Knightiana_Especificacao
  definicao_formal: SL(c, λ) = 1 − C(c, λ·AHT) e^{−(c − λ·AHT) t / AHT}, estacionária, sem abandono
  artefato_simulavel: estocologia/central.py::nivel_servico
```

Repare que a fórmula de Erlang C está tipada como **incerteza knightiana**. Ela supõe estacionariedade e
ausência de abandono, e o Mundo não garante nenhuma das duas. O cartão diz, em um campo, onde a fórmula
pode falhar. A frase longa não dizia.

**As arestas de patologia.** Estas não são dependências, e sim desalinhamentos estruturais, o momento em
que o Agente aplica um Modelo incompatível com o Mundo.

| Patologia | O que a aresta marca |
| --- | --- |
| Falácia da média | $\mathrm{SL}(\mathbb{E}[\Lambda])$ no lugar de $\mathbb{E}[\mathrm{SL}(\Lambda)]$ |
| Paradoxo da inspeção | O Agente impõe um Modelo estável a um Mundo que oscila |
| Transposição do condicional | $P(E \mid I)$ lido como $P(I \mid E)$ — a falácia do promotor |
| Peru de Russell | O processo real está fora do espaço de hipóteses, e mais dados só aumentam a confiança no lugar errado |
| Ilusão do quarto escuro | Procura-se a causa do caso, não a causa da frequência |

**As pontes matemáticas.** As arestas transversais ligam módulos distantes por uma relação explícita,
navegável nos dois sentidos. O contrato dos agentes fixa quatro.

| Ponte | Relação | O que liga |
| --- | --- | --- |
| Jensen | $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$ para $f$ convexa | A função da média à informação e à ergodicidade |
| Priori como regularização | $-\ln P(\theta) \leftrightarrow \lambda\Omega(\theta)$ | Bayes ao aprendizado com penalidade e à elicitação SHELF do $F_7$ |
| Informação como crescimento | $\Delta W \le I(X; Y)$ | A informação mútua ao ganho de crescimento de Kelly |
| Ruído como temperatura | Ruído de minilote $\leftrightarrow$ temperatura $T$ | A mecânica estocástica à inferência ativa e ao aprendizado |

A segunda ponte diz que penalizar coeficientes grandes equivale a declarar uma priori que os acha
improváveis. Quem regulariza já calcula uma moda a posteriori, sem dizer qual priori usou. A terceira diz
que um apostador com informação lateral $Y$ sobre o resultado $X$ ganha, em taxa de crescimento, no máximo
$I(X; Y)$ bits por rodada, e esses bits viram crescimento apenas para quem aposta por Kelly. A quarta diz
que o ruído de estimar o gradiente num minilote age como uma temperatura, e ajuda a escapar de mínimos
rasos.

**As cinco lentes.** As hiperarestas funcionam como perguntas que se fazem a qualquer problema. Cada lente
tem uma grandeza que mede e um hábito crítico, e atravessa os conceitos sem acrescentar objetos novos.

| Lente | Pergunta | O que mede | Hábito crítico |
| --- | --- | --- | --- |
| Fluxonomia | Como a probabilidade e a informação fluem e se perdem? | Correntes, informação mútua | A conta fecha? Reprocessar não cria informação |
| Crononomia | A média entre muitos coincide com a média ao longo do tempo de um só? | Tempos de mistura, relaxação e escape; $\mathbb{E}[\ln M]$ | De que tempo se fala: do processo, do observador ou da simulação? |
| Escalonomia | A cauda é pesada a ponto de a média nem existir? | Expoente de cauda, tamanho efetivo, capacidade do modelo | Declarar se a média teórica existe |
| Morfonomia | A estrutura encontrada sobrevive quando o ambiente muda? | Invariância (ICP), compressibilidade (MDL) | Reescrever o modelo em duas formas e ver o que se conserva |
| Crisonomia | A informação vale o seu custo, e as previsões estão calibradas? | Perda, escore próprio, valor da informação | Cobrar de cada número o histórico de acertos na frequência que ele prometeu |

Dois exemplos de como uma lente vira conta. Na **Fluxonomia**, a desigualdade do processamento de dados
diz que, se $Z$ é obtido de $Y$, então $I(X; Z) \le I(X; Y)$. Três jornais que publicam a mesma pesquisa
valem um. Na **Morfonomia**, duas representações podem dar previsões idênticas e ainda assim autorizar
conclusões diferentes sob intervenção, e só a conclusão que se conserva entre ambientes serve fora deles.

**Como um domínio novo aciona as lentes.** A regra prática está no plano. É um processo financeiro, de
carreira ou de seguro? Aciona a **Crononomia**. Envolve sinistros, rendas, viralização ou tamanho de
empresas? Aciona a **Escalonomia**. Envolve recomendação, regulação ou previsão publicada? Aciona
**Fluxonomia** e **Morfonomia**, com simulação com e sem a ação.

**Onde ficam os dados.** O hipergrafo guarda só a topologia leve: nós, arestas, contratos YAML e hashes.
Microdados e séries ficam em Parquet particionado, consultado com projeção explícita de colunas, poda de
partição e um orçamento rígido de bytes varridos. O plano fixa também a regra de escala. Um modelo que
exige uma varredura completa por previsão não escala, por melhor que seja o erro de teste.

O tamanho do grafo cresceu com a pesquisa, mas abaixo das metas do plano. São 85 nós no $H_1$ real e
**454 178** nós no $H_3$, dos quais 453 973 são previsões e só 205 são topologia. O $H_3$ chegou a 45% do
milhão que o plano previa.

O número do $H_2$ merece uma correção, porque mostra como é fácil inflar essa conta. Os "1 538 nós" que a
primeira versão deste capítulo citava são de uma rodada antiga do gerador do grafo. Rodando o
`grafo.py` do $H_{2g}$ sobre os resultados do $H_{2g}$, o grafo tem **12 797 nós**, e
12 288 deles, 96%, são hipóteses do ICP, uma por subconjunto testado, que crescem como $2^k$ e não são
conhecimento novo.

O teste de escala da auditoria de fundo rodou o funil em lote sobre **329** domínios gerados por regra a
partir de um catálogo público (327 até o fim, a 3,8 segundos de processador por domínio) e levou o grafo a
**26 634 nós**, 28 619 arestas de árvore, 26 795 pontes e 4 628 hiperarestas. Só **6 494** dos nós são de
conceito (domínio, camadas, variáveis, ambientes, filtros, decisão), e só **554** vêm de pacotes curados;
os outros 20 140 pertencem àquela família de hipóteses. O gargalo não está na máquina, e sim na curadoria
semântica, que não escala por regra, na oferta de conjuntos com ambientes identificáveis (9% do catálogo
passa), nas pontes, que crescem com o quadrado do número de domínios, e em manter tudo em memória.

---

### Degrau 3 — Mão na massa

Três tarefas: montar um mini-hipergrafo com três nós e três lentes, provar que o validador morde em quatro
casos, e rodar as lentes com número medido. A **Escalonomia** entra pelo estimador de Hill, a
**Crisonomia** pelo valor esperado da informação perfeita (EVPI), e a **Morfonomia** pelo teste de
invariância entre regimes. Código em `crates/stochology-book/src/cap12.rs`; saída medida com
`cargo run --release` no crate de rascunho `/tmp/labs-cap12` (tabela `[workspace]` vazia, sem dependências
externas):

```text
== Capitulo 12 :: o mapa do conhecimento ==
grafo: nos=3 (L1=1 L2=1 L3=1) E_tree=2 E_graph=4 hiperarestas=3 validacao=[]
ordem topologica = ["L3_gestor", "L2_erlang_c", "L1_demanda"] | folhas=1 raizes=2 | nos sem lente = []
prova 1 (aresta para no ausente): ["aresta para no ausente: L2_erlang_c -> L2_no_fantasma"]
prova 2 (ciclo dedutivo): ["ciclo em E_tree: [\"L1_demanda\", \"L2_erlang_c\", \"L3_gestor\"]"]
prova 3 (hiperaresta com um membro): ["hiperaresta com menos de dois membros: H4"]
prova 4 (no fora de toda lente): ["L2_orfao"] validacao=[]
lentes acionadas = ["Crisonomia", "Crononomia", "Escalonomia"] | nao acionadas = ["Fluxonomia", "Morfonomia"]
Escalonomia: hill_alfa(k=250) = 1.8444 cauda_selvagem=true valores_acima_de_1000=0
Escalonomia: hill_alfa(k=500) = 1.7901 cauda_selvagem=true valores_acima_de_1000=0
Escalonomia: hill_alfa(k=1000) = 1.7949 cauda_selvagem=true valores_acima_de_1000=0
Escalonomia: hill_alfa(k=2000) = 1.8032 cauda_selvagem=true valores_acima_de_1000=0
Crisonomia: EVPI = 0.0000 | melhor ato sem informacao = 5.8000 | valor com informacao perfeita = 7.0000
Crisonomia: sem a opcao de agir (so apostar metade = 3.0000 ou nao apostar), o melhor sem informacao e 2.1000 e o EVPI sobe para 3.7000
Crisonomia: sensor inutil (pagamento igual nos dois estados): EVPI = 0.0000
Morfonomia: regra 1 -> inclinacao marginal = 24.37 (fracao tratada = 0.474) | por regime = [9.96, 10.09, 10.15] | invariante(0,5) = true
Morfonomia: regra 1 com a alocacao trocada -> inclinacao marginal = -4.60 | por regime = [10.34, 9.92, 10.15] | invariante(0,5) = true
Morfonomia: regra 2, regime do meio com efeito 40 -> inclinacao marginal = 40.03 | por regime = [9.96, 40.09, 10.15] | invariante(0,5) = false | passo dentro do regime = 30.13
```

Quatro leituras da saída, na ordem em que importam.

**O validador recusa.** O grafo válido passa com zero problemas. As quatro provas mostram que a recusa não
é decorativa: aresta para nó que não existe, ciclo dedutivo (a "volta" que faz a árvore virar roda),
hiperaresta com um membro só, e um nó órfão fora de toda lente. A prova 4 mostra o validador estrutural
deixando o nó órfão passar: a validação `[]` continua vazia, enquanto `nos_sem_lente()` aponta `L2_orfao`.
Os dois crivos cuidam de coisas distintas; um cuida da forma, o outro da cobertura.

**Nem toda lente é acionada.** O mini-hipergrafo aciona três lentes (Crononomia, Escalonomia e
Crisonomia) e deixa Fluxonomia e Morfonomia sem hiperaresta. A ausência não é erro do validador. Erro
seria se algum **nó** ficasse fora de toda lente, e nenhum fica. Uma lente sem uso é uma pergunta que
ainda não foi feita.

**A Escalonomia reproduz o número do instrumento do repositório.** Com a mesma semente (1909), 20 000
valores de uma Pareto com $\alpha = 1{,}81$ e o corte em $k = 500$, o estimador de Hill devolve
**1,7901**, e o portão do repositório publica 1,7901 nessa conta. A cauda é selvagem
($\hat\alpha < 2$), e por isso a variância teórica não existe. Nas pontas do corte o número anda, com
1,8444 em $k = 250$ e 1,8032 em $k = 2\,000$, e a sensibilidade é a lição. O expoente de cauda é uma
estimativa, e não uma constante da natureza.

**A Crisonomia depende de quais ações existem.** O sensor perfeito diz se o ano será bom ou ruim, com
probabilidade 0,70 de bom. Lucrar rende 10, perder custa 4. Quem pode *agir ou não agir* já age sem o
sensor, porque o valor esperado de agir é 5,80, maior que zero, e então o sensor vale **0,0000**, porque
ele nunca muda a decisão. Sem a opção de agir, restando *apostar metade* (valor 3,00) ou *não apostar*
(0,00), o melhor sem informação cai para 2,1000 e o sensor passa a valer **3,7000**. Se o pagamento for
igual nos dois estados, volta a valer 0,0000. **O valor da informação não é propriedade do dado: é
propriedade do conjunto de ações disponíveis.**

**A Morfonomia prefere o mapa ao número.** Três regimes, bases 100, 140 e 120, efeito verdadeiro 10 nos
três. Na primeira regra, a inclinação calculada sobre todos os dados juntos dá **24,37**, mais que o dobro
do efeito verdadeiro, porque quem recebeu o tratamento está concentrado no regime de base alta. Trocando
só a alocação, e mantendo o efeito de 10, a inclinação marginal vira **−4,60**, negativa, quando todo
efeito dentro de cada regime é positivo. É o desalinhamento clássico que o Capítulo 9 chamou de
confundimento. As inclinações por regime, essas, ficam em 9,96 / 10,09 / 10,15 e 10,34 / 9,92 / 10,15. As
duas regras passam no teste de invariância com tolerância 0,5. Quando o regime do meio realmente responde
40 em vez de 10, a invariância **falha** (9,96 contra 40,09, passo de 30,13). A inclinação marginal é um
número; a inclinação por regime é uma estrutura.

**Declaração de divergência.** A edição anterior deste livro, no Capítulo 6, apresenta as três amostras de
Hill com valores próximos de $1{,}81$; este instrumento mede 1,8444 ($k = 250$), 1,7901 ($k = 500$), 1,7949
($k = 1\,000$) e 1,8032 ($k = 2\,000$). A diferença é de amostragem e de corte, não de fórmula — o valor
publicado no repositório para $k = 500$ com semente 1909 reproduz exatamente. Fica registrado também que a
edição anterior e o plano do repositório contam portões de maneiras diferentes: o livro antigo descreve
**cinco** portões automáticos (contratos e sementes; esquema e idempotência; convergência; corte temporal;
governança, justiça e saída), enquanto o binário `portoes` implementa **sete** (G1 determinismo,
G2 semente fixa, G3 escopo declarado, G4 invariantes e tipo III, G5 integridade da trilha, G6 banca
$A_1$–$A_8$, G7 cláusula de saída), com saída de código 0 ou 1. O Capítulo 14 detalha os sete; nenhum valor
antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 12 | sem perda medida neste capitulo | o mapa do conhecimento; sem decisão de escala
```

**Exercícios.**

1. *(Conceito)* Escreva um nó $L_1$ e um nó $L_2$ para a alegação "com essa média de demanda, a espera
   média será de uns 9 minutos". Que aresta de patologia os liga, e em que sentido ela aponta?
2. *(Conceito)* Que lentes um modelo de concessão de crédito publicado aos clientes deve acionar?
   Justifique cada uma em uma frase.
3. *(Conta)* Na saída acima, o EVPI é 0,0000 com a opção de agir e 3,7000 sem ela. Explique por que o mesmo
   sensor vale zero em um caso e 3,70 no outro, e calcule o EVPI para uma terceira opção cujo melhor valor
   sem informação seja 4,00.
4. *(Laboratório)* Acrescente um quarto nó `L3_auditor` ao mini-hipergrafo e verifique o que o
   validador faz antes e depois de colocá-lo numa hiperaresta. Depois crie um segundo ciclo na árvore e
   confirme que a mensagem aponta o ciclo.
5. *(Laboratório)* Troque a tolerância do teste de invariância de 0,5 para 2,0 e para 0,05 nas duas
   primeiras regras. Em qual delas a estrutura por regime deixa de ser distinguível do ruído?
6. *(Conceito)* Por que a fórmula de Erlang C foi tipada como knightiana e não como epistêmica?

**Resumo.**

- O hipergrafo tipado guarda alegações como nós com camada, tipo de incerteza, definição formal e código.
- Arestas de patologia marcam desalinhamentos entre camadas; o validador recusa nós incompletos, pontas
  ausentes, ciclos dedutivos e hiperarestas com menos de dois membros.
- As cinco lentes são perguntas transversais com grandezas próprias: informação, tempo, cauda, invariância
  e custo da informação.
- O mesmo grafo dá números diferentes conforme a lente: Hill devolve 1,7901 para $k = 500$; o EVPI devolve
  0,0000 ou 3,7000 dependendo das ações disponíveis; a inclinação marginal troca de sinal enquanto as
  inclinações por regime ficam iguais.
- A conta de nós infla fácil: dos 26 634 nós da auditoria de escala, só 6 494 são de conceito.
