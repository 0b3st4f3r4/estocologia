## Capítulo 21. As sete lições, os dez pontos da auditoria e o que falta fazer

**Objetivos.** Ao fim deste capítulo você deve saber resumir as sete lições do programa em situações do dia a dia,
ler o balanço dos dez pontos da auditoria de fundo com o resultado de cada um na mão, nomear em voz alta as
questões que seguem abertas e sem resposta, e conduzir o projeto final. Escolher um problema da escola ou da
cidade, preencher a ficha, simular o mundo e defender a conclusão.

---

### Degrau 1 — O mundo real e o paradoxo

**A pergunta que fecha o curso.** No último encontro, alguém pergunta de que serviu o curso na vida real. Não de
que serviu para quem vai fazer estatística, e sim para quem vai decidir. A resposta do programa tem sete frases, e
cada uma nasceu de um caso concreto. As sete vêm contadas do jeito que aconteceram.

1. **A média esconde o essencial.** No posto de saúde, a demanda média é idêntica nos dois mundos, e a espera
   salta de 9,3 minutos no mundo constante para 20,7 minutos no mundo que oscila. Em cada 100 pacientes, 25 esperam
   mais de meia hora no mundo que oscila, contra 8 no mundo constante. Numa central de chamadas, a escala
   dimensionada pela média falha em **26 de cada 100 dias**. E a riqueza média de quem apostou 3 vezes na bolsa
   aparece como 19 vezes, quando a mediana é de 2,1.
2. **A unidade independente decide a margem.** 2,65 milhões de chamadas valem **81 dias**; 225 mil voos valem
   **364 dias**. Contar linha por linha, onde a variação se repete, separa uma margem honesta de uma margem que só
   parece estreita.
3. **Acertar não basta; é preciso ter podido errar.** O ajuste por propensão no experimento de LaLonde acertou o
   número, e um tratamento inócuo teria chegado a um valor parecido. No placebo de 1975 ele inventou um efeito
   fantasma de $-12{,}3$ pontos percentuais. Um número que acerta sem poder errar não é evidência.
4. **Reprovar o próprio modelo é resultado.** Nos incêndios dinamarqueses, 36 meses sem nenhum incêndio gigante
   valem só **2,2 bits** de informação. A chance mensal continua perto de $1/38$ pela regra de Laplace, enquanto a
   seguradora triplicava a aposta. Quando o modelo do programa foi testado no cofre, ele perdeu para a média
   histórica simples (habilidade $-0{,}015$) e **não ganhou o direito de decidir**. A alegação da seguradora caiu.
5. **O efeito transporta melhor que a lista de causas.** A invariância exata é rara. Quando ela aparece e é
   conferida contra uma verdade conhecida (o túnel de luz e os bloqueadores de hERG), protege contra a intervenção
   nova, como o verapamil mostrou. Quando não aparece, a direção e o tamanho do efeito ainda costumam sobreviver.
   9 domínios previram o ambiente nunca visto com habilidade de 0,04 a 0,88.
6. **Calibração se conserta, resolução não.** Foram necessários três cofres para a calibração passar pela
   tolerância de 0,03, e a auditoria mostrou que essa tolerância era frouxa nos cofres grandes. A habilidade final
   ficou em $+0{,}019$. Dizer "está bem calibrado" não assegura que o modelo separe bem os casos.
7. **Declarar antes de rodar torna a falha útil.** Sete rodadas no H2 e três tentativas no H3 só contam como
   ciência porque cada mudança foi escrita com hash **antes** de rodar, e porque cada ajuste feito depois de ver um
   resultado ficou registrado e contado no contador $K$.

**O programa virou o alvo.** Uma disciplina que exige teste severo das alegações alheias precisava passar pelo
teste que aplica aos outros. A auditoria de fundo fez isso. Dez suposições do programa, coisas que o texto
afirmava e que nunca tinham sido postas à prova, viraram frentes de trabalho, cada uma submetida à regra de casa:
declaração com hash antes de rodar, resultado relatado com os limites, e toda mudança de critério depois de ver o
resultado entrando como declaração nova e contando no $K$.

O resultado geral é desconfortável e vale a leitura. **Nenhum número publicado caiu por erro de conta**, e a
auditoria encolheu a autorização desses números. A tese central do livro empatou no teste direto. A tolerância de
calibração era frouxa. O laço de Goodhart declarado era quatro vezes maior que o medido. E o portão do H2 depende
de qual versão de cada escolha conta.

**A quem o programa não ouviu.** Em cada uma das sete decisões que envolvem pessoas (cortar o benefício de
acidentados do trabalho no Kentucky, reduzir turmas no Tennessee, mudar a coparticipação no seguro de saúde,
comparar o treino profissional do NSW com controles do CPS, marcar o detido em Toronto nos dois períodos e marcar o
motorista em Rhode Island), **nenhum grupo afetado foi ouvido**. Não há no repositório consulta, entrevista, painel
nem consentimento. A lista é grande e tem nome e número: 11.598 alunos em 80 escolas; 5.226 detidos; 65.276 prisões
de 2020 e 2021; 86.113 paradas de trânsito; 185 participantes do treino do NSW e 15.992 controles. Assinar não é
escutar. Quem assina um parecer fica do lado de quem decide usar o artefato; quem paga o erro fica do outro lado.

**Duas pontas ficaram no papel.** As duas continuam abertas e as duas dependem de gente. A primeira é a elicitação
de especialistas. O protocolo de painel e seis formulários estão escritos, com o critério de $K = 2$ pronto, e
**ninguém foi entrevistado**. Nenhuma previsão foi corrigida, nada foi assinado. Sem esse painel, cinco dos 33
domínios mudam de status ou de ação dependendo da priori escolhida. A segunda é a voz de quem paga o erro. O
registro dos grupos afetados existe e diz que nenhum foi ouvido.

**E uma ponta que era só de resolução.** Na genética de levedura, a validação que sustentava a réplica decidia com
**um mutante por ambiente**: o menor p-valor possível ali era $1/1\,000$, e o resultado saiu em $p = 0{,}001$, que
é o piso. O p não era o problema — faltava linha. O desenho dessa falta hoje tem instrumento: com uma linha por
mutante o teste aprova **4 de 40** (o que se procura, um artefato de linha e dois que são só ruído, todos no piso
de $0{,}001$, sem que o p separe uns dos outros), e com a segunda linha aprova **1 de 40**, com o piso caindo para
$10^{-6}$ e a estimativa deixando de ser um ponto (`labs --genetica`, Capítulo 19). O que continua aberto é
rodar os 40 genes **reais** com a segunda linha: o instrumento mede o desenho, não os dados da levedura.

---

### Degrau 2 — A regra do jogo

**As sete lições, com a condição em que valem.** As três primeiras lições são aritmética de decisão. A média de
uma função não é a função da média; a margem depende da unidade contada como independente; e um número só é
evidência se o procedimento que o produziu podia ter dado errado. As duas seguintes tratam do ganho que vem da
perda: reprovar o modelo que se construiu é um resultado publicável, e a invariância protege contra a intervenção
nova, enquanto o efeito, sozinho, só promete transportar em média. As duas últimas dizem respeito ao processo. A
decomposição de Murphy separa confiabilidade de resolução, e a declaração prévia separa ciência de trapaça. Nenhuma
das sete vale sem a sua condição. A sexta só vale para previsores que não podem ser melhorados por mais dados, e a
sétima só vale se a declaração for escrita antes e o contador $K$ for publicado depois.

**O balanço dos dez pontos da auditoria de fundo.** A tabela traz a pergunta e a resposta de cada ponto.

| Ponto | A suposição | O que foi medido | Resultado |
| --- | --- | --- | --- |
| 1. Tese central | A arquitetura pega erros que a prática comum deixa passar | Piloto cego de 12 casos, dois braços independentes | Empate no teto: 8 de 8 defeitos nos dois braços e 0 alarme falso nos 4 casos limpos — a tese **não fica sustentada** |
| 2. Independência | As saídas aprovadas não dependem de quem as escreveu | Quatro resultados reimplementados só a partir da especificação | Todos concordam e zero erros no original; várias escolhas que a especificação não fixa coincidem com as do autor, três delas só no STAR (evidência fraca) |
| 3 e 4. Multiplicidade | O portão do H2 foi tentado poucas vezes e nada foi ajustado depois | Inventário das tentativas e das aberturas de cofre | $K = 38$ testes distintos (32 a 72); com Bonferroni o H2g mantém as 4 áreas **antes da correção de agrupamento**, e depois da correção cai para três (Capítulo 19) |
| 5. Prioris | As prioris de literatura não foram conferidas com especialistas; a rodada de controle não tinha tolerância | Três rodadas em 33 domínios e a banda de equivalência de 0,05 dp | Cinco domínios mudam de status ou ação com a priori de literatura e dois com a fraca; o desvio máximo é 0,0071 dp e **33 dos 33** batem |
| 6. Laço performativo | Quem desiste da fila volta a chamar com probabilidade 0,6 | Desenho quase experimental numa central real, 1999 | $0{,}145$ [0,132; 0,157] — quatro vezes menos que o declarado |
| 7. Linhas dependentes | O ICP supõe linhas independentes, e há 16 instantes por voluntário e 80 escolas | Wald agrupado, permutação em blocos e réplica da genética com 40 genes novos | A farmacologia sobrevive intacta, a educação sai, a genética se replica num gene validado com $p = 0{,}001$ |
| 8. Calibração | A tolerância do ECE pode ser fixa em 0,03 | Nulo simulado com as mesmas previsões e o mesmo $n$ | Nenhum dos seis cofres binários passa pela regra que depende de $n$; pela regra secundária, o quadro não muda |
| 9 e 10. Escala e dados | O funil escala e o grafo mede conhecimento | 329 domínios em lote e dois domínios novos de saúde | Dois podados no teste severo e 327 até o fim; 26.634 nós no hipergrafo, dos quais 6.494 são de conceito |
| Evento raro | Um previsor de evento raro serve para decidir quando tem boa calibração | Decomposição de Brier em três cofres: indometacina, COVID e Rhode Island | Nenhum caminho vence nos três: só em Rhode Island a resolução bate a confiabilidade (+2,74643e-4 na logística e +3,11655e-4 no ranking), e a habilidade fica em +0,018629 e +0,017952, abaixo do +0,05 declarado; no COVID a ordenação é boa (AUC 0,7685) e a habilidade é $-0{,}020853$ |
| Saúde | O círculo que o posto de saúde abriu se fecha com dado real de saúde | Sepse precoce no espelho aberto da PhysioNet, com o funil do H2g rodado de ponta a ponta, sem uma linha mudada e com o cofre no segundo sistema hospitalar | O efeito existe e é preciso ($\beta = 0{,}0388$ em log-razão de chances por respiração por minuto, intervalo de 90% de 0,0257 a 0,0523), mas a invariância não: 0 de 512 subconjuntos aceitos, $\hat{S} = \emptyset$, e o portão falha; a habilidade no cofre é $+0{,}00568$ e a taxa de decisão registrada é 8,4% nas internações de um sistema e 3,9% nas do outro |
| Ficha e contrato | A pergunta que o modelo responde é a que o decisor fez; o contrato de dados não diz por que o dado falta | Campo obrigatório `pergunta_do_decisor` no pacote de domínio (18 do H2g, 12 do H2 e a ficha do H0) e o bloco de ausência com MCAR, MAR ou MNAR no contrato | Os dois campos entram validados no código; nos contratos de hoje a regra da ausência é vacuosa — nenhuma coluna tem um único valor ausente — e nenhum número publicado muda |
| Elicitação humana | A priori simulada seria substituída por painel de especialistas | Protocolo do painel e seis formulários, com o critério de $K = 2$ fixado | Nada foi elicitado: nenhuma entrevista, nenhuma previsão corrigida, nenhuma linha de decisão assinada |

**Nota de aposentadoria.** Os artefatos das extensões de evento raro, saúde e escala (pontos 9 e 10) saíram da árvore em 2026-10-16. Não sustentam resultado central da teoria, e os números continuam registrados na tabela acima. O material volta com `git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros`.

Três desses resultados apertam a autorização dos capítulos anteriores. O laço de 0,6 que o H3 corrigiu é cerca de
quatro vezes o valor medido num caso real. A tolerância de 0,03 do erro de calibração é maior que o ruído em todos
os cofres grandes, e nenhum deles passa na regra que depende de $n$. E o portão de transferência do H2 depende de
qual versão de cada escolha conta. A área da educação não sobrevive à correção de agrupamento, e a terceira área
agora se sustenta na genética replicada.

**As sete objeções, e a resposta a cada uma.** Uma disciplina que exige teste severo precisa sobreviver às objeções
que lhe fazem.

| Objeção | Resposta |
| --- | --- |
| "Isso é só estatística com outro nome." | Em parte, sim: quase todas as ferramentas têm dono há décadas. O que a Estocologia acrescenta é a organização — as camadas, a tipagem da incerteza e os mesmos portões para toda alegação. O primeiro teste direto disso, o piloto cego, **empatou no teto**: os dois braços apontaram os mesmos 8 defeitos e os mesmos 0 alarmes falsos. Um empate no teto não sustenta a tese nem a refuta |
| "Os dados são antigos e públicos." | Foi escolha: bases conhecidas, verificáveis por hash e com desfecho sabido permitem auditar o método, mas não substituem uma operação de verdade. Várias peças continuam supostas — o tempo de atendimento, a paciência dos clientes, o painel que nunca foi consultado. Outras deixaram de ser suposição: o laço performativo, a tolerância da calibração e a fragilidade da genética foram medidos |
| "Formalizar o julgamento é eliminar o julgamento." | A arquitetura não decide o que vale a pena perguntar nem quanto um erro custa para quem o sofre. Esses valores entram pela elicitação e pela razão entre custo e perda, declarados e não calculados. A ambição é tornar o julgamento visível |
| "O aparato é pesado demais." | Oito faixas de agentes, cinco lentes e cinco invariantes são muito para uma pergunta simples. O funil responde: a maior parte das hipóteses cai barato, antes de tocar os dados. Se o custo da máquina superar o valor das decisões que ela melhora, a própria lente do juízo manda desligá-la |
| "Bayes resolve o problema de Hume." | Não resolve. A atualização bayesiana é a regra coerente dentro de um espaço de hipóteses, mas a indução se desloca para a escolha da priori e do espaço, como a regra de Laplace mostrou. O próprio teorema de Cox depende de hipóteses técnicas (Halpern, 1999) |
| "A ergodicidade virou doutrina." | A lente do tempo se apoia no programa de Ole Peters, que é contestado, e a disciplina o ensina como debate. Pelo critério do crescimento, a aversão a perdas de cerca de 2,25 medida em laboratório só se justifica quando 56% da riqueza estão em jogo: a tese explica a recusa de arriscar meio patrimônio, não a aversão a apostas pequenas. Algo parecido vale para as caudas: Taleb pede que se suponha cauda pesada até prova em contrário; Clauset, Shalizi e Newman pedem prova antes de declarar uma lei de potência. A questão é de quem é o ônus da prova, e a resposta depende de quanto custa errar |
| "O auditor é parte interessada." | Os 21 pareceres foram assinados por quem conduz o programa, e nomear um responsável não torna a auditoria independente. A auditoria de fundo acrescentou uma checagem mais barata e mais fraca: reimplementar do zero quatro resultados que sustentam as saídas aprovadas, a partir das especificações. Os quatro concordaram e nenhum erro foi achado — mas o teste tem um defeito declarado, porque várias escolhas que a especificação não fixa saíram iguais às do código original, três delas só no STAR. Independência real pede gente de fora, com o mesmo contrato |

**A ficha exige cinco perguntas e quatro proibições.** O instrumento que transforma uma dúvida em alegação
auditável faz cinco perguntas de cabeçalho. Qual é o **processo gerador** dos dados, com semente; onde mora a
**não-linearidade**; qual é a **unidade independente**; qual é a **física do ruído**, se o ruído entra somado
(Itô) ou se a variável é que é arrastada (Stratonovich); e em que degrau da **escada causal** a alegação vive.
Quatro proibições fecham a lista: nenhum texto livre entre agentes, nenhuma conta de cabeça feita por máquina,
nenhuma autoaprovação, e nenhuma probabilidade fora do intervalo de 1% a 99%. Quatro invariantes e sete portões
automáticos verificam a ficha, e o binário de portões sai com código 0 ou 1.

**O roteiro do projeto final.** O projeto vale como o curso inteiro. São quatro semanas de trabalho para um
problema da escola ou da cidade. O roteiro tem oito passos, e o produto final é um memorando em frequências
naturais, com cláusula de saída, defendido em banca.

1. **Escolher o problema.** Serve um problema em que alguém decide algo e há uma ação concreta, como reduzir
   turmas, mudar o horário do intervalo, instalar um bebedouro, multar estacionamento em fila dupla ou mudar o
   ponto de ônibus. Não serve "entender a violência na cidade": falta decisor e falta ação.
2. **Escrever a pergunta do decisor**, em três partes: *Quem decide*: nomeie a pessoa ou o órgão; *Ação que a
   decisão muda*: copie a ação literalmente; *A pergunta do decisor é*: termine com interrogação. Sem essas três
   partes a ficha é recusada, e o laboratório deste capítulo mostra o verificador que a recusa.
3. **Preencher a ficha.** As cinco perguntas de cabeçalho, a alegação em uma frase, o termo de escopo (onde vale e
   onde não vale), a grandeza medida e a unidade.
4. **Fixar o processo gerador** com semente numérica. Quais elementos são sorteados, quais são fixos, quantas
   linhas, quais ambientes. A semente permite que outra pessoa veja aquele mundo.
5. **Construir o modelo nulo.** Aquilo que o acaso sozinho produziria naquele desenho, com a semente repetida. Sem
   nulo não há contraste.
6. **Pré-registrar com hash** e declarar o cofre. A partição de dados que ninguém viu fica guardada, e abre uma vez
   só. Se o critério mudar depois de abrir, o novo critério é declaração nova e entra no contador de tentativas.
7. **Rodar, medir e declarar.** Todos os números do memorando saem de código que outra pessoa roda; toda tentativa
   fica no placar; toda conta de cabeça é proibida.
8. **Escrever o memorando e defender.** Frequências naturais ("em cada 100 alunos"), o gráfico dos resultados
   hipotéticos, a cláusula objetiva de saída, aquela que diz quando a recomendação deixa de valer, e a resposta à
   pergunta final, quando um modelo é bom o bastante. O livro responde com três condições. Um modelo é bom o
   bastante quando ele declara os modos de falha, quando alguém de fora consegue reproduzir cada número, e quando a
   decisão que ele autoriza não depende de nenhuma suposição que não tenha sido escrita.

**Três coisas ficam abertas, com nome.** Nenhuma delas se resolve com mais código.

- **A elicitação com especialistas humanos.** O protocolo e os seis formulários estão prontos; nenhuma entrevista
  foi feita. Cinco dos 33 domínios dependem da priori escolhida, e é esse o tamanho do que ainda não foi medido.
- **A resolução da validação por intervenção da genética.** O desenho está medido: uma linha por mutante aprova
  4 de 40 no piso de $0{,}001$ e a segunda linha aprova 1 de 40, com o piso em $10^{-6}$ (`labs --genetica`).
  Falta rodar os 40 genes reais com a segunda linha, e o p-valor nunca foi o problema.
- **A voz de quem paga o erro.** Nas sete decisões que afetam pessoas reais, nenhum grupo afetado foi ouvido. A
  releitura dos 21 pareceres foi feita (17 confirmados, 4 retirados e reemitidos), e ela não substitui a escuta:
  a assinatura do responsável é de quem decide usar o artefato, não de quem paga o erro. A frente tem agora
  protocolo congelado com hash (`fundo/p13-escuta/`) e a caixa lacrada por instrumento: as sete decisões ficam
  como análise de sensibilidade ($n = 1$) e não viram diretriz de política pública nem decisão de portão F8 até a
  cobertura mínima de 2 dos 7 grupos ser atingida, ou até 2026-12-15 (`labs --escuta`).

**Quatro horizontes ficam nomeados.** **Resolução da pergunta:** a pergunta do decisor tem grão, e o grão é variável do modelo. R0 pergunta pelo dia médio, R1 pelo dia que existe, R2 pela hora e R3 pelo subgrupo. Com o recurso total idêntico e braços pareados, perguntar grosso custa **46,24 min** de espera média onde a demanda oscila de 5 a 11, **5,97 min** onde oscila de 7 a 9 e **nada** onde não oscila (`cargo run --release --bin labs -- --resolucao`, em `crates/stochology-core/src/resolucao.rs`). É o eixo novo, ortogonal ao H0–H3. As previsões falsificáveis dele estão medidas: o limiar de oscilação aparece na amplitude 2,5 com o plano calibrado — perda de $+2{,}87$ min no cruzamento e negativa abaixo dele — e **não** aparece com o plano de Erlang; a transferência não tem violação em 6 perfis, com 3 contraexemplos que passam em R0 e falham em algum dia (`labs --resolucao`). **Do argumento ao artefato:** a unidade de conhecimento deste programa é o artefato executável, e não o parágrafo. Alegação, processo gerador, modelo nulo e teste ficam lado a lado. **Agentes artificiais com disciplina epistêmica:** modelos de linguagem entram com rubrica de avaliação ($\kappa = (p_o - p_e)/(1 - p_e)$), registro da versão e da data do fornecedor, e a proibição de texto livre entre agentes que já vale na ficha. **Prazo de validade para a meta do livro:** toda métrica deste livro é declarada com a data em que passa a valer, e o Goodhart medido (Capítulo 20) é a razão de declarar o prazo antes.

---

### Degrau 3 — Mão na massa

O laboratório deste capítulo é o **verificador da ficha do projeto final**: um portão pequeno que recusa uma ficha
sem pergunta do decisor, sem semente e sem cláusula de saída, e prova que sabe reprovar, porque cada uma das sete
regras derruba ao menos uma ficha. O código está em `crates/stochology-book/src/cap21.rs`, autocontido e sem
dependências:

```bash
cargo run --release
```

```text
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
```

Leia linha por linha. **A ficha 01 é aceita** com quem decide nomeado, a ação copiada literalmente da decisão, a
pergunta terminando em interrogação, a semente 2026 no processo gerador e no modelo nulo, o resumo criptográfico de
64 dígitos e a cláusula de saída ancorada num número ("acima de 0,10 dp em três anos seguidos"). **As outras onze
são recusadas, e cada recusa é um defeito distinto.** R4 aparece quatro vezes porque a pergunta do decisor é a
regra com mais partes: ausente, genérica ("o decisor" não nomeia ninguém), sem a ação literal e sem a interrogação.
R7 aparece duas vezes: uma ficha sem cláusula nenhuma e outra com uma cláusula sem gatilho ("enquanto o modelo
continuar bom e ninguém reclamar"), que não diz quando a recomendação deixa de valer. A última linha é a conta que
interessa. **7 de 7 regras reprovaram ao menos uma ficha**. Um portão que nunca reprova não é um portão; é um
carimbo.

**Declaração de divergência.** O verificador do laboratório é uma reimplementação reduzida do validador do
repositório. O validador da pesquisa exige, além das regras acima, no mínimo 60 caracteres no texto da pergunta,
recusa uma lista maior de nomes genéricos e roda sobre 18 pacotes do H2g, 12 do H2 e a ficha do H0, sem que
nenhum número publicado mude; o do laboratório roda sobre 12 fichas de teste e não toca nos dados de ninguém. O
que os dois têm em comum é o essencial: a pergunta do decisor só vale se nomeia quem decide, copia a ação
literalmente e termina em pergunta.

**O lacre da escuta.** O segundo instrumento do capítulo é o portão de quem paga o erro. Ele lê o registro em
máquina das sete decisões que mexem com pessoas reais, conta a cobertura de escuta, confere se alguma decisão se diz
emitida sem ninguém ouvido e dá o veredito do lacre. O protocolo está em `fundo/p13-escuta/protocolo.md`, congelado
com hash antes da primeira tentativa de contato:

```bash
cargo run --release --bin labs -- --escuta
```

```text
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
```

**Nenhuma das sete coortes históricas está ao alcance** — são crianças do STAR dos anos 1980, famílias do RAND HIE,
detidos de Toronto de 1997, motoristas de Rhode Island de 2005 —, e o programa declara isso em vez de inventar
consulta que não houve. O que a escuta procura é equivalente estrutural: quem hoje ocupa a mesma posição de quem
pagou o erro então. O roteiro tem três perguntas em linguagem comum (pior cenário plausível, o que faria recusar,
quem de fato arca com o custo), quem conduz é de fora do programa ou com protocolo cego, e o operador **não conta**
como grupo ouvido. O prazo é 2026-12-15, a cobertura mínima é de 2 dos 7 grupos com registro assinado, e o que não
for coberto fica arquivado como exercício de sensibilidade. A prova negativa está no próprio instrumento: uma
decisão que se diz emitida com a coluna "ouvidos" em nenhum é reprovada, e a caixa só abre quando a cobertura chega
ao mínimo.

**Exercícios.**

1. *(Conceito)* Escolha um problema da sua escola e escreva a pergunta do decisor nas três partes exigidas. Depois
   troque o decisor por "o decisor" e explique, em duas frases, por que a ficha deixaria de ser auditável.
2. *(Conceito)* Escreva uma cláusula de saída com gatilho objetivo para uma decisão da sua cidade e uma cláusula
   sem gatilho nenhum. Qual das duas sobrevive a uma troca de gestão?
3. *(Laboratório)* Rode o verificador e acrescente uma ficha 13 com uma pergunta de decisor correta e uma semente
   não numérica ("qualquer"). Quantas fichas são recusadas agora, e por qual regra?
4. *(Conta)* No balanço da auditoria, cinco dos 33 domínios mudam de status ou de ação quando a priori de
   literatura substitui a do pacote. Escreva esse número como frequência natural ("em cada 100 domínios...") e
   diga por que ele é o tamanho do que ainda falta medir.
5. *(Conceito)* O piloto cego empatou no teto: cada braço apontou 8 defeitos, e as duas listas saíram idênticas.
   Explique, em quatro frases, por que um empate no teto não sustenta a tese e também não a refuta.
6. *(Ensaio)* Responda à pergunta que fecha o curso: quando um modelo é bom o bastante? Use pelo menos três
   números deste livro.

**Resumo.**

- As sete lições resumem o programa: a média esconde; a unidade independente decide a margem; acertar não basta
  se o teste não podia falhar; reprovar o modelo que se construiu é resultado; o efeito transporta melhor que a
  lista de causas; calibração se conserta e resolução não; declarar antes de rodar torna a falha útil.
- A auditoria de fundo testou dez suposições do programa e as extensões. Nenhum número publicado caiu por erro
  de conta; a auditoria estreitou a autorização desses números. A tese central empatou, a tolerância de calibração
  era frouxa, o laço declarado era quatro vezes o medido.
- Ficam em aberto, com nome: a elicitação com especialistas humanos (5 de 33 domínios dependem da priori), a
  segunda linha nos 40 genes reais da genética (o desenho já tem instrumento: 4 de 40 passam com uma linha, 1 de
  40 com duas) e a voz de quem paga o erro (nenhum grupo afetado ouvido nas sete decisões que envolvem pessoas, com
  a caixa lacrada por instrumento e prazo em 2026-12-15).
- O projeto final fecha o curso: escolher um problema da escola ou da cidade, escrever a pergunta do decisor,
  preencher a ficha, fixar a semente, construir o nulo, pré-registrar, rodar e defender um memorando em frequências
  naturais com cláusula de saída.
