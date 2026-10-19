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

MODULES = ["ch02_ch05", "ch05_propagate", "ch05_multiverse", "ch06_ch08", "ch08_stacking", "ch09", "ch10_ch11", "ch12", "ch13", "ch14", "ch14_discrepancy", "ch15_ch21", "ch16_passk", "ch19", "ch20_streams", "recipes"]


FAMILIES = {"anthropic-claude", "human-outside", "human-owner"}


def authors() -> dict[str, tuple[str, str]]:
    out: dict[str, tuple[str, str]] = {}
    for line in (HERE / "AUTHORS.tsv").read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        module, checker, instrument, _note = line.split("\t")
        if checker not in FAMILIES or instrument not in FAMILIES:
            raise SystemExit(f"AUTHORS.tsv: unknown family in the row of {module}")
        out[module] = (checker, instrument)
    return out


def main() -> int:
    families = authors()
    missing = [m for m in MODULES if m not in families]
    if missing:
        print(f"AUTHORS.tsv names no family for: {', '.join(missing)}")
        return 1
    owner: dict[int, str] = {}
    for name in MODULES:
        before = len(ROWS)
        importlib.import_module(name)
        for i in range(before, len(ROWS)):
            owner[i] = name
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
    same = sum(1 for i in range(len(ROWS)) if families[owner[i]][0] == families[owner[i]][1])
    print(
        f"checked by the same family that wrote the instrument: {same} of {len(ROWS)} quantities "
        "(AUTHORS.tsv); their agreement is weak evidence, and item 17 stays open until that count falls"
    )
    return 1 if counts["DIVERGES"] else 0


if __name__ == "__main__":
    sys.exit(main())
