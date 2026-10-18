# Point 16 — the bridge thesis at scale

**State: thesis NOT SUSTAINED, box closed, and the reason written — people are missing.**

The P01 pilot gave a **tie at the ceiling**: both arms pointed out the 8 planted defects and gave no false alarm
in the 4 clean cases, and the rule declared beforehand says that a tie at the ceiling counts as a thesis not sustained. This
front re-derives that result from the scores of the blind judge (`empirical/p01-tese/juiz/notas.yaml`) — 8 of 8 in both
arms, 0 of 4 false alarms, 0 discordant pairs — and **locks the thesis** until the fair test exists.

**What is missing, named.** A selector of cases **from outside the program**, who chooses before any
result exists; two human analysts per arm, with at least one from outside; a human judge from outside and blind to the arm; and
at least 12 cases harder than those on the list of Makin and Orban de Xivry (2019) — the list that any
competent statistician looks for, and that is why the pilot did not discriminate.

**The owner's answers (2026-10-16), recorded.** There is nobody from outside to choose the cases, there is no team of
analysts, there is no external judge — and the owner accepts the mechanical rule of choice and accepts, without reservations, the
definitive verdict if the window expires. `DECLARACAO_2.md` keeps the five answers, and the rule is in
`regra_de_escolha.md`.

**The gate.** While that does not exist, no line of the program may say that the thesis is sustained: the
instrument rejects whoever states the contrary, and there is a test for that. The deadline of the window is **2027-01-14**; expired
without the minimum coverage, the thesis stays **not sustained definitively** as the verdict of the program.

**How to reproduce.**

~~~bash
cargo run --release --bin labs -- --bridge
~~~

*Rectification of 2026-10-18:* the command above read `--ponte` until this date; the flag was renamed `--bridge` in the
English migration of 2026-10-17, and `labs --ponte` is refused as an unknown flag.

The instrument reads the record (`registro.tsv`) and the scores of the judge, prints the re-derivation, the state of the roles, and the
two proofs: a record that says the thesis is sustained without the fair test is rejected, and with the roles designated and the
cases at the minimum the test can run — the status only changes with the result of the judge.

## Thesis (c′), enforced since 2026-10-18

`DECLARACAO_3.md` declared a second thesis, (c′): the advantage of the architecture concentrated at the seams, with
a seam stratum and a classical stratum of 12 cases each. Its record is `registro_c_linha.tsv`, separate from
`registro.tsv` so that thesis (c), its rule, and its window stay exactly as frozen. `labs --bridge` now prints a third
section that reads it (`crates/stochology-book/src/costura.rs`, nine tests): the seam kinds must be the closed list of
six; while either stratum is below its minimum or the roles of the fair test are pending, the box is closed and no
line may say `sustentada`; with both strata scored, the status is computed from the judge's scores and a declared
status that differs is rejected. Today: **(c′) not supported, box closed**, both strata at 0 of 12. The reliability
axis (κ over five runs per machine arm) stays descriptive and outside the gate, as declared.
