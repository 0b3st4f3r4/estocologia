#!/usr/bin/env python3
"""List occurrences of each prose mannerism with line numbers.

Usage:
    python3 scripts/style/occurrences.py book/ch07.md [pattern ...]
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from scan import PATTERNS  # noqa: E402


def main() -> int:
    path = Path(sys.argv[1])
    selected = sys.argv[2:] or list(PATTERNS)
    lines = path.read_text(encoding="utf-8").splitlines()

    inside = False
    fence = []
    for i, line in enumerate(lines, start=1):
        if line.lstrip().startswith(("```", "~~~")):
            inside = not inside
            fence.append(i)
            continue
        if inside:
            fence.append(i)

    for name in selected:
        pattern = PATTERNS[name]
        found = [(i, l) for i, l in enumerate(lines, start=1)
                 if re.search(pattern, l, re.I) and i not in fence]
        print(f"== {name}: {len(found)}")
        for i, l in found:
            print(f"  {i:4d}: {l.strip()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
