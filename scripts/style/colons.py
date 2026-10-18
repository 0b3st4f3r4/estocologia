#!/usr/bin/env python3
"""Separate prose colons from table and code-fence colons.

Usage:
    python3 scripts/style/colons.py               # all textbook files
    python3 scripts/style/colons.py book/ch02.md  # with listing of prose lines
"""

from __future__ import annotations

import glob
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent


def count_colons(path: Path) -> tuple[int, int, int, list[tuple[int, str]]]:
    inside = False
    prose = table = fence = 0
    lines: list[tuple[int, str]] = []
    for i, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if line.lstrip().startswith(("```", "~~~")):
            inside = not inside
            fence += line.count(":")
            continue
        if inside:
            fence += line.count(":")
        elif line.startswith("|"):
            table += line.count(":")
        else:
            n = line.count(":")
            prose += n
            if n:
                lines.append((i, line.strip()))
    return prose, table, fence, lines


def main() -> int:
    targets = [Path(a) for a in sys.argv[1:]] or [
        Path(p)
        for p in sorted(glob.glob(str(ROOT / "book" / "*.md")))
        if Path(p).name != "stochology-textbook.md"
    ]
    print(f"{'file':44s} {'prose':>6} {'table':>7} {'fence':>6}")
    for path in targets:
        prose, table, fence, lines = count_colons(path)
        print(f"{path.name:44s} {prose:6d} {table:7d} {fence:6d}")
        if sys.argv[1:]:
            for i, l in lines:
                print(f"  {i:4d}: {l[:150]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
