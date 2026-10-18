# Declaration of Loss: The Three Reports That Were Never in Git

Record of 2026-10-17. This document exists because two files gave a recovery command that could never work, and
a printed command is a promise: anyone reading it expects the file to return with a `git checkout`.

## 1. What the Documents Claimed

| Document | Location | What it stated |
| --- | --- | --- |
| `docs/DECLARED_PROPOSALS.md` | §6, table | `book/revisao.md`, Section 5, now points to `docs/declarations/RUST_CONSOLIDATION.md` and `docs/EMPIRICAL_REGISTRY.md` |
| `docs/DECLARED_PROPOSALS.md` | §145, pending item 2 | "the pointers in §6 do not resolve", with `git checkout 1bfec905 -- book/revisao.md book/resposta-a-revisao.md book/resultados-que-dependem-de-python.md` |
| `docs/declarations/RUST_CONSOLIDATION.md` | §"How to Recover" | the same `git checkout`, with the three files at the end of the list after `h0-real/ h1-real/ h2/ h2g/ h3/ empirical/` |

## 2. What the Tree Answers

~~~text
$ git cat-file -e 1bfec905:book/revisao.md
fatal: path 'book/revisao.md' does not exist in '1bfec905'
$ git ls-tree -r --name-only 1bfec905 | wc -l
1314
$ git ls-tree -r --name-only 1bfec905 | grep -c '^book/'
0
~~~

Commit `1bfec905` — "Simplificação: o que é gerado por script sai do Git (armazém, cópias do p03, saída_base do
p09)" — contains 1 314 files and **none** under `book/`. The six folders requested on the same line do exist there:

| Requested path | Files in `1bfec905` |
| --- | --- |
| `h0-real/` | 39 |
| `h1-real/` | 47 |
| `h2/` | 59 |
| `h2g/` | 89 |
| `h3/` | 96 |
| `empirical/` | 579 |
| `book/revisao.md` | **0** |

A full scan of the repository history confirms the rest: out of 94 commits, none contain `book/revisao.md`, and
`git log --all --diff-filter=A -- book/revisao.md` returns nothing — the file was never added. The `book/`
directory entered Git in `64ef89a` ("Poda: andaime Python e cópias geradas saem do disco; workspace Rust entra"),
which is a descendant of `1bfec905`:

~~~text
$ git merge-base --is-ancestor 1bfec905 64ef89a && echo ancestral
ancestral
~~~

## 3. What the Removal Manifests List

The three removal manifests were searched for any path under `book/`:

~~~text
$ grep -c 'book/' history/registro-python-removido.txt
0
$ grep -n 'revisao\|resposta-a\|dependem' history/registro-python-removido.txt history/python-removido.txt history/nao-significante-removido.txt
(no lines)
~~~

In short: the three reports are neither in the retired Python manifest, nor in the non-significant material
manifest, nor in the registry of items without theoretical significance. There is no blob hash for them anywhere, and
therefore nothing to restore by hash.

## 4. The Verdict

**The three reports do not exist in this tree or in any snapshot of the versioned history, and have no recorded
blob.** Two readings remain possible, and the record does not choose between them on its own: either they were never
versioned under those paths, or they were versioned prior to the initial commit of this tree and left together with the
material discarded during consolidation. What the tree proves is their absence; the cause is declared
indeterminate rather than invented.

## 5. What Stands in Their Place

| What was sought | Where it lives today |
| --- | --- |
| the review of the numbers, `book/revisao.md` | `docs/EMPIRICAL_REGISTRY.md`, with the R, D, or P tag of each number, and `docs/declarations/RUST_CONSOLIDATION.md` |
| triage of results that depended on Python | `book/significant-results.md` |
| response to the review | the chapters and `governance/` |
| what left and how to recover it | `docs/declarations/RUST_CONSOLIDATION.md` §"What Left" and §"How to Recover", now without the phantom path |

## 6. What Changed in the Documents

1. `docs/DECLARED_PROPOSALS.md` §145 dropped the command, and the pending item became a record of loss.
2. `docs/declarations/RUST_CONSOLIDATION.md` dropped `book/revisao.md` from the `git checkout` line and gained the declared
   boundary with the commit file count.
3. `docs/DECLARED_PROPOSALS.md` §6 gained the note that the old pointer has no target in the repository history.

## 7. What Did Not Change

No result number changed. What changed was the recovery promise, which now matches what Git actually supports.
