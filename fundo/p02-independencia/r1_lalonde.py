"""R1: H1·B LaLonde reimplementado a partir de h1-real/README.md e da DECLARACAO.md desta pasta."""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import pandas as pd
from comum import DADOS, SEMENTE, gravar, logit_ajustar, sigm

nsw = pd.read_csv(os.path.join(DADOS, "nsw_experimental.csv"))
cps = pd.read_csv(os.path.join(DADOS, "cps_observational.csv"))
for d in (nsw, cps):
    d["y"] = (d.re78 > 0).astype(float)

tr = nsw[nsw.treat == 1].reset_index(drop=True)
ct = nsw[nsw.treat == 0].reset_index(drop=True)


def dif(a, b):
    p1, p0 = a.y.mean(), b.y.mean()
    ep = np.sqrt(p1 * (1 - p1) / len(a) + p0 * (1 - p0) / len(b))
    return {"dR": p1 - p0, "ic95": [p1 - p0 - 1.959964 * ep, p1 - p0 + 1.959964 * ep],
            "drenda": a.re78.mean() - b.re78.mean()}


exp_ = dif(tr, ct)
crua = dif(tr, cps)


def X_dw(d):
    re74, re75 = d.re74 / 1000, d.re75 / 1000
    cols = [d.age, d.age ** 2, d.educ, d.educ ** 2, d.marr, d.nodegree, d.black, d.hisp,
            re74, re75, (d.re74 == 0).astype(float), (d.re75 == 0).astype(float), d.educ * re74]
    return np.column_stack([np.ones(len(d))] + [np.asarray(c, float) for c in cols])


def ps_att(t, c):
    X = np.vstack([X_dw(t), X_dw(c)])
    z = np.r_[np.ones(len(t)), np.zeros(len(c))]
    mu, sd = X[:, 1:].mean(0), X[:, 1:].std(0)
    Xs = X.copy()
    Xs[:, 1:] = (X[:, 1:] - mu) / sd
    b = logit_ajustar(Xs, z)
    e = sigm(Xs @ b)
    ec = e[len(t):]
    w = ec / (1 - ec)
    yc, rc = c.y.to_numpy(), c.re78.to_numpy()
    return {"dR": t.y.mean() - np.sum(w * yc) / w.sum(),
            "drenda": t.re78.mean() - np.sum(w * rc) / w.sum(),
            "ess": w.sum() ** 2 / np.sum(w ** 2),
            "frac_cps_abaixo_min_tratado": float(np.mean(ec < e[:len(t)].min()))}


ps = ps_att(tr, cps)
rng = np.random.default_rng(SEMENTE)
B = 2000
bd, br = np.empty(B), np.empty(B)
for i in range(B):
    t = tr.iloc[rng.integers(0, len(tr), len(tr))].reset_index(drop=True)
    c = cps.iloc[rng.integers(0, len(cps), len(cps))].reset_index(drop=True)
    r = ps_att(t, c)
    bd[i], br[i] = r["dR"], r["drenda"]
ps["ic95_dR"] = list(np.quantile(bd, [0.025, 0.975]))
ps["ic95_drenda"] = list(np.quantile(br, [0.025, 0.975]))

res = {"experimento": exp_, "crua_cps": crua, "escore_propensao": ps,
       "n": {"tratados": len(tr), "controles_nsw": len(ct), "cps": len(cps)}}
res = {k: ({kk: (float(vv) if np.isscalar(vv) else [float(x) for x in vv]) for kk, vv in v.items()})
       for k, v in res.items()}
gravar("r1_lalonde.json", res)
for k, v in res.items():
    print(k, {kk: (round(vv, 4) if isinstance(vv, float) else [round(x, 4) for x in vv]) for kk, vv in v.items()})
