## Capítulo 4. Sorte, ignorância e o peru de Natal: os três tipos de incerteza

**Objetivos.** Ao fim deste capítulo você deve saber distinguir incerteza aleatória, epistêmica e
knightiana, usar a lei da variância total para separá-las, situar um problema na matriz de Stirling e
reconhecer um modelo que envelhece em silêncio.

---

### Degrau 1 — O mundo real e o paradoxo

**O peru de Natal.** Um peru é alimentado todos os dias, pontualmente, às 9 horas da manhã. No dia 1,
ele não sabe nada. No dia 10, desconfia que a comida tem algo a ver com o galinheiro. No dia 60, já tem
uma teoria. No dia 100, ele faz a conta dos seus dias: *em 100 manhãs, a comida chegou em 100*. Na
véspera do dia 101, o peru está **99% confiante** de que amanhã, às 9 horas, a comida chega.

O dia 101 é o Natal. E o Natal nunca esteve na lista de hipóteses do peru. Nenhum dado a mais o teria
salvo: **mais observações só aumentaram a confiança no lugar errado.** (Bertrand Russell contou a
história com uma galinha; Nassim Taleb a recontou com o peru.)

**Em cada 100 experimentos.** Imagine dois tipos de incerteza convivendo:

- **A moeda.** Você joga uma moeda honesta 100 vezes. Em cada 100 experimentos de 100 lances, a
  proporção de caras fica perto de 50%, mas nunca exatamente 50%. Jogar mais vezes **não** faz a
  proporção virar exatamente 50%: o sorteio continua sorteio.
- **A moeda desconhecida.** Você joga uma moeda sem saber se é honesta. Depois de 10 lances, ainda não
  sabe. Depois de 10.000 lances, sabe quase com certeza. Aqui, mais dados resolvem.

Esses dois casos têm nomes diferentes, e confundi-los custa caro. Existe ainda um terceiro, o do peru.

---

### Degrau 2 — A regra do jogo

**A lei da variância total.** A dispersão total de um resultado se decompõe em duas parcelas:

$$\operatorname{Var}(Y) = \underbrace{\mathbb{E}_\theta\left[\operatorname{Var}(Y \mid \theta)\right]}_{\text{aleatória (Mundo)}} + \underbrace{\operatorname{Var}_\theta\left(\mathbb{E}[Y \mid \theta]\right)}_{\text{epistêmica (Modelo)}}$$

A primeira parcela é o sorteio: existe mesmo com o parâmetro conhecido. A segunda é a ignorância sobre o
parâmetro: desaparece quando o parâmetro é identificável e você tem dados.

| Incerteza | Camada | O que mais dados fazem | Como se mitiga |
| --- | --- | --- | --- |
| Aleatória | Mundo | Nada | Margens de segurança, redundância |
| Epistêmica | Modelo | Reduzem até zero, se o parâmetro for identificável | Amostragem, atualização bayesiana |
| Knightiana | Modelo e Agente | Nada — ou pior: falsa confiança | Cenários, robustez, estresse de premissas, ampliar o espaço de hipóteses |

Frank Knight (1921) separou o **risco**, de probabilidades conhecidas, da **incerteza**, de
probabilidades que não se podem medir. No caso extremo — que este livro chama de incerteza knightiana
ou erro de especificação — o processo real está **fora** do espaço de hipóteses. É a mais perigosa,
porque não aparece na equação: o intervalo estreita em volta da resposta errada.

**A matriz de Stirling.** Duas perguntas organizam o diagnóstico: sabemos quais desfechos são
possíveis? Sabemos suas probabilidades?

|  | Probabilidades conhecidas | Probabilidades desconhecidas |
| --- | --- | --- |
| **Desfechos bem definidos** | Risco: calcular | Incerteza: cenários, sensibilidade |
| **Desfechos mal definidos ou disputados** | Ambiguidade: deliberar sobre o que conta como dano | Ignorância: vigilância, reversibilidade, diversidade de opções |

O peru de Russell está no canto de baixo à direita, e nenhum cálculo do canto de cima à esquerda o
salva.

**A tipologia de Walker e colegas (2003).** Cada incerteza se classifica em três dimensões. A
*localização* diz onde ela entra: contexto, estrutura do modelo, parâmetros, entradas ou resultados. O
*nível* vai da certeza determinística à ignorância total. A *natureza* separa variabilidade (aleatória)
de falta de conhecimento (epistêmica). Na arquitetura deste livro, a localização corresponde à camada e
ao campo da ficha (Capítulo 14), e o nível diz se basta uma margem ou se é preciso um teste de estresse.

**A regra de sucessão de Laplace.** Na véspera do dia 101, o peru usa:

$$P(\text{sucesso amanhã}\mid s,n)=\frac{s+1}{n+2}$$

A regra também não é neutra, porque deriva de uma priori uniforme: com sete sucessos em dez, a uniforme
dá 0,667, uma uniforme sobre $\theta^2$ dá 0,692 e uma sobre $\sqrt{\theta}$ dá 0,652. **Até a
ignorância tem forma**, e é por isso que a escolha de priori precisa ser declarada (Capítulo 13, faixa
F7).

---

### Degrau 3 — Mão na massa

O laboratório põe um mundo que muda debaixo de um modelo que não muda. Uma moeda cujo vício **sobe de
0,2 para 0,6** ao longo de 6.400 lances, 200 repetições, com o modelo 1 supondo moeda fixa e o modelo 2
descontando o passado com fator 0,99. Código em `crates/stochology-book/src/cap04.rs`; saída
medida com `cargo run --release --bin labs -- --cap 4`:

```text
== Capítulo 4 :: a moeda cujo vício muda ==
vies sobe de 0.2 a 0.6 em 6400 passos, 200 repeticoes, desconto 0.99
cobertura do intervalo de 95%: modelo fixo = 0.1199, modelo com esquecimento = 0.9398
largura do intervalo fixo no passo 100 = 0.1564, no passo 6400 = 0.0240
largura do intervalo com esquecimento no passo 6400 = 0.1360 (razao 5.7x o fixo final)
```

O modelo fixo declara "95% de confiança" e **cobre a verdade em 12% dos passos**: o intervalo encolhe de
0,1564 para 0,0240 enquanto o vício caminha para fora dele. O modelo que esquece o passado, com um
intervalo **5,7 vezes mais largo**, cobre 93,98% — perto do nominal. O preço da honestidade é a largura
do intervalo; o preço da teimosia é a confiança falsa.

**Declaração de divergência.** A edição anterior deste livro (e o roteiro do curso) registra 11% de
cobertura para o modelo fixo e 99% para o modelo com esquecimento, com larguras de 0,140 a 0,035. Este
instrumento, com 6.400 passos e desconto 0,99, mede **12%** e **93,98%**, com larguras de 0,1564 a
0,0240 e razão de 5,7×. A cobertura do modelo fixo coincide dentro de um ponto percentual; a do modelo
com esquecimento é menor porque a largura do intervalo usado aqui é a do erro-padrão do EWMA, e um
intervalo mais largo (ou um desconto menor) eleva a cobertura acima de 97% — a grade de parâmetros em
`crates/stochology-book/examples/grade_cap04.rs` mostra o efeito. Os dois valores ficam
registrados; nenhum foi apagado.

**Exercícios.**

1. *(Conceito)* Classifique cada incerteza: o resultado do próximo lance de uma moeda; o vício
   verdadeiro dessa moeda; a chance de o governo mudar a regra do jogo no ano que vem; o erro de
   medição de uma balança calibrada.
2. *(Conta)* Aplique a regra de Laplace com 7 sucessos em 10 tentativas e compare com as duas priores
   alternativas (0,667, 0,692, 0,652). Qual das três você declararia antes de ver os dados?
3. *(Conta)* Quantos desvios-padrão de distância tem o vício verdadeiro no passo 6.400, se o intervalo
   do modelo fixo tem largura 0,0240 e o vício está em 0,6 enquanto a estimativa fixa aponta para perto
   de 0,4?
4. *(Laboratório)* Mude o desconto de 0,99 para 0,98 e para 0,995 no laboratório. Explique, olhando as
   duas coberturas e as duas larguras, o compromisso entre janela de memória e cobertura.
5. *(Conceito)* Descreva um caso real (da sua área) em que o processo mudou de regime e o modelo
   continuou confiante. Em que canto da matriz de Stirling ele cai?

**Resumo.**

- Incerteza aleatória não encolhe com mais dados; epistêmica encolhe; knightiana engana.
- A lei da variância total separa as duas primeiras com precisão.
- A matriz de Stirling e a tipologia de Walker organizam o diagnóstico antes do cálculo.
- Um mundo que muda devagar derrota um modelo fixo em silêncio: a confiança nominal sobrevive ao erro.
