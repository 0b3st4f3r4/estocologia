# H0 em domínio real: a central de chamadas de um banco

O H0 simulado (`../h0-lab0/`) supunha uma demanda bimodal: 5 ou 11 pacientes por hora. Aqui a demanda é
observada. São 2,65 milhões de chamadas reais do cofre (jul–out 2003), de uma série de 164 dias úteis contados a
cada 5 minutos na central de um grande banco norte-americano (`../dados-reais/bank_calls.csv`, FONTES.md).

```
python3 portoes.py      # Φ1–Φ4 + conferência do hash da base; ~5 min; grava resultados.json e figura_dias.png
```

## A alegação

"Escalando cada meia hora pelo Erlang C sobre a taxa média do histórico, a central atende 82% das chamadas em até
20 s, com espera média de 10 s, todo dia." É a prática padrão do setor. A escala sai dos 83 dias de treino
(mar–jun), com pico de 234 atendentes e 2 264 agente-horas por dia, e é aplicada aos 81 dias do cofre.

## O que a base real mostra

| | mundo da alegação | só variação entre dias | chegadas reais |
|---|---|---|---|
| nível de serviço (com desistência) | 97% | 89% | 86% |
| dias abaixo da meta de 80% | **0%** | 17% | **25%** |
| pior dia | 94% | 22% | 20% |
| espera média sem desistência | 10 s | 761 s | 956 s |

- **A demanda não é Poisson.** Entre dias, o desvio é de cerca de 2 900 chamadas; Poisson preveria 180. A
  dispersão Var/média é 273 (p ≈ 0). Parte disso é calendário: segunda-feira tem +15%, e os dias mais cheios
  são segundas, viradas de mês e o dia seguinte ao Labor Day. O resíduo depois do dia da semana tem CV de 6%.
- **Jensen dentro da própria fórmula.** Aplicado à taxa real de cada dia, o Erlang C da alegação dá em média
  57%, não 82%, e 65% dos dias ficam abaixo da meta. Isso dispensa simulação: o nível de serviço é côncavo
  na carga.
- **A variação entre dias explica a maior parte.** O mundo que mantém só o total real de cada dia, com o perfil
  médio, já perde 17% dos dias. O resto vem da forma intradiária.
- **Φ2: a unidade é o dia.** O erro padrão por dia é 119 vezes o erro por chamada. O nulo de metades arbitra
  qual dos dois vale: seu desvio padrão é 0,0474, contra 0,0470 previsto pelo dia e 0,0033 previsto pela
  chamada.
- **Φ3: teste severo.** Em 400 dias simulados no mundo da alegação, nenhum ficou abaixo da meta (q99 da
  reamostragem de 81 dias = 0%). O real teve 25%.
- **Fora do tempo.** O intervalo conforme de 90% para o total do dia, sobre a média do dia da semana, cobriu 86%
  do cofre. A deriva entre treino e cofre é leve (KS 0,19, p = 0,08).

## A decisão (Φ4)

| regra de escala (cofre, chegadas reais) | agente-horas/dia | dias abaixo da meta | nível de serviço |
|---|---|---|---|
| R0 alegação: perfil médio | 2 264 | 26% | 86% |
| R1 perfil do dia da semana (custo zero) | 2 261 | 12% | 90% |
| R3 R0 + 12 atendentes por igual | 2 433 | 7,4% | 94% |
| **R2 dia da semana + margem p90 do CV residual** | 2 428 | **3,7%** | 96% |

R2 e R3 custam o mesmo, mas R2 põe a folga onde está o risco. Em frequências naturais: de cada 100 dias, 26
falham com a escala da alegação, nenhum no mundo que ela descreve e cerca de 4 com R2.

## O limite da paciência e do atendimento (Bloco D)

O tempo de atendimento (4 min) e a paciência (3 min) não estão na base, e a sensibilidade já publicada variava um
eixo por vez. Aqui os dois varrem juntos, e o que se mede não é o tamanho do efeito — é a **decisão**: a malha de
4 × 6 pontos (atendimento de 3, 4, 5 e 6 min × paciência de 1, 2, 3, 5, 7 e 10 min) reescala as quatro regras de Φ4
ponto a ponto e pergunta se a regra vencedora continua sendo o R2. Declaração antes de rodar:
[DECLARACAO_PACIENCIA.md](DECLARACAO_PACIENCIA.md), SHA-256 em `DECLARACAO_PACIENCIA.sha256` (61bb2936…).

**Nenhum ponto da malha troca a regra vencedora.** O R2 (perfil do dia da semana com a margem p90 do CV residual) é
a única regra vencedora em 20 dos 24 pontos. Nos outros quatro, todos com paciência de 1 min, as quatro regras
empatam porque nenhuma deixa mais de 1 dia abaixo da meta (0,0% a 1,2% dos dias) — nos quatro pontos de 1 min o
R2 não deixou nenhum dia abaixo da meta em nenhuma réplica (16 réplicas de 81 dias, 1 296 dias-réplica, zero). Pela
regra declarada (ponto de virada é o ponto em que W(p) ∩ W0 = ∅, com W o conjunto das regras não batidas
estritamente e W0 o vencedor do ponto base, 4 min e 3 min), **a decisão é robusta na malha inteira e não há ponto
de virada**. A margem mais apertada fora do empate é de 2 dias do cofre (2,5 pontos
percentuais) em (6 min, 2 min); a escala de ruído declarada ficou entre 1,2 e 3,1 pontos percentuais.

**Pior caso: (5 min de atendimento, 10 min de paciência).** Nem a melhor das quatro regras segura a meta — 14,2% dos
dias do cofre abaixo dela (11 a 12 dias em cada uma das 4 réplicas de 81 dias), nível de serviço de 91,3%, espera
média de 5,8 s e 1,0% de desistência. A regra da alegação no mesmo ponto: 37,7% dos dias abaixo da meta, nível de
serviço de 71,1%, espera média de 20,1 s e 3,3% de desistência. Em frequências naturais: de cada 100 dias, 38 falham
com a escala da alegação contra 14 com o R2 — e o R2 custa +7,2% de agente-horas em qualquer um dos quatro
atendimentos (1 830 contra 1 707 a 3 min;
3 617 contra 3 373 a 6 min).

| atendimento (min) | paciência (min) | R0 | R1 | R2 | R3 | vencedor do ponto |
|---|---|---|---|---|---|---|
| 3 | 1 | 0.6% | 0.3% | **0.0%** | 0.0% | empate: R2, R3, R1, R0 |
| 3 | 2 | 9.9% | 5.6% | **1.2%** | 3.7% | R2 |
| 3 | 3 | 25.3% | 15.7% | **2.8%** | 7.4% | R2 |
| 3 | 5 | 33.3% | 22.2% | **7.7%** | 15.4% | R2 |
| 3 | 7 | 35.2% | 31.2% | **9.3%** | 20.7% | R2 |
| 3 | 10 | 38.6% | 36.4% | **13.9%** | 26.5% | R2 |
| 4 | 1 | 0.6% | 0.9% | **0.0%** | 0.0% | empate: R2, R3, R0, R1 |
| 4 | 2 | 9.0% | 4.9% | **1.2%** | 3.7% | R2 |
| 4 | 3 | 25.3% | 14.5% | **3.4%** | 7.1% | R2 |
| 4 | 5 | 33.0% | 21.9% | **8.3%** | 14.5% | R2 |
| 4 | 7 | 35.5% | 29.9% | **11.1%** | 23.5% | R2 |
| 4 | 10 | 37.7% | 38.3% | **13.9%** | 26.5% | R2 |
| 5 | 1 | 0.9% | 1.2% | **0.0%** | 0.0% | empate: R2, R3, R0, R1 |
| 5 | 2 | 11.1% | 5.9% | **1.2%** | 4.0% | R2 |
| 5 | 3 | 25.0% | 15.4% | **3.7%** | 7.1% | R2 |
| 5 | 5 | 32.7% | 24.1% | **8.3%** | 16.0% | R2 |
| 5 | 7 | 35.5% | 29.9% | **9.9%** | 23.5% | R2 |
| 5 | 10 | 37.7% | 39.5% | **14.2%** | 26.5% | R2 |
| 6 | 1 | 1.2% | 0.9% | **0.0%** | 0.0% | empate: R2, R3, R1, R0 |
| 6 | 2 | 10.8% | 6.5% | **1.2%** | 4.3% | R2 |
| 6 | 3 | 25.9% | 15.4% | **3.7%** | 7.1% | R2 |
| 6 | 5 | 33.3% | 23.8% | **7.7%** | 15.4% | R2 |
| 6 | 7 | 35.2% | 30.6% | **11.1%** | 20.7% | R2 |
| 6 | 10 | 38.0% | 38.9% | **13.9%** | 26.9% | R2 |

Cada fração é a média das 4 réplicas por ponto; o R3 é o R0 com +9, +12, +15 e +18 atendentes por igual (o menor
aumento que empata o custo com o R2 em cada atendimento, o que dá 12 a 4 min, como em Φ4).

**A paciência governa o tamanho do efeito; o atendimento quase não.** Fixando a paciência, a fração de dias abaixo da
meta do R2 anda no máximo 0,9 ponto percentual em todo o intervalo de atendimento (e 0,3 ponto com 10 min de
paciência); fixando o atendimento, ela vai de 0,0% com 1 min a 13,9%–14,2% com 10 min. O limite que a malha dá à
decisão é esse: em nenhum ponto a regra do perfil do dia da semana com margem passa de 14,2% dos dias abaixo da
meta — contra 0,6% a 38,6% da regra da alegação e 0,3% a 39,5% do perfil do dia da semana sem margem. As duas frases
da seção de pendências continuam de pé: os "1%" e os "40%" que ela cita são a linha R0 desta malha (0,6% a 1,2% com
1 min e 37,7% a 38,6% com 10 min). Com paciência de 10 min o perfil do dia da semana sem margem alcança a escala da
alegação (38,3% contra 37,7% a 4 min), mas a diferença fica dentro da escala de ruído declarada: são
indistinguíveis, não uma troca de posição.

### O que foi feito

`sensibilidade_paciencia.py` reaproveita a simulação do horizonte — `estocologia/central.py` (a fila FCFS com
desistência) e `rodar_mundo`/`agente_horas` do `estocologia/h0.py`; a esteira `portoes.py` fica intocada e é ela
que confere no fim —, sem reescrever nada: o que o script acrescenta é a malha, o critério e as conferências. As
regras de escala são construídas em cada ponto com o atendimento daquele ponto (a margem p90 do R2 não depende
dele); a chegada é sempre a real, os 81 dias
do cofre. Saída completa em [`saida/sensibilidade.json`](saida/sensibilidade.json), com um arquivo por ponto em
`saida/pontos/`. Sementes: a réplica 0 do ponto base usa o mesmo fluxo do H0 real publicado, e as demais usam
`SeedSequence([2003, i_aht, i_pac, r])`. Rodou em 2026-10-15, `OMP_NUM_THREADS=1`, 2 processos, 1 196 s de parede
(máquina dividida com outros trabalhos).

**Duas conferências internas, declaradas antes.** (i) No ponto base o R3 dá +12 atendentes e 2 264,25, 2 427,68 e
2 433,25 agente-horas por dia em R0, R2 e R3 — os números publicados em Φ4. (ii) Com o mesmo fluxo e a mesma ordem
de sorteios, a réplica 0 do ponto base reproduz `resultados.json → resultados.regras` **exatamente**: diferença
máxima 0,0 nas três grandezas conferidas (fração de dias abaixo da meta, nível de serviço e espera média) das quatro
regras, contra a tolerância declarada de 1e-9.

**Um desvio de execução, relatado.** A primeira execução rodou os 24 pontos e caiu na agregação, antes de gravar
qualquer número: `KeyError: 'R3_perfil_medio_mais_9_atendentes'`, porque o rótulo do R3 no `h0.py` carrega o
número de atendentes, que muda com o atendimento, e a comparação entre pontos precisa de uma chave estável. Corrigi
só o rótulo (`R3_perfil_medio_mais_extra_atendentes`, com o rótulo antigo guardado em `desenho`), o que não muda
malha, regra, semente, simulação nem critério, e re-rodei. Nada da malha tinha sido lido antes da correção (o log da
primeira execução só tem os tempos por ponto), e a simulação é determinística com as sementes declaradas. O script
passou a gravar um arquivo por ponto para que uma falha depois da malha não custe outra hora de máquina.
`saida/sensibilidade.json` é a malha inteira; o ponto base rodado de novo, em outro diretório, reproduz o arquivo
guardado identicamente (mesmo JSON, mesmo SHA-256).

**A esteira continua passando.** `python3 portoes.py` sai com código 0 (Φ1–Φ4 e a conferência do hash da base, com
os mesmos números publicados). Ela reescreve `resultados.json` e `figura_dias.png`, e nesta execução os dois saíram
com bytes diferentes do commit sem que nada do conteúdo mude: no JSON o único valor diferente é o tempo do pytest
dentro do texto do Φ1 (4,98 s no commit contra 2,25 s agora; o campo `hash` do resultado é o mesmo) e a figura
diferiu só na codificação PNG (comparação pixel a pixel: mesma imagem, diferença máxima de um nível em 12% dos
pixels). Restaurei os dois arquivos para os bytes do commit — `resultados.json` 1462c063… e `figura_dias.png`
815b723e… —, para que nada do publicado fique alterado.

### O que continua sem medida na base

- **Nada disso é medido — tudo é sensibilidade.** A base só traz contagens de chamadas: não tem espera, desistência,
  tempo de atendimento nem paciência. A malha mostra que a decisão não depende de qual dessas suposições se escolhe
  dentro da faixa declarada; ela não estima nenhuma delas.
- **A família das distribuições.** Atendimento e paciência ficam exponenciais com CV 1; a malha mexe só nas médias
  (a lognormal do atendimento está na sensibilidade publicada, um eixo por vez).
- **Paciência e atendimento independentes.** Cada chamada sorteia os dois em separado; correlação entre eles (quem
  espera mais ser atendido mais devagar) não está na base.
- **Uma paciência só, para o dia inteiro.** Não há paciência por hora do dia nem por fila, e não há rediscagem — o
  laço performativo (M11) segue aberto.
- **Nada além de 10 min de paciência.** O pior caso da malha é o canto dela; paciência maior não foi varrida, e a
  extrapolação não é medida.
- **A meta e o custo fixos.** 80% em 20 s em todas as regras e pontos; as regras são comparadas por dias abaixo da
  meta e por agente-horas, sem custo de turno por hora do dia.
- **4 réplicas.** A resolução entre regras é de cerca de 1 ponto percentual, e é por isso que o critério declara uma
  escala de ruído em vez de tratar empates finos como troca de vencedor.

### Como reproduzir

```
cd h0-real
OMP_NUM_THREADS=1 python3 sensibilidade_paciencia.py --processos 2      # ~20 min; usa saida/pontos/ como cache
sha256sum -c DECLARACAO_PACIENCIA.sha256
```

## O que continua suposto (pendências)

- **Tempo de atendimento e paciência não estão na base.** O atendimento foi suposto exponencial com média de
  4 min, e a paciência exponencial com média de 3 min. A sensibilidade (`resultados.json → sensibilidade`)
  mostra que atendimento de 3 ou 5 min, ou lognormal, mal mexe no resultado (23–25% dos dias abaixo da meta).
  A paciência governa o tamanho do efeito: com 1 min, 1% dos dias ficam abaixo da meta; com 10 min, 40%. O
  sentido do efeito nunca muda.
- **O domínio de saúde do Lab 0 ficou de fora.** Os portais do DATASUS e de dados.gov.br estão bloqueados na
  rede deste ambiente. Com chegadas reais de um pronto-atendimento no Drive, o mesmo código roda trocando
  `carregar`.
- **Rediscagens estão nas contagens sem identificação.** É o laço performativo (M11) ainda aberto.
