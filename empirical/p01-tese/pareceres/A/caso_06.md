# Opinion — Case 06: supported employment program (1970s)

Script: `bracoA/scripts/caso_06.py` (seed 20261015; uses scikit-learn for the propensity score).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "1978 income = f(employment history, schooling, age, race, marital status) + program effect; self-selected participation among people with difficulty in employment"
    camada_onde_pode_falhar: "L1/L2: the compared groups come from different populations (selection into the program); the regression with age and schooling does not remove that bias"
  nao_linearidade_jensen: "income with mass at zero (program: 71% with zero income in 1974; national: 12%) and a right tail; the mean effect must be complemented by the proportion with income > 0"
  unidade_independente_e_margem:
    unidade: "person; 185 treated vs 15992 comparisons"
    n_efetivo_comparavel: "only ~236 controls with propensity score > 0,1; the matching used 113 distinct controls"
  fisica_ruido_agente_ativo: "zero income is almost an absorbing state (unemployment); regression to the mean expected: those who enroll are in an income valley"
  escada_causal:
    degrau: "intervention alleged (the program reduced income) from observational data"
    identificacao: "backdoor adjustment with all pre-treatment covariates (including 1974/75 income), propensity matching, difference-in-differences"
    premissa_nao_testavel: "conditional ignorability given the observed covariates"
```

## 2. Sample card

```yaml
ficha_amostra:
  tratados: "185 men enrolled in a program aimed at people with difficulty in employment (entry filter = disadvantage)"
  comparacao: "15992 respondents of a national survey (general working population)"
  desbalanco_pre_tratamento:
    idade: "25,8 vs 33,2"
    anos_estudo: "10,3 vs 12,0"
    negro: "84% vs 7%"
    casado: "19% vs 71%"
    sem_diploma_medio: "71% vs 30%"
    renda_1974: "2096 vs 14017"
    renda_1975: "1532 vs 13651"
  suporte_comum: "propensity of the controls: median 0,0001; most of the controls are not comparable"
  chave: "pessoa, without duplicates; 0 missing"
```

## 3. DAG

```
desvantagem_laboral ──► inscrição_no_programa
desvantagem_laboral ──► renda_1974/75 ──► renda_1978
desvantagem_laboral ──────────────────────► renda_1978
programa ──► renda_1978
```

The raw comparison mixes the effect of the program with the backdoor paths "disadvantage → participation" and "disadvantage → income". Age and schooling block only a part of it; pre-program income is the strongest proxy of disadvantage and was omitted.

## 4. Null model and severe test

- Reproduced: raw difference -8498; with age and schooling -6485.
- **Placebo test (pre-treatment null):** the same regression applied to the income of **1975**, before the program, gives -8906. The negative "effect" already existed before the program; it measures selection, not the program.
- Full adjustment (all covariates, 74/75 income and zero-income indicators): **+1184** (95% CI -46 to +2414).
- Difference-in-differences 1975→1978: +3621 (CI 2426 to 4817). Part of that may be regression to the mean.
- 1-NN matching by propensity (ATT): **+1914** (bootstrap 95% CI -50 to +3593); balance after matching is good (age 25.8 vs 25.8; negro 0.8 vs 0.8; 1975 income 1532 vs 1374).
- IPW (ATT): +1600. Trimmed sample (ps ≥ 0.01): +1248 (CI -21 to +2517).
- No specification that respects the backdoor gives a negative effect. K = 7.

## 5. Lenses

- **Scalonomy:** income with mass at zero and a tail; the mean alone is insufficient. Proportion with zero income in 1978: 24% in the program (against 60–71% before) and 14% in the national sample.
- **Chrononomy:** regression to the mean (enrollment in an income valley) inflates the DiD; that is why matching on pre income is preferable.
- **Crisonomy:** the proposed decision (cut the funding because the program reduces income) rests on an effect the data do not show. The probable sign is positive.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "the client's model predicts a difference of ~-8900 in 1975, before the program: it is falsified by the placebo"
  modelo_rival: "adjustment by propensity/pre income: effect ≈ +1200 to +1900, CI close to including 0"
  identificado_pelos_dados: ["extreme imbalance", "lack of common support for most of the controls"]
  trazido_de_fora: ["ignorability given the income history"]
  performatividade: "does not apply"
  premissas_indecidiveis: "unobserved variables (motivation, health); the ideal comparison is a control group drawn among the enrolled"
  tentativas_K: 7
```

## 7. Decision (F8)

The comparison with the general population compares very different people: before the program, the participants already earned about 12 thousand dollars less per year. Compared with similar people, the participants earned in 1978 about 1.2 to 1.9 thousand **more**, with an uncertainty that ranges from ~0 to ~3.6 thousand. The conclusion "the program reduced income" does not hold and must not ground the cut. If there is a new evaluation, draw the slot among the enrolled.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: The negative difference reflects the selection of the participants, not the program. The same "effect" already appears in the 1975 income, before the program. With adjustment by pre-program income and covariates (regression, matching, IPW), the estimate stays between 0 and positive, never negative.
```
