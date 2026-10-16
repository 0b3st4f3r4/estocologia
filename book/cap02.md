## Capítulo 2. Mundo, Modelo e Agente: onde moram os erros?

**Objetivos.** Ao fim deste capítulo você deve saber separar as três camadas da realidade, enunciar a
tese da ponte, aplicar as três formas de inferência de Peirce, distinguir os três sentidos da palavra
"viés" e reconhecer a falácia do promotor.

---

### Degrau 1 — O mundo real e o paradoxo

**O território, o mapa e o motorista.** Você está num carro em uma cidade desconhecida. Existe a
cidade de verdade: ruas, buracos, obras, o caminhão parado na esquina. Existe o mapa, uma folha de
papel que representa a cidade, sempre incompleta e sempre desatualizada em algum ponto. E existe o
motorista, você, com fome, com pressa, com atenção dividida, decidindo a cada cruzamento.

Os três são coisas diferentes, e o mapa nunca é o território. Mas o motorista só age pelo mapa. Ali
nasce quase todo erro que este livro estuda. **Não no mapa, e sim na passagem do mundo para o
mapa, e do mapa para a mão do motorista.**

**A fila do posto de saúde.** Em cada 100 dias, o posto recebe pacientes de duas maneiras. Em metade
dos dias chegam 5 pessoas por hora; na outra metade, chegam 11. Em média, 8 pessoas por hora. O posto atende
12 por hora.

A alegação do gestor é esta: *"a demanda média é 8, o posto atende 12, então a espera é de cerca de 10
minutos e não há por que contratar".* A conta está certa para um mundo em que chegam exatamente 8
pessoas por hora, toda hora.

Só que o mundo é o outro. Simulados com a mesma semente, os dois mundos dão:

| Mundo | Espera média por paciente | Em cada 100 pacientes, quantos esperam mais de 30 min |
| --- | --- | --- |
| Sempre 8 por hora | 9,3 minutos | 8 |
| Metade dos dias com 5, metade com 11 | 20,7 minutos | 25 |

Nenhuma conta errou. A média da demanda é 8 nos dois casos. O **mundo** mudou. Quando a
demanda oscila, a fila do dia cheio cresce muito mais do que a fila do dia vazio encolhe, porque a
espera cresce **em curva**. Quem olhou só a média, com a frase "o paciente médio espera 10 minutos",
decidiu por um mundo que não existe. O Capítulo 5 explica a curva; o Capítulo 16 refaz o caso inteiro com os
números do repositório.

---

### Degrau 2 — A regra do jogo

**As três camadas.**

| Camada | Código | O que é | O que se audita nela |
| --- | --- | --- | --- |
| Mundo | $L_1$ | A realidade que gera os dados e que nunca vemos inteira | Física do ruído, mecanismos causais, dependência de trajetória, caudas pesadas |
| Modelo | $L_2$ | A representação formal, simplificada e simulável | Premissas ocultas, espaço de hipóteses, verossimilhança, sobreajuste |
| Agente | $L_3$ | Quem decide: pessoa, instituição ou algoritmo, com tempo e atenção limitados | Atalhos heurísticos, calibração, utilidade, laços em que a ação muda o Mundo |

As três formam um laço. O Mundo gera dados, o Modelo os interpreta, o Agente age, e a ação volta a
mudar o Mundo. Cada seta é uma **ponte** onde o erro entra. Os dados chegam filtrados entre Mundo e Modelo, a previsão
é mal lida entre Modelo e Agente, e a ação deforma o Mundo que se queria medir.

**A tese da ponte.** *Uma falácia, quase sempre, não é um erro de aritmética, e sim um desalinhamento
entre camadas.* Vale a pena dizer o que essa tese **não** afirma. Ela não diz que contas erradas não
existem, nem que quem usa a arquitetura não erra. Ela diz onde procurar primeiro.

**As três formas de inferência (Peirce).** Peirce obteve dedução, indução e abdução permutando as
mesmas três proposições: regra, caso e resultado.

| Forma | Vai de | Para | Preserva a verdade? |
| --- | --- | --- | --- |
| Dedução | Regra e caso | Resultado | Sim |
| Indução | Caso e resultado | Regra | Não: amplia e pode falhar |
| Abdução | Regra e resultado | Caso (a melhor explicação) | Não: amplia e pode falhar |

A abdução escolhe a hipótese que torna os dados menos surpreendentes, mas só entre as hipóteses
consideradas. É a precursora da máxima verossimilhança e herda o limite dela, que o Capítulo 4
chamará de incerteza knightiana.

**Os três sentidos de "viés".** A palavra é usada para três coisas que a disciplina separa para
depois reunir. São o viés estatístico de um estimador (Modelo), o viés do processo que produziu os
dados (Mundo) e o viés cognitivo de quem raciocina (Agente). Nos três casos o erro não é ruído que se
cancela com mais dados, e sim uma distorção que se acumula.

**A tese tem uma versão operacional.** A maior parte dos vieses do catálogo clássico corresponde a um
modelo de acaso que foi ignorado.

| Viés | Modelo de acaso ignorado |
| --- | --- |
| Negligência da taxa-base | A verossimilhança tomada pela posterior |
| Disponibilidade, sobrevivência | Uma amostra filtrada tomada pela população |
| Ilusão de padrão | A hipótese nula de acaso esquecida |
| Falácia do apostador | Memória atribuída a um processo sem memória |
| Negligência de eventos raros | Uma cauda pesada modelada como gaussiana |
| Excesso de confiança | Má calibração |
| Raciocínio motivado | O modelo escolhido pelo resultado desejado |

**A falácia do promotor.** O Agente lê a verossimilhança do Modelo, $P(\text{evidência} \mid
\text{inocente})$, como se fosse a resposta sobre o Mundo, $P(\text{inocente} \mid \text{evidência})$.
Os dois números são diferentes, e a diferença depende inteiramente da camada $L_1$, ou seja, de quantas
pessoas inocentes se está falando.

**O paradoxo da inspeção.** O Agente impõe um Modelo estável a um Mundo que oscila, como o gestor do
posto, que projetou a fila do dia médio para todos os dias.

**A tese foi testada uma vez.** Doze casos foram montados a partir de uma lista externa de dez erros
estatísticos comuns (Makin e Orban de Xivry, 2019). Oito casos tinham um defeito plantado e quatro
estavam limpos; dois braços independentes os analisaram às cegas, um com a arquitetura deste livro,
outro com a instrução de analisar com rigor, sem agentes nem portões. Os dois apontaram os 8 defeitos e deram zero
alarme falso nos 4 casos limpos. A regra fixada de antemão dizia que **um empate no teto não sustenta a
tese**, e foi isso que o resultado disse. O empate também não a refuta. Ele mostra que, para os dez erros
dessa lista, um analista competente acha tudo o que o outro acha. Se a tese vale, vale para erros que a
lista não cobre, e um teste justo disso precisa de casos mais difíceis, escolhidos fora do programa, de
analistas humanos e de um juiz humano. O Capítulo 21 volta a esse ponto.

---

### Degrau 3 — Mão na massa

Duas tarefas: representar as camadas como tipos de dados, para que o programa não possa confundi-las, e
medir a falácia do promotor em três mundos diferentes.

**As camadas como tipos.** Em Rust, a separação deixa de ser uma recomendação de estilo e passa a ser
uma regra do compilador (`crates/stochology-core/src/camada.rs`):

```rust
pub enum Camada { Mundo, Modelo, Agente }

pub enum TipoIncerteza { Aleatoria, Epistemica, Knightiana }
```

Quem escrever uma função que recebe `Camada::Modelo` não consegue passar um `Camada::Agente`
sem escrever essa conversão à mão, e a conversão à mão é o lugar onde o leitor do código vai parar
para pensar.

**A ponte medida.** O laboratório do capítulo pergunta o seguinte. Uma coincidência de DNA ocorre por
acaso em 1 pessoa em cada 1 milhão. Em três mundos diferentes, em que a chance prévia de o réu ser o
culpado é de 1%, de 0,1% e de 0,0001%, qual é a resposta correta sobre o réu?

```text
== Capítulo 2 :: a falácia mora na ponte ==
prevalencia 1e-2: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.0001  fator da troca=99.0
prevalencia 1e-3: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.0010  fator da troca=998.0
prevalencia 1e-6: P(evidencia|inocente)=0.00000100  P(inocente|evidencia)=0.5000  fator da troca=499999.7
```

A primeira coluna não muda nos três mundos, **0,000001**. É a verossimilhança, e é o número que o
promotor cita. A segunda coluna é a resposta. Com evidência e verossimilhança idênticas, a chance
de inocência vai de **0,0001** (0,01%) a **0,5** (50%). A terceira coluna mostra quanto a troca mente:
99 vezes no primeiro mundo, 499.999,7 vezes no último. O erro não está na conta; está em responder
sobre o Mundo com um número do Modelo.

**Exercícios.**

1. *(Conceito)* Classifique cada item como Mundo, Modelo ou Agente: a taxa real de chegada de
   pacientes; a fórmula de fila usada pelo gestor; a decisão de contratar dois enfermeiros; a
   desistência dos pacientes quando veem a fila.
2. *(Conceito)* Uma notícia diz: "a chance de o DNA coincidir por acaso é minúscula, logo a chance de o
   réu ser inocente é igualmente minúscula". Em que ponte está o erro? Que quantidade falta para chegar
   a $P(\text{inocente} \mid \text{evidência})$?
3. *(Conta)* No laboratório, com prevalência de 0,0001% ($10^{-6}$), o resultado é 0,5. Calcule à mão
   qual prevalência daria 0,09 e explique em uma frase por que a resposta depende tanto da camada $L_1$.
4. *(Laboratório)* Modifique o laboratório para usar uma coincidência de 1 em 1.000 em vez de 1 em
   1.000.000 e refaça a tabela dos três mundos.
5. *(Conceito)* Escolha três vieses da tabela da seção anterior e descreva, para cada um, um ambiente
   em que o atalho deixaria de ser erro (Capítulo 7 retoma essa ideia com Kahneman e Gigerenzer).

**Resumo.**

- Mundo ($L_1$), Modelo ($L_2$) e Agente ($L_3$) são camadas distintas, e a maior parte dos erros mora
  nas pontes entre elas.
- Peirce: dedução preserva a verdade; indução e abdução ampliam e podem falhar.
- "Viés" tem três sentidos, do dado, do estimador e da mente, e todos se acumulam.
- A falácia do promotor troca $P(E \mid I)$ por $P(I \mid E)$; a troca mente por um fator que depende
  da prevalência no Mundo.
