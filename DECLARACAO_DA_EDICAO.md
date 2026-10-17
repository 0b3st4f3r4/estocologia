# Declaração da edição: o congelamento

**Congelada em 2026-10-17T06:47:33Z; retificada às 07:27Z e às 08:10Z.** Esta é a declaração que fecha a edição: o que ela fez, o que não pôde fazer, o que
reprovou contra si mesma, o que mudaria cada veredito e como tudo se reproduz. O manifesto com o hash de cada
declaração de ponto está em `RESULTADOS_RUST.md` §5, **gerado** por `python3 book/gerar_resultados.py` e conferido
por `--check`.

A retificação de 07:27Z corrigiu o que o congelamento anterior não viu, e cada conserto está declarado no lugar
próprio: a edição montada ficou 353 linhas atrás de `book/` (§6), o lacre do `CEGAMENTO.md` estava vencido desde
2026-10-15 e a convenção de caminho dos lacres tinha duas regras (§3), e a receita de reprodução não rodava os
instrumentos que provam os itens fechados (§5).

A retificação de 08:10Z corrigiu a divergência que a de 07:27Z criou: o livro ganhou duas linhas em `cap13.md`
e só o manifesto foi regerado, de modo que esta declaração, o `README.md` e o `PLAN.md` passaram a afirmar uma
contagem que o `montar()` já não produzia. Consertado em §6, com o número corrigido nos quatro lugares; e o
`gerar.py --check` ganhou a terceira cobrança que faltava, para que a contagem escrita à mão não volte a
sobreviver ao crescimento do livro. O §7 trocou uma afirmação falsa sobre o transcript Jev pela verdadeira, o §5
passou a contar o alvo novo do lacre, e a contagem de comandos desta seção foi corrigida. Os dois hashes desta
declaração, antes e depois, estão lado a lado em `DECLARACAO_DA_EDICAO_2.md`.

## 1. Os nove itens que fecharam, com a prova e o comando

| item | prova | comando |
| --- | --- | --- |
| Sistema 2 deliberativo | `crates/stochology-s2`, sete módulos testados | `cargo test --workspace` e `labs --s2` |
| Crates do roteiro | os dez nomes vivos, cada um com teste próprio | `cargo test --workspace` |
| Resolução fina da genética | `stochology-h2/src/genetica.rs`: 4 de 40 passam com uma linha, 1 de 40 com duas, piso de 0,001 para 0,000001 | `labs --genetica` |
| Conferência da bibliografia | as 50 linhas de D.2 a D.4 com **[conferido]**, 27 conferidas contra DOI e editora, 0 divergência de conteúdo | `referencias/CONFERENCIA.md` e `book/apendices.md` |
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
| Ponte Jev-LLM (13) | os pesos congelados com o `sha256` e o runner local | um artefato externo | sem data |

**O que os quatro têm em comum:** nenhum deles se resolve com mais código. Três dependem de pessoas de fora do
programa e um depende de um modelo de verdade para rodar. O arcabouço de cada um está armado, com o critério
congelado antes de olhar o número.

## 3. O que esta edição reprovou contra si mesma

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
| a convenção dos lacres | **uma só**: os 26 arquivos passaram a usar caminho relativo à raiz, as 8 linhas que apontavam para o Python removido ou para a pasta de casos que saiu da árvore viraram comentário declarado, e `sha256sum -c` deixou de depender da pasta de onde se roda | `python3 conferir_lacres.py` |

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
cargo test --workspace                                  # 260 testes aprovados, nenhum reprovado
cargo run --release --bin labs -- --todos               # os 21 laboratórios
cargo run --release --bin portoes -- --horizonte todos --reproducao
cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23   # sai com 1, como declarado
python3 book/gerar.py --check                           # a rampa dos 21 capítulos E o arquivo em disco
python3 book/conferir_labs.py                           # 48 blocos citados, 0 divergentes
python3 book/conferir_livro.py
python3 book/gerar_resultados.py --check                # o registro e o manifesto da edição
python3 conferir_lacres.py                              # os 26 lacres: 63 alvos, 8 fora da árvore, 0 problemas
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

Cada pasta de ponto tem o seu `DECLARACAO.sha256`, e os 26 lacres da árvore se conferem de um só comando,
rodado da raiz: `python3 conferir_lacres.py` devolve **63 alvos conferidos**, **8 linhas declaradas fora da
árvore** e **0 problemas**. Dois lacres ficam sem alvo vivo, e aparecem como declarados em vez de aprovados:
`p01-tese/CASOS.sha256` (o banco de 12 casos saiu da árvore) e `p01-tese/GERADORES.sha256` (o gerador era
Python).

## 6. O manifesto

Em `RESULTADOS_RUST.md` §5, gerado: **20 declarações de ponto**, das quais 4 são declarações 2 ou mais (retificação
com os dois valores lado a lado), **16 pontos de fundo**, **20 instrumentos** no `labs`, **21 capítulos** e
**15 crates**. Cada linha do manifesto traz o `sha256` do arquivo da declaração.

A edição montada entra no mesmo manifesto com o que a identifica: **5 904 linhas**, **74 861 palavras** e
`sha256` do arquivo. O congelamento de 2026-10-17 tinha um defeito que este parágrafo fecha: o
`book/gerar.py --check` da época montava o livro na memória e comparava só a rampa, sem olhar o arquivo em
disco — e o arquivo em disco estava **353 linhas atrás** de `book/`, sem os 21 blocos do eixo R e sem as
reescritas dos itens 3 a 13. O portão cobria o motor e não a entrega. Agora o `--check` compara
`montar()` com `estocologia-livro-didatico.md` **caractere por caractere**, imprime a diferença e sai com 1
quando o disco não é o livro montado — e cobra uma terceira coisa: que a contagem declarada em `README.md`,
`PLAN.md`, nesta declaração e no registro seja a contagem que o mesmo `montar()` mede. O número escrito à mão
passou a ter instrumento, porque a retificação de 08:10Z nasceu de quatro documentos afirmando 5 903 linhas
enquanto o arquivo em disco tinha 5 904.

## 7. O que fica escrito

Esta edição não deixa promessa. O que ela mediu está no registro, com o instrumento ao lado; o que ela reprovou
está reprovado, com o número e o critério que o reprovou; o que ela não pôde fazer está declarado, com quem
resolve e até quando. A árvore fica limpa, e o transcript que originou a auditoria da ponte Jev
(`Integração de Modelos Jev à Estocologia.md`) está **versionado** desde o commit `5393515` — a versão anterior
desta frase o dava como o único arquivo não versionado, e era falsa nos dois lados: o arquivo está no Git e a
árvore tinha seis arquivos modificados quando a retificação de 08:10Z começou.

Quem quiser conferir não precisa acreditar em nada: rode os dezoito comandos da seção 5 — dez na reprodução e oito
nos instrumentos dos itens fechados —, e o `sha256sum -c` de cada
pasta de ponto.
