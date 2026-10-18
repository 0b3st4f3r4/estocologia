# Opinion — Case 05: salesperson selection test

Script: `bracoA/scripts/caso_05.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "1st-year sales = f(aptitude, portfolio/territory, market) + noise; score = noisy measure of aptitude"
    camada_onde_pode_falhar: "L2: Pearson is dominated by a single extreme point; L3: the cutoff rule at 70 extrapolates a relation that does not exist in the bulk of the data"
  nao_linearidade_jensen: "the cutoff rule is a step function of the score; the mean of sales above 70 (496) is pulled by one point. The median above 70 (413,5) is equal to the median below (413,2)."
  unidade_independente_e_margem:
    unidade: "salesperson, n = 30"
    margem: "bootstrap of r: 95% CI -0,19 to 0,87 (bimodal, depending on whether point V29 enters or not)"
  fisica_ruido_agente_ativo: "sales have a right tail (V29 = 980 thousand, z = 12 relative to the others); without absorbing states; all completed 12 months"
  escada_causal:
    degrau: "association (prediction), adequate for a selection decision"
    identificacao: "does not need causality, but needs a robust and generalizable association"
    premissa_nao_testavel: "last year's hiring did not depend on the score (stated); the interview filtered in an equal way"
```

## 2. Sample card

```yaml
ficha_amostra:
  populacao_alvo: "candidates approved in the interview"
  amostra: "30 hired last year (all those approved, without a filter by score)"
  chave: "vendedor, without duplicates; 0 missing"
  ponto_influente: "V29: score 97 (z = 4,9) and sales 980 (z = 12,3); the card must check for a recording error or a special circumstance (e.g. inherited portfolio)"
  com_nota_>=_70: "6 salespeople, 5 of them with sales between 339 and 436"
```

## 3. DAG

```
aptidão ──► nota_teste
aptidão ──► vendas ◄── território/carteira ◄── (alocação do vendedor?)
entrevista (filtro) ──► amostra
```

If V29 received a large portfolio for being "promising", there would be a path score → allocation → sales that does not generalize to a cutoff rule.

## 4. Null model and severe test

- Reproduced: Pearson r = 0.648, p = 1.1e-4.
- **Without V29:** r = 0.106 (p = 0.58); Spearman 0.05.
- **With everyone, but with robust measures:** Spearman 0.14 (p = 0.45); Kendall 0.11 (p = 0.39).
- Leave-one-out: r ranges from 0.11 (without V29) to 0.71. **A single salesperson fabricates the correlation.**
- Null by permutation: with V29, p = 0.0099 (the permutation keeps the leverage of the point, and the result remains dependent on one point); without V29, p = 0.58.
- Proposed rule (≥ 70): without V29, the 5 with score ≥ 70 sold on average 399 against 414 for the others (Mann-Whitney p = 0.52). The rule would eliminate salespeople as good as those approved.
- K = 6 specifications.

## 5. Lenses

- **Scalonomy:** with a right tail, the mean and the Pearson correlation are dominated by one point; use rank statistics, the median and the eCDF. The eCDF of sales by score group is practically overlapped without V29.
- **Crisonomy:** the eliminatory filter has a cost: losing good salespeople, and 24 of every 30 hired last year would have been rejected. The benefit depends on a single observation.
- **Morphonomy:** the relation is not invariant to the removal of one point.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "a linear regression fitted without V29 predicts sales of ~450 for score 97, against 980 observed: V29 is inconsistent with the rest"
  modelo_rival: "no association + one outlier: explains the data equally (r_sem_V29 = 0,11)"
  identificado_pelos_dados: ["absence of association between scores 45 and 75"]
  trazido_de_fora: ["nature of the V29 case"]
  performatividade: "a filter at 70 changes the hired population (sample truncated in the future); validating it later becomes impossible without keeping candidates below the cutoff"
  premissas_indecidiveis: "whether V29 reflects exceptional aptitude or luck/portfolio"
  tentativas_K: 6
```

## 7. Decision (F8)

In 29 of the 30 salespeople, the score does not order the sales: those who scored 70 or more sold, in the median, the same as the others. The "strong correlation" comes from a single salesperson. Do not adopt the eliminatory cutoff. Investigate V29 and collect more cohorts, keeping the test as non-eliminatory information.
Exit clause: only reconsider if, in ≥ 2 cohorts, the rank correlation is > 0.3 without depending on one point.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: The r = 0,65 comes almost entirely from a single extreme salesperson (V29, score 97, sales 980). Without him r = 0,11, and the robust measures (Spearman 0,14) show no association. Above 70, the median of sales is the same as that of the others.
```
