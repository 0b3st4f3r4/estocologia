"""Métricas de calibração fora do tempo do H3 (CRITERIOS.md, G2).

Tudo sai do diário. A previsão oficial é a do modelo quando a origem é "modelo", e a do reserva
nos demais casos.

A climatologia móvel de um evento binário é a taxa-base dos rótulos já conhecidos no instante da
emissão, isto é, das previsões com t_rotulo ≤ t_emissao. Ela é calculada pelo próprio diário, sem
olhar adiante.
"""

from __future__ import annotations

import numpy as np
import pandas as pd
from scipy import stats


def oficial_binario(df: pd.DataFrame) -> np.ndarray:
    return np.where(df["origem"] == "modelo", df["modelo"], df["reserva"]).astype(float)


def oficial_quantis(df: pd.DataFrame, niveis) -> np.ndarray:
    m = np.column_stack([df[f"modelo_q{n:g}"] for n in niveis])
    r = np.column_stack([df[f"reserva_q{n:g}"] for n in niveis])
    return np.where((df["origem"] == "modelo").to_numpy()[:, None], m, r)


def climatologia_movel(df: pd.DataFrame) -> np.ndarray:
    """Para cada previsão, a média dos rótulos com t_rotulo ≤ t_emissao (todas as previsões do diário,
    inclusive as anteriores ao início do placar)."""
    res = df.dropna(subset=["y"]).sort_values("t_rotulo")
    tr, cum = res["t_rotulo"].to_numpy(), np.cumsum(res["y"].to_numpy())
    k = np.searchsorted(tr, df["t_emissao"].to_numpy(), side="right")
    clima = np.where(k > 0, cum[np.maximum(k - 1, 0)] / np.maximum(k, 1), 0.5)
    return np.clip(clima, 0.01, 0.99)


def ece(p: np.ndarray, y: np.ndarray, caixas: int = 10) -> float:
    ordem = np.argsort(p, kind="stable")
    partes = np.array_split(ordem, caixas)
    return float(sum(len(c) * abs(p[c].mean() - y[c].mean()) for c in partes if len(c)) / len(p))


def murphy(p: np.ndarray, y: np.ndarray, caixas: int = 10) -> dict:
    ordem = np.argsort(p, kind="stable")
    ybar = y.mean()
    rel = res = 0.0
    for c in np.array_split(ordem, caixas):
        if len(c):
            rel += len(c) * (p[c].mean() - y[c].mean()) ** 2
            res += len(c) * (y[c].mean() - ybar) ** 2
    n = len(p)
    return {"REL": rel / n, "RES": res / n, "UNC": float(ybar * (1 - ybar))}


def binario(df: pd.DataFrame, clima: np.ndarray) -> dict:
    p, y = oficial_binario(df), df["y"].to_numpy(float)
    b = float(np.mean((p - y) ** 2))
    bc = float(np.mean((clima - y) ** 2))
    br = float(np.mean((df["reserva"].to_numpy(float) - y) ** 2))
    bm = float(np.mean((df["modelo"].to_numpy(float) - y) ** 2))
    return {"n": int(len(y)), "p_medio": float(p.mean()), "y_medio": float(y.mean()), "citl": float(p.mean() - y.mean()),
            "ece": ece(p, y), "brier": b, "brier_climatologia": bc, "habilidade_vs_climatologia": 1 - b / bc,
            "habilidade_vs_reserva": 1 - b / br, "habilidade_modelo_sombra_vs_climatologia": 1 - bm / bc,
            "murphy": murphy(p, y), "fracao_oficial_reserva": float((df["origem"] != "modelo").mean())}


def escore_intervalo(lo, hi, y, alpha=0.10):
    return (hi - lo) + 2 / alpha * (lo - y) * (y < lo) + 2 / alpha * (y - hi) * (y > hi)


def quantis(df: pd.DataFrame, niveis) -> dict:
    niveis = np.asarray(niveis)
    q, y = oficial_quantis(df, niveis), df["y"].to_numpy(float)
    r = np.column_stack([df[f"reserva_q{n:g}"] for n in niveis])
    out = {"n": int(len(y)), "acertos_por_nivel": {f"{n:g}": float(np.mean(y < q[:, i])) for i, n in enumerate(niveis)},
           "fracao_oficial_reserva": float((df["origem"] != "modelo").mean())}
    if 0.05 in niveis and 0.95 in niveis:
        i5, i95 = list(niveis).index(0.05), list(niveis).index(0.95)
        out["cobertura_90"] = float(np.mean((y >= q[:, i5]) & (y <= q[:, i95])))
        s_of = escore_intervalo(q[:, i5], q[:, i95], y).mean()
        s_r = escore_intervalo(r[:, i5], r[:, i95], y).mean()
        out["escore_intervalo"], out["escore_intervalo_reserva"] = float(s_of), float(s_r)
        out["habilidade_intervalo_vs_reserva"] = float(1 - s_of / s_r)
    return out


def kupiec(viol: np.ndarray, p: float) -> dict:
    n, x = len(viol), int(viol.sum())
    ph = x / n
    ll0 = (n - x) * np.log(1 - p) + x * np.log(p)
    ll1 = (n - x) * np.log(1 - ph) + x * np.log(ph) if 0 < x < n else 0.0
    lr = -2 * (ll0 - ll1)
    return {"n": n, "violacoes": x, "taxa": ph, "LR": float(lr), "p": float(stats.chi2.sf(lr, 1))}


def christoffersen(viol: np.ndarray) -> dict:
    v = viol.astype(int)
    a, b = v[:-1], v[1:]
    n00, n01 = int(((a == 0) & (b == 0)).sum()), int(((a == 0) & (b == 1)).sum())
    n10, n11 = int(((a == 1) & (b == 0)).sum()), int(((a == 1) & (b == 1)).sum())
    p01 = n01 / max(n00 + n01, 1)
    p11 = n11 / max(n10 + n11, 1)
    p = (n01 + n11) / max(n00 + n01 + n10 + n11, 1)

    def ll(k, q):
        return k * np.log(q) if k > 0 and q > 0 else 0.0
    l0 = ll(n00 + n10, 1 - p) + ll(n01 + n11, p)
    l1 = ll(n00, 1 - p01) + ll(n01, p01) + ll(n10, 1 - p11) + ll(n11, p11)
    lr = -2 * (l0 - l1)
    return {"p_viol_depois_de_viol": p11, "p_viol_depois_de_nao": p01, "LR": float(lr), "p": float(stats.chi2.sf(lr, 1))}
