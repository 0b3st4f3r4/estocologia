# Declaração K = 2 do H1 real: o mecanismo da ausência no contrato de dados do F2

Escrita em 2026-10-15T19:52:03Z (UTC), **antes** de tocar em `h1-real/projetos/*.py`, em `h1-real/portoes.py`, em
`h2g/estocologia/dominio.py` ou em qualquer teste. O SHA-256 deste arquivo e a hora UTC do registro ficam em
`DECLARACAO_F2_AUSENCIA.sha256`.

## O ponto da auditoria que isto fecha

A auditoria de fundo deixou em aberto, com nome: falta ao F2 a pergunta "**por que o dado está ausente?**"
(`../estocologia-livro-didatico.md`, seção 7.2 e a lista da seção 14.6 — "um campo no contrato de dados do F2
para o mecanismo da ausência (MCAR, MAR ou MNAR), com análise de sensibilidade quando a ausência pode depender
do próprio valor"). Hoje o `contrato()` declara colunas, faixas, `nulo` e chave; o `portao2` recusa nulo onde o
contrato não o admite; e nada classifica o mecanismo da ausência. Esta declaração fecha esse ponto.

## O campo novo (opcional, de topo, no contrato de dados)

    ausencia:
      mecanismo: MCAR | MAR | MNAR | NAO_DECLARADO
      colunas: [<nomes de coluna do próprio contrato>]
      sensibilidade: {feito: <bool>, metodos: [<texto curto>]}

- `mecanismo`: a classe da ausência que o contrato admite (livro, Cap. 3). `NAO_DECLARADO` é a ausência sem
  classificação e existe para poder ser recusada, não para ser usada por omissão.
- `colunas`: as colunas cuja ausência a classificação cobre. Lista vazia = nenhuma coluna admite valor ausente
  (que é o caso hoje nos três contratos: `portao2` já prova zero nulos na loja, e os CSV de origem não têm
  célula vazia).
- `sensibilidade`: se a sensibilidade à premissa de ausência foi estudada e com que métodos. Regra declarada:
  `MAR` e `MNAR` **exigem** `feito: true` com métodos; `feito: false` exige `metodos` vazio.
- Chave `nota` (texto) é permitida no bloco, para dizer em uma frase o que a ausência é naquele contrato.

## Regra do portão 2 (`h1-real/portoes.py`, função `checar_ausencia`, chamada por `portao2`)

Recusa, em cinco cláusulas:

1. contrato sem `ausencia` ou com o bloco em tipo errado;
2. `mecanismo` fora de `{MCAR, MAR, MNAR, NAO_DECLARADO}`;
3. `colunas` que não seja lista de textos, ou que cite coluna que o contrato não tem;
4. `sensibilidade` que não seja `{feito: bool, metodos: [...]}`, ou incoerente (`feito: true` sem método,
   ou `feito: false` com método);
5. `MAR` ou `MNAR` sem `sensibilidade.feito: true`.

## O que cada um dos três contratos vai declarar (dito antes de escrever)

Nenhum dos três tem coluna que admita nulo, e nenhuma célula falta na fonte (conferido antes: `nsw_experimental`,
`cps_observational`, `djia_1885_2023` e `danish_fire` não têm campo vazio em nenhuma coluna). Então
`colunas: []` e `sensibilidade: {feito: false, metodos: []}` nos três, com o mecanismo na `nota`:

- **a `djia_pregoes`**: `mecanismo: MCAR`. A ausência é de **linha**, não de célula: não há pregão nos feriados
  nem nos 135 dias corridos de jul–dez/1914 (fechamento da bolsa). O motivo é do calendário da bolsa, não do
  valor do retorno; o caso de 1914 já está declarado como limite no cabeçalho (`fisica_do_ruido`) e nenhum
  pregão é imputado nem descartado.
- **b `nsw_cps_lalonde`**: `mecanismo: MCAR`. Nada falta. Renda zero em `re74`/`re75` é **valor observado**
  ("sem renda no ano"), não ausente: entra no desenho com indicador próprio (`r74 == 0`). A distinção entre MAR e
  MNAR, que só importaria com célula vazia, não é testável nos dados observados (Cap. 3) e fica declarada como
  premissa, não como fato.
- **c `sinistros_incendio_dinamarca`**: `mecanismo: MCAR`. O registro da Copenhagen Re (> 1 M DKK, 1980–90) é
  completo. Mês sem sinistro grande é zero observado, não dado ausente.

Se algum dia entrar célula vazia em um dos contratos, o mecanismo tem de ser reclassificado e, para `MAR`/`MNAR`,
com análise de sensibilidade — é o que a cláusula 5 passa a exigir.

## O lado do `h2g`

`h2g/estocologia/dominio.py:checar_tabela` passa a:

1. **contar ausentes por coluna** na tabela do domínio;
2. ler o bloco opcional `ausencia` do `pacote.yaml` (mesma forma do contrato; `mecanismo` ausente = `NAO_DECLARADO`);
3. **recusar `NAO_DECLARADO` quando alguma coluna tem valor ausente**;
4. recusar coluna com valor ausente que não esteja em `ausencia.colunas`;
5. recusar `MAR`/`MNAR` sem `sensibilidade.feito`.

Medido **antes** de escrever a regra, nos 18 domínios, com `preparar()` de verdade: nenhuma das 18 tabelas tem
**um único valor ausente** em qualquer coluna (13 colunas em 17 domínios, 12 em dois). Logo a regra é **vacuosa**
no material de hoje: não pode mudar status, `resultados.json` nem portão nenhum, e nenhum pacote do `h2g`
precisa do bloco `ausencia` para continuar passando. Ela passa a morder no dia em que aparecer ausência.

O `h2` **não** recebe esta parte: o `h2` é cópia congelada e a regra nova entra só no `h2g`, como é o costume do
repositório (cada horizonte é uma cópia; a mudança mora no horizonte novo). A partir daqui
`h2/estocologia/dominio.py` e `h2g/estocologia/dominio.py` deixam de ser idênticos — o `h2` fica com a regra do
`pergunta_do_decisor` (parte H) e sem a da ausência.

## Escopo: o que muda e o que não muda

Muda:

- `h1-real/projetos/a_real.py`, `b_real.py`, `c_real.py`: `contrato()` emite o bloco `ausencia`;
- `h1-real/portoes.py`: `checar_ausencia` + chamada no portão 2;
- `h1-real/tests/test_f2_ausencia.py` (novo): os três contratos, e a recusa de mecanismo inválido,
  de coluna fora do contrato e de `MNAR` sem sensibilidade;
- `h2g/estocologia/dominio.py`: `checar_tabela` com a contagem de ausentes e as recusas;
- `h2g/tests/test_k3_ausencia.py` (novo): tabela com ausente e `NAO_DECLARADO` é recusada; com mecanismo
  declarado passa; `MAR` sem sensibilidade é recusado;
- **saída regenerada**: `h1-real/saida/a/datacontract.yaml`, `b/`, `c/` ganham o bloco `ausencia` (é a
  regeneração esperada do contrato; nenhum outro arquivo de saída é tocado).

Não muda:

- nenhum `resultados.json` de horizonte nenhum;
- `h1-real/saida/portoes.json` **não** é reescrito: é saída publicada e a linha do portão 1 carrega a contagem e o
  tempo da suíte, que não são número de resultado; a checagem nova é exercitada importando `portao2` e pelo teste
  novo, sem reescrever o arquivo;
- o `barramento.jsonl` do H1 real não é reescrito: a ficha do `datacontract` gravada na execução original
  (`2026-10-15`) guarda o contrato **sem** o campo, e passa a divergir do `datacontract.yaml` regenerado. Fica
  declarado como consequência: a ficha do F2 só volta a casar com o arquivo numa nova execução do `rodar.py`,
  que **não** foi feita (e não devia ser, para não mexer em número publicado). A cadeia de hash do barramento em
  si não é rompida: `verificar()` confere o encadeamento, não o conteúdo atual do contrato.

## Numeração

Conta como **K = 2 no H1 real** — a segunda tentativa de especificação do horizonte, e a primeira que mexe no
contrato de dados do F2 (o `tentativas_de_especificacao` que aparece em `f2` de `resultados.json` é a constante
de `estocologia/agentes.py` e **não** é tocado: mexer nele mudaria número publicado). No lado do `h2g` a
checagem entra na rodada **K = 3** já declarada para o H2g nesta mesma tarefa
(`h2g/DECLARACAO_K3_pergunta_do_decisor.md`), sem tocar em contador nenhum.

## Prova exigida

1. `python3 -m pytest h1-real/tests -q` verde (40 testes antes, mais os novos).
2. `python3 -m pytest h2g/tests -q` verde (31 antes desta parte, mais os novos).
3. Nenhum número publicado muda: SHA-256 dos três `h1-real/saida/*/resultados.json` e dos 18
   `h2g/saida/*/resultados.json` conferidos antes e depois, contra a tabela de `governanca/ASSINATURAS.md`
   (que é o registro assinado por sil).
4. Os únicos arquivos de saída regravados são os três `h1-real/saida/*/datacontract.yaml`, e a causa é a
   regeneração do contrato pela própria esteira.

Se a única forma de avançar em algum passo for mudar número publicado, para-se e relata-se.
