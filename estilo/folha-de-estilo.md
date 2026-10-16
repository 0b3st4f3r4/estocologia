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
| travessão apostivo ` — ` | 430 | ≤ 170 (≈ 1 por 400 palavras) | fica só onde a interrupção é de fato abrupta. Piso estrutural: 3 por capítulo, nos cabeçalhos `### Degrau N — …`, mais o dos estilemas declarados |
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

## Passadas registradas

### `book/cap05.md` — 2026-10-16

Prosa fora das cercas: 2069 → 2083 palavras. Contagem depois: travessão apostivo 10 → 4 (das quais 3
são cabeçalhos `### Degrau N — …`, fora da passada, e 1 fecha a frase de propósito: "sobra uma diferença
— o pedágio"); `travessao_fecha_com_e` 1 → 0; `mesmo/mesma` 13 → 1; `o mesmo` 2 → 0; `o que` relativo
5 → 4; `, não` 1 → 1 (a correção "A variabilidade, não a média, decide" é o contraste que o argumento
usa); dois-pontos 29 → 18; `exatamente` 1 → 1 ("o posto é exatamente esse caso", onde o valor é o
argumento). `python3 estilo/conferir-passada.py book/cap05.md` fecha com `com regressao: 0`.

Decisões desta passada, para valerem nos capítulos seguintes:

- Definição com aposto explicativo curto vira parêntese: `Numa função **convexa** (boca para cima, como
  $x^2$),…`. Só o travessão de interrupção abrupta fica.
- Comparação quantitativa dentro do mesmo cálculo carrega a conjunção e não o travessão: "29,3 minutos,
  quase o triplo dos 10 minutos da alegação"; "prevê 0,185, quase igual".
- Definição nominal perde o travessão para o `**negrito**` fazer a marcação: "sobra uma diferença — o
  pedágio" fica; "A fórmula se lê como um preço — **curvatura vezes dispersão**" não ficou, porque o
  alvo de travessão do capítulo é apertado e a vírgula dá conta.
- Identidade que **é** o argumento conserva o adjetivo e diz de que identidade se trata: em `cap05` só
  ficou uma ocorrência, "a média é a mesma" no `**Resumo.**` depois de o corpo do capítulo provar a
  igualdade da demanda nos dois mundos.
- `o que` relativo cede ao verbo direto ("O nulo mede o que a margem deve prever" já é verbo; "vê o que
  a planilha esconde" fica porque o contraste é entre ver e esconder, não entre relativos).

### `book/cap06.md` — 2026-10-16

Prosa fora das cercas: 2311 → 2295 palavras. Contagem depois: travessão apostivo 19 → 7 (dos sete, um é
célula de tabela no quadro do Rubin, um fecha a frase de propósito — "o erro-padrão de sempre" — e cinco
são prosa convertida em vírgula, dois-pontos ou período); `travessao_fecha_com_e` 3 → 1; `mesmo/mesma`
21 → 7 (seis fora do `**Declaração de divergência**`, que fica intacto); `o mesmo` 9 → 4, todas em
igualdade numérica ("vale o mesmo que 81 observações"); `o que` relativo 10 → 7 (três em cabeçalho de
tabela, um em exercício, três em exercício ou `**Resumo.**`); `, não` 8 → 2, as duas dentro de tabela;
dois-pontos 30 → 16; `próprio` 2 → 1; `não é X, é Y` 1 → 1; `exatamente` 0 → 0.
`python3 estilo/conferir-passada.py book/cap06.md` fecha com `com regressao: 0`.

Decisões desta passada, para valerem nos capítulos seguintes:

- Oposição retórica cabe dentro de célula de tabela sem `, não`: "Analisar os casos completos perde
  precisão — nenhum viés entra". O travessão existe para não gastar o par travessão-ponto na única linha
  da tabela que ainda trazia a vírgula.
- `mesmo` que expressa igualdade numérica é argumento, não intensificador: "2,65 milhões de chamadas
  valem o mesmo que **81** observações" fica; "na mesma proporção em que votam" vira "na proporção do
  voto de cada um".
- `o mesmo` que introduz uma unidade já nomeada vira artigo definido: "os pacientes de um mesmo dia"
  vira "os pacientes de um dia"; onde o capítulo compara a própria unidade, a forma fica ("As chamadas
  de um mesmo dia, porém, compartilham a demanda").

## O que ficou intacto de propósito

- Todas as oito linhas da lista fechada acima, por contrato.
- Os 24 blocos ` ```text `, conferidos contra o instrumento: `0` divergentes antes da passada.
- Os valores da rampa, as declarações de divergência e os nomes de instrumento, porque são o contrato
  auditável do livro e não prosa.

## Exceções registradas por capítulo

- **`book/cap19.md` (2026-10-16).** O capítulo tinha 39 travessões apostivos, o maior valor do livro, com
  alvo de ≤8: fechou em 6, dos quais 3 são títulos de degrau, 1 é célula de tabela, 1 é fecho de
  exercício e 1 é prosa. `próprio` ficou em 8, contra o alvo de ≤4, porque as cinco ocorrências de prosa
  são identidade reflexiva real (o eletrocardiograma que mede, o teste que se calibra, o gene que se
  deleta, o grafo causal, a função principal do crate) e as três restantes estão em tabela. `virgula_nao`
  ficou em 2: uma é qualificação (`não vazio`) e a outra é aposto, não oposição retórica. A decisão sobre
  as três, com linha e citação, está em `estilo/decisoes.md`.

## Decisões de contorno tomadas durante a passada

1. **Prosa dentro de item de lista é prosa.** Item de `**Exercícios.**`, de `**Resumo.**` ou de lista
   comum pode ser reescrito desde que o marcador, a ordem, a quantidade e a afirmação continuem iguais.
   Célula de tabela segue congelada, porque a coluna manda no comprimento. Isso vale para as duas
   passadas.
2. **Piso do travessão apostivo.** Os 63 travessões dos cabeçalhos `### Degrau 1/2/3 — …` (3 por
   capítulo) mais os dos parágrafos de **Declaração de divergência** são piso, não folga: o alvo de
   densidade global (≤170) já os desconta.
3. **Dois-pontos: a conta que vale é a de prosa.** `python3 estilo/dois_pontos.py` separa prosa, tabela e
   cerca. O alvo de 60% de corte incide sobre a coluna de prosa; o piso de cada arquivo é o número de
   dois-pontos que abrem lista, definição, equação, citação, quadro ou cerca.
4. **Fala citada e pergunta de exercício** ficam na forma do autor: o `o que` interrogativo e o `, não`
   dentro de citação direta não entram na reescrita.

## Rulings da terceira rodada

1. **Rótulo em negrito que abre parágrafo.** `**Monotonicidade:** com recurso total igual…` vira
   `**Monotonicidade.** Com recurso total igual…`, que é o molde dos marcadores do livro
   (`**Objetivos.**`, `**Resumo.**`) e o que o Capítulo 4 já fazia. Aplicado em
   `book/horizontes-de-resolucao-da-pergunta.md` (nove rótulos), `book/README.md` e `book/cap02.md`.
   Nome de seção que funciona como identificador de quadro continuou em pendência.
2. **Dois-pontos que não é pontuação de prosa.** Endereço (`https://…`), par arquivo:linha
   (`camada.rs:52`), referência com volume, número e páginas (`157(3):740–752`), hora e par de valor não
   entram na conta de prosa nem na reescrita. Em `book/apendices.md` isso é a maior parte do resíduo.
3. **Duas passadas por capítulo.** A primeira reescreve a prosa inteira; a segunda trata só os dois-pontos
   de prosa que a primeira deixou passar. O alvo global do dois-pontos é 60% de corte sobre a coluna de
   prosa, e o piso de cada arquivo é o número de colons que abrem lista, definição, equação, citação,
   quadro ou cerca.
4. **`o que` e `próprio` em célula de tabela, em título, em cabeçalho de bloco em negrito e em citação**
   ficam: a varredura conta, a reescrita não toca. É o que fixa o piso de vários capítulos.
5. **`mesmo` concessivo** (`pois mesmo o mundo que…`) e **`mesmo` idiomático de comparação**
   (`valem o mesmo que X`) não são intensificadores: viraram `até` e `equivalem a`, e o padrão caiu junto.
