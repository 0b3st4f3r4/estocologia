"""R2: ICP no STAR (turma sorteada × série), teste K = 2 e regra de saturação K = 3.

Fonte da especificação: h2d/dominios/educacao_star_turmas/pacote.yaml, h2b/DECLARACAO_K2.md,
h2d/DECLARACAO_K3.md, h2d/README.md. Escolhas onde a especificação cala: DECLARACAO.md desta pasta.
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import pandas as pd
from comum import DADOS, gravar, Candidatos, icp

d = pd.read_csv(os.path.join(DADOS, "h2b", "AER_STAR.csv"))
series = ["k", "1", "2", "3"]
linhas = []
for i, g in enumerate(series):
    star = d["star" + g]
    ok = star.isin(["small", "regular"]) & d["read" + g].notna() & d["math" + g].notna()
    sub = d[ok]
    antes = np.zeros(len(sub))
    for g0 in series[:i]:
        antes += (sub["star" + g0] == "small").to_numpy()
    eth = sub.ethnicity.where(sub.ethnicity.isin(["cauc", "afam"]), "outra")
    eth = eth.where(sub.ethnicity.notna(), np.nan)
    linhas.append(pd.DataFrame({
        "serie": g,
        "turma": sub["star" + g].to_numpy(),
        "nota": (sub["read" + g] + sub["math" + g]).to_numpy(float),
        "turma_pequena": (sub["star" + g] == "small").to_numpy(float),
        "anos_pequena_antes": antes,
        "almoco_gratuito": np.where(sub["lunch" + g].isna(), np.nan, (sub["lunch" + g] == "free").astype(float)),
        "menina": np.where(sub.gender.isna(), np.nan, (sub.gender == "female").astype(float)),
        "etnia": eth.to_numpy(),
        "experiencia_prof": sub["experience" + g].to_numpy(float),
        "mestrado_prof": np.where(sub["degree" + g].isna(), np.nan, (sub["degree" + g] != "bachelor").astype(float)),
        "prof_negro": np.where(sub["tethnicity" + g].isna(), np.nan, (sub["tethnicity" + g] == "afam").astype(float)),
        "tipo_escola": sub["school" + g].to_numpy(),
    }))
L = pd.concat(linhas, ignore_index=True)
tipos = {"turma_pequena": "num", "anos_pequena_antes": "num", "serie": "cat", "almoco_gratuito": "num",
         "menina": "num", "etnia": "cat", "experiencia_prof": "num", "mestrado_prof": "num", "prof_negro": "num",
         "tipo_escola": "cat"}
L = L.dropna(subset=list(tipos)).reset_index(drop=True)
reg = L[L.turma == "regular"].groupby("serie").nota.agg(["mean", "std"])
L["y"] = (L.nota - L.serie.map(reg["mean"])) / L.serie.map(reg["std"])
L["amb"] = L.serie + "_" + np.where(L.turma == "small", "pequena", "regular")
treino = L[L.amb != "3_pequena"].reset_index(drop=True)

cand = Candidatos(treino, tipos)
r = icp(treino.y.to_numpy(), cand, treino.amb.to_numpy())
r["n_treino"] = int(len(treino))
r["n_cofre"] = int((L.amb == "3_pequena").sum())
r["n_por_ambiente"] = treino.amb.value_counts().sort_index().to_dict()
r["p_turma_pequena_sozinha"] = r["p_por_conjunto"]["turma_pequena"]
gravar("r2_star.json", r)
print({k: v for k, v in r.items() if k != "p_por_conjunto"})
