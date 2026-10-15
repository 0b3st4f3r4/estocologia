# Critérios do portão de saída do H3 (declarados antes de abrir qualquer cofre)

Declarados em 2026-10-14, antes de qualquer replay de desenvolvimento. O hash SHA-256 deste arquivo entra em
cada pré-registro, então mudar uma linha depois muda o hash que abre o cofre, e a troca fica visível.

PLAN.md, H3: "calibração empírica comprovada fora da amostra (out-of-time), zero vazamento temporal e controle
de performatividade/Goodhart", sob operação contínua com o F3 (deriva KS/Wasserstein), o F5/F6 (privacidade e
justiça) e os disjuntores do F8.

## Fluxos

Seis fluxos reais com carimbo de tempo, cada um com corte do cofre fora do tempo fixado aqui:

| fluxo | dado | previsão | tipo | cofre (t_disp ≥) |
|---|---|---|---|---|
| energia | vic_elec, demanda a cada 30 min, 2012–2014 | demanda das 48 meias-horas do dia seguinte, emitida à meia-noite UTC | quantis 5/50/95% | 2014-01-01 |
| central | bank_calls, chamadas a cada 5 min, 2003 | chamadas dos 169 intervalos do dia útil seguinte, emitida no fim do dia | quantis 5/50/95% | 2003-09-01 |
| bolsa | DJIA diário, 1885–2023 | retorno do pregão seguinte | quantis 1/5/50% (VaR) | 2000-01-01 |
| voos | nycflights13, 2013 | P(chegada com atraso > 15 min ou cancelado), emitida 2 h antes da partida programada | binário | 2013-10-01 |
| ciclones | storms (NOAA HURDAT2), 1975–2022 | P(vento sobe ≥ 30 kt em 24 h), emitida a cada observação | binário | 2015-01-01 |
| prisões | Arrests de Toronto, 1997–2002 | P(liberação) de cada detido, modelo treinado só nos anos anteriores | binário | 2001-01-01 |

O previsor oficial de cada instante é o modelo, ou o reserva determinístico quando o disjuntor do F8 está aberto
ou a entrada sai do escopo. **A previsão oficial é a que se pontua.** Recusar ou abster-se não tira a previsão do
placar: no lugar dela entra a do reserva.

## G1. Simulabilidade (herdado)

O pytest passa, e uma segunda execução limpa reproduz os resultados idênticos.

## G2. Calibração fora do tempo (PLAN)

Vale para todo previsor oficial, no cofre de **todos** os seis fluxos.

- **Binário (voos, ciclones, prisões):**
  - |p̄ − ȳ| ≤ 0,02;
  - ECE (10 caixas de massa igual) ≤ 0,03;
  - habilidade de Brier > 0 contra a climatologia móvel, que é a taxa-base do passado disponível no instante da
    emissão.
- **Quantis (energia, central):**
  - cobertura do intervalo 5–95% entre 0,86 e 0,94;
  - habilidade do escore de intervalo > 0 contra o reserva sazonal ingênuo.
- **Bolsa:**
  - frequência de violação do VaR 99% (retorno abaixo do quantil 1%) entre 0,5% e 1,5%;
  - teste de Kupiec com p > 0,05.

  A independência de Christoffersen é relatada, sem ser exigida.

## G3. Zero vazamento temporal (PLAN)

Todas as condições abaixo têm de valer:

- **(a) Leituras.** Nenhuma leitura devolve evento com t_disp acima do pedido ou do relógio.
- **(b) Previsões.** Toda previsão foi emitida com estado formado só por eventos com t_disp ≤ t_emissão, e o
  evento que a resolve tem t_disp > t_emissão. Zero violações nas duas contagens.
- **(c) Cofre.** Nenhuma leitura do cofre antes do pré-registro, e nenhuma segunda abertura.
- **(d) Banca.** São recusados todos os ataques:
  - ler o futuro;
  - ler o cofre cedo;
  - abrir o cofre com o agente errado;
  - abrir o cofre duas vezes;
  - fazer SELECT *;
  - voltar o relógio.
- **(e) Canário.** Em cada fluxo binário, uma coluna igual ao rótulo é publicada no instante em que o rótulo se
  torna conhecível.
  - Com o relógio honesto, o previsor que a usa não ganha mais que 0,02 de habilidade sobre o previsor sem ela.
  - Com o relógio desligado (controle positivo), ganha mais que 0,2. O controle positivo mostra que o canário
    detectaria um vazamento.

## G4. Performatividade e Goodhart (PLAN)

Nenhum dos fluxos reais registra as ações do próprio sistema, então a performatividade não é mensurável nos
dados. O teste é um laço declarado: as chegadas reais da central são as primeiras tentativas exógenas. A escala
sai da previsão. Quem desiste rediscam com probabilidade r, e as rediscagens entram na contagem observada.

Todas as condições abaixo têm de valer:

- **(a) Detecção.** Com exploração aleatória da escala em 10% dos intervalos, o sensor da lente M11 detecta a
  dependência das chegadas observadas em relação à escala quando r = 0,6 (p < 0,01). Quando r = 0, ele não
  detecta (p > 0,05).
- **(b) Controle.** Com r = 0,6, a previsão corrigida pelo efeito estimado erra a demanda exógena em no máximo
  3% (viés relativo médio no período de avaliação). O viés da previsão ingênua, treinada na contagem observada,
  é relatado.
- **(c) Goodhart: banca contra o placar.** Todos os ataques são neutralizados:
  - O previsor que se abstém quando está incerto recebe exatamente o escore do reserva nesses instantes, e a
    abstenção não o deixa à frente do previsor honesto.
  - O previsor constante na taxa-base recebe peso 0 no diário.
  - A segunda abertura do cofre é recusada.

  O índice de Goodhart (habilidade no desenvolvimento menos habilidade no cofre) é relatado por fluxo.

## G5. Contenção de varredura (herdado do H2)

Valem todas as condições abaixo:

- Nenhum SELECT * executado.
- Toda forma de consulta tem a projeção e o filtro em t_disp confirmados no EXPLAIN.
- Cada tabela é varrida no máximo 3 vezes no replay inteiro (bytes varridos / bytes da tabela ≤ 3).
- O gasto por agente fica dentro do orçamento.

## Informativos

- **G6, F5/F6.**
  - k-anonimato.
  - Orçamento ε das liberações com privacidade diferencial, com disjuntor de privacidade.
  - Bits perdidos.
  - Trilema da justiça por ano no fluxo de prisões.
- **G7, operação.**
  - Alarmes de deriva no desenvolvimento e no cofre.
  - Aberturas do disjuntor.
  - Recusas por escopo.
  - Decisões alteradas pelo limiar C/L.
  - Nós do hipergrafo, com a meta de 10⁶ do PLAN.
- **Trilha.** A cadeia de hashes do barramento de cada fluxo deve ser íntegra. Este item é exigido.

## Regra de saída

Saída do H3 = G1 ∧ G2 ∧ G3 ∧ G4 ∧ G5 ∧ Trilha.

O portão do H2 (ICP com Ŝ ≠ ∅ em 3 áreas) continua em aberto. O H3 não depende dele, e o relatório diz isso.

## Desenvolvimento e K

Antes do corte, cada fluxo pode ser ajustado à vontade. Cada especificação distinta rodada no desenvolvimento
fica registrada (hash do módulo do fluxo) e conta no K do fluxo. Depois do pré-registro, nada do fluxo muda.
