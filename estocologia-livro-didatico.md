# Estocologia: livro didático

**Edição em 21 capítulos** · cinco partes · rampa de três degraus por capítulo

Oct 16, 2026 · Silvano Neto

---

## Prefácio: como usar este livro

Este livro ensina a Estocologia de ponta a ponta. O texto apresenta os conceitos da área, o modo de
operar numa arquitetura auditável e o comportamento dessa arquitetura quando posta à prova em dados
reais, do horizonte H0 ao H3. A pesquisa que o sustenta foi encerrada em 15 de outubro de 2026.

**O que mudou nesta edição.** A edição anterior tinha 14 capítulos e supunha cálculo e probabilidade
de nível universitário já dominados. Esta edição desdobra esse conteúdo em 21 capítulos, agrupados
em cinco partes, e constrói uma **rampa pedagógica contínua**. O primeiro degrau de cada capítulo usa
apenas matemática do Ensino Médio (proporções, conjuntos, médias, árvores de contagem, gráficos de
funções do primeiro e do segundo grau). Nada da edição anterior foi jogado fora. Toda ideia, todo
teorema e todo número da pesquisa continuam aqui, redistribuídos em passos menores.

**Para quem é.** O livro serve a quatro leitores. O primeiro é o estudante do Ensino Médio, que quer
entender decisão sob incerteza sem esperar a faculdade. O segundo é o estudante de graduação ou pós
que já viu probabilidade básica. O terceiro é o profissional de dados, que precisa saber por que um
modelo aprovado no teste falha em produção. O quarto é o decisor, que lê um número e precisa saber até
onde ele o autoriza a concluir.

**O que se supõe.** Nesta edição, para os Degraus 1 e 2, apenas aritmética, porcentagens, proporções
e leitura de gráfico. Cálculo e álgebra linear aparecem só onde o texto avisa, e sempre depois da
intuição geométrica. Os laboratórios estão escritos em **Rust**, com semente fixa, no crate
`crates/stochology-book`; quem não programa pode ler a saída dos laboratórios, que é citada no
texto, e pular os exercícios marcados como *Laboratório*.

**Uma regra que vale para o livro inteiro.** Todo número citado aqui sai de um instrumento que o
leitor pode rodar. Quando um número desta edição divergir do número publicado na edição anterior ou
num roteiro do curso, a divergência é **declarada no texto**, com os dois valores, em vez de
escondida. É a regra que a pesquisa aplica a si própria (seção 9.7 da edição anterior, hoje
Parte V, Capítulo 21).

---

## A rampa de três degraus

Cada capítulo atravessa três degraus, sempre nesta ordem. Os nomes são fixos e aparecem como títulos
internos.

**Degrau 1 — O mundo real e o paradoxo.** Uma história concreta, com pessoas, números redondos e
frequências naturais ("em cada 100 pessoas..."), tabelas de contagem e gráficos. Nenhum símbolo
matemático aparece sem antes ter sido dito em palavras. Quando o capítulo precisa de uma curva, ela
aparece desenhada e descrita pela curvatura antes de virar fórmula.

**Degrau 2 — A regra do jogo.** A formalização. Cada símbolo é traduzido em palavras simples antes
da equação; cada teorema vem com a condição em que vale e o caso em que falha. É aqui que aparecem
Bayes, Jensen, Meng, Murphy, Pearl, Kelly e Chouldechova.

**Degrau 3 — Mão na massa.** Um laboratório em Rust, autocontido, com semente fixa, seguido de
exercícios de três tipos: *Conceito* (responder com palavras), *Conta* (responder com um número) e
*Laboratório* (responder com código que outra pessoa rode e obtenha resultado idêntico).

**Por que três degraus.** Porque cada ideia deste livro tem duas falhas previsíveis de aprendizado. O
estudante que aprende a fórmula sem ver o mundo aplica a fórmula no mundo errado. O estudante que vê o
mundo sem formalizar não consegue auditar nada. O terceiro degrau existe para fechar o ciclo. Se o
teorema está certo, ele roda.

---

## Mapa das cinco partes e dos 21 capítulos

| Parte | Capítulos | Pergunta que responde |
| --- | --- | --- |
| **I. A arte de mirar no escuro** | 1 a 4 | O que é julgar sob incerteza, e onde os erros moram? |
| **II. As armadilhas do mundo e dos dados** | 5 a 8 | Por que a média mente, como amostras gigantes enganam e como se mede quem prevê bem? |
| **III. Causa, tempo e sobrevivência** | 9 a 11 | O que é causa, por que o tempo arruína e quem paga pelo erro? |
| **IV. Construindo a máquina auditável** | 12 a 15 | Como transformar esse pensamento numa máquina que se audita? |
| **V. A máquina no mundo real** | 16 a 21 | Como a máquina enfrenta dados reais, horizonte por horizonte? |

| Cap. | Título | Ideia central |
| --- | --- | --- |
| 1 | O alvo invisível | *stókhos*; a costura de três tradições; por que a semente fixa é a base da conferência |
| 2 | Mundo, Modelo e Agente | a tese da ponte; Peirce; os três sentidos de viés; a falácia do promotor |
| 3 | Probabilidade sem mistério | cinco leituras da probabilidade; Bayes em árvores e em chances; decibans |
| 4 | Sorte, ignorância e o peru de Natal | os três tipos de incerteza; matriz de Stirling; o modelo que envelhece em silêncio |
| 5 | Quando a média mente | Jensen em curvas; convexidade da espera; variância cobra pedágio |
| 6 | O filtro invisível | a identidade de Meng; MCAR, MAR, MNAR; a unidade independente |
| 7 | Caçando fantasmas no ruído | modelo nulo exato; jardim dos caminhos; erros I, II, III, S e M; severidade |
| 8 | O termômetro do adivinho | escores próprios; decomposição de Murphy; calibração se conserta, resolução não |
| 9 | A escada de causa e efeito | os três degraus de Pearl; confundidor contra colisor; ICP |
| 10 | O tempo e a ruína | não-ergodicidade; arrasto da volatilidade; Kelly; caudas selvagens |
| 11 | O preço do erro | custo/perda; EVPI; Goodhart; trilema de Chouldechova; cláusula de saída |
| 12 | O mapa do conhecimento | o hipergrafo tipado; as cinco lentes; arestas de patologia |
| 13 | A equipe de especialistas | faixas F1–F8; a banca adversarial A1–A8; IA sob contrato |
| 14 | O contrato de ferro | fichas; quatro proibições; invariantes Φ1–Φ4; portões |
| 15 | O cofre e a senha | pré-registro com hash; contador K; auditoria contra si mesmo |
| 16 | H0: o posto de saúde e o banco | a mesma média, duas esperas; Erlang C contra a fila real |
| 17 | H1, parte 1: a bolsa e o emprego | alavancagem 3x; LaLonde; severidade que reprova o próprio método |
| 18 | H1, parte 2: os incêndios | cauda selvagem; Laplace; reprovar o próprio modelo é vitória |
| 19 | H2: sete rodadas de ICP | cinco falhas honestas; luz, coração e levedura |
| 20 | H3: o relógio ao vivo | zero vazamento com canário; Goodhart medido; três cofres |
| 21 | Sete lições e o que falta | balanço, auditoria de fundo e projeto final do aluno |

---

## Trilhas de leitura

| Leitor | Caminho | Tempo sugerido |
| --- | --- | --- |
| Ensino Médio | capítulos 1 a 8, só o Degrau 1 e os exercícios de *Conceito* | 8 encontros |
| Graduação | capítulos 1 a 11, os três degraus, laboratórios em Rust | um semestre |
| Profissional de dados | capítulos 1, 2, 5, 6, 8, 14, 15 e a Parte V | quatro semanas |
| Quem quer só a arquitetura | capítulos 2, 12, 13, 14, 15 | duas semanas |
| Quem quer ver o método em ação | Parte V inteira, voltando aos fundamentos quando precisar | duas semanas |

---

## Convenções

**Semente fixa.** Todo laboratório declara a semente. Duas pessoas em máquinas diferentes precisam
obter os mesmos números; é isso que torna a conferência possível (Capítulo 1, Degrau 3).

**Números medidos.** Os valores citados no Degrau 3 saem do instrumento:

```bash
cargo run --release --bin labs -- --cap 4     # um capítulo
cargo run --release --bin labs -- --todos     # todos os laboratórios
```

**Registro consolidado.** Em 2026-10-16 o andaime de medição em Python saiu da árvore. As frentes citadas
nos capítulos (`h0-real/`, `h1-real/`, `h2*/`, `h3/`) foram consolidadas em `REGISTRO_EMPIRICO.md`, que traz
cada número com a marca do instrumento Rust que o reproduz, e o material original vive no commit `1bfec905`.
As menções às frentes antigas no texto são **históricas**, e dizem de onde o número veio, não um caminho vivo.

**Declaração de divergência.** Quando este livro cita um número do repositório da pesquisa e o
instrumento desta edição devolve outro, os dois aparecem, com o motivo provável.

Os exercícios vêm em três tipos: *(Conceito)* exige palavras; *(Conta)* exige um número; *(Laboratório)* exige código
que reproduza o resultado.

**Notação.** $L_1$ é o Mundo, $L_2$ é o Modelo, $L_3$ é o Agente. $\operatorname{do}(x)$ é a
intervenção. $\mathbb{E}$ é a média de longo prazo, $\operatorname{Var}$ é a dispersão em torno
dela. O Apêndice B traduz cada símbolo do livro para o português e para o código.

# Parte I. A arte de mirar no escuro

## Capítulo 1. O alvo invisível: o que é a Estocologia?

**Objetivos.** Ao fim deste capítulo você deve saber dizer a que se dedica a Estocologia, de onde vem
o nome, quais três tradições ela costura e por que a conferência de um resultado depende de o sorteio
ser repetível.

---

### Degrau 1 — O mundo real e o paradoxo

**A decisão antes da resposta.** Numa terça-feira de manhã, uma médica atende um homem de 54 anos com
dor no peito, e não sabe como aquele caso vai terminar. Ela tem em mãos um eletrocardiograma, a idade
dele, o histórico, e quinze minutos. A decisão é entre internar e mandar para casa. Se ela internar, o
hospital gasta um leito que faltará para outro; se ela mandar para casa, e o pior acontecer, ninguém
vai esquecer o nome dela.

Repare no que acaba de acontecer. **A decisão foi tomada antes de a resposta existir.** Isso não é
descuido do sistema de saúde. É a condição de praticamente toda decisão que importa. O juiz decide sem
ter visto o crime. O gestor contrata pessoal para uma demanda que ainda não chegou. Você escolhe um
curso antes de saber se vai gostar dele.

Chamamos de **Estocologia** a ciência (e a arte) que estuda esse tipo de julgamento: **o julgamento e
a decisão sob incerteza**.

**Em cada 100 decisões.** Para tornar a conversa concreta, o livro inteiro usa frequências naturais.
Em vez de "a probabilidade de erro é de 9%", diga: *em cada 100 pacientes como este, cerca de 9
recebem o diagnóstico errado*. Em vez de "a prevalência é de 1%", diga: *em cada 1.000 pessoas que
entram na fila, 10 estão doentes*. E, em vez de "o intervalo de confiança cobre a verdade em 95% dos
casos", diga: *em cada 100 experimentos repetidos, 95 dão um intervalo que contém o valor real*. Os
números não mudam, mas o segundo jeito de dizer já mostra quem paga a conta.

**De onde vem o nome.** *Stókhos* é a palavra grega para o alvo onde se mira, e também para a
conjectura bem orientada, o palpite de quem mira no que não vê. Em 1713, três anos depois de sua
morte, foi publicado o *Ars Conjectandi*, de Jacob Bernoulli: a arte de conjecturar, que ele chamou de
*stochastice*. Com o tempo, "estocástico" passou a significar apenas "aleatório", e perdeu justamente
a parte interessante, **mirar bem quando não se vê o alvo**. Este livro quer recuperar esse sentido.

**A parte "arte".** A definição diz "ciência (ou arte)" de propósito. Há uma parte do julgamento sob
incerteza que se formaliza, e o livro inteiro mostra como: a coerência das crenças (Capítulo 3), a
atualização pela evidência (Capítulo 3), a margem de erro na unidade certa (Capítulo 6), a
calibração (Capítulo 8), o custo do erro (Capítulo 11). E há uma parte que não se formaliza por
completo: qual modelo vale a pena levar a sério, quando parar de coletar dados, quem paga se o erro
acontecer. Uma disciplina honesta sobre incerteza precisa ser honesta sobre os limites do próprio
formalismo. É por isso que todo capítulo deste livro, no fim, declara onde pode falhar.

**Por que dois computadores precisam sortear os mesmos números.** Um experimento científico precisa
ser **repetível**, e outra pessoa, em outra máquina, no ano que vem, tem de obter resultado idêntico.
Se o programa sorteia números ao acaso a cada execução, nada se pode conferir, e cada rodada dá uma
resposta diferente; a discussão vira "no meu computador deu outro valor".

A solução é simples, e **a semente** é a base de todo laboratório deste livro. Um gerador de números
pseudoaleatórios é uma receita determinística. Dada a mesma semente de partida, a receita produz a
mesma sequência de números, sempre, em qualquer máquina. O "acaso" do experimento é um acaso
*reprodutível*, aleatório para quem olha o resultado e auditável por quem sabe a semente.

---

### Degrau 2 — A regra do jogo

**Definição.** A Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.
Formalmente, ela estuda como um agente transforma informação incompleta em uma ação e responde pelas
consequências.

**A costura de três tradições.** A disciplina costura três tradições que cresceram separadas e
raramente se citam; não é um ramo novo da estatística.

- **A filosofia da indução.** Hume mostrou que toda defesa da indução pressupõe que o futuro se
  parecerá com o passado. Popper trocou a confirmação pela refutação. Lakatos mostrou como as teorias
  reagem quando a refutação chega.
- **A inferência estatística.** Pascal e Fermat, Bernoulli, Bayes, Laplace e Kolmogorov construíram a
  probabilidade; Fisher, Neyman e a escola bayesiana construíram a inferência.
- **A mecânica dos processos estocásticos.** Markov e Langevin estudaram como o acaso se acumula no
  tempo.

**Os ajustes de Lakatos.** Duhem e Quine lembraram que uma previsão frustrada refuta a teoria junto
com suas hipóteses auxiliares, nunca sozinha. Lakatos separou dois modos de salvar a teoria. O ajuste
**progressivo** prevê algo novo que depois se confirma; em 1846, a anomalia na órbita de Urano não
derrubou Newton, porque Le Verrier postulou um planeta oculto e Netuno foi encontrado. O ajuste
**degenerativo** apenas protege a teoria, e a manobra equivalente, com o planeta Vulcano, fracassou
ao explicar Mercúrio. Toda mudança de especificação depois de uma falha (e este livro relata várias)
precisa responder a uma pergunta. O conserto previu algo novo, ou só escondeu o erro?

**A pergunta que muda.** Em vez de perguntar "qual é a probabilidade?", a Estocologia formula outras
três. **Em que camada esta afirmação pode falhar? Que tipo de incerteza a ameaça? Que teste ela
teria de enfrentar para merecer confiança?** O resto do livro responde a essas três perguntas.

---

### Degrau 3 — Mão na massa

O laboratório deste capítulo não calcula nada de interessante, mas mostra *por que* tudo que vem
depois é conferível. O código está em `crates/stochology-book/src/cap01.rs`, e a saída abaixo
foi medida com `cargo run --release --bin labs -- --cap 1`.

```text
== Capítulo 1 :: o alvo invisível ==
duas maquinas com a semente 1713 concordam em 10000 de 10000 sorteios: true
sorteios coincidentes com a semente seguinte: 0
semente 1713: frequencia de caras em 100 mil jogadas = 0.5000
semente 1909: frequencia de caras em 100 mil jogadas = 0.4992
semente 2026: frequencia de caras em 100 mil jogadas = 0.4987
```

Leia linha por linha. **10000 de 10000**. Duas "máquinas" independentes, com a semente de partida em
comum, produzem sequências idênticas, e é isso que torna o experimento auditável. **0**. Quando a
semente muda, as sequências não têm nada em comum (zero coincidências em dez mil sorteios). A semente
não foi escolhida para forçar um resultado; ela é só o ponto de partida. **0,5000, 0,4992 e 0,4987**.
Três sementes diferentes, três moedas honestas, frequências de caras a menos de 0,0013 de um meio em
100 mil jogadas. O acaso, quando se acumula, se comporta.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  1 | sem perda medida neste capitulo | o capítulo apresenta o alvo e a tese; não há decisão de escala
```

**Exercícios.**

1. *(Conceito)* Explique, com suas palavras, por que a definição diz "ciência (ou arte)". Dê um
   exemplo de decisão em que a parte "arte" é inevitável.
2. *(Conceito)* Um jornal escreve: "o sorteio foi auditado e ninguém sabe o resultado antes da hora".
   Se o programa que faz o sorteio tem semente fixa, quem conhece a semente sabe o resultado? Por que
   isso não invalida o sorteio?
3. *(Conta)* Você repete o experimento e obtém 0,53 de caras em 100 jogadas. Em 100 mil jogadas,
   esperaria desvio parecido? Justifique com uma frase sobre como o acaso se comporta quando se
   acumula.
4. *(Laboratório)* Rode `labs --cap 1` e depois troque a semente de 1909 para 1909 no arquivo. O
   resultado muda? Troque para 1910 e meça quantas coincidências aparecem em 100 mil sorteios.
5. *(Conceito)* Cite uma decisão da sua vida em que a resposta só se conhece meses depois, e diga qual
   informação você gostaria de ter tido e não teve.

**Resumo.**

- Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.
- O nome vem do *stókhos* de Bernoulli (1713). Mirar bem quando o alvo não se vê.
- Ela costura filosofia da indução, inferência estatística e processos estocásticos.
- A conferência de um resultado depende de o acaso do experimento ser reprodutível. Isso exige semente
  fixa.

## Capítulo 2. Mundo, Modelo e Agente: onde moram os erros?

**Objetivos.** Ao fim deste capítulo você deve saber separar as três camadas da realidade, enunciar a
tese da ponte, aplicar as três formas de inferência de Peirce, distinguir os três sentidos da palavra
"viés" e reconhecer a falácia do promotor.

---

### Degrau 1 — O mundo real e o paradoxo

**O território, o mapa e o motorista.** Você está num carro em uma cidade desconhecida. Existe a
cidade de verdade: ruas, buracos, obras, o caminhão parado na esquina. Existe o mapa, uma folha de
papel que representa a cidade, sempre incompleta e sempre desatualizada em algum ponto. E existe o
motorista, você, com fome, com pressa, com atenção dividida, decidindo a cada cruzamento.

Os três são coisas diferentes, e o mapa nunca é o território. Mas o motorista só age pelo mapa. Ali
nasce quase todo erro que este livro estuda. **Não no mapa, e sim na passagem do mundo para o
mapa, e do mapa para a mão do motorista.**

**A fila do posto de saúde.** Em cada 100 dias, o posto recebe pacientes de duas maneiras. Em metade
dos dias chegam 5 pessoas por hora; na outra metade, chegam 11. Em média, 8 pessoas por hora. O posto atende
12 por hora.

A alegação do gestor é esta: *"a demanda média é 8, o posto atende 12, então a espera é de cerca de 10
minutos e não há por que contratar".* A conta está certa para um mundo em que chegam exatamente 8
pessoas por hora, toda hora.

Só que o mundo é o outro. Simulados com a mesma semente, os dois mundos dão:

| Mundo | Espera média por paciente | Em cada 100 pacientes, quantos esperam mais de 30 min |
| --- | --- | --- |
| Sempre 8 por hora | 9,3 minutos | 8 |
| Metade dos dias com 5, metade com 11 | 20,7 minutos | 25 |

Nenhuma conta errou. A média da demanda é 8 nos dois casos. O **mundo** mudou. Quando a
demanda oscila, a fila do dia cheio cresce muito mais do que a fila do dia vazio encolhe, porque a
espera cresce **em curva**. Quem olhou só a média, com a frase "o paciente médio espera 10 minutos",
decidiu por um mundo que não existe. O Capítulo 5 explica a curva; o Capítulo 16 refaz o caso inteiro com os
números do repositório.

---

### Degrau 2 — A regra do jogo

**As três camadas.**

| Camada | Código | O que é | O que se audita nela |
| --- | --- | --- | --- |
| Mundo | $L_1$ | A realidade que gera os dados e que nunca vemos inteira | Física do ruído, mecanismos causais, dependência de trajetória, caudas pesadas |
| Modelo | $L_2$ | A representação formal, simplificada e simulável | Premissas ocultas, espaço de hipóteses, verossimilhança, sobreajuste |
| Agente | $L_3$ | Quem decide: pessoa, instituição ou algoritmo, com tempo e atenção limitados | Atalhos heurísticos, calibração, utilidade, laços em que a ação muda o Mundo |

As três formam um laço. O Mundo gera dados, o Modelo os interpreta, o Agente age, e a ação volta a
mudar o Mundo. Cada seta é uma **ponte** onde o erro entra. Os dados chegam filtrados entre Mundo e Modelo, a previsão
é mal lida entre Modelo e Agente, e a ação deforma o Mundo que se queria medir.

**A tese da ponte.** *Uma falácia, quase sempre, não é um erro de aritmética, e sim um desalinhamento
entre camadas.* Vale a pena dizer o que essa tese **não** afirma. Ela não diz que contas erradas não
existem, nem que quem usa a arquitetura não erra. Ela diz onde procurar primeiro.

**As três formas de inferência (Peirce).** Peirce obteve dedução, indução e abdução permutando as
mesmas três proposições: regra, caso e resultado.

| Forma | Vai de | Para | Preserva a verdade? |
| --- | --- | --- | --- |
| Dedução | Regra e caso | Resultado | Sim |
| Indução | Caso e resultado | Regra | Não: amplia e pode falhar |
| Abdução | Regra e resultado | Caso (a melhor explicação) | Não: amplia e pode falhar |

A abdução escolhe a hipótese que torna os dados menos surpreendentes, mas só entre as hipóteses
consideradas. É a precursora da máxima verossimilhança e herda o limite dela, que o Capítulo 4
chamará de incerteza knightiana.

**Os três sentidos de "viés".** A palavra é usada para três coisas que a disciplina separa para
depois reunir. São o viés estatístico de um estimador (Modelo), o viés do processo que produziu os
dados (Mundo) e o viés cognitivo de quem raciocina (Agente). Nos três casos o erro não é ruído que se
cancela com mais dados, e sim uma distorção que se acumula.

**A tese tem uma versão operacional.** A maior parte dos vieses do catálogo clássico corresponde a um
modelo de acaso que foi ignorado.

| Viés | Modelo de acaso ignorado |
| --- | --- |
| Negligência da taxa-base | A verossimilhança tomada pela posterior |
| Disponibilidade, sobrevivência | Uma amostra filtrada tomada pela população |
| Ilusão de padrão | A hipótese nula de acaso esquecida |
| Falácia do apostador | Memória atribuída a um processo sem memória |
| Negligência de eventos raros | Uma cauda pesada modelada como gaussiana |
| Excesso de confiança | Má calibração |
| Raciocínio motivado | O modelo escolhido pelo resultado desejado |

**A falácia do promotor.** O Agente lê a verossimilhança do Modelo, $P(\text{evidência} \mid
\text{inocente})$, como se fosse a resposta sobre o Mundo, $P(\text{inocente} \mid \text{evidência})$.
Os dois números são diferentes, e a diferença depende inteiramente da camada $L_1$, ou seja, de quantas
pessoas inocentes se está falando.

**O paradoxo da inspeção.** O Agente impõe um Modelo estável a um Mundo que oscila, como o gestor do
posto, que projetou a fila do dia médio para todos os dias.

**A tese foi testada uma vez.** Doze casos foram montados a partir de uma lista externa de dez erros
estatísticos comuns (Makin e Orban de Xivry, 2019). Oito casos tinham um defeito plantado e quatro
estavam limpos; dois braços independentes os analisaram às cegas, um com a arquitetura deste livro,
outro com a instrução de analisar com rigor, sem agentes nem portões. Os dois apontaram os 8 defeitos e deram zero
alarme falso nos 4 casos limpos. A regra fixada de antemão dizia que **um empate no teto não sustenta a
tese**, e foi isso que o resultado disse. O empate também não a refuta. Ele mostra que, para os dez erros
dessa lista, um analista competente acha tudo o que o outro acha. Se a tese vale, vale para erros que a
lista não cobre, e um teste justo disso precisa de casos mais difíceis, escolhidos fora do programa, de
analistas humanos e de um juiz humano. O Capítulo 21 volta a esse ponto.

---

### Degrau 3 — Mão na massa

Duas tarefas: representar as camadas como tipos de dados, para que o programa não possa confundi-las, e
medir a falácia do promotor em três mundos diferentes.

**As camadas como tipos.** Em Rust, a separação deixa de ser uma recomendação de estilo e passa a ser
uma regra do compilador (`crates/stochology-core/src/camada.rs`):

```rust
pub enum Camada { Mundo, Modelo, Agente }

pub enum TipoIncerteza { Aleatoria, Epistemica, Knightiana }
```

Quem escrever uma função que recebe `Camada::Modelo` não consegue passar um `Camada::Agente`
sem escrever essa conversão à mão, e a conversão à mão é o lugar onde o leitor do código vai parar
para pensar.

**A ponte medida.** O laboratório do capítulo pergunta o seguinte. Uma coincidência de DNA ocorre por
acaso em 1 pessoa em cada 1 milhão. Em três mundos diferentes, em que a chance prévia de o réu ser o
culpado é de 1%, de 0,1% e de 0,0001%, qual é a resposta correta sobre o réu?

```text
== Capítulo 2 :: a falácia mora na ponte ==
prevalencia 1e-2: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.0001  fator da troca=99.0
prevalencia 1e-3: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.0010  fator da troca=998.0
prevalencia 1e-6: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.5000  fator da troca=499999.7
```

A primeira coluna não muda nos três mundos, **0,000001**. É a verossimilhança, e é o número que o
promotor cita. A segunda coluna é a resposta. Com evidência e verossimilhança idênticas, a chance
de inocência vai de **0,0001** (0,01%) a **0,5** (50%). A terceira coluna mostra quanto a troca mente:
99 vezes no primeiro mundo, 499.999,7 vezes no último. O erro não está na conta; está em responder
sobre o Mundo com um número do Modelo.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  2 | sem perda medida neste capitulo | as três camadas; o grão da pergunta não entra na conta aqui
```

**Exercícios.**

1. *(Conceito)* Classifique cada item como Mundo, Modelo ou Agente: a taxa real de chegada de
   pacientes; a fórmula de fila usada pelo gestor; a decisão de contratar dois enfermeiros; a
   desistência dos pacientes quando veem a fila.
2. *(Conceito)* Uma notícia diz: "a chance de o DNA coincidir por acaso é minúscula, logo a chance de o
   réu ser inocente é igualmente minúscula". Em que ponte está o erro? Que quantidade falta para chegar
   a $P(\text{inocente} \mid \text{evidência})$?
3. *(Conta)* No laboratório, com prevalência de 0,0001% ($10^{-6}$), o resultado é 0,5. Calcule à mão
   qual prevalência daria 0,09 e explique em uma frase por que a resposta depende tanto da camada $L_1$.
4. *(Laboratório)* Modifique o laboratório para usar uma coincidência de 1 em 1.000 em vez de 1 em
   1.000.000 e refaça a tabela dos três mundos.
5. *(Conceito)* Escolha três vieses da tabela da seção anterior e descreva, para cada um, um ambiente
   em que o atalho deixaria de ser erro (Capítulo 7 retoma essa ideia com Kahneman e Gigerenzer).

**Resumo.**

- Mundo ($L_1$), Modelo ($L_2$) e Agente ($L_3$) são camadas distintas, e a maior parte dos erros mora
  nas pontes entre elas.
- Peirce: dedução preserva a verdade; indução e abdução ampliam e podem falhar.
- "Viés" tem três sentidos, do dado, do estimador e da mente, e todos se acumulam.
- A falácia do promotor troca $P(E \mid I)$ por $P(I \mid E)$; a troca mente por um fator que depende
  da prevalência no Mundo.

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

## Capítulo 4. Sorte, ignorância e o peru de Natal: os três tipos de incerteza

**Objetivos.** Ao fim deste capítulo você deve saber distinguir incerteza aleatória, epistêmica e
knightiana, usar a lei da variância total para separá-las, situar um problema na matriz de Stirling e
reconhecer um modelo que envelhece em silêncio.

---

### Degrau 1 — O mundo real e o paradoxo

**O peru de Natal.** Um peru é alimentado todos os dias, pontualmente, às 9 horas da manhã. No dia 1,
ele não sabe nada; no dia 10, desconfia que a comida tem algo a ver com o galinheiro. No dia 60, já
tem uma teoria. No dia 100, ele faz a conta dos seus dias. *Em 100 manhãs, a comida chegou em 100.* Na
véspera do dia 101, o peru está **99% confiante** de que amanhã, às 9 horas, a comida chega.

O dia 101 é o Natal, e o Natal nunca esteve na lista de hipóteses do peru. Nenhum dado a mais o teria
salvo. **Mais observações só aumentaram a confiança no lugar errado.** (Bertrand Russell contou a
história com uma galinha; Nassim Taleb a recontou com o peru.)

**Em cada 100 experimentos.** Imagine dois tipos de incerteza convivendo:

- **A moeda.** Você joga uma moeda honesta 100 vezes. Em cada 100 experimentos de 100 lances, a
  proporção de caras fica perto de 50%, mas nunca igual a 50%. Jogar mais vezes **não** faz a
  proporção virar 50%. O sorteio continua sorteio.
- **A moeda desconhecida.** Você joga uma moeda sem saber se é honesta. Depois de 10 lances, ainda não
  sabe; depois de 10.000 lances, sabe quase com certeza. Aqui, mais dados resolvem.

Esses dois casos têm nomes diferentes, e confundi-los custa caro. Existe ainda um terceiro, o do peru.

---

### Degrau 2 — A regra do jogo

**A lei da variância total.** A dispersão total de um resultado se decompõe em duas parcelas:

$$\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{aleatória (Mundo)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistêmica (Modelo)}}$$

A primeira parcela nasce do sorteio e persiste quando o parâmetro é conhecido. A segunda nasce da
ignorância sobre o parâmetro e desaparece quando você o identifica e tem dados.

| Incerteza | Camada | O que mais dados fazem | Como se mitiga |
| --- | --- | --- | --- |
| Aleatória | Mundo | Nada | Margens de segurança, redundância |
| Epistêmica | Modelo | Reduzem até zero, se o parâmetro for identificável | Amostragem, atualização bayesiana |
| Knightiana | Modelo e Agente | Nada — ou pior: falsa confiança | Cenários, robustez, estresse de premissas, ampliar o espaço de hipóteses |

Frank Knight (1921) separou o **risco**, de probabilidades conhecidas, da **incerteza**, de
probabilidades que não se podem medir. No caso extremo, que este livro chama de incerteza knightiana
ou erro de especificação, o processo real está **fora** do espaço de hipóteses. Essa incerteza é a mais
perigosa das três, porque não aparece na equação. O intervalo estreita em volta da resposta errada.

**A matriz de Stirling.** Duas perguntas organizam o diagnóstico. Sabemos quais desfechos são
possíveis? Sabemos suas probabilidades?

|  | Probabilidades conhecidas | Probabilidades desconhecidas |
| --- | --- | --- |
| **Desfechos bem definidos** | Risco: calcular | Incerteza: cenários, sensibilidade |
| **Desfechos mal definidos ou disputados** | Ambiguidade: deliberar sobre o que conta como dano | Ignorância: vigilância, reversibilidade, diversidade de opções |

O peru de Russell está no canto de baixo à direita, e nenhum cálculo do canto de cima à esquerda o
salva.

**A tipologia de Walker e colegas (2003).** Cada incerteza se classifica em três dimensões. A
*localização* diz onde ela entra: contexto, estrutura do modelo, parâmetros, entradas ou resultados. O
*nível* vai da certeza determinística à ignorância total. A *natureza* separa variabilidade (aleatória)
de falta de conhecimento (epistêmica). Na arquitetura deste livro, a localização corresponde à camada e
ao campo da ficha (Capítulo 14), e o nível diz se basta uma margem ou se é preciso um teste de estresse.

**A regra de sucessão de Laplace.** Na véspera do dia 101, o peru usa:

$$P(\text{sucesso amanhã}\mid s,n)=\frac{s+1}{n+2}$$

A regra também não é neutra, porque deriva de uma priori uniforme. Com sete sucessos em dez, a uniforme
dá 0,667, uma uniforme sobre $\theta^2$ dá 0,692 e uma sobre $\sqrt{\theta}$ dá 0,652. **Até a
ignorância tem forma**, e é por isso que a escolha de priori precisa ser declarada (Capítulo 13, faixa
F7).

---

### Degrau 3 — Mão na massa

O laboratório põe um mundo que muda debaixo de um modelo que não muda. Uma moeda cujo vício **sobe de
0,2 para 0,6** ao longo de 6.400 lances, 200 repetições, com o modelo 1 supondo moeda fixa e o modelo 2
descontando o passado com fator 0,99. Código em `crates/stochology-book/src/cap04.rs`; saída
medida com `cargo run --release --bin labs -- --cap 4`:

```text
== Capítulo 4 :: a moeda cujo vício muda ==
vies sobe de 0.2 a 0.6 em 6400 passos, 200 repeticoes, desconto 0.99
cobertura do intervalo de 95%: modelo fixo = 0.1199, modelo com esquecimento = 0.9398
largura do intervalo fixo no passo 100 = 0.1564, no passo 6400 = 0.0240
largura do intervalo com esquecimento no passo 6400 = 0.1360 (razao 5.7x o fixo final)
```

O modelo fixo declara "95% de confiança" e **cobre a verdade em 12% dos passos**. O intervalo encolhe
de 0,1564 para 0,0240 enquanto o vício caminha para fora dele. Já o modelo que esquece o passado, com
um intervalo **5,7 vezes mais largo**, cobre 93,98%, perto do nominal. O preço da honestidade é a
largura do intervalo; o preço da teimosia é a confiança falsa.

**Declaração de divergência.** A edição anterior deste livro (e o roteiro do curso) registra 11% de
cobertura para o modelo fixo e 99% para o modelo com esquecimento, com larguras de 0,140 a 0,035. Este
instrumento, com 6.400 passos e desconto 0,99, mede **12%** e **93,98%**, com larguras de 0,1564 a
0,0240 e razão de 5,7×. A cobertura do modelo fixo coincide dentro de um ponto percentual; a do modelo
com esquecimento é menor porque a largura do intervalo usado aqui é a do erro-padrão do EWMA, e um
intervalo mais largo (ou um desconto menor) eleva a cobertura acima de 97% — a grade de parâmetros em
`crates/stochology-book/examples/grade_cap04.rs` mostra o efeito. Os dois valores ficam
registrados; nenhum foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  4 | sem perda medida neste capitulo | os três tipos de incerteza; o eixo R é ortogonal a eles
```

**Exercícios.**

1. *(Conceito)* Classifique cada incerteza: o resultado do próximo lance de uma moeda; o vício
   verdadeiro dessa moeda; a chance de o governo mudar a regra do jogo no ano que vem; o erro de
   medição de uma balança calibrada.
2. *(Conta)* Aplique a regra de Laplace com 7 sucessos em 10 tentativas e compare com as duas priores
   alternativas (0,667, 0,692, 0,652). Qual das três você declararia antes de ver os dados?
3. *(Conta)* Quantos desvios-padrão de distância tem o vício verdadeiro no passo 6.400, se o intervalo
   do modelo fixo tem largura 0,0240 e o vício está em 0,6 enquanto a estimativa fixa aponta para perto
   de 0,4?
4. *(Laboratório)* Mude o desconto de 0,99 para 0,98 e para 0,995 no laboratório. Explique, olhando as
   duas coberturas e as duas larguras, o compromisso entre janela de memória e cobertura.
5. *(Conceito)* Descreva um caso real (da sua área) em que o processo mudou de regime e o modelo
   continuou confiante. Em que canto da matriz de Stirling ele cai?

**Resumo.**

- Incerteza aleatória não encolhe com mais dados; epistêmica encolhe; knightiana engana.
- A lei da variância total separa as duas primeiras com precisão.
- A matriz de Stirling e a tipologia de Walker organizam o diagnóstico antes do cálculo.
- Um mundo que muda devagar derrota um modelo fixo em silêncio: a confiança nominal sobrevive ao erro.

# Parte II. As armadilhas do mundo e dos dados

## Capítulo 5. Quando a média mente: curvas e a desigualdade de Jensen

**Objetivos.** Ao fim deste capítulo você deve saber olhar um gráfico e dizer se a curva é convexa ou
côncava; enunciar a desigualdade de Jensen e a aproximação de segunda ordem que mede o pedágio da
variância; reconhecer quando o caso médio deixa de ser o resultado médio; e calcular toda margem na
unidade de fato independente.

---

### Degrau 1 — O mundo real e o paradoxo

**O posto de saúde.** Em cada 100 dias, o posto recebe pacientes de duas maneiras. Em 50 dias chegam 5
pessoas por hora; nos outros 50, chegam 11. A média é 8 pessoas por hora. O posto consegue atender 12 por hora.

O gestor olha a planilha e diz: *"a demanda média é 8, o posto atende 12, sobra folga; a espera média
tem de ficar em torno de 10 minutos, e não há por que contratar"*. Nenhuma conta errada foi feita. E,
ainda assim, o mundo que a planilha descreve não existe.

A tabela abaixo retoma os dois mundos do Capítulo 1, repetidos aqui para medir o pedágio, e a demanda
média não muda de uma linha para a outra.

| Mundo | Espera média por paciente | Em cada 100 pacientes, quantos esperam mais de 30 min |
| --- | --- | --- |
| Sempre 8 por hora | 9,3 minutos | 8 |
| Metade dos dias com 5, metade com 11 | 20,7 minutos | 25 |

A média não mudou em nada. A espera média mais que dobrou, e o número de pessoas que esperam mais de meia hora
triplicou. Uma decisão sobre contratar pessoal, tomada no primeiro mundo, é aplicada no segundo.

**A curva.** Desenhe num papel a função $y = x^2$. Ela é uma parábola com a boca para cima. Marque dois
pontos, $x = 0$ e $x = 2$. A média deles é 1, e a altura da curva em 1 é 1. Mas a média das alturas é
$(0^2 + 2^2)/2 = 2$. Olhe o desenho. O ponto da curva no meio (1) fica **abaixo** da corda que liga os
dois pontos (2). É essa a assimetria que este capítulo mede.

Numa função **convexa** (boca para cima, como $x^2$), a média das alturas é sempre maior ou igual à
altura da média. Na **côncava** (boca para baixo, como $-x^2$ ou a raiz quadrada), vale o contrário, e a
média das alturas fica abaixo da altura da média.

**A fila é uma curva de boca para cima.** Quando a demanda chega perto da capacidade de atendimento, a
espera não cresce em linha reta; cresce cada vez mais rápido. No mundo do posto, isso significa que o
dia cheio não é apenas o espelho do dia vazio. O dia cheio cobra muito mais do que o dia vazio
devolve.

**A turma de média 6,2.** Numa prova com nota mínima 6,0, a média da turma foi 6,2. Quem lê só a média
conclui que "o aluno médio foi aprovado". A conclusão exigiria um aluno que não existe, e a contagem
desmente o resumo. **Em cada 100 alunos, 45 ficaram abaixo de 6,0**. A média de 6,2 convive sem nenhum
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
só neles, quando $f$ é uma reta, ou quando $X$ não varia. Fora desses dois, sobra uma diferença, o
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
$W(5) = 3,57$ min e $W(11) = 55$ min, e a média dos dois é 29,3 minutos, quase o triplo dos 10
minutos da alegação. A derivada segunda de $W$ em $\lambda = 8$ é
$f''(8) = 120/(\mu-8)^3 = 1,875$ minuto por (paciente/hora)². Com a variância de $X$ igual a 9, o pedágio de segunda ordem é
$\tfrac{1}{2}\cdot 1,875\cdot 9 = 8,44$ minutos. É muito, mas é menos que os 19,3 minutos que faltam.
**A aproximação de segunda ordem subestima o pedágio quando a dispersão é grande**, porque a curva se
afasta da parábola antes de o dia terminar.

**A nota média e o limiar.** Nem toda função torta é uma curva do segundo grau. Reprovar é um
**limiar**, e o desfecho de cada aluno vira um degrau em 6,0. A média da turma responde a pergunta "qual é a
nota típica?" e não responde a pergunta "quantos reprovam?". Quem precisa da segunda pergunta e usa a
primeira está trocando o resumo pelo resultado, e a troca não tem conserto por mais alunos que se
incluam.

**A unidade da margem.** Todo número deste livro vem com uma margem, e a margem só tem sentido se for
calculada na **unidade de fato independente**. No posto, a pergunta é esta. Os 320 mil pacientes são 320
mil observações independentes, ou os 4 mil dias são 4 mil observações independentes? Pacientes de um
dia compartilham a demanda do dia, o servidor e a fila. Tratá-los como independentes é
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
chegam nos dias de surto, e é por isso que a média por paciente pesa os dias ruins. São 27,0 minutos nos
dias de surto contra 3,4 nos dias leves. **(3)** O pedágio de segunda ordem (8,44 min) captura menos da
metade da diferença verdadeira (19,29 min entre 10,00 e 29,29). A fórmula é local, e o posto é um caso
de dispersão grande. **(4)** O nulo de metades dá 0,201 minuto no mundo constante, e a margem calculada
por dia prevê 0,185, quase igual; a margem por paciente prevê 0,046, quatro vezes menor. No mundo que
oscila a diferença entre as duas leituras é de 5,8 vezes, com 0,563 contra 0,097. **(5)** A mediana do
mundo que oscila é 8,6 minutos, praticamente a média de 9,1 min que a alegação prometia. Quem olha "o
paciente típico" vê a promessa cumprida, e a diferença está toda na cauda. A diferença entre os mundos,
10,6 minutos, fica a 52,7 desvios do nulo.

**Declaração de divergência.** O material do curso e o README do H0 publicam, para estes mundos,
9,3 min e 20,7 min de espera média, **8** e **25** pacientes em cada 100 acima de 30 minutos, margem de
±0,51 minuto por dia contra ±0,07 por paciente, e um nulo de metades de 0,216 minuto com margens
previstas de 0,227 (por dia) e 0,047 (por paciente). Este instrumento mede **9,1** e **19,6** minutos,
**7,6** e **23,7** em cada 100, um nulo de **0,201** minuto com margens de **0,185** (por dia) e
**0,046** (por paciente) no mundo constante, e, no mundo que oscila, **0,742** de nulo com margens de
**0,563** e **0,097**. O mundo uniforme dá 13,8 min aqui contra 12,3 no material. Os números estão
todos numa vizinhança próxima. A diferença tem duas causas prováveis: o gerador (o material usa o PCG64
do NumPy com `SeedSequence(1909)`; aqui, o SplitMix64 com a semente 1909) e o começo de cada dia, que
aqui é sempre uma fila vazia. A identidade de Jensen, o sinal do pedágio e a conclusão sobre a unidade
da margem não dependem de nenhuma delas. Nenhum valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  5 | perda de resolucao 46.24 min (demanda de 5 a 11) | a média mente: o capítulo é o posto que oscila entre 5 e 11
```

**Exercícios.**

1. *(Conceito)* Para cada caso, diga se a média esconde um pedágio e de que sinal. (a) O tempo de
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

- A média da entrada não é a média da saída quando a função é curva. O resultado é $\mathbb{E}[f(X)] \ge
  f(\mathbb{E}[X])$ para $f$ convexa.
- O pedágio cresce com a curvatura e com a variância, e a fórmula de segunda ordem **subestima** o
  pedágio quando a dispersão é grande.
- A demanda média não se move, e as esperas não se parecem. Quem simula o mundo vê o que a planilha
  esconde.
- A margem só vale o que vale a unidade independente, e o nulo de metades decide qual unidade é a
  certa.
- Um limiar (reprovar, transbordar, falir) não se resume por média nenhuma.

## Capítulo 6. O filtro invisível: por que 2,4 milhões de votos valem 7 pessoas?

**Objetivos.** Ao fim deste capítulo você deve saber separar os três fatores do erro de uma amostra
(qualidade, quantidade e dificuldade); nomear o mecanismo que produz um dado ausente; calcular o
tamanho efetivo de uma amostra na unidade de fato independente; e reconhecer o filtro de seleção que
fabrica relações que não existem no mundo.

---

### Degrau 1 — O mundo real e o paradoxo

**A revista que ouviu 2,4 milhões de pessoas e errou o presidente.** Em 1936, a revista norte-americana
*Literary Digest* fez a maior pesquisa eleitoral da história até então. Recebeu **2,38 milhões** de
cédulas de leitores e anunciou, com estardalhaço, que Alfred Landon venceria Franklin Roosevelt com
**57%** dos votos. Roosevelt ganhou com **60,8%**. Naquela eleição, George Gallup acertou o vencedor
com cerca de **50 mil** entrevistas.

Em cada 100 cédulas que chegaram à redação, 57 diziam Landon. Em cada 100 votantes de verdade, 61
diziam Roosevelt. A revista não errou a aritmética, errou a **porta**. As cédulas não chegaram de um
sorteio do eleitorado, chegaram de quem tinha telefone, de quem era assinante, de quem estava
disposto a preencher e devolver um formulário. Cada uma dessas etapas é uma tela, com efeito
diferente para eleitores de Roosevelt e de Landon.

**Em cada 100 pessoas.** Vale a pena ver a conta em frequências naturais. Imagine 100 eleitores. Se a
chance de responder à pesquisa fosse igual para todos, os 100 responderiam na proporção do voto de cada
um. Não foi isso que aconteceu. Rodando o mundo de 1936 com 10 milhões de eleitores e uma chance de
responder que depende do voto, este livro mede assim:

- dos apoiadores de Roosevelt, **16,8 em cada 100** responderam;
- dos apoiadores de Landon, **35,2 em cada 100** responderam;
- no conjunto, 24 em cada 100 responderam — 2,4 milhões de cédulas.

O resultado é aritmética simples, 0,608 × 0,168 = 10,2% de Roosevelt contra 0,392 × 0,352 = 13,8% de
Landon entre os respondentes. Roosevelt fica com 10,2 de 24, isto é, **42,6%**, e Landon, com 57,4%.
A pesquisa recebeu milhões de votos e mediu a audiência da revista; não mediu o país.

**A pergunta que dá nome ao capítulo.** Quantas pessoas *sorteadas na rua* equivaleriam àqueles
2,4 milhões de cédulas? A resposta medida é **7**, sete pessoas. O desvio de uma amostra aleatória de 7
pessoas num mundo com 60,8% de Roosevelt é de 18,4 pontos, praticamente do tamanho do erro da
pesquisa.

Mais dados viciados não dão erro menor. Eles dão mais certeza sobre a resposta errada. A margem de erro que
a revista podia calcular era de 0,03 ponto, e o erro foi de 18,2 pontos, ou **661 margens**.

**A unidade independente.** Esse veneno aparece sem nenhuma má-fé, em dados "internos". Uma central
de chamadas registra 2,65 milhões de chamadas em 81 dias úteis. Um analista trata cada chamada como uma
observação e calcula a margem. As chamadas de um dia, porém, compartilham a demanda, a escala de
atendentes e o clima. Elas se repetem, não se somam. A unidade independente ali é o **dia**, e 2,65
milhões de chamadas equivalem a **81** observações. É por isso que um relatório que trate as
chamadas como independentes promete uma precisão que a unidade real não sustenta.

---

### Degrau 2 — A regra do jogo

**A identidade de Meng.** Meng (2018) mostrou que o erro de qualquer média amostral se decompõe em
três fatores, cujo produto é exato:

$$\bar Y_n - \bar Y_N \;=\; \underbrace{\rho_{R,Y}}_{\text{qualidade}}\times
\underbrace{\sqrt{\tfrac{1-f}{f}}}_{\text{quantidade}}\times \underbrace{\sigma_Y}_{\text{dificuldade}}$$

| Fator | O que é | O que ele faz |
| --- | --- | --- |
| Qualidade $\rho_{R,Y}$ | Correlação entre **entrar na amostra** e **a resposta** | É o veneno: não encolhe com $n$ |
| Quantidade $\sqrt{(1-f)/f}$ | $f$ é a fração da população amostrada | Encolhe com mais dados, mas devagar |
| Dificuldade $\sigma_Y$ | O quanto a resposta varia na população | É a física do problema |

Numa amostra **sorteada**, $\rho$ é da ordem de $1/\sqrt N$ — minúsculo — e a identidade devolve o
erro-padrão de sempre. Quando a entrada na amostra depende do que se mede, $\rho$ **não** encolhe com
$n$. O primeiro fator fica parado enquanto o segundo diminui. Com $\rho = -0,21$, $f = 0,24$ e
$\sigma_Y = 0,488$, a identidade prevê um erro de $-0,1824$, ou 18,2 pontos, o valor que a simulação
mede. Mais dados enviesados não consertam nada; eles **estreitam o intervalo em volta da resposta
errada**.

**Como tratar um dado ausente.** Rubin (1976) classificou a ausência
pelo mecanismo que a produz, e a classe decide o tratamento legítimo.

| Mecanismo | A chance de faltar depende de | Exemplo | O que se pode fazer |
| --- | --- | --- | --- |
| MCAR, completamente ao acaso | Nada que importe | Um formulário perdido no correio | Analisar os casos completos perde precisão — nenhum viés entra |
| MAR, ao acaso dado o observado | Variáveis que foram medidas | Jovens respondem menos, e a idade está na base | Ponderar ou imputar condicionando no observado |
| MNAR, não ao acaso | O próprio valor que falta | Quem ganha mais declara menos a renda | Nenhuma correção sai só dos dados: exige um modelo da ausência e análise de sensibilidade |

A distinção entre MAR e MNAR **não pode ser testada** com os dados observados. Trata-se de uma premissa,
e ela precisa ser declarada como tal. Na identidade de Meng, MNAR é o caso em que $\rho$ continua
diferente de zero depois de condicionar no que foi observado. O valor sentinela, o ausente gravado como
0 ou 9999, é pior que os outros três, porque ninguém sabe que o dado falta.

**O filtro fabrica relações.** Berkson (1946) mostrou que duas doenças independentes aparecem
associadas entre internados, porque cada uma aumenta a chance de internação. Olhar só os selecionados é
condicionar num **colisor**. Na simulação deste capítulo, talento e simpatia têm correlação $+0,001$ na
população e $-0,679$ entre os selecionados. A relação não estava escondida nos dados; o filtro a
**criou**.

**A tabela que inverte.** Simpson mostrou o caso espelhado. Charig e colegas (1986) compararam duas
cirurgias de cálculo renal.

| Grupo | Cirurgia aberta | Outra técnica |
| --- | --- | --- |
| Cálculos pequenos | 93% | 87% |
| Cálculos grandes | 73% | 69% |
| Total bruto | 78% | 83% |
| Ponderado pela população inteira | 83,3% | 77,9% |

A cirurgia aberta vence nos dois estratos e perde no total, porque os casos difíceis iam para ela.
Condicionar num confundidor fecha um caminho espúrio; condicionar num colisor abre um. "Controlar por
tudo" não é neutro, e o grafo causal decide qual conta fazer, e não a tabela (Capítulo 9).

**A unidade independente, formalizada.** O tamanho efetivo de uma amostra é uma propriedade do
**processo**, e não da contagem. Em 1913, Markov contou vogais e consoantes nas 20 000 primeiras letras
do *Ievguêni Oniéguin* e mostrou que a lei dos grandes números sobrevive à dependência, mas que a
dependência muda quanto a amostra vale. Como as letras tendem a alternar, um bloco de 100 letras vale
cerca de 330 letras independentes; numa cadeia "pegajosa", com cada estado tendendo a se repetir, 100
observações valem bem menos que 100.

$$n_{\text{ef}} \;=\; \frac{p(1-p)}{\operatorname{Var}(\bar X)}$$

Em palavras, compare a dispersão que a média amostral **realmente tem** com a que ela teria se as
observações fossem independentes. Em dados reais, 225 mil voos de Nova York valem, em informação
independente, a 364 dias; 2,65 milhões de chamadas equivalem a 81 dias. Tratar os
pacientes de um dia, ou as chamadas de uma hora, como observações independentes é
**pseudorreplicação**: a margem sai pequena demais e o teste aprova aquilo que não deveria.

**O caso da central de chamadas, inteiro.** O setor alegava o seguinte. Escalar cada meia hora pelo
Erlang C sobre a taxa média do histórico garantiria 82% das chamadas atendidas em até 20 segundos, todo
dia. Nos 81 dias do cofre, a regra padrão falha em **26 de cada 100 dias** e entrega 86%. A variação
entre dias responde pela maior parte, pois até o mundo que mantém só o total real de cada dia, com o
perfil médio, já perde 17% dos dias. Erlang C, aplicado à taxa real de cada dia, dá **57%**, no lugar dos 82% da alegação, e 65% dos dias ficam abaixo da meta; é Jensen dentro da fórmula (Capítulo 5), sem
precisar de simulação. A unidade sai do nulo de metades. São desvios de 0,0474, contra 0,0470 previsto
pelo dia e 0,0033 previsto pela chamada. A leitura por chamada promete uma margem catorze vezes menor.
Em 400 dias simulados no mundo da alegação, nenhum ficou abaixo da meta; no real, cerca de um em quatro.
A decisão que sai disso custa **+7,2% de agente-horas**, 2 428 por dia contra 2 264, e derruba a falha
de 26 para **menos de 4 em cada 100 dias**. A malha de sensibilidade, declarada antes de rodar em
`h0-real/DECLARACAO_PACIENCIA.md`, varre atendimento de 3 a 6 minutos contra paciência de 1 a 10
minutos, 24 pontos. A regra vencedora acerta em 20 deles; nos quatro restantes, todos com 1 minuto de
paciência, as quatro regras empatam porque nenhuma deixa mais de 1 dia abaixo da meta. Não há ponto de
virada. No pior canto da malha, nem a melhor regra segura a meta. São 14,2% dos dias abaixo dela, contra
37,7% da regra da alegação.

**Como isso se confere.** Nada disso é uma questão de opinião sobre quem está certo. Os rastros da
pesquisa (diários, hipergrafos, resultados) ficam fora do Git por tamanho, com as somas SHA-256
publicadas em `FORA_DO_GIT.md` e no `SHA256SUMS` de cada frente; quem quiser refazer a conta baixa o
dado bruto, regenera o rastro e confere byte a byte. O filtro é invisível no relatório e no arquivo.

---

### Degrau 3 — Mão na massa

O laboratório reconstrói a pesquisa de 1936, mede a correlação que o filtro cria, refaz a tabela de
Simpson e mede o tamanho efetivo de uma cadeia com memória. Código em `crates/stochology-book/src/cap06.rs`, saída
medida com `cargo run --release`:

```text
== Capítulo 6 :: o filtro invisível ==
Literary Digest: 2400031 de 10000000 eleitores responderam (f = 0.240); Roosevelt entre eles = 0.4259, Landon = 0.5741
erro medido = -0.1821 (-18.2 pontos); identidade de Meng com rho = -0.21: -0.1824
o erro vale o erro-padrao de uma amostra aleatoria de 7.2 pessoas; a margem de erro da pesquisa era 0.0276 pontos, e o erro foi de 18.2 pontos = 661 margens
desvio de 2000 amostras aleatorias de 7 pessoas = 0.1836 (18.4 pontos); de 50 000 entrevistas = 0.0022 (0.22 pontos)
Berkson com corte 1.0: correlacao na populacao = +0.0012, entre selecionados (24.0% da populacao) = -0.6168
Berkson com corte 1.5: correlacao na populacao = +0.0012, entre selecionados (14.4% da populacao) = -0.6790
Berkson com corte 2.0: correlacao na populacao = +0.0012, entre selecionados (7.9% da populacao) = -0.7315
Simpson: pequenos 0.931 x 0.867; grandes 0.730 x 0.688; bruto 0.780 x 0.826 (inverte)
ponderado por uma populacao meio a meio: 0.831 x 0.777 (a aberta volta a vencer)
cadeia de dois estados com persistencia 0.500: variancia da media de 100 letras = 0.002487 (independente: 0.002500), tamanho efetivo = 101
cadeia de dois estados com persistencia 0.800: variancia da media de 100 letras = 0.009899 (independente: 0.002500), tamanho efetivo = 25
cadeia de dois estados com persistencia 0.235: variancia da media de 100 letras = 0.000765 (independente: 0.002500), tamanho efetivo = 327
```

Quatro leituras. **(1)** O erro medido (−18,2 pontos) coincide com a identidade de Meng (−18,24) e vale
o desvio-padrão de uma amostra de **7,2 pessoas**. A simulação não é uma história, é uma conta
fechada. **(2)** A margem de erro da pesquisa (0,0276 ponto) não é margem nenhuma contra um veneno de
18 pontos; são 661 margens de distância. **(3)** A correlação entre talento e simpatia sai de $+0,001$
na população para **−0,679** entre os selecionados com o corte 1,5. O filtro **fabrica** a associação
negativa, e ela fica mais forte quanto mais estreita é a porta (−0,617 com 24% selecionados, −0,732 com
7,9%). **(4)** O tamanho efetivo segue o processo. São 100 letras que valem 101 observações quando não há
memória, 25 quando a cadeia é pegajosa (persistência 0,8) e 327 quando as letras alternam (persistência
0,235), o número que Markov encontrou no *Oniéguin*.

**Declaração de divergência.** A edição anterior deste livro registra, para a mesma simulação de Berkson,
uma correlação de $+0,002$ na população e $−0,669$ entre os selecionados; este instrumento mede
$+0,0012$ e $−0,679$ com o corte 1,5 (e $−0,617$ com o corte 1,0). A diferença é de um centésimo e vem
do corte de seleção, que o texto antigo não declara. Os números de 1936 convergem sem ajuste. São 2,4
milhões de cédulas, 57,4% para Landon contra os 57% publicados, e a equivalência de 7 pessoas. Nenhum
valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  6 | sem perda medida neste capitulo | a unidade independente é o Φ2; o eixo R é o primo dele no grão da pergunta
```

**Exercícios.**

1. *(Conceito)* Uma empresa diz: "temos milhões de usuários, então nossa pesquisa de satisfação é
   precisa". Que fator da identidade de Meng ela esqueceu, e o que ele tem a ver com o *Literary
   Digest*?
2. *(Conta)* Na tabela de Simpson, recalcule a taxa bruta de cada cirurgia como média dos estratos
   ponderada pelo número de casos (81, 234, 192 e 55) e explique por que ela inverte os estratos. Qual é
   a variável confundidora?
3. *(Conta)* Com $\rho = -0,21$ e $\sigma_Y = 0,488$, qual $f$ faz o erro de Meng valer 10 pontos?
   E 5 pontos? Diga em uma frase por que a pesquisa de 1936 não conseguiria chegar lá nem com 50
   milhões de cédulas.
4. *(Laboratório)* No laboratório, mude 0,168 e 0,3517 para um par em que a chance de responder seja
   igual nos dois grupos e confirme que o erro desaparece. Depois escolha um par com correlação de
   $−0,05$ e diga quantas pessoas valeria a pesquisa inteira.
5. *(Laboratório)* Rode a cadeia de dois estados com persistência 0,9 e com 0,1 e meça o tamanho
   efetivo nos dois casos. Explique, em uma frase para cada caso, o que isso faz com a margem de um
   relatório que trate as 100 letras como independentes.
6. *(Conceito)* Dê um exemplo do seu campo em que o dado ausente seja MNAR e um em que seja MAR, e
   diga o que muda na conclusão de cada um.

**Resumo.**

- O erro de uma média é qualidade × quantidade × dificuldade, e tamanho não compensa seleção.
- MCAR perde precisão, MAR se corrige condicionando no observado, MNAR só se corrige com um modelo da
  ausência declarado como premissa.
- O filtro não só distorce a média: ele cria relações que não existem na população.
- O tamanho efetivo de uma amostra é uma propriedade do processo; a margem se calcula sempre na unidade
  independente.
- 2,4 milhões de cédulas viciadas valem 7 pessoas, e a margem de erro de uma pesquisa viciada mede
  coisa nenhuma.

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
o jornal de amanhã traz um talento que não existe. Não houve erro de medida, apenas um **modelo nulo**
esquecido.

**A mão quente.** O caso mais famoso é o da "mão quente" no basquete. Gilovich, Vallone e Tversky (1985)
analisaram os arremessos de jogadores profissionais e trataram a crença de que quem acerta tende a
acertar de novo como ilusão de padrão. A proporção de acertos depois de um acerto era igual à proporção
geral. Por três décadas o resultado foi citado como o exemplo canônico de ilusão.

Em 2018, Miller e Sanjurjo mostraram que o **teste** estava viciado. A conta original agrupava todos os
arremessos e comparava a proporção de "acerto depois de acerto" com um meio. Mas essa proporção, num
processo sem memória nenhuma, **não vale um meio**. Em três lances de uma moeda honesta, a média das
proporções por sequência é $5/12 = 0{,}4167$. O alvo estava no lugar errado. Corrigida a conta, a
diferença nos dados de Cornell passa de $+3$ para $+13$ pontos percentuais. O fantasma não estava nos
jogadores. Estava no denominador.

**O jardim dos caminhos que se bifurcam.** Simmons, Nelson e Simonsohn (2011) pediram a
pesquisadores que declarassem o plano antes de coletar os dados. Quatro decisões comuns são:
*quando parar de coletar*, *quais variáveis usar*, *quais casos excluir* e *quais comparações
relatar*. Cada uma parece razoável isolada. Juntas, num experimento em que **não existe efeito
nenhum**, elas levam a taxa de falsos positivos de 5% para **60,7%**. Em cada 100 estudos feitos
assim, em 61 aparece um "efeito significativo" que não existe.

**Cinquenta casos, cinquenta preditores, nada.** Um analista tem 50 casos e 50 variáveis medidas, sem
nenhuma relação com a resposta. Ele ajusta 500 modelos diferentes e fica com o de menor erro de
validação. O vencedor parece melhor que "prever zero" em **73% das repetições**, e diante de dados
novos a vantagem some, porque não há conta errada nem dado inventado: apenas 500 tiros num alvo que
não existe, e o melhor tiro escolhido depois de ver onde ele caiu.

---

### Degrau 2 — A regra do jogo

**O modelo nulo exato.** O primeiro passo de qualquer análise não é olhar os dados, e sim escrever, em
código, o mundo em que **nada acontece**, e medir os resultados que ele produz. O modelo nulo precisa
reproduzir a estrutura dos dados, com o número de casos, o número de comparações e a liberdade de
escolha do experimento, porque dele sai o alvo certo.

A mão quente é o exemplo mais limpo. Em três lances de uma moeda honesta, as oito sequências possíveis
são igualmente prováveis. O estimador que a literatura usava é a **média das proporções por sequência**,
e as sequências sem nenhum acerto não têm proporção definida e saem da conta. Sobram seis sequências,
com proporções 1, 1/2, 0, 0, 1 e 0. A média é $2{,}5/6 = 5/12$. Já o estimador **agrupado**, total de
acertos depois de acertos sobre total de transições depois de acerto, dá $4/8 = 1/2$. Os dois
estimadores respondem a perguntas diferentes, e comparar o primeiro com 1/2 é comparar com o alvo
errado. O erro é de 0,0833, oito pontos percentuais de "mão quente" inventada.

**A severidade de Mayo.** Deborah Mayo propôs a pergunta que organiza tudo: *um resultado só é
evidência se o procedimento provavelmente teria dado errado caso a afirmação fosse falsa.* Para um teste
pré-registrado que produz $p = 0{,}04$, a severidade do resultado é $1 - p = 0{,}96$. Se a hipótese
fosse falsa, o teste teria dado um resultado menos extremo em 96 de cada 100 vezes. Quando o
"$p = 0{,}04$" sai de um jardim de caminhos, a severidade que importa deixa de ser $1 - p$, e sim $1$
menos a taxa de falso positivo **do procedimento**, porque quem produziu o número foi o procedimento
inteiro.

**O teorema de Ioannidis.** O valor preditivo de um achado positivo depende do poder $1-\beta$, das
chances $R$ de que uma hipótese da área seja verdadeira e do nível de significância $\alpha$:

$$\text{VPP}=\frac{(1-\beta)R}{R-\beta R+\alpha}$$

Em palavras: mesmo um estudo bem-feito pode ter pouca chance de estar certo, se a área testa muitas
hipóteses falsas. Com poder 0,80 e $\alpha = 0{,}05$, se uma em cada dez hipóteses é verdadeira
($R = 1/10$), o VPP é 0,615; se uma em cada mil ($R = 1/1000$), cai para 0,016. Se $\alpha$ for o do
jardim em vez de 0,05, os dois valores desabam. A Open Science Collaboration (2015) replicou 100
estudos de psicologia, e **97% dos originais** tinham $p < 0{,}05$, contra **36% das replicações**.

**Um ajuste só é evidência se o modelo podia ter falhado.** Num espaço de duas dimensões, um
semiplano separa ao acaso 50 pontos com probabilidade $2 \times 10^{-12}$, cerca de **39 bits** de
evidência. Uma tabela que guarda cada rótulo também separa os 50 pontos, e não traz bit nenhum. Ela
separaria **qualquer** conjunto de rótulos, com ou sem sinal. A separação é idêntica, e a evidência é
zero. O teste tem de poder errar para que acertar signifique algo.

**Os cinco erros.** A estatística clássica nomeia dois, e a prática acrescentou outros três.

| Erro | O que acontece | Fonte |
| --- | --- | --- |
| Tipo I | Rejeitar uma hipótese nula verdadeira: o falso alarme | Neyman e Pearson |
| Tipo II | Não rejeitar uma nula falsa: o efeito perdido | Neyman e Pearson |
| Tipo III | Dar a resposta certa ao problema errado | Kimball (1957) |
| Tipo S | Estimar um efeito com o sinal trocado | Gelman e Tuerlinckx (2000); Gelman e Carlin (2014) |
| Tipo M | Exagerar a magnitude de um efeito verdadeiro | Gelman e Carlin (2014) |

Os tipos S e M explicam por que estudos pequenos e "significativos" decepcionam na replicação. Com
pouco poder, só passam no filtro de $p < 0{,}05$ as estimativas que o acaso inflou, e algumas têm até o
sinal errado. O tipo III é o mais estocológico de todos, porque não se trata de erro de conta, mas de um
desalinhamento entre a pergunta do Agente e a pergunta que o Modelo responde. O Capítulo 5 mostra o
exemplo do gestor que perguntou pela espera média e recebeu a resposta de um mundo que não existe.

**Atalho contra ambiente.** Kahneman e Tversky catalogaram como as pessoas se afastam das regras da
probabilidade; Simon e Gigerenzer responderam que um atalho só é erro **relativo a um ambiente**. A
Estocologia propõe um critério em vez de escolher um lado. Um desvio é **erro** quando a informação
certa estava disponível e a regra de combinação foi violada; é **adaptação candidata** quando a norma
supõe uma informação que o ambiente não fornece de modo confiável.

Os dois números que separam os casos. Num rastreamento com prevalência de 1%, quem ignora a taxa-base
tem escore de Brier de **0,083**, contra **0,009** de quem a usa. Num ambulatório de referência, onde a
prevalência real é de 33%, quem insiste em 1% tem **0,28**, contra **0,084** de quem a ignora. Uma única
heurística é erro num ambiente e acerto no outro. O escore de Brier é o assunto do Capítulo 8.

**A banca adversarial.** Numa arquitetura de pesquisa, a defesa contra o jardim é um ataque formal, e não
uma boa intenção. Oito agentes tentam quebrar o artefato antes da aprovação, cada um numa faixa:

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

com $\text{Severidade}_{\min} = 6$ numa escala de 0 a 10, e a ficha repetida com a semente fixa devolve
sempre a mesma severidade. A regra é dura de propósito. A nota média da banca não aprova nada, e uma
faixa fraca declarada vale mais que um trabalho uniforme sem lacuna admitida. Nenhum agente aprova o
trabalho que produziu. Três falhas eliminam o trabalho antes de qualquer discussão de mérito: vazamento
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
"prever zero". Parece ótimo, e o erro sobe para **1,191 em dados novos**. Ele é pior que o zero, e por isso
"bate zero" apenas 30% das vezes quando o juiz é novo. A validação foi usada como alvo, e o alvo virou
veneno. **(2)** O modelo nulo exato da mão quente é 0,4167, e não 0,5; a média das proporções por
sequência coincide com 5/12 na casa dos quatro decimais, enquanto o estimador agrupado fica em 0,5000.
O fantasma é a diferença entre os dois. **(3)** As quatro liberdades clássicas levam o falso
positivo de 5,2% (teste pré-registrado) para **21,5%**, e a família grande, com parada opcional em
quatro pontos, três conjuntos de covariáveis, exclusão do caso extremo e dois subgrupos, leva para
**47,0%**. **(4)** A severidade de Mayo do "$p = 0{,}04$" pré-registrado é 0,96; a do número que sai
do jardim é 0,53, porque quem produz o número é o procedimento. **(5)** A banca A1–A8 com a
semente 2026 dá severidades 9, 8, 10, 9, 8, 9, 8 e 10 numa ficha com os oito cuidados, aprovada, e
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

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  7 | sem perda medida neste capitulo | testes severos e multiplicidade; sem decisão de escala
```

**Exercícios.**

1. *(Conceito)* Explique, sem conta, por que a média das proporções por sequência ($5/12$) é menor que
   a proporção agrupada ($1/2$). Que sequências entram numa conta e não na outra?
2. *(Conta)* Com $R = 1/20$, poder 0,80 e $\alpha = 0{,}05$, calcule o VPP. Refaça com o poder caindo
   para 0,40 e diga, em uma frase, como muda o valor de um achado positivo quando o estudo é
   pequeno.
3. *(Laboratório)* No laboratório, reduza os candidatos de 500 para 10 e depois para 5000. Meça a
   fração de vencedores que batem zero na validação e em dados novos. Desenhe os dois pontos e diga o
   que o número de tentativas faz com a aparência do vencedor.
4. *(Laboratório)* Acrescente ao jardim a liberdade "usar a média ou a mediana do desfecho, a que der o
   menor p" e meça a nova taxa de falso positivo. Que consequência isso tem para declarar as liberdades
   antes de rodar?
5. *(Conta)* Na banca adversarial, a severidade de cada agente é sorteada entre 8 e 10 quando o cuidado
   existe e entre 2 e 4 quando não existe. Calcule a probabilidade de aprovação de uma ficha com os oito
   cuidados e de uma ficha com uma única faixa sem o cuidado. Depois explique por que o desenho escolheu
   esses dois intervalos, e não um sorteio de 0 a 10.
6. *(Conceito)* Dê um exemplo do seu campo em que ignorar a taxa-base seja adaptação ao ambiente, com
   os dois números de Brier que sustentam a sua resposta.

**Resumo.**

- Todo padrão precisa de um modelo nulo exato, com a mesma estrutura dos dados e do procedimento.
- O alvo da mão quente é $5/12$, e não $1/2$: o estimador, comparado com o alvo errado, inventa o
  efeito.
- Quatro liberdades de pesquisador levam o falso positivo de 5% a dezenas por cento sem nenhum efeito
  no mundo.
- Ioannidis: o valor preditivo de um achado cai com $\alpha$ e com as chances de a hipótese ser falsa.
- Cinco erros, e não dois: I (alarme falso), II (cegueira), III (resposta certa à pergunta errada), S
  (sinal trocado) e M (exagero). A defesa é institucional: a banca A1–A8 e a severidade de Mayo
  trocam a boa intenção por um teste que podia ter falhado.

## Capítulo 8. O termômetro do adivinho: calibração, resolução e o escore de Brier

**Objetivos.** Ao fim deste capítulo você deve saber por que nem toda métrica de acerto é honesta;
calcular o escore de Brier e decompô-lo em calibração, resolução e incerteza; explicar por que a
calibração se conserta depois e a resolução não; e decidir a tolerância de calibração pelo tamanho da
amostra, e não por um número redondo.

---

### Degrau 1 — O mundo real e o paradoxo

**Dois meteorologistas.** Numa cidade chove, em média, 20 dias em cada 100. O primeiro meteorologista,
chamado aqui de *preguiçoso*, diz todos os dias a mesma frase: *"chance de chuva: 20%"*. O segundo,
chamado de *especialista*, olha o céu e diz: *"hoje, 70%"*.

Quem é melhor?

Nenhuma das duas respostas prontas serve. O preguiçoso está, no longo prazo, **certo na média**. Em
cada 100 dias em que ele anunciou 20%, choveu cerca de 20. Ele é calibrado, e não serve para levar
guarda-chuva. Em cada 100 avisos dele, 80 dias secos e 20 molhados, exatamente a taxa do lugar. O
especialista **separa** os dias. Num dia em que ele diz 70%, chove muito mais do que num dia em que ele
diz 10%. De um lado, um previsor calibrado e sem resolução; do outro, um calibrado com resolução.

Medidos num mundo de 4 mil dias, os números são estes:

| Meteorologista | Escore de Brier | Erro de calibração (REL) | Resolução (RES) |
| --- | --- | --- | --- |
| Preguiçoso (0,20 sempre) | 0,1858 | 0,0021 | 0,0002 |
| Especialista | 0,1569 | 0,0012 | 0,0267 |

O preguiçoso quase não tem erro de calibração (0,0021) e não tem resolução nenhuma (0,0002). Ele
acerta a frequência e não distingue nada. O especialista tem erro de calibração ainda menor e uma
resolução **133 vezes maior**. É por isso que ele vale mais, e é isso que a taxa de acerto não veria.

**A pergunta que a taxa de acerto não responde.** Um terceiro meteorologista é avaliado por taxa de
acerto. Ele diz "vai chover" quando a chance passa de 50% e acerta 80% das vezes. Essa métrica não
distingue o previsor que acredita em 51% do que acredita em 99%. A ação é a mesma para os dois, e a
nota também. Uma métrica assim não pune quem mente sobre a crença declarada, e por isso não serve para
auditar ninguém.

---

### Degrau 2 — A regra do jogo

**Regra de pontuação própria.** Uma regra de pontuação é **própria** (ou *estritamente própria*) quando
quem realmente acredita em $q$ minimiza a penalidade esperada declarando essa probabilidade. A regra mais
conhecida é o **escore de Brier**:

$$\text{BS} = \frac{1}{n}\sum_{i=1}^{n}(p_i - y_i)^2$$

com $p_i$ a probabilidade declarada e $y_i$ o desfecho (1 se aconteceu, 0 se não). É a média do erro
quadrático da probabilidade, e é essa propriedade que faz dela uma métrica em que **mentir sobre a
crença sempre piora a nota**. A taxa de acerto não é própria.

**Passar na média não é passar.** A checagem preditiva de Box, Gelman e Shalizi compara os dados reais com réplicas do modelo ajustado. Um Poisson ajustado à demanda da central passa na média e falha na variância. A razão variância/média observada é **273**, e o modelo subestima em dezenas de vezes a chance de um dia de pico (**180 contra 2 900** chamadas, `h0-real/README.md`). Um modelo que acerta o valor médio e erra a dispersão erra o dia que importa.

**A decomposição de Murphy.** Murphy (1973) separou o Brier em três partes:

$$\text{BS} = \underbrace{\text{REL}}_{\text{erro de calibração}} -
\underbrace{\text{RES}}_{\text{resolução}} + \underbrace{\text{UNC}}_{\text{incerteza dos eventos}}$$

- **REL** (calibração): quanto as probabilidades declaradas se afastam das frequências observadas.
  Zero significa "quando digo 30%, acontece em 30% das vezes".
- **RES** (resolução): quanto as frequências observadas se afastam da taxa-base. Zero significa "todos
  os meus avisos são o mesmo aviso".
- **UNC** (incerteza): $\bar y(1-\bar y)$, o que o desfecho tem de imprevisível para quem só conhece a
  taxa-base.

A identidade fecha quando as previsões são agrupadas em caixas (dez caixas, por exemplo) e o
Brier é calculado sobre a média de cada caixa. O **ECE** (*expected calibration error*) é o parente
não quadrático de REL, a média das distâncias absolutas entre previsto e observado por caixa.

**A assimetria prática: a calibração se conserta depois, a resolução não.** A **recalibração de Platt**
ajusta dois números, $a$ e $b$, e transforma $p$ em $\sigma(a + b\,\text{logit}(p))$. Ela conserta
distorções monótonas, um previsor sistematicamente confiante demais ou sistematicamente frouxo, e é
por isso que ela é barata. O que ela **não** faz é criar informação. Um previsor ruidoso continua
ruidoso depois de recalibrado, porque o ruído não estava na escala, estava no sinal.

Medido no laboratório, um previsor confiante demais sai de Brier 0,1712 para **0,1559**, praticamente o
valor do especialista honesto (0,1560), e toda a diferença era distorção de escala. Um previsor ruidoso
sai de 0,2119 para **0,1755**, uma melhora que ainda não alcança o especialista. O preguiçoso
recalibrado dá **0,1840**, que é o UNC do mundo. Recalibrar um previsor constante devolve só a
taxa-base.

**Quanto a calibração pode ser cobrada.** A tolerância do ECE é o ponto prático. A regra do tipo
$\mathrm{ECE} \le 0{,}03$ parece austera, mas não é. O ECE de um previsor **perfeitamente calibrado**
não é zero, e sim um número aleatório cuja escala anda com $1/\sqrt n$. A tolerância honesta é o quantil
95% desse nulo, calculado com essas previsões no $n$ do cofre, e ele encolhe quando $n$ cresce. Num
cofre de 1 488 casos o ruído é de 0,03; num de 104 489, de 0,0035. Uma tolerância fixa é, então, duas
coisas. **Apertada demais** nos cofres pequenos, onde reprova um previsor honesto, e
**frouxa demais** nos grandes, onde deixa passar descalibração perfeitamente detectável.

O H3 mediu isso em seis cofres binários:

| cofre | n | ECE observado | q95 do nulo | tolerância fixa 0,03 |
| --- | --- | --- | --- | --- |
| voos NY 2013 | 84 301 | 0,0267 | 0,0046 | passa |
| ciclones | 5 766 | 0,0123 | 0,0087 | passa |
| prisões Toronto 1997–2002 | 1 488 | 0,0363 | 0,0320 | falha |
| prisões Toronto 2020–2021 | 16 933 | 0,0668 | 0,0115 | falha |
| voos NY 2023 | 104 489 | 0,0079 | 0,0035 | passa |
| Rhode Island 2013–2015 | 24 466 | 0,0048 | 0,0034 | passa |

Aplicada a tolerância que depende de $n$, **nenhum** cofre binário do H3 passa, nem os três que a
regra fixa aprovava. O caso de Toronto 1997–2002 é o espelho. Ali o ECE de 0,0363 era ruído da ordem
da tolerância, porque o q95 nulo é 0,0320, mas o valor observado passa dele, e a reprovação se mantém.

**A resolução é a parte difícil.** O fluxo de Rhode Island terminou com ECE 0,0048 e habilidade
$+0{,}019$, isto é, calibração boa e resolução baixa. Prison numa parada de trânsito é um evento raro e
pouco previsível pelas variáveis disponíveis, e o modelo compensa isso com Platt (intercepto e
inclinação) e com o disjuntor, que põe o reserva como previsão oficial em 12% do cofre. Uma previsão
pode ser honesta e pouco informativa ao mesmo tempo, e as duas coisas se reportam **separadas**, porque
a primeira se conserta e a segunda não.

**Pôr a calibração à prova exige cofre novo.** A correção de intercepto do H3b ($\delta \leftarrow
\delta + 0{,}002(y - p')$) foi desenhada depois de ver o desvio de 2013. Para ser honesta, ela precisou
de um cofre que ninguém tinha visto, os 435 mil voos de Nova York em 2023, no trimestre que havia
falhado. Passou, com CITL 0,0015 e ECE 0,008, mas o modelo sem correção também teria passado, porque
2023 não repetiu o desvio de 2013. Fica declarado que a correção melhorou a calibração e **não foi
decisiva** para o portão.

**Duas ferramentas que vêm de graça.** A **predição conforme** dá cobertura garantida sem supor a forma
da distribuição, $P(Y \in C(X)) \ge 1-\alpha$, e foi a que o H3 usou em versão adaptativa sob deriva.
Uma regra vale para as duas. Toda previsão é pontuada por regra própria **fora do ajuste**, e nenhuma
previsão declara 0% ou 100%.

---

### Degrau 3 — Mão na massa

O laboratório pontua quatro previsores, recalibra todos com Platt e mede a tolerância do ECE em três
tamanhos de cofre. O código está em `crates/stochology-book/src/cap08.rs`, e a saída foi medida com
`cargo run --release`:

```text
== Capítulo 8 :: o termômetro do adivinho ==
4000 dias, taxa-base observada = 0.243, chance media declarada pelo especialista = 0.252
preguicoso (0,20 sempre): Brier 0.1858 | REL 0.0021 RES 0.0002 UNC 0.1840 | ECE 0.0430 | apos Platt: Brier 0.1840, ECE 0.0130 (a = -1.14, b = 0.00)
            especialista: Brier 0.1569 | REL 0.0012 RES 0.0267 UNC 0.1840 | ECE 0.0281 | apos Platt: Brier 0.1560, ECE 0.0115 (a = -0.22, b = 0.81)
        confiante demais: Brier 0.1712 | REL 0.0150 RES 0.0275 UNC 0.1840 | ECE 0.1109 | apos Platt: Brier 0.1559, ECE 0.0120 (a = -0.25, b = 0.37)
                 ruidoso: Brier 0.2119 | REL 0.0363 RES 0.0083 UNC 0.1840 | ECE 0.1449 | apos Platt: Brier 0.1755, ECE 0.0166 (a = -0.82, b = 0.27)
identidade de Murphy no especialista: REL - RES + UNC = 0.1584339, Brier das previsoes agrupadas nas caixas = 0.1584339, diferenca = 3.47e-15
a previsao crua tem Brier 0.1569462; agrupar em 10 caixas custa 1.49e-3, e a identidade fecha
 prisoes Toronto 1997-2002: n =    1488 | q50 nulo 0.0292 | q95 nulo 0.0425 | q99 nulo 0.0481 | media 0.0296 | ECE nulo acima de 0,03 em 45.1% das simulacoes
    Rhode Island 2013-2015: n =   24466 | q50 nulo 0.0028 | q95 nulo 0.0043 | q99 nulo 0.0051 | media 0.0029 | ECE nulo acima de 0,03 em 0.0% das simulacoes
              voos NY 2023: n =  104489 | q50 nulo 0.0025 | q95 nulo 0.0037 | q99 nulo 0.0042 | media 0.0026 | ECE nulo acima de 0,03 em 0.0% das simulacoes
n = 1 488, espalhamento 0.4: q50 nulo 0.0320, q95 nulo 0.0456
n = 1 488, espalhamento 1.0: q50 nulo 0.0291, q95 nulo 0.0419
n = 1 488, espalhamento 2.0: q50 nulo 0.0243, q95 nulo 0.0349
```

Cinco leituras. **(1)** A identidade de Murphy fecha até $3{,}5\times10^{-15}$ quando as previsões são
agrupadas nas dez caixas; a previsão crua tem Brier 0,1569, e o agrupamento custa 0,0015. **(2)** O
preguiçoso tem REL 0,0021 e RES 0,0002. Está calibrado e continua inútil, e a recalibração de um
previsor constante devolve exatamente o UNC (0,1840). **(3)** O confiante demais cai de 0,1712 para
0,1559, abaixo do especialista (0,1560), porque era só escala. O ruidoso cai de 0,2119 para 0,1755 e
**não alcança** o especialista, porque era sinal, e sinal perdido não volta. **(4)** A tolerância de
0,03 é reprovada por um previsor honesto em **45,1%** das simulações quando $n = 1\,488$, e em **0,0%**
quando $n$ é 24 466 ou 104 489. Nos cofres grandes ela não proíbe quase nada. **(5)** A nula depende das
previsões, não só de $n$. Com $n = 1\,488$, o q50 do ECE nulo vai de 0,0320 com previsões concentradas
a 0,0243 com previsões espalhadas.

**Declaração de divergência.** A edição anterior registra, para um exemplo de 4 000 eventos, um previsor
confiante demais que vai de Brier 0,1956 a 0,1851 com Platt, um perito honesto em 0,1846 e um previsor
ruidoso que vai de 0,2154 a 0,2089. Este instrumento mede **0,1712 → 0,1559**, **0,1560** e
**0,2119 → 0,1755**. O mundo não é o mesmo — aqui a taxa-base observada é 0,243, contra os 0,5 do
exemplo antigo —, e a ordem das grandezas é a mesma: a recalibração leva o confiante ao nível do
honesto e deixa o ruidoso atrás. A tolerância do ECE também diverge em nível: o H3 publica q95 nulo de
0,0320 para $n = 1\,488$, 0,0034 para 24 466 e 0,0035 para 104 489; aqui saem **0,0425**, **0,0043** e
**0,0037**. A diferença é a distribuição das previsões, que o README não publica — com previsões
concentradas ($n = 1\,488$, espalhamento 0,4) o q50 nulo deste instrumento é 0,0320, o número do
relatório. O que interessa não muda: o ruído do ECE anda com $1/\sqrt n$, e por isso uma tolerância fixa
é apertada num cofre pequeno e frouxa num cofre grande. Nenhum valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  8 | perda de resolucao 5.97 min (demanda de 7 a 9) | a tolerância de calibração depende do n do cofre: a oscilação de 7 a 9 é o caso leve
```

**Exercícios.**

1. *(Conceito)* Por que a taxa de acerto não é uma regra própria? Dê um exemplo de dois previsores
   com a mesma taxa de acerto e escores de Brier muito diferentes.
2. *(Conta)* Um previsor declara 0,30 em 200 dias e chove em 48 deles. Calcule REL e RES naquela
   caixa, supondo taxa-base 0,24, e diga o que aquele previsor deveria fazer com a escala.
3. *(Conta)* O ECE nulo anda com $1/\sqrt n$. Se o q95 nulo é 0,0320 com $n = 1\,488$, qual você
   espera com $n = 148\,800$? Compare com o q95 medido de 104 489 e diga se o crescimento bate com a
   previsão.
4. *(Laboratório)* No laboratório, troque a recalibração de Platt por uma simples mudança de
   intercepto ($\sigma(\text{logit}(p) + a)$, ajustando só $a$) e refaça as quatro linhas. Em que
   previsor a inclinação faz falta?
5. *(Laboratório)* Mude a semente do mundo de 2026 para 1909 e para 2027 e meça o Brier do
   especialista nas três. A que distância os três ficam, e o que isso diz sobre comparar dois
   previsores num mundo de 4 mil dias?
6. *(Conceito)* Escreva, em quatro linhas, o memorando que explica à diretoria por que o ECE de 0,0363
   de Toronto 1997–2002 não pode ser lido com a régua do ECE de 0,0079 dos voos de 2023.

**Resumo.**

- Só uma regra própria pontua crença: o escore de Brier pune quem mente sobre o que acredita.
- $\text{BS} = \text{REL} - \text{RES} + \text{UNC}$, e a identidade fecha exatamente com as previsões
  agrupadas em caixas.
- Calibrado e sem resolução é o preguiçoso: certo na média e inútil em cada dia.
- A calibração se conserta depois (Platt); a resolução, não: informação perdida não volta.
- A tolerância da calibração se calcula pelo tamanho da amostra: 0,03 é duro demais em mil e quinhentos
  casos e frouxo demais em cem mil.

# Parte III. Causa, tempo e sobrevivência

## Capítulo 9. A escada de causa e efeito: grafos, confundidores e invariância (ICP)

**Objetivos.** Ao fim deste capítulo você deve saber situar uma pergunta num dos três degraus de
Pearl, desenhar um grafo causal simples, distinguir um confundidor de um colisor, decompor uma
diferença ingênua em efeito mais viés de seleção e aplicar a predição causal invariante (ICP) para
escolher entre duas variáveis que a correlação trata como equivalentes.

---

### Degrau 1 — O mundo real e o paradoxo

**Duas cirurgias e uma tabela que se contradiz.** Em 1986, Charig e colegas compararam duas
cirurgias para cálculos renais. A cirurgia aberta venceu nos dois estratos em que os pacientes
costumam ser separados, e perdeu no total:

| Grupo | Cirurgia aberta | Outra técnica |
| --- | --- | --- |
| Cálculos pequenos | 93% de sucesso (81 de 87) | 87% (234 de 270) |
| Cálculos grandes | 73% (192 de 263) | 69% (55 de 80) |
| Total bruto | 78% (273 de 350) | 83% (289 de 350) |

A explicação está na coluna esquerda da tabela. Os casos difíceis iam para a cirurgia aberta.
**Em cada 100 pacientes operados abertamente, 75 tinham cálculos grandes; na outra técnica, 23.**
Comparar os totais é comparar dois grupos de pacientes diferentes. Se os dois procedimentos
tivessem atendido a casos com a **mesma mistura, 51% pequenos e 49% grandes, a da população
inteira**, os números seriam 83,3% para a aberta e 77,9% para a outra. A vantagem volta para a
aberta, agora por uma diferença maior do que a de qualquer estrato isolado.

Nada nessa tabela está errado. Entre a segunda e a quarta linha muda **a pergunta**.
*Entre os operados, qual técnica foi melhor?* e *para o próximo paciente, qual técnica escolher?*
são duas perguntas, e a resposta da primeira não serve à segunda. Essa distinção tem nome e tem
uma escada.

**Os três degraus.** Em cada 100 manchetes com a palavra "causa", nove em cada dez estão no
primeiro degrau. Pearl separou os degraus pela pergunta que se faz:

| Degrau | Pergunta | Exemplo |
| --- | --- | --- |
| 1. Ver | Quem tem mais de quê? | Bairros com mais policiais têm mais crimes |
| 2. Agir | Se eu mexer, o que muda? | Se dobrarmos o policiamento, o crime cai? |
| 3. Imaginar | O que teria acontecido? | Esse assalto teria acontecido com uma viatura na esquina? |

O primeiro degrau é o da correlação; o segundo exige um **do(x)**, a intervenção; o terceiro exige
um mundo paralelo. Os dados sozinhos só sustentam o primeiro. A ponte para os outros dois é um
grafo causal, **uma declaração humana sobre o mundo, e nunca uma saída de software**.

**O alarme que parece causa.** Numa unidade de terapia intensiva, o quadro da equipe acende um
alarme quando a resistência bacteriana estimada passa de um limite. O alarme prevê o desfecho
grave melhor do que qualquer outra variável do sistema. Em cada 100 pacientes que o alarme marca,
muitos evoluem mal, e a equipe começa a tratar o alarme como se ele fosse a doença.

Só que o alarme é **filho** da resistência, e não o pai dela. Quem o aciona é a resistência, e é
ela que causa o desfecho. Trocar o alarme de lugar, desligá-lo ou calibrá-lo de outro jeito não muda o
desfecho de um paciente sequer, porque mexer no filho não é mexer no pai. A correlação entre o
alarme e o desfecho é altíssima; o efeito causal do alarme é zero. Quando os dados vêm de três
hospitais diferentes, o que separa os dois é a **invariância**. O efeito da resistência se repete
em todo hospital, o do alarme não.

---

### Degrau 2 — A regra do jogo

**Grafos.** Um grafo causal tem nós (variáveis) e setas (relações diretas). $X \to Y$ diz que $X$
está na equação que gera $Y$. Três arranjos recebem nomes que vale a pena decorar:

| Arranjo | Forma | O que condicionar faz | Nome |
| --- | --- | --- | --- |
| Confundidor | $X \leftarrow Z \to Y$ | Fecha um caminho falso | Ajustar ajuda |
| Mediador | $X \to Z \to Y$ | Apaga o efeito que se quer medir | Ajustar apaga |
| Colisor | $X \to Z \leftarrow Y$ | Abre um caminho falso | Ajustar piora |

Berkson (1946) mostrou o terceiro caso com duas doenças independentes. Se cada uma aumenta a
chance de internação, olhar só os internados as faz parecer associadas. Numa simulação do curso,
talento e simpatia têm correlação de $+0{,}002$ na população e de $-0{,}669$ entre os que passam de
um limiar. **"Controlar por tudo o que foi medido" não é neutro**. Ajustar um mediador apaga o
efeito, ajustar um colisor o fabrica e ajustar um instrumento amplifica o viés que sobra.

**A porta dos fundos.** O critério de Pearl diz o que ajustar. Um conjunto $Z$ fecha a porta dos
fundos quando bloqueia todos os caminhos que entram em $X$ por trás, sem conter nenhum descendente
de $X$. Quando isso vale, a distribuição observada responde à pergunta de intervenção:

$$P(y \mid \operatorname{do}(x)) = \sum_z P(y \mid x, z)\,P(z)$$

O caso trabalhado canônico é a terapia hormonal. Em 1991, a coorte do Nurses' Health Study, com
cerca de 48 mil mulheres, relatou risco relativo de **0,56** de doença coronariana grave entre as
usuárias de estrogênio. Em 2002, o ensaio sorteado da Women's Health Initiative, com 16 608
mulheres, encontrou razão de riscos de **1,29**. As usuárias da coorte tinham outros hábitos,
outra renda e outro acesso a médicos, e uma porta dos fundos seguia aberta, até o sorteio fechá-la.

**Resultados potenciais.** Cada pessoa tem dois desfechos possíveis, $Y(1)$ com o tratamento e
$Y(0)$ sem ele, e só um é observado. A comparação entre os tratados e os não tratados dá:

$$\mathbb{E}[Y\mid D{=}1]-\mathbb{E}[Y\mid D{=}0] = \underbrace{\mathbb{E}[Y(1)-Y(0)\mid D{=}1]}_{\text{efeito nos tratados}} + \underbrace{\mathbb{E}[Y(0)\mid D{=}1]-\mathbb{E}[Y(0)\mid D{=}0]}_{\text{viés de seleção}}$$

Quando o tratamento é **sorteado**, o segundo termo é zero por construção, porque quem foi sorteado
para o tratamento tem $Y(0)$ esperado igual ao de quem não foi. O Capítulo 17 encontra isso nos
dados de LaLonde, onde o viés de seleção troca o sinal do efeito. A comparação crua dá $-8\,498$
dólares de renda em 1978 e o experimento dá $+1\,794$; a diferença, $-10\,292$, é o termo do meio.

**A diferença ingênua não é só efeito.** A decomposição dá um nome ao terceiro elemento, e ele não
tem conserto estatístico. Nada nos dados observados diz quanto ele vale, porque um dos dois
desfechos potenciais de cada pessoa está faltando por construção. Só o sorteio zera esse termo.

**Invariância: o que sobrevive à troca de ambiente.** Uma relação causal sobrevive à mudança de
ambiente; uma correlação de conveniência não. A Predição Causal Invariante, de Peters,
Bühlmann e Meinshausen (2016), transforma isso num teste. Se $S$ é o conjunto das causas diretas de
$Y$, $P(Y \mid X_S, E = e)$ é a distribuição idêntica em todo ambiente $e$ que intervenha nas
outras variáveis sem interferir na equação de $Y$.

O procedimento tem três passos. (1) Para cada subconjunto candidato $S$, testar se a relação de
$Y$ com $X_S$ é invariante entre os ambientes; (2) guardar todos os $S$ que passam; (3) devolver a
interseção deles. **A garantia é assimétrica**. $\hat S$ só contém causas diretas, com
probabilidade de pelo menos $1-\alpha$, mas nada garante que as contenha todas. Abster-se,
devolvendo o conjunto vazio, é resultado, e não falha. A abstenção acontece de dois modos. O teste
rejeita todos os conjuntos, ou aceita tantos que a interseção fica vazia.

O teste trata cada linha como unidade independente. Quando um voluntário dá 16 medidas, ou uma
escola dá 403 alunos, a unidade honesta é o voluntário ou a escola, e o veredito muda de
área para área. No H2 isso apareceu inteiro:

| Leitura | O que aconteceu |
| --- | --- |
| 6 dos 12 domínios | o ICP rejeitou todos os conjuntos: os ambientes agem direto em $Y$ |
| 4 domínios | aceitou tanto que $\hat S$ ficou vazio: os ambientes mexem pouco em $X$ |
| 1 domínio | $\hat S$ não vazio, e contra a alegação: cidadania e emprego, não a cor da pele |

**Uma lição que o H2 acrescentou: o efeito transporta melhor que o conjunto.** Quando o ICP
rejeita todos os conjuntos, o modelo ajustado nos ambientes de treino previu bem um ambiente nunca
visto em 9 domínios, com habilidade de 0,04 a 0,88. Encontrar o conjunto exato de causas é mais
exigente do que prever sob mudança.

**Por que isso importa para decidir.** A invariância é também a resposta à crítica de Lucas
(Capítulo 11). Só o que é invariante entre regimes merece uso quando o regime muda. No H2g, com a
verdade de referência declarada antes, o ICP recuperou os pais verdadeiros da corrente elétrica
numa câmara física, $\hat S = \{red, green, blue\}$, e deixou de fora sensores de luz fortemente
correlacionados, porque eram filhos dos LEDs.

---

### Degrau 3 — Mão na massa

Três tarefas: refazer a conta de Charig com as contagens, classificar perguntas nos três degraus e
construir um ICP mínimo que escolhe a causa e rejeita o alarme falso. Código verificado em
`crates/stochology-book/src/cap09.rs`; saída medida com o crate de rascunho `labs_cap09`:

```text
== Capítulo 9 :: o paradoxo de Simpson nas cirurgias de pedra nos rins ==
pequenas: aberta 81/87 = 0.9310  nova 234/270 = 0.8667
grandes:  aberta 192/263 = 0.7300  nova 55/80 = 0.6875
cru:      aberta 273/350 = 0.7800  nova 289/350 = 0.8257
peso dos estratos na populacao inteira: pequenas 0.5100, grandes 0.4900
ponderado pela populacao inteira: aberta 0.8325  nova 0.7789  -> a vantagem inverte de novo
fracao de casos dificeis na aberta = 0.7514, na nova = 0.2286

== Capítulo 9 :: os tres degraus de Pearl ==
  [   1. associação] bairros com mais policia tem mais crimes
  [  2. intervenção] se dobrarmos o policiamento, o crime cai?
  [3. contrafactual] esse assalto teria acontecido com uma viatura na esquina?

== Capítulo 9 :: um ICP minimo, tres sitios, tres candidatos ==
linhas = 12000 (4000 por sitio), X -> Y com coeficiente 2, X -> Z (filho) com ruido diferente por sitio, W puro ruido
correlacao ingenua com Y: corr(X,Y) = 0.9314  corr(Z,Y) = 0.7208  corr(W,Y) = 0.0055
inclinacao ingenua (todos os sitios juntos): X = 1.9885  Z = 0.6003  W = 0.0152
  candidato {    x}  F =   0.755  p = 4.702e-1  comum = 1.985  por sitio = [1.970, 1.996, 1.990]  ACEITO
  candidato {    z}  F = 976.809  p = 0.000e0  comum = 0.477  por sitio = [0.928, 0.496, 0.299]  rejeitado
  candidato {    w}  F =   1.474  p = 2.290e-1  comum = 0.011  por sitio = [0.056, -0.030, 0.008]  ACEITO
  candidato {  x+z}  F =   0.612  p = 6.541e-1  comum = 1.991  por sitio = [1.918, 1.998, 1.999]  ACEITO
  candidato {  x+w}  F =   0.704  p = 5.890e-1  comum = 1.985  por sitio = [1.970, 1.996, 1.990]  ACEITO
  candidato {  z+w}  F = 488.188  p = 0.000e0  comum = 0.477  por sitio = [0.928, 0.496, 0.299]  rejeitado
  candidato {x+z+w}  F =   0.624  p = 7.111e-1  comum = 1.991  por sitio = [1.919, 1.998, 1.999]  ACEITO
  candidato {vazio}  F = 0.000  p = 1.0  (so interceptos)
conjuntos aceitos: 5 -> intersecao S_hat = {}

== Capítulo 9 :: 200 repetições, mecanismo estavel: Y = 2X + ruido em todo sitio
inclinação ingênua média (todos os sítios juntos): X = 2.0004  Z = 0.6015
  {    x}  F médio =     1.04  taxa de rejeição = 0.060
  {    z}  F médio =   973.87  taxa de rejeição = 1.000
  {    w}  F médio =     1.14  taxa de rejeição = 0.085
  {  x+z}  F médio =     0.99  taxa de rejeição = 0.050
  {  x+w}  F médio =     1.04  taxa de rejeição = 0.075
  {  z+w}  F médio =   487.45  taxa de rejeição = 1.000
  {x+z+w}  F médio =     1.01  taxa de rejeição = 0.055
  {x} aceito em 0.940 das rodadas
  interseção = {x} em 0.080 das rodadas
  interseção = {z} em 0.000 das rodadas
  interseção = {w} em 0.025 das rodadas
  interseção contém x e não contém w em 0.080 das rodadas
  interseção vazia em 0.895 (dos quais 0.025 porque só o ruído W sobrou, 0.005 porque a própria causa não sobrou, e 0.890 porque W entrou em alguns conjuntos e não em outros)

== Capítulo 9 :: 200 repetições, mecanismo instavel: o coeficiente de X em Y muda de sitio para sitio
inclinação ingênua média (todos os sítios juntos): X = 4.0004  Z = 1.2029
  {    x}  F médio =     1.04  taxa de rejeição = 0.060
  {    z}  F médio =  1352.54  taxa de rejeição = 1.000
  {    w}  F médio =     1.10  taxa de rejeição = 0.070
  {  x+z}  F médio =     0.99  taxa de rejeição = 0.050
  {  x+w}  F médio =     1.04  taxa de rejeição = 0.075
  {  z+w}  F médio =   676.76  taxa de rejeição = 1.000
  {x+z+w}  F médio =     1.01  taxa de rejeição = 0.055
  {x} aceito em 0.940 das rodadas
  interseção = {x} em 0.065 das rodadas
  interseção = {z} em 0.000 das rodadas
  interseção = {w} em 0.025 das rodadas
  interseção contém x e não contém w em 0.065 das rodadas
  interseção vazia em 0.910 (dos quais 0.025 porque só o ruído W sobrou, 0.005 porque a própria causa não sobrou, e 0.905 porque W entrou em alguns conjuntos e não em outros)
```

**O que cada linha diz.**

**(1) A conta de Charig fecha.** A cirurgia aberta vence nos pequenos (0,9310 contra 0,8667) e nos
grandes (0,7300 contra 0,6875), e perde no bruto (0,7800 contra 0,8257). Ponderando os dois
estratos pela população inteira, com pesos 0,5100 e 0,4900, a aberta fica com 0,8325 e a outra com
0,7789. A vantagem inverte de novo, agora para o lado da aberta, com a diferença ampliada. Quem
produziu o paradoxo foi a mistura de casos. **75% dos pacientes da aberta tinham cálculos grandes,
contra 23% na outra técnica**. Repete-se aqui a lição dos 12 casos do Capítulo 2, agora com números
de um ensaio real.

**(2) A correlação ingênua não consegue separar a causa do sintoma.** Com 12 000 linhas em três
sítios, o desfecho $Y$ foi gerado por $Y = 2X + \text{ruído}$, o sintoma $Z$ por $Z = 2X +
\text{ruído do sítio}$ e $W$ é ruído puro. Olhando tudo junto, $\operatorname{corr}(X,Y) =
0{,}9314$ e $\operatorname{corr}(Z,Y) = 0{,}7208$. A causa e o sintoma parecem duas variáveis
explicativas, e a regressão ingênua dá inclinação **1,9885** para $X$ e **0,6003** para $Z$. Se o
analista parasse aqui, atribuiria ao alarme um efeito de 0,6.

**(3) O ICP separa os dois.** No teste de invariância, ajustando uma inclinação comum aos três
sítios contra inclinações livres:

| Candidato | F | p | Inclinação comum | Por sítio | Veredito |
| --- | --- | --- | --- | --- | --- |
| $\{X\}$ | 0,755 | 0,470 | 1,985 | 1,970 · 1,996 · 1,990 | aceito |
| $\{Z\}$ | 976,809 | ~0 | 0,477 | 0,928 · 0,496 · 0,299 | rejeitado |

A inclinação de $X$ é praticamente constante nos três sítios (1,970, 1,996, 1,990); a de $Z$ vai de
0,928 a 0,299, quase triplicando de um sítio para o outro. O mecanismo que liga a causa ao desfecho
não mudou; o que liga o sintoma ao desfecho mudou em cada lugar. A p de $Z$ é indistinguível de
zero (F perto de mil) e a de $X$ é 0,470. Em outras palavras, os dados não têm como distinguir a
relação de $X$ de uma relação que valha em todo lugar.

**(4) O que o teste acerta e onde ele se abstém.** Em 200 repetições com o mecanismo estável, o
conjunto $\{X\}$ é aceito em **94,0%** das rodadas e $\{Z\}$ é rejeitado em **100%** delas. A
interseção de todos os conjuntos aceitos, porém, só devolve $\{X\}$ em **8,0%** dos casos, e sai
vazia em 89,5%. O conjunto da causa não sobreviveu em 0,5% das vezes, e em 2,5% sobrou só o ruído
$W$, porque o teste aceitou os conjuntos que o continham e recusou os que continham $X$. Nos outros
86,5% o ruído $W$ entrou em alguns conjuntos e ficou fora de outros, e a interseção perdeu o
denominador comum. **Abster-se é resultado, e não defeito do instrumento**, e o teste nunca
devolveu $\hat S = \{Z\}$.

**(5) Quando o mecanismo é instável, a causa também cai.** Na segunda bateria, o coeficiente de
$X$ em $Y$ muda de sítio para sítio. Os dados agora dizem a verdade sobre um mundo incoerente. A
inclinação ingênua de $X$ sobe de 2,0004 para **4,0004**, porque o estimador está misturando o
efeito verdadeiro com a variação entre sítios; $Z$ vai de 0,6015 para 1,2029, e $\{X\}$ deixa de
ser uma relação estável. Quando os sítios intervêm no mecanismo, nenhum conjunto é a resposta
certa, e o teste rejeita, como deve.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  9 | sem perda medida neste capitulo | a escada de causa; o grão entra como a unidade dos ambientes
```

**Exercícios.**

1. *(Conceito)* Classifique no degrau de Pearl: "bairros com mais polícia têm mais crimes"; "se
   dobrarmos o policiamento, o crime cai?"; "esse assalto teria acontecido com uma viatura na
   esquina?".
2. *(Conceito)* Desenhe um grafo em que ajustar por $Z$ cria uma associação entre $X$ e $Y$ que não
   existia. Que nome tem $Z$? Dê um exemplo da sua área.
3. *(Conta)* Recalcule as taxas brutas da tabela de Charig como média dos estratos ponderada pelo
   número de casos, e explique por que ela inverte os estratos. Qual é a variável confundidora?
4. *(Conta)* No laboratório, a inclinação ingênua de $Z$ foi 0,6003 e as inclinações por sítio
   foram 0,928, 0,496 e 0,299. Explique em uma frase por que a média ingênua não é a média dessas
   três, e o que ela revela sobre a leitura do coeficiente de uma regressão só.
5. *(Laboratório)* Faça o ruído de $Z$ ser igual nos três sítios no laboratório e mostre que
   $\{Z\}$ passa a ser aceito. O que isso diz sobre a força do ICP? Ele encontra a causa ou apenas
   rejeita aquilo que varia?
6. *(Conceito)* Por que o ICP pode devolver o conjunto vazio quando tem dados demais? Relacione com
   a sensibilidade do teste a desvios do modelo.

**Resumo.**

- Associação, intervenção e contrafactual são perguntas diferentes; o grafo faz a ponte, e os dados
  sozinhos não a fazem.
- O paradoxo de Simpson nasce de uma mistura de casos. A cirurgia aberta vence em cada estrato e
  perde no bruto porque 75% dos seus pacientes eram os casos difíceis.
- Confundidor, mediador e colisor pedem três reações opostas; ajustar tudo não é neutro.
- Diferença ingênua = efeito nos tratados + viés de seleção, e só o sorteio zera o segundo termo.
- O ICP separa a causa do sintoma pela invariância entre ambientes, e abster-se é um resultado
  legítimo. No laboratório ele nunca devolveu o filho como causa.

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

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 10 | perda de resolucao 0.00 min (demanda plana) | o arrasto e a ruína se medem por dia; sem oscilação, refinar não rende nada
```

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

## Capítulo 11. O preço do erro, a lei de Goodhart e quem paga a conta

**Objetivos.** Ao fim deste capítulo você deve saber decidir pelo limiar custo/perda, calcular o valor
esperado da informação perfeita, usar os critérios robustos quando não há probabilidade em que
confiar, reconhecer um laço em que a previsão altera o previsto e declarar quem paga pelo erro.

---

### Degrau 1 — O mundo real e o paradoxo

**O armador que se pergunta se vale segurar.** Um armador tem 100 000 na conta. O contrato de seguro
custa 1 800 por mês e cobre uma perda de 34 000 que acontece com chance de 5%. Vale a pena?

- **Em esperança**, o armador paga 1 800 e recebe, em média, $0{,}05 \times 34\,000 = 1\,700$. Perde
  100 por mês. A seguradora ganha 100 por mês. Quem olha só essa conta conclui que o seguro é caro.
- **No tempo**, cada mês multiplica a riqueza por um fator. Sem seguro, em 95% dos meses ela é
  multiplicada por 1,0 e em 5% por 0,66. Com seguro, a multiplicação é de 0,982 todo mês, sem
  exceção. As duas contas dão números diferentes.

O critério que resolve o impasse é uma regra de uma linha: **pague para se proteger quando a chance
do evento é maior que o custo dividido pela perda**, isto é, $p > C/L$. Aqui $C/L = 1\,800/34\,000 =
5{,}294\%$. A chance é de 5%, portanto abaixo do limiar por 0,294 ponto percentual, e a regra
manda **não** segurar. É um dos casos em que a resposta da esperança e a resposta do tempo não
coincidem, e o capítulo mostra as duas.

**Em cada 100 semanas.** O vendedor de seguro do capítulo anterior tem a situação invertida. Ele
ganha 1% por semana e perde metade do capital com chance de 0,4%. Em 100 semanas, 67 terminam com
lucro; o perigo mora nas outras 33. Um histórico de 250 semanas não mostra nenhuma perda em
$0{,}996^{250} = 37\%$ das vezes, e quem estima a chance pela frequência observada conclui que ela é
zero e alavanca. É o **evento raro que não aparece na amostra**, e o Capítulo 10 já mediu o custo.

**A previsão que muda o que prevê.** Uma previsão publicada não é um termômetro, porque as pessoas
reagem a ela. Um aplicativo de trânsito que manda todos os usuários para a rota mais rápida de ontem
transfere o engarrafamento para a rota que recomendou. Com metade dos motoristas seguindo o
aplicativo, quem usa leva 41,0 minutos e quem não usa, 36,8. O previsor que divide os usuários para
igualar os tempos erra por 0,3 minuto, e todos levam 35,0. **A métrica virou meta e deixou de
medir.** Essa é a lei de Goodhart e o inverso da versão do capítulo. Aqui o previsor *desenhou* o
laço e o estabilizou.

**Quem paga quando a taxa de erro é diferente para grupos diferentes.** Um sistema de triagem
decide quem entra num programa. Ele acerta a probabilidade dentro de cada grupo (é calibrado) e,
ainda assim, erra de modos diferentes para cada um: o grupo com mais casos reais é marcado com mais
frequência sem precisar, e o grupo com menos casos é perdido com mais frequência. A saída fácil
("não use a cor no modelo") não resolve, porque a cor entra pelos correlatos.

---

### Degrau 2 — A regra do jogo

**O limiar e o valor da informação.** Se proteger custa $C$ e não proteger custa $L$ quando o evento
ocorre, protege-se quando $p > C/L$. Uma informação vale pelo quanto pode mover a probabilidade para
o outro lado desse limiar. Os bits que carrega não entram nessa conta. O valor de saber o estado do
mundo antes de agir, o EVPI, é o teto do valor de qualquer informação:

$$\mathrm{EVPI} = \mathbb{E}_\theta\big[\max_a U(a,\theta)\big] - \max_a \mathbb{E}_\theta\big[U(a,\theta)\big]$$

O valor de qualquer exame real (EVSI) fica entre zero e ele. **Bits não são valor.** Num exemplo do
curso, uma doença tem prevalência de 0,3; tratar custa 1 e deixar um doente sem tratamento custa 10,
de modo que se trata quando a chance passa de 0,10. Dois exames carregam a mesma informação mútua,
0,319 bit. O exame sensível vale 0,419, num teto (o EVPI) de 0,70. O específico vale zero, porque até
o resultado negativo dele deixa a chance acima do limiar e não muda a ação.

**Nem toda informação é bem-vinda.** Good (1967) provou que informação gratuita nunca piora a decisão
de quem tem preferências sobre estados do mundo. Mas quem tem preferências sobre as observações
futuras pode pagar para não saber. Entre pessoas com risco de doença de Huntington, só uma minoria
faz o exame genético antes dos sintomas, 17,4% no Reino Unido entre 1994 e 2014 (Baig et al., 2016).

**Critérios que dispensam probabilidades.** O limiar $C/L$ e o EVPI supõem uma distribuição sobre os
estados do mundo. No canto da incerteza da matriz de Stirling (Capítulo 4), ela não existe de modo
confiável, e a teoria da decisão oferece critérios robustos. O invariante $\Phi_4$ (Capítulo 14) exige
declarar qual deles foi usado:

| Critério | Regra | Postura |
| --- | --- | --- |
| Wald (maximin) | Escolher a ação cujo pior caso é o menos ruim | Pessimismo máximo |
| Savage (arrependimento mínimo) | Escolher a ação cujo maior arrependimento é o menor | Evitar o "se eu soubesse" |
| Laplace | Tratar os estados como igualmente prováveis | Ignorância como uniforme |
| Hurwicz | Ponderar o pior e o melhor caso por um índice de otimismo | Meio-termo declarado |

**Os diques da Holanda.** Depois da enchente de 1953, a Comissão Delta adotou para a Holanda central
um padrão de 1 em 10 000 anos, e van Dantzig (1956) tratou a altura do dique como um problema de
decisão, equilibrando o custo da obra e o dano esperado. O laboratório refaz o problema com um
modelo. As máximas anuais seguem uma GEV de forma $\xi$ desconhecida, entre $-0{,}1$ e $0{,}2$, e o
custo anual de uma altura $h$ é a obra, $h - 2$, mais o dano esperado, $3000 \times P(\text{máxima} > h)$.
Cinco regras de decisão dão cinco alturas diferentes, e o critério e a lista de modelos têm de ser
declarados antes de ver a resposta. Chernoff (1954) mostrou que o critério de Savage pode mudar de
escolha só por acrescentar uma alternativa que ninguém escolheria; aqui se acrescenta um modelo.
**Os critérios robustos não são mais "corretos" que o esperado: são a escolha honesta quando a
probabilidade de que a conta depende não merece confiança.**

**A previsão que muda o que prevê.** Quando um modelo é publicado, as pessoas reagem a ele. Lucas
(1976) mostrou o fenômeno na economia. Os parâmetros de um modelo estimado sob uma política dependem
das regras que os agentes esperam, e mudam quando a política muda. Num exemplo do curso, a curva de
Phillips antiga promete 4,59% de desemprego e o regime novo entrega 5,88%. A resposta de Kuhn e
Perdomo e colegas (2020) é conceitual: **performativamente estável** é o modelo ótimo para a
distribuição que ele induz.

**Aprender ou deformar?** Um agente que minimiza a energia livre variacional pode baixar a surpresa de
três modos: mudando a crença, mudando o mundo ou deixando de olhar.

$$F[q] = D\big(q \,\|\, p(s \mid o)\big) - \ln p(o) \;\ge\; -\ln p(o)$$

Um termostato que "acredita" em 22 °C numa sala a 18 °C chega a um $F$ idêntico quando aprende que a
sala está fria e quando a aquece até 22 °C. Só um contrafactual de fora separa os dois casos, e por
isso a arquitetura exige rodar o modelo com e sem a ação.

**O laço do H3, medido.** O H3 supôs que quem desiste da fila volta a chamar com probabilidade 0,6.
Esse parâmetro foi medido em outra central real, de 1999, com um desenho quase experimental. A fila
encontrada ao entrar, em data e hora idênticas, serve de instrumento para a desistência. Por esse
desenho, desistir em vez de ser atendido aumenta em **0,145** a chance de aquele cliente voltar a
ligar em até quinze minutos, com intervalo de 90% de 0,132 a 0,157. O sinal confere e a ordem de
grandeza não. O laço declarado é cerca de quatro vezes o observado, fora da faixa de um terço a três
que estava fixada de antemão. O excesso de chegadas que o laço medido gera nessa central é de 2,3%
das chegadas, contra o viés de 29% que o laço declarado produziu no H3; a comparação é de ordem de
grandeza, porque o viés do H3 depende também da taxa de desistência simulada, e não só do laço. E o
valor medido é provavelmente um limite inferior, porque uma chamada nova só é visível quando entra na
fila.

No H3, o controle do laço foi feito com um pequeno experimento embutido na operação. Em 10% dos
intervalos, um sorteio multiplicava a escala de atendentes por 0,7 ou 1,3. Como o sorteio independe
da demanda, a regressão das chegadas seguintes no déficit sorteado identifica o efeito da ação. O
previsor ingênuo, treinado nas chegadas observadas, **superestima a demanda exógena em 29%** e ainda
assim acerta os valores observados com 1% de erro. A métrica do alvo aprova um previsor errado sobre
o mundo. Com o instrumento, $\hat r$ sai em 0,56 (intervalo de 90% de 0,51 a 0,61) e o erro da
previsão corrigida cai para **1,7%**, dentro do limite de 3%.

**Quem paga pelo erro.** Quando as taxas-base diferem entre grupos, um limiar único produz taxas de
erro diferentes. A identidade de Chouldechova (2017) mostra por quê:

$$\text{FPR}=\frac{p}{1-p}\cdot\frac{1-\text{VPP}}{\text{VPP}}\cdot(1-\text{FNR})$$

Com valor preditivo (VPP) e falsos negativos iguais entre grupos, prevalências $p$ diferentes forçam
taxas de falso positivo diferentes. **Igualar aprovações, igualar erros e igualar calibração por grupo
são exigências incompatíveis quando as prevalências diferem e o preditor não é perfeito.** "Quero
justiça" ainda não é uma decisão. É preciso declarar que igualdade se escolheu, quanto ela custa, para
onde o erro migrou e quem passa a pagá-lo.

**Comunicar é parte da decisão.** Um resultado mal comunicado falhou na camada do Agente. "30% de
chance" não diz 30% de quê; "de cada 100 pessoas como você, 30" diz. As regras são denominadores
fixos, risco absoluto antes do relativo e classe de referência explícita. Gráficos de resultados
hipotéticos (HOPs), que mostram desfechos simulados um após o outro, fazem o leitor sentir a variação
que uma barra de erro esconde. A recomendação declara o ponto em que a ação muda, agir se $p > C/L$,
e uma cláusula objetiva de quando deixa de valer.

---

### Degrau 3 — Mão na massa

Quatro tarefas: a calculadora de custo/perda no navio de Bernoulli com o crescimento no tempo, o EVPI
e o EVSI em um caso de dois atos, os quatro modelos de cauda do dique com cinco regras de decisão, e a
tabela do trilema com a identidade de Chouldechova. Código verificado em `crates/stochology-book/src/cap11.rs`; saída
medida com o crate de rascunho `labs_cap11`:

```text
== Capítulo 11 :: o navio de Bernoulli (C = 1.800, L = 34.000, p = 0,05) ==
limiar p > C/L: p* = 0.05294 (5.294%)  ->  p = 0,050 nao passa o limiar
em esperança: perda esperada sem seguro = 1700, resultado do seguro = +100 por mês
crescimento no tempo do armador: 1 navio sem seguro = -0.0208, 1 navio com seguro = -0.0182, 3 navios sem seguro = -inf
seguradora com     1 contratos: crescimento no tempo = -0.000186 por mês
seguradora com    10 contratos: crescimento no tempo = -0.001808 por mês
seguradora com   100 contratos: crescimento no tempo = -0.013466 por mês
seguradora com  1000 contratos: crescimento no tempo = +0.100410 por mês

== Capítulo 11 :: o EVPI (tratar custa 1, deixar um doente custa 10) ==
limiar de ação: p > 0.10  ->  com p = 0,3 o melhor ato sem informação tratar (-1.00)
ganho de quem sabe o estado antes de agir = -0.30
EVPI = +0.70
  exame sensivel  (0,90; 0,70): EVSI = +0.2400
  exame especifico (0,30; 0,95): EVSI = +0.2450
  com prevalência 0.05: melhor ato sem informação = nao tratar, EVPI = +0.450
  com prevalência 0.30: melhor ato sem informação = tratar, EVPI = +0.700
  com prevalência 0.60: melhor ato sem informação = tratar, EVPI = +0.400

== Capítulo 11 :: os diques de van Dantzig, quatro modelos de cauda ==
  modelo    xi = -0.1  xi = +0.0  xi = +0.1  xi = +0.2   altura ótima
  T = 100          4.07       1.26       5.71       7.01
  T = 10000        5.84       1.26      12.78      21.49
  altura ótima com xi = -0.1: 5.839 m  (custo 4.143)
  altura ótima com xi = +0.0: 3.059 m  (custo 1.130)
  altura ótima com xi = +0.1: 9.809 m  (custo 9.426)
  altura ótima com xi = +0.2: 12.584 m  (custo 13.613)
  valor esperado com o Gumbel: 3.06 m
  media dos modelos: 7.82 m
  maximin de Wald: 12.58 m
  arrependimento minimax de Savage: 9.81 m (maior arrependimento 6.68)
  precaucao: 1 em 10 000 no pior modelo = 21.49 m
  EVPI do conjunto de modelos = +18.427 m

== Capítulo 11 :: o trilema de Chouldechova ==
grupo     taxa-base        FNR        FPR falsos positivos falsos negativos        VPP
White         0.069      0.240      0.105              467               79     0.9408
Black         0.111      0.360      0.196              832              191     0.9096
casos observados: White 329 marcados pelos fatos, Black 530

-- FNR igualado para baixo, ao FNR do White em 0.240 --
White         0.069      0.240      0.105              467               79
Black         0.111      0.240      0.045              192              127
falsos positivos totais: 1299 -> 659 (-640)
falsos negativos totais: 270 -> 206 (-64)

-- FNR igualado para cima, ao FNR do Black em 0.360 --
White         0.069      0.360      0.246             1095              119
Black         0.111      0.360      0.196              832              191
falsos positivos totais: 1299 -> 1927 (+628)
falsos negativos totais: 270 -> 309 (+40)

identidade de Chouldechova: FPR = p/(1-p) * (1-VPP)/VPP * (1-FNR)
grupo       VPP da matriz  VPP da identidade          razao
White              0.3491             0.0592            0.2x
Black              0.2896             0.0904            0.3x
leitura: com o FNR e o FPR observados, o escore so seria calibrado por grupo
se o valor preditivo positivo fosse o da coluna do meio. Ele e muito maior: o escore
separa razoavelmente e nao se le como probabilidade de risco.
```

**O que cada linha diz.**

**(1) O navio de Bernoulli.** O limiar é $C/L = 0{,}05294$, ou 5,294%, e a chance de 5% fica **abaixo**
dele. A regra do custo/perda manda não segurar. Em esperança, a perda sem seguro é 1 700 e o
resultado do seguro é $+100$ por mês para a seguradora. O crescimento no tempo inverte a leitura para
o armador. Com um navio, $-0{,}0208$ por mês sem seguro e $-0{,}0182$ com seguro, os dois negativos,
e o seguro **melhora** o resultado apesar de a esperança dizer o contrário. Com **três** navios, o
crescimento sem seguro é $\mathbf{-\infty}$, porque três perdas simultâneas deixam a riqueza sem
cobertura para o prejuízo; a ruína deixa de ser uma probabilidade pequena e vira um estado
absorvente.

**(2) Do outro lado do balcão.** A seguradora melhora quando acumula contratos independentes. Com 1
contrato o crescimento no tempo é $-0{,}000186$ por mês; com 10, $-0{,}001808$; com 100,
$-0{,}013466$; com **1 000 contratos, $+0{,}100410$**. O prêmio não muda em nenhum desses casos;
muda o número de trajetórias independentes que ela detém. As primeiras linhas descem porque a
fórmula usada é a aproximação média menos variância dividida por dois, e com poucos contratos a
variância é grande demais para a aproximação valer; a partir de algumas centenas de contratos a lei
funciona e o crescimento vira positivo. **Quem reparte o risco cobra o mesmo e cresce; quem carrega
a trajetória sozinho afunda.** É a resposta do Capítulo 10, agora com uma companhia de seguros no
lugar de um apostador.
**(3) O EVPI.** Com prevalência de 0,3, tratar custa 1 e deixar um doente sem tratamento custa 10, o
limiar de ação é $p > 0{,}10$ e a melhor ação sem informação é tratar, com ganho $-1{,}00$. Quem sabe
o estado antes de agir tem $-0{,}30$, e a diferença é o **EVPI $= +0{,}70$**. Os dois exames do texto
entregam valores próximos, EVSI de $0{,}2400$ para o sensível $(0{,}90; 0{,}70)$ e $0{,}2450$ para o
específico $(0{,}30; 0{,}95)$, ambos abaixo do teto. Com prevalência 0,05 o EVPI cai para 0,450; com
0,30, sobe a 0,700; com 0,60, cai de novo para 0,400. **O valor da informação é máximo quando a
probabilidade está perto do limiar de ação**, e é aí que a informação muda a decisão.

**(4) Os diques.** Os quatro modelos discordam pouco no nível de **100 anos** (de 1,26 a 7,01 m) e
muito no de **10 000 anos** (de 1,26 a 21,49 m). A cauda é onde os modelos se separam. As cinco
regras dão cinco alturas: valor esperado com o Gumbel, **3,06 m**; média dos modelos, **7,82 m**;
maximin de Wald, **12,58 m**; arrependimento minimax de Savage, **9,81 m**; precaução de 1 em 10 000
no pior modelo, **21,49 m**. O valor esperado de um só modelo escolhe a altura mais barata e a mais
frágil; a precaução escolhe a mais cara; Savage limita o quanto se pode lamentar. O EVPI do conjunto
de modelos, a distância entre a precaução e o valor esperado, vale **18,427 m**, e é a medida de
quanto custa não saber qual é a cauda certa.

**(5) A tabela do trilema.** Os dois grupos são White, com taxa-base 0,069, FNR 0,24 e FPR 0,105, e
Black, com taxa-base 0,111, FNR 0,36 e FPR 0,196. No limiar do sistema, são 467 e 832 falsos
positivos e 79 e 191 falsos negativos. Igualando o FNR **para baixo**, ao nível do White, os falsos
positivos caem de 1 299 para 659 e os falsos negativos de 270 para 206. Igualando **para cima**, ao
nível do Black, os falsos positivos sobem de 1 299 para 1 927 ($+628$) e os falsos negativos de 270
para 309. **A escolha de qual igualdade perseguir não é neutra: cada direção move os dois erros em
direções opostas e em magnitudes diferentes.**

**(6) A identidade fecha e acusa.** O valor preditivo positivo que a matriz de confusão entrega é
0,3491 (White) e 0,2896 (Black). A identidade de Chouldechova exige, dadas a taxa-base, o FNR e o
FPR observados, 0,0592 e 0,0904, de quatro a seis vezes menor. **O escore separa razoavelmente e não
se lê como probabilidade de risco.** É a tradução numérica do que a identidade diz. Com as três
taxas observadas, o preditor não pode ser calibrado por grupo e manter as taxas de erro iguais nos
dois.

**Declaração de divergência.** A edição anterior registra 35,8% de ruína para quem aposta pelo próprio
histórico e 0% com Laplace (Capítulo 10), e uma tabela de diques com alturas de 4,76 m (valor
esperado com Gumbel), 6,20 m (média dos modelos), 7,46 m (Wald), 5,93 m (Savage) e 9,96 m (precaução),
com níveis de 100 anos de 3,11 a 4,26 m e de 10 000 anos de 3,81 a 9,96 m. Este instrumento, com
$\mu = 1{,}2567$ e $\sigma = 0{,}762$ calibrados para o nível de 100 anos do modelo de $\xi = -0{,}1$
chegar a 4,26 m, mede 3,06 m, 7,82 m, 12,58 m, 9,81 m e 21,49 m. As alturas antigas vêm de um
conjunto de modelos calibrado de outro modo, que este laboratório não reproduz; os dois conjuntos
ficam registrados. No trilema, o valor antigo de 1 273 falsos positivos para igualar o FNR é da tabela
de controle do H1; este laboratório mede $+628$ (com $+274$ na direção inversa), porque o cálculo aqui
parte das taxas por grupo em vez das unidades. As divergências estão declaradas e nenhum valor antigo
foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 11 | sem perda medida neste capitulo | o trilema é a subpergunta do R3: cada hora, e para quem
```

**Exercícios.**

1. *(Conta)* No navio de Bernoulli, o seguro custa 1 800 e cobre uma perda de 34 000 com chance de
   5%. Pela regra do custo/perda, o armador deveria se segurar? Como responde o crescimento temporal?
   Explique as duas respostas numa frase cada.
2. *(Conta)* Refaça o EVPI com prevalência de 0,10 (exatamente o limiar). O valor da informação
   perfeita aumenta ou diminui? Como isso orienta o gasto com exames?
3. *(Conceito)* No exemplo do aplicativo de duas rotas, quem erra mais, o previsor ingênuo ou o
   previsor que iguala os tempos? Em que sentido o segundo é "performativamente estável"?
4. *(Conta)* Com as taxas do laboratório (White com 0,069, 0,24, 0,105), verifique a identidade de
   Chouldechova e diga em quanto ela erra. O erro é de arredondamento ou de modelagem?
5. *(Laboratório)* No laboratório, troque a taxa-base do segundo grupo de 0,111 para 0,20 e refaça a
   tabela do trilema. Que igualdade fica mais cara e por quê?
6. *(Conceito)* A Regra 14.2 do livro proíbe 0% e 100% de probabilidade. Que problema do laboratório
   do navio essa proibição evita?

**Resumo.**

- Decide-se pelo limiar $C/L$. Com $C = 1\,800$ e $L = 34\,000$, o limiar é 5,294% e a chance de 5%
  fica abaixo dele. O crescimento no tempo pode dizer o contrário, e os dois números precisam ser
  declarados.
- A informação vale o quanto move a decisão. O EVPI é o teto, e bits não são valor.
- Quando a probabilidade não merece confiança, os critérios de Wald, Savage e a precaução substituem o
  valor esperado; o critério e a lista de modelos são escolhas declaradas, nunca deduções.
- Previsões publicadas mudam o mundo. O Goodhart medido no H3 derruba o viés de 29% para 1,7%, e o
  laço real medido é quatro vezes menor que o declarado.
- Justiça exige escolher entre igualdades incompatíveis. Igualar o FNR para cima custa $+628$ falsos
  positivos, para baixo economiza 640, e quem paga é declarado antes de rodar.

### Quem paga, na pesquisa

A pesquisa fechou os sete pareceres de governança e os três fluxos de prisões. Em 15 de outubro de
2026, `governanca/GRUPOS.md` registra que **nenhum dos sete grupos afetados foi ouvido, em nenhuma das
sete decisões.** O arquivo é a resposta da frente à objeção "o auditor é parte interessada".
Assinatura do responsável não é escuta do grupo afetado. O responsável está do lado de quem decide
usar o artefato; quem paga o erro fica do outro lado. A lista inclui os 5 226 detidos de Toronto em
1997–2002, os 65 276 de 2020–2021 e as 86 113 paradas de trânsito em Rhode Island 2005–2015, todas
marcadas por cor percebida. A auditoria não pôde olhar três coisas: quem autoriza, o painel humano
que nunca foi feito e as decisões que não chegaram a ninguém. O Capítulo 21 volta a esse ponto.

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

## Capítulo 13. A equipe de especialistas, a banca adversarial e a IA sob controle

**Objetivos.** Ao fim deste capítulo você deve saber o ponto cego de cada uma das oito faixas Faixa 1 a Faixa 8,
percorrer o ciclo de seis estágios, aplicar a condição de aprovação da banca adversarial
$\mathrm{Aprovado}(X) = \bigwedge_{i=1}^{8} \left( A_i(X) \ge \mathrm{Severidade}_{\min} \right)$ e
distinguir o Sistema 1 (triagem rápida, probabilidades presas entre 1% e 99%) do Sistema 2
(deliberação sobre o grafo).

---

### Degrau 1 — O mundo real e o paradoxo

**O grupo que se aprova.** Um estudante entrega um trabalho de estatística. O estudante escolheu a
amostra, limpou os dados, rodou o teste, olhou o resultado, mudou o teste, rodou de novo e escreveu
a conclusão. Quando a conclusão finalmente agradou, ele parou. Ninguém trapaceou. Cada passo pareceu
razoável no momento em que foi dado.

Agora reparta o trabalho entre oito colegas, cada um responsável por uma coisa só, e entregue a um
nono colega os **artefatos** no lugar do relatório: o código, o arquivo de dados, o modelo salvo, a
trilha de auditoria, e a tarefa dele é quebrá-los. Em cada 100 trabalhos feitos por uma pessoa só, a
parte mais frágil é a que ninguém olhou de fora. Em 100 trabalhos com uma equipe que ataca, a parte
frágil é a que sobreviveu ao ataque.

O nono colega tem uma regra: **não ler o relatório**. Ele roda o código com uma semente que o autor não
escolheu, injeta um lote corrompido, adultera uma linha da trilha para ver se o verificador percebe. Quem
só lê relatórios concorda com eles.

**As oito faixas e o ponto cego de cada uma.** Cada faixa existe porque há um jeito de errar que ela
vê melhor do que as outras, um ponto cego que as demais não cobrem.

| Faixa | O que faz | O ponto cego que ela cobre | O que ela entrega |
| --- | --- | --- | --- |
| Faixa 1 Código | Transforma hipóteses em processos geradores executáveis | "O código roda" não é "o código é a hipótese" | Semente fixa, pré e pós-condições, testes de propriedades, `cProfile` e `tracemalloc` |
| Faixa 2 Dados | Audita a entrada do Mundo | O que é uma linha, e por que o dado falta | `datacontract.yaml`, quarentena de registros defeituosos, ingestão idempotente, orçamento de bytes |
| Faixa 3 Ciclo de vida | Testa o modelo no tempo | O mundo muda e o modelo não avisa | Corte temporal sem vazamento, sensores de deriva, política de ignorância, post-mortem |
| Faixa 4 Fronteira | Roda a inferência pesada | Uma amostra não é uma resposta | Diagnóstico de cadeias (as várias cadeias têm de concordar entre si), calibração por simulação, predição conforme |
| Faixa 5 Governança | Cuida de legalidade, privacidade e trilha | Dado pessoal é combinação de campos, não tipo de arquivo | Parecer de impacto, privacidade diferencial, responsável humano nomeado, SHA-256 |
| Faixa 6 Justiça | Pergunta quem paga pelo erro | A métrica escolhida decide quem paga | Auditoria de taxas de erro entre grupos, trilema da justiça |
| Faixa 7 Domínio | Traduz problemas de outras áreas | O parâmetro que não vem dos dados vem de alguém | Glossário, elicitação SHELF, termo de escopo |
| Faixa 8 Decisão | Converte o resultado em ação | Um número sem frequência natural não decide nada | Frequências naturais, HOPs, árvore C/L, cláusula de saída |
| Orquestrador | Coordena e comanda a banca | Nenhum passo aprova o próprio resultado | Ficha da crítica de modelos e ataque ao artefato antes da aprovação |

**O ofício de cada faixa, em um caso.** A faixa 2 começa por uma decisão que ninguém vê: **o que é
uma linha**. Uma linha por pessoa, por dia ou por evento muda a média, a variância e a margem, e por
isso o grão é declarado e provado com uma contagem. Esse agente caça o **valor sentinela**, o
ausente gravado como 0, o −1 ou o 9 999 que entra na soma como se fosse dado, e exige ingestão
idempotente. Rodar a carga duas vezes não pode mudar o número.

A faixa 5 lembra que dado pessoal não é um tipo de arquivo: é a **combinação de campos** que decide.
CEP, data de nascimento e sexo, juntos, identificam provavelmente 87% das pessoas nos Estados
Unidos, pelo censo de 1990. A privacidade diferencial limita quanto a presença de uma pessoa pode
mudar qualquer resposta. Em duas bases que diferem no registro de uma única pessoa, o resultado de
qualquer consulta não pode mudar mais do que um fator fixo. Esse fator é um orçamento, gasto a cada
consulta. A medida mais barata continua sendo não coletar.

Cabe à faixa 3 cobrar que todo modelo em operação declare o comportamento em três casos: entrada fora do
domínio, campo ausente e confiança baixa. As opções são recusar, ativar um reserva determinístico
seguro ou devolver incerteza calibrada. Um modelo em uso piora por motivos diferentes, que pedem
respostas diferentes.

| Situação | O que muda | A resposta certa |
| --- | --- | --- |
| Deriva de covariável | Muda a distribuição das entradas, e a relação entre atributo e desfecho continua valendo | Reponderar, com pesos pelo quociente das densidades |
| Deriva de rótulo | Muda a frequência dos desfechos, e a distribuição do atributo dentro de cada desfecho continua valendo | Reajustar as taxas-base (correção de intercepto) |
| Deriva de conceito | Muda a própria relação entre atributo e desfecho | Nenhuma reponderação salva: refazer o modelo |
| População atendida mudou | A métrica cai porque o público mudou | Não é piora do modelo: ler a métrica com a taxa-base |

A correção da deriva de rótulo tem forma fechada. Se a distribuição do atributo dentro de cada
classe não muda, a probabilidade corrigida é a antiga multiplicada pela razão entre a taxa-base nova
e a antiga, depois renormalizada. Com dois desfechos, a correção soma uma constante ao logito. É uma
**correção de intercepto**. A hierarquia de Van Calster e colegas (2016) separa dois níveis que o
terceiro horizonte encontrou. Um deles é a calibração na média, medida pelo intercepto; o outro é a
**calibração fraca**, que exige intercepto 0 e inclinação 1. No terceiro horizonte, os voos de 2023 passaram com
a correção do intercepto e as prisões de Toronto não, porque o erro delas era de inclinação.

A faixa 7 não pede ao especialista uma média. O protocolo de Sheffield (SHELF) pede a **mediana**,
os **quartis** e os **extremos plausíveis**, ajusta uma distribuição a esses quantis e a devolve
para que ele a confirme, controlando a âncora do entrevistador, a ordem dos cenários e o excesso de
confiança. Na pesquisa, nenhuma elicitação foi feita com especialista humano. A auditoria conseguiu
medir a **sensibilidade à escolha da priori**. Em 33 domínios, a priori do pacote foi trocada pela
de literatura e por uma deliberadamente fraca, e cinco domínios mudam de status ou de ação com a de
literatura e dois com a fraca. A conclusão não é que as prioris simuladas estivessem erradas. É que
em cinco de 33 domínios a decisão não sobrevive à troca da priori.

**Uma lacuna honesta.** Falta à arquitetura uma pergunta que a faixa 2 deveria fazer. **Por que o
dado está ausente?** O contrato de dados limita a nulidade, mas não classifica o mecanismo da
ausência (MCAR, MAR ou MNAR, Capítulo 6), e é essa classe que decide se o descarte dos casos
incompletos é inofensivo. A auditoria depois acrescentou um bloco `ausencia` ao contrato, e nos três
contratos do horizonte 1 real e nas 18 tabelas do segundo horizonte a regra é **vacuosa hoje**, pois
nenhuma coluna tem um único valor ausente. A trava passa a morder no dia em que aparecer.

**Sistema 1 e Sistema 2.** O computador que ajuda a pensar tem dois modos. O **Sistema 1** faz
triagem rápida e categórica e fecha a ficha com uma árvore de sufixos determinística (mesma entrada,
mesma resposta), emitindo probabilidades **presas ao intervalo de 1% a 99%**. O **Sistema 2**
delibera: busca em árvore sobre o hipergrafo, síntese de código, inferência pesada, estresse
adversarial. A regra do intervalo não é estética. Uma probabilidade de 0% ou de 100% não pode ser
pontuada por uma regra própria, nunca perde e nunca aprende.

---

### Degrau 2 — A regra do jogo

**O ciclo de seis estágios.** Uma pergunta percorre seis estágios, cada um com fichas de saída. O caminho
até a decisão passa por todos.

1. **Tradução e elicitação (Faixa 7).** Ficha da transferência, elicitação SHELF, termo de escopo.
2. **Ingestão, grão, causalidade e ética (Faixa 2, Faixa 5, Faixa 6).** Contrato de dados, ficha da amostra, grafo
   causal com o degrau de Pearl, parecer de privacidade, auditoria de justiça.
3. **Simulação, modelo nulo e fronteira (Faixa 1, Faixa 4, Faixa 3).** Processo gerador executável, modelo nulo
   simulado **antes** de ler qualquer padrão, diagnósticos de inferência, partição fora do tempo, contagem
   de tentativas.
4. **Projeção nas cinco lentes (o orquestrador).** Varredura de Fluxonomia, Crononomia, Escalonomia,
   Morfonomia e Crisonomia.
5. **Banca adversarial (o orquestrador).** Reprovado volta ao estágio que falhou; aprovado segue.
6. **Decisão e fechamento (Faixa 8).** Memorando em frequências naturais, HOPs, árvore C/L e cláusula de saída.

O único caminho até a decisão passa pela banca. Um trabalho reprovado volta ao **estágio em que
falhou**, em vez de voltar ao começo.

**A banca adversarial.** Três falhas eliminam o trabalho antes de qualquer discussão de mérito: vazamento
temporal, contrato de dados violado sem tratamento e ausência de um responsável humano nomeado. Depois, cada
faixa recebe o seu ataque, oito no total.

| Faixa atacada ($A_i$) | O ataque | O cuidado que a ficha precisa ter |
| --- | --- | --- |
| $A_1$ Código | Uma bateria nova de propriedades, com semente que o autor não escolheu | teste de propriedade |
| $A_2$ Dados | Um lote com registros corrompidos, que precisa ser isolado | tratamento de ausentes |
| $A_3$ Ciclo de vida | Um lote com deriva forte, ao vivo | deriva declarada |
| $A_4$ Inferência | Um conjunto de dados patológico, para ver se a incerteza denuncia a inadequação | patologias cobertas |
| $A_5$ Governança | Adulteração da trilha de auditoria, que o verificador da cadeia tem de detectar | trilha íntegra |
| $A_6$ Justiça | Inverter a métrica escolhida e mostrar quem passaria a pagar | subgrupos auditados |
| $A_7$ Domínio | Um cenário atípico do domínio, com priori alternativa | priori elicitada |
| $A_8$ Decisão | Uma crise que viola a própria cláusula de saída | cláusula de saída declarada |

Os oito ataques aparecem em registros antigos como **MB1–MB8**. O nome novo é $A_1$–$A_8$, e a lista é uma só.

**Calibração por simulação (faixa 4).** $\mathrm{rank}(\tilde\theta; \theta^{(1)}, \dots, \theta^{(L)}) \sim \mathrm{Uniforme}\{0, \dots, L\}$. Se o posterior estiver calibrado, o posto do valor verdadeiro entre as $L$ réplicas é uniforme. É assim que a faixa 4 verifica se a inferência pesada devolve incerteza do tamanho certo, em vez de apenas um número.

**A condição de aprovação.** Cada agente $A_i$ devolve uma **severidade** na escala de 0 a 10, e o artefato
$X$ só é aprovado se **todos** os oito atingirem o piso. A condição é esta:

$$\mathrm{Aprovado}(X) = \bigwedge_{i=1}^{8} \Big( A_i(X) \ge \mathrm{Severidade}_{\min} \Big),\qquad
\mathrm{Severidade}_{\min} = 6.$$

O $\bigwedge$ é o "e" lógico. Basta um agente abaixo do piso para o trabalho voltar. A severidade é
determinística. Ela sai do cuidado correspondente na ficha e de uma semente explícita, e repetir a
ficha com a semente declarada devolve sempre o mesmo número. A condição **falha** quando um cuidado
foi declarado como ausente, e é isso que a torna auditável. Não há juízo subjetivo escondido na
nota.

**A ficha da crítica de modelos.** O orquestrador só aprova depois de preencher cinco itens: checagem
preditiva contra réplicas do modelo ajustado; comparação com pelo menos um modelo rival; separação entre
parâmetros identificados pelos dados e trazidos de fora; análise de performatividade, Goodhart e Lucas; e
defesa das premissas indecidíveis pelo **custo do erro**.

**Um trabalho com lacuna declarada vale mais que um trabalho uniforme.** O artefato completo com uma
faixa fraca admitida supera o artefato uniforme que não admite lacuna nenhuma. No horizonte 1 real,
foi a banca que reprovou o projeto dos incêndios dinamarqueses. A alegação perdeu, mas o modelo
rival **não ganhou o direito de decidir**.

---

### Degrau 3 — Mão na massa

Duas medidas: rodar a banca completa sobre uma ficha com os oito cuidados e sobre uma ficha com cuidados
removidos, imprimindo as oito severidades; e conferir que a triagem do Sistema 1 nunca sai do intervalo
entre 1% e 99%. Código em `crates/stochology-book/src/cap13.rs`; saída medida com `cargo run --release`
no crate de rascunho `/tmp/labs-cap13`:

```text
== Capitulo 13 :: a banca adversarial A1-A8 ==
ficha completa (ficha h3.voos): A1-codigo=9 A2-dados=8 A3-ciclo-de-vida=10 A4-inferencia=9 A5-governanca=8 A6-justica=9 A7-dominio=8 A8-decisao=10 | menor=8 piso=6 aprovado=true
ficha com A5 e A6 removidos (ficha h3.voos-com-defeito): A1-codigo=9 A2-dados=8 A3-ciclo-de-vida=10 A4-inferencia=9 A5-governanca=2 A6-justica=3 A7-dominio=8 A8-decisao=10 | menor=2 piso=6 aprovado=false
sem A1-codigo: reprovam=["A1-codigo"] qtd_reprovando=1 menor=3
sem A2-dados: reprovam=["A2-dados"] qtd_reprovando=1 menor=2
sem A3-ciclo-de-vida: reprovam=["A3-ciclo-de-vida"] qtd_reprovando=1 menor=4
sem A4-inferencia: reprovam=["A4-inferencia"] qtd_reprovando=1 menor=3
sem A5-governanca: reprovam=["A5-governanca"] qtd_reprovando=1 menor=2
sem A6-justica: reprovam=["A6-justica"] qtd_reprovando=1 menor=3
sem A7-dominio: reprovam=["A7-dominio"] qtd_reprovando=1 menor=2
sem A8-decisao: reprovam=["A8-decisao"] qtd_reprovando=1 menor=4
varredura: fichas reprovadas=8 de 8 | menor severidade global=2 | severidades < piso por agente=[1, 1, 1, 1, 1, 1, 1, 1]
ficha completa com semente 7: menor=8 aprovado=true
ficha completa com semente 2026: menor=8 aprovado=true
ficha completa com semente 999983: menor=8 aprovado=true
sistema 1: rotulo=ficha-em-ordem produtos=[0.06, 0.94] soma=1.0000 fora_de[0.01;0.99]=0
```

O laboratório acima usa uma triagem de duas probabilidades, suficiente para mostrar a trava. O **Sistema 1 de
verdade** está implementado em `crates/stochology-core/src/s1.rs`, com vocabulário fechado de seis categorias e
35 temas, árvore de sufixos canônica de 136 entradas e soma conferida a cada rodada. A saída medida com
`cargo run --release --bin labs -- --s1`:

```text
== Sistema 1 :: triagem determinística de fichas ==
arvore de sufixos: 136 sufixos indexados em 6 temas declarados
categoria=fila probabilidade=0.8988 produtos=[0.8988, 0.0202, 0.0202, 0.0202, 0.0202, 0.0202] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.8988 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=causal probabilidade=0.9361 produtos=[0.0128, 0.9361, 0.0128, 0.0128, 0.0128, 0.0128] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9361 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=cauda probabilidade=0.9117 produtos=[0.0177, 0.0177, 0.9117, 0.0177, 0.0177, 0.0177] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9117 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=calibracao probabilidade=0.9361 produtos=[0.0128, 0.0128, 0.0128, 0.9361, 0.0128, 0.0128] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9361 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=justica probabilidade=0.9117 produtos=[0.0177, 0.0177, 0.0177, 0.0177, 0.9117, 0.0177] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9117 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=outros probabilidade=0.8718 produtos=[0.0256, 0.0256, 0.0256, 0.0256, 0.0256, 0.8718] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.8718 | soma 1.0000 | fora de [0.01;0.99] 0
```

Seis linhas, e o contrato se repete em todas. A soma é 1,0000 e nenhuma probabilidade cai fora de
[0,01; 0,99]. A entrada desconhecida (`xyzzy plugh abracadabra`) cai em `outros` com 0,8718. Esse
valor é a priori declarada, e não um palpite. Os limites ficam registrados. O vocabulário é fechado,
o casamento é lexical e não entende negação, e isto é triagem, e não julgamento. Julgar é do Sistema
2.

Quatro leituras.

**A ficha completa passa, e passa com folga.** As oito severidades ficam entre 8 e 10, e a menor é
8, dois pontos acima do piso 6. O portão do repositório, no horizonte `todos` com 12 fichas, também
publica pior = 8, e mudar a semente do laboratório (7, 2026, 999 983) não altera o veredito. A
reprovação vem de um cuidado ausente, e não de sorte de semente.

**Cada cuidado removido derruba exatamente o agente que o cobra.** Nas oito linhas da varredura, o
conjunto de reprovados tem sempre **um** elemento, e é sempre o agente correspondente. Sem trilha
íntegra reprova o $A_5$ (severidade 2), sem cláusula de saída reprova o $A_8$ (4), sem teste de
propriedade reprova o $A_1$ (3). Os outros sete ficam intactos. A banca não é uma nota média que
dilui, mas um "e" lógico que localiza a falha.

**Remover dois cuidados reprova a ficha inteira.** Com A5 e A6 removidos, as severidades caem para 2
e 3 e a ficha é reprovada, com 8 reprovações em 8 fichas defeituosas e a menor severidade global em
2. Compare com o portão de verdade. O comando `portoes --horizonte h3 --defeito h3.voos23` devolve
`G6 reprovado banca A1-A8 com piso 6 e pior 2`. O laboratório reproduz o pior = 2 removendo o
cuidado de governança e o de justiça.

**O Sistema 1 respeita o intervalo.** A triagem devolve 0,06 e 0,94, soma 1,0000, e zero valores
fora de entre 1% e 99%. O intervalo existe por um motivo prático. Uma probabilidade de 0% não seria
pontuada por regra própria nem seria corrigida pelo placar. O limite separa um previsor de uma
afirmação dogmática.

**Declaração de divergência.** A edição anterior deste livro fala em "oito faixas e um orquestrador M15". O
texto do repositório (`PLAN.md`) chama o orquestrador de o orquestrador e a implementação
(`OrquestradorM`) também; a sigla "M15" sobrevive no `AGENTS.md` e em trechos antigos. São o
mesmo objeto, e este livro usa o orquestrador por ser o nome do código. O laboratório desta edição mede as
severidades a partir dos **oitos cuidados da ficha** (bandeiras verdadeiro/falso) e não dos ataques em si:
rodar o ataque é a maquinaria do repositório, e o número que a banca publica é o da severidade. Por isso os
valores 8, 9 e 10 aqui reproduzem o pior = 8 do portão, e não um relatório de ataque executado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 13 | sem perda medida neste capitulo | a banca adversarial julga a ficha, e não o grão dela
```

**Exercícios.**

1. *(Conceito)* Uma base tem uma linha por consulta médica, e a pergunta é sobre pacientes. Que faixa deve
   barrar a análise, e com que teste?
2. *(Conceito)* Desenhe o ataque da banca para um modelo de triagem de crédito. Que crise violaria a
   cláusula de saída? Que grupo passaria a pagar se a métrica fosse invertida?
3. *(Conta)* A ficha completa tem menor severidade 8 e piso 6. Qual é o número máximo de agentes que podem
   cair para a faixa baixa (2 a 4) sem reprovar a ficha? Justifique com a definição do $\bigwedge$.
4. *(Laboratório)* Implemente a calibração por simulação (SBC) para um modelo Beta-Binomial: sorteie o
   parâmetro da priori, simule os dados com ele e rode o amostrador. Verifique que o posto do valor
   verdadeiro entre as amostras da posterior é uniforme. Depois introduza um erro no amostrador e veja o
   histograma deixar de ser plano.
5. *(Conceito)* Dê um exemplo de cada um dos três modos de piora de um modelo em uso: deriva de
   covariável, deriva de rótulo e deriva de conceito; dê também um exemplo da quarta situação, que
   não é piora. Diga a resposta certa para cada uma.
6. *(Conceito)* Por que a resposta do Sistema 1 precisa ser determinística (entrada repetida,
   probabilidade igual)? Relacione com a semente fixa do Capítulo 1.

**Resumo.**

- Oito faixas, cada uma ligada a um modo de errar, mais um orquestrador que ataca; os agentes trocam
  fichas estruturadas, nunca texto livre, e ninguém aprova o trabalho que produziu.
- O ciclo tem seis estágios, e o laço de volta leva ao estágio que falhou, em vez do começo.
- A aprovação é um "e" lógico das oito severidades contra o piso 6; cada cuidado ausente derruba o
  agente que o cobra.
- Sistema 1 triagem determinística com probabilidades entre 1% e 99%; Sistema 2 delibera sobre o
  hipergrafo.
- Um artefato com faixa fraca declarada vale mais que um artefato uniforme sem lacuna admitida.

## Capítulo 14. O contrato de ferro: fichas, proibições, invariantes e portões

**Objetivos.** Ao fim deste capítulo você deve saber ler e escrever uma ficha epistemológica, preencher as
cinco perguntas do cabeçalho, enunciar as quatro proibições e os cinco invariantes $\Phi_1$–$\Phi_5$,
percorrer o funil de quatro filtros e dizer qual verificação cada portão automático faz.

---

### Degrau 1 — O mundo real e o paradoxo

**O formulário que recusa.** Todo mundo já preencheu um formulário que aceita qualquer coisa. Você escreve
"sim" no campo de observações, o sistema aceita, e três semanas depois ninguém sabe quem autorizou o quê.
O oposto existe e é chato, e é disso que este capítulo trata. O formulário aqui **recusa** o envio quando um
campo obrigatório está vazio.

A ficha epistemológica é uma lista fechada de perguntas. Ela é o **contrato** do trabalho, e não o resumo
dele. Em cada 100 fichas preenchidas com pressa, as que passam sem reclamação são as que tiveram todos
os campos respondidos com algo conferível. Uma ficha sem o campo do processo gerador não diz "não sei". Diz
"ninguém escreveu isso", e é por aí que o erro entra.

**A pergunta que ninguém faz ao decisor.** Há um jeito específico de errar que tem nome e quase nunca é
checado, o **erro de tipo III**. Nos testes comuns, o erro de tipo I é acusar quem é inocente e o erro de
tipo II é absolver quem é culpado. O erro de tipo III é mais banal e mais frequente, e consiste em **responder
brilhantemente a pergunta errada**. O estudo mede o efeito A com precisão impecável, e o gestor precisava
saber sobre o efeito B.

Imagine uma prefeitura que precisa decidir se abre uma segunda sala de atendimento nos dias cheios. O
relatório responde, com rigor, à pergunta *"qual é o nível de serviço médio do ano?"* A resposta é 82%, e o
gestor lê isso como resposta à pergunta dele, *"devo abrir uma segunda sala?"*. Ninguém escreveu a pergunta
do gestor em lugar nenhum. Em 100 decisões apressadas, uma parte respeitável morre por conta certa na
pergunta trocada, e não por conta errada.

**Como o contrato trava isso.** O pacote de domínio passa a carregar um campo obrigatório, o
`pergunta_do_decisor`, com três partes na ordem: **quem decide**, **qual ação a decisão muda**
(copiada literalmente do campo de decisão do pacote) e **a pergunta que essa decisão usa**,
terminada em interrogação. Quem valida é o código, que recusa texto genérico sem quem decide e barra uma ação
diferente da do pacote. Depois disso, "a pergunta que o modelo responde é a que o decisor fez"
deixou de ser suposição e passou a ser uma frase conferível.

**As quatro coisas que o contrato proíbe.** O contrato dos agentes começa por quatro proibições absolutas, e
cada uma tapa um buraco conhecido:

1. **Linguagem natural livre entre agentes.** A comunicação é só por fichas estruturadas; texto livre é
   ambíguo e inventa.
2. **A média solitária.** Quando a função é não linear, a dinâmica multiplicativa ou a cauda pesada, é
   proibido reportar só a média: reportam-se o resultado do caso médio, a média simulada dos
   resultados, a mediana, os quantis e a curva acumulada.
3. **Confundir aprendizado com performatividade.** Erro de previsão pequeno não prova que o modelo é
   verdadeiro; roda-se o modelo com e sem a ação.
4. **Extremos dogmáticos e omissão de tentativas.** Nenhuma previsão declara 0% ou 100%; toda previsão é
   pontuada por regra própria fora do ajuste; e declara-se quantas especificações foram tentadas.

---

### Degrau 2 — A regra do jogo

**A ficha e as cinco perguntas do cabeçalho.** Todo payload que circula abre com cinco perguntas
obrigatórias.

| Campo | Pergunta |
| --- | --- |
| `processo_gerador` | Qual mecanismo oculto gera os dados, escrito de modo simulável, e em que camada a alegação pode falhar? |
| `nao_linearidade` | O resultado é função não linear de algo que varia? Compare $f(\mathbb{E}[X])$ com $\mathbb{E}[f(X)]$; a dinâmica é aditiva ou multiplicativa? |
| `unidade_e_margem` | Qual é a unidade de fato independente, e a margem foi calculada com ela? |
| `fisica_do_ruido` | Que ruído é esse (gaussiano ou com saltos)? Há estados absorventes? O agente é passivo ou deforma o mundo? |
| `escada_causal` | Em que degrau de Pearl está a afirmação, com que estratégia de identificação e que premissa não testável? |

O cabeçalho real do $H_0$ em domínio real, abreviado:

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

Cada módulo do curso acrescenta perguntas às cinco do cabeçalho. A ficha da evidência pede a taxa-base, as
duas verossimilhanças e a posterior calculada de dois modos, em chances e numa árvore de 10 000 casos, que
precisam coincidir. A do grafo causal começa pelo degrau. A da crítica de modelos fecha a série e diz em que
item cada ficha anterior entra.

**A física do ruído, em detalhe.** O quarto campo pede três classificações do ruído.

| Pergunta | Opções | Por que importa |
| --- | --- | --- |
| Que forma tem o ruído? | Gaussiano (contínuo) ou com saltos | Um modelo gaussiano não tem o 19 de outubro de 1987 no seu espaço de hipóteses |
| Como o ruído se integra? | Itô ou Stratonovich | Quando o ruído depende do estado, as duas leituras dão derivas diferentes |
| Há estados absorventes? | Sim (ruína, morte, falência) ou não | Num estado absorvente, a média de conjunto deixa de descrever qualquer trajetória |

A segunda pergunta parece técnica e não é. Num crescimento multiplicativo com ruído proporcional,
$dX = \mu X\,dt + \sigma X\,dW$, lido à maneira de Itô, a esperança cresce à taxa $\mu$, mas a
trajetória típica cresce à taxa

$$\frac{d}{dt}\,\mathbb{E}[\ln X_t] = \mu - \tfrac{1}{2}\sigma^2.$$

É o arrasto da volatilidade do Dow Jones alavancado, e é Jensen outra vez. Lido à maneira de Stratonovich, o
mesmo símbolo descreve outro processo, em que o termo $-\sigma^2/2$ não aparece. A escolha depende da física.
Ruído branco idealizado de um sistema discreto pede Itô; o limite de um ruído real com correlação curta pede
Stratonovich. A ficha pede também um teste com passo de tempo menor. Se o resultado muda quando $\Delta t$
encolhe, a simulação estava medindo o integrador em vez do Mundo. Esse teste **não** decide entre Itô e
Stratonovich, porque o esquema de Euler converge para a leitura de Itô, e a de Stratonovich pede outro integrador.

O ruído também faz saltar entre estados estáveis. Num sistema com dois poços, o tempo médio até o salto
segue a fórmula de Kramers e cresce exponencialmente com a altura da barreira $\Delta U$ em relação à
temperatura $T$:

$$\tau \approx \frac{2\pi}{\sqrt{U''(x_{\min})\,\lvert U''(x_{\max})\rvert}}\; e^{\Delta U/T}.$$

Para barreiras altas, esse tempo quase não tem memória. Quem esperou muito não está mais perto do salto.
Olhado só nos saltos, porém, o ruído parece um empurrão. Em 591 saltos simulados no curso, o ruído logo
antes do salto apontou no sentido dele em 99% dos casos, sem nenhuma força externa. Procurar a causa de um
salto é erro de seleção; procurar a causa de uma mudança na frequência dos saltos não é.

**Os cinco invariantes.** Nada entra no grafo sem passar pelos invariantes, que funcionam como travas de banco
de dados. Os quatro primeiros são exigidos desde o começo; o quinto entrou com o eixo da resolução da pergunta
(Capítulo 21) e morde no caso em que o desalinhamento é mais caro.

| Invariante | O que exige | O que barra |
| --- | --- | --- |
| $\Phi_1$ Simulabilidade | Toda alegação vira código com semente declarada e testes de propriedades | Argumento só em prosa |
| $\Phi_2$ Margem e unidade | Todo número vem com margem calculada na unidade de fato independente | Pseudorreplicação, duplicatas, valores sentinela |
| $\Phi_3$ Camadas e teste severo | Auditar o filtro da amostra, desenhar o grafo causal e confrontar a hipótese com um modelo nulo simulado | Padrão lido antes do nulo, colisor, Simpson |
| $\Phi_4$ Escopo ecológico | Julgar a decisão no ambiente em que vai operar, com termo de escopo e cláusula objetiva de saída | Recomendação sem prazo de validade |
| $\Phi_5$ Resolução da pergunta | A ação declarada tem de estar no mesmo grão da unidade independente: com o dado no dia ou mais grosso, resolução agregada (R0) reprova | Ação por dia com conta no agregado. Unidade mais fina que o dia fica fora desta trava, e isso está declarado |

**O funil de quatro filtros.** Um sistema que simula toda hipótese que encontra colapsa por custo e por
inflação de falsas descobertas. O funil, a poda por valor da informação e o contador de tentativas vivem em
Rust, em `crates/stochology-core/src/funil.rs`, com 17 testes. Os quatro filtros recusam pelo cabeçalho (sem
pergunta, sem processo gerador, sem unidade ou nulo, sem ação com saída). A poda corta quando a cota do valor
da informação não pode cruzar o limiar: com $C/L = 0{,}1$ a cota vale 0,0090 em $p = 0{,}001$ e o exame de
0,05 é podado, enquanto no limiar $p = 0{,}10$ a cota é 0,9000. Por fim, o contador fecha a conta na forma
`K = 3 tentativas (F1 1, F2 0, F3 0, F4 1); aprovadas 1; versao final: v3`. O limite declarado é que a cota
é o valor esperado da informação perfeita, exata no modelo de dois atos e dois estados. Cada frente se
organiza como um funil, e nele a maior parte das hipóteses fracas cai antes de tocar em simulação pesada.

1. **Valor da informação.** Corta-se a hipótese que não mudaria nenhuma decisão nem com resposta perfeita. O log
   padrão é "podado por Crisonomia: curiosidade sem uso operacional".
2. **Poda topológica e unidade.** Corta-se, por d-separação, todo nó que fica fora do cobertor de Markov da
   variável de interesse, inclusive os colisores que, se condicionados, criariam o viés de Berkson. Corta-se
   também a alegação que os dados não identificam, ou cujo tamanho efetivo, contado na unidade independente
   pelo $F_2$, fica pequeno demais para concluir. O contrato de dados confere ainda um bloco `ausencia` para o
   mecanismo da falta: MCAR, MAR, MNAR ou não declarado, com as colunas que a classificação cobre e a análise
   de sensibilidade exigida quando a ausência pode depender do valor da variável. Ausência sem classificação
   é recusada; nos três contratos do $H_1$ real e nas 18 tabelas do $H_{2g}$ a regra é vacuosa hoje, e passa
   a morder no dia em que aparecer.
3. **Teste severo contra o nulo.** Se o padrão alegado já aparece no modelo nulo, ou numa referência de um
   parâmetro só como a taxa-base, a pesquisa encerra ali. Quando a simulação é cara, o $F_4$ faz esse teste
   primeiro num emulador barato.
4. **Inferência pesada, lentes e banca.** Só o restante recebe o tratamento caro.

**Caso trabalhado: a identificabilidade de $R_0$.** No início de uma epidemia, os casos crescem como
$\exp((\beta - \gamma)t)$, e os dados determinam a **diferença** $\beta - \gamma$, não a razão
$R_0 = \beta/\gamma$. Com 25 dias de casos simulados, em que a verdade é $R_0 = 2$, os valores compatíveis
vão de **1,38 a 5,90**, e o tamanho final previsto vai de **50% a 100%** da população. Com 45 dias, $R_0$
fica entre **2,00 e 2,24**, mas aí a curva já dobrou. Tomar a duração da infecção de outros estudos estreita
$R_0$ para **1,81 a 2,11**. Isso não é defeito, desde que se declare que a margem passou a ser a da fonte.

**Os portões automáticos.** Nenhuma entrega é consolidada sem passar nos portões numa máquina limpa. Os
agentes e o $\Phi$ já foram formalizados; os portões **repetem o trabalho e comparam**. Entrada repetida tem
de produzir saída idêntica, e um defeito plantado tem de ser detectado. O binário
`portoes` implementa sete.

| Portão | Faixa | O que verifica |
| --- | --- | --- |
| G1 | $\mathcal{M}$ | Determinismo: a raiz de Merkle da segunda execução é idêntica à da primeira |
| G2 | $F_1$ | Semente fixa declarada em toda ficha |
| G3 | $F_1$ | Escopo declarado: só artefato simulável |
| G4 | $F_1$/$F_7$ | Invariantes $\Phi_1$–$\Phi_4$ e a trava de tipo III |
| G5 | $F_5$ | Trilha SHA-256 íntegra **e** adulteração detectada |
| G6 | $\mathcal{M}$ | Banca adversarial $A_1$–$A_8$ com o piso |
| G7 | $F_8$ | Cláusula de saída declarada e $A_8$ acima do piso |

Dois detalhes fazem o conjunto valer mais que a soma. O G5 não verifica só que a corrente está íntegra. Ele
**adultera de propósito** e exige que a verificação falhe com a corrente adulterada. O G1, por sua vez,
compara duas execuções do horizonte declarado. Um portão que nunca falha não é portão (Capítulo 15).

---

### Degrau 3 — Mão na massa

O laboratório valida uma ficha completa e uma ficha com um defeito plantado por vez, imprimindo as falhas por
portão, no espírito de `portoes --defeito`. Código em `crates/stochology-book/src/cap14.rs`; saída medida com
`cargo run --release` no crate de rascunho `/tmp/labs-cap14`:

```text
== Capitulo 14 :: o contrato de ferro ==
ficha h0.posto-saude: camada=L1 incerteza=aleatoria cabecalho=5 campos | falhas=[] total=0
taxonomia declarada: camadas=["L1", "L2", "L3"] incertezas=["aleatoria", "epistemica", "knightiana", "nao-se-aplica"]
defeito SemSemente: falhas=1 portoes=Phi1-simulabilidade detalhe="falta semente fixa ou artefato simulavel"
defeito SemDefinicao: falhas=1 portoes=Phi3-camadas-e-modelo-nulo detalhe="sem modelo nulo ou sem definicao em uma linha"
defeito SemMargemNemUnidade: falhas=1 portoes=Phi2-unidade-independente detalhe="margem sem unidade independente declarada"
defeito SemModeloNulo: falhas=1 portoes=Phi3-camadas-e-modelo-nulo detalhe="sem modelo nulo ou sem definicao em uma linha"
defeito SemEscopoSemQuemPagaSemSaida: falhas=1 portoes=Phi4-escopo-e-saida detalhe="escopo ecologico, quem paga pelo erro ou clausula de saida ausente"
defeito PerguntaSemInterrogacao: falhas=1 portoes=tipo-III-pergunta-do-decisor detalhe="pergunta_do_decisor precisa terminar em ? e citar a acao do pacote"
defeito PerguntaSemAcao: falhas=1 portoes=tipo-III-pergunta-do-decisor detalhe="pergunta_do_decisor precisa terminar em ? e citar a acao do pacote"
grafo integro: validacao=[]
grafo com aresta fantasma: validacao=["aresta para no ausente: L2_erlang -> L2_fantasma"]
grafo com ciclo dedutivo: validacao=["ciclo em E_tree: Some(\"L1_fila\")"]
grafo com hiperaresta de um membro: validacao=["hiperaresta com menos de dois membros: H2"]
teste de propriedades (10.000 fichas simuladas): violacoes=0 margem_min=0.00617 margem_max=0.09798
mesma semente, dois validadores: execucoes divergentes em 0 de 32
```

Quatro leituras.

**A ficha completa passa sem uma única falha.** Cinco campos de cabeçalho na ordem exigida, semente
declarada, artefato simulável, margem com unidade independente, modelo nulo, escopo ecológico, quem paga
pelo erro, cláusula de saída e a pergunta do decisor citando a ação. O total é zero. A trava
de tipo III é **literal**, e a pergunta precisa terminar em interrogação **e** conter a ação do pacote.

**Cada defeito cai no portão certo.** Sete defeitos plantados, sete reprovações, uma por portão, sem
contaminação. Sem semente cai no $\Phi_1$; sem margem ou unidade cai no $\Phi_2$; sem definição ou sem
modelo nulo cai no $\Phi_3$; sem escopo, sem quem paga ou sem cláusula de saída cai no $\Phi_4$. Os dois
defeitos da pergunta do decisor caem os dois na trava de tipo III, mas por motivos diferentes. Um perde o
ponto de interrogação, o outro perde a ação. **A mensagem é idêntica nos dois casos.** O validador
enuncia a exigência da regra, sem dizer qual parte faltou. É um detalhe pequeno que o exercício 4 pede
para melhorar.

**O validador de estrutura atravessa os capítulos.** Reaparecem aqui as três recusas do Capítulo 12:
aresta para nó fantasma, ciclo dedutivo e hiperaresta com um membro só. Isso é de propósito. Uma trava que só
existe num capítulo é uma trava que ninguém usa.

**O teste de propriedades e a semente.** Dez mil fichas simuladas com margem $1{,}96\sqrt{p(1-p)/n}$ para
$n$ entre 100 e 999 e $p$ entre 0,01 e 0,99 dão **zero violações**, com margens entre 0,00617 e 0,09798. O
intervalo de confiança nunca é negativo nem maior que meio. Os dois validadores rodados sobre a semente
declarada divergem em **0 de 32** execuções, a reprodutibilidade que o Capítulo 1 exige.

**A esteira por horizonte.** No repositório cada horizonte tem a sua esteira. O $H_0$ aplica $\Phi_1$–$\Phi_4$, as rodadas $H_{2b}$ a $H_{2g}$ aplicam cinco critérios e o $H_3$ aplica sete. Cada `portoes.py` repete a execução, compara os resumos criptográficos e sai com 0 ou 1; os sete deste capítulo
são os do binário `portoes`. (O `portoes.py` de cada frente foi aposentado em 2026-10-16 e vive no commit
`1bfec905`; o portão continua no binário.)

**Declaração de divergência.** Duas contas deste capítulo divergem entre as fontes, e ficam as duas
registradas. (1) O livro anterior descreve **cinco** portões automáticos — contratos e sementes; esquema
declarativo e idempotência; convergência das cadeias; corte temporal; governança, justiça e saída —, enquanto
o binário `portoes` implementa **sete** (G1 a G7). A diferença é de agrupamento: o portão 3 do livro
antigo ("convergência das cadeias") some do binário, e o portão 5 se desdobra em G5, G6 e G7. Os dois textos
contam a mesma esteira de formas diferentes; nenhuma lista foi apagada. (2) O livro anterior e o plano do
repositório dizem que "no $H_0$ a esteira aplica $\Phi_1$–$\Phi_4$", e este laboratório mede exatamente
isso: uma ficha completa gera zero falhas de invariante, e cada defeito gera uma. O número "falhas = 0" do
portão G4, no horizonte `todos`, reproduz aqui.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 14 | sem perda medida neste capitulo | o contrato carrega o campo resolucao (R0 a R3) e o Φ5; a trava só morde com unidade igual ou mais grossa que o dia
```

**Exercícios.**

1. *(Conceito)* Preencha as cinco perguntas do cabeçalho para a alegação "alunos que estudam com música
   tiram notas maiores".
2. *(Conceito)* Que filtro do funil deve cortar a pergunta "qual é a cor preferida dos clientes que compram
   seguro?" numa seguradora que não vai mudar o produto?
3. *(Conta)* Por que 25 dias de crescimento exponencial identificam $\beta - \gamma$ e não $\beta/\gamma$?
   Mostre que dois pares $(\beta, \gamma)$ com diferenças iguais geram curvas iniciais idênticas.
4. *(Laboratório)* Melhore a mensagem da trava de tipo III para distinguir "faltou o ponto de interrogação"
   de "a pergunta não cita a ação". Faça os dois defeitos caírem em mensagens diferentes e reproduza a
   tabela.
5. *(Laboratório)* Escreva um teste de propriedades que verifique que a espera simulada de uma fila é sempre
   não negativa e não diminui quando o tempo de atendimento aumenta, mantidas as chegadas.
6. *(Conceito)* Explique, em três frases, por que o G5 é mais forte que um teste que só confere se a corrente
   está íntegra, já que ele adultera de propósito a corrente que produziu.

**Resumo.**

- A ficha é uma lista fechada de perguntas; cinco são obrigatórias em todo payload, e o campo
  `pergunta_do_decisor` trava o erro de tipo III.
- As quatro proibições e os cinco invariantes são travas, e não recomendações; o $\Phi_5$ é o mais novo e o mais estreito.
- O funil corta barato antes de gastar caro: valor da informação, poda topológica e de unidade, teste severo
  contra o nulo, e só então inferência pesada e banca.
- O binário `portoes` implementa sete portões, com código de saída 0 ou 1, e um portão que nunca
  falha não é portão.

## Capítulo 15. O cofre, a senha criptográfica e a auditoria contra si mesmo

**Objetivos.** Ao fim deste capítulo você deve saber explicar por que o pré-registro com hash SHA-256 é a
única senha contra o ajuste depois de ver o resultado, o que o contador de tentativas $K$ mede, por que o
cofre fora do tempo se abre uma única vez, como funcionam o diário de previsões e os pareceres assinados, e
por que o auditor ser parte interessada é um limite que nenhuma assinatura resolve.

---

### Degrau 1 — O mundo real e o paradoxo

**Pintar o alvo depois do tiro.** Um atirador dispara dez tiros contra a parede de um celeiro. Depois olha os
buracos, pega um pincel e desenha um alvo em volta do grupo mais apertado. Aí mostra o alvo e diz: "cinco
acertos no centro". A conta de acertos está certa, e o alvo é falso. A única coisa que separa a ciência
desse atirador é **quando** o alvo foi desenhado.

Não existe jeito de desfazer isso olhando o resultado. A senha consiste em escrever e **congelar** o
alvo antes do tiro, de um jeito que ninguém consiga alterar depois sem que se perceba. Em cada 100
resultados publicados, os que mais se parecem com trapaca são justamente os limpos, ajustados um pouquinho
depois de ver os dados. A diferença entre "ajustei o modelo" e "ajustei o modelo depois de ver o resultado"
não está no modelo, e sim na **ordem dos fatos**.

**A senha é um número.** Como congelar? Publicando o **hash** do arquivo. O hash é uma impressão digital. Ele pega
o conteúdo inteiro, do primeiro ao último caractere, e devolve uma linha de 64 caracteres. Trocar uma vírgula
muda a impressão toda. Publicar o hash antes do tiro é como lacrar o envelope em cartório. Você não precisa
mostrar o conteúdo agora, mas depois ninguém abre o envelope sem que apareça.

**O cofre.** Imagine um banco que recebe um boleto de cada cliente. Um dia o banco decide construir um
"cofre fora do tempo": uma pasta com os dados dos próximos três meses, que nenhum analista pode abrir. O
modelo é construído com o passado; quando termina, alguém abre o cofre e roda o modelo contra dados que ele
nunca viu. Abrir o cofre custa a única coisa que ele tinha: a surpresa. Na pesquisa, quando um cofre já foi
aberto e o modelo precisa de correção, a correção só vale testada num **cofre novo**, que ninguém viu. Foram
três cofres até acertar a calibração das prisões de Toronto.

**O diário de previsões.** Toda previsão probabilística emitida fica registrada num livro-razão contínuo, com
a fonte, o filtro, se o sistema pode influir no resultado, quantos fatores a justificativa tem e o
degrau de Pearl. E com um limite: **a previsão nunca é 0% nem 100%**, e sim algo entre "um em cem" e
"noventa e nove em cem". Sem esse limite, quem aposta na certeza nunca perde pontos e nunca aprende; com o
limite, cada previsor acumula um histórico que pode ser cobrado.

---

### Degrau 2 — A regra do jogo

**O risco de escala é epistemológico.** Quando um sistema testa centenas de hipóteses e especificações em
paralelo, o maior risco não é técnico; é o sobreajuste em escala industrial, o jardim dos caminhos que se
bifurcam percorrido por máquinas incansáveis. O plano impõe três mecanismos de contenção.

**Pré-registro com hash e contador de tentativas.** Antes que qualquer agente acesse a partição de teste, a
hipótese é congelada com hash. Toda mudança de critério, de teste ou de dados é escrita num arquivo de
declaração, com o seu SHA-256 gravado **antes** de rodar. Nenhuma regra foi mudada depois de ver o resultado
da rodada a que se aplicava, mas cada rodada nova foi escrita sabendo das anteriores, e as correções feitas
depois de ver resultados ficaram registradas e **contam no $K$**.

O sistema mantém um contador $K$ de especificações tentadas por pergunta. Toda tentativa falha fica
registrada, e o relatório final é obrigado a declarar $K$. O plano vai mais longe e pede que $K$ penalize a
complexidade, pelo comprimento mínimo de descrição (MDL) ou pelo fator de Occam, junto com a correção para
comparações múltiplas. Na pesquisa, $K$ foi declarado, mas **nenhuma dessas penalidades foi aplicada**. Foi
por esse contador que a respecificação dos domínios do $H_2$ parou em $K = 3$, e o passo seguinte foi buscar
dados novos. O contador por domínio não registra, porém, que o portão foi tentado em sete rodadas, e
nenhuma correção por isso foi aplicada. A auditoria de fundo depois desse passo mediu o $K$ **honesto** do
portão do $H_2$: **38 testes distintos** (com a faixa de 32 a 72 segundo o critério de contagem), e com
Bonferroni o $H_{2g}$ mantém as quatro áreas de então, com os conjuntos $\hat S$ encolhendo.

**O cofre fora do tempo.** A partição temporal futura fica sob custódia exclusiva do agente de ciclo de vida
($F_3$). Nenhum agente de modelagem a lê durante o desenvolvimento; ela é aberta uma única vez para pontuar
calibração e verificar ausência de vazamento. A regra é dura e tem consequência. Quando um cofre já foi
aberto e o modelo precisa de correção, a correção só vale testada num cofre novo. O $H_3$ fez isso três
vezes: com os voos de 2023, com as prisões de Toronto de 2020–2021 e com as paradas de trânsito de Rhode
Island.

**O diário de previsões e o barramento.** Cada agente que emite previsões probabilísticas as grava num
livro-razão contínuo, restrito ao intervalo $[1\%, 99\%]$. Agentes cujo escore de Brier perde para a
taxa-base, ou cujo capital de Kelly colapsa, têm o peso reduzido pelo orquestrador. O barramento de fichas é
**encadeado por hash**: cada mensagem carrega o hash da anterior, e a trilha não pode ser reescrita sem quebrar
a cadeia. Uma lição de implementação veio cedo. No $H_1$, o placar do diário somava num só o previsor
que aparecia em projetos diferentes, e o erro foi corrigido e declarado no README; os números do $H_1$
publicados depois disso já vêm da versão corrigida.

**Pareceres de risco e o responsável nomeado.** O agente de governança ($F_5$) emite um parecer de risco para
cada projeto avaliado, inclusive os que não chegam à decisão. Na pesquisa foram **21**: 18 do $H_2$ e 3 do
$H_1$ real. Quatro deles envolvem decisões sobre pessoas: Direito (regra de indenização trabalhista),
Educação (turmas do STAR), Saúde (o experimento de seguro da RAND) e o programa de treinamento de LaLonde.

Os 21 pareceres foram assinados em 15 de outubro de 2026, depois de ler um resumo deles que destacava os
quatro que envolvem pessoas. A assinatura fica num registro separado, em que cada parecer é identificado pelo
SHA-256 do arquivo no momento da assinatura. Os arquivos de resultado **não foram reescritos**, para não mudar
as saídas que a reprodução limpa compara. O parecer do Direito carrega uma ressalva factual. A recomendação de
adotar um corte de 10% vem de uma decisão e de um custo **inventados** no $F_1$, e não é conselho de política.

As assinaturas são nominais. Cada parecer traz o nome curto do responsável (**sil**) e o resumo criptográfico do arquivo, em `governanca/ASSINATURAS.md`.

**O limite: o auditor é parte interessada.** Os pareceres foram assinados por quem conduz o programa. Uma
auditoria autorizada por quem é auditado tem valor limitado. O responsável nomeado responde à pergunta que
uma auditoria precisa responder (quem decidiu usar, quem aprovou, quem olha o placar, quem desliga), mas
nomear um responsável **não torna a auditoria independente**. Na direção que uma auditoria exige, o programa
registrou os pontos que a auditoria **não pôde olhar** e quais grupos afetados foram ouvidos. O registro
declara que, nas sete decisões que envolvem pessoas (cortar o benefício em 10%, reduzir as turmas
do STAR, trocar o cuidado gratuito por coparticipação, comparar o treino do NSW com controles do CPS e as
três marcas de revisão dos fluxos de prisões), **nenhum dos sete grupos afetados foi ouvido**. Não há no
repositório registro de consulta, entrevista, painel ou consentimento. Falta também o passo mais barato e
mais óbvio da lista: a elicitação com especialistas humanos.

**A auditoria de fundo: o que foi suposto e o que se mediu.** A governança também serve para marcar os pontos
**não** medidos, e para depois medi-los. A pesquisa se vira contra si mesma numa auditoria de fundo. Dez
pontos que o texto sustentava e que nunca haviam sido postos à prova viraram frentes de trabalho, às quais se
somaram, depois, as extensões (evento raro, saúde, ficha e contrato) e a frente da elicitação humana, que
espera gente. Cada frente repete a regra dos horizontes: a declaração com hash vem **antes** de rodar, o
resultado é relatado com os limites, e mudança de critério depois de ver o resultado entra como declaração
nova e conta no número de tentativas.

| Ponto | A suposição | O que foi medido | Resultado |
| --- | --- | --- | --- |
| 1. Tese central | A arquitetura pega erros que a prática comum deixa passar | Piloto cego de 12 casos, dois braços | Empate no teto: 8 de 8 defeitos nos dois, 0 alarme falso nos 4 casos limpos — a tese não fica sustentada |
| 2. Independência | As saídas aprovadas não dependem de quem as escreveu | Quatro resultados reimplementados só a partir da especificação | Todos concordam; zero erros no original; várias escolhas que a especificação não fixa coincidem com as do autor, três delas só no STAR (evidência fraca) |
| 3 e 4. Multiplicidade | O portão do $H_2$ foi tentado poucas vezes, e nada foi ajustado depois | Inventário das tentativas e das aberturas de cofre | $K = 38$ testes distintos (32 a 72); com Bonferroni o $H_{2g}$ mantém as 4 áreas de então e os conjuntos encolhem |
| 5. Prioris | As prioris de literatura não foram conferidas com especialistas; a rodada de controle não tinha tolerância declarada | Três rodadas em 33 domínios (pacote, literatura e fraca) e a banda de equivalência de 0,05 dp | Cinco domínios mudam de status ou ação com a de literatura e dois com a fraca; o veredito deixa de ser condicional — desvio máximo de **0,0071 dp** e **33 dos 33** domínios idênticos |
| 6. Laço performativo | Quem desiste da fila volta a chamar com probabilidade 0,6 | Desenho quase experimental numa central real, 1999 | **0,145** [0,132; 0,157] — quatro vezes menos que o declarado |
| 7. Linhas dependentes | O ICP supõe linhas independentes, e há 16 instantes por voluntário e 80 escolas | Wald agrupado e permutação em blocos; réplica da Genética com 40 genes novos e taxa nula por permutação ($B = 999$) | A Farmacologia sobrevive intacta, a Educação sai; a Genética se replica em um gene validado, com $p = 0{,}001$ da taxa nula |
| 8. Calibração | A tolerância do ECE pode ser fixa em 0,03 | Nulo simulado com as mesmas previsões e o mesmo $n$ | Nenhum dos seis cofres binários do $H_3$ passa pela regra que depende de $n$; pela regra secundária, o quadro não muda |
| 9 e 10. Escala e dados | O funil escala e o grafo mede conhecimento | 329 domínios em lote; dois domínios novos de saúde | Dois podados no teste severo; 327 até o fim; 26 634 nós, dos quais 6 494 de conceito |
| Evento raro | Um previsor de evento raro serve para decidir quando tem boa calibração | Decomposição de Brier em três cofres: indometacina, COVID e Rhode Island | Nenhum caminho vence nos três: só em Rhode Island RES − REL fica acima de zero (+2,74643e-4 na logística, +3,11655e-4 no ranking), e a habilidade de Brier fica em +0,018629 e +0,017952, abaixo do +0,05 declarado; no COVID a AUC é 0,7685 e a habilidade é −0,020853 |
| Saúde | O círculo que o Lab 0 abriu se fecha com dado real de saúde, em espelho aberto | PhysioNet/CinC Challenge 2019 (sepse precoce) no espelho do PhysioNet; o funil do $H_{2g}$ rodou de ponta a ponta sem uma linha mudada, com o cofre no segundo sistema hospitalar | O efeito existe e é preciso ($\beta = 0{,}0388$ em log-razão de chances por respiração por minuto, IC90 0,0257 a 0,0523), mas a invariância não: 0 de 512 subconjuntos aceitos, $\hat S = \emptyset$, e o G3 falha; a habilidade no cofre é +0,00568 e a taxa de decisão registrada é 8,4% nas internações de um sistema e 3,9% nas do outro |
| Ficha e contrato | A pergunta que o modelo responde é a que o decisor fez; o contrato de dados não diz por que o dado falta | Campo obrigatório `pergunta_do_decisor` no pacote de domínio e bloco `ausencia` no contrato do $F_2$ | Os dois campos entram validados no código; nos três contratos do $H_1$ real e nas 18 tabelas do $H_{2g}$ a regra da ausência é vacuosa hoje e nenhum número publicado muda |
| Elicitação humana (SHELF) | A priori simulada seria substituída por painel de especialistas | Protocolo do painel e seis formulários escritos; critério de $K = 2$ fixado | Nada foi elicitado: nenhuma entrevista, nenhuma previsão corrigida e nenhuma linha de decisão assinada — a frente só tem instrumento |

**Nota de aposentadoria.** Os artefatos das extensões de evento raro, saúde e escala saíram da árvore em 2026-10-16 por não sustentarem resultado central da teoria; os números acima continuam registrados aqui, e o material volta com `git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros`.

Três desses resultados apertam o alcance autorizado pelos capítulos anteriores. O laço de 0,6 que o $H_3$
corrigiu é cerca de quatro vezes a medida de um caso real. A tolerância de 0,03 do erro de calibração é maior
que o ruído em todos os cofres grandes, e nenhum deles passa na regra que depende de $n$. E o portão de
transferência do $H_2$ depende de qual versão de cada escolha conta. A área da Educação não sobrevive à
correção de agrupamento, e quem sustenta a terceira área agora é a Genética replicada. **Nenhum número
publicado caiu por erro de conta; a auditoria estreita o alcance autorizado desses números.**

As questões em aberto têm nome. A elicitação com especialistas humanos de verdade, porque a auditoria
mediu a sensibilidade às prioris e isso não substitui um painel. O protocolo e os seis formulários estão
escritos, com o critério de $K = 2$ pronto, e **nenhuma entrevista foi feita**. A taxa nula da validação por
intervenção da Genética, que decide com um único gene em 27, saiu em $p = 0{,}001$, o piso de 999
sorteios; a resolução segue em aberto ali, e não o $p$. E o tempo de atendimento e a paciência dos
clientes da central, que a base não traz e que continuam supostos.

Fica também o limite de o auditor ser parte interessada, que a auditoria não resolveu. Um dos passos que ela
propõe é que quem assinou releia por inteiro os 21 pareceres assinados, um a um, e confirme ou retire
a assinatura registrando que leu o texto integral. O formulário de releitura das 21 linhas existe, com os
hashes conferidos um a um (**21 de 21 batem**), e está **em branco de propósito**. Nada foi relido, nada foi
confirmado e nada foi assinado ali. Enquanto isso não acontece, a assinatura vale pelo que ela é: a
declaração de quem conduz o programa de que leu um resumo.

---

### Degrau 3 — Mão na massa

Duas tarefas: construir um livro-razão encadeado por SHA-256 e provar que trocar **um caractere** no passado
derruba a corrente; e calcular a raiz de Merkle de um lote de fichas. O código traz o SHA-256 escrito à mão,
sem dependências, e confere a implementação contra os vetores conhecidos. Código em `crates/stochology-book/src/cap15.rs`;
saída medida com `cargo run --release` no crate de rascunho `/tmp/labs-cap15`:

```text
== Capitulo 15 :: o cofre, a senha e a auditoria ==
sha256("")=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
sha256("abc")=ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
corrente: eventos=5 integra=true raiz_merkle=92f2ab76eb6485ab
  evento 0 ficha=h3.voos conteudo=ECE=0.0048 hash=902e82d6ca6cf280
  evento 1 ficha=h3.bolsa conteudo=mediana=2.1x hash=a9c54b2c488f0397
  evento 2 ficha=h3.energia conteudo=canario=ok hash=fbbcfbdbd5970a6c
  evento 3 ficha=h3.prisoes conteudo=ECE=0.036 hash=4248815bc8f679d9
  evento 4 ficha=h3.ri conteudo=ECE=0.0048 habilidade=0.019 hash=8059d78760a22282
adulteracao no evento 2 (canario=ok -> canario=nok, hash do evento recalculado): integra=false raiz_mudou=true
adulteracao + reescrita de 3 eventos seguintes: integra=true mas muda a raiz publicada (true)
um caractere a mais em cada evento: 5 de 5 alteracoes invalidam a corrente
lote de fichas: n=7 raiz_merkle=17679ce182647dbb
trocar uma ficha do lote muda a raiz em 7 de 7 casos
```

Quatro leituras.

**A implementação está certa, e isso é a base de tudo.** Os dois vetores conhecidos do SHA-256 batem
caractere a caractere. A string vazia dá `e3b0c442...b855` e `abc` dá `ba7816bf...15ad`. Sem essa
conferência, todo o resto da trilha seria um número inventado com aparência de segurança.

**Um caractere no passado derruba a corrente.** A corrente de cinco eventos está íntegra. Mudando
`canario=ok` para `canario=nok` no evento 2 e **recalculando o hash daquele evento** (o ataque mais
óbvio, em que conteúdo e hash concordam entre si), a verificação falha. O evento 3 guarda o hash antigo do
evento 2 como "anterior", e a incompatibilidade aparece. Das cinco alterações testadas, uma por evento,
**5 de 5** invalidam a corrente.

**A corrente sozinha não basta: a raiz é a prova.** Se o atacante reescrever também os três eventos seguintes,
a corrente volta a fechar localmente (a verificação passa), mas a **raiz publicada** muda. Essa linha é o
coração do capítulo. A trilha fica inviolável pelo hash publicado **antes**, e não pela corrente. Quem
guarda só a corrente guarda um documento que pode ser reescrito por inteiro; quem guarda a raiz guarda o
lacre.

**A raiz de Merkle muda sempre.** O lote de sete fichas tem raiz `17679ce1...`. Trocando qualquer uma das
sete por uma versão alterada, a raiz muda em **7 de 7** casos. É isso que permite lacrar um lote inteiro com
64 caracteres em vez de 64 por arquivo.

**Declaração de divergência.** Dois pontos ficam registrados. (1) O plano do repositório publica, para o
horizonte `todos`, a raiz de Merkle `9716e28a87bec9cf...` sobre as 12 fichas dos quatro
horizontes, e um hash do relatório `08e7c8f0...`; este laboratório usa uma corrente **própria** de
cinco eventos de livro didático, e por isso a raiz medida aqui é `92f2ab76...`, não aquela. As duas
contas são corretas para os seus conteúdos; comparar as duas só faz sentido se a entrada for a mesma, e é
exatamente assim que a reprodução limpa do G1 funciona. (2) O livro anterior diz que a primeira execução da
central travou porque o Erlang C escrito com fatoriais estourava em cargas grandes; a troca pela recursão
estável de Erlang B só afeta a decisão, não as previsões, mas muda o hash do módulo e **conta no $K$ (3)**.
Nenhum valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 15 | sem perda medida neste capitulo | cada cofre é um grão novo: a régua do eixo R quando a correção muda de pergunta
```

**Exercícios.**

1. *(Conceito)* Por que abrir o cofre duas vezes destrói o seu valor? Relacione com a severidade de Mayo,
   isto é, com a exigência de que um teste pudesse ter dado outro resultado.
2. *(Conceito)* Um colega propõe reescrever os arquivos de resultado com a assinatura dentro. Que propriedade
   da reprodução limpa isso quebraria?
3. *(Conta)* O contador do portão do $H_2$ parou em $K = 3$ por domínio; a auditoria de fundo mediu $K = 38$
   testes distintos no conjunto. Explique, em três frases, por que os dois números convivem sem que nenhum
   esteja errado.
4. *(Laboratório)* Estenda o laboratório para um barramento em que cada mensagem carrega o hash da anterior
   **e** o índice, e escreva um verificador que detecte não só a alteração de conteúdo mas também a troca de
   ordem de duas mensagens.
5. *(Laboratório)* Calcule a raiz de Merkle do lote de sete fichas para as raízes intermediárias e confirme
   quantos hashes são recalculados quando só a última ficha muda.
6. *(Conceito)* Proponha um procedimento para tornar a auditoria dos quatro pareceres que envolvem pessoas
   menos dependente de quem conduz o programa.

**Resumo.**

- O pré-registro com hash é a única senha contra o ajuste depois de ver o resultado. A hipótese é congelada
  antes, e o hash publicado é o lacre.
- O contador $K$ torna as tentativas visíveis; a auditoria de fundo mediu $K = 38$ no portão do $H_2$, contra
  o $K = 3$ por domínio.
- O cofre fora do tempo só se abre uma vez; correções pedem cofre novo, e foram três até acertar a calibração.
- O diário pontua cada previsor com regra própria, dentro de $[1\%, 99\%]$, e pesa os agentes pelo placar; a
  corrente de fichas é encadeada por SHA-256.
- Pareceres assinados nomeiam o responsável, mas não tornam a auditoria independente. Nenhum dos sete grupos
  afetados foi ouvido, e o formulário de releitura dos 21 pareceres está em branco de propósito.

# Parte V. A máquina no mundo real

## Capítulo 16. Horizonte H0: o posto de saúde e os 2,65 milhões de chamadas do banco

**Objetivos.** Ao fim deste capítulo você deve saber por que a mesma demanda média pode produzir duas esperas
muito diferentes, quanto a convexidade da fila cobra de quem decide, por que a unidade independente de uma margem
pode ser o paciente ou o dia e o que a fórmula do setor prometia a uma central de chamadas que ela não
entregou.

---

### Degrau 1 — O mundo real e o paradoxo

**O posto de saúde.** Num posto com um só enfermeiro, o atendimento leva em média 5 minutos por paciente, e a conta
fecha em 12 atendimentos por hora. A demanda é que oscila. Em metade dos dias chegam 5 pacientes por hora, e na
outra metade chegam 11. A média é 8, e 8 é menos que 12, então a fila não cresce sem fim.

O gestor olha a planilha e escreve a frase de sempre: *"a demanda média é 8 e o posto atende 12; a espera é de uns
10 minutos e não há por que contratar"*. A conta por trás dessa frase está certa, e vale 10 minutos, mas só num
mundo em que chegam exatamente 8 pacientes por hora, toda hora. O problema é que esse mundo não é o mundo.

Simulados os dois mundos com a mesma semente (2 000 dias de 10 horas cada, um atendente, semente 1909), a
diferença aparece inteira.

| Mundo (pacientes por hora) | Espera média por paciente | O paciente típico espera | 90 em 100 esperam menos de | Em cada 100 pacientes, mais de 30 min |
| --- | --- | --- | --- | --- |
| Sempre 8, todo dia | 9,3 min | 3,8 min | 26,7 min | 8 |
| Metade dos dias com 5, metade com 11 | 20,7 min | 9,3 min | 59,3 min | 25 |

Nenhuma média mudou. São 8 nos dois casos. O que mudou foi o mundo, e a espera é **curva** na demanda. Quem chega num
dia calmo espera pouco, mas quem chega num dia de surto espera muito, e o ganho do dia calmo não compensa o
estrago do dia cheio. O dia de surto responde por cerca de 70 em cada 100 pacientes (são 110 pacientes contra 50),
de modo que a média por paciente pesa justamente os dias ruins.

**A armadilha da mediana.** Leia a segunda linha da tabela de novo. No mundo que oscila, **o paciente típico
espera 9,3 minutos**. É o mesmo número que a alegação prometia como média. Quem olha o "paciente típico" vê a
promessa cumprida; a diferença inteira está na cauda, onde 25 em cada 100 pacientes passam de meia hora de espera.

**Reforço só nos dias de surto.** Com dois enfermeiros nos
dias de 11 pacientes e um nos dias de 5, a espera média cai para 2,5 minutos e menos de 1 paciente em cada 100
passa de 30 minutos. A folga não precisa ser maior; precisa estar onde está o risco.

**A pergunta escondida é quantos pacientes valem uma margem.** Cada margem de erro precisa de uma unidade. Se o
posto mede a espera média e trata cada paciente como uma observação independente, a margem fica em torno de
±0,07 minuto; se trata cada *dia* como a unidade, ela fica em ±0,51 minuto, sete vezes maior. Qual das duas vale
não é questão de gosto. É o mundo que decide. O teste é direto. Pegue 4 000 dias do mundo da alegação, reparta-os
ao acaso em duas metades e meça a diferença de espera média entre elas. Se os pacientes fossem independentes, essa
diferença deveria andar na casa de 0,047 minuto. Ela anda em 0,216, e a previsão pela unidade *dia*, 0,227, acerta
na mosca. Os pacientes de um mesmo dia compartilham a mesma demanda, e compartilhar não é ser independente.

**O caso real é a central de um banco.** O posto era uma suposição; a central é observação. São 2,65 milhões de
chamadas reais de uma central bancária norte-americana, contadas a cada 5 minutos, ao longo de 164 dias úteis de
2003. A escala é construída nos 83 dias de treino (março a junho), com pico de 234 atendentes e 2 264
agente-horas por dia, e aplicada aos 81 dias de cofre (julho a outubro). A alegação é a prática padrão do
setor: *escalar cada meia hora pela fórmula de Erlang C sobre a taxa média do histórico garante 82% das chamadas
atendidas em até 20 segundos, todo dia.*

A base mostrou isto.

- **A demanda não é Poisson.** Entre dias, o desvio é de cerca de 2 900 chamadas, onde o modelo Poisson preveria
  180; a razão variância/média é 273. Parte disso é calendário (segunda-feira tem +15%), e o resíduo depois do dia
  da semana ainda tem coeficiente de variação de 6%.
- **Jensen mora dentro da própria fórmula.** Aplicado à taxa real de cada dia, o Erlang C da alegação dá 57% em
  média, e não 82%. O nível de serviço é côncavo na carga.
- **A unidade é o dia.** O nulo de metades dá desvio de 0,0474, contra 0,0470 previsto pelo dia e 0,0033 previsto
  pela chamada.
- **Teste severo.** Em 400 dias simulados no mundo que a alegação descreve, nenhum ficou abaixo da meta. No mundo
  real, cerca de um em cada quatro dias ficou.
- **Fora do tempo.** O intervalo conforme de 90% para o total de chamadas do dia, construído sobre a média do dia
  da semana, cobriu 86% dos dias do cofre, e a deriva entre treino e cofre é leve (KS 0,19, p = 0,08).

A decisão também está medida no cofre, contra as chegadas reais:

| Regra de escala | Agente-horas por dia | Em cada 100 dias, dias abaixo da meta | Nível de serviço |
| --- | --- | --- | --- |
| A alegação: perfil médio | 2 264 | 26 | 86% |
| Perfil do dia da semana (custo zero) | 2 261 | 12 | 90% |
| Perfil do dia da semana + margem para a variação residual | 2 428 | 4 (3,7) | 96% |
| A alegação + 12 atendentes por igual | 2 433 | 7 (7,4) | 94% |

As duas últimas regras custam quase o mesmo (cerca de 7% a mais de agente-horas) e colocam a folga em lugares
diferentes. A que põe a folga onde está o risco derruba a falha de 26 para menos de 4 dias em cada 100; a que
espalha atendentes por igual derruba só até 7.

---

### Degrau 2 — A regra do jogo

**A fila, em palavras.** Um posto tem uma taxa de chegada (pacientes por hora) e uma taxa de atendimento
(pacientes por hora). Se a chegada fica abaixo do atendimento, a fila não explode para o infinito; oscila em
torno de um tamanho médio. A espera média de longo prazo de uma fila com um atendente, chegadas sem memória e
atendimento sem memória é

$$W(\lambda)=\frac{\lambda}{\mu\,(\mu-\lambda)},$$

em que $\lambda$ é a taxa de chegada e $\mu$ a de atendimento, ambas na mesma unidade de tempo. Com $\lambda = 8$,
$\mu = 12$ e a hora como unidade, $W = 8/(12 \cdot 4) = 1/6$ de hora, ou 10 minutos. A condição em que a fórmula
vale está escrita na expressão: $0 \le \lambda < \mu$, demanda que não muda e um horizonte longo. Quando
$\lambda$ se aproxima de $\mu$, o denominador vai a zero e a espera explode; é a curvatura que o Degrau 1 mostrou,
agora em fórmula.

**Onde a fórmula falha.** Aplicada a dias que alternam entre 5 e 11 pacientes por hora, ela daria
$(W(5)+W(11))/2 = 29,3$ minutos. A simulação dá 20,7. A diferença ensina três coisas. Primeiro, um dia de 10 horas
não é o longo prazo da fórmula. Nos dias de surto a espera fica em 28,2 minutos, não nos 55 que $W(11)$ prevê,
porque a fila começa vazia e tem só 10 horas para crescer. Segundo, cerca de 70% dos pacientes chegam nos dias de
surto, e por isso a média *por paciente* pesa mais os dias ruins. Terceiro, a fórmula não está errada; ela responde
à espera de longo prazo com demanda constante quando o gestor perguntou quantos pacientes esperam demais nos dias
ruins. É um erro de tipo III. A resposta é certa para a pergunta errada (Capítulo 7).

**Jensen na fila.** A espera é convexa na demanda, e a desigualdade de Jensen (Capítulo 5) diz o que acontece com
qualquer função convexa:

$$\mathbb{E}\big[W(\lambda)\big] \;\ge\; W\big(\mathbb{E}[\lambda]\big), \qquad
\mathbb{E}[W(\lambda)] - W(\mathbb{E}[\lambda]) \approx \tfrac{1}{2} W''(\mu)\,\operatorname{Var}(\lambda).$$

O lado esquerdo é a espera que quem chega realmente sente; o lado direito é a espera do dia médio. Quanto maior a
oscilação da demanda, maior a conta, porque o ganho e a perda não são simétricos. Cinco mundos com a mesma média 8
dão esperas muito diferentes:

| Mundo (pacientes por hora) | Espera média | Em cada 100 pacientes, mais de 30 min |
| --- | --- | --- |
| 8 todo dia (o da alegação) | 9,3 min | 8 |
| 7 ou 9 | 10,5 min | 10 |
| uniforme entre 5 e 11 | 12,3 min | 13 |
| 5 ou 11 | 20,7 min | 25 |
| 5 ou 11, com reforço só nos dias de surto | 2,5 min | menos de 1 |

**A unidade independente decide a margem.** O tamanho efetivo de uma amostra é uma propriedade do processo, não da
contagem (Capítulo 6). Chamadas de um mesmo dia compartilham a taxa e a fila; tratá-las como observações
independentes é **pseudorreplicação**, e a margem sai pequena demais. Os 2,65 milhões de chamadas do cofre valem,
em informação independente, o que valem 81 dias. O modelo nulo de metades arbitra a questão sem convenção. Ele
reparte ao acaso os dias de um mundo conhecido em duas metades, mede a diferença entre elas e diz qual das duas
margens previstas descreve o que se viu. A margem por dia descreve; os dados refutam a margem por paciente.

**O teste severo (Mayo).** Uma diferença entre dois mundos só é evidência se o procedimento teria acusado a
diferença caso ela não existisse. A diferença entre o mundo de 8 e o mundo de 5 ou 11 é de 11,4 minutos, e fica
a 53 desvios do nulo; em 2 000 repartições ao acaso, o nulo nunca passou de 0,81 minuto. É a definição de um
resultado que podia ter dado errado.

**O que ficou suposto no caso real, e como se mediu o tamanho da suposição.** Nem o tempo de atendimento nem a
paciência dos clientes estão na base; o arquivo traz contagens de chamadas, nada mais. O atendimento foi suposto
exponencial com média de 4 minutos, e a paciência exponencial com média de 3. A auditoria transformou as duas
suposições numa malha declarada de 4 × 6 pontos (atendimento de 3, 4, 5 e 6 minutos contra paciência de 1, 2, 3,
5, 7 e 10 minutos) e reescalou as quatro regras ponto a ponto. Por isso a regra do perfil do dia da semana com
margem vence em 20 dos 24 pontos, e nos outros quatro (todos com paciência de 1 minuto) as quatro regras empatam
sem que nenhuma deixe mais de 1 dia abaixo da meta. **Não há ponto de virada.** O canto da malha é o pior caso,
com 5 minutos de atendimento e 10 de paciência. Nem a melhor das quatro regras segura a meta, com 14,2% dos dias abaixo
dela contra 37,7% da regra da alegação. Em cada 100 dias, 38 falham com a escala da alegação e 14 com a regra
certa, que custa 7,2% a mais de agente-horas. A paciência governa o tamanho do efeito; o tempo de atendimento
quase não mexe (com a paciência fixada, a fração da regra vencedora anda no máximo 0,9 ponto percentual em toda a
faixa). O que a malha **não** mede é a família das distribuições (as duas continuam exponenciais, com coeficiente
de variação 1) e a dependência entre paciência e atendimento. E o domínio de saúde do exemplo original ficou de
fora, porque os portais públicos de dados estavam fora do alcance da rede.

**A reprodução.** O ciclo inteiro passou pelos invariantes vigentes naquele momento e reproduziu os seis números
que o material do curso publicava, com contagens idênticas de pacientes nos dois mundos (161 867 e 160 220) e o
mesmo hash nas duas execuções, e isso faz de cada número acima uma medida conferível, e não uma narrativa.

---

### Degrau 3 — Mão na massa

O laboratório reconstrói o Lab 0 em Rust, com semente fixa e sem dependências: SplitMix64 embutido, um dia de
posto com chegadas de Poisson espalhadas no dia, atendimento exponencial e a recursão de Lindley para a espera,
que é a regra "espero o que sobrou da fila, ou nada, o que for maior". O código está em `crates/stochology-book/src/cap16.rs`;
a saída abaixo foi medida com `cargo run --release` no crate de rascunho `~/tmp/labs-cap16`.

```text
== Capítulo 16 :: a mesma média, duas esperas ==
2000 dias de 10 h por mundo, um atendente, mu = 12 por hora, semente 1909 (+ índice do mundo)
mundo (pacientes por hora)   pacientes  espera mediana     p90  >30min   margem/dia     margem/pac.
8 todo dia (a alegacao)        159716     9.1     3.7    26.1      8%         0.14            0.03
7 ou 9                         159057    10.1     4.0    29.1      9%         0.20            0.04
uniforme entre 5 e 11          160475    12.8     5.0    37.1     14%         0.32            0.05
5 ou 11                        158656    19.3     8.1    55.8     24%         0.51            0.07
5 ou 11, reforco no surto      163683     2.5     0.0     8.2      0%         0.03            0.01

nulo de metades: 4000 dias de demanda 8, 2000 reparticoes de 2000 dias contra 2000
desvio medido das diferencas = 0.2062 min
previsto pela unidade dia     = 0.1928 min
previsto pela unidade paciente= 0.0473 min  (319704 pacientes)
nulo: max |diferenca| em 2000 reparticoes = 0.664 min

teste severo: diferenca observada = 10.2 min, 49.4 desvios do nulo, fracao do nulo >= observado = 0.0000
cauda: 8 em 100 pacientes esperam mais de 30 min no mundo 8; 24 em 100 no mundo 5 ou 11
```

Leia linha por linha. **9,1 contra 19,3.** A demanda média é a mesma, 8 pacientes por hora, e a espera mais que
dobra só porque o mundo oscila. **8% contra 24%** e **8,1 contra 55,8** no percentil 90. A média separa os dois
mundos, e a cauda separa muito mais. **Reforço no surto é 2,5 min e 0%**, e a folga onde está o risco zera a cauda.
**0,2062, 0,1928 e 0,0473.** O desvio medido no nulo de metades é cinco vezes maior que a margem da unidade
"paciente". Quem medisse a espera tratando cada paciente como independente declararia uma precisão que o mundo não
tem. **10,2 min, 49,4 desvios, 0,0000.** A diferença entre os mundos é grande demais para vir do acaso, e o
procedimento podia ter dado errado e não deu.

**Declaração de divergência.** A edição anterior deste livro e o `h0-lab0/README.md` registram 9,3 min e 20,7 min
para os dois mundos, 8 e 25 pacientes em cada 100 acima de 30 minutos, medianas de 3,8 e 9,3, percentis 90 de 26,7
e 59,3, e, no nulo de metades, desvio medido de 0,216 contra 0,227 previsto pelo dia e 0,047 pela chamada (teste
severo: 11,4 min, 53 desvios, teto de 0,81 em 2 000 repartições). Este instrumento mede **9,1 e 19,3**, **8 e 24**
em cada 100, medianas de **3,7 e 8,1**, percentis 90 de **26,1 e 55,8**, e, no nulo, **0,2062** medido contra
**0,1928** pelo dia e **0,0473** pela chamada (teste severo: **10,2 min**, **49,4** desvios, teto de **0,664**). A
margem por paciente coincide até a terceira casa, porque é uma conta de desvio padrão sobre o número de pacientes.

**A base real também roda em Rust.** O laboratório acima é o mundo simulado; a central do banco é medida pelo
crate `labs`, lendo `dados-reais/bank_calls.csv` com `cargo run --release --bin labs -- --banco`. Saída medida.

```text
== H0 real :: a central de chamadas do banco, em Rust ==
treino 83 dias (mar-jun) | cofre 81 dias (jul-out) | 2649354 chamadas no cofre
escala do perfil medio: 234 agentes no pico, 2259 agente-horas por dia
nivel de servico no perfil medio = 0.8193 | media dos dias do cofre = 0.5683 | dias abaixo da meta de 80% = 0.6543
dispersao entre dias: Var/media = 246.7 no cofre, 272.6 no treino, 261.1 em todos | desvio entre dias = 2841 | Poisson preveria 181
nulo de metades: por dia = 0.0441 | por chamada = 0.0006
```

A **fila com desistência** e as quatro regras de escala (R0 a R3) rodam em Rust também, em
`crates/stochology-book/src/fila.rs`, com atendimento exponencial de 240 s, paciência exponencial de 180 s, alvo
de 20 s, meta de 80% e semente 2003. Saída medida com `cargo run --release --bin labs -- --fila`.

```text
== H0 real :: a fila com desistencia e as quatro regras de escala (R0 a R3) ==
treino 83 dias (mar-jun) | cofre 81 dias (jul-out) | 2654950 chamadas no cofre
atendimento exponencial 240 s | paciencia exponencial 180 s | alvo 20 s | meta 80% | semente 2003 | 3 replicas por dia
CV residual do dia 0.0598 | margem de R2 = 1 + 1.2816 x CV = 1.0767 | R3 = R0 + 12 atendentes por igual
dias no treino por dia da semana [16, 17, 17, 17, 16, 0, 0]

motor Exato (chegadas observadas, FCFS com desistencia) — a reconstrucao
regra  agente-horas/dia   pico  nivel de servico    dias abaixo da meta     desistencia/100  espera media s espera med. s espera p90 s
R0                 2264    234             83.9%                  30.9%                 8.4              14            0           0
R1                 2261    267             84.4%                  19.8%                 8.1              14            0          44
R2                 2428    287             91.6%                   4.9%                 4.2               7            0           0
R3                 2433    246             90.3%                   9.9%                 5.0               8            0           0

motor Eventos (M/M/c+M evento a evento, chegadas de Poisson) — verificacao independente
regra  agente-horas/dia   pico  nivel de servico    dias abaixo da meta     desistencia/100  espera media s espera med. s espera p90 s
R0                 2264    234             84.4%                  28.4%                 4.3               7            0           6
R1                 2261    267             87.5%                  16.0%                 3.7               6            0          14
R2                 2428    287             95.0%                   4.9%                 1.6               3            0           1
R3                 2433    246             92.4%                   8.6%                 2.2               4            0           0
```

Quatro leituras, com as duas divergências declaradas. **(1)** Os agente-horas batem com o registro:
2 264, 2 261, 2 428 e 2 433 por dia nas quatro regras. **(2)** A ordem é a do registro: R2 é a melhor, R3 perde
para R2 gastando o mesmo, e a alegação (R0) é a pior. **(3)** Os níveis de serviço saem um pouco abaixo do
publicado (83,9% contra 86% em R0, 91,6% contra 96% em R2) e as falhas por dia um pouco acima: 30,9% contra
26% em R0, 19,8% contra 12% em R1, 4,9% contra 3,7% em R2, 9,9% contra 7,4% em R3. **(4)** A verificação
independente, com chegadas de Poisson evento a evento, chega a R2 com 95,0% e 4,9% de dias falhos, encostando no
publicado. O registro mediu com outro motor de simulação e outra semente; a diferença fica declarada, e o que
importa para a decisão, R2 ganha com o mesmo custo, se sustenta nos dois motores. O instrumento foi
reconstruído a partir do código aposentado, que continua no histórico (`git show HEAD:h0-real/estocologia/central.py`),
com duas verificações extras provando o motor: a espera média sem desistência de R0 sai em **1 040 s** contra os
956 s publicados, e o mundo da alegação dá **96,3%** e zero dia falho em 81, contra 97% e zero. Uma diferença de
grade fica registrada. Este instrumento inclui o intervalo das 21:00, e por isso conta 2 654 950 chamadas no
cofre contra as 2 649 354 do instrumento de Erlang C, que corta essa linha.

O instrumento da Tabela de Erlang C reproduz em Rust os números centrais do $H_0$ real. O perfil médio fica em
**82%** (0,8193), a escala da alegação encontra o dia real em **57%** (0,5683) e **65%** dos dias ficam abaixo da
meta (0,6543); o desvio entre dias de
**2 841** contra os **181** que Poisson previria; e a `Var/média` de **272,6 no treino**, que é o 273 publicado.
No cofre ela é 246,7 e na série toda 261,1, e o motivo é o recorte da janela. O nulo de metades por dia sai em
0,0441 contra os 0,0474 publicados (esquema de repartição diferente), e o nulo por chamada desta conta é 0,0006
pela binomial, contra os 0,0033 do roteiro, que usavam outro desenho de correlação intradiária. As duas
divergências ficam declaradas, e o que segue vindo do registro antigo é a tabela de decisão com os esquemas R0 a
R3, que é simulação de fila com desistência e não tem instrumento em Rust ainda.
Os outros valores diferem porque o gerador de números é outro (SplitMix64 aqui, PCG64 no material original) e
porque a espera média por paciente no mundo oscilante é razão de somas de cauda pesada. Rodando o instrumento
com seis sementes diferentes, a espera do mundo 5 ou 11 vai de **19,3 a 20,6 minutos**. A diferença
está dentro da variação do instrumento, e os dois conjuntos de números ficam registrados.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 16 | perda de resolucao 46.24 min (demanda de 5 a 11) | o capítulo traz a tabela R0 a R3 do posto e do banco
```

**Exercícios.**

1. *(Conta)* Use $W(\lambda)=\lambda/(\mu(\mu-\lambda))$ com $\mu = 12$ para achar a espera de longo prazo com
   $\lambda = 8$, $\lambda = 5$ e $\lambda = 11$; depois calcule a média das duas últimas e compare com os 20,7
   minutos da simulação. Por que a fórmula dá mais?
2. *(Conceito)* Por que a média por paciente pesa mais os dias de surto? Calcule, em cada 100 pacientes, quantos
   chegam em dias de 11 e quantos em dias de 5.
3. *(Laboratório)* Rode `crates/stochology-book/src/cap16.rs` com a semente 1909 e confirme a espera do mundo 5 ou 11 e a
   margem por dia. Depois troque o reforço do surto de 24 para 18 atendimentos por hora: a espera média melhora?
   A cauda muda de tamanho?
4. *(Conta)* Em cada 100 pacientes do mundo da alegação, 8 esperam mais de 30 minutos; no mundo real, 25. Quantos
   pacientes a mais são, se o posto atende 8 000 pacientes por ano?
5. *(Conceito)* Escreva, em quatro frases e em frequências naturais, o memorando do H0 real para a diretoria do
   banco, com uma cláusula de saída (que fato novo faria a decisão ser revista).
6. *(Conceito)* Explique por que "a fórmula de Erlang C não está errada, mas a resposta é de tipo III". Que pergunta
   o gestor fez e que pergunta a fórmula respondeu?

**Resumo.**

- A mesma demanda média pode dar 9 ou 20 minutos de espera: a espera é convexa na demanda, e quem olha só a média
  escolhe um mundo que não existe.
- A mediana engana. No mundo que oscila, o paciente típico espera justo o número que a alegação prometia como
  média, e a diferença inteira fica na cauda.
- A margem se calcula na unidade independente, que é o dia, e não o paciente; o modelo nulo de metades arbitra a
  escolha e refuta a pseudorreplicação.
- Em 2,65 milhões de chamadas reais, a escala padrão do setor falha em 26 de cada 100 dias, e gastar cerca de 7% a
  mais de agente-horas onde está o risco reduz isso a menos de 4.

## Capítulo 17. Horizonte H1, parte 1: a ilusão de multiplicar por 3 na bolsa e o programa de emprego

**Objetivos.** Ao fim deste capítulo você deve saber medir o arrasto da volatilidade, distinguir a média do
conjunto da trajetória típica, calcular a alavancagem em que um único dia destrói o capital e reconhecer um viés de
seleção, inclusive no caso incômodo em que o ajuste acerta o número e continua sem valer como evidência.

---

### Degrau 1 — O mundo real e o paradoxo

**A promessa da alavancagem.** Um investidor abre 115 anos de cotações do Dow Jones (de 1885 ao fim de 1999,
31 919 pregões) e calcula a média dos retornos diários, 0,0668 ao ano em soma aritmética. A conclusão parece
inevitável: *"o índice rende 0,07 por ano; alavancado 3 vezes, rende 0,20 por ano, e quem segura por décadas
sempre ganha."*

Só que 0,20 é o número do dia médio visto de fora. Ao longo do tempo, uma **trajetória** vive outra
coisa. Medidos nesses 31 919 pregões, com rebalanceamento diário:

| Alavancagem | A soma aritmética promete | O crescimento realizado no tempo | O que a volatilidade cobra |
| --- | --- | --- | --- |
| 1× | 0,0668 ao ano | 0,0515 ao ano | 0,0153 ao ano |
| 2× | 0,1336 ao ano | 0,0719 ao ano | 0,0617 ao ano |
| 3× | 0,2004 ao ano | 0,0601 ao ano | 0,1403 ao ano |

A alavancagem de 3 vezes promete 20% ao ano e entrega 6%. A diferença, 14 pontos por ano, é o **arrasto da
volatilidade**, um pedágio que se paga todos os dias, sem exceção, e cresce mais rápido que a alavancagem
(mais adiante, com o quadrado dela). Alavancar 3 vezes rende **menos** que alavancar 2. Existe um
ponto ótimo, e passar dele destrói crescimento.

**A sexta-feira que apagou 68%.** Em 19 de outubro de 1987 o índice caiu 22,61% num único dia. Em cada 100 reais
de capital:

- sem alavancagem, restam 77 reais;
- com 2 vezes, restam 55 reais;
- com 3 vezes, restam **32 reais**, 68 reais destruídos em uma sessão;
- com 4,42 vezes, restam zero. Acima dessa alavancagem basta um único dia, e o capital vai a zero sem volta,
  porque não se ganha nada de zero.

**Quem segura por 20 anos.** O teste mais generoso para a alegação é o horizonte longo. Em cada 100 coortes de 20
anos (o investidor compra no primeiro pregão de um mês e vende 20 anos depois):

| Alavancagem | Terminam acima do início | Riqueza mediana | Riqueza somada ÷ número de coortes |
| --- | --- | --- | --- |
| 1× | 96 | 2,13× | 2,9× |
| 2× | 78 | 2,67× | 7,8× |
| 3× | 64 | 2,10× | **19,1×** |

A média de 19 vezes convive com uma mediana de 2,1. O número médio é puxado por um punhado de inícios felizes, e
a coorte típica que o investidor tem na mão mal dobra. Cerca de 35,7 em cada 100 coortes alavancadas
acabam abaixo do ponto de partida, o complemento exato dos 64,3% que terminam acima.

**Quantas observações independentes são?** As coortes se sobrepõem, já que um ano entra em dezenas de
janelas de 20 anos, e por isso as 1 134 coortes do treino valem só cerca de **5,8 blocos
independentes** (`h1-real/README.md`). Contar coortes não é contar observações. Uma margem calculada
sobre 1 134 seria cerca de 14 vezes menor que a honesta ($\sqrt{1\,134/5{,}8}$). É a lição de unidade
independente do Capítulo 6, agora no tempo.

**A Grande Depressão, em frequências naturais.** Do pico de 3 de setembro de 1929 ao fundo de 8 de julho de 1932,
cada 100 reais investidos viram **11 reais** sem alavancagem, e **2,5 centavos** com 3 vezes. Nenhum desses números
sai de simulação. A série é real, e o arrasto médio não registra um dia assim.

**O programa de emprego que parecia um desastre.** Em 1976 e 1977, um programa americano de treinamento
profissional (NSW) inscreveu trabalhadores em situação difícil. Para saber se funcionou, o caminho óbvio é
comparar os participantes com trabalhadores de uma pesquisa nacional (CPS), em 1978:

| Grupo | Em cada 100, empregados em 1978 | Renda média em 1978 |
| --- | --- | --- |
| Participantes do programa (185 pessoas) | 76 | 6 349 dólares |
| Sorteio: 260 controles, também candidatos, fora do programa | 65 | 4 555 dólares |
| Pesquisa nacional (15 992 pessoas) | 86 | 14 847 dólares |

Lida crua, a tabela condena o programa. Os participantes estão 10,7 pontos percentuais **menos** empregados que a
população e ganham 8 498 dólares menos. Lida pelo sorteio, a tabela absolve o programa. Entre candidatos
parecidos, quem entrou ficou 11,1 pontos mais empregado e ganhou 1 794 dólares a mais. Entre as duas
leituras há uma distância de 10 292 dólares por pessoa, e essa distância não é efeito do treino. É a diferença
entre quem se candidata a um programa de treinamento e o trabalhador médio do país.

**O placebo que revela a diferença.** Antes de existir o programa, em 1975, em cada 100 participantes 40 tinham
emprego, contra 89 em cada 100 da pesquisa nacional. Toda a diferença que o ajuste vai tentar consertar já estava
lá, três anos antes de o programa começar.

---

### Degrau 2 — A regra do jogo

**Alavancagem rebalanceada.** Manter alavancagem constante $L$ significa multiplicar a riqueza por $1 + L r_t$
a cada pregão, em que $r_t$ é o retorno simples do índice. Duas quantidades ficam separadas:

- a **média aritmética** do retorno, $\mu = \mathbb{E}[r]$, que a soma dos retornos promete;
- a **taxa de crescimento temporal**, $g_L = \mathbb{E}\big[\ln(1 + L r)\big]$, que uma trajetória
  experimenta no longo prazo.

Elas não coincidem em nenhum mundo com volatilidade:

$$g_L \;\approx\; L\mu \;-\; \frac{L^2\sigma^2}{2}, \qquad
\text{arrasto}_L \;=\; L\mu - g_L \;\approx\; \frac{L^2\sigma^2}{2}.$$

O arrasto cresce com o **quadrado** da alavancagem. Medido no treino (desvio diário $\sigma = 0,010496$, ou
0,1749 ao ano), os arrastos observados são 0,0153, 0,0617 e 0,1403, contra 0,0153, 0,0612 e 0,1377 previstos pela
fórmula. A conta de segunda ordem já pega o essencial, e sobra pouco para os momentos mais altos da distribuição.
A aproximação vale para retornos pequenos e nenhum salto grande; num dia de −22,6% ela deixa de
descrever o acontecido e passa só a subestimar o estrago.

**Não ergodicidade, em uma frase.** A média do conjunto de todos os investidores não é a média do tempo de
nenhum deles. A média de 19 vezes no 3× é uma média de conjunto, feita de um punhado de inícios muito
felizes e de muitas coortes que mal dobraram, e ninguém vive a média. Quando existe um **estado absorvente** (a
ruína, de onde não se sai), a média do conjunto deixa de ser uma descrição útil da vida real.

**Ruína em um dia.** Num mundo de retornos diários, a alavancagem que zera a riqueza num único pregão é

$$L_{\text{ruína}} = \frac{1}{\min_t |r_t|},$$

e no treino ela vale 4,42. Acima disso, a ruína deixa de ser questão de probabilidade e passa a ser uma data marcada no calendário.

**Kelly.** A alavancagem que maximiza a taxa temporal é $L^* = \arg\max_L g_L$. Na posterior do projeto, o
intervalo de 90% vai de **1,26 a 3,44**, com mediana **2,35**; no dia a dia, o Kelly empírico da série é 2,14.
Alavancar 3 vezes está dentro do intervalo, mas no seu extremo superior; o projeto A decidiu por bem menos que
isso, uma alavancagem de 0,26 no exercício simulado dessa lente.

**O teste severo.** O mundo da alegação afirma que a riqueza cresce 3 vezes a soma aritmética anual, sem arrasto. Nele,
a fração das coortes de 20 anos que terminam acima do ponto de partida tem média **0,933** (com o quantil 1% em
0,656). No índice real, a fração é **0,646**, e o procedimento teria acusado esse valor? Só em 0,95% das réplicas
(p = 0,0095). O resultado é severo.

**O que a refutação não alcança.** Delimitar o alvo que caiu é parte do resultado. A alegação refutada é "rende 3
vezes a média" e "sempre ganha". "Alavancar nunca compensa" sobrevive à refutação. Sem custo de financiamento, 2× e 3× ainda
crescem mais que 1× no tempo (0,0719 e 0,0601 contra 0,0515), e o cofre de 2000 a 2023 até favoreceu a alegação.
O previsor sem arrasto teve habilidade 0,44, contra 0,40 do modelo. Duas ausências da base favorecem a alegação e
estão declaradas. A série é só de preço (sem dividendos, e isso subestima o retorno total em cerca de 3% a 4% ao
ano no século XX), e não há custo de financiamento da alavancagem.

**Quem paga o erro.** Os 65,5% da riqueza somada das coortes vêm dos 10% melhores inícios; as piores datas de
entrada são todas de 1929 (3 de setembro, 1º de julho, 1º de agosto). Em cada 100 coortes de 20 anos, terminam
abaixo do início 4 sem alavancagem, 22 com 2× e 36 com 3×. Quem entrou no pior mês teve retorno médio igual ao do
mercado e não sobreviveu para vê-lo.

**A decomposição do LaLonde.** Comparar participantes com não participantes dá:

$$\underbrace{\bar Y_{\text{trat}} - \bar Y_{\text{não trat}}}_{\text{diferença crua}}
= \underbrace{Y_1 - Y_0}_{\text{efeito causal}}
+ \underbrace{\mathbb{E}[Y_0 \mid T=1] - \mathbb{E}[Y_0 \mid T=0]}_{\text{viés de seleção}}.$$

O cofre, os 260 controles sorteados do NSW, fica fora da amostra **por construção**, e esse afastamento não é
temporal. Essas 260 pessoas estiveram fora do ajuste desde o primeiro dia. O sorteio zera o segundo termo por
construção. Os controles foram sorteados entre os candidatos, então $\mathbb{E}[Y_0 \mid T=1] = \mathbb{E}[Y_0 \mid T=0]$. O ajuste por
escore de propensão não zera nada. Ele
**estima** esse termo a partir do que é observável, e o placebo de 1975 mostra que a estimativa erra. Com o
emprego de 1975 como desfecho (um desfecho que o programa não podia ter causado), o ajuste dá −12,3 pontos
(p = 0,04), enquanto o experimento dá +8,5, com intervalo de −0,6 a +17,5, compatível com zero.

**Acertar o número não é ter evidência.** Os três números trazem cada um o seu intervalo de 95%. A comparação crua dá
−10,7 pontos [−17,0; −4,5]; o ajuste por propensão dá +9,6 [−0,6; +20,8]; o experimento sorteado dá +11,1
[+2,6; +19,6]. O ajuste chega perto do experimento e continua sem valer como evidência. Mas um tratamento **inerte** teria dado +9,0, dentro da faixa do nulo, de −10,3 a +11,9 (p = 0,08). O
procedimento não podia ter dado errado de forma grosseira, e por isso o acerto não conta como evidência (Mayo). O
ajuste se apoia em cerca de 103 controles efetivos, e 72% do CPS está abaixo do menor escore dos tratados. Na
prática, ele compara os tratados com um punhado de pessoas do CPS que se parecem com eles; essas pessoas são
diferentes em coisas que o modelo não vê.

**O que o cofre mostrou.** Com os 260 controles sorteados, a habilidade do modelo é +0,18 (previu 0,655 de
emprego, o observado foi 0,646); a regressão logística fica em +0,09 e o contrafactual cru do CPS, em −0,002. O
k-anonimato das linhas vai de 1 a 11 depois da generalização de quase identificadores.

**Justiça, medida.** As taxas de falso negativo diferem por grupo. São 0,72 para hispânicos, 0,50 para negros e 0,57
para os demais. Igualá-las capta mais 22 desempregados e custa 87 falsos positivos. É o trilema de Chouldechova
medido em números. Não há como acertar as três coisas de uma vez, e alguém vai pagar a escolha (Capítulo 11).

**Os dois projetos deste capítulo passaram.** O da bolsa e o do emprego atravessaram os cinco portões do H1 em
domínio real e foram aprovados pela banca adversarial, com os números acima registrados no momento em que foram
produzidos. O terceiro projeto do horizonte, os incêndios dinamarqueses, é o Capítulo 18, e ele foi reprovado.

---

### Degrau 3 — Mão na massa

O laboratório lê as três bases públicas e mede todas as quantidades citadas acima: o crescimento temporal e o arrasto,
as coortes de 20 anos de início mensal, o choque de 1987, a Depressão, o mundo da alegação por reamostragem de
anos e a decomposição crua do NSW contra o CPS. O código está em `crates/stochology-book/src/cap17.rs`; a saída abaixo foi
medida com `cargo run --release` no crate de rascunho `~/tmp/labs-cap17`.

```text
== Capítulo 17 :: arrasto da volatilidade e o viés de seleção ==
treino 1885-1999: 31919 pregoes em 115 anos; serie inteira 1885-2023: 37930 pregoes
media aritmetica diaria = 0.000240669 (0.0668 ao ano), desvio diario = 0.010496 (0.1749 ao ano)

crescimento temporal por ano e arrasto (coortes e trajetorias do treino)
  L=1: a soma aritmetica daria 0.0668 ao ano; o crescimento realizado e 0.0515; arrasto 0.0153
  L=2: a soma aritmetica daria 0.1336 ao ano; o crescimento realizado e 0.0719; arrasto 0.0617
  L=3: a soma aritmetica daria 0.2004 ao ano; o crescimento realizado e 0.0601; arrasto 0.1403
pior pregao: -0.2261 (-22.61%) em 1987-10-19; perda com L=1 0.226, L=2 0.452, L=3 0.678; ruina diaria em L >= 4.42
  ano civil 1929-1932 com L=1: o capital termina em 0.19521 do inicial (80.5% destruido)
  ano civil 1929-1932 com L=3: o capital termina em 0.00088 do inicial (99.9% destruido)
  pico a fundo, 1929-09-03 a 1932-07-08 com L=1: o capital termina em 0.10814 do inicial (89.2% destruido)
  pico a fundo, 1929-09-03 a 1932-07-08 com L=3: o capital termina em 0.00025 do inicial (100.0% destruido)

coortes de 20 anos com inicio no primeiro pregao de cada mes (treino):
L           coortes    mediana (x)      media (x)  acima do inicio
1              1134           2.13            2.9           0.9577
2              1134           2.67            7.8           0.7848
3              1134           2.10           19.1           0.6429

coortes por ano-unidade (115 anos, soma movel de 20): acima do inicio em 1x = 0.9583, em 3x = 0.6458 (96 coortes)
mundo da alegacao (anos do treino reamostrados, crescimento 3*soma aritmetica anual, sem arrasto):
  2000 replicas: fracao media das coortes acima do inicio = 0.9286, quantil 1% = 0.6354, p(fracao <= 0.6458) = 0.0140

LaLonde: NSW com 185 tratados e 260 controles sorteados; CPS com 15992 observacoes
comparacao (desfecho de 1978)               emprego    renda (US$)
experimento sorteado (NSW trat. - NSW ctrl)      0.111           1794
comparacao crua (NSW trat. - CPS)            -0.107          -8498
vies de selecao implicado (cru - experimento)     -0.218         -10292
niveis de emprego em 1978: NSW tratados 0.757, NSW controles 0.646, CPS 0.864; em 100 pessoas: 76, 65 e 86
renda media de 1978 (US$): NSW tratados 6349, NSW controles 4555, CPS 14847
placebo de 1975 (emprego): NSW tratados 0.400, NSW controles 0.315, CPS 0.891; diferenca crua tratados - CPS = -0.491
```

Leia linha por linha. **0,2004, 0,0601, 0,1403**: a soma aritmética do 3× paga 20% ao ano e a volatilidade fica com
14, mais do que o ganho. **0,678** e **4,42**: um dia basta para destruir 68% do capital
alavancado 3 vezes, e acima de 4,42 vezes basta para zerá-lo. **2,10 contra 19,1**: no 3×, a coorte típica mal
dobra enquanto a média do conjunto chega a 19 vezes. **0,6429 e 0,9286**: no índice real, 64 em cada 100 coortes
de 20 anos terminam acima do ponto de partida; no mundo em que a alegação é verdadeira, 93 em cada 100, e a
diferença é o arrasto. **−0,107 e +0,111**: a pergunta sobre o programa dá dois sinais opostos, e a
diferença de −10 292 dólares é o viés de seleção, medido e não suposto.

**Declaração de divergência.** O `h1-real/README.md` registra 0,051, 0,072 e 0,060 de crescimento ao ano;
este instrumento mede **0,0515, 0,0719 e 0,0601** — a coincidência é de três casas, porque a conta é a mesma. As
coortes de início mensal publicadas são 1 135 e as daqui são 1 134 (a diferença é o critério de borda: uma coorte
cujo fim cai no último pregão do treino). Os valores publicados de fração acima do início são 0,9586 / 0,7841 /
0,6405; os daqui são **0,9577 / 0,7848 / 0,6429**, e as riquezas medianas publicadas (2,13 / 2,68 / 2,08) batem
com as daqui (2,13 / 2,67 / 2,10). No mundo da alegação, o material publicado traz média 0,9332, quantil 1% de
0,6563 e p = 0,0095, contra **0,9286**, **0,6354** e **0,0140** aqui: a diferença está no sorteio das
reamostragens (2 000 réplicas em cada caso, geradores diferentes), e o p-valor muda na terceira casa. Um caso
diferente merece registro: a edição anterior cita "de 1929 a 1932, o capital cai a 0,11 com 1× e a 0,00025 com
3×". Com a janela do **ano civil** (1º de janeiro de 1929 ao fim de 1932), este instrumento mede **0,19521** e
**0,00088**; com a janela do **pico ao fundo** (3 de setembro de 1929 a 8 de julho de 1932), mede **0,10814** e
**0,00025** — exatamente os números do material. A divergência não era de método: era de janela, e as duas
aparecem agora, com a data de cada uma.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 17 | sem perda medida neste capitulo | a unidade independente é a pessoa; o eixo R aparece como o grão da comparação
```

**Exercícios.**

1. *(Conta)* Use o arrasto de segunda ordem, $\tfrac{1}{2}L^2\sigma^2$, com $\sigma = 0{,}1749$ ao ano para
   prever o arrasto de 1×, 2× e 3× e compare com 0,0153, 0,0617 e 0,1403 medidos. Em que alavancagem o arrasto
   começa a superar o ganho aritmético?
2. *(Conta)* Qual alavancagem zera o capital no dia de −22,61%? E no pior dia de 1929? Explique por que "ruína"
   é um estado absorvente e o efeito disso sobre a média do conjunto.
3. *(Conceito)* Explique a frase "quem segura por décadas sempre ganha" e distinga o que a refutação
   derruba daquilo que ela deixa de pé.
4. *(Laboratório)* Rode `crates/stochology-book/src/cap17.rs` e refaça a tabela de coortes com horizonte de 10 anos
   (troque a constante `HORIZONTE`). Como fica a fração de coortes 3× acima do início? E a
   mediana?
5. *(Conceito)* Por que o placebo com o emprego de 1975 mede o viés de seleção? Escreva a resposta usando a
   decomposição da diferença crua.
6. *(Conta)* Com os níveis medidos de emprego em 1978 (76% dos tratados, 65% dos controles sorteados), calcule o
   efeito em pontos percentuais e explique por que a comparação crua dá o sinal contrário.

**Resumo.**

- A alavancagem multiplica a média aritmética, mas o tempo cobra o arrasto $\tfrac{1}{2}L^2\sigma^2$. O 3× promete
  0,20 ao ano e entrega 0,06.
- Um único dia pode destruir 68% do capital alavancado 3 vezes; acima de $1/\min|r|$ (4,42 aqui) a ruína é
  questão de calendário, não de probabilidade.
- Média de conjunto e trajetória típica se separam. Os 19,1× de média convivem com 2,10× de mediana, e cerca de 35,7 em cada 100
  coortes 3× terminam abaixo do início.
- Uma refutação precisa declarar o alvo: "rende 3 vezes" e "sempre ganha" caem; "alavancar nunca compensa" não.
- Acertar o número não basta. No LaLonde, o ajuste por propensão dá +9,6 pontos, e um tratamento inerte teria dado
  +9,0; o placebo de 1975 mostra que o viés de seleção continua lá, valendo 10 292 dólares por pessoa.

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
autocontido, com função principal própria, sem dependências, com o acaso embutido em dez linhas de SplitMix64. No
rascunho de escrita ele rodou num crate com a tabela de espaço de trabalho vazia no arquivo de manifesto:

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

## Capítulo 21. As sete lições, os dez pontos da auditoria e o que falta fazer

**Objetivos.** Ao fim deste capítulo você deve saber resumir as sete lições do programa em situações do dia a dia,
ler o balanço dos dez pontos da auditoria de fundo com o resultado de cada um na mão, nomear em voz alta as
questões que seguem abertas e sem resposta, e conduzir o projeto final. Escolher um problema da escola ou da
cidade, preencher a ficha, simular o mundo e defender a conclusão.

---

### Degrau 1 — O mundo real e o paradoxo

**A pergunta que fecha o curso.** No último encontro, alguém pergunta de que serviu o curso na vida real. Não de
que serviu para quem vai fazer estatística, e sim para quem vai decidir. A resposta do programa tem sete frases, e
cada uma nasceu de um caso concreto. As sete vêm contadas do jeito que aconteceram.

1. **A média esconde o essencial.** No posto de saúde, a demanda média é idêntica nos dois mundos, e a espera
   salta de 9,3 minutos no mundo constante para 20,7 minutos no mundo que oscila. Em cada 100 pacientes, 25 esperam
   mais de meia hora no mundo que oscila, contra 8 no mundo constante. Numa central de chamadas, a escala
   dimensionada pela média falha em **26 de cada 100 dias**. E a riqueza média de quem apostou 3 vezes na bolsa
   aparece como 19 vezes, quando a mediana é de 2,1.
2. **A unidade independente decide a margem.** 2,65 milhões de chamadas valem **81 dias**; 225 mil voos valem
   **364 dias**. Contar linha por linha, onde a variação se repete, separa uma margem honesta de uma margem que só
   parece estreita.
3. **Acertar não basta; é preciso ter podido errar.** O ajuste por propensão no experimento de LaLonde acertou o
   número, e um tratamento inócuo teria chegado a um valor parecido. No placebo de 1975 ele inventou um efeito
   fantasma de $-12{,}3$ pontos percentuais. Um número que acerta sem poder errar não é evidência.
4. **Reprovar o próprio modelo é resultado.** Nos incêndios dinamarqueses, 36 meses sem nenhum incêndio gigante
   valem só **2,2 bits** de informação. A chance mensal continua perto de $1/38$ pela regra de Laplace, enquanto a
   seguradora triplicava a aposta. Quando o modelo do programa foi testado no cofre, ele perdeu para a média
   histórica simples (habilidade $-0{,}015$) e **não ganhou o direito de decidir**. A alegação da seguradora caiu.
5. **O efeito transporta melhor que a lista de causas.** A invariância exata é rara. Quando ela aparece e é
   conferida contra uma verdade conhecida (o túnel de luz e os bloqueadores de hERG), protege contra a intervenção
   nova, como o verapamil mostrou. Quando não aparece, a direção e o tamanho do efeito ainda costumam sobreviver.
   9 domínios previram o ambiente nunca visto com habilidade de 0,04 a 0,88.
6. **Calibração se conserta, resolução não.** Foram necessários três cofres para a calibração passar pela
   tolerância de 0,03, e a auditoria mostrou que essa tolerância era frouxa nos cofres grandes. A habilidade final
   ficou em $+0{,}019$. Dizer "está bem calibrado" não assegura que o modelo separe bem os casos.
7. **Declarar antes de rodar torna a falha útil.** Sete rodadas no H2 e três tentativas no H3 só contam como
   ciência porque cada mudança foi escrita com hash **antes** de rodar, e porque cada ajuste feito depois de ver um
   resultado ficou registrado e contado no contador $K$.

**O programa virou o alvo.** Uma disciplina que exige teste severo das alegações alheias precisava passar pelo
teste que aplica aos outros. A auditoria de fundo fez isso. Dez suposições do programa, coisas que o texto
afirmava e que nunca tinham sido postas à prova, viraram frentes de trabalho, cada uma submetida à regra de casa:
declaração com hash antes de rodar, resultado relatado com os limites, e toda mudança de critério depois de ver o
resultado entrando como declaração nova e contando no $K$.

O resultado geral é desconfortável e vale a leitura. **Nenhum número publicado caiu por erro de conta**, e a
auditoria encolheu a autorização desses números. A tese central do livro empatou no teste direto. A tolerância de
calibração era frouxa. O laço de Goodhart declarado era quatro vezes maior que o medido. E o portão do H2 depende
de qual versão de cada escolha conta.

**A quem o programa não ouviu.** Em cada uma das sete decisões que envolvem pessoas (cortar o benefício de
acidentados do trabalho no Kentucky, reduzir turmas no Tennessee, mudar a coparticipação no seguro de saúde,
comparar o treino profissional do NSW com controles do CPS, marcar o detido em Toronto nos dois períodos e marcar o
motorista em Rhode Island), **nenhum grupo afetado foi ouvido**. Não há no repositório consulta, entrevista, painel
nem consentimento. A lista é grande e tem nome e número: 11.598 alunos em 80 escolas; 5.226 detidos; 65.276 prisões
de 2020 e 2021; 86.113 paradas de trânsito; 185 participantes do treino do NSW e 15.992 controles. Assinar não é
escutar. Quem assina um parecer fica do lado de quem decide usar o artefato; quem paga o erro fica do outro lado.

**Duas pontas ficaram no papel.** As duas continuam abertas e as duas dependem de gente. A primeira é a elicitação
de especialistas. O protocolo de painel e seis formulários estão escritos, com o critério de $K = 2$ pronto, e
**ninguém foi entrevistado**. Nenhuma previsão foi corrigida, nada foi assinado. Sem esse painel, cinco dos 33
domínios mudam de status ou de ação dependendo da priori escolhida. A segunda é a voz de quem paga o erro. O
registro dos grupos afetados existe e diz que nenhum foi ouvido.

**E uma ponta que era só de resolução.** Na genética de levedura, a validação que sustentava a réplica decidia com
**um mutante por ambiente**: o menor p-valor possível ali era $1/1\,000$, e o resultado saiu em $p = 0{,}001$, que
é o piso. O p não era o problema — faltava linha. O desenho dessa falta hoje tem instrumento: com uma linha por
mutante o teste aprova **4 de 40** (o que se procura, um artefato de linha e dois que são só ruído, todos no piso
de $0{,}001$, sem que o p separe uns dos outros), e com a segunda linha aprova **1 de 40**, com o piso caindo para
$10^{-6}$ e a estimativa deixando de ser um ponto (`labs --genetica`, Capítulo 19). O que continua aberto é
rodar os 40 genes **reais** com a segunda linha: o instrumento mede o desenho, não os dados da levedura.

---

### Degrau 2 — A regra do jogo

**As sete lições, com a condição em que valem.** As três primeiras lições são aritmética de decisão. A média de
uma função não é a função da média; a margem depende da unidade contada como independente; e um número só é
evidência se o procedimento que o produziu podia ter dado errado. As duas seguintes tratam do ganho que vem da
perda: reprovar o modelo que se construiu é um resultado publicável, e a invariância protege contra a intervenção
nova, enquanto o efeito, sozinho, só promete transportar em média. As duas últimas dizem respeito ao processo. A
decomposição de Murphy separa confiabilidade de resolução, e a declaração prévia separa ciência de trapaça. Nenhuma
das sete vale sem a sua condição. A sexta só vale para previsores que não podem ser melhorados por mais dados, e a
sétima só vale se a declaração for escrita antes e o contador $K$ for publicado depois.

**O balanço dos dez pontos da auditoria de fundo.** A tabela traz a pergunta e a resposta de cada ponto.

| Ponto | A suposição | O que foi medido | Resultado |
| --- | --- | --- | --- |
| 1. Tese central | A arquitetura pega erros que a prática comum deixa passar | Piloto cego de 12 casos, dois braços independentes | Empate no teto: 8 de 8 defeitos nos dois braços e 0 alarme falso nos 4 casos limpos — a tese **não fica sustentada** |
| 2. Independência | As saídas aprovadas não dependem de quem as escreveu | Quatro resultados reimplementados só a partir da especificação | Todos concordam e zero erros no original; várias escolhas que a especificação não fixa coincidem com as do autor, três delas só no STAR (evidência fraca) |
| 3 e 4. Multiplicidade | O portão do H2 foi tentado poucas vezes e nada foi ajustado depois | Inventário das tentativas e das aberturas de cofre | $K = 38$ testes distintos (32 a 72); com Bonferroni o H2g mantém as 4 áreas **antes da correção de agrupamento**, e depois da correção cai para três (Capítulo 19) |
| 5. Prioris | As prioris de literatura não foram conferidas com especialistas; a rodada de controle não tinha tolerância | Três rodadas em 33 domínios e a banda de equivalência de 0,05 dp | Cinco domínios mudam de status ou ação com a priori de literatura e dois com a fraca; o desvio máximo é 0,0071 dp e **33 dos 33** batem |
| 6. Laço performativo | Quem desiste da fila volta a chamar com probabilidade 0,6 | Desenho quase experimental numa central real, 1999 | $0{,}145$ [0,132; 0,157] — quatro vezes menos que o declarado |
| 7. Linhas dependentes | O ICP supõe linhas independentes, e há 16 instantes por voluntário e 80 escolas | Wald agrupado, permutação em blocos e réplica da genética com 40 genes novos | A farmacologia sobrevive intacta, a educação sai, a genética se replica num gene validado com $p = 0{,}001$ |
| 8. Calibração | A tolerância do ECE pode ser fixa em 0,03 | Nulo simulado com as mesmas previsões e o mesmo $n$ | Nenhum dos seis cofres binários passa pela regra que depende de $n$; pela regra secundária, o quadro não muda |
| 9 e 10. Escala e dados | O funil escala e o grafo mede conhecimento | 329 domínios em lote e dois domínios novos de saúde | Dois podados no teste severo e 327 até o fim; 26.634 nós no hipergrafo, dos quais 6.494 são de conceito |
| Evento raro | Um previsor de evento raro serve para decidir quando tem boa calibração | Decomposição de Brier em três cofres: indometacina, COVID e Rhode Island | Nenhum caminho vence nos três: só em Rhode Island a resolução bate a confiabilidade (+2,74643e-4 na logística e +3,11655e-4 no ranking), e a habilidade fica em +0,018629 e +0,017952, abaixo do +0,05 declarado; no COVID a ordenação é boa (AUC 0,7685) e a habilidade é $-0{,}020853$ |
| Saúde | O círculo que o posto de saúde abriu se fecha com dado real de saúde | Sepse precoce no espelho aberto da PhysioNet, com o funil do H2g rodado de ponta a ponta, sem uma linha mudada e com o cofre no segundo sistema hospitalar | O efeito existe e é preciso ($\beta = 0{,}0388$ em log-razão de chances por respiração por minuto, intervalo de 90% de 0,0257 a 0,0523), mas a invariância não: 0 de 512 subconjuntos aceitos, $\hat{S} = \emptyset$, e o portão falha; a habilidade no cofre é $+0{,}00568$ e a taxa de decisão registrada é 8,4% nas internações de um sistema e 3,9% nas do outro |
| Ficha e contrato | A pergunta que o modelo responde é a que o decisor fez; o contrato de dados não diz por que o dado falta | Campo obrigatório `pergunta_do_decisor` no pacote de domínio (18 do H2g, 12 do H2 e a ficha do H0) e o bloco de ausência com MCAR, MAR ou MNAR no contrato | Os dois campos entram validados no código; nos contratos de hoje a regra da ausência é vacuosa — nenhuma coluna tem um único valor ausente — e nenhum número publicado muda |
| Elicitação humana | A priori simulada seria substituída por painel de especialistas | Protocolo do painel e seis formulários, com o critério de $K = 2$ fixado | Nada foi elicitado: nenhuma entrevista, nenhuma previsão corrigida, nenhuma linha de decisão assinada |

**Nota de aposentadoria.** Os artefatos das extensões de evento raro, saúde e escala (pontos 9 e 10) saíram da árvore em 2026-10-16. Não sustentam resultado central da teoria, e os números continuam registrados na tabela acima. O material volta com `git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros`.

Três desses resultados apertam a autorização dos capítulos anteriores. O laço de 0,6 que o H3 corrigiu é cerca de
quatro vezes o valor medido num caso real. A tolerância de 0,03 do erro de calibração é maior que o ruído em todos
os cofres grandes, e nenhum deles passa na regra que depende de $n$. E o portão de transferência do H2 depende de
qual versão de cada escolha conta. A área da educação não sobrevive à correção de agrupamento, e a terceira área
agora se sustenta na genética replicada.

**As sete objeções, e a resposta a cada uma.** Uma disciplina que exige teste severo precisa sobreviver às objeções
que lhe fazem.

| Objeção | Resposta |
| --- | --- |
| "Isso é só estatística com outro nome." | Em parte, sim: quase todas as ferramentas têm dono há décadas. O que a Estocologia acrescenta é a organização — as camadas, a tipagem da incerteza e os mesmos portões para toda alegação. O primeiro teste direto disso, o piloto cego, **empatou no teto**: os dois braços apontaram os mesmos 8 defeitos e os mesmos 0 alarmes falsos. Um empate no teto não sustenta a tese nem a refuta |
| "Os dados são antigos e públicos." | Foi escolha: bases conhecidas, verificáveis por hash e com desfecho sabido permitem auditar o método, mas não substituem uma operação de verdade. Várias peças continuam supostas — o tempo de atendimento, a paciência dos clientes, o painel que nunca foi consultado. Outras deixaram de ser suposição: o laço performativo, a tolerância da calibração e a fragilidade da genética foram medidos |
| "Formalizar o julgamento é eliminar o julgamento." | A arquitetura não decide o que vale a pena perguntar nem quanto um erro custa para quem o sofre. Esses valores entram pela elicitação e pela razão entre custo e perda, declarados e não calculados. A ambição é tornar o julgamento visível |
| "O aparato é pesado demais." | Oito faixas de agentes, cinco lentes e cinco invariantes são muito para uma pergunta simples. O funil responde: a maior parte das hipóteses cai barato, antes de tocar os dados. Se o custo da máquina superar o valor das decisões que ela melhora, a própria lente do juízo manda desligá-la |
| "Bayes resolve o problema de Hume." | Não resolve. A atualização bayesiana é a regra coerente dentro de um espaço de hipóteses, mas a indução se desloca para a escolha da priori e do espaço, como a regra de Laplace mostrou. O próprio teorema de Cox depende de hipóteses técnicas (Halpern, 1999) |
| "A ergodicidade virou doutrina." | A lente do tempo se apoia no programa de Ole Peters, que é contestado, e a disciplina o ensina como debate. Pelo critério do crescimento, a aversão a perdas de cerca de 2,25 medida em laboratório só se justifica quando 56% da riqueza estão em jogo: a tese explica a recusa de arriscar meio patrimônio, não a aversão a apostas pequenas. Algo parecido vale para as caudas: Taleb pede que se suponha cauda pesada até prova em contrário; Clauset, Shalizi e Newman pedem prova antes de declarar uma lei de potência. A questão é de quem é o ônus da prova, e a resposta depende de quanto custa errar |
| "O auditor é parte interessada." | Os 21 pareceres foram assinados por quem conduz o programa, e nomear um responsável não torna a auditoria independente. A auditoria de fundo acrescentou uma checagem mais barata e mais fraca: reimplementar do zero quatro resultados que sustentam as saídas aprovadas, a partir das especificações. Os quatro concordaram e nenhum erro foi achado — mas o teste tem um defeito declarado, porque várias escolhas que a especificação não fixa saíram iguais às do código original, três delas só no STAR. Independência real pede gente de fora, com o mesmo contrato |

**A ficha exige cinco perguntas e quatro proibições.** O instrumento que transforma uma dúvida em alegação
auditável faz cinco perguntas de cabeçalho. Qual é o **processo gerador** dos dados, com semente; onde mora a
**não-linearidade**; qual é a **unidade independente**; qual é a **física do ruído**, se o ruído entra somado
(Itô) ou se a variável é que é arrastada (Stratonovich); e em que degrau da **escada causal** a alegação vive.
Quatro proibições fecham a lista: nenhum texto livre entre agentes, nenhuma conta de cabeça feita por máquina,
nenhuma autoaprovação, e nenhuma probabilidade fora do intervalo de 1% a 99%. Quatro invariantes e sete portões
automáticos verificam a ficha, e o binário de portões sai com código 0 ou 1.

**O roteiro do projeto final.** O projeto vale como o curso inteiro. São quatro semanas de trabalho para um
problema da escola ou da cidade. O roteiro tem oito passos, e o produto final é um memorando em frequências
naturais, com cláusula de saída, defendido em banca.

1. **Escolher o problema.** Serve um problema em que alguém decide algo e há uma ação concreta, como reduzir
   turmas, mudar o horário do intervalo, instalar um bebedouro, multar estacionamento em fila dupla ou mudar o
   ponto de ônibus. Não serve "entender a violência na cidade": falta decisor e falta ação.
2. **Escrever a pergunta do decisor**, em três partes: *Quem decide*: nomeie a pessoa ou o órgão; *Ação que a
   decisão muda*: copie a ação literalmente; *A pergunta do decisor é*: termine com interrogação. Sem essas três
   partes a ficha é recusada, e o laboratório deste capítulo mostra o verificador que a recusa.
3. **Preencher a ficha.** As cinco perguntas de cabeçalho, a alegação em uma frase, o termo de escopo (onde vale e
   onde não vale), a grandeza medida e a unidade.
4. **Fixar o processo gerador** com semente numérica. Quais elementos são sorteados, quais são fixos, quantas
   linhas, quais ambientes. A semente permite que outra pessoa veja aquele mundo.
5. **Construir o modelo nulo.** Aquilo que o acaso sozinho produziria naquele desenho, com a semente repetida. Sem
   nulo não há contraste.
6. **Pré-registrar com hash** e declarar o cofre. A partição de dados que ninguém viu fica guardada, e abre uma vez
   só. Se o critério mudar depois de abrir, o novo critério é declaração nova e entra no contador de tentativas.
7. **Rodar, medir e declarar.** Todos os números do memorando saem de código que outra pessoa roda; toda tentativa
   fica no placar; toda conta de cabeça é proibida.
8. **Escrever o memorando e defender.** Frequências naturais ("em cada 100 alunos"), o gráfico dos resultados
   hipotéticos, a cláusula objetiva de saída, aquela que diz quando a recomendação deixa de valer, e a resposta à
   pergunta final, quando um modelo é bom o bastante. O livro responde com três condições. Um modelo é bom o
   bastante quando ele declara os modos de falha, quando alguém de fora consegue reproduzir cada número, e quando a
   decisão que ele autoriza não depende de nenhuma suposição que não tenha sido escrita.

**Três coisas ficam abertas, com nome.** Nenhuma delas se resolve com mais código.

- **A elicitação com especialistas humanos.** O protocolo e os seis formulários estão prontos; nenhuma entrevista
  foi feita. Cinco dos 33 domínios dependem da priori escolhida, e é esse o tamanho do que ainda não foi medido.
- **A resolução da validação por intervenção da genética.** O desenho está medido: uma linha por mutante aprova
  4 de 40 no piso de $0{,}001$ e a segunda linha aprova 1 de 40, com o piso em $10^{-6}$ (`labs --genetica`).
  Falta rodar os 40 genes reais com a segunda linha, e o p-valor nunca foi o problema.
- **A tese da ponte em escala.** O piloto cego empatou no teto e a tese segue **não sustentada**, agora com portão:
  um registro que a declare sustentada sem o teste justo é reprovado, e o piloto é re-derivado das notas do juiz
  (`labs --ponte`). O teste justo precisa de um escolhedor de casos de fora do programa, de analistas humanos e de um
  juiz humano, e o prazo é 2027-01-14 — vencido sem eles, a tese fica não sustentada em definitivo, e o dono já
  aceitou essa consequência por escrito.
- **A voz de quem paga o erro.** Nas sete decisões que afetam pessoas reais, nenhum grupo afetado foi ouvido. A
  releitura dos 21 pareceres foi feita (17 confirmados, 4 retirados e reemitidos), e ela não substitui a escuta:
  a assinatura do responsável é de quem decide usar o artefato, não de quem paga o erro. A frente tem agora
  protocolo congelado com hash (`fundo/p13-escuta/`) e a caixa lacrada por instrumento: as sete decisões ficam
  como análise de sensibilidade ($n = 1$) e não viram diretriz de política pública nem decisão de portão F8 até a
  cobertura mínima de 2 dos 7 grupos ser atingida, ou até 2026-12-15 (`labs --escuta`).

**Quatro horizontes ficam nomeados.** **Resolução da pergunta:** a pergunta do decisor tem grão, e o grão é variável do modelo. R0 pergunta pelo dia médio, R1 pelo dia que existe, R2 pela hora e R3 pelo subgrupo. Com o recurso total idêntico e braços pareados, perguntar grosso custa **46,24 min** de espera média onde a demanda oscila de 5 a 11, **5,97 min** onde oscila de 7 a 9 e **nada** onde não oscila (`cargo run --release --bin labs -- --resolucao`, em `crates/stochology-core/src/resolucao.rs`). É o eixo novo, ortogonal ao H0–H3. As previsões falsificáveis dele estão medidas: o limiar de oscilação aparece na amplitude 2,5 com o plano calibrado — perda de $+2{,}87$ min no cruzamento e negativa abaixo dele — e **não** aparece com o plano de Erlang; a transferência não tem violação em 6 perfis, com 3 contraexemplos que passam em R0 e falham em algum dia (`labs --resolucao`). **Do argumento ao artefato:** a unidade de conhecimento deste programa é o artefato executável, e não o parágrafo. Alegação, processo gerador, modelo nulo e teste ficam lado a lado. **Agentes artificiais com disciplina epistêmica:** modelos de linguagem entram com rubrica de avaliação ($\kappa = (p_o - p_e)/(1 - p_e)$), registro da versão e da data do fornecedor, e a proibição de texto livre entre agentes que já vale na ficha. **Prazo de validade para a meta do livro:** toda métrica deste livro é declarada com a data em que passa a valer, e o Goodhart medido (Capítulo 20) é a razão de declarar o prazo antes.

---

### Degrau 3 — Mão na massa

O laboratório deste capítulo é o **verificador da ficha do projeto final**: um portão pequeno que recusa uma ficha
sem pergunta do decisor, sem semente e sem cláusula de saída, e prova que sabe reprovar, porque cada uma das sete
regras derruba ao menos uma ficha. O código está em `crates/stochology-book/src/cap21.rs`, autocontido e sem
dependências:

```bash
cargo run --release
```

```text
== Capítulo 21 :: o verificador da ficha do projeto final ==
sete regras, e cada uma tem de reprovar ao menos uma ficha:

  1. R1 processo gerador declarado
  2. R2 unidade independente declarada
  3. R3 modelo nulo com a mesma semente
  4. R4 pergunta do decisor (quem decide, qual ação, pergunta)
  5. R5 semente fixa numerica
  6. R6 hash do pre-registro com 64 digitos hexadecimais
  7. R7 clausula de saida com gatilho objetivo

01 completa                  ACEITA (quem decide: a secretaria municipal de educacao)

02 sem pergunta do decisor   RECUSA
    R4 campo pergunta_do_decisor ausente

03 quem decide generico      RECUSA
    R4 quem decide e generico

04 a acao nao aparece literal RECUSA
    R4 a acao da decisao nao aparece literalmente

05 sem interrogar            RECUSA
    R4 o texto nao termina em pergunta

06 sem semente               RECUSA
    R5 sem semente fixa numerica

07 sem clausula de saida     RECUSA
    R7 sem clausula de saida

08 clausula sem gatilho      RECUSA
    R7 clausula de saida sem gatilho objetivo

09 sem pre-registro          RECUSA
    R6 hash do pre-registro ausente ou fora do formato

10 sem modelo nulo           RECUSA
    R3 modelo nulo ausente ou sem a mesma semente

11 sem gerador               RECUSA
    R1 campo gerador vazio ou curto demais

12 sem unidade               RECUSA
    R2 campo unidade vazio

12 fichas: 1 aceita, 11 recusadas
regras que reprovaram ao menos uma ficha: 7 de 7 | reprovacoes por regra: [1, 1, 1, 4, 1, 1, 2]
```

Leia linha por linha. **A ficha 01 é aceita** com quem decide nomeado, a ação copiada literalmente da decisão, a
pergunta terminando em interrogação, a semente 2026 no processo gerador e no modelo nulo, o resumo criptográfico de
64 dígitos e a cláusula de saída ancorada num número ("acima de 0,10 dp em três anos seguidos"). **As outras onze
são recusadas, e cada recusa é um defeito distinto.** R4 aparece quatro vezes porque a pergunta do decisor é a
regra com mais partes: ausente, genérica ("o decisor" não nomeia ninguém), sem a ação literal e sem a interrogação.
R7 aparece duas vezes: uma ficha sem cláusula nenhuma e outra com uma cláusula sem gatilho ("enquanto o modelo
continuar bom e ninguém reclamar"), que não diz quando a recomendação deixa de valer. A última linha é a conta que
interessa. **7 de 7 regras reprovaram ao menos uma ficha**. Um portão que nunca reprova não é um portão; é um
carimbo.

**Declaração de divergência.** O verificador do laboratório é uma reimplementação reduzida do validador do
repositório. O validador da pesquisa exige, além das regras acima, no mínimo 60 caracteres no texto da pergunta,
recusa uma lista maior de nomes genéricos e roda sobre 18 pacotes do H2g, 12 do H2 e a ficha do H0, sem que
nenhum número publicado mude; o do laboratório roda sobre 12 fichas de teste e não toca nos dados de ninguém. O
que os dois têm em comum é o essencial: a pergunta do decisor só vale se nomeia quem decide, copia a ação
literalmente e termina em pergunta.

**O lacre da escuta.** O segundo instrumento do capítulo é o portão de quem paga o erro. Ele lê o registro em
máquina das sete decisões que mexem com pessoas reais, conta a cobertura de escuta, confere se alguma decisão se diz
emitida sem ninguém ouvido e dá o veredito do lacre. O protocolo está em `fundo/p13-escuta/protocolo.md`, congelado
com hash antes da primeira tentativa de contato:

```bash
cargo run --release --bin labs -- --escuta
```

```text
== Ponto 13 :: a escuta de quem paga o erro ==
decisoes: 7 | carimbo do registro: 2026-10-16 | conducao: protocolo cego | prazo da janela: 2026-12-15 | cobertura minima: 2 de 7
decisao                              titulares ouvidos      registro   pode emitir
h2g/saida/direito_teto_beneficio             - nenhum       -          nao
h2g/saida/educacao_star_turmas           11598 nenhum       -          nao
h2g/saida/saude_rand                         - nenhum       -          nao
h1-real/saida/b                          16177 nenhum       -          nao
h3/prisoes                                5226 nenhum       -          nao
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

cobertura de escuta: 0 de 7 grupos ouvidos
divergencias: nenhuma — nenhuma decisao se diz emitida sem ninguem ouvido
veredito: CAIXA LACRADA — a cobertura de 0 fica abaixo do minimo de 2; as decisoes ficam como analise de sensibilidade (n = 1) e nao viram diretriz de politica publica nem decisao de portao F8 ate 2026-12-15 ou ate a cobertura minima
```

**Nenhuma das sete coortes históricas está ao alcance** — são crianças do STAR dos anos 1980, famílias do RAND HIE,
detidos de Toronto de 1997, motoristas de Rhode Island de 2005 —, e o programa declara isso em vez de inventar
consulta que não houve. O que a escuta procura é equivalente estrutural: quem hoje ocupa a mesma posição de quem
pagou o erro então. O roteiro tem três perguntas em linguagem comum (pior cenário plausível, o que faria recusar,
quem de fato arca com o custo), quem conduz é de fora do programa ou com protocolo cego, e o operador **não conta**
como grupo ouvido. O prazo é 2026-12-15, a cobertura mínima é de 2 dos 7 grupos com registro assinado, e o que não
for coberto fica arquivado como exercício de sensibilidade. A prova negativa está no próprio instrumento: uma
decisão que se diz emitida com a coluna "ouvidos" em nenhum é reprovada, e a caixa só abre quando a cobertura chega
ao mínimo.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 21 | limiar de oscilacao na amplitude 2.5 (plano calibrado) | o capítulo nomeia o eixo e as previsões 3 e 5
```

**Exercícios.**

1. *(Conceito)* Escolha um problema da sua escola e escreva a pergunta do decisor nas três partes exigidas. Depois
   troque o decisor por "o decisor" e explique, em duas frases, por que a ficha deixaria de ser auditável.
2. *(Conceito)* Escreva uma cláusula de saída com gatilho objetivo para uma decisão da sua cidade e uma cláusula
   sem gatilho nenhum. Qual das duas sobrevive a uma troca de gestão?
3. *(Laboratório)* Rode o verificador e acrescente uma ficha 13 com uma pergunta de decisor correta e uma semente
   não numérica ("qualquer"). Quantas fichas são recusadas agora, e por qual regra?
4. *(Conta)* No balanço da auditoria, cinco dos 33 domínios mudam de status ou de ação quando a priori de
   literatura substitui a do pacote. Escreva esse número como frequência natural ("em cada 100 domínios...") e
   diga por que ele é o tamanho do que ainda falta medir.
5. *(Conceito)* O piloto cego empatou no teto: cada braço apontou 8 defeitos, e as duas listas saíram idênticas.
   Explique, em quatro frases, por que um empate no teto não sustenta a tese e também não a refuta.
6. *(Ensaio)* Responda à pergunta que fecha o curso: quando um modelo é bom o bastante? Use pelo menos três
   números deste livro.

**Resumo.**

- As sete lições resumem o programa: a média esconde; a unidade independente decide a margem; acertar não basta
  se o teste não podia falhar; reprovar o modelo que se construiu é resultado; o efeito transporta melhor que a
  lista de causas; calibração se conserta e resolução não; declarar antes de rodar torna a falha útil.
- A auditoria de fundo testou dez suposições do programa e as extensões. Nenhum número publicado caiu por erro
  de conta; a auditoria estreitou a autorização desses números. A tese central empatou, a tolerância de calibração
  era frouxa, o laço declarado era quatro vezes o medido.
- Ficam em aberto, com nome: a elicitação com especialistas humanos (5 de 33 domínios dependem da priori), a
  segunda linha nos 40 genes reais da genética (o desenho já tem instrumento: 4 de 40 passam com uma linha, 1 de
  40 com duas) e a voz de quem paga o erro (nenhum grupo afetado ouvido nas sete decisões que envolvem pessoas, com
  a caixa lacrada por instrumento e prazo em 2026-12-15).
- O projeto final fecha o curso: escolher um problema da escola ou da cidade, escrever a pergunta do decisor,
  preencher a ficha, fixar a semente, construir o nulo, pré-registrar, rodar e defender um memorando em frequências
  naturais com cláusula de saída.

# Apêndices

## Apêndice A. Dicionário visual da Estocologia

O dicionário reúne todos os termos do livro, cada um definido em uma frase simples e ilustrado com um exemplo
do dia a dia. A ordem é alfabética.

| Termo | Em uma frase | Exemplo do dia a dia |
| --- | --- | --- |
| **Abdução** | Escolher a hipótese que torna o que se viu menos surpreendente, entre as hipóteses consideradas | O barulho no motor acorda o vizinho: a explicação mais simples é o carro, não um drone |
| **Agente ($L_3$)** | Quem decide, com tempo, atenção e dinheiro limitados: pessoa, instituição ou algoritmo | Você escolhendo o caminho de casa às 7h da manhã, com o tanque na reserva |
| **Amostra filtrada** | Uma amostra que só contém os casos que sobreviveram a algum filtro, tomada pela população inteira | Contar quantos músicos famosos abandonaram a faculdade e concluir que a faculdade atrapalha |
| **Aresta de patologia** | A marca, no mapa do conhecimento, de um desalinhamento entre as camadas do mundo, do modelo e do agente | Duas pastas do mesmo cliente com números de telefone diferentes, e ninguém saber qual vale |
| **Auditoria de fundo** | Uma frente que testa uma suposição do próprio programa, com declaração escrita antes de rodar | O professor que refaz a própria prova antes de aplicar aos alunos, com gabarito fechado |
| **Banca adversarial** | A etapa em que alguém ataca o trabalho ao vivo, procurando onde ele quebra, antes de aprovar | O treino em que o técnico faz o time reserva jogar contra o titular, sem piedade |
| **Barramento de fichas** | O canal por onde as peças do trabalho trocam fichas, encadeado por resumo criptográfico, de modo que nada possa ser reescrito sem quebrar a corrente | A ata de uma reunião assinada página por página: trocar uma página invalida a assinatura das seguintes |
| **Bayes (regra de)** | A regra que diz como a crença de hoje se atualiza com a evidência de agora, partindo da crença de ontem | Um teste de gravidez positivo em uma pessoa de risco baixo ainda pode ser falso positivo na maioria das vezes |
| **Brier (escore de)** | A média do quadrado do erro de uma previsão probabilística: menor é melhor, e 0 é perfeito | Apostar 90% em um jogo e perder custa mais caro no placar do que apostar 55% e perder |
| **Calibração** | A propriedade de quem diz "30%" e acerta em cerca de 30% dos casos | Um meteorologista que diz "chance de chuva" 100 vezes e chove em 62 delas está descalibrado |
| **Canário** | Um vazamento plantado de propósito para provar que o teste de vazamento funciona | A câmera de teste que a loja dispara de propósito para conferir se o alarme toca |
| **Cauda pesada** | Uma distribuição em que eventos extremos são muito mais frequentes do que a curva em sino faria supor | Dias de bolsa a −10% existem; em um mundo gaussiano, quase nunca aconteceriam |
| **Cláusula de saída** | O gatilho objetivo que diz quando a recomendação deixa de valer | "Reviso o remédio se a febre passar de 39 °C por dois dias seguidos" |
| **Cofre** | A parte dos dados que fica guardada até o pré-registro e se abre uma única vez | O envelope lacrado da prova, aberto só depois de o aluno entregar a resposta |
| **Confundidor** | Uma causa comum de duas variáveis que faz parecer que uma causa a outra | Sorvete e afogamento sobem juntos: os dois sobem com o calor |
| **Contador $K$** | O placar do número de especificações tentadas para responder à mesma pergunta | O número de tentativas anotado na margem do caderno para acertar uma senha |
| **Crisonomia** | A lente do juízo: valor da informação, escores próprios, calibração e custo do erro | Pagar por um exame a mais antes de decidir se a cirurgia vale a pena |
| **Crononomia** | A lente do tempo: a média de muitas pessoas contra a média de uma vida, ruína e crescimento | Quem aposta metade do patrimônio cada mês, quebra antes de colher a média bonita |
| **Desenho (efeito de)** | Quantas linhas dependentes valem uma linha independente | As mesmas 30 pessoas respondendo 10 vezes não valem 300 pessoas |
| **Disjuntor** | O mecanismo que troca o modelo por um reserva seguro quando ele perde numa janela móvel | O piloto automático que se desliga quando o sensor discorda por tempo demais |
| **$\operatorname{do}(x)$ (intervenção)** | Fixar uma variável por decisão, e não apenas observar quem já a tem | Sortear quem recebe o remédio, em vez de comparar quem já tomava com quem não tomava |
| **ECE (erro esperado de calibração)** | A distância média entre o previsto e o observado, calculada por faixas de confiança | Estimar o erro de um termômetro faixa por faixa, de 35 a 42 graus |
| **Erlang C** | A fórmula clássica que estima quantos atendentes são precisos para uma fila | Quantos caixas abrir no mercado para a fila não passar de cinco pessoas |
| **Erro de tipo I** | Alarmar quando não havia nada | O detector de fumaça que toca com o pão torrando |
| **Erro de tipo II** | Não alarmar quando havia algo | O detector com a pilha fraca que não toca no incêndio |
| **Erro de tipo III** | Responder com precisão a uma pergunta que não era a do decisor | Entregar a nota média quando a pergunta era quantos alunos ficam para recuperação |
| **Erro de tipo M** | Errar o tamanho do efeito, para mais ou para menos | "Sorteia 3 centímetros de chuva" quando foram 30 milímetros |
| **Erro de tipo S** | Errar o sinal do efeito: dizer que melhora quando piora | Recomendar um remédio que atrapalha, por causa de um ruído de sinal |
| **Escalonomia** | A lente da escala: expoente da cauda, existência da média, tamanho efetivo | Medir se uma enchente pode engolir a média histórica de enchentes |
| **Escore próprio** | O escore em que declarar a própria crença sincera é a melhor jogada | O bolão em que chutar o mais provável, e não o time do coração, é o que pontua |
| **EVPI e EVSI** | Quanto vale, em dinheiro esperado, a informação perfeita e a informação de uma amostra | Quanto pagaria para saber hoje a demanda de amanhã, antes de comprar o estoque |
| **Ficha** | A lista fechada de perguntas feita a um texto, a um número ou a um modelo | O checklist do mecânico: sem ele, sempre esquece o mesmo item |
| **Fluxonomia** | A lente da corrente: como a probabilidade e a informação fluem, e onde se perdem | O telefone sem fio: a mensagem perde bits a cada repetição |
| **Frequências naturais** | Dizer o risco como "x em cada 100", com denominador fixo | "Em cada 100 pacientes, 25 esperam mais de meia hora" em vez de "25% de risco" |
| **Goodhart (lei de)** | Quando a medida vira a meta, ela deixa de medir | A escola que treina os alunos para a prova e a nota sobe sem o aprendizado subir |
| **Habilidade de Brier** | Quanto o modelo melhora, em porcentagem, do climatologista que só repete a taxa-base | Quanto a previsão de chuva ganha do "chove com a mesma frequência de sempre" |
| **Hipergrafo tipado** | Um mapa do conhecimento em que as ligações têm tipo e podem unir mais de duas coisas | O mapa de metrô com linhas nomeadas, e não só pontos ligados por traços |
| **ICP (predição causal invariante)** | O método que devolve o conjunto de variáveis cuja relação com o desfecho não muda entre ambientes | A receita que dá certo na sua casa e na casa da avó, com o forno diferente |
| **Identidade de Meng** | A equação que diz quanto da resposta final depende do dado que falta | Avaliar a turma medindo só quem veio à prova: os ausentes mudam a média |
| **Incerteza aleatória** | A variabilidade do mundo, que mais dados não reduzem | O resultado do dado honesto: rolar dez mil vezes não deixa o dado previsível |
| **Incerteza epistêmica** | A ignorância do modelo sobre o mundo, que encolhe com dados | Não saber a altura média da turma; medir todos resolve |
| **Incerteza knightiana** | Um processo real fora do espaço de hipóteses, em que mais dados dão falsa confiança | O peru que engorda por mil dias até o dia do Natal |
| **Invariantes $\Phi_1$ a $\Phi_4$** | As quatro condições que toda alegação precisa cumprir: pertinência, contraste, justiça e cláusula de saída | As quatro perguntas da banca que reprovam um trabalho, faça ele o que fizer |
| **Jensen (desigualdade de)** | A média de uma função curvada não é a função da média | A média dos tempos de espera cresce quando a chegada oscila, mesmo com a mesma média de chegada |
| **Kelly (critério de)** | A fração do patrimônio que maximiza o crescimento no longo prazo | Apostar 20% quando a chance é 60% a 1 para 1; apostar tudo leva à ruína |
| **Kupiec (teste de)** | O teste que confere se o número de violações do valor em risco é o esperado | Conferir se o seguro de enchente acionou na frequência prometida |
| **Laplace (regra de)** | A regra que soma um sucesso e uma falha imaginários antes de estimar uma probabilidade | "Nunca vi isso acontecer" em 36 meses não é o mesmo que "não pode acontecer" |
| **Lentes (as cinco)** | Os cinco modos de olhar fixos do programa: escala, corrente, forma, tempo e juízo | Cinco especialistas examinando a mesma casa: encanador, eletricista, pedreiro, arquiteto e o morador |
| **Modelo ($L_2$)** | A representação formal, simplificada e simulável do mundo | A maquete do prédio, com paredes de papel |
| **Modelo nulo** | A simulação do que o acaso sozinho produziria, para comparar com o que se observou | Sortear grupos por sorteio honesto mil vezes antes de acreditar no efeito do remédio |
| **Morfonomia** | A lente da forma: o que se conserva quando o ambiente muda | O sotaque que sobrevive à mudança de cidade |
| **Mundo ($L_1$)** | A realidade que gera os dados e que nunca vemos inteira | O bairro de verdade, com os buracos que o mapa não mostra |
| **Murphy (decomposição de)** | A conta que separa o erro de uma previsão em confiabilidade, resolução e dificuldade do evento | Um bom aluno que erra a prova por dificuldade, não por não saber a matéria |
| **Não-ergodicidade** | Quando a média de uma trajetória no tempo não é a média de muitas trajetórias | A média das pessoas que atravessaram o rio a pé não descreve quem foi levado pela correnteza |
| **$p$-valor** | A frequência com que um mundo sem efeito produziria um resultado tão extremo quanto o visto | Jogar 100 vezes a moeda e ver 62 caras: quão estranho é isso no mundo honesto? |
| **Pearl (escada de)** | Os três degraus: ver, agir e imaginar o que não aconteceu | Ver o preço do tomate subir, decidir comprar abacaxi, e imaginar como estaria sem a geada |
| **Pergunta do decisor** | A pergunta que a decisão usa, com quem decide, qual ação muda e a interrogação no fim | "Reduzir a turma para 15 alunos melhora a leitura um ano depois?" |
| **Portão** | O teste automático que diz sim ou não, com código de saída 0 ou 1 | A catraca do metrô: não abre para quem não passou |
| **Pré-registro com hash** | Congelar o critério antes de ver os dados, com o resumo criptográfico gravado | Escrever o palpite e lacrar o envelope antes de abrir o resultado |
| **Pseudorreplicação** | Tratar observações dependentes como independentes, encolhendo a margem sem razão | Contar dez fotos da mesma pessoa como dez pessoas na multidão |
| **Resolução** | A capacidade de separar os casos que têm o evento dos que não têm | Distinguir os dias de chuva forte dos garoa, e não só acertar a média do mês |
| **Ruína** | Chegar a zero e não poder mais jogar, independentemente da média das apostas | Quem usa todo o cartão no limite perde o direito de continuar no jogo |
| **Saturação** | Quando o ambiente é função das variáveis do conjunto, e o teste não tem como reprovar | Contar a cor do semáforo para prever a cor do semáforo |
| **Semente fixa** | O ponto de partida do sorteio, declarado para que duas máquinas produzam o mesmo mundo | A receita com a quantidade exata, para o bolo sair igual na casa do vizinho |
| **Severidade** | A chance de o procedimento ter dado errado se a afirmação fosse falsa | O exame que só é útil se detecta a doença quando ela existe |
| **SHELF** | O protocolo de elicitação de especialistas por mediana, quartis e extremos | Pedir ao médico não um palpite, mas uma faixa e um valor mais provável |
| **$t_{\mathrm{disp}}$ (relógio)** | O instante em que cada dado se torna conhecível | O resultado do exame que só existe depois de sair do laboratório |
| **Termo de escopo** | A declaração de onde a conclusão vale e onde não vale | "Isto vale para o turno da manhã; no fim de semana a fila é outra" |
| **Trilema de Chouldechova** | Com taxas-base diferentes entre grupos, calibração e taxas de erro iguais são incompatíveis | Duas turmas com dificuldades diferentes não podem ter, ao mesmo tempo, nota média justa e o mesmo número de reprovações |
| **Unidade independente** | A menor unidade cujas observações não compartilham a mesma fonte de variação | A escola, e não o aluno, quando alunos da mesma escola se parecem entre si |
| **Verossimilhança** | A chance de ver o que se viu, supondo cada hipótese verdadeira | "Quão provável é esta mancha no telhado se for chuva? E se for infiltração?" |
| **Viés (três sentidos)** | Distorção do dado, do estimador e da mente, e as três se acumulam | Uma balança desregulada, uma média que mistura pesos e a pressa de quem lê o número |

---

## Apêndice B. Guia de bolso das fórmulas

### B.1 Como ler os símbolos

| Símbolo | Leia em português | Em código Rust |
| --- | --- | --- |
| $\sum_{k=1}^{n} a_k$ | "Some tudo o que vem depois, de $k$ igual a 1 até $n$" | `let soma: f64 = a.iter().sum();` |
| $\mathbb{E}[X]$ | "A média de longo prazo de $X$" | `let media = x.iter().sum::<f64>() / x.len() as f64;` |
| $\operatorname{Var}(X)$ | "A dispersão média de $X$ em torno da sua média, ao quadrado" | `let var = x.iter().map(\|v\| (v - media).powi(2)).sum::<f64>() / x.len() as f64;` |
| $\sigma$ ou $\mathrm{dp}$ | "Desvio padrão: a raiz da variância, na mesma unidade de $X$" | `let dp = var.sqrt();` |
| $P(A \mid B)$ | "Probabilidade de $A$ **dado que** $B$ aconteceu" | `if b { p_a_dado_b } else { p_a_dado_nao_b }` |
| $\operatorname{do}(x)$ | "Fixar $x$ por decisão, e não apenas observar quem já tem $x$" | `let y = funcao(x_fixo);` — em vez de `let y = funcao(x_observado);` |
| $\ln$ | "Logaritmo natural: o expoente do número de Euler que devolve o valor" | `let z = valor.ln();` |
| $\log_{10}$ | "Logaritmo na base 10" | `let d = 10.0 * razao.log10();` (decibans) |
| $\ge$ | "É maior ou igual a" | `a >= b` |
| $\approx$ | "Aproximadamente igual", dentro de uma tolerância declarada | `(a - b).abs() < 1e-9` |
| $\sqrt{x}$ | "A raiz quadrada de $x$" | `x.sqrt()` |
| $x^2$ | "O quadrado de $x$" | `x.powi(2)` |
| $\alpha$ | "O nível de alarme falso que se aceita: 5% é o de sempre" | `const ALFA: f64 = 0.05;` |
| $1 - \alpha$ | "A cobertura prometida do procedimento: 95% no caso de sempre" | `let cobertura = 1.0 - ALFA;` |
| $\hat{S}$ | "O conjunto devolvido pelo método, com chapéu de estimado" | `let s_hat: Vec<usize> = ...;` |
| $\emptyset$ | "O conjunto vazio: o método não apontou nada" | `Vec::new()` |
| $\cap$ e $\subseteq$ | "A interseção de dois conjuntos" e "está contido em" | `a.iter().filter(\|x\| b.contains(x)).count()` |
| $\Phi_1$ a $\Phi_5$ | "Os invariantes que toda alegação precisa cumprir" | um portão por invariante, com código de saída 0 ou 1 |
| $\kappa$ | "O acordo entre avaliadores, já descontado o que o acaso daria" | `(po - pe) / (1.0 - pe)` |
| $\theta$ | "O parâmetro que se quer estimar: o efeito, a inclinação, a taxa" | `let theta = 0.148;` |
| $\bar{y}$ e $\bar{p}$ | "A taxa observada" e "a média das previsões" | `let citl = p.iter().sum::<f64>() / n as f64 - y.iter().sum::<f64>() / n as f64;` |

### B.2 As fórmulas que o livro usa

| Fórmula | Em palavras | Em código |
| --- | --- | --- |
| $P(H \mid E) = \frac{P(E \mid H)P(H)}{P(E)}$ | A crença de hoje é a de ontem multiplicada pela força da evidência | chances posteriores $=$ chances anteriores $\times$ razão de verossimilhança |
| $10\log_{10}\text{RV}$ | A força da evidência em decibans: cada 10 dB é um fator de 10 | `10.0 * rv.log10()` |
| $\operatorname{Brier} = \frac{1}{n}\sum (p_i - y_i)^2$ | A média do quadrado do erro de uma previsão probabilística | `p.iter().zip(y).map(\|(a,b)\| (a-b).powi(2)).sum::<f64>() / n as f64` |
| $\text{habilidade} = 1 - \operatorname{Brier}/\operatorname{Brier}_{\text{clima}}$ | Quanto o modelo ganha do climatologista | `1.0 - brier / brier_ref` |
| $\mathrm{ECE} = \sum_k \frac{n_k}{n}\lvert \bar{p}_k - \bar{y}_k\rvert$ | A distância média entre previsto e observado, faixa por faixa | 10 caixas de massa igual, somando a diferença absoluta ponderada |
| $f^* = 2p - 1$ | A fração de Kelly para uma aposta que paga o mesmo que arrisca | `let f = 2.0 * p - 1.0;` |
| $\frac{s+1}{n+2}$ | A regra de Laplace: um sucesso e uma falha imaginários | `(s + 1.0) / (n + 2.0)` |
| $W_{M/M/1} = \frac{\rho}{\mu - \lambda}$ | A espera média numa fila com um servidor cresce em curva | Não use a média para decidir a escala: simule o dia |
| $\mathrm{CITL} = \bar{p} - \bar{y}$ | O desvio de nível: quanto a previsão média erra a taxa observada | `media(p) - media(y)` |

**Uma advertência de leitura.** Em Rust, `f64` guarda cerca de 15 a 16 dígitos. Portanto $\approx$ nunca é
igualdade, e toda comparação de números medidos precisa de uma tolerância **declarada**, como a banda de 0,05
desvios padrão que a auditoria de fundo fixou para comparar duas rodadas do mesmo domínio. Uma banda escolhida
depois de ver o resultado conta como declaração nova e entra no contador de tentativas.

---

## Apêndice C. Guia de reprodução

### C.1 Os crates em Rust

| Comando | O que prova |
| --- | --- |
| `cargo test --workspace` | Que os tipos e os portões da arquitetura funcionam: medido nesta sessão, 25 testes passam (3 de auditoria, 10 do núcleo e 12 do hipergrafo) |
| `cargo run --release --bin portoes -- --horizonte todos --reproducao` | Que o portão roda duas vezes e dá o mesmo resultado: medido, "7 portões ok", com a raiz de Merkle `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81` |
| `cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23` | Que um portão **sabe reprovar**: com o defeito plantado, o comando sai com código 1 e diz qual portão caiu |
| `cargo run --release --bin labs -- --cap 19` | Que os números citados no Degrau 3 do capítulo 19 saem de código, com semente fixa. Vale trocar 19 por qualquer capítulo; `--todos` roda todos |
| `rustc -O crates/stochology-book/src/cap21.rs -o /tmp/cap21 && /tmp/cap21` | Que cada laboratório desta edição é autocontido: um arquivo, nenhuma dependência, o mesmo resultado |

Os laboratórios desta edição vivem direto no crate, em `crates/stochology-book/src/capNN.rs`. São módulos
testados (`cap01` a `cap04` trazem testes no arquivo) e **todos os 21** são servidos pelo binário `labs`. As
cópias intermediárias foram apagadas depois da integração, porque arquivo de trabalho não fica no repositório.
Nenhum laboratório usa relógio do sistema nem número aleatório sem semente, e é por isso que a conferência é
possível.

### C.2 O registro empírico (o código Python foi aposentado)

| Pasta | O que reproduz | Comando principal |
| --- | --- | --- |
| `h0-lab0/` | O posto de saúde simulado (Capítulo 16) | `python3 portoes.py` |
| `h0-real/` | A central de chamadas do banco (Capítulo 16) | `python3 portoes.py` |
| `h1/` | O H1 simulado, com a banca (Capítulo 17) | `python3 rodar.py` e `python3 portoes.py --reproducao` |
| `h1-real/` | Dow Jones, LaLonde e os incêndios dinamarqueses (Capítulos 17 e 18) | `python3 rodar.py` e `python3 portoes.py --reproducao` |
| `h2/` a `h2g/` | As sete rodadas do ICP (Capítulo 19) | `python3 rodar.py` e `python3 portoes.py --reproducao` em cada pasta |
| `h3/` | Os seis fluxos e as revisões H3b a H3d (Capítulo 20) | `python3 rodar.py` e `python3 portoes.py --reproducao` |
| `governanca/` | O registro das assinaturas dos 21 pareceres (Capítulos 15 e 21) | Leitura |
| `fundo/` | A auditoria de fundo e as extensões (Capítulo 21) | Leitura; `rodar.sh` em p02 e p05, script próprio nas demais; o p12 só tem protocolo e formulários |

Os comandos que amarravam o registro, copiados do plano da pesquisa, hoje exigem o código de volta:

```bash
git checkout 1bfec905 -- h1/          # o código Python das frentes vive no histórico
cd h1 && python3 rodar.py && python3 portoes.py --reproducao && python3 grafo.py
cd h3 && python3 rodar.py && python3 grafo.py && python3 figura.py && python3 portoes.py --reproducao
cd fundo/p07-icp/codigo && python3 nulo_validacao.py          # a taxa nula da genética, 0,5 s
```

**O código Python de simulação foi aposentado em 2026-10-16** (576 arquivos, cerca de 2,9 MB), porque o aparato
de engenharia do projeto passou a ser o workspace Rust. Na árvore fica o registro. São os dados e as somas, os
`README.md` de cada frente com os números publicados, as declarações, os pareceres e as saídas conferidas. O
manifesto `python-removido.txt` traz o hash de blob de cada arquivo removido, e a declaração completa está em
`DECLARACAO_REMOVIDO_PYTHON.md`.

Naquele dia saíram os artefatos que não sustentam resultado central. São as extensões de escala, saúde e evento raro (pontos 9 e 10 da auditoria de fundo) e a edição anterior do livro. Os números dessas extensões continuam nos Capítulos 15 e 21, com a nota de aposentadoria na tabela. Depois saiu o andaime de medição em Python inteiro. Também saíram as frentes `h0` a `h3`, com os seus READMEs, e os dados que só elas usavam. Os números ficaram em `REGISTRO_EMPIRICO.md`, com o commit `1bfec905` como âncora (`DECLARACAO_CONSOLIDACAO_RUST.md`).

Três conferências que continuam rodando sem Python, e que custam segundos:

```bash
python3 book/conferir_labs.py       # confere cada bloco de saída contra o instrumento
python3 book/gerar.py --check       # confere a rampa de três degraus dos 21 capítulos
cargo test --workspace               # os crates do livro e da arquitetura
```

### C.3 O ambiente de registro, e como ler uma divergência

O registro do H3 é: **Python 3.13.7**, **numpy 2.5.1** (obtido com `PYTHONNOUSERSITE=1 python3`), pandas 3.0.5,
scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0, duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1 e hypothesis 6.165.7, com
`export OMP_NUM_THREADS=1` e no máximo dois processos em paralelo. Duas dependências faltam nesta máquina, e o
arquivo de ambiente diz como instalá-las. A primeira é `statsmodels`, que só o gabarito do ponto 1 usa, e a segunda é `pyreadr`, que só
um passo do ponto 8 usa.

O caso do numpy mostra por que o ambiente faz parte do número. O efeito estimado do laço performativo estava
publicado como 0,5632 com 2.375 choques, e o código idêntico, rodado com a mesma semente, dava 0,5529 e 2.395
choques. Não havia duas versões do código. Havia dois numpys no interpretador, e o caminho padrão importava o
2.4.6 antes do 2.5.1. O ponto da divergência ficou isolado. Nas duas versões, o gerador partia do mesmo estado,
e um sorteio binomial de 248 tentativas com probabilidade 0,6 devolve 171 em uma e 150 na outra, porque a 2.5
corrigiu a série de Stirling do teste de aceitação do BTPE. Se você encontrar uma divergência assim, o caminho é
sempre este. **Primeiro o ambiente, depois o código**, porque nenhum número publicado é trocado sem declaração escrita
antes de rodar.

---

## Apêndice D. Bibliografia comentada para jovens cientistas

### D.0 Como esta lista foi conferida

Esta bibliografia segue três regras, e as três são declarações.

**1. Todo arquivo foi baixado para consulta.** O autor baixou os arquivos dos links desta lista em
2026-10-16, para conferência direta. Os cinco documentos do projeto foram **integrados** ao repositório
(arquitetura, plano escalável, fundamentação e o artigo) e as cópias soltas foram apagadas, como manda a regra
dos arquivos novos. O material integrado está em `PLAN.md` §9.13, em `artigo/` e neste apêndice. Os arquivos de
terceiros não são versionados, por direito autoral. Deles ficam a citação, o DOI e o número que o livro usa.

**2. Cada item tem um estado de conferência.**

| Marca | Significa |
| --- | --- |
| **[conferido]** | o número citado no livro foi conferido contra o próprio artigo, contra o DOI ou contra a página da editora |
| **[arquivo]** | há arquivo baixado para consulta, ainda sem conferência item a item contra o texto do livro |
| **[histórico]** | material aposentado, que fica só como registro de onde um número veio |

**O estado de cada linha está na linha.** Depois da conferência de 2026-10-16, todas as linhas de D.2 a D.4 levam
a marca **[conferido]**, e a fonte de cada conferência está em `referencias/CONFERENCIA.md`.

**3. Todo experimento deste livro é Rust.** O Python aparece apenas como registro histórico aposentado
(veja o Apêndice C). Nenhum número do Degrau 3 depende de Python, e nenhum depende do curso.

**Aviso: o curso foi aposentado.** O curso *Pensamento Crítico e Modelos Estocásticos*, de onde vinham
vários números de simulação e boa parte da numeração interna (M0 a M15, ME1 a ME24, MB1 a MB8), foi
**obsoleto em 2026-10-16**. As menções a "roteiro do curso" e a "material do curso" que ainda aparecem nos
capítulos são **históricas**. Elas dizem de onde um número veio, e não são material vigente. Vale o que este
repositório guarda. É o registro empírico conferido, as declarações e os laboratórios em Rust.

### D.1 Documentos do projeto

- [Arquitetura](https://docs.google.com/document/d/1l0YQHrV_UrKisAjQcz2BqT9KpwhcT0s0WVoXm_muxtE/edit),
  [PLAN.md](https://docs.google.com/document/d/1hXvDuJyckooL1_qx9gspkSkBqgjyhAUVQvQ3F-Crinc/edit),
  [AGENTS.md](https://docs.google.com/document/d/1hniAsjLqgNVGqR1wZ1845GVkS5lZTEjhlH7MaI3U2Xk/edit) e
  [Estocologia: Fundamentação, Três Camadas e Inferência sob Incerteza](https://docs.google.com/document/d/1L5ittj7-RiSacEvNrkR4IS6yrQUyhusFpcDIWazINLI/edit)
  são o desenho da máquina, o plano diretor e a fundamentação.
- [Estocologia: Velhas Fronteiras, Novos Horizontes](https://claude.ai/code/artifact/69dbc377-114b-4cb7-a3c1-322387abc674)
  é o artigo, com a lista completa de referências.
- **[histórico]** O curso *Pensamento Crítico e Modelos Estocásticos*, **aposentado em 2026-10-16**. Fica citado porque vários números do livro vieram das suas simulações, com a fonte agora declarada nos capítulos; nada do livro depende dele.
- `referencias/CONFERENCIA.md`: a conferência das referências, feita em 15 de outubro de 2026 contra a API do
  Crossref, as páginas das editoras, o PubMed e o RePEc. É o modelo do que fazer com a sua bibliografia. São
  25 referências conferidas uma a uma, uma correção aplicada (o artigo é de **Meyer**, Viscusi e Durbin, e não de
  Viscusi e Durbin), uma revista trocada (Baltagi e Levin saiu em *Structural Change and Economic Dynamics*), e a
  nota de que SHELF é um protocolo, não o livro de O'Hagan e colegas.

### D.2 Clássicos para começar

| Referência | Por que ler agora |
| --- | --- |
| **[conferido]** Bernoulli, *Ars Conjectandi* (1713) | O livro que transformou "provavelmente" em conta. A regra de Laplace deste livro é filha dele |
| **[conferido]** Hume, *Enquiry* (1748; o título *Enquiry* é da edição de 1758) | A pergunta que nenhuma estatística resolve: por que o futuro se parece com o passado? |
| **[conferido]** Knight, *Risk, Uncertainty and Profit* (1921) | A distinção entre risco mensurável e incerteza que não se mede, no Capítulo 4 |
| **[conferido]** Savage, *The Foundations of Statistics* (1954) e Savage, *JASA* 46:55–67 (1951) | De onde vem a ideia de regra própria: apostar contra a própria crença é perder dinheiro |
| **[conferido]** Wald, *Statistical Decision Functions* (1950) e Chernoff, *Econometrica* 22(4) (1954) | Decidir com perda, e não só com probabilidade: a base do Capítulo 11 |
| **[conferido]** Royall, *Statistical Evidence: A Likelihood Paradigm* (1997) | A verossimilhança como medida de evidência, e não como resposta final |
| **[conferido]** Tversky e Kahneman, "Judgment under Uncertainty" (1974) | O catálogo dos atalhos mentais que o Capítulo 2 organiza por modelo de acaso ignorado |
| **[conferido]** Mayo, *Error and the Growth of Experimental Knowledge* (1996) | A severidade: um teste só vale o que vale o seu poder de errar |
| **[conferido]** Pearl, *Causality* (2000) | A escada de ver, agir e imaginar, e a porta dos fundos |
| **[conferido]** Gigerenzer, *Calculated Risks* (2002) | Como escrever risco para quem precisa decidir: frequências naturais, sem jargão |
| **[conferido]** Mandelbrot, *Fractals and Scaling in Finance* (1997) | Caudas selvagens contadas por quem as viu primeiro nos preços |

### D.3 Métodos e dados usados na pesquisa

| Referência | Onde entra |
| --- | --- |
| Shen, H.; Huang, J. Z. *Annals of Applied Statistics* 2(2):601–623, 2008. DOI: 10.1214/08-AOAS164 | A central de chamadas do banco, em 2003 (Capítulos 16 e 20) |
| LaLonde, R. J. *American Economic Review* 76(4):604–620, 1986 (JSTOR 1806062) | O experimento de treino profissional do NSW e a diferença crua que engana (Capítulo 17) |
| Dehejia, R. H.; Wahba, S. *JASA* 94(448):1053–1062, 1999. DOI: 10.1080/01621459.1999.10473858 | A resposta ao LaLonde: o pareamento acerta o número, e por quê isso ainda não basta |
| McNeil, A. J. *ASTIN Bulletin* 27(1):117–137, 1997. DOI: 10.2143/AST.27.1.563210 | Os incêndios dinamarqueses e a cauda selvagem (Capítulo 18) |
| Embrechts, P.; Klüppelberg, C.; Mikosch, T. *Modelling Extremal Events for Insurance and Finance*, Springer, 1997. DOI: 10.1007/978-3-642-33483-2 | A teoria da cauda que o estimador de Hill usa |
| Peters, J.; Bühlmann, P.; Meinshausen, N. *JRSS B* 78(5):947–1012, 2016. DOI: 10.1111/rssb.12167 | O ICP, o método das sete rodadas (Capítulo 19). Leia a discussão: as objeções estão lá |
| Heinze-Deml, C.; Peters, J.; Meinshausen, N. *Journal of Causal Inference* 6(2):20170016, 2018. DOI: 10.1515/jci-2017-0016 | O ICP não linear, que inspirou a segunda régua do H2c |
| Gamella, J. L.; Peters, J.; Bühlmann, P. *Nature Machine Intelligence* 7:107–118, 2025. DOI: 10.1038/s42256-024-00964-x | As câmaras físicas, o túnel de luz e o túnel de vento |
| Kemmeren, P. et al. *Cell* 157(3):740–752, 2014. DOI: 10.1016/j.cell.2014.02.054 | As deleções de genes em levedura e a réplica de 40 genes |
| Meinshausen, N.; Hauser, A.; Mooij, J. M.; Peters, J.; Versteeg, P.; Bühlmann, P. *PNAS* 113(27):7361–7368, 2016. DOI: 10.1073/pnas.1510493113 | A validação por intervenção que decide a réplica da genética |
| Johannesen, L.; Vicente, J.; Mason, J. W. et al. *Clinical Pharmacology & Therapeutics* 96(5):549–558, 2014. DOI: 10.1038/clpt.2014.155 | O ensaio do FDA com dofetilida, quinidina, ranolazina e verapamil |
| Vicente, J. et al. *Journal of the American Heart Association* 4(4):e001615, 2015. DOI: 10.1161/JAHA.114.001615 | A morfologia da onda T no mesmo ensaio |
| Sachs, K.; Perez, O.; Pe'er, D.; Lauffenburger, D. A.; Nolan, G. P. *Science* 308(5721):523–529, 2005. DOI: 10.1126/science.1105809 | A rede de sinalização que o H2 podou no primeiro filtro |
| Krueger, A. B. *Quarterly Journal of Economics* 114(2):497–532, 1999. DOI: 10.1162/003355399556052 | O STAR: turmas pequenas sorteadas no Tennessee |
| Finn, J. D.; Achilles, C. M. *American Educational Research Journal* 27(3):557–577, 1990. DOI: 10.3102/00028312027003557 | O desenho do STAR, contado por quem o aplicou |
| Card, D.; Krueger, A. B. *American Economic Review* 84(4):772–793, 1994 (JSTOR 2118030) | Salário mínimo e emprego, o modelo de diferenças em diferenças |
| Meyer, B. D.; Viscusi, W. K.; Durbin, D. L. *American Economic Review* 85(3):322–340, 1995 (JSTOR 2118177) | O teto do benefício por acidente de trabalho, no Kentucky e em Michigan |
| Thornton, R. L. *American Economic Review* 98(5):1829–1863, 2008. DOI: 10.1257/aer.98.5.1829 | O sorteio do resultado do teste de HIV, um ambiente que é intervenção |
| Newhouse, J. P.; Insurance Experiment Group. *Free for All?*, Harvard University Press, 1993 | O experimento de seguro da RAND, cuja coparticipação satura o teste |
| Baltagi, B. H.; Levin, D. *Structural Change and Economic Dynamics* 3(2):321–335, 1992. DOI: 10.1016/0954-349X(92)90010-4 | O imposto sobre a cerveja, podado no terceiro filtro |
| Stock, J. H.; Watson, M. W. *Introduction to Econometrics*, 4. ed., Pearson, 2019, cap. 12 | Variáveis instrumentais, o exemplo dos cigarros |
| Gibbs, I.; Candès, E. *NeurIPS 34*:1660–1672, 2021 (arXiv:2106.00170) | A inferência conforme adaptativa, que dá os quantis do H3 |
| O'Hagan, A. et al. *Uncertain Judgements: Eliciting Experts' Probabilities*, Wiley, 2006. DOI: 10.1002/0470033312 | A elicitação que ficou em aberto no Capítulo 21. Nota: SHELF é um protocolo, não este livro |

### D.4 Tipologias de erro, incerteza e calibração

| Referência | Onde entra |
| --- | --- |
| **[conferido]** Rubin, D. B. *Biometrika* 63(3):581–592, 1976. DOI: 10.1093/biomet/63.3.581 | MCAR, MAR e MNAR: por que o dado falta (Capítulo 6) |
| **[conferido]** Kimball, A. W. *JASA* 52(278):133–142, 1957. DOI: 10.1080/01621459.1957.10501374 | O erro de tipo III: responder à pergunta errada |
| **[conferido]** Gelman, A.; Tuerlinckx, F. *Computational Statistics* 15(3):373–390, 2000. DOI: 10.1007/s001800000040 | Os erros de tipo S (sinal) e tipo M (magnitude) |
| **[conferido]** Gelman, A.; Carlin, J. *Perspectives on Psychological Science* 9(6):641–651, 2014. DOI: 10.1177/1745691614551642 | Como o poder pequeno produz exageros de tamanho |
| **[conferido]** Walker, W. E. et al. *Integrated Assessment* 4(1):5–17, 2003. DOI: 10.1076/iaij.4.1.5.16466 | A tipologia da incerteza, em linguagem de decisão pública |
| **[conferido]** Stirling, A. *Nature* 468(7327):1029–1031, 2010. DOI: 10.1038/4681029a | Incerteza, ambiguidade e ignorância, e por que simplificar custa |
| **[conferido]** Moreno-Torres, J. G. et al. *Pattern Recognition* 45(1):521–530, 2012. DOI: 10.1016/j.patcog.2011.06.019 | A visão unificada das mudanças de distribuição, no Capítulo 4 |
| **[conferido]** Ellsberg, D. *Quarterly Journal of Economics* 75(4):643–669, 1961. DOI: 10.2307/1884324 | A aversão à ambiguidade, na matriz de Stirling |
| **[conferido]** Sweeney, L. *Simple Demographics Often Identify People Uniquely*, Carnegie Mellon, 2000 | Quase-identificadores e o k-anonimato do H3 |
| **[conferido]** Baig, S. S. et al. *European Journal of Human Genetics* 24(10):1396–1402, 2016. DOI: 10.1038/ejhg.2016.36 | O teste preditivo da doença de Huntington, e o que "17,4%" quer dizer |
| **[conferido]** Makin, T. R.; Orban de Xivry, J.-J. *eLife* 8:e48175, 2019. DOI: 10.7554/eLife.48175 | Os dez erros estatísticos mais comuns, usados no piloto cego |
| **[conferido]** van Dantzig, D. *Econometrica* 24(3):276–287, 1956 | O problema do dique: quanto gastar contra uma enchente improvável (Capítulo 11) |
| **[conferido]** Saerens, M.; Latinne, P.; Decaestecker, C. *Neural Computation* 14:21–41, 2002 | Ajustar as saídas de um classificador a novas taxas-base |
| **[conferido]** Lipton, Z.; Wang, Y.-X.; Smola, A. *ICML* 80:3122–3130, 2018 | O teste de deriva de rótulo que não avisou o desvio de 2013 (Capítulo 20) |
| **[conferido]** Janssen, K. J. M. et al. *Canadian Journal of Anesthesia* 56(3):194–201, 2009 | Ajustar um modelo de risco às circunstâncias locais |
| **[conferido]** Van Calster, B. et al. *Journal of Clinical Epidemiology* 74:167–176, 2016 | A hierarquia da calibração, de utopia a dado empírico |

### D.5 Como usar esta lista

Leia na ordem em que a curiosidade pedir, e desconfie de quem apresenta qualquer um destes trabalhos como
resposta final. Três conselhos práticos. Primeiro, leia os dados antes do método, porque quase todo erro de
leitura nasce de não perguntar de onde veio a tabela. Segundo, quando um artigo cita um número, procure o
número no código ou no apêndice, e não no resumo. Terceiro, guarde a versão do ambiente junto do número que
reproduziu; este livro tem um capítulo inteiro que só existe porque alguém fez isso.

### D.6 O que já foi conferido e o que falta conferir

A conferência das referências do projeto está em `referencias/CONFERENCIA.md`. São 25 referências checadas
uma a uma contra a API do Crossref, as páginas das editoras, o PubMed e o RePEc, com uma correção aplicada (o
artigo do teto do benefício é de **Meyer**, Viscusi e Durbin, e não de Viscusi e Durbin) e uma revista
trocada (Baltagi e Levin saiu em *Structural Change and Economic Dynamics*).

**A conferência terminou em 2026-10-16.** As 50 linhas de D.2 a D.4 foram conferidas item a item contra a API do
Crossref, as páginas das editoras, o PubMed, os anais do PMLR e catálogos de biblioteca: as 23 de D.3 na primeira
passada (2026-10-15) e as 27 de D.2 e D.4 na segunda. Cada linha deste apêndice traz a marca **[conferido]** e cada
conferência está na sua linha de `referencias/CONFERENCIA.md`, com a fonte ao lado.

O que apareceu: **nenhuma divergência de conteúdo** nas 27 linhas novas — autores, anos, títulos, periódicos e
páginas conferem —, **onze complementos** (os DOIs que faltavam em dez linhas e o número do *working paper* do
Sweeney) e **duas ressalvas de fonte declaradas**: a página final do Chernoff de 1954 e a do van Dantzig de 1956
não saíram nas fontes consultadas, e o ano do Sweeney (2000) vem do registro do CMU.

A regra 1 deste apêndice continua valendo: todo arquivo foi baixado para consulta, e o que o livro guarda de cada
um é a citação, o DOI e o número que ele usa. O que não entra no repositório são os arquivos de terceiros, por
direito autoral.
