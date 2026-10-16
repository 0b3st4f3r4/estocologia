# Ponto 16 — a tese da ponte em escala

**Estado: tese NÃO SUSTENTADA, caixa fechada, e o motivo escrito — falta gente.**

O piloto P01 deu **empate no teto**: os dois braços apontaram os 8 defeitos plantados e não deram nenhum alarme
falso nos 4 casos limpos, e a regra declarada antes diz que empate no teto conta como tese não sustentada. Esta
frente re-deriva esse resultado das notas do juiz cego (`fundo/p01-tese/juiz/notas.yaml`) — 8 de 8 nos dois
braços, 0 de 4 alarmes falsos, 0 pares discordantes — e **trava a tese** até existir o teste justo.

**O que falta, nomeado.** Um escolhedor de casos **de fora do programa**, que escolha antes de existir
resultado; dois analistas humanos por braço, com pelo menos um de fora; um juiz humano de fora e cego ao braço; e
no mínimo 12 casos mais difíceis que os da lista de Makin e Orban de Xivry (2019) — a lista que qualquer
estatístico competente procura, e por isso o piloto não discriminou.

**O portão.** Enquanto isso não existir, nenhuma linha do programa pode dizer que a tese está sustentada: o
instrumento reprova quem afirmar o contrário, e há teste para isso. O prazo da janela é **2027-01-14**; vencido
sem a cobertura mínima, a tese fica **não sustentada em definitivo** como veredito do programa.

**Como reproduzir.**

~~~bash
cargo run --release --bin labs -- --ponte
~~~

O instrumento lê o registro (`registro.tsv`) e as notas do juiz, imprime a re-derivação, o estado dos papéis e as
duas provas: um registro que diz a tese sustentada sem o teste justo é reprovado, e com os papéis designados e os
casos no mínimo o teste pode rodar — o status só muda com o resultado do juiz.
