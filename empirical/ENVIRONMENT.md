# Environment of the background fronts

## Result

The nine dependencies of the fronts of `empirical/` that are installed on this machine were checked one by one with
`python3 -c "import <mod>; print(<mod>.__version__)"` on **2026-10-15**, and they are those of
[`requirements.txt`](requirements.txt): **numpy 2.5.1, pandas 3.0.5, scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0,
duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1, and hypothesis 6.165.7**. **Two dependencies are not installed, and without
them an entire front and one step of another do not reproduce here:** `statsmodels` (the answer key of p01) and `pyreadr`
(one step of p08, the conversion of the `.rda` of new flights). The installation commands are below and stayed
commented in `requirements.txt`.

Interpreter: **Python 3.13.7** (`~/.pyenv/versions/3.13.7/bin/python3`, GCC 15.2.0), pip 25.2.
Nothing was installed in this front: what is written here is the snapshot of what the machine has.

**numpy has two copies on the path, and it is the only package in this situation.** **2.5.1** is the one from `pyenv` and only
appears with `PYTHONNOUSERSITE=1`; **2.4.6** lives in `~/.local/lib/python3.13/site-packages` and comes earlier in
`sys.path`, so that the standard `python3` imports 2.4.6. **The record number is 2.5.1**, and the command of
reproduction of the rounds is `PYTHONNOUSERSITE=1 python3 ...`. The other eight packages give the same version on both
paths (checked on 2026-10-15, one by one). The practical consequence is in the section "The numpy on the path" below.

## What was done

**1. Checking one by one, with the output of the command.** The output below is literal; no version was written from
memory.

```
$ python3 -c "import numpy; print(numpy.__version__)"
  2.4.6                       # the one in ~/.local/lib/python3.13/site-packages, which comes earlier in sys.path
$ PYTHONNOUSERSITE=1 python3 -c "import numpy; print(numpy.__version__)"
  2.5.1                       # the one from pyenv: it is the numpy of record, and it is from it that the published G4 of H3 comes
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

**2. Where each package is used.** Count by AST (top-level and relative imports) in all the `.py` of the tree, on the
same day:

| package | version | files | where |
|---|---|---|---|
| numpy | 2.5.1 | 424 | the whole tree: h0-real, h1, h1-real, h2–h2g, h3, and the fronts of `empirical/` |
| pandas | 3.0.5 | 298 | idem; the reading of the CSV and of the Parquet |
| scipy | 1.18.0 | 152 | idem; `scipy.stats`, `scipy.spatial.cKDTree` (the 1-NN rule of ICP) |
| pyarrow | 26.0.0 | 23 | `estocologia/armazem.py` and the copies: reading and writing of Parquet |
| duckdb | 1.5.5 | 26 | `estocologia/armazem.py` and the copies: the warehouse in SQL |
| rdata | 1.1.0 | 1 | `empirical/p07-icp/codigo/construir_parquet_h2f.py` (`Kemmeren.RData` → Parquet) |
| PyYAML | 6.0.3 | 40 | `estocologia/dominio.py`, the YAML packages of criteria and priors, `juiz/notas.yaml` |
| pytest | 9.1.1 | 18 | the suites `tests/` of the horizons and of the fronts |
| hypothesis | 6.165.7 | 50 | the property tests of the `conftest.py`/`tests` |

In `requirements.txt` the name is that of the import package (`numpy`), and the distribution one is the same in all, except
`PyYAML` — which is imported as `yaml` and prints `6.0.3`.

**3. The two absences, and what each one prevents.**

| absent | what stops running | how to install it |
|---|---|---|
| `statsmodels` | the answer key of p01, `empirical/p01-tese/gabarito/gerar_casos.py:16` (`import statsmodels.formula.api as smf`); the README of p01 already records "It depends on numpy, pandas, scipy, statsmodels, and PyYAML" | `python3 -m pip install statsmodels` |
| `pyreadr` | one step of p08, `empirical/p08-calibracao/preparar_dados.py:22` — the conversion of the `.rda` of NY 2017 and PDX/SEA 2014 to CSV; the "How to reproduce" of p08 lists `pyarrow`, `duckdb`, and `pyreadr` | `python3 -m pip install pyreadr` |

**Divergence of the statement, recorded.** The request of this task said "`statsmodels` (p01 and some steps of p02)".
The checking finds `statsmodels` in **a single file of the whole tree**, the answer key of p01, and no file of
p02 imports it — the `empirical/p02-independencia/comum.py` carries its own Newton logistic (`logit_ajustar`). What
is written above is what the tree shows.

## The numpy on the path (2.5.1 against 2.4.6)

The `Generator.binomial` of numpy 2.5 fixed two independent errors of the Stirling series of the
acceptance/rejection step of BTPE: the 3rd and 4th error terms were **summed** instead of subtracted (an error inherited from section
5.3 of the 1988 paper, Kachitvichyanukul & Schmeiser) and the leading coefficient said `13680` where it should say
`13860`. BTPE is the path that `binomial` uses when `n·min(p, 1−p) > 30`. With the **same** state of PCG64 the
draw changes value and consumes another number of raw words:

```
np.random.default_rng(693).binomial(248, 0.6)      ->  171 in numpy 2.4.6, 143 in 2.5.1
```

In the loop of G4 of H3 (`h3/estocologia/performativo.py`) this happens **once in 9126 draws** — day 16,
interval 55, with the same state of PCG64 in both: `binomial(248; 0.6)` gives **171** in 2.4.6 and **150** in 2.5.1.
From there on the two rounds decorrelate and G4 comes out with **2375 shocks and r̂ = 0.5632** in 2.5.1 (the published one)
against **2395 and r̂ = 0.5529** in 2.4.6. The block of r = 0.0 of the loop does not pass through that split and is identical in
both numpys. The complete diagnosis, with the hashes of the two installations and the proof of reproduction of the two outputs,
is in [`../h3/DECLARACAO_G4_REGENERACAO_2.md`](../h3/DECLARACAO_G4_REGENERACAO_2.md).

**Other published numbers that depend on the same draw** (reported, not corrected here): the
`empirical/p03-multiplicidade/laco_versoes.py`, which reimplements the same loop and publishes r̂ = 0.563 with bias +1.7% (the
regime of 2.5.1; in 2.4.6 it would give 0.553 and +2.2%), and the identical copies of the module in `empirical/p08-calibracao/codigo/` and
`empirical/p11-eventos-raros/codigo/h3/`.

**The fronts of H1 were rechecked on both numpys in this round.** Running `h1/rodar.py` in full (projects `a`,
`b`, and `c`) with `OMP_NUM_THREADS=1` and the fixed seed of `h1/rodar.py:44` (2026, 2126, and 2226), 2.4.6 and 2.5.1
diverge in **5 fields in `a`, 7 in `b`, and 59 in `c`**. The largest relative differences: 4.1e-4 in the average wealth of
the Kelly strategy (`a`), 7.9e-4 in the quantiles of the null model (`b`), and **0.37 in the xi² of SBC** (`f4.sbc.chi2`, `c`,
3.49 against 2.21); in `c` the `f3.cofre.hash_pre_registro` also changes. **The three projects pass the gate in both
numpys**, and the **repeatability control** (two rounds of `c` on the same numpy and the same seed) differs in **1**
field, only the pytest time: the pipe is deterministic given the seed, so the difference is from numpy and not from the clock.

Reading adjustment, without correcting anything: the published numbers of `h1/saida/` stay **closer to the round of 2.5.1**
than to that of 2.4.6 — 2 against 6 different fields in `a`, 35 against 41 in `b`, and 31 against 62 in `c` — which is
consistent with the record environment being 2.5.1 here too. They, however, **do not reproduce bit by bit in
either of the two numpys** (the `hash_pre_registro` changes and dust of 1e-8 to 1e-9 remains): that is another question, of another
nature, and it stays open.

The other fronts that call `Generator.binomial` in the BTPE regime — `h1-real/projetos/{b_real,c_real}.py`,
`empirical/p05-prioris/h1-real/projetos/{b_real,c_real}.py`, and the `evsi_beta_bernoulli` of the `estat.py` (callers in
`h1/projetos/c_cauda.py:483` and `h1-real/projetos/c_real.py:678`) — remain **not** rechecked on both numpys.
(`Generator.multinomial`, which numpy itself says uses binomial internally, does not appear in any file of the
tree.)

## Limits

- **The snapshot is of today's machine, not of the stack that recorded the derivatives outside Git.** The Parquet of h2f records
  `parquet-cpp-arrow 26.0.0` in its own metadata (`created_by`, checked), and the header of
  `FORA_DO_GIT.reconstruido.sha256` carries the whole stack of the writer: parquet-cpp-arrow 26.0.0, pandas 3.0.5, and
  rdata 1.1.0. The versions checked here are the same in the two conversions; what cannot be checked is the
  old stack that produced the reference sums.
- **It is not a lock.** `requirements.txt` is a snapshot with `==`, not a lock file: it fixes neither the version of
  Python (3.13.7) nor the wheels, and nothing guarantees that a future `pip install` resolves exactly these versions.
- **I installed nothing.** The two absences remain absent; for that reason the answer key of p01 and the conversion step of
  p08 do not run on this machine today.
- **Outside the list there are two installed packages used in the tree:** `matplotlib 3.11.1` (figures; for example
  `h0-real/portoes.py`, `h3/figura.py`, and the copies inside the fronts `empirical/p03-multiplicidade/copias/h3_ri32/figura.py`
  and `empirical/p05-prioris/h1-real/estocologia/agentes.py`) and `numba 0.65.0` (`h0-real/estocologia/central.py`). The two
  are installed and did not enter `requirements.txt`, which follows the requested list of nine.
- **Nothing here is a front result.** There is no declared criterion, hence there is no declaration with hash to record in this
  front: the versions are neither a deviation of design nor a result that depends on a rule.

## How to reproduce

From the root of the repository:

```
for m in numpy pandas scipy pyarrow rdata duckdb yaml pytest hypothesis; do
  printf '%s ' "$m"; PYTHONNOUSERSITE=1 python3 -c "import $m; print($m.__version__)"
done
python3 -c "import numpy; print(numpy.__version__)"   # 2.4.6: the one in ~/.local, which the standard python3 imports
python3 -c "import statsmodels" || python3 -m pip install statsmodels
python3 -c "import pyreadr"     || python3 -m pip install pyreadr
```

The count of usage per file comes from an `ast.walk` over all the `.py` of the tree, summing `ast.Import` and
`ast.ImportFrom` by the first component of the module (relative imports count by the module of the package, and the copies of
horizon inside `empirical/p03-multiplicidade/copias/` count as files of their own).

**Resource rule of this machine** (the same as the rounds of the fronts): `export OMP_NUM_THREADS=1` and at most
**2 processes in parallel** — the machine is shared.
