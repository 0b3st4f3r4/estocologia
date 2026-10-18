# Declaration of point 12 (SHELF): freezing of the protocol and adjustment of the pre-test

**Frozen on 2026-10-16T14:55:54Z**, before any expert answered anything.

| file | SHA-256 | what it is |
| --- | --- | --- |
| `protocolo.md` | `175004ac45caa0fc8e231ea0bd85466b913ef7da104f3d0c4d394cb7e6e95058` | the protocol, frozen: from here on it is not rewritten |
| `DECLARACAO.md` (this one) | `5b4d32a17ca84e4cd6e6c5233f143aa40509d890945c5410ec06f5ffe694690a` | this declaration, with the adjustment and the record of the freezing |
| `DECLARACAO.sha256` | — | the two hashes above, in `sha256sum` form |

## Declaration of adjustment 1 — the gate of the pre-test

The protocol fixes, in section 2, four approval criteria and the ECE threshold of 0.10 in 10 bins of equal
mass. Before freezing, the design was measured with the instrument itself:

~~~bash
cargo run --release --bin labs -- --preteste
~~~

| design | ECE p50 | p90 | p95 | fraction that passes the threshold of 0.10 |
| --- | --- | --- | --- | --- |
| 20 items, 10 bins (that of the protocol) | 0.2272 | 0.3002 | 0.3218 | 0.0008 |
| 20 items, 5 bins | 0.1584 | 0.2316 | 0.2559 | 0.1072 |
| 20 items, 20 bins | 0.3325 | 0.4017 | 0.4222 | 0.0000 |
| 200 items, 10 bins | 0.0694 | 0.0932 | 0.1002 | 0.9486 |

The null is the **perfectly calibrated** expert: the prior is uniform and the outcome is drawn from it, in the same
design. The reading:

1. **The threshold of 0.10 is not a gate, it is a wall.** In the design of the protocol it approves 0.08% of the calibrated ones — 1 in
   1 250. The cause is the sample size: the ECE falls with the square root of n, and with 200 items the same threshold approves 95%.
2. **Criterion 3 becomes vacuous** in the design of the protocol: 20 items in 10 bins of equal mass give 2 items per
   bin, and the rule only bites from n ≥ 4.
3. **Criterion 4 is inert as written.** It compares the predicted mean of the top bin with that of the bottom, and the
   bins are cut by ordered p: the top one exceeds the bottom one always, with the expert calibrated or
   inverted. The test `o_criterio_4_do_protocolo_nao_sabe_reprovar` proves that.

**Adjusted gate, declared before running:**

- the pre-test comes to be read in **5 bins of equal mass, 4 items each**, with **ECE ≤ 0.25**, which is the p90 of the
  calibrated null in that design (0.2316) rounded up;
- **criterion 3 bites again** in that design: n = 4 per bin, exactly at the limit;
- **criterion 4 stays as written** — inert, and declared so — and the report of the pre-test carries alongside the
  diagnostic that bites: the **observed rate** of the top bin above that of the bottom;
- criteria 1 (skill > 0) and the Brier against the climatology of the pool itself continue as the protocol fixed them.

This is a new declaration, as the house rule requires: it counts in the K of the front, which goes from 2 (protocol = 1,
comparison = 2) to **K = 3**.

## List of items, per domain

The protocol requires a seed and a list frozen before running. The list is **per domain**, and each domain has its
declaration of items, with its own hash, before the pre-test of that domain:

| domain | file of the list | hash | state |
| --- | --- | --- | --- |
| `h2/desenvolvimento_preston` | `preteste/desenvolvimento_preston.yaml` | *(enters the declaration of items of the domain)* | in preparation |

## What this declaration does not do

- **It does not substitute the panel.** The protocol asks for three experts per domain, with one from outside the program. One
  approved expert closes no panel at all (section 2.4), and the consensus of n = 1 **does not** substitute the prior of the
  package in the official round: it stays as sensitivity.
- **It does not fill in anybody's form.** What is not written with name and date did not happen.
---

## Declaration 2 — rectification of the hash of the pool of the pre-test

**2026-10-16.** The file of the pool was rewritten by the executor **after** the declaration of items was frozen:
it added the detail of the discarded items, and the file went from 200 to 204 lines. The hash that the first
version of the declaration of items recorded ceased to hold.

| version | hash | state |
| --- | --- | --- |
| pool before the rectification | `c287a013d0865be891c6368da5f4f68f364412396ed11b6d9f77a3463598cb1e` | superseded |
| pool after the rectification | `e4bd218d16fc50064c2f10e55221af8b3321e8926d3857f5e239edc5a69cffbb` | current |
| `DECLARACAO.md` in version 1 | `fbe4fac2d768649d11b2274c5c4ca8c82cbe7f445491b85ac8abd2baa8f50031` | superseded by this rectification; the current hash is in `DECLARACAO.sha256` |

**What did not change, checked and not assumed.** The file still has 22 items, in the same order of identifiers
(`P01 … P22`); the declared draw was **reproduced against the new file** and returns exactly the frozen list
(`P05 P01 P06 P18 P10 P07 P22 P09 P17 P11 P21 P02 P14 P13 P08 P20 P19 P04 P12 P15`), because the seed
shuffles the list of identifiers and the list did not change order. The resolutions of the 20 drawn were
rechecked against the cited sources.

**K:** this rectification is not a new specification — no criterion, threshold, or item changed. It fixes the record
of the hash, and does not count K. The front continues with **K = 3**.


---

## Declaration 3 — criterion 3 of the pre-test, corrected by the right test

**2026-10-16.** Before accepting the rejection of expert E1 in the pre-test of \`h2/educacao_star\`, the gate was measured on the calibrated null (20 000 repetitions of the design of 20 items in 5 bins, `labs --preteste`):

| ruler | rejection of **perfectly calibrated** experts |
| --- | --- |
| criterion 3 as written (n ≥ 4 and abs(deviation) > 0.25) | **0.7052** |
| criterion 2 (ECE ≤ 0.25) | 0.0592 |
| criterion 3 corrected (binomial test, Bonferroni) | **0.0031** |
| total approval of the gate as written | 0.2935 |

**The reading.** Criterion 3, in the letter of the protocol, rejected seven calibrated ones out of ten: in a bin of four items, the deviation of 0.25 is noise of a small sample, not a systematic section. The threshold of 0.25 was a choice of design of this front (declaration 1, on opting for 5 bins to make the criterion bite), and the measurement showed that it bites the wrong side.

**What comes to hold, declared before use:**

- criterion 3 is judged by the **two-sided binomial test** of the number of hits of each bin, given the **mean that the expert himself predicted** for it, with **Bonferroni** by the number of bins and alpha 0.05 (`caixas_por_teste_binomial`, in `crates/stochology-horizons/src/h3/preteste.rs`);
- the fixed threshold of 0.25 ceases to gate and stays **reported alongside**, as a reading of the protocol as written;
- criteria 1 (skill > 0), 2 (ECE ≤ 0.25 in the 5 bins), and the diagnostic of the observed curve continue as they were;
- the verdict comes to be issued by `labs --preteste`, which prints the **two rulers** side by side.

This is a new declaration, as the house rule requires, and the K of the front goes from **4** to **5**.

**Effect on the pre-tests already done**, recalculated with the ruler in force: `desenvolvimento_preston` **approved** (the two rulers agree), `direito_prisoes` **approved** (the two agree), and `educacao_star` **approved** — the latter was rejected by the fixed threshold (bin 3: four items with prediction 0.7250, all `sim`, deviation 0.2750) and passes by the binomial test, with two-sided p 0.55 and corrected p 1.00. The rejection of that domain was from the ruler, and stays recorded as such.


---

## Execution note — the order of the domains and the exclusion of the pair of the same base

**2026-10-16.** Decision of the operator. Expert E1 served `h2/educacao_star`, and section 3 of the protocol forbids the same expert from serving both sides of a pair of the same base: `h2/educacao_star` and `h2g/educacao_star_turmas` are born from the same base `AER_STAR`. Asked, the operator chose **not** to open an exception.

Consequences, declared:

- `h2g/educacao_star_turmas` leaves the queue of E1 and stays **reserved** for a second expert — and for that reason the panel of this domain cannot be closed by E1, neither now nor later;
- the fourth elicited domain becomes `h2g/direito_teto_beneficio`, whose pair of the same base declared in p05 is `h1-real/b` (NSW/LaLonde), which E1 did not serve as a SHELF expert;
- the live queue of E1 becomes: `h2/desenvolvimento_preston` (done), `h2/direito_prisoes` (done), `h2/educacao_star` (done), `h2g/direito_teto_beneficio` (in progress), and `h1-real/c` (pending).

This is not a change of criterion or of threshold: it is the design of the panel following the protocol, and it does not count K. The front continues with **K = 5**.
