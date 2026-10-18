# Opinion — Case 07: sleep hygiene app (effect by sex)

Script: `bracoA/scripts/caso_07.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "sleep = individual level + app effect (possibly heterogeneous by sex) + wristband measurement noise"
    camada_onde_pode_falhar: "L2/L3: concluding 'no effect in men' from p > 0,05 and 'difference between sexes' without testing the interaction"
  nao_linearidade_jensen: "additive outcome in minutes, without a heavy tail (285–516); adequate mean"
  unidade_independente_e_margem:
    unidade: "participant, 60 per cell"
    margens:
      mulheres: "+17,5 min (95% CI 3,9 to 31,1)"
      homens: "+8,2 min (95% CI -8,2 to 24,6)"
      diferenca_entre_sexos: "-9,3 min (95% CI -30,4 to +11,9)"
  fisica_ruido_agente_ativo: "without absorbing states; Gaussian noise; SD 37–47 min"
  escada_causal:
    degrau: "intervention (draw stratified by sex)"
    identificacao: "randomization within each sex; the interaction is identified, but with little precision"
    premissa_nao_testavel: "the wristband measures sleep equally in both sexes"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "240 adults (enrollment by advertisement, complaint of little sleep)"
  estratos: "120 F / 120 M; 60 app and 60 control in each sex"
  chave: "participant, without duplicates; 0 missing; 0 losses"
  escopo: "self-selected adults with a complaint; not the general population of users"
```

## 3. DAG

```
sorteio(estrato sexo) ──► app ──► sono ◄── sexo
                              (sexo como possível modificador de efeito)
```

## 4. Null model and severe test

- Reproduced: F +17.5 (p = 0.012); M +8.2 (p = 0.32).
- **Test of the real claim (app × sex interaction):** -9.25 min, p = 0.39 (OLS); permutation p = 0.40. There is no evidence that the effect differs between the sexes.
- Common effect (additive model): +12.9 min (95% CI 2.3 to 23.4), p = 0.017.
- **Simulated null:** if the effect were **equal** in both sexes (+12.9 min, observed SD), the probability of getting "one sex significant and the other not" is **48%**. The observed pattern is the typical result of a homogeneous effect with 60 per arm.
- "p = 0.32 in men" means lack of precision, not absence of effect: the CI of the men includes the effect estimated in the women.
- K = 4 specifications.

## 5. Lenses

- **Crisonomy:** the decision (abandon men) has a high cost: losing half of the market based on a difference that is not distinguishable from zero. The EVSI of a test of the interaction with a larger n is high.
- **Morphonomy:** the best compact description (MDL) is a common effect: the model with interaction does not improve the fit (p = 0.39).
- Scalonomy and Chrononomy: do not apply.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "replicates of a homogeneous effect reproduce the pair (p=0,012; p=0,32) with high frequency"
  modelo_rival: "common effect of ~13 min in both sexes: explains the data as well as the heterogeneous model"
  identificado_pelos_dados: ["total mean effect > 0"]
  nao_identificado: ["difference of effect between sexes (CI -30 to +12)"]
  performatividade: "does not apply"
  premissas_indecidiveis: "real effect in men between -8 and +25 min"
  tentativas_K: 4
```

## 7. Decision (F8)

The app seems to increase sleep by about 13 minutes per night in total (CI 2 to 23). The data do not show that the effect is different between women and men: the difference between the two effects has a CI from -30 to +12 minutes. Do not reorient the product nor cut the acquisition of men based on this. If segmentation matters, run a trial sized for the interaction (hundreds per cell).

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Having p < 0,05 in one sex and p > 0,05 in the other does not show that the effects are different. The app × sex interaction is not significant (p ≈ 0,39) and the CI of the men includes the effect of the women. An equal effect in both sexes would produce this pattern in ~48% of the times.
```
