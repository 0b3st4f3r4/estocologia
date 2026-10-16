#!/usr/bin/env python3
"""Varredura mecanica dos maneirismos de prosa do livro Estocologia.

Uso:
    python3 estilo/varredura.py                     # todos os .md de book/
    python3 estilo/varredura.py book/cap07.md ...   # arquivos escolhidos
    python3 estilo/varredura.py --json /tmp/antes.json

A contagem e a mesma dos padroes declarados em estilo/folha-de-estilo.md.
Sem o antes e o depois medidos nao houve revisao: houve gosto pessoal.
"""

from __future__ import annotations

import argparse
import collections
import glob
import json
import re
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent

PADROES = {
    "travessao_apostivo": r" — ",
    "travessao_fecha_com_e": r"—\s*e\b",
    "mesmo_mesma": r"\bmesm[ao]s?\b",
    "o_mesmo": r"\bo mesmo\b",
    "o_que_relativo": r"\bo que\b",
    "virgula_nao": r",\s*não\b",
    "nao_e_X_e_Y": r"não (?:é|foi|são)\s[^.,;]{1,40},\s*(?:é|foi|são|mas)\b",
    "dois_pontos": r":",
    "proprio": r"\bprópri[ao]s?\b",
    "exatamente": r"\bexatamente\b",
}

# A rampa, os marcadores e as cercas nao entram na conta de prosa:
# o que esta dentro de cerca de codigo ou de bloco ```text e saida de
# instrumento e nao se toca.
CERCA = re.compile(r"(?:\x60\x60\x60|~~~).*?(?:\x60\x60\x60|~~~)", re.DOTALL)


def prosa(texto: str) -> str:
    return CERCA.sub("", texto)


def medir(caminho: Path) -> tuple[int, collections.Counter]:
    texto = prosa(caminho.read_text(encoding="utf-8"))
    palavras = len(re.findall(r"\w+", texto))
    contagem = collections.Counter({k: len(re.findall(p, texto, re.I)) for k, p in PADROES.items()})
    return palavras, contagem


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("arquivos", nargs="*")
    ap.add_argument("--json", type=Path, help="grava a contagem em JSON")
    ap.add_argument("--sem-cabecalho", action="store_true")
    args = ap.parse_args()

    if args.arquivos:
        caminhos = [Path(a) for a in args.arquivos]
    else:
        caminhos = [Path(p) for p in sorted(glob.glob(str(RAIZ / "book" / "*.md")))]

    total: collections.Counter = collections.Counter()
    per: dict[str, dict] = {}
    palavras_total = 0
    for caminho in caminhos:
        palavras, contagem = medir(caminho)
        palavras_total += palavras
        total.update(contagem)
        per[caminho.name] = {"palavras": palavras, **dict(contagem)}

    if not args.sem_cabecalho:
        print(f"palavras (fora das cercas): {palavras_total}")
        for k, v in total.most_common():
            print(f"{v:6d}  {k}")
        print()
        for nome, d in per.items():
            denso = ", ".join(f"{k}={v}" for k, v in sorted(d.items(), key=lambda x: -x[1]) if k != "palavras" and v)
            print(f"{nome:42s} {d['palavras']:6d} palavras | {denso}")

    if args.json:
        args.json.write_text(
            json.dumps({"palavras": palavras_total, "total": dict(total), "por_arquivo": per},
                       ensure_ascii=False, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        print(f"gravado {args.json}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
