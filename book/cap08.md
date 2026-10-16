## Capítulo 8. O termômetro do adivinho: calibração, resolução e o escore de Brier

**Objetivos.** Ao fim deste capítulo você deve saber por que nem toda métrica de acerto é honesta;
calcular o escore de Brier e decompô-lo em calibração, resolução e incerteza; explicar por que a
calibração se conserta depois e a resolução não; e decidir a tolerância de calibração pelo tamanho da
amostra, e não por um número redondo.

---

### Degrau 1 — O mundo real e o paradoxo

**Dois meteorologistas.** Numa cidade chove, em média, 20 dias em cada 100. O primeiro meteorologista,
chamado aqui de *preguiçoso*, diz todos os dias a mesma frase: *"chance de chuva: 20%"*. O segundo,
chamado de *especialista*, olha o céu e diz: *"hoje, 70%"*.

Quem é melhor?

Nenhuma das duas respostas prontas serve. O preguiçoso está, no longo prazo, **certo na média**. Em
cada 100 dias em que ele anunciou 20%, choveu cerca de 20. Ele é calibrado, e não serve para levar
guarda-chuva. Em cada 100 avisos dele, 80 dias secos e 20 molhados, exatamente a taxa do lugar. O
especialista **separa** os dias. Num dia em que ele diz 70%, chove muito mais do que num dia em que ele
diz 10%. De um lado, um previsor calibrado e sem resolução; do outro, um calibrado com resolução.

Medidos num mundo de 4 mil dias, os números são estes:

| Meteorologista | Escore de Brier | Erro de calibração (REL) | Resolução (RES) |
| --- | --- | --- | --- |
| Preguiçoso (0,20 sempre) | 0,1858 | 0,0021 | 0,0002 |
| Especialista | 0,1569 | 0,0012 | 0,0267 |

O preguiçoso quase não tem erro de calibração (0,0021) e não tem resolução nenhuma (0,0002). Ele
acerta a frequência e não distingue nada. O especialista tem erro de calibração ainda menor e uma
resolução **133 vezes maior**. É por isso que ele vale mais, e é isso que a taxa de acerto não veria.

**A pergunta que a taxa de acerto não responde.** Um terceiro meteorologista é avaliado por taxa de
acerto. Ele diz "vai chover" quando a chance passa de 50% e acerta 80% das vezes. Essa métrica não
distingue o previsor que acredita em 51% do que acredita em 99%. A ação é a mesma para os dois, e a
nota também. Uma métrica assim não pune quem mente sobre a crença declarada, e por isso não serve para
auditar ninguém.

---

### Degrau 2 — A regra do jogo

**Regra de pontuação própria.** Uma regra de pontuação é **própria** (ou *estritamente própria*) quando
quem realmente acredita em $q$ minimiza a penalidade esperada declarando essa probabilidade. A regra mais
conhecida é o **escore de Brier**:

$$\text{BS} = \frac{1}{n}\sum_{i=1}^{n}(p_i - y_i)^2$$

com $p_i$ a probabilidade declarada e $y_i$ o desfecho (1 se aconteceu, 0 se não). É a média do erro
quadrático da probabilidade, e é essa propriedade que faz dela uma métrica em que **mentir sobre a
crença sempre piora a nota**. A taxa de acerto não é própria.

**Passar na média não é passar.** A checagem preditiva de Box, Gelman e Shalizi compara os dados reais com réplicas do modelo ajustado. Um Poisson ajustado à demanda da central passa na média e falha na variância. A razão variância/média observada é **273**, e o modelo subestima em dezenas de vezes a chance de um dia de pico (**180 contra 2 900** chamadas, `h0-real/README.md`). Um modelo que acerta o valor médio e erra a dispersão erra o dia que importa.

**A decomposição de Murphy.** Murphy (1973) separou o Brier em três partes:

$$\text{BS} = \underbrace{\text{REL}}_{\text{erro de calibração}} -
\underbrace{\text{RES}}_{\text{resolução}} + \underbrace{\text{UNC}}_{\text{incerteza dos eventos}}$$

- **REL** (calibração): quanto as probabilidades declaradas se afastam das frequências observadas.
  Zero significa "quando digo 30%, acontece em 30% das vezes".
- **RES** (resolução): quanto as frequências observadas se afastam da taxa-base. Zero significa "todos
  os meus avisos são o mesmo aviso".
- **UNC** (incerteza): $\bar y(1-\bar y)$, o que o desfecho tem de imprevisível para quem só conhece a
  taxa-base.

A identidade fecha quando as previsões são agrupadas em caixas (dez caixas, por exemplo) e o
Brier é calculado sobre a média de cada caixa. O **ECE** (*expected calibration error*) é o parente
não quadrático de REL, a média das distâncias absolutas entre previsto e observado por caixa.

**A assimetria prática: a calibração se conserta depois, a resolução não.** A **recalibração de Platt**
ajusta dois números, $a$ e $b$, e transforma $p$ em $\sigma(a + b\,\text{logit}(p))$. Ela conserta
distorções monótonas, um previsor sistematicamente confiante demais ou sistematicamente frouxo, e é
por isso que ela é barata. O que ela **não** faz é criar informação. Um previsor ruidoso continua
ruidoso depois de recalibrado, porque o ruído não estava na escala, estava no sinal.

Medido no laboratório, um previsor confiante demais sai de Brier 0,1712 para **0,1559**, praticamente o
valor do especialista honesto (0,1560), e toda a diferença era distorção de escala. Um previsor ruidoso
sai de 0,2119 para **0,1755**, uma melhora que ainda não alcança o especialista. O preguiçoso
recalibrado dá **0,1840**, que é o UNC do mundo. Recalibrar um previsor constante devolve só a
taxa-base.

**Quanto a calibração pode ser cobrada.** A tolerância do ECE é o ponto prático. A regra do tipo
$\mathrm{ECE} \le 0{,}03$ parece austera, mas não é. O ECE de um previsor **perfeitamente calibrado**
não é zero, e sim um número aleatório cuja escala anda com $1/\sqrt n$. A tolerância honesta é o quantil
95% desse nulo, calculado com essas previsões no $n$ do cofre, e ele encolhe quando $n$ cresce. Num
cofre de 1 488 casos o ruído é de 0,03; num de 104 489, de 0,0035. Uma tolerância fixa é, então, duas
coisas. **Apertada demais** nos cofres pequenos, onde reprova um previsor honesto, e
**frouxa demais** nos grandes, onde deixa passar descalibração perfeitamente detectável.

O H3 mediu isso em seis cofres binários:

| cofre | n | ECE observado | q95 do nulo | tolerância fixa 0,03 |
| --- | --- | --- | --- | --- |
| voos NY 2013 | 84 301 | 0,0267 | 0,0046 | passa |
| ciclones | 5 766 | 0,0123 | 0,0087 | passa |
| prisões Toronto 1997–2002 | 1 488 | 0,0363 | 0,0320 | falha |
| prisões Toronto 2020–2021 | 16 933 | 0,0668 | 0,0115 | falha |
| voos NY 2023 | 104 489 | 0,0079 | 0,0035 | passa |
| Rhode Island 2013–2015 | 24 466 | 0,0048 | 0,0034 | passa |

Aplicada a tolerância que depende de $n$, **nenhum** cofre binário do H3 passa, nem os três que a
regra fixa aprovava. O caso de Toronto 1997–2002 é o espelho. Ali o ECE de 0,0363 era ruído da ordem
da tolerância, porque o q95 nulo é 0,0320, mas o valor observado passa dele, e a reprovação se mantém.

**A resolução é a parte difícil.** O fluxo de Rhode Island terminou com ECE 0,0048 e habilidade
$+0{,}019$, isto é, calibração boa e resolução baixa. Prison numa parada de trânsito é um evento raro e
pouco previsível pelas variáveis disponíveis, e o modelo compensa isso com Platt (intercepto e
inclinação) e com o disjuntor, que põe o reserva como previsão oficial em 12% do cofre. Uma previsão
pode ser honesta e pouco informativa ao mesmo tempo, e as duas coisas se reportam **separadas**, porque
a primeira se conserta e a segunda não.

**Pôr a calibração à prova exige cofre novo.** A correção de intercepto do H3b ($\delta \leftarrow
\delta + 0{,}002(y - p')$) foi desenhada depois de ver o desvio de 2013. Para ser honesta, ela precisou
de um cofre que ninguém tinha visto, os 435 mil voos de Nova York em 2023, no trimestre que havia
falhado. Passou, com CITL 0,0015 e ECE 0,008, mas o modelo sem correção também teria passado, porque
2023 não repetiu o desvio de 2013. Fica declarado que a correção melhorou a calibração e **não foi
decisiva** para o portão.

**Duas ferramentas que vêm de graça.** A **predição conforme** dá cobertura garantida sem supor a forma
da distribuição, $P(Y \in C(X)) \ge 1-\alpha$, e foi a que o H3 usou em versão adaptativa sob deriva.
Uma regra vale para as duas. Toda previsão é pontuada por regra própria **fora do ajuste**, e nenhuma
previsão declara 0% ou 100%.

---

### Degrau 3 — Mão na massa

O laboratório pontua quatro previsores, recalibra todos com Platt e mede a tolerância do ECE em três
tamanhos de cofre. O código está em `crates/stochology-book/src/cap08.rs`, e a saída foi medida com
`cargo run --release`:

```text
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
```

Cinco leituras. **(1)** A identidade de Murphy fecha até $3{,}5\times10^{-15}$ quando as previsões são
agrupadas nas dez caixas; a previsão crua tem Brier 0,1569, e o agrupamento custa 0,0015. **(2)** O
preguiçoso tem REL 0,0021 e RES 0,0002. Está calibrado e continua inútil, e a recalibração de um
previsor constante devolve exatamente o UNC (0,1840). **(3)** O confiante demais cai de 0,1712 para
0,1559, abaixo do especialista (0,1560), porque era só escala. O ruidoso cai de 0,2119 para 0,1755 e
**não alcança** o especialista, porque era sinal, e sinal perdido não volta. **(4)** A tolerância de
0,03 é reprovada por um previsor honesto em **45,1%** das simulações quando $n = 1\,488$, e em **0,0%**
quando $n$ é 24 466 ou 104 489. Nos cofres grandes ela não proíbe quase nada. **(5)** A nula depende das
previsões, não só de $n$. Com $n = 1\,488$, o q50 do ECE nulo vai de 0,0320 com previsões concentradas
a 0,0243 com previsões espalhadas.

**Declaração de divergência.** A edição anterior registra, para um exemplo de 4 000 eventos, um previsor
confiante demais que vai de Brier 0,1956 a 0,1851 com Platt, um perito honesto em 0,1846 e um previsor
ruidoso que vai de 0,2154 a 0,2089. Este instrumento mede **0,1712 → 0,1559**, **0,1560** e
**0,2119 → 0,1755**. O mundo não é o mesmo — aqui a taxa-base observada é 0,243, contra os 0,5 do
exemplo antigo —, e a ordem das grandezas é a mesma: a recalibração leva o confiante ao nível do
honesto e deixa o ruidoso atrás. A tolerância do ECE também diverge em nível: o H3 publica q95 nulo de
0,0320 para $n = 1\,488$, 0,0034 para 24 466 e 0,0035 para 104 489; aqui saem **0,0425**, **0,0043** e
**0,0037**. A diferença é a distribuição das previsões, que o README não publica — com previsões
concentradas ($n = 1\,488$, espalhamento 0,4) o q50 nulo deste instrumento é 0,0320, o número do
relatório. O que interessa não muda: o ruído do ECE anda com $1/\sqrt n$, e por isso uma tolerância fixa
é apertada num cofre pequeno e frouxa num cofre grande. Nenhum valor antigo foi apagado.

**O grão desta pergunta.** Todo capítulo tem uma decisão com grão, e o eixo R mede o que custa perguntar
grosso. A linha deste capítulo, pelo instrumento do eixo (`labs --resolucao`):

```text
capítulo  8 | perda de resolucao 5.97 min (demanda de 7 a 9) | a tolerância de calibração depende do n do cofre: a oscilação de 7 a 9 é o caso leve
```

**Exercícios.**

1. *(Conceito)* Por que a taxa de acerto não é uma regra própria? Dê um exemplo de dois previsores
   com a mesma taxa de acerto e escores de Brier muito diferentes.
2. *(Conta)* Um previsor declara 0,30 em 200 dias e chove em 48 deles. Calcule REL e RES naquela
   caixa, supondo taxa-base 0,24, e diga o que aquele previsor deveria fazer com a escala.
3. *(Conta)* O ECE nulo anda com $1/\sqrt n$. Se o q95 nulo é 0,0320 com $n = 1\,488$, qual você
   espera com $n = 148\,800$? Compare com o q95 medido de 104 489 e diga se o crescimento bate com a
   previsão.
4. *(Laboratório)* No laboratório, troque a recalibração de Platt por uma simples mudança de
   intercepto ($\sigma(\text{logit}(p) + a)$, ajustando só $a$) e refaça as quatro linhas. Em que
   previsor a inclinação faz falta?
5. *(Laboratório)* Mude a semente do mundo de 2026 para 1909 e para 2027 e meça o Brier do
   especialista nas três. A que distância os três ficam, e o que isso diz sobre comparar dois
   previsores num mundo de 4 mil dias?
6. *(Conceito)* Escreva, em quatro linhas, o memorando que explica à diretoria por que o ECE de 0,0363
   de Toronto 1997–2002 não pode ser lido com a régua do ECE de 0,0079 dos voos de 2023.

**Resumo.**

- Só uma regra própria pontua crença: o escore de Brier pune quem mente sobre o que acredita.
- $\text{BS} = \text{REL} - \text{RES} + \text{UNC}$, e a identidade fecha exatamente com as previsões
  agrupadas em caixas.
- Calibrado e sem resolução é o preguiçoso: certo na média e inútil em cada dia.
- A calibração se conserta depois (Platt); a resolução, não: informação perdida não volta.
- A tolerância da calibração se calcula pelo tamanho da amostra: 0,03 é duro demais em mil e quinhentos
  casos e frouxo demais em cem mil.
