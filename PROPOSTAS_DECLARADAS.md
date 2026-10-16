# Tudo o que ainda está como proposta declarada

Inventário revisto em 2026-10-16. A revisão conferiu cada linha no código desta árvore, e não na redação
anterior: o que já existe aparece com o arquivo, a linha e o teste que o sustentam, e o que falta aparece com
o que precisaria existir para fechar. Cada ponteiro foi aberto antes de entrar na tabela, e um deles não
existia, o que gerou §6 e o conserto das linhas que dependiam dele.

A lista viva tem quatro itens (§1). São os sete originais, mais as cinco frentes que estavam fora dela e
entraram por decisão do autor em 2026-10-16, mais a ponte Jev-LLM do item 13, que entrou na mesma data, menos
o item 1 — os dez crates do roteiro —, o item 2 — o Sistema 2 deliberativo —, o item 4 — a resolução fina da
genética —, o item 6 — a conferência da bibliografia —, o item 7 — a reexecução da cadeia do H3 — e o item 8 —
as previsões 3 e 5 do eixo R, o item 10 — as migrações de contrato —, o item 11 — o eixo R nos capítulos — e o item 12 — a segunda opinião do registro: os nove passaram a viver com a prova em §2. Os itens que a consolidação em Rust fechou saíram para §2, com a
prova ao lado, e §3 registra as decisões do dia.

## 1. O que continua como proposta declarada

Os números dos itens são os da revisão anterior, para o registro não perder a referência: os itens 1 e 2
saíram da lista em 2026-10-16 e a numeração de 3 a 12 continua como estava. O item 13 entrou depois da
revisão, também em 2026-10-16, ao receber a transcrição *Integração de Modelos Jev à Estocologia*: a auditoria
de cada alegação dela contra o código está em `INTEGRACAO_JEV.md`.

| Proposta | Onde está declarada | O que falta |
| --- | --- | --- |
| **3. Elicitação humana SHELF** | Capítulos 15 e 21; `fundo/p12-shelf/` (protocolo congelado com as declarações 1, 3 e a nota de execução; 6 formulários) e `governanca/RELEITURA.md` (21 linhas preenchidas) | **o painel completo.** A trilha individual do primeiro especialista (E1, o autor do programa) está fechada em 2026-10-16: **cinco domínios elicitados de ponta a ponta** — `h2/desenvolvimento_preston`, `h2/direito_prisoes`, `h2/educacao_star`, `h2g/direito_teto_beneficio` e `h1-real/c` —, cada um com pré-teste (20/20, 20/20, 20/20, 20/20 e 16/20; habilidades 0,6500, 0,9730, 0,7425, 0,8585 e 0,4240), cinco quantis confirmados na rodada 1, controles lidos e a comparação preenchida até o funil, com a marca **P** nas três prioris de cada domínio (`formularios/comparacao.md`, seções 6 a 10). `h2g/educacao_star_turmas` fica **reservado**, pela exclusão do par de mesma base. Faltam as outras duas pessoas por domínio e, em todas, o **controle de fora do programa**, que é o que impede a elicitação de substituir a priori do pacote na rodada oficial. A releitura dos 21 pareceres foi feita: 17 confirmados, 4 retirados e reemitidos com nota assinada |
| **5. A voz de quem paga o erro** | Capítulos 11, 15, 20 e 21; `governanca/GRUPOS.md`; `fundo/p13-escuta/` | **escuta, e só ela.** O protocolo está congelado com hash (`fundo/p13-escuta/DECLARACAO.sha256`) com o roteiro em linguagem comum, quem conduz de fora do programa ou com protocolo cego e a regra de não substituição; o instrumento `labs --escuta` mede o lacre: **0 de 7** grupos ouvidos, cobertura mínima de **2 de 7** e prazo em **2026-12-15** — as sete decisões ficam como análise de sensibilidade ($n = 1$) e não viram diretriz de política pública nem decisão de portão F8 até a cobertura, ou ficam arquivadas como exercício metodológico. A coluna "ouvidos" diz **nenhum** nas sete, e nenhuma coorte histórica está ao alcance: o que se procura é equivalente estrutural. Falta ouvir |
| **9. A tese da ponte em escala** *(entrou em 2026-10-16)* | Capítulos 1 e 2; `fundo/p01-tese/`; `fundo/p16-ponte/` | **o teste justo, e ele depende de gente.** O piloto segue **não sustentada** pela regra declarada, e o ponto 16 tornou isso verificável: o `labs --ponte` re-deriva o empate no teto das notas do juiz cego (8 de 8 defeitos nos dois braços, 0 de 4 alarmes falsos, 0 pares discordantes) e **trava a tese** — um registro que diz `sustentada` sem o teste justo é reprovado, e há teste para isso. Protocolo congelado com hash (`fundo/p16-ponte/DECLARACAO.sha256`): escolhedor dos casos **de fora do programa**, dois analistas humanos por braço com um de fora, juiz humano de fora e cego ao braço, e 12 casos mais difíceis que a lista de 2019. **Falta gente:** os quatro papéis estão `não designado` e os casos em 0 de 12. Janela até **2027-01-14**; vencida sem a cobertura mínima, a tese fica não sustentada em definitivo. As cinco perguntas do protocolo foram respondidas pelo dono em 2026-10-16 e estão registradas em `fundo/p16-ponte/DECLARACAO_2.md`: não há ninguém de fora para escolher os casos, não há equipe de analistas e não há juiz externo; a **regra mecânica de escolha** foi aceita integralmente (`regra_de_escolha.md`, lista externa com hash antes da escolha, filtro por catálogo público de replicação, semente 20270114, 8 com defeito e 4 limpos, e a lista que não sustenta a proporção reprova a escolha) e o veredito definitivo foi aceito **sem ressalvas** |
| **13. Ponte Jev-LLM (Sistema 1 sobre pesos abertos congelados)** *(entrou em 2026-10-16)* | `PLAN.md` §4, o cabeçalho de `crates/stochology-core/src/s1.rs` e `INTEGRACAO_JEV.md` | o runner local com pesos congelados e o `sha256` dos pesos, a decodificação restrita ao esquema da ficha, a prova de determinismo na forma do G1 (duas execuções, hash idêntico), a calibração contra o Brier do Jev determinístico, o custo por chamada e a pergunta paralela de adequação do espaço de hipóteses. A abstenção já existe em código: `Categoria::Outros` em `s1.rs` |

## 2. Fechado depois do inventário anterior

Oito linhas do inventário anterior, agrupadas aqui em sete itens, passaram a ter código, teste e comando.
Ficam como registro, e não como proposta.

| Item | Prova no código | Comando |
| --- | --- | --- |
| **Sistema 2 deliberativo (item 2)** | `crates/stochology-s2`, com sete módulos testados: `lats` (busca com UCB e o ganancioso que serve de nulo), `crenca` (estado de crença com piso e teto), `hipergrafo` (o mundo sobre `Grafo`, com o valor de cada nó medido em lentes que o cobrem e o nó sem lente como alvo), `inferencia` (Metropolis com alvo declarado), `estresse` (a banca A1-A8 atacada na ordem que a crença aprendeu) e `planejamento` (o eixo R decidido pelo instrumento). A síntese de código (F1) fica declarada e não implementada | `cargo test --workspace` e `labs --s2` |
| **Crates do roteiro (item 1)** | os dez nomes que faltavam vivem, cada um com lib, teste próprio e dependência declarada: `stochology-schema` (contrato em arquivo, versão e cadeia de migrações), `-s2` (LATS e estado de crença), `-h0` (Erlang e escala mínima, que passaram ao núcleo em 2026-10-16 e são re-exportadas por ele, nulo de Poisson, resumo obrigatório), `-h1` (arrasto, viés de seleção, Hill), `-h2` (rodadas, contador $K$, Bonferroni, nulo por embaralhamento), `-h3` (relógio estrito com canário, ECE, Brier, laço performativo), `-icp`, `-drift` (CUSUM e KL), `-fairness` (paridade, oportunidades, ECE por subgrupo) e `-wasm` (ponte C e a página de `www/`). Sem dependência externa | `cargo test --workspace` e `cargo build --release --target wasm32-unknown-unknown -p stochology-wasm` |
| **$\Phi_5$ e o campo de resolução na ficha** | `invariantes.rs:12,110` com 10 testes no arquivo, entre eles `acao_agregada_contra_unidade_dia_reprova_phi5`; campo `resolucao` em `camada.rs:52`, troca em `camada.rs:98`; `portoes` conta $\Phi_1$–$\Phi_5$ em `crates/stochology-gates/src/main.rs:316`. Limite declarado: a trava só morde com unidade igual ou mais grossa que o dia | `cargo test --workspace` |
| **Sistema 1 determinístico** | `crates/stochology-core/src/s1.rs` (877 linhas, 12 testes), vocabulário fechado, árvore de sufixos e probabilidade presa em $[0{,}01; 0{,}99]$ | `labs --s1` |
| **Funil de quatro filtros e contador $K$** | `crates/stochology-core/src/funil.rs` (878 linhas, 17 testes), com a poda por valor da informação e `o_funil_conta_as_tentativas_ate_a_versao_final` | `cargo test --workspace` |
| **Cofre fora do tempo** | `crates/stochology-audit/src/cofre.rs` (705 linhas, 6 testes): abertura única, adulteração invalida a verificação e errar depois da abertura exige cofre novo | `cargo test --workspace` |
| **Fila com desistência e a tabela $H_0$ de R0 a R3** | `crates/stochology-book/src/fila.rs` (1 113 linhas, 8 testes); `REGISTRO_EMPIRICO.md` §H0 marca **R** nos agente-horas e **D** nas falhas por dia, com a divergência declarada no Capítulo 16 | `labs --fila` |
| **Resolução fina da genética (item 4)** | `crates/stochology-h2/src/genetica.rs` (290 linhas, 6 testes próprios: o piso cai com a potência das linhas, o intervalo é um ponto sem a segunda linha, todo efeito ganha faixa com ela, o artefato de linha passa na validação de uma linha, a segunda linha separa o artefato do mover e o desenho é determinístico) e a leitura em `crates/stochology-book/src/genetica.rs`. O desenho planta dois casos — o mover verdadeiro, com efeito nas duas linhas, e o artefato de linha, com efeito numa só — e roda os dois arranjos sobre o mesmo mundo: com uma linha por mutante passam **4 de 40** no piso de $0{,}001$ (o mover, o artefato e dois de ruído, que o p não separa), e com duas passa **1 de 40**, com o piso caindo para $10^{-6}$ e a estimativa deixando de ser um ponto. O livro mudou junto: `book/cap19.md` (Degrau 1, o bloco do instrumento no Degrau 3 e o exercício 3) e `book/cap21.md` (as três passagens) | `cargo test -p stochology-h2` e `labs --genetica` |
| **Conferência da bibliografia (item 6)** | as 50 linhas de D.2 a D.4 levam a marca **[conferido]** em `book/apendices.md`, e a conferência das 27 linhas novas está em `referencias/CONFERENCIA.md`, na seção "As 27 linhas de D.2 e D.4 (conferência de 2026-10-16)": uma a uma contra a API do Crossref, as páginas das editoras, o PubMed, os anais do PMLR (`lipton18a`, primeira e última página) e catálogos de biblioteca (Chicago, CiNii, HathiTrust, Internet Archive, kilthub do CMU). Resultado: nenhuma divergência de conteúdo nas 27; onze complementos de completude, que são os DOIs que faltavam em dez linhas mais o número do *working paper* do Sweeney; e duas ressalvas de fonte declaradas (a página final do Chernoff de 1954 e a do van Dantzig de 1956, e o ano do Sweeney, que vem do registro do CMU). O Apêndice D.6 registra o estado | as fontes estão linha a linha na tabela de `referencias/CONFERENCIA.md` |
| **Reexecução da cadeia do H3 (item 7)** | `crates/stochology-h3/src/cadeia.rs` (7 testes próprios) e a leitura em `labs --h3`, com o registro em `fundo/p14-h3/registro.tsv` e as cinco declarações em `fundo/p14-h3/DECLARACAO.md`. As quatro linhas do H3 ganharam classe: **medida** (o dado bruto está na árvore), **derivada** (o número está num arquivo versionado, e o instrumento refaz a conta e o veredito) e **registro** (o dado bruto saiu da árvore). Uma linha é derivável — o laço performativo, cujo `fundo/p06-laco/resultados.json` está versionado: a re-derivação devolve r = 0,1446378859145094, IC90 de 0,131981188365084 a 0,15683852512801463, F = 5523,46, razão de 4,148290720694298 fora da banda declarada de 1/3 a 3, e o mesmo veredito da frente, com zero divergência entre a conta e o registro. Três linhas ficam declaradas, com o motivo: os diários dos seis fluxos e do cofre de Rhode Island e o grafo do H3 saíram com o andaime `h3/` (`DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`); para elas existe o mecanismo no Capítulo 20, e mecanismo medido não é cadeia medida. A legenda do registro ganhou a marca **V**, declarada sem contagem em K porque nenhum critério de decisão mudou | `cargo test -p stochology-h3` e `labs --h3` |
| **Previsões 3 e 5 do eixo R (item 8)** | `crates/stochology-core/src/resolucao.rs` ganhou as duas previsões (6 testes próprios) e o `labs --resolucao` as imprime junto com a perda de resolução dos três casos. **Previsão 3:** com a escala de Erlang C do dia médio (12 agentes) a previsão **não se sustenta** — a demanda plana já falha 1 dia em 5 (0,205), a fração não é monótona (0,260 em 0,5 contra 0,255 em 1,0) e no cruzamento refinar custa (−1,89 min); com a escala calibrada na demanda plana (13 agentes, no máximo 10% de dias falhos sem oscilação), ela **se sustenta** — a fração cresce de 0,095 a 0,330, cruza o quarto na amplitude **2,5** e a perda vira **+2,87 min** no cruzamento, com perda negativa em todas as amplitudes abaixo dele. A calibração é especificação nova, declarada com hash, e entra no **K da frente: K = 2**. **Previsão 5:** medida em 6 perfis × 200 dias, com **0 violações** da implicação e **3 contraexemplos**, e a conservação do agregado conferida (a soma dos dias reconstrói o agregado; um agregado que não bata é reprovado). O critério de dia falho (mais de 10% acima de 30 min) é declarado no ponto 15, porque o documento do eixo fixava o quarto e não o dia | `cargo test -p stochology-core` e `labs --resolucao` |
| **Migrações de contrato (item 10)** | a cadeia saiu do código e virou arquivo: `crates/stochology-schema/migracoes.tsv`, embutido na compilação com `include_str!`, com dois elos contínuos — 1.0 → 1.1 acrescenta `resolucao = R0-agregada` (o campo nasceu com o Φ5, e a migração não inventa pergunta mais fina que a declarada) e 1.1 → 1.2 acrescenta `priori_elicitada = false` (o campo nasceu com a frente SHELF) — e a versão da ficha passa de 1.0 para **1.2**. O leitor `cadeia_de_texto` reprova buraco entre versões, operação desconhecida, etapa que anda para trás e linha curta; o `labs --schema` imprime a cadeia e demonstra a migração de um documento da 1.0 (declarada como reconstrução da forma antiga, porque não existe ficha real na 1.0 na árvore). Nenhum número publicado muda: as 12 fichas do `portoes` são construídas em código, na versão corrente | `cargo test -p stochology-schema` e `labs --schema` |
| **Eixo R nos capítulos (item 11)** | a tabela `book/eixo-r.tsv` declara, por capítulo, um caso e uma frase editorial, e o `labs --resolucao` **calcula** a leitura de cada um e imprime **21 linhas**, uma por capítulo: perda de 46,24 min (demanda de 5 a 11) nos capítulos 5, 16 e 20, 5,97 min (7 a 9) no 8, 0,00 min (demanda plana) no 10 e no 18, e o limiar de oscilação na amplitude 2,5 no 21; os outros catorze declaram que a perda **não é medida ali**, porque o exemplo do capítulo não tem a mesma decisão em dois grãos. Cada Degrau 3 ganhou o mesmo parágrafo curto e o bloco com a sua linha, e o conferidor do livro passou de 27 para **48 blocos conferidos, 0 divergentes** — nenhum número do eixo foi escrito à mão no capítulo. O eixo entra como linha, não como laboratório novo. K = 1, declarado no ponto 18 | `labs --resolucao` e `python3 book/conferir_labs.py` |
| **Segunda opinião do registro (item 12)** | `crates/stochology-book/src/segunda.rs` (6 testes próprios) e o `labs --segunda`, com o registro em `fundo/p19-segunda-opiniao/registro.tsv`. Das catorze linhas marcadas **P**: **cinco conferidas** por segunda implementação — os limiares `α/M` do p03 recalculados como 0,05/M na forma exata do arquivo (0,05/32, 0,05/38, 0,05/72); o viés do previsor ingênuo do laço, que aparece em **dois arquivos independentes** e concorda dígito por dígito (0,29124674633711956 no p03 e no p06); a base do banco recontada em Rust (164 dias, cofre 2 654 950 chamadas contra 2 649 354, Var/média entre dias de 261,7 e 247,4 contra 261,1 e 246,7, com a diferença de 0,2% atribuída à janela de horas do laboratório); os pareceres contados (30 linhas na releitura e 4 arquivos reemitidos); e a contagem da reprodução do p05. **Duas derivadas** (a levedura no ponto 14 e o piloto no ponto 16) e **sete que só restam como registro**, todas porque o dado bruto saiu da árvore com o andaime. **Uma divergência fica declarada e não resolvida**: o campo em máquina `reproducao_rodada_a` tem 33 domínios e marca `true` em 8, enquanto a frase publicada fala de 33 de 33 pelo critério da banda de 0,05 dp — o campo não declara o próprio sentido, e a decisão é humana. O item fecha **com o limite dentro**, como o Φ5: não há segunda opinião para o que não tem dado. K = 1 | `cargo test -p stochology-book` e `labs --segunda` |

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
| 4. Pesquisa empírica em aberto | 4 | 2 (SHELF e voz), nos itens 3 e 5 — a genética fina saiu em 2026-10-16 (item 4, §2) |
| 5. Registro sem segunda opinião | 12 resultados | 12, no item 12 |
| 6. Decisões de não fazer | 5 | 5 |
| 7. Limites estruturais | 3 | 3 |
| — Lista viva de proposta declarada | 7 | **4**: os itens 8 a 12 entraram em 2026-10-16, os itens 1, 2, 4, 6, 7, 8, 10, 11 e 12 saíram nela e o item 13 entrou nela |

### Verificação do fechamento do item 4

Rodada em 2026-10-16, depois de a resolução fina da genética entrar no código e no livro. Código de saída
entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test --workspace` | 214 testes aprovados, nenhum reprovado: `core` 51, `h3` 33, `s2` 29, `book` 23, `h2` 13, `hypergraph` 12, `audit` 9, `h0` 9, `schema` 9, `icp` 8, `h1` 6, `drift` 5, `fairness` 4, `wasm` 3, `gates` e `labs` 0 (0) |
| `labs --genetica` | 40 mutantes com dois casos plantados: uma linha por mutante aprova 4 de 40 no piso de $0{,}001$; duas linhas aprovam 1 de 40, o piso cai para $10^{-6}$ e o artefato de linha sai da lista porque o efeito não se reproduz na segunda (0) |
| `python3 book/gerar.py --check` | a rampa de três degraus dos 21 capítulos confere, com 5 652 linhas e 72 268 palavras (0) |
| `python3 book/conferir_labs.py` | 25 blocos citados, 0 divergentes — o bloco 2 do Capítulo 19 confere em `labs --genetica` (0) |
| `python3 book/conferir_livro.py` | partes, capítulos, cercas e ordem conferem (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` está atualizado, com o bloco novo da genética (0) |

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
