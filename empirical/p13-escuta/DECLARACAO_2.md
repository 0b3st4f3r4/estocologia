# Declaration 2 of point 13 (listening): the choice of the conduction — blind protocol

**Frozen on 2026-10-16T21:35:00Z**, before any attempt at contact with any group. The
[`DECLARACAO.md`](DECLARACAO.md) stays as it is: this second declaration lives alongside it, which is what the
protocol of the front orders for the choice of section 3.

| file | before | now | why |
| --- | --- | --- | --- |
| `registro.tsv` | `646f880ad35f53d6289bf4f18e855966fec9e844279e17d8a2ab432ba8417f47` | `353a7638b1d7cfd77f61ecd97409881c7399b7477a50b5c6186693ccc1d09dde` | the line `# conducao` entered with the choice of this declaration |
| `protocolo.md` | `fee645912c789b4ab8a0d4ba5585e0f36d4f41eb23c4fd82d56a0191d2fd4b02` | the same | the protocol is not rewritten |
| `DECLARACAO.md` | `3631bb81ef17b2e3e04b521bd6e2a08e510a4b974881f08c35e40f17d047a51b` | the same | it stays as the record of the freezing |
| `DECLARACAO_2.md` (this one) | — | — | the choice; its own hash comes out in `DECLARACAO.sha256`, computed after the writing |

## Declaration 6 — the conduction is the blind protocol (form B)

The protocol left two forms open and ordered the choice to be declared before the first conversation. **The choice is
form B: blind protocol.** The declared reason: the operator is who built the adjustments and the priors, and a neutral or
institutional facilitator **is not hired in the current scope**. Since there is no third party to conduct, neutrality
comes to depend on the design, and not on the person.

What "blind" means, in five rules that hold for every conversation:

1. **Only the script of section 4 of the protocol enters the conversation.** The three questions, in order, without adding anything.
2. **No number of the program enters.** Neither quantile, nor median, nor distribution, nor Brier, nor EVPE/EVPI, nor
   the outputs of the funnels F1 to F8, nor the name of the domain. The interviewee speaks first, and the program listens afterwards.
3. **Whatever is asked as a number is answered with "that is not part of this conversation"**, and the question stays
   recorded as a question of the interviewee.
4. **The answer is recorded literally, in natural frequencies** ("out of every 100 people, how many"), with date, place, and
   recorded consent. Nothing is corrected, summarized, or rounded by whoever conducts.
5. **The translation, if there is one, is made separately and marked as a reading of the program**, never inside the record of the group.

The operator conducts, and the conduction is **declared**: he is an interested party and cannot become the voice of the group
(declaration 4). The risk that remains is that of anchoring in the question, and it is that risk that the fixed script and the absence of numbers
reduce; the risk that remains after that stays recorded here, instead of hidden.

## Declaration 7 — the rectification of the record, declared with the two hashes

The `registro.tsv` gained a comment line, `# conducao protocolo cego`, so that the instrument prints the
conduction together with the seal. The two hashes stay side by side in the table above: the first is that of the freezing of
2026-10-16T21:19:42Z, recorded in the `DECLARACAO.md`; the second is that of this act. It is not a change of criterion — it is the choice
that the protocol left open and ordered to be declared before the first contact —, and for that reason **it does not enter the K of the
program**.

**A reading note.** The block of the instrument transcribed in the `DECLARACAO.md` is that of the measurement of
2026-10-16T21:19:42Z: its opening line does not carry the field `conducao`, which only came to exist with this
rectification. The two stay as they are, and what holds from here on is the line with `conducao: protocolo cego` — the
same one that Chapter 21 of the book transcribes. Nothing else changes: the object (the seven decisions), the minimum coverage of
2 of 7, the deadline of 2026-12-15, the rule of no substitution, and the exclusion of the voice of the operator remain as they were.

## Verification

Run on 2026-10-16, after the choice. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `labs --escuta` | the first line now carries `conducao: protocolo cego` alongside the deadline and the minimum coverage; 7 decisions, 0 of 7 heard, and the box sealed remain (0) |
| `cargo test -p stochology-book` | the test `a_conducao_declarada_e_protocolo_cego` holds the new line in the record (0) |
| `sha256sum -c DECLARACAO.sha256` | the four files match the updated hashes (0) |
