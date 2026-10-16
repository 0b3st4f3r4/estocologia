## Capítulo 13. A equipe de especialistas, a banca adversarial e a IA sob controle

**Objetivos.** Ao fim deste capítulo você deve saber o ponto cego de cada uma das oito faixas Faixa 1 a Faixa 8,
percorrer o ciclo de seis estágios, aplicar a condição de aprovação da banca adversarial
$\mathrm{Aprovado}(X) = \bigwedge_{i=1}^{8} \left( A_i(X) \ge \mathrm{Severidade}_{\min} \right)$ e
distinguir o Sistema 1 (triagem rápida, probabilidades presas entre 1% e 99%) do Sistema 2
(deliberação sobre o grafo).

---

### Degrau 1 — O mundo real e o paradoxo

**O grupo que se aprova.** Um estudante entrega um trabalho de estatística. Ele mesmo escolheu a amostra,
limpou os dados, rodou o teste, olhou o resultado, mudou o teste, rodou de novo e escreveu a conclusão.
Quando a conclusão finalmente agradou, ele parou. Ninguém trapaceou: cada passo pareceu razoável no momento
em que foi dado.

Agora imagine que o mesmo trabalho seja dividido entre oito colegas, cada um responsável por uma coisa só,
e que um nono colega receba os artefatos — não o relatório, os **artefatos**: o código, o arquivo de dados,
o modelo salvo, a trilha de auditoria — e tente quebrá-los. Em cada 100 trabalhos feitos por uma pessoa só,
a parte mais frágil é a que ninguém olhou de fora. Em 100 trabalhos com uma equipe que ataca, a parte frágil
é a que sobreviveu ao ataque.

O nono colega tem uma regra: **não ler o relatório**. Ele roda o código com uma semente que o autor não
escolheu, injeta um lote corrompido, adultera uma linha da trilha para ver se o verificador percebe. Quem
só lê relatórios concorda com eles.

**As oito faixas e o ponto cego de cada uma.** Cada faixa existe porque existe um jeito de errar que ela vê
melhor do que as outras — um ponto cego que as demais não cobrem.

| Faixa | O que faz | O ponto cego que ela cobre | O que ela entrega |
| --- | --- | --- | --- |
| Faixa 1 Código | Transforma hipóteses em processos geradores executáveis | "O código roda" não é "o código é a hipótese" | Semente fixa, pré e pós-condições, testes de propriedades, `cProfile` e `tracemalloc` |
| Faixa 2 Dados | Audita a entrada do Mundo | O que é uma linha, e por que o dado falta | `datacontract.yaml`, quarentena de registros defeituosos, ingestão idempotente, orçamento de bytes |
| Faixa 3 Ciclo de vida | Testa o modelo no tempo | O mundo muda e o modelo não avisa | Corte temporal sem vazamento, sensores de deriva, política de ignorância, post-mortem |
| Faixa 4 Fronteira | Roda a inferência pesada | Uma amostra não é uma resposta | Diagnóstico de cadeias (as várias cadeias têm de concordar entre si), calibração por simulação, predição conforme |
| Faixa 5 Governança | Cuida de legalidade, privacidade e trilha | Dado pessoal é combinação de campos, não tipo de arquivo | Parecer de impacto, privacidade diferencial, responsável humano nomeado, SHA-256 |
| Faixa 6 Justiça | Pergunta quem paga pelo erro | A métrica escolhida decide quem paga | Auditoria de taxas de erro entre grupos, trilema da justiça |
| Faixa 7 Domínio | Traduz problemas de outras áreas | O parâmetro que não vem dos dados vem de alguém | Glossário, elicitação SHELF, termo de escopo |
| Faixa 8 Decisão | Converte o resultado em ação | Um número sem frequência natural não decide nada | Frequências naturais, HOPs, árvore C/L, cláusula de saída |
| Orquestrador | Coordena e comanda a banca | Nenhum passo aprova o próprio resultado | Ficha da crítica de modelos e ataque ao artefato antes da aprovação |

**O ofício de cada faixa, em um caso.** A faixa 2 começa por uma decisão que ninguém vê: **o que é uma linha**.
Uma linha por pessoa, por dia ou por evento muda a média, a variância e a margem. Por isso o grão é
declarado e provado com uma contagem. O mesmo agente caça o **valor sentinela** — o ausente gravado como 0,
o −1 ou o 9 999 que entra na soma como se fosse dado — e exige ingestão idempotente: rodar a carga duas vezes
tem de dar o mesmo número.

A faixa 5 lembra que dado pessoal não é um tipo de arquivo: é a **combinação de campos** que decide. CEP, data
de nascimento e sexo, juntos, identificam provavelmente 87% das pessoas nos Estados Unidos, pelo censo de
1990. A privacidade diferencial limita quanto a presença de uma pessoa pode mudar qualquer resposta: para
duas bases que diferem no registro de uma única pessoa, o resultado de qualquer consulta não pode mudar
mais do que um fator fixo. Esse fator é um orçamento, gasto a cada consulta. A medida mais barata continua sendo não coletar.

A faixa 3 exige que todo modelo em operação declare o que faz em três casos: entrada fora do domínio, campo
ausente e confiança baixa. As opções são recusar, ativar um reserva determinístico seguro ou devolver
incerteza calibrada. Um modelo em uso piora por motivos diferentes, que pedem respostas diferentes.

| Situação | O que muda | A resposta certa |
| --- | --- | --- |
| Deriva de covariável | Muda a distribuição das entradas, e a relação entre atributo e desfecho continua valendo | Reponderar, com pesos pelo quociente das densidades |
| Deriva de rótulo | Muda a frequência dos desfechos, e a distribuição do atributo dentro de cada desfecho continua valendo | Reajustar as taxas-base (correção de intercepto) |
| Deriva de conceito | Muda a própria relação entre atributo e desfecho | Nenhuma reponderação salva: refazer o modelo |
| População atendida mudou | A métrica cai porque o público mudou | Não é piora do modelo: ler a métrica com a taxa-base |

A correção da deriva de rótulo tem forma fechada: se a distribuição do atributo dentro de cada classe não muda,
a probabilidade corrigida é a antiga multiplicada pela razão entre a taxa-base nova e a antiga, depois
renormalizada. Com dois desfechos, a correção soma uma constante ao logito — é uma **correção de
intercepto**. A hierarquia de Van Calster e colegas (2016) separa dois níveis que o terceiro horizonte encontrou: a
calibração na média, medida pelo intercepto, e a **calibração fraca**, que exige intercepto 0 e inclinação 1.
No terceiro horizonte, os voos de 2023 passaram com a correção do intercepto e as prisões de Toronto não, porque o erro
delas era de inclinação.

A faixa 7 não pede ao especialista uma média: o protocolo de Sheffield (SHELF) pede a **mediana**, os
**quartis** e os **extremos plausíveis**, ajusta uma distribuição a esses quantis e a devolve para que ele a
confirme, controlando a âncora do entrevistador, a ordem dos cenários e o excesso de confiança. Na pesquisa,
nenhuma elicitação foi feita com especialista humano. O que a auditoria conseguiu medir foi a **sensibilidade
à escolha da priori**: em 33 domínios, a priori do pacote foi trocada pela de literatura e por uma
deliberadamente fraca; cinco domínios mudam de status ou de ação com a de literatura e dois com a fraca. A
conclusão não é que as prioris simuladas estivessem erradas: é que em cinco de 33 domínios a decisão não
sobrevive à troca da priori.

**Uma lacuna honesta.** Falta à arquitetura uma pergunta que a faixa 2 deveria fazer: **por que o dado está
ausente?** O contrato de dados limita a nulidade, mas não classifica o mecanismo da ausência (MCAR, MAR ou
MNAR, Capítulo 6), e é essa classe que decide se o descarte dos casos incompletos é inofensivo. A auditoria
depois acrescentou um bloco `ausencia` ao contrato; e, nos três contratos do horizonte 1 real e nas 18
tabelas do segundo horizonte, a regra é **vacuosa hoje** — nenhuma coluna tem um único valor ausente. A trava passa a
morder no dia em que aparecer.

**Sistema 1 e Sistema 2.** O computador que ajuda a pensar tem dois modos. O **Sistema 1** triagem rápida,
categórica, fecha a ficha com uma árvore de sufixos determinística — mesma entrada, mesma resposta — e emite
probabilidades **presas ao intervalo de 1% a 99%**. O **Sistema 2** delibera: busca em árvore sobre o
hipergrafo, síntese de código, inferência pesada, estresse adversarial. A regra do intervalo não é
estética: uma probabilidade de 0% ou de 100% não pode ser pontuada por uma regra própria, nunca perde e
nunca aprende.

---

### Degrau 2 — A regra do jogo

**O ciclo de seis estágios.** Uma pergunta percorre seis estágios, cada um com fichas de saída. O caminho
até a decisão passa por todos.

1. **Tradução e elicitação (Faixa 7).** Ficha da transferência, elicitação SHELF, termo de escopo.
2. **Ingestão, grão, causalidade e ética (Faixa 2, Faixa 5, Faixa 6).** Contrato de dados, ficha da amostra, grafo
   causal com o degrau de Pearl, parecer de privacidade, auditoria de justiça.
3. **Simulação, modelo nulo e fronteira (Faixa 1, Faixa 4, Faixa 3).** Processo gerador executável, modelo nulo
   simulado **antes** de ler qualquer padrão, diagnósticos de inferência, partição fora do tempo, contagem
   de tentativas.
4. **Projeção nas cinco lentes (o orquestrador).** Varredura de Fluxonomia, Crononomia, Escalonomia,
   Morfonomia e Crisonomia.
5. **Banca adversarial (o orquestrador).** Reprovado volta ao estágio que falhou; aprovado segue.
6. **Decisão e fechamento (Faixa 8).** Memorando em frequências naturais, HOPs, árvore C/L e cláusula de saída.

O único caminho até a decisão passa pela banca; um trabalho reprovado volta ao **estágio em que falhou**, não
ao começo.

**A banca adversarial.** Três falhas eliminam o trabalho antes de qualquer discussão de mérito: vazamento
temporal, contrato de dados violado sem tratamento e ausência de um responsável humano nomeado. Depois, cada
faixa recebe o seu ataque, oito no total.

| Faixa atacada ($A_i$) | O ataque | O cuidado que a ficha precisa ter |
| --- | --- | --- |
| $A_1$ Código | Uma bateria nova de propriedades, com semente que o autor não escolheu | teste de propriedade |
| $A_2$ Dados | Um lote com registros corrompidos, que precisa ser isolado | tratamento de ausentes |
| $A_3$ Ciclo de vida | Um lote com deriva forte, ao vivo | deriva declarada |
| $A_4$ Inferência | Um conjunto de dados patológico, para ver se a incerteza denuncia a inadequação | patologias cobertas |
| $A_5$ Governança | Adulteração da trilha de auditoria, que o verificador da cadeia tem de detectar | trilha íntegra |
| $A_6$ Justiça | Inverter a métrica escolhida e mostrar quem passaria a pagar | subgrupos auditados |
| $A_7$ Domínio | Um cenário atípico do domínio, com priori alternativa | priori elicitada |
| $A_8$ Decisão | Uma crise que viola a própria cláusula de saída | cláusula de saída declarada |

Os oito ataques aparecem em registros antigos como **MB1–MB8**; é a mesma lista, com o nome novo $A_1$–$A_8$.

**Calibração por simulação (faixa 4).** $\mathrm{rank}(\tilde\theta; \theta^{(1)}, \dots, \theta^{(L)}) \sim \mathrm{Uniforme}\{0, \dots, L\}$: se o posterior estiver calibrado, o posto do valor verdadeiro entre as $L$ réplicas é uniforme. É assim que a faixa 4 verifica se a inferência pesada devolve incerteza do tamanho certo — e não apenas um número.

**A condição de aprovação.** Cada agente $A_i$ devolve uma **severidade** na escala de 0 a 10, e o artefato
$X$ só é aprovado se **todos** os oito atingirem o piso:

$$\mathrm{Aprovado}(X) = \bigwedge_{i=1}^{8} \Big( A_i(X) \ge \mathrm{Severidade}_{\min} \Big),\qquad
\mathrm{Severidade}_{\min} = 6.$$

O $\bigwedge$ é o "e" lógico: basta um agente abaixo do piso para o trabalho voltar. A severidade é
determinística: sai do cuidado correspondente na ficha e de uma semente explícita, e a mesma ficha com a
mesma semente devolve sempre o mesmo número. A condição **falha** exatamente quando um cuidado foi declarado
como ausente; e é isso que a torna auditável — não há juízo subjetivo escondido na nota.

**A ficha da crítica de modelos.** O orquestrador só aprova depois de preencher cinco itens: checagem
preditiva contra réplicas do modelo ajustado; comparação com pelo menos um modelo rival; separação entre
parâmetros identificados pelos dados e trazidos de fora; análise de performatividade, Goodhart e Lucas; e
defesa das premissas indecidíveis pelo **custo do erro**.

**Um trabalho com lacuna declarada vale mais que um trabalho uniforme.** Um artefato completo com uma faixa
fraca declarada vale mais que um artefato uniforme sem lacuna admitida. No horizonte 1 real, foi a banca que
reprovou o projeto dos incêndios dinamarqueses: a alegação perdeu, mas o modelo rival **não ganhou o direito
de decidir**.

---

### Degrau 3 — Mão na massa

Duas medidas: rodar a banca completa sobre uma ficha com os oito cuidados e sobre uma ficha com cuidados
removidos, imprimindo as oito severidades; e conferir que a triagem do Sistema 1 nunca sai do intervalo
entre 1% e 99%. Código em `crates/stochology-book/src/cap13.rs`; saída medida com `cargo run --release`
no crate de rascunho `/tmp/labs-cap13`:

```text
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
```

O laboratório acima usa uma triagem de duas probabilidades, suficiente para mostrar a trava. O **Sistema 1 de
verdade** está implementado em `crates/stochology-core/src/s1.rs`, com vocabulário fechado de seis categorias e
35 temas, árvore de sufixos canônica de 136 entradas e soma conferida a cada rodada. A saída medida com
`cargo run --release --bin labs -- --s1`:

```text
== Sistema 1 :: triagem determinística de fichas ==
arvore de sufixos: 136 sufixos indexados em 6 temas declarados
categoria=fila probabilidade=0.8988 produtos=[0.8988, 0.0202, 0.0202, 0.0202, 0.0202, 0.0202] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.8988 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=causal probabilidade=0.9361 produtos=[0.0128, 0.9361, 0.0128, 0.0128, 0.0128, 0.0128] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9361 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=cauda probabilidade=0.9117 produtos=[0.0177, 0.0177, 0.9117, 0.0177, 0.0177, 0.0177] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9117 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=calibracao probabilidade=0.9361 produtos=[0.0128, 0.0128, 0.0128, 0.9361, 0.0128, 0.0128] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9361 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=justica probabilidade=0.9117 produtos=[0.0177, 0.0177, 0.0177, 0.0177, 0.9117, 0.0177] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.9117 | soma 1.0000 | fora de [0.01;0.99] 0
categoria=outros probabilidade=0.8718 produtos=[0.0256, 0.0256, 0.0256, 0.0256, 0.0256, 0.8718] soma=1.0000 fora_de[0.01;0.99]=0 | probabilidade 0.8718 | soma 1.0000 | fora de [0.01;0.99] 0
```

Seis linhas, e em todas o mesmo contrato: soma 1,0000 e nenhuma probabilidade fora de [0,01; 0,99]. A entrada
desconhecida (`xyzzy plugh abracadabra`) cai em `outros` com 0,8718 — a priori declarada, não um palpite. Os
limites ficam registrados: o vocabulário é fechado, o casamento é lexical e não entende negação, e isto é
triagem, não julgamento — julgar é do Sistema 2.

Quatro leituras.

**A ficha completa passa, e passa com folga.** As oito severidades ficam entre 8 e 10, e a menor é 8 — dois
pontos acima do piso 6. O portão do repositório, no horizonte `todos` com 12 fichas, também publica
pior = 8, e muda a semente do laboratório (7, 2026, 999 983) não altera o veredito: a reprovação não é sorte
de semente, é cuidado ausente.

**Cada cuidado removido derruba exatamente o agente que o cobra.** Nas oito linhas da varredura, o conjunto
de reprovados tem sempre **um** elemento, e é sempre o agente correspondente: sem trilha íntegra reprova o
$A_5$ (severidade 2), sem cláusula de saída reprova o $A_8$ (4), sem teste de propriedade reprova o $A_1$
(3). Os outros sete ficam intactos. É exatamente o que o desenho promete: a banca não é uma nota média que
dilui, é um "e" lógico que localiza a falha.

**Remover dois cuidados reprova a ficha inteira.** Com A5 e A6 removidos, as severidades caem para 2 e 3 e a
ficha é reprovada — 8 de 8 fichas defeituosas falham, a menor severidade global é 2. Compare com o portão de
verdade: `portoes --horizonte h3 --defeito h3.voos23` devolve `G6 reprovado banca A1-A8 com
piso 6 e pior 2`. O laboratório reproduz o mesmo pior = 2 removendo o cuidado de governança e o de justiça.

**O Sistema 1 respeita o intervalo.** A triagem devolve 0,06 e 0,94, soma 1,0000, e zero valores fora de
entre 1% e 99%. Note o que o intervalo protege: uma probabilidade de 0% não seria pontuada por regra
própria e nunca seria corrigida pelo placar — o limite é a diferença entre um previsor e uma afirmação
dogmática.

**Declaração de divergência.** A edição anterior deste livro fala em "oito faixas e um orquestrador M15". O
texto do repositório (`PLAN.md`) chama o orquestrador de o orquestrador e a implementação
(`OrquestradorM`) também; a sigla "M15" sobrevive no `AGENTS.md` e em trechos antigos. São o
mesmo objeto, e este livro usa o orquestrador por ser o nome do código. O laboratório desta edição mede as
severidades a partir dos **oitos cuidados da ficha** (bandeiras verdadeiro/falso) e não dos ataques em si:
rodar o ataque é a maquinaria do repositório, e o número que a banca publica é o da severidade. Por isso os
valores 8, 9 e 10 aqui reproduzem o pior = 8 do portão, e não um relatório de ataque executado.

**Exercícios.**

1. *(Conceito)* Uma base tem uma linha por consulta médica, e a pergunta é sobre pacientes. Que faixa deve
   barrar a análise, e com que teste?
2. *(Conceito)* Desenhe o ataque da banca para um modelo de triagem de crédito. Que crise violaria a
   cláusula de saída? Que grupo passaria a pagar se a métrica fosse invertida?
3. *(Conta)* A ficha completa tem menor severidade 8 e piso 6. Qual é o número máximo de agentes que podem
   cair para a faixa baixa (2 a 4) sem reprovar a ficha? Justifique com a definição do $\bigwedge$.
4. *(Laboratório)* Implemente a calibração por simulação (SBC) para um modelo Beta-Binomial: sorteie o
   parâmetro da priori, simule os dados com ele e rode o amostrador. Verifique que o posto do valor
   verdadeiro entre as amostras da posterior é uniforme. Depois introduza um erro no amostrador e veja o
   histograma deixar de ser plano.
5. *(Conceito)* Dê um exemplo de cada um dos três modos de piora de um modelo em uso — deriva de covariável,
   deriva de rótulo e deriva de conceito — e da quarta situação, que não é piora. Diga a resposta certa para
   cada uma.
6. *(Conceito)* Por que a resposta do Sistema 1 precisa ser determinística (mesma entrada, mesma
   probabilidade)? Relacione com a semente fixa do Capítulo 1.

**Resumo.**

- Oito faixas, cada uma ligada a um modo de errar, mais um orquestrador que ataca; os agentes trocam fichas
  estruturadas, nunca texto livre, e ninguém aprova o próprio trabalho.
- O ciclo tem seis estágios, e o laço de volta vai para o estágio que falhou, não para o começo.
- A aprovação é um "e" lógico das oito severidades contra o piso 6; cada cuidado ausente derruba exatamente
  o agente que o cobra.
- Sistema 1 triagem determinística com probabilidades entre 1% e 99%; Sistema 2 delibera sobre o
  hipergrafo.
- Um artefato com faixa fraca declarada vale mais que um artefato uniforme sem lacuna admitida.
