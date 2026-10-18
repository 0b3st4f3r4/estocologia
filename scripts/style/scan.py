#!/usr/bin/env python3
"""Mechanical scan of prose mannerisms in the Stochology textbook.

Usage:
    python3 scripts/style/scan.py                     # all .md files in book/
    python3 scripts/style/scan.py book/ch07.md ...    # selected files
    python3 scripts/style/scan.py --json /tmp/out.json
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent

# The patterns audited in the Portuguese textbook. The keys keep their Portuguese names on purpose: they name
# mannerisms of Portuguese prose, they are the column headers of the table in scripts/style/repetition-index.md,
# and the JSON written below is consumed by scripts/style/update_index.py. The regexes themselves are the
# measurement, so they are reproduced here as they are; only the reading of each key is documented.
PATTERNS = {
    # em dash used as an appositive aside ("o modelo — e não a média — decide")
    "travessao_apostivo": r" — ",
    # em dash clause that closes with "e", the fault of a dash doing the work of a conjunction
    "travessao_fecha_com_e": r"—\s*e\b",
    # "mesmo/mesma" as filler instead of comparison
    "mesmo_mesma": r"\bmesm[ao]s?\b",
    # "o mesmo" as a connective between sentences
    "o_mesmo": r"\bo mesmo\b",
    # relative "o que" dangling after a noun
    "o_que_relativo": r"\bo que\b",
    # comma before "não", the pause that hands the sentence to the negation
    "virgula_nao": r",\s*não\b",
    # the "não é X, é Y" antithesis, the mannerism that turns a definition into a correction
    "nao_e_X_e_Y": r"não (?:é|foi|são)\s[^.,;]{1,40},\s*(?:é|foi|são|mas)\b",
    # colon in running prose (colons inside tables and fences are separated by scripts/style/colons.py)
    "dois_pontos": r":",
    # "próprio/própria" used as an intensifier
    "proprio": r"\bprópri[ao]s?\b",
    # "exatamente" used as an intensifier
    "exatamente": r"\bexatamente\b",
}

FENCE_RE = re.compile(r"(?:\x60\x60\x60|~~~).*?(?:\x60\x60\x60|~~~)", re.DOTALL)


def prose(text: str) -> str:
    """Prose only: code fences and their bodies are removed before counting."""
    return FENCE_RE.sub("", text)


def measure(path: Path) -> tuple[int, collections.Counter]:
    text = prose(path.read_text(encoding="utf-8"))
    words = len(re.findall(r"\w+", text))
    counts = collections.Counter({k: len(re.findall(p, text, re.I)) for k, p in PATTERNS.items()})
    return words, counts


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="*")
    ap.add_argument("--json", type=Path, help="write counts to JSON")
    ap.add_argument("--no-header", "--sem-cabecalho", action="store_true", dest="no_header")
    args = ap.parse_args()

    if args.files:
        paths = [Path(a) for a in args.files]
    else:
        paths = [
            Path(p)
            for p in sorted(glob.glob(str(ROOT / "book" / "*.md")))
            if Path(p).name != "stochology-textbook.md"
        ]

    total: collections.Counter = collections.Counter()
    per: dict[str, dict] = {}
    total_words = 0
    for path in paths:
        words, counts = measure(path)
        total_words += words
        total.update(counts)
        per[path.name] = {"words": words, **dict(counts)}

    if not args.no_header:
        print(f"words (outside code fences): {total_words}")
        for k, v in total.most_common():
            print(f"{v:6d}  {k}")
        print()
        for name, d in per.items():
            dense = ", ".join(f"{k}={v}" for k, v in sorted(d.items(), key=lambda x: -x[1]) if k != "words" and v)
            print(f"{name:42s} {d['words']:6d} words | {dense}")

    if args.json:
        args.json.write_text(
            json.dumps({"words": total_words, "total": dict(total), "per_file": per},
                       ensure_ascii=False, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        print(f"wrote {args.json}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
