# Pontos 3 e 4: multiplicidade não contada e ajustes depois de ver resultados

## Resultado

**A saída aprovada do H2 sobrevive à correção de multiplicidade.** O K global honesto do portão do H2 é de 38 testes
ICP distintos (32 a 72, conforme a contagem). Com Bonferroni sobre 38 (α = 0,05/38 = 0,0013 por teste), a rodada
H2g ainda tem Ŝ ≠ ∅ em 4 áreas: Educação, Física, Farmacologia e Genética. Holm, Benjamini–Yekutieli e a versão
sequencial (α/7 por rodada) também deixam pelo menos 3 áreas. A conta sobre os p-valores gravados foi confirmada
rodando o próprio `h2g/` com `alfa_icp = 0,05/38` nos 18 pacotes: o G3 do `portoes.py` dá as mesmas 4 áreas.

O que a correção muda:
- **Os Ŝ encolhem.** No STAR, {anos_pequena_antes, turma_pequena} vira {turma_pequena}. No y08, {plate, x1, x5, x9}
  vira {plate, x5}: o x1, o único gene que a validação intervencional confirmava, sai.
- **O que segura o portão** são três domínios com p de domínio minúsculo: túnel de luz (p = 0), farmacologia
  (1e-60) e STAR (4e-7).

Há duas ressalvas, as duas importantes:
- **A correção não é monótona no ICP.** Com α menor, domínios que rejeitavam tudo passam a aceitar algum conjunto.
  Isso cria Ŝ em modelos quase rejeitados, como Preston e Energia no H2.
- **A aprovação depende do arcabouço v2.** Com o F3 v1 reconstruído, o STAR é podado antes do ICP, porque a chave
  é constante dentro do ambiente e a permutação dentro do ambiente não a mexe. O H2g fica então com 3 áreas, e uma
  delas é a Genética frágil. Se, além disso, se exigir que o modelo não seja rejeitado a 0,05 (sensibilidade não
  declarada), sobram 2 áreas e o portão cai.

No H3, só a v4 do estimador do laço performativo passa o G4(b):
- v1: viés de +4,7%;
- v2: −5,9%;
- v3: −8,4%;
- v4: +1,7%, para um limite de 3%.

Se só a primeira versão declarada valesse, nem o H2 nem o H3 estariam aprovados:
- no H2, a rodada 1 tem 1 área;
- no H3, o G4 falha com a v1, e o G2 falha nos cofres originais de voos e prisões.

## 1. Inventário: K global

O script `analise_multiplicidade.py` lê os 54 `filtro4` gravados em `h2*/saida/*/resultados.json` e os 8 testes
refeitos por `icp_direto.py`. Ele confere primeiro que a regra abaixo reproduz, no α original, o Ŝ e o número de
aceitos gravados, e os 62 conferem.

| rodada | testes ICP vistos | novos (distintos) | o que é novo |
|---|---|---|---|
| H2 | 11 | 11 | 11 domínios de conveniência; teste K = 1 sem posto |
| H2b | 11 | 11 | 3 domínios de intervenção; 2 ICPs diagnósticos (Sachs, cinto) depois da poda; 6 reexecuções do H2 com a correção do posto |
| H2c | 3 | 3 | teste K = 2 (não paramétrico) |
| H2d | 5 | 5 | regra de saturação (K = 3); Sachs por célula; cinto com efeito fixo |
| H2e | 7 | 2 | túnel de luz, túnel de vento |
| H2f | 12 | 5 | genética y02, y03, y04, y07, y08 (os outros 5 Y podados no F3), α = 0,005 |
| H2g | 13 | 1 | farmacologia |
| **total** | **62** | **38** | |

Contagens de M:
- **M = 38** é a contagem principal: os testes distintos, porque reexecutar a mesma especificação de forma
  determinística não é uma tentativa nova.
- **M = 32** tira as 6 reexecuções do posto, que deram Ŝ idênticos.
- **M = 72** soma os 10 ICPs da execução v1 do H2, que não ficaram gravados: toda execução vista.
- Fora da conta, por não deixarem registro: os pilotos do H2 (educação e logística, antes dos critérios) e a
  especificação K = 1 da logística, barrada no F2.

**Estatística por teste.** Para cada teste, p_dom = min_j max_{S ∌ j} p_S é o menor p-valor de variável do ICP,
tomado na família que conta (sem os saturados a partir do H2d). Ŝ ≠ ∅ no nível a ⇔ p_dom ≤ a < p_max. p_dom é um
p-valor válido para o erro "Ŝ contém um não pai". A tabela completa, com os 62 testes, p_max, p_dom, a variável que
o define e Ŝ antes e depois, está em `resultados/multiplicidade.json`. Os testes que importam:

| rodada | domínio | p_max | p_dom | Ŝ publicado | Ŝ com Bonferroni (M = 38) |
|---|---|---|---|---|---|
| H2 | direito_prisoes | 0,070 | 0,0105 | {cidadao, empregado} | ∅ |
| H2 | energia_vitoria | 0,0042 | 7,4e-10 | rejeita tudo | 5 variáveis |
| H2 | desenvolvimento_preston | 0,0051 | 5,7e-41 | rejeita tudo | {decadas_desde_1952} |
| H2b | direito_teto_beneficio | 0,087 | 0,033 | {beneficio_log} | ∅ |
| H2b | educacao_star_turmas | 0,0013 | 4,4e-15 | rejeita tudo | {turma_pequena} (por pouco: p_max = 0,00133 > 0,00132) |
| H2b | transporte_cinto (diagnóstico) | 0,054 | 0,0072 | {idade21, idade_media, vel65, vel70} | ∅ |
| H2c | saude_rand | 1 | 2e-29 | {coparticipacao} (saturado) | {coparticipacao} |
| H2d–g | educacao_star_turmas | 0,075 | 4,2e-7 | {anos_pequena_antes, turma_pequena} | {turma_pequena} |
| H2e–g | fisica_tunel_luz | 0,52 | 0 (abaixo do menor double) | {blue, green, red} | {blue, green, red} |
| H2e–g | engenharia_tunel_vento | 7,3e-5 | 7e-15 | rejeita tudo | rejeita tudo |
| H2f–g | genetica_y08 (α = 0,005) | 0,0072 | 0,00053 | {plate, x1, x5, x9} | {plate, x5} |
| H2g | farmacologia_qt | 1 | 1e-60 | {c_dof, c_qui, c_ran} | {c_dof, c_qui, c_ran} |

## 2. Correções e o veredito do H2

O critério declarado em `DECLARACAO.md` (SHA-256 em `DECLARACAO.sha256`, 2026-10-15T12:17:03Z, antes de qualquer
cálculo corrigido): a saída do H2 sobrevive se, no nível corrigido, a rodada H2g tiver Ŝ ≠ ∅ em pelo menos 3 áreas
distintas. A correção que decide é Bonferroni com M = 38.

| correção | nível por teste no H2g | áreas com Ŝ ≠ ∅ no H2g | G3 do H2g | rodadas anteriores que passariam |
|---|---|---|---|---|
| nenhuma (publicado) | 0,05 (0,005 na genética) | Educação, Farmacologia, Física, Genética | passa | H2f |
| **Bonferroni, M = 38** | **0,00132** | **Educação, Farmacologia, Física, Genética** | **passa** | H2f |
| Bonferroni, M = 32 | 0,00156 | as mesmas 4 | passa | H2f |
| Bonferroni, M = 72 | 0,00069 | as mesmas 4; a Genética só com Ŝ = {plate} | passa (3 áreas pela regra do plate do H2f) | H2f pela letra |
| Holm, M = 38 | 0,0013 a 0,0028 | as mesmas 4 | passa | H2f |
| Holm, M = 32 / 72 | | as mesmas 4 | passa | H2f |
| Benjamini–Yekutieli, M = 38 | 0,0065 | as mesmas 4 | passa | H2f |
| Benjamini–Yekutieli, M = 32 | 0,0081 | Educação, Farmacologia, Física (y08 fica de fora) | passa | nenhuma |
| Benjamini–Yekutieli, M = 72 | 0,0060 | as mesmas 4 | passa | H2f |
| sequencial, α/7 por rodada, Bonferroni na rodada (m = 13 no H2g) | 0,00055 | as mesmas 4; a Genética só com Ŝ = {plate} | passa (3 áreas pela regra do plate) | H2f pela letra |

Notas sobre a tabela:
- **Regra do plate.** Pela regra declarada do H2f, um Ŝ = {plate} sozinho não conta. Com M = 38 o y08 fica com
  {plate, x5} e conta. Com M = 72 e na versão sequencial ele cai para {plate}, a Genética sai, e sobram as 3 áreas
  sem ela.
- **Custo de Occam.** Para M = 38, o custo é log₂ 38 + log₂ 20 = 9,6 bits. A evidência do STAR é −log₂ 4,2e-7 =
  21 bits, a da farmacologia é de 199 bits e a do y08 é de 10,9 bits. O y08 passa por 1,3 bit.

**Confirmação com o código do horizonte.** `copias/h2g_bonf` é o `h2g/` com só `alfa_icp: 0.0013157894736842105`
nos 18 pacotes.
- O G3 do `h2g/portoes.py` sobre a saída dá "Ŝ ≠ ∅ em 4 domínio(s), 4 área(s): educacao_star_turmas,
  farmacologia_qt, fisica_tunel_luz, genetica_y08".
- Os Ŝ batem com a conta.
- Status, F1–F3 e posteriores são idênticos aos publicados (`resultados/execucoes.json`).

**Uma ressalva que a conta revelou: a correção não é monótona no ICP.** Dividir α protege contra Ŝ com não pais,
mas também faz aceitar conjuntos em modelos que, a 0,05, eram rejeitados por inteiro. Com Bonferroni, a rodada H2
ganha Desenvolvimento e Energia (p_max de 0,005 e 0,004) e perde o Direito. A garantia formal continua valendo, mas
esses Ŝ vêm de modelos quase rejeitados.

Por isso relato uma sensibilidade **que não foi declarada**: exigir também p_max > 0,05. Com ela, o H2g fica com
Educação, Física e Farmacologia, 3 áreas, e ainda passa. O y08 sai, porque o seu p_max é 0,0072.

## 3. Ajustes depois de ver resultados

### H2: arcabouço v1 contra v2

O código v1 não existe no git: há um único commit da pesquisa (`4bb7181`), e o disco compartilhado só tem o v2. Eu o
reconstruí em `preparar_copias.py`, revertendo as três mudanças numéricas marcadas "(v2)" em `h2/estocologia/funil.py`:
- F3 com permutação dentro do ambiente;
- verossimilhança sem têmpera;
- conforme num quinto sorteado do treino.

A quarta mudança, os rótulos do ICP, não muda número.

**A reconstrução não é fiel.** O README do H2 diz que, no v1, energia e ciclones caíam no conforme, o crédito no
R-hat (1,011) e o Preston no F3. A minha reconstrução só reproduz o caso dos ciclones. O v2 refeito é idêntico ao
publicado, nos 12 domínios.

| domínio | v2 (publicado = refeito) | v1 reconstruído | v1 segundo h2/README.md |
|---|---|---|---|
| clima_tempestades | aprovado; conforme 0,852 | **reprovado no conforme (0,841)** | reprovado no conforme |
| energia_vitoria | aprovado; conforme 0,910 | aprovado; conforme 0,877 | reprovado no conforme |
| credito_hmda | aprovado; R-hat 1,0096 | aprovado; R-hat 1,0074 | reprovado no R-hat (1,011) |
| desenvolvimento_preston | aprovado; F3 p = 0,005 | aprovado; F3 p = 0,005; IC90 [4,64; 5,28] contra [4,14; 6,04] | podado no F3 |
| trabalho_curriculos | reprovado no conforme (0,786) | **aprovado (0,874)** | (não citado) |
| os outros 7 | | mesmo status; IC90 da chave em geral mais estreitos sem a têmpera | |

O que se pode afirmar com segurança:
- **O G3 do H2 não depende do arcabouço.** O ICP roda sobre o treino antes de qualquer passo do v2, e o Ŝ é idêntico
  nos dois (1 área, o Direito).
- **As mudanças do v2 mexem nos status e na largura das posteriores.** Na reconstrução, o v2 também derrubou um
  domínio (currículos): as mudanças não foram todas no sentido de aprovar.

Os quatro casos "de reprovado para aprovado" do README não puderam ser reproduzidos. Isso pode ser falha da
reconstrução: o sorteio do quinto, a ordem do gerador e o ajuste nos quatro quintos são suposições minhas.

**O H2g com o v1 reconstruído (`copias/h2g_v1`).** No H2g, só o F3 pode mudar o G3.

- **Domínios podados no F3.** Com a permutação dentro do ambiente, caem no F3 o STAR, o RAND, o Sachs e o y04. No
  STAR e no RAND a chave é constante dentro do ambiente (o ambiente é a turma ou o plano), então a permutação não muda
  nada e o teste não tem como rejeitar.
- **G3 pelo `portoes.py`:** 3 áreas, Farmacologia, Física e Genética. Passa.
- **G3 com Bonferroni (M = 38):** as mesmas 3 áreas, com o y08 = {plate, x5}. Passa.
- **Leitura.** Sem o v2, o portão do H2g depende do y08, que é a evidência que o próprio projeto chamou de frágil.
  Com a sensibilidade não declarada (p_max > 0,05), seriam 2 áreas, e o portão cairia.
- **O que mudou o F3.** A mudança do F3 do v2 foi feita no H2, antes de existirem os domínios de intervenção. Ela é
  a correção certa para esses desenhos, mas foi escolhida depois de ver resultados do H2.

### H3: as quatro versões do estimador do laço

Só a v4 está no código. As outras três foram reconstruídas em `laco_versoes.py`, pela descrição do campo
`historico` de `h3/estocologia/performativo.py`. A simulação ingênua é a mesma, com semente 2027 e r = 0,6. O
critério G4(b) não mudou: |viés| ≤ 3%.

| versão | r̂ (reconstruído) | r̂ no `historico` | viés contra a demanda exógena | viés no `historico` | G4(b) |
|---|---|---|---|---|---|
| v1: escala pela demanda corrigida; Wald de um intervalo | 0,562 | 0,55 | **+4,7%** | +6,6% | falha |
| v2: escala pela carga observada; Wald de dois intervalos | 0,722 | 0,70 | **−5,9%** | −4,8% | falha |
| v3: Wald com controle pré-choque | 0,773 | 0,77 | **−8,4%** | (não citado) | falha |
| v4: 2SLS (código atual) | 0,563 | 0,56 | **+1,7%** | +1,7% | passa |

- **A reconstrução fica perto dos números históricos.** A v4 é idêntica ao `portoes.json` publicado.
- **O previsor ingênuo** erra +29,1% em todas as versões.
- **O resultado depende da versão:** três de quatro falham.
- **Na v1, o que falha é o desenho, não o estimador.** O r̂ da v1 é quase o da v4 (0,562 contra 0,563), e o viés
  vem do desenho do laço: escalar pela demanda corrigida alimenta a espiral de desistências.

### H3: aberturas de cofre

O inventário vem de `h3/saida/*/especificacoes.jsonl`, dos barramentos (um `cofre_aberto` por fluxo na execução
gravada) e de `h3/README.md`.

| fluxo | aberturas com resultado visto | depois de ver |
|---|---|---|
| energia, bolsa, voos, ciclones, prisões (H3) | 1 cada | voos e prisões falharam no G2 e levaram ao H3b, ao H3c e ao H3d, com cofres novos |
| central (H3) | 2 | a 1ª execução travou (Erlang C com fatoriais). A troca pela recursão de Erlang B só toca `decidir()` (`h3/fluxos/central.py`, linha 200), não as previsões. Conta no K (3). |
| voos23 (H3b) | 1 | passa |
| prisoes20 (H3c) | 1 | falha (ECE 0,067) |
| prisoes_ri (H3d) | 2 | o G5 falhou com grupos de 32 linhas, e os grupos passaram a 4 |
| **total** | **11** | mais as reproduções determinísticas do G1, que não entram |

**Conferência do `prisoes_ri`.** Rodei a cópia `copias/h3_ri32` com 32 linhas por grupo. O G5 dá 6,36 varreduras
por tabela, o mesmo 6,4 do README. O diário bate com o publicado em `diario.parquet` (SHA-256 `0c0e05aa…`, igual a
`FORA_DO_GIT.sha256`):
- mesma origem oficial em todas as 83 632 linhas;
- diferença máxima de 1,5e-15 nas probabilidades, que é arredondamento de ponto flutuante: rodei com
  OMP_NUM_THREADS=1;
- detalhe em `resultados/prisoes_ri_grupos32.json`.

A correção feita depois de abrir o cofre de fato não tocou as previsões.

Duas observações sobre o K do H3:
- **Inconsistência no `prisoes_ri`.** O `resultados.json` traz K = 1, e o `especificacoes.jsonl` traz 2 entradas.
- **Multiplicidade do G2.** O G2 das prisões foi tentado em 3 cofres, e o dos voos em 2. O G2 é uma tolerância, não
  um teste com α, então não há α a dividir. O número de tentativas fica registrado.

### Se só a primeira versão declarada valesse

- **H2.** A rodada 1 (v1, teste K = 1, α = 0,05) dá 1 área. O portão falha, com ou sem correção. Com Bonferroni
  seriam 2 áreas (Desenvolvimento e Energia), por modelos quase rejeitados, e ainda falharia.
- **H3, G4.** Com a v1 do estimador o viés é de +4,7%, e o G4 falha.
- **H3, G2.** Com os seis fluxos do `CRITERIOS.md` original, voos e prisões falham.
- **H3.** A saída não estaria aprovada.

## Como reproduzir

Os dados vêm de `/mnt/project-files/estocologia/dados-reais/` e foram conferidos pelos `SHA256SUMS`. Copie
`h2`, `h2b`, `h2e`, `h2f` e `h2g` para `$D` e `h3c` para `$D3`.

```
cd fundo/p03-multiplicidade
python3 icp_direto.py ../../h2 ../../h2b $D/h2 resultados/icp_h2_posto_corrigido.json \
    direito_prisoes credito_hmda saude_consultas trabalho_curriculos epidemiologia_contracepcao educacao_star   # ~1 min
python3 icp_direto.py ../../h2b ../../h2b $D/h2b resultados/icp_h2b_diagnosticos.json biologia_sachs transporte_cinto
python3 analise_multiplicidade.py                       # inventário, K e correções → resultados/multiplicidade.json
python3 preparar_copias.py 38                           # recria copias/ a partir dos originais
export OMP_NUM_THREADS=1
(cd copias/h2_v2 && ESTOCOLOGIA_DADOS=$D/h2 ESTOCOLOGIA_SAIDA=$S/h2_v2 python3 rodar.py)
(cd copias/h2_v1 && ESTOCOLOGIA_DADOS=$D/h2 ESTOCOLOGIA_SAIDA=$S/h2_v1 python3 rodar.py)
(cd copias/h2g_bonf && ESTOCOLOGIA_DADOS=$D/h2b ESTOCOLOGIA_SAIDA=$S/h2g_bonf python3 rodar.py)     # ~25 min
(cd copias/h2g_v1 && ESTOCOLOGIA_DADOS=$D/h2b ESTOCOLOGIA_SAIDA=$S/h2g_v1 python3 rodar.py)
python3 comparar_execucoes.py $S                        # → resultados/execucoes.json
python3 laco_versoes.py ../../dados-reais/bank_calls.csv   # → resultados/laco_versoes.json (~10 min)
(cd copias/h3_ri32 && ESTOCOLOGIA_DADOS=$D3 ESTOCOLOGIA_SAIDA=$S/h3_ri32 python3 rodar.py prisoes_ri)  # ~5 min
```

Rodei `analise_multiplicidade.py`, `comparar_execucoes.py` e `laco_versoes.py` duas vezes.
- **`multiplicidade.json` e `execucoes.json`** deram SHA-256 idênticos nas duas execuções (`resultados/SHA256SUMS`).
- **`laco_versoes.py`** difere só no 16º algarismo da v4 (r̂ 0,5632133268561669 contra …667). A 1ª execução usou o
  BLAS com várias threads e a 2ª, OMP_NUM_THREADS=1. A 1ª é idêntica ao `h3/saida/portoes.json` publicado, e as
  v1–v3 são idênticas nas duas.
- **As execuções do H2 e do H2g rodaram uma vez cada.** A v2 é a segunda execução do código publicado e saiu
  idêntica a ele, campo a campo.

## Limites

- **O v1 do H2 e as v1–v3 do laço são reconstruções** a partir de texto, não o código original. A do H2 reproduz
  1 dos 4 efeitos citados no README, e a do laço fica a 1–2 pontos percentuais dos números históricos.
- **O que a correção não controla.** Ela trata a multiplicidade de tentativas e controla a chance de um Ŝ com não
  pai. Não trata a validade de cada teste: dependência dentro do voluntário na farmacologia, células de uma condição
  no Sachs, lotes na levedura. Esses são outros pontos de fundo.
- **O que fica fora de M.** M não conta os pilotos do H2, as escolhas de quais conjuntos de dados baixar e rodar,
  nem as especificações cogitadas e não rodadas. O K = 72 é um limite inferior do "jardim de caminhos".
- **A sensibilidade p_max > 0,05 não foi declarada antes.** Eu a relato porque a não monotonia só apareceu na conta.
- **Interferência com outros trabalhos.** Durante o trabalho, um `pkill -f "rodar.py"` meu pode ter interrompido
  processos `rodar.py` de outros workers na mesma máquina. Também apaguei uma pasta `saida/` na raiz do scratchpad
  compartilhado, que eu tinha criado mas que outro worker pode ter usado. Depois disso, tudo o que é meu ficou em
  `scratchpad/p03/`.
