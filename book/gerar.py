#!/usr/bin/env python3
"""Monta a edicao em 21 capitulos a partir de book/.

Uso:
    python3 book/gerar.py --check      # confere a rampa de 3 degraus e a presenca dos capitulos
    python3 book/gerar.py              # escreve estocologia-livro-didatico.md (so se os 21 existirem)
    python3 book/gerar.py --saida x.md # escreve em outro caminho
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
LIVRO = RAIZ / "book"
DESTINO = RAIZ / "estocologia-livro-didatico.md"

EXIGIDOS = ["## Capítulo ", "### Degrau 1", "### Degrau 2", "### Degrau 3", "**Exercícios.**", "**Resumo.**"]


def arquivos() -> list[Path]:
    return [LIVRO / f"cap{n:02d}.md" for n in range(1, 22)]


def conferir() -> list[str]:
    problemas: list[str] = []
    abertura = LIVRO / "00-abertura.md"
    if not abertura.exists():
        problemas.append("falta book/00-abertura.md")
    for caminho in arquivos():
        if not caminho.exists():
            problemas.append(f"falta {caminho.relative_to(RAIZ)}")
            continue
        texto = caminho.read_text(encoding="utf-8")
        numero = int(caminho.stem[3:])
        if f"## Capítulo {numero}." not in texto:
            problemas.append(f"{caminho.name}: titulo do capitulo {numero} ausente")
        for marcador in EXIGIDOS[1:]:
            if marcador not in texto:
                problemas.append(f"{caminho.name}: falta {marcador!r} (rampa de 3 degraus)")
        palavras = len(re.findall(r"\w+", texto))
        if palavras < 700:
            problemas.append(f"{caminho.name}: {palavras} palavras (minimo 700)")
        if re.search(r"\[cite:", texto):
            problemas.append(f"{caminho.name}: sobrou marcador de citacao [cite:")
    if not (LIVRO / "apendices.md").exists():
        problemas.append("falta book/apendices.md")
    return problemas


def montar() -> str:
    partes = [(LIVRO / "00-abertura.md").read_text(encoding="utf-8").rstrip()]
    for caminho in arquivos():
        partes.append(caminho.read_text(encoding="utf-8").rstrip())
    partes.append((LIVRO / "apendices.md").read_text(encoding="utf-8").rstrip())
    return "\n\n".join(partes) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--saida", type=Path, default=DESTINO)
    args = ap.parse_args()

    problemas = conferir()
    if args.check:
        if problemas:
            print("livro incompleto ou fora da rampa:")
            for p in problemas:
                print(f"  - {p}")
            return 1
        texto = montar()
        print(f"livro pronto: {len(texto.splitlines())} linhas, {len(re.findall(r'\w+', texto))} palavras")
        return 0

    if problemas:
        print("nao vou escrever o livro enquanto houver pendencia:")
        for p in problemas:
            print(f"  - {p}")
        return 1
    texto = montar()
    args.saida.write_text(texto, encoding="utf-8")
    print(f"escrito {args.saida.relative_to(RAIZ)}: {len(texto.splitlines())} linhas")
    return 0


if __name__ == "__main__":
    sys.exit(main())
