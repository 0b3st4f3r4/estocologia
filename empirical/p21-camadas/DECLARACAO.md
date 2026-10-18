# Declaration of point 21: can the layers fail?

**Written on 2026-10-18, before any error is drawn, coded, or counted.** The SHA-256 of this file is in
`DECLARACAO.sha256`. A change after that enters as a new declaration and counts in the K.

## 0. Provenance

It comes from the outside review of 2026-10-18 requested by the owner (`empirical/p16-ponte/DECLARACAO_3.md` §0
describes the reviewer and why it is not an independent specialist).

## 1. The thesis under test

The paper states that most errors of reasoning under uncertainty are not errors of arithmetic but misalignments between
World, Model, and Agent (thesis **(a)**). World, Model, and Agent cover by construction the reality, the representation,
and whoever decides: any error can be redescribed as a wrong bridge. A classification that absorbs every case is a
vocabulary, and a vocabulary cannot be refuted. This point asks the question that separates a typing from a vocabulary:
**do independent coders, with the same codebook, put the same error in the same place?**

## 2. Material

- **At least 60 documented errors** from sources outside the program: published retractions and corrections that state
  a statistical or inferential reason, the error lists of the methodological literature, and public post-mortems of
  decisions. The source list, the rule that draws from it, and the seed are frozen in a second declaration, with its
  own hash, **before** any draw. Nothing that the program wrote or analysed enters the material.
- Each error is presented as a short neutral description: what was claimed, from which data, and what went wrong
  according to the source. The layer vocabulary does not appear in it.

## 3. Codes

One code per error, from a closed list of seven:

| Code | Meaning |
| --- | --- |
| WM | the Model does not represent the World it is used for |
| MA | the Agent reads the Model as answering a question it does not answer |
| WA | the Agent acts on the World without the Model that would mediate it |
| WMA | the error needs all three to be stated |
| ARIT | an error of arithmetic or procedure, with the bridges right |
| FORA | outside the frame: none of the above describes it |
| NC | cannot be coded from the description |

**ARIT and FORA are the falsifiers.** They are the codes that let the material say that the frame is wrong, and the
test is only fair if coders can and do reach for them.

## 4. Coders

At least **three**, of whom **two from outside the program**, coding independently and blind to each other. The
codebook is §3 of the paper and this table, and nothing else. A coder sees no other coder's codes before closing their
own.

## 5. Metrics

Fleiss' κ over the seven codes and per code (one against the rest), with a 90% interval by bootstrap over the errors;
the share of errors coded ARIT and FORA by the majority, with their intervals.

## 6. Decision rule, fixed in advance

- **The layers are a typing** if overall κ ≥ 0.60 and the lower bound of its interval ≥ 0.40.
- **The layers are a vocabulary** if overall κ < 0.40.
- Between the two, **inconclusive**, reported as such.
- **"Most errors are misalignment, not arithmetic"** is sustained only if the layers are a typing **and** the
  per-code κ of ARIT is at least 0.40 (coders agree on what is arithmetic, so the category is alive) **and** the upper
  bound of the ARIT share plus the FORA share is below 0.5.
- If ARIT is almost never used by any coder, the sentence is **not testable by this design**, and that is the
  result: a frame whose falsifier nobody reaches for is not refuted, but it is not confirmed either.

## 7. Rehearsal

Before people, the same design runs with three model coders of different families. The output carries the label
`SYNTHETIC` and measures only the floor: what κ fluent coders reach without being outside the program.

## 8. Dependence and K

The coders from outside the program are **people**, and by the owner's decision of 2026-10-18 the program recruits no
panel on its own: this point enters `docs/DECLARED_PROPOSALS.md` as item 14, with no deadline. **K = 1.**
