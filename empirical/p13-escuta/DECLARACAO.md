# Declaration of point 13 (listening): the seal of the box and the exit clause

**Frozen on 2026-10-16T21:19:42Z**, before any attempt at contact with any group.

| file | SHA-256 | what it is |
| --- | --- | --- |
| `protocolo.md` | `fee645912c789b4ab8a0d4ba5585e0f36d4f41eb23c4fd82d56a0191d2fd4b02` | the protocol, frozen: from here on it is not rewritten |
| `registro.tsv` | `646f880ad35f53d6289bf4f18e855966fec9e844279e17d8a2ab432ba8417f47` | the machine record of the seven decisions, of whose the error is and who was heard |
| `DECLARACAO.md` (this one) | — | this document; its own hash comes out in `DECLARACAO.sha256`, computed after the writing |
| `DECLARACAO.sha256` | — | the three hashes, in `sha256sum` form |

## Declaration 1 — the seven historical cohorts are not within reach

Declared by the owner of the program: **none of the seven historical groups is within reach today.** The cohorts are from
decades ago — STAR children of the 1980s, RAND HIE families of the 1970s, Toronto detainees of
1997–2002, Toronto arrestees of 2020–2021, Rhode Island drivers of 2005–2015, injured workers of
Kentucky and Michigan of the 1980s, NSW participants and CPS controls of the 1970s —, and direct contact is
unfeasible because of the temporal, legal, ethical, and anonymity barrier.

The program does not invent a record of a consultation that did not occur. The column "heard" of
[`governance/GRUPOS.md`](../../governance/GRUPOS.md) remains **none** in the seven lines, and remains so while
there is no recorded listening. What is sought, from here on, is a **structural equivalent**: who today occupies the
same position as those who paid for the error then.

## Declaration 2 — the seal, the minimum coverage, and the deadline

Measured by the instrument that reads this record, at the moment of the freezing:

~~~bash
cargo run --release --bin labs -- --escuta
~~~

~~~text
== Ponto 13 :: a escuta de quem paga o erro ==
decisoes: 7 | carimbo do registro: 2026-10-16 | prazo da janela: 2026-12-15 | cobertura minima: 2 de 7
decisao                              titulares ouvidos      registro   pode emitir
h2g/saida/direito_teto_beneficio             - nenhum       -          nao
h2g/saida/educacao_star_turmas           11598 nenhum       -          nao
h2g/saida/saude_rand                         - nenhum       -          nao
h1-real/saida/b                          16177 nenhum       -          nao
h3/prisoes                                5226 nenhum       -          nao
h3/prisoes20                             65276 nenhum       -          nao
h3/prisoes_ri                            86113 nenhum       -          nao

cobertura de escuta: 0 de 7 grupos ouvidos
divergencias: nenhuma — nenhuma decisao se diz emitida sem ninguem ouvido
veredito: CAIXA LACRADA — a cobertura de 0 fica abaixo do minimo de 2; as decisoes ficam como analise de sensibilidade (n = 1) e nao viram diretriz de politica publica nem decisao de portao F8 ate 2026-12-15 ou ate a cobertura minima
~~~

| what the seal decides | declared value | where it comes from |
| --- | --- | --- |
| listening coverage | 0 of 7 | `registro.tsv`, column "heard" |
| minimum coverage to open the box | 2 of the 7 groups, with a signed record | `registro.tsv`, key `grupos_minimos` |
| deadline of the window | 60 calendar days, until **2026-12-15** | `registro.tsv`, key `prazo` |
| divergent decisions (issued with nobody heard) | 0 | the gate of the instrument |
| verdict | **box sealed** | coverage below the minimum |

**What the seal does.** The seven decisions stay as a sensitivity analysis ($n = 1$) and **do not** become a guideline of
public policy nor an F8 gate decision until the minimum coverage is reached. If the deadline expires without the coverage, the
artifact stays archived as a methodological exercise of sensitivity, and the seal has a hash. If the coverage is
reached, the covered lines leave the seal and the records enter `governance/GRUPOS.md`.

**The smoke producer.** The numbers of the seal stay in the machine record, and the instrument that reads them **knows how to
reject**: a decision that claims to be issued with the column "heard" as none is a divergence, and the box only opens when the
coverage reaches the minimum. The two synthetic cases go in the report of the instrument, and the tests of the module hold
both.

## Declaration 3 — who conducts and the script

The listening **is not conducted by the owner of the program**: the priors and the adjustments came from the same hand. One of the two
forms is worth, and the choice between them enters its own declaration **before** the first conversation:

1. neutral or institutional facilitator, with no link to the program; or
2. blind protocol: whoever interviews receives only the script, without the numbers of the funnel, without the prior, and without knowing which
   decision is behind the conversation.

The script has three questions, in plain language, without the vocabulary of the program:

1. *What is your worst scenario that is still plausible?*
2. *What would make you refuse this intervention?*
3. *When this policy fails, who in fact bears the cost?*

What the group said is not translated into the vocabulary of the program by whoever conducts. Translation, if there is one, is made
separately and stays marked as a reading of the program.

## Declaration 4 — the voice of the operator does not count

The voice of whoever models is already entirely in the priors and in the adjustments. The operator projects the rule; the group suffers the
consequence. Recording the operator in the column "heard" would be a structural conflict of interest and would annul the front.
**The operator does not count as a group heard in any of the seven decisions**, and this declaration is what prevents the
count from going up on its own.

## Declaration 5 — what this front does not change

No official round changes prior because of this declaration: the funnel continues with the prior of the package and the five
elicited rounds of E1 remain as sensitivity with $n = 1$, as the protocol of SHELF already said. Chapter 21
gained the paragraph of the seal and the block of the instrument; no other number of the book changed.

## Verification

Run on 2026-10-16, after the freezing. Exit code in parentheses.

| Command | Result |
| --- | --- |
| `cargo test --workspace` | 219 tests passed, none failed, with the 5 of the listening gate (0) |
| `cargo test -p stochology-book` | 28 tests passed, among them `o_registro_do_programa_tem_sete_decisoes_e_nenhum_ouvido` and `a_decisao_emitida_sem_ninguem_ouvido_reprova` (0) |
| `labs --escuta` | 7 decisions, 0 of 7 heard, 0 divergences, box sealed with deadline on 2026-12-15, plus the two synthetic proofs (0) |
| `python3 scripts/book/check_labs.py` | 26 cited blocks, 0 divergent — block 2 of Chapter 21 checks in `labs --escuta` (0) |
| `python3 scripts/book/generate_results.py --check` | `docs/RUST_RESULTS.md` updated, with the new block of the seal (0) |
