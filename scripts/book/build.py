#!/usr/bin/env python3
"""Assemble the 21-chapter textbook from book/.

Usage:
    python3 scripts/book/build.py --check       # verify ramp, assembled file on disk, and declared counts
    python3 scripts/book/build.py               # write book/stochology-textbook.md
    python3 scripts/book/build.py --output x.md # write to a custom path
"""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
BOOK_DIR = ROOT / "book"
DEFAULT_OUTPUT = BOOK_DIR / "stochology-textbook.md"

CANONICAL_DOCS = [
    "README.md",
    "docs/PLAN.md",
    "docs/STATE_DECLARATION.md",
    "docs/RUST_RESULTS.md",
]

COUNT_RE = re.compile(
    r"(?P<lines>\d[\d\s.,]*)\s*(?:linhas|lines)[^.;:\n]{0,60}?(?P<words>\d[\d\s.,]*)\s*(?:palavras|words)"
)


def chapter_files() -> list[Path]:
    if (BOOK_DIR / "ch01.md").exists():
        return [BOOK_DIR / f"ch{n:02d}.md" for n in range(1, 22)]
    return [BOOK_DIR / f"ch{n:02d}.md" for n in range(1, 22)]


def opening_file() -> Path:
    if (BOOK_DIR / "00-opening.md").exists():
        return BOOK_DIR / "00-opening.md"
    return BOOK_DIR / "00-opening.md"


def appendices_file() -> Path:
    if (BOOK_DIR / "appendices.md").exists():
        return BOOK_DIR / "appendices.md"
    return BOOK_DIR / "appendices.md"


def verify_chapters() -> list[str]:
    problems: list[str] = []
    opening = opening_file()
    if not opening.exists():
        problems.append(f"missing {opening.relative_to(ROOT)}")
    for path in chapter_files():
        if not path.exists():
            problems.append(f"missing {path.relative_to(ROOT)}")
            continue
        text = path.read_text(encoding="utf-8")
        num = int(re.sub(r"\D", "", path.stem))
        if f"## Capítulo {num}." not in text and f"## Chapter {num}." not in text:
            problems.append(f"{path.name}: missing chapter {num} heading")
        markers_pt = ["### Degrau 1", "### Degrau 2", "### Degrau 3", "**Exercícios.**", "**Resumo.**"]
        markers_en = ["### Step 1", "### Step 2", "### Step 3", "**Exercises.**", "**Summary.**"]
        use_en = "### Step 1" in text or "## Chapter " in text
        for marker in (markers_en if use_en else markers_pt):
            if marker not in text:
                problems.append(f"{path.name}: missing {marker!r} (3-step ramp)")
        words = len(re.findall(r"\w+", text))
        if words < 700:
            problems.append(f"{path.name}: {words} words (minimum 700)")
        if re.search(r"\[cite:", text):
            problems.append(f"{path.name}: leftover citation marker [cite:")
    appendices = appendices_file()
    if not appendices.exists():
        problems.append(f"missing {appendices.relative_to(ROOT)}")
    return problems


def assemble() -> str:
    parts = [opening_file().read_text(encoding="utf-8").rstrip()]
    for path in chapter_files():
        parts.append(path.read_text(encoding="utf-8").rstrip())
    parts.append(appendices_file().read_text(encoding="utf-8").rstrip())
    return "\n\n".join(parts) + "\n"


def verify_output_on_disk(text: str, target: Path) -> list[str]:
    if not target.exists():
        return [f"{rel_label(target)} does not exist on disk"]
    on_disk = target.read_text(encoding="utf-8")
    if on_disk == text:
        return []
    diff = list(
        difflib.unified_diff(
            text.splitlines(keepends=True),
            on_disk.splitlines(keepends=True),
            fromfile=f"assembled ({len(text.splitlines())} lines)",
            tofile=f"{rel_label(target)} ({len(on_disk.splitlines())} lines)",
            n=1,
        )
    )
    problems = [
        f"{rel_label(target)} diverges from assembled text: "
        f"{sum(1 for l in diff if l.startswith('+') and not l.startswith('+++'))} extra lines on disk, "
        f"{sum(1 for l in diff if l.startswith('-') and not l.startswith('---'))} assembled lines missing"
    ]
    problems.extend(l.rstrip("\n") for l in diff[:40])
    if len(diff) > 40:
        problems.append(f"... and {len(diff) - 40} more diff lines")
    return problems


def parse_number(raw: str) -> int:
    return int(re.sub(r"[\s.,]", "", raw))


def verify_declared_counts(text: str) -> list[str]:
    measured_lines = len(text.splitlines())
    measured_words = len(re.findall(r"\w+", text))
    problems: list[str] = []
    for name in CANONICAL_DOCS:
        path = ROOT / name
        if not path.is_file():
            problems.append(f"{name}: canonical document missing from tree")
            continue
        found = 0
        for lineno, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            for match in COUNT_RE.finditer(line):
                found += 1
                declared = (parse_number(match.group("lines")), parse_number(match.group("words")))
                if declared != (measured_lines, measured_words):
                    problems.append(
                        f"{name}:{lineno}: declares {declared[0]} lines and {declared[1]} words; "
                        f"assembled text has {measured_lines} lines and {measured_words} words"
                    )
        if not found:
            problems.append(
                f"{name}: no declared count ('N lines ... N words') found in canonical document"
            )
    return problems


def rel_label(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(ROOT))
    except ValueError:
        return str(path)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--output", "--saida", type=Path, default=DEFAULT_OUTPUT, dest="output")
    ap.add_argument("--force", "--forcar", action="store_true", dest="force")
    args = ap.parse_args()

    problems = verify_chapters()
    if args.check:
        if problems:
            print("book incomplete or out of ramp:")
            for p in problems:
                print(f"  - {p}")
            return 1
        text = assemble()
        print(f"book ready: {len(text.splitlines())} lines, {len(re.findall(r'\w+', text))} words")
        divergences = verify_output_on_disk(text, DEFAULT_OUTPUT)
        if divergences:
            print(f"disk does not match assembled book ({rel_label(DEFAULT_OUTPUT)}):")
            for d in divergences:
                print(d)
            print("rebuild with: python3 scripts/book/build.py")
            return 1
        print(f"{rel_label(DEFAULT_OUTPUT)} matches assembled text character by character")
        counts = verify_declared_counts(text)
        if counts:
            print("declared count does not match assembled count:")
            for c in counts:
                print(f"  - {c}")
            return 1
        print(
            f"declared count matches across {', '.join(CANONICAL_DOCS)}: "
            f"{len(text.splitlines())} lines and {len(re.findall(r'\w+', text))} words"
        )
        return 0

    if problems:
        print("cannot write assembled book while problems remain:")
        for p in problems:
            print(f"  - {p}")
        return 1
    text = assemble()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(text, encoding="utf-8")
    print(f"wrote {rel_label(args.output)}: {len(text.splitlines())} lines")
    return 0


if __name__ == "__main__":
    sys.exit(main())
