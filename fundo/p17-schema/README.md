# Ponto 17 — a cadeia de migrações do contrato

**Estado: fechada. O contrato da ficha agora é um arquivo, e a cadeia tem dois elos com os valores declarados.**

Até esta frente, a cadeia de migrações era código e estava **vazia**: o `PADRAO` dizia em Rust que a ficha nasceu
na 1.0 e que nenhuma migração existia. Agora ela é um arquivo — `crates/stochology-schema/migracoes.tsv` —
embutido no binário na compilação, com dois elos contínuos e a razão escrita em cada um:

| etapa | campo | valor | por quê |
| --- | --- | --- | --- |
| 1.0 → 1.1 | `resolucao` | `R0-agregada` | o campo nasceu com o Φ5; a migração não inventa pergunta mais fina que a declarada |
| 1.1 → 1.2 | `priori_elicitada` | `false` | o campo nasceu com a frente SHELF |

A versão da ficha passou de 1.0 para **1.2**, e o leitor da cadeia reprova o que não for cadeia: buraco entre
versões, operação desconhecida, etapa que anda para trás, linha curta.

**A demonstração, declarada como demonstração.** Não existe ficha real na 1.0 na árvore — as fichas são construídas
em código, na versão atual, e os formulários do SHELF são outro formato. O instrumento reconstrói a forma antiga
(tira os dois campos que a cadeia acrescenta, marca o documento como 1.0) e mostra a migração: os dois campos entram
com os valores declarados e o resto do documento fica intacto.

**Como reproduzir.**

~~~bash
cargo run --release --bin labs -- --schema
~~~

**O limite.** O histórico do arquivo da ficha tem um único commit, então a cadeia não é reconstruída do Git: os
dois elos são as duas mudanças de contrato que o repositório já declarava. Campo novo entra como declaração nova,
com o valor de acréscimo e a razão, e a versão sobe.
