# Ponto 10 (saúde): o círculo do Lab 0, fechado com dado aberto

## Resultado

O primeiro candidato da ordem declarada — o **PhysioNet/CinC Challenge 2019 (sepse precoce)**, no espelho aberto do
PhysioNet no S3 — passa os quatro itens da regra de seleção R0, e é ele que entra. Os outros três não passam:
o **challenge-2012** cai no item 3 (o desfecho é a morte intra-hospitalar, estado vital registrado, não decisão
clínica), o **ctu-uhb-ctgdb** cai no item 1 (552 partos, a unidade independente não chega a 10 000) e o
**cerebral-vasoreg-diabetes** cai no item 1 (dezenas de sujeitos).

O funil rodou de ponta a ponta, sem uma linha mudada, e **a alegação não se sustenta**:

| filtro | número | passa? |
|---|---|---|
| F1 · EVPI da decisão contra o custo da pesquisa | EVPI 0,002014 contra custo 0,0005; P(adotar compensa) = 0,389 | **sim** |
| F2 · porta dos fundos e informatividade | 5 466 linhas = 5 466 internações de treino; EP da chave 0,007734 contra dp da priori 0,152957; efeito de desenho 0,950 | **sim** |
| F3 · nulo de permutação, entre unidades | z observado 4,981; q95 da nula 1,970; p = 0,004975 (200 permutações) | **sim** |
| F4 · ICP entre ambientes | **0 de 512 subconjuntos aceitos; Ŝ = ∅** (nenhum conjunto não saturado aceito) | **não** |

**O que o Ŝ = ∅ diz aqui.** O teste de invariância não aceitou nenhum subconjunto dos 9 candidatos nas três UTIs do
primeiro sistema (MICU, SICU, outros): o melhor subconjunto dos 512 tem p = 2,1e-07, ou seja, em nenhuma leitura a
lei de y dado X é a mesma nos três sítios de treino. Pela regra declarada ("confirma se o funil chega ao F4, o Ŝ
contiver fr, a habilidade no cofre for positiva e o M15 aprovar; falha se o Ŝ não contiver a chave"), o domínio
**falha**, e o que fica é *efeito sem invariância certificada*, não confirmação.

**O efeito existe e é preciso — o que não existe é a regularidade.** A posterior da chave: β = 0,0388 em log-razão
de chances por respiração por minuto (IC90 0,0257 a 0,0523), P(β > 0) = 0,9999, com rhat 1,0043 e ess 1 357. O
F3 confirma o mesmo pelo nulo: a FR da primeira janela marca a decisão de sepse registrada a 5 desvios do nulo de
permutação. O que cai é a **invariação entre ambientes**, que é o produto do ICP.

**O cofre é o segundo sistema hospitalar (Emory), 15 977 internações, nunca vistas no ajuste.** E o quadro que o
manuscrito do desafio anuncia aparece inteiro:

| leitura | habilidade contra a climatologia no cofre |
|---|---|
| modelo ajustado (todos os candidatos) | **+0,00568** |
| todos os candidatos, ajuste conjunto | +0,00791 |
| invariante (Ŝ = ∅, só o intercepto) | −1,0e-09 |

A deriva do desfecho entre treino e cofre tem KS p = 2,2e-07, e a taxa é a razão dela: **8,4% nas internações do
sistema A e 3,9% nas do sistema B**. O mesmo registro que o rótulo mede é decidido duas vezes mais num sistema do
que no outro.

**Portão de calibração do p08 (tolerância de ECE dependente de n).** Nenhum dos três braços do diário passa, e o
motivo é um só: o nível.

| braço | n | ECE | q95 da nula | passa T95? | p |
|---|---|---|---|---|---|
| modelo ajustado | 2 000 | 0,03306 | 0,02156 | não | 0,000 |
| todos os candidatos | 2 000 | 0,03331 | 0,02124 | não | 0,000 |
| invariante (Ŝ = ∅) | 2 000 | 0,03679 | 0,02224 | não | 0,000 |

O ECE é praticamente todo o CITL (+0,033 nos dois modelos que preveem algo): o modelo calibrado nas UTIs do
sistema A prevê 8,0% de decisões registradas num cofre onde 4,7% acontecem. É a mesma conclusão do p08, num cofre
novo: com tolerância que depende de n, o cofre binário reprova.

**Portões do funil.** G1 passa (pytest: 23 testes, e a reprodução limpa é idêntica campo a campo), G2 passa (funil
completo, pré-registro antes da leitura do cofre, SHELF com erro máximo nos quartis 0,0027 contra 10% de 0,153),
G4 passa (2 leituras no armazém, nenhum \`SELECT *\`, projeção contida, cofre só no F3, toda a banca de ataques
recusada), o barramento fecha com 11 fichas sem quebra e o M15 aprova 1 de 1. **G3 falha** (ele pede Ŝ ≠ ∅ em 3
áreas distintas; este domínio dá Ŝ = ∅ e é o único da pasta), então a "saída do H2" fica **NÃO aprovada** por esse
critério — é o resultado, não um defeito de execução.

Resumo em uma linha: **a decisão de sepse registrada responde à FR da primeira janela com um efeito preciso, mas
nem as três UTIs do primeiro sistema compartilham a mesma lei (0 de 512 subconjuntos aceitos), nem o segundo
sistema tem a mesma taxa de decisão (3,9% contra 8,4%), e a calibração reprova em todos os braços.**

## O que foi feito

1. **Declaração com hash antes de qualquer desfecho.** `DECLARACAO.md` (e `DECLARACAO.sha256`, com hora UTC), com
   a regra de seleção R0 e a leitura de cada item, a ordem dos quatro candidatos, a alegação, a priori de
   literatura (conferida na web), a decisão, o termo de escopo e a previsão do autor. A declaração lista, item por
   item, **o que já tinha sido visto** antes do hash: a listagem do espelho, o cabeçalho e **duas linhas de um
   único arquivo** (as duas com SepsisLabel = 0), o padrão das flags Unit1/Unit2 em 300 arquivos (contagem por
   arquivo, nunca o desfecho), a listagem dos outros três candidatos e o texto do manuscrito do desafio (sistemas
   hospitalares e definição do rótulo). Nenhuma contagem ou média de desfecho foi feita antes do hash.
2. **Download do conjunto escolhido** para `dados-reais/p10/`, por `fundo/p10-saude/codigo/baixar.py`: 40 336 arquivos (20 336 do
   sistema A, 20 000 do B), 255,1 MB, 1 395,3 s (23,3 min) com 24 threads, nenhuma falha. Dois manifestos de soma,
   os dois conferidos: 40 336 OK / 0 FAILED cada um. Origem, licença, referências e o que o espelho publica (ou
   não) de soma estão em `dados-reais/p10/FONTES.md`.
3. **Domínio no formato do pacote do H2g**, espelhando `fundo/p09-escala-dados/dominios/saude_indometacina_ercp/`:
   `dominios/saude_sepse_alerta/pacote.yaml` (pergunta, alegação, 14 termos de glossário, fontes, DAG de 15 arestas, ajuste, ICP
   com 9 candidatos, ambientes, cofre, SHELF, decisão, escopo, K = 1) e `preparo.py` (uma linha por internação, a
   janela de 6 h, o gatilho, os recortes e o sítio). Linha = unidade = a internação; ambiente = sítio = sistema ×
   tipo de UTI; cofre = os três níveis do sistema B.
4. **Funil sem mudança**: `estocologia/` (ICP, d-separação, amostrador, armazém, barramento, diário, GLM, funil),
   `rodar.py` e `portoes.py` são cópia do p09 sem uma linha mudada. Rodou em **19,9 s** com
   `OMP_NUM_THREADS=1` e um processo só. Semente 2027 (a do `rodar.py` para o primeiro domínio).
5. **Portão de calibração**: `calibracao_ece.py`, com o `ece` e a `nula_ece` copiados do
   `fundo/p08-calibracao` sem mudança (mesma semente 20261015, B = 2000, caixas de mesmo tamanho).
6. **Diagnóstico dos recortes** (`codigo/diagnostico_recortes.py`), para o README poder dizer **por que** a amostra é o que
   é: das 40 336 internações, 21 443 entram; 8 793 saem porque a janela não é ICULOS 1 a 6; 618 já tinham alarme
   dentro da janela; e 9 482 saem por candidato ausente na janela (5 292 delas só por `pad`, a diastólica).

| sítio | n | taxa de y | mediana de fr |
|---|---|---|---|
| A_MICU (treino) | 821 | 9,99% | 21 |
| A_SICU (treino) | 1 285 | 3,81% | 20 |
| A_outros (treino) | 3 360 | 9,73% | 20 |
| B_MICU (cofre) | 5 561 | 3,60% | 21,5 |
| B_SICU (cofre) | 5 678 | 4,40% | 20 |
| B_outros (cofre) | 4 738 | 3,82% | 19 |

## Limites

- **Dois arquivos mudaram depois do hash, e nenhum critério mudou.** O `pacote.yaml` tinha dois valores com `:` sem
  aspas no glossário e o YAML não carregava (o funil nem começava); as duas linhas foram citadas. O
  `codigo/baixar.py` gravava o prefixo do espelho duplicado; a árvore foi movida para o layout do espelho e o caminho foi
  corrigido. Os hashes de antes estão em `dominio.sha256` e os de depois em `dominio_2.sha256`; nada foi reescrito. **K
  continua 1**, porque nenhuma regra, priori, decisão ou recorte mudou — só a sintaxe do YAML e o caminho de
  gravação.
- **O horário da declaração.** Ela diz que o download começou às 16:21 UTC; o relógio dos arquivos desta máquina é
  local (UTC−3), então foram 19:21 UTC, e o hash saiu às 19:24:49Z — o download começou antes do hash, como a
  própria declaração relata, e nenhum conteúdo foi lido por causa disso.
- **A linha é a unidade, então o F2 avisa e não agrupa.** "uma linha por unidade: nenhum agrupamento declarado; o
  EP pode ser otimista se houver dependência não modelada". A dependência que existe é a de pacientes no mesmo
  sítio (mistura e protocolo), e ela não é modelada.
- **Os dois sistemas não são simétricos na amostra.** O sistema A perde muito mais internações nos recortes
  (5 466 de 20 336) do que o B (15 977 de 20 000): a monitoração das primeiras horas e os campos de laboratório
  não são registrados do mesmo jeito nos dois prontuários. O treino ficou **menor que o cofre**, o que não estava
  previsto e é uma assimetria do dado, não do desenho.
- **O desfecho é uma decisão registrada, não um desfecho fisiológico.** Quem pede hemocultura e antibiótico olha
  a FR; a chave e o rótulo compartilham a indicação. Todo o resultado vale como **lei da decisão registrada**, não
  como efeito causal da FR sobre a sepse. O SOFA, que também entra no rótulo, não está todo medido aqui: o ICP
  rejeitar **todos** os subconjuntos é consistente com pai oculto, e não só com heterogeneidade verdadeira.
- **O rótulo é grosseiro de propósito**: y = 1 se houver alarme em qualquer hora depois da janela, sem usar o
  instante de início. "Precoce" aqui quer dizer "a decisão veio depois da janela", nada mais fino que isso.
- **A taxa-base da priori ficou longe do observado.** A declaração fixou 5% sem conferir o dado; o treino tem
  8,4%. O F1 (que usa a priori) passa com EVPI 0,002014 contra 0,0005 — a decisão de passar não viraria com a
  taxa observada, mas o número do F1 carrega esse 5% declarado.
- **O ICP não tem intervalo para a chave**, porque não houve conjunto aceito: o dicionário de intervalos sai vazio.
- **G3 falha e a "saída do H2" não é aprovada.** O critério pede Ŝ ≠ ∅ em 3 áreas distintas; esta pasta tem um
  domínio só, e ele dá Ŝ = ∅.
- **A calibração do teste (o `calibracao.py` do p09, simulação de K = 1 contra K = 2) não foi rodada**: ela é um
  estudo de simulação daquele ponto, não faz parte do funil pedido aqui. O portão de calibração pedido é o do p08,
  e esse rodou.
- **O que não deu para fazer, por escopo.** O `.gitignore` e o `FORA_DO_GIT.md` (raiz) são fora da minha área de
  escrita: **40 336 arquivos novos e não rastreados, 255 MB, ficam em `dados-reais/p10/`**, e um `git status` a
  partir da raiz vai pescar. A regra da casa (dado bruto acima de 10 MB fica fora do Git com `FONTES.md` e
  `SHA256SUMS`, e o commit correspondente) pediria uma linha em `.gitignore` e uma entrada no `FORA_DO_GIT.md` —
  quem tem esse escopo decide. Nada foi commitado nem empurrado, como pedido.

## Como reproduzir

\`\`\`
cd <raiz do repositório>
python3 fundo/p10-saude/codigo/baixar.py dados-reais/p10/challenge-2019/1.0.0        # 40 336 arquivos, ~23 min
cd dados-reais/p10 && find challenge-2019 -name "*.psv" -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS
sha256sum -c SHA256SUMS                                                              # 40 336 OK
cd <raiz> && export OMP_NUM_THREADS=1
export ESTOCOLOGIA_DADOS=<raiz>/dados-reais/p10 ESTOCOLOGIA_SAIDA=<raiz>/fundo/p10-saude/saida
python3 fundo/p10-saude/rodar.py                       # F1–F4 + decisão do F8, 19,9 s
python3 fundo/p10-saude/portoes.py --reproducao        # G1–G5 e a reprodução limpa
python3 fundo/p10-saude/calibracao_ece.py              # portão de ECE com tolerância que depende de n
python3 fundo/p10-saude/codigo/diagnostico_recortes.py <raiz>/dados-reais/p10
\`\`\`

Os testes: `cd fundo/p10-saude && python3 -m pytest -q tests` (23 testes; os quatro arquivos de teste dos domínios do
p09 não vieram na cópia, e no lugar entrou o do domínio novo, `tests/test_p10.py`). As somas dos arquivos de código e dado
estão em `DECLARACAO.sha256`, `dominio.sha256`, `dominio_2.sha256` e `dados-reais/p10/SHA256SUMS`.

## Nota sobre ferramentas

Nenhuma skill do catálogo desta sessão serve a este repositório (o catálogo é do curso PCEM e do LoopX), então
tudo aqui saiu de bash e Python diretos: urllib para o espelho S3, pandas/numpy/scipy para a tabela, e o funil do
p09 copiado. Nada de MCP.
