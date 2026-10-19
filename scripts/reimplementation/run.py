#!/usr/bin/env python3
"""Reimplementation of the textbook's laboratories from the text alone (docs/OUTSIDE_AUDIT.md, activity 2).

Each module recomputes, in Python with the standard library, the quantities a chapter prints, from the
specification the chapter states in words, formulas and data files, without reading the Rust of that chapter's
laboratory. The comparison is with the value the book prints. See common.py for the three verdicts.

Declared limit: written by an AI assistant (2026-10-19), of the same kind as the readings of 2026-10-18. A
cross-check, not the independent reimplementation by people that item 17 asks for.

Usage:
    python3 -I scripts/reimplementation/run.py            # summary; exit 1 on any divergence
    python3 -I scripts/reimplementation/run.py --all      # every row
"""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from common import ROWS, verdict  # noqa: E402

MODULES = ["ch02_ch05", "ch06_ch08", "ch09", "ch10_ch11", "ch12", "ch13", "ch14", "ch15_ch21", "ch19", "recipes"]


def main() -> int:
    for name in MODULES:
        importlib.import_module(name)
    show_all = "--all" in sys.argv
    counts = {"agrees": 0, "DIVERGES": 0, "open": 0}
    print(f"{'ch':<4} {'quantity':<62} {'book':>11} {'reimpl.':>11}  verdict")
    for row in ROWS:
        v = verdict(row)
        counts[v] += 1
        if show_all or v != "agrees":
            ours = "—" if v == "open" else f"{row['ours']:.6g}"
            print(f"{row['chapter']:<4} {row['what']:<62} {row['book']:>11.6g} {ours:>11}  {v} ({row['kind']})")
    by_chapter: dict[str, int] = {}
    for row in ROWS:
        by_chapter[row["chapter"]] = by_chapter.get(row["chapter"], 0) + 1
    chapters = ", ".join(f"{k}: {v}" for k, v in sorted(by_chapter.items(), key=lambda kv: int(kv[0])))
    print(f"\n{len(ROWS)} quantities ({chapters})")
    print(f"agree {counts['agrees']} | diverge {counts['DIVERGES']} | open {counts['open']}")
    return 1 if counts["DIVERGES"] else 0


if __name__ == "__main__":
    sys.exit(main())
