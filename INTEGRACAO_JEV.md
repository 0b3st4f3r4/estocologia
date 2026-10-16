# Integração do material Jev à Estocologia

Registro da integração da transcrição *Integração de Modelos Jev à Estocologia*. A transcrição responde a doze
perguntas sobre como acoplar modelos do tipo Jev — classificação categórica fechada em passo único — à
arquitetura desta árvore. Cada alegação dela foi conferida contra o código, e não contra a redação anterior: o
que já existe aparece com o arquivo e o comando, o que está aposentado aparece como **declaração de divergência**
com os dois valores, e o que é novo aparece como proposta declarada em `PROPOSTAS_DECLARADAS.md` (item 13).

Data da passada: **2026-10-16** (contexto de execução da sessão, conforme `AGENTS.md`).

Legenda, a mesma de `REGISTRO_EMPIRICO.md`:

| Marca | Significado |
| --- | --- |
| **V** | verificado no código desta árvore, com arquivo e comando |
| **D** | divergência declarada, com o valor do transcript e o valor da árvore |
| **P** | declarado sem instrumento: existe no texto e não existe em código |

## 1. Fonte e procedência

| Campo | Valor |
| --- | --- |
| Título | *Integração de Modelos Jev à Estocologia* |
| Origem | conversa do Gemini, com a URL no cabeçalho do próprio arquivo |
| Arquivo local | `Integração de Modelos Jev à Estocologia.md`, 470 linhas, 6 263 palavras, 12 turnos |
| `sha256` do bruto | `cf223807b4ac578b4fc68bf5180703dff4246a6f09ee1d209b34dd894da2276b` |
| Estado no Git | fora do índice. O bruto é a fonte; a forma integrada é este registro (`PLAN.md` §9.12) |

## 2. Veredito de cada alegação

| Alegação do transcript | Marca | Onde vive, e o comando que reproduz |
| --- | --- | --- |
| O Sistema 1 é triagem tipada em `O(1)`, com árvore de sufixos determinística | **V** | `crates/stochology-core/src/s1.rs` (877 linhas, 12 testes), com a árvore materializada em `BTreeMap` (`s1.rs:275-328`) — `labs --s1` |
| O Sistema 1 emite rótulo de conjunto fechado e probabilidade presa no intervalo | **V** | `Categoria::TODAS` com seis rótulos (`s1.rs:127`), `PISO` e `TETO` em `s1.rs:52-55`, transferência afim `p = 0,01 + 0,94 x escore` em `s1.rs:84` |
| A abstenção é resultado válido e auditável | **V** | `Categoria::Outros` é o "não sei" declarado (`s1.rs:22`, `s1.rs:122`, `s1.rs:403`), com prova em `s1.rs:772` (texto vazio devolve `Outros`) |
| O Sistema 2 é deliberativo, com busca em árvore e estado de crença | **V** | `crates/stochology-s2` (sete módulos: `lats`, `crenca`, `hipergrafo`, `inferencia`, `estresse`, `planejamento`, `lib`) — `labs --s2` |
| A banca adversarial aprova por conjunção das oito severidades | **V** | `agentes.rs:111` implementa `Aprovado(X) = AND A_i(X) >= Severidade_min`, com `SEVERIDADE_MINIMA = 6` em `agentes.rs:11` |
| O orquestrador conduz o ciclo e submete o artefato à banca | **V** | `orquestrador.rs:7` (`OrquestradorM`), `orquestrador.rs:21` (`conduzir_ataque`), `orquestrador.rs:30` (`banca`) |
| O nome do orquestrador é $M_{15}$ e a banca é $MB_1$–$MB_8$ | **D** | a árvore usa $\mathcal{M}$ e $A_1$–$A_8$; ver a declaração 1 |
| O funil tem quatro filtros, com poda por valor da informação | **D** | `crates/stochology-core/src/funil.rs` (878 linhas, 17 testes); ver a declaração 2 |
| Os invariantes estruturais são $\Phi_1$–$\Phi_4$ | **D** | a árvore tem $\Phi_1$–$\Phi_5$ (`invariantes.rs:12`, `invariantes.rs:110`); ver a declaração 3 |
| O lugar do Sistema 1 é ocupado por `LLM-as-Jev` sobre pesos abertos congelados | **D** | o lugar do LLM é ocupado por um **Jev** determinístico, sem geração livre (`s1.rs:10-22`); ver a declaração 4 |
| `temperature=0` sobre a árvore de sufixos dá os mesmos logits e o mesmo `sha256` | **D** | não há instrumento que meça isso aqui, e o argumento do `s1.rs` é o oposto; ver a declaração 5 |
| O cabeçalho obrigatório tem cinco perguntas, com `pergunta_do_decisor` dentro | **D** | cinco perguntas em `PLAN.md` §10 e a trava de tipo III como campo próprio (`camada.rs:59`); ver a declaração 6 |
| "0% de falha de esquema sintático" | **D** | vale para o Jev determinístico, que não gera texto; para LLM livre não há medida nesta árvore; ver a declaração 7 |
| Os trechos de Rust do transcript (`OrquestradorM`, `AgenteA`, `AgenteAtaqueAs`) | **D** | os dois primeiros nomes existem, com assinatura diferente, e o terceiro não existe; ver a declaração 8 |
| As faixas $F_2$, $F_5$, $F_6$ e $F_7$ ganham classificação em lote por rubrica | **P** | declarado no texto; não há classificador de rubrica em código. O que existe é a validação da ficha e o funil |
| A pergunta paralela de adequação do espaço de hipóteses acompanha toda chamada de Sistema 1 | **P** | a abstenção existe; a pergunta paralela de adequação não existe em código. Entra no item 13 |
| Truncar a probabilidade do LLM em $[1\%, 99\%]$ alimenta o diário de Brier e Kelly | **P** | a faixa existe na escala do Jev; a ponte com um LLM fica declarada |

## 3. Declarações de divergência

1. **$M_{15}$ e $MB_1$–$MB_8$.** O transcript parte da numeração do roteiro antigo e a substitui, no meio da
   conversa, por $\mathcal{M}$ e $A_1$–$A_8$. Os dois valores: o texto abre com $M_{15}$ e $MB_1$–$MB_8$ e fecha
   com $\mathcal{M}$ e $A_1$–$A_8$; a árvore usa $\mathcal{M}$ (`orquestrador.rs`) e $A_1$–$A_8`
   (`agentes.rs:15`). Os rótulos $M0$–$M15$ e $MB_1$–$MB_8$ estão aposentados desde 2026-10-16, registrados no
   Capítulo 13 e em `PROPOSTAS_DECLARADAS.md` §4. O que a conversa chama de "ajuste de notação" é a decisão que
   já está na árvore.
2. **Quatro filtros.** O texto fala de quatro filtros e aponta a poda por valor da informação como futura. Os
   dois valores: quatro filtros no texto; um funil com **17 testes** em `funil.rs` (878 linhas), incluindo
   `o_funil_conta_as_tentativas_ate_a_versao_final`. A divergência de estado também estava na árvore, na linha 2
   da tabela de eixos do `PLAN.md` §9.13, que dizia "declarado, não implementado"; a linha foi corrigida nesta
   passada.
3. **$\Phi_1$–$\Phi_4$.** O texto enumera quatro invariantes. Os dois valores: quatro no texto; cinco na
   árvore, com $\Phi_5$ (resolução da pergunta) em `invariantes.rs:12` e `invariantes.rs:110`, contado pelo
   binário `portoes`. A redação do transcript é anterior à entrada do campo `resolucao` na ficha.
4. **`LLM-as-Jev`.** O texto trata o LLM como o corpo do Sistema 1. Os dois valores: pesos abertos congelados
   com `temperature=0` no texto; um Jev determinístico implementado sem LLM na árvore. O motivo está escrito no
   cabeçalho de `s1.rs`: um LLM livre gera texto novo a cada chamada, duas execuções divergem e o portão G1
   (determinismo) não fecha. Por isso o material vira proposta declarada (item 13), e não implementação.
5. **Determinismo do `temperature=0`.** O texto afirma que execuções sucessivas produzem os mesmos logits, as
   mesmas probabilidades e o mesmo `sha256`. Os dois valores: afirmação do texto, sem medição; nenhuma medição
   nesta árvore. O que a árvore mede é o determinismo do Jev: `portoes --horizonte todos --reproducao` aprova
   G1 e G2 com a segunda execução idêntica.
6. **O cabeçalho.** O texto escreve o *payload* com seis campos e chama o conjunto de "cinco perguntas". Os dois
   valores: seis campos no texto; cinco perguntas no `PLAN.md` §10 (processo gerador, não-linearidade, unidade
   independente, física do ruído, escada causal), com `pergunta_do_decisor` como trava de tipo III em campo
   próprio (`camada.rs:59`).
7. **Falha de esquema.** O texto atribui "0% de falha de esquema sintático" ao modelo Jev. Os dois valores:
   zero por cento no texto; zero por cento medido só para o Jev determinístico, que escolhe entre seis rótulos e
   nunca gera frase. Para um LLM livre a árvore não tem instrumento, e a marca é **P**.
8. **Os trechos de Rust.** O texto propõe `OrquestradorM`, `AgenteAtaqueAs` e `AgenteA`, com um
   `conduzir_ataque` que devolve `Ok(())`. Os dois valores: os esboços do texto; `OrquestradorM`
   (`orquestrador.rs:7`) e `AgenteA` (`agentes.rs:15`) existem com o mesmo nome e assinatura diferente —
   `conduzir_ataque(&self, agente: AgenteA, ficha: &Ficha) -> Result<Severidade, String>` —, e
   `AgenteAtaqueAs` não existe. O texto não devolveu severidade, e a árvore devolve.

## 4. O que fica como proposta declarada

A ponte entre o Jev determinístico e um modelo de linguagem rodando localmente é o item **13** de
`PROPOSTAS_DECLARADAS.md`: *Ponte Jev-LLM (Sistema 1 sobre pesos abertos congelados)*. Falta o runner com pesos
congelados e `sha256` dos pesos, a decodificação restrita ao esquema da ficha, a prova de determinismo na forma
do G1, a calibração contra o Brier do Jev e o custo por chamada. A abstenção já existe; falta a pergunta paralela
de adequação do espaço de hipóteses, que a incerteza knightiana exige.

## 5. Verificação desta integração

Rodada em 2026-10-16, nesta árvore, com o código como está. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test --workspace` | 201 testes aprovados, nenhum reprovado (0) |
| `cargo run --release --bin labs -- --s1` | a triagem roda: árvore de 136 sufixos, seis categorias, soma 1,0000 em cada linha e **0** probabilidades fora de $[0{,}01; 0{,}99]$ (0) |
| `cargo run --release --bin labs -- --s2` | o Sistema 2 roda: 8 nós, 2 arestas de grafo e 5 hiperarestas validados, o LATS chega em `f1.sintese-de-codigo` onde o ganancioso para em `h0.fila`, a posterior da Beta(4, 8) dá 0,3346 contra a forma fechada 0,3333 e o eixo R escolhe R1-dia com a perda 46,24 min (0) |
| `cargo run --release --bin portoes -- --horizonte todos --reproducao` | os sete portões aprovados, com a banca $A_1$–$A_8$ no pior 8 contra o piso 6, segunda execução idêntica e raiz de Merkle `9716e28a87bec9cf691db6127e810430738eab58a083c303777a00814cdf3a81` (0) |
| `python3 book/gerar.py --check` | o livro monta e confere: 5 593 linhas e 71 318 palavras (0) |
| `python3 book/conferir_labs.py` | 24 blocos citados, 0 divergentes (0) |
| `python3 book/conferir_livro.py` | 5 593 linhas, 21 capítulos, 5 partes e cercas equilibradas (0) |
| `python3 book/gerar_resultados.py --check` | **estava desatualizado (1)** quando a passada começou: os testes novos do painel SHELF, de outra frente, ainda não estavam no registro. Com `python3 book/gerar_resultados.py`, o arquivo passou a conferir (0). A diferença é de conteúdo de instrumento — 26 testes de `shelf` e `preteste` onde o registro trazia 15, e o veredito do portão no mundo calibrado do pré-teste —, e não desta integração |

A integração não toca código: `cargo test` e os dois instrumentos do Sistema 1 e do Sistema 2 rodam o que já
existia. O único estado corrigido foi o registro `RESULTADOS_RUST.md`, que estava atrás do código por causa da
frente do painel SHELF.
