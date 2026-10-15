"""(portao de calibracao do fundo/p08, aplicado ao cofre binario do fundo/p10)

O ece e a nula_ece sao copia do fundo/p08-calibracao (codigo/estocologia/calibracao.py e ece_n.py), sem mudanca:
caixas de probabilidade de mesmo tamanho (mesma contagem), B = 2000 simulacoes da nula com as mesmas
probabilidades e o mesmo n, semente 20261015. A tolerancia que depende de n e o quantil 95 dessa nula.

Uso:  python3 calibracao_ece.py [caminho_do_diario.json]
Grava saida/calibracao_ece.json e imprime uma linha por previsor.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import numpy as np

AQUI = Path(__file__).resolve().parent
SEMENTE = 20261015
B = 2000
CAIXAS = 10


def ece(p: np.ndarray, y: np.ndarray, caixas: int = CAIXAS) -> float:
    ordem = np.argsort(p, kind="stable")
    partes = np.array_split(ordem, caixas)
    return float(sum(len(c) * abs(p[c].mean() - y[c].mean()) for c in partes if len(c)) / len(p))


def nula_ece(p: np.ndarray, rng, b: int = B, caixas: int = CAIXAS) -> np.ndarray:
    ordem = np.argsort(p, kind="stable")
    partes = [p[c] for c in np.array_split(ordem, caixas)]
    n = len(p)
    out = np.zeros(b)
    for pc in partes:
        sp = pc.sum()
        s = np.zeros(b)
        passo = max(1, 20_000_000 // len(pc))
        for i in range(0, b, passo):
            k = min(passo, b - i)
            s[i:i + k] = (rng.random((k, len(pc))) < pc).sum(axis=1)
        out += np.abs(sp - s)
    return out / n


def main() -> int:
    padrao = AQUI / "saida" / "diario.json"
    if not padrao.exists():
        padrao = AQUI / "saida" / "saude_sepse_alerta" / "diario.json"
    diario = Path(sys.argv[1]) if len(sys.argv) > 1 else padrao
    itens = json.loads(diario.read_text())
    por: dict[str, list] = {}
    for it in itens:
        if it.get("resultado") is not None:
            por.setdefault(it["previsor"], []).append(it)
    rng = np.random.default_rng(SEMENTE)
    res = {}
    for previsor, g in sorted(por.items()):
        p = np.array([it["prob"] for it in g], float)
        y = np.array([it["resultado"] for it in g], float)
        e = ece(p, y)
        nula = nula_ece(p, rng)
        q50, q95, q99 = (float(np.quantile(nula, q)) for q in (0.50, 0.95, 0.99))
        res[previsor] = {"n": int(len(p)), "p_medio": float(p.mean()), "y_medio": float(y.mean()),
                         "citl": float(p.mean() - y.mean()), "ece": e, "nula_media": float(nula.mean()),
                         "q50": q50, "q95": q95, "q99": q99, "valor_p": float((nula >= e).mean()),
                         "passa_0_03": bool(e <= 0.03), "passa_T95": bool(e <= q95),
                         "passa_T95mais": bool(e <= max(0.03, q95))}
        print(previsor, json.dumps({k: (round(v, 5) if isinstance(v, float) else v)
                                    for k, v in res[previsor].items()}, ensure_ascii=False), flush=True)
    out = AQUI / "saida" / "calibracao_ece.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps({"fonte": str(diario), "B": B, "semente": SEMENTE, "caixas": CAIXAS, "previsores": res},
                              indent=1, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
