# `book/` — a edição em 21 capítulos

Esta pasta guarda o livro didático sendo desdobrado em **21 capítulos e 5 partes**, com **três degraus
por capítulo** (intuição de Ensino Médio, formalização acessível, laboratório em Rust). O plano do
desdobramento é o §9 do `PLAN.md`.

| Arquivo | Conteúdo |
| --- | --- |
| `00-abertura.md` | prefácio, rampa de três degraus, mapa dos 21 capítulos, trilhas de leitura e convenções |
| `cap01.md` … `cap21.md` | um arquivo por capítulo |
| `apendices.md` | apêndices A a D |
| — | os laboratórios vivem direto no crate: `crates/stochology-book/src/capNN.rs` |
| `gerar.py` | monta a edição e confere a rampa |

## Comandos

```bash
python3 book/gerar.py --check      # confere capítulos presentes, três degraus, exercícios e resumo
python3 book/conferir_labs.py     # confere o bloco de saída de cada capítulo contra o instrumento
python3 book/gerar_resultados.py  # escreve RESULTADOS_RUST.md com a saída literal dos instrumentos
python3 book/gerar_resultados.py --check   # confere se o registro está atualizado
python3 book/gerar.py              # escreve estocologia-livro-didatico.md quando os 21 estiverem prontos
cargo run --release --bin labs -- --todos    # roda todos os laboratórios com semente fixa
cargo test --workspace                       # testes dos crates do livro e da arquitetura
```

## Dados reais nos laboratórios

Os laboratórios dos capítulos 17 e 18 leem as bases do próprio repositório
(~dados-reais/djia_1885_2023.csv~, ~nsw_experimental.csv~, ~cps_observational.csv~,
~danish_fire.csv~). O caminho padrão é relativo à raiz do repositório; para apontar para outro lugar,
use a variável de ambiente:

```bash
ESTOCOLOGIA_DADOS=/caminho/para/dados cargo run --release --bin labs -- --cap 17
```

## Regra dos arquivos novos

Arquivo novo de trabalho — cópia intermediária, script de uma vez, rascunho de laboratório — é **apagado depois
da integração**. O que fica no repositório é artefato com função contínua (o gerador, os dois conferidores, o
crate) ou registro (capítulos, declarações, manifestos, relatórios).

## Regra dos números

Todo número do Degrau 3 sai de código que roda:

```bash
cargo run --release --bin labs -- --cap 4
```

Quando um número desta edição divergir do número registrado no livro anterior ou num README da
pesquisa, o capítulo traz um parágrafo **Declaração de divergência** com os dois valores e o motivo
provável. Nenhum valor antigo é apagado.

## Estado

| Parte | Capítulos | Estado |
| --- | --- | --- |
| I. A arte de mirar no escuro | 1 a 4 | escritos; laboratórios no crate e blocos de saída conferidos |
| II. As armadilhas do mundo e dos dados | 5 a 8 | escritos; laboratórios no crate e blocos de saída conferidos |
| III. Causa, tempo e sobrevivência | 9 a 11 | escritos; laboratórios no crate e blocos de saída conferidos |
| IV. Construindo a máquina auditável | 12 a 15 | escritos; laboratórios no crate e blocos de saída conferidos |
| V. A máquina no mundo real | 16 a 21 | escritos; laboratórios no crate e blocos de saída conferidos |
| Apêndices A a D | — | escritos; conferidos contra a edição anterior (o antigo Apêndice C, respostas selecionadas, virou material embutido nos Degraus 3) |

A edição de 14 capítulos continua em `estocologia-livro-didatico.md` até a montagem da nova,
que só sobrescreve aquele arquivo quando os 21 capítulos estiverem prontos (`gerar.py` se recusa a
escrever antes disso).
