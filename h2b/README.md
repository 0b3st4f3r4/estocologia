# H2b: o portão do ICP com ambientes que são intervenções conhecidas em X

**Saída do H2b: NÃO aprovada.** G1, G2, G4 e a trilha passam. O G3 falha de novo: o critério pede Ŝ ≠ ∅ em pelo
menos 3 domínios de áreas distintas, e só 1 entregou (direito, Ŝ = {benefício}). O critério é o do PLAN, copiado do
H2 sem nenhuma mudança.

Esta rodada segue a decisão de sil: buscar dados com intervenções conhecidas em X e manter o portão forte. São 5
domínios novos, com dados em `../dados-reais/h2b/` (ver `FONTES.md` e `SHA256SUMS` lá).

```
python3 rodar.py                    # funil nos 5 domínios (~2 min)
python3 portoes.py --reproducao     # portões G1–G5, com segunda execução limpa (~2 min a mais)
python3 grafo.py                    # hipergrafo: 3 199 nós, 3 301 arestas de árvore, 10 pontes, 245 hiperarestas
```

A execução registrada em `saida/` rodou num diretório local e depois foi copiada para cá
(`ESTOCOLOGIA_SAIDA=<local>`): o DuckDB fica lento no disco de rede. O código é o mesmo.

## O que mudou em relação ao H2

- **Dados e ambientes.** Os 12 domínios de conveniência deram lugar a 5 domínios em que o ambiente é um regime
  de intervenção documentado sobre um preditor. O cofre é sempre um regime inteiro deixado de fora.
- **Arcabouço.** É uma cópia de `h2/` (estocologia/, rodar.py, portoes.py, grafo.py, conftest.py, tests/). Nada
  em `h2/` foi tocado.
  - `rodar.py` agora lê `../dados-reais/h2b` por padrão.
  - `portoes.py` ganhou só uma nota no cabeçalho. Os critérios G1–G5 são idênticos.
- **Uma falha corrigida (`estocologia/icp.py`).** Ela foi achada lendo o código, antes de qualquer execução.
  - **O problema.** O teste de invariância contava os graus de liberdade dos ajustes por ambiente pelo número
    de colunas, não pelo posto. Com ambientes que são intervenções, um candidato costuma ser constante dentro do
    ambiente: coparticipação do plano, turma pequena, estado. O mesmo acontece quando um nível categórico some de
    um ambiente. Essas colunas colineares somavam graus de liberdade sem reduzir a soma dos quadrados. O F ficava
    diluído e o teste aceitava demais.
  - **A correção.** Os graus de liberdade agora usam `matrix_rank`.
  - **O teste.** Há um teste novo em `tests/test_h2.py`, com X constante por ambiente. Sob H0 o teste rejeita
    7% (meta < 9%); sob H1, 100%.
  - **Efeito no H2.** Rerodei o ICP dos 6 domínios do H2 que chegaram ao F4 com o ICP aceitando algo ou
    rejeitando tudo (direito, crédito, saúde, currículos, contracepção, STAR), com as duas versões do código. Ŝ e o número
    de aceitos ficaram idênticos. A falha não mudou nenhum resultado do H2.

## Resultados por domínio

| domínio | área | Y | X intervindo → ambientes (treino / cofre) | ICP | Ŝ | habilidade no cofre (modelo / Ŝ) | status |
|---|---|---|---|---|---|---|---|
| `biologia_sachs` | Biologia | log p44/42 (Erk) | reagente por condição: 11 condições / cond. 14 (ICAM2 + LY294002) | não rodou | — | — | podado no F3: p = 0,10 com 11 condições como unidades (efeito de desenho 67) |
| `saude_rand` | Saúde | log(1 + gasto médico) | coparticipação sorteada: c0, c25, c95, c100 / c50 | rejeitado (0/1024; maior p = 2,5e-14) | ∅ | 0,07 / 0,00 | aprovado no F4: adotar 25% (P = 0,72); β = −0,84 [−0,94; −0,73] |
| `direito_teto_beneficio` | Direito | log da duração do afastamento | teto do benefício: ky_antes, ky_depois, mi_antes / mi_depois | 240/1024 aceitos, todos com o benefício | **{beneficio_log}** | 0,19 / 0,07 | aprovado no F4: adotar (P = 1,00); elasticidade 0,58 [0,49; 0,67] |
| `transporte_cinto` | Transporte | log mortes/milha relativo a 1983–84 | lei de cinto: no, secondary / primary | não rodou | — | — | podado no F2: EP por estado 0,14 > dp da priori 0,13 |
| `educacao_star_turmas` | Educação | nota padronizada pelas regulares | turma sorteada × série: 7 ambientes / 3_pequena | rejeitado (0/1024; maior p = 0,0013, {turma_pequena}) | ∅ | 0,10 / 0,00 | aprovado no F4: manter (P = 0,47); efeito 0,15 dp [0,09; 0,21] |

Os intervalos são de 90% da posterior. O β da saúde é por unidade de taxa, de 0 a 100%. O P da coluna de status é
P(adotar compensa), do F8. A habilidade é 1 − EQM/EQM da climatologia, no regime do cofre.

### O que cada domínio diz

- **Direito (Meyer, Viscusi & Durbin).**
  - **O que deu Ŝ.** É o único com Ŝ ≠ ∅, e o Ŝ é a própria chave: todos os 240 conjuntos aceitos contêm o log
    do benefício. Com 95% de cobertura, o benefício está entre os pais invariantes da duração.
  - **Ressalva honesta.** A aceitação é marginal. O maior p é 0,087, e o conjunto vazio dá p = 1,5e-6. Os
    aceitos passam por pouco do α = 0,05.
  - **Posterior e cofre.** A elasticidade posterior (0,58) ficou acima da mediana da literatura (0,35). O modelo
    previu o Michigan pós-reforma com habilidade 0,19.
- **Saúde (RAND).**
  - **Onde o ICP rejeitou.** O ICP rejeitou tudo. A massa em zero e a resposta côncava à coparticipação (queda
    maior de 0 a 25%), declaradas como risco no pacote, bastam com 15 mil pessoas-ano para rejeitar uma reta.
  - **O efeito ainda transporta.** A previsão do regime de 50% teve habilidade 0,07 e cobertura conforme 0,92.
- **Educação (STAR por série).**
  - **Onde o ICP rejeitou.** O ICP rejeitou tudo. O melhor conjunto é exatamente {turma_pequena}, com p = 0,0013.
  - **O que isso sugere.** A lei da nota dada a turma pequena quase é invariante entre séries, mas não passa. A
    rejeição vem do teste de média por F ou da variância dos resíduos por Levene; não separei qual.
  - **Por que "manter".** O efeito de 0,15 dp ficou abaixo do custo suposto de 0,15, e a decisão é manter.
- **Biologia (Sachs).**
  - **Por que parou no F3.** A unidade declarada é a condição: as células de uma condição dividem tubo e corrida.
    Com 11 condições no treino e efeito de desenho 67, o z por agrupamento (3,8) não vence o nulo de permutação.
  - **Diagnóstico depois da poda.** Rodei o ICP no treino depois, só para informar. Ele não conta no G3 e não
    mudou nenhuma especificação. Rejeita tudo com folga: maior p = 1e-211. A lei log-linear de Erk não é a mesma
    nas 11 condições, com ~800 células cada.
- **Transporte (cinto).**
  - **Por que parou no F2.** Os 48 estados com uso do cinto observado não informam mais que a priori.
  - **Diagnóstico depois da poda.** No mesmo regime de diagnóstico, que não conta, o ICP aceitaria um único
    conjunto, {idade21, idade_media, vel65, vel70}, com p = 0,054. Ele não contém o uso do cinto e não sustenta
    nada.

## Portões

| portão | resultado |
|---|---|
| G1 simulabilidade e reprodução | PASSA: 10 testes; a segunda execução limpa é idêntica nos 5 domínios |
| G2 funil completo e pré-registro | PASSA: todo domínio termina com motivo, K declarado e pré-registro antes do cofre |
| **G3 ICP entre ambientes** | **FALHA: Ŝ ≠ ∅ em 1 domínio (direito), 1 área; exigido: 3 áreas** |
| G4 contenção de varredura | PASSA: 8 leituras, 0 SELECT *, cofre só pelo F3, banca toda recusada |
| Trilha do barramento | PASSA: 42 fichas, sem quebra |
| G5 (informativo) | O M15 aprovou 3/3 que chegaram ao F4; 5 pareceres de risco esperam a assinatura de sil |

## K e respecificações

- **Respecificações.** K = 1 em todos os 5 pacotes. Nenhum Y, X, ambiente, cofre, DAG, ajuste ou priori mudou
  depois de qualquer execução sobre os dados.
- **Antes da primeira execução:**
  - Troquei um dois-pontos que quebrava o YAML do STAR. Foi sintaxe, não especificação.
  - Conferi com dados só a estrutura: contagens por ambiente, faltantes e se o F1 passa, que não toca os dados.
- **Escolhas do programa, declaradas:**
  - **Os 5 conjuntos.** São os 5 que o `FONTES.md` recomendou antes de qualquer ajuste. Os outros 7 baixados
    (Fatalities, CigarettesSW, Cigar, njmin3, thornton, nsw, Sachs discretizado) não foram rodados.
  - **O STAR já era conhecido.** A base foi usada no H2, com tipo de escola como ambiente, e lá o ICP rejeitou
    tudo. A nova especificação (turma × série) foi escrita sabendo disso.
  - **Sachs.** As condições 6 e 13 (U0126) foram excluídas por decisão anterior a qualquer ajuste, porque o
    U0126 age na lei de Erk dado pmek. A escolha de Y = Erk também é anterior.
- **Depois dos resultados.** O ICP dos dois domínios podados rodou como diagnóstico. Ele está relatado acima e
  não entra no portão. **Não respecifiquei nada para tentar passar.** Exemplos que mudariam o funil:
  - tomar a célula como unidade no Sachs, o que provavelmente passaria o F3;
  - trocar o cofre do cinto para ganhar ambientes de treino;
  - usar um Y binário de uso no RAND.

  Cada um seria escolha depois de ver a falha. Se sil quiser, cada um deve entrar como K = 2 declarado.

## Limites

- **Poder e premissas do ICP.** O ICP com n grande rejeita qualquer desvio do modelo linear gaussiano: não
  linearidade, massa em zero, variância que muda com o regime. Os dois "rejeitados" deste lote (RAND, STAR) caem
  nisso, mais do que em pai oculto. Um teste de invariância não paramétrico ou de resíduos, como o ICP não linear
  de Heinze-Deml, Peters & Meinshausen (2018), citado de memória, conferir, mudaria o teste. Seria mudar a régua,
  e isso cabe a sil.
- **Funil antes do ICP.** O G3 só conta domínios que chegam ao F4. No Sachs e no cinto, os filtros baratos
  podaram antes, por motivos legítimos: unidade com poucas condições e priori já tão precisa quanto os dados.
- **Treino com 2 ambientes no cinto.** O cinto teria só 2 ambientes de treino, com pouco poder.
- **Agrupamento.** No RAND o sorteio foi por família, mas o arquivo só tem a pessoa. No direito não há
  agrupamento: a unidade é o pedido. Os EP podem ser otimistas.
- **Prioris e decisões.** Todas as prioris SHELF vêm de literatura citada de memória, marcada "conferir"; nenhum
  especialista foi consultado. A priori do Sachs é quase só a forma da cascata MAPK, sem número publicado.
  Custos e benefícios das decisões são suposições.
- **Escala.** São 1,0 MB varridos e 3 199 nós no hipergrafo, ainda longe da escala do PLAN.

## O que isto deixa para sil

Com intervenções conhecidas, o ICP padrão achou Ŝ não vazio onde a intervenção é uma mudança de regra limpa sobre
uma variável contínua (o teto do benefício). Em ensaios sorteados com X discreto e Y de cauda pesada ou em bloco
(RAND, STAR) ele rejeitou. Para passar o portão como está, faltam 2 áreas. Os caminhos honestos são:

- mais dados de mudança de regra sobre X contínuo, em outras áreas;
- respecificações declaradas como K = 2, como as listadas acima;
- uma decisão explícita sobre o teste de invariância usado pelo ICP. Ela muda o instrumento, não o critério de 3
  áreas.
