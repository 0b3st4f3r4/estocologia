# H2d: o portão do ICP com a regra de saturação (K = 3), o Sachs por célula e o cinto com efeito fixo

**Saída do H2d: NÃO aprovada.** O G3 falha: Ŝ ≠ ∅ em **1 domínio, 1 área (Educação)**; o critério pede 3. A regra
de saturação tirou o Ŝ do RAND, como a declaração previa. O Sachs e o cinto agora chegam ao F4, mas nenhum dá
Ŝ ≠ ∅: o Sachs rejeita todos os 1 024 conjuntos e o cinto aceita conjuntos disjuntos. G1, G2, G4 e a trilha passam
(`saida/portoes.json`).

```
python3 rodar.py                    # funil nos 5 domínios (~9 min)
python3 portoes.py --reproducao     # portões G1–G5, com segunda execução limpa
python3 grafo.py                    # hipergrafo
```

A execução registrada em `saida/` rodou num diretório local (`ESTOCOLOGIA_SAIDA=/tmp/...`) e depois foi copiada
para cá. O código é o mesmo. `calibracao.py` é o do H2c, sem mudança (o teste K = 2 não mudou); não foi rodado de
novo, e `saida/calibracao.json` não existe aqui.

## Ordem dos passos (pré-registro)

1. **A declaração veio antes.** `DECLARACAO_K3.md` foi escrita antes de qualquer execução do código do H2d.
   - SHA-256 e hora UTC em `DECLARACAO_K3.sha256`: `574ea148…cf204c73`, 2026-10-14T22:33:00Z.
   - Ela lista o que eu já sabia dos resultados anteriores (inclusive o ICP diagnóstico do H2b no Sachs e no
     cinto e o F2/F3 que os podaram).
2. **Implementação** em `h2d/`, cópia do `h2c/`. `h2/`, `h2b/`, `h2c/`, `h3/` e `dados-reais/` não foram tocados.
3. **Testes.** `tests/test_k3.py` (5 testes novos); os 22 testes passaram de primeira.
4. **Só então os dados reais**: funil e portões, uma vez. **Nada foi ajustado depois de ver os resultados do K = 3.**

## O que mudou e por quê

### (a) Regra de saturação, nos 5 domínios
- **Princípio.** Se o ambiente E é função determinística de X_S, Y ⊥ E | X_S vale por construção, e o teste de
  invariância tem poder zero. Um teste sem poder não certifica invariância.
- **Regra.** Conjuntos saturados saem da família: não contam como aceitos nem como rejeitados. Ŝ é a interseção
  dos aceitos **não saturados**. Se nenhum não saturado é aceito, o ICP fica como "nada aceito" e Ŝ = ∅.
- **Medida.** Acerto do vizinho mais próximo deixando a linha de fora (1-NN LOO) ao prever E pelas colunas de S
  (dp 1 por coluna). Empates exatos são resolvidos pela fração de linhas da célula no mesmo ambiente. Saturado se o
  acerto médio ≥ 0,99. Cobre X discreto (célula pura) e X contínuo em faixas disjuntas.
- **Código.** `estocologia/icp.py`: `acerto_1nn`, `saturado_k3` e o parâmetro `regra_saturacao` de `icp()`
  (padrão ligado). O resultado traz `regra_saturacao`, com o Ŝ que o K = 2 daria (`S_hat_sem_a_regra`), e o
  funil publica a lista de saturados. O teste K = 2 não mudou.

### (b) biologia_sachs: a célula é a unidade
- As intervenções são por condição e as células são as réplicas, como nas análises clássicas de ICP desses dados
  (Meinshausen, Hauser, Mooij, Peters, Versteeg & Bühlmann 2016, PNAS 113:7361 — citada de memória, conferir).
- **Isto muda o F2 e o F3, e foi o que levou o Sachs ao F4.** É uma mudança feita depois da poda no F3 do H2b.
  Y, candidatos, ambientes, cofre, DAG, ajuste, priori e decisão são os do H2b.
- **Ressalva.** O efeito de desenho por condição é 67 (H2b); por célula, 1,13. As células de uma condição dividem
  tubo, coloração e corrida. O EP do F2, o z do F3 (29 contra 3,8 por condição) e a posterior ficam otimistas.

### (c) transporte_cinto: efeito fixo de estado (within)
- **Diagnóstico conhecido.** O F2 do H2b podou por EP por estado 0,140 > dp da priori 0,133, com efeito de
  desenho 6,1. A linha de base de 2 anos (1983–1984) deixava ruído constante por estado no y e não tirava o nível
  de estado dos X.
- **Correção.** y e os 9 candidatos menos a média de treino do próprio estado (regimes no e secondary). O cofre
  (primary) é centrado com as médias de treino do estado. É o efeito fixo de estado padrão de painel (Cohen & Einav
  2003 — citada de memória, conferir). Escolhida sem rodar alternativas.
- **Efeito colateral.** 3 estados com fiscalização primária não têm linha de treino com uso observado e saíram do
  cofre: o cofre caiu de 81 para 57 linhas (5 estados).
- **Conhecimento prévio declarado.** O ICP diagnóstico do H2b (K = 1) no cinto aceitava só
  {idade21, idade_media, vel65, vel70}, p = 0,054. Isso já era conhecido quando a correção foi escolhida.

### (d) Conjunto adicional: nenhum
Nenhum reserva de `../dados-reais/h2b/FONTES.md` tem intervenção limpa em X numa área nova (justificativa por
arquivo na declaração). Rodar um deles seria procurar área, não testar invariância.

## Resultados por domínio: K = 1, K = 2, K = 3

"Maior p" é o maior p da família que conta no K. No K = 3, a família exclui os saturados. "Saturados" no K = 2 é o
diagnóstico antigo (célula discreta pura); no K = 3, a regra 1-NN, como aceitos saturados / saturados testados.

| domínio | área | K = 1: Ŝ (maior p) | K = 2: Ŝ (maior p; aceitos saturados) | K = 3: Ŝ | K = 3: aceitos na família / 1 024 | K = 3: maior p (conjunto) | K = 3: saturados (aceitos / testados) |
|---|---|---|---|---|---|---|---|
| `biologia_sachs` | Biologia | não rodou (podado no F3; diagnóstico: tudo rejeitado, 1e-211) | não rodou (podado no F3) | **∅**, tudo rejeitado | 0 | 2,0e-108 ({jnk, mek, p38, pip2, pka, pkc, raf}) | 0 / 0 |
| `transporte_cinto` | Transporte | não rodou (podado no F2; diagnóstico: só {idade21, idade_media, vel65, vel70}, 0,054) | não rodou (podado no F2) | **∅**, aceitos disjuntos | 496 / 512 | 1,00 ({ano}) | 0 / 0 |
| `direito_teto_beneficio` | Direito | **{beneficio_log}** (0,087) | ∅, aceitos disjuntos (1,00; 0/580) | **∅**, aceitos disjuntos | 578 | 1,00 ({beneficio_log, industria, kentucky}) | 2 / 2 |
| `educacao_star_turmas` | Educação | ∅, tudo rejeitado (0,0013) | **{turma_pequena}** (0,69; 152/209) | **{anos_pequena_antes, turma_pequena}** | 57 | 0,075 ({anos_pequena_antes, menina, mestrado_prof, prof_negro, tipo_escola, turma_pequena}) | 152 / 256 |
| `saude_rand` | Saúde | ∅, tudo rejeitado (2,5e-14) | **{coparticipacao}** (1,00; 512/512) | **∅**, só saturados aceitos | 0 | 1,7e-29 ({doencas_cronicas, idade, local, pagamento_log, renda_log, teto_log}) | 512 / 597 |

O cinto tem 9 candidatos, por isso 512 conjuntos.

### Outras saídas do F4 (K = 3)

| domínio | F2: EP da chave (unidade) / dp priori; efeito de desenho | F3: z; p | habilidade no cofre (modelo / Ŝ / todos) | cobertura conforme | M15 | F8 |
|---|---|---|---|---|---|---|
| Sachs | 0,0083 / 0,30; 1,13 (por condição: 67) | 29,0; 0,005 | 0,14 / 0,00 / 0,83 | 0,96 | aprovado | manter (P = 0,00); elasticidade 0,24 [0,23; 0,26] |
| cinto | 0,077 / 0,133; 1,85 | −2,9; 0,010 | 0,80 / 0,00 / 0,80 | **0,75** | **reprovado (conforme)** | — |
| direito | igual ao K = 2 | igual | 0,19 / 0,00 / 0,19 | 0,89 | aprovado | adotar (P = 1,00) |
| STAR | igual ao K = 2 | igual | 0,10 / 0,002 / 0,10 | 0,88 | aprovado | manter (P = 0,47) |
| RAND | igual ao K = 2 | igual | 0,07 / 0,00 / 0,07 | 0,92 | aprovado | adotar (P = 0,72) |

Os intervalos são de 90% da posterior. Direito, STAR e RAND só mudam no ICP e no que depende de Ŝ (preditor
invariante e hash do pré-registro); posterior, F2, F3 e decisão são os do H2c.

### O que cada domínio diz

- **Educação (STAR): o único Ŝ, e ele sobrevive à regra.**
  - Os 152 aceitos com turma_pequena + série são saturados, iguais ao diagnóstico do K = 2. Sobram 57 aceitos não
    saturados, e Ŝ = {anos_pequena_antes, turma_pequena}, como a sensibilidade do H2c antecipava.
  - **Ressalvas.** O maior p não saturado é 0,075, perto do α. {turma_pequena} sozinho é rejeitado (p = 0,010). O
    preditor invariante quase não ganha da climatologia no cofre: habilidade de 0,002.
- **Saúde (RAND): perdeu o Ŝ.** Os 512 conjuntos com a coparticipação são saturados. Também saturam 85 sem ela
  (82 contêm renda_log + teto_log: o teto de desembolso depende da renda, e os dois juntos quase identificam o
  plano), todos rejeitados. Nenhum não saturado passa; o melhor
  tem p = 1,7e-29. A invariância da coparticipação não é testável com estes ambientes, e a regra diz isso.
- **Direito: continua sem Ŝ.** Dois aceitos são saturados: {beneficio_log, kentucky} e
  {alta_renda, beneficio_log, kentucky}. O benefício com o estado identifica o regime antes/depois do teto, porque
  o teto muda o valor. Eles saírem não muda nada: 578 aceitos, com conjuntos sem o benefício, interseção vazia.
- **Biologia (Sachs): chega ao F4 e rejeita tudo.** A unidade por célula passou F2 e F3 com folga. O ICP rejeita
  os 1 024 conjuntos, o melhor com p = 2e-108. Nenhum conjunto é saturado, porque as condições se sobrepõem em X. A
  lei de Erk dado os outros sinais não é a mesma entre as 11 condições de treino. O resultado repete o diagnóstico
  K = 1 do H2b (1e-211) com o teste não paramétrico.
  - **Leitura.** Pode ser intervenção oculta (inibidores que mudam atividade, não a fosforilação medida), efeito
    de lote por condição (o efeito de desenho 67 é isso) ou pai não medido. O ICP não separa as três.
- **Transporte (cinto): chega ao F4, mas sem poder.**
  - **O que o efeito fixo fez.** Cortou o efeito de desenho de 6,1 para 1,85 e o EP de 0,140 para 0,077, e o
    domínio passou F2 e F3.
  - **O que o ICP fez.** Com só 2 ambientes de treino e 475 linhas, aceitou 496 dos 512 conjuntos. {uso_cinto}
    sozinho passa (p = 0,30), mas {ano} também (p = 1,00), e a interseção é vazia. Só o vazio e poucos outros são
    rejeitados (p(∅) = 3e-17). É o limite de poder que o pacote já declarava.
  - **Fora do ambiente.** O M15 reprovou a cobertura conforme no cofre: 0,75 < 0,85, com 57 linhas de 5 estados.
    Esse portão não entra no G3, mas o cinto não emite decisão.

## Portões

| portão | resultado |
|---|---|
| G1 simulabilidade e reprodução | PASSA: 22 testes; a segunda execução limpa é idêntica nos 5 domínios |
| G2 funil completo e pré-registro | PASSA: todo domínio termina com motivo, K = 3 declarado e pré-registro antes do cofre |
| **G3 ICP entre ambientes** | **FALHA: Ŝ ≠ ∅ em 1 domínio (educação), 1 área; exigido: 3 áreas** |
| G4 contenção de varredura | PASSA: 10 leituras, 0 SELECT *, cofre só pelo F3, banca toda recusada |
| Trilha do barramento | PASSA: 54 fichas, sem quebra |
| G5 (informativo) | O M15 aprovou 4/5 que chegaram ao F4 (o cinto reprova na cobertura conforme); 5 pareceres de risco esperam a assinatura de sil |

**O G3 contra o texto sem mudança.** "Em pelo menos 3 domínios de áreas distintas, o ICP padrão aceita algum
conjunto E devolve Ŝ ≠ ∅." No K = 3, só a Educação cumpre: o ICP aceita 57 conjuntos não saturados e
Ŝ = {anos_pequena_antes, turma_pequena}.

**Quais áreas contam.**
- **Conta só a Educação.**
- **Saúde não conta.** O RAND não aceita nenhum conjunto não saturado. Pela letra do G3 no K = 2 contava, só por
  saturação; a regra (a) existe para não contar isso.
- **Biologia não conta.** O ICP rejeita tudo.
- **Transporte e Direito não contam.** O ICP aceita, mas Ŝ = ∅.
- **Mesmo ignorando a regra, seriam 2 áreas** (Educação e Saúde): o portão falharia de qualquer jeito.

## Limites

- **Multiplicidade entre K.** Foram três especificações sucessivas, cada uma escrita depois de ver os resultados da
  anterior:
  - K = 1: o teste linear;
  - K = 2: troca do teste;
  - K = 3: a regra de saturação, a unidade do Sachs e o efeito fixo do cinto.

  Cada K foi declarado antes de rodar, mas a sequência é uma busca: a chance de algum Ŝ ≠ ∅ espúrio aparecer em
  algum K é maior que α.
  - Se o G3 tivesse passado no K = 3, a passagem precisaria de desconto por isso.
  - Ele não passou, e o único Ŝ que sobrevive em dois K (STAR, K = 2 e K = 3) aparece só depois de trocar o teste.
    No K = 1, com o teste linear, o STAR rejeitava tudo.
- **Sachs por célula.**
  - A garantia do ICP supõe linhas independentes dentro do ambiente, e as células de uma condição não são. Como
    ambiente = condição, a dependência dentro do ambiente é real (efeito de desenho 67).
  - Aqui o ICP rejeitou tudo, então isso não inflou nenhum Ŝ. Mas o F3 e a posterior do Sachs são otimistas.
- **A regra 1-NN.**
  - Ambientes quase separáveis (≥ 99%) contam como saturados, o que é deliberado.
  - O limiar 0,99 é herdado do K = 2 e não foi calibrado contra alternativas.
  - Em X contínuo de alta dimensão, o 1-NN pode subestimar a separabilidade. Nenhum Sachs ou cinto saturou.
- **Cinto com 2 ambientes.** O poder do ICP é baixo por desenho. Aceitar 496 de 512 conjuntos é pouco poder, não
  evidência de invariância.
  - O cofre ficou com 57 linhas e a cobertura conforme falhou.
  - O efeito fixo foi escolhido sabendo que o F2 tinha falhado por efeito de desenho, e sabendo do ICP
    diagnóstico do H2b.
- **Os limites do H2c continuam valendo:** forma que depende de X (massa em zero no RAND), ajuste aditivo sem
  interações, prioris citadas de memória, agrupamento ausente no RAND e no direito, escala pequena.

## O que fica para sil (sem executar: qualquer item é K = 4)

- **O que o K = 3 mostrou.**
  - A regra de saturação fez o que devia: tirou o Ŝ do RAND, que não era evidência.
  - As correções do Sachs e do cinto levaram os dois ao F4, mas o ICP não achou Ŝ em nenhum dos dois: rejeição
    total no Sachs e poder insuficiente no cinto.
- **Propostas de K = 4, só listadas.** Nenhuma foi rodada, e cada uma seria escolhida depois de ver o K = 3:
  - no RAND, ambientes que não fixem a coparticipação, como o local cruzado com o plano, se a premissa permitir;
  - no cinto, mais ambientes de treino, mas o cofre primary é o único regime que sobra;
  - no Sachs, outro Y ou a exclusão de condições com intervenção oculta suspeita.
- **O que eu recomendaria.** Nenhuma das propostas acima. Com 3 respecificações já feitas, a recomendação honesta
  é aceitar que o G3, como está, não passa com estes 5 conjuntos.
  - O próximo passo útil é **dados novos de intervenção em X contínuo**, em áreas novas, declarados antes de olhar.
  - A alternativa é sil decidir explicitamente sobre o critério de 3 áreas.
