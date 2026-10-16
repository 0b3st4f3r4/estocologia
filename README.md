# Estocologia

Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.

Este repositório guarda a pesquisa conduzida segundo a arquitetura do projeto (hipergrafo tipado, camadas
Mundo/Modelo/Agente, banca adversarial A1–A8, orquestrador M, fichas estruturadas, invariantes Φ1–Φ5), do H0 ao
H3, em dados reais. A pesquisa foi encerrada em 2026-10-15 e o aparato de engenharia é **Rust**.

| Pasta | Conteúdo |
|---|---|
| `REGISTRO_EMPIRICO.md` | Os números da pesquisa, com o instrumento Rust que reproduz cada um e a marca de divergência declarada |
| `dados-reais/` | Os cinco dados que os instrumentos leem (`bank_calls.csv`, `djia_1885_2023.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `danish_fire.csv`), com origens em `FONTES.md` e somas em `SHA256SUMS` |
| `governanca/` | Pareceres de risco e a auditoria de quem paga pelo erro |
| `AGENTS.md` | O contrato de governança, engenharia e orquestração desta edição |
| `fundo/` | Pontos de fundo: auditoria do livro, uma frente por ponto (tese central, independência, multiplicidade, prioris, laço performativo, ICP, calibração, escala e dados) |
| `referencias/` | Conferência das referências bibliográficas |
| `artigo/` | Artigo "Estocologia": Velhas Fronteiras, Novos Horizontes, com as figuras |
| `estocologia-livro-didatico.md` | Livro didático em 21 capítulos e 5 partes, três degraus por capítulo, com laboratórios em Rust cuja saída está citada no texto (a edição anterior, de 14 capítulos, vive no histórico do git, em `estocologia-livro-didatico.md` no commit `1bfec905`) |
| `book/` | Edição do livro em 21 capítulos e 5 partes, três degraus por capítulo: um arquivo por capítulo, `gerar.py` monta e confere |
| `PLAN.md` | Plano diretor: epistemologia, pedagogia, arquitetura, banca adversarial A1–A8 e governança |
| `crates/` | Workspace Rust (Edition 2024): quinze crates — núcleo, hipergrafo e lentes, trilha SHA-256, portões, laboratórios do livro, contrato em arquivo (`-schema`), deliberativo (`-s2`), os quatro horizontes (`-h0` a `-h3`), causalidade invariante (`-icp`), deriva (`-drift`), justiça (`-fairness`) e o playground WebAssembly (`-wasm`) |
| `RESULTADOS_RUST.md` | Registro dos resultados em Rust, gerado por `book/gerar_resultados.py` a partir dos próprios instrumentos |
| `PROPOSTAS_DECLARADAS.md` | Inventário do que está declarado e ainda não exigido nem medido: SHELF, genética fina, a voz de quem paga, a conferência da bibliografia, a reexecução do H3, as previsões 3 e 5 do eixo R, a ponte em escala, as migrações de contrato, o eixo R nos capítulos e o registro sem segunda opinião |
| `DECLARACAO_REMOVIDO_PYTHON.md` | O que saiu da árvore em 2026-10-16: o código Python de simulação, com o commit de recuperação e o manifesto `python-removido.txt` |

Em 2026-10-16 o andaime de medição em Python saiu da árvore — o código, as frentes `h0` a `h3` com os seus
READMEs e os dados que só elas usavam. O número ficou: em `REGISTRO_EMPIRICO.md` e no livro, com o commit
`1bfec905` como âncora de recuperação e os manifestos `python-removido.txt`, `registro-python-removido.txt` e
`nao-significante-removido.txt` com o hash de blob de cada arquivo.

Como verificar o que existe hoje:

```bash
cargo test --workspace
cargo run --release --bin labs -- --todos
cargo run --release --bin labs -- --s2
cargo run --release --bin portoes -- --horizonte todos --reproducao
cargo build --release --target wasm32-unknown-unknown -p stochology-wasm   # o playground
python3 book/gerar.py --check && python3 book/conferir_labs.py && python3 book/gerar_resultados.py --check
```
