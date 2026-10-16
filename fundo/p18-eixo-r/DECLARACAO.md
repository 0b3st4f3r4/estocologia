# Declaração do ponto 18: o eixo R nos 21 capítulos

**Congelada em 2026-10-16T23:19:00Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `book/eixo-r.tsv` | `5e0c7db5d931b70610fb0cd4c99718ae9f2d830f502baaef140402303c3ea04d` | a tabela por capítulo: o caso declarado e a frase editorial |
| `DECLARACAO.md` (esta) | — | as quatro declarações; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `3e705914f6eef17dc6f2e532fe12b549a15ccebe7288dc57a39746ac5843c4fe` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — a forma do bloco, igual nos 21 capítulos

Cada capítulo ganha, no Degrau 3, o mesmo parágrafo curto e **uma linha** do instrumento do eixo:

> **O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
> grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):
>
> ```text
> capítulo 16 | perda de resolucao 46.24 min (demanda de 5 a 11) | o capítulo traz a tabela R0 a R3 do posto e do banco
> ```

A linha é **saída literal** do instrumento, e o conferidor de blocos do livro a cobra: `conferir_labs.py` passou de
27 para **48 blocos conferidos**, com zero divergente. Forma única, colocação única (antes dos exercícios), e nenhum
número escrito à mão no capítulo.

## Declaração 2 — o que o instrumento calcula e o que a tabela declara

A tabela `book/eixo-r.tsv` declara, por capítulo, **um caso** e **uma frase editorial**. O instrumento lê a tabela,
**calcula** a leitura do caso e monta a linha; a frase não carrega número nenhum. Os casos são cinco, e todos saem
de medição:

| caso | o que o instrumento calcula |
| --- | --- |
| `posto` | a perda de resolução na demanda que oscila de 5 a 11: **46,24 min** |
| `posto-leve` | a perda na oscilação de 7 a 9: **5,97 min** |
| `plano` | a perda na demanda plana: **0,00 min** |
| `previsoes` | o limiar de oscilação na amplitude **2,5**, com o plano calibrado |
| `teoria` | nenhum número: o capítulo não tem duas resoluções para comparar, e a linha diz isso |

Cinco capítulos levam número (5, 8, 10, 16, 18 e 20, com `posto`, `posto-leve` e `plano` repetidos) e o capítulo 21
leva o limiar. Os outros catorze levam a declaração honesta de que a perda **não é medida ali** — porque não há, no
exemplo daquele capítulo, a mesma decisão tomada em dois grãos. É a diferença entre preencher uma tabela e medir.

## Declaração 3 — a leitura editorial, por capítulo

Onde o capítulo decide escala, a frase nomeia o grão: o posto que oscila (5), a tolerância que depende do n do
cofre (8), o arrasto e a ruína medidos por dia (10), a tabela R0 a R3 do posto e do banco (16), a janela dos
incêndios (18), o cofre ao vivo com dois grãos de teste (20). Onde o capítulo não decide escala, a frase diz o que
ele faz e como o eixo entra: as camadas (2), os três tipos de incerteza (4), a unidade independente como Φ2 (6 e 17),
a escada de causa (9), o trilema como subpergunta do R3 (11), o mapa do conhecimento (12), a banca (13), o contrato
que carrega o campo `resolucao` e o Φ5 (14), o cofre como grão novo (15) e a agregação por escola (19).

## Declaração 4 — o limite e o K

**O limite.** O eixo R entra no Degrau 3 de cada capítulo como uma linha, e não como um laboratório novo: o
laboratório do eixo continua sendo um só, o `labs --resolucao`, e a perda de resolução só é medida onde existem as
duas resoluções. Os sete casos documentados do eixo continuam na tabela do documento (`book/horizontes-de-
resolucao-da-pergunta.md`), cada um com a sua fonte.

**O K.** A atribuição de um caso a cada um dos 21 capítulos é escolha de especificação, feita uma vez e declarada
aqui: **K = 1**. Mudar um caso depois de ver a linha de um capítulo é declaração nova, com hash próprio, e conta no
K.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `labs --resolucao` | a perda dos três casos, as previsões 3 e 5 e as **21 linhas** do eixo, uma por capítulo, com a leitura calculada pelo caso declarado (0) |
| `python3 book/conferir_labs.py` | **48 blocos citados, 0 divergentes** — as 21 linhas novas conferem em `labs --resolucao`, e o bloco do eixo é o único de cada capítulo que veio do instrumento do eixo (0) |
| `python3 book/gerar.py --check` | a rampa de três degraus dos 21 capítulos confere, com 5 903 linhas e 74 847 palavras (0) |
| `python3 book/conferir_livro.py` | partes, capítulos, cercas e ordem conferem (0) |
