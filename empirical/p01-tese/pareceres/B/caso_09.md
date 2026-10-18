# Opinion — Case 09: small classes in early childhood education

## Design
The students and teachers were drawn for the type of class within each school, so that the small × regular comparison within the school is experimental. The data have 6,325 students (1,900 in a small class and 2,194 in a regular class, besides the classes with an aide, which I excluded from this comparison, as the client did).

## Checks
- **Missing values**: 8.5% without a score in the small class and 8.6% in the regular one. Within the school, the difference is −0.3 p.p. (p = 0.80). The loss is small and does not differ between arms.
- **Balancing** (within the school) of black race, free lunch and sex: differences < 0.6 p.p., all with p > 0.6. The teacher's experience is similar (8.9 against 9.1 years).
- **Reproduction**: raw difference of 13.9 points. With school fixed effect and error clustered by school (79 schools, 3,743 students): 16.0 points (95% CI from 7.4 to 24.6; p = 0.0003), equal to 0.22 SD of the regular classes' score (SD = 73.1).
- **Robustness**:
  - With covariates (sex, ethnicity, lunch, teacher's experience): 16.1 (CI from 7.9 to 24.4).
  - By component: reading +6.6 (CI from 3.1 to 10.1) and mathematics +8.8 (CI from 3.3 to 14.4).
  - In 78 schools with both types of class, the mean difference within the school is +15.3, positive in 63% of them.
- **Worst-case bound for the missing values**: imputing the 5th percentile to the missing values of the small class and the 95th percentile to those of the regular one, the effect becomes −7.3 (CI from −16.7 to 2.1). It is an extreme and implausible scenario, given that the loss is equal between arms and the covariates are balanced. Under any mechanism that is not strongly differential, the effect is maintained.

## Caveats
- The real clustering is by class/teacher within the school. Clustering by school is conservative in that sense, and the result is maintained.
- External validity is the weak point: this is another country, another period and "small" classes with a specific size (~13–17 students). The result justifies a pilot, which is what is asked, but does not guarantee the same effect size in the client's network.

## Conclusion
The experimental evidence supports that small classes raised the scores at the end of that year, in an effect of ~0.2 SD, robust and with balanced losses. It is enough to justify a pilot, preferably itself evaluated with a draw.

```
VEREDITO: SUSTENTADA
MOTIVO PRINCIPAL: With a draw within the school, school fixed effect and clustered errors, small classes had +16 points (95% CI from 7,4 to 24,6; ~0,22 SD), with balanced missing values (8,5% against 8,6%), balanced covariates and consistent results in reading and mathematics. The transposition to another network is the main caveat, adequate to a pilot.
```
