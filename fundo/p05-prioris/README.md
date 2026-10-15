# Ponto 5: as prioris simuladas (H2, H2g e H1 real)

## Resultado

**N_b = 5 e N_c = 2, sobre os 33 domínios (12 do H2 + 18 do H2g + 3 do H1 real).** Pela regra fixada na
[DECLARACAO.md](DECLARACAO.md), esse é o ramo "**dependiam das prioris simuladas nos domínios que mudam em (b)**":
cinco domínios mudam de status ou de ação do F8 com a priori de literatura, dois com a priori fraca, e as mudanças não
são cosméticas — uma reprovação vira poda e uma reprovação vira aprovação. Nenhuma mudança teve como único motivo um
portão computacional (R-hat, ESS, SBC): `so_portao_computacional` é falso nas sete.

**Ressalva que vale para tudo abaixo: o portão da rodada (a) foi reprovado pelo critério escrito, então estes números
saem rotulados como não interpretados — o veredito é condicional a essa leitura.** Campo a campo, nos 33 domínios,
contra os `saida/` do repositório: **0 diferem em categoria de status, em lista de portões que falham ou em ação do
F8**, e **24 dos 24 que têm média posterior dos dois lados diferem nela** — da 9ª casa decimal à 3ª significativa
(relativa de 4,1e-9 a 2,2e-3, mediana 5,8e-7; em unidades do dp posterior da rodada (a), no máximo 0,0071). O
`comparar.py` reprova **25 dos 33**: os 24 da média posterior mais o `h1-real/c`, cuja razão dp posterior / dp priori
difere em 1,7e-9 — quase toda a folga do limiar de 1e-9, e sem nenhuma mudança de ação. A diferença observada é
**numérica, não categórica**, e a causa está na seção seguinte.

A declaração escreve a condição de falha **sem tolerância** ("a cópia não reproduzir a rodada original (status ou
posterior diferentes dos `saida/` originais em (a))"). Por letra, ela dispara, e as rodadas (b) e (c) **não são
interpretadas**. Não foi mexido na declaração nem no hash: afrouxar a tolerância depois de ver o resultado seria ajuste
post-hoc, e pela regra de casa isso não é proibido — entra como declaração nova, com hash próprio (`DECLARACAO_2.md`)
e contando no K. Isso não foi feito aqui.

### O portão da rodada (a): o que falhou e por quê

| verificação | resultado |
|---|---|
| categoria de status, portões que falham e ação do F8, 33 domínios | **33/33 idênticos** aos `saida/` do repositório |
| posterior da chave (`comparar.py`, tolerância 1e-9) | **25/33 reprovados**: 24 pela média posterior (relativa de 4,1e-9 a 2,2e-3, mediana 5,8e-7) e o `h1-real/c` pela razão dp posterior / dp priori (1,7e-9) |
| domínios com média posterior dos dois lados que diferem | **24 de 24** (a diferença é numérica, não categórica) |
| desvio em unidades do dp posterior da rodada (a) | **≤ 0,0071 dp** (pior caso: `h2g/genetica_y07`) |
| quantidades determinísticas do F2/F3 (`beta_chave_mv`, `z_obs`, `nulo_q95`, ...) | diferem na 15ª casa (`~1e-15` relativo) |
| cópias contra os originais (`diff -r`) | só o gancho `$P05_PRIORI` (no-op com tabela vazia) e o caminho dos dados do H1 real |

O desvio é do ambiente numérico desta máquina contra o que gerou os `saida/` comitados (numpy 2.4.6, scipy-openblas
0.3.31), e não das cópias em `fundo/p05-prioris/`. A prova é o controle: o `h2g/` **original** copiado para fora da
árvore do p05 e rodado com a semente da varredura deu 25,210701694357784 no posterior da chave da farmacologia, o nosso
número, e não o gravado, 25,21079859117877. Rodando duas vezes aqui, o resultado sai igual ao bit (mesma semente, mesma
máquina). Para dimensionar: o desvio ambiental máximo é 0,0071 dp, e o menor deslocamento que mudou uma decisão é
0,18 dp — vinte e cinco vezes maior.

### As cinco mudanças de (b), a priori de literatura

| domínio | status (a) → (b) | ação do F8 (a) → (b) | desloc. (b) em dp da (a) | fonte de (b) |
|---|---|---|---|---|
| `h2/desenvolvimento_preston` | aprovado → aprovado | adotar → **manter** | 0,48 | Pritchett & Summers (1996), elasticidade-renda da expectativa de vida — **independente** |
| `h2/direito_prisoes` | reprovado no F4 (modelo_vence_climatologia) → **podado no F1** (EVPI < custo) | — → — | — (podado antes do F4) | Kochel, Wilson & Mastrofski (2011), efeito da raça na prisão — **independente** |
| `h2/educacao_star` | aprovado → **podado no F2** (a unidade não informa mais que a priori) | adotar → — | — | Krueger (1999), 4 pontos de percentil no 1º ano — **mesma base** (`AER_STAR.csv`) |
| `h2g/educacao_star_turmas` | aprovado → **podado no F1** (EVPI < custo) | manter → — | — | Krueger (1999) — **mesma base** (`AER_STAR.csv`) |
| `h1-real/c` | reprovado (modelo_vence_climatologia) → **aprovado** | 0,4 → **0,0** | — (o `comparar.py` não calcula deslocamento no H1) | **sem fonte conferida**: entrou a fraca Beta(1,1) |

Leitura de cada uma:

- **`h2/desenvolvimento_preston`** é a única mudança que aparece nas duas rodadas: P(adotar compensa) cai de 0,565
  (a) para 0,366 em (b) e 0,495 em (c), cruzando o limiar do F8. A fonte de (b) é independente da base do domínio, mas
  a fraca muda a ação igual, então o que essa mudança mostra é que **a ação do F8 daquele domínio é frágil a qualquer
  priori informativa**, não à de literatura em particular.
- **`h2/direito_prisoes`** e **`h2g/educacao_star_turmas`** saem no F1: a priori de literatura é bem mais estreita que
  o painel simulado (dp 0,105 contra 0,283 no primeiro; 0,050 contra 0,089 no segundo) e o EVPI cai abaixo do custo da
  pesquisa. Nos dois o status deixa de existir antes do F4.
- **`h2/educacao_star`** para no F2: com a priori de literatura construída da **própria base** do domínio, o dado da
  unidade independente não informa mais que a priori. É o caso de dupla contagem que a declaração previa marcar, e é
  onde ela morde.
- **`h1-real/c`** é o projeto que o M15 reprovou no H1 real. Trocado o painel SHELF simulado por Beta(1,1), ele passa,
  e a decisão vai de 0,4 para 0,0. Como não há fonte conferida para os três projetos do H1 real, a entrada de (b) é a
  mesma de (c): das cinco mudanças de (b), esta não é uma mudança "de literatura".

Das cinco, **três vêm de fonte independente** (Preston, direito_prisoes, e a de h1-real/c que não tem fonte) e **duas de
fonte da mesma base** (as duas de educação, sobre `AER_STAR`). Em nenhuma delas o estado do domínio melhora com a
priori mais informativa: ou poda, ou inverte a favor de não adotar (Preston), ou — no H1 real — aprova o que o painel
simulado reprovava.

### As duas mudanças de (c), a priori fraca

| domínio | status (a) → (c) | ação do F8 (a) → (c) | desloc. (c) em dp da (a) |
|---|---|---|---|
| `h2/desenvolvimento_preston` | aprovado → aprovado | adotar → **manter** | 0,18 |
| `h1-real/c` | reprovado → **aprovado** | 0,4 → **0,0** | — |

Com dp dez vezes maior (e no H1 real Beta(1,1)), a priori fraca quase não mexe em nada: **31 dos 33 domínios ficam
iguais** em status e ação. Onde ela mexe é onde o dado é fraco contra a decisão, e as duas mudanças são as mesmas duas
que a literatura já tinha movido. Isso é o ramo que a declaração chamava de "não dependiam das prioris simuladas no
conteúdo, mas dependiam de haver alguma priori informativa".

### Deslocamento maior não é mudança

Dois domínios com deslocamento grande **não** mudaram nada, e é útil ver por quê: `h2g/direito_teto_beneficio` anda
**1,36 dp** em (b) (fonte da mesma base, Meyer, Viscusi & Durbin 1995) e continua aprovado com a ação `adotar`;
`h2/credito_hmda` anda **0,80 dp** em (b) e em (c) (sem fonte conferida, então a mesma tabela nas duas) e continua
aprovado com `adotar`. O deslocamento mede sensibilidade da posterior, não da decisão: o F8 decide por P(adotar
compensa) contra o limiar, e nesses dois a posterior se move sem cruzar o limiar.

O caso oposto, `h2g/fisica_tunel_luz`, quase não sente a troca (0,004 dp) porque a priori de literatura é a fraca e a
posterior já era dominada pelo dado (razão dp posterior / dp priori de 0,0084 na rodada (a)).

### O G3 do H2g

O ICP não usa a priori, então o G3 só podia mudar se um domínio fosse podado antes do F4. Um foi: a educação.

| rodada | domínios com Ŝ ≠ ∅ | áreas | G3 (Ŝ ≠ ∅ em pelo menos 3 áreas) |
|---|---|---|---|
| (a) original | 4: `educacao_star_turmas` (Educação), `fisica_tunel_luz` (Física), `genetica_y08` (Genética), `farmacologia_qt` (Farmacologia) | 4 | passa |
| (b) literatura | 3: Física, Genética, Farmacologia — `educacao_star_turmas` podado no F1 | 3 | **passa, sem margem** |
| (c) fraca | 4, os mesmos de (a) | 4 | passa |

**O G3 passa nas três rodadas, mas em (b) ele cai de 4 áreas para exatamente 3**, que é o mínimo do PLAN, porque a
priori de literatura da educação (mesma base do domínio) derruba a turma pequena no F1. O Ŝ de cada domínio que
sobrevive é o mesmo das três rodadas: `{c_dof, c_qui, c_ran}` na farmacologia, `{blue, green, red}` no túnel de luz,
`{plate, x1, x5, x9}` no y08 e `{anos_pequena_antes, turma_pequena}` na educação.

## O que foi feito

1. **A declaração veio antes.** A [DECLARACAO.md](DECLARACAO.md) foi escrita antes de qualquer execução do funil com
   as prioris alternativas, e o `DECLARACAO.sha256` (9 entradas: a declaração, `prioris.py` e as seis tabelas
   `prioris/*/*.yaml`) confere — `sha256sum -c` dá OK em todas; a declaração é `167fbd6a…`. Nenhum critério, semente,
   agrupamento ou regra R1–R3 foi mudado depois. O `h2g/DECLARACAO_H2g.sha256` confere o `a5d879b3…` citado no README
   do H2g (o arquivo tem uma linha com carimbo de data que o `sha256sum -c` não entende; o hash bate).
2. **Os dados foram conferidos com `sha256sum -c` contra o `SHA256SUMS` de cada pasta, antes de rodar.**

   | pasta / arquivo | resultado |
   |---|---|
   | `dados-reais/h2/` (12 arquivos) | **OK** |
   | `dados-reais/h2b/` (o H2g lê daqui) | **OK** |
   | `dados-reais/h2g/` (inclui `ecgcipa/adeg.csv`, `3ab7b3cd…`) | **OK** |
   | `dados-reais/h2e/causal_chamber_wt_walks_v1_regime_jumps_single.csv` | **OK** (`ae00d09a…`) |
   | `dados-reais/h2e/causal_chamber_lt_interventions_standard_v1.csv` | **não**: o arquivo presente tem `a80bc1a6…`, e a `SHA256SUMS` registra `99b75516…` — ver abaixo |
   | `dados-reais/h2f/Kemmeren.RData` | **OK** (`08e95f39…`) |
   | `dados-reais/h2f/kemmeren2014_obs_int.parquet` | **não**: `f152937f…` contra `0921bda4…` registrado; o desvio e a conferência do conteúdo estão em `dados-reais/h2f/CONVERSAO.md` (1741 × 6175, os blocos `obs` e `int` idênticos bit a bit ao `RData`) |
   | `dados-reais/h3b/nycflights23_flights.csv` | ausente — fora do escopo (o H3 não usa priori SHELF) |

   O desvio do `lt`: o arquivo é a concatenação dos 59 CSVs do zip de `lt_interventions_standard_v1`, na ordem do
   zip, com a coluna `experiment` acrescentada no fim com o nome do arquivo sem extensão — forma 68 000 × 47,
   `uniform_reference` com 10 000 linhas e os outros 58 com 1 000. Foi construído duas vezes de forma independente
   (aqui e pelo processo pai) e as duas deram `a80bc1a6…`. Os dois zips de origem conferem com os MD5 declarados nas
   `FONTES.md`; o que difere é o byte do derivado, o mesmo fenômeno do parquet do H2f (o Parquet carrega a versão do
   escritor nos metadados). Nenhum domínio deste ponto usou as 8 colunas derivadas para além do que o `preparo.py`
   já lia; nenhum número foi ajustado por causa disso.
3. **As três rodadas, na ordem declarada, a partir das cópias.** A rodada (a) rodou sozinha e primeiro; depois (b) e
   (c). `rodar.sh` (com `P05_H="h2 h1-real"`) para o H2 e o H1 real, `rodar_h2g.sh` para o H2g. A combinação usada
   no H2g foi a do `rodar_h2g.sh`: domínio a domínio, com a semente da varredura mantida (`SEMENTE + 100 × posição`),
   (b) só nos 4 domínios com fonte conferida e, nos outros 14, o resultado de (c) copiado para (b) — a cópia foi
   conferida aqui: nos 22 domínios em que a entrada de literatura é a fraca (14 do H2g, 5 do H2 e os 3 do H1 real) os
   quartis das duas tabelas são iguais e, nos resultados, a priori e a posterior de (b) e (c) são idênticas.
   O H2g **não** foi rodado pela varredura completa de 18 domínios dentro do `rodar.sh` (seriam três varreduras
   sequenciais de ~13 min); a via do `rodar_h2g.sh` é a que mantém a mesma semente e paraleliza.
4. **Saídas fora do repositório**, em `/home/silvano-neto/p05-saida/` (o repositório só recebeu este README, o
   `resultados.json` e a `tabela.md` gerados pelo `comparar.py`).
5. **A checagem de (a) contra os originais** foi feita com o `comparar.py` (tolerância de 1e-9) e campo a campo
   nos 33 domínios (`comparar_33.py`, que separa o que é categoria do que é momento); o diagnóstico do desvio tem o
   controle descrito acima (`/tmp/ctrl_h2g.py`, o `h2g/` original copiado para `/tmp/ctrl-h2g`).

## Tabelas

As rodadas completas estão em [`tabela.md`](tabela.md) (33 domínios: status nas três rodadas, ação do F8, média
posterior, razão dp posterior / dp priori e deslocamento em dp da rodada (a)), gerada pelo `comparar.py`. Os
agregados:

| rodada | domínios que mudam | dos que mudam, fonte independente | dos que mudam, mesma base | dos que mudam, sem fonte |
|---|---|---|---|---|
| (b) literatura | **5** | 2 (`h2/desenvolvimento_preston`, `h2/direito_prisoes`) | 2 (`h2/educacao_star`, `h2g/educacao_star_turmas`) | 1 (`h1-real/c`) |
| (c) fraca | **2** | 0 | 0 | 2 (`h2/desenvolvimento_preston`, `h1-real/c`) |

| domínios com fonte conferida | quantos | quais |
|---|---|---|
| fonte lida de fora da base do domínio | 6 | `h2/clima_tempestades`, `h2/desenvolvimento_preston`, `h2/agronomia_nitrogenio`, `h2/trabalho_curriculos`, `h2/direito_prisoes`, `h2g/farmacologia_qt` |
| fonte da mesma base do domínio (dupla contagem, marcada) | 5 | `h2/transito_alcool`, `h2/educacao_star`, `h2g/direito_teto_beneficio`, `h2g/educacao_star_turmas`, `h2g/transporte_cinto` |
| sem fonte conferida (a entrada de literatura é a fraca) | 22 | os 14 do H2g sem fonte, os 5 do H2 (`credito_hmda`, `epidemiologia_contracepcao`, `energia_vitoria`, `logistica_voos`, `saude_consultas`) e os 3 do H1 real |

## Limites

- **Não há especialista humano.** Nenhuma priori desta rodada é uma elicitação SHELF: as de literatura são leituras
  de fontes publicadas convertidas para a quantidade do domínio por três regras fixas (R1–R3), e as fracas são uma
  declaração de ignorância. Isto testa **se as conclusões do funil dependiam das prioris simuladas**; não substitui um
  painel de elicitação e não diz o que um painel de verdade teria produzido.
- **O portão da rodada (a) falhou no sentido escrito.** Categoria de status, lista de portões e ação do F8 batem em
  33/33; a média posterior difere em 24 dos 24 domínios que a têm, de 4,1e-9 a 2,2e-3 de diferença relativa (≤ 0,0071
  dp). O controle mostra que o motivo é o ambiente numérico, não as cópias. Como a condição de falha está escrita sem
  tolerância, por letra ela dispara e as rodadas (b) e (c) não são interpretadas: o veredito deste README é condicional
  a essa leitura. Mexer na tolerância depois de ver o resultado seria declaração nova com hash próprio
  (`DECLARACAO_2.md`, contando no K); não foi feito.
- **O `comparar.py` não mede o deslocamento no H1 real**: a forma do resultado daquele horizonte não traz a média
  posterior da chave (`post_media` é nulo), então as três linhas do H1 real ficam sem o número que a declaração pedia
  para todos os domínios. Ficou o que existe: a razão dp posterior / dp priori (0,87 → 0,072 no projeto c), a ação e o
  status.
- **14 dos 18 domínios do H2g não têm fonte conferida**, então a rodada (b) do H2g é, neles, a própria fraca (copiada,
  como manda o cabeçalho do `rodar_h2g.sh`). Todo o efeito de (b) no H2g vem de um domínio só: `educacao_star_turmas`.
- **Nenhuma das sete mudanças é de portão computacional**, mas isso não é um teste de robustez dos portões: R-hat, ESS
  e SBC são verificados dentro de cada execução e nenhum domínio foi podado por eles em nenhuma das rodadas.
- **O escopo de H2b–H2f não foi rerodado**: a declaração cobre os 33 domínios acima e justifica que as versões finais
  moram no H2g, com quartis, `taxa_base` e decisão idênticos. **O H3 fica fora**: o `estat.py` tem a função de priori
  SHELF, mas nenhum fluxo a chama.
- **Não rodou por falta de dado:** nada deste escopo. `dados-reais/h3b/nycflights23_flights.csv` está ausente e não é
  lido por nenhum domínio daqui; o `h2f` e o `h2e` derivados estão presentes com hash divergente do registrado, o que
  está reportado acima e em `dados-reais/h2*/CONVERSAO.md`.

## Como reproduzir

Tudo com `OMP_NUM_THREADS=1` e, fora do repositório, `OUT=/home/silvano-neto/p05-saida`:

```
export P05_DADOS=/home/silvano-neto/Documents/estocologia/dados-reais
export OUT=/home/silvano-neto/p05-saida
export OMP_NUM_THREADS=1
cd fundo/p05-prioris

# rodada (a), sozinha e primeiro (a cópia com a tabela vazia é o código original)
(cd h2      && P05_PRIORI="" ESTOCOLOGIA_SAIDA=$OUT/h2/original      ESTOCOLOGIA_DADOS=$P05_DADOS/h2  python3 p05_rodar.py)
(cd h1-real && P05_PRIORI="" ESTOCOLOGIA_SAIDA=$OUT/h1-real/original ESTOCOLOGIA_DADOS=$P05_DADOS     python3 p05_rodar.py)
(cd h2g && (cd dominios && ls -d */ | tr -d / | grep -v '^_') | xargs -P 3 -I{} bash -c \
   'P05_PRIORI="" ESTOCOLOGIA_SAIDA=$OUT/h2g/original ESTOCOLOGIA_DADOS=$P05_DADOS/h2b python3 p05_um.py {}')

# rodadas (b) e (c): o H2g pela via do rodar_h2g.sh, o resto pelo rodar.sh
P05_DADOS=$P05_DADOS P05_H="h2 h1-real" bash rodar.sh $OUT
P05_DADOS=$P05_DADOS bash rodar_h2g.sh $OUT 3

# comparação (grava resultados.json e tabela.md nesta pasta)
python3 comparar.py $OUT
```

Os `saida/` brutos das três rodadas, os `fim.txt`/`fim_h2g.txt` com o código de saída de cada domínio e os
`*-{original,literatura,fraca}.log` ficam em `$OUT`. As checagens deste README que não estão no `comparar.py`
ficam lá também: `checar_reproducao.py` e `comparar_33.py` (rodada (a) contra os `saida/` do repositório, campo a
campo nos 33 domínios), `drift_dp.py` (o desvio em unidades do dp posterior), `g3.py`/`g3b.py` (Ŝ ≠ ∅ por rodada),
`verif_fraca.py` (a cópia de (c) para (b) nos 22 domínios sem fonte) e `numeros.py`. No `/tmp` ficou o controle do desvio: `/tmp/ctrl-h2g` (o `h2g/` original
copiado) com `/tmp/ctrl_h2g.py`.

Tempo de parede nesta máquina (16 núcleos, 3–4 workers): 34 min no total, dos quais 8 min para a rodada (a) sozinha e
26 min para (b) e (c). O que domina são os domínios pesados com a priori fraca: `h2/logistica_voos`,
`h2g/fisica_tunel_luz`, `h2g/saude_rand`, `h2g/educacao_star_turmas` e `h2g/engenharia_tunel_vento`.
