"""AER::USSeatBelts: estado-ano; ambiente = regime de fiscalização do cinto; y relativo à linha de base 1983–1984 do estado."""

from pathlib import Path

import numpy as np
import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_USSeatBelts.csv")
    d["ano"] = d["year"].astype(int)
    d["lfat"] = np.log(d["fatalities"].astype(float))
    d["lmil"] = np.log(d["miles"].astype(float))
    # linha de base: anos 1983–1984, todos sem lei de cinto (ambiente "no", nunca o cofre)
    base = d[d["ano"] <= 1984].groupby("state")[["lfat", "lmil"]].mean()
    d = d.join(base, on="state", rsuffix="_base")
    d = d.dropna(subset=["seatbelt", "lfat_base"])
    sim = lambda c: (d[c] == "yes").astype(float)
    return pd.DataFrame({
        "y": (d["lfat"] - d["lfat_base"]).astype(float),
        "ambiente": d["enforce"].astype(str),
        "unidade": d["state"].astype(str),
        "uso_cinto": d["seatbelt"].astype(float),
        "ano": d["ano"].astype(float),
        "renda_log": np.log(d["income"].astype(float)),
        "idade_media": d["age"].astype(float),
        "vel65": sim("speed65"),
        "vel70": sim("speed70"),
        "idade21": sim("drinkage"),
        "alcool008": sim("alcohol"),
        "trafego_log": (d["lmil"] - d["lmil_base"]).astype(float),
    })
