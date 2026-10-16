# Folha de estilo — Estocologia (revisão de prosa)

Registro de decisões do revisor de língua e estilo. Vale sobre o gosto do momento: a consistência
entre capítulos manda mais do que a preferência do dia. Data da abertura: **2026-10-16**
(contexto de execução desta sessão, conforme AGENTS.md).

## Onde vive o livro

O livro está em `book/` (não em `livro/`): `book/00-abertura.md`, `book/cap01.md` a `book/cap21.md`,
`book/apendices.md`. A edição montada é `estocologia-livro-didatico.md`, escrita por `book/gerar.py`.
Os instrumentos que fecham a passada são `python3 book/gerar.py --check` e
`python3 book/conferir_labs.py`. A varredura de maneirismos é `python3 estilo/varredura.py`.

## Registro e tratamento do leitor

- Segunda pessoa `você`, português do Brasil, impessoal onde o agente é o processo e não a pessoa.
- Sem exclamação, sem elogio ao leitor, sem conversa, sem gíria, sem pergunta retórica de abertura.
- Formal, porém mais humano: sintaxe variada, sujeito explícito, exemplo concreto onde o texto ficou
  abstrato. **Não** é empolado.
- Proibido nesta revisão: locução no lugar de verbo simples (`faz uso de` por `usa`), empilhar
  advérbio em `-mente`, encadear `o qual`, abrir com `é importante destacar que` / `vale ressaltar que`,
  enfeitar coesão com `outrossim`, `destarte`, `por conseguinte`.
- Voz passiva fica onde o agente é irrelevante ou desconhecido (método, resultado de instrumento,
  decisão de terceiro). Em todo o resto, o sujeito volta a aparecer.

## Termos fixados (forma exata, e o que não se usa)

| Forma fixada | Sinônimos proibidos |
| --- | --- |
| incerteza knightiana | incerteza de Knight, incerteza radical |
| cauda pesada | cauda grossa, cauda longa |
| taxa-base | taxa base, taxa de base |
| margem de erro | faixa de erro |
| modelo nulo | hipótese nula simulada, contrafactual nulo |
| unidade independente | unidade de análise, observação independente |
| rampa de três degraus | tríade, escada didática |
| declaração de divergência | nota de divergência, ressalva |

Nomes próprios, siglas, nomes de arquivo, de crate e de comando ficam na forma exata do texto:
`stochology-core`, `labs --cap N`, `Phi5`, `RESULTADOS_RUST.md`, `A1`–`A8`, `G1`–`G7`, `H0`–`H3`, `R0`–`R3`.

## Decisões por padrão, com alvo numérico

Base: varredura de `book/` com as cercas de código fora da conta — **66.526 palavras**, contagem em
`estilo/indice-de-repeticao.md`. Alvo geral: redução de 60 a 80% de cada maneirismo. A meta é o texto
parar de se repetir, não o número chegar a zero.

| Padrão | Antes | Alvo | Regra de exceção |
| --- | --- | --- | --- |
| travessão apostivo ` — ` | 430 | ≤ 170 (≈ 1 por 400 palavras) | fica só onde a interrupção é de fato abrupta |
| `mesmo/mesma` | 302 | ≤ 90 | fica onde a identidade **é** o argumento, e aí a frase diz de que identidade se trata |
| `o mesmo` | 90 | ≤ 20 | idem, com o substantivo repetido em vez de `o mesmo` |
| `o que` relativo | 287 | ≤ 90 | nunca duas ocorrências no mesmo período, nem uma por período em parágrafo curto |
| `, não` de oposição retórica | 84 | ≤ 21 (máx. 1 por capítulo, 2 nos capítulos longos) | guardado para o contraste que o argumento de fato usa |
| ` — e` fechando frase | 76 | ≤ 25 (máx. 2 por capítulo) | vira oração coordenada comum ou período novo |
| dois-pontos | 1096 | ≤ 600 | fica o que abre lista, equação, definição formal ou quadro |
| `próprio/própria` | 100 | ≤ 40 | fica quando marca identidade reflexiva real (`o próprio instrumento`) |
| `exatamente` | 40 | ≤ 15 | fica onde o valor é o argumento |
| `não é X, é Y` | 9 | ≤ 4 | correção que o argumento usa de verdade |

Comprimento de período na prosa de argumento: média entre 18 e 28 palavras. Nenhum parágrafo abre com
o conectivo que abriu o anterior. Nenhuma sequência de três períodos repete o mesmo esqueleto.
Frase telegráfica de remate (três a cinco palavras fechando seção): no máximo uma por seção, nunca em
duas seções seguidas.

## A lista fechada: o que não se toca

Nenhum caractere disto muda em nenhuma passada. Se uma reescrita obrigaria a mexer aqui, a linha vira
pendência em `estilo/decisoes.md` e a decisão é do autor.

1. Números, valores, unidades, datas, sementes, percentuais e intervalos — sem arredondar, sem converter
   unidade, sem trocar a vírgula decimal do português por ponto.
2. Código Rust, Python e shell, dentro de cerca ` ``` ` ou ` ~~~ `, comentários e identificadores inclusos.
3. Os blocos ` ```text `, que `book/conferir_labs.py` confere linha a linha contra a saída real de
   `labs --cap N`, `labs --banco`, `labs --resolucao`, `labs --s1`, `labs --fila`.
4. Cabeçalhos, títulos de seção, `## Capítulo N.`, `### Degrau 1/2/3`, `### N.M` e títulos de parte.
5. Os marcadores `**Objetivos.**`, `**Exercícios.**`, `**Resumo.**`, e nomes de quadro, figura e tabela.
6. Tabelas, listas e a estrutura de parágrafos que elas organizam; a ordem das linhas.
7. Termos técnicos consagrados, nomes próprios, citações, siglas, e as referências cruzadas
   ("o Capítulo 2", "a seção 9.7") — com o mesmo número.
8. Os estilemas declarados do autor: o parágrafo **Declaração de divergência**, a seção 9.7 e as frases
   curtas que fecham argumento de propósito. Onde o autor já declarou a regra, o revisor não a aplica
   contra ele.

Piso de 700 palavras por capítulo, cobrado por `book/gerar.py --check`. A reescrita corta repetição, não
conteúdo.

## O que ficou intacto de propósito

- Todas as oito linhas da lista fechada acima, por contrato.
- Os 24 blocos ` ```text `, conferidos contra o instrumento: `0` divergentes antes da passada.
- Os valores da rampa, as declarações de divergência e os nomes de instrumento, porque são o contrato
  auditável do livro e não prosa.
