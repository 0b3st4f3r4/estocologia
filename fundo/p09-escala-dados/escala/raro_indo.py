"""Exploratório (DECLARACAO_2.md, K = 2 só para esta pergunta): resolução de previsões de evento raro no cofre do Indo.

O funil podou o domínio no F3; isto NÃO reabre o funil nem emite decisão. Ajusta uma logística (indometacina,
risco, idade, homem) no treino (IU + outros), prevê Michigan e decompõe o Brier (Murphy, 10 caixas iguais).
Uso:  python3 escala/raro_indo.py <pasta_dados>
"""

import json
import sys
from pathlib import Path

import numpy as np
from scipy import special

RAIZ = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(RAIZ))
from estocologia import dominio as D  # noqa: E402
from estocologia.glm import ajustar  # noqa: E402

VARS = ["indometacina", "risco", "idade", "homem"]


def murphy(p, y, caixas=10):
    b = np.minimum((p * caixas).astype(int), caixas - 1)
    ybar = y.mean()
    rel = res = 0.0
    for k in range(caixas):
        s = b == k
        if s.any():
            rel += s.sum() * (p[s].mean() - y[s].mean()) ** 2
            res += s.sum() * (y[s].mean() - ybar) ** 2
    n = y.size
    return {"confiabilidade": rel / n, "resolucao": res / n, "incerteza": ybar * (1 - ybar)}


def main(dados):
    p = D.carregar(RAIZ / "dominios" / "saude_indometacina_ercp")
    df = p.preparar(Path(dados))
    tr, cf = df[df.ambiente != "UM"], df[df.ambiente == "UM"]
    mu, sd = tr[VARS].mean(), tr[VARS].std()
    X = lambda d: np.column_stack([np.ones(len(d)), ((d[VARS] - mu) / sd).to_numpy(float)])  # noqa: E731
    f = ajustar(X(tr), tr.y.to_numpy(float), "binomial")
    pc = special.expit(X(cf) @ f["beta"])
    y = cf.y.to_numpy(float)
    clima = tr.y.mean()
    brier, brier_c = np.mean((pc - y) ** 2), np.mean((clima - y) ** 2)
    rng = np.random.default_rng(2027)
    hb = []
    for _ in range(2000):
        i = rng.integers(0, y.size, y.size)
        hb.append(1 - np.mean((pc[i] - y[i]) ** 2) / np.mean((clima - y[i]) ** 2))
    out = {"n_treino": int(len(tr)), "eventos_treino": int(tr.y.sum()), "n_cofre": int(y.size), "eventos_cofre": int(y.sum()),
           "taxa_treino": float(clima), "taxa_cofre": float(y.mean()), "brier": float(brier), "brier_clima": float(brier_c),
           "habilidade": float(1 - brier / brier_c), "habilidade_ic90_bootstrap": np.quantile(hb, [0.05, 0.95]).tolist(),
           "murphy_modelo": murphy(pc, y), "prob_prevista_min_max": [float(pc.min()), float(pc.max())],
           "beta_indometacina_padronizado": float(f["beta"][1])}
    print(json.dumps(out, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main(sys.argv[1])
