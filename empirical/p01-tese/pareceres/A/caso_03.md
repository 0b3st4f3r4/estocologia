# Opinion — Case 03: incentive program for store managers

Script: `bracoA/scripts/caso_03.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "weekly store sales = store level × common seasonality × program effect × weekly noise"
    camada_onde_pode_falhar: "L1 (common shock or different trend between groups) and L2 (log scale, unit)"
  nao_linearidade_jensen:
    dinamica: "multiplicative (stores range from 37 to 272 thousand/week); the log scale is the adequate one"
    jensen: "exp(E[log ratio]) - 1 = 7,0%; ratio of the arithmetic means of the T/C ratios = 1,070. The difference is negligible."
  unidade_independente_e_margem:
    unidade: "store (draw by store); n = 60, without pseudoreplication of the 720 weeks"
    margem: "95% CI OLS 3,9% to 10,3%; HC3 3,9% to 10,2%; bootstrap by store 4,0% to 10,0%"
  fisica_ruido_agente_ativo: "without absorbing states (no store closed). Incentivized managers may bring sales forward or manipulate the metric (Goodhart). Verified: the effect holds over the 12 weeks, without an initial peak followed by a fall."
  escada_causal:
    degrau: "intervention (draw of 30 of 60 stores)"
    identificacao: "randomization + pre/post difference; pre-specified analysis"
    premissa_nao_testavel: "no contamination between stores; the cash register system measures real sales (not displaced from other stores)"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "60 stores (the whole chain), 30/30"
  chave: "loja; store×week sales complete (720, without duplicates or missing values)"
  selecao: "none; no store opened or closed"
  balanco_baseline: "base sales 108,8 vs 102,5 thousand (t p = 0,59)"
  pre_registro: "analysis by store and comparison with the previous period defined before the start"
  valores_extremos: "J25 (~300 thousand/week) and J06 (~33 thousand); maximum influence (Cook) 0,13; leave-one-out 6,3%–7,3%"
```

## 3. DAG

```
sorteio ──► programa ──► venda_pós ◄── venda_pré (nível da loja)
                              ▲
                     choque_comum(semana)  (reaches both groups and is cancelled in the contrast)
```

The log of the post/pre ratio removes the store level. The common shock is cancelled by the control group.

## 4. Null model and severe test

- Reproduced: difference in log 0.0679, that is +7.0% (CI 3.9%–10.3%), p = 2.7e-5.
- **Null by permutation of the labels (20000):** p = 5e-5.
- Mann-Whitney p = 1.9e-5; ANCOVA log(post) ~ T + log(pre): 6.9% (CI 3.9%–9.9%).
- **Severity:** under the null, a difference ≥ 6.8% appears in fewer than 1 in 10000 draws. If the real effect were ≤ 1% (the cost), the observed lower limit (3.9%) would be very unlikely.
- Weekly trajectory: post/pre ratio of the program between 1.05 and 1.10 in all 12 weeks; control between 0.98 and 1.04. There is no sign of a transitory effect.
- K = 5 specifications; all agree.

## 5. Lenses

- **Chrononomy:** multiplicative dynamics treated in log. The 12-week horizon does not guarantee persistence; the effect does not decay inside the window.
- **Morphonomy:** the effect is stable in the leave-one-out and between weeks.
- **Crisonomy:** the estimated effect (7%, lower limit 3.9%) exceeds the cost of 1% of sales. Caution: the cost is on sales and the benefit as well, but the profit depends on the margin; with a low gross margin, +3.9% in sales may not pay for 1% of sales. That account belongs to the client, and the question is about sales.
- **Fluxonomy/Goodhart:** the incentive may displace sales between periods or channels; there is no fall after the peak inside the window.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "distribution of the log-ratios in the two groups with similar SD (0,057 vs 0,058); compatible with a displacement effect"
  modelo_rival: "ANCOVA in log and non-parametric test; same result"
  identificado_pelos_dados: ["relative mean effect in 12 weeks"]
  trazido_de_fora: ["persistence beyond 12 weeks", "absence of contamination"]
  performatividade: "Goodhart risk (managers optimize the metric); monitor margin, returns and sales between stores"
  premissas_indecidiveis: "long-term effect; effect on profit"
  tentativas_K: 5
```

## 7. Decision (F8)

In 60 drawn stores, the stores of the program grew about 7 out of every 100 reais more than those of the control (CI 4 to 10). The cost of the program is about 1 out of every 100 reais sold. The extension to the chain is sustained for the goal "increase sales".
Exit clause: keep a control group of stores (or a rotation) for 2 quarters. If the measured effect falls below 1% (the cost), or if margins and returns worsen, suspend.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: The design is randomized by store and pre-specified, and the unit of analysis is correct. The effect of about +7% (CI 3,9%–10,3%) resists the permutation, the leave-one-out and alternative specifications, and holds over the 12 weeks.
```
