# Parecer — Caso 08: troca de fornecedor de rolamentos

## Erro de raciocínio
Um p = 0,81 no teste de Fisher significa apenas que não se detectou diferença. Não demonstra equivalência. Para afirmar que a troca não piora a qualidade, é preciso mostrar que o aumento plausível da taxa de defeito fica abaixo da margem relevante, que a qualidade fixou em 3 p.p. Isso é um teste de não inferioridade.

## Resultados
- Taxas de defeito: atual 8/150 (5,3%), novo 10/150 (6,7%), diferença de +1,3 p.p. O Fisher p = 0,81 se reproduz.
- IC da diferença (novo − atual):
  - 95% Newcombe: −4,4 a +7,1 p.p.; Agresti-Caffo, −4,3 a +6,9.
  - 90% (equivale a um teste unilateral de não inferioridade a 5%): −3,4 a +6,1 p.p.
  - Com erro agrupado por lote (5 lotes por fornecedor), o IC de 95% vai de −4,0 a +6,7 p.p.
- Em todos os métodos o limite superior (≈ 6–7 p.p.) fica bem acima da margem de 3 p.p. Os dados são compatíveis com um aumento de defeitos de mais que o dobro do tolerado.
- Poder: com 150 peças por fornecedor, a chance de detectar um aumento real de 5,3% para 8,3% era de só ~27%. O estudo não tinha tamanho para responder à pergunta.
- Os lotes do novo fornecedor variam de 1 a 3 defeituosos em 30, sem lote extremo.

## Recomendação
Para demonstrar não inferioridade com margem de 3 p.p. a partir de uma taxa de ~5–6%, são necessárias da ordem de 1.000 ou mais peças por fornecedor, retiradas de vários lotes. Outra saída é uma troca gradual, com inspeção reforçada e regra de parada.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Ausência de significância não é equivalência. O IC da diferença de defeitos (novo − atual) vai até cerca de +6 a +7 p.p., bem acima da margem relevante de 3 p.p., e o estudo tinha só ~27% de poder para detectar esse aumento.
```
