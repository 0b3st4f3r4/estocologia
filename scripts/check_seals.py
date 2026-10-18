#!/usr/bin/env python3
"""Verify all SHA-256 seals across the repository tree from a single command.

Usage:
    python3 scripts/check_seals.py          # all seals, per-file report
    python3 scripts/check_seals.py --quiet  # summary only

Format rule across all 30 seal files: each active line is

    <64 hex>  <path relative to repository root>

Paths are relative to the repository root, not to the seal's directory, so a single command
verifies everything. Lines starting with "#" are historical records of retired artifacts.
A seal file with zero live targets is reported as declared, never silently approved.

Exits with 1 if any live target is missing, diverges, or has an invalid format.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LINE_RE = re.compile(r"^([0-9a-f]{64})  (\S.*)$")
EXTRAS = [
    "docs/STATE_DECLARATION.sha256",
    "data/SHA256SUMS",
    "governance/DECLARACAO.sha256",
]


def find_seals() -> list[Path]:
    found = sorted((ROOT / "empirical").rglob("*.sha256"))
    found += [ROOT / name for name in EXTRAS if (ROOT / name).is_file()]
    return found


def verify_seal(path: Path) -> tuple[int, int, int, list[str]]:
    """Return (live verified, declared out-of-tree, invalid lines, problems)."""
    live = declared = invalid = 0
    problems: list[str] = []
    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.rstrip()
        if not line:
            continue
        if line.startswith("#"):
            declared += 1
            continue
        m = LINE_RE.match(line)
        if not m:
            invalid += 1
            problems.append(f"{path.relative_to(ROOT)}:{lineno}: invalid format: {line[:80]}")
            continue
        expected, rel_path = m.group(1), m.group(2).strip()
        target = ROOT / rel_path
        if not target.is_file():
            problems.append(f"{path.relative_to(ROOT)}:{lineno}: missing target: {rel_path}")
            continue
        actual = hashlib.sha256(target.read_bytes()).hexdigest()
        if actual != expected:
            problems.append(
                f"{path.relative_to(ROOT)}:{lineno}: {rel_path} diverges (seal {expected[:12]}, disk {actual[:12]})"
            )
            continue
        live += 1
    return live, declared, invalid, problems


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--quiet", "--quieto", action="store_true", dest="quiet")
    args = ap.parse_args()

    total_live = total_declared = total_invalid = 0
    without_live: list[str] = []
    problems: list[str] = []
    files = find_seals()
    for path in files:
        live, declared, invalid, found_problems = verify_seal(path)
        total_live += live
        total_declared += declared
        total_invalid += invalid
        problems.extend(found_problems)
        if live == 0:
            without_live.append(str(path.relative_to(ROOT)))
        if not args.quiet:
            status = "ok" if not found_problems and live else ("declared" if not found_problems else "FAIL")
            print(f"{path.relative_to(ROOT)}: {live} targets, {declared} declared out-of-tree — {status}")
    for p in problems:
        print(f"  {p}")
    print(
        f"seals: {len(files)} | targets verified: {total_live} | "
        f"declared out-of-tree: {total_declared} | invalid lines: {total_invalid} | "
        f"problems: {len(problems)}"
    )
    if without_live:
        print("seals without live targets (declared): " + ", ".join(without_live))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
