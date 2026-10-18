# Opinion — Case 03: incentive program for managers

## Design
There are 60 stores, 30 drawn for the program. The unit of analysis (store) coincides with the unit of draw, and the outcome and the method were pre-specified: log of the ratio between the mean weekly sales of the 12 weeks after and that of the 12 weeks before. There are no missing values: 60 stores × 12 weeks, nobody opened or closed. The baseline sales are similar between groups (means of 108.8 and 102.5 thousand/week, within what is expected under a draw). Since the outcome is a ratio with respect to its own baseline, the level difference between stores does not matter either.

## Results
- Regression of the log-ratio on the group: +0.0679 (SE 0.015), that is +7.0% (95% CI from +3.9% to +10.3%), p < 0.001. The client's numbers are reproduced.
- Robustness:
  - HC3 errors give the same result, p < 0.001.
  - Mann-Whitney: p = 2e-5.
  - Permutation test with 20,000 rearrangements: p < 0.0001.
  - No single store dominates; the whole distribution of the program group is shifted (median 8.6% against −0.5% in the control).
- Effect per week: the sales/baseline ratio in the program group stays between 1.05 and 1.10 in all 12 weeks, against 0.98–1.04 in the control. It is stable, with no initial peak that dissipates.

## Implication for the decision
The cost is about 1% of sales. Even the lower limit of the CI (+3.9%) exceeds that cost with room to spare, so the evidence of a sales increase is solid. Two caveats, which do not change the verdict: the criterion of economic decision should be margin, not revenue (the program pays for itself if the incremental margin on +7% of sales exceeds 1% of sales); and 12 weeks do not guarantee persistence in the long run, so it is worth monitoring after the expansion.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: Randomized experiment by store, with a pre-specified analysis in the correct unit: +7,0% (95% CI from 3,9% to 10,3%), robust to nonparametric and permutation tests, and stable across the 12 weeks. The lower limit exceeds the cost of ~1% of sales.
```
