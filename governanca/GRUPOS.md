# Grupos afetados ouvidos

O livro pede que o programa registre **o que a auditoria não pôde olhar e quais grupos afetados foram ouvidos,
ou declare que nenhum foi** (seção 14.2, objeção "O auditor é parte interessada"). Este arquivo é esse registro.
Uma linha por decisão que envolve pessoas: as quatro dos 21 pareceres e os três fluxos de prisões do H3.

| decisão | grupos afetados | ouvidos | por quem | quando | onde está o registro |
|---|---|---|---|---|---|
| `h2g/saida/direito_teto_beneficio` — cortar o benefício semanal em 10% para encurtar os afastamentos | trabalhadores acidentados no trabalho cobertos pelo teto, Kentucky (1980) e Michigan (1982); o corte atinge o grupo `alta_renda` (acima do teto) | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h2g/direito_teto_beneficio`), `governanca/pareceres/h2g-direito_teto_beneficio.md`, `governanca/RELEITURA.md` e `fundo/p12-shelf/elicitacoes/direito_teto_beneficio.yaml` (origem histórica em `h2f/` e `h2g/`, aposentados no commit `1bfec905`, `DECLARACAO_REMOVIDO_PYTHON.md`) |
| `h2g/saida/educacao_star_turmas` — reduzir as turmas das séries iniciais para ~15 alunos | crianças do jardim à 3ª série das escolas públicas do Tennessee no STAR (11 598 alunos, 80 escolas), por etnia declarada: `afam` 4 524, `cauc` 8 632, `outra` 80 | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h2g/educacao_star_turmas`), `governanca/pareceres/h2g-educacao_star_turmas.md` e `governanca/RELEITURA.md` (origem histórica em `h2f/` e `h2g/`, aposentados em `1bfec905`) |
| `h2g/saida/saude_rand` — trocar o cuidado gratuito por coparticipação de 25% | famílias não idosas dos EUA no RAND HIE (anos 1970), nos planos com teto de desembolso | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h2g/saude_rand`), `governanca/pareceres/h2g-saude_rand.md`, `governanca/RELEITURA.md` e `fundo/p12-shelf/elicitacoes/saude_rand.yaml` (origem histórica em `h2f/` e `h2g/`, aposentados em `1bfec905`) |
| `h1-real/saida/b` (NSW/LaLonde) — comparar o treino profissional do NSW com controles do CPS | 185 participantes do treino do NSW e 15 992 controles do CPS, por raça declarada (`negro`, `hispanico`, `outro`) | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h1-real/b`), `governanca/pareceres/h1-real-b.md`, `governanca/RELEITURA.md` e `crates/stochology-book/src/cap17.rs` (origem histórica em `h1-real/`, aposentado em `1bfec905`) |
| `h3` fluxo `prisoes` — marcar o detido para revisão da liberação (Toronto 1997–2002) | 5 226 detidos, por cor percebida (`colour`, atributo protegido do contrato do fluxo) | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h3/prisoes`), `governanca/pareceres/h3-prisoes.md`, `governanca/RELEITURA.md` e `fundo/p14-h3/registro.tsv` (origem histórica em `h3/`, aposentado em `1bfec905`) |
| `h3` fluxo `prisoes20` — marcar a pessoa presa para revisão do registro na delegacia (Toronto 2020–2021) | 65 276 prisões, por cor percebida; 37 mil pessoas distintas | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h3/prisoes20`), `governanca/pareceres/h3d-prisoes20.md`, `governanca/RELEITURA.md` e `fundo/p14-h3/registro.tsv` (origem histórica em `h3/`, aposentado em `1bfec905`) |
| `h3` fluxo `prisoes_ri` — marcar o motorista para revisão da prisão na parada (Rhode Island 2005–2015, cofre 2013–2015) | 86 113 paradas, por cor percebida; sem identificador de motorista nem de policial | **nenhum** | — | — | `fundo/p13-escuta/registro.tsv` (`h3/prisoes_ri`), `governanca/pareceres/h3d-prisoes_ri.md`, `governanca/RELEITURA.md` e `fundo/p14-h3/registro.tsv` (origem histórica em `h3/`, aposentado em `1bfec905`) |

**Nenhum dos sete grupos afetados foi ouvido, em nenhuma das sete decisões.** Não há no repositório registro de
consulta, entrevista, painel dos grupos afetados ou consentimento.

## O lacre (2026-10-16)

O programa parou de tratar a escuta como pendência de opinião. O protocolo do ponto 13 está congelado com hash
([`fundo/p13-escuta/`](../fundo/p13-escuta/DECLARACAO.md)), o registro em máquina tem as sete decisões e o
instrumento `labs --escuta` dá o veredito: com **0 de 7** grupos ouvidos e cobertura mínima de **2 de 7**, a caixa
fica **lacrada**. As sete decisões ficam como análise de sensibilidade ($n = 1$) e **não** viram diretriz de política
pública nem decisão de portão F8 até a cobertura mínima ser atingida, ou até **2026-12-15**; vencido o prazo sem
cobertura, o artefato fica arquivado como exercício metodológico de sensibilidade, com o lacre em hash.

A coluna "ouvidos" desta tabela continua **nenhum** nas sete linhas, e é isso que o instrumento confere: nenhuma
decisão pode se dizer emitida sem alguém ouvido — o caso sintético que mente é reprovado no relatório do portão.

## O que a auditoria não pôde olhar

- **Quem autoriza.** Os 21 pareceres foram assinados por sil em 2026-10-15T03:53:28Z
  (`governanca/ASSINATURAS.md`) e relidos um a um em 2026-10-16 (`governanca/RELEITURA.md`), e sil confirmou a troca
  de pergunta do H3d em 2026-10-15T04:08Z (registrado em `REGISTRO_EMPIRICO.md` §4 e `fundo/p14-h3/README.md`).
  Assinatura do responsável não é escuta do grupo afetado: o responsável está do lado de quem decide usar o artefato,
  não de quem paga o erro.
- **O painel humano de especialistas de fora nunca foi concluído.** Os três domínios do `h2g` nasceram declarando
  `shelf.painel: "literatura publicada; nenhum especialista humano consultado"` (`fundo/p05-prioris/README.md`;
  livro, seção 9.7); em 2026-10-16 o primeiro especialista interno (`E1`) fechou cinco domínios em
  `fundo/p12-shelf/elicitacoes/` ($n = 1$), mas faltam as outras duas pessoas por domínio e o especialista de fora do
  programa, e nenhum membro dos grupos afetados foi ouvido.
- **As decisões não chegaram a ninguém.** Nos três fluxos de prisões o artefato emite uma marca para revisão; quem
  seria marcado — o detido, a pessoa presa, o motorista abordado — nunca viu o artefato nem a sua taxa de erro.
- **Sem consentimento nem comitê.** Os dados são públicos e antigos (Toronto 1997–2002 e 2020–2021, Rhode Island
  2005–2015, STAR, RAND HIE, NSW, Kentucky e Michigan dos anos 1980). O F5 faz k-anonimato e orçamento de ε por
  pessoa em dois fluxos, o que mede risco de reidentificação, e não consulta.
- **Pergunta de substituição.** Em Rhode Island a pergunta mudou de "liberação depois da prisão" para "prisão numa
  parada de trânsito". A troca foi confirmada por sil, e não pelo grupo que passou a ser o alvo da decisão.

## Como isto foi apurado

Não há protocolo de consulta concluído no programa, então o registro saiu de uma busca de qualquer rastro de escuta
no repositório inteiro. Os únicos achados ligados a pessoas são os campos `shelf.painel`, os `tem_titulares` e os
quase-identificadores — declarações sobre dados, não sobre grupos ouvidos.

```
grep -rni "grupos afetados|grupo afetado|ouvidos|partes afetadas" --include=*.md .
grep -rni "painel de especialistas|entrevista|consulta pública|stakeholder|grupo de interesse" --include=*.md .
grep -rn "titular|tem_titulares" --include=*.md --include=*.tsv --include=*.yaml .
```

## Bloco G: a inversão da métrica

A pergunta do Φ4 — inverter a métrica escolhida e mostrar quem passaria a pagar — foi rodada nos três fluxos de
prisões. No registro histórico de `h3/saida/<fluxo>/justica_invertida.json` (preservado no manifesto do commit
`1bfec905` e consolidado nesta tabela e em `REGISTRO_EMPIRICO.md`), a métrica **escolhida** é a taxa de falso
positivo do flag, P(marcado | não sofre o desfecho de interesse), e a **invertida** é a taxa de falso negativo,
P(não marcado | sofre o desfecho). O grupo menos favorecido sai da maior taxa-base do desfecho, e o pagador é o
de maior taxa sob a métrica; empate é o que tem intervalos de Wilson de 95% sobrepostos. A varredura de limiares é
0,05 a 0,95 de 0,05 em 0,05, mais o limiar declarado do fluxo.

| fluxo | limiar | FNR do grupo menos favorecido (no limiar do fluxo) | quem paga sob a métrica escolhida | sob a métrica invertida | troca no limiar? |
|---|---|---|---|---|---|
| `prisoes` (1997–2002, por ano) | 0,70 | 0,64 / 0,68 / 0,66 / 0,67 / 0,44 (1998 a 2002) | `Black` em 1998–2001, empate em 2002 | empate em todos os anos | não |
| `prisoes20` (por semestre) | 0,70 | 0,062 / 0,056 / 0,048 / 0,071 | empate nos quatro semestres | empate nos quatro | não |
| `prisoes_ri` (cofre 2013–2015) | 0,90 | 0,927 / 0,906 / 0,850 | `Black` em 2014, empate em 2013 e 2015 | empate nos três | não |

O achado que interessa: **em Toronto 1997–2002, o grupo menos favorecido é o que tem a MENOR taxa de falso
negativo**. Com o limiar declarado de 0,70, o FNR do `Black` é 0,64 em 1998 contra 0,70 do `White`; a métrica
escolhida (FPR) põe o `Black` pagando (0,196 contra 0,105) e a invertida (FNR) tira o `Black` e deixa o empate ou
o `White`. Quem é marcado com mais frequência sem sofrer o desfecho é o `Black`; quem é perdido com mais
frequência é o `White`. Trocar a métrica troca quem paga, e em 1998–2002 isso acontece num limiar do grid em
1998 (0,85), em 2000 (0,50) e em 2001 (0,80), e em nenhum em 1999 e 2002 — sempre acima do limiar declarado de
0,70. Em Toronto 2020–2021 a troca aparece no semestre 2020-S2, em 5 dos 20 limiares (0,35 a 0,55), e em Rhode
Island em nenhum: lá os intervalos de Wilson se sobrepõem em quase toda a varredura, e a inversão não troca o
pagador. O padrão de fundo é o mesmo nos três fluxos: o `Black` tem o FPR maior e o FNR menor.

**K = 2 neste bloco.** A declaração 1 nomeava "empate" quando os intervalos das taxas-base se cruzavam, e isso
deixava Toronto 2001 e 2002 sem o número pedido (as taxas-base são 21% e 19%). A declaração 2 nomeia o grupo pela
taxa-base pontual e carrega o empate ao lado. As duas ficaram gravadas, lado a lado, com os hashes abaixo.

| fluxo | declaração 1 | declaração 2 | `governanca.py` na declaração | na rodada |
|---|---|---|---|---|
| `prisoes` | `dc2624f85a2cbbef09f326f9ab2ad84d45da1faa0075f26063d9e2f8a56ca7e4` | `ace45ccd919323a93ae0f5ad62a8c782644726cc6d38147e632d70eb5343f2e2` | `a44d14010b6c2235df7d5a7acee1ed8339b093da0ac56b528b9f14e37f28729b` | `81f231e78228fa6c932f609c97da96abc2d7bac7936c877c95d5a3c7da11ff70` |
| `prisoes20` | `449bfa97459eb1971eb196030c7e121c2bd5367c4f902562736310f1ef29f9e8` | `2be0152924dc2ee4d61cc2eb33e2050dd9a95cfc212b3e65964048f67d7122c6` | idem | idem |
| `prisoes_ri` | `a48c8899cee9d1a962102cd31aa7974af1c6306c88d318e816e900fa1a289251` | `7fb1cfbfd57edd956a9de8f4f4debbdd40ebb20b0f6f8732113cb35b23436b17` | idem | idem |

Verificação vigente na árvore: `cargo run --release --bin labs -- --escuta` confere `fundo/p13-escuta/registro.tsv`
contra a cobertura mínima, e `crates/stochology-fairness` implementa o trilema de Chouldechova e a inversão de
métrica em Rust. O andaime Python original (`h3/estocologia/governanca.py`) saiu da árvore em 2026-10-16 e permanece
recuperável pelo commit `1bfec905` (`DECLARACAO_REMOVIDO_PYTHON.md`).
