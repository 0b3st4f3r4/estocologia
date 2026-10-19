#!/usr/bin/env python3
"""Verify that every output block cited in the book matches the literal stdout of a Rust instrument.

Usage: python3 scripts/book/check_labs.py
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
BOOK_DIR = ROOT / "book"
BIN = ROOT / "target" / "release" / "labs"
BLOCK_RE = re.compile(r"(?:\x60\x60\x60|~~~)text\n(.*?)(?:\x60\x60\x60|~~~)", re.DOTALL)


def run_output(cmd: list[str]) -> list[str]:
    proc = subprocess.run(cmd, capture_output=True, text=True, cwd=ROOT)
    return [l.rstrip() for l in (proc.stdout + proc.stderr).splitlines()]


def contains_slice(needle: list[str], haystack: list[str]) -> bool:
    n = len(needle)
    if n == 0:
        return True
    return any(haystack[i : i + n] == needle for i in range(len(haystack) - n + 1))


def main() -> int:
    if not BIN.exists():
        print(f"missing instrument: {BIN} (run: cargo build --release --bin labs)")
        return 1

    instruments: list[tuple[str, list[str]]] = [
        ("labs --bank", run_output([str(BIN), "--bank"])),
        ("labs --resolution", run_output([str(BIN), "--resolution"])),
        ("labs --s1", run_output([str(BIN), "--s1"])),
        ("labs --s2", run_output([str(BIN), "--s2"])),
        ("labs --queue", run_output([str(BIN), "--queue"])),
        ("labs --genetics", run_output([str(BIN), "--genetics"])),
        ("labs --streams", run_output([str(BIN), "--streams"])),
        ("labs --eprocess", run_output([str(BIN), "--eprocess"])),
        ("labs --listening", run_output([str(BIN), "--listening"])),
        ("labs --h3", run_output([str(BIN), "--h3"])),
        ("labs --bridge", run_output([str(BIN), "--bridge"])),
        ("labs --schema", run_output([str(BIN), "--schema"])),
        ("labs --second-opinion", run_output([str(BIN), "--second-opinion"])),
        ("labs --jev", run_output([str(BIN), "--jev"])),
    ]

    chapter_paths = sorted(BOOK_DIR.glob("ch*.md")) or sorted(BOOK_DIR.glob("cap*.md"))
    total = 0
    failures = 0
    for path in chapter_paths:
        ch_num = int(re.sub(r"\D", "", path.stem))
        blocks = BLOCK_RE.findall(path.read_text(encoding="utf-8"))
        if not blocks:
            continue
        ch_out = run_output([str(BIN), "--chapter", str(ch_num)])
        for idx, block in enumerate(blocks, start=1):
            total += 1
            needle = [l.rstrip() for l in block.strip("\n").splitlines()]
            where = f"labs --chapter {ch_num}"
            if contains_slice(needle, ch_out):
                print(f"{path.stem} block {idx}: matches {where} ({len(needle)} lines)")
                continue
            found = next((name for name, inst_out in instruments if contains_slice(needle, inst_out)), None)
            if found:
                print(f"{path.stem} block {idx}: matches {found} ({len(needle)} lines)")
                continue
            failures += 1
            print(f"{path.stem} block {idx}: DIVERGES from all instruments")
            for a, b in zip(needle, ch_out):
                if a != b:
                    print(f"   in text : {a}")
                    print(f"   measured: {b}")
                    break

    print(f"blocks verified: {total}, divergent: {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
