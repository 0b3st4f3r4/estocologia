# Ponto 20 — a ponte Jev-LLM

**Estado: o arcabouço fecha e os pesos faltam. Nenhum número da ponte é publicado sem eles.**

O item 13 pede o runner local de pesos congelados, o `sha256` dos pesos, a decodificação restrita ao esquema da
ficha, a prova de determinismo na forma do G1, a calibração contra o Brier do Jev determinístico, o custo por
chamada e a pergunta paralela de adequação do espaço de hipóteses. **O runner local (`crates/stochology-s2/src/runner.rs`,
397 linhas, 9 testes), o leitor de calibração (`calibracao.rs`, 9 testes), a ponte (`ponte.rs`, 12 testes) e o binário
`ingestar-jev` (7 testes de integração) já existem nesta árvore** — o que falta são os dois artefatos de operador
(`fundo/p20-jev/pesos/modelo.congelado` com `pesos/SHA256` e `fundo/p20-jev/calibracao/desfechos.tsv` com
`calibracao/SHA256`), com os portões armados.

**O que já fecha, com prova negativa.** O portão dos pesos: sem o arquivo, ou com o hash diferente do declarado, a
ponte **não roda**. A decodificação restrita: texto livre e margem em 100% são recusados, e a recusa é a abstenção
`Categoria::Outros` — o "não sei" auditável que já existia no Sistema 1. A prova de determinismo na forma do G1:
duas execuções e o mesmo `sha256` fecha no modelo de ensaio e **não fecha** no modelo instável, que é a declaração
do `INTEGRACAO_JEV.md` sobre o LLM livre, agora medida. O custo por chamada e a fração de abstenções saem do ensaio.

**O critério declarado antes.** Se a fração de pedidos em que a ponte tem de abster passa de **30%**, o esquema da
ficha não serve àquele modelo — e a ponte reporta isso em vez de forçar resposta fora do esquema.

**Como reproduzir.**

~~~bash
cargo run --release --bin labs -- --jev
~~~

**O limite.** Sem os pesos verificados e o conjunto rotulado, nada de número publicado: nem Brier calibrado, nem custo
real, nem abstenção do modelo de verdade. O K da frente é 1.
