# Ambiente das frentes de fundo

## Resultado

As nove dependências das frentes de `fundo/` que estão instaladas nesta máquina foram conferidas uma a uma com
`python3 -c "import <mod>; print(<mod>.__version__)"` em **2026-10-15**, e são as do
[`requirements.txt`](requirements.txt): **numpy 2.5.1, pandas 3.0.5, scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0,
duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1 e hypothesis 6.165.7**. **Duas dependências não estão instaladas, e sem
elas uma frente inteira e um passo de outra não reproduzem aqui:** `statsmodels` (o gabarito do p01) e `pyreadr`
(um passo do p08, a conversão dos `.rda` de voos novos). Os comandos de instalação estão abaixo e ficaram
comentados no `requirements.txt`.

Interpretador: **Python 3.13.7** (`/home/silvano-neto/.pyenv/versions/3.13.7/bin/python3`, GCC 15.2.0), pip 25.2.
Nada foi instalado nesta frente: o que está escrito aqui é a foto do que a máquina tem.

**O numpy tem dois exemplares no caminho, e é o único pacote nesta situação.** O **2.5.1** é o do `pyenv` e só
aparece com `PYTHONNOUSERSITE=1`; o **2.4.6** mora em `~/.local/lib/python3.13/site-packages` e vem antes no
`sys.path`, de modo que o `python3` padrão importa o 2.4.6. **O número de registro é o 2.5.1**, e o comando de
reprodução das rodadas é `PYTHONNOUSERSITE=1 python3 ...`. Os outros oito pacotes dão a mesma versão nos dois
caminhos (conferido em 2026-10-15, um a um). A consequência prática está na seção "O numpy do caminho" abaixo.

## O que foi feito

**1. Conferência uma a uma, com a saída do comando.** A saída abaixo é literal; nenhuma versão foi escrita de
memória.

```
$ python3 -c "import numpy; print(numpy.__version__)"
  2.4.6                       # o de ~/.local/lib/python3.13/site-packages, que vem antes no sys.path
$ PYTHONNOUSERSITE=1 python3 -c "import numpy; print(numpy.__version__)"
  2.5.1                       # o do pyenv: é o numpy de registro, e é dele que sai o G4 publicado do H3
$ python3 -c "import pandas; print(pandas.__version__)"
  3.0.5
$ python3 -c "import scipy; print(scipy.__version__)"
  1.18.0
$ python3 -c "import pyarrow; print(pyarrow.__version__)"
  26.0.0
$ python3 -c "import rdata; print(rdata.__version__)"
  1.1.0
$ python3 -c "import duckdb; print(duckdb.__version__)"
  1.5.5
$ python3 -c "import yaml; print(yaml.__version__)"          # PyYAML
  6.0.3
$ python3 -c "import pytest; print(pytest.__version__)"
  9.1.1
$ python3 -c "import hypothesis; print(hypothesis.__version__)"
  6.165.7
$ python3 -c "import statsmodels; print(statsmodels.__version__)"
  ModuleNotFoundError: No module named 'statsmodels'
$ python3 -c "import pyreadr; print(pyreadr.__version__)"
  ModuleNotFoundError: No module named 'pyreadr'
```

**2. Onde cada pacote é usado.** Contagem por AST (imports de topo e relativos) em todos os `.py` da árvore, no
mesmo dia:

| pacote | versão | arquivos | onde |
|---|---|---|---|
| numpy | 2.5.1 | 424 | toda a árvore: h0-real, h1, h1-real, h2–h2g, h3 e as frentes de `fundo/` |
| pandas | 3.0.5 | 298 | idem; a leitura dos CSV e dos Parquet |
| scipy | 1.18.0 | 152 | idem; `scipy.stats`, `scipy.spatial.cKDTree` (a regra 1-NN do ICP) |
| pyarrow | 26.0.0 | 23 | `estocologia/armazem.py` e as cópias: leitura e escrita de Parquet |
| duckdb | 1.5.5 | 26 | `estocologia/armazem.py` e as cópias: o armazém em SQL |
| rdata | 1.1.0 | 1 | `fundo/p07-icp/codigo/construir_parquet_h2f.py` (`Kemmeren.RData` → Parquet) |
| PyYAML | 6.0.3 | 40 | `estocologia/dominio.py`, os pacotes YAML de critérios e prioris, `juiz/notas.yaml` |
| pytest | 9.1.1 | 18 | as suítes `tests/` dos horizontes e das frentes |
| hypothesis | 6.165.7 | 50 | os testes de propriedade dos `conftest.py`/`tests` |

No `requirements.txt` o nome é o do pacote de importação (`numpy`), e o de distribuição é o mesmo em todos, menos
`PyYAML` — que se importa como `yaml` e imprime `6.0.3`.

**3. As duas ausências, e o que cada uma impede.**

| ausente | o que fica sem rodar | como instalá-la |
|---|---|---|
| `statsmodels` | o gabarito do p01, `fundo/p01-tese/gabarito/gerar_casos.py:16` (`import statsmodels.formula.api as smf`); o README do p01 já registra "Depende de numpy, pandas, scipy, statsmodels e PyYAML" | `python3 -m pip install statsmodels` |
| `pyreadr` | um passo do p08, `fundo/p08-calibracao/preparar_dados.py:22` — a conversão dos `.rda` de NY 2017 e PDX/SEA 2014 para CSV; o "Como reproduzir" do p08 lista `pyarrow`, `duckdb` e `pyreadr` | `python3 -m pip install pyreadr` |

**Divergência do enunciado, registrada.** O pedido desta tarefa dizia "`statsmodels` (p01 e alguns passos do p02)".
A conferência acha `statsmodels` em **um único arquivo de toda a árvore**, o gabarito do p01, e nenhum arquivo do
p02 o importa — o `fundo/p02-independencia/comum.py` traz uma logística de Newton própria (`logit_ajustar`). O que
está escrito acima é o que a árvore mostra.

## O numpy do caminho (2.5.1 contra 2.4.6)

O `Generator.binomial` do numpy 2.5 corrigiu dois erros independentes da série de Stirling do passo de
aceitação/rejeição do BTPE: o 3º e o 4º termos de erro eram **somados** em vez de subtraídos (erro herdado da seção
5.3 do artigo de 1988, Kachitvichyanukul & Schmeiser) e o coeficiente principal dizia `13680` onde devia dizer
`13860`. O BTPE é o caminho que o `binomial` usa quando `n·min(p, 1−p) > 30`. Com o **mesmo** estado do PCG64 o
sorteio muda de valor e consome outro número de palavras cruas:

```
np.random.default_rng(693).binomial(248, 0.6)      ->  171 no numpy 2.4.6, 143 no 2.5.1
```

No laço do G4 do H3 (`h3/estocologia/performativo.py`) isso acontece **uma vez em 9126 sorteios** — dia 16,
intervalo 55, com o mesmo estado do PCG64 nos dois: `binomial(248; 0,6)` dá **171** no 2.4.6 e **150** no 2.5.1.
Daí em diante as duas rodadas descorrelacionam e o G4 sai com **2375 choques e r̂ = 0,5632** no 2.5.1 (o publicado)
contra **2395 e r̂ = 0,5529** no 2.4.6. O bloco de r = 0,0 do laço não passa por essa repartição e é idêntico nos
dois numpys. O diagnóstico completo, com os hashes das duas instalações e a prova de reprodução das duas saídas,
está em [`../h3/DECLARACAO_G4_REGENERACAO_2.md`](../h3/DECLARACAO_G4_REGENERACAO_2.md).

**Outros números publicados que dependem do mesmo sorteio** (relatados, não corrigidos aqui): o
`fundo/p03-multiplicidade/laco_versoes.py`, que reimplementa o mesmo laço e publica r̂ = 0,563 com viés +1,7% (o
regime do 2.5.1; no 2.4.6 daria 0,553 e +2,2%), e as cópias idênticas do módulo em `fundo/p08-calibracao/codigo/` e
`fundo/p11-eventos-raros/codigo/h3/`.

**As frentes do H1 foram reconferidas nos dois numpys nesta rodada.** Rodando `h1/rodar.py` inteiro (projetos `a`,
`b` e `c`) com `OMP_NUM_THREADS=1` e a semente fixa de `h1/rodar.py:44` (2026, 2126 e 2226), o 2.4.6 e o 2.5.1
divergem em **5 campos no `a`, 7 no `b` e 59 no `c`**. As maiores diferenças relativas: 4,1e-4 na riqueza média da
estratégia de Kelly (`a`), 7,9e-4 nos quantis do modelo nulo (`b`) e **0,37 no xi² do SBC** (`f4.sbc.chi2`, `c`,
3,49 contra 2,21); no `c` muda também o `f3.cofre.hash_pre_registro`. **Os três projetos passam o portão nos dois
numpys**, e o **controle de repetibilidade** (duas rodadas do `c` no mesmo numpy e na mesma semente) difere em **1**
campo, só o tempo do pytest: o cano é determinístico dada a semente, então a diferença é do numpy e não do relógio.

Ajuste de leitura, sem corrigir nada: os números publicados de `h1/saida/` ficam **mais perto da rodada do 2.5.1**
do que da do 2.4.6 — 2 contra 6 campos diferentes no `a`, 35 contra 41 no `b` e 31 contra 62 no `c` — o que é
consistente com o ambiente de registro ser o 2.5.1 também aqui. Eles, no entanto, **não reproduzem bit a bit em
nenhum dos dois numpys** (o `hash_pre_registro` muda e sobra poeira de 1e-8 a 1e-9): isso é outra pergunta, de outra
natureza, e fica aberta.

As demais frentes que chamam `Generator.binomial` no regime do BTPE — `h1-real/projetos/{b_real,c_real}.py`,
`fundo/p05-prioris/h1-real/projetos/{b_real,c_real}.py` e o `evsi_beta_bernoulli` dos `estat.py` (chamadores em
`h1/projetos/c_cauda.py:483` e `h1-real/projetos/c_real.py:678`) — continuam **não** reconferidas nos dois numpys.
(`Generator.multinomial`, que o próprio numpy diz usar binomial por dentro, não aparece em nenhum arquivo da
árvore.)

## Limites

- **A foto é da máquina de hoje, não da pilha que gravou os derivados fora do Git.** O Parquet do h2f registra
  `parquet-cpp-arrow 26.0.0` nos próprios metadados (`created_by`, conferido), e o cabeçalho do
  `FORA_DO_GIT.reconstruido.sha256` traz a pilha inteira do escritor: parquet-cpp-arrow 26.0.0, pandas 3.0.5 e
  rdata 1.1.0. As versões conferidas aqui são as mesmas nas duas conversões; o que não dá para conferir é a pilha
  antiga que produziu as somas de referência.
- **Não é lock.** O `requirements.txt` é uma foto com `==`, não um arquivo de travamento: não fixa a versão do
  Python (3.13.7) nem as rodas, e nada garante que um `pip install` futuro resolva exatamente estas versões.
- **Não instalei nada.** As duas ausências continuam ausentes; por isso o gabarito do p01 e o passo de conversão do
  p08 não rodam nesta máquina hoje.
- **Fora da lista há dois pacotes instalados e usados na árvore:** `matplotlib 3.11.1` (figuras; por exemplo
  `h0-real/portoes.py`, `h3/figura.py` e as cópias dentro das frentes `fundo/p03-multiplicidade/copias/h3_ri32/figura.py`
  e `fundo/p05-prioris/h1-real/estocologia/agentes.py`) e `numba 0.65.0` (`h0-real/estocologia/central.py`). Os dois
  estão instalados e não entraram no `requirements.txt`, que segue a lista pedida das nove.
- **Nada aqui é resultado de frente.** Não há critério declarado, logo não há declaração com hash a registrar nesta
  frente: as versões não são um desvio de desenho nem um resultado que dependa de regra.

## Como reproduzir

A partir da raiz do repositório:

```
for m in numpy pandas scipy pyarrow rdata duckdb yaml pytest hypothesis; do
  printf '%s ' "$m"; PYTHONNOUSERSITE=1 python3 -c "import $m; print($m.__version__)"
done
python3 -c "import numpy; print(numpy.__version__)"   # 2.4.6: o de ~/.local, que o python3 padrão importa
python3 -c "import statsmodels" || python3 -m pip install statsmodels
python3 -c "import pyreadr"     || python3 -m pip install pyreadr
```

A contagem de uso por arquivo sai de um `ast.walk` sobre todos os `.py` da árvore, somando `ast.Import` e
`ast.ImportFrom` pelo primeiro componente do módulo (imports relativos contam pelo módulo do pacote, e as cópias de
horizonte dentro de `fundo/p03-multiplicidade/copias/` contam como arquivos próprios).

**Regra de recursos desta máquina** (a mesma das rodadas das frentes): `export OMP_NUM_THREADS=1` e no máximo
**2 processos em paralelo** — a máquina é compartilhada.
