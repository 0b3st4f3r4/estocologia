## Capítulo 15. O cofre, a senha criptográfica e a auditoria contra si mesmo

**Objetivos.** Ao fim deste capítulo você deve saber explicar por que o pré-registro com hash SHA-256 é a
única senha contra o ajuste depois de ver o resultado, o que o contador de tentativas $K$ mede, por que o
cofre fora do tempo se abre uma única vez, como funcionam o diário de previsões e os pareceres assinados, e
por que o auditor ser parte interessada é um limite que nenhuma assinatura resolve.

---

### Degrau 1 — O mundo real e o paradoxo

**Pintar o alvo depois do tiro.** Um atirador dispara dez tiros contra a parede de um celeiro. Depois olha os
buracos, pega um pincel e desenha um alvo em volta do grupo mais apertado. Aí mostra o alvo e diz: "cinco
acertos no centro". A conta de acertos está certa, e o alvo é falso. A única coisa que separa a ciência
desse atirador é **quando** o alvo foi desenhado.

Não existe jeito de desfazer isso olhando o resultado. A senha consiste em escrever e **congelar** o
alvo antes do tiro, de um jeito que ninguém consiga alterar depois sem que se perceba. Em cada 100
resultados publicados, os que mais se parecem com trapaca são justamente os limpos, ajustados um pouquinho
depois de ver os dados. A diferença entre "ajustei o modelo" e "ajustei o modelo depois de ver o resultado"
não está no modelo, e sim na **ordem dos fatos**.

**A senha é um número.** Como congelar? Publicando o **hash** do arquivo. O hash é uma impressão digital. Ele pega
o conteúdo inteiro, do primeiro ao último caractere, e devolve uma linha de 64 caracteres. Trocar uma vírgula
muda a impressão toda. Publicar o hash antes do tiro é como lacrar o envelope em cartório. Você não precisa
mostrar o conteúdo agora, mas depois ninguém abre o envelope sem que apareça.

**O cofre.** Imagine um banco que recebe um boleto de cada cliente. Um dia o banco decide construir um
"cofre fora do tempo": uma pasta com os dados dos próximos três meses, que nenhum analista pode abrir. O
modelo é construído com o passado; quando termina, alguém abre o cofre e roda o modelo contra dados que ele
nunca viu. Abrir o cofre custa a única coisa que ele tinha: a surpresa. Na pesquisa, quando um cofre já foi
aberto e o modelo precisa de correção, a correção só vale testada num **cofre novo**, que ninguém viu. Foram
três cofres até acertar a calibração das prisões de Toronto.

**O diário de previsões.** Toda previsão probabilística emitida fica registrada num livro-razão contínuo, com
a fonte, o filtro, se o sistema pode influir no resultado, quantos fatores a justificativa tem e o
degrau de Pearl. E com um limite: **a previsão nunca é 0% nem 100%**, e sim algo entre "um em cem" e
"noventa e nove em cem". Sem esse limite, quem aposta na certeza nunca perde pontos e nunca aprende; com o
limite, cada previsor acumula um histórico que pode ser cobrado.

---

### Degrau 2 — A regra do jogo

**O risco de escala é epistemológico.** Quando um sistema testa centenas de hipóteses e especificações em
paralelo, o maior risco não é técnico; é o sobreajuste em escala industrial, o jardim dos caminhos que se
bifurcam percorrido por máquinas incansáveis. O plano impõe três mecanismos de contenção.

**Pré-registro com hash e contador de tentativas.** Antes que qualquer agente acesse a partição de teste, a
hipótese é congelada com hash. Toda mudança de critério, de teste ou de dados é escrita num arquivo de
declaração, com o seu SHA-256 gravado **antes** de rodar. Nenhuma regra foi mudada depois de ver o resultado
da rodada a que se aplicava, mas cada rodada nova foi escrita sabendo das anteriores, e as correções feitas
depois de ver resultados ficaram registradas e **contam no $K$**.

O sistema mantém um contador $K$ de especificações tentadas por pergunta. Toda tentativa falha fica
registrada, e o relatório final é obrigado a declarar $K$. O plano vai mais longe e pede que $K$ penalize a
complexidade, pelo comprimento mínimo de descrição (MDL) ou pelo fator de Occam, junto com a correção para
comparações múltiplas. Na pesquisa, $K$ foi declarado, mas **nenhuma dessas penalidades foi aplicada**. Foi
por esse contador que a respecificação dos domínios do $H_2$ parou em $K = 3$, e o passo seguinte foi buscar
dados novos. O contador por domínio não registra, porém, que o portão foi tentado em sete rodadas, e
nenhuma correção por isso foi aplicada. A auditoria de fundo depois desse passo mediu o $K$ **honesto** do
portão do $H_2$: **38 testes distintos** (com a faixa de 32 a 72 segundo o critério de contagem), e com
Bonferroni o $H_{2g}$ mantém as quatro áreas de então, com os conjuntos $\hat S$ encolhendo.

**O cofre fora do tempo.** A partição temporal futura fica sob custódia exclusiva do agente de ciclo de vida
($F_3$). Nenhum agente de modelagem a lê durante o desenvolvimento; ela é aberta uma única vez para pontuar
calibração e verificar ausência de vazamento. A regra é dura e tem consequência. Quando um cofre já foi
aberto e o modelo precisa de correção, a correção só vale testada num cofre novo. O $H_3$ fez isso três
vezes: com os voos de 2023, com as prisões de Toronto de 2020–2021 e com as paradas de trânsito de Rhode
Island.

**O diário de previsões e o barramento.** Cada agente que emite previsões probabilísticas as grava num
livro-razão contínuo, restrito ao intervalo $[1\%, 99\%]$. Agentes cujo escore de Brier perde para a
taxa-base, ou cujo capital de Kelly colapsa, têm o peso reduzido pelo orquestrador. O barramento de fichas é
**encadeado por hash**: cada mensagem carrega o hash da anterior, e a trilha não pode ser reescrita sem quebrar
a cadeia. Uma lição de implementação veio cedo. No $H_1$, o placar do diário somava num só o previsor
que aparecia em projetos diferentes, e o erro foi corrigido e declarado no README; os números do $H_1$
publicados depois disso já vêm da versão corrigida.

**Pareceres de risco e o responsável nomeado.** O agente de governança ($F_5$) emite um parecer de risco para
cada projeto avaliado, inclusive os que não chegam à decisão. Na pesquisa foram **21**: 18 do $H_2$ e 3 do
$H_1$ real. Quatro deles envolvem decisões sobre pessoas: Direito (regra de indenização trabalhista),
Educação (turmas do STAR), Saúde (o experimento de seguro da RAND) e o programa de treinamento de LaLonde.

Os 21 pareceres foram assinados em 15 de outubro de 2026, depois de ler um resumo deles que destacava os
quatro que envolvem pessoas. A assinatura fica num registro separado, em que cada parecer é identificado pelo
SHA-256 do arquivo no momento da assinatura. Os arquivos de resultado **não foram reescritos**, para não mudar
as saídas que a reprodução limpa compara. O parecer do Direito carrega uma ressalva factual. A recomendação de
adotar um corte de 10% vem de uma decisão e de um custo **inventados** no $F_1$, e não é conselho de política.

As assinaturas são nominais. Cada parecer traz o nome curto do responsável (**sil**) e o resumo criptográfico do arquivo, em `governanca/ASSINATURAS.md`.

**O limite: o auditor é parte interessada.** Os pareceres foram assinados por quem conduz o programa. Uma
auditoria autorizada por quem é auditado tem valor limitado. O responsável nomeado responde à pergunta que
uma auditoria precisa responder (quem decidiu usar, quem aprovou, quem olha o placar, quem desliga), mas
nomear um responsável **não torna a auditoria independente**. Na direção que uma auditoria exige, o programa
registrou os pontos que a auditoria **não pôde olhar** e quais grupos afetados foram ouvidos. O registro
declara que, nas sete decisões que envolvem pessoas (cortar o benefício em 10%, reduzir as turmas
do STAR, trocar o cuidado gratuito por coparticipação, comparar o treino do NSW com controles do CPS e as
três marcas de revisão dos fluxos de prisões), **nenhum dos sete grupos afetados foi ouvido**. Não há no
repositório registro de consulta, entrevista, painel ou consentimento. Falta também o passo mais barato e
mais óbvio da lista: a elicitação com especialistas humanos.

**A auditoria de fundo: o que foi suposto e o que se mediu.** A governança também serve para marcar os pontos
**não** medidos, e para depois medi-los. A pesquisa se vira contra si mesma numa auditoria de fundo. Dez
pontos que o texto sustentava e que nunca haviam sido postos à prova viraram frentes de trabalho, às quais se
somaram, depois, as extensões (evento raro, saúde, ficha e contrato) e a frente da elicitação humana, que
espera gente. Cada frente repete a regra dos horizontes: a declaração com hash vem **antes** de rodar, o
resultado é relatado com os limites, e mudança de critério depois de ver o resultado entra como declaração
nova e conta no número de tentativas.

| Ponto | A suposição | O que foi medido | Resultado |
| --- | --- | --- | --- |
| 1. Tese central | A arquitetura pega erros que a prática comum deixa passar | Piloto cego de 12 casos, dois braços | Empate no teto: 8 de 8 defeitos nos dois, 0 alarme falso nos 4 casos limpos — a tese não fica sustentada |
| 2. Independência | As saídas aprovadas não dependem de quem as escreveu | Quatro resultados reimplementados só a partir da especificação | Todos concordam; zero erros no original; várias escolhas que a especificação não fixa coincidem com as do autor, três delas só no STAR (evidência fraca) |
| 3 e 4. Multiplicidade | O portão do $H_2$ foi tentado poucas vezes, e nada foi ajustado depois | Inventário das tentativas e das aberturas de cofre | $K = 38$ testes distintos (32 a 72); com Bonferroni o $H_{2g}$ mantém as 4 áreas de então e os conjuntos encolhem |
| 5. Prioris | As prioris de literatura não foram conferidas com especialistas; a rodada de controle não tinha tolerância declarada | Três rodadas em 33 domínios (pacote, literatura e fraca) e a banda de equivalência de 0,05 dp | Cinco domínios mudam de status ou ação com a de literatura e dois com a fraca; o veredito deixa de ser condicional — desvio máximo de **0,0071 dp** e **33 dos 33** domínios idênticos |
| 6. Laço performativo | Quem desiste da fila volta a chamar com probabilidade 0,6 | Desenho quase experimental numa central real, 1999 | **0,145** [0,132; 0,157] — quatro vezes menos que o declarado |
| 7. Linhas dependentes | O ICP supõe linhas independentes, e há 16 instantes por voluntário e 80 escolas | Wald agrupado e permutação em blocos; réplica da Genética com 40 genes novos e taxa nula por permutação ($B = 999$) | A Farmacologia sobrevive intacta, a Educação sai; a Genética se replica em um gene validado, com $p = 0{,}001$ da taxa nula |
| 8. Calibração | A tolerância do ECE pode ser fixa em 0,03 | Nulo simulado com as mesmas previsões e o mesmo $n$ | Nenhum dos seis cofres binários do $H_3$ passa pela regra que depende de $n$; pela regra secundária, o quadro não muda |
| 9 e 10. Escala e dados | O funil escala e o grafo mede conhecimento | 329 domínios em lote; dois domínios novos de saúde | Dois podados no teste severo; 327 até o fim; 26 634 nós, dos quais 6 494 de conceito |
| Evento raro | Um previsor de evento raro serve para decidir quando tem boa calibração | Decomposição de Brier em três cofres: indometacina, COVID e Rhode Island | Nenhum caminho vence nos três: só em Rhode Island RES − REL fica acima de zero (+2,74643e-4 na logística, +3,11655e-4 no ranking), e a habilidade de Brier fica em +0,018629 e +0,017952, abaixo do +0,05 declarado; no COVID a AUC é 0,7685 e a habilidade é −0,020853 |
| Saúde | O círculo que o Lab 0 abriu se fecha com dado real de saúde, em espelho aberto | PhysioNet/CinC Challenge 2019 (sepse precoce) no espelho do PhysioNet; o funil do $H_{2g}$ rodou de ponta a ponta sem uma linha mudada, com o cofre no segundo sistema hospitalar | O efeito existe e é preciso ($\beta = 0{,}0388$ em log-razão de chances por respiração por minuto, IC90 0,0257 a 0,0523), mas a invariância não: 0 de 512 subconjuntos aceitos, $\hat S = \emptyset$, e o G3 falha; a habilidade no cofre é +0,00568 e a taxa de decisão registrada é 8,4% nas internações de um sistema e 3,9% nas do outro |
| Ficha e contrato | A pergunta que o modelo responde é a que o decisor fez; o contrato de dados não diz por que o dado falta | Campo obrigatório `pergunta_do_decisor` no pacote de domínio e bloco `ausencia` no contrato do $F_2$ | Os dois campos entram validados no código; nos três contratos do $H_1$ real e nas 18 tabelas do $H_{2g}$ a regra da ausência é vacuosa hoje e nenhum número publicado muda |
| Elicitação humana (SHELF) | A priori simulada seria substituída por painel de especialistas | Protocolo do painel e seis formulários escritos; critério de $K = 2$ fixado | Nada foi elicitado: nenhuma entrevista, nenhuma previsão corrigida e nenhuma linha de decisão assinada — a frente só tem instrumento |

**Nota de aposentadoria.** Os artefatos das extensões de evento raro, saúde e escala saíram da árvore em 2026-10-16 por não sustentarem resultado central da teoria; os números acima continuam registrados aqui, e o material volta com `git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros`.

Três desses resultados apertam o alcance autorizado pelos capítulos anteriores. O laço de 0,6 que o $H_3$
corrigiu é cerca de quatro vezes a medida de um caso real. A tolerância de 0,03 do erro de calibração é maior
que o ruído em todos os cofres grandes, e nenhum deles passa na regra que depende de $n$. E o portão de
transferência do $H_2$ depende de qual versão de cada escolha conta. A área da Educação não sobrevive à
correção de agrupamento, e quem sustenta a terceira área agora é a Genética replicada. **Nenhum número
publicado caiu por erro de conta; a auditoria estreita o alcance autorizado desses números.**

As questões em aberto têm nome. A elicitação com especialistas humanos de verdade, porque a auditoria
mediu a sensibilidade às prioris e isso não substitui um painel. O protocolo e os seis formulários estão
escritos, com o critério de $K = 2$ pronto, e **nenhuma entrevista foi feita**. A taxa nula da validação por
intervenção da Genética, que decide com um único gene em 27, saiu em $p = 0{,}001$, o piso de 999
sorteios; a resolução segue em aberto ali, e não o $p$. E o tempo de atendimento e a paciência dos
clientes da central, que a base não traz e que continuam supostos.

Fica também o limite de o auditor ser parte interessada, que a auditoria não resolveu. Um dos passos que ela
propõe é que quem assinou releia por inteiro os 21 pareceres assinados, um a um, e confirme ou retire
a assinatura registrando que leu o texto integral. O formulário de releitura das 21 linhas existe, com os
hashes conferidos um a um (**21 de 21 batem**), e está **em branco de propósito**. Nada foi relido, nada foi
confirmado e nada foi assinado ali. Enquanto isso não acontece, a assinatura vale pelo que ela é: a
declaração de quem conduz o programa de que leu um resumo.

---

### Degrau 3 — Mão na massa

Duas tarefas: construir um livro-razão encadeado por SHA-256 e provar que trocar **um caractere** no passado
derruba a corrente; e calcular a raiz de Merkle de um lote de fichas. O código traz o SHA-256 escrito à mão,
sem dependências, e confere a implementação contra os vetores conhecidos. Código em `crates/stochology-book/src/cap15.rs`;
saída medida com `cargo run --release` no crate de rascunho `/tmp/labs-cap15`:

```text
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
```

Quatro leituras.

**A implementação está certa, e isso é a base de tudo.** Os dois vetores conhecidos do SHA-256 batem
caractere a caractere. A string vazia dá `e3b0c442...b855` e `abc` dá `ba7816bf...15ad`. Sem essa
conferência, todo o resto da trilha seria um número inventado com aparência de segurança.

**Um caractere no passado derruba a corrente.** A corrente de cinco eventos está íntegra. Mudando
`canario=ok` para `canario=nok` no evento 2 e **recalculando o hash daquele evento** (o ataque mais
óbvio, em que conteúdo e hash concordam entre si), a verificação falha. O evento 3 guarda o hash antigo do
evento 2 como "anterior", e a incompatibilidade aparece. Das cinco alterações testadas, uma por evento,
**5 de 5** invalidam a corrente.

**A corrente sozinha não basta: a raiz é a prova.** Se o atacante reescrever também os três eventos seguintes,
a corrente volta a fechar localmente (a verificação passa), mas a **raiz publicada** muda. Essa linha é o
coração do capítulo. A trilha fica inviolável pelo hash publicado **antes**, e não pela corrente. Quem
guarda só a corrente guarda um documento que pode ser reescrito por inteiro; quem guarda a raiz guarda o
lacre.

**A raiz de Merkle muda sempre.** O lote de sete fichas tem raiz `17679ce1...`. Trocando qualquer uma das
sete por uma versão alterada, a raiz muda em **7 de 7** casos. É isso que permite lacrar um lote inteiro com
64 caracteres em vez de 64 por arquivo.

**Declaração de divergência.** Dois pontos ficam registrados. (1) O plano do repositório publica, para o
horizonte `todos`, a raiz de Merkle `9716e28a87bec9cf...` sobre as 12 fichas dos quatro
horizontes, e um hash do relatório `08e7c8f0...`; este laboratório usa uma corrente **própria** de
cinco eventos de livro didático, e por isso a raiz medida aqui é `92f2ab76...`, não aquela. As duas
contas são corretas para os seus conteúdos; comparar as duas só faz sentido se a entrada for a mesma, e é
exatamente assim que a reprodução limpa do G1 funciona. (2) O livro anterior diz que a primeira execução da
central travou porque o Erlang C escrito com fatoriais estourava em cargas grandes; a troca pela recursão
estável de Erlang B só afeta a decisão, não as previsões, mas muda o hash do módulo e **conta no $K$ (3)**.
Nenhum valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo 15 | sem perda medida neste capitulo | cada cofre é um grão novo: a régua do eixo R quando a correção muda de pergunta
```

**Exercícios.**

1. *(Conceito)* Por que abrir o cofre duas vezes destrói o seu valor? Relacione com a severidade de Mayo,
   isto é, com a exigência de que um teste pudesse ter dado outro resultado.
2. *(Conceito)* Um colega propõe reescrever os arquivos de resultado com a assinatura dentro. Que propriedade
   da reprodução limpa isso quebraria?
3. *(Conta)* O contador do portão do $H_2$ parou em $K = 3$ por domínio; a auditoria de fundo mediu $K = 38$
   testes distintos no conjunto. Explique, em três frases, por que os dois números convivem sem que nenhum
   esteja errado.
4. *(Laboratório)* Estenda o laboratório para um barramento em que cada mensagem carrega o hash da anterior
   **e** o índice, e escreva um verificador que detecte não só a alteração de conteúdo mas também a troca de
   ordem de duas mensagens.
5. *(Laboratório)* Calcule a raiz de Merkle do lote de sete fichas para as raízes intermediárias e confirme
   quantos hashes são recalculados quando só a última ficha muda.
6. *(Conceito)* Proponha um procedimento para tornar a auditoria dos quatro pareceres que envolvem pessoas
   menos dependente de quem conduz o programa.

**Resumo.**

- O pré-registro com hash é a única senha contra o ajuste depois de ver o resultado. A hipótese é congelada
  antes, e o hash publicado é o lacre.
- O contador $K$ torna as tentativas visíveis; a auditoria de fundo mediu $K = 38$ no portão do $H_2$, contra
  o $K = 3$ por domínio.
- O cofre fora do tempo só se abre uma vez; correções pedem cofre novo, e foram três até acertar a calibração.
- O diário pontua cada previsor com regra própria, dentro de $[1\%, 99\%]$, e pesa os agentes pelo placar; a
  corrente de fichas é encadeada por SHA-256.
- Pareceres assinados nomeiam o responsável, mas não tornam a auditoria independente. Nenhum dos sete grupos
  afetados foi ouvido, e o formulário de releitura dos 21 pareceres está em branco de propósito.
