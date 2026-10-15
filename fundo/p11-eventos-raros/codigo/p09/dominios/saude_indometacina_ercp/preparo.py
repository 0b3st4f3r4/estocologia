"""medicaldata::indo_rct (Elmunzer et al. 2012): uma linha por paciente; ambiente = centro do ensaio.

Os centros 3 (Kentucky, 22 pacientes) e 4 (Case Western, 3) entram juntos como "outros", por regra declarada
antes de rodar (DECLARACAO.md): sozinhos não sustentam um teste de invariância.
"""

from pathlib import Path

import pandas as pd


def _sim(s: pd.Series) -> pd.Series:
    return s.astype(str).str.startswith("1").astype(float)


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / "medicaldata_indo_rct.csv")
    d = d.dropna(subset=["outcome", "rx", "site", "age", "risk", "gender"])
    amb = d["site"].map({"1_UM": "UM", "2_IU": "IU", "3_UK": "outros", "4_Case": "outros"})
    return pd.DataFrame({
        "y": _sim(d["outcome"]),
        "ambiente": amb,
        "unidade": d["id"].astype(int),
        "indometacina": _sim(d["rx"]),
        "idade": d["age"].astype(float),
        "risco": d["risk"].astype(float),
        "homem": d["gender"].astype(str).str.startswith("2").astype(float),
        "sod": _sim(d["sod"]),
        "pep_previa": _sim(d["pep"]),
        "esfincterotomia_pancreatica": _sim(d["psphinc"]),
        "pre_corte": _sim(d["precut"]),
        "canulacao_dificil": _sim(d["difcan"]),
        "stent_pancreatico": _sim(d["pdstent"]),
        "residente": _sim(d["train"]),
    })
