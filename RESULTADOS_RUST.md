# Resultados em Rust

Registro colhido direto dos instrumentos, em 2026-10-16. Nada aqui e transcricao: cada bloco e a
saida literal do comando acima dele. Quem duvidar de um numero roda o comando e compara.

| Instrumento | O que ele mede |
| --- | --- |
| `labs --cap N` | o laboratorio do capitulo N do livro, com semente fixa |
| `labs --todos` | os 21 laboratorios em sequencia |
| `labs --banco` | o H0 real: a base de 2,65 milhoes de chamadas, escala de Erlang C, unidade independente e nulo de metades |
| `labs --resolucao` | o horizonte R: quanto custa perguntar grosso, com o mesmo recurso total |
| `labs --s1` | a triagem deterministica do Sistema 1 |
| `labs --s2` | o Sistema 2: a busca sobre o hipergrafo, a crenca da banca, a inferencia e o eixo R |
| `labs --preteste` | o nulo do pre-teste de calibracao do painel SHELF |
| `labs --genetica` | a resolucao da validacao por intervencao: o que a segunda linha muda |
| `labs --escuta` | o lacre das sete decisoes que mexem com pessoas: quem foi ouvido |
| `labs --h3` | a cadeia do H3: o que se refaz, o que se re-deriva e o que so resta declarar |
| `labs --ponte` | a tese da ponte em escala: o piloto re-derivado e o portao do teste justo |
| `labs --schema` | o contrato da ficha: a versao e a cadeia de migracoes em arquivo |
| `labs --segunda` | a segunda opiniao dos resultados marcados P |
| `labs --jev` | a ponte Jev-LLM: os portoes da ponte e o que so fecha com os pesos |
| `labs --fila` | a fila com desistencia e as quatro regras de escala (R0 a R3) |
| `portoes` | os sete portoes deterministicos e a banca adversarial A1-A8 |
| `cargo test --workspace` | os testes dos quinze crates |

## 1. O H0 real, em Rust

### A central de chamadas do banco (Capitulo 16)

Le `dados-reais/bank_calls.csv` (a base publica de 5 em 5 minutos), monta a escala de Erlang C sobre o
perfil medio do treino e mede o que a mesma escala faz nos dias do cofre. Declaracao de divergencia contra
o README do H0 real: a `Var/media` de 273 que o README publica sai de 272,6 no treino (este instrumento
mede 246,7 no cofre e 261,1 na serie toda); o nulo de metades por dia e 0,0441 contra 0,0474 (esquema de
reparticao diferente); e o nulo por chamada e 0,0006 pela binomial, contra os 0,0033 publicados, que
usavam outro desenho de correlacao intradiaria.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --banco
~~~

~~~text
== H0 real :: a central de chamadas do banco, em Rust ==
treino 83 dias (mar-jun) | cofre 81 dias (jul-out) | 2649354 chamadas no cofre
escala do perfil medio: 234 agentes no pico, 2259 agente-horas por dia
nivel de servico no perfil medio = 0.8193 | media dos dias do cofre = 0.5683 | dias abaixo da meta de 80% = 0.6543
dispersao entre dias: Var/media = 246.7 no cofre, 272.6 no treino, 261.1 em todos | desvio entre dias = 2841 | Poisson preveria 181
nulo de metades: por dia = 0.0441 | por chamada = 0.0006
~~~

### A fila com desistencia e as quatro regras de escala (R0 a R3)

Le a base do banco, reconstroi a fila com desistencia por dois motores independentes (o exato sobre as
chegadas observadas e o de eventos com chegadas de Poisson) e mede as quatro regras de escala. Os
agente-horas batem com o registro; os niveis e as falhas por dia divergem para baixo e para cima, e a
divergencia fica declarada no Capitulo 16.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --fila
~~~

~~~text
== H0 real :: a fila com desistencia e as quatro regras de escala (R0 a R3) ==
treino 83 dias (mar-jun) | cofre 81 dias (jul-out) | 2654950 chamadas no cofre
atendimento exponencial 240 s | paciencia exponencial 180 s | alvo 20 s | meta 80% | semente 2003 | 3 replicas por dia
CV residual do dia 0.0598 | margem de R2 = 1 + 1.2816 x CV = 1.0767 | R3 = R0 + 12 atendentes por igual
dias no treino por dia da semana [16, 17, 17, 17, 16, 0, 0]

motor Exato (chegadas observadas, FCFS com desistencia) — a reconstrucao
regra  agente-horas/dia   pico  nivel de servico    dias abaixo da meta     desistencia/100  espera media s espera med. s espera p90 s
R0                 2264    234             83.9%                  30.9%                 8.4              14            0           0
R1                 2261    267             84.4%                  19.8%                 8.1              14            0          44
R2                 2428    287             91.6%                   4.9%                 4.2               7            0           0
R3                 2433    246             90.3%                   9.9%                 5.0               8            0           0

motor Eventos (M/M/c+M evento a evento, chegadas de Poisson) — verificacao independente
regra  agente-horas/dia   pico  nivel de servico    dias abaixo da meta     desistencia/100  espera media s espera med. s espera p90 s
R0                 2264    234             84.4%                  28.4%                 4.3               7            0           6
R1                 2261    267             87.5%                  16.0%                 3.7               6            0          14
R2                 2428    287             95.0%                   4.9%                 1.6               3            0           1
R3                 2433    246             92.4%                   8.6%                 2.2               4            0           0
~~~

### O Sistema 1: triagem deterministica de fichas

Vocabulario fechado de 6 categorias e 35 temas, arvore de sufixos de 136 entradas, probabilidade presa em
[0,01; 0,99] e soma conferida a cada rodada. Sem casamento, a resposta e a priori declarada de outros.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --s1
~~~

~~~text
== Sistema 1 :: triagem determinística de fichas ==
arvore de sufixos: 136 sufixos indexados em 6 temas declarados
categoria=fila probabilidade=0.8988 produtos=[0.8988, 0.0202, 0.0202, 0.0202, 0.0202, 0.0202] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.8988 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=causal probabilidade=0.9361 produtos=[0.0128, 0.9361, 0.0128, 0.0128, 0.0128, 0.0128] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9361 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=cauda probabilidade=0.9117 produtos=[0.0177, 0.0177, 0.9117, 0.0177, 0.0177, 0.0177] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9117 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=calibracao probabilidade=0.9361 produtos=[0.0128, 0.0128, 0.0128, 0.9361, 0.0128, 0.0128] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9361 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=justica probabilidade=0.9117 produtos=[0.0177, 0.0177, 0.0177, 0.0177, 0.9117, 0.0177] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9117 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=outros probabilidade=0.8718 produtos=[0.0256, 0.0256, 0.0256, 0.0256, 0.0256, 0.8718] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.8718 | soma 1.0000 | fora de [0.01;0.99] 0
~~~

### O pre-teste de calibracao do painel SHELF

O nulo declarado: um especialista perfeitamente calibrado, com a priori uniforme, no desenho do
protocolo. Mede o que o limiar de ECE de 0,10 aprova e mostra os dois limites do desenho (o criterio 3
vacuoso com 20 itens em 10 caixas e o criterio 4 inerte como escrito).

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --preteste
~~~

~~~text
== Pré-teste SHELF :: calibração do painel ==
desenho do protocolo: 20 itens, 10 caixas de massa igual; limiar do ECE 0.1
limites medidos do desenho: o critério 3 (caixa com n >= 4) fica vacuoso, porque 20 itens em 10 caixas dão 2 por caixa; o critério 4, na letra do protocolo, é inerte: as caixas são cortadas por p ordenado e a média prevista da de cima supera a da de baixo por construção
nulo calibrado, 20 itens em 5 caixas, limiar de ECE 0.25: ECE p50 0.1584 | p90 0.2316 | p95 0.2559 | maximo 0.4487 | habilidade p50 0.3076 | p10 0.0370 | Brier p50 0.1645
    veredito do portao no mundo calibrado: aprovados 0.2935 | reprovados pelo criterio 3 0.7052 | pelo criterio 3 corrigido (binomial) 0.0031 | pelo criterio 2 0.0592 | por algum criterio 0.7065
nulo calibrado, 20 itens em 10 caixas, limiar de ECE 0.1: ECE p50 0.2272 | p90 0.3002 | p95 0.3218 | maximo 0.5474 | habilidade p50 0.3076 | p10 0.0370 | Brier p50 0.1645
    veredito do portao no mundo calibrado: aprovados 0.0008 | reprovados pelo criterio 3 0.0000 | pelo criterio 3 corrigido (binomial) 0.0007 | pelo criterio 2 0.9992 | por algum criterio 0.9992
nulo calibrado, 200 itens em 10 caixas, limiar de ECE 0.1: ECE p50 0.0694 | p90 0.0932 | p95 0.1002 | maximo 0.1470 | habilidade p50 0.3316 | p10 0.2583 | Brier p50 0.1663
    veredito do portao no mundo calibrado: aprovados 0.8729 | reprovados pelo criterio 3 0.0995 | pelo criterio 3 corrigido (binomial) 0.0179 | pelo criterio 2 0.0514 | por algum criterio 0.1271
~~~

### O Sistema 2 deliberativo

LATS contra o ganancioso no grafo declarado da amostra, a crenca da banca medida em ataques ate o
primeiro reprovado, a posterior da Beta(4, 8) contra a forma fechada (1/3) e a resolucao escolhida
pelo proprio instrumento do eixo R. O no que nenhuma lente cobre e a frente declarada e nao
implementada (f1.sintese-de-codigo).

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --s2
~~~

~~~text
== Sistema 2 :: deliberação sobre o hipergrafo ==
grafo declarado: 8 nos, 6 arestas de arvore, 2 de grafo, 5 hiperarestas | validacao ok
nos que nenhuma lente cobre: ["f1.sintese-de-codigo"]
ganancioso: para em h0.fila (valor 0.600) | LATS: raiz.estocologia -> r.resolucao -> f1.sintese-de-codigo (valor 1.000, no sem lente true)
crenca da banca: faixa mais provavel A5-governanca com probabilidade 0.9205 | entropia 0.4324
estresse: deliberado parou em A5-governanca com 1 ataque(s) | ordem declarada parou em A5-governanca com 5 ataques | mesma reprovacao true
inferencia: Beta(4, 8) media da cadeia 0.3346 contra a forma fechada 0.3333 | aceitacao 0.676
eixo R: perda do agregado 46.24 min contra margem de 12,00 min -> R1-dia (a perda do agregado paga a margem do dia) | fora do alcance medido: R2-hora, R3-subpergunta
eixo R sem oscilacao: perda 0.00 min -> R0-agregada
~~~

### A resolucao da validacao por intervencao (genetica de levedura)

O desenho da replica: 40 mutantes, um deles o mover verdadeiro (o efeito aparece nas duas linhas) e
outro o artefato de linha (o efeito aparece numa linha so). O mesmo mundo e validado com uma linha por
mutante e com duas, e o que se mede e quantos mutantes passam e qual e o piso do desenho em cada arranjo.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --genetica
~~~

~~~text
== Capítulo 19 :: a resolução fina da validação por intervenção ==
desenho: 40 mutantes | 60 individuos por linha | 999 sorteios por teste | alfa = 0.05 | Bonferroni alfa/40 = 0.001250
plantados pelo instrumento: mover verdadeiro = mutante 7 | artefato de linha = mutante 23

-- a validacao de hoje: cada mutante e um ambiente de uma linha so --
  40 mutantes x 60 individuos | piso do desenho = 0.001000 | limiar = 0.001250
  mutante  7 | efeito  +0.940 | p  0.001000 | passa no piso <- mover verdadeiro (efeito nas duas linhas)
  mutante 14 | efeito  -0.450 | p  0.001000 | passa no piso
  mutante 23 | efeito  +0.813 | p  0.001000 | passa no piso <- artefato de linha (efeito numa linha so)
  mutante 29 | efeito  +0.496 | p  0.001000 | passa no piso
  passam: 4 de 40

-- o mesmo mundo, com a segunda linha: o efeito tem de estar nas duas --
  40 mutantes x 2 linhas x 60 individuos | piso do desenho = 0.000001 | limiar = 0.001250
  mutante  7 | efeito l1  +0.456 | efeito l2  +1.016 | faixa  0.560 | p  0.000001 | passa <- mover verdadeiro (efeito nas duas linhas)
  mutante 23 | efeito l1  +0.607 | efeito l2  +0.028 | faixa  0.579 | p  0.000831 | rejeitado <- artefato de linha (efeito numa linha so)
  passam: 1 de 40
  efeito na segunda linha: artefato  +0.028 (nao se reproduz) | mover  +1.016 (se reproduz) | na validacao de uma linha a estimativa e um ponto e a faixa e zero

leitura: com uma linha o piso de 0.001000 e o limiar de 0.001250 deixam passar 4 dos 40 mutantes — o mover verdadeiro, o artefato de linha e dois mutantes que sao so ruido. Todos encostam no chao do desenho, e o p nao distingue nenhum dos tres do que se procura. Com a segunda linha o artefato sai da lista, porque o efeito dele na segunda linha e zero, e o piso cai para 0.000001000: passa 1 dos 40 mutantes. O p nunca foi o problema: o problema era a linha ser uma so.
~~~

### A cadeia do H3, re-executada ate onde a arvore deixa

Le o registro do ponto 14 (`fundo/p14-h3/registro.tsv`) com as quatro linhas do H3 e da a classe de cada
uma: medida (o dado bruto esta na arvore), derivada (o numero esta num arquivo versionado e a conta e o
veredito sao refeitos) e registro (o dado bruto saiu da arvore). Re-deriva o laco performativo do
`resultados.json` do ponto 6 e mostra as provas negativas: um registro em que a razao cai dentro da banda
declarada muda o veredito, e um registro em que o campo da razao nao bate com a conta e reprovado.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --h3
~~~

~~~text
== Ponto 14 :: a cadeia do H3, re-executada ate onde a arvore deixa ==
carimbo do registro: 2026-10-16
linha            classe    marca registro versionado                o que o instrumento faz
h3.fluxos        registro  P    -                                  declara: o dado bruto nao esta na arvore
h3.ri            registro  D    -                                  declara: o dado bruto nao esta na arvore
h3.laco          derivada  V    fundo/p06-laco/resultados.json     re-deriva a conta e o veredito
h3.hipergrafo    registro  P    -                                  declara: o dado bruto nao esta na arvore

-- o laco performativo, re-derivado do registro em arvore --
estimativa 2SLS: r = 0.1446378859 | IC90 de 0.1319811884 a 0.1568385251 | F do primeiro estagio = 5523.46
razao r declarado / r medido: 4.1482907207 (o registro traz 4.1482907207) | banda declarada: 0.3333 a 3.0000
sinal confere: true | ordem de grandeza confere: false | compativel no sentido estrito: false (o registro diz false)
vies do previsor ingenuo contra a demanda exogena: 0.2912
divergencias entre a conta e o registro: nenhuma
veredito: o laco do H3 superestima o medido: o sinal confere e a ordem nao (razao 4.1483, banda de 0.3333 a 3.0000)

-- prova negativa: um registro em que a razao cai dentro da banda declarada --
  com r medido 0,4: ordem confere = true | veredito: o laco do H3 tem o sinal e a ordem do medido (razao 1.5000 dentro da banda)
-- prova negativa 2: um registro em que o campo da razao nao bate com a conta --
  divergencias: ["a razao do registro (2.000000) nao bate com a conta (4.148291)", "a contencao do registro (true) nao bate com a conta (false)"]
~~~

### A ponte Jev-LLM

O portao dos pesos (sem o arquivo ou com o sha256 diferente, a ponte nao roda), a decodificacao
restrita ao esquema da ficha com a abstencao declarada, a prova de determinismo na forma do G1 nos
dois modelos de ensaio, o custo por chamada e a adequacao do espaco de hipoteses com o criterio de 30%.
Os pesos congelados nao estao na arvore, e por isso nenhum numero da ponte e publicado.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --jev
~~~

~~~text
== Ponto 20 :: a ponte Jev-LLM ==

-- o portao dos pesos --
  RECUSADO: pesos ausentes em fundo/p20-jev/pesos/modelo.congelado | abstencao Outros | sem os pesos, a ponte nao roda e nenhum numero sai

-- a decodificacao restrita --
  o modelo de ensaio responde no esquema: ficha jev.ponte.1 | camada L2 | palavras do documento: 92
  texto livre recusado: ForaDoEsquema("LinhaSemSeparador(\"isto e texto livre, nao uma ficha\")") | abstencao Outros
  margem em 100% recusada: ExtremoDogmatico("margem 1") | abstencao Outros

-- a prova de determinismo, na forma do G1 --
  modelo de ensaio: execucao 1 8813a29826c09a52 | execucao 2 8813a29826c09a52 | identicas: true
  modelo instavel: execucao 1 832a7ff86889326c | execucao 2 3495fa30b037ce75 | identicas: false
  leitura: o portao G1 fecha com um modelo congelado e NAO fecha com um modelo que muda de resposta — e o que o INTEGRACAO_JEV.md declara sobre o LLM livre

-- o custo por chamada e a adequacao do espaco --
  modelo de ensaio: 8 chamadas | 42.0 tokens por chamada | 0.0 ms por chamada
  modelo instavel: 4 pedidos | 4 abstencoes (100.0%) | o esquema da ficha NAO serve a este modelo: 100.0% de abstencoes, acima do limite declarado de 30%
  modelo de ensaio: 4 pedidos | 0 abstencoes (0.0%) | o esquema da ficha serve a este modelo: 0.0% de abstencoes, dentro do limite declarado de 30%

-- o estado declarado --
  pesos congelados: ausentes da arvore | runner local: nao existe aqui | por isso a ponte fica declarada, e nenhum numero de calibracao ou de custo real e publicado
  o que ja fecha: o portao dos pesos, a decodificacao restrita com abstencao, a prova de determinismo, o custo por chamada e a regra de adequacao do espaco
~~~

### A segunda opiniao dos resultados marcados P

Le o registro do ponto 19 com as catorze linhas marcadas P e da a classe de cada uma: conferida (uma
segunda implementacao refez a conta), derivada ou registro. Mede o que da: os limiares alfa/M do p03,
o vies do laco em duas fontes independentes, a base do banco recontada em Rust, os pareceres contados e a
reproducao do p05, onde a divergencia entre o campo em maquina e a frase publicada fica declarada.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --segunda
~~~

~~~text
== Ponto 19 :: a segunda opiniao dos resultados marcados P ==
carimbo do registro: 2026-10-16
item                       classe      segunda opiniao
h1.coortes                 registro    declara: o dado saiu com o andaime; nao ha segunda implementacao possivel
h1.brier                   registro    declara: o teste fora do tempo e do roteiro antigo
h2.luz                     registro    declara: o metodo e reproduzido em mundos simulados no capitulo 19
h2.fda                     registro    declara: o dado do ensaio nao esta na arvore
h2.levedura                derivada    o desenho da replica e medido de novo no ponto 14; os 40 genes reais seguem sem instrumento
h2.star                    registro    declara: e o segundo teste da unidade independente, e o dado saiu
h2.k                       conferida   os limiares alfa/M gravados sao recalculados aqui como 0,05/M, na forma exata do arquivo
h3.fluxos                  registro    declara: o mecanismo do canario e medido no capitulo 20
h3.goodhart                conferida   o vies do previsor ingenuo aparece em duas fontes independentes e as duas concordam digito por digito
h3.hipergrafo              registro    declara: o grafo saiu com o andaime; o hipergrafo do Rust e o validador
aud.piloto                 derivada    o ponto 16 re-deriva o empate no teto das notas do juiz
aud.prioris                conferida   a reproducao e lida do registro e contada de novo; o desvio maximo nao esta em maquina e fica como registro
aud.pareceres              conferida   as linhas da releitura e os arquivos reemitidos sao contados
h0.real                    conferida   a base e recontada em Rust por um caminho proprio, com a dispersao por uma passada so

-- o que a segunda opiniao mediu --
  multiplicidade: M = 32 | alfa/M esperado 0.0015625 | gravado no registro: confere
  multiplicidade: M = 38 | alfa/M esperado 0.0013157894736842105 | gravado no registro: confere
  multiplicidade: M = 72 | alfa/M esperado 0.0006944444444444445 | gravado no registro: confere
  prioris: reproducao do p05 lida do registro: 8 identicos de 33 dominios
  laco performativo: vies do ingenuo em duas fontes independentes: 0.29124674633711956 (p03) e 0.29124674633711956 (p06) | concordam digito por digito
  H0 real: recontado em Rust: 164 dias | cofre 2654950 chamadas (publicado: 2 649 354) | Var/media entre dias 261.7 em todos e 247.4 no cofre (publicado: 261.1 e 246.7)
  pareceres: 30 linhas na releitura | 4 pareceres reemitidos na pasta

linhas conferidas por segunda implementacao: 5 | linhas que so restam como registro: 7
~~~

### O contrato da ficha: a cadeia de migracoes em arquivo

Le a cadeia declarada em `crates/stochology-schema/migracoes.tsv`, embutida na compilacao, e imprime os dois
elos com a nota de cada um. Depois demonstra a migracao de um documento da versao 1.0 ate a versao da ficha:
os dois campos que a cadeia acrescenta entram com os valores declarados e o resto do documento fica intacto.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --schema
~~~

~~~text
== O contrato :: a ficha em arquivo, a versao e a cadeia de migracoes ==
versao da ficha: 1.2 | cadeia declarada em crates/stochology-schema/migracoes.tsv
  etapa 1: de 1.0 para 1.1: acrescenta resolucao = R0-agregada | a resolucao da pergunta entrou no contrato com o Phi5; a migracao nao inventa pergunta mais fina que a declarada no documento
  etapa 2: de 1.1 para 1.2: acrescenta priori_elicitada = false | o campo nasceu com a frente SHELF; um documento anterior nao tem priori elicitada de painel
  versoes da cadeia: 1.0 -> 1.1 -> 1.2

-- demonstracao: um documento da versao 1.0 migra ate a versao da ficha --
  o documento da 1.0 nao declara os dois campos que a cadeia acrescenta
  migrou: resolucao = R0-agregada (o valor de acrescimo declarado) | priori_elicitada = false | definicao intacta = true
  a ficha migrada validou: id lab.schema.1 | camada L2 | acao "dimensionar a escala do dia"
  a ficha na versao atual nao passa pela cadeia: true
~~~

### A tese da ponte em escala, com o teste justo por fazer

Re-deriva o piloto P01 das notas do juiz cego (8 de 8 defeitos nos dois bracos, 0 de 4 alarmes falsos, 0 pares
discordantes, empate no teto) e da o veredito do portao: enquanto faltar escolhedor de casos, analistas
humanos, juiz humano e os 12 casos, a tese fica nao sustentada e nenhuma linha do registro pode dizer o contrario.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --ponte
~~~

~~~text
== Ponto 16 :: a tese da ponte em escala ==
-- o piloto P01, re-derivado das notas do juiz cego --
casos 12 | com defeito 8 | limpos 4 | pares discordantes 0
apontou o defeito: braco X 8 de 8 | braco Y 8 de 8
alarme falso nos limpos: braco X 0 de 4 | braco Y 0 de 4
veredito: empate no teto: os dois bracos apontaram 8 de 8 defeitos e deram 0 alarmes falsos em 4 casos limpos; pela regra declarada, a tese NAO e sustentada

-- o teste justo --
prazo da janela: 2027-01-14 | casos minimos: 12
se a janela vencer sem os papeis: nao sustentada em definitivo, aceito pelo dono em 2026-10-16
  escolhedor_dos_casos   nao designado de fora do programa, escolhe os casos antes de existir resultado
  analistas_braco_A      nao designado dois humanos, um de fora do programa
  analistas_braco_B      nao designado dois humanos, um de fora, cegos de qualquer texto da Estocologia
  juiz                   nao designado um humano de fora do programa, cego ao braco, com o gabarito
  casos                  0 de 12    casos mais dificeis que a lista de 2019, escolhidos pelo de fora
  status_da_tese         nao sustentada nao sustentada ate o teste justo (o empate no teto conta como nao sustentada)
designados: [] | pendentes: ["escolhedor_dos_casos", "analistas_braco_A", "analistas_braco_B", "juiz", "casos (0 de 12)"] | casos faltando: 12
divergencias: nenhuma — nenhuma linha afirma o que o teste justo ainda nao sustenta
veredito: TESE NAO SUSTENTADA e caixa fechada — com 5 pendente(s), a tese nao vira "sustentada" por nenhum caminho que nao o teste justo, e que seja ate 2027-01-14

-- prova negativa: um registro que diz a tese sustentada sem o teste justo --
  divergencias: ["status declarado \"sustentada\" com o teste justo por fazer: a tese so muda de estado depois do resultado do juiz"]
  caixa fechada: true
-- prova positiva: papeis designados e casos no minimo --
  divergencias: [] | caixa fechada: false
~~~

### O lacre das sete decisoes que mexem com pessoas

Le o registro em maquina do ponto 13 (`fundo/p13-escuta/registro.tsv`): as sete decisoes, quem paga o erro
de cada uma, quem foi ouvido e a cobertura minima declarada. Da o veredito do lacre e mostra, com dois casos
sinteticos, que o portao sabe reprovar uma decisao que se diz emitida sem ninguem ouvido e que sabe abrir a
caixa quando a cobertura chega ao minimo.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --escuta
~~~

~~~text
== Ponto 13 :: a escuta de quem paga o erro ==
decisoes: 7 | carimbo do registro: 2026-10-16 | conducao: protocolo cego | prazo da janela: 2026-12-15 | cobertura minima: 2 de 7
decisao                              titulares ouvidos      registro   pode emitir
h2g/saida/direito_teto_beneficio             - nenhum       -          nao
h2g/saida/educacao_star_turmas           11598 nenhum       -          nao
h2g/saida/saude_rand                         - nenhum       -          nao
h1-real/saida/b                          16177 nenhum       -          nao
h3/prisoes                                5226 nenhum       -          nao
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

cobertura de escuta: 0 de 7 grupos ouvidos
divergencias: nenhuma — nenhuma decisao se diz emitida sem ninguem ouvido
veredito: CAIXA LACRADA — a cobertura de 0 fica abaixo do minimo de 2; as decisoes ficam como analise de sensibilidade (n = 1) e nao viram diretriz de politica publica nem decisao de portao F8 ate 2026-12-15 ou ate a cobertura minima

sem escuta, uma por uma:
  h2g/saida/direito_teto_beneficio   | trabalhadores acidentados cobertos pelo teto (Kentucky 1980, Michigan 1982)
  h2g/saida/educacao_star_turmas     | criancas do jardim a 3a serie do STAR no Tennessee
  h2g/saida/saude_rand               | familias nao idosas do RAND HIE (anos 1970)
  h1-real/saida/b                    | participantes do NSW e controles do CPS
  h3/prisoes                         | detidos de Toronto 1997-2002
  h3/prisoes20                       | prisoes de Toronto 2020-2021
  h3/prisoes_ri                      | paradas de Rhode Island 2005-2015

-- prova negativa: a decisao que se diz emitida sem ninguem ouvido --
decisoes: 3 | carimbo do registro: 2026-10-16 | conducao: protocolo cego | prazo da janela: 2026-12-15 | cobertura minima: 2 de 3
decisao                              titulares ouvidos      registro   pode emitir
h3/prisoes                                5226 nenhum       -          sim
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

cobertura de escuta: 0 de 3 grupos ouvidos
divergencias: REPROVADO em ["h3/prisoes"]
veredito: CAIXA LACRADA — a cobertura de 0 fica abaixo do minimo de 2; as decisoes ficam como analise de sensibilidade (n = 1) e nao viram diretriz de politica publica nem decisao de portao F8 ate 2026-12-15 ou ate a cobertura minima

-- prova positiva: dois grupos ouvidos, com registro assinado --
decisoes: 2 | carimbo do registro: 2026-10-16 | conducao: protocolo cego | prazo da janela: 2026-12-15 | cobertura minima: 2 de 2
decisao                              titulares ouvidos      registro   pode emitir
h3/prisoes                                5226 associacao de defesa publica de TORONTO registro-01.md sim
h3/prisoes20                             65276 coletivo de ex-detentos registro-02.md sim

cobertura de escuta: 2 de 2 grupos ouvidos
divergencias: nenhuma — nenhuma decisao se diz emitida sem ninguem ouvido
veredito: CAIXA ABERTA para as decisoes cobertas — a cobertura de 2 alcancou o minimo de 2
~~~

### O horizonte R: a resolucao da pergunta

Bracos pareados, mesmo recurso total. A perda de resolucao mede quanto custa perguntar pelo dia medio em
vez de pelo dia que existe.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --resolucao
~~~

~~~text
== Horizonte R :: resolucao da pergunta ==
bracos pareados: mesmas chegadas e mesmas bases de servico; muda so a taxa de atendimento
demanda 5 ou 11 (o posto do Capitulo 16): capacidade media 8.0 | espera agregada 86.48 min, espera fina 40.24 min, perda 46.24 min
   acima de 30 min: agregada 0.6042, fina 0.4840
demanda 7 ou 9 (oscilacao leve): capacidade media 8.0 | espera agregada 47.70 min, espera fina 41.73 min, perda 5.97 min
   acima de 30 min: agregada 0.5168, fina 0.5063
demanda 8 ou 8 (sem oscilacao): capacidade media 8.0 | espera agregada 41.56 min, espera fina 41.56 min, perda 0.00 min
   acima de 30 min: agregada 0.5013, fina 0.5013
escada de resolucao: R0-agregada -> R1-dia -> R2-hora -> R3-subpergunta

-- previsao 3: o limiar de oscilacao (dia falho: mais de 10% acima de 30 min; limiar: mais de 1/4 dos dias) --
  200 dias por amplitude | 10 h por dia | especificacao 1: o plano de Erlang do dia medio, 12 agentes
   amplitude  dias falhos         fracao  perda (min) acima30 R0
         0.0           41          0.205        -0.26     0.0612
         0.5           52          0.260        -1.89     0.0776
         1.0           51          0.255         1.97     0.0954
         1.5           64          0.320         2.60     0.1267
         2.0           66          0.330         2.27     0.1365
         2.5           73          0.365         7.04     0.1634
         3.0           80          0.400         8.93     0.2314
  limiar: amplitude 0.5 | perda de resolucao no limiar -1.89 min | monotona: false

-- previsao 3, especificacao 2: a escala de R0 calibrada na demanda plana --
  a escala calibrada tem 13 agentes (a plana falha no maximo 10% dos dias)
   amplitude  dias falhos         fracao  perda (min) acima30 R0
         0.0           19          0.095        -2.24     0.0339
         0.5           30          0.150        -4.08     0.0467
         1.0           33          0.165        -0.66     0.0554
         1.5           39          0.195        -0.65     0.0739
         2.0           42          0.210        -1.12     0.0854
         2.5           53          0.265         2.87     0.1027
         3.0           66          0.330         3.36     0.1458
  limiar: amplitude 2.5 | perda de resolucao no limiar 2.87 min | monotona: true

-- previsao 5: a transferencia (sobrevive a R1 implica sobrevive a R0; o contrario nao) --
  6 perfis x 200 dias | limiar do dia 0.10 | violacoes da implicacao: 0 | contraexemplos: 3 | conservacao do agregado conferida: true
  leitura: nenhum perfil sobrevive a R1 e falha em R0; 3 perfis passam em R0 e falham em algum dia

-- o eixo R, capitulo por capitulo (a tabela esta em book/eixo-r.tsv; a leitura e calculada aqui) --
capítulo  1 | sem perda medida neste capitulo | o capítulo apresenta o alvo e a tese; não há decisão de escala
capítulo  2 | sem perda medida neste capitulo | as três camadas; o grão da pergunta não entra na conta aqui
capítulo  3 | sem perda medida neste capitulo | probabilidade sem decisão de escala
capítulo  4 | sem perda medida neste capitulo | os três tipos de incerteza; o eixo R é ortogonal a eles
capítulo  5 | perda de resolucao 46.24 min (demanda de 5 a 11) | a média mente: o capítulo é o posto que oscila entre 5 e 11
capítulo  6 | sem perda medida neste capitulo | a unidade independente é o Φ2; o eixo R é o primo dele no grão da pergunta
capítulo  7 | sem perda medida neste capitulo | testes severos e multiplicidade; sem decisão de escala
capítulo  8 | perda de resolucao 5.97 min (demanda de 7 a 9) | a tolerância de calibração depende do n do cofre: a oscilação de 7 a 9 é o caso leve
capítulo  9 | sem perda medida neste capitulo | a escada de causa; o grão entra como a unidade dos ambientes
capítulo 10 | perda de resolucao 0.00 min (demanda plana) | o arrasto e a ruína se medem por dia; sem oscilação, refinar não rende nada
capítulo 11 | sem perda medida neste capitulo | o trilema é a subpergunta do R3: cada hora, e para quem
capítulo 12 | sem perda medida neste capitulo | o mapa do conhecimento; sem decisão de escala
capítulo 13 | sem perda medida neste capitulo | a banca adversarial julga a ficha, e não o grão dela
capítulo 14 | sem perda medida neste capitulo | o contrato carrega o campo resolucao (R0 a R3) e o Φ5; a trava só morde com unidade igual ou mais grossa que o dia
capítulo 15 | sem perda medida neste capitulo | cada cofre é um grão novo: a régua do eixo R quando a correção muda de pergunta
capítulo 16 | perda de resolucao 46.24 min (demanda de 5 a 11) | o capítulo traz a tabela R0 a R3 do posto e do banco
capítulo 17 | sem perda medida neste capitulo | a unidade independente é a pessoa; o eixo R aparece como o grão da comparação
capítulo 18 | perda de resolucao 0.00 min (demanda plana) | os incêndios: o número muda de janela, e a janela é o grão
capítulo 19 | sem perda medida neste capitulo | agrupar por escola derruba a área; por aluno, não: a unidade independente no vocabulário do eixo
capítulo 20 | perda de resolucao 46.24 min (demanda de 5 a 11) | o cofre ao vivo: laboratório 0,0022 e cofre real 0,0048 são dois grãos de teste
capítulo 21 | limiar de oscilacao na amplitude 2.5 (plano calibrado) | o capítulo nomeia o eixo e as previsões 3 e 5
~~~

## 2. Os 21 laboratorios do livro

### Todos os laboratorios

Uma execucao, semente fixa, sem rede.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/labs --todos
~~~

~~~text
== Capítulo 1 :: o alvo invisível ==
duas maquinas com a semente 1713 concordam em 10000 de 10000 sorteios: true
sorteios coincidentes com a semente seguinte: 0
semente 1713: frequencia de caras em 100 mil jogadas = 0.5000
semente 1909: frequencia de caras em 100 mil jogadas = 0.4992
semente 2026: frequencia de caras em 100 mil jogadas = 0.4987
== Capítulo 2 :: a falácia mora na ponte ==
prevalencia 1e-2: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.0001  fator da troca=99.0
prevalencia 1e-3: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.0010  fator da troca=998.0
prevalencia 1e-6: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.5000  fator da troca=499999.7
== Capítulo 3 :: árvores, chances e decibans ==
mamografia: 100 doentes, 90 positivos doentes, 9900 saudaveis, 891 falsos positivos, 981 positivas
P(doente|positivo) = 0.0917  (uma em cada 10.9 positivas)
segundo exame: independente = 10.0 dB, correlacionado (razao efetiva 4,2) = 6.23 dB
posterior com dois positivos: independentes = 0.5025, correlacionados = 0.2979
== Capítulo 4 :: a moeda cujo vício muda ==
vies sobe de 0.2 a 0.6 em 6400 passos, 200 repeticoes, desconto 0.99
cobertura do intervalo de 95%: modelo fixo = 0.1199, modelo com esquecimento = 0.9398
largura do intervalo fixo no passo 100 = 0.1564, no passo 6400 = 0.0240
largura do intervalo com esquecimento no passo 6400 = 0.1360 (razao 5.7x o fixo final)
== Capítulo 5 :: quando a média mente ==
W(5) = 3.57 min, W(8) = 10.00 min, W(11) = 55.00 min
Jensen: f(E[X]) = 10.00 min | E[f(X)] = 29.29 (5 ou 11) e 14.10 (uniforme 5..11)
segunda ordem: meia f''(8) Var(X) = 8.44 min (5 ou 11) e 2.81 min (uniforme)
                  8 todo dia:  319405 pacientes | media   9.1 min | mediana   3.7 | p90  26.2 | >30 min:   7.6 em 100
                     5 ou 11:  319115 pacientes | media  19.6 min | mediana   8.6 | p90  56.1 | >30 min:  23.7 em 100
              uniforme 5..11:  319843 pacientes | media  13.8 min | mediana   5.5 | p90  39.8 | >30 min:  15.0 em 100
   5 ou 11, reforco no surto:  319115 pacientes | media   2.5 min | mediana   0.0 | p90   8.1 | >30 min:   0.4 em 100
nos dias de surto: 27.0 min em 219458 pacientes (68.8% do total); nos dias leves: 3.4 min
nulo de metades no mundo  8 todo dia: desvio medido 0.201 min | margem por dia 0.185 | por paciente 0.046 | variancia da espera 168 min^2
nulo de metades no mundo     5 ou 11: desvio medido 0.742 min | margem por dia 0.563 | por paciente 0.097 | variancia da espera 747 min^2
diferenca entre os mundos = 10.6 min = 52.7 desvios do nulo
mediana do mundo 5 ou 11 = 8.6 min, contra a espera media de 9.1 min do mundo constante
turma de 100000 alunos: media = 6.21, reprovados (nota < 6,0) = 45.1 em 100
== Capítulo 6 :: o filtro invisível ==
Literary Digest: 2400031 de 10000000 eleitores responderam (f = 0.240); Roosevelt entre eles = 0.4259, Landon = 0.5741
erro medido = -0.1821 (-18.2 pontos); identidade de Meng com rho = -0.21: -0.1824
o erro vale o erro-padrao de uma amostra aleatoria de 7.2 pessoas; a margem de erro da pesquisa era 0.0276 pontos, e o erro foi de 18.2 pontos = 661 margens
desvio de 2000 amostras aleatorias de 7 pessoas = 0.1836 (18.4 pontos); de 50 000 entrevistas = 0.0022 (0.22 pontos)
Berkson com corte 1.0: correlacao na populacao = +0.0012, entre selecionados (24.0% da populacao) = -0.6168
Berkson com corte 1.5: correlacao na populacao = +0.0012, entre selecionados (14.4% da populacao) = -0.6790
Berkson com corte 2.0: correlacao na populacao = +0.0012, entre selecionados (7.9% da populacao) = -0.7315
Simpson: pequenos 0.931 x 0.867; grandes 0.730 x 0.688; bruto 0.780 x 0.826 (inverte)
ponderado por uma populacao meio a meio: 0.831 x 0.777 (a aberta volta a vencer)
cadeia de dois estados com persistencia 0.500: variancia da media de 100 letras = 0.002487 (independente: 0.002500), tamanho efetivo = 101
cadeia de dois estados com persistencia 0.800: variancia da media de 100 letras = 0.009899 (independente: 0.002500), tamanho efetivo = 25
cadeia de dois estados com persistencia 0.235: variancia da media de 100 letras = 0.000765 (independente: 0.002500), tamanho efetivo = 327
== Capítulo 7 :: caçando fantasmas no ruído ==
jardim de modelos: 200 replicas, 500 candidatos em 50 preditores de puro ruido, 25 casos de treino
o vencedor da validacao bate prever zero em 79.5% das replicas; diante de dados novos, em 30.0%
erro quadratico medio do vencedor: 0.814 na validacao (prever zero: 1,000) e 1.191 em dados novos
mao quente: media das proporcoes por sequencia = 0.4167 (5/12 = 0.4167); proporcao agrupada = 0.5000
100 pessoas jogando 20 moedas: em 100.0% dos experimentos alguem emenda 6 caras seguidas; a maior sequencia tem media 9.5
jardim dos caminhos (4000 experimentos sem efeito nenhum): com as quatro liberdades de Simmons, alguem acha p < 0,05 em 21.5% deles; com a familia grande, em 47.0%; o teste pre-registrado acusa 5.2%
severidade de Mayo: 1 - p = 0.96 no teste pre-registrado com p = 0,04; 1 - taxa de falso positivo = 0.53 no jardim
Ioannidis com R = 1/1000, poder 0,80: VPP = 0.016 com alfa 0,05 e 0.002 com o alfa do jardim
Ioannidis com R = 1/10, poder 0,80: VPP = 0.615 com alfa 0,05 e 0.146 com o alfa do jardim
Ioannidis com R = 1/3, poder 0,80: VPP = 0.842 com alfa 0,05 e 0.362 com o alfa do jardim
banca A1-A8 com a semente 2026, ficha com os oito cuidados: severidades [9, 8, 10, 9, 8, 9, 8, 10], piso 6, aprovada = true
a mesma ficha sem o cuidado de uma faixa: a menor severidade cai para 2 (reprovada)
== Capítulo 8 :: o termômetro do adivinho ==
4000 dias, taxa-base observada = 0.243, chance media declarada pelo especialista = 0.252
preguicoso (0,20 sempre): Brier 0.1858 | REL 0.0021 RES 0.0002 UNC 0.1840 | ECE 0.0430 | apos Platt: Brier 0.1840, ECE 0.0130 (a = -1.14, b = 0.00)
            especialista: Brier 0.1569 | REL 0.0012 RES 0.0267 UNC 0.1840 | ECE 0.0281 | apos Platt: Brier 0.1560, ECE 0.0115 (a = -0.22, b = 0.81)
        confiante demais: Brier 0.1712 | REL 0.0150 RES 0.0275 UNC 0.1840 | ECE 0.1109 | apos Platt: Brier 0.1559, ECE 0.0120 (a = -0.25, b = 0.37)
                 ruidoso: Brier 0.2119 | REL 0.0363 RES 0.0083 UNC 0.1840 | ECE 0.1449 | apos Platt: Brier 0.1755, ECE 0.0166 (a = -0.82, b = 0.27)
identidade de Murphy no especialista: REL - RES + UNC = 0.1584339, Brier das previsoes agrupadas nas caixas = 0.1584339, diferenca = 3.47e-15
a previsao crua tem Brier 0.1569462; agrupar em 10 caixas custa 1.49e-3, e a identidade fecha
 prisoes Toronto 1997-2002: n =    1488 | q50 nulo 0.0292 | q95 nulo 0.0425 | q99 nulo 0.0481 | media 0.0296 | ECE nulo acima de 0,03 em 45.1% das simulacoes
    Rhode Island 2013-2015: n =   24466 | q50 nulo 0.0028 | q95 nulo 0.0043 | q99 nulo 0.0051 | media 0.0029 | ECE nulo acima de 0,03 em 0.0% das simulacoes
              voos NY 2023: n =  104489 | q50 nulo 0.0025 | q95 nulo 0.0037 | q99 nulo 0.0042 | media 0.0026 | ECE nulo acima de 0,03 em 0.0% das simulacoes
n = 1 488, espalhamento 0.4: q50 nulo 0.0320, q95 nulo 0.0456
n = 1 488, espalhamento 1.0: q50 nulo 0.0291, q95 nulo 0.0419
n = 1 488, espalhamento 2.0: q50 nulo 0.0243, q95 nulo 0.0349
== Capítulo 9 :: o paradoxo de Simpson nas cirurgias de pedra nos rins ==
pequenas: aberta 81/87 = 0.9310  nova 234/270 = 0.8667
grandes:  aberta 192/263 = 0.7300  nova 55/80 = 0.6875
cru:      aberta 273/350 = 0.7800  nova 289/350 = 0.8257
peso dos estratos na populacao inteira: pequenas 0.5100, grandes 0.4900
ponderado pela populacao inteira: aberta 0.8325  nova 0.7789  -> a vantagem inverte de novo
fracao de casos dificeis na aberta = 0.7514, na nova = 0.2286

== Capítulo 9 :: os tres degraus de Pearl ==
  [   1. associação] bairros com mais policia tem mais crimes
  [  2. intervenção] se dobrarmos o policiamento, o crime cai?
  [3. contrafactual] esse assalto teria acontecido com uma viatura na esquina?

== Capítulo 9 :: um ICP minimo, tres sitios, tres candidatos ==
linhas = 12000 (4000 por sitio), X -> Y com coeficiente 2, X -> Z (filho) com ruido diferente por sitio, W puro ruido
correlacao ingenua com Y: corr(X,Y) = 0.9314  corr(Z,Y) = 0.7208  corr(W,Y) = 0.0055
inclinacao ingenua (todos os sitios juntos): X = 1.9885  Z = 0.6003  W = 0.0152
  candidato {    x}  F =   0.755  p = 4.702e-1  comum = 1.985  por sitio = [1.970, 1.996, 1.990]  ACEITO
  candidato {    z}  F = 976.809  p = 0.000e0  comum = 0.477  por sitio = [0.928, 0.496, 0.299]  rejeitado
  candidato {    w}  F =   1.474  p = 2.290e-1  comum = 0.011  por sitio = [0.056, -0.030, 0.008]  ACEITO
  candidato {  x+z}  F =   0.612  p = 6.541e-1  comum = 1.991  por sitio = [1.918, 1.998, 1.999]  ACEITO
  candidato {  x+w}  F =   0.704  p = 5.890e-1  comum = 1.985  por sitio = [1.970, 1.996, 1.990]  ACEITO
  candidato {  z+w}  F = 488.188  p = 0.000e0  comum = 0.477  por sitio = [0.928, 0.496, 0.299]  rejeitado
  candidato {x+z+w}  F =   0.624  p = 7.111e-1  comum = 1.991  por sitio = [1.919, 1.998, 1.999]  ACEITO
  candidato {vazio}  F = 0.000  p = 1.0  (so interceptos)
conjuntos aceitos: 5 -> intersecao S_hat = {}

== Capítulo 9 :: 200 repetições, mecanismo estavel: Y = 2X + ruido em todo sitio
inclinação ingênua média (todos os sítios juntos): X = 2.0004  Z = 0.6015
  {    x}  F médio =     1.04  taxa de rejeição = 0.060
  {    z}  F médio =   973.87  taxa de rejeição = 1.000
  {    w}  F médio =     1.14  taxa de rejeição = 0.085
  {  x+z}  F médio =     0.99  taxa de rejeição = 0.050
  {  x+w}  F médio =     1.04  taxa de rejeição = 0.075
  {  z+w}  F médio =   487.45  taxa de rejeição = 1.000
  {x+z+w}  F médio =     1.01  taxa de rejeição = 0.055
  {x} aceito em 0.940 das rodadas
  interseção = {x} em 0.080 das rodadas
  interseção = {z} em 0.000 das rodadas
  interseção = {w} em 0.025 das rodadas
  interseção contém x e não contém w em 0.080 das rodadas
  interseção vazia em 0.895 (dos quais 0.025 porque só o ruído W sobrou, 0.005 porque a própria causa não sobrou, e 0.890 porque W entrou em alguns conjuntos e não em outros)

== Capítulo 9 :: 200 repetições, mecanismo instavel: o coeficiente de X em Y muda de sitio para sitio
inclinação ingênua média (todos os sítios juntos): X = 4.0004  Z = 1.2029
  {    x}  F médio =     1.04  taxa de rejeição = 0.060
  {    z}  F médio =  1352.54  taxa de rejeição = 1.000
  {    w}  F médio =     1.10  taxa de rejeição = 0.070
  {  x+z}  F médio =     0.99  taxa de rejeição = 0.050
  {  x+w}  F médio =     1.04  taxa de rejeição = 0.075
  {  z+w}  F médio =   676.76  taxa de rejeição = 1.000
  {x+z+w}  F médio =     1.01  taxa de rejeição = 0.055
  {x} aceito em 0.940 das rodadas
  interseção = {x} em 0.065 das rodadas
  interseção = {z} em 0.000 das rodadas
  interseção = {w} em 0.025 das rodadas
  interseção contém x e não contém w em 0.065 das rodadas
  interseção vazia em 0.910 (dos quais 0.025 porque só o ruído W sobrou, 0.005 porque a própria causa não sobrou, e 0.905 porque W entrou em alguns conjuntos e não em outros)

== Capítulo 10 :: a moeda de Peters (cara +50%, coroa -40%) ==
média de conjunto por rodada = 1.0500  (+5.00% por rodada)
média no tempo por rodada  = -0.052680  (-5.268% por rodada)
produto de uma cara e uma coroa: 1,5 x 0,6 = 0.90
Kelly cheio (5:4): f* = 0.1400 da riqueza; com pagamento de igual para igual, 2p - 1 = 0.20
crescimento no tempo com Kelly cheio: +0.0175 por rodada

aposta total (f = 1,00)    acima do início: 10 rodadas 0.374  100 rodadas 0.128  1000 rodadas 0.00000
                           ruína (< 1e-6 do capital) = 0.9971  mediana final = 1.322e-23  média final = 2.725e-6
Kelly cheio (f = 0,70)     acima do início: 10 rodadas 0.374  100 rodadas 0.300  1000 rodadas 0.07830
                           ruína (< 1e-6 do capital) = 0.5089  mediana final = 6.810e-7  média final = 4.982e4
meio Kelly (f = 0,35)      acima do início: 10 rodadas 0.618  100 rodadas 0.611  1000 rodadas 0.85620
                           ruína (< 1e-6 do capital) = 0.0000  mediana final = 1.854e2  média final = 2.119e6
quarto de Kelly (f = 0,14) acima do início: 10 rodadas 0.618  100 rodadas 0.811  1000 rodadas 0.99560
                           ruína (< 1e-6 do capital) = 0.0000  mediana final = 1.506e2  média final = 1.016e3

aposta total: log10 da mediana final = -22.9, log10 da média final = -5.6
parte da média que vem da trajetória mais rica (1% mais rico): 1.000

== Capítulo 10 :: o vendedor de seguro (ganha 1% por semana, perde 50% com q = 0,004) ==
crescimento no tempo da aposta total: +0.00714 por semana (negativo: a ruína é certa)
frequência observada + Kelly pleno sem perda = 0.368  aposta total = 0.368  ruina em 1000 semanas = 0.0000  mediana final = 3.531e2  média final = 1.255e3
Laplace + Kelly pleno            sem perda = 0.368  aposta total = 0.000  ruina em 1000 semanas = 0.0000  mediana final = 8.983e1  média final = 2.529e2
Laplace + meio Kelly             sem perda = 0.368  aposta total = 0.000  ruina em 1000 semanas = 0.0000  mediana final = 1.097e1  média final = 1.371e1
probabilidade teórica de 250 semanas sem perda = 0.3671
Laplace com zero perdas em 250 semanas: q_est = 1/252 = 0.003968

== Capítulo 10 :: a cauda selvagem: Pareto com alfa = 1,1 ==
média verdadeira (alfa/(alfa-1)) = 11.0
n =      1000: média amostral média =     7.76  mediana =    6.22  abaixo de 11 em 0.94  IC95 cobre em 0.24
n =   1000000: média amostral média =     9.53  mediana =    8.39  abaixo de 11 em 0.95  IC95 cobre em 0.26
fração da média que vem de valores acima do maior visto: 0.000
em mediana, 25.12% da média verdadeira vêm de valores acima do máximo de uma amostra de um milhão

estimador de Hill com alfa = 1,1 e n = 200 000:
  k =    100  alfa_hat = 1.083  IC95 aprox [0.870; 1.295]
  k =    500  alfa_hat = 1.063  IC95 aprox [0.970; 1.157]
  k =   1000  alfa_hat = 1.100  IC95 aprox [1.032; 1.168]
  k =   5000  alfa_hat = 1.101  IC95 aprox [1.070; 1.131]
  k =  20000  alfa_hat = 1.095  IC95 aprox [1.080; 1.111]
gregarina: um modelo gaussiano ajustado a essa cauda subestima a perda em todas as repetições

== Capítulo 10 :: o arrasto da volatilidade em 1x, 2x e 3x ==
L = 1x: media aritmetica = +0.000317/sessao, crescimento medido = +0.000249/sessao, arrasto = +0.000068/sessao (+0.0172/ano), riqueza em 100000 sessoes = 6.363e10
L = 2x: media aritmetica = +0.000634/sessao, crescimento medido = +0.000267/sessao, arrasto = +0.000367/sessao (+0.0926/ano), riqueza em 100000 sessoes = 3.805e11
L = 3x: media aritmetica = +0.000951/sessao, crescimento medido = +0.000053/sessao, arrasto = +0.000898/sessao (+0.2264/ano), riqueza em 100000 sessoes = 1.955e2
horizonte de 2000 sessões:
  L = 1x: crescimento medido = +0.000196/sessao, riqueza final = 1.480e0
  L = 2x: crescimento medido = +0.000162/sessao, riqueza final = 1.384e0
  L = 3x: crescimento medido = -0.000101/sessao, riqueza final = 8.168e-1
horizonte de 10000 sessões:
  L = 1x: crescimento medido = +0.000352/sessao, riqueza final = 3.385e1
  L = 2x: crescimento medido = +0.000472/sessao, riqueza final = 1.126e2
  L = 3x: crescimento medido = +0.000360/sessao, riqueza final = 3.649e1
o crash de 1987 (-22,6% num dia) tira 67.8% do capital em 3x, 22.6% em 1x; a ruína comeca em L = 4.4
com um crash de -22,6% num dia e 40 dias de -3% (20 mil sessões):
  L = 1x: crescimento medido = +0.000147/sessão, riqueza final = 1.890e1
  L = 2x: crescimento medido = +0.000058/sessão, riqueza final = 3.167e0
  L = 3x: crescimento medido = -0.000274/sessão, riqueza final = 4.188e-3

== Capítulo 11 :: o navio de Bernoulli (C = 1.800, L = 34.000, p = 0,05) ==
limiar p > C/L: p* = 0.05294 (5.294%)  ->  p = 0,050 nao passa o limiar
em esperança: perda esperada sem seguro = 1700, resultado do seguro = +100 por mês
crescimento no tempo do armador: 1 navio sem seguro = -0.0208, 1 navio com seguro = -0.0182, 3 navios sem seguro = -inf
seguradora com     1 contratos: crescimento no tempo = -0.000186 por mês
seguradora com    10 contratos: crescimento no tempo = -0.001808 por mês
seguradora com   100 contratos: crescimento no tempo = -0.013466 por mês
seguradora com  1000 contratos: crescimento no tempo = +0.100410 por mês

== Capítulo 11 :: o EVPI (tratar custa 1, deixar um doente custa 10) ==
limiar de ação: p > 0.10  ->  com p = 0,3 o melhor ato sem informação tratar (-1.00)
ganho de quem sabe o estado antes de agir = -0.30
EVPI = +0.70
  exame sensivel  (0,90; 0,70): EVSI = +0.2400
  exame especifico (0,30; 0,95): EVSI = +0.2450
  com prevalência 0.05: melhor ato sem informação = nao tratar, EVPI = +0.450
  com prevalência 0.30: melhor ato sem informação = tratar, EVPI = +0.700
  com prevalência 0.60: melhor ato sem informação = tratar, EVPI = +0.400

== Capítulo 11 :: os diques de van Dantzig, quatro modelos de cauda ==
  modelo    xi = -0.1  xi = +0.0  xi = +0.1  xi = +0.2   altura ótima
  T = 100          4.07       1.26       5.71       7.01
  T = 10000        5.84       1.26      12.78      21.49
  altura ótima com xi = -0.1: 5.839 m  (custo 4.143)
  altura ótima com xi = +0.0: 3.059 m  (custo 1.130)
  altura ótima com xi = +0.1: 9.809 m  (custo 9.426)
  altura ótima com xi = +0.2: 12.584 m  (custo 13.613)
  valor esperado com o Gumbel: 3.06 m
  media dos modelos: 7.82 m
  maximin de Wald: 12.58 m
  arrependimento minimax de Savage: 9.81 m (maior arrependimento 6.68)
  precaucao: 1 em 10 000 no pior modelo = 21.49 m
  EVPI do conjunto de modelos = +18.427 m

== Capítulo 11 :: o trilema de Chouldechova ==
grupo     taxa-base        FNR        FPR falsos positivos falsos negativos        VPP
White         0.069      0.240      0.105              467               79     0.9408
Black         0.111      0.360      0.196              832              191     0.9096
casos observados: White 329 marcados pelos fatos, Black 530

-- FNR igualado para baixo, ao FNR do White em 0.240 --
White         0.069      0.240      0.105              467               79
Black         0.111      0.240      0.045              192              127
falsos positivos totais: 1299 -> 659 (-640)
falsos negativos totais: 270 -> 206 (-64)

-- FNR igualado para cima, ao FNR do Black em 0.360 --
White         0.069      0.360      0.246             1095              119
Black         0.111      0.360      0.196              832              191
falsos positivos totais: 1299 -> 1927 (+628)
falsos negativos totais: 270 -> 309 (+40)

identidade de Chouldechova: FPR = p/(1-p) * (1-VPP)/VPP * (1-FNR)
grupo       VPP da matriz  VPP da identidade          razao
White              0.3491             0.0592            0.2x
Black              0.2896             0.0904            0.3x
leitura: com o FNR e o FPR observados, o escore so seria calibrado por grupo
se o valor preditivo positivo fosse o da coluna do meio. Ele e muito maior: o escore
separa razoavelmente e nao se le como probabilidade de risco.

== Capitulo 12 :: o mapa do conhecimento ==
grafo: nos=3 (L1=1 L2=1 L3=1) E_tree=2 E_graph=4 hiperarestas=3 validacao=[]
ordem topologica = ["L3_gestor", "L2_erlang_c", "L1_demanda"] | folhas=1 raizes=2 | nos sem lente = []
prova 1 (aresta para no ausente): ["aresta para no ausente: L2_erlang_c -> L2_no_fantasma"]
prova 2 (ciclo dedutivo): ["ciclo em E_tree: [\"L1_demanda\", \"L2_erlang_c\", \"L3_gestor\"]"]
prova 3 (hiperaresta com um membro): ["hiperaresta com menos de dois membros: H4"]
prova 4 (no fora de toda lente): ["L2_orfao"] validacao=[]
lentes acionadas = ["Crisonomia", "Crononomia", "Escalonomia"] | nao acionadas = ["Fluxonomia", "Morfonomia"]
Escalonomia: hill_alfa(k=250) = 1.8444 cauda_selvagem=true valores_acima_de_1000=0
Escalonomia: hill_alfa(k=500) = 1.7901 cauda_selvagem=true valores_acima_de_1000=0
Escalonomia: hill_alfa(k=1000) = 1.7949 cauda_selvagem=true valores_acima_de_1000=0
Escalonomia: hill_alfa(k=2000) = 1.8032 cauda_selvagem=true valores_acima_de_1000=0
Crisonomia: EVPI = 0.0000 | melhor ato sem informacao = 5.8000 | valor com informacao perfeita = 7.0000
Crisonomia: sem a opcao de agir (so apostar metade = 3.0000 ou nao apostar), o melhor sem informacao e 2.1000 e o EVPI sobe para 3.7000
Crisonomia: sensor inutil (pagamento igual nos dois estados): EVPI = 0.0000
Morfonomia: regra 1 -> inclinacao marginal = 24.37 (fracao tratada = 0.474) | por regime = [9.96, 10.09, 10.15] | invariante(0,5) = true
Morfonomia: regra 1 com a alocacao trocada -> inclinacao marginal = -4.60 | por regime = [10.34, 9.92, 10.15] | invariante(0,5) = true
Morfonomia: regra 2, regime do meio com efeito 40 -> inclinacao marginal = 40.03 | por regime = [9.96, 40.09, 10.15] | invariante(0,5) = false | passo dentro do regime = 30.13
== Capitulo 13 :: a banca adversarial A1-A8 ==
ficha completa (ficha h3.voos): A1-codigo=9 A2-dados=8 A3-ciclo-de-vida=10 A4-inferencia=9 A5-governanca=8 A6-justica=9 A7-dominio=8 A8-decisao=10 | menor=8 piso=6 aprovado=true
ficha com A5 e A6 removidos (ficha h3.voos-com-defeito): A1-codigo=9 A2-dados=8 A3-ciclo-de-vida=10 A4-inferencia=9 A5-governanca=2 A6-justica=3 A7-dominio=8 A8-decisao=10 | menor=2 piso=6 aprovado=false
sem A1-codigo: reprovam=["A1-codigo"] qtd_reprovando=1 menor=3
sem A2-dados: reprovam=["A2-dados"] qtd_reprovando=1 menor=2
sem A3-ciclo-de-vida: reprovam=["A3-ciclo-de-vida"] qtd_reprovando=1 menor=4
sem A4-inferencia: reprovam=["A4-inferencia"] qtd_reprovando=1 menor=3
sem A5-governanca: reprovam=["A5-governanca"] qtd_reprovando=1 menor=2
sem A6-justica: reprovam=["A6-justica"] qtd_reprovando=1 menor=3
sem A7-dominio: reprovam=["A7-dominio"] qtd_reprovando=1 menor=2
sem A8-decisao: reprovam=["A8-decisao"] qtd_reprovando=1 menor=4
varredura: fichas reprovadas=8 de 8 | menor severidade global=2 | severidades < piso por agente=[1, 1, 1, 1, 1, 1, 1, 1]
ficha completa com semente 7: menor=8 aprovado=true
ficha completa com semente 2026: menor=8 aprovado=true
ficha completa com semente 999983: menor=8 aprovado=true
sistema 1: rotulo=ficha-em-ordem produtos=[0.06, 0.94] soma=1.0000 fora_de[0.01;0.99]=0
== Capitulo 14 :: o contrato de ferro ==
ficha h0.posto-saude: camada=L1 incerteza=aleatoria cabecalho=5 campos | falhas=[] total=0
taxonomia declarada: camadas=["L1", "L2", "L3"] incertezas=["aleatoria", "epistemica", "knightiana", "nao-se-aplica"]
defeito SemSemente: falhas=1 portoes=Phi1-simulabilidade detalhe="falta semente fixa ou artefato simulavel"
defeito SemDefinicao: falhas=1 portoes=Phi3-camadas-e-modelo-nulo detalhe="sem modelo nulo ou sem definicao em uma linha"
defeito SemMargemNemUnidade: falhas=1 portoes=Phi2-unidade-independente detalhe="margem sem unidade independente declarada"
defeito SemModeloNulo: falhas=1 portoes=Phi3-camadas-e-modelo-nulo detalhe="sem modelo nulo ou sem definicao em uma linha"
defeito SemEscopoSemQuemPagaSemSaida: falhas=1 portoes=Phi4-escopo-e-saida detalhe="escopo ecologico, quem paga pelo erro ou clausula de saida ausente"
defeito PerguntaSemInterrogacao: falhas=1 portoes=tipo-III-pergunta-do-decisor detalhe="pergunta_do_decisor precisa terminar em ? e citar a acao do pacote"
defeito PerguntaSemAcao: falhas=1 portoes=tipo-III-pergunta-do-decisor detalhe="pergunta_do_decisor precisa terminar em ? e citar a acao do pacote"
grafo integro: validacao=[]
grafo com aresta fantasma: validacao=["aresta para no ausente: L2_erlang -> L2_fantasma"]
grafo com ciclo dedutivo: validacao=["ciclo em E_tree: Some(\"L1_fila\")"]
grafo com hiperaresta de um membro: validacao=["hiperaresta com menos de dois membros: H2"]
teste de propriedades (10.000 fichas simuladas): violacoes=0 margem_min=0.00617 margem_max=0.09798
mesma semente, dois validadores: execucoes divergentes em 0 de 32
== Capitulo 15 :: o cofre, a senha e a auditoria ==
sha256("")=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
sha256("abc")=ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
corrente: eventos=5 integra=true raiz_merkle=92f2ab76eb6485ab
  evento 0 ficha=h3.voos conteudo=ECE=0.0048 hash=902e82d6ca6cf280
  evento 1 ficha=h3.bolsa conteudo=mediana=2.1x hash=a9c54b2c488f0397
  evento 2 ficha=h3.energia conteudo=canario=ok hash=fbbcfbdbd5970a6c
  evento 3 ficha=h3.prisoes conteudo=ECE=0.036 hash=4248815bc8f679d9
  evento 4 ficha=h3.ri conteudo=ECE=0.0048 habilidade=0.019 hash=8059d78760a22282
adulteracao no evento 2 (canario=ok -> canario=nok, hash do evento recalculado): integra=false raiz_mudou=true
adulteracao + reescrita de 3 eventos seguintes: integra=true mas muda a raiz publicada (true)
um caractere a mais em cada evento: 5 de 5 alteracoes invalidam a corrente
lote de fichas: n=7 raiz_merkle=17679ce182647dbb
trocar uma ficha do lote muda a raiz em 7 de 7 casos
== Capítulo 16 :: a mesma média, duas esperas ==
2000 dias de 10 h por mundo, um atendente, mu = 12 por hora, semente 1909 (+ índice do mundo)
mundo (pacientes por hora)   pacientes  espera mediana     p90  >30min   margem/dia     margem/pac.
8 todo dia (a alegacao)        159716     9.1     3.7    26.1      8%         0.14            0.03
7 ou 9                         159057    10.1     4.0    29.1      9%         0.20            0.04
uniforme entre 5 e 11          160475    12.8     5.0    37.1     14%         0.32            0.05
5 ou 11                        158656    19.3     8.1    55.8     24%         0.51            0.07
5 ou 11, reforco no surto      163683     2.5     0.0     8.2      0%         0.03            0.01

nulo de metades: 4000 dias de demanda 8, 2000 reparticoes de 2000 dias contra 2000
desvio medido das diferencas = 0.2062 min
previsto pela unidade dia     = 0.1928 min
previsto pela unidade paciente= 0.0473 min  (319704 pacientes)
nulo: max |diferenca| em 2000 reparticoes = 0.664 min

teste severo: diferenca observada = 10.2 min, 49.4 desvios do nulo, fracao do nulo >= observado = 0.0000
cauda: 8 em 100 pacientes esperam mais de 30 min no mundo 8; 24 em 100 no mundo 5 ou 11
== Capítulo 17 :: arrasto da volatilidade e o viés de seleção ==
treino 1885-1999: 31919 pregoes em 115 anos; serie inteira 1885-2023: 37930 pregoes
media aritmetica diaria = 0.000240669 (0.0668 ao ano), desvio diario = 0.010496 (0.1749 ao ano)

crescimento temporal por ano e arrasto (coortes e trajetorias do treino)
  L=1: a soma aritmetica daria 0.0668 ao ano; o crescimento realizado e 0.0515; arrasto 0.0153
  L=2: a soma aritmetica daria 0.1336 ao ano; o crescimento realizado e 0.0719; arrasto 0.0617
  L=3: a soma aritmetica daria 0.2004 ao ano; o crescimento realizado e 0.0601; arrasto 0.1403
pior pregao: -0.2261 (-22.61%) em 1987-10-19; perda com L=1 0.226, L=2 0.452, L=3 0.678; ruina diaria em L >= 4.42
  ano civil 1929-1932 com L=1: o capital termina em 0.19521 do inicial (80.5% destruido)
  ano civil 1929-1932 com L=3: o capital termina em 0.00088 do inicial (99.9% destruido)
  pico a fundo, 1929-09-03 a 1932-07-08 com L=1: o capital termina em 0.10814 do inicial (89.2% destruido)
  pico a fundo, 1929-09-03 a 1932-07-08 com L=3: o capital termina em 0.00025 do inicial (100.0% destruido)

coortes de 20 anos com inicio no primeiro pregao de cada mes (treino):
L           coortes    mediana (x)      media (x)  acima do inicio
1              1134           2.13            2.9           0.9577
2              1134           2.67            7.8           0.7848
3              1134           2.10           19.1           0.6429

coortes por ano-unidade (115 anos, soma movel de 20): acima do inicio em 1x = 0.9583, em 3x = 0.6458 (96 coortes)
mundo da alegacao (anos do treino reamostrados, crescimento 3*soma aritmetica anual, sem arrasto):
  2000 replicas: fracao media das coortes acima do inicio = 0.9286, quantil 1% = 0.6354, p(fracao <= 0.6458) = 0.0140

LaLonde: NSW com 185 tratados e 260 controles sorteados; CPS com 15992 observacoes
comparacao (desfecho de 1978)               emprego    renda (US$)
experimento sorteado (NSW trat. - NSW ctrl)      0.111           1794
comparacao crua (NSW trat. - CPS)            -0.107          -8498
vies de selecao implicado (cru - experimento)     -0.218         -10292
niveis de emprego em 1978: NSW tratados 0.757, NSW controles 0.646, CPS 0.864; em 100 pessoas: 76, 65 e 86
renda media de 1978 (US$): NSW tratados 6349, NSW controles 4555, CPS 14847
placebo de 1975 (emprego): NSW tratados 0.400, NSW controles 0.315, CPS 0.891; diferenca crua tratados - CPS = -0.491
== Capítulo 18 :: a cauda, a janela limpa e a informação ==
incendios da Copenhagen Re acima de 1 M DKK, 1980-1990: 2167 sinistros em 132 meses

periodo        meses   meses atingidos   maior perda
1980-1982         36                 4         263.3
1983-1985         36                 1          57.4
1986-1988         36                 0          47.0
1989-1990         24                 2         152.4
meses atingidos e valor: 1980-07 263.3; 1981-05 56.2; 1981-12 50.1; 1982-10 65.7; 1985-08 57.4; 1989-08 152.4; 1990-10 144.7;

treino 1980-1988: 1714 sinistros em 108 meses
Hill no treino: k=25 alpha=1.9625 k=50 alpha=1.8109 k=100 alpha=1.4100
Hill k=50, IC 95% por bootstrap de blocos mensais (300 replicas): [1.37; 2.61]
Pareto simulado com alpha = 1.81, n = 1500, 200 repeticoes: Hill k=50 medio = 1.900 (dp 0.265), 90% das repeticoes entre 1.51 e 2.39

janela de 36 meses sem sinistro na camada:
Laplace na janela: p = (0+1)/(36+2) = 0.02632, ou 1 chance em 38 por mes
frequencia mensal medida no treino: p = 5/108 = 0.04630
p mensal da cauda do treino (ajuste POT/GPD publicado: xi = 0.406, sigma = 7.387, 83 excessos acima de 10): p = 0.04296
  frequencia medida  p = 0.04630: P(janela limpa | perigoso) = 0.1815; razao de verossimilhanca = 5.35 (2.42 bits); meses limpos para 95% = 64
                     informacao mutua janela x (perigoso vs desprezivel) = 0.5455 bits
  cauda POT/GPD      p = 0.04296: P(janela limpa | perigoso) = 0.2058; razao de verossimilhanca = 4.72 (2.24 bits); meses limpos para 95% = 69
                     informacao mutua janela x (perigoso vs desprezivel) = 0.5147 bits
referencia: sob a taxa que o cofre mostrou (2 meses atingidos em 24, p = 1/12), a mesma janela limpa valeria 4.48 bits (razao 22.3)
== Capítulo 19 :: o portão do conjunto invariante ==
semente 2019 | 3 ambientes x 600 linhas | 999 permutacoes por teste | alfa = 0.05

mundo A: o ambiente intervem em X (3 escolas, 4 candidatos)
  conjunto                           p_local p_escala   p_conj  veredito      1-NN
  {}                                  0.0010   0.8500   0.0020  rejeitado    0.333
  {x1 estudo}                         0.0010   0.4260   0.0020  rejeitado    0.477
  {x2 sono}                           0.0010   0.0610   0.0020  rejeitado    0.702
  {x3 ruido}                          0.0010   0.8350   0.0020  rejeitado    0.343
  {x4 transporte}                     0.0010   0.9010   0.0020  rejeitado    0.725
  {x1 estudo, x2 sono}                0.9920   0.6480   1.0000  aceito       0.743
  {x1 estudo, x2 sono, x4 transporte}  0.9910   0.6150   1.0000  aceito       0.868
  aceitos na familia: 4 de 16 (com os saturados seriam 4) | saturados testados: 0 | aceitos saturados: 0
  S_hat com a regra de saturacao = {x1 estudo, x2 sono}  | sem a regra = {x1 estudo, x2 sono}  | portao: PASSA (S_hat nao vazio)

mundo B: o ambiente age direto em Y (o ano, 3 candidatos)
  conjunto                           p_local p_escala   p_conj  veredito      1-NN
  {}                                  0.0010   0.8460   0.0020  rejeitado    0.333
  {x1 estudo}                         0.0010   0.6370   0.0020  rejeitado    0.477
  {x2 sono}                           0.0010   0.3730   0.0020  rejeitado    0.702
  {x3 ruido}                          0.0010   0.8280   0.0020  rejeitado    0.343
  {x1 estudo, x2 sono}                0.0010   0.0350   0.0020  rejeitado    0.743
  aceitos na familia: 0 de 8 (com os saturados seriam 0) | saturados testados: 0 | aceitos saturados: 0
  S_hat com a regra de saturacao = {}  | sem a regra = {}  | portao: FALHA (S_hat vazio)

mundo C: mundo A mais x5, a coluna que e o proprio ambiente (5 candidatos)
  conjunto                           p_local p_escala   p_conj  veredito      1-NN
  {}                                  0.0010   0.8500   0.0020  rejeitado    0.333
  {x1 estudo}                         0.0010   0.4260   0.0020  rejeitado    0.477
  {x2 sono}                           0.0010   0.0610   0.0020  rejeitado    0.702
  {x3 ruido}                          0.0010   0.8350   0.0020  rejeitado    0.343
  {x4 transporte}                     0.0010   0.9010   0.0020  rejeitado    0.725
  {x1 estudo, x2 sono}                0.9920   0.6480   1.0000  aceito       0.743
  {x1 estudo, x2 sono, x4 transporte}  0.9910   0.6150   1.0000  aceito       0.868
  {x5 turma}                          0.9640   0.9990   1.0000  aceito       1.000
  {x1 estudo, x2 sono, x5 turma}      0.9410   1.0000   1.0000  aceito       1.000
  aceitos na familia: 4 de 32 (com os saturados seriam 20) | saturados testados: 16 | aceitos saturados: 16
  S_hat com a regra de saturacao = {x1 estudo, x2 sono}  | sem a regra = {}  | portao: PASSA (S_hat nao vazio)
== Capítulo 20 :: o relogio, o canario e o cofre ==
semente 2021 | treino 120000 paradas (taxa 0.0352) | desenvolvimento 58000 (taxa 0.0222) | cofre 24466 (taxa 0.0220)
modelo cru (treinado no periodo antigo de taxa mais alta): media prevista 0.0351 (60% acima da taxa do cofre)
  CITL = +0.0132 | ECE = 0.0132 | Brier = 0.02155 | habilidade = -0.0020
modelo oficial com Platt (a = 0.98, b = -0.54): media prevista 0.0222
  CITL = +0.0002 | ECE = 0.0022 | Brier = 0.02130 | habilidade = +0.0098
climatologia fixa na taxa do passado: Brier = 0.02151 (p(1-p) = 0.02171)

canario: uma coluna igual ao rotulo, publicada no instante do rotulo
  relogio honesto: canario disponivel em 0 de 24466 emissoes | beta_canario = +0.00 | ganho de habilidade = +0.0000
  relogio desligado: canario disponivel em 24466 de 24466 emissoes | beta_canario = +9.23 | ganho de habilidade = +0.6521
== Capítulo 21 :: o verificador da ficha do projeto final ==
sete regras, e cada uma tem de reprovar ao menos uma ficha:

  1. R1 processo gerador declarado
  2. R2 unidade independente declarada
  3. R3 modelo nulo com a mesma semente
  4. R4 pergunta do decisor (quem decide, qual ação, pergunta)
  5. R5 semente fixa numerica
  6. R6 hash do pre-registro com 64 digitos hexadecimais
  7. R7 clausula de saida com gatilho objetivo

01 completa                  ACEITA (quem decide: a secretaria municipal de educacao)

02 sem pergunta do decisor   RECUSA
    R4 campo pergunta_do_decisor ausente

03 quem decide generico      RECUSA
    R4 quem decide e generico

04 a acao nao aparece literal RECUSA
    R4 a acao da decisao nao aparece literalmente

05 sem interrogar            RECUSA
    R4 o texto nao termina em pergunta

06 sem semente               RECUSA
    R5 sem semente fixa numerica

07 sem clausula de saida     RECUSA
    R7 sem clausula de saida

08 clausula sem gatilho      RECUSA
    R7 clausula de saida sem gatilho objetivo

09 sem pre-registro          RECUSA
    R6 hash do pre-registro ausente ou fora do formato

10 sem modelo nulo           RECUSA
    R3 modelo nulo ausente ou sem a mesma semente

11 sem gerador               RECUSA
    R1 campo gerador vazio ou curto demais

12 sem unidade               RECUSA
    R2 campo unidade vazio

12 fichas: 1 aceita, 11 recusadas
regras que reprovaram ao menos uma ficha: 7 de 7 | reprovacoes por regra: [1, 1, 1, 4, 1, 1, 2]
~~~

## 3. Os portoes deterministicos

### Sete portoes e a banca adversarial

Sete portoes (G1 a G7), reproducao identica e raiz de Merkle da trilha. O portao G6 e a banca A1-A8.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/portoes --horizonte todos --reproducao
~~~

~~~text
== Estocologia :: portoes deterministicos ==
horizonte=todos fichas=12 defeito=nenhum
  ficha h0.posto-saude severidade_min=8 aprovada=true
  ficha h0.banco-2003 severidade_min=8 aprovada=true
  ficha h1.a-multiplicativo severidade_min=8 aprovada=true
  ficha h1.b-lalonde severidade_min=8 aprovada=true
  ficha h1.c-incendios severidade_min=8 aprovada=true
  ficha h2.icp-tunel-de-luz severidade_min=8 aprovada=true
  ficha h2.icp-fda-herg severidade_min=8 aprovada=true
  ficha h2.icp-estrela-levedura severidade_min=8 aprovada=true
  ficha h3.voos23 severidade_min=8 aprovada=true
  ficha h3.bolsa severidade_min=8 aprovada=true
  ficha h3.energia severidade_min=8 aprovada=true
  ficha h3.prisoes severidade_min=8 aprovada=true
  lentes: hill_alfa=1.7901 cauda_selvagem=true mediana_log_riqueza=-10.5361 arruinadas=0.892 murphy_rel=0.0375 res=0.1719 unc=0.2344
raiz_merkle=9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81
reproducao: segunda execucao identica = true (raiz 9716e28a87bec9cf)
G1 ok       determinismo da raiz 9716e28a87bec9cf
G2 ok       semente fixa em 12 fichas
G3 ok       escopo declarado: so artefato simulado
G4 ok       Phi1-Phi5 e tipo III (falhas=0)
G5 ok       corrente SHA-256 integra e adulteracao detectada
G6 ok       banca A1-A8 com piso 6 e pior 8
G7 ok       clausula de saida declarada e A8 acima do piso
hash_do_relatorio=08e7c8f0e8e706f9bf95286dd5f6b265c965125e5f496c83b8d9417f53d0e5c2
veredito: 7 portoes ok
~~~

### Prova negativa: o portao sabe reprovar

Com um defeito plantado, o G6 reprova e o binario sai com codigo 1.

~~~bash
/home/silvano-neto/Documents/estocologia/target/release/portoes --horizonte h3 --defeito h3.voos23
~~~

~~~text
== Estocologia :: portoes deterministicos ==
horizonte=h3 fichas=4 defeito=h3.voos23
  ficha h3.voos23 severidade_min=2 aprovada=false
  ficha h3.bolsa severidade_min=8 aprovada=true
  ficha h3.energia severidade_min=8 aprovada=true
  ficha h3.prisoes severidade_min=8 aprovada=true
  lentes: hill_alfa=1.7901 cauda_selvagem=true mediana_log_riqueza=-10.5361 arruinadas=0.892 murphy_rel=0.0375 res=0.1719 unc=0.2344
raiz_merkle=802e84b64e432c259582d4d1322d602ea9634b8a05f8421e8205f3f9672252d2
G1 ok       determinismo da raiz 802e84b64e432c25
G2 ok       semente fixa em 4 fichas
G3 ok       escopo declarado: so artefato simulado
G4 ok       Phi1-Phi5 e tipo III (falhas=0)
G5 ok       corrente SHA-256 integra e adulteracao detectada
G6 reprovado banca A1-A8 com piso 6 e pior 2
G7 ok       clausula de saida declarada e A8 acima do piso
hash_do_relatorio=bb012a0c50679cdd1d58b9717c5e94e40a005a6727d0bd0713a9b2f749718277
veredito: 1 portoes reprovados
~~~

## 4. Os testes

### Os quinze crates

Cada crate com o seu bloco de testes.

~~~bash
cargo test --workspace
~~~

~~~text
running 9 tests
test cofre::testes::abertura_unica_e_a_segunda_falha_registrada ... ok
test cofre::testes::adulterar_o_registro_invalida_a_verificacao ... ok
test cofre::testes::corte_degenerado_serie_vazia_e_serie_desordenada_sao_recusados ... ok
test cofre::testes::errar_depois_da_abertura_exige_cofre_novo_e_nao_reajuste ... ok
test cofre::testes::raiz_de_merkle_muda_quando_o_conteudo_muda ... ok
test cofre::testes::separacao_por_indice_e_por_tempo_nao_se_sobrepoem ... ok
test testes::corrente_detecta_adulteracao ... ok
test testes::merkle_e_reproduzivel ... ok
test testes::sha256_bate_com_os_vetores_conhecidos ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 43 tests
test banco::testes::agente_a_mais_melhora_o_nivel ... ok
test banco::testes::erlang_c_cresce_com_a_carga ... ok
test banco::testes::escalar_pela_meta_devolve_meta ... ok
test cap01::testes::duas_maquinas_concordam_lance_a_lance ... ok
test cap01::testes::moeda_honesta_concentra_perto_de_meio ... ok
test cap01::testes::sementes_diferentes_quase_nao_coincidem ... ok
test cap02::testes::coincidencia_rara_nao_vira_culpa_em_populacao_pequena ... ok
test cap02::testes::o_fator_da_troca_cresce_quando_a_prevalencia_cai ... ok
test cap02::testes::quando_a_prevalencia_e_alta_a_ponte_quase_nao_mente ... ok
test cap03::testes::dez_de_razao_valem_dez_decibans ... ok
test cap03::testes::dois_exames_independentes_contra_exames_correlacionados ... ok
test cap03::testes::mamografia_da_uma_em_dez ... ok
test cap04::testes::intervalo_fixo_encolhe_e_o_descontado_fica_largo ... ok
test cap04::testes::modelo_fixo_erra_quase_sempre_e_o_modelo_com_esquecimento_cobre ... ok
test cap04::testes::o_vies_muda_de_dois_decimos_a_seis_decimos ... ok
test escuta::testes::a_cobertura_minima_abre_a_caixa ... ok
test escuta::testes::a_conducao_declarada_e_protocolo_cego ... ok
test escuta::testes::a_decisao_emitida_sem_ninguem_ouvido_reprova ... ok
test escuta::testes::a_escuta_sem_registro_assinado_nao_conta ... ok
test escuta::testes::o_prazo_e_a_cobertura_minima_saem_do_registro ... ok
test escuta::testes::o_registro_do_programa_tem_sete_decisoes_e_nenhum_ouvido ... ok
test fila::testes::a_margem_de_r2_e_o_p90_do_cv_residual ... ok
test fila::testes::dia_da_semana_de_datas_conhecidas ... ok
test fila::testes::faixas_do_registro_e_divergencia_declarada ... ok
test fila::testes::fila_com_desistencia_estabiliza ... ok
test fila::testes::grade_e_o_recorte_do_instrumento ... ok
test fila::testes::impressao_de_trabalho ... ok
test fila::testes::instrumento_roda_com_semente_fixa ... ok
test fila::testes::monotonicidade_r2_melhor_que_r0 ... ok
test ponte::testes::a_escolha_e_mecanica_deterministica_e_na_proporcao ... ok
test ponte::testes::a_lista_que_nao_sustenta_a_proporcao_reprova ... ok
test ponte::testes::com_os_papeis_designados_o_teste_pode_rodar ... ok
test ponte::testes::o_piloto_re_deriva_o_empate_no_teto ... ok
test ponte::testes::o_registro_declara_o_veredito_se_o_prazo_vencer ... ok
test ponte::testes::o_registro_do_teste_justo_declara_o_que_falta ... ok
test ponte::testes::o_status_em_teste_sem_os_papeis_reprova ... ok
test ponte::testes::uma_tese_sustentada_sem_teste_justo_e_reprovada ... ok
test segunda::testes::a_releitura_e_os_pareceres_sao_contados ... ok
test segunda::testes::a_reproducao_das_prioris_tem_33_dominios_e_uma_divergencia_declarada ... ok
test segunda::testes::o_banco_e_recontado_em_rust_e_bate_com_o_publicado ... ok
test segunda::testes::o_registro_declara_as_classes ... ok
test segunda::testes::o_vies_do_laco_concorda_em_duas_fontes ... ok
test segunda::testes::os_limiares_da_multiplicidade_batem_com_alfa_sobre_m ... ok

test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 59 tests
test agentes::testes::falta_de_cuidado_reprova_o_agente_da_faixa ... ok
test agentes::testes::oito_agentes_aprovam_ficha_com_cuidados ... ok
test agentes::testes::severidade_reproduz_com_a_mesma_semente ... ok
test camada::testes::com_resolucao_troca_a_resolucao_e_completa_preserva ... ok
test camada::testes::nova_traz_r1_dia_como_resolucao_padrao ... ok
test erlang::testes::a_escala_minima_entrega_a_meta_e_uma_a_menos_nao ... ok
test erlang::testes::erlang_b_e_erlang_c_ficam_na_faixa ... ok
test funil::testes::a_cota_superior_limita_o_evsi_de_qualquer_exame ... ok
test funil::testes::a_poda_declara_sem_incerteza_no_extremo ... ok
test funil::testes::a_poda_dispara_quando_o_evsi_nao_cobre_o_limiar ... ok
test funil::testes::a_poda_nao_dispara_quando_o_evsi_cobre ... ok
test funil::testes::evsi_medido_bate_com_o_exato ... ok
test funil::testes::exame_mais_acurado_nao_vale_menos ... ok
test funil::testes::f1_recusa_ficha_sem_pergunta_do_decisor ... ok
test funil::testes::f2_recusa_ficha_sem_processo_gerador ... ok
test funil::testes::f2_recusa_tambem_ficha_sem_semente ... ok
test funil::testes::f3_recusa_ficha_sem_modelo_nulo ... ok
test funil::testes::f3_recusa_ficha_sem_unidade_independente ... ok
test funil::testes::f4_recusa_ficha_sem_acao_declarada ... ok
test funil::testes::f4_recusa_ficha_sem_clausula_de_saida ... ok
test funil::testes::ficha_completa_passa_pelos_quatro_filtros ... ok
test funil::testes::o_contador_soma_por_portao_e_guarda_a_sequencia ... ok
test funil::testes::o_funil_conta_as_tentativas_ate_a_versao_final ... ok
test funil::testes::o_funil_para_no_primeiro_portao ... ok
test invariantes::testes::acao_agregada_contra_unidade_dia_reprova_phi5 ... ok
test invariantes::testes::codigo_do_portao_phi5_aparece_no_relatorio_de_falhas ... ok
test invariantes::testes::ficha_completa_em_r1_dia_passa_phi5 ... ok
test invariantes::testes::ficha_completa_passa ... ok
test invariantes::testes::resolucao_mais_fina_que_o_dia_tambem_passa ... ok
test invariantes::testes::sem_pergunta_do_decisor_reprova_tipo_iii ... ok
test invariantes::testes::sem_semente_reprova_phi1 ... ok
test invariantes::testes::sem_unidade_o_phi5_nao_acusa_e_o_phi2_sim ... ok
test invariantes::testes::unidade_de_semana_tambem_reprova_com_r0_agregada ... ok
test invariantes::testes::unidade_mais_fina_que_o_dia_fica_fora_do_phi5_declarado ... ok
test orquestrador::testes::ataque_unico_roda ... ok
test orquestrador::testes::banca_reprova_ficha_incompleta ... ok
test resolucao::testes::a_checagem_da_implicacao_acha_o_contraexemplo_e_nao_a_violacao ... ok
test resolucao::testes::a_conservacao_reprova_um_agregado_que_nao_bate ... ok
test resolucao::testes::a_ordem_das_resolucoes_e_a_da_finura ... ok
test resolucao::testes::a_transferencia_nao_tem_violacao_e_tem_contraexemplo ... ok
test resolucao::testes::abaixo_do_limiar_refinar_compra_menos_do_que_acima ... ok
test resolucao::testes::com_o_mesmo_recurso_a_pergunta_fina_espera_menos ... ok
test resolucao::testes::com_o_plano_calibrado_o_limiar_existe_e_a_fracao_cresce ... ok
test resolucao::testes::com_o_plano_de_erlang_a_previsao_3_nao_se_sustenta ... ok
test resolucao::testes::sem_variacao_nao_ha_perda_de_resolucao ... ok
test rng::testes::mesma_semente_mesma_sequencia ... ok
test rng::testes::sementes_diferentes_divergem ... ok
test s1::tests::caminho_felizes_cada_tema_acha_a_sua_categoria ... ok
test s1::tests::caso_que_tem_de_falhar_a_ficha_com_evidencia_tematica_nao_cai_em_outros ... ok
test s1::tests::caso_que_tem_de_falhar_casamento_curto_nao_vira_evidencia ... ok
test s1::tests::desempate_e_pela_ordem_declarada ... ok
test s1::tests::determinismo_mesma_entrada_mesma_saida ... ok
test s1::tests::entrada_desconhecida_cai_em_outros ... ok
test s1::tests::leitura_do_registro ... ok
test s1::tests::nenhuma_probabilidade_em_zero_ou_um ... ok
test s1::tests::normalizacao_e_estavel ... ok
test s1::tests::reprojecao_do_teto_preserva_a_soma ... ok
test s1::tests::soma_um_depois_do_ajuste ... ok
test s1::tests::sufixo_longo_vence_tema_curto ... ok

test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 5 tests
test testes::a_kl_e_zero_contra_si_mesma_e_cresce_com_a_distancia ... ok
test testes::a_vigilancia_e_deterministica ... ok
test testes::o_nulo_nao_dispara_em_cem_mil_pontos ... ok
test testes::o_salto_dispara_o_alarme_depois_de_acontecer ... ok
test testes::o_salto_para_baixo_tambem_dispara ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 4 tests
test testes::a_calibracao_acha_a_regiao_descalibrada ... ok
test testes::a_paridade_mede_o_gap_injetado ... ok
test testes::a_tolerancia_da_ece_cai_com_a_raiz_de_n ... ok
test testes::grupos_iguais_nao_tem_amplitude ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 9 tests
test testes::a_cauda_pesada_estoura_o_indice_de_dispersao ... ok
test testes::a_recorrencia_e_a_soma_logaritmica_concordam ... ok
test testes::a_serie_do_nulo_e_deterministica ... ok
test testes::erlang_bate_com_a_referencia_exata ... ok
test testes::erlang_c_cai_com_mais_servidores_e_fica_entre_b_e_um ... ok
test testes::escala_minima_entrega_a_meta_e_uma_a_menos_nao ... ok
test testes::nulo_de_poisson_bate_media_e_variancia ... ok
test testes::o_resumo_traz_mediana_e_quantis_junto_com_a_media ... ok
test testes::quantil_interpola_e_nao_sai_da_amostra ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 6 tests
test testes::a_mediana_e_a_media_contam_historias_diferentes ... ok
test testes::arrasto_cresce_com_a_alavancagem ... ok
test testes::arrasto_de_serie_simetrica_tem_valor_exato ... ok
test testes::hill_recupera_o_alfa_de_uma_pareto ... ok
test testes::o_dia_de_1987_derruba_quem_esta_a_tres_vezes ... ok
test testes::o_vies_de_selecao_inverte_o_sinal_do_ganho ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 13 tests
test genetica::testes::a_segunda_linha_separa_o_artefato_do_mover ... ok
test genetica::testes::com_a_segunda_linha_todo_efeito_tem_intervalo ... ok
test genetica::testes::o_artefato_de_linha_passa_sem_a_segunda_linha ... ok
test genetica::testes::o_desenho_e_deterministico ... ok
test genetica::testes::o_piso_cai_com_a_potencia_das_linhas ... ok
test genetica::testes::sem_a_segunda_linha_o_intervalo_e_um_ponto ... ok
test testes::a_rodada_guarda_a_area_e_a_contagem_de_conjuntos ... ok
test testes::bonferroni_derruba_uma_area_e_conta_o_k ... ok
test testes::dez_candidatos_dao_mil_e_vinte_e_quatro_conjuntos ... ok
test testes::o_contador_k_junta_especificacoes_repetidas ... ok
test testes::o_grao_da_unidade_muda_a_familia_aceita ... ok
test testes::o_nulo_embaralhado_mostra_o_poder_do_teste ... ok
test testes::sem_area_a_rodada_devolve_area_vazia ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 40 tests
test beta::testes::a_incompleta_bate_com_a_forma_fechada_da_beta_2_2 ... ok
test beta::testes::a_incompleta_bate_com_a_uniforme ... ok
test beta::testes::o_ajuste_da_uniforme_fica_simetrico ... ok
test beta::testes::o_ajuste_e_deterministico ... ok
test beta::testes::o_ajuste_recupera_a_beta_que_gerou_os_quartis ... ok
test beta::testes::o_log_gama_bate_com_fatoriais ... ok
test beta::testes::o_quantil_recupera_os_quartis_conhecidos ... ok
test cadeia::testes::a_cadeia_tem_quatro_linhas_com_as_marcas_do_registro ... ok
test cadeia::testes::o_instrumento_fraco_indecide ... ok
test cadeia::testes::o_laco_e_re_derivado_do_registro_versionado ... ok
test cadeia::testes::o_leitor_de_json_le_o_registro_real ... ok
test cadeia::testes::o_registro_que_mente_na_razao_e_reprovado ... ok
test cadeia::testes::o_veredito_vira_quando_a_razao_entra_na_banda ... ok
test cadeia::testes::uma_linha_derivada_sem_registro_versionado_reprova ... ok
test preteste::testes::a_caixa_na_media_tem_p_proximo_de_um ... ok
test preteste::testes::a_identidade_de_murphy_fecha_exata ... ok
test preteste::testes::o_criterio_4_do_protocolo_nao_sabe_reprovar ... ok
test preteste::testes::o_desenho_do_protocolo_poe_dois_itens_por_caixa ... ok
test preteste::testes::o_exagerado_reprova_por_ece_e_por_caixa ... ok
test preteste::testes::o_nulo_do_pre_teste_cresce_quando_a_amostra_encolhe ... ok
test preteste::testes::o_nulo_e_deterministico ... ok
test preteste::testes::o_p_e_a_unidade_do_protocolo ... ok
test preteste::testes::o_teste_binomial_nao_reprova_a_caixa_calibrada ... ok
test preteste::testes::o_teste_binomial_reprova_a_caixa_impossivel ... ok
test preteste::testes::probabilidade_invertida_reprova_por_monotonia ... ok
test preteste::testes::um_pre_teste_calibrado_passa_nos_quatro_criterios ... ok
test shelf::testes::a_ancora_e_o_deslocamento_tem_limites_declarados ... ok
test shelf::testes::a_ancora_preve_o_que_a_prioris_diz ... ok
test shelf::testes::a_largura_implicada_bate_a_elicitada_quando_os_extremos_sao_da_normal ... ok
test shelf::testes::extremos_estreitos_reprovam_no_controle ... ok
test shelf::testes::o_ajuste_simetrico_da_media_no_q50 ... ok
test shelf::testes::phi_bate_com_a_tabela ... ok
test shelf::testes::quartis_assimetricos_deixam_erro_visivel ... ok
test shelf::testes::recupera_a_normal_que_gerou_os_quartis ... ok
test testes::a_correlacao_passiva_segue_a_conta_exata ... ok
test testes::a_ece_e_zero_no_calibrado_e_alta_no_exagerado ... ok
test testes::a_habilidade_de_brier_premeia_o_bom_e_pune_o_mau ... ok
test testes::a_tolerancia_da_ece_depende_de_n ... ok
test testes::o_canario_pega_a_leitura_do_futuro ... ok
test testes::o_laco_mede_o_que_a_acao_faz_com_o_indicador ... ok

test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 12 tests
test lentes::testes::arrasto_da_volatilidade_desconta_variacao ... ok
test lentes::testes::brier_e_murphy_fecham_a_conta ... ok
test lentes::testes::cadeia_deterministica_respeita_processamento ... ok
test lentes::testes::evpi_do_exemplo_vale_tres ... ok
test lentes::testes::hill_recupera_alfa_de_pareto ... ok
test lentes::testes::inclinacao_simples ... ok
test lentes::testes::moeda_de_peters_nao_e_ergodica ... ok
test lentes::testes::regime_invariante_aprova_regime_quebrado ... ok
test testes::aresta_para_no_ausente_reprova ... ok
test testes::ciclo_dedutivo_reprova ... ok
test testes::grafo_completo_valida ... ok
test testes::no_sem_lente_e_apontado ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 8 tests
test testes::a_unidade_de_grupo_encolhe_a_familia ... ok
test testes::alvo_como_preditor_reprova ... ok
test testes::area_vazia_quando_nada_e_invariante ... ok
test testes::contagem_de_conjuntos_bate_com_a_enumeracao ... ok
test testes::ks_conhece_o_proprio_limiar ... ok
test testes::limiar_cai_com_o_tamanho_da_amostra ... ok
test testes::preditor_invariante_e_aceito_e_confundido_e_recusado ... ok
test testes::preditores_colineares_reprovam ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 36 tests
test crenca::testes::a_evidencia_de_ficha_move_a_crenca_para_a_camada_da_ficha ... ok
test crenca::testes::evidencia_invalida_nao_zera_o_estado ... ok
test crenca::testes::o_estado_de_crenca_nunca_bate_no_zero_nem_no_um ... ok
test crenca::testes::o_ranking_desempata_pela_ordem_declarada ... ok
test estresse::testes::a_crenca_aprende_a_faixa_fraca_e_ataca_ela_primeiro ... ok
test estresse::testes::a_crenca_da_banca_respeita_o_piso_e_o_teto ... ok
test estresse::testes::ficha_com_os_oito_cuidados_nao_reprova_em_nenhuma_ordem ... ok
test estresse::testes::o_ataque_e_deterministico ... ok
test estresse::testes::sem_aprendizado_a_ordem_deliberativa_e_a_declarada ... ok
test hipergrafo::testes::a_acao_declara_o_tipo_da_aresta ... ok
test hipergrafo::testes::a_busca_e_deterministica_no_grafo_de_teste ... ok
test hipergrafo::testes::a_busca_no_grafo_da_amostra_chega_na_frente_sem_lente ... ok
test hipergrafo::testes::a_cobertura_conta_lentes_distintas ... ok
test hipergrafo::testes::a_deliberacao_acha_o_no_sem_lente_onde_o_ganancioso_para_antes ... ok
test hipergrafo::testes::no_ausente_do_grafo_nao_vale_nada ... ok
test hipergrafo::testes::o_grafo_da_amostra_e_valido_e_tem_um_no_descoberto ... ok
test inferencia::testes::a_cadeia_e_deterministica_na_semente ... ok
test inferencia::testes::a_cadeia_recupera_a_media_de_uma_normal ... ok
test inferencia::testes::a_cadeia_recupera_a_posterior_conjugada_da_beta ... ok
test inferencia::testes::alvo_fora_do_suporte_nao_aceita_nada ... ok
test inferencia::testes::passo_grande_derruba_a_aceitacao ... ok
test lats::testes::a_decisao_e_deterministica_na_semente ... ok
test lats::testes::a_deliberacao_bate_o_ganancioso ... ok
test lats::testes::mundo_sem_acao_nao_decide ... ok
test planejamento::testes::a_avaliacao_declara_os_degraus_fora_do_alcance ... ok
test planejamento::testes::a_escada_de_resolucao_vem_do_core ... ok
test planejamento::testes::com_oscilacao_a_perda_paga_a_margem ... ok
test planejamento::testes::margem_cara_volta_para_o_agregado ... ok
test planejamento::testes::sem_oscilacao_a_perda_e_zero_e_o_agregado_basta ... ok
test ponte::testes::a_margem_no_extremo_e_recusada ... ok
test ponte::testes::a_prova_de_determinismo_passa_no_modelo_de_ensaio ... ok
test ponte::testes::a_prova_de_determinismo_reprova_o_modelo_instavel ... ok
test ponte::testes::a_saida_fora_do_esquema_vira_abstencao ... ok
test ponte::testes::o_custo_por_chamada_e_a_adequacao_saem_do_ensaio ... ok
test ponte::testes::pesos_com_hash_diferente_recusam ... ok
test ponte::testes::sem_pesos_verificados_a_ponte_nao_roda ... ok

test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 13 tests
test testes::a_cadeia_do_arquivo_e_continua_e_termina_na_versao_da_ficha ... ok
test testes::a_cadeia_nao_mexe_num_documento_ja_na_versao_da_ficha ... ok
test testes::cadeia_aplica_renomeacao_descarte_e_acrescimo ... ok
test testes::cadeia_com_buraco_reprova ... ok
test testes::campo_ausente_vira_none ... ok
test testes::documento_sem_versao_reprova ... ok
test testes::etapa_ausente_reprova_com_a_versao_do_buraco ... ok
test testes::quebra_de_linha_na_definicao_sobrevive ... ok
test testes::texto_e_de_texto_preservam_a_ficha ... ok
test testes::um_documento_da_versao_1_0_migra_com_os_padroes_declarados ... ok
test testes::uma_cadeia_com_buraco_ou_operacao_desconhecida_reprova ... ok
test testes::valor_invalido_reprova ... ok
test testes::versao_de_texto_reprova_texto_livre ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 3 tests
test testes::a_moeda_bate_a_probabilidade_e_nao_depende_do_relogio ... ok
test testes::a_ponte_entrega_os_mesmos_numeros_da_funcao_pura ... ok
test testes::a_probabilidade_nunca_bate_nos_extremos ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in Xs
~~~
