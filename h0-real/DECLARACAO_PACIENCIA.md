# H0 real, Bloco D: declaração da malha de paciência e tempo de atendimento

Escrita em 2026-10-15, antes de rodar a malha que gera `saida/sensibilidade.json`. O SHA-256 deste arquivo está em
`DECLARACAO_PACIENCIA.sha256`. Mudar critério depois de ver o resultado não é proibido: entra como declaração nova,
com hash próprio, e conta no K. Este arquivo e o seu hash não são reescritos.

**O que já se via antes de escrever.** Li `h0-real/README.md` e `h0-real/resultados.json`: a tabela da decisão (Φ4)
publicada — R0 26% dos dias abaixo da meta, R1 12%, R3 7,4%, R2 3,7%, com pico de 234 atendentes, 2 264 agente-horas
por dia na alegação e R3 = R0 + 12 atendentes (2 433) — e a seção `sensibilidade` já publicada, que varia só um eixo
por vez: atendimento de 180 s e 300 s, atendimento lognormal, e paciência de 60 s (1% dos dias abaixo da meta) e de
600 s (40%). Nada da malha nova (paciência × atendimento em conjunto, regras comparadas ponto a ponto) foi calculado
antes desta declaração. O código lido é `h0-real/estocologia/central.py`, `h0-real/estocologia/h0.py` e
`h0-real/portoes.py`.

**O que a malha troca.** `h0-real/README.md` diz hoje que "a paciência governa o tamanho do efeito: com 1 min, 1%
dos dias ficam abaixo da meta; com 10 min, 40%" — e chama isso de suposto, porque nem o tempo de atendimento nem a
paciência estão na base (`dados-reais/bank_calls.csv` só traz contagens de chamadas). O objetivo é trocar o "suposto"
por um limite e um ponto de virada: sob que paciência (e sob que tempo de atendimento) a *regra vencedora* deixa de
ser a publicada.

## A malha

Tempo médio de atendimento (AHT) em {180, 240, 300, 360} s (3, 4, 5 e 6 min) × paciência média em {60, 120, 180, 300,
420, 600} s (1, 2, 3, 5, 7 e 10 min). São 24 pontos. O ponto (240 s, 180 s) é o ponto base: é o que está publicado no
`h0-real/README.md`. A chegada é sempre a real: os 81 dias do cofre (jul–out 2003) de `estocologia.central.carregar`.

## As regras de escala comparadas

As mesmas quatro do H0 real, recalculadas em cada ponto com o AHT daquele ponto (a fórmula da alegação é função de
AHT; a margem não):

| regra | construção |
|---|---|
| R0 alegação | `escala_do_perfil(perfil médio do treino, aht)` |
| R1 perfil do dia da semana | `escala_do_perfil(perfil do dia da semana, aht)`, custo zero sobre R0 |
| R2 alocar por risco | `escala_do_perfil(perfil do dia da semana, aht, fator = 1 + 1,2816·cv)`, com cv = 0,0598466 o CV residual dos totais diários do treino depois do dia da semana |
| R3 acrescentar por igual | `R0 + extra`, com `extra` o menor inteiro tal que os agente-horas do cofre com `R0 + extra` alcancem os de R2 (mesma regra do H0 real, custo empatado) |

O `extra` só depende do AHT. Já calculado (é parâmetro de desenho, não resultado da malha):
AHT 180 s → 9; 240 s → 12; 300 s → 15; 360 s → 18. Em 240 s: 2 264,25 agente-horas/dia em R0, 2 427,68 em R2 e
2 433,25 em R3, os números publicados. A paciência não entra na construção da escala em nenhuma regra — ela só entra
na simulação.

## As réplicas e os sorteios

Cada ponto roda **4 réplicas** independentes, cada uma simulando os 81 dias do cofre para as quatro regras. Cada
réplica usa **um único fluxo** `np.random.default_rng`, consumido na ordem R0, R1, R2, R3, exatamente como
`h0-real/estocologia/h0.py` faz na seção 7 (regras). Sementes:

- réplica 0 do ponto base (240 s, 180 s): `SeedSequence(2003).spawn(8)[5]`, o mesmo fluxo do H0 real publicado;
- todas as demais: `SeedSequence([2003, i_aht, i_pac, r])`, com os índices do ponto na malha e `r` o índice da
  réplica (0…3).

Assim cada ponto é independente dos outros — mexer num ponto não mexe nos demais — e o ponto base tem uma conferência
exata contra o publicado. A simulação é a do horizonte, sem reescrever nada: `estocologia.central.dia`/`_fcfs` e
`estocologia.h0.rodar_mundo`/`agente_horas`.

## O que se mede

Por ponto, por regra e por réplica: fração dos dias do cofre abaixo da meta (80% em 20 s), nível de serviço agregado
(fração das chamadas atendidas em até 20 s), espera média, desistência, pior dia, agente-horas por dia e chamadas.
O JSON guarda as quatro réplicas e o resumo por regra, mais o ranking completo das regras em cada ponto.

## O critério declarado

- **Resumo por ponto.** A fração de dias abaixo da meta de uma regra no ponto é a média das 4 réplicas; o nível de
  serviço e a espera média também.
- **Escala de ruído do ponto.** `g = max(1/81; maior semi-amplitude entre réplicas no ponto)`, em que a maior
  semi-amplitude é o maior, sobre as quatro regras, de (máximo − mínimo)/2 das frações das 4 réplicas, e 1/81 é o
  passo da unidade (um dia do cofre).
- **Vencedora e empate.** A regra vencedora do ponto é a de menor fração média; **W(p)** é o conjunto das regras cuja
  fração média fica a no máximo `g` da menor — isto é, as que não são batidas estritamente (por mais de `g`) por
  nenhuma outra. Se |W(p)| > 1, o ponto é um **empate** e o vencedor é o conjunto. Desempate declarado para o ranking
  completo: menor fração, depois maior nível de serviço, depois menor espera média.
- **Robustez.** W0 é o conjunto vencedor do **ponto base** (240 s, 180 s). O ponto p é **ponto de virada** se
  W(p) ∩ W0 = ∅, isto é, se toda regra vencedora na base é batida estritamente em p. A decisão é **robusta** se nenhum
  dos 24 pontos é ponto de virada; onde houver ponto de virada, o ponto de virada é o resultado.
- **Pior caso da decisão.** O ponto cuja *melhor* regra tem a maior fração média de dias abaixo da meta. É o pior
  caso da malha, porque é onde nem a melhor das quatro regras disponíveis segura a meta.
- **Sem ajuste depois.** Nada de limiar, ponto, regra ou semente é escolhido depois de ver a malha. Se eu quiser
  mudar alguma coisa, sai declaração nova, com hash novo, e conta no K.

## Conferências internas (declaradas agora, cobradas no relatório)

1. No ponto base, `extra` = 12, e os agente-horas por dia de R0, R2 e R3 são 2 264,25, 2 427,68 e 2 433,25 — os
   publicados.
2. No ponto base, a réplica 0 deve **reproduzir** `h0-real/resultados.json → resultados.regras` (fração de dias
   abaixo da meta e nível de serviço das quatro regras) porque o fluxo e a ordem dos sorteios são os mesmos. A
   diferença máxima é reportada; acima de 1e-9 é divergência, não reprodução.
3. `python3 portoes.py` do H0 real continua saindo com código 0 e `resultados.json` continua com o mesmo SHA-256
   (1462c063…): nada do publicado muda.

## O que a malha não mede (limites conhecidos de antemão)

- **A família das distribuições não varia.** Atendimento e paciência seguem exponenciais com CV 1; a malha só mexe
  nas médias. Paciência lognormal ou com CV diferente é outro eixo (a lognormal do atendimento está na
  `sensibilidade` publicada, e um eixo de cada vez).
- **Independência entre paciência e atendimento.** Cada chamada sorteia as duas em separado; uma correlação entre elas
  (quem espera mais também é atendido mais devagar, ou desiste de forma dependente do serviço) não está na base.
- **Uma paciência só, para o dia inteiro.** Não há paciência por hora do dia, nem por fila, nem fatia de quem
  abandona e volta (rediscagem), que o `h0-real/README.md` já marca como laço performativo aberto.
- **A meta não varia.** 80% em até 20 s em todos os pontos, em todas as regras.
- **Sem custo nem turnos.** As regras são comparadas por dias abaixo da meta e por agente-horas; o custo de recursos
  por hora do dia e a granularidade de turno (meia hora) não entram.
- **4 réplicas.** A resolução da comparação entre regras é de cerca de 1 ponto percentual; a escala de ruído do
  critério é declarada por isso, e não para "provar" empates finos.
- **A base não traz espera nem abandono**, então nada disso é medida de ajuste: a malha é sobre a sensibilidade da
  decisão às duas suposições, não sobre a estimativa delas.
