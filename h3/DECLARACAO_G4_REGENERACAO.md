# Declaração: regeneração do G4 (laço performativo) e do `h3/saida/portoes.json`

**Data:** 2026-10-15T20:36Z (17:36 -03). **Quem:** agente delegado da frente de regeneração do H3, ramo
`claude/project-thread-7rrw8h`. **Declaração escrita e conferida com hash antes de qualquer arquivo publicado
ser tocado.**

**Autorização.** O dono do repositório autorizou, nesta rodada, mexer no número publicado. A regra de casa que
valia antes (número publicado intocável) fica suspensa só para esta regeneração; o resto da ordem continua
valendo: declarar com hash antes, registrar o valor antigo e dizer no texto por que ele mudou.

## 1. O que será regenerado

`h3/saida/portoes.json` inteiro, com foco em `G4 performatividade e Goodhart` → `detalhe.laco_performativo`
(as duas rodadas, r = 0,0 e r = 0,6, com `deteccao_M11`, `ingenuo` e `controlado`), e os demais arquivos
publicados de `h3/saida/**` que mudarem de número — relatados um por um, com nome, valor antigo e valor novo.
Nenhum arquivo grande fora do Git é tocado: as saídas da regeneração vão para `/tmp/g4regen`.

## 2. O valor publicado hoje (registro obrigatório, não se perde)

Bloco `r=0.6` de `deteccao_M11`:

- `r_hat` = **0,5632133268561669**
- `n_choques` = **2375**
- `coef_choque` = −0,1687860154332624
- `t` = −6,436797453264879; `p` = 1,471613436735315e-10
- `r_hat_ic90_bootstrap_por_dia` = [0,5054256740255859; 0,6098441503477998]
- `ingenuo`: viés +29,1247% contra a demanda exógena, +1,0353% contra o observado, 2934,6474
  atendentes-hora/dia, 15142,1346 desistências/dia
- `controlado`: viés +1,7306%, 2930,9447 atendentes-hora/dia, 15094,75 desistências/dia

Bloco `r=0.0`: `n_choques` = 2383, `r_hat` = 0,0, `coef_choque` = 0,0033213574336744024, `p` = 0,7037249570692391,
IC90 = [−0,02986614193055604; 0,004896677103509835], viés −0,07112% nas duas pontas.

## 3. Por que o número muda (diagnóstico, com a evidência)

**A divergência é de ambiente, não de versão de código.**

- `h3/estocologia/performativo.py` tem **uma única versão** no git (o commit único de pesquisa `4bb7181`), e as
  quatro cópias na árvore — `h3/`, `fundo/p03-multiplicidade/copias/h3_ri32/`, `fundo/p08-calibracao/codigo/` e
  `fundo/p11-eventos-raros/codigo/h3/` — têm o **mesmo SHA-256** `6bdfcb197b786f0e95ed6e74d7101e25daff597b46bb305a2b963eefc4f18cfc`.
  Não existe código anterior de onde o número publicado pudesse ter saído.
- O módulo do HEAD, rodado **duas vezes** nesta máquina com `OMP_NUM_THREADS=1`, é determinístico: r̂ =
  0,5528868864293444 e n_choques = 2395 nas duas. Sem `OMP_NUM_THREADS`, 0,5528868864293441 — muda só o 16º
  algarismo, que é o `np.linalg.lstsq`.
- O **mesmo** módulo, com o numpy **2.5.1** do interpretador (o do `pyenv`, que serve o `python3` quando o site do
  usuário não entra no `sys.path`), devolve o número publicado: r̂ = 0,5632133268561668 e **n_choques = 2375**,
  com `coef_choque` = −0,1687860154332624 idêntico até o 16º algarismo.
- Onde os dois ambientes divergem: rastreando a simulação intervalo a intervalo (164 dias × 169 intervalos =
  27 716), numpy 2.4.6 e 2.5.1 coincidem em **tudo** até o dia 16, intervalo 55 — mesmo c = 236, mesmo pa, mesmo
  n = 539, mesmo abandono = 408 e mesmas rediscagens = 248 — e divergem no sorteio seguinte, o
  `rng.binomial(248; 0,6)` que reparte a rediscagem: **171 no numpy 2.4.6 e 150 no 2.5.1**. Daí em diante as duas
  simulações descorrelacionam e o sorteio de exploração (10% dos intervalos, escala × 0,7 ou × 1,3) muda de
  padrão: 2395 choques contra 2375, e o bloco r = 0,6 inteiro se move.
- O bloco de r = 0,0 é **insensível** ao numpy: 2383 choques e `coef_choque` iguais até 1e-15 nos dois. O caminho
  sem rediscagem não passa pelo sorteio que mudou — por isso a divergência aparece num bloco e não no outro.
- O `fundo/p03-multiplicidade` chegou ao mesmo lugar por outro caminho: lá o `laco_versoes.py`, que importa este
  mesmo módulo sem mudança, deu o r̂ de 0,5632133268561669 igual ao publicado, e o README atribuiu a diferença de
  16º algarismo ao BLAS com várias threads.

**Qual numpy o `python3` importa hoje.** `~/.local/lib/python3.13/site-packages/numpy` é o **2.4.6** e vem antes do
site do `pyenv` (**2.5.1**) na ordem do `sys.path`; é o 2.4.6 que o `fundo/AMBIENTE.md` certifica com a saída do
comando. O número publicado veio do **2.5.1**, isto é, de uma execução em que o site do usuário não entrava no
caminho. O valor novo, portanto, é o do HEAD **no ambiente declarado abaixo** (o que o `AMBIENTE.md` certifica).

## 4. Ambiente declarado desta regeneração

`fundo/AMBIENTE.md` de 2026-10-15: **Python 3.13.7** (`/home/silvano-neto/.pyenv/versions/3.13.7/bin/python3`,
GCC 15.2.0), **numpy 2.4.6**, **pandas 3.0.5**, **scipy 1.18.0**, **pyarrow 26.0.0**, duckdb 1.5.5, PyYAML 6.0.3,
pytest 9.1.1, hypothesis 6.165.7 — e **`export OMP_NUM_THREADS=1`**.

## 5. Procedimento exato

```
export OMP_NUM_THREADS=1
cd /home/silvano-neto/Documents/estocologia/h3
export ESTOCOLOGIA_SAIDA=/tmp/g4regen
export ESTOCOLOGIA_DADOS=/home/silvano-neto/Documents/estocologia/dados-reais
python3 rodar.py energia voos     # os fluxos em pares, para não passar de 2 processos
python3 rodar.py central bolsa
python3 rodar.py ciclones prisoes
python3 grafo.py
python3 portoes.py --reproducao   # G1–G7 com um segundo replay limpo; grava portoes.json em $ESTOCOLOGIA_SAIDA
```

O `portoes.py` recalcula o laço do G4 com `P.rodar(dados-reais/bank_calls.csv)`, de semente 2027 — a mesma
chamada determinística descrita acima. O G1 do próprio `portoes.py` roda um `rodar.py` limpo em diretório
temporário; esse passo interno abre um processo por fluxo (é o que o código declara), e por isso ele passa dos
2 processos por alguns minutos — desvio registrado, não corrigido (mexer no `portoes.py` mudaria o hash do
módulo e contaria K).

## 6. A regra

**O número publicado do G4 passa a ser o do HEAD no ambiente declarado:** r̂ = 0,5528868864293444 e n_choques =
2395 no bloco r = 0,6. **O valor antigo (0,5632133268561669 e 2375) fica registrado nesta declaração e no
`h3/README.md`, com a razão:** ele veio de outro numpy (2.5.1), não de outro código — o mesmo arquivo reproduz
os dois números, e o que os separa é o ambiente. O veredito do G4 não muda: a detecção segue com p < 0,01 para
r = 0,6 e p > 0,05 para r = 0, e o viés do controlado passa de +1,73% para +2,20%, ainda dentro do limite de 3%.

## 7. K do H3

- Nos seis fluxos do H3, **K = 16** (energia 3, central 3, bolsa 2, voos 3, ciclones 3, prisões 2), como está no
  `G7` do `portoes.json`. Contando as extensões, os `especificacoes.jsonl` somam 20 entradas (voos23 1,
  prisoes20 1, prisoes_ri 2 a mais).
- O laço performativo teve **4 versões** antes de passar (v1–v4, registradas no campo `historico` do módulo).
- **Esta regeneração não é especificação nova:** o código não muda, o critério não muda e nenhum hash de módulo
  muda. Não cria entrada de K; o que muda é o número publicado e o ambiente em que ele foi medido.

## 8. Limites

- O que fica sem explicação fechada é **de quem é a culpa** entre os dois numpys: o 2.4.6 é o que o
  `AMBIENTE.md` certifica como o ambiente da máquina, e é o que o `python3` importa hoje; o 2.5.1 é o do outro
  site do mesmo interpretador.
- O bloco de r = 0,0 do publicado continua batendo com o do HEAD até 1e-15; a regeneração vai reescrevê-lo com o
  ruído do `lstsq`/bootstrap desta máquina (14º–16º algarismo), sem mudança de valor.
- Se algum número publicado além do G4 mudar como efeito colateral (G2, G3, G5, G7, os `resultados.json` dos
  fluxos) ele é relatado um por um; **a decisão sobre o que fazer com esses é do dono do repositório**.
- O `painel_operacao.png` (figura, não número) não é regenerado nesta rodada.
