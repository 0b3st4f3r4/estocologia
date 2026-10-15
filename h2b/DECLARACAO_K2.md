# Declaração K = 2: troca do teste de invariância do ICP

Escrita em 2026-10-14T21:29Z (UTC), **antes** de qualquer execução do teste novo, em simulação ou em dados reais.
O SHA-256 deste arquivo e a hora UTC do registro ficam em `../h2c/DECLARACAO_K2.sha256`. Este é o único arquivo
novo em `h2b/`; o resto de `h2b/` fica como registro do K = 1.

## Situação

- **Decisão pendente.** Há um cartão de decisão com sil. Ele recomenda trocar o teste de invariância do ICP por um
  teste não paramétrico de resíduos.
- **Trabalho reversível.** O trabalho segue a recomendação enquanto sil decide. Ele fica todo em `../h2c/`, uma
  cópia do `h2b/`. Se sil escolher outro caminho, basta apagar `h2c/`.

## Motivo

- **O que falhou.** No H2b, o G3 falhou: só o direito deu Ŝ ≠ ∅. No RAND e no STAR, o ICP rejeitou todos os
  1 024 conjuntos.
- **A suspeita.** O teste K = 1 é o "approximate" linear gaussiano: F aninhado contra coeficientes por ambiente,
  mais Levene dos resíduos. Com n grande, ele rejeita qualquer desvio do modelo linear gaussiano.
  - Exemplos: X discreto com resposta não linear (a coparticipação do RAND), Y com massa em zero ou cauda pesada,
    variância que depende de X.
  - Por isso o teste rejeitaria mesmo quando a lei de Y dado X_S é invariante.
- **A conclusão.** A suspeita é que o instrumento está mal especificado, não que falte invariância. A troca muda
  o instrumento. Ela não muda o critério.

## O teste K = 2 (`p_invariancia_np`, família gaussiana)

Ele segue o espírito do ICP não linear de Heinze-Deml, Peters & Meinshausen (2018), "Invariant causal prediction
for nonlinear models", J. Causal Inference (citada de memória, conferir). A ideia é ajustar um único modelo
agrupado e testar se a distribuição dos resíduos é a mesma entre os ambientes.

Para cada subconjunto S de candidatos:

1. **Expansão de cada candidato.** Todas as estatísticas vêm só do treino: os dados passados ao ICP, sem o cofre.
   - **Categórico.** Mantém o bloco de indicadoras que o funil já monta: um nível de referência, o mais frequente.
   - **Numérico com ≤ 10 valores distintos no treino.** Vira fator, com uma indicadora por valor, exceto o mais
     frequente.
   - **Numérico com > 10 valores distintos.**
     - Entra como termo linear mais spline cúbica restrita (Harrell), com 4 nós nos quantis 5, 35, 65 e 95%.
       São 3 colunas.
     - Nós repetidos são removidos. Com menos de 3 nós distintos, fica só o termo linear.
   - **Sem interações** entre candidatos.
2. **Ajuste agrupado.** Um único MQO de Y em [1, expansões de S], com todos os ambientes de treino juntos.
   Dele sai o resíduo r.
3. **Escala agrupada.**
   - Um MQO de |r| na mesma matriz.
   - ŝ = max(ajuste, 0,1 · média de |r|).
   - O resíduo padronizado é u = r / ŝ. Com S = ∅, ŝ é constante.
   - **Por quê.** Sob invariância com variância que depende de X, a dispersão de r muda entre ambientes só porque a
     distribuição de X muda. A padronização agrupada remove esse efeito sem olhar o ambiente.
4. **Locação.** Kruskal–Wallis de u entre os ambientes de treino, que dá p_loc.
5. **Escala.** Brown–Forsythe de u entre os ambientes, que dá p_esc. É o Levene centrado na mediana,
   `scipy.stats.levene(center="median")`.
6. **Combinação.** Bonferroni: p_S = min(1, 2 · min(p_loc, p_esc)).
7. **Anderson–Darling k-amostras: não entra.**
   - O p do `scipy.stats.anderson_ksamp` é truncado em [0,001; 0,25].
   - Ele testaria a forma inteira. A hipótese do K = 1, que o K = 2 substitui, é de locação e escala.
   - Assim a combinação fica com 2 testes, como no K = 1.

Parâmetros fixos:

- **ICP.** α = 0,05 e todos os 2^p subconjuntos (p ≤ 10). Ŝ é a interseção dos conjuntos aceitos, e a lógica do
  ICP não muda.
- **Limiar de discreto.** 10 valores distintos.
- **Spline.** 4 nós em (0,05; 0,35; 0,65; 0,95).
- **Piso da escala.** 0,1.
- **Combinação.** Bonferroni sobre 2 testes.

Outras partes que não mudam:

- **Intervalos dos coeficientes.** Os intervalos relatados pelo ICP continuam vindo do ajuste agrupado na
  codificação linear do K = 1. São informativos e não entram no portão.
- **Família binomial.** Continua com a razão de verossimilhanças do K = 1. Nenhum dos 5 domínios a usa.

### Diagnóstico pré-declarado (não muda Ŝ nem o portão)

- **O que é "saturado".** Um conjunto aceito é saturado quando a parte discreta de S determina o ambiente: ≥ 99%
  das linhas do treino caem em células de níveis discretos de S que só aparecem em um ambiente.
- **Por que importa.** Nesse caso o ambiente é função de X_S, e Y ⊥ E | X_S vale por construção. O teste não tem
  como rejeitar a locação nem a escala.
  - A aceitação, então, não é evidência de invariância.
  - Ela só repousa na premissa do ICP de que o ambiente não age direto em Y.
- **O que será relatado.**
  - Por conjunto aceito: se é saturado.
  - Ŝ recalculado só com os aceitos não saturados, como sensibilidade.
  - Nenhum dos dois entra no G3.

## Critérios de calibração em simulação (antes de tocar os dados reais)

- **Validade sob H0.**
  - **Cenário.** X discreto, ruído de cauda pesada (t com 3 g.l. e log-normal), n grande (≥ 4 000), ambientes que
    mudam só a distribuição de X.
  - **Critério.** Taxa de rejeição a 5% ≤ 0,09 em ≥ 200 sementes.
- **Poder.** Quando o ambiente desloca Y | X, a rejeição tem de ficar > 0,8.
- **Cobertura por frequência, como no H2.**
  - **Cenário.** O mesmo SCM x1 → y → x2, com intervenções em x1 e x2.
  - **Critério.** A fração de sementes com Ŝ ⊄ pa(y) fica ≤ 2α em 60 sementes.
- **Se a calibração falhar.** O teste é revisto antes de qualquer contato com os dados reais, e a revisão entra
  aqui como adendo datado, com novo SHA-256. Depois de rodar os dados reais, qualquer mudança no teste é K = 3 e
  será declarada.

## Escopo da mudança

- **Versão K = 2 nos 5 domínios.** Valem `biologia_sachs`, `direito_teto_beneficio`, `educacao_star_turmas`,
  `saude_rand` e `transporte_cinto`.
- **Pacotes.** Cada pacote em `h2c/dominios/` muda só em dois pontos:
  - `tentativas_de_especificacao: 2`;
  - uma entrada em `historico_especificacoes` com a troca do teste.
- **Nada mais muda.** Y, X, ambientes, cofre, DAG, ajuste, prioris, decisão, sementes e os filtros F1–F3 seguem
  iguais.
- **O critério do G3 não muda.** A frase de `portoes.py` fica igual, letra por letra: "em pelo menos 3 domínios de
  áreas distintas, o ICP padrão aceita algum conjunto E devolve Ŝ ≠ ∅".
  - "ICP padrão" passa a ser lido como a lógica padrão do ICP (interseção dos aceitos, α = 0,05) com o teste K = 2.
  - Essa leitura cabe a sil confirmar.
- **Ordem do funil.** O ICP só roda no F4. Os domínios podados antes (Sachs no F3, cinto no F2) não são afetados
  pela troca do teste.
- **Sem ajuste depois de ver os dados.** O teste não será ajustado depois de ver os resultados do ICP nos dados
  reais. Qualquer mudança posterior é K = 3 e será declarada.
