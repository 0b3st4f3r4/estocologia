# H2e — dados reais com intervenção em variável CONTÍNUA (para ICP)

Recuperado em 2026-10-15. Nenhum modelo de Y foi ajustado; apenas o esquema foi conferido.

---

## 1. Causal Chambers — light tunnel, `lt_interventions_standard_v1` (física / engenharia)

- **Arquivo:** `causal_chamber_lt_interventions_standard_v1.csv` (68 000 × 47). São os 59 CSVs do zip concatenados, sem nenhuma outra alteração; foi adicionada apenas a coluna `experiment` (o nome do arquivo de origem = ambiente).
  - `causal_chamber_lt_variables.csv`: dicionário de variáveis do repositório.
  - `causal_chamber_lt_ground_truth_edges.csv`: as 57 arestas (parent, child) do grafo causal de referência, extraídas de `causalchamber.ground_truth` (`_edges_lt_standard`, pacote PyPI `causalchamber` 0.2.8).
- **URLs:**
  - Dados: https://causalchamber.s3.eu-central-1.amazonaws.com/downloadables/lt_interventions_standard_v1.zip (MD5 476664d024f88e8b7640998bb5e9ee33, conferido)
  - Documentação: https://github.com/juangamella/causal-chamber/tree/main/datasets/lt_interventions_standard_v1
  - Grafo: pacote `causalchamber` 0.2.8 (PyPI), `causalchamber/ground_truth/main.py`
- **Citação:** Gamella, Peters & Bühlmann (2025), "Causal chambers as a real-world physical testbed for AI methodology", Nature Machine Intelligence, doi:10.1038/s42256-024-00964-x (arXiv 2404.11341).
- **Licença:** os CSVs são CC BY 4.0; o código é MIT.
- **Sistema:** um túnel com fonte de luz LED RGB, dois polarizadores motorizados e três sensores de luz (infravermelho `ir_j` e visível `vis_j`, j=1,2,3), além de LEDs auxiliares `l_j1`, `l_j2` junto a cada sensor. O sensor 3 fica depois dos dois polarizadores (lei de Malus). `current` é o consumo elétrico da fonte. `angle_1` e `angle_2` são as leituras dos encoders dos polarizadores.
- **Ambientes (`experiment`, 59 níveis):**
  - `uniform_reference` (n=10 000) é o regime de referência. Nele os atuadores são sorteados de forma independente: R,G,B ~ U{0..85}; L_ij ~ U{0..170}; pol_1, pol_2 ~ U{-15..25} graus. Os parâmetros dos sensores ficam fixos (osr=1, v=5, diode_ir=2, diode_vis=1, t=3).
  - Os outros 58 experimentos (n=1000 cada) aplicam uma intervenção de alvo único que desloca a distribuição de uma variável:
    - **Atuadores contínuos (deslocamento da faixa de dose):**
      - `red|green|blue_mid`: U{86..170}
      - `red|green|blue_strong`: U{171..255}
      - `pol_1|pol_2_mid`: U{26..65}
      - `pol_1|pol_2_strong`: U{66..105}
      - `l_11..l_32_mid`: U{171..255}
    - **Parâmetros discretos de sensor:** `osr_c`, `osr_angle_j`, `v_c`, `v_angle_j`, `diode_ir_j`, `diode_vis_j`, `t_ir_j` e `t_vis_j`, em níveis weak/mid/strong. Valem como intervenções do tipo "do" sobre o mecanismo de medição.
  - Observação: o README cita `uniform_osr_1/2_*`, mas os arquivos se chamam `uniform_osr_angle_1/2_*`.
- **Colunas:** todas float64, exceto `experiment` e `config` (string). Não há valores ausentes. `config` é sempre "standard", `intervention` é sempre 1 e `camera` é sempre 0. `timestamp`, `counter`, `flag`, `v_board` e `v_reg` são colunas de controle de qualidade, não variáveis causais.
- **Grafo de referência (documentado pelos autores):** todas as arestas vão de atuadores ou parâmetros para sensores. Nenhum sensor causa outro sensor.
  - `ir_3` ← red, green, blue, pol_1, pol_2, l_31, l_32, t_ir_3, diode_ir_3
  - `vis_3` ← red, green, blue, pol_1, pol_2, l_31, l_32, t_vis_3, diode_vis_3
  - `ir_1` ← red, green, blue, l_11, l_12, t_ir_1, diode_ir_1 (o mesmo padrão vale para `vis_1`, `ir_2` e `vis_2`, com os L, t e diode correspondentes)
  - `current` ← red, green, blue, osr_c, v_c
  - `angle_j` ← pol_j, osr_angle_j, v_angle_j
- **Candidatos naturais para o ICP:** Y = `ir_3` (ou `vis_3`). Preditores contínuos: red, green, blue, pol_1, pol_2, l_31, l_32 (pais verdadeiros) e ir_1, vis_1, ir_2, vis_2, current, angle_1, angle_2 (não-pais, correlacionados com Y por causas comuns). Ambientes: reference contra os regimes de intervenção. Uma ressalva: na referência os atuadores são independentes entre si, então os não-pais só se correlacionam com Y por meio dos pais comuns (R,G,B,pol).

## 2. Las Rosas corn — `agridat::lasrosas.corn` (agricultura)

- **Arquivo:** `agridat_lasrosas_corn.csv` (3443 × 10). É a cópia exata do CSV do Rdatasets; `rownames` é o índice do R.
- **URL:** https://raw.githubusercontent.com/vincentarelbundock/Rdatasets/master/csv/agridat/lasrosas.corn.csv (o espelho GitHub de vincentarelbundock.github.io/Rdatasets, que o proxy bloqueia). Documentação: .../doc/agridat/lasrosas.corn.html
- **Licença:** o pacote agridat 1.26 declara "MIT + file LICENSE". Os dados foram "used with permission of the ASU GeoDa Center" (fonte original: https://geodacenter.asu.edu/sdata).
- **Descrição:** dados de monitor de colheita de milho na fazenda Las Rosas (Río Cuarto, Córdoba, Argentina), safras de 1999 e 2001. Os tratamentos de nitrogênio foram aplicados em faixas: 6 doses × 3 repetições. Segundo a documentação, a dose foi aleatorizada dentro de uma repetição, e a mesma aleatorização foi usada nas outras duas.
- **Colunas:**
  - `year` (int: 1999, 2001)
  - `lat`, `long` (float)
  - `yield` (float, quintais/ha)
  - `nitro` (float, kg N/ha; 11 valores distintos)
  - `topo` (str: E/HT/LO/W, a posição topográfica)
  - `bv` (float, "brightness value", proxy de baixo teor de matéria orgânica)
  - `rep` (str R1–R3)
  - `nf` (str N0–N5, um fator de dose comum aos dois anos)
  - Não há valores ausentes.
- **Variável intervinda:** `nitro`, uma dose contínua e aleatorizada.
  - 1999: 0, 29, 53, 66, 106 e 131.5 kg/ha, com 286–291 obs por dose.
  - 2001: 0, 39, 50.6, 75.4, 99.8 e 124.6 kg/ha, com 282–286 obs por dose.
- **Ambientes:**
  - `year`: 1999 tem n=1738 e 2001 tem n=1705.
  - Alternativa: `topo` (contagens 1999/2001: E 368/362, HT 354/431, LO 461/424, W 555/488).
  - Alternativa: `year`×`topo`.
- **Y candidato:** `yield`. Preditores: `nitro`, `bv`, `lat`, `long` (e `topo` como dummies).
- **Ground truth:** não há grafo documentado. Pelo desenho do experimento, `nitro` é causa (foi aleatorizado). `bv` e `topo` são características do solo e do relevo anteriores ao tratamento, portanto plausivelmente causas e não efeitos. lat/long são proxies espaciais. A literatura citada (Anselin et al. 2004) reporta resposta ao N que varia com a posição topográfica (interação N×topo), o que pode violar a invariância de um modelo linear aditivo.

## 3. Cigarros e impostos (economia / política fiscal)

### 3a. `AER::CigarettesSW` (Stock & Watson)
- **Arquivo:** `AER_CigarettesSW.csv` (96 × 10), cópia exata do Rdatasets.
- **URL:** https://raw.githubusercontent.com/vincentarelbundock/Rdatasets/master/csv/AER/CigarettesSW.csv
- **Licença:** pacote AER, GPL-2 | GPL-3. Fonte original: material online de Stock & Watson (2007), *Introduction to Econometrics*.
- **Descrição:** 48 estados contíguos dos EUA × 2 anos (1985, 1995).
- **Colunas:**
  - `state` (str)
  - `year` (int)
  - `cpi`, `packs` (maços per capita), `tax` (imposto específico médio estadual+federal+local, centavos), `price` (preço médio com imposto sobre vendas), `taxs` (impostos incluindo o imposto sobre vendas): float
  - `population`, `income` (renda pessoal total, nominal): int
  - Não há valores ausentes.
- **Variável intervinda:** `tax` (alíquota, contínua, definida por política estadual e federal; o imposto federal subiu em 1991 e 1993). Média de `tax`: 31.7 em 1985 e 53.7 em 1995. O canal esperado é tax → price → packs.
- **Ambientes:** `year` (48 obs em 1985 e 48 em 1995).
- **Y candidato:** `packs` (log, no uso padrão). Preditores: `price`, `tax`, `taxs`, `income`, `population`, `cpi`; costumam ser deflacionados pelo `cpi`.
- **Ground truth:** não há grafo. No livro, `tax`/`taxs` servem de instrumentos para `price`, o que corresponde à hipótese de que o imposto não é pai direto de `packs`. Em Stock & Watson, os pais estruturais de `packs` são `price` e `income`. Isso é hipótese econômica, não verdade de laboratório. Atenção: n=96 é muito pouco.

### 3b. `Ecdat::Cigar` (Baltagi)
- **Arquivo:** `Ecdat_Cigar.csv` (1380 × 10), cópia exata do Rdatasets.
- **URL:** https://raw.githubusercontent.com/vincentarelbundock/Rdatasets/master/csv/Ecdat/Cigar.csv
- **Licença:** Ecdat, GPL (>= 2). Fonte: Baltagi & Levin (1992); Baltagi, Griffin & Xiong (2000).
- **Descrição:** painel de 46 estados × 30 anos (1963–1992).
- **Colunas:**
  - `state` (int, códigos 1–51)
  - `year` (int, 63–92)
  - `price` (preço por maço), `pop`, `pop16`, `cpi`, `ndi` (renda disponível per capita), `sales` (maços per capita), `pimin` (preço mínimo nos estados vizinhos): float
  - Não há valores ausentes.
- **Ambientes:** `year` (30 níveis, 46 obs cada) ou períodos agrupados; também é possível usar estado.
- **Variável intervinda:** não há coluna de imposto. As mudanças de imposto entram apenas através de `price`. Por isso a intervenção é implícita, e esta base é mais fraca que a 3a como teste de ICP.
- **Y candidato:** `sales`. Preditores: price, pimin, ndi, cpi, pop, pop16.
- **Ground truth:** não há.

---

## Candidatos descartados
- `agridat::caribbean.maize`: multiambiente (ilha/sítio), mas N/P/K só aparecem como níveis ordinais 0–3; as doses reais não constam.
- Wind tunnel do Causal Chambers (`wt_*`): os experimentos são de calibração, passos e change-points, sem um desenho referência + intervenções como o do light tunnel.
- vincentarelbundock.github.io está bloqueado pelo proxy (403); usou-se o espelho raw.githubusercontent.com.

---

## 4. Causal Chambers — wind tunnel, `wt_walks_v1` / `regime_jumps_single` (engenharia / dinâmica de fluidos)

Adicionado em 2026-10-15 (busca complementar). Só o esquema e os valores dos parâmetros intervindos foram conferidos; nenhum resumo de Y por ambiente foi calculado.

**Correção da nota em "Candidatos descartados" acima:** a versão v1.1 do `wt_walks_v1` (19.06.2024) traz `regime_jumps_single` e `regime_jumps_multi`. São desenhos com regime de referência + intervenções de alvo conhecido, e servem para o ICP.

- **Arquivos:**
  - `causal_chamber_wt_walks_v1_regime_jumps_single.csv` (320 000 × 37): cópia exata do CSV do zip, sem alteração.
  - `causal_chamber_wt_regime_jumps_single_targets.csv` (`flag`, `targets`): alvo de intervenção de cada regime, transcrito de `regimes_targets.py` do repositório. Confere com os valores dos parâmetros observados no CSV.
  - `causal_chamber_wt_ground_truth_edges.csv`: as 42 arestas (parent, child) de `_edges_wt_standard` (pacote PyPI `causalchamber` 0.2.8, `ground_truth/main.py`).
  - `causal_chamber_wt_variables.csv`: dicionário de variáveis do repositório.
- **URLs:**
  - Dados: https://causalchamber.s3.eu-central-1.amazonaws.com/downloadables/wt_walks_v1.zip (MD5 19bb4e92cbe0b8dff49299b9b509ac36, conferido)
  - Documentação: https://github.com/juangamella/causal-chamber/tree/main/datasets/wt_walks_v1 (protocolo: `generators/regime_jumps.py`; alvos: `regimes_targets.py`)
- **Citação:** a mesma da seção 1 (Gamella, Peters & Bühlmann 2025, Nat. Mach. Intell.). As variáveis estão descritas nos apêndices II e III do arXiv 2404.11341.
- **Licença:** os CSVs são CC BY 4.0; o código é MIT.
- **Sistema:** um túnel de vento com dois ventiladores, de entrada e de saída. Os ventiladores são comandados por `load_in` e `load_out` (carga 0–1). Há também:
  - uma escotilha `hatch` (ângulo);
  - quatro barômetros: `pressure_upwind`, `pressure_downwind`, `pressure_ambient`, `pressure_intake` (pressão absoluta, Pa);
  - tacômetros `rpm_in` e `rpm_out`;
  - sensores de corrente `current_in` e `current_out`;
  - um microfone `mic`;
  - potenciômetros `pot_1` e `pot_2` com as leituras `signal_1` e `signal_2`.
  - `osr_*` (oversampling), `v_*` (tensão de referência) e `res_in`/`res_out` (resolução do tacômetro) são parâmetros dos sensores.
- **Desenho:** 32 regimes (`flag` 0–31) de 10 000 medições cada.
  - Em todos os regimes, `load_in`, `load_out`, `pot_1` e `pot_2` seguem passeios aleatórios espelhados independentes. Os passeios são contínuos ao longo do experimento inteiro, com passo de 5 vezes a resolução.
  - Em cada regime, com probabilidade 0,7, há uma intervenção de alvo único (do) sobre um parâmetro. Com probabilidade 0,3 não há intervenção. Ao fim do regime, o alvo volta ao valor-base.
  - O regime 0 é a referência, sem intervenção.
- **Ambientes (`flag`, n=10 000 cada) e alvos:**
  - nenhum: 0, 2, 8, 20, 24, 26 (6 regimes)
  - `hatch` 0→45: 5, 18
  - `osr_upwind` 8→1: 7, 9, 13, 19, 27
  - `osr_downwind` 8→1: 15
  - `osr_ambient` 8→1: 16, 29
  - `osr_intake` 8→1: 23, 28
  - `res_in` 1→0: 12, 21, 31
  - `res_out` 1→0: 3, 14
  - `v_in` 1.1→5: 17, 30
  - `v_out` 1.1→5: 4, 10
  - `v_1` 5→2.56: 6, 11, 22, 25
  - `v_2` 5→2.56: 1
- **Colunas:** float64, exceto `config` (sempre "standard"). Não há valores ausentes. `timestamp`, `counter`, `flag` e `intervention` são de controle. `osr_in`, `osr_out`, `osr_mic`, `v_mic`, `osr_1` e `osr_2` ficam constantes.
- **Grafo de referência (documentado pelos autores, arestas de atuadores/parâmetros → sensores; nenhum sensor causa outro):**
  - `pressure_downwind` ← load_in, load_out, hatch, osr_downwind (o mesmo vale para `pressure_upwind` com osr_upwind, e para `pressure_intake` com osr_intake)
  - `pressure_ambient` ← osr_ambient
  - `rpm_in` ← load_in, load_out, hatch, res_in; `rpm_out` ← load_in, load_out, hatch, res_out
  - `current_in` ← load_in, load_out, osr_in, v_in; `current_out` ← load_in, load_out, osr_out, v_out
  - `mic` ← pot_1, load_in, load_out, hatch, osr_mic, v_mic; `signal_1` ← pot_1, osr_1, v_1; `signal_2` ← pot_1, pot_2, osr_2, v_2
- **Candidatos naturais para o ICP:**
  - **Y = `pressure_downwind`.**
    - Pais verdadeiros (S*): load_in, load_out, hatch, osr_downwind.
    - Não-pais contínuos, que são proxies/irmãos do estado dos ventiladores: rpm_in, rpm_out, current_in, current_out, pressure_upwind, pressure_intake e, opcionalmente, pressure_ambient.
    - Intervenções sobre não-pais: res_in/res_out (mudam o mecanismo de rpm), v_in/v_out (corrente), osr_upwind/osr_intake/osr_ambient (outros barômetros), v_1/v_2 (irrelevantes para Y).
    - Intervenções que agem diretamente no mecanismo de Y: `hatch` e `osr_downwind`. Ambas são **medidas** como colunas e estão em S*.
  - **Alternativa: Y = `rpm_in`**, com S* = {load_in, load_out, hatch, res_in}. res_in age direto em Y e é medido. Não-pais: current_in, current_out, pressures, rpm_out.
- **Ressalvas:**
  - (i) Os dados são séries temporais: as cargas fazem passeio aleatório, há forte autocorrelação, e os regimes são consecutivos no tempo. Isso fere a hipótese i.i.d. do teste do ICP.
  - (ii) Os barômetros medem pressão absoluta. Uma deriva atmosférica lenta, comum a todos eles, é fisicamente plausível e não aparece como aresta no grafo de referência. `pressure_ambient` é a coluna que a mede.
  - (iii) `hatch` e os parâmetros são constantes dentro de cada regime. Como candidatos, funcionam como indicadores de ambiente.
  - (iv) O `regime_jumps_multi.csv` (96 MB, 32 regimes com até 5 alvos cada; alvos em `regimes_targets.py`) está no mesmo zip e não foi copiado.
