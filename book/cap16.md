# Parte V. A máquina no mundo real

## Capítulo 16. Horizonte H0: o posto de saúde e os 2,65 milhões de chamadas do banco

**Objetivos.** Ao fim deste capítulo você deve saber por que a mesma demanda média pode produzir duas esperas
muito diferentes, quanto a convexidade da fila cobra de quem decide, por que a unidade independente de uma margem
pode ser o paciente ou o dia e o que a fórmula do setor prometia a uma central de chamadas que ela não
entregou.

---

### Degrau 1 — O mundo real e o paradoxo

**O posto de saúde.** Num posto com um só enfermeiro, o atendimento leva em média 5 minutos por paciente, e a conta
fecha em 12 atendimentos por hora. A demanda é que oscila. Em metade dos dias chegam 5 pacientes por hora, e na
outra metade chegam 11. A média é 8, e 8 é menos que 12, então a fila não cresce sem fim.

O gestor olha a planilha e escreve a frase de sempre: *"a demanda média é 8 e o posto atende 12; a espera é de uns
10 minutos e não há por que contratar"*. A conta por trás dessa frase está certa, e vale 10 minutos, mas só num
mundo em que chegam exatamente 8 pacientes por hora, toda hora. O problema é que esse mundo não é o mundo.

Simulados os dois mundos com a mesma semente (2 000 dias de 10 horas cada, um atendente, semente 1909), a
diferença aparece inteira.

| Mundo (pacientes por hora) | Espera média por paciente | O paciente típico espera | 90 em 100 esperam menos de | Em cada 100 pacientes, mais de 30 min |
| --- | --- | --- | --- | --- |
| Sempre 8, todo dia | 9,3 min | 3,8 min | 26,7 min | 8 |
| Metade dos dias com 5, metade com 11 | 20,7 min | 9,3 min | 59,3 min | 25 |

Nenhuma média mudou. São 8 nos dois casos. O que mudou foi o mundo, e a espera é **curva** na demanda. Quem chega num
dia calmo espera pouco, mas quem chega num dia de surto espera muito, e o ganho do dia calmo não compensa o
estrago do dia cheio. O dia de surto responde por cerca de 70 em cada 100 pacientes (são 110 pacientes contra 50),
de modo que a média por paciente pesa justamente os dias ruins.

**A armadilha da mediana.** Leia a segunda linha da tabela de novo. No mundo que oscila, **o paciente típico
espera 9,3 minutos**. É o mesmo número que a alegação prometia como média. Quem olha o "paciente típico" vê a
promessa cumprida; a diferença inteira está na cauda, onde 25 em cada 100 pacientes passam de meia hora de espera.

**Reforço só nos dias de surto.** Com dois enfermeiros nos
dias de 11 pacientes e um nos dias de 5, a espera média cai para 2,5 minutos e menos de 1 paciente em cada 100
passa de 30 minutos. A folga não precisa ser maior; precisa estar onde está o risco.

**A pergunta escondida é quantos pacientes valem uma margem.** Cada margem de erro precisa de uma unidade. Se o
posto mede a espera média e trata cada paciente como uma observação independente, a margem fica em torno de
±0,07 minuto; se trata cada *dia* como a unidade, ela fica em ±0,51 minuto, sete vezes maior. Qual das duas vale
não é questão de gosto. É o mundo que decide. O teste é direto. Pegue 4 000 dias do mundo da alegação, reparta-os
ao acaso em duas metades e meça a diferença de espera média entre elas. Se os pacientes fossem independentes, essa
diferença deveria andar na casa de 0,047 minuto. Ela anda em 0,216, e a previsão pela unidade *dia*, 0,227, acerta
na mosca. Os pacientes de um mesmo dia compartilham a mesma demanda, e compartilhar não é ser independente.

**O caso real é a central de um banco.** O posto era uma suposição; a central é observação. São 2,65 milhões de
chamadas reais de uma central bancária norte-americana, contadas a cada 5 minutos, ao longo de 164 dias úteis de
2003. A escala é construída nos 83 dias de treino (março a junho), com pico de 234 atendentes e 2 264
agente-horas por dia, e aplicada aos 81 dias de cofre (julho a outubro). A alegação é a prática padrão do
setor: *escalar cada meia hora pela fórmula de Erlang C sobre a taxa média do histórico garante 82% das chamadas
atendidas em até 20 segundos, todo dia.*

A base mostrou isto.

- **A demanda não é Poisson.** Entre dias, o desvio é de cerca de 2 900 chamadas, onde o modelo Poisson preveria
  180; a razão variância/média é 273. Parte disso é calendário (segunda-feira tem +15%), e o resíduo depois do dia
  da semana ainda tem coeficiente de variação de 6%.
- **Jensen mora dentro da própria fórmula.** Aplicado à taxa real de cada dia, o Erlang C da alegação dá 57% em
  média, e não 82%. O nível de serviço é côncavo na carga.
- **A unidade é o dia.** O nulo de metades dá desvio de 0,0474, contra 0,0470 previsto pelo dia e 0,0033 previsto
  pela chamada.
- **Teste severo.** Em 400 dias simulados no mundo que a alegação descreve, nenhum ficou abaixo da meta. No mundo
  real, cerca de um em cada quatro dias ficou.
- **Fora do tempo.** O intervalo conforme de 90% para o total de chamadas do dia, construído sobre a média do dia
  da semana, cobriu 86% dos dias do cofre, e a deriva entre treino e cofre é leve (KS 0,19, p = 0,08).

A decisão também está medida no cofre, contra as chegadas reais:

| Regra de escala | Agente-horas por dia | Em cada 100 dias, dias abaixo da meta | Nível de serviço |
| --- | --- | --- | --- |
| A alegação: perfil médio | 2 264 | 26 | 86% |
| Perfil do dia da semana (custo zero) | 2 261 | 12 | 90% |
| Perfil do dia da semana + margem para a variação residual | 2 428 | 4 (3,7) | 96% |
| A alegação + 12 atendentes por igual | 2 433 | 7 (7,4) | 94% |

As duas últimas regras custam quase o mesmo (cerca de 7% a mais de agente-horas) e colocam a folga em lugares
diferentes. A que põe a folga onde está o risco derruba a falha de 26 para menos de 4 dias em cada 100; a que
espalha atendentes por igual derruba só até 7.

---

### Degrau 2 — A regra do jogo

**A fila, em palavras.** Um posto tem uma taxa de chegada (pacientes por hora) e uma taxa de atendimento
(pacientes por hora). Se a chegada fica abaixo do atendimento, a fila não explode para o infinito; oscila em
torno de um tamanho médio. A espera média de longo prazo de uma fila com um atendente, chegadas sem memória e
atendimento sem memória é

$$W(\lambda)=\frac{\lambda}{\mu\,(\mu-\lambda)},$$

em que $\lambda$ é a taxa de chegada e $\mu$ a de atendimento, ambas na mesma unidade de tempo. Com $\lambda = 8$,
$\mu = 12$ e a hora como unidade, $W = 8/(12 \cdot 4) = 1/6$ de hora, ou 10 minutos. A condição em que a fórmula
vale está escrita na expressão: $0 \le \lambda < \mu$, demanda que não muda e um horizonte longo. Quando
$\lambda$ se aproxima de $\mu$, o denominador vai a zero e a espera explode; é a curvatura que o Degrau 1 mostrou,
agora em fórmula.

**Onde a fórmula falha.** Aplicada a dias que alternam entre 5 e 11 pacientes por hora, ela daria
$(W(5)+W(11))/2 = 29,3$ minutos. A simulação dá 20,7. A diferença ensina três coisas. Primeiro, um dia de 10 horas
não é o longo prazo da fórmula. Nos dias de surto a espera fica em 28,2 minutos, não nos 55 que $W(11)$ prevê,
porque a fila começa vazia e tem só 10 horas para crescer. Segundo, cerca de 70% dos pacientes chegam nos dias de
surto, e por isso a média *por paciente* pesa mais os dias ruins. Terceiro, a fórmula não está errada; ela responde
à espera de longo prazo com demanda constante quando o gestor perguntou quantos pacientes esperam demais nos dias
ruins. É um erro de tipo III. A resposta é certa para a pergunta errada (Capítulo 7).

**Jensen na fila.** A espera é convexa na demanda, e a desigualdade de Jensen (Capítulo 5) diz o que acontece com
qualquer função convexa:

$$\mathbb{E}\big[W(\lambda)\big] \;\ge\; W\big(\mathbb{E}[\lambda]\big), \qquad
\mathbb{E}[W(\lambda)] - W(\mathbb{E}[\lambda]) \approx \tfrac{1}{2} W''(\mu)\,\operatorname{Var}(\lambda).$$

O lado esquerdo é a espera que quem chega realmente sente; o lado direito é a espera do dia médio. Quanto maior a
oscilação da demanda, maior a conta, porque o ganho e a perda não são simétricos. Cinco mundos com a mesma média 8
dão esperas muito diferentes:

| Mundo (pacientes por hora) | Espera média | Em cada 100 pacientes, mais de 30 min |
| --- | --- | --- |
| 8 todo dia (o da alegação) | 9,3 min | 8 |
| 7 ou 9 | 10,5 min | 10 |
| uniforme entre 5 e 11 | 12,3 min | 13 |
| 5 ou 11 | 20,7 min | 25 |
| 5 ou 11, com reforço só nos dias de surto | 2,5 min | menos de 1 |

**A unidade independente decide a margem.** O tamanho efetivo de uma amostra é uma propriedade do processo, não da
contagem (Capítulo 6). Chamadas de um mesmo dia compartilham a taxa e a fila; tratá-las como observações
independentes é **pseudorreplicação**, e a margem sai pequena demais. Os 2,65 milhões de chamadas do cofre valem,
em informação independente, o que valem 81 dias. O modelo nulo de metades arbitra a questão sem convenção. Ele
reparte ao acaso os dias de um mundo conhecido em duas metades, mede a diferença entre elas e diz qual das duas
margens previstas descreve o que se viu. A margem por dia descreve; os dados refutam a margem por paciente.

**O teste severo (Mayo).** Uma diferença entre dois mundos só é evidência se o procedimento teria acusado a
diferença caso ela não existisse. A diferença entre o mundo de 8 e o mundo de 5 ou 11 é de 11,4 minutos, e fica
a 53 desvios do nulo; em 2 000 repartições ao acaso, o nulo nunca passou de 0,81 minuto. É a definição de um
resultado que podia ter dado errado.

**O que ficou suposto no caso real, e como se mediu o tamanho da suposição.** Nem o tempo de atendimento nem a
paciência dos clientes estão na base; o arquivo traz contagens de chamadas, nada mais. O atendimento foi suposto
exponencial com média de 4 minutos, e a paciência exponencial com média de 3. A auditoria transformou as duas
suposições numa malha declarada de 4 × 6 pontos (atendimento de 3, 4, 5 e 6 minutos contra paciência de 1, 2, 3,
5, 7 e 10 minutos) e reescalou as quatro regras ponto a ponto. Por isso a regra do perfil do dia da semana com
margem vence em 20 dos 24 pontos, e nos outros quatro (todos com paciência de 1 minuto) as quatro regras empatam
sem que nenhuma deixe mais de 1 dia abaixo da meta. **Não há ponto de virada.** O canto da malha é o pior caso,
com 5 minutos de atendimento e 10 de paciência. Nem a melhor das quatro regras segura a meta, com 14,2% dos dias abaixo
dela contra 37,7% da regra da alegação. Em cada 100 dias, 38 falham com a escala da alegação e 14 com a regra
certa, que custa 7,2% a mais de agente-horas. A paciência governa o tamanho do efeito; o tempo de atendimento
quase não mexe (com a paciência fixada, a fração da regra vencedora anda no máximo 0,9 ponto percentual em toda a
faixa). O que a malha **não** mede é a família das distribuições (as duas continuam exponenciais, com coeficiente
de variação 1) e a dependência entre paciência e atendimento. E o domínio de saúde do exemplo original ficou de
fora, porque os portais públicos de dados estavam fora do alcance da rede.

**A reprodução.** O ciclo inteiro passou pelos invariantes vigentes naquele momento e reproduziu os seis números
que o material do curso publicava, com contagens idênticas de pacientes nos dois mundos (161 867 e 160 220) e o
mesmo hash nas duas execuções, e isso faz de cada número acima uma medida conferível, e não uma narrativa.

---

### Degrau 3 — Mão na massa

O laboratório reconstrói o Lab 0 em Rust, com semente fixa e sem dependências: SplitMix64 embutido, um dia de
posto com chegadas de Poisson espalhadas no dia, atendimento exponencial e a recursão de Lindley para a espera,
que é a regra "espero o que sobrou da fila, ou nada, o que for maior". O código está em `crates/stochology-book/src/cap16.rs`;
a saída abaixo foi medida com `cargo run --release` no crate de rascunho `~/tmp/labs-cap16`.

```text
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
```

Leia linha por linha. **9,1 contra 19,3.** A demanda média é a mesma, 8 pacientes por hora, e a espera mais que
dobra só porque o mundo oscila. **8% contra 24%** e **8,1 contra 55,8** no percentil 90. A média separa os dois
mundos, e a cauda separa muito mais. **Reforço no surto é 2,5 min e 0%**, e a folga onde está o risco zera a cauda.
**0,2062, 0,1928 e 0,0473.** O desvio medido no nulo de metades é cinco vezes maior que a margem da unidade
"paciente". Quem medisse a espera tratando cada paciente como independente declararia uma precisão que o mundo não
tem. **10,2 min, 49,4 desvios, 0,0000.** A diferença entre os mundos é grande demais para vir do acaso, e o
procedimento podia ter dado errado e não deu.

**Declaração de divergência.** A edição anterior deste livro e o `h0-lab0/README.md` registram 9,3 min e 20,7 min
para os dois mundos, 8 e 25 pacientes em cada 100 acima de 30 minutos, medianas de 3,8 e 9,3, percentis 90 de 26,7
e 59,3, e, no nulo de metades, desvio medido de 0,216 contra 0,227 previsto pelo dia e 0,047 pela chamada (teste
severo: 11,4 min, 53 desvios, teto de 0,81 em 2 000 repartições). Este instrumento mede **9,1 e 19,3**, **8 e 24**
em cada 100, medianas de **3,7 e 8,1**, percentis 90 de **26,1 e 55,8**, e, no nulo, **0,2062** medido contra
**0,1928** pelo dia e **0,0473** pela chamada (teste severo: **10,2 min**, **49,4** desvios, teto de **0,664**). A
margem por paciente coincide até a terceira casa, porque é uma conta de desvio padrão sobre o número de pacientes.

**A base real também roda em Rust.** O laboratório acima é o mundo simulado; a central do banco é medida pelo
crate `labs`, lendo `dados-reais/bank_calls.csv` com `cargo run --release --bin labs -- --banco`. Saída medida.

```text
== H0 real :: a central de chamadas do banco, em Rust ==
treino 83 dias (mar-jun) | cofre 81 dias (jul-out) | 2649354 chamadas no cofre
escala do perfil medio: 234 agentes no pico, 2259 agente-horas por dia
nivel de servico no perfil medio = 0.8193 | media dos dias do cofre = 0.5683 | dias abaixo da meta de 80% = 0.6543
dispersao entre dias: Var/media = 246.7 no cofre, 272.6 no treino, 261.1 em todos | desvio entre dias = 2841 | Poisson preveria 181
nulo de metades: por dia = 0.0441 | por chamada = 0.0006
```

A **fila com desistência** e as quatro regras de escala (R0 a R3) rodam em Rust também, em
`crates/stochology-book/src/fila.rs`, com atendimento exponencial de 240 s, paciência exponencial de 180 s, alvo
de 20 s, meta de 80% e semente 2003. Saída medida com `cargo run --release --bin labs -- --fila`.

```text
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
```

Quatro leituras, com as duas divergências declaradas. **(1)** Os agente-horas batem com o registro:
2 264, 2 261, 2 428 e 2 433 por dia nas quatro regras. **(2)** A ordem é a do registro: R2 é a melhor, R3 perde
para R2 gastando o mesmo, e a alegação (R0) é a pior. **(3)** Os níveis de serviço saem um pouco abaixo do
publicado (83,9% contra 86% em R0, 91,6% contra 96% em R2) e as falhas por dia um pouco acima: 30,9% contra
26% em R0, 19,8% contra 12% em R1, 4,9% contra 3,7% em R2, 9,9% contra 7,4% em R3. **(4)** A verificação
independente, com chegadas de Poisson evento a evento, chega a R2 com 95,0% e 4,9% de dias falhos, encostando no
publicado. O registro mediu com outro motor de simulação e outra semente; a diferença fica declarada, e o que
importa para a decisão, R2 ganha com o mesmo custo, se sustenta nos dois motores. O instrumento foi
reconstruído a partir do código aposentado, que continua no histórico (`git show HEAD:h0-real/estocologia/central.py`),
com duas verificações extras provando o motor: a espera média sem desistência de R0 sai em **1 040 s** contra os
956 s publicados, e o mundo da alegação dá **96,3%** e zero dia falho em 81, contra 97% e zero. Uma diferença de
grade fica registrada. Este instrumento inclui o intervalo das 21:00, e por isso conta 2 654 950 chamadas no
cofre contra as 2 649 354 do instrumento de Erlang C, que corta essa linha.

O instrumento da Tabela de Erlang C reproduz em Rust os números centrais do $H_0$ real. O perfil médio fica em
**82%** (0,8193), a escala da alegação encontra o dia real em **57%** (0,5683) e **65%** dos dias ficam abaixo da
meta (0,6543); o desvio entre dias de
**2 841** contra os **181** que Poisson previria; e a `Var/média` de **272,6 no treino**, que é o 273 publicado.
No cofre ela é 246,7 e na série toda 261,1, e o motivo é o recorte da janela. O nulo de metades por dia sai em
0,0441 contra os 0,0474 publicados (esquema de repartição diferente), e o nulo por chamada desta conta é 0,0006
pela binomial, contra os 0,0033 do roteiro, que usavam outro desenho de correlação intradiária. As duas
divergências ficam declaradas, e o que segue vindo do registro antigo é a tabela de decisão com os esquemas R0 a
R3, que é simulação de fila com desistência e não tem instrumento em Rust ainda.
Os outros valores diferem porque o gerador de números é outro (SplitMix64 aqui, PCG64 no material original) e
porque a espera média por paciente no mundo oscilante é razão de somas de cauda pesada. Rodando o instrumento
com seis sementes diferentes, a espera do mundo 5 ou 11 vai de **19,3 a 20,6 minutos**. A diferença
está dentro da variação do instrumento, e os dois conjuntos de números ficam registrados.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 16 | perda de resolucao 46.24 min (demanda de 5 a 11) | o capítulo traz a tabela R0 a R3 do posto e do banco
```

**Exercícios.**

1. *(Conta)* Use $W(\lambda)=\lambda/(\mu(\mu-\lambda))$ com $\mu = 12$ para achar a espera de longo prazo com
   $\lambda = 8$, $\lambda = 5$ e $\lambda = 11$; depois calcule a média das duas últimas e compare com os 20,7
   minutos da simulação. Por que a fórmula dá mais?
2. *(Conceito)* Por que a média por paciente pesa mais os dias de surto? Calcule, em cada 100 pacientes, quantos
   chegam em dias de 11 e quantos em dias de 5.
3. *(Laboratório)* Rode `crates/stochology-book/src/cap16.rs` com a semente 1909 e confirme a espera do mundo 5 ou 11 e a
   margem por dia. Depois troque o reforço do surto de 24 para 18 atendimentos por hora: a espera média melhora?
   A cauda muda de tamanho?
4. *(Conta)* Em cada 100 pacientes do mundo da alegação, 8 esperam mais de 30 minutos; no mundo real, 25. Quantos
   pacientes a mais são, se o posto atende 8 000 pacientes por ano?
5. *(Conceito)* Escreva, em quatro frases e em frequências naturais, o memorando do H0 real para a diretoria do
   banco, com uma cláusula de saída (que fato novo faria a decisão ser revista).
6. *(Conceito)* Explique por que "a fórmula de Erlang C não está errada, mas a resposta é de tipo III". Que pergunta
   o gestor fez e que pergunta a fórmula respondeu?

**Resumo.**

- A mesma demanda média pode dar 9 ou 20 minutos de espera: a espera é convexa na demanda, e quem olha só a média
  escolhe um mundo que não existe.
- A mediana engana. No mundo que oscila, o paciente típico espera justo o número que a alegação prometia como
  média, e a diferença inteira fica na cauda.
- A margem se calcula na unidade independente, que é o dia, e não o paciente; o modelo nulo de metades arbitra a
  escolha e refuta a pseudorreplicação.
- Em 2,65 milhões de chamadas reais, a escala padrão do setor falha em 26 de cada 100 dias, e gastar cerca de 7% a
  mais de agente-horas onde está o risco reduz isso a menos de 4.
