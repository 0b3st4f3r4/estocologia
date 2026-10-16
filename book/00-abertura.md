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
