# Declaração K = 3: regra de saturação, unidade do Sachs e efeito fixo de estado no cinto

Escrita em 2026-10-14T22:32Z (UTC), **antes** de qualquer execução do código do H2d, em simulação ou em dados reais.
O SHA-256 deste arquivo e a hora UTC do registro ficam em `DECLARACAO_K3.sha256`. Todo o trabalho fica em `h2d/`,
uma cópia de `h2c/`; `h2/`, `h2b/`, `h2c/`, `h3/` e `dados-reais/` não são tocados.

## Situação e motivo

- **O pedido.** sil disse "Prossiga com as pendências". Para o H2, isso é tentar, com honestidade, uma terceira
  área sem enfraquecer o portão.
- **Onde se está.** No K = 2 (H2c), Ŝ ≠ ∅ em educação (STAR, sólido) e em saúde (RAND, só por saturação). O
  direito perdeu o Ŝ. O Sachs foi podado no F3 e o cinto no F2, ambos antes do ICP.
- **O que este K = 3 faz.** Três mudanças, cada uma declarada aqui antes de rodar e cada uma contada como K = 3 no
  domínio afetado. O critério do G3 **não muda**: a frase de `portoes.py` fica igual, letra por letra.
- **O que já se sabe e pode contaminar as escolhas.** Declaro o que conheço dos resultados anteriores, porque as
  escolhas abaixo foram feitas sabendo disso:
  - o resultado K = 1 e K = 2 dos três domínios que chegaram ao F4 (README do H2b e do H2c);
  - o ICP diagnóstico do H2b no Sachs (K = 1, depois da poda): rejeita tudo, maior p = 1e-211;
  - o ICP diagnóstico do H2b no cinto (K = 1, depois da poda): aceita só {idade21, idade_media, vel65, vel70},
    p = 0,054, sem o uso do cinto;
  - o F2 do cinto no H2b: EP por estado 0,140 contra dp da priori 0,133; EP ingênuo 0,057; efeito de desenho 6,1;
  - o F3 do Sachs no H2b: z = 3,76 contra q95 do nulo 5,20 (p = 0,10) com 11 condições; efeito de desenho 67.

  Nenhuma das escolhas abaixo foi feita olhando o ICP do K = 2 nesses dois domínios, que não existe (o ICP não
  rodou lá no H2c), nem rodando qualquer alternativa.

## (a) Regra de saturação (todos os domínios)

- **Princípio.** Se o ambiente E é uma função determinística das colunas de S, então Y ⊥ E | X_S vale por
  construção: condicionado a X_S, E é constante. Um teste de invariância nesse S tem poder zero contra qualquer
  alternativa, e um teste com poder zero não pode certificar invariância. Aceitar um S saturado não é evidência;
  rejeitá-lo também não seria informativo sobre invariância (só sobre a forma do ajuste).
- **A regra.** Conjuntos saturados são **excluídos da família**: não contam como aceitos nem como rejeitados. Ŝ é a
  interseção dos conjuntos aceitos **não saturados**. Se nenhum conjunto não saturado for aceito, o ICP é tratado
  como "nada aceito" (`modelo_rejeitado = true`) e Ŝ = ∅. A lógica do ICP no resto não muda: α = 0,05, todos os
  2^p subconjuntos (p ≤ 10), o teste K = 2 de resíduos (KW + Brown–Forsythe, Bonferroni) sem nenhuma mudança.
- **Como "E é função determinística de X_S" é medido (substitui o diagnóstico do K = 2).**
  1. As colunas de S são as do bloco que o funil passa ao ICP (numéricos padronizados, categóricos em indicadoras),
     cada coluna reescalada para dp 1 no treino; colunas constantes saem.
  2. Cada linha recebe um escore de acerto do vizinho mais próximo, deixando-a de fora (1-NN LOO):
     - se há outras linhas com exatamente os mesmos valores de S (uma "célula" com m ≥ 2 linhas), o escore é a
       fração das outras m − 1 linhas da célula que estão no mesmo ambiente (empates resolvidos exatamente);
     - se a linha é única em S, o escore é 1 se a linha mais próxima (euclidiana, `scipy.spatial.cKDTree`) está no
       mesmo ambiente, e 0 se não.
     - S = ∅: uma célula com todas as linhas.
  3. **S é saturado se o acerto médio ≥ 0,99** (o mesmo limiar do diagnóstico do K = 2).
  - **Por que esta forma.** Ela cobre X discreto (a célula pura do K = 2: a coparticipação do RAND, a turma × série
    do STAR) e também X contínuo que separa os ambientes em faixas (um valor de plano, um degrau de incentivo). Com
    X contínuo que se sobrepõe entre ambientes, o vizinho mais próximo erra o ambiente com frequência e S não é
    saturado.
  - **Limite declarado.** Com 1-NN, ambientes quase separáveis em X_S (acerto ≥ 99% sem ser exatamente
    determinístico) também contam como saturados. É deliberado: nesse caso o teste também tem poder quase nulo.
- **Relato.** Por domínio: número de aceitos saturados, aceitos não saturados, saturados testados, e o Ŝ que a
  lógica do K = 2 daria (interseção de todos os aceitos, saturados inclusos), para comparação.
- **Teste de unidade.** Antes dos dados reais: (i) X discreto que determina E é saturado; (ii) X contínuo com
  faixas disjuntas por ambiente é saturado; (iii) X contínuo sobreposto e X discreto independente de E não são;
  (iv) num SCM em que o único conjunto aceito contendo o pai é saturado, Ŝ não o devolve; (v) os testes de
  cobertura do H2c continuam passando.
- **Efeito esperado, dito antes.** Pode remover o Ŝ do RAND (todos os 512 aceitos do K = 2 eram saturados no
  diagnóstico antigo). Será relatado como sair.
- **K.** Conta como K = 3 nos 5 domínios.

## (b) biologia_sachs: a célula vira a unidade

- **Mudança.** A coluna `unidade` passa a ser a célula (condição + índice da linha), não a condição.
- **Justificativa.** As intervenções são aplicadas por condição e as células são as réplicas; é assim que as
  análises clássicas de ICP desses dados tratam a célula (Meinshausen, Hauser, Mooij, Peters, Versteeg & Bühlmann
  2016, PNAS 113:7361, "Methods for causal inference from gene perturbation experiments and validation" — citada de
  memória, conferir; e Peters, Bühlmann & Meinshausen 2016, JRSS-B — citada de memória, conferir). No ICP a unidade
  que importa é a linha, e o ICP já usa a célula; a mudança alinha F2/F3 com isso.
- **O que muda, dito claramente.** Muda o F2 (EP agrupado por célula, quase o EP robusto) e o F3 (o nulo de
  permutação troca o pmek entre células, não entre condições), e por isso o Sachs deve chegar ao F4. O peso da
  verossimilhança no F4 (1/efeito de desenho) passa a usar o efeito de desenho por célula, perto de 1. **Esta é uma
  mudança que afrouxa a dependência dentro da condição**; ela foi apontada no README do H2b como "escolha depois de
  ver a falha" e é feita aqui como tal, declarada.
- **Ressalva obrigatória.** O efeito de desenho no nível da condição (67 no H2b) será relatado ao lado: as células
  de uma condição dividem tubo, coloração e corrida, e a margem do F3 e a posterior ficam otimistas. O G3 só olha
  o ICP, cuja garantia (Ŝ ⊆ pa(Y) com prob. ≥ 1 − α) supõe linhas independentes dentro do ambiente; a dependência
  dentro da condição é dependência dentro do ambiente, porque ambiente = condição. Fica declarado como limite.
- **Nada mais muda.** Y (log p44/42), os 10 candidatos, `ajuste`, DAG, ambientes (12 condições, sem as 6 e 13),
  cofre (14_cd3cd28icam2_ly), priori, decisão e sementes ficam como no H2b.
- **K.** O Sachs passa a K = 3 (K = 2 foi a troca do teste; K = 3 junta (a) e (b)).

## (c) transporte_cinto: efeito fixo de estado (transformação "within") no lugar da linha de base de 2 anos

- **O diagnóstico conhecido.** O F2 podou o cinto porque o EP agrupado por estado (0,140) passou o dp da priori
  (0,133). O EP ingênuo é 0,057: o que infla o EP é o efeito de desenho 6,1, ou seja, resíduos muito correlacionados
  dentro do estado.
- **A causa provável.** O Y do H2b é o log da mortalidade menos a média do próprio estado em 1983–1984. Essa linha
  de base usa só 2 anos, e em estados pequenos tem ruído de Poisson grande; esse ruído entra como uma constante
  por estado em todos os anos, que é exatamente um componente de resíduo comum ao estado. Além disso, a linha de
  base só tira o nível do Y, não o dos candidatos, e o confundimento estado-nível (opinião sobre segurança,
  estradas, frota) fica nos X.
- **A correção declarada (uma só).** Efeito fixo de estado pela transformação within, como no desenho de painel
  com efeitos fixos de estado de Cohen & Einav (2003), REStat 85:828 (citada de memória, conferir):
  - para cada estado, médias calculadas **só nas linhas de treino** (regimes `no` e `secondary`, com uso do cinto
    observado) de log(mortes por milha), uso do cinto, ano, log renda, idade média, vel65, vel70, idade21,
    alcool008 e log das milhas;
  - y = log(mortes por milha) − média de treino do estado; cada candidato = valor − média de treino do estado;
    `trafego_log` passa a ser log das milhas − média de treino do estado;
  - as linhas do cofre (`primary`) são centradas com as médias de treino do próprio estado, nunca com linhas do
    cofre; linhas do cofre de estados sem nenhuma linha de treino saem (o número será relatado).
- **Por que esta e não outra.**
  - **Unidade.** Trocar o estado por estado-ano como unidade ignoraria a correlação serial; seria afrouxar o F2,
    não corrigir. Rejeitada.
  - **Cofre.** Trocar o cofre não ataca o efeito de desenho. `no` no cofre quebraria a linha de base (que usa anos
    `no`) e `secondary` no cofre deixaria 201 linhas de treino. Rejeitada.
  - A transformação within ataca a causa diagnosticada (componente comum ao estado) e é a especificação padrão de
    painel para essa pergunta.
- **O que pode acontecer.** O EP pode subir ou cair: tirar o componente de estado reduz o efeito de desenho, mas
  usar só a variação dentro do estado reduz a variação do uso do cinto. Não rodei para saber. Se o F2 ainda podar,
  é o resultado.
- **Nada mais muda.** Candidatos (os mesmos 9, agora centrados no estado), `ajuste`, DAG, ambientes (`no`,
  `secondary` no treino; `primary` no cofre), unidade (estado), priori, decisão e sementes. A pergunta e a priori
  continuam sobre a variação do log da mortalidade por unidade de uso.
- **Valores binários centrados.** vel65, vel70, idade21 e alcool008 centrados no estado deixam de ser 0/1; a
  expansão do K = 2 os trata pelo número de valores distintos, sem nenhuma regra nova.
- **K.** O cinto passa a K = 3.

## (d) Conjunto adicional: nenhum

- **Decisão.** Não adiciono um sexto conjunto. Os reservas de `../dados-reais/h2b/FONTES.md`:
  - `AER_Fatalities` (Transporte, mesma área do cinto; leis em pacote e confundidas com o ano: médio/alto);
  - `AER_CigarettesSW` (n = 96; impostos vêm com campanhas antitabagismo: médio) e `Ecdat_Cigar` (sem imposto: alto);
  - `poe5_njmin3` (não tem o salário, o X intervindo);
  - `causaldata_thornton_hiv` (Saúde, área já contada; o ambiente é o próprio X, saturado por construção);
  - `causaldata_nsw_mixtape` (o tratamento age em Y por construção);
  - Sachs discretizado (mesma fonte do Sachs).
- Nenhum tem intervenção limpa em X numa área nova. Rodar um deles seria procurar área, não testar invariância.

## O que não muda

- O teste K = 2, seus parâmetros, os filtros F1–F4, os critérios G1–G5 e a frase do G3.
- Os pacotes de direito, STAR e RAND mudam só em `tentativas_de_especificacao: 3` e no histórico (a regra (a)).
- `portoes.py` ganha só uma nota no cabeçalho.

## Depois de rodar

Nada será ajustado depois de ver os resultados do K = 3. Qualquer mudança posterior é K = 4 e será relatada como
proposta, não executada.
