# Declaração do ponto 16: a tese da ponte em escala, com o teste justo por fazer

**Congelada em 2026-10-16T22:37:13Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `registro.tsv` | `fb547bcd6de4fc7f047c10f0b095b551fb5c1b3ace09784a2f25f235ee0c23c8` | os papéis do teste justo, o prazo e o mínimo de casos |
| `DECLARACAO.md` (esta) | — | as quatro declarações; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `c204219a96e412398df83da135422821cbefc930da6afed2b85ee46b711a3ff5` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — o piloto, re-derivado e não repetido

O piloto P01 deu **empate no teto**, e este instrumento não repete o número do README: ele lê as notas do juiz
cego (`fundo/p01-tese/juiz/notas.yaml`) e refaz a conta. O que ele devolve:

| o que | valor re-derivado |
| --- | --- |
| casos | 12, sendo 8 com defeito e 4 limpos |
| apontou o defeito | braço X: 8 de 8; braço Y: 8 de 8 |
| alarme falso nos limpos | braço X: 0 de 4; braço Y: 0 de 4 |
| pares discordantes | 0 |
| veredito pela regra declarada | **empate no teto: a tese não é sustentada** |

O juiz codificou cada parecer com uma nota (1 = apontou, em caso com defeito; 1 = alarme falso, em caso limpo).
Sem par discordante, o teste de McNemar não tem o que testar — o piloto **não discrimina**, e é isso que o
empate no teto quer dizer. A regra que transforma o empate em veredito está na `DECLARACAO.md` do piloto, escrita
antes de qualquer parecer; ela não é reescrita aqui.

## Declaração 2 — o que um teste justo exige

O README do piloto nomeia o que falta: casos mais difíceis escolhidos por alguém **de fora do programa**,
analistas **humanos** e um juiz **humano**. Esta frente transforma isso em exigência verificável, no
`registro.tsv`:

| papel | exigência |
| --- | --- |
| escolhedor dos casos | de fora do programa, e escolhe os casos **antes** de existir resultado |
| analistas do braço A (arquitetura) | dois humanos, um de fora do programa |
| analistas do braço B (análise comum) | dois humanos, um de fora, cegos de qualquer texto da Estocologia |
| juiz | um humano de fora do programa, cego ao braço, com o gabarito |
| casos | no mínimo 12, mais difíceis que a lista de 2019, escolhidos pelo de fora |

## Declaração 3 — o portão da tese

Enquanto os papéis não estiverem designados e os casos no mínimo, o status da tese é **não sustentada**, e
nenhuma linha do registro pode dizer outra coisa. O instrumento reprova quem afirmar o contrário: a prova está
nos testes — um registro que diz `sustentada` sem o teste justo é **REPROVADO** com a divergência nomeada, e um
status `em teste` sem os papéis designados também. É a diferença entre uma tese que está aberta e uma tese que
alguém promoveu por decurso de prazo.

## Declaração 4 — a cláusula de saída

- **Janela:** 90 dias corridos a partir deste carimbo, até **2027-01-14**.
- **Cobertura mínima:** os quatro papéis designados (com pelo menos um analista de fora em cada braço e o juiz de
  fora) e **12 casos** escolhidos pelo de fora.
- **Se a cobertura não for atingida até o prazo:** a tese fica **não sustentada em definitivo** como veredito do
  programa, e o piloto fica como o que ele é — um teste que empatou no teto contra a lista de 2019, sem poder
  para discriminar. Nenhuma outra frente promove a tese.
- **Se for atingida:** o teste roda, o juiz pontua, e o veredito sai das notas dele pela regra da declaração 1.

Mudança de exigência, prazo ou cobertura mínima é **declaração nova**, com hash próprio, e conta no K do programa.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-book` | 34 testes aprovados, entre eles os cinco do `ponte`: a re-derivação do empate no teto, o registro que declara o que falta, a tese sustentada sem teste justo **reprovada**, os papéis designados que deixam o teste rodar e o status `em teste` sem os papéis, reprovado (0) |
| `labs --ponte` | o piloto re-derivado (8 de 8 nos dois braços, 0 de 4 alarmes falsos, 0 pares discordantes, empate no teto), os cinco papéis pendentes e as duas provas sintéticas (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` atualizado, com o bloco do ponto 16 (0) |
