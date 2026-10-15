"""Demanda elétrica de Victoria (Austrália), meia-hora a meia-hora em 2012–2014 (tsibbledata::vic_elec), agregada ao dia."""

from pathlib import Path

import numpy as np
import pandas as pd

BASE_GRAUS_DIA = 18.0          # base convencional (°C) dos graus-dia de aquecimento e resfriamento
LATITUDE_MELBOURNE = -37.81


def _horas_de_luz(dia_do_ano: np.ndarray) -> np.ndarray:
    """Duração astronômica do dia (fórmula de Cooper para a declinação solar): função só da data."""
    decl = np.deg2rad(23.44) * np.sin(2 * np.pi * (284 + dia_do_ano) / 365)
    lat = np.deg2rad(LATITUDE_MELBOURNE)
    return 24 / np.pi * np.arccos(np.clip(-np.tan(lat) * np.tan(decl), -1, 1))


def _estacao(mes: pd.Series) -> pd.Series:
    return mes.map({12: "verao", 1: "verao", 2: "verao", 3: "outono", 4: "outono", 5: "outono",
                    6: "inverno", 7: "inverno", 8: "inverno", 9: "primavera", 10: "primavera", 11: "primavera"})


def preparar(dados: Path) -> pd.DataFrame:
    e = pd.read_csv(Path(dados) / "tsibbledata_vic_elec.csv", usecols=["Demand", "Temperature", "Date", "Holiday"])
    d = e.groupby("Date").agg(demanda=("Demand", "mean"), t_media=("Temperature", "mean"), t_max=("Temperature", "max"),
                              feriado=("Holiday", "max")).reset_index()
    d["Date"] = pd.to_datetime(d["Date"])
    d = d.sort_values("Date").reset_index(drop=True)
    resfr = (d["t_media"] - BASE_GRAUS_DIA).clip(lower=0)
    aquec = (BASE_GRAUS_DIA - d["t_media"]).clip(lower=0)
    ds = d["Date"].dt.dayofweek
    ano = d["Date"].dt.year
    out = pd.DataFrame({
        "y": d["demanda"].astype(float),                                       # demanda média do dia (MW)
        # treino: estação do ano em 2012–2013 (as duas safras juntas); cofre: o ano de 2014 inteiro
        "ambiente": np.where(ano == 2014, "2014", _estacao(d["Date"].dt.month)).astype(str),
        "unidade": (d["Date"] - pd.to_timedelta(ds, unit="D")).dt.strftime("%Y-%m-%d"),   # a semana (segunda-feira)
        "graus_dia_resfriamento": resfr.astype(float),
        "graus_dia_aquecimento": aquec.astype(float),
        "resfriamento_ontem": resfr.shift(1).astype(float),                    # calor acumulado (inércia térmica dos prédios)
        "horas_de_luz": _horas_de_luz(d["Date"].dt.dayofyear.to_numpy()).astype(float),
        "feriado": d["feriado"].astype(bool).astype(float),
        "dia_semana": np.select([ds == 5, ds == 6], ["sabado", "domingo"], "dia_util").astype(str),
    })
    return out.iloc[1:].reset_index(drop=True)   # o primeiro dia não tem véspera
