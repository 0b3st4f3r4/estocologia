"""Informativo, depois de ver o resultado (não decide nada): quanto da nova chamada fica invisível.

Uma nova chamada só tem customer_id se entrar na fila. Aqui se mede a fração de pedidos de atendente que
entram na fila, no geral e nos 15 min que seguem cada desistência da amostra índice.
"""
import json
import sys
from pathlib import Path

import numpy as np

AQUI = Path(__file__).resolve().parent
sys.path.insert(0, str(AQUI))
from estimar import preparar, montar  # noqa: E402

d = preparar()
idx = montar(d)
nt = d[(d["type"] != "TT") & d["outcome"].isin(["AGENT", "HANG"])]
ped = nt[nt["fila"] | (nt["outcome"] == "AGENT")]
t = np.sort(ped["ve"].to_numpy())
na_fila = ped.sort_values("ve")["fila"].to_numpy().astype(float)
cum = np.concatenate([[0], np.cumsum(na_fila)])
fim = idx.loc[idx["D"] == 1, "fim"].to_numpy()
a, b = np.searchsorted(t, fim, "right"), np.searchsorted(t, fim + 900, "right")
n, k = (b - a).sum(), (cum[b] - cum[a]).sum()
idq = nt[nt["fila"]]
out = {"P(fila | pedido de atendente), geral": float(ped["fila"].mean()),
       "P(fila | pedido nos 15 min depois de uma desistência índice)": float(k / n),
       "P(customer_id gravado | fila, atendido)": float(idq.loc[idq.outcome == "AGENT", "id_ok"].mean()),
       "P(customer_id gravado | sem fila, atendido)": float(nt.loc[~nt["fila"] & (nt.outcome == "AGENT"), "id_ok"].mean())}
r = json.loads((AQUI / "resultados.json").read_text())["principal_15min"]["r_IV"]
out["r_IV_15min"] = r
out["r_IV / P(fila depois da desistência) (correção grosseira, supõe novas chamadas como pedidos comuns)"] = r / out["P(fila | pedido nos 15 min depois de uma desistência índice)"]
print(json.dumps(out, indent=1, ensure_ascii=False))
(AQUI / "posthoc.json").write_text(json.dumps(out, indent=1, ensure_ascii=False))
