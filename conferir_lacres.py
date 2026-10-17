#!/usr/bin/env python3
"""Confere todos os lacres SHA-256 da arvore, de um so comando e de um so diretorio.

Uso:
    python3 conferir_lacres.py          # todos os lacres, relatorio por arquivo
    python3 conferir_lacres.py --quieto # so o resumo

Regra do formato, a mesma para os 25 lacres: cada linha util e

    <64 hex>  <caminho relativo a raiz do repositorio>

Caminho relativo a raiz e nao a pasta do lacre: o mesmo comando roda de qualquer lugar e
nao ha duas convencoes. Linha comecando em "#" e registro, nao alvo: serve para declarar o
que saiu da arvore sem fingir que ainda esta selado. Lacre sem nenhum alvo vivo nao passa
como se tivesse passado: aparece como declarado.

Sai com 1 quando algum alvo vivo falta, diverge, ou quando alguma linha util esta fora do
formato.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent
LINHA = re.compile(r"^([0-9a-f]{64})  (\S.*)$")
EXTRAS = ["DECLARACAO_DO_ESTADO.sha256"]


def lacres() -> list[Path]:
    achados = sorted((RAIZ / "fundo").rglob("*.sha256"))
    achados += [RAIZ / nome for nome in EXTRAS if (RAIZ / nome).is_file()]
    return achados


def conferir(caminho: Path) -> tuple[int, int, int, list[str]]:
    """Devolve (vivos conferidos, declarados fora da arvore, linhas invalidas, problemas)."""
    vivos = declarados = invalidas = 0
    problemas: list[str] = []
    for numero, bruta in enumerate(caminho.read_text(encoding="utf-8").splitlines(), 1):
        linha = bruta.rstrip()
        if not linha:
            continue
        if linha.startswith("#"):
            declarados += 1
            continue
        m = LINHA.match(linha)
        if not m:
            invalidas += 1
            problemas.append(f"{caminho.relative_to(RAIZ)}:{numero}: linha fora do formato: {linha[:80]}")
            continue
        esperado, relativo = m.group(1), m.group(2).strip()
        alvo = RAIZ / relativo
        if not alvo.is_file():
            problemas.append(f"{caminho.relative_to(RAIZ)}:{numero}: alvo ausente: {relativo}")
            continue
        real = hashlib.sha256(alvo.read_bytes()).hexdigest()
        if real != esperado:
            problemas.append(
                f"{caminho.relative_to(RAIZ)}:{numero}: {relativo} diverge (lacre {esperado[:12]}, disco {real[:12]})"
            )
            continue
        vivos += 1
    return vivos, declarados, invalidas, problemas


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--quieto", action="store_true")
    args = ap.parse_args()

    total_vivos = total_declarados = total_invalidas = 0
    sem_alvo: list[str] = []
    problemas: list[str] = []
    arquivos = lacres()
    for caminho in arquivos:
        vivos, declarados, invalidas, achados = conferir(caminho)
        total_vivos += vivos
        total_declarados += declarados
        total_invalidas += invalidas
        problemas.extend(achados)
        if vivos == 0:
            sem_alvo.append(str(caminho.relative_to(RAIZ)))
        if not args.quieto:
            estado = "ok" if not achados and vivos else ("declarado" if not achados else "FALHA")
            print(f"{caminho.relative_to(RAIZ)}: {vivos} alvos, {declarados} declarados fora da arvore — {estado}")
    for p in problemas:
        print(f"  {p}")
    print(
        f"lacres: {len(arquivos)} | alvos conferidos: {total_vivos} | "
        f"declarados fora da arvore: {total_declarados} | linhas invalidas: {total_invalidas} | "
        f"problemas: {len(problemas)}"
    )
    if sem_alvo:
        print("lacres sem alvo vivo (declarados): " + ", ".join(sem_alvo))
    return 1 if problemas else 0


if __name__ == "__main__":
    sys.exit(main())
