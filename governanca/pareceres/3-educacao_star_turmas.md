# Parecer reemitido 3 — Educação (turmas menores no STAR)

**Reemissão da releitura de 2026-10-16.** O parecer original é `h2g/saida/educacao_star_turmas/resultados.json`, com SHA-256 `f5b4307e4cbd103b96e7e0750c24e1758ec29995ae4eb7fa2bca28bc39477994`,
assinado em 2026-10-15T03:53:28Z e **retirado nesta releitura** (`governanca/RELEITURA.md`). O arquivo original
não é reescrito: ele volta do commit `1bfec905` e o hash dele continua sendo o da assinatura antiga. Este
documento é o parecer novo; o hash dele entra na seção "Releitura" de `governanca/ASSINATURAS.md`, ao lado do
antigo.

## Decisão (inalterada na reemissão)
- ação: **manter — **não** adotar a redução das turmas das séries iniciais para ~15 alunos**
- P(adotar compensa): 0,47275 (abaixo de 0,50, o parecer recomenda não adotar)
- escopo, o que **vale**: séries iniciais (jardim a 3ª) de escolas públicas dos EUA, turmas de 13 a 17 contra 22 a 25
- escopo, o que **não vale**: séries posteriores, redução em escala estadual com professores menos experientes e outros países

## O que a reemissão acrescenta
- o bloco de justiça por etnia com as taxas-base por grupo, que a decisão do original não trazia;
- o escopo declarado, na letra do original;
- a declaração de que o painel SHELF deste domínio segue ausente;
- a nota obrigatória.

## Bloco de governança (do `resultados.json` original)
- titulares: **sim**; quase-identificadores: `menina`, `etnia`, `almoco_gratuito`; k-anonimato bruto: 9; dados reais (`AER_STAR.csv`); bloco de justiça por `etnia` com taxas-base: `afam` n = 4 524, `cauc` n = 8 632, `outra` n = 80.

## Nota obrigatória — duas partes
*Proposta escrita pelo facilitador a partir do material levantado (`governanca/GRUPOS.md` e o próprio
`resultados.json`); **confirmada e assinada por sil em 2026-10-16, na íntegra**, sem ajuste.*

- **não olhou:** o que acontece com o grupo `outra` (n = 80) quando a turma encolhe — é pequeno demais para sustentar intervalo; a implementação em escala estadual com professores menos experientes; e a priori elicitada, que **não existe** neste domínio (nenhum educador foi consultado; a priori é a de literatura do p05).
- **grupos afetados:** crianças do jardim à 3ª série das escolas públicas do Tennessee no STAR (11 598 alunos, 80 escolas), por etnia declarada: `afam` 4 524, `cauc` 8 632, `outra` 80; **nenhum** grupo ouvido (`GRUPOS.md`, linha da decisão `educacao_star_turmas`).
