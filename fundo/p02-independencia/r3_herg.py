"""R3: ICP no ensaio ECGRDVQ (farmacologia_qt) e inclinação do QTcF na dofetilida.

Fonte da especificação: h2g/DECLARACAO_H2g.md, h2g/dominios/farmacologia_qt/pacote.yaml, h2d/DECLARACAO_K3.md,
h2b/DECLARACAO_K2.md. Escolhas onde a especificação cala: DECLARACAO.md desta pasta.
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import pandas as pd
from comum import DADOS, gravar, Candidatos, icp

e = pd.read_csv(os.path.join(DADOS, "h2g", "ecgrdvq", "SCR-002.Clinical.Data.csv"))
e["qtcf"] = e.QT / (e.RR / 1000.0) ** (1.0 / 3.0)
braco = {"Placebo": "placebo", "Dofetilide": "dofetilida", "Quinidine Sulph": "quinidina",
         "Ranolazine": "ranolazina", "Verapamil HCL": "verapamil"}
e["amb"] = e.EXTRT.map(braco)
chave = ["RANDID", "amb", "TPT"]
g = e.groupby(chave)
L = g.agg(y=("qtcf", "mean"), rr=("RR", "mean"), pr=("PR", "mean"), tamp=("Twave_amplitude", "mean"),
          conc=("PCSTRESN", "first"), unid=("PCSTRESU", "first"), sexo=("SEX", "first"),
          idade=("AGE", "first"), peso=("WGHT", "first")).reset_index()
L["sexo"] = (L.sexo == "F").astype(float)
pre = L.TPT == -0.5
# pós-dose sem dosagem sai (no placebo não há dosagem: concentrações 0)
sem_dose = (~pre) & (L.amb != "placebo") & L.conc.isna()
L = L[~sem_dose].copy()
pre = L.TPT == -0.5
conc = np.where(pre | (L.amb == "placebo"), 0.0, L.conc.to_numpy(float))
conc = np.where(L.unid == "pg/mL", conc / 1000.0, conc)
for col, a in [("c_dof", "dofetilida"), ("c_qui", "quinidina"), ("c_ran", "ranolazina"), ("c_ver", "verapamil")]:
    L[col] = np.where(L.amb == a, conc, 0.0)
tipos = {k: "num" for k in ["c_dof", "c_qui", "c_ran", "rr", "pr", "tamp", "sexo", "idade", "peso"]}
L = L.dropna(subset=["y"] + list(tipos)).reset_index(drop=True)
treino = L[L.amb != "verapamil"].reset_index(drop=True)

cand = Candidatos(treino, tipos)
r = icp(treino.y.to_numpy(), cand, treino.amb.to_numpy())
r["n_treino"] = int(len(treino))
r["n_por_ambiente"] = L.amb.value_counts().sort_index().to_dict()

# inclinação: MQO linear de y em c_dof + ajuste, EP agrupado por voluntário, priori normal SHELF
cols = ["c_dof", "c_qui", "c_ran", "rr", "sexo", "idade", "peso"]
X = np.column_stack([np.ones(len(treino))] + [treino[c].to_numpy(float) for c in cols])
y = treino.y.to_numpy()
XtX_inv = np.linalg.inv(X.T @ X)
b = XtX_inv @ X.T @ y
res = y - X @ b
grupos = treino.RANDID.to_numpy()
meat = np.zeros((X.shape[1], X.shape[1]))
for gid in np.unique(grupos):
    m = grupos == gid
    s = X[m].T @ res[m]
    meat += np.outer(s, s)
G = len(np.unique(grupos))
n, k = X.shape
corr = G / (G - 1) * (n - 1) / (n - k)
V = corr * XtX_inv @ meat @ XtX_inv
ep_agr = float(np.sqrt(V[1, 1]))
ep_ing = float(np.sqrt(res @ res / (n - k) * XtX_inv[1, 1]))
mu0, sd0 = 28.0, (36.0 - 20.0) / 1.349
prec = 1 / sd0 ** 2 + 1 / ep_agr ** 2
mpost = (mu0 / sd0 ** 2 + b[1] / ep_agr ** 2) / prec
sdpost = prec ** -0.5
r["inclinacao_dof"] = {"mqo": float(b[1]), "ep_agrupado": ep_agr, "ep_ingenuo": ep_ing,
                       "efeito_desenho": (ep_agr / ep_ing) ** 2,
                       "priori": [mu0, sd0], "posterior": float(mpost), "sd_posterior": float(sdpost),
                       "ic90": [float(mpost - 1.644854 * sdpost), float(mpost + 1.644854 * sdpost)]}
gravar("r3_herg.json", r)
print({k: v for k, v in r.items() if k != "p_por_conjunto"})
