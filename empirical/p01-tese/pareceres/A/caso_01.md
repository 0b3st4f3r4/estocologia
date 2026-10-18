# Opinion — Case 01: intensity control by polarizers (optical bench)

Scripts: `bracoA/scripts/caso_01.py`, `bracoA/scripts/caso_01b.py` (seed `np.random.default_rng(20261015)`).

## 1. Header of the five questions

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "RGB LED source -> polarizer 1 -> polarizer 2 -> sensor (IR and visible channels); auxiliary LEDs near the sensor add spurious light; sensor counting noise."
    camada_onde_pode_falhar: "L1 (the IR channel might not be polarized by the filters) and L2 (comparison of bands of |angle1-angle2| may be confounded if the physics depends on the signed angle or on a mounting offset)."
  nao_linearidade_jensen:
    f: "I = I0 * cos^2(theta - offset)"
    comentario: "f is concave near 0°; f(E[theta]) != E[f(theta)], so we model cos^2 by measurement instead of comparing band means. Additive dynamics (readings add sources), without a heavy tail (max. IR 1175, bounded distribution)."
  unidade_independente_e_margem:
    unidade: "measurement (commands drawn independently in each measurement)"
    n: 10000
    margem: "SE of the Malus coefficient = 7,2 counts over a coefficient of 430 (1/sqrt(n) ~ 1%)"
    pseudorreplicacao: "lag-1 autocorrelation of the residuals = -0,02; there is no relevant serial dependence"
  fisica_ruido_agente_ativo:
    estados_absorventes: "none"
    ruido: "sensor counting, approximately Gaussian/additive; without jumps"
    agente_ativo: "the controller will act (do(angle)), and the data are already interventional: the angles were drawn by the computer"
  escada_causal:
    degrau: "intervention, P(reading | do(angles))"
    identificacao: "independent randomization of the commands; no adjustment is needed"
    premissa_nao_testavel: "the commanded angles correspond to the physical angles (without systematic mechanical play)"
```

## 2. Sample card (M4)

```yaml
ficha_amostra:
  populacao_alvo: "states of the bench with fixed LED and polarizers in the range -15° to 25°"
  mecanismo_de_selecao: "no filter; 10000 consecutive measurements, all kept"
  chave_primaria: "medicao (without duplicates)"
  faltantes: 0
  sentinelas: "none observed (ranges coherent with the dictionary)"
  balanco_entre_faixas_comparadas: "means of LED R/G/B and auxiliary almost equal between |dif|<=5° and 30–40° (e.g. red 42,1 vs 43,0)"
  escopo: "angular difference up to ~40°; beyond that it is extrapolation"
```

## 3. DAG (M13)

```
LED_R, LED_G, LED_B ──► luz na fonte ──► [pol1, pol2] ──► sensor IR / visível
ang_pol1 ──┐                                   ▲
ang_pol2 ──┴──► transmissão cos²(Δ - offset) ──┘
LED_aux_a, LED_aux_b ──────────────────────────► sensor
(all root nodes drawn independently: no backdoor path)
```

There is no conditioned collider and no confounder: the band comparison is a comparison between randomized arms.

## 4. Null model and severe test (M5)

- Claim: IR difference 455.9 vs 344.2 (Welch t = 13.8; reproduced exactly).
- Full model: `leitura_ir ~ LEDs + aux + cos²(Δ)`; Malus coefficient = 429 (SE 7.9), R² = 0.90.
- **Null simulated by permutation** (500 permutations of cos²(Δ), keeping the rest): null coefficient between -17.6 and 18.4; observed 429; p < 1/500.
- **Additional severe test (out of sample)**: fit on measurements 1–5000 and test on 5001–10000. RMSE with the Malus term 62.9 vs 73.4 without it. The prediction wins on data the model did not see.
- Secondary finding: the transmission maximum is at angle1 - angle2 ≈ -4° (mounting offset; profile by bands: maximum in [-10,-5]). That does not change the conclusion, but it must enter the calibration of the control.
- The visible channel has the same pattern (coef. 116, SE 2.2): the effect is not an artifact of a single channel.
- The auxiliary LEDs barely affect the reading (coef. 0.006 and 0.014, not significant).
- Specifications tried: K = 4 (bands, plain cos², cos² with interaction by color, cos² with offset).

## 5. Relevant lenses

- **Morphonomy (invariance):** the coefficient is stable between the halves of the experiment and between the two channels.
- **Scalonomy:** bounded readings, without a heavy tail; the mean is an adequate summary. Median and quantiles by band follow the mean.
- **Crisonomy:** the decision is one of control engineering; the out-of-sample prediction error (RMSE ~63 counts) gives the margin of the controller.
- Fluxonomy and Chrononomy: do not apply (there is no temporal dynamics; drift per block of 1000 measurements between 421 and 449, without a trend).

## 6. Card of the model critique (M15)

```yaml
critica_modelos:
  checagem_preditiva: "profile of the means by band of signed Δ follows cos²(Δ+4°) (max. ~470, ~330 at ±35–40°)"
  modelo_rival: "without angular effect (only LEDs): loses out of sample (RMSE 73,4 vs 62,9)"
  parametros_identificados_pelos_dados: ["transmission coefficient", "offset ≈ -4°", "weights by color"]
  parametros_trazidos_de_fora: ["cos² form (Malus law)"]
  performatividade: "none: the control acts on a physical system that does not react to the model"
  premissas_indecidiveis: "linearity of the sensor outside the tested range; with a fixed LED different from the one drawn, the scale changes but the form does not"
  tentativas_K: 4
```

## 7. Decision (F8)

With the LED fixed, rotating the polarizer from aligned to ~35° of misalignment reduces the IR by about 25% (from ~455 to ~340 counts), with an uncertainty of about 1 percentage point in the coefficient. It is recommended to program the control by cos²(Δ - offset), calibrating the offset (~-4°) and limiting the command to the tested range (|Δ| ≤ 40°).
Exit clause: if, in operation, the reading predicted by the model departs by more than 3 RMSE (~190 counts) in 5% of the measurements, or if the command leaves the tested range, suspend the automatic control and recalibrate.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: The angles were drawn independently, so the fall of the reading with misalignment is an intervention effect. It follows the Malus law, resists the permutation null and the out-of-sample validation, and appears in both channels of the sensor.
```
