#!/usr/bin/env python3
"""Monta a edicao em 21 capitulos a partir de book/.

Uso:
    python3 book/gerar.py --check      # confere a rampa e, principalmente, o arquivo em disco
    python3 book/gerar.py              # escreve estocologia-livro-didatico.md (so se os 21 existirem)
    python3 book/gerar.py --saida x.md # escreve em outro caminho

O --check tem duas cobrancas: os 21 capitulos na rampa de 3 degraus, e o texto montado
igual, caractere por caractere, ao arquivo em disco. A segunda e a que impede a edicao
de envelhecer atras de book/: um livro que monta na memoria e nao esta no disco reprova.
"""

from __future__ import annotations

import argparse
import difflib
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


def conferir_destino(texto: str, destino: Path) -> list[str]:
    """O portao que olha o alvo: o arquivo em disco tem de ser o texto montado."""
    if not destino.exists():
        return [f"{rotulo(destino)} nao existe no disco"]
    em_disco = destino.read_text(encoding="utf-8")
    if em_disco == texto:
        return []
    diferenca = list(
        difflib.unified_diff(
            texto.splitlines(keepends=True),
            em_disco.splitlines(keepends=True),
            fromfile=f"montado ({len(texto.splitlines())} linhas)",
            tofile=f"{rotulo(destino)} ({len(em_disco.splitlines())} linhas)",
            n=1,
        )
    )
    problemas = [
        f"{rotulo(destino)} diverge do texto montado: "
        f"{sum(1 for l in diferenca if l.startswith('+') and not l.startswith('+++'))} linhas no disco a mais, "
        f"{sum(1 for l in diferenca if l.startswith('-') and not l.startswith('---'))} linhas montadas ausentes"
    ]
    problemas.extend(l.rstrip("\n") for l in diferenca[:40])
    if len(diferenca) > 40:
        problemas.append(f"... e mais {len(diferenca) - 40} linhas de diferenca")
    return problemas


def rotulo(caminho: Path) -> str:
    try:
        return str(caminho.resolve().relative_to(RAIZ))
    except ValueError:
        return str(caminho)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--saida", type=Path, default=DESTINO)
    ap.add_argument("--forcar", action="store_true", help="escreve mesmo com divergencia declarada")
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
        divergencias = conferir_destino(texto, DESTINO)
        if divergencias:
            print(f"o disco nao e o livro montado ({rotulo(DESTINO)}):")
            for d in divergencias:
                print(d)
            print("remonte com: python3 book/gerar.py")
            return 1
        print(f"{rotulo(DESTINO)} confere caractere por caractere com o texto montado")
        return 0

    if problemas:
        print("nao vou escrever o livro enquanto houver pendencia:")
        for p in problemas:
            print(f"  - {p}")
        return 1
    texto = montar()
    args.saida.parent.mkdir(parents=True, exist_ok=True)
    args.saida.write_text(texto, encoding="utf-8")
    print(f"escrito {rotulo(args.saida)}: {len(texto.splitlines())} linhas")
    return 0


if __name__ == "__main__":
    sys.exit(main())
