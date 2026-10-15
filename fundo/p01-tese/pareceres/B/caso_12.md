# Parecer — Caso 12: triagem de 40 compostos antibacterianos

## Problema central: comparações múltiplas
Foram testados 40 compostos sem hipótese prévia, cada um a α = 0,05. Mesmo que nenhum tivesse atividade, o esperado seriam 40 × 0,05 = 2 resultados "significativos". Foram observados 3.

## Resultados
- Os p-valores reproduzidos são K40 p = 0,030 (−18,2%), K12 p = 0,035 (−11,3%) e K19 p = 0,037 (−11,6%).
- Corrigidos para 40 testes:
  - Holm: p ajustado = 1,0 para os três.
  - Benjamini-Hochberg: q = 0,49 para os três, ou seja, cerca de metade das "descobertas" seria esperada como falsa.
- A distribuição dos 40 p-valores não difere de uma uniforme (Kolmogorov-Smirnov p = 0,11), que é o padrão esperado se nenhum composto tiver efeito. Dos 40, 22 reduziram e 18 aumentaram o crescimento, perto de 50/50.
- Há efeitos de mesma magnitude no sentido oposto: K11 +15% (p = 0,074), K17 +14%, K28 +12%. Diferenças de ±10–18% aparecem dos dois lados, o que é consistente com ruído de 8 poços por condição.
- A média dos controles varia entre placas de 0,91 a 1,09 (DP 0,047), por isso a comparação deve ser dentro da placa, como foi feita. Comparar contra o controle agrupado de todas as placas faria o K19 parecer forte (p = 0,0002), mas esse teste ignora o efeito de placa e não é válido. K40 e K12 nem ficam significativos assim.

## Recomendação
Antes de gastar R$ 1,2 milhão, fazer uma etapa de confirmação barata: retestar K40, K12 e K19 (e eventualmente os próximos da lista, como K31) em várias placas independentes, com controle positivo e dose-resposta, e com a hipótese e o critério definidos antes. Só os que se replicarem devem avançar.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Com 40 compostos testados sem hipótese prévia, 3 significativos a 5% é o que se espera por acaso (2 esperados). Corrigindo por multiplicidade, nenhum sobrevive (Holm p = 1,0; FDR q = 0,49), e os p-valores se distribuem como sob ausência total de efeito. Os três precisam de replicação antes de investir R$ 400 mil em cada.
```
