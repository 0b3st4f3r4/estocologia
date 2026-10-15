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
