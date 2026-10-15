# Parecer — caso_01: controle de intensidade por polarizadores (bancada óptica)

Scripts: `bracoA/scripts/caso_01.py`, `bracoA/scripts/caso_01b.py` (semente `np.random.default_rng(20261015)`).

## 1. Cabeçalho das cinco perguntas

```yaml
cabecalho:
  processo_gerador_e_camada:
    mecanismo: "Fonte LED RGB -> polarizador 1 -> polarizador 2 -> sensor (canais IR e visível); LEDs auxiliares perto do sensor somam luz espúria; ruído de contagem do sensor."
    camada_onde_pode_falhar: "L1 (o canal IR poderia não ser polarizado pelos filtros) e L2 (comparação de faixas de |ângulo1-ângulo2| pode estar confundida se a física depender do ângulo assinado ou de um offset de montagem)."
  nao_linearidade_jensen:
    f: "I = I0 * cos^2(theta - offset)"
    comentario: "f é côncava perto de 0°; f(E[theta]) != E[f(theta)], por isso modelamos cos^2 por medição em vez de comparar médias de faixas. Dinâmica aditiva (leituras somam fontes), sem cauda pesada (máx. IR 1175, distribuição limitada)."
  unidade_independente_e_margem:
    unidade: "medição (comandos sorteados independentemente em cada medição)"
    n: 10000
    margem: "EP do coeficiente de Malus = 7,2 contagens sobre um coeficiente de 430 (1/sqrt(n) ~ 1%)"
    pseudorreplicacao: "autocorrelação lag-1 dos resíduos = -0,02; não há dependência serial relevante"
  fisica_ruido_agente_ativo:
    estados_absorventes: "nenhum"
    ruido: "contagem do sensor, aproximadamente gaussiano/aditivo; sem saltos"
    agente_ativo: "o controlador vai atuar (do(angulo)), e os dados já são intervencionais: ângulos foram sorteados pelo computador"
  escada_causal:
    degrau: "intervenção, P(leitura | do(angulos))"
    identificacao: "randomização independente dos comandos; não é preciso ajuste"
    premissa_nao_testavel: "os ângulos comandados correspondem aos ângulos físicos (sem folga mecânica sistemática)"
```

## 2. Ficha da amostra (M4)

```yaml
ficha_amostra:
  populacao_alvo: "estados da bancada com LED fixo e polarizadores na faixa -15° a 25°"
  mecanismo_de_selecao: "nenhum filtro; 10000 medições consecutivas, todas mantidas"
  chave_primaria: "medicao (sem duplicatas)"
  faltantes: 0
  sentinelas: "nenhuma observada (faixas coerentes com o dicionário)"
  balanco_entre_faixas_comparadas: "médias de LED R/G/B e auxiliares quase iguais entre |dif|<=5° e 30–40° (ex.: vermelho 42,1 vs 43,0)"
  escopo: "diferença angular até ~40°; fora disso é extrapolação"
```

## 3. DAG (M13)

```
LED_R, LED_G, LED_B ──► luz na fonte ──► [pol1, pol2] ──► sensor IR / visível
ang_pol1 ──┐                                   ▲
ang_pol2 ──┴──► transmissão cos²(Δ - offset) ──┘
LED_aux_a, LED_aux_b ──────────────────────────► sensor
(todos os nós-raiz sorteados de forma independente: nenhum caminho de porta dos fundos)
```

Não há colisor condicionado nem confundidor: a comparação de faixas é uma comparação entre braços aleatorizados.

## 4. Modelo nulo e teste severo (M5)

- Alegação: diferença IR 455,9 vs 344,2 (Welch t = 13,8; reproduzida exatamente).
- Modelo completo: `leitura_ir ~ LEDs + aux + cos²(Δ)`; coeficiente de Malus = 429 (EP 7,9), R² = 0,90.
- **Nulo simulado por permutação** (500 permutações de cos²(Δ), mantendo o resto): coeficiente nulo entre -17,6 e 18,4; observado 429; p < 1/500.
- **Teste severo adicional (fora da amostra)**: ajuste nas medições 1–5000 e teste nas 5001–10000. RMSE com o termo de Malus 62,9 vs 73,4 sem ele. A previsão ganha nos dados que o modelo não viu.
- Achado secundário: o máximo de transmissão está em ângulo1 - ângulo2 ≈ -4° (offset de montagem; perfil por faixas: máximo em [-10,-5]). Isso não muda a conclusão, mas deve entrar na calibração do controle.
- O canal visível tem o mesmo padrão (coef. 116, EP 2,2): o efeito não é artefato de um canal só.
- Os LEDs auxiliares quase não afetam a leitura (coef. 0,006 e 0,014, não significativos).
- Especificações tentadas: K = 4 (faixas, cos² simples, cos² com interação por cor, cos² com offset).

## 5. Lentes pertinentes

- **Morfonomia (invariância):** o coeficiente é estável entre as metades do experimento e entre os dois canais.
- **Escalonomia:** leituras limitadas, sem cauda pesada; a média é um resumo adequado. Mediana e quantis por faixa seguem a média.
- **Crisonomia:** a decisão é de engenharia de controle; o erro de previsão fora da amostra (RMSE ~63 contagens) dá a margem do controlador.
- Fluxonomia e Crononomia: não se aplicam (não há dinâmica temporal; deriva por bloco de 1000 medições entre 421 e 449, sem tendência).

## 6. Ficha da crítica de modelos (M15)

```yaml
critica_modelos:
  checagem_preditiva: "perfil das médias por faixa de Δ assinado segue cos²(Δ+4°) (máx. ~470, ~330 a ±35–40°)"
  modelo_rival: "sem efeito angular (só LEDs): perde fora da amostra (RMSE 73,4 vs 62,9)"
  parametros_identificados_pelos_dados: ["coeficiente de transmissão", "offset ≈ -4°", "pesos por cor"]
  parametros_trazidos_de_fora: ["forma cos² (lei de Malus)"]
  performatividade: "nenhuma: o controle atua sobre um sistema físico que não reage ao modelo"
  premissas_indecidiveis: "linearidade do sensor fora da faixa testada; com LED fixo diferente do sorteado, a escala muda mas a forma não"
  tentativas_K: 4
```

## 7. Decisão (F8)

Com o LED fixo, girar o polarizador de alinhado para ~35° de desalinhamento reduz o IR em cerca de 25% (de ~455 para ~340 contagens), com incerteza de cerca de 1 ponto percentual no coeficiente. Recomenda-se programar o controle por cos²(Δ - offset), calibrando o offset (~-4°) e limitando o comando à faixa testada (|Δ| ≤ 40°).
Cláusula de saída: se, em operação, a leitura prevista pelo modelo se afastar mais de 3 RMSE (~190 contagens) em 5% das medições, ou se o comando sair da faixa testada, suspender o controle automático e recalibrar.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Os ângulos foram sorteados de forma independente, então a queda da leitura com o desalinhamento é um efeito de intervenção. Ela segue a lei de Malus, resiste ao nulo por permutação e à validação fora da amostra, e aparece nos dois canais do sensor.
```
