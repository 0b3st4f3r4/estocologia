# Parecer reemitido 2 — Direito (regra de indenização trabalhista)

**Reemissão da releitura de 2026-10-16.** O parecer original é `h2g/saida/direito_teto_beneficio/resultados.json`, com SHA-256 `ba8a63e9924224a16c539ce7b00db3814a91d6fd2389721bd1f9679cf3d11d30`,
assinado em 2026-10-15T03:53:28Z e **retirado nesta releitura** (`governanca/RELEITURA.md`). O arquivo original
não é reescrito: ele volta do commit `1bfec905` e o hash dele continua sendo o da assinatura antiga. Este
documento é o parecer novo; o hash dele entra na seção "Releitura" de `governanca/ASSINATURAS.md`, ao lado do
antigo.

## Decisão (inalterada na reemissão)
- ação: **adotar — cortar o benefício semanal em 10% para encurtar os afastamentos**
- P(adotar compensa): 1,00
- escopo, o que **vale**: indenização por acidente de trabalho em estados dos EUA no início dos anos 1980, com mudanças moderadas do teto
- escopo, o que **não vale**: doenças ocupacionais de longa latência, cortes grandes, outros países e duração total sem benefício

## O que a reemissão acrescenta
- o bloco de governança, que não estava na decisão do original;
- o escopo declarado, na letra do original;
- a nota obrigatória, que os quatro pareceres com pessoas exigem.

## Bloco de governança (do `resultados.json` original)
- titulares: **sim**; quase-identificadores: `homem`, `casado`, `kentucky`; k-anonimato bruto: 63; dados reais (`wooldridge_injury.csv`); responsável humano: sil, assinatura pendente.

## Nota obrigatória — duas partes
*Escrita pelo facilitador a partir do material levantado (`governanca/GRUPOS.md` e o próprio
`resultados.json`) e **confirmada e assinada por sil em 2026-10-16**, com um microajuste de redação no primeiro
ponto.*

- **não olhou:** o efeito da mudança sobre quem está **abaixo** do teto (o parecer mede a duração do afastamento, não a recomposição de renda de quem o reajuste do teto não atinge); a incidência fora de Kentucky e Michigan; e o custo real da medida — o `C` do F1 é **inventado**, e a ressalva de que isto não é conselho de política acompanha o parecer desde a assinatura.
- **grupos afetados:** trabalhadores acidentados no trabalho cobertos pelo seguro em Kentucky (1980) e Michigan (1982), com a mudança incidindo no grupo `alta_renda` acima do teto; **nenhum** deles foi ouvido (`GRUPOS.md`, linha da decisão `direito_teto_beneficio`).
