# AGENTS.md — Contrato de governança, engenharia e orquestração (Estocologia, edição Rust)

Contrato vinculante para qualquer agente ou colaborador que opere neste repositório. A máquina descrita aqui é
uma **máquina epistemológica auditável**: alegação, processo gerador, modelo nulo e teste no mesmo lugar, com
semente fixa e portão que sabe reprovar. Quem lê o resultado é **humano** — estudante, docente ou decisor.

Este arquivo substitui a versão em Python do contrato, integrada em 2026-10-16. O que mudou: o aparato é
**Rust**; o código Python de simulação foi aposentado; o curso que dava a numeração interna foi obsoletado; e
tudo o que segue declarado e ainda não exigido está em `PROPOSTAS_DECLARADAS.md`.

---

## I. Regras de ouro — as quatro proibições

1. **Proibida a linguagem natural livre entre agentes.** A comunicação passa por fichas estruturadas
   (`Ficha`, em `crates/stochology-core/src/camada.rs`), com identificador, camada, tipo de incerteza,
   definição em uma linha, artefato simulável, semente, unidade independente, modelo nulo, escopo, quem paga
   pelo erro e cláusula de saída.
2. **Proibida a média solitária.** Onde a função é não linear, a dinâmica é multiplicativa ou a cauda é
   pesada, é proibido reportar só a média: reporte o caso médio $f(E[X])$, a média simulada $E[f(X)]$, a
   mediana, os quantis e a curva acumulada. O Capítulo 5 do livro mede o pedágio; o instrumento
   `labs --cap 5` o reproduz.
3. **Proibido confundir aprendizado com performatividade.** Erro de previsão pequeno não prova que o modelo é
   verdadeiro. Rode com e sem a ação (`do(a)` contra observação passiva) e declare o laço: o Goodhart
   medido está no Capítulo 20 e vale 0,145, quatro vezes menos que os 0,6 declarados.
4. **Proibidos os extremos dogmáticos e a omissão de tentativas.** Nenhuma probabilidade pode ser 0% ou 100%;
   toda previsão é pontuada por regra própria (Brier ou log) fora do ajuste; e o número de especificações
   tentadas tem de ser declarado — no passado foi o contador K, que hoje é registro (`K = 38` no portão do
   H2).

## II. Invariantes de contorno

| Invariante | O que exige | Onde vive no código |
| --- | --- | --- |
| Phi1 — simulabilidade | semente fixa e artefato que roda | `crates/stochology-core/src/invariantes.rs` e `rng.rs` |
| Phi2 — unidade independente | margem na unidade real, nunca pseudorreplicação | validado na ficha; medido em `labs --cap 16` e `labs --banco` |
| Phi3 — teste severo | modelo nulo simulado antes de ler padrão | validado na ficha; `labs --cap 7` e `labs --cap 14` |
| Phi4 — escopo e saída | quem paga pelo erro e cláusula objetiva de saída | validado na ficha; banca A8 |
| Phi5 — resolução da pergunta | exigido quando a unidade é o dia ou mais grossa: aí resolução agregada (R0) reprova; unidade mais fina fica fora, declarado | `crates/stochology-core/src/invariantes.rs` e `src/resolucao.rs` |

O binário `portoes` cobra os invariantes e a banca em sete portões (G1 a G7) e sai com código 0 ou 1.

## III. Ontologia e as cinco lentes

O conhecimento é um hipergrafo tipado G = (V, E_tree, E_graph, H, Phi). Nó tem camada (L1 Mundo, L2 Modelo,
L3 Agente), tipo de incerteza (aleatória, epistêmica, knightiana), definição e ponteiro para o artefato.
Arestas de patologia marcam desalinhamento entre camadas. As cinco lentes — **Fluxonomia, Crononomia,
Escalonomia, Morfonomia, Crisonomia** — vivem em `crates/stochology-hypergraph/src/lentes.rs`, com testes.

## IV. Banca adversarial, orquestrador e ciclo

Os oito agentes da banca são A1 a A8 (código, dados, ciclo de vida, inferência, governança, justiça, domínio,
decisão), em `crates/stochology-core/src/agentes.rs`, com a condição

    Aprovado(X) = E de i=1..8 ( A_i(X) >= Severidade_min ), Severidade_min = 6.

Registros antigos usam os rótulos MB1–MB8: é a mesma lista. O orquestrador M
(`crates/stochology-core/src/orquestrador.rs`) conduz o ciclo de seis estágios: tradução e escopo, ingestão
e grão, simulação e modelo nulo, projeção nas cinco lentes, banca e portão, decisão e fechamento. Dos dois
modos, o **Sistema 1** já vive em `crates/stochology-core/src/s1.rs`, com o funil de filtros ao lado, e o
**Sistema 2** vive em `crates/stochology-s2`: LATS, motor de estado de crença, inferência de Metropolis e
estresse deliberativo da banca, com a síntese de código (F1) declarada e não implementada. Veja
`PROPOSTAS_DECLARADAS.md`.

## V. Verificação obrigatória antes de consolidar

~~~bash
cargo test --workspace                              # os quinze crates
cargo run --release --bin labs -- --todos           # os 21 laboratórios do livro
cargo run --release --bin labs -- --banco           # o H0 real, lendo dados-reais/bank_calls.csv
cargo run --release --bin labs -- --resolucao       # o horizonte R
cargo run --release --bin labs -- --s1              # a triagem do Sistema 1
cargo run --release --bin labs -- --s2              # o Sistema 2: hipergrafo, crença, inferência e eixo R
cargo run --release --bin labs -- --fila            # a fila com desistência e a tabela R0 a R3
cargo run --release --bin portoes -- --horizonte todos --reproducao   # os sete portões
cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23  # prova negativa: sai com 1
python3 book/gerar.py --check                       # a rampa E o disco: o arquivo tem de ser o livro montado
python3 book/conferir_labs.py                       # cada bloco citado contra o instrumento
python3 book/conferir_livro.py                      # partes, capítulos, cercas e ordem
python3 book/gerar_resultados.py --check            # o registro RESULTADOS_RUST.md
python3 conferir_lacres.py                          # os 26 lacres SHA-256, caminho relativo à raiz
python3 estilo/conferir-passada.py                  # a passada de prosa contra HEAD:book/, arquivo por arquivo
~~~

Os instrumentos que provam o que os itens de `PROPOSTAS_DECLARADAS.md` §2 fecharam também entram na
verificação, porque cada linha daquela tabela cita um deles:

~~~bash
cargo run --release --bin labs -- --genetica    # a resolução fina da genética
cargo run --release --bin labs -- --h3          # a cadeia do H3
cargo run --release --bin labs -- --segunda     # a segunda opinião do registro
cargo run --release --bin labs -- --ponte       # a tese da ponte em escala
cargo run --release --bin labs -- --escuta      # a voz de quem paga o erro
cargo run --release --bin labs -- --schema      # a cadeia de migrações do contrato
cargo run --release --bin labs -- --jev         # a ponte Jev-LLM
cargo run --release --bin labs -- --resolucao   # o eixo R e as previsões 3 e 5
~~~

Nenhum número entra no livro sem sair de um destes comandos. Quando um valor divergir do registro antigo, o
capítulo traz uma **declaração de divergência** com os dois valores e o motivo — é o que os Capítulos 4, 5, 8,
16, 17, 18, 20 e 21 fazem.

## VI. Regra dos arquivos novos

Arquivo novo de trabalho — cópia intermediária, script de uma vez, rascunho — é **apagado depois da
integração**. Fica no repositório só artefato com função contínua (`book/gerar.py`, os dois conferidores do
livro, o gerador do registro, o conferidor de lacres, o instrumental de estilo, os crates) ou registro
(capítulos, declarações, manifestos, relatórios).

## VII. Estado do projeto

| Camada | Onde | Estado |
| --- | --- | --- |
| Livro | `estocologia-livro-didatico.md`, escrito em `book/` | 21 capítulos, 5 partes, montado e conferido |
| Aparato | `crates/stochology-*` | quinze crates vivos; nenhum nome do roteiro ficou para trás (2026-10-16) |
| Resultados | `RESULTADOS_RUST.md` | registro gerado dos próprios instrumentos |
| Registro empírico | `REGISTRO_EMPIRICO.md`, `fundo/`, `governanca/`, `dados-reais/` | cada número com a marca do instrumento que o reproduz; o andaime Python saiu em 2026-10-16 |
| Aposentado | commit `1bfec905` | o código Python de simulação e os artefatos não significativos para a teoria |
| Declarado e não exigido | `PROPOSTAS_DECLARADAS.md` | **quatro itens vivos**, e nenhum se fecha com mais código: **3** elicitação humana SHELF (o painel completo), **5** a voz de quem paga o erro (0 de 7 grupos ouvidos, prazo 2026-12-15), **9** a tese da ponte em escala (0 de 12 casos, janela 2027-01-14) e **13** a ponte Jev-LLM (faltam os pesos com `sha256` e o runner local). Os itens 1, 2, 4, 6, 7, 8, 10, 11 e 12 saíram da lista em 2026-10-16, com a prova em §2 do mesmo arquivo; o Phi5, o campo de resolução, o Sistema 1, o funil, o cofre, a fila e o Sistema 2 saíram antes, ao serem implementados |

## VIII. O que não entra neste repositório

- **Python de simulação:** aposentado em 2026-10-16 (`DECLARACAO_REMOVIDO_PYTHON.md`). O que resta em Python
  são as ferramentas de conferência do livro e do registro, e nada mais: `book/gerar.py`, os dois conferidores
  do livro, `book/gerar_resultados.py`, `conferir_lacres.py` e o instrumental de estilo em `estilo/`. O
  instrumental de estilo é autorizado aqui porque a passada de prosa é parte do ciclo de verificação contínua,
  está declarada em `estilo/decisoes.md` e entra na verificação obrigatória de §V. Ferramenta de conferência
  que não entra em §V não fica na árvore.
- **O curso e a sua numeração:** o curso *Pensamento Crítico e Modelos Estocásticos* foi obsoletado; os rótulos
  M0–M15, ME1–ME24 e MB1–MB8 são históricos e não são nomenclatura vigente.
- **Artefato sem significância para a teoria:** o critério está em `book/resultados-significantes.md`, e a
  última aplicação está em `DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`.
