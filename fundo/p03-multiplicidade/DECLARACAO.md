# Declaração: pontos 3 e 4 (multiplicidade não contada e ajustes depois de ver resultados)

Escrita em 2026-10-15, antes de calcular qualquer p-valor corrigido e antes de rodar qualquer cópia de horizonte.
O SHA-256 deste arquivo e a hora UTC ficam em `DECLARACAO.sha256`.

## O que eu já sabia ao escrever

Li os READMEs, as declarações e os critérios de h2 … h2g e h3, e listei, a partir de `h2*/saida/*/resultados.json`,
que domínios chegaram ao F4 em cada rodada, com que α, que teste e que Ŝ. Conheço, portanto, os Ŝ publicados e os
"maiores p" citados nos READMEs (por exemplo, 0,075 no STAR do H2d; 0,0072 no y08; 0,29 em {red, green, blue} no
túnel de luz). Não calculei, antes desta declaração, o p-valor de nível de domínio definido abaixo, nem nenhum
resultado corrigido, nem nenhuma execução v1.

## Ponto 3: multiplicidade

### Unidade de contagem

Um **teste ICP** é uma execução do ICP sobre um domínio com uma especificação (dados, unidade, ambientes, candidatos,
teste de invariância, regra de saturação, α) que produziu p-valores por subconjunto. Conto três versões de M:

- **M = 38 (principal): testes distintos.** 11 do H2; 6 reexecuções dos domínios do H2 com a correção do posto
  (H2b); 3 do H2b mais 2 ICPs diagnósticos (Sachs e cinto, depois da poda); 3 do H2c (K = 2); 5 do H2d (K = 3); 2
  novos no H2e; 5 novos no H2f (genética que chegou ao F4); 1 novo no H2g. Uma reexecução determinística, idêntica,
  da mesma especificação não é uma tentativa nova e não entra.
- **M = 32 (sensibilidade, para baixo):** sem as 6 reexecuções com a correção do posto, que deram Ŝ e contagens de
  aceitos idênticos.
- **M = 72 (sensibilidade, para cima): toda execução cujo resultado foi visto.** As 54 execuções do F4 gravadas em
  `h2*/saida`, mais 2 diagnósticos, mais 6 reexecuções do posto, mais 10 ICPs da execução v1 do H2 (o Preston foi
  podado no F3 no v1).

Os pilotos do H2 (educação e logística, antes dos critérios) e as segundas execuções de `portoes.py --reproducao`
não entram: dos pilotos não há registro de ICP, e as reproduções são determinísticas.

### Estatística por teste

Para cada teste, com F = a família que conta (os não saturados, quando a regra de saturação do H2d está ativa; todos
os subconjuntos antes disso) e p_S o p-valor de invariância do subconjunto S:

- p_max = max_{S ∈ F} p_S;
- p_j = max_{S ∈ F, j ∉ S} p_S, o p-valor de variável do ICP (Peters, Bühlmann & Meinshausen 2016);
- **p_dom = min_j p_j.**

Como o ICP aceita S quando p_S > α, vale Ŝ ≠ ∅ no nível a se e somente se p_dom ≤ a < p_max. p_dom é um p-valor
válido para o erro "Ŝ traz um não pai": para todo não pai j, p_j ≥ p_{S*}, com S* = pa(Y). Rodar o ICP com α
dividido equivale a trocar a por α/M nessa condição; a saturação não depende de α. Antes de qualquer correção, eu
confiro que essa regra reproduz, no α original, o Ŝ gravado de cada teste. Se não reproduzir, a conta para e eu
relato.

Os p-valores por subconjunto vêm de `filtro4.icp.p_por_subconjunto` e `filtro4.icp.saturados` dos resultados
gravados. Para os 8 testes sem saída gravada (6 reexecuções do posto, 2 diagnósticos), rodo o ICP com o código do
próprio horizonte (H2b) sobre o treino que o funil do horizonte monta. Se não der, eles entram em M com p_dom = 1
para Holm e BY, e eu digo isso.

### Correções (α = 0,05)

1. **Bonferroni:** nível α/M por teste.
2. **Holm:** passo a passo descendente sobre os p_dom dos M testes. O teste de posto i (do menor ao maior) é
   comparado a α/(M − i + 1).
3. **Benjamini–Yekutieli** (FDR sob dependência arbitrária): passo a passo ascendente, com limiar i·α/(M·c_M) e
   c_M = Σ_{k=1}^{M} 1/k.
4. **Sequencial, com as 7 rodadas como olhadas.** O gasto de α é uniforme, α/7 por rodada: H2, H2b, H2c, H2d, H2e,
   H2f e H2g. Em cada rodada, Bonferroni sobre os m_r testes que a rodada avalia, inclusive os herdados: nível
   α/(7·m_r). Pela desigualdade da união, a chance de algum Ŝ com não pai em alguma olhada fica ≤ α. Com m_r:
   H2 11; H2b 11 (3 + 2 diagnósticos + 6 do posto); H2c 3; H2d 5; H2e 7; H2f 12; H2g 13.

Em Holm e BY, um teste conta como Ŝ ≠ ∅ quando é rejeitado pelo procedimento e p_max > o limiar do seu posto
(ainda há conjunto aceito naquele nível).

Como nota, sem decidir nada: o custo de Occam de escolher entre M tentativas é log₂ M bits. A evidência −log₂ p_dom
precisa passar de log₂ M + log₂(1/α), e isso equivale ao Bonferroni.

### Critério

- **Pergunta principal:** a saída aprovada do H2 sobrevive? O portão aprovado é o G3 do H2g: Ŝ ≠ ∅ em pelo menos 3
  domínios de áreas distintas. Ele sobrevive a uma correção se, no nível corrigido, pelo menos 3 áreas distintas
  da rodada H2g tiverem Ŝ ≠ ∅.
- **Correção que decide: Bonferroni com M = 38.** Holm, BY, a versão sequencial e M = 32 ou 72 são relatados ao
  lado, e eu digo onde discordam.
- **Áreas.** A área é o campo `area`, pela primeira palavra (Educação, Física, Genética, Farmacologia, Direito,
  Saúde, …). Aplico também, ao lado, a regra do H2f: um Ŝ = {plate} sozinho não conta.
- **Leitura agrupada, só informativa:** a união das descobertas corrigidas de todas as rodadas, contadas por área.
- **O que conta como falha da saída do H2:** menos de 3 áreas com Ŝ ≠ ∅ no nível de Bonferroni com M = 38.

### Confirmação com o código do horizonte

Copio `h2g/` para uma pasta de trabalho e mudo só o α. O funil já lê `alfa_icp` do pacote, então escrevo
`alfa_icp: α/38` nos 18 pacotes e nada mais. Rodo `rodar.py` e o G3 de `portoes.py`, sem `--reproducao`, e o
resultado tem de bater com a conta feita sobre os p-valores gravados. Se não bater, relato as duas.

## Ponto 4: ajustes depois de ver resultados

### H2, arcabouço v1 contra v2

O código v1 não está no git: o histórico tem um único commit da pesquisa (`4bb7181`) e só traz o v2. Reconstruo o
v1 numa cópia de `h2/`, revertendo as quatro mudanças descritas em `h2/README.md` e nos comentários "(v2)" de
`h2/estocologia/funil.py`:

1. **F3:** a chave é permutada linha a linha dentro de cada ambiente. É o que diz o cabeçalho de `funil.py`: "o
   nulo de permutação dentro do ambiente".
2. **Verossimilhança sem têmpera:** peso = 1.
3. **Conforme:** calibração num quinto do treino sorteado com o gerador do funil. O ajuste de máxima verossimilhança
   usa os outros quatro quintos, e os escores são os mesmos do v2 (resíduo na gaussiana, 1 − p da classe observada
   na binomial).
4. **Rótulos do ICP:** sem efeito numérico; não reconstruo.

**Validação da reconstrução.** O README diz que, no v1, energia e ciclones caíam na cobertura conforme, o crédito no
R-hat (1,011) e o Preston no F3. Se a reconstrução reproduzir os quatro, eu a trato como fiel. Se não, relato quais
batem e trato o resto como sensibilidade a uma reconstrução incerta.

Rodo os 12 domínios do H2 com o v1 reconstruído e com o v2 (cópia intacta) e mostro lado a lado. Como o v2 também
foi herdado pelo H2b … H2g, rodo o H2g com o v1 reconstruído: só o F3 pode mudar o G3, porque o G3 conta todo
domínio que chega ao F4, aprovado ou não no M15. Aplicar os mesmos três reversos ao `funil.py` do H2g é a única
mudança.

### H3, as quatro versões do estimador do laço performativo

Só a v4 (2SLS) existe no código. Reconstruo as outras três numa cópia de `h3/estocologia/performativo.py`, pela
descrição em `historico` do próprio módulo. Os choques são z_j = expl[d, j] ∈ {−0,3; 0; +0,3}, o excesso é
exc_j = obs_j − previsão_j e a_j são as desistências:

- **v1:** a escala do previsor controlado sai da previsão da demanda corrigida, não da carga observada. O r̂ usa
  um intervalo só: r̂ = Σ z_j·exc_{j+1} / (0,6·Σ z_j·a_j).
- **v2:** a escala serve a carga observada, como na v4. r̂ = Σ z_j·(exc_{j+1} + exc_{j+2}) / Σ z_j·a_j.
- **v3:** a razão da v2 com controle pré-choque: numerador Σ z_j·[(exc_{j+1} + exc_{j+2}) − (exc_{j−1} + exc_{j−2})]
  e denominador Σ z_j·(a_j − a_{j−1}).
- **v4:** o código atual, sem mudança.

As somas correm sobre os dias com previsão e os intervalos com os índices definidos. A semente é a do módulo (2027).

**Métrica:** o viés relativo do previsor controlado contra a demanda exógena, com r = 0,6, nos dias 60 em diante.
**Critério G4(b) do H3, sem mudança:** |viés| ≤ 3%.

**Validação:** o `historico` dá os números de referência: v1 r̂ = 0,55 e viés 6,6%; v2 r̂ = 0,70 e viés −4,8%;
v3 r̂ = 0,77; v4 r̂ = 0,56 e viés 1,7%. Relato quanto a reconstrução se afasta deles.

### H3, aberturas de cofre

Inventario as aberturas a partir de `h3/saida/*/especificacoes.jsonl`, dos barramentos e dos READMEs. Para as duas
correções depois de abrir o cofre, a da central (Erlang C) e a dos grupos de linhas do `prisoes_ri`, digo pelo código
se elas tocam as previsões. Se der, confiro a de `prisoes_ri` rodando a cópia com grupos de 32 linhas e comparando o
diário.

### "Se só a primeira versão declarada valesse"

Relato o veredito de cada portão com a primeira especificação declarada:

- **H2:** a rodada H2, com o v1, o teste K = 1 e α = 0,05.
- **H3 G4:** a v1 do estimador.
- **H3 G2:** os seis fluxos do CRITERIOS.md original, sem os cofres novos.
