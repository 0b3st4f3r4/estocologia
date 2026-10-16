# Ponto 15 — as previsões 3 e 5 do eixo R

**Estado: medidas, com uma previsão que só se sustenta sob a segunda especificação do plano, e isso declarado.**

O documento do eixo (`book/horizontes-de-resolucao-da-pergunta.md`) escreveu cinco previsões falsificáveis em
2026-10-16. As previsões 1, 2 e 4 já tinham instrumento (`resolucao.rs`, o `labs --resolucao`). Esta frente mede
as duas que faltavam.

**Previsão 3 — o limiar de oscilação.** Depende de como o plano de R0 é dimensionado, e as duas leituras ficam
publicadas. Com a escala de Erlang C do dia médio (12 agentes), a previsão **não se sustenta**: a demanda plana já
falha 1 dia em 5, a fração não é monótona e no cruzamento refinar custa. Com a escala calibrada na demanda plana
(13 agentes), ela **se sustenta**: a fração cresce de 0,095 a 0,330, cruza o quarto na amplitude **2,5**, e a perda
de resolução é **negativa em todas as amplitudes abaixo do limiar** e vira **+2,87 min** no cruzamento. A segunda
escala é especificação nova, declarada, e entra no K da frente (K = 2).

**Previsão 5 — a transferência.** Sustenta-se: em 6 perfis × 200 dias, **0 violações** da implicação (nenhum perfil
sobrevive a R1 e falha em R0) e **3 contraexemplos** (perfis que passam em R0 e falham em algum dia). O instrumento
confere a conservação do agregado — a soma dos dias tem de reconstruir o agregado — e reprova um agregado que não
bata.

**Como reproduzir.**

~~~bash
cargo run --release --bin labs -- --resolucao
~~~

**O limite.** É simulação declarada: um perfil de média, 200 dias, serviço exponencial. O limiar de 2,5 é uma
propriedade deste desenho, e a operação real com a mesma decisão em dois grãos continua sendo o que falta.