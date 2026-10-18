# Point 18 — the R axis in the 21 chapters

**State: closed. Every chapter has its axis line, and the line is literal output of the instrument.**

The R axis existed outside the book — `labs --resolucao` measures the resolution loss, and the axis document carries the
seven documented cases —, but it did not appear in any Step 3. Now it appears in all 21, in the same form: a short
paragraph, the cited command, and **one line** of the instrument.

The table `book/eixo-r.tsv` declares, per chapter, a case and an editorial sentence, and the instrument **computes** the
reading: a loss of 46.24 min where demand oscillates from 5 to 11 (chapters 5, 16, and 20), 5.97 min where it oscillates
from 7 to 9 (8), 0.00 min where it does not oscillate (10 and 18), and the oscillation threshold at amplitude 2.5 in
chapter 21. The other fourteen chapters say what they are — the loss **is not measured there**, because their example
does not contain the same decision at two grains.

**The proof.** The block checker of the book charges every block against the instrument, and it went from 27 to **48
blocks checked, with zero divergence**. No number of the axis was written by hand in the chapters.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --resolucao
~~~

**The limit.** The axis enters as a line, not as a new laboratory: the laboratory is a single one, and the loss is measured
only where both resolutions exist. The K of this front is 1 — the assignment of one case to each chapter was made once.
