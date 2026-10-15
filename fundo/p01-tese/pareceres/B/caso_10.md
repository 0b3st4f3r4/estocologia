# Parecer — Caso 10: embalagem de envio (não inferioridade)

## Desenho
O desenho é bom: sorteio por envio com probabilidade 1/2, margem de não inferioridade (+0,5 p.p.) e regra de decisão registradas antes, 20.000 envios por braço e desfecho completo. A pergunta é se o limite superior do IC da diferença (nova − atual) fica abaixo de +0,5 p.p.

## Resultados
- Avaria: 1,91% na atual e 1,98% na nova. Diferença de +0,07 p.p., com IC 95% de −0,20 a +0,34 p.p., o que reproduz a conta do cliente.
- O critério convencional de não inferioridade usa o IC bilateral de 95%, equivalente a um teste unilateral a 2,5%. O limite superior, 0,34 p.p., fica abaixo da margem de 0,5 p.p. Pelo IC de 90%, o limite seria 0,30.
- Ajustando por mês (OLS com erros robustos): +0,07 p.p., IC de −0,20 a +0,34, idêntico.
- Heterogeneidade por mês: as diferenças foram +0,29 (mês 1), +0,27 (mês 2) e −0,35 p.p. (mês 3). O teste da razão de verossimilhança para interação embalagem × mês dá p = 0,10, sem evidência de que o efeito mude com o tempo. Os ICs mensais individuais são largos (limites superiores de ~0,75) por terem um terço da amostra cada. Não são a análise pré-registrada e não devem substituí-la.
- Os grupos estão balanceados por mês (≈ 6.600–6.700 por célula).

## Ressalvas
- A margem de 0,5 p.p. equivale a até ~26% de aumento relativo sobre 1,9%. Isso foi decisão de negócio do cliente, registrada antes, e é respeitada aqui.
- A avaria é registrada pelo cliente em até 15 dias, o que mede avaria reclamada. Vale monitorar a taxa após a troca.

## Conclusão
Pela regra pré-registrada, a nova embalagem é não inferior: o aumento máximo plausível da avaria (+0,34 p.p.) fica abaixo da margem de 0,5 p.p.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Ensaio randomizado grande, com margem e regra pré-registradas. A diferença de avaria (nova − atual) é de +0,07 p.p., com IC 95% de −0,20 a +0,34 p.p., cujo limite superior está abaixo da margem de 0,5 p.p., sem heterogeneidade significativa por mês.
```
