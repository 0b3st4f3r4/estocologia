# Declaration H2g: most solid area — cardiac pharmacology (FDA trial, ECGRDVQ)

Written on 2026-10-15, **before** any run of the H2g code. Of these data only the schema was seen, the
column dictionary, the counts by arm and by instant, the units and how many values are missing. No ECG or
concentration value was seen, nor summary, nor fit. The SHA-256 and the UTC time stay in
`DECLARACAO_H2g.sha256`.

The work stays in `h2g/`, a copy of `h2f/` with the 17 domains unchanged. The earlier folders are not touched.

## The request

sil approved H2 with the caveat that genetics is weak, and asked to "include the search for a third one to
improve the support of H2". The goal here is to have a third area with evidence that does not depend on genetics.
G3 remains the PLAN's, without change.

## The data

- **The trial.** ECGRDVQ, from the FDA (Johannesen et al. 2014, Clin Pharmacol Ther — quoted from memory, verify),
  obtained from the official PhysioNet mirror on AWS (ODC-By; see `../data/h2g/FONTES.md`).
- **Design.** Crossover, randomized and double-blind, with 22 healthy volunteers. Each received in separate
  periods placebo, dofetilide 500 µg, quinidine 400 mg, ranolazine 1,500 mg and verapamil 120 mg.
- **Measurements.** There are 16 instants, from −0.5 to 24 h. At each one there are 3 ECGs and a plasma assay of the drug of the
  period.
- **Area: "Clinical pharmacology (cardiology)".** It is distinct from Education, Physics and Genetics, and also from RAND
  Health, which deals with insurance and service use.

## Design of the `farmacologia_qt` domain

- **Row.** One row per volunteer × arm × instant, with the mean of the triple of ECGs. There are about 350 per
  arm.
  - Post-dose rows without assay are dropped.
  - In the pre-dose row (−0.5 h), the concentration is worth 0.
  - Rows missing Y or some candidate are dropped.
- **Y = QTcF** = QT / (RR/1000)^(1/3), in ms.
- **Environments:** the randomized arm. Placebo, dofetilide, quinidine and ranolazine stay in training.
- **Vault: verapamil.** It is a strong intervention that, by pharmacology, barely changes QTc, but lengthens PR. It is a
  good out-of-environment test: a model that uses PR as a proxy errs there.
- **ICP candidates (9).**
  - `c_dof`, `c_qui`, `c_ran`: the plasma concentration of each drug, in ng/mL. Dofetilide comes in pg/mL and is
    divided by 1,000. Each is worth 0 outside its arm.
  - `rr`: the mean RR of the triple, in ms.
  - `pr`: the mean PR, in ms.
  - `tamp`: the amplitude of the T wave, in µV.
  - `sexo` (1 = F), `idade` and `peso`.
  - `c_ver` does not enter, because it is constant (0) in all of training.
- **Key:** `c_dof`. **Adjustment:** c_qui, c_ran, rr, sexo, idade, peso.
- **Declared DAG.**
  - Each concentration → y.
  - c_qui → rr.
  - Each concentration → pr and → tamp.
  - rr → y.
  - sexo, idade and peso → y.
  - y → tamp: the T wave is part of repolarization and is treated as a descendant.
- **Unit:** the volunteer (22), for the F2 SE and the F3 permutation.
- **ICP α:** 0.05, the default, because there is a single Y.
- **The test.** The invariance test and the saturation rule are those of H2d, without change.
- **SHELF prior of the key.** It is the QTcF slope on dofetilide concentration, in ms per ng/mL.
  - With 500 µg, the peak concentration is near 2–3 ng/mL and the ΔQTc at the peak near 70–80 ms, which gives
    about 25–35 ms per ng/mL. This is quoted from memory and was not verified.
  - Quantiles: q05 8, q25 20, q50 28, q75 36, q95 50.
  - No human specialist was consulted.
- **Decision.** It is invented, with cost 28, near the median; F1 informs nothing about the world.

## Reference truth to check Ŝ (pharmacology, declared before)

- **Causes of QTcF:** c_dof, c_qui, c_ran (the ranolazine effect is small), sex and, possibly, age and the residual RR,
  because the Fridericia correction is not perfect.
- **Non-causes:**
  - `pr`, which is a parallel outcome of atrioventricular conduction;
  - `tamp`, which is a descendant of repolarization;
  - weight, with no known direct role.
- **Validation reading, informative and decided now:**
  - a Ŝ containing `pr` or `tamp` is an **error** of ICP;
  - a Ŝ ⊆ {c_dof, c_qui, c_ran, sexo, idade, rr} is **coherent**.

## Known violations

They are declared before running.
- **Dependent rows.** The rows are not i.i.d.: they are 16 instants per volunteer. The same 22 volunteers
  appear in all arms, so each person's effect balances out between the environments. Even so, the test
  assumes independence and tends to reject more.
- **Hysteresis.** There is a lag between concentration and effect on QT, and it differs between drugs.
- **Unmeasured active metabolites:** 3-hydroxyquinidine and norverapamil.

All these violations tend to **reject more sets**, not to accept a false one.

## G3 rule

- G3 is computed as it is in `portoes.py`, over the 18 domains.
- I also report the reading **without genetics**. If pharmacology gives Ŝ ≠ ∅, H2 then has 3 areas without depending on
  the yeast y08. That is the requested "support".
- If it gives Ŝ = ∅, sil's approval with caveat remains as it is, and nothing is adjusted.

## After this declaration

- Tests: those of H2f, plus loading of the new package.
- One run, with nothing adjusted afterwards.
