# Parecer — Caso 02: suplemento na ração de leitões

## Problema central: a unidade de sorteio é a baia, não o leitão
O suplemento foi sorteado entre 8 baias (4 contra 4) e é dado por um comedouro comum, de modo que todos os leitões de uma baia compartilham tratamento, ambiente e comedouro. O teste t com 240 leitões trata como independentes animais que não são, e o p < 0,001 é ilusório. A amostra efetiva para o efeito do tratamento é de 8 unidades.

## Resultados
- Ganho de peso (final − inicial) por leitão: 21,77 kg com suplemento e 19,51 kg no padrão, diferença de 2,26 kg. O t ingênuo dá p ≈ 4e-8, o que reproduz a conta do cliente.
- Médias por baia. Suplemento: 20,4; 18,7; 25,2; 22,9. Padrão: 21,1; 20,6; 16,2; 20,2. A variação entre baias dentro do mesmo tratamento (16,2 a 25,2) é maior que a diferença entre tratamentos.
- Correlação intraclasse do ganho, por modelo misto com intercepto aleatório de baia: ICC ≈ 0,58, muito alta. O efeito de desenho, 1 + 29·0,58 ≈ 18, reduz os 240 leitões a algo como 13 independentes.
- Análises corretas:
  - Teste t com as 8 médias de baia: diferença de 2,26 kg, IC 95% de −2,17 a +6,70, p = 0,26.
  - Modelo misto (baia aleatória): 2,26 kg, EP 1,81, p = 0,21.
  - OLS com peso inicial e erro agrupado por baia: 2,27 kg, IC de −1,04 a 5,58, p = 0,18.
  - Teste de aleatorização exato, com as 70 formas de escolher 4 de 8 baias: p = 0,29.
- O peso inicial é parecido entre os grupos e não explica nada.

## Implicação para a decisão
O dado é compatível tanto com nenhum efeito (ou até perda) quanto com um ganho grande. Não há base para uma compra cara para 30 mil leitões por ano. Para uma decisão desse porte seria preciso um ensaio com muito mais baias, ou com sorteio por animal e comedouro individual, dimensionado para o ganho mínimo que paga o suplemento.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: O tratamento foi sorteado por baia (4 contra 4), com ICC ≈ 0,58. Analisado na unidade correta, o efeito de 2,26 kg tem IC 95% de −2,2 a +6,7 kg e p ≈ 0,2–0,3 (aleatorização exata p = 0,29). O p < 0,001 vem de tratar 240 leitões como independentes.
```
