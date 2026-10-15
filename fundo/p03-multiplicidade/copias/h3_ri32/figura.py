"""Painel da operação contínua: perda móvel do modelo e do reserva, disjuntor aberto, alarmes de deriva e
corte do cofre, um painel por fluxo. Grava saida/painel_operacao.png."""

from __future__ import annotations

import json
import os
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

from estocologia.monitor import pinball

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
FLUXOS = ["energia", "central", "bolsa", "voos", "ciclones", "prisoes"]


def perdas(d: pd.DataFrame, tipo: str):
    if tipo == "binario":
        return (d["modelo"] - d["y"]) ** 2, (d["reserva"] - d["y"]) ** 2
    nv = np.array([float(c.split("modelo_q")[1]) for c in d.columns if c.startswith("modelo_q")])
    qm = d[[f"modelo_q{n:g}" for n in nv]].to_numpy()
    qr = d[[f"reserva_q{n:g}" for n in nv]].to_numpy()
    y = d["y"].to_numpy()
    return (pd.Series([pinball(a, b, nv) for a, b in zip(y, qm)], index=d.index),
            pd.Series([pinball(a, b, nv) for a, b in zip(y, qr)], index=d.index))


def main():
    fig, eixos = plt.subplots(len(FLUXOS), 1, figsize=(11, 2.3 * len(FLUXOS)))
    for ax, nome in zip(eixos, FLUXOS):
        p = SAIDA / nome / "resultados.json"
        if not p.exists():
            continue
        r = json.loads(p.read_text())
        d = pd.read_parquet(SAIDA / nome / "diario.parquet")
        d = d[d["y"].notna()].sort_values("t_emissao")
        pm, pr = perdas(d, r["tipo"])
        t = pd.to_datetime(d["t_emissao"], unit="s")
        w = max(50, len(d) // 150)
        razao = (pm.rolling(w).mean() / pr.rolling(w).mean()).to_numpy()
        ax.plot(t, razao, lw=0.8, color="#3b6ea8", label="perda do modelo / perda do reserva (móvel)")
        ax.axhline(1, color="#888", lw=0.6)
        aberto = (d["origem"] == "reserva_disjuntor").to_numpy()
        ax.fill_between(t, 0, 2, where=aberto, color="#d9822b", alpha=0.25, step="mid", label="disjuntor aberto (reserva oficial)")
        for e in r["eventos_operacao"]:
            if e["tipo"] == "deriva" and e["evento"] == "ligou":
                ax.axvline(pd.to_datetime(e["t"], unit="s"), color="#b33", lw=0.5, alpha=0.6)
        ax.axvline(pd.to_datetime(r["corte"], unit="s"), color="k", lw=1.2, ls="--")
        ax.set_ylim(0.4, 1.6)
        ax.set_title(f"{nome}: cofre fora do tempo à direita da linha tracejada; traços vermelhos = alarme de deriva", fontsize=9, loc="left")
        ax.tick_params(labelsize=8)
    eixos[0].legend(fontsize=7, loc="upper left")
    fig.tight_layout()
    fig.savefig(SAIDA / "painel_operacao.png", dpi=130)
    print("gravado", SAIDA / "painel_operacao.png")


if __name__ == "__main__":
    main()
