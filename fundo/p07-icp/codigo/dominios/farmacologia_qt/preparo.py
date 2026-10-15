"""ECGRDVQ (FDA): QTcF por voluntário × braço × instante (média do triplo); ambiente = braço sorteado.
Declarado em ../../DECLARACAO_H2g.md antes de rodar."""

from pathlib import Path

import numpy as np
import pandas as pd

ARQ = Path("ecgrdvq") / "SCR-002.Clinical.Data.csv"
BRACO = {"Placebo": "placebo", "Dofetilide": "dofetilida", "Quinidine Sulph": "quinidina", "Ranolazine": "ranolazina",
         "Verapamil HCL": "verapamil"}
COL = {"dofetilida": "c_dof", "quinidina": "c_qui", "ranolazina": "c_ran", "verapamil": "c_ver"}


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados).parent / "h2g" / ARQ)
    d["braco"] = d["EXTRT"].map(BRACO)
    conc = d["PCSTRESN"].astype(float) / np.where(d["PCSTRESU"] == "pg/mL", 1000.0, 1.0)
    conc = conc.where(~((d["TPT"] < 0) & conc.isna()), 0.0)                 # pré-dose: 0
    conc = conc.where(d["braco"] != "placebo", 0.0)
    d["conc"] = conc
    d["qtcf"] = d["QT"] / (d["RR"] / 1000.0) ** (1 / 3)
    g = d.groupby(["RANDID", "braco", "TPT"], as_index=False).agg(
        y=("qtcf", "mean"), rr=("RR", "mean"), pr=("PR", "mean"), tamp=("Twave_amplitude", "mean"), conc=("conc", "first"),
        sexo=("SEX", "first"), idade=("AGE", "first"), peso=("WGHT", "first"))
    for b, c in COL.items():
        g[c] = np.where(g["braco"] == b, g["conc"], 0.0)
    g = g.dropna(subset=["y", "rr", "pr", "tamp", "conc", "peso"]).reset_index(drop=True)
    return pd.DataFrame({"y": g["y"].astype(float), "ambiente": g["braco"].astype(str), "unidade": g["RANDID"].astype(int),
                         "c_dof": g["c_dof"].astype(float), "c_qui": g["c_qui"].astype(float), "c_ran": g["c_ran"].astype(float),
                         "rr": g["rr"].astype(float), "pr": g["pr"].astype(float), "tamp": g["tamp"].astype(float),
                         "sexo": (g["sexo"] == "F").astype(float), "idade": g["idade"].astype(float), "peso": g["peso"].astype(float)})
