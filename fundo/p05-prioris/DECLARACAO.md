# Declaração do ponto 5 (prioris sem especialista), escrita antes de rerodar

Data: 2026-10-15. Escrita depois de montar as prioris alternativas (`prioris.py`, `prioris/*/*.yaml`) e antes de
qualquer execução do funil ou do ciclo com elas. Nenhuma execução das cópias foi feita até aqui.

## Limite que não se resolve daqui

Não há especialista humano disponível. Nenhuma priori desta rodada é uma elicitação SHELF de verdade: as prioris
de literatura são leituras de fontes publicadas, conferidas online, convertidas para a quantidade do domínio por
regras fixas escritas abaixo. Isto testa se as conclusões dependiam das prioris simuladas; não substitui um painel.

## Pergunta e critério

**Pergunta:** a decisão do F8 e o status de cada domínio mudam com a priori de literatura, com a priori fraca, ou com
nenhuma das duas?

**Escopo.** Todas as análises bayesianas com priori SHELF da chave:
- H2 (12 domínios, `h2/`) e H2g (18 domínios, `h2g/`). Os domínios de H2b a H2f têm as mesmas prioris e decisões
  que as versões finais em `h2g/` (conferido: quartis, `taxa_base` e `decisao` idênticos), por isso o H2g os cobre.
- H1 real (projetos a, b, c, `h1-real/`), com o painel SHELF simulado trocado.
- H3: não usa priori SHELF (o `estat.py` tem a função, mas nenhum fluxo a chama). Fica fora, e isso é dito.

**Três rodadas por horizonte**, a partir de cópias em `fundo/p05-prioris/` (os originais não são editados); a única
diferença entre elas é a tabela indicada em `$P05_PRIORI`:
- (a) original: sem tabela (as prioris dos pacotes). Serve também para conferir que a cópia reproduz o original.
- (b) literatura: `prioris/<h>/literatura.yaml`.
- (c) fraca: `prioris/<h>/fraca.yaml`.
Sementes, dados, código, decisões (custos, benefícios) e `taxa_base` ficam iguais.

**Regras das prioris (fixadas antes de rodar):**
- R1: fonte com estimativa e e erro padrão s: mediana e, dp = 2s (fator 2 de transporte entre estudo e domínio).
- R2: fonte com faixa [a, b]: mediana (a + b)/2, quartis a e b.
- R3: fonte com um ponto só: mediana e, dp = |e|/2.
- Binomiais: a conta é feita no log da razão de chances.
- Sem fonte conferida: a priori de literatura é a priori fraca, e isso fica escrito.
- Fraca: centro 0 (razão de chances 1), dp = 10 vezes o dp da priori original. No H1 real, Beta(1, 1).
- Fontes da mesma base de dados do domínio são usadas, mas marcadas "mesma base" (dupla contagem).

**O que se mede, por domínio e rodada:** status (categoria: podado no F1/F2/F3, reprovado no F4 com a lista dos
portões que falharam, aprovado), ação do F8 (adotar/manter, ou a ação equivalente no H1 real) e P(adotar compensa),
média e IC90 da posterior da chave, razão dp posterior / dp priori. A sensibilidade é o deslocamento da média
posterior em unidades do dp posterior da rodada (a): |m_b − m_a| / dp_a e |m_c − m_a| / dp_a.

**Mudança.** Um domínio "muda" numa rodada se a categoria de status muda (inclusive a lista de portões que falham)
ou se a ação do F8 muda (inclusive passar de decisão emitida a nenhuma, ou o contrário). Mudanças cujo único motivo
é um portão computacional (R-hat, ESS, SBC) são contadas como mudança e também listadas à parte.

**Veredito (regra fixa):** sejam N_b e N_c os números de domínios que mudam em (b) e em (c), sobre os 33 (12 + 18 + 3).
- N_b = 0 e N_c = 0: as conclusões não dependiam das prioris simuladas.
- N_b = 0 e N_c > 0: não dependiam das prioris simuladas no conteúdo, mas dependiam de haver alguma priori
  informativa nos domínios que mudam em (c).
- N_b > 0: dependiam das prioris simuladas nos domínios que mudam em (b); o veredito nomeia cada um e diz se a fonte
  de (b) era independente ou da mesma base.
Também se diz se o G3 do H2g (ICP com Ŝ ≠ ∅ em pelo menos 3 áreas) muda; o ICP não usa a priori, então só mudaria se
um domínio fosse podado antes do F4.

**O que conta como falha desta tentativa:** a cópia não reproduzir a rodada original (status ou posterior diferentes
dos `saida/` originais em (a)). Nesse caso as rodadas (b) e (c) não são interpretadas.
