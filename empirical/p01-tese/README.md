# P01 · Pilot of the central thesis

## Result

**Tie at the ceiling: the pilot does not support the thesis, by the rule declared in advance.** Both arms pointed out the planted
defect in the 8 cases with defect and gave no false alarm in the 4 clean cases (blind judge, `juiz/notas.yaml`).
Sensitivity 8/8 in both; false alarm 0/4 in both; no discordant pair, so the McNemar test has nothing
to test. `DECLARACAO.md` fixed that a tie at the ceiling counts as a thesis not supported.

The honest reading is that the pilot does not discriminate: the ten errors of the list of Makin and Orban de Xivry (2019) are the
errors that any competent statistician looks for, and a model instructed only to "analyse rigorously" found all of them.
The pilot does not refute the thesis, but it shows that, for these errors, the architecture adds no accuracy. If the thesis
holds, it holds for errors that the list does not cover, and a fair test of that needs harder cases chosen by
someone outside the program, human analysts and a human judge.

The verdicts of the two arms coincided in the 12 cases. Arm A left aside a requirement of the architecture (the
property tests with Hypothesis), and the three roles are models of the same family, which may explain part of the
coincidence.

After the scoring, the answer key, the generators and the X/Y map were published in this folder (`gabarito/`, `juiz/`); the
SHA-256 of the answer key matches `GABARITO.sha256` (4d5450eb…).

## What was done

1. **External list of defects.** Makin, T. R. & Orban de Xivry, J.-J. (2019), "Ten common statistical mistakes to
   watch out for when writing or reviewing a manuscript", *eLife* 8:e48175, doi:10.7554/eLife.48175. The page
   https://elifesciences.org/articles/48175 did not load (it requires JavaScript); the list was checked in the PMC copy,
   https://pmc.ncbi.nlm.nih.gov/articles/PMC6785265. The ten errors are transcribed in section 2 of `DECLARACAO.md`.
2. **Draw.** Eight of the ten errors were drawn without replacement with `np.random.default_rng(20261015)`. Two errors
   stayed out (which ones are in `DECLARACAO.md`, section 8).
3. **Cases.** Twelve cases: eight with one defect each and four clean. The order was shuffled with a fixed seed.

   | Case | Data |
   |---|---|
   | caso_01 | real |
   | caso_02 | simulated |
   | caso_03 | simulated |
   | caso_04 | simulated |
   | caso_05 | simulated |
   | caso_06 | real |
   | caso_07 | simulated |
   | caso_08 | simulated |
   | caso_09 | real |
   | caso_10 | simulated |
   | caso_11 | simulated |
   | caso_12 | simulated |

   The three real cases use files already downloaded in `/mnt/project-files/estocologia/data/` (see the `SOURCES.md`
   there): NSW/CPS of LaLonde, Project STAR and the light tunnel of the Causal Chambers. The columns were renamed and the
   description is that of a client.

   Each case is in `/mnt/project-files/estocologia/empirical/p01-tese/casos/caso_XX/` with `alegacao.md`, one or more CSV
   and `dicionario.md` (3.2 MB in total; that is why it is outside Git). The hashes of all the files are in `CASOS.sha256`
   (a copy of `casos/SHA256SUMS`).
4. **Answer key and generator.** `gabarito.yaml` (for each case: clean or with defect, which error of the list, where it is, the
   correct conclusion, the objective criterion of the judge and the calculated numbers) and `gerar_casos.py` (generates the 12 cases and the
   answer key) stay in the `p01-gabarito` folder of the coordinator, outside the repository, because both reveal the defect.
   Hashes in `GABARITO.sha256` and `GERADORES.sha256`.
   *Rectification of 2026-10-18:* this item describes the state **before** the scoring. After it, the answer key
   entered the tree at `gabarito/gabarito.yaml`, as the Result section says, and `GABARITO.sha256` now verifies a live
   target (4d5450eb…); the generator stayed outside, and `GERADORES.sha256` remains declared without a live target. The
   sealed `DECLARACAO.md` still says that the answer key "stays outside the repository", which was true when it was
   frozen. For the fair test of point 16, the answer key must stay out of the tree until the judge has scored, and the
   selector must not hand it to any arm.

## Numbers

All the numbers cited in the claims and in the answer key come from `gerar_casos.py`, run over the files of
`data/` or over simulated processes with a fixed seed. No number was typed by hand. The numbers of
section 6 of `DECLARACAO.md` (minimum p of the exact McNemar) come from `scipy.stats.binomtest`.

## How to reproduce

With access to the answer-key folder:

```
python3 -I p01-gabarito/gerar_casos.py /mnt/project-files/estocologia/dados-reais \
    /mnt/project-files/estocologia/empirical/p01-tese/casos p01-gabarito/gabarito.yaml
cd /mnt/project-files/estocologia/empirical/p01-tese/casos && sha256sum -c SHA256SUMS
sha256sum p01-gabarito/gabarito.yaml   # compare with GABARITO.sha256
```

The generator was run twice in a row; the hashes of all the case files and of the answer key matched.
It depends on numpy, pandas, scipy, statsmodels and PyYAML.

## Next steps (protocol in DECLARACAO.md)

1. Arm A (architecture) and arm B (common analysis) analyse the 12 cases, each one isolated, with the final verdict block.
2. The coordinator blinds and shuffles the opinions; the judge applies the criterion of the answer key.
3. Sensitivity (/8), false alarm (/4), exact McNemar and the declared decision rule.

## Limits

Twelve cases do not give power for a firm conclusion; the three roles are models of the same family; the planter knows the
catalogue of Stochology; the real cases come from famous sets that a model may recognize. Details in section 8
of `DECLARACAO.md`.
