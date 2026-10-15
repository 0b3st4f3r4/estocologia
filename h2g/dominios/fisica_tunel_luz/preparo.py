"""Causal Chambers, túnel de luz (lt_interventions_standard_v1): Y = corrente da fonte; ambiente = experimento.
Declarado em ../../DECLARACAO_H2e.md antes de rodar."""

from pathlib import Path

import numpy as np
import pandas as pd

ARQ = "causal_chamber_lt_interventions_standard_v1.csv"
FORA = ("uniform_osr_c_", "uniform_v_c_")        # agem no sensor de Y: violam a premissa do ICP
CANDIDATOS = ["red", "green", "blue", "pol_1", "pol_2", "ir_1", "vis_1", "ir_2", "vis_2", "ir_3"]


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados).parent / "h2e" / ARQ, usecols=["experiment", "current", *CANDIDATOS])
    d = d[~d["experiment"].str.startswith(FORA)].reset_index(drop=True)
    ref = d.loc[d["experiment"] == "uniform_reference", "current"]
    out = pd.DataFrame({"y": (d["current"] - ref.mean()) / ref.std(), "ambiente": d["experiment"].astype(str),
                        "unidade": np.arange(len(d))})
    for c in CANDIDATOS:
        out[c] = d[c].astype(float)
    out["red"] = d["red"].astype(float) / 255.0
    return out
