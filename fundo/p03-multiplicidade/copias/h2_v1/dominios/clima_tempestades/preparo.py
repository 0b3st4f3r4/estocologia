"""Trajetórias de ciclones do Atlântico Norte (dplyr::storms, best track do NHC), 1975 em diante."""

from pathlib import Path

import numpy as np
import pandas as pd

PRESSAO_AMBIENTE = 1013.0   # hPa, pressão de referência do ambiente


def _regiao_genese(lat: float, lon: float) -> str:
    """Região do primeiro ponto da trajetória (gênese): propriedade da tempestade inteira."""
    if lon < -81 and lat > 18:
        return "golfo_mexico"
    if lat <= 22 and -88 <= lon <= -60:
        return "caribe"
    if lat < 25 and lon > -60:
        return "atlantico_tropical"
    return "atlantico_subtropical"


def _estrutura(status: str) -> str:
    if status in ("tropical depression", "tropical storm", "hurricane"):
        return "tropical"            # as três classes diferem só pelo vento: agrupadas para não codificar o desfecho
    if status == "extratropical":
        return "extratropical"
    if status in ("subtropical storm", "subtropical depression"):
        return "subtropical"
    return "outra_baixa"             # onda tropical, perturbação, outra baixa


def _km(lat1, lon1, lat2, lon2):
    p1, p2 = np.deg2rad(lat1), np.deg2rad(lat2)
    a = np.sin((p2 - p1) / 2) ** 2 + np.cos(p1) * np.cos(p2) * np.sin(np.deg2rad(lon2 - lon1) / 2) ** 2
    return 6371 * 2 * np.arcsin(np.sqrt(a))


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "dplyr_storms.csv", usecols=["name", "year", "month", "day", "hour", "lat", "long",
                                                               "status", "wind", "pressure"])
    d = d.dropna(subset=["wind", "pressure", "lat", "long"])
    d["tempestade"] = d["name"].astype(str) + "-" + d["year"].astype(str)
    d["t"] = pd.to_datetime(dict(year=d["year"], month=d["month"], day=d["day"], hour=d["hour"]))
    d = d.sort_values(["tempestade", "t"]).reset_index(drop=True)
    g = d.groupby("tempestade")
    horas = (d["t"] - g["t"].shift(1)).dt.total_seconds() / 3600
    dist = _km(g["lat"].shift(1), g["long"].shift(1), d["lat"], d["long"])
    d["translacao"] = dist / horas
    d = d[(horas > 0) & d["translacao"].notna()]            # o primeiro ponto de cada trajetória não tem velocidade
    gen = d.groupby("tempestade").head(1).set_index("tempestade")
    regiao = {k: _regiao_genese(r.lat, r.long) for k, r in gen.iterrows()}
    deficit = (PRESSAO_AMBIENTE - d["pressure"]).clip(lower=0)
    return pd.DataFrame({
        "y": d["wind"].astype(float),                                   # vento máximo sustentado de 1 min (nós)
        "ambiente": d["tempestade"].map(regiao).astype(str),            # região de gênese da tempestade
        "unidade": d["tempestade"].astype(str),
        "raiz_deficit_pressao": np.sqrt(deficit).astype(float),        # √(1013 − p) em √hPa
        "latitude": d["lat"].astype(float),
        "translacao_kmh": d["translacao"].clip(upper=100).astype(float),
        "estrutura": d["status"].map(_estrutura).astype(str),
    }).reset_index(drop=True)
