# Fechamento do item 13: o protocolo, e o que ainda falta

Este arquivo é o checklist do fechamento, escrito **antes** dos dados entrarem. Ele obedece à regra da casa: o
critério é declarado antes, o insumo entra pela rampa estrita, o instrumento mede, e o registro recebe o número com
a procedência ao lado. Enquanto os quatro critérios da fase 3 não estiverem atestados com o insumo real, o item 13
**não sai** da lista de pendências vivas de `PROPOSTAS_DECLARADAS.md` — e este arquivo diz por quê, em vez de
deixar a lista mudar por entusiasmo.

## Fase 1 — as fontes, fora da árvore

| insumo | formato | o que a fonte tem de cumprir |
| --- | --- | --- |
| `inferencias_reais.tsv` | `pedido<TAB>resposta`, com a quebra interna da ficha escapada como `\n` | resposta no esquema formal da ficha, sem texto livre e sem tag solta |
| `desfechos_auditados.tsv` | `caso_id<TAB>prob<TAB>desfecho<TAB>fonte` | N ≥ 30, p em (0, 1) estrito, desfecho 0 ou 1, procedência exógena na coluna `fonte` |

O catálogo de casos pode ser montado à mão ou com `tools/gerar_fontes_incendios.py`, que tem duas etapas
desacopladas: `conferir-casos` valida o catálogo sem chamar modelo nenhum, e `inferir` submete cada prompt ao runner
— `llama-cli` por subprocesso, `vllm` por `/v1/completions`, `api` por `/v1/chat/completions` com a chave lida de
`JEV_API_KEY`, `arquivo` para respostas já colhidas e `demo` para atestar o encanamento com ficha determinística —,
valida o esquema e grava as duas fontes com a quebra escapada.

**A rota por API não entrega peso congelado.** Provedor remoto não publica `sha256` dos pesos e pode trocar o modelo
sem avisar, então dois critérios da fase 3 mudam de natureza: o portão dos pesos passa a ser a **revisão declarada do
modelo** (identificador de versão, data e, quando houver, o `sha256` que o provedor publicar), e o G1 deixa de ser
garantia e passa a ser **medido** — `--repetir 3` faz a mesma pergunta três vezes e conta quantas respostas são
idênticas à primeira. Repetição que não fecha é resultado, não ruído: significa que aquele provedor não serve como
peso congelado, e a ponte reporta isso em vez de fingir determinismo. O `demo` e o `exemplo` existem para provar o caminho: o que sai deles carrega a etiqueta
`SINTETICO-...` e `sintetico-demo` na coluna de procedência, e não serve para calibrar.

A conferência da fase 1 é mecânica e não escreve nada:

~~~bash
cargo run --release --bin ingestar-jev -- --conferir \
  caminho/para/inferencias_reais.tsv \
  caminho/para/desfechos_auditados.tsv
~~~

Ele lê as duas fontes, aplica as regras campo a campo, roda o ensaio em diretório temporário, imprime o volume
contra o mínimo de 30 e a leitura da calibração, e **não toca em destino nenhum**. Volume abaixo do mínimo sai com
código 1.

## Fase 2 — a ingestão e os lacres

~~~bash
cargo run --release --bin ingestar-jev -- \
  fundo/p20-jev/pesos \
  caminho/para/inferencias_reais.tsv \
  fundo/p20-jev/calibracao \
  caminho/para/desfechos_auditados.tsv
(cd fundo/p20-jev/pesos && sha256sum -c SHA256)
(cd fundo/p20-jev/calibracao && sha256sum -c SHA256)
~~~

Saída esperada: `modelo.congelado: OK` e `desfechos.tsv: OK`. O ingestor só persiste depois de o ensaio passar
nos mesmos portões da ponte; se recusar, o destino fica intocado.

## Fase 3 — os quatro critérios, medidos pelo instrumento

~~~bash
cargo run --release --bin labs -- --jev
~~~

| critério | o que a saída tem de mostrar | o que reprova |
| --- | --- | --- |
| 1. Portão dos pesos | `pesos verificados: <sha256>` | `RECUSADO` por hash ausente, divergente ou artefato ilegível |
| 2. Determinismo G1 | `identicas: true` com o mesmo `sha256` nas duas passagens | `identicas: false` |
| 3. Adequação do espaço | `abstencoes (x%)` com x ≤ 30, medida **sobre os casos do próprio artefato** | x > 30: o item fecha com o veredito formal de que o esquema **não** serve ao modelo, com as recusas contadas, sem imputação |
| 4. Brier e ECE | `Brier <valor> e ECE <valor> em 10 baldes sobre N pares`, com N ≥ 30 e abstenções fora do denominador | N < 30: `AmostraInsuficiente`, nenhum escore |

O critério 3 admite dois desfechos e os dois fecham o item: o esquema serve, ou o esquema não serve. O que não
fecha é esconder a taxa de abstenção ou imputar 0,5 no lugar da recusa. Desde 2026-10-17 a adequação é medida sobre
os casos que o artefato guarda, e não sobre uma sonda fixa do instrumento; artefato sem o texto do pedido (v1) cai
na sonda, e o instrumento declara qual dos dois usou.

O ECE usa a mesma função do `stochology-h3` — `ece(confianca, acerto, 10)` —, reusada em vez de reescrita, para o
repositório não ter duas definições de calibração.

## Fase 4 — registro, lacres e o movimento da lista

O `registro.tsv` desta pasta ganha uma linha por rodada real, com o formato já declarado:

| campo | de onde sai |
| --- | --- |
| `data` | a data da execução, não a da declaração |
| `N` | o número de pares do conjunto verificado |
| `abstencoes` | a contagem que o instrumento imprimiu |
| `custo_por_chamada` | tokens e milissegundos por chamada, como o instrumento imprimiu |
| `brier` | o escore condicional, com seis casas |

Depois disso:

~~~bash
python3 conferir_lacres.py
python3 book/gerar_resultados.py && python3 book/gerar_resultados.py --check
python3 book/gerar.py --check
python3 book/conferir_labs.py
python3 estilo/conferir-passada.py
cargo test --workspace
~~~

O item 13 só passa para a seção de fechados de `PROPOSTAS_DECLARADAS.md` quando a linha do `registro.tsv` existir
com os quatro critérios atestados e a saída do instrumento colada em `ENSAIO_SINTETICO.md` (ou no registro da
rodada real). Antes disso, o estado é o de hoje:

| fase | estado em 2026-10-17 |
| --- | --- |
| 1. fontes | **faltam as duas**: não há inferência de modelo real nem desfecho auditado na árvore |
| 2. ingestão | pronta e provada: o ingestor grava, recusa e não toca no destino quando recusa |
| 3. critérios | medidos no ensaio sintético de 30 pares (`ENSAIO_SINTETICO.md`); **não medidos no modelo** |
| 4. registro | formato declarado; **nenhuma linha de rodada real**, porque não houve rodada real |

O ensaio sintético prova o caminho, não o modelo. O número dele é do conjunto sintético e está etiquetado assim;
publicá-lo como calibração seria trocar a declaração pelo carimbo.
