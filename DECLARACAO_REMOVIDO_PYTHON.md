# Declaração: o código Python de simulação foi aposentado

Em 2026-10-16, o repositório deixou de manter na árvore o **código Python de simulação** das frentes de
pesquisa. A pesquisa está encerrada desde 2026-10-15 e o aparato de engenharia do projeto passou a ser o
workspace Rust (`crates/stochology-*`), de modo que as simulações em Python cumpriram o seu ciclo.

## O que saiu

**576 arquivos `.py`** (cerca de 2,9 MB), entre eles:

| Categoria | Exemplos |
| --- | --- |
| Pacotes de simulação | `h*/estocologia/{estat,icp,dsep,glm,armazem,diario,funil,amostrador,dominio,barramento}.py` |
| Motores de cada frente | `h*/rodar.py`, `h*/grafo.py`, `h*/figura.py`, `h3/fluxos/*.py`, `fundo/*/codigo/*.py` |
| Portões e testes | `h*/portoes.py`, `h*/conftest.py`, `h*/tests/*.py` |
| Cópias de trabalho das frentes de fundo | `fundo/p05-prioris/`, `fundo/p07-icp/codigo/`, `fundo/p08-calibracao/`, `fundo/p09-escala-dados/`, `fundo/p10-saude/`, `fundo/p11-eventos-raros/` |

## O que ficou

| Fica | Por quê |
| --- | --- |
| `dados-reais/` (dados, `FONTES.md`, `SHA256SUMS`) | é a origem conferível de cada número |
| `README.md` das frentes, `DECLARACAO_*.md`, `CRITERIOS*.md`, `ficha.yaml` | é o registro dos resultados, com os números publicados |
| `saida/` versionada (307 arquivos) | são os resumos criptográficos e os relatórios que a auditoria conferiu |
| `governanca/`, `fundo/*/README.md`, `fundo/p01-tese/pareceres/`, `referencias/`, `artigo/` | é a governança e a auditoria de fundo |
| `book/` (ferramentas do livro) | estrutura o livro: monta e confere |
| `fundo/p09-escala-dados/escala/baixar.py` e `fundo/p10-saude/codigo/baixar.py` | não são simulação: são procedência de dado bruto — e **saíram depois**, com as extensões que serviam, na remoção de 2026-10-16 por não serem significativas para a teoria (`DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`) |
| `crates/stochology-*` | é o aparato de engenharia agora |

## Como recuperar

O código não foi perdido: vive no histórico do git, no commit **`1bfec9050bf046820787bdec8cb9765529658bdf`**.
O manifesto `python-removido.txt` guarda o hash de blob de cada arquivo, para conferência:

```bash
git show 1bfec905:h1/rodar.py           # ver um arquivo
git checkout 1bfec905 -- h1/            # trazer a frente inteira de volta
git checkout 1bfec905 -- fundo/         # trazer a auditoria de fundo inteira de volta
```

## Consequência, declarada

Os comandos Python citados no livro (Apêndice C) e no `FORA_DO_GIT.md` — `python3 rodar.py`,
`python3 portoes.py --reproducao`, `python3 grafo.py`, `python3 figura.py`, `python3 -m pytest -q` —
**não rodam mais na árvore atual**. Eles voltam a rodar depois do `git checkout` acima. O que continua
rodando sem Python é a esteira do livro e dos crates: `cargo test --workspace`,
`cargo run --release --bin labs -- --todos`, `cargo run --release --bin portoes -- --horizonte todos
--reproducao`, `python3 book/gerar.py --check`, `python3 book/conferir_labs.py` e
`python3 book/conferir_livro.py`.

**Antes de aposentar**, a revisão independente do livro conferiu 36 números contra os READMEs das frentes
(34 conferem, 2 são divergência declarada) e rodou `cd h1 && python3 -m pytest -q` (24 testes) e
`portoes --horizonte todos --reproducao` (7 portões ok). Os números conferidos ficam no
`book/revisao.md`, com a fonte de cada um.
