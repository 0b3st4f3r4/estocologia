# Declaration of pilot P01: the central thesis of Stochology

Declaration written on 2026-10-15, after assembling the case bank and the answer key and before any opinion from the
analysts. The SHA-256 of this file is in `DECLARACAO.sha256`. Any change after that enters as a new
declaration and counts in the K.

## 1. Hypothesis

The central thesis says that the organization in layers, cards and gates of Stochology (World/Model/Agent layers,
YAML cards, invariants and approval gates, as described in AGENTS.md, docs/PLAN.md and in the Architecture) catches judgment
errors that a common and competent statistical analysis lets through, without giving more false alarms.

The pilot tests a narrow and operational version of the thesis: before client claims with data, the arm that
follows the architecture points out more planted defects than the common-analysis arm, without accusing more clean
cases.

## 2. Material

- Bank of 12 cases in `/mnt/project-files/estocologia/empirical/p01-tese/casos/caso_01` to `caso_12` (hashes in
  `CASOS.sha256`). Each case has `alegacao.md` (the client's claim with the decision question), one or more CSV and
  `dicionario.md`.
- Eight cases have one planted defect each, drawn with a fixed seed from the external list of Makin & Orban de Xivry
  (2019); four are clean (the claim is correct and well supported by the data). The order was shuffled with a
  fixed seed; the case number says nothing.
- Three cases use real data with a known answer already published; the others are simulated with a generative process and
  seed declared.
- The answer key (`gabarito.yaml`) and the generator (`gerar_casos.py`) stay outside the repository, in the `p01-gabarito`
  folder of the coordinator. In the repository only the hashes stay: `GABARITO.sha256` and `GERADORES.sha256`.

External list used: Makin, T. R. & Orban de Xivry, J.-J. (2019), "Ten common statistical mistakes to watch out
for when writing or reviewing a manuscript", *eLife* 8:e48175, doi:10.7554/eLife.48175. Checked at
https://pmc.ncbi.nlm.nih.gov/articles/PMC6785265 (the elifesciences.org page did not load without JavaScript). The ten
errors, in the order and with the titles of the article:

1. Absence of an adequate control condition/group
2. Interpreting comparisons between two effects without directly comparing them
3. Inflating the units of analysis
4. Spurious correlations
5. Use of small samples
6. Circular analysis
7. Flexibility of analysis: p-hacking
8. Failing to correct for multiple comparisons
9. Over-interpreting non-significant results
10. Correlation and causation

## 3. The three roles

- **Planter** (already done): assembled the cases and the answer key. Does not take part in the analysis nor in the judgment.
- **Arm A (architecture):** analyses the 12 cases following the architecture of Stochology: assembles the layers, fills in the
  cards and goes through the gates before giving the opinion. May read AGENTS.md, docs/PLAN.md and the Architecture.
- **Arm B (common analysis):** analyses the same 12 cases as a competent statistician would, without the architecture:
  receives neither AGENTS.md, docs/PLAN.md, the Architecture nor any text of Stochology. The instruction is "analyse the data and
  answer the client's decision question with the rigor of a good statistician".
- **Judge:** receives the 24 opinions without the identification of the arm and the answer key, and codes each opinion.

Rules common to both arms:

- Both receive only the folders `caso_XX`. Neither arm receives this declaration, the README of this folder, the answer key,
  the generator or the information of how many cases have a defect. The instruction does not say that there are planted defects; it says only that
  they are client requests.
- Same model, same tools (Python with pandas, scipy and statsmodels), each case analysed
  independently (without consulting the other cases), and a similar budget of time and of tokens.
- Each opinion ends with a fixed block:

  ```
  VEREDITO: SUSTENTADA | NAO_SUSTENTADA
  MOTIVO PRINCIPAL: <one to three sentences>
  ```

  `SUSTENTADA` means that the data support the claim and the decision asked; `NAO_SUSTENTADA`, that they do not
  support it.

Blinding of the judge: the judge decides the score by reading only the final block (verdict and main reason) and, if needed, the text
of the opinion without the cards. The coordinator erases from that material the mentions of the arm and the terms proper to the architecture
(names of layers, cards, gates, invariants) and labels the two opinions of each case as X and Y by a draw, with a
fixed seed recorded before opening the opinions. The judge may see the full opinion only after giving the score of the
case, and may not change it.

## 4. Coding by the judge

For each opinion, the judge applies the criterion written in the answer key for that case:

- **Case with defect:** "pointed out the defect" = 1 if the verdict is `NAO_SUSTENTADA` (or the opinion rejects the conclusion or
  the decision asked) **and** the reason names the mechanism of the planted defect, according to the objective criterion of the
  answer key. A verdict `NAO_SUSTENTADA` for another reason, without the mechanism, counts 0.
- **Clean case:** "false alarm" = 1 if the verdict is `NAO_SUSTENTADA`, or if the opinion states a defect that, according to
  it, invalidates the conclusion or the decision. Caveats that do not change the verdict do not count.

The judge records, for each opinion, the score (0/1) and a sentence quoting the passage of the opinion on which it relied.

## 5. Metrics

- **Sensitivity** of each arm = defects pointed out / 8.
- **False alarm rate** of each arm = clean cases accused / 4.
- **Case by case comparison:** exact McNemar test (two-sided binomial over the discordant pairs) in the 8 cases with
  defect; the same test in the 4 clean ones, as a description.
- Descriptive: case by case table (A, B), without weights.

## 6. Decision rule (fixed in advance)

- **The thesis is supported in the pilot** if the sensitivity of A is greater than that of B **and** the false alarm rate of A
  is not greater than that of B.
- **The thesis is not supported in the pilot** in any other case: sensitivity of A equal to or less than that of B
  (including a tie at the ceiling, 8/8 against 8/8), or A with more false alarms than B.
- The McNemar p is reported, but does not change the rule above.

Power: with 12 cases the pilot has no power for a firm conclusion and this is written here. In the exact McNemar with the 8
cases with defect, the smallest possible two-sided p with d discordant pairs, all in favour of A, is 2·0.5^d: 0.0625 with
d=5, 0.031 with d=6, 0.016 with d=7 and 0.0078 with d=8 (calculated with `scipy.stats.binomtest`). That is, there would only be
p < 0.05 if A got it right and B got it wrong in at least 6 of the 8 cases with defect and there were no discordant pair in the
opposite direction. With 4 clean cases, a difference of one false alarm is 25 percentage points and nothing can be
concluded about the alarm rate. A favourable result justifies a larger study; it does not confirm the thesis. An
unfavourable result counts against the thesis and must be reported as such.

## 7. What counts as a failure

- Of the pilot as a test: if an arm does not deliver the verdict block in some case, that case counts as not
  pointed out (defect) or as not accused (clean) for that arm, and this is reported.
- Of the case bank: no case is withdrawn after seeing the opinions. If an opinion shows that a clean case has
  a real problem that was not planted, or that a case with a defect has a second defect, that is recorded as a failure of the
  planting; the main result continues to be that of the 12 cases as declared, and the result without the affected case is
  reported alongside.
- Of the thesis: the criterion of section 6 not satisfied.

## 8. Declared limitations

- The three roles (planter, analysts and judge) are language models of the same family. Errors common to the model
  appear in both arms and the judge may share the reading bias of the analysts.
- The planter knows the catalogue of Stochology and the list of Makin. He may have written cases that the architecture catches more
  easily, even without intending to. The errors came from an external list, drawn, to limit that effect, but
  the text of the claims and of the dictionaries is the planter's.
- Errors 1 (lack of an adequate control) and 6 (circular analysis) stayed out of the draw.
- The three cases with real data come from famous sets. Even with renamed columns and a neutral description, a model
  may recognize them and remember the published answer, which favours both arms equally.
- The simulated cases with a defect were generated by searching, from a base seed, for the first seed in which the
  defect appears (for example, a test per animal significant with the test per pen not significant). This is
  part of the planting and is declared in the answer key.
- Twelve cases are few and the metrics are discrete (steps of 12.5 and 25 percentage points).
- Arm B is "common and competent statistical analysis" done by a model instructed for that; it is not a human
  statistician.
