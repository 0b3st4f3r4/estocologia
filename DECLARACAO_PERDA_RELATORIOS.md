# Declaração de perda: os três relatórios que nunca estiveram no Git

Registro de 2026-10-17. Existe porque dois documentos davam um comando de recuperação que nunca pôde funcionar, e
um comando impresso é uma promessa: quem o lê acredita que o arquivo volta com um `git checkout`.

## 1. O que os documentos afirmavam

| documento | linha | o que dizia |
| --- | --- | --- |
| `PROPOSTAS_DECLARADAS.md` | §6, tabela | `book/revisao.md`, seção 5, aponta hoje para `DECLARACAO_CONSOLIDACAO_RUST.md` e `REGISTRO_EMPIRICO.md` |
| `PROPOSTAS_DECLARADAS.md` | §145, pendência 2 | "os ponteiros de §6 não voltam", com `git checkout 1bfec905 -- book/revisao.md book/resposta-a-revisao.md book/resultados-que-dependem-de-python.md` |
| `DECLARACAO_CONSOLIDACAO_RUST.md` | §"Como recuperar" | o mesmo `git checkout`, com as três no fim da lista, depois de `h0-real/ h1-real/ h2/ h2g/ h3/ fundo/` |

## 2. O que a árvore responde

~~~text
$ git cat-file -e 1bfec905:book/revisao.md
fatal: path 'book/revisao.md' does not exist in '1bfec905'
$ git ls-tree -r --name-only 1bfec905 | wc -l
1314
$ git ls-tree -r --name-only 1bfec905 | grep -c '^book/'
0
~~~

O commit `1bfec905` — "Simplificação: o que é gerado por script sai do Git (armazém, cópias do p03, saída_base do
p09)" — tem 1 314 arquivos e **nenhum** sob `book/`. As seis pastas que a mesma linha manda buscar existem lá:

| caminho pedido | arquivos em `1bfec905` |
| --- | --- |
| `h0-real/` | 39 |
| `h1-real/` | 47 |
| `h2/` | 59 |
| `h2g/` | 89 |
| `h3/` | 96 |
| `fundo/` | 579 |
| `book/revisao.md` | **0** |

A varredura completa do histórico confirma o resto: dos 94 commits, nenhum contém `book/revisao.md`, e
`git log --all --diff-filter=A -- book/revisao.md` não devolve nada — o arquivo nunca foi adicionado. O diretório
`book/` entra no Git em `64ef89a` ("Poda: andaime Python e cópias geradas saem do disco; workspace Rust entra"),
que é descendente de `1bfec905`:

~~~text
$ git merge-base --is-ancestor 1bfec905 64ef89a && echo ancestral
ancestral
~~~

## 3. O que os manifestos de remoção listam

Os três manifestos de remoção foram procurados por qualquer caminho sob `book/`:

~~~text
$ grep -c 'book/' historico/registro-python-removido.txt
0
$ grep -n 'revisao\|resposta-a\|dependem' historico/registro-python-removido.txt historico/python-removido.txt historico/nao-significante-removido.txt
(nenhuma linha)
~~~

Ou seja: os três relatórios não estão no manifesto do Python aposentado, nem no manifesto do material não
significante, nem no registro sem significância para a teoria. Não há hash de blob para eles em lugar nenhum, e
portanto não há o que restaurar por hash.

## 4. O veredito

**Os três relatórios não existem nesta árvore nem em nenhum snapshot do histórico versionado, e não têm blob
registrado.** Duas leituras ficam possíveis, e o registro não escolhe por conta própria: ou eles nunca foram
versionados sob esses caminhos, ou foram versionados antes do primeiro commit desta árvore e saíram junto com o
material que a consolidação descartou. O que a árvore prova é a ausência; a causa fica declarada como
indeterminada, em vez de inventada.

## 5. O que fica no lugar

| o que se procurava | onde está hoje |
| --- | --- |
| a revisão dos números, `book/revisao.md` | `REGISTRO_EMPIRICO.md`, com a marca R, D ou P de cada número, e a `DECLARACAO_CONSOLIDACAO_RUST.md` |
| a triagem dos resultados que dependiam de Python | `book/resultados-significantes.md` |
| a resposta à revisão | os capítulos e `governanca/` |
| o que saiu e como voltar | `DECLARACAO_CONSOLIDACAO_RUST.md` §"O que saiu" e §"Como recuperar", agora sem o caminho fantasma |

## 6. O que mudou nos documentos

1. `PROPOSTAS_DECLARADAS.md` §145 perdeu o comando, e a pendência passou a registro de perda.
2. `DECLARACAO_CONSOLIDACAO_RUST.md` perdeu `book/revisao.md` da linha de `git checkout` e ganhou o limite
   declarado com o número de arquivos do commit.
3. `PROPOSTAS_DECLARADAS.md` §6 ganhou a nota de que o ponteiro antigo não tem destino no histórico.

## 7. O que não mudou

Nenhum número de resultado mudou. O que mudou foi a promessa de recuperação, que agora é a que o Git sustenta.
