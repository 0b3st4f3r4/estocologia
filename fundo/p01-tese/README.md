# P01 · Piloto da tese central

## Resultado

**Empate no teto: o piloto não sustenta a tese, pela regra declarada antes.** Os dois braços apontaram o defeito
plantado nos 8 casos com defeito e não deram nenhum alarme falso nos 4 casos limpos (juiz cego, `juiz/notas.yaml`).
Sensibilidade 8/8 nos dois; alarme falso 0/4 nos dois; nenhum par discordante, logo o teste de McNemar não tem o
que testar. A `DECLARACAO.md` fixou que empate no teto conta como tese não sustentada.

A leitura honesta é que o piloto não discrimina: os dez erros da lista de Makin e Orban de Xivry (2019) são os
erros que qualquer estatístico competente procura, e um modelo instruído só a "analisar com rigor" achou todos.
O piloto não refuta a tese, mas mostra que, para esses erros, a arquitetura não acrescenta acerto. Se a tese
vale, vale para erros que a lista não cobre, e um teste justo disso precisa de casos mais difíceis escolhidos por
alguém de fora do programa, de analistas humanos e de um juiz humano.

Os vereditos dos dois braços coincidiram nos 12 casos. O braço A deixou de lado uma exigência da arquitetura (os
testes de propriedades com Hypothesis), e os três papéis são modelos da mesma família, o que pode explicar parte
da coincidência.

Depois da pontuação, o gabarito, os geradores e o mapa X/Y foram publicados nesta pasta (`gabarito/`, `juiz/`); o
SHA-256 do gabarito confere com `GABARITO.sha256` (4d5450eb…).

## O que foi feito

1. **Lista externa de defeitos.** Makin, T. R. & Orban de Xivry, J.-J. (2019), "Ten common statistical mistakes to
   watch out for when writing or reviewing a manuscript", *eLife* 8:e48175, doi:10.7554/eLife.48175. A página
   https://elifesciences.org/articles/48175 não carregou (exige JavaScript); a lista foi conferida na cópia do PMC,
   https://pmc.ncbi.nlm.nih.gov/articles/PMC6785265. Os dez erros estão transcritos na seção 2 de `DECLARACAO.md`.
2. **Sorteio.** Oito dos dez erros foram sorteados sem repetição com `np.random.default_rng(20261015)`. Dois erros
   ficaram de fora (quais estão em `DECLARACAO.md`, seção 8).
3. **Casos.** Doze casos: oito com um defeito cada e quatro limpos. A ordem foi embaralhada com semente fixa.

   | Caso | Dados |
   |---|---|
   | caso_01 | real |
   | caso_02 | simulado |
   | caso_03 | simulado |
   | caso_04 | simulado |
   | caso_05 | simulado |
   | caso_06 | real |
   | caso_07 | simulado |
   | caso_08 | simulado |
   | caso_09 | real |
   | caso_10 | simulado |
   | caso_11 | simulado |
   | caso_12 | simulado |

   Os três casos reais usam arquivos já baixados em `/mnt/project-files/estocologia/dados-reais/` (ver o `FONTES.md`
   de lá): NSW/CPS de LaLonde, Project STAR e o túnel de luz das Causal Chambers. As colunas foram renomeadas e a
   descrição é a de um cliente.

   Cada caso está em `/mnt/project-files/estocologia/fundo/p01-tese/casos/caso_XX/` com `alegacao.md`, um ou mais CSV
   e `dicionario.md` (3,2 MB no total; por isso fora do Git). Os hashes de todos os arquivos estão em `CASOS.sha256`
   (cópia de `casos/SHA256SUMS`).
4. **Gabarito e gerador.** `gabarito.yaml` (para cada caso: limpo ou com defeito, qual erro da lista, onde está, a
   conclusão correta, o critério objetivo do juiz e os números calculados) e `gerar_casos.py` (gera os 12 casos e o
   gabarito) ficam na pasta `p01-gabarito` do coordenador, fora do repositório, porque os dois revelam o defeito.
   Hashes em `GABARITO.sha256` e `GERADORES.sha256`.

## Números

Todos os números citados nas alegações e no gabarito saem de `gerar_casos.py`, rodado sobre os arquivos de
`dados-reais/` ou sobre processos simulados com semente fixa. Nenhum número foi digitado à mão. Os números da
seção 6 de `DECLARACAO.md` (p mínimo do McNemar exato) vêm de `scipy.stats.binomtest`.

## Como reproduzir

Com acesso à pasta do gabarito:

```
python3 -I p01-gabarito/gerar_casos.py /mnt/project-files/estocologia/dados-reais \
    /mnt/project-files/estocologia/fundo/p01-tese/casos p01-gabarito/gabarito.yaml
cd /mnt/project-files/estocologia/fundo/p01-tese/casos && sha256sum -c SHA256SUMS
sha256sum p01-gabarito/gabarito.yaml   # comparar com GABARITO.sha256
```

O gerador foi rodado duas vezes seguidas; os hashes de todos os arquivos de casos e do gabarito bateram.
Depende de numpy, pandas, scipy, statsmodels e PyYAML.

## Próximos passos (protocolo em DECLARACAO.md)

1. Braço A (arquitetura) e braço B (análise comum) analisam os 12 casos, cada um isolado, com o bloco final de veredito.
2. O coordenador cega e embaralha os pareceres; o juiz aplica o critério do gabarito.
3. Sensibilidade (/8), alarme falso (/4), McNemar exato e a regra de decisão declarada.

## Limites

Doze casos não dão poder para conclusão firme; os três papéis são modelos da mesma família; o plantador conhece o
catálogo da Estocologia; os casos reais vêm de conjuntos famosos que um modelo pode reconhecer. Detalhes na seção 8
de `DECLARACAO.md`.
