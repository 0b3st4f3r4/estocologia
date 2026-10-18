# Opinion — Case 05: salesperson selection test

## Results
- Pearson r = 0.65 (p = 0.0001) is reproduced. But the rank correlations, robust to extreme values, are almost null: Spearman ρ = 0.14 (p = 0.45) and Kendall τ = 0.11 (p = 0.39).
- A single salesperson, V29 (score 97, sales of 980 thousand, more than double the second largest, 493 thousand), concentrates the whole correlation. His Cook's distance is 9.1 (the usual alert threshold is ~1) and the studentized residual is 8.4. The scores of the other 29 range from 45 to 75.
- Without V29: r = 0.11 (p = 0.58), Spearman 0.05. Between 45 and 75 points, the range in which practically the whole decision lies, the score does not predict sales.
- The bootstrap of r (10,000 resamples) gives a 95% CI from −0.19 to 0.87, very unstable. The regression residuals are not normal (Jarque-Bera p = 0.001), which invalidates the Pearson p.
- Proposed cutoff of 70 points: those who scored ≥ 70 (n = 6) have a median of sales of 413.5 thousand, against 413.2 thousand for those who scored < 70 (n = 24); Mann-Whitney p = 1.0. Apart from V29, those approved at the cutoff sold the same or less (e.g. V24, score 75, sold 339 thousand).

## Positive point of the design
Last year the score was not used to hire, so there is no restriction of range by selection. The problem is another one: the sample is small and the association depends on one point.

## Conclusion
The "strong correlation" is an artifact of a single atypical case. In the mass of the data there is no relation, and the cutoff at 70 does not separate better salespeople. Adopting the eliminatory filter now would discard candidates with no demonstrated gain. Before that, it is worth accumulating more validation data with the score still not used as a filter.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: The r = 0,65 is produced by a single atypical salesperson (score 97, sales of 980 thousand; Cook = 9,1). Without him, r = 0,11 (p = 0,58), and the Spearman correlation with everyone is 0,14 (p = 0,45). The cutoff at 70 does not separate sales (medians of 413 against 414 thousand).
```
