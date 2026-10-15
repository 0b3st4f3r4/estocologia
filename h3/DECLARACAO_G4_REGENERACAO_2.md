# Declaração 2: o G4 publicado reproduz — era o numpy do caminho

**Data:** 2026-10-15T21:19:40Z. **Quem:** agente delegado da frente do G4/H3, ramo `claude/project-thread-7rrw8h`.
**Declaração escrita e conferida com hash (`.sha256`) antes de tocar no texto publicado desta rodada.**

## 0. O que esta declaração decide

A [`DECLARACAO_G4_REGENERACAO.md`](DECLARACAO_G4_REGENERACAO.md) (2026-10-15T20:37:11Z, SHA-256
`e0dc2898…`) trocou o número publicado do G4 pelo do numpy que o `python3` importa hoje (2.4.6). **Aquela
regeneração fica superada e nenhum número publicado muda.** O r̂ = 0,5632… com 2375 choques **não estava errado e
não era de código anterior**: ele reproduz no ambiente de registro. O que estava errado era a nossa documentação de
ambiente e a conclusão de que o publicado não reproduzia.

Restauração feita nesta rodada, antes de escrever isto (`git diff --stat` vazio para os dois):

| arquivo | conteúdo | SHA-256 |
|---|---|---|
| `h3/saida/portoes.json` | o de HEAD, sem nenhuma alteração | `32349c9da80ec93155ec68cb0681f86c38479d8ad787301323319bec6eaa4a6f` |
| `h3/README.md` | o de HEAD | `30ce9cc55012b7946d1b45a1b3205de047c059f84fca3763248eeb6cda35b4da` |

O `h3/README.md` recebe depois desta declaração só a nota de ambiente da §5 (texto exato lá), que muda o seu hash; o
hash final está no `.sha256` desta declaração e no relatório da rodada. O `DECLARACAO_G4_REGENERACAO.md` continua
no histórico como registro do que foi feito e por quê; nada nele é reescrito.

## 1. O diagnóstico

**(a) Não é código.** `h3/estocologia/performativo.py` tem uma versão só no git (o commit único de pesquisa
`4bb7181`), e as quatro cópias na árvore — `h3/`, `fundo/p03-multiplicidade/copias/h3_ri32/`,
`fundo/p08-calibracao/codigo/` e `fundo/p11-eventos-raros/codigo/h3/` — têm todas o mesmo SHA-256
`6bdfcb197b786f0e95ed6e74d7101e25daff597b46bb305a2b963eefc4f18cfc`. Não existe código anterior de onde o número
publicado pudesse ter saído.

**(b) Dois numpys no mesmo interpretador (Python 3.13.7, o do `pyenv`).**

| | 2.5.1 — o do registro | 2.4.6 |
|---|---|---|
| caminho | `/home/silvano-neto/.pyenv/versions/3.13.7/lib/python3.13/site-packages/numpy` | `/home/silvano-neto/.local/lib/python3.13/site-packages/numpy` |
| como se importa | `PYTHONNOUSERSITE=1 python3` | `python3` (o site do usuário vem antes no `sys.path` e sombreia o do `pyenv`) |
| `_core/_multiarray_umath.cpython-313-x86_64-linux-gnu.so` | 10 678 145 B, SHA-256 `fca8e1e6dd525c32c497f30ac554d0230d6c4915e6c2eace27d9790739798d9b` | 10 452 641 B, SHA-256 `d527de761a83209d571d666d215696d9c9540acc5c4e96753b1dca59694516fa` |
| `numpy-<v>.dist-info/METADATA` | SHA-256 `6ae45122ee97050e48849438320430d05f01814f72e66e69cbeed027d2c6a1e8` | SHA-256 `b082c52ccbe3b880bae177c35171b52ed6c947f3ad71e3762f2723f60a411de0` |
| `numpy-<v>.dist-info/RECORD` | SHA-256 `1299abe399d8237bd4861ced263352ea2887071d0c93a4414a6698e53d14d271` | SHA-256 `c76b7579ede1d36af3d0ed51b373fe171de2a4577130bd9e0d548882612fc304` |

**O numpy é o único pacote que difere entre os dois caminhos.** Conferido hoje, um a um: pandas 3.0.5, scipy 1.18.0,
pyarrow 26.0.0, rdata 1.1.0, duckdb 1.5.5, PyYAML 6.0.3, pytest 9.1.1 e hypothesis 6.165.7 dão a mesma versão com e
sem `PYTHONNOUSERSITE=1` (`statsmodels` e `pyreadr` continuam ausentes nos dois).

**(c) O MRE do `binomial`, no estado exato do laço.** Rastreando as chamadas de `simular` (r = 0,6, ingênuo,
semente 2027) nos primeiros 18 dias, as duas versões fazem 9126 chamadas idênticas até a de índice 8278. Na 8279, com
o **mesmo estado do PCG64** (o par `state`/`inc` é igual nos dois, conferido), `binomial(248; 0,6)` devolve **171**
no 2.4.6 e **150** no 2.5.1. Replay do estado gravado, com a contagem de palavras cruas consumidas:

```
$ python3 - <<'EOF'
import numpy as np
st = {"bit_generator": "PCG64", "has_uint32": 0, "uinteger": 0,
      "state": {"state": 98450282421402197031551936096562614758,
                "inc": 120608724771122158120179303604584792733}}
g = np.random.Generator(np.random.PCG64()); g.bit_generator.state = st
h = np.random.Generator(np.random.PCG64()); h.bit_generator.state = st
v = int(g.binomial(248, 0.6))
nxt = int(g.bit_generator.random_raw(1)[0])
raw = [int(x) for x in h.bit_generator.random_raw(8)]
print("binomial(248, 0.6) =", v, "palavras consumidas =", raw.index(nxt))
EOF
2.4.6  ->  binomial(248, 0.6) = 171 palavras consumidas = 2
2.5.1  ->  binomial(248, 0.6) = 150 palavras consumidas = 4
```

MRE mínimo, só com semente (sem estado): `np.random.default_rng(693).binomial(248, 0.6)` dá **171** no 2.4.6 e
**143** no 2.5.1. Percorrendo 3000 estados do PCG64 (sementes 0–2999, primeiro sorteio), o par (248; 0,6) diverge em
**1** deles; os pares (248; 0,5), (408; 0,6), (539; 0,765), (1000; 0,6), (10000; 0,5) e outros dez conferidos não
divergiram em nenhum. A divergência é rara e depende do estado — não é deslocamento sistemático do fluxo.

**Correção de detalhe à declaração 1.** Lá está escrito que o 2.4.6 "consome duas palavras a mais". Medido agora
pelo índice da próxima palavra crua a partir do mesmo estado, é o contrário: o 2.4.6 sai do sorteio com o fluxo em
+2 palavras e o 2.5.1 em +4 (uma volta a mais do laço de aceitação). O que decide é justamente o teste de aceitação
corrigido, descrito em (d).

**(d) A causa raiz, que é mais forte que "os dois sorteiam diferente".** Duas correções independentes na série de
Stirling do passo de aceitação/rejeição do BTPE, que é o algoritmo que o `Generator.binomial` usa quando
`n·min(p, 1−p) > 30`:

```
$ diff -u distributions.c (v2.4.6) (v2.5.1)      # única mudança do arquivo, 18 linhas
@@ -741,15 +741,21 @@
+  /* Note that the third and fourth error terms are subtracted. This is a correction from the
+     original 1988 paper (Kachitvichyanukul & Schmeiser) which erroneously adds all four terms */
   if (A > (xm * log(f1 / x1) + (n - m + 0.5) * log(z / w) + (y - m) * log(w * r / (x1 * q)) +
-           (13680. - (462. - (132. - (99. - 140. / f2) / f2) / f2) / f2) / f1 / 166320. +
-           (13680. - ... / z2) / z / 166320. +
-           (13680. - ... / x2) / x1 / 166320. +
-           (13680. - ... / w2) / w / 166320.)) {
+           (13860. - ... / f2) / f1 / 166320. +
+           (13860. - ... / z2) / z / 166320. -
+           (13860. - ... / x2) / x1 / 166320. -
+           (13860. - ... / w2) / w / 166320.)) {
```

As notas de release do numpy 2.5.0 registram as duas: o 3º e o 4º termos de erro eram **somados** em vez de
subtraídos (erro herdado da seção 5.3 do artigo de 1988) e o coeficiente principal tinha `13680` onde devia estar
`13860` (troca de dígitos da implementação inicial) — "as a result, `Generator.binomial` and
`Generator.multinomial` … may now return different samples for the same seed". O `RandomState.binomial` legado não
muda: mantém o comportamento original.

**O 2.5.1 é o corrigido.** O número publicado veio do sorteio corrigido: com `q = min(p, 1−p) = 0,4` e `n = 248`,
a média é 148,8 — o 150 do 2.5.1 está em cima dela, enquanto o 171 do 2.4.6 fica a ~3,7 desvios. O valor que
descorrelaciona o laço é o do teste de aceitação errado.

**(e) Como isso vira 2375 contra 2395.** Do índice 8279 em diante as duas simulações descorrelacionam (a chamada
8280 já sai com `n` e `p` diferentes) e o sorteio de exploração — 10% dos intervalos, escala × 0,7 ou × 1,3 — muda
de padrão: **2383 choques no bloco r = 0,0 (igual nos dois)** e, no bloco r = 0,6, 2375 no 2.5.1 contra 2395 no
2.4.6. O caminho de r = 0,0 não passa pela repartição da rediscagem (`red = rng.binomial(ab, r) if r > 0 else 0`),
e é por isso que o bloco fica insensível: `coef_choque` concorda até 1e-15 nos dois numpys.

## 2. A prova de reprodução

Rodei o caminho que produz o G4 — a mesma chamada do `portoes.py`, `P.rodar(dados-reais/bank_calls.csv)`, com
`OMP_NUM_THREADS=1` e a árvore do HEAD:

```
export OMP_NUM_THREADS=1
cd /home/silvano-neto/Documents/estocologia/h3
PYTHONNOUSERSITE=1 python3 -c "from pathlib import Path; from estocologia import performativo as P; \
  d=P.rodar(Path('../dados-reais/bank_calls.csv'))['r=0.6']['deteccao_M11']; print(repr(d['r_hat']), d['n_choques'])"
python3 -c "from pathlib import Path; from estocologia import performativo as P; \
  d=P.rodar(Path('../dados-reais/bank_calls.csv'))['r=0.6']['deteccao_M11']; print(repr(d['r_hat']), d['n_choques'])"
```

| bloco `r=0.6` de `deteccao_M11` | **numpy 2.5.1** (`PYTHONNOUSERSITE=1`) | numpy 2.4.6 (`python3`) | publicado (HEAD) |
|---|---|---|---|
| `r_hat` | 0,5632133268561668 | 0,5528868864293444 | 0,5632133268561669 |
| `n_choques` | **2375** | **2395** | **2375** |
| `coef_choque` | −0,1687860154332624 | −0,16203089278056845 | −0,1687860154332624 |
| `t` | −6,436797453264871 | −6,18352906613737 | −6,4367974532648793 |
| `p` | 1,4716134367353879e-10 | 7,348473503449291e-10 | 1,471613436735315e-10 |
| IC 90% | 0,5054256740255859 – 0,6098441503477998 | 0,49764604356801223 – 0,6010526342040088 | 0,5054256740255859 – 0,6098441503477998 |
| viés do controlado | +1,7306% | +2,2015% | +1,7306% |

No bloco r = 0,0 os dois numpys dão o mesmo: `n_choques` 2383, `r_hat` 0,0 e `coef_choque` igual até 1e-15.

**O que bate bit a bit no 2.5.1:** `n_choques` = 2375, `coef_choque` = −0,1687860154332624 e os quatro números de
`ingenuo`/`controlado` (viés +29,1247%, +1,0353%, 2934,6474 e 15142,1346; viés +1,7306%, 2930,9447 e 15094,75). O
`r_hat` fica **1 ulp** abaixo do publicado e o `t`, o `p` e a ponta inferior do IC diferem no 15º–16º algarismo —
o mesmo resíduo de `lstsq` que a declaração 1 já tinha atribuído ao BLAS, não ao gerador. Fechando o círculo: com
`OMP_NUM_THREADS=4` e o 2.5.1 o `r_hat` sai **0,5632133268561669**, idêntico ao publicado, e o viés do controlado sai
0,017306254796738196, também idêntico. Ou seja, o bloco publicado é o deste módulo no numpy 2.5.1 com BLAS de 4
threads.

## 3. Ambiente de registro do H3

- **Interpretador:** Python 3.13.7 (`/home/silvano-neto/.pyenv/versions/3.13.7/bin/python3`, GCC 15.2.0).
- **numpy 2.5.1**, o do `pyenv`, obtido com `PYTHONNOUSERSITE=1` — é esse o numpy do registro do H3, e é dele que
  sai o G4 publicado.
- Com o `python3` padrão entra o **numpy 2.4.6** de `~/.local/lib/python3.13/site-packages`, que vem antes no
  `sys.path` e sombreia o 2.5.1. A divergência do G4 era só isso.
- Demais dependências (iguais nos dois caminhos): pandas 3.0.5, scipy 1.18.0, pyarrow 26.0.0, rdata 1.1.0, duckdb
  1.5.5, PyYAML 6.0.3, pytest 9.1.1, hypothesis 6.165.7.
- `export OMP_NUM_THREADS=1` (regra de recursos da máquina) e no máximo 2 processos.
- **Comando de reprodução:** `PYTHONNOUSERSITE=1 python3 ...`.
- `fundo/AMBIENTE.md` e `fundo/requirements.txt` foram corrigidos nesta rodada: o número de registro passa a ser
  numpy 2.5.1 e o comando de reprodução leva `PYTHONNOUSERSITE=1`.

## 4. Outros números publicados que dependem disto (relatados; não corrigidos)

- **`fundo/p03-multiplicidade`.** O `laco_versoes.py` reimplementa o mesmo laço e o README publica r̂ = 0,563 com
  viés +1,7% para a v4 — o regime do 2.5.1 (o 2.4.6 daria 0,553 e +2,2%). É um número publicado que depende do
  mesmo sorteio, e o README de lá já registra a diferença de 16º algarismo. Fica como está.
- **`fundo/p08-calibracao/codigo/` e `fundo/p11-eventos-raros/codigo/h3/`.** Têm cópias do mesmo módulo (mesmo
  SHA-256); qualquer número reimpresso do laço nesses dois segue a mesma regra.
- **Frentes cujo cano chama `Generator.binomial` no regime do BTPE** (`n·min(p, 1−p) > 30`), onde a correção do
  2.5 morde: `h1/projetos/{a_multiplicativo,b_causal,c_cauda}.py`, `h1-real/projetos/{b_real,c_real}.py`,
  `fundo/p05-prioris/h1-real/projetos/{b_real,c_real}.py` e `estat.evsi_beta_bernoulli`
  (`h3/estocologia/estat.py:189`, com chamadores em `h1/projetos/c_cauda.py:483` e
  `h1-real/projetos/c_real.py:678`). `Generator.multinomial`, que o próprio numpy diz usar binomial por dentro,
  **não é usado em nenhum arquivo da árvore**. Nenhum desses foi reconferido aqui: o teste que fecha é rodar cada
  frente nos dois numpys e comparar a saída, e isso é decisão do dono do repositório.
- **O que não depende:** o G1–G3, G5, G7 e a trilha do `portoes.json` do H3. A regeneração da declaração 1 rodou
  o `portoes.py` inteiro no 2.4.6 e mexeu **só** no bloco `laco_performativo` do G4 — o resto do arquivo
  reproduziu bit a bit.

## 5. O que muda no texto publicado

- `h3/README.md`: volta ao de HEAD e ganha, no fim da seção **G4**, esta nota (única alteração):

> **Registro de ambiente (2026-10-15).** O r̂ de 0,56 e os 2375 choques acima são do **numpy 2.5.1** do `pyenv`, o
> interpretador sem o site do usuário no caminho: `PYTHONNOUSERSITE=1 python3`. Com o `python3` padrão, que importa
> o **numpy 2.4.6** de `~/.local`, o mesmo código dá r̂ = 0,5529 e 2395 choques. A troca não é de código — o
> `estocologia/performativo.py` tem uma versão só — e é do gerador: com o mesmo estado do PCG64,
> `binomial(248; 0,6)` devolve **171** no 2.4.6 e **150** no 2.5.1, porque o numpy 2.5 corrigiu a série de Stirling
> do teste de aceitação do BTPE (o 3º e o 4º termos eram somados, e o coeficiente principal dizia `13680` em vez de
> `13860`). O número publicado reproduz no 2.5.1 e fica como está. Diagnóstico, hashes e prova:
> [`DECLARACAO_G4_REGENERACAO_2.md`](DECLARACAO_G4_REGENERACAO_2.md).

- `fundo/AMBIENTE.md` e `fundo/requirements.txt`: o registro do numpy passa a 2.5.1, com o aviso do 2.4.6 do
  `~/.local` e o comando de reprodução com `PYTHONNOUSERSITE=1`.
- `fundo/README.md`: a nota que registrava a divergência do G4 como **ponto aberto** passa a dizer que ela está
  resolvida, com o MRE.
- **Não tocados:** `artigo/README.md`, `estocologia-livro-didatico.md`, `fundo/p03-multiplicidade/**` e
  `h3/saida/**/resultados.json`. Como o número publicado não muda, o texto que o cita continua certo. Achei uma
  citação a conferir e nenhuma a corrigir: o `fundo/p03-multiplicidade/README.md` publica r̂ = 0,563 (o regime do
  2.5.1) e registra a diferença de 16º algarismo contra `laco_versoes.py` — é o item 1 da §4.

## 6. Limites

- **Do 15º–16º algarismo do bloco publicado.** Com `OMP_NUM_THREADS=1` o `r_hat` do 2.5.1 fica 1 ulp abaixo do
  publicado; com 4 threads sai idêntico. Em que contagem de threads a rodada original saiu não está registrado em
  lugar nenhum, e não dá para reconstruir.
- **A taxa de divergência é do estado, não do par.** Nos 3000 estados que varri, só (248; 0,6) divergiu (1 caso); um
  par que não divergiu numa varredura curta não é prova de insensibilidade. O único teste que fecha é o A/B do §4.
- **O que não foi feito:** não reconferi as frentes do §4 de ponta a ponta nos dois numpys, não regenerei o
  `painel_operacao.png` e não mexi em nenhum arquivo fora de `h3/**` e de `fundo/{AMBIENTE.md,README.md,requirements.txt}`.
- **Sem commit e sem push**, como pedido: o que mudou está na árvore de trabalho.

## 7. Adendo (2026-10-15T21:22:55Z): as frentes do H1 nos dois numpys

O §4 deixou as frentes do H1 fora da reconferência. Depois de fechar esta declaração, rodei o A/B que faltava:
`h1/rodar.py` inteiro (projetos `a`, `b` e `c`), `OMP_NUM_THREADS=1`, com a semente fixa de `h1/rodar.py:44`
(2026, 2126 e 2226), uma vez com `python3` e outra com `PYTHONNOUSERSITE=1`. Saídas em `/tmp/ab/`.

- **O H1 depende deste sorteio.** Com a mesma semente, os dois numpys divergem em **5 campos no `a`, 7 no `b` e 59
  no `c`**. As maiores diferenças relativas: **4,1e-4** na riqueza média da estratégia de Kelly (`a`, 413 650,38
  contra 413 480,02), **7,9e-4** nos quantis do modelo nulo (`b`) e **0,37 no xi² do SBC** (`f4.sbc.chi2`, `c`,
  3,4933 contra 2,2133); no `c` mudam também o `f3.cofre.hash_pre_registro` e o `prior`. **Os três projetos passam o
  portão nos dois numpys** (`aprovado: true` para a, b e c).
- **Controle de repetibilidade.** Duas rodadas do `c` no **mesmo** numpy e na mesma semente diferem em **1** campo, e
  é o tempo do pytest: o cano é determinístico dada a semente. Logo a diferença acima é do numpy, não do relógio.
- **Leitura, sem corrigir nada.** Os números publicados de `h1/saida/` ficam mais perto da rodada do 2.5.1 do que da
  do 2.4.6 (2 contra 6 campos diferentes no `a`, 35 contra 41 no `b`, 31 contra 62 no `c`), o que é consistente com
  o ambiente de registro ser o 2.5.1 também no H1. Eles **não** reproduzem bit a bit em nenhum dos dois numpys
  (muda o `hash_pre_registro` e sobra poeira de 1e-8 a 1e-9): é outra pergunta, de outra natureza, e fica aberta.
- **O que continua sem A/B:** `h1-real/projetos/{b_real,c_real}.py`,
  `fundo/p05-prioris/h1-real/projetos/{b_real,c_real}.py` e o `evsi_beta_bernoulli` dos `estat.py`.

O texto de `fundo/AMBIENTE.md` foi atualizado com este resultado; o §4 desta declaração fica como estava quando foi
escrita, e é este adendo que o corrige. Este adendo foi escrito **antes** de o `.sha256` ser regerado, e o hash
registrado lá é o do arquivo já com ele.
