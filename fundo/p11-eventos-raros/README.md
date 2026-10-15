# Ponto 11 (eventos raros): resolução contra confiabilidade, não só calibração

## Resultado

O p08 mediu a calibração; aqui se mede a decomposição inteira onde o evento é raro. **Pela regra declarada
(RES > REL com IC90 de (RES − REL) acima de zero E habilidade de Brier no cofre >= +0,05), nenhum caminho vence
em nenhum dos três casos.** O que se separa é *por que*:

- **Em Rhode Island a resolução bate a confiabilidade, e é o único caso em que isso é verificável.** Com 24 466
  paradas no cofre e 553 prisões (2,26%), a logística reajustada dá RES − REL = **+2,75e-4**, IC90
  [+1,71e-4; +3,79e-4], e o escore de ranking **+3,12e-4**, IC90 [+2,06e-4; +4,07e-4]. Os dois IC90 ficam
  inteiramente acima de zero, então a **condição (1) passa** — ao contrário do que acontece nos cofres pequenos.
  A habilidade de Brier, porém, fica em **+1,86%** (logística) e **+1,80%** (ranking), longe do +5% declarado:
  a **condição (2) reprova**, e é ela que decide.
- **Nos dois cofres pequenos nem a condição (1) passa.** No `indo` (164 pacientes, 36 eventos), RES − REL =
  −8,29e-3 com IC90 [−2,52e-2; +1,45e-3] — o intervalo **cruza zero**; no `covid` (240 países-mês, 52 eventos),
  −1,81e-2 com IC90 [−3,34e-2; −6,55e-3], inteiramente **abaixo** de zero.
- **A ordenação existe e é boa; o nível é que estraga.** AUC 0,714 no `indo` e **0,768** no `covid`, contra
  habilidades de Brier de +0,033 e **−0,021**. O `covid` é o caso mais limpo: a logística ordena bem os
  países-mês (AUC 0,77) e mesmo assim **perde para a constante do treino**, porque a taxa do evento no cofre
  (21,7%) é o dobro da do treino (10,0%) e esse deslocamento de nível entra inteiro na confiabilidade.
- **RES > REL equivale a Brier < UNC** (a previsão bate a climatologia *do próprio cofre*). É exatamente o que se
  vê: nos dois cofres pequenos ninguém bate o UNC (0,171327 no `indo`, 0,169722 no `covid`); em Rhode Island a
  logística e o ranking batem (0,021692 e 0,021707 contra UNC 0,022092) e a climatologia não (0,022104).
- **O caminho R (ranking com o nível tirado dos decis do desenvolvimento) nunca melhora o L**: repete a ordenação
  dele por construção e piora o Brier nos três casos. No `indo` ele é pior que a própria constante, porque com
  44 pacientes por decil as taxas do desenvolvimento saem não monótonas
  (0,0455 · 0,0227 · 0,1163 · 0,0455 · 0,1364 · 0,1136 · 0 · 0,1364 · 0,1556 · 0,2045).

| caso | cofre: n / eventos / taxa | caminho | Brier | REL | RES | RES − REL [IC90] | habilidade [IC90] | AUC [IC90] | vence |
|---|---|---|---|---|---|---|---|---|---|
| `indo` (Michigan) | 164 / 36 / 0,2195 | C climatologia | 0,186050 | 0,027356 | 0,012633 | −0,0147231 [−0,03105; −0,004417] | 0 | 0,500 | não |
| | | L logística | 0,179944 | 0,035918 | 0,027630 | −0,0082881 [−0,02520; +0,001452] | +0,032816 [+0,004529; +0,063309] | 0,714 [0,630; 0,796] | não |
| | | R ranking | 0,186822 | 0,039238 | 0,022990 | −0,0162483 [−0,03446; −0,004005] | −0,004149 [−0,04826; +0,03956] | 0,564 [0,459; 0,662] | não |
| `covid` (América do Sul) | 240 / 52 / 0,2167 | C climatologia | 0,183309 | 0,024282 | 0,010694 | −0,0135872 [−0,02637; −0,005607] | 0 | 0,500 | não |
| | | L logística | 0,187132 | 0,058664 | 0,040556 | −0,0181081 [−0,03339; −0,006549] | −0,020853 [−0,04640; +0,008005] | 0,768 [0,717; 0,820] | não |
| | | R ranking | 0,190501 | 0,048157 | 0,025972 | −0,0221846 [−0,03728; −0,009927] | −0,039234 [−0,06035; −0,01650] | 0,755 [0,704; 0,805] | não |
| `ri` (Rhode Island 2013-15) | 24 466 / 553 / 0,02260 | C climatologia | 0,0221036 | 1,7810e-5 | 6,4443e-6 | −1,13651e-5 [−2,214e-5; −2,992e-6] | 0 | 0,4964 [0,4775; 0,5150] | não |
| | | L logística | 0,0216918 | 7,3860e-5 | 3,48502e-4 | **+2,74643e-4** [+1,708e-4; +3,793e-4] | +0,018629 [+0,011364; +0,025768] | 0,7250 [0,7073; 0,7421] | não |
| | | R ranking | 0,0217068 | 3,7584e-5 | 3,49240e-4 | **+3,11655e-4** [+2,057e-4; +4,067e-4] | +0,017952 [+0,012909; +0,022829] | 0,7300 [0,7133; 0,7458] | não |
| | | oficial H3 (informativo) | 0,0217326 | 5,3e-5 | 2,83e-4 | +2,30281e-4 [+1,393e-4; +3,308e-4] | +0,016785 [+0,009992; +0,023764] | 0,7110 [0,6934; 0,7283] | fora do critério |

UNC: 0,171327 (`indo`), 0,169722 (`covid`), 0,0220919 (`ri`). Taxas de treino: 0,09817 (438 pacientes, 43 eventos),
0,10010 (2 927 países-mês, 293 eventos) e 0,03113 (59 166 paradas, 1 842 prisões). IC90 = quantis 0,05 e 0,95 de
2 000 reamostragens do cofre com reposição, semente 2027, caixas refeitas dentro de cada reamostragem.

**Leitura.** Em evento raro, "tem resolução?" e "serve para alguma coisa?" são perguntas diferentes. Em Rhode
Island a resposta à primeira é sim, com precisão (o IC90 não cruza zero com 553 eventos), e a resposta à segunda
é não, à luz do limite declarado: +1,9% de habilidade de Brier é cerca de 40 vezes menos que o +5% exigido.
No `covid` a ordenação é boa (AUC 0,77) e a previsão é **pior que não dizer nada** — o nível, não a discriminação,
é o que falha. Nenhum dos três casos sustenta a leitura "resolução útil" do p09.

## O que foi feito

A declaração foi escrita antes de qualquer análise do ponto: [DECLARACAO.md](DECLARACAO.md), SHA-256
`ef3a42c71fab864406e733d7202b1870abba5baf9a80235a728c386b64e900f3` em `DECLARACAO.sha256`, com a hora.
Nada foi ajustado depois de ver os números; o único desvio encontrado está em **Limites** (a conferência de
identidade declarada, que não fecha para previsão crua).

Três casos, três caminhos, fontes só do repositório:

1. **`indo`** — indometacina retal e pancreatite pós-CPRE; leitura do armazém do p09
   (`fundo/p09-escala-dados/saida_saude/saude_indometacina_ercp/armazem/.../parte.parquet`), conferido contra o
   publicado em `saida_saude/raro_indo.json`. A logística é a de `escala/raro_indo.py` (indometacina, risco, idade
   e homem padronizados no treino, IRLS de `glm.ajustar`).
2. **`covid`** — painel país-mês de vacinação e mortes (Our World in Data, 2021-2022), armazém do p09. O desfecho do
   pacote é contínuo, então a binarização do evento raro foi declarada antes: T = quantil 0,90 amostral de
   y = log(1 + mortes por milhão no mês seguinte) **no treino** = 4,610116926832605, y_raro = 1 se y > T.
   A logística usa o desenho do próprio funil do p09 (`funil.Desenho` do treino: chave `vacinados` só centrada,
   ajuste declarado `idade_mediana, idosos, pib_log, leitos, diabetes, mes`).
3. **`ri`** — prisão do motorista nas paradas de Rhode Island (Stanford Open Policing 2005-2015). O
   `diario.parquet` do H3d está fora do Git, então o ponto roda o **próprio H3d** em cópia: `codigo/h3/` tem
   `h3/estocologia`, `h3/fluxos`, `h3/rodar.py`, `h3/CRITERIOS.md` e `h3/CRITERIOS_H3d.md` byte a byte
   (conferidos por SHA-256 um a um), e o replay escreve em `saida/h3_ri/` (`ESTOCOLOGIA_SAIDA`), 286,8 s de parede,
   `OMP_NUM_THREADS=1`, um processo por vez. O caminho L é a coluna `modelo` do diário (logística reajustada a cada
   1º de janeiro desde 2006, λ = 1e-4, mais a recalibração online de Platt); o caminho oficial do H3 (que troca o
   modelo pelo reserva quando o disjuntor abre ou o caso sai do escopo) sai como informativo, fora do critério:
   ele substitui o modelo em 12,5% do cofre e tem habilidade +0,016785, abaixo do limite nas duas leituras.

Arquivos escritos (tudo dentro de `fundo/p11-eventos-raros/`):

- `DECLARACAO.md`, `DECLARACAO.sha256` (declaração e hash, antes de rodar);
- `rodar.py` (SHA-256 `eb2d71599a60fd357490086a11bfa8dbaa2c0620fe1004690c50dac2a17ce804`) e `comum.py`
  (SHA-256 `2d630cfcbaf247fc0a0122b2ccaca90527c0ec2ae8c72629639eb7d005ca20c7`);
- `codigo/h3/` e `codigo/p09/` (cópias byte a byte do código, só para ler);
- `saida/indo.json`, `saida/covid.json`, `saida/ri.json`, `saida/resumo.json`;
- `saida/h3_ri/prisoes_ri/` (o replay: diário de 3,58 MB, placar, barramento e registro de varredura).

**Checagens de aceitação rodadas.**

| checagem | resultado |
|---|---|
| `sha256sum -c DECLARACAO.sha256` | `DECLARACAO.md: OK`, saída 0 |
| `indo`: número publicado conferido contra `raro_indo.json` (método do próprio arquivo: caixas de largura igual, semente 2027) | reproduzido até 1,04e-17 — Brier 0,17994425378015869, climatologia 0,18604966637018594, habilidade 0,03281603621841067, IC90 bootstrap [0,0045290492465221875; 0,06330865734391301], Murphy 0,025176993 / 0,014283102 / 0,171326591, n 164, 36 eventos |
| `covid`: linhas do treino contra `saude_covid_vacina_paises/resultados.json` | 2 927 = `filtro2.n_linhas`; cofre 240 (América do Sul); 6 ambientes no armazém |
| `ri`: hash de especificação do replay contra `h3/saida/prisoes_ri/resultados.json` | `577de4d8111705a9babcc337f7e855df2629eba9f3403b0e7a45f8b9588c1062` nos dois |
| `ri`: placar do cofre do replay contra o publicado | perda do modelo 0,021691811119829506 e do reserva 0,022103568223804892, idênticos (`max_diferenca_absoluta` = 0,0); contagens também (83 632 emitidas, 22 de escopo, 6 508 de disjuntor, 12 reajustes) |
| identidade de Murphy `Brier = REL − RES + UNC` | fecha para a previsão constante na caixa (≤ 2,8e-17, tolerância 1e-12) e **não** fecha para a previsão crua (até 1,41e-3) — ver Limites |
| conferência independente dos números, fora do código do ponto | `ri` por SQL puro sobre o diário: perda do modelo 0,02169181111982583, do reserva 0,02210356822380517, n 24 466, 553 eventos — bate com o JSON e com o H3; `indo` com um ajuste logístico próprio (Newton–Raphson, sem o `glm` do p09): Brier 0,17994425508195233 contra 0,17994425378015869 do publicado (1,3e-9 de diferença, o ridge 1e-6 do IRLS); `covid`: T, 2 927 linhas de treino, 293 eventos e 240/52 no cofre reconferidos por leitura própria |
| determinismo | segunda execução completa de `rodar.py`: os quatro arquivos de `saida/` saem idênticos (`diff` byte a byte) |

## Limites

- **A conferência de identidade declarada não fecha como declarada.** Escrevi no `DECLARACAO.md` "identidade
  conferida: Brier = REL − RES + UNC (tolerância 1e-12)". Com caixas de massa igual e previsão que **varia dentro
  da caixa**, a identidade de Murphy é exata só para a previsão constante na caixa (a média da caixa). A diferença
  da versão crua é 0,00033 (`indo`, L), 0,00070 (`covid`, L), 0,00141 (`covid`, R) e 0,00013 (`ri`, L); com a
  previsão constante na caixa a diferença é ≤ 2,8e-17 nos 12 pares caso × caminho. O desvio está **relatado**, os
  dois números saem em `saida/*.json`, e o critério não mudou (ele não usa a identidade). Corrigir o texto da
  declaração seria reescrever um arquivo já com hash, o que a regra de casa proíbe.
- **No `indo` e no `covid`, o dado lido é o `parte.parquet` do armazém do p09, não o CSV bruto**, que não está no
  repositório. No caso do `indo`, a conferência de aceitação acima mostra que o caminho é o mesmo do publicado (a
  reprodução chega ao último bit), mas a leitura **não** é do arquivo de origem.
- **O cofre do `indo` não é cego**, como o próprio p09 declarou (a tabela centro × braço × desfecho foi vista
  antes de declarar). A ressalva passa inteira para cá.
- **A binarização do `covid` é transformação declarada**, não o desfecho do pacote (que é contínuo). O veredito do
  F3 do p09 (podado, p = 0,28) não é reaberto nem contestado por este ponto.
- **O caminho R não é um concorrente justo do L no `indo`**: com 438 pacientes, cada decil tem cerca de 44 pacientes
  e 4 a 9 eventos, e as taxas do desenvolvimento saem não monótonas — o nível do ranking fica pior que a constante.
  No `ri` e no `covid`, os decis do desenvolvimento são aproximadamente monótonos.
- **Cofres pequenos com eventos raros têm pouca resolução para medir**: no `indo` o IC90 de RES − REL cruza zero com
  largura de 2,7e-2; isso é falta de poder, não evidência de que a resolução não exista.
- **O diário do replay (`saida/h3_ri/prisoes_ri/diario.parquet`, 3,58 MB) passa do limite de 2 MB** da convenção
  de arquivos fora do Git. Ele é regenerável pelo comando em "Como reproduzir", e `FORA_DO_GIT.md` está fora do
  escopo de escrita deste ponto, então a decisão de registrá-lo ou ignorá-lo fica com quem fecha a frente.
- O que **não** deu para fazer: nada dos três casos ficou pendente. O CSV bruto das duas bases de saúde não está no
  repositório (limite acima) e o `diario.parquet` do H3d também não estava — resolvido pelo replay, com hash e
  placar conferidos.
- A declaração tem um erro de digitação no caminho do comando do passo 2 (`prioses_ri`); o comando que rodou é o
  correto (`prisoes_ri`), e a declaração não foi reescrita.

## Como reproduzir

A partir da raiz do repositório, com python 3.13.7 (numpy 2.4.6, scipy 1.18.0, pandas 3.0.5, duckdb 1.5.5,
pyarrow 26.0.0, PyYAML 6.0.3), `OMP_NUM_THREADS=1` e um processo por vez:

``%%
cd fundo/p11-eventos-raros
sha256sum -c DECLARACAO.sha256                       # DECLARACAO.md: OK

# 1) replay do H3d dentro desta pasta (286,8 s de parede; nada é escrito em h3/)
cd codigo/h3
OMP_NUM_THREADS=1 PYTHONDONTWRITEBYTECODE=1 \
  ESTOCOLOGIA_DADOS=$PWD/../../../../dados-reais \
  ESTOCOLOGIA_SAIDA=$PWD/../../saida/h3_ri \
  python3 rodar.py prisoes_ri

# 2) os três casos
cd ../..
OMP_NUM_THREADS=1 PYTHONDONTWRITEBYTECODE=1 python3 rodar.py
``%%

O passo 2 imprime uma linha por caso e o veredito. Ele lê `saida/h3_ri/prisoes_ri/diario.parquet` para o caso
`ri`: sem o passo 1, esse caso é relatado como indisponível e os outros dois seguem.
