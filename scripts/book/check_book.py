#!/usr/bin/env python3
"""Verify code fence balance, part order, and paper numbers in the assembled textbook."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
default_book = ROOT / "book" / "stochology-textbook.md"
path = Path(sys.argv[1]) if len(sys.argv) > 1 else default_book
lines = path.read_text(encoding="utf-8").splitlines()

opened_at = None
problems = []
for i, line in enumerate(lines, start=1):
    if line.startswith("```"):
        if opened_at is None:
            opened_at = i
        else:
            opened_at = None
if opened_at is not None:
    problems.append(f"unclosed code fence opened at line {opened_at}")

parts = [l for l in lines if l.startswith("# ")]
expected_pt = [
    "# Estocologia: livro didático",
    "# Parte I.",
    "# Parte II.",
    "# Parte III.",
    "# Parte IV.",
    "# Parte V.",
    "# Apêndices",
]
expected_en = [
    "# Stochology: A Textbook",
    "# Part I.",
    "# Part II.",
    "# Part III.",
    "# Part IV.",
    "# Part V.",
    "# Appendices",
]
use_en = bool(parts and parts[0].startswith("# Stochology"))
expected = expected_en if use_en else expected_pt

if len(parts) != len(expected):
    problems.append(f"level-1 headings: {len(parts)}, expected {len(expected)}")
else:
    for a, b in zip(parts, expected):
        if not a.startswith(b):
            problems.append(f"part order: {a!r} instead of {b!r}")

ch_prefix = "## Chapter " if use_en else "## Capítulo "
chapters = [l for l in lines if l.startswith(ch_prefix)]
if len(chapters) != 21:
    problems.append(f"chapters: {len(chapters)}, expected 21")
nums = [int(l.split()[2].rstrip(".")) for l in chapters]
if nums != list(range(1, 22)):
    problems.append(f"chapter numbering: {nums}")

markers = (
    ["### Step 1", "### Step 2", "### Step 3", "**Exercises.**", "**Summary.**"]
    if use_en
    else ["### Degrau 1", "### Degrau 2", "### Degrau 3", "**Exercícios.**", "**Resumo.**"]
)
for marker in markers:
    n = sum(1 for l in lines if l.startswith(marker))
    if n != 21:
        problems.append(f"{marker}: {n} occurrences, expected 21")

paper = ROOT / "paper" / "README.md"
bin_labs = ROOT / "target" / "release" / "labs"
if paper.is_file() and bin_labs.is_file() and not use_en:
    paper_text = paper.read_text(encoding="utf-8")

    def run_labs(*flags: str) -> str:
        return subprocess.check_output([str(bin_labs), *flags], text=True, cwd=ROOT)

    # An instrument that refuses the invocation is a problem of the same weight as a diverging number:
    # it is reported, not raised, so the rest of the verification list still runs.
    try:
        s_cap5 = run_labs("--chapter", "5")
        s_cap16 = run_labs("--chapter", "16")
        s_cap17 = run_labs("--chapter", "17")
        s_cap18 = run_labs("--chapter", "18")
        s_h3 = run_labs("--h3")
        s_ponte = run_labs("--bridge")
        # Each number is re-derived from the instrument and then looked up in paper/README.md, in both the
        # decimal-comma and the decimal-point spelling.
        recomputed = [
            ("cap16 constant wait", re.search(r"8 every day \(the claim\)\s+\d+\s+([0-9.]+)", s_cap16).group(1).replace(".", ",")),
            ("cap16 burst wait", re.search(r"5 or 11\s+\d+\s+([0-9.]+)", s_cap16).group(1).replace(".", ",")),
            ("cap16 severe diff", re.search(r"observed difference = ([0-9.]+) min", s_cap16).group(1).replace(".", ",")),
            ("cap05 burst fraction", re.search(r"\(([0-9.]+%) of the total\)", s_cap5).group(1).replace(".", ",")),
            ("cap05 failed exam", re.search(r"failing \(score < 6\.0\) = ([0-9.]+) per 100", s_cap5).group(1).replace(".", ",") + "%"),
            ("cap17 3x median", f"{float(re.search(r'^3\s+\d+\s+([0-9.]+)\s+([0-9.]+)', s_cap17, re.M).group(1)):.1f}".replace(".", ",")),
            ("cap17 3x mean", f"{float(re.search(r'^3\s+\d+\s+([0-9.]+)\s+([0-9.]+)', s_cap17, re.M).group(2)):.0f} times"),
            ("cap17 LaLonde RCT", f"+{float(re.search(r'randomised experiment \(NSW treat\. - NSW ctrl\)\s+([0-9.]+)', s_cap17).group(1)) * 100:.1f}".replace(".", ",")),
            ("cap17 LaLonde raw", f"−{float(re.search(r'raw comparison \(NSW treat\. - CPS\)\s+-([0-9.]+)', s_cap17).group(1)) * 100:.1f}".replace(".", ",")),
            ("cap18 Laplace clean window", re.search(r"or (1 chance in \d+) per month", s_cap18).group(1)),
            ("h3 naive bias", f"{float(re.search(r'bias of the naive forecaster against exogenous demand: ([0-9.]+)', s_h3).group(1)) * 100:.0f}%"),
            ("bridge blind pilot", re.search(r"flagged the defect: arm X (\d+ of \d+)", s_ponte).group(1)),
        ]
    except subprocess.CalledProcessError as err:
        problems.append(f"labs refused the invocation: {' '.join(err.cmd[1:])} exited with {err.returncode}")
        recomputed = []
    for origin, value in recomputed:
        if value not in paper_text and value.replace(",", ".") not in paper_text:
            problems.append(f"paper/README.md diverges from {origin}: expected {value!r}")

if problems:
    print("problems in assembled book:")
    for p in problems:
        print(f"  - {p}")
    sys.exit(1)
print(f"assembled book ok: {len(lines)} lines, {len(chapters)} chapters, 5 parts, balanced code fences")
