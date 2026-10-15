"""Converte os voos de NY 2017 (jayleetx/nycflights) e de Portland/Seattle 2014 (ismayc/pnwflights14) para o formato
CSV do nycflights13 do Rdatasets que o fluxo `voos` lê: rownames, time_hour (hora programada cheia, em UTC, "...Z"),
minute (minuto programado), dep_time, dep_delay, arr_delay, air_time, carrier, origin, dest, distance. O tempo
meteorológico vira origin, time_hour (UTC), visib, wind_speed, precip.

Uso: python3 -I preparar_dados.py <pasta dos .rda> <pasta de saída>
- NY 2017: a hora programada é sched_dep_time (hhmm, hora local de Nova York) na data do voo.
- PNW 2014: o pacote não traz o horário programado; hour e minute são da partida real. A hora programada é
  (partida real − dep_delay) mod 24 h na data do voo (a data é a programada, como na BTS). Os voos cancelados não
  têm horário nenhum e saem (relatado no README).
- Tempo meteorológico: year, month, day, hour locais, localizados no fuso do aeroporto; horas ambíguas ou
  inexistentes da troca de horário saem.
"""

from __future__ import annotations

import sys
from pathlib import Path

import numpy as np
import pandas as pd
import pyreadr


def utc_de_local(ano, mes, dia, minutos_do_dia, fuso):
    base = pd.to_datetime(pd.DataFrame({"year": ano, "month": mes, "day": dia}))
    local = base + pd.to_timedelta(minutos_do_dia, unit="min")
    loc = local.dt.tz_localize(fuso, ambiguous="NaT", nonexistent="NaT")
    return loc.dt.tz_convert("UTC")


def voos(f: pd.DataFrame, fuso: str, programada_min: pd.Series) -> pd.DataFrame:
    t = utc_de_local(f["year"], f["month"], f["day"], programada_min, fuso)
    ok = t.notna()
    f, t = f[ok].reset_index(drop=True), t[ok].reset_index(drop=True)
    hora = t.dt.floor("h")
    out = pd.DataFrame({
        "rownames": np.arange(1, len(f) + 1),
        "dep_time": pd.to_numeric(f["dep_time"], errors="coerce"),
        "dep_delay": f["dep_delay"].astype(float),
        "arr_delay": f["arr_delay"].astype(float),
        "carrier": f["carrier"].astype(str), "origin": f["origin"].astype(str), "dest": f["dest"].astype(str),
        "air_time": f["air_time"].astype(float), "distance": f["distance"].astype(float),
        "minute": ((t - hora) // pd.Timedelta(minutes=1)).astype(int),
        "time_hour": hora.dt.strftime("%Y-%m-%dT%H:%M:%SZ"),
    })
    return out, int((~ok).sum())


def clima(w: pd.DataFrame, fuso: str) -> pd.DataFrame:
    t = utc_de_local(w["year"].astype(int), w["month"].astype(int), w["day"].astype(int), w["hour"].astype(int) * 60, fuso)
    ok = t.notna()
    w, t = w[ok].reset_index(drop=True), t[ok].reset_index(drop=True)
    out = pd.DataFrame({"rownames": np.arange(1, len(w) + 1), "origin": w["origin"].astype(str),
                        "visib": w["visib"].astype(float), "wind_speed": w["wind_speed"].astype(float),
                        "precip": w["precip"].astype(float), "time_hour": t.dt.strftime("%Y-%m-%dT%H:%M:%SZ")})
    out = out.drop_duplicates(["origin", "time_hour"]).reset_index(drop=True)
    return out, int((~ok).sum())


def main():
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    dst.mkdir(parents=True, exist_ok=True)
    rel = []
    # NY 2017
    f = pyreadr.read_r(src / "nyc17" / "flights.rda")["flights"]
    hhmm = f["sched_dep_time"].astype(int)
    v, perdidos = voos(f, "America/New_York", (hhmm // 100) * 60 + hhmm % 100)
    v.to_csv(dst / "nyc17_flights.csv", index=False)
    w, wp = clima(pyreadr.read_r(src / "nyc17" / "weather.rda")["weather"], "America/New_York")
    w.to_csv(dst / "nyc17_weather.csv", index=False)
    rel.append(f"nyc17: {len(f)} voos, {perdidos} sem instante UTC (troca de horário); clima {len(w)} horas, {wp} descartadas")
    # PNW 2014
    f = pyreadr.read_r(src / "pnw14" / "flights.rda")["flights"]
    canc = f["dep_delay"].isna() | f["hour"].isna()
    g = f[~canc].reset_index(drop=True)
    real = g["hour"].astype(int) * 60 + g["minute"].astype(int)
    prog = (real - g["dep_delay"].round().astype(int)) % 1440
    v, perdidos = voos(g, "America/Los_Angeles", prog)
    v.to_csv(dst / "pnw14_flights.csv", index=False)
    w, wp = clima(pyreadr.read_r(src / "pnw14" / "weather.rda")["weather"], "America/Los_Angeles")
    w.to_csv(dst / "pnw14_weather.csv", index=False)
    rel.append(f"pnw14: {len(f)} voos, {int(canc.sum())} cancelados sem horário (saem), {perdidos} sem instante UTC; clima {len(w)} horas, {wp} descartadas")
    (dst / "conversao.txt").write_text("\n".join(rel) + "\n")
    print("\n".join(rel))


if __name__ == "__main__":
    main()
