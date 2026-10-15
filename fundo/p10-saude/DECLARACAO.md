# Declaração do fundo/p10 (saúde: alerta de sepse), escrita antes de rodar o funil

Data: 2026-10-15. Pasta: \`fundo/p10-saude/\`, que usa o funil de \`fundo/p09-escala-dados/\` (cópia sem uma linha
mudada: \`estocologia/\`, \`rodar.py\`, \`portoes.py\`, \`conftest.py\`, \`tests/\`). O hash desta declaração fica em
\`DECLARACAO.sha256\`, com a hora UTC. Qualquer mudança depois do hash entra como declaração nova e conta no K.

Objetivo do bloco: fechar o círculo de saúde que o Lab 0 abriu, com dado aberto e uma decisão clínica de verdade.

## O que já foi visto antes desta declaração (para ninguém achar que é cego)

- **Listagem do espelho** \`https://physionet-open.s3.amazonaws.com/\` (sem credencial): os prefixos, os arquivos
  e os tamanhos dos quatro candidatos da ordem. Nenhum conteúdo de dado lido nessa listagem.
- **Cabeçalho e duas linhas** de \`challenge-2019/1.0.0/training/training_setA/p000001.psv\` (41 colunas; as duas
  linhas mostram \`SepsisLabel = 0\`, e nada mais do desfecho). Fiz isso para saber os nomes das colunas.
- **Padrão das flags \`Unit1\`/\`Unit2\`** nos 300 primeiros pacientes do conjunto A, contado por arquivo (só o
  conjunto de valores de cada flag, nunca o desfecho): 126 pacientes sem flag, 91 só com \`Unit2 = 1\`, 83 só com
  \`Unit1 = 1\`; nenhum paciente com as duas. Nenhuma linha de desfecho foi tabulada.
- **Manuscrito do desafio** (\`challenge-2019/1.0.0/physionet_challenge_2019_ccm_manuscript.pdf\`, lido só por
  busca de texto): os três sistemas hospitalares (A = Beth Israel Deaconess Medical Center, B = Emory University
  Hospital, C não identificado), o fato de A e B serem os públicos e C o sequestrado, e a definição do rótulo
  ("antibióticos IV e hemoculturas dentro de um intervalo de tempo", mais a disfunção de órgão). **Não li a
  tabela de prevalência nem qualquer número de desfecho do manuscrito.**
- **Listagem dos outros três candidatos**: 2012 com \`set-a/b/c\` e \`Outcomes-a.txt\` (só o nome); \`ctu-uhb-ctgdb\`
  com 552 pares \`.dat\`/\`.hea\`; \`cerebral-vasoreg-diabetes\` com dezenas de arquivos por sujeito.
- **O download do conjunto escolhido começou às 16:21 UTC**, antes do hash desta declaração (o download é lento:
  40 336 arquivos pequenos). Nenhum arquivo baixado por esse processo foi aberto ou tabulado até aqui.
- **Literatura conferida na web** (15/16-10-2026): Sepsis-3 (Singer et al. 2016, JAMA 315:801, DOI
  10.1001/jama.2016.0287) e os critérios clínicos do qSOFA (Seymour et al. 2016, JAMA, DOI 10.1001/jama.2016.0288:
  "1 point each for systolic hypotension [<=100 mm Hg], tachypnea [>=22/min], or altered mentation"), os dois pelo
  Europe PMC (título, revista, ano e resumo conferidos).
- **Ainda não vi**: nenhuma contagem de \`SepsisLabel\` por sítio, nenhuma prevalência, nenhum ajuste, nenhuma
  média de nenhuma variável do conjunto escolhido.

## Parte 1. A regra de seleção (R0) e a ordem dos candidatos

A regra foi fixada pelo plano (Bloco E) **antes** desta declaração. Estes são os quatro itens e a leitura
operacional que uso em cada um, sem mexer em nenhum deles:

1. **Unidade independente com ≥ 10 000 linhas.** Leitura: o conjunto tem uma coluna que identifica a unidade (a
   internação, o parto, o sujeito) e, colapsado a uma linha por unidade, dá ≥ 10 000 linhas. Contar linhas de
   minuto ou de hora não vale: a unidade é o que é independente.
2. **≥ 2 ambientes definíveis SEM usar o desfecho** (sítio, coorte, campanha ou período). Leitura: ≥ 2 ambientes
   **de treino** mais o cofre declarado. O teste de invariância do funil (F4) precisa de dois ambientes de treino
   e o funil não tem cofre opcional (\`checar_tabela\` recusa "menos de 2 ambientes de treino"), então um conjunto
   com só dois níveis no total não roda o F4 sem mudar o funil, e o funil não muda.
3. **Desfecho que corresponda a uma decisão clínica registrada.** Leitura: o desfecho **é** o registro de uma
   decisão (uma ordem ou um ato clínico), e não um valor de laboratório, um escore derivado ou um estado vital.
4. **Download aberto.** Espelho S3 do PhysioNet, sem credencial, com os arquivos listáveis.

Aplicação, na ordem declarada:

| candidato | 1 · unidade | 2 · ambientes | 3 · desfecho = decisão registrada | 4 · aberto | entra? |
|---|---|---|---|---|---|
| \`challenge-2019\` | ✓ 40 336 internações (20 336 em A, 20 000 em B) | ✓ sítio = sistema × tipo de UTI (3 níveis em A, 3 em B) | ✓ o rótulo é a decisão registrada (antibiótico IV + hemocultura) com a disfunção de órgão | ✓ espelho S3 | **sim** |
| \`challenge-2012\` | ✓ 12 000 internações (3 × 4 000) | ✓ três sistemas hospitalares (A, B, C) | ✗ o desfecho é a morte intra-hospitalar: estado vital registrado, não decisão | ✓ | não (item 3) |
| \`ctu-uhb-ctgdb\` | ✗ 552 partos (a unidade é o parto, não a linha de minuto) | — | ✓ parto operatório é decisão registrada | ✓ | não (item 1) |
| \`cerebral-vasoreg-diabetes\` | ✗ dezenas de sujeitos | — | — | ✓ | não (item 1) |

**Entra o primeiro: \`challenge-2019\`.** A ordem declarada é essa; se o download falhar ou estourar o orçamento
de ~2 h, o item 3 do próximo candidato exigiria a leitura alternativa "desfecho clínico duro registrado" (morte
intra-hospitalar), que é critério novo: entra como declaração nova, com hash próprio, e conta no K.

## Parte 2. O domínio: \`saude_sepse_alerta\`

**Pergunta.** A mesma lei liga a frequência respiratória da primeira janela de 6 h à decisão de sepse registrada
em qualquer UTI e em qualquer sistema hospitalar, de modo que a lei ajustada nas UTIs de um sistema preveja o
outro sistema, que não entrou no ajuste?

**Alegação.** "Uma FR alta na primeira janela diz a mesma coisa sobre a decisão de sepse em qualquer UTI: o
gatilho calibrado num sistema hospitalar vale no outro."

**Linha, unidade e ambiente.**
- Uma **linha por internação** (um arquivo \`.psv\` = uma internação; o nome do arquivo é a unidade). A unidade
  independente é a internação, então linha e unidade coincidem — o que o F2 nota e relata.
- **Janela** = as 6 primeiras horas registradas (\`ICULOS\` 1 a 6). Entra a internação que tem as 6 horas e que
  está **sem alarme** na janela (\`SepsisLabel = 0\` em \`ICULOS\` 1 a 6): é o desenho de alerta precoce, o gatilho
  é medido antes de qualquer decisão registrada.
- **y** = 1 se \`SepsisLabel = 1\` em qualquer hora depois da janela (\`ICULOS\` ≥ 7); 0 se nunca.
- **Ambiente** = sítio, cruzando sistema e tipo de UTI: sistema pela pasta (A = \`training_setA\`, B =
  \`training_setB\`); tipo = \`MICU\` se \`Unit1 = 1\`, \`SICU\` se \`Unit2 = 1\`, e \`outros\` se nenhuma das duas (flag
  ausente ou 0), pelo primeiro valor não ausente da janela. Nível com menos de 200 linhas vira \`outros\` do seu
  sistema (regra mecânica); nível que fica vazio não existe.
- **Cofre** = os níveis do sistema B (Emory), nunca vistos: a lista sai dos níveis existentes de B. Treino = os
  níveis de A (Beth Israel Deaconess). É o teste do manuscrito do desafio: o que se ajusta num sistema não vale
  automaticamente no outro.
- **Chave:** \`fr\` = maior frequência respiratória (\`Resp\`) da janela, em respirações por minuto. É o gatilho
  documentado (qSOFA: FR ≥ 22/min) e é medido pelo monitor, não por decisão de quem pede exame.
- **Candidatos** (todos da janela): \`fr\` (chave), \`temp\` (maior temperatura), \`pas\` (menor SBP), \`pad\` (menor
  DBP), \`spo2\` (menor O2Sat), \`fc\` (maior HR), \`idade\`, \`sexo\` (\`Gender\` como está no arquivo, 0/1),
  \`tempo_hospital_uti\` (\`HospAdmTime\`, horas entre a admissão hospitalar e a da UTI).
- **ICP:** os 9 candidatos (2^9 subconjuntos), sem mais de 10 como o contrato do pacote exige.
- **Ajuste:** \`idade, sexo, temp, pas, spo2, tempo_hospital_uti\`.
- **DAG declarado:** \`idade → fr\`, \`idade → temp\`, \`idade → pas\`, \`idade → spo2\`, \`idade → y\`;
  \`sexo → y\`; \`tempo_hospital_uti → y\`, \`tempo_hospital_uti → fr\`; \`temp → fr\`, \`temp → y\`;
  \`spo2 → fr\`, \`spo2 → y\`; \`pas → y\`, \`pad → pas\`; \`fr → y\`. \`fc\` não recebe aresta para \`y\`: a
  taquicardia é irmã/descendente da mesma fisiologia, não pai da decisão.
- **Família:** binomial.

**Verdade de referência para conferir o Ŝ (declarada agora, antes de rodar).**
- Causas plausíveis da decisão registrada: \`fr\`, \`temp\`, \`pas\`, \`pad\`, \`spo2\` e, mais fraca, \`idade\`
  (o limiar de suspeita muda com a idade). Um Ŝ ⊆ {\`fr\`, \`temp\`, \`pas\`, \`pad\`, \`spo2\`, \`idade\`} é coerente.
- Não-causas: \`fc\` (taquicardia é descendente da febre e da sepse) e \`sexo\`. Um Ŝ que contenha \`fc\` é **erro** do
  ICP; com \`sexo\`, é leitura fraca.

**Priori SHELF da chave.** Quantidade: inclinação da decisão registrada na FR, em log-razão de chances por
respiração por minuto. Fonte: o qSOFA (Seymour et al. 2016, JAMA, DOI 10.1001/jama.2016.0288, conferido no Europe
PMC) adota o limiar de taquipneia ≥ 22/min entre os três critérios e reporta discriminação modesta para desfecho
em UTI (AUROC 0,66 para mortalidade intra-hospitalar na coorte de validação), e o critério é composto de três
itens: ler 6 respirações por minuto como uma razão de chances de ~2 no limiar e diluir por 6 dá ~0,11 por
minuto, atenuado pela severidade já medida; daí a mediana de 1,06 e a cauda até 1,45. Quartis em razão de
chances: q05 0,84, q25 0,96, q50 1,06, q75 1,18, q95 1,45 (dp 0,17 em log). \`taxa_base\` 0,05: a sepse é rara no
conjunto público do desafio; 5% é ordem de grandeza declarada, **sem conferir o dado**. Painel: literatura
publicada, nenhum especialista humano consultado.

**Decisão (F1/F8).** Ação: usar a FR máxima da primeira janela (limiar 22/min) como gatilho do pacote de sepse
(hemoculturas + antibiótico + lactato) em toda UTI. \`beneficio_por_unidade\` = 1,0 (um caso-equivalente por
unidade de efeito marginal na probabilidade); \`custo\` = 0,005 (pacote extra e alarme falso por paciente
rastreado, suposição, não medida); \`custo_pesquisa\` = 0,0005 (análise secundária de registro aberto, sem coleta
nova). Termo de escopo: vale para internações de UTI dos dois sistemas públicos do desafio A e B; não vale para
pronto-socorro, enfermaria, outros limiares ou sistemas sem o mesmo par de exames registrados.

**Verdito declarado.** O domínio **confirma** a alegação se o funil chegar ao F4, o Ŝ do ICP contiver \`fr\`, a
habilidade do modelo ajustado no cofre for positiva e o portão M15 aprovar. **Falha** se parar no F1–F3, se o Ŝ
não contiver \`fr\` ou se a habilidade no cofre for ≤ 0. Ŝ = ∅ com β > 0 conta como "efeito sem invariância
certificada", não como confirmação.

**Previsão do autor.** O rótulo é uma decisão registrada e o gatilho é fraco: espero Ŝ pequeno ou vazio, e
calibração no cofre pior que no treino — o próprio manuscrito do desafio mostra que a transferência entre
sistemas é o ponto fraco. Isso é o que o domínio foi montado para medir, não para evitar.

**Violações declaradas antes de rodar.** (i) A FR e o rótulo compartilham a decisão: quem registra hemocultura e
antibiótico olha a FR, então há confusão por indicação, não um efeito causal limpo. (ii) O rótulo também depende
de SOFA (PA, plaquetas, bilirrubina, creatinina, Glasgow), que não está todo medido aqui — pai oculto. (iii) O
tipo de UTI não é sorteado: a mistura de pacientes muda com ele. (iv) Faltantes de vitais na janela tiram a
internação da amostra, o que pode enviesar para o lado de quem tem monitoração mais completa.

## Parte 3. O funil, sem mudança

Roda o do p09 (\`rodar.py\` + \`dominios/saude_sepse_alerta/\`) com \`OMP_NUM_THREADS=1\` e no máximo 2 processos
em paralelo: F1 (EVPI da decisão contra o custo da pesquisa, sem tocar nos dados), F2 (porta dos fundos no DAG e
informatividade na unidade), F3 (nulo de permutação no nível em que a chave foi atribuída — aqui, a chave é
constante na unidade, então a troca é entre unidades), F4 (ICP entre os ambientes de treino, MCMC + SBC,
pré-registro antes de abrir o cofre, cofre do sistema B, portão M15) e a decisão do F8. Sementes: 2027 + 100·i,
como no p09. Nada é ajustado depois de ver o resultado; o que mudar entra como declaração nova com hash e conta
no K.

**Portão de calibração.** Como o cofre é binário, entra também o portão do \`fundo/p08-calibracao\`: ECE do diário
no cofre medido contra a tolerância que depende de n (a que o p08 declara), nas caixas de probabilidade de mesma
largura. O resultado entra no README como resultado, passando ou não.

## Parte 4. Se o primeiro candidato falhar

Se o download do \`challenge-2019\` falhar ou não terminar dentro do orçamento de ~2 h, o próximo da ordem é o
\`challenge-2012\`, e ele só entra sob a leitura alternativa do item 3 (desfecho clínico duro registrado: a morte
intra-hospitalar), que **não** é a leitura declarada acima. Nesse caso escrevo \`DECLARACAO_2.md\` com o hash
próprio, o K sobe para 2, e o README diz qual entrou e por quê.
