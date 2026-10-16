# Declaração: o que saiu da árvore por não ser significativo para a teoria

Em 2026-10-16, junto com a aposentadoria do código Python (`DECLARACAO_REMOVIDO_PYTHON.md`), o projeto
removeu os artefatos que serviam de **ilustração, escala ou extensão lateral** e não sustentavam nenhum
resultado central. A triagem que decide isso está em `book/resultados-significantes.md`: fica o que
sustenta um resultado da camada A (o que a teoria não pode perder) ou da camada B (princípio conhecido,
medido por este programa), mais o livro, os crates e a procedência dos números.

## O que saiu

| Artefato | Por que saiu |
| --- | --- |
| `book/edicao-14-capitulos.md` | edição superada pela de 21 capítulos; o texto vive no commit `1bfec905` |
| `fundo/p09-escala-dados/` | escala (329 domínios em lote, 26 634 nós): mostra que o funil roda em lote, não move a teoria |
| `fundo/p10-saude/` | extensão de saúde (sepse precoce): resultado próprio, fora do núcleo |
| `fundo/p11-eventos-raros/` | extensão de evento raro (indometacina, COVID): resultado próprio, fora do núcleo |
| `dados-reais/p10/` (219 MB, não versionado) | dado bruto da extensão de saúde, baixável pelas origens do `FONTES.md` |

**Total: 181 arquivos versionados e 258,89 MB.** Os números que o livro cita dessas extensões continuam no
texto (Capítulos 15 e 21), com a nota de aposentadoria ao lado da tabela.

## O que ficou, e por que fica

| Fica | Papel na teoria |
| --- | --- |
| `book/`, `estocologia-livro-didatico.md`, `crates/stochology-*` | a obra e o aparato |
| `h0-lab0/`, `h0-real/`, `h1/`, `h1-real/` | camada A: unidade independente, severidade, o portão que reprova o agente |
| `h2/`, `h2b/` … `h2g/` | camada A: os três sucessos do ICP e as cinco falhas honestas com o contador $K$ |
| `h3/` | camada A: Goodhart medido, calibração contra resolução, os três cofres |
| `fundo/p01-tese/` | o piloto cego dos 12 casos (resultado negativo que delimita a tese) |
| `fundo/p02-independencia/`, `p03-multiplicidade/`, `p05-prioris/`, `p06-laco/`, `p07-icp/`, `p08-calibracao/` | pontos da auditoria de fundo que mexem no veredito |
| `fundo/p12-shelf/` | o limite ético declarado: protocolo pronto, nenhuma entrevista feita |
| `governanca/` | quem assina e quem paga pelo erro |
| `dados-reais/` (menos `p10`) | procedência de cada número que o livro cita |
| `referencias/`, `artigo/` | a bibliografia conferida e o artigo que apresenta a teoria |
| `DECLARACAO_REMOVIDO_*.md`, `python-removido.txt`, `nao-significante-removido.txt` | o rastro das duas remoções |

## Como recuperar

```bash
git checkout 1bfec905 -- fundo/p09-escala-dados fundo/p10-saude fundo/p11-eventos-raros book/edicao-14-capitulos.md
git show 1bfec905:estocologia-livro-didatico.md | head -50   # a edição de 14 capítulos
```

O manifesto `nao-significante-removido.txt` traz o hash de blob de cada arquivo versionado removido.

## Consequência declarada

Os números das extensões removidas — a escala (327 de 329 domínios, 26 634 nós, 6 494 de conceito), o evento
raro (habilidade de +0,018629 e +0,017952, AUC 0,7685 no COVID) e a saúde ($\beta = 0{,}0388$, 0 de 512
subconjuntos, habilidade de +0,00568) — continuam no livro, agora como **registro citado**, e não mais como
resultado que se pode reexecutar. A triagem de significância não mudou por causa disso: nenhum deles entra na
camada A ou B.
