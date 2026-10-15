# H2g: farmacologia cardíaca (ECG × concentração plasmática), ambientes = fármaco sorteado

Baixado em **2026-10-15** para o ICP (Peters, Bühlmann & Meinshausen 2016; citada de memória, conferir).
Área nova: **farmacologia clínica / eletrofisiologia cardíaca**. Não é educação, física/engenharia, genética, sinalização celular, direito, transporte nem seguro-saúde.

**Cegueira:** só li os cabeçalhos, as contagens de linhas por braço/tempo, as unidades e as contagens de valores faltantes das colunas de ECG. **Não vi nenhum valor de ECG** (QT, RR, PR etc.), nenhuma média e nenhum ajuste. Também não vi valores de concentração (só contei quantos faltam).
Arquivos não confiáveis: inspeção leve só com `python3 -I`, fora desta pasta.
Integridade: `SHA256SUMS` (caminhos relativos a esta pasta). Os hashes dos CSVs batem com o `SHA256SUMS.txt` oficial do PhysioNet.

Origem: o site physionet.org está bloqueado pelo proxy, mas o espelho oficial do PhysioNet no AWS Open Data funciona:
`https://physionet-open.s3.amazonaws.com/<projeto>/1.0.0/<arquivo>`.

---

## 1. `ecgrdvq/`: conjunto PRINCIPAL

**FDA "ECG Effects of Ranolazine, Dofetilide, Verapamil, and Quinidine in Healthy Subjects" (ECGRDVQ, v1.0.0)**

- URL: `https://physionet-open.s3.amazonaws.com/ecgrdvq/1.0.0/SCR-002.Clinical.Data.csv` (+ `SCR-002.Clinical.Data.Description.txt`, o dicionário de colunas). Página: https://physionet.org/content/ecgrdvq/1.0.0/
- Licença: **Open Data Commons Attribution License v1.0 (ODC-By)**. Citar PhysioNet (Goldberger et al. 2000) e o artigo original.
- Referências: Johannesen L, Vicente J, Mason JW, et al. *Differentiating drug-induced multichannel block on the electrocardiogram: randomized study of dofetilide, quinidine, ranolazine, and verapamil.* Clin Pharmacol Ther 2014;96:549–558, doi:10.1038/clpt.2014.155. Medidas de morfologia: Vicente et al., JAHA 2015. (Volume e páginas citados de memória, conferir.)
- **Desenho:** ensaio randomizado, duplo-cego, **cruzado em 5 períodos**, com 22 voluntários saudáveis. Cada sujeito recebeu os 5 tratamentos em dose única, em jejum, com 7 dias de washout. Há 16 tempos nominais (`TPT`): −0,5 (pré-dose), 0,5, 1, 1,5, 2, 2,5, 3, 3,5, 4, 5, 6, 7, 8, 12, 14 e 24 h. Em cada tempo há 3 ECGs (triplicata) e uma coleta de plasma.
- Arquivo: 5 232 linhas (uma por ECG) × 32 colunas.

### Ambientes (intervenções conhecidas, alvo molecular conhecido)
| ambiente (`EXTRT`) | dose | alvo principal (farmacologia estabelecida) | ECGs | tempos (sujeito×TPT) |
|---|---|---|---|---|
| Placebo | — | nenhum | 1 056 | 352 |
| Dofetilide | 500 µg | bloqueio **seletivo de hERG (IKr)** | 1 056 | 352 |
| Quinidine Sulph | 400 mg | hERG **+ Na⁺ (pico e tardio)** + outros; vagolítico | 1 008 (1 sujeito sem este período) | 336 |
| Ranolazine | 1 500 mg | hERG **+ Na⁺ tardio** (equilibrado) | 1 056 | 352 |
| Verapamil HCL | 120 mg | **Ca²⁺ tipo L** + hERG (equilibrado); nó AV | 1 056 | 352 |

Os mesmos 22 sujeitos aparecem em todos os ambientes (desenho cruzado). Por isso a composição da população é idêntica entre ambientes. Esse é um ponto forte para o ICP.

### Como montar X (sugestão; declarar antes de olhar Y)
- `PCSTRESN` é a concentração do fármaco do período, em formato longo (`PCTEST` diz qual fármaco). As unidades diferem: dofetilida em pg/mL, os demais em ng/mL. Ela é **NA no placebo e na linha de base (TPT −0,5)**. Na linha de base a concentração é 0 por construção (pré-dose).
  - Sugestão: criar 4 colunas, `c_dof`, `c_qui`, `c_ran` e `c_ver`, cada uma igual a 0 fora do seu braço e na pré-dose, possivelmente em log1p.
  - A concentração varia **dentro** do ambiente ao longo da curva PK (0 → pico → eliminação). Logo X não é degenerado dentro de cada ambiente.
- Há uma única concentração por tempo, repetida nas 3 ECGs. **Recomendo fazer a média das triplicatas** (n ≈ 352 por ambiente; 1 744 no total) ou usar só a 1ª ECG.
- Outras colunas: `RR` (FC), `PR`, `QRS`, `QT`, `JTPEAK`, `TPEAKTEND`, `TPEAKTPEAKP`, `ERD_30`, `LRD_30`, `Twave_amplitude`, `Twave_asymmetry` e `Twave_flatness`. Covariáveis basais: `SEX`, `AGE`, `HGHT`, `WGHT`, `SYSBP`, `DIABP` (constantes por sujeito).
- Faltantes (só contagem): PR 9, QRS 9, QT 13, JTPEAK 9, TPEAKTEND 13 e Twave_amplitude 7, de 5 232 linhas.

### Y candidato
- **QTc** (ex.: QTcF = QT/(RR/1000)^(1/3)), ou **QT** com `RR` entre os preditores. Contínuo.
- Alternativa mecanicamente mais limpa: **`JTPEAK`** (repolarização precoce), contínuo.
- Atenção: QT = QRS + J-Tpeak + Tpeak-Tend (aproximadamente, por definição). **Não pôr os componentes do QT em X quando Y = QT**, porque seria uma identidade, não uma causa.

### Preditores candidatos (3–10) e verdade de domínio
| X | papel esperado para Y = QTc | base |
|---|---|---|
| `c_dof` | **causa** (bloqueio de hERG → prolonga a repolarização) | farmacologia + desfecho primário de Johannesen 2014 |
| `c_qui` | **causa** (hERG) | idem |
| `c_ran` | **causa fraca** (prolongamento modesto) | idem; efeito pequeno, então pode não ser detectado |
| `c_ver` | **não-causa (ou efeito ~nulo)**: bloqueio de hERG compensado pelo bloqueio de Ca²⁺ | idem; é um "falso candidato" com intervenção forte |
| `RR` | causa fisiológica do QT bruto (relação QT–FC); depois de corrigir (QTcF), deveria sobrar pouco | fisiologia. Os fármacos também mudam a FC (quinidina é vagolítica), então RR é um X intervindo indiretamente |
| `PR` | **não-causa / desfecho paralelo** (condução AV; verapamil prolonga PR) | farmacologia do verapamil |
| `Twave_amplitude`, `Twave_flatness`, `Twave_asymmetry` | **efeitos a jusante / proxies** da mesma repolarização (achatamento da onda T com bloqueio de hERG) | Vicente et al. 2015. Bons exemplos de "filho de Y ou irmão" que o ICP deve rejeitar |
| `AGE`, `SEX`, `WGHT` | constantes por sujeito; sexo é determinante conhecido do QTc (QTc maior em mulheres) | invariantes entre ambientes (mesmos sujeitos) |

Conjunto-verdade plausível para S* (Y = QTcF): {`c_dof`, `c_qui`, (`c_ran`)} ± `SEX`. **Não** deveriam entrar: `c_ver`, `PR` e a morfologia da onda T. Os nomes dos efeitos vêm da literatura, citada de memória; **conferir no artigo** antes de fixar a verdade.

### O que age diretamente em Y, e se está medido
- O fármaco age no QT via bloqueio de canais iônicos. Esse bloqueio não é medido, mas é função (aproximadamente monótona) da **concentração plasmática, que é medida**. Na prática, ambiente → concentração (medida) → QT.
- **Ressalvas:**
  1. *Histerese*: atraso entre plasma e efeito, de modo que o efeito não é função instantânea da concentração.
  2. *Metabólitos ativos não medidos*: 3-hidroxiquinidina, norverapamil e metabólitos da ranolazina. Isso é um canal ambiente → Y não medido, pequeno.
  3. O *ritmo circadiano e as refeições* afetam QT/FC, mas o horário (`TPT`) é igual em todos os ambientes. Incluir `TPT` (ou o placebo pareado por tempo) se necessário.
  4. A quinidina muda o RR, que está medido.

### Ressalvas gerais
- As linhas **não são i.i.d.**: são 22 sujeitos × 16 tempos (séries temporais curtas). Há correlação intra-sujeito, que pode inflar os testes de invariância. Considerar centrar por sujeito (placebo como referência), fazer subamostragem ou usar teste por blocos.
- n efetivo pequeno em sujeitos (22). Cerca de 350 tempos por ambiente.
- Cada fármaco foi dado em uma única dose, então a variação de concentração vem só do tempo e da variabilidade PK entre sujeitos.

---

## 2. `ecgcipa/`: conjunto SECUNDÁRIO (replicação, mesma área)

**FDA "CiPA Phase I ECG Biomarker Validation Study" (ECGCIPA, v1.0.0; NCT03070470)**

- URLs: `https://physionet-open.s3.amazonaws.com/ecgcipa/1.0.0/{adsl,adpc,adeg,addm}.csv` e `ecgcipa_metadata.docx` (dicionário de variáveis CDISC ADaM). Página: https://physionet.org/content/ecgcipa/1.0.0/
- Licença: **ODC-By v1.0**.
- Referência: Vicente J, Zusterzeel R, Johannesen L, et al. *Assessment of multi-ion channel block in a phase I randomized study design: results of the CiPA Phase I ECG Biomarker Validation Study.* Clin Pharmacol Ther 2019, doi:10.1002/cpt.1303.
- **Desenho:**
  - Parte 1: **paralelo**, com 50 sujeitos (10 por braço): placebo, ranolazina, verapamil, lopinavir+ritonavir e cloroquina, por 3 dias de doses (exposição baixa → alta).
  - Parte 2: **cruzado**, com 10 sujeitos: dofetilida e diltiazem+dofetilida.
  - As doses não estão nos arquivos; ver o protocolo.
- Arquivos:
  - `adsl` (60 sujeitos; braço, idade, sexo, raça).
  - `addm` (altura, peso, IMC, PA).
  - `adpc` (1 934 concentrações, 25 tempos de 0,5 a 73 h; analitos: ranolazina, verapamil, lopinavir, ritonavir, cloroquina, dofetilida e diltiazem).
  - `adeg` (69 556 linhas em formato longo: `PARAMCD` ∈ HR, RR, PR, QRS, QT, QTCF, JTP, JTPC, TPTE; `EGLEAD` VCGMAG/GLOBAL; linhas `DTYPE=AVERAGE` são médias de réplicas).
- Ambientes (`TRTA`), com tempos PK: Ranolazine 250, Chloroquine 250, Dofetilide 250, Verapamil 230, Lopinavir+Ritonavir 249 (×2 analitos), Diltiazem+Dofetilide 228 (×2 analitos), Placebo (sem PK; cerca de 1 124 linhas de ECG por parâmetro).
- Y/X: análogos ao ECGRDVQ. Y = `QTCF` ou `JTPC`. X = concentrações por fármaco + HR/RR + PR + QRS. A junção ECG↔PK é por `USUBJID` + `APERIOD` + `ATPT`.
- Verdade de domínio (hipótese do estudo): fármacos de "hERG predominante" (dofetilida, cloroquina) prolongam o QTc. Fármacos "equilibrados" (ranolazina, verapamil, lopinavir/ritonavir) prolongam pouco ou nada o J-Tpeakc. O diltiazem (bloqueio de Ca²⁺) atenua o efeito da dofetilida no J-Tpeakc. Esse último ponto é **interessante para o ICP**: é um caso de interação.
- **Ressalvas extras:** na Parte 1, **sujeito e ambiente são confundidos** (10 sujeitos diferentes por braço), então a população muda entre ambientes. As doses sobem em 3 dias (há carryover dentro do período). As linhas também são repetidas por sujeito. Use-o como validação externa, não como principal.

---

## Tentativas que falharam / descartadas
- **International Stroke Trial (IST):** a fonte oficial (datashare.ed.ac.uk) está bloqueada, e não achei cópia em GitHub. Além disso, Y seria binário e o tratamento tem caminhos diretos não medidos para Y.
- **PhysioNet ECGDMMLD** (dofetilida + mexiletina/lidocaína, moxifloxacino + diltiazem): não está no espelho S3 `physionet-open`, e physionet.org está bloqueado.
- physionet.org, OSF, Hugging Face e datashare: CONNECT 403 no proxy.
- Rdatasets (agridat `lasrosas.corn`, lme4 `Arabidopsis`, geepack `dietox`, medicaldata `opt`, mediation `framing`/`jobs`): têm poucos ambientes, poucas linhas, nenhum X mediador medido, ou caem em área já usada (JOBS II ≈ treinamento de mão de obra). `causaldata thornton_hiv` já está em `h2b/`.
