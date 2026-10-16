# Parecer reemitido 17 — Saúde (seguro e uso de serviços, RAND HIE)

**Reemissão da releitura de 2026-10-16.** O parecer original é `h2g/saida/saude_rand/resultados.json`, com SHA-256 `4e9c87b011fc1e01b8faea4a66ed1bbd5454504082435a5b62a9eb5bc44c7d6c`,
assinado em 2026-10-15T03:53:28Z e **retirado nesta releitura** (`governanca/RELEITURA.md`). O arquivo original
não é reescrito: ele volta do commit `1bfec905` e o hash dele continua sendo o da assinatura antiga. Este
documento é o parecer novo; o hash dele entra na seção "Releitura" de `governanca/ASSINATURAS.md`, ao lado do
antigo.

## Decisão (inalterada na reemissão)
- ação: **adotar — trocar o cuidado gratuito por coparticipação de 25%**
- P(adotar compensa): 0,74125
- escopo, o que **vale**: famílias não idosas dos EUA nos anos 1970, planos com teto de desembolso e gasto médico coberto (sem odontologia e psicoterapia)
- escopo, o que **não vale**: idosos (Medicare), planos sem teto de desembolso, sistemas de saúde de outros países e **desfechos de saúde** — o parecer só olha gasto

## O que a reemissão acrescenta
- o limite **"só olha gasto"** em destaque, como parte do escopo e não como rodapé;
- a nota obrigatória;
- o registro de que o grupo excluído (idosos) não foi avaliado nem ouvido.

## Bloco de governança (do `resultados.json` original)
- titulares: **sim**; quase-identificadores: `mulher`, `local`; k-anonimato bruto: 1 103; dados reais (`sampleSelection_RandHIE.csv`).

## Nota obrigatória — duas partes
*Proposta escrita pelo facilitador a partir do material levantado (`governanca/GRUPOS.md` e o próprio
`resultados.json`); **confirmada e assinada por sil em 2026-10-16, na íntegra**, sem ajuste.*

- **não olhou:** qualquer desfecho de saúde (o parecer mede gasto, e o próprio escopo diz que saúde não está coberta); os idosos, que o escopo exclui por Medicare; e o sacrifício orçamentário das famílias que passam a pagar 25% — não há medida de renda nem de adesão a tratamento no parecer.
- **grupos afetados:** famílias não idosas dos EUA no RAND HIE (anos 1970), nos planos com teto de desembolso; **nenhum** grupo ouvido (`GRUPOS.md`, linha da decisão `saude_rand`).
