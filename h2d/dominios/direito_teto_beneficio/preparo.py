"""wooldridge::injury (Meyer, Viscusi & Durbin 1995): ambiente = estado × antes/depois da mudança do teto do benefício."""

from pathlib import Path

import numpy as np
import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "wooldridge_injury.csv")
    d = d.dropna(subset=["ldurat", "benefit", "lprewage", "highearn", "ky", "male", "married", "lage", "hosp", "injtype",
                         "indust", "afchnge"])
    d = d[d["benefit"] > 0]
    amb = np.where(d["ky"] == 1, "ky", "mi") + np.where(d["afchnge"] == 1, "_depois", "_antes")
    return pd.DataFrame({
        "y": d["ldurat"].astype(float),
        "ambiente": amb,
        "unidade": d["rownames"].astype(int),             # um pedido por linha
        "beneficio_log": np.log(d["benefit"].astype(float)),
        "salario_log": d["lprewage"].astype(float),
        "alta_renda": d["highearn"].astype(float),
        "kentucky": d["ky"].astype(float),
        "homem": d["male"].astype(float),
        "casado": d["married"].astype(float),
        "idade_log": d["lage"].astype(float),
        "internacao": d["hosp"].astype(float),
        "tipo_lesao": d["injtype"].astype(int).astype(str),
        "industria": d["indust"].astype(int).astype(str),
    })
