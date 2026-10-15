# H3b: declaração antes de rodar (pendências do H3)

Esta declaração foi escrita em 2026-10-14, antes de qualquer replay com os dados de 2023. Até ali, desses dados
só se tinham visto o cabeçalho e os tipos das colunas. O hash SHA-256 deste arquivo entra no pré-registro do
fluxo novo.

## O que falhou no H3 e o que é testado aqui

**Voos.** O fluxo de 2013 falhou no G2 com |p̄ − ȳ| = 0,027 no cofre de out–dez, e a tolerância era 0,02. O
desenho não tinha nenhum mecanismo que corrigisse um desvio da média: o disjuntor compara com o reserva, não com
a calibração.

O mecanismo novo, declarado aqui, é a **recalibração online do intercepto (F4/F3)**:
- a previsão oficial do modelo passa a ser p' = σ(logit p + δ);
- a cada rótulo resolvido, δ ← δ + η (y − p'), com η = 0,002 e δ inicial 0;
- o reserva não muda.

O mecanismo não pode ser avaliado honestamente no cofre de 2013, que já foi visto. O teste é feito num cofre
novo: **nycflights23**, todos os voos de NY em 2023 (pacote CRAN nycflights23, espelho cran/ no GitHub).
- O modelo, as features, o reserva, o disjuntor e os sensores são os do fluxo `voos`, sem nenhuma outra mudança.
- O início é 2023-01-22.
- O corte do cofre é **2023-10-01**: o cofre é out–dez de 2023, o mesmo trimestre que falhou em 2013.

**Prisões.** O fluxo falhou com ECE 0,036, numa tolerância de 0,03 e com 1 488 casos. Não há dados de prisões de
Toronto depois de 2002 ao alcance desta rede. A pendência fica aberta e declarada. Ela não é "resolvida"
afrouxando a tolerância depois de ver o resultado.

## Critério (o mesmo do G2 do H3, para fluxos binários)

No cofre de 2023, com a previsão oficial (modelo recalibrado, ou reserva quando o disjuntor ou o escopo mandam):
- |p̄ − ȳ| ≤ 0,02;
- ECE (10 caixas de massa igual) ≤ 0,03;
- habilidade de Brier > 0 contra a climatologia móvel.

Também valem, para este fluxo:
- **G3, zero vazamento:** leituras, estado, rótulo, pré-registro, e o canário com o controle positivo;
- **G5, contenção de varredura:** no máximo 3 varreduras por tabela.

O G1 é o pytest mais uma segunda execução limpa idêntica.

O resultado sem recalibração (o modelo cru em sombra, que fica no diário) é relatado ao lado. Isso mostra se foi
o mecanismo que fez a diferença.

## Regra

O H3 passa a ter os portões G2–G5 aprovados para os voos se o fluxo de 2023 passar. A pendência das prisões
continua aberta e impede a saída do H3, a menos que sil decida outra coisa.
