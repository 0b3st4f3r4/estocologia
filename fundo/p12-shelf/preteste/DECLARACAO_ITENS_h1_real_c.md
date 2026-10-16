# Declaração de itens do pré-teste — \`h1-real/c\`

**Congelada em 2026-10-16**, antes de o especialista responder qualquer coisa.

| o que | valor |
| --- | --- |
| pool de origem | \`fundo/p12-shelf/preteste/h1_real_c.yaml\`, 30 itens (15 \`sim\` e 15 \`nao\`), 10 descartados, SHA-256 \`fb5a8d87a206546427edf2b62c0ec68a53f15349c6374c0e13115c058bf7046f\` |
| semente do sorteio | \`2026\` |
| regra do sorteio | **estratificado**: embaralha cada estrato de resolução com o \`SplitMix64\` do repositório e toma 10 de cada. O pool é 15 \`sim\` × 15 \`nao\`, então não houve descarte por desequilíbrio |
| gabarito | fica com o facilitador; o especialista vê **só os enunciados** desta declaração |

## O filtro de gabarito: **30 de 30 conferidos**, o melhor dos cinco domínios

O facilitador baixou as **dez fontes** por conta própria (quatro PDFs lidos com \`pdftotext\` e seis páginas convertidas em texto) e localizou o número declarado de cada item. Resultado: **30 de 30**, nenhum item excluído.

- **24** apareceram na primeira passada, com o trecho conferido à vista (Swiss Re sigma 1/2025, sigma 1/2024 e o sigma 3/2024; Munich Re 2023 e 2024; NOAA/NCEI 2023 e 2024; EIOPA; OCDE).
- **6** só apareceram depois de corrigir a **leitura do separador**: o pool escreve os números em português (\`74.000\`, \`31,9\`, \`182,7\`, \`1.543,2\`, \`96,2\`, \`6,2\`) e as fontes em inglês (\`74,000\`, \`31.9\`, \`182.7\`, \`1,543.2\`, \`96.2\`, \`6.2 billion\`). Localizados os seis, com o trecho conferido.

Os trechos que ancoram os itens frágeis ficam registrados: \`more than 74,000 fatalities\` (C05), a tabela do III com \`2016 $31.9 $43.1\` e \`2017 96.2 127.2\` (C08 e C17), \`approximately $182.7 billion\` (C14), \`$1,543.2 billion\` em ciclones (C16) e \`insured losses of USD 6.2 billion\` (C23).

## Os 20 itens, na ordem em que o especialista os vê

| # | id | enunciado | dificuldade declarada |
| --- | --- | --- | --- |
| 1 | C23 | No ano-base 2023, as perdas seguradas do terremoto da Turquia e da Síria eram maiores que 20 bilhões de dólares? | dificil |
| 2 | C27 | No ano-base 2024, considerando a série de 1980 a 2024, mais de 70% das perdas econômicas por catástrofes naturais na Europa estavam cobertas por seguro? | dificil |
| 3 | C01 | No ano-base 2024, as perdas seguradas por catástrofes naturais no mundo eram maiores que 100 bilhões de dólares? | facil |
| 4 | C28 | No ano-base 2024, considerando a série de 1980 a 2024, o terremoto na Itália respondia por mais de 30% das perdas não seguradas por catástrofes naturais no espaço econômico europeu? | dificil |
| 5 | C15 | No ano-base 2024, o número de desastres de bilhão de dólares catalogados pela NOAA nos Estados Unidos era maior que o número registrado em 2023? | media |
| 6 | C29 | No ano-base 2023, a penetração do seguro de danos (não-vida) nos Países Baixos era maior que 6% do PIB? | dificil |
| 7 | C19 | No ano-base 2023, a penetração do seguro no mundo, medida como prêmios diretos em porcentagem do PIB, era maior que 6%? | media |
| 8 | C14 | No ano-base 2024, o custo total dos desastres de bilhão de dólares catalogados pela NOAA nos Estados Unidos era maior que 200 bilhões de dólares? | media |
| 9 | C20 | No ano-base 2023, a penetração do seguro no México era maior que 5% do PIB? | media |
| 10 | C08 | No ano-base 2016, as perdas seguradas por catástrofes naturais nos Estados Unidos eram maiores que 50 bilhões de dólares? | facil |
| 11 | C22 | No ano-base 2023, as perdas seguradas por tempestades convectivas severas no mundo eram maiores que 50 bilhões de dólares? | dificil |
| 12 | C21 | No ano-base 2023, o número de catástrofes naturais que geraram perdas seguradas no mundo era maior que 130? | dificil |
| 13 | C13 | No ano-base 2024, as catástrofes meteorológicas respondiam por mais de 90% das perdas seguradas por catástrofes naturais no mundo? | media |
| 14 | C12 | No ano-base 2024, as perdas seguradas por perigos secundários (cheias, incêndios florestais e tempestades severas) no mundo eram maiores que as registradas em 2023? | media |
| 15 | C04 | No ano-base 2023, as perdas econômicas por catástrofes naturais no mundo eram maiores que 200 bilhões de dólares? | facil |
| 16 | C26 | No ano-base 2024, o número de desastres de bilhão de dólares ocorridos nos Estados Unidos na década de 2015 a 2024 era maior que 150? | dificil |
| 17 | C16 | No ano-base 2024, o custo acumulado dos desastres de bilhão de dólares causados por ciclones tropicais nos Estados Unidos desde 1980 era maior que 1 trilhão de dólares? | media |
| 18 | C06 | No ano-base 2023, as perdas seguradas por catástrofes naturais no mundo, na contagem da Munich Re, eram maiores que 80 bilhões de dólares? | facil |
| 19 | C03 | No ano-base 2024, mais de 70% das perdas econômicas por catástrofes naturais no mundo ficaram sem cobertura de seguro? | facil |
| 20 | C30 | No ano-base 2023, a penetração do seguro de danos (não-vida) na maioria das jurisdições que reportam à OCDE era maior que 8% do PIB? | dificil |

## O que o especialista recebe e o que ele devolve

- Recebe: os 20 enunciados e o pedido de \`p em (0, 1)\` para cada um, de cabeça e sem consulta.
- Devolve: 20 probabilidades.
- O facilitador roda: Brier, climatologia do pool (0,50), habilidade, ECE em 5 caixas de massa igual, curva observada, decomposição de Murphy e o critério 3 pelo **teste binomial** (declaração 3).

## Limites declarados

- **O especialista é o autor do programa**: painel de 1 de 3. Este é o **último domínio da fila dele**; \`h2g/educacao_star_turmas\` fica reservado para um segundo especialista, pela exclusão do par de mesma base.
- **A quantidade deste domínio é uma frequência em (0, 1)**, e por isso o ajuste da priori é **Beta** — o instrumento entrou em código em 2026-10-16 (\`stochology-h3::beta\`), com a priori fraca do p05 (Beta(1; 1)) recuperada exatamente no teste.
- **Nenhum item toca o domínio**: nada da base dinamarquesa de incêndios, nada do esquema "100 xs 50", nada da camada de resseguro daquele esquema.
