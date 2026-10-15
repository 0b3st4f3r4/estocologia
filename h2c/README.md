# H2c: o portão do ICP com o teste de invariância K = 2 (não paramétrico)

**Saída do H2c: NÃO aprovada.** O G3 falha mais uma vez: Ŝ ≠ ∅ em 2 domínios de 2 áreas (Saúde e Educação), e o
critério pede 3. O direito, que tinha Ŝ no K = 1, ficou com Ŝ = ∅ no K = 2. No RAND, o Ŝ = {coparticipacao}
apareceu por saturação, não como evidência de invariância (ver abaixo). G1, G2, G4 e a trilha passam
(`saida/portoes.json`).

Este trabalho segue a recomendação de um cartão de decisão que **ainda espera sil**. Se sil escolher outro
caminho, `h2c/` inteiro pode ser apagado sem efeito em mais nada. `h2b/` segue intacto como registro do K = 1;
lá só entrou um arquivo novo, a declaração.

```
python3 calibracao.py               # calibração do teste em simulação (~30 s) → saida/calibracao.json
python3 rodar.py                    # funil nos 5 domínios (~7 min; o teste novo roda 1 024 subconjuntos por domínio)
python3 portoes.py --reproducao     # portões G1–G5, com segunda execução limpa (~8 min a mais)
python3 grafo.py                    # hipergrafo
```

A execução registrada em `saida/` rodou num diretório local, com `ESTOCOLOGIA_SAIDA=<local>`, e depois foi
copiada para cá. O código é o mesmo.

## Ordem dos passos (pré-registro)

1. **A declaração veio antes.** `../h2b/DECLARACAO_K2.md` foi escrita antes de qualquer execução, inclusive em
   simulação. Ela traz o teste exato, os parâmetros, o motivo, o K = 2 para os 5 domínios e o G3 sem mudança.
   - O SHA-256 e a hora UTC estão em `DECLARACAO_K2.sha256`: `ee97a497…a364d025`, 2026-10-14T21:29:59Z.
2. **Implementação.** O código foi escrito em `h2c/`, uma cópia do `h2b/`.
3. **Testes.** Calibração, poder e cobertura em simulação; todos passaram de primeira.
   - O teste não foi revisto, e não houve adendo à declaração.
4. **Só então os dados reais.** Depois vieram o funil e os portões. **Nada foi ajustado depois de ver os
   resultados do ICP nos dados reais.**

## O que mudou em relação ao H2b

- **Código.**
  - `estocologia/icp.py` ganhou `expandir`, `p_invariancia_np` e o diagnóstico `_saturado`.
  - `icp()` ganhou o parâmetro `teste`, com o padrão `"residuos_np"` na família gaussiana.
  - O teste K = 1 (`p_invariancia`) ficou intacto, para comparação e para a binomial.
  - `funil.py` só passa `teste="residuos_np"` de forma explícita.
- **O teste K = 2.** Ele segue o espírito do ICP não linear de Heinze-Deml, Peters & Meinshausen (2018),
  "Invariant causal prediction for nonlinear models", citado de memória, conferir. Para cada S:
  1. **Expansão de cada candidato**, com estatísticas do treino:
     - categórico: indicadoras;
     - numérico com ≤ 10 valores distintos: fator;
     - numérico contínuo: linear + spline cúbica restrita, com 4 nós nos quantis 5, 35, 65 e 95%.
  2. **Ajuste agrupado.** Um único MQO agrupado de Y em S, com todos os ambientes juntos. Dele sai o resíduo r.
  3. **Escala agrupada.** Um MQO de |r| na mesma base, com piso de 0,1 · média de |r|. O resíduo padronizado é
     u = r / ŝ.
  4. **Os dois testes.**
     - Kruskal–Wallis de u entre ambientes: locação.
     - Brown–Forsythe de u entre ambientes: Levene centrado na mediana, para a escala.
  5. **Combinação.** Bonferroni sobre os 2 testes. O Anderson–Darling não entra; o motivo está na declaração.
- **Lógica do ICP.** Não mudou: α = 0,05, os 1 024 subconjuntos e Ŝ = interseção dos aceitos.
- **Pacotes de domínio.** Mudaram só em dois pontos:
  - `tentativas_de_especificacao: 2`;
  - `historico_especificacoes`, com o K = 1 e o K = 2.

  Y, X, ambientes, cofre, DAG, ajuste, prioris, decisão e sementes são idênticos.
- **`portoes.py`.** Ganhou só uma nota no cabeçalho. A frase do G3 é idêntica.
- **Novos arquivos.** `calibracao.py` e `tests/test_k2.py`.

## Evidência de calibração (simulação, 200 sementes por cenário, α = 0,05)

Rejeição a 5%. Sob H0, a lei de Y dado X é a mesma em todos os ambientes; os 3 ambientes mudam só a
distribuição de X. Na coluna K = 1, X entra linear, como o K = 1 fazia nos dados reais. Os números estão em
`saida/calibracao.json`.

| cenário | n | K = 2 | K = 1 |
|---|---|---|---|
| H0: X discreto (4 níveis, resposta côncava), ruído t₃ | 4 500 | **0,030** | 0,915 |
| H0: X discreto, ruído log-normal centrado | 4 500 | **0,015** | 0,725 |
| H0: X discreto, ruído t₃ com escala que cresce com X | 4 500 | **0,030** | 1,000 |
| H0: X contínuo, resposta não linear (tanh), ruído t₃ | 4 500 | **0,015** | 0,920 |
| H0: massa em zero cuja proporção depende de X | 4 500 | **0,080** | 1,000 |
| H0: X discreto, t₃ | 15 000 | **0,050** | 0,995 |
| H0: X discreto, t₃ heterocedástico | 15 000 | **0,050** | 1,000 |
| H0: massa em zero que depende de X | 15 000 | **0,165** (acima do α) | 1,000 |
| H1: o ambiente desloca a locação de Y \| X (0; +0,15; −0,10) | 4 500 | **0,985** | 0,995 |
| H1: o ambiente multiplica a escala de Y \| X (1; 1,25; 1) | 4 500 | **0,995** | 0,995 |

- **Leitura.**
  - Sob H0 com X discreto, cauda pesada e n grande, o K = 2 fica em 0,015–0,05. O critério declarado era ≤ 0,09.
  - Nos mesmos cenários, o K = 1 rejeita de 72% a 100% das vezes. Isso confirma a suspeita: o instrumento K = 1
    estava mal especificado para esses dados.
  - O poder contra deslocamentos reais de Y | X fica perto de 1.
- **Limite que a calibração mostra.**
  - Quando a *forma* da distribuição de Y | X depende de X, e não só a locação e a escala, o resíduo padronizado
    ainda carrega X. Com n = 15 000, o K = 2 rejeita 16,5% sob H0.
  - Esse é o caso da massa em zero do RAND. O cenário não estava nos critérios declarados e é relatado, não
    corrigido. Corrigi-lo exigiria outro teste, e isso seria K = 3.
- **Testes em `tests/test_k2.py`** (17 testes no total com os do H2b, todos passando):
  - calibração H0 (discreto t₃, log-normal, heterocedástico e n = 15 000; contínuo não linear): ≤ 0,09;
  - poder em locação e escala: > 0,8;
  - cobertura por frequência como no H2, no SCM x1 → y → x2 com intervenções em x1 e x2: violações de
    Ŝ ⊆ pa(y) ≤ 2α em 60 sementes;
  - cobertura com pai discreto côncavo e ruído t₃: violações ≤ 2α, e Ŝ = {x} em mais de metade das sementes;
  - expansão e saturação.
- **O teste antigo também passa no código novo.** O `test_icp_contem_so_pais…` do H2 agora roda com o K = 2 e
  continua passando.

## Resultados por domínio: K = 1 contra K = 2

| domínio | área | chegou ao ICP? | K = 1: Ŝ (aceitos; maior p) | K = 2: Ŝ (aceitos; maior p) | K = 2: p de Ŝ sozinho | aceitos saturados (K = 2) | habilidade do invariante no cofre (K = 1 / K = 2) |
|---|---|---|---|---|---|---|---|
| `biologia_sachs` | Biologia | não: podado no F3 | — | — | — | — | — |
| `transporte_cinto` | Transporte | não: podado no F2 | — | — | — | — | — |
| `direito_teto_beneficio` | Direito | sim | **{beneficio_log}** (240/1024; 0,087) | **∅**, aceitos disjuntos (580/1024; 1,00 em {beneficio_log, industria, kentucky}) | {beneficio_log}: 0,047 (rejeitado por pouco) | 0/580 | 0,07 / 0,00 |
| `educacao_star_turmas` | Educação | sim | ∅, tudo rejeitado (0/1024; 0,0013 em {turma_pequena}) | **{turma_pequena}** (209/1024; 0,69) | 0,010 (sozinho é rejeitado) | 152/209 | 0,00 / 0,003 |
| `saude_rand` | Saúde | sim | ∅, tudo rejeitado (0/1024; 2,5e-14) | **{coparticipacao}** (512/1024; 1,00 em {coparticipacao, doencas_cronicas}) | 0,50 | **512/512** | 0,00 / 0,005 |

As outras saídas do F4 são idênticas às do K = 1, conferido campo a campo: posterior, cobertura conforme, portão
M15 e decisão do F8. O ICP não consome o gerador aleatório, e só o preditor "icp_invariante" e o hash do
pré-registro, que inclui Ŝ e K, mudam.

- **Status por domínio.** Igual ao do K = 1:
  - direito: adotar, P = 1,00;
  - RAND: adotar, P = 0,72;
  - STAR: manter, P = 0,47;
  - Sachs: podado no F3;
  - cinto: podado no F2.

### O que cada domínio diz

- **Saúde (RAND): Ŝ = {coparticipacao}, mas por construção.**
  - **Por que é saturado.** O ambiente é o regime de coparticipação, e a coparticipação é constante dentro de
    cada regime. Como fator, ela determina o ambiente.
  - **O que isso implica.** Todos os 512 conjuntos que contêm a coparticipação são saturados: Y ⊥ E | X_S vale
    trivialmente, e o teste não tem como rejeitar.
    - Os conjuntos sem a coparticipação são todos rejeitados; o vazio dá p = 9e-85.
    - Por isso Ŝ = {coparticipacao}.
  - **O que o resultado diz.** Com estes ambientes, a invariância da coparticipação não é testável. O Ŝ repousa
    inteiro na premissa do ICP de que o regime age em Y só pela coparticipação, e não pela franquia, pelo teto ou
    pelo atrito diferencial.
  - **O diagnóstico pré-declarado.** Ŝ só com os aceitos não saturados não existe, porque não há nenhum. O
    resultado conta no G3 pela letra do critério, mas eu não o leria como evidência.
- **Educação (STAR): Ŝ = {turma_pequena}.**
  - **Saturação.** É parcial: 152 dos 209 aceitos têm turma_pequena e serie juntos, que determinam o ambiente.
  - **A sensibilidade.** Com os 57 aceitos não saturados, Ŝ = {anos_pequena_antes, turma_pequena}, ainda não
    vazio e ainda com a turma. É o resultado mais sólido do lote.
  - **Ressalva.** {turma_pequena} sozinho é rejeitado (p = 0,010). O Ŝ vem da interseção de conjuntos maiores
    que contêm a turma.
  - **Fora do ambiente.** O preditor invariante quase não ganha da climatologia no cofre: habilidade de 0,003.
- **Direito: perdeu o Ŝ.**
  - **O que mudou.** No K = 1, {beneficio_log} passava por pouco (p = 0,087 no melhor conjunto). No K = 2, 113 dos
    580 aceitos não contêm o benefício, e a interseção fica vazia.
    - Exemplos: {alta_renda, kentucky} com p = 0,054 e {idade_log, kentucky}.
  - **Por que pode acontecer.** O benefício é uma função quase determinística do salário, do estado e do teto.
    Com a spline, outros conjuntos conseguem reproduzir a mesma lei agrupada.
  - **Leitura.** É a conclusão honesta do teste mais flexível: os dados não separam o benefício de seus
    determinantes com 95% de confiança. O K = 1 dava Ŝ porque a forma linear restringia as alternativas.
- **Sachs e cinto.**
  - Foram podados antes do ICP: o Sachs no F3, o cinto no F2. Nenhum dos dois filtros usa o teste de invariância.
  - **A troca não os afeta.** A ordem do funil não mudou, e o ICP continua só no F4.

## Portões

| portão | resultado |
|---|---|
| G1 simulabilidade e reprodução | PASSA: 17 testes; a segunda execução limpa é idêntica nos 5 domínios |
| G2 funil completo e pré-registro | PASSA: todo domínio termina com motivo, K = 2 declarado e pré-registro antes do cofre |
| **G3 ICP entre ambientes** | **FALHA: Ŝ ≠ ∅ em 2 domínios (educação, saúde), 2 áreas; exigido: 3 áreas** |
| G4 contenção de varredura | PASSA: 8 leituras, 0 SELECT *, cofre só pelo F3, banca toda recusada |
| Trilha do barramento | PASSA: 42 fichas, sem quebra |
| G5 (informativo) | O M15 aprovou 3/3 que chegaram ao F4; 5 pareceres de risco esperam a assinatura de sil |

## Limites

- **Saturação.** Com ambientes que fixam X (RAND, STAR), um X discreto codificado como fator determina o
  ambiente. A invariância vira intestável: aceitar não é evidência.
  - O critério do G3, pela letra, conta esses casos.
  - A saturação foi declarada antes como diagnóstico e não mexe no portão.
  - Se sil quiser excluir domínios saturados do G3, isso muda o critério, e a decisão é de sil.
- **Forma que depende de X.** O teste de resíduos só controla locação e escala. Com massa em zero que varia com X
  e n grande, ele rejeita cerca de 16% sob H0 em simulação.
  - A rejeição de conjuntos sem a coparticipação no RAND pode ter um pouco disso, mas o vazio tem p = 9e-85.
- **Sem interações.** O ajuste agrupado é aditivo. Uma interação verdadeira entre candidatos, invariante, pode
  ser lida como não invariância quando a distribuição dos candidatos muda entre ambientes.
- **A leitura de "ICP padrão" no G3.** Passei a ler "ICP padrão" como a lógica padrão do ICP com o teste K = 2.
  Essa leitura muda o instrumento, não o critério, e cabe a sil confirmar.
- **Os limites do H2b continuam valendo:** prioris citadas de memória, agrupamento ausente no RAND e no direito,
  dois domínios podados antes do ICP e a escala pequena.

## O que isto deixa para sil

- **O que a troca mostrou.** A troca de régua confirmou a suspeita: o K = 1 rejeitava de 72% a 100% sob H0 nesses
  regimes.
- **O que ela não resolveu.** O portão continua sem passar:
  - ganhou-se a educação, com um resultado razoavelmente sólido;
  - ganhou-se a saúde, mas só por saturação;
  - perdeu-se o direito.
- **Caminhos.** Nenhum deles foi tentado aqui:
  - **ambientes que não fixem X**, como intervenções que mudam a distribuição de X sem torná-la constante, para a
    invariância ser testável;
  - **mais domínios de mudança de regra** em outras áreas;
  - uma decisão explícita sobre **como tratar domínios saturados** no G3;
  - **qualquer mudança de teste daqui em diante**, como uma comparação por célula de X discreto contra a forma
    inteira, que será K = 3 e declarada.
