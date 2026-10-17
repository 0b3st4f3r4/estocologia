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

import re
import subprocess

artigo = Path("artigo/README.md")
bin_labs = Path("target/release/labs")
if artigo.is_file() and bin_labs.is_file():
    texto_artigo = artigo.read_text(encoding="utf-8")

    def rodar_labs(* flags: str) -> str:
        return subprocess.check_output([str(bin_labs), *flags], text=True)

    s_cap5 = rodar_labs("--cap", "5")
    s_cap16 = rodar_labs("--cap", "16")
    s_cap17 = rodar_labs("--cap", "17")
    s_cap18 = rodar_labs("--cap", "18")
    s_h3 = rodar_labs("--h3")
    s_ponte = rodar_labs("--ponte")

    recomputados = [
        ("cap16 espera constante", re.search(r"8 todo dia \(a alegacao\)\s+\d+\s+([0-9.]+)", s_cap16).group(1).replace(".", ",")),
        ("cap16 espera surtos", re.search(r"5 ou 11\s+\d+\s+([0-9.]+)", s_cap16).group(1).replace(".", ",")),
        ("cap16 diferenca severa", re.search(r"diferenca observada = ([0-9.]+) min", s_cap16).group(1).replace(".", ",")),
        ("cap05 fracao no surto", re.search(r"\(([0-9.]+%) do total\)", s_cap5).group(1).replace(".", ",")),
        ("cap05 reprovados na prova", re.search(r"reprovados \(nota < 6,0\) = ([0-9.]+) em 100", s_cap5).group(1).replace(".", ",") + "%"),
        ("cap17 mediana 3x", f"{float(re.search(r'^3\s+\d+\s+([0-9.]+)\s+([0-9.]+)', s_cap17, re.M).group(1)):.1f}".replace(".", ",")),
        ("cap17 media 3x", f"{float(re.search(r'^3\s+\d+\s+([0-9.]+)\s+([0-9.]+)', s_cap17, re.M).group(2)):.0f} vezes"),
        ("cap17 LaLonde sorteado", f"+{float(re.search(r'experimento sorteado \(NSW trat\. - NSW ctrl\)\s+([0-9.]+)', s_cap17).group(1)) * 100:.1f}".replace(".", ",")),
        ("cap17 LaLonde cru", f"−{float(re.search(r'comparacao crua \(NSW trat\. - CPS\)\s+-([0-9.]+)', s_cap17).group(1)) * 100:.1f}".replace(".", ",")),
        ("cap18 Laplace janela limpa", re.search(r"ou (1 chance em \d+) por mes", s_cap18).group(1)),
        ("h3 vies ingenuo", f"{float(re.search(r'vies do previsor ingenuo contra a demanda exogena: ([0-9.]+)', s_h3).group(1)) * 100:.0f}%"),
        ("ponte piloto cego", re.search(r"apontou o defeito: braco X (\d+ de \d+)", s_ponte).group(1)),
    ]
    for origem, valor in recomputados:
        if valor not in texto_artigo:
            problemas.append(f"artigo/README.md diverge de {origem}: esperado {valor!r}")

if problemas:
    print("problemas no livro montado:")
    for p in problemas:
        print(f"  - {p}")
    sys.exit(1)
print(f"livro montado ok: {len(linhas)} linhas, {len(capitulos)} capítulos, 5 partes, cercas equilibradas")

