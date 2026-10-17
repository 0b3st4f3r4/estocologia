## Capítulo 14. O contrato de ferro: fichas, proibições, invariantes e portões

**Objetivos.** Ao fim deste capítulo você deve saber ler e escrever uma ficha epistemológica, preencher as
cinco perguntas do cabeçalho, enunciar as quatro proibições e os cinco invariantes $\Phi_1$–$\Phi_5$,
percorrer o funil de quatro filtros e dizer qual verificação cada portão automático faz.

---

### Degrau 1 — O mundo real e o paradoxo

**O formulário que recusa.** Todo mundo já preencheu um formulário que aceita qualquer coisa. Você escreve
"sim" no campo de observações, o sistema aceita, e três semanas depois ninguém sabe quem autorizou o quê.
O oposto existe e é chato, e é disso que este capítulo trata. O formulário aqui **recusa** o envio quando um
campo obrigatório está vazio.

A ficha epistemológica é uma lista fechada de perguntas. Ela é o **contrato** do trabalho, e não o resumo
dele. Em cada 100 fichas preenchidas com pressa, as que passam sem reclamação são as que tiveram todos
os campos respondidos com algo conferível. Uma ficha sem o campo do processo gerador não diz "não sei". Diz
"ninguém escreveu isso", e é por aí que o erro entra.

**A pergunta que ninguém faz ao decisor.** Há um jeito específico de errar que tem nome e quase nunca é
checado, o **erro de tipo III**. Nos testes comuns, o erro de tipo I é acusar quem é inocente e o erro de
tipo II é absolver quem é culpado. O erro de tipo III é mais banal e mais frequente, e consiste em **responder
brilhantemente a pergunta errada**. O estudo mede o efeito A com precisão impecável, e o gestor precisava
saber sobre o efeito B.

Imagine uma prefeitura que precisa decidir se abre uma segunda sala de atendimento nos dias cheios. O
relatório responde, com rigor, à pergunta *"qual é o nível de serviço médio do ano?"* A resposta é 82%, e o
gestor lê isso como resposta à pergunta dele, *"devo abrir uma segunda sala?"*. Ninguém escreveu a pergunta
do gestor em lugar nenhum. Em 100 decisões apressadas, uma parte respeitável morre por conta certa na
pergunta trocada, e não por conta errada.

**Como o contrato trava isso.** O pacote de domínio passa a carregar um campo obrigatório, o
`pergunta_do_decisor`, com três partes na ordem: **quem decide**, **qual ação a decisão muda**
(copiada literalmente do campo de decisão do pacote) e **a pergunta que essa decisão usa**,
terminada em interrogação. Quem valida é o código, que recusa texto genérico sem quem decide e barra uma ação
diferente da do pacote. Depois disso, "a pergunta que o modelo responde é a que o decisor fez"
deixou de ser suposição e passou a ser uma frase conferível.

**As quatro coisas que o contrato proíbe.** O contrato dos agentes começa por quatro proibições absolutas, e
cada uma tapa um buraco conhecido:

1. **Linguagem natural livre entre agentes.** A comunicação é só por fichas estruturadas; texto livre é
   ambíguo e inventa.
2. **A média solitária.** Quando a função é não linear, a dinâmica multiplicativa ou a cauda pesada, é
   proibido reportar só a média: reportam-se o resultado do caso médio, a média simulada dos
   resultados, a mediana, os quantis e a curva acumulada.
3. **Confundir aprendizado com performatividade.** Erro de previsão pequeno não prova que o modelo é
   verdadeiro; roda-se o modelo com e sem a ação.
4. **Extremos dogmáticos e omissão de tentativas.** Nenhuma previsão declara 0% ou 100%; toda previsão é
   pontuada por regra própria fora do ajuste; e declara-se quantas especificações foram tentadas.

---

### Degrau 2 — A regra do jogo

**A ficha e as cinco perguntas do cabeçalho.** Todo payload que circula abre com cinco perguntas
obrigatórias.

| Campo | Pergunta |
| --- | --- |
| `processo_gerador` | Qual mecanismo oculto gera os dados, escrito de modo simulável, e em que camada a alegação pode falhar? |
| `nao_linearidade` | O resultado é função não linear de algo que varia? Compare $f(\mathbb{E}[X])$ com $\mathbb{E}[f(X)]$; a dinâmica é aditiva ou multiplicativa? |
| `unidade_e_margem` | Qual é a unidade de fato independente, e a margem foi calculada com ela? |
| `fisica_do_ruido` | Que ruído é esse (gaussiano ou com saltos)? Há estados absorventes? O agente é passivo ou deforma o mundo? |
| `escada_causal` | Em que degrau de Pearl está a afirmação, com que estratégia de identificação e que premissa não testável? |

O cabeçalho real do $H_0$ em domínio real, abreviado:

```yaml
alegacao: >
  "Escalando cada meia hora pelo Erlang C sobre a taxa média do histórico, a central atende
  82% das chamadas em até 20 s, com espera média de 10 s, todo dia."
cabecalho:
  nao_linearidade: >
    O nível de serviço do Erlang C é côncavo e despenca perto da saturação: aplicada à taxa
    real de cada dia do cofre, a própria fórmula da alegação dá em média 57%, não 82%.
  unidade_e_margem: >
    A unidade independente é o DIA: as chamadas de um dia compartilham a taxa e a fila.
    n_efetivo = 81 dias de cofre, não 2,65 milhões de chamadas.
  fisica_do_ruido: >
    A demanda diária NÃO é Poisson: dispersão Var/média = 273 (Poisson: 1).
  escada_causal: >
    Degrau 2 (intervenção): as regras de escala são do(escala) simuladas sobre a demanda real.
    Premissa não testável: a demanda não reage à escala.
```

Cada módulo do curso acrescenta perguntas às cinco do cabeçalho. A ficha da evidência pede a taxa-base, as
duas verossimilhanças e a posterior calculada de dois modos, em chances e numa árvore de 10 000 casos, que
precisam coincidir. A do grafo causal começa pelo degrau. A da crítica de modelos fecha a série e diz em que
item cada ficha anterior entra.

**A física do ruído, em detalhe.** O quarto campo pede três classificações do ruído.

| Pergunta | Opções | Por que importa |
| --- | --- | --- |
| Que forma tem o ruído? | Gaussiano (contínuo) ou com saltos | Um modelo gaussiano não tem o 19 de outubro de 1987 no seu espaço de hipóteses |
| Como o ruído se integra? | Itô ou Stratonovich | Quando o ruído depende do estado, as duas leituras dão derivas diferentes |
| Há estados absorventes? | Sim (ruína, morte, falência) ou não | Num estado absorvente, a média de conjunto deixa de descrever qualquer trajetória |

A segunda pergunta parece técnica e não é. Num crescimento multiplicativo com ruído proporcional,
$dX = \mu X\,dt + \sigma X\,dW$, lido à maneira de Itô, a esperança cresce à taxa $\mu$, mas a
trajetória típica cresce à taxa

$$\frac{d}{dt}\,\mathbb{E}[\ln X_t] = \mu - \tfrac{1}{2}\sigma^2.$$

É o arrasto da volatilidade do Dow Jones alavancado, e é Jensen outra vez. Lido à maneira de Stratonovich, o
mesmo símbolo descreve outro processo, em que o termo $-\sigma^2/2$ não aparece. A escolha depende da física.
Ruído branco idealizado de um sistema discreto pede Itô; o limite de um ruído real com correlação curta pede
Stratonovich. A ficha pede também um teste com passo de tempo menor. Se o resultado muda quando $\Delta t$
encolhe, a simulação estava medindo o integrador em vez do Mundo. Esse teste **não** decide entre Itô e
Stratonovich, porque o esquema de Euler converge para a leitura de Itô, e a de Stratonovich pede outro integrador.

O ruído também faz saltar entre estados estáveis. Num sistema com dois poços, o tempo médio até o salto
segue a fórmula de Kramers e cresce exponencialmente com a altura da barreira $\Delta U$ em relação à
temperatura $T$:

$$\tau \approx \frac{2\pi}{\sqrt{U''(x_{\min})\,\lvert U''(x_{\max})\rvert}}\; e^{\Delta U/T}.$$

Para barreiras altas, esse tempo quase não tem memória. Quem esperou muito não está mais perto do salto.
Olhado só nos saltos, porém, o ruído parece um empurrão. Em 591 saltos simulados no curso, o ruído logo
antes do salto apontou no sentido dele em 99% dos casos, sem nenhuma força externa. Procurar a causa de um
salto é erro de seleção; procurar a causa de uma mudança na frequência dos saltos não é.

**Os cinco invariantes.** Nada entra no grafo sem passar pelos invariantes, que funcionam como travas de banco
de dados. Os quatro primeiros são exigidos desde o começo; o quinto entrou com o eixo da resolução da pergunta
(Capítulo 21) e morde no caso em que o desalinhamento é mais caro.

| Invariante | O que exige | O que barra |
| --- | --- | --- |
| $\Phi_1$ Simulabilidade | Toda alegação vira código com semente declarada e testes de propriedades | Argumento só em prosa |
| $\Phi_2$ Margem e unidade | Todo número vem com margem calculada na unidade de fato independente | Pseudorreplicação, duplicatas, valores sentinela |
| $\Phi_3$ Camadas e teste severo | Auditar o filtro da amostra, desenhar o grafo causal e confrontar a hipótese com um modelo nulo simulado | Padrão lido antes do nulo, colisor, Simpson |
| $\Phi_4$ Escopo ecológico | Julgar a decisão no ambiente em que vai operar, com termo de escopo e cláusula objetiva de saída | Recomendação sem prazo de validade |
| $\Phi_5$ Resolução da pergunta | A ação declarada tem de estar no mesmo grão da unidade independente: com o dado no dia ou mais grosso, resolução agregada (R0) reprova | Ação por dia com conta no agregado. Unidade mais fina que o dia fica fora desta trava, e isso está declarado |

**O funil de quatro filtros.** Um sistema que simula toda hipótese que encontra colapsa por custo e por
inflação de falsas descobertas. O funil, a poda por valor da informação e o contador de tentativas vivem em
Rust, em `crates/stochology-core/src/funil.rs`, com 17 testes. Os quatro filtros recusam pelo cabeçalho (sem
pergunta, sem processo gerador, sem unidade ou nulo, sem ação com saída). A poda corta quando a cota do valor
da informação não pode cruzar o limiar: com $C/L = 0{,}1$ a cota vale 0,0090 em $p = 0{,}001$ e o exame de
0,05 é podado, enquanto no limiar $p = 0{,}10$ a cota é 0,9000. Por fim, o contador fecha a conta na forma
`K = 3 tentativas (F1 1, F2 0, F3 0, F4 1); aprovadas 1; versao final: v3`. O limite declarado é que a cota
é o valor esperado da informação perfeita, exata no modelo de dois atos e dois estados. Cada frente se
organiza como um funil, e nele a maior parte das hipóteses fracas cai antes de tocar em simulação pesada.

1. **Valor da informação.** Corta-se a hipótese que não mudaria nenhuma decisão nem com resposta perfeita. O log
   padrão é "podado por Crisonomia: curiosidade sem uso operacional".
2. **Poda topológica e unidade.** Corta-se, por d-separação, todo nó que fica fora do cobertor de Markov da
   variável de interesse, inclusive os colisores que, se condicionados, criariam o viés de Berkson. Corta-se
   também a alegação que os dados não identificam, ou cujo tamanho efetivo, contado na unidade independente
   pelo $F_2$, fica pequeno demais para concluir. O contrato de dados confere ainda um bloco `ausencia` para o
   mecanismo da falta: MCAR, MAR, MNAR ou não declarado, com as colunas que a classificação cobre e a análise
   de sensibilidade exigida quando a ausência pode depender do valor da variável. Ausência sem classificação
   é recusada; nos três contratos do $H_1$ real e nas 18 tabelas do $H_{2g}$ a regra é vacuosa hoje, e passa
   a morder no dia em que aparecer.
3. **Teste severo contra o nulo.** Se o padrão alegado já aparece no modelo nulo, ou numa referência de um
   parâmetro só como a taxa-base, a pesquisa encerra ali. Quando a simulação é cara, o $F_4$ faz esse teste
   primeiro num emulador barato.
4. **Inferência pesada, lentes e banca.** Só o restante recebe o tratamento caro.

**Caso trabalhado: a identificabilidade de $R_0$.** No início de uma epidemia, os casos crescem como
$\exp((\beta - \gamma)t)$, e os dados determinam a **diferença** $\beta - \gamma$, não a razão
$R_0 = \beta/\gamma$. Com 25 dias de casos simulados, em que a verdade é $R_0 = 2$, os valores compatíveis
vão de **1,38 a 5,90**, e o tamanho final previsto vai de **50% a 100%** da população. Com 45 dias, $R_0$
fica entre **2,00 e 2,24**, mas aí a curva já dobrou. Tomar a duração da infecção de outros estudos estreita
$R_0$ para **1,81 a 2,11**. Isso não é defeito, desde que se declare que a margem passou a ser a da fonte.

**Os portões automáticos.** Nenhuma entrega é consolidada sem passar nos portões numa máquina limpa. Os
agentes e o $\Phi$ já foram formalizados; os portões **repetem o trabalho e comparam**. Entrada repetida tem
de produzir saída idêntica, e um defeito plantado tem de ser detectado. O binário
`portoes` implementa sete.

| Portão | Faixa | O que verifica |
| --- | --- | --- |
| G1 | $\mathcal{M}$ | Determinismo: a raiz de Merkle da segunda execução é idêntica à da primeira |
| G2 | $F_1$ | Semente fixa declarada em toda ficha |
| G3 | $F_1$ | Escopo declarado: só artefato simulável |
| G4 | $F_1$/$F_7$ | Invariantes $\Phi_1$–$\Phi_4$ e a trava de tipo III |
| G5 | $F_5$ | Trilha SHA-256 íntegra **e** adulteração detectada |
| G6 | $\mathcal{M}$ | Banca adversarial $A_1$–$A_8$ com o piso |
| G7 | $F_8$ | Cláusula de saída declarada e $A_8$ acima do piso |

Dois detalhes fazem o conjunto valer mais que a soma. O G5 não verifica só que a corrente está íntegra. Ele
**adultera de propósito** e exige que a verificação falhe com a corrente adulterada. O G1, por sua vez,
compara duas execuções do horizonte declarado. Um portão que nunca falha não é portão (Capítulo 15).

---

### Degrau 3 — Mão na massa

O laboratório valida uma ficha completa e uma ficha com um defeito plantado por vez, imprimindo as falhas por
portão, no espírito de `portoes --defeito`. Código em `crates/stochology-book/src/cap14.rs`; saída medida com
`cargo run --release --bin labs -- --cap 14`:

```text
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
```

Quatro leituras.

**A ficha completa passa sem uma única falha.** Cinco campos de cabeçalho na ordem exigida, semente
declarada, artefato simulável, margem com unidade independente, modelo nulo, escopo ecológico, quem paga
pelo erro, cláusula de saída e a pergunta do decisor citando a ação. O total é zero. A trava
de tipo III é **literal**, e a pergunta precisa terminar em interrogação **e** conter a ação do pacote.

**Cada defeito cai no portão certo.** Sete defeitos plantados, sete reprovações, uma por portão, sem
contaminação. Sem semente cai no $\Phi_1$; sem margem ou unidade cai no $\Phi_2$; sem definição ou sem
modelo nulo cai no $\Phi_3$; sem escopo, sem quem paga ou sem cláusula de saída cai no $\Phi_4$. Os dois
defeitos da pergunta do decisor caem os dois na trava de tipo III, mas por motivos diferentes. Um perde o
ponto de interrogação, o outro perde a ação. **A mensagem é idêntica nos dois casos.** O validador
enuncia a exigência da regra, sem dizer qual parte faltou. É um detalhe pequeno que o exercício 4 pede
para melhorar.

**O validador de estrutura atravessa os capítulos.** Reaparecem aqui as três recusas do Capítulo 12:
aresta para nó fantasma, ciclo dedutivo e hiperaresta com um membro só. Isso é de propósito. Uma trava que só
existe num capítulo é uma trava que ninguém usa.

**O teste de propriedades e a semente.** Dez mil fichas simuladas com margem $1{,}96\sqrt{p(1-p)/n}$ para
$n$ entre 100 e 999 e $p$ entre 0,01 e 0,99 dão **zero violações**, com margens entre 0,00617 e 0,09798. O
intervalo de confiança nunca é negativo nem maior que meio. Os dois validadores rodados sobre a semente
declarada divergem em **0 de 32** execuções, a reprodutibilidade que o Capítulo 1 exige.

**A esteira por horizonte.** No repositório cada horizonte tem a sua esteira. O $H_0$ aplica $\Phi_1$–$\Phi_4$, as rodadas $H_{2b}$ a $H_{2g}$ aplicam cinco critérios e o $H_3$ aplica sete. Cada `portoes.py` repete a execução, compara os resumos criptográficos e sai com 0 ou 1; os sete deste capítulo
são os do binário `portoes`. (O `portoes.py` de cada frente foi aposentado em 2026-10-16 e vive no commit
`1bfec905`; o portão continua no binário.)

**Declaração de divergência.** Duas contas deste capítulo divergem entre as fontes, e ficam as duas
registradas. (1) O livro anterior descreve **cinco** portões automáticos — contratos e sementes; esquema
declarativo e idempotência; convergência das cadeias; corte temporal; governança, justiça e saída —, enquanto
o binário `portoes` implementa **sete** (G1 a G7). A diferença é de agrupamento: o portão 3 do livro
antigo ("convergência das cadeias") some do binário, e o portão 5 se desdobra em G5, G6 e G7. Os dois textos
contam a mesma esteira de formas diferentes; nenhuma lista foi apagada. (2) O livro anterior e o plano do
repositório dizem que "no $H_0$ a esteira aplica $\Phi_1$–$\Phi_4$", e este laboratório mede exatamente
isso: uma ficha completa gera zero falhas de invariante, e cada defeito gera uma. O número "falhas = 0" do
portão G4, no horizonte `todos`, reproduz aqui.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 14 | sem perda medida neste capitulo | o contrato carrega o campo resolucao (R0 a R3) e o Φ5; a trava só morde com unidade igual ou mais grossa que o dia
```

**Exercícios.**

1. *(Conceito)* Preencha as cinco perguntas do cabeçalho para a alegação "alunos que estudam com música
   tiram notas maiores".
2. *(Conceito)* Que filtro do funil deve cortar a pergunta "qual é a cor preferida dos clientes que compram
   seguro?" numa seguradora que não vai mudar o produto?
3. *(Conta)* Por que 25 dias de crescimento exponencial identificam $\beta - \gamma$ e não $\beta/\gamma$?
   Mostre que dois pares $(\beta, \gamma)$ com diferenças iguais geram curvas iniciais idênticas.
4. *(Laboratório)* Melhore a mensagem da trava de tipo III para distinguir "faltou o ponto de interrogação"
   de "a pergunta não cita a ação". Faça os dois defeitos caírem em mensagens diferentes e reproduza a
   tabela.
5. *(Laboratório)* Escreva um teste de propriedades que verifique que a espera simulada de uma fila é sempre
   não negativa e não diminui quando o tempo de atendimento aumenta, mantidas as chegadas.
6. *(Conceito)* Explique, em três frases, por que o G5 é mais forte que um teste que só confere se a corrente
   está íntegra, já que ele adultera de propósito a corrente que produziu.

**Resumo.**

- A ficha é uma lista fechada de perguntas; cinco são obrigatórias em todo payload, e o campo
  `pergunta_do_decisor` trava o erro de tipo III.
- As quatro proibições e os cinco invariantes são travas, e não recomendações; o $\Phi_5$ é o mais novo e o mais estreito.
- O funil corta barato antes de gastar caro: valor da informação, poda topológica e de unidade, teste severo
  contra o nulo, e só então inferência pesada e banca.
- O binário `portoes` implementa sete portões, com código de saída 0 ou 1, e um portão que nunca
  falha não é portão.
