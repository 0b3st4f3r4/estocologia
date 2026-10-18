# Declaration 2 of point 16: the mechanical choice and the verdict of the window

**Frozen on 2026-10-16T22:51:23Z.** The `DECLARACAO.md` stays as it is; this second declaration lives alongside it, which is what
the protocol orders when a criterion gains new content.

| file | before | now | why |
| --- | --- | --- | --- |
| `registro.tsv` | `fb547bcd6de4fc7f047c10f0b095b551fb5c1b3ace09784a2f25f235ee0c23c8` | `7ebd1c65c59e6d46e6c5335e8355e4a2b5858b225c7f782146d03a8679c7780c` | the verdict of the window entered, and the pointer to the rule of choice |
| `regra_de_escolha.md` | — | `18648bee18c09e856d4f568026e1b15dd8f1f8d31f9c4b631fce13f2d4ebb7cd` | the mechanical rule, frozen before a list exists |
| `DECLARACAO.md` | `109910beac758a052676c928da9ef7e890d4bf60fa165b22262441795b5fa96a` | the same | it stays as the record of the freezing |
| `DECLARACAO_2.md` (this one) | — | — | the answers of the owner; its own hash comes out in `DECLARACAO.sha256` |
| `README.md` | `c204219a96e412398df83da135422821cbefc930da6afed2b85ee46b711a3ff5` | `3b60e6bf8f7608547aa83a0164b546e34612b9a5b7e13eabcc201a96663e8501` | the record of the answers of the owner entered, and the pointer to the rule of choice |

## Declaration 5 — the answers of the owner, recorded

Asked on 2026-10-16 about the four roles of the fair test, the owner answered the following, and the answer stays
recorded as a declaration, and not as a conversation:

1. **Selector of the cases:** there is nobody today, outside the program, for the role of exogenous selector.
2. **Analysts:** there is no team. The owner can act as internal analyst in one of the arms; there is no second human
   analyst, much less an independent external analyst, to compose the two arms.
3. **Judge:** there is none. Without an independent researcher outside the circle of the program, the figure of the blind judge is
   empty.
4. **Rule of choice:** accepted in full, and the reason is the right one — any choice made by the author contaminates
   exogeneity and invalidates the test by selection bias. The choice has to be algorithmic or delegated to a blind external
   source. The rule is in `regra_de_escolha.md`, with the seed declared as **20270114**.
5. **If the window expires with nobody:** the owner accepts **without reservations** the definitive verdict — the thesis of the bridge at
   scale stays **not sustained**, and the program treats it exactly so. A thesis does not live on a perpetual promise.

## Declaration 6 — what this changes, and what it does not change

**It changes no criterion.** The exit clause of declaration 4 already said that the window expired without minimum coverage
leaves the thesis not sustained definitively; the owner confirmed the consequence, and for that reason nothing enters the K.
What changes is the record: the `registro.tsv` comes to carry, in machine form, the verdict of the window and the pointer to the rule
of choice, and `labs --ponte` prints both.

**What gains form is the rule of choice.** It is mechanical by design: an external list with a hash recorded before
the choice, an eligibility filter that uses nothing of the program (public catalogs of replication failure,
correction, and retraction, and cases with successful replication for the clean ones), the declared seed, the proportion of
8 with a defect and 4 clean, and the order coming from the draw itself. The gate of the rule: a list that does not sustain the
requested proportion **rejects the choice** instead of being completed with whatever there is — there is a test for that.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test -p stochology-book` | 37 tests passed, among them the eight of `ponte`: the pilot re-derived, the record that declares what is missing, the thesis sustained without a fair test rejected, the designated roles that release the test, the status `em teste` without the roles rejected, the mechanical and deterministic choice in the proportion, the list that does not sustain the proportion rejected, and the verdict of the window read from the record (0) |
| `labs --ponte` | the pilot re-derived, the five pending roles, the verdict of the window, and the two synthetic proofs (0) |
