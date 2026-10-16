# Declaração do ponto 13 (escuta): o lacre da caixa e a cláusula de saída

**Congelada em 2026-10-16T21:19:42Z**, antes de qualquer tentativa de contato com qualquer grupo.

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `protocolo.md` | `fee645912c789b4ab8a0d4ba5585e0f36d4f41eb23c4fd82d56a0191d2fd4b02` | o protocolo, congelado: daqui em diante não é reescrito |
| `registro.tsv` | `646f880ad35f53d6289bf4f18e855966fec9e844279e17d8a2ab432ba8417f47` | o registro em máquina das sete decisões, de quem é o erro e quem foi ouvido |
| `DECLARACAO.md` (esta) | — | este documento; o hash próprio sai em `DECLARACAO.sha256`, calculado depois da gravação |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — as sete coortes históricas não estão ao alcance

Declarado pelo dono do programa: **nenhum dos sete grupos históricos está ao alcance hoje.** As coortes são de
décadas atrás — crianças do STAR dos anos 1980, famílias do RAND HIE dos anos 1970, detidos de Toronto de
1997–2002, presos de Toronto de 2020–2021, motoristas de Rhode Island de 2005–2015, trabalhadores acidentados do
Kentucky e de Michigan dos anos 1980, participantes do NSW e controles do CPS dos anos 1970 —, e o contato direto é
inviável por barreira temporal, legal, ética e de anonimato.

O programa não inventa registro de consulta que não houve. A coluna "ouvidos" de
[`governanca/GRUPOS.md`](../../governanca/GRUPOS.md) continua **nenhum** nas sete linhas, e continua assim enquanto
não houver escuta registrada. O que se procura, a partir daqui, é **equivalente estrutural**: quem hoje ocupa a
mesma posição de quem pagou o erro então.

## Declaração 2 — o lacre, a cobertura mínima e o prazo

Medido pelo instrumento que lê este registro, na hora do congelamento:

~~~bash
cargo run --release --bin labs -- --escuta
~~~

~~~text
== Ponto 13 :: a escuta de quem paga o erro ==
decisoes: 7 | carimbo do registro: 2026-10-16 | prazo da janela: 2026-12-15 | cobertura minima: 2 de 7
decisao                              titulares ouvidos      registro   pode emitir
h2g/saida/direito_teto_beneficio             - nenhum       -          nao
h2g/saida/educacao_star_turmas           11598 nenhum       -          nao
h2g/saida/saude_rand                         - nenhum       -          nao
h1-real/saida/b                          16177 nenhum       -          nao
h3/prisoes                                5226 nenhum       -          nao
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

cobertura de escuta: 0 de 7 grupos ouvidos
divergencias: nenhuma — nenhuma decisao se diz emitida sem ninguem ouvido
veredito: CAIXA LACRADA — a cobertura de 0 fica abaixo do minimo de 2; as decisoes ficam como analise de sensibilidade (n = 1) e nao viram diretriz de politica publica nem decisao de portao F8 ate 2026-12-15 ou ate a cobertura minima
~~~

| o que o lacre decide | valor declarado | de onde sai |
| --- | --- | --- |
| cobertura de escuta | 0 de 7 | `registro.tsv`, coluna "ouvidos" |
| cobertura mínima para abrir a caixa | 2 dos 7 grupos, com registro assinado | `registro.tsv`, chave `grupos_minimos` |
| prazo da janela | 60 dias corridos, até **2026-12-15** | `registro.tsv`, chave `prazo` |
| decisões divergentes (emitidas sem ninguém ouvido) | 0 | o portão do instrumento |
| veredito | **caixa lacrada** | cobertura abaixo do mínimo |

**O que o lacre faz.** As sete decisões ficam como análise de sensibilidade ($n = 1$) e **não** viram diretriz de
política pública nem decisão de portão F8 até a cobertura mínima ser atingida. Se o prazo vencer sem a cobertura, o
artefato fica arquivado como exercício metodológico de sensibilidade, e o lacre tem hash. Se a cobertura for
atingida, as linhas cobertas saem do lacre e os registros entram em `governanca/GRUPOS.md`.

**O produtor de fumaça.** Os números do lacre ficam no registro em máquina, e o instrumento que os lê **sabe
reprovar**: uma decisão que se diz emitida com a coluna "ouvidos" em nenhum é divergência, e a caixa só abre quando a
cobertura chega ao mínimo. Os dois casos sintéticos vão no relatório do instrumento, e os testes do módulo prendem
os dois.

## Declaração 3 — quem conduz e o roteiro

A escuta **não é conduzida pelo dono do programa**: as prioris e os ajustes saíram da mesma mão. Vale uma das duas
formas, e a escolha entre elas entra em declaração própria **antes** da primeira conversa:

1. facilitador neutro ou institucional, sem vínculo com o programa; ou
2. protocolo cego: quem entrevista recebe só o roteiro, sem os números do funil, sem a priori e sem saber qual
   decisão está por trás da conversa.

O roteiro tem três perguntas, em linguagem comum, sem vocabulário do programa:

1. *Qual é o seu pior cenário que ainda é plausível?*
2. *O que faria você recusar essa intervenção?*
3. *Quando essa política falha, quem de fato arca com o custo?*

O que o grupo disse não é traduzido para o vocabulário do programa por quem conduz. Tradução, se houver, é feita à
parte e fica marcada como leitura do programa.

## Declaração 4 — a voz do operador não conta

A voz de quem modela já está inteira nas prioris e nos ajustes. O operador projeta a regra; o grupo sofre a
consequência. Registrar o operador na coluna "ouvidos" seria conflito de interesse estrutural e anularia a frente.
**O operador não conta como grupo ouvido em nenhuma das sete decisões**, e esta declaração é a que impede a
contagem de subir sozinha.

## Declaração 5 — o que esta frente não muda

Nenhuma rodada oficial troca de priori por causa desta declaração: o funil segue com a priori do pacote e as cinco
rodadas elicitadas do E1 seguem como sensibilidade com $n = 1$, como o protocolo do SHELF já dizia. O Capítulo 21
ganhou o parágrafo do lacre e o bloco do instrumento; nenhum outro número do livro mudou.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test --workspace` | 219 testes aprovados, nenhum reprovado, com os 5 do portão da escuta (0) |
| `cargo test -p stochology-book` | 28 testes aprovados, entre eles `o_registro_do_programa_tem_sete_decisoes_e_nenhum_ouvido` e `a_decisao_emitida_sem_ninguem_ouvido_reprova` (0) |
| `labs --escuta` | 7 decisões, 0 de 7 ouvidos, 0 divergências, caixa lacrada com prazo em 2026-12-15, mais as duas provas sintéticas (0) |
| `python3 book/conferir_labs.py` | 26 blocos citados, 0 divergentes — o bloco 2 do Capítulo 21 confere em `labs --escuta` (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` atualizado, com o bloco novo do lacre (0) |
