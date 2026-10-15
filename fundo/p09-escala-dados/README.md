# fundo/p09: escala (ponto 9) e dados (ponto 10)

## Resultado

**Dados (ponto 10).** Entraram dois domínios novos de saúde no pacote padronizado do F7, com o funil do H2g sem
mudança: um ensaio sorteado de indometacina retal contra pancreatite pós-CPRE (602 pacientes, 4 centros,
2009 a 2011) e um painel país-mês de vacinação e mortes por COVID-19 (Our World in Data, 2021 e 2022, 132 países).
**Os dois foram podados no F3**: a chave não vence o nulo de permutação (p = 0,060 no ensaio, p = 0,28 no painel).
Pelo critério declarado, nenhum confirma a alegação. No ensaio, o sinal está do lado esperado (razão de chances de
0,54 no treino), mas sem o centro de Michigan sobram 438 pacientes e 43 eventos, e o funil não tem poder. Uma
análise exploratória declarada à parte (K = 2) mostra que a previsão do evento raro num centro novo tem resolução
baixa, como no H3: habilidade de Brier +0,033 (IC90 de +0,005 a +0,063), com a confiabilidade (0,025) pior que a
resolução (0,014), porque a taxa de eventos no centro novo (22%) é o dobro da do treino (9,8%).

**Escala (ponto 9).** O funil rodou em lote sobre 329 domínios gerados por regra declarada a partir do catálogo do
Rdatasets. Rodou até o fim em 327; 2 quebraram no armazém. O hipergrafo real passou de 12 797 nós (H2g) para
26 634 nós, 28 619 arestas de árvore, 26 795 pontes e 4 628 hiperarestas. **Nós honestos, com dado e artefato
simulável: 26 634 no total, mas só 6 494 são nós de conceito** (domínio, camadas, variáveis, ambientes, filtros,
decisão); os outros 20 140 são hipóteses do ICP, uma por subconjunto testado, que crescem como 2^k e não são
conhecimento novo. E dos 6 494, só 554 vêm de pacotes curados; 5 940 vêm de pacotes automáticos sem semântica de
domínio (priori vaga, decisão sintética, DAG em estrela). **O gargalo não é CPU**: um domínio custa em média 3,8 s
de CPU. Os gargalos são, em ordem: (1) a curadoria semântica do F7, que não escala por regra; (2) a oferta de
conjuntos com ambientes identificáveis (9% do catálogo passa a regra); (3) as pontes do hipergrafo, que crescem com
o quadrado do número de domínios; (4) o hipergrafo inteiro num JSON em memória.

O número "1 538 nós no H2" da auditoria está desatualizado: o `grafo.py` do H2g, rodado sobre os resultados do
próprio H2g, já dá 12 797 nós, dos quais 12 288 (96%) são hipóteses do ICP. Isso mostra o risco que o brief aponta:
contar nós infla fácil.

## Ordem dos passos (pré-registro)

1. `DECLARACAO.md` (SHA-256 `39e89e85…`, 2026-10-15T12:20:47Z, em `DECLARACAO.sha256`) foi escrita antes de
   rodar o funil, com as alegações, ambientes, cofres, critérios, a regra de seleção R1 e os hashes do código.
   Ela registra também o que eu já tinha visto: **o cofre do ensaio não é cego**, porque olhei por descuido a
   tabela centro × braço × desfecho antes de declarar.
2. Os dois domínios de saúde rodaram uma vez. Os dois pararam no F3.
3. `DECLARACAO_2.md` (hash em `DECLARACAO_2.sha256`) abriu a análise exploratória do evento raro, porque o diário
   ficou vazio com a poda. Conta como K = 2 só para essa pergunta e não reabre o funil.
4. A regra R1 gerou 329 domínios; o lote rodou uma vez, depois o lote de perfil (30 domínios, cProfile e
   tracemalloc), depois o hipergrafo.
5. Desvio de execução, não de análise: a primeira tentativa do lote foi interrompida porque a máquina era dividida
   com outros trabalhos (carga 17 em 4 núcleos) e cada processo abria várias threads de BLAS. Recomecei do zero
   com uma thread de BLAS por processo, 3 processos em vez de 4 e prazo de 1 800 s em vez dos 900 s declarados.
   Nenhum domínio passou de 900 s (o maior levou 166 s), então o prazo declarado não mudaria nada.

## Parte 1: dados novos de saúde

Os pacotes estão em `dominios/saude_indometacina_ercp/` e `dominios/saude_covid_vacina_paises/`. As fontes e os
hashes dos dados estão em `/mnt/project-files/estocologia/dados-reais/fundo/p09-escala-dados/FONTES.md` e
`SHA256SUMS`. O que foi procurado e não entrou: `NHANES` e `survival` (dados anteriores a 2020, sem ambiente com
intervenção); `medicaldata::covid_testing` (2020, mas a documentação diz que os dados foram "anonymized,
time-shifted, and permuted", então as relações entre colunas podem estar quebradas); `medicaldata::opt` (ensaio de
4 centros, mas não consegui conferir a literatura anterior para a priori).

### saude_indometacina_ercp (ensaio sorteado; intervenção conhecida)

- **Ambientes.** Indiana (413) e "outros" (Kentucky + Case Western, 25) no treino; Michigan (164) no cofre.
- **Priori.** Razão de chances com mediana 0,50 e quartis 0,36 e 0,70, a partir da meta-análise anterior ao
  ensaio (Elmunzer et al. 2008: redução de 64%, IC95% 0,22 a 0,60), conferida em
  https://pmc.ncbi.nlm.nih.gov/articles/PMC3422786. Normal ajustada: média −0,69 e dp 0,49 em log-razão.
- **F1.** EVPI 0,0020 contra custo de pesquisa 0,001: passa.
- **F2.** EP da chave 0,33 por paciente, menor que o dp da priori (0,49): passa. β de máxima verossimilhança
  −0,62 (razão de chances 0,54).
- **F3.** z = −1,85 contra o quantil 95% do nulo de 1,96; p de permutação 0,060: **podado**.
- **Leitura.** O ensaio publicado tem p = 0,005 com os 602 pacientes (9,2% contra 16,9%, mesma fonte). Tirando o
  centro de origem para servir de cofre, o funil perde o poder. Isso é o custo de exigir um ambiente nunca visto
  num ensaio pequeno de evento raro, não uma evidência contra a droga.
- **Evento raro (exploratório, `escala/raro_indo.py`, `saida_saude/raro_indo.json`).** Logística em braço, risco,
  idade e sexo ajustada no treino, prevista em Michigan (164 pacientes, 36 eventos). Brier 0,180 contra 0,186 da
  climatologia; habilidade +0,033 (IC90 bootstrap +0,005 a +0,063). Murphy: confiabilidade 0,025, resolução
  0,014, incerteza 0,171. As probabilidades previstas ficam entre 0,03 e 0,22. Pela regra declarada (< 0,05),
  é **resolução baixa**, na mesma ordem do +0,019 do H3 em Rhode Island.

### saude_covid_vacina_paises (2021 e 2022; observacional)

- **Ambientes.** África, Ásia, Europa, América do Norte e Oceania no treino (2 927 países-mês, 122 países);
  América do Sul no cofre (240, 10 países).
- **Priori.** Inclinação em log(1 + mortes por milhão no mês seguinte) por unidade de cobertura completa, mediana
  −1,5, quartis −2,3 e −0,8, ancorada na efetividade contra morte de 96,7% de Haas et al. (2021), conferida em
  https://pmc.ncbi.nlm.nih.gov/articles/PMC8099312, e encolhida pela confusão ecológica.
- **F1.** EVPI 0,011 contra 0,005: passa.
- **F2.** Efeito de desenho 6,3 (os meses do mesmo país não são independentes); EP por país 0,34, menor que o dp
  da priori (1,11): passa. β de máxima verossimilhança −0,39.
- **F3.** z = −1,16; p de permutação 0,28: **podado**.
- **Leitura.** Ajustada por idade, renda, leitos, diabetes e mês, a associação ecológica entre cobertura e mortes
  do mês seguinte é fraca demais para vencer o nulo que embaralha as médias entre países e os desvios dentro de
  cada um. Não diz nada sobre a efetividade individual.

## Parte 2: escala do funil

### Regra R1 e o que ela deixa passar

A regra está em `DECLARACAO.md` e em `escala/gerar.py`. Do catálogo de 3 648 conjuntos, 837 passam os metadados
(linhas, colunas numéricas, ao menos uma coluna categórica). Desses, por passo da regra
(`escala/analise.json`, `selecao_R1`):

| Passo que descartou | Conjuntos |
|---|---|
| R1.1 download falhou | 20 |
| R1.2 menos de 4 colunas com nome permitido ou CSV ilegível | 43 |
| R1.3 nenhuma coluna de ambiente (3 a 8 níveis com ≥ 30 linhas) | 351 |
| R1.4 nenhum alvo numérico | 20 |
| R1.5 menos de 2 candidatos | 43 |
| R1.6 casos completos insuficientes | 12 |
| R1.8 duplicata de outro conjunto | 19 |
| **gerados** | **329** |

A falta de ambiente identificável é o maior filtro: 42% dos que passam os metadados. As 20 falhas de download são
um defeito meu: o `baixar.py` não codifica nomes de item com espaço e parênteses na URL.

A regra é mecânica e isso aparece no que ela escolhe. Uma amostra de 8 (semente 2027): `Ecdat::Garch` usa `date`
como chave; `fivethirtyeight::trump_approval_trend` prevê `approve_estimate` com `approve_high`, que é parte da
definição; `wooldridge::injury` acerta por acaso a mudança de política `afchnge` como chave. Nenhum desses pacotes
tem priori elicitada, decisão real ou DAG.

### Funil em lote (327 rodaram até o fim, 2 quebraram)

Números de `escala/analise.json` (`lote`), 3 processos em paralelo, máquina dividida com outros trabalhos:

| Etapa final | Domínios | Mediana de parede (s) |
|---|---|---|
| podado no F2 | 9 | 2,2 |
| podado no F3 | 125 | 2,4 |
| reprovado no F4 (M15) | 76 | 17,2 |
| aprovado no F4 | 117 | 17,3 |
| erro no armazém (Recusa) | 2 | não se aplica |

- **Tempo.** 2 049 s de parede para os 329. Por domínio: mediana 12,4 s de parede, máximo 166 s; **CPU média
  3,8 s** (soma 1 240 s). A correlação entre log(linhas) e log(segundos) é 0,46.
- **Memória.** Pico de RSS mediano 226 MB, máximo 486 MB por processo. No lote de perfil, o pico do tracemalloc
  (memória Python alocada) é mediano 2,8 MB e máximo 24,9 MB: o RSS é quase todo o custo fixo de importar numpy,
  scipy, pandas e DuckDB.
- **No F4 automático (193 domínios):** o ICP rejeita todos os conjuntos em 108 (56%), dá Ŝ = ∅ em 58 e Ŝ não
  vazio em 27. O portão M15 falha por cobertura conforme em 53 e por não vencer a climatologia em 36.
- **F1 não filtra nada nos automáticos.** Com priori N(0, 1) e custo zero, o EVPI é 0,40 contra 0,01. É a
  consequência direta de não haver decisão real: o filtro mais barato fica sem efeito.

### Onde quebra

1. **Armazém, EXPLAIN do DuckDB (2 domínios).** O armazém confere a projeção lendo o texto do EXPLAIN. O DuckDB
   quebra nomes de coluna longos em várias linhas (`x_pendingchargeattimeofoffense` aparece como
   `x_pendingchargeattimeofoffe` + `nse`), e o armazém recusa a leitura. É um defeito do código do H2
   (`estocologia/armazem.py`, linha 110 a 114): qualquer coluna com nome maior que cerca de 27 caracteres derruba o
   domínio. Não corrigi, como declarado.
2. **SBC domina o custo do F4.** No lote de perfil (cProfile, 30 domínios, 863 s perfilados), `_sbc` leva 515 s
   cumulativos (60%), dos quais quase tudo é o amostrador (`amostrar`, 670 s em 1 281 chamadas, somando as cadeias
   da posterior). O ICP leva 23 s porque k ≤ 6. Com k = 10, como nos pacotes curados, são 1 024 subconjuntos por
   domínio.
3. **Disco de rede.** O armazém grava Parquet por ambiente: `mkdir` levou 21,8 s em 327 chamadas no perfil. No
   `grafo.py`, 2 714 `stat` levaram 35,7 s (13 ms cada) e o YAML puro levou 12,3 s para 347 pacotes. O hipergrafo
   de 347 domínios levou 142 s (contra 2,5 s para 18), quase tudo E/S e YAML, não algoritmo.
4. **Pontes quadráticas.** O `grafo.py` liga todo par de domínios com o mesmo rótulo (mesmo status, mesmo achado).
   Com 347 domínios já são 26 795 pontes, cerca de 0,22·N². Mais domínios não acrescentam informação a essas
   pontes, só arestas.
5. **Hipergrafo em JSON único.** 15,6 MB para 26 634 nós (584 bytes por nó) e pico de 106 MB no tracemalloc
   (4 kB por nó) ao montar.

### Quantos nós e qual o gargalo em cada ordem

Base medida: um domínio automático acrescenta 18,2 nós de conceito e 42,2 nós no total; 3,8 s de CPU; pontes
≈ 0,22·N². As contas abaixo são **extrapolação linear desses números**, não medição.

| Meta | Domínios automáticos necessários (só nós de conceito) | CPU | Pontes | Gargalo |
|---|---|---|---|---|
| 10⁴ | cerca de 520 | 0,5 h | 6 × 10⁴ | a oferta: R1 tira 329 do Rdatasets inteiro; contando hipóteses do ICP, 10⁴ já foi passado |
| 10⁵ | cerca de 5 500 | 6 h | 7 × 10⁶ | não há catálogo alcançável (só GitHub e PyPI) com 17 vezes os conjuntos do Rdatasets; pontes |
| 10⁶ | cerca de 55 000 | 58 h | 7 × 10⁸ | pontes e grafo em memória quebram antes da CPU; a curadoria do F7 é impossível nessa escala |

- **Nós honestos alcançados: 26 634** com dado real e artefato simulável (pacote, preparo e CSV com hash). Destes,
  **6 494 são nós de conceito** e só **554 são de pacotes curados** (o H2g tinha 509; os dois de saúde trouxeram 45).
- **10⁴** é alcançável só contando hipóteses do ICP ou pacotes sem semântica. Com nós de conceito automáticos,
  faltam cerca de 190 domínios que a R1 não acha no Rdatasets.
- **10⁵ e 10⁶** não são problema de computação. O funil por domínio é barato e paralelizável. O que não escala é o
  F7: priori elicitada, decisão, DAG e escolha de ambiente com intervenção são trabalho humano por domínio. Um
  pacote gerado por regra roda o funil inteiro, mas o F1 fica sem efeito e o "conhecimento" do nó é nenhum.

## Como reproduzir

```
cd fundo/p09-escala-dados
D=/mnt/project-files/estocologia/dados-reais/fundo/p09-escala-dados
python3 -m pytest -q tests/test_p09.py                                      # 3 testes
python3 escala/um.py dominios/saude_covid_vacina_paises $D saida_saude 2027
python3 escala/um.py dominios/saude_indometacina_ercp  $D saida_saude 2127
python3 escala/raro_indo.py $D                                              # exploratório (DECLARACAO_2.md)
python3 escala/baixar.py $D                                                 # catálogo e CSV do Rdatasets
python3 -I escala/gerar.py $D $D/dominios_auto                              # regra R1
OMP_NUM_THREADS=1 python3 escala/lote.py $D/dominios_auto $D $D/saida_lote --trabalhadores 3 --prazo 1800
OMP_NUM_THREADS=1 python3 escala/lote.py $D/dominios_auto $D $D/saida_perfil --trabalhadores 1 --perfil --limite 30
python3 escala/analisar.py $D                                               # escala/analise.json e hipergrafos
```

`saida_base/` tem cópias dos `resultados.json` do `h2g/saida` (os originais não foram tocados).

**Segunda execução.** Os dois domínios de saúde e a análise do evento raro, rodados de novo numa pasta limpa,
deram resultados idênticos (fora o tempo). A regra R1 rodada de novo gerou pacotes idênticos. Lote: ver a seção
"Reprodução do lote" abaixo.

## Limites

- O cofre do ensaio não é cego (declarado). Como o domínio parou no F3, antes do cofre, isso não mexeu no funil,
  mas pesa na análise exploratória.
- Os tempos de parede foram medidos numa máquina dividida com outros trabalhos; o tempo de CPU é a medida firme.
  O perfil com cProfile e tracemalloc infla o tempo (25 s de CPU por domínio no lote de perfil contra 3,8 s sem).
- A unidade dos domínios automáticos é a linha, sem agrupamento: os EP e o F3 deles são otimistas onde houver
  painel ou grupos.
- Os 26 634 nós contam como "honestos" no sentido do brief (dado e artefato), não no sentido de conhecimento. A
  classe que importa para o PLAN é a dos 554 nós de conceito curados.
