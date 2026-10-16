#!/usr/bin/env python3
"""Confere se o bloco de saida de cada capitulo bate com a execucao real de um instrumento.

Uso: python3 book/conferir_labs.py

Cada bloco de saida citado no livro tem de ser a saida literal de algum instrumento:
  labs --cap N     laboratorio do capitulo
  labs --banco     o H0 real, na base do banco
  labs --resolucao o horizonte R
Sai com 1 quando algum bloco nao aparece em nenhum deles.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
LIVRO = RAIZ / "book"
BIN = RAIZ / "target" / "release" / "labs"
BLOCO = re.compile(r"(?:\x60\x60\x60|~~~)text\n(.*?)(?:\x60\x60\x60|~~~)", re.DOTALL)


def saida(comando: list[str]) -> list[str]:
    proc = subprocess.run(comando, capture_output=True, text=True, cwd=RAIZ)
    return [l.rstrip() for l in (proc.stdout + proc.stderr).splitlines()]


def contem(agulha: list[str], palheiro: list[str]) -> bool:
    n = len(agulha)
    if n == 0:
        return True
    return any(palheiro[i : i + n] == agulha for i in range(len(palheiro) - n + 1))


def main() -> int:
    if not BIN.exists():
        print(f"instrumento ausente: {BIN} (rode: cargo build --release --bin labs)")
        return 1

    instrumentos: list[tuple[str, list[str]]] = [
        ("labs --banco", saida([str(BIN), "--banco"])),
        ("labs --resolucao", saida([str(BIN), "--resolucao"])),
        ("labs --s1", saida([str(BIN), "--s1"])),
        ("labs --fila", saida([str(BIN), "--fila"])),
        ("labs --genetica", saida([str(BIN), "--genetica"])),
        ("labs --escuta", saida([str(BIN), "--escuta"])),
        ("labs --h3", saida([str(BIN), "--h3"])),
    ]

    total = 0
    falhas = 0
    for caminho in sorted(LIVRO.glob("cap*.md")):
        cap = int(caminho.stem[3:])
        blocos = BLOCO.findall(caminho.read_text(encoding="utf-8"))
        if not blocos:
            continue
        do_cap = saida([str(BIN), "--cap", str(cap)])
        for i, bloco in enumerate(blocos, start=1):
            total += 1
            agulha = [l.rstrip() for l in bloco.strip("\n").splitlines()]
            onde = "labs --cap %d" % cap
            if contem(agulha, do_cap):
                print(f"cap{cap:02d} bloco {i}: confere em {onde} ({len(agulha)} linhas)")
                continue
            achado = next((nome for nome, saida_inst in instrumentos if contem(agulha, saida_inst)), None)
            if achado:
                print(f"cap{cap:02d} bloco {i}: confere em {achado} ({len(agulha)} linhas)")
                continue
            falhas += 1
            print(f"cap{cap:02d} bloco {i}: DIVERGE de todos os instrumentos")
            for a, b in zip(agulha, do_cap):
                if a != b:
                    print(f"   no texto: {a}")
                    print(f"   medido  : {b}")
                    break

    print(f"blocos conferidos: {total}, divergentes: {falhas}")
    return 1 if falhas else 0


if __name__ == "__main__":
    sys.exit(main())
