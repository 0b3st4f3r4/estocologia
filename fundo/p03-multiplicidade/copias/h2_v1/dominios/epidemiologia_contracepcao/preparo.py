"""Pesquisa de Fecundidade de Bangladesh (1988–89), subamostra de mulheres casadas usada por Huq & Cleland.
Só tradução de nomes, tipos e recortes; nenhuma modelagem."""

from pathlib import Path

import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "mlmRev_Contraception.csv")
    idade = d["age"].astype(float)                                   # idade centrada na média da amostra (anos)
    grupo = pd.cut(d["district"], [0, 15, 30, 45, 61], labels=["g1", "g2", "g3", "g4"])
    return pd.DataFrame({
        "y": (d["use"] == "Y").astype(float),                        # uso atual de contraceptivo
        "ambiente": grupo.astype(str),                               # grupo de 15 distritos pelo código do distrito
        "unidade": d["district"].astype(int),                        # o distrito: conglomerado da amostra e da oferta
        "tem_filhos": (d["livch"] != "0").astype(float),             # ao menos um filho vivo
        "filhos_3mais": (d["livch"] == "3+").astype(float),          # três ou mais filhos vivos
        "idade": idade,
        "idade_quad": idade ** 2 / 10.0,                             # uso em U invertido ao longo da vida reprodutiva
        "urbano": (d["urban"] == "Y").astype(float),
    })
