#!/usr/bin/env python3
"""Lista as ocorrencias de cada maneirismo, com numero de linha.

Uso:
    python3 estilo/ocorrencias.py book/cap07.md [padrao ...]

Serve para a releitura do revisor: a contagem diz quanto; a linha diz o que fazer.
Linhas dentro de cerca de codigo sao marcadas com [cerca].
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from varredura import PADROES  # noqa: E402


def main() -> int:
    caminho = Path(sys.argv[1])
    escolhidos = sys.argv[2:] or list(PADROES)
    linhas = caminho.read_text(encoding="utf-8").splitlines()

    dentro = False
    cerca = []
    for i, linha in enumerate(linhas, start=1):
        if linha.lstrip().startswith(("```", "~~~")):
            dentro = not dentro
            cerca.append(i)
            continue
        if dentro:
            cerca.append(i)

    for nome in escolhidos:
        padrao = PADROES[nome]
        achados = [(i, l) for i, l in enumerate(linhas, start=1)
                   if re.search(padrao, l, re.I) and i not in cerca]
        print(f"== {nome}: {len(achados)}")
        for i, l in achados:
            print(f"  {i:4d}: {l.strip()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
