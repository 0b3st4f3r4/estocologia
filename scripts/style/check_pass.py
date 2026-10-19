#!/usr/bin/env python3
"""Verify a prose revision pass against the baseline version of each chapter file.

Usage:
    python3 scripts/style/check_pass.py                  # all textbook source files in book/
    python3 scripts/style/check_pass.py book/ch07.md
"""

from __future__ import annotations

import ast
import bisect
import math
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

# Files rewritten from scratch under the owner's decision of 2026-10-18: content only, no metacommentary, the
# whole theory of the research. A rewrite is not a prose pass, so numbers and fences may move. These files are
# checked for the ramp markers, the word floor, and number provenance instead.
#
# Provenance (item 16, closed on 2026-10-19). A number in running prose has a source when it is
#   1. in an output block of the same file (check_labs.py holds the block to the instrument);
#   2. in the literal output of an instrument (docs/RUST_RESULTS.md, regenerated and checked by generate_results.py),
#      in the code that computes it (crates/*/src), in a record of the research (empirical/**/*.md, governance/*.md),
#      in the data sources, the paper, the plan, the registry or the bibliography check;
#   3. with three significant figures or more, the rounding of such a number (0.6168 -> 0.617), or of it times 100
#      (0.4259 -> 42.6, a share written as a percentage);
#   4. a row of book/provenance.tsv, which says where it comes from in a form this script checks (see LEDGER_KINDS).
# Until 2026-10-19 a fifth clause accepted any number of the previous assembled book. It was circular: a stale number
# certified itself. It is gone.
REWRITTEN = {"00-opening.md", "appendices.md"} | {f"ch{n:02d}.md" for n in range(1, 22)}
SOURCES = [
    "paper/README.md",
    "docs/PLAN.md",
    "docs/EMPIRICAL_REGISTRY.md",
    "docs/RUST_RESULTS.md",
    "references/VERIFICATION.md",
    "data/SOURCES.md",
]
SOURCE_GLOBS = ["crates/*/src/**/*.rs", "empirical/**/*.md", "governance/*.md"]
LEDGER = ROOT / "book" / "provenance.tsv"
LEDGER_KINDS = {
    # the expression, arithmetic only, evaluates to the number at the decimals it is printed with
    "derivation",
    # the pointer is "path" or "path::needle"; the file exists and contains the needle (default: the number itself)
    "file",
    # the pointer is "commit:path"; the file at that commit contains the number (a value the text records as earlier)
    "git",
    # the pointer is a phrase of Appendix D (a checked reference) that names the source of a date or a figure
    "reference",
    # a value an exercise asks the reader to try; the pointer is "path::needle", the line of the instrument it changes
    "exercise",
}


def _source_texts() -> list[str]:
    texts = []
    for rel in SOURCES:
        path = ROOT / rel
        if path.is_file():
            texts.append(path.read_text(encoding="utf-8"))
    for pattern in SOURCE_GLOBS:
        for path in sorted(ROOT.glob(pattern)):
            texts.append(path.read_text(encoding="utf-8", errors="ignore"))
    return texts


def source_numbers() -> set[str]:
    known: set[str] = set()
    for text in _source_texts():
        # "1 800" is one number written with a thin thousands space; read it whole as well as in parts.
        text = text.replace("{,}", ",")  # LaTeX decimal comma: 1{,}96
        text = re.sub(r"(?<=\d)_(?=\d)", "", text)  # Rust digit separators: 400_000
        for joined in (text, re.sub(r"(\d) (\d{3})(?!\d)", r"\1\2", text)):
            for n in NUMBER_RE.findall(joined):
                # A source may write 0,294 or 1.800 (Portuguese decimal comma and thousands point): keep every
                # reading, so that a number moved into English notation still finds its source.
                known.update({n, n.replace(",", "."), n.replace(",", ""), n.replace(".", "")})
    return known


def source_values(known: set[str]) -> list[float]:
    values = set()
    for n in known:
        if re.fullmatch(r"\d+(?:\.\d+)?", n):
            values.add(float(n))
    return sorted(values)


def significant(n: str) -> int:
    return len(n.replace(",", "").replace(".", "").lstrip("0"))


def rounds_from(n: str, values: list[float]) -> bool:
    """A number of three or more significant figures that is the rounding of a source value, or of it times 100."""
    if significant(n) < 3:
        return False
    c = canonical(n)
    try:
        x = float(c)
    except ValueError:
        return False
    dec = len(c.split(".")[1]) if "." in c else 0
    lo, hi = bisect.bisect_left(values, x - 1), bisect.bisect_right(values, x + 1)
    if any(round(v, dec) == x for v in values[lo:hi]):
        return True
    lo, hi = bisect.bisect_left(values, (x - 1) / 100), bisect.bisect_right(values, (x + 1) / 100)
    return any(round(v * 100, dec) == x for v in values[lo:hi])


def safe_eval(expr: str) -> float:
    tree = ast.parse(expr, mode="eval")
    allowed = (ast.Expression, ast.BinOp, ast.UnaryOp, ast.Constant, ast.Add, ast.Sub, ast.Mult, ast.Div,
               ast.Pow, ast.USub, ast.UAdd, ast.Call, ast.Name, ast.Load)
    funcs = {"log": math.log, "log2": math.log2, "exp": math.exp, "sqrt": math.sqrt}
    for node in ast.walk(tree):
        if not isinstance(node, allowed):
            raise ValueError(f"not arithmetic: {expr}")
        if isinstance(node, ast.Name) and node.id not in funcs:
            raise ValueError(f"unknown name {node.id} in {expr}")
    return float(eval(compile(tree, "<ledger>", "eval"), {"__builtins__": {}}, funcs))


def read_ledger() -> tuple[dict[tuple[str, str], tuple[str, str]], list[str]]:
    """Rows of book/provenance.tsv, checked one by one: (file, number) -> (kind, pointer), and the problems."""
    rows: dict[tuple[str, str], tuple[str, str]] = {}
    problems: list[str] = []
    if not LEDGER.is_file():
        return rows, problems
    appendix_d = (ROOT / "book" / "appendices.md").read_text(encoding="utf-8")
    for i, line in enumerate(LEDGER.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) != 5:
            problems.append(f"provenance.tsv:{i}: five tab-separated fields expected")
            continue
        name, number, kind, pointer, _note = parts
        where = f"provenance.tsv:{i} ({name} {number})"
        if kind not in LEDGER_KINDS:
            problems.append(f"{where}: unknown kind {kind}")
            continue
        if kind == "derivation":
            try:
                value = safe_eval(pointer)
            except (ValueError, SyntaxError, ZeroDivisionError) as e:
                problems.append(f"{where}: {e}")
                continue
            c = canonical(number)
            dec = len(c.split(".")[1]) if "." in c else 0
            if round(value, dec) != float(c):
                problems.append(f"{where}: {pointer} = {value:.6g}, not {number}")
                continue
        elif kind in ("file", "exercise"):
            path, _, needle = pointer.partition("::")
            target = ROOT / path
            needle = needle or number
            if not target.is_file() or needle not in target.read_text(encoding="utf-8", errors="ignore"):
                problems.append(f"{where}: {path} does not contain {needle!r}")
                continue
        elif kind == "git":
            commit, _, path = pointer.partition(":")
            proc = subprocess.run(["git", "show", f"{commit}:{path}"], cwd=ROOT, capture_output=True, text=True)
            # the earlier text may be the Portuguese one: 0,9586 for 0.9586, 26 795 for 26,795
            swapped = number.translate(str.maketrans(",.", ".,"))
            variants = {number, swapped, number.replace(",", " ")}
            if proc.returncode != 0 or not any(v in proc.stdout for v in variants):
                problems.append(f"{where}: {commit}:{path} does not contain {number}")
                continue
        elif kind == "reference":
            if pointer not in appendix_d:
                problems.append(f"{where}: Appendix D does not contain {pointer!r}")
                continue
        rows[(name, number)] = (kind, pointer)
    return rows, problems


def canonical(n: str) -> str:
    """Read a number written in English notation: 1,299 is one thousand two hundred ninety-nine."""
    if re.fullmatch(r"[1-9]\d{0,2}(?:,\d{3})+(?:\.\d+)?", n):
        return n.replace(",", "")
    return n.replace(",", ".")


def check_rewrite(name: str, text: str, known: set[str], values: list[float],
                  ledger: dict[tuple[str, str], tuple[str, str]], used: set[tuple[str, str]]) -> list[str]:
    body = extract_section(name, text)
    problems: list[str] = []
    if name.startswith("ch"):
        for marker in MARKERS_EN:
            if marker not in body:
                problems.append(f"missing marker: {marker}")
    words = len(re.findall(r"\w+", prose(body)))
    if words < 700 and name.startswith("ch"):
        problems.append(f"word floor: {words}")
    own = {n for _, block in FENCE_RE.findall(body) for n in NUMBER_RE.findall(block)}
    running = FENCE_RE.sub("", body)
    running = "\n".join(l for l in running.splitlines() if not l.startswith("|")).replace("{,}", ",")
    unknown = set()
    for n in set(NUMBER_RE.findall(running)):
        # a ledger row is the declared source even when another source happens to carry the same digits
        if (name, n) in ledger:
            used.add((name, n))
            continue
        if n in own or n in known or canonical(n) in known or rounds_from(n, values):
            continue
        unknown.add(n)
    if unknown:
        problems.append(f"numbers without a source: {', '.join(sorted(unknown)[:40])}")
    return problems


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
    known: set[str] | None = None
    values: list[float] = []
    ledger, ledger_problems = read_ledger()
    used: set[tuple[str, str]] = set()
    for p in ledger_problems:
        print(f"   REGRESSION: {p}")
    failures += len(ledger_problems)
    for path in files:
        name = path.name
        if name in REWRITTEN:
            if known is None:
                known = source_numbers()
                values = source_values(known)
            problems = check_rewrite(name, path.read_text(encoding="utf-8"), known, values, ledger, used)
            if problems:
                failures += 1
                print(f"{name}: FAIL (rewrite)")
                for p in problems:
                    print(f"   REGRESSION: {p}")
            else:
                print(f"{name}: rewrite verified (markers, word floor, every number has a source)")
            continue
        if name.startswith("00-"):
            key = "opening"
        elif name in ("apendices.md", "appendices.md"):
            key = "appendices"
        elif (name.startswith("cap") and name[3:5].isdigit()) or (name.startswith("ch") and name[2:4].isdigit()):
            key = name.split(".")[0]
        else:
            print(f"{name}: outside assembled book — skipped")
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
    checked = {path.name for path in files}
    stale = sorted(k for k in ledger if k[0] in checked and k not in used)
    for name, number in stale:
        print(f"   REGRESSION: provenance.tsv names {number} in {name}, which no longer has it in running prose")
    failures += len(stale)
    print(f"provenance ledger: {len(ledger)} rows, {len(used)} used")
    print(f"files verified: {len(files)}, with regression: {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
