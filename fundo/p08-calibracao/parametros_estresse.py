"""Parâmetros do teste de estresse (c2), saídos só dos diários do H3 (DECLARACAO.md, item c2)."""
import json
import sys
from pathlib import Path

import numpy as np
import pandas as pd

AQUI = Path(__file__).resolve().parent
sys.path.insert(0, str(AQUI / "codigo"))
from estocologia.calibracao import oficial_binario  # noqa: E402

H3 = Path("/mnt/project-files/estocologia/h3/saida")
CORTE13, CORTE23 = 1380585600, 1696118400
D13 = pd.read_parquet(H3 / "voos" / "diario.parquet")
D23 = pd.read_parquet(H3 / "voos23" / "diario.parquet")


def cofre(d, corte):
    c = d[(d["t_emissao"] >= corte) & d["y"].notna()].copy()
    c["p"] = oficial_binario(c)
    c["mes"] = pd.to_datetime(c["t_alvo"], unit="s", utc=True).dt.tz_convert("America/New_York").dt.month
    return c


c13, c23 = cofre(D13, CORTE13), cofre(D23, CORTE23)
desvio13 = float(c13["y"].mean() - c13["p"].mean())
y23 = float(c23["y"].mean())
out = {"desvio_2013_cofre": desvio13, "y_cofre_2023": y23, "n_cofre_2023": int(len(c23)),
       "S1_f": desvio13 / (1 - y23), "S2": {}}
for m in (10, 11, 12):
    a, b = c13[c13["mes"] == m], c23[c23["mes"] == m]
    d = float(a["y"].mean() - a["p"].mean())
    out["S2"][str(m)] = {"desvio_2013": d, "n_2013": int(len(a)), "y_2023": float(b["y"].mean()),
                         "f": max(d, 0.0) / (1 - float(b["y"].mean()))}
(AQUI / "saida" / "parametros_estresse.json").write_text(json.dumps(out, indent=1))
print(json.dumps(out, indent=1))
