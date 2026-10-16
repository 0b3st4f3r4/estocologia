# Regra de escolha mecânica dos casos (ponto 16)

**Congelada em 2026-10-16, antes de existir lista.** Quem escolhe não é ninguém: a escolha é uma função de (a) uma
lista externa, (b) o filtro de elegibilidade e (c) a semente declarada. Quem aplica a função é o instrumento, e o
resultado sai antes de existir analista.

## 1. A lista externa

Um catálogo externo de casos publicados, com identificador e endereço, fixado e com hash registrado **antes** da
escolha. Fonte aceita: catálogos públicos de falha de replicação e de correção publicada — o projeto de replicação
da Open Science Collaboration, os registros de replicação registrada, e as correções e retratações de catálogo
público. **O autor do programa não toca na lista:** qualquer edição dele contamina a exogeneidade e invalida o
teste. A lista entra com hash; se a lista mudar depois, o teste é outro e a declaração é outra.

## 2. O filtro de elegibilidade, objetivo e declarado antes

- **Caso com defeito:** o catálogo documenta um erro de julgamento estatístico que passou pela revisão por pares —
  falha de replicação, correção publicada ou retratação — e o erro **não** é fraude declarada.
- **Caso limpo:** o catálogo documenta replicação bem-sucedida ou resultado confirmado de forma independente.

O filtro não usa nada do programa: nem a lista de Makin e Orban de Xivry de 2019, nem os capítulos, nem os
domínios do repositório. Um caso só entra se o catálogo externo o classifica, e a classificação fica citada na
ficha do caso.

## 3. O sorteio

Com a lista filtrada fixada e com hash, a escolha é a permutação da lista pela semente e os primeiros da ordem
sorteada, na proporção declarada: **8 casos com defeito e 4 limpos**, sorteados dentro de cada grupo. Semente
declarada e registrada antes: **20270114**. O instrumento aplica a função (`labs --ponte`), e o resultado é o
arquivo dos 12 casos.

## 4. A ordem não diz nada

O embaralhamento é o mesmo sorteio, e a numeração dos casos sai dele. O número do caso não informa se há defeito,
como no piloto.

## 5. O que invalida o teste

Lista editada depois da escolha; caso trocado à mão; o autor vendo o gabarito antes dos analistas; analista lendo
o gabarito ou os outros casos; lista que não sustenta a proporção pedida (por exemplo, menos de 8 casos com
defeito elegíveis) — nesse caso o instrumento **reprova** a escolha em vez de completar com o que houver.

## 6. Mudança de critério

Mudança de fonte, filtro, semente ou proporção é **declaração nova**, com hash próprio, e conta no K do programa.
