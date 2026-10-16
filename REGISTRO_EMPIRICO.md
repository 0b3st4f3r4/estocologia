# Registro empírico consolidado

Os números da pesquisa, com o instrumento Rust que os reproduz hoje. As frentes que os mediram em Python
(` h0-lab0/`, ` h0-real/`, ` h1/`, ` h1-real/`, ` h2/` a ` h2g/`, ` h3/`) saíram da árvore em 2026-10-16: o
andaime de medição foi apagado e o número ficou aqui e no livro. O original volta com
`git checkout 1bfec905 -- <pasta>`, e o manifesto `registro-python-removido.txt` traz o hash de blob de cada
arquivo removido.

Legenda: **R** o instrumento Rust mede de novo; **D** divergência declarada no capítulo, com os dois valores;
**P** só registro (o número está aqui e no capítulo, sem instrumento em Rust); **V** o veredito é re-derivado em
Rust a partir do registro versionado (a medida continua sem instrumento — declaração 2 de `fundo/p14-h3/`).

## H0 — a fila e a central de chamadas

| Resultado | Marca | Fonte / instrumento |
| --- | --- | --- |
| Posto: mesma média de 8 por hora dá 9,3 min de espera no mundo constante e 20,7 min no mundo que oscila entre 5 e 11; 8 e 25 em cada 100 pacientes passam de 30 min | **D** | `labs --cap 16` mede 9,1 e 19,3 min, com a divergência declarada |
| Unidade independente da fila: margem por dia de 0,51 min contra 0,07 por paciente | **R** | `labs --cap 16` |
| Central do banco: 2,65 milhões de chamadas, 83 dias de treino e 81 de cofre, pico de 234 atendentes | **R** | `labs --banco`, lendo `dados-reais/bank_calls.csv` |
| Erlang C promete 82% no perfil médio e entrega 57% no dia real, com 65% dos dias abaixo da meta | **R** | `labs --banco`: 0,8193, 0,5683 e 0,6543 |
| Dispersão entre dias: Var/média de 273 e desvio de cerca de 2 900, contra 180 de Poisson | **D** | `labs --banco` mede 272,6 no treino (246,7 no cofre) e 2 841 contra 181 |
| Tabela de decisão R0 a R3: 26% dos dias falham na escala da alegação, 12% com o perfil do dia da semana, 3,7% com a margem p90, por +7,2% de agente-horas | **R** / **D** | `labs --fila`: os agente-horas batem (2 264, 2 261, 2 428, 2 433) e a ordem se sustenta; as falhas medidas são 30,9%, 19,8%, 4,9% e 9,9%, com a divergência declarada no Capítulo 16 |

## H1 — a bolsa, o emprego e o fogo

| Resultado | Marca | Fonte / instrumento |
| --- | --- | --- |
| Dow Jones: 5,1% / 7,2% / 6,0% ao ano em 1×, 2× e 3×; arrasto de 0,1403; −22,6% em 1987 destrói 68% de quem estava a 3×; mediana 2,10× contra média 19,1× | **R** | `labs --cap 17`, lendo `dados-reais/djia_1885_2023.csv` |
| LaLonde: comparação crua de −8 498 dólares contra o sorteio de +1 794, viés de seleção de −10 292, e o placebo de 1975 com −12,3 pontos | **R** | `labs --cap 17`, lendo `nsw_experimental.csv` e `cps_observational.csv` |
| Coortes de 20 anos: 1 134 coortes que valem cerca de 5,8 blocos independentes; 64,3% terminam acima do início | **P** | registro; correlação intracoorte não tem instrumento em Rust |
| Incêndios dinamarqueses: 2 167 sinistros, camada atingida em 4/1/0 meses por triênio, 152,4 M em agosto de 1989, Hill 1,81 [1,41; 2,56], 2,2 bits, 1/38 | **R** / **D** | `labs --cap 18`, lendo `dados-reais/danish_fire.csv`; o IC do Hill muda com a reamostragem |
| Habilidade de Brier de −0,015 para o modelo do agente e crescimento de 4,6% ao ano nos incêndios | **P** | registro; o teste fora do tempo é do roteiro antigo |

## H2 — as sete rodadas de invariância causal

| Resultado | Marca | Fonte / instrumento |
| --- | --- | --- |
| Túnel de luz: 128 conjuntos aceitos de 1 024, com os três LEDs red, green e blue | **P** | `labs --cap 19` reproduz o método em mundos simulados, não os dados originais |
| Farmacologia (FDA/hERG): 35 de 512 conjuntos, três canais iônicos e o verapamil previsto com habilidade +0,57 | **P** | registro |
| Genética de levedura: 40 genes, p = 0,001, réplica de um gene validado | **P** | registro; os 40 genes reais seguem sem instrumento, e o **desenho** da réplica é medido na linha abaixo |
| Resolução da validação por intervenção: com uma linha por mutante passam 4 de 40 no piso de 0,001 (o mover, o artefato de linha e dois de ruído, que o p não separa); com duas linhas passa 1 de 40, o piso cai para 0,000001 e a estimativa deixa de ser um ponto | **R** | \`labs --genetica\` |
| Educação (STAR): 11 598 alunos em 80 escolas; por aluno a área sobrevive, por escola cai para conjunto vazio | **P** | registro; é o segundo teste da unidade independente |
| $K = 38$ testes distintos (32 a 72), com Bonferroni mantendo 4 áreas antes da correção e 3 depois | **P** | registro; contador de tentativas é proposta declarada para Rust |

## H3 — a malha contínua

| Resultado | Marca | Fonte / instrumento |
| --- | --- | --- |
| Seis fluxos sob relógio estrito, zero vazamento com canário plantado de propósito | **P** | o dado bruto dos fluxos saiu com o andaime `h3/`; o **mecanismo** é medido em `labs --cap 20` (canário com o relógio honesto e com o relógio desligado) e a classe está declarada em `fundo/p14-h3/registro.tsv` |
| Cofre de Rhode Island: 24 466 paradas, ECE 0,0048, habilidade +0,019 | **D** | `labs --cap 20` mede o mecanismo em simulação (ECE 0,0022), o capítulo declara a diferença e `labs --h3` declara a classe: o diário de Rhode Island saiu da árvore |
| Goodhart medido por instrumento: 0,145 [0,132; 0,157], quatro vezes menos que os 0,6 declarados | **P** / **V** | a medida é registro (é resultado de campo e o dado bruto não está versionado); o veredito do critério declarado e a razão de 4,1483 são **re-derivados** de `fundo/p06-laco/resultados.json` por `labs --h3`, com zero divergência |
| Hipergrafo: 454 178 nós no H3, dos quais 453 973 previsões e 205 de topologia | **P** | o grafo do H3 saiu com o andaime; o hipergrafo do Rust é o validador, não o grafo do H3. Classe declarada em `fundo/p14-h3/registro.tsv` |

## Eixo R — a resolução da pergunta

| Resultado | Marca | Fonte / instrumento |
| --- | --- | --- |
| Perda de resolução: 46,24 min onde a demanda oscila de 5 a 11, 5,97 min onde oscila de 7 a 9 e 0,00 min onde não oscila | **R** | `labs --resolucao`; Capítulos 16 e 21 |
| Previsão 3 (limiar de oscilação): com o plano calibrado na demanda plana a fração de dias falhos cresce de 0,095 a 0,330 e cruza o quarto na amplitude 2,5, com a perda virando +2,87 min no cruzamento e negativa abaixo dele; com o plano de Erlang a previsão **não se sustenta** (a demanda plana já falha 1 dia em 5, a fração não é monótona e no cruzamento refinar custa) | **R** / **D** | `labs --resolucao`; ponto de fundo 15, com K = 2 declarado |
| Previsão 5 (transferência): 0 violações da implicação (nenhum perfil sobrevive a R1 e falha em R0) e 3 contraexemplos em 6 perfis × 200 dias, com a conservação do agregado conferida | **R** | `labs --resolucao`; ponto de fundo 15 |

## Auditoria de fundo

| Resultado | Marca | Fonte |
| --- | --- | --- |
| Piloto cego de 12 casos: empate no teto, com 8 de 8 defeitos nos dois braços e 0 alarme falso | **P** / **V** | as notas do juiz cego (`fundo/p01-tese/juiz/notas.yaml`) são **re-derivadas** por `labs --ponte`, que devolve os mesmos 8 de 8, 0 de 4 alarmes falsos e 0 pares discordantes; o portão do ponto 16 trava a tese em **não sustentada** até o teste justo, com o motivo declarado (faltam escolhedor de casos, analistas e juiz humanos) |
| Multiplicidade: $K = 38$; com Bonferroni, o H2g mantém 4 áreas antes da correção e 3 depois | **P** | registro |
| Prioris: três rodadas em 33 domínios, desvio máximo de 0,0071 dp, 33 dos 33 idênticos | **P** | `fundo/p05-prioris/` |
| Laço performativo: 0,145 medido contra 0,6 declarado | **P** | `fundo/p06-laco/` |
| Escala, evento raro e saúde: pontos 9 e 10 e as extensões | **aposentado** | saíram da árvore em 2026-10-16 (`DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`) |
| Elicitação humana (SHELF), quinto domínio: `h1-real/c` — pré-teste com 16 de 20 e habilidade 0,4240 (o mais fraco dos cinco, e positivo), quintis confirmados (Beta de a = 1,168188 e b = 9,911852, mediana 0,08) | **R** / **P** | pré-teste e ajuste em Rust — e o ajuste Beta do Python aposentado (`a = 1,168188`, `b = 9,911852`) **confere com o ajuste Beta em Rust** (`a = 1,171038`, `b = 9,919921`, diferença de 0,24% e 0,08%), segunda opinião de duas implementações independentes; o funil com a priori elicitada roda a cópia aposentada do p05 (**P**, semente 2226) e o projeto **passa** (`modelo_vence_climatologia` verdadeiro), como já passava com a fraca Beta(1;1) e ao contrário do painel simulado do p05; razão dp posterior/priori de **0,221084**, entre 0,872232 do painel simulado e 0,072434 da fraca. A literatura deste domínio é **sem fonte conferida** no próprio p05, então a elicitação ocupa um lugar vazio | `fundo/p12-shelf/` |
| Elicitação humana (SHELF), quarto domínio: `h2g/direito_teto_beneficio` — pré-teste com a maior habilidade dos quatro (0,8585, ECE 0,1375), quintis confirmados (média 0,35 e dp 0,222390) | **R** / **P** | pré-teste e ajuste em Rust; o funil com a priori elicitada roda a cópia aposentada do p05 (**P**, base `h2b`, semente 2127) e sai **idêntico à rodada do pacote até o último dígito** (ação `adotar`, P 1,0, média posterior 0,578765, razão dp 0,242860), porque os três quartis que o ajuste usa são os mesmos: a elicitação formal **reproduziu a priori do pacote** em vez de trazer priori nova. As pontas diferem (q05 0,05 contra 0,00; q95 0,65 contra 0,75) e só aparecem nos controles (razão da largura 90%: 0,8201 contra 1,0000). Painel incompleto: 1 de 3, e o especialista é o autor do programa | `fundo/p12-shelf/`, `governanca/RELEITURA.md` |
| Elicitação humana (SHELF), terceiro domínio: `h2/educacao_star` — pré-teste aprovado pela régua em vigor (habilidade 0,7425 e ECE 0,1525), quintis confirmados (média 12,0 e dp 8,8956, contra 12,0 e 5,9304 do pacote e 7,3332 e 3,6666 da literatura de **mesma base**) | **R** / **P** | pré-teste e ajuste em Rust; o funil com a priori elicitada roda a cópia aposentada do p05 (**P**): o domínio fica **aprovado no F4** e a ação segue `adotar`, com P(adotar compensa) 0,836500 contra 0,839625 do pacote, e a **priori de mesma base é podada no F2** ("dados na unidade independente não informam mais que a priori") enquanto a elicitada passa — a circularidade de usar a própria base como priori, medida pelo F2. EVPI: 2,640820 (elicitada), 1,502145 (pacote) e 0,497782 (literatura) | `fundo/p12-shelf/` |
| Elicitação humana (SHELF), segundo domínio: `h2/direito_prisoes` — pré-teste aprovado **nos dois portões** (habilidade 0,973 e ECE 0,0450), quintis confirmados (média −0,4639 e dp 0,4715 no log da razão de chances, contra −0,3546/0,2833 do pacote e −0,3482/0,1046 da literatura) | **R** / **P** | pré-teste, ajuste e controles em Rust (`labs --preteste` e `labs --elicitacao --escala log`); o funil com a priori elicitada roda a cópia aposentada do p05 (**P**): o domínio fica **reprovado no F4** nas três prioris — a elicitação não muda a ação — e o que ela move é o EVPI, de 0,541955 (pacote) e 0,009989 (literatura) para 1,054148 (elicitada), com a posterior indo de −0,729791 para −0,752532 e a razão dp post/priori de 0,275878 para 0,169906. Painel incompleto: 1 de 3, e o especialista é o autor do programa | `fundo/p12-shelf/`, `governanca/RELEITURA.md` |
| Elicitação humana (SHELF): um especialista entrevistado, no domínio `h2/desenvolvimento_preston`; pré-teste aprovado no portão ajustado (habilidade 0,65 e ECE 0,2250) e cinco quantis confirmados (média 3,25 e dp 2,2239, contra 6,5 e 2,2239 do pacote e 1,536 e 2,6034 da literatura) | **R** / **P** | pré-teste, ajuste e controles em Rust (`labs --preteste` e `labs --elicitacao`); a passagem pelo funil com a priori elicitada roda a cópia aposentada do p05, com a marca **P** — `P(adotar compensa)` 0,418875 e ação do F8 `manter`, contra 0,565375 (`adotar`) do pacote e 0,366375 (`manter`) da literatura, essas duas reproduzidas pelo mesmo comando. Painel incompleto: 1 de 3, e o especialista é o autor do programa | `fundo/p12-shelf/`, `governanca/RELEITURA.md` |

## Onde ficam as fontes

| Fonte | Onde está |
| --- | --- |
| Dados usados pelos instrumentos | `dados-reais/` (`bank_calls.csv`, `djia_1885_2023.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `danish_fire.csv`), com `FONTES.md` e `SHA256SUMS` |
| Números publicados, com o instrumento que os mede | este arquivo e `RESULTADOS_RUST.md` |
| Auditoria de fundo, governança e ética | `fundo/` e `governanca/` (mantidos) |
| Bibliografia conferida | `referencias/CONFERENCIA.md` e o Apêndice D do livro |
| Ancoragem do que saiu | `registro-python-removido.txt`, `python-removido.txt`, `nao-significante-removido.txt`, no commit `1bfec905` |
