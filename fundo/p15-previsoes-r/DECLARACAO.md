# Declaração do ponto 15: as previsões 3 e 5 do eixo R, medidas

**Congelada em 2026-10-16T22:17:44Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `DECLARACAO.md` (esta) | — | as cinco declarações; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `1f97cd7c0b79ade4f264f2e1b08588602e27968f454c64d7b5ff534e125a7c91` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os dois hashes, em forma de `sha256sum` |

## Declaração 1 — o critério, e onde ele estava escrito

As duas previsões estão em `book/horizontes-de-resolucao-da-pergunta.md` desde 2026-10-16, escritas **antes** de
existir instrumento para elas:

> 3. **Limiar de oscilação.** Existe um ponto a partir do qual uma decisão tomada em R0 falha em mais de um
> quarto dos dias, e abaixo dele refinar deixa de compensar.
>
> 5. **Transferência.** Um resultado que sobrevive a R1 sobrevive a R0; o contrário não é verdadeiro.

O documento fixa o **quarto** (mais de 25% dos dias) e não define o que é um dia falho. O critério de dia falho
fica declarado aqui, antes da segunda especificação: **o dia falha quando mais de 10% dos clientes esperam mais de
30 min** — o mesmo limiar que o Capítulo 16 usa para falar de dias ruins. A escala de R0 é dimensionada pela
fórmula de Erlang C com meta de 80% (no máximo 20% dos clientes entram em fila), que é o que o H0 faz.

## Declaração 2 — as duas especificações do plano de R0

**Especificação 1.** A escala de R0 é a do dia médio, pela fórmula de Erlang C (meta de 80%). É a leitura natural
de perguntar pelo dia médio, e a que o instrumento do eixo já usava.

**Especificação 2.** Depois de ver a primeira, a escala de R0 passou a ser **calibrada na demanda plana**: o menor
número de agentes em que a demanda do dia médio, sem oscilação nenhuma, faz falhar no máximo 10% dos dias. A
calibração entrou porque a especificação 1 deixa a previsão **impossível de medir**: com 12 agentes a demanda plana
já falha 1 dia em 5 (0,205), então o ponto a partir do qual falha mais de um quarto não marca nada sobre a
oscilação — ele marca o plano.

Isso é **especificação nova**, não conserto, e entra no **K desta frente: K = 2**. As duas ficam medidas e
publicadas lado a lado; a primeira não é reescrita, e o resultado dela é negativo.

## Declaração 3 — os números das duas especificações

~~~bash
cargo run --release --bin labs -- --resolucao
~~~

Desenho comum às duas: 200 dias por amplitude, 10 h por dia, demanda média de 8 chegadas por hora, o dia ímpar
forte e o par fraco, semente 1909, 12 ou 13 agentes. A perda é a espera média do agregado menos a do fino, em
minutos.

**Especificação 1 (plano de Erlang, 12 agentes).**

| amplitude | dias falhos | fração | perda (min) |
| --- | --- | --- | --- |
| 0,0 | 41 | 0,205 | −0,26 |
| 0,5 | 52 | 0,260 | −1,89 |
| 1,0 | 51 | 0,255 | +1,97 |
| 1,5 | 64 | 0,320 | +2,60 |
| 2,0 | 66 | 0,330 | +2,27 |
| 2,5 | 73 | 0,365 | +7,04 |
| 3,0 | 80 | 0,400 | +8,93 |

Cruzamento em 0,5, com perda de **−1,89 min**, e a fração **não é monótona** (0,260 em 0,5 contra 0,255 em 1,0).
**A previsão 3 não se sustenta nesta especificação.**
**Especificação 2 (plano calibrado, 13 agentes).**

| amplitude | dias falhos | fração | perda (min) |
| --- | --- | --- | --- |
| 0,0 | 19 | 0,095 | −2,24 |
| 0,5 | 30 | 0,150 | −4,08 |
| 1,0 | 33 | 0,165 | −0,66 |
| 1,5 | 39 | 0,195 | −0,65 |
| 2,0 | 42 | 0,210 | −1,12 |
| 2,5 | 53 | 0,265 | +2,87 |
| 3,0 | 66 | 0,330 | +3,36 |

Monótona, cruzamento em **2,5**, perda no cruzamento de **+2,87 min**, e **perda negativa em todas as amplitudes
abaixo dele** (−2,24 a −0,65 min). As duas metades da previsão se sustentam: o ponto existe, e abaixo dele refinar
não paga — no desenho medido, refinar chega a **custar** 4,08 min de espera média, porque o plano grosso fica
sobredimensionado nos dias fracos.

## Declaração 4 — a previsão 5

Seis perfis de demanda (média 8, amplitudes de 0,5 a 3,0), 200 dias cada, com o plano de R0 do dia médio:

| o que | valor |
| --- | --- |
| perfis em que R1 sobrevive e R0 não (**violações da previsão**) | **0** |
| perfis em que R0 sobrevive e R1 não (**o contrário, que a previsão diz que existe**) | **3** |
| conservação do agregado conferida (a soma dos dias reconstrói o agregado) | sim |
| limiar do dia | 0,10 |

**A previsão 5 se sustenta.** E ela é um teorema, não um acidente: o agregado é a média ponderada dos dias, então
se todo dia fica abaixo do limiar o agregado também fica. O instrumento confere essa conta em cada perfil
(`conservacao_do_agregado`), reprova um agregado que não bata com a soma dos dias, e conta **quantos** perfis
provam que o contrário não vale — três, no desenho medido.

## Declaração 5 — os limites

1. **É simulação declarada.** A fila é a de um servidor por agente, serviço exponencial de média 1 h, 10 h por dia,
   200 dias, um único perfil de média (8 chegadas por hora). O limiar de 2,5 é uma propriedade **deste** desenho, e
   não uma constante do mundo.
2. **A previsão 3 continua precisando do que o documento pedia:** uma operação real com a mesma decisão tomada em
   dois grãos. Aqui ela ficou medida em laboratório, que é menos do que isso.
3. **A perda negativa abaixo do limiar não estava prevista** e é o achado mais interessante da rodada: refinar pode
   custar quando o plano grosso é sobredimensionado nos dias fracos. Fica registrado como leitura, sem previsão nova.
4. **O critério de dia falho foi declarado nesta frente**, e não no documento do eixo: quem for comparar com outro
   número precisa usar o mesmo (mais de 10% acima de 30 min).

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-core` | 59 testes aprovados, entre eles os seis das previsões: o limiar com o plano calibrado, o plano de Erlang que reprova a previsão 3, a transferência sem violação e com contraexemplo, a conservação que reprova um agregado que não bate e a checagem da implicação (0) |
| `labs --resolucao` | a perda de resolução dos três casos do documento (46,24 / 5,97 / 0,00 min), a previsão 3 nas duas especificações e a previsão 5 (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` atualizado, com o bloco do eixo R (0) |
