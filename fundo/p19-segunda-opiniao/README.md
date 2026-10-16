# Ponto 19 — a segunda opinião dos resultados marcados P

**Estado: fechada com o limite dentro. 5 linhas conferidas por segunda implementação, 2 com veredito re-derivado e 7
que só restam como registro.**

O registro empírico tem catorze linhas com a marca **P** — resultados que vieram do andaime Python aposentado, sem
instrumento em Rust que os medisse de novo. Esta frente dá a segunda opinião onde ela é possível: onde o dado ou o
registro em máquina ficou na árvore, uma segunda implementação refaz a conta por um caminho diferente e confere.

**As que conferiram.** Os limiares `α/M` do p03 são exatamente 0,05 dividido por M. O viés do previsor ingênuo do
laço (0,29124674633711956) aparece em **dois arquivos independentes** e concorda dígito por dígito. A base do banco
foi recontada em Rust e bate com o publicado, com uma diferença de 0,2% que é a janela de horas do laboratório —
atribuída e quantificada, não uma discordância. Os pareceres foram contados: 30 linhas na releitura e 4 arquivos.

**A divergência, declarada e não resolvida.** O registro publicado diz “33 dos 33 idênticos” pelo critério da banda
de 0,05 dp; o campo em máquina `reproducao_rodada_a` tem 33 domínios e marca `true` em **8**. O campo não declara o
próprio sentido, então a segunda opinião registra os dois valores em vez de adivinhar. A decisão é humana.

**As que não têm segunda opinião.** Sete linhas — as coortes, o Brier dos incêndios, o túnel de luz, a
farmacologia, o STAR, os seis fluxos e o hipergrafo — dependem de dado que saiu da árvore com o andaime. Ficam
declaradas, com o motivo, em vez de virarem promessa.

**Como reproduzir.**

~~~bash
cargo run --release --bin labs -- --segunda
~~~

**O limite.** Não há segunda opinião para o que não tem dado. O K da frente é 1.
