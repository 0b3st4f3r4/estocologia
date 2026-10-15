# Parecer — Caso 06: programa de emprego apoiado

## Problema central: comparação com um grupo incomparável
Os participantes são homens que se inscreveram num programa para pessoas com dificuldade de manter emprego. A amostra nacional representa a população trabalhadora em geral. Antes do programa, os grupos já eram radicalmente diferentes:

| | Programa | Amostra nacional |
|---|---|---|
| Idade | 25,8 | 33,2 |
| Anos de estudo | 10,4 | 12,0 |
| Negro | 84% | 7% |
| Casado | 19% | 71% |
| Sem ensino médio | 71% | 30% |
| Renda 1974 | US$ 2.096 | US$ 14.017 |
| Renda 1975 | US$ 1.532 | US$ 13.651 |
| Sem renda em 1975 | 60% | 11% |

A diferença de renda em 1978 (−8.498) é menor que a diferença que já existia em 1975 (cerca de −12.100). Isso é seleção, não efeito do programa. Controlar só por idade e estudo (−6.485, que reproduzi) deixa de fora justamente o preditor mais forte da renda futura: a renda passada.

## Análises que levam em conta a seleção
- Regressão com idade, estudo (com termos quadráticos), raça, estado civil, diploma, rendas de 1974–75 e indicadores de renda zero: +1.184 (IC 95% de −46 a +2.414).
- Diferenças em diferenças (mudança de renda 1975→1978): +3.621 (IC de 2.426 a 4.817) sem covariáveis; ajustada, +1.184.
- Escore de propensão:
  - Os participantes têm escore médio de 0,40, contra 0,007 dos controles, o que mostra sobreposição pobre. Apliquei aparamento ao suporte comum (4.458 controles).
  - Pareamento 5-NN no logit do escore: efeito sobre os tratados (ATT) de +1.344, com IC bootstrap de 95% de −116 a +3.014. Com o pareamento, o balanceamento das covariáveis ficou bom (renda 1975: 1.532 contra 1.688; negro: 84% contra 86%).
  - Ponderação por propensão (ATT): +1.593.
- Restrito a pessoas com renda < 5.000 em 1974 e 1975: +2.612.

Todas as estimativas que ajustam pela renda prévia têm sinal positivo, ou seja, apontam para ganho de renda e nunca para perda. Mesmo assim, são estimativas observacionais com sobreposição limitada e dependem de que não haja confusão residual. O cenário ideal seria comparar com um grupo controle sorteado entre os próprios inscritos, se existir.

## Conclusão
A conclusão do conselho, de que o programa reduziu a renda, resulta de comparar populações diferentes desde o início. Os dados não a sustentam. Ajustados pela história de renda, indicam efeito nulo a positivo.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: A diferença de −8.498 reflete seleção: os participantes já ganhavam cerca de US$ 12 mil a menos em 1975 e diferiam em raça, estado civil e estudo. Ajustando pela renda pré-programa (regressão, DiD, pareamento por propensão), as estimativas ficam entre +1,2 e +3,6 mil, sem sinal de redução de renda.
```
