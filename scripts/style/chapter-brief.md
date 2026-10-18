# Brief de revisão de um capítulo — Estocologia

Você é revisor de língua e estilo de **um** capítulo do livro *Estocologia*. O texto já é formal. O defeito
que você conserta é a **repetição**. Formal, porém mais humano quer dizer: sintaxe variada, sujeito
explícito, maneirismo previsível cortado, a frase respirando. Não quer dizer coloquial, e não quer dizer
empolado.

Trabalhe em `/home/silvano-neto/Documents/estocologia`. Leia **antes de tocar em qualquer linha**:

- `scripts/style/style-sheet.md` — as decisões fixadas, os termos consagrados e os alvos.
- `scripts/style/repetition-index.md` — a contagem de antes, do capítulo inteiro e do livro.
- este brief.

## O que não se toca, em nenhum caso

1. Números, valores, unidades, datas, sementes, percentuais, intervalos. Sem arredondar, sem converter, sem
   trocar a vírgula decimal do português por ponto.
2. Código Rust, Python e shell dentro de cerca ` ``` ` ou ` ~~~ `, comentários e identificadores inclusos.
3. Os blocos ` ```text `: são a saída literal de `labs --chapter N`, conferida por `scripts/book/check_labs.py` linha
   a linha. Um caractere mexido ali quebra o capítulo.
4. Cabeçalhos e numeração: `## Capítulo N.`, `### Degrau 1/2/3`, `### N.M`, títulos de parte.
5. Os marcadores `**Objetivos.**`, `**Exercícios.**`, `**Resumo.**` e nomes de quadro, figura e tabela.
6. Tabelas, listas e a ordem das linhas. A prosa em volta muda; o esqueleto fica.
7. Termos técnicos consagrados, nomes próprios, siglas, nomes de arquivo e de crate, e as referências
   cruzadas ("o Capítulo 2", "a seção 9.7") — com o mesmo número.
8. Os estilemas declarados do autor: o parágrafo **Declaração de divergência**, a seção 9.7 e as frases
   curtas que fecham argumento de propósito.

**Não parafraseie o conteúdo.** Toda afirmação, todo número, toda conclusão continua igual. A passada mexe
em sintaxe, ritmo e vício de construção, e em nada mais. Não acrescente exemplo novo nem número novo; quando
o texto era abstrato, torne concreto o caso que o próprio capítulo já nomeia. Não reduza conteúdo para
baixar contagem: o capítulo tem piso de 700 palavras cobrado por `scripts/book/build.py --check`, e a reescrita
corta repetição, não informação.

## Alvos da passada

Meça o seu arquivo antes de começar:

```bash
cd /home/silvano-neto/Documents/estocologia && python3 scripts/style/varredura.py book/capNN.md
```

| Padrão | O que fazer | Alvo |
| --- | --- | --- |
| travessão apostivo ` — ` | vírgula se a informação é acessória; parêntese se é aparte; dois-pontos se explica o que vem antes; período novo se tem peso próprio. Conserva só onde a interrupção é de fato abrupta | ≤ 20% do valor de antes |
| `mesmo/mesma` | apaga. "o mesmo resultado" vira "esse resultado" ou "o resultado". Sobrevive só onde a identidade é o argumento, e aí a frase diz de que identidade se trata | ≤ 30% |
| `o mesmo` | idem, com o substantivo repetido | ≤ 20% |
| `o que` relativo | período curto vira frase direta; antecedente possuído vira `cujo/cuja`; onde o sujeito importa, o verbo entra no lugar do relativo. Nunca dois no mesmo período | ≤ 30% |
| `, não` de oposição retórica | a informação vem pelo lado positivo e o contraste se dissolve | ≤ 25%, e no máximo **1** no capítulo |
| ` — e` fechando frase | oração coordenada comum ou período novo | ≤ 25%, e no máximo **2** no capítulo |
| dois-pontos | fica o que abre lista, equação, definição formal ou quadro; o resto vira ponto seguido de período, com o sujeito de volta | ≤ 60% |
| `próprio/própria` | apaga, salvo identidade reflexiva real (`o próprio instrumento`) | ≤ 40% |
| `exatamente` | apaga, salvo onde o valor é o argumento | ≤ 40% |
| `não é X, é Y` | guarda só a correção que o argumento usa | ≤ 50% |

Também: média de período entre **18 e 28 palavras** na prosa de argumento; nenhum parágrafo abre com o
conectivo que abriu o anterior; nenhuma sequência de três períodos repete o mesmo esqueleto; a frase
telegráfica de remate (três a cinco palavras fechando seção) aparece no máximo uma vez por seção, nunca em
duas seções seguidas.

## Como editar

1. Leia o capítulo inteiro primeiro. Reescrever um parágrafo sem ter lido o seguinte desfaz o que o
   seguinte já dizia.
2. Edite **por parágrafo**, com a âncora visível. Maneirismo é fenômeno de parágrafo, não de linha solta.
3. Fora da cerca, fora da tabela, fora do cabeçalho, fora do marcador.
4. Rode a varredura de novo e compare padrão por padrão.
5. Escreva só no seu arquivo. Não mexa em outro capítulo, não rode `cargo`, não faça `git commit`.

## O que entregar na resposta final

1. A contagem **antes e depois** de cada padrão do índice, no seu arquivo.
2. De **cinco a dez pares antes-e-depois**, escolhidos entre os casos difíceis, não os mais fáceis, no
   formato: `antes: …` / `depois: …` / `o que mudou e por quê` (uma linha).
3. O que ficou intacto de propósito, e o motivo.
4. As pendências para o autor: linha cuja reescrita obrigaria a mexer em número, código, bloco ` ```text `,
   cabeçalho ou tabela — com o número da linha e a citação.

Não escreva que o texto "melhorou" ou "ficou mais fluido". Mostre a linha: o defeito que saiu, o que entrou
no lugar.
