# Opinion — Case 12: screening of antibacterial compounds

Script: `bracoA/scripts/caso_12.py` (seed 20261015).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "OD600 per well = plate level + compound effect (probably 0 for most) + well noise (SD ~0,13–0,15)"
    camada_onde_pode_falhar: "L2/L3: 40 independent tests at α = 0,05 without correction (multiple comparisons); ~2 false positives expected under the global null"
  nao_linearidade_jensen: "OD approximately linear in growth in the observed range (0,55–1,43); relative effect = ratio of means"
  unidade_independente_e_margem:
    unidade: "well within plate (8 vs 8 per compound, drawn within the plate)"
    margem: "individual effects with SE ~6–7% of the control; the 3 'significant' ones have p between 0,030 and 0,037"
  fisica_ruido_agente_ativo: "without absorbing states; variation between plates in the controls with SD 0,047"
  escada_causal:
    degrau: "intervention (compound drawn into the wells)"
    identificacao: "valid per compound; the problem is the simultaneous inference"
    premissa_nao_testavel: "independence between wells (without an edge/position effect)"
```

## 2. Sample card

```yaml
ficha_amostra:
  unidades: "640 wells, 40 plates × 16"
  chave: "(placa, condicao, poco), without duplicates; 0 missing; no well discarded"
  hipotese_previa: "none (exploratory screening of 40 compounds)"
  escopo: "one plate per compound, without independent biological replication"
```

## 3. DAG

```
sorteio_poço ──► composto ──► crescimento ──► DO600
placa (inóculo, posição) ─────► crescimento
analista ──► seleção dos "vencedores" por p<0,05 ◄── ruído
```

## 4. Null model and severe test

- Reproduced: K40 −18.2% (p 0.030), K12 −11.3% (p 0.035), K19 −11.6% (p 0.037).
- Expected under the global null: 40 × 0.05 = 2 "significant"; observed 3.
- **Simulated null** (permutation of the 16 wells within each plate, 2000 replicates): mean number of significant reductions = 1.0; P(≥ 3) = 0.085. P(smallest p ≤ 0.030) = **0.71**.
- Corrections: Bonferroni and Holm leave no compound; Benjamini-Hochberg gives q = 0.49 for the three (about half of the "discoveries" would be false).
- The distribution of the 40 p-values does not differ from the uniform (KS p = 0.11). The estimated effects are distributed around 0 (mean −0.9%, SD 7.0%), symmetric: the largest increase is +15% (K11, p = 0.07), of the same magnitude as the reductions.
- K = 40 primary tests + 3 corrections.

## 5. Lenses

- **Crisonomy:** each compound costs R$ 400 thousand in the next phase. With a high probability of being false positives, taking the three costs R$ 1.2 million of expected value largely wasted. A replication (a few plates per compound, low cost) has a very high EVSI.
- **Scalonomy:** the "winner" effects are inflated (winner's curse); in a replication, expect smaller effects.
- **Morphonomy:** without replication, the invariance between plates was not tested.

## 6. Card of the model critique

```yaml
critica_modelos:
  checagem_preditiva: "the global null reproduces the number of 'significant' ones, the magnitude of the largest effect and the increase/reduction symmetry"
  modelo_rival: "no active compound: compatible with everything that was observed"
  identificado_pelos_dados: ["noise per well and per plate"]
  nao_identificado: ["activity of any individual compound"]
  performatividade: "does not apply"
  premissas_indecidiveis: "small real effect in some compound; requires replication"
  tentativas_K: 40
```

## 7. Decision (F8)

Testing 40 compounds with no activity, about 2 are expected to "come out significant" by chance; here 3 did, and none survives the correction for multiple comparisons. Do not take the three directly to the R$ 400 thousand phase. First, replicate K40, K12 and K19 (and perhaps K31) in new and independent plates (e.g. 3–4 plates each, with the success criterion pre-registered).
Exit clause: advance only compounds whose reduction repeats in the replication with p < 0.05 (pre-registered one-sided) and effect ≥ 10%.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: There were 40 tests without a previous hypothesis: ~2 false positives were expected and there were 3, with p between 0,030 and 0,037. None survives Bonferroni, Holm or BH (q ≈ 0,49), and the simulated null reproduces the pattern. Replication is missing before spending R$ 400 thousand per compound.
```
