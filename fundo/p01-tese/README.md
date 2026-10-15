# P01 · Piloto da tese central (README provisório)

## Resultado

Ainda não há resultado sobre a tese. O que está pronto é a montagem: o banco de 12 casos às cegas, o gabarito fora do
repositório (só o hash aqui) e a declaração do protocolo (`DECLARACAO.md`, hash em `DECLARACAO.sha256`), escrita
antes de qualquer parecer. Faltam os pareceres dos braços A e B e a codificação do juiz. Este README não revela
quais casos têm defeito e não deve ser entregue aos analistas.

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
