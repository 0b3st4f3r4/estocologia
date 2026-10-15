"""Causal Chambers, túnel de vento (wt_walks_v1 / regime_jumps_single): Y = pressão a jusante; ambiente = regime.
Declarado em ../../DECLARACAO_H2e.md antes de rodar: uma medição a cada 20; unidade = bloco de 50 medições mantidas."""

from pathlib import Path

import numpy as np
import pandas as pd

ARQ = "causal_chamber_wt_walks_v1_regime_jumps_single.csv"
CANDIDATOS = ["load_in", "load_out", "hatch", "osr_downwind", "rpm_in", "rpm_out", "current_in",
              "pressure_upwind", "pressure_intake", "pressure_ambient"]
PASSO, BLOCO, COFRE = 20, 50, 18


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados).parent / "h2e" / ARQ, usecols=["flag", "counter", "pressure_downwind", *CANDIDATOS])
    d = d.sort_values(["flag", "counter"], kind="stable")
    d = d[d.groupby("flag").cumcount().to_numpy() % PASSO == 0].reset_index(drop=True)
    pos = d.groupby("flag").cumcount().to_numpy()
    treino = d["flag"] != COFRE
    y = d["pressure_downwind"].astype(float)
    out = pd.DataFrame({"y": (y - y[treino].mean()) / y[treino].std(), "ambiente": "r" + d["flag"].astype(int).astype(str),
                        "unidade": ["r%d_b%d" % (f, p // BLOCO) for f, p in zip(d["flag"].astype(int), pos)]})
    for c in CANDIDATOS:
        out[c] = d[c].astype(float)
    return out
