"""Olha só o desenvolvimento (t_emissao < corte) de um fluxo já rodado com --fase dev. Ferramenta de ajuste."""
import json, sys
from pathlib import Path
import numpy as np, pandas as pd
from estocologia import calibracao as C
for nome in sys.argv[1:]:
    s = Path(__file__).resolve().parent / "saida" / nome
    r = json.loads((s / "resultados_dev.json").read_text())
    df = pd.read_parquet(s / "diario.parquet")
    clima = C.climatologia_movel(df)
    ok = df["y"].notna().to_numpy() & (df["t_emissao"] < r["corte"]).to_numpy()
    d = df[ok]
    if r["tipo"] == "binario":
        out = C.binario(d, clima[ok])
    else:
        niv = [float(c.split("modelo_q")[1]) for c in df.columns if c.startswith("modelo_q")]
        out = C.quantis(d, niv)
        if nome == "bolsa":
            out["kupiec"] = C.kupiec((d["y"] < C.oficial_quantis(d, niv)[:, 0]).to_numpy(), 0.01)
    print(nome, json.dumps(out, default=float), r["contagens"], [e for e in r["eventos_operacao"]][:0])
