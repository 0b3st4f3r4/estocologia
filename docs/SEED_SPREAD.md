# The spread of the seed over whole laboratories

Record of `cargo run --release --bin labs -- --seed-spread`, run on 2026-10-19 with a binary built from the source of commit `5c7f238` (only the text of the register in
`mc_error.rs` changed between the build and the commit, and none of the instruments rerun here reads it; the
instrument's run took 14 minutes on two cores, which is why it is not in `scripts/verify.sh`; rerun it with the
command above and compare). Every block below is the literal stdout. The instrument reruns each of the thirteen
instruments whose generators the register of seeds listed as not audited, under `STOCHOLOGY_SEED_OFFSET` = 1 to 20,
and compares each line with the published run: the numbers of a line may move (that is the Monte Carlo error); the
words of a line should not, because a word that moves is a verdict that depends on the draw.

**What it found.** Three lines of 312 have words that move. Chapter 9: the set {x+w}, which contains the true cause,
is accepted in the published run and rejected under 1 of 20 seeds, at a test of level 0.05; the chapter's 200
repetitions already show a rejection rate of 0.080 for that set, and the intersection, {x}, does not move. Chapter 15:
the Hill estimate at the cutoff k = 250 reads the tail as not wild (above 2) under 2 of 20 seeds, and the invariance
test of rule 1, with a tolerance of 0.5, fails under 2 of 20 seeds although the effect is the same in every regime.
Chapters 9 and 15 now say so.

```text
== The spread of the seed over whole laboratories: what moves, and whether a verdict moves with it ==
each instrument rerun under seed offsets 1 to 20 and compared line by line with the published run (offset 0)

-- labs --chapter 4 --
  5 lines | 16 numbers, 4 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 5 --
  14 lines | 84 numbers, 37 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 6 --
  13 lines | 57 numbers, 26 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 9 --
  52 lines | 137 numbers, 74 of them move under another seed | lines whose words move: 1 | runs with another number of lines: 0
  line 24, words differ in 1 of 20 seeds
    published: candidate {  x+w}  F =    0.640  p = 6.984e-1  common = 1.988  per site = [1.970, 1.996, 1.990]  ACCEPTED
    another:   candidate {  x+w}  F =    2.101  p = 4.981e-2  common = 2.008  per site = [2.014, 2.035, 1.990]  rejected

-- labs --chapter 11 --
  60 lines | 148 numbers, 0 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 12 --
  17 lines | 73 numbers, 37 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 15 --
  18 lines | 73 numbers, 22 of them move under another seed | lines whose words move: 2 | runs with another number of lines: 0
  line 9, words differ in 2 of 20 seeds
    published: Scalonomy: hill_alpha(k=250) = 1.8444 wild_tail=true values_above_1000=0
    another:   Scalonomy: hill_alpha(k=250) = 2.0961 wild_tail=false values_above_1000=0
  line 16, words differ in 2 of 20 seeds
    published: Morphonomy: rule 1 -> marginal slope = 24.37 (treated fraction = 0.474) | by regime = [9.96, 10.09, 10.15] | invariant(tolerance 0.5) = true
    another:   Morphonomy: rule 1 -> marginal slope = 22.14 (treated fraction = 0.514) | by regime = [9.58, 10.34, 9.95] | invariant(tolerance 0.5) = false

-- labs --chapter 17 --
  16 lines | 30 numbers, 2 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 19 --
  38 lines | 146 numbers, 47 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --chapter 20 --
  11 lines | 32 numbers, 20 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --bank --
  6 lines | 17 numbers, 1 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --queue --
  19 lines | 103 numbers, 30 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

-- labs --rehearsal-shelf --
  43 lines | 168 numbers, 48 of them move under another seed | lines whose words move: 0 | runs with another number of lines: 0

312 lines in 13 instruments; lines whose words move under another seed: 3
```
