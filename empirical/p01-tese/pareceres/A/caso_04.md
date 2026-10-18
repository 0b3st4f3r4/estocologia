# Opinion — Case 04: new checkout (A/B test)

Script: `bracoA/scripts/caso_04.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "binary conversion per user; version drawn 50/50; heterogeneity by segment unknown"
    camada_onde_pode_falhar: "L3/L2: the choice of the segment was made after seeing the data (garden of forking paths)"
  nao_linearidade_jensen: "proportions; binary outcome, without a heavy tail"
  unidade_independente_e_margem:
    unidade: "user (individual draw); n = 6000"
    margem_metrica_pre_registrada: "7d difference = -0,13 p.p. (95% CI -1,53 to +1,26 p.p.)"
    margem_segmento: "+7,1 p.p. with n = 478; CI 1,9 to 12,7 p.p., not corrected for selection"
  fisica_ruido_agente_ativo: "without absorbing states; 7/14/30d windows nested and coherent (0 monotonicity violations)"
  escada_causal:
    degrau: "intervention (A/B)"
    identificacao: "valid randomization (age, device and new customer balanced)"
    premissa_nao_testavel: "no critique; the problem is inferential (selection of the cut), not of identification"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "6000 users, 3000/3000"
  chave: "usuario (without duplicates); 0 missing"
  pre_registro: "primary metric: 7d conversion of ALL those drawn"
  recortes: "age, device, customer type and 14/30d windows chosen afterwards (not pre-registered)"
  balanco: "age 43,8 vs 44,3; computer 39,0% vs 39,5%; new customer 39,4% vs 41,0%"
```

## 3. DAG

```
sorteio ──► versão ──► conversão ◄── idade, dispositivo, histórico
                                 (effect modifiers, not confounders)
analista ──► escolha_do_recorte ◄── dados de resultado   (seleção dependente do desfecho)
```

## 4. Null model and severe test

- **Pre-registered metric:** new 8.20% vs current 8.33%, p = 0.85. In the 14d and 30d windows the new one is numerically **worse** (-0.8 and -1.4 p.p.).
- Reproduced: alleged segment 13.0% vs 5.9%, p = 0.0072.
- In a grid of 162 cuts (3 devices × 6 age cuts × 3 customer types × 3 windows), 18 have p < 0.05, and the strongest ones favor the **current** checkout (e.g. > 35 years, old customer, 14d: -4.1 p.p., p = 0.002).
- **Simulated null:** permuting the version labels (zero effect by construction) and searching for the smallest p in the same grid, the minimum stays ≤ 0.007 in **30%** of the simulations and < 0.05 in 83%. The p = 0.007 of the segment is compatible with pure noise plus the search.
- Bonferroni for K = 162: threshold 0.0003; no cut passes.

## 5. Lenses

- **Crisonomy:** the decision covers all users. For that public, the best estimate is ~0 (CI -1.5 to +1.3 p.p.), and the longer windows suggest a possible loss.
- **Morphonomy (invariance):** the effect is not invariant between windows or between cuts; it changes sign.
- **M12 (count of attempts):** K effective unknown on the client side; the reconstructed grid has at least 162.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "under the global null, the distribution of the minimum p of the grid has median 0,016; the observed 0,007 is in the lower third, it is not extreme"
  modelo_rival: "homogeneous null effect: explains the primary metric and the finding of the segment"
  identificado_pelos_dados: ["global mean effect ≈ 0"]
  trazido_de_fora: ["no a priori hypothesis about the segment"]
  performatividade: "does not apply"
  premissas_indecidiveis: "real effect in small subgroups; it would require a dedicated confirmatory test"
  tentativas_K: ">= 162 (reconstructed)"
```

## 7. Decision (F8)

Out of every 100 users, about 8 convert in 7 days with either of the two versions. The test shows no gain with the new checkout for the total public, which is the metric combined before the test. The highlighted segment was chosen after seeing the results, and an equal pattern appears by chance in ~3 out of every 10 searches without a real effect. Do not replace based on this test. If the segment matters, run a new confirmatory A/B only in it, with the metric and the size pre-registered.
Exit clause: if the change is made anyway, monitor the 30d conversion and revert if it falls ≥ 1 p.p.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: In the pre-registered metric (7d conversion of all users) there is no effect: -0,13 p.p., p = 0,85. The significant segment was chosen afterwards, among at least 162 possible cuts, and an equal or smaller p comes out of the search under the null in ~30% of the times.
```
