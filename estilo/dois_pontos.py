#!/usr/bin/env python3
"""Separa os dois-pontos de prosa dos de tabela e de cerca de codigo.

Uso:
    python3 estilo/dois-pontos.py                # todos os arquivos do livro
    python3 estilo/dois-pontos.py book/cap02.md  # com a listagem das linhas de prosa

O dois-pontos de tabela e o de dentro de cerca nao entram na reescrita: um e estrutura,
o outro e saida de instrumento. O que sobra e o que se corta, quando so introduz um
periodo explicativo.
"""

from __future__ import annotations

import glob
import re
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent


def contar(caminho: Path) -> tuple[int, int, int, list[tuple[int, str]]]:
    dentro = False
    prosa = tabela = cerca = 0
    linhas: list[tuple[int, str]] = []
    for i, linha in enumerate(caminho.read_text(encoding="utf-8").splitlines(), start=1):
        if linha.lstrip().startswith(("```", "~~~")):
            dentro = not dentro
            cerca += linha.count(":")
            continue
        if dentro:
            cerca += linha.count(":")
        elif linha.startswith("|"):
            tabela += linha.count(":")
        else:
            n = linha.count(":")
            prosa += n
            if n:
                linhas.append((i, linha.strip()))
    return prosa, tabela, cerca, linhas


def main() -> int:
    alvos = [Path(a) for a in sys.argv[1:]] or [Path(p) for p in sorted(glob.glob(str(RAIZ / "book" / "*.md")))]
    print(f"{'arquivo':44s} {'prosa':>6} {'tabela':>7} {'cerca':>6}")
    for caminho in alvos:
        prosa, tabela, cerca, linhas = contar(caminho)
        print(f"{caminho.name:44s} {prosa:6d} {tabela:7d} {cerca:6d}")
        if sys.argv[1:]:
            for i, l in linhas:
                print(f"  {i:4d}: {l[:150]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
