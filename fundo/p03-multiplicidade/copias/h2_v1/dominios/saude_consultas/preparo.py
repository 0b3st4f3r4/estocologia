"""Demanda por consultas médicas de idosos (66+) com Medicare nos EUA, NMES 1987–88 (AER::NMES1988)."""

from pathlib import Path

import numpy as np
import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_NMES1988.csv")
    sim = lambda c: (d[c] == "yes").astype(float)
    return pd.DataFrame({
        "y": np.log1p(d["visits"]).astype(float),                   # log(1 + consultas em consultório médico no ano)
        "ambiente": d["region"].astype(str),                         # northeast, midwest, west, other
        "unidade": np.arange(len(d)),                                # a pessoa (amostra transversal)
        "seguro_privado": sim("insurance"),
        "medicaid": sim("medicaid"),
        "saude": d["health"].map({"poor": "ruim", "average": "media", "excellent": "excelente"}).astype(str),
        "doencas_cronicas": d["chronic"].astype(float),
        "limitacao_funcional": (d["adl"] == "limited").astype(float),
        "idade_decadas": d["age"].astype(float),                     # idade em décadas
        "renda_log": np.log1p(d["income"].clip(lower=0)).astype(float),   # log(1 + renda familiar em US$ 10 mil)
        "escolaridade": d["school"].astype(float),
        "mulher": (d["gender"] == "female").astype(float),
        "afro": sim("afam"),
        "casado": sim("married"),
        "empregado": sim("employed"),
    })
