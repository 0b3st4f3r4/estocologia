# Declaração: consolidação em Rust (o andaime de medição em Python saiu da árvore)

Em 2026-10-16, depois de a revisão e os relatórios serem lidos, o repositório fez a consolidação final: o
**andaime de medição em Python** saiu, e o que resta é o livro, os crates, o registro dos números, os dados que
os instrumentos leem, a governança e a auditoria de fundo.

## O que saiu

**715 arquivos versionados e 422,0 MB**, entre pastas e medições:

| Item | O que era |
| --- | --- |
| ` h0-lab0/`, ` h0-real/` | as frentes do $H_0$, com READMEs, declarações, saídas e figura |
| ` h1/`, ` h1-real/` | as frentes do $H_1$ (Dow Jones, LaLonde, incêndios) |
| ` h2/` a ` h2g/` | as sete rodadas do ICP, com as reexecuções e as declarações |
| ` h3/` | a malha contínua, critérios e revisões H3b a H3d |
| `dados-reais/h2*/`, `h3b/`, `h3c/`, `crsp_day_1989_1998.csv` | os dados que só essas frentes usavam |
| `book/revisao.md`, `book/resposta-a-revisao.md`, `book/resultados-que-dependem-de-python.md` | relatórios gerados, já lidos. **Correção de 2026-10-17:** eles não têm blob em `registro-python-removido.txt`, não existem em `1bfec905` e não existem em nenhum dos 94 commits da árvore. O registro do desaparecimento está em `DECLARACAO_PERDA_RELATORIOS.md` |
| `FORA_DO_GIT.md` e as somas gêmeas | descreviam arquivos grandes que saíram com as frentes |

## O que fica, e por quê

| Fica | Papel |
| --- | --- |
| `RESULTADOS_RUST.md` | a saída literal dos instrumentos, com o comando em cima de cada bloco |
| `REGISTRO_EMPIRICO.md` | os números da pesquisa, com a marca R, D ou P de cada um |
| `dados-reais/` | os cinco dados que os instrumentos leem, `FONTES.md` e `SHA256SUMS` |
| `governanca/`, `fundo/`, `referencias/` | quem assina, a auditoria de fundo e a conferência da bibliografia |
| `artigo/` | o artigo, que é outra peça e não andaime |
| `book/`, `estocologia-livro-didatico.md`, `crates/` | o livro e o aparato |

## Como recuperar

~~~bash
git checkout 1bfec905 -- h0-real/ h1-real/ h2/ h2g/ h3/ fundo/
~~~

O manifesto `registro-python-removido.txt` traz o hash de blob de cada arquivo versionado removido.

**Limite declarado em 2026-10-17.** A linha de recuperação acima pedia também `book/revisao.md`, e isso nunca
funcionou: o diretório `book/` não existe em `1bfec905` (o commit tem 1 314 arquivos, e as seis pastas acima
estão entre eles), e nenhum dos 94 commits da árvore contém os três relatórios. O comando perdeu o caminho
fantasma, e o registro do que não se recupera está em `DECLARACAO_PERDA_RELATORIOS.md`.

## Consequência declarada

Os capítulos citam as frentes pelo nome (`h0-real/README.md`, `h1-real/README.md`, `h2g/README.md` e
outras): essas menções passam a ser **históricas**, e o caminho vigente para o número é o
`REGISTRO_EMPIRICO.md` e a nota de recuperação acima. Nenhum número foi trocado: o que mudou foi onde ele
mora.
