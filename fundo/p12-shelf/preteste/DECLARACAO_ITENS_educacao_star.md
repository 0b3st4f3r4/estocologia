# Declaração de itens do pré-teste — \`h2/educacao_star\`

**Congelada em 2026-10-16**, antes de o especialista responder qualquer coisa.

| o que | valor |
| --- | --- |
| pool de origem | \`fundo/p12-shelf/preteste/educacao_star.yaml\`, 30 itens (15 \`sim\` e 15 \`nao\`), 12 descartados, SHA-256 \`5c4b6890ea73e845c603b3bb22c8ea3484a71229ae8be15b3b50c37588441c8a\` |
| semente do sorteio | \`2026\` |
| regra do sorteio | **estratificado**: embaralha cada estrato de resolução com o \`SplitMix64\` do repositório, toma 10 de cada e embaralha os 20 para a ordem de apresentação |
| gabarito | fica com o facilitador; o especialista vê **só os enunciados** desta declaração |

## O filtro de gabarito, declarado, com **dois graus** de conferência

O pool deste domínio pediu equilíbrio desde o começo (15 \`sim\` e 15 \`nao\`), e os **30 itens foram conferidos pelo facilitador** — não houve item excluído por falta de gabarito, como aconteceu em \`h2/direito_prisoes\`. A conferência tem dois graus, e cada item carrega o seu:

- **igualdade exata**: o valor da fonte foi lido por máquina e comparado ao número declarado — Banco Mundial (13 itens, API \`api.worldbank.org\`) e Eurostat (7 itens, API JSON-stat). **20 itens.**
- **presença na página citada**: o número declarado aparece no texto da página ou do PDF baixado (\`pdftotext\` no PDF do IBGE) — NCES (6), IBGE Censo 2022 (2) e o digest do PISA (2). **10 itens.**

O segundo grau é mais fraco que o primeiro, e está escrito para não se confundir com ele.

## Os 20 itens, na ordem em que o especialista os vê

| # | id | enunciado | dificuldade declarada |
| --- | --- | --- | --- |
| 1 | E20 | No ano-base 2022, a proporção de pessoas de 25 a 34 anos com ensino superior completo na Irlanda era maior que 65%? | dificil |
| 2 | E26 | No ano-base 2022, a pontuação média de matemática dos alunos do 8º ano dos Estados Unidos na avaliação NAEP era maior que 280? | medio |
| 3 | E01 | No ano-base 2019, o gasto público total em educação do Brasil, medido como porcentagem do PIB, era maior que 4%? | facil |
| 4 | E28 | No ano-base 2022, a taxa de alfabetização das pessoas de 15 anos ou mais de idade na Região Nordeste do Brasil era maior que 90%? | medio |
| 5 | E15 | No ano-base 2022, a razão entre alunos e professores no ensino primário da Grécia era maior que 10? | medio |
| 6 | E29 | No ano-base 2018, a pontuação média de leitura dos estudantes de 15 anos da Estônia na avaliação PISA era maior que 520? | dificil |
| 7 | E21 | No ano escolar de 2021-22, a taxa média de conclusão do ensino médio em quatro anos (ACGR) das escolas públicas dos Estados Unidos era maior que 85%? | facil |
| 8 | E13 | No ano-base 2019, a taxa de conclusão do ensino primário da Índia era maior que 99%? | dificil |
| 9 | E18 | No ano-base 2022, a proporção de pessoas de 25 a 34 anos com ensino superior completo na Itália era maior que 35%? | medio |
| 10 | E07 | No ano-base 2019, a taxa de conclusão do ensino primário da África do Sul era maior que 95%? | dificil |
| 11 | E25 | No ano-base 2019, o gasto por aluno equivalente em tempo integral (FTE) no nível fundamental e médio da Coreia do Sul era maior que 15.000 dólares (dólares constantes de 2021, ajustados pela paridade de poder de compra)? | dificil |
| 12 | E23 | No outono de 2021, o número de matrículas de graduação nas instituições de ensino superior dos Estados Unidos era maior que 15 milhões? | facil |
| 13 | E14 | No ano-base 2022, a razão entre alunos e professores no ensino primário da Romênia era maior que 15? | facil |
| 14 | E12 | No ano-base 2019, a taxa bruta de matrícula no ensino superior da China era maior que 70%? | facil |
| 15 | E04 | No ano-base 2018, o gasto público por aluno do ensino primário da Costa Rica, medido como porcentagem do PIB per capita, era maior que 15%? | facil |
| 16 | E27 | No ano-base 2022, a taxa de alfabetização das pessoas de 15 anos ou mais de idade no Brasil era maior que 90%? | facil |
| 17 | E16 | No ano-base 2022, a razão entre alunos e professores no ensino primário da Chéquia era maior que 16? | dificil |
| 18 | E06 | No ano-base 2019, a taxa de conclusão do ensino primário do México era maior que 100%? | medio |
| 19 | E02 | No ano-base 2022, o gasto público total em educação do México, medido como porcentagem do PIB, era maior que 6%? | facil |
| 20 | E30 | No ano-base 2018, a pontuação média de leitura dos estudantes de 15 anos do Brasil na avaliação PISA era maior que 450? | facil |

## O que o especialista recebe e o que ele devolve

- Recebe: os 20 enunciados e o pedido de \`p em (0, 1)\` para cada um, com a instrução de responder de cabeça, sem consulta.
- Devolve: 20 probabilidades.
- O facilitador roda: Brier, climatologia do pool (0,50 — o pool é equilibrado por construção), habilidade, ECE em 5 caixas de massa igual (portão ajustado), curva observada e decomposição de Murphy.

## Limites declarados desta lista

- **O especialista é o autor do programa**, como nos dois domínios anteriores: painel de 1 de 3, sem o controle de fora.
- **Mesma base**: \`h2/educacao_star\` e \`h2g/educacao_star_turmas\` compartilham a base \`AER_STAR\`; o painel não pode ter a mesma pessoa dos dois lados do par, e isso vale para quando o painel existir — com um especialista só, fica declarado.
- **Nenhum item toca o domínio elicitado**: nada de tamanho de turma nos EUA, nada de STAR, Toronto, Rhode Island ou LaLonde/NSW.
