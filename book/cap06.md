## Capítulo 6. O filtro invisível: por que 2,4 milhões de votos valem 7 pessoas?

**Objetivos.** Ao fim deste capítulo você deve saber separar os três fatores do erro de uma amostra
(qualidade, quantidade e dificuldade); nomear o mecanismo que produz um dado ausente; calcular o
tamanho efetivo de uma amostra na unidade de fato independente; e reconhecer o filtro de seleção que
fabrica relações que não existem no mundo.

---

### Degrau 1 — O mundo real e o paradoxo

**A revista que ouviu 2,4 milhões de pessoas e errou o presidente.** Em 1936, a revista norte-americana
*Literary Digest* fez a maior pesquisa eleitoral da história até então. Recebeu **2,38 milhões** de
cédulas de leitores e anunciou, com estardalhaço, que Alfred Landon venceria Franklin Roosevelt com
**57%** dos votos. Roosevelt ganhou com **60,8%**. Naquela eleição, George Gallup acertou o vencedor
com cerca de **50 mil** entrevistas.

Em cada 100 cédulas que chegaram à redação, 57 diziam Landon. Em cada 100 votantes de verdade, 61
diziam Roosevelt. A revista não errou a aritmética, errou a **porta**. As cédulas não chegaram de um
sorteio do eleitorado, chegaram de quem tinha telefone, de quem era assinante, de quem estava
disposto a preencher e devolver um formulário. Cada uma dessas etapas é uma tela, com efeito
diferente para eleitores de Roosevelt e de Landon.

**Em cada 100 pessoas.** Vale a pena ver a conta em frequências naturais. Imagine 100 eleitores. Se a
chance de responder à pesquisa fosse igual para todos, os 100 responderiam na proporção do voto de cada
um. Não foi isso que aconteceu. Rodando o mundo de 1936 com 10 milhões de eleitores e uma chance de
responder que depende do voto, este livro mede assim:

- dos apoiadores de Roosevelt, **16,8 em cada 100** responderam;
- dos apoiadores de Landon, **35,2 em cada 100** responderam;
- no conjunto, 24 em cada 100 responderam — 2,4 milhões de cédulas.

O resultado é aritmética simples, 0,608 × 0,168 = 10,2% de Roosevelt contra 0,392 × 0,352 = 13,8% de
Landon entre os respondentes. Roosevelt fica com 10,2 de 24, isto é, **42,6%**, e Landon, com 57,4%.
A pesquisa recebeu milhões de votos e mediu a audiência da revista; não mediu o país.

**A pergunta que dá nome ao capítulo.** Quantas pessoas *sorteadas na rua* equivaleriam àqueles
2,4 milhões de cédulas? A resposta medida é **7**, sete pessoas. O desvio de uma amostra aleatória de 7
pessoas num mundo com 60,8% de Roosevelt é de 18,4 pontos, praticamente do tamanho do erro da
pesquisa.

Mais dados viciados não dão erro menor. Eles dão mais certeza sobre a resposta errada. A margem de erro que
a revista podia calcular era de 0,03 ponto, e o erro foi de 18,2 pontos, ou **661 margens**.

**A unidade independente.** Esse veneno aparece sem nenhuma má-fé, em dados "internos". Uma central
de chamadas registra 2,65 milhões de chamadas em 81 dias úteis. Um analista trata cada chamada como uma
observação e calcula a margem. As chamadas de um dia, porém, compartilham a demanda, a escala de
atendentes e o clima. Elas se repetem, não se somam. A unidade independente ali é o **dia**, e 2,65
milhões de chamadas equivalem a **81** observações. É por isso que um relatório que trate as
chamadas como independentes promete uma precisão que a unidade real não sustenta.

---

### Degrau 2 — A regra do jogo

**A identidade de Meng.** Meng (2018) mostrou que o erro de qualquer média amostral se decompõe em
três fatores, cujo produto é exato:

$$\bar Y_n - \bar Y_N \;=\; \underbrace{\rho_{R,Y}}_{\text{qualidade}}\times
\underbrace{\sqrt{\tfrac{1-f}{f}}}_{\text{quantidade}}\times \underbrace{\sigma_Y}_{\text{dificuldade}}$$

| Fator | O que é | O que ele faz |
| --- | --- | --- |
| Qualidade $\rho_{R,Y}$ | Correlação entre **entrar na amostra** e **a resposta** | É o veneno: não encolhe com $n$ |
| Quantidade $\sqrt{(1-f)/f}$ | $f$ é a fração da população amostrada | Encolhe com mais dados, mas devagar |
| Dificuldade $\sigma_Y$ | O quanto a resposta varia na população | É a física do problema |

Numa amostra **sorteada**, $\rho$ é da ordem de $1/\sqrt N$ — minúsculo — e a identidade devolve o
erro-padrão de sempre. Quando a entrada na amostra depende do que se mede, $\rho$ **não** encolhe com
$n$. O primeiro fator fica parado enquanto o segundo diminui. Com $\rho = -0,21$, $f = 0,24$ e
$\sigma_Y = 0,488$, a identidade prevê um erro de $-0,1824$, ou 18,2 pontos, o valor que a simulação
mede. Mais dados enviesados não consertam nada; eles **estreitam o intervalo em volta da resposta
errada**.

**Como tratar um dado ausente.** Rubin (1976) classificou a ausência
pelo mecanismo que a produz, e a classe decide o tratamento legítimo.

| Mecanismo | A chance de faltar depende de | Exemplo | O que se pode fazer |
| --- | --- | --- | --- |
| MCAR, completamente ao acaso | Nada que importe | Um formulário perdido no correio | Analisar os casos completos perde precisão — nenhum viés entra |
| MAR, ao acaso dado o observado | Variáveis que foram medidas | Jovens respondem menos, e a idade está na base | Ponderar ou imputar condicionando no observado |
| MNAR, não ao acaso | O próprio valor que falta | Quem ganha mais declara menos a renda | Nenhuma correção sai só dos dados: exige um modelo da ausência e análise de sensibilidade |

A distinção entre MAR e MNAR **não pode ser testada** com os dados observados. Trata-se de uma premissa,
e ela precisa ser declarada como tal. Na identidade de Meng, MNAR é o caso em que $\rho$ continua
diferente de zero depois de condicionar no que foi observado. O valor sentinela, o ausente gravado como
0 ou 9999, é pior que os outros três, porque ninguém sabe que o dado falta.

**O filtro fabrica relações.** Berkson (1946) mostrou que duas doenças independentes aparecem
associadas entre internados, porque cada uma aumenta a chance de internação. Olhar só os selecionados é
condicionar num **colisor**. Na simulação deste capítulo, talento e simpatia têm correlação $+0,001$ na
população e $-0,679$ entre os selecionados. A relação não estava escondida nos dados; o filtro a
**criou**.

**A tabela que inverte.** Simpson mostrou o caso espelhado. Charig e colegas (1986) compararam duas
cirurgias de cálculo renal.

| Grupo | Cirurgia aberta | Outra técnica |
| --- | --- | --- |
| Cálculos pequenos | 93% | 87% |
| Cálculos grandes | 73% | 69% |
| Total bruto | 78% | 83% |
| Ponderado pela população inteira | 83,3% | 77,9% |

A cirurgia aberta vence nos dois estratos e perde no total, porque os casos difíceis iam para ela.
Condicionar num confundidor fecha um caminho espúrio; condicionar num colisor abre um. "Controlar por
tudo" não é neutro, e o grafo causal decide qual conta fazer, e não a tabela (Capítulo 9).

**A unidade independente, formalizada.** O tamanho efetivo de uma amostra é uma propriedade do
**processo**, e não da contagem. Em 1913, Markov contou vogais e consoantes nas 20 000 primeiras letras
do *Ievguêni Oniéguin* e mostrou que a lei dos grandes números sobrevive à dependência, mas que a
dependência muda quanto a amostra vale. Como as letras tendem a alternar, um bloco de 100 letras vale
cerca de 330 letras independentes; numa cadeia "pegajosa", com cada estado tendendo a se repetir, 100
observações valem bem menos que 100.

$$n_{\text{ef}} \;=\; \frac{p(1-p)}{\operatorname{Var}(\bar X)}$$

Em palavras, compare a dispersão que a média amostral **realmente tem** com a que ela teria se as
observações fossem independentes. Em dados reais, 225 mil voos de Nova York valem, em informação
independente, a 364 dias; 2,65 milhões de chamadas equivalem a 81 dias. Tratar os
pacientes de um dia, ou as chamadas de uma hora, como observações independentes é
**pseudorreplicação**: a margem sai pequena demais e o teste aprova aquilo que não deveria.

**O caso da central de chamadas, inteiro.** O setor alegava o seguinte. Escalar cada meia hora pelo
Erlang C sobre a taxa média do histórico garantiria 82% das chamadas atendidas em até 20 segundos, todo
dia. Nos 81 dias do cofre, a regra padrão falha em **26 de cada 100 dias** e entrega 86%. A variação
entre dias responde pela maior parte, pois até o mundo que mantém só o total real de cada dia, com o
perfil médio, já perde 17% dos dias. Erlang C, aplicado à taxa real de cada dia, dá **57%**, no lugar dos 82% da alegação, e 65% dos dias ficam abaixo da meta; é Jensen dentro da fórmula (Capítulo 5), sem
precisar de simulação. A unidade sai do nulo de metades. São desvios de 0,0474, contra 0,0470 previsto
pelo dia e 0,0033 previsto pela chamada. A leitura por chamada promete uma margem catorze vezes menor.
Em 400 dias simulados no mundo da alegação, nenhum ficou abaixo da meta; no real, cerca de um em quatro.
A decisão que sai disso custa **+7,2% de agente-horas**, 2 428 por dia contra 2 264, e derruba a falha
de 26 para **menos de 4 em cada 100 dias**. A malha de sensibilidade, declarada antes de rodar em
`h0-real/DECLARACAO_PACIENCIA.md`, varre atendimento de 3 a 6 minutos contra paciência de 1 a 10
minutos, 24 pontos. A regra vencedora acerta em 20 deles; nos quatro restantes, todos com 1 minuto de
paciência, as quatro regras empatam porque nenhuma deixa mais de 1 dia abaixo da meta. Não há ponto de
virada. No pior canto da malha, nem a melhor regra segura a meta. São 14,2% dos dias abaixo dela, contra
37,7% da regra da alegação.

**Como isso se confere.** Nada disso é uma questão de opinião sobre quem está certo. Os rastros da
pesquisa (diários, hipergrafos, resultados) ficam fora do Git por tamanho, com as somas SHA-256
publicadas em `FORA_DO_GIT.md` e no `SHA256SUMS` de cada frente; quem quiser refazer a conta baixa o
dado bruto, regenera o rastro e confere byte a byte. O filtro é invisível no relatório e no arquivo.

---

### Degrau 3 — Mão na massa

O laboratório reconstrói a pesquisa de 1936, mede a correlação que o filtro cria, refaz a tabela de
Simpson e mede o tamanho efetivo de uma cadeia com memória. Código em `crates/stochology-book/src/cap06.rs`, saída
medida com `cargo run --release`:

```text
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
```

Quatro leituras. **(1)** O erro medido (−18,2 pontos) coincide com a identidade de Meng (−18,24) e vale
o desvio-padrão de uma amostra de **7,2 pessoas**. A simulação não é uma história, é uma conta
fechada. **(2)** A margem de erro da pesquisa (0,0276 ponto) não é margem nenhuma contra um veneno de
18 pontos; são 661 margens de distância. **(3)** A correlação entre talento e simpatia sai de $+0,001$
na população para **−0,679** entre os selecionados com o corte 1,5. O filtro **fabrica** a associação
negativa, e ela fica mais forte quanto mais estreita é a porta (−0,617 com 24% selecionados, −0,732 com
7,9%). **(4)** O tamanho efetivo segue o processo. São 100 letras que valem 101 observações quando não há
memória, 25 quando a cadeia é pegajosa (persistência 0,8) e 327 quando as letras alternam (persistência
0,235), o número que Markov encontrou no *Oniéguin*.

**Declaração de divergência.** A edição anterior deste livro registra, para a mesma simulação de Berkson,
uma correlação de $+0,002$ na população e $−0,669$ entre os selecionados; este instrumento mede
$+0,0012$ e $−0,679$ com o corte 1,5 (e $−0,617$ com o corte 1,0). A diferença é de um centésimo e vem
do corte de seleção, que o texto antigo não declara. Os números de 1936 convergem sem ajuste. São 2,4
milhões de cédulas, 57,4% para Landon contra os 57% publicados, e a equivalência de 7 pessoas. Nenhum
valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  6 | sem perda medida neste capitulo | a unidade independente é o Φ2; o eixo R é o primo dele no grão da pergunta
```

**Exercícios.**

1. *(Conceito)* Uma empresa diz: "temos milhões de usuários, então nossa pesquisa de satisfação é
   precisa". Que fator da identidade de Meng ela esqueceu, e o que ele tem a ver com o *Literary
   Digest*?
2. *(Conta)* Na tabela de Simpson, recalcule a taxa bruta de cada cirurgia como média dos estratos
   ponderada pelo número de casos (81, 234, 192 e 55) e explique por que ela inverte os estratos. Qual é
   a variável confundidora?
3. *(Conta)* Com $\rho = -0,21$ e $\sigma_Y = 0,488$, qual $f$ faz o erro de Meng valer 10 pontos?
   E 5 pontos? Diga em uma frase por que a pesquisa de 1936 não conseguiria chegar lá nem com 50
   milhões de cédulas.
4. *(Laboratório)* No laboratório, mude 0,168 e 0,3517 para um par em que a chance de responder seja
   igual nos dois grupos e confirme que o erro desaparece. Depois escolha um par com correlação de
   $−0,05$ e diga quantas pessoas valeria a pesquisa inteira.
5. *(Laboratório)* Rode a cadeia de dois estados com persistência 0,9 e com 0,1 e meça o tamanho
   efetivo nos dois casos. Explique, em uma frase para cada caso, o que isso faz com a margem de um
   relatório que trate as 100 letras como independentes.
6. *(Conceito)* Dê um exemplo do seu campo em que o dado ausente seja MNAR e um em que seja MAR, e
   diga o que muda na conclusão de cada um.

**Resumo.**

- O erro de uma média é qualidade × quantidade × dificuldade, e tamanho não compensa seleção.
- MCAR perde precisão, MAR se corrige condicionando no observado, MNAR só se corrige com um modelo da
  ausência declarado como premissa.
- O filtro não só distorce a média: ele cria relações que não existem na população.
- O tamanho efetivo de uma amostra é uma propriedade do processo; a margem se calcula sempre na unidade
  independente.
- 2,4 milhões de cédulas viciadas valem 7 pessoas, e a margem de erro de uma pesquisa viciada mede
  coisa nenhuma.
