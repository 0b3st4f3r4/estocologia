"""Calibração do instrumento sob H0 (fundo/p07-icp): taxa de rejeição a 5% do teste original, do Wald agrupado e da
permutação em blocos, em dados simulados com a forma do ensaio do FDA (22 voluntários × 4 braços × 16 instantes,
efeito do voluntário e AR(1) dentro do período). O S testado é o verdadeiro, então H0 vale. Não usa dados reais."""

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from estocologia.icp import expandir, p_invariancia_np  # noqa: E402
from icp_robusto import Permutador, p_cr, residuo_u  # noqa: E402


def simular(rng, G=22, m=4, T=16, rho=0.8, sd_vol=1.0, sd_per=0.5):
    linhas = []
    for g in range(G):
        v = rng.normal(0, sd_vol)
        for e in range(m):
            per = rng.normal(0, sd_per)
            x = rng.normal(0, 1, T) + (e == 1) * rng.gamma(2, 1, T)    # o braço 1 mexe em X
            ar = np.zeros(T)
            ar[0] = rng.normal()
            for t in range(1, T):
                ar[t] = rho * ar[t - 1] + np.sqrt(1 - rho ** 2) * rng.normal()
            y = 2 * x + v + per + ar
            linhas += [(g, e, xi, yi) for xi, yi in zip(x, y)]
    a = np.array(linhas)
    return a[:, 0].astype(int), a[:, 1].astype(int), a[:, 2], a[:, 3]


if __name__ == "__main__":
    rng = np.random.default_rng(11)
    R = int(sys.argv[1]) if len(sys.argv) > 1 else 200
    rej = {"original": 0, "cr": 0, "perm": 0}
    for _ in range(R):
        cl, ae, x, y = simular(rng)
        Xe = np.hstack([np.ones((y.size, 1)), expandir(x[:, None])[0]])
        u = residuo_u(Xe, y)
        rej["original"] += p_invariancia_np(Xe, y, ae.astype(str)) < 0.05
        rej["cr"] += p_cr(u, ae, cl, 4, 22)[0] < 0.05
        rej["perm"] += Permutador(ae, cl, 4, 199, int(rng.integers(1 << 30))).p(u)[0] < 0.05
    out = {k: v / R for k, v in rej.items()}
    out["R"] = R
    print(json.dumps(out))
    (Path(__file__).resolve().parent.parent / "saida").mkdir(exist_ok=True)
    (Path(__file__).resolve().parent.parent / "saida" / "calibracao_nulo.json").write_text(json.dumps(out, indent=1))
