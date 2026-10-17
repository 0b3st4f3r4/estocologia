# Declaração do estado da árvore

**Isto é um estado, e não uma edição.** A pesquisa e o livro não foram publicados. Antes de publicar não há edição
numerada, não há "congelamento" com número de ordem e não há segunda nem terceira edição: há um **estado da árvore**,
que este arquivo declara, e há o **registro datado** das correções que trouxeram a árvore até aqui, em `historico/`
— `estado-2026-10-17T0810Z.md` e `estado-2026-10-17T0840Z.md`, cada um com os hashes de antes e depois e com os
nomes que os arquivos tinham naquele momento. Numerar edições antes de publicar carimba de definitivo o que ainda se
corrige; quando houver publicação, aí a numeração passa a ter sentido.

O que este arquivo declara: o que a árvore tem, o que ela reprovou contra si mesma, o que ela não pôde fazer, o que
mudaria cada veredito e como tudo se reproduz. O manifesto com o hash de cada declaração de ponto está em
`RESULTADOS_RUST.md` §5, **gerado** por `python3 book/gerar_resultados.py` e conferido por `--check`. O selo
deste arquivo é `DECLARACAO_DO_ESTADO.sha256`.

O caminho até aqui, em três correções datadas, e cada conserto declarado no lugar próprio: (1) o livro montado ficou
353 linhas atrás de `book/` e o `gerar.py --check` passou a comparar o disco com o que o `montar()` produz (§6); o
lacre do `CEGAMENTO.md` estava vencido desde 2026-10-15 e a convenção de caminho dos lacres tinha duas regras (§3);
a receita de reprodução não rodava os instrumentos que provam os itens fechados (§5). (2) Uma contagem escrita à mão
sobreviveu ao crescimento do livro, e o `--check` ganhou a terceira cobrança, que lê o par "N linhas e N palavras"
nos quatro documentos canônicos. (3) Duas afirmações documentais caíram por medição — a de que as 50 linhas de D.2 a
D.4 levavam a marca, quando 27 a levavam, e a de que os três relatórios sairiam de `1bfec905` por `git checkout`,
quando `book/` não existe naquele commit —, e o registro de cada uma está em `referencias/CONFERENCIA.md` e
`DECLARACAO_PERDA_RELATORIOS.md`.

## 1. Os nove itens que fecharam, com a prova e o comando

| item | prova | comando |
| --- | --- | --- |
| Sistema 2 deliberativo | `crates/stochology-s2`, dez módulos testados (81 testes unitários e 7 de integração) | `cargo test --workspace` e `labs --s2` |
| Crates do roteiro | os dez nomes vivos, cada um com teste próprio | `cargo test --workspace` |
| Resolução fina da genética | `stochology-h2/src/genetica.rs`: 4 de 40 passam com uma linha, 1 de 40 com duas, piso de 0,001 para 0,000001 | `labs --genetica` |
| Conferência da bibliografia | as 50 linhas de D.2 a D.4 com **[conferido]**, 50 conferidas contra DOI, editora, catálogo ou anais, 0 divergência de conteúdo. **Correção de 2026-10-17:** esta linha dizia "50 linhas com a marca" e "27 conferidas", e as duas coisas juntas não podiam ser verdade — a marca valia em 27, e as 23 de D.3 foram conferidas em 2026-10-15 sem marca. A reexecução de 2026-10-17 mediu as 23 de novo e aplicou a marca | `referencias/CONFERENCIA.md` (seções "As 27 linhas de D.2 e D.4" e "As 23 linhas de D.3") e `book/apendices.md` |
| Cadeia do H3 | `stochology-h3/src/cadeia.rs`: uma linha derivada, três declaradas com o motivo | `labs --h3` |
| Previsões 3 e 5 do eixo R | o limiar na amplitude 2,5 com o plano calibrado (perda de +2,87 min, negativa abaixo dele); a transferência com 0 violações e 3 contraexemplos | `labs --resolucao` |
| Migrações de contrato | a cadeia em `migracoes.tsv`, embutida na compilação, 1.0 → 1.1 → 1.2 | `cargo test -p stochology-schema` e `labs --schema` |
| Eixo R nos capítulos | as 21 linhas do eixo, calculadas pelo instrumento, no Degrau 3 de cada capítulo | `labs --resolucao` e `python3 book/conferir_labs.py` |
| Segunda opinião do registro | cinco linhas conferidas por segunda implementação, duas derivadas, sete declaradas como registro | `labs --segunda` |

## 2. Os quatro que ficam, e por quê

| item | o que falta | quem resolve | prazo |
| --- | --- | --- | --- |
| Elicitação SHELF (3) | as outras duas pessoas por domínio e o especialista de fora do programa | gente | sem data; a trilha do primeiro especialista está fechada em cinco domínios |
| Voz de quem paga (5) | a escuta dos sete grupos afetados, com registro assinado | gente | **2026-12-15** |
| Tese da ponte em escala (9) | escolhedor de casos de fora, analistas humanos e juiz humano | gente | **2027-01-14**; sem eles, a tese fica não sustentada em definitivo |
| Ponte Jev-LLM (13) | **os pesos**, com o `sha256` declarado antes de rodar, e **o conjunto rotulado** de `fundo/p20-jev/calibracao/desfechos.tsv`. O runner existe desde 2026-10-17 (`crates/stochology-s2/src/runner.rs`) e o leitor do Brier também (`calibracao.rs`, amostra mínima de 30, abstenção fora do denominador); o manual do operador é `fundo/p20-jev/RUNNER.md`, o binário `ingestar-jev` prepara os dois arquivos com ensaio antes de persistir, e o fechamento tem protocolo declarado antes dos dados em `fundo/p20-jev/FECHAMENTO.md` | o operador, com dois arquivos | sem data |

**O que os quatro não têm em comum.** Três dependem de **gente de fora do programa** e não se resolvem com código:
o que falta é consentimento, escuta e julgamento cego, e é por isso que os instrumentos travam a caixa em vez de
preencher a lacuna. O quarto depende de **artefato e código**: os pesos com `sha256` e o runner que os carregue, e
o runner já está escrito; falta o arquivo. O critério de cada um está declarado antes de olhar o número.

## 3. O que este estado reprovou contra si mesmo

| o que caiu | o que ficou no lugar | instrumento |
| --- | --- | --- |
| a tese da ponte em escala | **não sustentada**: empate no teto, 8 de 8 nos dois braços e 0 alarme falso | `labs --ponte` |
| o laço declarado em 0,6 | é **4,1483 vezes** o medido, fora da banda de 1/3 a 3 que o pré-registro fixou | `labs --h3` |
| o limiar de ECE de 0,10 | era parede, não portão: aprovava 0,08% dos calibrados no desenho do protocolo | `labs --preteste` |
| a validação de uma linha na genética | o piso de 1/1 000 deixa passar 4 de 40, e a segunda linha separa | `labs --genetica` |
| a tolerância de calibração fixa | com tolerância que depende do n, nenhum dos seis cofres binários passa | `fundo/p08-calibracao` |
| o campo em máquina das prioris | marca `true` em 8 dos 33 domínios, e a frase publicada fala de 33 de 33 | `labs --segunda` |
| a escuta de quem paga | **0 de 7** grupos ouvidos, e a caixa lacrada por instrumento | `labs --escuta` |
| o lacre do `p01-tese/CEGAMENTO.md` | **re-selado**: o hash gravado às 12:42 de 2026-10-15 (`fcf8da5a…`) parou de valer quando o adendo do commit `935807e`, às 15:36, mudou o arquivo, e o lacre seguiu apontando para o texto de antes; agora vale `d92afa7c…` | `python3 conferir_lacres.py` |
| a convenção dos lacres | **uma só**: os 28 arquivos passaram a usar caminho relativo à raiz (incluindo `dados-reais/SHA256SUMS` e `governanca/DECLARACAO.sha256`), as 9 linhas que apontavam para o Python removido, para a pasta de casos que saiu da árvore ou para `crsp_day_1989_1998.csv` viraram comentário declarado, e `sha256sum -c` deixou de depender da pasta de onde se roda | `python3 conferir_lacres.py` |
| a higiene da árvore | **medida e corrigida**: `cargo fmt --check` e `clippy -D warnings` passam nos dezesseis crates (eram 615 diffs de formato e 35 avisos), a toolchain está fixada em `rust-toolchain.toml`, `scripts/verificar.sh` passa a ser a lista única de §V — o CI chamava 11 dos 29 comandos —, e o arquivo morto saiu da raiz para `historico/`. Nenhum número publicado mudou: **373 testes** e `RESULTADOS_RUST.md` idêntico byte a byte | `scripts/verificar.sh` |
| o registro por instrumento | **era amarrado à máquina**: cada bloco citava o caminho absoluto do binário (`/home/silvano-neto/…/target/release/labs`), então `gerar_resultados.py --check` só podia conferir nesta máquina e reprovava no CI desde 2026-10-17T16:23Z. O comando passou a entrar relativo à raiz (`./target/release/labs`), e o `--check` agora imprime a divergência em diff em vez de só reprovar | `python3 book/gerar_resultados.py --check` |

## 4. O que mudaria cada veredito

- **A escuta:** dois dos sete grupos com registro assinado até 2026-12-15 abrem a caixa das decisões cobertas; sem
  isso, o artefato fica arquivado como exercício de sensibilidade.
- **A ponte:** o escolhedor de casos de fora, os analistas humanos e o juiz humano até 2027-01-14 põem o teste
  justo em marcha; sem eles, a tese fica não sustentada em definitivo, como o dono aceitou por escrito.
- **O SHELF:** as outras duas pessoas por domínio e o especialista de fora fazem a elicitação valer na rodada
  oficial; hoje ela é sensibilidade com n = 1.
- **A Jev:** os pesos com o `sha256` batendo e o runner local liberam o Brier calibrado, o custo real e a
  abstenção do modelo de verdade.
- **A divergência do p05:** uma decisão humana sobre o sentido do campo `reproducao_rodada_a` resolve o que a
  segunda opinião registrou sem adivinhar.

## 5. A reprodução

~~~bash
cargo test --workspace                                  # 373 testes aprovados, nenhum reprovado
cargo run --release --bin labs -- --todos               # os 21 laboratórios
cargo run --release --bin portoes -- --horizonte todos --reproducao
cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23   # sai com 1, como declarado
python3 book/gerar.py --check                           # a rampa dos 21 capítulos E o arquivo em disco
python3 book/conferir_labs.py                           # 48 blocos citados, 0 divergentes
python3 book/conferir_livro.py
python3 book/gerar_resultados.py --check                # o registro e o manifesto do estado
python3 conferir_lacres.py                              # os 28 lacres: 82 alvos, 9 fora da árvore, 0 problemas
python3 estilo/conferir-passada.py                      # a passada de prosa contra HEAD:book/
~~~

Os instrumentos que provam os itens fechados de §1 também entram na receita, porque a tabela de §1 cita cada
um deles e uma receita que não os roda não confere a própria tabela:

~~~bash
cargo run --release --bin labs -- --genetica    # resolução fina da genética: 4 de 40 com uma linha, 1 de 40 com duas
cargo run --release --bin labs -- --h3          # a cadeia do H3: uma linha derivada, três declaradas
cargo run --release --bin labs -- --segunda     # a segunda opinião: cinco conferidas, duas derivadas, sete registro
cargo run --release --bin labs -- --ponte       # a tese da ponte: o piloto re-derivado e o portão do teste justo
cargo run --release --bin labs -- --escuta      # a voz de quem paga: 0 de 7 ouvidos, mínimo 2
cargo run --release --bin labs -- --schema      # a cadeia de migrações do contrato: 1.0 -> 1.1 -> 1.2
cargo run --release --bin labs -- --jev         # a ponte Jev-LLM: os portões fecham, os pesos faltam
cargo run --release --bin labs -- --resolucao   # o eixo R e as previsões 3 e 5
~~~

Cada pasta de ponto tem o seu `DECLARACAO.sha256`, e os 28 lacres da árvore se conferem de um só comando,
rodado da raiz: `python3 conferir_lacres.py` devolve **82 alvos conferidos**, **9 linhas declaradas fora da
árvore** e **0 problemas**. Dois lacres ficam sem alvo vivo, e aparecem como declarados em vez de aprovados:
`p01-tese/CASOS.sha256` (o banco de 12 casos saiu da árvore) e `p01-tese/GERADORES.sha256` (o gerador era
Python).

## 6. O manifesto

Em `RESULTADOS_RUST.md` §5, gerado: **20 declarações de ponto**, das quais 4 são declarações 2 ou mais (correção
com os dois valores lado a lado), **16 pontos de fundo**, **19 instrumentos** (e 5 flags modificadoras) no `labs`, **21 capítulos** e
**16 crates**. Cada linha do manifesto traz o `sha256` do arquivo da declaração.

O livro montado entra no mesmo manifesto com o que o identifica: **5 927 linhas**, **75 290 palavras** e
`sha256` do arquivo. O estado de 2026-10-17 tinha um defeito que este parágrafo fecha: o
`book/gerar.py --check` da época montava o livro na memória e comparava só a rampa, sem olhar o arquivo em
disco — e o arquivo em disco estava **353 linhas atrás** de `book/`, sem os 21 blocos do eixo R e sem as
reescritas dos itens 3 a 13. O portão cobria o motor e não a entrega. Agora o `--check` compara
`montar()` com `estocologia-livro-didatico.md` **caractere por caractere**, imprime a diferença e sai com 1
quando o disco não é o livro montado — e cobra uma terceira coisa: que a contagem declarada em `README.md`,
`PLAN.md`, nesta declaração e no registro seja a contagem que o mesmo `montar()` mede. O número escrito à mão
passou a ter instrumento, porque a correção de 2026-10-17T08:10Z nasceu de quatro documentos afirmando 5 903 linhas
enquanto o arquivo em disco tinha 5 904.

## 7. O que fica escrito

Este estado não deixa promessa. O que ela mediu está no registro, com o instrumento ao lado; o que ela reprovou
está reprovado, com o número e o critério que o reprovou; o que ela não pôde fazer está declarado, com quem
resolve e até quando. A árvore fica limpa, e o transcript que originou a auditoria da ponte Jev
(`historico/transcricao-jev-2026-10-16.md`) está **versionado** desde o commit `5393515` — a versão anterior
desta frase o dava como o único arquivo não versionado, e era falsa nos dois lados: o arquivo está no Git e a
árvore tinha seis arquivos modificados quando a correção de 2026-10-17T08:10Z começou.

Quem quiser conferir não precisa acreditar em nada: rode os dezoito comandos da seção 5 — dez na reprodução e oito
nos instrumentos dos itens fechados —, e o `sha256sum -c` de cada
pasta de ponto.
