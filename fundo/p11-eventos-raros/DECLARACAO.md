# Ponto 11 (eventos raros): declaração antes de rodar

Escrita em 2026-10-15, antes de qualquer análise que gere os números abaixo. O SHA-256 deste arquivo está em
`DECLARACAO.sha256`, com a hora. Mudança de critério depois de ver o resultado entra como declaração nova e conta
no K; este arquivo não é reescrito.

## O que já se via antes de escrever

- Os resultados publicados do H3 (`h3/README.md`, `h3/saida/prisoes_ri/resultados.json`): no cofre de Rhode Island,
  n = 24 466, perda do modelo 0,021692 contra 0,022104 do reserva, habilidade oficial contra reserva +0,016785.
- O exploratório do p09 (`fundo/p09-escala-dados/saida_saude/raro_indo.json`): n no cofre 164, 36 eventos,
  Brier 0,179944 contra 0,186050 da climatologia, habilidade +0,032816 (IC90 +0,004529 a +0,063309), Murphy
  confiabilidade 0,025177, resolução 0,014283, incerteza 0,171327, probabilidades previstas entre 0,0292 e 0,2201.
- Os resultados do painel COVID do p09 (`saida_saude/saude_covid_vacina_paises/resultados.json`): 2 927 países-mês no
  treino, 240 no cofre (América do Sul), podado no F3 (p de permutação 0,28), desfecho contínuo
  y = log(1 + mortes por milhão no mês seguinte).
- Os esquemas das tabelas: colunas de `armazem/.../parte.parquet` dos dois domínios de saúde e as colunas do CSV de
  Rhode Island. Nenhuma taxa de evento por caminho foi calculada antes desta declaração.

## Pergunta

Onde o evento é raro, **a resolução (RES) de uma previsão vence a confiabilidade (REL) dela** — ou a previsão é só
um nível deslocado, com ordenação fraca? A calibração já foi medida no p08; aqui se mede a decomposição inteira.

## Três casos e suas fontes

| caso | cofre | fonte do resultado publicado | dado lido por este ponto |
|---|---|---|---|
| `indo` | Michigan (164 pacientes, 36 eventos) | `fundo/p09-escala-dados/saida_saude/raro_indo.json` | `.../saude_indometacina_ercp/armazem/saude_indometacina_ercp/ambiente=*/parte.parquet` (o CSV `medicaldata_indo_rct.csv` não está no repositório) |
| `covid` | América do Sul (240 países-mês) | `.../saude_covid_vacina_paises/resultados.json` | `.../saude_covid_vacina_paises/armazem/saude_covid_vacina_paises/ambiente=*/parte.parquet` |
| `ri` | Rhode Island 2013-01-01 em diante (24 466 paradas) | `h3/saida/prisoes_ri/resultados.json` | `dados-reais/h3c/backup_rhode_island_traffic_stops_2005_2015.csv`, pelo replay do próprio H3d (ver abaixo) |

**Evento raro declarado.** `indo`: pancreatite pós-CPRE (36 de 164 no cofre, 43 de 438 no treino) — como está.
`ri`: prisão do motorista (ordem de 3% a 5%). `covid`: o desfecho do pacote é contínuo, então declaro a
binarização **antes** de olhar o cofre: T = quantil amostral 0,90 de y no treino (y = log(1 + mortes por milhão no mês
seguinte)), e y_raro = 1 se y > T. O limiar sai só do treino; o cofre não entra no cálculo de T.

**Replay do caso `ri`.** O `diario.parquet` do H3d está fora do Git. O ponto 11 copia `h3/estocologia`,
`h3/fluxos`, `h3/rodar.py`, `h3/CRITERIOS.md` e `h3/CRITERIOS_H3d.md` byte a byte para
`fundo/p11-eventos-raros/codigo/h3/` e roda `rodar.py prisoes_ri` com `ESTOCOLOGIA_SAIDA` dentro desta pasta.
Conferência de aceitação: o hash de especificação do replay tem de dar `577de4d8111705a9babcc337f7e855df2629eba9f3403b0e7a45f8b9588c1062`,
o do `resultados.json` publicado, e o placar do cofre tem de bater com o publicado. Nada é escrito fora de
`fundo/p11-eventos-raros/`.

## Três caminhos comparados (os mesmos nos três casos)

- **C — climatologia.** Probabilidade constante = taxa do desenvolvimento (no `ri`, a coluna `reserva` do diário,
  que é a taxa do ano anterior refeita a cada 1º de janeiro; nos outros dois, a taxa de eventos do treino).
- **L — logística reajustada.** A logística do próprio caso: no `indo`, a de `escala/raro_indo.py` (indometacina,
  risco, idade, homem padronizados pelo treino, `glm.ajustar` binomial); no `covid`, a mesma família sobre o
  desenho do funil do p09 (`funil.Desenho` do treino com a chave `vacinados` só centrada e o conjunto de ajuste
  declarado no pacote: idade_mediana, idosos, pib_log, leitos, diabetes e mês); no `ri`, a coluna `modelo` do
  diário (logística reajustada a cada 1º de janeiro desde 2006 com λ = 1e-4, mais a recalibração online de Platt).
- **R — escore de ranking.** O mesmo preditor linear de L, usado só como **ordem**: o nível vem da climatologia dos
  10 decis de massa igual do desenvolvimento (`p_R` = taxa de eventos do decis, estimada só no desenvolvimento;
  a ordenação, e portanto a AUC, é a de L).

No `ri` o caminho oficial do H3 (modelo onde `origem == "modelo"`, reserva nos demais casos) também é calculado,
como **informativo**, fora do critério.

## Medidas, no cofre, por caso e por caminho

- Brier = média de (p − y)²; identidade conferida: Brier = REL − RES + UNC (tolerância 1e-12).
- Decomposição de Murphy com a função `murphy` de `h3/estocologia/calibracao.py`: 10 caixas de **massa igual**;
  REL = Σ n_k (p̄_k − ȳ_k)²/n, RES = Σ n_k (ȳ_k − ȳ)²/n, UNC = ȳ(1 − ȳ).
- AUC (Mann–Whitney, empates por posto médio), para C, L e R.
- Habilidade de Brier = 1 − Brier/Brier_C, com Brier_C o do caminho C no mesmo cofre.
- **IC90 por bootstrap**: B = 2 000 reamostragens do cofre com reposição, semente 2027, caixas refeitas dentro de
  cada reamostragem; IC90 = quantis 0,05 e 0,95. Sai o IC90 de RES − REL, da habilidade e da AUC.
- Replicação declarada (não é resultado do ponto, é conferência de esteira): no `indo`, refazer o número publicado
  com o método do próprio arquivo (10 caixas de mesma **largura**, 2 000 reamostragens, semente 2027) e exigir
  Brier 0,179944, Murphy 0,025177/0,014283/0,171327 e habilidade 0,032816 dentro de 1e-9.

## Critério (declarado agora, antes de ver qualquer número do ponto)

Um caminho **vence** no caso quando as duas condições valem juntas no cofre:

1. RES > REL com o IC90 de (RES − REL) inteiramente acima de zero;
2. habilidade de Brier ≥ **+0,05**.

Observações declaradas: RES > REL equivale a Brier < UNC (a previsão bate a climatologia **do próprio cofre**), que é
uma condição diferente da (2), que compara com a climatologia do **desenvolvimento**. Leituras secundárias, só
informativas e **não** decisivas: o IC90 da habilidade acima de 0,05 (leitura estrita) e a AUC.

## Limites declarados de antemão

- No `indo` e no `covid`, o dado lido é o `parte.parquet` do armazém do p09, não o CSV bruto, que não está no
  repositório; o p09 declarou o cofre do `indo` como **não cego** (a tabela centro × braço × desfecho foi vista
  antes de declarar), e essa ressalva passa para cá.
- No `ri`, a linha de base é um replay de 2006 a 2015 com ~6 minutos de parede; se o replay não reproduzir o placar
  publicado, o caso `ri` é relatado como não disponível e os outros dois seguem.
- A binarização do `covid` é transformação declarada, não o desfecho do pacote; o número do F3 do p09 não é
  reaberto.
- Caixas de massa igual com previsão degenerada (todos os decis com a mesma taxa no desenvolvimento) fazem o
  caminho R virar C; se acontecer, é relatado, não corrigido.

## Ordem dos passos

1. Esta declaração, com hash e hora (`DECLARACAO.sha256`) — **antes** de rodar.
2. Cópia byte a byte do código do H3d e do p09 para `codigo/`; replay do `prioses_ri`/`prisoes_ri` para
   `saida/h3_ri/`, com conferência do hash de especificação e do placar.
3. `rodar.py` calcula os três casos, grava `saida/indo.json`, `saida/covid.json`, `saida/ri.json` e
   `saida/resumo.json`.
4. `README.md` no formato das outras frentes (Resultado, O que foi feito, Limites, Como reproduzir).

Recursos: `OMP_NUM_THREADS=1`, no máximo dois processos em paralelo.
