# Declaração K = 3 do H2g: a pergunta do decisor no pacote de domínio

Escrita em 2026-10-15T19:30:11Z (UTC), **antes** de tocar em qualquer `pacote.yaml`, em qualquer validador e antes de rodar
qualquer teste novo. O SHA-256 deste arquivo e a hora UTC do registro ficam em
`DECLARACAO_K3_pergunta_do_decisor.sha256`.

Este arquivo **não substitui** `DECLARACAO_K3.md` (a cópia do H2d que rege a regra de saturação, a unidade da
célula no Sachs e o efeito fixo de estado no cinto): aquela declaração está commitada e não é reescrita. Esta é a
declaração da rodada K = 3 do H2g para o ponto da auditoria, em arquivo próprio.

## Por que existe

A auditoria de fundo deixou em aberto, com nome, a **pergunta explícita na ficha contra o erro de tipo III**
(`../estocologia-livro-didatico.md`, seção 14.4): "a pergunta que o modelo responde é a que o decisor fez?".
O pacote de domínio do F7 declara pergunta, alegação, desfecho, decisão e termo de escopo, mas nada obriga o
texto a amarrar a pergunta do decisor à ação que a decisão muda. Esta declaração fecha esse ponto.

**Antes de mudar nada, leu-se e conferiu-se:**

- `h2g/portoes.py`, G1, **não compara hash de pacote nem hash de arquivo de entrada**. Ele roda `pytest` na raiz
  do H2g e, com `--reproducao`, roda `rodar.py` de novo num diretório temporário e compara o **conteúdo de
  `saida/*/resultados.json`** (chave a chave, fora as voláteis de `VOLATEIS = {"segundos"}`, floats arredondados a
  8 casas). A comparação é de saída publicada, não do que entrou.
- `h2g/estocologia/dominio.py:validar(meta)` devolve `falta <k>` para cada chave ausente em `OBRIGATORIOS`, e
  `carregar()` levanta `PacoteInvalido` quando a lista não está vazia. Nada em `validar` entra em soma nenhuma, e
  `funil.py` copia para `resultados.json` só campos escolhidos (`dominio`, `area`, `titulo`, `alegacao`,
  `ingestao`, `K = tentativas_de_especificacao`, os filtros): chave nova do meta **não** vaza para a saída.
- Conclusão lida antes de escrever código: acrescentar campo obrigatório mexe no `pytest` (tem de estar preenchido
  em todo pacote, senão o G1 falha) e **não** mexe em número publicado.

## O campo novo

`pergunta_do_decisor` — texto, obrigatório no `pacote.yaml` (entra em `OBRIGATORIOS`), na chave de topo.

Forma exigida, três partes nesta ordem:

    Quem decide: <quem>. Ação que a decisão muda: <decisao.acao, copiada literalmente>. A pergunta do decisor é: <a pergunta que essa decisão usa>?

Regra de validação (implementada em `estocologia/dominio.py`, função `_checar_pergunta_do_decisor`, chamada por
`validar`):

1. tem de ser texto, `strip()` não vazio, com pelo menos 60 caracteres;
2. tem de **nomear quem decide**: a etiqueta `Quem decide:` seguida de 3 a 120 caracteres antes da etiqueta
   seguinte; esse trecho não pode ser genérico — recusam-se `o decisor`, `os decisores`, `o usuário`, `o cliente`,
   `o modelo`, `os dados`, `a decisão`, `quem decide`, `alguém`, `ninguém` (sem distinguir maiúsculas);
3. tem de **nomear qual ação muda**: a etiqueta `Ação que a decisão muda:` seguida, literalmente, do texto de
   `decisao.acao` do próprio pacote;
4. tem de ser pergunta: conter `?` no fim.

Recusa, em resumo: pacote sem o campo, texto genérico (sem quem decide), texto sem a ação literal e texto sem
pergunta. O validador do `h2` (`h2/estocologia/dominio.py`, cópia própria) recebe a mesma função e a mesma regra.

## Escopo da propagação

- **18 pacotes do `h2g/`**: `dominios/*/pacote.yaml`, todos.
- **12 pacotes do `h2/`**: `dominios/*/pacote.yaml`, todos.
- **`h0-lab0/ficha.yaml`**: a ficha do H0 (posto de saúde), onde a pergunta de tipo III tem caso concreto
  (o gestor decide reforçar nos dias de surto lendo a espera média?). Na ficha não existe bloco `decisao`: a ação
  é copiada de `termo_de_escopo.regra_de_decisao`, e nenhum validador do H0 é tocado.
- Fora disso, nada: nenhum outro horizonte (`h1`, `h2b`–`h2f`, `h3`, `fundo/`) tem `pacote.yaml` ou validador
  tocado.

## O texto de cada pacote

O texto não é inventado. O **quem decide** sai do que o próprio pacote já declara (`decisao`, `decisao.nota`,
`termo_escopo.vale_para`, `titulo`); a **ação** é copiada literalmente de `decisao.acao`; a pergunta final é a
pergunta do pacote reduzida ao desfecho que a decisão usa. Nenhum número, custo, priori, limiar, fonte ou
vocabulário novo de especialista é acrescentado.

## Numeração

Conta como **K = 3 do H2g** — o mesmo K = 3 que o H2g já herda de `DECLARACAO_K3.md`. O campo é **metadado do
pacote**: `tentativas_de_especificacao` **não** muda em domínio nenhum (fica 1 ou 3 como está) e nenhuma entrada de
`historico_especificacoes` é acrescentada. Mudar o contador mudaria o campo `K` de `resultados.json`, que é o que a
prova abaixo proíbe.

## Prova exigida

1. `python3 -m pytest h2g/tests -q` verde (27 testes antes, mais os novos).
2. `python3 -m pytest h2/tests -q` verde (9 antes, mais os novos, se houver).
3. Nenhum **número publicado** muda: os 18 arquivos `h2g/saida/*/resultados.json` e os 12 `h2/saida/*/resultados.json`
   ficam byte a byte iguais — SHA-256 conferido antes e depois, e contra a tabela de `governanca/ASSINATURAS.md` no
   caso do H2g.
4. `python3 -m pytest h0-lab0/tests -q` verde: a ficha do H0 continua cumprindo `h0-lab0/estocologia/esquema.py`,
   que não é tocado.

Se a única forma de avançar em algum passo for mudar número publicado, para-se e relata-se, como manda a regra da
casa.
## Reemissão (2026-10-15T22:10:32Z)

Ponteiro corrigido: a citação acima apontava para a “seção 14.6” do livro didático, que não existe — a lista dos pontos
em aberto da auditoria de fundo é a **seção 14.4**. Nada mais mudou: o critério, os números, o K = 3 e a hora do registro
original continuam os mesmos, e o trabalho descrito aqui continua sendo o que foi declarado antes de rodar.

O SHA-256 anterior deste arquivo era `a9fe5fed1720eca37f211ea78361e778e0a8321788f8ae8ee74e2462388e53bd`, registrado em 2026-10-15T19:30:11Z, e está
preservado em `DECLARACAO_K3_pergunta_do_decisor.sha256`.
