# 13. The Three Canonical Epistemological Cards

> **Learning Objectives.** By the end of this chapter, you will be able to inspect the three canonical YAML Epistemological Cards in `cards/*.yaml`, understand the operational decision and real dataset behind each card, and see why one card recommends `do_not_protect` while the other two recommend `protect`.

---

## 13.0 Comparative Overview of the Three Canonical Cards

Before reading the full YAML listings below, compare how the three cards span all three ontological layers ($L\_1, L\_2, L\_3$), three different uncertainty regimes, and three real empirical domains:

| Card File | Primary Layer & Uncertainty | Real Dataset (`data/*.csv`) | Cost-Loss Threshold $p^{\ast} = \frac{C}{\eta L}$ | Calibrated Forecast $p$ | Declared Action & Why |
| --- | --- | --- | --- | --- | --- |
| **`card_01_atlantic_storms_evacuation.yaml`** | `L1` / `aleatory` | `data/atlantic_storms.csv` ($n = 20\text{,}778$, $n\_{\mathrm{eff}} = 1\text{,}811.45$) | $\frac{12\text{,}000}{0.80 \times 125\text{,}000} = 0.12000$ | $p = 0.09800$ | **`do_not_protect`** ($p = 0.09800 \le p^{\ast} = 0.12000$): calibrated 85-kt threat is below the evacuation threshold. |
| **`card_02_bank_queue_staffing.yaml`** | `L2` / `epistemic_specification` | `data/bank_calls.csv` ($n = 27\text{,}716$, $n\_{\mathrm{eff}} = 980.70$) | $\frac{1\text{,}600}{0.80 \times 25\text{,}000} = 0.08000$ | $p = 0.09911$ | **`protect`** ($p = 0.09911 > p^{\ast} = 0.08000$): once Jensen's Gap and Moulton-Campbell tail clustering are accounted for, surge risk exceeds $p^{\ast}$. |
| **`card_03_nyc_flights_holding.yaml`** | `L3` / `knightian` | `data/nyc_flights_2013.csv` ($n = 138\text{,}375$, $n\_{\mathrm{eff}} = 1\text{,}364.00$) | $\frac{2\text{,}700}{0.75 \times 60\text{,}000} = 0.06000$ | $p = 0.09800$ | **`protect`** ($p = 0.09800 > p^{\ast} = 0.06000$): under severe cascade loss $L = USD 60\text{,}000$, $p^{\ast} = 6\%$ is crossed and Info-Gap + Wald safeguard holds. |

---

## 13.1 `card_01_atlantic_storms_evacuation.yaml` (Atmospheric Physics & Coastal Evacuation)

This card models 6-hour synoptic rapid-intensification advisories across $n = 20\text{,}778$ NOAA Atlantic tropical cyclone track records (`data/atlantic_storms.csv`). Notice how the card records the exact Moulton-Campbell cluster adjustment across multi-point storm tracks ($\tilde{m} = 45.20 \implies n\_{\mathrm{eff}} = 1\text{,}811.45$), the isotonic CORP-PAV decomposition ($\mathrm{BS} = 0.03210 = 0.00578 - 0.06183 + 0.08815$), and the rational decision `do_not_protect` whenever $p = 0.09800 \le p^{\ast} = 0.12000$:

~~~yaml
{{#include ../cards/card_01_atlantic_storms_evacuation.yaml}}
~~~

---

## 13.2 `card_02_bank_queue_staffing.yaml` (Queueing Convexity & Shift Staffing)

This card models 5-minute customer call arrivals across $164$ business days (`data/bank_calls.csv`, $n = 27\text{,}716$). It illustrates **Bridge $L\_2 \to L\_3$** (Prohibition 2, with Jensen's Gap $\Delta\_J = +9\text{,}033\text{,}837.19$) and **Invariant $\Phi\_2$** (where $\tilde{m} = 169.0$ and $\rho\_I(0.90) = 0.1623$ inflate variance by $\mathrm{DEFF} = 28.26\times$, reducing $n = 27\text{,}716$ to $n\_{\mathrm{eff}} = 980.70$):

~~~yaml
{{#include ../cards/card_02_bank_queue_staffing.yaml}}
~~~

---

## 13.3 `card_03_nyc_flights_holding.yaml` (Bitemporal Aviation Governance & Performative Holding)

This card governs ground-holding and gate-buffering decisions across $n = 138\text{,}375$ New York flight departures (`data/nyc_flights_2013.csv`). It operates on **Layer $L\_3$** under **Knightian** uncertainty, auditing performative air-traffic feedback ($r\_{\mathrm{RRM}} = 0.30 < 1$), Info-Gap robustness ($\hat{\alpha} = 2.80$) with a verified Global Wald Maximin floor, and bitemporal/drift exit clauses:

~~~yaml
{{#include ../cards/card_03_nyc_flights_holding.yaml}}
~~~
