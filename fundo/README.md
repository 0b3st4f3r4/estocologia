# Pontos de fundo da pesquisa

A auditoria do livro listou dez pontos de fundo da pesquisa — coisas que o texto afirmava ou supunha e que nunca
haviam sido testadas. Cada ponto ganhou uma frente nesta pasta, com a mesma regra de casa dos horizontes: uma
`DECLARACAO.md` com hash, gravada **antes** de rodar, um `README.md` com o resultado, e nada ajustado depois de ver
o resultado. Dado grande e rastro pesado ficam fora do Git, com `FONTES.md` e `SHA256SUMS` na pasta de origem (ver
[FORA_DO_GIT.md](../FORA_DO_GIT.md)); o ambiente de reprodução está em [`AMBIENTE.md`](AMBIENTE.md).

| ponto da auditoria | frente | estado | resultado em uma linha |
|---|---|---|---|
| 1 · a tese central nunca testada | [`p01-tese`](p01-tese/README.md) | fechada | No piloto cego de 12 casos, os dois braços apontaram 8 de 8 defeitos plantados e 0 alarme falso em 4 casos limpos: empate no teto, que pela regra declarada **não sustenta a tese**. |
| 2 · independência das versões | [`p02-independencia`](p02-independencia/README.md) | fechada | Quatro resultados reimplementados só a partir da especificação: todos concordam dentro da tolerância declarada, e o LaLonde e o Rhode Island dependem de escolhas que só estão no código. Zero erros do original encontrados. |
| 3 e 4 · multiplicidade e ajustes post-hoc | [`p03-multiplicidade`](p03-multiplicidade/README.md) | fechada | O K honesto do portão do H2 é 38 testes; com Bonferroni o H2g mantém 4 áreas e os Ŝ encolhem. No H3, só a quarta versão do estimador do laço passa no G4. |
| 5 · prioris simuladas | [`p05-prioris`](p05-prioris/README.md) | fechada | Com a priori de literatura 5 dos 33 domínios mudam e com a fraca 2 — as conclusões **dependiam da priori** nos domínios que mudam. O portão da rodada de controle tem a banda de equivalência fixada em `DECLARACAO_2.md` (0,05 dp, K = 2); o desvio máximo é 0,0071 dp, então 33 de 33 dominios reproduzem e o veredito é firme. |
| 6 · o laço performativo declarado | [`p06-laco`](p06-laco/README.md) | fechada | Medido numa central real (AnonymousBank, 1999): 0,145 [0,132; 0,157], contra 0,6 declarado no H3 — o laço do H3 é cerca de quatro vezes o observado. |
| 7 · ICP com linhas dependentes | [`p07-icp`](p07-icp/README.md) | fechada | Com o Wald agrupado, a Farmacologia sobrevive intacta e a Educação sai (Ŝ = ∅); a réplica da genética com 40 Y novos deu Ŝ com gene em 10 deles e 1 gene validado por intervenção estrita (n34, Y = YLR354C, x6 = YIL033C fora da faixa das 1 740 outras linhas), então as áreas são três — Física, Farmacologia e Genética — e o portão do H2g segue de pé. A taxa nula da validação, declarada em `DECLARACAO_2.md`, dá p = 0,001 (o piso com B = 999) para o único gene validado. |
| 8 · lacunas de calibração | [`p08-calibracao`](p08-calibracao/README.md) | fechada | Com tolerância de ECE que depende de n, nenhum cofre binário do H3 passa — nem os aprovados; pela regra secundária, o quadro do H3 fica igual. |
| 9 e 10 · escala e dados | [`p09-escala-dados`](p09-escala-dados/README.md) | fechada | Dois domínios novos de saúde podados no F3 (p = 0,060 e 0,28); 329 domínios em lote, 327 até o fim; 26 634 nós no hipergrafo, dos quais só 6 494 são de conceito. |

| 9 e 10 · dados de saúde | [`p10-saude`](p10-saude/README.md) | em andamento | Primeiro domínio real de saúde no espelho aberto da PhysioNet, no formato do H2g e com o portão de calibração do p08. |
| 9 e 10 · resolução em evento raro | [`p11-eventos-raros`](p11-eventos-raros/README.md) | fechada | Nos três casos de evento raro, nenhum caminho vence: em Rhode Island a resolução bate a confiabilidade com IC90 acima de zero, mas a habilidade (+1,9%) fica 40 vezes abaixo do +5% declarado; no painel COVID a ordenação é boa (AUC 0,77) e a previsão perde para a constante. |
| 11 · elicitação humana (SHELF) | [`p12-shelf`](p12-shelf/README.md) | protocolo congelado, painel faltando | Protocolo congelado com hash, pré-teste ajustado e a trilha do primeiro especialista (E1, o autor do programa) fechada em cinco domínios: pré-testes aprovados, quantis confirmados na rodada 1, controles lidos e a comparação preenchida até o funil, com a marca **P**. Falta o painel: as outras duas pessoas por domínio e o especialista de fora do programa. |
| 13 · a voz de quem paga o erro (frente nova) | [`p13-escuta`](p13-escuta/README.md) | protocolo congelado, escuta não executada | Nas sete decisões que mexem com pessoas, **0 de 7** grupos ouvidos, cobertura mínima de **2 de 7** e prazo em **2026-12-15**: a caixa está lacrada por instrumento (`labs --escuta`) e nenhuma decisão vira diretriz de política pública ou decisão de portão F8 até a cobertura. |
| 14 · a cadeia do H3 re-executada | [`p14-h3`](p14-h3/README.md) | fechada | Das quatro linhas do H3, uma é **derivável** — o laço performativo, cujo registro está versionado: a re-derivação confirma a razão de 4,1483 fora da banda declarada, com zero divergência entre a conta e o registro. Três ficam na classe **registro**, com o motivo: o dado bruto saiu com o andaime `h3/`. |
| 15 · as previsões 3 e 5 do eixo R | [`p15-previsoes-r`](p15-previsoes-r/README.md) | fechada | O limiar de oscilação **só se sustenta com o plano calibrado**: a fração de dias falhos cresce de 0,095 a 0,330, cruza o quarto na amplitude 2,5 e a perda vira +2,87 min — negativa abaixo dele. Com o plano de Erlang a previsão não se sustenta, e isso fica publicado. A transferência não tem violação em 6 perfis, com 3 contraexemplos. |

## O que falta

- **As dez frentes e as duas extensões de máquina estão fechadas**, com as ressalvas no README de cada uma. Os dois
  derivados de dado têm hash diferente do registrado por causa do escritor (conteúdo conferido; ver
  `FORA_DO_GIT.reconstruido.sha256`), e o ambiente de reprodução está em [`AMBIENTE.md`](AMBIENTE.md) e
  [`requirements.txt`](requirements.txt) — onde `statsmodels` (usado só pelo gabarito do p01) e `pyreadr` (um passo do
  p08) aparecem como ausentes, e por isso esses dois passos não rodam aqui.
- **Depende de gente, e só disso:** a escuta de quem paga o erro ([p13-escuta](p13-escuta/README.md): protocolo
  congelado, caixa lacrada, 0 de 7 grupos ouvidos, prazo em 2026-12-15) e o painel completo do SHELF
  ([p12-shelf](p12-shelf/README.md): a trilha do primeiro especialista está fechada em cinco domínios, faltam as
  outras duas pessoas por domínio e o especialista de fora do programa). A releitura dos 21 pareceres saiu em
  [`governanca/RELEITURA.md`](../governanca/RELEITURA.md): 17 confirmados, 4 retirados e reemitidos com nota assinada.
- **Divergência do G4 do H3, resolvida: era o numpy do caminho.** O G4 publicado (`h3/saida/portoes.json`: r̂ 0,5632 com 2375 choques) **reproduz** — no numpy **2.5.1** do `pyenv`, que só entra no `sys.path` com `PYTHONNOUSERSITE=1`. Com o `python3` padrão, que importa o numpy **2.4.6** de `~/.local`, o mesmo código dá r̂ 0,5529 com 2395 choques. O MRE: com o **mesmo** estado do PCG64, `binomial(248, 0.6)` devolve **171** no 2.4.6 e **150** no 2.5.1 — o numpy 2.5 corrigiu a série de Stirling do teste de aceitação do BTPE (o 3º e o 4º termos eram somados, e o coeficiente principal dizia `13680` em vez de `13860`). Não foi ponto aberto por muito tempo e **nenhum número publicado mudou**: diagnóstico e prova em `h3/DECLARACAO_G4_REGENERACAO_2.md`.
- **Levar o resumo ao livro, feito para as dez frentes originais.** Os pontos estão na seção 8 do artigo e na seção 9.7
  do livro didático; as extensões (saúde, evento raro, ficha, contrato) entram na mesma seção quando fecharem.

## Regras de casa

- A declaração vem antes, com hash (`DECLARACAO.sha256`) e hora.
- O README de cada frente diz o que foi feito, os números, os limites e como reproduzir.
- Mudar critério depois de ver um resultado não é proibido: entra como declaração nova e conta no K.
- Falha de execução e desvio de desenho são relatados como resultado, não escondidos.
