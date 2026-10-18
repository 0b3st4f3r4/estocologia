# Opinion — Case 12: screening of 40 antibacterial compounds

## Central problem: multiple comparisons
Forty compounds were tested with no prior hypothesis, each at α = 0.05. Even if none had activity, the expectation would be 40 × 0.05 = 2 "significant" results. Three were observed.

## Results
- The reproduced p-values are K40 p = 0.030 (−18.2%), K12 p = 0.035 (−11.3%) and K19 p = 0.037 (−11.6%).
- Corrected for 40 tests:
  - Holm: adjusted p = 1.0 for the three.
  - Benjamini-Hochberg: q = 0.49 for the three, that is, about half of the "discoveries" would be expected to be false.
- The distribution of the 40 p-values does not differ from a uniform one (Kolmogorov-Smirnov p = 0.11), which is the pattern expected if no compound has an effect. Of the 40, 22 reduced and 18 increased the growth, close to 50/50.
- There are effects of the same magnitude in the opposite direction: K11 +15% (p = 0.074), K17 +14%, K28 +12%. Differences of ±10–18% appear on both sides, which is consistent with noise of 8 wells per condition.
- The mean of the controls varies between plates from 0.91 to 1.09 (SD 0.047), which is why the comparison must be within the plate, as was done. Comparing against the pooled control of all the plates would make K19 look strong (p = 0.0002), but that test ignores the plate effect and is not valid. K40 and K12 do not even become significant that way.

## Recommendation
Before spending R$ 1.2 million, do a cheap confirmation stage: retest K40, K12 and K19 (and eventually the next on the list, such as K31) in several independent plates, with a positive control and dose-response, and with the hypothesis and the criterion defined in advance. Only those that replicate should move forward.

```
VEREDITO: NAO_SUSTENTADA
MOTIVO PRINCIPAL: With 40 compounds tested with no prior hypothesis, 3 significant at 5% is what is expected by chance (2 expected). Correcting for multiplicity, none survives (Holm p = 1,0; FDR q = 0,49), and the p-values are distributed as under total absence of effect. The three need replication before investing R$ 400 thousand in each.
```
