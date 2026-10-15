# Declaração (ponto de fundo 6): o laço performativo do H3 medido em dados reais

Escrita antes de qualquer estimativa do efeito. O hash SHA-256 deste arquivo fica em `DECLARACAO.sha256`.
Qualquer mudança depois de ver resultados entra como nova declaração e conta no K.

## O que o H3 declarou

Em `h3/estocologia/performativo.py`, o laço da central é declarado, não medido. A escala sai da previsão pelo
Erlang C, quem espera demais desiste (paciência exponencial de 180 s, suposta), e **quem desiste rediscam com
probabilidade r = 0,6**, 60% no intervalo de 5 min seguinte e 40% no outro (`DIV = (0.6, 0.4)`, linha 36).
Quem é atendido não liga de novo. O r̂ estimado por 2SLS no laço simulado foi 0,563, IC 90% por bootstrap de
dias de 0,51 a 0,61 (`h3/saida/portoes.json`, linhas 270–277). O previsor ingênuo superestimou a demanda
exógena em 29,1% (`portoes.json`, linha 283).

Portanto o parâmetro do laço é: **o efeito causal de desistir da fila, contra ser atendido, sobre a
probabilidade de uma nova chamada do mesmo cliente logo depois**. No H3 esse efeito é r = 0,6, porque quem é
atendido tem probabilidade 0 de voltar.

## Dado

AnonymousBank, central telefônica de um banco israelense, todas as chamadas de 1999 (Technion SEE Lab, Guedj e
Mandelbaum). Cada linha é uma chamada com `customer_id`, instantes de entrada na fila, saída da fila e
atendimento, e o desfecho (AGENT, HANG, PHANTOM). Baixado de dois espelhos independentes no GitHub, conferidos
entre si (`carregar.py`): 444 448 linhas em cada um, 444 436 pareadas pela chave, e todos os campos iguais
depois de normalizar os zeros à esquerda. Detalhes em
`/mnt/project-files/estocologia/dados-reais/fundo/p06-laco/FONTES.md`.

### O que olhei antes de declarar (estrutura, não o efeito)

- Desfechos: AGENT 353 143, HANG 87 706, PHANTOM 3 599.
- Por um defeito do sistema registrado na documentação, o `customer_id` quase não é gravado para quem não
  entrou na fila: só 5 632 de 157 137 atendidos sem fila têm identificação. Entre quem entrou na fila, têm
  identificação 163 920 de 196 006 atendidos e 37 138 de 64 006 desistentes.
- 300 linhas têm `customer_id` em notação científica (por exemplo 3.27E+11), corrompido nos dois espelhos.
- Chamadas na fila, identificadas, com desfecho AGENT ou HANG: 201 058, de 12 525 clientes, em 360 dias.
- Não calculei nenhuma taxa de nova chamada, nem por grupo nem no total.

## Desenho

**Não há sorteio.** A central não variou a escala ao acaso, e a base não registra a escala. O melhor desenho
disponível é quase experimental: a fila que o cliente encontra ao entrar. Dentro da mesma data e da mesma hora,
o número de pessoas à frente na fila depende de quantos outros clientes chegaram nos minutos anteriores e de
quanto duraram os atendimentos em curso. Isso é ruído de Poisson de terceiros e não deveria depender da
paciência nem da necessidade deste cliente. A fila mais longa aumenta a chance de desistir. É o mesmo canal da
ação do H3: menos atendentes do que a carga pede significa fila mais longa.

- **Unidade de análise:** a chamada índice, uma chamada identificada (`customer_id` diferente de 0 e sem
  notação científica) que entrou na fila (`q_start` > 0), com desfecho AGENT ou HANG, tipo diferente de TT
  (TT é uma chamada de retorno iniciada pelo banco).
- **Tratamento D:** desistiu (HANG) = 1, atendido (AGENT) = 0.
- **Instrumento Z:** número de outras chamadas na fila no instante `q_start` da chamada índice (contadas em
  todas as chamadas da base, identificadas ou não, com `q_start` ≤ t < `q_exit`), excluída a própria.
- **Desfecho Y_W:** existe outra chamada do mesmo `customer_id`, não TT e não PHANTOM, com `vru_entry` no
  intervalo (fim da chamada índice, fim + W], em que o fim é `q_exit` para quem desistiu e `ser_exit` para
  quem foi atendido. W principal = 15 min, que cobre os dois intervalos de 5 min do H3 contados de qualquer
  ponto do intervalo da desistência. W secundários: 60 min e até o fim do mesmo dia.
- **Controles (efeitos fixos):** data × hora de `q_start`, prioridade (0/1 contra 2) e tipo de serviço.
- **Estimador principal:** 2SLS com os efeitos fixos absorvidos por centragem dentro das células
  (data × hora × prioridade × tipo), uma equação, um instrumento.
- **Comparação passiva (o que o AGENTS.md proíbe confundir):** a diferença por MQO de Y_W entre desistentes e
  atendidos com os mesmos efeitos fixos. Ela mistura o efeito da ação com a seleção (quem desiste pode ser
  quem tinha menos urgência ou mais pressa).
- **Margem:** o dia é a unidade independente, como no H3. IC 90% por bootstrap de dias (1 000 réplicas,
  `np.random.default_rng(2026)`), percentil.
- **Força do instrumento:** F do primeiro estágio com erro robusto por dia; se F < 10, o 2SLS é relatado como
  fraco e o critério não se decide por ele.
- **Robustez declarada (não decide):**
  1. a mesma 2SLS com um controle a mais: a fila média de outras chamadas nos 15 min depois do fim da
     chamada índice. Serve para o problema de medida abaixo. A fila depois é posterior ao tratamento, mas uma
     única chamada muda pouco a fila de todos;
  2. o instrumento trocado pelo log(1 + Z).

## Problema de medida declarado

Uma nova chamada só é vista se for identificada, e quase só é identificada se entrar na fila. Uma nova
chamada atendida sem espera fica invisível. Por isso Y_W mede "nova chamada que encontrou fila", e a taxa
medida é um limite inferior da taxa real de nova chamada nos dois grupos. Como a fila persiste no tempo, um Z
alto também aumenta a chance de a nova chamada ser vista, para desistentes e atendidos. Isso pode violar a
exclusão. A robustez 1 tenta controlar esse caminho, mas não o elimina.

## Métricas e critério

Pergunta: **o laço declarado no H3 tem o sinal e a ordem de grandeza do medido?**

Com r_IV a estimativa 2SLS principal (W = 15 min) e IC 90% por bootstrap de dias:

- **Sinal confere** se o IC 90% de r_IV fica inteiro acima de 0.
- **Ordem de grandeza confere** se 0,6 / r_IV está entre 1/3 e 3, isto é, se r_IV está entre 0,2 e 1,8.
- **Compatível no sentido estrito** se 0,6 está dentro do IC 90% de r_IV (relatado, não decide).
- **Veredito:**
  - "o laço do H3 tem o sinal e a ordem do medido" se as duas primeiras condições valem e F ≥ 10;
  - "o laço do H3 superestima (ou subestima) o medido" se o sinal confere mas a ordem não;
  - "não confere" se o IC 90% inclui 0 ou fica abaixo de 0;
  - "indecidido" se F < 10.

## Métrica secundária (relatada, não decide)

1. A fração das novas chamadas em 60 min que chega em até 15 min, entre desistentes. O H3 supõe 100%.
2. O excesso de chegadas observadas causado pelo laço: r_IV × (desistências na fila) / (pedidos de atendente
   − r_IV × desistências na fila), com pedidos de atendente = chamadas que entraram na fila ou foram atendidas
   sem fila. No H3, o análogo é o viés do previsor ingênuo contra a demanda exógena, 29,1%.
3. As taxas cruas de Y_W por grupo e a diferença por MQO (a leitura passiva).

## O que conta como falha desta tentativa

- Se o F do primeiro estágio for menor que 10.
- Se a robustez 1 mudar o sinal de r_IV ou tirá-lo da faixa 0,2–1,8 em que a principal cair, o resultado
  principal é relatado como frágil.
- Este desenho é quase experimental. Mesmo que passe, ele não substitui a medida numa operação real com
  variação sorteada da escala, que fica proposta no README.
