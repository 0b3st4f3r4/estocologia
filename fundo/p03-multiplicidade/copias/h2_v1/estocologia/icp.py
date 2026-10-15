"""Predição Causal Invariante (Peters, Bühlmann & Meinshausen 2016) — a lente Morfonomia do H2.

Para cada subconjunto S de preditores candidatos, testa H0_S: "a lei de Y dado X_S é a mesma em
todos os ambientes". Os pais causais de Y satisfazem H0 sob intervenções que não tocam Y; logo,
com probabilidade ≥ 1 − α, a interseção dos S aceitos está contida nos pais (Ŝ ⊆ pa(Y)).

Teste aproximado (o "approximate" do pacote InvariantCausalPrediction):
- gaussiana: F do modelo agrupado contra um modelo por ambiente, mais Levene dos resíduos (Bonferroni);
- binomial: razão de verossimilhanças do agrupado contra coeficientes por ambiente.

Cada candidato pode ocupar várias colunas (uma variável categórica vira um bloco de indicadoras).
"""

from __future__ import annotations

from itertools import combinations

import numpy as np
from scipy import stats

from estocologia.glm import ajustar


def _matriz(blocos: dict[str, np.ndarray], S: tuple, n: int) -> tuple[np.ndarray, list]:
    cols, nomes = [np.ones((n, 1))], [("intercepto", 0)]
    for v in S:
        b = blocos[v].reshape(n, -1)
        cols.append(b)
        nomes += [(v, j) for j in range(b.shape[1])]
    return np.hstack(cols), nomes


def p_invariancia(X: np.ndarray, y: np.ndarray, amb: np.ndarray, familia: str) -> float:
    """p-valor de H0: mesmos coeficientes (e, na gaussiana, mesma variância residual) em todos os ambientes."""
    rotulos = np.unique(amb)
    m, (n, p) = rotulos.size, X.shape
    agr = ajustar(X, y, familia)
    ll_sep, rss_sep, gl_sep = 0.0, 0.0, 0
    for a in rotulos:
        s = amb == a
        if s.sum() <= p:          # ambiente pequeno demais para um ajuste próprio: entra só pelo intercepto
            Xa = np.ones((s.sum(), 1))
        else:
            Xa = X[s]
        f = ajustar(Xa, y[s], familia)
        ll_sep += f["ll"]
        gl_sep += Xa.shape[1]
        if familia == "gaussiana":
            rss_sep += f["rss"]
    gl = gl_sep - p
    if gl <= 0:
        return 1.0
    if familia == "gaussiana":
        F = ((agr["rss"] - rss_sep) / gl) / (rss_sep / max(n - gl_sep, 1))
        p1 = float(stats.f.sf(F, gl, max(n - gl_sep, 1)))
        res = y - agr["mu"]
        p2 = float(stats.levene(*[res[amb == a] for a in rotulos], center="median").pvalue)
        return min(1.0, 2 * min(p1, p2))
    lr = 2 * (ll_sep - agr["ll"])
    return float(stats.chi2.sf(max(lr, 0.0), gl))


def icp(blocos: dict[str, np.ndarray], y: np.ndarray, amb: np.ndarray, familia: str, alpha: float = 0.05,
        max_tamanho: int | None = None) -> dict:
    """Roda o ICP sobre todos os subconjuntos (até max_tamanho). Devolve Ŝ, os aceitos e os intervalos.

    O intervalo de confiança de cada coeficiente é a união, sobre os S aceitos, do intervalo de nível
    1 − α no ajuste agrupado (0 quando a variável não está em S), como em Peters et al. (2016, §3.2).
    """
    nomes = sorted(blocos)
    n = y.size
    k = len(nomes) if max_tamanho is None else min(max_tamanho, len(nomes))
    z = stats.norm.ppf(1 - alpha / 2)
    testados, aceitos = [], []
    ic: dict[str, list] = {}
    for t in range(k + 1):
        for S in combinations(nomes, t):
            X, cols = _matriz(blocos, S, n)
            pv = p_invariancia(X, y, amb, familia)
            testados.append({"S": list(S), "p": pv})
            if pv > alpha:
                aceitos.append(list(S))
                f = ajustar(X, y, familia)
                se = np.sqrt(np.clip(np.diag(f["cov"]), 0, None))
                for v in nomes:
                    idx = [i for i, (nm, _) in enumerate(cols) if nm == v]
                    lo = min((f["beta"][i] - z * se[i] for i in idx), default=0.0)
                    hi = max((f["beta"][i] + z * se[i] for i in idx), default=0.0)
                    if not idx:
                        lo = hi = 0.0
                    cur = ic.setdefault(v, [np.inf, -np.inf])
                    ic[v] = [min(cur[0], lo), max(cur[1], hi)]
    S_hat = sorted(set.intersection(*map(set, aceitos))) if aceitos else []
    return {
        "alpha": alpha, "candidatos": nomes, "subconjuntos_testados": len(testados),
        "aceitos": aceitos, "S_hat": S_hat,
        "modelo_rejeitado": not aceitos,   # nenhum S invariante: heterogeneidade, não linearidade ou pai oculto
        "situacao": ("rejeitado: nenhum conjunto invariante" if not aceitos else
                     "vazio aceito: os ambientes não mexem o bastante em Y para separar conjuntos (pouco poder)" if [] in aceitos
                     else ("Ŝ não vazio" if set.intersection(*map(set, aceitos)) else "Ŝ vazio por conjuntos aceitos disjuntos")),
        "ambientes": int(np.unique(amb).size),
        "intervalos": {v: [float(a), float(b)] for v, (a, b) in ic.items()},
        "testes": testados,
    }
