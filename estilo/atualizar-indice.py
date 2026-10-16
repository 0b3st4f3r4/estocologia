#!/usr/bin/env python3
"""Escreve a tabela de contagens em estilo/indice-de-repeticao.md.

Uso:
    python3 estilo/atualizar-indice.py antes  /tmp/prosa_antes.json
    python3 estilo/atualizar-indice.py depois /tmp/prosa_depois.json

O arquivo tem dois blocos marcados; o script troca so o bloco pedido.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
INDICE = RAIZ / "estilo" / "indice-de-repeticao.md"

ORDEM = [
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


def tabela(dados: dict) -> str:
    linhas = ["| arquivo | palavras | " + " | ".join(ORDEM) + " |",
              "| --- | --- |" + " --- |" * len(ORDEM)]
    for nome in sorted(dados["por_arquivo"], key=lambda n: (not n.startswith("cap"), n)):
        d = dados["por_arquivo"][nome]
        celulas = " | ".join(str(d.get(k, 0)) for k in ORDEM)
        linhas.append(f"| `{nome}` | {d['palavras']} | {celulas} |")
    t = dados["total"]
    linhas.append("| **total** | **%d** | %s |" % (dados["palavras"], " | ".join(f"**{t.get(k, 0)}**" for k in ORDEM)))
    return "\n".join(linhas)


def main() -> int:
    qual, caminho = sys.argv[1], Path(sys.argv[2])
    dados = json.loads(caminho.read_text(encoding="utf-8"))
    ini, fim = f"<!-- {qual.upper()}:INICIO -->", f"<!-- {qual.upper()}:FIM -->"
    texto = INDICE.read_text(encoding="utf-8")
    a = texto.index(ini) + len(ini)
    b = texto.index(fim)
    novo = texto[:a] + "\n" + tabela(dados) + "\n" + texto[b:]
    INDICE.write_text(novo, encoding="utf-8")
    print(f"bloco {qual} atualizado em {INDICE.relative_to(RAIZ)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
