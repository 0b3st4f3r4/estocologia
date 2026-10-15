# H3c: declaração antes de rodar (prisões com dados novos)

Escrita em 2026-10-15, antes de qualquer replay com os dados novos. Destes dados só se viram o esquema: colunas,
tipos, contagens de linhas por trimestre e os níveis das variáveis categóricas (sexo, faixa etária, cor percebida,
divisão, categoria da ocorrência). Nenhuma taxa de desfecho, calibração ou cruzamento por cor foi calculado. O
SHA-256 deste arquivo entra no pré-registro do fluxo novo.

## O pedido e o que fica aberto

sil pediu "buscar dados novos: prisões depois de 2002". O fluxo `prisoes` do H3 (Toronto, 1997–2002) falhou no G2
com ECE 0,036 contra a tolerância de 0,03. Esse resultado fica como está; nada nele é refeito.

O teste novo usa dados que ninguém viu: **Toronto Police Service, "Arrests and Strip Searches" (RBDC-ARR-TBL-001),
2020–2021**, 65 276 prisões, espelho no GitHub de dados oficiais do portal de Toronto (ver
`../dados-reais/h3c/FONTES.md`, com hashes). Os portais oficiais estão bloqueados por esta rede.

**Não é a mesma pergunta.** O fluxo de 1997–2002 previa a liberação com intimação de detidos por porte de
maconha. O novo prevê se a pessoa presa por qualquer ocorrência é registrada na delegacia (booked) em 24 h. As
duas são a mesma decisão de custódia (liberar no local ou levar à delegacia), na mesma cidade, mas a população e a
época mudam. Por isso o fluxo novo se chama `prisoes20` e é avaliado por conta própria.

## Desenho do fluxo `prisoes20` (fixado aqui)

- **Rótulo.** y = 1 se Booked = 1 **ou** StripSearch = 1. Os metadados oficiais avisam que, por defeito no
  formulário, há registros com StripSearch = 1 e Booked = 0, e que nesses casos se deve presumir que houve
  registro na delegacia. A regra segue o aviso.
- **Tempo.** A base só traz o ano e o trimestre. O instante de cada prisão dentro do trimestre é sintético e
  determinístico, como no fluxo de 1997–2002: as linhas são espalhadas uniformemente no trimestre, na ordem do
  arquivo (ObjectId). O rótulo fica conhecido um dia depois da prisão.
- **Features, todas conhecidas no momento da prisão:**
  - sexo;
  - faixa etária (as grafias duplicadas são fundidas; faixa ausente vira categoria própria);
  - categoria da ocorrência (ausente vira categoria própria);
  - divisão policial;
  - as seis marcações de comportamento na prisão (Actions_at_arrest_*);
  - prisões anteriores da mesma pessoa (PersonID) em trimestres anteriores, contadas pelo próprio fluxo a partir
    do que já consumiu, como min(n, 5)/5 e o quadrado disso.
- **Fora do modelo:**
  - a cor percebida: o modelo é cego à cor, como antes, e a cor só entra no F6;
  - StripSearch, SearchReason_* e ItemsFound, que são registrados depois da decisão (vazamento);
  - EventID, ArrestID e ObjectId.
- **Modelo (F4).** Regressão logística com a penalização de `fluxos.base.logistica` (λ = 1e-3). É reajustada no
  início de cada trimestre só com os trimestres anteriores, inclusive no reajuste pedido pela deriva.
- **Recalibração online do intercepto, a mesma do H3b.** p' = σ(logit p + δ), δ ← δ + 0,002 (y − p'), δ inicial
  0. O modelo cru fica no diário e é relatado ao lado.
- **Reserva.** A taxa de y do último trimestre completo.
- **Escopo.** O modelo vale se a categoria da ocorrência e a divisão apareceram no treino. Fora disso vale o
  reserva.
- **Disjuntor e sensores.** Os parâmetros do fluxo `prisoes` não mudam: janela 400, margem 0,03, k_on 3, k_off 3,
  intervalo 50. Os sensores são as prisões anteriores (no lugar de "verificações", que não existe aqui) e o resíduo
  padronizado.
- **Decisão declarada.** Marcar o caso para revisão se p(registro) > 0,30, o espelho do limiar de 0,70 de
  liberação do fluxo antigo.
- **Calendário.**
  - 2020-T1 é só treino; o início é 2020-04-01.
  - **O corte do cofre é 2021-07-01**, de modo que o cofre é o 2º semestre de 2021, cerca de 16 mil prisões.
  - O cofre fica trancado até o pré-registro, que traz o hash deste arquivo.

## Critério: o mesmo do G2 do H3 para fluxos binários

No cofre, com a previsão oficial (o modelo recalibrado, ou o reserva quando o disjuntor ou o escopo mandam):
- |p̄ − ȳ| ≤ 0,02;
- ECE (10 caixas de massa igual) ≤ 0,03;
- habilidade de Brier > 0 contra a climatologia móvel.

Também valem para este fluxo:
- **G1:** pytest mais uma segunda execução limpa idêntica;
- **G3, zero vazamento:** leituras, estado, rótulo, pré-registro, e o canário com o controle positivo (ganho honesto
  ≤ 0,02, vazado > 0,2);
- **G5:** no máximo 3 varreduras por tabela;
- trilha do barramento íntegra.

O F6 (trilema por cor percebida, Black contra White, por semestre) e o F5 (publicações com privacidade diferencial
por cor e trimestre) são relatados como informativos, como no H3.

**Ressalva declarada.** A mesma pessoa aparece em várias linhas (37 347 pessoas em 65 276 prisões). O ECE e o
CITL são médias sobre prisões e não ganham IC aqui; a dependência é relatada, não corrigida.

## Regra

- Se `prisoes20` passar em G1, G2, G3, G5 e na trilha, a pendência de prisões do H3 é dada por fechada com dados
  novos, do mesmo jeito que os voos foram fechados com 2023 no H3b. Com isso a saída do H3 fica aprovada. Como a
  pergunta não é idêntica (ver acima), essa leitura fica registrada para sil confirmar.
- Se falhar, a pendência continua aberta e nada é ajustado depois de ver o cofre.

Este é o K = 1 do `prisoes20`. Nenhuma alternativa de desenho foi rodada antes.
