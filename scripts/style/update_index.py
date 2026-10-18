#!/usr/bin/env python3
"""Update the repetition count table in scripts/style/repetition-index.md.

Usage:
    python3 scripts/style/update_index.py antes  /tmp/prose_before.json
    python3 scripts/style/update_index.py depois /tmp/prose_after.json

The first argument names the block to replace: it is uppercased and used as the prefix of the pair of HTML
anchors that delimit the table in repetition-index.md (ANTES:DEPOIS and DOIS_PONTOS today, kept in the
Portuguese spelling the document uses), so the table is never transcribed by hand.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
INDEX_DOC = Path(__file__).resolve().parent / "repetition-index.md"

# Column order of the table. The names are the pattern keys of scan.py, in the Portuguese spelling the audited
# prose uses and the document already carries.
ORDER = [
    "travessao_apostivo",
    "travessao_fecha_com_e",
    "mesmo_mesma",
    "o_mesmo",
    "o_que_relativo",
    "virgula_nao",
    "nao_e_X_e_Y",
    "dois_pontos",
    "proprio",
    "exatamente",
]


def build_table(data: dict) -> str:
    # "per_file"/"words" are what scan.py writes today; "por_arquivo"/"palavras" are read as a fallback so a
    # JSON file produced before the key rename still opens.
    per_file = data.get("per_file") or data.get("por_arquivo", {})
    total_words = data.get("words") or data.get("palavras", 0)
    lines = ["| file | words | " + " | ".join(ORDER) + " |",
             "| --- | --- |" + " --- |" * len(ORDER)]
    for name in sorted(per_file, key=lambda n: (not n.startswith(("cap", "ch")), n)):
        d = per_file[name]
        w = d.get("words") or d.get("palavras", 0)
        cells = " | ".join(str(d.get(k, 0)) for k in ORDER)
        lines.append(f"| `{name}` | {w} | {cells} |")
    t = data["total"]
    lines.append("| **total** | **%d** | %s |" % (total_words, " | ".join(f"**{t.get(k, 0)}**" for k in ORDER)))
    return "\n".join(lines)


def main() -> int:
    which, path = sys.argv[1], Path(sys.argv[2])
    data = json.loads(path.read_text(encoding="utf-8"))
    start_tag, end_tag = f"<!-- {which.upper()}:INICIO -->", f"<!-- {which.upper()}:FIM -->"
    text = INDEX_DOC.read_text(encoding="utf-8")
    a = text.index(start_tag) + len(start_tag)
    b = text.index(end_tag)
    updated = text[:a] + "\n" + build_table(data) + "\n" + text[b:]
    INDEX_DOC.write_text(updated, encoding="utf-8")
    print(f"updated block {which} in {INDEX_DOC.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
