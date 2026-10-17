# Ensaio sintético do item 13: a rodada de 30 pares

Registro de 2026-10-17. Este arquivo atesta a saída completa do `labs --jev` com **dado sintético**, e existe
porque uma saída de instrumento só vale com a procedência ao lado. **Não é calibração do modelo:** é o ensaio do
caminho inteiro — artefato, hash declarado, decodificação restrita, G1, custo, adequação e Brier.

## 1. O que foi gerado, e por quem

O gerador é `crates/stochology-s2/examples/rodada_sintetica.rs`, determinístico e sem sorteio:

- 30 pedidos, cada um o texto que a ponte manda (`ponte::pedido`), para os casos 1 a 30;
- 30 respostas, cada uma o documento canônico da ficha do modelo de ensaio, que passa no esquema;
- probabilidade `0.05 + 0.90 * (((n * 7) % 19) / 18)`, sempre no intervalo aberto (0, 1);
- desfecho `1` quando `(n * 13) % 5 < 3`, e `0` nos outros dois quintos: 18 positivos e 12 negativos;
- fonte `rodada-sintetica` em todas as linhas, para o registro não confundir com desfecho observado.

## 2. Os comandos, na ordem

~~~bash
cargo run --release --example rodada_sintetica -- /tmp/sint
cargo run --release --bin ingestar-jev -- \
  fundo/p20-jev/pesos \
  /tmp/sint/inferencias_congeladas.tsv \
  fundo/p20-jev/calibracao \
  /tmp/sint/desfechos_auditados.tsv
(cd fundo/p20-jev/pesos && sha256sum -c SHA256)
(cd fundo/p20-jev/calibracao && sha256sum -c SHA256)
cargo run --release --bin labs -- --jev
~~~

Saída da ingestão: 30 respostas congeladas no artefato, hash verificado
`39d42d7d15320b00d35939eccf7e40e254a308969c03daa608840c29d423a5e5`; conjunto rotulado com Brier `0.348000`
sobre 30 pares e 0 abstenções; `modelo.congelado: OK` e `desfechos.tsv: OK` na conferência dos dois lacres.

## 3. A saída literal do instrumento

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
  leitura: o portao G1 fecha com um modelo congelado e NAO fecha com um modelo que muda de resposta — e o que o INTEGRACAO_JEV.md declara sobre o LLM livre

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
  pesos congelados: artefato verificado em fundo/p20-jev/pesos | a ponte roda com ele, e o hash fica declarado antes de cada execucao
  conjunto rotulado: com escore publicado | o Brier acima e o do conjunto verificado, nao do ensaio
  o que ja fecha: o portao dos pesos, a decodificacao restrita com abstencao, a prova de determinismo, o custo por chamada, a regra de adequacao do espaco, o runner e o leitor do conjunto rotulado
~~~

## 4. O que estes números atestam, e o que não atestam

**Atestam** o caminho de ponta a ponta: o portão dos pesos verifica um artefato real com o hash declarado; a
decodificação restrita continua recusando texto livre e margem em 100%; o G1 fecha no runner com o mesmo
`sha256` nas duas execuções (`476b6fe5cbe0a88d`); a adequação do espaço mede 0% de abstenção; o custo sai em
87 tokens por chamada; e a calibração publica escore porque há 30 pares, exatamente o mínimo declarado.

**Não atestam** calibração do modelo. O Brier `0.348000` é do conjunto sintético contra as respostas do modelo de
ensaio, e não do LLM congelado contra desfecho observado. Publicá-lo como calibração seria a fraude que a frente
inteira existe para impedir: a etiqueta `rodada-sintetica` está em cada linha, e este arquivo é o aviso.

## 5. O estado depois do ensaio

Os dois arquivos sintéticos foram **removidos** de `fundo/p20-jev/pesos/` e `fundo/p20-jev/calibracao/` no fim do
ensaio, e o instrumento voltou a recusar — é o que `RESULTADOS_RUST.md` registra, porque o manifesto é gerado
depois da limpeza. O que fica nesta pasta é este registro e o gerador; nenhum peso, nenhum desfecho.
