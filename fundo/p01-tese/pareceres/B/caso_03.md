# Parecer — Caso 03: programa de incentivo para gerentes

## Desenho
São 60 lojas, 30 sorteadas para o programa. A unidade de análise (loja) coincide com a de sorteio, e o desfecho e o método foram pré-especificados: log da razão entre a venda média semanal das 12 semanas depois e a das 12 semanas antes. Não há faltantes: 60 lojas × 12 semanas, ninguém abriu ou fechou. A venda base é parecida entre os grupos (médias de 108,8 e 102,5 mil/semana, dentro do esperado sob sorteio). Como o desfecho é uma razão em relação à própria base, a diferença de nível entre lojas também não importa.

## Resultados
- Regressão do log-razão no grupo: +0,0679 (EP 0,015), ou seja +7,0% (IC 95% de +3,9% a +10,3%), p < 0,001. Os números do cliente se reproduzem.
- Robustez:
  - Erros HC3 dão o mesmo resultado, p < 0,001.
  - Mann-Whitney: p = 2e-5.
  - Teste de permutação com 20.000 rearranjos: p < 0,0001.
  - Nenhuma loja isolada domina; a distribuição do grupo programa está toda deslocada (mediana 8,6% contra −0,5% no controle).
- Efeito por semana: a razão venda/base no grupo programa fica entre 1,05 e 1,10 em todas as 12 semanas, contra 0,98–1,04 no controle. É estável, sem pico inicial que se dissipe.

## Implicação para a decisão
O custo é cerca de 1% das vendas. Mesmo o limite inferior do IC (+3,9%) supera esse custo com folga, então a evidência de aumento de vendas é sólida. Duas ressalvas, que não mudam o veredito: o critério de decisão econômica deveria ser margem, não faturamento (o programa se paga se a margem incremental sobre +7% de vendas superar 1% das vendas); e 12 semanas não garantem persistência no longo prazo, de modo que vale monitorar após a expansão.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Experimento randomizado por loja, com análise pré-especificada na unidade correta: +7,0% (IC 95% de 3,9% a 10,3%), robusto a testes não paramétricos e de permutação, e estável nas 12 semanas. O limite inferior supera o custo de ~1% das vendas.
```
