"""(a) Tolerância de ECE que depende de n (DECLARACAO.md, item a)."""
from __future__ import annotations

import json

import numpy as np

from comum import AQUI, FLUXOS, SEMENTE, C, cofre, diario, resultados

B = 2000


def nula_ece(p: np.ndarray, rng, B: int = B, caixas: int = 10) -> np.ndarray:
    ordem = np.argsort(p, kind="stable")
    partes = [p[c] for c in np.array_split(ordem, caixas)]
    n = len(p)
    out = np.zeros(B)
    for pc in partes:
        sp = pc.sum()
        # soma de Bernoulli(pc) por simulação, em blocos para caber na memória
        s = np.zeros(B)
        passo = max(1, 20_000_000 // len(pc))
        for i in range(0, B, passo):
            k = min(passo, B - i)
            s[i:i + k] = (rng.random((k, len(pc))) < pc).sum(axis=1)
        out += np.abs(sp - s)
    return out / n


def main():
    rng = np.random.default_rng(SEMENTE)
    res = {}
    for fl, rot in FLUXOS.items():
        r = resultados(fl)
        cf = cofre(diario(fl), r["corte"])
        p, y = C.oficial_binario(cf), cf["y"].to_numpy(float)
        e = C.ece(p, y)
        nul = nula_ece(p, rng)
        q50, q95, q99 = (float(np.quantile(nul, q)) for q in (0.5, 0.95, 0.99))
        res[fl] = {"rotulo": rot, "n": int(len(p)), "y_medio": float(y.mean()), "p_medio": float(p.mean()),
                   "citl": float(p.mean() - y.mean()), "ece": e, "nula_media": float(nul.mean()), "q50": q50,
                   "q95": q95, "q99": q99, "valor_p": float((nul >= e).mean()), "ece_menos_nula": e - float(nul.mean()),
                   "passa_0_03": e <= 0.03, "passa_T95": e <= q95, "passa_T95mais": e <= max(0.03, q95)}
        print(fl, {k: (round(v, 5) if isinstance(v, float) else v) for k, v in res[fl].items()})
    (AQUI / "saida" / "ece_n.json").write_text(json.dumps(res, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
