# Ambiente das frentes de fundo

## Resultado

As nove dependências das frentes de `fundo/` que estão instaladas nesta máquina foram conferidas uma a uma com
`python3 -c "import <mod>; print(<mod>.__version__)"` em **2026-10-15**, e são as do
[`requirements.txt`](requirements.txt): **numpy 2.4.6, pandas 3.0.5, scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0,
duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1 e hypothesis 6.165.7**. **Duas dependências não estão instaladas, e sem
elas uma frente inteira e um passo de outra não reproduzem aqui:** `statsmodels` (o gabarito do p01) e `pyreadr`
(um passo do p08, a conversão dos `.rda` de voos novos). Os comandos de instalação estão abaixo e ficaram
comentados no `requirements.txt`.

Interpretador: **Python 3.13.7** (`/home/silvano-neto/.pyenv/versions/3.13.7/bin/python3`, GCC 15.2.0), pip 25.2.
Nada foi instalado nesta frente: o que está escrito aqui é a foto do que a máquina tem.

## O que foi feito

**1. Conferência uma a uma, com a saída do comando.** A saída abaixo é literal; nenhuma versão foi escrita de
memória.

```
$ python3 -c "import numpy; print(numpy.__version__)"
  2.4.6
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
| numpy | 2.4.6 | 424 | toda a árvore: h0-real, h1, h1-real, h2–h2g, h3 e as frentes de `fundo/` |
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
  printf '%s ' "$m"; python3 -c "import $m; print($m.__version__)"
done
python3 -c "import statsmodels" || python3 -m pip install statsmodels
python3 -c "import pyreadr"     || python3 -m pip install pyreadr
```

A contagem de uso por arquivo sai de um `ast.walk` sobre todos os `.py` da árvore, somando `ast.Import` e
`ast.ImportFrom` pelo primeiro componente do módulo (imports relativos contam pelo módulo do pacote, e as cópias de
horizonte dentro de `fundo/p03-multiplicidade/copias/` contam como arquivos próprios).

**Regra de recursos desta máquina** (a mesma das rodadas das frentes): `export OMP_NUM_THREADS=1` e no máximo
**2 processos em paralelo** — a máquina é compartilhada.
