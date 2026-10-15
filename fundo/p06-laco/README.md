# Ponto de fundo 6: o laço performativo do H3 medido em dados reais

## Resultado

Não há operação real ao alcance daqui. O laço do H3 continua sem medida numa operação que a Estocologia
controle. O que deu para fazer foi medir o mesmo mecanismo numa central real, a do AnonymousBank (banco
israelense, todas as chamadas de 1999), com um desenho quase experimental e não com um sorteio. O desenho usa a
fila encontrada ao entrar, dentro da mesma data e hora, como instrumento para a desistência. Por esse desenho,
desistir da fila em vez de ser atendido aumenta em **0,145** (IC 90% por bootstrap de dias de 0,132 a 0,157) a
probabilidade de o mesmo cliente ligar de novo em até 15 min. O H3 declarou **r = 0,6** nessa mesma janela.
**O sinal confere e a ordem de grandeza não: o laço do H3 superestima o medido cerca de 4 vezes** (0,6 / 0,145
= 4,1, e o critério declarado pedia razão entre 1/3 e 3). O excesso de chegadas que o laço medido produz nesta
central é de 2,3% das chegadas, contra o viés de 29% que o laço declarado gerou no H3. Isso significa que a
correção performativa do G4 foi testada contra um laço bem mais forte que o observado num caso real. Há duas
ressalvas de peso. Uma nova chamada só é visível quando entra na fila, então 0,145 é provavelmente um limite
inferior. Uma correção grosseira, feita depois de ver o resultado, leva o valor a 0,16, ainda longe de 0,6. E a
central é de outro banco, de outro país e de outro ano que os da central do H3.

## O que o H3 declarou

`h3/estocologia/performativo.py` monta o laço sobre as chegadas reais do `bank_calls`. A escala sai da previsão
pelo Erlang C. A desistência segue o M/M/c+M com paciência de 180 s, suposta. **Quem desiste rediscam com
probabilidade r = 0,6**, 60% no intervalo de 5 min seguinte e 40% no outro (`DIV = (0.6, 0.4)`, linha 36).
Quem é atendido não volta. No laço simulado, o 2SLS com os choques sorteados recuperou r̂ = 0,563, com IC 90% de
0,51 a 0,61, e o previsor ingênuo superestimou a demanda exógena em 29,1% (`h3/saida/portoes.json`, linhas
276 e 283). O próprio README do H3 diz que é "um laço declarado sobre chegadas reais, não uma medida do mundo".

O parâmetro medido aqui é o mesmo r: o efeito causal de desistir, contra ser atendido, sobre a chance de uma
nova chamada logo depois. No H3 ele é igual a P(nova chamada | desistiu), porque P(nova chamada | atendido) = 0.

## Busca do dado

O que procurei, com o que a rede alcança (raw.githubusercontent.com, busca de código no GitHub, índice do
Rdatasets):

- **Índice do Rdatasets (3 648 conjuntos).** Busquei call, queue, wait, abandon, redial, energy, electric,
  nudge, disclos, experiment, field, recommend, lottery e random. Achei experimentos de campo com informação ou
  intervenção (por exemplo `causaldata/thornton_hiv` e vários do `experimentr`) e séries de chamadas
  agregadas (`fpp2/calls`, a mesma família do `bank_calls` do H3). Nenhum tem uma previsão ou ação publicada
  que mude o comportamento previsto no canal do H3, que é escala → espera → desistência → nova chamada.
- **Busca de código no GitHub** por `vru_entry q_time outcome HANG AGENT`. Ela achou espelhos do AnonymousBank
  1999 do Technion SEE Lab, a base clássica de central com registro por chamada e identificação do cliente.
  Esse é o único dado alcançável com o mecanismo exato do H3.
- **Não tentei** registros de recomendação com exposição sorteada (Open Bandit Dataset, KuaiRand) nem
  experimentos de normas sociais em energia (Opower, que não é público). Eles medem outro laço, que não é o
  declarado no H3.
- **Não achei** nenhum dado de central com escala variada por sorteio, nem com tempo de espera anunciado
  sorteado.

O dado foi baixado de dois espelhos independentes e conferido entre si (`carregar.py`). Cada um tem 444 448
linhas, e 444 436 pareiam pela chave. Depois de normalizar os zeros à esquerda, os campos coincidem em mais de
99,98%. A origem, os hashes e os avisos estão em
`/mnt/project-files/estocologia/dados-reais/fundo/p06-laco/FONTES.md`.

## Desenho (pré-registrado em [DECLARACAO.md](DECLARACAO.md), SHA-256 em `DECLARACAO.sha256`)

- **Chamada índice:** chamada identificada que entrou na fila, desfecho AGENT ou HANG, tipo diferente de TT.
  São 198 980 chamadas, 36 658 delas desistências, de 12 511 clientes em 360 dias.
- **Tratamento:** desistiu, contra foi atendido.
- **Instrumento:** número de outras chamadas na fila no instante em que a chamada índice entra nela. A média é
  3,1, a mediana 2 e o q90 7.
- **Efeitos fixos:** data × hora × prioridade × tipo de serviço. Dentro da célula, a fila encontrada vem das
  chegadas e dos atendimentos de terceiros.
- **Desfecho:** outra chamada do mesmo cliente em até 15 min depois do fim da chamada índice. As janelas de
  60 min e do mesmo dia são secundárias.
- **Margem:** o dia é a unidade independente. Bootstrap de 1 000 dias com `default_rng(2026)`.

Isto é um do(desistência) aproximado por variação quase aleatória, não um sorteio. Ao lado vai a leitura
passiva, MQO com os mesmos efeitos fixos, que é a comparação que o AGENTS.md proíbe confundir com efeito.

## Números (`resultados.json`)

| | efeito sobre P(nova chamada) | IC 90% (dias) | MQO passivo | F do 1º estágio |
|---|---|---|---|---|
| **principal, 15 min** | **0,145** | 0,132 a 0,157 | 0,129 | 5 523 |
| robustez 1: controle da fila nos 15 min depois | 0,134 | 0,121 a 0,146 | 0,126 | 4 413 |
| robustez 2: instrumento log(1 + fila) | 0,142 | 0,129 a 0,153 | 0,129 | 3 128 |
| secundária, 60 min | 0,218 | 0,200 a 0,235 | 0,165 | 5 523 |
| secundária, mesmo dia | 0,262 | 0,241 a 0,281 | 0,151 | 5 523 |
| **H3 declarado** | **0,6** | (r̂ simulado 0,563, 0,51 a 0,61) | | |

**Critério declarado:**
- **Sinal.** O IC fica todo acima de 0, então o sinal confere.
- **Ordem de grandeza.** A razão 0,6 / 0,145 é 4,1, fora da faixa de 1/3 a 3, então a ordem não confere.
- **Compatibilidade estrita.** O valor 0,6 fica fora do IC 90%.
- **Robustez.** A robustez 1 não muda o sinal nem a faixa, então o resultado não é frágil.
- **Veredito declarado:** "o laço do H3 superestima o medido".

**Taxas cruas.** Em 15 min, 21,2% dos desistentes ligam de novo, contra 7,2% dos atendidos. Em 60 min são
33,8% contra 15,4%, e até o fim do dia 49,8% contra 32,9%. Quem é atendido também volta, coisa que o H3 supõe
que não acontece. A diferença crua em 15 min (0,140) fica perto da estimativa por instrumento (0,145). Aqui a
seleção pela paciência pesa pouco nessa janela, mas pesa no mesmo dia, onde o MQO dá 0,151 e o IV 0,262.

**Secundárias declaradas:**
1. Das novas chamadas em 60 min dos desistentes, 63% chegam em até 15 min. O H3 supõe 100% em dois intervalos.
2. Com r = 0,145, as 63 203 desistências na fila sobre 405 358 pedidos de atendente geram um excesso de 2,3%
   de chegadas observadas (IC de 2,1% a 2,5%). No H3, o laço declarado produziu um viés de 29,1% do previsor
   ingênuo. Os dois números não medem exatamente a mesma coisa, porque o do H3 depende também da taxa de
   desistência simulada. Mesmo assim, a distância é de uma ordem de grandeza.

**Informativo, feito depois de ver o resultado (`posthoc.py`, `posthoc.json`; não decide).** Pedidos atendidos
sem fila quase nunca têm identificação: a fração é 0,00002, contra 0,84 para quem passou pela fila. Nos 15 min
depois de uma desistência, 88,8% dos pedidos entram na fila. Se as novas chamadas entrassem na fila nessa mesma
proporção, o efeito corrigido seria 0,145 / 0,888 ≈ 0,16.

## Como reproduzir

```
python3 -I fundo/p06-laco/carregar.py   # confere os dois espelhos
python3 -I fundo/p06-laco/estimar.py    # resultados.json (~35 s)
python3 -I fundo/p06-laco/posthoc.py    # posthoc.json (informativo)
sha256sum -c fundo/p06-laco/DECLARACAO.sha256   # rode de dentro de fundo/p06-laco
```

Os dados ficam em `/mnt/project-files/estocologia/dados-reais/fundo/p06-laco/`, com `SHA256SUMS`. O
`estimar.py` rodou duas vezes, e as duas saídas `resultados.json` são idênticas byte a byte.

## Limites

- **Não é sorteio.** A exclusão supõe que a fila encontrada só muda a chance de nova chamada pela desistência.
  Há pelo menos dois caminhos fora dela. Quem é atendido depois de uma espera longa pode voltar mais. E a fila
  persiste, o que faz uma nova chamada ser vista com mais frequência quando a fila estava longa. A robustez 1
  controla o segundo caminho em parte e baixa a estimativa um pouco, de 0,145 para 0,134.
- **Medida.** Só se vê uma nova chamada que entra na fila, e só de clientes identificados. Clientes não
  identificados (customer_id = 0) e os 300 identificadores corrompidos ficam fora.
- **Validade externa.** É outro banco (Israel, 1999, cerca de 8 posições de atendimento) e não a central do
  `bank_calls` (EUA, 2003). Isso informa a ordem de grandeza plausível de r, mas não o r daquela central.
- **Estimando local.** O 2SLS mede o efeito entre quem desiste por causa da fila mais longa. É o grupo que
  importa para a escala, mas não é toda desistência.
- **Paciência e timing.** A paciência de 180 s suposta no H3 não foi testada aqui.

## O que medir numa operação real (o que sil precisaria ter)

Para medir o laço de verdade, como o AGENTS.md pede, com do(a) contra observação passiva:

1. **Acesso de escrita à escala**, ou a um parâmetro equivalente como roteamento, transbordo ou tempo de
   espera anunciado. Com isso, sortear, em 5% a 10% dos intervalos de 15 min, um desvio de ±15% a ±30% na
   escala planejada. O sorteio é registrado antes, com semente e hash, como no G4 simulado.
2. **Registro por chamada com identificador estável do chamador** (ANI ou cliente) em todas as chamadas,
   atendidas com ou sem fila, porque o defeito do AnonymousBank é justamente não ter isso. Também registrar a
   entrada e a saída da fila e o desfecho.
3. **Registro das ações do sistema:** escala planejada, escala efetiva e previsão emitida, com o instante de
   cada uma, para cada intervalo.
4. **Estimação.** O 2SLS do `performativo.py` (v4) com os choques sorteados como instrumento e o dia como
   unidade, ao lado do mesmo r medido no nível individual. A comparação desse r com o desta medida, 0,15 a 0,26
   conforme a janela, e com o 0,6 declarado diz qual laço vale para aquela central.
5. **Tamanho.** No AnonymousBank, com cerca de 550 chamadas índice por dia (198 980 em 360 dias), o IC de 90% do efeito tem cerca
   de ±0,013 com 360 dias. Um sorteio de escala diretamente na operação deveria ser mais forte que a variação
   natural da fila. A potência para um dado número de dias tem de ser calculada no pré-registro da operação,
   não suposta aqui.
