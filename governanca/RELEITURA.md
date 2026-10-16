# Releitura dos pareceres de risco (formulário)

**Em branco de propósito: nada foi relido, nada foi confirmado e nada foi assinado aqui.** Este arquivo é o
instrumento da releitura dos 21 pareceres que sil assinou em 2026-10-15T03:53:28Z (registro em
[`ASSINATURAS.md`](ASSINATURAS.md)). Quem preparou este formulário não leu parecer nenhum como quem concorda: só
copiou número, arquivo e hash.

Os 21 SHA-256 abaixo são os de `ASSINATURAS.md` e **foram conferidos um a um com `sha256sum`** contra o arquivo
citado, em 2026-10-15: **21 de 21 batem**, nenhum arquivo faltou. A conferência é reproduzível pelo trecho de
"Como conferir", no fim.

## Como preencher

- **`data`** — a data em que a releitura foi feita (ISO ou como em `ASSINATURAS.md`).
- **`decisão`** — exatamente uma das três:
  - **confirmo** — o parecer fica como está, com o hash desta tabela;
  - **retiro** — a assinatura é retirada; o parecer deixa de ser assinado e isso fica escrito;
  - **retiro e reemito** — a assinatura é retirada e o parecer volta **com outro** (ou com o mesmo conteúdo
    reemitido), e o hash novo entra na seção "Releitura" de `ASSINATURAS.md`, ao lado do antigo, que **não** é
    apagado.
- **`nota`** — o que a releitura viu. Não é lugar de reescrever o parecer.
- **Nada é reescrito.** Linha já preenchida não muda: releitura nova entra como linha nova datada, e o
  `ASSINATURAS.md` **não** é editado por quem prepara este formulário (é fora do escopo de escrita desta tarefa).

### Regra dos quatro pareceres que envolvem pessoas

Nos **quatro** pareceres com "sim" nesta tabela (2, 3, 17, 20), a nota é obrigatória e diz **duas coisas**:

1. **o que o parecer NÃO olhou** — o limite declarado: o que ficou fora do parecer, o que ele não avaliou, o que
   não tem fonte; e
2. **quais grupos afetados existem** — quem é atingido pela decisão do parecer (grupos, populações, categorias),
   dito por nome.

Cada uma das duas pode ser **"nenhum"** quando não houver: `não olhou: nenhum` / `grupos afetados: nenhum`. Nos 17
pareceres com "não", a nota pode ficar vazia ou dizer "nenhum"; escrever nota sobre eles é permitido, só não é
exigido.

O material levantado do próprio repositório para essas notas (titulares, quase-identificadores, k-anonimato,
grupos de justiça e escopo de cada decisão) está na seção "Material para a nota", no fim deste arquivo. Ele é
**material**, não é a nota: quem relê escreve a dele.

O arquivo irmão [`GRUPOS.md`](GRUPOS.md) — preparado na mesma data, com os grupos afetados das sete decisões que
envolvem pessoas e a seção "O que a auditoria não pôde olhar" — é a outra metade desse material: a nota dos quatro
pareceres deve **citar** `GRUPOS.md` em vez de repetir o que ele já levanta.

## Tabela da releitura

| # | parecer | envolve pessoas | arquivo | SHA-256 | data | decisão | nota |
|---|---|---|---|---|---|---|---|
| 1 | Biologia (sinalização celular): biologia_sachs | não | `h2g/saida/biologia_sachs/resultados.json` | `c4586baf485c2edf1b77f2c2718df317e246e4729f05601f2a3be0fa11f5168b` | 2026-10-16 | confirmo | nenhum |
| 2 | Direito (regra de indenização trabalhista): direito_teto_beneficio | sim | `h2g/saida/direito_teto_beneficio/resultados.json` | `ba8a63e9924224a16c539ce7b00db3814a91d6fd2389721bd1f9679cf3d11d30` | 2026-10-16 | retiro e reemito | parecer novo em `pareceres/2-direito_teto_beneficio.md`; nota confirmada e assinada por sil em 2026-10-16 |
| 3 | Educação: educacao_star_turmas | sim | `h2g/saida/educacao_star_turmas/resultados.json` | `f5b4307e4cbd103b96e7e0750c24e1758ec29995ae4eb7fa2bca28bc39477994` | 2026-10-16 | retiro e reemito | parecer novo em `pareceres/3-educacao_star_turmas.md`; nota confirmada e assinada por sil em 2026-10-16 |
| 4 | Engenharia (aerodinâmica): engenharia_tunel_vento | não | `h2g/saida/engenharia_tunel_vento/resultados.json` | `fbe8d0731fc3e7ab1a35c7a4fef134c5687e4026f7e8955130c46b8ee2e444fe` | 2026-10-16 | confirmo | nenhum |
| 5 | Farmacologia clínica (cardiologia): farmacologia_qt | não | `h2g/saida/farmacologia_qt/resultados.json` | `d0042974a80fb3166d6e0f601c4b5facec8d6e66eb0547eb6e60fc9fce3c979e` | 2026-10-16 | confirmo | nenhum |
| 6 | Física (óptica e eletrônica): fisica_tunel_luz | não | `h2g/saida/fisica_tunel_luz/resultados.json` | `9bfa0f14b512611fc142a726505054925022767cac385bfb8dcfcd6f63c86ec0` | 2026-10-16 | confirmo | nenhum |
| 7 | Genética (expressão gênica em levedura): genetica_y01 | não | `h2g/saida/genetica_y01/resultados.json` | `e14275f56b8e42f52d872c1782c16591d9fede8cb72f95488095e911758326b3` | 2026-10-16 | confirmo | nenhum |
| 8 | Genética (expressão gênica em levedura): genetica_y02 | não | `h2g/saida/genetica_y02/resultados.json` | `5877dc7fc993b0a0f87f0c809e132dd61a8ef0e87eca352c22087bc9faa75cc8` | 2026-10-16 | confirmo | nenhum |
| 9 | Genética (expressão gênica em levedura): genetica_y03 | não | `h2g/saida/genetica_y03/resultados.json` | `28f77502bf7ce135925874262b476b92eb60fb4a6a79912a479c0dca2611aac7` | 2026-10-16 | confirmo | nenhum |
| 10 | Genética (expressão gênica em levedura): genetica_y04 | não | `h2g/saida/genetica_y04/resultados.json` | `e7543937f2c7c95c7781c5318aab1add0d5d1afef6195fa76489a9ed28d4f587` | 2026-10-16 | confirmo | nenhum |
| 11 | Genética (expressão gênica em levedura): genetica_y05 | não | `h2g/saida/genetica_y05/resultados.json` | `620b251f38ebe1e4dac86fca07e02fc613230d5b617c0e6befb215bae098a24a` | 2026-10-16 | confirmo | nenhum |
| 12 | Genética (expressão gênica em levedura): genetica_y06 | não | `h2g/saida/genetica_y06/resultados.json` | `c346f8a4d0b3a4be07e8d9e51e4ebae5723b223d23ee541e5b6bfdf4ffbc051e` | 2026-10-16 | confirmo | nenhum |
| 13 | Genética (expressão gênica em levedura): genetica_y07 | não | `h2g/saida/genetica_y07/resultados.json` | `65666d206376fa73c826ccb7aef1b42dee9152556fe398f015d240f62297ed68` | 2026-10-16 | confirmo | nenhum |
| 14 | Genética (expressão gênica em levedura): genetica_y08 | não | `h2g/saida/genetica_y08/resultados.json` | `86f19427ef3057afa25ed3da0f02c5cd700ab1529c7eced343ed062b843dacc5` | 2026-10-16 | confirmo | nenhum |
| 15 | Genética (expressão gênica em levedura): genetica_y09 | não | `h2g/saida/genetica_y09/resultados.json` | `b4733257657015481c0f608e7c391e39c8b545d658a80d61e919d1112391aab2` | 2026-10-16 | confirmo | nenhum |
| 16 | Genética (expressão gênica em levedura): genetica_y10 | não | `h2g/saida/genetica_y10/resultados.json` | `7fcfc653b550d0ae71e8491086683b4d2aaf286150be6961a891c7b26dc405e2` | 2026-10-16 | confirmo | nenhum |
| 17 | Saúde (seguro e uso de serviços): saude_rand | sim | `h2g/saida/saude_rand/resultados.json` | `4e9c87b011fc1e01b8faea4a66ed1bbd5454504082435a5b62a9eb5bc44c7d6c` | 2026-10-16 | retiro e reemito | parecer novo em `pareceres/17-saude_rand.md`; nota confirmada e assinada por sil em 2026-10-16 |
| 18 | Transporte (segurança viária): transporte_cinto | não | `h2g/saida/transporte_cinto/resultados.json` | `390b12022336021768a6febe9976f6faca46fc1540aab02b04d5289e682d5347` | 2026-10-16 | confirmo | nenhum |
| 19 | H1·A Dow Jones, aditivo contra multiplicativo (1885–2023) | não | `h1-real/saida/a/resultados.json` | `2b95a0b4832289c221f05c03c3d1136649c0d0e13c571e01bfe5f51255e4d73a` | 2026-10-16 | confirmo | nenhum |
| 20 | H1·B NSW (LaLonde): treino profissional contra controles do CPS | sim | `h1-real/saida/b/resultados.json` | `5aaa954e56ede16cd7d38360bd923b3c75dea201f46464e7bb398c36f436aa58` | 2026-10-16 | retiro e reemito | parecer novo em `pareceres/20-nsw-lalonde.md`; nota confirmada e assinada por sil em 2026-10-16 |
| 21 | H1·C cauda dos incêndios dinamarqueses, camada 100 xs 50 | não | `h1-real/saida/c/resultados.json` | `0ecd25593905e8bf021262a1735538d27d574f58a75b64474d25c7b2c4116f38` | 2026-10-16 | confirmo | nenhum |

## Material para a nota (levantado do repositório; não é a nota)

Campos copiados dos próprios `resultados.json` dos quatro pareceres que envolvem pessoas. Servem de ponto de
partida para as duas partes da nota ("não olhou" e "grupos afetados") e **não** substituem a releitura. O
[`GRUPOS.md`](GRUPOS.md) traz a mesma lista com os números de titulares por grupo, e a parte de "o que a auditoria
não pôde olhar" já levantada; este bloco fica como a leitura campo a campo de cada `resultados.json`.

- **2 · Direito (regra de indenização trabalhista), `direito_teto_beneficio`.** `governanca`: `tem_titulares: true`,
  quase-identificadores `["homem", "casado", "kentucky"]`, `k_anonimato_bruto: 63`, dados reais
  (`wooldridge_injury.csv`), responsável humano com assinatura pendente. Decisão do parecer: "cortar o benefício
  semanal em 10% para encurtar os afastamentos", com "adotar" e P = 1,00. Escopo declarado: vale para indenização
  por acidente de trabalho em estados dos EUA no início dos anos 1980 com mudanças moderadas do teto; **não vale**
  para doenças ocupacionais de longa latência, cortes grandes, outros países e duração total sem benefício.
  Ressalva que já acompanha o parecer em `ASSINATURAS.md`: a recomendação do corte de 10% vem de uma decisão e de
  um custo **inventados** no F1, e não é conselho de política.
- **3 · Educação, `educacao_star_turmas`.** `governanca`: `tem_titulares: true`, quase-identificadores
  `["menina", "etnia", "almoco_gratuito"]`, `k_anonimato_bruto: 9`, dados reais (`AER_STAR.csv`). Bloco de
  justiça com grupo `etnia` e taxa-base por grupo: `afam` n = 4 524, `cauc` n = 8 632, `outra` n = 80. Decisão:
  "reduzir as turmas das séries iniciais para ~15 alunos", com "manter" e P(adotar compensa) = 0,47275. Escopo: vale
  para séries iniciais de escolas públicas dos EUA com turmas de 13 a 17 contra 22 a 25; **não vale** para séries
  posteriores, redução em escala estadual com professores menos experientes e outros países.
- **17 · Saúde (seguro e uso de serviços), `saude_rand`.** `governanca`: `tem_titulares: true`,
  quase-identificadores `["mulher", "local"]`, `k_anonimato_bruto: 1103`, dados reais
  (`sampleSelection_RandHIE.csv`). Decisão: "trocar o cuidado gratuito por coparticipação de 25%", com "adotar" e
  P = 0,74125. Escopo: vale para famílias não idosas dos EUA nos anos 1970, planos com teto de desembolso e gasto
  médico coberto; **não vale** para idosos (Medicare), planos sem teto de desembolso, outros países e desfechos de
  saúde (o parecer só olha gasto).
- **20 · H1·B NSW (LaLonde), `h1-real/b`.** `f8`: recomendação "financiar o treinamento", efeito causal ATT com
  mediana 0,0963 (IC90 0,0088 a 0,1879), "financiar se o ATT causal > C/L" com C por participante **ilustrativo**.
  Classe de referência declarada: homens elegíveis ao NSW, 1976–77, 10 cidades dos EUA, 84% negros. O disjuntor
  declarado manda parar se a fração de negros de um lote se afastar mais de 0,25 de 0,84. Não há bloco
  `governanca` com titulares nem quase-identificadores neste `resultados.json` (o do H1 real não tem esse campo):
  é um dos pontos que a nota tem de dizer.

## Como conferir os 21 SHA-256

Roda da raiz do repositório, sem escrever nada:

```bash
cd "$(git rev-parse --show-toplevel)"
python3 - <<'EOF'
import re, subprocess, pathlib
root = pathlib.Path(".")
txt = (root/"governanca/RELEITURA.md").read_text()
rows = re.findall(r'^\|\s*(\d+)\s*\|.*?\|\s*`(.+?)`\s*\|\s*`([0-9a-f]{64})`\s*\|.*$', txt, re.M)
assert len(rows) == 21, len(rows)
ruins = []
for n, arq, h in rows:
    got = subprocess.run(["sha256sum", arq], capture_output=True, text=True).stdout.split()[0]
    if got != h:
        ruins.append((n, arq, h, got))
print("linhas:", len(rows), "| divergentes:", len(ruins))
for r in ruins:
    print("DIFERE", r)
EOF
```

Saída esperada: `linhas: 21 | divergentes: 0`.

## Bloco para `governanca/ASSINATURAS.md` — **aplicado em 2026-10-16**

A releitura foi decidida, e a seção foi para o fim de `ASSINATURAS.md` com os campos preenchidos: os dezessete
confirmados, os quatro retirados e reemitidos com o hash novo de cada um, e as quatro notas em duas partes. O
esqueleto abaixo fica como registro do que era para ser feito.

````markdown
## Releitura

**Relida em <DATA> por sil.** A releitura usou o formulário `governanca/RELEITURA.md` (tabela de 21 linhas, com os
mesmos SHA-256 conferidos por `sha256sum`). O resultado de cada parecer está abaixo; `confirmo` mantém a assinatura
de 2026-10-15T03:53:28Z, `retiro` a retira, `retiro e reemito` a retira e registra o hash novo.

| # | parecer | SHA-256 na releitura | decisão | nota |
|---|---|---|---|---|
| 1 | ... | `<sha256>` | <confirmo / retiro / retiro e reemito> | <nota> |
| ... | (as 21 linhas de `RELEITURA.md`) | | | |

**Os quatro pareceres que envolvem pessoas (2, 3, 17 e 20).** Em cada um, a nota diz o que o parecer **não olhou** e
quais **grupos afetados** existem (ou "nenhum", quando não houver):

- **2 · Direito (regra de indenização trabalhista)** — não olhou: <...>; grupos afetados: <... ou nenhum>.
- **3 · Educação (STAR)** — não olhou: <...>; grupos afetados: <... ou nenhum>.
- **17 · Saúde (seguro e uso de serviços)** — não olhou: <...>; grupos afetados: <... ou nenhum>.
- **20 · H1·B NSW (LaLonde)** — não olhou: <...>; grupos afetados: <... ou nenhum>.

**Nada foi reescrito.** O registro de 2026-10-15T03:53:28Z continua no arquivo; onde a decisão foi "retiro e
reemito", o parecer novo entra como linha nova (ou como `resultados.json` novo em pasta nova) e o hash antigo fica
citado ao lado, para a reprodução limpa continuar comparável.
````

## Resultado da releitura de 2026-10-16

**Quem releu:** sil, o signatário de 2026-10-15T03:53:28Z, com o facilitador preparando o material. **Data:**
2026-10-16, na árvore de trabalho.

**Os 21 arquivos não estão na árvore.** A poda de 2026-10-16 levou as frentes `h2g/` e `h1-real/` para o commit
`1bfec905`. A conferência dos 21 SHA-256 foi refeita hoje nesse commit:

~~~text
git archive 1bfec905 h2g/saida h1-real/saida | tar -x -C /tmp/releitura
linhas: 21 | conferidos: 21 | divergentes: 0 | ausentes: 0
~~~

Na árvore, sem restaurar, o mesmo bloco de conferência dá `linhas: 21 | ausentes: 21 | divergentes: 0`. O
registro fica com os dois números: os arquivos saíram da árvore, o conteúdo deles está a salvo no commit e os
21 hashes batem lá.

**Decisões.** Dezessete pareceres **confirmo** (1, 4 a 16, 18, 19 e 21): a assinatura fica de pé e a nota é
"nenhum", como o instrumento permite para os que não envolvem pessoas. Quatro pareceres **retiro e reemito**
(2, 3, 17 e 20, os que envolvem pessoas): a assinatura antiga sai e o parecer volta com outro, em
`governanca/pareceres/`.

| # | parecer novo | o que a reemissão acrescenta |
| --- | --- | --- |
| 2 | `pareceres/2-direito_teto_beneficio.md` | bloco de governança (titulares, quase-identificadores, k = 63), escopo e a nota obrigatória |
| 3 | `pareceres/3-educacao_star_turmas.md` | bloco de justiça por etnia com as taxas-base, escopo, a ausência do painel SHELF e a nota |
| 17 | `pareceres/17-saude_rand.md` | o limite "só olha gasto" em destaque, os idosos como excluídos do escopo e a nota |
| 20 | `pareceres/20-nsw-lalonde.md` | o bloco de governança que faltava, o custo ilustrativo declarado e a nota |

**A nota dos quatro está proposta, não assinada.** O instrumento diz que quem relê escreve a nota dele; o
signatário escolheu retirar e reemitir, e o facilitador escreveu a proposta de nota a partir do material
levantado (`GRUPOS.md` e os `resultados.json` do commit). Cada um dos quatro documentos traz a nota marcada como
**pendente de confirmação do signatário**. Enquanto ela não for confirmada, o parecer novo **não** entra em
`ASSINATURAS.md`: o hash dele só se registra com a nota fechada, e é por isso que `ASSINATURAS.md` segue
intocado.

**Fecho da confirmação, no mesmo dia.** O signatário revisou os quatro pareceres novos e: confirmou o **3** e o **17**
na íntegra; confirmou o **2** com um microajuste de redação no primeiro ponto da nota ("o efeito da mudança sobre
quem está abaixo do teto", e não "o efeito do corte"); e confirmou o **20** com um microajuste na cláusula final
("o disjuntor declarado desarma caso a fração de negros de um lote se afaste mais de 0,25 de 0,84", no lugar de
"para se afastar"). As notas ficaram como o signatário as assinou, o rótulo de pendência saiu dos quatro arquivos, e
os quatro SHA-256 entraram na seção "Releitura" de `ASSINATURAS.md`, ao lado dos antigos.

**Contagem.** K da releitura: as 21 releituras são um ato só (K = 1 da releitura). As quatro reemissões não somam
K, porque não são especificação nova: é o mesmo parecer reemitido com o que faltava declarado.

