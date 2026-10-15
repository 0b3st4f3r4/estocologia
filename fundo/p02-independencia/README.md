# Ponto de fundo 2: independência (reimplementação N-versões a partir da especificação)

## Resultado

Os quatro resultados que sustentam as saídas aprovadas foram reimplementados do zero, sem ler o código nem as
saídas dos horizontes, e **todos concordam dentro das tolerâncias declaradas antes**. O ICP do STAR e o do hERG
saíram idênticos ao publicado (Ŝ, contagem de aceitos, saturados e maior p). O LaLonde e o Rhode Island concordam
na conclusão e nos números principais, mas não exatamente, e as diferenças vêm de escolhas que **não estão nos
README nem nas declarações**: as covariáveis do escore de propensão e o fato de o "+0,096" ser a mediana de uma
posterior bayesiana (LaLonde), e o filtro de linhas, a hora de emissão e os parâmetros do disjuntor (Rhode Island).
Achei zero erros do original e zero erros meus que mudem uma conclusão. O limite mais sério: a "equipe
independente" fui eu, um modelo da mesma família que escreveu o original, e várias escolhas que a especificação
não fixa coincidiram com as do original sem estarem escritas. Isso é falha correlacionada, não independência; ver
"Limites".

## O que foi feito

1. **Contrato.** Li só README, declarações, critérios, pacotes YAML e o `datacontract.yaml` do H1·B, mais os
   cabeçalhos dos CSV. A lista exata está em `DECLARACAO.md`.
2. **Declaração antes.** `DECLARACAO.md` traz cada número publicado com arquivo:linha, a tolerância e as escolhas
   que fiz onde a especificação cala. SHA-256 e hora em `DECLARACAO.sha256`
   (`5cd635e3…f1665a`, 2026-10-15T12:11:34Z).
3. **Implementação.** `comum.py` (logística de Newton, teste K = 2, regra 1-NN K = 3, ICP), `r1_lalonde.py`,
   `r2_star.py`, `r3_herg.py`, `r4_ri.py`. Semente fixa 20261015 (só o bootstrap do R1 usa o gerador).
4. **Congelamento.** Hashes dos `.py` e do `rodar.sh` em `CONGELAMENTO.sha256`, às 2026-10-15T12:18:39Z. Nada
   desses arquivos mudou depois.
5. **Só então** li o código original (`h1-real/projetos/b_real.py`, `h2d/dominios/educacao_star_turmas/preparo.py`,
   `h3/fluxos/prisoes_ri.py`, `h3/fluxos/base.py`, `h3/estocologia/calibracao.py`, `h3/avaliar_h3d.py`) e as saídas
   JSON, para explicar cada diferença. Dois diagnósticos pós-congelamento, separados da versão congelada, estão em
   `diagnostico/`.

## Tabela

| resultado | publicado | minha reimplementação | concorda? | causa da diferença |
|---|---|---|---|---|
| R1 · experimento, ΔR emprego | +0,111 [0,026; 0,196] (`h1-real/README.md:58`) | +0,1106 [0,0257; 0,1955] | sim | nenhuma (IC: variância com ddof = 1 no original, binomial na minha) |
| R1 · experimento, Δ renda | +1 794 | +1 794,3 | sim | nenhuma |
| R1 · crua contra CPS, ΔR | −0,107 [−0,170; −0,045] (`:56`) | −0,1074 [−0,1695; −0,0454] | sim | nenhuma |
| R1 · crua, Δ renda | −8 498 | −8 497,5 | sim | nenhuma |
| R1 · escore de propensão, ΔR | +0,096 [−0,006; 0,208] (`:57`) | +0,088 [−0,027; 0,215] (ponderação, bootstrap) | sim (diferença 0,008 ≤ 0,03) | **(iii) não especificado.** O +0,096 publicado é a mediana e o IC95 de uma posterior MCMC de 4 células (braço × negro) com pseudo-contagens de Kish e padronização pela raça (`b_real.py:485-500`, `:770-773`). O ponto da ponderação no original é 0,0905. O README chama isso só de "ajuste por escore de propensão". |
| R1 · escore, Δ renda | +1 570 [218; 3 265] | +1 680 [178; 3 203] | sim (Δ = 110 ≤ 500) | **(iii).** As covariáveis do escore (com re74² e re75², sem educ·re74) estão só na docstring de `b_real.py:22-24`. Com elas, meu código dá 1 570,2 (`diagnostico/diag_r1_spec_original.py`). |
| R1 · controles efetivos | ≈ 103 (`:61`) | 87,9 | sim (−15%, tolerância ±30%) | **(iii)**, a mesma causa: com a especificação do original, meu código dá 102,5. |
| R1 · CPS abaixo do menor escore | 72% (`:61`) | 73,2% | sim | (iii), a mesma causa: 71,6% com a especificação do original. |
| R2 · STAR Ŝ | {anos_pequena_antes, turma_pequena} (`h2d/README.md:76`) | {anos_pequena_antes, turma_pequena} | sim, idêntico | nenhuma |
| R2 · aceitos não saturados | 57 | 57 | sim, idêntico | nenhuma |
| R2 · aceitos saturados / saturados testados | 152 / 256 | 152 / 256 | sim, idêntico | nenhuma |
| R2 · maior p não saturado | 0,075 no conjunto de 6 (`:76`) | 0,0751 no mesmo conjunto | sim, idêntico | nenhuma |
| R2 · p de {turma_pequena} | 0,010 (`:99`) | 0,0101 | sim | nenhuma |
| R3 · hERG Ŝ | {c_dof, c_qui, c_ran} (`h2g/README.md:6`) | {c_dof, c_qui, c_ran} | sim, idêntico | nenhuma |
| R3 · aceitos / saturados | 35 / 0 (`:33`) | 35 / 0 | sim, idêntico | nenhuma |
| R3 · inclinação QTcF na dofetilida | 25,2 [23,0; 27,5] (`:38`) | 25,21 [22,99; 27,43] (conjugação normal) | sim | nenhuma de substância: o original usa MCMC (média 25,211) e EP agrupado 1,355; o meu, 1,359 (correção de amostra pequena) |
| R4 · n no cofre | 24 466 (`h3/README.md:227`) | 24 502 | sim (+0,15%) | **(iii).** O original tira também linhas sem sexo ou sem idade (`prisoes_ri.py:72`); a declaração só fala das linhas sem desfecho. E a previsão é emitida no fim do tick, então 5 paradas de 31/12/2012 depois das 18 h entram no cofre. 24 461 + 5 = 24 466 (`diagnostico/diag_r4_filtro_original.py`). |
| R4 · ȳ no cofre | 2,26% | 2,265% | sim | a mesma |
| R4 · \|p̄ − ȳ\| oficial | 0,0020 | 0,0007 | sim (passa nos dois limites; Δ ≤ 0,003) | **(iii).** O disjuntor (janela 400, margem 0,03, k = 3, intervalo 50; `prisoes_ri.py:33`) não está publicado e eu não o implementei: o reserva foi oficial em 12% do cofre no original e em 0% no meu. Também a idade é padronizada por constantes fixas (34; 12,7) e o passo de Platt usa p' da emissão. |
| R4 · ECE oficial | 0,0048 | 0,0051 | sim (passa nos dois limites) | as mesmas |
| R4 · habilidade oficial | +0,019 | +0,022 | sim | as mesmas |
| R4 · modelo cru | 0,013; 0,013; +0,004 (`:233`) | 0,013; 0,013; +0,005 | sim | nenhuma de substância |
| R4 · Platt no fim | a = −0,30, b = 1,07 (`:236`) | a = −0,297, b = 1,071 | sim | nenhuma |
| R4 · critério (a) e (b) | passa | passa | sim | nenhuma |

Classificação das causas: (i) erro meu: nenhum que mude um número fora da tolerância; (ii) erro do original:
nenhum achado; (iii) escolha não especificada: cinco, todas no LaLonde e no Rhode Island.

## O que as divergências dizem

- **A especificação não basta para reproduzir o LaLonde.** O README dá o número, mas não diz que ele vem de uma
  posterior, nem qual escore. Uma equipe independente chega a +0,088 em vez de +0,096, e a 88 em vez de 103
  controles efetivos. A conclusão ("o ajuste acerta o ponto, mas se apoia em poucos controles") sobrevive.
- **A especificação não basta para reproduzir o Rhode Island exatamente.** O disjuntor do fluxo é parte da
  previsão oficial, mas os parâmetros só estão no código. Sem ele, a calibração fica até melhor (|p̄ − ȳ| 0,0007
  contra 0,0020), então a aprovação do H3d não depende do disjuntor.
- **STAR e hERG reproduzem exatamente pelas declarações.** As declarações K = 2, K = 3 e H2g, com o pacote YAML,
  bastaram. Mas veja o primeiro limite abaixo.

## Os pareceres assinados

O que sil precisaria fazer: reler por inteiro os 21 pareceres listados em `governanca/ASSINATURAS.md` (os arquivos
com os SHA-256 de lá, não o resumo) e então confirmar ou retirar a assinatura, registrando que leu o texto integral.
Nada foi feito aqui sobre isso.

## Como reproduzir

```
cd fundo/p02-independencia
./rodar.sh                                          # ~4 min; grava saida/*.json e imprime os hashes
python3 -I diagnostico/diag_r1_spec_original.py     # pós-congelamento: escore com as covariáveis do original
python3 -I diagnostico/diag_r4_filtro_original.py   # pós-congelamento: filtro de linhas do original
sha256sum -c <(head -6 CONGELAMENTO.sha256)         # confere que o código é o congelado
```

Rodei a reprodução duas vezes depois da primeira execução; os quatro JSON tiveram hashes idênticos nas três:
`r1_lalonde.json 8461cca1…`, `r2_star.json d758e7b0…`, `r3_herg.json f0fd0d1e…`, `r4_ri.json 14102f54…`.

## Limites

- **Não é independência de autor.** A reimplementação foi feita por um modelo da mesma família do que escreveu o
  original. Várias escolhas que nenhum documento fixa coincidiram com as do original: no STAR, "mestrado =
  grau diferente de bachelor", "etnia = cauc, afam ou outra" e a padronização pelas regulares da série
  (`preparo.py:23-38`); a coincidência exata de 57 aceitos depende delas. Isso é exatamente o modo de falha que o
  ponto de fundo 2 aponta: versões que concordam porque compartilham o autor. Uma concordância daqui é evidência
  fraca de correção; uma divergência seria forte. Independência real pede uma pessoa ou equipe de fora, com o
  mesmo contrato.
- **O contrato não foi só o README.** Para STAR e hERG usei também as declarações e os pacotes YAML, como o pedido
  permitia. O README sozinho não basta para nenhum dos quatro.
- **R4 sem disjuntor.** Não reimplementei o disjuntor nem os sensores de deriva, porque os parâmetros não estão
  publicados. A comparação de R4 é, portanto, do previsor sem disjuntor contra o oficial com disjuntor.
- **R1 sem o MCMC.** Não reimplementei a posterior de 4 células do original, que eu não tinha como saber que
  existia; comparei o estimador de ponderação com bootstrap.
- **Escopo.** Só os quatro resultados pedidos. Placebo, severidade, justiça e cofre do LaLonde, o F2/F3/F8 dos
  domínios do H2 e o F6/G3/G5 do H3d não foram reimplementados.
