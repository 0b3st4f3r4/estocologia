# H3d: declaração antes de rodar (prisões, segundo teste com dados novos)

Escrita em 2026-10-15, antes de qualquer replay destes dados. O SHA-256 deste arquivo entra no pré-registro do
fluxo novo.

## O pedido e o que já se sabe

sil disse "Prossiga com H3". A única pendência do H3 são as prisões.
- Em 1997–2002, o ECE foi 0,036.
- No H3c (Toronto 2020–2021), o ECE foi 0,067, com |p̄ − ȳ| = 0,0002. O modelo era **pouco confiante**: um erro de
  inclinação, que a recalibração do intercepto não corrige.

Esses resultados ficam como estão. Não há prisões de Toronto depois de 2021 ao alcance desta rede, então o cofre
novo vem de outra base: **Rhode Island, paradas de trânsito da polícia estadual, 2005–2015** (Stanford Open
Policing Project, ODC-By; espelho no GitHub; ver `../dados-reais/h3c/FONTES.md`).

**O que vi destes dados antes de declarar.** Vi o esquema, os níveis das categorias e, sem querer, as contagens
gerais de `stop_outcome` na base inteira. Essas contagens dão uma taxa-base de prisão do motorista em torno de 3%.
Não vi nada por ano, por cor nem por período do cofre.

## A lição do H3c, transformada em especificação (declarada aqui, testada só neste cofre)

- **Recalibração online de Platt.** A previsão oficial é p' = σ(a + b · logit p). A cada rótulo resolvido, (a, b)
  dão um passo de gradiente na log-perda: a ← a + η (y − p'), b ← b + η (y − p') · logit p, com η = 0,002 e
  início a = 0, b = 1. Ela substitui a recalibração só do intercepto e corrige também a inclinação.
- **Penalização menor.** A `logistica` passa a usar λ = 1e-4 (no H3c era 1e-3). A causa inferida do H3c foi o
  encolhimento excessivo.

## Desenho do fluxo `prisoes_ri` (fixado aqui)

- **Pergunta.** O motorista parado vai ser preso? É a decisão de custódia na abordagem.
- **Rótulo.** y = 1 se `stop_outcome` = "Arrest Driver". As linhas sem desfecho (os cerca de 5 300 registros vazios)
  saem antes de qualquer coisa. A prisão de um passageiro não conta.
- **Tempo.** `stop_date` + `stop_time` (hora local, tratada como UTC; só a ordem importa). O rótulo fica conhecido um
  dia depois. O relógio anda a cada 6 h.
- **Features, todas conhecidas na abordagem:** sexo, idade (padronizada, com o quadrado), `violation_raw` (12
  categorias), hora do dia (4 faixas) e dia da semana.
- **Fora do modelo:**
  - a cor (`driver_race`), porque o modelo é cego à cor; ela entra só no F6;
  - `search_conducted`, `search_type`, `drugs_related_stop` e `stop_duration`, que são registrados durante ou depois
    da decisão (vazamento);
  - `county_name`, que está vazio.
- **Modelo.** Regressão logística, λ = 1e-4. É reajustada em 1º de janeiro só com os anos anteriores, inclusive no
  reajuste por deriva, e passa pela recalibração de Platt acima. O modelo cru fica no diário.
- **Reserva.** A taxa de y do ano anterior.
- **Escopo.** O modelo vale se a categoria de `violation_raw` apareceu no treino.
- **Disjuntor e sensores.** Os parâmetros do fluxo `prisoes`. Os sensores são a idade e o resíduo padronizado.
- **Decisão declarada.** Marcar o caso para revisão se p(prisão) > 0,10.
- **Calendário.**
  - 2005 é só treino; o início é 2006-01-01.
  - **O corte do cofre é 2013-01-01**: o cofre é 2013–2015.

## Critério

**(a) O G2 do H3, sem mudança.** |p̄ − ȳ| ≤ 0,02, ECE (10 caixas de massa igual) ≤ 0,03 e habilidade de Brier > 0
contra a climatologia móvel.

**(b) Um critério relativo, mais estrito, que também é obrigatório.** Com taxa-base perto de 3%, as tolerâncias
absolutas de (a) quase não testam nada; um previsor constante as cumpriria. Por isso, para este fluxo, exijo
também:
- |p̄ − ȳ| ≤ 0,2 · ȳ;
- ECE ≤ 0,3 · ȳ;

com ȳ sendo a taxa observada no cofre. Isto torna o teste mais difícil, não mais fácil.

Também valem: G1 (pytest e reexecução limpa idêntica), G3 (zero vazamento, com o canário e o controle positivo), G5
(no máximo 3 varreduras por tabela) e a trilha.

O F6 (trilema Black contra White, por ano do cofre) e o F5 (publicações com privacidade diferencial por cor e ano)
são relatados como informativos.

## Regra

- Se `prisoes_ri` passar em (a), (b), G1, G3, G5 e na trilha, a pendência das prisões fica fechada e a saída do H3
  fica aprovada. A pergunta (prisão na abordagem de trânsito) não é idêntica às anteriores, e isso fica registrado
  para sil.
- Se falhar, a pendência continua aberta e nada é ajustado depois de ver o cofre.

Este é o K = 1 do `prisoes_ri`. Nenhuma alternativa foi rodada.
