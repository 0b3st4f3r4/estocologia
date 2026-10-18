# Synthetic rehearsal of item 13: the round of 30 pairs

Record of 2026-10-17. This file attests the complete output of `labs --jev` with **synthetic data**, and it exists
because an output of an instrument is only worth something with the provenance alongside. **It is not calibration of the model:** it is the rehearsal of the
whole path — artifact, declared hash, restricted decoding, G1, cost, adequacy, and Brier.

## 1. What was generated, and by whom

The generator is `crates/stochology-s2/examples/rodada_sintetica.rs`, deterministic and without drawing:

- 30 requests, each one the text that the bridge sends (`ponte::pedido`), for cases 1 to 30;
- 30 answers, each one the canonical document of the card of the test model, which passes in the schema;
- probability `0.05 + 0.90 * (((n * 7) % 19) / 18)`, always in the open interval (0, 1);
- outcome `1` when `(n * 13) % 5 < 3`, and `0` in the other two fifths: 18 positive and 12 negative;
- source `rodada-sintetica` in all the lines, so that the registry does not confuse it with an observed outcome.

## 2. The commands, in order

~~~bash
cargo run --release --example rodada_sintetica -- /tmp/sint
cargo run --release --bin ingestar-jev -- \
  empirical/p20-jev/pesos \
  /tmp/sint/inferencias_congeladas.tsv \
  empirical/p20-jev/calibracao \
  /tmp/sint/desfechos_auditados.tsv
(cd empirical/p20-jev/pesos && sha256sum -c SHA256)
(cd empirical/p20-jev/calibracao && sha256sum -c SHA256)
cargo run --release --bin labs -- --jev
~~~

Output of the ingestion: 30 frozen answers in the artifact, hash verified
`39d42d7d15320b00d35939eccf7e40e254a308969c03daa608840c29d423a5e5`; labeled set with Brier `0.348000`
over 30 pairs and 0 abstentions; `modelo.congelado: OK` and `desfechos.tsv: OK` in the check of the two seals.

## 3. The literal output of the instrument

~~~text
== Ponto 20 :: a ponte Jev-LLM ==

-- o portao dos pesos --
  pesos verificados: 39d42d7d15320b00d35939eccf7e40e254a308969c03daa608840c29d423a5e5

-- a decodificacao restrita --
  o modelo de ensaio responde no esquema: ficha jev.ponte.1 | camada L2 | palavras do documento: 92
  texto livre recusado: ForaDoEsquema("LinhaSemSeparador(\"isto e texto livre, nao uma ficha\")") | abstencao Outros
  margem em 100% recusada: ExtremoDogmatico("margem 1") | abstencao Outros

-- a prova de determinismo, na forma do G1 --
  modelo de ensaio: execucao 1 8813a29826c09a52 | execucao 2 8813a29826c09a52 | identicas: true
  modelo instavel: execucao 1 832a7ff86889326c | execucao 2 3495fa30b037ce75 | identicas: false
  leitura: o portao G1 fecha com um modelo congelado e NAO fecha com um modelo que muda de resposta — e o que o docs/JEV_INTEGRATION.md declara sobre o LLM livre

-- o custo por chamada e a adequacao do espaco --
  modelo de ensaio: 8 chamadas | 42.0 tokens por chamada | 0.0 ms por chamada
  modelo instavel: 4 pedidos | 4 abstencoes (100.0%) | o esquema da ficha NAO serve a este modelo: 100.0% de abstencoes, acima do limite declarado de 30%
  modelo de ensaio: 4 pedidos | 0 abstencoes (0.0%) | o esquema da ficha serve a este modelo: 0.0% de abstencoes, dentro do limite declarado de 30%

-- o runner local dos pesos congelados --
  pesos verificados: 39d42d7d15320b00d35939eccf7e40e254a308969c03daa608840c29d423a5e5 | 30 respostas congeladas no artefato
  determinismo na forma do G1: 476b6fe5cbe0a88d contra 476b6fe5cbe0a88d | identicas: true
  adequacao: 4 pedidos | 0 abstencoes (0.0%) | o esquema da ficha serve a este modelo: 0.0% de abstencoes, dentro do limite declarado de 30%
  custo medido: 87.0 tokens por chamada | 0.0 ms por chamada
  Brier: a regra esta em runner::brier e o conjunto rotulado em calibracao::carregar, com amostra minima e abstencao contada

-- a calibracao: o Brier do modelo congelado --
  Brier 0.348000 sobre 30 pares | abstencoes 0 (0.0%) fora do denominador
  regra declarada: 0.0 e 1.0 recusam, id fora de ordem recusa, desfecho fora de 0 e 1 recusa, abaixo de 30 pares o escore nao e publicado, abstencao sai do denominador
  ensaio do runner (nao sao os pesos reais): 0ea082454dea3bc3 | 4 respostas | G1 identicas: true | abstencoes 0.0% | 87.0 tokens por chamada | 0.0 ms por chamada
  leitura: o caminho artefato -> hash declarado -> tabela congelada -> portao roda de ponta a ponta; o que falta e o arquivo do modelo

-- o estado declarado --
  runner local: existe em crates/stochology-s2/src/runner.rs, com artefato, hash declarado antes da execucao e abstencao por pedido sem resposta
  pesos congelados: artefato verificado em empirical/p20-jev/pesos | a ponte roda com ele, e o hash fica declarado antes de cada execucao
  conjunto rotulado: com escore publicado | o Brier acima e o do conjunto verificado, nao do ensaio
  o que ja fecha: o portao dos pesos, a decodificacao restrita com abstencao, a prova de determinismo, o custo por chamada, a regra de adequacao do espaco, o runner e o leitor do conjunto rotulado
~~~

## 4. What these numbers attest, and what they do not attest

**They attest** the end-to-end path: the weights gate verifies a real artifact with the declared hash; the
restricted decoding keeps refusing free text and margin at 100%; G1 closes in the runner with the same
`sha256` in the two executions (`476b6fe5cbe0a88d`); the adequacy of the space measures 0% of abstention; the cost comes out at
87 tokens per call; and the calibration publishes a score because there are 30 pairs, exactly the declared minimum.

**They do not attest** calibration of the model. The Brier `0.348000` is that of the synthetic set against the answers of the test
model, and not of the frozen LLM against an observed outcome. Publishing it as calibration would be the fraud that the entire front
exists to prevent: the label `rodada-sintetica` is in every line, and this file is the warning.

## 5. The state after the rehearsal

The two synthetic files were **removed** from `empirical/p20-jev/pesos/` and `empirical/p20-jev/calibracao/` at the end of the
rehearsal, and the instrument went back to refusing — which is what `docs/RUST_RESULTS.md` records, because the manifest is generated
after the cleanup. What stays in this folder is this record and the generator; no weights, no outcome.
