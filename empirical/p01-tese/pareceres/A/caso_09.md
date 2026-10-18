# Opinion — Case 09: class size in early childhood education

Script: `bracoA/scripts/caso_09.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "grade = school effect + teacher/class effect + class size effect + characteristics of the student + noise"
    camada_onde_pode_falhar: "L1 (selective attrition: missing grades), L2 (unit of inference: classes/schools, not students), external validity (USA 1980s → local chain)"
  nao_linearidade_jensen: "additive point scale; grade with a ceiling (627 appears repeated in reading and math); distribution without a heavy tail"
  unidade_independente_e_margem:
    unidade: "class within school (randomization); inference clustered by school (79), more conservative"
    margem: "FE effect 16,0 points, 95% CI 7,4 to 24,6 (cluster by school); bootstrap of schools 6,9 to 23,7"
  fisica_ruido_agente_ativo: "without absorbing states; teachers know the condition (possible expectancy effect)"
  escada_causal:
    degrau: "intervention (draw within the school)"
    identificacao: "school fixed effect + randomization"
    premissa_nao_testavel: "missing values ignorable, conditional on the arm"
```

## 2. Sample card

```yaml
ficha_amostra:
  base: "6325 students in the year of early childhood education; 1900 small, 2194 regular, 2231 regular+assistant"
  chave: "aluno, without duplicates"
  faltantes_nota: "8,5% (small) vs 8,6% (regular); difference within the school -0,3 p.p., p = 0,80 (non-differential attrition)"
  analisados: "3743 students with the two grades, 79 schools (78 with both arms)"
  balanco_dentro_da_escola: "free lunch -0,04 p.p. (p=0,98); sex F -0,5 p.p. (p=0,74); African-American -0,3 p.p. (p=0,63); teacher experience 9,0 vs 9,1 years"
  comparacao: "the 'regular with assistant' arm was excluded from the contrast (as the question asks)"
```

## 3. DAG

```
escola ──► sorteio_dentro_escola ──► tamanho_turma ──► nota
escola ──────────────────────────────────────────────► nota
aluno (renda, etnia, sexo) ──────────────────────────► nota
professor (sorteado) ────────────────────────────────► nota
nota faltante ◄── (S) — não depende do braço (verificado)
```

## 4. Null model and severe test

- Reproduced: raw difference 13.9; FE school 16.0 (CI 7.4–24.6); 0.22 SD.
- By component: reading +6.6 (CI 3.1–10.1); math +9.4 (CI 3.7–15.0). Both in the same direction.
- With covariates (sex, ethnicity, free lunch): 16.1 (CI 7.8–24.4).
- **Null at the school unit (random sign flip of the 78 differences within the school, 20000 replicates):** p = 0.00045. The mean difference within the school is 15.3 and the median 10.3; 63% of the schools have a positive difference.
- The result does not depend on the number of students (pseudoreplication): the analysis with schools only keeps the effect.
- K = 6 specifications; all in the range 14–16 points with a CI excluding zero.

## 5. Lenses

- **Morphonomy (invariance):** effect present in reading and math and in 63% of the schools. The heterogeneity between schools is large (which is why the CI is wide).
- **Scalonomy:** the ceiling of the scale at 627 may underestimate the effect; it does not change the conclusion.
- **Crisonomy:** the decision is a **pilot** (reversible, cheap relative to a broad adoption). With an effect of ~0.22 SD (lower limit ~0.1 SD), the pilot is justified. The main risk is one of transfer (another country and another time).

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "distribution of the differences by school compatible with a heterogeneous positive mean effect"
  modelo_rival: "null effect with school heterogeneity: rejected by the sign-flip test (p < 0,001)"
  identificado_pelos_dados: ["mean effect in the year of early childhood education, in the context of the experiment"]
  trazido_de_fora: ["transportability to the local chain", "absence of expectancy effect of the teachers"]
  performatividade: "teachers and families knew the condition; possible Hawthorne effect"
  premissas_indecidiveis: "ignorable attrition (supported by the balance); transport"
  tentativas_K: 6
```

## 7. Decision (F8)

In the experiment, out of every 100 schools, about 63 had higher grades in the small classes than in the regular ones, with a mean gain of ~16 points (0.22 SD; CI 7–25). The evidence sustains that small classes raised the grades in that year and justifies a **pilot**, which must measure the effect locally (preferably with a draw between classes/schools).
Exit clause: if the pilot, with a pre-sized sample, does not show an effect ≥ 0.1 SD, do not expand.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: The draw was made within the school, the attrition does not differ between arms and the covariates are balanced. The effect of about 16 points (CI 7,4–24,6) holds at the school unit (sign flip p < 0,001), in reading and in math. To justify a pilot, that is enough.
```
