# Declaração da edição: o congelamento

**Congelada em 2026-10-17T06:47:33Z.** Esta é a declaração que fecha a edição: o que ela fez, o que não pôde fazer, o que
reprovou contra si mesma, o que mudaria cada veredito e como tudo se reproduz. O manifesto com o hash de cada
declaração de ponto está em `RESULTADOS_RUST.md` §5, **gerado** por `python3 book/gerar_resultados.py` e conferido
por `--check`.

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
python3 book/gerar.py --check                           # a rampa dos 21 capítulos
python3 book/conferir_labs.py                           # 48 blocos citados, 0 divergentes
python3 book/conferir_livro.py
python3 book/gerar_resultados.py --check                # o registro e o manifesto da edição
~~~

Cada pasta de ponto tem o seu `DECLARACAO.sha256`; `sha256sum -c` em cada uma confere o lacre. Os oito lacres
desta edição (pontos 13 a 20) passam.

## 6. O manifesto

Em `RESULTADOS_RUST.md` §5, gerado: **20 declarações de ponto**, das quais 4 são declarações 2 ou mais (retificação
com os dois valores lado a lado), **16 pontos de fundo**, **20 instrumentos** no `labs`, **21 capítulos** e
**15 crates**. Cada linha do manifesto traz o `sha256` do arquivo da declaração.

## 7. O que fica escrito

Esta edição não deixa promessa. O que ela mediu está no registro, com o instrumento ao lado; o que ela reprovou
está reprovado, com o número e o critério que o reprovou; o que ela não pôde fazer está declarado, com quem
resolve e até quando. A árvore fica limpa, com um único arquivo não versionado — o transcript que originou a
auditoria da ponte Jev —, que fica fora do Git por decisão.

Quem quiser conferir não precisa acreditar em nada: rode os oito comandos da seção 5, e o `sha256sum -c` de cada
pasta de ponto.
