# Opinion — Case 11: diesel additive

Script: `bracoA/scripts/caso_11.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "monthly consumption of the truck = truck level + route/load/climate of the month + driver + additive effect + noise"
    camada_onde_pode_falhar: "L1 (truck-to-truck and month-to-month variation described as large); L2 (n=5 vs 5, one month, without a baseline measure); L3 (budgeting the point estimate of 14%; test conducted by the supplier)"
  nao_linearidade_jensen: "annual savings = consumption × km × price: linear, but budgeting the point (14%) ignores the distribution; the CI of the savings ranges from 5% to 23%"
  unidade_independente_e_margem:
    unidade: "truck-month; n = 10"
    margem: "difference -5,09 L/100 km; 95% CI -8,3 to -1,9 (−23% to −5%); exact randomization test p = 0,0079 (the smallest p possible with 5 vs 5)"
  fisica_ruido_agente_ativo: "without absorbing states; large noise between trucks according to the client, but not measured here (no pre-test consumption); a single month"
  escada_causal:
    degrau: "intervention (draw by the supplier, blinded drivers)"
    identificacao: "randomization with 252 possible allocations; without a pre-period there is no adjustment for truck level"
    premissa_nao_testavel: "the supplier's draw was honest and did not pick more economical trucks (conflict of interest; not auditable)"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "10 trucks of the same model/year, similar routes, from a fleet of 800"
  chave: "caminhao, without duplicates"
  duracao: "1 month"
  linha_de_base: "ABSENT (consumption of the same trucks before the test not informed)"
  quem_sorteou: "the additive supplier itself"
  escopo_da_decisao: "800 trucks, possibly other models and routes, for 12 months"
  km_no_mes: "10732 vs 10187 (no km–consumption correlation, r=0,18)"
```

## 3. DAG

```
fornecedor ──► sorteio ──► aditivo ──► consumo ◄── nível_caminhão, rota_mês, motorista
                 ? (se o sorteio não foi cego ao nível do caminhão: fornecedor → seleção → consumo)
```

## 4. Null model and severe test

- Reproduced: 30.92 vs 36.01 (−14.1%), t p = 0.0067 (Welch 0.0093).
- **Exact null:** all 5 with the additive consumed less than all 5 without it. That occurs in 2 of 252 allocations under the null, p = 0.0079. There is a signal, but the evidence is at the maximum limit that a 5 vs 5 design can give, and it depends entirely on the integrity of a draw made by the interested party.
- **Significance filter (winner's curse), simulated with the observed SD (2.2 L/100 km):** with a real effect of 3%, the tests that reach significance estimate on average −8.6%; with 5%, −9.5%. Small studies that "work out" exaggerate the effect. The 14% are an inflated estimate, conditional on the result having been significant.
- The client reports that consumption "varies a lot from truck to truck from one month to another". The SD in this month (1.6 and 2.7) does not necessarily represent that variation, and there is no pre-period to separate the level of each truck.
- K = 6 specifications.

## 5. Lenses

- **Crisonomy:** the decision is double: hire for 800 trucks **and budget 14%**. Even if the additive works, the CI ranges from 5% to 23%, and the probable effect is smaller than 14% (winner's curse). Budgeting the point exposes the budget to a deficit. The EVSI of a larger test (e.g. 40–80 trucks, 3 months, with a pre-period and a draw by the carrier) is high relative to its cost.
- **Chrononomy:** a single month; it captures neither seasonality nor the persistence of the effect.
- **Morphonomy:** without replication in other models or routes; invariance was not tested.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "Gaussian model with common SD is plausible; with n=5 it is not tested"
  modelo_rival: "small real effect (3–5%) + favorable chance/selection: compatible with what is observed after the significance filter"
  identificado_pelos_dados: ["probable signal of reduction in this group and in this month"]
  trazido_de_fora: ["honesty of the supplier's draw", "transfer to the whole fleet and to 12 months"]
  performatividade: "the supplier controls the design; incentive to present the best result"
  premissas_indecidiveis: "previous level of each truck"
  tentativas_K: 6
```

## 7. Decision (F8)

In this test, the 5 trucks with the additive spent less, but with only 10 trucks in one month the real savings may be at any point between ~5 and ~23 per 100 liters, and the most likely is that it is less than 14. Do not sign for the whole fleet with 14% in the budget. A test of our own is recommended: draw by the carrier, ≥ 40 trucks, consumption 2–3 months before and during, and a difference-in-differences analysis. If a decision is needed right away, a pilot contract with a conservative budget (the lower limit, ~5%) and a termination clause.
Exit clause: terminate if, after 3 months, the savings measured against an internal control group are smaller than the cost of the additive.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: With 5 against 5 trucks, a single month, without a baseline and with a draw made by the supplier itself, the savings lie between ~5% and ~23%, and the 14% are inflated by the significance filter. The data do not sustain budgeting 14% for 800 trucks.
```
