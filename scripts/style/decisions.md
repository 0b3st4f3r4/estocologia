# Decisões e pendências — revisão de prosa

Diário do revisor de língua e estilo. Cada entrada ou é decisão já aplicada (e registrada em
`scripts/style/style-sheet.md`) ou pendência do autor, com arquivo, linha e citação. A última palavra sobre
cada parágrafo é do revisor principal: as reescritas delegadas voltaram para releitura contra a edição
montada de antes (`book/stochology-textbook.md` no commit de partida), com conferência de número,
cerca, bloco ` ```text ` e marcador por arquivo.

Data da passada: **2026-10-16** (contexto de execução da sessão).

## Decisões estruturais

1. **O livro vive em `book/`**, não em `livro/`. `scripts/book/build.py`, `scripts/book/check_labs.py` e
   `scripts/book/check_book.py` apontam para lá, e a verificação por arquivo é `scripts/style/check_pass.py`.
2. **Contagem de prosa.** A varredura (`scripts/style/varredura.py`) descarta cerca de código e bloco ` ```text `;
   `scripts/style/dois_pontos.py` separa prosa, tabela e cerca. Os alvos de percentual incidem sobre prosa.
3. **Piso do travessão apostivo.** Os 63 travessões dos cabeçalhos `### Degrau N — …` (três por capítulo)
   e os dos parágrafos **Declaração de divergência** não se tocam. Fixam o piso, e o alvo global de
   densidade os desconta.
4. **Célula de tabela é congelada.** Prosa dentro de item de lista, de `**Exercícios.**` e de `**Resumo.**`
   é prosa: pode ser reescrita com o marcador, a ordem, a quantidade e a afirmação intactos.
5. **Rótulo em negrito que abre parágrafo** é prosa e foi reescrito quando repetia o corpo do parágrafo
   (`**Uma costura, não um ramo novo.**` → `**A costura de três tradições.**`). Nome de seção que funciona
   como identificador de quadro ficou em pendência (cap19, L105).
6. **Fala citada, pergunta de exercício e referência cruzada** ficam na forma do autor.
7. **Baseline.** `scripts/style/check_pass.py` compara contra `git show HEAD:book/stochology-textbook.md`;
   aceita `ESTILO_BASELINE=<caminho>` para usar outra cópia.
8. **Prosa dentro de item de lista e rótulo em negrito editados**: cinco linhas de lista em `cap07.md`,
   os itens do `**Resumo.**` em `cap11.md`, `cap13.md`, `cap16.md` e `cap17.md`, o bullet do
   `**Resumo.**` em `cap03.md`, e `**A tese tem uma versão operacional.**` em `cap02.md`. Marca, número,
   ordem e afirmação continuam iguais; números e fórmulas intactos.
9. **Instrumental de estilo, declarado por inteiro.** Os cinco scripts de `scripts/style/` têm função contínua e
   nenhum é rascunho: `varredura.py` conta os maneirismos e é a base dos outros quatro; `dois_pontos.py` separa
   prosa, tabela e cerca; `ocorrencias.py` lista as ocorrências de cada padrão com o número da linha, que é o
   que transforma contagem em reescrita; `atualizar-indice.py` troca um dos dois blocos marcados de
   `scripts/style/repetition-index.md` pela tabela recalculada, para o índice não ser transcrito à mão; e
   `check_pass.py` é o portão, o único que entra na verificação obrigatória de `AGENTS.md` §V. Os quatro
   primeiros instrumentam a passada; o quinto a cobra.

## Pendências do autor

### Travessão em cabeçalho, `## Capítulo N.` e título de seção

Ocorrência em todos os 21 capítulos: `### Degrau 1 — O mundo real e o paradoxo`, `### Degrau 2 — A regra
do jogo`, `### Degrau 3 — Mão na massa`. Sem tocar nos três, o alvo de 20% do padrão não fecha em
capítulo nenhum; o piso é três por capítulo, mais o que a tabela e a declaração de divergência carregam.

- `book/cap01.md` L3: `## Capítulo 1. O alvo invisível: o que é a Estocologia?` segura um dois-pontos e um
  `o que`.
- `book/cap15.md` L1: `## Capítulo 15. O cofre, a senha criptográfica e a auditoria contra si mesmo` —
  `mesmo` no título.
- `book/cap15.md` L93, L104, L188: `**O limite: o auditor é parte interessada.**`,
  `**A auditoria de fundo: o que foi suposto e o que se mediu.**`, `**A corrente sozinha não basta: a raiz
  é a prova.**` — dois-pontos em título de bloco.
- `book/cap19.md` L105: `**Definição de ambiente.**` substituiu `**O que é um ambiente.**`. O nome da
  seção é escolha do autor; a troca foi feita para caber no orçamento de `o que`.

### Célula de tabela congelada (o padrão conta, a reescrita não toca)

- `book/cap08.md` L45; `book/cap13.md` L31, L34, L41, L61, L37, L133; `book/cap15.md` L112, L114, L117,
  L118, L120, L125; `book/cap21.md` L88, L92, L94, L98, L114, L115, L117, L118, L119, L120;
  `book/cap20.md` L45, L172; `book/cap18.md` L22-27; `book/apendices.md` L10, L18, L24, L96, L140, L141,
  L144; `book/cap04.md` L45; `book/cap07.md` L88, L125; `book/cap10.md` célula do `, não` e do `próprio`.
- São travessão apostivo, `o que`, `mesmo/mesma`, `próprio` e `, não` dentro de coluna: a coluna manda no
  comprimento, e a lista é fechada.

### Declaração de divergência (estilema declarado)

- `book/cap08.md` L165-166; `book/cap12.md` L232-241; `book/cap14.md` L242-250; `book/cap15.md` L198-206;
  `book/cap16.md` L200-208; `book/cap17.md` L224, L234; `book/cap18.md` L191-201; `book/cap20.md` L225,
  L226, L232, L289; `book/cap21.md` L246. Cada parágrafo conserva travessão, `, não`, `exatamente` e
  dois-pontos que a varredura conta; o conteúdo e os dois valores ficaram byte a byte.

### Conteúdo que o autor precisa decidir (o revisor não conserta)

- `book/cap10.md` L24: `a mediana final de quem aposta tudo é` e o valor `$1{,}322 \times 10^{-23}$` que
  abre a frase seguinte. A oração fica sem predicado; recompor mexe em número.
- `book/cap10.md` L21-25 contra a tabela L55-59: `37/13/nenhum terminam acima do início` e
  `37,7%/13,6%/0,01%` contam coisas diferentes sem que a diferença esteja dita.
- `book/cap10.md` L101-104 contra a tabela: `mediana 8,3` e `abaixo de 11 em 91%` contra `mediana = 8.39`
  e `0,95`.
- `book/cap10.md` L129: `crates/stochology-book/src/cap10.rs` — o caminho não está na lista de crates
  vivos do `AGENTS.md` (`crates/stochology-*`). Conferir antes de consolidar.
- `book/cap18.md` L52-53: `quem usou a taxa da cauda ajustada` fica sem predicado, e a frase seguinte
  (`O ajuste de cauda (POT/GPD) cobre 78%.`) troca de sujeito. Recompor obriga escolher o número que fecha
  a comparação.
- `book/cap10.md` L153: no bloco ` ```text `, `+0.00714 por semana (negativo: a ruína é certa)`. O valor é
  positivo e o parêntese diz negativo. Intocável por contrato: é saída de instrumento.
- `book/apendices.md`: dos 22 dois-pontos fora de tabela, a maioria é endereço (`https://…`) e referência
  bibliográfica (volume, número, páginas). Não é pontuação de prosa; ficam.

### Estilemas do autor preservados por decisão

- Frases curtas de remate: `Se o teorema está certo, ele roda.` (abertura), `Está na ponte.`,
  `Três lições em três linhas.`, `Dois argumentos independentes.` (cap03), `Não há ponto de virada.`,
  `O teste é direto.` (cap16), `Ninguém trapaceia.`, `Estava no denominador.`,
  `O AND das oito não perdoa.` (cap07), `A taxa de acerto não é própria.` (cap08),
  `Um portão que nunca falha não é portão (Capítulo 15).` (cap14),
  `A alegação da seguradora caiu.` (cap21).
- `regra própria` como termo do contrato (proibição 4): `cap08`, `cap13`, `cap14`, `cap15`, apêndices.
- Ecos que o capítulo usa de propósito: `**A previsão que muda o que prevê.**` (cap11, dois degraus),
  `a mesma informação mútua` (cap11), `o mesmo símbolo` (cap14, Itô × Stratonovich),
  `mesma entrada, mesma resposta` (cap13), `Seis relógios ligados ao mesmo tempo` (cap20).

## Releitura do revisor principal

Cada arquivo passou por `scripts/style/check_pass.py`: multissed de números igual, cercas e blocos
` ```text ` idênticos, marcadores da rampa presentes, piso de 700 palavras respeitado. Zero regressão nos
26 arquivos. Duas correções de rumo feitas na releitura:

1. `book/cap12.md` L211-212: a reescrita dizia `O expoente de cauda é uma estimativa da amostra, e a
   natureza não guarda constante correspondente.` Voltou ao sentido do autor:
   `O expoente de cauda é uma estimativa, e não uma constante da natureza.`
2. `book/cap02.md` L20-29: a frase que apresentava a tabela perdeu o valor `8 pacientes por hora`; o
   valor continua na tabela (`Sempre 8 por hora`) e no Degrau 3 (`todos com a demanda média de 8`).
   Decisão registrada, nenhum número novo entrou.

## Fecho da passada

### Correções que o revisor principal fez na releitura

1. `book/cap06.md` L128: `no lugardos 82% da alegação` (palavra colada, estrago de reescrita) →
   `no lugar dos 82% da alegação`.
2. `book/cap13.md` L257-258: linha órfã `cada uma.`, repetição do fecho do item 5 de **Exercícios.** →
   removida; o item volta a fechar uma vez só.
3. `book/cap06.md`: `valeriam o mesmo que` → `equivaleriam a`; `de um mesmo dia` → `de um dia`;
   `valem o mesmo que` → `equivalem a` (duas ocorrências); `pois mesmo o mundo` → `pois até o mundo`;
   `aprova o que não deveria` → `aprova aquilo que não deveria`. `mesmo/mesma` 6 → 1 no arquivo.
4. `book/cap12.md` L211-212: sentido do autor restaurado (ver a seção de releitura acima).
5. `book/cap02.md`, `book/cap05.md`: dois-pontos que sobravam do primeiro passo foram cortados na
   releitura; `book/cap02.md` fechou em 15 de prosa contra 30 de antes.

### Resíduo de `o que` e `próprio` (pendência do autor, não do revisor)

O resíduo ficou acima do alvo em capítulos cuja maior parte do padrão vive em lista fechada:
`book/cap06.md` (7), `book/cap09.md` (14, dos quais 6 em tabela ou título de bloco),
`book/cap16.md` (12), `book/cap19.md` (11, 5 em tabela), `book/cap21.md` (4),
`book/cap13.md` (5 `o que`, 5 `próprio`), `book/cap15.md` (5 `mesmo`, 2 `próprio`),
`book/cap10.md` (5 `mesmo`, 5 `o que`, 2 `próprio`). Baixar esses números exige reescrever título de
quadro, célula de tabela ou item de lista já contado no contrato: decisão do autor.

### Fora do escopo desta passada

`Cargo.toml`, `Cargo.lock` e os crates novos (`stochology-icp`, `stochology-drift`, `stochology-h0` a `stochology-h3`, `stochology-fairness`, hoje reunidos em `stochology-methods` e `stochology-horizons`)
mudaram no diretório de trabalho durante a sessão, em trabalho de outra frente. Nada disso é da revisão de prosa; a passada toca `book/*.md`, `scripts/style/` e `book/stochology-textbook.md`.

### Números do fecho

- Prosa do livro: 66.526 palavras antes, 66.554 depois. Cercas e blocos ` ```text ` intactos; os 24
  blocos conferidos por `scripts/book/check_labs.py` seguem em `0` divergentes.
- `scripts/book/build.py --check` exit 0 (5.593 linhas, 71.318 palavras). `scripts/book/check_book.py` exit 0.
  `scripts/style/check_pass.py` sobre os 26 arquivos: `0` regressões.

### Normalização declarada em 2026-10-17 (reorganização da árvore)

`scripts/style/check_pass.py` ganhou duas normalizações, e ambas são declaradas aqui porque mudam o que a
comparação contra `HEAD` enxerga:

1. **Caminhos migrados.** O mapa de renomeações da reorganização de 2026-10-17 (`dados-reais/` → `data/`,
   `fundo/` → `empirical/`, `governanca/` → `governance/`, `historico/` → `history/`, `artigo/` → `paper/`,
   `referencias/` → `references/`, `REGISTRO_EMPIRICO.md` → `docs/EMPIRICAL_REGISTRY.md`, `RESULTADOS_RUST.md` →
   `docs/RUST_RESULTS.md`, `book/gerar.py` → `scripts/book/build.py`, `PLAN.md` → `docs/PLAN.md`, entre outros) é
   aplicado aos **dois** lados da comparação. Um caminho migrado cancela; um número que mudou não cancela.
2. **Cerca comparada por conteúdo.** O corpo de cada cerca é comparado com espaços colapsados, porque o
   alinhamento de coluna de um instrumento não é uma medida. Número dentro da cerca continua sendo contado pelo
   balde de números, que não colapsa nada.

O que a normalização **não** afrouxa: contagem de números, presença dos cinco marcadores de degrau, piso de
palavras e os dois tetos de prosa corrente (`virgula_nao` e `travessao_fecha_com_e`) seguem estritos.

## Reescrita de 2026-10-18: conteúdo puro

Decisão da dona do projeto: o livro gastava prosa demais com metacomentário (caminhos, "este livro", o programa,
declarações de divergência, o bloco do eixo R repetido nos 21 capítulos). O livro é reescrito parte por parte,
com o conteúdo da teoria e sem autorreferência. Uma reescrita não é uma passada de prosa, então números e cercas
mudam por desenho, e a comparação contra `HEAD` reprovaria tudo.

Por isso `check_pass.py` passou a tratar os arquivos do livro em **modo reescrita**:

1. **Marcadores e piso.** Os cinco marcadores de degrau e o piso de 700 palavras seguem estritos.
2. **Procedência dos números.** Todo número da prosa corrente (fora das cercas e das linhas de tabela) precisa
   existir numa fonte (o livro montado na base, `paper/README.md`, `docs/PLAN.md`, `docs/EMPIRICAL_REGISTRY.md`,
   `references/VERIFICATION.md`) ou num bloco de saída do mesmo arquivo, que `check_labs.py` prende ao
   instrumento. Um número inventado reprova.
3. **Declarações de divergência** saem dos capítulos e vão para o Apêndice E (procedência dos números), com os
   dois valores e a causa.
