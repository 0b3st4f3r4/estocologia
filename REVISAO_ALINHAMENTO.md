# Revisão de alinhamento — o que é fato medido, o que é método, o que é distração

Estado da árvore: 2026-10-17, HEAD `5888faa7`, 107 commits, `git status` limpo. Revisão por enxame de cinco
agentes sobre o livro, o aparato Rust, a governança, o declarado e o lado prático, mais execução própria da
verificação de `AGENTS.md` §V. Classifica o que existe em três faixas, mede o custo de reproduzir, lista as
contradições documentais e diz onde a custódia é de máquina e onde é de pessoa.

## 1. Critério de corte

Uma alegação só conta como **fato** se cumprir as cinco condições ao mesmo tempo:

1. dado real versionado em `dados-reais/`, com origem declarada em `FONTES.md`;
2. instrumento Rust que reproduz o bloco publicado;
3. margem na unidade independente, não em pseudorréplica (Φ2);
4. teste que podia errar — nulo simulado, placebo, cofre fora do tempo ou prova negativa;
5. bloco conferido por `book/conferir_labs.py`.

Faltando uma, cai para **demonstração**. Faltando o dado real, cai para **ornamento**. Não existe sexto
critério, e nenhuma quantidade de código compra a primeira condição.

## 2. Fato com dado real

Cinco bases, 84 256 linhas somadas. Três capítulos as leem.

| base | linhas | consumidor | resultado | comando |
| --- | --- | --- | --- | --- |
| `bank_calls.csv` | 27 717 | `book/banco.rs`, `book/fila.rs`, `book/segunda.rs` | 2 654 950 chamadas no cofre, 164 dias; Erlang C promete 0,8193 e entrega 0,5683 no pico e 0,6543 no dia; nulo 0,0441/dia e 0,0006/chamada; Var/média 272,6 | `labs --banco` |
| `djia_1885_2023.csv` | 37 932 | `book/cap17.rs` | crescimento real 5,15 / 7,19 / 6,01 %/ano em 1× / 2× / 3×; 1987 −22,61%; ruína em L≥4,42; mediana 2,10× contra média 19,1× | `labs --cap 17` |
| `nsw_experimental.csv` + `cps_observational.csv` | 446 + 15 993 | `book/cap17.rs` | viés de seleção −10 292; sorteio +1 794; placebo de 1975 inventa −12,3 pp; ATT causal mediano 0,0963 (IC90 0,0088–0,1879) | `labs --cap 17` |
| `danish_fire.csv` | 2 168 | `book/cap18.rs` | Hill 1,9625 / 1,8109 / 1,4100, IC [1,37;2,61]; deriva 4,6 %/ano, p=2e-6; silêncio de 36 meses vale 2,42 bits, chance 1/38; modelo do agente com habilidade de Brier −0,015 e proibido de decidir | `labs --cap 18` |
| `bank_calls.csv` | — | `h3/cadeia.rs` + `fundo/p14-h3/registro.tsv` | laço performativo 0,1446 [0,1320;0,1568], razão 4,1483 contra o 0,6 declarado | `labs --h3` |

**Fronteira medida:** `grep` em `crates/stochology-book/src/cap19.rs` não acha CSV. O capítulo que coroa o
programa com "causas verdadeiras na ciência" — túnel de luz, hERG, levedura, STAR — hoje é laboratório
simulado. O dado real dessa frente saiu com o andaime Python (commit `1bfec905`).

O que os fatos acima sustentam, sem enfeite: fila real não obedece à média; alavancagem não paga o que a
média promete; viés de seleção é maior que o efeito; cauda de 1,81 mata a variância e o silêncio não é
evidência; um modelo pode estar calibrado e ainda perder para a média histórica.

## 3. Método sem fato

Sete capítulos rodam mundos simulados com semente fixa: cap05 (fila 9,1 contra 19,3 min), cap07 (jardim de
caminhos 21,5 % e 47,0 %), cap08 (Brier 0,1858 contra 0,1569; Platt 0,1712→0,1559), cap11 (diques 3,06 a
21,49 m; EVPI +18,427), cap12 (Hill 1,79; validador em 4 provas), cap14 (ficha, funil, 7 defeitos → 7
portões), cap19 (ICP simulado).

Legítimo como demonstração de instrumento e pedagogia. Não é afirmação sobre o mundo, e o livro não separa as
duas coisas no texto.

## 4. Aparato sólido, e quanto custa

16 crates, ~27 000 linhas de Rust em 84 arquivos, **zero dependência externa** (`Cargo.lock` com 16 pacotes
locais, build offline), um só gerador SplitMix64 no núcleo (`passo_splitmix64` compartilhado), **371 testes e 0
falhas**, quatro binários (`labs`, `portoes`, `onda`, `ingestar-jev`).

Custo medido de verificação fria: `cargo build --release` 7,96 s e 16 MB; `cargo test --workspace` 64,63 s;
`labs --todos` 31,72 s com 487 linhas; `labs --fila` 13,61 s; `conferir_labs.py` 43,62 s;
`gerar_resultados.py --check` 103,52 s; os demais abaixo de 0,4 s. **Total ≈ 4 min 20 s**, dos quais 147 s
são os dois conferidores que reexecutam instrumentos.

Cinco travas de reprodutibilidade: Φ1 recusa ficha sem semente; G1 roda a passada duas vezes e compara a raiz
de Merkle; `--reproducao` exige relatório e raiz idênticos byte a byte; o motor ondulatório tem portão de
determinismo próprio; os conferidores reconstroem registro, blocos e lacres.

Prova negativa de verdade: `portoes --horizonte h3 --defeito h3.voos23` imprime `veredito: 1 portoes
reprovados` e **sai com 1**; `onda --defeito mutacao` e `--defeito inversao` também.

## 5. Aparato inerte — diagnosticado e corrigido em 2026-10-17

Os dez pontos abaixo foram medidos na revisão e atacados na mesma data, mantendo a raiz de Merkle (`9716e28a…`) e os 48 blocos do livro. **Saldo auditado depois da terceira passada: nove fecharam por inteiro (1 a 9), e o décimo é limite estrutural do instrumento, não defeito a corrigir.**

1. **A banca adversarial A1–A8 ganhou âncora estrutural, fronteira 5–6 e fichas históricas reprovadas no portão.** Antes `agentes.rs` fazia só
   `8 + rng(3)` contra `2 + rng(3)` e `gates/main.rs` carimbava `com_cuidados().completa()`. Agora `AgenteA::ancora_estrutural` confere o campo estrutural de cada agente
   na `Ficha` e separa quatro regimes (`8..=10`, `5..=6`, `3..=5`, `1..=3`); `gates/main.rs` não tem nenhuma chamada a `com_cuidados` (deriva os cuidados de `AgenteA::ancora_estrutural(&f)`) e avalia em todo G6 as três fichas reprovadas no histórico (`fichas_reprovadas_no_historico`: `h1.c-incendios.alavancar`, `h2.icp-levedura.1linha` e `h3.prisoes.toronto`), exigindo que a banca reprove todas.
2. **Os instrumentos `labs` e `portoes` passaram a reprovar flag inválida.** Ambos saem com `std::process::exit(1)`
   em flag desconhecida, `--cap` fora de 1..=21 ou erro de ensaio.
3. **Os crates `drift`, `fairness` e `h1` entraram no portão G6 e nos capítulos.** `portoes` exercita
   `stochology_drift::cusum`, `stochology_fairness::paridade_demografica` e `stochology_h1::hill` dentro do G6, e
   `stochology-book` consome `stochology-h1`, `stochology-audit` e `stochology-hypergraph`.
4. **Fórmulas desduplicadas em uma única implementação no núcleo.** `stochology_core::rng::passo_splitmix64` centraliza o passo do gerador em todos os
   módulos; `stochology_core::hill_no_corte` centraliza o estimador de Hill (consumido por `stochology_h1::hill`, `stochology_hypergraph::lentes::indice_hill`, `cap10.rs`, `cap12.rs` e `cap18.rs`); `stochology_core::decomposicao_murphy` centraliza a decomposição de Murphy (consumida por `stochology_hypergraph::lentes::decomposicao_murphy` e `stochology_h3::preteste::decomposicao_de_murphy`); `banco.rs` usa `stochology_core::Rng` e `stochology_h0::erlang_c`; `cap15.rs` delega
   SHA-256, Merkle e `LivroRazao` a `stochology_audit`.
5. **Campos mortos eliminados.** `teste_severo_fracao_simulada` foi removido de `MedidaBanco` (`banco.rs`);
   `OrquestradorM::conduzir_ataque` passou a validar `hash_trilha` (exigindo 64 hexadecimais quando preenchido, com
   teste de corrupção).
6. **Testes adicionados onde faltavam.** `stochology-gates` ganhou 3 testes unitários e os 17 módulos de capítulo
   (`cap05.rs` a `cap21.rs`) ganharam `#[cfg(test)] mod testes`, elevando a suíte de 350 para **371 testes**.
7. **Relógio físico removido do aparato.** `std::time::Instant` saiu de `crates/stochology-s2/src/ponte.rs` (zero
   ocorrências de `std::time` em `crates/`); o custo em milissegundos passou a ser reportado deterministicamente
   por `ModeloCongelado::ms()`.
8. **Base factual e governança lacradas.** `dados-reais/SHA256SUMS` passou a usar caminhos relativos à raiz, com
   `crsp_day_1989_1998.csv` declarado como ausente da árvore (`#`, espelhado em `FONTES.md`); `governanca/DECLARACAO.sha256`
   lacra os 7 arquivos de `governanca/`; e `conferir_lacres.py` confere **28 lacres, 82 alvos e 9 declarados fora
   da árvore, 0 problemas**.
9. **Automação de CI e conferência viva do artigo contra o instrumento.** `.github/workflows/ci.yml` executa a verificação obrigatória de
   `AGENTS.md` §V em cada push/PR, e `book/conferir_livro.py` executa `target/release/labs` (`--cap 5`, `--cap 16`, `--cap 17`, `--cap 18`, `--h3`, `--ponte`), extrai os números ao vivo da saída padrão, formata para a notação do artigo e confere `artigo/README.md`.
10. **Limite estrutural do portão de estilo.** Quando a árvore de trabalho coincide com `HEAD`, `conferir-passada.py`
    compara `X->X`; ele atua como guarda de regressão entre o índice/trabalho e `HEAD`.

## 6. O declarado e o entregue

Quatro itens vivos, e eles não são da mesma natureza.

| # | o que exige | bloqueia | prazo | estado medido |
| --- | --- | --- | --- | --- |
| 3 | painel SHELF completo, 3 especialistas por domínio, 1 de fora | gente (consentimento) | nenhum | 5 domínios elicitados só por E1, **n = 1**; habilidades 0,6500 / 0,9730 / 0,7425 / 0,8585 / 0,4240; **0 especialistas de fora** |
| 5 | escuta de ≥ 2 dos 7 grupos com registro assinado | gente (escuta) | **2026-12-15** | `labs --escuta`: 7 decisões, `ouvidos` = nenhum nas 7, cobertura **0 de 7**, veredito `CAIXA LACRADA` |
| 9 | escolhedor de casos e juiz humanos de fora, 12 casos | gente (julgamento cego) | **2027-01-14** | `labs --ponte`: `TESE NAO SUSTENTADA`, empate no teto, papéis `nao designado`, **0 de 12** |
| 13 | pesos congelados com `sha256` declarado antes, mais conjunto rotulado | máquina e operador | nenhum | `labs --jev`: `RECUSADO: HashNaoDeclarado`; `pesos/` e `calibracao/` **não existem** |

Os nove que fecharam em 2026-10-16 têm instrumento: dez crates do roteiro, Sistema 2, genética (4 de 40 com
uma linha, 1 de 40 com duas), bibliografia (50 de 50, registro e não instrumento), cadeia do H3, previsões 3 e
5 do eixo R, migrações 1.0→1.1→1.2, eixo R nos capítulos, segunda opinião (5 conferidas, 2 derivadas, 7
registro).

O Sistema 2 está maior do que os documentos diziam: **10 módulos e 81 testes** mais 7 de integração — não sete
módulos. E o motor ondulatório declara as duas divergências que o rascunho escondia, com a conta certa: na
antifase simétrica `ψ = (1,−1)/√2` é autovetor de `A` com autovalor 1, logo `ψ ∈ ker L` e `<ψ, Lψ> = 0`
exatamente quando o conflito existe; medido, a frustração dá 0,6667 no grafo construtivo **e** no destrutivo,
e o gatilho real é a coerência das chegadas.

## 7. Contradições reais — corrigidas em 2026-10-17

As vinte linhas abaixo foram corrigidas na árvore nesta data, com 38 arquivos modificados na primeira passada e os 10 pontos de §5 fechados na segunda, livro regerado
(5 927 linhas, 75 289 palavras), lacres re-selados e a verificação de §V verde: 371 testes, 48 blocos, 28
lacres e 82 alvos com 0 problemas, 7 portões ok, provas negativas saindo com 1. Ficam registradas porque a
declaração de divergência é o método do repositório: o valor antigo não se apaga.

| # | onde | diz | a árvore diz |
| --- | --- | --- | --- |
| 1 | 11 capítulos (9–18, 20) e docstrings de `cap16.rs:5`, `cap20.rs:6` | saída medida em `/tmp/labs-capN` | o crate não existe; `labs --cap N` reproduz os blocos |
| 2 | `PLAN.md:148` e tabela | "Quinze existem", 15 crates, sem `-wave` | 16 crates; `AGENTS.md` §VII diz dezesseis |
| 3 | `PROPOSTAS:46`, `INTEGRACAO_JEV.md:36` | `-s2` com sete módulos | 10 módulos |
| 4 | `PROPOSTAS:38`, `p20-jev/registro.tsv` | ponte 7 testes, runner 7, calibração 8 | 12, 9 e 9 |
| 5 | `fundo/p20-jev/README.md:7` | "não há runner local aqui" | `s2/src/runner.rs`, 397 linhas, 9 testes |
| 6 | `fundo/p12-shelf/README.md:5` | "nada rodou e nada foi elicitado … em branco" | 5 elicitações e 21 linhas de releitura |
| 7 | `cap21.md:162` | "nenhuma entrevista foi feita" | 5 yaml com habilidade medida; falta só o de fora |
| 8 | `cap21.md:164`, `PLAN.md:463` | genética "em aberto" | item 4 fechado em 2026-10-16 |
| 9 | `PROPOSTAS:113` tabela de contagem | lista 3, 5, 12 e ignora 9 e 13 | lista viva é 3, 5, 9, 13 |
| 10 | `PROPOSTAS:126,141` | 214 e 181 testes; 25 e 24 blocos | 371 testes; 48 blocos |
| 11 | `conferir_lacres.py:8` | "os 25 lacres" | 28 |
| 12 | `README.md:8` / `:41` / `RESULTADOS` | 24 instrumentos / 20 / 24 flags | a regex conta modificadores (`--escala`, `--limiar`, `--respostas`, `--gabarito`, `--medida`); são 19 instrumentos + 5 modificadores |
| 13 | `README.md:46` / `DECLARACAO:91` / `:110` | 64 / 70 / 65 alvos | 82 alvos em 28 lacres |
| 14 | `apendices.md:52` | Φ1–Φ4 = pertinência, contraste, justiça, saída | Φ1–Φ5 = simulabilidade, unidade, teste severo, escopo, resolução; sem verbete para Φ5 |
| 15 | `cap21.md:131` | quatro invariantes e quatro proibições | cap14 e `AGENTS.md` §I trazem cinco e outra lista |
| 16 | `cap16.md:264` | tabela R0–R3 sem instrumento em Rust | `fila.rs` é o instrumento, citado em `:218` |
| 17 | G1–G6 de `onda` e G1–G7 de `portoes` | mesmo nome | conteúdos diferentes, sem desambiguação em documento nenhum |
| 18 | `INTEGRACAO_JEV.md:31-49` | audita 12 alegações | omite os 4 substitutos do M15 e os "80 a 300 ms"; `banca.rs` implementa a FSM (G8.1–G8.4) e nenhum documento liga as duas |
| 19 | `GRUPOS.md:9-15` | ancora as sete linhas em `h2f/`, `h3/fluxos/*.py`, `h3/saida/*` | nenhum existe |
| 20 | `artigo/README.md` | 9,3 / 20,7 min; "70 %"; "45 %" | 9,1 / 19,3; 68,8 %; 45,1 — agora coberto por `book/conferir_livro.py` |

## 8. Prática: públicos, trilhas e o que falta

`00-abertura.md:22-26` declara quatro leitores, com trilhas em `:103-111`: Ensino Médio (cap. 1–8, só Degrau 1,
8 encontros), graduação (cap. 1–11, três degraus, um semestre), profissional de dados (cap. 1, 2, 5, 6, 8, 14,
15 e Parte V, quatro semanas), arquitetura (cap. 2, 12, 13, 14, 15, duas semanas). `AGENTS.md` usa três.

O laboratório tem anatomia fixa: cabeçalho com semente, gerador SplitMix64 delegando a `passo_splitmix64`, modelo nulo antes de ler
padrão (2 000 repartições em cap05; 4 000 dias e 2 000 repartições em cap16) e uma `rodar()` que só imprime. O
núcleo de Φ2 cabe em duas funções: `margem_por_dia` contra `margem_por_paciente`, a pseudorreplicação que o
nulo põe à prova — 0,51 contra 0,07 minuto.

Falta por público. Estudante: gabarito retirado de propósito, nenhum HTML, PDF, epub ou notebook. Docente:
nenhuma ementa, carga, cronograma, slide ou rubrica — o curso antigo foi obsoletizado e nada o substituiu.
Decisor: ninguém de fora assinou; sem sumário de uma página. Pesquisador: artigo sem veículo, DOI ou parecer.

## 9. Custódia: onde a máquina alcança e onde é pessoa

Verificável por máquina do **número ao lacre**: 48 blocos conferidos, 371 testes, 7 portões com raiz de Merkle,
6 portões ondulatórios, 28 lacres e 82 alvos de lacre (incluindo `dados-reais/SHA256SUMS` e `governanca/DECLARACAO.sha256`), 0 problemas.

**Humana** em tudo o que decide o significado:

- `ASSINATURAS.md:3`: "Sil respondeu 'Assino' no thread do projeto". Sem chave criptográfica externa; o conteúdo em disco de `governanca/` agora está lacrado em `governanca/DECLARACAO.sha256`, mas a autoria continua declaratória.
- Os 21 pareceres: os `resultados.json` saíram da árvore; conferir exige `git archive 1bfec905`.
- A escuta: o portão confere campo, não proveniência; sete registros fabricados abrem a caixa com cobertura
  7 de 7. O `hash_da_transcricao` assinado "não existe hoje", declarado pelo próprio instrumento.
- Quem assina é parte interessada; K da releitura = 1.
- O campo contestado do p05: `reproducao_rodada_a` marca `true` em 8 de 33 num arquivo e 33 de 33 no outro;
  a frase publicada diz 33 de 33.
- O ambiente: `requirements.txt` é foto, não lock; dois numpys levam o G4 de 0,5632 a 0,5529.

## 10. Status da tese central

A tese — erro de julgamento é desalinhamento entre camadas — teve um teste direto, o piloto cego de 12 casos
com dois braços. **Empatou no teto:** 8 de 8 defeitos nos dois braços, 0 alarme falso nos 4 limpos. Empate no
teto não separa nada. O livro declara a reprovação, e a ponte em escala está arquivada como não sustentada.

Conclusão de alinhamento: a Estocologia, como está na árvore, não é uma ciência nova comprovada. É um método
de auditoria com currículo, e o que carrega de comprovado são os fatos da faixa 2 mais os resultados
metodológicos que sobrevivem a teste — unidade independente muda a margem, média esconde a espera, cauda de
1,81 mata a variância, calibração se conserta e resolução não.

## 11. Regra de decisão

**Manter como fato:** cap16, cap17, cap18 e o laço do cap20, com os cinco comandos da faixa 2.

**Rotular como demonstração:** cap05 a cap15 e cap19, dizendo no texto que o mundo é sintético e que o dado
real do cap19 saiu com o Python.

**Feito em 2026-10-17:** as 20 contradições da seção 7 e os 10 pontos de aparato inerte da seção 5. Verificado por execução, não por leitura.

**Fechar o que código não fecha:** 0 de 7 grupos ouvidos até 2026-12-15, 0 de 12 casos na ponte até
2027-01-14, SHELF com n = 1. Sem isso, o programa tem instrumento e livro, e não tem campo.

## 12. Verificação executada nesta revisão

Reexecutado nesta máquina após o fechamento de §5 e §7:

```text
cargo test --workspace                                  371 aprovados, 0 reprovados
portoes --horizonte todos --reproducao                  7 portoes ok, raiz 9716e28a..., hash 08e7c8f0...
portoes --horizonte h3 --defeito h3.voos23              1 portao reprovado, exit 1
onda --check                                            6 portoes ok
labs --todos                                            exit 0, 487 linhas
labs --flag-que-nao-existe                              exit 1 (flag desconhecida recusada)
conferir_lacres.py                                      28 lacres, 82 alvos, 9 fora, 0 problemas
book/gerar.py --check                                   5927 linhas, 75289 palavras, disco == montado
book/conferir_labs.py                                   48 blocos, 0 divergentes
book/conferir_livro.py                                  ok (incluindo artigo/README.md)
gerar_resultados.py --check                             registro atualizado
estilo/conferir-passada.py                              26 arquivos, 0 regressoes
```
