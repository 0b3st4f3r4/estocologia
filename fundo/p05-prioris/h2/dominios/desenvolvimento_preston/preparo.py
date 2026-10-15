"""Curva de Preston: expectativa de vida e PIB per capita, 142 países, 1952–2007 em ondas de 5 anos (gapminder)."""

from pathlib import Path

import numpy as np
import pandas as pd


def _periodo(ano: int) -> str:
    if ano <= 1962:
        return "1952-1962"
    if ano <= 1977:
        return "1967-1977"
    if ano <= 1992:
        return "1982-1992"
    return "1997-2007"


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "gapminder_gapminder.csv")
    cont = {"Africa": "africa", "Americas": "americas", "Asia": "asia", "Europe": "europa", "Oceania": "oceania"}
    return pd.DataFrame({
        "y": d["lifeExp"].astype(float),                           # expectativa de vida ao nascer (anos)
        "ambiente": d["year"].map(_periodo).astype(str),            # período; 1997–2007 fica no cofre
        "unidade": d["country"].astype(str),
        "log_pib": np.log(d["gdpPercap"]).astype(float),           # log natural do PIB per capita (US$ PPC de 2005)
        "decadas_desde_1952": ((d["year"] - 1952) / 10).astype(float),
        "continente": d["continent"].map(cont).astype(str),
    })
