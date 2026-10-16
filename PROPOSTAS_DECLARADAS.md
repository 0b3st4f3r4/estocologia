# Tudo o que ainda está como proposta declarada

Inventário revisto em 2026-10-16. A revisão conferiu cada linha no código desta árvore, e não na redação
anterior: o que já existe aparece com o arquivo, a linha e o teste que o sustentam, e o que falta aparece com
o que precisaria existir para fechar. Cada ponteiro foi aberto antes de entrar na tabela, e um deles não
existia, o que gerou §6 e o conserto das linhas que dependiam dele.

A lista viva tem dez itens (§1). São os sete originais mais as cinco frentes que estavam fora dela e
entraram por decisão do autor em 2026-10-16, menos o item 1 — os dez crates do roteiro — e o item 2 — o
Sistema 2 deliberativo: os dois passaram a viver no mesmo dia, e a prova de cada um está em §2. Os itens que
a consolidação em Rust fechou saíram para §2, com a prova ao lado, e §3 registra as decisões do dia.

## 1. O que continua como proposta declarada

Os números dos itens são os da revisão anterior, para o registro não perder a referência: os itens 1 e 2
saíram da lista em 2026-10-16 e a numeração de 3 a 12 continua como estava.

| Proposta | Onde está declarada | O que falta |
| --- | --- | --- |
| **3. Elicitação humana SHELF** | Capítulos 15 e 21; `fundo/p12-shelf/` (protocolo, 6 formulários, `comparacao.md` em esqueleto) e `governanca/RELEITURA.md` (21 linhas em branco) | a primeira entrevista. Nenhum especialista foi consultado, nenhum parecer relido, nenhum arquivo assinado; cinco dos 33 domínios mudam de decisão conforme a priori escolhida |
| **4. Resolução fina da genética** | `book/cap19.md:80` e `book/cap21.md:68-69,161,274` | a segunda linha: cada mutante é hoje um ambiente de uma linha só, e o piso do desenho é $p = 1/1\,000$ |
| **5. A voz de quem paga o erro** | Capítulos 11, 15, 20 e 21; `governanca/GRUPOS.md` | escuta. Nas sete decisões que afetam pessoas reais, a coluna "ouvidos" diz **nenhum** nas sete |
| **6. Conferência da bibliografia contra os arquivos baixados** | Apêndice D, regras 1 e 2 (linha 208) e §D.6 (linha 325) | marcar as 50 linhas de D.2 a D.4, item a item, como `[conferido]` contra os arquivos baixados em 2026-10-16, e registrar no livro qualquer divergência que apareça. Das 50, 23 já têm evidência em `referencias/CONFERENCIA.md` (D.3 inteiro), e as 27 de D.2 e D.4 seguem sem fonte conferida |
| **7. Reexecução completa do $H_3$** | `REGISTRO_EMPIRICO.md` §H3 (linha 43) | o instrumento Rust que re-rodasse a cadeia: das quatro linhas do $H_3$, três levam a marca **P** e uma leva **D**, com a segunda opinião presa a simulação (item 12) |
| **8. Previsões 3 e 5 do eixo R** *(entrou em 2026-10-16)* | `book/horizontes-de-resolucao-da-pergunta.md`, seção das previsões falsificáveis | frente nova, com a mesma decisão tomada em dois grãos. A 3 é o limiar de oscilação, a 5 é a transferência entre resoluções; as previsões 1, 2 e 4 já têm instrumento (`resolucao.rs`, 3 testes, `labs --resolucao`) |
| **9. A tese da ponte em escala** *(entrou em 2026-10-16)* | Capítulos 1 e 2; `fundo/p01-tese/` | o teste justo. O empate no teto deixou a tese **não sustentada** para o catálogo clássico, e falta rodar com casos escolhidos fora do programa, analistas humanos e juiz humano |
| **10. Migrações de contrato** *(entrou em 2026-10-16)* | `PLAN.md` §7 | o crate de schema e o versionamento por migração: hoje a ficha é o contrato em código, sem arquivo de migração |
| **11. Eixo R nos laboratórios dos capítulos** *(entrou em 2026-10-16)* | `book/horizontes-de-resolucao-da-pergunta.md` | bloco do eixo nos 21 capítulos; o instrumento existe fora do livro (`labs --resolucao`) e não aparece em nenhum Degrau 3 |
| **12. Registro empírico sem segunda opinião** *(entrou em 2026-10-16)* | `REGISTRO_EMPIRICO.md` | segunda opinião para os 12 resultados marcados **P**: $H_0$ real inteiro, $H_2$ real, $H_3$ real (item 7), os números do hipergrafo ($454\,178$ nós, $26\,634$, $12\,797$, 329 domínios) e os da auditoria de fundo ($K = 38$, 0,0071 dp, 33 dos 33, 0,145, os 21 pareceres). Todos voltam com `git checkout 1bfec905 -- <pasta>` |

## 2. Fechado depois do inventário anterior

Oito linhas do inventário anterior, agrupadas aqui em sete itens, passaram a ter código, teste e comando.
Ficam como registro, e não como proposta.

| Item | Prova no código | Comando |
| --- | --- | --- |
| **Sistema 2 deliberativo (item 2)** | `crates/stochology-s2`, com sete módulos testados: `lats` (busca com UCB e o ganancioso que serve de nulo), `crenca` (estado de crença com piso e teto), `hipergrafo` (o mundo sobre `Grafo`, com o valor de cada nó medido em lentes que o cobrem e o nó sem lente como alvo), `inferencia` (Metropolis com alvo declarado), `estresse` (a banca A1-A8 atacada na ordem que a crença aprendeu) e `planejamento` (o eixo R decidido pelo instrumento). A síntese de código (F1) fica declarada e não implementada | `cargo test --workspace` e `labs --s2` |
| **Crates do roteiro (item 1)** | os dez nomes que faltavam vivem, cada um com lib, teste próprio e dependência declarada: `stochology-schema` (contrato em arquivo, versão e cadeia de migrações), `-s2` (LATS e estado de crença), `-h0` (Erlang, escala mínima, nulo de Poisson, resumo obrigatório), `-h1` (arrasto, viés de seleção, Hill), `-h2` (rodadas, contador $K$, Bonferroni, nulo por embaralhamento), `-h3` (relógio estrito com canário, ECE, Brier, laço performativo), `-icp`, `-drift` (CUSUM e KL), `-fairness` (paridade, oportunidades, ECE por subgrupo) e `-wasm` (ponte C e a página de `www/`). Sem dependência externa | `cargo test --workspace` e `cargo build --release --target wasm32-unknown-unknown -p stochology-wasm` |
| **$\Phi_5$ e o campo de resolução na ficha** | `invariantes.rs:12,110` com 10 testes no arquivo, entre eles `acao_agregada_contra_unidade_dia_reprova_phi5`; campo `resolucao` em `camada.rs:52`, troca em `camada.rs:98`; `portoes` conta $\Phi_1$–$\Phi_5$ em `crates/stochology-gates/src/main.rs:316`. Limite declarado: a trava só morde com unidade igual ou mais grossa que o dia | `cargo test --workspace` |
| **Sistema 1 determinístico** | `crates/stochology-core/src/s1.rs` (877 linhas, 12 testes), vocabulário fechado, árvore de sufixos e probabilidade presa em $[0{,}01; 0{,}99]$ | `labs --s1` |
| **Funil de quatro filtros e contador $K$** | `crates/stochology-core/src/funil.rs` (878 linhas, 17 testes), com a poda por valor da informação e `o_funil_conta_as_tentativas_ate_a_versao_final` | `cargo test --workspace` |
| **Cofre fora do tempo** | `crates/stochology-audit/src/cofre.rs` (705 linhas, 6 testes): abertura única, adulteração invalida a verificação e errar depois da abertura exige cofre novo | `cargo test --workspace` |
| **Fila com desistência e a tabela $H_0$ de R0 a R3** | `crates/stochology-book/src/fila.rs` (1 113 linhas, 8 testes); `REGISTRO_EMPIRICO.md` §H0 marca **R** nos agente-horas e **D** nas falhas por dia, com a divergência declarada no Capítulo 16 | `labs --fila` |

## 3. Decisões registradas em 2026-10-16

1. **`stochology-s1` não volta como crate.** A triagem do Sistema 1 fica em
   `crates/stochology-core/src/s1.rs`, com o funil ao lado. O roteiro tinha dez nomes, e os dez passaram a
   viver na segunda passada de 2026-10-16, cada um com teste próprio; some da lista a pergunta sobre um crate
   de fachada.
2. **As cinco frentes que estavam fora da lista entraram nela.** Previsões 3 e 5 do eixo R, a tese da ponte em
   escala, as migrações de contrato, o eixo R nos laboratórios dos capítulos e o registro empírico sem segunda
   opinião são agora os itens 8 a 12.
3. **A conferência da bibliografia é a próxima tarefa.** Marcar de D.2 a D.4 continua sendo trabalho de mesa,
   sem dado novo e sem gente.

## 4. Decisões declaradas de **não** fazer

- **Gabarito próprio:** o antigo Apêndice C (respostas selecionadas) não voltou; as respostas ficam embutidas
  nos Degraus 3 e nos Resumos.
- **Rótulos M0–M15:** aposentados; o Capítulo 13 registra a aposentadoria da sigla M15 e a banca passou a
  $A_1$–$A_8$ (registros antigos usam MB1–MB8).
- **Nomes de binário:** `labs` e `portoes` continuam em português, com o motivo declarado em `PLAN.md` §9.9.
- **Cinco remissões à numeração antiga:** mantidas e declaradas como divergência (P-12 do relatório de
  revisão), em vez de apagadas.
- **Extensões aposentadas** (escala, saúde, evento raro): saíram por não sustentarem resultado central; os
  números ficam no livro como registro citado.

## 5. Limites que não se fecham por trabalho

- **O auditor é parte interessada** (Capítulos 15 e 21): quem assina os pareceres é quem conduz o programa.
- **A incerteza knightiana** não se reduz com mais dado (Capítulo 4); o programa a declara e a cerca por
  cenários.
- **A tolerância do ECE depende de $n$** (Capítulos 15 e 20): o que fica é a regra que depende de $n$, e não
  um número único.

## 6. Ponteiros que a consolidação em Rust invalidou

A poda de 2026-10-16 levou o andaime Python e três relatórios que este inventário citava. As linhas de §1
apontam agora para o que ficou.

| Ponteiro antigo | Onde a coisa mora hoje |
| --- | --- |
| `book/revisao.md`, seção 5 | `DECLARACAO_CONSOLIDACAO_RUST.md` traz o que saiu e a nota de recuperação; `REGISTRO_EMPIRICO.md` traz a marca de cada número |
| `book/resultados-que-dependem-de-python.md` | `REGISTRO_EMPIRICO.md` (marca **P**) e `book/resultados-significantes.md`, a triagem que nasceu dele |
| `book/resposta-a-revisao.md` | saiu com a revisão; o que sobreviveu está nos capítulos e em `governanca/` |
| `book/labs/` e `book/edicao-14-capitulos.md` | saíram pela regra dos arquivos novos (`PLAN.md` §9.12): as 17 cópias intermediárias já estavam integradas em `crates/stochology-book/src/capNN.rs` |
| `dados-reais/h2*/`, `dados-reais/h3b/`, `dados-reais/h3c/` | ficaram as cinco bases que os instrumentos leem, com `dados-reais/FONTES.md` e `dados-reais/SHA256SUMS` |

## 7. Contagem

| Bloco | Antes | Depois |
| --- | --- | --- |
| 1. Teoria e invariantes | 4 | 0 como proposta de código; as previsões 3 e 5 são o item 8 |
| 2. Crates do roteiro | 11 | 0 — os dez nomes vivem desde 2026-10-16 (item 1, §2) |
| 3. Método declarado e não implementado | 6 | 0 — o Sistema 2 entrou em §2 e o restante estava fechado |
| 4. Pesquisa empírica em aberto | 4 | 3 (SHELF, genética, voz), nos itens 3 a 5 |
| 5. Registro sem segunda opinião | 12 resultados | 12, no item 12 |
| 6. Decisões de não fazer | 5 | 5 |
| 7. Limites estruturais | 3 | 3 |
| — Lista viva de proposta declarada | 7 | **10**: os itens 8 a 12 entraram em 2026-10-16 e os itens 1 e 2 saíram na mesma data |

### Verificação desta revisão

Rodada em 2026-10-16, nesta árvore, com o código já consolidado. Código de saída entre parênteses. A segunda
passada do dia fechou os itens 1 e 2: os números de `cargo test`, a linha do alvo `wasm` e a linha do
`labs --s2` são dela.

| Comando | Resultado |
| --- | --- |
| `cargo test --workspace` | 181 testes aprovados, nenhum reprovado: `core` 51, `s2` 29, `book` 23, `hypergraph` 12, `audit` 9, `schema` 9, `h0` 9, `icp` 8, `h2` 7, `h1` 6, `h3` 6, `drift` 5, `fairness` 4, `wasm` 3, `gates` 0 (0) |
| `labs --s2` | o Sistema 2 roda: o grafo declarado valida com 8 nós e 5 hiperarestas, o LATS chega em `f1.sintese-de-codigo` (o nó que nenhuma lente cobre) onde o ganancioso para em `h0.fila`, a crença da banca acerta A5 de primeira contra 5 ataques da ordem declarada, a posterior da Beta(4, 8) dá 0,3346 contra a forma fechada 1/3 e o eixo R escolhe R1-dia com a perda de 46,24 min (0) |
| `cargo build --release --target wasm32-unknown-unknown -p stochology-wasm` | o playground compila: `stochology_wasm.wasm`, 30 606 bytes (0) |
| `labs --todos` | os 21 laboratórios rodam (0) |
| `labs --banco` | o $H_0$ real roda sobre `dados-reais/bank_calls.csv` (0) |
| `labs --resolucao` | o horizonte R roda (0) |
| `labs --s1` | a triagem do Sistema 1 roda, com a árvore de sufixos e a probabilidade presa em $[0{,}01; 0{,}99]$ (0) |
| `labs --fila` | os dois motores da fila rodam: o exato reproduz 2 264, 2 261, 2 428 e 2 433 agente-horas e o evento a evento dá 28,4%, 16,0%, 4,9% e 8,6% de dias falhos, contra os 30,9%, 19,8%, 4,9% e 9,9% do motor exato e os 26%, 12%, 3,7% da alegação antiga, divergência já declarada no Capítulo 16 (0) |
| `portoes --horizonte todos --reproducao` | os sete portões aprovados (0) |
| `portoes --horizonte h3 --defeito h3.voos23` | prova negativa: reprova no G6, com a banca $A_1$–$A_8$ no pior valor 2 contra o piso 6 (1, como declarado) |
| `python3 book/gerar.py --check` | a rampa de três degraus dos 21 capítulos confere, com 5 567 linhas e 71 300 palavras (0) |
| `python3 book/conferir_labs.py` | 24 blocos citados, 0 divergentes (0) |
| `python3 book/conferir_livro.py` | partes, capítulos, cercas e ordem conferem (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` está atualizado (0) |

### Pendências abertas

1. **A conferência de D.2 a D.4 precisa dos arquivos baixados.** Eles não estão na árvore, e também não estão
   na pasta de downloads desta máquina. Sem eles, o caminho é conferir de novo contra DOI e página de editora,
   como em `referencias/CONFERENCIA.md`, e marcar o que ficar sem fonte como `[arquivo]`, não como
   `[conferido]`. As 27 linhas de D.2 e D.4 são as que dependem disso: as 23 de D.3 já estão cobertas.
2. **Os ponteiros de §6 não voltam.** Quem precisar dos três relatórios usa
   `git checkout 1bfec905 -- book/revisao.md book/resposta-a-revisao.md book/resultados-que-dependem-de-python.md`.
