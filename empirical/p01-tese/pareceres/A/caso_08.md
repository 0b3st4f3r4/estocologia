# Opinion — Case 08: change of bearing supplier

Script: `bracoA/scripts/caso_08.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "defect ~ Bernoulli(p_fornecedor, lote), with possible variation between batches"
    camada_onde_pode_falhar: "L2/L3: treating 'not significant' as 'equivalent' (absence of evidence ≠ evidence of absence)"
  nao_linearidade_jensen: "the cost of the defect is asymmetric (rework of the whole engine); the expected loss is convex in p"
  unidade_independente_e_margem:
    unidade: "part within a batch; 10 batches (5 per supplier); the variation between batches is small here"
    margem: "new-current difference = +1,3 p.p.; 95% CI (Newcomb) -4,4 to +7,1 p.p.; 90% CI -3,4 to +6,1 p.p.; CI pooled by batch -4,0 to +6,7 p.p."
  fisica_ruido_agente_ativo: "rare events (8 and 10 defects); binomial noise dominates"
  escada_causal:
    degrau: "comparison between suppliers (association, but it is the quantity of interest)"
    identificacao: "random sampling within the batches; same team and procedure"
    premissa_nao_testavel: "the 5 batches represent the future production of the new supplier"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "300 parts: 30 drawn from each one of 10 batches"
  chave: "peca, without duplicates; 0 missing"
  defeitos_por_lote: {atual: [2,0,2,0,4], novo: [3,1,3,1,2]}
  margem_relevante_definida_pelo_cliente: "+3 p.p."
  escopo: "5 batches of the new supplier is a small sample of its process variation"
```

## 3. DAG

```
fornecedor ──► processo_fabril ──► lote ──► defeito ──► (inspeção, igual para ambos)
```

## 4. Null model and severe test

- Reproduced: 8/150 vs 10/150, Fisher p = 0.81.
- The question is one of **non-inferiority** with a margin of +3 p.p. The right test is to check whether the upper limit of the CI is < 3 p.p. It is **+7.1 p.p.** (95% CI) or +6.1 p.p. (90% CI, one-sided 5%). **Not demonstrated.**
- **Power (simulation):** if the new one were in fact 3 p.p. worse (8.3% vs 5.3%), the Fisher test with 150+150 would detect the difference only **12%** of the times. The "p = 0.81" had an 88% chance of appearing even with the relevant worsening.
- Posterior (Beta(1.1)): P(difference ≥ 3 p.p.) ≈ **27%**; P(new worse) ≈ 68%.
- To demonstrate non-inferiority with a margin of 3 p.p. (rates ~6%, power 80%) it would take ~770 parts per supplier, from more batches.
- K = 6 specifications.

## 5. Lenses

- **Crisonomy:** asymmetric risk: 15% of savings against rework of the whole engine. With ~1 chance in 4 of the rate worsening ≥ 3 p.p., the decision depends on C/L. Even without these numbers, the premise "it does not get worse" is not demonstrated.
- **Scalonomy:** rare events; exact/score CI instead of Wald.
- **Chrononomy:** the quality of the supplier may change from batch to batch; 5 batches do not characterize the process.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "binomial model without batch effect is compatible (small variation between batches)"
  modelo_rival: "new 3 p.p. worse: compatible with the data (inside the CI)"
  identificado_pelos_dados: ["rates ~5–7%"]
  nao_identificado: ["whether the difference is < 3 p.p."]
  performatividade: "the supplier may send better batches during qualification"
  premissas_indecidiveis: "quality of the future batches"
  tentativas_K: 6
```

## 7. Decision (F8)

Out of every 100 bearings, about 5 of the current supplier and 7 of the new one came out defective. With 150 parts of each, the data are compatible both with "equal" and with "up to 7 more per 100", which is well above the limit of 3 per 100 accepted by quality. The inspection does not show that the change is safe. Options: extend the inspection (~770 parts per supplier, from more batches) before changing, or change partially with incoming inspection.
Exit clause: if the rate of the new one, accumulated by batch, exceeds that of the current one by 3 p.p. with a one-sided CI, return to the current supplier.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: p = 0,81 does not demonstrate equivalence. The 95% CI of the difference (-4,4 to +7,1 p.p.) includes with room to spare the relevant worsening of 3 p.p., and the test had only ~12% power to detect it. Non-inferiority was not demonstrated.
```
