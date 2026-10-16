#!/usr/bin/env python3
"""Confere o equilibrio das cercas de codigo e a ordem das partes no livro montado."""

from __future__ import annotations

import sys
from pathlib import Path

caminho = Path(sys.argv[1] if len(sys.argv) > 1 else "estocologia-livro-didatico.md")
linhas = caminho.read_text(encoding="utf-8").splitlines()

aberta_em = None
problemas = []
for i, linha in enumerate(linhas, start=1):
    if linha.startswith("```"):
        if aberta_em is None:
            aberta_em = i
        else:
            aberta_em = None
if aberta_em is not None:
    problemas.append(f"cerca aberta e não fechada, linha {aberta_em}")

partes = [l for l in linhas if l.startswith("# ")]
esperado = [
    "# Estocologia: livro didático",
    "# Parte I.",
    "# Parte II.",
    "# Parte III.",
    "# Parte IV.",
    "# Parte V.",
    "# Apêndices",
]
if len(partes) != len(esperado):
    problemas.append(f"títulos de nível 1: {len(partes)}, esperado {len(esperado)}")
else:
    for a, b in zip(partes, esperado):
        if not a.startswith(b):
            problemas.append(f"ordem das partes: {a!r} no lugar de {b!r}")

capitulos = [l for l in linhas if l.startswith("## Capítulo ")]
if len(capitulos) != 21:
    problemas.append(f"capítulos: {len(capitulos)}, esperado 21")
numeros = [int(l.split()[2].rstrip(".")) for l in capitulos]
if numeros != list(range(1, 22)):
    problemas.append(f"numeração dos capítulos: {numeros}")

for marca in ["### Degrau 1", "### Degrau 2", "### Degrau 3", "**Exercícios.**", "**Resumo.**"]:
    n = sum(1 for l in linhas if l.startswith(marca))
    if n != 21:
        problemas.append(f"{marca}: {n} ocorrências, esperado 21")

if problemas:
    print("problemas no livro montado:")
    for p in problemas:
        print(f"  - {p}")
    sys.exit(1)
print(f"livro montado ok: {len(linhas)} linhas, {len(capitulos)} capítulos, 5 partes, cercas equilibradas")
