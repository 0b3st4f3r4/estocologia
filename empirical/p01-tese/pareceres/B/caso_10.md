# Opinion — Case 10: shipping packaging (non-inferiority)

## Design
The design is good: draw per shipment with probability 1/2, margin of non-inferiority (+0.5 p.p.) and decision rule recorded in advance, 20,000 shipments per arm and complete outcome. The question is whether the upper limit of the CI of the difference (new − current) stays below +0.5 p.p.

## Results
- Damage: 1.91% in the current and 1.98% in the new. Difference of +0.07 p.p., with a 95% CI from −0.20 to +0.34 p.p., which reproduces the client's calculation.
- The conventional non-inferiority criterion uses the two-sided 95% CI, equivalent to a one-sided test at 2.5%. The upper limit, 0.34 p.p., stays below the margin of 0.5 p.p. By the 90% CI, the limit would be 0.30.
- Adjusting by month (OLS with robust errors): +0.07 p.p., CI from −0.20 to +0.34, identical.
- Heterogeneity by month: the differences were +0.29 (month 1), +0.27 (month 2) and −0.35 p.p. (month 3). The likelihood ratio test for packaging × month interaction gives p = 0.10, with no evidence that the effect changes over time. The individual monthly CIs are wide (upper limits of ~0.75) because each has a third of the sample. They are not the pre-registered analysis and must not replace it.
- The groups are balanced by month (≈ 6,600–6,700 per cell).

## Caveats
- The margin of 0.5 p.p. is equivalent to up to ~26% of relative increase over 1.9%. That was the client's business decision, recorded in advance, and it is respected here.
- The damage is recorded by the client within up to 15 days, which measures claimed damage. It is worth monitoring the rate after the change.

## Conclusion
By the pre-registered rule, the new packaging is non-inferior: the maximum plausible increase in damage (+0.34 p.p.) stays below the margin of 0.5 p.p.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Large randomized trial, with pre-registered margin and rule. The damage difference (new − current) is +0,07 p.p., with a 95% CI from −0,20 to +0,34 p.p., whose upper limit is below the margin of 0,5 p.p., with no significant heterogeneity by month.
```
