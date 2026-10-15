"""OWID COVID-19 (owid/covid-19-data): painel país-mês, 2021-01 a 2022-12; ambiente = continente.

y      = log(1 + mortes por COVID por milhão no mês seguinte ao da medida de vacinação)
chave  = fração da população com esquema primário completo no fim do mês (0 a 1)
Recortes declarados: países com população ≥ 1 milhão; linhas com todas as colunas presentes.
"""

from pathlib import Path

import numpy as np
import pandas as pd

COLS = ["iso_code", "continent", "location", "date", "new_deaths_per_million", "new_cases_per_million",
        "people_fully_vaccinated_per_hundred", "stringency_index", "median_age", "aged_65_older", "gdp_per_capita",
        "hospital_beds_per_thousand", "diabetes_prevalence", "population"]


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "owid-covid-data.csv", usecols=COLS, parse_dates=["date"])
    d = d[d["continent"].notna() & (d["population"] >= 1e6) & ~d["iso_code"].str.startswith("OWID")]
    d = d.sort_values(["iso_code", "date"])
    d["vac"] = d.groupby("iso_code")["people_fully_vaccinated_per_hundred"].ffill()
    d["mes"] = d["date"].dt.to_period("M")
    g = d.groupby(["iso_code", "mes"])
    m = pd.DataFrame({
        "continente": g["continent"].first(),
        "mortes": g["new_deaths_per_million"].sum(min_count=1),
        "casos": g["new_cases_per_million"].sum(min_count=1),
        "vac": g["vac"].last(),
        "rigor": g["stringency_index"].mean(),
        "idade_mediana": g["median_age"].first(),
        "idosos": g["aged_65_older"].first(),
        "pib": g["gdp_per_capita"].first(),
        "leitos": g["hospital_beds_per_thousand"].first(),
        "diabetes": g["diabetes_prevalence"].first(),
    }).reset_index()
    m = m.sort_values(["iso_code", "mes"])
    m["mortes_mes_seguinte"] = m.groupby("iso_code")["mortes"].shift(-1)
    prox = m.groupby("iso_code")["mes"].shift(-1)
    m = m[(prox == m["mes"] + 1)]
    m = m[(m["mes"] >= pd.Period("2021-01", "M")) & (m["mes"] <= pd.Period("2022-12", "M"))]
    m["vac"] = m["vac"].fillna(0.0)          # antes da primeira dose registrada a cobertura é zero
    m = m.dropna(subset=["mortes_mes_seguinte", "casos", "rigor", "idade_mediana", "idosos", "pib", "leitos", "diabetes"])
    return pd.DataFrame({
        "y": np.log1p(m["mortes_mes_seguinte"].clip(lower=0)).to_numpy(),
        "ambiente": m["continente"].str.replace(" ", "_").to_numpy(),
        "unidade": m["iso_code"].to_numpy(),
        "vacinados": (m["vac"] / 100).clip(0, 1).to_numpy(),
        "casos_log": np.log1p(m["casos"].clip(lower=0)).to_numpy(),
        "rigor": m["rigor"].to_numpy(),
        "idade_mediana": m["idade_mediana"].to_numpy(),
        "idosos": m["idosos"].to_numpy(),
        "pib_log": np.log(m["pib"]).to_numpy(),
        "leitos": m["leitos"].to_numpy(),
        "diabetes": m["diabetes"].to_numpy(),
        "mes": m["mes"].astype(str).to_numpy(),
    })
