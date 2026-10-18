# Opinion — Case 02: supplement in the piglet feed

Script: `bracoA/scripts/caso_02.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "weight gain = effect of the pen (shared feeder, environment, competition, health) + effect of the feed + individual variation"
    camada_onde_pode_falhar: "L2: the t test treats 240 piglets as independent, but the treatment was drawn by pen (8 units)"
  nao_linearidade_jensen: "additive outcome (kg); without a heavy tail (gain 12,9–29,8 kg). Jensen barely relevant; the decision (30 thousand piglets/year) is linear in the mean gain."
  unidade_independente_e_margem:
    unidade: "pen (unit of randomization and of feeding)"
    n_efetivo: "8 pens (4 vs 4); ICC = 0,58, so the design effect is ~1+29*0,58 ≈ 18"
    margem: "difference 2,26 kg; SE per pen 1,68–1,82 kg; 95% CI ≈ -1,3 to 5,8 kg"
  fisica_ruido_agente_ativo: "without absorbing states (no deaths). The variation between pens (var 6,4 kg²) is larger than the variation within the pen (4,6 kg²)."
  escada_causal:
    degrau: "intervention (draw of pens)"
    identificacao: "cluster randomization; the inference needs to be made at the pen"
    premissa_nao_testavel: "there is no interference between pens"
```

## 2. Sample card (M4)

```yaml
ficha_amostra:
  unidades: "240 piglets in 8 pens of 30; 4 pens drawn for the supplement"
  chave_primaria: "leitao (without duplicates)"
  faltantes: 0
  selecao: "no loss and no change of pen"
  balanco_baseline: "mean initial weight per pen between 6,7 and 7,3 kg"
  pseudorreplicacao: "YES: the treatment varies only between pens; the piglets are subsamples"
```

## 3. DAG

```
sorteio ──► ração(baia) ──► ganho_leitão ◄── efeito_baia (comedouro, ambiente)
                                  ▲
                              peso_inicial
```

The pen effect is not a confounder (the draw makes it independent of the feed), but it is a source of **correlated** variance. With only 4 pens per arm, it can produce differences of 2 kg on its own.

## 4. Null model and severe test

- Test of the claim (t per piglet): p = 3.6e-8 (reproduced).
- Means per pen, supplement: 20.4; 18.7; 25.2; 22.9. Standard: 21.1; 20.6; 16.2; 20.2.
- **Exact randomization test over the 70 possible allocations of 4 pens:** p = 0.29.
- t on the pen means: p = 0.26. SE pooled by pen: p = 0.18. Mixed model with initial weight: 2.27 kg (95% CI -1.3 to 5.8), p = 0.21.
- **Simulated null** (with no effect of the supplement, with the between-pen variance estimated): the t test per piglet gives p < 0.05 in **68%** of the simulations. The "p < 0.001" is what is expected even with no effect at all.
- K = 5 specifications (individual t, t per pen, cluster, mixed, exact permutation); all of those that respect the unit agree.

## 5. Lenses

- **Scalonomy/M4 (unit):** effective n = 8, not 240.
- **Crisonomy:** the decision is expensive and covers 30 thousand piglets/year. With a CI from -1.3 to +5.8 kg, the result can go from a loss to a large gain. The EVSI of a new trial with more pens is high.
- Chrononomy and Fluxonomy: do not apply.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "a model of independent piglets predicts pen means with SD ~0,5 kg; the observed ones range from 16,2 to 25,2 kg. The model is rejected."
  modelo_rival: "mixed model with random pen intercept: effect 2,3 kg, CI includes 0"
  identificado_pelos_dados: ["mean difference", "variances between and within pen"]
  trazido_de_fora: ["absence of interference between pens"]
  performatividade: "none"
  premissas_indecidiveis: "with 8 pens it is not possible to separate the effect of the supplement from the heterogeneity between pens"
```

## 7. Decision (F8)

The data do not justify the purchase for all farms. At the right scale (pens), the observed difference appears in about 3 out of every 10 possible draws even with no effect at all. A trial with more pens is recommended (e.g. ≥ 12 per arm, in more than one farm), analyzed by pen and with a pre-defined cost-benefit (the minimum kg that pay for the supplement). Exit clause: adopt only if the lower limit of the CI per pen exceeds the gain that pays the cost.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: The supplement was drawn by pen (8 units), not by piglet. The p < 0,001 comes from pseudoreplication. Analyzed by pen, the effect has p ≈ 0,2–0,3 and a 95% CI that includes zero (about -1,3 to 5,8 kg).
```
