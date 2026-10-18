# Opinion — Case 02: supplement in the feed of piglets

## Central problem: the unit of draw is the pen, not the piglet
The supplement was drawn among 8 pens (4 against 4) and is given by a common feeder, so that all the piglets of a pen share treatment, environment and feeder. The t test with 240 piglets treats as independent animals that are not, and the p < 0.001 is illusory. The effective sample for the treatment effect is 8 units.

## Results
- Weight gain (final − initial) per piglet: 21.77 kg with supplement and 19.51 kg in the standard, a difference of 2.26 kg. The naive t gives p ≈ 4e-8, which reproduces the client's calculation.
- Means per pen. Supplement: 20.4; 18.7; 25.2; 22.9. Standard: 21.1; 20.6; 16.2; 20.2. The variation between pens within the same treatment (16.2 to 25.2) is larger than the difference between treatments.
- Intraclass correlation of the gain, by a mixed model with random intercept for pen: ICC ≈ 0.58, very high. The design effect, 1 + 29·0.58 ≈ 18, reduces the 240 piglets to something like 13 independent ones.
- Correct analyses:
  - t test with the 8 pen means: difference of 2.26 kg, 95% CI from −2.17 to +6.70, p = 0.26.
  - Mixed model (random pen): 2.26 kg, SE 1.81, p = 0.21.
  - OLS with initial weight and error clustered by pen: 2.27 kg, CI from −1.04 to 5.58, p = 0.18.
  - Exact randomization test, with the 70 ways of choosing 4 of 8 pens: p = 0.29.
- The initial weight is similar between the groups and explains nothing.

## Implication for the decision
The data are compatible both with no effect (or even a loss) and with a large gain. There is no basis for an expensive purchase for 30 thousand piglets per year. For a decision of that size it would take a trial with many more pens, or with draw by animal and individual feeder, sized for the minimum gain that pays for the supplement.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: The treatment was drawn by pen (4 against 4), with ICC ≈ 0,58. Analysed in the correct unit, the effect of 2,26 kg has a 95% CI from −2,2 to +6,7 kg and p ≈ 0,2–0,3 (exact randomization p = 0,29). The p < 0,001 comes from treating 240 piglets as independent.
```
