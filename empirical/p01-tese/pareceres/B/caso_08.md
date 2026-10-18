# Opinion — Case 08: change of bearing supplier

## Reasoning error
A p = 0.81 in Fisher's test means only that no difference was detected. It does not demonstrate equivalence. To state that the change does not worsen quality, it is necessary to show that the plausible increase in the defect rate stays below the relevant margin, which quality set at 3 p.p. That is a non-inferiority test.

## Results
- Defect rates: current 8/150 (5.3%), new 10/150 (6.7%), a difference of +1.3 p.p. The Fisher p = 0.81 is reproduced.
- CI of the difference (new − current):
  - 95% Newcombe: −4.4 to +7.1 p.p.; Agresti-Caffo, −4.3 to +6.9.
  - 90% (equivalent to a one-sided non-inferiority test at 5%): −3.4 to +6.1 p.p.
  - With error clustered by batch (5 batches per supplier), the 95% CI goes from −4.0 to +6.7 p.p.
- In all the methods the upper limit (≈ 6–7 p.p.) stays well above the margin of 3 p.p. The data are compatible with an increase in defects of more than double the tolerated one.
- Power: with 150 pieces per supplier, the chance of detecting a real increase from 5.3% to 8.3% was only ~27%. The study did not have the size to answer the question.
- The batches of the new supplier range from 1 to 3 defective in 30, with no extreme batch.

## Recommendation
To demonstrate non-inferiority with a margin of 3 p.p. from a rate of ~5–6%, on the order of 1,000 or more pieces per supplier are required, taken from several batches. Another way out is a gradual change, with reinforced inspection and a stopping rule.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: Absence of significance is not equivalence. The CI of the defect difference (new − current) goes up to about +6 to +7 p.p., well above the relevant margin of 3 p.p., and the study had only ~27% power to detect that increase.
```
