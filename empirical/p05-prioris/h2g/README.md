# H2g: third solid area — cardiac pharmacology (FDA trial, ECGRDVQ)

**G3 passes without depending on genetics.** ICP gives Ŝ ≠ ∅ in 4 areas: Education, Physics, Genetics and Pharmacology.
Without the yeast y08, 3 areas remain, which is what the PLAN demands.

The Ŝ of pharmacology is **{c_dof, c_qui, c_ran}**: the concentrations of the three drugs that block hERG. It is the result
that the reference truth, declared beforehand, calls coherent, and it does not bring `pr` or `tamp`.

The other gates pass: G1 (27 tests and identical clean reproduction in the 18 domains), G2, G4 and the track
(`saida/portoes.json`).

```
python3 rodar.py                    # funnel in the 18 domains
python3 portoes.py --reproducao     # gates G1–G5, with a second clean run
```

## Order of the steps (pre-registration)

1. **The declaration came first.** `DECLARACAO_H2g.md` was written before any run, with the hash
   `a5d879b3…`, at 2026-10-15T03:08:50Z.
2. **The package came after the hash and implements the declaration.** They are `dominios/farmacologia_qt/` and
   `tests/test_h2g.py`.
3. **Tests.** There are 27 and they passed on the first try.
4. **One run** of the funnel and of the gates, with nothing adjusted afterwards.

## farmacologia_qt

- **Funnel.**
  - F1: EVPI 4.7, but the decision is invented.
  - F2: SE per volunteer of 1.4 ms per ng/mL, against a prior sd of 11.9.
  - F3: p = 0.005, the floor.
  - M15 approves.
- **ICP.** It accepts 35 sets out of 512, and none is saturated.
  - All 35 contain the three concentrations.
  - The intersection is {c_dof, c_qui, c_ran}.
  - `pr`, `tamp`, `rr`, sex, age and weight appear in some accepted sets and not in others, so they
    stay out of Ŝ.
- **Posterior of the key.** The QTcF slope on dofetilide is 25.2 ms per ng/mL, with 90% CI from 23.0 to 27.5. It is
  inside the prior quantiles, which were taken from memory with median 28.
- **Out of environment, on verapamil, never seen.**
  - The skill is +0.57 with the invariant model and +0.65 with the adjusted one.
  - The conformal coverage is 0.89.
  - The model with **all** candidates explodes: the loss is of the order of 10¹⁵. Verapamil lengthens PR outside
    the training range, and the spline extrapolates. It is the textbook example of what ICP protects: PR predicts well inside
    training and fails under a new intervention.

## Reading

- **By the written criterion,** G3 passes with 4 areas.
- **Without genetics,** which sil approved with a caveat, 3 areas remain: Education (STAR), Physics (light tunnel, which
  recovers the authors' graph truth) and Pharmacology (the FDA trial, which recovers the drugs that block the
  hERG). Two of the three have external reference truth and coincide with it.
- **Caveat that remains.** The test assumes independent rows, and here there are 16 instants per volunteer. That
  would tend to reject more, and even so the sets with the drugs were accepted.
