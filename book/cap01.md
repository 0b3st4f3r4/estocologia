# Parte I. A arte de mirar no escuro

## Capítulo 1. O alvo invisível: o que é a Estocologia?

**Objetivos.** Ao fim deste capítulo você deve saber dizer a que se dedica a Estocologia, de onde vem
o nome, quais três tradições ela costura e por que a conferência de um resultado depende de o sorteio
ser repetível.

---

### Degrau 1 — O mundo real e o paradoxo

**A decisão antes da resposta.** Numa terça-feira de manhã, uma médica atende um homem de 54 anos com
dor no peito, e não sabe como aquele caso vai terminar. Ela tem em mãos um eletrocardiograma, a idade
dele, o histórico, e quinze minutos. A decisão é entre internar e mandar para casa. Se ela internar, o
hospital gasta um leito que faltará para outro; se ela mandar para casa, e o pior acontecer, ninguém
vai esquecer o nome dela.

Repare no que acaba de acontecer. **A decisão foi tomada antes de a resposta existir.** Isso não é
descuido do sistema de saúde. É a condição de praticamente toda decisão que importa. O juiz decide sem
ter visto o crime. O gestor contrata pessoal para uma demanda que ainda não chegou. Você escolhe um
curso antes de saber se vai gostar dele.

Chamamos de **Estocologia** a ciência (e a arte) que estuda esse tipo de julgamento: **o julgamento e
a decisão sob incerteza**.

**Em cada 100 decisões.** Para tornar a conversa concreta, o livro inteiro usa frequências naturais.
Em vez de "a probabilidade de erro é de 9%", diga: *em cada 100 pacientes como este, cerca de 9
recebem o diagnóstico errado*. Em vez de "a prevalência é de 1%", diga: *em cada 1.000 pessoas que
entram na fila, 10 estão doentes*. E, em vez de "o intervalo de confiança cobre a verdade em 95% dos
casos", diga: *em cada 100 experimentos repetidos, 95 dão um intervalo que contém o valor real*. Os
números não mudam, mas o segundo jeito de dizer já mostra quem paga a conta.

**De onde vem o nome.** *Stókhos* é a palavra grega para o alvo onde se mira, e também para a
conjectura bem orientada, o palpite de quem mira no que não vê. Em 1713, três anos depois de sua
morte, foi publicado o *Ars Conjectandi*, de Jacob Bernoulli: a arte de conjecturar, que ele chamou de
*stochastice*. Com o tempo, "estocástico" passou a significar apenas "aleatório", e perdeu justamente
a parte interessante, **mirar bem quando não se vê o alvo**. Este livro quer recuperar esse sentido.

**A parte "arte".** A definição diz "ciência (ou arte)" de propósito. Há uma parte do julgamento sob
incerteza que se formaliza, e o livro inteiro mostra como: a coerência das crenças (Capítulo 3), a
atualização pela evidência (Capítulo 3), a margem de erro na unidade certa (Capítulo 6), a
calibração (Capítulo 8), o custo do erro (Capítulo 11). E há uma parte que não se formaliza por
completo: qual modelo vale a pena levar a sério, quando parar de coletar dados, quem paga se o erro
acontecer. Uma disciplina honesta sobre incerteza precisa ser honesta sobre os limites do próprio
formalismo. É por isso que todo capítulo deste livro, no fim, declara onde pode falhar.

**Por que dois computadores precisam sortear os mesmos números.** Um experimento científico precisa
ser **repetível**, e outra pessoa, em outra máquina, no ano que vem, tem de obter resultado idêntico.
Se o programa sorteia números ao acaso a cada execução, nada se pode conferir, e cada rodada dá uma
resposta diferente; a discussão vira "no meu computador deu outro valor".

A solução é simples, e **a semente** é a base de todo laboratório deste livro. Um gerador de números
pseudoaleatórios é uma receita determinística. Dada a mesma semente de partida, a receita produz a
mesma sequência de números, sempre, em qualquer máquina. O "acaso" do experimento é um acaso
*reprodutível*, aleatório para quem olha o resultado e auditável por quem sabe a semente.

---

### Degrau 2 — A regra do jogo

**Definição.** A Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.
Formalmente, ela estuda como um agente transforma informação incompleta em uma ação e responde pelas
consequências.

**A costura de três tradições.** A disciplina costura três tradições que cresceram separadas e
raramente se citam; não é um ramo novo da estatística.

- **A filosofia da indução.** Hume mostrou que toda defesa da indução pressupõe que o futuro se
  parecerá com o passado. Popper trocou a confirmação pela refutação. Lakatos mostrou como as teorias
  reagem quando a refutação chega.
- **A inferência estatística.** Pascal e Fermat, Bernoulli, Bayes, Laplace e Kolmogorov construíram a
  probabilidade; Fisher, Neyman e a escola bayesiana construíram a inferência.
- **A mecânica dos processos estocásticos.** Markov e Langevin estudaram como o acaso se acumula no
  tempo.

**Os ajustes de Lakatos.** Duhem e Quine lembraram que uma previsão frustrada refuta a teoria junto
com suas hipóteses auxiliares, nunca sozinha. Lakatos separou dois modos de salvar a teoria. O ajuste
**progressivo** prevê algo novo que depois se confirma; em 1846, a anomalia na órbita de Urano não
derrubou Newton, porque Le Verrier postulou um planeta oculto e Netuno foi encontrado. O ajuste
**degenerativo** apenas protege a teoria, e a manobra equivalente, com o planeta Vulcano, fracassou
ao explicar Mercúrio. Toda mudança de especificação depois de uma falha (e este livro relata várias)
precisa responder a uma pergunta. O conserto previu algo novo, ou só escondeu o erro?

**A pergunta que muda.** Em vez de perguntar "qual é a probabilidade?", a Estocologia formula outras
três. **Em que camada esta afirmação pode falhar? Que tipo de incerteza a ameaça? Que teste ela
teria de enfrentar para merecer confiança?** O resto do livro responde a essas três perguntas.

---

### Degrau 3 — Mão na massa

O laboratório deste capítulo não calcula nada de interessante, mas mostra *por que* tudo que vem
depois é conferível. O código está em `crates/stochology-book/src/cap01.rs`, e a saída abaixo
foi medida com `cargo run --release --bin labs -- --cap 1`.

```text
== Capítulo 1 :: o alvo invisível ==
duas maquinas com a semente 1713 concordam em 10000 de 10000 sorteios: true
sorteios coincidentes com a semente seguinte: 0
semente 1713: frequencia de caras em 100 mil jogadas = 0.5000
semente 1909: frequencia de caras em 100 mil jogadas = 0.4992
semente 2026: frequencia de caras em 100 mil jogadas = 0.4987
```

Leia linha por linha. **10000 de 10000**. Duas "máquinas" independentes, com a semente de partida em
comum, produzem sequências idênticas, e é isso que torna o experimento auditável. **0**. Quando a
semente muda, as sequências não têm nada em comum (zero coincidências em dez mil sorteios). A semente
não foi escolhida para forçar um resultado; ela é só o ponto de partida. **0,5000, 0,4992 e 0,4987**.
Três sementes diferentes, três moedas honestas, frequências de caras a menos de 0,0013 de um meio em
100 mil jogadas. O acaso, quando se acumula, se comporta.

**Exercícios.**

1. *(Conceito)* Explique, com suas palavras, por que a definição diz "ciência (ou arte)". Dê um
   exemplo de decisão em que a parte "arte" é inevitável.
2. *(Conceito)* Um jornal escreve: "o sorteio foi auditado e ninguém sabe o resultado antes da hora".
   Se o programa que faz o sorteio tem semente fixa, quem conhece a semente sabe o resultado? Por que
   isso não invalida o sorteio?
3. *(Conta)* Você repete o experimento e obtém 0,53 de caras em 100 jogadas. Em 100 mil jogadas,
   esperaria desvio parecido? Justifique com uma frase sobre como o acaso se comporta quando se
   acumula.
4. *(Laboratório)* Rode `labs --cap 1` e depois troque a semente de 1909 para 1909 no arquivo. O
   resultado muda? Troque para 1910 e meça quantas coincidências aparecem em 100 mil sorteios.
5. *(Conceito)* Cite uma decisão da sua vida em que a resposta só se conhece meses depois, e diga qual
   informação você gostaria de ter tido e não teve.

**Resumo.**

- Estocologia é a ciência (ou arte) que estuda o julgamento e a decisão sob incerteza.
- O nome vem do *stókhos* de Bernoulli (1713). Mirar bem quando o alvo não se vê.
- Ela costura filosofia da indução, inferência estatística e processos estocásticos.
- A conferência de um resultado depende de o acaso do experimento ser reprodutível. Isso exige semente
  fixa.
