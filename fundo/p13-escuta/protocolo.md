# Protocolo de escuta — o ponto 13 da pesquisa

**Frente:** `p13-escuta`. **Dono:** `sil`. **Congelado antes da primeira tentativa de contato.**

Este protocolo trata a única coisa que o programa ainda não fez: **ouvir quem paga o erro**. O livro pede isso na
seção 14.2, objeção "O auditor é parte interessada"; o registro de quem foi ouvido está em
[`governanca/GRUPOS.md`](../../governanca/GRUPOS.md), e ele diz **nenhum** nas sete decisões que envolvem pessoas.

## 1. Objeto

As sete decisões do programa que afetam pessoas reais, com os grupos que pagam o erro de cada uma:

| decisão | o que ela faz | quem paga o erro |
| --- | --- | --- |
| `h2g/saida/direito_teto_beneficio` | corta o benefício semanal em 10% para encurtar afastamentos | trabalhadores acidentados no trabalho cobertos pelo teto, Kentucky (1980) e Michigan (1982); o grupo acima do teto |
| `h2g/saida/educacao_star_turmas` | reduz as turmas da séries iniciais para cerca de 15 alunos | crianças do jardim à 3ª série das escolas públicas do Tennessee no STAR (11 598 alunos, 80 escolas), por etnia declarada |
| `h2g/saida/saude_rand` | troca o cuidado gratuito por coparticipação de 25% | famílias não idosas do RAND HIE (anos 1970), nos planos com teto de desembolso |
| `h1-real/saida/b` (NSW/LaLonde) | compara o treino profissional do NSW com controles do CPS | 185 participantes do treino do NSW e 15 992 controles do CPS, por raça declarada |
| `h3` fluxo `prisoes` | marca o detido para revisão da liberação (Toronto 1997–2002) | 5 226 detidos, por cor percebida |
| `h3` fluxo `prisoes20` | marca a pessoa presa para revisão do registro na delegacia (Toronto 2020–2021) | 65 276 prisões, 37 mil pessoas distintas, por cor percebida |
| `h3` fluxo `prisoes_ri` | marca o motorista para revisão da prisão na parada (Rhode Island 2005–2015) | 86 113 paradas, por cor percebida |

O registro em máquina das sete linhas é `registro.tsv`, lido pelo instrumento `labs --escuta`.

## 2. O fato declarado: nenhuma das coortes históricas é alcançável

Declarado pelo dono do programa em 2026-10-16: **nenhum dos sete grupos históricos está ao alcance.** As coortes
são de décadas atrás — crianças do STAR nos anos 1980, famílias do RAND HIE, detidos de Toronto em 1997, motoristas
de Rhode Island em 2005 — e o contato direto é inviável por barreira temporal, legal, ética e de anonimato. O
programa não tem, e não vai inventar, registro de consulta que não aconteceu.

Isso não fecha a frente: muda o que se procura. O que se procura é **equivalente estrutural** — quem hoje ocupa a
mesma posição de quem pagou o erro então.

## 3. Quem conduz

A escuta **não é conduzida pelo dono do programa**. Os ajustes e as prioris saíram da mesma mão que faria as
perguntas, e isso é viés de confirmação estrutural. Vale uma das duas formas, declaradas antes:

1. **facilitador neutro ou institucional**, sem vínculo com o programa; ou
2. **protocolo cego**: o entrevistador recebe só o roteiro da seção 4, sem os números do funil, sem a priori, sem
   a hipótese do programa, e não sabe qual decisão está por trás da conversa.

A escolha entre as duas formas, e o nome do facilitador, entram em `DECLARACAO.md` **antes** da primeira conversa.

## 4. O roteiro, nas palavras do grupo

Três perguntas, sem ficha técnica e sem vocabulário do programa — nada de quantil, Beta, Brier, EVPI, F1 a F8:

1. *Qual é o seu pior cenário que ainda é plausível?*
2. *O que faria você recusar essa intervenção?*
3. *Quando essa política falha, quem de fato arca com o custo?*

O entrevistador registra as respostas em frequências naturais ("de cada 100 pessoas, quantas"), anota a data e o
local, e pede a assinatura ou o consentimento registrado de quem falou. O que o grupo disse **não** é traduzido
para o vocabulário do programa por quem conduz: a tradução, se houver, é feita depois, à parte, e fica marcada como
leitura do programa.

## 5. Regra de não substituição

Quem não é do grupo não fala pelo grupo. Vale para o dono do programa, para o facilitador, para o pesquisador
herdeiro da base e para qualquer parecerista. Um equivalente estrutural (associação de classe, defensoria,
representante setorial, pesquisador herdeiro das bases originais) entra **com o rótulo de equivalente**, nunca como
o grupo. Declarar o rótulo é parte do registro, e a cobertura da seção 6 conta só o que foi ouvido de verdade.

## 6. Cláusula de saída

- **Janela:** 60 dias corridos a partir do carimbo desta declaração, procurando equivalentes estruturais dos sete
  grupos.
- **Cobertura mínima:** **2 dos 7** grupos, cada um com registro assinado ou consentimento registrado.
- **Se a cobertura não for atingida até o prazo:** o artefato é **arquivado como exercício metodológico de
  sensibilidade (n = 1)**, impedido de virar diretriz de política pública ou decisão de portão F8. A caixa é
  lacrada e o lacre tem hash.
- **Se a cobertura for atingida:** os registros entram em `governanca/GRUPOS.md`, a coluna "ouvidos" deixa de dizer
  nenhum nas linhas cobertas, e o portão do F8 é reaberto só para elas.

## 7. A voz do operador não conta

A voz de quem modela já está inteira nas prioris e nos ajustes, e o operador projeta a regra enquanto o grupo sofre
a consequência. Registrar o operador na coluna "ouvidos" seria conflito de interesse estrutural e anularia o
objetivo da frente. **O operador não conta como grupo ouvido, em nenhuma das sete decisões.**

## 8. O que fica na caixa e o que fica fora

**Na caixa, sem uso em campo:** as sete decisões da seção 1. Nenhuma delas vira diretriz de política pública ou
decisão emitida de portão enquanto a cobertura da seção 6 não for atingida.

**Fora da caixa, como está:** as cinco rodadas elicitadas do E1 (`fundo/p12-shelf/formularios/`) seguem como
**análise de sensibilidade com n = 1**, que é o que o protocolo do SHELF já declarava. O funil oficial continua
com a priori do pacote.

## 9. Mudança de critério

Roteiro, prazo, cobertura mínima, quem conduz: nada disso é reescrito depois de ver resultado. Mudança de critério
é **declaração nova**, com hash próprio, e conta no K do programa.

## Como reproduzir

```bash
cargo run --release --bin labs -- --escuta
```

O instrumento lê `registro.tsv`, conta a cobertura, confere se alguma decisão se diz emitida sem ninguém ouvido e
dá o veredito do lacre. A prova negativa está nos testes do módulo: uma linha que se diz emitida com a coluna
"ouvidos" em nenhum é reprovada.
