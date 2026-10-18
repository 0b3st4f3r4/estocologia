# Opinion — Case 06: supported employment program

## Central problem: comparison with an incomparable group
The participants are men who enrolled in a program for people with difficulty in keeping a job. The national sample represents the working population in general. Before the program, the groups were already radically different:

| | Program | National sample |
|---|---|---|
| Age | 25.8 | 33.2 |
| Years of schooling | 10.4 | 12.0 |
| Black | 84% | 7% |
| Married | 19% | 71% |
| No high school | 71% | 30% |
| Income 1974 | US$ 2,096 | US$ 14,017 |
| Income 1975 | US$ 1,532 | US$ 13,651 |
| No income in 1975 | 60% | 11% |

The income difference in 1978 (−8,498) is smaller than the difference that already existed in 1975 (about −12,100). That is selection, not an effect of the program. Controlling only for age and schooling (−6,485, which I reproduced) leaves out precisely the strongest predictor of future income: past income.

## Analyses that take selection into account
- Regression with age, schooling (with quadratic terms), race, marital status, diploma, incomes of 1974–75 and indicators of zero income: +1,184 (95% CI from −46 to +2,414).
- Differences in differences (income change 1975→1978): +3,621 (CI from 2,426 to 4,817) without covariates; adjusted, +1,184.
- Propensity score:
  - The participants have a mean score of 0.40, against 0.007 for the controls, which shows poor overlap. I applied trimming to the common support (4,458 controls).
  - 5-NN matching on the logit of the score: effect on the treated (ATT) of +1,344, with a bootstrap 95% CI from −116 to +3,014. With the matching, the balance of the covariates became good (income 1975: 1,532 against 1,688; black: 84% against 86%).
  - Propensity weighting (ATT): +1,593.
- Restricted to people with income < 5,000 in 1974 and 1975: +2,612.

All the estimates that adjust for prior income have a positive sign, that is, they point to an income gain and never to a loss. Even so, they are observational estimates with limited overlap and depend on there being no residual confounding. The ideal scenario would be to compare with a control group drawn among the enrollees themselves, if one exists.

## Conclusion
The conclusion of the board, that the program reduced income, results from comparing populations that were different from the start. The data do not support it. Adjusted for income history, they indicate a null to positive effect.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: The difference of −8.498 reflects selection: the participants already earned about US$ 12 thousand less in 1975 and differed in race, marital status and schooling. Adjusting for pre-program income (regression, DiD, propensity matching), the estimates lie between +1,2 and +3,6 thousand, with no sign of income reduction.
```
