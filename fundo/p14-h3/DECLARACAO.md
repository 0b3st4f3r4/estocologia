# Declaração do ponto 14 (a cadeia do H3): o que se refaz, o que se re-deriva e o que só resta declarar

**Congelada em 2026-10-16T21:57:54Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `registro.tsv` | `b4774e1b19745acc84284c90e8855b1dab0687b996b41b4be8d40fb98fef0a36` | as quatro linhas do H3, com a classe, a marca e o critério declarado do laço |
| `DECLARACAO.md` (esta) | — | este documento; o hash próprio sai em `DECLARACAO.sha256`, calculado depois da gravação |
| `README.md` | `b4dbd57a51e6e3af36b58be3c0571dbd9510437dde336ed2ed5e5d228d036502` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — as três classes da cadeia

O instrumento `labs --h3` percorre as quatro linhas do H3 e diz, para cada uma, o que é possível fazer com ela.
Nenhuma linha pode ser declarada de uma classe que o dado não sustenta:

| classe | o que exige | o que o instrumento faz |
| --- | --- | --- |
| **medida** | o dado bruto está na árvore | mede de novo |
| **derivada** | o número está num arquivo versionado da árvore | refaz a conta e o veredito, e confere contra o que o registro traz |
| **registro** | o dado bruto não está na árvore | declara o que se mediu e por que não se refaz |

O que tirou o dado bruto da árvore está declarado em `DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`: o andaime
`h3/` saiu em 2026-10-16 pelo critério de significância, e com ele os diários dos fluxos e o grafo do H3. O dado
bruto do laço performativo (AnonymousBank, 1999) nunca esteve versionado — a frente do ponto 6 guarda só o
`resultados.json` e o `posthoc.json`.

## Declaração 2 — a marca nova **V** no registro empírico

A legenda de `REGISTRO_EMPIRICO.md` tinha três marcas: **R** (o instrumento Rust mede de novo), **D** (divergência
declarada no capítulo, com os dois valores) e **P** (só registro). Esta declaração acrescenta a quarta:

**V** — o veredito é re-derivado em Rust a partir do registro versionado; a medida continua sem instrumento.

O motivo é a honestidade da própria legenda. Chamar a linha do laço de **R** seria mentira: o instrumento não mede
de novo o efeito de desistir sobre a nova chamada, porque o dado bruto não está aqui. Deixá-la em **P** esconderia
o que passou a existir: o número que o livro cita, e o veredito que o livro emite, são agora recalculados de um
arquivo versionado, com a conferência entre a conta e o registro. Não é mudança de critério — nenhum critério de
decisão do programa muda —, e por isso **não entra no K**.

## Declaração 3 — o que a re-derivação devolveu

~~~bash
cargo run --release --bin labs -- --h3
~~~

| o que | valor re-derivado | o que o registro trazia |
| --- | --- | --- |
| estimativa 2SLS principal (W = 15 min) | 0,1446378859145094 | o mesmo, dígito por dígito |
| IC 90% por bootstrap de dias | [0,131981188365084; 0,15683852512801463] | o mesmo |
| F do primeiro estágio | 5523,46 | o mesmo |
| razão r declarado / r medido | 4,148290720694298 | 4,148290720694298 |
| 0,6 dentro do IC 90% | não | não |
| viés do previsor ingênuo contra a demanda exógena | 0,29124674633711956 | o mesmo |
| veredito do critério declarado no pré-registro | **o laço do H3 superestima o medido** (sinal confere, ordem não: a razão fica fora da banda de 1/3 a 3) | o mesmo |

**Divergências entre a conta e o registro: nenhuma.** O instrumento sabe reprovar e a prova está nos testes: um
registro em que a razão caia dentro da banda declarada vira "tem o sinal e a ordem do medido"; um registro em que
o campo da razão não bata com a conta é reprovado; um instrumento fraco (F abaixo do mínimo declarado) indecide.

## Declaração 4 — o limite, declarado linha por linha

| linha do H3 | classe | o que fica declarado |
| --- | --- | --- |
| os seis fluxos sob relógio estrito, com canário | **registro**, marca **P** | os diários dos fluxos saíram da árvore. O que existe em Rust é o **mecanismo**, no Capítulo 20: com o relógio honesto o canário fica disponível em 0 de 24 466 emissões e o ganho é +0,0000; com o relógio desligado, em 24 466 de 24 466, com ganho de +0,6521. Mecanismo medido não é cadeia medida |
| o cofre de Rhode Island | **registro**, marca **D** | o diário de Rhode Island saiu da árvore. O laboratório mede o mecanismo (ECE 0,0022) e o capítulo declara a divergência contra o cofre real (0,0048), que segue sem instrumento |
| o laço performativo (Goodhart) | **derivada**, marca **V** | é a única linha que se re-deriva, e é isso que esta frente entrega |
| o hipergrafo do H3 | **registro**, marca **P** | o grafo saiu com o andaime; o hipergrafo do Rust é o validador, não o grafo do H3 |

**O que isto não resolve.** A segunda opinião sobre os números do H3 que continuam sem dado bruto — o cofre de
Rhode Island e as contagens do hipergrafo — segue presa a simulação, e é o item 12 do inventário. Este instrumento
não produz segunda opinião: ele re-deriva a aritmética de uma medida registrada, e declara o resto.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-h3` | 40 testes aprovados, entre eles os sete do `cadeia`: as quatro linhas com as marcas `P, D, V, P`, a re-derivação com os dez dígitos, o veredito que vira quando a razão entra na banda, o registro que mente na razão (duas divergências), o instrumento fraco que indecide, a linha derivada sem registro versionado e o leitor de JSON contra o arquivo real (0) |
| `labs --h3` | as quatro linhas com a classe e a marca, a re-derivação do laço com zero divergência e as duas provas negativas (0) |
| `python3 book/conferir_labs.py` | o bloco do Capítulo 20 confere em `labs --h3` (0) |
