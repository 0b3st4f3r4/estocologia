# Ponto 18 — o eixo R nos 21 capítulos

**Estado: fechada. Cada capítulo tem a sua linha do eixo, e a linha é saída literal do instrumento.**

O eixo R existia fora do livro — `labs --resolucao` mede a perda de resolução, e o documento do eixo traz os sete
casos documentados —, mas não aparecia em nenhum Degrau 3. Agora aparece em todos os 21, na mesma forma: um
parágrafo curto, o comando citado e **uma linha** do instrumento.

A tabela `book/eixo-r.tsv` declara, por capítulo, um caso e uma frase editorial, e o instrumento **calcula** a
leitura: perda de 46,24 min onde a demanda oscila de 5 a 11 (capítulos 5, 16 e 20), 5,97 min onde oscila de 7 a 9
(8), 0,00 min onde não oscila (10 e 18), e o limiar de oscilação na amplitude 2,5 no capítulo 21. Os outros catorze
capítulos dizem o que são — a perda **não é medida ali**, porque não há no exemplo deles a mesma decisão em dois
grãos.

**A prova.** O conferidor de blocos do livro cobra cada bloco contra o instrumento, e passou de 27 para **48 blocos
conferidos, com zero divergente**. Nenhum número do eixo foi escrito à mão nos capítulos.

**Como reproduzir.**

~~~bash
cargo run --release --bin labs -- --resolucao
~~~

**O limite.** O eixo entra como linha, não como laboratório novo: o laboratório é um só, e a perda só é medida
onde existem as duas resoluções. O K desta frente é 1 — a atribuição de um caso a cada capítulo foi feita uma vez.
