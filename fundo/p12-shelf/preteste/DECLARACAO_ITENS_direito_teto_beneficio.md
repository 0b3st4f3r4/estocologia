# Declaração de itens do pré-teste — \`h2g/direito_teto_beneficio\`

**Congelada em 2026-10-16**, antes de o especialista responder qualquer coisa.

| o que | valor |
| --- | --- |
| pool de origem | \`fundo/p12-shelf/preteste/direito_teto_beneficio.yaml\`, 30 itens (17 \`sim\` e 13 \`nao\`), 16 descartados, SHA-256 \`965faff6503d0a6348851fefd7a5f124e18bf4917af41d449ca74cf6e719db37\` |
| semente do sorteio | \`2026\` |
| regra do sorteio | **estratificado**: embaralha cada estrato de resolução com o \`SplitMix64\` do repositório e toma 10 de cada. Os conferidos deram **exatamente 10 \`sim\` e 10 \`nao\`**, então não houve descarte por desequilíbrio |
| gabarito | fica com o facilitador; o especialista vê **só os enunciados** desta declaração |

## O filtro de gabarito, declarado

Dos 30 itens do pool, **20 foram conferidos pelo facilitador** e são os que entram: todos os 14 do Eurostat, os 2 do IBGE/SIDRA, os 2 da API v1 do BLS, o 1 do Banco Mundial, e o da página da OIT.

- **igualdade exata (19 itens)**: o valor foi lido por máquina na API e comparado ao declarado — Eurostat (14: T09 a T21 e T24), IBGE/SIDRA (2: T26 e T27, série 202312 = 39,2 e 202412 = 38,6), BLS API v1 (2: T22 = 0,42 na \`CMU2270000000000D\` e T23 = 0,44 na \`CMU1270000000000D\`, quarto trimestre de 2024) e Banco Mundial (1: T28 = 27,0083134719641).
- **presença na página citada (1 item)**: T30, o 47,6% da OIT, localizado no texto da página \`world-social-protection-report-2024-26-figures\`.

**Os 10 itens que eu não reproduzi ficaram fora do sorteio**, e não entram por confiança no executor: T01 a T08 (as oito páginas do BLS, que sem o conjunto completo de cabeçalhos de navegador respondem bloqueio), T25 (a página da SSA, mesmo caso) e T29 (o PDF da OIT, que baixou com 37 KB e não traz o texto do relatório). Fica declarado, e não escondido.

## Os 20 itens, na ordem em que o especialista os vê

| # | id | enunciado | dificuldade declarada |
| --- | --- | --- | --- |
| 1 | T11 | No ano-base 2022, o número de acidentes de trabalho fatais na França era maior que 700? | dificil |
| 2 | T23 | No ano-base 2024, o custo do seguro de acidentes de trabalho por hora trabalhada entre os trabalhadores civis dos Estados Unidos, no trimestre encerrado em dezembro de 2024, era maior que US$ 0,40? | dificil |
| 3 | T13 | No ano-base 2022, o número de acidentes de trabalho fatais na Espanha era maior que 350? | medio |
| 4 | T15 | No ano-base 2022, a taxa de acidentes de trabalho fatais por 100.000 trabalhadores na França era maior que 3,0? | dificil |
| 5 | T30 | No ano-base 2023, a proporção da população mundial sem nenhuma proteção social era maior que 55%? | medio |
| 6 | T14 | No ano-base 2022, o número de acidentes de trabalho fatais nos Países Baixos era maior que 50? | facil |
| 7 | T19 | No ano-base 2022, o número de acidentes de trabalho não fatais com pelo menos 4 dias de afastamento na Itália era maior que 300.000? | dificil |
| 8 | T21 | No ano-base 2022, o número de acidentes de trabalho na União Europeia (27 países) que resultaram em incapacidade permanente ou afastamento de 183 dias ou mais era maior que 150.000? | dificil |
| 9 | T28 | No ano-base 2023, o emprego vulnerável como proporção do emprego total no Brasil era maior que 35%? | medio |
| 10 | T12 | No ano-base 2024, o número de acidentes de trabalho fatais na França era maior que 1.000? | dificil |
| 11 | T10 | No ano-base 2022, o número de acidentes de trabalho fatais na Alemanha era maior que 350? | medio |
| 12 | T27 | No ano-base 2024, a taxa de informalidade das pessoas de 14 anos ou mais ocupadas no Brasil, no trimestre móvel encerrado em dezembro de 2024, era maior que 45%? | medio |
| 13 | T22 | No ano-base 2024, o custo do seguro de acidentes de trabalho por hora trabalhada na indústria privada dos Estados Unidos, no trimestre encerrado em dezembro de 2024, era maior que US$ 0,50? | medio |
| 14 | T09 | No ano-base 2023, o número de acidentes de trabalho fatais na Alemanha era maior que 500? | medio |
| 15 | T24 | No ano-base 2024, a proporção de empregados de 16 a 64 anos na União Europeia (27 países) com direito a benefício por doença era maior que 80%? | medio |
| 16 | T26 | No ano-base 2023, a taxa de informalidade das pessoas de 14 anos ou mais ocupadas no Brasil, no trimestre móvel encerrado em dezembro de 2023, era maior que 35%? | facil |
| 17 | T18 | No ano-base 2022, o número de acidentes de trabalho não fatais com pelo menos 4 dias de afastamento na Alemanha era maior que 1 milhão? | medio |
| 18 | T17 | No ano-base 2022, o número de acidentes de trabalho não fatais com pelo menos 4 dias de afastamento na União Europeia (27 países) era maior que 2,5 milhões? | facil |
| 19 | T16 | No ano-base 2022, a taxa de acidentes de trabalho fatais por 100.000 trabalhadores na Alemanha era maior que 1,0? | medio |
| 20 | T20 | No ano-base 2022, o número de acidentes de trabalho na União Europeia (27 países) que resultaram em afastamento de 3 a 6 meses era maior que 150.000? | dificil |

## O que o especialista recebe e o que ele devolve

- Recebe: os 20 enunciados e o pedido de \`p em (0, 1)\` para cada um, de cabeça e sem consulta.
- Devolve: 20 probabilidades.
- O facilitador roda: Brier, climatologia do pool (0,50 — equilibrado por construção), habilidade, ECE em 5 caixas de massa igual, curva observada e decomposição de Murphy, com o **critério 3 pelo teste binomial** (declaração 3).

## Limites declarados desta lista

- **O especialista é o autor do programa**: painel de 1 de 3, sem o controle de fora, como nos três domínios anteriores.
- **O par de mesma base deste domínio é \`h1-real/b\`** (NSW/LaLonde), que o E1 não serviu como especialista SHELF — por isso este domínio é elegível para ele, ao contrário de \`h2g/educacao_star_turmas\`.
- **Nenhum item toca o domínio elicitado**: nada de elasticidade da duração em relação ao benefício, nada do teto do benefício, nada de Kentucky/Michigan dos anos 1980.
