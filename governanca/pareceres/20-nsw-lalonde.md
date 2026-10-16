# Parecer reemitido 20 — H1·B NSW (LaLonde): treino profissional contra controles do CPS

**Reemissão da releitura de 2026-10-16.** O parecer original é `h1-real/saida/b/resultados.json`, com SHA-256 `5aaa954e56ede16cd7d38360bd923b3c75dea201f46464e7bb398c36f436aa58`,
assinado em 2026-10-15T03:53:28Z e **retirado nesta releitura** (`governanca/RELEITURA.md`). O arquivo original
não é reescrito: ele volta do commit `1bfec905` e o hash dele continua sendo o da assinatura antiga. Este
documento é o parecer novo; o hash dele entra na seção "Releitura" de `governanca/ASSINATURAS.md`, ao lado do
antigo.

## Decisão (inalterada na reemissão)
- ação: **financiar o treinamento, com a regra "financiar se o ATT causal > C/L"**
- P(adotar compensa): ATT causal mediano 0,0963, IC90 de 0,0088 a 0,1879
- escopo, o que **vale**: homens elegíveis ao NSW, 1976–77, 10 cidades dos EUA, classe de referência com 84% de negros; disjuntor declarado: parar se a fração de negros de um lote se afastar mais de 0,25 de 0,84
- escopo, o que **não vale**: outros programas de treino, outras coortes e outros países

## O que a reemissão acrescenta
- o bloco de governança que **faltava** no original: classe de referência, grupos por raça declarada e o disjuntor de 0,25 sobre a fração de negros;
- a declaração de que o custo ilustrativo impede a recomendação de campo;
- a nota obrigatória.

## Bloco de governança (do `resultados.json` original)
- titulares: **o `resultados.json` do H1 real não tem bloco `governanca`** — não há `tem_titulares`, não há quase-identificadores, não há k-anonimato neste arquivo; o que existe é o bloco `f6` (grupos por raça declarada e taxas-base) e `f5.aplicavel_a_titulares: true`.

## Nota obrigatória — duas partes
*Proposta escrita pelo facilitador a partir do material levantado (`governanca/GRUPOS.md` e o próprio
`resultados.json`); **pendente de confirmação do signatário**, que é quem escreve a nota dele.*

- **não olhou:** o custo real por participante: o `C` é **ilustrativo**, de modo que a regra de decisão não fecha recomendação de campo; o bloco de governança, que simplesmente não existia; e o desfecho de emprego de longo prazo, que o parecer não mede.
- **grupos afetados:** 185 participantes do treino do NSW e 15 992 controles do CPS, por raça declarada (`negro`, `hispanico`, `outro`); **nenhum** grupo ouvido (`GRUPOS.md`, linha da decisão `h1-real/saida/b`).
