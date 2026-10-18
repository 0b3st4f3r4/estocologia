# Declaration of point 5, second: the equivalence band of round (a)

Date: 2026-10-15 (UTC stamp in `DECLARACAO_2.sha256`). Written **after** [DECLARACAO.md](DECLARACAO.md) had been
fulfilled and after `comparar.py` had failed the gate of round (a) for numerical difference, and **before** any
recomputation with the new band. `DECLARACAO.md` (hash `167fbd6a…`) and `DECLARACAO.sha256` **were not touched**:
the old rule holds as written.

## What motivated it

`DECLARACAO.md` wrote the failure condition of the control round **without tolerance** — "the copy does not reproduce the
original round (status or posterior different from the original `saida/` in (a))". `comparar.py` implemented
"posterior different" as an absolute difference `< 1e-9` in the posterior mean (and in the posterior sd / prior sd ratio, in
real H1). The result observed: **0 of the 33 domains** differ in status category, in list of failing gates
or in F8 action, and the posterior mean differs in **24 of the 24** domains that have one, from 4.1e-9 to 2.2e-3 of relative
difference — at most **0.0071 sd** of the posterior of round (a) itself. Plus `h1-real/c`, whose sd ratio differs by
1.7e-9. That is: 25 of the 33 fail by a criterion of bit equality that no floating-point reimplementation
satisfies.

This declaration **does not rewrite** that one. It fixes, for the reading of the same check, a declared equivalence band.

## The two facts that fix the band (already measured, not measured now)

| fact | value | where it is |
|---|---|---|
| worst environmental deviation observed (copy against the repository `saida/`) | **0.0071 sd** (worst case `h2g/genetica_y07`) | README of this front, table of the gate of round (a) |
| smallest shift **that changed a decision** in (b) or (c) | **0.18 sd** (`h2/desenvolvimento_preston` in (c)) | README of this front, table of the two changes of (c) |

The first says how much environmental noise this check produces. The second is the scale at which a difference starts to
matter for the conclusion: below it, swapping the prior moves nothing in any of the 33 domains. The two facts are
**25.4 times** apart (0.18 / 0.0071).

## The declared band

**Equivalence band: 0.05 sd of the posterior sd of round (a)** (`0,05 × dp_posterior_a`).

The number sits **between** the two facts, and that is why it is a choice and not a convenience:

- **7.0 times above** the worst environmental deviation ever observed (0.05 / 0.0071 = 7.04) — with slack for the noise that the
  band exists to absorb;
- **3.6 times below** the smallest shift that has already changed a decision (0.18 / 0.05 = 3.6) — no difference of this
  order changes gate, status or action in any observed domain.

## Reading of the check with the band

1. **Categorical, without a band:** status category, list of failing gates and F8 action are still required
   **identical**, in the 33 domains. No band forgives a status change.
2. **Numerical, with the band:** the deviation compared is
   `|m_repositório − m_rodada_a| / dp_posterior_a`, domain by domain. In the three domains of **real H1** the shape of the
   result does not carry the posterior mean, so the quantity compared is the posterior sd / prior sd ratio — which is
   dimensionless and lives in (0, 1] — and the band enters there **absolute, 0.05 on the ratio itself** (the same number, read on
   the scale of the quantity that exists).
3. **Reading criterion, fixed here:**
   - maximum deviation **≤ 0.05 sd** ⇒ round (a) **reproduces**, the gate of round (a) **passes** and the verdict of point 5
     stops being conditional on that reading;
   - maximum deviation **> 0.05 sd** ⇒ the round (a) does not reproduce within the band, the gate remains failed and the verdict
     published keeps the conditional label, as it stands in the README.

## What has already been seen, and what the band does not reopen

Rounds (b) and (c) **had already run and had already been read** before this declaration: N_b = 5 and N_c = 2 over the 33 domains,
the seven changes named in the README. I declare that explicitly and I declare that **the band does not reopen (b) or (c)**:

- the band only decides whether round **(a)** reproduces; it enters no comparison between rounds;
- no criterion of (b) or (c), no seed, no grouping, no R1–R3 rule of the priors and no
  decision (costs, benefits, `taxa_base`) is changed by this declaration;
- no number of (b) or (c) is recomputed, reinterpreted or renumbered because of it; the five and the two changes
  and the G3 with 3 areas in (b) stay as they are in the README.

## K

**The K of this front goes from 1 to 2.** K counts reading rules declared for the same result: the one of
`DECLARACAO.md` (without tolerance, which fails) and this one (with a band of 0.05 sd). The old rule is neither erased nor
replaced; this is an **additional** reading, declared with its own hash and counted in K.

## What counts as a failure of this declaration

- The band being applied and the maximum deviation staying above 0.05 sd: in that case it rescues nothing and the conditional label
  remains. That is a result and it goes to the README as such.
- The categorical check ceasing to match in any domain: then nothing here applies.
