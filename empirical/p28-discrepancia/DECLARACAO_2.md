# Declaration 2 of point 28: the spread of the prediction at twice the range

**Written on 2026-10-19, after the first run of `labs --discrepancy` (output in `saida.txt`) and before the second.**
Written with an AI assistant.

## What the first run printed, and what was wrong with reading it

The first run printed, for each fit, the **mean** error of its prediction at x = 8 over the 1,000 datasets: +0.803
without discrepancy, +0.236 with the constrained discrepancy, +0.101 with the free one. A reading drafted for Chapter 14
said that the free fit "predicts best at x = 8". The reimplementation from the text (`scripts/reimplementation/`) then
found +0.516 and +0.382 for the last two, and the search for the cause found no error of arithmetic in either: the fits
agree with exact rational arithmetic on the noiseless world (+0.2784 and +0.1437, now a unit test), and the difference
is sampling. The prediction of a quartic discrepancy at twice the range has a standard deviation of about 6.9 per
dataset, so the mean of 1,000 of them moves by about 0.22 from one set of draws to another; without discrepancy the
same standard deviation is about 0.03. The mean error measured the bias of each fit and hid its spread, and the
drafted reading took a small bias for a good prediction. It was not published.

## What changes

The instrument also prints the root mean square error of the prediction at x = 8 over the datasets, which carries the
spread. Nothing else changes: the world, the design, the fits, the priors, the seed, the rules and their thresholds
are those of declaration 1, and none of the three rules reads the prediction at x = 8. The new hash of the instrument
enters `DECLARACAO.sha256`; the hash of the first version stays there as a line declared superseded.

## K

The rules did not change, and no rule's verdict depends on the column added. The change is to what is reported, made
after reading a result, and it is counted: K = 2 for this point.
