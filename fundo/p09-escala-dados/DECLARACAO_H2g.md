# Declaração H2g: área mais sólida — farmacologia cardíaca (ensaio do FDA, ECGRDVQ)

Escrita em 2026-10-15, **antes** de qualquer execução do código do H2g. Destes dados só se viram o esquema, o
dicionário de colunas, as contagens por braço e por instante, as unidades e quantos valores faltam. Não se viu
nenhum valor de ECG nem de concentração, nem resumo, nem ajuste. O SHA-256 e a hora UTC ficam em
`DECLARACAO_H2g.sha256`.

O trabalho fica em `h2g/`, uma cópia de `h2f/` com os 17 domínios sem mudança. As pastas anteriores não são tocadas.

## O pedido

sil aprovou o H2 com a ressalva de que a genética é fraca, e pediu para "incluir a busca de um terceiro para
melhorar a sustentação do H2". O objetivo aqui é ter uma terceira área com evidência que não dependa da genética.
O G3 continua o do PLAN, sem mudança.

## Os dados

- **O ensaio.** ECGRDVQ, do FDA (Johannesen et al. 2014, Clin Pharmacol Ther — citado de memória, conferir),
  obtido no espelho oficial da PhysioNet no AWS (ODC-By; ver `../dados-reais/h2g/FONTES.md`).
- **Desenho.** Cruzado, randomizado e duplo-cego, com 22 voluntários saudáveis. Cada um recebeu em períodos
  separados placebo, dofetilida 500 µg, quinidina 400 mg, ranolazina 1 500 mg e verapamil 120 mg.
- **Medidas.** São 16 instantes, de −0,5 a 24 h. Em cada um há 3 ECGs e uma dosagem plasmática da droga do
  período.
- **Área: "Farmacologia clínica (cardiologia)".** É distinta de Educação, Física e Genética, e também da Saúde do
  RAND, que trata de seguro e uso de serviços.

## Desenho do domínio `farmacologia_qt`

- **Linha.** Uma linha por voluntário × braço × instante, com a média do triplo de ECGs. Ficam cerca de 350 por
  braço.
  - As linhas pós-dose sem dosagem saem.
  - Na linha pré-dose (−0,5 h), a concentração vale 0.
  - Saem as linhas em que falte Y ou algum candidato.
- **Y = QTcF** = QT / (RR/1000)^(1/3), em ms.
- **Ambientes:** o braço sorteado. O placebo, a dofetilida, a quinidina e a ranolazina ficam no treino.
- **Cofre: o verapamil.** É uma intervenção forte que, pela farmacologia, quase não muda o QTc, mas alonga o PR. É um
  bom teste fora do ambiente: um modelo que use o PR como proxy erra ali.
- **Candidatos do ICP (9).**
  - `c_dof`, `c_qui`, `c_ran`: a concentração plasmática de cada droga, em ng/mL. A dofetilida vem em pg/mL e é
    dividida por 1 000. Cada uma vale 0 fora do seu braço.
  - `rr`: o RR médio do triplo, em ms.
  - `pr`: o PR médio, em ms.
  - `tamp`: a amplitude da onda T, em µV.
  - `sexo` (1 = F), `idade` e `peso`.
  - `c_ver` não entra, porque é constante (0) em todo o treino.
- **Chave:** `c_dof`. **Ajuste:** c_qui, c_ran, rr, sexo, idade, peso.
- **DAG declarado.**
  - Cada concentração → y.
  - c_qui → rr.
  - Cada concentração → pr e → tamp.
  - rr → y.
  - sexo, idade e peso → y.
  - y → tamp: a onda T é parte da repolarização e é tratada como descendente.
- **Unidade:** o voluntário (22), para o EP do F2 e a permutação do F3.
- **α do ICP:** 0,05, o padrão, porque há um só Y.
- **O teste.** O teste de invariância e a regra de saturação são os do H2d, sem mudança.
- **Priori SHELF da chave.** É a inclinação de QTcF na concentração de dofetilida, em ms por ng/mL.
  - Com 500 µg, o pico de concentração fica perto de 2–3 ng/mL e o ΔQTc no pico perto de 70–80 ms, o que dá
    cerca de 25–35 ms por ng/mL. Isto é citado de memória e não foi conferido.
  - Quartis: q05 8, q25 20, q50 28, q75 36, q95 50.
  - Nenhum especialista humano foi consultado.
- **Decisão.** É inventada, com custo 28, perto da mediana; o F1 não informa nada sobre o mundo.

## Verdade de referência para conferir Ŝ (farmacologia, declarada antes)

- **Causas do QTcF:** c_dof, c_qui, c_ran (o efeito da ranolazina é pequeno), sexo e, possivelmente, a idade e o RR
  residual, porque a correção de Fridericia não é perfeita.
- **Não são causas:**
  - `pr`, que é um desfecho paralelo da condução atrioventricular;
  - `tamp`, que é descendente da repolarização;
  - o peso, sem papel direto conhecido.
- **Leitura de validação, informativa e decidida agora:**
  - um Ŝ que contenha `pr` ou `tamp` é um **erro** do ICP;
  - um Ŝ ⊆ {c_dof, c_qui, c_ran, sexo, idade, rr} é **coerente**.

## Violações conhecidas

São declaradas antes de rodar.
- **Linhas dependentes.** As linhas não são i.i.d.: são 16 instantes por voluntário. Os mesmos 22 voluntários
  aparecem em todos os braços, então o efeito de cada pessoa se equilibra entre os ambientes. Mesmo assim, o teste
  supõe independência e tende a rejeitar mais.
- **Histerese.** Há atraso entre a concentração e o efeito no QT, e ele difere entre as drogas.
- **Metabólitos ativos não medidos:** a 3-hidroxiquinidina e o norverapamil.

Todas essas violações tendem a **rejeitar mais conjuntos**, não a aceitar um falso.

## Regra do G3

- O G3 é calculado como está em `portoes.py`, sobre os 18 domínios.
- Relato também a leitura **sem a genética**. Se a farmacologia der Ŝ ≠ ∅, o H2 passa a ter 3 áreas sem depender do
  y08 da levedura. Essa é a "sustentação" pedida.
- Se der Ŝ = ∅, a aprovação com ressalva de sil continua como está, e nada é ajustado.

## Depois desta declaração

- Testes: os do H2f, mais a carga do pacote novo.
- Uma execução, sem nada ajustado depois.
