"""Sachs et al. (2005): 11 fosfossinais por célula T em condições de reagente; ambiente = condição (arquivo)."""

from pathlib import Path

import numpy as np
import pandas as pd

# condições 6 e 13 (U0126, inibe MEK) ficam de fora: intervêm na lei de Erk dado pmek (ver pacote.yaml)
CONDICOES = ["1_cd3cd28", "2_cd3cd28icam2", "3_cd3cd28_aktinhib", "4_cd3cd28_g0076", "5_cd3cd28_psitect", "7_cd3cd28_ly",
             "8_pma", "9_b2camp", "10_cd3cd28icam2_aktinhib", "11_cd3cd28icam2_g0076", "12_cd3cd28icam2_psit",
             "14_cd3cd28icam2_ly"]
NOMES = {"praf": "raf", "pmek": "mek", "plcg": "plcg", "pip2": "pip2", "pip3": "pip3", "p44/42": "y", "pakts473": "akt",
         "pka": "pka", "pkc": "pkc", "p38": "p38", "pjnk": "jnk"}


def preparar(dados: Path) -> pd.DataFrame:
    partes = []
    for c in CONDICOES:
        d = pd.read_csv(Path(dados) / "sachs2005" / f"{c}.csv")
        d.columns = [str(k).strip().lower() for k in d.columns]      # 8_pma.csv traz pip2/pip3 em minúsculas
        d = d.rename(columns=NOMES)[list(NOMES.values())]
        d = np.log(d.astype(float))                                   # intensidades positivas e assimétricas → log
        d["ambiente"] = c
        d["unidade"] = c
        partes.append(d)
    return pd.concat(partes, ignore_index=True)
