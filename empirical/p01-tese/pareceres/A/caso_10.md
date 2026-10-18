# Opinion — Case 10: change of shipping packaging (non-inferiority)

Script: `bracoA/scripts/caso_10.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "damage ~ Bernoulli(p_embalagem, mês); record made by the customer within up to 15 days"
    camada_onde_pode_falhar: "L1 (heterogeneity by month/seasonality; record by the customer) and L2 (margin and decision rule)"
  nao_linearidade_jensen: "low proportions (~2%); linear cost per damage. The absolute difference is the pre-defined metric (relative risk 1,04, cited only as a complement)."
  unidade_independente_e_margem:
    unidade: "shipment, drawn individually; n = 40000"
    margem: "difference +0,07 p.p.; 95% CI -0,20 to +0,34 p.p.; 90% CI -0,16 to +0,30 p.p.; pre-registered margin 0,5 p.p."
  fisica_ruido_agente_ativo: "rare but numerous events (382 and 396); without absorbing states; the sorting could treat packages differently if it saw the packaging (not blindable)"
  escada_causal:
    degrau: "intervention (draw by order)"
    identificacao: "randomization; proportion 50,0% new, stable by month (49,6%–50,6%)"
    premissa_nao_testavel: "the probability that the customer records the damage does not depend on the packaging"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "40000 shipments, 20000 per arm, 3 months"
  chave: "envio, without duplicates; 0 missing; outcome recorded in all"
  pre_registro: "margin of 0,5 p.p. and decision rule defined beforehand"
  por_mes_nova_menos_atual_pp: {mes1: +0.29, mes2: +0.27, mes3: -0.35}
```

## 3. DAG

```
sorteio ──► embalagem ──► avaria ◄── mês (transporte, sazonalidade)
                            │
                            └──► registro pelo cliente
```

## 4. Null model and severe test

- Reproduced: 1.91% vs 1.98%; 95% CI -0.20 to +0.34 p.p. (Wald and Newcomb equal).
- Stratified by month: +0.07 p.p. (CI -0.20 to +0.34).
- Bootstrap (5000): CI -0.20 to +0.35; P(difference ≥ 0.5 p.p.) = 0.0006.
- **Null at the margin (severe test):** simulating a real worsening of exactly +0.5 p.p., the rule "upper limit < 0.5" would be satisfied only in 2.4% of the times (type I error ≤ 2.5% one-sided). If the truth were "no difference", the rule would approve in 95%. The test had the power to separate the hypotheses.
- Heterogeneity by month: interaction p = 0.10. In months 1–2 the new packaging was ~0.3 p.p. above; the monthly CIs (up to +0.76 p.p.) are wide for having one third of the n. That does not invalidate the combined result, but justifies monitoring.
- K = 6 specifications.

## 5. Lenses

- **Crisonomy:** the C/L rule was pre-registered (margin 0.5 p.p.) and met with room to spare (upper limit 0.34).
- **Chrononomy:** 3 months; transport seasonality (end-of-year peaks) not covered, so validity outside the window is not guaranteed.
- **Scalonomy:** rare events but with sufficient counts; normal approximation adequate.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "binomial with a common rate per arm reproduces the monthly counts (interaction p=0,10)"
  modelo_rival: "new worse by 0,5 p.p.: incompatible (bootstrap P 0,0006)"
  identificado_pelos_dados: ["damage difference in 3 months"]
  trazido_de_fora: ["non-differential record by the customer", "stability in other seasons"]
  performatividade: "the sorting team may treat the new packaging more carefully during the test"
  premissas_indecidiveis: "behaviour in peak months"
  tentativas_K: 6
```

## 7. Decision (F8)

Out of every 1000 shipments, about 19 had damage in the current packaging and 20 in the new one. The worst scenario compatible with the data is about 3 more per 1000 shipments (0.34 p.p.), below the agreed limit of 5 per 1000. The change is sustained by the pre-registered rule.
Exit clause: monitor the monthly damage with a reference group (e.g. 5–10% of the shipments in the current packaging). If the accumulated difference exceeds +0.5 p.p. with a 95% one-sided CI, or if in a peak month it exceeds +1 p.p., return to the current packaging.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: With a draw by shipment, a complete outcome and a pre-registered margin, the upper limit of the 95% CI of the difference (+0,34 p.p.) stays below the margin of 0,5 p.p. The simulation at the margin shows that the test had a type I error ≤ 2,5% and a power of ~95%.
```
