# Ponto 8 (calibração): declaração antes de rodar

Escrita em 2026-10-15, antes de qualquer análise que gere os resultados abaixo. O SHA-256 deste arquivo está em
`DECLARACAO.sha256`. Mudanças depois disso entram como declaração nova e contam no K.

**O que já se via antes de escrever.** Os resultados do H3 publicados em `h3/README.md` e em `h3/saida/*.json`
(CITL e ECE de cada cofre, alarmes do sensor de deriva). Dos dados novos de voos (NY 2017, Portland e Seattle
2014, Houston 2011) vi só o esquema: colunas, tipos, número de linhas, meses e aeroportos presentes. Não calculei
nenhuma taxa de atraso, nenhuma média e nada por período. O desvio de 2013 que serve de referência é o publicado:
ȳ − p̄ = +0,026691 no cofre out–dez de 2013 (`h3/saida/portoes.json`, chave `G2 ... voos.citl` = −0,026690800).

Todo o código roda em cópias, dentro de `fundo/p08-calibracao/`. Os diários dos cofres são os de
`/mnt/project-files/estocologia/h3/saida/<fluxo>/diario.parquet`, conferidos contra `FORA_DO_GIT.sha256`.
Sementes fixas com `np.random.default_rng`.

## (a) Tolerância de ECE que depende de n

**Hipótese.** A tolerância fixa de 0,03 não leva em conta o ruído do ECE, que depende de n e da distribuição das
previsões. Em cofres pequenos o ruído pode se aproximar de 0,03; em cofres grandes ele é muito menor, e 0,03 deixa
passar descalibração real.

**Método.** Para cada cofre binário do H3 (voos 2013, ciclones, prisões de Toronto 1997–2002, prisões de Toronto
2020–2021, voos 2023, Rhode Island), tomo as previsões oficiais p̂ do cofre (`calibracao.oficial_binario`, linhas
com t_emissao ≥ corte) e simulo B = 2000 vezes y* ~ Bernoulli(p̂), independentes, semente 20261015. Em cada
simulação calculo o ECE com a mesma função do H3 (`calibracao.ece`, 10 caixas de massa igual). Isso dá a
distribuição do ECE sob calibração perfeita com as mesmas previsões e o mesmo n.

**Critério declarado.**
- **Regra principal (T95):** o cofre passa no ECE se ECE_obs ≤ q95 da distribuição nula. É um teste de calibração
  perfeita com nível 5%.
- **Regra secundária (T95+):** passa se ECE_obs ≤ max(0,03; q95). Mantém a tolerância prática e só a alarga quando
  o ruído passa dela.
- Relato também q50, q99, o valor-p (fração das simulações com ECE* ≥ ECE_obs) e o ECE menos a média nula.
- As outras condições do G2 (|p̄ − ȳ| ≤ 0,02, habilidade > 0, e no Rhode Island o critério relativo) não mudam e são
  relatadas como estão.

**O que conta como falha do ponto.** Se algum cofre aprovado no H3 (ciclones, voos 2023, Rhode Island) tiver
ECE_obs > q95, a aprovação dele não se sustenta pela regra T95, e eu digo isso. Se o cofre de Toronto 1997–2002 ficar
abaixo do q95, a reprovação dele era ruído pela regra T95.

**Limite declarado.** A simulação supõe rótulos independentes dada a previsão. Nos voos há choques comuns por dia e
aeroporto (efeito de desenho de 11,5 no H2), e então a nula independente subestima o ruído real.

## (b) Teste de deriva de rótulo

**Hipótese.** O sensor original (KS e Wasserstein com histerese sobre uma entrada e sobre o resíduo padronizado)
compara distribuições com uma referência; não testa se a taxa realizada se afasta da média prevista. Um teste
direto de deriva de rótulo veria o desvio de 2013 mais cedo ou com mais clareza.

**Dois testes, rodados fora da linha sobre os diários do H3** (a ordem temporal do replay se reconstrói: a previsão
existe em t_emissao e o rótulo em t_rotulo). Os parâmetros de janela vêm do disjuntor de cada fluxo (janela W e
intervalo m de `fluxos/*.py`), sem ajuste.

- **R1, média prevista contra taxa realizada (CITL móvel, com janela e histerese).** As previsões resolvidas são
  ordenadas por t_rotulo. A cada m resoluções, sobre as W mais recentes, calculo d = ȳ − p̄ (p̄ da previsão oficial)
  e z = (Σy − Σp) / √Σp(1−p). Liga quando |d| > 0,02 e |z| > 3 em 3 verificações seguidas. Desliga quando
  |d| < 0,01 em 3 verificações seguidas.
- **R2, BBSE (Lipton, Wang e Smola 2018), sem rótulo.** A cada m emissões, no instante t da última: a fonte são as W
  previsões mais recentes com t_rotulo ≤ t; o alvo são as W emissões mais recentes, sem rótulo. Classificação dura
  ŷ = 1{p > π_fonte}, com π_fonte a taxa da fonte. Com TPR e FPR da fonte e μ = média de ŷ no alvo, a taxa do alvo
  estimada é q̂ = (μ − FPR)/(TPR − FPR), cortada em [0, 1]. A estatística é d_B = q̂ − p̄_alvo. Liga quando
  |d_B| > 0,02 em 3 verificações seguidas e desliga quando |d_B| < 0,01 em 3 seguidas. A hipótese do BBSE (p(x|y)
  fixo e classificador fixo) não vale aqui, porque o modelo se reajusta; isso é um limite declarado.

**Fluxos:** os seis binários (voos 2013, voos 2023, ciclones, prisões 1997–2002, prisões 2020–2021, Rhode Island).

**Medida.** Para cada fluxo: a data (e o mês) do primeiro alarme no cofre e de cada alarme no desenvolvimento, de R1,
R2 e do sensor original (eventos `deriva` em `h3/saida/<fluxo>/resultados.json`). Os alarmes no desenvolvimento são o
preço (alarmes que não resultaram em falha de cofre).

**Pergunta de decisão: "o teste de rótulo teria visto 2013 antes?"** Sim, se R1 ou R2 ligar no cofre de 2013 antes de
2013-10-16 (o primeiro alarme do sensor original no cofre, `residuo_padronizado`, t = 1381881600), ou se ligar no
cofre quando o original não liga. Também relato se ligaria ainda em setembro (antes do corte), quando a correção
teria chegado a tempo. Os testes só detectam; não ligo nenhuma ação a eles nesta análise.

## (c) A correção dos voos posta à prova

A correção é a recalibração online do intercepto do H3b: p' = σ(logit p + δ), δ ← δ + 0,002 (y − p'). Ela foi
declarada antes de ver qualquer dado de 2023, e não tem parâmetro ajustado em dados posteriores a 2013.

**Definição de "desvio como o de 2013".** Num período, o modelo cru (sem a correção; o reserva onde a origem não é o
modelo) tem |p̄ − ȳ| ≥ 0,02.

**(c1) Dados reais novos.** Achei, no GitHub, dois anos de voos com o mesmo esquema e tempo meteorológico:
- **NY 2017**, pacote `nycflights` de jayleetx (github.com/jayleetx/nycflights), 303 748 voos de EWR, JFK e LGA;
- **Portland e Seattle 2014**, pacote `pnwflights14` de ismayc (github.com/ismayc/pnwflights14), 162 049 voos.

Rodo o fluxo `voos23` sem mudança no modelo nem na correção, com corte em 1º de outubro e início em 22 de janeiro
do respectivo ano. As únicas adaptações são de leitura: o mapa de arquivos, a conversão de hora local para UTC
(America/New_York e America/Los_Angeles) e, nos dados do noroeste, a lista de aeroportos (PDX, SEA) no lugar de
(EWR, JFK, LGA) e o instante programado montado de year, month, day, hour e minute. O pacote `hflights` (Houston
2011) fica de fora: não tem horário programado dos voos cancelados nem tempo meteorológico.

**Critério (c1).** Para cada ano novo:
- **Teste principal:** se o modelo cru tiver |p̄ − ȳ| ≥ 0,02 no cofre out–dez, a correção passa se a previsão
  oficial cumprir o G2 (|p̄ − ȳ| ≤ 0,02, ECE ≤ 0,03, habilidade > 0). Se o cru não tiver esse desvio, o ano não testa
  a correção, e eu digo isso.
- **Teste secundário:** em todo trimestre civil, de 2017, 2014 e 2023, inclusive os de desenvolvimento (que também
  são fora da amostra para a correção, declarada antes), em que o cru tiver |p̄ − ȳ| ≥ 0,02, a correção passa no
  trimestre se o oficial tiver |p̄ − ȳ| ≤ 0,02. Relato todos os trimestres que cumprirem a condição, não só os
  favoráveis. O 1º trimestre começa em 22 de janeiro.

**(c2) Teste de estresse no cofre de 2023 (é estresse, não prova).** No fluxo `voos23`, só nos voos com partida
programada ≥ 2023-10-01, troco y = 0 por y = 1 com probabilidade f, sorteada por voo com semente 20261015, sem
mudar o instante do rótulo nem nenhuma outra tabela. Isso é deriva pura de rótulo, sem sinal nas features, o caso
mais difícil para o modelo.
- **S1, degrau:** f = 0,026691 / (1 − ȳ₂₀₂₃), com ȳ₂₀₂₃ a taxa observada no cofre de 2023 (já aberto no H3b), para
  que o aumento esperado da taxa seja o desvio de 2013.
- **S2, perfil de 2013:** um f por mês (out, nov, dez), com o desvio ȳ − p̄ medido em cada mês do cofre de 2013 no
  diário `h3/saida/voos/diario.parquet` (previsão oficial), dividido por (1 − ȳ do mês em 2023). Se o desvio de um
  mês for negativo, f = 0 nesse mês.

Faço o replay completo (motor, disjuntor, sensores, reajustes) sobre as cópias. **Critério (c2):** a correção
aguenta o estresse se a previsão oficial cumprir o G2 no cofre estressado (|p̄ − ȳ| ≤ 0,02, ECE ≤ 0,03, habilidade
> 0). Relato o cru ao lado; se o cru também passar, o estresse não foi forte o bastante para discriminar, e eu digo
isso.

**(c3) Informativo: retrospectiva de 2013 fora da linha.** Aplico a recursão de δ sobre o diário de 2013, na ordem
do replay, mantendo as origens (modelo ou reserva) do diário. Antes, valido essa réplica fora da linha contra o δ
gravado no diário de 2023. A correção foi desenhada depois de ver 2013, então isto não é teste.

## O que conta no K deste ponto

Esta é a declaração 1. Os replays dos itens (c1) e (c2) são 4 (2017, 2014, S1, S2). Correções de leitura que não
mudem previsões serão relatadas; qualquer mudança de critério depois de ver resultado entra como declaração nova.
