# Declaração do ponto 17: a cadeia de migrações do contrato

**Congelada em 2026-10-16T23:07:35Z.**

| arquivo | SHA-256 | o que é |
| --- | --- | --- |
| `crates/stochology-schema/migracoes.tsv` | `520f9e095c88e3693ab7d12bd8e7eff951f3e3ebd7910e5db792332c64d6f60b` | a cadeia declarada em arquivo; o contrato é este arquivo |
| `DECLARACAO.md` (esta) | — | as cinco declarações; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `b73c9cc48712c863eb38785a45293644aa1130361df0653e7058f34493f78265` | o resultado em uma página |
| `DECLARACAO.sha256` | — | os três hashes, em forma de `sha256sum` |

## Declaração 1 — a cadeia sai do código e vira arquivo

Até aqui o contrato da ficha era código: o `PADRAO` era uma cadeia **vazia** declarada em Rust, com a nota de que
a ficha nasceu na 1.0 e nenhuma migração existia. A partir daqui a cadeia é um **arquivo** versionado,
`crates/stochology-schema/migracoes.tsv`, embutido no binário na compilação (`include_str!`). Quem revisar o
contrato lê o arquivo; quem rodar o instrumento carrega o mesmo arquivo.

A versão da ficha vai de **1.0 para 1.2**, e a cadeia tem dois elos contínuos: 1.0 → 1.1 → 1.2. A leitura do
arquivo é uma função declarada (`cadeia_de_texto`), e ela reprova o que não for cadeia: buraco entre versões,
operação desconhecida, etapa que anda para trás e linha curta.

## Declaração 2 — os dois valores de acréscimo, com a razão

| etapa | campo | valor declarado | por quê |
| --- | --- | --- | --- |
| 1.0 → 1.1 | `resolucao` | `R0-agregada` | o campo nasceu com o Φ5. A migração **não inventa pergunta mais fina do que a declarada** no documento: um documento que não fala de resolução é lido no grão mais grosso |
| 1.1 → 1.2 | `priori_elicitada` | `false` | o campo nasceu com a frente SHELF. Um documento anterior não tem priori elicitada de painel, e `false` é o que ele é |

Os dois valores são os únicos acrescentados pela cadeia, e cada um tem a razão escrita no próprio arquivo.

## Declaração 3 — a reconstrução, declarada como reconstrução

**Não existe ficha real na versão 1.0 na árvore.** As fichas do repositório são construídas em código, na versão
atual, e os formulários do SHELF são outro formato (protocolo de elicitação, não documento de ficha). O que o
instrumento demonstra é a **reconstrução** da forma antiga: pega uma ficha completa, tira dela os dois campos que a
cadeia acrescenta, marca o documento como 1.0 e manda migrar. A demonstração é honesta sobre isso — ela mostra o
mecanismo da cadeia, e não um artefato histórico que não existe.

**Limite do que a cadeia cobre.** O histórico do arquivo da ficha tem um único commit (a consolidação em Rust, com
a poda do Python), então a cadeia não pode ser reconstruída campo a campo a partir do Git: os dois elos são as duas
mudanças de contrato que o próprio repositório declara (o Φ5 e a frente SHELF). Se aparecer outro campo novo, ele
entra como declaração nova, com o valor de acréscimo e a razão, e a versão da ficha sobe.

## Declaração 4 — o K desta frente

Os dois valores de acréscimo são escolha de especificação, feita uma vez e aqui declarada: **K = 1**. Mudar um
deles depois de ver qualquer documento migrado é declaração nova, com hash próprio, e conta no K.

## Declaração 5 — o que isto não muda

Nenhuma ficha da árvore é anterior à versão atual, e nenhum número publicado muda: as 12 fichas do binário
`portoes` são construídas em código, já na versão corrente, e a validação delas não passa pela cadeia. O que a
frente entrega é o **contrato em arquivo** para o que vier: documento externo antigo, campo novo, ou mudança de
forma declarada com valor de acréscimo.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-schema` | 13 testes aprovados, entre eles os quatro novos: a cadeia do arquivo é contínua e termina na versão da ficha, a cadeia com buraco ou operação desconhecida reprova, o documento da 1.0 migra com os padrões declarados e o resto intacto, e a versão atual não passa pela cadeia (0) |
| `labs --schema` | a versão 1.2, os dois elos com a nota de cada um, e a demonstração: um documento da 1.0 migra, ganha `resolucao = R0-agregada` e `priori_elicitada = false`, e valida (0) |
| `python3 book/gerar_resultados.py --check` | `RESULTADOS_RUST.md` atualizado, com o bloco do contrato (0) |
