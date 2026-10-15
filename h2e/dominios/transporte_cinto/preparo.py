"""AER::USSeatBelts: estado-ano; ambiente = regime de fiscalização do cinto.

H2d (K = 3, ../../DECLARACAO_K3.md): efeito fixo de estado pela transformação within. y e os 9 candidatos são
centrados na média do próprio estado calculada SÓ nas linhas de treino (regimes no e secondary, uso observado); as
linhas do cofre (primary) usam as médias de treino do estado e nunca entram nas médias. Linhas do cofre de estados
sem linha de treino saem. Até o H2c, y era relativo à média de 1983–1984 do estado e os candidatos ficavam brutos.
"""

from pathlib import Path

import numpy as np
import pandas as pd

COFRE = "primary"          # igual a ambientes.cofre do pacote


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_USSeatBelts.csv")
    d = d.dropna(subset=["seatbelt"])
    sim = lambda c: (d[c] == "yes").astype(float)
    t = pd.DataFrame({
        "estado": d["state"].astype(str),
        "ambiente": d["enforce"].astype(str),
        "lfat": np.log(d["fatalities"].astype(float)),
        "uso_cinto": d["seatbelt"].astype(float),
        "ano": d["year"].astype(float),
        "renda_log": np.log(d["income"].astype(float)),
        "idade_media": d["age"].astype(float),
        "vel65": sim("speed65"),
        "vel70": sim("speed70"),
        "idade21": sim("drinkage"),
        "alcool008": sim("alcohol"),
        "trafego_log": np.log(d["miles"].astype(float)),
    })
    vars_ = ["lfat", "uso_cinto", "ano", "renda_log", "idade_media", "vel65", "vel70", "idade21", "alcool008", "trafego_log"]
    medias = t[t["ambiente"] != COFRE].groupby("estado")[vars_].mean()
    t = t[t["estado"].isin(medias.index)]                       # cofre de estado sem treino sai
    m = medias.loc[t["estado"]].to_numpy()
    w = t[vars_].to_numpy() - m
    out = pd.DataFrame(w, columns=vars_, index=t.index).rename(columns={"lfat": "y"})
    out["ambiente"] = t["ambiente"]
    out["unidade"] = t["estado"]
    return out[["y", "ambiente", "unidade", *vars_[1:]]].reset_index(drop=True)
