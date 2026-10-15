# Declaração do ponto de fundo 2 (independência): reimplementação N-versões

Escrita em 2026-10-15, antes de escrever qualquer código desta pasta e antes de qualquer cálculo sobre os dados.
O SHA-256 deste arquivo fica em `DECLARACAO.sha256`.

## O que foi lido antes desta declaração (o "contrato")

Só documentos de especificação e cabeçalhos dos dados: `h1-real/README.md`, `h1-real/saida/b/datacontract.yaml`,
`h2b/README.md`, `h2b/DECLARACAO_K2.md`, `h2c/README.md`, `h2d/README.md`, `h2d/DECLARACAO_K3.md`,
`h2d/dominios/educacao_star_turmas/pacote.yaml`, `h2g/README.md`, `h2g/DECLARACAO_H2g.md`,
`h2g/dominios/farmacologia_qt/pacote.yaml`, `h3/README.md`, `h3/CRITERIOS.md`, `h3/CRITERIOS_H3d.md`, os
`FONTES.md` de `dados-reais/h2g` e `dados-reais/h3c`, a primeira linha de cada CSV usado e o dicionário de colunas do
ECGRDVQ. Nenhum `.py` de `h1-real/`, `h2*/`, `h3/` e nenhum JSON de saída foi aberto.

## Hipótese

A especificação publicada (README, declarações, critérios, pacotes YAML) basta para que uma equipe independente,
sem o código, chegue aos mesmos números que sustentam as saídas aprovadas, dentro das tolerâncias abaixo.

## Resultados, números publicados e tolerâncias

### R1. H1·B LaLonde

| quantidade | publicado | origem | tolerância de concordância |
|---|---|---|---|
| experimento, ΔR de emprego (y = re78 > 0) | +0,111 [0,026; 0,196] | `h1-real/README.md:58` | ±0,001 no ponto, ±0,005 nos limites |
| experimento, Δ renda 1978 | +1 794 | `h1-real/README.md:58` | ±US$ 2 |
| crua NSW tratados − CPS, ΔR | −0,107 [−0,170; −0,045] | `h1-real/README.md:56` | ±0,001 no ponto, ±0,01 nos limites |
| crua, Δ renda | −8 498 | `h1-real/README.md:56` | ±US$ 2 |
| escore de propensão, ΔR | +0,096 [−0,006; 0,208] | `h1-real/README.md:57` | ±0,03 no ponto; meu intervalo cobre o experimento |
| escore de propensão, Δ renda | +1 570 [218; 3 265] | `h1-real/README.md:57` | ±US$ 500 no ponto |
| controles efetivos | ≈ 103 | `h1-real/README.md:61` | ±30% |
| fração do CPS abaixo do menor escore dos tratados | 72% | `h1-real/README.md:61` | ±10 p.p. |

Para o experimento e a comparação crua, que são diferenças de médias, a tolerância é quase exata. Para o escore de
propensão, cuja especificação não está publicada, a tolerância é larga e a concordância de conclusão é: o ajuste
fica a até 0,03 do experimento (o README diz "acerta o ponto").

### R2. H2 Educação STAR (K = 3, teste K = 2 com regra de saturação)

| quantidade | publicado | origem | tolerância |
|---|---|---|---|
| Ŝ | {anos_pequena_antes, turma_pequena} | `h2d/README.md:76` | igualdade exata de conjuntos |
| aceitos não saturados / 1 024 | 57 | `h2d/README.md:76` | ±20% (46 a 68) |
| maior p não saturado | 0,075 em {anos_pequena_antes, menina, mestrado_prof, prof_negro, tipo_escola, turma_pequena} | `h2d/README.md:76`, `:99` | p em [0,05; 0,10]; o conjunto não precisa ser o mesmo |
| aceitos saturados / saturados testados | 152 / 256 | `h2d/README.md:76` | ±20% |
| p de {turma_pequena} sozinho | 0,010 | `h2d/README.md:99` | rejeitado (p < 0,05) |

### R3. H2g hERG (farmacologia_qt)

| quantidade | publicado | origem | tolerância |
|---|---|---|---|
| Ŝ | {c_dof, c_qui, c_ran} | `h2g/README.md:6`, `:35` | igualdade exata |
| aceitos / 512, saturados | 35, nenhum saturado | `h2g/README.md:33` | ±20% (28 a 42); zero saturados |
| inclinação do QTcF na dofetilida (posterior) | 25,2 ms por ng/mL, IC90 [23,0; 27,5] | `h2g/README.md:38` | ±2,0 no ponto e IC90 que se sobrepõe |

### R4. H3d Rhode Island (`prisoes_ri`, cofre 2013–2015)

| quantidade | publicado | origem | tolerância |
|---|---|---|---|
| n no cofre | 24 466 | `h3/README.md:227` | ±1% |
| ȳ no cofre | 2,26% | `h3/README.md:227` | ±0,05 p.p. |
| \|p̄ − ȳ\| oficial | 0,0020 | `h3/README.md:232` | passa ou falha nos dois limites igual ao publicado; ±0,003 |
| ECE oficial (10 caixas de massa igual) | 0,0048 | `h3/README.md:232` | passa ou falha nos dois limites igual ao publicado; ±0,003 |
| habilidade de Brier oficial | +0,019 | `h3/README.md:232` | mesmo sinal; ±0,01 |
| modelo cru: \|p̄ − ȳ\|, ECE, habilidade | 0,013; 0,013; +0,004 | `h3/README.md:233` | ±0,005; ±0,005; ±0,01 |
| Platt no fim | a = −0,30, b = 1,07 | `h3/README.md:236` | ±0,15 em cada |

A concordância principal de R4 é a decisão do critério: (a) e (b) de `h3/CRITERIOS_H3d.md:55-64` passam na minha
versão como passaram na publicada.

## O que conta como divergência

Qualquer quantidade fora da tolerância declarada acima. Cada divergência será classificada, só depois de congelar
minha versão e ler o código original, em uma de três causas: (i) erro meu; (ii) erro do original; (iii) escolha
não especificada no README ou nas declarações (a especificação não basta para reproduzir).

## Escolhas que eu faço onde a especificação cala (fixadas agora, antes de rodar)

Estas escolhas são minhas e não serão mudadas depois de ver resultados. Uma divergência causada por uma delas é,
por definição, do tipo (iii) se a escolha original também não estiver escrita.

- **R1.** Cofre e treino como no README: o escore é ajustado em NSW tratados (185) + CPS (15 992), sem os controles
  do NSW. Logit com as covariáveis de Dehejia & Wahba: idade, idade², educ, educ², casado, sem diploma, negro,
  hispânico, re74, re75, u74, u75, educ·re74 (re em milhares). Estimando: efeito nos tratados com pesos de chance
  e/(1 − e) no CPS. Intervalo: bootstrap percentil 95%, 2 000 reamostras estratificadas por grupo, semente
  20261015. Controles efetivos: (Σw)²/Σw².
- **R2.** Uma linha por aluno × série em que a turma é "small" ou "regular" (sem o braço com auxiliar) e há nota
  de leitura e de matemática. Y = leitura + matemática, padronizada por média e dp das turmas regulares da mesma
  série. anos_pequena_antes = número de séries anteriores em turma "small". almoco_gratuito = lunch "free".
  menina = gender "female". etnia = cauc, afam ou outra. mestrado_prof = grau do professor diferente de
  "bachelor". prof_negro = tethnicity "afam". tipo_escola = school. Linhas com algum candidato faltando saem.
  Ambientes: série × turma; cofre 3_small fora do ICP. Teste K = 2 e regra K = 3 exatamente como em
  `h2b/DECLARACAO_K2.md` e `h2d/DECLARACAO_K3.md`. Numéricos são padronizados no treino antes da expansão.
- **R3.** Como `h2g/DECLARACAO_H2g.md`: média do triplo por voluntário × braço × instante; Y = média do QTcF de cada
  ECG; placebo com todas as concentrações 0; dofetilida em pg/mL dividida por 1 000; pós-dose sem dosagem sai.
  A inclinação: MQO de Y em c_dof + ajuste (lineares) no treino, EP agrupado por voluntário, combinado com uma
  priori normal de mediana 28 e dp (36 − 20)/1,349, por conjugação normal-normal. IC90 da posterior.
- **R4.** Linhas sem `stop_outcome` saem. Features: sexo (M/F; faltante como nível próprio), idade padronizada no
  treino e o quadrado (faltante imputada pela média do treino), `violation_raw` em indicadoras, 4 faixas de hora
  (0–6, 6–12, 12–18, 18–24) e dia da semana. Logística com λ = 1e-4 sobre a log-perda média (intercepto livre),
  reajustada em 1º de janeiro com todos os anos anteriores. Platt online contínuo desde 2006-01-01, com o rótulo
  disponível 24 h depois da parada e atualizações aplicadas no início de cada tick de 6 h, em ordem de
  disponibilidade. Disjuntor: os parâmetros do fluxo `prisoes` não estão publicados, então **não implemento o
  disjuntor**; o reserva só entra fora do escopo. Climatologia móvel: média de todos os rótulos disponíveis no
  instante da emissão. ECE: previsões ordenadas, 10 caixas de massa igual, Σ (n_b/N)·|p̄_b − ȳ_b|.

## Regra

A versão é congelada (hash dos `.py` em `CONGELAMENTO.sha256`) antes de abrir o código original. Depois disso, o
código desta pasta não muda; correções de erro meu, se houver, entram como versão 2 separada e relatada.
