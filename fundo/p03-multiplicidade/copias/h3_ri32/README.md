# H3: malha autônoma contínua

**Saída do H3: NÃO aprovada.** Dos três critérios do PLAN, dois passam. Não houve vazamento temporal, e a
performatividade e o Goodhart foram controlados. O terceiro, calibração fora do tempo, passa em 4 dos 6 fluxos e
falha em dois:
- **voos:** a frequência real de atraso no 4º trimestre ficou 2,7 p.p. acima da prevista, e a tolerância
  declarada era 2 p.p.;
- **prisões:** o ECE ficou em 0,036, para uma tolerância de 0,03, com só 1 488 casos no cofre.

Reprodução limpa, contenção de varredura e trilha passam.

Os critérios estão em [CRITERIOS.md](CRITERIOS.md). Foram escritos antes de qualquer replay, e o hash do arquivo
entra em cada pré-registro.

```
python3 rodar.py                    # replay dos 6 fluxos, em paralelo (~25 min; os voos sozinhos levam ~20)
python3 grafo.py                    # hipergrafo em Parquet: 454 178 nós
python3 figura.py                   # saida/painel_operacao.png
python3 portoes.py --reproducao     # G1–G7, com um segundo replay limpo
```

## O que foi construído

- **Armazém temporal (F2).** Cada fato é um evento com `t_disp`, o instante em que se torna conhecível. Os agentes
  só leem janelas (último tick, agora], e o armazém recusa:
  - ler o futuro;
  - SELECT *;
  - ler o cofre antes do pré-registro;
  - abrir o cofre duas vezes ou com o agente errado;
  - voltar o relógio.

  Projeção e filtro são conferidos no EXPLAIN do DuckDB. Os grupos de linhas do Parquet têm o tamanho de um
  tick, e por isso a operação inteira varre cada tabela no máximo 2,03 vezes.
- **Motor contínuo.** Faz o replay sob relógio. No corte, o F3 congela a especificação num hash de pré-registro e
  abre o cofre uma vez. Para cada previsão, o diário colunar guarda o modelo e o reserva lado a lado, a origem da
  previsão oficial, o t_disp máximo do estado no momento da emissão e o t_disp do rótulo.
- **F3, deriva.** Usa KS e Wasserstein com histerese. Os limiares são o q99/q90 da própria variação da primeira
  metade do desenvolvimento, não valores convencionais. O alarme dispara um reajuste e rebaseia a referência.
- **F8, disjuntor e política de ignorância.** Quando o modelo perde para o reserva determinístico numa janela
  móvel, o reserva passa a ser a previsão oficial. O modelo continua em sombra, e é a vitória dele em sombra que
  fecha o disjuntor. Uma entrada fora do escopo do F7 também cai no reserva.
- **Abster-se não tira a previsão do placar.** Quando o modelo é recusado, pontua-se o reserva no lugar dele.
- **F4.** Os quantis vêm de resíduos padronizados com inferência conforme adaptativa (Gibbs & Candès 2021).

## Fluxos (dados reais) e calibração no cofre

| fluxo | previsão | cofre | n no cofre | calibração | habilidade | portão |
|---|---|---|---|---|---|---|
| energia (Victoria) | demanda de cada meia-hora de amanhã, sem a temperatura de amanhã | 2014 | 17 376 | cobertura 90% = 0,887 | +0,10 contra o sazonal ingênuo; reserva oficial em 48% do tempo | passa |
| central (banco, 2003) | chamadas por 5 min do próximo dia útil | set–out | 6 084 | cobertura 0,900 | +0,25 | passa |
| bolsa (DJIA) | VaR 99% do próximo pregão | 2000–2023 | 6 010 | violações 0,98%, Kupiec p = 0,89 | +0,04 (pinball) | passa |
| voos (NY 2013) | P(atraso > 15 min), 2 h antes | out–dez | 84 301 | ȳ − p̄ = +0,027, ECE 0,027 | +0,13 contra a climatologia | **falha** (\|CITL\| > 0,02) |
| ciclones (HURDAT2) | P(+30 kt em 24 h) | 2015–2022 | 5 766 | CITL 0,001, ECE 0,012 | +0,10 | passa |
| prisões (Toronto) | P(liberação) | 2001–2002 | 1 488 | CITL 0,016, **ECE 0,036** | +0,08 | **falha** (ECE > 0,03) |

Notas sobre a tabela:
- **Bolsa.** As violações do VaR se aglomeram: Christoffersen dá p = 0,02. O teste é relatado, mas não era
  exigido.
- **Voos.** O desenvolvimento cobriu jan–set. O 4º trimestre teve mais atrasos que o modelo de 60 dias esperava,
  e o disjuntor abriu só uma vez no cofre: o modelo continuou vencendo o reserva, só que descalibrado na média.
  O sensor de deriva da entrada viu o atraso de partida mudar (1 alarme no cofre), mas não há nenhum mecanismo de
  recalibração da média. Lição: o disjuntor compara com o reserva, não com a calibração.
- **Prisões.** São 1 488 casos e um modelo fixo por ano: o ECE de 10 caixas tem ruído da ordem da tolerância. O
  previsor é "cego à cor" (ver F6).

## G3: zero vazamento temporal (passa)

- **Leituras e previsões.** Nenhuma leitura devolveu evento além do relógio. Nenhuma previsão foi emitida com
  estado do futuro, e nenhum rótulo já era conhecido na emissão. São 453 973 previsões, zero violações.
- **Cofre.** O pré-registro veio antes da abertura nos 6 fluxos, e cada cofre abriu uma vez.
- **Banca.** Todos os ataques foram recusados: ler o futuro, SELECT *, cofre antes do pré-registro, agente errado,
  segunda abertura, voltar o relógio.
- **Canário.** Uma coluna igual ao rótulo foi publicada no instante do rótulo. Com o relógio honesto, ela nunca
  apareceu numa emissão, e o ganho foi 0,000 nos três fluxos binários. Com o relógio desligado (controle
  positivo), o ganho é de +0,86 a +0,92. O teste teria detectado um vazamento.

## G4: performatividade e Goodhart (passa)

**Laço performativo.** Nenhuma base registra as ações do sistema, então o laço é declarado. As chegadas reais da
central são as primeiras tentativas. A escala sai da previsão pelo Erlang C. Quem desiste rediscam com
probabilidade r. Em 10% dos intervalos, um sorteio muda a escala para × 0,7 ou × 1,3.

- **Detecção (M11).** Com r = 0,6, t = −6,4 e p = 1,5e-10. Com r = 0, p = 0,70.
- **Controle.** A previsão ingênua, treinada nas chegadas observadas, superestima a demanda exógena em **29%**,
  e mesmo assim acerta o que observa (1%). É o Goodhart em estado puro: a métrica do próprio alvo aprova um
  previsor errado sobre o mundo. O r̂ estimado por 2SLS com os choques sorteados como instrumento é 0,56 (IC 90%
  de 0,51 a 0,61). Com ele, a previsão corrigida erra a demanda exógena em **1,7%**, dentro do limite de 3%.
- **Banca contra o placar.** O previsor que se abstém na zona incerta recebe exatamente o escore do reserva e fica
  atrás do honesto nos três fluxos binários. O previsor constante na climatologia recebe peso 0. A segunda
  abertura do cofre é recusada.
- **Índice de Goodhart (habilidade no desenvolvimento menos no cofre).**
  - energia: +0,14;
  - voos: +0,02;
  - nos outros fluxos, o cofre ficou acima do desenvolvimento.

## Informativos

- **F5.**
  - O k-anonimato da base de prisões é 1: 11% das linhas são únicas nos quase-identificadores, e 2% continuam
    únicas com a idade em faixas de 5 anos.
  - Republicar o acumulado todo ano gasta ε por pessoa. O disjuntor de privacidade retirou a coorte de 1997 da
    publicação de 2001, e as de 1997–98 da de 2002. Nenhuma coorte passou de ε = 2.
- **F6, trilema.**
  - A taxa de não liberação de negros foi de 32% contra 12% de brancos em 1998, e de 21% contra 19% em 2002.
  - O FPR dos negros é cerca do dobro em todos os anos: 0,16–0,23, contra 0,06–0,11.
  - A calibração dentro dos grupos vale de 2000 em diante; os equilíbrios de FPR e de FNR nunca valem.
  - Não usar a cor no modelo não dá paridade, porque a cor entra pelos correlatos.
- **Operação.**
  - Alarmes de deriva no cofre: energia 1, central 2, voos 1, prisões 1.
  - Aberturas do disjuntor: energia 10 no desenvolvimento e 6 no cofre; voos 4 e 1.
  - Decisões alteradas pelo limiar C/L em relação ao reserva: voos 21%, bolsa 18%, energia 13%.
- **Hipergrafo.** São 454 178 nós, dos quais 453 973 são previsões e 205 são topologia, mais 51 hiperarestas e 7
  pontes. A meta do PLAN é 10⁶, então ficou em 45% dela. Nós e arestas estão em Parquet.

## Mudanças depois de ver resultados (K)

- **Desenvolvimento** (permitido antes do corte, registrado em `saida/<fluxo>/especificacoes.jsonl`):
  - os ciclones passaram a treinar numa janela de 20 anos, que tirou o CITL do desenvolvimento de 0,013 para
    0,010;
  - os tamanhos dos grupos de linhas foram ajustados para o G5.
- **Central depois da abertura do cofre.** A primeira execução completa da central travou num laço infinito: o
  Erlang C, escrito com fatoriais, estourava em cargas grandes e devolvia NaN. Troquei pela recursão estável de
  Erlang B e rodei a central de novo. A troca só afeta a decisão (atendentes), não as previsões, mas muda o hash
  do módulo e conta no K (3).
- **Laço performativo.** Teve 4 versões antes de passar, todas registradas em `estocologia/performativo.py`:
  - **v1:** a escala servia a demanda corrigida, o que gerava uma espiral de desistências e viés de 6,6%;
  - **v2:** razão de Wald, com viés de −4,8%;
  - **v3:** Wald com controle pré-choque, que contava a cascata de rediscagens e deu r̂ = 0,77;
  - **v4:** 2SLS.

  É ajuste de estimador num laço simulado depois de ver resultados, e é o ponto mais fraco do G4.

## Limites declarados

- **Escala.** O PLAN pede 10⁶ nós e monitoramento em tempo real. O que existe são 4,5 × 10⁵ previsões em replay
  sob relógio, não um serviço rodando.
- **Instantes.** A latência do boletim de ciclones (3 h) e o táxi dos voos (20 min) são suposições. O instante de
  cada detenção dentro do ano é sintético.
- **Performatividade.** É um laço declarado sobre chegadas reais, não uma medida do mundo.
- **Parâmetros supostos.** AHT, paciência e os limiares de C/L são suposições do F7.
- **Pareceres.** Os pareceres de risco esperam a assinatura de sil.

## O que falta para passar

Os dois fluxos que falharam pedem mecanismos que o desenho não tinha:
- **voos:** um disjuntor de calibração, isto é, um sensor de CITL móvel com recalibração da intercepção, ou um
  reajuste com mais peso no recente;
- **prisões:** um cofre maior, ou um critério de ECE com tolerância pelo tamanho da amostra.

As duas coisas mudam o desenho depois de ver o cofre. Precisariam de novos dados fora do tempo para serem
testadas honestamente: 2014 para os voos, o que exige o nycflights14, e anos posteriores a 2002 para as prisões.

## H3b: pendências (declarado em [CRITERIOS_H3b.md](CRITERIOS_H3b.md) antes de rodar)

**Voos com cofre novo: passa.** O mecanismo novo é a recalibração online do intercepto, p' = σ(logit p + δ) com
δ ← δ + 0,002 (y − p'). Ele foi testado num cofre que ninguém tinha visto: os 435 mil voos de NY em 2023
(nycflights23), com corte em 2023-10-01, de modo que o cofre é o mesmo trimestre que falhou em 2013. O modelo, as
features, o reserva e os sensores são os mesmos.

| | |p̄ − ȳ| | ECE | habilidade contra a climatologia |
|---|---|---|---|
| oficial, recalibrado (104 489 previsões) | **0,0015** | **0,008** | **+0,149** |
| modelo cru, em sombra | 0,012 | 0,014 | +0,144 |

- **O que a recalibração fez.** O δ variou de −0,52 a +0,43 ao longo do ano. Mas o 4º trimestre de 2023 não
  repetiu o desvio de 2013, e o modelo cru também teria passado. A recalibração melhorou a calibração e não foi
  decisiva para o portão.
- **Os outros portões também passam no fluxo novo.** G1: pytest e reprodução limpa idêntica. G3: zero leituras
  do futuro, zero estados do futuro, zero rótulos já conhecidos; canário honesto com ganho 0,000 e vazado com
  +0,85. G5: no máximo 1,8 varredura por tabela. A trilha está íntegra.
- **Mudança no motor.** O motor passou a registrar o hash do CRITERIOS_H3b.md no pré-registro do voos23. Isso
  muda o hash do motor: rodar de novo os fluxos do H3 cria uma entrada nova de K neles, sem mudar previsão
  nenhuma.

**Prisões: pendência aberta.** Nenhum dado de Toronto posterior a 2002 está ao alcance desta rede. Afrouxar o ECE
depois de ver o resultado não seria honesto. Por isso a saída do H3 continua NÃO aprovada por causa deste fluxo,
a menos que sil decida tratá-lo de outra forma.

## H3c: prisões com dados novos (declarado em [CRITERIOS_H3c.md](CRITERIOS_H3c.md) antes de rodar)

**Prisões de 2020–2021: falha no G2.** Os dados novos são as prisões do Toronto Police Service em 2020–2021
(RBDC-ARR-TBL-001, 65 276 prisões; `../dados-reais/h3c/FONTES.md`). O fluxo `prisoes20` prevê se a pessoa presa é
registrada na delegacia. O cofre é o 2º semestre de 2021, com 16 933 prisões. A pendência das prisões continua
aberta.

| cofre 2021-S2 | |p̄ − ȳ| | ECE | habilidade contra a climatologia |
|---|---|---|---|
| oficial, recalibrado | **0,0002** | **0,067** (tolerância 0,03) | **+0,43** |
| modelo cru, em sombra | 0,014 | 0,067 | +0,43 |

- **O que falhou.** O modelo acerta a média e separa bem os casos, mas é **pouco confiante**: nas caixas de
  baixo prevê 0,12 onde a taxa observada é 0,04, e nas de cima prevê 0,82 a 0,90 onde a taxa é 0,91 a 0,94. É um
  erro de inclinação, não de intercepto, e a recalibração do intercepto (H3b) não o corrige.
- **Causa provável (inferida, não testada).** A penalização da `logistica` é λ·n por coeficiente, aplicada a cerca
  de 60 indicadoras. Isso encolhe demais. Além disso, a polícia trocou o esquema de categorias de ocorrência em
  2021, e os coeficientes das categorias de 2020 deixaram de servir.
- **Nada foi ajustado depois de ver o cofre.** Corrigir a inclinação (por exemplo, com recalibração de Platt online)
  seria uma especificação nova. Ela só poderia ser testada num cofre que ninguém viu, e não há prisões de Toronto
  posteriores a 2021 ao alcance desta rede.
- **Os outros portões passam.**
  - G1: pytest (8) e reprodução limpa idêntica.
  - G3: zero leituras do futuro, zero estados do futuro, zero rótulos já conhecidos; o pré-registro vem antes do
    cofre; canário honesto com ganho 0,000 e vazado com +0,55.
  - G5: no máximo 2,35 varreduras por tabela.
  - A trilha está íntegra.
- **F6, informativo.** Em cada semestre, Black e White ficam a menos de 0,05 de distância em calibração, FPR e FNR.
  As taxas-base diferem pouco (0,54 contra 0,50–0,52 em três dos quatro semestres). É o oposto do que se viu em
  1997–2002, onde o FPR dos Black era o dobro.
- **F5, informativo.** O orçamento de ε foi respeitado, com 3 recusas do disjuntor de privacidade.

**O que foi mudado durante o desenvolvimento.** As duas mudanças vieram antes de abrir o cofre e não mudam o
modelo; estão relatadas aqui:
- O relógio passou a andar a cada 6 h, e não a cada evento. Com um tick por evento eram 130 mil leituras por
  tabela, e a execução não terminava. Os grupos de linhas passaram a ter 32 linhas.
- Na fase de desenvolvimento apareceu que o 1º trimestre de 2021 caiu quase todo no reserva. A causa é que a
  polícia renomeou e subdividiu as 14 categorias de ocorrência de 2020 em 21 categorias novas. A regra de escopo
  declarada fez o que devia, e não foi mudada; a partir do 2º trimestre de 2021 o modelo já conhece as categorias
  novas.

Arquivos: `fluxos/prisoes20.py`, `avaliar_h3c.py` (portões, escrito antes da execução completa),
`saida/prisoes20/`, `saida/portoes_h3c.json`.

## H3d: prisões em paradas de trânsito, Rhode Island (declarado em [CRITERIOS_H3d.md](CRITERIOS_H3d.md) antes de rodar)

**Passa. Com isto, a pendência das prisões fecha e a saída do H3 fica aprovada pela regra declarada.** O fluxo
`prisoes_ri` prevê a prisão do motorista em 83 mil paradas da polícia estadual de Rhode Island, entre 2005 e 2015. O
cofre é 2013–2015, com 24 466 paradas e taxa observada de 2,26%. A especificação é nova e incorpora a lição do H3c:
recalibração online de Platt (intercepto e inclinação) e λ = 1e-4.

| cofre 2013–2015 | |p̄ − ȳ| | ECE | habilidade contra a climatologia |
|---|---|---|---|
| oficial (Platt; reserva quando o disjuntor manda) | **0,0020** (0,09·ȳ; limites 0,02 e 0,2·ȳ) | **0,0048** (0,21·ȳ; limites 0,03 e 0,3·ȳ) | **+0,019** |
| modelo cru, em sombra | 0,013 (0,58·ȳ) | 0,013 (0,58·ȳ) | +0,004 |

- **O que fez a diferença.** O modelo cru superestima a taxa em 58% e teria falhado no critério relativo. A
  recalibração de Platt terminou com a = −0,30 e b = 1,07 e corrigiu o desvio. O disjuntor pôs o reserva como
  oficial em 12% do cofre.
- **Ressalva de substância.** A habilidade é pequena (+0,019): prisão numa parada de trânsito é um evento raro e
  pouco previsível pelas variáveis disponíveis. O portão mede calibração, e ela é boa; a resolução é baixa.
- **Os outros portões.**
  - G1: pytest e reexecução limpa idêntica.
  - G3: zero leituras do futuro, zero estados do futuro, zero rótulos já conhecidos; canário honesto com ganho 0,000
    e vazado com +0,97.
  - G5: 1,53 varredura por tabela. A trilha está íntegra.
- **Uma correção depois de abrir o cofre, só de leitura em disco.** Na primeira execução o G5 falhou com 6,4
  varreduras por tabela. Os grupos tinham 32 linhas, contra cerca de 5 paradas por tick de 6 h. Os grupos passaram
  a 4 linhas. O diário de previsões da segunda execução é **idêntico** ao da primeira (conferido com
  `DataFrame.equals`), então G2 e G3 não mudam. A mudança no módulo cria uma segunda entrada de especificação:
  K = 2, em `saida/prisoes_ri/especificacoes.jsonl`.
- **F6, informativo, nos anos do cofre.** A taxa de prisão dos motoristas Black é o dobro da dos White (3,0–3,8%
  contra 1,4–1,7%). Em 2013 e 2014, o FPR dos Black é quase o dobro do dos White (0,019 contra 0,011; 0,025 contra
  0,013), num modelo cego à cor. É o mesmo padrão de 1997–2002 em Toronto.
- **A pergunta não é idêntica às anteriores:** aqui é prisão em parada de trânsito; antes, a liberação depois da
  prisão. Isso fica registrado para sil.

Arquivos: `fluxos/prisoes_ri.py`, `avaliar_h3d.py`, `saida/prisoes_ri/`, `saida/portoes_h3d.json`.

### Confirmação de sil (2026-10-15T04:08Z)

sil confirmou que a pergunta do H3d (prisão do motorista na abordagem de trânsito) substitui a pergunta de custódia e
liberação. **A saída do H3 fica aprovada sem pendência.**
