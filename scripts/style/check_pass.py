#!/usr/bin/env python3
"""Verify a prose revision pass against the baseline version of each chapter file.

Usage:
    python3 scripts/style/check_pass.py                  # all textbook source files in book/
    python3 scripts/style/check_pass.py book/ch07.md
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))
from scan import PATTERNS, prose  # noqa: E402

TARGETS = {
    "travessao_apostivo": 0.20,
    "travessao_fecha_com_e": 0.25,
    "mesmo_mesma": 0.30,
    "o_mesmo": 0.20,
    "o_que_relativo": 0.30,
    "virgula_nao": 0.25,
    "nao_e_X_e_Y": 0.50,
    "dois_pontos": 0.60,
    "proprio": 0.40,
    "exatamente": 0.40,
}

FENCE_RE = re.compile(r"(?:\x60\x60\x60|~~~)([a-zA-Z]*)\n(.*?)(?:\x60\x60\x60|~~~)", re.DOTALL)
NUMBER_RE = re.compile(r"\d+(?:[.,]\d+)*")
HEADING_RE = re.compile(r"(?m)^## (?:Capítulo|Chapter) (\d+)\.")
MARKERS_PT = ["### Degrau 1", "### Degrau 2", "### Degrau 3", "**Exercícios.**", "**Resumo.**"]
MARKERS_EN = ["### Step 1", "### Step 2", "### Step 3", "**Exercises.**", "**Summary.**"]

BASE = os.environ.get("ESTOCOLOGIA_BASE", "HEAD")


def file_baseline(name: str) -> str | None:
    proc = subprocess.run(
        ["git", "show", f"{BASE}:book/{name}"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        return None
    return proc.stdout


def extract_section(name: str, text: str) -> str:
    if name == "apendices.md" and "# Apêndices" in text:
        return text.split("# Apêndices", 1)[1]
    if name == "appendices.md" and "# Appendices" in text:
        return text.split("# Appendices", 1)[1]
    m = HEADING_RE.search(text)
    return text[m.end():] if m else text


def extract_blocks(text: str) -> list[tuple[str, str]]:
    return [(lang, normalize_fence(body)) for lang, body in FENCE_RE.findall(text)]


RENAMES = [
    ("dados-reais/", "data/"),
    ("fundo/", "empirical/"),
    ("governanca/", "governance/"),
    ("historico/", "history/"),
    ("artigo/", "paper/"),
    ("referencias/", "references/"),
    ("book/gerar_resultados.py", "scripts/book/generate_results.py"),
    ("book/conferir_labs.py", "scripts/book/check_labs.py"),
    ("book/conferir_livro.py", "scripts/book/check_book.py"),
    ("book/gerar.py", "scripts/book/build.py"),
    ("conferir_lacres.py", "scripts/check_seals.py"),
    ("scripts/verificar.sh", "scripts/verify.sh"),
    ("estocologia-livro-didatico.md", "book/stochology-textbook.md"),
    ("PROPOSTAS_DECLARADAS.md", "docs/DECLARED_PROPOSALS.md"),
    ("REGISTRO_EMPIRICO.md", "docs/EMPIRICAL_REGISTRY.md"),
    ("RESULTADOS_RUST.md", "docs/RUST_RESULTS.md"),
    ("DECLARACAO_DO_ESTADO.md", "docs/STATE_DECLARATION.md"),
    ("DECLARACAO_CONSOLIDACAO_RUST.md", "docs/declarations/RUST_CONSOLIDATION.md"),
    ("DECLARACAO_PERDA_RELATORIOS.md", "docs/declarations/LOST_REPORTS.md"),
    ("DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md", "docs/declarations/REMOVED_NON_SIGNIFICANT.md"),
    ("DECLARACAO_REMOVIDO_PYTHON.md", "docs/declarations/REMOVED_PYTHON.md"),
    ("INTEGRACAO_JEV.md", "docs/JEV_INTEGRATION.md"),
    ("PLAN.md", "docs/PLAN.md"),
]


def normalize_paths_for_diff(text: str) -> str:
    """Normalize reorganized paths so the 2026-10-17 migration does not read as a prose regression.

    The baseline in @@HEAD@@ speaks the old directory and file names; the working tree speaks the new
    ones. The mapping is applied to both sides, so a path migration cancels out while a number, a
    removed sentence, or a re-measured fence still shows.
    """
    guard = "\x00GUARD\x00"
    protected = [new for _, new in RENAMES if new.endswith(".md") or new.endswith(".py") or new.endswith(".sh")]
    for new in protected:
        text = text.replace(new, guard + new)
    for old, new in RENAMES:
        text = text.replace(old, new)
    return text.replace(guard, "")


def normalize_fence(body: str) -> str:
    """Compare fences by content, not by column padding: a re-laid-out block is not a new measurement."""
    return re.sub(r"[ \t]+", " ", body).strip()


def main() -> int:
    files = [Path(a) for a in sys.argv[1:]] or [
        p for p in sorted((ROOT / "book").glob("*.md")) if p.name != "stochology-textbook.md"
    ]

    failures = 0
    for path in files:
        name = path.name
        if name.startswith("00-"):
            key = "opening"
        elif name in ("apendices.md", "appendices.md"):
            key = "appendices"
        elif (name.startswith("cap") and name[3:5].isdigit()) or (name.startswith("ch") and name[2:4].isdigit()):
            key = name.split(".")[0]
        else:
            print(f"{name}: outside assembled edition — skipped")
            continue

        raw_before = file_baseline(name)
        if raw_before is None:
            # Also check if this is a fresh English chapter without a baseline yet; still enforce markers & min words!
            after_txt = extract_section(name, path.read_text(encoding="utf-8"))
            problems: list[str] = []
            if key.startswith(("cap", "ch")):
                markers = MARKERS_EN if ("### Step 1" in after_txt or name.startswith("ch")) else MARKERS_PT
                for marker in markers:
                    if marker not in after_txt:
                        problems.append(f"missing marker: {marker}")
            words = len(re.findall(r"\w+", prose(after_txt)))
            if words < 700:
                problems.append(f"word floor: {words} (< 700)")
            if problems:
                failures += 1
                print(f"{name}: FAIL")
                for p in problems:
                    print(f"   REGRESSION: {p}")
            else:
                print(f"{name}: verified ({words} words, markers ok; no prior baseline in {BASE}:book/{name})")
            continue

        before_txt = normalize_paths_for_diff(extract_section(name, raw_before))
        after_txt = normalize_paths_for_diff(extract_section(name, path.read_text(encoding="utf-8")))
        lines_before = [l for l in before_txt.splitlines() if not l.startswith("|")]
        lines_after = [l for l in after_txt.splitlines() if not l.startswith("|")]

        problems = []

        n_before = NUMBER_RE.findall("\n".join(lines_before))
        n_after = NUMBER_RE.findall("\n".join(lines_after))
        ca, cd = Counter(n_before), Counter(n_after)
        delta = {k: (ca[k], cd[k]) for k in set(ca) | set(cd) if ca[k] != cd[k]}
        if delta:
            detail = ", ".join(f"{k!r}: {a}->{d}" for k, (a, d) in sorted(delta.items())[:8])
            problems.append(f"numbers changed: {detail}")

        b_before = extract_blocks(before_txt)
        b_after = extract_blocks(after_txt)
        if len(b_before) != len(b_after):
            problems.append(f"code fences changed count: before {len(b_before)} after {len(b_after)}")
        for lang, body in b_before:
            if (lang, body) not in b_after:
                problems.append(f"code fence {lang!r} ({len(body.splitlines())} lines) changed")

        if key.startswith(("cap", "ch")):
            markers = MARKERS_EN if ("### Step 1" in after_txt or name.startswith("ch")) else MARKERS_PT
            for marker in markers:
                if marker not in after_txt:
                    problems.append(f"missing marker: {marker}")

        words = len(re.findall(r"\w+", prose(after_txt)))
        if words < 700:
            problems.append(f"word floor: {words}")

        p_before = prose(before_txt)
        p_after = prose(after_txt)
        fields = []
        for pattern, target in TARGETS.items():
            a = len(re.findall(PATTERNS[pattern], p_before, re.I))
            d = len(re.findall(PATTERNS[pattern], p_after, re.I))
            if pattern in ("virgula_nao", "travessao_fecha_com_e"):
                running = [l for l in prose(after_txt).splitlines() if not l.startswith(("|", "#"))]
                d_running = len(re.findall(PATTERNS[pattern], "\n".join(running), re.I))
                if d_running > 2:
                    problems.append(f"{pattern} above chapter ceiling in running prose: {d_running}")
            limit = int(a * target)
            flag = "" if d <= limit else "!"
            fields.append(f"{pattern} {a}->{d}{flag}")
        print(f"{name}: " + " | ".join(fields))
        if problems:
            failures += 1
            for p in problems:
                print(f"   REGRESSION: {p}")
    print(f"files verified: {len(files)}, with regression: {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
