# Estocologia

Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.

Este repositório guarda a pesquisa conduzida segundo a arquitetura do projeto (hipergrafo tipado, camadas
Mundo/Modelo/Agente, banca adversarial A1–A8, orquestrador M, fichas estruturadas, invariantes Φ1–Φ5), do H0 ao
H3, em dados reais. A pesquisa foi encerrada em 2026-10-15 e o aparato de engenharia é **Rust** (Edition 2024):
**dezesseis crates**, **344 testes**, **vinte e um instrumentos** no binário `labs` e **sete portões** no binário `portoes`
que sabem reprovar.

## O que está aqui

| Caminho | Conteúdo |
|---|---|
| `estocologia-livro-didatico.md` | O livro didático em **21 capítulos e 5 partes**, três degraus por capítulo, com a saída dos laboratórios citada no texto. É o artefato do leitor humano, e é **gerado**: 5 927 linhas e 75 209 palavras montadas de `book/` |
| `book/` | A edição em fonte, um arquivo por capítulo, com `gerar.py` (monta e confere), os dois conferidores e o gerador do registro |
| `crates/` | O workspace Rust: núcleo, hipergrafo e lentes, trilha SHA-256, portões, laboratórios do livro, contrato em arquivo (`-schema`), deliberativo (`-s2`), os quatro horizontes (`-h0` a `-h3`), causalidade invariante (`-icp`), deriva (`-drift`), justiça (`-fairness`), o playground WebAssembly (`-wasm`) e o motor ondulatório (`-wave`: superposição fasorial, Laplaciano magnético, plasticidade Kuramoto-Oja e espículas com STDP) |
| `REGISTRO_EMPIRICO.md` | Os números da pesquisa, com o instrumento Rust que reproduz cada um e a marca de divergência declarada |
| `RESULTADOS_RUST.md` | O registro gerado a partir dos próprios instrumentos: cada bloco é a saída literal do comando acima dele. §5 é o manifesto da edição, com o `sha256` de cada declaração de ponto |
| `dados-reais/` | Os cinco dados que os instrumentos leem (`bank_calls.csv`, `djia_1885_2023.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `danish_fire.csv`), com origens em `FONTES.md` e somas em `SHA256SUMS` |
| `fundo/` | Os pontos de fundo: tese central, independência, multiplicidade, prioris, laço performativo, ICP, calibração, genética, H3, previsões do eixo R, ponte em escala, SHELF, escuta, segunda opinião e Jev. Cada pasta traz a declaração e o seu `*.sha256` |
| `governanca/` | Pareceres de risco e a auditoria de quem paga pelo erro |
| `AGENTS.md` | O contrato de governança, engenharia e orquestração: as quatro proibições, os invariantes, a banca, o ciclo e a **verificação obrigatória de §V** |
| `PLAN.md` | Plano diretor: epistemologia, pedagogia, arquitetura, banca adversarial A1–A8 e governança |
| `PROPOSTAS_DECLARADAS.md` | O inventário do que está declarado e ainda não exigido nem medido: **quatro itens vivos** — SHELF (3), a voz de quem paga o erro (5), a tese da ponte em escala (9) e a ponte Jev-LLM (13). Os nove que fecharam estão em §2, com a prova ao lado |
| `DECLARACAO_DO_ESTADO.md` | **O estado da árvore, não uma edição.** Antes de publicar não há edições numeradas: há um estado vivo, com o que ele mediu, o que reprovou contra si mesmo, o que não pôde fazer e como se reproduz. O selo é `DECLARACAO_DO_ESTADO.sha256`. As correções de 2026-10-17 estão em `historico/`, como registros datados com os hashes de antes e depois |
| `referencias/` | A conferência das referências bibliográficas |
| `artigo/` | O artigo "Estocologia: Velhas Fronteiras, Novos Horizontes", com as figuras |
| `estilo/` | O instrumental da passada de prosa: `conferir-passada.py` compara cada arquivo com a versão dele no `HEAD` |
| `conferir_lacres.py` | O conferidor dos 26 lacres SHA-256 da árvore, de um comando e de um diretório |
| `INTEGRACAO_JEV.md` | A auditoria da transcrição *Integração de Modelos Jev à Estocologia*: cada alegação conferida contra o código, com as divergências declaradas — a numeração do roteiro antigo contra a vigente ($M_{15}$ e $MB_1$–$MB_8$ de um lado, $\mathcal{M}$ e $A_1$–$A_8$ do outro), quatro filtros contra cinco, $\Phi_1$–$\Phi_4$ contra $\Phi_1$–$\Phi_5$, e o *LLM-as-Jev* — e a ponte Jev-LLM como proposta declarada |
| `DECLARACAO_REMOVIDO_PYTHON.md`, `DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md` | O que saiu da árvore em 2026-10-16, com o commit de recuperação e os manifestos de hash de blob |

## Estado medido

Medição desta árvore em **2026-10-17**, cada linha com o comando que a produz:

| Medida | Valor | Comando |
|---|---|---|
| Testes | **344 aprovados**, 0 reprovados, nos dezesseis crates | `cargo test --workspace` |
| Instrumentos | **20** no `labs`, todos com código 0 | `labs --todos`, `--banco`, `--fila`, `--s1`, `--s2`, `--resolucao`, `--genetica`, `--h3`, `--segunda`, `--ponte`, `--escuta`, `--schema`, `--jev` e os demais |
| Portões | **7 ok**, raiz de Merkle `9716e28a87bec9cf…` e segunda execução idêntica | `portoes --horizonte todos --reproducao` |
| Prova negativa | **1 portão reprovado**, como declarado, com a banca no pior valor 2 contra o piso 6 | `portoes --horizonte h3 --defeito h3.voos23` |
| Livro montado | **5 927 linhas**, **75 209 palavras**, 21 capítulos, 5 partes | `book/gerar.py --check` |
| Blocos citados | **48**, nenhum divergente | `book/conferir_labs.py` |
| Lacres | **26 arquivos**, **64 alvos conferidos**, 8 linhas declaradas fora da árvore, 0 problemas | `conferir_lacres.py` |

## Como verificar

~~~bash
# a máquina
cargo test --workspace
cargo run --release --bin labs -- --todos
cargo run --release --bin labs -- --s2
cargo run --release --bin portoes -- --horizonte todos --reproducao
cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23   # sai com 1

# o livro: a rampa, o disco, os blocos e a prosa
python3 book/gerar.py --check          # compara com o disco caractere por caractere e cobra a contagem declarada
python3 book/conferir_labs.py          # cada bloco citado contra o instrumento que o gerou
python3 book/conferir_livro.py         # partes, capítulos, cercas e ordem
python3 book/gerar_resultados.py --check
python3 estilo/conferir-passada.py     # a passada de prosa contra HEAD:book/, arquivo por arquivo

# os lacres, de um só comando, de qualquer diretório
python3 conferir_lacres.py

# o playground
cargo build --release --target wasm32-unknown-unknown -p stochology-wasm
~~~

A lista vinculante é a de `AGENTS.md` §V: nenhum número entra no livro sem sair de um daqueles comandos.

## A regra que a edição aprendeu

Portão que não olha o alvo é portão cego. O `book/gerar.py --check` de antes montava o livro na memória e
conferia só a rampa dos 21 capítulos: passava com código 0 enquanto o arquivo em disco estava **353 linhas atrás**
de `book/`. Hoje o `--check` compara `montar()` com `estocologia-livro-didatico.md` caractere por caractere,
imprime a diferença e sai com 1 quando o disco não é o livro montado. O mesmo princípio vale para os lacres: um
comando, uma convenção de caminho (relativa à raiz) e lacre sem alvo vivo aparece como **declarado**, nunca como
aprovado. E vale para a contagem: o livro ganhou duas linhas em `book/cap13.md`, o manifesto foi regerado e três
documentos continuaram afirmando 5 903 linhas — com o `--check` passando, porque ele só olhava o texto. Desde a
retificação de 2026-10-17 ele lê os pares "N linhas ... N palavras" do `README.md`, do `PLAN.md`, da declaração
e do registro, compara com o que o mesmo `montar()` mede e sai com 1 quando divergem.

## O que segue aberto

Os quatro não são da mesma natureza. Os três primeiros dependem de **gente de fora do programa**, e não fecham com
mais código: o que falta é consentimento, escuta e julgamento cego. O quarto depende de **artefato e código**: os
pesos congelados com `sha256` e o runner que os carregue, e ele fecha com um download, uma declaração de hash e uma
implementação.

| Item | Dependência | O que falta | Prazo |
|---|---|---|---|
| 3. Elicitação SHELF | **gente de fora do programa** | as outras duas pessoas por domínio e o controle de fora do programa; a trilha do primeiro especialista está fechada em cinco domínios | sem data |
| 5. A voz de quem paga o erro | **gente de fora do programa** | a escuta dos grupos afetados: **0 de 7** ouvidos, cobertura mínima de 2 | **2026-12-15** |
| 9. A tese da ponte em escala | **gente de fora do programa** | escolhedor de casos de fora, dois analistas humanos e um juiz humano cego: **0 de 12** casos | **2027-01-14** |
| 13. A ponte Jev-LLM | **artefato e código** | os pesos com o `sha256` declarado; o runner local existe desde 2026-10-17 (`crates/stochology-s2/src/runner.rs`) | sem data |

Vencida a janela da escuta sem a cobertura mínima, as sete decisões ficam arquivadas como exercício de
sensibilidade e não viram diretriz de política pública. Vencida a janela da ponte, a tese fica **não sustentada
em definitivo**, como o dono aceitou por escrito.

## O que saiu da árvore

Em 2026-10-16 o andaime de medição em Python saiu da árvore — o código, as frentes `h0` a `h3` com os seus
READMEs e os dados que só elas usavam. O número ficou: em `REGISTRO_EMPIRICO.md` e no livro, com o commit
`1bfec905` como âncora de recuperação e os manifestos `python-removido.txt`, `registro-python-removido.txt` e
`nao-significante-removido.txt` com o hash de blob de cada arquivo. O que resta em Python são as ferramentas de
conferência do livro, do registro e dos lacres — e nada mais.
