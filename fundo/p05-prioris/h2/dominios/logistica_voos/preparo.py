"""Partidas de Nova York em 2013 (nycflights13), unidas ao clima horário do aeroporto de origem."""

from pathlib import Path

import pandas as pd


def preparar(dados: Path) -> pd.DataFrame:
    dados = Path(dados)
    f = pd.read_csv(dados / "nycflights13_flights.csv",
                    usecols=["year", "month", "day", "hour", "dep_delay", "carrier", "origin", "distance", "time_hour"])
    w = pd.read_csv(dados / "nycflights13_weather.csv", usecols=["origin", "time_hour", "visib", "wind_speed", "precip", "temp"])
    w = w.drop_duplicates(["origin", "time_hour"])
    d = f.dropna(subset=["dep_delay"]).merge(w, on=["origin", "time_hour"], how="inner").dropna(subset=["visib", "wind_speed", "precip", "temp"])
    grandes = d["carrier"].value_counts().index[:8]
    return pd.DataFrame({
        "y": (d["dep_delay"] > 15).astype(float),                      # atraso de partida acima de 15 min (padrão do DOT)
        "ambiente": d["origin"].astype(str),                           # EWR, JFK, LGA
        "unidade": (d["month"] * 100 + d["day"]).astype(int),          # o dia: clima e propagação de atrasos são comuns ao dia
        "hora_programada": d["hour"].astype(float),
        "chuva": (d["precip"] > 0).astype(float),
        "visibilidade": d["visib"].astype(float),
        "vento": d["wind_speed"].clip(upper=60).astype(float),
        "temperatura": d["temp"].astype(float),
        "distancia_mil_milhas": (d["distance"] / 1000).astype(float),
        "companhia": d["carrier"].where(d["carrier"].isin(grandes), "outra").astype(str),
    })
