"""Project STAR em formato longo (jardim a 3ª série): ambiente = tipo de turma sorteado × série (pequena ou regular)."""

from pathlib import Path

import numpy as np
import pandas as pd

SERIES = ["k", "1", "2", "3"]


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_STAR.csv")
    partes = []
    for i, g in enumerate(SERIES):
        anteriores = sum((d[f"star{h}"] == "small").astype(int) for h in SERIES[:i]) if i else 0
        t = pd.DataFrame({
            "tipo": d[f"star{g}"], "nota": d[f"read{g}"] + d[f"math{g}"], "unidade": d[f"schoolid{g}"],
            "anos_pequena_antes": anteriores, "serie": g, "almoco": d[f"lunch{g}"], "genero": d["gender"],
            "etn": d["ethnicity"], "exp": d[f"experience{g}"], "grau": d[f"degree{g}"], "tetn": d[f"tethnicity{g}"],
            "escola": d[f"school{g}"],
        })
        t = t[t["tipo"].isin(["small", "regular"])].dropna()
        reg = t[t["tipo"] == "regular"]["nota"]      # padronização pelas turmas regulares da série (todas no treino)
        t["y"] = (t["nota"] - reg.mean()) / reg.std()
        partes.append(t)
    t = pd.concat(partes, ignore_index=True)
    return pd.DataFrame({
        "y": t["y"].astype(float),
        "ambiente": t["serie"] + np.where(t["tipo"] == "small", "_pequena", "_regular"),
        "unidade": t["unidade"].astype(int),
        "turma_pequena": (t["tipo"] == "small").astype(float),
        "anos_pequena_antes": t["anos_pequena_antes"].astype(float),
        "serie": t["serie"].astype(str),
        "almoco_gratuito": (t["almoco"] == "free").astype(float),
        "menina": (t["genero"] == "female").astype(float),
        "etnia": t["etn"].map(lambda e: e if e in ("cauc", "afam") else "outra").astype(str),
        "experiencia_prof": t["exp"].astype(float),
        "mestrado_prof": (t["grau"] != "bachelor").astype(float),
        "prof_negro": (t["tetn"] == "afam").astype(float),
        "tipo_escola": t["escola"].astype(str),
    })
