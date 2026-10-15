"""Mortes no trânsito nos 48 estados contíguos dos EUA, 1982–1988 (Ecdat::Fatality), em desvios dentro do estado."""

from pathlib import Path

import numpy as np
import pandas as pd

# códigos FIPS → região do Census Bureau
NORDESTE = {9, 23, 25, 33, 34, 36, 42, 44, 50}
CENTRO_OESTE = {17, 18, 19, 20, 26, 27, 29, 31, 38, 39, 46, 55}
OESTE = {4, 6, 8, 16, 30, 32, 35, 41, 49, 53, 56}


def _regiao(fips: int) -> str:
    if fips in NORDESTE:
        return "nordeste"
    if fips in CENTRO_OESTE:
        return "centro_oeste"
    if fips in OESTE:
        return "oeste"
    return "sul"


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "Ecdat_Fatality.csv")
    d["renda_log"] = np.log(d["perinc"])
    d["prisao_obrigatoria"] = (d["jaild"] == "yes").astype(float)
    d["servico_comunitario"] = (d["comserd"] == "yes").astype(float)
    # transformação "dentro" (efeito fixo de estado): cada variável numérica menos a média do próprio estado nos 7 anos
    num = {"y": "mrall", "imposto_cerveja": "beertax", "idade_minima": "mlda", "prisao_obrigatoria": "prisao_obrigatoria",
           "servico_comunitario": "servico_comunitario", "milhas_por_motorista": "vmiles", "desemprego": "unrate",
           "renda_log": "renda_log"}
    out = pd.DataFrame({"ambiente": d["state"].map(_regiao).astype(str), "unidade": d["state"].astype(int),
                        "ano": d["year"].astype(str)})
    for novo, col in num.items():
        x = d[col].astype(float)
        out[novo] = (x - x.groupby(d["state"]).transform("mean")).astype(float)
    return out
