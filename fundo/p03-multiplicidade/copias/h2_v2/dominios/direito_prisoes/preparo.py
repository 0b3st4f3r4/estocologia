"""Prisões por porte de pequena quantidade de maconha em Toronto (1997–2002), série do Toronto Star.
Só tradução de nomes, tipos e recortes; nenhuma modelagem."""

from pathlib import Path

import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "carData_Arrests.csv")
    sim = lambda c: (d[c] == "Yes").astype(float)
    idade = d["age"].astype(float)
    return pd.DataFrame({
        "y": sim("released"),                                # liberado com notificação para comparecer (1) ou detido
        "ambiente": d["year"].astype(str),                   # ano da prisão
        "unidade": d["rownames"].astype(int),                # a prisão: a base não identifica a pessoa nem o policial
        "negro": (d["colour"] == "Black").astype(float),     # cor da pele registrada pela polícia
        "idade": idade,
        "idade_quad": (idade - 25.0) ** 2 / 100.0,           # curvatura: jovens e mais velhos tratados de outro modo
        "masculino": (d["sex"] == "Male").astype(float),
        "empregado": sim("employed"),
        "cidadao": sim("citizen"),
        "registros": d["checks"].astype(float),              # nº de bases policiais em que o nome aparece (0 a 6)
    })
