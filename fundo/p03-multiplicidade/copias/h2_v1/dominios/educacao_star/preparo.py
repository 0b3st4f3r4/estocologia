"""Project STAR, jardim de infância (1985–86): turmas sorteadas dentro de cada escola do Tennessee."""

from pathlib import Path

import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "AER_STAR.csv")
    d = d.dropna(subset=["stark", "readk", "mathk", "schoolk", "schoolidk", "lunchk", "gender", "ethnicity", "experiencek", "degreek"])
    return pd.DataFrame({
        "y": (d["readk"] + d["mathk"]).astype(float),        # soma das notas SAT de leitura e matemática
        "ambiente": d["schoolk"].astype(str),                # tipo de escola: rural, suburbana, urbana, centro antigo
        "unidade": d["schoolidk"].astype(int),               # a escola: o sorteio e o professor estão dentro dela
        "turma_pequena": (d["stark"] == "small").astype(float),
        "auxiliar": (d["stark"] == "regular+aide").astype(float),
        "almoco_gratuito": (d["lunchk"] == "free").astype(float),
        "menina": (d["gender"] == "female").astype(float),
        "etnia": d["ethnicity"].map(lambda e: e if e in ("cauc", "afam") else "outra").astype(str),
        "experiencia_prof": d["experiencek"].astype(float),
        "mestrado_prof": (d["degreek"] != "bachelor").astype(float),
    })
