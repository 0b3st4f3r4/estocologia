# The safeguard panel against the failures of teams of agents

Written on 2026-10-19 for item 18 (f) of `docs/DECLARED_PROPOSALS.md`. Written with an AI assistant.

Chapter 16 describes a team of eight specialists, an orchestrator, a six-stage cycle and a panel of eight safeguards
($A_1$ to $A_8$). Cemri and colleagues (2025, arXiv:2503.13657) annotated more than 200 traces of seven open
multi-agent frameworks and grouped their failures into 14 modes in three classes, with an agreement between human
annotators of κ = 0.88; their conclusion is that most failures come from the design of the system and the
coordination of its agents, not from the base model. This audit asks, mode by mode, whether something in the program
would catch the failure, and where.

**Reading of the columns.** *Caught* means a mechanism of the program refuses or counts the failure without anyone
having to notice it. *Partly* means a mechanism exists but checks a declaration, not the behaviour, or depends on
people the program does not have. *Not caught* means nothing in the program looks.

| Mode (Cemri and colleagues) | What would catch it here | Verdict |
| --- | --- | --- |
| FM-1.1 Disobey the task specification | the card carries the decision maker's question and the package action; Φ5 refuses an action coarser than the question's grain; nothing checks that the answer answers the question | partly |
| FM-1.2 Disobey the role specification | the rule that nobody approves the work they produced; nothing checks that a specialist kept to its role | not caught |
| FM-1.3 Step repetition | the counter $K$ counts every attempt and the funnel stops at zero gain of information (τ* = 0) | caught |
| FM-1.4 Loss of the history | the trail of $A_5$, verified by the chain, and the dated records of `history/` | caught |
| FM-1.5 Unaware of the stopping conditions | the e-process ledger (Chapter 19) and the exit clause of $A_8$, both declared before | caught |
| FM-2.1 Conversation reset | a rejection returns to the stage that failed, not to the beginning; nothing detects an unrequested restart | partly |
| FM-2.2 Fail to ask for clarification | stage 1, translation and elicitation; the elicitation needs people outside the program (item 3, open) | partly |
| FM-2.3 Task derailment | the type III lock and the resolution of the question (Φ5) catch a change of grain, not a change of subject | partly |
| FM-2.4 Information withholding | a declared gap is valued above a uniform artifact, but nothing checks that a specialist passed on what it found | not caught |
| FM-2.5 Ignored another agent's input | the cycle sends a rejection back to its stage; nothing checks that the returned work answered the rejection | partly |
| FM-2.6 Reasoning–action mismatch | nothing compares the reasoning of a card with its action | not caught |
| FM-3.1 Premature termination | stopping rules declared before (e-process, funnel, exit clause), and $K$ | caught |
| FM-3.2 No or incomplete verification | the panel checks that each safeguard is declared and anchored, not that it holds under attack (Chapter 16 says so); the gates run some attacks (trail tampering) | partly |
| FM-3.3 Incorrect verification | the reimplementation from the text, the provenance ledger and the authorship record by family; the checker is of the same family as the author (item 17, open) | partly |

**Count.** Caught: 4 (FM-1.3, 1.4, 1.5, 3.1). Partly: 7. Not caught: 3 (FM-1.2, 2.4, 2.6).

**What the count says.** What the program catches is what it **declares before**: attempts, history, stopping. What it
does not catch is what happens **between** the specialists: who kept to a role, who withheld a finding, whose action
departed from its reasoning. Those three need a record of the exchanges, not only of the cards, and a reader of that
record who is not one of the specialists. The panel is a checklist of declarations; the classes where teams of agents
fail most, misalignment between agents and verification, are where it is weakest.

**What would close the three gaps, without implementing them here.** (a) Role: each card records which specialist
wrote each field, and a gate refuses a card whose field was written by a specialist outside its role. (b) Withholding:
every finding a specialist logs enters the trail, and the card lists the findings it did not use, with the reason.
(c) Reasoning–action: the card states the inequality its action rests on (for a decision, $p > C/L$), and a gate
recomputes it from the card's own fields. Each is a change of the contract and needs its own declaration.
